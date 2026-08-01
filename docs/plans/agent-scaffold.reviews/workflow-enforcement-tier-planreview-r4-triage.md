# `workflow-enforcement-tier` plan review, round 4: triage

Triager model: Claude Opus 5 (1M context). Exact model id `claude-opus-5[1m]`.
Worktree: `.claude/worktrees/triage-q55-r4`, branch `triage/q55-r4` at commit `e34c2c9`, the exact commit both reviewers reviewed, so every line citation resolves against the text they read.
`TMPDIR` for any scratch work was `/tmp/triage-r4-scratch`, outside any git repository.

Findings files triaged: `workflow-enforcement-tier-planreview-r4-reviewer-residue.md` (`R4A-1`, the residue and fix-verification lens, `claude-sonnet-5`) and `workflow-enforcement-tier-planreview-r4-reviewer-consistency.md` (`R4B-1` through `R4B-7`, the internal-consistency lens, `claude-opus-5[1m]`). The nine files of rounds 1 to 3 were read as the record of what was already ruled, and the round 1, 2 and 3 triage files were read in full for the `EX-5`, `INC2-6`, `INC2-7`, `F-5` and `R3B-1` rulings the round 4 findings claim lineage from.

## Summary

Raw findings 8. After deduplication 7. All seven VALID, none accepted as residual, none dismissed.

Adjusted severity: 0 critical, 0 high, 0 medium, 7 low. Every reviewer rating stands. I considered `medium` for the merged `:280` finding and for `R4B-4` and took neither; the reasoning is in each entry, and neither was held down to keep a count tidy.

| id | source file | reviewer severity | my severity | verdict |
| --- | --- | --- | --- | --- |
| `R4B-1` | `-r4-reviewer-consistency.md` | low | low | VALID |
| `R4B-2` | `-r4-reviewer-consistency.md` | low | low | VALID |
| `R4A-1` / `R4B-3` (merged) | both files | low / low | low | VALID |
| `R4B-4` | `-r4-reviewer-consistency.md` | low | low | VALID |
| `R4B-5` | `-r4-reviewer-consistency.md` | low | low | VALID |
| `R4B-6` | `-r4-reviewer-consistency.md` | low | low | VALID |
| `R4B-7` | `-r4-reviewer-consistency.md` | low | low | VALID |

## Deduplication

`R4A-1` AND `R4B-3` ARE ONE FINDING. Both are about the exclusivity claim at `workflow-enforcement-tier.md:280`, both rule it false, and both prescribe a deletion of the same clause. They are merged and triaged once, below. The two reviewers reached the site by different routes (residue-of-a-fix versus claim inventory) and blamed different increments; the merged entry rules which analysis is correct.

NO OTHER OVERLAP. I checked each remaining pair. `R4B-1` (a doc-comment count), `R4B-2` (a rejection-ground count), `R4B-4` (an end-property absolute), `R4B-5` (a ledger-default absolute), `R4B-6` (a response count) and `R4B-7` (a cause count) have disjoint sites and disjoint root causes. Three are counts and three are absolutes, but no two share a sentence, a claim or a fix.

ONE LINE CARRIES TWO INDEPENDENT EDITS, WHICH THE FIX PASS MUST NOT COLLAPSE INTO ONE. `workflow-enforcement-tier.md:298` is a fix site for BOTH `R4B-1` (`falsifying three doc comments` -> `falsifying four doc comments`) and `R4B-6` (strike `THREE` from `one predicate now drives THREE responses`). The two edits are in different clauses of the same long line and are independent; applying one and re-reading the line as done would leave the other open.

## `R4B-1`. `VALID`. Severity `low` (unchanged). The doc-comment count is FOUR at two sites and THREE at three others, one of which is the plan TOML

TENSE APPLIED: FORWARD, and the two tenses do not differ here. Both sides are claims about a set of doc comments the change makes false or incomplete; the set is fixed by the code as it stands today plus what `Q-55-jsonreason` specifies, and neither number is a statement about the current tree alone.

BOTH SIDES REPRODUCED.

Side A, FOUR, two sites. `workflow-enforcement-tier.md:198`: "THIS IS A DOCUMENTED-CONTRACT CHANGE AND IS TREATED AS ONE. Four doc claims are falsified or made incomplete by it, found by sweeping `src/next.rs` and the `status` projection for exhaustiveness claims rather than by patching the one already known." The list it heads has exactly four bullets, at `:200`, `:201`, `:202` and `:203`. `workflow-enforcement-tier.md:354`: "THE FOUR DOC COMMENTS `Q-55-jsonreason` FALSIFIES OR LEAVES INCOMPLETE, all four in the same change because two of them are tied by a cross-reference", naming the same four.

Side B, THREE, three sites. `workflow-enforcement-tier.md:275`: "with the three falsified doc comments corrected in the same change". `workflow-enforcement-tier.md:298`: "falsifying three doc comments and breaking a byte-compare golden". `docs/plans/agent-scaffold.plan.toml:1704`: "three doc comments that claim the JSON contract is exhaustive or enumerate the causes of an absent part are falsified and corrected in the same change". All three quoted strings reproduce verbatim.

THE TRUE COUNT, ESTABLISHED FROM THE CODE RATHER THAN FROM EITHER NUMBER AS STATED, AS THE BRIEF REQUIRES. I opened all four cited comments in the tree at `e34c2c9` and tested each against what the change specifies:

- `src/next.rs:114-115`: "Why there is no active loop, for the human renderer. Not serialised (the JSON contract is exactly the fields above); recomputed each call, never stored." `:220` retypes `no_active_loop_reason` and removes `#[serde(skip)]`, so both halves become false. COUNTS.
- `src/next.rs:95-97`: "Every derived part is optional so a missing plan or log yields a partial projection rather than a failure (mirrors `status`'s `Projection`)". An enumeration of two causes; the unsafe pairing is a third. COUNTS.
- `src/main.rs:561-564`: "Every part is optional so a missing plan or metrics file yields a partial projection rather than a failure". Same enumeration, and tied to the previous by the cross-reference at `src/next.rs:96`. COUNTS.
- `src/next.rs:111-112`: "or `None` when the ledger is absent or carries no such section". Two causes; `ledger-not-this-project` (`:230`) is a third. COUNTS.

Four. I also re-tested the two comments round 2's triage cleared as non-members, because a wrong exclusion would change the count in the other direction: `src/next.rs:106` ("present only when the metrics log was readable") and `src/main.rs:569` ("present only when the metrics log exists") are necessary-condition claims of the form "present implies X", which an extra cause of ABSENCE leaves true. They are correctly excluded. The fifth item at `src/next.rs:108-109` is excluded by `:205`'s own explicit ruling that it is pre-existing and not a consequence of this change, and `:354` names it separately as such. So the set is four and neither three nor five.

