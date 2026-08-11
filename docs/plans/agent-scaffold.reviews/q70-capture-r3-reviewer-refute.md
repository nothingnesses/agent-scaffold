# Q-70 capture, round 3, reviewer: refute the framing

Lens: attack the PREMISE of the item rather than its contents. Is a design pass the right vehicle, is the scope right, is it solving the right problem, is anything load-bearing but unexamined.

Artifact: `git diff main..HEAD` on `review/q70r3-refute`, commits `2c2be88`, `58b677f`, `198556e`, `61b1d35`. The `[[question]] Q-70` entry at `docs/plans/agent-scaffold.plan.toml:1880-1904`, the `q70-capture` `[meta].orphan_tasks` token, the empty `docs/plans/agent-scaffold.questions/Q-70.md`, and the regenerated `docs/plans/agent-scaffold.md`.

Binary: `target/debug/agent-scaffold` built in this worktree at HEAD. Fixture root: `<scratch>/refute-q70/`. One fixture was built, from a fresh `cp -r docs/`. Nothing outside that directory was written or deleted.

RESULT: FIVE FINDINGS, three `high` and two `medium`. This is NOT a clean round.

Every one of the five is an OMISSION OF A CODE FACT that determines something the item hands to the pass as open. None of them contradicts a settled verdict from round 1 or round 2, and none re-raises one. I checked: the tokens `shim`, `pre-migration` and `Inc 2` occur ZERO times across all eight files in `docs/plans/agent-scaffold.reviews/`, and `valid_findings` as a schema question occurs nowhere in them. The axis this round found is the one both triagers predicted was uncovered: the item read against the CODE OF THE CHECKS IT COMMISSIONS, rather than against the code it already cites.

I record honestly which conclusions rest on measurement and which on argument. `R3B-1`, `R3B-3`, `R3B-4` and `R3B-5` rest wholly on measurement and citation. `R3B-2` rests on two measured facts plus one step of argument about what a check that does not exist must do; that step is marked in the finding.

---

## R3B-1. The codebase already states a direction for this join class, and the item records neither the statement nor the fact that W5 half-implements it

SEVERITY: `high`.

CLAIM. The item presents three candidate directions as an open space and requires every proposal to choose. The code carries an explicit statement of the project's intended direction for exactly this join class, and W5's OWN OTHER CHECK, in the same function, already follows it. The item cites neither, so the pass is commissioned to deliberate a comparison the source has already leaned.

EVIDENCE, all at the line.

1. `leading_slug`'s doc comment closes, at `src/workflow.rs:83-87`: "Inc 2 retires this risk for NEW data: a `round`/`escalation` record may carry a structured `step`/`increment` id, and `round_step_slug`/`escalation_step_slug` (and their increment counterparts) prefer it, so a record with the field joins without ever reaching this lexical strip. This shim remains only for pre-migration records that omit the structured id."

2. Inside `w5_problems`, the two checks are asymmetric. The ownership check is lexical: `if leading_slug(increment) != waiver.step` at `src/workflow.rs:564`. The record-backed evidence check, thirty lines later in the same function, is structured, and says so in its own comment at `:590-592`: "Tie the joined escalation to the unit the waiver exempts, preferring the escalation's structured ids (Inc 2) over the `leading_slug`/`task` shim when it carries them", implemented at `:594-596` through `escalation_increment_id` and `escalation_step_slug`. So W5 was migrated on one axis and not on the other.

3. The item never reaches either fact. Term census over the entry (`awk 'NR>=1880 && NR<=1904'`): `shim` 0, `pre-migration` 0, `Inc 2` 0. Its one citation into this doc block is `src/workflow.rs:64-68`, which is the `INCREMENT_MARKER` comment about the `-incA` / `-incB` alphanumeric run. The retirement sentence is at `:83-87` and is never cited.

4. `leading_slug` is documented as "The leading step slug of a `task`" (`:71`). W5 applies it to `waiver.increment`, a plan-authored id that is not a `task` and carries no structured alternative in W5's inputs. The only way to honour the stated preference for a structured id on this axis is to read the round log, which is direction (iii).

