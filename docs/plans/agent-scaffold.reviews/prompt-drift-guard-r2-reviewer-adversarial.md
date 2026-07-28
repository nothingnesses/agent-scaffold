# Step 92 `prompt-drift-guard`: reviewer findings (work review, round 2, fresh adversarial lens)

Artifact: `git diff 3164404..38d9db4`, a single-file change to `src/agents_md_drift.rs` (+175 / -25).
Brief: `docs/plans/agent-scaffold.steps/prompt-drift-guard.md`.
Worktree: `.claude/worktrees/rev2-pdg-adversarial`, detached at `38d9db4`.
Lens: fresh adversarial attack on the guard mechanism, hunting a demonstrated false negative. I attacked the code before reading any round-1 material, then read `prompt-drift-guard-triage.md` (and the round-1 falseneg reviewer's avenue list) only to classify overlaps.

Baseline at `38d9db4`: `cargo test` 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed. `cargo clippy --all-targets -- -D warnings` clean.

## Summary

| Finding | Severity | Kind |
| --- | --- | --- |
| `A2-1` | low | Doc overclaim in the module header and the test name |
| `A2-2` | low | Doc: the pointer to the "authoritative asset list" omits one emitted, committed, unguarded file |

NO finding at `medium`, `high`, or `critical`. I state that explicitly rather than leaving it inferred.

I did NOT find a reachable false negative in the guard mechanism. I attacked the normalisation, the derived set, the pairing, the render config, the build-staleness path, and the pre-existing checks, with twelve mutations (listed below). Every mutation that SHOULD have failed the guard did fail it, and the two that passed are both already-accepted residuals rather than new holes. Both findings below are documentation defects of the same species the round-1 triage ruled VALID (`FN-1`, `CT-1`); neither changes behaviour and neither requires a new or changed test.

## `A2-1`: the module header claims coverage of "every deployed role prompt", which the accepted `H4-3` residual denies

SEVERITY: `low`. DOC defect, not a mechanism defect. Comment-only fix (plus, optionally, the test name).

THE TEXT. `src/agents_md_drift.rs:1-3`:

    //! Whole-file drift guard for the scaffold files this repo dogfoods: the generated
    //! `AGENTS.md`, its tool-owned copy `.agents/AGENTS.reference.md`, and every deployed
    //! role prompt under `.agents/prompts/`.

and the test name at `src/agents_md_drift.rs:427`, `the_committed_role_prompts_match_a_fresh_render`, which asserts the same correspondence over "the committed role prompts" as a set.

WHAT THE MECHANISM ACTUALLY DELIVERS. The loop iterates the RENDERED assets and, for each, demands a matching committed file (`src/agents_md_drift.rs:440-469`). That is a one-way map: render -> committed. It asserts nothing about a committed file under `.agents/prompts/` that the module-free render does not emit. So the guarded set is "every role prompt the module-free render emits", which is a subset of "every deployed role prompt under `.agents/prompts/`", and the two coincide only in the tree's current state.

EVIDENCE (re-run immediately before writing this file; reverted, tree clean). Deploy an eighth role prompt under the guarded prefix, deliberately stale relative to its pack source, and the whole module stays green:

    cp pack/prompts/checks-reviewer.md .agents/prompts/checks-reviewer.md
    printf '\nTHIS DEPLOYED ROLE PROMPT IS STALE AND CONTRADICTS ITS PACK SOURCE.\n' >> .agents/prompts/checks-reviewer.md
    ls .agents/prompts/
      checks-reviewer.md
      clarifying-questions.md
      implementer.md
      open-questions-gate.md
      orchestrator.md
      planner.md
      reviewer.md
      triager.md
    cargo test --bin agent-scaffold agents_md_drift
      test agents_md_drift::tests::precondition_exempts_fenced_indented_lines_but_not_bare_ones ... ok
      test agents_md_drift::tests::normalization_tolerates_wrapping_but_not_content_change ... ok
      test agents_md_drift::tests::precondition_rejects_non_space_whitespace_and_round_one_cases ... ok
      test agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render ... ok
      test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... ok
      test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 362 filtered out

Eight deployed role prompts under `.agents/prompts/`, one of them contradicting its pack source, and the guard whose header says it covers "every deployed role prompt under `.agents/prompts/`" reports success.

I AM NOT REOPENING THE MECHANISM RESIDUAL, and I say so plainly. The MECHANISM behind that output is the accepted `H4-3` / `FN-3` residual (a prompt outside the module-free render drops silently out of the derived set), and the round-1 triager attacked this same extra-committed-file shape itself (`prompt-drift-guard-triage.md:163`) and classified it as that residual. I accept that verdict without qualification. `A2-1` is ONLY about the header text and the test name asserting a coverage property that the accepted residual explicitly denies.

WHY IT IS A FINDING RATHER THAN A RESTATEMENT. The round-1 triage ruled two texts of exactly this species VALID and required them fixed:

- `FN-1`: "a prompt added to the pack is guarded" was true of the tree as it stood and false as a general claim about the mechanism. Fixed by inserting one word (`MANIFEST`).
- `CT-1`: a list that "must stop asserting a completeness it does not have" (`prompt-drift-guard-triage.md:145`).

The header carries the identical defect at a different location: a general coverage claim that the mechanism does not deliver. The lower body of the module doc states the mechanism correctly (`src/agents_md_drift.rs:35-38`, "every rendered asset whose `dest` starts with `.agents/prompts/`"), and commit `38d9db4` corrected three neighbouring overclaims, so this one reads as an omission from that pass rather than a deliberate choice. The header is the first and most-quoted line of the module and is the durable record a future maintainer reads before the 30 lines that qualify it.

REACHABILITY, LABELLED HONESTLY. The header is TRUE of the tree today: the deployed set equals the rendered set (verified, `diff -r` of a real CLI render against `.agents/prompts/` reports no difference, and the repo commits no `checks-reviewer.md`). The overclaim is therefore LATENT in exactly the same sense `FN-1`'s was. What makes it more than hypothetical is that this repo demonstrably does commit files at module-gated destinations already: `.agents/checks.toml` is tracked (`git ls-files .agents/`) while its `[[asset]]` row is `module = "checks"` (`pack/pack.toml:196-200`), and the module-free self-scaffold config never emits it. One `scaffold --module checks --write --force` run puts `.agents/prompts/checks-reviewer.md` into the tree the same way.

WHAT THE FIX MUST ACHIEVE. `src/agents_md_drift.rs:1-3` must not claim more coverage than the mechanism sentence at `:35-38` delivers. One clause, for example "every role prompt the module-free render emits under `.agents/prompts/`" in place of "every deployed role prompt under `.agents/prompts/`". Renaming the test is optional and I do not require it; if the name is left alone, the header fix alone removes the contradiction, since the loop's own comment block already describes the correspondence accurately. Comment-only, no behaviour change, no new test.

## `A2-2`: the "authoritative asset list" pointer cannot express one emitted, committed, unguarded file

SEVERITY: `low`. DOC defect. Comment-only fix. This is the marginal one of the two and I flag it as such, but it is a defect in text this artifact introduced, so it belongs in this round rather than being carried.

THE TEXT. `src/agents_md_drift.rs:84-87`, part of the round-1 `CT-1` fix:

    /// group as complete; the authoritative asset list is
    /// `builtin_manifest_lists_the_expected_assets` in `src/manifest.rs`, and everything in
    /// it outside this prefix, other than the two files the `include_str!` guards above
    /// cover, is unguarded.

That sentence directs a reader who wants the complete unguarded set to a specific test. The whole purpose of the surrounding note, per the brief (`prompt-drift-guard.md:21`) and per the round-1 triage's own reasoning (`prompt-drift-guard-triage.md:143`), is to let a human judge whether widening the guard is worth it.

THE GAP. `just scaffold-self` emits, and this repo commits, one file that is NOT in that list and is guarded by nothing:

    git ls-files docs/plans/TEMPLATE.md
      docs/plans/TEMPLATE.md

    # absent from the expected-asset list (src/manifest.rs:588-618): 12 TEMPLATE entries,
    # none of them the bare TEMPLATE.md
    sed -n '588,618p' src/manifest.rs | grep 'TEMPLATE\.md"'
      (no match)

    # and nothing anywhere in src/ or tests/ compares it to anything
    grep -rn 'TEMPLATE\.md' src/ tests/ | grep -v testdata
      (no match)

It is absent from the manifest list because it is not a manifest asset: the scaffold generates it AFTER the assets land, from the `TEMPLATE.plan.toml` skeleton (`src/main.rs:1667`, the `strip_suffix(".plan.toml")` post-write render loop). My end-to-end run of the real justfile config into a scratch directory printed `render  docs/plans/TEMPLATE.md` among its outputs, confirming `just scaffold-self` writes it.

The round-1 triager counted this file explicitly when sizing the residual: "omits 13 (the 12 TEMPLATE assets plus the generated `docs/plans/TEMPLATE.md`)" and "22 of 31 emitted files" are unguarded (`prompt-drift-guard-triage.md:143`, `:149`). A reader who follows the new pointer enumerates 21 unguarded files, not 22.

THE COUNTER-ARGUMENT, AND WHY I STILL RAISE IT. The comment's own example list does say "the `docs/plans/TEMPLATE*` family", and `TEMPLATE.md` matches that glob, so no reader is actively misled about the file's existence. The sentence is also literally true as written: everything in the list, outside the prefix, minus the two `include_str!` files, IS unguarded. What it is not is complete, and completeness is exactly what the sentence offers ("Do not read either group as complete; the authoritative asset list is ..."). The practical cost is small but real and specific: `docs/plans/TEMPLATE.md` is a RENDER artifact rather than a copied asset, so widening the drift guard to it needs a different mechanism from the other 21. Someone scoping that widening work off this pointer would miss the one file that does not fit the pattern they are planning for.

WHAT THE FIX MUST ACHIEVE. Either qualify "authoritative asset list" as covering the manifest ASSETS only and note that the scaffold additionally emits the generated `docs/plans/TEMPLATE.md` view, or drop the completeness offer. One clause; comment-only; no behaviour change.

## Attacks attempted, with outcomes

Twelve mutations plus several static checks. Every mutation was reverted with the Edit tool (or, for file add/remove, by removing/restoring the exact file); never `git checkout`. The ones that correctly FAILED the guard are the evidence of coverage and are listed with the same weight as the ones that passed.

Mutations that CORRECTLY caused a failure (the guard works):

1. PACK EDIT NOT REGENERATED. Changed the last sentence of `pack/prompts/planner.md` ("your output is the plan." -> "your output is the MUTATED plan."), deployed copy left stale. `the_committed_role_prompts_match_a_fresh_render` FAILED at `src/agents_md_drift.rs:464` naming `.agents/prompts/planner.md`, both causes, and `just scaffold-self`. This also proves `build.rs`'s `rerun-if-changed` tracking makes a pack edit visible with no manual rebuild, closing the stale-build-artefact avenue.
2. HAND EDIT OF A DEPLOYED COPY. Same sentence edited in `.agents/prompts/planner.md`, pack left alone. FAILED at `:464` naming the same file. The two-way correspondence is real in both directions, and the runtime `CARGO_MANIFEST_DIR` read picks up a working-tree edit with no rebuild.
3. DEPLOYED COPY DELETED. Moved `.agents/prompts/triager.md` out of the tree. FAILED at `src/agents_md_drift.rs:134` with "failed to read the committed .agents/prompts/triager.md at <abs path>: No such file or directory (os error 2). The self-scaffold render produces this file, so the repo must commit it; run `just scaffold-self`". A vanished deployed prompt is caught, not skipped.
4. WHITESPACE-SIGNIFICANT CONSTRUCT IN A REAL PROMPT. Inserted a double space into `.agents/prompts/planner.md:7`. FAILED at `src/agents_md_drift.rs:195` with the precondition message naming the file, the line number, the observed line, and its canonical form. The fail-safe fires on real guarded content, not just on synthetic fixtures.
5. PRE-EXISTING GUARD 1 STILL INTACT. Edited the first prose line of `.agents/AGENTS.reference.md`. `the_committed_scaffold_matches_a_fresh_render` FAILED at `src/agents_md_drift.rs:419`, the reference `assert_eq!`. The `self_scaffold_asset` -> `self_scaffold_assets` + lookup refactor did not weaken it.
6. PRE-EXISTING GUARD 2 STILL INTACT. Edited the same line of the root `AGENTS.md`. FAILED at `src/agents_md_drift.rs:414`, the `AGENTS.md` `assert_eq!`. Both older checks survive the refactor unchanged.
7. NON-VACUITY ASSERT IS LIVE, NOT DEAD CODE. Changed `PROMPT_DEST_PREFIX` from `.agents/prompts/` to `.agents/roles/`. FAILED at `src/agents_md_drift.rs:448` with "the self-scaffold render dropped no asset under .agents/roles/, so this guard checked nothing; if the role prompts moved, point PROMPT_DEST_PREFIX at their new destination". The defensive emptiness check actually fires.

Mutations that PASSED, and what that means:

8. EXTRA STALE DEPLOYED ROLE PROMPT under the guarded prefix. PASSED. This is the `A2-1` evidence above. The MECHANISM is the accepted `H4-3` residual and the round-1 triager found the same shape (`prompt-drift-guard-triage.md:163`); only the header text is my finding.
9. SOFT-WRAP SPLIT OF A DEPLOYED PROMPT. Split one paragraph of `.agents/prompts/planner.md` across three physical lines, no content change. PASSED. This is the guard behaving as designed and as the brief requires (`prompt-drift-guard.md:11`, an incidental prettier reflow MUST NOT fail). Not a finding.
10. HEADING ABSORBS THE FOLLOWING LINE (transform non-injectivity). Added a temporary probe test and ran it: `normalize_wrapping("# Title\nBody text\n")` and `normalize_wrapping("# Title Body text\n")` both equal `"# Title Body text"`, and `assert_no_unprotected_construct` accepts both, so two documents with different heading text normalise equal. Probe reverted. NOT RAISED: this is the settled cross-line-join residual, and the round-1 falseneg reviewer already attacked this exact case (its avenue 6) and correctly dismissed it as unreachable, because prettier inserts a blank line after a heading so no `nix fmt`-clean file can hold the masked form. My run corroborates the transform behaviour and I accept the unreachability verdict. LATENT, not reachable.

Static and end-to-end checks that produced no finding:

11. RENDER CONFIG FIDELITY, END TO END. Ran the real CLI at the justfile's config into a scratch directory (`cargo run -- scaffold --output-dir <tmp> --write --force --principles default --instrument --vcs none`) and diffed. `diff -r <tmp>/.agents/prompts .agents/prompts` reported no difference across all seven prompts, and `AGENTS.md` came back byte-identical too. Separately, argument by argument: `Detail::Summary` is the CLI default (`src/main.rs:402-403`), `--principles default` matches, `instrument = true` matches `--instrument`, `&HashMap::new()` matches no `--var`, `&[]` matches no `--module`. The test's pinned config is faithful to what `just scaffold-self` runs.
12. NO POST-RENDER TRANSFORM BYPASSED. The test compares `build_assets` output directly, so I checked whether the CLI transforms contents between `build_assets` and the file write. `apply_asset` (`src/main.rs:111-128`) writes `asset.contents` verbatim with no header, banner, or line-ending normalisation, and `outcome_of` (`src/main.rs:82-106`) always refreshes an `Ownership::Reference` asset, which all seven prompt rows are (`pack/pack.toml:105-137`). The comparison is faithful to the bytes that land on disk, and the remediation the failure message prescribes actually fixes the failure.
13. MISSING PACK SOURCE CANNOT SILENTLY SHRINK THE SET. `manifest::load` reads each row's source with `source.read(&spec.source)?` (`src/manifest.rs`), so a manifest row whose pack file is absent is an `Err`, which the test's `.expect("build_assets succeeds for the self-scaffold config")` turns into a loud panic. A deleted pack prompt cannot quietly drop out of the derived set the way a re-tagged row can.
14. FENCE-TRACKING AGREEMENT. `assert_no_unprotected_construct` (`src/agents_md_drift.rs:186-193`) and `normalize_wrapping` (`:336-349`) toggle fence state on the identical predicate (`trim_start()` starts with ``` or ~~~), so the precondition can never exempt a line the transform then collapses, and an odd/unclosed fence makes the tail STRICTER (verbatim) on both sides rather than looser. No masking path here. Separately, no guarded prompt contains a fence at all today (`grep -c '```' pack/prompts/*.md` returns 0 for all eight).
15. CURRENT MASKING SURFACE IS EMPTY. Every one of the seven deployed prompts has zero soft-continuation lines (no non-blank, non-hard-start line following a non-blank line) and zero lines with leading whitespace, so `normalize_wrapping` currently only collapses blank-line runs and the trailing newline. The seven `pack/prompts/*.md` are byte-identical to their `.agents/prompts/*.md` copies (`cmp` on all seven). The transform is effectively a byte compare on this content today, which bounds every normalisation-masking avenue to LATENT.
16. THE PAIRING. Each asset is compared against `CARGO_MANIFEST_DIR/<its own dest>` (`src/agents_md_drift.rs:131-139`), which is the same path `apply_asset` writes to (`output_dir.join(&asset.dest)` with `--output-dir .`). Both precondition assertions run on BOTH sides of EVERY file inside the loop with no early `continue` (`:461-462`). No crossed or skipped comparison.
17. DOCUMENTATION CURRENCY. The change is source-only and adds no scaffolded content, so no deployed file goes stale. No CHANGELOG entry: consistent, since the precedent step `agents-md-drift-guard` added none either (`grep -in "drift\|guard" CHANGELOG.md` returns nothing), and the round-1 triage already settled the convention (`prompt-drift-guard-triage.md:33`). The step sidecar's "Not started." opening line is stale against `status = "in-progress"` in `docs/plans/agent-scaffold.plan.toml:1255`, but that is a repo-wide pattern rather than a defect of this artifact (`planner-folds-decisions` reads "Not started." at `status = "complete"`), so I do not raise it.

## Attacks that landed on already-settled items (corroboration, not findings)

- The extra committed file under `.agents/prompts/` (attack 8). Settled as the `H4-3` / `FN-3` accepted residual, and independently found by the round-1 triager (`prompt-drift-guard-triage.md:163`). I accept the verdict. My `A2-1` is about the header text only.
- The cross-line join masking a construct whose lines are individually canonical (attack 10). Settled as an accepted residual, now documented at `src/agents_md_drift.rs:302-326`. My heading instance corroborates the transform behaviour; the round-1 reviewer's avenue 6 had already established it is unreachable because prettier normalises the masked form away.
- Working tree versus git index. `committed_asset` reads the working tree, not `HEAD`. Settled (round-1 avenue 13): the trade-off is disclosed at `src/agents_md_drift.rs:126-130` and the pre-existing `include_str!` sides read the working tree in the same sense at compile time, so the change adds no new exposure. Not raised.
- The `checks-reviewer.md` implicit exclusion. I verified it independently (module-gated at `pack/pack.toml:219-223`, absent from the module-free render, no committed copy) and agree it is SOUND. Not raised.
- Render-config duplication between the test and the justfile. Considered, verified equivalent end to end (attack 11), pre-existing and self-guarding. Not raised.

## What I did NOT find

No reachable false negative in the guard mechanism. Specifically, no mutation I could construct made the guard pass over a difference between a rendered prompt and its committed copy. Every content difference I introduced on either side was caught, either by the equality check or by the precondition fail-safe. The two masking classes that exist are both bounded to LATENT by attack 15 (the current content has no construct either class can reach), and both are already-accepted residuals.

I want to be explicit that this is a real conclusion and not an absence of effort: attacks 1 to 7 are the positive evidence that the mechanism fires, attacks 11 to 16 are the structural checks that it fires on the right inputs, and attacks 8 to 10 are the three ways I found to make it stay silent, all three of which resolve to settled residuals.

## Round outcome from my seat

TWO findings, both `low`, both comment-only, neither changing behaviour and neither requiring a new or changed test. No `medium`, `high`, or `critical` finding. The backstop is not engaged by anything I raise.

## Tree state

Clean. `git status --porcelain` is empty and `git diff` is empty at `38d9db4`, with only this findings file (untracked) added. Every mutation above was reverted before I wrote this. Post-review re-verification: `cargo test` 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed; `cargo clippy --all-targets -- -D warnings` clean. I ran no `nix fmt` or `just fmt`, and edited no source, plan, or pack file as a deliverable.
