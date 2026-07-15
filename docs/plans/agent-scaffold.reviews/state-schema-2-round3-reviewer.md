# Reviewer findings: state-schema increment 2, round 3 (LOW-1 confirming review)

Artifact: the metrics drift-guard test in `src/metrics.rs`, after the LOW-1 fix. Diff reviewed: `git diff c9a6c1c..71e4475 -- src/metrics.rs` (commit `71e4475`). Classification: LOW-risk artifact (one consecutive clean round to converge, per the ledger). Scale: four-level (`low` / `medium` / `high` / `critical`).

## Verdict summary

The LOW-1 fix landed and is correct. The drift guard now catches the previously-missed short-name case, demonstrated below. I found no critical, high, or medium findings. I found one new LOW finding (the same substring-weakness class LOW-1 addressed, left unfixed on the third loop of the same test); it is a new observation, not a re-raise of a settled finding.

## What the fix does (verified against the diff)

- The field-name loop (`src/metrics.rs:442-445`) and the enum-value loop (`src/metrics.rs:461-464`) now match `prose.contains(&format!("`{x}`"))` instead of the bare `prose.contains(x)`. A short field or value name (for example `ts`) can therefore no longer pass by appearing as an incidental substring of another word; the match now requires the backtick-wrapped token.
- The change is test-only, mechanical (two match expressions plus explanatory comments), and adds no production logic. The added comments accurately describe the rationale.

## Verification performed (Principle 6)

- `direnv exec . just test`: 64 passed, 0 failed. Passing also proves every field name and every enum spelling IS backtick-wrapped in `pack/instrument.md`; otherwise the anchored checks would fail.
- `direnv exec . cargo clippy --all-targets -- -D warnings`: clean.
- Drift experiment (a), code-side enum rename: temporarily changed `RecheckResult`'s `"overturned"` spelling to `"overturned_DRIFT"` in `src/metrics.rs`. The guard failed as expected: `enum `RecheckResult`value`overturned_DRIFT` ... is not documented` (src/metrics.rs:461). Reverted.
- Drift experiment (b), the previously-missed short-name case: temporarily removed only the backticks around the `ts` field mention in `pack/instrument.md` (leaving the plain word `ts`). The bare substring "ts" still occurs elsewhere in the prose (lines 3, 10, 12), so the OLD `contains("ts")` check would have passed; the NEW anchored check correctly failed: `field `ts` checked by the validator is not documented` (src/metrics.rs:442). This is exactly the case LOW-1 aimed to catch. Reverted.
- After reverting both experiments: `just test` = 64 passed, `git status --short` empty (no tracked-file drift; only this findings file is added by me).

## Findings

### LOW-1-followup (low). The record-type loop keeps the substring weakness the fix removed from the other two loops.

Evidence: the same test has three loops. The fix anchored the field loop (`src/metrics.rs:442`) and the enum loop (`src/metrics.rs:461`), but the record-type loop at `src/metrics.rs:415-416` still uses the unanchored `prose.contains(record_type)` for `["round", "escalation", "dismissal_recheck", "intake"]`.

Two of these names occur as bare English/domain words in `pack/instrument.md` independent of their `type: "..."` documentation:

- `round`: "the round constants" (line 3), "one per review round" and "the streak after this round" (line 5).
- `escalation`: "one per total-round-cap escalation" (line 6).

So `prose.contains("round")` and `prose.contains("escalation")` would still pass even if the `type: "round"` / `type: "escalation"` documentation were deleted. The guard therefore does not truly verify those two record types are documented (Principle 11: the check does not exercise what it claims), which is the identical false-negative class LOW-1 fixed for fields and enums. The fix is incomplete: it left one of the three structurally-identical loops unanchored.

Note on the anchor: record types are documented as `type: "round"` (double-quoted), not backtick-wrapped like fields and enums, so the exact `{x}` anchor does not apply here; the correct anchor is the quoted form, e.g. `prose.contains(&format!("\"{record_type}\""))`. `dismissal_recheck` and `intake` are distinctive enough not to collide today, but `round` and `escalation` are not, so the weakness is real, not hypothetical.

Severity rationale: LOW. Impact is confined to a possible false-negative in a test-only drift guard for two record types; the validator's runtime behaviour is unaffected, and any real drift in the field/enum sets is still caught. Same severity as the original LOW-1.

## Nothing else

- No critical findings.
- No high findings.
- No medium findings.
- No other low findings. The two changed loops are correct; the comments are accurate; nothing new was introduced beyond the record-type loop already noted above (which the fix did not touch, so it is not a regression, only an incompleteness).
- Not raised, per scope: the absence of status / plan-parsing (increment 3), the deferred duplicate-keys item (L1), and line length / wrapping.
