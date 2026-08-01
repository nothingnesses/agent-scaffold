# `workflow-enforcement-tier` plan review, verification round: mechanical lens

Reviewer model: Claude Sonnet 5. Exact model id `claude-sonnet-5`.
Worktree: `.claude/worktrees/verify-q55-a`, branch `verify/q55-a` at commit `61fc8b2`, the fix commit under review. Its parent is `c63a1e8`.
`TMPDIR` for any scratch work was `/tmp/verify-a-scratch`, outside any git repository.

## Scope

This is the single authorised verification round after an escalation, not round 1 of a fresh loop. The question is narrow: did the eleven edit points the round 4 triage (`workflow-enforcement-tier-planreview-r4-triage.md`) prescribed land as prescribed, and did the fix re-seed anything. `git diff c63a1e8 61fc8b2` is the whole change: 3 files, 20 insertions, 20 deletions (`docs/plans/agent-scaffold.md` 20, `docs/plans/agent-scaffold.plan.toml` 2, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` 18).

## Summary verdict: CLEAN. Zero findings.

All eleven prescribed edit points, across the ten prescribed lines, landed exactly as prescribed, in the prescribed form, closing all seven round 4 findings (`R4B-1`, `R4B-2`, merged `R4A-1`/`R4B-3`, `R4B-4`, `R4B-5`, `R4B-6`, `R4B-7`). The three triager-specific forms, which replaced forms the reviewers had proposed, all landed in the triager's own form:

- `R4B-2` landed as a numeral DELETION at `:242` ("rejected on measured grounds"), not a "five"->"four" substitution.
- `R4B-6` landed as THREE PURE DELETIONS at `:180`, `:282`, `:298` (no "consumers" substitution anywhere).
- Merged `R4A-1`/`R4B-3` landed as the triager's FULLER deletion at `:280` (the whole first clause including its semicolon), not the reviewer's eight-word one.

`:298` carries both `R4B-1`'s and `R4B-6`'s independent edits ("three"->"four doc comments" and the deletion of "THREE" before "responses"); both landed and neither overwrote the other.

The regenerated `docs/plans/agent-scaffold.md` matches a fresh render exactly: `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` reports `docs/plans/agent-scaffold.plan.toml: up to date`, exit 0. `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` passes cleanly on the resulting tree (`95 steps, 69 questions, valid`; `workflow invariants hold`).

No re-seeding found. No orphaned referents, no ungrammatical residue, no unedited twin of any edited string, anywhere in either other sidecar or the TOML. The two accepted residuals (`INC2-7` at `:234`, `F-5`, the dangling `validation-constraints` reference) and the unprojected TOML step `title` are all present and untouched, as required.

One observation, not a blocking finding: the "nine of eleven edit points author no words" bookkeeping in the round 4 triage's own round-summary line, repeated verbatim in the fix commit message, undercounts. My own mechanical word-diff (methodology below) finds THREE edit points insert net-new words, not two: `R4B-4` (+11), `R4B-7` (+2), `R4B-5` (+4). This is not a defect in the edited documents: every individual edit landed exactly as its own per-finding prescription specified, and the triage's own per-finding section for `R4B-7` already states "Three words inserted" for that exact site, correctly. The mismatch is between that correct per-finding statement and the aggregate round-summary line 30-odd lines later (and the commit message that inherited the aggregate line), not between the prescription and the landed text. All inserted words are pre-existing document vocabulary (verified). ZERO new sentences and ZERO new bullets were introduced (verified).

One producer disclosure was ruled on independently (below): not a defect, correctly left untouched, out of scope for this round regardless.

## Findings table

None. Zero `VA-` findings.

## Producer disclosure ruling: `workflow-enforcement-tier.md:170` "all four surfaces" vs. the quoted receipt's "all three"

RULING: NOT A DEFECT. Correctly out of scope and correctly left untouched by this fix.

THE TWO SITES, REPRODUCED. `workflow-enforcement-tier.md:170`: "`Q-55-mechanism`'s text (\"a refusal for unsafe pairings, covering `validate`, `next`, `status` and the ledger path\") admitted two readings: the refusal on the validator only, or the refusal on all four surfaces. [...] (receipt `q_id:\"Q-55-refusalscope\"`, options \"Omit the unsafe part, exit 0\" / \"Narrow: refusal on validate only\" / \"Wide: refusal on all three\", chosen OMIT THE UNSAFE PART, EXIT 0)." `docs/plans/agent-scaffold.plan.toml:1702` (and its mirror in the sidecar at the same line's paragraph two): "Chosen over refusal on the validator only [...] and over refusal on all three (which would have broken the projections' documented never-fails contract)."

WHY THIS LOOKS LIKE A CONTRADICTION ON A FIRST READ. The same sentence at `:170` first paraphrases the wide reading of `Q-55-mechanism`'s ambiguous text as covering "all four surfaces" (validate, next, status, and the ledger path, i.e. exactly the four items that text's own quoted clause names), then quotes the resolving receipt's own "Wide" option as "refusal on all three". If both are describing the identical rejected option, the numeral clashes.

