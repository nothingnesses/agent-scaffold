# Step 92 `prompt-drift-guard`: work review round 2, fix verification and fact-checking

Reviewer lens: FIX VERIFICATION AND FACT-CHECKING. I did not see round 1 and judged from the artifacts.

Artifact: `git diff 3164404..38d9db4` (whole change) and `git diff 97a587c..38d9db4` (the round-1 fix commit, `docs: correct three overclaims in the drift guard's comments`), both `src/agents_md_drift.rs` only.
Worktree: `.claude/worktrees/rev2-pdg-verify`, detached at `38d9db4`. Every mutation below was reverted; `git status --short` and `git diff` are both empty at the end, with only this findings file untracked.

## Summary

| Fix | Verdict | New claims checked | Verified |
| --- | --- | --- | --- |
| `FN-1` | CLOSED | 4 | 4 |
| `FN-2` | CLOSED | 6 | 6 |
| `CT-1` | CLOSED | 7 | 7 |

ONE finding, `V2-1`, severity `low`. It is NOT a re-raise of the accepted `FN-2` mechanism residual (I accept that residual in full); it is that the fix left two OTHER statements in the same file still asserting the totality the fix went in to retract, one of them demonstrably false.

Severities found: `critical` NONE. `high` NONE. `medium` NONE. `low` ONE (`V2-1`).

## Checks

`cargo test`: 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed. Matches the expected 379.
`cargo clippy --all-targets -- -D warnings`: clean, exit 0.

## 1. The change is comment-only. CONFIRMED, two independent ways.

I did not use the implementer's filter. Both of these are re-runnable from the worktree root.

Filter A, every added or removed line must be a doc line:

    git diff 97a587c..38d9db4 -U0 | grep -E '^[+-]' | grep -vE '^(\+\+\+|---)' \
      | sed -E 's/^[+-]//' | grep -vE '^[[:space:]]*(//!|///)'

Output: EMPTY (0 lines) out of 77 changed lines. `git diff 97a587c..38d9db4 --name-only` returns `src/agents_md_drift.rs` alone.

Filter B, stronger, strip every doc line from both revisions and diff the remainder:

    git show 97a587c:src/agents_md_drift.rs | grep -vE '^[[:space:]]*(//!|///)' > old.rs
    git show 38d9db4:src/agents_md_drift.rs | grep -vE '^[[:space:]]*(//!|///)' > new.rs
    diff old.rs new.rs

Output: EMPTY. So no executable line, no test, no assertion message, no signature, and no non-doc `//` comment changed. This also settles the two settled-item checks in section 5 outright.

## 2. `FN-1`. CLOSED. All four new claims VERIFIED.

CLAIM: "the set comes from the `[[asset]]` rows of `pack/pack.toml`, NOT from a directory listing of `pack/prompts/`". VERIFIED by code path, not by inference. `manifest::load` (`src/manifest.rs:470`) ends in `manifest.asset.into_iter().filter(...).map(...)` (`src/manifest.rs:524-534`), where `manifest` is `toml::from_str` of the `pack.toml` text (`src/manifest.rs:312-315`). Content comes from `PackSource::read`, which for the embedded pack is `dir.get_file(rel)`, an exact-path lookup (`src/manifest.rs:295-309`). `grep -n read_dir src/manifest.rs src/main.rs` finds no directory listing anywhere in the render path (the only `read_dir` in the repo is `build.rs:38`, which tracks rebuild inputs and emits nothing).

CLAIM: "a file dropped into `pack/prompts/` with no matching row is neither rendered nor guarded and the suite stays green". VERIFIED empirically. I created `pack/prompts/experimental.md` with no `[[asset]]` row. `build.rs:22` tracks the pack directory for `cargo:rerun-if-changed`, so the file really was re-embedded rather than being invisible to a stale build. Results:

    cargo test  ->  367 + 5 + 1 + 3 + 1 + 2 passed, 0 failed

    cargo run -- scaffold --output-dir <tmp> --write --force --principles default --instrument
    find <tmp>/.agents/prompts -type f | sort
      .agents/prompts/clarifying-questions.md
      .agents/prompts/implementer.md
      .agents/prompts/open-questions-gate.md
      .agents/prompts/orchestrator.md
      .agents/prompts/planner.md
      .agents/prompts/reviewer.md
      .agents/prompts/triager.md
    find <tmp> -name experimental.md   ->  no match

