# structured-skeleton design B: the RICH-SCHEMA / structure-maximal lens

Explorer lens: push as much as sensibly possible into the structured `<task>.plan.toml`, so the plan becomes genuinely queryable and the single structured source of truth for the workflow's live state. Accept a larger render engine and a bit more authoring ceremony in exchange for maximal Principle-8 payoff: `status`, `next`, and `validate` read rich structure, not prose. Only genuinely free-form narrative stays in prose sidecars. This is one of three independent designs for Q-45 option B (clean-slate plan-as-data plus render); it is complete and self-standing and argues for its choices.

This document improves and concretises `docs/plans/structured-architecture.explorations/target-arch-B-cleanslate.md` rather than restating it. Where I diverge from that starting point, I say so and defend it.

## 0. The ground truth this design starts from (not the exploration doc's assumed starting point)

The exploration doc (`target-arch-B-cleanslate.md`) and the brief both describe a world where the decision receipt and the unified waiver model are still to be built as "Increment 1" and "Increment 2". That world is already shipped. The current repo state, read from the code and the live artifacts, is:

- The JSONL log `docs/metrics/workflow.jsonl` (113 records: 93 `round`, 16 `waiver`, 2 `escalation`, 1 `decision`, 1 `baseline`) already carries the unified `type:"waiver"` records, the `type:"decision"` receipt, and the `type:"baseline"` cutoff. `src/metrics.rs` parses all of them (`parse_rounds`, `parse_decisions`, `parse_baseline`, `parse_waivers`, `parse_escalations`).
- `src/workflow.rs` already implements W3 (convergence-OR-waiver), W4 (decided-item receipt cutoff), W5 (waiver integrity: step exists, increment ownership, record-backed evidence join, reason<->tier pairing), and the round-log consistency check. `leading_slug` still does the lexical `-inc<x>` strip (SE-10 / B6 open).
- `src/plan.rs` still parses the Roadmap table and the Open-Questions queue out of the Markdown plan. `ROADMAP_STATUSES` has already retired `trivial`/`grandfathered` (they are now waiver reason codes). The queue statuses are `open`/`exploring`/`superseded` plus the parametric `decided -> folded into <slug>`.
- `src/main.rs` has `scaffold`, `validate` (`--plan`, `--metrics`, `--workflow`), `status` (`--json`), and `checks`. There is no `render` and no `next` (the `state-queries` step is `not started`).
- The live `agent-scaffold.md` is 750 lines: a ~1-screen prose Status line (D1), 8 Project Principles, a Repository Layout prose index, a 45-item Open-Questions queue (Q-1..Q-45), a 51-row Roadmap, ~55 Step Detail blocks, and a 17-item enumerated Success Criteria list.

This changes the shape of the remaining work: the enforcement substrate (waivers, receipts, baselines, W3/W4/W5) already exists and lives in the JSONL. So the structured-skeleton initiative is no longer "add structure"; it is "decide where the structured skeleton lives, move the human-authored skeleton (Roadmap, queue, Status, Success Criteria, principles) into it, and define the boundary between the new TOML skeleton and the already-shipped JSONL event log." That boundary is the central design question of this pass, and the rich-schema lens forces a strong, specific answer to it (section 2).

## 1. The two-substrate model and the boundary (the load-bearing decision)

The workflow's data splits cleanly into two KINDS, and the rich-schema lens keeps them in two substrates with different rules, not one:

- STATE / STRUCTURE (mutable, current, structural): what the steps are, their status and order and dependencies, what increments each step has, which steps are exempt and why, the queue and its outcomes, the principles, the success criteria, the cross-references. This is the SKELETON. It changes as the work progresses (a step goes `next` -> `in progress` -> `complete`; a waiver is added and could later be removed when real rounds land). It benefits from being editable, queryable, and validated as a whole.
- EVENTS / EVIDENCE (append-only, historical, temporal): what happened and when. A review round with its outcome and streak. A human decision at a gate with the options presented and the choice taken (the receipt). A cap escalation with the human's verdict. An intake classification. A dismissal recheck. These are timestamped facts that must never be rewritten, because they are the audit trail and the calibration corpus.

The rich-schema position: the SKELETON goes into `docs/plans/<task>.plan.toml`; the EVENT LOG stays in the append-only `docs/metrics/workflow.jsonl`. This is the boundary. It is defensible on first principles (state and history are different data with different mutation rules), and it lets each substrate be maximal in its own domain without either duplicating the other.

Applying that boundary to the five existing JSONL record types precisely:

| Record kind | Substrate under this design | Why |
| --- | --- | --- |
| `round` | JSONL event (unchanged) | A timestamped convergence fact; the calibration corpus. Immutable. W3 joins rounds to steps by a STRUCTURED `step_id`/`increment_id` (section 3), not the lexical strip. |
| `escalation` | JSONL event (unchanged) | A timestamped human accept at the cap. It is the durable evidence a record-backed waiver points at. Immutable. |
| `intake`, `dismissal_recheck` | JSONL event (unchanged) | Timestamped process facts; calibration only. |
| `decision` (receipt) | JSONL event (KEPT) plus a TOML cross-reference (section 4) | The receipt is a timestamped attestation that the human-input contract was met (options presented + choice). That is a temporal fact and stays a durable event. The TOML queue item holds the live `chosen`/`options`/`folded_into` and a `receipt` pointer; W4 cross-checks the two agree. Two homes, one check (the pattern the repo already blesses for step-status-in-Roadmap vs convergence-in-JSONL). |
| `waiver` | MOVES to TOML `[[waiver]]` (section 3) | A waiver is a standing DECLARATION of exemption, not an event. It is live state co-located with the step it exempts. Moving it makes "which complete steps are exempt and why" a local, queryable fact and lets the render show the exemption inline in the Roadmap. A record-backed waiver's `evidence` still references a JSONL `escalation` event. |
| `baseline` (cutoff) | MOVES to TOML `[meta].w4_baseline` | The baseline is one-time CONFIG (a declared cutoff), not an event. It belongs with the plan meta, not in the temporal log. |

Two of these (waiver, baseline) MOVE out of the JSONL. This is the rich-schema lens's strongest and most contestable call, so I defend it head-on and flag the alternative in section 9.

Why move waivers to the TOML (the rich-schema argument):

- Queryability. Under the current design, answering "why is `optional-modules` allowed to be complete without a converged streak?" means scanning 113 JSONL lines for a waiver whose `step` matches, then a second scan for the `escalation` its `evidence` points at. Under this design it is `plan.toml` -> the `[[step]]` for `optional-modules` -> its `[[waiver]]` block, with the reason, tier, and evidence pointer right there. `status --json` can list every exempt step and its reason in one pass over the parsed skeleton.
- Revisability. A waiver is not a fact-that-happened; it is a standing claim that can legitimately be withdrawn (a `predates-logging` step that later gets real review rounds should drop its waiver). An append-only log cannot express "this exemption no longer applies" except by convention. TOML state can.
- Co-location with the thing it modifies. W3's question is "is this complete step's convergence requirement satisfied or waived?" The waiver is an attribute of the step. Putting it on the step (Principle 5, make illegal states unrepresentable: a waiver cannot dangle to a nonexistent step because it is nested under one) is cleaner than a separate log line joined by a slug string.
- The audit-immutability argument is weaker for waivers than for events. The reason the pilots put waivers in the append-only log was defensiveness (a malformed waiver must never silently grant an exemption; `parse_waivers` drops malformed records so W3 only sees well-formed ones). That property is preserved verbatim under TOML: the strict `validate --source` reports a malformed waiver, and the best-effort skeleton projection W3 reads drops it, so a malformed waiver still cannot silently exempt. The substrate does not change the strict/best-effort discipline; it only changes where the well-formed declaration lives.

Why KEEP the decision receipt as a JSONL event (not move it too): the receipt's whole value is that it is a timestamped attestation the presentation happened, written at the moment of the decision. That IS temporal evidence, exactly the kind the event log exists for. The live `chosen`/`options` belong in the queryable TOML queue; the durable "this was presented at time T" belongs in the log. W4 cross-checks. Collapsing the receipt into the TOML would lose the timestamped-attestation property that motivated it in the first place (Q-42).

Net effect on the JSONL after migration: it holds `round`, `escalation`, `decision`, `intake`, `dismissal_recheck` (pure events). Its 16 historical `waiver` lines and 1 `baseline` line are TRANSLATED into the TOML during migration and left in place in the log as valid-but-unconsulted history (the append-only rule is honored: no line is rewritten; `validate_log` still accepts them as well-formed; W3/W4/W5 simply stop reading `waiver`/`baseline` from the log and read them from the TOML). Section 6 details this.

## 2. The `<task>.plan.toml` schema (maximal structure)

The file is TOML, matching the project's existing structured-data format (`pack/principles.toml`, `pack/pack.toml`, `.agents/checks.toml`) and the `toml` crate already in the toolchain. Everything the tooling produces, consumes, or could sensibly query is a typed field. Only genuinely free-form narrative is a `*_ref` pointer to a Markdown sidecar.

### 2.1 Meta

```toml
[meta]
title = "agent-scaffold plan"
# The Status line is DERIVED by render; there is no hand-authored status field.
# Optional free-form framing prose (the two paragraphs under today's title) as a sidecar.
intro_ref = "agent-scaffold.intro.md"
motivations_ref = "agent-scaffold.motivations.md"
# Project-specific prose sections that are not part of the workflow skeleton.
repository_layout_ref = "agent-scaffold.repo-layout.md"
# The W4 historical-exemption cutoff (was a type:"baseline" JSONL record).
w4_baseline = "Q-44"
# The render self-check: render writes this; validate re-renders and compares (section 5).
render_sha256 = "b1a4...e6f1"
# Verification convention prose the render prints verbatim into the generated Status footer.
verification = "cargo clippy --all-targets -- -D warnings; nix fmt; ASCII-clean before each commit."
```

### 2.2 Principles (structured, numbered by position)

```toml
[[principle]]
n = 1
name = "clean long-term architecture"
text = "Optimise for the cleanest long-term shape; this is a pre-adoption tool, so prefer the best architecture over backwards-compatibility."

[[principle]]
n = 8
name = "structured source, derived views"
text = "A structured file or small set of files is the single source for the workflow's data; agent-scaffold projects it into the human-readable plan, ledger, and status."
```

