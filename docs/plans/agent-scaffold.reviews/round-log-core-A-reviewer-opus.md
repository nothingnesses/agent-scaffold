# round-log-core increment A: reviewer (opus, correctness lens)

Branch `impl/round-log-core`, HEAD `1ef49c7`, diff `f3855a3..HEAD`.
Scope reviewed: (1) `src/metrics.rs` making `risk_class` required on `type:"round"` records; (2) the `risk_class` backfill into `docs/metrics/workflow.jsonl`.

## Verification (all green)

- `just test`: 96 passed, 0 failed. Matches the expected 96.
- `just clippy` (`cargo clippy --all-targets`): clean, no warnings.
- `cargo run -q -- validate --metrics docs/metrics/workflow.jsonl`: `52 records, valid`.

## Code change (`src/metrics.rs`)

Correct. `risk_class` is now checked with an unconditional `require_enum(obj, "risk_class", RiskClass::VARIANTS, ...)` (was gated behind `if obj.contains_key("risk_class")`). `require_enum` -> `require_str` returns `missing field \`risk_class\`` when absent, so a pre-backfill round now fails. `reviewers` stays optional (still behind `if obj.contains_key("reviewers")`). Check order is sensible: the scalar `risk_class` is checked before the nested `reviewers`, after the other required scalars.

Test coverage is correct and each test now fails/passes for the right reason:
- The renamed `the_optional_reviewers_field_is_accepted_present_or_absent`: both `with` and `without` fixtures now carry `risk_class:"low_risk"`, so the `without` case exercises "reviewers omitted, risk_class present" rather than the old "both omitted". Correct.
- New `a_round_missing_risk_class_is_reported`: a round with neither `risk_class` nor `reviewers` asserts `missing field \`risk_class\``. Because the `risk_class` check precedes the `reviewers` check, this asserts the right first error; field-check order matters here and is respected.
- `a_bad_risk_class_is_reported`: unchanged, still asserts the enum error for `"medium"`.
- The five `reviewers` error tests (`..._missing_a_field`, `..._of_wrong_type`, `an_empty_reviewers_array`, `a_non_object_reviewers_element`, `..._with_a_bad_count`) each had `risk_class:"low_risk"` inserted into their fixtures. This insertion is NECESSARY and correct: without it those fixtures would now fail on `missing field \`risk_class\`` before reaching the `reviewers` check, so each test still asserts its intended reviewers error rather than passing/failing for the wrong reason.

## JSONL backfill mechanics

Verified programmatically (jq + line-by-line diff against `f3855a3:docs/metrics/workflow.jsonl`):
- 52 lines, all valid JSON, one object per line, no blank/corrupt lines.
- Every record now carries `risk_class`; values are only `low_risk` (46) or `risky` (6). No missing, no out-of-set values.
- Lines 47-52 are byte-identical old vs new (they already had `risk_class` mid-object plus `reviewers`/`ts`); untouched as required.
- Only lines 1-46 changed (`git diff` reports `1,46c1,46`). On each of those 46 lines `risk_class` is the sole added key, appended as the last field immediately before the closing brace; no other field was added, removed, reordered, or altered (confirmed by stripping the inserted `,"risk_class":"..."` and matching the result byte-for-byte to the old line).

(Classification judgment, i.e. whether each low_risk/risky label is the right tier, is out of my lens; the sonnet reviewer covers it.)

## Edge-case interactions

- `count_records` counts non-blank lines only; unaffected by `risk_class` being required.
- `run_status` / `status --json` (`src/main.rs:471`) reports `count_records` only; it does not parse `risk_class`, so no interaction.
- The metrics drift guard (`instrument_prose_documents_every_accepted_schema_value`) lists `risk_class` and iterates `RiskClass::VARIANTS`; it passes and correctly still requires the field name to appear in the prose. See the finding below on what it does NOT catch.

## Findings

### F1 (medium): `pack/instrument.md` still documents `risk_class` as optional, now contradicting the validator

`pack/instrument.md:5` still reads: "Two optional calibration fields should be included when known: `risk_class` (...) and `reviewers` (...). ... A record written without these optional fields still validates." The validator now REQUIRES `risk_class`, so this prose is factually wrong: a round record written without `risk_class` no longer validates. The drift-guard comment names `pack/instrument.md` as the human-readable half of the single-source schema (Principle 16), but the drift-guard test only asserts the field name `risk_class` appears in backticks, so it does not catch an optional-vs-required mismatch and the test stays green.

Concrete harm: an orchestrator following `instrument.md` is told it may omit `risk_class` and the record still validates, which will now produce a validation failure. The plan scopes increment A as "updating fixtures and the drift guard" and does not name the `instrument.md` prose; increment B names the ledger template and `AGENTS.md`/orchestrator convergence prose but also not this line. So the `risk_class` optionality sentence risks falling through the increment boundary. Recommend correcting the prose (move `risk_class` out of the "optional calibration fields" description; keep `reviewers` as the sole optional one) in this increment or explicitly tracking it, and consider whether the drift guard should assert required-vs-optional, not just field presence.

Location: `pack/instrument.md:5`.

## Summary

No correctness defects in the code change or the backfill mechanics; the validator, tests, clippy, and log validation are all green and internally consistent. One medium documentation-drift finding (F1): the pack prose still calls `risk_class` optional and states records without it validate, which the now-required validator contradicts and the drift-guard test does not catch.
