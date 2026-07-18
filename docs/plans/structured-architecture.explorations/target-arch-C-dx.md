# Q-44 phase 2, target architecture (Explorer C): the developer / agent experience, projection-centric lens

Lens: DEVELOPER / AGENT EXPERIENCE, PROJECTION-CENTRIC. I design backward from the interaction. The two authors of this workflow state are (i) a human and (ii) an LLM orchestrator, and the system is dogfooded, so the agent's own authoring ergonomics weigh as much as the human's. The single question I keep asking of every datum is: who writes this, with which tool operation, and what happens when they get it slightly wrong? A structured source that is miserable for an LLM to hand-author (deep nesting, id bookkeeping, brittle multi-line cross-refs) gets worked around or corrupted, so authoring ergonomics is a first-class constraint, not a finish. The data model serves the workflow, not the reverse.

## The design question, restated

The human adopted Project Principle 8: the workflow's DATA should live in structured, machine-readable form as the single source, with `agent-scaffold` PROJECTING it into human-readable views, and the three below-bar convergence exemptions (`grandfathered`, `trivial`, the proposed `escalation-exempt`) should collapse into one unified waiver model with two evidence tiers. Backwards-compat and blast-radius are demoted at this pre-adoption stage; optimise for the best long-term design. I must cover, as a coherent whole: (1) the structured source data model, (2) the file format(s) and how a human edits the source, (3) the projection/render and CLI surface, (4) the unified waiver model, (5) the migration path including this repo's own dogfooded plan/ledger, and (6) a staged roadmap with the receipt and the waiver as early pilots. I must preserve the load-bearing invariants: the pause.md catch, exemptions declared and visible, the escalation waiver scope (decision-only, one increment, streak-shortfall-only), per-increment convergence, the strict-validate vs best-effort-projection split, and two waiver evidence tiers.

## The one decision that drives the whole design: authors WRITE events and PROSE; the tool only ever PROJECTS

Before the six areas, the load-bearing DX choice, because it determines the answers to all six. There are two ways to give the workflow a structured single source:

- SOURCE-IS-VIEW (round-trip): keep a rich human-readable document (the plan Markdown as today, or a richer one) as the thing authors edit, and have `agent-scaffold` PARSE it back into structure and re-render it. This is the (b)/(c)-heavy direction the receipt-encoding passes examined.
- SOURCE-IS-SEPARATE (no round-trip): authors write to a structured source in the form that is cheapest and safest to write, and the human-readable document is a PROJECTION the tool derives and never parses back.

I choose SOURCE-IS-SEPARATE, and I choose the structured source's form specifically for LLM and human write-ergonomics: an APPEND-ONLY EVENT LOG for all mutable workflow STATE, plus FREE PROSE Markdown for the reasoning payload, with the two homes DISJOINT so they cannot drift against each other. The tool reads both and PROJECTS the views (`status`, a rendered plan). It never parses a hand-edited rich view back into state. This deletes the entire round-trip failure class (see the authoring-ergonomics section) and it matches what both authors already do well.

Why append-only events are the right write surface, concretely from the interaction:

- An append is the LLM's cheapest and safest write. Emitting one self-contained line at the end of a file needs no cursor placement, no unique-match against existing text, no re-indentation, and no preservation of surrounding structure. Contrast the status quo, where advancing a step means locating and editing one cell of a GFM pipe-table without disturbing the other rows (an Edit with a uniqueness requirement), then re-narrating the Status line, then writing a ledger paragraph, then appending a JSONL round record: four scattered writes for one state change, three of them in-place edits, and the audit records that this hand-sync drifts (B1, B2, B3, and the queue the ledger itself calls error-prone).
- A malformed append is an ISOLATED failure. One bad JSONL line is one bad line: `validate` reports it by number, the best-effort projection skips it, and its neighbours are untouched. A bad in-place edit can corrupt a whole table or a multi-line block. The project already relies on exactly this property for the JSONL log today (`metrics.rs:5-11`, "records are not guaranteed well-formed"; validation is detection).
- An append-only diff REVIEWS well. A workflow-state change becomes a diff that adds lines to one file, each event self-contained and in order. That is far easier to review than today's scattered in-place edits across plan table, Status line, ledger, and log.
- The format already exists and both authors already produce it. The orchestrator hand-writes `docs/metrics/workflow.jsonl` today; `metrics.rs` already owns a per-record schema, a strict validator, and a best-effort projection. I am extending a proven substrate, not inventing one.

