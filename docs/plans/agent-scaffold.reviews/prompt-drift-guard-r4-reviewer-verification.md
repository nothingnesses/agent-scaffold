# Step 92 `prompt-drift-guard`: work review round 4, reviewer (deletion verification and residue lens)

Artifact: `git diff 0517838..90b1527` (primary), `git diff 149d415..90b1527` (whole step). Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev4-pdg-verify`, detached at `90b1527`.

## Verdict

ZERO findings. No `low`, no `medium`, no `high`, no `critical`.

Every one of the four fixes reproduces as described, the `:312` judgement call is CORRECT and I verified it independently rather than accepting the implementer's reasoning, the deletion-only and comment-only disciplines hold under mechanical proof, the invariant scan returns exactly the one recorded expected exemption, and the settled items are untouched. `cargo test` is 379 passed 0 failed; `cargo clippy --all-targets -- -D warnings` is clean.

I state explicitly, per the reviewer prompt, that I found nothing at any severity rather than having nothing to say: I went looking for a fifth manufactured defect (each of rounds 1 to 3 produced one) and the deletion-only constraint appears to have worked. The one thing I would have raised, had it been in scope, is already routed (see "Not findings" below).

## Fix 1 (`V3-1` / `RD-1`): the "suite stays green" clause. CLOSED

The clause is gone and nothing replaced it. `src/agents_md_drift.rs:71-76` now reads, in full:

    R1, THE DERIVED-SET RESIDUAL (accepted, not a defect to fix here). Check 3 maps render
    -> committed and asserts nothing in the other direction, so a committed file under the
    prefix that the pinned render does not emit is invisible to it. Reached by deleting an
    asset row from `pack/pack.toml`, by module-tagging one (the pinned config selects no
    modules, so a tagged row is not rendered), by changing a row's `dest`, or by hand-placing
    a stale extra file in `.agents/prompts/`.

Command: `git diff --word-diff=plain --word-diff-regex='[^[:space:]]+' 0517838..90b1527 -- src/agents_md_drift.rs` shows the site as `[-`.agents/prompts/`: the copy is orphaned and the suite stays green.-]{+`.agents/prompts/`.+}`. The only surviving character change is the terminator (`:` -> `.`). No replacement clause.

DID THE DELETION LOSE INFORMATION? No, and I checked this against the residual's definition rather than taking the implementer's word.

The residual `R1` is "check 3 is one-way in set membership". The surviving preceding sentence states it in full and operationally: "Check 3 maps render -> committed and asserts nothing in the other direction, so a committed file under the prefix that the pinned render does not emit is invisible to it." That is the whole residual. The deleted clause added no residual content; it added a claim about the SUITE's colour, which is a different proposition and was false on three of the four routes it was attached to (round 3's triage table, `prompt-drift-guard-r3-triage.md:51-57`).

The stronger result, which is why I call this properly closed rather than merely shortened: with the clause gone, the sentence is now TRUE ON ALL FOUR ROUTES, including the within-prefix `dest` rename that defeated `RD-1`'s proposed alternative wording. Each route does produce a committed file under the prefix that the pinned render does not emit, and check 3 does not see it, regardless of whether some other test goes red for an unrelated reason. Route 4 re-verified live in this worktree just now:

    printf '# Stale orphan\n\nThis file has no asset row.\n' > .agents/prompts/zz-r4-stale-orphan.md
    cargo test --bin agent-scaffold agents_md_drift
    -> test result: ok. 6 passed; 0 failed  (6 because a temporary probe test was present; 5 without it)
    rm .agents/prompts/zz-r4-stale-orphan.md   # reverted, tree clean

Routes 1 to 3 were reproduced by round 3's triager on a clean tree with the same outcome for this module (5 passed in every case except the within-prefix rename, where check 3 fails about the NEW dest, not about the orphan). I did not re-run them; the surviving sentence makes no claim about test colour, so the only thing that needs to be true is invisibility to check 3, which round 3 verified on all five routes and I re-verified on the one that is cheapest and most direct.

What IS lost is the decision-relevant nuance the round-3 triager offered as an OPTIONAL addition (`prompt-drift-guard-r3-triage.md:82`: "If the human wants the decision-relevant difference recorded, one added clause covers it"). The same paragraph ends "Prefer the deletion." Losing that nuance is therefore the triager's own preferred outcome, chosen knowingly, not an information-loss defect. I judge the preceding sentence sufficient: a reader who needs to know which routes trip some other test can derive it, and any attempt to state it in prose is exactly the authoring that manufactured a defect in each of the three prior rounds.

## Fix 2 (`RD-2`): the loudness promise in the test comment. CLOSED

`src/agents_md_drift.rs:387-391` now ends "Asserted on both sides." The word-diff shows `[-sides so the guard fails loudly the day-] [-such a construct enters the guidance.-]{+sides.+}`: two clause fragments deleted, one period added, no word authored.

ACCURATE? Yes. "Asserted on both sides" is verified by the four calls immediately below it at `src/agents_md_drift.rs:392-401`: committed `AGENTS.md`, rendered `AGENTS.md`, committed `.agents/AGENTS.reference.md`, rendered `.agents/AGENTS.reference.md`. Both sides of both comparisons.

SUFFICIENT? Yes. The requirement (`prompt-drift-guard-r3-triage.md:116`) was: "Delete the loudness clause ... Do not author a new explanation; `:176-179` already carries it and the site already points there. Deleting the sentence outright satisfies the requirement." The implementer deleted less than it was permitted to and authored nothing, which is the conservative side of the requirement. The site's own pointer at `:387` ("see its doc comment") leads to `normalize_wrapping`'s doc at `:311-316`, which carries the `R2` scope limit, and `assert_no_unprotected_construct`'s own doc at `:177-179` says "It is a per-line check, so it does not cover the class of constructs described by residual `R2` in COVERAGE. Do not read a pass here as the precondition being fully established." The false totality is gone and the true scope is two pointers away in both directions.

## Fix 3 (`RD-3`): the `checks-reviewer` locator. CLOSED

`src/agents_md_drift.rs:80` now reads "whose row is module-gated in `pack/pack.toml`". Verified against BOTH files:

- `pack/pack.toml:219-223` holds the row AND the tag: `[[asset]]`, `source = "prompts/checks-reviewer.md"`, `dest = ".agents/prompts/checks-reviewer.md"`, `ownership = "reference"`, `module = "checks"`. Command: `grep -n "checks-reviewer" -B4 -A4 pack/pack.toml`.
- `src/manifest.rs` holds NO asset row for it. Command: `grep -n "checks-reviewer" -B6 -A6 src/manifest.rs` returns only test-list entries at `:658` (the absent-when-off list), `:677` (a comment), and `:685` (an ownership assertion). Those are assertions ABOUT the gating, not the gating data.

The rest of the sentence still checks out: `ls -1 .agents/prompts/` returns seven files with no `checks-reviewer.md`, so "the repo commits no copy for it to drift from" holds, and `grep -n "checks-reviewer\|exclude\|skip" src/agents_md_drift.rs` shows the module contains no exclusion for it, so "it needs no explicit exclusion and has none" holds.

## Fix 4 (`V3-2`): "just when" -> "only when". CLOSED, and the other "just when" is correctly left alone

`src/agents_md_drift.rs:302`: "Two inputs then normalize equal only when they carry the same ordered stream of non-whitespace characters, the same block-boundary structure up to blank-run collapsing, and byte-identical fences."

"A only when B" is A -> B, the necessary condition, which is the direction the safety argument needs (normalize equal -> same content) and which is true: the transform never deletes, adds, or reorders a non-whitespace character, so equal normalized output implies equal token stream. The converse is FALSE and would have been asserted by "just when" (iff). Counterexample, from the code's own semantics at `:355-365`: `"- a\n- b"` and `"- a - b"` carry the same ordered non-whitespace stream and the same single-block structure, yet normalize to `"- a\n- b"` and `"- a - b"` respectively, because `is_hard_start` splits one and not the other. So B does not imply A, and only the one-directional "only when" is correct. Restored wording verified correct.

THE SEPARATE "just when" AT `:344-346` IS GENUINELY AN IFF AND CORRECTLY UNTOUCHED. The comment reads "Consecutive blanks collapse to one boundary, recorded just when the last emitted item is not already one." The code immediately below is `if out.last().is_some_and(|line| !line.is_empty()) { out.push(String::new()); }` (`:346-348`). A boundary is pushed exactly when that guard is true and never otherwise, so the biconditional is what the code implements. The empty-`out` case (a leading blank line, `out.last()` is `None`, nothing pushed) is consistent with the comment under the reading that a non-existent last item is not a non-boundary. This is a mechanical statement, not a coverage claim, and is exempt under the amended invariant regardless. Not a finding.

## THE JUDGEMENT CALL: `:311-313` is NOT the same defect as `RD-2`. Implementer's determination UPHELD

The site, `src/agents_md_drift.rs:311-316`:

    `assert_no_unprotected_construct` asserts that precondition on both sides of each
    comparison and fails loudly the day guidance gains one of those constructs, so the
    gap is a fail-safe rather than a silent hole. The class it cannot see is residual R2
    in COVERAGE. Harden this transform (make list indentation significant, treat indented
    code and HTML blocks verbatim, without losing the soft-wrap tolerance R2 explains)
    before adding such content to a guarded file.

I verified the two things that decide it, independently.

1. WHAT "THOSE CONSTRUCTS" REFERS TO. Anaphora resolves to the nearest preceding enumeration, which is the precondition sentence at `:292-296`: "no indentation-significant construct (a nested or continuation-indented list item, a 4-space indented code block) and no whitespace-significant inline construct (a run of two or more spaces, including inside an inline code span)". That is a CLOSED, parenthetically enumerated list, not an open category. Contrast the `RD-2` site, whose "such a construct" pointed back to "any indentation- or whitespace-significant construct" (`:388-389`), an open category that includes the `R2` class. The two sites are not the same defect because their antecedents are not the same class.

2. WHETHER THE PER-LINE CHECK ACTUALLY CATCHES EVERY ENUMERATED MEMBER. This is the load-bearing half and I did not take it on reasoning alone. Temporary probe test added to the `tests` module, run, then reverted with the Edit tool:

    running 6 tests
    test agents_md_drift::tests::r4_temp_probe_enumerated_constructs_versus_r2 ... ok
    ...
    test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 362 filtered out

The probe asserted, through the module's own helpers:

    precondition_rejects("- parent\n  - child")                    == true   nested list item
    precondition_rejects("- parent\n  continuation line")          == true   continuation-indented item
    precondition_rejects("# T\n\n    let indented = code;\n")      == true   4-space indented code
    precondition_rejects("# T\n\n\tlet indented = code;\n")        == true   tab-indented code
    precondition_rejects("a paragraph with  two spaces")           == true   a two-space run
    precondition_rejects("a paragraph with `a  b` inline")         == true   multi-space in an inline code span
    precondition_rejects("# T\n\n<pre>\nline one\nline two\n</pre>\n") == false   the R2 class, as documented
    normalize_wrapping(html_split) == normalize_wrapping(html_joined)          R2 masking still live

Every member of the enumerated list is rejected, including the inline-code-span case, which is the only member with no existing regression test of its own. The one construct that passes is the raw HTML block, and that is precisely the class the NEXT SENTENCE names: "The class it cannot see is residual `R2` in COVERAGE." So the paragraph states a true bounded claim and then names its own exception, in that order, within four lines.

3. THE THIRD THING I CHECKED, WHICH NEITHER SIDE RAISED. "Fails loudly the day GUIDANCE gains one" could over-reach if the assertion did not run on all guarded sides. It does: `:392-401` for checks 1 and 2, and `:448-449` inside check 3's loop for every derived prompt. A construct entering any guarded file is asserted on both the committed and the rendered side before the equality check, and `assert!` panics with the diagnostic at `:220`, so "loudly" is accurate. A construct entering an UNGUARDED file is not asserted, but nothing compares that file either, so nothing is masked and the sentence's "each comparison" scoping already says so.

VERDICT: the implementer's reasoning is correct and its decision to leave `:312` byte-identical was right. No false totality claim survives at that site. Had it "fixed" this site it would have authored a fifth defect, which is the pattern the deletion-only constraint existed to break.

## Deletion-only discipline. PROVEN

COMMENT LINES WENT DOWN, 280 -> 279:

    git show 0517838:src/agents_md_drift.rs > before.rs ; cp src/agents_md_drift.rs after.rs
    grep -c '^[[:space:]]*//' before.rs   -> 280
    grep -c '^[[:space:]]*//' after.rs    -> 279
    wc -l                                 -> 568 before, 567 after

COMMENT-ONLY, by stripping every comment line and diffing the remainder:

    grep -v '^[[:space:]]*//' before.rs > before.nocomment
    grep -v '^[[:space:]]*//' after.rs  > after.nocomment
    diff before.nocomment after.nocomment   -> empty, exit 0
    wc -l                                   -> 288 lines each

THE ONE WAY THAT FILTER COULD LIE, CLOSED. A `///` or `//!` inside a multi-line string literal at line start would be stripped as a comment and hide a code change. There is none, in either revision:

    grep -n '///\|//!' <rev>.rs | grep -v '^[0-9]*:[[:space:]]*///\|^[0-9]*:[[:space:]]*//!'   -> empty, both revisions

