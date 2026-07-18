# Q-44 phase 2: target architecture, clean-slate / source-of-truth-first lens

Explorer lens: clean-slate / source-of-truth-first. The single question this document keeps asking: what is the IDEAL structured source if we start from scratch, optimising for Principle 16 (one source, derive the rest), Principle 1 (clean long-term architecture), and Principle 8 (a structured file or a small set of files as the single source, with human-readable plan/ledger/status fully derived by projection)? Backwards-compat and blast-radius are explicitly demoted at this pre-adoption stage; they inform cost sections but do not govern design choices.

---

## The design question, restated

The workflow operates on a web of related data: the plan Roadmap (step slugs, statuses, order), the Open Questions queue (id, status, ask, options, choice, folding target), the event log (round records, escalation records, intake records, decision receipts), waivers/exemptions, and cross-references tying these together. Today, this data lives in three homes: a hand-authored Markdown plan (two structured regions parsed by `src/plan.rs`, everything else prose), a hand-authored JSONL event log (parsed by `src/metrics.rs` and `src/workflow.rs`), and the ledger (parsed by nothing). The audit named the major single-source smells: the ledger round narrative vs the JSONL round record (B1, acknowledged double-write), the Status line and RESUME STATE both re-narrating the Roadmap (B2/A1), decision option-labels with no structured home (B4), the status vocabulary living in three places with the live plan stale (B3), and the round-to-step relationship carried only as a lexical `-inc<x>` suffix (B6). The enforcement audit named the three below-bar exemptions (`grandfathered`, `trivial`, and the proposed `escalation-exempt`) as one concept wearing three hats, needing unification (E2/E3/E4).

Project Principle 8 states the pivot: a structured file or a small set of files is the single source for the workflow's data, and `agent-scaffold` projects that source into the human-readable views. The receipt-encoding explorations converged on the substrate for decision receipts: a `type:"decision"` JSONL record with `q_id`, `options`, `recommendation`, `chosen` (validated as a member of `options`), and `ts`, plus a W4 check. This document places that substrate in the larger model without re-litigating it.

---

## Area 1: The Structured Source Data Model

### What is structured and what stays prose

The structured skeleton holds every datum the tooling produces or consumes: step identities and statuses, question identities and outcomes, waiver declarations, and the cross-references between them. The prose payload is everything that exists to be read by a human reasoning about the work: step design/outcome rationale, question body narrative (the extended "why" behind the options), per-round commentary in the ledger, and the project motivations. Prose is authoritative for its content and is never machine-consumed.

Concretely:

Structured (every field is a typed value the validator, W3, W4, and the render engine read):
- Step: slug, title, status, order (integer), `blocked_by` (slug, optional), `detail_ref` (pointer to the prose sidecar)
- Question: id, status (an enum), ask (short, one line), options (string array, populated when decided), recommendation (string, optional), chosen (string, optional, must be in options), folded_into (step slug, when decided), `body_ref` (pointer to the prose sidecar)
- Waiver: id, unit_step (step slug), unit_increment (increment id, optional), reason (reason code), evidence_tier, evidence_ref (optional JSONL line pointer), note (short prose, optional)
- Principle: number (integer), text (prose, but a one-liner the render uses for the numbered list)

Prose payload (read only by humans; not machine-consumed):
- Step detail body: the design, decisions, and outcome rationale for a step (can be many paragraphs)
- Question body: the extended rationale and option elaboration (can be thousands of words)
- Ledger per-round narrative: what was reviewed, the findings, the verdicts, the convergence decision
- Plan meta prose: the Motivations section and the Success Criteria (narrative, not an enumerated checklist)

### The data schema in detail

**Steps.** Each step has a slug (kebab-case, stable once created), a short human title, a status from a closed set, an integer order (the render uses this to sequence the Roadmap table), and an optional `blocked_by` field when the step is blocked. The status set is cleaned of the exemption-as-status pattern: `complete`, `in-progress`, `not-started`, `next`, `deferred`, `optional`, `skipped`, and `blocked`. `grandfathered` and `trivial` are REMOVED as statuses; they become waiver reason codes (see Area 4). A step is `complete` if it is done; waivers explain absent or short evidence separately.

