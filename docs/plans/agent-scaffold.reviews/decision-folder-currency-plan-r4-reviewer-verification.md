# Plan-review round 4 findings, step 90 `decision-folder-currency` (SPLIT VERIFICATION AND REGRESSION lens)

Reviewer: independent, round 4, split-verification and regression lens. Worktree `.claude/worktrees/rev4-verify`, detached at `b36c4c6`. Artifact `ab6c01d..b36c4c6`; primary target the SPLIT commit `e47f4cf..b36c4c6`.

Every line cited below was re-read in THIS worktree at `b36c4c6` immediately before citing, and every command below was re-run here rather than copied from a previous round's file.

## Summary

Three findings, all `low`. No `critical`, no `high`, no `medium`: I looked for each and found none.

| id | severity | where |
| --- | --- | --- |
| `R4-1` | `low` | `docs/plans/agent-scaffold.steps/prompt-drift-guard.md:23` and `:25` (view `docs/plans/agent-scaffold.md:1267`, `:1269`) |
| `R4-2` | `low` | `docs/plans/agent-scaffold.steps/decision-folder-currency.md:46` (view `docs/plans/agent-scaffold.md:1237`) |
| `R4-3` | `low` | `docs/plans/agent-scaffold.plan.toml:1722` (view `docs/plans/agent-scaffold.md:196`) |

`R4-1` and `R4-2` are the same class: the split removed step 91 and updated every reference that NAMES the slug, but two prose references that say "step 91" or "its own ... step" without the slug were missed. A slug grep does not catch them; a "step 91" grep does.

The seven verification items I was given all pass except for the residue `R4-1` and `R4-2` describe under item 3. Details per item below the findings.

---

## `R4-1` (`low`): step 92's sidecar still documents an interaction with a step that no longer exists

Location: `docs/plans/agent-scaffold.steps/prompt-drift-guard.md:23` and `:25`, projected verbatim to `docs/plans/agent-scaffold.md:1267` and `:1269`.

Exact text, read at `b36c4c6`:

- `:23`: "The step adds no scaffolded content, so no deployed file goes stale and no regeneration is required, which is the reason it can land independently of steps 90 and 91."
- `:25`: "Interaction with steps 90 and 91. This step does not block either of them and neither blocks it. ... the order given (90, then 91, then 92) reflects that step 90 is already reviewed and in flight, not a judgement that the guard matters less. If the human wants the enforcement first, reordering costs nothing, since the two touch disjoint files."

Reproduce:

```
$ grep -n "steps 90 and 91\|90, then 91, then 92" docs/plans/agent-scaffold.steps/prompt-drift-guard.md
23:...independently of steps 90 and 91....
25:Interaction with steps 90 and 91....the order given (90, then 91, then 92)...
$ grep -c "^| " docs/plans/agent-scaffold.md      # 93 = 91 step rows + header + separator
93
$ grep -n "exploring-item-actor-boundary" docs/plans/agent-scaffold.plan.toml | grep -c "slug ="
0
```

There is no step 91 in the plan any more: `validate` reports 91 steps, the Roadmap's last two rows are `decision-folder-currency` and `prompt-drift-guard`, and `order` jumps 90 -> 92. Both sentences read in the present tense about a live sibling step, and `:25`'s "the order given (90, then 91, then 92)" describes a Roadmap ordering that no longer exists. An implementer entering step 92 is told to reason about an interaction with a step it cannot find, and the ordering rationale it is given is not the ordering in the plan.

Why it is the split's defect and not pre-existing: at `4de155a`, where this sidecar was authored, step 91 existed and both sentences were correct. The split commit `b36c4c6` deleted the step and left them. `git diff e47f4cf..b36c4c6 -- docs/plans/agent-scaffold.steps/prompt-drift-guard.md` is empty.

I record that the brief given to the fix pass said to leave step 92 EXACTLY as it was, and it was obeyed. That instruction is what produced the staleness, so this is a scoping consequence rather than carelessness; it is still a stale doc left by the change, which is a finding like any other under the reviewer role's documentation-currency duty. I raise it, not the obedience.

Severity `low`: step 92 is `not-started`, no decision rests on these two sentences, and the operative content of the step (what the guard must catch, the reuse notes, the scope boundary) is untouched and correct.

