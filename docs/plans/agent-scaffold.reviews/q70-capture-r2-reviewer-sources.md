# Q-70 capture, round 2, source-material audit

Lens: does `Q-70` faithfully represent the documents it was built from. Not the code it cites, not its prose. Sources read: `docs/plans/agent-scaffold.ledger.md` (the 2026-08-11 anchor block and every older paragraph naming the W5 defect, the two owed waivers, the escape routes, the three detection mechanisms, the `agent-scaffold next` defects, the inc3 defects, and every paragraph mentioning `validation-constraints`), the `Q-55-entryroute` receipt and its sibling receipts in `docs/metrics/workflow.jsonl`, `Q-59`, `Q-68` and `Q-69` in the plan TOML, the Design explorations rule in `pack/AGENTS.md`, and round 1's three findings files plus `q70-capture-triage.md`.

Artifact: `git diff main..HEAD` on `review/q70r2-sources`, commits `0a2e1e3`, `3a74e4e`, `129215d`. Every ledger passage below is located by its quoted text, never by a line number.

SIX FINDINGS: one `high`, three `medium`, two `low`. No `critical`.

All six are new. No round 1 finding is re-raised, and no round 1 verdict is contested. Checked mechanically: `grep -ioE 'Q-55-mechanism|Q-55-resumecost|project identity|project-identity|COMPLETE MANDATE|human-input contract|Project Principles'` over all four round 1 files returns nothing, so no site below was in front of any round 1 lens.

`Q-70`'S FACTUAL RELAY IS OTHERWISE ACCURATE, and I checked it rather than assuming it. Every direct quotation the item takes from the ledger resolves byte-for-byte in the source: the W5 fix as "teaching W5 the structured step association W3 already uses", the entry-route ground "the two inc3 defects plus the three `next` defects are already-diagnosed point defects with NO OPEN DESIGN SPACE", item (4)'s "the three `agent-scaffold next` defects routed here by an earlier human decision", the fourth `next` defect's "belongs to the validation-constraints step with the other three", the two-waiver anchor "TWO WAIVERS ARE OWED AND CANNOT YET BE WRITTEN", the caveat about "the roughly eleven `src/checks.rs` citations `Q-55-check21b` deliberately left stale", and the standing cure "an artifact must not assert a count of something that keeps moving" attributed to orchestrator defect (12). The `Q-55-entryroute` receipt carries the four options and the `chosen` value exactly as the item states them, including the declined "Split out the W5 fix first". `Q-59`'s two quoted fragments are exact. The three detection mechanisms match the round 3 paragraph that produced them, and that paragraph is inc4's round 3 (`R3C-3`, `R3C-4` and `R3C-5` are adjudicated in the same block). Both of the item's own reproduction commands run as written and return the sets the item describes. The findings below are omissions and mislabelled attributions, not falsified quotations.

---

## R2B-1. The item omits a receipted human decision that queues a third change to the same record schema, to the same step, and rules that schema owed ONE deliberate edit

SEVERITY: `high`.

CLAIM. `Q-70` states the coupling question as a two-body problem, W5's ownership check against the prospective W6 waiver-note join, and asks the pass to rule whether those two share a mechanism. The durable record already carries a THIRD unbuilt change to the same `Round` schema and the same `check_workflow_toml` join, queued by a human decision with a receipt to the same `validation-constraints` step, whose recorded reasoning is that this schema must take ONE deliberate edit rather than a rider. `Q-70` does not mention it anywhere.

THE SOURCE, `docs/metrics/workflow.jsonl`, the receipt whose `chosen` option names the queue in its own text:

```
{"type":"decision","task":"workflow-enforcement-tier","q_id":"Q-55-mechanism","options":["Anchor plus refusal, identity queued","Anchor, refusal, and identity fields now","Anchor only, minimal"],"recommendation":"Anchor plus refusal, identity queued","chosen":"Anchor plus refusal, identity queued","ts":"2026-07-31"}
```

THE SOURCE, the ledger paragraph beginning "TWO HUMAN DECISIONS CLOSED THE DESIGN PASS (2026-07-31), receipts 236 and 237":

