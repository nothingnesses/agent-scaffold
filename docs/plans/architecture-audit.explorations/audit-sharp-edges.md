# Broad sharp-edges audit

This is the third of the three Phase 1 audit files for Q-44. The other two cover the data model and the enforcement/exemption machinery specifically. This file covers what those narrower scopes miss: naming drift, pack internals, code dead ends, structural enforcement gaps, and doc/process fragility. Light overlap with the other audits is acceptable and noted where relevant.

Files read: all of `src/*.rs`, `docs/plans/agent-scaffold.md`, `docs/plans/agent-scaffold.ledger.md`, `docs/metrics/workflow.jsonl`, `pack/AGENTS.md`, `pack/plan-template.md`, `pack/LEDGER.template.md`, `pack/instrument.md`, `pack/principles.toml`, `pack/prompts/`.

---

## High-priority findings

These three are called out first because they each generate compounding confusion across multiple files.

### SE-1: Principle number namespace collision

Files: `src/plan.rs:11-12`, `src/metrics.rs:9-10`, `src/main.rs:822`, `src/checks.rs:22`, `src/checks.rs:179`

The project has two independent principle numbering systems: Plan Project Principles P1-P8 (in `docs/plans/agent-scaffold.md`) and the scaffolded AGENTS.md Principles P1-P22 (rendered from `pack/principles.toml`). Code comments in the same files cite both without any namespace indicator, so "Principle 5" at `src/plan.rs:11` means the Plan's P5 ("illegal states caught") while "Principle 16" at `src/plan.rs:12` means the scaffolded AGENTS.md P16 ("one source of truth"), and a reader has no way to distinguish them without cross-referencing both documents. Severity: moderate. Effort: small (add a namespace prefix, e.g. "Project P5" vs "Scaffold P16", or map by ID string). P8 direction: independent (this is a naming/citation hygiene problem; it predates P8 and persists regardless of the structured-data pivot).

### SE-2: "trivial" names two unrelated concepts

Files: `src/plan.rs:59-69` (ROADMAP_STATUSES), `src/metrics.rs` (Classification enum, `Trivial => "trivial"`)

The word "trivial" appears as both a Roadmap step status (W3-exempt completion where review was deliberately skipped) and as a `Classification` variant in the intake metrics enum (a risk class for a round, meaning the change is so small it needs no formal review). These are different concepts at different abstraction levels, and they share the same string token. A reader encountering "trivial" in a JSONL record or in plan context must resolve which sense is meant from context alone. The exemption-machinery audit covers the Roadmap side; this finding flags the cross-module naming collision. Severity: moderate. Effort: small (rename one; the metrics Classification variant is younger and more easily changed). P8 direction: partially resolves (a structured schema with distinct field paths for step status vs round classification makes the collision visible and forces a rename).

### SE-3: Two-tier workflow (instrumented vs non-instrumented)

Files: `src/workflow.rs` (W3 check gated on log existence), `src/main.rs:242-244` (empty instrument block when `--instrument` is off), `pack/instrument.md`

W3 convergence enforcement only operates when a project has opted into `--instrument` and has a JSONL log. A project that scaffolds without `--instrument` gets no machine enforcement of any workflow invariant; W3, the metrics validator, and the drift guards are all no-ops or absent. This makes `validate --workflow` silently pass for non-instrumented projects, giving a false green. The two-tier split is not documented in the scaffolded AGENTS.md as a trade-off; a user reading the role prompt would expect the same guarantees regardless. Severity: significant. Effort: medium (could add a warning on `validate --workflow` when no log exists, or require the log unconditionally). P8 direction: independent (opt-in instrumentation is orthogonal to whether the data format is structured; but P8 makes JSONL the primary source, which implies non-opt-in becomes harder to justify).

---

## Exemption machinery (light overlap with audit-enforcement-exemptions.md)

### SE-4: `optional-modules` permanently stuck as `in progress`

Files: `docs/plans/agent-scaffold.md` (step `optional-modules`, status `in progress`), `docs/metrics/workflow.jsonl:78-83`, `src/workflow.rs:137`

