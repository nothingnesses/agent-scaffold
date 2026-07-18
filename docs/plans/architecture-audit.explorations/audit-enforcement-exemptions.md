# Audit: the enforcement model and its exemptions / special-cases (Q-44 phase 1)

Read-only audit. Scope: the `validate --workflow` (W3) enforcement machinery and every special-case, exemption, escape hatch, and hack in it. Evidence is cited as `file:line`. This maps and enumerates; it does not design the unified solution (that is the phase-2 design pass). All assessments are one auditor's read, to be pressure-tested in design.

## Orientation: what the enforcement model is

The workflow enforces its own rules by DETECTION, not prevention: `src/workflow.rs:1-6` states the binary reads two artifacts (the plan Roadmap and the JSONL round log) and reports violations into the `validate` problem list, with no runtime dependency during a workflow run. The scaffolded workflow still writes both artifacts by hand (an LLM orchestrator); the check is a CI / pre-commit / on-demand backstop.

Two checks live in `check_workflow` (`src/workflow.rs:77-86`):

1. W3, the key invariant: every Roadmap step marked `complete` must have round records that converge (`src/workflow.rs:150-212`).
2. The round-log internal-consistency check: within one increment, the logged `consecutive_clean` must match what the outcome sequence implies (`src/workflow.rs:96-133`).

W3 depends on two projections it does not own: `plan::parse_roadmap` (the status table, `src/plan.rs:235-257`) and `metrics::parse_rounds` (the round records, `src/metrics.rs:372-415`). It owns no parser of its own; there is no ledger parse (`src/workflow.rs:8-10`).

LOAD-BEARING ORIGIN (do not regress): W3 exists because of a real incident. The `pause.md` step was committed OUTSIDE the review loop and no check would have caught it (`docs/plans/agent-scaffold.md:107`, `Q-27`). W3's core guarantee is that a step marked `complete` with no supporting review evidence is flagged (`src/workflow.rs:161-167`, tested at `src/workflow.rs:294-304` and end-to-end at `461-487`). Every exemption below is a hole deliberately punched in that guarantee; the unification must keep the hole DECLARED and VISIBLE so an UNDECLARED complete-with-no-evidence step still fails.

## Inventory of enforcement special-cases, exemptions, and hacks

### E1. The `complete`-only guard (the exemption MECHANISM itself)

- Evidence: `src/workflow.rs:156-158` (`if step.status != "complete" { continue; }`); documented `src/workflow.rs:16-18,135-137`; pinned by test `src/workflow.rs:391-399` (an `in progress` step with rounds is NOT checked).
- Why it exists: W3 only asserts convergence for steps that CLAIM to be done. Every non-`complete` status (in-flight or terminal) is out of scope by construction, so the entire exemption surface is "pick a status other than `complete`."
- Assessment: honest necessity as a concept (you cannot demand convergence evidence for an unfinished step), but it is ALSO the root of the proliferation: because the only lever is the status string, every new kind of "done but exempt" must become a new terminal status parallel to `complete`. This is the structural reason grandfathered/trivial exist as statuses rather than as attributes.
- Unify note: separate the two questions the status string currently conflates, "is this step done?" and "why is its convergence evidence absent/short?", so exemption becomes an attribute of a done step rather than a substitute status for it.

### E2. `trivial` terminal status (exemption #1)

- Evidence: constant `src/plan.rs:59-69` (esp. `:67`); rationale `src/plan.rs:54-58`; W3 skips it via E1; tested `src/workflow.rs:276-280`, `src/plan.rs:523-540`. Design decision `docs/plans/agent-scaffold.md:648` (sub-decision A).
- What it lets through: a `complete`-like step with ZERO round records, declared as a deliberately review-skipped low-stakes completion. Chosen over logging a round that never happened because a status is "the hardest of the candidates to fake" and is drift-guard-covered (`docs/plans/agent-scaffold.md:648`).
- Why it exists: to turn a SILENT skip (the pause.md failure mode) into a VISIBLE declared one. It is the honest, on-the-record form of "we chose not to review this."
- Assessment: honest necessity in FUNCTION (a project genuinely needs to mark a change as not-worth-reviewing), but a REAL sharp edge in FORM: it is a second terminal status carrying an orchestrator self-declaration, distinct from `complete` only in what W3 demands. The evidence backing it is weak (the orchestrator asserts triviality; nothing independent vouches).
- Unify note: a reason-coded waiver `{reason: review-skipped}` attached to a `complete` step, rather than a parallel status.

### E3. `grandfathered` terminal status (exemption #2)

