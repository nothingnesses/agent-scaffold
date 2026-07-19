# Design C: migration-safety / incremental-cutover lens

Explorer lens: migration-safety-first. The single question this document keeps asking: given that the dogfooded plan is 32,000+ tokens of live, depended-upon prose with 51 Roadmap steps, 45 queue items, 113 committed JSONL records, three actively enforced checks (W3/W4/W5), and a very large hand-authored Status line, what schema, sidecar structure, and render engine minimize the probability that the migration breaks something and maximize the probability that any breakage is caught and reversed before it ships? Backward-compat is explicitly demoted per Principle 8, but the invariant "the repo is always in a validatable state" is not negotiable.

Thesis: migration safety should drive several schema and engine choices that look like costs from a pure-architecture view but are essential for a live dogfooded system. Specifically: the TOML parser should have an explicit `.md` fallback so the repo is never un-validatable; the schema should be narrower than what the clean-slate exploration proposes (waivers and decisions are already in the JSONL, so the TOML holds only the skeleton); the render engine should be a strict one-way operation with a content-hash staleness gate; and the migration should proceed in a shadow-render phase where the generated output is compared against the hand-maintained source before any cutover. These choices are not architecture regressions; they are the pragmatic path to getting option B into production without stranding the repo's own 51-step plan.

---

## Area 1: The `<task>.plan.toml` schema

### What goes in the TOML and what stays in the JSONL

The pilots already settled this boundary more clearly than the clean-slate exploration assumed. After `decision-receipt` (pilot 1) and `waiver-model` (pilot 2), the JSONL already holds:

- `type:"decision"` records: q_id, options, recommendation, chosen, ts (W4 checks these)
- `type:"waiver"` records: unit, step, increment, reason, evidence_tier, evidence, ts (W5 checks these)
- `type:"baseline"` records: the Q-1..Q-41 historical exemption boundary
- `type:"escalation"` and `type:"round"` records (pre-existing)

None of these need TOML entries. The TOML's job is to hold the PLAN SKELETON only: the structural metadata the plan Markdown already carries in its two machine-parsed regions (the Roadmap table and the Open Questions queue) plus the prose anchors and plan metadata that are NOT currently structured.

The TOML schema:

```toml
[meta]
task = "agent-scaffold"
title = "agent-scaffold plan"
# Populated by `agent-scaffold render`; checked by `validate --source`.
# Empty string = never rendered. A stale value triggers a warning.
render_hash = ""
# Controls which source the parser uses. "markdown" (the default) keeps the
# hand-maintained .md as the primary source; "toml" cuts over to this file.
# This is the single cutover bit; it defaults to markdown so a partial TOML
# in progress does not accidentally become the primary source.
primary = "markdown"
# JSONL task slugs that no longer match any Roadmap step (steps renamed/removed;
# the log is append-only so these cannot be retracted). W3 skips tasks in this
# list when they have no matching step. Declare once, not per-check.
orphan_tasks = [
    "consolidate-plan",
    "metrics-fields",
    "plan-fold",
    "plan-maintenance",
    "workflow-hardening",
]
# Optional prose sidecar for the extended status narrative (the hand-authored
# resume-context paragraph). When present, the render appends it after the
# computed status summary. When absent, only the computed summary is rendered.
status_file = "steps/_status-narrative.md"

[[step]]
slug = "core-assets"
title = "Author the minimal core assets and principle data"
status = "complete"
order = 1
# Path relative to the plan directory; the render engine inlines this file.
detail_file = "steps/core-assets.md"

[[step]]
slug = "structured-skeleton"
title = "Convert the plan skeleton to plan.toml + sidecars + render"
status = "next"
order = 51
detail_file = "steps/structured-skeleton.md"

[[question]]
id = "Q-1"
# Exact-match statuses: "open", "exploring", "superseded".
# Parametric decided status: "decided" (no slug baked in; folded_into is the
# separate structured field, replacing the "decided -> folded into `slug`" hack).
status = "decided"
folded_into = "convergence-accounting"
ask = "convergence-accounting round accounting: how to make the escalation cap computable from the ledger"
body_file = "questions/Q-1.md"

[[question]]
id = "Q-45"
status = "decided"
folded_into = "structured-skeleton"
ask = "the Q-44 skeleton-depth split: how far to structure the human-authored skeleton"
body_file = "questions/Q-45.md"

[[principle]]
number = 1
name = "Prefer clean long-term architecture"
text = "Prefer the cleaner long-term architecture over the smallest diff: prioritise correctness, internal coherence, and maintainability, and when a local fix conflicts with a cleaner design, choose the cleaner design unless a concrete limitation prevents it."
```

