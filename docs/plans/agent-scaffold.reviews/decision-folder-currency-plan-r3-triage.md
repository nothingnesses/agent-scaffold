# Plan-review round 3 triage, step 90 `decision-folder-currency`

Triager: independent of both the planner that authored the fold and the orchestrator that drives the loop. Triage worktree `.claude/worktrees/triage3-dfc`, detached at `3c5f1eb` (main plus both round-3 reviewers' findings).

Artifact under review: `e66d11a..981d9f5`; the round-2 fix commit alone is `09ef94e..981d9f5`.

## Note on where the artifact lives, and how I read it

`981d9f5` is NOT an ancestor of my worktree HEAD (`git merge-base --is-ancestor 981d9f5 HEAD` fails). The fold lives only on the planner branch, so the plan source and sidecars in my checkout are the PRE-fold versions and every reviewer line number would have resolved to the wrong text if read from my tree. I extracted the artifact blobs at `981d9f5` (`git archive 981d9f5 docs | tar -x`) and read every cited line from those. `pack/AGENTS.md`, `src/isolation_policy.rs`, `AGENTS.md`, and `.agents/AGENTS.reference.md` are byte-identical between `981d9f5` and my HEAD (`git diff --stat 981d9f5 HEAD -- <those>` is empty), so those citations were read directly in my tree.

This matters for the record: an orchestrator that hands a triager a worktree at main while the artifact sits on an unmerged branch is handing it the wrong text. It did not cost anything here because the mismatch was loud, but it is the fourth line-number hazard this loop has produced.

## Verdict summary

| id | reviewer | reviewer severity | my verdict | my severity | evidence reproduced |
| --- | --- | --- | --- | --- | --- |
| `DEC-1` | decision | `medium` | VALID | `medium` (confirmed, top of band) | YES, every citation and every command |
| `DEC-2` | decision | `low` | VALID | `low` (confirmed) | YES |
| `DEC-3` | decision | `low` | VALID | `low` (confirmed) | YES |
| `R3-1` | verification | `low` | VALID | `low` (confirmed), one sub-claim corrected | YES for the main claim, PARTLY for one sub-claim |
| `R3-2` | verification | `low` | VALID | `low` (confirmed) | YES, including the `--numstat` demonstration |

Five valid findings. No dismissals, so no finding at or above the backstop severity was dismissed and NO second-triager re-check is owed on this round. No new accepted residuals and no new out-of-scope items.

Round outcome: `new_valid`, 5 valid findings, severities `["medium", "low", "low", "low", "low"]`. The consecutive-clean streak stays at 0.

## Independent mechanical re-verification (run against the artifact tree, not taken from either reviewer)

```
$ ./target/debug/agent-scaffold render --check <artifact-tree>/docs/plans/agent-scaffold.plan.toml
<...>/agent-scaffold.plan.toml: up to date

$ ./target/debug/agent-scaffold validate --source <...>/agent-scaffold.plan.toml --metrics <...>/workflow.jsonl --workflow
<...>/workflow.jsonl: 211 records, valid
<...>/agent-scaffold.plan.toml: 92 steps, 69 questions, valid
<...>/agent-scaffold.plan.toml vs <...>/workflow.jsonl: workflow invariants hold
```

The verification reviewer's counts (92 steps, 69 questions, 211 records, invariants hold, view current) all reproduce.

Full citation sweep, because this loop has already produced three misnumbered-citation defects. I extracted every distinct `file:line` citation in the artifact (the `Q-69` `ask`, the step 90, 91, and 92 sidecars, and the step 90 and 91 `title`s) and opened each one: `pack/AGENTS.md` `:41`, `:43`, `:45`, `:63`, `:65`, `:71`, `:79`, `:91`, `:108`; `pack/prompts/orchestrator.md` `:27`, `:31`, `:33`; `pack/user-prompts/explore.md` `:3`, `:7`, `:13`; `pack/LEDGER.template.md:3`; `pack/pack.toml:166-167`; `src/manifest.rs:615`; `justfile:46-48`. ALL SIXTEEN RESOLVE, and the quoted fragments at `pack/AGENTS.md:63`, `:65`, `pack/prompts/orchestrator.md:31`, `:33`, `pack/LEDGER.template.md:3`, and `pack/user-prompts/explore.md:13` are verbatim. There is no fourth misnumbered citation in the artifact.

The two reviewers' own citations into the generated view also resolve: `docs/plans/agent-scaffold.md` `:194`, `:196`, `:204`, `:212`, `:214` are the lines they say they are.

---

## `DEC-1` (`medium`): VALID, severity confirmed at the top of the band

Evidence reproduced: YES, in full.

- `src/isolation_policy.rs:33` is the `ISOLATION_POLICY_FRAGMENT` const, and its closing sentence is verbatim what the reviewer quoted: "The only edits made directly on main are the orchestrator's own integration-level ones, which author no reviewed product content and so stay the orchestrator's direct job rather than a spawned agent's: flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor."
- `AGENTS.md:91` carries that sentence verbatim. `grep -n "isolation_policy" pack/AGENTS.md` returns exactly `91:{{isolation_policy}}`.
- The three "closed" assertions are where the reviewer says: two at `plan.toml:1733` ("the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits", quoting `pack/AGENTS.md:71`; and "whose closed list of integration edits contains no `[[question]]` authoring of any status"), one at `:1741` ("so (a) has to supply a REASON the fragment's closed list does not reach an `exploring` placeholder").
- Option (c)'s pitch at `:1745` is verbatim "it is the only option that resolves the contradiction by reading the generated fragment rather than by carving an exception around it".
- The step 91 half reproduces exactly: `grep -c "isolation_policy" exploring-item-actor-boundary.md` returns `1`, at `:15`, and that single occurrence is the do-NOT constraint. `:11` gives option (a)'s regeneration set as "`AGENTS.md` and `.agents/AGENTS.reference.md`"; `:13` gives option (c)'s as "as (a)". `src/isolation_policy.rs` is named nowhere as an edit target, in either the sidecar or the `ask`.

Why it is valid, stated more sharply than the reviewer stated it. The defect is not only an omission; there is an internal inconsistency inside the `ask` itself. The item asserts the fragment's list is CLOSED, and then at `:1741` says option (a) can be rescued by supplying "a REASON the fragment's closed list does not reach an `exploring` placeholder". Those two cannot both stand. If the list is closed, no reason makes it not reach an unlisted edit; you have to amend the list. If a reason can make it not reach the placeholder, the list is illustrative of the "authors no reviewed product content" criterion and calling it closed is wrong. The item never rules, and the ruling is what determines whether options (a) and (c) require an amendment to a generated, drift-guarded const that ships into every scaffolded project's `AGENTS.md` and `.agents/AGENTS.reference.md`.

One correction to the reviewer's framing. The reviewer writes that option (a) would reintroduce "the same class of two-passages-disagree defect that `Q-69` exists to remove". That is slightly too strong: the disagreement between the fragment and the three call sites already exists in the shipped pack today, and is precisely the contradiction `Q-69` was raised to resolve. What option (a) does is not create it but RELOCATE it, from an implicit conflict (three call sites versus a generated enumeration) to an explicit adjacent one (`pack/AGENTS.md:71` prose permitting an edit that `pack/AGENTS.md:91` twenty lines below says is not among "the only edits made directly on main"). An option offered as the resolution that leaves an equally direct contradiction in the same file has not resolved anything, so the finding's force survives the correction intact.

One point neither reviewer drew, which I add because it changes what the fix must cover. The exhaustiveness ruling does not only move the (a)-versus-(c) comparison, as the reviewer says; it can invert one of the two grounds the RECOMMENDATION uses against option (b). The recommendation's stated grounds at `:1747` and `:1751` are proportionality (Principle 2, "Minimal by default") and edit surface ("it is the only option that widens the shipped edit surface to a fourth deployed asset"). Under the exhaustive reading, (b) becomes the ONLY option that needs no change to a generated source: (a) and (c) would each have to amend `src/isolation_policy.rs` and regenerate, and a Rust const that ships into every scaffolded project is arguably a wider blast radius than a fourth deployed prose asset. So the edit-surface argument against (b) may run backwards. The proportionality ground is untouched, so the recommendation may well survive; but a human weighing (a) against (b) is currently weighing an argument that could invert under a ruling they have not been asked to make.

Not a re-raise of `NEW-2`. I confirm this, on stronger ground than the reviewer offered. `NEW-2` was about four places describing `pack/AGENTS.md:71` as "without qualification"; all four are fixed and the verification reviewer's closure of `NEW-2` is sound. More importantly, the round-2 triage's own "what the fix must achieve" (`decision-folder-currency-plan-r2-triage.md:113`) required the planner to "re-check the (a) versus (b) versus (c) comparison against that accurate baseline, including the point above that option (a) must reconcile with the generated fragment rather than only reword a sentence", and set out a binary at `:105`: "Either the `exploring` placeholder is argued to author no reviewed product content, which is option (c)'s move, or the guidance carries an exception to the fragment's own rationale." The planner took the first horn. `DEC-1` is the finding that the first horn does not close, because it answers the fragment's rationale clause and not its enumeration. That is a fix-completeness finding against the round-2 verdict's own stated requirement, raised on text the round-2 fix commit authored. It contests nothing that was settled.

Severity: `medium`, confirming the reviewer, and I record that it sits at the top of the band rather than in the middle. Not `high`, and the line I draw is this: round 2's `NEW-1` was rated `high` because the item asserted something the project's own durable record CONTRADICTED and deployed it as an argument. `DEC-1` contains no false claim. It is an unstated premise and an incomplete edit list in an option set that is still `open`, all three options remain live, the primary ground of the recommendation is untouched, and the item already puts the human on notice that (a) carries a fragment-reconciliation cost. Not `low`, obviously: it is the disclosed cost and edit surface of the recommended option in a live human decision that gains relitigation protection the moment it is folded.

What the fix must achieve (broader than the reviewer's "cheapest fix"):

1. Rule, in the `ask`, on whether the fragment's four-item list is EXHAUSTIVE or ILLUSTRATIVE of its "authors no reviewed product content" criterion, and stop using "closed" and "a reason it does not reach" in the same item.
2. If exhaustive: add "amend `src/isolation_policy.rs` and regenerate" to options (a) and (c) in `exploring-item-actor-boundary.md:11` and `:13`, and say in the `ask` that (a) and (c) reach a generated source while (b) does not.
3. Re-check the edit-surface comparison against (b) at `:1743` and `:1751` under whichever reading is chosen, because under the exhaustive reading the "fourth deployed asset" point stops favouring (a) and (c).

If the planner instead concludes the list is illustrative, that is an acceptable answer, but it must then be stated, and the three "closed" assertions must go, including the one that quotes `pack/AGENTS.md:71`'s own "closed set" wording (which would itself become a thing option (c) has to fix).

---

## `DEC-2` (`low`): VALID, severity confirmed

Evidence reproduced: YES. All three passages read verbatim at `plan.toml:1733` and `:1747`, and `grep -c "the wider reading" plan.toml` returns `1` (both uses are on the same long line, as the reviewer said).

I read the whole paragraph, as asked, to decide whether both usages can be true. They cannot, under one consistent referent:

- `:1733` opening: "three shipped passages follow the wider reading".
- `:1733` later: "So the wider reading has a real source, and it is the trailing clause, not the main one, that collides with the call sites."
- `:1747`: "bending three call sites to its wider half".

The second and third fix the referent as the trailing clause (the wide EXCLUSION), and both say the call sites do not currently follow it, one by "collides with", one by requiring option (b) to bend them to it. The first says the call sites follow it. The only way to make the opening sentence true is to read "wider" in the opposite sense there, as the wider reading of the ORCHESTRATOR'S LATITUDE rather than of the exclusion, which is a second referent for an undefined comparative in the same paragraph. So the reviewer's claim holds on either horn: either the opening sentence is wrong, or the term carries two opposite senses.

Severity `low` is right and I confirm it. The body corrects the direction within the same paragraph and no argument in the item depends on the opening clause. It is worth fixing rather than accepting because it is the item's framing sentence, it renders verbatim as the opening of the queue bullet at `docs/plans/agent-scaffold.md:194`, and the fix is one clause in a sentence that will be edited anyway for `DEC-1`. The reviewer's suggestion to model it on step 91's `title` (`plan.toml:1254`), which states the conflict unambiguously, is sound.

---

## `DEC-3` (`low`): VALID, severity confirmed

Evidence reproduced: YES. `plan.toml:1735` reads verbatim "Read correctly, the episode is PARTIAL COMPLIANCE with option (b), a planner authored the placeholder and the review round was skipped, NOT a breach of it." `plan.toml:1743` reads verbatim "but the review-round half was not paid, and under (b) as written a planner-authored `[[question]]` is reviewed product content, so that round is not optional."

The two are inconsistent as written. If (b) has two mandatory halves and `:1743` says the review round "is not optional", then skipping it is a breach of that half. "Partial compliance" is accurate; "NOT a breach of it" is not, and the same clause names the skip.

I considered the most charitable reading available, that the parenthetical which follows ("Offered as evidence about the cost of the options, not as a finding of a misstep against any role") shows "not a breach" is meant as "not an accusation of misconduct" rather than as a compliance claim. That intent is plausible and I think it is the real one, but it does not rescue the sentence: "NOT a breach of it" attaches grammatically to option (b), not to any role, and a human weighing "would (b) actually be followed?" reads it as a compliance claim about the item's only empirical datum. The fix the reviewer proposes (make `:1735` say what `:1743` says: the actor half was complied with, the review half was not) preserves the intent and removes the inconsistency.

The reviewer's direction note is correct and I confirm it: the inaccuracy makes (b) look MORE followable, so it cuts against the recommended option. That rules out motivated reasoning as an explanation and is a point in the planner's favour, not against it.

Severity `low`: the accurate version is present at `:1743`, in the option (b) trade-offs, which is where a human actually weighs (b).

---

## `R3-1` (`low`): VALID, severity confirmed, one sub-claim corrected

Evidence reproduced: YES for the main claim. `plan.toml:1735` contains both "What is NOT established is who authored it." and, three sentences later, "a planner authored the placeholder".

PARTLY for one sub-claim. The reviewer says the assertion "is reused downstream: `plan.toml:1751` says 'the observed cost of (b)'s planner half was low, which is the best evidence available for (b)', again unhedged." I read `:1751` in full and that characterisation is not quite right: the clause "which is the best evidence available for (b)" is itself a statement about the evidence's strength, so it is not unhedged in the way `:1735`'s conclusion is. The genuinely unhedged instance is the one at `:1735`, singular. This does not change the verdict, but a fix aimed only at `:1751` would be aimed at the wrong sentence.

On the substance, which the orchestrator asked me to judge rather than accept. Is this a real self-contradiction or an acceptable "on the record as it stands" reading? It is closer to the second than the reviewer allows, but there is still something to fix. The paragraph's structure is sound and honest: it states what the hard facts establish (the commit added `Q-68`; no review round exists), states that authorship is not among them, gives the only durable record that speaks to it, checks that record for chronological coherence, and draws a conclusion. That is ordinary argument from best available evidence, and the reader can see exactly what the conclusion rests on because the source is quoted two sentences earlier. A reader is not misled.

What survives is narrower but real: the conclusion sentence spends the epistemic care the paragraph just bought, in an item whose round-2 defect (`NEW-1`, `high`) was an attribution asserted beyond what the record supported. Either the qualification belongs in the conclusion ("on that record, a planner authored the placeholder") or the opening disclaimer is itself too strong and should say that the only record speaking to authorship attributes it to a planner. The reviewer offers both fixes and either works. Since `DEC-3` already requires this exact sentence to be rewritten, the hedge costs nothing to carry, but I want the record to show I judged it on merits and not on that convenience: I would rule it valid at `low` regardless, on the "durable decision record that gains relitigation protection" ground, and I would not rule it above `low` under any reading, because both the hedged and unhedged versions point the same way, the item is still `open`, and nothing has been decided on it.

Severity `low`, confirming the reviewer.

---

## `R3-2` (`low`): VALID, severity confirmed

Evidence reproduced: YES, including the demonstration, which I re-ran rather than read.

The clause is verbatim at `decision-folder-currency.md:7`: "(Find that note by the quoted text, not by a line number: the ledger is append-only in practice, so any line citation into it rots.)" It is text the round-2 fix commit authored (`git diff 09ef94e..981d9f5 -- docs/plans/agent-scaffold.steps/decision-folder-currency.md` shows it replacing the `ledger.md:345` citation `R2-1` flagged), so it is not a re-raise of anything.

The demonstration:

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

Exactly as reported: ten net lines landed mid-file at 334, and the note shifted 345 -> 355. So both halves hold. The inference is backwards (strict append-only is the one case where line numbers do NOT move) and the factual premise is false (the ledger is edited in the middle, which is what moved this very citation).

I judged only the stated reason, as instructed. The instruction it justifies ("find that note by the quoted text") is correct and `R2-1` stays closed either way, which is why this is `low` and not higher. It is nonetheless worth fixing rather than accepting: an implementer-facing sidecar that teaches a convention by giving a self-defeating reason for it is a "document the why" defect, and the why is the only part of that clause doing any work.

One detail on the reviewer's proposed replacement, so the planner does not adopt it uncritically: "the ledger's resume block is rewritten in place, so line numbers into it shift" is not quite the mechanism behind THIS shift, which came from insertion above the note rather than from rewriting the block. The reviewer's alternative fix (delete the reason and keep "Find that note by the quoted text, not by a line number", matching the hedge the same sidecar already uses at `:10`) is the cleaner one. The parallel hedge in the `ask` at `plan.toml:1735` ("the file is append-heavy and line citations into it rot") is a conjunction rather than an inference and does not carry the error, so it does not need changing.

---

## What both reviewers missed

Nothing that rises to a finding, but three things the orchestrator should have on the record.

1. THE LOOP'S RISK CLASSIFICATION WAS SET ON A SMALLER ARTIFACT THAN THE ONE NOW UNDER REVIEW. The ledger (`docs/plans/agent-scaffold.ledger.md:341`) and metrics record 210 classify this artifact `low_risk` at loop-open, so ONE clean round converges. At loop-open the artifact was step 90 alone; `Q-69`, step 91, and step 92 were added in `09ef94e`, after round 1, in response to `T-3a`. `AGENTS.md:56` deliberately fixes the classification once at loop-open, so the orchestrator has followed the rule and I am not asking for a reclassification. But four of the five findings this round, and four of the seven last round, are in material that was not in the artifact when it was classified. The human should know that when weighing the structural question below. It also means only one clean round is needed, which materially changes the cost of continuing (see below).

2. STEP 91'S `blocked_by` IS EMPTY WHILE ITS SIDECAR SAYS IT IS BLOCKED. `plan.toml` gives step 91 `blocked_by = []`, and `exploring-item-actor-boundary.md:3` opens "Not started, and BLOCKED ON A DECISION rather than on another step". I checked whether this is a structured-data gap and it is NOT a defect: `src/plan/source.rs:598-607` validates every `blocked_by` entry as a real STEP slug, so a `q_id` cannot go there, no step in the whole plan uses a non-empty `blocked_by`, and the sidecar states the distinction explicitly in its first sentence. Recording that I checked so nobody spends a round on it.

3. THE FULL CITATION SWEEP IS CLEAN. Given three misnumbered citations in three rounds, I checked all sixteen distinct `file:line` citations in the artifact rather than spot-checking. All resolve. The reviewers each spot-checked a subset; neither did the whole set, and it was worth doing.

---

## Structural read on `Q-69` (advisory, for the human, not a verdict)

Asked for plainly, so: `Q-69`'s PROSE is converging and its OPTION SET is not. Those are separable and the evidence separates them cleanly.

What the three rounds actually show, counted by where the defects landed:

- STEP 90 PROPER (the original four-passage currency fix, the thing the human approved as small): 3 valid findings in round 1 (`T-1`, `T-2`, `T-6`, plus `T-3b`), 2 in round 2 (`R2-1`, `R2-2`), 1 in round 3 (`R3-2`). Monotonically decreasing, every closure verified twice and holding, and the single round-3 finding is fix-induced (a wrong justification clause the round-2 fix authored) rather than a defect in the step's design. Step 90 is one one-clause edit from clean.
- `Q-69`'S WORDING: converging. Round 2's worst was `NEW-1` at `high` (an inference the project's own ledger contradicted). Round 3's wording findings are three `low`s (`DEC-2`, `DEC-3`, `R3-1`), all in one paragraph, all one clause each. Severity is falling and the fixes are sticking.
- `Q-69`'S OPTION SET: not converging. In each of the two rounds `Q-69` has existed, a fresh reviewer has found a valid defect in the option set's PREMISES rather than its prose, and in each case the defect was invisible to the previous round's reviewers AND to the previous round's triager. Round 2: `NEW-4`, the option set never states its own boundary (whether the placeholder must be a plan `[[question]]` at all). Round 3: `DEC-1`, the option set never settles whether the generated fragment's four-item list is exhaustive, which determines the edit surface of two of the three options and can invert one of the two arguments used against the third. Two for two, and both are the same shape: the options were authored before the design space was mapped.

That is the signature of an under-mapped design space, not of a nearly-correct document. A document with a settled design and rough prose produces a shrinking tail of wording findings, which is exactly what step 90 is doing. `Q-69` is producing one fresh premise defect per round on top of a shrinking wording tail.

There is also a self-referential test available, and `Q-69` fails it. `AGENTS.md:45` defines the project's own criterion for when a question is not ready for the human-input contract: "when the options are not yet clear enough to put through the contract ... or the orchestrator finds a decision's design space genuinely open", the remedy being an `exploring` item with a design pass owed. `DEC-1` establishes that answering `Q-69` requires the human to first rule on a sub-question the item never presents (is the generated enumeration exhaustive?), and that the ruling changes the edit surface of two options and possibly the direction of one argument against the third. A decision that cannot be made without first making an unpresented decision is the case that criterion describes.

### Options for the human

(a) SPLIT `Q-69` AND STEP 91 OUT OF THIS ARTIFACT AND DEMOTE `Q-69` TO `exploring`, with a design pass owed, naming the two premises the pass must settle: whether the placeholder must be a plan `[[question]]` at all (`NEW-4`'s boundary), and whether the generated fragment's four-item list is exhaustive or illustrative (`DEC-1`). Step 90 then carries only `R3-2`, a one-clause fix, and converges on the next round. Trade-offs: it uses the project's own mechanism rather than inventing one, adds no phase and no role, and unblocks a step the human approved as small; against it, an exploration pass is heavier ceremony than the clause `DEC-1` might otherwise cost, and it needs a planner re-fold plus a plan review of the split itself, which is not free.

(b) FIX ALL FIVE FINDINGS IN PLACE AND RUN ROUND 4. Trade-offs: only ONE clean round is required (`low_risk`, per the ledger and metrics record 210), and the cap is 5, so there is genuine headroom: a clean round 4 converges outright, and even a round-4 miss leaves round 5, where the convergence check applies before the cap (`AGENTS.md:57`). Against it, the same bet was available and taken at round 2 and again at round 3, and both times a fresh reviewer found a new premise defect in the option set. There is no evidence yet that the premise supply is exhausted, and `DEC-1`'s fix as I have scoped it (rule on exhaustiveness, revise two option edit lists, re-check the (b) comparison) is itself a substantive change to the option set, which is exactly the kind of change that has produced the next round's finding twice running.

(c) DECIDE `Q-69` NOW ON THE ITEM AS IT STANDS, accepting `DEC-1` as a residual. I do not recommend this and record it only for completeness: it folds a decision whose disclosed edit surface is known to be incomplete, and folding is what confers relitigation protection.

### Recommendation, judged against the plan's Project Principles by name

(a), the split.

- GROUND DECISIONS IN EVIDENCE (Principle 6). The evidence that `Q-69` is not ready is not an impression: two independent fresh reviewers found two distinct unstated premises in the only two rounds the item has existed, and each was missed by the round before. Continuing to fix in place bets against that evidence.
- PREFER THE CLEANER LONG-TERM ARCHITECTURE OVER THE SMALLEST DIFF (Principle 1). Adding a clause to `Q-69` so the human can rule on exhaustiveness inside the decision is the smallest diff. Mapping the design space first is what the project's own exploration mode exists for, and the resulting decision is the one that will hold, which matters for an item that gains relitigation protection when folded.
- MINIMAL BY DEFAULT (Principle 2) is the principle that argues AGAINST me, and I want that on the record rather than buried: an exploration pass is more ceremony than a clause, and `AGENTS.md:15` says to match ceremony to stakes. My answer is that the stakes here are a generated const that ships into every scaffolded project plus a decision that becomes relitigation-protected, and that three rounds of ceremony have already been spent on the cheaper path.
- STRUCTURED DATA FIRST, PROJECT FOR HUMANS (Principle 8) supports the split mechanically: `exploring` is a real typed `QuestionStatus` variant (`src/plan/source.rs`), so demoting `Q-69` is a status change in the structured source, not a document rewrite, and the projection follows.

The asymmetry is the decisive practical point. Step 90 is one clause from clean and has been held in a review loop for two extra rounds by an item that was added to its artifact after round 1. Splitting lets the thing the human approved ship, and gives the thing the human did not approve the design pass its own findings say it needs.

If the human instead picks (b), the one thing I would ask is that `DEC-1`'s fix be scoped as I set it out above (all three requirements, not just the reviewer's "cheapest fix"), because the narrow version answers the enumeration question implicitly and leaves the (b) comparison unre-checked, which is the pattern that has produced the next round's finding twice.

---

## Disposition

Must fix before round 4:

| id | verdict | severity | where |
| --- | --- | --- | --- |
| `DEC-1` | VALID | `medium` | `plan.toml` `Q-69` `ask` (`:1733`, `:1741`, `:1743`, `:1745`, `:1751`) and `exploring-item-actor-boundary.md:11`, `:13` |
| `DEC-2` | VALID | `low` | `plan.toml:1733` opening clause (and `:1747` if the term is kept) |
| `DEC-3` | VALID | `low` | `plan.toml:1735` conclusion sentence |
| `R3-1` | VALID | `low` | `plan.toml:1735` conclusion sentence (same sentence as `DEC-3`) |
| `R3-2` | VALID | `low` | `decision-folder-currency.md:7` justification clause |

Plus a re-render of `docs/plans/agent-scaffold.md`.

`DEC-2`, `DEC-3`, and `R3-1` are three clauses in one paragraph of the `Q-69` `ask` and can be done in one pass; `DEC-3` and `R3-1` are the same sentence. `R3-2` is independent and touches step 90's sidecar only. Only `DEC-1` requires re-argument rather than rewording.

Unchanged from earlier rounds, not reopened and not re-raised: `T-4` and `R2-3` remain ACCEPTED RESIDUALS, `T-5` and `T-7` remain DISMISSED, `T-3a` remains the provenance of `Q-69`. I found no evidence that any of those verdicts was wrong.

No dismissals this round, so the high/critical backstop re-check is NOT triggered and no second triager is owed.