so every `///` and `//!` occurrence in the file is line-leading. Stronger still, `grep -n '"[^"]*//' after.rs` returns nothing: no string literal in the file contains `//` at all, so no string content can be mistaken for a comment by any line-based filter.

NO NEW SENTENCE, CLAUSE, OR QUALIFIER ANYWHERE. The word-level diff over the whole change is four lines, reproduced in full:

    git diff --word-diff=plain --word-diff-regex='[^[:space:]]+' 0517838..90b1527 -- src/agents_md_drift.rs

    //! a stale extra file in [-`.agents/prompts/`: the copy is orphaned and the suite stays green.-]{+`.agents/prompts/`.+}
    //! `.agents/prompts/checks-reviewer.md`, whose row is module-gated in [-`src/manifest.rs`,-]{+`pack/pack.toml`,+} so
      /// lines through verbatim. Two inputs then normalize equal [-just-]{+only+} when they carry the same
        // on masked drift. Asserted on both [-sides so the guard fails loudly the day-] [-such a construct enters the guidance.-]{+sides.+}

Total authored content: two sentence terminators (`:` -> `.`, and a `.` after "sides"), one path token corrected, one word reverted to already-reviewed text. Zero new words of prose.

## The amended invariant (semantic: no coverage claim outside the COVERAGE block). HOLDS