> (1) SCOPE: ANCHOR PLUS REFUSAL, IDENTITY QUEUED. Ship candidate (a) anchored to the plan source PLUS a refusal for unsafe pairings, covering `validate`, `next`, `status` and the ledger path, and QUEUE project identity to the validation-constraints step. The human declined both the wider option (identity fields in the same step) and the narrower one (anchor only). The reasoning that carried it: the pass measured that anchoring alone leaves the DEFECT CLASS open, so the refusal is not gold-plating; while pulling identity in would edit the record schema that W3, W4 and W5 all read, which the calibration close already argued should be ONE deliberate edit rather than a rider on a path fix (Minimal by default, against Ground decisions in evidence, with the evidence deciding where the line falls).

THE SOURCE, what the queued work actually is, the ledger paragraph beginning "THE ONE DIRECTION NONE OF THE FOUR CANDIDATES TOOK, BUILT BY C":

> PROJECT IDENTITY IN THE ROUND RECORD ITSELF, an optional `project` field on `Round` and on the plan's `[meta]`, filtering the join in `check_workflow_toml`. It is the ONLY mechanism measured that separates two projects LEGITIMATELY SHARING ONE MERGED LOG, which no path-based mechanism can address, and it is a no-op when undeclared so the correct case is unaffected. ... This is the data-model direction the sidecar already predicted would land in the queued validation-constraints step, now with a working proof rather than a prediction.

WHAT `Q-70` SAYS INSTEAD, `docs/plans/agent-scaffold.plan.toml:1895`:

> THE COUPLING HYPOTHESIS THE PASS MUST SETTLE ... W5's ownership check and the prospective W6 join BOTH KEY ON HOW A WAIVER NAMES ITS UNIT, and whether they share a mechanism IS A CLAIM NOBODY HAS MEASURED.

MEASURED, that the omission is total and that the queued work is still unbuilt:

```
$ sed -n '1880,1904p' docs/plans/agent-scaffold.plan.toml > q70body.txt   # the whole Q-70 entry
$ grep -c 'Q-55-mechanism' q70body.txt        -> 0
$ grep -c 'Q-55-resumecost' q70body.txt       -> 0
$ grep -oE '.{55}(project|identity).{55}' q70body.txt
 ... every increment identity whose `leading_slug` is not the step it joins under W3
 ... so each group's identity under W3 is the full `task` (`round_increment_id` ...
 ... this project's own standing cure, recorded in the ledger against or ...
 ... so both substrates project identically, which this ...
 ... at exit 0 over a log OUTSIDE the project root. (b) The `agent-scaffold next` ...
$ grep -n "project" src/metrics.rs | grep -v projection   -> (no field; `Round` carries no `project`)
```

So the five hits are "increment identity", "identity under W3", "this project's own", "project identically" and "project root". Not one is the queued work.

WHY IT MATTERS, and why it is `high`. The item's direction (ii), "A REWORK OF HOW A WAIVER NAMES ITS UNIT", is priced at three representations, one of which is "the JSONL `type:"waiver"` arm of `check_record` (`src/metrics.rs:539-601`)". That is a record-schema edit. The human has already ruled, with a receipt, that the record schema W3, W4 and W5 all read is owed ONE deliberate edit rather than a rider, and has already queued a specific edit to it, into the same step this pass defines. A proposal can satisfy every letter of `Q-70`'s mandate, choose direction (ii), and never learn that a receipted decision constrains the schema it proposes to change and that a second change to the same schema is already waiting in the same step. The coupling question the item declares open is stated one participant short, and the missing participant is the one the human has already spoken about.

This is not `R1B-1` or `R1C-3` in new words. `R1B-1` is about the opening `ask` under-framing the mandate `Q-55-entryroute` decided; `R1C-3` is about a third DIRECTION for the W5 fix that the item foreclosed. This is a third BODY OF WORK, decided by a different receipt seven weeks earlier, that the item never registers at all.

---

