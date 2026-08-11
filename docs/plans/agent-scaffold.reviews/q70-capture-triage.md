# Q-70 capture, round 1 triage

Triager verdicts on the sixteen raw findings in `q70-capture-reviewer-premises.md` (`R1A-*`), `q70-capture-reviewer-consumer.md` (`R1B-*`) and `q70-capture-reviewer-source.md` (`R1C-*`).

Artifact: `git diff main..HEAD` on `triage/q70-r1`, commits `c58427f` and `869943e`, which add the `[[question]] Q-70` entry, the `q70-capture` `[meta].orphan_tasks` line, an empty `docs/plans/agent-scaffold.questions/Q-70.md`, and the regenerated `docs/plans/agent-scaffold.md`.

Binary: `target/debug/agent-scaffold` built from this worktree at HEAD. Fixture root: `<scratch>/tri-q70/`. Every fixture was built from a fresh `cp -r docs/` or from `git ls-files` of this worktree; nothing outside `<scratch>/tri-q70/` was written or deleted; no fixture was left at mode 000 (none was created).

Result: ELEVEN VALID, ONE DISMISSED, FOUR DUPLICATE. Severity ceiling `high`, three of them. No `high` or `critical` finding is dismissed, so NO BACKSTOP RE-CHECK IS OWED for this round.

Two severities are corrected DOWNWARD from `high` to `medium` (`R1B-1`, `R1C-1`). A downgrade is not a dismissal and does not engage the backstop: both findings stand as valid.

---

## Verdict table

| id | verdict | reviewer severity | my severity | settled by |
| --- | --- | --- | --- | --- |
| R1A-1 | VALID | medium | medium | running |
| R1A-2 | VALID | medium | medium | running |
| R1A-3 | DUPLICATE OF R1C-2 | medium | (see R1C-2) | reading |
| R1A-4 | VALID | low | low | running |
| R1A-5 | VALID | low | low | running |
| R1B-1 | VALID | high | medium | running |
| R1B-2 | VALID | high | high | reading |
| R1B-3 | DISMISSED | low | low if valid | running |
| R1B-4 | DUPLICATE OF R1C-3 | low | (see R1C-3) | reading |
| R1C-1 | VALID | high | medium | running |
| R1C-2 | VALID | high | high | running plus reading |
| R1C-3 | VALID | high | high | reading, strengthened by running |
| R1C-4 | VALID | medium | medium | running |
| R1C-5 | VALID | medium | medium | running |
| R1C-6 | DUPLICATE OF R1A-2 | medium | (see R1A-2) | running |
| R1C-7 | DUPLICATE OF R1C-2 | low | (see R1C-2) | reading |

---

## Per-finding verdicts

### R1A-1. "the convention already exists at three sites" is stale

VERDICT: VALID. SEVERITY: `medium`, confirmed.

Reproduced. `grep -onE "\([0-9]+(, [0-9]+){1,6}\)" docs/plans/agent-scaffold.plan.toml` returns four lines and only four: `1331:(3, 4, 6)`, `1340:(9, 5, 6, 4)`, `1349:(11, 9, 6, 4, 5)`, `1358:(6, 4, 2, 0, 2)`. A widened form anchored on the word `findings` returns the same four lines, so the population is complete rather than sampled.

The provenance reproduces too, and it is what makes this in scope rather than inherited. `git blame -L 1349,1349` dates the fourth site to `c857fb8` (2026-08-11 12:29). `git log --date=iso main..HEAD` dates the two `Q-70` commits to 2026-08-11 17:43 and 17:53. The fourth site existed for five hours before `Q-70` was written. The relayed source, `docs/plans/agent-scaffold.ledger.md:621`, blames to 2026-08-08 and was correct on that date.

I confirm `medium` rather than `low`. The same paragraph refuses to state the dangling-receipt count on the ground that a moving count must not be asserted, then asserts a different moving count one sentence earlier. That is the project's own standing cure, self-contradicted inside one paragraph. I do not raise it further, because the consequence is bounded: a pass sizes one mechanism's population one short and would re-derive it while designing the check.

### R1A-2. "Two loops hit this" under-counts the affected population

VERDICT: VALID. SEVERITY: `medium`, confirmed.

Reproduced, twice. The reviewer's pipeline returns exactly six identities across three steps, byte-identical to its report:

```
decision-folder-currency        decision-folder-currency-fold
workflow-driver                 workflow-driver-stage0a
workflow-driver                 workflow-driver-stage0b
workflow-driver                 workflow-driver-stage1
workflow-enforcement-tier       workflow-enforcement-tier-endproperty-fold
workflow-enforcement-tier       workflow-enforcement-tier-fold
```

The declared half is the material half and I rebuilt its fixture independently rather than accepting the reviewer's. A fresh `cp -r docs/` with one `[[step.waiver]]` injected on the live `workflow-driver` step naming its own declared increment `workflow-driver-stage0a` (`unit = "increment"`, `reason = "review-skipped"`, `evidence_tier = "self-declared"`, no `evidence`) returns EXACTLY ONE problem at exit 1:

```
TOML waiver `workflow-driver-w1`: increment waiver names step `workflow-driver` but increment `workflow-driver-stage0a` belongs to step `workflow-driver-stage0a`
```

The `src/plan/source.rs` membership path is silent because the increment IS declared. `docs/plans/agent-scaffold.plan.toml:688-698` declares all three `workflow-driver-stage*` ids, and the step is `in-progress`, so W3 skips it today (`src/workflow.rs:445-447`) and the case is latent rather than firing.

`medium` confirmed. The item is the pass's measured input on the defect's shape and states that shape at one third of its size, but the direction of the error is conservative (the real population is larger, not smaller) and a pass that measures anything will find it.

### R1A-3. Escape route 4's "never reads `step.increments`" describes a field that does not exist

VERDICT: DUPLICATE OF R1C-2.

The claim is true, and I verified it directly: `w5_problems(waivers: &[Waiver], steps: &[Step], escalations: &[Escalation])` at `src/workflow.rs:544-548` takes `plan::Step`, which is `pub struct Step { pub slug: String, pub status: String }` at `src/plan.rs:55-60` and has no increment field, and `grep -c increment src/plan.rs` returns 0.

It is the same defect as `R1C-2`: one sentence in escape route 4 attributes to W5 a choice not to look, when there is nothing to look at, and the consequence in both statements is that the narrow-lookup direction's edit surface is understated. Same site (`docs/plans/agent-scaffold.plan.toml:1887`), same fix. `R1C-2` is the better statement, because it carries what `R1A-3` only implies: the `status --json` output contract, the Markdown substrate that cannot supply the lookup's input at all, and the parity property in `src/next.rs:517-523` that the direction puts under opposite pressure. Adjudicated under `R1C-2`; remedy B site 1 is `R1A-3`'s fix.

### R1A-4. "THE DURABLE RECORD SAYS FOUR, NOT THREE" is not what the durable record says

VERDICT: VALID. SEVERITY: `low`, confirmed.

Reproduced. The ledger says both, and both "three" passages are dated the same day as `Q-70`:

