# Q-70 capture, round 3 triage

Triager verdicts on the nine raw findings in `q70-capture-r3-reviewer-refute.md` (`R3B-*`), `q70-capture-r3-reviewer-quotes.md` (`R3A-*`) and `q70-capture-r3-reviewer-gates.md` (`R3C-*`).

Artifact: `git diff main..HEAD` on `triage/q70-r3`, commits `c344ca5`, `dda6ae3`, `4e176a1` and `896b053` (the same four commits the reviewers reviewed as `2c2be88`, `58b677f`, `198556e` and `61b1d35`; the branch was rebased between the spawns and the content is identical). The change adds the `[[question]] Q-70` entry at `docs/plans/agent-scaffold.plan.toml:1880-1903`, the `q70-capture` `[meta].orphan_tasks` token, an empty `docs/plans/agent-scaffold.questions/Q-70.md`, and the regenerated `docs/plans/agent-scaffold.md`.

TREE MEASURED: every command below ran against `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/tri-q70-r3`, or against a fixture copied from it. Binary: `target/debug/agent-scaffold` built from that worktree at HEAD. Fixture root `<S>` for `<scratchpad>/tri-q70r3/`. Nothing outside `<S>` was written or deleted; the only mode-600 files created were cargo's own incremental lock files inside `<S>/target-mut4`, and they were chmodded to 644 before finishing.

RESULT: EIGHT VALID, ONE DISMISSED, NO DUPLICATES. Severity ceiling `high`, two of them.

BOTH `high` FINDINGS ARE VALID. No finding at `high` or above is dismissed, so NO BACKSTOP RE-CHECK IS OWED for this round. The one dismissal (`R3B-5`) the reviewer rated `medium` and I rate `low` had it been valid, which is below the backstop severity either way.

Two severities are corrected DOWNWARD (`R3B-1` `high` -> `medium`, `R3B-3` `medium` -> `low`). A downgrade is not a dismissal and engages no backstop: both findings stand as valid.

NO VALID FINDING IS OUT OF SCOPE. I checked the precedent's four conditions on every one; condition 1 (provenance predates the base commit) fails for all eight, because each defect is in text authored by one of the four reviewed commits. The provenance of the two I could most plausibly have inherited is recorded per finding (`R3A-1` -> `896b053`, `R3A-2` -> `4e176a1`).

---

## Verdict table

| id | verdict | reviewer severity | my severity | settled by |
| --- | --- | --- | --- | --- |
| R3B-1 | VALID | high | medium | running plus reading |
| R3B-2 | VALID | high | high | running |
| R3B-3 | VALID | medium | low | running |
| R3B-4 | VALID | high | high | running, including a source mutation |
| R3B-5 | DISMISSED | medium | low if valid | running |
| R3A-1 | VALID | medium | medium | running |
| R3A-2 | VALID | low | low | reading |
| R3C-1 | VALID | medium | medium | running |
| R3C-2 | VALID | low | low | running |

---

## The re-raise check, run before any verdict

The refute lens claims no prior lens covered its territory and offers a token measurement for it. I VERIFIED THE CLAIM RATHER THAN ACCEPTING IT, over the eight prior findings files in `docs/plans/agent-scaffold.reviews/`:

```
$ for t in shim pre-migration "Inc 2" valid_findings; do grep -cF "$t" <the eight round 1 and round 2 files>; done
shim            0 in all eight
pre-migration   0 in all eight
Inc 2           0 in all eight
valid_findings  1 in q70-capture-reviewer-premises.md, 1 in q70-capture-reviewer-source.md,
                1 in q70-capture-r2-reviewer-surfaces.md, 0 in the other five
```

The three tokens are absent as claimed. The `valid_findings` claim is hedged ("as a schema question") and the hedge is correct: I opened all three occurrences and none raises the schema question. One is a `printf` fixture line, one is a `grep` pattern about orphan tasks, and one is a verbatim quotation of the item's own mechanism (1) sentence. No prior finding asks whether the projection the checks consume carries the field. `R3B-2` is therefore new, not a re-raise of `R2B-1`.

I also held both prior triage files throughout. No round 1 or round 2 dismissal or duplicate is re-adjudicated below, and none of this round's findings brings evidence that a settled verdict was wrong.

---

## Per-finding verdicts

### R3B-1. The codebase states a direction for this join class, and the item records neither the statement nor W5's own half-migration

VERDICT: VALID. SEVERITY: `medium`, CORRECTED DOWN from `high`.

REPRODUCED, all four evidence limbs.

1. `leading_slug`'s doc closes at `src/workflow.rs:83-87` with, byte-exact: "Inc 2 retires this risk for NEW data: a `round`/`escalation` record may carry a structured `step`/`increment` id, and `round_step_slug`/`escalation_step_slug` (and their increment counterparts) prefer it, so a record with the field joins without ever reaching this lexical strip. This shim remains only for pre-migration records that omit the structured id."
2. The asymmetry inside `w5_problems` is real and I opened both halves. The ownership check is lexical, `if leading_slug(increment) != waiver.step` at `:564`. The record-backed evidence check is structured, at `:594-596` through `escalation_increment_id` and `escalation_step_slug`, under its own comment at `:590-592`: "Tie the joined escalation to the unit the waiver exempts, preferring the escalation's structured ids (Inc 2) over the `leading_slug`/`task` shim when it carries them". One function, two checks, one migrated and one not.
3. The item reaches neither. Term census over `plan.toml:1880-1904`: `shim` 0, `pre-migration` 0, `Inc 2` 0. Its fifteen distinct `src/workflow.rs` citations are `:64-68`, `:88`, `:119`, `:127`, `:141`, `:206-221`, `:237-267`, `:258`, `:321`, `:445-447`, `:450`, `:498-502`, `:549`, `:553`, `:564`. `:83-87` is not among them; `:64-68` is the `INCREMENT_MARKER` doc about the `-incA` / `-incB` run, which is a different doc block.
4. `leading_slug` is documented as "The leading step slug of a `task`" at `:71`, and W5 applies it to `waiver.increment`, a plan-authored id that is not a `task`.

VALID. This is the class round 1 ruled valid twice (`R1C-2`, `R1C-3`): a code fact about the check the pass is commissioned to fix, which bears on the comparison the item hands the pass as open, and which the item does not carry. Recording it is not recommending a direction, and the item already has the form for that ("THIS IS RECORDED, NOT RECOMMENDED").

SEVERITY CORRECTED DOWN to `medium`, and I give the reasoning because the correction is mine and not the reviewer's. The finding's impact statement is that an explorer "weighs three directions without knowing that the code states one of them as the migration in progress". But the item ALREADY names direction (iii) and already attributes it to the durable record's own statement of the fix ("teaching W5 the structured step association W3 already uses"). So the omitted fact does not reveal an option the explorer lacks; it changes how heavily one already-named option should weigh. That is materially less reach than `R1C-3`, where the excluded direction was foreclosed by a mandatory binary, or than `R3B-4` below, where the direction is named nowhere at all. Two further things hold it at `medium` rather than `high`. The doc's retirement sentence is scoped to RECORDS carrying structured ids, and `waiver.increment` is not a record field, so the fact bears on the axis without settling it; the reviewer concedes this in its own evidence limb 4. And a reader following the item's own citation to `src/workflow.rs:88` lands on the line immediately below the retirement sentence, so the fact is one screen from where the item already sends them. It is a genuine omission with a bounded consequence, which is `medium` on this loop's own calibration (`R1B-1`, `R1C-1`, both corrected to `medium` for the same reason).

### R3B-2. The prospective W6 join needs a second field on the record struct a receipted decision already constrains, and the item prices it at zero

VERDICT: VALID. SEVERITY: `high`, confirmed.

EVERY MEASURED LIMB REPRODUCES, in this worktree.

1. `Round` (`src/metrics.rs:620-651`) carries exactly `line`, `task`, `artifact`, `outcome`, `consecutive_clean`, `risk_class`, `step`, `increment`. No `valid_findings`. I read the struct open to close.
2. `parse_rounds` (`src/metrics.rs:660-711`) reads `task`, `artifact`, `outcome`, `consecutive_clean`, `risk_class`, `step`, `increment` and nothing else. `valid_findings` is required by `check_record` (`src/metrics.rs:367`, `:454`) and is dropped by the projection.
3. `grep -c serde_json src/workflow.rs` returns 0 and `grep -c valid_findings src/workflow.rs` returns 0. `workflow.rs` owns no JSON parsing; both entry points funnel through `metrics::parse_rounds`.
4. The only waiver-to-rounds join in the tool is W3's covering-waiver match at `src/workflow.rs:498-502`. I confirmed it structurally rather than by search: of the three check functions, `w3_problems` (`:437`) is the only one whose signature takes both `rounds` and `waivers`; `w4_problems` (`:309`) takes neither waivers nor rounds' findings, and `w5_problems` (`:544`) takes `waivers`, `steps` and `escalations` and no rounds.
5. The waiver `note` is read by exactly one non-test site in the tool, `src/plan/render.rs:527`, which writes it into the generated Markdown. No check reads it.

