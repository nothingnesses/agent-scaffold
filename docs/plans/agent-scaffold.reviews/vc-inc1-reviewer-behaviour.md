# `validation-constraints-inc1`, round 1, reviewer B: does the code do what was decided, and can it be broken

Artifact: `git diff main..HEAD` on `review/inc1-behaviour` (commit `fe5b31a`), which changes `src/workflow.rs`, `src/plan/source.rs`, `CHANGELOG.md`, `pack/instrument.md`, `AGENTS.md` and `.agents/AGENTS.reference.md`.

Specification: `docs/plans/agent-scaffold.steps/validation-constraints.md`, increment 1 and the whole Acceptance section.

Lens: behaviour and breakability only. Prose, naming and doc currency are another reviewer's lens and are not raised here, except where a claim in the diff is falsified by a run.

Everything below was built in a scratch copy under the session scratchpad. The worktree carries this file and nothing else.

## Summary

Two findings: one `medium`, one `low`. NO `critical` AND NO `high` FINDINGS, stated explicitly.

The shipped behaviour is correct on every tree I could construct. The `medium` finding is a mutation the whole suite and the whole acceptance list fail to catch, not a defect in what the code does today.

The three claims I was asked to settle all hold:

- The human's `Q-70-emptycase` choice (REPORT, over silence and over a log-non-empty conditional) is implemented unconditionally, and both declined options are caught by a test.
- W3 and W5 really do consult one implementation: one edit to `waiver_covers_round` reddens 16 tests spanning both checks.
- No tree the pre-fix binary refuses is accepted by the new binary except where the decided direction (iii) explains it. W3's verdict is unchanged over 32 constructed trees.

## Findings

### `W1A-1`: the INCREMENT axis of the shared predicate is unguarded, so a build that compares only the step passes the entire suite and the entire acceptance list while turning an unconverged risky increment into `workflow invariants hold` at exit 0

Severity: `medium`.

Claim. Deleting one line from `waiver_covers_round` (`src/workflow.rs:431`, `&& waiver.increment.as_deref() == Some(round_increment_id(round))`) leaves 382 of 382 unit tests passing, `clippy --all-targets -- -D warnings` clean, and acceptance items 1, 2, 3, 4, 4b, 5, 6, 7, 7b and 8 all satisfied. That build accepts trees which both the pre-fix binary and the correct fix refuse. This is the same class of hole acceptance item 4b was written to close on the STEP axis; the symmetric INCREMENT axis has no equivalent catch.

Reproduce the mutation (mutation id `m3` in the driver at `<scratch>/mutate.py`):

```
--- a/src/workflow.rs
+++ b/src/workflow.rs
@@ fn waiver_covers_round(
 	waiver.unit == WaiverUnit::Increment
-		&& waiver.increment.as_deref() == Some(round_increment_id(round))
 		&& waiver.step == round_step_slug(round)
 }
```

Evidence 1, the suite does not notice.

```
cargo test --bins   ->  test result: ok. 382 passed; 0 failed
cargo clippy --all-targets -- -D warnings  ->  exit 0
```

Evidence 2, W3 false green at exit 0. Fixture `<scratch>/fx-b`: Roadmap step `beta` is `complete`; increment `beta-incA` is `risky` with a peak `consecutive_clean` of 0 (needs 2); increment `beta-incB` converged and carries a correctly-scoped increment waiver. The waiver names `beta-incB`, not `beta-incA`.

```
prefix  : Roadmap step `beta` increment `beta-incA` reached a consecutive-clean streak of 0 but its `risky` risk class needs 2   exit=1
postfix : Roadmap step `beta` increment `beta-incA` reached a consecutive-clean streak of 0 but its `risky` risk class needs 2   exit=1
m3      : workflow invariants hold                                                                                              exit=0
```

Over the 32-tree W3 matrix at `<scratch>/matrix-w3.py`, the `m3` build suppresses a W3 shortfall the correct build reports in 17 trees. One of them is worse than a mis-scoped waiver: in `structured-both__increment-alpha-alpha-fold` the waiver names an increment id (`alpha-fold`) that occurs NOWHERE in the log, and it still exempts both of step `alpha`'s short increments.