## R2B-2. "Recorded as deferred inputs so nobody loses them" is a completeness claim, and the record carries at least one more queued item plus one recorded as unowned

SEVERITY: `medium`.

CLAIM. `Q-70`'s out-of-scope paragraph presents itself as the place the eventual step's deferred inputs are preserved, then lists two of them. The durable record queues a third to the same step by receipt, and records a fourth as having no owner anywhere in the plan.

WHAT `Q-70` CLAIMS, `docs/plans/agent-scaffold.plan.toml:1899`:

> EXPLICITLY OUT OF THE DESIGN PASS AND IN THE EVENTUAL STEP, recorded as deferred inputs so nobody loses them. The pass does NOT weigh these: they are already-diagnosed point defects with NO OPEN DESIGN SPACE ... (a) Two pre-existing defects routed from inc3 ... (b) The `agent-scaffold next` defects routed here by the human decision of 2026-07-30.

THE THIRD, the ledger paragraph beginning "`Q-55-resumecost` DECIDED (2026-08-02, receipt)":

> ACCEPT THE `status --resume` COST AS ACCEPTED COST (iv) AND PIN IT, AND QUEUE THE SHARED ROOT CAUSE to the validation-constraints step as ONE treatment ... THE QUEUED ITEM: costs (iii) and (iv) share ONE root cause, `src/main.rs:project_root_of_source`'s fallback to the plan's own parent where there is no `docs/plans`-shaped ancestor, and treating it ONCE in the validation-constraints step is better than accumulating a fresh accepted cost on every new surface (Prefer the cleaner long-term architecture over the smallest diff, One source of truth). This joins the PROJECT-IDENTITY work already queued to that step.

Its receipt exists:

```
$ jq -c 'select(.q_id=="Q-55-resumecost")' docs/metrics/workflow.jsonl
{"type":"decision","task":"workflow-enforcement-tier","q_id":"Q-55-resumecost","options":["Accept as (iv), queue the shared cause","Accept as cost (iv), nothing queued","Carve out the conventionless case"],"recommendation":"Accept as (iv), queue the shared cause","chosen":"Accept as (iv), queue the shared cause","ts":"2026-08-02"}
```

THE FOURTH, recorded as belonging to neither queued item, in the ledger paragraph beginning "THE BACKSTOP CORRECTED BOTH EARLIER AGENTS ON OWNERSHIP":

> The reviewer assigned the defect to the FALLBACK root cause queued to the validation-constraints step; the triager assigned it to the PROJECT-IDENTITY work queued to the same step. IT BELONGS TO NEITHER ... So the ledger half of this false green currently has NO OWNER anywhere in the plan.

WHY IT MATTERS. The paragraph's stated purpose is preservation, "so nobody loses them", and it is the only inventory of the eventual step's deferred inputs that lives in the plan rather than in the ledger. It is short by one receipted queue entry and does not record the one item the record says nobody owns. This project has already ruled on this exact class, in the ledger paragraph beginning "`Q-55-impactclaim` DECIDED (2026-08-10, receipt record 303)": "A DOCUMENTATION-IMPACT LIST THAT ENUMERATES ITS OWN EXCLUSIONS IS A COMPLETENESS CLAIM". This list enumerates its exclusions and makes the claim explicitly.

Distinct from `R2B-1`: that finding is about a design input the PASS needs, this one is about the inventory the eventual STEP needs, and fixing either leaves the other short. They share one source paragraph and no wording.

---

## R2B-3. The item declares its lettered list the COMPLETE MANDATE, twice, and its own body states a ruling the list does not carry

SEVERITY: `medium`.

CLAIM. The fix pass discharged `R1B-2` by consolidating the mandate into a lettered list, and then attached a sufficiency claim the remedy did not ask for. The claim is false against the item's own text.

THE CLAIM, stated twice. `docs/plans/agent-scaffold.plan.toml:1901`:

> THIS LETTERED LIST IS THE COMPLETE MANDATE, and it is the only place in this item where the mandate is complete: every duty stated in the body above is repeated here, so a proposal that satisfies this list satisfies the item, and a proposal that satisfies only part of it is short whatever the body seemed to ask.

