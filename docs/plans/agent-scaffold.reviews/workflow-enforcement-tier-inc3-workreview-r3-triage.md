# `workflow-enforcement-tier-inc3` work review, ROUND 3, TRIAGE

Triaged on branch `triage/inc3-r3` at `09c4710` in worktree `.claude/worktrees/triage-inc3-r3`. `09c4710` is `ce820fb` (the tip both reviewers reviewed) plus the two round 3 findings files and nothing else: `git diff --stat ce820fb..HEAD` touches only `docs/plans/agent-scaffold.reviews/`, so the product under my binaries is byte-identical to the product under theirs.

Governing specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`. Round 1's triage and round 2's triage were read in full before any verdict, and every ruling in them is treated as settled except where this file says otherwise and shows the measurement.

## Method

TOOLCHAIN, confirmed before every build-dependent claim: `cd <worktree> && direnv allow && eval "$(direnv export bash)"` puts `which cargo` at `/nix/store/76jaab43a2l7n7fiifxjngp68kk167vm-rust-mixed/bin/cargo`, `cargo 1.98.0-nightly (a335d47ff 2026-06-26)`, `rustc 1.98.0-nightly (f46ec5218 2026-06-30)`. No claim below rests on a build made outside that environment, and no `2>/dev/null` was used on the `direnv export` call.

THREE BINARIES, each built by me from source in my own directory:

| Name | Commit | Location | What it is |
| --- | --- | --- | --- |
| NEW | `09c4710` | the triage worktree's `target/debug` | the tip under review |
| BASE | `bd5bd47` | `<scratch>/tri-r3/build/base` | before the round 2 fix pass |
| PRE | `9eeca42` | `<scratch>/tri-r3/build/pre` | before the whole increment |

`<scratch>` abbreviates `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad`. All fixtures live under `<scratch>/tri-r3`, a directory of my own naming; every directory chmodded to 600 or 000 was chmodded back, and a closing `find <scratch>/tri-r3 -type d ! -perm -u+rwx` plus `find <scratch>/tri-r3 -type f -perm 000` both return nothing. `TMPDIR` was `<scratch>/tri-r3/tmpdir`, outside any git repository, for every `cargo test`.

GATES MEASURED AT `09c4710`: `cargo test` 422 passed / 0 failed across 9 binaries; `cargo clippy --all-targets -- -D warnings` exit 0; `agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow` on this repository's own plan reports `workflow invariants hold` at exit 0.

ONE TEMPORARY SOURCE EDIT was made and reverted: the nine-word deletion `R3A-1` proposes, applied to `src/main.rs:1073` to measure whether any test guards it (see "What the reviewers missed"). Reverted with `git checkout -- src/main.rs`; nothing was lost, since the edit is exactly the remedy `R3A-1` already records. `git status --short` is empty at the time of writing.

Nothing below is adjudicated by reading. `Q-66` governs: every verdict rests on a command run against a binary I built, with the fixture and its output shown.

---

# Part 1: the `R3A-1` / `R3B-1` interaction, resolved as ONE decision

The two findings are about the same nine words. `R3A-1` wants them deleted from the `Err` arm's message (`src/main.rs:1073`); `R3B-1` wants the word "only" deleted from the comment above that arm (`src/main.rs:1060`), because those same nine words falsified it. If `R3A-1`'s deletion lands, `R3B-1` evaporates. So this is one decision.

## The measurement that decides it, and neither reviewer ran it

`R3A-1` reproduces exactly as reported. The fixture is a TOML-primary plan at `docs/plans/p.plan.toml` and a real one-record log at `docs/metrics/workflow.jsonl`, with `chmod 600 docs/metrics` (readable, not searchable):

```
$ ls -A docs/metrics
workflow.jsonl
$ stat docs/metrics/workflow.jsonl
stat: cannot statx 'docs/metrics/workflow.jsonl': Permission denied

