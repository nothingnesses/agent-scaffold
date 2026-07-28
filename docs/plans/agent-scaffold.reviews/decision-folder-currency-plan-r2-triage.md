# Plan-review triage, step 90 `decision-folder-currency`, round 2

Triager worktree: `.claude/worktrees/triage2-dfc`, detached at `2364a83` (main plus both round-2 reviewer findings files). Artifact under review: `e3fca03..f8b3cdc` on `plan/decision-folder-currency`; the round-1 fix commit alone is `0905620..f8b3cdc`.

Reviewers triaged:

- `docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-r2-reviewer-verification.md` (`R2-1`, `R2-2`, `R2-3`).
- `docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-r2-reviewer-newcontent.md` (`NEW-1`..`NEW-4`).

Reproduction method. `f8b3cdc` is NOT an ancestor of this worktree's `HEAD`, so no artifact claim could be checked against the checked-out tree. The artifact tree was extracted with `git archive f8b3cdc` into a scratch directory and every artifact citation was read there; pack, `src/`, `justfile`, and ledger claims were read in the same extracted tree; history claims were checked with `git show` / `git log` in the worktree. Every testable claim in both findings files was re-run or re-read, per decision `Q-66` and the Triager role in `AGENTS.md`. Per-finding reproduction results are recorded below.

Outcome: 6 valid findings the planner must fix (1 high, 1 medium, 4 low), 1 accepted residual (low), 0 invalid, 0 out of scope. No finding at or above the backstop severity was dismissed, so no second-triager re-check is owed on this round.