And `:1883`:

> The complete statement of what the pass must resolve is the lettered list in WHAT THE PASS OWES BACK at the end of this item; every duty in the body between here and there is repeated in it.

THE DUTY THE LIST DOES NOT CARRY, `docs/plans/agent-scaffold.plan.toml:1889`, on the coverage of the comment at `src/plan/source.rs:785-790`:

> the membership check at `:807` is NEITHER a presence rule NOR the pairing, and `check_record` could not perform it, having no access to a step's declared increments, so it is a rule the comment's own enumeration does not reach. Whether that is a documentation defect, a deliberate design divergence, or correct as it stands is THE PASS'S RULING TO MAKE and a reviewer's to raise; nothing here calls it either way.

"THE PASS'S RULING TO MAKE" is a duty in the plain sense the list uses for (a) to (g). Read (a) to (g) in full: (a) coupling, (b) authoritative path, (c) direction and edit surface, (d) W6 disambiguation, (e) the sub-decision ruling, (f) the scope of mechanisms 2 and 3, (g) the YAGNI boundary. None of them is the comment-divergence ruling. An explorer instructed twice that the list is complete does not open the body, and the ruling the item calls the pass's own is the one it drops.

A SECOND, WEAKER SITE in the same class, recorded so a fix reaches it. `:1895` closes with "must say what the other mechanism costs under that choice", which the round 1 triage kept deliberately (remedy D item 4). The list's (a) asks only "whether W5's ownership check and the prospective W6 waiver-note join share a mechanism, ruled explicitly rather than left implied", which is the coupling ruling without the costing duty attached to it.

WHY IT MATTERS. This is not `R1B-2` re-raised: `R1B-2` was VALID, its remedy landed, and the list now carries all seven duties the triage enumerated. The defect is one the fix pass introduced, in the sentence the fix pass wrote. The item is the sole input to a design pass, it tells its readers twice that one paragraph is sufficient, and that paragraph is not.

---

## R2B-4. The list declared complete omits three of the five components the Design explorations rule it cites requires of every exploration document

SEVERITY: `medium`.

CLAIM. `Q-70` cites the Design explorations rule as its authority for where explorers write, takes the file-location half of it, and then declares a mandate that drops what the same rule requires each proposal to contain.

WHAT `Q-70` CITES, `docs/plans/agent-scaffold.plan.toml:1901`:

> Explorers write to `docs/plans/validation-constraints.explorations/`, one file each, named `Q-70.md` or `Q-70-<disambiguator>.md` when several run in parallel, per the Design explorations rule in `pack/AGENTS.md`.

WHAT THAT RULE ALSO SAYS, `pack/AGENTS.md`, the Design explorations paragraph:

> Each document follows the human-input contract written to a file: the question, the design space (the viable options), each option's trade-offs judged against the numbered Project Principles, a recommendation with its reasoning, and an explicit "what not to build" (the YAGNI boundary).

Five components. `Q-70`'s (a) to (g) carry the YAGNI boundary, at (g), and the edit surface, at (c), which the rule does not require. They do not ask for a design space or option set, for trade-offs judged against the numbered Project Principles, or for a recommendation with its reasoning.

MEASURED:

```
$ grep -ic "principle" q70body.txt     -> 0
```

Zero, in an item whose proposals the rule requires to be judged against the numbered principles, and in a plan file that defines eight of them by name (`[[principle]]` n = 1 to 8, including "Prefer the cleaner long-term architecture over the smallest diff", "Minimal by default" and "Ground decisions in evidence").

WHY IT MATTERS, and why it is not the out-of-scope "the item fails to recommend". The item carrying no options is correct and by design. The defect is what the item asks of the EXPLORERS. Its own closing sentence is:

> The orchestrator then synthesises, moves this item to `open`, and puts the options to the human through the human-input contract.