`n` is the render's list number; `name` and `text` are one-liners the render numbers into the Project Principles section. This gives project principles a structured home distinct from `pack/principles.toml` (the scaffold catalogue), which resolves the SE-1 namespace collision at the data level: code can cite "Project P8" meaning `principle.n == 8` in `plan.toml`, unambiguously distinct from a scaffold AGENTS.md principle. (SE-1 stays a citation-hygiene fix in code comments; the schema makes the two sets structurally distinct so the fix is mechanical.)

### 2.3 Steps and increments (the Roadmap, with structured step<->increment links)

Each step is a `[[step]]` table. Increments are nested `[[step.increment]]` tables, which is the structural replacement for the lexical `-inc<x>` strip (audit B6, SE-10, E6). A step's status is from the cleaned closed set (`ROADMAP_STATUSES`): `not-started`, `in-progress`, `complete`, `next`, `optional`, `deferred`, `skipped`, plus the structured `blocked_by` field replacing the parametric `blocked on <slug>` string.

```toml
[[step]]
slug = "round-log-core"
title = "Unify round data on the JSONL round log"
status = "complete"
order = 41
detail_ref = "steps/round-log-core.md"
# Structured decision provenance: which queue items folded into this step.
folds = ["Q-34"]

  [[step.increment]]
  id = "round-log-core-incA"
  risk_class = "low_risk"

  [[step.increment]]
  id = "round-log-core-incB"
  risk_class = "risky"

[[step]]
slug = "optional-modules"
title = "Optional modules"
status = "complete"
order = 30
detail_ref = "steps/optional-modules.md"
folds = ["Q-25", "Q-26", "Q-33", "Q-36", "Q-38", "Q-39", "Q-41"]

  [[step.increment]]
  id = "optional-modules-inc2cii"
  risk_class = "risky"

  # A waiver is nested on the step it exempts (section 3).
  [[step.waiver]]
  id = "W-15"
  unit = "increment"
  increment = "optional-modules-inc2cii"
  reason = "accepted-at-escalation"
  evidence_tier = "record-backed"
  evidence = "optional-modules-inc2cii"   # joins to a JSONL escalation record's task
  note = "Human accepted at streak 1 at the 5-round cap; write-escape class closed and independently confirmed."

[[step]]
slug = "structured-skeleton"
title = "The clean-slate plan-as-data plus render"
status = "next"
order = 45
detail_ref = "steps/structured-skeleton.md"
folds = ["Q-45"]
blocked_by = []   # structured, replaces `blocked on <slug>`
```

Design points:

- `order` is an explicit integer, so the render sequences the Roadmap table by it and reordering is a field edit, not a line move. Steps are still identified by stable `slug` (cross-references survive reordering).
- `[[step.increment]]` gives every increment a structured `id` and its own `risk_class`. W3 joins a JSONL `round` record to an increment by exact `increment_id`/`task` equality against a DECLARED increment id, so the lexical strip (`leading_slug`) is retired: there is no more `-inc<alnum>` ambiguity (SE-10 / T3). The declared `risk_class` here is the source; a round record still carries its own `risk_class` and W3's risk-class-consistency guard cross-checks them (a round whose `risk_class` disagrees with its increment's declared class is a data-integrity fault, never waived).
- `folds` makes the decided-queue-item -> step link a structured field (it appears on both ends: the step lists what folded into it, the question names `folded_into`). This is the queryable form of the audit's B4/SE-11 (option-labels and fold targets had no structured home).
- Dependencies are `blocked_by = ["<slug>", ...]`, a structured list. `status` no longer encodes the blocker (SE-11 parametric-status removed).

### 2.4 Waivers (moved from JSONL, nested on their step)

A `[[step.waiver]]` (or a top-level `[[waiver]]` that names its `step`, equivalent; nesting is the recommended authoring form) carries exactly the fields W3/W5 read today, with a stable `id`:

```toml
  [[step.waiver]]
  id = "W-1"
  unit = "step"
  reason = "predates-logging"
  evidence_tier = "self-declared"
  note = "Predates the round-logging regime; no retrospective review evidence possible."
```

Fields: `id` (stable, e.g. `W-1`), `unit` (`step` | `increment`), `increment` (present exactly when `unit == increment`), `reason` (`predates-logging` | `review-skipped` | `accepted-at-escalation`), `evidence_tier` (`self-declared` | `record-backed`), `evidence` (present exactly when `evidence_tier == record-backed`; names the backing JSONL `escalation` record by its `task`), `note` (optional short prose). The `reason <-> evidence_tier` pairing, the `increment`-presence rule, and the `evidence`-presence rule are the same schema constraints `check_record` enforces today, moved into `validate --source`. The two evidence tiers do NOT launder: `validate --source` reports a `self-declared` waiver dressed as `record-backed`, and W5's record-backed join (the `evidence` must point at a real `type:"escalation" human_decision:"decision"` record scoped to the waived unit) is unchanged; it now reads the waiver from the TOML and the escalation from the JSONL.

### 2.5 Open-Questions queue (rich outcome fields, prose in a sidecar)

Each queue item is a `[[question]]`. The id/status/ask/options/chosen/recommendation/folded_into are STRUCTURED; the long rationale body is a sidecar. This closes SE-15 (the multi-KB `ask` blob) and B4 (option-labels with no structured home): `ask` is a one-liner; the thousands of words that today live inline move to `questions/<id>.md`.

