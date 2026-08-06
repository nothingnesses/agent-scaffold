# `workflow-enforcement-tier-inc3` work review, round 2, REVIEWER: ADVERSARIAL CONSTRUCTION

Reviewed on branch `review/inc3-r2-adversarial` at `141cf1c`. Governing specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`. Round 1's triage (`...-inc3-workreview-r1-triage.md`) was read first and every verdict in it was treated as settled.

## Method

Nothing below is adjudicated by reading the diff. Three binaries were built and every claim rests on running them:

- NEW: `141cf1c`, built in this worktree at `target/debug/agent-scaffold`.
- PREFIX: `60679ca`, the branch tip immediately before the round 1 fix `4801898`, exported with `git archive` into `<scratch>/build/prefix` and built independently. This is the increment as it stood before the fix pass touched `src/`.
- PRE: `9eeca42`, which predates the whole increment, exported and built the same way. Verified that `git diff 9eeca42 18176fa -- src/ tests/` and `git diff 9eeca42 dd947a7 -- src/ tests/ Cargo.toml` are both EMPTY, so a binary from `9eeca42` is byte-for-byte the same product as one from `18176fa` (the hash the brief names) and from `main`; the commits between them are documentation only.

A FOURTH binary, REMEDY, was built to measure the candidate fix under `R2A-2`. It was produced by exporting `HEAD` into `<scratch>/build/remedy` and editing THAT COPY. The reviewed worktree was never edited: `git status --short` on it is empty and was empty throughout.

All fixtures live under `<scratch>/r2adv/fix/`. Every directory chmodded to 600 or 000 was chmodded back; a closing `find ... ! -perm -u+rwx -type d` over the fixture root and the test TMPDIR returned nothing. `TMPDIR` was pointed at `<scratch>/r2adv/tmpdir`, outside any repository, for every `cargo test` run.

GATES on the tree as reviewed: `cargo test` 422 passing across nine binaries, 0 failing; `cargo clippy --all-targets -- -D warnings` clean; `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` reports `up to date`.

`<scratch>` abbreviates `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad`. The fixture project in every reproduction below is a TOML-primary plan at `docs/plans/p.plan.toml` with one `not-started` step and a real one-record round log at `docs/metrics/workflow.jsonl`, unless the reproduction says otherwise.

FOUR FINDINGS: one `medium`, three `low`. No `high` and no `critical`. The increment's own contract, the exit status on the tier boundary, is CORRECT on every input I constructed: I found no cell in which `--workflow` reports success over a check that did not run, and no cell in which it exits non-zero over a check that did run and passed.

---

## `R2A-1` `medium`: the fix pass's new test fails when the suite runs as root, because its exit-code assertion sits outside the guard the test computed for exactly that case

### Claim

`a_round_log_that_cannot_be_checked_is_not_reported_as_missing` measures `opaque` to stay correct where mode 600 does not hide the entry, then asserts `code == Some(1)` OUTSIDE the `if opaque` block, so in that environment the test fails on an assertion whose message misdescribes what happened.

### The code

`tests/validate_workflow_toml_source_needs_no_plan.rs`:

```
305     let opaque = fs::metadata(metrics.join("workflow.jsonl")).is_err();
306     let (code, stdout, stderr) = validate(...);
309     fs::set_permissions(&metrics, fs::Permissions::from_mode(0o755)).unwrap();
313     assert_eq!(code, Some(1), "a check that could not run must still refuse; ...");
318     if opaque {  ... the three message assertions ... }
```

The doc comment above it says "as root it does not [hide the entry], and then there is nothing to say", and `4801898`'s commit message repeats the claim. But line 313 does say something, and when `!opaque` what it says is wrong: the log IS readable, the check DOES run, `PLAN_TOML`'s only step is `not-started` so W3 has nothing to enforce, and the run exits 0.

### Evidence

The suite as the real user, and the same suite inside a user namespace where the process is uid 0 and DAC checks are bypassed for its own files (`unshare -Ur`, a faithful stand-in for the container-as-root case, and verified first: `stat` on the fixture file is `Permission denied` as the user and succeeds in the namespace):

```
$ export TMPDIR=<scratch>/r2adv/tmpdir
$ cargo test
   378 / 5 / 1 / 1 / 9 / 3 / 20 / 1 / 4 passing, 0 failing        (422 total)