The step `optional-modules` cannot be marked `complete` because its increment 2c-ii converged at risky/1-clean (W3 requires 2) after a human-authorised escalation, and the fix (`escalation-exempt`, Q-40) is deferred into Q-44. So the step is permanently `in progress` in the Roadmap, which distorts status reporting, prevents the W3 check from validating it, and leaves the Roadmap in a state that cannot be closed without either adding the exemption or retroactively relabelling the round records. This is a concrete live symptom of the exemption proliferation problem. Severity: significant. Effort: medium (blocked on exemption unification in Q-44). P8 direction: partially resolves (if the exemption mechanism is modelled as structured data with a `reason` field rather than a status-string special-case, the stuck step can be closed cleanly).

---

## JSONL / logging gaps

### SE-5: `task` field in round records is an unchecked free string

Files: `docs/metrics/workflow.jsonl` (records 1-85), `src/metrics.rs` (parse_rounds does not validate task against Roadmap slugs), `src/workflow.rs` (W3 groups by task value)

Five task values in `workflow.jsonl` do not match any current Roadmap slug: `consolidate-plan`, `metrics-fields`, `plan-fold`, `plan-maintenance`, `workflow-hardening`. These are presumably steps that were renamed or removed, but because the `task` field is never cross-checked against the live Roadmap, the orphan records are silently exempt from W3 (no matching step is `complete`, so no check triggers). New orphan records can accumulate without any warning. Severity: moderate. Effort: small (add a cross-check in `validate --workflow` that warns on task values with no matching Roadmap slug). P8 direction: partially resolves (if the Roadmap is a structured source and W3 reads it at validation time, slug-matching becomes a natural schema join rather than a convention).

### SE-6: Metrics drift guard checks field name presence but not optionality

Files: `src/metrics.rs:674` (schema drift-guard test), `pack/instrument.md`

The drift-guard test at line 674 confirms that every validator-accepted enum value appears in `pack/instrument.md`, but it does not check whether fields marked required in `instrument.md` are actually required in the parser, or whether optional fields are consistent. The `validate_log()` function may accept records missing fields that the documentation says are mandatory; mismatches silently accumulate. Severity: moderate. Effort: small-medium (the `workflow-invariants` step description names this as a known gap). P8 direction: partially resolves (a JSON Schema or typed record schema as single source of truth would make required/optional status explicit and testable).

---

## Pack internals

### SE-7: `instrument.md` is an undocumented third category of pack content

Files: `src/main.rs:242-244`, `pack/instrument.md`, `pack/AGENTS.md` (tail render slots), pack manifest format

A pack has three kinds of content: manifest `[[asset]]` entries (declared in `pack.toml`), the `principles.toml` source (special-cased for the `{{principles}}` slot), and `instrument.md` (special-cased for the `{{instrument}}` slot, read directly at line 242-244 via `source.read("instrument.md").unwrap_or_default()`). The third category is not documented in the pack format spec. An author writing a custom pack has no signal that `instrument.md` is a magic filename, or that omitting it silently produces an empty `{{instrument}}` slot rather than an error. The `unwrap_or_default()` makes absent `instrument.md` silently succeed, which is correct for packs that don't need it, but surprising for packs that meant to include it and typo'd the path. Severity: moderate. Effort: small (document the special filenames in the pack format reference; optionally add a warning when the file is absent but the `--instrument` flag is set). P8 direction: independent (special-cased magic filenames are a design smell regardless of the data model pivot).

### SE-8: `ModuleSpec.description` field is dead code

Files: `src/manifest.rs` (ModuleSpec struct, `#[expect(dead_code, reason = "declared for the schema and TUI; not yet read by the loader")]`)

The `description` field on `ModuleSpec` is expected-dead because the TUI module selection pane it was designed for is deferred. The field is parsed from `pack.toml` (so pack authors can write it), but nothing reads or displays it. This is a held placeholder rather than a truly vestigial piece, but it means every pack is carrying schema surface that has no effect. Severity: cosmetic. Effort: small (remove and re-add when TUI lands, or document the placeholder intent in `pack.toml` comments). P8 direction: independent.

### SE-9: `checks.rs` parses `Test` and `Mutation` Kind variants but silently skips them

Files: `src/checks.rs:660` (`runnable_for()`), checks module `Kind` enum

`Kind::Test` and `Kind::Mutation` are valid parsed variants (a user can write `kind = "test"` in their checks config) but are silently skipped at runtime with only an informational message via `Runnable::Skip`. A user who configures test checks expecting them to run will get a quiet no-op. The skip is not surfaced as a warning in the `checks` subcommand output in a way that would be hard to miss. Severity: moderate. Effort: small (either surface a louder warning, or document the limitation in the config format so authors know these variants are reserved for future use). P8 direction: independent.

