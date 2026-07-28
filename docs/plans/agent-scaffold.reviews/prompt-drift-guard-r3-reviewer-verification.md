# Step 92 `prompt-drift-guard`: work review round 3, consolidation verification and regression

Reviewer lens: CONSOLIDATION VERIFICATION AND REGRESSION. I did not see rounds 1 or 2 and judged from the artifacts.

Artifact: `git diff 9f0966c..9174a74` (whole step) and `git diff 9f94acf..9174a74` (the consolidation commit, `docs: consolidate the drift guard's coverage prose into one COVERAGE block`), both `src/agents_md_drift.rs` only.
Worktree: `.claude/worktrees/rev3-pdg-verify`, detached at `9174a74`. Every mutation below was reverted with the Edit tool (files I created were removed by path); `git status --porcelain` and `git diff` are both empty at the end, with only this findings file untracked.

## Summary

| Round-2 finding | Verdict |
| --- | --- |
| `V2-1` (`is_hard_start` "does NOT affect correctness") | ABSORBED, by deletion |
| `A2-1` (header quantified over deployed prompts) | ABSORBED, routed into `R1` |
| `A2-2` ("the authoritative asset list" pointer) | ABSORBED, pointer removed |
| Unflagged "Only prettier's own freedoms ... are discarded" | CORRECTED, line-ending style now named |

| Property required of the consolidation | Delivered |
| --- | --- |
| 1. Guarded set defined OPERATIONALLY | Yes |
| 2. Complement stated as a RULE | Yes |
| 3. Residuals NUMBERED and cited, not re-derived | Yes |
| 4. No quantifier in comments outside the COVERAGE block | Yes, grep verified live |

COVERAGE claims fact-checked: 21. VERIFIED: 20. NOT VERIFIED: 1 (`V3-1`).

TWO findings, both `low`, both doc-only, both in text this commit wrote:

- `V3-1`, severity `low`: the `R1` sentence's trailing clause "the suite stays green" does not reproduce for three of the four routes it is applied to.
- `V3-2`, severity `low`: the consolidation rewrote a correct one-directional claim in `normalize_wrapping`'s safety argument ("normalize equal ONLY WHEN ...") into a biconditional ("just when ..."), and the added direction is false.

Severities found: `critical` NONE. `high` NONE. `medium` NONE. `low` TWO.

NO MECHANISM DEFECT FOUND. The consolidation is code-identical to the mechanism commit, both pre-existing guarded files are still guarded (proved by mutation), `R1` and `R2` are documented and not fixed, and no exclusion or special case was added.

## Checks

`cargo test`: 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed. Matches the expected 379.
`cargo clippy --all-targets -- -D warnings`: clean, exit 0.
I ran no `nix fmt` and no `just fmt`, and edited no source, plan, or pack file as a deliverable.

## 1. Comment-only. CONFIRMED, and stronger than the diff alone shows

Strip every comment line (`//`, `///`, `//!`) from both revisions and diff the remainder:

    git show 9f94acf:src/agents_md_drift.rs | grep -vE '^[[:space:]]*//' > old.rs
    git show 9174a74:src/agents_md_drift.rs | grep -vE '^[[:space:]]*//' > new.rs
    diff old.rs new.rs     ->  EMPTY

The one way that filter can lie is a line inside a string literal beginning with `///` or `//!`, which the filter would strip. `grep -nE '"[^"]*(///|//!)'` over the new revision returns nothing (exit 1), so no such line exists. `git diff 9f94acf..9174a74 --name-only` returns `src/agents_md_drift.rs` alone.

I also ran the same filter against the MECHANISM commit rather than only against round 2's fix:

    git show 6d5d220:src/agents_md_drift.rs | grep -vE '^[[:space:]]*//' > mech.rs
    diff mech.rs new.rs     ->  EMPTY

So the code has not moved by one byte since the mechanism landed. Every subsequent commit in this step, round 1's fix, round 2's fix, and this consolidation, is prose.

## 2. The three round-2 findings

