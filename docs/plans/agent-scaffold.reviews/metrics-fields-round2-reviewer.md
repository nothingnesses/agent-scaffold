# Round 2 reviewer findings: metrics-fields

Commit under review: e210002 ("fix: apply metrics-fields triage verdicts (round 1)")
Artifact risk class: low_risk (one consecutive clean round required to converge)

## V-1 (doc): counting-rule prose in instrument.md / AGENTS.md / .agents/AGENTS.reference.md

CONFIRMED LANDED.

The line in `pack/instrument.md` now includes the following appended sentence block:

> The two counts are per reviewer against that reviewer's own findings: a finding raised by two reviewers and judged valid credits each reviewer's `valid_findings` and counts once in the round-level `valid_findings`, so the per-reviewer `valid_findings` can sum to more than the deduplicated round-level total (do not expect them to match). Omit the `reviewers` array for a round you are not attributing, rather than writing an empty one.

Accuracy assessment:
- "The two counts are per reviewer against that reviewer's own findings" correctly describes both `raw_findings` and `valid_findings` as self-reported by each reviewer.
- The shared-finding example is correct: each crediting reviewer's `valid_findings` increments by 1, the round-level `valid_findings` increments by 1 once, so the per-reviewer sum can exceed the round-level total.
- "Omit the `reviewers` array ... rather than writing an empty one" is consistent with the validator behavior introduced by V-2.
- No inaccuracies or ambiguities found.

Rendered identically into AGENTS.md and .agents/AGENTS.reference.md: confirmed. `diff AGENTS.md .agents/AGENTS.reference.md` returns no output (files are byte-for-byte identical). The commit diff shows both files have the same before and after blob hash (71a7d5a -> 3536a51), which is structural confirmation they were generated from the same source. `just scaffold-self` substitutes `pack/instrument.md` via the `{{instrument}}` template variable, so both destination files pick up the same prose.

## V-2 (validator + test): empty reviewers array rejection

CONFIRMED LANDED.

In `src/metrics.rs` at line 215, after the type check and before element iteration, `require_reviewers` now includes:

```rust
if array.is_empty() {
    return Err(format!("field `{name}` is empty"));
}
```

Called from line 270 as `require_reviewers(obj, "reviewers")`, so the error message correctly reads "field `reviewers` is empty".

The check is correctly placed: it fires only when the `reviewers` key is present (the `obj.contains_key("reviewers")` guard at line 269 gates the call), consistent with the optional field contract.

Test `an_empty_reviewers_array_is_reported` (line 479) passes and asserts the exact error string "field `reviewers` is empty" for a round record with `"reviewers":[]`.

## V-3 (test): non-object reviewers element

CONFIRMED LANDED.

Test `a_non_object_reviewers_element_is_reported` (line 488) passes. It supplies `"reviewers":[42]` and asserts the error "field `reviewers`[0] has wrong type (expected object)", which matches the existing `as_object().ok_or_else(...)` path at line 220-221 in `require_reviewers`.

## Verification commands

- `just test`: 95 passed, 0 failed. Both new tests appear in the output: `test metrics::tests::an_empty_reviewers_array_is_reported ... ok` and `test metrics::tests::a_non_object_reviewers_element_is_reported ... ok`.
- `just clippy`: clean, no warnings.
- `direnv exec . cargo run -q -- validate --metrics docs/metrics/workflow.jsonl`: "46 records, valid".
- Drift guard `instrument_prose_documents_every_accepted_schema_value`: passes. Covers record types, field names (including `risk_class` and `reviewers`), and all enum variants.

## New defects

None. No new defect introduced by the fix.

## Severity summary

- Critical: none.
- High: none.
- Medium: none.
- Low: none.

The fixes are correctly applied and the artifact is clean.