Cheapest fix: `:23` "it can land independently of step 90"; `:25` retitle to "Interaction with step 90" and drop the "(90, then 91, then 92)" parenthetical, or replace it with a note that the order now runs 90 then 92 because the former step 91 was withdrawn to a design pass.

---

## `R4-2` (`low`): step 90's scope-history bullet says the exploration-mode class was given "its own question and step", contradicting the same file two paragraphs earlier

Location: `docs/plans/agent-scaffold.steps/decision-folder-currency.md:46`, projected to `docs/plans/agent-scaffold.md:1237`.

Exact text at `:46`: "The exploration-mode class was raised by the plan-review triager (finding T-3a, `medium`, valid but out of scope) and the human kept it OUT of this step (2026-07-27), giving it its own question and step so the design gets decided rather than assumed. Do not fold it back in here."

Against `:40` in the same file, which the split commit DID update: "It is held as the queue item `Q-69`, status `exploring`, with a design pass owed and NO step yet".

Reproduce:

```
$ grep -n "its own question and step" docs/plans/agent-scaffold.steps/decision-folder-currency.md
46:- The exploration-mode class was raised by the plan-review triager...giving it its own question and step...
$ grep -n "NO step yet" docs/plans/agent-scaffold.steps/decision-folder-currency.md
40:...held as the queue item `Q-69`, status `exploring`, with a design pass owed and NO step yet...
$ git diff e47f4cf..b36c4c6 -- docs/plans/agent-scaffold.steps/decision-folder-currency.md | grep -c "^[-+]- The exploration-mode class"
0
```

The split commit rewrote the two step-91 references at `:20` and `:40` and left this third one. The three now disagree inside one sidecar: two say there is no step, one says the human gave it one.

I considered the defence that `:46` is past-tense history of a 2026-07-27 call, which was accurate when made. It does not hold, for two reasons. First, the bullet is not marked as superseded, unlike the `Q-69` `ask`'s equivalent sentence (`plan.toml:1734`), which explicitly says the step "was removed on 2026-07-28"; a reader has no signal that the second half of the clause has since been undone. Second, the section it sits in is headed "Scope history, so a later reader does not re-litigate it", so its function is to tell a later reader the settled disposition, and the disposition it states is no longer the one in the plan.

Severity `low`: the bullet's operative instruction ("Do not fold it back in here") is unaffected and correct, and `:40` states the current disposition correctly a few lines above.

Cheapest fix: "...giving it its own question (`Q-69`, now `exploring` with a design pass owed) so the design gets decided rather than assumed."

---

## `R4-3` (`low`, cosmetic): missing space before an opening quotation mark in the `Q-69` `ask`

Location: `docs/plans/agent-scaffold.plan.toml:1722`, projected to `docs/plans/agent-scaffold.md:196`.

Exact text: `Its MAIN clause is qualified:"Here 'updates this queue' means raising and pushing the open items, ...`

Reproduce:

```
$ grep -c 'qualified:"Here' docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md
docs/plans/agent-scaffold.plan.toml:1
docs/plans/agent-scaffold.md:1
```

Introduced by the split commit: the pre-split sentence read `its MAIN clause is ALREADY QUALIFIED, "Here 'updates this queue' means`, with a comma and a space. Every other quotation in the same `ask` is introduced with a space (for example `:1722` "TRAILING RATIONALE CLAUSE, after the colon, drops that qualifier: \"the generated isolation-policy fragment...\"").

Severity `low` and I want the record to show I regard it as cosmetic with no effect on meaning: it is one character in an item that is now `exploring`, no argument turns on it, and I would not object if the triager accepts it as a residual rather than spending a round on it. I report it because it is a real, objective defect the split commit authored and suppressing it is not mine to do. It costs one character to fix alongside either of the other two findings.

---

# Verification of the seven assigned items

## 1. `R3-2` genuinely fixed: PASS

The replacement clause at `docs/plans/agent-scaffold.steps/decision-folder-currency.md:7` reads:

"(Find that note by the quoted text, not by a line number. A line citation into the ledger rots because the file is edited IN PLACE, not only appended to: the resume block sits above this note and grew by ten lines mid-file, which moved the note from 345 to 355 and broke the citation an earlier draft of this sidecar carried.)"

The old, backwards reason ("the ledger is append-only in practice, so any line citation into it rots") is gone: `grep -c "append-only in practice" docs/plans/agent-scaffold.steps/` returns 0 across the tree.