THE ONE ARGUED STEP, judged as argued. The reviewer marks it and states its falsifier ("anyone who can name a second waiver-to-rounds join, or a `valid_findings` reader inside `workflow.rs`, refutes it"). I ran both halves of that falsifier and neither exists. I record one qualification the finding overstates slightly: the widening does not STRICTLY have to land on `Round` itself, since a second projection type could carry `valid_findings` alongside it. That does not change the finding, because either form edits the projection layer through which every check reads the record schema, which is the layer `Q-55-mechanism`'s recorded constraint is about. The item's sentence, "the convention already exists, carried in the `note` field of `[[step.waiver]]` entries, and only the enforcement is missing", is what the measurement contradicts, and it contradicts it either way.

`high` CONFIRMED. This is the same class and reach as `R2B-1`, which round 2 held at `high`: a fact about the record schema, under a recorded human constraint, that changes what the pass is told to weigh on letter (a), the one deliverable the ledger records as the human's ground for commissioning a pass at all. The item registers, correctly, that `Q-55-mechanism` queued a `project` field on `Round` and that the human's reasoning requires "ONE DELIBERATE EDIT RATHER THAN A RIDER ON A PATH FIX"; it then names a second participant that needs a second field on the same struct and prices it as enforcement over data already in hand. A proposal ruling "they do not couple" on that basis rules on a zero. It is not `critical` because nothing is asserted falsely: the note convention does exist and no check does enforce it.

### R3B-3. Two of the three mechanisms have no data surface in common with the question the pass exists to settle

VERDICT: VALID. SEVERITY: `low`, CORRECTED DOWN from `medium`.

REPRODUCED. Mechanism (2)'s inputs are exactly W4's: `run_checks` calls `w4_problems(questions, decisions, baselines)`, fed from `plan.question_views()` and `metrics::parse_decisions` (`src/workflow.rs:180-195`, `:206-221`). It reads no waiver and no round. Mechanism (3) resolves `src/checks.rs:<line>` citations in a step sidecar against source content and reads no workflow record at all. The deciding record's ground reproduces at the line, and its three limbs are the unmeasured W5/W6 coupling, the W5 direction choice made with W6 in view, and the exclusion rule for the already-diagnosed defects; none reaches mechanisms (2) or (3). The next-action paragraph scopes explorer output as quoted: "(2) EXPLORERS write to that directory, each ruling on the W5/W6 coupling and carrying an explicit 'what not to build' boundary."

VALID, on a narrow reading and not on the reviewer's wider one. What is valid is that the item's own standard is to register the measured inputs a ruling needs, letter (f) commissions a ruling, and the measurement that bears on it exists and is not in the item. What I do NOT accept is the framing that mechanisms (2) and (3) lack a ground for being in the pass: they are in the pass's SCOPE by the human's recorded decision, which `Q-55-entryroute` settles and the item relays correctly, and the scope rules put "the item fails to decide" out of bounds. The remedy is therefore scoped to recording the measured data surfaces, explicitly NOT to ruling letter (f).

SEVERITY CORRECTED DOWN to `low`. Nothing is asserted falsely; the item states the mechanisms are in scope, which is true, and says it has never ruled designed-versus-bounded, which is also true. The reviewer's own impact statement is that the failure "wastes pass effort rather than producing a wrong ruling", and that an explorer who opens either mechanism's data reaches the same conclusion in one step. That is the calibration round 2 used for `R2B-4` and `R2B-6`, both `low`: an omission whose consequence is recoverable by the next reader. It is not `medium`, which this loop has reserved for a claim that measurement contradicts (`R2A-3`) or an inventory a later author relies on (`R2B-2`).

### R3B-4. The ownership rule's premise is unrepresentable on the substrate this project uses, and the item never raises the question

VERDICT: VALID. SEVERITY: `high`, confirmed.

THE FIXTURE REPRODUCES BYTE-FOR-BYTE, rebuilt from scratch in `<S>/fake-owner` (a fresh `cp -r docs/` with one `[[step.waiver]]` added under `workflow-enforcement-tier`, `id = "workflow-enforcement-tier-wX"`, `unit = "increment"`, `increment = "totally-not-a-step-inc1"`, `reason = "review-skipped"`, `evidence_tier = "self-declared"`):

```
$ agent-scaffold validate --source <S>/fake-owner/docs/plans/agent-scaffold.plan.toml \
    --metrics <S>/fake-owner/docs/metrics/workflow.jsonl --workflow
<PLAN>: waiver `workflow-enforcement-tier-wX` on step `workflow-enforcement-tier` names increment `totally-not-a-step-inc1`, which is not one of the step's increments
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-wX`: increment waiver names step `workflow-enforcement-tier` but increment `totally-not-a-step-inc1` belongs to step `totally-not-a-step`
EXIT=1
$ grep -c 'slug = "totally-not-a-step"' <S>/fake-owner/docs/plans/agent-scaffold.plan.toml
0
```

W5 asserts that the increment belongs to a step the plan does not contain, and its own real-step rule stays silent because the waiver's `step` was inherited and is real.

THE STRUCTURAL LIMBS REPRODUCE TOO, each opened at the line. `primary = "toml"` is at `plan.toml:4`. `waivers_from_toml` sets `step: step.slug.clone()` at `src/workflow.rs:258`, and the TOML `Waiver` struct (`src/plan/source.rs:279-300`) is `#[serde(deny_unknown_fields)]` with no `step` field; its own doc says so, "the fields mirror that record (minus `task`/`step`, which the nesting supplies)". `check_workflow_toml` (`src/workflow.rs:180-195`) derives both `plan.step_views()` and `waivers_from_toml(plan)` from the same `plan.steps`, so `waiver.step` is always a member of `slugs` and W5's first rule at `:553` cannot fire on that path.

I WENT FURTHER THAN THE REVIEWER, because the finding's strongest claim is that this direction plus the declaration is a COMPLETE unblocking of the two owed waivers, and that is testable rather than arguable. In a scratch copy of the tracked tree (`<S>/mut4`, `git archive HEAD | tar -x`, its own `CARGO_TARGET_DIR`), I disabled W5's increment-ownership rule alone and rebuilt. The fixture `<S>/unblock` is a fresh `cp -r docs/` with the two fold tokens declared as `[[step.increment]]` entries, both owed waivers written (`-w5` and `-w6`, `accepted-at-escalation` / `record-backed`), and `workflow-enforcement-tier` flipped to `complete`:

```
$ <worktree binary, unmutated> validate --source <S>/unblock/... --workflow
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w6`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-endproperty-fold` belongs to step `workflow-enforcement-tier-endproperty-fold`
EXIT=1

$ <S>/target-mut4/debug/agent-scaffold validate --source <S>/unblock/... --workflow
<LOG>: 310 records, valid
<PLAN>: 95 steps, 70 questions, valid
<PLAN> vs <LOG>: workflow invariants hold
EXIT=0
```

So on the live plan, with the step `complete`, the ownership rule is the ONLY thing that fails: W3's covering-waiver match accepts both waivers, W5's record-backed evidence join resolves through the `task` fallback and passes, and the `src/plan/source.rs` membership check is satisfied by the declaration. The item's "ONLY THE OWNERSHIP CHECK BLOCKS THEM" is now confirmed by measurement rather than by reading, and so is the fourth direction's claim to be a complete unblocking.

DEFECT OR LEGITIMATE BOUNDARY. A DEFECT, and I ruled it against the strongest version of the boundary argument. The boundary argument is that the item labels its directions "NOT a decided option set, NOT exhaustive", so omitting a fourth is not a defect. That argument fails on two grounds. First, the finding's core is not "a fourth option is missing from a non-exhaustive list"; it is that the item omits measured facts about the rule the pass exists to fix, and those facts bear directly on a ruling the item itself commissions. Letter (b) asks which of W5 and the per-step membership check is authoritative, and the item frames the two as symmetric ("THE TWO PATHS RETURN OPPOSITE VERDICTS ON THE SAME WAIVER"). They are not symmetric: one consults the step's declared increment set structurally, the other consults a substring and names a step that need not exist, on a substrate where the violation it exists to report cannot be authored. That asymmetry is the fact that decides letter (b), and it is absent. Second, the item's own convention already answers the objection: it records direction (iii) explicitly and marks it "THIS IS RECORDED, NOT RECOMMENDED", so recording a direction without steering is a form the item has and uses.

