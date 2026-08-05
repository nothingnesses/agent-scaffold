# `workflow-enforcement-tier-inc3` work review, round 1, reviewer: ADVERSARIAL CONSTRUCTION

Artifact: `workflow-enforcement-tier-inc3`, branch `review/inc3-r1-adversarial` at `cd257dd` (two commits: `2356473` the fix, `cd257dd` the `SE-3` qualifier). Parent for every before/after comparison below: `230cdb8`.

METHOD. Two binaries were built and every claim was produced by running them, not by reading the diff:

- NEW: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-inc3-r1-adversarial/target/debug/agent-scaffold` (`cd257dd`).
- OLD: a detached worktree at `230cdb8` under this reviewer's own scratch directory, built independently.

Every fixture lives under `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/adv/`. `TMPDIR` was pointed at `.../scratchpad/adv/tmp` (outside any repository) for `cargo test`.

GATE RUNS, all clean on this branch: `cargo test` (all suites pass, including the three new cases and the two drift guards), `cargo clippy --all-targets -- -D warnings` (clean), `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` (`up to date`).

FOUR FINDINGS: one `medium`, three `low`. No finding claims a wrong exit code on the increment's own boundary: the exit-status contract inc3 changes answers correctly on every input constructed here, including the ones designed to break it. The `medium` is about what the new message ASSERTS on an input where it cannot know.

---

## `ADV-1` (medium): a round log that EXISTS but cannot be reached is reported as "no round log at <path>", with a remedy telling the operator to record rounds that are already recorded there

CLAIM. The new problem is gated on `metrics_path.exists()` (`src/main.rs:845`), which returns `false` both for "the file is not there" and for "the check could not be performed" (a directory above the log the process cannot traverse, a symlink loop, a name the kernel rejects). Inc3 turns that collapsed `false` into a loud, prescriptive problem that states a filesystem falsehood and misclassifies an INSTRUMENTED project into the guidance tier, which is the exact boundary this increment exists to report truthfully.

REPRODUCTION (self-contained; `/tmp/.../scratchpad/adv/repro_adv1.sh`):

```sh
R=<scratch>/adv1
mkdir -p "$R/docs/plans" "$R/docs/metrics"
cat > "$R/docs/plans/p.plan.toml" <<'EOF'
[meta]
title = "TOML-only project"
primary = "toml"