File removed; tree clean.

CLAIM: "an unregistered file is never emitted and so has no committed copy that could go stale". VERIFIED by the same sweep: `find <tmp> -name experimental.md` matched nothing anywhere in the output tree, not just under `.agents/prompts/`. Seven prompts emitted, `checks-reviewer.md` correctly absent (module-gated).

CLAIM: "it is the manifest, not the guard, that decides what ships". VERIFIED, this is the `manifest.asset.into_iter()` line above.

## 3. `FN-2`. CLOSED. All six new claims VERIFIED.

Method: I added a temporary `v2_probe` test to `src/agents_md_drift.rs` calling the module's real `is_hard_start`, `normalize_wrapping`, and `precondition_rejects`, ran it with `--nocapture`, and removed it. Verbatim output below.

CLAIM: "a raw HTML block is a construct whose lines are not hard starts and are therefore JOINED by `normalize_wrapping`". VERIFIED.

    is_hard_start("<pre>")     = false
    is_hard_start("line one")  = false
    is_hard_start("</pre>")    = false
    normalized multi : "# T\n\n<pre> line one line two </pre>"
    normalized single: "# T\n\n<pre> line one line two </pre>"

CLAIM: "`assert_no_unprotected_construct` is a PER-LINE check that makes no check on the cross-line join, so such a construct passes the precondition and is masked". VERIFIED.

    precondition rejects multi-line form : false
    precondition rejects single-line form: false
    MASKED (normalize equal): true

Second fixture, a newline relocated inside a `<pre>` (a real content difference in whitespace-significant HTML): `precondition rejects either: false / false`, `MASKED: true`.

CLAIM: "prettier keeps a raw HTML block verbatim rather than reflowing it". VERIFIED independently, prettier 3.6.2 with the repo's own `.prettierrc.json` (`{"proseWrap": "never"}`), in a scratch directory outside the repo. A soft-wrapped paragraph in the same file was joined onto one line; both a `<pre>` block and a `<div>` block came back byte-identical and still multi-line. So the multi-line form is a fixed point of `nix fmt` and the difference is not attributable to the formatter.

CLAIM: "(a) to (c) ARE pinned by the precondition while (d) is NOT". VERIFIED. I constructed each and confirmed the precondition actually fires; none of the three slipped through.

    (a) nested list item                 rejected: true
    (a) continuation-indented list item  rejected: true
    (a) tab-indented list item           rejected: true
    (b) 4-space indented code block      rejected: true
    (c) multi-space inline code span     rejected: true
    (c) bare multi-space run             rejected: true
    (d) raw HTML block                   rejected: false

CLAIM: "a predicate treating an unrecognised line-structured block as verbatim was implemented and measured, and it rejects an ordinary soft-wrapped paragraph". VERIFIED by re-implementing the predicate from scratch (reject a non-hard-start, non-blank, non-fenced line that directly follows a non-blank line) rather than trusting the round-1 numbers:

    proposed rejects an ordinary soft-wrapped paragraph: true
    current precondition rejects that same reflow:       false
    proposed rejects the html block:                     true
    proposed rejects guarded AGENTS.md:                       false
    proposed rejects guarded .agents/AGENTS.reference.md:     false
    proposed rejects guarded .agents/prompts/{orchestrator,planner,clarifying-questions,
      open-questions-gate,reviewer,triager,implementer}.md:   all false

So the predicate does close (d) and does accept every guarded file today, and it does regress the reflow tolerance that `prompt-drift-guard.md:11` requires. The comment's description is accurate.

CLAIM: "The guarded files carry none of these today, so all four are latent". VERIFIED. (a) to (c): the precondition runs on all nine guarded files in the passing suite, so none is present. (d): `grep -nE '^\s*<' AGENTS.md .agents/AGENTS.reference.md .agents/prompts/*.md` returns nothing in any of the nine files, so there is no raw HTML block start.

