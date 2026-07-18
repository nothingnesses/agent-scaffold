CLEAN - one low-severity test gap noted; no blocking defect found.

## Findings

### F1 - LOW - src/metrics.rs:415-421 - Untested empty-task validation on escalation

The change adds an empty-task check to the `escalation` arm of `check_record` (lines 415-421):

```rust
if require_str(obj, "task")?.is_empty() {
    return Err("field `task` is empty".to_string());
}
```

Every other new schema constraint added in this change has a dedicated test (empty step, forbidden increment on step-unit, forbidden evidence on self-declared, missing increment on increment-unit, missing evidence on record-backed). This constraint has none. The common top-level `require_str(obj, "task")?` already requires `task` to be a present string, but does not reject an empty string; this per-arm check closes that gap. It is a correct and well-motivated addition (the comment explains: an empty-task escalation can never satisfy the W5 join, so the strict validator should reject it now rather than silently pass a record the projection would silently drop). However, it introduces a behavioural change to escalation validation with no test pinning the change.

Why this is not blocking: `parse_escalations` already filters empty-task escalations at the projection level (line 849: `.filter(|s| !s.is_empty())`), so the W5 join cannot be satisfied by such a record regardless. The existing VALID_LOG fixture and the `parse_escalations_projects_well_formed_records_and_skips_others` test both use non-empty tasks; no test currently would catch a regression that removed the empty-task rejection.

Direction: add a single test along the lines of the existing empty-field tests in the waiver block - e.g., `assert_eq!(one_error(r#"{"type":"escalation","task":"","artifact":"a","human_decision":"decision"}"#), "field \`task\` is empty")`.

---

## What was verified correct

**Migration honesty (all 15 waivers):**

Zero-record steps (11 predates-logging step waivers): confirmed by extracting all pre-migration round records. None of core-assets, file-dropper, idempotency-safety, selection-ui, mode-enum, tag-selection, available-filter, pack-manifest, external-packs, pack-owned-principles, or init-vcs appear in any `type:"round"` record before the waiver records were appended. The waiver type (step-unit, predates-logging, self-declared) is correct for zero-record steps.

Short-streak steps (3 review-skipped increment waivers): confirmed by extracting the pre-migration records for convergence-accounting, pack-rebuild-tracking, and user-prompts-dir. Each has exactly one `type:"round"` record with `outcome:"new_valid"`, `consecutive_clean:0`, `risk_class:"low_risk"`. Low-risk requires a streak of 1; peak streak is 0. The shortfall is genuine. The waiver type (increment-unit, review-skipped, self-declared) is the best available reason in the model's vocabulary for "clean follow-up round was not done." The increment token equals the bare step slug (no -inc suffix) matching the round's `task`; `leading_slug` returns the full string for tasks without a valid -inc suffix, so the W3 increment match (`waiver.increment == task`) and the W5 slug check (`leading_slug(increment) == step`) both hold.

Record-backed waiver (optional-modules-inc2cii): the pre-migration records show five rounds for `optional-modules-inc2cii` (four `new_valid`, one `clean` with `consecutive_clean:1`), for `risk_class:"risky"` which requires a streak of 2. Peak streak is 1. The escalation record at `task:"optional-modules-inc2cii"` with `human_decision:"decision"` is present pre-migration. The waiver's `evidence:"optional-modules-inc2cii"` joins to this escalation. W5 scope check for increment-unit: `escalation.task == waiver.increment` -> "optional-modules-inc2cii" == "optional-modules-inc2cii". Verified by running `validate --workflow` against the live repo, which exits 0 with "workflow invariants hold."

**Roadmap count**: 14 rows changed from `grandfathered` to `complete`, 1 row changed from `in progress` to `complete` (optional-modules). Total 15 rows flipped. 15 waivers added to workflow.jsonl. 1:1 mapping holds. `waiver-model` remains `next`.

**Plan scope constraint**: the agent-scaffold.md diff has exactly one hunk covering lines 127-184 (the Roadmap table). No non-table lines are modified (verified by filtering the diff to non-table additions). Status line, Step Details, and Open Questions are untouched as required.

**Consistency:**
- AGENTS.md and .agents/AGENTS.reference.md are byte-identical (`diff` reports no differences).
- pack/instrument.md and pack/plan-template.md contain the same waiver description text as the two AGENTS files (verified by grep; pack files are the pack source, so they match the rendered output in AGENTS.md).
- CHANGELOG.md properly retires `trivial`/`grandfathered` and describes W3 refactor and new W5 check.
- src/plan.rs doc comment updated; `ROADMAP_STATUSES` no longer contains `trivial` or `grandfathered`; plan validator test updated to reject them and verify `skipped` stays accepted.
- src/workflow.rs module doc updated; W3 doc comment updated to describe convergence-OR-waiver; W5 fully documented.
- docs/plans/TEMPLATE.md updated with the new status vocabulary and waiver explanation.
- No stale `baseline-cutoff` note remains in AGENTS.md / pack/instrument.md (the second commit removed it).
- No stale `trivial`/`grandfathered` appear in live code (only in historical exploration docs, test fixture strings testing their rejection, retirement comment in CHANGELOG, and the plan Status line / Step Details which are correctly left untouched per the migration spec).

**Test coverage (overall assessment):**
- Schema validation for waiver records: all six new constraints tested individually (bad unit, bad reason, bad evidence_tier, empty step, forbidden increment on step-unit, missing increment on increment-unit, forbidden evidence on self-declared, missing evidence on record-backed).
- `parse_waivers` projection: well-formed and malformed records in combination; confirms malformed waiver is dropped not projected.
- `parse_escalations` projection: well-formed and non-escalation records in combination.
- W3 with waivers: step waiver covers no-records step, increment waiver covers short-streak, step waiver does not cover short-streak increment (unit specificity), risk_class inconsistency not suppressed, mis-scoped increment waiver (step mismatch) does not exempt, bare-slug increment waiver exempts correctly.
- W5: nonexistent step, record-backed without escalation, record-backed with matching escalation (passes), all three forbidden reason/tier pairings flagged, all three valid pairings accepted, resume-not-decision escalation fails, step-unit join by leading_slug passes and fails correctly, increment waiver naming wrong step (leading_slug mismatch) flagged, unrelated escalation does not launder, step-unit join pins both branches.
- End-to-end: `check_workflow_passes_the_optional_modules_migration_shape` mirrors the live migration shape.
- Plan validator: retired statuses rejected, skipped accepted, drift guard extended to include waiver.
- All 213 tests pass. Clippy passes with no warnings. ASCII-clean diff confirmed.
