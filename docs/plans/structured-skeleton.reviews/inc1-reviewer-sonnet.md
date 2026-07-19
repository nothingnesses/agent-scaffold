# Inc 1 review: structured-skeleton-inc1

Reviewer: sonnet (independent).
Change range: `a780541..aa42412`.
Files changed: `src/plan/source.rs` (new), `src/plan/testdata/skeleton.plan.toml` (new), `src/plan.rs`, `src/main.rs`, `src/metrics.rs`, `src/workflow.rs`.
Tests run: 229 passed, 0 failed (matches claim). Clippy: clean (matches claim).

## Summary

No CRITICAL or HIGH findings. Two MEDIUM findings and six LOW findings. The headline concern is S1: `workflow.rs` live W5 enforcement code was refactored in an increment explicitly described as a pure addition unconsulted by W3/W4/W5 (Principle 8 violation). S2 is a concrete validation gap: waiver `id` uniqueness is not checked, which will matter when Inc 4 needs to join on waiver ids. Schema fidelity against the design-B spec is otherwise good; all described fields are present and correctly typed. The `options`/`chosen` absence from `[[question]]` is correctly enforced. Tests cover every acceptance-criterion case from the plan.

---

## Findings

### S1 - medium - `workflow.rs` W5 live code modified in a "pure addition" increment

**Evidence:** `src/workflow.rs` diff (lines 43-44, 411-926 range in the new file). The W5 pairing check in `w5_problems` was refactored from an inline `match waiver.reason { ... }` to `waiver.reason.required_tier() == waiver.evidence_tier`. The `WaiverReason` import was also removed.

**Defect:** The Inc 1 plan states "Risk: LOW (a pure addition, unconsulted by W3/W4/W5)". `workflow.rs` hosts the live W5 enforcement path. The refactor is functionally neutral and its goal (single-sourcing the pairing logic via `required_tier()`, Principle 16) is correct, but it was not listed as in-scope for Inc 1, and Principle 8 requires flagging anything beyond what was asked rather than silently doing it. An incident where the refactor introduced a regression would be invisible to a reviewer who trusted the "pure addition" characterisation.

**Direction:** Accept the refactor (it is functionally correct and enforces Principle 16), but flag it in the commit message or the plan ledger as intentional scope relative to the pure-addition claim, so the scope is visible to the triager.

---

### S2 - medium - waiver `id` uniqueness is not validated

**Evidence:** `src/plan/source.rs` lines 429-451 (slug and increment-id uniqueness loops) and lines 536-601 (waiver integrity loop). A `BTreeSet` is built for step slugs and for increment ids, but no such set is built for waiver ids. Two waivers on the same step, or across steps, can share an id and `validate_source` returns no problem.

**Defect:** The plan describes the waiver `id` as "the waiver's stable id" and Inc 4 (W5 TOML read-path) will need to join on waiver ids (matching a TOML waiver to its JSONL escalation evidence). Duplicate waiver ids are undetected here, meaning a malformed document reaches Inc 4 without a prior rejection. The slug and increment-id checks demonstrate the intended pattern; waiver ids were omitted.

**Direction:** Add a `BTreeSet<&str>` for waiver ids, checking uniqueness per-waiver inside the existing `step.waivers` loop. Scope of uniqueness (per-step vs. plan-wide) should match how Inc 4 will look them up; plan-wide uniqueness is the safer default.

---

### S3 - low - `parse_toml` returns `PlanToml` (named struct) rather than the plan-specified `(Vec<Step>, Vec<Question>, Meta)` tuple

**Evidence:** `src/plan/source.rs` lines 175-193 (struct definition) and lines 388-396 (function signature). The plan bullet specifies "a strict `plan::parse_toml -> (Vec<Step>, Vec<Question>, Meta)`". The implementation returns `PlanToml { meta, steps, questions, principles }`.

**Defect:** The return type deviates from the spec. The implementer's rationale (dropping principles from the tuple would silently lose data the schema defines) is correct, and the named struct is a strict superset of the tuple, but it is still a literal departure from the written contract. The review prompt explicitly flags this as a check point. Nothing depends on the signature yet (Inc 1 is additive), so the impact is low.

**Direction:** Accept the named struct (it is the right design for Inc 3 and Inc 4 callers), but record the deviation from the spec explicitly, either in a plan update or the ledger, so future increment plan authors use `PlanToml` rather than the tuple type.

---

### S4 - low - `[meta.sidecars]` sub-table not described in the plan's `[meta]` field list

**Evidence:** `src/plan/source.rs` lines 209-247 (`Sidecars` struct, `Meta.sidecars` field). `src/plan/testdata/skeleton.plan.toml` lines 14-16 (`[meta.sidecars]` sub-table). The Inc 1 plan bullet lists "the front-and-tail prose-sidecar references" as fields within `[meta]` alongside `title`, `w4_baseline`, `primary`, and `render_sha256`, with no mention of a `sidecars` sub-table.

**Defect:** The TOML schema shape diverges from what the plan implies. Authors writing `.plan.toml` files in Inc 5/6 will need `[meta.sidecars]` with nested `front`/`tail` keys rather than direct `[meta].front` / `[meta].tail` fields. The sub-table grouping is a reasonable design choice (and cleanly separates the sidecar concerns), but it is an undisclosed schema decision relative to the plan description.