### Key schema choices and their migration-safety rationale

**`[meta] primary` field.** This is the single cutover bit. The parser reads `.md` when `primary = "markdown"` (or the field is absent) and reads TOML when `primary = "toml"`. A partial TOML in progress never accidentally becomes the primary source and does not break `validate`. The cutover is a deliberate, one-line edit to the TOML, committed alongside the rendered `.md` in one atomic commit.

**No `[[waiver]]` section.** The clean-slate exploration proposed migrating waivers into TOML entries. The pilots already moved them to the JSONL. Adding them back to the TOML would create two homes for waivers and re-introduce the Principle 16 smell the pilots resolved. W5 already reads the JSONL; it does not need a TOML source. The TOML is narrower and therefore safer to migrate to.

**`[meta] orphan_tasks`.** The five orphan JSONL task slugs (`consolidate-plan`, etc.) are a known gap (SE-5/SE-16). Under a `.md` source, W3 silently skips them (no matching step is `complete`, so no check triggers). Under a TOML source, the parser can cross-check task slugs against the step list and emit warnings. Declaring the orphans in `[meta]` makes the known-orphan set explicit and machine-checkable, which closes SE-16 without rewriting the JSONL.

**`[meta] status_file`.** The current hand-maintained Status line is about 1,200 words of editorial narrative. The render engine generates a structured summary from step and question statuses. The editorial narrative cannot be fully derived; it captures the history of which clusters completed when and what the current priority is. Discarding it would lose real context for a resuming agent. Preserving it in `steps/_status-narrative.md` keeps it as hand-authored prose (exactly what prose sidecars are for) and avoids data loss. During migration, the current Status line text is copied verbatim to this file. Over time it can be pruned or replaced; but it is never lost.

**`[[question]]` `folded_into` as a separate field.** The `decided -> folded into <slug>` parametric prefix is a string convention parsed by prefix-match in `src/plan.rs` (QUEUE_FOLD_PREFIX, SE-11). Under TOML, `status = "decided"` and `folded_into = "<slug>"` are two separate structured fields. W4 reads `status == "decided"` items and checks for a matching `q_id` decision record in the JSONL, reusing the existing JSONL-side check without changing it. The `folded_into` field closes SE-11 (the prefix-match hack) without any change to the JSONL.

**Principles stay in the TOML, not the JSONL.** Principles do not change mid-task and have no event-sourcing story. They are plan metadata, correctly structured in `[[principle]]` entries. The render engine numbers them and inlines them in the Project Principles section, closing B8 (plan principles vs AGENTS.md drift) once the TOML is the source.

### What the TOML does NOT hold

Waivers, decisions, baselines, escalations, and round records: all already in the JSONL. Step dependencies (`blocked_by`): optionally added later; not needed for the migration. The `step_id`/`increment_id` structured round fields (replacing the `-inc<x>` strip): added to NEW JSONL records post-migration; old records keep the compat shim in `leading_slug`. No `[[increment]]` table: increments are a JSONL-side concept identified by the `task` field convention; the TOML knows only steps.

---

## Area 2: Prose sidecars

### Directory structure

```
docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.md              (generated; GENERATED banner; never hand-edited)
docs/plans/agent-scaffold.steps/
    _status-narrative.md                  (the current hand-authored Status line prose)
    core-assets.md
    file-dropper.md
    idempotency-safety.md
    ... (one file per step, ~49 total)
    structured-skeleton.md
docs/plans/agent-scaffold.questions/
    Q-1.md
    Q-2.md
    ... (one file per question with prose body, ~45 total)
```