- `docs/plans/agent-scaffold.ledger.md:533`, the `Q-55-entryroute` decision record that `Q-70`'s own opening cites as its authority: "the two inc3 defects plus the three `next` defects are already-diagnosed point defects with NO OPEN DESIGN SPACE". `git blame` gives `903b70b8`, 2026-08-11.
- `docs/plans/agent-scaffold.ledger.md:557`, the current next-action paragraph, item (4): "the three `agent-scaffold next` defects routed here by an earlier human decision". `git blame` gives `8fa56939`, 2026-08-11.
- `docs/plans/agent-scaffold.ledger.md:967` does begin "A FOURTH `agent-scaffold next` DEFECT" and does contain "belongs to the validation-constraints step with the other three". `git blame` gives `a46cd97d`, 2026-08-01.

The `blocked_by` re-measurement `Q-70` relies on also reproduces first-hand: `grep -c "^blocked_by = \[\]"` returns 95 against `grep -c "^\[\[step\]\]"` 95, and a filter for any populated `blocked_by` line returns nothing.

`low` confirmed. The count `Q-70` states is the correct one, so nothing downstream is mis-scoped; what is false is the claim about where the wrong count lives, and the cost is that two live ledger "three"s are not flagged as owed corrections.

### R1A-5. `src/plan/source.rs:791-843` under-cites the block it names

VERDICT: VALID. SEVERITY: `low`, confirmed.

Reproduced by measuring the block rather than by reading around it. The per-step waiver loop opens at `src/plan/source.rs:791` (`for step in &plan.steps {`), the inner waiver loop at `:794`, and the outer loop closes at `:856`. The cited range stops at `:843`, which is the close of the `evidence` presence match. The `reason` to `evidence_tier` pairing check is at `:844-854` (`if waiver.reason.required_tier() != waiver.evidence_tier {` at `:846`), outside the cited range.

That matters inside the same `Q-70` paragraph, which says the block's introducing comment "reaches the presence rules and the pairing, both of which genuinely hold, the pairing single-sourced through `WaiverReason::required_tier`". A reader who opens the cited range as instructed finds the presence rules and no pairing. The paragraph's two other citations are exact and I confirmed both: the increment set is built at `:792-793`, the membership check is at `:807-811`.

### R1B-1. The opening `ask` frames the pass at half the mandate `Q-55-entryroute` decided

VERDICT: VALID. SEVERITY: `medium`, CORRECTED DOWN from `high`.

Reproduced. The receipt is real and its option set is as quoted:

```
{"type":"decision","task":"validation-constraints","q_id":"Q-55-entryroute","options":["Design pass, validator cluster only","Planner straight to the step","Design pass over all four bodies","Split out the W5 fix first"],"recommendation":"Design pass, validator cluster only","chosen":"Design pass, validator cluster only","ts":"2026-08-11"}
```

`docs/plans/agent-scaffold.ledger.md:533` states what the chosen option means: "A DESIGN PASS OVER THE VALIDATOR CLUSTER ONLY, meaning the W5 fix plus the three detection mechanisms". `Q-70`'s opening sentence at `docs/plans/agent-scaffold.plan.toml:1883` names the W5 fix and its coupling with one of the three. Both citations resolve exactly.

Severity corrected to `medium`. The finding is a framing claim, and the framing is real, but the item does carry the fuller scope, under a heading a reader cannot miss: "THE THREE DETECTION MECHANISMS IN THE PASS'S SCOPE" at `:1893`. The failure this finding describes requires an explorer who forms a scope from the first sentence of their own brief and does not revise it three paragraphs later. That is a plausible reader, not the likely one. The operative defect is that the mandate is never consolidated anywhere, which is `R1B-2` and which I hold at `high`; this finding is the second site of it.

### R1B-2. "WHAT THE PASS OWES BACK" omits three duties the item's own body makes mandatory

VERDICT: VALID. SEVERITY: `high`, confirmed.

Settled by reading the item, and every cited line resolves (I checked `:1883`, `:1889`, `:1893`, `:1895`, `:1897`, `:1899`, `:1901`, `:1903` individually). "WHAT THE PASS OWES BACK" at `:1901` asks for three things: an explicit ruling on the coupling hypothesis, the edit surface, and a YAGNI boundary. The item states these duties elsewhere and does not restate them there:

1. "WHAT THE PASS OWES ON THIS: a ruling on WHICH PATH IS AUTHORITATIVE for waiver ownership, or whether both should be", `:1889`.
2. "The pass must therefore STATE WHICH CHECK IT MEANS wherever it writes 'W6'", `:1897`.
3. Anything at all on mechanisms 2 and 3, which `:1893`'s own heading places "IN THE PASS'S SCOPE" and which `:1899` contrasts with the items explicitly out of it.

The finding UNDER-counts if anything: `:1893` also carries a fourth duty, that the pass "must rule on whether those are dangling receipts or a legitimate sub-decision convention the check has to model", and that is not in the closing paragraph either. `Q-68`'s precedent at `:1857` is exactly the consolidated form this item lacks: "OPEN DESIGN QUESTIONS the pass must resolve (none pre-decided): (a) ... (e)".

`high` confirmed. This is the finding with the largest causal reach in the round: a proposal can satisfy the paragraph that reads as the deliverables checklist, in full, and still omit four of the pass's rulings. The consumer of this document is an explorer working to a checklist, and the checklist is wrong.

### R1B-3. The Design-explorations citation drops the line number `Q-69` carries

VERDICT: DISMISSED. Severity had it been valid: `low`. No backstop is engaged; the backstop covers dismissals at `high` or above.

The facts reproduce. `grep -n "Design explorations" pack/AGENTS.md AGENTS.md` returns `65:` for both, `Q-69` cites `pack/AGENTS.md:65` at `:1876`, and `Q-70` cites `pack/AGENTS.md` at `:1901`. I dismiss on the defect, not on the evidence.

Ground, in three parts. First, `Q-70` states the opposite convention for itself, twice, and both are in this diff: "find them by the quoted text ... rather than by a line number" at `:1891`, and "Find all of these by their quoted text rather than by a line number" at `:1899`. A citation form the item explicitly adopts is not a regression against precedent, it is the precedent the item chose. Second, `AGENTS.md` is a GENERATED file, regenerated from `pack/` by `just scaffold-self` on every pack edit, so its line numbers move whenever unrelated pack prose changes. Third, this repository has already measured that class: the third detection mechanism in this very item exists because roughly eleven `src/checks.rs` line citations went stale, and the item records that a quotation resolver "would immediately go red" on them. A line-numbered citation to a generated file is the failure mode, not the standard.

The stated impact, "one extra search in a roughly 150-line file", is not a defect with a cost. Dismissed.

### R1B-4. "This item carries NO options" sits next to a sentence naming two candidate shapes

VERDICT: DUPLICATE OF R1C-3.

Both quoted texts are accurate: "This item carries NO options and NO recommendation, deliberately" at `:1883`, and "the answer decides the shape of the fix: a narrow lookup ... or a rework of how a waiver names its unit" at `:1895`. `Q-69`'s contrasting label at `:1872` is real and reads as quoted.

