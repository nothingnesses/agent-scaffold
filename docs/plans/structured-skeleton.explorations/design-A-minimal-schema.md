# Structured skeleton, design A: the minimal-schema / prose-maximal lens

Explorer lens: keep the `<task>.plan.toml` skeleton as THIN as the machine allows. The single question this document keeps asking is: what is the SMALLEST set of typed fields the tooling genuinely has to read, and can everything else stay as directly-editable Markdown the tool never parses? Everything the checks do not consume stays prose. The design is complete (schema, sidecars, render, migration, roadmap) but every trade-off is resolved toward a thinner TOML surface, a dumber render engine, fewer things an author edits in TOML, and the smallest possible do-not-hand-edit blast radius.

This document improves on `docs/plans/structured-architecture.explorations/target-arch-B-cleanslate.md` rather than restating it; where I diverge from that document I say so and defend it.

---

## Starting reality: most of the pivot has already shipped

The target-arch-B document was written before the enforcement rewrite landed. The current repo is materially further along than that document assumes, and this changes the whole cost picture:

- `trivial` and `grandfathered` are already RETIRED from `ROADMAP_STATUSES` (`src/plan.rs:59-60`). The unified waiver model already exists as a `type:"waiver"` JSONL record with `unit` / `step` / `increment` / `reason` / `evidence_tier` / `evidence` (documented in `pack/instrument.md:11`).
- W3 (convergence-OR-waiver), W4 (decision receipts vs a declared `type:"baseline"` cutoff), W5 (waiver integrity), and the round-log consistency check are all implemented and tested in `src/workflow.rs`. The `pause.md` catch is pinned by tests (`check_workflow_catches_the_pause_pattern...`).
- Decision receipts (`type:"decision"` with `q_id` / `options` / `recommendation` / `chosen`), and the baseline cutoff (`type:"baseline"`), are already JSONL record types with their own projections in `src/metrics.rs`.

So the option-labels-with-no-home smell (B4), the exemption proliferation (E2/E3/E4), and the decision-receipt substrate are ALREADY resolved, and they live in the append-only JSONL log, not in the plan. The old target-arch-B roadmap spent its first two increments building decision receipts and the waiver model; those are done.

What is STILL a hand-authored Markdown-parsed-region source, and is therefore all that the structured-skeleton initiative has left to move:

1. The Roadmap table (`## Roadmap`), parsed by `plan::parse_roadmap` into `Step { slug, status }`.
2. The Open Questions queue (`## Open Questions...`), parsed by `plan::parse_questions` into `Question { id, status, ask }`.
3. The Step-Detail heading slugs, cross-referenced against the Roadmap by `validate_plan`.
4. Everything else in the plan: the Status line, Motivations, Project Principles, Documentation Protocol, Repository Layout, Step Detail bodies, Success Criteria (all prose no code reads).

That is the entire migration surface. The thin-lens consequence is large: because the checks read only `slug` / `status` from steps and `id` / `status` (plus the `folded_into` slug baked into the status string) from questions, the `<task>.plan.toml` needs to carry almost nothing, and the enforcement code barely changes.

---

## Area 1: the `<task>.plan.toml` schema (as thin as the machine allows)

### The boundary rule

A field goes in TOML only if a CHECK or the RENDER reads it as a typed value. Everything a human reads to reason about the work stays in a Markdown sidecar that the tool includes verbatim and never parses. One-line LABELS that name a skeleton row (a step's title, a question's ask) are the one concession: they are one line, they are part of the skeleton rather than the reasoning payload, and keeping them inline lets every sidecar be a fully opaque blob (see Area 3 for why that is worth two extra short strings).

### What the machine actually reads today

Grounding the boundary in the current code, the total set of plan-derived values any check consumes is:

- Per step: `slug` (W3 join key, cross-reference key, uniqueness), `status` (W3 gate: is it `complete`; render: the Roadmap cell), and the blocking target inside `blocked on <slug>` (validated to resolve to a real slug).
- Per question: `id` (W4 scope + index vs the baseline cutoff), `status` (W4 gate: does it start with `decided -> folded into`), and the fold-into `<slug>` (validated to resolve to a Roadmap slug).

