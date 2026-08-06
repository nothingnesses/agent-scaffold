# `workflow-enforcement-tier-inc3` work review, ROUND 3, ADVERSARIAL-CONSTRUCTION lens

Reviewed in worktree `.claude/worktrees/rev-inc3-r3-adversarial` on branch `review/inc3-r3-adversarial` at `ce820fb`, the tip of the branch under review. Governing specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`. Both prior triage files (`...-r1-triage.md`, `...-r2-triage.md`) were read in full before any measurement, and every ruling in them is treated as settled unless this file says otherwise and shows why.

THREE FINDINGS, ALL `low`. Nothing at `medium`, `high` or `critical`. The four claims the brief singled out as the round 2 fix pass's load-bearing promises all HELD under attack, and the section "What I attacked that produced nothing" below sets out exactly what was tried, because a near-clean round is only worth anything if the reader can see the shape of the attack.

## Method

`<scratch>` abbreviates `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad`. All fixtures live under `<scratch>/r3adv`, a directory of my own naming. Every directory chmodded to 000 or 600 was chmodded back; the closing `find <scratch>/r3adv -type d ! -perm -u+rwx` returns nothing and `find <scratch>/r3adv/fx <scratch>/r3adv/race -type f -perm 000` returns nothing. `TMPDIR` was `<scratch>/r3adv/tmpdir`, outside any git repository, for every `cargo test`.

FOUR BINARIES, each built by me from source in my own directory, never another worktree's target:

| Name | Commit | Location | What it is |
| --- | --- | --- | --- |
| NEW | `ce820fb` | the reviewed worktree's `target/debug` | the branch tip |
| BASE | `bd5bd47` | `<scratch>/r3adv/build/base` | the round 2 fix pass's base, so the newest 4 commits are isolable |
| PRE | `9eeca42` | `<scratch>/r3adv/build/pre` | predates the whole increment |
| PREFIX1 | `a35e92b` | `<scratch>/r3adv/build/prefix1` | the last commit BEFORE the `Ok`/`Err` split (`f932970`), used for the RED demonstration |

PRE IS A VALID BASELINE, CHECKED RATHER THAN ASSUMED: `git diff --stat 9eeca42 main -- src/ tests/ Cargo.toml Cargo.lock build.rs` is EMPTY, and `9eeca42` is an ancestor of `main`. So a PRE binary is the same product as one built from `main`.

TOOLCHAIN, and the brief was right to warn: every `cargo` invocation went through `cd <worktree> && direnv allow && eval "$(direnv export bash)"` first. `which cargo` resolves to `/nix/store/76jaab43a2l7n7fiifxjngp68kk167vm-rust-mixed/bin/cargo`, `cargo 1.98.0-nightly (a335d47ff 2026-06-26)`, `rustc 1.98.0-nightly (f46ec5218 2026-06-30)`. No claim below rests on a build made outside that environment.

GATES MEASURED AT `ce820fb`:

- `cargo test` as the user: **422 passed, 0 failed**, 9 binaries.
- `cargo test` under `unshare -Ur` (namespace root): **422 passed, 0 failed**. Round 2's `V-1` is fixed and NO test in this repository is root-fragile any more.
- `cargo clippy --all-targets -- -D warnings`: **exit 0**, in the direnv environment.

---

## `R3A-1` `low`: the errno clause added by `ce820fb` is a NO-OP INSTRUCTION on the one input the `Err` arm exists for, and following it literally reproduces byte-identical output

### The claim

`ce820fb` appended `; pass a `--metrics` naming this project's log` to the `Err` arm at `src/main.rs:1072-1075`. On EACCES, which is the errno of round 1's `T-1` fixture, of the test at `tests/validate_workflow_toml_source_needs_no_plan.rs:284`, and of the only input this arm was created to serve, the resolved path ALREADY IS this project's log, so the instruction cannot change anything: the operator who obeys it gets the same sentence back.

### Reproducible evidence

