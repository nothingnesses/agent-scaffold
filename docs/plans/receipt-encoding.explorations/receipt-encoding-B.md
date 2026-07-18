# Q-43 exploration: how the (chosen) Full decision receipt is ENCODED and where it lives (architecture / single-source-of-truth lens)

Explorer lens: architecture / single-source-of-truth, optimising for Principle 16 (one source of truth, derive the rest) and Principle 1 (clean long-term architecture). The question this document keeps asking of each encoding: which one gives the option-labels+choice exactly ONE authoritative home, derives the human-readable view from that home rather than duplicating it, and fits the existing architecture without introducing structural contradictions?

## 1. The question, restated, and what is fixed / out of scope

Q-43 (`docs/plans/agent-scaffold.md:122`) asks ONLY how the already-chosen Full decision receipt is ENCODED and where it lives, choosing among three contenders:

- (a) a permanent `type:"decision"` JSONL record in `docs/metrics/workflow.jsonl` (matches existing instrumentation records; leaves the plan format untouched; is a second permanent hand-authored home for option-labels+choice alongside the queue-item prose).
- (b) a structured block on the `decided` Open-Questions item IN THE PLAN (single home, co-located), parsed by `src/plan.rs`, checked by W4.
- (c) a structured receipt SOURCE with a single home, plus a PROJECTION/render so `agent-scaffold` renders it into the human-readable plan/status view (single source, derive the view).

Fixed inputs this pass may not re-open (from Q-42 resolution, `docs/plans/agent-scaffold.md:121`, and Q-43 item at `:122`): the Full receipt is chosen; the receipt records options + recommendation + human's choice with `chosen` validated as a member of `options`; a W4 `validate --workflow` check asserts every `decided` item has a receipt (forward-looking; historical Q-1..Q-41 exempt); receipts do NOT live in the ledger; `type:"escalation"` and `type:"intake"` records stay separate. The motivating concern for this pass: Explorer A of the prior round showed that a JSONL receipt is a second permanent hand-authored home for option-labels+choice alongside the queue-item prose, a Principle 16 drift risk.

Out of scope (noted, not designed): the broader "rewrite the whole plan as a uniform structured file format" idea. The plan is ~80% reasoning narrative; structuring a prose-heavy document fights the format and is a behaviour change for every scaffolded project. The receipt needs only the narrow encoding decision, not the full rewrite.

## 2. The three encodings with concrete cost accounting

### Ground truth: the parsing boundary and the existing projection

`src/plan.rs` parses exactly two structured regions and nothing else (`:1-15`): the Roadmap pipe-table (`parse_roadmap`, `:235`) and the Open-Questions queue (`parse_questions`, `:274-309`). The queue parser projects exactly three fields per item, reading ONE list line per item. The module header states the boundary explicitly: "the free narrative (Step Details, motivations) holds no machine state and is not parsed." This boundary is not incidental; it is what lets `validate --plan` hard-fail on real structural violations without false-positives from prose that happens to look structured.

`src/metrics.rs` owns the JSONL schema. `check_record` (`:276-332`) has a `match record_type` with four arms (`round`, `escalation`, `dismissal_recheck`, `intake`) and an `other` arm that rejects unknown types. A fifth arm slots in cleanly before `other`. `parse_rounds` (`:372-415`) is the projection pattern: best-effort, skips lines that don't match, returns only the fields the cross-reference needs. A `parse_decisions` function would parallel it exactly.

`src/workflow.rs`: `check_workflow` (`:77-86`) already takes both `plan_markdown` and `log_contents`, calls `plan::parse_roadmap` and `metrics::parse_rounds`, and hands both to W3. W4 would call `plan::parse_questions` and the new `metrics::parse_decisions` and extend the `problems.extend(...)` chain. No signature change needed.

`src/main.rs` `run_status` (`:643-695`): reads the plan's two regions into `PlanProjection { steps, open_questions }`, reads the metrics log for a record count into `MetricsProjection { records }`, wraps them in `Projection { plan, metrics }`, and renders either as `serde_json::to_string_pretty` (`:668`) or as a short human-readable summary (`:671-693`). This is the established pattern for deriving a view from structured sources without writing any file.