Steps with very short or empty detail bodies still get a file (it may be one sentence or a placeholder). The question body files hold the prose narrative that currently lives after the one-line ask in each queue item: the extended rationale, option elaboration, trade-off analysis. `Q-43` (superseded) and `Q-42` (decided/resolved) still get question sidecar files; `superseded` status does not delete their prose. A superseded question's prose is historical evidence; it should be kept, not discarded.

### No-round-trip guarantee

The render engine reads sidecar files and writes the generated `.md`. It never reads the generated `.md` and never writes sidecar files. This is a strict one-way pipeline: TOML + sidecars -> generated `.md`. There is no parser that reads the generated `.md` back to update the TOML or the sidecars. This is the key safety property: a hand-edit to the generated `.md` is overwritten on the next render, but the sidecar files and the TOML are never clobbered by a render run.

The sidecars are plain Markdown with no frontmatter. The render engine includes them by file path reference in the TOML's `detail_file` and `body_file` fields. The tool never parses sidecar Markdown for structured data; it inlines the bytes verbatim into the rendered output (after ensuring a trailing newline). If a sidecar contains a Markdown heading, it appears in the rendered output as-is; the render engine does not re-structure prose content.

### Migration-period sidecar state

During the migration's shadow phase (before cutover), sidecar files exist alongside the hand-maintained `.md`. A reviewer or agent who wants to edit a step's prose detail can either: (a) edit the sidecar file directly (if they know about it), or (b) continue editing the hand-maintained `.md` (the `.md` is still the primary source during shadow phase). Both are valid during the shadow phase; the comparison run catches any divergence between the sidecar-based render and the hand-maintained `.md`. After cutover, editing the `.md` is a protocol violation (it is a generated artifact); all prose edits go to the sidecars.

---

## Area 3: The `agent-scaffold render` engine

### Input, output, and the GENERATED banner

`agent-scaffold render [--plan <task>.plan.toml] [--output <path>]` reads the TOML schema and inlines the sidecar prose files to produce the generated Markdown. The output file always begins with:

```
<!-- GENERATED: do not hand-edit this file. -->
<!-- Source: <task>.plan.toml + <task>.steps/ + <task>.questions/ -->
<!-- Regenerate with: agent-scaffold render -->
```

The render engine exits non-zero if a referenced sidecar file is missing or the TOML schema is invalid. It does not produce partial output; a broken source must not produce a half-rendered plan that an agent reads as authoritative.

The generated plan sections, in order:

1. The GENERATED banner (3-line comment block)
2. Plan title (from `[meta] title`)
3. Status line: "Status: in progress; N complete, M not-started, K next, ..." (computed from step status distribution) followed by the content of `status_file` if declared and present.
4. Plan motivation prose (from a `motivation_file` in meta, or a fixed sidecar `steps/_motivation.md`).
5. Project Principles: numbered list from `[[principle]]` entries in declaration order.
6. Documentation Protocol: rendered from a fixed internal template fragment, with the status vocabulary generated from the code constants (closing B3).
7. Repository Layout: rendered from an optional `steps/_repo-layout.md` sidecar.
8. Open Questions queue: one list item per `[[question]]` entry, showing `id`, status (with `folded_into` rendered as "decided -> folded into `<slug>`" for backwards-compatible display), and the `ask` field. Body prose from `body_file` is included as an indented or quoted block after the ask line.
9. Roadmap table: rendered in ascending `order` from `[[step]]` entries.
10. Step Details: one block per step in order, headed by "### `<slug>`: <title>", with content from `detail_file` inlined.

### How `validate`, `status`, and `next` read the new source

The parser in `src/plan.rs` gains a `parse_toml` path alongside the existing `parse_roadmap` and `parse_questions` Markdown paths. The dispatch is:

```
fn load_plan(path: &Path) -> Result<PlanSource> {
    let toml_path = path.with_extension("plan.toml");
    if toml_path.exists() {
        let meta = read_toml_meta(&toml_path)?;
        if meta.primary == "toml" {
            return Ok(PlanSource::Toml(parse_toml(&toml_path)?));
        }
    }
    // Fallback: read the Markdown plan as today.
    Ok(PlanSource::Markdown(read_plan_markdown(path)?))
}
```

