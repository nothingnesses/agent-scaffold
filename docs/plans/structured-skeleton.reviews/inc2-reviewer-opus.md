# Inc 2 review (reviewer: opus) - lens: soundness / correctness

Change under review: commit `e395d44` on base `a780541`. Structured `step`/`increment` ids added to `round`/`escalation` JSONL records; W3/W5 prefer them, fall back to `leading_slug(task)`.

Verification run (all in the Nix env, run from the change worktree `.claude/worktrees/structured-skeleton-inc2` at `e395d44`, since main is checked out at the base without the change):

- `cargo test --all-targets`: GREEN, `225 + 1 + 3 = 229` passed, matching the claim. (An earlier run from the main working tree showed 218; that tree is the base `a780541` and lacks the new tests, so it does not reflect the change. Corrected.)
- `cargo clippy --all-targets -- -D warnings`: clean, confirmed.
- `cargo run -- validate --metrics docs/metrics/workflow.jsonl --plan docs/plans/agent-scaffold.md --workflow`: green, confirmed (`116 records, valid`; `51 steps, 46 open-questions items, valid`; `workflow invariants hold`).

The new join tests are NOT vacuous: `w3_a_round_carrying_a_structured_step_joins_without_the_lexical_strip` asserts `leading_slug("foo-incidental") == "foo"` and then proves the structured record still joins to the `complete` `foo-incidental` step; its companion `w3_the_same_task_without_the_structured_step_over_strips_and_is_missed` proves the fieldless record is caught by the pause.md catch; `w3_a_pre_migration_round_still_joins_its_step_via_leading_slug` proves the shim fallback; and the two W5 tests mirror this for the escalation join. The Principle 11 "prove the real path" requirement the task named is met for the step-join axis.

## Findings

### O1 (low): `step` and `increment` fall back INDEPENDENTLY, so a partially-populated record leaves a T3 residue on one axis, and this is untested

Evidence: `src/workflow.rs` `round_step_slug` (`round.step.as_deref().unwrap_or_else(|| leading_slug(&round.task))`) and `round_increment_id` (`round.increment.as_deref().unwrap_or(&round.task)`) each fall back on their own field, with no coupling; `escalation_step_slug` / `escalation_increment_id` mirror this. `src/metrics.rs` `require_structured_ids` checks `step` and `increment` independently (one `for name in ["step","increment"]` loop, `contains_key` per field), so a record may carry `increment` WITHOUT `step` (or vice versa) and still validate.

Consequence: a record that carries `increment` but omits `step` still resolves its STEP join through `leading_slug(task)`. If its `task` is exactly the over-strip shape the increment exists to fix (a slug ending `-inc<alnum>`, e.g. `foo-incidental`), the step join still over-strips to `foo` despite the record carrying structured data. The change's own claim ("a record with the field joins without ever reaching this lexical strip", `leading_slug` doc comment) is therefore true only when BOTH fields are present; an increment-only record reintroduces exactly the SE-10/B6/T3 over-strip on the step axis. Symmetrically, a step-only record groups by the raw `task` for the increment identity.

Test gap: every new test supplies both fields together (or neither); there is no test for a step-only or an increment-only record, so the independent-optionality the schema permits is unexercised.

Why it is low, not higher: Inc 2 is additive and the orchestrator, following the instrument.md doc (which introduces the two as a pair), will normally write both; the failure is data-dependent and, on the step axis, tends to fail toward a false W3 FAILURE (safe direction, see O2) rather than a silent pass. But it is a genuine soundness edge the contract's "retire T3 for new data" wording does not actually guarantee.

Direction: either document that the two ids are meant to co-occur and require it in `require_structured_ids` (reject one-without-the-other), or explicitly accept partial records and add a test pinning the increment-only step-join behaviour so the residual over-strip is a known, chosen outcome rather than an accident.