[[step]]
slug = "only-step"
title = "The only step"
status = "not-started"
order = 1
EOF
cat > "$R/docs/metrics/workflow.jsonl" <<'EOF'
{"type":"round","task":"only-step","artifact":"x","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk"}
EOF
chmod 000 "$R/docs/metrics"
cd "$R" && agent-scaffold validate --source docs/plans/p.plan.toml --workflow
```

OBSERVED, NEW (`cd257dd`), exit 1:

```
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there
```

OBSERVED, OLD (`230cdb8`), exit 0:

```
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
```

CONTROL, same command after `chmod 755 "$R/docs/metrics"`, NEW, exit 0: `docs/metrics/workflow.jsonl: 1 records, valid` / `docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold`. The log has one record and was there the whole time.

WHY THIS IS IN SCOPE AND NOT A PRE-EXISTING NOTE RE-RAISED. The conflation in `Path::exists` is pre-existing; what inc3 authored is the loud line built on top of it. This tree already decided that distinction, one increment earlier, in this same step, and wrote the reasoning into `src/main.rs:1127-1133`:

> THREE CASES, NOT TWO. `try_exists` separates "not there" from "there, but a directory above it cannot be traversed", which `Path::exists` collapses into one `false`. This is the whole Fail-loudly half of `Q-55-emptyroot`'s remedy, and a loud line that states a falsehood about the filesystem is worse than a quiet one: it sends the operator to fix a path that is already correct.

`note_missing_anchors` (`src/main.rs:1134-1148`) implements exactly that three-way split for ANCHORS and prints `note: --source <path> could not be checked: <error>` for the third case, and `README.md:236` documents the rule in user-facing prose ("its `note:` says the check failed rather than that the path is missing"). Inc3's new line is the same question asked of the LOG and answered the collapsed way, and its remedy clause ("record the project's review rounds there") is precisely "sends the operator to fix a path that is already correct".

WHY NOT HIGHER. The exit status, which is the contract inc3 changes, is right here: the check genuinely could not run, so non-zero is correct, and there is no false green in this class (an unreadable path never produces a pass). Only the reported diagnosis is false. Two neighbouring cases already answer honestly and are unchanged by this increment, so the collapse is narrow: a log file that is itself mode 000 and a directory sitting at the log path both propagate the io error (`Error: Os { code: 13, kind: PermissionDenied, ... }` and `Error: Os { code: 21, kind: IsADirectory, ... }`, exit 1, identical on NEW and OLD).

RIGHT BEHAVIOUR. Decide the new problem on `metrics_path.try_exists()` rather than `exists()`, and give the third case its own sentence in the same vocabulary inc2 established: on `Err`, say the check could not be performed and name the error, rather than asserting the log is absent and prescribing that rounds be recorded. The exit code stays 1 in both cases, so this is a message split and not a behaviour change. A broken symlink and a symlink loop at the log path also land in this branch today (both reproduced: NEW exit 1 with "no round log", OLD exit 0); the loop belongs with the "could not be checked" case, the dangling symlink is arguably fine as an absence but reads better as "the log path is a symlink that resolves to nothing".

---

## `ADV-2` (low): the shipped tier sentence names `--instrument` as the discriminator, but the tool's discriminator is a file on disk, and a project scaffolded WITH `--instrument` fails identically until its first record is written

CLAIM. `pack/AGENTS.md:93` (and the two regenerated deployed copies, and the narrower `README.md` sentence) tell the reader that the non-zero exit belongs to "a project scaffolded without `--instrument`". `scaffold --instrument` writes no `docs/metrics/` and no log; `pack/instrument.md:3` says the log is created lazily ("creating `docs/metrics/` if it does not exist"). So a freshly instrumented project gets the identical refusal, and a reader following the shipped sentence will conclude their project was scaffolded without `--instrument`, which is false.

REPRODUCTION:

```sh
mkdir -p <scratch>/f03/noinst <scratch>/f03/inst
(cd <scratch>/f03/noinst && agent-scaffold scaffold --output-dir . --write --force --principles default)
(cd <scratch>/f03/inst   && agent-scaffold scaffold --output-dir . --write --force --principles default --instrument)
ls <scratch>/f03/inst/docs                 # -> plans          (no docs/metrics)
(cd <scratch>/f03/noinst && agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow)
(cd <scratch>/f03/inst   && agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow)
```

OBSERVED, NEW (`cd257dd`): BOTH runs exit 1 with byte-identical output:

```
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there
```

OBSERVED, OLD (`230cdb8`): both exit 0 with the skip note. So the instrumented project's new failure is introduced by this increment, and the sentence introduced alongside it in the same commit attributes that failure to a property the project does not have. Both renders carry the sentence (`grep -c "when instrumentation is on, the deterministic"` returns 1 in each), and only the instrumented render also carries `## Instrumentation`.

SCOPE OF THE HARM. The window is scaffold-time to the first appended record, and the audience is an orchestrator agent reading its own `AGENTS.md`. It is a real misdiagnosis but a short-lived and self-correcting one, hence `low`. The same misreading is available outside that window, on any correctly instrumented project whose run is mis-anchored (accepted cost (i), pinned by check 18): the exit code and the message are identical, and the shipped sentence offers "you did not scaffold with `--instrument`" as the explanation.