**Questions.** Each question has a stable id (Q-n), a status from `open | exploring | decided | superseded`, a short ask (one line), and structured outcome fields that are populated only when the question is decided: `options` (the presented list), `recommendation` (the recommended option), `chosen` (the human's selection, validated as a member of `options` at schema-check time), and `folded_into` (the step slug the decision was folded into). The `decided` status replaces the current parametric `decided -> folded into <slug>` string, and `folded_into` becomes a proper separate field. This eliminates SE-11 (the prefix-match hack). The question's body prose (the extended rationale, the option elaboration, the principles-judged trade-off analysis) lives in a separate prose payload; the `ask` field is a one-liner summary only.

**Decision receipts (the converged substrate).** The JSONL `type:"decision"` record lives in `docs/metrics/workflow.jsonl` alongside the existing round, escalation, dismissal_recheck, and intake records. Fields: `q_id` (matching a question's `id`), `options` (string array matching the question's `options`), `recommendation`, `chosen` (validated as a member of `options`), and `ts`. W4 requires that every `decided` question in the structured source has a matching `q_id` receipt in the JSONL. The JSONL receipt carries the timestamp (durable audit evidence of when the decision was presented and taken); the question entry in the plan carries the live state (what was decided). These two overlap intentionally on `options` and `chosen`; W4 cross-checks that they agree.

**Waivers.** Each waiver declares that a specific unit (a step or one increment of a step) is done despite absent or short convergence evidence. Fields: `id` (stable, e.g. W-1), `unit_step` (the step slug), `unit_increment` (optional; absent means a step-level waiver, present means an increment-level waiver), `reason` (one of three reason codes: `predates-logging`, `review-skipped`, `accepted-at-escalation`), `evidence_tier` (`self-declared` or `record-backed`), `evidence_ref` (optional JSONL line number, required when `evidence_tier = "record-backed"`), and `note` (optional short prose). The `self-declared` tier is the honest label for a waiver backed only by the orchestrator's assertion in the plan (the same evidence as the current `grandfathered` and `trivial` statuses). The `record-backed` tier is the stronger tier backed by an independent JSONL record (the escalation case).

**Round records.** The existing JSONL round records gain two new structured fields going forward: `step_id` (the Roadmap step slug this round belongs to) and `increment_id` (the increment within that step, e.g. "incA", optional). These replace the lexical `-inc<x>` suffix convention. The `task` field is retained for backwards compatibility during the transition period; W3 uses `step_id` when present and falls back to the `leading_slug` strip for old records that predate the migration. This closes SE-10 and B6.

**Principles.** Each principle has a number (integer), a name (short string), and a text (the principle body). The render engine numbers them in the rendered Markdown. Code can cite "Principle 1" meaning the first entry in the structured list, unambiguously. This closes SE-1 (namespace collision) by giving project principles a structured home distinct from the scaffold's AGENTS.md principles (which remain in `pack/principles.toml`).

### What the structured source does NOT hold

The structured source does not hold: the ledger per-round narrative (genuine transient prose), the step detail prose bodies (the reasoning narrative), the question body prose (the rationale), the RESUME STATE content (transient pointer, deleted at task close), or the per-reviewer findings (live in the review files under `docs/plans/<task>.reviews/`). These stay prose and are not parsed.

---

## Area 2: File Format(s) and the Editing Story

### Format: TOML

The plan's structured source is a TOML file: `docs/plans/<task>.plan.toml`. TOML is chosen for three reasons. First, it is already the project's format for structured data (`pack/principles.toml`, `pack/checks.toml`, `pack.toml`), so the Rust toolchain already has `toml` crate support and the team knows the format. Second, TOML's `[[table]]` arrays map cleanly onto the step/question/waiver/principle lists, and its literal multi-line strings (`'''...'''`) can hold the short prose fields (notes, the principle text, the ask) without escaping. Third, TOML is human-editable without tooling: a developer opens the file in any editor, edits the fields, and saves; there is no compile step for the source itself.

The JSONL event log stays in the existing format, extended with the two new round fields (`step_id`, `increment_id`) and the existing `type:"decision"` receipt record.

### One file or several?

The plan has two layers: a structured skeleton (the step list, the question list, the waiver list, the principle list, plan meta) and prose payloads (the step detail bodies and the question body narratives). These two layers could live together in a single TOML file (prose as multi-line strings) or split across a TOML skeleton plus Markdown prose files.

The clean-slate recommendation is a two-layer split:

- `docs/plans/<task>.plan.toml` holds the entire structured skeleton. Prose payloads that are short (the `note` field on a waiver, the `ask` on a question, the `text` on a principle) live inline as strings. Prose payloads that are long (step detail bodies, question body narratives) are referenced by a `detail_file` or `body_file` field pointing to a path relative to the plan's directory.

- `docs/plans/<task>.steps/<slug>.md` holds the prose body for each step. This is a plain Markdown file, no frontmatter; it is the authoritative prose payload for that step's design and outcome reasoning.

- `docs/plans/<task>.questions/<id>.md` holds the prose body for each question. Same plain Markdown format.

The honest cost of this split is more files on disk. The current dogfooded plan has approximately 48 step detail blocks and 44 question items; in the target architecture that is up to 92 prose sidecar files. However, each file is focused and directly editable; there is no navigating a 700-line document to find step detail prose.

The alternative (inline multi-line strings in the TOML) avoids the file proliferation but makes the TOML file very large and degrades the editing experience for prose: editors do not render TOML multi-line strings as Markdown, so prose editing loses syntax highlighting, word-wrap, and preview. For a document that is 80% prose by word count, this is a real cost. The two-layer split is the right trade-off.

### The editing story

An orchestrator or human working with the plan:

- Edits `plan.toml` to change a step status, add a waiver, update a question outcome, or add a new step entry (including its `detail_file` pointer). This is structured data editing and TOML is appropriate.

- Edits `steps/<slug>.md` to write or update the design/outcome reasoning for a specific step. This is prose editing in a natural Markdown file.

- Edits `questions/<id>.md` to extend the rationale narrative for a question. Same prose editing story.

- Runs `agent-scaffold render` to regenerate `docs/plans/<task>.md` after any edit. The generated Markdown is what the team reads when reviewing the plan as a document. It is committed to version control as a derived artifact (so it is readable without the tool), but it is never hand-edited.

- Appends to `docs/metrics/workflow.jsonl` directly (the existing orchestrator-writes-JSONL convention is unchanged).

The key new constraint: the generated `docs/plans/<task>.md` is not a source. An agent that edits the generated Markdown directly has made a change that will be overwritten on the next render. This must be documented in `AGENTS.md` and the plan template, and `agent-scaffold validate` should warn when the generated plan's checksum does not match a fresh render (detecting stale or hand-edited derived files).

---

## Area 3: Projection / Render

### The render pipeline

`agent-scaffold render` reads the structured source (the TOML + prose sidecar files) and writes the human-readable Markdown plan to `docs/plans/<task>.md`. The render is one-way: source -> output, never the reverse.

The generated plan contains:

- A header section with the plan title and a derived status summary (computed from the step status distribution, the open question count, and the waiver count). This replaces the hand-written Status line (D1/A1), eliminating the drift that the audit found (the Status line asserting "no open questions" while Q-42/Q-44 were `exploring`).

- The Project Principles section: a numbered list rendered from `[[principle]]` entries in the TOML. This resolves B8 and SE-1 by single-sourcing principle text in the TOML; code can cite "Principle 1" and the render generates the human-readable list.

- The Documentation Protocol section: rendered from a fixed template fragment (the vocabulary lists and the rules). The Roadmap status vocabulary is generated from the `ROADMAP_STATUSES` constant in `src/plan.rs`, resolving B3 (the live plan's stale vocabulary prose). The drift-guard tests (E9) become unnecessary for this section because there is one source.

- The Open Questions section: one rendered list item per `[[question]]` entry. The item renders: id, status, ask, and (when decided) the options and chosen fields inline. The question's body prose is included by reference from `questions/<id>.md`. This replaces the hand-written mixed-structure question items, and the `decided -> folded into <slug>` prefix-match hack becomes a rendered field display.

- The Roadmap table: rendered in order by `step.order`, with each row showing the slug and status. Waivers are indicated in the table (e.g. a `complete` step with a step-level waiver shows a note in the table, so readers can see the exemption at a glance).

- The Step Details section: one block per step in order, with the heading generated from slug and title, followed by the step's detail prose included from `steps/<slug>.md`.

- The Success Criteria section: rendered from the TOML `[meta].success_criteria` string.

### The CLI surface

New subcommand: `agent-scaffold render [--plan <task>.plan.toml] [--output <path>]`. Reads the TOML source and writes the Markdown output. Exits non-zero if the source is invalid (missing referenced prose files, schema violations). This is a strict operation (not best-effort): a broken source must not produce a partial render that agents will then read as authoritative.

`agent-scaffold status` is extended: when the `--plan` argument points to a `.plan.toml` file (detected by extension), it reads from the TOML source directly via a new `plan::parse_toml` projection. When it points to a `.md` file (the current behaviour), it reads from the existing Markdown parsers. This gives backwards compatibility during the migration period. The `--json` output gains a `decisions` slot (the `DecisionProjection` from the receipt-encoding explorations, populated from `metrics::parse_decisions`) and a `waivers` slot (a serialised list of waiver entries from the TOML).

`agent-scaffold validate` gains a `--source` mode that validates the TOML directly (schema check, cross-references, waiver integrity). The existing `--plan` mode (which validates the generated Markdown) is kept for the transition period and eventually deprecated. The `--workflow` mode (W3 + W4) is extended to read the TOML source when available, with the Markdown parse as a fallback.

### Round-tripping

There is no round-trip: the TOML is the source, the Markdown is the output. An agent reads the Markdown for context (or runs `agent-scaffold status` for a compact projection). An agent writes structured changes by editing the TOML. The render regenerates the Markdown. This is the same model as a compiled artifact: the source is edited, the artifact is regenerated, both are committed (the artifact so it is readable without the tool).

A `validate --source` check detects staleness: after any TOML or prose-sidecar edit, running `validate` without first running `render` is caught if `validate` includes a "generated plan matches current source" check. The implementation is a content hash of the generated plan stored in the TOML meta, updated by `render`, checked by `validate`.

---

## Area 4: The Unified Waiver / Exemption Model

### One concept, three reason codes, two evidence tiers

The three current below-bar exemptions (`grandfathered`, `trivial`, and the proposed `escalation-exempt`) share one function: they tell W3 that a specific unit is done even though the normal convergence evidence is absent or short. The audit found that they currently wear three structural hats (two Roadmap status values and a proposed third code branch), which proliferates special cases in the status vocabulary and the enforcement logic.

The unified model is a `[[waiver]]` entry in the plan TOML. A waiver:

- Is attached to a unit (a step, or one increment of a step). Step-level waivers (`unit_step` present, `unit_increment` absent) exempt the entire step. Increment-level waivers (`unit_step` and `unit_increment` both present) exempt only the named increment's streak shortfall while leaving every other W3 check live.

- Carries a `reason` code:
  - `predates-logging`: absorbs `grandfathered`. The step or increment predates the round-logging regime and cannot retrospectively produce review evidence.
  - `review-skipped`: absorbs `trivial`. The review loop was deliberately skipped as low-stakes; the orchestrator declares this explicitly.
  - `accepted-at-escalation`: absorbs `escalation-exempt`. The human accepted a below-streak convergence at the escalation decision point; a JSONL escalation record is the backing evidence.

- Carries an `evidence_tier`:
  - `self-declared`: the waiver's authority rests on the orchestrator's assertion in the TOML. No independent record backs it. This is the honest tier for `predates-logging` and `review-skipped`.
  - `record-backed`: the waiver's authority is backed by an independent, durable JSONL record. The `evidence_ref` field carries the line number (e.g. "L82") of the `type:"escalation"` `human_decision:"decision"` record in `workflow.jsonl`. This is the stronger tier for `accepted-at-escalation`.

The evidence tiers are preserved and reported visibly. `agent-scaffold validate --workflow` names the tier for each waiver it reports: a `self-declared` waiver is reported as such, not dressed up as equal to a `record-backed` one. This preserves the audit's finding that escalation's independent record is genuinely stronger evidence and must not be laundered down to the level of an orchestrator self-assertion.

### How W3 consumes the waiver model

W3's logic changes from a status-filter to a two-pass check:

Pass 1: for each step that is `complete`, look for at least one converged increment (peak `consecutive_clean` >= `risk_class.required_streak`). If found, the step passes.

Pass 2: if no converged increment is found, look for a step-level waiver (`unit_step = <slug>`, no `unit_increment`). If found, the step passes with an informational note naming the reason code and evidence tier.

For increment-level shortfall: if a step's increment has peak `consecutive_clean` below the required streak, look for a waiver with `unit_step = <slug>` AND `unit_increment = <inc>`. If found and the `reason = "accepted-at-escalation"` and `evidence_tier = "record-backed"`, the increment's streak shortfall passes. The risk_class-consistency guard (E7) and every other W3 check remain live; the waiver exempts ONLY the streak shortfall for the matched increment.

The pause.md catch is preserved: a `complete` step with no converged increments AND no declared waiver still fails W3. An undeclared completion cannot pass.

### Disposition of the current exemption cases

The 14 `grandfathered` Roadmap steps become `complete` steps with `[[waiver]]` entries carrying `reason = "predates-logging"` and `evidence_tier = "self-declared"`. A single batch waiver with a note ("11 steps have zero records and predate logging; 3 steps logged some rounds but never reached the required streak") is preferable to 14 individual waivers; the batch is represented as one waiver with `unit_step = "*"` and a `applies_to = [...]` list of slugs, or as a per-step waiver list generated at migration. This is a design choice for the migration; the model supports either.

`trivial` steps (if any; the current dogfooded plan has none in the Roadmap, though the word appears in the intake Classification enum) become `complete` steps with `[[waiver]]` entries carrying `reason = "review-skipped"` and `evidence_tier = "self-declared"`.

`optional-modules` (currently stuck at `in progress` because of the SE-4 / E4 issue) is marked `complete` with a waiver carrying `reason = "accepted-at-escalation"`, `evidence_tier = "record-backed"`, `unit_step = "optional-modules"`, `unit_increment = "inc2cii"`, and `evidence_ref = "L82"` (the `type:"escalation"` `human_decision:"decision"` record at workflow.jsonl line 82). This resolves SE-4 cleanly.

`skipped` remains as a distinct status (it answers "is this done?" with "no, abandoned," not "yes but exempt") and is not touched by the waiver model.

### Forward-looking historical exemptions (E10)

The W4 check has its own historical exemption: Q-1 through Q-41 are exempt because no decision records predate the mechanism's introduction. Under the unified model, this is not a separate waiver type. Instead, W4 fires only when at least one `type:"decision"` record exists in the JSONL log; the log's absence for pre-mechanism items is the grandfathering boundary. This is the same pattern the receipt-encoding explorations converged on; it does not need a new waiver entry per question.

---

## Area 5: Migration Path

### The dogfooded plan

The current `docs/plans/agent-scaffold.md` is the migration target. The migration is:

Step A: create the TOML skeleton. Parse the Roadmap table and the Open Questions id/status/ask list from the current Markdown (the two regions `src/plan.rs` already parses). Write `docs/plans/agent-scaffold.plan.toml` with `[[step]]` entries (slug, title from the Step Detail heading, status converted to the new vocabulary, order from table position) and `[[question]]` entries (id, status converted to the new vocabulary, ask from the one-liner, `folded_into` extracted from the `decided -> folded into \`slug\`` prefix). The conversion is scriptable.

Step B: extract prose payloads. For each step, copy the Step Detail body prose to `docs/plans/agent-scaffold.steps/<slug>.md` and set `detail_file = "steps/<slug>.md"` in the TOML entry. For each question, copy the body prose (the text after the one-line ask) to `docs/plans/agent-scaffold.questions/<id>.md` and set `body_file = "questions/<id>.md"` in the TOML entry. This step is the most labour-intensive: ~48 step bodies and ~44 question bodies.

Step C: add waivers. Enumerate the 14 `grandfathered` steps and convert them to `complete` status + `[[waiver]]` entries. Change any `trivial` steps similarly. Add the `optional-modules` `accepted-at-escalation` waiver.

Step D: add principles. Copy the 8 Project Principles from the plan prose into `[[principle]]` entries in the TOML.

Step E: run `agent-scaffold render` to generate `docs/plans/agent-scaffold.md` from the TOML. Compare with the current plan for accuracy. Commit both the TOML source and the generated Markdown.

Step F: delete the old hand-edited plan Markdown as the source. Going forward, the generated Markdown is committed but not hand-edited.

### The JSONL round log

The existing JSONL records are NOT rewritten (the log is append-only and permanent). New records from the migration forward include `step_id` and `increment_id` fields. W3 uses `step_id` when present and falls back to the `leading_slug` lexical strip for records that predate the migration. The fallback is kept as a compat shim until all increments are complete (at which point no new old-format records will be written and the shim can be removed in a future cleanup step).

The five orphan `task` values in the current JSONL (`consolidate-plan`, `metrics-fields`, `plan-fold`, `plan-maintenance`, `workflow-hardening`) reference steps that were renamed or removed. Under the migration, these are annotated with a `_orphan = true` comment-field in the TOML meta (not in the JSONL, which is append-only), so W3 knows to skip them without treating them as errors. A separate `validate --workflow` warning continues to flag unchecked orphans.

### The pack template

The pack's plan template (`pack/plan-template.md`) is replaced by `pack/plan-template.toml` plus `pack/plan-template.steps/.gitkeep` and `pack/plan-template.questions/.gitkeep` as starter directories. Scaffolded projects receive the TOML template as the plan source and a generated Markdown as the initial human-readable plan. The `plan-template.md` drift-guard tests migrate to testing the TOML schema constants against `pack/plan-template.toml`.

This is the highest-impact migration change: every scaffolded project that adopts the new pack inherits the TOML plan format. Projects on the old format continue to work with the existing Markdown parsers (backwards compat during transition). At the Principle 8 mandate level, the new format is the target and old-format projects are in a migration backlog state.

---

## Area 6: Staged, Reviewable Roadmap

Each increment is small enough to review independently, and the first two are the pilots that validate the structured-data approach before the rest of the plan structure migrates.

### Increment 1 (Pilot A): Decision receipt

Scope: add `options`, `recommendation`, `chosen`, and `folded_into` as structured fields to the `[[question]]` entry in the TOML schema; add the `type:"decision"` JSONL record type to `src/metrics.rs` (the `check_record` arm, `parse_decisions` projection, and drift-guard updates); add W4 to `src/workflow.rs`; extend `agent-scaffold status` with the `decisions` projection slot; document the convention that `decided` question prose does not re-enumerate option labels. The TOML question schema for this increment has only the new outcome fields; the rest of the TOML format is not yet built (the plan Markdown remains the hand-edited source). This isolates the receipt mechanism as a standalone testable unit.

Risk class: risky (it extends the enforcement backstop, the same risk classification `workflow-invariants`/W3 carried). Requires two consecutive clean rounds.

This increment is the decision-receipt substrate the receipt-encoding explorations converged on, placed in the TOML model rather than as a standalone JSONL-only feature.

### Increment 2 (Pilot B): Unified waiver model

Scope: add the `[[waiver]]` table to the TOML schema (the four fields: unit_step, unit_increment, reason, evidence_tier, evidence_ref, note); teach W3 to read waivers from the TOML instead of checking for `grandfathered`/`trivial` status values; remove `grandfathered` and `trivial` from `ROADMAP_STATUSES`; migrate the current dogfooded plan's 14 `grandfathered` Roadmap rows to `complete` + `[[waiver]]` entries; add the `optional-modules` `accepted-at-escalation` waiver and mark `optional-modules` `complete`; extend `agent-scaffold status` with a `waivers` projection slot. At this increment, the TOML is still a parallel artifact (the Markdown plan is still the main source for steps); W3 reads waivers from TOML when the file exists, and falls back to the status-based exemptions for plans without a TOML.

Risk class: risky (W3 behaviour changes; incorrect waiver handling would false-green or false-red on step completion). Requires two clean rounds.

### Increment 3: Step Roadmap in TOML

Scope: move the Roadmap from the plan Markdown to `[[step]]` entries in the TOML, including slug, title, status, order, `blocked_by`, and `detail_file`. Implement `agent-scaffold render` for the Roadmap table section. The generated Roadmap section in the Markdown plan is now derived. `src/plan.rs parse_roadmap` is replaced by a `parse_toml_steps` function; the old Markdown Roadmap parser is kept as a compat fallback for plans without a TOML. Add `step_id` and `increment_id` to forward-looking JSONL round records; W3 reads `step_id` when present.

This increment closes SE-10 (the `-inc<x>` lexical strip) for all future rounds and closes B3 (the stale status vocabulary in the plan prose) because the vocabulary is now rendered by the engine.

Risk class: risky (plan parsing changes). Requires two clean rounds.

### Increment 4: Open Questions queue in TOML

Scope: move the Open Questions queue from the plan Markdown to `[[question]]` entries in the TOML (the structured fields from Increment 1, plus `body_file` pointers). Extract the question body prose to `docs/plans/<task>.questions/<id>.md` sidecar files. Implement `agent-scaffold render` for the Open Questions section. `src/plan.rs parse_questions` is replaced by a `parse_toml_questions` function; the Markdown parser is kept as a compat fallback. W4 reads from TOML question entries directly (instead of parsing the Markdown for `QUEUE_FOLD_PREFIX` items).

This increment closes B4 (decision option-labels with no structured home) fully, because the question TOML entry now holds options/chosen as structured fields, and the body prose (the rationale) lives in its sidecar.

Risk class: risky (question parsing and W4 change). Requires two clean rounds.

### Increment 5: Full plan render

Scope: implement the remaining render sections (Principles, Documentation Protocol, Step Details, Success Criteria, plan meta). Extract the step detail prose to `docs/plans/<task>.steps/<slug>.md` sidecar files. The generated `docs/plans/<task>.md` is now the complete derived plan. The hand-edited plan Markdown ceases to be the source. Add the `validate --source` mode that checks the TOML schema, cross-references, and waiver integrity. Add the staleness check (content hash in meta, checked by `validate`). Update the pack template to ship a TOML plan.

This increment is the completion of the Principle 8 pivot. After this, the Markdown plan is a committed derived artifact and is never hand-edited.

Risk class: risky (wide scope: parser changes, new render engine, template change). Requires two clean rounds.

### Increment 6: Dogfooding migration of the agent-scaffold plan itself

Scope: convert `docs/plans/agent-scaffold.md` to `docs/plans/agent-scaffold.plan.toml` + sidecar prose files using the migration steps in Area 5. Run `agent-scaffold render` to regenerate the Markdown. Delete the old Markdown as a hand-edited source. Validate the round log migration (annotate orphan task values).

This is a housekeeping and verification increment: it validates that the migration procedure works on the largest and most complex plan in the repo. Risk class: low-risk (no code changes; validation of data migration). Requires one clean round.

---

## Trade-offs against the numbered Project Principles

**Principle 1 (clean long-term architecture).** The clean-slate model achieves the best score on this principle. A single structured source with projection to human-readable views is a cleaner architecture than the current mixed model (two structured regions in a Markdown file, a separate JSONL, and a prose ledger that duplicates the JSONL's facts). The cost is that the architecture requires a render engine that does not exist today; until Increment 5 ships, the architecture is partially built and the projection is incomplete.

**Principle 2 (minimal by default; do not complicate the core).** The clean-slate approach is a significant addition to the core: a TOML parser, a render engine, new validation modes, the waiver model, and the sidecar file conventions all add surface area. This is the direct cost of the clean-slate mandate. The staged roadmap mitigates it by shipping each addition as a small, independently reviewed increment. Increments 1 and 2 are the highest-value, lowest-scope pilots.

**Principle 3 (safe on existing projects; never change what a populated repo inherits without cause).** This principle is explicitly demoted for this design pass by the Principle 8 mandate. The pack template change (Increment 5) is a blast-radius change that every scaffolded project inherits. Projects on the old Markdown format must explicitly migrate or stay on the old format with the compat fallback parsers. The migration is a one-time cost; the clean-slate design is worth it.

**Principle 4 (create-if-absent, not clobber).** The render engine writes the Markdown plan file. If the file exists and has been hand-edited (which is wrong but possible), `render` overwrites it with the derived output. This is acceptable because the rendered output IS the correct content; hand edits to the derived file are a protocol violation, not legitimate content. `validate --source` catches stale derived files so the clobber is detected, not silent.

**Principle 5 (make illegal states unrepresentable).** The unified waiver model is a direct improvement: `grandfathered` and `trivial` as status values allow a step to be "done but exempt" without any explicit declaration of WHY; the waiver model requires a reason code, making the exemption's nature unambiguous. The question's `chosen in options` constraint is enforced at schema-check time, the same constraint the receipt's JSONL validation enforces. Structured fields for `folded_into` and `blocked_by` replace parametric-prefix string parsing (SE-11), making the relationship a data field rather than a string convention.

**Principle 6 (proven by dogfooding).** The migration of the dogfooded plan in Increment 6 proves the whole system works on a real, large plan. Increments 1 and 2 pilot the two most novel sub-systems (decision receipts and the waiver model) before the full render engine is built.

**Principle 7 (reproducibility, not tribal knowledge).** The render engine ensures the human-readable plan is always regenerable from the source. This is strictly better than today's model, where the human-readable plan IS the source and losing it is unrecoverable.

**Principle 8 (structured source, derived human-readable views).** This is the principle the whole design serves. The clean-slate model achieves full compliance: all machine-consumed data lives in the TOML or the JSONL, and the human-readable Markdown plan is fully derived. No other design achieves this.

**Principle 16 (one source of truth, derive the rest).** This is also directly served. The current smells resolved:
- B1 (ledger prose vs JSONL): the ledger round narrative stays prose (it is genuine transient content), and the JSONL is the structured round source; the acknowledged double-write is resolved by ceasing to treat the ledger narrative as a source for any count or convergence judgment. The LEDGER.template.md already says "when instrumentation is on, the orchestrator ALSO appends a `round` record"; under the clean-slate model, the JSONL is the sole source and the ledger narrative is read-only commentary.
- B2/A1/A2 (Status line, RESUME STATE re-stating Roadmap): the Status line is derived by the render engine from the step status distribution; it is never hand-written. The RESUME STATE stays prose (it is genuinely transient and non-derivable: the round number and streak are not stored anywhere structured when the ledger is the only in-flight record).
- B3 (status vocabulary triplicated): the code constants are the single source; the render engine writes them into the generated Markdown, not into a hand-maintained prose list.
- B4 (decision option-labels with no structured home): the question TOML entry holds them as the source; the JSONL receipt carries the timestamp.
- B6 (round-to-step link lexical): resolved by adding `step_id` / `increment_id` to round records.

The intentional overlap (TOML question options/chosen AND JSONL receipt options/chosen) is bounded and cross-checked: the W4 validation asserts they agree. This is the same pattern as the step-status-in-Roadmap vs convergence-in-JSONL overlap that W3 resolves: two homes, one check that they are consistent. It is not a Principle 16 violation because the two homes serve different purposes (TOML = live state, JSONL = durable audit log) and the overlap is explicit and enforced.

---

## Honest costs of the clean-slate purity

**The editing story regresses for prose.** Today, an agent or human opens one Markdown file and edits everything in place. In the clean-slate model, editing a step's design rationale requires opening a sidecar file; editing the step's status requires opening the TOML; reading the plan as a whole requires opening the generated Markdown (or running `agent-scaffold render` if it is stale). This is three files where today there is one. The sidecar model is cleaner for tooling but adds navigational overhead for humans.

**The render is a required tooling step.** Today, a change to the plan is one file write and one commit. In the clean-slate model, a change to the TOML or a sidecar file requires a `render` run before the commit, or the committed Markdown is stale. The `validate --source` staleness check catches this, but it adds a step. If `agent-scaffold` is not on PATH during a workflow run (e.g. a cold bootstrap on a new machine before `cargo install`), the human-readable plan cannot be regenerated. The compat fallback (the Markdown plan, committed as a derived artifact) mitigates this: an agent can still read the last committed render even without the tool.

**The migration is a one-time heavy lift.** Converting the dogfooded plan (48 step bodies, 44 question bodies, 14 waiver entries, 8 principle entries, the full Open Questions prose) is hours of manual work, even with a migration script doing the structured fields. This is the largest single-task cost of the clean-slate approach. For greenfield plans (projects scaffolded after Increment 5 ships), there is no migration: the TOML template is the starting point.

**The `render` engine is non-trivial to build.** The current `agent-scaffold` code has no file-write path for plan content: `run_status` reads and projects, `scaffold` create-if-absents asset files. A full render engine that writes the plan Markdown from TOML + sidecar prose is a new capability class. Estimating the scope: the render needs to assemble sections in order, interpolate prose from sidecars, format the Roadmap table, number the principles, and render the question list with structured fields inline. This is several hundred lines of new Rust, and it needs its own review loop. The five increments spread this cost, with Increments 3 and 4 building partial render (Roadmap section, Questions section) before Increment 5 completes the engine.

**The Markdown plan is no longer self-contained for authoring.** Today, a human can clone the repo, read `docs/plans/agent-scaffold.md`, and understand the entire plan state without any tooling. In the clean-slate model, the committed Markdown is readable but not editable as the source; authoring requires the TOML. For a first-time contributor without `agent-scaffold` installed, this is a discovery friction: they might edit the Markdown, not realise it is derived, and have their changes overwritten. Clear documentation in the generated Markdown's header ("This file is generated from agent-scaffold.plan.toml; edit the TOML source, not this file") mitigates but does not eliminate this risk.

**Two evidence tiers require documentation and discipline.** The `self-declared` / `record-backed` evidence tier distinction is correct and important, but it is not enforced by the schema alone: an orchestrator could write a waiver with `evidence_tier = "record-backed"` and a fake or wrong `evidence_ref`. W3 must validate that the `evidence_ref` line number exists and is a `type:"escalation"` `human_decision:"decision"` record (not just that the field is present). This is an additional validation step that was not needed in the old status-based model.

---

## What NOT to build (the YAGNI boundary)

Do NOT write a two-way sync from the generated Markdown back to the TOML. The render is one-way. If a diff-based "read Markdown edits and push them back to TOML" capability is ever wanted, it is a separate tool with its own design pass; do not conflate it with the render engine.

Do NOT try to structure the step detail prose or the question body prose. They are the human reasoning layer and they benefit from being free text. Forcing them into structured fields (e.g. a `## Trade-offs` section that the render engine extracts) would add parsing brittleness without adding machine utility. The boundary between the structured skeleton and the prose payload is correct and should be respected.

Do NOT unify the scaffold-level principles (in `pack/principles.toml`, parsed by `src/pack.rs`) with the plan-level principles (in `[[principle]]` entries in `plan.toml`). They serve different purposes: scaffold principles are a catalogue a project selects from at scaffold time; plan principles are the governance tenets for one specific plan. They share numbering by coincidence (both use P1, P2, etc.) and that collision is a naming problem (SE-1), not a structural one. Renaming with namespace prefixes (e.g. "Project P5" vs "Scaffold P16") is the right fix, not merging the two into one schema.

Do NOT build a GUI or interactive editor for the TOML. The editing story is plain-text TOML in any editor. If a TUI for the plan is wanted in the future, that is a new subcommand with its own design pass (analogous to the existing `tui.rs` for principle selection).

Do NOT build the full render engine in one increment. Increments 3, 4, and 5 split it into reviewable pieces. Shipping a partial render that only generates the Roadmap section (Increment 3) and then the Questions section (Increment 4) is the right staged approach; it produces reviewable output at each step.

Do NOT make the staleness check a hard failure by default. The `validate --source` staleness check should warn but not fail when the generated Markdown exists and is readable; a hard failure would block a workflow run if an agent forgot to re-render after a TOML edit. Warn loudly; fail hard only when explicitly asked (`--strict`).

Do NOT add `q_id` to `type:"escalation"` or `type:"intake"` records to make them serve as partial receipts (the receipt-encoding explorations fixed this: one purpose per record). An escalation that corresponds to a queue decision gets its own `type:"decision"` receipt alongside the escalation record.

Do NOT retroactively add `step_id` / `increment_id` to old JSONL records. The append-only log must not be rewritten. The `leading_slug` compat shim handles old records; only new records carry the structured fields.