The block is `src/agents_md_drift.rs:34-101` (`grep -n "COVERAGE\. Stated once\|End of COVERAGE"` returns `34` and `101`).

    awk 'NR<34 || NR>101 {print NR": "$0}' src/agents_md_drift.rs | grep -E ':[[:space:]]*//' | grep -iE '\b(every|only|authoritative|exhaustive|all)\b'
    302:  /// lines through verbatim. Two inputs then normalize equal only when they carry the same

EXACTLY ONE HIT OUTSIDE THE BLOCK, and it is the RECORDED EXPECTED EXEMPTION from fix 4, not a regression: `prompt-drift-guard-r3-triage.md:181` predicted it verbatim ("After `V3-2`'s fix: one hit outside the block, at roughly `:302`, the restored 'only when', exempt under part 3"). Classified per the amended rule as a logical connective in a mechanical statement about the transform, not a quantifier over the coverage set. Left alone, correctly.

FILTER LIVENESS CONFIRMED (the scan is not silently matching nothing):

    awk 'NR>=34 && NR<=101 {print NR": "$0}' src/agents_md_drift.rs | grep -iE '\b(every|only|authoritative|exhaustive|all)\b'
    38: //! comments to cite. Write a coverage claim here or not at all.
    83: //! prompt test catches check 3 collapsing entirely (the filter matching nothing at all), not

