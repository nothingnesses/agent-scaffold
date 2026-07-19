# Inc 2 round 2 review (structured-skeleton-inc2) -- CONFIRMING pass

Reviewer: independent (not producer, not orchestrator, not the round-1 reviewers). Adversarial confirming round on the round-1 fix commit `e395d44 -> f0ad390`; full increment `a780541 -> f0ad390`. Contract: the `structured-skeleton-inc2` bullet in `docs/plans/agent-scaffold.md` (additive optional `step`/`increment` ids on `round`/`escalation`; W3/W5 prefer them, fall back to `leading_slug(task)`/`task`).

## Verdict: CLEAN

No new valid finding after a genuine adversarial pass. The four round-1 findings are all resolved (three by code/doc changes in the fix commit; S1 is a tracked merge-gate that legitimately remains pending, see below). One below-threshold observation is recorded for transparency; it is NOT a finding and needs no fix.

## Build/run verification (all reproduced against `f0ad390` in the read-only worktree)

- `cargo test --all-targets`: 235 passed, 0 failed (matches the implementer claim of 235). Summed across all binaries.
- `cargo clippy --all-targets -- -D warnings`: clean, no diagnostics.
- `cargo run -- validate --metrics docs/metrics/workflow.jsonl --plan docs/plans/agent-scaffold.md --workflow`: exit 0, "workflow invariants hold" (116 records, 51 steps, 46 open-questions items, all valid). The live log is all pre-migration records (no structured ids), so this also exercises the fallback path end to end.

## Round-1 fix confirmation

- O1 + S3 (single-field independent optionality): FIXED and non-vacuous. All six claimed tests exist and assert real behavior:
  - `a_round_with_only_a_structured_step_is_accepted`, `a_round_with_only_a_structured_increment_is_accepted`, `an_escalation_with_only_a_structured_step_is_accepted`, `an_escalation_with_only_a_structured_increment_is_accepted` (`src/metrics.rs`): each feeds a record carrying exactly one of the two ids and asserts `validate_log(line) == Vec::new()`. Non-vacuous: they would fail if `require_structured_ids` coupled the two fields.
  - `w3_an_increment_only_round_falls_back_to_the_shim_on_the_unfilled_step_axis` (`src/workflow.rs`): asserts `leading_slug("foo-incidental") == "foo"`, then that an `increment`-only round over-strips on the unfilled step axis so the `complete` step sees no matching rounds (`problems.len() == 1`, message contains "has no round records and no covering waiver"). Non-vacuous; pins the residual T3 over-strip as the chosen outcome.
  - `w3_a_step_only_round_joins_on_its_structured_step_and_falls_back_on_the_increment_axis` (`src/workflow.rs`): asserts `problems.is_empty()` for a `step`-only round whose `task` (`foo-incidental`) would over-strip; passing requires `round_step_slug` to use the structured `step`, so the step-axis half is non-vacuously pinned (it would fail if the code fell back to `leading_slug`).
  - Code comment: the block comment above the four join accessors in `src/workflow.rs` documents per-field independence explicitly ("each accessor therefore falls back on its OWN field alone, with no coupling to the other"), matching the code and the contract.
- S1 (deployed `AGENTS.md` / `.agents/AGENTS.reference.md` stale vs `pack/instrument.md`): STILL PENDING, as tracked. Fixed-string grep confirms the Inc 2 schema text ("Two further optional fields link the record to the plan") and the new waiver-join wording are present only in `pack/instrument.md`, absent from both deployed copies. Per the triage this is a hard MERGE-GATE item resolved by the orchestrator's `just scaffold-self` regen at merge, not by the implementer's fix commit; it is expected to be open here. Not complete until the deployed copies are regenerated. This is a completion gate, not a code regression, and the fail direction is benign (old records still validate; new records keep using the shim).
- S2 (waiver bullet's W5 escalation-join description in `pack/instrument.md`): FIXED and accurate to code. The bullet now reads "the escalation's structured `increment` id, or its `task` when that id is absent, equals the waived increment; or its structured `step` slug, or `leading_slug(task)` when that id is absent, equals the waived step". This matches `w5_problems`: `WaiverUnit::Increment => waiver.increment.as_deref() == Some(escalation_increment_id(escalation))` (structured increment else `task`) and `WaiverUnit::Step => escalation_step_slug(escalation) == waiver.step` (structured step else `leading_slug(task)`). Deployed-copy propagation is the S1 merge-gate tail.

## Adversarial checks (ruled out)

- No regression from the fix commit: 235 tests green, clippy clean, live `validate --workflow` green.
- Backward compatibility holds: `require_structured_ids` treats absent fields as fine; `parse_rounds`/`parse_escalations` map empty-or-absent to `None`; the join accessors fall back to `leading_slug(task)`/`task`. Pinned by `w3_a_pre_migration_round_still_joins_its_step_via_leading_slug`, `parse_rounds_projects_optional_structured_ids`, `parse_escalations_projects_optional_structured_ids`, and the green live run over an all-pre-migration log.
- Empty-string guard is real: `a_round_with_an_empty_structured_step_is_reported` and `an_escalation_with_an_empty_structured_increment_is_reported` assert a present-but-blank id is rejected ("field `<name>` is empty"), and `parse_*` filter empties to `None` so a blank never becomes a join key.
- Waivers gained no field (Q-46): the `Waiver` struct and `parse_waivers` are unchanged in the increment; only how W5 READS the escalation side changed (`escalation_increment_id`/`escalation_step_slug`). Confirmed by diffing `src/metrics.rs`/`src/workflow.rs`.
- No new defect in the added tests/comment/doc: the comment and the two W3 join tests describe behavior that matches the code; the S2 doc wording matches `w5_problems`.

## Non-blocking observation (NOT a finding, no fix required)

- N1, informational, `src/workflow.rs` `w3_a_step_only_round_joins_on_its_structured_step_and_falls_back_on_the_increment_axis`. The test name and comment claim to pin BOTH the filled step axis (structured id used) AND the unfilled increment axis (fallback to raw `task`). The single assertion (`problems.is_empty()`) non-vacuously pins the step-axis half, but with only one round in the group the increment grouping key value does not affect the outcome (a lone `clean` low_risk round converges under any key), so the increment-axis fallback claimed in the name is exercised but not independently asserted here. This is not a defect: the increment-axis `None -> task` fallback is mechanically identical to pre-migration grouping and is separately covered by `parse_rounds_projects_optional_structured_ids` and the increment-only step-axis test. I raise it only for transparency; it does not warrant a change and does not reset convergence. Direction to consider IF ever touched: not needed. Left as-is.

Line length was not treated as a finding.