Nothing else. `ask` is carried into `status --json` but no check reads it. Options / recommendation / chosen are NOT read from the plan at all; they live in the JSONL `type:"decision"` receipt.

### The schema

`docs/plans/<task>.plan.toml`:

```toml
# The plan skeleton. Machine-read structure only; all narrative lives in the
# sibling Markdown sidecars and is never parsed by the tool. Regenerate the
# human-readable docs/plans/<task>.md with `agent-scaffold render` after editing.

[meta]
task  = "agent-scaffold"   # the task slug; also the sidecar basename convention
title = "agent-scaffold"   # the rendered "# <title> plan" heading

[[step]]
slug   = "core-assets"
title  = "Bundle the pack assets into the binary"
status = "complete"

[[step]]
slug       = "state-queries"
title      = "status --resume and friends"
status     = "not started"
blocked_by = "state-schema"   # optional; replaces the "blocked on <slug>" status string

[[question]]
id     = "Q-45"
ask    = "Skeleton depth: how much of the plan becomes structured data?"
status = "decided"
folded_into = "structured-skeleton"   # required iff status == "decided"; forbidden otherwise

[[question]]
id     = "Q-50"
ask    = "Should render ever consume the JSONL log?"
status = "open"
```

Field-by-field, and what is deliberately ABSENT:

**`[[step]]`**: `slug` (kebab, stable), `title` (one line), `status` (one of `not started`, `in progress`, `complete`, `skipped`, `next`, `optional`, `deferred`), and OPTIONAL `blocked_by` (a slug). Order is ARRAY ORDER: the sequence of `[[step]]` tables IS the Roadmap order. There is deliberately no `order` integer (an explicit index is redundant with array position and is a renumbering-drift hazard; TOML preserves array order, so the array is the single source of sequence). There is deliberately no `detail_file` pointer (the sidecar path is a fixed convention, see Area 2; a pointer field can dangle, a convention cannot). `blocked_by` as a field replaces the parametric `blocked on <slug>` status string, so "blocked" is no longer a status value at all, which removes a parametric-status special case from the vocabulary.

**`[[question]]`**: `id` (`Q-<n>`), `ask` (one line), `status` (one of `open`, `exploring`, `decided`, `superseded`), and OPTIONAL `folded_into` (a slug, required exactly when `status == "decided"`). `decided` is now a plain enum value and `folded_into` is a separate field, which closes SE-11 (the `decided -> folded into <slug>` prefix hack) with no parsing convention. There is deliberately no `options` / `recommendation` / `chosen` and no `body_file` pointer.

**`[meta]`**: `task` and `title` only. `task` doubles as the sidecar basename so no per-item path field is needed.

That is the ENTIRE authored schema: two one-line strings and an enum per step, three one-line strings and an enum per question, two strings of meta. No principle list, no success-criteria string, no per-item file pointers, no order integers, no increment enumeration, no decision option arrays.

### Why NOT put options / chosen / principles / success-criteria in the TOML

This is the sharpest divergence from target-arch-B, which put `options` / `recommendation` / `chosen` on the question entry (intentionally duplicating the JSONL receipt and cross-checking with W4), and `[[principle]]` entries and a `success_criteria` meta string in the TOML.

The thin lens rejects all of that:

- **Decision option-labels + choice stay ONLY in the JSONL receipt.** They already have exactly one structured home (the `type:"decision"` record) and W4 already enforces that a decided question has one. Copying them into the plan TOML re-creates the two-homes-plus-a-cross-check pattern the audit was trying to shrink. Under Principle 16 the cleanest state is ONE home; the plan says only "this question is `decided`, folded into `<slug>`", and the durable option/choice audit trail is the receipt. The rendered human view shows the decision NARRATIVE from the question sidecar prose and the compact structured facts via `agent-scaffold status --json` (which reads the receipt), so nothing is lost by keeping the labels out of the plan. This is strictly LESS overlap than target-arch-B, and it removes a whole W-check's worth of "do the two homes agree" logic that target-arch-B had to add.
- **Project Principles stay prose.** No code parses the plan's principle list; the "Principle N" citations in `src/*.rs` refer to the SCAFFOLD principles in `pack/principles.toml` (which already have structured ids) and to the plan's own list only positionally. SE-1 (the namespace collision) is a NAMING problem solved by prefixing citations ("Project P5" vs "Scaffold P16"), not a storage problem that needs the plan's principles turned into typed rows. Modeling them as `[[principle]]` entries would add schema surface with zero machine consumer. They live in an opaque prose sidecar and render splices them verbatim.
- **Success Criteria, Motivations, Documentation Protocol, Repository Layout stay prose**, for the same reason: nothing reads them.

The result: the plan TOML holds the skeleton and only the skeleton. An author who wants to change a status or fold a question edits a handful of typed fields; an author who wants to write reasoning edits Markdown. The "do not hand-edit" footgun (Area 3) is confined to a single generated file, and the TOML is small enough to read at a glance.

### The step <-> increment link (B6 / SE-10), kept out of the plan

The lexical `-inc<x>` strip (`leading_slug` in `src/workflow.rs`) is the last string-convention join. The thin fix does NOT enumerate increments in the plan TOML (that would be a growing structured list the author must keep in sync with the log). Instead, give the JSONL records that already name a unit an explicit typed link: add an optional `step` field to `type:"round"`, `type:"waiver"`, and `type:"escalation"` records, naming the Roadmap step slug directly. W3 and W5 then join `round.step == step.slug` structurally; `leading_slug(task)` becomes a fallback used only for pre-migration records that lack the field. The plan TOML stays thin (it has no increment data), the append-only log is never rewritten, and the fragile lexical strip is retired for all new records. The increment token itself stays where it already is (the `task` string), which is the natural home for it because it keys the round-log consistency streak.

### How the JSONL log relates to the plan TOML (roles kept separate)

The two sources have disjoint, defined roles and MUST NOT be merged:

- `<task>.plan.toml` is the LIVE, mutable skeleton: the current set of steps and questions and their current status. It is edited in place; a status changes from `in progress` to `complete`.
- `docs/metrics/workflow.jsonl` is the APPEND-ONLY EVENT LOG: rounds, escalations, dismissals, intakes, decision receipts, baselines, waivers. It is never rewritten; it is the durable audit trail.

The TOML subsumes NONE of the JSONL record types. The overlap between them is minimal and one-directional: the plan says a step is `complete` or a question is `decided`, and the log carries the EVIDENCE that justifies that state (converging rounds / a covering waiver for W3; a decision receipt for W4). W3/W4/W5 are exactly the joins that check the live state against the durable evidence. This is the same two-homes-one-check shape that exists today, unchanged; the only thing the initiative changes is that the "live state" side is read from typed TOML instead of parsed Markdown.

### The enforcement model survives with near-zero change

Because the machine-read fields are preserved (`Step { slug, status }`, `Question { id, status }` plus a now-separate `folded_into`), swapping the source from Markdown regions to TOML is a PARSER SWAP, not a rewrite of the checks:

- W3 reads `Vec<Step>`; it does not care whether they came from a pipe-table or a TOML array.
- W4's `question.status.starts_with(QUEUE_FOLD_PREFIX)` gate becomes `question.status == "decided"`, and it reads `folded_into` where it needs the target. Its baseline logic, its receipt join, its exemption semantics are untouched.
- W5 is entirely JSONL-side; it is untouched except that its `step`-slug existence check reads the TOML step list.
- The round-log consistency check is entirely JSONL-side; untouched.
- The `pause.md` catch is untouched (a `complete` step with no records and no waiver still fails).

So every load-bearing invariant in the brief keeps working, and the risk of the migration is concentrated in the parser and the render, not in the enforcement logic.

---

## Area 2: the prose sidecars

### Layout (fixed-convention paths, no pointer fields)