Two hits inside, both the word "all", both fine, and both are the same two hits round 3 recorded (`prompt-drift-guard-r3-triage.md:41`, `:181`). The partition reproduces exactly across rounds.

SEMANTIC SWEEP, SINCE THE WORD LIST IS ONLY AN AID. I read every comment outside the block and classified each for a coverage claim, rather than trusting the grep. The sites that touch coverage all CITE rather than restate: `:1-5` defers to the block by name; `:124-125` (`PROMPT_DEST_PREFIX`) points at COVERAGE for what is guarded, the complement, and `R1`; `:133-134` cites `R1` for the module selection; `:157-158` cites `R1` for the reverse case; `:177-179` cites `R2` for the per-line scope; `:234-236` cites `R2` for `is_hard_start`'s marker set; `:313` cites `R2`; `:376-377` cites checks 1 and 2; `:417` and `:426` cite check 3 and `R1`. The two prose sites that assert something about what the guard catches (`:380-383` "fails on a real content drift, a hand edit, a dropped slot, or a stale pack source"; `:422-426` "two-way in CONTENT ... one-way in SET MEMBERSHIP") are non-exhaustive lists of true triggers with no totality operator, and the second one names its own residual. No leak.

## Regressions and settled items. CLEAN

- MECHANISM UNCHANGED. Proven by the empty non-comment diff above: no predicate, no test, no assertion message, no signature changed between `0517838` and `90b1527`. `is_hard_start`, `normalize_wrapping`, `assert_no_unprotected_construct`, `committed_asset`, `self_scaffold_assets`, all five tests and all their assertion strings are byte-identical.
- `R1` AND `R2` STILL ACCEPTED RESIDUALS, NO EXCLUSION OR GUARD ADDED. `R1`: check 3 at `:427-430` is still a bare prefix filter over the rendered set with no reverse-direction check and no exclusion list; `grep -n "checks-reviewer\|exclude\|skip" src/agents_md_drift.rs` finds no exclusion; and the stale-orphan mutation above kept the module green. `R2`: the probe above shows the HTML block still passes the precondition and still masks a content change under `normalize_wrapping`. Both residuals are exactly as accepted, with nothing quietly added to close them.
- `RD-4` UNTOUCHED AND NOT RE-RAISED. The thematic-break comment at `:255-256` and its predicate at `:257-263` are byte-identical to `0517838` (they do not appear in the diff). It is backlogged as VALID BUT OUT OF SCOPE (`prompt-drift-guard-r3-triage.md:185-187`) and I make no claim about it here.
- BOTH PRE-EXISTING GUARDED FILES STILL GUARDED. `:384-412` still compares committed `AGENTS.md` and `.agents/AGENTS.reference.md` against their fresh renders through the precondition and the normalization, and `the_committed_scaffold_matches_a_fresh_render` passes in the run below.
- DOC CURRENCY. Nothing cites the deleted phrases: `grep -rn "suite stays green"` over the tree (excluding worktrees and this `.reviews/` directory) returns nothing. `grep -rn "fails loudly" src/ pack/ docs/plans/agent-scaffold.steps/ AGENTS.md` shows the two step briefs that describe the fail-safe both scope it to the ENUMERATED constructs ("a nested list, indented code, or a multi-space inline span"), which the probe above proves the per-line check does catch, so neither brief is made stale or is itself wrong on this point.