Every factual claim in the replacement reproduces, re-run here:

```
$ git diff --numstat caeee2b e3fca03 -- docs/plans/agent-scaffold.ledger.md
11	1	docs/plans/agent-scaffold.ledger.md
$ git diff caeee2b e3fca03 -- docs/plans/agent-scaffold.ledger.md | grep '^@@'
@@ -334,7 +334,17 @@ ...
$ git show caeee2b:docs/plans/agent-scaffold.ledger.md | grep -n 'AND the parallel'
345:RESUME/NEXT: ...
$ git show e3fca03:docs/plans/agent-scaffold.ledger.md | grep -n 'AND the parallel'
355:RESUME/NEXT: ...
```

- "edited IN PLACE, not only appended to": correct. 11 added and 1 deleted at line 334 of a file then ~350 lines long is a mid-file replacement plus insertion, not an append.
- "the resume block sits above this note and grew by ten lines mid-file": correct and it is the actual mechanism. Reading the hunk body, what landed at 334 is a new `CURRENT TRANSIENT STATE (updated 2026-07-27 ...)` anchor plus its `IN FLIGHT`, `PLAN REVIEW ROUND 1`, `TWO NEW STEPS`, and `BLOCKED HERE` paragraphs, replacing the one-line header of the older anchor. That is insertion ABOVE the cited note, which is what the round-3 triager identified as the true cause and explicitly warned the planner not to describe as an in-place rewrite of the block. The new text does not make that mistake.
- "grew by ten lines": 11 added minus 1 deleted is net 10. Correct.
- "moved the note from 345 to 355": correct, shown above.
- "broke the citation an earlier draft of this sidecar carried": correct. `git show 4de155a:docs/plans/agent-scaffold.steps/decision-folder-currency.md | grep -o "ledger.md:3[0-9][0-9]"` returns `ledger.md:345` twice, and at `b36c4c6` the sidecar carries no `ledger.md:<line>` citation at all.

The reason now given is the real mechanism and is itself accurate, which is exactly what `R3-2` asked for.

## 2. `DEC-2`, `DEC-3`, `R3-1` gone, not reworded: PASS

```
$ grep -rc "wider reading" docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md docs/plans/agent-scaffold.steps/*.md   # 0 everywhere
$ grep -rn "wider half" docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md docs/plans/agent-scaffold.steps/          # no hits
$ grep -rn "breach" docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md docs/plans/agent-scaffold.steps/             # no hits
$ grep -rn "What is NOT established" docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md docs/plans/agent-scaffold.steps/   # no hits
```

Beyond the greps, I read the replacement paragraph (`plan.toml:1730`) to check the defects are gone in substance and not just in wording.

- `DEC-2`: the ambiguous comparative is gone entirely. `plan.toml:1722` now states the split concretely and with one referent each way: "So on the main clause the exclusion reaches only a DECIDED question's fold, while on the trailing clause it reaches a `[[question]]` of any status. Three shipped passages act on the narrower main-clause reading and are therefore in conflict with the trailing one". No sentence anywhere now claims the call sites follow the wide exclusion.
- `DEC-3`: the compliance verdict is gone and the accurate two-halves statement replaces it: "On that record, then: a planner authored the placeholder, and no review round was run on it." Nothing now says "NOT a breach", and no option-(b)-compliance claim survives, which is consistent since the option set itself is withdrawn.
- `R3-1`: the disclaimer-then-rely contradiction is gone. The paragraph now states the record and its reliance on it in one direction: "the only durable record that speaks to it is the ledger's contemporaneous entry ... and this item relies on that record as it stands rather than treating authorship as unknown." No sentence says authorship is not established.

I also re-verified the surviving factual claims in that paragraph rather than assuming the previous round's checks carry over, since the paragraph was rewritten: `grep -c "Q-68" docs/metrics/workflow.jsonl` returns 0; `b6ba317` (`2026-07-26 22:37:46 +0100`) adds `id = "Q-68"` to the plan source plus the empty `Q-68.md`; the ledger carries exactly one `NEW BACKLOG (captured 2026-07-26 by a planner, per Q-67)` and the commit that added it, `8d12264`, is dated `2026-07-26 22:40:29 +0100`, so "three minutes later" holds.