WHY THIS IS A FRAMING DEFECT AND NOT A DETAIL ONE. `Q-55-entryroute`'s recorded ground is a claim about the STATE OF MEASUREMENT: "whether they share a mechanism is a claim NOBODY HAS MEASURED, which is what an exploration exists for" (`docs/plans/agent-scaffold.ledger.md:533`, attached to Project Principle 6, "Ground decisions in evidence"). A doc comment that names the intended direction and a function that already half-implements it are measurement, and they were in the tree when the decision was recorded. The item's three-way framing is therefore wider than the evidence supports.

WHAT I AM NOT SAYING. I am not saying the item should recommend direction (iii). Recording a code fact is not a recommendation, and this item and this loop have already established that: round 1 remedy D required direction (iii) to be named "WITHOUT recommending it", and the item carries it as "THIS IS RECORDED, NOT RECOMMENDED". The same treatment applies here.

IMPACT IF UNFIXED. An explorer weighs three directions without knowing that the code states one of them as the migration in progress, and can produce a well-argued proposal for (i) or (ii) that contradicts the source's own stated direction. The human then decides on that basis. `high`, on the round 1 and round 2 precedent for findings that change what the pass is told to consider on the axis the pass exists for (`R1C-2`, `R1C-3`, `R2A-1`, `R2B-1`).

---

## R3B-2. The prospective W6 join cannot be built without widening the same record struct a receipted decision already constrains, and the item prices it at zero

SEVERITY: `high`.

CLAIM. The item describes mechanism (1) as a join "against the `valid_findings` of the round records the same command already reads". The command reads them; the projection every check consumes drops them. W6 therefore requires an edit to `Round`, which is the same struct the queued project-identity work edits and which the human has already constrained to "ONE DELIBERATE EDIT RATHER THAN A RIDER ON A PATH FIX", a constraint this item itself records one paragraph earlier.

MEASURED.

1. `Round` (`src/metrics.rs:620-651`) carries exactly `line`, `task`, `artifact`, `outcome`, `consecutive_clean`, `risk_class`, `step`, `increment`. No `valid_findings`.
2. `parse_rounds` (`src/metrics.rs:660-709`) reads `task`, `artifact`, `outcome`, `consecutive_clean`, `risk_class`, `step`, `increment` and nothing else. `valid_findings` is required by `check_record` (`src/metrics.rs:367`, `:454`) and is discarded by the projection.
3. `workflow.rs` owns no JSON parsing: `grep -c serde_json src/workflow.rs` returns 0, and its own doc says so at `src/metrics.rs:615-618`. Both entry points funnel through `metrics::parse_rounds` (`src/workflow.rs:162`, `:189`).
4. The only waiver-to-rounds join anywhere in the tool is W3's covering-waiver match, `waiver.unit == Increment && waiver.increment == Some(round_increment_id(round)) && waiver.step == step.slug`, at `src/workflow.rs:498-502`. W5's evidence join is waiver-to-escalations (`:583-598`), not waiver-to-rounds. `waivers_from_toml` (`:237-267`) is a flattening and joins nothing.
5. The waiver `note` is consumed by exactly one site in the whole tool, `src/plan/render.rs:527`, which writes it into the generated Markdown. No check reads it.

THE ONE STEP OF ARGUMENT, marked as such. A check that compares a note's per-round breakdown to the rounds of a waived unit must (a) obtain that unit's round records and (b) read each record's `valid_findings`. (a) has exactly one existing implementation, W3's join at `:498-502`; (b) requires a field the projection drops. I did not build W6, so this is a derivation from where the data lives rather than a measurement of a check that exists. It is falsifiable: anyone who can name a second waiver-to-rounds join, or a `valid_findings` reader inside `workflow.rs`, refutes it.

WHAT IT DETERMINES, which is why this belongs to the framing lens rather than to a pricing lens. The coupling question is decided by (a): W6 must key on the round log, because that is the only place its inputs live. So W5 and the prospective W6 share a mechanism IF AND ONLY IF W5 adopts the round-log join. Under direction (i) or (ii) they necessarily use different keys and the project ends with two notions of waiver ownership. That is the pass's letter (a) ruling, available from the source without deliberation.

AND IT COMPOUNDS THE CONSTRAINT THE ITEM ALREADY CARRIES. The item registers, correctly, that `Q-55-mechanism` queued an optional `project` field on `Round` and that the human's reasoning requires the record schema to take one deliberate edit rather than a rider. Unstated: W6 needs a second field on that same struct. Two of the three participants the item names for the coupling question both edit `Round`, and the item names only one of them as doing so.