`validate`, `status`, `status --json`, and `next` all go through `load_plan`. They never need to know whether the data came from TOML or from Markdown; `PlanSource` projects to the same `Vec<Step>` and `Vec<Question>` that the existing code already handles. W3/W4/W5 are unchanged; they read from the projected types, not from the raw source format.

The implication: before cutover (`primary = "markdown"`), the repo behaves identically to today. After cutover (`primary = "toml"`), the same checks run against the TOML source. The migration is transparent to the enforcement layer.

### The staleness check

`agent-scaffold render` computes a SHA-256 of the generated output and writes it to `[meta] render_hash` in the TOML (using an in-place edit of the TOML field only, not a full rewrite). `validate --source` recomputes the hash from the current TOML + sidecars and compares it to `render_hash`. A mismatch means the committed `.md` is stale (a source file was edited without re-rendering).

During migration, the staleness check is a WARNING, not a hard failure (the `.md` is still hand-maintained; a stale render hash is expected). After cutover, `validate --source --strict` treats a stale hash as a hard failure. The CI check runs `validate --source --strict` only after cutover; before cutover it runs `validate --plan` as today.

A git pre-commit hook that runs `agent-scaffold render && git add docs/plans/<task>.md` is the correct enforcement mechanism post-cutover. The hook is opt-in (a developer command, not a scaffolded hook). The CI staleness check is the safety net if the hook is not installed.

---

## Area 4: The migration path for this repo's dogfooded artifacts

### What must be migrated and why it is hard

The dogfooded plan is the hardest migration target in the ecosystem: it is the largest plan (51 steps, 45 questions, 113 JSONL records), it is actively being modified during the migration itself, and it is its own validation subject (the plan describes the structured-skeleton step, which is in progress during the migration). Any migration procedure that fails or corrupts this plan leaves the project in a worse state than before option B was chosen.

The 113 JSONL records (93 round, 16 waiver, 2 escalation, 1 decision, 1 baseline) do NOT need migration. The JSONL is append-only and already correct. W3/W4/W5 continue to read it unchanged. The JSONL is a non-problem.

The prose challenge: the current Status line is approximately 1,200 words in one paragraph. The step detail blocks total approximately 30,000 words across 49 blocks. The question body prose is approximately 15,000 words across 45 items. This prose must be extracted LOSSLESSLY from the hand-maintained `.md` and written to sidecar files. A lossy extraction (wrong block boundary, cut-off prose, dropped Unicode) would be a real data-loss event. The extraction must be verified.

### Cutover sequence (six stages with gates)

**Stage 0: Baseline validation.** Gate: `agent-scaffold validate --plan docs/plans/agent-scaffold.md --workflow` passes GREEN. Record the baseline output (step count, question count, waiver count, round count) as a reference.

**Stage 1: Parser fallback + render engine (no plan changes).** Build `parse_toml` in `src/plan.rs` and the `render` subcommand. The dogfooded plan is NOT modified. `agent-scaffold.plan.toml` does not exist yet. Tests for the new code use a synthetic small plan (3-5 steps, 2-3 questions) kept in a test fixture. The existing drift-guard tests still pass; the Markdown parsers are untouched. Gate: all existing tests pass; `cargo clippy --all-targets -- -D warnings` clean; `agent-scaffold validate --plan docs/plans/agent-scaffold.md --workflow` still GREEN (identical to Stage 0 baseline). No change to any committed plan file.