Same defect, same site, same fix as `R1C-3`: one sentence presents two named shapes as the space, in a document that says it carries none. `R1C-3` is the better statement because it measures that the space the sentence closes contains a real third direction, where `R1B-4` reads it as a labelling wrinkle and explicitly discounts it as "not a practical steer". On the evidence `R1C-3` is right and `R1B-4` under-rates its own finding. Adjudicated under `R1C-3`; remedy D discharges both, and its first clause is `R1B-4`'s fix verbatim.

### R1C-1. The declared-increment namespace covers under half the increment identities, and two cannot be declared

VERDICT: VALID. SEVERITY: `medium`, CORRECTED DOWN from `high`.

Reproduced exactly. The reviewer's measurement returns 94 distinct round increment identities, 43 declared, 51 undeclared; of the 51, 24 are exactly a step slug, 14 are `[meta].orphan_tasks` tokens, and the remaining 13 are the same thirteen the reviewer lists, in the same order. The two further shapes reproduce: `round-log-core` is `complete` and declares ZERO increments while its rounds carry `round-log-core-incA` and `-incB`; `optional-modules` declares exactly one increment, `optional-modules-inc2cii`, which is exactly the one it waives, while five more of its identities appear only in the log.

The undeclarable pair I settled by MEASUREMENT rather than by citation, because the reviewer settled it by citation and it is the sharper half of the claim. A fresh `cp -r docs/` with `round-log-core`'s `increment = []` replaced by a real `[[step.increment]]` entry `id = "round-log-core-incA"` returns, at exit 1:

```
increment id `round-log-core-incA` is not a well-formed kebab-case id
```

So the `-incA` / `-incB` form that `src/workflow.rs:64-68` documents the strip as existing for cannot be entered into the set the "narrow lookup" direction would key on. That is a hard structural exclusion, confirmed against the tool.

Severity corrected to `medium`. The finding is an OMISSION, not a false claim: the item nowhere asserts that the declared set models the plan's increments, it only names the direction. The exclusion is real and the coverage is genuinely poor, but an explorer instructed to rule on coupling and to state an edit surface reaches this measurement in the course of the work, and the item's job is to register inputs rather than to complete the analysis. It belongs in the item, which is why it is valid; it does not carry the reach of `R1C-2` or `R1C-3`, which change what an explorer is told to do rather than what they are told about.

### R1C-2. W5 cannot perform the declared-increment lookup without widening a shared, serialised, cross-substrate type

VERDICT: VALID. SEVERITY: `high`, confirmed. This is the primary of the three-finding duplicate group with `R1A-3` and `R1C-7`.

Every structural citation verified at the line:

- `w5_problems(waivers: &[Waiver], steps: &[Step], escalations: &[Escalation])` at `src/workflow.rs:544-548`, whose `Step` is `plan::Step`.
- `plan::Step` at `src/plan.rs:55-60` carries `slug` and `status` and nothing else.
- `PlanToml::step_views()` at `src/plan/source.rs:422-430` maps only `slug` and `status` and DROPS the increments.
- `grep -c increment src/plan.rs` returns 0. `src/next.rs:520` and `:551` state it in the code's own words, "the Markdown substrate (which declares no increments)" and "The Markdown substrate declares no increments".
- `plan::Step` is `Serialize` and is the `status --json` payload through `PlanProjection.steps: Vec<plan::Step>` at `src/main.rs:582-585`, so widening it changes a machine output contract.
- `src/next.rs:517-523` records that the declared `[[step.increment]].risk_class` is deliberately NOT carried into the projection so that "the Markdown substrate ... produces an identical projection to the TOML one (the parity property)".

The runnable half reproduces: a two-line Markdown plan plus a log carrying a JSONL increment waiver returns, at exit 1, `round log line 2: increment waiver names step 'alpha' but increment 'alpha-stage0a' belongs to step 'alpha-stage0a'`, so W5's ownership rule fires on a substrate where no declared-increment set exists to consult. ONE IMPRECISION IN THE REVIEWER'S OWN REPORT, which does not change the verdict: my rebuild emits TWO problems, the W5 one plus `Roadmap step 'alpha' has no matching '### 'alpha'' Step Detail heading`, and the reviewer quoted only the first. The finding's claim is unaffected.

`high` confirmed. The item requires each proposal to "state the edit surface its direction implies (naming which source files it touches, and in particular whether any generated const or drift-guarded file is involved)" and then frames one of the two directions it forces a choice between as a lexical test at a single line. The real surface is a shared type, a JSON output contract, a second substrate that cannot supply the input, and a documented parity property under opposite pressure. This mis-prices one side of the pass's central comparison, in the one dimension the item itself asks proposals to price.

### R1C-3. The item imposes a binary the code and the durable record both exceed

VERDICT: VALID. SEVERITY: `high`, confirmed. Primary of the duplicate group with `R1B-4`.

Settled by reading the code, and I then found evidence the reviewer did not use, which strengthens it materially.

The code half verified at the line. W3's covering-waiver match is `waiver.increment.as_deref() == Some(*increment) && waiver.step == step.slug` at `src/workflow.rs:498-502`, where `increment` is `round_increment_id(round)`, "the structured `increment` id when the record carries one, else its `task` verbatim" (`src/workflow.rs:127-129`). There is no lexical strip in that match. `run_checks` at `src/workflow.rs:206-221` already holds `rounds` and already hands them to `w3_problems`; it simply does not hand them to `w5_problems`. Both entry points feed the same funnel from `metrics::parse_rounds` (`check_workflow_toml` at `src/workflow.rs:180-195`). So an ownership rule stated against the round log needs no new data source, no type change and no substrate fork, and it is neither of the two shapes the item names.

THE EVIDENCE THE REVIEWER MISSED, and the reason I hold this at `high` without hesitation. `docs/plans/agent-scaffold.ledger.md:557` states the W5 fix, in the paragraph a resuming agent reads first, as: "(1) THE W5 FIX, teaching W5 the structured step association W3 already uses". That IS the third direction. The durable record's own statement of the fix is the option `Q-70`'s binary excludes, and `Q-70` at `:1895` instructs each proposal that it "must state which it is choosing" between the other two.

The scope rule that a finding may not fault this item for failing to present options does not reach this. The complaint is the reverse: an `exploring` item that says at `:1883` it "carries NO options and NO recommendation, deliberately" closes the design space to two named shapes at `:1895` and makes choosing between them mandatory. That is a steer, and the scope rules admit a steer as a valid finding.

I note for the record what the item was relaying. `docs/plans/agent-scaffold.ledger.md:533` phrases the human's ground as "the choice between a lookup against the step's declared increments and a rework of waiver-unit naming must be made with W6 in view". That is a ground for running a pass, phrased as a contrast. `Q-70` hardens it into an instruction to pick one of two. The remedy therefore does not overturn the human's framing; it restores the two shapes to the status the ledger gives them.

### R1C-4. A third live waiver-validation path exists and the item refers to it only in the past tense

VERDICT: VALID. SEVERITY: `medium`, confirmed.

