# `workflow-enforcement-tier-inc3` work review, round 1, TRIAGE

Triaged on branch `triage/inc3-r1` at `74e6426`, the tip of the branch under review with the three reviewers' findings merged in. The product change is the two commits `9a7555f` (the fix) and `74e6426` (the `SE-3` qualifier); their parent is `d4042bb`. Governing specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, the `workflow-enforcement-tier-inc3` bullet, acceptance checks 15 to 20, and the `INC3:` documentation-impact block.

METHOD. Nothing below is adjudicated from reading. Two binaries were built and every verdict rests on running them:

- NEW: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-inc3-r1/target/debug/agent-scaffold` (`74e6426`).
- OLD: a detached worktree at `d4042bb` under this triager's own scratch directory, built independently: `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/triage-r1/old-d4042bb/target/debug/agent-scaffold`.

Every fixture lives under `.../scratchpad/triage-r1/fix/`. `TMPDIR` was pointed at `.../scratchpad/triage-r1/tmpdir`, outside any repository, for `cargo test`.

GATES, run on the tree as triaged: `cargo test` 421 passing across nine binaries, 0 failing; `cargo clippy --all-targets -- -D warnings` clean; `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` reports `up to date`.

ONE TEMPORARY SOURCE EDIT was made, to measure a candidate remedy for `T-1` (recorded under that finding). It was reverted with the `Edit` tool and `git status --short` and `git diff --stat` are both empty on the tree that carries this file.

---

## Deduplication

Eleven raw findings from three lenses collapse to EIGHT unique findings. Three defects were reached by two lenses each, by different routes; that is corroboration and it raised confidence in all three, so each reviewer keeps credit for its own id while the round-level count is the deduplicated one.

| Unique | Reviewer ids | Same defect because | Verdict | Severity |
| --- | --- | --- | --- | --- |
| `T-1` | `ADV-1` | (single lens) | VALID | medium |
| `T-2` | `ADV-2`, `DOC-1` | Both: the shipped tier boundary is written at the `--instrument` FLAG while the tool tests for the LOG. `ADV-2` reached it by scaffolding two fixtures and diffing their behaviour; `DOC-1` reached it by reading the three shipped sentences against `build_assets`. Same sentence, same falsehood, same remedy. `DOC-1` additionally covers the `CHANGELOG` site. | VALID | medium |
| `T-3` | `ADV-3`, `SC-2` | Both: `PLAN_MD`'s Roadmap status is `not-started` where the Markdown vocabulary is `not started`, so sub-case (b)'s exit-code assertion is satisfied pre-fix by a plan-schema failure, and the fixture's "schema-valid" doc comment is false. Identical one-character remedy. | VALID | low |
| `T-4` | `DOC-2` | (single lens) | VALID | medium |
| `T-5` | `DOC-3`, `SC-1` | Both: `README.md:234` still says the bare-filename-from-`docs/plans` case "reports that it found no log", on an example command carrying `--workflow` that now exits 1. Identical site, identical remedy. | VALID | low |
| `T-6` | `DOC-4` | (single lens) | VALID | low |
| `T-7` | `ADV-4` | (single lens) | ACCEPT-AS-RESIDUAL | low |
| `T-8` | `SC-3` | (single lens) | INVALID | n/a |

---

## The already-ruled scope question, checked rather than assumed

The implementer added a sentence to `src/main.rs:ValidateArgs::workflow`'s help: "So is no round log at the resolved path at all: the check cannot run, and a check that did not run must not report success." The sidecar assigns that help string to INC2 and does not name it under INC3. The orchestrator ruled KEEP; the scope reviewer independently agreed. A ruling by the orchestrator is not evidence, so I checked it.

I AGREE WITH KEEP, on evidence:

- The clause is TRUE. Reproduced on every problem-producing path: the `_` arm pushes a problem naming the resolved path and exits 1 (`T-1` through `T-5` evidence below all exercise it).
- The enumeration would otherwise be short by one. The help string names three refusal causes; before this diff it named two, and this diff creates the third. I checked each problem-producing path in the `--workflow` block against the help string's sentences: the `(None, None, _)` arm ("no plan source resolved"), the containment guard ("a round log that lies outside the project root"), and the new `_` arm ("no round log at the resolved path"). Each is named exactly once, none is named twice, and the fourth non-zero path from `--workflow` (a malformed `--workflow-spec`, which `std::process::exit(1)`s before the match) is named in `--workflow-spec`'s own help. Verified that precedence too: a malformed spec plus a missing log reports the spec error and never reaches the round-log problem.
- The governing sentence of the documentation-impact section is "All in-repo, and each item travels with the increment that makes it stale rather than being left as a documentation step owed." INC2 made this help string stale (it gained the refusal) and INC3 makes it stale AGAIN (it gains a third cause). Both are true at once. Listing it under INC2 records where it travelled the first time; it does not make INC3's list exclusive.

One honest qualification, which is `T-1` and not an objection to KEEP: the clause asserts "no round log at the resolved path at all" on inputs where the tool cannot actually tell. That is a defect in the gate the clause describes, not in the clause's scope.

---

## `T-1` (`ADV-1`) VALID, medium: a round log that exists is reported absent, with a remedy telling the operator to record rounds that are already recorded

### The question this round turned on, answered

The task set one question: `ADV-1` says the new problem is gated on `Path::exists()`, which collapses "absent" with "could not be answered", while the SAME reviewer reports under "what produced nothing" that a mode-000 log file and a directory at the log path both propagate their io error instead of being absorbed. Those two statements must be reconciled, and the reachable set stated precisely rather than gestured at.

THEY RECONCILE EXACTLY, and the mechanism is the split between the two calls. `Path::exists()` is `fs::metadata(path).is_ok()`. The gate at `src/main.rs:845` is `if metrics_path.exists()`, and the read inside it is `fs::read_to_string(&metrics_path)?`.

- When `fs::metadata` SUCCEEDS, control enters the branch and any failure belongs to `read_to_string`, whose io error propagates through `?`. This is the mode-000 log FILE (stat succeeds, open gives EACCES) and the directory-at-the-log-path (stat succeeds, read gives EISDIR). Neither reaches the `_` arm at all.
- When `fs::metadata` FAILS, `exists()` answers `false` for a reason that is not "the entry is not there", `metrics_contents` is `None`, and the `_` arm fires with "no round log at <path>".

The two sets are disjoint by construction, so both of the reviewer's statements are true simultaneously. All four arrangements reproduced, NEW and OLD:

```
MODE-000 LOG FILE            NEW: Error: Os { code: 13, kind: PermissionDenied }   exit=1
                             OLD: identical                                        exit=1
