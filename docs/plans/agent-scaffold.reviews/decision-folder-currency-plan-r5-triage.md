# Plan-review round 5 triage: step 90 (`decision-folder-currency`), step 92 (`prompt-drift-guard`), `Q-69`

Triager, independent of the planner that produced the artifact and of the orchestrator that drives the loop.

Worktree `.claude/worktrees/triage5-dfc`, detached at `72ca2d7` (the planner branch tip, which is the artifact). Artifact `git diff 7707df2..72ca2d7`; round-4 fix commit `git diff b5ddb52..72ca2d7`. Every citation below was re-read and every command re-run in this worktree.

Reviewers triaged:

- `decision-folder-currency-plan-r5-reviewer-residue.md`: one finding, `R5-1` (`low`).
- `decision-folder-currency-plan-r5-reviewer-executability.md`: zero findings, plus one candidate it examined and deliberately did not raise (the same tension `R5-1` reports).

## Verdict summary

| Finding | Reviewer severity | Verdict | Triager severity |
| --- | --- | --- | --- |
| `R5-1` | `low` | VALID BUT ACCEPT RESIDUAL | `low` (confirmed) |

**ROUND 5 IS CLEAN.** An accepted residual does not block convergence (`AGENTS.md`, Convergence rule: "A valid finding may instead be resolved by consciously accepting its residual risk and recording that; an accepted risk does not block convergence"). There are no new valid findings requiring a fix, so this round does NOT reach the `new_valid` branch and does NOT escalate at the total-round cap.

No backstop re-check is triggered: the backstop covers a DISMISSED finding at `high` or above (`AGENTS.md:51`, `:59`). `R5-1` is neither dismissed nor at or above `high`.

---

## `R5-1` (`low`): `:34`'s "copy the pointing" was not reconciled with the round-4 fix's new `:28`

**Verdict: VALID BUT ACCEPT RESIDUAL. Severity `low`, confirming the reviewer.**

### Evidence reproduced: YES, in full and exactly

I re-derived every element rather than accepting the reviewer's rendering of it.

`pack/AGENTS.md:71`, read in this worktree, closes with one sentence that divides at a colon:

> Here "updates this queue" means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job (routed as above): the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits, which author no reviewed product content, so a `[[question]]` or `[[step]]` (reviewed product content) is not among them.

The last colon falls after "(routed as above)". Everything before it is the MAIN clause; everything after it is the TRAILING RATIONALE clause. That is the split `decision-folder-currency.md:28` draws ("NOT its trailing rationale clause after the colon") and the split `plan.toml:1722` draws in the `Q-69` ask. The two are consistent.

The load-bearing identity claim reproduces mechanically. A fixed-string grep of the trailing clause returns exactly one hit in the pack and exactly one in the sidecar:

```
$ grep -c -F 'the generated isolation-policy fragment below lists the orchestrator'"'"'s closed set of direct-on-main edits, which author no reviewed product content, so a `[[question]]` or `[[step]]` (reviewed product content) is not among them' pack/AGENTS.md docs/plans/agent-scaffold.steps/decision-folder-currency.md
pack/AGENTS.md:1
docs/plans/agent-scaffold.steps/decision-folder-currency.md:1
```

The sidecar hit is at `:34`, inside the parenthetical that "copy the pointing" refers back to. So the reviewer's central claim is exact: the "pointing" quoted at `:34` IS `pack/AGENTS.md:71`'s trailing rationale clause, character for character, and it is the same clause `:28` forbids reproducing in the prompt's checkpoint paragraph.

The fix-induced framing also reproduces. `git diff b5ddb52..72ca2d7 -- docs/plans/agent-scaffold.steps/decision-folder-currency.md` shows the removed line said only "that the checkpoint paragraph ends up saying what `pack/AGENTS.md:71` says", with no main/trailing split. Under that wording `:34` and the requirement agreed. The round-4 fix narrowed the requirement and left `:34` untouched. The reviewer's account of the history is correct.

Two supporting claims inside `:28` also check out, which matters because they are the reason the requirement exists: `pack/prompts/orchestrator.md` has the checkpoint paragraph at `:27` and the Socratic three-branch sentence at `:31`, with `:29` between them, so "two paragraphs above the branch-3 sentence it contradicts" is accurate; and the trailing clause is indeed one side of the `Q-69` contradiction per `plan.toml:1722`.