Not raised, and I record why so a later round does not spend time on it: the "`Q-67` had been live on main for about half an hour" clause is retained text that the round-3 decision reviewer explicitly examined and declined to raise (`decision-folder-currency-plan-r3-reviewer-decision.md:138`), noting it is accurate under the guidance-landing reading (`4f48283` author-dated `22:05:29`, 32 minutes before `b6ba317`) and off under the decision-recorded reading, with the conclusion holding either way. I reproduced both readings and have no new evidence that assessment was wrong.

## 3. No dangling step reference: PARTIAL, and the residue is `R4-1` and `R4-2`

The slug sweep is clean. `git grep -n "exploring-item-actor-boundary"` over the plan source, all sidecars, all question files, and the generated view returns exactly six hits, and every one is in the allowed set:

- `plan.toml:1720` and `agent-scaffold.md:194`: the exploration DIRECTORY path `docs/plans/exploring-item-actor-boundary.explorations/`.
- `plan.toml:1732` and `agent-scaffold.md:206`: the same directory path in "WHAT THE PASS OWES BACK".
- `plan.toml:1734` and `agent-scaffold.md:208`: the past-tense provenance sentence, which reads "An earlier draft of this fold carried a step (`exploring-item-actor-boundary`, order 91) whose sidecar enumerated a different edit set per option; it was removed on 2026-07-28 ...". Correctly past tense and explicitly records the removal.

No roadmap row and no `blocked_by`:

```
$ grep -n "blocked_by" docs/plans/agent-scaffold.plan.toml | grep -v "= \[\]"     # no hits, every blocked_by is empty
$ grep -n "^| " docs/plans/agent-scaffold.md | tail -2
303:| `decision-folder-currency` | next | why: decisions Q-67 |
304:| `prompt-drift-guard` | not started |  |
$ ls docs/plans/agent-scaffold.steps/ | grep -c "^exploring-item-actor-boundary.md$"
0
```

The sidecar `docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md` is deleted, as a committed deletion in `b36c4c6` (`git diff e47f4cf..b36c4c6 --diff-filter=D --name-only` lists it), so the commit-before-delete discipline holds and its reasoning is recoverable, as `plan.toml:1734` claims.

On the exploration directory name: `docs/plans/exploring-item-actor-boundary.explorations/` names a slug that is no longer a step, which I checked against the convention in `pack/AGENTS.md:65` (`docs/plans/<task>.explorations/`) before deciding not to raise it. The repo's established practice is mixed and topic-named directories are the norm, not the exception: `docs/plans/` already holds `code-value-audit.explorations/`, `review-mode.explorations/`, `structured-skeleton.explorations/` and eleven more alongside the task-named `agent-scaffold.explorations/`, and the sibling `exploring` item `Q-68` points at a not-yet-created `docs/plans/structured-ledger.explorations/` on exactly the same pattern. So this matches the in-plan precedent it was told to match, and `plan.toml:1734` tells a reader where the name came from. Not a finding.

What the slug sweep does NOT catch, and what I found by sweeping for the ORDINAL instead, is `R4-1` (`prompt-drift-guard.md:23`, `:25`) and `R4-2` (`decision-folder-currency.md:46`). Both treat the removed step as live without naming it. Recommended sweep for the fix pass:

```
$ grep -rn "step 91\|steps 90 and 91\|order 91\|own question and step" docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.md
```

## 4. The generated fragment is cited as evidence without reproducing its enumeration: PASS

`plan.toml:1724` quotes only the operative opening clause and the count: the fragment's "closing sentence begins \"The only edits made directly on main are\" and then names four integration edits. Read the fragment itself for those four; this item deliberately does not reproduce them, because that list is generated and single-sourced and a copy here could drift from it."

Checked against the const at `src/isolation_policy.rs:33`. Its closing sentence is "The only edits made directly on main are the orchestrator's own integration-level ones, which author no reviewed product content and so stay the orchestrator's direct job rather than a spawned agent's: flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor." The quoted opening clause is verbatim, and the item count is four: status flip, increment declaration, round record, ledger resume anchor. The count is right.

The enumeration is not reproduced anywhere in the `Q-69` `ask` or the generated view:

```
$ grep -rn "flipping a step's status\|declaring an increment\|recording a round record\|moving the ledger's resume anchor" docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.md
(no hits)
```