`high` CONFIRMED. Absolute impact if unfixed: the pass weighs three ways to teach a check a better lookup on the substrate where the check has no violation to catch, and the step authored from the winning proposal widens a shared serialised type (direction (i)) or reworks three waiver representations (direction (ii)) where a scoping change would have done. That is the wrong end of Project Principle 2, "Minimal by default", and it lands on the highest-blast-radius surface this tool has. It is the same class as `R1C-3`, which round 1 held at `high`, and stronger in one respect: `R1C-3`'s excluded direction was already named in the durable record, so an explorer could stumble on it; this one is named nowhere. I record the counterweight the reviewer records honestly and that keeps this off `critical`: `run_checks` is deliberately one implementation across both substrates (`src/workflow.rs:196-200`), so a substrate-conditional rule pushes against a real parity property, and the JSONL substrate keeps the rule meaningful. That is the content of the ruling, not a reason to omit the option.

### R3B-5. The item applies its moving-target cure to numbers and not to the record it restates

VERDICT: DISMISSED. Severity had it been valid: `low`. The reviewer rated it `medium`; the backstop covers dismissals at `high` or above, so none is engaged on either rating.

THE MEASUREMENT LARGELY REPRODUCES, AND I DISMISS ON THE DEFECT, NOT ON THE EVIDENCE. The classification of the twenty valid findings is defensible and I checked it against both triage files: `R1A-1`, `R1A-4`, `R1B-1`, `R2A-2`, `R2A-4`, `R2B-1`, `R2B-2`, `R2B-5` and `R2B-6` are all defects in content the item copies from, or points into, the ledger and the receipts. Nine of twenty. The finding's own arithmetic is wrong in one place, which I record because this loop's standing caution is exactly about that: it says "In round 2 alone that is five of nine" and then lists SIX round 2 ids. Six of nine is the count from its own list, and its total of nine is right.

GROUND FOR DISMISSAL, IN FOUR PARTS.

First, IT NAMES NO DEFECTIVE SENTENCE AT HEAD. Every one of the nine instances it classifies has already been fixed by a settled remedy. This round's own quotation lens checked 44 resolvable quotations and 42 citations exhaustively; 43 of 44 quotations resolve and every citation resolves and says what is claimed. The one failure is `R3A-1`, which I rule valid separately. A finding whose evidence is entirely a classification of already-fixed defects, with no live instance, is a diagnosis rather than a defect.

Second, ITS ACTIONABLE CORE IS `R3A-1`'S CLASS AND NOTHING IS LOST BY DISMISSING IT. The narrow rule the finding can ground is: do not assert a measured property of content that keeps moving. That is precisely what `R3A-1` catches, and I have scoped remedy L to that class over the whole item rather than to the one clause, so the cure the finding asks for is prescribed.

Third, ITS STRUCTURAL PRESCRIPTION WOULD UNDO SETTLED REMEDIES WITHOUT NEW EVIDENCE. The item restates ledger and receipt content because remedies G, H and J required it: register the `Q-55-mechanism` receipt with its `chosen` and its constraint, complete the deferred-inputs entries, restore the three Project Principle names the deciding record attaches. Those remedies were adjudicated on the ground that the item's consumers are explorers who need the inputs in front of them, and this round brought no evidence that ground was wrong. A remedy that says "restate less" reopens three settled verdicts by argument.

Fourth, ITS OWN IMPACT STATEMENT PUTS THE COST OUTSIDE THE ARTIFACT. "That is a cost to this loop rather than to the pass", in the finding's own words. A cost to the loop is the orchestrator's convergence problem, not a defect in the document the pass reads.

WHAT I PRESERVE FROM IT. The diagnosis is worth keeping and I record it in the assessment below: the ledger-derived half of this item's content has generated nine of the loop's twenty valid findings, and that is the strongest argument for the class scoping in remedy L. The finding's failed attacks section is also the most useful part of its file and I relied on it in ruling `R3B-4`.

### R3A-1. The handle the item picks because it "resolves uniquely" resolves to two paragraphs, and the first hit is the wrong one

VERDICT: VALID. SEVERITY: `medium`, confirmed.

REPRODUCED IN MY TREE, and the ledger line numbers below are this worktree's:

```
$ grep -cF "THE MEMBERS KNOWN AT THIS WRITING" docs/plans/agent-scaffold.ledger.md
2
$ grep -cF "teaching W5 the structured step association W3 already uses" docs/plans/agent-scaffold.ledger.md
2
$ grep -nF "THE MEMBERS KNOWN AT THIS WRITING" docs/plans/agent-scaffold.ledger.md | cut -d: -f1
567
587
```

Both handles measure 2, so the distinction the item's sentence draws between them, which is the whole reason it tells a reader to prefer one, does not exist. The first hit is not the target: line 587 opens "THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP" and carries the member set ("THE MEMBERS KNOWN AT THIS WRITING. (a) THE W5 FIX, ..."), while line 567 is the orchestrator-defects paragraph, which contains the handle only inside the phrase "THE MEMBERS KNOWN AT THIS WRITING", which resolves to 1".

THE CLAIM WAS FALSE WHEN WRITTEN, so this is a `Q-70` defect and not a mid-loop ledger move:

```
$ git log --oneline -S "which resolves uniquely" main..HEAD -- docs/plans/agent-scaffold.plan.toml
896b053 docs: apply the round 2 remedies to Q-70
$ for c in c344ca5 dda6ae3 4e176a1 896b053 main; do <ledger count> <Q-70 claim count>; done
c344ca5 ledger=2 q70claim=0
dda6ae3 ledger=2 q70claim=0
4e176a1 ledger=2 q70claim=0
896b053 ledger=2 q70claim=1
main    ledger=2 q70claim=0
```

The ledger already carried two occurrences at every commit in the reviewed range, including the base. The round 2 fix pass asserted a measured property of a source it could read, without measuring it, in the sentence that instructs its reader to reference by quoted text rather than by position.

BOTH USE SITES ARE AFFECTED, and the second is the worse one. At `:1899` the same handle locates "the authoritative set" of the eventual step's deferred inputs, and the first raw hit does not contain the member set at all, so a reader who stops at hit one gets no members.

NOT AN INSTANCE OF A SETTLED FINDING. `R2B-5` (VALID, `low`) was the wrong LABEL on this paragraph and was fixed by relabelling it the routing paragraph; the label is now correct. What fails is a uniqueness assertion attached to the handle, in a clause the fix pass wrote. New site, new defect, no settled verdict covers it.

`medium` CONFIRMED. It is worse than `R2B-5` (`low`), because there the quotations were exact and only the label was wrong, whereas here the item asserts a false measured property and attaches an instruction not to check it. It is not `high` because both hits are adjacent, both concern the same subject, and a reader who reads both recovers the target. It sits where round 2 put `R2A-3`: a claim carrying a measured label that measurement contradicts, in a document whose whole referencing mechanism is quoted text.

THE RULING THE BRIEF ASKS FOR, on what `Q-70` should do given that the ledger paragraph it points into is itself defective and the orchestrator will fix that separately. `Q-70`'s remedy MUST NOT be conditioned on the orchestrator's ledger fix. Two forms were offered and I measured both:

```
$ grep -cF "THE MEMBERS KNOWN AT THIS WRITING. (a)" docs/plans/agent-scaffold.ledger.md
1
$ grep -c '^THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP' docs/plans/agent-scaffold.ledger.md
1
```

Both resolve to 1 today. I prescribe the SECOND, the paragraph-beginning form anchored with `^`, for a structural reason rather than a preference: the decay that produced this defect is the ledger quoting itself mid-paragraph, and a line-start anchor cannot be matched by a mid-paragraph quotation. It is also the form the item already uses successfully twice, and this round measured that it holds where the raw form does not (raw "THREE DEFECTS IN `agent-scaffold next`" measures 3 with the target LAST, raw "A FOURTH `agent-scaffold next` DEFECT" measures 4 with the target LAST, and the anchored form measures 1 for both). The narrower fix, deleting the words "which resolves uniquely" and keeping the handle, is NOT sufficient: it removes the false claim and leaves the reader on the wrong first hit at the site that matters more.

### R3A-2. `src/agents_md_drift.rs:41-55` starts one line inside the block it names

VERDICT: VALID. SEVERITY: `low`, confirmed.

Settled by opening the block. It begins at line 40:

```
40	//! GUARDED SET. Three comparisons make up the drift coverage; the other tests in this
41	//! file exercise the helpers and add none.
```

A reader who opens `41-55` as instructed reads "file exercise the helpers and add none." as the first line, with the `GUARDED SET` label and the three-comparisons statement cut off. `40-55` is the block.

IN SCOPE. `git log --oneline -S "src/agents_md_drift.rs:41-55" main..HEAD` returns `4e176a1`, the round 1 fix pass, so the citation was authored inside the reviewed range. Condition 1 of the out-of-scope precedent fails. I note for the fix pass that the range came from round 1 remedy B site 4, so it is a triager's own citation the item inherited faithfully; that is not a reason to leave it wrong.

`low` CONFIRMED, and the reviewer is right that the surrounding claim holds either way: lines 43 to 46 list the three comparisons, checks 1 and 2 are the two generated files against a fresh pack render, and `the_committed_scaffold_matches_a_fresh_render` (`src/agents_md_drift.rs:375`) asserts both. Round 1 ruled the identical class (`R1A-5`, a block under-cite) VALID at `low`, and this is milder than that one, where the cited range excluded a construct the same sentence attributed to the block. It stays valid rather than being waved through because the item's whole method is citation, the cost of the fix is one character, and this project does not have a "too small to fix" tier below `low`.