NOTE, NOT A FINDING. The parenthetical "so a real content change inside one could be masked" is true as a modal claim (relocating a newline inside a `<pre>` is a real content change and is masked, shown above), but the masked class is narrower than a first reading suggests: a word change inside the same block is NOT masked (`MASKED: false` for `line two` -> `line three`). The narrowing is stated elsewhere in the file (`src/agents_md_drift.rs:211-214`, "can at most change a newline into a space"), so a reader has it, and the claim as written is not false. I raise it only so the triager knows it was checked.

## 4. `CT-1`. CLOSED. All seven new claims VERIFIED.

CLAIM: "the remainder is LARGER than what is guarded". VERIFIED by count. `builtin_manifest_lists_the_expected_assets` (`src/manifest.rs:584-622`) asserts an exact 30-entry dest list for the module-free render. Guarded = 9 (`AGENTS.md`, `.agents/AGENTS.reference.md`, 7 prompts). Unguarded = 21. 21 > 9.

CLAIM: the named examples. VERIFIED, all present in that list and all unguarded: `.agents/user-prompts/{kickoff,explore,review,pause,compaction-prep,resume}.md` (6), `.agents/LEDGER.template.md`, `.agents/principles.toml`, `.agents/workflow.toml`, and the `docs/plans/TEMPLATE*` family (12 rows, `pack/pack.toml:40-95`). The comment's "NOT AN EXHAUSTIVE LIST" hedge is itself accurate: 9 + 12 = 21, which happens to be the whole remainder, so the hedge is conservative rather than wrong.

CLAIM: "the authoritative asset list is `builtin_manifest_lists_the_expected_assets` in `src/manifest.rs`". VERIFIED. It exists at `src/manifest.rs:584`, and its body is a single `assert_eq!(dests, vec![...])` over the full ordered dest list, so it is an exact-list assertion that any add, removal, module-tag, or re-`dest` breaks. It is the right pointer for this guard specifically, because it loads with `&[]` modules, the same module-free selection `self_scaffold_assets` uses.

CLAIM: "everything in it outside this prefix, other than the two files the `include_str!` guards above cover, is unguarded". VERIFIED by sweep. `grep -rn "include_str!" src/ tests/` shows the only committed-copy embeds are `../AGENTS.md` and `../.agents/AGENTS.reference.md` (in `findings_naming.rs`, `workflow_spec.rs`, `recommendation_rule.rs`, `isolation_policy.rs`, `agents_md_drift.rs`). `src/workflow_spec.rs:187` and `src/pack.rs:57` embed `pack/workflow.toml` and `pack/principles.toml`, which are pack SOURCES, not the deployed `.agents/` copies, so they do not guard those copies. `grep -rn CARGO_MANIFEST_DIR src/ tests/` shows the only committed-copy read is `agents_md_drift.rs:132`, the prompt loop. `src/manifest.rs:644` touches `.agents/principles.toml` only as a rendered-asset property check, and `src/main.rs:1827,1885` operate on a temp output dir.

CLAIM: widening to the Markdown copies is "close to a one-line change" because "they already satisfy the precondition". VERIFIED, both halves, and the second half is the one that could have been wrong.

Precondition, run against every committed Markdown copy:

    .agents/user-prompts/kickoff.md                    rejected: false
    .agents/user-prompts/explore.md                    rejected: false
    .agents/user-prompts/review.md                     rejected: false
    .agents/user-prompts/pause.md                      rejected: false
    .agents/user-prompts/compaction-prep.md            rejected: false
    .agents/user-prompts/resume.md                     rejected: false
    .agents/LEDGER.template.md                         rejected: false
    docs/plans/TEMPLATE._status-narrative.md           rejected: false
    docs/plans/TEMPLATE.motivations.md                 rejected: false
    docs/plans/TEMPLATE.principles-note.md             rejected: false
    docs/plans/TEMPLATE.documentation-protocol.md      rejected: false
    docs/plans/TEMPLATE.repo-layout.md                 rejected: false
    docs/plans/TEMPLATE.queue-intro.md                 rejected: false
    docs/plans/TEMPLATE.roadmap-intro.md               rejected: false
    docs/plans/TEMPLATE.success-criteria.md            rejected: false
    docs/plans/TEMPLATE.steps/example-step.md          rejected: false