The subsidiary claim also reproduces: `grep -ci "question"` over the const's line returns 0, so "it contains no occurrence of the word 'question'" is accurate and verified against the const rather than asserted. The render-slot citation resolves too: `grep -n "isolation_policy" pack/AGENTS.md` returns exactly `91:{{isolation_policy}}`, so "rendered into the `{{isolation_policy}}` slot at `pack/AGENTS.md:91`" is right.

I did not raise the quoting of the operative clause, per the brief and on the merits: quoting the clause whose meaning is the unsettled premise is the minimum needed to state the premise, and it is a pointer to the single source rather than a second copy of the list.

Separately, `T-4`'s accepted residual, the paraphrase of the same list at `decision-folder-currency.md:26`, is still present and unchanged. The new `Q-69` text does not add a second instance of that residual class, so the accepted residual stays a single knowing exception rather than becoming a pattern.

## 5. `Q-69` is a well-formed `exploring` item: PASS

The whole entry is `plan.toml:1717-1734`: `[[question]]`, `id = "Q-69"`, `status = "exploring"` (`:1719`), `ask = """..."""`. There is no `folded_into` key and no `receipt` key, matching `Q-68` at `:1708-1715` exactly (`[[question]]`, `id`, `status = "exploring"`, `ask`), and unlike every `decided` item, which carries both (for example `Q-67` at `:1702-1706`).

No recommendation and no decided option set: `grep -c "RECOMMENDATION" ` over the `ask` returns 0, and `:1728` labels the directions as "CANDIDATE DIRECTIONS for the pass to weigh, extend, or discard. NOT a decided option set, no recommendation attached, and their costs move with premise 1." The lettering moved from `(a)/(b)/(c)` to `(i)/(ii)/(iii)`, which usefully makes the withdrawal visible rather than leaving the old labels to read as a live option set. The item also declines to pre-decide: `:1724` says "THREE CONSEQUENCES the pass must carry, none of them pre-decided here", and `:1726` says the boundary evidence "is recorded here as evidence rather than as a ruling".

Against `Q-68`, the in-plan precedent, the shapes line up point for point: both are `exploring`; both say a design pass is owed with a human-directed date; both point at an explorations directory by path, as `pack/AGENTS.md:65` requires while an item is `exploring`; both state the open questions the pass must resolve without pre-deciding them; both say there is no step and no receipt yet. `Q-69` adds "WHAT THE PASS OWES BACK" (`:1732`), which `Q-68` does not have; that is a superset, not a deviation, and its file-naming instruction (`Q-69.md`, or `Q-69-<disambiguator>.md` for parallel explorers) matches `pack/AGENTS.md:65` verbatim.

The no-receipt claim is correct on the schema: `plan.toml:1734` says W4 requires a receipt only for a `decided` item past the `[meta].w4_baseline` cutoff, and `validate --workflow` passes with `Q-69` receiptless, which is the mechanical confirmation.

Citation spot-check on the rewritten text, since the `ask` is largely new prose and only the previous version had been swept. All resolve verbatim in this worktree: `pack/AGENTS.md:45` ("the orchestrator records the question as an Open-Questions item with status `exploring`"), `:65` ("The Open-Questions item points at the exploration by path while it is `exploring`"), `:71` (both the main clause and the trailing clause, word for word; the main clause's inner double quotes around "updates this queue" are rendered as single quotes, an unchanged transcription convention the item carried before the split and which the round-3 triager's sweep already accepted), `:91` (the slot); `pack/prompts/orchestrator.md:31` ("record it as an `exploring` Open-Questions item", the third branch of the three-branch sentence); `pack/user-prompts/explore.md:13` ("record this as an `exploring` open question") and `:7` (the act-as-the-orchestrator instruction); `pack/pack.toml:166-167` (the `user-prompts/explore.md` source and dest pair) and `src/manifest.rs:615` (`".agents/user-prompts/explore.md"`); `src/plan/source.rs` (`QuestionStatus::Exploring`). No fifth line-number defect in the artifact.

## 6. Regressions and scope violations: PASS