DIRECTORY AT THE LOG PATH    NEW: Error: Os { code: 21, kind: IsADirectory }        exit=1
                             OLD: identical                                        exit=1
UNREADABLE ANCESTOR          NEW: "--workflow requested but no round log at ..."    exit=1
                             OLD: "--workflow has a plan source but the metrics log is missing; skipping"  exit=0
SYMLINK LOOP / ENOTDIR / DANGLING SYMLINK    NEW: same "no round log" problem       exit=1
                             OLD: same skip note                                    exit=0
```

### The reachable set, stated precisely

Within the `fs::metadata`-fails set, I constructed every arrangement I could and asked of each whether a REAL log sits behind the false answer:

| Arrangement | `metadata` error | Is a real log there? | Is "no round log at <path>" a falsehood? |
| --- | --- | --- | --- |
| An ancestor directory of the resolved log denies SEARCH (`x`) permission | EACCES | YES | YES |
| Symlink loop at the log path (`workflow.jsonl -> workflow.jsonl`) | ELOOP | No, nothing resolves | No |
| Symlink loop in an ancestor (`docs/metrics -> metrics`) | ELOOP | No, nothing resolves | No |
| A non-directory component (`docs/metrics` is a regular file) | ENOTDIR | No | No |
| Dangling symlink at the log path | ENOENT | No | No |
| An over-long name | ENAMETOOLONG | No | No |

THE REACHABLE SET IS EXACTLY ONE CLASS AND IT IS NOT EMPTY: **an ancestor directory of the resolved metrics path denies search permission to the running process, so `fs::metadata` returns EACCES while a real, non-empty, correctly-placed round log sits behind it.** Every other member of the `metadata`-fails set has no log behind it, so for those the message is terse rather than false, and I do not count them. In particular this is NOT the class "unreadable inputs": the log's OWN permissions are irrelevant, because a mode-000 log file stats fine and propagates. It is specifically the TRAVERSAL of the path, one level above the log or higher.

One bound on the class, which I established rather than assumed: the unsearchable directory must be at or below the level that does not also block reading the plan source, or the run fails earlier for a different and correct reason. With `docs` itself at mode 000, the run reports `--workflow requested but no plan source resolved` (verified). In the default anchored layout the reachable ancestor is therefore `docs/metrics` itself; with an explicit `--metrics` it is any component below the plan's root (verified with `--metrics docs/metrics/sub/log.jsonl` under a mode-000 `sub`).

### Evidence

Fixture: a TOML-primary plan at `docs/plans/p.plan.toml` with one `not-started` step, and a real one-record log at `docs/metrics/workflow.jsonl`.

```
$ chmod 000 docs/metrics
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there
exit=1                                                                    <-- NEW (74e6426)

