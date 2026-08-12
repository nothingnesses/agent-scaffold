# Q-70, the evidence lens: which of this pass's questions are still questions

This exploration does not primarily propose a design. It rules, for every lettered duty `Q-70` sets, whether the duty is ALREADY SETTLED BY AVAILABLE EVIDENCE or GENUINELY OPEN, and for each "settled" it states what the evidence settles it to. It also checks the opposite direction: what the item or the record treats as settled that is not.

Where a duty is settled, this document says whether it should still reach the human as a CHOICE or as a FINDING. Those are different things. A human asked to choose between two options that the evidence has already collapsed to one spends attention for nothing.

## Provenance of every figure below

Every measurement was taken first-hand in the worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/exp-vc-evidence`, branch `explore/vc-evidence`, at commit `736d526`, which is an ancestor of `main`. The worktree was clean before and after; this file is the only thing written to it. Fixtures and scratch builds live under a session scratchpad directory outside the repository and outside any other tree.

Two binaries were built from a copy of that worktree: `agent-scaffold-base`, an unmodified build, and `agent-scaffold-iii`, the same tree with one rule in `src/workflow.rs` replaced (the direction (iii) proof-of-concept described below). Neither the worktree's `src/` nor its plan was edited.

Headline figures were re-measured at the end of the session and are unchanged: population 6, receipts 63, registered questions 70, dangling 41.

## The question, as `Q-70` frames it

How to fix W5's waiver-ownership check, and whether that fix shares a mechanism with the prospective W6 waiver-note join, so the two are designed together rather than one at a time. The pass's mandate is wider: the W5 fix plus all three detection mechanisms, per the `Q-55-entryroute` receipt.

## The central comparison is a measurement, and here is the measurement

`Q-70` frames the coupling as "A CLAIM NOBODY HAS MEASURED". That framing is wrong in a specific way. The claim is not empirically unknown. It is a CONSEQUENCE OF THE DIRECTION CHOSEN FOR THE W5 FIX, and each consequence is derivable from source that can be read today. Measured:

| W5 fix direction | Does W5's ownership check share a mechanism with the W6 note join? | Why, measured |
| --- | --- | --- |
| (iii), ownership stated against the round log | YES | Both resolve a waiver's `(step, increment)` pair against the round log through `round_step_slug` (`src/workflow.rs:119`) and `round_increment_id` (`src/workflow.rs:127`). That predicate already exists, at W3's covering-waiver match, `src/workflow.rs:498-502`. W6's join is that same lookup plus a comparison of the matched rounds' `valid_findings` against the note breakdown. The shared part is one small helper, on the order of five lines. |
| (i), lookup against the declared `[[step.increment]]` set | NO | W5 would key ownership on the plan-declared set and W6 on the round log. Measured, those two sets differ badly: 45 declared ids, 95 distinct round-log identities, 43 in both, 2 declared and never logged, 52 logged and never declared. Keying two checks on two divergent sets is the defect this item exists to fix, reproduced in a second check. |
| (iv), retire or substrate-scope the rule | VACUOUSLY NO | Nothing is left on the TOML path to share. W6 still needs the round-log join, and builds it alone. |
| (ii), rework how a waiver names its unit | ORTHOGONAL | (ii) changes representation across three surfaces. It neither creates nor removes the join. |

So the coupling is real under exactly one of the four candidate directions, and under that direction it is small and already written. What is genuinely open is WHICH DIRECTION, and that is the decision the human owns.

The third participant resolves the same way and the answer is not the one the item implies. The project-identity edit queued by `Q-55-mechanism` adds a `project` field to `Round`. The W6 join needs a `valid_findings` field on the same `Round` (verified absent: `src/metrics.rs:620-651` carries `line`, `task`, `artifact`, `outcome`, `consecutive_clean`, `risk_class`, `step`, `increment`, and nothing else). Those two DO couple, on one struct, and the human's recorded "one deliberate edit rather than a rider on a path fix" constraint binds them. The W5 fix under direction (iii) adds NO field to `Round` and no field to `Waiver`; it passes the `rounds` slice that `run_checks` already holds (`src/workflow.rs:216-219`, where `rounds` goes to `w3_problems` and is simply not handed to `w5_problems`). So the schema constraint does NOT bind the W5 fix under (iii).

That inverts the pass's premise. The item names three participants and asks whether they couple. Measured, the coupling runs between W6 and project identity, and the W5 fix is the SEPARABLE one. The reason to run a design pass at all was that the W5 choice "must be made with W6 in view". It should have been made with W6 in view, and having made it that way the answer is that the W5 fix can ship on its own.

## Per-duty ruling: settled or open

### (a) The coupling ruling: SETTLED, present as a FINDING

Settled by the table above and by the `Round` measurement. All three parts have answers derivable from source. It should reach the human as a finding with its conditional structure shown, not as a question.

Cross-pricing, the part the item calls the deliverable the pass was commissioned to obtain. What W6 costs under the recommended direction (iii):

- `metrics::Round` gains `valid_findings`, and `parse_rounds` (`src/metrics.rs:660-711`) must project it. No log schema change is needed: `check_record` already requires the field in the raw record (`src/metrics.rs:367` and `:454`), so the data is present and only the projection discards it.
- `metrics::Waiver` (`src/metrics.rs:813-841`) gains an optional `note`, `parse_waivers` must project it, and `waivers_from_toml` (`src/workflow.rs:237-267`) must carry `waiver.note` through. The TOML struct already has the field (`src/plan/source.rs:299`).
- A parser for the `<total> (<r1>, <r2>, ...)` shape, out of free human prose.
- The join itself, which is the part direction (iii) donates for free.
- If the JSONL `waiver` record is also to carry a note, that is a log schema addition and it reaches `pack/instrument.md` plus the two drift-guarded generated copies.

Under (i) the cost is the same and the join is not donated. Under (iv) the cost is the same minus the shared helper.

The honest summary of the cross-pricing: direction (iii) makes W6 cheaper by one small helper. That is the whole measured size of the coupling this pass was run to price.

### (b) The authoritative-path ruling: HALF SETTLED BY MEASUREMENT, half a genuine choice

Settled half, measured in scratch and reproducible. The `src/plan/source.rs` per-step membership check is a HARD GATE that no change to W5 can bypass. Four fixture runs:

- Undeclared fold tokens, baseline binary: four problems at exit 1, two from the source path and two from W5. The double lock reproduces exactly as the item records it.
- Declared fold tokens, baseline binary: two problems at exit 1, both W5 ownership refusals. The source-path problem disappears. Opposite verdicts on the same waiver, confirmed.
- Undeclared, direction (iii) binary: two problems at exit 1, BOTH from the source path. W5 now accepts. The source path still refuses.
- Declared, direction (iii) binary: `workflow invariants hold`, exit 0.

The consequence the item does not state: DECLARING THE TWO FOLD TOKENS AS `[[step.increment]]` ENTRIES IS NECESSARY UNDER EVERY DIRECTION that leaves the source path alone. The item frames declaring as escape route 4, "confirmed insufficient". It is insufficient alone and it is also required. Every direction therefore ships a plan edit as well as a source edit, and that belongs in every proposal's edit surface.

Open half. Whether W5 should keep an ownership rule at all, given that the TOML substrate already has a structural membership check at `src/plan/source.rs:807-811`, is a genuine design question and it is the same question direction (iv) raises. It is a choice, and it is the choice this pass should put to the human. See the design space below.

### (c) The direction and its edit surface: GENUINELY OPEN, and it is THE decision

This is the one duty that is a real human choice. The design space is smaller than the item's four candidates, for reasons measured rather than argued (see below).

### (d) The W6 disambiguation: SETTLED as a measurement, present as a FINDING

Measured: `W6` occurs at 14 positions in `docs/plans/agent-scaffold.plan.toml`. Thirteen are inside `Q-70` (lines 1883 to 1901). Exactly one is outside it, at line 1774, and line 1774 falls inside the `[[question]]` whose `id = "Q-59"` begins at line 1770. The item's claim is confirmed exactly.

The ruling: this document means the WAIVER-NOTE BREAKDOWN JOIN wherever it writes "W6", and says so at each use. Recommendation, offered as a finding rather than a choice: do not renumber anything now. Both checks are unbuilt. A collision between two unbuilt checks costs nothing until one ships, and whoever ships the first assigns the number once, with the other check's claim visible. Renumbering now edits the durable record of two decisions to avoid a conflict that may never occur.

The adjacent confusion is separate and stands: `workflow-enforcement-tier-w5` and `workflow-enforcement-tier-w6` are waiver ids continuing an established `-w1` to `-w4` sequence, not check names. The waiver-id convention is established and is not the thing to change.

### (e) The sub-decision ruling: SETTLED BY MEASUREMENT, and sharper than the item states

Reproduce with `jq -r 'select(.type=="decision") | .q_id' docs/metrics/workflow.jsonl | sort -u`, the line-anchored `[[question]]` ids from the plan, and the set difference. Measured in this worktree: 63 distinct receipt ids, 70 registered questions, 41 receipt ids that resolve to no registered question.

ALL 41 are `Q-55-<suffix>`. Not "dominated by", as the item says. Every single one. Zero dangling receipts of any other shape exist.

And `Q-55` itself IS registered, at plan line 1724, `status = "decided"`, `folded_into = "workflow-enforcement-tier"`.

So the ruling is binary and it fully determines what mechanism (2) is worth:

- Rule the `Q-55-<suffix>` ids a DANGLING-RECEIPT DEFECT, and mechanism (2) is red on 41 records, all belonging to one question, and the cure is to register 41 questions the project never intended to register.
- Rule them a LEGITIMATE SUB-DECISION CONVENTION that the check must model, so that a receipt `Q-<n>-<suffix>` resolves when `Q-<n>` is registered, and mechanism (2) IS RED ON NOTHING TODAY, exactly like mechanism (1).

Recommendation: the convention reading. Forty-one instances under one question, written by one mechanism over weeks, is a convention, not forty-one mistakes. Modelling it costs one clause in the detector.

This ruling should be handed to the human as a finding with a recommendation, and the actual detector design belongs to the step that builds it, not to this pass.

### (f) The scope of mechanisms 2 and 3: SETTLED by the same measurement, present as a FINDING

Mechanism (2)'s inputs are exactly W4's: `w4_problems(questions, decisions, baselines)` at `src/workflow.rs:218`, fed from `plan.question_views()` and `metrics::parse_decisions`. Mechanism (3) reads no workflow record at all. Neither touches a waiver, a round record, or the join this pass exists to settle. Verified by reading `run_checks` (`src/workflow.rs:206-221`) rather than taken from the item.

Ruling: BOUNDED, not designed. There is nothing for this pass to co-design with them, because there is no shared mechanism to get wrong. Designing them here would be design done at the wrong moment by the wrong pass, which Minimal by default argues against and which Ground decisions in evidence gives no reason to override.

The bound this pass owes them is exactly three sentences, and they are:

- Mechanism (2) must model the `Q-<n>-<suffix>` sub-decision convention, per (e). Without that clause it is red on 41 records of one question and useful for nothing.
- Mechanism (3) must scope to live passages, excluding a record's own post-mortems and round records, and must have an expected-tool-output escape. Both constraints are confirmed below.
- Mechanism (1) must decide what it does with a waiver that has a note but no breakdown, and with a waiver that has no note. Measured: 25 waivers, 5 carry a note, 4 of those 5 carry a breakdown. The convention it checks covers 4 of 25.

### (g) The YAGNI boundary: owed, and given below.

### (h) The comment-coverage ruling: SETTLED, present as a FINDING

The comment at `src/plan/source.rs:785-790` introduces the block as the `increment`/`evidence` presence rules moved from `check_record`, plus the `reason` to `evidence_tier` pairing, closing "one behaviour, two data representations". The membership check at `:807-811` sits inside the `WaiverUnit::Increment` match arm, written as a third case alongside `None` and empty, so it reads as part of the presence match. It is not a presence rule, and `check_record` could not have performed it, having no access to a step's declared increments.

Verdict: DOCUMENTATION DEFECT, minor, one clause. Not a design divergence, because the check is correct and load-bearing. Measured load-bearing: it is the sole gate that still refuses the undeclared fixture after the W5 ownership rule has been fixed. A block whose one un-documented rule is the one that holds the line is exactly the case where the comment should name it.

This is a finding. It should be fixed in the step, not put to a human as a choice.

## What is treated as settled and is not

This is the half most likely to be skipped, so it was done deliberately. Five items.

### 1. The affected population is 6, not the 5 the item enumerates

Running the item's own reproduction returns six pairs:

```
decision-folder-currency decision-folder-currency-fold
workflow-driver workflow-driver-stage0a
workflow-driver workflow-driver-stage0b
workflow-driver workflow-driver-stage1
workflow-enforcement-tier workflow-enforcement-tier-endproperty-fold
workflow-enforcement-tier workflow-enforcement-tier-fold
```

The item names five of them: the two blockers and the three `workflow-driver` ids. It does not mention `decision-folder-currency` anywhere; a grep over the item's own text returns zero.

The omission matters because the sixth member has a DIFFERENT LATENCY MODE from the ones the item accounts for. The item explains latency by "W3 skips every step that is not `complete`", which is why `workflow-driver` (status `in-progress`) does not fire. But `decision-folder-currency` IS `complete` (plan line 1243). It does not fire for an unrelated reason: its five rounds are `low_risk` and reach a peak `consecutive_clean` of 1 against the 1 that class requires, so the increment converged, no waiver is needed, and W5 is never consulted about it.

A reader who takes the item's two latency modes as exhaustive will conclude that flipping a step to `complete` is what exposes a population member. It is not sufficient. A `complete` step can carry a population member silently for as long as its increments keep converging, and the member becomes a blocker the moment one of them ever needs a waiver. The population is therefore larger and quieter than the item's account implies, and it will keep growing, because nothing prevents the next fold token from being written.

### 2. The item's own `totally-not-a-step` reproduction is falsified by the act of writing it down

The item states that a scratch fixture makes W5 report "increment `totally-not-a-step-inc1` belongs to step `totally-not-a-step`", "while `grep -c 'slug = "totally-not-a-step"'` over that same plan returns 0".

Measured today, on the live plan in this worktree, that grep returns 1. The single hit is plan line 1889, which is the item's own paragraph asserting that the grep returns 0.

The underlying FACT still holds: there is no such Roadmap step, provable with the line-start anchored form `grep -c '^slug = "totally-not-a-step"$'`, which returns 0. The fixture behaviour also reproduces: I built it and both binaries report the substring-derived step name.

This is a live instance of the item's own constraint (3a), the self-quoting record, occurring inside the item's strongest evidence rather than in the ledger. The item scopes (3a) to the ledger and to post-mortems. It reaches plan prose too, and it has already bitten. The cure the item preaches for ledger handles, a line-start anchored grep, is the cure here as well, and the item's stated reproduction should be corrected to use it. That is a defect in `Q-70`, not in the design space, and it is a reviewer's to raise.

### 3. Mechanism (2)'s "red today" status is contingent on an unmade ruling

The item states, as a measured yield fact, that mechanism (1) "is red on nothing today" while mechanism (2) "is red today on the whole unregistered-receipt set", and uses that contrast to warn that the buildability order runs opposite to the yield order.

Measured, mechanism (2)'s redness depends entirely on ruling (e), which the item explicitly leaves to the pass. Under the convention reading, which this document recommends and which the 41-of-41 concentration supports, mechanism (2) is red on nothing today, exactly like mechanism (1). So the yield axis the item presents as measured is conditional on a ruling nobody has made, and under the natural ruling the contrast disappears.

The part of the yield claim that survives is the part about mechanism (3): it is the only one of the three that reaches a class no gate in this repository catches. That stands, and it is confirmed independently below.

### 4. Direction (iii) narrows what a waiver may cover, which the item does not name

The item prices direction (iii) as needing "no new data source and no type change". True, and confirmed: the change is one signature, one call site, one rule body. But it is not behaviour-neutral. Under the lexical rule, an increment-unit waiver whose increment has NO round records at all passes W5 silently. Under direction (iii), it is reported.

Measured on the live plan: no regression. The direction (iii) binary run against the unmodified plan and log returns `workflow invariants hold` at exit 0, identical to the baseline binary, so all 13 increment-unit waivers currently in the plan do join to round records. The three `review-skipped` increment waivers, the class most likely to cover unreviewed work, name `convergence-accounting`, `pack-rebuild-tracking` and `user-prompts-dir`, each of which has exactly 1 round record.

I judge the narrowing an IMPROVEMENT, because a waiver that covers an increment with no round records grants nothing in W3 anyway (W3's increment loop is built from the records, so an increment with none never enters it) and reporting it turns a dead waiver into a visible one, which is Make illegal states unrepresentable applied to a waiver. But it is a behaviour change and a proposal that does not name it has under-priced its own direction.

### 5. Declaring is necessary, not merely insufficient

Covered under (b). Repeated here because it is the fact most likely to be lost: every direction that leaves `src/plan/source.rs` alone must also declare the two fold tokens in the plan. Measured at exit 1 with the direction (iii) binary against the undeclared fixture.

## Confirmations of item claims, taken first-hand

Recorded so the pass does not re-derive them, and because a lens that only contradicts is as biased as one that only agrees.

- The two fold tokens each carry exactly 5 `type:"round"` records, every one with structured `step` `workflow-enforcement-tier` and no structured `increment`; peak `consecutive_clean` is 1 for the plan fold and 0 for the endproperty fold, against the 2 the `risky` class requires. Confirmed.
- Each fold token has a `type:"escalation"` record whose `task` equals it and whose `human_decision` is `decision`, with no structured `increment`, so W5's record-backed evidence join resolves through the `task` fallback and passes. Confirmed, and confirmed end to end by the declared fixture going fully green.
- Every waiver-note breakdown currently agrees with the round records. Confirmed, 4 of 4: `workflow-enforcement-tier-inc1` note `(3, 4, 6)` against records 3,4,6; `-inc2` `(9, 5, 6, 4)` against 9,5,6,4; `-inc3` `(6, 4, 2, 0, 2)` against 6,4,2,0,2; `-inc4` `(11, 9, 6, 4, 5)` against 11,9,6,4,5. The fifth note, on `prompt-drift-guard-w1`, carries no breakdown but states "Six review rounds, 15 valid findings" against 6 records summing to 15, which also agrees. So mechanism (1) is a regression guard on a convention held correctly by hand, on all five notes that exist.
- `blocked_by = []` on all 95 steps with none populated. Confirmed with a line-start anchored grep: 95 `[[step]]` blocks, 95 `^blocked_by = \[\]$` lines. A naive unanchored grep returns one apparent exception, which is prose inside `Q-51`'s `ask`. The item is right and the naive measurement is the wrong one, which is the item's own lesson applied to the item's own claim.
- The declared `[[step.increment]]` set is not a model of the identities the checks operate on: 45 declared, 95 logged, 43 shared, 2 declared-and-never-logged, 52 logged-and-never-declared. Confirmed, and the drift is worse than the item's prose suggests.
- `round-log-core-incA` and `round-log-core-incB` are the two logged identities containing an uppercase byte, so they cannot be declared at all under `is_kebab_case_token`. Confirmed by enumeration, and they are the only two.
- The W5 ownership clause is stated verbatim in exactly three files. A tree-wide grep for the phrase "must own its `increment` (the increment's leading slug equals the step)" returns `AGENTS.md`, `.agents/AGENTS.reference.md` and `pack/instrument.md`, and no others. Confirmed.
- Mechanism (3)'s red list is all of the `src/checks.rs` citations, not a subset. Fifteen distinct citations exist in `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md`. Spot-checked six against the current source: `:78` is cited as the `RUNNER_PREFIX` constant and is an import line reading `PathBuf,`; `:1462` and `:1491` are cited as hand-built fixture names and are both `init_repo(&dir);`; `:329-342` is cited for the collision window and is `impl From<io::Error> for RunError`; `:845-847` is cited for `nanos()` and is a `Command::new("sh")` construction. Every one checked was stale. The item's correction of the recorded "about eleven" is sound on this sample, and the count is 15.

## The design space

Four candidates are named by the item. Measurement collapses two of them, which is the point of this lens.

**(i) Lookup against the step's declared `[[step.increment]]` set. NOT VIABLE, on measurement rather than taste.** It keys W5 on a hand-maintained set that shares 43 of its 45 members with a 95-member round-log population, so 52 identities the checks actually operate on are invisible to it. It requires widening `plan::Step`, which is `Serialize` and is the `status --json` payload (`src/main.rs:582-585`), so it changes a machine output contract to fix a validator bug. The Markdown substrate declares no increments at all, so the rule would be unenforceable there. And it would place a second copy of a membership check that `src/plan/source.rs:807-811` already performs correctly.

**(ii) Rework how a waiver names its unit. NOT THE FIX, on scope rather than merit.** It reaches three representations and does not itself resolve the ownership question. It may be right work; it is not this work.

**(iii) State the ownership rule against the round log, keyed on `round_increment_id` plus the step, exactly as W3's covering-waiver match already is. VIABLE, and proved.** Implemented in a scratch build and measured across four cells, tabulated below.

**(iv) Retire the W5 ownership rule, or scope it to the substrate where its premise holds, leaving the structural membership check at `src/plan/source.rs:807` as the TOML-path authority. VIABLE.** The premise really does fail on this project's substrate: `waivers_from_toml` sets `step: step.slug.clone()` (`src/workflow.rs:258`) and the TOML `Waiver` struct is `deny_unknown_fields` with no `step` field (`src/plan/source.rs:279-300`), so the mis-scoped state the rule reports cannot be authored.

### The four-cell experiment

| Binary | Plan | Result |
| --- | --- | --- |
| baseline | live worktree plan | `workflow invariants hold`, exit 0 (control) |
| direction (iii) | live worktree plan | `workflow invariants hold`, exit 0 (no regression on 25 waivers) |
| direction (iii) | fold tokens undeclared, waivers written, step `complete` | exit 1, two problems, both from `src/plan/source.rs` |
| direction (iii) | fold tokens declared, waivers written, step `complete` | `workflow invariants hold`, exit 0 |

The patch under test, in full:

```rust
let owned = rounds.iter().any(|round| {
    round_increment_id(round) == increment && round_step_slug(round) == waiver.step
});
```

plus passing `rounds` into `w5_problems`, replacing `leading_slug(increment) != waiver.step`.

### Trade-offs against the numbered Project Principles, by name

**Direction (iii).**

- Prefer the cleaner long-term architecture over the smallest diff: strongly favoured. It makes W5 and W3 state the same relation the same way, through the same two accessors, and it retires the last lexical join in `src/workflow.rs`. The `leading_slug` doc at `src/workflow.rs:83-87` already calls the shim transitional and says it "remains only for pre-migration records"; W5's evidence check thirty lines below its ownership check is already structured (`:594-596`). One function currently holds one migrated check and one unmigrated one. This finishes the migration the code says it wants.
- Ground decisions in evidence: satisfied in the strongest available form. This is not an argued direction, it is a built one, measured green on the live plan and green on the target fixture.
- Structured data first, project for humans: favoured. The round log is the structured event record of what actually happened; the declared increment list is hand-authored prose about it. Grounding ownership in the former is this principle's own preference, and the 52-versus-43 drift is what hand-authored prose diverging from structured data looks like.
- Make illegal states unrepresentable: partly favoured. It removes the state where W5 asserts a step that does not exist. It does not remove the underlying representability, since a waiver can still name any string; it converts an unfounded assertion into an honest "no record joins this".
- Minimal by default: neutral to slightly favoured. One signature, one call site, one rule body, 14 in-crate test call sites that each need a `rounds` argument. No struct changes, no machine output contract change, no second substrate's projection touched, no substrate conditional.
- Idempotent, Safe on existing projects, Reproducible: not engaged. No scaffolding behaviour changes.

**Direction (iv).**

- Minimal by default: favoured, since it deletes rather than adds.
- Prefer the cleaner long-term architecture over the smallest diff: opposed on one specific ground. `run_checks` is deliberately one implementation across both substrates, stated at `src/workflow.rs:196-200` as AGENTS.md's Principle 16. A substrate-conditional rule is the first crack in that. The alternative form, retiring the rule outright, leaves the JSONL substrate with no ownership check at all, on a path where the mis-scoped state IS authorable.
- Ground decisions in evidence: satisfied. The premise failure is measured, not asserted.
- Make illegal states unrepresentable: opposed. Retiring a rule because its state cannot currently be authored on one substrate encodes an accident of the current schema as a permanent guarantee.

**Direction (i).** Opposed by Prefer the cleaner long-term architecture over the smallest diff (a second copy of an existing check, a machine output contract widened to fix a validator bug), by Structured data first, project for humans (it grounds the check in the hand-authored set rather than the structured record), and by Ground decisions in evidence (the 45-versus-95 measurement is direct evidence against it). Not recommended.

## Recommendation

**Take direction (iii), and declare the two fold tokens as `[[step.increment]]` entries in the same change.** Proved green end to end.

**Do not run a deliberation over the coupling question.** Answer it. Give the human the conditional table, the `Round` measurement, and the one sentence that follows from them: under direction (iii) the W5 fix and the W6 join share one small lookup that already exists, and the W5 fix is independent of the queued project-identity schema edit, so it can ship first and alone. Then ask the human the ONE question the evidence does not answer.

**The one question to put through the human-input contract**, framed as it actually is:

> W5's increment-ownership rule is broken. Fix it against the round log (direction (iii), proved green), or retire it and let the plan-side structural membership check at `src/plan/source.rs:807` own waiver ownership on the TOML substrate (direction (iv))? Both unblock the two owed waivers. (iii) keeps one implementation across both substrates and finishes a migration the code documents as in progress. (iv) is smaller and removes a rule whose premise fails on this project's own substrate, at the cost of either a substrate conditional or leaving the JSONL path unchecked.

Recommended: (iii).

**Scope mechanisms 2 and 3 as BOUNDED, not designed**, per (f), and carry the three bounds listed there into the step.

**Take the following as findings rather than choices**: (a) the coupling answer, (b)'s measured half, (d) the W6 disambiguation and the do-not-renumber recommendation, (e) the sub-decision convention reading, (h) the comment defect, and the five items under "what is treated as settled and is not". A human should be told these. A human should not be asked about them.

Of the eight lettered duties, ONE is a genuine human choice, and it is a two-option choice. That is the honest size of this pass.

## What not to build (the YAGNI boundary)

- Do NOT build the W6 waiver-note join in this change. It is red on nothing: all four breakdowns and the fifth note's total agree with the records today. It needs two struct widenings and a prose parser, and it would arrive as a rider on a validator path fix, which is the shape the human's recorded constraint on the record schema exists to prevent.
- Do NOT build the dangling-receipt detector or the quotation resolver in this change. Neither reads a waiver, a round record, or the join. Bound them, per (f).
- Do NOT widen `Round`. Not with `valid_findings`, not with `project`. The recommended direction needs neither, and the record schema is owed one deliberate edit when W6 and project identity are actually built together.
- Do NOT widen `plan::Step`. It is a machine output contract and no viable direction needs it.
- Do NOT change the waiver schema, in TOML or in JSONL. Direction (iii) needs no new waiver field.
- Do NOT renumber either W6. Both are unbuilt; the collision costs nothing until one ships.
- Do NOT rewrite, re-key or re-model the append-only log. Already ruled out and re-confirmed.
- Do NOT register 41 `Q-55-<suffix>` questions. Under the recommended reading of (e) they are a convention, and the detector models it instead.
- Do NOT rename any already-declared increment id. Direction (iii) makes the `workflow-driver-stage0a`/`-stage0b`/`-stage1` case join correctly as they stand, since their round records carry structured `step` and `increment` ids.
- Do NOT touch `src/next.rs`. Its declared-increment parity property (`src/next.rs:517-523`) is pushed against only by direction (i), which is not recommended.

## Edit surface implied by the recommendation

Source, `src/workflow.rs` only:

- `w5_problems` signature gains `rounds: &[Round]`.
- `run_checks` (`:219`) passes `rounds` through. It already holds them (`:210`, `:217`).
- The ownership rule body at `:562-574` is replaced by the round-log lookup, and its problem message stops naming a derived step.
- The `w5_problems` doc comment (`:519-543`) restates the rule.
- 14 in-crate test call sites in `src/workflow.rs`'s own `mod tests` (lines 1238, 1257, 1278, 1292, 1301, 1309, 1330, 1374, 1393, 1406, 1426, 1450, 1619, 1632) each gain a rounds argument. No file under `tests/` calls `w5_problems`; the whole test surface is in-crate.
- New tests for the changed rule, including the no-joining-record case.

Plan, `docs/plans/agent-scaffold.plan.toml`:

- Two `[[step.increment]]` entries under `workflow-enforcement-tier` for `workflow-enforcement-tier-fold` and `workflow-enforcement-tier-endproperty-fold`. REQUIRED, not optional, per (b).
- The two owed `[[step.waiver]]` entries, `-w5` and `-w6`.
- `workflow-enforcement-tier` status to `complete`.

Drift-guarded generated files. YES, three of them, and they must move together:

- `pack/instrument.md:11`, the pack source, carries the clause "an `increment`-unit waiver's `step` must own its `increment` (the increment's leading slug equals the step)".
- `AGENTS.md:147` and `.agents/AGENTS.reference.md:147` carry it verbatim. The guard is `src/agents_md_drift.rs`, and the test that fails on a partial edit is `the_committed_scaffold_matches_a_fresh_render`.
- Regeneration is `just scaffold-self`. WARNING: its second line runs `nix fmt` over the whole tree, and this repository is not formatter-clean at HEAD, so a naive regeneration reformats roughly 22 unrelated files. The step must handle that deliberately.

Generated consts: none involved. Golden fixtures: none involved. `render --check` is engaged only if `waiver_note` (`src/plan/render.rs:516-530`) changes, which the recommendation does not require; adding the two waivers does change the generated `<task>.md` output, so `render` must be re-run for the plan edit.

Machine output contracts: none changed. `status --json` (`PlanProjection`, `src/main.rs:582-585`) is untouched, because `plan::Step` is untouched.

## Commands run, for reproduction

All were run against `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/exp-vc-evidence` at `736d526`.

```
git -C <tree> rev-parse --abbrev-ref HEAD                       # explore/vc-evidence
git merge-base --is-ancestor 736d526 main                       # yes

