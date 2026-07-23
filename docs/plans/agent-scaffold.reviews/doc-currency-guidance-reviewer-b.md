# Mechanical and consistency review: doc-currency-guidance

Branch `plan/doc-currency-guidance`, commit `211c6c836ae34c1378391e04f2e3779c1fd59137`, based on main `b93056f4a58fa05dfdc86b63aa4b12ebcf121f4d`.

Lens: mechanical soundness and source-vs-generated consistency. I am read-only with respect to the product; this file is my only committed write.

## Findings

No valid findings. The change is mechanically sound and internally consistent across the source pack and the generated copies.

## What I checked

Scope. `git diff --stat b93056f..211c6c8` shows exactly the 9 intended files and nothing else:
`pack/AGENTS.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`, `pack/prompts/{planner,implementer,reviewer}.md`, and `.agents/prompts/{planner,implementer,reviewer}.md`. No plan.toml, ledger, metrics JSONL, Rust code, or TEMPLATE files were touched. No collateral change.

Source-vs-generated consistency. The pack is the source and carries template placeholders (`{{workflow_control}}`, `{{principles}}`, `{{instrument}}`, `{{modules}}`, `{{isolation_policy}}`), so a wholesale `diff` of `pack/AGENTS.md` against the generated `AGENTS.md` and `.agents/AGENTS.reference.md` differs only in those expanded blocks, as expected. The two doc-currency additions themselves are byte-identical across all three AGENTS files (verified by exact-string `grep` of the phase-2 sentence and the phase-5 sentences). The three prompt sidecars are IDENTICAL between `pack/prompts/*` and `.agents/prompts/*` (whole-file `diff` returned no differences). No source edit is missing from a generated copy and no generated copy diverges from source.

Placement. The phase-2 addition sits inside bullet `2. Plan.`, between the render-check sentence and "The planner also states the Success Criteria and resolves the open questions before implementation." The phase-5 addition sits inside bullet `5. Accept.`, between the backstop sentence and "If the triager confirms every criterion is met, the work is done." Neither bleeds into an adjacent phase and the numbered-list structure is intact. The three prompt additions are inserted as standalone paragraphs within their respective prompt bodies, not breaking surrounding structure.

Prose rules. A non-ASCII scan (`grep -nP "[^\x00-\x7F]"`) over all 9 files at commit `211c6c8` returned nothing: no em-dashes, en-dashes, unicode, or emoji. The additions use ASCII only; hyphens appear only in ordinary hyphenated compounds (`documentation-currency`). Each addition is a single unwrapped line, so no hard-wrap / manual mid-sentence line break was introduced. No characteristic AI filler phrasings ("it's worth noting", "seamless", "robust", "leverage", "delve", "surface" as a verb) appear in the added text.

Validation (run from this worktree with the branch tree checked out via `git checkout 211c6c8 -- pack .agents AGENTS.md`, under the project Nix toolchain):

- `cargo run -- render --check docs/plans/agent-scaffold.plan.toml`: `docs/plans/agent-scaffold.plan.toml: up to date`. (Expected, since the change does not touch the plan.toml view.)
- `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml`: `164 records, valid`; `74 steps, 62 questions, valid`.
- `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow`: same source/records valid, plus `workflow invariants hold`.
- `cargo test`: all suites passed, 0 failed (main library suite 342 passed; other suites 3, 2, 1, 1 passed).

After validation I restored the worktree with `git checkout HEAD -- pack .agents AGENTS.md`, leaving this findings file as the only change committed on branch `review/doc-currency-b`.
