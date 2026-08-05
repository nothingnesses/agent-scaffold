# workflow-enforcement-tier-inc3, work review round 1, scope-and-test-coverage lens

Reviewed: `git diff main...HEAD` on `review/inc3-r1-scope` at `cd257dd` (two commits,
`2356473` and `cd257dd`, on top of merge-base `230cdb8`). Spec:
`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, the `workflow-enforcement-tier-inc3`
bullet, acceptance checks 15-20, the `INC3:` documentation-impact block, and "Scope: what this
step does not do".

## Verdict on the open scope question (the `--workflow` help clause)

**KEEP is correct.** The added sentence, `src/main.rs:438`, "So is no round log at the resolved
path at all: the check cannot run, and a check that did not run must not report success", is:

- **True.** It matches the code exactly: the new catch-all arm at `src/main.rs:1054-1057` pushes
  a problem for precisely this case, and the doc comment above `run_validate` (`src/main.rs:89-96`)
  states the identical reasoning.
- **A necessary completion, not scope creep.** Before this diff, the help string enumerated two
  `--workflow` error causes (no plan source resolved; log outside the project root). This
  increment adds a third live cause (no round log at all), so the enumeration would be
  incomplete-by-one if left alone. The general rule the sidecar states ("each documentation item
  travels with the increment that makes it stale") assigns the fix to whichever increment
  creates the staleness, which is inc3 here, regardless of the INC3 documentation-impact list's
  own silence on this specific string.
- **Now complete.** I checked the three-arm match (`src/main.rs:1004-1058`) plus the
  containment guard (`src/main.rs:989-1002`) against the three sentences in the help string
  (`no plan source resolved` / `log outside root, refused` / `no round log, error`) and every
  problem-producing path in the code is named exactly once.

Verified this against the pre-fix binary built from `230cdb8` in a separate worktree: see the
mutation-testing section below for the reproducible runs backing "the added clause is true."

The prompt's "most likely to yield something" lead (another enumeration left short by one)
did find something, but not in the help string: see SC-1.

## Findings

### SC-1 (medium): README.md:234 still describes the pre-inc3 answer for a case whose own example command now hard-fails

**Claim.** `README.md:234` ("One consequence to know about: a bare filename run from inside
`docs/plans` ... looks for `docs/metrics/workflow.jsonl` beneath `docs/plans` and reports that
it found no log; run it from the project root instead") is stale after inc3. Its own example
command, `cd docs/plans && agent-scaffold validate --source my-task.plan.toml --workflow`,
includes `--workflow`, which is exactly the flag whose accepted-cost-(i) answer this increment
changes from a stderr note at exit 0 to a hard failure at exit 1. This is the one other
enumeration/description left short by the change: the CHANGELOG entry added by this same diff
(`CHANGELOG.md:23`) correctly documents the new answer ("...which was a note at exit 0 and is
now this failure naming the path it looked for"), but the pre-existing README prose describing
the identical scenario was not updated to match, so the two committed documents now disagree
about what the literal example in the README does.

**Evidence.**
```
$ grep -n "bare filename" README.md
234:...a bare filename run from inside `docs/plans` (`cd docs/plans && agent-scaffold validate --source my-task.plan.toml --workflow`) has no parent directories to derive a root from, so it looks for `docs/metrics/workflow.jsonl` beneath `docs/plans` and reports that it found no log; run it from the project root instead.
```
Reproduced directly (built binary, current worktree HEAD):
```
$ mkdir -p fixture/docs/plans && cd fixture/docs/plans
$ agent-scaffold validate --source p.plan.toml --workflow
stderr: no metrics log at docs/metrics/workflow.jsonl; nothing to validate
stderr: --workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there
exit:   1
```
This is not "reports that it found no log" as a benign, exit-0 consequence; it is the hard
failure check 18 pins. The suite's own test for this scenario
(`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:462-478`,
`a_bare_filename_from_inside_docs_plans_stays_a_miss_and_now_fails_loudly`) asserts exactly this
exit code and message, which confirms the code and the CHANGELOG are right and the README
paragraph is the one document left behind.

**Remedy.** Update the sentence at `README.md:234` to state the current (post-inc3) answer:
that this exact command now hard-fails at exit 1 naming the path it looked for, not a soft note
at exit 0, matching the CHANGELOG entry's own wording and check 18.

### SC-2 (medium): the new test's Markdown-arm sub-assertion on exit code is satisfied by the pre-fix binary for an unrelated reason, and the fixture's own doc comment claim is false

**Claim.** In `workflow_with_no_metrics_log_hard_errors_instead_of_skipping`
(`tests/validate_workflow_toml_source_needs_no_plan.rs:180-253`), sub-case (b) (the Markdown
arm, `:208-217`) uses the `PLAN_MD` fixture (`:39-59`), whose doc comment claims it is "a
minimal, **schema-valid** Markdown `--plan`" and that "Only its PRESENCE matters below: the
tier policy answers before any check runs." Both claims are false: `PLAN_MD`'s Roadmap row uses
status `not-started` (hyphenated), but the Markdown schema's accepted vocabulary is
space-separated (`src/plan.rs:92-93`, `ROADMAP_STATUSES = ["not started", "in progress",
"complete", ...]`). `validate_plan` therefore reports `Roadmap step `only-step` has an unknown
status `not-started`` on every run against this fixture, independent of the round-log fix, which
means the pre-fix binary already exits 1 on sub-case (b) for a reason that has nothing to do with
the increment: the `assert_eq!(code, Some(1), ...)` line at `:210-214` passes against the
pre-fix build by accident. Only the following `stderr.contains("no round log at ...") &&
stderr.contains("could not run")` assertion (`:215-218`) actually discriminates the fix on this
sub-case.

**Evidence.** Built the pre-fix binary from the merge-base `230cdb8` (`git worktree add ...
230cdb8 --detach`, `cargo build`), ran the same fixture and args as sub-case (b):
```
PRE-FIX  ($ agent-scaffold validate --workflow --plan plan.md, streams separated):
  stdout: (empty)
  stderr: no metrics log at docs/metrics/workflow.jsonl; nothing to validate
          --workflow has a plan source but the metrics log is missing; skipping the workflow check
          plan.md: Roadmap step `only-step` has an unknown status `not-started`
  exit:   1                      <-- matches the test's `assert_eq!(code, Some(1), ...)` already, pre-fix