## Test and clippy

    cargo test
    PASSED 379  FAILED 0  IGNORED 0   (exit 0)

379 exactly, as expected. Summed with `grep -E "^test result:" | awk '{p+=$4; f+=$6; i+=$8}'` over the full output.

    cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.11s   (exit 0, no warnings)

## Not findings (recorded so nothing is lost, deliberately NOT raised)

- The step brief at `docs/plans/agent-scaffold.steps/prompt-drift-guard.md:21` still says `.agents/prompts/checks-reviewer.md` "is module-gated (`src/manifest.rs`, emitted only under `--module checks`)", the same mis-citation `RD-3` corrected in the comment. This is ALREADY ROUTED: round 3's triager recorded it at `prompt-drift-guard-r3-triage.md:140` as "FOR THE PLANNER, NOT THE IMPLEMENTER ... Plan content, planner-owned; route it, do not fold it into this fix." I raise no finding; I note only that it is still outstanding, so the routing is not lost when this step closes.
- `:292-296` states the precondition as "the guarantee holds while the guarded text carries no [indentation-significant] and no [whitespace-significant inline] construct", which taken alone reads as a sufficiency claim that `R2` contradicts. I considered raising it and decline, on evidence rather than deference: the same doc comment names `R2` as the class it cannot see at `:313` and prescribes "treat indented code and HTML blocks verbatim" at `:314-315`, so the paragraph does not claim the enumeration is exhaustive for the guarantee; and round 3's triage certified this exact passage as the site that "now carries the `R2` scope limit" (`prompt-drift-guard-r3-triage.md:114`). Re-raising it would be re-litigation without new evidence, and any rewrite here is exactly the authoring that produced a new defect in each of rounds 1, 2, and 3.
- `src/agents_md_drift.rs:76` is now a short line because a clause was deleted from it. Line length and wrapping are never findings.

## Mutations and tree state

Two mutations, both reverted, both listed so the triager can reproduce and confirm:

1. A temporary `#[test] fn r4_temp_probe_enumerated_constructs_versus_r2` appended to the `tests` module, run once, removed with the Edit tool (never `git checkout`).
2. `.agents/prompts/zz-r4-stale-orphan.md` created for the `R1` route-4 demonstration, then removed (an untracked file; `rm` is its revert).

`git status --porcelain` returns EMPTY. `ls -1 .agents/prompts/` returns the same seven files as at `90b1527`. Tree clean.