Fixture (a mode-600 `docs/metrics`: readable, so the log is listable by name, and unsearchable, so `metadata` on it fails; this is the fix's own fixture shape):

```sh
mkdir -p <scratch>/r3adv/fx/proj/docs/plans <scratch>/r3adv/fx/proj/docs/metrics
cd <scratch>/r3adv/fx/proj
printf '[meta]\ntitle = "T"\nprimary = "toml"\n\n[[step]]\nslug = "only-step"\ntitle = "The only step"\nstatus = "not-started"\norder = 1\n' > docs/plans/p.plan.toml
printf '{"type":"round","task":"only-step","step":"only-step","increment":"only-step","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk"}\n' > docs/metrics/workflow.jsonl
chmod 600 docs/metrics
```

STEP 1, the tool's own diagnosis and remedy:

```
$ NEW validate --source docs/plans/p.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked
(Permission denied (os error 13)): the workflow check could not run, so it cannot report that
the invariants hold; pass a `--metrics` naming this project's log
exit=1
```

STEP 2, OBEY THE INSTRUCTION. `docs/metrics/workflow.jsonl` IS this project's log, so naming it is the literal execution of what the tool just asked for:

```
$ NEW validate --source docs/plans/p.plan.toml --workflow --metrics docs/metrics/workflow.jsonl
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked
(Permission denied (os error 13)): the workflow check could not run, so it cannot report that
the invariants hold; pass a `--metrics` naming this project's log
exit=1
```

BYTE-IDENTICAL. The tool printed an instruction, the operator carried it out exactly, and the tool printed the same instruction again.

BASELINES, so the reader can see this is fix-induced and not inherited:

```
BASE (bd5bd47)  --workflow requested but the round log at docs/metrics/workflow.jsonl could not
                be checked (Permission denied (os error 13)): the workflow check could not run,
                so it cannot report that the invariants hold                            exit=1
                  -> names the errno and STOPS. No instruction, so nothing to obey uselessly.
PRE (9eeca42)   --workflow has a plan source but the metrics log is missing; skipping the
                workflow check                                                          exit=0
```

The clause exists only in `ce820fb`, the newest commit on the branch. Same result on the Markdown arm of the same match (`--plan docs/plans/p.md --workflow`, mode-600 `docs/metrics`), and on a symlinked path behind a mode-000 ancestor (`--metrics sub/l.jsonl`), so it is the arm and not one path spelling.

### Why this is a defect and not a quibble

The `Err` arm's whole purpose, stated in its own comment at `src/main.rs:1060-1063`, is that "a real log may sit behind that error and sending its operator to record rounds that are already recorded is the falsehood `Q-55-emptyroot` decided against." The `Ok` arm can say "pass a `--metrics` naming this project's log" honestly, because there genuinely is nothing at the resolved path, so the presupposition "your path is wrong" is warranted. The `Err` arm has NO evidence the path is wrong. It has evidence that it could not look. `ce820fb` imported the `Ok` arm's presupposition into the arm built precisely to avoid making it.

There is also a way to make the instruction bite, and it is worse than the no-op. Naming a DIFFERENT readable file is the only execution that changes the outcome, and it converts the correct refusal into an affirmative pass:

```
$ mkdir -p alt && cp <a copy of the round record> alt/workflow.jsonl
$ NEW validate --source docs/plans/p.plan.toml --workflow --metrics alt/workflow.jsonl
alt/workflow.jsonl: 1 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs alt/workflow.jsonl: workflow invariants hold
exit=0
```

And the other literal reading, naming the project's log where it can actually be read from outside the tree, is refused by the containment guard:

```
$ NEW validate --source docs/plans/p.plan.toml --workflow --metrics <scratch>/r3adv/outside.jsonl
--workflow would join docs/plans/p.plan.toml against <scratch>/r3adv/outside.jsonl, which is not
under the plan's project root <scratch>/r3adv/fx/proj; pass a `--metrics` under that root, run
against the plan's own log, or correct the `--source` and `--plan` pair          exit=1
```

So on this input every reading of the instruction either changes nothing, is refused, or green-passes a different file. I DO NOT lean on the green-pass as the weight of the finding, and I say so explicitly: the `Ok` arm has carried the identical clause since round 1, so pointing `--metrics` at a substitute log is a property of the flag rather than of this commit. The weight is the no-op.

### NEW EVIDENCE THAT A ROUND 2 STATEMENT WAS WRONG, stated explicitly as the brief requires

I am NOT reopening `V-3`'s verdict. `V-3` was VALID and its diagnosis (three errno classes lost a true sentence and its remedy clause) reproduces here: `ENOTDIR`, `ELOOP` and `ENAMETOOLONG` all now carry the clause and all are genuinely helped by it, because on those a corrected `--metrics` really is the fix. What is wrong is one sentence INSIDE `V-3`'s remedy, at `...-r2-triage.md:182`: "it applies to every errno including EACCES, where 'pass a `--metrics` naming this project's log' is also useful advice." STEP 2 above measures that claim false. The triage asserted the EACCES case rather than running it; running it is what this lens is for.

### The right behaviour

Two candidate remedies, and I state a preference without pretending the choice is mine:

1. SPLIT THE CLAUSE OFF EACCES, keeping it on the errnos where a corrected path is the fix. This authors a classification of errnos that this tree does not have, which round 2's triage rejected on `V-3` for a reason that still applies ("a fix pass which authors new structure manufactures the next round's finding").
2. REVERT `ce820fb`, returning the `Err` arm to naming the errno and stopping. Zero authored words, a pure deletion of nine, and it restores `BASE`'s behaviour on EACCES while giving up the improvement `V-3` bought on three narrow errno classes.

I lean to (2) and would understand a human choosing to do NOTHING. The exit code is right on every errno, the errno is named in every case, and the cost of the defect is one wasted command. If the fix pass is to spend words at all, this is the finding I would drop first, exactly as `V-3` was for round 2.

---

## `R3A-2` `low`: the test's doc comment still names `Path::exists` as the gate, and the round 2 fix pass edited that same doc comment four lines below without correcting it

### The claim

`tests/validate_workflow_toml_source_needs_no_plan.rs:267` reads:

```rust
/// (`Q-55-existsgate`). The gate the policy above hangs on is `Path::exists`, which is
/// `fs::metadata(..).is_ok()` and so answers false both for a log that is not there and for
/// one behind a directory the process cannot traverse.
```

`7f2e3c3` replaced that gate. `metrics_path.exists()` no longer appears anywhere in `run_validate`; the gate is now `let metrics_probe = metrics_path.try_exists();` at `src/main.rs:845` and `matches!(metrics_probe, Ok(true))` at `:846`.

### Reproducible evidence

```
$ grep -n "\.exists()\|try_exists()" src/main.rs
...
845:	let metrics_probe = metrics_path.try_exists();
880:		if source_path.exists() {
924:		if plan_path.exists() {
...
1237:	} else if metrics_path.exists() {
```

`:845` is the metrics gate and it is `try_exists()`. The nearest `metrics_path.exists()` in the file is `:1237`, in a DIFFERENT function (the `status`/`next` projection path), which is exactly the confusion the stale sentence sets up for a reader who greps.

That the fix pass had the file open on this very comment:

```
$ git show 17fcb69 -- tests/validate_workflow_toml_source_needs_no_plan.rs
-/// RED against `1799f8b`, the round 1 tip: that build printed `no round log at
+/// RED before this commit: the prior build printed `no round log at
```

`:279` is inside the same `///` block as `:267`. `7f2e3c3` also updated the CORRESPONDING comment in `src/main.rs` in the same pass, dropping its `Path::exists` reference for `Ok(true)` and `the gate above keeps that predicate`. Only the test file's copy was left behind.

BASELINE: at `main` and at `BASE` the sentence was TRUE, because the gate was `metrics_path.exists()`. It became false at `7f2e3c3`. This is fix-induced staleness in the newest material, not an inherited error.

### Why it matters at all, and why only `low`

The comment's REASONING survives the change: `matches!(try_exists(), Ok(true))` is `metadata().is_ok()`, so "answers false both for a log that is not there and for one behind a directory the process cannot traverse" is still exactly right, and the test's behaviour is unaffected. What is wrong is the API the sentence names, in the one file a maintainer would open to understand why `Q-55-existsgate` was scoped the way it was. `low`, and nothing more.

### The right behaviour

A minimal substitution that authors no new claim, reusing the wording `7f2e3c3` already put in `src/main.rs`: "The gate the policy above hangs on is a single `try_exists()` probe tested for `Ok(true)`, which is `fs::metadata(..).is_ok()` and so answers false both for ...". The rest of the sentence stands unchanged.

---

## `R3A-3` `low`, PRE-EXISTING and NOT FIX-INDUCED: the check's OTHER input still makes exactly `T-1`'s false claim, with exactly `T-1`'s unfollowable remedy, on exactly `T-1`'s fixture shape

### The claim

`Q-55-existsgate` established, inside this increment, that a round log the tool CANNOT ASK ABOUT is not a round log that is ABSENT. `src/main.rs:814-815` says the check has two inputs and that "Both of the check's inputs answer that way: no resolvable plan source, and no round log at the resolved metrics path." The increment applied the distinction to one of the two. The plan-source input, gated by `source_path.exists()` at `src/main.rs:880`, still collapses "not there" and "cannot look" into one answer, and the sentence it produces is false in the second case while its remedy instructs the operator to do the thing they already did.

### Reproducible evidence

The `T-1` fixture shape, transposed onto `docs/plans` instead of `docs/metrics`:

```sh
chmod 600 docs/plans          # readable, not searchable, exactly as the log fixture is
```

The source is demonstrably there:

```
$ ls -A docs/plans
p.plan.toml
$ stat docs/plans/p.plan.toml
stat: cannot statx 'docs/plans/p.plan.toml': Permission denied
```

What the tool says:

```
$ NEW validate --source docs/plans/p.plan.toml --workflow
no source plan at docs/plans/p.plan.toml; nothing to validate
--workflow requested but no plan source resolved: pass a TOML-primary --source or a Markdown --plan
exit=1
```

Both sentences are wrong about the world. There IS a source plan at that path, and the operator DID pass a TOML-primary `--source`. The control, the same fixture with nothing but the mode changed back, proves the source behind the error is a working one:

```
$ chmod 755 docs/plans && NEW validate --source docs/plans/p.plan.toml --workflow
docs/metrics/workflow.jsonl: 1 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

Obeying the remedy changes nothing, the same way `R3A-1`'s does:

```
$ NEW validate --source ./docs/plans/p.plan.toml --workflow
no source plan at ./docs/plans/p.plan.toml; nothing to validate
--workflow requested but no plan source resolved: pass a TOML-primary --source or a Markdown --plan
exit=1
```

ATTRIBUTION, WHICH IS THE HONEST PART OF THIS FINDING: `NEW`, `BASE` and `PRE` are BYTE-IDENTICAL on this input, stdout, stderr and exit code. This behaviour predates the increment entirely and no commit under review touches `src/main.rs:880`. It is NOT a regression and NOT fix-induced.

### Why I raise it anyway, and what I checked first

I checked that it is not already settled. It is not on the do-not-raise list: it is not one of the four standing residuals, not accepted costs (i) to (iv), not `ADV-4`, not `SC-3`, not `R2A-4`, and it is NOT the queued plain-`validate` inconsistency, which is about the LOG input's exit codes (mode-000 log FILE at exit 1 versus unsearchable DIRECTORY at exit 0). This is the PLAN SOURCE input's message on the `--workflow` surface, a different input and a different failure.

Round 1's triage came within one line of it and did not look. `...-r1-triage.md:88` reads: "With `docs` itself at mode 000, the run reports `--workflow requested but no plan source resolved` (verified)." That run was used as a BOUND on how far up the tree the log-side fixture could put the unsearchable directory. Nobody asked whether the sentence it produced was true. It is not.

I record my own view of the right handling rather than pushing for a fix: ROUTE IT, do not fix it in this increment. The fix is the same shape as `T-1`'s (a `try_exists()` probe and a second sentence) but on a gate whose `else` branch also feeds plain `validate`, so it lands squarely in `Q-55-existsgate`'s DECLINED territory and is a fresh human decision, not a triager's instruction. The natural home is the queued validation-constraints step, beside `R2A-4` and the plain-`validate` inconsistency, so whoever takes that queue item finds all three halves of the same subject together. A triager who rules this out of scope as pre-existing untouched code is making a defensible call and I would not argue with it; what I would argue against is losing the observation.

---

## What I attacked that produced nothing

This is the section that makes a near-clean round credible, so it is specific.

### 1. THE SINGLE-PROBE REBINDING. `Ok(true)` IS UNREACHABLE IN THE ARM. I could not reach it.

BY CONSTRUCTION, established from the code and then tested rather than the other way round. `metrics_contents` is `Some` if and only if `matches!(metrics_probe, Ok(true))` (`src/main.rs:846`, `:865`, `:868`). The `_` arm at `:1067` is reachable only from `(Some(source), _, None)` or `(None, Some(plan), None)`, both of which require `metrics_contents == None`, hence require the gate to have been false, hence require the SAME bound value to be `Ok(false)` or `Err`. `matches!` binds nothing, so no move occurs and the arm reads the identical value.

BY CONSTRUCTED RACE, because "by construction" is what round 2's `R2A-2` disproved about the previous shape. I rebuilt both of `R2A-2`'s cells with the FIFO widener (a FIFO at the `--source` path blocks `run_validate` at `fs::read_to_string` at `:881`, which is AFTER the probe at `:845`, so the filesystem can be changed mid-run under my control), and ran each against NEW and BASE:

```
CELL 1, gate answers ENOENT, the log is CREATED mid-run:
  NEW   no round log at docs/metrics/workflow.jsonl ... record the project's review rounds there  exit=1
  BASE  (identical)
  log on disk at end: 241 bytes
CELL 2, gate answers Ok(false) TRUTHFULLY, the directory becomes unsearchable mid-run,
        the log NEVER exists:
  NEW   no round log at docs/metrics/workflow.jsonl ...                                           exit=1
  BASE  --workflow requested but the round log ... could not be checked (Permission denied
        (os error 13)): ...                                                                       exit=1
  log dir contents at end: []
```

CELL 2 is round 2's `V-2` and IT IS FIXED: BASE prints a "could not be checked" for a question it had successfully answered; NEW prints the true sentence. CELL 1 is unchanged, exactly as the round 2 triage predicted and warned in its own correction at `...-r2-triage.md:149`; no single-stat implementation can do better, and the sentence is now a faithful report of the one observation that decided the exit code. In neither cell, and in no other construction I tried, did `Ok(true)` reach the arm.

THE REVERSE DIRECTION the brief asked about, whether a case that should reach the arm no longer does or vice versa: NO. The gate predicate is unchanged (item 2 below), so the set of inputs reaching the `_` arm is unchanged, and the 22-input `--workflow` sweep in item 5 confirms it input by input.

THE ONE THING I WILL NOTE WITHOUT RAISING IT: the arm matches `Ok(_)` rather than `Ok(false)`. Today that is safe and the comment "`Ok` asserts absence" is true across the whole reachable domain. It is safe only because of the gate, and the compiler cannot say so; if the gate is ever widened, `Ok(true)` will silently take the absent sentence rather than failing to compile. That is a latent trap, not a defect, and turning it into `Ok(false)` plus an explicit `Ok(true)` arm would author structure for no measured gain. Recorded as an observation.

### 2. `Q-55-existsgate` IS NOT VIOLATED. Plain `validate` is byte-identical to PRE across 35 inputs, AND identical BY CONSTRUCTION on this toolchain.

THE CONSTRUCTION ARGUMENT, read out of the toolchain's own std source rather than remembered. `Path::exists` is `fs::metadata(self).is_ok()` (`library/std/src/path.rs:3547-3549`). `Path::try_exists` is `fs::exists(self)` (`:3580-3582`), `fs::exists` is `fs_imp::exists` (`library/std/src/fs.rs:3503-3505`), and on unix that resolves through `sys/fs/unix.rs:93` to `sys/fs/common.rs:56-62`:

```rust
pub fn exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}
```

So `matches!(try_exists(), Ok(true))` IS `metadata().is_ok()`, the same call with the same syscall and the same errno handling. This matters because it rules out the failure mode I went looking for: if this std had used `access(F_OK)` for `fs::exists`, as some versions were proposed to, the real-versus-effective-uid difference would have made the gate genuinely different from `exists()` and `Q-55-existsgate`'s promise breakable. It does not, on `rustc 1.98.0-nightly (f46ec5218 2026-06-30)`, which is the flake's toolchain.

THE MEASUREMENT, because a construction argument is not evidence. 35 plain-`validate` inputs, NEW versus PRE, stdout and stderr and exit code compared as one string: **35 cases, 0 differing.** The inputs: `docs/metrics` at modes 000, 600, 111, 555 and 755; a mode-000 `docs` ancestor; a mode-000 log FILE; a genuinely absent log; an explicit present `--metrics`; an absent named `--metrics`; an empty `--metrics ""`; ENOTDIR through a file; a `--metrics` that is a directory; a symlink loop (ELOOP); a dangling symlink; a 300-character leaf (ENAMETOOLONG); `/dev/null`; `/etc/hostname` and `/root/x.jsonl` (out of root, one EACCES); a `../` path leaving the root; a trailing slash on a regular file; a symlink to a valid log; `/proc/self/mem`; `.`; malformed records; invalid UTF-8; an empty file; no `--source` at all; a missing `--source`; a `--source` that is a directory; a Markdown `--plan`; a missing `--plan`; `--source` plus `--plan`; no arguments at all; and a mode-000 `--source`.

### 3. `status`, `next`, `status --resume`, `render` AND `audit` ARE UNTOUCHED. 35 more inputs, 0 differing.

NEW versus PRE, same three-way comparison: **35 cases, 0 differing.** 20 `status` inputs (default, `--json`, `--resume`, `--resume --json`, no args, `--plan`, absent log, metrics dir at 600 and 000, ENOTDIR, `--metrics` a directory, ENAMETOOLONG, a missing `--ledger-fragment`, a mode-000 ledger, a ledger that is a directory, a `--source` that is a directory, `/dev/null`, an out-of-root `--metrics`, `--resume` with no source, and `--ledger-fragment` without `--resume`), 12 `next` inputs (including `--json`, `--isolation-tier`, absent log, a mode-600 metrics dir, ENOTDIR, a directory, an out-of-root `--metrics` in both human and JSON form, and a missing `--source`), and `render --check`, `render --check --strict`, `audit --json`. The increment leaked into none of them.

### 4. THE MOVED ASSERTION STILL PINS WHAT IT CLAIMS, and no other test is environment-fragile.

The `assert_eq!(code, Some(1))` that `afb96ba` moved inside `if opaque` still fires for the ordinary user. Demonstrated rather than argued: I exported the tree at `a35e92b`, the last commit BEFORE the `Ok`/`Err` split, dropped the CURRENT test file into it unmodified, and ran it in the project toolchain:

```
running 4 tests
test a_round_log_that_cannot_be_checked_is_not_reported_as_missing ... FAILED
panicked at tests/validate_workflow_toml_source_needs_no_plan.rs:319:9:
the log is on disk, so this sentence is false; stderr:
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: ... record the project's
  review rounds there
