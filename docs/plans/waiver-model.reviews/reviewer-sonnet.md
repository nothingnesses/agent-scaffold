Four findings: one high (migration honesty), two medium (stale comments, test gap), one low (test shape gap). No critical defects. ASCII clean, AGENTS.md in sync, Roadmap edits confined to status cells, 15 waivers match 15 Roadmap row changes, optional-modules escalation record confirmed real.

---

## S1 - high

**File:** `docs/metrics/workflow.jsonl` lines 102-104 (waiver records for `convergence-accounting`, `pack-rebuild-tracking`, `user-prompts-dir`)

**Description.** All three increment-level waivers carry `reason:"predates-logging"`, but each of these steps has a round record in the log. The log at line 3 shows `convergence-accounting` with `outcome:"new_valid"` and `consecutive_clean:0`; line 7 shows `pack-rebuild-tracking` the same; line 16 shows `user-prompts-dir` the same. The literal meaning of `predates-logging` is "no records; the step was done before the logging mechanism existed." The step details (agent-scaffold.md around the `workflow-invariants` step outcome) explicitly distinguish "b1" zero-record steps (which truly predate logging) from "b2 short-streak steps" that "converged informally before per-round logging was disciplined." These three are b2, not b1. A future reader who sees `reason:"predates-logging"` for `convergence-accounting` can immediately look up `convergence-accounting` in `workflow.jsonl` and find a record, directly contradicting the stated reason.

The more accurate reason for a review that started (one `new_valid` round found problems) but whose completion rounds were never run is `review-skipped`. The convergence rounds were skipped, even if the first round was not. Alternatively, if neither existing label fits cleanly, the correct response is to note the mismatch - using `predates-logging` for steps with records conflates the b1 and b2 categories that the design explicitly kept separate.

W5 does not catch this because W5 only enforces the reason-to-tier pairing (`predates-logging` must be `self-declared`, which is correct here); it does not verify that a `predates-logging` waiver's step actually has no records. So the mislabeling silently passes all checks.

**Direction.** Change the three increment waivers at lines 102-104 to `reason:"review-skipped"`. Both `predates-logging` and `review-skipped` pair with `self-declared` so the W5 pairing check is satisfied either way. `review-skipped` is the more accurate label for a step whose completion rounds were never run: the round loop started, found problems, and stopped, so the clean-round review cycle was effectively skipped. If there is a principled argument that these b2 steps should use `predates-logging` (because the convergence discipline did not yet exist at the time), document that interpretation explicitly in the waiver or in this step's narrative, rather than leaving a factual mismatch implicit.

---

## S2 - medium

**File:** `src/metrics.rs` lines 455-459 and line 664

**Description.** Two comments in the pre-existing `baseline` handling predict that the `waiver-model` step would add a W3 cutoff field to the `baseline` record type.

