# Review: state-schema increment 2 (validate verb + metrics validator + V-4 fields)

Reviewer: sonnet (schema consistency and design lens)
Diff range: `4de2b35..bdc0955`
Files changed: `Cargo.toml`, `Cargo.lock`, `pack/instrument.md`, `src/main.rs`, `src/metrics.rs`

---

## Schema comparison result: PASS

The key check - exact match between the prose schema in `pack/instrument.md` and the Rust validator in `src/metrics.rs` - passes. Field-by-field:

**Common fields (every record):**
- `type`: required string in prose; `require_str(obj, "type")` in code. Match.
- `task`: required string in prose ("a string naming the plan/ledger task"); `require_str(obj, "task")` in code. Match.
- `ts`: optional string in prose ("may carry `ts`"); code does `if obj.contains_key("ts") { require_str(obj, "ts")?; }`. Match.

**`type: "round"`:**
- `artifact`: string. Code: `require_str(obj, "artifact")`. Match.
- `phase`: enum `plan_review`/`work_review`/`acceptance`. Code: `Phase` enum with those exact literals. Match.
- `changed_since_prev`: boolean. Code: `require_bool`. Match.
- `outcome`: enum `clean`/`new_valid`. Code: `RoundOutcome` enum with those exact literals. Match.
- `valid_findings`: count. Code: `require_count`. Match.
- `severities`: list of severity strings. Code: `require_severities` with `Severity` enum (`low`/`medium`/`high`/`critical`). Match.
- `consecutive_clean`: count. Code: `require_count`. Match.

**`type: "escalation"`:**
- `artifact`: string. Match.
- `human_decision`: enum `decision`/`resume`. Code: `HumanDecision` with those literals. Match.

**`type: "dismissal_recheck"`:**
- `artifact`: string. Match.
- `result`: enum `upheld`/`overturned`. Code: `RecheckResult` with those literals. Match.

**`type: "intake"`:**
- `classification`: enum `trivial`/`non_trivial`. Code: `Classification` with those literals. Match.
- `replanned`: boolean. Code: `require_bool`. Match.

No field present in one but absent from the other. No enum value spelled differently. No required-vs-optional mismatch. The `enum_field!` macro is a sound internal design: each enum's accepted spellings and the type modelling it live in one place, so the validator cannot accept a value it does not recognise (Principle 16 within the code).

**Calibration coverage check (workflow-calibration "Data to gather"):** Every field listed in the workflow-calibration step is recordable and every recorded field has a stated calibration purpose. The V-4 addition (`task`, optional `ts`) directly satisfies the "records cannot be segmented by task" gap it was meant to close. No surplus fields, no missing ones.

**serde_json dependency:** Justified. The use case - parse each line as arbitrary JSON then inspect fields by name with type checks - requires a `Value`-style dynamic representation. `serde_json = "1"` is the idiomatic and minimal specification; there is no feature flag to strip serialization capability, and no lighter crate provides the `Map<String, Value>` API needed for field-by-name inspection.

**Exit-code convention:** Sensible and documented. `std::process::exit(1)` for validation failures; clap's built-in `exit(2)` for usage errors; `Ok(())` (exit 0) for absent log and valid log. The `run_validate` doc comment states the distinction clearly. The split matches the plan's hard-fail decision in Q-24.

**`validate --help` and instrument.md consistency:** Confirmed by running `direnv exec . cargo run -- validate --help`. The help text reads "Validate the workflow's metrics log against the record schema; exits non-zero on any malformed record." The instrument.md addition reads "The log can be checked against this schema with `agent-scaffold validate`, which exits non-zero and reports any malformed record." Both are accurate descriptions of the behaviour; neither contradicts the other or the code.

---

## Findings

### F1 - MEDIUM: Schema-in-two-places has no automated alignment guard

**Evidence:** `src/metrics.rs` module doc (line 2) says "record schema pinned in `pack/instrument.md`", and instrument.md's new final paragraph says "The log can be checked against this schema with `agent-scaffold validate`." Both pointers exist and the two sources are in sync now. However, no test reads `pack/instrument.md` and verifies that the code's accepted field names and enum values match it. Any future edit to instrument.md's field list or enum spellings that is not mirrored in `src/metrics.rs` (or vice versa) will silently produce drift, since there is nothing to catch it before a human notices at runtime.

This is the project's own stated "recurring single-source-of-truth failure class" and Principle 16 tension: the schema exists in two places by deliberate design (one for LLM authoring, one for enforcement), but the only current guard is the prose pointer in the module doc.

**Recommendation:** Add a doc-test or unit test in `src/metrics.rs` that reads the embedded `pack/instrument.md` (accessible via `include_str!` at the path relative to the source file, or via `include_dir`) and asserts that each expected type name and at least one representative enum value from each per-type enum appears in it. This is a light check - it does not parse instrument.md's markdown structure - but it would fail if a field or type is renamed in one place and not the other, turning a silent drift risk into a visible CI failure.

Alternatively, add a comment directly above each `match record_type` arm pointing to the specific bullet in instrument.md it implements, making the mapping auditable on a per-arm basis. Weaker than a test but still improves traceability.

**Severity:** medium. No immediate bug; the two sources are currently identical. But the drift risk is real and documented as the project's recurring failure mode, and there is no guard short of a human manually cross-checking both files after every future edit.

---

### F2 - LOW: `ts` field validated as any string, not as ISO 8601

**Evidence:** `pack/instrument.md` describes the `ts` field as "may carry `ts` (an ISO 8601 timestamp)." The code in `src/metrics.rs:check_record` (the `if obj.contains_key("ts")` block, around line 239-241) calls `require_str(obj, "ts")`, which accepts any JSON string. A record with `"ts": "not-a-date"` passes validation without error.

This is a schema mismatch between the prose ("ISO 8601 timestamp") and the code ("any string"). The validator's stated job is to check records "against the schema" - the format of `ts` is part of the schema.

The impact is limited: `ts` is optional and the primary calibration signals (counts, severities, outcomes) do not depend on it. A malformed timestamp would only affect downstream time-based analysis. Still, a validator that accepts an obviously wrong timestamp is less useful than one that catches it.

**Severity:** low.

---

## Findings not raised

- **Status/plan-parsing absent:** Increment 3; excluded per the review scope.
- **Cargo.lock `serde_json` transitive dependencies (`serde_core`, `zmij`):** These appear to be post-training-cutoff versions of standard serde_json internals. No finding raised.
- **`path.exists()` before `read_to_string`:** A TOCTOU pattern, but the window is negligible for a single-user dev tool and the error handling for a failed `read_to_string` (OS error message, exit 1) is acceptable. Not a finding.
- **Test data: `valid_findings: 3` with `severities: ["high","low"]` (2 elements):** No invariant linking these two fields is stated in the prose or enforced by the validator. The test is about structural validity. Not a finding.