test result: FAILED. 3 passed; 1 failed
```

The panic is at `:319`, INSIDE the `if opaque` block, which proves both halves at once: the block is entered for the ordinary user (so the guard has not made the test vacuous) and the regression it pins is still detected. Nothing was neutered.

WHOLE-SUITE ROOT RUN, the control round 2's `V-1` needed: `unshare -Ur ... cargo test` at `ce820fb` gives **422 passed, 0 failed**, identical to the user run. `V-1` is closed and no other test in the repository fails as root.

### 5. A WRONG MESSAGE ON A RIGHT EXIT CODE. I swept for more and found the two above and no third.

22 `--workflow` inputs, each run against NEW, BASE and PRE, with every message read for truth against the fixture on disk rather than for plausibility. The inputs: a valid present log; a genuinely absent log; `docs/metrics` at 600, 000 and 111; a mode-000 log FILE; a `--metrics` that is a directory; ENOTDIR; ELOOP; a dangling symlink; ENAMETOOLONG; `/dev/null`; an out-of-root `--metrics`; an empty `--metrics ""`; a `--metrics` under a parent directory that does not exist; a missing `--source`; no source at all; malformed records; an invalid-UTF-8 log; an empty log; `/root/x.jsonl`; and a mode-000 `docs/plans`. Findings `R3A-1` and `R3A-3` came out of that sweep. Everything else was either true, or identical on all three builds and therefore pre-existing and outside this increment, or already settled:

- `--metrics` a DIRECTORY and a mode-000 log FILE both produce a bare `Error: Os { code: 21, kind: IsADirectory, ... }` / `Error: Os { code: 13, kind: PermissionDenied, ... }` at exit 1, naming no path and never mentioning the workflow. NEW, BASE and PRE are byte-identical. This is the QUEUED plain-`validate` inconsistency's own territory and the brief forbids raising it; I record only that the `--workflow` surface inherits its message shape too.
- `no round log at <path>` on ENOENT-class paths where nothing could ever exist (a missing parent, a dangling symlink) is terse rather than false, which is round 1's triage ruling and I reproduce it rather than reopen it.
- The stale `no metrics log at <path>; nothing to validate` note one line above the corrected sentence is round 2's `R2A-4`, ACCEPTED AS A RESIDUAL. It appears in most of my transcripts above. Not raised.
- An empty file at the resolved path still yields `workflow invariants hold` at exit 0. That is round 1's `ADV-4`, ACCEPTED AS A RESIDUAL. Not raised.

### 6. FALSE-GREEN HUNT, the failure class this increment exists to close. Nothing.

I tried to find any input where `--workflow` exits 0 without having read a real log, by construction and then by measurement. Every path out of the `--workflow` block with `metrics_contents == None` pushes a problem: `(None, None, _)` at `:1042`, the `_` arm at `:1067`, and the containment guard at `:992-1003` which fires before the match. `(Some(_), _, Some(_))` and `(None, Some(_), Some(_))` are unreachable with `None`. No input in the 22-case sweep exited 0 without a readable log, and the two arms that could be gamed are refused:

```
--metrics docs/metrics/sneaky.jsonl -> symlink to /dev/null, INSIDE the root
  --workflow would join ... which is not under the plan's project root ...          exit=1