---

## Naming / vocabulary drift

### SE-10: `leading_slug()` is purely lexical and fragile

Files: `src/workflow.rs:55-69` (documented at lines 55-69), T3 latent issue

The `leading_slug()` function strips an `-inc<alnum>` suffix to group increment records under their parent step for W3. This is a purely lexical convention: it works only because the step naming convention happens to use that suffix pattern. If a step is renamed, split, or given a suffix that looks like an increment but isn't, the grouping silently breaks. The function is self-documented as a latent issue (T3). Severity: moderate. Effort: medium (requires either formalising the increment relationship as a structured field in the Roadmap, or adding validation that every `-inc`-suffix slug has a matching parent). P8 direction: directly resolves (a structured Roadmap with an explicit `parent` or `increments` field makes the relationship a data relationship, not a naming convention).

### SE-11: Queue status "decided" is a prefix match, not an exact status

Files: `src/plan.rs:82` (QUEUE_EXACT_STATUSES), `src/plan.rs:87` (QUEUE_FOLD_PREFIX)

`QUEUE_EXACT_STATUSES` contains `["open", "exploring", "superseded"]` and `decided -> folded into ...` is handled separately via `QUEUE_FOLD_PREFIX` as a prefix match. This asymmetry means the queue has four semantic states but three are checked as exact strings and the fourth as a prefix. The `decided` state also bakes the linking syntax (`folded into <slug>`) into the status string itself, coupling the status with a reference in a single field. If a `decided` item is not folded into a specific step (e.g. it is simply resolved without creating a step), the status format does not fit cleanly. Severity: cosmetic. Effort: small-medium (could split `status` and `folded_into` into separate fields in a structured queue). P8 direction: directly resolves (structured queue entries with distinct `status` and `linked_step` fields remove the prefix-parsing hack).

### SE-12: `superseded` queue status hides still-relevant design content

Files: `docs/plans/agent-scaffold.md:Q-43` (marked `superseded`)

Q-43 is marked `superseded` by Q-44 but contains load-bearing design content that is not preserved elsewhere: the reject-(b) reasoning for in-plan structured blocks (the multi-line parser boundary argument), the decided receipt fields (`q_id`, `options`, `recommendation`, `chosen`, `ts`), the `chosen in options` invariant, and the convention that a `decided` item's prose does not re-enumerate option labels. Marking it `superseded` tells readers (and future automated tools) to ignore it, but the content has not been folded into a durable location. If Q-43 is ever pruned from the plan, that reasoning is gone. Severity: moderate. Effort: small (fold the surviving design decisions into Q-44 or into a design-pass doc; or add a `content-preserved-in` annotation on superseded items). P8 direction: independent (this is a process hygiene gap; a structured plan could include a `superseded_by` link but would not automatically migrate content).

---

## Structural enforcement gaps

### SE-13: Plan `## Status` line is hand-maintained and unchecked

Files: `docs/plans/agent-scaffold.md` (top-of-file Status line), `src/plan.rs` (validate_plan does not check Status)

The plan's `## Status` line (e.g., "Status: in progress") is authored by the orchestrator and not verified by `validate --plan` against the actual Roadmap state. It can silently drift from the computed state (e.g., all steps complete but Status still says `in progress`). Severity: cosmetic (a misleading summary line, not a broken invariant). Effort: small (add a derived-status check to `validate --plan` that computes the implied status from Roadmap rows and warns if the header line disagrees). P8 direction: directly resolves (if the Status line is projected from structured Roadmap data, it cannot drift because it is not hand-authored).

### SE-14: RESUME STATE pointer rule is unenforced

Files: `pack/LEDGER.template.md` (RESUME STATE section), `docs/plans/agent-scaffold.ledger.md` (RESUME STATE), pack prompts

The AGENTS.md role prompts and the LEDGER template describe a convention that RESUME STATE must be a POINTER (slug + a one-line cursor) rather than a restatement of content. No check enforces this; the rule relies entirely on agents following the prose instruction. An agent that copies in a large prose restatement instead of a pointer will not be caught by `validate`. Severity: cosmetic (the section is not machine-read). Effort: small (a `validate --workflow` check could warn if the RESUME STATE block exceeds N lines). P8 direction: partially resolves (if RESUME STATE becomes a structured field with a typed `slug` and `cursor`, the pointer-not-restatement rule is enforced by the schema).