- Evidence: constant `src/plan.rs:59-69` (esp. `:68`); rationale `src/plan.rs:54-58`; W3 skips it via E1; tested `src/workflow.rs:282-286`, `src/plan.rs:523-540`. Design decision `docs/plans/agent-scaffold.md:649` (sub-decision B); data grounding `docs/plans/agent-scaffold.md:653`; 14 live rows carry it (`docs/plans/agent-scaffold.md:132-150`).
- What it lets through: two sub-populations (`docs/plans/agent-scaffold.md:653`): (b1) eleven steps `core-assets`..`init-vcs` with ZERO round records (predate logging entirely), and (b2) three steps `convergence-accounting`, `pack-rebuild-tracking`, `user-prompts-dir` that DID converge informally but whose backfilled rounds never reach the required streak (predate DISCIPLINED logging).
- Why it exists: to exempt legacy steps that predate the round-logging regime. Chosen over a positional cutoff (brittle to insertion) and a closed slug list (a drifting separate artifact); a status is insertion-safe and general (`docs/plans/agent-scaffold.md:649`).
- Assessment: honest necessity for the ONE-TIME migration (you cannot retroactively review history), but a REAL sharp edge as a PERMANENT status. It is a per-project historical boundary frozen into the shared status vocabulary that every scaffolded project inherits. The human named it explicitly as the example sharp edge (`docs/plans/agent-scaffold.md:124` item 1).
- Unify note: a reason-coded waiver `{reason: predates-logging}`, or better, a one-time migration boundary recorded once (e.g. a baseline marker) rather than a status stamped on 14 rows in perpetuity.

### E4. The proposed `escalation-exempt` branch (exemption #3, NOT yet built)

- Evidence: step detail `docs/plans/agent-scaffold.md:657-663`; `Q-40` `docs/plans/agent-scaffold.md:120`; live triggering data `docs/metrics/workflow.jsonl:78-83` (increment `optional-modules-inc2cii`: four `new_valid` rounds, then a `type:"escalation"` `human_decision:"decision"` record at `:82`, then one `clean` round at `consecutive_clean:1`; risky needs 2).
- What it WOULD let through: an increment whose peak `consecutive_clean` falls short of its `risk_class` requirement, when the increment's records are accompanied by a `type:"escalation"` `human_decision:"decision"` record matched by `task`. Precise semantics at `docs/plans/agent-scaffold.md:663`: fires ONLY on `decision` (never `resume`), scoped to the one matching increment, exempts ONLY the clean-streak shortfall (risk_class-consistency still applies), passes with an informational note.
- Why it exists: `optional-modules-inc2cii` was `risky` but the human accepted it at ONE clean round via the escalation contract at the 5-round cap (the write-escape class closed and independently confirmed). Neither `trivial` (review was NOT skipped) nor `grandfathered` (not pre-logging) fits (`docs/plans/agent-scaffold.md:659`). Without this, W3 will false-flag `optional-modules` as non-converged once it is marked `complete`.
- Assessment: the NEED is an honest necessity (a human-authorised below-bar convergence is a real, recurring state, and W3 must not false-flag it). But building it as a THIRD special-case is itself the sharp edge the human flagged (`docs/plans/agent-scaffold.md:659,124` item 2). It is currently HELD, folded into `Q-44`, precisely so it is not built as a one-off (`docs/plans/agent-scaffold.md:124`, `173` shows status `deferred`).
- Unify note: this is the strongest-evidence member of the exemption family (backed by an independent durable record, not a self-declared status), and should be the model the other two are pulled toward, not a third parallel branch.

### E5. `skipped` terminal status (a quieter exemption)

- Evidence: constant `src/plan.rs:59-69` (`:66`); W3 skips it via E1 and NAMES it as exempt (`src/workflow.rs:16-18,136-137`); tested `src/workflow.rs:288-292`; two live rows `include-all-visible`, `ledger-parse` (`docs/plans/agent-scaffold.md:139,171`).
- What it lets through: a step that was dropped / never done. Distinct from the three "converged below the bar" exemptions: `skipped` means "this step is NOT done and never will be," so exempting it is uncontroversial (there is nothing to have reviewed).
- Assessment: honest necessity; not part of the below-bar exemption family. Keep it distinct in any unification (it answers "is this done?" with "no, abandoned," not "yes, but exempt").

### E6. The `-inc<x>` leading-slug strip and per-increment grouping

