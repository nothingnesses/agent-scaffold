# Inc 2 review: structured step/increment id on JSONL records

Reviewer: sonnet (independent) Diff: `git diff a780541 e395d44` Date: 2026-07-19

## Verification results

Commands run on the review commit via a temporary worktree at `/tmp/inc2-review` (detached HEAD at `e395d44`):

- `cargo test --all-targets`: 225 + 1 + 3 = 229 tests, all pass. Matches the implementer's stated count.
- `cargo clippy --all-targets -- -D warnings`: clean, no warnings.
- `cargo run -- validate --metrics docs/metrics/workflow.jsonl --plan docs/plans/agent-scaffold.md --workflow`: green (116 records valid, 51 steps, 46 open-questions items, workflow invariants hold).

All three pass. The implementer's claims are correct.

## Ruled out

- **Scope creep (Principle 8)**: No. The change is exactly scoped to Inc 2: optional `step`/`increment` on `round` and `escalation`, fallback to `leading_slug`/`task` shim for records without them. Waivers correctly gain no field per Q-46. W4 is untouched. No enforcement-semantic changes beyond what the plan described.
- **Backward compatibility**: Old records without the new fields still parse, validate, and join via `leading_slug`/`task`. Confirmed by `w3_a_pre_migration_round_still_joins_its_step_via_leading_slug` and `w5_without_the_structured_step_the_escalation_over_strips_and_is_missed` (the companion test showing the baseline behavior is preserved).
- **Naming consistency**: `step`/`increment` on `round`/`escalation` is consistent with the same field names on `waiver`. The plan specified these names.
- **Empty-string defense**: `require_structured_ids` rejects present-but-empty fields at validation time; `parse_rounds`/`parse_escalations` additionally `.filter(!s.is_empty())` so a blank field becomes `None` at parse time. Belt-and-suspenders, both correct.
- **W3/W5 logic**: The helper functions `round_step_slug`, `round_increment_id`, `escalation_step_slug`, `escalation_increment_id` correctly prefer the structured id and fall back to `leading_slug(task)` / `task`. The T3/SE-10/B6 risk is retired for records that carry the structured id. Tested end-to-end in `w3_a_round_carrying_a_structured_step_joins_without_the_lexical_strip` and `w5_a_record_backed_waiver_joins_via_the_escalations_structured_step`.
- **`round_log_consistency_problems` grouping**: Now keys on `round_increment_id` instead of raw `task`. Correct; tested implicitly via the existing consistency tests (pre-migration records are unchanged, structured-id records project the right key).
- **Import reordering in `workflow.rs`**: `question_id_index` moved to alphabetical position, `QUEUE_FOLD_PREFIX` similarly. Purely cosmetic, no behavioral change.

## Findings

### S1 - medium

**File**: `AGENTS.md` and `.agents/AGENTS.reference.md` at commit `e395d44`

**Evidence**: `git show e395d44:AGENTS.md | grep "structured skeleton"` returns only the `waiver` bullet (which predates Inc 2); the `round` bullet ends at "A record written without the optional `reviewers` field still validates." with no mention of the new `step`/`increment` fields. The `escalation` bullet reads only "`artifact` and `human_decision`..." with no mention of the optional structured ids. `git show e395d44:.agents/AGENTS.reference.md` shows identical stale content. Meanwhile `git show e395d44:pack/instrument.md` has the updated text for both bullets.

**Defect**: `pack/instrument.md` is the source template for the `{{instrument}}` slot. It was correctly updated in this commit. But the two deployed copies of `AGENTS.md` in this repo - `AGENTS.md` (ownership=working) and `.agents/AGENTS.reference.md` (ownership=reference) - were not regenerated. The reference copy is specifically designated as always-fresh (it has `ownership = "reference"` in `pack.toml` and lives under `.agents/`); leaving it stale breaks the ownership contract for that copy. Any agent working in this repo and reading `AGENTS.md` will not know the `round` or `escalation` records accept the new `step`/`increment` fields, so the orchestrator will not write them, defeating the T3 retirement in practice for this repo's own metrics log.