All 16 accepted. "One-line change" tested literally: I changed the single filter line in `the_committed_role_prompts_match_a_fresh_render` from `asset.dest.starts_with(PROMPT_DEST_PREFIX)` to `asset.dest.ends_with(".md")` and ran it:

    test agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render ... ok

So the widening really is one line and really does pass today. Reverted with the Edit tool.

CLAIM: "prose under the same prettier settings". VERIFIED. `flake.nix:62-70` enables prettier with `includes = ["*.md", ...]`; `flake.nix:50-54` excludes only `src/plan/testdata/render-fixture*`, `docs/plans/agent-scaffold.md`, and `docs/plans/TEMPLATE.md`, none of which is a copied Markdown asset. There is no `.prettierignore`. `.prettierrc.json` is `{"proseWrap": "never"}`, the same setting the guarded prompts sit under.

CLAIM: "`.agents/principles.toml` has lines outside canonical whitespace form (indented multi-line array continuations among them) that `assert_no_unprotected_construct` rejects on sight". VERIFIED, including the parenthetical, which is the part that could have been decoration. The precondition rejects the file (`rejected: true`). It has 21 non-canonical lines; 15 are aligned comment blocks (`.agents/principles.toml:4-18`) and 6 ARE indented multi-line array continuations:

    248  '  "make-failure-and-absence-explicit",'
    249  '  "prefer-compile-time-enforcement",'
    264  '  "https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/",'
    514  '  "document-the-why-not-the-what",'
    515  '  "leave-durable-notes",'
    516  '  "keep-docs-next-to-code",'

The comment does NOT overclaim here: it says only that `principles.toml` trips, and covers the rest with "even where it happens not to trip". That hedge is load-bearing and correct, because `.agents/workflow.toml` is in fact ACCEPTED by the precondition (`rejected: false`). Had the comment said "the TOML assets trip the precondition", it would have been false; it does not.

## 5. Settled items and regressions. CLEAN.

`FN-2` MECHANISM, accepted residual, must be untouched: CONFIRMED. Filter B in section 1 shows the non-doc content of the file is byte-identical between `97a587c` and `38d9db4`, so `assert_no_unprotected_construct`, `is_hard_start`, and `normalize_wrapping` are unchanged as code. No helpful hardening was slipped in.

`FN-3`, accepted residual, must have no code change or exclusion added: CONFIRMED by the same filter. `PROMPT_DEST_PREFIX` still equals `".agents/prompts/"`, the filter is still `starts_with(PROMPT_DEST_PREFIX)`, and no exclusion list, skip, or module-tag special case was introduced.

I did not re-raise `H4-3`, `R3-CQ-1`, the `checks-reviewer.md` implicit exclusion, or the render-config duplication, and I have no new evidence against any of them. `.agents/prompts/checks-reviewer.md` is still correctly absent from the render (section 2 sweep).

## `V2-1` (severity `low`): the fix retracts the precondition's totality in two places and leaves it asserted in two others, one of them demonstrably false

The fix commit's own message is "correct three overclaims in the drift guard's comments". It corrected the overclaim at the enumeration (`src/agents_md_drift.rs:297-317`) and added the retraction at the predicate's own doc (`src/agents_md_drift.rs:150-155`, "IT IS A PER-LINE CHECK AND SO NOT TOTAL"). Two further statements of the same overclaim are still in the file, unchanged.

PRIMARY, and this one is false rather than merely unqualified. `src/agents_md_drift.rs:209-214`, the `is_hard_start` doc:

> Precision here affects only how closely the canonical form mirrors prettier and how readable a failure diff is; it does NOT affect correctness. The transform only ever deletes or collapses whitespace and is applied identically to both sides, so misclassifying a structural line can at most change a newline into a space (or vice versa) on both sides equally; it can never merge two distinct non-whitespace tokens into one.

The new text at `src/agents_md_drift.rs:302-303` and `:315-317` says the opposite in the same file: a raw HTML block is masked because "its lines are not hard starts, so they are JOINED", and "the same reasoning admits any future line-structured construct `is_hard_start` does not recognise". A recognition failure IS a precision failure, so the new paragraph asserts exactly what `:209-210` denies.

