# Correctness review: `risk_class` + `reviewers` round metrics (commit f57c87f)

Reviewer: Opus (correctness lens). Scope: `src/metrics.rs`, `pack/instrument.md`, and the drift-guard test.

Summary: The change is correct and the optional-field gating is sound. No critical, high, or medium correctness defects found. The non-breaking guarantee holds (real 46-record log validates, clippy clean, 19 metrics tests pass), all probed edge cases behave sensibly, and the drift guard fully covers the new fields and the `RiskClass` enum. Two low-severity observations below, both judgement calls; one is a pre-existing structural limitation, not introduced here.

## Verification performed

- `cargo run -- validate --metrics docs/metrics/workflow.jsonl` -> `46 records, valid`, exit 0. The non-breaking guarantee is confirmed against the real log: nothing in this change rejects a pre-existing record that omits both new fields, and a `round` omitting them is independently asserted valid by `optional_round_calibration_fields_are_accepted`.
- `cargo clippy` clean (no warnings on the new code).
- `cargo test metrics` -> 19 passed.
- Edge cases probed directly against the validator (see "Edge cases" below): all behave correctly except the two noted permissive cases.

## Correctness of the validator logic (no defect)

- Optional gating via `obj.contains_key(...)` is correct and mirrors the existing `ts` pattern (metrics.rs:240-242, 258-265). Absent field -> branch skipped -> pass.
- `require_reviewers` (metrics.rs:204-227): array type check, per-element object check, and the four scalar checks are all correct. The `at` closure (metrics.rs:218-220) prefixes the position, so element-level errors carry `field \`reviewers\`[i]: ...`; verified empirically (e.g. `field \`reviewers\`[0]: field \`raw_findings\` value \`1.5\` is not a non-negative integer`). Error-message location prefixing is accurate.
- `require_count` reuse correctly rejects negative and fractional counts inside reviewer entries (`as_u64` returns `None` for `-1` and `1.5`).
- `risk_class` uses `require_enum` -> correct out-of-set message `field \`risk_class\` value \`medium\` not one of [low_risk, risky]`.

## Edge cases (all probed empirically)

- Wrong-typed `role`/`model`: reported as `field \`reviewers\`[0]: field \`role\` has wrong type (expected string)`. Correct.
- Non-object array element (`reviewers:[42]`): `field \`reviewers\`[0] has wrong type (expected object)`. Correct.
- Fractional / negative counts: rejected. Correct.
- `risk_class` present but wrong JSON type (number, null): `field \`risk_class\` has wrong type (expected string)`. Correct (see nit below on message).
- `reviewers: null`: `field \`reviewers\` has wrong type (expected array)`. Correct.
- Reviewer object with extra unknown keys: accepted. Intended (forward-compatible, consistent with the module's documented policy at metrics.rs:231).

## Drift guard coverage (no defect for this change)

`instrument_prose_documents_every_accepted_schema_value` (metrics.rs:503-570) does cover the new schema for this change:

- The new field names `risk_class`, `reviewers`, `role`, `model`, `raw_findings` are added to the field list (metrics.rs:531-535); `valid_findings` was already present and covers the reviewer sub-field too.
- `RiskClass` is added to the VARIANTS-driven enum loop (metrics.rs:558), so a code-side rename of `low_risk`/`risky` is auto-checked at its new spelling and would fail the test unless the prose is updated.
- Confirmed the prose (`pack/instrument.md:5`) documents `low_risk`, `risky`, `role`, `model`, `raw_findings`, `valid_findings` verbatim in backticks. So the schema cannot currently drift from `instrument.md` without failing this test.

## Findings

### Finding 1 (low): empty `reviewers: []` is silently accepted

Location: `src/metrics.rs:204-227` (`require_reviewers`), exercised by the `if obj.contains_key("reviewers")` branch at metrics.rs:263.

An empty `reviewers` array passes validation (the `for` loop body never runs; verified: `{...,"reviewers":[]}` produces no error). The prose defines `reviewers` as "an array with one object per reviewer that examined the artifact this round" (instrument.md:5), and every real round has at least one reviewer, so an empty array cannot represent a real round and is a data error the validator will not catch. This mirrors the deliberate permissiveness of `severities: []` (legitimately empty for a clean round), so it is defensible, and the field is optional (a round with no reviewer data simply omits it). The ambiguous middle, present-but-empty, is the only uncaught case. Severity low; judgement call. Suggested direction: if a present `reviewers` array should always be non-empty, reject an empty array with `field \`reviewers\` is empty`; otherwise leave as-is and consider a one-line test documenting that empty is intentionally accepted.

### Finding 2 (low): drift-guard field list is hand-maintained, so a future field addition can drift undetected (pre-existing, not introduced here)

Location: `src/metrics.rs:520-548` (the field-name list in `instrument_prose_documents_every_accepted_schema_value`).

The enum half of the guard is derived from each type's `VARIANTS` (metrics.rs:552-569), so it is genuinely drift-proof. The field-name half is a hardcoded list, disconnected from `check_record`; the comment at metrics.rs:501-502 acknowledges this ("if a field is added to or removed from the validator, update this list to match"). For this change the list was updated correctly, so there is no current drift. The latent weakness is that a future field added to `check_record`/`require_reviewers` without a matching list entry would not be caught, and the substring match (`prose.contains("\`role\`")`) does not verify a field is documented in the right context. This is a pre-existing design property of the guard, not something this commit worsened; raising only because the review lens asked whether the schema can drift without a test failing. Severity low. Suggested direction: none required for this change; longer term the field list could be derived from a single declarative schema so both halves are auto-checked.

## Nit (not a finding)

- `risk_class: null` reports `has wrong type (expected string)` rather than the enum-set message, because `require_enum` delegates to `require_str` first (metrics.rs:166). This is consistent with how every other enum field and the optional `ts` field behave, so it is not a defect; noting only for completeness.