Both fixtures reproduce byte-for-byte. A log-only file with a `type:"waiver"` record missing `increment` returns `w.jsonl:1: missing field 'increment'` at exit 1, so `check_record`'s waiver arm is live and reachable. A log-only file whose waiver names `"step":"totally-made-up-step"` and `"increment":"some-other-step-inc1"` returns `1 records, valid` at EXIT 0: that path performs no ownership check at all.

The third-schema claim verified: `waivers_from_toml` at `src/workflow.rs:237-267` flattens the TOML typed struct into the `metrics::Waiver` shape W3 and W5 consume, so the "rework how a waiver names its unit" direction touches three representations, not one. `check_workflow_toml` reads `waivers_from_toml(plan)` alone (`src/workflow.rs:180-195`), and this repository's own log carries zero waiver records (`grep -c '"type":"waiver"' docs/metrics/workflow.jsonl` returns 0) while the plan carries 25 TOML waivers.

One clarification on what is and is not the defect. `Q-70` does NOT claim the arm was retired; it faithfully relays what the comment at `src/plan/source.rs:785-790` says ("rules moved from"), and the reviewer concedes escape route 2 is correctly substrate-scoped. The valid part is the second half: the item prices one of its two directions at one schema when the tool carries three. That is why this is a distinct defect from `R1C-2` rather than a duplicate of it: `R1C-2` under-prices direction ONE, `R1C-4` under-prices direction TWO, and fixing either leaves the other wrong. They share remedy B and are separate sites within it.

`medium` confirmed.

### R1C-5. The fix's edit surface includes drift-guarded generated files the item points at neither

VERDICT: VALID. SEVERITY: `medium`, confirmed.

I rebuilt the mutation rather than accepting it, in a scratch copy of the tracked tree, because I may not edit source in this worktree. `git ls-files` copied to `<scratch>/tri-q70/mut/`, a separate `CARGO_TARGET_DIR`, baseline `cargo test` GREEN at 378 lib tests plus every integration binary, 0 failures. Changing the W5 clause in `pack/instrument.md` ALONE, from "(the increment's leading slug equals the step)" to "(the increment is one of the step's declared increments)", then re-running:

```
test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... FAILED
test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 373 filtered out
```

The mutation was reverted in the scratch copy and the full suite returns to 378 passed, 0 failed. The main worktree was never edited.

The guarded set is as stated at `src/agents_md_drift.rs:41-55`, and `LC_ALL=C grep -n "must own its .increment." AGENTS.md .agents/AGENTS.reference.md pack/instrument.md` hits all three at `AGENTS.md:147`, `.agents/AGENTS.reference.md:147` and `pack/instrument.md:11`. The second direction's equivalents also check out: `pack/plan-template.plan.toml` carries the commented `[[step.waiver]]` example from line 39, and `waiver_note` in `src/plan/render.rs:516-529` writes `waived: increment '<id>' ...` into the generated Markdown, which `render --check` pins. Two of the reviewer's line ranges are slightly loose (`src/plan/render.rs:513-529` for a function that opens at `:516`; `pack/plan-template.plan.toml:39-44` for a block that runs to about `:49`), which does not change the verdict.

`medium` confirmed. The item asks proposals a question it does not answer for itself, and the cost lands after the design pass, when a step authored from a compliant proposal fails `just test` on a guard nobody costed.

### R1C-6. A third instance of the blocker is already latent on declared increments

VERDICT: DUPLICATE OF R1A-2.

The fixture reproduces, and it is the same fixture I built for `R1A-2`: one self-declared increment waiver on `workflow-driver`'s own declared `workflow-driver-stage0a` returns exactly one problem, from W5, at exit 1, with the source-path membership check silent.

Same defect as `R1A-2`: the item states the blocker's population as two orphan fold tokens when the live plan already carries the same shape on declared increments. `R1A-2` is the better statement because it derives the whole population by one reproducible command and reaches a case in a third step (`decision-folder-currency-fold`) that `R1C-6` does not name, where `R1C-6` reaches only the three `workflow-driver` ids.

`R1C-6` carries ONE thing `R1A-2` does not, and it is not lost: the consequence for the second direction, that keeping the lexical rule would mean renaming three ALREADY-DECLARED increments rather than two orphan tokens. Remedy A site 2 carries that sentence explicitly.

### R1C-7. The routed writer item resolves on the code, via a type distinction the item never names

VERDICT: DUPLICATE OF R1C-2.

The type distinction is real and I verified both types: `plan::Step` at `src/plan.rs:55-60` with `slug` and `status`, and the TOML `Step` at `src/plan/source.rs` carrying `increments`, which `validate_source` walks at `:791-793`. The observation that route 4's phrasing "reads as a behavioural choice" and that the type-level statement is stronger is correct.

It is the same defect and the same site as `R1A-3` and `R1C-2`, and its stated impact ("a reader of route 4 can reasonably infer W5 could read the declared increments and simply does not") is `R1A-3`'s impact in different words. `R1C-2` is the primary. Remedy B site 1 is written at the type level, which is `R1C-7`'s prescription.

---

## The item the writer raised against itself

RULED. The planner reported that escape route 4 and the later second-path paragraph state route 4's evidence at two different scopes, named two possible closures (narrow route 4 to `src/workflow.rs` explicitly, or leave it as written with the later paragraph as the labelled correction), and declined to choose. All three reviewers addressed it and none is the deciding authority, so I rule it.

AS RAISED, IT IS NOT A DEFECT, AND NEITHER CLOSURE IS OWED. Checked first-hand. Every clause of route 4 names W5 as its subject ("W5's check is lexical", "`w5_problems` derives only a slug set from the steps", "`src/workflow.rs:549`"), and none of them claims that no structural lookup exists anywhere in the tool. The later paragraph opens by declaring its relation to route 4 ("MEASURED AFTER ESCAPE ROUTE 4 WAS WRITTEN AND BEARING DIRECTLY ON IT"), names the other path by file, and closes by confirming route 4 rather than correcting it. Two true statements about two different functions in two different modules are not a scope mismatch to close.

Route 4's operative conclusion also holds against my own from-scratch fixtures, which I rebuilt rather than reusing the reviewer's. Undeclared, the live shape: both paths fire on one waiver at exit 1, with the two problem strings byte-identical to the two `Q-70` quotes. Declared, the same fixture plus a `[[step.increment]]` for the fold token: the `src/plan/source.rs` problem disappears, the W5 problem still fires, exit 1. "ONE REFUSAL PLUS ONE PASS" reproduces exactly as recorded, so "declaring the fold tokens does not help" is confirmed by measurement.

THE ONE REAL DEFECT IN THAT SENTENCE IS A DIFFERENT ONE, and the writer did not raise it: "never reads `step.increments`" attributes a choice to W5 where no field exists to read. That is `R1C-2` (with `R1A-3` and `R1C-7`), it is valid at `high`, and remedy B site 1 fixes it. Fixing it also happens to make the planner's question moot, because a type-level statement cannot be read at the wrong scope.

For the writer's record: declining to choose was the right call on the wrong question. There was nothing to choose between, and the sentence still needed changing for an unrelated reason.