I also bounded the footprint, which neither reviewer did. `grep -rn -F "copy the pointing"` and `grep -rn -F "the pointing"` across `docs/plans/`, `pack/` and `src/` return the sidecar `:34` and its projection at `agent-scaffold.md:1225`, and nothing else. No other instruction anywhere in the artifact asks for a pointing, so the tension is confined to one clause and its generated copy.

### Why it is nonetheless an acceptable residual and not a required fix

The reviewer is right about the text and right that the fix left a pair half-reconciled. Where I part from a required-fix reading is on what `:34` actually OBLIGES, and I reached this structurally rather than by deferring to either reviewer.

**1. `:34` imposes no obligation for `:28` to contradict.** Read the paragraph's instructions in order. "Where a passage benefits from the connection, CROSS-REFERENCE the fragment; do not copy from it" is conditional on the passage benefiting and prescribes a FORM. "Copy the pointing, not the list" is the same X-not-Y form contrast, and its subject is `pack/AGENTS.md:71` as a MODEL. Then the paragraph's closing sentence, which is the one that states the paragraph's actual requirements, is explicitly permissive on the reference and prohibitive only on the enumeration: "The prompt MAY reference the rule in `AGENTS.md` rather than reproduce the fragment's contents, and it MUST NOT enumerate the four direct-on-main edits itself." So `:34` lays exactly one hard requirement on the checkpoint paragraph, that it not enumerate, and grants one permission. `:28` narrows a permission. A specific carve-out narrowing a general permission is the ordinary relation between the two, not a contradiction, and `:28` is the more specific instruction (it names the paragraph and states its reason), so the standard resolution runs the right way even for a reader who notices both.

**2. The parenthetical at `:34` is evidentiary, not prescriptive.** It is cited to establish the premise that the model "ends by POINTING at the fragment", as against ending by listing it. That premise is true, and the quote being verbatim is what makes it check out. Reading a quotation offered as proof of a form as an instruction to transcribe a string is the reading the round-1 triager already rejected on `T-5`, and it is self-defeating in the target file anyway, since the quoted text says "the fragment BELOW" and `pack/prompts/orchestrator.md` has no fragment below.

**3. The operative instruction forecloses the failure mode independently.** `:19`, in the "What the implementer changes" section, prescribes the checkpoint edit in full and closes "match the guidance's existing clause, do not invent a different rule". An implementer works the edit from `:19`. That is a third instruction pointing the same way as `:28`, and it does not depend on how anyone reads `:34`.

**4. The harm ceiling is bounded and the protection that matters holds under every reading.** The worst outcome of a misreading is one extra clause of hand prose in `pack/prompts/orchestrator.md`, adding a fourth passage to what the `Q-69` design pass has to fix. Reversible, visible in the diff, in a paragraph that pass is already chartered to read, and checkable at work review against `:28`. Critically, NO reading of `:28` or `:34` or their interaction can produce a restatement of the generated four-item list: `:34`'s one hard requirement forbids it, `:28` forbids the whole clause, and `:19` prescribes prose that contains neither. So the protection that step 89's finding F1, `T-4`, and plan Principle 8's one-source-of-truth thinking actually exist for is untouched. That is what keeps this at `low`.

### One thing both reviewers missed, and why it does not change the verdict

Both framed the risk as binary: the implementer either reproduces the trailing clause or does not. There is a third path neither named. A reader who takes "copy the pointing" as an imperative and "the checkpoint paragraph is where this bites hardest" as singling that paragraph out for the connection, and who then meets `:28`, can try to satisfy both by writing a PARAPHRASED pointing: not the trailing clause verbatim, and not an enumeration. That threads between `:28`'s letter (which bars reproducing the clause) and `:34`'s prohibition (which bars the list), while still putting a fresh instance of the `Q-69`-contradicted proposition into `pack/prompts/orchestrator.md`, which is the harm `:28` states it is preventing. So the residual is marginally wider than `R5-1` frames it.

It does not change the verdict, because `:19` bars it directly: a paraphrased pointing is inventing a different rule, and `:19` closes by forbidding exactly that. It also lands in the same bounded harm ceiling as the reproduction case. I record it so the residual is accepted at its true width rather than at the narrower width the finding describes, and so the `Q-69` pass inherits an accurate note.

### On the executability reviewer's simulation, which I do not treat as decisive