### `V2-1`. ABSORBED, by deletion, which is what triage asked for

The false sentence existed at `9f94acf:src/agents_md_drift.rs:209-210`:

    /// Precision here affects only how closely the canonical form mirrors prettier and
    /// how readable a failure diff is; it does NOT affect correctness.

`grep -n 'affect correctness\|Precision here affects\|readable a failure diff'` over `9174a74` returns nothing (exit 1). Nothing equivalent survives: the whole "precision is presentational" framing is gone, and `is_hard_start`'s doc (`src/agents_md_drift.rs:232-236`) now says the opposite and cites the residual by number:

    /// The set of markers this recognises is part of residual R2 in COVERAGE, not a
    /// presentation detail: a line-structured construct missing from the list above is
    /// classified soft and joined onto the preceding logical line, which is how R2 is
    /// reached.

I checked the SECOND site triage named as well (`9f94acf:src/agents_md_drift.rs:400-401`, "Asserted on both sides so the guard fails loudly the day such a construct enters the guidance"; triage cited it as `:401-402` against commit `38d9db4`). The text at `src/agents_md_drift.rs:387-392` keeps the same words, but the paragraph it points at (`assert_no_unprotected_construct`'s doc, `:176-179`) now carries the scope limit explicitly ("It is a per-line check, so it does not cover the class of constructs described by residual R2 in COVERAGE. Do not read a pass here as the precondition being fully established"). Triage's stated reason for calling that site weaker was that its "(see its doc comment)" pointer leads to corrected text; that is now the case. CLOSED.

### `A2-1`. ABSORBED, and the property is now stated as a route into `R1`

The quantifier existed at `9f94acf:src/agents_md_drift.rs:1-3` ("and every deployed role prompt under `.agents/prompts/`"). `grep -n 'every deployed'` over `9174a74` returns nothing. The header (`:1-5`) now defers instead of quantifying, and the denied property appears exactly once, as `R1` (`:71-73`):

    //! R1, THE DERIVED-SET RESIDUAL (accepted, not a defect to fix here). Check 3 maps render
    //! -> committed and asserts nothing in the other direction, so a committed file under the
    //! prefix that the pinned render does not emit is invisible to it.

I reproduced `A2-1`'s own evidence to confirm the residual is still real (it must be, or the new text would be the overclaim): copying `pack/prompts/checks-reviewer.md` to `.agents/prompts/checks-reviewer.md` and appending `THIS DEPLOYED ROLE PROMPT IS STALE AND CONTRADICTS ITS PACK SOURCE.` leaves `cargo test` at 379 passed, 0 failed. Reverted.

### `A2-2`. ABSORBED, pointer removed

`grep -n 'builtin_manifest_lists_the_expected_assets\|authoritative'` over `9174a74:src/agents_md_drift.rs` returns nothing (it returned `:84-85` and `:438` on `9f94acf`). No completeness offer replaced it; the complement is a rule (see property 2 below), and the `docs/plans/TEMPLATE` family appears as an illustration explicitly labelled non-bounding.

### The unflagged "Only prettier's own freedoms" claim. CORRECTED

`9f94acf:src/agents_md_drift.rs:293-295` read "Only prettier's own freedoms, where a line is wrapped, how many spaces sit between words, how many blank lines separate blocks, are discarded." `grep -n 'Only prettier'` over `9174a74` returns nothing. The replacement (`:307-309`) drops the quantifier and names the missing member:

    /// What the comparison discards on both sides: where a line is wrapped, how many
    /// spaces sit between words, how many blank lines separate blocks, and line-ending style,
    /// since `str::lines()` strips a trailing CR so a CRLF and an LF file normalize alike.

VERIFIED empirically through the module's real helpers (temporary probe, since reverted):

    raw equal = false
    normalized equal = true
    precondition rejects crlf = false

`"# T\r\n\r\npara one\r\n\r\n- item\r\n"` and its LF twin differ as bytes and normalize identically, and the precondition does not object, which is exactly the stated behaviour. Also correct for fenced lines, which are pushed from `input.lines()` and so have the CR stripped on both sides too.

## 3. Fact-check of the COVERAGE block

`src/agents_md_drift.rs:34-101`. Twenty-one claims. All evidence below is re-runnable; behavioural claims were tested rather than read.

| # | Claim (line) | Verdict |
| --- | --- | --- |
| 1 | Three comparisons make up the drift coverage (`:40`) | VERIFIED |
| 2 | The other tests in this file add none (`:40-41`) | VERIFIED |
| 3 | Checks 1 and 2 are `AGENTS.md` and `.agents/AGENTS.reference.md` (`:43-44`) | VERIFIED, by mutation |
| 4 | Check 3 filters rendered assets by `PROMPT_DEST_PREFIX` against `CARGO_MANIFEST_DIR` (`:45-46`) | VERIFIED |
| 5 | The render is the pinned self-scaffold config (`:48`) | VERIFIED |
| 6 | Both sides go through `assert_no_unprotected_construct` then `normalize_wrapping` (`:49-50`) | VERIFIED |
| 7 | Check 3 is self-extending for a new `[[asset]]` row under the prefix (`:50-53`) | VERIFIED, by mutation |
| 8 | Checks 1 and 2 use `include_str!`; check 3 cannot, so it reads the working tree (`:53-56`) | VERIFIED |
| 9 | The complement rule and its four illustrations (`:58-62`) | VERIFIED |
| 10 | Widening to the Markdown copies is a filter change; they already satisfy the precondition (`:63-65`) | VERIFIED, by probe |
| 11 | `.agents/principles.toml` carries lines outside canonical form that the precondition rejects (`:65-69`) | VERIFIED, by probe |
| 12 | Check 3 asserts nothing render <- committed, so an unemitted committed file is invisible (`:71-73`) | VERIFIED, by mutation |
| 13 | Route: deleting an asset row (`:73`) | VERIFIED for the guard; see `V3-1` |
| 14 | Route: module-tagging a row (`:74-75`) | VERIFIED for the guard; see `V3-1` |
| 15 | Route: changing a row's `dest` (`:75`) | VERIFIED for the guard; see `V3-1` |
| 16 | Route: hand-placing a stale extra file (`:75-76`) | VERIFIED, suite stays green |
| 17 | The mirror case is harmless: an unregistered `pack/prompts/` file never ships (`:77-79`) | VERIFIED, by mutation |
| 18 | `checks-reviewer.md` is the standing benign instance and needs no explicit exclusion (`:79-82`) | VERIFIED |
| 19 | The non-vacuity assertion catches total collapse, not partial (`:82-84`) | VERIFIED, by mutation, both halves |
| 20 | `R2`: per-line precondition, `is_hard_start` misses raw HTML, prettier keeps such a block verbatim (`:86-93`) | VERIFIED, incl. prettier 3.6.2 |
| 21 | The tightening that would close `R2` was measured to reject an ordinary soft-wrapped paragraph (`:95-98`) | VERIFIED, re-implemented |

The evidence, claim by claim where it is not a plain code read.

CLAIMS 1 AND 2. Five `#[test]` functions live in this module. `the_committed_scaffold_matches_a_fresh_render` performs checks 1 and 2, `the_committed_role_prompts_match_a_fresh_render` performs check 3, and `normalization_tolerates_wrapping_but_not_content_change`, `precondition_rejects_non_space_whitespace_and_round_one_cases`, and `precondition_exempts_fenced_indented_lines_but_not_bare_ones` operate only on string fixtures written inline. None of the latter three reads a committed file or calls `self_scaffold_assets`. Exactly three comparisons, no more.

CLAIM 3, BY MUTATION. I changed one word in BOTH `AGENTS.md` and `.agents/AGENTS.reference.md` (`instrumentation was enabled` -> `instrumentation was DISABLED`, the last line of each file) and ran the test:

    assertion `left == right` failed: root AGENTS.md has drifted from a fresh pack render (ignoring prettier wrapping); run `just scaffold-self`
    test result: FAILED. 0 passed; 1 failed

Both mutations reverted with the Edit tool. The two pre-existing guarded files are still guarded, and the consolidation did not quietly disarm them.

CLAIM 5. `self_scaffold_assets` pins `pack::Detail::Summary`, `&HashMap::new()` for vars, `true` for instrument, `&[]` for modules, and `pack::resolve_selection(&principles, "default")`. The justfile recipe (`justfile:46-47`) is `cargo run -- scaffold --output-dir . --write --force --principles default --instrument`. `--principle-detail` defaults to `Detail::Summary` (`src/main.rs:402`), `--principles` defaults to `"default"` (`src/main.rs:399`), and no `--var` or `--module` is passed. The remaining flags (`--output-dir`, `--write`, `--force`) select where output lands, not what is rendered. The configs match. I did not raise the duplication itself, which is already settled.

CLAIM 7, BY MUTATION, IN THREE STEPS. I added a new `[[asset]]` row to `pack/pack.toml` (`source = "prompts/zz-probe.md"`, `dest = ".agents/prompts/zz-probe.md"`, `ownership = "reference"`) plus its pack source, and touched `src/agents_md_drift.rs` not at all.

    (a) row + pack source, no committed copy:
        panicked at src/agents_md_drift.rs:162:13:
        failed to read the committed .agents/prompts/zz-probe.md at <root>/.agents/prompts/zz-probe.md:
        No such file or directory (os error 2). The self-scaffold render produces this file, so the
        repo must commit it; run `just scaffold-self`
        test result: FAILED

    (b) committed copy added:
        test result: ok. 1 passed; 0 failed

    (c) pack source then edited, copy left stale:
        assertion `left == right` failed: .agents/prompts/zz-probe.md has drifted from a fresh
        render of the pack's prompts (ignoring prettier wrapping) ...
        test result: FAILED

The new row is not merely noticed, it is genuinely guarded in both directions, with zero edits to the guard. Row and both files removed; tree clean.

CLAIMS 10 AND 11, BY PROBE. A temporary test running the module's real `assert_no_unprotected_construct` over the unguarded copies:

    .agents/user-prompts/kickoff.md: rejected=false
    .agents/user-prompts/explore.md: rejected=false
    .agents/user-prompts/review.md: rejected=false
    .agents/user-prompts/pause.md: rejected=false
    .agents/user-prompts/compaction-prep.md: rejected=false
    .agents/user-prompts/resume.md: rejected=false
    .agents/LEDGER.template.md: rejected=false
    docs/plans/TEMPLATE._status-narrative.md: rejected=false
    docs/plans/TEMPLATE.motivations.md: rejected=false
    docs/plans/TEMPLATE.principles-note.md: rejected=false
    docs/plans/TEMPLATE.documentation-protocol.md: rejected=false
    docs/plans/TEMPLATE.repo-layout.md: rejected=false
    docs/plans/TEMPLATE.queue-intro.md: rejected=false
    docs/plans/TEMPLATE.roadmap-intro.md: rejected=false
    docs/plans/TEMPLATE.success-criteria.md: rejected=false
    docs/plans/TEMPLATE.steps/example-step.md: rejected=false
    .agents/principles.toml: rejected=true
    .agents/workflow.toml: rejected=false

Every Markdown asset the widened filter could reach already satisfies the precondition, and `.agents/principles.toml` does not, exactly as claimed. The indented array continuations the text names are real (`.agents/principles.toml:248-249`, `:264`, `:514-516`, two-space continuation lines inside multi-line arrays).

ONE THING I CHECKED AND AM NOT RAISING. `docs/plans/TEMPLATE.md` is rejected by the precondition (`rejected=true`), so it would not come along free. It is out of the claim's scope: it is not a manifest asset (it is written by the post-write render loop), so it is not in the set check 3's filter ranges over, and the claim is scoped to "check 3's filter". Reported for the record, not as a defect.

CLAIM 17, BY MUTATION. I placed `pack/prompts/zz-unregistered.md` (a copy of a real prompt) with no `[[asset]]` row. `cargo test`: 379 passed, 0 failed. Nothing in the suite asserts a pack-directory to manifest-row correspondence, so the file is inert, and it has no committed copy to go stale. Removed.

CLAIM 18. `git ls-files .agents/prompts/` returns seven files and `checks-reviewer.md` is not among them, so there is no copy to drift. Its row carries `module = "checks"` (`pack/pack.toml:217-223`), the module-free `load` drops it (`src/manifest.rs:656-664` asserts precisely that), and `grep` over `src/agents_md_drift.rs` finds no exclusion list, skip, or special case. The implicit exclusion is sound, as already settled.

CLAIM 19, BY MUTATION, BOTH HALVES.

    TOTAL collapse: PROMPT_DEST_PREFIX -> ".agents/prompts-moved/"
        panicked at src/agents_md_drift.rs:436:9:
        the self-scaffold render dropped no asset under .agents/prompts-moved/, so this guard
        checked nothing; if the role prompts moved, point PROMPT_DEST_PREFIX at their new destination
        test result: FAILED     ->  caught

    PARTIAL collapse: module = "checks" added to the reviewer.md row (7 prompts -> 6)
        cargo test --bin agent-scaffold agents_md_drift
        test result: ok. 5 passed; 0 failed     ->  NOT caught

Both reverted. The assertion's stated scope is exactly right.

CLAIM 20, IN THREE PARTS.

`is_hard_start` misses raw HTML, and the masking is real, through the module's own helpers:

    is_hard_start("<pre>")  = false
    is_hard_start("</pre>") = false
    precondition rejects multi  = false
    precondition rejects single = false
    MASKED (normalize equal) = true

for `"# T\n\n<pre>\nline one\nline two\n</pre>\n"` against `"# T\n\n<pre> line one line two </pre>\n"`.

Prettier keeps such a block verbatim: VERIFIED INDEPENDENTLY, prettier 3.6.2 with the repo's own `.prettierrc.json` (`{"proseWrap": "never"}`), in a scratch directory outside the repo. The soft-wrapped paragraph in the same file was joined onto one line; the `<pre>` block and a `<div>` block came back byte-identical and still multi-line. So the multi-line form is a fixed point of the formatter and the difference is not attributable to reflow.

"No guarded file carries such a construct today, so R2 is latent": VERIFIED, and by a stronger measure than an HTML grep. Across all nine guarded files, zero lines begin with `<` after `trim_start()`, AND zero lines are classified as continuations at all (I re-ran the transform's own hard-start/soft-line classification and counted joins per file: 0 for every one). `normalize_wrapping` performs no cross-line join anywhere in the guarded set today, so `R2` cannot be reached by any construct, not only by HTML. Latent is correct.

CLAIM 21, BY RE-IMPLEMENTATION. I re-implemented the tightening from its own description ("treat an unrecognised line-structured block as verbatim", i.e. reject a non-blank, non-fenced, non-hard-start line that directly follows a non-blank line) rather than trusting the prior rounds' numbers:

    proposed rejects an ordinary soft-wrapped paragraph = true
    current precondition rejects that same paragraph    = false
    proposed rejects the raw HTML block (would close R2) = true
    proposed rejects <each of the nine guarded files>    = false

So the tightening does close `R2`, does accept the tree as it stands today, and does reject an ordinary soft-wrapped paragraph, which is the incidental formatter reflow the brief requires the guard to tolerate (`prompt-drift-guard.md:11`). Every clause of the sentence reproduces. Probe reverted.

## 4. The four required properties

PROPERTY 1, OPERATIONAL DEFINITION. DELIVERED. The guarded set is given as two named files plus "each rendered asset whose `dest` starts with `PROMPT_DEST_PREFIX`" (`:45`), with the render named by function (`self_scaffold_assets`, `:48`) and the read side named by macro and by directory root (`:53-56`). No natural-language category is used. The one place a category could have crept back, the module header, defers instead (`:2-3`, "the role prompts the COVERAGE block at the end of this comment defines").

PROPERTY 2, COMPLEMENT AS A RULE. DELIVERED, and the anti-list is explicit (`:58-62`):

    //! COMPLEMENT, AS A RULE. Anything else the scaffold emits, or the repo commits, is
    //! unguarded by this module. A rule rather than an inventory, because an inventory carries
    //! an obligation to stay complete that prose reliably fails; the ... illustrate the rule and
    //! do not bound it.

PROPERTY 3, NUMBERED RESIDUALS. DELIVERED. `R1` and `R2` are defined once each (`:71`, `:86`) and cited, never re-derived, at nine other sites: `:99`, `:124`, `:134`, `:158`, `:178`, `:232`, `:234`, `:236`, `:313`, `:315`, `:427`. I read each citation; none restates the residual's content beyond one clause of context.

PROPERTY 4, THE GREPPABLE INVARIANT. DELIVERED. Partition the file at the block boundaries (`:34` `COVERAGE. Stated once`, `:101` `End of COVERAGE.`), keep comment lines only, and grep:

    awk 'NR<34 || NR>101' src/agents_md_drift.rs | grep -E '^[[:space:]]*//' \
      | grep -inE '\b(every|only|authoritative|exhaustive|all)\b'
    ->  no output (exit 1), over 212 comment lines

LIVENESS OF THE FILTER, because an empty scan proves nothing on its own. The identical filter pointed INSIDE the block returns hits:

    awk 'NR>=34 && NR<=101' src/agents_md_drift.rs | grep -E '^[[:space:]]*//' \
      | grep -inE '\b(every|only|authoritative|exhaustive|all)\b'
    ->  2 hits, over 68 comment lines (file lines :38 and :83, both the word "all")

Whole-file, the same grep returns exactly those two lines and nothing else, so no outside line was lost to a partition error. The invariant holds and the scan that shows it holds is demonstrably live.

I also ran a WIDER sweep than the specified list (`never`, `always`, `none`, `each`, `any`, `not total`, `complete`, `full`) over the same outside-block comment lines. Every hit is a mechanical description of the transform ("it never deletes, adds, or reorders a non-whitespace character", "each hard-start line starts a new logical line", "prettier never reflows code"), not a quantifier over the coverage set. No leak.

## 5. Regressions and settled items

MECHANISM UNCHANGED. Proved above by the all-comment strip against `6d5d220`, the mechanism commit: the non-comment text is byte-identical. So no predicate, no test, no assertion message, and no signature moved. `PROMPT_DEST_PREFIX` is still `".agents/prompts/"`, `is_hard_start`'s marker set is unchanged, and `assert_no_unprotected_construct`'s canonical-form predicate is unchanged.

RESIDUALS DOCUMENTED, NOT FIXED. `grep` over the module finds no exclusion list, no skip, no allowlist, and no module special case. `R1` is still reachable (mutation under `A2-1` above: 379 passed with a contradicting extra file in `.agents/prompts/`) and `R2` is still reachable (masking probe under claim 20). Neither was quietly closed under cover of a doc change.

TWO PRE-EXISTING GUARDED FILES STILL GUARDED. Proved by mutation, see claim 3.

SETTLED ITEMS NOT RE-RAISED. I did not raise: `R1` or `R2` as defects; the `checks-reviewer.md` implicit exclusion; the render-config duplication with the justfile; `R3-CQ-1`; the `src/checks.rs` runner-worktree name collision; formatter reflow or line length. I also did not raise the presence of the two deliberately-retained inherited paragraphs, and I fact-checked both rather than only noting them:

- The "Empirically, at the time this guard was written the raw render is already byte-identical to both committed files" paragraph (`:27-32`) is STILL TRUE, and is true of more files than it claims. Byte compare through the module's own helpers: `AGENTS.md` true, `.agents/AGENTS.reference.md` true, and all seven prompt copies true as well. Its supporting reason ("the pack authors each paragraph on a single line, so `proseWrap=never` is a no-op on them") is corroborated by the zero-join count reported under claim 20.
- The canonical-form explanation (`:181-190`) matches the code exactly: the predicate is `line == line.split_whitespace().collect::<Vec<_>>().join(" ")`, and the two regression tests pin the tab and NBSP cases the paragraph claims it catches.

Neither is factually wrong, so neither is a finding.

## `V3-1`: three of the four `R1` routes do NOT leave the suite green

SEVERITY: `low`. Doc-only. In text this commit wrote.

SITE: `src/agents_md_drift.rs:73-76`, the trailing clause of the `R1` route sentence.

    //! prefix that the pinned render does not emit is invisible to it. Reached by deleting an
    //! asset row from `pack/pack.toml`, by module-tagging one (the pinned config selects no
    //! modules, so a tagged row is not rendered), by changing a row's `dest`, or by hand-placing
    //! a stale extra file in `.agents/prompts/`: the copy is orphaned and the suite stays green.

The colon applies "the copy is orphaned and the suite stays green" to all four routes. I ran all four. The orphaning half holds every time; the suite half holds once.

EVIDENCE. Each mutation applied alone to a clean tree, `cargo test` run in full, then reverted with the Edit tool.

| Route | `cargo test --bin agent-scaffold agents_md_drift` | Full `cargo test` |
| --- | --- | --- |
| Delete the `.agents/prompts/reviewer.md` `[[asset]]` row | 5 passed | FAILED, 1 failure |
| Add `module = "checks"` to that row | 5 passed | FAILED, 2 failures |
| Change that row's `dest` to `.agents/reviewer.md` | 5 passed | FAILED, 1 failure |
| Copy `pack/prompts/checks-reviewer.md` into `.agents/prompts/` and append a contradicting line | 5 passed | 379 passed, 0 failed |

The failing test in rows 1 and 3 is `manifest::tests::builtin_manifest_lists_the_expected_assets`; row 2 adds `manifest::tests::builtin_checks_module_adds_its_five_assets`. That test (`src/manifest.rs:584-622`) asserts an exact 30-entry `dest` list over the module-free `load`, so any row deletion, `dest` change, or module tag trips it immediately.

WHY IT MATTERS RATHER THAN BEING PEDANTRY. The block's own charter is "Write a coverage claim here or not at all", and every other site now cites `R1` instead of re-deriving it, so this sentence is the only description of the residual a reader will ever get. As written it says the first three routes are silent from the first keystroke. They are not; they are loud, on a test that forces the maintainer to visit the manifest expectations. Only the fourth route, the hand-placed file, is silent from the outset. That difference is decision-relevant for anyone scoping a fix for `R1`: it is the difference between four unguarded paths and one.

THE STEELMAN, WHICH I TAKE SERIOUSLY AND WHICH DOES NOT FULLY RESCUE THE SENTENCE. `builtin_manifest_lists_the_expected_assets` is a hand-maintained mirror of the manifest that MUST be updated as part of any legitimate row change. Once the maintainer does that, the suite is green and the copy is orphaned, which is the end state the sentence describes; and what fires is a manifest-membership check, never a detection of the orphan itself. So the residual's substance is intact under every route. What is inaccurate is the claim as literally written and as a reader will time it. I would accept a triage verdict that this is a compressed-but-defensible statement; I would not accept passing it unremarked, because it is a testable claim in the file's single authoritative coverage statement and it does not reproduce.

WHAT A FIX WOULD ACHIEVE (one clause, no mechanism change). Either scope the clause to this guard ("check 3 stays green" rather than "the suite stays green"), or split the routes: note that the three `pack.toml` routes also require updating `builtin_manifest_lists_the_expected_assets`, after which the orphan is unnoticed, while the hand-placed file is unnoticed from the start. Nothing about `R1`'s accepted status changes either way.

NOT A RE-RAISE. I accept `R1` in full and ask for no exclusion, no reverse-direction check, and no new test. The finding presupposes the residual.

## `V3-2`: the consolidation turned a correct one-directional claim into a false biconditional

SEVERITY: `low`. Doc-only. In text this commit wrote, replacing text that was correct.

SITE: `src/agents_md_drift.rs:302-304`, inside `normalize_wrapping`'s safety argument.

    /// lines through verbatim. Two inputs then normalize equal just when they carry the same
    /// ordered stream of non-whitespace characters, the same block-boundary structure up to
    /// blank-run collapsing, and byte-identical fences.

The predecessor text (`9f94acf:src/agents_md_drift.rs:287-290`) read "two inputs normalize equal ONLY WHEN they carry the identical ordered stream ...". "Only when" states the necessary condition, which is the direction the safety argument needs and which is true. "Just when" states a biconditional. The added direction is false.

THAT THE AUTHOR MEANS "JUST WHEN" AS A BICONDITIONAL IS NOT MY IMPUTATION. The same file uses it that way, correctly, twenty lines below: `src/agents_md_drift.rs:344` reads "Consecutive blanks collapse to one boundary, recorded just when the last emitted item is not already one", describing `if out.last().is_some_and(|line| !line.is_empty())`, which is an exact iff.

EVIDENCE, a counterexample satisfying the right-hand side while normalizing unequal. Temporary probe over the module's real helpers, since reverted:

    A = "- a\n- b\n"      B = "- a - b\n"
    A precondition rejected = false
    B precondition rejected = false
    same non-whitespace stream = true       stream = "-a-b"
    A blocks = 1                            B blocks = 1
    (no fences on either side)
    normalize A = "- a\n- b"
    normalize B = "- a - b"
    normalize equal = false

Both inputs satisfy the precondition the sentence is scoped to ("Given the precondition ... Two inputs THEN normalize equal just when"), both carry the same ordered stream of non-whitespace characters, both are a single block, and neither has a fence. The biconditional says they normalize equal. They do not, because `is_hard_start` splits A into two logical lines and leaves B as one.

WHY IT IS `low` AND NOT HIGHER. The direction that fails is the safe one. The transform is STRICTER than the sentence claims, so the failure mode is a false FAILURE, not a false pass, and the implication the guard actually rests on (normalize equal -> same content) is untouched. The counterexample is also not reachable through prettier, which would never join two list items. A reader is misled about how tight the transform is, not about whether it can mask drift.

WHAT A FIX WOULD ACHIEVE. Restore the one-directional form: "only when" (or "normalize equal only if they carry ..."). One word.

## Round outcome

TWO findings, `V3-1` and `V3-2`, both `low`, both comment-only, both against text this commit introduced. Neither requires a behaviour change, a new test, or a changed test.

The three round-2 findings are genuinely absorbed rather than reworded: `V2-1`'s sentence is deleted and its function's doc now argues the opposite and cites `R2`; `A2-1`'s quantifier is gone and the property it denied is the definition of `R1`; `A2-2`'s pointer is gone with no completeness offer put in its place. The previously-unflagged "Only prettier's own freedoms" claim was corrected and the correction is empirically right. All four consolidation properties were delivered, including the greppable invariant, whose scan I verified is live rather than vacuous.

This is the first round in this step to find no mechanism defect AND no false claim in the block that is meant to own the coverage statement, with two exceptions both of which are narrower than any prior round's findings: one is a scope word ("the suite" for "this guard") and one is a single connective ("just when" for "only when"). Whether that clears the bar for a clean round is the triager's call, not mine.

Tree state: clean. `git status --porcelain` shows only this findings file; `git diff` is empty; HEAD is `9174a74`. Every mutation was reverted with the Edit tool, and the four files I created (`pack/prompts/zz-probe.md`, `.agents/prompts/zz-probe.md`, `pack/prompts/zz-unregistered.md`, `.agents/prompts/checks-reviewer.md`) were removed by path. `cargo test` on the final clean tree: 379 passed, 0 failed. `cargo clippy --all-targets -- -D warnings`: clean, exit 0.
