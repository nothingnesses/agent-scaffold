# Round 2 review: waiver-model (reviewer-sonnet)

Summary: NEW VALID FINDINGS (2, both low). The round-1 blockers are fixed and the gates pass. Two new low-severity issues were found that were not in round 1.

## Gates run

- `cargo test`: 211 passed, 0 failed.
- `cargo clippy`: clean.
- `cargo run -- validate --plan docs/plans/agent-scaffold.md --workflow`: 105 records valid; 50 steps, 44 open-questions items, valid; workflow invariants hold.

---

## R2-S1 (low) - Stale forward-pointing sentence in three docs, S2 remnant

File: `pack/instrument.md` (also `AGENTS.md` and `.agents/AGENTS.reference.md`)

The final sentence of the `type:"baseline"` entry in all three docs reads: "The record is extensible: a later mechanism (for example a step/round convergence cutoff) may add further cutoff fields to the same `baseline` type."

This sentence is now stale. The `waiver-model` step was that later mechanism, and it deliberately chose per-unit `type:"waiver"` records rather than adding a cutoff field to `baseline`. The S2 follow-up removed the matching stale sentence from `src/metrics.rs` (lines 464-467 and 671-675 now say "`baseline` serves W4 only; W3's historical exemption is carried by per-unit `type:"waiver"` records, not a cutoff on this type") but the same forward-pointing text was not updated in the three docs files. In each file, this sentence now sits directly above the newly added `type:"waiver"` entry that explains the alternative the text was predicting, making the inconsistency visible to any reader of the docs.

The sentence does not describe an incorrect implementation (the code is correct), but it misleads a reader: it says the design may add a W3 cutoff to `baseline`, and the very next bullet shows a different design was chosen instead. A reader following the pointer will look for a cutoff that does not exist.

Direction: remove the final sentence from the `baseline` entry in all three files, or replace it with a note that W3's exemption uses waivers, not a `baseline` cutoff (mirroring the corrected wording already in `src/metrics.rs`).

---

## R2-S2 (low/info) - W5 step-unit escalation join branch untested

File: `src/workflow.rs`, line 404

The W5 record-backed escalation join has two branches based on `waiver.unit` (workflow.rs lines 401-405):
- `WaiverUnit::Increment`: exact match `waiver.increment == escalation.task` - tested by multiple tests.
- `WaiverUnit::Step`: `leading_slug(&escalation.task) == waiver.step` - not tested.

The step-unit branch is reachable when a step-unit waiver carries `reason:"accepted-at-escalation"` and `evidence_tier:"record-backed"`. The schema (`check_record`) permits this combination, and `parse_waivers` projects it. The docs (`pack/instrument.md`, `AGENTS.md`) explicitly describe the step-unit scoping rule ("the escalation's leading slug equals the waived step"), so the feature is documented and implemented, but no test exercises it.

The gap is low priority because no migration record uses this path (all step-unit waivers in the repo are `predates-logging`/`review-skipped`/`self-declared`), and the code is correct as written. But it is a branch with its own comparison logic (`leading_slug` vs exact match) that could silently diverge from the docs on a future refactor.

Direction: add a test exercising W5 on a step-unit `accepted-at-escalation`/`record-backed` waiver: one that passes (escalation.task's leading slug equals the step) and ideally one that fails (mismatch).

---

## Verified clean

1. Migration honesty (S1 relabel). The three b2 waivers (`convergence-accounting`, `pack-rebuild-tracking`, `user-prompts-dir`) are now `reason:"review-skipped"`. Each has exactly one `outcome:"new_valid"` round record with `consecutive_clean:0` in `docs/metrics/workflow.jsonl`, confirming one review round ran but the convergence streak was never reached. `review-skipped` is honest. The 11 step waivers (`core-assets`, `file-dropper`, `idempotency-safety`, `selection-ui`, `mode-enum`, `tag-selection`, `available-filter`, `pack-manifest`, `external-packs`, `pack-owned-principles`, `init-vcs`) remain `predates-logging`; all 11 have zero round records in `workflow.jsonl`. The `optional-modules` record-backed waiver is intact: `evidence:"optional-modules-inc2cii"` matches the escalation at `task:"optional-modules-inc2cii"` with `human_decision:"decision"`, and `increment:"optional-modules-inc2cii"` satisfies the W5 unit-scope check exactly.

2. O1 fix (unit-scoped W5 join). workflow.rs lines 400-405 match the escalation against `escalation.task == evidence` AND scope it to the waived unit. For increment waivers the unit check is `waiver.increment.as_deref() == Some(escalation.task.as_str())`. The test `w5_flags_a_record_backed_waiver_citing_an_unrelated_escalation` confirms that citing a real `decision` escalation with a different task fails; `w5_passes_a_record_backed_waiver_with_a_matching_escalation` confirms the correct citation passes.

3. O3 fix (increment-step cross-check). W3 predicate (workflow.rs lines 309-313) now includes `&& waiver.step == step.slug` alongside the increment identity check. W5 checks `leading_slug(increment) != waiver.step` at lines 374-383. Both are tested: `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment` (W5 check) and `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment` (W3 check).

4. O4 fix (non-empty guards). `check_record` escalation arm requires `task` non-empty (metrics.rs line 417). `check_record` waiver arm requires `evidence` non-empty for `record-backed` tier (line 526). `parse_waivers` filters empty evidence (line 802-805). `parse_escalations` filters empty task (line 856). All four guards confirmed present.

5. S2 fix in source. The two stale "baseline W3 cutoff" comments in `src/metrics.rs` (previously at the `baseline` arm of `check_record` and the `Baseline` struct doc) are replaced with correct descriptions saying baseline serves W4 only and W3 uses waivers.

6. S3 test (resume-decision gap). `w5_flags_a_record_backed_waiver_whose_escalation_resumed_not_decided` exercises a matching-task escalation with `human_decision:"resume"` and confirms it does not satisfy the join.

7. S4 test (bare-slug shape). `a_bare_slug_increment_waiver_exempts_a_short_streak` pins the case where `step == increment == bare-step` (no `-inc` suffix), confirming `leading_slug("bare-step") == "bare-step"` and W3 exempts it.

8. O2 doc fix. The `WaiverUnit` enum doc comment (metrics.rs lines 156-159) now says an increment waiver may be self-declared OR record-backed and that the two tiers exist to prevent laundering, not to forbid self-declaration. No longer oversells enforcement.

9. Roadmap changes. Exactly one diff hunk (the Roadmap table), 15 rows changed: 14 from `grandfathered` to `complete`, 1 from `in progress` (`optional-modules`) to `complete`. These 15 correspond 1:1 to the 15 waiver records added to `workflow.jsonl`. The `waiver-model` row stays `next`. Status line, Step Details, and Open Questions are untouched.

10. TEMPLATE.md and pack/plan-template.md. `trivial` and `grandfathered` removed from the status enumeration. The instruction correctly points to `type:"waiver"` for exemptions. CHANGELOG marks both retired.

11. No `trivial` or `grandfathered` as live status anywhere. The instances in AGENTS.md and .agents/AGENTS.reference.md are correct English uses ("trivial change", "even for a trivial or low-risk review round") or CHANGELOG retirement notices. `src/plan.rs` removed both from `ROADMAP_STATUSES` and the test now asserts they are rejected.

12. ASCII cleanliness. No non-ASCII characters in any added or modified line across all 11 files in the commit.

13. File count. Exactly 11 files changed, matching the brief.