```toml
[[question]]
id = "Q-45"
status = "decided"
ask = "How far to structure the human-authored skeleton (Roadmap, queue, Status, RESUME STATE)?"
options = ["A incremental", "C event-sourced", "B clean-slate"]
recommendation = "C event-sourced"
chosen = "B clean-slate"
folded_into = "structured-skeleton"
receipt = "Q-45"                 # joins to the JSONL type:"decision" q_id; W4 cross-checks
body_ref = "questions/Q-45.md"

[[question]]
id = "Q-44"
status = "open"
ask = "The structured-data plus human-projection architecture and the cleanup of accumulated sharp edges."
body_ref = "questions/Q-44.md"

[[question]]
id = "Q-43"
status = "superseded"
superseded_by = "Q-44"           # structured, replaces the prose "(superseded)" with a link
ask = "How the chosen Full decision receipt is encoded and where it lives."
body_ref = "questions/Q-43.md"
```

Design points:

- `status` is a clean enum: `open` | `exploring` | `decided` | `superseded`. The parametric `decided -> folded into <slug>` string (SE-11) becomes `status = "decided"` plus a separate `folded_into` field. The `superseded` case gains an optional `superseded_by` link (SE-12: superseded items point at their successor instead of silently telling readers to ignore them).
- `options`, `recommendation`, `chosen` are populated only when `status == decided`. `validate --source` enforces `chosen` is a member of `options` (the same cross-field constraint the `decision` record enforces). This is the QUERYABLE home for the option-labels; the sidecar carries only rationale and never re-enumerates the labels (the convention the receipt-encoding pass established, now enforceable because the labels have a structured home to point at).
- `receipt` is the cross-reference to the JSONL `type:"decision"` record's `q_id`. W4 reads decided `[[question]]` entries from the TOML and requires a matching receipt in the log for every decided item whose id index is strictly after `[meta].w4_baseline`, cross-checking that the receipt's `options`/`chosen` agree with the TOML's. Two homes (TOML live state, JSONL durable attestation), one check.

### 2.6 Success Criteria (structured list, not a prose blob)

The live plan's Success Criteria is already an enumerated 17-item list. The rich-schema lens structures it so it is queryable and each criterion can link to the step(s) that satisfy it:

```toml
[[success_criterion]]
id = "SC-1"
text = "One command drops the minimal core into an empty dir and an existing repo; writes are off unless confirmed."
satisfied_by = ["core-assets", "file-dropper", "idempotency-safety"]

[[success_criterion]]
id = "SC-14"
text = "The scaffolded workflow offers a human-invokable review entry mode."
satisfied_by = ["review-mode"]
```

`satisfied_by` is a structured cross-reference (each slug must resolve to a `[[step]]`). This lets `status` report criterion coverage ("14 of 17 success criteria have at least one complete satisfying step") and lets the render show, per criterion, whether its satisfying steps are done. The exploration doc kept Success Criteria as a narrative meta string; the rich lens improves on that because the live data is already a checklist and structuring it pays off in queryability (a criterion whose `satisfied_by` steps are all `complete` is demonstrably met).

### 2.7 What stays prose (the sidecars, section 4)

Only genuinely free-form narrative: step detail bodies (`steps/<slug>.md`), question rationale bodies (`questions/<id>.md`), the intro/motivations/repository-layout project prose (referenced from `[meta]`), and the ledger's round narratives and RESUME STATE (which stay in the ledger, unchanged, section 4.2). None of these is machine-parsed.

## 3. The prose sidecars and the no-clobber guarantee

### 3.1 Layout

```
docs/plans/
  agent-scaffold.plan.toml            # the structured skeleton (the source)
  agent-scaffold.md                   # GENERATED, committed, never hand-edited
  agent-scaffold.intro.md             # meta.intro_ref
  agent-scaffold.motivations.md       # meta.motivations_ref
  agent-scaffold.repo-layout.md       # meta.repository_layout_ref
  agent-scaffold.steps/<slug>.md      # one per step, detail_ref
  agent-scaffold.questions/<id>.md    # one per question, body_ref
  agent-scaffold.ledger.md            # unchanged (prose narrative + RESUME STATE)
docs/metrics/
  workflow.jsonl                      # the append-only event log (unchanged)
```