WHY THEY ARE NOT THE SAME COUNT, AND WHY THAT IS NOT AN ERROR. "Three" is this document's own established, consistently-used convention for the NON-VALIDATOR set (the projections: `status`, `status --resume`, `next`), independent of this fix and predating it by three rounds. I confirmed this by grepping the review history: `workflow-enforcement-tier-planreview-r1-triage.md:76` ("line 180 makes containment the trigger for the omit on all three surfaces") and `workflow-enforcement-tier-planreview-r2-reviewer-inc2.md:234` ("Line 180 makes the same predicate the trigger on all three surfaces") both use "three" for exactly this set, in review rounds that closed before this fold's `R4B-6` finding even existed. `validate` already refuses under every option on the table, including the rejected "Narrow" one, so the "Wide" option's DELTA over "Narrow" is the three projections switching from omit/never-fail to refuse; "four" at `:170` is instead the TOTAL count of every surface `Q-55-mechanism`'s original text named (validate + the three projections, with the ledger path counted as a distinct fourth surface from plain `status`, which is exactly how `R4B-6`'s own set analysis treats them: `status`, `status --resume` and `next` are three separate consumers with three separate bulleted behaviours, `:182`-`:184`). Delta-of-three and total-of-four are both true, simultaneously, of the identical rejected option; they answer different questions ("what newly starts refusing" vs. "what refuses in total"), not different facts about what the option covers.

WHY IT IS OUT OF SCOPE REGARDLESS OF THE RULING. `:170` was not touched by this fix (it is not one of the eleven prescribed edit points; `git diff c63a1e8 61fc8b2` contains no hunk at `:170`), the receipt's quoted option text ("Wide: refusal on all three") is a verbatim historical decision record with a human timestamp and must not be edited to satisfy a paraphrase written around it, and the planner's own disclosed reasoning (four counts all surfaces including the validator; three is the receipt's own wording for the non-validator set; altering a quoted receipt would widen scope) matches what I independently derived. I checked whether this had already been examined and dismissed at record: it was not, but its two "half" sites (`R1-triage:76`, `R2-inc2:234`) were, without the numeral being flagged, which is corroborating rather than binding evidence for the reading. No `VA-` finding raised.

## Zero-new-sentences and word-count verification, methodology and result

CLAIM UNDER TEST: the round 4 triage's fix-class breakdown states "ZERO NEW SENTENCES AND ZERO NEW BULLETS. Nine of the eleven edits author no words at all; the two that do author fourteen words between them" (`-r4-triage.md`, round totals section). The fix commit message repeats "eleven edit points across ten lines, nine of which author no words."

METHOD. `git diff --word-diff=porcelain c63a1e8 61fc8b2 -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md docs/plans/agent-scaffold.plan.toml` (the two SOURCE files only; `docs/plans/agent-scaffold.md` is excluded because it is a generated mirror, not authored text, confirmed identical to a fresh render above). For each of the eleven edit points I take the OLD and NEW text of the changed span exactly as the porcelain diff delimits it, split each on whitespace, and compute NET WORD DELTA = (new token count) - (old token count). This sidesteps any subjective judgement about whether e.g. "The"->"the" or "projects)."->"projects)," is "the same word", since the net delta is identical either way.

RESULT, PER EDIT POINT (old tokens -> new tokens, net delta):

| site | finding | old | new | net |
| --- | --- | --- | --- | --- |
| TOML `:1704` | R4B-1 | `three` (1) | `four` (1) | 0 |
| sidecar `:111` | R4B-4 | `projects).` (1) | `projects), except for the symlinked-`docs/plans` layout recorded below as accepted cost (ii).` (12) | +11 |
| sidecar `:180` | R4B-6 | `One predicate, three consumers, three responses.` (6) | (deleted, 0) | -6 |
| sidecar `:226` | R4B-7 | `The` (1) | `Two of the` (3) | +2 |
| sidecar `:242` | R4B-2 | `five` (1) | (deleted, 0) | -1 |
| sidecar `:275` | R4B-1 | `three` (1) | `four` (1) | 0 |
| sidecar `:280` | merged R4A-1/R4B-3 | 25-token clause (counted by direct split) | (deleted, 0) | -25 |
| sidecar `:282` | R4B-6 | `three` (1) | (deleted, 0) | -1 |
| sidecar `:298` (doc-comment count) | R4B-1 | `three` (1) | `four` (1) | 0 |
| sidecar `:298` (response count) | R4B-6 | `THREE` (1) | (deleted, 0) | -1 |
| sidecar `:343` | R4B-5 | `source.` (1) | `source when there is one.` (5) | +4 |

TOTALS. Net word-count change across both source files: -17 (the fold shrinks by 17 words net, dominated by the 25-word `:280` deletion). Gross words inserted (sum of positive deltas only): 17, from THREE sites (`R4B-4` +11, `R4B-7` +2, `R4B-5` +4), not two. Gross words deleted (sum of negative deltas): 34, from the six deletion/numeral sites plus the negative half of `:280`.

READING AGAINST THE CLAIM. "ZERO NEW SENTENCES": TRUE, verified by inspection of all three word-adding sites. `R4B-4`'s appended clause is joined to its sentence by a comma, not a new terminal period; `R4B-7`'s edit is a subject-phrase substitution inside one continuous sentence; `R4B-5`'s "when there is one" is a subordinate clause inside the existing sentence, with the original single terminal period retained. "ZERO NEW BULLETS": TRUE, verified; no `- ` or numbered list item was added anywhere in the diff (`git diff --word-diff=porcelain` shows no line beginning a new bullet marker on any `+`-only span). "NINE OF ELEVEN AUTHOR NO WORDS; THE TWO THAT DO AUTHOR FOURTEEN WORDS": not accurate as literally stated. Three edit points insert words, not two, and my total (17 gross / 15, treating the `R4B-4` "projects),"-for-"projects)." swap as free since only trailing punctuation changed, giving 11 for `R4B-4` + 4 for `R4B-5` = 15 if `R4B-7` is excluded by the same "numeral-qualifier, not prose" convention the triage's fix-class table uses) does not match either the triage's 14 or the commit message's inherited 9-vs-2 framing under any convention I could construct, because the triage's own per-finding section for `R4B-7` (`-r4-triage.md` line ~263: "Three words inserted") already contradicts its own later round-summary bucketing of `R4B-7` under the numeral-edit class it counts as word-free. This is pre-existing imprecision in a prior, already-closed round's document and in the commit message that echoed it; it is not a defect in the plan documents under review, since every individual edit's TEXT matches its own per-finding prescription exactly (verified site by site above and in the enumeration below). VOCABULARY CHECK: `accepted cost (ii)` appears 4 times and `symlinked \`docs/plans\`` appears once in the PRE-FIX tree (`git show c63a1e8:...` greps, both confirmed), and `other two` appears once, substantiating "no new vocabulary" for all three word-adding sites despite the word-count bookkeeping being off by one site.

No `VA-` finding raised for this: it is a description-accuracy issue in commit-message and prior-round bookkeeping text, not in the artifact.

## Enumeration

### The eleven edit points, each verdict

1. `docs/plans/agent-scaffold.plan.toml:1704` (`R4B-1`, third site). Prescribed: "three"->"four". Landed: `four doc comments that claim the JSON contract is exhaustive...`. VERDICT: correct, exact match.
2. `workflow-enforcement-tier.md:111` (`R4B-4`). Prescribed: append `, except for the symlinked-\`docs/plans\` layout recorded below as accepted cost (ii).` to the third sentence. Landed: byte-identical to the prescribed text. VERDICT: correct, exact match. Negative confirmed: check 9 at `:316` ("byte-identical to the pre-fix binary's") and check 19 at `:333` both unchanged, as required.
3. `workflow-enforcement-tier.md:180` (`R4B-6`, first site). Prescribed: delete the whole sentence "One predicate, three consumers, three responses." Landed: sentence fully absent; the two flanking sentences read on unbroken. VERDICT: correct, exact match. `:168`'s heading ("two responses") and `:284` ("two responses... THIRD response") both left untouched, as required.
4. `workflow-enforcement-tier.md:226` (`R4B-7`). Prescribed: "The three causes are already distinguished" -> "Two of the three causes are already distinguished". Landed: exact match. VERDICT: correct. Negative confirmed: the three-bullet variant list at `:228`-`:230` unchanged, and the two other "three causes" twins (`:377`, `status-resume-ignores-json.md:97`) both correctly left as "three" (they count variants, not code branches).
5. `workflow-enforcement-tier.md:242` (`R4B-2`). Prescribed: strike "five" (deletion, not a "four" substitution). Landed: "It is rejected on measured grounds" -- no numeral present. VERDICT: correct, and correctly the triager's own prescribed form, not the reviewer's "four" substitution. Negative confirmed: `:248`'s "the four grounds above" unchanged.
6. `workflow-enforcement-tier.md:275` (`R4B-1`, second site). Prescribed: "three"->"four". Landed: "with the four falsified doc comments corrected". VERDICT: correct, exact match.
7. `workflow-enforcement-tier.md:280` (merged `R4A-1`/`R4B-3`). Prescribed: strike the whole clause "It is the only part of the mechanism that changes what a currently-succeeding invocation REPORTS, whether by failing (the validator) or by withholding (the projections); " including its semicolon. Landed: exact match, the fuller deletion; no trace of the reviewer's eight-word alternative. VERDICT: correct, and correctly the triager's fuller form. Negative confirmed: no enumeration or cross-reference restates the deleted clause; `:282` and `:284` argue from `:272` (independently verified by reading both in full), not from `:280`.
8. `workflow-enforcement-tier.md:282` (`R4B-6`, second site). Prescribed: delete "three", leaving "SECOND, the responses are only reviewable AGAINST EACH OTHER." Landed: exact match.
9. `workflow-enforcement-tier.md:298`, doc-comment count (`R4B-1`, first sidecar site). Prescribed: "three"->"four". Landed: "falsifying four doc comments and breaking a byte-compare golden". VERDICT: correct, exact match.
10. `workflow-enforcement-tier.md:298`, response count (`R4B-6`, third site). Prescribed: delete "THREE", leaving "one predicate now drives responses, two of which must NOT fail". Landed: exact match. VERDICT for the shared line: BOTH `R4B-1`'s and `R4B-6`'s edits present simultaneously, in different clauses of the same long line, neither overwrote the other.
11. `workflow-enforcement-tier.md:343` (`R4B-5`). Prescribed: narrow to "after inc1 it is `<task>.ledger.md` BESIDE the plan source when there is one." Landed: exact match. VERDICT: correct, and smaller than the reviewer's fuller restatement proposal, as prescribed. Negative confirmed: `:136`, `:158`, `:274`, `:278` (the twin sites the triage identified) all read correctly as-is, none needed and none received the qualifier.

### Commands run, with counts

- `git diff c63a1e8 61fc8b2 --stat`: `docs/plans/agent-scaffold.md | 20`, `docs/plans/agent-scaffold.plan.toml | 2`, `.../workflow-enforcement-tier.md | 18`, `3 files changed, 20 insertions(+), 20 deletions(-)`.
- `git diff --word-diff=porcelain c63a1e8 61fc8b2 -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md docs/plans/agent-scaffold.plan.toml`: used for the word-count table above.
- `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` (via `just run render ... --check`): `docs/plans/agent-scaffold.plan.toml: up to date`, exit 0.
- `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` (via `just run validate ...`): `docs/metrics/workflow.jsonl: 244 records, valid`; `docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid`; `docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold`; exit 0.
- `grep -c 'validation-constraints' docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`: 4 (unchanged from round 3's own count; `F-5` residual confirmed present).
- `grep -c 'validation-constraints' docs/plans/agent-scaffold.plan.toml`: 3 (all three are prose references inside decision receipts, none is a `slug =` line).
- `grep -n '^slug = "validation-constraints"' docs/plans/agent-scaffold.plan.toml`: 0 hits. Confirms `F-5`'s load-bearing claim (no such step exists) still holds and was not "fixed" by this pass.
- `sed -n '234p' workflow-enforcement-tier.md`: `INC2-7`'s narrowed correlation-rule paragraph, present, unchanged.
- `grep -n '\bconsumers\b'` over both other sidecars + the primary sidecar + the TOML: 2 hits, `:304` (unrelated, "several consumers", no numeral, correctly untouched) and one hit in `doc-redundancy-cleanup.md` (a different step, outside this fold). Zero hits of "three consumers" anywhere post-fix.
- `grep -n '\bresponses\b'` over the same scope: 5 hits, all in `workflow-enforcement-tier.md` (`:168`, `:275`, `:282`, `:284`, `:298`), none carrying a stray "three"/"THREE".
- `grep -rn 'already distinguished'`: 1 hit, `:226`, the fixed site.
- `grep -rn '\bgrounds\b'`: 2 hits, `:242` (fixed, no numeral) and `:248` (correct negative, "the four grounds above", unchanged).
- `grep -rni 'beside the plan'`: 3 hits, `:136` and `:250` (unrelated derivation-rule prose, correctly untouched) and `:343` (fixed).
- `grep -rn 'must be unchanged'`: 1 hit, `:111`, the fixed site. No twin in either other sidecar or the TOML.
- `grep -niE` over `test-tmpdir-repo-assumption.md` and `status-resume-ignores-json.md` for every fixed string (three/four doc, falsif, three consumer, three response, already distinguished, five measured, must be unchanged, beside the plan source, symlinked-`docs/plans` layout): 0 hits in both files. NEGATIVE: neither of the other two sidecars needed or received any edit, matching the triage's own sweep.
- `grep -n 'three causes'` over all sidecars: 2 hits, `status-resume-ignores-json.md:97` and `workflow-enforcement-tier.md:377`, both correctly left as "three" (they count the closed vocabulary's three variants, not the two branches distinguished in today's code, which is what `:226` alone was about).
- `grep -rn 'all four surfaces\|refusal on all three\|all three surfaces\|Wide: refusal'` across reviews, steps and the TOML: confirms `:170`'s "four surfaces" and the TOML's/sidecar's "all three" are the only two sites, plus two round 1/2 reviewer-file precedents for the "three surfaces = the projections" convention (`r1-triage.md:76`, `r2-reviewer-inc2.md:234`), both pre-dating this fold's findings and neither flagging the numeral.
- `git show c63a1e8:.../workflow-enforcement-tier.md | grep -c 'accepted cost (ii)'`: 4. `| grep -c 'symlinked \`docs/plans\`'`: 1. `| grep -c '\bother two\b'`: 1. All confirm the inserted words at `R4B-4` and `R4B-7` reuse pre-existing document vocabulary.

### Negatives, summarised

- No edit landed anywhere outside the eleven prescribed sites: `git diff --stat` shows exactly 10 changed lines across the two source files (9 in the sidecar, 1 in the TOML), matching "ELEVEN EDIT POINTS ACROSS TEN LINES" exactly, with no room for an unprescribed twelfth edit.
- No file outside the three (`agent-scaffold.md`, `agent-scaffold.plan.toml`, `workflow-enforcement-tier.md`) was touched. `test-tmpdir-repo-assumption.md` and `status-resume-ignores-json.md` are untouched and needed no twin fix, confirmed by direct grep, not by trusting the triage's own claim.
- No accepted residual was reopened or touched: `INC2-7` (`:234`), `F-5` (the dangling `validation-constraints` reference, count still 4/0), and the TOML's unprojected step `title` (`:1322`, confirmed still absent from `src/plan/render.rs`'s read set) all stand exactly as before.
- No orphaned referent: the `:280` deletion leaves "It carries..." and "Its correctness property..." both still anchored to "THE PREDICATE" in the same paragraph's opening sentence; the `:180` deletion leaves "all three cases" anchored to the bullet list immediately below it, unaffected since that deleted sentence was not its antecedent.
- The render is exact: `render --check` reports "up to date", not merely "close"; this is a byte-level guarantee, not a sampled one.