$ (same command, OLD, d4042bb)
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
exit=0                                                                    <-- OLD
```

CONTROL, the same command after `chmod 755 docs/metrics`, NEW: `docs/metrics/workflow.jsonl: 1 records, valid` / `docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold`, exit 0. The log had one record and was there the whole time.

A SHARPER DEMONSTRATION THE REVIEWER DID NOT FIND, mode 600 rather than 000, so the directory is READABLE but not SEARCHABLE:

```
$ chmod 600 docs/metrics
$ ls docs/metrics
workflow.jsonl                                          <-- same user, same shell, the log is right there
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow
--workflow requested but no round log at docs/metrics/workflow.jsonl: ...
exit=1
```

The operator can list the file by name at the exact path the tool says has no round log. This is a better fixture than `chmod 000` and I recommend it if a regression test is ever written.

`chmod 111` (searchable, not readable) is the negative control: `exists()` answers true, the read succeeds, and the run exits 0 with `workflow invariants hold`. So the discriminator is search permission on the ancestor and nothing else.

### Reasoning

The collapse in `Path::exists` is pre-existing; what inc3 authored is the loud, prescriptive line built on top of it, and this tree already decided this exact distinction ONE INCREMENT EARLIER IN THIS SAME STEP. `src/main.rs:1127-1133` writes the decision down:

> THREE CASES, NOT TWO. `try_exists` separates "not there" from "there, but a directory above it cannot be traversed", which `Path::exists` collapses into one `false`. This is the whole Fail-loudly half of `Q-55-emptyroot`'s remedy, and a loud line that states a falsehood about the filesystem is worse than a quiet one: it sends the operator to fix a path that is already correct.

That is not just a comment: the behaviour is live in the shipped binary. Reproduced on the same class of arrangement, on the ANCHOR rather than the log:

```
$ chmod 000 locked && agent-scaffold status --source locked/p.plan.toml
note: --source locked/p.plan.toml could not be checked: Permission denied (os error 13)
exit=0
```

So the tool says "could not be checked" for a `--source` it cannot stat, and "no round log at <path>" for a log it cannot stat, on the same filesystem arrangement, in the same run. The new message's second remedy clause, "record the project's review rounds there", is precisely the "sends the operator to fix a path that is already correct" the decision was written to prevent, and it misclassifies a fully instrumented project into the guidance tier, which is the exact boundary this increment exists to report truthfully.

NOT HIGHER THAN MEDIUM, and I agree with the reviewer's rating. The exit status, which is the contract inc3 changes, is RIGHT here: the check genuinely could not run, so non-zero is correct and there is no false green anywhere in this class. Only the diagnosis is false. The arrangement also requires an unusual permission state.

NOT LOWER THAN MEDIUM, which is why I did not re-rate it down despite the narrow reachable set: the increment converted a quiet falsehood at exit 0 into a loud, prescriptive falsehood at exit 1, and the cited decision says in as many words that this direction is the worse one.

### Smallest remedy

RECOMMENDED, and it is the smallest that stays inside the tier boundary the increment's own comment draws ("This is the tier boundary and nothing wider"): decide the NEW PROBLEM on `metrics_path.try_exists()` inside the `_` arm only, and on `Err` say the check could not be performed and name the error, in the vocabulary `note_missing_anchors` already established, instead of asserting the log is absent and prescribing that rounds be recorded. The exit code stays 1 either way, so this is a message split and not a behaviour change, and it touches no surface other than `--workflow`.

I ALSO MEASURED THE DELETION-CLASS ALTERNATIVE, because this project prefers a deletion to an addition where both work. Changing the gate itself from `metrics_path.exists()` to `metrics_path.try_exists()?` is a ONE-TOKEN edit that authors no prose at all: it removes the absorption of the io error rather than adding a branch to explain it, and it makes the unreadable-ancestor case answer identically to the mode-000 log file and the directory-at-the-log-path, which already propagate. Measured with that edit applied and then reverted:

- Unreadable ancestor, `--workflow`: `Error: Os { code: 13, kind: PermissionDenied }`, exit 1. The falsehood is gone.
- Genuinely absent log (acceptance check 15): byte-identical to HEAD, exit 1.
- Absent log without `--workflow` (acceptance check 16): byte-identical to HEAD, exit 0.
- The correct case: byte-identical to HEAD, exit 0.
- `cargo test`: 421 passing, 0 failing.

ITS ONE REAL COST, which is why I do not recommend it outright and why this is a decision rather than a triager's instruction: it also changes PLAIN `validate` (no `--workflow`) on the unreadable-ancestor case, from a false note at exit 0 to a hard io error at exit 1, which is wider than the tier boundary. Note that plain `validate` ALREADY hard-errors on an unreadable log FILE at HEAD (`Error: Os { code: 13 }`, exit 1, verified), while on an unreadable ANCESTOR of the same log it prints `no metrics log at ...; nothing to validate` at exit 0 (verified). So the one-token edit removes a pre-existing inconsistency rather than introducing a new class, but it does so on a surface the increment deliberately did not touch. The message split is the conservative choice; the one-token edit is the smaller and more consistent one. Both were measured; the choice is a design call.

---

## `T-2` (`ADV-2` + `DOC-1`) VALID, medium: the shipped tier boundary is written at the `--instrument` flag, and the tool tests for the log

### Evidence

Two fixtures scaffolded with the NEW binary, one without and one with `--instrument`:

```
$ ls noinst/docs        ->  plans
$ ls inst/docs          ->  plans                    <-- --instrument created NO docs/metrics
$ diff -rq noinst inst
Files noinst/.agents/AGENTS.reference.md and inst/.agents/AGENTS.reference.md differ
Files noinst/AGENTS.md and inst/AGENTS.md differ     <-- and nothing else differs at all
```

Acceptance check 15 run in each, NEW:

```
$ (cd noinst && agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow)
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, ...
exit=1