IMPACT IF UNFIXED. Mechanism (1) is presented to explorers as an enforcement-only gap over data already in hand, when it is a record-schema change under a recorded human constraint. A proposal that rules "they do not couple" on that basis rules wrongly, and the cross-pricing the pass exists to deliver (letter (a), the deliverable `Q-55-entryroute` names as its ground) is computed from a zero. `high`, the same class and reach as `R1C-2`.

NOT A RE-RAISE. `R1C-2` and `R1C-4` under-price directions (i) and (ii) of the W5 FIX. `R2B-1` establishes that `Round` carries no `project` field and that a receipt constrains it. Neither reaches W6's own schema requirement; `valid_findings` appears in no review file in the directory.

---

## R3B-3. Two of the pass's three mechanisms are provably uncoupled from the question the pass exists to settle, and the item hands that to the pass as an open ruling

SEVERITY: `medium`.

CLAIM. The item scopes the pass to the W5 fix plus three detection mechanisms, and letter (f) asks the pass to rule whether mechanisms (2) and (3) are DESIGNED or only BOUNDED, saying the item "has deliberately never said either way". Which they are is determined by where their data lives, and it is measurable now: neither touches a waiver, a round record, or the join the pass exists to settle.

EVIDENCE.

1. Mechanism (2), dangling decision-receipt detection, joins `type:"decision"`.`q_id` to the registered `[[question]]` ids. Those are exactly W4's two inputs: `w4_problems(questions, decisions, baselines)` at `src/workflow.rs:220`, fed by `plan.question_views()` and `metrics::parse_decisions`. It is the converse of the check W4 already performs. It reads no waiver and no round.
2. Mechanism (3), the quotation resolver, resolves `src/checks.rs:<line>` citations in `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md` against source content. It reads no workflow record at all.
3. The recorded ground has three limbs and none reaches them. `docs/plans/agent-scaffold.ledger.md:533`: the unmeasured W5/W6 coupling ("Ground decisions in evidence"); the W5 direction choice made with W6 in view ("Prefer the cleaner long-term architecture over the smallest diff"); and why the inc3 and `next` defects stay OUT ("Minimal by default"). The first two are about W5 and W6. The third is an exclusion rule.
4. The same record's next-action paragraph scopes explorer output to the coupling alone: "(2) EXPLORERS write to that directory, each ruling on the W5/W6 coupling and carrying an explicit 'what not to build' boundary" (`docs/plans/agent-scaffold.ledger.md`, the paragraph beginning "THE IMMEDIATE NEXT ACTION, IN ORDER").

SO THE BOUNDARY IS DEFENDED IN ONE DIRECTION ONLY. The item states, correctly and with its receipt, why entries (a) and (b) are out. It states no ground at all for why mechanisms (2) and (3) are in a DESIGN pass, and the deciding record supplies none beyond the decision's own "meaning" clause. On the measured facts they are in the STEP and bounded by the pass, not designed by it, and the item can record that as a measured input in the same way it records every other one.

IMPACT IF UNFIXED. Explorers may spend a proposal designing two mechanisms that share no design surface with the ruling the pass was commissioned for, and letter (f) invites exactly that. It is `medium` rather than `high` because the failure wastes pass effort rather than producing a wrong ruling, and because an explorer who opens either mechanism's data reaches the same conclusion.

---

## R3B-4. The rule the pass exists to fix has a premise that cannot hold on the substrate this project uses, and nobody has asked whether it should exist there

SEVERITY: `high`.

CLAIM. Every one of the item's three candidate directions is a way to FIX the ownership rule. None asks whether the rule has work to do. On `[meta].primary = "toml"`, the state the rule exists to catch is unrepresentable, its sibling rule is unreachable, and its message asserts an ownership fact that need not be true of the plan. That yields a fourth direction, smaller than all three named, which the item forecloses by never raising the question.

MEASURED AND CITED.

