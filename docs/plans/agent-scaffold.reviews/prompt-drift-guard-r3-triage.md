# Step 92 `prompt-drift-guard`: triage verdicts (work review, round 3)

Artifact: `git diff 9f0966c..9174a74` (the whole step, +244 / -107, `src/agents_md_drift.rs` only) and `git diff 9f94acf..9174a74` (the consolidation commit `docs: consolidate the drift guard's coverage prose into one COVERAGE block`, comments only).
Brief: `docs/plans/agent-scaffold.steps/prompt-drift-guard.md`.
Prior verdicts: `prompt-drift-guard-triage.md` (round 1) and `prompt-drift-guard-r2-triage.md` (round 2), both authoritative.
Findings triaged: `prompt-drift-guard-r3-reviewer-verification.md` (`V3-1`, `V3-2`) and `prompt-drift-guard-r3-reviewer-reader.md` (`RD-1`, `RD-2`, `RD-3`, `RD-4`). Six raw, all rated `low` by their reviewers.
Worktree: `.claude/worktrees/triage3-pdg`, detached at `9174a74`. Every mutation below was reverted with the Edit tool (created files removed by path); `git status --porcelain` shows only the three review files and `git diff` is empty at the end.

## Summary

| Finding | Reviewer severity | Verdict | My severity | Reproduced |
| --- | --- | --- | --- | --- |
| `V3-1` + `RD-1` (merged) | low | VALID, doc-only fix | low | Yes, all four routes, plus a fifth variant the reviewers split on |
| `RD-2` | low | VALID, doc-only fix, MISSED REQUIREMENT | low | Yes, exactly, including the byte-identity claim |
| `RD-3` | low | VALID, doc-only fix | low | Yes, exactly |
| `V3-2` | low | VALID, doc-only fix | low | Yes, exactly |
| `RD-4` | low | VALID BUT OUT OF SCOPE | low | Yes, exactly |

FOUR deduplicated findings require an implementer fix (`V3-1`/`RD-1`, `RD-2`, `RD-3`, `V3-2`). All four are comment-only edits to `src/agents_md_drift.rs`; none changes behaviour, and none requires a new or changed test. ONE is valid but out of scope (`RD-4`) and routes to the backlog.

DEDUPLICATION: `V3-1` and `RD-1` are one finding, found independently by both reviewers. They differ only in which `dest`-change variant each ran; both variants matter and both are recorded below. One verdict covers both ids.

BACKSTOP: NOT triggered. No finding is rated high or critical by any reviewer or by me, and I dismissed nothing at any severity, so no second independent triager is required. `RD-4` is ruled out of scope rather than dismissed on its merits; its factual claim is confirmed.

NO MECHANISM DEFECT in round 3, consistent with rounds 1 and 2. See the structural section for what that now means.

## Spot-checks

COMMENT-ONLY, INDEPENDENTLY CONFIRMED. I re-ran the all-comment strip myself rather than accepting either reviewer's:

    diff <(git show 6d5d220:src/agents_md_drift.rs | grep -vE '^[[:space:]]*//') \
         <(git show 9174a74:src/agents_md_drift.rs | grep -vE '^[[:space:]]*//')
    ->  EMPTY

`grep -nE '"[^"]*(///|//!)'` over `9174a74` returns nothing (exit 1), so no string literal can be hiding behind that filter. `git diff 9f94acf..9174a74 --name-only` returns `src/agents_md_drift.rs` alone, and `git diff 9f0966c..9174a74 --stat` shows that one file for the whole step. So the code has not moved by one byte since the mechanism commit; every commit in this step after `6d5d220` is prose.

CLEAN TREE BASELINE. `cargo test`: 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed. `cargo clippy --all-targets -- -D warnings`: clean, exit 0. I ran no `nix fmt` and no `just fmt`, and edited no source, plan, or pack file as a deliverable.