The drift-guard test in `src/metrics.rs` (`:673-741`) asserts every accepted record type, field name, and enum spelling appears verbatim in `pack/instrument.md`. Adding a `decision` record type requires updating both the `check_record` match and the prose, and extending the field list in the test.

### Option (a): `type:"decision"` JSONL record + W4

Data model. A new arm in `check_record`'s match, between the `intake` arm (`:322-331`) and the `other` arm (`:329`). Required fields: `q_id` (`require_str`), `options` (a non-empty array of strings, a new ~8-line helper modelled on `require_severities` at `:206-226` minus the enum step), `recommendation` (`require_str`), `chosen` (`require_str`, then a cross-field `options.contains(chosen)` check - roughly 5 lines, the only genuinely new constraint type). Optional: `ts` (already handled by the common block at `:283-285`).

`src/plan.rs` changes: ZERO. The queue grammar, the one-line-per-item parser, and the no-prose boundary are untouched.

`src/metrics.rs` changes: one `"decision"` arm (~20 lines), a new `parse_decisions(contents) -> Vec<(q_id, line_number)>` function parallel to `parse_rounds` (~25 lines), the cross-field `chosen`-in-`options` validation (~5 lines), drift-guard test extension (~4 lines), and prose additions to `pack/instrument.md`. The `chosen`-in-`options` check is the one structurally new thing; everything else follows existing patterns in the file.