### SE-15: `Question.ask` captures the full remainder of the line for every queue item

Files: `src/plan.rs` (Question struct, ask field), `src/main.rs` (Projection struct, PlanProjection)

The `Question.ask` field captures the entire remainder of the line after the status parentheses. For Q-42 this is thousands of characters of dense design narrative. The `Projection` struct carries a full `Vec<plan::Question>` in `PlanProjection`, meaning the in-memory plan projection is very large relative to what any downstream consumer (e.g., `status --json`) realistically needs. There is also no structured `ask`/`options`/`resolution` split; the entire prose blob is one string. Severity: moderate. Effort: medium (splitting the question into structured fields, or capping the `ask` to a short summary with a separate `detail` field, would require format changes to the plan template and the parser). P8 direction: directly resolves (a structured queue entry would separate the title/summary from the body narrative, and `Projection` would carry only the typed fields).

---

## Documentation and process

### SE-16: Orphan task slugs in `workflow.jsonl` are permanent and growing

Files: `docs/metrics/workflow.jsonl` (records with task values `consolidate-plan`, `metrics-fields`, `plan-fold`, `plan-maintenance`, `workflow-hardening`)

Five task values in the committed JSONL log do not match any step in the current Roadmap. They are presumably steps that were renamed, merged, or removed. Because the JSONL log is permanent (append-only, never rewritten), these orphan records will always be present, and future step renames or removals will add more. There is no warning or annotation when a task slug disappears from the Roadmap while records for it remain in the log. Severity: cosmetic (they do not cause wrong W3 results, just silent exemption). Effort: small (a `validate --workflow` warning; and a process note in AGENTS.md that step slugs should not be renamed after rounds have been logged). P8 direction: partially resolves (structured cross-references between the log and the Roadmap enable automated orphan detection).

### SE-17: Module header principle citations mix two namespaces in the same comment block

Files: `src/plan.rs:11-12`, `src/metrics.rs:9-10`, `src/checks.rs:22`, `src/main.rs:822`

This is the code-level symptom of SE-1. Each module header cites "Principle N" values drawn from different documents in the same comment. For example, `src/plan.rs` lines 11-12 cite "Principle 5" (Plan's P5) and "Principle 16" (AGENTS.md P16) back to back. A contributor reading the code cannot tell which principle system is being cited without looking up both. Severity: cosmetic. Effort: small (add a namespace prefix or cite by ID string). P8 direction: independent.

---

## Summary table

| ID | Title | Severity | Effort | P8 resolves? |
|----|-------|----------|--------|--------------|
| SE-1 | Principle number namespace collision | moderate | small | No |
| SE-2 | "trivial" names two unrelated concepts | moderate | small | Partial |
| SE-3 | Two-tier workflow (instrumented vs not) | significant | medium | No |
| SE-4 | `optional-modules` permanently stuck in progress | significant | medium | Partial |
| SE-5 | `task` field is unchecked free string | moderate | small | Partial |
| SE-6 | Drift guard checks presence but not optionality | moderate | small-medium | Partial |
| SE-7 | `instrument.md` is undocumented third pack category | moderate | small | No |
| SE-8 | `ModuleSpec.description` is dead code | cosmetic | small | No |
| SE-9 | Test/Mutation Kind parsed but silently skipped | moderate | small | No |
| SE-10 | `leading_slug()` is purely lexical and fragile | moderate | medium | Yes |
| SE-11 | Queue "decided" is a prefix match, not a status | cosmetic | small-medium | Yes |
| SE-12 | Superseded Q-43 hides surviving design content | moderate | small | No |
| SE-13 | Plan Status line is hand-maintained and unchecked | cosmetic | small | Yes |
| SE-14 | RESUME STATE pointer rule is unenforced | cosmetic | small | Partial |
| SE-15 | `Question.ask` captures full multi-KB prose blob | moderate | medium | Yes |
| SE-16 | Orphan task slugs in JSONL are permanent | cosmetic | small | Partial |
| SE-17 | Module header citations mix two principle namespaces | cosmetic | small | No |