The parallel reviewer derived the same tension independently and reported that it "did not actually mislead me in execution": working from `:19` and `:28` it produced the main clause with no pointing and no enumeration. That is real evidence and it is the only evidence in the round from an agent that actually simulated the edit.

I weigh it, but I decline to rest on it, and I say so plainly because it would be the convenient thing to lean on. The reviewer derived the tension WHILE simulating the passage, so by the time it wrote its edit it was no longer a naive reader. That makes it evidence that an attentive implementer resolves correctly, not evidence that an inattentive one would. What carries the verdict for me is point 1 above, which is structural and holds regardless of any reader's attentiveness: there is no obligation at `:34` for `:28` to contradict. The simulation is corroboration, not the foundation.

### Why not INVALID, and why not a required fix

Not INVALID. The tension is real, reproduces exactly, and was genuinely created by the round-4 fix narrowing one member of a pair. `:34` does single out the checkpoint paragraph by name and does use the verb "copy" with the trailing clause as its antecedent. Ruling this INVALID would misdescribe the artifact, and at round 5 it would be the convenient verdict wearing a stronger one's clothes. The reviewer's claim is accurate; the only question is whether accuracy at this magnitude compels a fix.

Not a required fix. The reasoning above is round-independent, and I have checked that against myself: I would rule the same way at round 2. At round 2 I would have added that the sidecar should be reconciled opportunistically if it were being edited for something else, which is how the round-1 triager handled `T-5`'s related looseness. Nothing else in this round requires editing the sidecar, so there is no opportunistic edit to attach it to, and manufacturing one would trade a certain cost (a sixth round, or a human escalation at the cap) against a bounded and owned residual.

### The accepted residual, recorded

`decision-folder-currency.md:34`'s "copy the pointing, not the list" clause, and its projection at `agent-scaffold.md:1225`, are not reconciled with the requirement added at `:28`. A reader who takes that clause as an imperative rather than as a form contrast could add a pointing clause, verbatim or paraphrased, to `pack/prompts/orchestrator.md`'s checkpoint paragraph, adding a fourth passage to the `Q-69` design pass's scope. It cannot produce a restatement of the generated list under any reading.

Owner and closure: the `Q-69` design pass, which already owns `pack/AGENTS.md:71`'s two halves and the three shipped passages that turn on them. When that pass runs, or whenever `decision-folder-currency.md` is next opened for any reason, reconcile `:34` with `:28` in one clause, along the lines the reviewer proposes (note that for the checkpoint paragraph the pointing is itself out of scope per the requirement above, so the form instruction bites only where a passage does carry the connection). If the step is implemented before that, the work review should check the checkpoint paragraph against `:28` specifically, including for a paraphrased pointing.

### The `T-5` re-raise question, ruled explicitly

**`R5-1` is NOT a re-raise of `T-5`, and the ledger rule does not bar it.** I ruled this on the primary documents rather than on the reviewer's argument for it.

- `T-5`'s claim (round-1 triage, `decision-folder-currency-plan-triage.md:123-131`) was that "copy the pointing, not the list" is NON-EXECUTABLE in a prompt that has no fragment. Verdict INVALID, on the ground that the instruction names the FORM rather than a string to transcribe, and that the following sentence resolves it. That verdict stands. I found no evidence against it and `R5-1` brings none; indeed `R5-1` relies on the same form reading in its own severity argument.
- `R5-1`'s claim is different in kind: that `:34` conflicts with `:28`. `:28` was CREATED by `72ca2d7`, the round-4 fix, and did not exist when `T-5` was ruled. A claim about a conflict with text that post-dates a settled verdict cannot be a re-raise of that verdict, because it does not assert the verdict was wrong. `R5-1` expressly accepts it.
- Nor is it a re-raise of the round-4 triager's related note (`decision-folder-currency-plan-r4-triage.md:198`). That note was a deliberate NON-raise, not a settled finding carrying a verdict, and it is the note that CAUSED `:28` to be written. It could not have addressed a conflict with text it was itself requesting.

The ledger rule forbids re-raising a settled finding without new evidence that its verdict was wrong. This is a distinct case again: new text creating a new claim, which is neither a re-raise nor an appeal. The reviewer was right to raise it rather than suppress it, and the executability reviewer was also acting correctly in recording its examination of the same tension rather than staying silent. Both handled the ledger rule properly.

---

## Spot-checks of the residue reviewer's PASS verdicts

Its PASS verdicts are load-bearing for convergence, so I re-ran them rather than accepting them. All hold.