Options the mandate never asks any proposal to produce, judged against principles the item never names. A proposal that satisfies (a) to (g) in full, which the item states twice is sufficient, is not a document the orchestrator can synthesise into a human-input contract. `Q-69`'s equivalent paragraph is thinner still, so this is a shared convention rather than a regression, but `Q-69` does not claim its list is complete and `Q-70` does, which is what makes it a defect here and not there.

Related to `R2B-3` in cause, distinct in fix: `R2B-3` is a duty the item states and the list drops, this is a component the cited rule requires and the list never had. Correcting either leaves the other wrong.

---

## R2B-5. "The ledger's current next-action paragraph", used twice, names the superseded one

SEVERITY: `low`.

CLAIM. Both times `Q-70` attributes something to "the ledger's current next-action paragraph", the passage it quotes is in the older, explicitly superseded block, and a newer next-action paragraph exists that carries neither quotation.

THE TWO SITES. `docs/plans/agent-scaffold.plan.toml:1895`:

> It is named because the durable record already states the W5 fix in its terms: the ledger's current next-action paragraph describes the fix as "teaching W5 the structured step association W3 already uses", which is this direction and neither of the two above.

And `:1899`:

> and the ledger's current next-action paragraph, item (4), "the three `agent-scaffold next` defects routed here by an earlier human decision".

Both quotations are exact, and both live in the ledger paragraph beginning "THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP, WHICH DOES NOT YET EXIST AND IS PLANNER WORK".

MEASURED, that this is the older one and that it sits under a supersession notice from its own commit:

```
$ git blame --date=short HEAD -- docs/plans/agent-scaffold.ledger.md   # located by quoted text, then blamed
8fa56939 (Test 2026-08-11) THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP ...
8fa56939 (Test 2026-08-11) SUPERSEDED 2026-08-11, READ THIS PARAGRAPH AND IGNORE EVERY ONE BELOW IT: ...
903b70b8 (Test 2026-08-11) THE IMMEDIATE NEXT ACTION, IN ORDER. (1) A WORKTREE-ISOLATED PLANNER registers ...
$ git log --date=iso --format='%h %ad %s' main -- docs/plans/agent-scaffold.ledger.md
903b70b 2026-08-11 17:29:07 +0100 docs: record the resume preflight and route validation-constraints to a design pass
8fa5693 2026-08-11 12:28:19 +0100 docs: record the W5 ownership defect and route it to validation-constraints
```

The quoted paragraph is from 12:28. The newer next-action paragraph is from 17:29, five hours later and before either `Q-70` commit, and it is the paragraph that commissioned this very registration: its item (1) is "A WORKTREE-ISOLATED PLANNER registers the design question as a NEW `[[question]]` with `status = "exploring"`". Its item (4) is "A PLANNER then authors the `validation-constraints` step with its increments", which is not what `Q-70` attributes to "item (4)".

CONSEQUENCE. A reader who follows the label rather than the quoted text lands on the 17:29 paragraph, finds neither quotation, and finds a different item (4). The item's own find-by-quoted-text instruction limits the damage, which is why this is `low` and not higher, and the current anchor does endorse the older block's content ("WHICH REMAINS ACCURATE ON THE BLOCKER, THE FOUR BODIES OF WORK, THE TWO OWED WAIVERS AND INC4'S RESIDUALS"). What is wrong is the word "current", twice, on a paragraph published under "SUPERSEDED ... IGNORE EVERY ONE BELOW IT", and the mis-numbered item reference that follows from it. The fix is a relabel, not a re-anchor: "the ledger's `validation-constraints` routing paragraph" would be true and would still find by quoted text.

---

## R2B-6. The entry-route decision's ground is relayed without the three Project Principles the record attaches to it

SEVERITY: `low`.

CLAIM. `Q-70` cites `Q-55-entryroute` as its own authority and reproduces all three limbs of the decision's ground, and drops the principle name the record attaches to each.

THE SOURCE, the ledger paragraph beginning "`Q-55-entryroute` DECIDED (2026-08-11, receipt record 308)":