$ unshare -Ur env PATH="$PATH" TMPDIR=$TMPDIR HOME="$HOME" cargo test
   378 / 5 / 1 / 1 / 9 / 3 / 20 / 1 passing, 0 failing            (eight binaries clean)
   test a_round_log_that_cannot_be_checked_is_not_reported_as_missing ... FAILED
   panicked at tests/validate_workflow_toml_source_needs_no_plan.rs:313:5:
   assertion `left == right` failed: a check that could not run must still refuse; stdout:
   docs/metrics/workflow.jsonl: 1 records, valid
   docs/plans/p.plan.toml: 1 steps, 0 questions, valid
   docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
     left: Some(0)
    right: Some(1)
   test result: FAILED. 3 passed; 1 failed
```

ATTRIBUTION, which is the part that makes this a round 2 finding rather than a suite property. The SAME command against PREFIX (`60679ca`, the increment before the fix pass):

```
$ cd <scratch>/build/prefix
$ unshare -Ur env PATH="$PATH" TMPDIR=$TMPDIR HOME="$HOME" cargo test
   378 / 5 / 1 / 1 / 9 / 3 / 20 / 1 / 3 passing, 0 failing        (421 total, all green)
```

421 of 421 pass as namespace root before the fix pass; 421 of 422 pass after it, and the one that does not is the test the fix pass added. No other test in this repository has this property. It is NOT the known `test-tmpdir-repo-assumption` class: `TMPDIR` is outside a repository in both runs and the three tests that assert that pass in both.

The failure leaves the fixture directory behind (the panic skips `fs::remove_dir_all` at line 343), but at mode 755, because line 309 restores the mode BEFORE the assertion. So the specific hazard the commit message claims to have designed against, an undeletable fixture, is genuinely absent, and every other test in this file leaks the same way on failure. That part is not a finding.

### Why `medium`

The product is untouched: no user of the binary is affected, and a human running the suite as themselves sees green. That caps it below `high`.

It is above `low` because `cargo test` is one of this project's four gates, the failure is in the newest material, and root-in-a-container is not an exotic environment: it is the default in most CI images, and this repository's own AGENTS.md directs every spawned agent into container isolation as the strongest tier. A gate that goes red there, with a message asserting "a check that could not run must still refuse" about a check that ran fine, costs whoever hits it a real investigation. The remedy is one line moved.

### Right behaviour and smallest remedy

Move line 313 inside the `if opaque` block at 318. The control at 335 to 342 is unaffected and still passes in both environments, so the test keeps its meaning where the mode bites and says nothing where it does not, which is what its own doc comment promises. No new prose, no new fixture, one assertion relocated.

---

## `R2A-2` `low`: the arm's sentence describes a SECOND, LATER stat than the one that produced the exit code, so the gate and the branch can disagree in both directions, and one of those disagreements is verbatim `T-1`'s falsehood reached through the branch `T-1`'s fix added

### Claim

`src/main.rs:845` decides `metrics_contents` from `metrics_path.exists()`; `src/main.rs:1066` decides WHICH SENTENCE to print from a fresh `metrics_path.try_exists()` taken later. The exit code comes from the first observation and the diagnosis from the second, so when the filesystem changes between them the run reports a state that did not decide anything.

### The cell enumeration, and which cells are reachable

`Path::exists()` is `fs::metadata(p).is_ok()`. `Path::try_exists()` is the same `fs::metadata` call with `Err(NotFound) -> Ok(false)` and every other `Err` propagated. Same syscall, two moments.

| Gate at 845 | Arm at 1066 | Reached how | Message | True? |
| --- | --- | --- | --- | --- |
| `metadata` Ok | (not reached) | log readable | the check runs, or `read_to_string` propagates | n/a |
| Err(ENOENT) | `Ok(false)` | log absent throughout | "no round log at X" | YES |
| Err(other) | `Err(e)` | unreadable throughout | "could not be checked (e)" | see `R2A-3` |
| Err(ENOENT) | `Ok(true)` | log CREATED mid-run | "no round log at X ... record the project's review rounds there" | NO |
| Err(ENOENT) | `Err(e)` | path became unreadable mid-run | "could not be checked (e)" | NO |

The implementer mapped `Ok(true)` to the absent message with the catch-all `Ok(_)`, and the arm's comment states "`Ok` asserts absence". THE CLAIM THAT `Ok(true)` IS RACE-ONLY IS CORRECT; I could not reach it with a static filesystem. What was mis-assessed is the consequence: the race's output is exactly the sentence and exactly the prescription that `T-1` and `Q-55-emptyroot` were decided against.

### Evidence: the `Ok(true)` cell, constructed deterministically

A FIFO at the `--source` path makes the race reproducible rather than lucky. `run_validate` stats the log at line 845 before it reads the source at line 880, and `fs::read_to_string` on a FIFO blocks at `open` until a writer appears, so the window between the two stats is under my control. The race itself needs no FIFO; the FIFO only widens it.

```
$ cd <scratch>/r2adv/fix/c            # docs/metrics/ exists, NO log in it
$ mkfifo docs/plans/p.plan.toml
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow &   # blocks on the FIFO
$ sleep 1 && cat stderr-so-far
no metrics log at docs/metrics/workflow.jsonl; nothing to validate       # the gate has ALREADY answered
$ printf '%s\n' '{"type":"round","task":"only-step",...}' > docs/metrics/workflow.jsonl
$ cat plan-body.toml > docs/plans/p.plan.toml                            # unblock the run
$ wait
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run,
  so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record
  the project's review rounds there