**All four step-90 instructions scope-covered, none bare. CONFIRMED.** Read `decision-folder-currency.md:19` to `:22` against `:26`. `:19` (checkpoint) carries the scope INLINE ("not authoring the decided decision's `[[question]]` or `[[step]]` fold"). `:20` (Socratic) carries it INLINE ("the planner authors the non-trivial fold"). `:21` (prompt ledger) carries it BY REFERENCE ("under the same scope as the other three (see SCOPE below)"). `:22` (guidance ledger) carries it BY REFERENCE ("and under that same scope"). `:26` opens "SCOPE, governing all four instructions above", which anchors both references and closes the loop for `:22`, whose only antecedent is `:21`. No instruction is left bare.

**The `:24` / `:28` main-clause quotation is verbatim. CONFIRMED.**

```
$ grep -c -F 'Here "updates this queue" means raising and pushing the open items, not authoring a decided decision'"'"'s `[[question]]` or `[[step]]` fold, which is the planner'"'"'s job' pack/AGENTS.md
1
$ grep -c -F "Here 'updates this queue' means raising and pushing the open items, not authoring a decided decision's \`[[question]]\` or \`[[step]]\` fold, which is the planner's job" docs/plans/agent-scaffold.steps/decision-folder-currency.md docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.steps/decision-folder-currency.md:2
docs/plans/agent-scaffold.plan.toml:1
```

The quoted span matches the shipped line exactly; the only difference is the standard nesting convention (the pack's double quotes around `updates this queue` become single quotes inside the sidecar's double-quoted span). Two hits in the sidecar, at `:12` and `:28`, both correct. I agree with the reviewer's decision not to raise the elision of "(routed as above)": that parenthetical is inside the main clause, but `:19` carries the routing element explicitly ("which is the planner's job and which the orchestrator routes"), so nothing operative is lost.

**`prompt-drift-guard.md` changed only its two currency sentences. CONFIRMED.** `git diff b5ddb52..72ca2d7 --numstat` reports `2 2` for that file, and the diff shows the two changed lines are `:23` (removing the false "steps 90 and 91") and `:25` (retitled "Interaction with step 90", replacing "the order given (90, then 91, then 92)" with "the order as it stands"). The operative independence claim survives verbatim, and no other content in the file moved.

**`T-4` and `R2-3` present and unchanged. CONFIRMED.** `T-4`'s four-item parenthetical is present once at `decision-folder-currency.md:34` ("a step's status flip, an increment declaration, a round record, and the ledger's resume anchor"), on a line the fix diff does not touch. `R2-3`'s "three actor-less" phrasing is at `plan.toml:1706`, in the `Q-67` ask; the fix commit changed exactly ONE line of `plan.toml` (`1722`, inserting a single space after a colon), so `Q-67` is untouched. Neither is re-raised by either reviewer, correctly.

**Mechanicals green. CONFIRMED, re-run in this worktree.**

```
$ cargo run --quiet -- render --check docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.plan.toml: up to date
exit: 0

$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 213 records, valid
docs/plans/agent-scaffold.plan.toml: 91 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

91 steps, 69 questions, 213 records, exactly as expected. The projection is current, which is why the `:28` and `:34` texts reproduce identically at `agent-scaffold.md:1219` and `:1225`.

**The `order` gap at 91.** Intentional per the brief; `validate` does not object, and `grep -n "^order = "` on `plan.toml` confirms `89`, `90`, `92` with no `91`. No live reference to a step 91 survives in the plan source, the sidecars, the question sidecars, or the generated view; the only hits are the deliberate past-tense provenance sentence at `plan.toml:1734` and its projection.

No PASS verdict failed, so this section raises no finding of its own.

## Settled items: not re-raised

`T-4`, `R2-3` and `H4-3` (accepted residuals) and `T-5` and `T-7` (dismissed) are all undisturbed by the round-4 fix and are not reopened here. I found no evidence against any of those verdicts. `Q-69`'s elided quotation of the generated fragment (operative clause plus item count, four items not reproduced) is deliberate and correct. Line length and formatter reflow are not findings.

## Round outcome

- Valid findings requiring a fix this round: NONE.
- Valid findings resolved by accepting the residual: ONE (`R5-1`, `low`).
- Dismissed findings: NONE. No backstop re-check is triggered.

**ROUND 5 OUTCOME: CLEAN.** The loop converges. It does not escalate at the total-round cap.