A sidecar is plain Markdown, no frontmatter (the structured metadata is the TOML entry's job; a sidecar is pure prose). Example `agent-scaffold.steps/optional-modules.md`:

```markdown
Optional modules: a `--module` machinery layer, a deterministic-checks module, and a guidance-only isolation module.

All three increments are built. Increment 1 added the `--module` machinery; increment 2 the whole checks module (the `{{modules}}` slot, `.agents/checks.toml`, the seeded ast-grep config, the `checks-reviewer` role, and worktree-isolated `agent-scaffold checks`); increment 3 the guidance-only isolation module.

The umbrella step is `complete`, unstuck by an `accepted-at-escalation` record-backed waiver on increment `optional-modules-inc2cii` (see the waiver on this step), whose evidence is the escalation record in the round log. Resolves Q-40 / SE-4.
```

Note what the sidecar does NOT contain: the status, the increment ids, the waiver fields, the fold list. Those are the TOML entry's structured fields. The sidecar is the reasoning narrative only. This is the boundary the brief calls "keep only genuinely free-form narrative in prose sidecars", enforced by construction (the render pulls structured facts from the TOML and only the body prose from the sidecar).

### 3.2 The no-round-trip / no-clobber guarantee

The tool NEVER parses a sidecar or the generated `agent-scaffold.md` back into structure. Reads go one direction only: TOML + JSONL + sidecars -> generated Markdown. Enforcement, `status`, and `next` read the TOML and the JSONL, never the generated Markdown or the sidecars (except to inline sidecar prose during a render). So there is no round-trip and nothing to clobber:

- An author editing a sidecar changes prose that the render inlines verbatim; the tool never interprets it, so it can never be "misparsed" or overwritten by a structure-to-prose rewrite.
- The generated `agent-scaffold.md` is the ONLY file the render writes, and it is never a source. Hand edits to it are a protocol violation caught by the render self-check (section 5.3), not silently honored.
- Sidecars and the generated file are separate files, so a render rewriting `agent-scaffold.md` never touches a sidecar.

RESUME STATE and the round narrative stay in the ledger as prose (section 4.2); the tool does not parse them either.

## 4. The `agent-scaffold render` engine

### 4.1 Pipeline and CLI

New subcommand: `agent-scaffold render [--plan <task>.plan.toml] [--output <path>] [--check]`.

Input: the `.plan.toml` skeleton, the sidecars it references, and (for derived counts only) the JSONL log. Output: the committed `<task>.md`. The render is strict: a missing referenced sidecar, a schema violation, or an unresolved cross-reference exits non-zero and writes nothing (a broken source must not produce a partial plan agents then read as authoritative). `--check` renders to memory and compares against the committed file without writing (the CI/pre-commit form, section 5.3).

### 4.2 The projection (what the generated `agent-scaffold.md` looks like)

The generated Markdown reproduces today's plan structure section-for-section, so a reader sees the same document; the difference is that every structured fact is derived. Sketch:

```markdown
<!-- GENERATED FILE. Do not edit. Source: agent-scaffold.plan.toml + sidecars. Regenerate with `agent-scaffold render`. Edits are overwritten and caught by the render self-check. -->

# agent-scaffold plan

Status: in progress; 41 of 51 steps complete, 1 next, 2 not started, 4 optional/deferred, 2 skipped; 1 open question (Q-44), 44 decided/superseded; 16 waivers (14 predates-logging/review-skipped, 2 accepted-at-escalation). See the Roadmap for per-step status.

<intro prose, inlined from agent-scaffold.intro.md>

## Motivations

<inlined from agent-scaffold.motivations.md>

## Project Principles

1. clean long-term architecture. Optimise for the cleanest long-term shape ...
8. structured source, derived views. A structured file ...

## Documentation Protocol

<fixed template fragment>. Roadmap statuses: not-started, in-progress, complete, skipped, next, optional, deferred, blocked_by <slug>. Queue statuses: open, exploring, decided, superseded. <the vocabulary lists are generated from the code constants, so they cannot go stale (resolves B3)>.

## Repository Layout and Current Architecture

<inlined from agent-scaffold.repo-layout.md>

## Open Questions, Decisions, Issues and Blockers

- `Q-44` (open) The structured-data plus human-projection architecture and the cleanup of sharp edges.
  <body inlined from questions/Q-44.md>
- `Q-45` (decided -> folded into `structured-skeleton`) How far to structure the skeleton. Options presented: A incremental, C event-sourced, B clean-slate. Chosen: B clean-slate (recommended: C). Receipt: Q-45.
  <body inlined from questions/Q-45.md>

## Roadmap

| Step | Status | Notes |
| ---- | ------ | ----- |
| `round-log-core` | complete | |
| `optional-modules` | complete | waived: increment optional-modules-inc2cii accepted-at-escalation (record-backed) |
| `structured-skeleton` | next | |

## Step Details

### `optional-modules`: Optional modules

<body inlined from steps/optional-modules.md>

## Success Criteria

- SC-1: One command drops the minimal core ... (satisfied by core-assets, file-dropper, idempotency-safety: all complete).
- SC-14: The workflow offers a human-invokable review entry mode (satisfied by review-mode: complete).
```

Every derived element is labelled above: the Status line (from the step-status distribution + queue + waiver counts, resolving D1/A1/SE-13), the numbered principles, the generated status vocabulary (resolving B3), the queue items with their options/chosen inline (resolving B4), the Roadmap Notes column showing waivers inline, and the Success Criteria coverage annotations. The Status line is NEVER hand-authored, so it cannot drift.

The ledger is a separate artifact and is NOT rendered from the TOML: its round narratives are genuine transient prose commentary and its RESUME STATE is transient in-flight state. They stay hand-authored in `agent-scaffold.ledger.md`. What changes is that `status --resume` and `next` derive the pointer half (active step, open questions, current streak) from the TOML + JSONL, so RESUME STATE shrinks to genuinely non-derivable in-flight context (a pending dismissal recheck, a running debate). This is the `state-queries` de-duplication discipline; the render engine does not touch the ledger.

### 4.3 What reads the new source

Nothing reads the generated Markdown except humans. All tooling reads the TOML skeleton and the JSONL event log directly:

- `validate --source <task>.plan.toml`: schema check (types, enums, `chosen in options`, waiver field-presence and reason<->tier pairing), plus the cross-reference invariants (every `folds`/`folded_into`/`blocked_by`/`satisfied_by`/`superseded_by` target resolves; every step slug is unique; every increment id is unique). This replaces `validate --plan` (which parsed the Markdown). The strict/best-effort split is preserved: `validate --source` REPORTS malformed entries; the projection W3/W4/W5 read DROPS them.
- `validate --workflow`: W3 (reads steps + increments + waivers from the TOML, rounds from the JSONL, joins by structured increment id), W4 (reads decided questions + `w4_baseline` from the TOML, receipts from the JSONL), W5 (reads waivers from the TOML, escalations from the JSONL). The round-log consistency check reads the JSONL alone. The pause.md catch is preserved verbatim: a `complete` step with no matching rounds and no covering `[[step.waiver]]` still fails.
- `status` / `status --json`: reads the TOML for the rich skeleton projection (steps by status, queue by status, waivers with reason/tier, decisions, success-criterion coverage) and the JSONL for round counts and the current streak. The `--json` `Projection` gains `waivers`, `decisions`, and `success_criteria` slots alongside the existing `steps`/`open_questions`.
- `next`: reads the TOML (Roadmap order + status + `blocked_by` + open questions) and the JSONL (streak) to compute the single authoritative next action, reusing the W3/W4 transition logic. This is the `state-queries` payload, now reading rich structure directly.

Because enforcement and queries read the TOML, not the rendered Markdown, the generated file being stale or hand-edited never affects correctness; it only misleads a human reader, which the self-check catches.

### 4.4 The render self-check (the do-not-edit guard)

Two layers, defence in depth:

1. A banner comment at the top of every generated file (shown above): a human or agent opening it is told it is generated and that edits are overwritten.
2. A content hash. `render` computes the SHA-256 of the generated bytes and writes it to `[meta].render_sha256`. `validate` (and a manifest/golden test, and the optional pre-commit hook) recompute a fresh render and compare against the committed `<task>.md` and the stored hash. A hand-edit to the generated file, or a stale render (TOML/sidecar edited without re-rendering), makes the committed file != the fresh render and is caught.

Severity policy: `validate` WARNS on a stale/edited generated file by default (a forgotten re-render should not block a workflow run mid-task), and FAILS hard under `validate --strict` and in CI (a golden test `generated_plan_matches_render` in the manifest suite, analogous to the existing byte-identical self-scaffold test). The existing `--with-precommit-hook` gains a `render --check` step so a commit that would ship a stale plan is caught before it lands. This is the render-before-commit enforcement point: the hook, not the human's memory.

## 5. The unified waiver model under the TOML skeleton (preserving W3/W4/W5)

The waiver model is already shipped and correct; this design MOVES its declaration home from the JSONL to the TOML (section 1) and leaves its semantics untouched. Concretely:

- W3 (`w3_problems`): unchanged logic. It reads `[[step]]` (status), `[[step.increment]]` (declared increments + risk_class), and `[[step.waiver]]` from the TOML, and `round` records from the JSONL. It joins rounds to increments by structured id (no `leading_slug`). Pass 1: a `complete` step with a converged increment passes. Pass 2: a `complete` step with no rounds passes iff a `unit=step` waiver covers it, else the pause.md catch fires. Increment shortfall: passes iff a `unit=increment` waiver names that increment. The risk-class-consistency guard stays live and is never waived.
- W4 (`w4_problems`): reads decided `[[question]]` and `[meta].w4_baseline` from the TOML and `decision` receipts from the JSONL. A decided item strictly after the baseline requires a matching receipt; the boundary is the declared `w4_baseline`, not derived (the circular-boundary bug the pilot loop caught stays fixed because the cutoff is independent declared data). The rich addition: W4 also cross-checks that a present receipt's `options`/`chosen` agree with the TOML question's `options`/`chosen` (the two-homes-one-check the boundary section promised).
- W5 (`w5_problems`): reads `[[step.waiver]]` from the TOML and `escalation` records from the JSONL. Every waiver names a real step (guaranteed by nesting; a top-level waiver's `step` is cross-checked), an increment waiver's step owns its increment, a record-backed waiver's `evidence` joins to a real decision-scoped escalation, and the reason<->tier pairing is consistent. Unchanged; only the waiver source moved.

Nothing in the enforcement guarantees regresses. The two evidence tiers stay distinct and un-launderable; exemptions stay declared and visible (now MORE visible, since they render inline in the Roadmap Notes column); the escalation scope (decision-only, one increment, streak-shortfall-only) is preserved.

## 6. Migration path for this repo's dogfooded artifacts

The migration is a staged cutover with a dual-run reconciliation, not a blind big-bang. Backwards-compat is not required (pre-adoption; Principle 8 > compat), but the migration must not lose data or break enforcement mid-flight, so each stage keeps `validate --workflow` green.

- M1. Generate the skeleton. Parse the current `agent-scaffold.md` Roadmap (51 rows) and queue (Q-1..Q-45) with the existing `src/plan.rs` parsers into `[[step]]` and `[[question]]` entries: slug/status/order from the table, id/status/ask from the queue, `folded_into` from the `decided -> folded into <slug>` prefix, `superseded_by` from context. Convert statuses to the cleaned enum (`not started` -> `not-started`, etc.). This is scriptable and lossless for the structured fields.
- M2. Reconstruct increments. Enumerate the distinct `task` values in the JSONL (via `leading_slug` one last time) to populate each step's `[[step.increment]]` ids and their `risk_class`. The 5 orphan `task` values (`consolidate-plan`, `metrics-fields`, `plan-fold`, `plan-maintenance`, `workflow-hardening`) that match no live step are recorded under `[meta].orphan_increments` so W3 knows to skip them (they are historical rounds for renamed/removed steps); a `validate --workflow` warning still flags any NEW orphan.
- M3. Re-home the waivers and baseline. Translate the 16 JSONL `waiver` records into `[[step.waiver]]` entries (11 `predates-logging` step waivers, 3 `review-skipped` increment waivers on `convergence-accounting`/`pack-rebuild-tracking`/`user-prompts-dir`, the `optional-modules-inc2cii` and `waiver-model` `accepted-at-escalation` record-backed waivers) with stable ids `W-1..W-16`, and the 1 `baseline` record into `[meta].w4_baseline = "Q-44"`. The JSONL keeps these 17 lines in place (append-only honored; they stay `validate_log`-valid); W3/W4/W5 stop reading them from the log after this stage. This is the only place a JSONL record type changes consumer, and it is a translation, not a rewrite.
- M4. Extract prose. Copy each Step Detail body to `steps/<slug>.md` and set `detail_ref`; copy each question body to `questions/<id>.md` and set `body_ref`; move the intro/Motivations/Repository-Layout prose to their sidecars. This is the heavy manual lift (~55 step bodies, 45 question bodies), because the structured extractor can split at headings but a human must confirm no structured fact was left stranded in prose. Populate `[[principle]]` (8) and `[[success_criterion]]` (17).
- M5. Render and reconcile. Run `agent-scaffold render`, diff the generated `agent-scaffold.md` against the current hand-written one, and reconcile every difference (a difference is either a bug in the migration or a drift the current plan had, e.g. the stale Status line; both get fixed at the source). Commit the TOML, the sidecars, and the generated Markdown together.
- M6. Cut over. Delete the notion of the hand-edited plan Markdown. Going forward the Markdown is generated-only; the pre-commit hook runs `render --check`. Repoint the pack template (section 7).

Throughout M1..M4 the live `agent-scaffold.md` remains the enforcement source (the code still reads it) so the repo stays green; the cutover to TOML-as-source happens per-consumer as each increment lands (section 8), and M5/M6 are the final flip. The 113 JSONL records' `round`/`decision`/`escalation`/`intake` semantics carry over untouched; only `waiver`/`baseline` change consumer.

## 7. The scaffold story (keeping the default output coherent)

`pack/plan-template.md` is replaced by a template TOML plus starter sidecar directories:

- `pack/plan-template.plan.toml`: a minimal skeleton (a `[meta]` with a `title` placeholder and no `w4_baseline`, one `[[principle]]` seeded from the scaffold's AGENTS.md, one `[[step]]` placeholder, an empty queue, a `[[success_criterion]]` placeholder).
- `pack/plan-template.steps/.gitkeep`, `pack/plan-template.questions/.gitkeep`: starter directories.
- The scaffold drops the template TOML and runs an initial `render` to produce the human-readable `<task>.md`, so a fresh project starts with both the source and a readable view.

A fresh scaffold has NO pre-existing decided items, so it declares no `w4_baseline` and every decision it makes under the mechanism correctly requires a receipt (the exemption is always declared, never assumed). The drift-guard test `plan_template_documents_every_accepted_status` migrates to asserting the TOML template's documented vocabulary matches the code constants; better, since the render now GENERATES the vocabulary into the human view, the live-plan drift (B3) is gone entirely and the guard shrinks to the template only.

The scaffolded default stays coherent: a fresh run drops a usable, internally consistent workflow whose plan is a TOML source plus a generated view, and the AGENTS.md/ledger-template prose is updated to say the plan source is the TOML and the Markdown is generated-do-not-edit.

## 8. Staged, reviewable roadmap

Each increment is independently shippable and reviewable under the role-separated loop, risk-classified, dependency-ordered, earliest-lowest-regret. The two ADDITIVE capabilities (schema+validate, then render) come first because they change nothing the enforcement reads; the two RISKY source-swaps come next; the low-risk full cutover last.

- Inc 1 (low-risk): the `plan.toml` schema, the strict TOML parser, and `validate --source` (schema + internal cross-references only). A `.plan.toml` is optional and unconsulted by W3/W4/W5; the live plan stays Markdown-sourced. Delivers the data model and its validator. Acceptance: `validate --source` passes on a fixture skeleton and reports each injected schema/cross-ref violation; the live repo is unaffected.
- Inc 2 (low-risk): the `render` engine (all sections) plus the render self-check (banner, `render_sha256`, `render --check`, the golden test). Additive: it only GENERATES a view; nothing depends on it yet. Tested against a fixture `.plan.toml` + fixture sidecars. Acceptance: render is deterministic and byte-stable; a hand-edited generated file fails `render --check` and the golden test; render exits non-zero on a missing sidecar or a broken cross-reference.
- Inc 3 (risky): swap the enforcement source for STEPS. W3 and W5 read `[[step]]`/`[[step.increment]]`/`[[step.waiver]]` from a `.plan.toml` when present (falling back to Markdown + JSONL waivers otherwise), joining rounds by structured increment id (retire `leading_slug`). Migrate this repo's steps + waivers (M1..M3 for steps/waivers) into the TOML. Acceptance: `validate --workflow` green reading the TOML; the pause.md catch, per-increment convergence, the two waiver tiers, and the risk-class-consistency guard all preserved (the existing W3/W5 test suite passes against TOML fixtures); SE-10/B6 closed (a `foo-incidental`-style slug no longer mis-routes). Risky because it rewrites the enforcement backstop's source.
- Inc 4 (risky): swap the enforcement source for the QUEUE. W4 reads decided `[[question]]` + `[meta].w4_baseline` from the TOML (cross-checking receipts' options/chosen), and `status --json` gains the `waivers`/`decisions`/`success_criteria` slots. Migrate the queue + baseline into the TOML (M1 queue, M3 baseline). Acceptance: `validate --workflow` W4 green reading the TOML; the circular-boundary fix preserved; SE-11/SE-15/B4 closed (structured status/folded_into, one-line ask, structured option labels). Risky because it changes W4's source and the receipt cross-check.
- Inc 5 (low-risk): the full dogfood cutover. Extract all prose sidecars (M4), render and reconcile the live plan against its current Markdown (M5), cut over to generated-only (M6), add `next` and `status --resume` reading the TOML, and swap the pack template (section 7). Acceptance: this repo's own `agent-scaffold.md` is generated and byte-matches the reconciled plan; `validate --workflow` green; a fresh `agent-scaffold` scaffold drops a coherent TOML plan + generated view; the pre-commit hook catches a stale render. Low-risk (data migration + additive `next`/template, no new enforcement logic).

Dependency order: Inc 1 -> Inc 2 (render needs the schema), Inc 1 -> Inc 3 -> Inc 4 (enforcement swaps need the schema; queue swap independent of step swap but sequenced to keep review surface small), Inc 3+Inc 4 -> Inc 5 (full cutover needs both sources swapped). Classify each risky increment for two consecutive clean rounds; the low-risk ones need one.

## 9. Trade-offs, risks, and open sub-questions for the synthesis

Defended lens choices and their costs:

- Moving waivers + baseline into the TOML is this design's signature call and its biggest risk. Upside: the plan becomes genuinely queryable (every exemption is a local, structured fact on its step; the render shows it inline), waivers become revisable, and the schema makes a dangling waiver unrepresentable. Downside: it re-homes just-shipped, tested enforcement data, so Inc 3 rewrites W3/W5's source (risky), and it leaves 17 valid-but-unconsulted historical lines in the append-only log (mild ugliness). The alternative (keep every event type in the JSONL and make the TOML rich only via cross-reference pointers `waiver_ref`/`receipt_ref` into the log) is lower-risk and keeps the event log the single home for events, at the cost of a less-local, pointer-chasing query story and non-revisable waivers. THIS IS THE KEY OPEN SUB-QUESTION for the synthesis: relocate waivers to the TOML (this design's rich position, best queryability) versus keep them in the JSONL and reference them (lower churn, event-log purity). I recommend relocation, but flag it as the one call most worth cross-checking against designs A and C.
- The heavier authoring ceremony is real: editing a step's status is a TOML edit, its reasoning a sidecar edit, and reading the whole plan is the generated Markdown (or a `render`). Three files where today there is one. The rich lens accepts this for the Principle-8 payoff, and mitigates it with `next`/`status` (an agent rarely needs the whole document) and the pre-commit render hook (the render step is enforced, not remembered).
- The render engine is several hundred lines of new Rust with its own review loop, and `agent-scaffold`-on-PATH becomes mandatory for authoring. The committed generated Markdown mitigates the cold-bootstrap case (a reader without the tool still has the last render). The generated-file-must-not-be-hand-edited footgun is real for an LLM-authored system; the banner + `render_sha256` + golden test + pre-commit `render --check` are the layered guard, but a hard CI failure (not just a warning) is what actually stops a bad commit, so I lean stricter than the exploration doc on the CI check while keeping the local `validate` a warning.
- Structuring Success Criteria with `satisfied_by` links is a rich-lens addition beyond the exploration doc; the risk is that the links rot if a step is renamed. `validate --source` cross-checks them, so a broken link is caught, not silent.

Open sub-questions the human or the synthesis must settle: (1) the waiver-home question above; (2) whether the 17 historical waiver/baseline JSONL lines should be left in place (append-only purity) or the migration is treated as a one-time clean-slate exception that prunes them (cleaner log, breaks strict append-only once); (3) whether RESUME STATE's pointer half should become a small structured `[resume]` block in the TOML (fully derived) or stay ledger prose (I keep it ledger prose, derived by `next`, but a structured block is defensible); (4) the exact split of the render self-check severity (warn-local / fail-CI is my recommendation) and whether the pre-commit hook should hard-block on a stale render.