The corollary that keeps this ergonomic rather than dogmatic: DERIVE everything that is a fold over events, so no author ever hand-maintains a running value. Statuses, the consecutive-clean streak, the RESUME-STATE counters, the Status line, the queue's decided/superseded markers are all folds; the author appends the events that cause them and the tool computes the current value. The one class of fact I deliberately do NOT event-source is step EXISTENCE and ORDER, because a human's mental model of a roadmap is a list they rearrange spatially; forcing "move this step up" through a `step_moved` event is bad DX. Existence and order stay authored, as the ORDER and PRESENCE of the prose headings themselves (below).

## Area 1: the structured source data model

Two source homes, disjoint by construction.

### Home A: the event log (all mutable STATE)

`docs/workflow/log.jsonl` (successor to `docs/metrics/workflow.jsonl`): append-only JSON Lines, one flat self-contained record per line. This is the single home for every datum that is a STATE TRANSITION or a mutable fact. Flat records only; no nesting beyond the one existing `reviewers[]` array, because nesting is the thing LLMs bookkeep badly. Record families (each carries the common `type`, a `ts`, and its own fields):

- `round` (exists today): one per review round. Keep `artifact`, `phase`, `changed_since_prev`, `outcome`, `valid_findings`, `severities`, `reviewers[]`. Two changes for the DX and the audit's B6/E6: replace the lexical `task` string (`round-log-core-incB`, parsed by `rfind("-inc")`) with two EXPLICIT fields, `step` (the slug) and `increment` (a bare token, `A`/`B`/`1`); and DROP the hand-authored `consecutive_clean`, because it is a fold over the `outcome` sequence and the audit flags it as a stored copy of a derivable value (D10.ii, E8/E16). The author stops maintaining a counter; the tool computes the peak-over-increment streak. This dissolves the `-inc<x>` over-strip risk (T3) and the round-log-consistency check (E8) at once: there is nothing to disagree with once the value is derived.
- `decision` (the receipt; substrate already converged in `receipt-encoding-{A,B}.md`): `q_id`, `options` (non-empty array of strings), `recommendation`, `chosen` (validated as a member of `options`), `ts`. One per queue item that goes decided. This is the machine home for option-labels + choice (closes B4), consumed by W4.
- `waiver` (NEW; the unified exemption, see Area 4): `unit` (a `step` slug or an `increment` reference), `reason` (a code), `tier` (`declared` or `recorded`), and an `evidence` pointer required only at the `recorded` tier.
- `step` lifecycle (NEW): the transitions of a step that are DECISIONS, not folds over rounds. `event` in {`started`, `complete_claim`, `skipped`, `deferred`, `optional`, `abandoned`}. Crucially `complete_claim` is a CLAIM the author asserts and the tool CHECKS; it is not `complete` written directly. This is what preserves the pause.md catch: the claim is the declaration, and W3 demands evidence or a covering waiver behind it. `not started` is the absence of any event for a step (no bookkeeping to add a "not started" record).
- `question` lifecycle (NEW): `id`, `event` in {`opened`, `exploring`, `decided`, `superseded`}, and on `opened` a one-line `ask`. Queue status is the fold; a `decided` event pairs with a `decision` receipt (W4 links them by `q_id`).
- `escalation`, `intake`, `dismissal_recheck` (exist today): unchanged, kept SEPARATE (one purpose per record); `escalation` is the independent record a `recorded`-tier waiver points at.
- `baseline` (NEW; one per project, see Area 4 and 5): the single regime-start marker that carries the historical cutoff all invariants consult, replacing per-step `grandfathered` stamps and per-check hand-rolled historical exemptions (E3, E10).