THREE MATCHES NO DEFENSIBLE READING, WHICH I CHECKED RATHER THAN ASSUMING. Under the strict sense of "falsified" the count is ONE (`:200` "BECOMES FALSE"; the other three are "BECOMES INCOMPLETE", "HAS THE SAME DEFECT", "IS SHORT BY ONE"). Under the sense the TOML itself states ("claim the JSON contract is exhaustive OR enumerate the causes of an absent part") the count is one plus three, which is four. Under a reading scoped to the serialised reason alone the count is one. There is no reading on which three is right.

THE RESIDUE LINEAGE IS TRUE BUT THE REVIEWER MISATTRIBUTES THE ROUND, AND I CORRECT IT. `R4B-1` says this is "round 3's `INC2-6` numeral fix". `INC2-6` is a ROUND 2 finding and its fix landed in the round 2 fix pass, `d9726fa`. The round 3 residue reviewer VERIFIED that fix with the two-literal-string sweep it records at `-r3-reviewer-residue.md:133` ("`Three doc claims` / `THE THREE DOC COMMENTS`: 0. `Four doc claims` / `THE FOUR DOC COMMENTS`: 1 each"), and it is that verification sweep, not the fix, that belongs to round 3. The substance is unaffected and is confirmed at commit level: `git show 05a8898:docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md | grep -n "three falsified doc\|falsifying three doc"` returns two hits, and `git show 05a8898:docs/plans/agent-scaffold.plan.toml | grep -c "three doc comments"` returns 1, so all three side-B sites predate every fix pass. Round 2's triage scoped the fix as "change 'Three doc claims' to 'Four doc claims' at line 198", the fix pass applied exactly that scope, and the three differently-spelled sites were outside it. This is the `RES-1` shape, and it is a MISSED TWIN of a correctly-scoped fix rather than a falsehood the fix authored.

WHY `low`. Impact if unfixed is a wrong count in an increment-scope sentence, in a risk paragraph and in the TOML decision receipt, against a list four lines long that enumerates the members. An implementer working from the enumerated list at `:198`-`:203` or `:354` gets all four; the only reader misled is one who counts from `:275`, `:298` or `:1704` without opening the list. Same band as `EX-6` and `INC2-6`, which were rated `low` for the same reason: it misleads about an enumeration, not about behaviour.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. NUMERAL-EDIT CLASS, THREE SITES, plus a re-render. Change "three" to "four" at `workflow-enforcement-tier.md:275`, at `workflow-enforcement-tier.md:298`, and at `docs/plans/agent-scaffold.plan.toml:1704`. This authors no new vocabulary: "four" is the word already used for the identical set at `:198` and `:354`. A pure deletion of the numeral at all three sites is an acceptable alternative and I would not reject it, but I prescribe the numeral because it converges all five sites on a count the document's own four-bullet enumeration establishes, and because dropping the count from the TOML would leave the structured decision record without a number the sidecar carries.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS AND NEGATIVES:

- `three doc comments`: 2 (`workflow-enforcement-tier.md:298`, `agent-scaffold.plan.toml:1704`). `three falsified doc`: 1 (`:275`). `falsifying three doc`: 1 (`:298`, the same hit).
- The numeral spelled against any doc-claim noun, swept as a regex rather than as literals (`(three|four|3|4|five|two)[^.]{0,30}doc (claim|comment)`) over all three sidecars and the TOML: exactly the five sites above, `:198`, `:202`, `:275`, `:298`, `:344`, `:353`, `:354` and TOML `:1704` in the raw hit set, of which `:202`, `:344` and `:353` are prose references to a single named comment carrying no count. No sixth counted site anywhere.
- DOES EITHER NUMERAL HEAD AN ENUMERATION? No. `:275`, `:298` and `:1704` are all mid-sentence and no list follows any of them. This is the round 2 trap and it does not apply here; I checked it explicitly because `R4B-7` below is a site where it DOES apply.
- IS ANY NUMBER DERIVED FROM THIS ONE? No. `:354`'s "all four" restates the same count in the same bullet and is already correct. Nothing computes a total from it.
- `test-tmpdir-repo-assumption.md` and `status-resume-ignores-json.md`: zero hits on any doc-claim count.
- RENDER MIRRORS OWED: `docs/plans/agent-scaffold.md:1670`, `:1693` and `:168`. All three are generated projections; re-render, do not hand-edit.

## `R4B-2`. `VALID`. Severity `low` (unchanged). Candidate (d) is rejected on five grounds in the section opener and on four in the section's own fifth bullet

TENSE APPLIED: PRESENT. Both sides are claims about the document's own recorded reasoning, not about any tree.

BOTH SIDES REPRODUCED.

Side A, `workflow-enforcement-tier.md:242`: "Explorer B BUILT it, including the sibling and ledger extensions, and argued against it; explorer C reached the same vulnerability independently. It is rejected on five measured grounds, recorded here so the direction is closed rather than rediscovered."

Side B, the closing sentence of the fifth bullet, `workflow-enforcement-tier.md:248`: "This bullet is relevant only as the record of why the first pass's cost accounting should not be reused; (d) is rejected on the four grounds above, not on its cost."

The list has exactly five bullets, at `:244`, `:245`, `:246`, `:247` and `:248`, and the fifth is headed "CORRECTION TO THE FIRST PASS'S COST LIST FOR (d)" and disclaims ground status in its own last clause. The two sides are in genuine contradiction: the opener counts five grounds, the list's own fifth member says four.

WHICH SIDE IS WRONG: THE OPENER, and I checked the alternative before accepting it. The fifth bullet does contain a substantive argument against (d) (the `#[serde(deny_unknown_fields)]` version fence at `src/plan/source.rs:102`), so it is not obviously a non-ground. But the bullet itself decides the question against that reading, in as many words, and the plan TOML's independent summary of the rejection agrees with four and states no count. `docs/plans/agent-scaffold.plan.toml:1700`, read in full: "its builder measured that it contributes nothing on shipping day, that a declared path reconstructs the same false pass in a worse form (CWD-independent, committed, and passing `validate --source` clean), that no validator can refuse it because the conventional log lives outside the plan directory so the declared ref must permit the `..` component `is_safe_sidecar_ref` exists to forbid, and that it cannot cover the Markdown `--plan` substrate at all." Four grounds, matching bullets 1 to 4 exactly, with the version fence absent. Two independent statements say four; one says five.

WHY IT MATTERS RATHER THAN BEING A SLIP. `:242`'s stated purpose is that the direction is "closed rather than rediscovered", so the paragraph is addressed to a later reader deciding what would have to be beaten to reopen (d). A reader who counts five and finds one of them is a cost correction the document says not to reuse has to work out which four are load-bearing.