> THE GROUND: W5's ownership check and a prospective W6 waiver-note join BOTH KEY ON HOW A WAIVER NAMES ITS UNIT, and whether they share a mechanism is a claim NOBODY HAS MEASURED, which is what an exploration exists for ("Ground decisions in evidence"); the choice between a lookup against the step's declared increments and a rework of waiver-unit naming must be made with W6 in view ("Prefer the cleaner long-term architecture over the smallest diff"); and the two inc3 defects plus the three `next` defects are already-diagnosed point defects with NO OPEN DESIGN SPACE, so they stay OUT of the pass and enter as later increments ("Minimal by default").

`Q-70` carries limb one at `:1895` ("whether they share a mechanism IS A CLAIM NOBODY HAS MEASURED"), limb two at `:1895` as the coupling hypothesis, and limb three at `:1899` ("already-diagnosed point defects with NO OPEN DESIGN SPACE"). It names none of the three principles, and `grep -ic "principle"` over the whole entry returns 0. All three are numbered Project Principles in the same file: n = 6, n = 1 and n = 2.

WHY IT MATTERS. The pass has to judge its directions against the numbered principles (see `R2B-4`), and the human's own decision already recorded which three govern this question and which limb each carries. Restoring three parenthetical names costs one clause and tells the pass what the deciding authority weighed. `low`, because the principles are recoverable from the ledger by any explorer who reads it, and the substance of the ground is relayed correctly.

---

## What I checked and found sound

Recorded so a later round does not re-derive it.

- Every direct quotation from the ledger, from the `Q-55-entryroute` receipt and from `Q-59` resolves in the source. Listed in the preamble above.
- The entry-route scope. The receipt's four options and its `chosen` are exactly as the item states, "Split out the W5 fix first" is genuinely among the declined three, and the item's characterisation of the mandate as the W5 fix plus all three mechanisms matches the ledger's gloss "meaning the W5 fix plus the three detection mechanisms". The other two declined options are each reached in the body, "Planner straight to the step" at `:1895` and "Design pass over all four bodies" implicitly at `:1899`.
- `Q-69`'s discipline. The item's claim that it "follows the discipline `Q-69` records for its own case" is accurate: `Q-69` withdrew its option set after a reviewer found a premise defect in each of its two rounds, and keeps its directions as "CANDIDATE DIRECTIONS for the pass to weigh, extend, or discard. NOT a decided option set, no recommendation attached". `Q-70`'s coupling paragraph now uses that wording verbatim.
- `Q-68`'s shape. The claim "the same shape `Q-68` and `Q-69` use" holds; `Q-68` closes "No receipt and no steps yet; the owed design pass will define the staged build. Do not build until decided."
- The three detection mechanisms match the ledger paragraph beginning "FOUR FINDINGS ARE OUT OF SCOPE AND BECOME ONE BACKLOG STEP, THREE MECHANISMS, ordered by buildability", and that paragraph is inc4's round 3, so "round 3 of inc4 recorded" is right.
- Mechanism (2)'s narrowing to `type:"decision"` records is not an unsupported addition. `jq -r 'select(has("q_id")) | .type' docs/metrics/workflow.jsonl | sort | uniq -c` returns `63 decision` and nothing else.
- The `W6` collision. `grep -n "W6" docs/plans/agent-scaffold.plan.toml` returns six lines, five inside `Q-70` and one at `Q-59`'s `ask`, so "exactly once outside this item" holds. In the ledger, all four `W6` lines mean the waiver-note join, so the second meaning does live only in `Q-59`. `grep -rn "W6" pack/ src/ AGENTS.md .agents/` returns nothing, so no third check is named anywhere in the tree.
- The waiver-id sequence. `workflow-enforcement-tier-w1` through `-w4` all exist in the plan, so "they continue the established `-w1` to `-w4` waiver-id sequence this step already carries" is accurate.
- The four `agent-scaffold next` defects match the two ledger paragraphs the item names, one for one, and the fourth paragraph is dated 2026-08-01 as stated. The two inc3 defects match item (3) of the ledger's routing paragraph.
- Both restated measurements hold against the log: five `type:"round"` records for each fold token, each carrying `step = "workflow-enforcement-tier"` and `increment = null`, peak `consecutive_clean` 1 for the plan fold and 0 for the endproperty fold, `risk_class = "risky"`; and one `type:"escalation"` record per fold token with `human_decision = "decision"` and no `increment`.
- Both of the item's own reproduction commands run as written and return what the item describes.
- Round 1's five remedies all landed. Remedy A at three sites with the three "leave alone" sites left alone, remedy B at four sites, remedy C at two, remedy D's four clauses, remedy E's two citation corrections including the `src/plan/source.rs:791-856` range. The one dismissal (`R1B-3`) stays dismissed and I found no evidence against its reasoning; the item's own find-by-quoted-text convention is if anything better supported now, since `R2B-5` above is a line-number-free citation going stale by supersession rather than by line drift.
- Mechanical state, re-run after the fix commit: `render docs/plans/agent-scaffold.plan.toml --check` prints "up to date" at exit 0; `validate --source docs/plans/agent-scaffold.plan.toml --workflow` prints 309 records valid, 95 steps and 70 questions valid, and "workflow invariants hold" at exit 0.