Evidence 3, W5 false green, and the decided empty case defeated. Fixture `<scratch>/fx-matrix/alpha-inc1__alpha__other-inc-alpha`: the log carries one round joining increment `alpha-other` to step `alpha`, and the waiver names increment `alpha-inc1`, which the log never records.

```
prefix  : exit=0 (the retired lexical rule: `alpha-inc1` strips to `alpha`)
postfix : increment waiver names increment `alpha-inc1`, which has no `type:"round"` records, so the round log attributes it to no step   exit=1
m3      : workflow invariants hold   exit=0
```

Two further trees in the same matrix have the same shape (`alpha-fold__alpha__other-inc-alpha` and `beta-inc1__alpha__other-inc-alpha`), and in those two BOTH the pre-fix binary and the correct build refuse while `m3` accepts, so `m3` is a regression against the shipped tool as well as against the fix.

Why the suite misses it. The only test of the empty case, `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` (`src/workflow.rs:1555-1584`), passes `&[]` for `rounds`. A globally empty log cannot distinguish "no records for THIS increment" from "no records at all", so the increment axis is never exercised on the W5 side. On the W3 side there is no test in which one increment of a step carries a waiver and another increment of the same step is short: `a_step_waiver_does_not_exempt_a_short_streak_increment` pins the UNIT axis, and `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment` pins the STEP axis.

Verified fix, two lines in the existing test (mutation ids `guard` and `m3guard`). Give the empty-case fixture a log that is non-empty but lacks the waived increment:

```
-		let problems = w5_problems(&waivers(&waiver), &steps, &[], &escalations);
+		let other = rounds(&owning_round_line("alpha", "alpha-other"));
+		let problems = w5_problems(&waivers(&waiver), &steps, &other, &escalations);
```

Measured: with that change and the pristine predicate, 382 pass. With that change and `m3`, the test fails. So the change costs nothing and closes the hole.

Why `medium` and not `high`. The shipped code is correct, so nothing is wrong in the tool today; the exposure is that a later edit to the one predicate the whole enforcement tier now rests on ships green. The argument for `high` is that the demonstrated failure mode is a `workflow invariants hold` at exit 0 over an unconverged `risky` increment, which is the false-green class the tier exists to remove, and that the plan review judged the symmetric step-axis case worth a dedicated acceptance item on exactly that reasoning.

### `W1A-2`: on a pre-migration round record the refusal names a step derived by `leading_slug` from the record's `task`, which can exist in neither the Roadmap nor the log, so the CHANGELOG's "the records actually attribute" is stronger than the code guarantees

Severity: `low`.

Claim. The mis-scope message says "the round log attributes increment `X` to step `Y`". When the joining record carries no structured `step`, `Y` is `leading_slug(round.task)`, a value the log does not carry and the plan need not contain. The verdict is right; the message's provenance claim is not.

Reproduce. Fixture `<scratch>/fx-matrix/alpha-fold__alpha__bare-task-only`. The log is one pre-migration round record plus an escalation plus the waiver:

```
{"type":"round","task":"alpha-fold","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk"}
```

Post-fix output:

```
round log line 3: increment waiver names step `alpha` but the round log attributes increment `alpha-fold` to step `alpha-fold`
```

`alpha-fold` is not a Roadmap step (the Roadmap has `alpha` and `beta`), and no record in the log carries `"step":"alpha-fold"`. The tool derived it lexically, from the same `leading_slug` shim the fix set out to retire.

How live the record shape is, in this repository's own log: 236 `type:"round"` records, of which 123 carry a structured `step`, so 113 do not.

```
grep -c '"type":"round"' docs/metrics/workflow.jsonl                          -> 236
grep '"type":"round"' docs/metrics/workflow.jsonl | grep -c '"step":'         -> 123
```

