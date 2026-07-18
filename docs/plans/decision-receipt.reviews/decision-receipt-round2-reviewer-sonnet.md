# decision-receipt round-2 review (sonnet)

Branch `impl/decision-receipt`, tip `efe90c6`. Read-only: this file is the only artifact written. Reviewing adversarially against the round-1 triage verdicts.

All 186 tests pass in the worktree.

---

## Confirmations

**V2 (QUEUE_FOLD_PREFIX duplication): CONFIRMED FIXED.**
`src/plan.rs:87` now declares `pub(crate) const QUEUE_FOLD_PREFIX`. `src/workflow.rs:37` imports it via the `use` block. There is no second `const QUEUE_FOLD_PREFIX` definition in `workflow.rs`. The exposure is minimal: `pub(crate)` makes the constant visible within the crate only, which is the narrowest scope that permits the reuse. No behavioral change.

**V4 (non-array `options` test): CONFIRMED FIXED.**
`src/metrics.rs` contains the test `a_decision_with_a_non_array_options_is_reported` (introduced in this diff, at the batch of decision tests). It feeds `"options":"A,B"` to `validate_log` and asserts the error `"field \`options\` has wrong type (expected array)"`. The test name matches the project's naming convention; it passes. The `require_options` branch at `src/metrics.rs:282-284` is now covered.

**V5 (recommendation docs clarification): CONFIRMED FIXED.**
`pack/instrument.md` now reads: `recommendation` is "the orchestrator's recommended option; any string, and unlike `chosen` it is NOT required to be a member of `options`, since only the human's actual choice needs to be one of the presented options". The prose is accurate and matches `src/metrics.rs:370` (`require_str(obj, "recommendation")?`), which places no membership constraint on `recommendation`. No code constraint was added. The same text appears in `AGENTS.md` and `.agents/AGENTS.reference.md`.

**V1 (baseline replaces derived-min boundary): CONFIRMED FIXED.**
`w4_problems` in `src/workflow.rs:131-162` reads the cutoff from `baselines.last().map(|baseline| baseline.questions_through)` (an independent declared value from a `type:"baseline"` record), not from the receipt set. A missing receipt cannot alter the cutoff. The no-baseline branch (`cutoff` is `None`) requires a receipt for every decided item, which is the opposite of the old "no receipts means no requirements" behavior. The five W4 tests all pass and collectively cover: at/below cutoff exempt, strictly above cutoff flagged, above cutoff with receipt passes, no-baseline requires all, no-baseline passes with receipt.

**Drift guard extension: CONFIRMED.**
The `instrument_prose_documents_every_accepted_schema_value` test at `src/metrics.rs:1016-1017` now includes `"decision"` and `"baseline"` in its type check. The field check at `src/metrics.rs:1048-1052` now includes `q_id`, `options`, `recommendation`, `chosen`, and `questions_through`. `pack/instrument.md` contains all five as backtick-wrapped names, so the test passes. A future deletion of any of these names from the prose would cause the test to fail.

**AGENTS.md and .agents/AGENTS.reference.md regeneration: CONFIRMED.**
The diff shows both files received exactly the same two bullet points that `pack/instrument.md` received (the `type:"decision"` and `type:"baseline"` entries). The text is identical across all three files. The `instrument_off_omits_the_block_and_on_includes_it` and `generated_agents_has_principles_and_no_placeholder` integration tests pass, confirming the scaffold regeneration path is intact.

**ASCII-clean: CONFIRMED.**
No non-ASCII characters found in any changed file (`src/metrics.rs`, `src/workflow.rs`, `src/plan.rs`, `pack/instrument.md`, `CHANGELOG.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`, `docs/metrics/workflow.jsonl`).

---

## New findings

### F-1 (low): CHANGELOG entry for W4 describes the invalidated derived-min design, not the implemented declared-baseline design

**File:** `CHANGELOG.md:11`

The CHANGELOG entry added in this branch reads (excerpt): "A best-effort `parse_decisions` projection feeds W4, which requires a matching receipt for every `decided -> folded into <slug>` Open-Questions item at or after a derived boundary (the lowest `q_id` index among recorded receipts), so the first receipt written establishes the cutoff and the historical decided items that predate the mechanism are not flagged; with no receipts W4 requires nothing."

This text describes the V1-invalidated design, not the implementation on this branch. Three specifics are wrong against the actual code:

1. "a derived boundary (the lowest `q_id` index among recorded receipts)" - the boundary is NOT derived from receipts. It is read from an independent `type:"baseline"` record's `questions_through` field (`src/workflow.rs:133`). A derived boundary is exactly what V1 found to be circular.
2. "so the first receipt written establishes the cutoff" - a receipt does not establish any cutoff. The `type:"baseline"` record establishes the cutoff.
3. "with no receipts W4 requires nothing" - without a declared baseline, `w4_problems` requires a receipt for EVERY decided item, regardless of whether any receipts exist (`src/workflow.rs:149`). This is the opposite of "requires nothing," and the no-baseline test `w4_with_no_baseline_requires_a_receipt_for_every_decided_item` confirms it.

The `type:"baseline"` record is not mentioned in the CHANGELOG entry at all, so a reader relying on the CHANGELOG to understand the feature would have a fully incorrect model of how W4 works and what data a migrating project must write.

Severity: low (documentation-only; code and tests are correct). Fix: rewrite the W4 description in the CHANGELOG to match the implementation: declared baseline sets the cutoff, without a baseline every decided item requires a receipt, and describe the `type:"baseline"` record as the new primitive the entry adds alongside `type:"decision"`.

---

### F-2 (observation, not a valid finding): inaccurate test comment in `w4_with_no_baseline_requires_a_receipt_for_every_decided_item`

**File:** `src/workflow.rs`, the W4 test for no-baseline behavior.

The test comment says Q-44 is "flagged (the derived-min design would have exempted it)." In the specific scenario the test exercises (Q-1 HAS a receipt, Q-44 does NOT), the derived-min design would NOT have exempted Q-44: the derived min of {1} is 1, and the old exemption was for items with index `< 1`, so Q-44 (index 44) would be required under the old design too. The "would have exempted it" claim is only true in a scenario where NO receipts exist at all.

This is a comment accuracy issue, not a code defect; the test name is correct, the behavior tested is correct, and the test passes. Raising it here as an observation rather than a finding because it does not meet the "factually wrong in a way that misleads about the code's behavior" bar for low findings. No action required.

---

### F-3 (observation, not a valid finding): `question_id_index` (`metrics.rs`) and `question_index` (`workflow.rs`) duplicate the same two-line algorithm

Both are private functions implementing `id.strip_prefix("Q-").and_then(|digits| digits.parse::<u64>().ok())`. Unlike V2, neither represents a real drift risk: `question_id_index` drives schema validation and the baseline projection; `question_index` drives W4's boundary check; tests cover both independently. The algorithm is trivially correct and its divergence would be caught by existing tests. Raising as an observation only; no action required unless the planner considers it a maintainability gap worth a follow-up refactor.

---

## Roll-up

One new valid finding (F-1, low): the CHANGELOG entry for W4 describes the old derived-min design. All round-1 fixes (V1, V2, V4, V5) confirmed correctly landed. All 186 tests pass. ASCII-clean.
