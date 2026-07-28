# Step 92 `prompt-drift-guard`: reviewer findings (false-negative lens)

Artifact: `git diff 852a8c4..8012e05`, a single-file change to `src/agents_md_drift.rs`.
Lens: can this guard PASS when it should FAIL?
Worktree: `.claude/worktrees/rev-pdg-falseneg`. Every mutation below was reverted; the tree is clean and `cargo test` is green (367 + 5 + 1 + 3 + 1 + 2 passed, 0 failed).

Summary: no reachable false negative was found in the guard's comparison for the current file set. The three findings below are a documentation overclaim, an incomplete fail-safe that is latent today, and an accuracy note on the accepted residual `H4-3`. All are `low`. There are no `medium`, `high`, or `critical` findings.

## FN-1: the doc claims a prompt added to the pack is guarded automatically; only a prompt added to `pack.toml` is

Severity: low.

`src/agents_md_drift.rs:36-38` states:

> every rendered asset whose `dest` starts with `.agents/prompts/` is compared against the committed file read from `CARGO_MANIFEST_DIR`, so a prompt added to the pack is guarded without editing this file.

The guarded set is derived from the RENDER, which is derived from `pack/pack.toml`'s `[[asset]]` rows, not from the `pack/prompts/` directory listing. A prompt file added to the pack directory with no `[[asset]]` row is not rendered, is never deployed, and no test in the repo notices. Nothing enumerates `pack/prompts/` (the only directory-level machinery over the pack is `build.rs`'s rebuild tracking and `include_dir!` at `src/manifest.rs:29`, neither of which asserts registration).

Evidence (mutation, run and reverted):

    $ cat > pack/prompts/experimental.md <<'EOF'
    # Experimental

    A new role prompt added to the pack directory without a `[[asset]]` row in `pack.toml`.
    EOF
    $ cargo test
    test result: ok. 367 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
    test result: ok. 5 passed; ...
    test result: ok. 1 passed; ...
    test result: ok. 3 passed; ...
    test result: ok. 1 passed; ...
    test result: ok. 2 passed; ...

The whole suite is green with an unregistered prompt sitting in the pack. Contrast the registered case, which the guard does catch loudly: `committed_asset` (`src/agents_md_drift.rs:110-118`) panics with a "run `just scaffold-self`" message when a rendered prompt has no committed copy.

Impact if left unfixed: a reader of the module doc concludes the derived set closes the "adding a prompt goes unguarded" hole completely. It closes it only for prompts registered in the manifest. The residual failure (a new role prompt that silently never ships) is loud in practice, since the role has no deployed prompt at all, which is why this is `low` and a doc fix rather than a code fix: narrow the claim to "a prompt added to the pack MANIFEST".

## FN-2: the precondition fail-safe does not cover a prettier-verbatim raw HTML block, so the join can still mask real drift

Severity: low.