Given `docs/plans/<task>.plan.toml`, the sidecars are, by convention:

- `docs/plans/<task>.overview.md`: the FRONT prose zone. Everything from below the Status line down to Open Questions in today's plan: Motivations, Project Principles, Documentation Protocol, Repository Layout. The author structures it with `##` headings however they like; render splices the whole file verbatim.
- `docs/plans/<task>.success.md`: the TAIL prose zone (Success Criteria), spliced verbatim at the end.
- `docs/plans/<task>.steps/<slug>.md`: one file per step, its design/decision/outcome body. Opaque include.
- `docs/plans/<task>.questions/<id>.md`: one file per question, its rationale body. Opaque include.

There is no frontmatter and no per-file pointer in the TOML: the path is derived from the task name and the slug/id. A missing sidecar is a hard render error (a step or question must have a body file), which is a cleaner invariant than a dangling `detail_file` pointer.

### Example step sidecar (`agent-scaffold.steps/core-assets.md`)

```markdown
Bundle every pack asset (AGENTS.md, the prompt files, the templates) into the binary at build time so a scaffold run has no runtime dependency on the source tree.

Outcome: assets are embedded via `include_dir!` and materialised by `run_scaffold` with create-if-absent semantics (Principle 4). Verified by the scaffold snapshot test. Converged at incB, risky, streak 2.
```

Note there is NO status label, NO slug heading, and NO title in the sidecar: those are in the TOML, and render synthesizes the `### \`core-assets\`: ...` heading. The sidecar is pure body prose, so the tool can include it byte-for-byte without parsing a single line of it.

### Example question sidecar (`agent-scaffold.questions/Q-45.md`)

```markdown
Skeleton depth. Option A keeps a thin plan.toml (Roadmap + queue skeleton only) and pushes all narrative to Markdown sidecars. Option B moves more of the plan into structured fields. The reasoning judged against the Project Principles: a thinner skeleton minimizes the do-not-hand-edit blast radius (Principle 2, minimal by default) at some cost to how much the rendered md can show structurally...

Decided: option B clean-slate, thin variant. The precise option set and the human's choice are recorded in the `type:"decision"` receipt for Q-45 in workflow.jsonl.
```

The decision NARRATIVE is here; the auditable option-set-and-choice is in the receipt. The sidecar prose does not re-enumerate the option labels as a canonical list (the convention SE-12 flagged), because the receipt is their home.

### The no-round-trip / no-clobber guarantee

Render WRITES exactly one file: `docs/plans/<task>.md`. It never writes the TOML and never writes any sidecar. It never READS a sidecar as anything but an opaque byte blob to splice. Therefore:

- The tool can never rewrite an author's Markdown from parsed structure (there is no parse of the prose to round-trip).
- The only file that gets overwritten is the generated `<task>.md`, which is derived and meant to be overwritten.

This is the strongest form of the invariant: because sidecars are opaque, there is no "reconstruct the prose from fields" path even in principle.

---

## Area 3: the `agent-scaffold render` engine

### Design goal: the dumbest engine that produces today's document

Render is a pure function of `(plan.toml, sidecars) -> <task>.md`. It does NOT read the JSONL log (defended below). It parses TOML (which the toolchain already does for `pack.toml` / `principles.toml`) and concatenates opaque Markdown blobs with a few generated fragments. No Markdown parsing, no templating language, no options-from-receipt lookups.

### Why render does not consume the JSONL log

Keeping render's input to `plan.toml + sidecars` is a deliberate thinness choice:

- It makes render a pure function of two committed, hand-authored sources, so the golden check (below) compares against a stable input, not against an append-only log that grows every round.
- The decided-question option labels (the only thing render might want from the log) are already narrated in the question sidecar and surfaced by `status --json`; the rendered plan document does not need to re-show them.
- Waiver annotations in the Roadmap table (which target-arch-B wanted) would couple render to the log. The thin lens drops them from the rendered table; waivers are visible through `agent-scaffold validate --workflow` and `status`, which is where an operator looks for enforcement state anyway.