**Stage 2: TOML draft (structured fields only, shadow mode).** Create `docs/plans/agent-scaffold.plan.toml` from the existing Roadmap and queue data. The structured fields are already parseable: `parse_roadmap` yields the step slugs and statuses; `parse_questions` yields the question ids, statuses, and asks. A migration script reads these projections and writes the TOML `[[step]]` and `[[question]]` entries. Waivers, decisions, and principles are added manually (principles: 8 entries; waivers: already in JSONL, so none needed in TOML; decisions: none needed in TOML). The `[meta] primary` field is set to `"markdown"` so the parser still reads the `.md`. No sidecar files yet; `detail_file` and `body_file` fields are absent. Gate (three checks, all must pass): (a) `validate --source` on the new TOML reports 0 schema errors; (b) `validate --plan docs/plans/agent-scaffold.md --workflow` still GREEN (the `.md` is still the primary source; the TOML is inert); (c) the step count and question count in the TOML match the `.md` parse exactly (a simple script cross-checks them). Abandonment test: delete `docs/plans/agent-scaffold.plan.toml` and all tests pass unchanged. The TOML is purely additive at this stage.

**Stage 3: Prose sidecar extraction.** Extract step detail prose to `docs/plans/agent-scaffold.steps/<slug>.md` and question body prose to `docs/plans/agent-scaffold.questions/<id>.md`. The extraction is semi-automated: a script identifies the section boundaries (a `### \`<slug>\`: `heading starts a step block; the next`### `heading ends it) and writes each block's body to a file. Special case: the Status line is copied verbatim to`docs/plans/agent-scaffold.steps/\_status-narrative.md`. The plan motivation prose (after the Status line, before the Project Principles heading) goes to `docs/plans/agent-scaffold.steps/\_motivation.md`. The Documentation Protocol section goes to a fixed template fragment (not a sidecar; the render engine generates it from code constants). Add `detail_file`and`body_file`pointers to the TOML entries. The`[meta] primary`field remains`"markdown"`. Gate: (a) every step slug has a corresponding non-empty `docs/plans/agent-scaffold.steps/<slug>.md`file; (b) every question id with substantive prose has a corresponding non-empty`docs/plans/agent-scaffold.questions/<id>.md`file; (c)`validate --plan docs/plans/agent-scaffold.md --workflow`still GREEN; (d) a word-count comparison between the`.md`step detail section (lines 185 to end) and the sum of the sidecar files shows no more than 1% discrepancy (accounting for markdown heading lines removed from the sidecar bodies). Abandonment test: delete the`agent-scaffold.steps/`and`agent-scaffold.questions/` directories. Tests pass unchanged.

**Stage 4: Shadow render + comparison.** Run `agent-scaffold render --plan docs/plans/agent-scaffold.plan.toml --output docs/plans/agent-scaffold.md.rendered`. This file is NOT committed; it is a local comparison artifact. Run a diff between `agent-scaffold.md.rendered` and `agent-scaffold.md` (the hand-maintained source). Expected differences at this stage: the GENERATED banner (3 lines added), the computed Status summary line (different from the hand-authored paragraph), section ordering (if any), and any formatting normalization the render engine applies. Unexpected differences: missing prose blocks, wrong block content, truncated question bodies. Iterate: for each unexpected difference, either fix the extraction (Stage 3) or fix the render engine output. Continue until the diff shows only the expected differences. Gate: the diff between `agent-scaffold.md.rendered` and `agent-scaffold.md` contains NO unexpected differences. Every step detail block appears in the rendered output. Every question body appears in the rendered output. The Roadmap table in the rendered output matches the Roadmap table in the hand-maintained `.md` exactly (column-by-column comparison). Note: the expected differences (GENERATED banner, computed Status summary) are documented explicitly so reviewers can verify they are intentional, not bugs.

**Stage 5: Cutover commit.** In a single atomic commit: (a) set `[meta] primary = "toml"` in `agent-scaffold.plan.toml`; (b) run `agent-scaffold render --plan docs/plans/agent-scaffold.plan.toml --output docs/plans/agent-scaffold.md` to write the generated plan over the hand-maintained one; (c) commit the TOML, all sidecar files, and the new generated `agent-scaffold.md` together. Gate (run before committing): `agent-scaffold validate --plan docs/plans/agent-scaffold.plan.toml --workflow` GREEN (now reads from TOML); `validate --source` GREEN; the generated `agent-scaffold.md` starts with the GENERATED banner; `agent-scaffold next` returns the correct next step. The hand-maintained `.md` is not deleted from git history; it is preserved as a committed artifact before this commit. If the cutover commit breaks something, `git revert` restores the hand-maintained `.md` and a second `git revert` deletes the TOML/sidecars; the repo returns to Stage 0.

