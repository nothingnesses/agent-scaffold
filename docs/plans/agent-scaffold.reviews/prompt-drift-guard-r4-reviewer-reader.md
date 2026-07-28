# Step 92 `prompt-drift-guard`: work review round 4, THE COLD READER lens

Artifact: `src/agents_md_drift.rs` at `90b1527` ("docs: delete two false comment claims and correct two tokens").
Worktree: `.claude/worktrees/rev4-pdg-reader`, detached at `90b1527`. Every mutation below was reverted with the Edit tool, and every file I created was removed by path; `git status --porcelain` is empty and `git diff` is empty at the end. `cargo test` on the final tree: 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed. I ran no `nix fmt` and no `just fmt`, and edited no source, plan, or pack file as a deliverable.

Method, as briefed. I read `.agents/prompts/reviewer.md` and the Workflow and Principles sections of `AGENTS.md`, then extracted ONLY the comment lines of `src/agents_md_drift.rs` (`grep -n -E '^\s*(//|/\*|\*)'`) and read them with no code in view. I wrote 24 beliefs to a scratch file BEFORE opening any code, then read the code and tested each belief, mostly by mutation. I read `docs/plans/agent-scaffold.reviews/` LAST, after every belief was fixed and every mutation run.

## Part 1: the beliefs I formed from the comments alone

Recorded before reading a line of code, so the results below are data rather than hindsight.