---

## Deduplication map

| duplicate | primary | why |
| --- | --- | --- |
| R1A-3 | R1C-2 | Same site (`plan.toml:1887`), same claim (route 4 attributes a non-lookup to W5 where no field exists), same fix. R1C-2 adds the JSON contract, the Markdown substrate and the parity property. |
| R1C-7 | R1C-2 | Same site and same claim as R1A-3, stated at the type level. Its prescription IS remedy B site 1. |
| R1B-4 | R1C-3 | Same site (`plan.toml:1895`), same claim (two named shapes against a stated "NO options"), same fix. R1C-3 measures that the closed space holds a real third direction; R1B-4 discounts its own finding as "not a practical steer" and is wrong to. |
| R1C-6 | R1A-2 | Same defect and the same fixture. R1A-2 derives the whole population in one command and reaches a third step; R1C-6's extra consequence for the second direction is carried into remedy A site 2. |

GENUINELY DISTINCT, though they look adjacent, and each stated because collapsing them would lose a site:

- `R1C-2`, `R1C-4` and `R1C-5` are three separate under-priced surfaces, not one. `R1C-2` under-prices direction one (the shared type), `R1C-4` under-prices direction two (three waiver schemas), `R1C-5` under-prices BOTH (drift-guarded generated files). Fixing any one leaves the other two wrong. They share remedy B as three of its four sites.
- `R1A-2` and `R1C-1` measure different things about the same relationship. `R1A-2` counts the identities W5 CANNOT OWN (six). `R1C-1` counts the identities the declared set DOES NOT COVER (51 of 94) and the two it structurally cannot. Correcting the blocker's population supplies neither the coverage figure nor the uppercase exclusion.
- `R1B-1` and `R1B-2` are two sites of one class (the mandate is never consolidated), and neither fix implies the other: correcting the opener leaves the checklist short of four duties, and consolidating the checklist leaves the opener under-framing the pass at its entry point. Both are carried by remedy C.

---

## Remedies

Each remedy is scoped to its CLASS over the whole enclosing sentence and paragraph, not to the quoted fragment. Every site any reviewer named is accounted for, including the sites I decide to leave alone.

### Remedy A. Every moving population in this item follows the rule the item already applies to one of them

Discharges `R1A-1`, `R1A-2` (with `R1C-6` folded in), `R1C-1`.

The class: `Q-70` refuses to state the dangling-receipt count, cites the project's standing cure for refusing it, and then states or implies three other populations of things that keep moving. Apply the same rule to all of them. DO NOT REPLACE "three" WITH "four" OR "two" WITH "six": a corrected figure is the same defect with a later expiry date.

Site 1, `plan.toml:1893`, mechanism (1): delete "the convention already exists at three sites and only the enforcement is missing". State the property (the breakdown is carried in the `note` field of `[[step.waiver]]` entries) and give the reproduction, `grep -onE "\([0-9]+(, [0-9]+){1,6}\)" docs/plans/agent-scaffold.plan.toml`. Say that the population grows with each escalated loop, which is why no count is stated.

Site 2, `plan.toml:1885`, THE BLOCKER's "Two loops hit this": keep both named tokens, since they are the two that BLOCK a step today, but stop presenting them as the population. State the property (any identity whose `leading_slug` is not the step it joins under W3) and give the reproducing pipeline. In the same paragraph record, from `R1C-6`, that the population already includes DECLARED `[[step.increment]]` ids, that `workflow-driver` is the live case and is latent only because the step is not `complete` (`src/workflow.rs:445-447`), and that this is what keeping the lexical rule would cost: renaming already-declared increments, not just orphan fold tokens.

Site 3, `plan.toml:1895`, the coupling paragraph's "the step's declared `[[step.increment]]` set": add, as a measured input and not as a figure, that this set is not a model of the plan's increments. It is currently a by-product of the membership rule at `src/plan/source.rs:807`; `complete` steps exist that declare none while their rounds carry increment ids; and an increment id cannot contain an uppercase byte (`is_kebab_case_token`, `src/plan/source.rs:475-477`, pinned by `an_uppercase_increment_id_is_flagged`), so the `-incA` / `-incB` form the round log uses, and that `src/workflow.rs:64-68` documents the strip as existing for, CANNOT BE DECLARED AT ALL. Give the reproduction script rather than the coverage ratio.

Sites left alone, with a verdict for each:

- `plan.toml:1893`, the dangling-receipt paragraph's deliberate refusal to state a count. CORRECT AS IT STANDS, and it is the model the other three sites must follow. Do not touch it. I re-measured its command first-hand: 62 distinct receipt `q_id`s against 70 registered questions, 40 dangling, and every one of the 40 is a `Q-55-<suffix>` id, so the item's "dominated by" is if anything conservative.
- `plan.toml:1887`, escape route 1's "these two have five each". LEAVE. It is a fact about two named tasks, not a population claim, and W3's step-unit branch keys on the STEP rather than the task, so the conclusion is unaffected either way.
- `plan.toml:1899`, the `blocked_by` re-measurement. LEAVE. It reproduces (95 of 95, zero populated), and it is stated as a measurement made for this registration rather than as a durable count.

### Remedy B. State the real edit surface of both directions, in the item that demands one

Discharges `R1C-2` (with `R1A-3` and `R1C-7` folded in), `R1C-4`, `R1C-5`.

The class: the item requires every proposal to name the source files its direction touches and to say whether any generated or drift-guarded file is involved, while naming only `src/workflow.rs` and `src/plan/source.rs` anywhere in itself, and framing one direction as a change at a single line.

Site 1, `plan.toml:1887`, escape route 4's clause "`w5_problems` derives only a slug set from the steps (`src/workflow.rs:549`) and never reads `step.increments`": restate at the TYPE level. `w5_problems` is handed `plan::Step` (`src/plan.rs:55-60`), which carries `slug` and `status` only, so there is no increment field to read and no choice being made. This STRENGTHENS route 4's conclusion, and it is also the fix for `R1A-3` and `R1C-7`.

Site 2, same paragraph or the coupling paragraph: state what the narrow-lookup direction costs. `PlanToml::step_views()` (`src/plan/source.rs:422-430`) drops the increments; `plan::Step` is `Serialize` and is the `status --json` payload (`src/main.rs:582-585`), so widening it changes a machine output contract; the Markdown substrate declares no increments at all (`src/next.rs:520`, `:551`), so the direction owes a ruling on what W5 does there; and `src/next.rs:517-523` records a parity property that deliberately withholds declared-increment data from the projection, which this direction pushes the other way.

Site 3, `plan.toml:1895`, the second direction: name its three representations rather than implying one. The JSONL `check_record` waiver arm (`src/metrics.rs:539-601`), the TOML typed struct (`src/plan/source.rs:279-300`), and the `waivers_from_toml` flattening that reconciles them (`src/workflow.rs:237-267`). Record that the JSONL arm is LIVE and performs NO ownership check, and that under `[meta].primary = "toml"` a JSONL waiver record grants nothing and is reported by nothing.