1. The project's substrate is TOML: `docs/plans/agent-scaffold.plan.toml:4`, `primary = "toml"`.
2. The rule's stated purpose, in the function doc at `src/workflow.rs:525-527` and again at the site at `:559-561`: "An `increment`-unit waiver's `step` must own its `increment` (`leading_slug(increment) == step`), so a waiver naming a real-but-wrong step is reported rather than silently mis-scoped."
3. On the TOML path a waiver CANNOT name a step. `waivers_from_toml` sets `step: step.slug.clone()` from the containing `[[step]]` (`src/workflow.rs:258`), and the TOML `Waiver` struct has no `step` field at all (`src/plan/source.rs:279-300`, `#[serde(deny_unknown_fields)]`). The item records this itself, as escape route 2. So the scenario the rule exists to report cannot occur on the substrate the project uses.
4. W5's FIRST rule is unreachable on that path for the same reason. `check_workflow_toml` (`src/workflow.rs:180-195`) feeds `plan.step_views()` and `waivers_from_toml(plan)`, both derived from the same `plan.steps`, so `waiver.step` is always a member of `slugs` and `if !slugs.contains(waiver.step.as_str())` at `:553` can never fire.
5. What the rule ACTUALLY does on that path is refuse any increment id not spelled `<containing-step-slug>-inc<alnum>`, and report a step that need not exist. DEMONSTRATED. Fixture `<scratch>/refute-q70/fake-owner/`, a fresh `cp -r docs/` with one `[[step.waiver]]` added under `workflow-enforcement-tier` (`id = "workflow-enforcement-tier-wX"`, `unit = "increment"`, `increment = "totally-not-a-step-inc1"`, `reason = "review-skipped"`, `evidence_tier = "self-declared"`):