This violates Principle 16 (one source of truth: the template and deployed docs now disagree) and Principle 19 (the documented JSONL schema is wrong for writers that read it). The `round` bullet gap matters more than the `escalation` gap because orchestrators write `round` records every review round.

**Direction**: Re-run `agent-scaffold --instrument` on this repo to regenerate at least `.agents/AGENTS.reference.md` (or pass `--force` to also refresh `AGENTS.md`). The existing tests in `tests/scaffold_*` and `src/manifest.rs` do not catch this because they do not verify the deployed docs match the pack source.

---

### S2 - low

**File**: `pack/instrument.md` (line 11, the `waiver` bullet)

**Evidence**: The `waiver` bullet at `e395d44` says W5's scope check uses "the escalation's `task` equals the waived increment, or its leading slug equals the waived step". The code at `e395d44` (`src/workflow.rs`, `w5_problems`) now does:

```
WaiverUnit::Increment =>
    waiver.increment.as_deref() == Some(escalation_increment_id(escalation)),
WaiverUnit::Step => escalation_step_slug(escalation) == waiver.step,
```

These prefer the escalation's structured `increment`/`step` id (Inc 2) and fall back to `task`/`leading_slug` only when absent. The documentation describes only the fallback path.

**Defect**: The `waiver` documentation says the scope check keys on `escalation.task` (and `leading_slug(escalation.task)` for step-unit). After Inc 2, it keys on `escalation_increment_id` / `escalation_step_slug`, which choose the structured id first. A reader following the docs to construct or debug a record-backed waiver with a new Inc 2 escalation would apply the wrong mental model. If the escalation carries structured ids that differ from what `leading_slug(task)` returns (the exact T3 case), the docs say it would fail W5 when it actually passes. The bug is benign in direction (code is more permissive than docs claim) but is still a Principle 19 gap. The `pack/instrument.md` escalation bullet was updated in this commit; the waiver bullet was not.

**Direction**: Extend the `waiver` bullet's description of W5's scope check to say: "the escalation's structured `increment` id (Inc 2; or `task` when absent) equals the waived increment, or its structured `step` slug (Inc 2; or `leading_slug(task)` when absent) equals the waived step". A single cross-reference to the `escalation` bullet's description of the structured ids would also work.

---

### S3 - low

**File**: `src/metrics.rs` (tests block, around line 1490 in the review commit)

**Evidence**: The Inc 2 tests added are:

- `a_round_with_structured_step_and_increment_ids_is_accepted` - both fields present
- `a_round_with_an_empty_structured_step_is_reported` - empty `step`, no `increment`
- `an_escalation_with_structured_ids_is_accepted` - both fields present
- `an_escalation_with_an_empty_structured_increment_is_reported` - empty `increment`, no `step`

No test has a record carrying exactly one of the two fields: `step` present and `increment` absent, or `increment` present and `step` absent.

**Defect**: The plan specifies "When either is present it must be a non-empty string" - each field is independently optional. The implementation handles this correctly (each field is parsed and validated independently in `require_structured_ids` and the `parse_rounds`/`parse_escalations` projections). But no test asserts that validation ACCEPTS a record with only one field. A regression that accidentally couples the two checks (requiring both when one is present) would go undetected. This is a Principle 11 gap (tests must actually exercise the code they claim to).

**Direction**: Add one validation-acceptance test with a `round` record carrying `step` but no `increment`, and one with `increment` but no `step`. One or two lines each.

---

## Summary

Three findings: one medium (S1, the deployed `AGENTS.md`/reference copy not regenerated after template update, so orchestrators writing to this repo's own log won't know to include the new fields), two lows (S2, the waiver documentation describes the pre-Inc-2 scope-check algorithm without acknowledging the new structured-id preference; S3, no test for a record carrying only one of the two optional fields). No critical or high findings. The core logic is correct, backward-compatible, well-scoped, and the test suite on the review commit passes cleanly at 229 tests with no clippy warnings.
