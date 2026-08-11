# Q-70 review, round 1, reviewer: PREMISE VERIFICATION

Lens: establish first-hand whether each factual claim in the `Q-70` entry is true. Every citation was opened, every count re-measured, both recorded scratch fixtures rebuilt from scratch, and every behavioural claim settled by running the tool rather than by reading the source.

Artifact: `git diff main..HEAD` on `review/q70-premises`, which adds `[[question]] Q-70`, the `q70-capture` orphan-task line, an empty `docs/plans/agent-scaffold.questions/Q-70.md`, and the regenerated `docs/plans/agent-scaffold.md`.

Binary used for every run: `target/debug/agent-scaffold` built from this worktree at HEAD. Fixture root: `<scratch>/q70-premises/`. No fixture was left at mode 000.

Five findings: three `medium`, two `low`. No `high` and no `critical`.

---

## R1A-1. "the convention already exists at three sites" is stale; there are four sites, and the fourth landed on the day this item was written

Severity: `medium`

Claim under test, from mechanism (1) of THE THREE DETECTION MECHANISMS: "A W6 join checking a waiver note's `<total> (<r1>, <r2>, ...)` per-round breakdown against the `valid_findings` of the round records the same command already reads; the convention already exists at three sites and only the enforcement is missing."

Re-measured. Every waiver note carrying the `<total> (<r1>, <r2>, ...)` breakdown:

```
$ grep -onE "\([0-9]+(, [0-9]+){1,6}\)" docs/plans/agent-scaffold.plan.toml
1331:(3, 4, 6)
1340:(9, 5, 6, 4)
1349:(11, 9, 6, 4, 5)
1358:(6, 4, 2, 0, 2)
```

Four sites, not three: waivers `workflow-enforcement-tier-w1` (:1331), `-w2` (:1340), `-w4` (:1349) and `-w3` (:1358). A broader search for the same pattern across the whole plan TOML returns the same four lines and nothing else, so this is the complete population, not a sample.

Why the count moved, and why the item should have caught it:

```
$ git blame -L 1349,1349 --date=short HEAD -- docs/plans/agent-scaffold.plan.toml
c857fb8f (Test 2026-08-11 1349) note = "Five work-review rounds, 35 valid findings in scope (11, 9, 6, 4, 5) ...
$ git blame -L 621,621 --date=short HEAD -- docs/plans/agent-scaffold.ledger.md
a6f42122 (Test 2026-08-08 621) FOUR FINDINGS ARE OUT OF SCOPE AND BECOME ONE BACKLOG STEP, THREE MECHANISMS ...
```

The source `Q-70` relays is the inc4 round 3 record at `docs/plans/agent-scaffold.ledger.md:621`, written 2026-08-08, which says "the convention already exists at three sites". It was correct then. The `-w4` waiver note, the fourth site, was written 2026-08-11, three days later, at inc4's close. `Q-70` was authored 2026-08-11 and restates the 2026-08-08 figure as present fact.

Impact. The same paragraph refuses to state the dangling-receipt count for exactly this reason, citing the project's own standing cure recorded against orchestrator defect (12) at `docs/plans/agent-scaffold.ledger.md:855` ("Prefer no count at all over a maintained one"), and then states a different moving count one sentence earlier. The pass sizing the W6 join reads a population one short of the real one.

---

## R1A-2. "Two loops hit this" under-counts the affected population by four, and three of the four are DECLARED increments in the live plan, so the item's "declared" case is a live condition rather than a fixture-only hypothetical

Severity: `medium`

Claim under test, from THE BLOCKER: "So a round record whose `task` ends in anything other than `-inc<alnum>` JOINS a step under W3 and CANNOT BE OWNED BY ANY WAIVER UNDER W5. Two loops hit this, `workflow-enforcement-tier-fold` and `workflow-enforcement-tier-endproperty-fold`."

Re-measured over the whole round log. For each round record I computed the pair W3 actually uses, its step (`round_step_slug`: structured `step` id, else `leading_slug(task)`) and its increment identity (`round_increment_id`: structured `increment` id, else `task` verbatim), then flagged every identity whose `leading_slug` is not the step, that is, every identity no `[[step.waiver]]` can name without W5 refusing it:

```
$ jq -r 'select(.type=="round") | [(.step // (.task|sub("-inc[a-zA-Z0-9]+$";""))), (.increment // .task)] | @tsv' docs/metrics/workflow.jsonl \
  | sort -u | awk -F'\t' '{step=$1; inc=$2; lead=inc; sub(/-inc[a-zA-Z0-9]+$/,"",lead); if (lead != step) print step"\t"inc}'
decision-folder-currency        decision-folder-currency-fold
workflow-driver                 workflow-driver-stage0a
workflow-driver                 workflow-driver-stage0b
workflow-driver                 workflow-driver-stage1
workflow-enforcement-tier       workflow-enforcement-tier-endproperty-fold
workflow-enforcement-tier       workflow-enforcement-tier-fold
```

Six identities across three steps, not two under one.

`decision-folder-currency-fold` is the exact shape the item describes, a third instance: `{"task":"decision-folder-currency-fold","step":"decision-folder-currency","increment":null, ...}`, five `plan_review` records, no structured increment id. It converged (peak `consecutive_clean` 1 against `low_risk`'s 1) so no waiver is owed and W3 passes today, but the shape is identical.

The three `workflow-driver-stage*` identities are the material half, because they are DECLARED. `docs/plans/agent-scaffold.plan.toml:688-698` declares `workflow-driver-stage0a`, `-stage0b` and `-stage1` as `[[step.increment]]` entries under step `workflow-driver`. Their round records carry `task: "workflow-driver"` plus the structured `increment` id, so W3 groups them by that id, and `leading_slug("workflow-driver-stage0a")` is the whole token, which is not `workflow-driver`.

Demonstrated, not inferred. Fixture `<scratch>/q70-premises/driver-waiver/`, a copy of `docs/` with the `workflow-driver` step's `waiver = []` line replaced by one `[[step.waiver]]` naming a live declared increment (`unit = "increment"`, `increment = "workflow-driver-stage0a"`, `reason = "review-skipped"`, `evidence_tier = "self-declared"`, chosen self-declared so the evidence join cannot confound the ownership rule):

```
$ agent-scaffold validate --workflow --source <scratch>/q70-premises/driver-waiver/docs/plans/agent-scaffold.plan.toml
<PLAN> vs <LOG>: TOML waiver `workflow-driver-w1`: increment waiver names step `workflow-driver` but increment `workflow-driver-stage0a` belongs to step `workflow-driver-stage0a`
EXIT=1
```

The `src/plan/source.rs` membership check is silent (the increment IS declared) and W5 alone refuses. That is the item's own "DECLARED" case, reproduced on the live plan rather than on an injected token.

Impact. The item is the pass's measured input on the defect's shape. As written it says the defect is two undeclared plan-fold loops in one step; measured, it is six identities in three steps, three of them declared increments whose ids the plan already commits to. That bears directly on the coupling hypothesis the item asks the pass to settle: the choice between "a narrow lookup of the waived increment against the step's declared `[[step.increment]]` set" and "a rework of how a waiver names its unit" is being made against a population the item states at one third of its real size, and the live plan already contains three declared increment ids whose naming diverges from W5's `-inc<alnum>` assumption.

---

## R1A-3. Escape route 4's "never reads `step.increments`" describes a field that does not exist on W5's input, which understates the fix's edit surface

Severity: `medium`

Claim under test, escape route (4): "W5's check is lexical on the token rather than a lookup against the step's declared increments: `w5_problems` derives only a slug set from the steps (`src/workflow.rs:549`) and never reads `step.increments`."

The citation resolves and the literal statement is true: `src/workflow.rs:549` is `let slugs: BTreeSet<&str> = steps.iter().map(|step| step.slug.as_str()).collect();` and no line in `w5_problems` reads any increment field. But "never reads `step.increments`" implies W5 is handed steps that carry increments and declines to look. It is not.

`w5_problems(waivers: &[Waiver], steps: &[Step], escalations: &[Escalation])` at `src/workflow.rs:544-548` takes `Step` from `crate::plan` (the import at `src/workflow.rs:49-55`). That type is:

```
src/plan.rs:55-60
pub struct Step {
	pub slug: String,
	pub status: String,
}
```

There is no `increments` field. `grep -n "increments" src/workflow.rs` returns six hits, all of them either prose or W3's own per-record `BTreeMap` at `:467-471`; nothing named `step.increments` exists anywhere in the file. Both substrates drop the data before W5 sees it: `PlanToml::step_views()` at `src/plan/source.rs:422-430` maps each TOML step to `super::Step { slug, status }` and discards `step.increments`, and the Markdown path's `plan::parse_roadmap` (`src/plan.rs:266-285`) builds the same two-field struct from a pipe table that never had increments.

Impact. The item requires each proposal to "state the edit surface its direction implies (naming which source files it touches)". As phrased, route 4 makes the narrow-lookup option look like a local edit inside `w5_problems`. It is not: it requires widening the shared `plan::Step` projection or the `w5_problems` signature, and that projection is also fed by the Markdown substrate, where declared increments do not exist at all, so the fix has to decide what W5 does on a substrate that cannot supply the lookup's input. That is a real constraint on the coupling hypothesis and it is currently invisible in the item.

---

## R1A-4. "THE DURABLE RECORD SAYS FOUR, NOT THREE" is not what the durable record says; it says both, and the "three" side includes the very decision this item cites as its own authority

Severity: `low`

Claim under test: "(b) The `agent-scaffold next` defects routed here by the human decision of 2026-07-30. THE DURABLE RECORD SAYS FOUR, NOT THREE, and the brief that commissioned this registration said three, so the count is corrected here rather than propagated".

The four-side is real and resolves exactly as quoted: `docs/plans/agent-scaffold.ledger.md:967` begins "A FOURTH `agent-scaffold next` DEFECT" and states it "belongs to the validation-constraints step with the other three", and the `blocked_by` re-measurement holds first-hand (`grep -c "^blocked_by = \[\]"` returns 95 against `grep -c "^\[\[step\]\]"` 95, and no `blocked_by` line is populated).

But the durable record also says three, in two passages dated 2026-08-11:

- `docs/plans/agent-scaffold.ledger.md:533`, the `Q-55-entryroute` decision record itself, the decision `Q-70`'s own opening cites as having decided the entry route: "the two inc3 defects plus the three `next` defects are already-diagnosed point defects with NO OPEN DESIGN SPACE, so they stay OUT of the pass".
- `docs/plans/agent-scaffold.ledger.md:557`, the ledger's current "THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP" paragraph, item (4): "the three `agent-scaffold next` defects routed here by an earlier human decision". `git blame` dates this line 2026-08-11.

So the sentence attributes the "three" solely to the brief when the ledger carries it in the paragraph a resuming agent reads first and in the human decision the item invokes. The count `Q-70` states (four) is the correct one, so nothing is lost from the plan; what is wrong is the claim about where the wrong count lives, and the consequence is that the ledger's own stale threes are not flagged as owed corrections.

---

## R1A-5. `src/plan/source.rs:791-843` under-cites the block it names, excluding the pairing rule the same paragraph attributes to it

Severity: `low`

Claim under test: "`src/plan/source.rs:791-843` validates waivers per step INDEPENDENTLY of `src/workflow.rs`".

The per-step waiver loop opens at `src/plan/source.rs:791` (`for step in &plan.steps {`) and closes at `:856`. The cited range stops at `:843`, the end of the `evidence` presence match. The `reason` to `evidence_tier` pairing check lives at `:844-854` and is outside the cited range.

That matters inside this same paragraph, which goes on to say the block's introducing comment "reaches the presence rules and the pairing, both of which genuinely hold, the pairing single-sourced through `WaiverReason::required_tier`". A reader who opens `:791-843` as instructed finds the presence rules and no pairing check. The two other citations in the paragraph are exact: the increment set is built at `:792-793` and the membership check is at `:807-811`, both confirmed.

---

# The item the writer raised against itself: NOT a defect, on the evidence

The planner reported that escape route 4 and the later divergence paragraph state route 4's evidence at two different scopes, and declined to choose between narrowing route 4 to `src/workflow.rs` explicitly and leaving it as written with the later paragraph as the labelled correction. Judged first-hand: there is no contradiction to close.

Route 4's subject is W5 throughout, and every clause names it: "W5's check is lexical", "`w5_problems` derives only a slug set", "`src/workflow.rs:549`". It never claims no structural lookup exists anywhere in the tool, only that W5 does not do one. The later paragraph is explicitly framed as "MEASURED AFTER ESCAPE ROUTE 4 WAS WRITTEN AND BEARING DIRECTLY ON IT", names the other path by file, and concludes "Escape route 4 is therefore CONFIRMED BY MEASUREMENT". Both statements are true and they are about different functions in different modules.

Route 4's operative conclusion, "declaring the fold tokens as `[[step.increment]]` entries does not help", is confirmed by my own from-scratch fixtures below: declaring the token removes the `src/plan/source.rs` problem and leaves W5 refusing at exit 1. Neither of the planner's two closures is required. (The separate defect I did find in route 4 is R1A-3, which is about the `step.increments` phrasing, not about the scope mismatch the planner raised.)

---

# The two recorded scratch fixtures, rebuilt from scratch and reproduced exactly

Neither fixture was reused; both were built from a fresh `cp -r docs/` of this worktree into `<scratch>/q70-premises/`, laid out as `<root>/docs/plans/` and `<root>/docs/metrics/` so the tool's project-root derivation resolves, then edited with the Edit tool.

Control, an unmodified copy:

```
$ agent-scaffold validate --workflow --source <scratch>/q70-premises/baseline/docs/plans/agent-scaffold.plan.toml
<LOG>: 308 records, valid
<PLAN>: 95 steps, 70 questions, valid
<PLAN> vs <LOG>: workflow invariants hold
EXIT=0
```

Fixture 1, UNDECLARED, the live shape. One `[[step.waiver]]` added to the `workflow-enforcement-tier` step, exactly the fields the item records (`id = "workflow-enforcement-tier-w5"`, `unit = "increment"`, `increment = "workflow-enforcement-tier-fold"`, `reason = "accepted-at-escalation"`, `evidence_tier = "record-backed"`, `evidence = "workflow-enforcement-tier-fold"`), no other change:

```
$ agent-scaffold validate --workflow --source <scratch>/q70-premises/undeclared/docs/plans/agent-scaffold.plan.toml
<PLAN>: waiver `workflow-enforcement-tier-w5` on step `workflow-enforcement-tier` names increment `workflow-enforcement-tier-fold`, which is not one of the step's increments
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`
EXIT=1
```

Both problem strings are byte-identical to the two the item quotes, both fire on one waiver, and the exit is 1. The double lock reproduces.

Fixture 2, DECLARED. The same fixture plus `[[step.increment]] id = "workflow-enforcement-tier-fold"`:

```
$ agent-scaffold validate --workflow --source <scratch>/q70-premises/declared/docs/plans/agent-scaffold.plan.toml
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`
EXIT=1
```

The `src/plan/source.rs` problem disappears, the W5 problem still fires, exit 1. "ONE REFUSAL PLUS ONE PASS" reproduces. No difference from the recorded outcome in any respect, including the exact problem strings.

Three further fixtures I built to test the blocker chain the item asserts but did not record a fixture for, each a fresh `cp -r docs/` with only the named edits:

```
$ # complete-nowaiver: step status flipped to `complete`, no waivers added
<PLAN> vs <LOG>: Roadmap step `workflow-enforcement-tier` increment `workflow-enforcement-tier-endproperty-fold` reached a consecutive-clean streak of 0 but its `risky` risk class needs 2
<PLAN> vs <LOG>: Roadmap step `workflow-enforcement-tier` increment `workflow-enforcement-tier-fold` reached a consecutive-clean streak of 1 but its `risky` risk class needs 2
EXIT=1

$ # complete-waivers-undeclared: `complete` plus both owed waivers, tokens not declared
<PLAN>: waiver `workflow-enforcement-tier-w5` ... which is not one of the step's increments
<PLAN>: waiver `workflow-enforcement-tier-w6` ... which is not one of the step's increments
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step ... belongs to step `workflow-enforcement-tier-fold`
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w6`: increment waiver names step ... belongs to step `workflow-enforcement-tier-endproperty-fold`
EXIT=1

$ # complete-waivers-declared: `complete`, both waivers, both tokens declared as increments
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step ... belongs to step `workflow-enforcement-tier-fold`
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w6`: increment waiver names step ... belongs to step `workflow-enforcement-tier-endproperty-fold`
EXIT=1
```

These confirm three of the item's claims by measurement rather than by reading: the two shortfalls are exactly 1 of 2 and 0 of 2 as stated; W3's exemption DOES accept both waivers once they exist (the W3 shortfall problems vanish), so "ONLY THE OWNERSHIP CHECK BLOCKS THEM" holds; and W5's record-backed evidence join passes on both (no evidence problem is reported in any run), which is the claim about the `escalation_increment_id` `task` fallback at `src/workflow.rs:141`.

I also reproduced the first of the two deferred inc3 defects, since it is a behavioural claim inside the diff:

```
$ chmod 000 <scratch>/q70-premises/mode000/a/workflow.jsonl
$ agent-scaffold validate --metrics <scratch>/q70-premises/mode000/a/workflow.jsonl
Error: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
EXIT=1
$ chmod 000 <scratch>/q70-premises/mode000/b          # unsearchable DIRECTORY, log inside it
$ agent-scaffold validate --metrics <scratch>/q70-premises/mode000/b/workflow.jsonl
no metrics log at .../mode000/b/workflow.jsonl; nothing to validate
EXIT=0
```

The mode-000 file exits 1 and the unsearchable directory exits 0, exactly as the item states. Both fixtures were chmodded back to 644 / 755 afterwards.

---

# Claims verified, and claims I could not verify

Verified first-hand, by opening the cited line or by running a command:

- `round_step_slug` at `src/workflow.rs:119` and its preference for the structured `step` id. Confirmed.
- `leading_slug` at `src/workflow.rs:88`, stripping only `-inc` followed by a non-empty run of ASCII alphanumerics (`INCREMENT_MARKER` at `:69`). Confirmed.
- W5's lexical ownership test `leading_slug(increment) != waiver.step` at `src/workflow.rs:564`. Confirmed, exact line.
- W5's real-Roadmap-step rule at `src/workflow.rs:553`. Confirmed, exact line.
- `round_increment_id` at `src/workflow.rs:127`, `escalation_increment_id` at `src/workflow.rs:141`. Confirmed, exact lines.
- W3 skipping every non-`complete` step at `src/workflow.rs:445`. Confirmed, exact line.
- Escape route 1: the step-unit waiver is consulted only inside the `matching.is_empty()` branch at `src/workflow.rs:450`. Confirmed.
- Escape route 2: `step: step.slug.clone()` at `src/workflow.rs:258`, and the TOML `Waiver` struct at `src/plan/source.rs:279-300` carries no `step` field under `#[serde(deny_unknown_fields)]` (`:278`), so the field is genuinely not authorable in the TOML flow. Confirmed, and the item's "one level stronger than the durable record previously stated" holds.
- Escape route 3: `orphan_tasks` appears nowhere in `src/workflow.rs` and only in `src/plan/source.rs` (`:116`, `:770-782`) across all of `src/`, `tests/`, `pack/` and `build.rs`. Confirmed. The item says "duplicate and slug-collision validation"; the block also does a well-formed-token check at `:772-774`, which is an omission rather than an error, and is not raised.
- The two fold tasks each carry exactly five `type:"round"` records, all `phase: "plan_review"`, all with `step: "workflow-enforcement-tier"` and no structured `increment` id, `risk_class: "risky"`. Confirmed by `jq`.
- Peak `consecutive_clean` 1 for the plan fold and 0 for the endproperty fold, against `required_streak(Risky) == 2` (`src/workflow_spec.rs:52-53`, `:195-196`). Confirmed twice, by `jq` and by the `complete-nowaiver` fixture's own message text.
- Both fold tokens have a `type:"escalation"` record whose `task` equals the token, `human_decision: "decision"`, no structured `increment` id. Confirmed by `jq` on the raw records.
- `src/plan/source.rs:792-793` builds the step's declared increment set, `:807-811` is the membership check. Confirmed, exact lines.
- The comment at `src/plan/source.rs:785-790` says what the item says it says, exactly that range. Confirmed. `check_record` is `fn check_record(value: &Value)` at `src/metrics.rs:435`, taking one JSON record and no step data, so it genuinely could not perform the membership check; its waiver arm at `:539-600` enforces exactly the two presence rules the source-side block restates, and explicitly leaves the pairing to W5 at `:546-548`. Confirmed.
- W4 skipping every item whose status is not a decided fold at `src/workflow.rs:321`, and the cutoff comparison at `:332-336`; `[meta].w4_baseline = "Q-44"` is declared at `docs/plans/agent-scaffold.plan.toml:3`. Confirmed.
- The ledger passage "TWO WAIVERS ARE OWED AND CANNOT YET BE WRITTEN" resolves at `docs/plans/agent-scaffold.ledger.md:561` and carries the drafted notes for both `-w5` and `-w6`, as claimed. Confirmed.
- "THREE DEFECTS IN `agent-scaffold next`" resolves at `:1333` and records the three defects the item summarises, in the item's order. "A FOURTH `agent-scaffold next` DEFECT" resolves at `:967` and contains the quoted clause verbatim. Confirmed.
- The human decision routing the `next` defects, at `:1323`, is dated 2026-07-30. Confirmed.
- `blocked_by = []` on all 95 steps and zero populated. Confirmed independently.
- The `W6` token occurs on exactly one line outside this item (`docs/plans/agent-scaffold.plan.toml:1774`) and that line is `Q-59`'s `ask`. Confirmed. `Q-59` records the fuller option verbatim as "session_state enum + a W6 transition-legality check, no commands" and the 2026-07-23 human decision defers "the richer lifecycle state (a session_state enum, a transition-legality check, refuse-while-checkpointed) behind an EVIDENCE GATE (a first recorded transition failure)". Both quoted accurately.
- The `-w1` to `-w4` waiver-id sequence already exists on this step (`docs/plans/agent-scaffold.plan.toml:1324`, `:1333`, `:1342`, `:1351`). Confirmed.
- Orchestrator defect (12)'s standing cure at `docs/plans/agent-scaffold.ledger.md:855`: "an artifact under review MUST NOT assert a count of anything the orchestrator appends to during the loop ... Prefer no count at all over a maintained one". The item's paraphrase is accurate.
- The dangling-receipt re-measurement. The item's own command, `jq -r 'select(.type=="decision") | .q_id' docs/metrics/workflow.jsonl | sort -u`, returns 62 distinct ids against 70 registered `[[question]]` ids, giving 40 dangling, versus the recorded "29 of 51". Materially different, as claimed. All 40 are `Q-55-<suffix>` ids, so "dominated by" understates it: the population is entirely `Q-55` sub-decisions.
- Acceptance check 21 exists and instructs exactly what the item says a quotation resolver would automate (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:345`). `Q-55-check21b`'s ledger record at `:599` records "about eleven `src/checks.rs` citations there are stale", matching the item's "roughly eleven ... deliberately left stale".
- `Q-55-entryroute`: the receipt exists with `ts: "2026-08-11"` and `chosen: "Design pass, validator cluster only"`, and the ledger record at `:533` states the coupling ground and the "no open design space" scoping the item relays, close to verbatim. Confirmed.
- No `validation-constraints` step exists in the plan TOML. Confirmed.
- `Q-68` and `Q-69` are both `status = "exploring"`, so "the same shape `Q-68` and `Q-69` use" holds. `Q-69`'s premise-defect discipline is recorded in its own `ask`, as the item says.
- Mechanical currency of the change: `render --check` reports "up to date", `validate --workflow` on an unmodified copy of the branch's `docs/` is clean at exit 0, and all three changed files are ASCII-clean under `LC_ALL=C grep -cP '[^\t\x20-\x7e]'` (0 each).

Not verified first-hand, and not presented as verified:

- The containment TOCTOU claim (a FIFO-widened mid-run symlink swap yielding "workflow invariants hold" at exit 0 over a log outside the project root). I did not build the race. Its wording matches the ledger source at `:557` exactly, so it is a faithful relay of a claim I did not re-measure.
- "the roughly eleven `src/checks.rs` citations". I confirmed the ledger says "about eleven"; I did not count the citations in `checks-runner-worktree-name-collision.md` myself.
- "the brief that commissioned this registration said three". I have no access to the brief.
- The individual round-count breakdowns inside the drafted `-w5` and `-w6` waiver notes in the ledger. Not claimed by `Q-70`, so not measured.

Checked and deliberately not raised:

- The item points explorers at `docs/plans/validation-constraints.explorations/` "per the Design explorations rule in `pack/AGENTS.md`", and that rule (`pack/AGENTS.md:65`) says `docs/plans/<task>.explorations/`, where the task here is `agent-scaffold`. But 15 of the 17 exploration directories in `docs/plans/` are named for a step slug rather than the task, and `Q-69` uses `docs/plans/exploring-item-actor-boundary.explorations/` for a step that likewise does not exist. The divergence between the rule and the practice predates this change, so it is out of scope.
- "a round record whose `task` ends in anything other than `-inc<alnum>` ... CANNOT BE OWNED BY ANY WAIVER UNDER W5" is strictly true only for a record with no structured `increment` id, since W3's identity is that id when present. The item supplies the measured qualification (both fold tokens carry none) in the next sentence, so the generalisation is not raised on its own; the population consequence is raised separately as R1A-2.
- Escape route 1's evidence, "these two have five each", is about the two fold tasks, whereas W3's step-unit waiver branch keys on the STEP having no round records at all. The conclusion is unaffected (the step has 27), so this is not raised.
- Line length and prose wrapping, per the project rule.