If a future need to show log-derived facts in the rendered md appears, it is a separate, opt-in enrichment with its own design pass; do not fold it into the core render.

### The projection (structure of the generated `<task>.md`)

```markdown
# agent-scaffold plan

<!-- GENERATED by `agent-scaffold render` from agent-scaffold.plan.toml and its
     sidecars. Do NOT edit this file. Edit the .plan.toml and the .overview /
     .success / .steps/ / .questions/ sidecars, then run `agent-scaffold render`. -->

Status: 38 complete, 5 in progress, 3 not started, 2 skipped, 1 deferred; 5 open questions (1 exploring). Next: `state-queries`.

<!-- Allowed statuses (generated from the tool's constants):
     steps: not started, in progress, complete, skipped, next, optional, deferred, blocked_by <slug>.
     questions: open, exploring, decided (folded into <slug>), superseded. -->

## Motivations

...verbatim from agent-scaffold.overview.md (which also carries Project Principles, Documentation Protocol, Repository Layout)...

## Open Questions, Decisions, Issues and Blockers

- `Q-45` (decided, folded into `structured-skeleton`) Skeleton depth: how much of the plan becomes structured data?
- `Q-50` (open) Should render ever consume the JSONL log?

## Roadmap

| Step            | Status                    |
| --------------- | ------------------------- |
| `core-assets`   | complete                  |
| `state-queries` | blocked on `state-schema` |

## Question Details

### `Q-45`: Skeleton depth: how much of the plan becomes structured data?

...verbatim from agent-scaffold.questions/Q-45.md...

## Step Details

### `core-assets`: Bundle the pack assets into the binary

...verbatim from agent-scaffold.steps/core-assets.md...

## Success Criteria

...verbatim from agent-scaffold.success.md...
```

Match to today's structure: near-identical. Three deliberate changes, each an improvement:

1. The Status line is DERIVED (step-status counts + open-question count + the first non-`complete` step as "Next"). This fixes D1 / A1 / SE-13 (the hand-maintained, drift-prone Status line) by construction: it cannot drift because it is not authored.
2. A single generated status-vocabulary comment replaces the hand-copied vocabulary prose that was drifting stale (B3). The live plan no longer carries a hand-maintained vocabulary list.
3. Question BODIES move from inline-in-the-queue to a `## Question Details` section that mirrors `## Step Details`, so the queue is a scannable one-line-per-item list (this also fixes SE-15, the multi-KB `ask` blob, because the ask is a one-liner in the TOML and the body is a sidecar). The `## Open Questions` queue keeps the same id/status/ask shape a reader expects.

The `blocked on \`slug\``rendering in the Roadmap cell is synthesized from the`blocked_by` field, so the human view is unchanged even though "blocked" is no longer a stored status.

### The CLI surface

- `agent-scaffold render [--plan <task>.plan.toml] [--output <path>]`: reads the TOML + sidecars, writes `<task>.md`. Strict: a schema violation, an unresolved `blocked_by` / `folded_into`, or a missing sidecar exits non-zero and writes nothing (no partial render an agent might read as authoritative).
- `agent-scaffold render --check [--plan ...]`: renders in memory and compares byte-for-byte to the committed `<task>.md`. Exit 0 if equal; non-zero (with a diff summary) if the committed file is stale or hand-edited. This is the do-not-hand-edit guard.

### The do-not-hand-edit guard, concretely

Two layers:

1. The generated banner (shown above) tells any human or agent that opens `<task>.md` that it is derived and where the real sources are.
2. `render --check` is the enforcement. It is wired in two places: (a) a `[[check]]` entry in the project's `checks.toml` so `agent-scaffold checks` runs it as part of the normal quality gate, and (b) a recommended git pre-commit hook (`agent-scaffold render --check || { echo "run agent-scaffold render"; exit 1; }`) documented in AGENTS.md and shipped as an optional hook in the pack. Default posture: `render --check` WARNS in `checks` local runs (so a forgotten re-render does not block an in-flight workflow step) and FAILS in CI (`--strict`), matching the "warn loudly, fail hard only when asked" stance. This catches both a hand-edit of the generated file and a stale render after a TOML/sidecar edit.

