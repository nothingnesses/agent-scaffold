# Round 3 review: waiver-model (reviewer-sonnet)

CLEAN. No new valid defects. Both round-2 findings are closed. All gates pass.

## Gates run

- `cargo test`: 213 passed, 0 failed.
- `cargo clippy`: clean, no warnings.
- `cargo run -- validate --metrics docs/metrics/workflow.jsonl --plan docs/plans/agent-scaffold.md --workflow`: 105 records valid; 50 steps, 44 open-questions items, valid; workflow invariants hold (rc=0).
- `cargo run -- validate --metrics docs/metrics/workflow.jsonl`: 105 records, valid (rc=0).
- `cargo run -- validate --plan docs/plans/agent-scaffold.md`: 105 records valid; 50 steps, 44 open-questions items, valid (rc=0).

## Round-2 findings verified CLOSED

**R2-S1** (stale `baseline` extensibility sentence). The forward-pointing sentence "The record is extensible: a later mechanism ... may add further cutoff fields to the same `baseline` type" is gone from all three docs files. `pack/instrument.md` line 10, `AGENTS.md` line 138, and `.agents/AGENTS.reference.md` (byte-identical to `AGENTS.md`) all now end the `baseline` entry with "The `baseline` type serves W4 only (the `questions_through` cutoff); W3's historical exemption is carried by per-unit `type: "waiver"` records, not by a cutoff on this type." No stale forward pointer remains. CLOSED.

**R2-S2** (W5 step-unit join branch untested). Two tests added in commit f24810c:
- `w5_passes_a_step_unit_record_backed_waiver_joined_by_leading_slug` (workflow.rs line 1070): a step-unit record-backed waiver whose evidence names a `decision` escalation whose `leading_slug(task)` equals the waived step passes W5.
- `w5_flags_a_step_unit_record_backed_waiver_whose_escalation_names_a_different_step` (workflow.rs line 1083): the same shape but the escalation's leading slug names a different step is flagged.
Both pass. CLOSED.

## Migration honesty - re-derived from raw data

**11 predates-logging step waivers.** Verified against `docs/metrics/workflow.jsonl`: the 11 steps (`core-assets`, `file-dropper`, `idempotency-safety`, `selection-ui`, `mode-enum`, `tag-selection`, `available-filter`, `pack-manifest`, `external-packs`, `pack-owned-principles`, `init-vcs`) have zero round records with a matching leading slug. The predates-logging claim is accurate and `self-declared` is the correct tier.

**3 review-skipped increment waivers.** Verified:
- `convergence-accounting`: one round record, `outcome:new_valid`, `consecutive_clean:0`, `risk_class:low_risk`. Peak 0, needs 1. The review started but never converged. `review-skipped`/`self-declared` is honest.
- `pack-rebuild-tracking`: same shape (one `new_valid` round, peak 0, needs 1). Honest.
- `user-prompts-dir`: same shape. Honest.
Each waiver uses `unit:increment` with the increment token equal to the bare step slug (no `-inc` suffix). W5's leading-slug ownership check (`leading_slug("convergence-accounting") == "convergence-accounting"`) holds.

**1 record-backed accepted-at-escalation increment waiver.** `optional-modules-inc2cii`: five round records, four `new_valid` and one `clean` (peak=1, needs 2 for risky). One escalation at `task:"optional-modules-inc2cii"`, `human_decision:"decision"`. The waiver carries `step:"optional-modules"`, `increment:"optional-modules-inc2cii"`, `evidence:"optional-modules-inc2cii"`. W5's join: `escalation.task == evidence` ("optional-modules-inc2cii" == "optional-modules-inc2cii") and `waiver.increment == escalation.task` ("optional-modules-inc2cii" == "optional-modules-inc2cii") both hold. `accepted-at-escalation`/`record-backed` is the correct type and tier. The escalation is scoped to the waived increment.

**No missing waivers.** Every `complete` step either has converging round records or a covering waiver. The orphan tasks with shortfalls (`convergence-accounting`-the-log-entry, `plan-maintenance`, `workflow-hardening`) have no Roadmap row and W3 does not check them. No 16th waiver is needed.

**Waiver count vs flipped rows.** 15 waiver records in the log correspond exactly to 15 flipped Roadmap rows: 14 from `grandfathered` to `complete`, 1 from `in progress` (`optional-modules`) to `complete`.

## Consistency

**AGENTS.md and .agents/AGENTS.reference.md**: byte-identical (md5 `afe2209beaff2242cdbaf30fbb1588f9` both). Both accurately describe the waiver schema, the W3 convergence-OR-waiver rule, and the W5 orthogonal integrity check.

