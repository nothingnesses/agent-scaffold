# Review: `user-prompts-dir` (Q-16) - reviewer (opus)

Diff range reviewed: `9f52ef0..9c1a88d`. Lens: correctness of the mechanics.

Verification performed:
- `just test` -> 46 passed, 0 failed. The updated `builtin_manifest_lists_the_expected_assets` test passes.
- `diff pack/user-prompts/kickoff.md .agents/user-prompts/kickoff.md` -> IDENTICAL (verbatim copy, as expected for `ownership = "reference"` with no `render`).
- `just scaffold-self` -> exit 0, `git status --short` clean afterward (generated mirrors in sync, no drift; Principle 4).
- Non-ASCII scan of `pack/user-prompts/kickoff.md`, `.agents/user-prompts/kickoff.md`, `pack/pack.toml`, `README.md` -> no matches (ASCII-clean).
- Manifest order cross-check: `pack/pack.toml` asset order (orchestrator, planner, clarifying-questions, open-questions-gate, reviewer, triager, implementer, principles.toml, user-prompts/kickoff.md) matches the test's expected `dests` order exactly; `user-prompts/kickoff.md` is last in both, which is correct since assets load in manifest order.
- No golden/byte-identical sync test in `src/`/`tests/` references the asset list beyond the two manifest tests; `builtin_renders_only_the_rendered_assets` is unaffected (it only inspects `AGENTS.md`, `.agents/AGENTS.reference.md`, `.agents/principles.toml`), and there is no count assertion to update.

## R1 (low): `ownership = "reference"` is the correct choice; recording the justification

Location: `pack/pack.toml:66-69`.

Evidence: The kickoff asset uses `ownership = "reference"` (tool-owned, refreshed on every run, copied verbatim), matching the role prompts under `.agents/prompts/`. This is the right call, not a defect. The kickoff prompt is a thin trigger the human copies OUT and fills in when pasting (`pack/user-prompts/kickoff.md`: "Copy this, fill in the bracketed parts, and paste it"); the human does not edit the file in place. Keeping it `reference` means a later change to the kickoff format or the workflow it points at propagates on the next scaffold run and cannot drift from `AGENTS.md` (Principle 1, single source of truth; Principle 4, no drift). `working` ownership would create-once-then-leave-alone, freezing a potentially stale trigger.

Residual (informational only): as with every `reference` asset, a human who did edit the file in place would have those edits clobbered on rerun. That is by design and consistent with the README's stated contract (README.md:50-52, ".agents/ assets are tool-owned and refreshed on every run"), so it is not a finding against this step. The plan's "editable kickoff prompt" wording (plan lines 306, 478) refers to the human editing their pasted copy, not the file, and is consistent with `reference`.

## Other checks - no findings

- Severity critical: none.
- Severity high: none.
- Severity medium: none.
- pack.toml `[[asset]]` entry: `source = "user-prompts/kickoff.md"`, `dest = ".agents/user-prompts/kickoff.md"`, `ownership = "reference"`, no `render` key (correct: verbatim prompt, not a rendered template). Comment is accurate and ASCII-clean.
- Asset drops correctly: `.agents/user-prompts/kickoff.md` exists and is byte-identical to the source; the loader created the nested `user-prompts/` directory.
- README.md:45-46: the layout block names `user-prompts/` and `kickoff.md` with indentation and description style consistent with the surrounding tree (two-space nesting under `.agents/`, four-space for the leaf, aligned description column).
- Scope: this step correctly does NOT add the AGENTS.md "Getting started" pointer (that is the separate `human-onboarding` step) and does NOT add the compaction-prep/resume prompt (separate `compaction-prep` step). No out-of-scope changes.
- Principle 3 (safe on existing): adding a namespaced asset under `.agents/`; `reference` refresh semantics are the established, safe behavior for tool-owned files.
- Principle 5: the asset is encoded as manifest data through the single existing load path, no special-casing.