**Direction:** Either flatten `front`/`tail` into `Meta` directly, or update the plan's `[meta]` field description to say `sidecars` is a sub-table with `front` and `tail` keys. The latter is simpler given the fixture already tests the sub-table form.

---

### S5 - low - `is_well_formed_token` allows uppercase in step slugs and increment ids

**Evidence:** `src/plan/source.rs` lines 398-407 (`is_well_formed_token`). The function permits any ASCII alphanumeric character, including uppercase. It is applied to step slugs (`slug`), increment ids (`increment.id`), and orphan-task tokens (`orphan_tasks`).

**Defect:** The plan calls the step slug a "kebab-case slug", which conventionally means lowercase-only with hyphens. Increment ids follow the same convention (e.g., `structured-skeleton-inc1`). Uppercase is needed for orphan-task tokens (the fixture uses `round-log-core-incA`) but not for slugs or increment ids. A step named `Alpha` or an increment named `Alpha-Inc1` would pass validation undetected.

**Direction:** Either split the validator (a lowercase-only check for slugs and increment ids, a mixed-case check for orphan tasks), or add a `!token.bytes().any(|b| b.is_ascii_uppercase())` guard and pass an `allow_uppercase: bool` parameter. The comment on the function already explains the exception, so the implementation knows about the distinction; the guard is not wired.

---

### S6 - low - fixture coverage gaps in step and question status variants

**Evidence:** `src/plan/testdata/skeleton.plan.toml`. Step statuses exercised: `complete`, `in-progress`, `next`. Step statuses NOT exercised: `not-started`, `skipped`, `optional`, `deferred` (four of seven `StepStatus` variants). Question statuses exercised: `open`, `decided`, `superseded`. NOT exercised: `exploring`.

**Defect:** The round-trip test (`the_fixture_skeleton_parses_and_round_trips_including_nested_waivers`) only verifies that the exercised variants survive deserialise-serialise-reparse. The unexercised variants are tested only implicitly by the serde enum derive. A serialisation-name typo in `StepStatus::Optional` (for example) would not be caught by the current fixture. Principle 11 (test adequacy).

**Direction:** Add entries for the missing variants to the fixture, or add a separate small parse test for each unexercised variant. Minimal cost; the enum is small.

---

### S7 - low - no bidirectional field-presence enforcement between status and cross-reference fields

**Evidence:** `src/plan/source.rs` lines 482-514 (question cross-reference checks). The code requires `folded_into` for `Decided` questions but does not forbid it on `Open`, `Exploring`, or `Superseded` questions. The code validates `superseded_by` when present but does not require it for `Superseded` questions, and does not forbid it on `Decided` or `Open` questions.

**Defect:** A `superseded` question with no `superseded_by` passes validation (the reference is declared optional but is semantically required when the status is `superseded`). A `decided` question that also carries a `superseded_by` passes validation. A `superseded` question with a `folded_into` passes validation. These are illegal combinations that `validate_source` should catch. The pattern for `Decided`/`folded_into` shows the implementer understands the rule; the symmetrical rules for `superseded` were omitted.

**Direction:** Add a `QuestionStatus::Superseded` arm that requires `superseded_by` to be present. Add a guard that `folded_into` is absent unless the status is `Decided`, and that `superseded_by` is absent unless the status is `Superseded`.

---

### S8 - low - no test for `folded_into` pointing to a non-existent step

**Evidence:** `src/plan/source.rs` lines 490-494 (the dangling-`folded_into` problem push) and lines 682-710 (test list). The code correctly reports a problem when `folded_into` names a slug that is not a step. The acceptance criteria from the plan say to test "a decided question with no `folded_into`" but not a decided question with a dangling `folded_into`. There is no test for the second case.

**Defect:** The code path (`Some(target) if !slugs.contains(target.as_str()) => problems.push(...)`) is exercised by zero tests. A regression that deleted this arm would go undetected. Minor coverage gap (Principle 11).

**Direction:** Add a test analogous to `a_dangling_blocked_by_is_flagged` but for `folded_into`: a `decided` question whose `folded_into` names a slug absent from the `[[step]]` list.

---

## What was ruled out

- `options`/`chosen` on `[[question]]`: correctly absent from the `Question` struct. Sub-question 3(c) is satisfied.
- All `[meta]`, `[[step]]`, `[[step.increment]]`, `[[step.waiver]]`, `[[question]]`, and `[[principle]]` fields from the Inc 1 spec: all present, correctly typed.
- `Primary` enum default: correctly `Markdown` when `primary` is absent (the `primary_defaults_to_markdown_when_absent` test covers this).
- `serde` derives added to unneeded `enum_field!` enums (`Phase`, `RoundOutcome`, `HumanDecision`, `RecheckResult`, `Classification`): harmless (the macro-generated `#[serde(rename = $text)]` uses the same text as `.parse()`, so no divergence risk) and these enums will likely need serde in later increments. Not raised.
- `validate --plan` on the live plan: the `src/plan.rs` Markdown-parsing path is untouched by this increment; no risk identified.
- The waiver presence rules relative to `check_record`: the `increment`/`evidence` presence and absence checks in `validate_source` correctly mirror the `check_record` waiver arm. The `validate_source` check is stricter in one place (it also checks that the named increment is one of the step's declared increments, which `check_record` cannot do without a full plan parse), which is appropriate.
- Test count: 229 as claimed (225 unit + 1 integration + 3 integration), all passing.
