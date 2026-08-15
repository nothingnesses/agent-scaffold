### `user-prompts-pointer-and-drift-coverage`: name `audit.md` in the shipped prompt list, and widen the drift guard to the `.agents/user-prompts/` copies

TWO ITEMS, HELD AS ONE STEP RATHER THAN TWO. Both concern the `.agents/user-prompts/` family, both are small, and both have a deterministic oracle, so holding them together costs one review loop instead of two (Minimal by default). They are otherwise independent and either can be dropped without the other. Both were found during the 0.0.3 release work, both were deliberately left alone there, and neither is fixed by the record you are reading.

### `A`: `audit.md` is not named in the shipped getting-started prompt list

`pack/AGENTS.md` opens with a getting-started paragraph that names the human-invoked prompts by filename, and a later paragraph that names the ones carrying state across a break. Between them they name six: `kickoff.md`, `explore.md` and `review.md` in the first, `pause.md`, `compaction-prep.md` and `resume.md` in the second. Measured on the tree: `audit.md` is named zero times in `pack/AGENTS.md`, zero times in the generated root `AGENTS.md`, and zero times in `.agents/AGENTS.reference.md`, while `kickoff.md` is named in all three. So a user whose only entry point is the shipped guidance cannot discover the seventh prompt from it. The README's scaffolded-layout block does list all seven, which is why acceptance criterion 5 of `audit-user-prompt` was met while this gap stayed open.

THIS DOES NOT RE-OPEN THE SELF-CONTAINED-VERSUS-ENTRY-MODE DECISION `audit-user-prompt` settled, and the distinction is the whole reason it is safe to do. That decision was about where the audit DISCIPLINES live, and it put them in the prompt so the shipped guidance would not grow a fifth mode paragraph. What is missing here is a POINTER, one clause naming a file, which carries no discipline into the guidance and adds no mode, no role and no phase.

THE COST, STATED SO IT IS WEIGHED RATHER THAN ASSUMED. `pack/AGENTS.md` is the source of the generated guidance, so editing it widens the regeneration set to the root `AGENTS.md` and `.agents/AGENTS.reference.md`, which the drift guard's checks 1 and 2 pin. The 2026-08-13 audit's recommendation 8 wants the shipped guidance cut rather than grown; one clause against that target is negligible, and the alternative of leaving the prompt undiscoverable from the guidance is the worse trade. Name the trade in the outcome either way.

ACCEPTANCE, each executable:

1. `audit.md` is named by filename in `pack/AGENTS.md`, in the same form the other six use.
2. The root `AGENTS.md` and `.agents/AGENTS.reference.md` are regenerated in the same change, and `cargo test` passes, which is what makes the drift guard's checks 1 and 2 assert the regeneration happened.
3. Nothing else in the guidance moves. No mode paragraph, no role and no phase is added.

### `B`: the `.agents/user-prompts/` copies sit outside the whole-file drift guard

PRE-EXISTING, and documented as a scope call in the module that owns it rather than discovered here. `src/agents_md_drift.rs` runs three comparisons. Checks 1 and 2 pin the committed root `AGENTS.md` and `.agents/AGENTS.reference.md` against a fresh render. Check 3 filters the rendered asset set by `PROMPT_DEST_PREFIX`, which is `.agents/prompts/`, so it covers the role prompts and nothing else. The module's COMPLEMENT rule states the consequence directly and names the `.agents/user-prompts/` copies as one of its illustrations. So nothing pins the committed `.agents/user-prompts/audit.md` against `pack/user-prompts/audit.md`, and the same holds for the other six copies: an edit to a committed copy, or a pack edit without a regeneration, ships a divergence that no gate reports.

THE MODULE HAS ALREADY PRICED THE FIX AND THE PRICE IS WHAT MAKES THIS WORTH DOING NOW. Its own rule records that widening to the Markdown asset copies is a small change to check 3's filter, because they are prose under the same prettier settings and already satisfy the `assert_no_unprotected_construct` precondition, whereas the `.toml` copies under `.agents/` would need a comparison of their own and are NOT in scope here. Widening the filter also inherits the module's accepted `R1` residual, which is that check 3 maps render to committed and asserts nothing in the other direction, so a committed copy the pinned render does not emit stays invisible to it. That residual is not this step's to close.

THE ALTERNATIVE IS TO LEAVE IT, and it is not unreasonable: the gap is documented where a maintainer meets it, and a user-prompt copy going stale ships worse guidance rather than worse behaviour. Take that option only by writing it down as a decision, because the module's rule already anticipates the widening and an undocumented no is indistinguishable from an oversight.

ACCEPTANCE, each executable:

1. Check 3 covers the `.agents/user-prompts/` copies as well as the `.agents/prompts/` ones, by widening the filter rather than by adding a hand-written list, so the check stays self-extending when the pack gains a prompt.
2. Red then green, demonstrated rather than asserted: hand-edit one committed `.agents/user-prompts/` copy, show the guard fails, revert, show it passes. The red state lands as evidence in the outcome.
3. The module doc's COVERAGE block is updated in the same change, since it currently states these copies as an illustration of what is NOT covered, and that sentence becomes false. The `.toml` copies stay out of scope and stay named as out of scope.
4. `cargo test` passes and no other guarded comparison changes.