WHY `low`. The rejection stands on four measured grounds either way, and every one of them is stated in full immediately below the wrong count. Nothing operative turns on it.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. DELETION CLASS, ONE SITE, plus a re-render. At `:242`, strike the word "five", leaving "It is rejected on measured grounds, recorded here so the direction is closed rather than rediscovered."

I PRESCRIBE THE DELETION RATHER THAN THE NUMERAL EDIT `R4B-2` PROPOSED, AND THE REASON IS THE ROUND 2 LESSON THIS LOOP HAS ALREADY PAID FOR ONCE. "five measured grounds" IMMEDIATELY HEADS the five-bullet list at `:244`-`:248`. Substituting "four" would put a numeral directly above a list of five visible items, which is exactly the shape round 2's triage prescribed and the planner had to widen. The fifth bullet's self-disclaimer makes "four" defensible, but it would be defensible only on a reading a later reviewer has to reconstruct, and that is how the next round's finding gets manufactured. Deleting the numeral removes the contradiction and leaves nothing to recount. `:248`'s "the four grounds above" is CORRECT AS WRITTEN, refers to bullets 1 to 4 which are literally above it, and MUST NOT CHANGE.

WHAT I SWEPT, INCLUDING NEGATIVES:

- `five measured grounds`: 1 site, `:242`. `five grounds`: 0. `measured grounds`: 1, the same site.
- `grounds` as a bare noun across all three sidecars and the TOML: 2 hits total, `:242` and `:248`. No third site states or derives a ground count.
- The TOML's own rejection summary at `:1700` carries FOUR grounds and NO count, so it needs no edit and is a supporting negative rather than a fix site.
- `test-tmpdir-repo-assumption.md`, `status-resume-ignores-json.md`: zero hits.
- RENDER MIRROR OWED: `docs/plans/agent-scaffold.md:1637`.

## `R4A-1` / `R4B-3`, MERGED. `VALID`. Severity `low` (unchanged, both reviewers). `:280`'s exclusivity claim for the predicate is false, and it is a MISSED TWIN of the claim family round 1 and round 3 each closed once

TENSE APPLIED: FORWARD, and the two tenses differ, so I state it explicitly. Against today's tree the sentence has no truth value, since no increment has landed. Against the tree the increments produce it is false. `R4B-3` applies the forward tense and says so; `R4A-1` labels its own tense "PRESENT for the operative half" on the ground that the claim is evaluable against the document's other forward-looking sections. That labelling is loose but harmless: both reviewers in fact evaluate `:280` against what inc1 PRODUCES, which is the forward tense, and the verdict is the same either way.

SIDE A REPRODUCED, `workflow-enforcement-tier.md:280`: "WHY THE PREDICATE IS ITS OWN INCREMENT. It is the only part of the mechanism that changes what a currently-succeeding invocation REPORTS, whether by failing (the validator) or by withholding (the projections); it carries a known false positive (accepted cost (ii)); and it deliberately uses a DIFFERENT resolution from the default, so its review must check the lexical/canonical SPLIT rather than one rule."

WHAT "THE MECHANISM" DENOTES, WHICH I RESOLVED FROM THE DOCUMENT RATHER THAN ASSUMING. `:150` heads the section "The mechanism, decided rather than chosen here" and `:152` gives its content: "ANCHOR PLUS REFUSAL, IDENTITY QUEUED". So "the mechanism" is inc1's anchor plus inc2's refusal, a two-part referent that INCLUDES inc1. Both reviewers reach this independently and both are right. The claim therefore asserts that inc1's half changes nothing about what a currently-succeeding invocation reports.

SIDE B REPRODUCED, ALL FIVE FALSIFIERS, EACH OPENED:

- `:274`, inc1's own description, which is the text round 1's `EX-5` fix wrote: "NO new REFUSAL mechanism: any new non-zero exit comes from the pre-existing W3 check finally running against the right project, which is check 4's whole point." A new non-zero exit on a run that exits 0 today is a currently-succeeding invocation whose report changes by failing.
- `:311`, check 4: "AFTER INC1, the false pass is dead ... Before the fix it exits 0 with `workflow invariants hold`. After, no green. Give the fixture a log of its OWN with no evidence for that slug and expect the correct RED instead of the absence of a green." Round 1's triage measured this exact flip on this branch (`-r1-triage.md:126-136`, exit 0 today, exit 1 after inc1).
- `:310`, check 3: "AFTER INC1, defect B's original reproduction is dead ... does NOT read agent-scaffold's own log and does NOT print `workflow invariants hold`. Expect the stderr note naming the FIXTURE's own missing log path and exit 0." A currently-succeeding invocation whose report changes WITHOUT failing.
- `:312`, `:313`, `:314`, checks 5, 6 and 7. Each takes an invocation that exits 0 today with a particular output (`state: converged` and `next: mark the step complete`; agent-scaffold's own record count; this repository's `## RESUME STATE` block) and requires a different output after inc1, in each case a withheld or replaced part (`metrics: no log found`, `no ledger at <fixture path>; nothing to resume`).
- `:296`, inc1's own risk argument: "It changes WHICH FILE the tool reads on every invocation of THREE commands that do not pass `--metrics`, plus which LEDGER two of them read".

WHICH ANALYSIS IS CORRECT: `R4B-3`'s, AND `R4A-1`'s IS PARTLY WRONG. `R4A-1` concedes the "failing" disjunct to inc1 but claims the "withholding" disjunct is exclusive to inc2, on the ground that "nothing else in the fold introduces an omit/withhold behaviour: `:172` assigns it to inc2's own decision, `Q-55-refusalscope`, alone". That conflates two different things. `Q-55-refusalscope`'s omit-with-a-reason MECHANISM is indeed inc2's alone; it does not follow that inc1 leaves what the projections report unchanged, and checks 5, 6 and 7 say in terms that it does not. After inc1 alone, `status` prints `metrics: no log found` where it printed a borrowed record count, `status --resume` prints `no ledger at <fixture path>; nothing to resume` where it printed this repository's entire resume block, and `next` omits the fabricated loop. That is withholding, on the projections, from inc1. `R4B-3`'s conclusion that BOTH halves of the parenthetical are wrong about inc1 is the correct and more complete analysis, and I adopt it.

THE LINEAGE CLAIM, VERIFIED AT COMMIT LEVEL RATHER THAN ACCEPTED: MISSED TWIN, NOT FIX-INDUCED RESIDUE. `git log --oneline -S "only part of the mechanism that changes what a currently-succeeding" -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` returns exactly one commit, `8db3f83` ("docs: fold the refusal-scope decision and schedule the TMPDIR suite defect"). `8db3f83` precedes the round 1 fix pass (`47c6460`), the round 2 fix pass (`d9726fa`), the round 3 fix pass (`1a1655c`) and the supplementary pass (`e34c2c9`). The sentence has stood unchanged since it was authored, so no fix pass wrote it and no fix pass made it false. `R4B-3`'s lineage claim is TRUE as stated.