WHY THE ARGUMENT AT `:210-213` DOES NOT RESCUE IT. "Applied identically to both sides" rules out a false FAILURE: a lossy transform applied to both sides cannot invent a difference. It says nothing about a false PASS, which is what (d) is. Applying a lossy transform to both sides is precisely how a real difference gets erased on both sides and the inputs compare equal. The conclusion "it does NOT affect correctness" is stronger than the stated argument supports, and (d) is the counterexample the fix itself just documented.

EVIDENCE, mutation form. With the temporary `v2_probe` in place, at `38d9db4` unmodified:

    is_hard_start("<pre>") = false
    normalized multi : "# T\n\n<pre> line one line two </pre>"
    normalized single: "# T\n\n<pre> line one line two </pre>"
    MASKED (normalize equal): true

Then changing ONLY `is_hard_start`'s precision, `src/agents_md_drift.rs:218`, `Some(b'#' | b'>' | b'|')` -> `Some(b'#' | b'>' | b'|' | b'<')`:

    is_hard_start("<pre>") = true
    normalized multi : "# T\n\n<pre> line one line two\n</pre>"
    normalized single: "# T\n\n<pre> line one line two </pre>"
    MASKED (normalize equal): false

One byte of `is_hard_start` precision flips the outcome from "two different inputs compare equal" (drift masked) to "they compare different" (drift caught). Precision therefore affects correctness, which is what `:209-210` denies. Both mutations reverted with the Edit tool; `git diff` is empty.

SECONDARY, weaker, same defect class. `src/agents_md_drift.rs:397-402`, inside `the_committed_scaffold_matches_a_fresh_render`:

> both the fresh render and the committed copy must be free of any indentation- or whitespace-significant construct ... Asserted on both sides so the guard fails loudly the day such a construct enters the guidance.

A raw HTML block is a whitespace-significant construct, and the guard does not fail loudly the day one enters. This is softened by its own "(see its doc comment)" pointer, which now leads to the corrected text, so I rate it as a secondary location of the same fix rather than a finding of its own.

NOT A DEFECT, checked and cleared: `src/agents_md_drift.rs:456-460`, the three-item restatement in the prompt loop that the round-1 triage flagged. It reads as illustrative ("a prompt could gain a nested list, indented code, or a multi-space inline span"), not as a closed set, and all three items are accurate. I would not require a change there.

WHY THIS IS NOT A RE-RAISE OF THE ACCEPTED RESIDUAL. I accept the `FN-2` mechanism residual entirely and ask for no predicate change, no `normalize_wrapping` change, and no new test. The finding is that the doc fix is incomplete: the file now states both that the precondition is not total (twice) and that it is (twice), and a maintainer who opens `is_hard_start` to decide whether a newly added block construct needs recognition is told, in that exact spot, not to bother.

SEVERITY `low`, absolute. Nothing misbehaves, reachability is zero today (no guarded file contains a raw HTML block), the correct statement is loud and in the same file, and the fix is comment-only. It is not lower than `low` because the claim is flatly wrong rather than vague, and it sits at the one place a reader would consult before making the decision it would mislead them on.

SUGGESTED FIX (comment-only, no behaviour change): at `src/agents_md_drift.rs:209-214`, replace "it does NOT affect correctness" with the accurate scoping, that identical application to both sides rules out a false FAILURE but not a false PASS, and that a line-structured construct `is_hard_start` does not recognise is joined and masked (pointing at the UNPROTECTED CONSTRUCTS paragraph). At `src/agents_md_drift.rs:401-402`, soften "fails loudly the day such a construct enters" to the per-line scope the predicate actually has.

## Anything else

Nothing further. I attacked the remaining shapes and found no additional issue: the `docs/plans/TEMPLATE*` group is fully committed and byte-clean under the one-line widening (section 4), `.agents/workflow.toml`'s acceptance by the precondition is correctly hedged rather than misstated, `builtin_manifest_lists_the_expected_assets` is scoped to the module-free render, which is the same selection the guard uses, so pointing at it is not an overclaim, and the unregistered-pack-file case is invisible to the whole suite exactly as `FN-1`'s new text says.

Tree state: clean. `git status --short` empty, `git diff` empty, HEAD `38d9db4`, only this findings file untracked. Post-review re-verification on the clean tree: `cargo test` 379 passed, 0 failed; `cargo clippy --all-targets -- -D warnings` clean.