### How validate / status read the new source

They read `plan.toml` DIRECTLY via a new `plan::parse_toml`, NOT the rendered md. The rendered md is a human artifact; no check ever parses it back. Concretely:

- `plan::parse_toml(path) -> (Vec<Step>, Vec<Question>, Meta)` replaces `parse_roadmap` / `parse_questions`. It deserializes the TOML with `serde` + the `toml` crate (already a dependency).
- `validate --plan` runs schema + cross-reference checks over the TOML: unique slugs, every `blocked_by` resolves to a real slug, every `decided` question has a `folded_into` that resolves to a real slug, question ids are well-formed, statuses are in-set. The Step-Detail cross-reference (`detail_slugs`) is REPLACED by a render-time "every step has a `steps/<slug>.md`" check, and the orphan-detail-block class of error becomes structurally impossible (details are keyed by step, so there is no free-floating detail block to orphan). This is a Principle 5 win (an illegal state removed, not just checked).
- `validate --workflow` (W3/W4/W5) reads `Vec<Step>` / `Vec<Question>` from `parse_toml` and the JSONL as before.
- `status` / `status --json` project from `parse_toml` (steps/questions) plus the existing JSONL projections (decisions, waivers). The `PlanProjection` shrinks because `ask` is now a one-liner, not a multi-KB blob (SE-15).

`--plan` accepts either a `.plan.toml` or a `.md` path during the transition; the extension selects the parser. After migration the `.md` parsers are removed (clean-slate; there is no backwards-compat mandate pre-adoption).

### The render-before-commit workflow

Author edits TOML and/or a sidecar; runs `agent-scaffold render`; commits the TOML, the sidecars, and the regenerated `<task>.md` together. `render --check` in `checks` / CI catches a missed re-render. Because the committed `<task>.md` is a full readable projection, a contributor without the tool installed can still READ the plan (they just cannot author it), which mitigates the "tool mandatory for authoring" cost the human accepted.

---

## Area 4: migration path for this repo

### What actually has to move (much less than target-arch-B assumed)

Because waivers, decision receipts, and baselines already migrated to the JSONL log, this migration touches ONLY the Roadmap + Open-Questions skeleton and the plan prose. The 113 JSONL records, and the `decision` / `waiver` / `baseline` semantics, are NOT touched by this migration at all: they stay exactly where they are, and W3/W4/W5 keep reading them. That is the lowest-regret property of the thin lens: the enforcement DATA never moves, so the migration cannot break an enforcement check mid-flight. The only join that changes is the plan side (Markdown regions -> TOML), and the `Step` / `Question` shapes are preserved so the joins still resolve.

### Steps

1. **Generate the TOML skeleton (scriptable).** Parse the current `## Roadmap` table into `[[step]]` entries (slug, status; `title` from the matching `### \`slug\`: <title>`Step-Detail heading;`blocked on X`->`blocked_by = "X"`). Parse the `## Open Questions`list into`[[question]]`entries (id;`ask`= the one-line ask;`decided -> folded into \`X\``->`status = "decided"`, `folded_into = "X"`; other statuses map straight across). No `trivial`/`grandfathered` conversion is needed (already retired). Array order = table order and queue order.
2. **Split the prose (the labour, but bounded).** Copy each Step-Detail BODY (everything under the `### \`slug\`:`heading, minus the heading) to`steps/<slug>.md`. Copy each question BODY (everything after the one-line ask) to `questions/<id>.md`. Copy Motivations + Project Principles + Documentation Protocol + Repository Layout to `overview.md`, and Success Criteria to `success.md`. This is roughly 51 step files + 45 question files + 2 meta files; it is copy-paste, not authoring.
3. **Render and diff.** Run `agent-scaffold render`; diff the output against the current hand-authored `agent-scaffold.md` to confirm fidelity (the Status line and the question-body relocation will differ by design; everything else should match closely). Adjust sidecar content, not the render, to close accidental gaps.
4. **Cut over.** Commit `agent-scaffold.plan.toml` + the sidecars + the generated `agent-scaffold.md`. From here the `.md` is generated and never hand-edited. Wire `render --check` into `checks.toml`.
5. **Add the structured `step` field going forward** (the B6 fix). New round/waiver/escalation records carry `step`; the `leading_slug` shim stays for the existing records. No existing record is rewritten (append-only).