COMMIT MAPPING, since the branch was rebased and the round-2 verdicts cite pre-rebase hashes. `9f0966c` is byte-identical to `3164404` (step start), `6d5d220` to `97a587c` (the mechanism), and `9f94acf` to `38d9db4` (round 1's fix, the round-2 artifact). `9174a74` is round 2's fix, the consolidation. All four round-2 citations resolve.

PROPERTY 4 RE-VERIFIED. The verification reviewer's partition grep reproduces exactly: zero quantifier hits on comment lines outside `:34-101`, two hits inside (`:38` and `:83`, both "all"), and the whole-file grep returns exactly those two. The invariant holds as stated today. See the `V3-2` section for why that is not an unmixed good.

## `V3-1` + `RD-1`: `R1`'s "the suite stays green" is false for three of the four routes it is applied to

VERDICT: VALID. Severity `low` (both reviewers' rating CONFIRMED). Doc-only fix required. NOT a re-raise of `R1`; both reviewers accept the residual explicitly and the finding presupposes it.

SITE: `src/agents_md_drift.rs:73-76`, the trailing clause of the `R1` route sentence.

EVIDENCE REPRODUCED: yes, every route, each mutation applied alone to a clean tree and reverted with the Edit tool.

| Route | `cargo test --bin agent-scaffold agents_md_drift` | Full `cargo test` |
| --- | --- | --- |
| Delete the `.agents/prompts/reviewer.md` `[[asset]]` row | 5 passed | FAILED, 1 failure (`manifest::tests::builtin_manifest_lists_the_expected_assets`) |
| Add `module = "checks"` to that row | 5 passed | FAILED, 2 failures (the same, plus `builtin_checks_module_adds_its_five_assets`) |
| Change that row's `dest` to `.agents/reviewer.md` (leaving the prefix) | 5 passed | FAILED, 1 failure (`builtin_manifest_lists_the_expected_assets`) |
| Change that row's `dest` to `.agents/prompts/reviewer-renamed.md` (staying under the prefix) | FAILED, 4 passed 1 failed | FAILED, 2 failures |
| Hand-place a stale extra file in `.agents/prompts/` | 5 passed | 379 passed, 0 failed |

The two reviewers ran different `dest` variants and BOTH results are load-bearing. The within-prefix rename fails inside this module:

    panicked at src/agents_md_drift.rs:162:13:
    failed to read the committed .agents/prompts/reviewer-renamed.md at
    <worktree>/.agents/prompts/reviewer-renamed.md: No such file or directory (os error 2).
    The self-scaffold render produces this file, so the repo must commit it; run `just scaffold-self`

THE STEELMAN, RULED ON. The verification reviewer recorded it in full and I take it as the strongest case for the sentence: `builtin_manifest_lists_the_expected_assets` is a hand-maintained mirror of the manifest that any legitimate row change must update anyway; once updated, the suite is green and the copy is orphaned, which is the end state the sentence describes; and what fires is a manifest-membership check, never a detection of the orphan itself. So the residual's substance survives every route.

It does not rescue the clause, for three reasons, and I would not rule on any one alone.

1. IT IS FACTUALLY WRONG ON ONE ROUTE, NOT MERELY MIS-TIMED. For the within-prefix `dest` rename the failing test is check 3 of this module, at `src/agents_md_drift.rs:162`. That is not a hand-maintained mirror; it is the mechanism the block exists to describe, failing loudly and naming the file. The steelman's "what fires is never an orphan detection" is true in the narrow sense (check 3 fires about the NEW dest, not the orphaned old copy) but the sentence's claim is about the suite's colour, and the suite is red because of this guard.
2. THE STEELMAN RESCUES THE CLAUSE ONLY BY RE-TIMING IT. It reads the sentence as describing the state after the maintainer has updated the expectations. The sentence carries no such timing, and the cold reader, an independent and methodologically clean test of exactly this question, recorded belief `B10` from the comments alone and then found it false for three of four routes. That is direct evidence the compression is not readable the way the steelman reads it.
3. THE SITE IS THE ONE THE CONSOLIDATION MADE AUTHORITATIVE. The block's own charter is "Write a coverage claim here or not at all" (`:38`) and every other site now cites `R1` rather than re-deriving it, so this sentence is the only description of the residual a reader will get. A testable claim inside that block that does not reproduce is precisely the species the consolidation was built to end.

PROVENANCE, ESTABLISHED RATHER THAN TAKEN. `git log -S "the suite stays green" -- src/agents_md_drift.rs` returns `9f94acf` (round 1's fix), where the clause attached to the unregistered-`pack/prompts/`-file case (`9f94acf:src/agents_md_drift.rs:40-43`) and was TRUE there, as round 3's verification reviewer re-confirmed under claim 17 (379 passed with an unregistered pack file present). The consolidation moved a true clause onto four claims it does not hold for. There is no scope question: this is text the artifact under review wrote.

WHY IT STAYS `low` AND I DO NOT ESCALATE. Nothing misbehaves. The mechanism claim in the same sentence ("invisible to it") is correct and I verified it on all five routes: check 3 never detects the orphan in any of them. The failure direction is a false FAILURE elsewhere in the suite, never a false pass, so no drift can be masked by this error. The fix is comment-only. Within `low` I rate this second-most consequential of the round, behind `RD-2`.

WHAT THE FIX MUST ACHIEVE, AND ONE TRAP TO AVOID. Stop attaching a single suite-wide outcome clause to four different changes.

THE TRAP: do NOT take `RD-1`'s suggested remedy verbatim. `RD-1`'s "WHAT A FIX MUST ACHIEVE" proposes scoping the clause to "check 3 stays green" and asserts "that is true of all four". It is not, and `RD-1`'s own path-3 evidence shows it is not: check 3 fails on the within-prefix rename. Substituting that phrase would replace one false claim with another, which is the exact pattern this step has repeated twice.

THE SAFE FIX IS DELETION. The preceding sentence already states the residual in full ("a committed file under the prefix that the pinned render does not emit is invisible to it"), and the four routes are examples of how one arises. The trailing clause adds nothing except the false scope. Delete it, or reduce it to "the copy is orphaned" with no claim about any test's colour. If the human wants the decision-relevant difference recorded, one added clause covers it: only the hand-placed extra file is silent from the first keystroke; the `pack.toml` routes also trip the manifest expectation list, which a legitimate change updates anyway, after which the orphan is unnoticed. Prefer the deletion. See the structural section for why.

## `RD-2`: the in-test precondition comment still claims a loudness `R2` denies, at the site round 2 required to be narrowed

VERDICT: VALID. Severity `low` (reviewer's rating CONFIRMED). Doc-only fix required. This is FIX INCOMPLETENESS against a requirement round 2 set, not a fresh finding, and not a barred re-raise: `V2-1` was ruled VALID, not dismissed.

SITE: `src/agents_md_drift.rs:387-392`.

EVIDENCE REPRODUCED: yes, both halves.

The masking, through the module's own helpers (temporary probe, reverted with the Edit tool):

    precondition rejects "# T\n\n<pre>\nline one\nline two\n</pre>\n"  = false
    precondition rejects "# T\n\n<pre>\nline one line two\n</pre>\n"   = false
    MASKED (normalize equal) = true

THE BYTE-IDENTITY CLAIM, WHICH I VERIFIED DIRECTLY BECAUSE IT IS WHAT ELEVATES THIS FINDING. The reviewer states the text is byte-identical to the text round 2's triage required be narrowed. It is, across three commits:

- `38d9db4:src/agents_md_drift.rs:397-404` (the round-2 artifact, which round 2's triage cited as `:401-402`).
- `9f94acf:src/agents_md_drift.rs:397-404` (identical; `9f94acf` is `38d9db4`).
- `9174a74:src/agents_md_drift.rs:387-392` (identical; the line numbers moved because the consolidation shortened the module doc).

And the requirement is where the reviewer says it is, `prompt-drift-guard-r2-triage.md:97`: "Secondarily, `src/agents_md_drift.rs:401-402` ... must be narrowed to the per-line scope the predicate has ... it is the same defect and should be fixed in the same pass."

SO THIS IS A MISSED REQUIREMENT, AND I RULE IT AS ONE. Round 2's triage set the requirement twice, not once: as the secondary clause of `V2-1`'s "WHAT THE FIX MUST ACHIEVE" (`:97`), and independently in the structural recommendation the human then chose, which says under Option B that "the in-test comments cite `R1` and `R2` instead of restating them" (`:182`). Both were dropped.

THE ATTRIBUTION QUESTION, RULED ON SO THE ORCHESTRATOR CAN CARRY IT. Three findings of fault are available and I weigh all three.

- THE BRIEF. I am told the orchestrator's consolidation brief did not carry the round-2 requirement forward. I cannot inspect the brief from this worktree, so I take that as given rather than verified. If it is so, the primary fault is there. `AGENTS.md:67` gives the reason findings files exist in the form they do: the triager reads the reviewers' files directly "so nothing is lost in transcription", and the orchestrator "references a finding by its file path in the ledger instead of copying its text". A valid verdict recorded in the triage file and not carried into the fix brief is exactly the hand-off loss that rule is built to prevent. This is a process defect worth fixing in the next brief regardless of what is decided about the artifact.
- THE IMPLEMENTER. Partly at fault, but I put it second. The implementer works from the brief it is handed, and re-deriving the requirement would have meant reading a triage file it may not have been given. What it cannot be excused for is the third item.
- THE ARTIFACT ITSELF. This is the decisive one, and it makes the attribution question largely moot for the VERDICT. The consolidation authored a closing marker at `src/agents_md_drift.rs:101`: "End of COVERAGE. Comments past this point cite it and do not restate it." The site at `:387-392` is past that point, restates rather than cites, and states the opposite of what `R2` says at `:86-99`. The artifact declares an invariant and ships one violation of it. That is a defect of the artifact on its own terms, independent of whether any prior round asked for it, and it is why I rule this VALID and not out of scope.

WHY IT STAYS `low`. I considered `medium` and decline it, for consistency with the settled precedent. This is a false claim in the DANGEROUS direction (it promises loudness where the guard is silent and can mask a content change), which is the strongest case for escalation available in this round. Against that: round 2's triage rated the primary site of the identical defect `low` and gave reasons that all still hold here. Reachability is zero today, re-verified by both round-3 reviewers with a stronger measure than an HTML grep (across all nine guarded files, zero lines begin with `<` after `trim_start()`, and `normalize_wrapping` performs no cross-line join at all). The corrected statement is loud and in the same file, at `:176-179` and `:311-316`. This site's own "(see its doc comment)" pointer leads to `:311-316`, which now carries the `R2` scope limit. And the fix is comment-only. `low`, and within `low` I rate it the most consequential finding of the round.

WHAT THE FIX MUST ACHIEVE. Delete the loudness clause ("Asserted on both sides so the guard fails loudly the day such a construct enters the guidance") and, if anything replaces it, a citation of `R2` in the form the block prescribes: the assertion is per-line and rules out the constructs `assert_no_unprotected_construct` can see; the cross-line class is `R2`. Do not author a new explanation; `:176-179` already carries it and the site already points there. Deleting the sentence outright satisfies the requirement.

## `RD-3`: `R1` cites the wrong file for where the `checks-reviewer` row is module-gated

VERDICT: VALID. Severity `low` (reviewer's rating CONFIRMED). Doc-only fix required. This is the weakest of the four valid findings and I say so plainly; see the note at the end of the section.

SITE: `src/agents_md_drift.rs:79-82`, "whose row is module-gated in `src/manifest.rs`".

EVIDENCE REPRODUCED: yes, exactly. The row and its `module = "checks"` tag are data at `pack/pack.toml:219-223`:

    [[asset]]
    source = "prompts/checks-reviewer.md"
    dest = ".agents/prompts/checks-reviewer.md"
    ownership = "reference"
    module = "checks"

`grep -rn "checks-reviewer" src/ pack/pack.toml` returns that row, the module's description line at `pack/pack.toml:13`, this comment at `src/agents_md_drift.rs:80`, and four test-list entries (`src/manifest.rs:658`, `:677`, `:685`, `src/main.rs:2094`). `src/manifest.rs` contains no asset row; it holds the loader and the tests that assert the gating behaviour.

THE CHARITABLE READING, WEIGHED AND STILL SHORT. "Whose row is module-gated in `src/manifest.rs`" has a true reading: the row carries a module tag, and the gate that drops it is applied by `expand_modules` in `src/manifest.rs`. I do not think that rescues the sentence. The cold reader recorded belief `B12` from the comments alone, before reading any code, and it was the wrong reading; it also recorded that the sentence contradicts `:51-53` of the same block, which says an `[[asset]]` row is added to `pack/pack.toml`. A sentence whose two readings send a maintainer to two different files, inside the block the file designates as its single source for coverage facts, is worth the few words it costs to disambiguate (Principle 7, cite sources so claims can be checked; Principle 20, documentation self-contained).

PROVENANCE. `git log -S "module-gated in \`src/manifest.rs\`" -- src/agents_md_drift.rs` returns `6d5d220`, this step's own mechanism commit, where it read "it is module-gated in `src/manifest.rs` (emitted only under `--module checks`)" at `9f94acf:51`. So it is text this step wrote, carried through the consolidation. No scope question.

WHAT THE FIX MUST ACHIEVE. Name `pack/pack.toml` as where the row and its `module = "checks"` tag live, or drop the locator entirely ("whose row is module-gated"). Do not add an explanation of the gating machinery; the sentence's job is to name the standing benign instance, not to teach module resolution. The rest of the sentence is accurate and verified: `git ls-files .agents/prompts/` returns seven files with no `checks-reviewer.md`, so there is no copy to drift.

FOR THE PLANNER, NOT THE IMPLEMENTER. The step brief carries the same ambiguity at `docs/plans/agent-scaffold.steps/prompt-drift-guard.md:21` ("`.agents/prompts/checks-reviewer.md` is module-gated (`src/manifest.rs`, emitted only under `--module checks`)"), which is where the comment's phrasing came from. Plan content, planner-owned; route it, do not fold it into this fix.

RANKING, RECORDED FOR THE HUMAN. If the human elects to accept residuals rather than run another fix pass, `RD-3` is the first candidate to accept: it has a true reading, it misleads only a reader who follows the pointer, and nothing it can cause is worse than opening the wrong file.

## `V3-2`: the consolidation turned a correct one-directional claim into a false biconditional

VERDICT: VALID. Severity `low` (reviewer's rating CONFIRMED). Doc-only fix required, ONE WORD, but see the invariant ruling below: the fix is blocked until the invariant is amended.

SITE: `src/agents_md_drift.rs:302-304`, inside `normalize_wrapping`'s safety argument.

EVIDENCE REPRODUCED: yes, exactly, through the module's real helpers (temporary probe, reverted with the Edit tool):

    A = "- a\n- b\n"      B = "- a - b\n"
    A precondition rejected = false
    B precondition rejected = false
    same non-whitespace stream = true
    normalize A = "- a\n- b"
    normalize B = "- a - b"
    normalize equal = false

Both inputs satisfy the precondition the sentence is scoped to, both carry the same ordered stream of non-whitespace characters, both are one block, and neither has a fence. The biconditional says they normalize equal. They do not, because `is_hard_start` splits A into two logical lines and leaves B as one.

THE READING IS NOT AN IMPUTATION. "Just when" is standardly "exactly when", and the same file uses it as an exact iff twenty lines below: `:343-344`, "Consecutive blanks collapse to one boundary, recorded just when the last emitted item is not already one", describing `if out.last().is_some_and(|line| !line.is_empty())`.

THE PREDECESSOR WAS CORRECT. `9f94acf:src/agents_md_drift.rs:287` reads "two inputs normalize equal only when they carry the identical ordered stream of non-whitespace characters ...". "Only when" states the necessary condition, which is the direction the safety argument needs (normalize equal -> same content) and which is true, since the transform never deletes, adds, or reorders a non-whitespace character. The consolidation replaced correct, already-reviewed text with false text.

WHY IT STAYS `low`. The direction that fails is the safe one: the transform is STRICTER than the sentence claims, so the failure mode is a false FAILURE, not a false pass, and the implication the guard rests on is untouched. The counterexample is also unreachable through prettier, which never joins two list items. A reader is misled about how tight the transform is, not about whether it can mask drift.

WHAT THE FIX MUST ACHIEVE. Restore "only when". One word, reverting to text two rounds have already reviewed.

## THE INVARIANT RULING (asked for explicitly; the orchestrator carries this into the next brief)

THE INVARIANT NEEDS AMENDING BEFORE THE NEXT FIX PASS, AND THE NEXT REVIEWER BRIEF NEEDS THE AMENDED FORM. This is not optional housekeeping: without it, `V3-2`'s one-word fix cannot be made, because restoring "only when" puts the word "only" on a comment line outside the COVERAGE block, which is what the current mechanical scan flags.

WHAT WENT WRONG, PRECISELY. The invariant as round 2's triage stated it (`prompt-drift-guard-r2-triage.md:184`) is SEMANTIC and correctly scoped: "no site outside the COVERAGE block should contain a quantifier OVER THE COVERAGE SET", with a word list ("every", "only", "all four", "not total", "authoritative", "exhaustive") offered as a greppable proxy, because "it is greppable". The word "only" in "two inputs normalize equal only when ..." is not a quantifier over the coverage set; it is a logical connective in a statement about the transform. Under the invariant as written, the predecessor text never violated it. What was applied was the PROXY as though it were the rule. That is the whole defect: a lexical filter cannot distinguish a quantifier from a connective, and it was given authority to trigger a rewrite.

THE AMENDMENT I RECOMMEND, in four parts.

1. STATE THE RULE SEMANTICALLY, ONCE. No comment outside the COVERAGE block may make a claim about WHAT IS OR IS NOT GUARDED. Statements about the transform's mechanics, the predicate's behaviour, or a test's assertion are not coverage claims and are unaffected by the rule.
2. DEMOTE THE GREP TO A TRIAGE AID. A hit is a prompt to CLASSIFY the line, never a licence to rewrite it. Classify each hit as a coverage claim (fix it) or a mechanical or logical statement (leave it, and record the exemption).
3. EXEMPT LOGICAL CONNECTIVES EXPLICITLY. "Only when", "only if", and "if and only if" inside a mechanical statement are exempt. Changing a logical connective to satisfy a lexical filter is a correctness change, and in this case it made a true statement false.
4. RECORD THE KNOWN EXEMPT HITS SO THE NEXT ROUND'S SCAN IS NOT MISREAD. Today: two hits inside the block, `:38` and `:83`, both the word "all", both fine. After `V3-2`'s fix: one hit outside the block, at roughly `:302`, the restored "only when", exempt under part 3. A round-4 reviewer running the round-2 scan without this note will see the outside-block count go from zero to one and can reasonably read it as a regression. Put the exemption in the brief and in the ledger.

I do NOT recommend abandoning the invariant. Its substance is what made round 3 the first round in this step to find no contradiction BETWEEN comment sites, which is a real result: property 4 verified live, the two-hit liveness check confirmed, and both reviewers independently reported no leak on a wider sweep. The invariant works. Its mechanical proxy was mistaken for it.

## `RD-4`: the thematic-break comment describes a space-insensitive rule the code does not implement

VERDICT: VALID BUT OUT OF SCOPE for this artifact. The factual claim is confirmed; it is not a defect of the change under review. Severity `low` (reviewer's rating CONFIRMED). Route to the backlog as a one-clause doc fix or a one-line predicate widening. This is NOT a dismissal, so the backstop is not engaged and nothing here needs a re-check.

SITE: `src/agents_md_drift.rs:255-256`, "Thematic break: three or more of the same marker (`-`, `*`, or `_`) and nothing else once spaces are removed."

EVIDENCE REPRODUCED: yes, exactly, through the module's real `is_hard_start` (temporary probe, reverted with the Edit tool):

    is_hard_start("---")   = true
    is_hard_start("- - -") = true
    is_hard_start("* * *") = true
    is_hard_start("_ _ _") = false
    is_hard_start("___")   = true

`:257-263` tests `bytes.iter().all(|&b| b == marker)` on the line as given; spaces are never removed. `_ _ _` is a valid CommonMark thematic break, the comment says it is handled, and it is classified soft and joined onto the preceding logical line. The reviewer's secondary point also holds: `- - -` and `* * *` pass through the LIST-MARKER branch at `:244`, so the comment misdescribes why they work.

THE SCOPE RULING, AGAINST THIS STEP'S OWN PRECEDENT RATHER THAN AGAINST MY TASTE. Round 2's triage ruled `V2-1` in scope despite inherited text, on three grounds it said explicitly it "would not rule it in on any one alone" (`prompt-drift-guard-r2-triage.md:87-91`): the artifact created the contradiction; it was fix incompleteness against a requirement a prior round set; and the remedy is one clause in text the artifact already edits, with precedent. Applying the same three-part test here:

1. DID THE ARTIFACT CREATE A CONTRADICTION? Weakly, at most. The consolidation rewrote `is_hard_start`'s doc at `:232-236` to say the marker set "is part of residual `R2`", which raises the stakes on the marker list's accuracy. But it did not touch `:255-256`, and the inaccurate clause does not contradict any text the consolidation wrote; it under-describes the predicate. Partial credit.
2. FIX INCOMPLETENESS AGAINST A PRIOR REQUIREMENT? No. No round required anything here.
3. ONE CLAUSE IN TEXT THE ARTIFACT EDITS? No. `git log -S "once spaces are removed" -- src/agents_md_drift.rs` returns `cba4fcc` (step 80) alone. The clause is untouched by this step's entire diff.

One partial of three, and the weakest of the three. Round 2's own standard fails it. Ruling it in would also expand the artifact's fix surface for a defect that is unreachable today (no guarded file contains a spaced thematic break, and `normalize_wrapping` performs no cross-line join at all across the nine guarded files), which is the precise mechanism that has generated findings in this step. `AGENTS.md` Principle 8, "No silent scope expansion": flag it, do not quietly do it.

FOR THE ORCHESTRATOR. Record this in the ledger as SETTLED, out of scope, backlogged, so a round-4 or round-5 reviewer does not re-raise it. It is a genuine defect of `src/agents_md_drift.rs` and should be fixed eventually, alongside the `FN-2` mechanism backlog item that owns the same predicate. If the human decides to fold it into this step's fix pass anyway, that is a human call under the request-intake rule, not mine, and it should be a conscious addition rather than scope creep.

## Structural judgement (advisory, for the human; not a verdict)

ASKED: how to reach two consecutive clean rounds within the two remaining, given three rounds of prose findings and zero mechanism defects.

### The evidence, counted rather than impressionistic

1. THE MECHANISM IS DONE AND THAT IS NOT IN DOUBT. Three rounds, two independently adversarial (round 2's adversarial reviewer ran 17 attacks; round 3's two reviewers between them proved checks 1 and 2 still guarded by mutation, proved check 3 self-extending in three steps by adding a live `[[asset]]` row, exercised both halves of the non-vacuity assertion, re-implemented the rejected `R2` tightening from its own description, and re-verified prettier's fixed-point behaviour against prettier 3.6.2 with the repo's own config). No reachable false negative has ever been found. The non-comment text is byte-identical to `6d5d220`, which I verified myself. Whatever is decided, it is not decided on doubt about the guard.

2. THE FINDING RATE HAS NOT FALLEN: 4, then 3, then 6 raw (5 deduplicated). Round 3 is the highest raw count of the step.

3. EVERY FIX PASS HAS MANUFACTURED AT LEAST ONE NEW DEFECT. Two for two, and both are documented with provenance rather than inferred. Round 1's fix (`9f94acf`) created `V2-1` (the file asserting both P and not-P) and the unflagged "Only prettier's own freedoms" claim that round 2's triager found on its first probe. Round 2's consolidation (`9174a74`) created `V3-1`/`RD-1` (a true clause moved onto three claims it does not hold for, confirmed by `git log -S`) and `V3-2` (correct text rewritten into false text to satisfy a lexical proxy). This is the base rate of this step, not a guess about the future.

4. PROSE VOLUME, MEASURED. Comment lines in `src/agents_md_drift.rs` across the step: 194 at `9f0966c` (step start, the file already existed), 246 at `6d5d220` (mechanism, +52), 293 at `9f94acf` (round 1's fix, +47), 280 at `9174a74` (the consolidation, -13). The step added 51 non-comment lines and 86 comment lines. The file is now 280 comment lines of 568, 49.3 percent. The consolidation, which round 2's triage predicted "should REMOVE prose", removed 13 lines net.

5. THE DECISIVE NUMBER, AND IT REFRAMES WHAT THE CONSOLIDATION ACHIEVED. Round 3's verification reviewer fact-checked 21 distinct claims in the 68-line COVERAGE block; 20 verified, 1 did not. That is a per-claim defect rate near 5 percent. The consolidation successfully collapsed the SITES (property 4 holds; no coverage quantifier survives outside the block; round 3 found no contradiction BETWEEN sites, which every prior round did). What it did not do is reduce the number of independently falsifiable STATEMENTS. Defect count tracks claim count, not site count. At about 21 claims in the block and more outside it, one false claim per freshly written pass is the expected yield, and that is exactly what round 3 found in newly written text: two, one of them from a lexical rule misapplied.

6. WHAT CHANGED IN ROUND 3 THAT HAS NOT BEEN TRUE BEFORE, AND IT IS THE MOST IMPORTANT DATUM FOR THE FORECAST. Rounds 1, 2, and 3 each reviewed DIFFERENT TEXT: mechanism prose, then round-1-fix prose, then a from-scratch consolidation. No round has ever re-measured the same words twice. Round 3 is the first round after which the file has been examined near-exhaustively: all 21 COVERAGE claims fact-checked with runnable evidence, plus a cold read of 100 percent of the comment lines with 14 recorded beliefs tested, plus both reviewers' explicit "checked and NOT raised" lists. The historical finding rate was measured on a moving target. The marginal yield of a fresh reviewer on an UNCHANGED artifact is genuinely lower than the rate in item 2 suggests, and that is the single strongest reason to think two clean rounds are reachable at all.

### The three options, and what each costs

Under the convergence rule, rounds 4 and 5 must BOTH be clean or the loop escalates at the cap. Any valid finding in round 4 resets the streak and round 5 hits the cap regardless.

OPTION (a), ANOTHER TARGETED FIX PASS. Fix the four valid findings, review twice.
- For it: it is the only option that removes two statements that actively mislead. `RD-2` tells a maintainer the guard will fail loudly on a construct it will silently mask, which is the dangerous direction; `V3-1`/`RD-1` is a claim that does not reproduce inside the block the file designates as its single source of coverage truth, written after a human specifically directed a consolidation to make that block trustworthy.
- Against it: it authors new text, and the base rate for that on this file is 2 for 2 defects per pass. It also consumes round 4 on changed text, which is where every finding in this step has come from.
- Cost: one implementer pass plus both remaining rounds.

OPTION (b), ACCEPT THE REMAINING FINDINGS AS RESIDUALS AND REVIEW AN UNCHANGED ARTIFACT TWICE.
- For it, and this is stronger than it first appears: the convergence rule counts "a ledger re-raise without new evidence" as a CLEAN round (`AGENTS.md:56`). Accepting all four as recorded residuals therefore neutralises them as convergence blockers by construction, and only a genuinely NEW finding can break a round. Combined with item 6, the probability of two clean rounds is the highest of the three options.
- Against it: the artifact ships with four known-false statements, two of them in the COVERAGE block, directly against Principle 16 (one source of truth) and against the block's own charter at `:38`. It also converges by arranging for the counter to read clean rather than by making the artifact right, which is the failure mode the separate-triager rule exists to resist. I am obliged to name that plainly since I am the role that is supposed to.
- Cost: zero implementer time; a permanent documented inaccuracy in the guard's authoritative comment.

OPTION (c), SUBSTANTIALLY CUT THE PROSE.
- For it: the diagnosis in item 5 is right, and cutting is the only move that lowers the claim count rather than relocating it. It is the correct long-term architecture.
- Against it: it is the largest possible injection of new text at the moment of least remaining runway, it re-enters the exact failure mode twice measured, and deciding which claims are load-bearing is itself a judgement call that produces new claims (`R1` and `R2` genuinely do need recording somewhere).
- Cost: near-certain loss of round 4, and escalation at the cap. Predicted: a new finding in the newly cut prose.

### What I would choose, and why

A CONSTRAINED VERSION OF (a): A DELETION-ONLY FIX PASS, WITH THE INVARIANT AMENDED FIRST.

The constraint is what makes it different from rounds 1 and 2, and it is not a slogan. Every defect this step has produced, all nine across three rounds, is a false or overreaching STATEMENT. Rounds 1 and 2 both fixed by AUTHORING: round 1 added 47 comment lines to correct three sentences; the consolidation rewrote 68 lines from scratch. A pass that only removes statements cannot create statements. The four fixes reduce to:

- `V3-1`/`RD-1`: delete the trailing clause. The residual is already stated in the preceding sentence.
- `RD-2`: delete the loudness sentence. `:176-179` already carries the correct scope and the site already points at it.
- `V3-2`: restore one word, "just when" -> "only when", reverting to text two rounds have reviewed.
- `RD-3`: replace a wrong locator with the right one, or delete the locator.

Net effect: the file gets shorter, no new claim enters, and the two misleading statements are gone. It also inverts item 4's trend for the first time in the step.

Three conditions, each of which I would state in the fix brief:

1. THE INVARIANT IS AMENDED FIRST, per the four-part ruling above, and the amended form goes to the round-4 reviewers. Without this, `V3-2` cannot be fixed and round 4 can raise a false finding on the restored "only".
2. NO NEW SENTENCE IS AUTHORED. If a fix seems to need an added explanation, that is the signal to delete more, not to write more. `RD-1`'s own proposed remedy is the cautionary case: its suggested replacement phrase ("check 3 stays green") is false on evidence contained in its own finding.
3. `RD-4` IS NOT FOLDED IN, and is recorded in the ledger as settled-out-of-scope so it is not re-raised.

WHAT I PREDICT. Round 4 on a shorter artifact whose remaining claims round 3 already fact-checked: better than the step's historical rate but not comfortable, call it a coin flip, with the most likely source of a finding being a reviewer re-reading the edited `R1` sentence rather than anything untouched. Round 5, reviewing text unchanged since round 4 with a full ledger: clearly likelier clean. Overall probability of converging within the cap: under a half. That is an honest number, not a hedge.

AND THE REASON I RECOMMEND IT ANYWAY. Reaching the cap is not a failure state; it is a human decision point, and the human arrives at it holding a mechanism verified three times over by two independent adversarial passes with the code provably unmoved, and a prose residual list of low-severity doc claims. "Escalate at the cap and accept" is a perfectly good terminal state for this artifact. Because the downside of not converging is that mild, the right thing to optimise is whether the artifact is CORRECT, not whether the counter reads clean. Option (b) optimises the counter, and I will not recommend it for that reason, though I have set out above exactly why it would probably work, so the human can overrule me on the evidence rather than on my framing. If the human's priority is closing this step within the cap rather than the prose being right, (b) is the higher-probability play and `RD-3` and `V3-2` are the two findings I would accept first.

## Round outcome

FOUR valid findings require an implementer fix: `V3-1`/`RD-1` (merged), `RD-2`, `RD-3`, `V3-2`. All four are comment-only edits to `src/agents_md_drift.rs`; none changes behaviour and none requires a new or changed test. One finding is valid but OUT OF SCOPE (`RD-4`) and routes to the backlog.

This round is NEW VALID FINDINGS, so the consecutive-clean streak remains 0. The artifact is classified `risky` and needs 2 consecutive clean rounds; rounds 4 and 5 must both be clean or the total-round cap escalates to a human.

ACCEPTED RESIDUALS, unchanged and not reopened by anything in this round: `R1` (a committed file under the prefix that the pinned render does not emit is invisible to check 3) and `R2` (the precondition is per-line and constrains no cross-line join). Both round-3 reviewers verified both are still reachable and that neither was quietly closed under cover of a doc change. Every round-3 finding presupposes the residuals rather than contesting them; none is a barred re-raise.

DISMISSED: nothing. BARRED RE-RAISES: none. BACKSTOP: NOT triggered, since nothing was dismissed and nothing is rated high or critical.

OUT OF SCOPE, FOR THE ORCHESTRATOR TO ROUTE.
1. `RD-4`, the thematic-break comment (`src/agents_md_drift.rs:255-256`), inherited from `cba4fcc`. Backlog, alongside the `FN-2` mechanism item that owns the same predicate.
2. The step brief's matching ambiguity at `docs/plans/agent-scaffold.steps/prompt-drift-guard.md:21` (`checks-reviewer.md` "module-gated (`src/manifest.rs`, ...)"), which is where `RD-3`'s phrasing came from. Planner-owned.
3. The consolidation-brief hand-off gap identified under `RD-2`: a valid triage requirement recorded at `prompt-drift-guard-r2-triage.md:97` did not reach the implementer. Process, orchestrator-owned, and worth fixing before the next brief regardless of what is decided about the artifact.
4. Carried forward unchanged: the `src/checks.rs` runner-worktree name collision (round 2's out-of-scope item 1), the `FN-2` mechanism backlog item, and `FN-1`'s registration-completeness gap.

Tree state: clean. `git status --porcelain` shows only the three untracked review files (the two round-3 findings files and this one); `git diff` is empty; HEAD is `9174a74`. Every mutation was reverted with the Edit tool and the one file I created (`.agents/prompts/stale-orphan.md`) was removed by path. Post-triage re-verification on the clean tree: `cargo test` 379 passed, 0 failed; `cargo clippy --all-targets -- -D warnings` clean, exit 0. I ran no `nix fmt` or `just fmt`, and edited no source, plan, or pack file as a deliverable.
