# Review: state-schema increment 2, round 2 (confirming)

Reviewer: sonnet (round-2 confirming pass) Fix range: cbe3508..c9a6c1c Risk class: low (one clean round to converge)

---

## Verification steps performed

### F1 - drift-guard test

The test `instrument_prose_documents_every_accepted_schema_value` exists at `src/metrics.rs:410`. It:

- Iterates the four record-type names (`round`, `escalation`, `dismissal_recheck`, `intake`) from a hardcoded list and asserts each appears in the prose. The comment correctly documents that this list is hardcoded and must be kept in sync manually.
- Iterates 14 field names from a hardcoded list (mirrors the fields `check_record` requires or optionally validates). Same manual-sync requirement, correctly stated in the comment.
- Iterates enum variants from each type's own `VARIANTS` slice (`Phase::VARIANTS`, `RoundOutcome::VARIANTS`, `HumanDecision::VARIANTS`, `RecheckResult::VARIANTS`, `Classification::VARIANTS`, `Severity::VARIANTS`) and asserts each appears in the prose. This is the key improvement: a code-side rename is automatically reflected in the check, not re-hardcoded.

Test suite: 64 tests pass (`just test`). Clippy clean (`cargo clippy --all-targets -- -D warnings`).

### F1 - drift-guard actually works (induced-drift test)

Temporary edit: changed `Medium => "medium"` to `Medium => "moderate"` in the `Severity` enum at `src/metrics.rs:102`.

Result from `cargo test instrument_prose_documents_every_accepted_schema_value`:

```
test metrics::tests::instrument_prose_documents_every_accepted_schema_value ... FAILED

thread 'metrics::tests::instrument_prose_documents_every_accepted_schema_value' panicked at src/metrics.rs:456:17:
enum `Severity` value `moderate` accepted by the validator is not documented in pack/instrument.md
```

The guard failed with a message that names both the enum and the undocumented value. Revert: changed `"moderate"` back to `"medium"`. Confirmed `just test` returns 64 passed. `git diff src/metrics.rs` is empty. `git status --short` shows only the two pre-existing round-1 review files as modified (not my changes). Working tree is clean of my edits.

### F2 - ts prose wording

`pack/instrument.md` line 3 now reads: "may carry `ts` (a timestamp string, ISO 8601 recommended)". The validator accepts any string for `ts` when present (`require_str` at `src/metrics.rs:199`). The new wording correctly describes the validation (any string, with ISO 8601 as a recommendation rather than a hard constraint). Fix landed correctly.

### F2 - severities scale fully documented

The updated `severities` prose (line 5 of `pack/instrument.md`) reads: "list of severity names on the four-level `low`/`medium`/`high`/`critical` scale". All four values from `Severity::VARIANTS` (`low`, `medium`, `high`, `critical`) are now present verbatim in the prose. The test confirms this programmatically.

### scaffold-self check

`just scaffold-self` ran and `git diff --stat -- AGENTS.md .agents/ docs/plans/TEMPLATE.md` produced no output. Those files are byte-identical after the tool re-renders them. The `pack/instrument.md` change affects only the `--instrument` render path and does not disturb the reference outputs.

### Scope check

No files changed in the fix range outside `src/metrics.rs` and `pack/instrument.md`. No new public API, no new dependencies, no Roadmap or Success Criteria changes. Nothing new introduced beyond the two fixes.

---

## Findings

### LOW-1 - `prose.contains("ts")` is a weak substring check for the `ts` field name

Severity: low

Evidence: `src/metrics.rs:439` asserts `prose.contains("ts")` for the field named `ts`. The string `"ts"` is a substring of several words already present in the prose regardless of whether the `ts` field is explicitly documented: `"tasks"` (appears twice on line 3 of `pack/instrument.md`), `"metrics"` (appears on lines 3 and 10), `"instruments"`, `"exists"`, etc. If a future edit removed the explicit `` `ts` (a timestamp string, ISO 8601 recommended) `` clause while leaving the surrounding prose intact, `prose.contains("ts")` would still pass, and the guard would not catch the removed documentation.

The current state is correct: the field IS documented (line 3 of `pack/instrument.md`), so the test passes for the right reason. The weakness is prospective: the guard is structurally unable to detect removal of the `ts` documentation as long as any word containing the substring `ts` remains in the prose.

Impact if left unfixed: the drift guard for the `ts` field name specifically is weaker than intended. The practical consequence is small because `ts` is optional, the validator for it is simple, and the test still guards all enum spellings (the higher-value checks) correctly via VARIANTS. No current correctness defect.

Suggestion: use a more anchored check, for example `prose.contains("`ts`")` (with backticks), which would match the field name as documented in Markdown and not coincidentally match "tasks" or "metrics". This is a one-line change.

---

## Summary

Both fixes landed correctly and are verified:

- F1: the drift-guard test exists, is driven from VARIANTS for enum spellings, and covers record types and fields. The induced-drift test confirmed it fails with a clear, named error message when a variant spelling is changed in code but not in the prose.
- F2: the `ts` prose now accurately describes string-only validation. The severities scale is fully documented, including `medium` (the value the guard caught was missing).
- scaffold-self: byte-identical.
- No new scope introduced.

One new low finding (LOW-1): `prose.contains("ts")` is a weak substring check that would not catch removal of the `ts` field documentation. Does not block convergence for this low-risk artifact.

No medium, high, or critical findings.