**Stage 6: Post-cutover enforcement.** Add the pre-commit hook and CI staleness check. Update `AGENTS.md` (or the scaffolded workflow) to document: "do not hand-edit `<task>.md`; it is a generated artifact; edit the TOML and run `agent-scaffold render` before committing." Update `pack/plan-template.md` to be replaced by `pack/plan-template.toml` and starter sidecar directories. Update the pack scaffold story. Gate: a deliberate hand-edit to `agent-scaffold.md` followed by `validate --source --strict` produces a non-zero exit code (staleness detected); `agent-scaffold render` followed by `validate --source --strict` produces a zero exit code.

### Why this sequence is safe

The sequence never has a period where `validate --workflow` fails or returns inconsistent results. The TOML is additive-only through Stage 4; the `primary = "markdown"` flag keeps the `.md` as the source until the cutover commit. Each stage has an explicit abandonment test: delete the additive artifact, tests pass unchanged.

The riskiest step is Stage 3 (prose extraction). A bug in the extraction script could truncate a step detail or question body. The gate check (word-count comparison, non-empty sidecar check, and the Stage 4 shadow render comparison) catches extraction bugs before the cutover. The hand-maintained `.md` remains the ground truth through Stage 4, so a bad extraction is caught against a known-good source.

### JSONL records and the TOML migration

The 113 JSONL records do not need migration. The existing `round`, `waiver`, `escalation`, `decision`, and `baseline` records are correct and will continue to be read by W3/W4/W5 unchanged. New records written after the cutover can include `step_id` and `increment_id` fields (closing SE-10/B6 for future rounds), with `leading_slug` as the compat shim for old records. The `orphan_tasks` list in `[meta]` handles the five orphan task slugs explicitly.

### The dogfooded plan migration as a validation of the migration tooling

An alternative migration order could run Stage 3 on a SYNTHETIC plan first (a small pilot plan with 5 steps and 3 questions), verify the extraction and render tooling against it, and THEN apply it to the dogfooded plan. This costs one extra commit round but provides confidence that the extraction script and render engine are correct before touching the large plan. Given the stakes (the dogfooded plan is the primary ongoing work context), this is the recommended order. The synthetic pilot plan is a throw-away test fixture that can be deleted after the cutover.

---

## Area 5: Staged, reviewable roadmap

Each increment is independently shippable and reviewable under the role-separated loop. Increments 1-3 do not touch the dogfooded plan; they build and validate the infrastructure. Increment 4 runs the migration. Increment 5 enforces the new model.

**Increment 1: Parser fallback + TOML schema validation (risky)** Scope: add `parse_toml` to `src/plan.rs`; add the `PlanSource` enum dispatch with `primary = "toml"` / `"markdown"` switch; add `validate --source` mode that checks the TOML schema (required fields, status vocabulary, cross-references between `detail_file`/`body_file` pointers and actual files); add the synthetic test-fixture plan; add `[meta] orphan_tasks` as a named field that `validate --source` reports. No render engine yet. No change to the dogfooded plan. Acceptance check: `validate --plan docs/plans/agent-scaffold.md --workflow` still GREEN; `validate --source` on the synthetic fixture plan reports 0 errors; all existing tests pass; a deliberate schema error in the fixture plan produces a non-zero exit code. Risk class: risky (touches `src/plan.rs`, which is load-bearing for all three checks). Requires two consecutive clean rounds.

**Increment 2: Render engine (risky)** Scope: implement `agent-scaffold render`; the render pipeline (TOML + sidecars -> Markdown); the GENERATED banner; the computed Status summary; the Roadmap table, Open Questions, and Step Details sections; the `[meta] render_hash` staleness mechanism; `validate --source --strict` for the staleness check. Test against the synthetic fixture plan: render it, compare the output against a golden file committed in the test fixture. Acceptance check: `agent-scaffold render` on the synthetic fixture plan produces the expected output (golden file comparison); `validate --source --strict` passes after a render; `validate --source --strict` fails after a deliberate hand-edit to the rendered output; dogfooded plan's `validate --workflow` still GREEN. Risk class: risky (new subcommand; render path cannot be best-effort for a load-bearing derived artifact). Requires two consecutive clean rounds.