### Big-bang vs incremental, and dual-run

Cut over in a SINGLE increment (increment 5 below), because the enforcement data does not move, so there is no window where a check reads half-migrated evidence. During the transition BEFORE that increment, the `.md` stays the hand-authored source and the TOML parser is developed and tested against fixtures and a copy; the live plan is only cut over once render fidelity is confirmed. There is no need for a long dual-run of two live sources; the dual-parser support (extension-selected) exists only so the tool can read either during the single cut-over commit and for any downstream project still on Markdown.

### Data-loss and check-integrity guards during migration

- The JSONL log is not touched, so W3/W4/W5 evidence is intact throughout.
- `validate --plan` and `validate --workflow` are run against the TOML BEFORE deleting the Markdown source; the cut-over commit only lands if both are green (same guarantees as today).
- The five orphan `task` values remain tolerated exactly as now (no matching `complete` step, so no W3 trigger); no change.

---

## Area 5: staged, reviewable roadmap

Each increment is independently shippable and reviewable under the role-separated loop. Risk class in parentheses; the earliest are the lowest-regret.

1. **TOML schema + parser (low).** Add the `plan.toml` schema and `plan::parse_toml -> (Vec<Step>, Vec<Question>, Meta)`. `validate --plan` reads TOML when the path ends `.plan.toml` (schema + cross-references: unique slugs, `blocked_by` resolves, `decided` implies a resolving `folded_into`, id shape, in-set statuses). No render, no behavior change to existing Markdown plans. Acceptance: `parse_toml` round-trips a fixture; `validate --plan` flags a bad status, a dangling `blocked_by`, and a `decided` question with no `folded_into`. LOWEST REGRET: pure addition, nothing reads it in anger yet.
2. **Point the checks at the TOML source (risky).** W3/W4/W5 and `status` read steps/questions from `parse_toml` when a `.plan.toml` is given; W4's decided-gate becomes `status == "decided"` and reads `folded_into`. Acceptance: the full `workflow.rs` test suite passes with TOML-sourced fixtures; the `pause.md` catch and the `optional-modules` waiver shape still behave identically.
3. **The render engine + golden check (risky).** Implement `agent-scaffold render` (opaque sidecar splicing, derived Status line, generated vocabulary fragment, do-not-edit banner) and `render --check`. Acceptance: rendering a fixture equals a committed golden; `render --check` fails on a one-byte hand-edit and on a stale render after a TOML edit.
4. **Structured step link on JSONL records (low).** Add optional `step` to `round` / `waiver` / `escalation`; W3/W5 prefer `step`, fall back to `leading_slug`. Acceptance: a round carrying `step` joins without the lexical strip; a pre-migration record with no `step` still joins via the shim. LOW REGRET: additive, retires the T3 over-strip risk for new data. Can land in parallel with 1-3.
5. **Migrate this repo (low).** Run the Area 4 procedure on `agent-scaffold.md`; render; commit TOML + sidecars + generated md; delete the Markdown source; wire `render --check` into `checks.toml` and document the pre-commit hook. Acceptance: render output matches the reviewed plan; `validate --plan`, `validate --workflow`, and `status` are all green on the migrated repo. No code change, so low risk.
6. **Pack template + scaffold story (low).** Replace `pack/plan-template.md` with `pack/plan-template.plan.toml` + `plan-template.overview.md` / `plan-template.success.md` + `plan-template.steps/.gitkeep` + `plan-template.questions/.gitkeep`; `scaffold` drops the TOML template and runs `render` once to produce the initial readable `<task>.md`. Migrate the `plan_template_documents_every_accepted_status` drift-guard to assert the vocabulary against the TOML template (or drop it, since render now generates the vocabulary from the constants directly, which single-sources it and makes the guard unnecessary). Acceptance: a fresh `agent-scaffold` run drops a coherent `plan.toml` + rendered `<task>.md` and `validate` passes on it.