Derived from this log by fold, never authored: every step's current status, every question's current status, the per-increment convergence streak, the total-round count and cap position, the Status-line summary, and the RESUME-STATE transient counters.

### Home B: the prose payload (the reasoning)

`docs/plans/<task>.md`, authored Markdown, the ~80% that is genuinely a document a human reads and edits: Motivations, Project Principles, Step Details bodies, Success Criteria. The tool NEVER writes here. Its only machine-relevant content is the STRUCTURAL SKELETON carried by the prose's own shape: each `### <slug>:` Step-Detail heading DECLARES that a step exists (the heading id) and the ORDER of the headings is the roadmap order. This is the one authored-structural fact, and it lives where a human already edits it spatially and comfortably; `plan.rs detail_slugs` already reads exactly these heading ids. Dependencies, if a project wants them enforced, are a `depends` field on the `step` `started`/declaration path or a one-line `Depends: <slug>` under the heading; I lean to keeping deps in the log so the prose stays pure prose, but either is defensible and low-churn.

So: the log holds STATE and links to prose by id (`step` slug, `q_id`); the plan holds PROSE and declares the skeleton by heading. Nothing is in both. The Roadmap status column, the Status line, the queue status markers, and the RESUME STATE all LEAVE the authored files entirely; they become projections. What stays prose payload and how it links: Step-Detail bodies stay under their `### <slug>:` heading (linked to the log by slug); a decision's rationale stays in the step it folds into (linked to its `decision` receipt by `q_id`); per-round commentary that is genuinely non-structured stays as an optional prose note, but the structured round facts live only in the log (killing the B1 double-write).

## Area 2: the file format(s) and how a human edits the source

Formats, chosen for who authors them:

- The log: JSON Lines. LLM-native, flat, append-friendly, already validated. The human essentially never hand-writes it (see below).
- The prose: Markdown, unchanged in feel. Directly editable, self-contained for reading, soft-wrapped, exactly the format both authors already use for the plan.

Who edits WHAT, and HOW:

- The ORCHESTRATOR (LLM) is the primary author of the log. Every state change is one append: record a round -> append a `round` line; advance a step -> append `{"type":"step","slug":"optional-modules","event":"started"}`; claim done -> append a `complete_claim`; record a decision -> append a `decision` line and write its rationale into the step prose; waive -> append a `waiver` line. It also edits the prose (adds a `### <slug>:` heading for a new step, writes Step-Detail bodies, folds decision rationale). It NEVER edits a status in place and NEVER hand-writes a streak, a Status line, or a RESUME STATE.
- The HUMAN edits PROSE freely (Motivations, Principles, Step-Detail bodies, Success Criteria) and DECIDES at the human-input contract points; the orchestrator relays a decision into a `decision`/`question` event. The human's READ surface is the projection (`status`, a rendered view), which is strictly nicer than reading raw JSONL. The human should almost never hand-edit the log; the one time they might is a direct correction, which is an appended corrective event (a superseding `decision`, a re-`opened` question), matching the existing "never rewrite past lines" rule and the queue's "mark resolved, do not delete" ethos.

Correction discipline, stated plainly because it is a real DX question: the log is append-only once committed (calibration integrity, cross-task, already the rule at `instrument.md:3`). An uncommitted mistyped line may simply be fixed before commit (it is not yet history, same as fixing a typo in a table cell today). After commit, correct by a new event. `round` records and `decision` receipts are the strictest (permanent calibration and permanent attestations); lifecycle events tolerate an in-session fix. This keeps the "just fix the typo" affordance that makes hand-authored logs bearable, without pretending the file is a distributed event store.