### R3C-1. The item records one design constraint on the quotation resolver, and two more are measured

VERDICT: VALID. SEVERITY: `medium`, confirmed.

CONSTRAINT ONE REPRODUCES, and it is the consequential one:

```
$ grep -cF 'the three `agent-scaffold next` defects routed here by an earlier human decision' docs/plans/agent-scaffold.ledger.md
1
$ grep -nF ... | cut -c1-90
569:ORCHESTRATOR DEFECT (21), CAUSED BY THE FIX FOR DEFECT (19) AND FOUND BY THE WRITER RA
$ grep -cF ... docs/plans/agent-scaffold.plan.toml
0
```

The only occurrence of the deleted sentence is inside the ledger's own post-mortem OF the deletion. A resolver that asks "does this quoted string occur in the cited file" returns TRUE, on the exact defect class it exists to catch, the moment the record writes the defect down. That is this project's invariable practice, and the loop has now measured the same decay three times independently: this one, round 2's "teaching W5 the structured step association W3 already uses" at 2 occurrences, and `R3A-1` above.

CONSTRAINT TWO REPRODUCES. The item's two quoted validator problem strings occur, in full, only in `docs/plans/agent-scaffold.plan.toml` and its projection `docs/plans/agent-scaffold.md`. They exist nowhere in `src/`, because they are runtime-substituted format output, so a resolver with no expected-output escape reports the item's own strongest evidence as dangling. I verified the strings are correct output rather than transcription: the `<S>/undeclared` fixture reproduces both byte-for-byte at exit 1.

VALID. Mechanism (3) is inside the pass's scope by the item's own heading, the item already carries one design constraint on it, and these two are measured properties of the same mechanism that the item's own method exists to register. No round 1 or round 2 finding touches mechanism (3)'s design; `R2A-4` was the count in the same sentence and its remedy was a count deletion, which landed.

`medium` CONFIRMED. Not `high`: the item asserts nothing false and names no implementation, so it does not steer toward the naive design, and an explorer who builds the mechanism against this repository's records meets the self-quoting case on the first run. Not `low`: the failure is a SILENT green on the highest-consequence defect class in the loop's record, the item is the sole input to the pass, and the constraint is exactly the kind of input the item's one recorded caveat establishes it should carry.

### R3C-2. The three mechanisms are ordered on buildability alone, with no evidence about what any of them catches

VERDICT: VALID. SEVERITY: `low`, confirmed.

REPRODUCED, all three yield measurements, in this worktree.

Mechanism (1): four `[[step.waiver]].note` breakdowns exist (`plan.toml:1331`, `:1340`, `:1349`, `:1358`), and every one already agrees with the `valid_findings` of its increment's round records:

```
workflow-enforcement-tier-inc1  note (3, 4, 6)          records 3 4 6
workflow-enforcement-tier-inc2  note (9, 5, 6, 4)       records 9 5 6 4
workflow-enforcement-tier-inc3  note (6, 4, 2, 0, 2)    records 6 4 2 0 2
workflow-enforcement-tier-inc4  note (11, 9, 6, 4, 5)   records 11 9 6 4 5
```

Zero red today. Mechanism (2): 62 distinct receipt `q_id`s against 70 registered `[[question]]` ids gives 40 dangling, all `Q-55-<suffix>`, and `Q-55-mechanism`, `Q-55-resumecost` and `Q-55-entryroute` are all among them. Mechanism (3): the only one that reaches the dangling-quotation class, per `R3C-1`.

VALID, and narrowly. The item's "buildability order" label is accurate and the finding says so; what is missing is the second axis, in the one paragraph the pass reads to rule letter (f), in an item that requires every proposal to price what it does not build.

`low` CONFIRMED. Nothing is asserted falsely, the qualifier is present, and an explorer told to price the mechanisms will measure them. It is the weakest finding of the round and I record that plainly. It stays valid because the measurement is cheap, in scope, not a re-raise, and material to a ruling the item commissions.

---

## Deduplication map

NO DUPLICATES THIS ROUND. Four pairs look adjacent and each is kept distinct, because collapsing any of them would lose a site:

- `R3B-1` and `R3B-4` both concern W5's ownership check. `R3B-1` records that the code states the lexical strip as a transitional shim and that W5's sibling check is already migrated, which bears on how heavily direction (iii) weighs. `R3B-4` records that the rule's premise is unrepresentable on the project's substrate, which yields a fourth direction and supplies the asymmetry that decides letter (b). Fixing either leaves the other unrecorded. Separate remedies, M and O.
- `R3B-3` and `R3C-2` both attack the mechanisms paragraph at `:1893` and both bear on letter (f). `R3B-3` is about DATA SURFACE (neither mechanism touches a waiver or a round record); `R3C-2` is about YIELD (which mechanism catches anything). Neither measurement implies the other. They share remedy P as separate sites.
- `R3C-1` and `R3C-2` both concern mechanism (3). `R3C-1` is about the mechanism's own failure modes; `R3C-2` is about its position in an ordering. Same paragraph, different content. Remedy P, separate sites.
- `R3A-1` and `R3B-5` both concern the item's relationship to the ledger. `R3A-1` is a live, reproduced false claim at two named sites. `R3B-5` is a structural prescription with no live instance, dismissed above; its actionable core is carried by remedy L's class scoping, so nothing is lost.

---

## Remedies

Lettering continues rounds 1 and 2, which ended at K. Each remedy is scoped to its CLASS over the whole enclosing sentence and paragraph, not to the quoted fragment. Every site any reviewer named is accounted for, including the sites I decide to leave alone.

THE STANDING PROHIBITION FROM REMEDIES A AND I IS UNCHANGED AND BINDING: do not substitute a corrected figure for a wrong one. It now extends to properties as well as counts, per remedy L.

### Remedy L. The item stops asserting a measured property of a document it does not own

Discharges `R3A-1`. Carries the actionable core of the dismissed `R3B-5`.

THE CLASS: the item has a stated cure for content that moves (state the property, give the command, carry no figure) and applies it to counts only. A uniqueness assertion about a handle is the same class as a count: it is a measurement of a document that this loop appends to every round, asserted from memory rather than measured, and it was false when written.

Site 1, `plan.toml:1895`, the whole sentence "Find that paragraph by the quoted text "THE MEMBERS KNOWN AT THIS WRITING", which resolves uniquely, rather than by a line number or by the fix quotation itself, which now resolves to two paragraphs because the ledger's own round records quote it." Two things in one sentence are wrong: the uniqueness assertion (measured 2) and the distinction it draws between the two handles (both measure 2). Rewrite the sentence rather than deleting the clause. Use the paragraph-beginning form the item already uses successfully twice, anchored so the ledger quoting itself cannot decay it: the routing paragraph begins "THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP". State no count for the new handle either.

Site 2, `plan.toml:1899`, "The authoritative set is the ledger's `validation-constraints` routing paragraph, found by the quoted text "THE MEMBERS KNOWN AT THIS WRITING"". Same replacement, and this site matters more: the first raw hit carries no member set at all, so a reader who stops there gets nothing.

Site 3, the class over the whole item. Do not attach a measured property to any other handle. Measured this round, no other site needs an edit: `TWO WAIVERS ARE OWED AND CANNOT YET BE WRITTEN`, `` `Q-55-resumecost` DECIDED ``, `THE BACKSTOP CORRECTED BOTH EARLIER AGENTS ON OWNERSHIP` and `NO OWNER anywhere in the plan` each resolve to 1 and none carries a uniqueness claim, and the two paragraph-beginning handles are already in the anchored form. The rule is forward-looking, and it is what makes this a class remedy rather than a two-site patch.

DO NOT condition any of this on the orchestrator's separate ledger correction. A `Q-70` pointer whose correctness depends on an edit to a document the item does not own is the coupling that produced the defect.

Sites left alone, with a verdict for each:

- The find-by-quoted-text convention itself (`plan.toml:1891` and `:1899`, "find them by the quoted text ... rather than by a line number"). CORRECT AS IT STANDS and it is the right convention; every one of the item's handles resolves. Do not weaken it. What failed is an assertion attached to one handle.
- `plan.toml:1899`, the "three `agent-scaffold next` defects" property and its reproduction command. LEAVE, as round 2 remedy I site 1 prescribed. It reproduces: six lines, at ledger `:533`, `:569`, `:1071`, `:1275`, `:1277` and `:1353`.
- `plan.toml:1895`'s statement that the fix quotation "now resolves to two paragraphs because the ledger's own round records quote it". True today (measured 2) and its stated cause is accurate, but it goes with site 1's rewrite rather than surviving as a standalone measured claim, for the same reason the uniqueness assertion goes.