**docs/plans/TEMPLATE.md and pack/plan-template.md**: byte-identical (md5 `4e1f176c15d6ea91c3cde5c6c709d57e` both). Both remove `trivial` and `grandfathered` from the status enumeration and correctly explain the waiver exemption.

**No stale `trivial`/`grandfathered` as live statuses.** The instances of "trivial" in AGENTS.md are correct workflow English ("trivial change", "trivial or low-risk artifact", intake classification `trivial`/`non_trivial`), not Roadmap statuses. `grandfathered` does not appear in any doc file except as a retired name in the waiver description and CHANGELOG.

**No stale baseline-W3 references.** `src/metrics.rs` (the baseline arm of `check_record` and the `parse_baseline` doc), `pack/instrument.md`, and `AGENTS.md` all now say the baseline type serves W4 only. No "W3 cutoff" prediction survives.

**`src/plan.rs` ROADMAP_STATUSES.** `trivial` and `grandfathered` removed; the validator rejects both and the test `the_retired_review_exempt_statuses_are_rejected_but_skipped_stays_accepted` asserts both are rejected and `skipped` is still accepted.

**CHANGELOG.md.** The added entry accurately names all new artifacts (`src/metrics.rs`, `src/workflow.rs`, `pack/instrument.md`), the retirement of `trivial`/`grandfathered`, and that `skipped` stays. No overclaim or underclaim.

## Tests

All major branches of the new logic are exercised:

- Schema (waiver): bad `unit`, bad `reason`, bad `evidence_tier`, empty `step`, `increment` forbidden on step-unit, `increment` required on increment-unit, `evidence` forbidden on self-declared, `evidence` required on record-backed. All tested in `src/metrics.rs` tests.
- `parse_waivers` projection: two well-formed waivers projected in file order; bad-unit, broken presence-rule, and non-waiver records all dropped. `parse_waivers_projects_well_formed_records_and_drops_malformed_ones` covers this.
- `parse_escalations` projection: `parse_escalations_projects_well_formed_records_and_skips_others`.
- W3 step-unit waiver exemption: `a_complete_step_with_no_records_but_a_covering_step_waiver_passes`. A waiver for a different step does not exempt (`a_complete_step_with_no_records_and_no_covering_waiver_is_caught`).
- W3 increment-unit waiver exemption: `a_short_streak_increment_with_a_covering_increment_waiver_passes`.
- W3 bare-slug increment: `a_bare_slug_increment_waiver_exempts_a_short_streak` (S4 shape).
- W3 mis-scoped increment waiver: `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment` (O3 W3 half).
- W5 nonexistent step: `w5_flags_a_waiver_naming_a_nonexistent_step`.
- W5 increment ownership: `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment` (O3 W5 half).
- W5 record-backed join (increment): `w5_passes_a_record_backed_waiver_with_a_matching_escalation`, `w5_flags_a_record_backed_waiver_with_no_matching_escalation`, `w5_flags_a_record_backed_waiver_citing_an_unrelated_escalation` (O1).
- W5 record-backed join (step): `w5_passes_a_step_unit_record_backed_waiver_joined_by_leading_slug`, `w5_flags_a_step_unit_record_backed_waiver_whose_escalation_names_a_different_step` (R2-S2).
- W5 resume-vs-decision: `w5_flags_a_record_backed_waiver_whose_escalation_resumed_not_decided` (S3).
- W5 reason/tier pairings: `w5_flags_each_inconsistent_reason_tier_pairing`, `w5_accepts_the_three_valid_reason_tier_pairings`.
- End-to-end (W3+W5 together): `check_workflow_passes_the_optional_modules_migration_shape`.
- Retirement in plan.rs: `the_retired_review_exempt_statuses_are_rejected_but_skipped_stays_accepted`.

One structural gap (non-defect): there is no explicit test that a zero-records step is NOT exempted by an increment-unit waiver. The code makes this impossible by control flow (the no-records path `continue`s before the increment waiver check) so it cannot silently become a bug without restructuring the function. Not a defect.

## Roadmap

Only status-cell edits. The table header column-width normalisation (trailing spaces adjusted) is the only non-cell change. The `waiver-model` row stays `next`. Status line, Step Details, and Open Questions are untouched. Exactly 15 rows changed.

## ASCII cleanliness

No non-ASCII characters in any added or modified line across all 11 changed files.

## Files touched

The commit touches exactly the 11 intended files: `src/metrics.rs`, `src/workflow.rs`, `src/plan.rs`, `pack/instrument.md`, `pack/plan-template.md`, `docs/plans/TEMPLATE.md`, `docs/plans/agent-scaffold.md`, `docs/metrics/workflow.jsonl`, `CHANGELOG.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`. No unintended files modified.
