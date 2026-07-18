# Q-44 phase 2, target architecture A: the INCREMENTAL / minimal-migration lens

Explorer lens: INCREMENTAL / MINIMAL-MIGRATION. My job is to make the strongest version of the LEAST radical change that genuinely resolves the single-source-of-truth smells and unifies the three exemptions, so the human can compare it against a clean-slate design. I favour EVOLVING the current Markdown-with-parsed-regions toward slightly more structured regions plus a read-only projection, keeping human-editable Markdown as the primary surface where it earns its place, and moving the MACHINE-authored records into the JSONL event log that already exists and is already validated. Principle 8 demotes blast-radius and backwards-compat, so I do not lean on those as arguments; I lean on the fact that the structured skeleton the pivot points at already exists in fragments (audit-data-model summary), and the minimal move is to finish structuring it rather than replace it. Where incrementalism leaves a smell unresolved I say so explicitly, because that honesty is the whole point of proposing this alongside a clean-slate option.

## The design question restated

Design the TARGET architecture in which the workflow's DATA lives in structured, machine-readable form as the single source, with `agent-scaffold` PROJECTING it into human-readable views (Project Principle 8), AND a UNIFIED exemption/waiver model that absorbs the three current below-bar exemptions (`grandfathered`, `trivial`, the proposed `escalation-exempt`). Cover, as a coherent whole: (1) the structured source data model and what stays prose; (2) the file format(s) and the human editing story; (3) the projection/render and CLI surface; (4) the unified waiver model with two evidence tiers; (5) the migration path including this repo's own dogfooded plan/ledger; (6) a staged, reviewable roadmap with the decision receipt and the waiver model as the early pilots.

## The governing insight this lens is built on

The audit found only four things are machine-parsed today (audit-data-model summary): the plan Roadmap table (D2), the Open-Questions id/status/ask (D3), the Step-Detail heading slugs (D4), and the JSONL records (D10-D12). Everything else in the plan and the ENTIRE ledger is prose no code reads. The incremental observation is that the workflow's data splits cleanly into two populations that want DIFFERENT homes, and the current design's smells all come from mixing them:

- Population A, the HUMAN-AUTHORED DECLARATIVE SKELETON: the Roadmap rows (slug + status + order), the Open-Questions items (id + status + ask + the fold pointer), and the Step-Detail slugs. A human edits these directly, by hand, as the act of running the workflow. They already parse as Markdown regions and they benefit from staying in a human-editable, self-contained, tool-free document (the value `state-schema` deliberately preserved, `docs/plans/agent-scaffold.md:423`).
- Population B, the MACHINE-AUTHORED APPEND-ONLY EVENT RECORDS: the round records, and (to be added) the decision receipts and the waivers. These are written once per event by the orchestrator LLM, never re-edited, and are consumed by checks and projections. They already have a structured home (the JSONL log) with a validator (`src/metrics.rs`) and a projection pattern (`parse_rounds`).

Every headline smell is a case of population-B data being ALSO hand-authored as population-A-style prose, or population-A data being restated in a second prose home:

- B1 (the headline double-write): the ledger round NARRATIVE (population B written as prose) duplicates the JSONL round RECORD (population B, structured). `pack/LEDGER.template.md:9` admits the orchestrator writes both.
- B4: the decision option-labels+choice (population B) trapped only in the queue-item PROSE (population A home), with no structured record.
- B2/A1/A2: the plan Status line and the ledger RESUME STATE (derived views of population A) hand-restating the Roadmap.
- B3/D6: the status vocabulary (owned by code) hand-copied into the live plan prose, now stale.
- B6/D14: the round<->step link (a population-B relationship) carried only as a lexical `-inc<x>` string convention instead of a structured field.

The minimal resolution, then, is NOT to restructure the plan document. It is to (i) finish moving population B into the JSONL event log as the single structured source and DERIVE its human views by projection, (ii) give population A the small amount of extra structure that stops the two option-labels/waiver leaks (the queue field split and the waiver record), and (iii) keep the prose payloads exactly where they are, linked to the skeleton by slug. This is the least radical change that closes B1, B2, B3, B4, and B6 and unifies the exemptions, because it reuses two formats that already exist (Markdown regions and validated JSONL) and one projection command that already exists (`status`).