Site 4, `plan.toml:1901`, the item's own edit-surface question: answer it for the rule being changed. The W5 ownership clause is stated verbatim in `pack/instrument.md:11` and in the two drift-guarded files `AGENTS.md:147` and `.agents/AGENTS.reference.md:147` (guard at `src/agents_md_drift.rs:41-55`), so a change to the rule must move all three together or `the_committed_scaffold_matches_a_fresh_render` fails; regeneration is `just scaffold-self`, whose second line is `nix fmt` over the whole tree. Add the second direction's equivalents: the commented `[[step.waiver]]` example in `pack/plan-template.plan.toml` and `waiver_note` in `src/plan/render.rs`, which `render --check` pins.

Site left alone, with a verdict: `plan.toml:1887`, escape route 2's "NOT AUTHORABLE AT ALL in the TOML flow". CORRECT AS WRITTEN and correctly substrate-scoped; `R1C-4` concedes this in terms. Do NOT weaken it to accommodate the JSONL half. Put the JSONL half under site 3, where it belongs to the direction it prices.

### Remedy C. Consolidate the pass's mandate into one list, and let the opener name it

Discharges `R1B-1`, `R1B-2`.

Site 1, `plan.toml:1901`, "WHAT THE PASS OWES BACK": replace the three-item sentence with a consolidated list of everything the pass must resolve, in the shape `Q-68` uses at `:1857`. It must carry, at minimum: the coupling ruling; the authoritative-path ruling (which of W5 and `src/plan/source.rs` owns waiver ownership, or both), currently only at `:1889`; the duty to state which check is meant by "W6", currently only at `:1897`; the ruling on whether the `Q-55-<suffix>` sub-decision ids are dangling receipts or a convention the check must model, currently only at `:1893`; whether mechanisms 2 and 3 are DESIGNED in this pass or only BOUNDED by it, which the item has never said either way; the edit surface (see remedy B); and the YAGNI boundary.

Site 2, `plan.toml:1883`, the opening `ask` sentence: name the mandate `Q-55-entryroute` decided, the W5 fix PLUS the three detection mechanisms, rather than the W5 fix and its coupling with one of them. Cite the receipt, and note that "Split out the W5 fix first" was among the rejected options, so the narrow reading is the option that was declined.

Site left alone, with a verdict: `plan.toml:1893`, "THE THREE DETECTION MECHANISMS IN THE PASS'S SCOPE". LEAVE the heading and the in-scope declaration as they are; they are the only place the mandate is currently complete and they are correct. Remedy A site 1 edits its first mechanism's count, and remedy C site 1 lifts its sub-decision ruling into the consolidated list. Nothing else there needs to change.

### Remedy D. Reopen the design space the item closed

Discharges `R1C-3` (with `R1B-4` folded in).

Single site, `plan.toml:1895`, the whole sentence "the answer decides the shape of the fix: a narrow lookup ... or a rework of how a waiver names its unit. Each proposal must state which it is choosing and why, and must say what the other mechanism costs under that choice."

1. Relabel the two shapes as NON-EXHAUSTIVE candidates, reusing the wording `Q-69` already carries at `:1872` ("CANDIDATE DIRECTIONS for the pass to weigh, extend, or discard. NOT a decided option set, no recommendation attached"). This alone discharges `R1B-4`'s inconsistency against `:1883`'s "NO options".
2. Replace "must state which it is choosing" with a requirement to state which direction the proposal takes AND whether it is one of the named candidates or outside them.
3. Name the third direction the code and the durable record both already carry, WITHOUT recommending it: an ownership rule stated against the round log rather than the plan, keyed on `round_increment_id` plus the step exactly as W3's covering-waiver match already is (`src/workflow.rs:498-502`), needing no new data source because `run_checks` already holds `rounds` (`src/workflow.rs:206-221`). Record that `docs/plans/agent-scaffold.ledger.md:557` states the W5 fix as "teaching W5 the structured step association W3 already uses", which is this direction and neither of the two the item names.
4. Keep "must say what the other mechanism costs under that choice", which is the coupling duty and is correct.

### Remedy E. Two citation corrections

Discharges `R1A-4`, `R1A-5`.

Site 1, `plan.toml:1899`, "THE DURABLE RECORD SAYS FOUR, NOT THREE, and the brief that commissioned this registration said three": the durable record says BOTH. Rewrite the whole clause: the ledger carries "three" at `:533`, inside the `Q-55-entryroute` decision record this item cites as its own authority, and at `:557`, in the current next-action paragraph, both dated 2026-08-11; it carries "four" at `:967`; four is the measured count; and the two ledger "three"s are therefore owed a correction. Keep the four, keep the `blocked_by` re-measurement, and keep the find-by-quoted-text instruction.

Site 2, `plan.toml:1889`, "`src/plan/source.rs:791-843`": the per-step waiver loop runs `:791-856` and the pairing check the same paragraph attributes to the block is at `:844-854`, outside the cited range. Either cite the whole block or cite the range and drop the pairing claim from that sentence. The paragraph's other two citations, `:792-793` for the increment set and `:807-811` for the membership check, are exact and stay.

---

## Overall assessment

WHAT THE ROUND'S REAL RESULT IS. Eleven valid findings, ceiling `high`, three at `high`, on a document whose factual spine is sound. I re-ran the item's own two recorded fixtures from scratch and both reproduce byte-for-byte, including the exact problem strings and the exit codes; I re-derived its structural citations at the line and every one resolves; I re-measured its round-record counts, its streak figures, its escalation join, its `W6` occurrence count and its `blocked_by` claim, and all of them hold. NO FINDING IN THIS ROUND SHOWS `Q-70` ASSERTING SOMETHING THE TOOL CONTRADICTS. The valid findings are, without exception, about what the item OMITS, what it UNDER-STATES, and what it INSTRUCTS. That is the correct result for a registration whose consumers are explorer agents: the document is true and its brief is incomplete.

ONE SYSTEMIC DEFECT OR MANY. Two systemic ones and a residue, not sixteen independent defects.

The first is A MOVING POPULATION STATED AS A FIXED FIGURE. The item diagnoses this class explicitly for the dangling-receipt count, cites the project's own standing cure for it, refuses to state that count, and supplies a command instead. Then it states "three sites" (measured four, and the fourth landed five hours before the item was written), states "two loops" (measured six identities across three steps), and treats the declared-increment set as an unproblematic key (measured 43 of 94, with two identities structurally undeclarable). `R1A-1`, `R1A-2`, `R1C-1` and `R1C-6` are four faces of one defect: the cure was applied at the site where it was noticed and nowhere else. That matches the standing caution this loop carries. The brief that commissioned `Q-70` carried two figures from durable records and both were wrong when measured; the item corrected both and then reproduced the same class three more times with different numbers. Remedy A is written to close the class rather than the instances, which is why it forbids substituting a corrected figure.