**Increment 3: TOML draft + sidecar extraction for the dogfooded plan (low risk)** Scope: create `docs/plans/agent-scaffold.plan.toml` from the dogfooded plan (Stage 2 + Stage 3 of the migration sequence); all sidecar files; `[meta] primary = "markdown"` (no cutover yet); `validate --source` passes. Commit the TOML and sidecars. The `.md` is still the primary source. Acceptance check: Stage 2 gates (TOML schema 0 errors; `.md` source still GREEN; step/question counts match); Stage 3 gates (all sidecars non-empty; word-count comparison within 1%); Stage 4 shadow render shows only expected differences; `validate --workflow` GREEN reading from `.md`. Risk class: low risk (no code changes; data migration only; the `.md` is still the primary source; the whole increment is reversible by deleting the TOML and sidecars). Requires one clean round.

**Increment 4: Cutover (low risk)** Scope: set `[meta] primary = "toml"`; run `agent-scaffold render` to write the generated `agent-scaffold.md`; commit the cutover state. Update `AGENTS.md` to document the generated-file rule. Acceptance check: Stage 5 gates (TOML-source `validate --workflow` GREEN; `agent-scaffold next` returns correct step; GENERATED banner present); Stage 6 gate (staleness detection works); `git revert` on the cutover commit restores the hand-maintained `.md` and `validate --workflow` GREEN. Risk class: low risk (a `git revert` returns the repo to a fully working state). Requires one clean round.

**Increment 5: Pack template and scaffold story update (risky)** Scope: replace `pack/plan-template.md` with `pack/plan-template.toml` and starter sidecar directories; update the scaffold subcommand to drop the TOML template and empty sidecar directories; update the drift-guard tests (which currently pin `pack/plan-template.md`) to pin `pack/plan-template.toml`; update the `AGENTS.md` documentation for the new three-file editing story; update `pack/instrument.md` if the schema documentation needs updating. Acceptance check: `agent-scaffold scaffold` (or equivalent) on a fresh directory produces a valid `plan.toml`, empty sidecar directories, and a rendered `plan.md` with the GENERATED banner; `validate --source` on the freshly scaffolded plan passes; the drift-guard tests pass; the existing dogfooded plan's `validate --workflow` still GREEN. Risk class: risky (changes the output that every scaffolded project inherits; the pack template is load-bearing for the scaffolded project story). Requires two consecutive clean rounds.

---

## Enforcement preservation at each stage

The following table shows which source each check reads at each stage and whether the check stays GREEN.

| Stage | W3 reads | W4 reads | W5 reads | validate GREEN? |
| --- | --- | --- | --- | --- |
| 0 (baseline) | .md plan + JSONL | .md queue + JSONL | JSONL | Yes |
| 1 (parser fallback built) | .md (primary="markdown") | .md | JSONL | Yes |
| 2 (TOML draft, primary="markdown") | .md | .md | JSONL | Yes |
| 3 (sidecars added, primary="markdown") | .md | .md | JSONL | Yes |
| 4 (shadow render, primary="markdown") | .md | .md | JSONL | Yes |
| 5 (cutover, primary="toml") | TOML + JSONL | TOML + JSONL | JSONL | Yes |
| 6 (post-cutover enforcement) | TOML + JSONL | TOML + JSONL | JSONL | Yes |

The pause.md catch (a `complete` step with no round records and no waiver fails W3) is preserved at every stage. Before cutover, W3 reads from the `.md` Roadmap exactly as today. After cutover, W3 reads from the TOML `[[step]]` entries, which are validated to match the `.md` Roadmap (Stage 2 gate). A step that was `complete` with a covering waiver in the JSONL at Stage 0 is `complete` with the same covering waiver in the JSONL at Stage 5; the waiver did not move and was not re-declared. The evidence tiers (self-declared vs record-backed) are untouched; W5 continues to validate them against the same JSONL escalation records.

