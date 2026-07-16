# Triage: `risk_class` + `reviewers` round metrics (commit f57c87f)

Triager: independent of producer (orchestrator) and both reviewers. Adjudicated against `.agents/principles.toml` and the artifact (`git show f57c87f`, current `src/metrics.rs`, `pack/instrument.md`).

Summary: 3 valid, 2 dismissed (from 8 raw findings across two reviewers, deduplicated to 5 distinct issues).

Dedup map:

- Opus F1 (empty array) == Sonnet F3 -> V-2.
- Sonnet F1 (over-count) + Sonnet F2 (attribution ambiguity) share one root and one fix -> V-1.
- Sonnet F4 (non-object test gap) -> V-3.
- Sonnet F5 (dead missing-field branch) -> D-1.
- Opus F2 (hand-maintained field list) + Sonnet F6 (`valid_findings` substring not distinctly guarded) share one root -> D-2.

---

## V-1: `reviewer.valid_findings` counting rule is undefined (over-count + attribution ambiguity)

- Source: Sonnet F1 (medium) + Sonnet F2 (medium), merged. Same root cause, same fix.
- Verdict: VALID.
- Final severity: medium.
- Owner: orchestrator (producer applies the fix).
- Why valid: The schema and prose never define what `reviewer.valid_findings` counts when a finding is raised by more than one reviewer. That single gap produces both reported symptoms: `sum(reviewer[i].valid_findings)` can exceed the deduplicated `round.valid_findings` (F1), and the orchestrator has no rule for which reviewer gets credit for a shared valid finding (F2), so an LLM writing the log by hand will guess inconsistently across tasks. Since the field exists only to calibrate per-reviewer productivity, undefined semantics directly degrade the data it is meant to collect. This engages `make-documentation-self-contained` (415), `document-the-why-not-the-what` (410), `make-common-case-easy` (180), and `one-source-of-truth` (230). Medium (not higher): optional calibration-only field on an internal metrics log, no correctness or user impact; medium (not low): the ambiguity undermines the field's stated purpose, not just a reader's convenience, and both reviewers converged here.
- Recommended action: documentation, not a schema change. In `pack/instrument.md` line 5 (and the parallel text in `AGENTS.md` and `.agents/AGENTS.reference.md` kept in lockstep), define the counts as per-reviewer against that reviewer's own findings: `raw_findings` and `valid_findings` count that reviewer's own raised findings and its own triager-confirmed-valid findings, so a finding raised by two reviewers and judged valid counts in each reviewer's `valid_findings` and once at round level. State the consequence explicitly: the per-reviewer `valid_findings` may sum to more than `round.valid_findings`, which is deduplicated. This picks the "credit both" semantics, which needs no tie-break (easy for an LLM to populate), matches how `raw_findings` already double-counts shared findings, and makes the over-count the documented expected relationship rather than a skew. Reject a schema change (e.g. an overlap field): YAGNI (340) / KISS (350) for a calibration log.

## V-2: present-but-empty `reviewers: []` is accepted

- Source: Opus F1 (low) + Sonnet F3 (low), same finding.
- Verdict: VALID.
- Final severity: low.
- Owner: orchestrator.
- Why valid: A present `reviewers` array with zero elements passes the validator (the `for` loop body never runs; `src/metrics.rs:212-225`). The prose defines the array as "one object per reviewer that examined the artifact this round", and a round always has at least one reviewer, so a present empty array cannot represent a real round; it is a data error the validator does not catch. Rejecting it aligns with `make-illegal-states-unrepresentable` (190), `fail-fast-and-loudly` (140), and `parse-dont-validate` (200). The `severities: []` precedent for permissiveness does not transfer: a clean round legitimately has zero severities, whereas a round with zero reviewers is not a round. Absence stays expressible because the field is optional (omit it when there is no reviewer data). Low: post-hoc consistency check on an optional field, not a runtime path.
- Recommended action: small validator change. In `require_reviewers` (`src/metrics.rs:204-227`), after the `as_array` check, reject an empty array, e.g. `if array.is_empty() { return Err(format!("field \`{name}\` is empty")); }`. Add one test asserting the message. Keeps the boundary tight and steers the hand-written log toward a correct shape.

## V-3: no test for a non-object element in the `reviewers` array

- Source: Sonnet F4 (low).
- Verdict: VALID.
- Final severity: low.
- Owner: orchestrator.
- Why valid: The non-object-element path (`src/metrics.rs:213-215`, `element.as_object().ok_or_else(...)`) is real behavior (Opus verified `reviewers:[42]` yields `field \`reviewers\`[0] has wrong type (expected object)`), but no test locks that message, while the four sibling error paths added in this commit are each tested. This is the one uncovered combination. Adding it satisfies `tests-must-exercise-code` (130) and matches the module's existing regression-test discipline. Low: a coverage gap on already-correct code, not a defect.
- Recommended action: add one test in the tests section (alongside lines 445-484) feeding `"reviewers":[42]` (or a string element) and asserting `field \`reviewers\`[0] has wrong type (expected object)`.

## D-1: "dead" missing-field branch in `require_reviewers`

- Source: Sonnet F5 (low).
- Verdict: INVALID / WONTFIX.
- Owner: n/a.
- Why dismissed: `obj.get(name).ok_or_else(|| "missing field ...")` at `src/metrics.rs:208` cannot fire the error because the sole caller guards with `if obj.contains_key("reviewers")` (line 263). But this is a deliberate signature-uniformity choice, not a defect: `require_reviewers` takes `(obj, name)` and does its own lookup exactly like `require_severities`, `require_count`, and `require_str`, so all the `require_*` checkers compose the same way. Sonnet itself notes it "does not affect correctness." The "why" the reviewer asks for is already in the function doc comment ("the caller only invokes it when the field is present, since it is optional", lines 202-203). Making the branch total by passing the value in would break that uniformity for no real gain and pull against `match-existing-conventions` (110). The "silent panic if the guard were removed" hypothetical is a future refactor concern, not a current defect (`no-silent-scope-expansion` (80), YAGNI (340)). No action.

## D-2: drift-guard field-name list is hand-maintained / `valid_findings` not distinctly guarded

- Source: Opus F2 (low) + Sonnet F6 (low), same root (substring-based, hand-maintained guard).
- Verdict: INVALID / WONTFIX (pre-existing, out of scope for this commit).
- Owner: n/a.
- Why dismissed: Both facets are pre-existing properties of the substring-and-hardcoded-list guard, not introduced or worsened here; Opus explicitly rates "none required for this change." For this commit the guard demonstrably covers the new schema: `raw_findings` is a new field name exclusive to the reviewer block, so its substring check does protect the reviewer description (removing that block would drop `raw_findings` and fail the test), and `RiskClass` is added to the VARIANTS-driven enum half, which is genuinely drift-proof. Sonnet's point that `valid_findings` alone is satisfied by the round-level mention is true but is exactly why `raw_findings` (reviewer-exclusive) is the effective guard for that block; the residual risk is theoretical. The general fix Opus floats (derive the field list from one declarative schema so both halves auto-check) is a redesign of the drift guard, which is scope expansion beyond this two-field change (`no-silent-scope-expansion` (80), `prefer-duplication-over-wrong-abstraction` (360), KISS (350)). If the team later wants the schema-derived guard, that is its own task. No action here.