jq -r 'select(.type=="round") | [(.step // (.task|sub("-inc[a-zA-Z0-9]+$";""))), (.increment // .task)] | join(" ")' \
  docs/metrics/workflow.jsonl | sort -u \
  | awk '{lead=$2; sub(/-inc[a-zA-Z0-9]+$/,"",lead); if (lead != $1) print $1, $2}'   # 6 pairs

jq -r 'select(.type=="decision") | .q_id' docs/metrics/workflow.jsonl | sort -u        # 63
grep -oE '^id = "Q-[^"]+"' docs/plans/agent-scaffold.plan.toml | sort -u               # 70
comm -23 receipts registered                                                           # 41, all Q-55-*
grep -c '^id = "Q-55"$' docs/plans/agent-scaffold.plan.toml                            # 1

grep -onE "[(][0-9]+(, [0-9]+){1,6}[)]" docs/plans/agent-scaffold.plan.toml            # 4
grep -cE '^\[\[step\.waiver\]\]' docs/plans/agent-scaffold.plan.toml                   # 25
grep -c 'note = ' docs/plans/agent-scaffold.plan.toml                                  # 5
jq -r --arg t <increment> 'select(.type=="round" and ((.increment // .task)==$t)) | .valid_findings' \
  docs/metrics/workflow.jsonl                                                          # per-increment

grep -on 'W6' docs/plans/agent-scaffold.plan.toml                                      # 14, one outside Q-70, at 1774 (Q-59)
grep -c 'slug = "totally-not-a-step"' docs/plans/agent-scaffold.plan.toml              # 1 (the item's own sentence)
grep -c '^slug = "totally-not-a-step"$' docs/plans/agent-scaffold.plan.toml            # 0
grep -c '^\[\[step\]\]$' docs/plans/agent-scaffold.plan.toml                           # 95
grep -c '^blocked_by = \[\]$' docs/plans/agent-scaffold.plan.toml                      # 95

grep -oE "src/checks[.]rs:[0-9]+(-[0-9]+)?" \
  docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md | sort -u   # 15
grep -rln "must own its \`increment\` (the increment's leading slug equals the step)" . # 3 files
grep -rn "w5_problems" src tests --include=*.rs                                        # 14 test call sites

cargo build --release                    # in a scratch copy, baseline and patched binaries
<binary> validate --workflow --source <plan> --metrics <log>   # the four-cell experiment
```

Fixtures were built by copying `docs/` into scratch and editing the copy. No file outside the session scratchpad was created or deleted, and no fixture used restrictive permissions.