Independent re-run of the mechanical checks against the extracted artifact tree (not taken from either reviewer's report):

```
$ cargo run --quiet -- render --check <art>/docs/plans/agent-scaffold.plan.toml
<art>/docs/plans/agent-scaffold.plan.toml: up to date

$ cargo run --quiet -- validate --source <art>/docs/plans/agent-scaffold.plan.toml --metrics <art>/docs/metrics/workflow.jsonl --workflow
<art>/docs/metrics/workflow.jsonl: 210 records, valid
<art>/docs/plans/agent-scaffold.plan.toml: 92 steps, 69 questions, valid
<art>/docs/plans/agent-scaffold.plan.toml vs <art>/docs/metrics/workflow.jsonl: workflow invariants hold
```

Round-1 ledger. `T-4` (accepted residual), `T-5` and `T-7` (dismissed) stay settled; no reviewer brought new evidence that any of those verdicts was wrong. `R2-3` touches the same prose class as `T-4` but is a different clause in a different field, so it is a new finding, not a re-raise. `NEW-1` does not re-open `T-3a`'s verdict (which was about the contradiction, and stands); it is a finding against `Q-69`, an artifact `T-3a` predates, and it carries evidence (`ledger.md:355`) that no round-1 reviewer or triager cited.

---

## NEW-1: the `Q-69` `ask`'s `b6ba317` evidence is an invalid inference and is contradicted by the project's own ledger

Verdict: VALID. Severity: `high` (confirming the reviewer's rating).

Evidence reproduced: YES, both halves, in full, plus corroboration the reviewer did not cite.

The claim under review, verbatim at `docs/plans/agent-scaffold.plan.toml:1733` (the `Q-69` `ask`; note the reviewer cites this line correctly):

> THE AMBIGUITY IS LIVE, not theoretical: commit `b6ba317` ... is a plain single-parent commit on main that added `[[question]] id = "Q-68"`, `status = "exploring"` straight into `docs/plans/agent-scaffold.plan.toml` with no planner branch and no review round.

Half (i), the commit-shape inference. Reproduced.

```
$ git log -1 --format='%h parents=[%p] %s' 557fa46
557fa46 parents=[e8f458c] docs: require reviewer findings to carry reproducible evidence (Q-66)
$ git log -1 --format='%h parents=[%p] %s' 4f48283
4f48283 parents=[dc9686a] docs: name the planner as folder of non-trivial decided decisions (Q-67)
$ git log -1 --format='%h parents=[%p] %s' cca1099
cca1099 parents=[44f848a] docs: apply Q-66/Q-67 plan-review round 1 fixes (F1 F2 F3 F-fid)
```

All three are single-parent. The ledger records all three as fast-forward merges of isolated worktree branches that each went through at least one full review round: "STEP 88 COMPLETE: ff-merged `557fa46`", "STEP 89 COMPLETE (ff `4f48283`)", "PLAN FOLD MERGED (ff `cca1099`)", all inside `docs/plans/agent-scaffold.ledger.md:355`. This repo's own worktree-lifecycle rule (`AGENTS.md:93`, `:95`) prescribes exactly that fast-forward integration and the deletion of the branch afterwards, and no branch survives for any of the three (`git for-each-ref` lists only `main`, `plan/decision-folder-currency`, `origin/main`, a stash, and `v0.0.1`). So single-parentness carries zero information about whether a planner branch was used. The inference in the `ask` does not follow from the evidence it cites.

Half (ii), the ledger's contemporaneous record. Reproduced, re-located by content rather than by any cited line number.

```
$ grep -n 'NEW BACKLOG' docs/plans/agent-scaffold.ledger.md
355:...
```

Line 355 reads, in relevant part: "NEW BACKLOG (captured 2026-07-26 by a planner, per Q-67): `Q-68` (`exploring`, DESIGN PASS OWED)". That text was written by `8d12264` ("docs: record Q-68 (structured-first ledger) backlog capture in ledger queue"), whose parent IS `b6ba317` and which landed three minutes later (`git log -1 --format='%h parents=[%p] %ad'`: `8d12264 parents=[b6ba317] 2026-07-26 22:40:29`, against `b6ba317 ... 22:37:46`). It is the contemporaneous record of the same event and it says a PLANNER captured `Q-68`, "per Q-67", meaning the actor rule was consciously applied rather than breached.

Corroboration neither reviewer cited: the "per Q-67" attribution is chronologically coherent. `Q-67` went live on main at `4f48283`, 2026-07-26 22:05:29, thirty-two minutes before `b6ba317` at 22:37:46. So the rule the ledger says was applied was in force at the time.

Why it is valid, and why the reviewer is right that this is not a nit. The `ask` asserts as fact something that (a) does not follow from the evidence offered, and (b) is denied by the only durable record that speaks to it. It is the only empirical claim in the item, it is presented under the heading "THE AMBIGUITY IS LIVE, not theoretical", and it is load-bearing twice more: option (b)'s trade-off ("would likely be honoured in the breach, exactly as `b6ba317` already shows") and the recommendation's closing sentence ("the evidence of `b6ba317` suggests the heavier rule would simply not be followed"). If a planner authored the placeholder, the one datum offered AGAINST option (b) becomes a datum FOR it: the heavier rule was in fact followed, at no observed cost. That does not merely weaken an argument, it inverts it.

The mechanism of the error is visible in the provenance, and is worth recording so it is not repeated. The round-1 triage carried the claim with an explicit epistemic hedge (`decision-folder-currency-plan-triage.md:76`: "I did not establish which role authored it, so I offer this as evidence that the ambiguity is live, not as a finding of a misstep"). The `Q-69` `ask` kept the no-blame half of that hedge ("Offered as evidence the boundary is unclear, not as a finding of a misstep against any role") and dropped the epistemic half, converting an admitted unknown into an asserted fact. That is the over-read.

What survives correction. Two things, and the fix must keep them. First, "no review round" IS verifiable and true: `grep -c 'Q-68' docs/metrics/workflow.jsonl` returns `0`, and the ledger records no round for it. Second, the underlying contradiction (`T-3a`) is independently established and does not depend on `b6ba317` at all. `Q-69` has a sound reason to exist without this datum.

Severity. `high`, upheld, and I considered `medium` seriously before settling. The round-1 calibration would suggest `medium` (T-1, a decision record misdescribing its own scope, was `medium` on the grounds that it is documentation, reversible, and contained to the plan). Three things push this above that line. First, it is not a description of past work but the evidentiary basis of a decision the human has not yet made, and it is used three times in the argument for the recommended option. Second, it is refuted by the project's own record, so leaving it standing means the plan of record asserts something the ledger denies; that is the exact failure mode `Q-66` (decided nine days ago, live and binding) exists to prevent, and it violates plan Principle 6 (Ground decisions in evidence). Third, and decisively, once the human decides, the decision is folded and thereafter "reopens only by evidence that beats its recorded reasoning" (`AGENTS.md:63`). A decision taken on refuted evidence acquires relitigation protection, which makes the defect much harder to reverse after the decision than before it. `high` is not `critical`: nothing is shipped yet, no code or data is at risk, and the item is `open` so the correction is free right now.

Backstop. This verdict is VALID, not a dismissal, so the high/critical dismissal re-check does NOT fire on it.

What the fix must achieve. In the `Q-69` `ask` at `plan.toml:1733`, and in the weaker restatement at `docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md:5`:

1. Drop the "plain single-parent commit" wording entirely from both places. It is inferentially inert in a repo that fast-forwards every converged branch, and stating it invites exactly the inference that failed here.
2. Drop the "no planner branch" assertion, or reconcile it explicitly against `docs/plans/agent-scaffold.ledger.md`'s "NEW BACKLOG (captured 2026-07-26 by a planner, per Q-67)" and state which record is authoritative.
3. Keep only what is verifiable: a `[[question]]` with `status = "exploring"` was added to `docs/plans/agent-scaffold.plan.toml` directly on main, and no review round exists for it (`grep -c 'Q-68' docs/metrics/workflow.jsonl` returns `0`).
4. Rewrite option (b)'s "honoured in the breach, exactly as `b6ba317` already shows" trade-off and the recommendation's closing sentence. Under the ledger's account, `b6ba317` is partial COMPLIANCE with option (b) (planner authored, no review round), not a breach of it, and the honest reading is that the planner half of (b)'s ceremony was paid without visible cost while the review-round half was not.
5. Cite the ledger passage by its quoted text, not by line number (see `R2-1`).

---

## NEW-2: `pack/AGENTS.md:71` is described as unqualified in four places, when its main clause carries the qualifier

Verdict: VALID. Severity: `medium` (confirming the reviewer's rating).

Evidence reproduced: YES, all four locations verbatim, and the full line read as the brief required.

`pack/AGENTS.md:71` read in full in the artifact tree:

> Checkpoints (the human-decision queue and progress). ... At every checkpoint the orchestrator updates this queue and pushes its open items to the human, each per the human-input contract, rather than waiting for the human to pull them: a new human would not know to watch it, and a pull-only model is fragile. Here "updates this queue" means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job (routed as above): the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits, which author no reviewed product content, so a `[[question]]` or `[[step]]` (reviewed product content) is not among them.

So the sentence splits at the colon: the main clause IS qualified ("a decided decision's `[[question]]` or `[[step]]` fold"), and the trailing rationale clause is not. The reviewer's reading of that split is correct.

The four artifact locations reproduce verbatim:

- `docs/plans/agent-scaffold.plan.toml:1733` (`Q-69` ask): "the actor boundary `Q-67` added at `pack/AGENTS.md:71` is stated without qualification".
- `docs/plans/agent-scaffold.plan.toml:1254` (step 91 `title`): "`pack/AGENTS.md:71` excludes a `[[question]]` from the orchestrator's direct-on-main edits without qualification".
- `docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md:5`: "`pack/AGENTS.md:71` says without qualification that ...".
- `docs/plans/agent-scaffold.steps/decision-folder-currency.md:40`: "`pack/AGENTS.md:71` says without qualification that a `[[question]]` or `[[step]]` is not among the orchestrator's direct-on-main edits".

The same-commit inconsistency reproduces: `docs/plans/agent-scaffold.steps/decision-folder-currency.md:12` quotes the qualifier correctly ("Here 'updates this queue' means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job"). One artifact in the commit knows the qualifier exists and four assert there is none.

Where I correct the reviewer, because it changes what the fix must say. The reviewer characterises the trailing clause as "a trailing rationale clause over-generalises past the subject of its own sentence", and concludes option (a) is "close to a clarification of a clause that already carries the intended scope". That understates the trailing clause. It is not loose restatement of the main clause; it is an independent derivation from the GENERATED `ISOLATION_POLICY_FRAGMENT`, whose closed list (status flip, increment declaration, round record, ledger resume anchor) genuinely contains no `[[question]]` authoring of any status. So the trailing clause is unqualified because the generated closed list is unqualified, and that is precisely why the contradiction with `pack/AGENTS.md:45` is real. The consequence for option (a) runs the other way from the reviewer's framing: narrowing the exclusion to a DECIDED question's fold cannot be done by rewording one sentence in isolation, because it has to be reconciled with a generated single source the same sidecars forbid restating (`decision-folder-currency.md:26`, `exploring-item-actor-boundary.md:15`). Either the `exploring` placeholder is argued to author no reviewed product content, which is option (c)'s move, or the guidance carries an exception to the fragment's own rationale.

Step 91's sidecar already gets this half right, which narrows the fix: `exploring-item-actor-boundary.md:11` says option (a) is "Narrow its closing clause", correctly localising the change to the trailing clause. Only the "without qualification" characterisations are wrong.

Why it is valid. `Q-69` is an undecided item put to the human under the human-input contract, which requires the trade-offs to be accurate. Misdescribing the baseline inflates the contradiction, makes option (a) read as adding a scope the guidance lacks rather than aligning two halves of one sentence against a generated fragment, and correspondingly distorts the cost of option (b) relative to the status quo. It also lands in a step `title`, which is durable plan data.

Severity `medium`, not `low`: it misstates the shipped text a live human decision is about, in four places including a `[[step]].title`, and the same commit contradicts it three lines from one instance. Not `high`: the contradiction it describes is genuine, all three options remain live, and the recommendation's principle reasoning does not depend on the mischaracterisation.

What the fix must achieve. Replace "without qualification" in all four places with a precise statement: `pack/AGENTS.md:71`'s main clause already scopes the "updates this queue" gloss to a DECIDED decision's fold; its trailing rationale clause, derived from the generated isolation-policy fragment's closed list, excludes any `[[question]]` or `[[step]]` regardless of status, and it is that clause which contradicts `pack/AGENTS.md:45`. Then re-check the (a) versus (b) versus (c) comparison against that accurate baseline, including the point above that option (a) must reconcile with the generated fragment rather than only reword a sentence.

---

## NEW-3: the `Q-69` recommendation never names Principle 8, whose content is option (c)'s whole argument

Verdict: VALID. Severity: `low` (confirming the reviewer's rating), with its reasoning corrected.

Evidence reproduced: YES, every citation.

- The plan has exactly eight `[[principle]]` entries (`grep -n '^\[\[principle\]\]'` returns 1747, 1752, 1757, 1762, 1767, 1772, 1777, 1782), and Principle 8 is "Structured data first, project for humans" at `plan.toml:1784` with its text at `:1785`.
- Principle 8's text ends verbatim: "when this conflicts with Principle 2 (minimal) or Principle 3 (safe on existing projects) at this stage, this wins, and it sharpens Principle 1 (cleaner long-term architecture) and Principle 16-equivalent one-source-of-truth thinking."
- The `Q-69` recommendation names Principles 1, 2, and 5 and no others; option (c) is argued on "adds no new rule and keeps ONE criterion, which is the strongest one-source-of-truth answer".
- The established-usage claim reproduces: the `Q-67` `ask` cites Principle 8 for prose single-sourcing ("aligned with plan Principle 8 ... and its one-source-of-truth thinking"), and `exploring-item-actor-boundary.md:15` does the same.
- The no-contract-violation concession is correct: `RECOMMENDATION_RULE_FRAGMENT` (`src/recommendation_rule.rs:34`) requires reasoning judged against the Principles by name, and three named principles satisfy it.

Where I correct the reviewer. The finding's framing, that Principle 8 is "the plan's declared tie-breaker over the Principle it leans on", overstates the mechanics. Principle 8's precedence clause governs conflicts with Principles 2 and 3. In the `Q-69` recommendation, Principle 2 is used to reject option (b), and Principle 8 would reject (b) too, so the precedence clause never bites there. The (a) versus (c) split turns on Principle 5 (read as its documentation analogue) and Principle 1, over which Principle 8 declares no precedence, only that it "sharpens" Principle 1. So naming Principle 8 would not mechanically flip anything.

Why it is nonetheless valid rather than a preference. Option (c)'s entire argument IS Principle 8's content, the recommendation engages that argument substantively ("arguably the more elegant statement of the same boundary, since it derives from the fragment instead of adding to it"), and it does so without naming the principle that owns the reasoning, in a project whose contract is specifically that reasoning is judged against the Principles BY NAME. That is a real gap in an argument put to a human, not a request for more thoroughness.

Severity `low`: it is an omission in the presentation of an undecided item, it changes no fact, and the human can see the argument even unlabelled.

What the fix must achieve. Name Principle 8 where option (c)'s one-source-of-truth argument is engaged, and say why (a) still wins over it. Do NOT assert that Principle 8's precedence clause decides the matter; it governs conflicts with Principles 2 and 3, and this split turns on Principle 5 and Principle 1.

---

## NEW-4: the option set's boundary is not stated, so the "keep it out of the plan entirely" alternative is invisible

Verdict: VALID. Severity: `low` (confirming the reviewer's rating).

Evidence reproduced: YES, every supporting citation.

- `pack/prompts/orchestrator.md:33` reproduces verbatim: "The ledger is separate from the plan: do not put individual findings in the plan's Open Questions section; only durable decisions, the ones that change the plan, fold into it."
- The both-homes claim reproduces: `b6ba317` wrote the `[[question]]` (`git show --stat b6ba317`: `agent-scaffold.md`, `agent-scaffold.plan.toml`, `Q-68.md`), and `8d12264` wrote the ledger entry (`git show --stat 8d12264`: `agent-scaffold.ledger.md` only).
- The lifetime asymmetry reproduces: `pack/AGENTS.md:69` contains "the ledger is deleted at task close", and `Q-68` at `plan.toml:1726` states "the ledger is per-task and DELETED at task close while `workflow.jsonl` is cross-task and NEVER rewritten".
- `pack/AGENTS.md:45`'s stated purpose reproduces: "keeps a not-yet-decidable question visible and distinct from one merely awaiting a choice".

Why it is valid rather than a preference. All three options share an unstated premise (that the `exploring` placeholder is a plan `[[question]]`), and the item never says the premise is a premise. The human-input contract puts the option set before the human as the design space; when a plausible alternative is excluded, the boundary is part of the advice.

Evidence I add that the reviewer did not have, which strengthens the exclusion while confirming the gap. `exploring` is a typed variant of `QuestionStatus` in the structured source (`src/plan/source.rs:337`, `:363`), and `pack/AGENTS.md:65` (Design explorations) requires that "The Open-Questions item points at the exploration by path while it is `exploring`". So the proposed fourth option would orphan a schema variant and break the mechanism by which an exploration is referenced, which is a schema and entry-mode change well beyond what `Q-69` asks and arguably belongs with `Q-68`. That is a good reason to exclude it, and precisely the sentence that is missing.

Severity `low`: an undecided item, no fact is wrong, and the cure is one sentence.

What the fix must achieve. One sentence in the `ask` stating that the option set takes as given that the placeholder is a plan `[[question]]`, and why: `exploring` is a typed `[[question]]` status in the structured source and `pack/AGENTS.md:65` requires the plan-side item to point at the exploration while it is `exploring`, so relocating the placeholder to the ledger is a schema and entry-mode change outside this question. Writing the full trade-offs of a fourth option is not required.

---

## R2-1: two `ledger.md:345` citations in the sidecar point at the wrong line

Verdict: VALID. Severity: `low` (confirming the reviewer's rating). The reviewer's cause analysis is also correct, and the orchestrator's working hypothesis about the cause is not.

Evidence reproduced: YES, in full, including the provenance analysis, which I re-derived independently.

- The note is at line 355 in the artifact tree, not 345: `grep -n 'silent on the actor' docs/plans/agent-scaffold.ledger.md` returns `355`, and line 345 is the round-2 "BLOCKED HERE (2026-07-27)" resume block, which contains neither the quote nor the attribution.
- Four instances of the wrong number, two in source and two in the generated view: `docs/plans/agent-scaffold.steps/decision-folder-currency.md:7` and `:14`, `docs/plans/agent-scaffold.md:1197` and `:1204`. No instance of `ledger.md:355` anywhere in the artifact.
- Wrong at the moment written, not broken afterwards. The passage's line number across revisions (`git show <rev>:docs/plans/agent-scaffold.ledger.md | grep -n 'silent on the actor'`): `caeee2b` -> 345 (466-line file), `e3fca03` -> 355 (476-line file), `0905620` -> 355, `f8b3cdc` -> 355, `2364a83` -> 355. The orchestrator's ten-line anchor block landed in `e3fca03`, and `e3fca03` is the PARENT of the artifact branch's first commit (`git log -1 --format='%h parent=%p' 9547004` -> `9547004 parent=e3fca03`). So the shift happened before the fix pass began, not after it.
- Added by the fix commit, not inherited: `git show 0905620:...decision-folder-currency.md | grep -c 'ledger.md:345'` -> `0`; the same grep at `f8b3cdc` -> `2`; `git diff 0905620..f8b3cdc -- <sidecar> | grep -c '^+.*ledger.md:345'` -> `2`.

On the cause, stated accurately because the brief asked. The number 345 was correct against `caeee2b`, the tree the round-1 triage was written in (`decision-folder-currency-plan-triage.md:53`, `:55`, `:61` all cite `ledger.md:345`). The fix pass copied it from that triage file into a tree where the correct number was already 355. So this is a copied-without-re-reading citation, the same defect class the round-1 triage recorded against `FID-2` (which cited 337 when the text was at 345), and NOT a citation broken by a later edit. Framing the fix as "a stale number that drifted" would be wrong.

Severity `low`, confirmed: the quote and the attribution are both correct, so a reader who greps the quoted text finds the note, and the cost is a wasted lookup. It is the third misnumbered ledger citation in this review chain, which is why it is worth closing rather than absorbing.

What the fix must achieve. Both citations must resolve. Preferred form: cite the note by its quoted text and drop the line number, since the ledger is append-heavy and the same sidecar already hedges its pack citations ("find them by paragraph, not by line number, if they have since moved") while giving the ledger no such hedge. If a number is kept, it must be `355` and must be re-read in the tree being edited. Re-render so `docs/plans/agent-scaffold.md:1197` and `:1204` carry the correction.

---

## R2-2: "two more points of the same kind" contradicts the two-class split three lines later

Verdict: VALID. Severity: `low` (confirming the reviewer's rating).

Evidence reproduced: YES.

- `docs/plans/agent-scaffold.steps/decision-folder-currency.md:3` reads verbatim: "Its review round recorded an accepted-residual follow-up, and reads while scheduling that follow-up found two more points of the same kind: FOUR passages are still out of step with the rule".
- `:5` reads "The four fall into TWO CLASSES, and the operation each needs is different, so do not treat them alike", and `:8` puts `:27` and `:31` in the opposite class from the residual's two, adding "These two were never covered by the step-89 residual".
- The residual's two are confirmed as `pack/prompts/orchestrator.md:33` and `pack/AGENTS.md:63` by the ledger note itself (at line 355, per `R2-1`).
- The fix commit did rewrite the second half of that sentence and leave the first: `git diff 0905620..f8b3cdc` shows "still leave the actor unnamed" replaced by "are still out of step with the rule" with "of the same kind" untouched.

Assessment. There is a reading on which the sentence survives: the colon glosses "the same kind" as "out of step with the rule", which is true of all four. But that reading is undercut by the same paragraph, because the class-split exists specifically to say the four are NOT alike, and `:8` says of the two new ones that they "were never covered by the step-89 residual", which is the residual the "same kind" phrase compares them to. In a sidecar whose entire subject is a rule stated one way in one place and another elsewhere, the first sentence a reader meets should not assert the sameness the next paragraph is added to deny.

Severity `low`, confirmed: the class-split immediately follows and is unambiguous, and the per-passage bullets at `:19-22` carry the right operation, so an implementer who reads on is not misled. Not accepted as a residual, because unlike `R2-3` this text is new in the fix commit, it is the first sentence of an implementer-facing instruction, and the fix is a three-word deletion with no sibling artifact to keep consistent with.

What the fix must achieve. Line 3 must stop asserting the two new passages are of the same kind as the residual's two. Deleting "of the same kind" is sufficient; "of a related kind, but not the same class (see below)" also works.

---

## R2-3: "the three actor-less `pack/AGENTS.md` prose points" is imprecise for pre-edit line 71

Verdict: VALID BUT ACCEPT RESIDUAL. Severity: `low` (confirming the reviewer's rating). Not blocking convergence.

Evidence reproduced: SUBSTANCE YES, CITATION MISNUMBERED.

- MISNUMBERED: the finding cites `docs/plans/agent-scaffold.plan.toml:1716`. That line is `status = "decided"`. The `Q-67` `ask` is at `plan.toml:1719` (`grep -n 'id = "Q-67"'` -> 1715). The quoted clause "the FIRST pass (step 89) restates none of that list and edits only the three actor-less `pack/AGENTS.md` prose points" is verbatim at `:1719`. The claim reproduces at a different line; the citation does not. See the observations section, since the sibling reviewer makes the same slip.
- Substance reproduces. `git show 4f48283 -- pack/AGENTS.md` has exactly two hunks (`@@ -38,9 +38,9 @@` and `@@ -68,7 +68,7 @@`), and the removed line 71 is "Checkpoints (the human-decision queue and progress). ... At every checkpoint the orchestrator updates this queue and pushes its open items to the human, each per the human-input contract, rather than waiting for the human to pull them: a new human would not know to watch it, and a pull-only model is fragile." It names the orchestrator.
- The internal tension reproduces: `decision-folder-currency.md:8`, added by the same commit, uses that very passage as the reference point for the opposite class ("`:27`'s unqualified 'update the plan's Open Questions queue' is the exact verb `pack/AGENTS.md:71` needed an added sentence to qualify").
- The counter-argument the reviewer supplied also reproduces: `docs/plans/agent-scaffold.steps/planner-folds-decisions.md:11`, in the converged step-89 sidecar, says "it only names the planner as the folder of decided entries at the three prose points (lines 41, 43, 71) where the actor was previously unnamed".

Strengthening the reviewer did not find, recorded for completeness: the inconsistency is inside the `Q-67` `ask` itself, not only across files. The same string opens by describing the checkpoint rule as "the checkpoint rule ('the orchestrator updates this queue') blurs the boundary", which names the orchestrator, and later calls all three points "actor-less".

Why the residual is right to accept rather than fix here. Four things bound it, and one of them makes an isolated fix actively worse.

1. The natural contextual reading is true. In a decision about naming the FOLDING actor, "actor-less" reads as "the folding actor was unstated", which is true of pre-edit line 71: it did not mention folding at all. The converged step-89 sidecar makes that reading explicit with "where the actor was previously unnamed".
2. It describes completed work in a settled record. No implementer instruction depends on it, and nothing downstream reads it.
3. Fixing this one instance makes the corpus MORE inconsistent, not less: `planner-folds-decisions.md:11` carries the same characterisation in a converged artifact, so a one-field correction leaves the two records disagreeing about the same three lines. This is the same reasoning the round-1 triage used to accept `T-4`, and it applies with more force here because the cost of leaving it is lower.
4. The durable cure is the plan-prose sweep `T-4` already named, not a step-90 edit.

Not a re-raise of `T-4`: different clause, different field, new evidence (`git show 4f48283`).

Residual accepted, with one opportunistic note. If the planner is editing the `Q-67` `ask` for any other reason, changing "the three actor-less `pack/AGENTS.md` prose points" to "the three `pack/AGENTS.md` prose points where the folding actor was unstated" costs nothing and is true of all three; but doing so should ideally carry the same change to `planner-folds-decisions.md:11` so the two records keep agreeing. Neither is required and neither blocks convergence.

---

## Deduplication

No two findings are the same underlying issue. Two pairs are close enough to state explicitly:

- `NEW-2` and `R2-3` both concern how the artifact characterises `pack/AGENTS.md:71`, but they are distinct claims about distinct text: `NEW-2` is about the POST-step-89 line being called unqualified, and it affects a live undecided human decision; `R2-3` is about the PRE-step-89 line being called actor-less, and it describes completed work. Different sentences, different files in three of four instances, different verdicts.
- `R2-2` and `R2-3` both arise from the two-class split introduced by the `T-2` fix being inconsistent with prose the same commit left standing, but they hit different sentences in different files and have different fixes.

The four `NEW-*` findings all land on one edit surface (the `Q-69` `ask` at `plan.toml:1733`, plus step 91's `title` and sidecar for `NEW-2`) and can be fixed in one pass, but they are four distinct defects, not one.

---

## Observations while reproducing, that both reviewers missed

Recorded as observations, not as findings against the artifact.

1. BOTH round-2 reviewers misnumber the `Q-67` `ask`. The verification reviewer cites `docs/plans/agent-scaffold.plan.toml:1716` in its `T-1` verification section and again in `R2-3`; the ask is at `:1719`, and `:1716` is `status = "decided"`. This is the same defect class as `R2-1`, in the files that raise it. Every other line citation I checked in both findings files resolved correctly, including the more load-bearing `plan.toml:1733`, `:1254`, `:1784-1785`, `ledger.md:355`, `pack/AGENTS.md:45/63/69/71`, `pack/prompts/orchestrator.md:27/31/33`, and `planner-folds-decisions.md:11`. The orchestrator should not propagate `:1716` into the ledger or the fix brief.

2. `exploring-item-actor-boundary.md:11` already states option (a) correctly as "Narrow its closing clause", which is the accurate localisation `NEW-2` asks for elsewhere. The `NEW-2` fix is therefore narrower than the finding's location list suggests: the four "without qualification" characterisations are wrong, but step 91's own option-(a) instruction is right and should not be changed to match them.

3. The `Q-69` `ask` is internally inconsistent about `pack/AGENTS.md:71` in two independent ways at once: it calls the post-step-89 line unqualified (`NEW-2`) while the parent `Q-67` `ask` in the same file calls the pre-step-89 line both "actor-less" and "the orchestrator updates this queue" (`R2-3`). Both are symptoms of the same habit, describing that passage from memory of an earlier characterisation rather than from the line. A single re-read of `pack/AGENTS.md:71` while fixing `NEW-2` would close all of them.

---

## Disposition

Must fix before this artifact can be re-reviewed:

| id | reviewer | verdict | severity | evidence reproduced |
| --- | --- | --- | --- | --- |
| `NEW-1` | newcontent | VALID | `high` | YES, both halves, plus corroboration |
| `NEW-2` | newcontent | VALID | `medium` | YES |
| `NEW-3` | newcontent | VALID | `low` | YES, reasoning corrected |
| `NEW-4` | newcontent | VALID | `low` | YES, plus evidence added |
| `R2-1` | verification | VALID | `low` | YES, including provenance |
| `R2-2` | verification | VALID | `low` | YES |

Not blocking:

| id | reviewer | verdict | severity | evidence reproduced |
| --- | --- | --- | --- | --- |
| `R2-3` | verification | VALID BUT ACCEPT RESIDUAL | `low` | substance YES, citation misnumbered (`:1719`, not `:1716`) |

Backstop. No finding at or above the backstop severity was dismissed. `NEW-1`, the round's only `high`, is UPHELD as valid at `high`, so the second-triager re-check does NOT fire and convergence is not blocked on it. No dismissals of any severity occurred this round.

Round outcome: NEW VALID FINDINGS. The consecutive-clean streak resets to zero.

Files the fixes touch: `docs/plans/agent-scaffold.plan.toml` (the `Q-69` `ask` at `:1733` for `NEW-1`, `NEW-2`, `NEW-3`, `NEW-4`; the step 91 `title` at `:1254` for `NEW-2`), `docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md` (`:5` for `NEW-1` and `NEW-2`), `docs/plans/agent-scaffold.steps/decision-folder-currency.md` (`:3` for `R2-2`, `:7` and `:14` for `R2-1`, `:40` for `NEW-2`), and a re-render of `docs/plans/agent-scaffold.md`.