The second is THE BRIEF IS SCATTERED AND ITS INSTRUCTIONS EXCEED ITS EVIDENCE. `R1B-1` and `R1B-2` are the scattering: the mandate appears in three places and is complete in none of them, and the paragraph that reads as the deliverables checklist carries one of five duties. `R1C-3` is the excess: an item that opens by saying it carries no options closes the design space to two named shapes and makes choosing between them mandatory, and the excluded space contains the direction the ledger's own next-action paragraph names as the W5 fix. `R1C-2`, `R1C-4` and `R1C-5` are the same excess seen from the source: the item demands an edit surface from every proposal and prices both of its own directions at a fraction of theirs.

The residue is two citation defects, `R1A-4` and `R1A-5`, both low, both ordinary.

WHAT THE THREE LENSES COLLECTIVELY MISSED. Three things, none of them a finding I am manufacturing, all measured.

1. `q70-capture` IS THE ONLY DECLARED ORPHAN TASK WITH NO ROUND RECORD. The field is documented as "tasks that appear in the round log but own no Roadmap step, declared here so they are visible rather than inferred" (`src/plan/source.rs:113-116`). Measured against the log: sixteen of the seventeen declared tokens appear in it; `q70-capture`, the one this diff adds, does not. No check fires either way, and the entry becomes true as soon as this loop's rounds are logged, so I read it as a deliberate pre-declaration rather than a defect. It is worth recording that the diff's third change was the one no lens examined at all: all three reviewers went at the `ask` prose, and none at the `[meta]` line or the empty sidecar. The sidecar is correct, incidentally: all 70 question sidecars are zero bytes.
2. THE STRONGEST EVIDENCE FOR THE ROUND'S SHARPEST FINDING WAS IN THE LEDGER AND NO LENS OPENED IT. `R1C-3` argues the foreclosed third direction from the code alone. `docs/plans/agent-scaffold.ledger.md:557` states the W5 fix as "teaching W5 the structured step association W3 already uses", which IS that direction, in the paragraph a resuming agent reads first. The premise lens read the ledger for counts, the consumer lens read it for the entry-route decision, the source lens did not read it. A lens pointed at THIS ITEM AGAINST THE LEDGER IT SUMMARISES, rather than against the code it cites, would have found this in one grep, and would probably also have found `R1A-4` faster.
3. AN OUT-OF-SCOPE CANDIDATE I AGREE SHOULD STAY OUT. `R1A` checked and declined to raise that `Q-70` points explorers at `docs/plans/validation-constraints.explorations/` "per the Design explorations rule in `pack/AGENTS.md`", where that rule gives `docs/plans/<task>.explorations/` and the task is `agent-scaffold`. I agree it is out of scope, and I checked all four conditions of the precedent rather than accepting the call: (1) provenance predates the base commit, since 15 of 17 exploration directories already diverge from the rule; (2) no commit in `main..HEAD` modifies `pack/AGENTS.md:65`; (3) THE SUBJECT IS INDEPENDENT, which does the real work here, because `Q-70`'s pointer is not what falsified the rule, `Q-69` and thirteen earlier directories did that; (4) no shared fix with any in-scope finding, since every remedy above edits the `Q-70` `ask` and none touches the pack. It is a project-wide convention question owed its own item, not a `Q-70` defect, and it does not reset the streak.

MECHANICAL STATE OF THE ARTIFACT, checked independently: `render docs/plans/agent-scaffold.plan.toml --check` reports "up to date" at exit 0; `validate --source ... --workflow` reports 308 records valid, 95 steps and 70 questions valid, and "workflow invariants hold" at exit 0; all three changed files return 0 under `LC_ALL=C grep -cP '[^\t\x20-\x7e]'`.

---

## Commands that decided a verdict, with their output

Every command below was run against `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/tri-q70-r1` or a fixture copied from it. Fixture root abbreviated `<S>` for `<scratch>/tri-q70`.

R1A-1, the site count and its provenance:

```
$ grep -onE "\([0-9]+(, [0-9]+){1,6}\)" docs/plans/agent-scaffold.plan.toml
1331:(3, 4, 6)
1340:(9, 5, 6, 4)
1349:(11, 9, 6, 4, 5)
1358:(6, 4, 2, 0, 2)
$ git blame -L 1349,1349 --date=short HEAD -- docs/plans/agent-scaffold.plan.toml
c857fb8f (Test 2026-08-11 1349) note = "Five work-review rounds, 35 valid findings in scope (11, 9, 6, 4, 5) ...
$ git show -s --format='%h %ad' c857fb8f
c857fb8 Tue Aug 11 12:29:20 2026 +0100
$ git log --date=iso --format='%h %ad %s' main..HEAD
869943e 2026-08-11 17:53:23 +0100 docs: record the two-path waiver-ownership divergence as a Q-70 pass input
c58427f 2026-08-11 17:43:34 +0100 docs: register Q-70, the owed design pass on W5 waiver ownership
$ git blame -L 621,621 --date=short HEAD -- docs/plans/agent-scaffold.ledger.md
a6f42122 (Test 2026-08-08 621) FOUR FINDINGS ARE OUT OF SCOPE ... the convention already exists at three sites ...
```

R1A-2 and R1C-6, the population and the declared-increment fixture:

```
$ jq -r 'select(.type=="round") | [(.step // (.task|sub("-inc[a-zA-Z0-9]+$";""))), (.increment // .task)] | @tsv' docs/metrics/workflow.jsonl \
  | sort -u | awk -F'\t' '{step=$1; inc=$2; lead=inc; sub(/-inc[a-zA-Z0-9]+$/,"",lead); if (lead != step) print step"\t"inc}'
decision-folder-currency        decision-folder-currency-fold
workflow-driver                 workflow-driver-stage0a
workflow-driver                 workflow-driver-stage0b
workflow-driver                 workflow-driver-stage1
workflow-enforcement-tier       workflow-enforcement-tier-endproperty-fold
workflow-enforcement-tier       workflow-enforcement-tier-fold

$ # <S>/driver-waiver: fresh cp -r docs/, one [[step.waiver]] on the live workflow-driver step
$ agent-scaffold validate --source <S>/driver-waiver/docs/plans/agent-scaffold.plan.toml --workflow
TOML waiver `workflow-driver-w1`: increment waiver names step `workflow-driver` but increment `workflow-driver-stage0a` belongs to step `workflow-driver-stage0a`
EXIT=1
```

The item's own two fixtures, rebuilt from scratch (control, undeclared, declared):

```
$ agent-scaffold validate --source <S>/baseline/docs/plans/agent-scaffold.plan.toml --workflow
<LOG>: 308 records, valid
<PLAN>: 95 steps, 70 questions, valid
<PLAN> vs <LOG>: workflow invariants hold
EXIT=0

$ agent-scaffold validate --source <S>/undeclared/docs/plans/agent-scaffold.plan.toml --workflow
<PLAN>: waiver `workflow-enforcement-tier-w5` on step `workflow-enforcement-tier` names increment `workflow-enforcement-tier-fold`, which is not one of the step's increments
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`
EXIT=1

$ agent-scaffold validate --source <S>/declared/docs/plans/agent-scaffold.plan.toml --workflow
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`
EXIT=1
```

R1C-1, the coverage measurement and the undeclarable pair:

```
$ python3 (tomllib over the plan TOML, json over the round log)
total ids: 94 declared: 43 undeclared: 51
of undeclared: step slugs: 24 orphan_tasks: 14
neither: ['decision-folder-currency-fold', 'optional-modules-inc1', 'optional-modules-inc2a', 'optional-modules-inc2b', 'optional-modules-inc2ci', 'optional-modules-inc3', 'round-log-core-incA', 'round-log-core-incB', 'state-schema-inc1', 'state-schema-inc2', 'state-schema-inc3', 'workflow-enforcement-tier-endproperty-fold', 'workflow-enforcement-tier-fold']
optional-modules complete declared: ['optional-modules-inc2cii'] waivers: [('optional-modules-w1', 'optional-modules-inc2cii')]
round-log-core complete declared: [] waivers: []
logged ids for round-log-core: ['round-log-core-incA', 'round-log-core-incB']

$ # <S>/upper: fresh cp -r docs/, round-log-core's `increment = []` replaced by a real [[step.increment]]
$ agent-scaffold validate --source <S>/upper/docs/plans/agent-scaffold.plan.toml --workflow
<PLAN>: increment id `round-log-core-incA` is not a well-formed kebab-case id
EXIT=1
```

R1C-2, the Markdown substrate:

```
$ agent-scaffold validate --plan <S>/md/docs/plans/t.md --metrics <S>/md/docs/metrics/workflow.jsonl --workflow
<PLAN>: Roadmap step `alpha` has no matching `### `alpha`` Step Detail heading
<PLAN> vs <LOG>: round log line 2: increment waiver names step `alpha` but increment `alpha-stage0a` belongs to step `alpha-stage0a`
EXIT=1
$ grep -c increment src/plan.rs
0
```

R1C-4, the third waiver path:

```
$ agent-scaffold validate --metrics <S>/logonly/w.jsonl
w.jsonl:1: missing field `increment`
EXIT=1
$ agent-scaffold validate --metrics <S>/logonly/w2.jsonl     # step names no step, increment belongs elsewhere
1 records, valid
EXIT=0
$ grep -c '"type":"waiver"' docs/metrics/workflow.jsonl
0
```

R1C-5, the drift mutation, run in a scratch copy of the tracked tree with its own CARGO_TARGET_DIR:

```
$ cargo test                                    # baseline, <S>/mut, unmutated
test result: ok. 378 passed; 0 failed; ...      (plus every integration binary green)
$ # pack/instrument.md only: "(the increment's leading slug equals the step)"
$ #                       -> "(the increment is one of the step's declared increments)"
$ cargo test agents_md_drift
test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... FAILED
test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 373 filtered out
$ # reverted in the scratch copy; full suite back to 378 passed, 0 failed
$ LC_ALL=C grep -n "must own its .increment." AGENTS.md .agents/AGENTS.reference.md pack/instrument.md
AGENTS.md:147 / .agents/AGENTS.reference.md:147 / pack/instrument.md:11   (all three hit)
```

R1A-4 and R1B-1, the ledger and the receipt:

```
$ jq -c 'select(.type=="decision" and (.q_id|test("entryroute")))' docs/metrics/workflow.jsonl
{"type":"decision","task":"validation-constraints","q_id":"Q-55-entryroute","options":["Design pass, validator cluster only","Planner straight to the step","Design pass over all four bodies","Split out the W5 fix first"],"recommendation":"Design pass, validator cluster only","chosen":"Design pass, validator cluster only","ts":"2026-08-11"}
$ git blame -L 533,533 --date=short HEAD -- docs/plans/agent-scaffold.ledger.md   -> 903b70b8 2026-08-11 ("the three `next` defects")
$ git blame -L 557,557 --date=short HEAD -- docs/plans/agent-scaffold.ledger.md   -> 8fa56939 2026-08-11 ("the three `agent-scaffold next` defects")
$ git blame -L 967,967 --date=short HEAD -- docs/plans/agent-scaffold.ledger.md   -> a46cd97d 2026-08-01 ("A FOURTH ... with the other three")
$ grep -c "^blocked_by = \[\]" docs/plans/agent-scaffold.plan.toml   -> 95
$ grep -c "^\[\[step\]\]" docs/plans/agent-scaffold.plan.toml        -> 95
```

R1A-5, the block bounds:

```
$ awk 'NR>=785 && NR<=860' src/plan/source.rs   (with line numbers)
791  for step in &plan.steps {
794      for waiver in &step.waivers {
809          "waiver `{}` on step `{}` names increment `{}`, which is not one of the step's increments",
846      if waiver.reason.required_tier() != waiver.evidence_tier {
854      }
855    }
856  }
```

R1B-3, the dismissal:

```
$ grep -n "Design explorations" pack/AGENTS.md AGENTS.md
pack/AGENTS.md:65: ... AGENTS.md:65: ...            (both at 65, as the finding says)
$ grep -c "rather than by a line number" docs/plans/agent-scaffold.plan.toml   (Q-70's own stated convention, twice in this diff)
```

Cross-checks of the item's own claims, all first-hand:

```
$ jq -r 'select(.type=="decision") | .q_id' docs/metrics/workflow.jsonl | sort -u | wc -l   -> 62
$ (registered [[question]] ids) | wc -l                                                     -> 70
$ comm -23 (receipt ids) (registered ids) | wc -l                                           -> 40
$ comm -23 (receipt ids) (registered ids) | grep -v '^Q-55-'                                -> (empty)
$ grep -n "W6" docs/plans/agent-scaffold.plan.toml | grep -v Q-70                           -> 1774 only, which is Q-59's ask
$ jq -c 'select(.type=="round" and (.task|test("^workflow-enforcement-tier-(endproperty-)?fold$")))' docs/metrics/workflow.jsonl
  -> five records each; every one step="workflow-enforcement-tier", increment=null, phase="plan_review", risk_class="risky";
     peak consecutive_clean 1 for the plan fold, 0 for the endproperty fold
$ jq -c 'select(.type=="escalation" and (.task|test("fold")))' docs/metrics/workflow.jsonl
  -> both fold tokens, human_decision="decision", increment=null
$ grep -c orphan_tasks src/workflow.rs                                                      -> 0
$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check                          -> up to date, EXIT=0
```

WHAT I SETTLED BY RUNNING AND WHAT BY READING. Run: `R1A-1`, `R1A-2`, `R1A-4`, `R1A-5`, `R1B-1`, `R1B-3`, `R1C-1`, `R1C-4`, `R1C-5`, `R1C-6`, the item's own three fixtures, and the artifact's `render --check`, `validate --workflow` and ASCII checks. Run and read: `R1C-2`, whose type-level claims are citations I opened and whose substrate claim I reproduced as a fixture. Read: `R1B-2`, which is a claim about what the item's own text asks for and is settled by opening the eight cited lines; `R1C-3`, whose code claims I settled by reading `src/workflow.rs:206-221` and `:498-502`, and whose ledger evidence I found by grep; `R1A-3` and `R1C-7`, settled as duplicates by opening `src/plan.rs:55-60`. Nothing above is presented as measured that was not run.