--metrics docs/metrics/foreign-link.jsonl -> symlink to a VALID log outside the root
  --workflow would join ... which is not under the plan's project root ...          exit=1
```

The containment guard resolves through the real on-disk location, so a symlink inside the tree cannot disguise a foreign log as the project's own. That is the README's claim and it holds.

### 7. THE CHECK ITSELF STILL WORKS, the regression control nobody should skip.

```
$ NEW validate --source docs/plans/c.plan.toml --workflow --metrics docs/metrics/empty.jsonl
docs/plans/c.plan.toml vs docs/metrics/empty.jsonl: Roadmap step `only-step` is `complete` but
has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"`
for it if it predates logging or its review was skipped                              exit=1
$ NEW validate --source docs/plans/c.plan.toml --workflow          # the same step, rounds present
docs/plans/c.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold       exit=0
```

W3 still fires and still clears. The increment did not damage the check it gates.

### 8. THE README'S TWO CLAIMS THIS INCREMENT CHANGED. Both true.

- "a bare filename run from inside `docs/plans` ... fails, naming the log it looked for" (`README.md:234`, reworded by this increment from "reports that it found no log"): `cd docs/plans && NEW validate --source my-task.plan.toml --workflow` exits 1 and names `docs/metrics/workflow.jsonl`. TRUE, and PRE exits 0 there, so the rewording was needed.
- "plain `validate` without `--workflow` is unaffected and still notes an absent log on stderr at exit 0" (`README.md:210`): TRUE, and item 2's 35-input sweep is the general form of that claim.

`pack/AGENTS.md:93`'s "that check exits non-zero reporting that it could not run rather than passing" is also true as measured; the emitted sentence contains "the workflow check could not run". Round 2 settled the wording of these three sites under `X-1` and I did not reopen it.

### 9. WHAT I DID NOT ATTACK, stated so the reader is not misled about coverage

- The Windows path. `cfg(unix)` gates the relevant test and I have no Windows host; `metadata` semantics there are not measured by anything above.
- Concurrency beyond the two FIFO cells. I did not attack the gate with a filesystem racing at the ~microsecond window between `:846` and `:847` (probe then read), because the FIFO widener does not reach it. A read failing immediately after a true probe lands in the `?` propagation at `:847` rather than the arm, which is the queued pre-existing message shape from item 5.
- Real-versus-effective-uid divergence (setuid). Ruled out as unreachable from a CLI once the std source showed `metadata`, not `access`, is the underlying call, so both would use the effective uid regardless.
- The scaffolded pack's rendered output beyond what the suite's drift-guard tests cover, since `just scaffold-self` is forbidden here. The suite is green at 422/0, which includes those guards.
- The ledger, which is not the reviewed product. Round 2's `X-2` settled that a ledger defect is reported to the orchestrator and never counted in a round; I read the ledger for decisions and residuals and raise nothing against it.

## Relitigation and constraints check

Nothing here re-raises the four standing residuals (the in-root bound, the single-anchor `..` case, the earlier increment's rejected-ledger context slot, the off-convention `--source` surface), accepted costs (i) to (iv), round 1's `ADV-4` or `SC-3`, round 2's `R2A-4`, `R2B-2` or `R2B-3`, or the queued plain-`validate` inconsistency. Each is named above where I met it and passed over it.

ONE VERDICT-ADJACENT CORRECTION IS MADE EXPLICITLY, per the brief's instruction to say so and give the evidence: `R3A-1` measures FALSE the round 2 triage's statement at `...-r2-triage.md:182` that the appended clause "applies to every errno including EACCES, where 'pass a `--metrics` naming this project's log' is also useful advice." `V-3`'s verdict itself stands; one claim inside its remedy does not.

No line-length, wrapping or comment-raggedness observation appears anywhere in this file.

The reviewed worktree was never edited: `git status --short` is empty in it and in the main repository at `/home/jessea/Documents/projects/agent-scaffold`, and was empty throughout. All four binaries were built in directories of my own under `<scratch>/r3adv/build/`. No `nix fmt` and no `just scaffold-self` was run.

## Tally

| Severity | Count | Findings |
| --- | --- | --- |
| critical | 0 | |
| high | 0 | |
| medium | 0 | |
| low | 3 | `R3A-1`, `R3A-2`, `R3A-3` |

`R3A-1` is fix-induced by `ce820fb`, the branch's newest commit. `R3A-2` is fix-induced by `7f2e3c3`. `R3A-3` is PRE-EXISTING, measured byte-identical on NEW, BASE and PRE, and offered for routing rather than for a fix in this increment.

REMEDY SHAPE: `R3A-1`'s preferred remedy is a nine-word DELETION, `R3A-2`'s is a substitution reusing wording that already exists elsewhere in the tree, and `R3A-3` asks for no code change at all. No new sentence is authored anywhere in this round's remedies, and a fix pass that took only `R3A-2` would be defensible.
