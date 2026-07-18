# Review: `decision-receipt` branch (commit `70903bd`)

Lens: schema-doc consistency and test quality.
Branch: `impl/decision-receipt`, diff `main..impl/decision-receipt`.
Build: 179 tests pass, clippy clean.

---

## Findings

### F-1: Missing test for `options` being a non-array type

SEVERITY: medium

`src/metrics.rs:282-284` (`require_options`, the `as_array()` check that returns "field `options` has wrong type (expected array)").

`require_options` has three error paths: missing field, wrong type (non-array), empty array, and non-string elements. Tests cover missing field (`a_decision_missing_options_is_reported`), empty array (`a_decision_with_an_empty_options_array_is_reported`), and non-string element (`a_decision_with_a_non_string_option_is_reported`). There is NO test for `options` being a non-array value, for example `"options": "A,B"`. A mutation removing or commenting out the `.as_array()` call at line 282-284 would produce a panic or incorrect behaviour on that path but would not be caught by any existing test.

The parallel exists for `reviewers`: `a_reviewers_field_of_wrong_type_is_reported` (line 699) tests `"reviewers":"opus"` and asserts "field `reviewers` has wrong type (expected array)". No analogous test exists for `options`. The gap is in Principle 11 terms: the test suite claims to exercise the validator for the new record type, but the wrong-type branch for the only array field goes untested.

Why it matters: the wrong-type path is a real failure mode if an orchestrator serialises `options` as a comma-joined string instead of an array. Without a test, the path can silently regress.

---

### F-2: Duplicated `QUEUE_FOLD_PREFIX` constant lacks a guard against silent drift

SEVERITY: low

`src/workflow.rs:48` and `src/plan.rs:87` each define `const QUEUE_FOLD_PREFIX: &str = "decided -> folded into ";` independently.

The comment at `workflow.rs:43-47` acknowledges the copy and states "the plan drift guard pins the template's copy." The plan drift guard in `plan.rs` (`plan_template_documents_every_accepted_status`, around line 754) checks that `plan.rs`'s `QUEUE_FOLD_PREFIX` (trimmed) appears in `pack/plan-template.md`. It does NOT verify `workflow.rs`'s copy against `plan.rs`'s copy.

If `workflow.rs:48`'s string were changed to a non-matching value (trailing space removed, arrow changed, etc.), W4 would silently stop matching any decided item: `question.status.starts_with(QUEUE_FOLD_PREFIX)` would always be false, all questions would be skipped, and `w4_problems` would always return an empty vector. The tests that would catch this are only those expecting a non-empty problems list: `w4_flags_a_decided_item_at_the_boundary_without_a_receipt`. Tests that assert `problems.is_empty()` (zero-record case, matching receipt, before-boundary, non-decided items) would all pass even if W4 were completely disabled.

Why it matters: a future edit that changes one constant but not the other would silently disable W4 enforcement without any test failure on the "passes" cases. The comment is correct that `plan.rs`'s copy is pinned, but `workflow.rs`'s is not.

---

### F-3: `recommendation` prose implies `options` membership that the validator does not enforce

SEVERITY: low

`pack/instrument.md:9` describes `recommendation` as "the orchestrator's recommended option," wording that implies it is one of the items in `options`. The validator (`src/metrics.rs:366`, `require_str(obj, "recommendation")?`) validates only that the field is a present string; it does NOT check that the value is a member of `options`.

The prose explicitly states the `chosen in options` constraint only for `chosen`: "the human's choice, which must be one of `options`." It does not say `recommendation` carries the same constraint. But the phrase "recommended option" (as opposed to "recommendation string") implies to a reader that the value is one of the presented options.

Why it matters: an orchestrator that writes `recommendation: "some reasoning text"` (a sentence, not an option label) would pass validation, but a reader of `instrument.md` who follows the prose's implied shape would not expect this. The asymmetry between `chosen` (enforced) and `recommendation` (unenforced) is not explicitly stated. A single parenthetical noting "any string; not required to be a member of `options`" would close the ambiguity without requiring a code change.

---

## Clean confirmations

- Schema-doc consistency: `check_record`'s `"decision"` branch validates exactly the fields documented in `pack/instrument.md:9`, with the `chosen in options` cross-field constraint, common `task`/`ts` handling, and no field documented-but-not-checked or checked-but-not-documented.

- Drift guard extension: `"decision"` correctly added to the record-type array at `metrics.rs:863`; `q_id`, `options`, `recommendation`, `chosen` added to the field array at `metrics.rs:890-893`. The guard would fail if any of these names were removed from `instrument.md`.

- Decision validator test coverage (beyond F-1): valid record with optional `ts` passes; `chosen` not in `options` is rejected; each of the four required fields missing in isolation is reported; empty `options` is reported; non-string `options` element is reported.

- W4 test coverage: zero-record no-op returns nothing; decided item with matching receipt passes; decided item at or after boundary with no receipt is flagged and the message contains the item id and "has no matching `type:\"decision\"` receipt"; decided item before boundary is not flagged; non-decided (open) item at or after boundary is ignored. The five tests combine to cover the boundary semantics correctly. The boundary-is-always-at-least-one-receipt invariant means the `<` vs `<=` edge case is not reachable in practice.

- `parse_decisions` test: confirms well-formed decisions are projected to `(line, q_id)`, a non-decision record is skipped, a decision missing `q_id` is skipped, and bad JSON is skipped.

- Self-scaffold regeneration: `AGENTS.md` and `.agents/AGENTS.reference.md` each gained exactly one line, byte-identical to the line added in `pack/instrument.md`. The default (no `--instrument`) scaffold is unaffected (the instrument block is conditional; the test `instrument_off_omits_the_block_and_on_includes_it` pins this).

- CHANGELOG: entry is under `## [Unreleased]` -> `### Added`, accurately describes both the record schema and the W4 check, is forward-looking, and is consistent in style with neighbouring entries. No prohibited characters.

- Naming conventions: `Decision`/`parse_decisions`/`w4_problems` are parallel to `Round`/`parse_rounds`/`w3_problems`. Field name `q_id` is consistent with the plan's `Question.id` and with the existing codebase's kebab-to-snake naming. Message wording in `w4_problems` follows the W3 message style.

- `src/plan.rs` and `pack/plan-template.md` are byte-identical (confirmed by the diff stat: no changes to those files).