IT IS GENUINELY THE THIRD SITE OF ONE CLAIM FAMILY, WHICH I CHECKED AGAINST THE EARLIER RULINGS RATHER THAN TAKING ON TRUST. Round 1's `EX-5` struck "NO new failure mode: every invocation that exited 0 before still exits 0" at what was then `:272`, scoped by `grep -c "still exits 0"`. Round 3's `R3B-1` struck "because it is the only increment that makes a previously-green run fail, and" at `:290`, scoped by `grep -c "only increment that makes a previously-green"`. Both scopes were applied faithfully; neither literal could match `:280`'s wording. All three claims are the same family (a statement about which increment turns a currently-succeeding invocation into a differently-reporting one) and all three are falsified by the same evidence, inc1's checks 3 to 7. Round 3's triage already accepted the `EX-5` to `R3B-1` lineage on the same reasoning.

WHY `low`, AND I CONSIDERED `medium` SERIOUSLY BECAUSE THIS IS THE THIRD SITE. Round 3 held `R3B-1` at `low` on the ground that the operative facts elsewhere were correct and that lineage from a `medium` finding is not itself a merit. The same containment holds here and is if anything stronger: `:274` states inc1's true property in inc1's own bullet, `:296` states what inc1 changes, `:298` states inc2's break, `:286` states the cost of inc2's placement, and checks 3 to 7 each demand a changed report from inc1. A reader who takes `:280` literally is contradicted by five other passages, four of them acceptance checks with expected outputs. Severity is absolute impact if unfixed. THE ROUND A FINDING ARRIVES IN IS NOT A MERIT, AND NEITHER IS BEING THE THIRD SITE OF A FAMILY; both would be reasons to inflate, and I decline both. `low`.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. DELETION CLASS, ONE SITE (`grep -c "only part of the mechanism"` returns 1 in `workflow-enforcement-tier.md` and 0 in the other two sidecars and the TOML), plus a re-render.

At `:280`, strike the whole first clause including its semicolon, that is "It is the only part of the mechanism that changes what a currently-succeeding invocation REPORTS, whether by failing (the validator) or by withholding (the projections); ", leaving:

"WHY THE PREDICATE IS ITS OWN INCREMENT. It carries a known false positive (accepted cost (ii)); and it deliberately uses a DIFFERENT resolution from the default, so its review must check the lexical/canonical SPLIT rather than one rule."

I PREFER `R4B-3`'s FULLER DELETION OVER `R4A-1`'s EIGHT-WORD ONE, AND THE REASON IS RE-SEED RISK RATHER THAN SIZE. `R4A-1` proposes striking only "is the only part of the mechanism that", leaving "It changes what a currently-succeeding invocation REPORTS, whether by failing (the validator) or by withholding (the projections)". That residue is TRUE of inc2 and grammatical, so it is not wrong. But the paragraph it sits in is a DISCRIMINATIVE list: every other item in it (a known false positive; a different resolution from the default) is a property inc2 has and the other increments do not, and the paragraph's whole job is to justify why the predicate needs its own increment. Leaving a non-discriminating property at the head of that list invites precisely the next round's finding, that the first reason is not a reason. Nothing is lost by the fuller deletion: the fact that inc2 changes what currently-succeeding invocations report survives verbatim at `:298` ("It INTRODUCES a non-zero exit on validator invocations that succeed today AND withholds output from projection invocations that answer today"), in the risk section where it is load-bearing. Both forms are deletion class; take the one that leaves nothing to argue with. AUTHOR NO REPLACEMENT CLAUSE. Round 3 established that the two positive replacements available in this family are themselves false on the facts.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS AND NEGATIVES:

- `only part of the mechanism`: 1 site, `:280`. `previously-green`, `only increment that makes a previously-green`, `still exits 0` as a safety claim, `exited 0 before`: the first two return 0 everywhere (round 3's fix is complete), and `still exits 0` returns 1, at `:321` inside check 14, which is a correct statement about the refusal not firing without `--workflow` and which rounds 1 and 3 both cleared. Do not touch it.
- `is the only` / `only the`, as the general exclusivity-claim pattern rather than as a literal: hits at `:51` (the only thing on stdout), `:111` (the only invocation the scaffolded guidance documents, which is `R4B-4`'s site for a different reason), `:146` (the only two places `pack/AGENTS.md` mentions the log, verified TRUE by `grep -c`), `:162` (a quoted self-report by an explorer, not a claim by the document), `:172` (the refusal remains the validator's alone, TRUE and consistent with `:275`, `:321`, `:374` and TOML `:1702`) and `:298` (the only place where two different resolutions run against each other, TRUE: inc1 is lexical only per `:158`, inc3 has no resolution rule). Only `:280` is false.
- No enumeration is headed by the deleted clause and no cross-reference restates it. `:282` and `:284` argue from the increment-division rule at `:272`, not from `:280`.
- RENDER MIRROR OWED: `docs/plans/agent-scaffold.md:1675`, which carries the sentence verbatim. `R4A-1`'s citation of that line is correct.

## `R4B-4`. `VALID`. Severity `low` (unchanged). The defect B end property requires a run from the plan's own project root to be unchanged, and accepted cost (ii) makes exactly such a run exit 1

TENSE APPLIED: FORWARD, AND THE TWO TENSES DIFFER, WHICH IS WHY THIS IS A FINDING. Against today's tree `:111` is unobjectionable, because it is a requirement on work not yet done and nothing has changed. Against the tree inc2 produces it is false, because the layout it says must be unchanged is refused. I state the difference because the reviewer did and because the ruling depends on it.

BOTH SIDES REPRODUCED.

Side A, `workflow-enforcement-tier.md:111`: "THE REQUIRED END PROPERTY, which is what 'done' means for this half regardless of the mechanism: `validate --workflow` must never pair a plan source with a metrics log belonging to a different project and report success. Where the tool cannot establish that the two belong together, it must say so and exit non-zero rather than proceed. A run made from the plan's own project root, which is the normal invocation and the only one the scaffolded guidance documents, must be unchanged (Safe on existing projects)."

Side B, `workflow-enforcement-tier.md:258`: "(ii) A SYMLINKED `docs/plans` DIRECTORY BECOMES A FALSE POSITIVE ON THE PREDICATE. Where `<root>/docs/plans` is a symlink to `<root>/elsewhere`, the lexical default and the canonical guard disagree about which project the plan belongs to, and the guard wins: A measured this layout going from reading its 37-record log to `exit=1 REFUSED`. This is a genuine new failure for a layout that works today, and it is the main false-positive risk in the mechanism." Check 19, `:333`: "a layout where `<root>/docs/plans` is a SYMLINK to a sibling directory is REFUSED under `validate --workflow` after inc2, with the refusal message and a non-zero exit".

THE TWO SIDES ARE IN CONTRADICTION ONCE READ IN CONTEXT, WHICH I TESTED RATHER THAN ASSUMED. The refused run IS a run made from the plan's own project root: `<root>` is the project root by the document's own lexical convention, the run is the one that reads that project's own 37-record log today, and `:258` says in terms that it is "a genuine new failure for a layout that works today". Both sides sit inside the defect B half that `:111` scopes itself to, because the containment refusal is that half's second mechanism (`:164`, "THE REFUSAL (candidate (b), layered on top)"), so the scoping does not rescue side A.

THE CONTRADICTION IS ALSO INTERNAL TO `:111`, WHICH I VERIFIED AND WHICH IS THE SHARPEST FORM OF IT. Sentence two ("Where the tool cannot establish that the two belong together, it must say so and exit non-zero rather than proceed") and sentence three ("A run made from the plan's own project root ... must be unchanged") give opposite verdicts on the symlinked layout, and the document resolves the conflict 147 lines later in favour of sentence two ("the guard wins"). A finding whose two sides sit in the same sentence pair needs no cross-section reading to establish.

THE SECOND FALSIFIER IS WEAKER AND I DO NOT REST THE RULING ON IT. `R4B-4` offers inc3 and check 15 as a second falsifier if `:111` is read as a property of the finished step. `:111` explicitly scopes itself to the defect B half ("what 'done' means for this half"), and inc3 belongs to defect A, so the second falsifier is out of `:111`'s stated scope. The reviewer flags this conditionality itself, correctly. The finding stands on the first falsifier alone.

I AM NOT REOPENING ACCEPTED COST (ii), AND NEITHER IS THE REVIEWER. `:254` tells a reviewer not to raise the accepted costs and `R4B-4` says explicitly that it is not raising one. What is defective is the unqualified "must" in the end-property statement, not the decided behaviour.

WHY `low`, AND I CONSIDERED `medium`. The upgrade case is real: an implementer who holds `:111` as the acceptance bar and tries to satisfy it would have to weaken the canonical guard, which `:166` says MUST NOT BE COLLAPSED, and that would reintroduce the defect class the increment exists to close. What holds it at `low` is that four independent passages block that path: `:166` forbids collapsing the split, `:254` forbids "fixing" the accepted costs, `:258` names this exact layout as an accepted new failure, and check 19 at `:333` pins the refusal as expected behaviour. An implementer reaching the bad outcome has to ignore all four. Severity is absolute impact if unfixed, and the containment is strong. `low`.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. ONE CLAUSE APPENDED INSIDE AN EXISTING SENTENCE, ONE SITE (`grep -rn "must be unchanged"` returns exactly one hit, `:111`), plus a re-render. At `:111`, append to the third sentence so it reads "... must be unchanged (Safe on existing projects), except for the symlinked-`docs/plans` layout recorded below as accepted cost (ii)."

WHY NOT A DELETION, WHICH IS THE CLASS I WOULD OTHERWISE PREFER. Striking the third sentence outright removes the contradiction, and the no-regression property does survive elsewhere in a precise form at check 9 (`:316`, byte-identical stdout on this repository's own plan after inc1). But the third sentence is the only statement in the defect B section of what the NORMAL invocation must keep doing, and it is what a reviewer of inc1 reads to know that Safe on existing projects is the governing principle for the correct case. Deleting it trades a false absolute for a missing requirement. The appended clause authors nine words and no new vocabulary: "accepted cost (ii)" is the document's own established phrase, used at `:258`, `:280` and `:333`, and "symlinked `docs/plans`" is used at `:258` and `:282`.

WHAT I SWEPT, INCLUDING NEGATIVES:

- `must be unchanged`: 1 site, `:111`. No twin in either other sidecar or the TOML.
- `unchanged` as a bare word across all three sidecars and the TOML, checked for absolute twins a literal could miss: `status-resume-ignores-json.md:118` ("unchanged when `--json` is absent", a different subject), `workflow-enforcement-tier.md:158` ("the historical CWD-relative path stands unchanged", a true statement about the no-anchor case), `:220` (the human golden unchanged by the retype, true), `:244` (candidate (d)'s absent-field policy, unrelated), `:250` (unrelated), `:323` check 14c ("prints the plan half unchanged", correct and scoped), `:328` check 14h ("the normal case is unchanged on the machine surface EXCEPT for the new always-present fields", which already carries its own exception), and three unrelated TOML hits at `:1748`, `:1755`, `:1762`. None is an absolute end-property twin.
- Check 9 (`:316`), the closest thing to a twin: "AFTER INC1, NO REGRESSION ON THE CORRECT CASE ... from the agent-scaffold repository root ... its three stdout lines are BYTE-IDENTICAL to the pre-fix binary's". CORRECT AS WRITTEN and MUST NOT CHANGE: it is scoped to inc1, to one named non-symlinked layout, and it is a concrete run rather than an absolute.
- RENDER MIRROR OWED: `docs/plans/agent-scaffold.md:1506`.

## `R4B-5`. `VALID`. Severity `low` (unchanged). The inc1 documentation-impact bullet states the new ledger default unconditionally; the inc1 description keeps the old default for the no-source case

TENSE APPLIED: FORWARD, and the two tenses differ. Against today's tree neither side is a claim about anything that exists. Against the tree inc1 produces, `:343` instructs three help strings to say something that is false for a reachable invocation, and `:274` states the rule that makes it false.

BOTH SIDES REPRODUCED.

Side A, `workflow-enforcement-tier.md:343`, the INC1 documentation-impact list: "- `src/main.rs:461` (`StatusArgs::resume`), `:464-466` (`StatusArgs::ledger_fragment`) and `:482-484` (`NextArgs::ledger_fragment`), all of which say the default is `docs/plans/<task>.ledger.md`; after inc1 it is `<task>.ledger.md` BESIDE the plan source."

Side B, `workflow-enforcement-tier.md:274`, the inc1 description: "with NEITHER, the ledger keeps today's `docs/plans/<task>.ledger.md`, as the metrics rule keeps its CWD-relative path for the same case." The same rule is stated for the metrics half at `:158` ("With neither a `--source` nor a `--plan` there is nothing to anchor to, so the historical CWD-relative path stands unchanged") and pinned by check 10 at `:317` ("Same for bare `agent-scaffold validate` with no source at all, which has nothing to anchor to and keeps the CWD-relative path").

THE OMITTED CASE IS REACHABLE, WHICH I ESTABLISHED BY RUNNING THE TOOL RATHER THAN BY READING THE CODE ALONE. `StatusArgs::source` and `StatusArgs::plan` are both plain `#[arg(long)] Option<PathBuf>` with no `required` (`src/main.rs:450-454`), and the same for `NextArgs` (`:474-478`), so an invocation with neither is valid. `derive_task` (`src/next.rs:993-1003`) falls back to the literal `"task"` at `:1002` (`.map_or_else(|| "task".to_string(), task_from_filename)`), and `run_resume` (`src/main.rs:1153-1154`) then calls `default_ledger_path(&task)`, which builds `docs/plans/{task}.ledger.md` (`:1136-1138`). Run from an empty directory outside any project at `e34c2c9`:

```
$ agent-scaffold status --resume
no ledger at docs/plans/task.ledger.md; nothing to resume
exit=0
```

That is a working, exit-0 invocation whose ledger path is CWD-relative and which `:274`'s own rule leaves unchanged after inc1. `R4B-5`'s three code citations all resolve (I read `src/next.rs:993-1003`, `src/main.rs:1152-1154` and `:1136-1138`); the `run_resume` call is at `:1153-1154` within the cited `:1152-1154` range.

WHY IT MATTERS RATHER THAN BEING A PEDANTRY. `:343` is the instruction for what three user-facing help strings must say after inc1. An implementer following it literally writes a help string that is false for the no-source invocation, in the release whose entire subject is a tool that names the wrong file.

WHY `low`. Impact if unfixed is one over-general clause in three help strings, about a path, in the same increment whose own description states the correct rule one section away and whose check 10 pins the parallel metrics case. This is the same band round 1 gave `EX-6` and round 2 gave `INC2-6`, both stale-path documentation-impact defects, and I keep the band for consistency rather than because the number is convenient.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. NARROWING CLASS, ONE SITE (`grep -rn "BESIDE the plan source"` returns exactly one hit, `:343`; `beside the plan` lowercase returns 0), plus a re-render. At `:343`, narrow the closing clause to "after inc1 it is `<task>.ledger.md` BESIDE the plan source when there is one."

FIVE WORDS, AND SMALLER THAN THE REVIEWER'S OWN PROPOSAL, which restated the fallback path in full. The fallback is already stated twice, at `:274` and `:158`, and check 10 pins it; a third statement would be the restatement class this project's calibration data warns about. "when there is one" narrows the absolute without repeating the rule. A pure deletion of the whole "after inc1 ..." clause is NOT acceptable here: the bullet's value is telling the implementer what the three help strings must say, and deleting it leaves three named sites with no instruction.

WHAT I SWEPT, INCLUDING NEGATIVES:

- `BESIDE the plan source`: 1 site, `:343`. No twin in either other sidecar or the TOML.
- The ledger rule stated elsewhere, checked for twins that would need the same qualifier: `:136` ("the ledger NEEDS NO ROOT DERIVATION AT ALL, because it lives BESIDE the plan, so the source's own directory is the whole rule") is a statement about the DERIVATION RULE, not about the default in the no-source case, and it is correct as written; `:274` and `:158` already carry the qualification; `:278` ("The ledger path is a genuinely DIFFERENT rule (the source's own directory, no upward walk, no fallback)") is about the rule's shape and needs no change. `:344`'s bullet names `default_ledger_path`'s and `run_resume`'s doc comments without asserting the post-inc1 default, so it is unaffected.
- RENDER MIRROR OWED: `docs/plans/agent-scaffold.md:1738`.

## `R4B-6`. `VALID`. Severity `low` (unchanged). The predicate drives "two responses" at two sites and "three responses" at three others, and the two three-counts name different sets

TENSE APPLIED: PRESENT. All five sites are claims about the design as specified, not about any tree, and the two tenses do not differ.

ALL FIVE SITES REPRODUCED.

Side A, TWO. `workflow-enforcement-tier.md:168`, the section heading: "## One predicate, two responses: the validator refuses, the projections omit (`Q-55-refusalscope`)". `workflow-enforcement-tier.md:284`: "That framing is wrong and the right one is stronger: the JSON reason is not a third response, it is the SECOND RENDERING of one response. The predicate yields two responses (refuse, omit), and the omit has two renderings (human text, JSON)."

Side B, THREE. `:180`: "The trigger in all three cases is the SAME containment predicate the validator's refusal uses (the canonically-derived plan root, and whether the resolved artifact lives under it). One predicate, three consumers, three responses." `:282`: "SECOND, the three responses are only reviewable AGAINST EACH OTHER." `:298`: "one predicate now drives THREE responses, two of which must NOT fail".

IS THIS A GENUINE CONTRADICTION OR TWO CORRECT STATEMENTS ABOUT DIFFERENT SETS? THE BRIEF ASKS AND I TESTED BOTH.

The reviewer's set analysis is CORRECT and I confirmed it. At `:180` the antecedent of "all three cases" is the bullet list immediately below, `status` (`:182`), `status --resume` (`:183`) and `next` (`:184`), so its three EXCLUDE the validator. At `:298` the three must INCLUDE the validator, because "two of which must NOT fail" identifies the other two as the projections and the whole clause is contrasted with "refuses on a surface that must never refuse". Two different memberships under the same numeral.

That alone would be weak, because each site is locally self-disclosing and neither is false of its own referent. WHAT MAKES IT A CONTRADICTION RATHER THAN POLYSEMY IS `:284`'s OWN NUMERAL, which the reviewer identifies and which I re-derived. `:284` says "The tempting framing is that this is the same predicate yielding a THIRD response". The ordinal THIRD is computed from a baseline of TWO, and `:284` states that baseline in the next sentence. `:282` is TWO PARAGRAPHS EARLIER in the same subsection and says "the three responses". Under `:282`'s count the JSON reason would be a FOURTH thing and the framing `:284` sets up and rebuts does not arise. So the document's own load-bearing argument for placing the serialised reasons in inc2 is computed against a baseline another paragraph of the same argument contradicts. That is a real internal contradiction, not two views of one polysemous word. The same collision occurs on a smaller scale between `:168`'s heading and `:180`, twelve lines apart in the same section.

WHICH SIDE IS WRONG: the three-counts at `:180`, `:282` and `:298`. `:168` and `:284` use "response" in the kinds sense (refuse, omit), which is the sense `:284`'s argument requires and which `:168` defines in its own heading text ("the validator refuses, the projections omit"). The argument's CONCLUSION survives either sense (a rendering is not a response either way), which is why this is a term-and-numeral defect and not a broken argument, and why it is `low`.

I REJECT THE REVIEWER'S PRESCRIBED FIX AND SUBSTITUTE MY OWN, BECAUSE ITS SUBSTITUTIONS RELOCATE THE AMBIGUITY RATHER THAN REMOVING IT. `R4B-6` proposes "three responses" -> "three consumers" at `:298` and "the three responses" -> "the three consumers' answers" at `:282`. Applied literally, `:180` would then say "three consumers" meaning {`status`, `status --resume`, `next`} and `:298` would say "three consumers" meaning {`validate`, `status`, `next`}, which is the SAME defect the finding raises, moved from one noun to another. This is exactly the manufactured-next-round shape the brief warns about, and it is why I re-derived the fix rather than adopting it.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. DELETION CLASS THROUGHOUT, THREE SITES, plus a re-render. No substitutions, no new vocabulary, and no numeral left attached to "responses" anywhere except the two-count the argument needs.

- `:180`: delete the whole sentence "One predicate, three consumers, three responses.", leaving "The trigger in all three cases is the SAME containment predicate the validator's refusal uses (the canonically-derived plan root, and whether the resolved artifact lives under it). The predicate is never re-implemented per surface (One source of truth)." The deleted sentence is a rhetorical restatement; both operative statements around it survive intact, and "all three cases" keeps its correct antecedent in the bullets below. I delete the sentence rather than only its second half because "One predicate, three consumers" left standing would carry a count whose membership excludes the validator inside a section whose heading includes it, which is a smaller version of the same defect.
- `:282`: delete "three", leaving "SECOND, the responses are only reviewable AGAINST EACH OTHER." The very next sentence supplies the membership without a numeral ("one predicate yields DIFFERENT answers on the validator and on the projections"), so nothing is lost, and `:284`'s "THIRD response" ordinal is no longer contradicted two paragraphs above it.
- `:298`: delete "THREE", leaving "one predicate now drives responses, two of which must NOT fail, so the failure this increment can ship is not only ...". "two of which must NOT fail" keeps its referent from the clause that follows.

`:168` AND `:284` MUST NOT CHANGE. They are the correct sense and `:284`'s argument depends on them.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS AND NEGATIVES:

- `three responses`: 2 (`:180`, `:282`). `THREE responses`: 1 (`:298`). `two responses`: 2 (`:168`, `:284`), both correct. `three consumers`: 1 (`:180`). `all three cases`: 1 (`:180`), correct against the bullets and unaffected by the deletion.
- `responses` and `consumers` as bare words across all three sidecars and the TOML: `responses` appears ONLY in `workflow-enforcement-tier.md`, zero hits in the other two sidecars and zero in the TOML, so the TOML's `Q-55-refusalscope` record at `:1702` states the per-surface behaviour without a count and needs no edit. `consumers` returns `workflow-enforcement-tier.md:180` and `:304` (plus one hit in `doc-redundancy-cleanup.md`, a different step and outside this fold).
- `:304`, the closest semantic twin a literal for "three" could not reach: "one predicate with several consumers on two surfaces is not evidenced by testing one consumer on one surface". It uses "several" and carries NO numeral, so it is consistent under either sense and MUST NOT CHANGE. This is the negative that most nearly became a fourth site.
- `:275`, inc2's own description, which lists FOUR responses (`validate --workflow` refuses, `status` and `next` omit, `status --resume` omits the block) and states no count. Correct as written, and a supporting negative: the document's most complete enumeration of the per-surface answers deliberately carries no numeral.
- `:172` ("The refusal (exit non-zero) remains the validator's alone"), consistent with everything and unaffected.
- RENDER MIRRORS OWED: `docs/plans/agent-scaffold.md:1575` (`:180`), `:1677` (`:282`), `:1693` (`:298`).

## `R4B-7`. `VALID`. Severity `low` (unchanged). The three `resume_state` causes are said to be "already distinguished IN THE CODE" one sentence after the third is said to arrive with inc2

TENSE APPLIED: PRESENT, AND THE TWO TENSES DIFFER, WHICH DECIDES THE FIX SHAPE. The sentence carries "already" and cites today's line numbers, so it is a claim about the CURRENT tree, and against the current tree it is false. Against the tree inc2 produces, all three causes are distinguished and the sentence would be true with "already" removed. The present tense is the operative one because the sentence chooses it itself.

BOTH SIDES ARE IN THE SAME PARAGRAPH AND BOTH REPRODUCE, `workflow-enforcement-tier.md:226`: "This one is included rather than skipped because without it the same defect lands on a third field: after inc2, `next --json` can omit the resume block for a THIRD reason and report the same bare `null` it reports for the other two. The three causes are already distinguished IN THE CODE at `src/main.rs:1208-1212`, where `ledger_path.exists()` being false and `extract_resume_state` returning `None` are separate branches that both collapse to `None`, so naming them costs nothing beyond the naming."

Sentence three says the third cause arrives with inc2. Sentence four says all three are already distinguished in the code and cites lines the sentence itself then enumerates as two branches.

THE CODE CHECK, WHICH I RAN RATHER THAN TOOK FROM THE REVIEWER. `src/main.rs:1207-1212` at `e34c2c9` reads exactly:

```rust
let ledger_path = args.ledger_fragment.clone().unwrap_or_else(|| default_ledger_path(&task));
let resume_state = if ledger_path.exists() {
	next::extract_resume_state(&fs::read_to_string(&ledger_path)?)
} else {
	None
};
```

Two branches, and two causes distinguishable at that site: the ledger absent, and `extract_resume_state` returning `None` for a ledger with no section. The third variant, `ledger-not-this-project` (`:230`, "an explicit `--ledger-fragment` resolves outside the plan's project root"), depends on the containment predicate, which inc2 introduces and which does not exist at those lines. The citation itself is CORRECT; only the numeral is wrong. The reviewer's quoted snippet matches the file byte for byte and its line range is right.

WHY `low`. The wrong numeral sits three words from a citation that enumerates the two branches in the same sentence, and the argument it supports ("naming them costs nothing beyond the naming") is right for the two pre-existing causes and harmless for the third, which inc2 computes anyway. Nothing operative turns on it.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. NUMERAL-QUALIFIER CLASS, ONE SITE, plus a re-render. At `:226`, change "The three causes are already distinguished" to "Two of the three causes are already distinguished". Three words inserted, no new vocabulary ("two" already appears in the preceding sentence as "the other two").

I EXAMINED THE SMALLER EDITS AND REJECTED BOTH, AND ONE OF THEM IS THE ROUND 2 TRAP. Substituting "three" -> "two" is a one-word numeral edit and reads cleanly on its own, BUT `:226` IS IMMEDIATELY FOLLOWED BY A THREE-BULLET LIST, `ledger-absent` (`:228`), `no-resume-section` (`:229`) and `ledger-not-this-project` (`:230`). A "two" as the last numeral of the paragraph directly above three bullets is precisely the numeral-heads-an-enumeration shape this loop has already been bitten by, and it would manufacture the next round's finding. Deleting "already" instead does not fix anything: the cited lines still hold two branches and the sentence still enumerates two. The reviewer's own proposal is the correct one, and I adopt it unchanged because it KEEPS the "three" that matches the bullets while making the "already distinguished" claim true of the two the code distinguishes today.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS AND NEGATIVES:

- `The three causes`: 1 site, `:226`. `three causes` as a bare phrase: 3 sites, `:226`, `workflow-enforcement-tier.md:377` and `status-resume-ignores-json.md:97`.
- `:377` ("If that step ever takes the other fork, it REUSES the `resume_state_absent_reason` vocabulary specified here rather than minting a second one for the same three causes") and `status-resume-ignores-json.md:97` ("those are the same three causes a resume JSON surface would have to report") are BOTH CORRECT AND MUST NOT CHANGE. Both count the three VARIANTS the vocabulary specifies, which is three, not the causes distinguished in today's code. The prescribed fix preserves "three" at `:226`, so all three sites stay consistent. This is the check that decided the fix shape.
- The three-bullet variant list at `:228-230`: unchanged and consistent with "three".
- `already distinguished`, `already` as a today-claim marker near a forward statement: 1 relevant site. No twin elsewhere in the fold.
- RENDER MIRROR OWED: `docs/plans/agent-scaffold.md:1621`.

## Accepted residuals, not reopened

`INC2-7` (round 2, no precedence for an over-determined `no_active_loop_reason`, `:234`) and `F-5` (round 1, the dangling `validation-constraints` reference) are deliberately accepted. Neither reviewer raised either, both confirmed both still present, and I confirm them present and unchanged. NOT REOPENED, and no finding above touches `:234`.

## The producer's TOML `title` disclosure

The round 4 residue lens ruled the plan TOML's unprojected step `title` a pre-existing property of the tool and out of scope for this fold. I AGREE, and I verified the load-bearing half myself rather than accepting it: `src/plan/render.rs` reads `plan.meta.title` at `:296` and NOWHERE reads `step.title`; the only other `title` occurrences in that file are the section comment at `:694` and four test fixture strings. The Step Details heading is the sidecar's own `###` line, inlined verbatim. So `docs/plans/agent-scaffold.plan.toml:1322`'s title has no rendered projection, the fold's edit to it changed no output, and the divergence between that title ("... makes a correct tree fail (a false red, 3 tests)") and `test-tmpdir-repo-assumption.md:1`'s heading (the same text without the parenthetical) is a superset rather than a contradiction: the parenthetical is true and the two do not disagree about anything. NO FINDING RAISED, and I have not converted it into one.

## Round totals

- RAW FINDINGS: 8 (`R4A-1`; `R4B-1` through `R4B-7`).
- AFTER DEDUPLICATION: 7. `R4A-1` and `R4B-3` are one finding, merged; `R4B-3`'s analysis is the correct one and `R4A-1`'s concession of the "withholding" disjunct to inc2 is wrong, falsified by checks 3, 5, 6 and 7.
- VALID (FIX REQUIRED): 7.
- VALID BUT ACCEPT RESIDUAL: 0.
- DISMISSED: 0.
- SEVERITY MIX OF THE VALID SET: 0 critical, 0 high, 0 medium, 7 low. No reviewer rating changed in either direction. I considered `medium` for the merged `:280` finding (third site of a family that produced a `medium` in round 1) and for `R4B-4` (an absolute a literal implementer could act on by weakening the guard) and held `low` on both, because severity is absolute impact if unfixed and both are contained by four or five explicit passages elsewhere. Neither the round a finding arrived in nor its lineage is a merit, and neither is a demerit.
- TOTAL EDIT POINTS: ELEVEN, across TEN distinct lines. `workflow-enforcement-tier.md` `:111`, `:180`, `:226`, `:242`, `:275`, `:280`, `:282`, `:298` (TWO independent edits, one from `R4B-1` and one from `R4B-6`), `:343`; and `docs/plans/agent-scaffold.plan.toml:1704`. Nothing in `test-tmpdir-repo-assumption.md` or `status-resume-ignores-json.md`, and I swept both for every string above.
- FIX-CLASS BREAKDOWN BY SITE: FIVE DELETIONS (`R4B-2` 1, merged `:280` 1, `R4B-6` 3), FOUR NUMERAL OR NUMERAL-QUALIFIER EDITS (`R4B-1` 3, `R4B-7` 1), ONE NARROWING (`R4B-5`), ONE CLAUSE APPENDED INSIDE AN EXISTING SENTENCE (`R4B-4`). ZERO NEW SENTENCES AND ZERO NEW BULLETS. Nine of the eleven edits author no words at all; the two that do author fourteen words between them, all of it vocabulary already in the same document.
- RE-RENDER OWED: `docs/plans/agent-scaffold.md` carries every edited site as a generated projection (`:168`, `:1506`, `:1575`, `:1621`, `:1637`, `:1670`, `:1675`, `:1677`, `:1693`, `:1738`). Re-render rather than hand-edit, and confirm with `render --check`.
- FIX SET, IN DESCENDING ORDER OF WHAT IT BUYS: the merged `:280` deletion (the only finding that could mislead a reviewer about what a correct inc1 looks like, and the third site of a family two rounds have already paid for), then `R4B-5` (the only finding that instructs a false user-facing string), then `R4B-4`, then the count set (`R4B-1`, `R4B-6`, `R4B-2`, `R4B-7`), which can land in one pass.
- TWO FINDINGS ARE CONTINUATIONS AT SITES EARLIER FIXES COULD NOT REACH, AND I VERIFIED BOTH LINEAGES AT COMMIT LEVEL RATHER THAN ACCEPTING THEM. The merged `:280` finding pins to `8db3f83`, which precedes every fix pass, so it is a MISSED TWIN and not fix-induced residue. `R4B-1`'s three sites pin to `05a8898` or earlier, so they too predate every fix pass; the round 2 fix applied its stated two-site scope faithfully and inherited the scope's blind spot. Neither is damage a fix pass authored, and nothing in this round re-raises anything dismissed, because rounds 1, 2 and 3 dismissed nothing.
- NOTHING IN THIS ROUND TOUCHES A DECIDED ITEM. No finding re-litigates the enforcement tier, the one-step three-increment shape, anchor-plus-refusal, the conventionless fallback, omit-and-exit-0, the serialised reason, either accepted cost, or the nearest-wins judgement. `R4B-4` is about the wording of an end-property sentence and is not a request to change accepted cost (ii).
- THE ROUND IS NOT CLEAN. Seven valid findings, so the risky-classification streak does not advance and stands at 0 of 2. This is round 4 of a cap of 5.