exit=1
$ cat docs/metrics/workflow.jsonl
{"type":"round","task":"only-step","step":"only-step",...,"outcome":"clean",...}
```

The log is there, with a real round record in it, and the run tells its operator to record the rounds that are already recorded. That is `T-1`'s sentence, `T-1`'s remedy clause and `T-1`'s falsehood, produced by the `Ok(_)` branch the `T-1` fix introduced.

PREFIX (`60679ca`) prints the identical sentence on this fixture, and PRE (`9eeca42`) prints the old skip note at exit 0, so this cell is not a REGRESSION in output; what is new is that the fix pass added a branch whose stated job is to prevent this sentence and routed this cell into it.

### Evidence: the opposite disagreement, which also proves the arm re-stats

Same fixture and same FIFO, but the log is genuinely absent and STAYS absent; only the directory's mode changes after the gate has already answered `Ok(false)`:

```
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow &   # blocks; log absent, dir 755
$ sleep 1 && cat stderr-so-far
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
$ chmod 600 docs/metrics && cat plan-body.toml > docs/plans/p.plan.toml
$ wait
--workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked
  (Permission denied (os error 13)): the workflow check could not run, so it cannot report that the
  invariants hold
exit=1
$ ls -A docs/metrics                                                     # (empty: no log, ever)
```

The tool DID answer the question, successfully, at the moment it mattered, and then reports that it could not answer it. This half is a REGRESSION: PREFIX has no such branch and says "no round log at ...", which is true here.

### Why `low` and not higher

Both cells require the filesystem to change mid-run, and I could not reach either statically. The exit code is 1 in every cell and is correct in every cell, so no CI gate is misled and no false green exists anywhere in this finding. The reachable real-world shape is narrow: a concurrent first append to a project's round log during the window in which `validate --workflow` reads its plan source, which happens at most once in a project's life.

It is not zero because the arm's own comment ("`Ok` asserts absence") is false as written about a live branch, and because the increment's stated purpose is that this surface not assert things it cannot know.

### Right behaviour and smallest remedy, MEASURED

Ask once and describe the answer you asked for. Two lines:

```rust
let metrics_probe = metrics_path.try_exists();
let metrics_contents = if matches!(metrics_probe, Ok(true)) {   // was: if metrics_path.exists()
...
_ => problems.push(match &metrics_probe {                       // was: match metrics_path.try_exists()
```

`matches!(try_exists(), Ok(true))` is `metadata().is_ok()` by definition, so the gate's predicate is UNCHANGED, `Ok(true)` becomes unreachable in the arm BY CONSTRUCTION rather than by argument, and the `Ok(_)` catch-all becomes provably `Ok(false)`. Measured on the REMEDY binary:

- The opposite-disagreement cell above now prints "no round log at docs/metrics/workflow.jsonl ...", exit 1. The regression is gone.
- The static unsearchable-ancestor case, the input `Q-55-existsgate` was decided on, still prints "could not be checked (Permission denied (os error 13))", exit 1. The fix's own purpose is preserved.
- Plain `validate` is BYTE-IDENTICAL to PRE (`9eeca42`) on all 19 inputs of the sweep in the "what produced nothing" section below, including all four directory modes and every errno class. `Q-55-existsgate`'s promise is untouched, which is the ground the one-token alternative was declined on.
- `cargo test` 422 passing, 0 failing. `cargo clippy --all-targets -- -D warnings` clean.

This is inside the tier boundary the increment's own comment draws: it touches no surface other than the one that asked for the check, and it authors no new prose.

---

## `R2A-3` `low`: the new `Err` branch disclaims knowledge the tool has, and drops the remedy clause, for the three errno classes where absence IS established

### Claim

The `Err` arm was designed for EACCES, the one class round 1 measured as having a real log behind it. It fires on every errno `stat` can return, including ENOTDIR, ENAMETOOLONG and ELOOP, where nothing can exist at the path and the pre-fix sentence was true; for those it replaces a true sentence carrying a two-part remedy with "could not be checked", carrying none.

### Evidence

```
$ agent-scaffold validate --source docs/plans/p.plan.toml \
    --metrics docs/plans/p.plan.toml/workflow.jsonl --workflow
NEW     --workflow requested but the round log at docs/plans/p.plan.toml/workflow.jsonl could not be
        checked (Not a directory (os error 20)): the workflow check could not run, so it cannot report
        that the invariants hold                                                             exit=1
PREFIX  --workflow requested but no round log at docs/plans/p.plan.toml/workflow.jsonl: the workflow
        check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming
        this project's log, or record the project's review rounds there                      exit=1
PRE     --workflow has a plan source but the metrics log is missing; skipping the workflow check
                                                                                             exit=0
```

Reproduced identically for ELOOP (`--metrics docs/loop/loopy` where `loopy -> loopy`, "Too many levels of symbolic links (os error 40)") and for ENAMETOOLONG (a 300-character leaf, "File name too long (os error 36)"). The dangling-symlink case is CORRECT and is the control: `metadata` follows the link, gets ENOENT, `try_exists` returns `Ok(false)`, and NEW prints "no round log at ..." exactly as it should.

Round 1's triage enumerated these same three errnos and ruled, on measurement, that "no round log at X" is "terse rather than false" for each because no log sits behind them. The fix routes exactly those three into the sentence written for the one class where a log DOES sit behind the error.

The most likely way to reach ENOTDIR is a mistyped `--metrics`, and that operator now gets an errno and no instruction, where before they were told to pass a `--metrics` naming this project's log. "Could not be checked" also points the reader at the tool rather than at their own argument.

### Why `low`

The exit code is right, the errno is named, and a reader who knows what ENOTDIR means is not misled about the filesystem. This is a message-quality regression on a narrow input class, not a wrong answer. I also record honestly that round 1's triage RECOMMENDED this remedy shape ("on `Err` say the check could not be performed and name the error"), so this finding is about what the shape does on inputs that recommendation did not separate, not about a departure from it.

### Right behaviour and smallest remedy

Two options, and doing nothing is defensible:

1. Keep the remedy clause in the `Err` branch, so the operator is still told what to do: append "; pass a `--metrics` naming this project's log" to the `Err` format string. One clause, no new sentence, applies to every errno.
2. Narrow the `Err` branch to the errors that genuinely leave the question open (`PermissionDenied`, and anything not in the definitively-absent set) and let ENOTDIR / ENAMETOOLONG / ELOOP keep the absent sentence. This is the more accurate split and the larger change; it authors a classification the tree does not have today, so I do not recommend it against option 1.

---

## `R2A-4` `low`: the fixed `--workflow` run still prints "no metrics log at <path>" for the very log it then says it could not check, and the new test's guard is written against the one spelling that changed

### Claim

On the `--workflow` surface at HEAD, stderr contains the corrected sentence one line BELOW an uncorrected one that asserts the same absence the fix removed, and the new test's assertion (`!stderr.contains("no round log at")`) does not see it.

### Evidence

The fix's own fixture, mode 600 on `docs/metrics`, log present with a real round record:

```
$ ls docs/metrics
workflow.jsonl                                    # the log is right there, same user, same shell
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate                         <-- line 1
--workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked
  (Permission denied (os error 13)): the workflow check could not run, so it cannot report
  that the invariants hold                                                                 <-- line 2
exit=1
```

Line 1 asserts there is no log at a path where a log with review evidence is sitting. Line 2 says the tool cannot tell. Both describe the same path in the same run.

The new test asserts `!stderr.contains("no round log at")` with the message "the log is on disk, so this sentence is false". The stderr it inspects contains `no metrics log at docs/metrics/workflow.jsonl`, which is the same false claim in different words, and the assertion passes over it. So the test pins the fix rather than the property the fix exists to establish.

PREFIX prints both lines with line 2 also false. PRE prints line 1 plus the skip note at exit 0. Line 1 itself is unchanged across all three builds.

### Scoping this against what is settled, explicitly

I am NOT raising the queued plain-`validate` inconsistency (a mode-000 log FILE hard-erroring at exit 1 while an unsearchable DIRECTORY notes at exit 0). I reproduced it, and I am not asking for it to change. I am also not asking for the gate at line 845 to change: `Q-55-existsgate` decided that, `R2A-2`'s remedy deliberately preserves it, and the sweep below confirms plain `validate` is byte-identical to PRE.

What is new here is that `Q-55-existsgate`'s promise and the ledger's byte-identity paragraph (`agent-scaffold.ledger.md:539`) are both about PLAIN `validate`, and neither examined the `--workflow` run's stderr AS A WHOLE. That output is the increment's own surface, and it still carries the asserted absence.

### Why `low`

The exit code is right, the true sentence is present, the false one carries no remedy and prescribes no action, and a reader who gets to line 2 is correctly informed. Nobody is sent to fix a correct path, which is what made `T-1` a `medium`.

### Right behaviour and smallest remedy

If the human reads `Q-55-existsgate` as covering the note on this surface too, then the only actionable half is the TEST, and it is cheap: assert the absent CLAIM is not made in any spelling, for example by also asserting `!stderr.contains("no metrics log at")` inside the existing `if opaque` block, or by re-wording the assertion to the substring both sentences share. That either pins the property or shows immediately that it does not hold, rather than passing quietly.

If the note is considered in scope, the note itself would have to move behind the same probe, which is `R2A-2`'s single-probe binding plus one branch at line 866, and that DOES change plain `validate`, so it is a decision and not a reviewer's instruction.

---

## What I attacked that produced nothing

A zero-finding claim is only credible against a stated attack surface, so here is what did not break.

1. **PLAIN `validate` BYTE-IDENTITY, 19 INPUTS.** `Q-55-existsgate`'s promise holds far beyond acceptance check 16's single fixture. NEW versus PRE (`9eeca42`), stdout, stderr and exit code compared as one string, all IDENTICAL: `docs/metrics` at modes 755, 600, 000 and 111, each with the default and with an explicit `--metrics`; an absent log; ENOTDIR; ELOOP; ENAMETOOLONG; a dangling symlink; the log path being a directory; no anchors at all; an empty `--metrics`; a missing `--source`; a `--metrics` outside the root; and a mode-000 log FILE. The arm-scoping did what it promised.
2. **THE CORRECT CASE AND ACCEPTANCE CHECKS 15 AND 18.** NEW and PREFIX are byte-identical on the genuinely absent log (check 15) and on the correct case; NEW and PRE are byte-identical on the correct case. The fix changed only the cells it meant to.
3. **THE MARKDOWN ARM.** `validate --plan docs/plans/p.md --workflow` on the unsearchable-ancestor fixture reaches the same split as the TOML arm and prints the same corrected sentence, so the `_` arm is genuinely shared. The new test only exercises the TOML arm, but the older `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` covers both for the absent case and the branch is provably one arm, so I do not raise the coverage gap.
4. **PRECEDENCE, FOUR WAYS.** A malformed `--workflow-spec` plus an unreadable log reports the spec error and never reaches the round-log problem. `--workflow` with no plan source plus an unreadable log reports "no plan source resolved". An explicit `--metrics` outside the plan's root plus an unreadable log reports the containment refusal, identically on NEW, PREFIX and PRE. A `..` spelling that stays inside the root (`docs/plans/../metrics/workflow.jsonl`) is correctly NOT refused and reaches the new split; the same `..` spelling that leaves the root IS refused. In every combination the check that answered first was the most useful true one.
5. **THE CONTAINMENT GUARD IS NOT DISTURBED BY AN UNREADABLE PATH.** I expected `resolve_for_containment`'s `fs::canonicalize` walk to fail-open or fail-closed under mode 600 and produce a false "not under the plan's project root". It does neither: canonicalizing `docs/metrics` needs search permission on `docs`, not on `docs/metrics` itself, so the longest-existing-ancestor walk resolves and the predicate answers correctly. No input made the containment refusal fire on an in-root log or fail to fire on an out-of-root one.
6. **A DIVERGENT `--source`/`--plan` PAIR ACROSS TWO PROJECTS, with the local log unreadable.** The TOML source drives, its own root and own log are used, the Markdown plan is correctly reported as a projection, and the run is a true green over the right project's evidence. No leakage.
7. **`status`, `next` AND `status --resume`.** Run on the unsearchable-ancestor fixture, NEW and PRE produce byte-identical stdout, stderr and exit code on all three. The increment does not reach them. (They print `metrics: no log found` for a log that is there, which is the same `Path::exists` collapse; it is identical on the pre-increment build, those surfaces are outside this increment, and I am not raising it.)
8. **THE ERROR-PROPAGATING CASES.** A mode-000 log FILE (`Error: Os { code: 13 }`) and a directory at the log path (`Error: Os { code: 21 }`) both propagate at exit 1, identically on NEW, PREFIX and PRE. They never reach the new arm, so the fix cannot have moved them, and it did not.
9. **THE ROUND 1 DOCUMENTATION CLOSURES, RE-RUN NOT RE-READ.** `once built` is gone from `pack/AGENTS.md:93` and its two deployed renders (`T-4`); all three sites plus `README.md` and `CHANGELOG.md` now name the population by the LOG with the flag as the case that stays in it (`T-2`); `README.md:234` reads "fails, naming the log it looked for" on the example command that does exit 1 (`T-5`); the CHANGELOG `Added` entry no longer claims `--workflow` requires `--plan`, which the suite pins (`T-6`); `PLAN_MD` now spells `not started` (`T-3`). `render --check` reports up to date and the drift guards pass, so the deployed copies were regenerated.
10. **A NON-RACE ROUTE TO THE `Ok(true)` CELL.** I tried to reach it statically and could not, which CONFIRMS the implementer's stated ground. `exists()` and `try_exists()` issue the same `fs::metadata` call and differ only in how they map its result, so with a fixed filesystem their answers cannot disagree. The finding under `R2A-2` is about what happens when they do, not about the claim being wrong.
11. **THE NEW TEST'S PERMISSION CLEANUP ON THE FAILURE PATH.** It genuinely restores mode 755 before its assertions, so an assertion failure cannot leave an undeletable fixture. Verified by inducing the failure under `unshare` and inspecting what was left: a mode-755 directory, removable, and every other test in the file leaks the same way. Not a defect.

## Relitigation check

I checked every finding above against the settled list before writing it. None of the four re-raises the four residuals (the in-root bound, the single-anchor `..` case, the earlier increment's rejected-ledger context slot, the off-convention `--source` surface), accepted costs (i) through (iv), round 1's `ADV-4` (the empty file yielding an affirmative green, which I reproduced as a control and do not raise), round 1's `SC-3`, or the queued plain-`validate` inconsistency. `R2A-4` is adjacent to that last one and says so in its own text, with the scoping argument stated rather than assumed. I have NO new evidence that any round 1 verdict was wrong.