- Step 92 untouched. `git diff e47f4cf..b36c4c6 -- docs/plans/agent-scaffold.steps/prompt-drift-guard.md` is empty, and its `[[step]]` block at `plan.toml:1252-1260` is unchanged (`order = 92`, `blocked_by = []`, `status = "not-started"`). `R4-1` is a consequence of that instruction, not a violation of it.
- `T-4` (accepted residual) present and unchanged: the paraphrase "a step's status flip, an increment declaration, a round record, and the ledger's resume anchor" is still at `decision-folder-currency.md:26`, in a paragraph the split commit did not touch.
- `R2-3` (accepted residual) present and unchanged: "the three actor-less `pack/AGENTS.md` prose points" is still in the `Q-67` `ask`, and `git diff e47f4cf..b36c4c6 -- docs/plans/agent-scaffold.plan.toml` shows no change to `Q-67`.
- `T-5` (dismissed) not reopened: "copy the pointing, not the list" is still at `decision-folder-currency.md:26`, unchanged, and nothing in the artifact re-argues it.
- `T-7` (dismissed) not reopened: the artifact makes no change to how `step.title` is projected, and the render is unchanged (`render --check` clean).
- Step 90's own content has not regressed. `git diff e47f4cf..b36c4c6 --numstat -- docs/plans/agent-scaffold.steps/decision-folder-currency.md` is `3 3`: exactly three lines changed, the `R3-2` reason inside the ACTOR-LESS bullet at `:7`, and the two step-91 references at `:20` and `:40`. Every other line of the sidecar is byte-identical to `e47f4cf`, including the second class definition, the four quoted passages, the four implementer instructions, the single-source constraint paragraph, the documentation-currency list, the regeneration instructions, the `pack/LEDGER.template.md:3` exclusion, and the no-receipt paragraph. The `:20` and `:40` rewrites are accurate against the new state: `:20` now says branch 3 "is held as the `exploring` queue item `Q-69` with a design pass owed and NO step yet", and `:40` correctly softens the fourth-deployed-asset claim to "Depending on which direction that pass lands on, it may also reach", which matches the withdrawn option set (only direction (ii) certainly reaches `.agents/user-prompts/explore.md`). The one miss in the same file is `R4-2`.
- No scope expansion. The commit touches four files and nothing outside `docs/plans/`: the plan source, the generated view, the step 90 sidecar, and the deleted step 91 sidecar. No pack file, no `src/`, no prompt.

## 7. Mechanical checks: PASS

Run in this worktree under the project toolchain:

```
$ cargo run --quiet -- render --check docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.plan.toml: up to date

$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl --workflow
docs/metrics/workflow.jsonl: 212 records, valid
docs/plans/agent-scaffold.plan.toml: 91 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

91 steps, 69 questions, 212 records, invariants hold, view current. Exactly the expected counts.

I also ran `cargo test`: all suites pass, including the `agents_md_drift` whole-file guard and the `isolation_policy` byte guard, confirming the artifact did not disturb the generated-asset correspondence.

The `order` gap at 91 is real (`grep -n "^order = 9[0-9]" docs/plans/agent-scaffold.plan.toml` gives `1243:order = 90` and `1256:order = 92`, no 91) and `validate` does not object, so per the brief I do not raise it. I checked the plan's Status line arithmetic against the new count independently: "91 steps (3 not started, 2 in progress, 60 complete, 4 skipped, 1 next, 3 optional, 18 deferred)" sums to 91, and the removed step was `not-started`, which is the bucket that went 4 -> 3.

---

## Severities I looked for and did not find

- `critical`: none. Nothing in the artifact is security-, data-, or money-sensitive, and no shipped asset changed.
- `high`: none. There is no false factual claim in the artifact. I re-verified every empirical claim in the rewritten `Q-69` paragraphs (the `b6ba317` commit contents and timestamp, the zero `Q-68` metrics records, the ledger attribution text and its commit time, the const's four-item count and its absence of the word "question", the `{{isolation_policy}}` slot line, and the `QuestionStatus::Exploring` variant) and every one reproduces. `NEW-1`, the round-2 `high`, was an assertion the project's own record contradicted; nothing of that class survives.
- `medium`: none. `DEC-1`, the round-3 `medium`, was that the option set never ruled on the fragment's exhaustiveness. It is not merely stated now, it is the pass's first named input (`plan.toml:1724`), with the three consequences the round-3 triager asked for, including the one it added itself (that under the exhaustive reading the edit-surface argument against direction (ii) runs backwards). The withdrawal of the option set removes the class of defect entirely, since there is no longer a recommendation resting on an unstated premise.