---

## Defects in a SOURCE document, not in `Q-70`, reported for routing

Out of scope for this artifact by the round's own rules. Recorded so the orchestrator can route them.

1. THE LEDGER'S OWN "FOUR THINGS" LIST IS SHORT. The paragraph beginning "THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP" says the step "now carries FOUR things" and enumerates them. Two further bodies are queued to that same step by receipted human decisions recorded elsewhere in the same ledger: project identity (`Q-55-mechanism`, receipt 236), and the `project_root_of_source` shared root cause (`Q-55-resumecost`). A third item, the ledger half of the `run_next` false green, is recorded as belonging to neither and as having "NO OWNER anywhere in the plan". This is the upstream cause of `R2B-1` and `R2B-2`: `Q-70` inherited an incomplete inventory. Correcting `Q-70` alone leaves the ledger still saying four.

2. THE CURRENT ANCHOR'S FORWARD REFERENCE DOES NOT RESOLVE LITERALLY. The anchor beginning "RESUMED AFTER A COMPACTION 2026-08-11 (SECOND ANCHOR MOVE THIS DAY)" says "READ THIS PARAGRAPH FIRST, THEN THE ONE DIRECTLY BELOW IT, WHICH REMAINS ACCURATE ON THE BLOCKER, THE FOUR BODIES OF WORK, THE TWO OWED WAIVERS AND INC4'S RESIDUALS". The paragraph directly below it is the `Q-55-entryroute` decision record, which covers none of those four topics. The paragraph that does cover them is the "SUPERSEDED 2026-08-11" anchor several paragraphs further down. This ambiguity is what makes `R2B-5` an easy mistake to make.

3. THE TWO LIVE "THREE"S. Already recorded by `Q-70` itself as owed corrections, at `:1899`. Not new, listed only for completeness of the routing set.

---

## What I settled by running and what by reading

RUN: `R2B-1`'s omission measurement and the `Round`-schema check that the queued work is still unbuilt; `R2B-2`'s receipt lookup; `R2B-4`'s zero-principle count; `R2B-5`'s blame and commit ordering, which is the whole finding; `R2B-6`'s zero-principle count; and every item in "What I checked and found sound" except the four listed next, including both of the item's own reproduction commands, the `q_id` record-type census, the `W6` counts across the plan, the ledger and the tree, the waiver-id sequence, the round and escalation record measurements, and `render --check` plus `validate --workflow`.

READ: `R2B-3`, which is a claim about what the item's own body says against what its own list carries, and is settled by opening `:1883`, `:1889` and `:1901`; `R2B-4`'s comparison of the five required components against (a) to (g); the quotation checks against the ledger, the `Q-55-entryroute` receipt, `Q-59`, `Q-68` and `Q-69`; and the remedy-by-remedy check that round 1's fix pass landed.

Nothing above is presented as measured that was not run. Fixtures: none were needed, so none was built, and nothing outside this worktree was written.
