# Round-3 Review: decision-receipt (impl/decision-receipt, tip 0c6672a)

Reviewer: sonnet (round-3 confirming pass)
Date: 2026-07-18

## Verdict: CLEAN

Both round-2 fixes landed correctly. No new findings.

---

## Confirmed fixes

### L1: Q-parser deduplication

`workflow.rs` no longer defines its own `Q-<n>` parser. There is exactly one
`strip_prefix("Q-")` + `parse::<u64>()` call in the codebase, at
`src/metrics.rs:308` inside `pub(crate) fn question_id_index`. `workflow.rs`
imports and calls it at line 135 (`question_id_index(&question.id)`) for the
decided item's index. The cutoff operand (`baselines.last().map(|b|
b.questions_through)`) is a `u64` populated in `parse_baseline` via
`question_id_index` at `src/metrics.rs:583`. Both sides of the `index <=
cutoff` comparison at `workflow.rs:142` therefore go through the same parser;
they cannot drift.

### F-1: CHANGELOG accuracy

The `[Unreleased]` entry in `CHANGELOG.md` accurately describes:
- the `type:"decision"` receipt with `q_id`, `options`, `recommendation`, and
  `chosen` (with `chosen` required to be a member of `options`).
- the `type:"baseline"` `questions_through` cutoff.
- W4 requiring a receipt for every decided item strictly after the declared
  cutoff, and for every decided item when no baseline is declared.
- the cutoff being an independent declared value rather than derived from the
  receipt set, so a forgotten receipt cannot move its own exemption boundary.

No "derived boundary", "lowest q_id", or "with no receipts W4 requires
nothing" phrasing survives. The word "derived" appears once only in the
correct negating context: "not derived from the receipt set."

---

## Holistic checks

- `just test`: 186 unit tests + 4 integration tests, all pass.
- `just clippy`: clean, no warnings.
- ASCII: all changed files clean (no byte > 127).
- `cargo run -- validate --workflow --plan docs/plans/agent-scaffold.md`: exits
  0, reports "workflow invariants hold."
- Default scaffold (no `--instrument`): exits 0, no regression.
- No `#[allow]` introduced in changed files (`src/metrics.rs`, `src/workflow.rs`,
  `src/plan.rs`, `CHANGELOG.md`, docs, pack). Pre-existing `#[allow]` in
  `src/pack.rs` and `src/checks.rs` are unchanged.
- Import ordering in `workflow.rs`: `Baseline`, `Decision`, `Round`,
  `RoundOutcome`, `question_id_index` - correct ASCII sort order (uppercase
  before lowercase).
- No new duplication introduced elsewhere. `QUEUE_FOLD_PREFIX` is now
  `pub(crate)` and imported in `workflow.rs` rather than redeclared.
- `docs/metrics/workflow.jsonl` migration `type:"baseline"` record is present
  and passes `validate_log` (covered by the `a_valid_baseline_record_passes`
  test and the live validate run above).
- `pack/instrument.md`, `AGENTS.md`, and `.agents/AGENTS.reference.md` carry
  identical additions; no divergence between the three copies.

No new findings.