RIGHT BEHAVIOUR. Name the condition the tool actually tests, which is also SHORTER than what is there now and so satisfies the spec's "smallest true qualifier is the target": the check needs a round log at the resolved path, so a project that keeps none, which includes every project scaffolded without `--instrument`, gets a non-zero exit saying the check could not run. That keeps acceptance check 20's property (a reader of the sentence alone can predict check 15's exit code) and stops the reader inferring the converse. This is NOT the `ADV-2` slot of the previous increment's review; that finding was about a rejected-ledger context slot and is unrelated.

---

## `ADV-3` (low): the new test's Markdown fixture is documented as schema-valid and is not, so case (b)'s exit-code assertion passes against the pre-fix build

CLAIM. `tests/validate_workflow_toml_source_needs_no_plan.rs`'s new `PLAN_MD` constant is introduced with the doc comment "A minimal, schema-valid Markdown `--plan` holding one `not-started` Roadmap step". Its Roadmap status is `not-started`, which is not in the Markdown vocabulary (`src/plan.rs:92-93`: `not started`, with a space). The plan validator therefore reports a problem on it, which makes the run exit 1 on its own, so the `assert_eq!(code, Some(1))` in case (b) of `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` is satisfied by a pre-existing plan-schema failure rather than by the tier policy.

REPRODUCTION, using the test's own two constants verbatim in a directory with no log:

```sh
cd <scratch>/f07/asis          # plan.plan.toml = PLAN_TOML, plan.md = PLAN_MD, verbatim
agent-scaffold validate --workflow --plan plan.md
```

OBSERVED, OLD (`230cdb8`), exit 1 (which is what case (b) asserts):

```
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
plan.md: Roadmap step `only-step` has an unknown status `not-started`
```

OBSERVED, NEW (`cd257dd`), exit 1, carrying both the same schema problem and the new one. And `agent-scaffold validate --plan plan.md` alone (no `--workflow`) on OLD is already exit 1 with the schema problem, which is the cleanest demonstration that the exit code in case (b) is over-determined.

THE ONE-CHARACTER FIX MAKES IT A TRUE RED. With `not-started` corrected to `not started` in `PLAN_MD` only (`<scratch>/f07/fixed`):

```
--- OLD (230cdb8): exit 0, stdout `plan.md: 1 steps, 0 open-questions items, valid`, stderr the skip note
--- NEW (cd257dd): exit 1, stderr `--workflow requested but no round log at docs/metrics/workflow.jsonl: ...`
```

So the Markdown arm IS genuinely covered by the fix, and the test can pin it attributably at the cost of one character.

WHY ONLY `low`. The test as a whole still goes red on a revert, because case (b)'s second assertion (stderr contains `no round log at ...` and `could not run`) is red pre-fix; case (a) is a clean, fully attributable red, verified by copying this file onto the `230cdb8` tree and running it (`workflow_with_no_metrics_log_hard_errors_instead_of_skipping ... FAILED`, `left: Some(0)`, `right: Some(1)`). The defect is a false claim in a fixture's doc comment plus one non-attributable assertion, not a lost guard.

RIGHT BEHAVIOUR. Spell the Roadmap status `not started` and keep the "schema-valid" claim, or drop the claim. The TOML fixture's hyphenated `not-started` is correct for the TOML schema and should not be touched.

---

## `ADV-4` (low): an empty file at the resolved path converts the new refusal into an affirmative `workflow invariants hold`, and this escape hatch is not recorded anywhere in the step

CLAIM. The tier boundary is drawn at file EXISTENCE, not at the presence of evidence, so `touch docs/metrics/workflow.jsonl` turns inc3's new hard failure back into a printed claim of success for any project with no `complete` step. The step's ordering argument says the tier policy goes last because "EVERY escape hatch a user reaches for is closed by an earlier increment" and names two (standing somewhere else, closed by inc1; `--metrics` at a foreign log, closed by inc2). This is a third, it is not closed, and it is in neither the accepted-cost list nor "What this step does not fix".

REPRODUCTION:

```sh
cd <scratch>/f05/d1                          # TOML-primary plan, single `not-started` step
mkdir -p docs/metrics && : > docs/metrics/workflow.jsonl
agent-scaffold validate --source docs/plans/p.plan.toml --workflow
```

OBSERVED, NEW and OLD identical, exit 0:

```
docs/metrics/workflow.jsonl: 0 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

Delete the same empty file and NEW exits 1 with the new problem while OLD exits 0. A whitespace-only log behaves the same as an empty one on both builds.

HONEST LIMIT OF THE CLAIM, which is why this is `low` and not higher. The green is vacuously TRUE: with no `complete` step, W3 has nothing to enforce, so the check did run and found no violation. The moment enforcement bites, the empty log fails correctly: the same fixture with `status = "complete"` and an empty log exits 1 on BOTH builds with the W3 message reporting that the step is complete but has no round records and no covering waiver (acceptance check 17's control, verified). So this is not a false green about any step's review evidence.

WHAT IS ACTUALLY WRONG. The increment's stated property is "a check that did not run must not report success", and the CHANGELOG tells a named population (every project scaffolded without `--instrument`) that their gate now fails. The one-command response available to that population produces `workflow invariants hold` on stdout, which reads as a stronger statement than "there was nothing to check". The distinction between an absent file and an empty file carries no information about whether the project is instrumented.

RIGHT BEHAVIOUR. Either record it as a known, accepted boundary in the step's own "what this does not fix" list (cheapest, and consistent with how the other costs are handled), or, if a stronger tier signal is wanted, say `workflow invariants hold (no round records to check)` when the log parses to zero records, so the affirmative line cannot be mistaken for evidence of enforcement. This reviewer's preference is the first: the second edits a line the correct case prints.

---

## What was attacked and produced nothing

Reported because a low-finding round is only credible if the covered surface is visible. Every item below was run on both binaries.

1. THE NO-`--workflow` PATH IS BYTE-IDENTICAL, on 16 inputs, comparing merged stdout+stderr and the exit code: bare `validate` (with and without a log present), `--source` only, a Markdown-primary `--source`, `--plan` only, `--source` plus `--plan`, a missing `--source`, a missing `--plan`, an explicit `--metrics` that is present, one that is missing, one that is malformed, an explicit `--metrics` outside the plan's root, a cross-project relative `--source`, an absolute `--source`, `--source` plus a missing `--plan`, and a bare filename run from inside `docs/plans`. All 16 IDENTICAL. This is the half the spec calls the easiest to break by accident, and it is not broken.

2. THE `_` ARM CAPTURES EXACTLY TWO TUPLES, `(Some(toml_primary), _, None)` and `(None, Some(plan), None)`, and no input was found where a non-zero exit is the wrong answer for either. The four match arms were exercised directly and each answered on its own terms.

3. PRECEDENCE AGAINST INC2'S CONTAINMENT REFUSAL IS CORRECT IN BOTH DIRECTIONS. An explicit `--metrics` outside the plan's root that does NOT exist still gets the containment refusal naming both paths and the derived root, not the new "no round log" line (NEW and OLD identical). An explicit `--metrics` INSIDE the root that does not exist gets the new line (NEW exit 1, OLD exit 0). A `..` that stays inside the root and is missing gets the new line. A `docs/metrics/workflow.jsonl` that is a symlink to `/dev/null` gets the containment refusal on both builds. A divergent `--source`/`--plan` pairing with a present foreign log gets the containment refusal on both builds. In every one of these the message that answered is the more useful of the two available.

4. PRECEDENCE AGAINST THE `(None, None, _)` ARM IS CORRECT. `--workflow` with no source and no plan, with a typo'd `--source`, or with a Markdown-primary `--source` and no `--plan`, all report `no plan source resolved` rather than the log message, on NEW and OLD identically, even when the log is also missing. The plan-source problem is the more useful diagnosis because the log path is derived from the anchor that did not resolve.

5. THE MESSAGE'S PRESCRIBED REMEDY ACTUALLY WORKS in the case most likely to receive it, accepted cost (i). From inside `docs/plans` with a bare `--source` filename, `--metrics ../metrics/workflow.jsonl` and the absolute spelling both reach the real log and print `workflow invariants hold` at exit 0, because the containment guard's root is CANONICAL and therefore finds the real `docs/plans` ancestor even though the lexical default could not. The lexical/canonical split makes the remedy sound; a canonical default would not have.

6. UNREADABLE INPUTS OTHER THAN THE TRAVERSAL CASE ARE NOT ABSORBED INTO "MISSING": a mode-000 log file and a directory at the log path both propagate the io error at exit 1, identically on NEW and OLD. Only the traversal/loop case is collapsed, which is `ADV-1`.

7. `status`, `next` and `status --resume` ARE UNTOUCHED on the same no-log fixture: byte-identical output and exit 0 on both builds, including `next`'s full `ACTIVE LOOP` block. The tier policy did not leak onto the projections.

8. NO SHIPPED ASSET INVOKES THE NEWLY FAILING COMMAND. `pack/checks.toml`, `pack/hooks/pre-commit`, `pack/prompts/*` and the repository `justfile` contain no `validate --workflow` invocation, so no scaffolded automation and no pre-commit hook starts failing for the broken population. The only pack mentions are prose in `pack/AGENTS.md` and `pack/instrument.md`.

9. DEGENERATE `--metrics` SPELLINGS produce no new wrong answer: a trailing slash and an over-long name both land in the new problem at exit 1 (correctly, the check cannot run), and an empty `--metrics ""` is rejected by clap at exit 2 on both builds.

10. THE DEPLOYED COPIES ARE IN SYNC. `cargo test` passes on this branch, which includes the `agents-md-drift-guard` comparison of root `AGENTS.md` and `.agents/AGENTS.reference.md` against a fresh render and the `prompt-drift-guard` over `.agents/prompts/`, so acceptance check 20's regeneration half holds.

## Residuals not raised

The four recorded residuals were checked against and deliberately not raised: the in-root bound (a foreign log copied inside the plan's own tree still joins by bare slug, unchanged here), the single-anchor `..` case, the previous increment's `ADV-2` rejected-ledger context slot, and `R2A-2`'s off-convention `--source` surface. Accepted costs (i) through (iv) were exercised as controls, not as findings; `ADV-2` above references cost (i) only as the population that receives the shipped sentence, and does not ask for the cost to be fixed. No new evidence was found that any of those verdicts was wrong.