A deliberate NON-choice: I do NOT propose a `agent-scaffold log <event>` write helper as the authoring path. It would reintroduce a runtime dependency on the binary during a workflow run, which `instrument-flag`/`Q-24` deliberately avoided (the harness-agnostic, tool-free direct-write constraint). Direct append stays the contract; the binary is a read/validate backstop, never required to make progress. A helper could exist as convenience later, but it must not become load-bearing.

## Area 3: the projection / render, the CLI surface, and round-tripping

The tool derives every human view from Home A + Home B and writes back to NEITHER source. This is the existing `Projection` pattern (`main.rs:404-431`, `run_status`) generalised.

The render, layered:

- `status` (extend the existing subcommand): default human summary and `--json`. The `Projection` gains derived slots computed by folding the log: `roadmap` (each declared step's slug, order, and DERIVED status), `open_questions` (id, DERIVED status, ask, and for decided items the linked `decision`), `decisions` (the receipts), and `waivers` (unit, reason, tier). The plan half now supplies only the skeleton (slugs and order from headings); the STATUS comes from the log fold, so the projection cannot be stale against a hand-edited table because there is no hand-edited table.
- `status --resume` (the planned `state-queries`/`Q-28` slice): the compaction-checkpoint view, folded live from the log: active step, current per-increment streak, total-round position vs the cap, open/exploring questions, any pending dismissal re-check, and the next required action. This REPLACES the hand-authored `## RESUME STATE` block (B2/A2, SE-14): it is always fresh because it is a fold, and it cannot drift because nobody writes it. The reasoning a resuming agent needs is still in the authored plan prose; the live state is one command away.
- `next` (planned `state-queries`): the single next required action, derived, reusing the W3 transition logic. Doubles as the enforcement aid, as `Q-28` already intends.
- A full `view` render (the plan-with-status Markdown, prose interleaved with the projected Roadmap and queue) is OPTIONAL and later. If built, it writes to a clearly-marked GENERATED file (for example `<task>.view.md`, header "generated by agent-scaffold view, do not edit") or to stdout, never to the authored plan. I would DEFER it (YAGNI): the authored plan prose plus `status`/`--resume` already cover reading and resume; a committed rendered file is convenience, not correctness. It is explicitly NOT the deferred `workflow-viz` streaming Gantt (a live dispatch/return visualiser); this is a one-time read of committed data, the same shape `status` already is, exactly the distinction both receipt explorers drew.

Round-tripping: there is NONE. Authors write the source (append events, edit prose); the tool projects views and parses them back never. The only "round-trip"-shaped risk is a generated `view` file going stale, which is COSMETIC because the view is derived and regenerable, never a source of truth. This is the decisive DX simplification over the (b)/(c)-heavy directions: the failure mode "the tool clobbered my hand-edits" and the failure mode "my hand-edit corrupted the structured block the parser needs" both cannot occur, because the tool does not write the authored files and the parser does not read structure out of a rich hand-edited view.

Strict-validate vs best-effort-projection split (preserved): `validate` is the strict layer. `validate --log` checks every event against the schema and reports malformed lines by number (today's metrics validate, extended). `validate --plan` checks the prose skeleton (well-formed headings, no duplicate slugs, every log-referenced slug/`q_id` resolves to a heading/question). `validate --workflow` runs W3 (convergence-or-waiver) and W4 (decided-has-receipt) over the fold. The projection (`status`, `view`) stays best-effort: it skips malformed lines and renders what it can, exactly as `parse_rounds`/`parse_questions` do now. Both layers run; the projection's silence on a dropped line still depends on `validate --log` catching it, which is the invariant to keep (E11).

## Area 4: the unified waiver / exemption model

One concept: an AUTHORISED WAIVER of the convergence requirement for a named unit. It absorbs `grandfathered`, `trivial`, and `escalation-exempt`, which the enforcement audit shows are one thing wearing three hats (E2/E3/E4, and the audit's synthesis). A waiver is an explicit `waiver` event, which is what keeps exemptions DECLARED and VISIBLE (the invariant): a waiver is an on-the-record artifact, never a silent default or an implicit code path.

Schema (flat, one-line, append):

```
{"type":"waiver",
 "unit":{"step":"<slug>"}  OR  {"increment":{"step":"<slug>","increment":"<token>"}},
 "reason":"predates-logging" | "review-skipped" | "accepted-at-escalation",
 "tier":"declared" | "recorded",
 "evidence":"<pointer>",   // REQUIRED iff tier == "recorded"; forbidden otherwise
 "ts":"..."}
```

The TWO evidence tiers (the load-bearing distinction the audit insists must not be laundered away):

- `declared`: the orchestrator/human asserts the waiver; no independent artifact backs it. This is the WEAK tier. It absorbs `trivial` (`reason: review-skipped`, a deliberately review-skipped low-stakes step) and `grandfathered` (`reason: predates-logging`, a legacy step). The waiver record IS the visible declaration, so the declared-and-visible invariant holds even though the evidence is self-asserted.
- `recorded`: the waiver is backed by an INDEPENDENT durable record already in the log, named by `evidence`. This is the STRONG tier. It absorbs `escalation-exempt` (`reason: accepted-at-escalation`, `evidence` -> the matching `type:"escalation" human_decision:"decision"` record). The tool VALIDATES that the pointed-to record exists and matches the unit, so a `recorded` waiver cannot be forged by assertion alone: there must be a real escalation event. This preserves escalation's stronger evidence and prevents a weak self-declaration from being dressed as a record-backed one.

The tool renders the tier in every view, so a reader sees at a glance which waivers are self-declared and which are record-backed. That visible tiering is the concrete mechanism that satisfies "do not launder a weak self-declaration into looking as strong as an escalation record."

How W3 consumes it (convergence-or-waiver, per increment, preserving every scope invariant):

- For each step with a `complete_claim`, W3 requires, per increment, EITHER convergence (peak-over-increment `consecutive_clean` derived from the outcome sequence reaches the `risk_class` required streak, with the risk_class-consistency guard still live) OR a covering waiver.
- A `step`-unit waiver (reason `predates-logging` or `review-skipped`) covers the whole step: this is exactly what `trivial`/`grandfathered` did by being a non-`complete` status, now expressed as an attribute of a claimed-complete step rather than a substitute status for it (the E1 conflation, "is it done?" vs "why is its evidence short?", is finally separated).
- An `increment`-unit waiver (reason `accepted-at-escalation`, tier `recorded`) covers ONE increment's STREAK SHORTFALL ONLY: it fires only when the `evidence` escalation carries `human_decision:"decision"` (never `resume`), is scoped to the one matching increment, and exempts only the clean-streak shortfall while risk_class-consistency and every other W3 check still apply. This is invariant 3 verbatim, now a data record instead of a third code branch.
- THE PAUSE.MD CATCH IS PRESERVED: a `complete_claim` with neither convergence nor a covering waiver still FAILS. Undeclared complete-with-no-evidence remains an error. This is the whole reason W3 exists (`Q-27`, the `pause.md` incident) and the unification does not weaken it; it only changes the exemption from a status-string to an explicit, tiered, checkable record.
- `skipped`/`abandoned` stay DISTINCT from waivers: they answer "is it done?" with "no", so nothing is demanded of them (E5). A waiver is only for "done, but the evidence is absent or short."

How W4 consumes it: W4 requires every decided question to have a `decision` receipt. The historical exemption (Q-1..Q-41 predate the mechanism) is NOT a per-check special case; it is the single `baseline` marker (below) that both W3 and W4 consult. This absorbs E10 ("history predates the regime" recurring per-check) into one home.

The `baseline` marker (one per project): a single event carrying the regime-start cutoff (a slug boundary or a timestamp) with `reason: predates-logging`. Steps and decisions before it are auto-waived for the checks that postdate them. This replaces the 14 `grandfathered` rows with ONE explicit, visible marker, which is arguably MORE visible than 14 identical status strings and stops the legacy boundary from being frozen into the shared Roadmap status vocabulary that every scaffolded project inherits (E3's sharp edge). New below-bar cases after the baseline get their own explicit per-unit waivers.

## Area 5: the migration path (including this repo's own dogfooded plan and ledger)

Because status becomes derived, the migration is mostly ADDING event families and DELETING hand-maintained restatements, not rewriting prose. Staged so the repo stays green at each step.

- The `decision` record and W4 arrive first (the receipt pilot), against the log; `src/plan.rs` and the plan format stay byte-identical, exactly as both receipt explorers established. This validates the "append an event, derive a check" loop on the smallest possible surface.
- The `waiver` record, the `baseline` marker, and the W3 refactor arrive next (the waiver pilot). For THIS repo: the 14 `grandfathered` rows collapse to one `baseline` event; the `trivial` rows become `declared` step waivers with `reason: review-skipped`; and `optional-modules` increment 2c-ii gets a `recorded` waiver pointing at its existing `type:"escalation" human_decision:"decision"` record at `workflow.jsonl:82`. That last one UNSTICKS the live-stuck step (SE-4): `optional-modules` can finally be `complete_claim`'d without a false W3 flag, which is the concrete symptom the human named and the reason the waiver model is an early pilot.
- Event-sourced step and question lifecycle arrive next: append `started`/`complete_claim`/`skipped` and `opened`/`exploring`/`decided`/`superseded` events; DERIVE the statuses; DELETE the Roadmap status column, the queue status markers, the Status line, and the stale Documentation-Protocol vocabulary prose (B3, now projected from the schema, not hand-listed in three places). The plan keeps its `### <slug>:` headings (skeleton) and its prose. The `round` record migrates from the lexical `task` to explicit `step`/`increment` fields and drops hand-authored `consecutive_clean`.
- The ledger double-write dissolves last (B1): the ledger's structured round facts already live in the log, so the ledger keeps only genuinely non-structured commentary and its RESUME STATE becomes `status --resume`. The streak-consistency check (E8) retires because the streak is derived. Where a single structured schema now feeds both the validator and a docs projection, the prose-to-code drift guards (E9) can retire too, because there is one source and nothing to drift.

The prose is never mass-rewritten; the Step-Detail bodies, Motivations, and Principles are untouched by the migration. What moves is STATE, out of the prose and into the log, plus the deletion of the hand-synced restatements the audit flags.

## Area 6: a staged, reviewable roadmap (receipt and waiver as the early pilots)

Each stage is a reviewable increment that leaves the repo green.

- Stage 0, RECEIPT PILOT: the `decision` record + `chosen in options` + `metrics::parse_decisions` + W4 + drift-guard + `instrument.md` docs + the no-re-enumerate convention. Smallest surface, converged substrate, validates the append-then-derive loop. RISKY (it extends the enforcement backstop like `workflow-invariants`/W3 did), so two clean rounds.
- Stage 1, WAIVER PILOT: the `waiver` record, the `baseline` marker, the two tiers with the `recorded`-tier evidence validation, and the W3 refactor to convergence-or-waiver. Migrate this repo's grandfathered/trivial/escalation cases and mark `optional-modules` complete. RISKY (it touches the pause.md catch), two clean rounds; the W3 test suite in `workflow.rs` is the regression net.
- Stage 2, EXPLICIT LINKS: replace the `round` record's lexical `task` with `step`/`increment` fields and derive the streak (retire the `-inc` strip and the consistency check). RISKY, because it moves the convergence data model.
- Stage 3, LIFECYCLE + PROJECTION: event-source step and question lifecycle, derive the statuses, extend `status`/`--json`/`--resume` and add `next` (the planned `state-queries` work), and DELETE the Roadmap status column, Status line, queue markers, and RESUME STATE. This is the largest stage; it is last because Stages 0 to 2 have already proven the append-then-derive-then-project pattern on smaller surfaces.
- Stage 4, CLEANUP (optional): the full `view` render if wanted, and retiring the drift guards that a single schema makes redundant. YAGNI-gated.

Sequencing rationale from the DX lens: the receipt and waiver pilots are first not only because the human named them, but because they are the increments where an author's WRITE is a pure append and the tool's job is a pure derive-and-check, with zero change to the authored prose or the plan format. They prove the model at its lowest risk before Stage 3 asks authors to stop hand-writing statuses at all.

## Trade-offs against the numbered Project Principles (P1 to P8)

- P8 (structured data first, project for humans): this IS P8. State has one machine home (the log); prose has one machine home (the plan); views derive. P8 explicitly wins over P2/P3 at this stage and sanctions the added machinery.
- P1 (cleaner long-term architecture over smallest diff): strongly served. The append-then-derive-then-project pattern is one coherent shape reused across rounds, decisions, waivers, and lifecycle, replacing a pile of hand-sync conventions. The event log generalises the JSONL the project already committed to in `round-log-core` (`Q-34`).
- P2 (minimal by default; adding a module must not complicate the core): in TENSION, and P8 says P8 wins here. The core gains event families, a fold, and a richer projection. I keep the tension small by reusing the existing JSONL substrate and the existing `Projection`/`run_status` machinery rather than inventing new artifact classes, and by DELETING as much hand-sync as I add (net conceptual load is lower even if line count is higher).
- P3 (safe on existing projects): also demoted by P8 at this stage. The mitigating design choice is that the tool never writes the authored files, so there is zero clobber surface on anything a human edits; the blast radius is on the plan FORMAT (status leaves the plan), which is the intended, human-sanctioned change.
- P4 (idempotent): preserved and strengthened. The tool is read-only over the source; projections are pure functions of the log + prose, so re-running produces identical output by construction.
- P5 (make illegal states unrepresentable): strongly served. A derived status cannot disagree with its evidence; "complete but no evidence and no waiver" is caught, not silently representable; the streak cannot disagree with the outcome sequence because it is computed, not stored.
- P6 (ground decisions in evidence): the staged pilots ARE proofs-of-concept; the receipt validates the pattern before Stage 3 commits to it.
- P7 (reproducible): unaffected; JSONL and Markdown are toolchain-neutral, and the tool-free direct-write contract is preserved (no runtime binary dependency during a workflow run).

The honest cost: P2/P3 are genuinely spent here, and the core is bigger. P8 authorises that, and the DX return (append-mostly authoring, no hand-sync, no drift, clean diffs, always-fresh resume) is where the spend is recovered.

## Authoring ergonomics and round-tripping failure modes (the first-class section)

Who authors what, and with which operation:

| Datum | Author | Operation | Failure blast radius |
| --- | --- | --- | --- |
| A round | orchestrator | append one `round` line | one bad line, isolated, `validate` names it |
| Advance a step | orchestrator | append one `step` line | one bad line, isolated |
| A decision | orchestrator | append one `decision` line + write rationale prose | one bad line + normal prose edit |
| A waiver | orchestrator/human | append one `waiver` line | one bad line, isolated |
| Add a step | orchestrator/human | add a `### <slug>:` heading + prose | normal prose edit |
| Step order | human | move a heading (spatial) | normal prose edit |
| A status, a streak, the Status line, RESUME STATE | NOBODY | derived | cannot drift; nothing to author |
| Step-Detail body, Motivations, Principles | human/orchestrator | edit prose freely | normal prose edit |
| A decision (human side) | human | decide at the contract; orchestrator records | none for the human |

The LLM-authoring worries the prompt names, addressed head-on:

- Deep nesting: avoided. Records are flat one-liners; the only array is the existing `reviewers[]`. There is no structure to descend into.
- Id bookkeeping: bounded to two id kinds. Slugs are human-chosen memorable strings authored once as a heading; `Q-n` is sequential. The tool projects "next free id" so the agent never scans the file to pick one, and `validate` catches any dangling reference, so a mistyped `q_id` or `step` slug is reported, not silently accepted. This is the one residual ergonomic cost, and it is a report-on-slip, not a corrupt-on-slip.
- Brittle cross-refs: the two cross-refs (slug <-> heading, `q_id` <-> `decision`) are validated joins, not lexical string-stripping. The `-inc<x>` `rfind` hack (the most brittle current cross-ref) is replaced by explicit `step`/`increment` fields, so the T3 over-strip class disappears.
- The "miserable to hand-author so it gets worked around or corrupted" failure: the append-mostly design is the direct mitigation. The operations an author performs most often (record a round, advance a step) are the cheapest and safest writes available, and the values that were most error-prone to hand-maintain (statuses, streak, the queue the ledger called error-prone) are removed from authoring entirely.

Round-tripping failure modes, compared:

- SOURCE-IS-VIEW (rejected): the tool must parse a hand-edited rich document back into structure and re-render it. Failure modes: an LLM editing a large rendered document in place corrupts the structured region the parser needs (the exact brittleness the strict/best-effort split fights today); the tool's re-render clobbers hand-edits (the heavy-(c) hazard the receipt passes rejected); and the parse-back is lossy and ambiguous for anything prose-shaped. These are the dominant, recurring failures of the round-trip approach.
- SOURCE-IS-SEPARATE (chosen): no parse-back exists, so none of the above can occur. The residual failure is a generated `view` file going stale, which is cosmetic (regenerate) because the view is never a source. This asymmetry, a whole failure class deleted versus one cosmetic failure introduced, is the core reason the DX lens picks this model.

How the three key moments FEEL:

- Editing: the orchestrator appends a line; the human edits prose or answers a question. Neither locates-and-edits a status cell, re-narrates a summary, or maintains a counter.
- Reviewing a diff: a state change is an append-only delta to one file, each event self-contained and in order, trivially readable, instead of today's scattered in-place edits across plan table, Status line, ledger, and log.
- Resuming after a compaction: run `status --resume`; the whole log folds to the current step, streak, cap position, open questions, pending re-checks, and next action, always fresh because it is derived, never a stale hand-written RESUME STATE. The reasoning a resuming agent needs is still in the authored plan prose.

## What NOT to build (the YAGNI boundary)

- Do NOT force the Step-Detail prose (~80% of the plan) into structured fields. Prose stays prose, linked by heading id. Fully structuring a reasoning-heavy document fights the format and loses direct Markdown editing (the caution both `Q-43` and the receipt passes recorded).
- Do NOT build a plan-WRITER that edits the authored plan in place, and do NOT parse a hand-edited rich view back into state. No round-trip, no clobber surface. This is the load-bearing YAGNI line.
- Do NOT event-source step ORDER via move-events. Order is heading position, edited spatially.
- Do NOT build the streaming `workflow-viz`. The projection is a one-time read of committed data, a different scope; keep them separate as both receipt explorers noted.
- Do NOT invent a new bespoke sidecar format. Reuse JSONL; its schema, validator, and projection patterns already exist.
- Do NOT keep a hand-authored `consecutive_clean` once the streak is derived, and do NOT keep the round-log-consistency check it justified.
- Do NOT keep per-check historical exemptions or 14 `grandfathered` status stamps. One `baseline` marker, consulted by all invariants.
- Do NOT deeply nest event records; keep them flat one-liners.
- Do NOT make an `agent-scaffold log <event>` write helper the authoring path; it would reintroduce the runtime binary dependency `instrument-flag`/`Q-24` avoided. Direct append is the contract; a helper, if ever added, must stay optional.
- Do NOT add `q_id` to `escalation`/`intake` records to double them as receipts; one purpose per record. A decided item that also escalated gets its own `decision` receipt and, if below the bar, a `recorded` waiver pointing at the escalation.
- Do NOT commit a generated `view` file as authoritative; if built at all, mark it generated and treat it as disposable. Defer it entirely until `status`/`--resume` prove insufficient.
