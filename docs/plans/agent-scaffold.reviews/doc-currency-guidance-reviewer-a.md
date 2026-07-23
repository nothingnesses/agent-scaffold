# Fidelity review: doc-currency-guidance (reviewer A)

Lens: fidelity to the decided spec (Q-50). Does the change faithfully and completely encode the three required parts, without missing or weakening any part and without scope overreach?

Change reviewed: branch `plan/doc-currency-guidance`, commit `211c6c8`, base `b93056f`.
Files inspected in full context: `pack/AGENTS.md` (phases 2 and 5 plus surrounding conventions), `pack/prompts/planner.md`, `pack/prompts/implementer.md`, `pack/prompts/reviewer.md`, and the parallel edits to root `AGENTS.md` and `.agents/AGENTS.reference.md`.

## Verdict summary

The three required parts are all present and faithful. I found one low-severity internal-consistency note. No missing part, no weakened part, and the prompt additions are minimal and correctly weighted (not over-reaching). Details below, including the checks that came back clean.

## Findings

### 1. Acceptance paragraph embeds an implementer action into a phase described as review-only (low)

Evidence: `pack/AGENTS.md` phase 5 (line 33), new text: "Acceptance also runs a documentation-currency check: does the shipped change leave any doc or prompt stale? The implementer updates the docs and prompts the change made stale, and the reviewer verifies that currency as part of the acceptance review."

The same phase 5 paragraph states that acceptance "is a single reviewers-then-triager pass, not the consecutive-clean convergence loop," and that "each valid shortfall is a finding that goes back to planning (add or revise steps) or implementation." Acceptance is described as a review-only pass with no in-phase writer; fixes route back out to implementation. Inserting "The implementer updates the docs and prompts the change made stale" into that paragraph can read as the implementer acting during the acceptance pass, which is in mild tension with the "reviewers-then-triager pass" and "goes back to ... implementation" framing.

The more coherent reading is that the clause assigns ownership of the doc-update duty to the implementer (a standing phase-4 duty, matching `pack/prompts/implementer.md`), while only the reviewer's verification happens "as part of the acceptance review." Under that reading the chain connects: planner identifies stale docs (phase 2) -> implementer updates them (phase 4 duty) -> reviewer verifies currency at acceptance -> any staleness is a shortfall that routes back to implementation. The wording faithfully mirrors the decision text ("with the implementer updating and the reviewer verifying"), so this is a phrasing/placement nit rather than a substantive fidelity gap. Why it still matters: a reader could infer an in-phase implement step at acceptance that the surrounding text otherwise rules out. A small rewording (for example attaching the update duty to the implementer's implementation work rather than to the acceptance paragraph, and keeping only the reviewer-verifies clause in phase 5) would remove the ambiguity.

## Checks performed that came back clean (no finding)

- Part 1 (doc-impact assessment in the planner's duties, in both `pack/AGENTS.md` phase 2 and `pack/prompts/planner.md`): present and faithful. `pack/AGENTS.md` line 30: "When it folds a change into the plan, the planner also assesses that change's documentation impact: it identifies which docs and prompts the change will make stale, so keeping them current is planned work rather than an afterthought." `pack/prompts/planner.md`: "When you fold a change into the plan, assess its documentation impact: identify which docs and prompts the change will make stale, so keeping them current is planned work rather than an afterthought." Both match the decision's "identify which docs/prompts it makes stale" wording. The trigger "When it folds a change into the plan" is consistent with the existing folding concept in the "Human requests" section.
- Part 2 (the same duty in `pack/AGENTS.md` phase 2): satisfied by the same phase-2 sentence above.
- Part 3 (doc-currency check at acceptance, implementer updating, reviewer verifying): present in `pack/AGENTS.md` phase 5 (see finding 1 for the one caveat). Both the "does the shipped change leave any doc or prompt stale?" check and the implementer-updates / reviewer-verifies role split are encoded.
- Prompt additions to `pack/prompts/implementer.md` and `pack/prompts/reviewer.md` (allowed but not required): consistent and correctly weighted. Implementer line: "Keep documentation current with your change: update any docs and prompts your change makes stale, so the shipped change does not leave them out of date." Reviewer line: "Check documentation currency: verify the change does not leave any doc or prompt it touches stale, and report a stale doc or prompt as a finding like any other." Each is a single sentence framing the duty as ordinary work / an ordinary finding, not a heavy new obligation. The reviewer line is written generally (all review rounds) rather than only at acceptance, which is broader than the literally named acceptance step, but the decision made documentation currency "a CORE CROSS-CUTTING guidance," so cross-cutting application to every review round is in line with intent rather than scope overreach.
- No contradiction or duplication with existing `pack/AGENTS.md` conventions. The "Prose formatting" convention (line 108) and the "Format only your own files" file-safety rule (line 79) concern formatting and repo-wide formatters; the new implementer duty is content authoring of the change's own stale docs (within its scope), so it does not conflict. The new reviewer doc-currency line does not clash with the "line length is never a finding" rule (line 108 and `pack/prompts/reviewer.md`); it adds a distinct finding category.
- No unrelated restructuring or new rules beyond the decision. The four insertions are additive single sentences/paragraphs; no surrounding prose was rewritten.
- Parallel identical edits to root `AGENTS.md` and `.agents/AGENTS.reference.md` mirror the `pack/AGENTS.md` change exactly, keeping those copies current rather than introducing new content. This is consistent with the doc-currency principle itself and is not scope creep; the inserted text is identical across all three files.

## Chain-consistency conclusion

The producer-to-verifier chain connects with no orphaned duty or check without a producer: the planner identifies stale docs (phase 2), the implementer updates them (a standing implementer duty), and the reviewer verifies currency at acceptance (phase 5), with any residual staleness routing back to implementation as an ordinary shortfall. The only caveat is the phrasing nit in finding 1.