The recorded `src/` defect the increment closes "by construction" is that W5's ownership message names a Roadmap step that does not exist. It is closed for the case it was recorded against (a step derived from the WAIVER's increment id), and it survives on the pre-migration record path (a step derived from the ROUND's task). The affected sentence in the diff is `CHANGELOG.md`: "A refusal now names the step or steps the records actually attribute the increment to."

Why `low`. No verdict changes, the derived value is the project's own documented join accessor (`round_step_slug`, which W3 also uses to route the same record), and the predicate's own doc comment is honest that a pre-migration record "falls back per axis". Only the emitted sentence and the CHANGELOG sentence overstate. A treatment could be as small as saying "joins" rather than "attributes", or naming the fallback where it was used.

## The mutation battery

Driver: `<scratch>/mutate.py` (exact-string replacement with an occurrence-count assertion, so a stale mutation fails loudly rather than silently rebuilding the pristine tree) and `<scratch>/run-mutation.sh`. Each run rebuilds from the pristine `src/workflow.rs` and reports the md5 of the built test binary; all eighteen md5s are distinct, so no run reused a stale fingerprint.

Control: `none` -> 382 passed, test binary `78f08d72`.

| id | mutation | caught | tests that went red |
| --- | --- | --- | --- |
| `m1` | invert the step comparison in the shared predicate | YES, 16 tests | `a_bare_slug_increment_waiver_exempts_a_short_streak`, `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment`, `a_short_streak_increment_with_a_covering_increment_waiver_passes`, `check_workflow_passes_the_optional_modules_migration_shape`, `check_workflow_toml_passes_the_optional_modules_accepted_at_escalation_waiver`, `check_workflow_toml_passes_the_waiver_model_self_referential_waiver`, `check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step`, `w5_accepts_an_increment_waiver_whose_id_does_not_strip_to_its_step`, `w5_accepts_the_three_valid_reason_tier_pairings`, `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`, `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment`, `w5_flags_a_record_backed_waiver_citing_an_unrelated_escalation`, `w5_flags_a_record_backed_waiver_whose_escalation_resumed_not_decided`, `w5_flags_a_record_backed_waiver_with_no_matching_escalation`, `w5_names_every_step_the_log_attributes_a_waived_increment_to`, `w5_passes_a_record_backed_waiver_with_a_matching_escalation` |
| `m2` | drop the STEP axis (compare the increment, not the step) | YES, 4 tests | `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment`, `check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step`, `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment`, `w5_names_every_step_the_log_attributes_a_waived_increment_to` |
| `m3` | drop the INCREMENT axis (compare the step, not the increment) | NO, 382 passed | none. See `W1A-1` |
| `m4` | drop the unit check from the predicate | NO, 382 passed | none, and it is an EQUIVALENT MUTANT rather than a gap: both waiver projections enforce `increment` present iff `unit == increment`, so a step-unit waiver always has `increment: None` and fails the `Some(...)` comparison anyway. `a_step_waiver_does_not_exempt_a_short_streak_increment` still passes for the right reason |
| `m5` | W5's `any` becomes `all` | YES, 1 test | `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |
| `m6` | report only when the log is non-empty (the option the human DECLINED) | YES, 1 test | `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |
| `m6b` | stay silent on the empty case (the other option the human DECLINED) | YES, 1 test | `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |
| `m7` | acceptance item 4b's own mutation: the step axis becomes a caller-supplied value and W5 passes the waiver's own `step` | YES, 3 tests | `check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step`, `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment`, `w5_names_every_step_the_log_attributes_a_waived_increment_to` |
| `m8` | compute `owners` on the step axis instead of the increment axis | YES, 3 tests | as `m7` |
| `m9` | invert the singular/plural branch of `step_attribution` | YES, 3 tests | as `m7` |
| `m10` | W3's inner `any` over an increment's records becomes `all` | NO, 382 passed | none, and it is an EQUIVALENT MUTANT: every record in a group shares both join axes by construction of the group, so `any` and `all` agree |
| `m11` | `run_checks` hands W5 an empty `rounds` slice | YES, 4 tests | `check_workflow_passes_the_optional_modules_migration_shape`, `check_workflow_toml_passes_the_optional_modules_accepted_at_escalation_waiver`, `check_workflow_toml_passes_the_waiver_model_self_referential_waiver`, `check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step` |
| `m12` | revert W5 to the retired lexical rule, keeping the new messages | YES, 2 tests | `w5_accepts_an_increment_waiver_whose_id_does_not_strip_to_its_step`, `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |
| `m13` | W5 inlines the join instead of calling the shared predicate | NO, 382 passed | none, and it is an EQUIVALENT MUTANT: the inlined body is the predicate's body. It shows that SHARING is a structural property no test can pin, which is why acceptance item 7 is written as a mutation of the predicate and is satisfied by `m1` |
| `m3m1` | `m3` plus `m1`, to settle that `m3` still satisfies acceptance item 7 | YES, 16 tests | the same 16 as `m1`, spanning W3 and W5, so the `m3` build's predicate is still genuinely shared and item 7 is satisfied by it |
| `guard` | the proposed `W1A-1` fix, applied alone | control: 382 passed | none, so the fix costs nothing |
| `m3guard` | the proposed `W1A-1` fix plus `m3` | YES, 1 test | `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |

Commands:

```
bash <scratch>/run-mutation.sh <id>
# = mutate.py <id> postfix/src/workflow.rs mut/src/workflow.rs
#   then CARGO_TARGET_DIR=<scratch>/target-mut cargo test --bins
```

## Pre-fix versus post-fix verdicts

Binaries built from separate `git archive` extracts into SEPARATE target directories, and confirmed distinct:

```
b8bef4db3fbe4faddeaa73374d9d9eed  target-prefix/debug/agent-scaffold   (main)
bf09c494cf815f8fbc3779f1112d1174  target-postfix/debug/agent-scaffold  (HEAD)
03cbb08afb20529aff2c6a402605df2a  target-m3/debug/agent-scaffold       (HEAD plus m3)
```

W5, 45 constructed Markdown-plus-JSONL trees (`<scratch>/matrix.py`): three increment-id shapes (`alpha-inc1`, `alpha-fold`, `beta-inc1`) x three waiver steps (`alpha`, `beta`, a ghost slug) x five log shapes (no rounds, structured join to `alpha`, structured join to `beta`, pre-migration bare `task`, a round for a DIFFERENT increment of `alpha`). Ten trees change verdict.

Post-fix ACCEPTS and pre-fix REFUSED, four trees. Every one is explained by the decided direction (iii), where the round log and not the id decides ownership:

- `alpha-fold__alpha__structured-alpha`: THE UNBLOCKING. The id does not end `-inc<alnum>`, so the retired rule could never accept it; the log joins it to `alpha`.
- `alpha-inc1__beta__structured-beta`, `alpha-fold__beta__structured-beta`, `beta-inc1__alpha__structured-alpha`: the id strips to one step and the log joins the increment to another. The log wins, which is the decision.

Post-fix REFUSES and pre-fix accepted, six trees. All are the documented narrowing: the waived increment has no round records at all (`__no-rounds` and `__other-inc-alpha`), or the log joins it to a step other than the waiver's.

W3, 32 constructed trees (`<scratch>/matrix-w3.py`), both steps `complete`, every round `risky` at a peak streak of 0, across structured, pre-migration, crossed and fold-shaped logs, and eight waiver shapes: 0 of 32 differ on the W3 problem set. The refactor into `waiver_covers_round` is verdict-preserving for W3, measured rather than argued. It is also provable from the code: `matching` is filtered by `round_step_slug(round) == step.slug` and the group key is `round_increment_id(round)`, so the new per-record comparison is the old per-group comparison.

The live plan, acceptance item 2, unmodified plan and unmodified log:

```
prefix : docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold   exit=0
postfix: docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold   exit=0
```

The unblocking, acceptance item 3, rebuilt independently rather than taken from the implementer (`<scratch>/unblock.py` copies the live plan and log, declares both fold tokens as `[[step.increment]]` entries, writes waivers `-w5` and `-w6`, and flips `workflow-enforcement-tier` to `complete`):

```
prefix : TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`
         TOML waiver `workflow-enforcement-tier-w6`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-endproperty-fold` belongs to step `workflow-enforcement-tier-endproperty-fold`
         exit=1
postfix: workflow invariants hold   exit=0
```

The pre-fix messages above are also the live instance of the recorded `src/` defect: neither `workflow-enforcement-tier-fold` nor `workflow-enforcement-tier-endproperty-fold` is a Roadmap step.

## The human's decision, `Q-70-emptycase`

The receipt in `docs/metrics/workflow.jsonl` is:

```
{"type":"decision","task":"validation-constraints","q_id":"Q-70-emptycase","options":["Report it","Stay silent on it","Report it, but only when the log is non-empty"],"recommendation":"Report it","chosen":"Report it","ts":"2026-08-12"}
```

Implemented as chosen. The report at `src/workflow.rs:617-621` is reached whenever `owners` is empty, with no guard on the log's size and no guard on the waiver beyond its unit and the presence of its `increment`. Both declined options were built and both are caught: `m6` (the log-non-empty conditional) and `m6b` (silence) each redden `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`. No declined conditional has crept in.

## The sharing is real

Acceptance item 7's demonstration, by mutation and not by reading. `m1` changes one line inside `waiver_covers_round` and reddens tests on BOTH sides:

- W3 side: `a_short_streak_increment_with_a_covering_increment_waiver_passes`, `a_bare_slug_increment_waiver_exempts_a_short_streak`, `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment`.
- W5 side: `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment`, `w5_accepts_an_increment_waiver_whose_id_does_not_strip_to_its_step`, `check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step`, and nine more.

There is also no third consumer left holding a copy of the relation. After the change, the only production call sites of `leading_slug` are `round_step_slug` and `escalation_step_slug`; `src/next.rs` contains no waiver logic at all.

```
grep -rn "leading_slug(" src/ | grep -v tests   -> only round_step_slug and escalation_step_slug
grep -n "waiver" src/next.rs                    -> no hits
```

## Messages checked against the data

Every message the new code can emit was checked against a constructed instance.

- "increment waiver names step `A` but the round log attributes increment `X` to step `B`". `B` comes from `owners`, which is non-empty on this branch and is built from the records carrying `X`. It can never contain `A`, because a record carrying both would have satisfied the predicate. TRUE of the data, with the pre-migration caveat in `W1A-2`.
- "... to steps `B`, `C`" for several owners. Reachable only on the JSONL substrate, deterministic (a `BTreeSet`, so sorted), and pinned by `w5_names_every_step_the_log_attributes_a_waived_increment_to`.
- "increment waiver names increment `X`, which has no `type:"round"` records, so the round log attributes it to no step". True of every well-formed record in the log. See the note below for the malformed case.

## Examined and NOT raised

- A round record that `parse_rounds` drops but `validate_log` accepts would make the empty-case message false of the file. There is no such record: `require_count` and `parse_rounds` both use `as_u64`, and `check_record` requires a superset of the fields `parse_rounds` requires. When a record IS malformed, both messages print on the same run, so the reader is not misled: a fixture with a round missing `risk_class` produces `docs/metrics/workflow.jsonl:1: missing field risk_class` immediately above the W5 line.
- The rounds slice W5 now reads is unfiltered by project. That is `validation-constraints-inc6`'s subject and the step already records it as inc6's fourth limitation.
- W5's own dangling-step check firing alongside the ownership check produces two problems for one waiver. The retired rule did the same.
- Performance: W5 is now O(waivers x rounds) plus one O(rounds) scan per refusal. On the live log (318 records, 96 steps) `validate --workflow` is unmeasurably fast.
- `src/plan/source.rs`'s change is eight comment lines and is behaviourally inert (`git diff main..HEAD -- src/plan/source.rs`).
- The plan-side unblocking edits (the two `[[step.increment]]` declarations, the two owed waivers, the status flip) are not in this diff. The step assigns them to the orchestrator and the planner, so their absence is not a finding against this artifact. I built them myself to settle acceptance item 3, above.

## Checks re-run for this review

```
cargo build                                     exit 0
cargo test                                      426 tests across 9 binaries, 0 failed
cargo clippy --all-targets -- -D warnings       exit 0
validate --source <live plan> --workflow        exit 0, workflow invariants hold
grep -c -F "the increment's leading slug equals the step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md   -> 0, 0, 0
grep -c -F "the round log must join that increment to that step, so an increment with no round records at all is reported" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md   -> 1, 1, 1
```

`TMPDIR` was pointed outside every git repository for the suite runs, per the Acceptance section's preamble.
