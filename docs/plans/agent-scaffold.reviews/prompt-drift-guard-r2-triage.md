# Step 92 `prompt-drift-guard`: triage verdicts (work review, round 2)

Artifact: `git diff 3164404..38d9db4`, a single-file change to `src/agents_md_drift.rs` (+175 / -25), of which `git diff 97a587c..38d9db4` (+61 / -14) is round 1's comment-only fix commit.
Brief: `docs/plans/agent-scaffold.steps/prompt-drift-guard.md`.
Round-1 verdicts: `docs/plans/agent-scaffold.reviews/prompt-drift-guard-triage.md` (authoritative).
Findings triaged: `prompt-drift-guard-r2-reviewer-verification.md` (`V2-1`) and `prompt-drift-guard-r2-reviewer-adversarial.md` (`A2-1`, `A2-2`). All three were rated `low` by their reviewers.
Worktree: `.claude/worktrees/triage2-pdg`, detached at `38d9db4`. Every mutation below was reverted with the Edit tool (file additions by removing the exact file added); `git status --porcelain` shows only the three review files and `git diff` is empty at the end.

## Summary

| Finding | Reviewer severity | My verdict | My severity | Evidence reproduced |
| --- | --- | --- | --- | --- |
| `V2-1` | low | VALID, doc-only fix | low | Yes, exactly |
| `A2-1` | low | VALID, doc-only fix | low | Yes, exactly |
| `A2-2` | low | VALID, doc-only fix | low | Yes, exactly |

THREE valid findings require an implementer fix. All three are comment-only edits to `src/agents_md_drift.rs`, none changes behaviour, and none requires a new or changed test. I dismissed nothing.

NO BARRED RE-RAISE. I examined the re-raise question closely for `A2-1` and rule that it is not one; reasoning in its section.

BACKSTOP: NOT triggered. No finding is rated high or critical by its reviewer or by me, and I dismissed nothing at any severity, so no second independent triager is required.

## Spot-checks (load-bearing for eventual convergence)

THE CHANGE IS COMMENT-ONLY. CONFIRMED, by a filter stronger than either the implementer's or the verification reviewer's. The verification reviewer stripped `//!` and `///` lines from both revisions and diffed the remainder (empty). I additionally stripped EVERY comment line (`//` as well as `//!` and `///`) from both revisions and diffed:

    git show 97a587c:src/agents_md_drift.rs | grep -vE '^[[:space:]]*//' > old.rs
    git show 38d9db4:src/agents_md_drift.rs | grep -vE '^[[:space:]]*//' > new.rs
    diff old.rs new.rs     ->  EMPTY

I also checked the one way that filter could lie: a line inside a string literal beginning with `///` or `//!` would be stripped and could hide a change. `grep -nE '"[^"]*(///|//!)'` over the new revision returns nothing, so no such line exists. `git diff 97a587c..38d9db4 --name-only` returns `src/agents_md_drift.rs` alone. The change is comment-only.

ROUND-1 FIXES CLOSED. CONFIRMED. I read the fix diff in full. It touches exactly three regions and each corresponds to one round-1 requirement:

- `FN-1` (`src/agents_md_drift.rs:36-43`): "a prompt added to the pack is guarded" -> "a prompt added to the pack MANIFEST is guarded", plus six lines making "manifest" strict. Round 1 asked for the one clause; the fix delivers that and more. CLOSED.
- `CT-1` (`src/agents_md_drift.rs:77-97`): the list is now labelled "NOT AN EXHAUSTIVE LIST", names the `docs/plans/TEMPLATE*` family (round 1's preferred remedy), states that the remainder is LARGER than the guarded set, and adds the per-group widening cost. CLOSED, subject to `A2-2` below, which is about the pointer the fix introduced rather than about the omission `CT-1` reported.
- `FN-2` doc portion (`src/agents_md_drift.rs:150-155` and `:297-326`): the cross-line join is added as construct (d), the per-line scope is stated at the predicate's own doc, and the residual is recorded as accepted with the reason the cheap tightening was rejected. CLOSED, subject to `V2-1` below, which is about two sites the fix did not sweep.

I checked round 1's third `FN-2` instruction separately, since the reviewers split on it: round 1 said the three-item restatement (now `src/agents_md_drift.rs:456-460`) "should either point at that enumeration or not read as exhaustive". It reads "a prompt could gain a nested list, indented code, or a multi-space inline span", which is illustrative rather than closed, and all three items are accurate. The instruction is satisfied. I agree with the verification reviewer and require no change there.

SETTLED ITEMS UNDISTURBED. CONFIRMED by the same all-comment filter: `assert_no_unprotected_construct`, `is_hard_start`, and `normalize_wrapping` are byte-identical as code between `97a587c` and `38d9db4`, `PROMPT_DEST_PREFIX` still equals `".agents/prompts/"`, and no exclusion list or module special case was added. No hardening was slipped in under cover of a doc fix.

`cargo test`: 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed, on the clean tree at `38d9db4`.
`cargo clippy --all-targets -- -D warnings`: clean, exit 0.

ONE CAVEAT ON "TESTS ARE GREEN", recorded because it is load-bearing and neither reviewer saw it. My FIRST `cargo test` run in this worktree failed 2 of 367 in `checks::tests` (`a_format_check_never_mutates_the_live_tree` and `an_empty_paths_array_runs_unscoped`). Five subsequent runs were green. This is a pre-existing flake with nothing to do with the artifact (the whole step touches `src/agents_md_drift.rs` only), and I traced it rather than leaving it as noise; see the out-of-scope section. `cargo test` is green for this artifact, but it is not deterministically green in this repo.

INDEPENDENT RENDER SWEEP. I re-ran the round-1 sweep rather than accepting it: `cargo run -- scaffold --output-dir <tmp> --write --force --principles default --instrument`, then a byte compare of every emitted file against its committed copy. 31 emitted files, 31 SAME, 0 DIFFERS, 0 missing from the repo. The `render docs/plans/TEMPLATE.md` line appears in the output, confirming the 31st file is the generated view rather than a manifest asset. Guarded = 9, unguarded = 22, and `builtin_manifest_lists_the_expected_assets` asserts a 30-entry list, so it can express 21 of the 22.

## `V2-1`: the fix left the precondition's totality asserted at two unswept sites, one of them false

VERDICT: VALID. Severity `low` (reviewer's rating CONFIRMED). Doc-only fix required. NOT a re-raise of the `FN-2` mechanism residual.

EVIDENCE REPRODUCED: yes, exactly, including the one-byte mutation.

Baseline at `38d9db4` unmodified, via a temporary probe calling the module's real `is_hard_start`, `normalize_wrapping`, and `precondition_rejects`:

    is_hard_start("<pre>") = false
    precondition rejects multi : false
    precondition rejects single: false
    normalized multi : "# T\n\n<pre> line one line two </pre>"
    normalized single: "# T\n\n<pre> line one line two </pre>"
    MASKED (normalize equal): true
    word change masked: false

Then changing ONE byte of `is_hard_start` precision at `src/agents_md_drift.rs:218`, `Some(b'#' | b'>' | b'|')` -> `Some(b'#' | b'>' | b'|' | b'<')`, and nothing else:

    is_hard_start("<pre>") = true
    normalized multi : "# T\n\n<pre> line one line two\n</pre>"
    normalized single: "# T\n\n<pre> line one line two </pre>"
    MASKED (normalize equal): false

One byte of precision flips the outcome from "two different inputs compare equal" to "they compare different". Both the mutation and the probe were reverted with the Edit tool; `git diff` is empty.

IS THIS A REAL CONTRADICTION, OR TWO COMPATIBLE CLAIMS? I tested the charitable reading directly, that `:209-210`'s "does NOT affect correctness" is scoped to false FAILURES while the new text at `:302-303` and `:315-317` concerns false PASSES. It does not survive, for three reasons.

1. THE TEXT DOES NOT SCOPE ITSELF. The sentence is "Precision here affects only how closely the canonical form mirrors prettier and how readable a failure diff is; it does NOT affect correctness." There is no restriction to failures anywhere in it. "affects ONLY ... how readable a failure diff is" is an exhaustive claim about what precision can affect.

2. THE SENTENCE'S OWN SUPPORTING ARGUMENT IS THE COUNTEREXAMPLE. It reads "misclassifying a structural line can at most change a newline into a space (or vice versa) on both sides equally". That is true, and it is exactly the masking mechanism: input A holds a newline where input B holds a space, the transform turns both into a space, and the two compare equal. The sentence states the mechanism and then draws the opposite conclusion from it. The closing clause "it can never merge two distinct non-whitespace tokens into one" is also true and is offered as though it closed the false-pass question, but token merging is not how the masking happens, so it does not.

3. THE FILE'S OWN VOCABULARY COUNTS MASKING AS CORRECTNESS. `src/agents_md_drift.rs:22-23` describes the safety argument as being that, under its precondition, the transform "cannot mask a content change". Under the module's own usage of the term, an unrecognised line-structured construct that gets joined and masked IS a correctness failure. So the two statements are not compatible even under the file's internal definitions.

The scoping the reviewer proposes is therefore not merely absent from the text; it is contradicted by the text's own argument.

PROVENANCE, which I checked because it bears on scope and neither reviewer established it. The `is_hard_start` paragraph is PRE-EXISTING, introduced by step 80 (`cba4fcc`), and is untouched by this step's diff. So the false sentence is inherited, not written here. What IS new is the CONTRADICTION: commit `38d9db4` added the text that says the opposite, in the same file, 90 lines away. Before that commit the file was consistently (and wrongly) confident; after it, the file asserts both P and not-P.

WHY IT IS IN SCOPE DESPITE BEING INHERITED TEXT. Three reasons, and I would not rule it in on any one alone.

- The artifact created the contradiction. A reviewer reading `38d9db4` on its own terms finds a file that argues against itself, and the commit that produced that state is the artifact under review.
- Round 1's `FN-2` instruction was to stop the file asserting the precondition's totality. The fix landed that at two sites and did not sweep the other two. This is fix incompleteness against a requirement round 1 set, which is squarely the verification lens's remit.
- The remedy is one clause in a file the artifact already edits, and round 1 set the precedent twice (`FN-1` and `CT-1` were both one-clause narrowings of an overclaim in this same file).

SEVERITY `low`, and I decline to escalate. Nothing misbehaves, reachability is zero today (no guarded file contains a raw HTML block, re-verified), the corrected statement is loud and in the same file, and the fix is comment-only. I will record that within `low` I rate this the most consequential of the three, because `is_hard_start` is precisely the function a maintainer opens when deciding whether a newly added block construct needs recognition, and the sentence sitting there tells them the question does not matter. That is an ordering within `low`, not a case for `medium`.

NOT A BARRED RE-RAISE. The settled item is the `FN-2` MECHANISM residual (the precondition has no cross-line fail-safe). `V2-1` asks for no predicate change, no `normalize_wrapping` change, and no new test, and its reviewer states acceptance of the residual explicitly. A finding that depends on a residual being accepted, and asks only that the prose stop denying it, is not a challenge to the residual's verdict.

WHAT THE FIX MUST ACHIEVE. `src/agents_md_drift.rs:209-214` must stop claiming that `is_hard_start`'s precision cannot affect correctness. The accurate statement is that applying the transform identically to both sides rules out a false FAILURE but not a false PASS, and that a line-structured construct `is_hard_start` does not recognise is joined and can be masked (accepted residual, see the UNPROTECTED CONSTRUCTS paragraph). Secondarily, `src/agents_md_drift.rs:401-402` ("Asserted on both sides so the guard fails loudly the day such a construct enters the guidance") must be narrowed to the per-line scope the predicate has. I confirm the reviewer's assessment that the secondary site is weaker, because its own "(see its doc comment)" pointer now leads to corrected text, but it is the same defect and should be fixed in the same pass. See the structural section before writing either edit.

## `A2-1`: the module header and test name assert a coverage property the accepted `H4-3` residual denies

VERDICT: VALID. Severity `low` (reviewer's rating CONFIRMED). Doc-only fix required. NOT a barred re-raise.

EVIDENCE REPRODUCED: yes, exactly.

    cp pack/prompts/checks-reviewer.md .agents/prompts/checks-reviewer.md
    printf '\nTHIS DEPLOYED ROLE PROMPT IS STALE AND CONTRADICTS ITS PACK SOURCE.\n' \
      >> .agents/prompts/checks-reviewer.md
    ls .agents/prompts/   ->  8 files, checks-reviewer.md among them

    cargo test --bin agent-scaffold agents_md_drift  ->  5 passed, 0 failed
    cargo test                                       ->  379 passed, 0 failed

Eight deployed role prompts under `.agents/prompts/`, one of them explicitly contradicting its pack source, and the entire suite is green. Planted file removed; tree clean.

THE RE-RAISE RULING, which is the question this finding turns on. NOT BARRED. My test is whether the finding contests the settled verdict or depends on it. `H4-3` / `FN-3` is the accepted residual that a prompt outside the module-free render drops silently from the derived set, and round 1's triager independently found this same extra-committed-file shape and classified it there (`prompt-drift-guard-triage.md:163`). `A2-1` accepts that verdict, asks for no mechanism change, no exclusion, and no new test, and would be false if the residual did not exist. A finding whose entire content is "the prose denies an accepted residual" cannot be a re-raise of that residual, because it presupposes it.

The reviewer's appeal to `FN-1` and `CT-1` as the same species also holds, and I checked it rather than accepting the analogy. Round 1 ruled `FN-1` VALID under an explicit heading "WHY THIS IS A DOC DEFECT AND NOT A MECHANISM DEFECT", having established that the unregistered-pack-file mechanism was correct and only the sentence describing it was wrong. That is the identical structure: mechanism sound and settled, sentence overclaims, fix is one clause. Round 1 established that accepting a mechanism limitation does not license prose that denies the limitation. `A2-1` is that rule applied at a site round 1 did not reach.

ONE POINT IN `A2-1`'S FAVOUR THAT IS STRONGER THAN THE REVIEWER CLAIMED, and that I established rather than took. The header text is NOT inherited. `git diff 3164404..38d9db4` shows `src/agents_md_drift.rs:1-3` rewritten by this step, with "and every deployed role prompt under `.agents/prompts/`" added as new text. So unlike `V2-1`, there is no scope question at all here: the overclaim is text this artifact wrote.

REACHABILITY VERIFIED, and the reviewer's argument is concrete rather than hypothetical. `.agents/checks.toml` is tracked (`git ls-files .agents/`) while its `[[asset]]` row carries `module = "checks"` (`pack/pack.toml:196-200`), so the module-free self-scaffold render never emits it. The repo therefore ALREADY commits a file at a module-gated destination. `.agents/hooks/pre-commit` is a second such row (`pack/pack.toml:232-236`). One `scaffold --module checks --write --force` run puts `.agents/prompts/checks-reviewer.md` into the tree by exactly the same route, and from that moment the header is false of the tree and not merely of the mechanism. That is a materially better reachability argument than `FN-1` had, and `FN-1` was ruled VALID.

WHY IT REMAINS `low` and I do not escalate. The header is true of the tree today: my 31-file sweep confirms the deployed set equals the rendered set, and the repo commits no `checks-reviewer.md`. Nothing misbehaves, the lower body of the module doc states the mechanism correctly at `src/agents_md_drift.rs:35-38`, and the fix is comment-only.

WHAT THE FIX MUST ACHIEVE. `src/agents_md_drift.rs:1-3` must not quantify over deployed files. It must claim no more than the mechanism sentence at `:35-38` delivers, for example "every role prompt the module-free render emits under `.agents/prompts/`". I do NOT require the test to be renamed: with the header corrected and the loop's own comment block already accurate, the name is not load-bearing prose. See the structural section, which proposes defining the guarded set operationally rather than restating a corrected description.

## `A2-2`: the "authoritative asset list" pointer cannot express one emitted, committed, unguarded file

VERDICT: VALID. Severity `low` (reviewer's rating CONFIRMED). Doc-only fix required. The reviewer's own "marginal" label is fair, and the finding still stands.

EVIDENCE REPRODUCED: yes, every element.

1. `git ls-files docs/plans/TEMPLATE.md` returns the file. It is committed.
2. It is absent from the pointer's target. `builtin_manifest_lists_the_expected_assets` (`src/manifest.rs:584`) asserts an exact 30-entry dest list; 12 entries match `TEMPLATE`, and a grep for the bare `"docs/plans/TEMPLATE.md"` returns 0 matches.
3. Nothing guards it. `grep -rn 'TEMPLATE\.md' src/ tests/` excluding testdata returns nothing at all.
4. `just scaffold-self` emits it. My end-to-end render printed `render  docs/plans/TEMPLATE.md` and wrote a 3097-byte file, and the sweep counted 31 emitted files against a 30-entry manifest list.

ONE SHARPENING THE REVIEWER DID NOT MAKE, and it is what lifts this above a nitpick. The list does not merely happen to omit `TEMPLATE.md`; it CANNOT contain it. The file is produced by the post-write render loop at `src/main.rs:1666-1681`, and that code's own comment (`src/main.rs:1657-1660`) says so in terms: "The generated view is NOT a manifest asset (it is derived, and `render`/`render --check` own it), so it is (re)generated here rather than copied." So the pointer directs a reader to an assertion that by construction enumerates manifest assets only, while offering it as the complete inventory of what the scaffold emits. This is not a gap that could be closed by adding a row; it is a category mismatch between what the sentence promises and what the target can express.

THE COUNTER-ARGUMENT, WEIGHED. The comment's own example list does say "the `docs/plans/TEMPLATE*` family" and `TEMPLATE.md` matches that glob, so no reader is misled about the file's existence, and the sentence is literally true as written. What fails is the completeness offer, which is the sentence's entire purpose: it opens "Do not read either group as complete" and then hands over a target that is itself not complete.

IS IT WORTH A FIX? Yes, and I say so having taken the reviewer's marginality seriously. Three reasons.

- It is text this artifact introduced. The pointer is part of the `CT-1` fix in `38d9db4`. Carrying a defect in text the round just wrote is the case for fixing it now rather than backlogging it.
- The note exists to inform a human's widening decision (`prompt-drift-guard.md:21`, and round 1's own reasoning at `prompt-drift-guard-triage.md:143`). The single file the pointer omits is the one that needs a DIFFERENT mechanism from the other 21: a comparison against a render of `TEMPLATE.plan.toml`, not a copied-asset compare. Someone scoping the widening off this pointer would plan for 21 files of one shape and meet a 22nd of another shape only after starting.
- The fix is one clause.

WHAT KEEPS IT `low`, and I verified this rather than assuming it. The gap is latent: my byte sweep shows `docs/plans/TEMPLATE.md` is currently identical to a fresh render (31 emitted, 31 SAME, 0 DIFFERS), so nothing has drifted. The error is one file in 22, and it is an error of completeness rather than of assertion.

WHAT THE FIX MUST ACHIEVE. `src/agents_md_drift.rs:83-87` must stop offering a complete inventory it cannot deliver. Either qualify the pointer as authoritative for the manifest ASSETS and name the generated `docs/plans/TEMPLATE.md` view as an additional emitted, committed, unguarded file that the asset list structurally cannot carry, or drop the completeness offer. See the structural section: this is the third round-2 finding that a completeness claim about coverage went stale, and I recommend the fix be made as part of one consolidated statement rather than as a third independent patch.

## Structural judgement (advisory, for the human and the next fix brief)

ASKED: is this convergent whack-a-mole, or is there a structural cause? My read, grounded in counts rather than impression: it is NOT converging, and the cause is structural.

THE EVIDENCE.

1. SIX FINDINGS ACROSS TWO ROUNDS, AND EVERY ONE OF THEM IS PROSE. Round 1: `FN-1`, `CT-1`, and the doc half of `FN-2` are overclaims; `FN-3` and the mechanism half of `FN-2` are residual descriptions. Round 2: `V2-1`, `A2-1`, `A2-2` are overclaims. Not one finding in either round is a mechanism defect. Round 2's adversarial reviewer ran 17 attacks and found no reachable false negative; I re-ran the extra-committed-file attack, the render sweep, and a line-ending attack of my own and found none either. The guard works. The prose about the guard does not.

2. THE FINDING RATE DID NOT FALL. Three overclaims in round 1, three in round 2. If this were whack-a-mole with a shrinking board, round 2 should have found fewer.

3. THE FIX CREATED THE NEXT FINDING. This is the decisive datum. Round 1's fix corrected the totality claim at two sites (`:150-155`, `:297-326`) and left it standing at two others (`:209-214`, `:401-402`), producing a file that now asserts both P and not-P. `V2-1` did not exist before the fix; the fix manufactured it. A process where correcting three statements creates a fourth inconsistency is not converging on its own.

4. THE CORRECTIONS GROW THE PROSE THEY MUST KEEP CONSISTENT. Doc lines in this file: 139 at `3164404` (step start), 172 at `97a587c` (implementation), 219 at `38d9db4` (after the three one-clause overclaim fixes). Round 1's fix added 47 doc lines, a 27 percent increase, to correct three sentences. The file is now 581 lines of which 293 (50.4 percent) are comments. Every added sentence is one more statement that must independently stay true against the residuals.

5. THE SAME FACTS ARE ALREADY RESTATED IN SEVERAL PLACES, WHICH IS THE MECHANISM. "The precondition is per-line and therefore not total" is now stated in full at `:150-155` AND again in full at `:308-317`. "The guarded set derives from the module-free render by dest prefix" is stated at `:35-38`, again at `:77-87`, again at `:100-107`, and again at `:126-130`. Counting the module header, the constant's doc, the two helper docs, the transform doc, the exclusion note, three in-test comment blocks and the test name, there are about ten independent prose sites in this one file that each make a statement about what is or is not covered. Findings have now landed on seven of them. Each site was written to be self-contained and readable on its own, which is exactly why each one drifts on its own.

6. A SEVENTH SITE, FOUND BY THE FIRST PROBE I AIMED SOMEWHERE NOBODY HAD LOOKED. `src/agents_md_drift.rs:293-295` reads "Only prettier's own freedoms, where a line is wrapped, how many spaces sit between words, how many blank lines separate blocks, are discarded." That "Only" is a completeness claim with an unlisted member: both `normalize_wrapping` and `assert_no_unprotected_construct` iterate `input.lines()`, and Rust's `str::lines()` strips a trailing carriage return, so line-ending style is a fourth discarded degree of freedom. I verified the language behaviour directly with a standalone program (`"# T\r\nline one\r\n".lines()` and `"# T\nline one\n".lines()` produce identical output while the raw strings differ). I do NOT raise this as a finding: line-ending style is not content, git and the formatter own it, and it sits in the same designed-tolerance class as the deliberately dropped trailing newline. I report it because of how it was found. I picked one un-reviewed completeness claim at random and it was also inaccurate. That is what a systemic problem looks like, as opposed to a shrinking list of individual mistakes.

THE DIAGNOSIS. The module documents its coverage in about ten independent, self-contained places. There is no single location that owns the answer to "what is and is not covered". So every correction has to be applied N times, every reviewer finds the site the last fix missed, and every future edit can desynchronise any one of them from the accepted residuals. The three findings in this round are three different sites of one defect, not three defects.

MY RECOMMENDATION: ONE CONSOLIDATED COVERAGE STATEMENT, REFERENCED RATHER THAN RESTATED. Concretely, what it should look like.

- A single named block in the module doc, for example `//! COVERAGE: WHAT THIS GUARD DOES AND DOES NOT CHECK`, appearing exactly once.

- It states THE GUARDED SET OPERATIONALLY, not descriptively: "the two `include_str!` files, plus every asset of the module-free self-scaffold render whose `dest` starts with `PROMPT_DEST_PREFIX`". Defining the set by the mechanism that computes it is what kills the `FN-1` and `A2-1` species at the root. A natural-language category such as "every deployed role prompt" can be true of the tree and false of the mechanism; a definition that just names the filter cannot be, because there is no gap between the description and the code for a residual to open up in.

- It states THE UNGUARDED COMPLEMENT AS A RULE, not a list: "everything else the scaffold emits". The examples follow, labelled as examples, with the `builtin_manifest_lists_the_expected_assets` pointer qualified as authoritative for manifest ASSETS and the one derived view (`docs/plans/TEMPLATE.md`, emitted by the post-write render loop at `src/main.rs:1666-1681`) named as the thing that list structurally cannot carry. Stating the complement as a rule removes the completeness obligation that `CT-1` and `A2-2` both broke; a rule cannot be incomplete, only wrong.

- It names and NUMBERS the accepted residuals so other sites can cite them instead of re-deriving them. `R1` = `H4-3`: a prompt that leaves the module-free render (row removed, module-tagged, or re-`dest`ed) drops silently from the derived set, and an extra committed file under the prefix is never noticed; one orphaned prose file, accepted. `R2` = `FN-2`: the precondition is PER-LINE and constrains no cross-line join, so a line-structured construct `is_hard_start` does not recognise is joined and can be masked, a raw HTML block being the known instance; accepted, with the reason the cheap tightening was rejected.

- Every other site then becomes a one-line cross-reference. `is_hard_start`'s doc stops arguing about correctness at all and says that a recognition failure is residual `R2`, see COVERAGE. The `PROMPT_DEST_PREFIX` doc points at COVERAGE for the complement. The in-test comments cite `R1` and `R2` instead of restating them. That is what turns `V2-1` from a sentence to be reworded into a sentence to be deleted.

- A cheap test for whether the consolidation worked: after it, no site outside the COVERAGE block should contain a quantifier over the coverage set ("every", "only", "all four", "not total", "authoritative", "exhaustive"). Any such quantifier elsewhere is by construction a duplicate that can drift, and it is greppable.

NET EFFECT ON SIZE: this should REMOVE prose. The four restatements of the derived-set fact collapse to one, and the two full restatements of the per-line fact collapse to one plus a citation. That matters, because the alternative approach has added 80 doc lines to this file across the step so far and has a per-round defect rate that has not moved.

THE TRADE-OFF, SO THE HUMAN DECIDES RATHER THAN ME.

- OPTION A, three one-clause fixes, as each finding's section specifies. Cheapest, smallest diff, matches what round 1 did, touches least inherited text. Against it: round 1's three one-clause fixes are what produced round 2's three findings, and after Option A the file still holds about ten independent coverage statements, so the probability that round 3 finds a seventh site of the same species is not small. On the evidence of item 6 above, I would put it above even.
- OPTION B, the consolidation, as a distinctly briefed fix in this step. Larger diff, and it touches `is_hard_start` and `normalize_wrapping` doc text that step 80 wrote rather than this step. Against that: the artifact needs TWO consecutive clean rounds, so every extra round of the same species costs two rounds of runway, and Option B is the only one of the two that changes the rate rather than the count.
- OPTION C, Option A now plus consolidation as a follow-up step. Converges this step soonest on paper, but it accepts finding and fixing the remaining sites one at a time first, which is the cost Option B exists to avoid.

I recommend OPTION B, judged against plan Principle 1 ("Prefer the cleaner long-term architecture over the smallest diff"), which the brief itself invoked to choose the derived set over the enumerated one, and Principle 8 ("One source of truth"), which is exactly what the coverage description currently lacks. It is worth stating plainly that Option B is a larger scope than three comment edits and is therefore a human call, not mine, and that if the human prefers A or C nothing I found makes that unreasonable. What I would not recommend is choosing A on the belief that the list is nearly exhausted; the evidence in items 2, 3 and 6 says it is not.

## Out of scope, for the orchestrator to route

1. A NEW ONE: A FLAKY TEST IN `src/checks.rs`, unrelated to this artifact but found by this round's spot-check. My first `cargo test` in this worktree failed `checks::tests::a_format_check_never_mutates_the_live_tree` and `checks::tests::an_empty_paths_array_runs_unscoped`; five later runs passed. Both failures cited the SAME runner worktree path, `agent-scaffold-checks-run-416707-1785235883764925866`, and each error named the OTHER test's fixture repo. The cause is in the naming: `src/checks.rs:792` builds the runner worktree path as `{temp}/{RUNNER_PREFIX}{pid}-{nanos}` and `nanos()` (`src/checks.rs:848-851`) is a bare `SystemTime::now()` read with no uniqueness guarantee. Cargo runs tests as threads of ONE process, so two concurrent `run()` calls share a pid and can read the same nanosecond, collide on the path, and each corrupt the other's `.git` pointer. In production each `checks` invocation is its own process, so the pid disambiguates and the collision is effectively test-only, but it makes `cargo test` non-deterministic, which is corrosive for a repo whose review process rests on a green suite. Fix would be to add a per-call disambiguator (an atomic counter, or the thread id) to the runner directory name. Recommend a small backlog step. NOT a finding against step 92, whose whole diff is comment-only in `src/agents_md_drift.rs`.

2. THE BRIEF'S OMISSION, carried forward from round 1's out-of-scope item 1 (`prompt-drift-guard.md:21` lists four asset groups and omits the twelve `docs/plans/TEMPLATE` assets). `A2-2` adds one thing the planner should be told alongside it: the complete unguarded set is 22 of 31 emitted files, and the 22nd (`docs/plans/TEMPLATE.md`) is a RENDER artifact, so widening the guard to cover everything needs two mechanisms rather than one. Plan content, planner-owned.

3. THE `FN-2` MECHANISM BACKLOG ITEM, carried forward unchanged from round 1's out-of-scope item 2. Round 2 independently re-measured the proposed cheap predicate and confirmed it regresses reflow tolerance, so the framing round 1 recorded still stands: "give `assert_no_unprotected_construct` a cross-line fail-safe that does not regress the guard's reflow tolerance."

4. `FN-1`'S REGISTRATION-COMPLETENESS GAP and the `H4-3` LEDGER WORDING, both carried forward from round 1 unchanged. The ledger wording update appears to have landed at `3164404` ("docs: widen the H4-3 residual description per step 92 round-1 triage").

## Anything the reviewers missed

- THE `is_hard_start` PARAGRAPH IS INHERITED, NOT NEW. The verification reviewer raised `V2-1` without establishing where the text came from. It is step 80's (`cba4fcc`) and is untouched by this step's diff, which is why the finding needs the "the fix created the contradiction" argument to be in scope rather than simply being an artifact defect. Conversely, the adversarial reviewer did not point out that `A2-1`'s header text IS new in this step, which is the strongest thing that could be said for that finding and makes its scope unarguable.

- A SEVENTH COMPLETENESS CLAIM, at `src/agents_md_drift.rs:293-295`. "Only prettier's own freedoms ... are discarded" omits line-ending style, since `str::lines()` strips a trailing carriage return on both sides. Verified with a standalone program. Not raised as a finding (line-ending style is not content and the omission is harmless), but recorded because it was found on the first probe aimed at an unreviewed site, which is evidence for the structural reading above.

- `cargo test` IS NOT DETERMINISTICALLY GREEN. Both reviewers reported 379 passed and stopped. The flake in item 1 above is real and reproducible in the sense that it happened on a cold first run. A review process that treats "379 passed" as a settled fact should know the suite is not reliable at that granularity.

- `A2-2` IS STRUCTURALLY STRONGER THAN ITS REVIEWER ARGUED. The reviewer treated the omission as an incompleteness. It is a category mismatch: `builtin_manifest_lists_the_expected_assets` asserts over manifest assets, `docs/plans/TEMPLATE.md` is by design not a manifest asset (`src/main.rs:1657-1660` says so), so no edit to that test could ever make the pointer complete. That is why the fix has to change the sentence's promise rather than the list.

- CORROBORATION, not a miss: I independently reproduced round 1's 31-file render sweep (31 SAME, 0 DIFFERS), the round-1 fix closure for all three findings, and the comment-only property by a stronger filter than either reviewer used. Nothing in either round-2 findings file failed to reproduce.

## Round outcome

THREE valid findings require an implementer fix: `V2-1`, `A2-1`, and `A2-2`. All three are comment-only edits to `src/agents_md_drift.rs`, none changes behaviour, and none requires a new or changed test. If the human takes Option B above, all three are absorbed into the single consolidated statement and should NOT be fixed as three separate patches.

ACCEPTED RESIDUALS, unchanged and not reopened by anything in this round: `H4-3` / `FN-3` (a prompt that leaves the module-free render drops silently from the derived set, and an extra committed file under the prefix is never noticed); and the `FN-2` mechanism gap (the precondition has no cross-line fail-safe), now documented.

DISMISSED: nothing. BARRED RE-RAISES: none. BACKSTOP: NOT triggered, since nothing was dismissed and nothing is rated high or critical.

This round is NEW VALID FINDINGS, so the consecutive-clean streak remains 0. The artifact is classified `risky` and needs 2 consecutive clean rounds, so at least two further rounds are required after these fixes land.

Tree state: clean. `git status --porcelain` shows only the three untracked review files (the two round-2 findings files and this one); `git diff` is empty; HEAD is `38d9db4`. Post-triage re-verification on the clean tree: `cargo test` 379 passed, 0 failed; `cargo clippy --all-targets -- -D warnings` clean, exit 0. I ran no `nix fmt` or `just fmt`, and edited no source, plan, or pack file as a deliverable.