The round-log consistency check is JSONL-only and is not affected by any stage of the migration.

W4 (decided items need decision receipts) transitions from reading the `QUEUE_FOLD_PREFIX` parametric string from the `.md` queue to reading `status == "decided"` items from the TOML queue at cutover. The set of items flagged by W4 is identical (the same 45 decided items; the single `type:"decision"` record for Q-45 covers the only item past the W4 baseline). Stage 2 gate explicitly checks that the TOML question count and decided-item count match the `.md` parse, so W4 can only change its read path, not its results.

---

## Trade-offs specific to this lens

**Against:** The `primary` field adds schema complexity. A cleaner design would use the presence/absence of the TOML as the sole cutover signal (if `plan.toml` exists, use it; otherwise use `.md`). The migration-safety argument for the explicit flag is: a PARTIAL TOML (all steps but no sidecars, or a TOML whose render has not been validated) should not automatically become the primary source. The `primary` flag decouples "the TOML exists" from "the TOML is ready to be the primary source."

**Against:** The shadow render comparison (Stage 4) requires running a tool that does not yet exist (`agent-scaffold render`) and comparing its output against the hand-maintained source. This creates a temporal dependency: the render engine (Increment 2) must ship before the dogfooded plan migration (Increment 3) can complete its Stage 4 gate. The migration can start (Stage 2-3 of Increment 3 can happen before Increment 2 ships), but cannot complete until Increment 2 ships. This is a real sequencing constraint but not a design flaw; the render engine is required by option B regardless.

**For:** The `primary = "markdown"` default is the safest possible default for a dogfooded system. A new scaffolded project starts with `primary = "markdown"` and migrates by updating the flag. An existing project never accidentally becomes broken by the presence of a partial TOML.

**For:** Keeping waivers and decisions in the JSONL (not adding them to the TOML) is both architecturally cleaner and migration-safer. The JSONL is the append-only event log; the TOML is the plan skeleton. Adding waivers to the TOML would re-introduce the two-homes smell the pilots fixed and add a migration step (moving 16 waiver records from JSONL to TOML) with no enforcement benefit (W5 already reads waivers from the JSONL and will continue to do so).

**For:** The staged roadmap puts the riskiest conversion (the dogfooded plan itself) after the infrastructure is built and validated on a synthetic plan. Increment 3 runs only after Increments 1 and 2 are complete and reviewed. Each increment has a clear abandonment path.

---

## Open questions for the synthesis

1. **The `_status-narrative.md` sidecar and the Status line.** The migration-safety case for preserving the current Status line prose is clear (it is the primary resume anchor for a continuing agent). But should it be preserved indefinitely or pruned over time? The synthesis should decide: is the rendered Status summary (computed) sufficient for the resume use case, or does the narrative sidecar have ongoing value? If the RESUME STATE in the ledger is the true resume anchor post-cutover, the narrative sidecar may be retired once the ledger RESUME STATE is kept current.

2. **The `step_id`/`increment_id` structured round fields.** New JSONL round records after the cutover should carry these fields (closing SE-10/B6 for future rounds). This is a JSONL-side addition (not a TOML schema change) and can be a separate increment after the cutover. The synthesis should confirm this is out of scope for the `structured-skeleton` step itself.

3. **The `pack/plan-template.md` blast radius.** Increment 5 changes the scaffolded output that every new project inherits. Projects already on the Markdown format continue to work (the `.md` fallback is permanent). But the pack template change means a fresh `agent-scaffold scaffold` run produces a TOML-first project by default. Is this the right default for greenfield projects (which have no migration cost and benefit immediately from option B), or should it be opt-in? The synthesis should decide: does Increment 5 change the scaffolded default unconditionally, or add a flag?

4. **The synthetic pilot plan.** This document recommends creating a small synthetic plan in a test fixture to validate the extraction and render tooling before touching the dogfooded plan. Is a synthetic plan sufficient, or should the test fixture also include the dogfooded plan's full complexity (many steps, long prose sidecars, orphan tasks)? A larger fixture is more representative but also more expensive to maintain.