## 1. The structured source data model

### The skeleton (what is structured) and its single homes

I resolve each duplicated datum by naming ONE authoritative home (the one that already parses) and deriving the rest. No datum gets a new authoritative store it did not already have.

- STEP STATUS + ORDER. Home: the plan Roadmap pipe-table (D2), unchanged, still human-edited Markdown. It already is the closest thing to a machine source of truth (audit A3). Everything else that states status (the Status line D1, the RESUME-STATE pointer half D8) DERIVES from it by projection; those hand-authored copies are retired (see area 3). This closes B2/A1/A2.
- OPEN-QUESTIONS ITEMS. Home: the queue list in the plan (D3), human-edited Markdown, but with a small grammar refinement (area 4 below and SE-11): the `status` and the `folded_into <slug>` pointer become two conceptual fields rather than one `decided -> folded into <slug>` string, and the item stops carrying the decision's option-labels in prose (they move to the receipt). The id/status/ask stay exactly as parsed today.
- DECISIONS / RECEIPTS. Home: a `type:"decision"` JSONL record, the already-CONVERGED substrate (receipt-encoding-{A,B}, and `docs/plans/agent-scaffold.md:123`): `q_id`, `options`, `recommendation`, `chosen` (validated as a member of `options`), optional `ts`, plus the common `task`/`ts`. A `metrics::parse_decisions` projection and a W4 check that every `QUEUE_FOLD_PREFIX` (`decided`) queue item has a matching `q_id` receipt (forward-looking, historical Q-1..Q-41 exempt via the log's absence). This closes B4. I place this in my model unchanged; I do not re-litigate it.
- WAIVERS / EXEMPTIONS. Home: a NEW `type:"waiver"` JSONL record (area 4). One record type absorbs `grandfathered` + `trivial` + `escalation-exempt`. This is population B (a machine-authored declaration of an exemption event), so it belongs in the event log next to the receipt, not on a Roadmap row.
- ROUND LOG. Home: the JSONL `round` record (D10), unchanged as the structured source. The ledger round NARRATIVE stops carrying the structured facts (outcome, streak, risk_class, severities, findings counts); it keeps only genuinely non-structured commentary (see prose payload below). This closes B1: the round facts are single-sourced in the log; the narrative becomes commentary + a pointer.
- CROSS-REFERENCES. The slug is the join key across all three artifacts: Roadmap slug <-> Step-Detail slug (already checked bidirectionally, `validate_plan`, plan.rs:384-399, the model of a checked duplication, B7) <-> JSONL record's step. The round<->step link stops being the lexical `-inc<x>` strip and becomes an EXPLICIT structured field on the record (area 4 and increment 4): each round/decision/waiver record names its `step` (the Roadmap slug) and, where it applies, its `increment` id, so the relationship is a schema join, not a `rfind` on a string (closes B6/D14/E6/SE-10). The strip stays only as a legacy fallback for pre-migration records.
- STATUS VOCABULARY. Home: the code constants (`ROADMAP_STATUSES`, `QUEUE_EXACT_STATUSES`, `src/plan.rs:59-87`), already the source. The live plan's Documentation Protocol prose stops hand-listing them (it is the stale third copy, B3/D6); instead the tool can project the accepted set on demand (area 3), and the pack template stays drift-guarded as today. This closes B3.

### What STAYS prose payload, and how it links to the skeleton

The prose payloads are the parts that genuinely hold reasoning, not machine state, and forcing them into fields fights the format (the caution the orchestrator recorded at `docs/plans/agent-scaffold.md:124` and both receipt explorers seconded). They stay Markdown, linked to the skeleton by the keys above:

- STEP-DETAIL REASONING BODIES (D4, ~80% of the plan). Stay prose under a `### `<slug>`` heading; the heading slug is the structured link to the Roadmap row. Unchanged.
- MOTIVATIONS, the Project Principles narrative, Repository-Layout index (D5, D7). Stay prose. (The principle NUMBERING/id-namespace cleanup, SE-1/SE-17, is an independent naming problem, not part of this data pivot; it goes to the parallel cleanup backlog.)
- PER-ROUND COMMENTARY (the residue of D9 after B1 is closed). The ledger round section keeps the prose that is NOT in the round record: why an artifact was classified as it was, what a debate turned on, a reviewer's judgement call, the narrative a re-spawned orchestrator reads. It references the structured round record by `task`/`increment`. It no longer restates outcome/streak/risk_class, which now live only in the log.
- RESUME-STATE TRANSIENT NARRATIVE (D8). The pointer half is derived (area 3); the genuinely transient, non-plan-derivable narrative (a running debate, a locked in-round context) stays prose in the ledger.

The linking rule is uniform: prose is keyed to the skeleton by slug (Step Details), by `q_id` (queue prose points at the receipt), or by `task`/`increment` (round commentary points at the round record). The skeleton never lives inside the prose; the prose always points at the skeleton.

## 2. File format(s) and the human editing story

Incremental means REUSE the two formats already in play; introduce no third artifact class.

- The PLAN Markdown (`docs/plans/<task>.md`) stays the human-edited source for population A (the skeleton) plus all the prose payloads. The human edits it directly in Markdown, exactly as today. This is where the incremental lens deliberately declines to go structured: the plan is a self-contained, tool-free, harness-portable resume document, and that value (Principle 20-equivalent, and the explicit `state-schema` decision at `:423`) is real. The only format changes are the two small region refinements (the queue field split in area 4; the retirement of the hand-authored Status line into a projected view in area 3).
- The JSONL EVENT LOG (`docs/metrics/workflow.jsonl`) becomes THE structured source for ALL population-B records: `round` (exists), `escalation`/`dismissal_recheck`/`intake` (exist), plus the new `decision` and `waiver` records. I would reframe its documented purpose from "metrics logging" to "the workflow event log" (a one-line `pack/instrument.md` framing change), because it is no longer only calibration data; it now carries the decision receipts and waivers that W4/W5 enforce. The format is unchanged: one JSON object per line, hand-written by the orchestrator LLM, validated by `agent-scaffold validate` (detection, not prevention, preserving the harness-agnostic runtime, `Q-24`/`Q-6`).
- The LEDGER Markdown (`docs/plans/<task>.ledger.md`) stays a per-task, committed-then-deleted prose narrative. After B1 is closed it carries commentary + the RESUME-STATE transient narrative only; its structured round facts move fully to the log.

Editing story, concretely: the human still opens the plan Markdown and edits the Roadmap table and the queue lines by hand (the skeleton is theirs to author). The orchestrator LLM appends event records (rounds, decisions, waivers) to the JSONL as the loop runs, exactly as it already appends round records, and writes prose commentary to the ledger. Nobody hand-authors a datum in two homes: status is authored once in the Roadmap, round facts once in the log, option-labels+choice once in the receipt, exemptions once in a waiver.

Why one plan file, not several, and not a single monolithic structured file: splitting the skeleton into a separate machine data file was already weighed and declined in `state-schema` (`:423`) because it would either duplicate the facts (drift) or move the Roadmap/queue out of the human-readable plan. The clean-slate option can revisit that; the incremental option honours it. Two source homes (Markdown skeleton + JSONL records) is the minimum, and it maps exactly onto the two data populations.

## 3. The projection / render and the CLI surface

Extend the EXISTING projection machinery (`run_status`, the `Projection` struct, `src/main.rs:404-431`), which already reads the two structured sources and derives a `--json` and a human-readable view without writing any file. This is the prior art both receipt explorers and `state-schema`/`state-queries` point at; nothing new in kind is required.

- Add a `DecisionProjection` slot (from `parse_decisions`) and a `WaiverProjection` slot (from a new `parse_waivers`) to `Projection`, shown in `status --json` and the human summary. ~30-35 lines each, reusing `run_status`.
- Derive the STATUS-LINE SUMMARY (a per-status count of Roadmap rows plus the active step and the open-question ids) from the Roadmap + queue, and the RESUME-STATE POINTER half from the same, feeding the already-planned `state-queries` `status --resume` slice (`Q-28`, `:665-667`). The transient counters (round number, streak) are derived from the round log; only the genuinely transient narrative stays hand-authored in the ledger.
- Project the accepted STATUS VOCABULARY from the code constants on demand (a small addition to `validate`/`status`, or simply drop the stale live-plan enumerations and rely on the pack-template drift guard). Either way the live plan stops being a third hand-copied home.

CLI surface (extends the existing `status`/`run_status` and `validate`, no new top-level verbs beyond what `state-queries` already plans):

- `status` (compact default) and `status --json` gain the decision and waiver slots.
- `status --resume` (planned in `state-queries`) echoes the derived pointer plus the ledger's verbatim transient narrative.
- `validate --workflow` gains W4 (every `decided` queue item has a receipt) and W5 (every waiver is well-formed and its evidence tier is honest; area 4). W3 is modified to consult waivers.
- The strict-validate vs best-effort-projection split is PRESERVED exactly (E11, load-bearing): `parse_decisions`/`parse_waivers` are best-effort projections that silently drop malformed lines (like `parse_rounds`), while `validate_log` strictly reports them; W3/W4/W5 read the best-effort projection and depend on `validate_log` catching malformed lines elsewhere. Both stay wired into `validate`.

### Round-tripping: the human edits the SOURCE, never the view (and the honest smell)

The projection is READ-ONLY and regenerable. `agent-scaffold` never writes back into the plan Markdown. This is a deliberate incremental boundary: a generative plan-writer that renders derived content into the Markdown is the deferred `workflow-viz` territory (`:431`), owes its own design pass, and I do NOT pull it forward. So the round-trip is one-directional: human edits Markdown source + the orchestrator appends JSONL records; the tool projects a view; the human reads the view.

This forces an honest choice for the derived views (Status line, RESUME-STATE pointer) that a clean-slate writer would not face. Two minimal options, and I recommend the first:

- (Recommended) DELETE the hand-authored Status line and RESUME-STATE pointer from the source and replace them with a one-line "run `agent-scaffold status` / `status --resume` for current state" pointer. The status view becomes authoritative; the plan Markdown is no longer self-contained for a status SUMMARY (you run the tool), but it is still self-contained for the Roadmap TABLE, which is the actual source. This removes the drift by removing the second home.
- (Fallback) KEEP a hand-authored Status line but add a `validate` CHECK that compares it against the projection and flags drift (resolving SE-13 by detection). This keeps the self-contained summary at the cost of a hand-authored copy that a check, not derivation, keeps honest.

The residual smell is unavoidable under incrementalism and I state it plainly: without a plan-writer, a human-readable status SUMMARY inside the document is either removed (option 1, lose self-containment) or hand-authored-and-checked (option 2, a copy kept honest by a guard rather than truly derived). Only a clean-slate writer that renders the summary into the document achieves both self-containment AND true derivation.

## 4. The unified waiver / exemption model

One concept absorbing `grandfathered` + `trivial` + `escalation-exempt`: an AUTHORISED WAIVER of the convergence requirement for a specific unit, encoded as a single `type:"waiver"` JSONL record (population B, event log). This retires the `trivial` and `grandfathered` Roadmap STATUSES entirely (they become waivers on a `complete` step) and never builds `escalation-exempt` as a third code branch (it becomes a waiver reason). This is the same architectural move as the decision receipt (a `type:` record in the event log, validated, projected), which is exactly why the two are the paired pilots.

### The record

`type:"waiver"` fields:

- `unit`: enum `step` | `increment`. This is the granularity axis the enforcement audit named as the concrete reason escalation could not have been a status: a status lives on a Roadmap row and there is no row per increment (audit E, difference 1). A record carries the unit explicitly, so trivial/grandfathered (step-level) and escalation (increment-level) unify.
- `step`: the Roadmap slug the waiver covers (the explicit structured link, also serving increment 4's kill of the lexical strip).
- `increment`: optional, present when `unit:increment`, the increment id (for example `optional-modules-inc2cii`).
- `reason`: enum `predates-logging` | `review-skipped` | `accepted-at-escalation`. These are the three audit-distinct reasons (audit E, difference 2): a reader needs to know WHY the bar was waived. The enum is extensible, which also absorbs E10 (a per-check historical boundary is another `predates-logging`-style reason, declared once rather than re-invented per check).
- `evidence`: enum `self-declared` | `independent-record`. The TWO tiers (load-bearing). `predates-logging` and `review-skipped` are `self-declared` (the orchestrator asserts them; no independent artifact vouches). `accepted-at-escalation` MUST be `independent-record` and MUST reference a real backing record.
- `authority`: optional pointer to the backing record for the `independent-record` tier (the `type:"escalation"` `human_decision:"decision"` record, matched by `task`/`increment`).
- common `task`/`ts`.

### The two evidence tiers, and not laundering the weak into the strong

The sharpest audit finding is that escalation's independent durable record is genuinely stronger than a self-declared status, and a naive unification must not flatten it (audit E, difference 3). The model keeps DECLARATION separate from EVIDENCE: the waiver record is always the DECLARATION (uniform, one concept, declared-and-visible); the evidence tier records whether that declaration STANDS ALONE (`self-declared`) or is BACKED BY A SEPARATE EVENT RECORD (`independent-record`). W5 enforces the honesty: a waiver claiming `independent-record` for which no matching `type:"escalation"` `human_decision:"decision"` record exists is REJECTED, so a self-declaration cannot dress itself in the strong tier. The escalation record stays a SEPARATE record (one purpose per record: the escalation logs the escalation EVENT; the waiver declares the convergence-bar EXEMPTION that event authorised). This preserves the tier distinction structurally, not by convention.

### How W3 consumes the waiver (and every invariant it preserves)

W3 (`w3_problems`, `src/workflow.rs:150-212`) is modified minimally. For every `complete` Roadmap step it still requires convergence UNLESS a waiver covers the unit:

- A `complete` step with ZERO matching round records is covered ONLY by a step-level waiver (`unit:step`, reason `predates-logging` or `review-skipped`). With no such waiver AND no records it still FAILS. This is the pause.md catch, preserved exactly (invariant 1): the current failure message (workflow.rs:161-167) becomes "no round records and no waiver".
- For each increment whose peak `consecutive_clean` falls short of its `risk_class` requirement, W3 looks for a waiver matching `step`+`increment`. An `accepted-at-escalation` waiver additionally requires (via W5, or inline) the matching escalation `decision` record to exist. If found, the increment passes with an INFORMATIONAL NOTE (not an error), exactly the escalation-exempt semantics decided at `:663`.
- The escalation scope is preserved (invariant 3): the waiver exempts ONLY the streak shortfall; the `risk_class`-consistency guard (workflow.rs:176-185) and every other W3 check still fire even with a waiver present. The `accepted-at-escalation` tier fires only when the backing record is `human_decision:"decision"` (never `resume`), scoped to the one matching increment. Per-increment convergence semantics (invariant 4) are untouched: W3 still groups by increment and takes the peak streak.
- Exemptions stay DECLARED and VISIBLE (invariant 2): the waiver is an explicit on-the-record JSONL line, never an implicit default or a silent code path. There is no status-string escape hatch anymore; `skipped` STAYS a status (E5) because it answers "is this done?" with "no", which is not a below-bar waiver and must not be folded in.

### W5, the new check

W5 (`validate --workflow`) asserts: every waiver names a real Roadmap step (and, for `unit:increment`, a plausible increment); every `independent-record` waiver names an existing backing record; the `reason`<->`evidence` pairing is consistent (no `self-declared` claiming `independent-record`, no `accepted-at-escalation` at the weak tier). This is the structural guarantee that the two tiers are not confusable and that a waiver cannot be silently widened.

### Migrating the 14 grandfathered rows: per-unit waivers vs one baseline marker

The least-radical migration mirrors the current per-row status: one `predates-logging` waiver per grandfathered step (11 b1 steps with zero records, 3 b2 short-streak steps, `:653`), and one `review-skipped` waiver per `trivial` step. That is 14+ records but it is a mechanical, one-time backfill and it keeps the model uniform (every exemption is a waiver). The cleaner alternative, which I flag honestly, is a single `type:"baseline"` marker record naming the regime-start (the commit or timestamp before which no step is expected to have disciplined logging), which W3 consults to exempt any step whose completion predates it, collapsing 14 records to 1. The trade: the baseline marker is cleaner and absorbs E10 globally, but it is a slightly larger conceptual addition (a boundary the checks consult) than a pile of per-step waivers. For the INCREMENTAL lens I recommend the per-unit waivers (they reuse exactly the waiver machinery the pilots already build, no new record type), and I note the baseline marker as the option the clean-slate lens should weigh.

## 5. The migration path (including this repo's own dogfooded plan/ledger)

Dogfood in place, one increment at a time; each increment migrates only the slice it introduces, so the repo's own plan/ledger stays valid and `validate --workflow` stays green after every increment.

1. Build the decision-receipt substrate (pilot A). No migration of existing data (forward-looking; Q-1..Q-41 exempt via the log's absence). Future `decided` items (Q-42/Q-43/Q-44 as they resolve) get a `type:"decision"` record.
2. Build the waiver record + W5 (pilot B). No behaviour change to W3 yet; the record type and validator land first so they can be reviewed in isolation.
3. Modify W3 to consult waivers; retire `trivial`/`grandfathered` from `ROADMAP_STATUSES`. Migrate this repo: convert the 14 `grandfathered` rows and any `trivial` rows back to `complete`, and append the corresponding `predates-logging` / `review-skipped` waiver records to the log. Convert `optional-modules` from its live-stuck `in progress` (SE-4) to `complete` plus an `accepted-at-escalation` waiver for `increment: optional-modules-inc2cii` referencing the existing escalation record (workflow.jsonl:82-83). After this increment, `validate --workflow` on this repo exits 0 with `optional-modules` finally closeable.
4. Add the explicit `step`/`increment` fields to `round`/`decision`/`waiver` records; W3 and the consistency check join on the field, with the lexical `-inc<x>` strip kept only as a fallback for legacy records. Add orphan-task detection (a `validate --workflow` warning for a `task` with no matching Roadmap slug, SE-5/SE-16). Backfill the fields into this repo's existing round records where cheap; leave the rest on the fallback.
5. Refine the queue-item grammar: split `status` from the `folded_into <slug>` pointer (SE-11) and stop re-enumerating option-labels in `decided` prose (the convention the receipt passes already adopted, now with the receipt as their machine home). Optionally split `Question.ask` into a short summary + a `detail` pointer (SE-15). Migrate this repo's queue lines.
6. Extend the projection: decision + waiver slots in `status`/`--json`; derive the Status-line summary and RESUME-STATE pointer; retire the hand-authored Status line per area 3 option 1 (or add the drift-check, option 2). Fold the render into the already-planned `state-queries` step where both receipt explorers agree it belongs. Migrate this repo's plan (delete/replace the Status line) and ledger (RESUME STATE becomes a pure pointer).
7. Close B1: rewrite `pack/LEDGER.template.md` so the round section is prose-commentary-only, pointing at the JSONL round record for the structured facts. Migrate this repo's ledger round narrative to drop the restated facts.

The independent sharp edges the audit flagged as NOT resolved by the pivot (SE-1/SE-2/SE-3/SE-6/SE-7/SE-8/SE-9/SE-17) are a PARALLEL cleanup backlog, scheduled separately and not blocking any increment above.

## 6. The staged, reviewable roadmap

Each increment is small and independently shippable, and each leaves the repo green. The decision receipt and the waiver model are the early PILOTS (Principle 6, prove the architecture on a small slice before touching the plan skeleton).

- INC 1 (PILOT A, decision receipt): `type:"decision"` record + `chosen in options` + `parse_decisions` + W4 + drift-guard + `instrument.md` docs + the no-re-enumerate convention. `src/plan.rs` and the plan-template byte-identical. RISKY (it extends the enforcement backstop, like W3 did) -> two clean rounds. This is the converged substrate, built under Q-44 rather than as a one-off.
- INC 2 (PILOT B, waiver record + W5): the `type:"waiver"` record, its validator (the reason/evidence tiers, the backing-record-exists check), and W5. No W3 change yet. RISKY (enforcement surface) -> two clean rounds.
- INC 3 (W3 consumes waivers + migrate this repo's exemptions): modify W3; retire `trivial`/`grandfathered` statuses; migrate the 14 grandfathered rows, the trivial rows, and `optional-modules` to `complete` + waivers. RISKY -> two clean rounds. Closes SE-4; unifies E2/E3/E4.
- INC 4 (structured step/increment link): explicit `step`/`increment` fields; W3 and the consistency check join on them; orphan-task detection. MODERATE. Closes B6/D14/E6/SE-10, SE-5/SE-16.
- INC 5 (queue field split): `status`/`folded_into` split; option-labels out of `decided` prose; optional `ask`/`detail` split. LOW-MODERATE. Closes SE-11/SE-15, finishes B4.
- INC 6 (projection of derived views): decision + waiver slots in `status`; derive Status line + RESUME-STATE pointer; retire or drift-check the hand-authored Status line; project the status vocabulary. Fold into `state-queries`. LOW. Closes B2/A1/A2, B3, SE-13.
- INC 7 (ledger round-narrative de-dup): `pack/LEDGER.template.md` round section becomes commentary-only. LOW (pack/doc). Closes B1.
- PARALLEL cleanup backlog (non-blocking): SE-1/SE-17 (principle namespace), SE-2 (rename the `trivial` metrics Classification variant now that the `trivial` status is gone, removing the collision), SE-3 (the two-tier no-instrument false-green), SE-6 (drift-guard required/optional gap), SE-7 (magic `instrument.md` filename), SE-8 (dead `ModuleSpec.description`), SE-9 (silently-skipped checks Kinds).

Ordering rationale: the two record-type pilots (INC 1, INC 2) are pure additions to the JSONL + validator + a new check, reviewable in isolation with zero plan-format churn, so they prove the "machine records live in the structured event log, enforced by a W-check, projected by `status`" pattern on the smallest possible slice. Only INC 3 changes W3's behaviour and migrates live data, once the record and its validator are already proven. The plan-Markdown region changes (INC 5, INC 6) come last, after the event-log spine is done, so the human-edited source is touched only when the derived views that justify the change already exist.

## Trade-offs judged against the numbered Project Principles (P1-P8)

- P8 (structured data first, project for humans): PARTIALLY served, and this is where the incremental approach is honestly weakest against the mandate. Population B (rounds, decisions, waivers) fully realises P8: it moves to the structured event log as the single source and is projected for humans. Population A (the Roadmap and queue skeleton) does NOT: it stays human-authored Markdown that a projection READS, rather than a structured source the human view is DERIVED from. So P8's letter ("structured formats as the single source ... derive human-readable views by projection, rather than hand-authoring prose that is also the machine input") is met for the event data but not for the plan skeleton, where the human still hand-authors the machine input (the Roadmap table) directly. My defence: the Roadmap table is already structured and already the single source; it is not PROSE that is also the machine input, it is a STRUCTURED region that is both the human surface and the machine input, which is a materially milder version of the smell P8 targets. But it is not the projected-view ideal, and I will not pretend it is.
- P1 (cleaner long-term architecture, sharpened by P8): genuinely advanced by the waiver unification (one concept replacing two statuses plus a proposed third branch), the explicit step/increment link replacing the lexical strip, and the single-home resolution of B1/B2/B3/B4. The retained two-source split (Markdown skeleton + JSONL records) is coherent and maps onto the two data populations, but it is not the single-artifact ideal a clean slate could reach.
- P2 (minimal by default): strongly favoured (reuses JSONL + the validator + `status`, adds two record types and two checks, no new artifact class). BUT P8 explicitly overrides P2 at this pre-adoption stage, so leaning on P2 is precisely where this proposal is arguing against the current priority. I present the minimality as a genuine benefit while acknowledging the human has demoted it.
- P3 (safe on existing projects): demoted by P8. The incremental path gets the low-churn, low-blast-radius benefit for free (the plan-template stays byte-identical through INC 1-4), but I do not weight it, per the mandate.
- P4 (idempotent): preserved; the projection is read-only and regenerable, and the append-only log is idempotent to re-read.
- P5 (make illegal states unrepresentable): advanced. The `chosen in options` check, the waiver reason/evidence-tier validator (W5), and retiring the ambiguous exemption statuses all remove representable-but-illegal states (a self-declaration masquerading as independent evidence becomes unrepresentable).
- P6 (ground decisions in evidence, prove on a slice): directly embodied by the two record-type pilots (INC 1, INC 2) before any W3 or plan-format change.
- P7 (reproducible): unaffected; no toolchain change.

## What this incremental approach does NOT achieve versus a clean-slate design

Stated plainly so the human can compare:

- The plan Roadmap and queue remain HAND-AUTHORED Markdown. A clean-slate design would make a structured file (or a database of records) the source and render the Markdown plan as a pure PROJECTION, so the human view is fully derived and the skeleton is never hand-typed as the machine input. Incrementalism keeps the skeleton editable-in-place and reaches P8 only for the event data.
- No plan-writer means the in-document status SUMMARY (Status line, RESUME-STATE pointer) is either REMOVED (losing the self-contained-document property) or hand-authored-and-drift-checked (a copy kept honest by a guard, not truly derived). A clean-slate writer renders the summary into the document, getting both self-containment and derivation. This residual is unavoidable here.
- The ledger round NARRATIVE stays a separately hand-authored prose file alongside the structured round record. B1's double-WRITE of the structured facts is closed (the facts live only in the log), but a soft residue remains: the commentary can still contradict the record it comments on, and nothing checks the prose against the log (checking prose content is exactly what the parsers deliberately do not do). A clean-slate design that generated the ledger round section from the log would eliminate even that residue.
- The `-inc<x>` lexical strip is fixed for NEW records via the explicit field, but legacy records keep the fallback until fully backfilled, so the T3 over-strip risk is reduced, not deleted, during the transition.
- The status vocabulary and schema still live in code constants pinned to pack-doc prose by DRIFT GUARDS (E9), not by a single declaration both the validator and the docs read. A clean-slate schema-as-single-declaration would make the drift guards unnecessary; incrementalism keeps them.

## What NOT to build (the YAGNI boundary)

- Do NOT rewrite the plan as a single monolithic structured file (per-step frontmatter + Markdown body for the whole document). The plan is ~80% reasoning narrative; structuring it fights the format, loses direct Markdown editing and self-contained resume value, and is exactly the full-rewrite the receipt passes and `state-schema` deliberately scoped out. If the human wants that, it is the clean-slate design, weighed on its own merits, not this track.
- Do NOT build a generative plan-writer that renders derived content back into the plan Markdown. The tool never writes the plan today; a plan-annotator is the deferred `workflow-viz` capability (`:431`) and owes its own design pass. The projection stays read-only.
- Do NOT add a new sidecar file (a `decisions.toml`, a `waivers.toml`) as a source. Reuse the JSONL event log; a new artifact class needs a new parser, a new validation mode, and a new ownership model for no gain over the log that already exists.
- Do NOT build `escalation-exempt` as a third code branch or a fourth Roadmap status. It is a waiver reason (`accepted-at-escalation`) at the independent-record tier; that is the whole point of the unification.
- Do NOT machine-enforce prose content (the no-re-enumerate-options convention, the RESUME-STATE-is-a-pointer rule). The parsers deliberately do not read prose; keep these as conventions, optionally with a coarse guard (for example a RESUME-STATE line-count warning, SE-14) rather than content parsing.
- Do NOT flatten the two evidence tiers into one. `independent-record` must stay distinguishable from `self-declared`, enforced by W5, so cleanup does not launder a weak self-declaration into looking as trustworthy as an escalation-backed one.
- Do NOT fold `skipped` into the waiver model. It answers "is this done?" with "no, abandoned", which is not a below-bar waiver; keep it a status.
- Do NOT bundle the independent naming/hygiene cleanups (SE-1/SE-2/SE-3/SE-6/SE-7/SE-8/SE-9/SE-17) into this data-model track. They are a parallel backlog; only SE-2's metrics-side rename becomes trivial once the `trivial` STATUS is retired, so schedule that one to follow INC 3.