POST-FIX (same command, current worktree HEAD):
  stderr: no metrics log at docs/metrics/workflow.jsonl; nothing to validate
          plan.md: Roadmap step `only-step` has an unknown status `not-started`
          --workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there
  exit:   1
```
The exit code is identical before and after; the only thing that changes is the message, which
is what the second assertion checks. The whole test (as a compound assertion) still correctly
goes red-then-green because of that second assertion, so the defect this increment fixes is
still caught in practice, but the exit-code line pins nothing on its own, and the fixture's doc
comment overstates what was actually built ("schema-valid" is not true of `PLAN_MD`).

**Remedy.** Fix the one-character typo in `PLAN_MD` (`tests/validate_workflow_toml_source_needs_no_plan.rs:58`,
and the doc-comment example at `:44`): `not-started` -> `not started`, matching
`ROADMAP_STATUSES`. This makes the fixture actually schema-valid as its own comment claims, and
makes the exit-code assertion in sub-case (b) discriminate the fix on its own instead of by
accident.

### SC-3 (medium): acceptance check 20's behavioural half has no automated test, only its drift-guard half does

**Claim.** Check 20 (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:341`) has two
distinct halves. The drift-guard half ("confirm the deployed copies are regenerated: `cargo test`
passes, which includes the `agents-md-drift-guard` comparison ... plus the `prompt-drift-guard`
comparison") **is** automated: `src/agents_md_drift.rs:375` (`the_committed_scaffold_matches_a_fresh_render`)
and `:415` (`the_committed_role_prompts_match_a_fresh_render`) run in every `cargo test` and would
catch a stale deployed copy. The other half ("rebuild the fixture WITHOUT `--instrument` and
grep its `AGENTS.md` for the backstop sentence. It must now carry the instrumentation qualifier,
and a reader of that sentence alone must be able to predict check 15's exit code") has **no**
corresponding test anywhere in the suite. I searched for any test asserting the qualifier's
wording against a scaffolded (non-`--instrument`) fixture's rendered `AGENTS.md`; the only test
that touches the `{{instrument}}` slot at all is `src/main.rs:2462`
(`instrument_off_omits_the_block_and_on_includes_it`), and it only asserts the presence/absence
of the whole instrumentation section heading, never the SE-3 sentence's content.

**Evidence.**
```
$ grep -rn "backstop\|deterministic.*validate\|instrumented tier\|has no round log for it to read" src/*.rs tests/*.rs
(no hits in any test asserting the qualifier's wording)
```
I reproduced the manual half of check 20 by hand and confirmed the behaviour itself is correct
(this is a coverage gap, not a functional defect):
```
$ agent-scaffold scaffold --output-dir fixture --write --force --principles default
Wrote to fixture (30 changed, 0 left untouched).
$ grep -n "instrumentation is on" fixture/AGENTS.md
93:...when instrumentation is on, the deterministic `validate --workflow` check, once built, is the backstop ... and a project scaffolded without `--instrument` has no round log for it to read, so on such a project that check exits non-zero reporting that it could not run rather than passing...
```
This matches check 15's exit code (1) exactly as required, so the shipped behaviour is correct.
But nothing in `cargo test` pins it: a future hand-edit of `pack/AGENTS.md:93` that garbled or
dropped the qualifier while keeping the deployed copies in sync (a `render`-then-recommit, which
passes the drift guard trivially) would not be caught by the suite, only by a human re-running
this exact acceptance check. The plan's own risk write-up for this increment (workflow-enforcement-tier.md:305)
flags documentation-truthfulness claims about this exact boundary as the highest-miss-rate
artifact class this project has calibration data on, which is why I am treating this gap as more
than decorative.

**Remedy.** Add a test (in `src/main.rs`'s `build_assets` test module, beside
`instrument_off_omits_the_block_and_on_includes_it`, or as a small integration test) that builds
the `off` (non-`--instrument`) `AGENTS.md` asset and asserts it contains the qualifying clause,
for example asserting it contains both `"when instrumentation is on"` and `"has no round log for
it to read"` in the worktree-lifecycle paragraph.

## Mutation testing on the new arm (`src/main.rs:1054-1057`)

All three mutations were applied directly to `src/main.rs` in this worktree, tested with
`cargo test --no-fail-fast` (TMPDIR pointed outside any repo:
`.../scratchpad/rev-inc3-r1-scope/tmpdir-for-cargo-test`), then reverted via `Edit` before the
next mutation. Final `git status --short` / `git diff --stat` on this worktree are both empty;
no source edits remain.

| # | Mutation | Description | Caught? | Evidence |
|---|----------|-------------|---------|----------|
| 1 | Revert to pre-fix ("invert the condition") | Replaced `problems.push(format!(...))` with the old `eprintln!(...)` soft-skip (no problem pushed) | Yes | `a_bare_filename_from_inside_docs_plans_stays_a_miss_and_now_fails_loudly` FAILED (`left: Some(0), right: Some(1)`); `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` FAILED |
| 2 | Change the problem string | Changed the pushed message to `"MUTATION2: workflow check skipped, metrics log missing at {}"` (drops `"no round log at"` and `"could not run"`) | Yes | Same two tests FAILED, on the `stderr.contains(...)` assertions |
| 3 | Move the problem push outside the branch | Replaced the `_ => problems.push(...)` arm with `_ => {}` and pushed the same message unconditionally after the whole `match`, so it fires even on a passing `--workflow` run | Yes | `the_correct_case_prints_the_same_relative_paths_it_always_did`, `a_divergent_source_and_plan_pairing_is_refused`, `toml_primary_skips_the_markdown_plan_validator_but_markdown_mode_still_fails`, and `workflow_on_a_toml_source_runs_without_a_markdown_plan` all FAILED |

There is no boolean `if` guarding this specific arm (it is a match catch-all), so "inverting the
condition" was interpreted as the closest real analogue: reverting the arm's behaviour to the
pre-fix answer. All three mutations were caught decisively; the new arm itself has solid direct
test coverage. The coverage gaps found (SC-2, SC-3) are both about a *different* test's fixture
hygiene and a *different* acceptance check's automation, not about this arm's own mutation
resistance.

## The renamed test: honest rename, verified

`a_bare_filename_from_inside_docs_plans_stays_a_miss_and_now_fails_loudly`
(`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:462`, formerly
`..._stays_a_silent_miss`) still pins the same underlying miss unchanged: the assertion
`!stdout.contains("records, valid")` (`:475-478`) is untouched by the diff, and it is what
proves the project's real log (in `away/docs/metrics/workflow.jsonl`, one record) is never
reached from `away/docs/plans`. Only the *answer* to that miss changed, from
`assert_eq!(code, Some(0))` + a soft stderr-note match to `assert_eq!(code, Some(1))` + the new
hard-failure message match, exactly tracking inc3's behaviour change. This is not a weakening;
it is the correct re-pin of an accepted cost whose answer moved. No finding.

## Scope check: diff accounted for against the spec

Every changed file in `git diff main...HEAD --stat` maps to an item in the INC3 bullet or the
INC3 documentation-impact block, with no residual:

- `src/main.rs`: the `_` catch-all arm (`workflow-enforcement-tier-inc3`'s own bullet), the
  `run_validate` doc comment (impact list), and the `--workflow` help clause (resolved above).
- `pack/AGENTS.md:93`, `AGENTS.md`, `.agents/AGENTS.reference.md`: the SE-3 qualifier and its
  two regenerated deployed copies (impact list), verified byte-identical to each other and to a
  fresh `cargo run -- scaffold --output-dir . --write --force --principles default --instrument`
  render (the drift-guard tests pass).
- `README.md:210`: the `validate` paragraph gains the tier-boundary sentence (impact list). One
  omission found: `README.md:234` should have gained the matching update too (SC-1).
- `CHANGELOG.md`: one `Changed` entry naming the exit-code flip and the broken population
  (impact list).
- `tests/validate_workflow_toml_source_needs_no_plan.rs`, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs`:
  the module doc and the `:96-98`-region comment (impact list, now at different line numbers)
  plus the new/renamed tests (own red-then-green test, acceptance checks 15-18).

No orchestrator files were touched: `git diff main...HEAD --stat -- docs/plans/agent-scaffold.ledger.md
docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md docs/metrics/workflow.jsonl
'docs/plans/*.reviews/'` is empty. No step status changed and no increment was declared
(`agent-scaffold.plan.toml` is untouched by this diff). `cargo build`, `cargo test`, `cargo
clippy --all-targets -- -D warnings`, and `cargo run -- render docs/plans/agent-scaffold.plan.toml
--check` all pass clean on this worktree.

## Summary

3 findings, all medium severity: SC-1 (README staleness on the accepted-cost-(i) example),
SC-2 (a fixture defect that makes one sub-assertion in the increment's own new test coincidental
rather than discriminating), SC-3 (check 20's behavioural half has no automated test). The
disputed help-string clause is confirmed correct and complete (KEEP stands). The renamed test is
an honest re-pin. The new match arm has solid, verified mutation coverage. No scope creep and no
omission found in the core code/test change itself; both real gaps found are in surrounding
documentation and test-fixture hygiene rather than in the mechanism under review.