```
$ agent-scaffold validate --source <S>/fake-owner/docs/plans/agent-scaffold.plan.toml \
    --metrics <S>/fake-owner/docs/metrics/workflow.jsonl --workflow
<PLAN>: waiver `workflow-enforcement-tier-wX` on step `workflow-enforcement-tier` names increment `totally-not-a-step-inc1`, which is not one of the step's increments
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-wX`: increment waiver names step `workflow-enforcement-tier` but increment `totally-not-a-step-inc1` belongs to step `totally-not-a-step`
EXIT=1
$ grep -c 'slug = "totally-not-a-step"' <S>/fake-owner/docs/plans/agent-scaffold.plan.toml
0
```

W5 asserts that the increment "belongs to step `totally-not-a-step`", which is not a Roadmap step, and W5's own real-step rule stays silent because `waiver.step` was inherited and is real. The ownership assertion is a substring of the id, not a fact about the plan.

THE FOURTH DIRECTION, recorded and NOT recommended. Scope the ownership rule to the substrate where its premise holds (the JSONL `type:"waiver"` record, whose `step` IS independently authorable), or drop it in favour of the structural membership check at `src/plan/source.rs:807` that already runs on the TOML path. It touches no shared type, no machine output contract, no second substrate's projection and no waiver schema, so it is smaller than directions (i), (ii) and (iii) alike. On the item's own measured fixture, declaring the two fold tokens as `[[step.increment]]` entries silences the source path and leaves only W5, so this direction plus that declaration is a complete unblocking of the two owed waivers.

WHAT IT COSTS, recorded so the finding is not one-sided. `run_checks` is deliberately one implementation across both substrates (`src/workflow.rs:196-200`, `:224-226`), and a substrate-conditional W5 rule pushes against that parity. That is a real trade-off and a real thing for the pass to weigh. It is not a reason for the item to omit the option; it is the content of the ruling.

HOW THIS BEARS ON LETTER (b). The item asks which of W5 and `src/plan/source.rs:807` is authoritative for waiver ownership, and frames it as symmetric ("THE TWO PATHS RETURN OPPOSITE VERDICTS ON THE SAME WAIVER"). They are not symmetric: the source path consults the step's declared increment set structurally, and W5 consults a substring and names a step that need not exist. The item records the disagreement and not the asymmetry, which is the fact that would decide it.

IMPACT IF UNFIXED. The pass weighs three ways to teach a check a better lookup, on a substrate where the check's premise does not hold, and the step authored from the winning proposal widens a shared serialised type or reworks three waiver representations where a scoping change would have done. That is the wrong end of Project Principle 2, "Minimal by default", and it is a check admitting a state that cannot arise, against Project Principle 5, "Make illegal states unrepresentable". `high`, on the `R1C-3` precedent for a foreclosed direction, and stronger than `R1C-3` in one respect: `R1C-3`'s direction was already named in the durable record, and this one is named nowhere.

---

## R3B-5. The item applies its own moving-target cure to numbers and not to the record it restates, and that is where two rounds of review cost went

SEVERITY: `medium`.

CLAIM. The item has a stated cure for content that keeps moving: state the property, give the command, do not carry the figure. It applies that cure to counts and not to the ledger prose it restates, and the restated prose is where nearly half the loop's valid findings have landed.

MEASURED. Classifying the twenty valid findings by what the defective material IS, rather than by remedy, NINE are defects in content the item copies from or points into the ledger and the receipts:

- Round 1: `R1A-1` (a relayed ledger figure), `R1A-4` (a claim about which ledger passages say what), `R1B-1` (a relay of the `Q-55-entryroute` mandate).
- Round 2: `R2A-2` (an enumeration of ledger passages, short by four), `R2A-4` (a relayed ledger figure), `R2B-1` (an inherited inventory missing a receipt), `R2B-2` (the same inventory at a second site), `R2B-5` (a ledger paragraph labelled "current" that is published under a supersession notice), `R2B-6` (the deciding record's principle attributions dropped in the relay).

In round 2 alone that is five of nine. The remaining eleven are defects in the item's OWN first-hand work, and the triagers' own summaries record that this half held up: "I re-ran the item's own two recorded fixtures from scratch and both reproduce byte-for-byte", "NO FINDING SHOWS `Q-70` ASSERTING SOMETHING THE TOOL CONTRADICTS", twice.

THE CURE THE ITEM ALREADY OWNS AND DOES NOT APPLY HERE. It states the pointer convention for the ledger twice in its own text, "find them by the quoted text ... rather than by a line number" and "Find all of these by their quoted text rather than by a line number", and it applies the property-not-figure rule to four counts. It applies neither to restated ledger PROSE, which moves the same way and for the same reason: this loop appends to the ledger every round. `R2B-5` is the cure's own failure mode arriving on schedule, and the round 2 triager measured a second one, the find-by-quoted-text handle that now resolves to two paragraphs because the ledger quoted itself.

SIZE, as context rather than as the finding. The three `exploring` items of this shape: `Q-68` 490 words, `Q-69` 1449 words, `Q-70` 4435 words. The growth is almost entirely in restated record content and in duty lists; the measured spine (the blocker, the five escape routes, the two-path fixture) is the part that has survived two rounds intact.

WHAT I AM NOT CLAIMING, because I could not ground it. I am NOT claiming the item should have been three sentences, and I attacked that and failed; see the attacks section. The recorded ground for the item's weight beats it. The narrower claim I can ground is that the class of content generating half the findings is the class the item's own conventions already tell it to point at rather than restate.

IMPACT IF UNFIXED. The item stays coupled to a document that changes every round, so each round produces fresh staleness findings and the loop does not converge on its own terms. That is a cost to this loop rather than to the pass. `medium`.

---

## Every attack I mounted, and where it failed

Recorded in full, because a failed attack tells the next round what not to spend a lens on. Seven of the eleven below failed, and two of the failures were informative enough to change a finding.

1. THE PASS'S QUESTION IS ALREADY ANSWERED, SO THE VEHICLE IS WRONG AND THE ITEM SHOULD BE `open` WITH OPTIONS RATHER THAN `exploring`. FAILED, and I pushed it hard. Even granting `R3B-1`, `R3B-2` and `R3B-4`, the pass still owes rulings with no code answer: letter (e), whether the `Q-55-<suffix>` receipt ids are dangling or a convention the detector must model, is a policy question about how this project records sub-decisions; letter (d), which check is renumbered, is a naming decision; letter (g), the YAGNI boundary, is definitionally the pass's. And letter (b) has a real counterweight in the cross-substrate parity the checks are built on. The premise survives: what I could establish is that the pass's CENTRAL COMPARISON is much narrower than the item presents, not that the vehicle is wrong. This is where the lens spent most of its effort and it is the most useful failure in the list.

2. THE COUPLING IS THIN, SO THE DECLINED OPTION "SPLIT OUT THE W5 FIX FIRST" WAS THE RIGHT ONE. FAILED, and it failed in the opposite direction to the one I expected. Measuring what W6 must do (`R3B-2`) shows the coupling is THICKER than the item says, not thinner: any structured W5 fix and any W6 join key on the same round-log join, and W6 additionally edits the same `Round` struct the queued project-identity work edits under a recorded human constraint. The human's ground for designing them together is better supported now than when it was recorded. I set out to refute this limb of the entry-route decision and ended up corroborating it.

3. THE BLOCKER HAS A SIMPLER RESOLUTION THE RECORD MISSED: DECLARE THE FOLD TOKENS AS THEIR OWN `[[step]]` ENTRIES, SO THE INHERITED `waiver.step` EQUALS `leading_slug(increment)` AND W5 PASSES. FAILED, settled by reading the code. W5 would indeed pass, since `leading_slug("workflow-enforcement-tier-fold")` returns the value unchanged and would equal the inherited `step`. But the waiver would then cover nothing: W3's covering-waiver match requires `waiver.step == step.slug` at `src/workflow.rs:501`, and the ten round records carry the structured `step` id `workflow-enforcement-tier`, so `round_step_slug` (`:119`) joins them to that step and only a waiver nested on THAT step can exempt them. The escape route the item enumerates as (2) is therefore closed for a second, independent reason it does not state, and the item's list of five is not short.

4. THE SCOPE IS THE SAME BUNDLING AS THE REJECTED WIDER PASS, AT A SMALLER RADIUS. PARTLY SUCCEEDED, became `R3B-3` in a narrower form. The strong version, that the scope is arbitrary, failed: the decision's exclusion rule for entries (a) and (b) is principled, recorded, and correctly relayed. What survived is that the INCLUSION of mechanisms (2) and (3) in a DESIGN pass has no recorded ground and no measurable coupling.

5. LETTER (f) IS UNANSWERABLE AND SHOULD BE DROPPED. FAILED as stated, and inverted into `R3B-3`. The item is right that the deciding record does not say whether mechanisms (2) and (3) are designed or bounded. The defect is not that the question is asked, it is that the item does not record the measured facts that answer it.

6. THE ITEM IS THE WRONG ARTIFACT TYPE: AN EXPLORER BRIEF BELONGS IN THE ORCHESTRATOR'S PROMPT OR IN THE EXPLORATIONS DIRECTORY, NOT IN A PLAN FIELD. FAILED, on two grounds I could not beat. First, `pack/AGENTS.md:65` requires the item to point at the exploration by path while it is `exploring`, and whether the placeholder belongs in the plan AT ALL is already registered as `Q-69`'s premise 2 and reserved to `Q-69`'s own pass; raising it here would duplicate a registered open question and pre-empt another pass. Second, the ledger records the ground for this item's weight explicitly, that `Q-70` is "the SOLE INPUT to a design pass on a validator invariant, the highest-blast-radius surface this tool has, so a wrong premise propagates into every explorer proposal and into the human's decision". A durable prompt beats a transient one on this project's own terms. What survived is the much narrower `R3B-5`.

7. THE ITEM SHOULD HAVE BEEN THREE SENTENCES, AND THE ELABORATION BOUGHT NOTHING. FAILED, and the record refutes it. The elaborated half is the half that HELD: the five escape routes, the two-path fixture and the blocker measurement all reproduced byte-for-byte under two independent triagers, and escape route 2 was measured to be STRONGER than the durable record had it. A three-sentence item would have sent explorers to re-derive all of that, and the ledger records that the same brief carried two figures from durable records that were both wrong when measured. The elaboration buys a pass that does not repeat the loop's known failure mode.

8. THE `risky` CLASSIFICATION AND THE TWO-CLEAN-ROUND BAR ARE DISPROPORTIONATE FOR A QUESTION REGISTRATION. FAILED, and it is out of scope besides. The ledger records the ground for `risky` in advance, and two rounds have vindicated it: each produced a `high`, and round 2's `high` was introduced by round 1's own fix pass. The recorded counter-argument, that a registration changes no behaviour so one clean round suffices, is the same reasoning the ledger records as having produced two of this project's longest loops.

9. THE TWO OWED WAIVERS ARE NOT ACTUALLY OWED, SO THE BLOCKER IS SELF-INFLICTED. FAILED. W3 skips steps that are not `complete` (`src/workflow.rs:445-447`), so leaving `workflow-enforcement-tier` at `in-progress` is not a resolution, it is the current state and the definition of the step being blocked. The record ties the step's completion to the W5 fix in terms, and that is not this item's to reopen.

10. `q70-capture` IN `[meta].orphan_tasks` WAS FALSE WHEN WRITTEN AND IS A DEFECT. FAILED, and it was already ruled. The round 1 triager measured that the token had no round record and read it as a deliberate pre-declaration. Re-measured now: `grep -c 'q70-capture' docs/metrics/workflow.jsonl` returns 2, so the pre-declaration is true and the objection has expired.

11. THE ITEM'S DUTY LIST HAS GROWN EVERY ROUND, SO THE LIST IS THE WRONG MECHANISM AND SHOULD BE DELETED IN FAVOUR OF THE BODY. FAILED. Round 1's `R1B-2` measured the concrete cost of NOT having a consolidated list (an explorer can satisfy the deliverables paragraph and omit four rulings), and round 2's remedy F already prescribes the right treatment for the growth, dropping the sufficiency claim rather than the list. The mechanism is not the problem; the guarantee attached to it was, and that is settled.

---

## Separately: a defect in `src/`, clearly marked and NOT part of the findings above

Reported here because the scope rules place `src/` defects outside the review. It is the same code fact `R3B-4` uses as evidence, stated as a source defect rather than as a framing one.

W5's increment-ownership message asserts an ownership fact that need not be true of the plan. At `src/workflow.rs:565-571` it formats "increment `{}` belongs to step `{}`" with `leading_slug(increment)`, a substring of the id. Demonstrated above: a TOML waiver naming `totally-not-a-step-inc1` produces "belongs to step `totally-not-a-step`", where no such Roadmap step exists, while W5's own real-step rule at `:553` stays silent because the waiver's `step` was inherited from its containing `[[step]]` and is real. On `[meta].primary = "toml"` the message can name a step the plan does not contain, on every firing.

Related and in the same function: on the TOML path, `if !slugs.contains(waiver.step.as_str())` at `src/workflow.rs:553` is structurally unreachable, because `check_workflow_toml` (`:180-195`) derives both `plan.step_views()` and `waivers_from_toml(plan)` from the same `plan.steps`.

Whether either is worth an item is not mine to decide, and both are inputs the `validation-constraints` step will meet.

---

## Mechanical state, checked independently

```
$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check          -> up to date, EXIT 0
$ agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml \
    --metrics docs/metrics/workflow.jsonl --workflow