STEP 1  $ NEW validate --source docs/plans/p.plan.toml --workflow
  --workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked
  (Permission denied (os error 13)): the workflow check could not run, so it cannot report
  that the invariants hold; pass a `--metrics` naming this project's log            exit=1

STEP 2  $ NEW validate --source docs/plans/p.plan.toml --workflow \
              --metrics docs/metrics/workflow.jsonl        # obey the instruction literally
  (identical, verified with diff on stdout, stderr and exit code)                   exit=1
```

BYTE-IDENTICAL, confirmed by `diff` rather than by eye. BASE names the errno and stops (no clause, so nothing to obey uselessly); PRE prints the old skip note at exit 0. So the clause is fix-induced by `ce820fb`, as reported.

THE MEASUREMENT NEITHER REVIEWER RAN, and it is the one that matters. `R3A-1` calls EACCES "the one input this arm was created to serve" and generalises the no-op to the whole errno class. EACCES is not one input. Round 1's own triage recorded the reachable EACCES set as "an ancestor directory of the resolved metrics path denies search permission", explicitly including the explicit-`--metrics` case ("verified with `--metrics docs/metrics/sub/log.jsonl` under a mode-000 `sub`"). That second sub-case is where the advice bites. A project whose real log is readable at `docs/metrics/workflow.jsonl` and whose operator names a path under an unsearchable directory:

```
$ chmod 600 docs/locked
$ NEW validate --source docs/plans/p.plan.toml --workflow --metrics docs/locked/workflow.jsonl
  --workflow requested but the round log at docs/locked/workflow.jsonl could not be checked
  (Permission denied (os error 13)): ... ; pass a `--metrics` naming this project's log  exit=1

$ NEW validate --source docs/plans/p.plan.toml --workflow --metrics docs/metrics/workflow.jsonl
  docs/metrics/workflow.jsonl: 1 records, valid
  docs/plans/p.plan.toml: 1 steps, 0 questions, valid
  docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold        exit=0