### Remedy M. Record what the code already states about W5's ownership axis

Discharges `R3B-1`.

THE CLASS: the item cites `src/workflow.rs` at fifteen distinct places and never reaches the one doc block that states the project's own direction for the lexical strip, nor the asymmetry inside the very function it asks the pass to fix.

Site 1, `plan.toml:1885` (THE BLOCKER) or `:1895` (the candidate directions), whichever paragraph the fix pass judges the better home. Record two facts as measured inputs, WITHOUT recommending a direction, in the form the item already uses for direction (iii) ("THIS IS RECORDED, NOT RECOMMENDED"):

1. `leading_slug`'s doc closes at `src/workflow.rs:83-87` with "Inc 2 retires this risk for NEW data ... This shim remains only for pre-migration records that omit the structured id", so the code states the lexical strip as transitional.
2. Inside `w5_problems` the two checks are asymmetric: the ownership check is lexical at `:564`, while the record-backed evidence check at `:594-596` prefers the escalation's structured ids, under its own comment at `:590-592` naming the Inc 2 preference and the shim. W5 is migrated on one axis and not on the other.

State the fact's LIMIT in the same passage, because the item's standard is to state limits: the retirement sentence is about `round` and `escalation` RECORDS carrying structured ids, and `waiver.increment` is a plan-authored id rather than a `task` (the doc's stated subject at `:71`), so the fact bears on the axis without settling it.

Site left alone, with a verdict: `plan.toml:1885`'s existing citation `src/workflow.rs:64-68`. CORRECT. It resolves and it is the right citation for the `-incA` / `-incB` alphanumeric run the sentence uses it for. Add the doc-block citation beside it; do not replace it.

### Remedy N. Price mechanism (1) at what building it costs

Discharges `R3B-2`.

THE CLASS: the item requires every proposal to price the other mechanism, and prices one of its own at zero.

Site 1, `plan.toml:1893`, mechanism (1)'s whole sentence, "The convention already exists, carried in the `note` field of `[[step.waiver]]` entries, and only the enforcement is missing." Replace the enforcement-only framing with the measured cost: the `note` is read by exactly one site in the tool, `src/plan/render.rs:527`, which writes it into the generated Markdown, and no check reads it; `Round` (`src/metrics.rs:620-651`) carries no `valid_findings` and `parse_rounds` (`:660-711`) discards it; `workflow.rs` owns no JSON parsing, so every check reads rounds through that projection; and the only waiver-to-rounds join in the tool is W3's at `src/workflow.rs:498-502`. So the join needs the round-log association AND a widening of the projection the checks consume. DO NOT rule whether that couples: that is letter (a) and the item carries no rulings by design.

Site 2, `plan.toml:1895`, the coupling paragraph. The item already records that `Q-55-mechanism` queued an optional `project` field on `Round` and that the human's reasoning constrains that schema to "ONE DELIBERATE EDIT RATHER THAN A RIDER ON A PATH FIX". Record that the prospective W6 join needs a second field on that same struct, so TWO of the three participants the item names edit it, not one.

Sites left alone, with a verdict for each:

- `plan.toml:1901`, letter (a). CORRECT AS IT STANDS. Its third part already demands the cross-pricing, which is where this evidence is consumed; no new letter is owed and the body site above is the right home for the input.
- `plan.toml:1895`, direction (ii)'s three representations (`src/metrics.rs:539-601`, `src/plan/source.rs:279-300`, `src/workflow.rs:237-267`). CORRECT AS THEY STAND, verified again at the line, including that the TOML struct is `deny_unknown_fields` and carries no `step` field.

### Remedy O. Record the substrate on which the ownership rule's premise holds

Discharges `R3B-4`.

THE CLASS: every direction the item names is a way to FIX the rule, and the item never records that on its own substrate the rule has no violation to catch, which is the fact that decides the ruling the item itself commissions at letter (b).

Site 1, `plan.toml:1889`, the two-path paragraph, and `:1901` letter (b). The item frames the two paths as symmetric. Record the asymmetry, measured: the `src/plan/source.rs` path consults the step's DECLARED increment set structurally (`:792-793`, `:807-811`), while W5 consults a substring of the id and reports a step that need not exist. Carry the fixture, which reproduces at exit 1: a `[[step.waiver]]` under `workflow-enforcement-tier` naming `totally-not-a-step-inc1` yields "increment `totally-not-a-step-inc1` belongs to step `totally-not-a-step`", where `grep -c 'slug = "totally-not-a-step"'` returns 0.

Site 2, the same paragraph or `:1895`. Record that on `[meta].primary = "toml"` (`plan.toml:4`) the state the rule exists to report is unrepresentable: `waivers_from_toml` sets `step: step.slug.clone()` (`src/workflow.rs:258`) and the TOML `Waiver` (`src/plan/source.rs:279-300`, `deny_unknown_fields`) has no `step` field, so a waiver cannot name a wrong step; and W5's own first rule at `:553` is structurally unreachable there, because `check_workflow_toml` (`:180-195`) derives both `plan.step_views()` and `waivers_from_toml(plan)` from the same `plan.steps`. The item already carries the first half as escape route 2, scoped to what a waiver AUTHOR can do; what is missing is what it implies for the RULE.

Site 3, `plan.toml:1895`, the candidate directions. Record the fourth direction as a candidate, WITHOUT recommending it and in the item's established "RECORDED, NOT RECOMMENDED" form: scope the ownership rule to the substrate where its premise holds (the JSONL `type:"waiver"` record, whose `step` IS independently authorable), or retire it in favour of the structural membership check at `src/plan/source.rs:807` that already runs on the TOML path. Price it on both sides, as the item requires of every direction. The benefit, measured first-hand this round: with the two fold tokens declared as `[[step.increment]]` entries, both owed waivers written and the step `complete`, the ownership rule is the ONLY problem the tool reports, and disabling that one rule in a scratch build returns "workflow invariants hold" at exit 0, so this direction plus the declaration is a complete unblocking of the two owed waivers. The cost: `run_checks` is deliberately one implementation across both substrates (`src/workflow.rs:196-200`), so a substrate-conditional rule pushes against the parity property direction (i) already owes a ruling on.

Site 4, `plan.toml:1901`, letter (c), or the body sentence that raises it. Direction (i) already owes a ruling on what W5 does on the Markdown substrate. Extend the same duty symmetrically: a direction that scopes or retires the rule owes a ruling on what W5 does on the JSONL substrate, where the rule's premise DOES hold.

Sites left alone, with a verdict for each:

- `plan.toml:1887`, escape route 4's structural statement that `w5_problems` is handed `plan::Step` and so has no declared-increment data to consult. CORRECT AS IT STANDS, verified again. The fourth direction does not weaken it; it changes what follows from it.
- `plan.toml:1891`, "ONLY THE OWNERSHIP CHECK BLOCKS THEM". CORRECT, and now confirmed by measurement rather than by reading, in the `<S>/unblock` fixture above. Do not touch it.
- `plan.toml:1887`, escape route 2. CORRECT AND CORRECTLY SUBSTRATE-SCOPED, as round 1 remedy B already ruled. Site 2 above adds what the same fact implies for the rule; it does not restate or weaken route 2.

### Remedy P. Record what the mechanisms are measured to touch and to catch

Discharges `R3B-3`, `R3C-1`, `R3C-2`. Three sites in one paragraph, none of which implies another.

THE CLASS: the item registers the three detection mechanisms and letter (f) makes their scope the pass's ruling, and the measurements that bear on that ruling exist today and are not in the item.

Site 1, `plan.toml:1893`, mechanism (3). The item records one caveat (the `src/checks.rs` red list, which is accurate and stays). Add two measured constraints, as properties with their reproductions:

1. THE SELF-QUOTING RECORD. The ledger records a dangling-quotation defect by quoting the deleted sentence, so a resolver that asks only "does this string occur in the cited file" goes GREEN on the class it exists for. Measured: the deleted sentence now occurs exactly once in the ledger, inside the post-mortem OF the deletion. A resolver must scope to LIVE passages, excluding a record's own post-mortems and round records.
2. EXPECTED TOOL OUTPUT IS NOT A DOCUMENT QUOTATION. The item's own two validator problem strings occur nowhere but this item and its projection, because they are runtime-substituted format output, so a resolver with no expected-output escape reports the item's strongest evidence as dangling.

Site 2, `plan.toml:1893`, the ordering sentence. Keep the "buildability order" label, which is accurate and correctly attributed. Add the yield axis as a measurement and NOT as a priority ruling: mechanism (1) has four `[[step.waiver]].note` breakdowns and all four already agree with their round records' `valid_findings`, so built today it is a regression guard with nothing to catch; mechanism (2) returns 40 unregistered receipt ids today, including the two whose omission this loop found; mechanism (3) is the only one that reaches the dangling-quotation class. Give the reproductions rather than carrying the figures forward, per remedies A and I.

Site 3, `plan.toml:1893` and `:1901` letter (f). Record the measured data surfaces so the DESIGNED-versus-BOUNDED ruling has evidence: mechanism (2)'s inputs are exactly W4's (`w4_problems(questions, decisions, baselines)`, fed by `plan.question_views()` and `metrics::parse_decisions`), and mechanism (3) reads no workflow record at all, so neither touches a waiver, a round record, or the join the pass exists to settle. DO NOT rule letter (f) and DO NOT argue the mechanisms out of scope: they are in the pass's scope by the human's recorded decision, which the item relays correctly.

Sites left alone, with a verdict for each:

- `plan.toml:1901`, letter (f) itself. CORRECT AS IT STANDS. The refute lens attacked "letter (f) is unanswerable and should be dropped" and its own analysis defeated the attack; the defect is the missing evidence, not the question.
- `plan.toml:1893`, the `Q-55-check21b` attribution and the "every citation is stale" property. CORRECT, re-measured: the census returns 15 distinct citations and the item states the property rather than the figure, as round 2 remedy I site 3 required.
- `plan.toml:1893`, mechanism (1)'s and mechanism (2)'s refusals to state a count. CORRECT, and they remain the model. Re-measured: the breakdown grep returns four lines, and the receipt extraction returns 62 distinct ids against 70 registered questions, 40 dangling, all `Q-55-`.

### Remedy Q. One citation range correction

Discharges `R3A-2`.

Single site, `plan.toml:1901`, "(the guard is `src/agents_md_drift.rs:41-55`)". The block it names opens at `:40` with "GUARDED SET. Three comparisons make up the drift coverage". Cite `:40-55`. The claim the citation supports is unaffected and stays as written: checks 1 and 2 at `:43-44` are the two generated files against a fresh pack render, and `the_committed_scaffold_matches_a_fresh_render` (`src/agents_md_drift.rs:375`) asserts both, so a change to the W5 clause that moves only one of the three sites fails the test.

---

## Overall assessment

WHAT THE ROUND'S REAL RESULT IS. Eight valid findings, ceiling `high`, two at `high`, on a document whose factual spine is now measured by three independent triagers and holds. I rebuilt the item's own two-path fixture from scratch and both cases reproduce byte-for-byte, including the exact problem strings and both exit codes; I re-ran the affected-population pipeline (six identities across three steps), the waiver-note breakdown grep (four sites), the receipt set difference (62 against 70, 40 dangling, all `Q-55-`), the `src/checks.rs` census (15 distinct) and the three-next-defects grep (six lines); and I opened every `src/workflow.rs`, `src/metrics.rs` and `src/plan/source.rs` citation this round's findings turn on. AS IN ROUNDS 1 AND 2, NO FINDING SHOWS `Q-70` ASSERTING SOMETHING THE TOOL CONTRADICTS. One finding shows it asserting something a measurement of the project's own records contradicts (`R3A-1`), which is the milder class rounds 1 and 2 also found. The rest are omissions of code facts and of measured properties.

ONE SYSTEMIC DEFECT OR MANY. ONE, and it is new this round, plus a residue of two.

The systemic one is THE ITEM IS MEASURED AGAINST ITS SOURCES AND NOT AGAINST THE CODE OF THE CHECKS IT COMMISSIONS. `R3B-1`, `R3B-2`, `R3B-4`, `R3B-3` and `R3C-2` are five faces of it: the code states the lexical strip as transitional and W5 is half-migrated; the prospective W6 join needs a field the projection drops; the ownership rule's premise is unrepresentable on the project's substrate; the two other mechanisms touch none of the pass's data; and the mechanisms carry no yield evidence. Every one is measurable in `src/` today, and every one changes what an explorer is told to weigh. Rounds 1 and 2 both predicted this axis was uncovered and both were right. It is a DIFFERENT class from rounds 1 and 2, whose systemic defects were a moving population stated as a fixed figure and a completeness claim the item could not keep; neither reproduced this round, which is worth recording as evidence that remedies A, I and F held.

The residue is `R3A-1` and `R3A-2`, one false property assertion and one citation range, both ordinary, and `R3C-1`, which sits between the two groups: it is a measured property of a mechanism, found by a lens pointed at gates rather than at the code.

IS THE ITEM CONVERGING OR STILL MOVING. CONVERGING ON ACCURACY, STILL MOVING ON COMPLETENESS, and the two are diverging rather than tracking each other. On accuracy the trend is strong: round 1 found four defects in counts and citations, round 2 found three, round 3 found one (`R3A-1`) plus one range (`R3A-2`), and this round's exhaustive quotation sweep resolved 43 of 44 quotations and 42 of 42 citations. The measured spine has now survived three independent reproductions unchanged. On completeness the item has gained duties in every round, and this round's five framing findings are all additions rather than corrections. That is the pattern the round 2 triage predicted in different words, and it means a fourth round's findings are likely to be additions again rather than repeats. I record one honest counterweight the refute lens established and I could not beat: its attack that the item should have been three sentences FAILED, and the elaborated half is the half that has held under three triagers. The growth is not evidence of a bloated document; it is evidence that a high-blast-radius design brief acquires inputs as the code is read more carefully.

IS THE PASS'S QUESTION PARTLY ALREADY ANSWERED. YES, PARTLY, AND I STATE IT PLAINLY BECAUSE THE ORCHESTRATOR MUST PUT IT TO THE HUMAN EITHER WAY. My verdicts imply the following, and none of it is a ruling on whether the human's recorded decision to run a pass should be revisited, which is the human's:

1. LETTER (a), THE COUPLING RULING, IS SUBSTANTIALLY CONSTRAINED BY THE SOURCE. The prospective W6 join's inputs live only in the round log, because the only waiver-to-rounds join in the tool is W3's and `workflow.rs` reads rounds through one projection. So W5 and W6 share a mechanism if and only if W5 adopts a round-log association. That limb of letter (a) is measurable rather than deliberable. What is NOT answered is the rest of the letter: the cross-pricing ("what the other mechanism costs under the direction taken"), which the ledger records as the human's ground for running a pass at all, and whether the queued project-identity edit shares the mechanism, which turns on a recorded human constraint about how the record schema may be edited rather than on a code fact.
2. LETTER (b), THE AUTHORITATIVE-PATH RULING, HAS ITS DECIDING FACT IN THE CODE. The two paths are not symmetric: one consults declared data structurally, the other consults a substring and can name a step the plan does not contain, on a substrate where the violation it exists to report cannot be authored. That fact does not settle the letter, because the cross-substrate parity property (`run_checks` is one implementation by design) is a genuine counterweight and the JSONL substrate keeps the rule meaningful, but it moves the ruling a long way.
3. LETTER (f), THE SCOPE OF MECHANISMS 2 AND 3, HAS ITS EVIDENCE AVAILABLE TODAY. Neither mechanism shares a data surface with the pass's question, and their yield order is the exact inverse of the order the item presents. The ruling stays a scope judgement, but it need not be made blind.
4. WHAT IS NOT ANSWERED AT ALL, so the pass is still owed: letter (d), which of the two W6 checks is renumbered, is a naming decision; letter (e), whether the `Q-55-<suffix>` receipt ids are dangling or a convention the detector must model, is a policy question about how this project records sub-decisions; letter (g), the YAGNI boundary, is definitionally the pass's; and the cross-pricing under (a) is design work no measurement supplies.

SO: THE PASS'S CENTRAL COMPARISON IS MUCH NARROWER THAN THE ITEM PRESENTS, AND THE VEHICLE IS STILL RIGHT. I reached that independently of the refute lens, which attacked the vehicle hardest and failed, and I agree with where its attack landed. The item should not become `open` on this round's evidence; it should record the measured facts so the pass deliberates the part that is genuinely open. That is what remedies M, N, O and P do, and none of them asks the item to decide anything.

ON THE LEDGER DEFECT, WHICH IS NOT A FINDING AGAINST THIS ARTIFACT. Two reviewers reached the orchestrator's own defect: the paragraph recording handle decay asserts that a handle "resolves to 1" in the sentence that is itself the second occurrence, and its stated property, that the first hit is the real paragraph, is false for both of its own examples (I measured both: raw counts 3 and 4 with the target LAST in each case). The orchestrator has confirmed it and will fix it separately, so I rule only on what `Q-70` owes. `Q-70` owes a pointer that does not depend on that fix: remedy L prescribes the paragraph-beginning anchored form, measured at 1 today, which the same decay mode cannot break, and forbids conditioning the item's correctness on an edit to a document it does not own. The item's exposure was never that the ledger is wrong; it was that the item asserted a property of the ledger instead of pointing at it.

WHAT THE THREE LENSES COLLECTIVELY MISSED. Three things, all measured.

1. NO LENS TESTED THE FOURTH DIRECTION'S STRONGEST CLAIM. `R3B-4` asserts that its direction plus the declaration is a complete unblocking of the two owed waivers, and argues it from the item's own fixture. I built it: the two tokens declared, both waivers written, the step flipped to `complete`, and W5's ownership rule disabled in a scratch build. The result is "workflow invariants hold" at exit 0. That also independently confirms two of the item's own claims that no lens had tested together, that W3's covering-waiver match accepts the waivers and that the record-backed evidence join resolves through the `task` fallback.
2. THE ROUND'S OWN CLASSIFICATION FINDING CARRIES A MISCOUNT. `R3B-5` says "In round 2 alone that is five of nine" and lists six. The loop's standing caution firing inside a finding written about restated-record defects, exactly as round 2 recorded it firing inside `R2A-2`.
3. THE ARTIFACT'S NON-PROSE CHANGES REMAIN UNEXAMINED, three rounds in. All nine findings this round are against the `ask` prose. `[meta].orphan_tasks`'s `q70-capture` token and the empty sidecar have been examined only by triagers, never by a reviewer. Both are correct: `grep -c 'q70-capture' docs/metrics/workflow.jsonl` now returns 2, so round 1's reading of the token as a deliberate pre-declaration has come true, and all 70 question sidecars are zero bytes.

MECHANICAL STATE OF THE ARTIFACT, checked independently in this worktree: `render docs/plans/agent-scaffold.plan.toml --check --strict` reports "up to date" at exit 0; `validate --source ... --metrics ... --workflow` reports 310 records valid, 95 steps and 70 questions valid, and "workflow invariants hold" at exit 0; all three changed files return 0 under `LC_ALL=C grep -cP '[^\t\x20-\x7e]'`. `Q-70`'s claim that "NO step exists yet" holds: `grep -c 'slug = "validation-constraints"'` returns 0.

---

## Defects in `src/` and in source documents, for the orchestrator to route

NOT findings against this artifact, and correctly marked as such by the reviewers who raised them. Listed so none is lost.

IN `src/`, both from the refute lens and both re-measured by me:

- S-SRC-1. W5's increment-ownership message asserts an ownership fact that need not be true of the plan. At `src/workflow.rs:565-571` it formats "increment `{}` belongs to step `{}`" with `leading_slug(increment)`, a substring of the id. Demonstrated at exit 1 in `<S>/fake-owner`: a TOML waiver naming `totally-not-a-step-inc1` produces "belongs to step `totally-not-a-step`", where that slug appears nowhere in the plan, while W5's own real-step rule stays silent because the waiver's `step` was inherited and is real. On `[meta].primary = "toml"` the message can name a non-existent step on every firing.
- S-SRC-2. On the TOML path `if !slugs.contains(waiver.step.as_str())` at `src/workflow.rs:553` is structurally unreachable, because `check_workflow_toml` (`:180-195`) derives both `plan.step_views()` and `waivers_from_toml(plan)` from the same `plan.steps`.

Both are inputs the `validation-constraints` step will meet, and both are the same code facts remedy O asks the item to record as pass inputs. Recording them in `Q-70` does not discharge them as source defects.

IN THE LEDGER, from the quotation lens, all three re-measured by me in this worktree:

- S-LED-1. The orchestrator-defects paragraph at `docs/plans/agent-scaffold.ledger.md:567` ends by citing "THE MEMBERS KNOWN AT THIS WRITING", which resolves to 1", and that sentence IS the second occurrence. `grep -cF` returns 2. The orchestrator has confirmed this and owns the fix.
- S-LED-2. The same paragraph's two other measurements are stale and its stated property is false. It records "THREE DEFECTS IN `agent-scaffold next`" at 2 and "A FOURTH `agent-scaffold next` DEFECT" at 3, and says "the first hit is the real paragraph in each case". Measured now: 3 and 4, with the target the LAST hit in both cases (`:1363` and `:997`). `Q-70` is unaffected, because it uses the paragraph-beginning form for both, which resolves to 1; that is the property worth promoting into the convention.
- S-LED-3. Every live "three `agent-scaffold next` defects" is still owed a correction, at ledger `:533`, `:1071`, `:1275`, `:1277` and `:1353` (the sixth hit, `:569`, is the defect (21) post-mortem quoting the old wording and is not a live claim). `Q-70` states this and correctly says it is not the item's to make.

ONE GATE PROPERTY, from the gates lens, recorded rather than routed: `agent-scaffold render --check` without `--strict` warns and exits 0, so the hard form is reached only through `.agents/checks.toml` or by spelling `--strict` by hand. That is documented behaviour, not a defect, and `Q-70` never claims otherwise. I used `--strict` for the mechanical check above.

---

## Commands that decided a verdict, with their output

Every command below ran in `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/tri-q70-r3`, or against a fixture copied from it. Fixture root abbreviated `<S>` for `<scratchpad>/tri-q70r3`.

The re-raise check:

```
$ grep -cF shim|pre-migration|"Inc 2"|valid_findings <the eight round 1 and round 2 findings files>
shim 0/8, pre-migration 0/8, "Inc 2" 0/8
valid_findings: 1 (premises, a printf fixture line), 1 (source, a grep pattern),
                1 (r2-surfaces, a quotation of the item's own sentence), 0 elsewhere
```

`R3B-1`, the doc block, the asymmetry and the census:

```
$ grep -n "Inc 2 retires this risk\|This shim remains only\|pre-migration records that omit\|The leading step slug of a\|leading_slug(increment) != waiver.step\|Tie the joined escalation\|escalation_increment_id(escalation)\|fn leading_slug" src/workflow.rs
71:  /// The leading step slug of a `task`: ...
83:  /// Inc 2 retires this risk for NEW data: ...
86:  /// without ever reaching this lexical strip. This shim remains only for
87:  /// pre-migration records that omit the structured id.
88:  fn leading_slug(task: &str) -> &str {
564:                 if leading_slug(increment) != waiver.step {
590:                 // Tie the joined escalation to the unit the waiver exempts, preferring
595:                 waiver.increment.as_deref() == Some(escalation_increment_id(escalation)),
$ awk 'NR>=1880 && NR<=1904' docs/plans/agent-scaffold.plan.toml | grep -oF <term> | wc -l
shim 0   pre-migration 0   "Inc 2" 0
$ awk 'NR>=1880 && NR<=1904' ... | grep -oE 'src/workflow\.rs:[0-9]+(-[0-9]+)?' | sort | uniq -c
:64-68 :88 :119 :127 :141 :206-221 :237-267 :258 :321 :445-447 :450 :498-502 :549 :553 :564
(fifteen distinct; :83-87 absent)
```

`R3B-2`, the projection and the joins:

```
$ grep -n "pub(crate) struct Round {" src/metrics.rs      -> 620   (closes at 651)
$ grep -n "pub(crate) fn parse_rounds" src/metrics.rs     -> 660   (closes at 711)
   fields read: task, artifact, outcome, consecutive_clean, risk_class, step, increment
$ grep -c serde_json src/workflow.rs                      -> 0
$ grep -c valid_findings src/workflow.rs                  -> 0
$ grep -n valid_findings src/metrics.rs | head -3         -> 336 (doc), 367, 454  (check_record only)
$ grep -n "fn w3_problems\|fn w4_problems\|fn w5_problems" src/workflow.rs
309: fn w4_problems(   437: pub(crate) fn w3_problems(   544: fn w5_problems(
   w3_problems is the only one taking both rounds and waivers
$ grep -rn "\.note\b" --include=*.rs src/ | grep -v test
src/plan/source.rs:299 (the field)   src/plan/render.rs:527 (the only reader)
```

`R3B-3` and `R3C-2`, the mechanisms:

```
$ sed -n '186,193p' src/workflow.rs   (check_workflow_toml feeds question_views + parse_decisions)
$ sed -n '216,220p' src/workflow.rs   (run_checks -> w4_problems(questions, decisions, baselines))
$ awk 'NR==533' docs/plans/agent-scaffold.ledger.md | grep -o "THE GROUND:.*"
   three limbs: the unmeasured W5/W6 coupling ("Ground decisions in evidence"); the choice
   made with W6 in view ("Prefer the cleaner long-term architecture over the smallest diff");
   the already-diagnosed defects staying out ("Minimal by default")
$ grep -oE '\(2\) EXPLORERS write to that directory[^.]*\.' docs/plans/agent-scaffold.ledger.md
   (2) EXPLORERS write to that directory, each ruling on the W5/W6 coupling and carrying an
   explicit "what not to build" boundary.
$ grep -onE "[(][0-9]+(, [0-9]+){1,6}[)]" docs/plans/agent-scaffold.plan.toml
1331:(3, 4, 6)  1340:(9, 5, 6, 4)  1349:(11, 9, 6, 4, 5)  1358:(6, 4, 2, 0, 2)
$ jq -r --arg t <increment> 'select(.type=="round" and ((.increment // .task)==$t)) | .valid_findings' docs/metrics/workflow.jsonl
inc1 -> 3 4 6      inc2 -> 9 5 6 4      inc3 -> 6 4 2 0 2      inc4 -> 11 9 6 4 5
   four sites, four agreements, zero red
$ receipt ids 62, registered questions 70, dangling 40, non-`Q-55-` dangling 0
   Q-55-mechanism, Q-55-resumecost and Q-55-entryroute are all in the dangling set
```

`R3B-4`, the fake-owner fixture and the substrate limbs:

```
$ agent-scaffold validate --source <S>/fake-owner/... --metrics <S>/fake-owner/... --workflow
<PLAN>: waiver `workflow-enforcement-tier-wX` on step `workflow-enforcement-tier` names increment `totally-not-a-step-inc1`, which is not one of the step's increments
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-wX`: increment waiver names step `workflow-enforcement-tier` but increment `totally-not-a-step-inc1` belongs to step `totally-not-a-step`
EXIT=1
$ grep -c 'slug = "totally-not-a-step"' <S>/fake-owner/docs/plans/agent-scaffold.plan.toml
0
$ sed -n '1,4p' docs/plans/agent-scaffold.plan.toml        -> primary = "toml" at :4
$ sed -n '274,300p' src/plan/source.rs
   struct Waiver, #[serde(deny_unknown_fields)], fields id/unit/increment/reason/
   evidence_tier/evidence/note; NO step field; doc says "minus `task`/`step`, which the
   nesting supplies"
$ sed -n '250,262p' src/workflow.rs                        -> step: step.slug.clone() at :258
$ sed -n '180,195p' src/workflow.rs                        -> step_views() and waivers_from_toml(plan)
   both derived from plan.steps, so :553 cannot fire on the TOML path
```

`R3B-4`, the fourth direction tested end to end (`<S>/mut4` is `git archive HEAD | tar -x` with W5's ownership block disabled and its own `CARGO_TARGET_DIR`; `<S>/unblock` declares both fold tokens as `[[step.increment]]`, writes both owed waivers, and flips the step to `complete`):

```
$ <worktree binary> validate --source <S>/unblock/... --metrics <S>/unblock/... --workflow
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: ... belongs to step `workflow-enforcement-tier-fold`
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w6`: ... belongs to step `workflow-enforcement-tier-endproperty-fold`
EXIT=1
$ <S>/target-mut4/debug/agent-scaffold validate --source <S>/unblock/... --workflow
<LOG>: 310 records, valid
<PLAN>: 95 steps, 70 questions, valid
<PLAN> vs <LOG>: workflow invariants hold
EXIT=0
```

The item's own two-path fixture, rebuilt from scratch:

```
$ agent-scaffold validate --source <S>/undeclared/... --workflow
<PLAN>: waiver `workflow-enforcement-tier-w5` on step `workflow-enforcement-tier` names increment `workflow-enforcement-tier-fold`, which is not one of the step's increments
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`
EXIT=1
$ agent-scaffold validate --source <S>/declared/... --workflow
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`
EXIT=1
```

`R3A-1`, the handles and the provenance:

```
$ grep -cF "THE MEMBERS KNOWN AT THIS WRITING" docs/plans/agent-scaffold.ledger.md          -> 2
$ grep -cF "teaching W5 the structured step association W3 already uses" ...ledger.md        -> 2
$ grep -nF "THE MEMBERS KNOWN AT THIS WRITING" ...ledger.md | cut -d: -f1                   -> 567, 587
$ awk 'NR==567' ...ledger.md | grep -o "THE MEMBERS KNOWN AT THIS WRITING.\{0,40\}"
THE MEMBERS KNOWN AT THIS WRITING", which resolves to 1.
$ awk 'NR==587' ...ledger.md | grep -o "THE MEMBERS KNOWN AT THIS WRITING.\{0,60\}"
THE MEMBERS KNOWN AT THIS WRITING. (a) THE W5 FIX, teaching W5 the structured step ...
$ git log --oneline -S "which resolves uniquely" main..HEAD -- docs/plans/agent-scaffold.plan.toml
896b053 docs: apply the round 2 remedies to Q-70
$ per-commit: c344ca5 ledger=2 claim=0 | dda6ae3 ledger=2 claim=0 | 4e176a1 ledger=2 claim=0
             896b053 ledger=2 claim=1 | main ledger=2 claim=0
$ grep -cF "THE MEMBERS KNOWN AT THIS WRITING. (a)" ...ledger.md                             -> 1
$ grep -c '^THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP' ...ledger.md     -> 1
```

`R3A-2`, the block bounds, and `R3C-1`, the two constraints:

```
$ awk 'NR>=40 && NR<=41' src/agents_md_drift.rs
40  //! GUARDED SET. Three comparisons make up the drift coverage; the other tests in this
41  //! file exercise the helpers and add none.
$ git log --oneline -S "src/agents_md_drift.rs:41-55" main..HEAD -- docs/plans/agent-scaffold.plan.toml
4e176a1 docs: apply the round 1 remedies to Q-70
$ grep -cF 'the three `agent-scaffold next` defects routed here by an earlier human decision' docs/plans/agent-scaffold.ledger.md
1        (line 569, inside the post-mortem OF the deletion)
$ same string in docs/plans/agent-scaffold.plan.toml                                         -> 0
$ grep -rlF <each full validator problem string> . (excluding .reviews/ and .git)
docs/plans/agent-scaffold.plan.toml, docs/plans/agent-scaffold.md      (both strings, no src/ hit)
```

The routed ledger items, re-measured:

```
$ grep -cF 'THREE DEFECTS IN `agent-scaffold next`' ...ledger.md   -> 3   (567, 587, 1363; target LAST)
$ grep -cF 'A FOURTH `agent-scaffold next` DEFECT' ...ledger.md    -> 4   (539, 567, 587, 997; target LAST)
$ grep -c '^THREE DEFECTS IN ...' -> 1        $ grep -c '^A FOURTH ... DEFECT' -> 1
$ grep -niE "three .?(agent-scaffold )?next.? defects" ...ledger.md | cut -d: -f1
533 569 1071 1275 1277 1353
```

The item's own reproduction commands and the artifact's mechanical state:

```
$ jq -r 'select(.type=="round") | [(.step // (.task|sub("-inc[a-zA-Z0-9]+$";""))), (.increment // .task)] | join(" ")' docs/metrics/workflow.jsonl | sort -u | awk '{lead=$2; sub(/-inc[a-zA-Z0-9]+$/,"",lead); if (lead != $1) print $1, $2}'
decision-folder-currency decision-folder-currency-fold
workflow-driver workflow-driver-stage0a / -stage0b / -stage1
workflow-enforcement-tier workflow-enforcement-tier-endproperty-fold / -fold
$ grep -oE "src/checks[.]rs:[0-9]+(-[0-9]+)?" docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md | sort -u | wc -l   -> 15
$ grep -c 'q70-capture' docs/metrics/workflow.jsonl                                          -> 2
$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check --strict   -> up to date, EXIT 0
$ agent-scaffold validate --source ... --metrics ... --workflow
310 records valid; 95 steps, 70 questions valid; workflow invariants hold;      EXIT 0
$ LC_ALL=C grep -cP '[^\t\x20-\x7e]' on the three changed files                 -> 0, 0, 0
$ grep -c 'slug = "validation-constraints"' docs/plans/agent-scaffold.plan.toml -> 0
```

WHAT I SETTLED BY RUNNING AND WHAT BY READING.

RUN: `R3B-2` (the projection, the join census and both falsifier halves), `R3B-3` (the check inputs and the ledger limbs), `R3B-4` (the fake-owner fixture, the two-path fixture, and the source-mutation unblock test), `R3B-5` (the classification against both triage files and its own arithmetic), `R3A-1` (the handle counts, the per-commit provenance and both candidate remedies), `R3C-1` (both constraints), `R3C-2` (all three yield measurements), the re-raise token sweep, every one of the item's own reproduction commands, and the artifact's `render --check --strict`, `validate --workflow` and ASCII sweep.

RUN AND READ: `R3B-1`, whose census I ran and whose doc-block and asymmetry claims I settled by opening `src/workflow.rs:60-145` and `:520-600`.

READ: `R3A-2`, settled by opening `src/agents_md_drift.rs:36-62`; both prior triage files in full, held throughout so no settled finding is re-raised; and the `Q-70` entry at `docs/plans/agent-scaffold.plan.toml:1880-1903`.

Nothing above is presented as measured that was not run. The one argued step in the round's evidence is inside `R3B-2` and is marked there by the reviewer; I ran its stated falsifier and recorded the one qualification it needs.

FIXTURE HYGIENE. Everything under `<S>` and nothing else. Four fixtures (`fake-owner`, `undeclared`, `declared`, `unblock`), one scratch source tree (`mut4`) with its own `CARGO_TARGET_DIR`, and nothing written or deleted outside `<S>`. The only mode-600 files were cargo's own incremental lock files under `<S>/target-mut4`; they were chmodded to 644 before finishing, and a re-check returns zero files at mode 000 or 600. The main repository and the reviewed worktree were never edited: `git -C <worktree> status --porcelain` is empty apart from this findings file.