### O2 (low): no check that a structured `step`/`increment` id resolves to a real plan step/increment; a typo silently removes a round from W3's view

Evidence: `parse_rounds` / `parse_escalations` accept any non-empty string for `step`/`increment`; nothing in this change cross-references them against the plan's steps or increment ids (that is deferred to Inc 4 per the plan). In `w3_problems`, `matching = rounds.iter().filter(|round| round_step_slug(round) == step.slug)`. A round whose `step` is a typo matches no `complete` step and drops out of convergence accounting entirely.

Why it matters: before Inc 2, a round's step identity was derived mechanically from `task` via `leading_slug`, so it could not silently point at a nonexistent step. Now an operator-authored `step` string is trusted verbatim with no resolvability check. The direction is mostly safe: a mislabeled `step` makes the real `complete` step see "no matching records" -> the pause.md catch FAILS it (a false failure, not a false pass), and a mislabeled `increment` that splits one loop's rounds across two ids generally trips `round_log_consistency_problems` (the recomputed streak per split group disagrees with the logged `consecutive_clean`). So the enforcement semantics are not laundered open. It is still an unvalidated trust boundary worth recording, since a plausible fat-finger degrades W3 into a spurious failure with a confusing message.

Direction: fine to defer the resolvability check to Inc 4 (where the TOML `[[step.increment]].id` is the join target), but note the gap explicitly so it is not assumed already covered; optionally add a test showing a bogus `step` yields the pause.md failure so the fail-safe direction is pinned.

## Checked and ruled out (no finding)

- Backward compatibility: a fieldless (pre-migration) record projects `step: None` / `increment: None` (`parse_rounds_projects_optional_structured_ids`, `parse_escalations_projects_optional_structured_ids`) and joins via the unchanged `leading_slug`/`task` shim; `require_structured_ids` returns `Ok` when neither key is present, so old records validate exactly as before. The live-log `validate --workflow` stays green.
- Empty-string values: rejected at schema time (`require_structured_ids` -> `field \`step\`/\`increment\` is empty`, tests `a_round_with_an_empty_structured_step_is_reported`, `an_escalation_with_an_empty_structured_increment_is_reported`) AND defensively treated as absent by `parse_*` (`.filter(|s| !s.is_empty())`), so a blank id can neither validate nor join to an empty slug. Consistent.
- Wrong-type field (e.g. `step: 123`): `require_str` inside `require_structured_ids` flags it; `validate` runs the schema check alongside `--workflow` (observed: it prints `116 records, valid` before the invariant line), so a non-string field is caught rather than silently coerced to `None`.
- Double-counting: W3 applies one `filter` then one `BTreeMap` grouping; `round_log_consistency_problems` groups once; no record is counted twice.
- W3 increment grouping and W5 scope joins consistently route through `round_increment_id` / `escalation_increment_id` / `round_step_slug` / `escalation_step_slug`; grep of the post-change `workflow.rs` shows no residual raw `round.task` / `leading_slug(&...task)` on a join path that a sibling call now routes through a helper (the remaining `leading_slug(increment)` at the W5 "step-owns-increment" structural check is on the waiver's own field, unchanged and out of Inc 2 scope since waivers do not gain the structured id per Q-46).
- Waivers correctly did NOT gain the field (Q-46 moves them to TOML): the diff touches only `round` and `escalation` schema arms and structs.
- Import reordering in `workflow.rs` (`question_id_index`, `QUEUE_FOLD_PREFIX`) is rustfmt cosmetic, not a behaviour change.

## Summary

No high or critical findings. Two low findings: O1 (independent step/increment fallback leaves a T3 over-strip residue for partially-populated records, untested), O2 (no resolvability check on structured ids; mislabels degrade W3 toward a false failure). Verification confirmed against the change worktree: 229 tests green, clippy clean, `validate --workflow` green. The change is additive, backward-compatible, and its join tests exercise the real over-strip path rather than passing vacuously.