```

The operator obeyed the same instruction, on the same errno, and it converted a correct refusal into a correct pass. So the clause is LIVE on EACCES, not inert on EACCES. What is inert is the clause on the DEFAULT-ANCHORED EACCES sub-case, where the resolved path already is the project's own log. `R3A-1` varied the errno and held the anchoring fixed, so its fixture could not discriminate the two sub-cases: the DIMENSIONS failure this project's brief names.

## Round 2's triage was OVER-BROAD on this, not wrong, and its verdict stands

Plainly, since the brief asks for it. Round 2's triage at `...-r2-triage.md:182` wrote that the clause "applies to every errno including EACCES, where 'pass a `--metrics` naming this project's log' is also useful advice." It asserted that of the EACCES class without running it, which `R3A-1` is right to object to. Measured: the claim is TRUE of the explicitly-named-`--metrics` EACCES sub-case above and FALSE of the default-anchored sub-case, and round 2 did not distinguish them. It is over-broad, not false. `R3A-1`'s own statement that "STEP 2 above measures that claim false" is over-broad in the mirror direction, for the same reason: one sub-case cannot falsify a class claim when another sub-case satisfies it.

`V-3`'S VERDICT STANDS UNCHANGED, VALID at `low`, and so does its shipped remedy. `V-3` was about ENOTDIR, ELOOP and ENAMETOOLONG losing a true sentence and its remedy clause; all three reproduce with the clause and all three are genuinely helped by it (see the closure spot-check of `V-3` below), and EACCES adds a fourth helped class. Only one supporting sentence inside round 2's remedy paragraph was over-stated.

## The single remedy, and what each candidate REMOVES

The brief asks which order leaves the fewest words and the fewest claims. Those two criteria disagree, and word count is the wrong one.

| Candidate | Words removed | Claims removed | Claims left false |
| --- | --- | --- | --- |
| A. `R3A-1`'s deletion alone (`R3B-1` then evaporates) | 9, from the product's own output | one TRUE and LIVE remedy claim, on 4 of the 5 reachable classes (ENOTDIR, ELOOP, ENAMETOOLONG, mis-named EACCES) | none |
| B. `R3B-1`'s deletion alone | 1, from a source comment | one FALSE exhaustiveness claim, and nothing else | none |
| C. Both | 10 | A's four plus B's one | none |

A is the fewest words and it is the wrong answer. It removes four true, live claims from user-visible output in order to remove one inert (not false) hint from a fifth class, and it regresses `V-3`, a finding round 2 ruled VALID and shipped. The brief's own test is what the remedy REMOVES, and A removes strictly more than the defect it cures. C is A plus a redundant edit.

TAKE B AND ONLY B. It removes exactly the one claim that is false and nothing that is true, it authors zero words, and it is SAFE UNDER BOTH FUTURES: if a human later overrules the residual below and takes A after all, the comment reading "`Err` says that the question could not be answered and names the error" is still true, so B never has to be undone. A does not have that property in reverse. There is therefore no ordering question: one remedy, one word, no sequencing constraint.

---

# Part 2: the findings, one section each

## `R3A-1` ACCEPT-AS-RESIDUAL, `low`: the `Err` arm's `--metrics` clause is inert when the resolved path is the project's own unreadable log

REPRODUCED: yes, in full, above. The byte-identical no-op on the default-anchored mode-600 fixture is real and is fix-induced by `ce820fb`.

REASONING. The observation is true and narrower than the finding states. The clause is an imperative, not an assertion, so it is not FALSE anywhere; it is inert on exactly one of the five reachable classes that reach this arm, and live on the other four (measured: ENOTDIR, ELOOP, ENAMETOOLONG in the `V-3` spot-check below, and mis-named EACCES above). The message that carries it already names the true cause on the inert case, `Permission denied (os error 13)`, which is the one thing that actually tells the operator what to do. The cost of leaving it is what the reviewer says: one wasted command, no wrong exit code, no false green.

Every available remedy costs more than that. The reviewer's option 1 (split the clause off EACCES) authors an errno classification this tree does not have anywhere, which round 2 rejected on `V-3` for a reason that still holds and which this project has five retrospective and one prospective measurement against; and it would now have to split EACCES itself, since EACCES lands on both sides of the line. The reviewer's option 2 (revert `ce820fb`) is candidate A above, measured as a net removal of true claims. The reviewer itself says it "would understand a human choosing to do NOTHING" and that this "is the finding I would drop first".

NOT INVALID, deliberately. The observation that a remedy clause can be inert when the tool cannot see whether the path it is talking about is already right is a genuine property of the mechanism, and a later round should not have to rediscover it. Accepting it records it.

SMALLEST REMEDY: none. No `src/` change, no prose change. If the human wants it on the record, it belongs in the ledger's residual list beside `R2A-4`, in one line: on an unreadable log at the anchored default path, the `Err` arm's `--metrics` clause is inert, because the path it invites the operator to correct is already the project's own.

## `R3A-2` VALID, `low`: the test's doc comment names a gate the increment replaced

REPRODUCED. `tests/validate_workflow_toml_source_needs_no_plan.rs:267` reads "The gate the policy above hangs on is `Path::exists`, which is `fs::metadata(..).is_ok()` and so answers false both for a log that is not there and for one behind a directory the process cannot traverse." The gate is no longer `Path::exists`:

```
$ grep -n "\.exists()\|try_exists()" src/main.rs
845:	let metrics_probe = metrics_path.try_exists();      <- the metrics gate
880:		if source_path.exists() {                        <- the plan-source gate
924:		if plan_path.exists() {
1237:	} else if metrics_path.exists() {                    <- inside run_status, a DIFFERENT function
```

`metrics_path.exists()` appears nowhere in `run_validate`. The nearest match a grepping reader lands on, `:1237`, is inside `run_status` (`grep -n "^fn " src/main.rs` puts `run_status` at `:1186`), which is exactly the wrong place to be sent.

FIX-INDUCED WITHIN THIS INCREMENT, established from history rather than assumed:

```
$ git show 9eeca42:src/main.rs | grep -n "metrics_path.exists()"   ->  837 (PRE: the gate)
$ git show bd5bd47:src/main.rs | grep -n "metrics_path.exists()"   ->  845 (BASE: the gate)
$ git log --oneline -S "let metrics_probe = metrics_path.try_exists();" main..HEAD -- src/main.rs
a5dc579 fix: describe the round log observation the exit code was decided on
$ git log --oneline -S "The gate the policy above hangs on is \`Path::exists\`" --all -- tests/...
65049fa fix: separate a round log the check cannot see from one that is not there
```

So the sentence was TRUE when `65049fa` wrote it and became false at `a5dc579`, both inside this increment. The same pass edited `src/main.rs`'s corresponding comment and left the test file's copy behind. The parallel `Path::exists` sentence at `tests/unsafe_pairings_are_refused_and_omitted.rs:937` is about the ANCHOR gate, is present at `main` (`git show main:... | grep -c` returns 1) and comes from `734746f`, an earlier step: pre-existing, out of scope, not raised.

REASONING. The comment's reasoning survives the change and its test is unaffected; what is wrong is the API the sentence names, in the one file a maintainer opens to understand why `Q-55-existsgate` was scoped the way it was. That is the same class as `V-4` and `T-4`, both VALID at `low` in this increment. It is not a line-length, wrapping or raggedness observation, which the brief forbids; it is a factual claim about which function the code calls, and the claim is false.

SMALLEST REMEDY, and it is smaller than the reviewer's. `R3A-2` proposes a substitution reusing wording from `src/main.rs`. A PURE DELETION of four words is available and is what this project prefers: delete "`Path::exists`, which is", leaving

> The gate the policy above hangs on is `fs::metadata(..).is_ok()` and so answers false both for a log that is not there and for one behind a directory the process cannot traverse.

That is true as the code stands (`matches!(try_exists(), Ok(true))` is `metadata().is_ok()`: `try_exists` maps only `NotFound` to `Ok(false)` and everything else to `Err`, so the predicate is identical), it authors zero words, and it removes the stale name without naming a replacement that could go stale in turn. Confirmed against behaviour: on ENOTDIR/ELOOP/ENAMETOOLONG the gate answers false and the arm fires, exactly as `metadata().is_ok()` would.

## `R3A-3` INVALID for this increment, OUT OF SCOPE, observation preserved and ROUTED

REPRODUCED, including its attribution claim, which is the part that decides it. The `T-1` fixture shape transposed onto the plan source (`chmod 600 docs/plans` over a real `p.plan.toml`):

```
$ ls -A docs/plans -> p.plan.toml ; stat docs/plans/p.plan.toml -> Permission denied
$ <BIN> validate --source docs/plans/p.plan.toml --workflow
  no source plan at docs/plans/p.plan.toml; nothing to validate
  --workflow requested but no plan source resolved: pass a TOML-primary --source or a Markdown --plan
                                                                                       exit=1
NEW == BASE : IDENTICAL   (stdout, stderr and exit code compared as one string)
NEW == PRE  : IDENTICAL
```

Both sentences are wrong about the world and the remedy instructs the operator to do what they already did, exactly as reported. And it is entirely pre-existing: byte-identical on all three builds, and `src/main.rs:880` is context in `git diff main...HEAD`, not a changed line.

RULING: it does not belong to this increment. Three independent grounds.

1. THE INCREMENT'S LOCKED SCOPE NAMES THE OTHER ARM. The ledger's in-flight record (`docs/plans/agent-scaffold.ledger.md:531`) fixes the increment as "the `_` catch-all of `src/main.rs:run_validate`'s `--workflow` match becomes a reported problem so `--workflow` exits non-zero while plain `validate` is untouched, plus the corrected `run_validate` doc comment, the `SE-3` qualifier ..., the README `validate` paragraph, the CHANGELOG entry, and its own red-then-green test". The plan-source gate at `:880` is in none of that.
2. THE REMEDY IS A DECLINED HUMAN DECISION, NOT A TRIAGER'S INSTRUCTION. `Q-55-existsgate` chose an ARM-SCOPED fix precisely because the metrics gate's `else` branch also feeds plain `validate`, and DECLINED changing the gate itself. `source_path.exists()` at `:880` has the same property: its `else` branch prints the `no source plan at ...` note that plain `validate --source` emits at exit 0 (measured identical NEW vs PRE in the 15-input sweep below). A fix there re-opens the decision the human already made.
3. THE SPECIFICATION'S "Scope: what this step does not do" LIST does not carve this in, and its neighbouring entries route work of exactly this kind out: project identity in the join is "QUEUED to the validation-constraints step (`Q-55-mechanism`)", and the ledger's remaining-order block names "(2) the VALIDATION-CONSTRAINTS step" as the holder of that queue.

WHERE IT BELONGS: the validation-constraints step, beside round 2's `R2A-4` and the queued pre-existing plain-`validate` inconsistency. It is NOT the same defect as that queued item, which is about the LOG input's exit codes in plain `validate` (mode-000 log FILE at exit 1 versus unsearchable DIRECTORY at exit 0), and the reviewer is right to say so. It is the SAME FAMILY: one gate collapsing "not there" into "cannot look", on a gate whose `else` branch feeds plain `validate`, which is the subject that step now owns in three halves. Routing it keeps all three together, which is the reviewer's own recommendation and it is the right one.

The observation is preserved here so it is not lost, which is the reviewer's stated concern and the only thing at risk in an out-of-scope ruling.

## `R3B-1` VALID, `low`: the comment's "only" is an exhaustiveness claim the message stopped satisfying

REPRODUCED. The comment at `src/main.rs:1059-1063` says "`Ok` asserts absence and prescribes recording rounds, `Err` says only that the question could not be answered and names the error". The `Err` arm's live format string at `src/main.rs:1072-1075` says three things, and the third is measured in Part 1 above on four errno classes (EACCES, ENOTDIR, ELOOP, ENAMETOOLONG): it also says "; pass a `--metrics` naming this project's log". The comment predates `ce820fb`; `ce820fb` touched the format string and not the comment, which `git show` confirms.

REASONING. "says only X and Y" is an exhaustiveness claim and the code no longer satisfies it. The reviewer is right that the contrastive point the sentence exists to make (that `Err` deliberately does NOT prescribe recording rounds) survives untouched, and that is the reason this is `low` and not higher: no user surface reads the comment. But a reader who trusts it would believe the `Err` arm offers no actionable next step, and would re-propose the remedy `V-3` already shipped, which is precisely the cost `V-4` was rated for in this same increment.

I checked that this is not a re-raise: no round 1 or round 2 artifact discusses this comment's "only", and it could not have, since the words that falsified it landed in the branch's newest commit.

SMALLEST REMEDY: delete "only". One word, zero authored, and safe under both futures as set out in Part 1. Nothing else in the sentence changes; the `because` clause that follows still correctly explains why "record the project's review rounds there" specifically is omitted, which is the point the sentence is actually making.

---

# Part 3: closure-table spot-check

`R3B` asserts all ten prior findings closed. I checked ALL TEN myself rather than the three the brief requires, because the table is more consequential than any finding in this round. THE TABLE IS CORRECT: 10 of 10 closed, none of the closures broke another check.

| Finding | Table says | I measured | Command |
| --- | --- | --- | --- |
| `T-1` | CLOSED | CLOSED. mode-600 `docs/metrics` over a real one-record log: NEW prints `could not be checked (Permission denied (os error 13))` at exit 1, asserting no absence and prescribing no rounds; PRE prints the old skip note at exit 0; the `chmod 755` control then reads the same log clean, `workflow invariants hold`, exit 0. mode-000 gives the same NEW answer. | `chmod 600 docs/metrics; NEW validate --source docs/plans/p.plan.toml --workflow` |
| `T-2` | CLOSED | CLOSED. All three deployed copies carry the LOG-scoped wording "on a project with no round log yet, which every project scaffolded without `--instrument` remains", and the three `:93` lines hash identically, so the drift guard holds. `README.md:210` carries the same boundary. | `grep -n "no round log yet" pack/AGENTS.md AGENTS.md .agents/AGENTS.reference.md README.md` plus a per-file `md5sum` |
| `T-3` | CLOSED | CLOSED, with the discriminating control the table did not run. The current `PLAN_MD` fixture written verbatim validates: `plan.md: 1 steps, 0 open-questions items, valid`, exit 0. The old hyphenated spelling on the same fixture FAILS: `Roadmap step `only-step` has an unknown status `not-started``, exit 1. So the fixture is genuinely schema-valid and the control proves the checker would have caught it. | `NEW validate --plan plan.md` and the same file with `not started` -> `not-started` |
| `T-4` | CLOSED | CLOSED. `grep -rn "once built"` over all five doc files returns nothing, and the check runs today: this repository's own `validate --source docs/plans/agent-scaffold.plan.toml --workflow` reports `279 records, valid` / `95 steps, 69 questions, valid` / `workflow invariants hold`, exit 0. | as shown |
| `T-5` | CLOSED | CLOSED. `cd docs/plans && NEW validate --source p.plan.toml --workflow` prints `no round log at docs/metrics/workflow.jsonl: the workflow check could not run ...`, exit 1, matching `README.md:234`'s "fails, naming the log it looked for". | as shown |
| `T-6` | CLOSED | CLOSED. `workflow: bool` at `src/main.rs:440` carries no `requires` (the only `requires = "workflow"` is on `--workflow-spec` at `:442`, correctly); the CHANGELOG `Added` bullet now reads "It reuses the same metrics log as the rest of `validate`." with the three words gone; and `--workflow` with no `--plan` runs end to end. | `grep -n "workflow: bool" -B2 -A2 src/main.rs`; the CHANGELOG diff |
| `V-1` | CLOSED | CLOSED. `unshare -Ur env PATH=... TMPDIR=... HOME=... cargo test --test validate_workflow_toml_source_needs_no_plan` gives 4 passed / 0 failed as namespace root, identical to the ordinary user run. | as shown |
| `V-2` | CLOSED | CLOSED, and this is the one I built from scratch rather than trusting. FIFO at the `--source` path (so `run_validate` blocks at `fs::read_to_string`, AFTER the probe), CELL 2: gate answers `Ok(false)` truthfully, then `chmod 600 docs/metrics` mid-run, the log NEVER created. NEW prints `no round log at docs/metrics/workflow.jsonl ...` (the TRUE sentence) at exit 1; BASE prints the false `could not be checked (Permission denied (os error 13))` at exit 1. `ls -A docs/metrics` at the end is empty in both. The single-probe rebinding closed the false direction. | the FIFO race harness, run against NEW and BASE |
| `V-3` | CLOSED | CLOSED on all four errno classes plus the control. ENOTDIR (`Not a directory (os error 20)`), ELOOP (`Too many levels of symbolic links (os error 40)`), ENAMETOOLONG (`File name too long (os error 36)`) and EACCES all print `... ; pass a `--metrics` naming this project's log`; the dangling-symlink control (ENOENT -> `Ok(false)`) still gets the fuller `no round log at ... ; ... , or record the project's review rounds there`. The two-way split is intact. | four `--metrics` fixtures plus the dangling-symlink control |
| `V-4` | CLOSED | CLOSED. No commit hash anywhere in the test file; the two RED lines read "RED before the change" and "RED before this commit". | `grep -nE "RED (before|against)|[0-9a-f]{7,40}" tests/validate_workflow_toml_source_needs_no_plan.rs` |

SUPPORTING GATES, re-measured independently: `cargo test` 422 passed / 0 failed; `cargo clippy --all-targets -- -D warnings` exit 0; the repository's own `validate --workflow` exits 0.

`Q-55-existsgate`'S PROMISE RE-MEASURED, because it is the ground the round 2 remedy was accepted on: plain `validate` (no `--workflow`) is BYTE-IDENTICAL between NEW and PRE across 15 inputs, stdout, stderr and exit code compared as one string, **15 SAME, 0 DIFF**: the default present log; an explicit present `--metrics`; an absent named `--metrics`; ENOTDIR; ELOOP; ENAMETOOLONG; a dangling symlink; `--metrics` naming a directory; an empty `--metrics ""`; no anchors at all; a missing `--source`; `docs/metrics` at mode 600 and at mode 000; a mode-000 log FILE; and a mode-600 `docs/plans`.

---

# Part 4: what the reviewers missed

Three items. NONE is raised as a finding, and each says why, so a fix pass does not act on them.

## 1. `ce820fb`, the branch's newest commit, is not guarded by any test. MEASURED, and NOT raised, on this increment's own precedent

No test asserts either remedy clause. `grep -rn "pass a \`--metrics\`\|naming this project" tests/ src/` returns exactly one test hit, `tests/unsafe_pairings_are_refused_and_omitted.rs:199`, and that is the CONTAINMENT message's "pass a `--metrics` under that root", a different string. I measured the consequence rather than inferring it: I applied `R3A-1`'s nine-word deletion to `src/main.rs:1073` and ran the suite in the project toolchain. **422 passed, 0 failed.** The entire content of the branch's newest commit can be deleted and nothing fails. The edit was reverted; `git status --short` is empty.

NOT RAISED, and the ground is round 1's own ruling in this increment. `T-8` (`SC-3`) was INVALID for asking that untested PROSE be pinned, on three grounds, of which the second is directly on point: a content guard here would be a new class of guard rather than a missing instance of an existing one, since nothing in this repository pins a message's advisory tail. What IS pinned is the behaviour, and the behaviour is covered: the arm's exit code, its `could not be checked` sentence, and its refusal to say `record the project's review rounds` are all asserted at `tests/validate_workflow_toml_source_needs_no_plan.rs:313-330`. The untested thing is nine words of advice hanging off a tested error path, and its silent loss would cost exactly what `R3A-1` says leaving it costs: one wasted command. `SC-3`'s third ground also has a weaker analogue here: pinning a clause whose fate I have just put to the human as a residual is premature. If the human accepts the residual and the wording settles, a guard on the settled sentence becomes a coherent proposal and should be raised then.

## 2. The `run_validate` doc comment's enumeration is one case short of the clap help it was written beside. NOT raised, and DO NOT touch it

`src/main.rs:814-815` says "Both of the check's inputs answer that way: no resolvable plan source, and no round log at the resolved metrics path." The `--workflow` clap help at `src/main.rs:438` was updated for the `Ok`/`Err` split and now names both log answers: "So is no round log at the resolved path at all, or a path the check cannot answer that question for". The doc comment names only the first.

NOT A FINDING. Unlike `R3B-1`'s "only", this carries no exhaustiveness word: "Both of the check's inputs answer that way" is a claim about the ANSWER (a reported problem and a non-zero exit), and both inputs do answer that way, including on the cannot-check path. It under-describes rather than mis-states. I record it only to say explicitly that it should NOT be fixed: every available remedy here ADDS words to a comment, on a round whose two valid findings are both deletions, and this project has five retrospective and one prospective measurement that a fix pass which authors prose manufactures the next round's finding. A fix pass editing the comment 245 lines below for `R3B-1` should not touch this one.

## 3. Nothing scaffolded runs `validate --workflow`, so the tier policy breaks no scaffolded project's gate. A negative result, recorded because it is the change's blast radius

`R3A-1`'s and `R3B`'s sweeps both measured the change at the CLI surface, and neither asked whether anything the tool itself scaffolds would start failing. I checked: `grep -rn "validate --workflow" pack/ justfile .github` finds the string only in `pack/instrument.md`'s prose describing the check, never in a scaffolded pre-commit hook, a justfile recipe, or a CI workflow. The scaffolded `.agents/hooks/pre-commit` (`src/manifest.rs:661`) does not invoke it. So the population the CHANGELOG names as broken, "every project with no round log at the resolved path", is broken only where a human or CI wired the command up by hand, which is what the scaffolded `AGENTS.md` sentence now warns about. This SUPPORTS the change rather than finding against it, and it is the check I would have wanted run before shipping an exit-code change.

---

# Tally

| Severity | Count | Findings |
| --- | --- | --- |
| critical | 0 | |
| high | 0 | |
| medium | 0 | |
| low | 2 | `R3A-2`, `R3B-1` |

VALID FINDINGS, DEDUPLICATED: **2**, both `low`. Ceiling: `low`.

NOT COUNTED VALID: `R3A-1` ACCEPTED AS A RESIDUAL at `low` (real, reproduced, no remedy earns its cost); `R3A-3` INVALID for this increment as pre-existing and out of scope, ROUTED to the validation-constraints step.

`R3A-1` and `R3B-1` are NOT duplicates of each other. They concern the same nine words from opposite sides (the message that carries them, the comment that mis-describes them), and Part 1 resolves them as one decision with one remedy.

THE FIX PASS OWES TWO DELETIONS AND NOTHING ELSE, five words in total, zero words authored:

1. `src/main.rs:1060`: delete "only".
2. `tests/validate_workflow_toml_source_needs_no_plan.rs:267`: delete "`Path::exists`, which is".

Neither touches behaviour, so the 422/0 suite, clippy, and the byte-identity of plain `validate` against PRE are all unaffected by both.

# Relitigation and constraints check

Nothing here raises or upholds anything on the settled list. The four standing residuals (the in-root bound, the single-anchor `..` case, the earlier increment's rejected-ledger context slot, the off-convention `--source` surface) are untouched. Accepted costs (i) to (iv) are pinned as expected behaviour and not questioned; `R3A-1`'s transcripts and mine both show `R2A-4`'s stale `no metrics log at <path>; nothing to validate` note one line above the corrected sentence, ACCEPTED AS A RESIDUAL, and it is not raised. Round 1's `ADV-4` (an empty log at the resolved path still yields `workflow invariants hold`) appears in no finding here. Round 1's `SC-3` and round 2's `R2B-2` and `R2B-3` stay INVALID; `SC-3`'s reasoning is CITED in Part 4 to decline a new proposal, not reopened. The pre-existing plain-`validate` inconsistency (mode-000 log FILE at exit 1, unsearchable DIRECTORY at exit 0) stays QUEUED to validation-constraints and is not raised; `R3A-3` is routed to sit beside it without being conflated with it. `Q-55-existsgate`'s DECLINED `try_exists()?` gate change is not asked for anywhere, and is the second ground on which `R3A-3` is ruled out of scope.

No line-length, prose-wrapping or comment-raggedness observation appears anywhere in this file.

FIXTURE HYGIENE: all fixtures under `<scratch>/tri-r3`, a directory of my own naming; nothing outside it was written or deleted; the closing `find <scratch>/tri-r3 -type d ! -perm -u+rwx` and `find <scratch>/tri-r3 -type f -perm 000` both return nothing. `TMPDIR` pointed outside any git repository for every `cargo test`. No `nix fmt` and no `just scaffold-self` was run. The one temporary source edit was reverted and `git status --short` is empty; the main repository at `/home/jessea/Documents/projects/agent-scaffold` was not touched.