$ (cd inst && agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow)
(byte-identical output)
exit=1
```

OLD, both fixtures: exit 0 with the skip note. So the instrumented project's failure is INTRODUCED BY THIS INCREMENT, and the sentence introduced alongside it in the same commit attributes that failure to a property the project does not have. Both fixtures' `AGENTS.md` carry the backstop sentence (`grep -c` returns 1 in each); only the instrumented one also carries `## Instrumentation`, so the qualifier does not adapt to the rendered tier either.

The three shipped sentences, all confirmed present at `74e6426`:

- `pack/AGENTS.md:93` and its two deployed renders: "a project scaffolded without `--instrument` has no round log for it to read, so on such a project that check exits non-zero reporting that it could not run rather than passing".
- `README.md`: "a project scaffolded without `--instrument` keeps no round log, so `--workflow` fails there".
- `CHANGELOG.md`: "THE POPULATION THIS BREAKS is every project scaffolded without `--instrument`, which keeps no round log at all".

### Reasoning

Acceptance check 20 states the property the sentence must have: "a reader of that sentence alone must be able to predict check 15's exit code", and the CHANGELOG asserts the same in its own words. Measured, the sentence supports that prediction over exactly half the population. A reader on a project scaffolded WITH `--instrument` predicts exit 0 from the sentence and gets exit 1, and the sentence hands them a false diagnosis ("you did not scaffold with `--instrument`") for a project that was.

RE-RATED UP FROM `ADV-2`'s `low` TO `medium`, agreeing with `DOC-1`. `ADV-2` bounded the harm to the window between scaffolding and the first appended record, which is short and self-correcting. That bound is too generous to the sentence, for a reason `ADV-2` itself records: the identical exit code and identical message are also produced on a fully instrumented, fully logged project whose run is mis-anchored (accepted cost (i), pinned by check 18), which is not a window but a permanent case, and the shipped sentence offers the same false explanation there. The failing property is check 20's, which is an acceptance criterion of this increment and not a general prose preference, so it belongs above `low`.

NOT HIGHER THAN `medium`: nothing about the tool's behaviour is wrong, no exit code is wrong, and the reader is misdirected rather than led into a destructive action.

### Smallest remedy

Re-scope the clause from the FLAG to the LOG. This is a substitution of equal or shorter length in each site, not an addition, and it drops the false discriminator rather than qualifying it:

- `pack/AGENTS.md:93`: "and a project scaffolded without `--instrument` has no round log for it to read, so on such a project that check exits non-zero" becomes "and on a project with no round log yet, which every project scaffolded without `--instrument` remains, that check exits non-zero". Then regenerate the two deployed copies with `cargo run -- scaffold --output-dir . --write --force --principles default --instrument` (NOT `just scaffold-self`, which runs `nix fmt`).
- `README.md`: the same substitution.
- `CHANGELOG.md`: name the population as "every project with no round log at the resolved path" rather than by the flag.

ONE EDIT, NOT TWO: `T-4` deletes two words from the SAME sentence in `pack/AGENTS.md:93`. Both should land in one edit and one re-render, or the second pass re-renders what the first just rendered.

If only one site can move, it is `pack/AGENTS.md`: that is the one acceptance check 20 binds and the one an agent reads at runtime.

---

## `T-3` (`ADV-3` + `SC-2`) VALID, low: the new test's Markdown fixture is documented as schema-valid and is not, so case (b)'s exit-code assertion passes against the pre-fix build

### Evidence

`PLAN_MD` (`tests/validate_workflow_toml_source_needs_no_plan.rs:39-59`) is introduced as "A minimal, schema-valid Markdown `--plan` ... Only its PRESENCE matters below". Its Roadmap row carries status `not-started`; `src/plan.rs:92-93` defines `ROADMAP_STATUSES` as `["not started", "in progress", "complete", ...]`, space-separated.

Reproduced with the test's own two constants verbatim, and with a one-character variant:

```
AS-IS (not-started):
  OLD  validate --workflow --plan plan.md   -> plan.md: Roadmap step `only-step` has an unknown status `not-started`   exit=1
  NEW  validate --workflow --plan plan.md   -> the same schema problem PLUS the new problem                            exit=1
  OLD  validate --plan plan.md (no --workflow) -> the same schema problem alone                                        exit=1

FIXED (not started):
  OLD  validate --workflow --plan plan.md   -> plan.md: 1 steps, 0 open-questions items, valid + the skip note         exit=0
  NEW  validate --workflow --plan plan.md   -> --workflow requested but no round log at docs/metrics/workflow.jsonl    exit=1
```

So `assert_eq!(code, Some(1))` in sub-case (b) is satisfied pre-fix by a plan-schema failure that has nothing to do with the increment, and one character converts it into a true red-then-green.

THE GUARD IS NOT LOST, which I verified rather than inferred, and this is what caps the severity. I copied the branch's test file verbatim onto the `d4042bb` tree and ran it:

```
$ cargo test --test validate_workflow_toml_source_needs_no_plan       (on d4042bb)
test workflow_with_no_metrics_log_hard_errors_instead_of_skipping ... FAILED
  panicked at tests/validate_workflow_toml_source_needs_no_plan.rs:197:5
  --workflow with no round log must not exit 0
  left: Some(0)   right: Some(1)
test result: FAILED. 2 passed; 1 failed
```