- B1. Exactly three drift comparisons exist; every other test in the file only exercises helpers.
- B2. Both sides of all three comparisons pass `assert_no_unprotected_construct`, then `normalize_wrapping`, then equality.
- B3. The pinned render config equals `scaffold --principles default --instrument`: built-in pack, default principles, `Summary` detail, no `--var`, no `--module`, matching the justfile recipe.
- B4. `PROMPT_DEST_PREFIX` is `.agents/prompts/` (never stated literally in the comments; inferred from R1).
- B5. Editing `pack/prompts/<role>.md` without regenerating FAILS.
- B6. Hand editing a deployed copy (`.agents/prompts/*.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`) FAILS.
- B7. Adding a new prompt: a file in `pack/prompts/` with no manifest row fails nothing and ships nothing; an `[[asset]]` row under the prefix is guarded with no edit to this file, and panics until `just scaffold-self` creates the copy.
- B8. Deleting: the committed copy removed while the row remains panics; the row removed while the copy remains is R1 and stays green in this module.
- B9. A pure reflow of a guarded file passes; a reformat that introduces indentation or a multi-space run trips the precondition loudly.
- B10. R1 lets through any committed file under the prefix the pinned render does not emit, reached the four ways it lists.
- B11. R2 lets through a masked content change inside a line-structured construct `is_hard_start` misses, a raw HTML block being the known instance.
- B12. `.agents/prompts/checks-reviewer.md` does NOT exist in the repo.
- B13. Its `pack/pack.toml` row IS module-gated.
- B14. The complement Markdown copies already satisfy the precondition, so widening check 3's filter to them is a small change.
- B15. `.agents/principles.toml` carries lines outside canonical whitespace form, including indented multi-line array continuations.
- B16. `isolation_policy.rs` and `workflow_spec.rs` each pin exactly ONE generated slot with `.contains()`.
- B17. The raw render is byte-identical to both committed files today.
- B18. No guarded file carries an R2 construct today.
- B19. Fence tracking uses the same rule in both functions: `trim_start()` beginning with ``` or ~~~.
- B20. `is_hard_start` recognises ATX heading, `- `/`* `/`+ `, ordered `N.`/`N)` plus space, `>`, `|`, and a thematic break.
- B21. Checks 1 and 2 embed the committed side with `include_str!`; check 3 reads the working tree via `CARGO_MANIFEST_DIR`.
- B22. The non-vacuity assertion catches only a total collapse of check 3's filter.
- B23. Nothing outside the COVERAGE block states or contradicts coverage.
- B24. The pipeline is render then `nix fmt`, whose prettier uses `proseWrap=never`.

## Part 2: which beliefs held

HELD, verified against the code and by execution: B1, B2, B3, B4, B5, B6, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B24. B7 held for the case I tested first and FAILED for a second case; that gap is `RD4-1`. B23 held in the "contradicts" half and failed in the "states" half; I do not raise it, for the reason recorded under Checked and NOT raised.

Evidence for the load-bearing ones, each mutation applied alone to a clean tree and reverted with the Edit tool:

- B5, by mutation. Inserting `RD4MUTATION.` into the first paragraph of `pack/prompts/reviewer.md`: `cargo test --bin agent-scaffold agents_md_drift` -> `the_committed_role_prompts_match_a_fresh_render ... FAILED`, 5 passed 1 failed. So a `pack/prompts/` edit without regeneration is caught, and the `include_dir!` rebuild question a reader might worry about does not bite (`build.rs:22` tracks the pack directory).
- B6, by mutation, both sides. The same insertion into `.agents/prompts/reviewer.md` -> `the_committed_role_prompts_match_a_fresh_render ... FAILED`. Inserting `RD4MUTATION` into `AGENTS.md`'s principle 1 -> `the_committed_scaffold_matches_a_fresh_render ... FAILED`, so the `include_str!` side rebuilds and fires too.
- B7 first case, by mutation. Adding `[[asset]] source = "prompts/rd4-new.md" dest = ".agents/prompts/rd4-new.md" ownership = "reference"` plus the source file, without regenerating: `panicked at src/agents_md_drift.rs:162:13: failed to read the committed .agents/prompts/rd4-new.md ... run 'just scaffold-self'`. Self-extension with no edit to this file, exactly as claimed.
- B8, by mutation, both directions. Deleting the `prompts/triager.md` asset row from `pack/pack.toml` while `.agents/prompts/triager.md` stays committed: the five `agents_md_drift` tests pass and the full suite fails only `manifest::tests::builtin_manifest_lists_the_expected_assets`, so the orphan really is invisible to check 3 as `:71-76` says. Hand-placing `.agents/prompts/rd4-stale-extra.md` with no asset row: full suite 367 passed, 0 failed, completely silent.
- B9, by mutation. Splitting the first paragraph of `.agents/prompts/triager.md` across two lines: 5 passed, 0 failed. Reflow is tolerated.
- B11 and B18, through the module's own helpers (temporary probe, reverted with the Edit tool). R2 mechanics reproduce exactly: `precondition_rejects("# T\n\n<div>\n<span>alpha</span>\n</div>\n") = false`, `precondition_rejects("# T\n\n<div>\n<span>alpha</span> </div>\n") = false`, and the two normalize EQUAL. R2 is latent as claimed: over all nine guarded texts (the two renders plus the seven prompts), zero non-fenced non-hard-start lines begin with `<`, are a setext `===` underline, or are a link-reference definition.
- B14, through the module's own predicate (same probe). All sixteen complement Markdown assets pass `assert_no_unprotected_construct` on BOTH the rendered and the committed side, and all sixteen are normalized-equal today: the nine `docs/plans/TEMPLATE.*` sidecars, `.agents/LEDGER.template.md`, and the six `.agents/user-prompts/*.md`. So `:63-65`'s "already satisfy the precondition" is true, and the stated asymmetry with the TOML copies is true as well: `.agents/principles.toml` and `docs/plans/TEMPLATE.plan.toml` are both REJECTED by the predicate on both sides.
- B17, through the module's own helpers (same probe). `AGENTS.md`, `.agents/AGENTS.reference.md`, `.agents/prompts/reviewer.md`, and `.agents/prompts/orchestrator.md` are each byte-identical between the raw render and the committed copy. The inherited "Empirically ..." paragraph at `:27-32` and the present-tense restatement at `:463` are both factually TRUE today. Recorded because I was told to raise either if wrong; neither is.
- B3, at `src/agents_md_drift.rs:135-142` against `justfile:47` and `src/main.rs:402` (`#[arg(long, value_enum, default_value_t = Detail::Summary)]`), so "the default `Summary` detail" is accurate rather than merely asserted, and `src/main.rs:245-252` confirms the positional `true` is `instrument` and `&[]` is `modules`.
- B16, at `src/isolation_policy.rs:78,82` and `src/workflow_spec.rs:241,245`: one fragment each, `.contains()` against both committed files. B24, at `justfile:47-48` and `.prettierrc.json:2`.

## Findings

ONE finding, severity `low`. No `medium`, no `high`, no `critical`. Nothing in the mechanism: every behavioural probe I ran matched what the code should do, and I found no reachable false negative.

### `RD4-1`: the GUARDED SET's self-extension sentence promises coverage for a class of row the guard does not cover, and the standing counterexample is in the same block

SEVERITY: `low`. Doc-only. NOT a re-raise of `R1`, which I accept as recorded and for which I ask no mechanism change, no exclusion, and no new test.

WHERE. `src/agents_md_drift.rs:50-53`:

    //! `normalize_wrapping`. Check 3 is a filter over a rendered set, not a directory listing
    //! and not a hand-written list, which is what makes it self-extending: an `[[asset]]` row
    //! added to `pack/pack.toml` whose `dest` falls under the prefix is guarded with no edit
    //! here.

THE BELIEF A READER FORMS, and I formed it before reading any code. Adding an `[[asset]]` row to `pack/pack.toml` with a `dest` under `.agents/prompts/` is sufficient to bring that asset into the guarded set. My recorded B7 carries no module qualifier, and neither did round 3's cold reader's B7 (`prompt-drift-guard-r3-reviewer-reader.md:18`), which was tested only on the non-module-tagged path and recorded as held. Two independent cold readers took the sentence as unconditional.

WHAT THE CODE DOES. The sufficient condition is a row that the PINNED render emits, and a module-tagged row is not emitted (`self_scaffold_assets` passes `&[]` for modules, `src/agents_md_drift.rs:140`). A module-tagged row under the prefix therefore gets no coverage at all, and the counterexample is standing, in this repo, named twenty-six lines below the sentence at `:79-82`: `.agents/prompts/checks-reviewer.md`, whose row is `pack/pack.toml:219-223` with `module = "checks"` and a `dest` under the prefix.

RUNNABLE DEMONSTRATION (created, run, removed by path). I placed a committed copy at `.agents/prompts/checks-reviewer.md` whose content shares nothing with `pack/prompts/checks-reviewer.md`, that is, a maximally drifted copy of a prompt whose `[[asset]]` row exists in `pack/pack.toml` with a `dest` under the prefix:

    cargo test --bin agent-scaffold
    -> test result: ok. 367 passed; 0 failed

Silent. Not merely undetected by check 3: undetected by the whole suite, unlike the `pack.toml` routes in `R1`, which trip `builtin_manifest_lists_the_expected_assets`. This is the reachability round 2 already established for a different sentence (`prompt-drift-guard-r2-triage.md:121`): the repo ALREADY commits files at module-gated destinations (`.agents/checks.toml`, `.agents/hooks/pre-commit`), and one `scaffold --module checks --write --force` run puts this exact file into the tree by the same route.

WHY IT IS A FINDING RATHER THAN A QUIBBLE. It is the species this consolidation exists to end, in the one place the file designates as authoritative: the block's own charter is "Write a coverage claim here or not at all" (`:38`), and every other site now cites rather than restates. The wrong ACTION is concrete: a maintainer adding a role prompt for a future module, or bringing `checks-reviewer.md` into the tree, reads `:50-53`, concludes the drift guard picks it up for free, and adds nothing. Under `:38`'s own rule the reader is entitled to stop at the GUARDED SET.

WHAT IS TRUE ABOUT THE SENTENCE, stated plainly because it bears on severity. The first clause, "a filter over a rendered set", is exactly right and carries the qualification for a reader who propagates it into the second clause. `R1` at `:74-75` then states the fact outright ("the pinned config selects no modules, so a tagged row is not rendered"). So the file contains everything needed; the defect is that the load-bearing sentence states the conclusion without the condition its own premise supplies, and `R1` frames module-tagging as a way to LOSE coverage rather than as a case that never had it.

WHY `low` AND NOT `medium`. I considered `medium`, because this is an overclaim in the dangerous direction inside the authoritative block, and I decline it for consistency with this step's settled precedent: round 3's `RD-2` was an overclaim in the same direction, at a site a prior round had already required fixed, and both the reviewer and the triager rated it `low` (`prompt-drift-guard-r3-triage.md:114`). The reasons given there hold here: nothing misbehaves, no drift is masked today (no committed copy of a module-gated prompt exists, verified: `git ls-files .agents/prompts/` returns seven files, none of them `checks-reviewer.md`), the corrected statement is in the same file two paragraphs down, and the fix is comment-only.

WHAT A FIX MUST ACHIEVE, and it can be done by deletion or by two words, consistent with round 3's deletion-only constraint. Either drop the sentence's second half, since the first half ("a filter over a rendered set") already states the property and the `include_str!` sentence that follows reads correctly without it, or qualify the row: "a non-module-tagged `[[asset]]` row added to `pack/pack.toml` whose `dest` falls under the prefix". Do NOT author a new explanation of module gating; `:74-75` already carries it.

## Checked and NOT raised

Recorded so the triager sees the negative results and a later round does not re-derive them.

- THE `:101` MARKER, THE "STATES" HALF OF B23. `:423-426` ("Two-way in CONTENT ... One-way in SET MEMBERSHIP, which is residual R1") is past the "End of COVERAGE" marker and does state what check 3 catches, which is closer to restating than to citing. I do not raise it: every statement in it is TRUE (I reproduced both directions by mutation, B5 and B6 above), it cites `R1` rather than re-deriving it, and it creates no wrong belief, since an equality comparison is two-way in content by construction. Round 3's cold reader reached the identical negative result on the identical text (`prompt-drift-guard-r3-reviewer-reader.md:183`), and I have no new evidence against it. Re-raising it would be relitigation.
- THE "CONTRADICTS" HALF OF B23 HOLDS. I read every comment line past `:101` against the block. `:377-383`, `:387-391`, `:417-426`, `:444-447`, and `:461-464` are all consistent with it; the loudness claim round 3 raised as `RD-2` is gone, and `:387-391` now stops at "Asserted on both sides."
- ROUND 3'S FOUR FIXES ALL LANDED, verified at the sites: the "the suite stays green" clause is absent from `R1` (`:71-84`), `:80` now names `pack/pack.toml` and that is where the row is (`pack/pack.toml:219-223`), `:387-391` no longer promises loudness, and `:302` reads "normalize equal only when". The expected exemption is present and is a logical connective in a mechanical statement, not a coverage claim, exactly as the brief describes.
- `R1`'S FOUR ROUTES ARE NOW STATED CORRECTLY. The sentence claims only invisibility to check 3, and that is true of all four; I reproduced the row-deletion route and the hand-placed-file route above. The suite-colour difference between them (three routes trip `manifest::tests::builtin_manifest_lists_the_expected_assets`, the hand-placed file trips nothing) is no longer claimed either way, which is what round 3's triage asked for.
- THE `checks-reviewer` IMPLICIT EXCLUSION is sound and I do not contest it; settled, and my finding is about a different sentence.
- THE THEMATIC-BREAK CLAUSE at `:255-256` is still inaccurate and is settled out of scope and backlogged. Not raised.
- THE JUSTFILE CONFIG DUPLICATION. `:129` quotes `scaffold --principles default --instrument` where `justfile:47` also passes `--output-dir . --write --force`; those change no rendered byte. Settled, not raised.
- THE NON-VACUITY MESSAGE at `:437` says the render "dropped no asset under" the prefix, where "dropped" means emitted. Momentarily ambiguous, but it is an assertion message rather than a comment, it fires only in the collapse case, and no wrong belief follows. Too thin to raise.
- I FOUND NOTHING MISSING that a maintainer needs. The five brief questions are all answerable from the comments alone, and four of the five are answerable CORRECTLY (see the direct answer).

## Direct answer to the brief

WOULD A COMPETENT DEVELOPER, READING ONLY THESE COMMENTS, BE CORRECTLY INFORMED? NO, but by one sentence, and the block is markedly better than round 3 found it.

Answering the brief's questions from the comments alone: edit a file under `pack/prompts/`, something fails, CORRECT, and the comments also let me predict the one exception (a file with no manifest row, `:77-79`). Edit a deployed copy, it fails, CORRECT. Add a new prompt, CORRECT for a core row and WRONG for a module-tagged one, which is `RD4-1`. Delete one, CORRECT in both directions, and this is the question round 3 answered wrongly, so the fix landed. Reformat one, CORRECT, including the loud-failure case. I could state both residuals from the comments alone and both statements were right, and for any change I made I could tell whether it fell into one; that was not true in round 3 for `R1`.

The consolidation is doing its job. The guarded set is defined by naming the filter, and I found no gap between that definition and the code. The single remaining gap is that the sentence explaining WHY the filter is self-extending states its conclusion without the condition its own premise supplies, and a standing counterexample sits in the same block.

## Tree state

`git status --porcelain` shows only this untracked findings file. `git diff` is empty. HEAD is `90b1527`. `cargo test` on the final tree: 379 passed, 0 failed across all six binaries. Every mutation (one `pack/prompts/reviewer.md` edit, two `.agents/prompts/reviewer.md` edits, one `AGENTS.md` edit, two `.agents/prompts/triager.md` edits, three `pack/pack.toml` edits, and two temporary probe tests in `src/agents_md_drift.rs`) was reverted with the Edit tool, and every file I created (`pack/prompts/rd4-new.md`, `.agents/prompts/rd4-stale-extra.md`, `.agents/prompts/checks-reviewer.md`) was removed by deleting the exact path added. I used no `git checkout` or `git restore`.
