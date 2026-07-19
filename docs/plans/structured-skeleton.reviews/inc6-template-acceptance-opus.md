# Inc 6 review: template coherence, acceptance criteria, self-scaffold regen (opus, read-only)

Lens: TEMPLATE COHERENCE, ACCEPTANCE CRITERIA, and the SELF-SCAFFOLD REGEN. Reviewed adversarially against the `structured-skeleton-inc6` acceptance clause in `docs/plans/agent-scaffold.steps/structured-skeleton.md`. Read and tested inside the worktree `.claude/worktrees/inc6` at branch `impl/structured-skeleton-inc6` (commits `f926b5b`, `88e7d8a`, `7f20bfc`, `b1f993a`).

## Verdict

Zero findings (no critical, no high, no medium, no low). Every acceptance criterion in the Inc 6 clause was performed and passed, the template renders into a coherent starter plan with a truthful do-not-hand-edit banner, and the self-scaffold regen produced `deployed == pack` while leaving the live plan untouched. Details and evidence below.

## Acceptance criteria: performed and passing

Fresh scaffold into a temp dir with the acceptance command
(`cargo run -- scaffold --output-dir <tmp> --write --principles default --instrument`,
run against the built worktree binary):

- Drops a coherent `docs/plans/TEMPLATE.plan.toml` skeleton, a rendered `docs/plans/TEMPLATE.md` projection, the starter sidecars, and the `TEMPLATE.steps/` + `TEMPLATE.questions/` directories (each with a `.gitkeep`). The scaffold ran an initial `render` (its output line `render  docs/plans/TEMPLATE.md`), so the generated view ships beside its source.
- `validate --source <tmp>/docs/plans/TEMPLATE.plan.toml` -> `1 steps, 0 questions, valid`, exit 0.
- `validate --workflow --source <tmp>/docs/plans/TEMPLATE.plan.toml --metrics <present-empty>.jsonl` with NO `--plan` -> `workflow invariants hold`, exit 0. This is the Inc 6 clap relaxation, and it is also pinned by the new integration test `tests/validate_workflow_toml_source_needs_no_plan.rs` (passes).
- `render docs/plans/TEMPLATE.plan.toml --check --strict` on the dropped template -> `up to date`, exit 0 (the scaffold's initial render is byte-stable).
- No active `w4_baseline`: the `[meta]` block declares only `title`, `primary = "toml"`, and `[meta.sidecars]`. `grep w4_baseline` on the dropped TOML matches ONLY the explanatory comment (`pack/plan-template.plan.toml:10`), so a fresh project has no baseline and W4 requires a receipt for every decision it later folds in. Confirmed against the acceptance wording exactly.

## Template coherence

`pack/plan-template.plan.toml` is minimal but coherent: `[meta]` has `primary = "toml"` and no active `w4_baseline`; one seeded `[[principle]]`; one placeholder `example-step` `[[step]]` (`status = "not-started"`, `order = 1`); an empty queue (a comment where `[[question]]` would go); and the `[[step.waiver]]` schema present-but-commented (available, unused). The `[meta.sidecars]` front/tail lists resolve to the shipped sidecars, all of which render into a coherent starter plan (front prose, generated Project Principles, the generated Roadmap Status Vocabulary fragment, empty Open Questions, the Roadmap table with the one placeholder step, Step Details from `example-step.md`, and the Success Criteria tail).

The do-not-hand-edit banner is present and truthful in the rendered output:
`<!-- GENERATED FILE - do not hand-edit. Source: TEMPLATE.plan.toml, TEMPLATE.steps/, TEMPLATE.questions/, and the [meta].sidecars prose (front/tail). Regenerate with agent-scaffold render TEMPLATE.plan.toml; hand edits are overwritten and caught by agent-scaffold render --check. -->`.
It names the real sources and both guards (`render` and `render --check`).

## Regen (b1f993a): deployed == pack, live plan untouched

- `b1f993a` changed ONLY the expected files: root `AGENTS.md`, the four verbatim root reference prompts + `kickoff.md`, root `.agents/AGENTS.reference.md`, and the dropped `docs/plans/TEMPLATE.*` render/scaffold artifacts. It did NOT touch `docs/plans/agent-scaffold.plan.toml` or any `agent-scaffold.*` sidecar (the template drops to the `TEMPLATE.*` namespace, a different namespace from the live plan, so `--force` did not clobber it). Verified by `git show --stat b1f993a`.
- `deployed == pack` byte-for-byte: every verbatim reference prompt and every `docs/plans/TEMPLATE.*` file is `diff`-identical to its `pack/` source, and the two RENDERED files (`AGENTS.md`, `.agents/AGENTS.reference.md`) are byte-identical to a fresh `--principles default --instrument` scaffold (the `scaffold-self` flags).
- `render --check --strict` is clean on both the committed `docs/plans/TEMPLATE.plan.toml` (`up to date`) and the live `docs/plans/agent-scaffold.plan.toml` (`up to date`), so the regen did not leave a stale generated view and the live plan still renders green.

## Doc/prompt currency (Q-47/Q-48)

The Inc 6 doc-currency edits are done and correct:

- `pack/AGENTS.md` phase-2 rewritten to the TOML skeleton + sidecars + `render` authoring/editing flow; the line-57 Tracking-progress SSOT rewritten to name `[[step]].status` as the status source of truth and the Roadmap table as a `render`-overwritten projection.
- `pack/prompts/planner.md` and `pack/user-prompts/kickoff.md` now start a plan from `docs/plans/TEMPLATE.plan.toml` + sidecars + `render` (no `TEMPLATE.md`-authoring language).
- `pack/prompts/implementer.md` and `pack/prompts/orchestrator.md` status-SSOT lines corrected to "edit `[[step]].status` in the `.plan.toml` and re-render; never hand-edit the generated Roadmap table".
- No stale `TEMPLATE.md` / `plan-template.md` / Markdown-only-plan authoring references remain in `pack/prompts/` or `pack/user-prompts/kickoff.md`. The only surviving `TEMPLATE.md` mentions anywhere (`pack/pack.toml:36`, `README.md:24`) correctly describe it as the GENERATED view, not a hand-authored source.
- `README.md` command reference covers `render`, `render --check` (and `--strict`), and `validate --source` (README lines 191-233).
- The `plan_template_documents_every_accepted_status` drift guard was DROPPED (`88e7d8a`, `src/plan.rs`), which the acceptance clause explicitly sanctions ("Migrate or drop"): the template no longer hardcodes the status vocabulary, and `render` now generates it from the code constants (the rendered "Generated status vocabulary (from the code constants, so it cannot drift)" fragment), so the drift the guard protected against is now structurally impossible and there is nothing left to guard.

## Tests

Full `cargo test` in the worktree: all binaries pass, 0 failed (292 lib/bin unit tests + the integration suites, including the three scaffold/precommit, `validate_toml_primary_skips_markdown_plan`, and the new `validate_workflow_toml_source_needs_no_plan`).

## Non-findings (noted, not raised)

- The `docs/plans/TEMPLATE.md` treefmt-excludes item is the known open item the orchestrator will handle at merge; `render --check --strict` on the committed artifact is currently clean, so there is no present correctness problem, only a merge-time formatter-policy call. Not re-raised.
- The derived Status line renders `1 steps (1 not started)` (no singular/plural agreement). This is pre-existing Inc 3 render-engine behaviour, not introduced or in scope for Inc 6, and purely cosmetic. Not raised as a finding.