Line 197 is sub-case (a), the TOML arm, which is a fully attributable red on the exit code alone. The test therefore goes red on a revert for the right reason, and sub-case (b)'s SECOND assertion (`stderr.contains("no round log at ...") && stderr.contains("could not run")`) discriminates the fix on the Markdown arm regardless.

### Reasoning

RE-RATED DOWN FROM `SC-2`'s `medium` TO `low`, agreeing with `ADV-3`. `SC-2` is right about the defect and right about the remedy, but "the increment's own new test does not discriminate" would be the medium-grade claim, and the measurement above shows that is not what happened: the test discriminates through sub-case (a) and through sub-case (b)'s stderr assertion. What is actually wrong is a FALSE CLAIM IN A FIXTURE'S DOC COMMENT ("schema-valid", and "Only its PRESENCE matters below", both untrue) plus one over-determined assertion beside a sound one. That is fixture hygiene, not a lost guard.

I do not discount it to nothing, because the false doc comment is exactly the kind of statement a later reader trusts when deciding what a test proves, and because the remedy costs one character.

### Smallest remedy

One character: `not-started` becomes `not started` in `PLAN_MD`'s Roadmap row (`:58`) and in the doc comment's own example (`:44`). Do NOT touch `PLAN_TOML`'s hyphenated `not-started`, which is correct for the TOML schema. This makes the fixture actually match its comment and makes sub-case (b) discriminate on its own.

---

## `T-4` (`DOC-2`) VALID, medium: `once built` is retained in a sentence that now reports what the check does today

### Evidence

The check is built and runs, with no `--plan` at all:

```
$ agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 276 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

`workflow invariants hold` is the backstop reporting a pass. Site count: `grep -rn "once built" pack/ README.md AGENTS.md .agents/ CHANGELOG.md` returns exactly three lines, `pack/AGENTS.md:93` plus its two deployed renders. The clause is PRE-EXISTING (present once at `d4042bb`), and this diff rewrote the line it sits in.

### Reasoning

One sentence now tells the reader both that the deterministic backstop is future work ("once built") and what its exit code is today. Only one of those can be acted on, and the false one is the one that tells an orchestrator not to bother invoking the gate. The audience is not a human skimming a README: it is an agent reading its own scaffolded `AGENTS.md` to decide whether the deterministic backstop is available to it. The step exists to make that answer yes.

IN SCOPE, and squarely so. The `INC3:` documentation-impact block names `pack/AGENTS.md:93` explicitly and quotes THE PASSAGE as "the deterministic `validate --workflow` check, once built, is the backstop that the required reviewed rounds happened before a step is marked complete". The implementer edited that exact sentence under that exact instruction and left the false clause inside it. The sidecar quotes `once built` to IDENTIFY the passage, not to endorse it; its instruction is about what the qualifier must add.

`medium` is right and I keep the reviewer's rating. The consequence is agent-facing and it is about whether the increment's own mechanism gets used at all. It is also the cheapest fix in the round.

### Smallest remedy

Delete two words. "the deterministic `validate --workflow` check, once built, is the backstop" becomes "the deterministic `validate --workflow` check is the backstop". A pure deletion, which is the class of fix the sidecar's own note prefers, and it lands in the same edit as `T-2`'s substitution on the same sentence, followed by one regeneration of the two deployed copies.

---

## `T-5` (`DOC-3` + `SC-1`) VALID, low: the README still describes accepted cost (i) as a note, on an example command that now exits 1

### Evidence

`README.md:234`, untouched by this diff (`git diff d4042bb..74e6426 -- README.md` contains no occurrence of "bare filename"):

> One consequence to know about: a bare filename run from inside `docs/plans` (`cd docs/plans && agent-scaffold validate --source my-task.plan.toml --workflow`) has no parent directories to derive a root from, so it looks for `docs/metrics/workflow.jsonl` beneath `docs/plans` and reports that it found no log; run it from the project root instead.

The literal example command:

```
$ cd docs/plans && agent-scaffold validate --source TEMPLATE.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, ...
exit=1                                                                       <-- NEW