310 records valid; 95 steps, 70 questions valid; workflow invariants hold;   EXIT 0
$ LC_ALL=C grep -cP '[^\t\x20-\x7e]' on the three changed files              -> 0, 0, 0
$ grep -n 'slug = "validation-constraints"' docs/plans/agent-scaffold.plan.toml -> (none)
```

`Q-70`'s "NO step exists yet" still holds.

## What I settled by running and what by reading

RUN: the `<scratch>/refute-q70/fake-owner` fixture and its exact output (`R3B-4`); the term census over the entry body (`R3B-1`); the review-directory token sweep for `shim`, `pre-migration`, `Inc 2` and `valid_findings`; the word counts for `Q-68`, `Q-69` and `Q-70`; the `q70-capture` round count; `grep -c serde_json src/workflow.rs`; `grep -c 'slug = "totally-not-a-step"'`; and the mechanical checks above.

READ, at the line: `src/workflow.rs:83-87`, `:180-200`, `:206-221`, `:224-267`, `:445-447`, `:496-502`, `:525-527`, `:544-598`; `src/metrics.rs:615-651`, `:660-709`; `src/plan/source.rs:279-300`, `:418-430`; `src/plan/render.rs:527`; `docs/plans/agent-scaffold.ledger.md:530-536`; the `Q-55-entryroute`, `Q-55-mechanism`, `Q-55-resumecost`, `Q-55-impactclaim` and `Q-55-check21b` receipts in `docs/metrics/workflow.jsonl`; and both triage files in full, held throughout so no settled finding is re-raised.

Nothing above is presented as measured that was not run. The one argued step is marked inside `R3B-2`.