- Evidence: `leading_slug` `src/workflow.rs:41-71`; per-increment grouping in W3 `src/workflow.rs:169-209`; in the consistency check `src/workflow.rs:96-108`. Tested `src/workflow.rs:258-273,307-322,342-358`. Build-time must `docs/plans/agent-scaffold.md:651`.
- Why it exists: a `task` is a step slug plus an optional `-inc<x>` suffix naming one increment (`round-log-core-incA` / `-incB`). W3 keys off the Roadmap step slug, so it must map every increment back onto its step; and convergence is judged PER INCREMENT because one step's increments can converge under different risk classes (`round-log-core`: `low_risk` at incA streak 1, `risky` at incB streak 2), which a per-step aggregate would false-flag as inconsistent risk_class (`src/workflow.rs:143-149,307-322`).
- Assessment: the per-increment grouping is a clean, well-justified abstraction (the increment is the real review-loop unit; the tests at `:342-358` show a single loop's streak legitimately spanning three artifacts). The STRIP is a lexical hack with a documented latent over-strip risk (T3, `src/workflow.rs:54-62`): a slug that itself ends `-inc<alnum>` (e.g. `foo-incidental`, or an `increment`/`increment-tracker` pair) would be mis-stripped and its rounds misrouted. No live slug hits it, and the alphanumeric run is genuinely needed (`-incA`/`-incB`), but the abstraction is "string convention parsed by `rfind`" rather than a modeled relation.
- Unify note: under a structured model the step<->increment relation would be an explicit field (increment records name their parent step id), removing the lexical strip and its T3 ambiguity entirely; the hardening already sketched at `src/workflow.rs:60-62` (gate the strip on a known-slug allowlist) is the interim, the structured relation is the endgame.

### E7. Peak-not-terminal streak (T9) and risk_class-consistency guard

- Evidence: peak computation `src/workflow.rs:196-198` with the T9 note `:192-197`; risk_class-consistency guard `src/workflow.rs:176-185`.
- Why they exist: `consecutive_clean` is one running per-loop counter across the artifacts an increment's rounds name, so W3 takes the PEAK over the increment (a converged loop stops at convergence, so peak equals terminal; peak lets trailing bookkeeping rounds not sink a converged increment). The risk_class-consistency guard exists because the required streak is undefined if one increment logs two different classes, so it reports and moves on rather than guessing.
- Assessment: both are honest, well-documented modeling choices, not sharp edges. Note the escalation exemption (E4) is scoped to spare ONLY the streak shortfall and explicitly keeps this risk_class-consistency check live (`docs/plans/agent-scaffold.md:663` (c)), which is the correct boundary and must be preserved in any unification.

### E8. The round-log internal-consistency check (and its latent T4 gap)

- Evidence: `round_log_consistency_problems` `src/workflow.rs:96-133`; latent limitation T4 `src/workflow.rs:112-117`.
- Why it exists: the logged `consecutive_clean` is fully determined by the outcome sequence (a `clean` adds one, a `new_valid` resets to zero), so a disagreement means a hand-authored log error. The implied streak is recomputed independently so one bad record yields exactly one problem, not a cascade (`src/workflow.rs:93-95`).
- What it MISSES (T4): there is no re-opened-loop boundary, so an increment that legitimately re-opens with a bare `clean` (rather than a `new_valid` reset) would keep climbing and be miscounted. Real re-opens start with `new_valid`, so current data never hits it (`src/workflow.rs:112-117`). It also shares E6's grouping-by-`task` assumption.
- Assessment: sound as far as it goes; the T4 gap is a genuine latent hole but not currently reachable. It is a consistency check on hand-authored data, honest necessity given the LLM-written log.
- Unify note: a structured writer (if the round log ever gains one) would make `consecutive_clean` derived rather than hand-authored, dissolving this check; until then it is the right backstop.

### E9. Drift guards pinning prose to code (two of them)

- Evidence: plan-status drift guard `src/plan.rs:740-784` (asserts every `ROADMAP_STATUSES` / `QUEUE_EXACT_STATUSES` / parametric-prefix value the validator accepts is documented verbatim in `pack/plan-template.md`); metrics-schema drift guard `src/metrics.rs:662-741` (asserts every record type, field name, and enum spelling the validator checks is documented in `pack/instrument.md`).
- Why they exist: the accepted vocabulary lives in two places (the validator, the source of truth, and the human-readable pack docs), so the guards fail if one side changes without the other (Principle 16, one source of truth). Enum spellings are iterated from the code's own `VARIANTS` / constants so a code-side rename auto-re-points the check (`src/metrics.rs:721-740`, `src/plan.rs:759-764`).
- What they cover well: value spellings. Anchoring is deliberate (trailing comma for statuses `src/plan.rs:759-764`, backticks for fields `src/metrics.rs:715-718`) to catch a deletion, not just a substring.
- What they MISS (the known gap): the metrics guard checks that field NAMES appear in the prose but NOT their required-vs-optional STATUS. This is the documented hole that let the increment-A `risk_class` required/optional mismatch slip to review (`docs/plans/agent-scaffold.md:655`). The guard would pass whether `risk_class` is documented as required or optional, or checked as required or optional; it only checks the name is present. The list of fields is also HAND-MIRRORED from `check_record` (`src/metrics.rs:672,690-711` "if a field is added ... update this list"), so a new field silently escapes the guard until someone updates the list, itself a drift surface.
- Assessment: a sound approach for value-set drift (the iterate-from-code design is genuinely good), but INCOMPLETE: it pins names and spellings, not semantics (required/optional, type, which record type owns a field), and the field list is a manual mirror rather than derived. Partial, not a real sharp edge in what it does, but a real gap in what it claims to guard.
- Unify note: if the schema were a single structured declaration that both the validator AND the docs projection read (the Principle 8 direction), the drift guard would become unnecessary (there would be one source, nothing to drift), which is the cleanest resolution; short of that, the guard should also assert required/optional and derive its field list.

### E10. Forward-looking exemptions (a recurring pattern, not code yet)

- Evidence: the proposed W4 decision-receipt check is specified as forward-looking, "historical Q-1..Q-41 exempt via the log's absence" (`docs/plans/agent-scaffold.md:122,123`). The same shape as `grandfathered` (E3): history predates the regime.
- Assessment: this is the SAME "predates the regime" concept as `grandfathered`, appearing a second time for a different check (W4 vs W3). If every new enforcement check re-invents its own historical-cutoff exemption, that is proliferation of a fourth kind. Flagged here because the unification should cover "history predates this rule" ONCE, not per-check.
- Unify note: a single regime-start boundary (per check, or global) that all invariants consult, rather than each check hand-rolling its historical exemption.

### E11. Minor enforcement special-cases in the validators (for completeness)

- Queue status: exact-match set vs parametric prefix. `QUEUE_EXACT_STATUSES` matches exactly so `openfoo` is rejected (`src/plan.rs:82,347-348,683-693`), while `blocked on <slug>` (`src/plan.rs:74,337-339`) and `decided -> folded into <slug>` (`src/plan.rs:87,347-348,407-423`) are parametric prefixes with cross-referenced trailing slugs. Honest schema, not a sharp edge, but note these parametric-status-with-embedded-slug forms are string conventions a structured model would make explicit fields.
- `reviewers` empty-array rejection: a present `reviewers` array must be non-empty (`src/metrics.rs:244-248,614-619`); a round with no attribution omits the field via the optional path. A deliberate illegal-state catch, honest.
- Best-effort-vs-report split: the parsers silently DROP malformed rows (`parse_roadmap` `src/plan.rs:235-257`, `parse_rounds` `src/metrics.rs:365-415`) while `validate_plan` / `validate_log` REPORT them (`src/plan.rs:159-192,201-229`, `src/metrics.rs:422-440`). This is a deliberate two-layer design (projection is best-effort, validation is strict), not a hack, but it means W3 reads a projection that has already dropped anything malformed, so a round record dropped by `parse_rounds` is invisible to W3 (it would be caught separately by `validate_log`). Worth preserving the invariant that both run.

## The exemption-unification question (grandfathered / trivial / escalation)

This is the core question the human posed (`docs/plans/agent-scaffold.md:124` item 2, `659`). Precise statement of what each represents:

| Exemption | Granularity | Evidence state | Why the evidence is absent/short | Where authority is recorded | Evidence strength |
| --- | --- | --- | --- | --- | --- |
| `trivial` (E2) | step | zero round records | review deliberately skipped (low stakes) | Roadmap status column (orchestrator-declared) | weak (self-declared) |
| `grandfathered` (E3) | step | zero records (b1) OR short streak (b2) | step predates the logging regime | Roadmap status column (orchestrator-declared) | weak (self-declared) |
| `escalation-exempt` (E4) | increment | records exist but peak streak short | human accepted below the bar at the round cap | `type:"escalation"` `human_decision:"decision"` JSONL record | STRONG (independent durable record) |

What they SHARE (the one concept): all three are ways the workflow says "this unit is DONE even though the normal convergence evidence is absent or short." In every case W3's default (`complete` demands a converged streak) would fire, and the exemption suppresses it. That common concept is: an AUTHORISED WAIVER of the convergence requirement for a specific unit.

Where they genuinely DIFFER (so they are not trivially identical):

1. Granularity: trivial/grandfathered exempt a whole STEP; escalation exempts one INCREMENT. A unified model must carry the unit (step or increment), which the current status-on-Roadmap-row form cannot express for an increment (there is no Roadmap row per increment). This is a concrete reason escalation could NOT have been a status and had to be a record.
2. Reason: "review skipped" vs "predates the regime" vs "human accepted at the cap." These are audit-distinct: a reader needs to know WHY the bar was waived, and the three carry materially different trust stories.
3. Evidence strength: this is the sharpest distinction. Escalation is backed by an INDEPENDENT durable record written at the moment of the human decision (`docs/metrics/workflow.jsonl:82`). Trivial and grandfathered are backed only by a status the orchestrator typed into the plan, no independent artifact. A naive unification that flattens all three into one status would DOWNGRADE escalation's evidence to the weaker self-declared tier, and a naive unification into one record would UPGRADE trivial/grandfathered to imply an independent authorisation they do not have.

My read: they ARE one concept wearing three hats at the level of "W3 must not flag a unit a human has vouched is done." One unified exemption concept can and should cover all three, and the current three-mechanism design (two Roadmap statuses plus a proposed third code branch) is a real sharp edge, exactly as the human suspected. The clean model is a single WAIVER / ATTESTATION notion carrying: (unit: step-or-increment id), (reason: predates-logging | review-skipped | accepted-at-escalation | ...), and (authority pointer: the escalation record, or the declarer). Under that model `grandfathered` and `trivial` stop being parallel statuses and become reason codes on a waiver against a `complete` step, and `escalation-exempt` is the SAME waiver DERIVED from an existing escalation record rather than a third branch. This also absorbs E10 (forward-looking historical exemptions) as a `predates-regime` waiver applied at a boundary.

The one thing unification must NOT erase: the evidence-strength tier. Escalation's independent record is genuinely stronger than a self-declared status; the unified concept should preserve a distinction between a human-authored-record-backed waiver and an orchestrator-declared one, so the cleanup does not launder a weak self-declaration into looking as trustworthy as an escalation record. So: one concept, three reason codes, but TWO evidence tiers.

## Load-bearing correctness any refactor must preserve

1. The pause.md catch (the reason W3 exists). A step marked `complete` (or any done-claiming status) with NO supporting evidence AND no declared exemption MUST still be flagged. `src/workflow.rs:161-167`, tested `:294-304,461-487`. Any unification that makes exemption easier or the default must keep the UNDECLARED case failing. This is the incident guarantee (`docs/plans/agent-scaffold.md:107`).
2. Exemptions stay DECLARED and VISIBLE. The whole point of `trivial` over a fake log record was that a silent skip becomes a visible declared one (`docs/plans/agent-scaffold.md:648`). A unified waiver must be an explicit on-the-record artifact, never an implicit default or a silent code path.
3. Escalation exemption scope (if/when built). It must fire ONLY on `human_decision:"decision"` (never `"resume"`), be scoped to the ONE matching increment, exempt ONLY the streak shortfall (risk_class-consistency and every other W3 check still apply), and not be widenable accidentally (`docs/plans/agent-scaffold.md:663` (a)-(d), and the reviewer risk note there). The unified model must not loosen this scoping.
4. Per-increment convergence semantics (E6/E7). The increment is the real review-loop unit; the peak-over-increment streak and per-increment risk_class-consistency are correct and let legitimately mixed-class steps pass (`round-log-core`). A refactor must keep judging convergence per increment, not per step slug.
5. Both checks run and the strict/best-effort split holds (E11). `validate_log` (strict, reports dropped rows) and W3 (reads the best-effort projection) are complementary; W3's silence on a malformed round depends on `validate_log` catching it elsewhere. Keep both wired into `validate`.

## One-line summary of the inventory

Eleven enforcement special-cases: E1 the `complete`-only guard (the mechanism that forces every exemption to be a status), E2 `trivial` + E3 `grandfathered` + E4 the proposed `escalation-exempt` (the three below-bar exemptions), E5 `skipped` (a distinct not-done exemption), E6 the `-inc<x>` strip + per-increment grouping, E7 peak-streak + risk_class-consistency, E8 the round-log consistency check (T4 gap), E9 the two prose-to-code drift guards (the required-vs-optional gap), E10 forward-looking historical exemptions, and E11 minor validator special-cases. The three below-bar exemptions are one concept (an authorised waiver of the convergence bar) with three reason codes and two evidence tiers, and their current three-mechanism spread is the real sharp edge; unify them, but preserve the pause.md catch, the declared-and-visible requirement, and escalation's stronger independent-record evidence.