$ (same, OLD)
--workflow has a plan source but the metrics log is missing; skipping the workflow check
TEMPLATE.plan.toml: 1 steps, 0 questions, valid
exit=0                                                                       <-- OLD
```

### Reasoning

The BEHAVIOUR is correct and pinned, and neither reviewer asks for it to change; accepted cost (i) is expected behaviour and is not re-raised here. What is stale is the sentence: the example carries `--workflow`, so "reports that it found no log" describes the inc1 answer in a paragraph whose other failure cases are labelled "exit 1" or "exits non-zero", which makes this one read as the benign one. Acceptance check 18 states the corrected answer in the spec's own words ("After inc1 alone: the stderr miss note and exit 0. After inc3: a HARD FAILURE naming the path it looked for"), the test was renamed for exactly this reason, and the CHANGELOG entry added by this same diff documents the new answer. Only the README sentence about the same case was left behind, so two committed documents now disagree about what a literal README example does. Under the governing sentence, inc3 made this site stale and it travels with inc3.

RE-RATED DOWN FROM `SC-1`'s `medium` TO `low`, agreeing with `DOC-3`. The discriminator is consequence, and the ACTIONABLE half of the sentence, "run it from the project root instead", is still exactly right and unchanged. No reader is led into a wrong action; the severity of the symptom is understated and nothing more. That caps it below the sentences in `T-2` and `T-4`, which do misdirect the reader about what is wrong and what to do.

### Smallest remedy

One clause, no new sentence and no length increase: "reports that it found no log" becomes "fails, naming the log it looked for".

---

## `T-6` (`DOC-4`) VALID, low: the CHANGELOG's `Added` entry for `--workflow` says it requires `--plan`, in the same unreleased section as the new `Changed` entry that says it does not

### Evidence

`CHANGELOG.md:15`, `## [Unreleased]` / `### Added`: "It requires `--plan` and reuses the same metrics log as the rest of `validate`."