Dependency order: 1 -> 2 -> 3 -> 5, with 4 parallel to 1-3 and 6 after 3. Increments 1 and 4 are the lowest-regret and can go first.

---

## Load-bearing invariants: how each is preserved

- **W3 / W4 / W5 / round-log consistency keep working.** They read the same `Step` / `Question` shapes from a different parser; the JSONL evidence is untouched. W4's only change is `starts_with(prefix)` -> `== "decided"` + `folded_into`.
- **The `pause.md` catch survives.** A `complete` step with no records and no covering waiver still fails W3; nothing about that path changes.
- **Pilots' existing records stay valid, `validate` still passes.** The 113 JSONL records are not rewritten; the migration only re-homes the plan skeleton, and the cut-over commit is gated on green `validate`.
- **Exemptions stay declared and visible; the two evidence tiers do not launder.** Waivers remain JSONL records read by W5, entirely outside this migration.
- **No round-trip / clobber of prose.** Render writes only the derived `<task>.md`; it reads sidecars as opaque blobs and never parses them, so there is no reconstruct-from-fields path.
- **The scaffolded default stays coherent.** Increment 6 makes a fresh scaffold drop a TOML skeleton + prose sidecars + a rendered md, all internally consistent and `validate`-green.

---

## Riskiest parts and open sub-questions for the synthesis

- **One-line labels: inline TOML vs sidecar first line.** I put `title` / `ask` inline in TOML so every sidecar is a fully opaque include (the strongest no-parse guarantee and the dumbest render). The alternative (take the label from the sidecar's first line) removes two TOML fields but makes the sidecar's first line load-bearing and reintroduces a tiny structured read of prose. I recommend inline; flag for the synthesis to confirm the trade (fewer TOML fields vs a fully-opaque sidecar).
- **Should render ever read the JSONL?** I say no (pure `plan.toml + sidecars`), which keeps the golden check stable and render dumb, at the cost that the rendered md does not show decided-question option labels or waiver annotations (those are in `status` / `validate` output). If the human wants the rendered plan to be a fully self-contained enforcement dashboard, that argues for letting render read the log; I recommend against it and keeping that a separate opt-in.
- **`## Question Details` relocation.** Moving question bodies out of the queue into a details section is a visible change from today's plan shape. It fixes SE-15 and makes the queue scannable, but it is a readability change a reviewer should sign off on.
- **B3 vocabulary fragment.** I have render emit a single generated vocabulary comment rather than dropping the vocabulary from the human doc entirely. Confirm that a generated comment is the wanted treatment (vs no vocabulary in the rendered plan at all, relying on `--help` / the template).
- **The migration prose split is the real cost.** ~51 + ~45 sidecar extractions is mechanical but large; a scripted split with a manual fidelity diff (increment 5) is the mitigation, but it is the single heaviest task and the place a data-loss slip would hide. The render fidelity diff against the current `.md` is the guard.

## What NOT to build (thin-lens YAGNI)

- No two-way sync from `<task>.md` back to the TOML. Render is one-way; the md is opaque output.
- No `options` / `chosen` / `recommendation` in the plan TOML. The JSONL receipt is their sole home; W4 already enforces it.
- No `[[principle]]` entries, no `success_criteria` string, no `[[waiver]]` table in the plan TOML. Principles and success criteria are prose; waivers are JSONL records already.
- No `order` integer and no `detail_file` / `body_file` pointers. Array order and path convention replace them.
- No increment enumeration in the plan TOML. The structured `step` field on JSONL records carries the link.
- No render consumption of the JSONL log, no waiver annotations in the rendered Roadmap table, no templating language in render.