Lines 455-459 (inside `check_record`'s `baseline` arm):
```
// The record is deliberately open to more cutoff fields: a future
// `waiver-model` step adds a steps/rounds cutoff for W3's historical
// exemption to this same `baseline` type. Only the field this step consumes
// is constrained here, and unknown extra fields stay permitted (see the
// doc comment above), so that extension needs no change to this arm.
```

Line 664 (in the `Baseline` struct's doc comment):
```
/// record may carry more (a future `waiver-model` W3 cutoff on the same type), but
```

This implementation is the `waiver-model` step, and it chose per-unit `type:"waiver"` records instead of a W3 cutoff on `baseline`. The comments now describe a design path that was not taken. A reader will see "the `waiver-model` step adds a cutoff" and then check the actual log - which has no such cutoff - and the plan - which chose per-step waivers as the migration path. The comments are stale.

**Direction.** Remove or rewrite both comments to reflect the actual design chosen. The correct framing is that `baseline` serves W4 only (the `questions_through` cutoff); W3's historical exemption is covered by per-unit `type:"waiver"` records, not a cutoff field on `baseline`. A simple clarification that the record is open to future extension (without the now-false prediction) is sufficient.

---

## S3 - medium

**File:** `src/workflow.rs` (W5 tests section, around line 876 onward)

**Description.** The test suite for W5 covers the "no matching escalation" case (empty escalation slice) but not the case where an escalation record EXISTS for the evidence task but has `human_decision:"resume"` rather than `"decision"`. The W5 implementation is:

```rust
let backed = escalations.iter().any(|escalation| {
    escalation.task == evidence
        && escalation.human_decision == HumanDecision::Decision
});
```

This correctly requires `HumanDecision::Decision`. But the only failing test (`w5_flags_a_record_backed_waiver_with_no_matching_escalation`) passes an empty escalation slice. There is no test confirming that an escalation with the matching `task` but `human_decision:"resume"` is rejected. The `escalation_line` helper always produces a `"decision"` outcome, so there is no path through the tests that exercises the `resume`/wrong-decision branch of the join.

**Direction.** Add a test that provides an escalation record with the matching task but `human_decision:"resume"` (or another non-decision outcome), and asserts that W5 still flags the waiver. The helper `escalation_line` can stay as-is; the new test constructs its own raw log line with `human_decision:"resume"` or adds a second helper for that case.

---

## S4 - low

**File:** `src/workflow.rs` (W3 increment-waiver tests)

**Description.** The migration uses increment-level waivers whose `increment` token equals the bare step slug with no `-inc` suffix (for example `increment:"convergence-accounting"` matching `task:"convergence-accounting"` in the log). This is technically correct: W3 groups rounds by full task value, and a task with no `-inc` suffix forms one group named identically to the step. The increment waiver's `increment` field matches it. However, none of the W3 tests exercises this exact shape. The test `a_short_streak_increment_with_a_covering_increment_waiver_passes` uses `stall-incA` style names throughout. The bare-slug shape is the actual migration data for three of the fifteen waivers, and a regression in the `leading_slug` function or the increment-grouping logic would silently break it.

**Direction.** Add one test in `src/workflow.rs` that uses a round with `task:"bare-step"` (no suffix) and an increment waiver with `increment:"bare-step"`, asserting the shortfall is exempted. This pins the bare-slug behavior explicitly.

---

## What the review found to be correct

- **11 zero-record step waivers.** Each of `core-assets`, `file-dropper`, `idempotency-safety`, `selection-ui`, `mode-enum`, `tag-selection`, `available-filter`, `pack-manifest`, `external-packs`, `pack-owned-principles`, and `init-vcs` has no round records in `workflow.jsonl`. `reason:"predates-logging"` is accurate for all eleven.

- **optional-modules-inc2cii waiver.** The escalation record exists at `workflow.jsonl` line 82 with `task:"optional-modules-inc2cii"` and `human_decision:"decision"`. The other optional-modules increments (inc1, inc2a, inc2b, inc2ci, inc3) all converged. The waiver is neither too broad nor too narrow.

- **W3 exemption scoping.** A step-level waiver does not exempt a short-streak increment (tested by `a_step_waiver_does_not_exempt_a_short_streak_increment`). A risk_class inconsistency is not suppressed by an increment waiver (tested). The no-records-no-waiver catch is tested.

- **W5 correctness.** The reason-to-tier pairing enforcement covers all three forbidden pairings and all three valid ones. The dangling-step check is tested. The evidence-join check covers the missing-escalation case.

- **Roadmap edit scope.** The diff to `docs/plans/agent-scaffold.md` changes only the Status column cells. The Status line, Step Details, and Open Questions are untouched.

- **Pack/template consistency.** `pack/plan-template.md` and `docs/plans/TEMPLATE.md` both remove `trivial`/`grandfathered` from the status list and add the waiver-based explanation. `AGENTS.md` and `.agents/AGENTS.reference.md` are byte-identical.

- **CHANGELOG accuracy.** The new entry describes the waiver record and W5 check correctly. The retired `trivial`/`grandfathered` entry is correctly amended to note retirement in the same unreleased cycle.

- **ASCII cleanliness.** No non-ASCII characters in any touched file.

- **Stale trivial/grandfathered references.** The remaining occurrences of these terms in `src/plan.rs` (retirement comment and test), `src/workflow.rs` (test comment), `src/metrics.rs` (intake `Classification::Trivial`, a different concept), `pack/AGENTS.md` (trivial in the intake/convergence-counting sense), and `docs/plans/` exploration documents are all appropriate: either in "RETIRED" context, in tests verifying rejection, or as the unrelated intake classification. None are stale references to the retired Roadmap statuses in an active code path.