`src/agents_md_drift.rs:269-279` enumerates the constructs `normalize_wrapping` can mask and states that `assert_no_unprotected_construct` "pins that precondition ... and fails LOUDLY the day guidance gains one of these". The new prompt loop repeats the same enumeration for the prompts at `src/agents_md_drift.rs:410-413` ("a nested list, indented code, or a multi-space inline span"). There is a fourth construct that the precondition does NOT trip on and that the normalization still collapses: a raw HTML block. `normalize_wrapping` exempts only fenced code (``` / ~~~); an HTML block's lines are not hard starts (`is_hard_start`, `src/agents_md_drift.rs:187-215`), so they are joined into one logical line, while every line is in canonical whitespace form and the precondition accepts it.

This matters because prettier keeps raw HTML blocks VERBATIM, so unlike the heading case (see the attacked-avenues list below) the multi-line form is a stable fixed point of `nix fmt` and can legitimately live in a guarded file.

Evidence (temporary test added to `src/agents_md_drift.rs`, run, reverted):

    #[test]
    fn temp_probe_raw_html_block_masking() {
        let multi = "# T\n\n<pre>\nline one\nline two\n</pre>\n";
        let single = "# T\n\n<pre> line one line two </pre>\n";
        println!("precondition rejects multi: {}", precondition_rejects(multi));
        println!("precondition rejects single: {}", precondition_rejects(single));
        println!("normalized multi:  {:?}", normalize_wrapping(multi));
        println!("normalized single: {:?}", normalize_wrapping(single));
        assert!(!precondition_rejects(multi), "the precondition does NOT trip on the block form");
        assert!(!precondition_rejects(single), "the precondition does NOT trip on the joined form");
        assert_eq!(normalize_wrapping(multi), normalize_wrapping(single), "MASKED");
    }

    $ cargo test temp_probe_raw_html_block_masking -- --nocapture
    precondition rejects multi: false
    precondition rejects single: false
    normalized multi:  "# T\n\n<pre> line one line two </pre>"
    normalized single: "# T\n\n<pre> line one line two </pre>"
    test agents_md_drift::tests::temp_probe_raw_html_block_masking ... ok

Prettier's verbatim treatment of the block form, which is what makes it reachable (run in a scratch directory outside the repo, with `proseWrap: never`):

    input:                       prettier output:
    <div>                        <div>
    raw html line                raw html line
    </div>                       </div>

Why this is `low` and not higher: no guarded file contains an HTML block today (`grep` for `<` structures in `.agents/prompts/` and `pack/prompts/` finds none), so nothing is masked right now, and for most HTML (`<details>`, `<summary>`, `<div>`) joining the lines is semantically inert. It becomes a real masked drift only for whitespace-significant HTML such as `<pre>`. It is nonetheless the same LATENT class as round 1's `F1` (a mid-line tab or NBSP that `contains("  ")` missed), which was treated as a real finding and fixed rather than accepted, so the triager may consider that precedent binding and escalate. The cheap fix in the same spirit as the existing precondition: reject a non-hard-start, non-blank line that directly follows a non-blank line, which is exactly the shape of every construct the join can collapse and which no prettier-clean guarded file contains today (verified below).

Note this construct is inherited from step 80 and applies to `AGENTS.md` and `.agents/AGENTS.reference.md` as well; step 92 extends the same fail-safe to seven more files and restates the incomplete enumeration at `:410-413`, which is why it is raised here.

## FN-3: `H4-3`'s recorded description names one of several mechanisms that reach the same residual

Severity: low. This is the accuracy note the brief asked for on the accepted residual, not a new finding, and the residual itself is not reopened.

`docs/plans/agent-scaffold.ledger.md:353` records `H4-3` as: "the derived-from-manifest guard would silently drop a prompt REMOVED from the pack, where the enumerated form panics; an unstated limitation of a choice judged correct, costing one orphaned prose file."

That is accurate as far as it goes: removing the `[[asset]]` row drops the prompt from the derived set while its committed copy stays, and the enumerated `include_str!` form would instead panic in `self_scaffold_asset` (`src/agents_md_drift.rs:97-103`). Two other edits reach the identical residual without "removing a prompt from the pack": tagging an existing prompt row with a module, and changing its `dest` out from under `PROMPT_DEST_PREFIX`. In both, the prompt is still in the pack, still shipped to some configuration, and its committed copy in this repo becomes permanently unguarded.

Evidence (mutation, run and reverted): added `module = "checks"` to the `dest = ".agents/prompts/reviewer.md"` row in `pack/pack.toml`.

    $ cargo test the_committed_role_prompts_match_a_fresh_render
    test agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render ... ok
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 366 filtered out

The guard passed over six files instead of seven and said nothing; the non-empty assertion at `src/agents_md_drift.rs:401-404` cannot see a set that shrinks from 7 to 6. The mitigation, which the guard does not own and does not mention, is elsewhere in the suite:

    $ cargo test
    test manifest::tests::builtin_manifest_lists_the_expected_assets ... FAILED
    test manifest::tests::builtin_checks_module_adds_its_five_assets ... FAILED
    test result: FAILED. 365 passed; 2 failed

`manifest::tests::builtin_manifest_lists_the_expected_assets` (`src/manifest.rs:584-621`) is an exact-list assertion over the module-free render, so any of these edits forces a deliberate update to that list. Once that list is updated in the same change, the orphan is silent again, which is precisely `H4-3`. Suggested wording fix: say "a prompt that leaves the module-free render (removed, module-tagged, or re-destined)" rather than "REMOVED from the pack", and record that the exact-list manifest test is what makes the mechanism deliberate rather than accidental.

## False-negative avenues attacked that did NOT produce a finding

These are recorded because a guard that correctly fails is the evidence that it covers the case.

1. Control, pack edit not regenerated. Changed "First, read" to "First, skim" in `pack/prompts/planner.md`. FAILED as required, naming the file: "`.agents/prompts/planner.md` has drifted from a fresh render of the pack's prompts ... run `just scaffold-self`". This also confirms `build.rs`'s `rerun-if-changed` tracking makes a pack edit visible without a manual rebuild.
2. Control, hand edit of a deployed copy. Changed `# Orchestrator` to `# Orchestrator (hand edited)` in `.agents/prompts/orchestrator.md` (the file step 90 regenerates). FAILED as required, naming `.agents/prompts/orchestrator.md`. The runtime `CARGO_MANIFEST_DIR` read picks up a working-tree edit with no rebuild, so the two-way check is real in both directions.
3. Vacuous coverage. `PROMPT_DEST_PREFIX` matches exactly the seven core prompts. Verified end to end rather than by reading the code: ran the real justfile config into a scratch directory (`cargo run -- scaffold --output-dir <tmp> --write --force --principles default --instrument`) and `diff -r <tmp>/.agents/prompts .agents/prompts` reports no difference, with exactly the seven files `clarifying-questions, implementer, open-questions-gate, orchestrator, planner, reviewer, triager`. The derived set therefore equals what `just scaffold-self` actually writes, and the committed copies are byte-identical to a fresh render today.
4. Config divergence against the justfile, line by line. `justfile:47` is `scaffold --output-dir . --write --force --principles default --instrument`; the test pins `manifest::builtin()`, `resolve_selection(principles, "default")`, `Detail::Summary` (the CLI default at `src/main.rs:402-403`), `&HashMap::new()` (no `--var`), `true` (`--instrument`), `&[]` (no `--module`). All six match, and item 3 above confirms it behaviourally. Note the prompts are copied verbatim (no `render = true` row in `pack/pack.toml`), so `principles`, `detail`, `instrument`, and `vars` cannot affect them at all; only the module selection can, and that is the case the module doc calls out.
5. A prompt added to the pack MANIFEST but never committed. `committed_asset` panics with a message naming the path and telling the reader to regenerate. Correctly fails.
6. Normalisation masking, heading absorbs the following line. `normalize_wrapping("# X\nY")` does equal `normalize_wrapping("# X Y")` and the precondition trips on neither, but the state is unreachable: prettier inserts a blank line after a heading, so no `nix fmt`-clean file can hold it. Verified with prettier 3.6.2 in a scratch directory: input `# Planner\nFirst, read AGENTS.md.` comes out as `# Planner\n\nFirst, read AGENTS.md.`. Not a finding.
7. Normalisation masking, list-item and blockquote lazy continuations. Same unreachability: prettier at `proseWrap=never` joins `- alpha\n  beta` to `- alpha beta` and `> quote\nlazy` to `> quote lazy`, so the split form is not a formatter fixed point, and joining them is prettier's own freedom rather than drift.
8. Normalisation masking, tables. Prettier pads table cells (`| a | b |` becomes `| a   | b   |`), which is a multi-space run, so a table entering a guarded prompt trips `assert_no_unprotected_construct` loudly rather than being masked. Correct fail-safe behaviour.
9. Normalisation masking on the CURRENT file set. Checked every guarded file for the only structure the join can collapse: a non-blank line whose predecessor is non-blank and which is not itself a hard start. There are none in any of the seven prompts or their pack sources, so `normalize_wrapping` currently maps each non-blank line to its own logical line and the transform is injective on content there. Concretely this means block-boundary edits are all caught: removing the blank line between a heading and its paragraph, between two paragraphs, or between two list items each changes the normalized string. FN-2 is the one construct that would break this, and it is absent today.
10. Whitespace-significant edits, other forms. A trailing two-space hard break, a mid-line tab, an NBSP, and any leading indentation all trip the precondition (pinned by the existing `precondition_rejects_*` tests). A zero-width space is not `White_Space`, so it survives normalization on both sides and any difference in it is caught by the equality check, not masked. CRLF versus LF normalizes equal, but a line-ending change is not content drift.
11. Comparison pairing. Each asset is compared against `CARGO_MANIFEST_DIR/<its own dest>`; `assert_no_unprotected_construct` runs on BOTH sides of EVERY file inside the loop (`src/agents_md_drift.rs:414-415`) with no early `continue`, and a wrong `source` mapping in `pack.toml` would surface as a content mismatch. No skipped or crossed comparison.
12. Environment dependence and staleness. `build.rs` emits `cargo:rerun-if-changed` for the pack directory and every file under it, so an edited, added, or removed pack file forces a recompile and re-embed (avenue 1 confirms this empirically). The committed side is read at runtime, so it needs no rebuild at all. No shared `CARGO_TARGET_DIR` is set in `.envrc` or `flake.nix`, so a cross-worktree stale binary is not reachable here.
13. Working tree versus git index. `committed_asset` reads the working tree, not `HEAD`, so a deployed prompt written by `just scaffold-self` and never `git add`ed would pass locally. Not raised as a finding: the module doc discloses exactly this trade-off at `src/agents_md_drift.rs:38-42`, and the pre-existing `include_str!` sides read the working tree in the same sense at compile time, so the change introduces no new exposure.
14. Scope-boundary doc accuracy. The `PROMPT_DEST_PREFIX` doc (`src/agents_md_drift.rs:71-77`) lists `.agents/user-prompts/*`, `.agents/LEDGER.template.md`, `.agents/principles.toml`, and `.agents/workflow.toml` as carrying the same gap. Verified accurate: `src/workflow_spec.rs:187-209` pins `pack/workflow.toml` against `WorkflowSpec::builtin()` but never compares the DEPLOYED `.agents/workflow.toml`, and `src/pack.rs:57` embeds `pack/principles.toml` with no deployed-copy comparison either. The `checks-reviewer` exclusion is also accurate: it is module-gated in `pack/pack.toml:219-223`, absent from the module-free render, and the repo commits no copy of it.