`ValidateArgs::plan` is `Option<PathBuf>` under a bare `#[arg(long)]` with no `required`, and `ValidateArgs::workflow` is a bare `#[arg(long)] workflow: bool` with no `requires = "plan"`. Reproduced (this is the same run as `T-4`'s):

```
$ agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

No `--plan`, no usage error, the check ran. The suite pins the same property in `tests/validate_workflow_toml_source_needs_no_plan.rs::workflow_on_a_toml_source_runs_without_a_markdown_plan`, which passes.

### Reasoning

VALID as a claim: the sentence is false, it has not shipped, and it sits in the same unreleased section as the `Changed` bullet this diff adds and the `--workflow` help clause this diff adds, both of which contradict it directly. A release note that contradicts itself about a flag's required arguments is a false claim a user acts on.

PROVENANCE, stated plainly because it bears on whether the fix belongs to THIS round: inc3 did not make this stale. An earlier increment's clap relaxation did, and under the documentation-impact section's governing sentence it should have travelled with THAT increment. So the human may legitimately route it to the documentation-currency step instead of this fix round. My reason for not ruling it out of scope is that the remedy is a three-word DELETION in a file this diff already edits, in the same unreleased block, and the bookkeeping to defer it costs more than the deletion.

`low` is right and I keep the reviewer's rating, which the reviewer itself flagged honestly.

### Smallest remedy

Delete three words: "It requires `--plan` and reuses" becomes "It reuses".

---

## `T-7` (`ADV-4`) ACCEPT-AS-RESIDUAL, low: an empty file at the resolved path converts the new refusal into an affirmative `workflow invariants hold`

### Evidence

Reproduced exactly as reported, and the parent-commit comparison is the decisive part:

```
$ mkdir -p docs/metrics && : > docs/metrics/workflow.jsonl
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow
docs/metrics/workflow.jsonl: 0 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0                                       <-- NEW and OLD BYTE-IDENTICAL

$ rm docs/metrics/workflow.jsonl && (same command)
NEW: --workflow requested but no round log at ...   exit=1
OLD: --workflow has a plan source but the metrics log is missing; skipping   exit=0
```

THE CONTROL, which is what settles it. The same fixture with the step at `status = "complete"` and the same empty log:

```
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `only-step` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped
exit=1                                       <-- NEW and OLD BYTE-IDENTICAL
```

### Reasoning

The reproduction is real and I do not dismiss it. The verdict turns on three measured facts.

1. THE BEHAVIOUR IS PRE-EXISTING AND UNCHANGED. NEW and OLD are byte-identical on the empty log in both directions. This increment did not create it; it made it newly REACHABLE as a workaround, by making the adjacent case fail.
2. THE GREEN IS TRUE, NOT FALSE. The increment's stated property is "a check that did not run must not report success". With an empty log the check DID run: W3 iterated the Roadmap, found no `complete` step, and found no violation. The control shows enforcement biting correctly the instant there is anything to enforce, identically on both builds. So no invariant is bypassed and no review evidence is misreported.
3. THE STEP'S ORDERING ARGUMENT IS NOT FALSIFIED. I read the sentence `ADV-4` cites (`workflow-enforcement-tier.md:295`) in full. Its subject is escape hatches into the CONTAMINATED GREEN of defect B: "a user whose `--workflow` run started failing could 'fix' it by running from a directory that happens to contain a log", and the two it names (standing somewhere else, `--metrics` at a foreign log) are both routes to reading ANOTHER PROJECT'S evidence. `touch` is not that. It produces a truthful vacuous answer over the project's OWN empty log, which is the opposite failure mode from the one the ordering argument is defending against.

WHAT THE REMEDY WOULD ADD AND WHAT IT WOULD REMOVE, which is the question this round is meant to ask. Neither available remedy removes anything. The reviewer's first option adds a line to the step's "what this does not fix" list; its second adds a parenthetical to `workflow invariants hold`, which is a line the CORRECT case prints, and is squarely the fix-pass-authors-prose pattern this project has five retrospective and one prospective measurement against. The reviewer prefers the first and so do I, and the first IS the residual-acceptance action rather than a product change.

ACCEPT-AS-RESIDUAL is therefore the honest verdict: reproduced, real, pre-existing, truthful, and not worth a code or prose change. I did not rule it INVALID, because the observation that the tier signal is drawn at file EXISTENCE rather than at the presence of evidence is a genuine property of the mechanism and a later round should not have to rediscover it.

### Disposition, not a fix

If the human wants it on the record, one line in the step's "Scope: what this step does not do" list: the tier boundary is file existence, so an empty log at the resolved path is inside the instrumented tier and reports a vacuously true `workflow invariants hold` until a step is marked `complete`. No `src/` change.

---

## `T-8` (`SC-3`) INVALID: acceptance check 20's behavioural half has no automated test

### Evidence, including the part that supports the reviewer

The reviewer's factual claim is TRUE and I confirmed it: `grep -rn "backstop\|instrumentation is on\|has no round log for it to read\|exits non-zero reporting that it could not run" src/*.rs tests/*.rs` returns no test asserting the qualifier's wording, and the only test touching the `{{instrument}}` slot (`src/main.rs:2462`, `instrument_off_omits_the_block_and_on_includes_it`) asserts only the presence and absence of the `Instrumentation (metrics logging)` section heading. The drift half IS automated (`src/agents_md_drift.rs:375` and `:415`), and a hand edit of `pack/AGENTS.md` followed by a re-render would pass it, since the guard compares the committed copies against a fresh render OF THAT PACK.

### Reasoning

The claim is true and the conclusion does not follow. Three reasons, in ascending order of weight.

1. CHECK 20 IS WRITTEN AS A MANUAL ACCEPTANCE CHECK AND THE SPEC DISTINGUISHES ITS TWO HALVES DELIBERATELY. Its text is "rebuild the fixture WITHOUT `--instrument` and grep its `AGENTS.md` for the backstop sentence ... THEN confirm the deployed copies are regenerated: `cargo test` passes, which includes the `agents-md-drift-guard` comparison". The spec names `cargo test` for the second half and a hand-run `grep` for the first. Asking why the first half is not in `cargo test` is asking why the spec is written the way it is, not reporting an unmet criterion. The criterion itself was met: a fresh non-`--instrument` render carries the qualifier, which the scope reviewer verified and I verified independently.
2. IT WOULD BE A NEW CLASS OF GUARD, NOT A MISSING INSTANCE OF AN EXISTING ONE. This tree does have per-fragment content guards (`src/isolation_policy.rs:78`, `src/recommendation_rule.rs:81`, `src/findings_naming.rs:156`, `src/workflow_spec.rs:241`, all `COMMITTED_AGENTS.contains(FRAGMENT)`), and I read one to see what they are for. They exist because the fragment is authored ONCE IN RUST and projected into MULTIPLE views (an `AGENTS.md` render slot AND the driver's reminder, or generated from the control constants), so the guard pins the projection against the single source. The backstop qualifier is hand-written pack prose with exactly one source and one set of consumers, already fully covered against the drift those guards protect against. Nothing in this repository asserts the presence of ordinary hand-written pack prose, for any sentence.
3. DECISIVE, AND IT IS AN INTERACTION BETWEEN FINDINGS THAT ONLY THE TRIAGE STEP CAN SEE: the remedy would hard-pin the exact wording that `T-2` proves is MIS-SCOPED. `SC-3` proposes asserting that the rendered `AGENTS.md` contains `"when instrumentation is on"` and `"has no round log for it to read"`, and `T-2` (raised independently by two other lenses, and reproduced above) establishes that the surrounding clause draws the tier boundary at the wrong thing. Applying `SC-3` this round would freeze a false claim into the suite and make `T-2`'s fix fail the test that was just added to protect it. A guard that pins prose is only as good as the prose, and this prose is a valid finding in the same round.

The remedy is also an ADDITION with nothing removed, on a round where the same reviewer set has surfaced three deletions and two substitutions that are all measurably true.

INVALID on ground 3 primarily, with 1 and 2 as independent support. I record it here rather than dropping it because ground 3 is contingent: if `T-2` is fixed and the qualifier's final wording is settled, a guard on the settled sentence becomes a coherent proposal, and it should be raised then, against the corrected text, rather than now.

---

## Tally

VALID FINDINGS, DEDUPLICATED: 6.

| Severity | Count | Findings |
| --- | --- | --- |
| critical | 0 | |
| high | 0 | |
| medium | 3 | `T-1` (`ADV-1`), `T-2` (`ADV-2` + `DOC-1`), `T-4` (`DOC-2`) |
| low | 3 | `T-3` (`ADV-3` + `SC-2`), `T-5` (`DOC-3` + `SC-1`), `T-6` (`DOC-4`) |

NOT COUNTED AS VALID: `T-7` (`ADV-4`) ACCEPT-AS-RESIDUAL; `T-8` (`SC-3`) INVALID.

RAW-TO-UNIQUE: 11 raw findings from 3 lenses, 8 unique, 6 valid.

SEVERITY CEILING: `medium`. No `high` and no `critical` was raised by any lens, and I found none. The increment's own contract, the exit status on the tier boundary, is correct on every input constructed here by three reviewers and by me: acceptance checks 15, 16, 17 and 18 all hold, the projections and `audit` are byte-identical to the parent build on the same fixtures, and no input was found where `--workflow` reports success over a check that did not run. Every valid finding is about what a message or a sentence ASSERTS, not about what the tool DOES.

RE-RATINGS I MADE, all stated with reasons at their findings: `ADV-2` `low` up to `medium` (`T-2`); `SC-2` `medium` down to `low` (`T-3`); `SC-1` `medium` down to `low` (`T-5`).

REMEDY SHAPE, since this project measures it: of the six valid findings, three are pure DELETIONS (`T-4` two words, `T-6` three words, `T-3` one character), two are SUBSTITUTIONS of equal or shorter length (`T-2`, `T-5`), and one is a message split confined to a single match arm (`T-1`). None requires new prose to be authored on a surface that is currently correct. `T-2` and `T-4` are the same sentence and must be one edit followed by one regeneration of the two deployed copies.

## Residuals and pinned costs

None of the eleven raw findings re-raises a residual accepted by human decision, and I checked each against all four. The in-root bound, the single-anchor `..` case, the earlier increment's `ADV-2` rejected-ledger context slot, and `R2A-2`'s off-convention `--source` surface are absent from all three files. This round's `ADV-2` is a DIFFERENT finding from the earlier increment's `ADV-2` and the reviewer says so explicitly in its own text; I confirmed they share nothing but the label.

Accepted costs (i) through (iv) were exercised as CONTROLS by the reviewers and by me, never as findings. `T-5` concerns the README sentence DESCRIBING cost (i) and not the cost itself, which I confirm behaves exactly as acceptance check 18 specifies. `T-2` references cost (i) only as part of the population that receives the mis-scoped sentence and does not ask for the cost to change.

---

## What the reviewers missed (my own observations, not triaged findings)

Recorded separately because these are mine and carry no reviewer credit. None of them is a finding raised against this increment; the first is corroboration that changes a severity argument, the second and third are method notes for whoever fixes this.

1. **THE UNRELEASED CHANGELOG ALREADY PROMISES THE USER THE DISTINCTION `T-1` COLLAPSES, one bullet below the bullet this increment added.** `ADV-1` grounded its argument in a source comment (`src/main.rs:1127-1133`) and one README line. There is a stronger, user-facing site nobody cited, in `CHANGELOG.md` under the same `## [Unreleased]` / `### Changed` heading:

   > ON DISK means the existence check answered yes: an anchor the check cannot answer for at all (a directory above it the process cannot traverse, a symlink loop, a name the kernel rejects) is grouped with the anchors that are not on disk rather than with the ones that are, so a path the tool could not check never becomes the one that decides, and its `note:` says the check failed rather than that the path is missing.

   That sentence names "a directory above it the process cannot traverse" as a case the tool distinguishes, and it is the release note directly BELOW the one this increment wrote. It is scoped to anchors, so it is not itself false. But it means the two bullets shipping in the SAME release tell the user that the tool separates "could not check" from "not there" for `--source` and `--plan`, while the newer bullet's mechanism does not for the log, on the identical filesystem arrangement. This does not add a finding, and I deliberately did not raise it as one; it raises my confidence in `T-1`'s severity, because the distinction is not merely an internal comment's preference but a promise already written into the release notes.

2. **A BETTER FIXTURE FOR `T-1` THAN THE ONE THE REVIEWER USED.** `ADV-1` used `chmod 000` on the log's directory. `chmod 600` is strictly better: the directory becomes readable but not searchable, so `ls docs/metrics` prints `workflow.jsonl` for the same user in the same shell while the tool reports no round log at that exact path. The falsehood is then visible in two adjacent commands with no reasoning in between. `chmod 111` is the matching negative control (searchable, not readable: the run succeeds at exit 0), which pins that the discriminator is search permission on the ancestor and nothing about the log's own mode. If a regression test is ever written for `T-1`, these three modes are the fixture.

3. **PLAIN `validate` IS ALREADY INTERNALLY INCONSISTENT ON UNREADABLE LOGS AT HEAD, independently of `--workflow`.** Measured on the parent build and on this one, identically:

   ```
   validate --source ... (no --workflow), log FILE at mode 000
     -> Error: Os { code: 13, kind: PermissionDenied }        exit=1
   validate --source ... (no --workflow), the log's DIRECTORY unsearchable
     -> no metrics log at docs/metrics/workflow.jsonl; nothing to validate    exit=0
   ```

   Same user, same log, same inability to read it, two different answers and two different exit codes. This is pre-existing, this increment neither created nor worsened it, and I am NOT raising it as a finding against inc3. I record it because it is the reason the one-token `try_exists()?` remedy measured under `T-1` is defensible rather than reckless: that edit makes these two lines agree, on both surfaces, rather than introducing a new failure class. Whoever weighs the two remedies for `T-1` should weigh this, and if the message-split remedy is chosen instead, this inconsistency stays and is worth a backlog line rather than silence.