`src/workflow.rs` (W4): call `plan::parse_questions` (already imported via `use crate::plan`), call `metrics::parse_decisions`, for each `Question` whose `status` starts with `QUEUE_FOLD_PREFIX` (`:87`) check for a matching `q_id` in the decisions. Forward-looking boundary: W4 fires only when at least one `type:"decision"` record exists in the log (the log's absence for pre-mechanism decisions is the historical exemption). Roughly 30-40 lines plus tests, directly mirroring W3 (`:150-212`).

Pack plan-template format change: NONE. `pack/plan-template.md` is byte-identical. Blast radius across scaffolded projects: zero for uninstrumented projects; for instrumented projects, the plan format is still unchanged - only the JSONL log gains one more record type.

SSOT analysis for (a). The option-labels+choice have ONE machine home: the JSONL record's `options` and `chosen` fields. The human-readable view comes from the queue-item prose, which traditionally ALSO enumerates the options and choice (see Q-40 at `:119`, Q-41 at `:120`). This is the second home Explorer A correctly identified. The standard closure is a convention: the queue prose writes reasoning and a decision pointer but NOT option labels. The receipt is their machine home. This convention is not machine-enforced; it relies on the orchestrator following the split.

Where does the human-readable view come from? In (a) without any render extension, the human reads options+choice EITHER from the queue-item prose (if the convention is broken and options are re-enumerated there) or from the raw JSONL (if the convention is followed). Neither is derived. If the convention is followed, a human reading the plan does NOT see options+choice inline; they must read raw JSON or run a command.

### Option (b): structured block on the `decided` OQ item in the plan + W4

Data model. The queue-item format extends from a one-line bullet to a multi-line item: the existing `- \`Q-n\` (status) ask`line followed by indented structured fields (for example,`options: [a, b, c]`, `recommendation: a`, `chosen: a`). Alternatively, an appended TOML/JSON block under each item. In either form, the parser must now track CONTINUATION LINES belonging to a queue item, a concept `parse_questions` (`:274-309`) entirely lacks.

`src/plan.rs` changes: the LARGEST cost of the three, and it crosses the stated boundary. `parse_questions` iterates `section_lines` one line at a time with independent `continue` guards; adding multi-line item tracking requires: carrying state ("am I inside a decided item?"), detecting continuation lines (by indentation or a block fence), parsing structured fields from those lines, adding optional `receipt: Option<ReceiptBlock>` to the `Question` struct (`:35-44`), validating the member constraint (`chosen` in `options`) inside `queue_structure_problems` (`:201-229`) or a new validator, and writing tests for all the new paths. Conservative estimate: 60-100 lines of new parser, plus new tests, plus the `ReceiptBlock` struct. More important than the line count, this erodes the "two clean parsed regions, prose not parsed" invariant that makes `src/plan.rs` tractable and that Principle 5 (illegal states unrepresentable) relies on.

`src/metrics.rs` changes: NONE. The receipt is in the plan; the JSONL is uninvolved.

`src/workflow.rs` (W4): simpler than (a)'s version, since the cross-reference is now plan-only (no log needed). W4 could live entirely in `validate_plan` (`:360-428`) rather than `check_workflow`. But this saving (~10 lines) is far outweighed by the parser cost above.

Pack plan-template format change: MANDATORY, and this is the decisive blast radius difference. `pack/plan-template.md:24` defines the queue-item format for EVERY scaffolded project, instrumented or not, decision-recording or not. The current definition says "Each item has a stable id, a one-line ask, a status ..., and a pointer to the step, ledger, or exploration that carries the detail." Option (b) changes that format and the parser that enforces it for ALL downstream projects. The drift-guard test at `src/plan.rs:754-784` pins the plan-template vocabulary to the validator's constant sets; it would also need extension for the receipt block format. This is the one encoding whose cost every downstream scaffolded project pays unconditionally - the inverse of Principle 3 (safe on existing projects).

SSOT analysis for (b). The option-labels+choice have ONE home in the plan file: the structured block on the queue item. The block IS both the machine-parseable receipt AND the human-readable display (a human reading the plan sees it inline). No separate JSONL record. No convention about queue prose needed. But: the STEP DETAIL section ("`### `slug`:`" heading with the decision rationale) narrates the same decision in prose, including the options considered and why one was chosen. That prose is the authoritative content record (the queue=CONTENT / receipt=PROCESS split the Q-42 resolution accepted). So within the PLAN FILE, both the structured block (labels+choice) and the step-detail prose (rationale that mentions the same labels) exist. This is intra-file soft duplication, which is softer than cross-artifact duplication (harder to drift when both are visible in one edit session), but it is still duplication.

Where does the human-readable view come from? The structured block itself, inline in the plan. A human reading the plan sees it directly without running any command. This is the strongest display property of the three, but it costs the parser change and the template blast radius.

How does W4 read the receipt? From `plan::parse_question_receipts` (or the extended `parse_questions`), entirely within `validate --plan`. No log read needed for the W4 check specifically.

### Option (c): JSONL as the single structured source + thin render into `status` + W4

Data model. The receipt fields are identical to option (a): a `type:"decision"` JSONL record with `q_id`, `options`, `recommendation`, `chosen` (validated as a member of `options`), and optional `ts`. The source IS the JSONL - this is not a new artifact class or a separate sidecar file. What makes (c) distinct from (a) is: (1) a thin render layer that projects receipts into the `status` output, making the options+choice visible without running a separate command or reading raw JSON; and (2) the explicit convention that the queue-item PROSE does not re-enumerate option labels, because the `status` view derives them from the receipt.

`src/plan.rs` changes: ZERO. Identical to (a).

`src/metrics.rs` changes: IDENTICAL to (a). Same `"decision"` arm, same `parse_decisions`, same drift-guard updates.

`src/workflow.rs` changes: IDENTICAL to (a). Same W4 implementation.

Pack plan-template format change: NONE. Identical to (a).

The render layer (the differentiator between (a) and (c)). `run_status` (`:643-695`) already projects two structured sources into a `Projection` struct and renders both JSON and human-readable views without writing any file. Option (c) extends this with a third projection slot. Concretely:

- Add a new `DecisionProjection { q_id: String, options: Vec<String>, recommendation: String, chosen: String }` struct deriving `Serialize` (~5 lines).
- Add `decisions: Option<Vec<DecisionProjection>>` to `Projection` (`:408-413`) (~1 line plus the struct field annotation).
- In `run_status`, when the metrics file exists, call `metrics::parse_decisions(&contents)` and convert the results to `Vec<DecisionProjection>` (~5-10 lines).
- In the JSON branch (`:667-669`), the field is serialised automatically by `serde_json`.
- In the human-readable branch (`:671-693`), print a decisions section: for each receipt, one line such as `decision Q-42: [a, b, c], recommended a, chosen a` (~10-15 lines).

Total render cost: approximately 30-35 lines in `src/main.rs`, reusing machinery already present (`run_status`, `Projection`, `serde_json`). This does NOT require `workflow-viz`. The deferred `workflow-viz` step (`docs/plans/agent-scaffold.md:429-431`) is a live Gantt/NOM-style streaming visualiser consuming a dispatch/return event log not yet built; the thin `status` extension here is a one-time best-effort read of existing committed data, which is exactly what `status` already does for `plan` and `metrics`. The prior art is `state-schema`'s `status --json` (Q-11/Q-28), and the planned `state-queries` step (not yet started) is already extending `status` with `--resume`; adding `--decisions` or including decisions in the default JSON output is a natural fit for that step.

SSOT analysis for (c). The option-labels+choice have ONE machine home: the JSONL receipt. The human-readable view DERIVES from that home via the `status` render. The queue-item prose does NOT hand-author the option labels (the convention says: `decided -> folded into \`slug\`` plus a rationale pointer, no option enumeration). If this convention is followed, there is one machine home and one derived display, satisfying Principle 16's "one source, derive the rest."

The honest question is whether "if the convention is followed" is good enough. The convention is not machine-enforced: `validate --plan` does not check that a `decided` item's prose omits option labels, because checking prose content is exactly what `src/plan.rs` deliberately does not do. The SSOT guarantee is soft - it relies on the orchestrator writing the queue item correctly.

But option (c) creates a concrete INCENTIVE STRUCTURE that option (a) lacks. When `agent-scaffold status` visibly shows the options and choice from the JSONL, the orchestrator has a clear reason NOT to also enumerate them in the queue prose: the display is already handled. This is the meaningful difference from option (a) in practice: (a) without a render has no display home for options+choice in the derived view, so the pull to put them in prose is strong; (c) with a render removes that pull because there IS a derived display home.

Where does the human-readable view come from? The `status` output (derived). A human reading the plan Markdown directly does NOT see options+choice inline; they see the decision pointer and rationale in the queue item and step detail. To see options+choice, they run `agent-scaffold status`. This is the one limitation vs option (b): the plan file is not self-contained for options+choice.

How does W4 read the receipt? From `metrics::parse_decisions`, identical to (a).

### Which encoding has one authoritative home for option-labels+choice?

Option (a): one machine home (JSONL) IF the convention is followed. Drift is possible but unsupported: no display surface makes the drift visible.

Option (b): one home (the plan file), but intra-file soft duplication with the step-detail prose is unresolved. Structurally stronger than (a) across artifact boundaries; softer within the file.

Option (c): one machine home (JSONL) IF the convention is followed. The render creates a derived display surface that reduces the pull to hand-author a second copy in prose.

Honest ranking on structural SSOT: (b) > (c) > (a), where the gap between (c) and (a) is the render creating the incentive structure.

## 3. Trade-offs judged against the numbered Project Principles

Principle 16 (one source of truth, derive the rest). This is the lens for this pass. Option (b) achieves the strongest structural single-sourcing: the structured block in the plan IS the single home and the human-readable display, no derivation step needed. Option (c) achieves derived-view SSOT at the cost of requiring the `status` command for the display, with SSOT enforced by convention rather than machinery. Option (a) without a render has no derived display surface and therefore the weakest incentive structure for the convention; with the render, (a) collapses to (c). Principle 16 ranks (b) > (c) >= (a).

Principle 1 (clean long-term architecture). The JSONL is already the established machine-readable event log for the workflow (established in `round-log-core`, `Q-34`); adding a `decision` record type extends the established pattern cleanly. The `status` command already derives a view from structured sources; adding a decisions slot extends that established pattern. Option (c) is architecturally coherent with both patterns. Option (b) introduces multi-line parsing into `src/plan.rs` past the stated "two clean structured regions" boundary (`:1-15`), which works against the clean architecture rather than extending it. Principle 1 ranks (c) >= (a) > (b).

Principle 2 (minimal by default; do not complicate the core). Explorer A's document measures this accurately: (a) adds the least to the core, (c) adds ~30 lines over (a) for the render, (b) adds 60-100 lines of parser plus format changes. On pure minimalism: (a) < (c) < (b). But the 30-line render in (c) is not "complicating the core"; it extends an already-planned `state-queries` extension to `status`. The gap between (a) and (c) is small.

Principle 3 (safe on existing projects). Both (a) and (c) leave `pack/plan-template.md` byte-identical, so the inherited queue-item format is unchanged for every scaffolded project. Option (b) changes the inherited format unconditionally. Principles 2 and 3 both point away from (b) and toward (a)/(c).

Principle 5 (make illegal states unrepresentable). Option (b)'s structured block gives the plan validator direct machine-parseable access to the receipt, enabling a hard-fail on a `decided` item missing its structured block, without needing the JSONL at all. Option (a)/(c) requires the JSONL to be present and well-formed for W4 to fire; absent or malformed log lines are silently skipped by `parse_decisions` (the same best-effort pattern as `parse_rounds`, `:372-415`). On catching missing receipts: (b) can hard-fail in `validate --plan` (no log needed); (a)/(c) can hard-fail in `validate --workflow` (log required). Both are machine checks, just reading different sources.

Summary table (ranked best to worst per principle):

- Principle 16 (SSOT): (b) > (c) > (a)
- Principle 1 (architecture): (c) = (a) > (b)
- Principle 2 (minimal): (a) < (c) < (b)
- Principle 3 (safe on existing projects): (a) = (c) > (b)
- Principle 5 (illegal states): (b) slight edge; (a)/(c) acceptable

No single option dominates. The right call depends on how strictly to weight the SSOT goal.

## 4. Recommendation

Recommend option (c): JSONL as the single structured source, thin render in `status --json` and `status` human-readable, with the explicit convention that the queue-item prose does not re-enumerate option labels. The decision between (c) and (a) is close; the 30-line render is worth building because it creates the derived display surface that makes the SSOT convention self-reinforcing.

The case for (c) over (a). The only structural difference is the render layer. Without the render, the human has no display home for options+choice EXCEPT the queue prose, which creates constant pull to write them there. With the render, `agent-scaffold status` shows options+choice clearly, and the orchestrator writing the queue item knows the display is already handled elsewhere. This is not machine enforcement (validate --plan does not police prose content) but it is meaningful: the convention is supported rather than unsupported. For a tool that already uses detection-not-prevention across all its checks (W3, `validate --log`, `validate --plan`), a convention with a visible derived-display home is consistent with the project's discipline.

The case for (c) over (b). Option (b) achieves stronger structural SSOT but at a cost that exceeds its benefit on this project's own principles. The parser change crosses the `src/plan.rs` boundary that has been held deliberately clean across three converged increments (`state-schema`, Q-11/Q-24); re-opening it for a receipt block contradicts Principle 1's "clean long-term architecture" judgement embedded in those increments. The template blast radius (changing the inherited queue-item format for all scaffolded projects unconditionally) contradicts Principle 3 and Principle 2. And (b)'s SSOT claim is weaker inside the plan file than it appears across artifact boundaries: the step-detail narrative co-located in the same file still narrates the same options in prose, so intra-file soft duplication persists regardless. The structural gain of (b) over (c) is "the plan is self-contained for options+choice without running a command," which is real but not worth crossing the `src/plan.rs` boundary for.

The case against the null render (= pure option (a)). Building the receipt (option (a)) without the render creates a structurally weaker SSOT position: the derived display surface does not exist, the convention has no structural support, and the human must read raw JSONL to see options+choice. The extra 30 lines for the render close this gap without introducing any new complexity class. The render also aligns with the planned `state-queries` step, which is already extending `status` with `--resume` and a compact default; decisions fold into that extension naturally rather than requiring a separate later increment.

Minimal projection needed, concretely. The render surface is the existing `status` subcommand extended with a third projection slot. Specifically: add `DecisionProjection { q_id, options, recommendation, chosen }` (5 lines), add `decisions: Option<Vec<DecisionProjection>>` to `Projection` (`:408-413`, 1 field), populate it in `run_status` from `metrics::parse_decisions` (5-10 lines), serialise it for free in the JSON branch (`:667-669`), and print a human-readable decisions section in the summary branch (`:671-693`, 10-15 lines). This is ~30-35 lines in `src/main.rs`, reuses all existing machinery, does not require a new subcommand, does not write the plan file, and is explicitly NOT the deferred `workflow-viz` (which is a live streaming Gantt/NOM visualiser consuming a dispatch/return event log not yet built). The deferred `workflow-viz` is about agent lifecycle visualization in real time; this is a one-time committed-data read for `status`. No dependency on `workflow-viz` exists.

The SSOT convention to adopt alongside (c). The Open Questions section of `pack/plan-template.md:24` already says each item has "a one-line ask ... and a pointer to the step, ledger, or exploration that carries the detail." The convention for receipts is: a `decided -> folded into \`slug\``item carries a pointer; the option labels and the human's choice live in the JSONL receipt (projected by`status`); the prose carries rationale (WHY the choice was made), not labels. The step-detail under `slug` elaborates the rationale. This is the queue=CONTENT / receipt=PROCESS ATTESTATION split the Q-42 resolution accepted (`docs/plans/agent-scaffold.md:121`). The convention should be stated in one sentence in the `pack/plan-template.md`queue-section guidance and in the relevant section of`pack/AGENTS.md`'s instrumentation guidance.

Forward-looking historical exemption. W4 fires only when at least one `type:"decision"` record exists in the log (the presence of ANY decision record signals the mechanism is active). Items decided before the `decision-receipts` Roadmap step is complete are exempt by the log's absence for those items. No new queue vocabulary or status is needed. This boundary is identical across (a) and (c) and requires no extra design.

## 5. What NOT to build (the YAGNI boundary)

Do not write the receipt as a structured block on the queue item (reject (b)). The parser cost crosses the `src/plan.rs` deliberate boundary, the template format change is an unconditional blast radius for every scaffolded project, and the intra-file SSOT gain is softer than it appears (step-detail prose still narrates the same options).

Do not build a plan-writer that injects derived receipt content INTO the Markdown plan file (the heavy reading of (c)). The tool never writes the plan today; a generative plan-annotator is the deferred `workflow-viz` territory (`docs/plans/agent-scaffold.md:429-431`) and owes its own design pass. Do not pull it forward for a receipt.

Do not create a separate structured sidecar file (a `docs/decisions/receipts.toml` or equivalent) as the receipt source. A new artifact class needs a new parser, a new validation mode, and a new ownership model. The JSONL already exists and its schema, validator, and projection patterns are established; a new file buys nothing over reusing the JSONL and adds a second artifact the workflow must track.

Do not build the thin render as a SEPARATE subcommand (`agent-scaffold decisions` or similar). It is a natural extension to the existing `status` command (which already projects plan + metrics; decisions is a third slot in the same `Projection` struct). The planned `state-queries` step already extends `status`; fold the decisions projection into that increment.

Do not add `q_id` to `type:"escalation"` or `type:"intake"` records to make them serve double duty as partial receipts (fixed input: one purpose per record). An escalation that corresponds to a queue item gets its own `type:"decision"` receipt alongside the `type:"escalation"` event record.

Do not retroactively require receipts for Q-1 through Q-41. The forward-looking boundary (the log's absence for pre-mechanism items is the historical exemption) makes retroactive records unnecessary and the attempt to reconstruct options from git history or step prose would be unreliable.

Do not re-enumerate option labels in the queue-item prose once the receipt exists. This is the one action that would convert (c) into (a)-with-a-render-bolted-on, collapsing the SSOT gain. The convention is: the receipt is the machine home for labels; the prose is the content home for rationale. Both serve their purpose and neither duplicates the other.
