# Reviewer findings: exploration-mode (Q-29), commit `9757a64`

Lens: correctness + consistency. Reviewer: opus.

## Summary

The change adds `exploring` to the Open-Questions queue vocabulary and documents a design-space-exploration workflow mode. The validator behaviour is correct, the drift guard extends automatically and stays correct, the rendered copies are in sync, and `just test` (95 passed) and `just clippy` are green. No other code path needs to know about `exploring`.

- critical: none.
- high: none.
- medium: none.
- low: 1 (prose consistency, non-blocking).

## Verification performed

- `just test`: 95 passed, 0 failed (`test result: ok. 95 passed`).
- `just clippy`: clean (`Finished dev profile`, no warnings).
- Rendered-copy sync via `diff` of `git show 9757a64:` blobs:
  - `pack/AGENTS.md` vs `AGENTS.md`: differs only by the expected `{{principles}}` and `{{instrument}}` placeholder expansion; the `exploring` paragraphs are byte-identical.
  - `pack/AGENTS.md` vs `.agents/AGENTS.reference.md`: same, expected placeholder expansion only.
  - `pack/plan-template.md` vs `docs/plans/TEMPLATE.md`: identical. `just scaffold-self` left them consistent.

## Detailed correctness review

### Validator behaviour is correct (no finding)

`QUEUE_EXACT_STATUSES` now holds `["open", "exploring", "superseded"]` (src/plan.rs:63). `question_status_ok` (src/plan.rs:328-330) accepts a status if it is an exact member or starts with `QUEUE_FOLD_PREFIX`. Consequences, all correct:

- An `exploring` item validates: `QUEUE_EXACT_STATUSES.contains("exploring")` is true.
- Near-misses are rejected: `exploringxyz` is not an exact member and does not start with the fold prefix, so `question_status_ok` returns false. The added test at src/plan.rs:649,652 asserts both directions (`exploring` accepted, `exploringxyz` rejected).
- `exploring` is exact-matched, so it never reaches the `else if ... strip_prefix(QUEUE_FOLD_PREFIX)` cross-reference branch (src/plan.rs:388-405). It requires no target slug and triggers no "folds into ... not a Roadmap step" error. Correct: `exploring` points at an exploration artifact, not a Roadmap slug, so it should not be slug-cross-referenced.

### Drift guard is still correct and now requires `exploring` to be documented (no finding)

`plan_template_documents_every_accepted_status` (src/plan.rs:716-746) iterates `QUEUE_EXACT_STATUSES` and asserts the template contains `` `<status>` `` (backtick-anchored, src/plan.rs:734-739). Because `exploring` is now in the set, the test now requires `` `exploring` `` in `pack/plan-template.md`. The template's item-format line includes `` `open`, `exploring`, `decided -> folded into <slug>`, or `superseded` ``, so the assertion is satisfied. The guard extends automatically as intended, and the backtick anchoring still catches a removed status.

### No other code path breaks (no finding)

- `parse_questions` (src/plan.rs:255) and `queue_structure_problems` (src/plan.rs:182-210) parse the `(<status>)` group generically; neither enumerates a hardcoded status vocabulary, so adding `exploring` needs no change there.
- The `status` command (`run_status`, src/main.rs:471-512) counts Roadmap steps by status (`by_status`, src/main.rs:502-507) but projects Open-Questions items only by count (`plan.open_questions.len()`, src/main.rs:512). It does not group questions by status, so the new value cannot break the `status`/`status --json` projection.
- `Question.status` is a free-form `String`; no exhaustive `match` on question status exists that a new variant could make non-exhaustive.

### Prose-vs-code consistency (no finding)

The template describes `exploring` as "a sub-state of `open`: a design-space exploration is owed before the item's options are decidable ... then moves the item to `open`". `pack/AGENTS.md` describes the same lifecycle (record as `exploring`, run the exploration, write the design-notes artifact, then move to `open`). This matches the code's treatment of `exploring` as an accepted exact status distinct from `open`, with no contradiction against the existing `decided -> folded into <slug>` or `superseded` semantics.

## Finding 1 (low): template item-format pointer wording does not mention the exploration artifact target

Evidence: `docs/plans/TEMPLATE.md` / `pack/plan-template.md`, Open-Questions item-format line describes each item as carrying "a pointer to the step or ledger that carries the detail". The new `pack/AGENTS.md` "Design explorations" paragraph says that while an item is `exploring` it "points at it [the exploration file under `docs/plans/<task>.explorations/`] by path". An exploration artifact is neither "the step" nor "the ledger", so the template's enumeration of pointer targets is now slightly narrower than the AGENTS.md prose for the `exploring` case.

Impact if unfixed: none on the validator (pointer targets are never validated, for any status; the `open` case is not validated either). Purely a documentation-precision nit: a reader following the template alone would not learn that an `exploring` item's pointer is the explorations artifact. Low severity, non-blocking. Noted for completeness; not a correctness defect.

## Conclusion

The change is correct and internally consistent. The validator accepts `exploring` and rejects its near-misses, the drift guard extends and stays correct, rendered copies are in sync, and the full suite plus clippy are green. One low-severity documentation-precision observation; no medium, high, or critical findings.
