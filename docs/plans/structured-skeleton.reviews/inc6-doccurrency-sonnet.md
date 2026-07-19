# Inc 6 review: DOCUMENTATION / PROMPT CURRENCY and SINGLE-SOURCING

Reviewer lens: doc/prompt currency and single-sourcing. Scope: the doc sweep in commit `7f20bfc` (`pack/AGENTS.md`, `pack/prompts/`, `pack/user-prompts/`, `README.md`) reviewed in the worktree at `.claude/worktrees/inc6` against `git diff 593f8c9..impl/structured-skeleton-inc6`. Code, template, and regen commits (`f926b5b`/`88e7d8a`/`b1f993a`) and the regenerated root `AGENTS.md`/`.agents/*` copies are out of scope (other reviewers own them).

## Result: zero findings

No `low`, `medium`, `high`, or `critical` findings. The sweep is complete, accurate, and internally consistent for the doc-currency / single-sourcing lens. The record of what was checked is below.

## Verification record

### 1. Fidelity + completeness (plan-authoring currency)

All in-scope locations that named the old Markdown-plan / `TEMPLATE.md` authoring flow were updated:

- `pack/AGENTS.md:30` (phase 2) now describes the TOML skeleton + prose sidecars + `agent-scaffold render` + `render --check`-before-commit flow directly (Q-48), replacing "draft a plan ... from `docs/plans/TEMPLATE.md`".
- `pack/prompts/planner.md:5` moved from "copied from `docs/plans/TEMPLATE.md`" to copying `TEMPLATE.plan.toml` + sidecars, editing the TOML and sidecars, then `agent-scaffold render`, and references the phase-2 flow in `AGENTS.md`.
- `pack/user-prompts/kickoff.md:7` moved from "start one from `docs/plans/TEMPLATE.md`" to "start one from the TOML plan skeleton `docs/plans/TEMPLATE.plan.toml` and its sidecars".

Straggler greps over the worktree `pack/` came back clean:

- `TEMPLATE.md`: the only hits (`pack/pack.toml:36`, `README.md`, and the AGENTS/prompt prose) refer to the GENERATED view, never to authoring from it. No residual "author a plan from `TEMPLATE.md`" instruction remains.
- `Status cell`: no hits anywhere in `pack/` (the stale `implementer.md` phrase was removed).
- `Roadmap table`: every remaining hit correctly frames the generated table as a `render`-overwritten projection that must not be hand-edited (`AGENTS.md:57`, `implementer.md:5`, `orchestrator.md:31`, and the template's `roadmap-intro.md` lead-in).
- `single source of truth for`: the two hits (`AGENTS.md:57`, template `plan-template.plan.toml:2`) both attribute the SSOT to the TOML structured source, not the Markdown table.

The abstract "Roadmap" references that remain (`orchestrator.md:23` "mark it complete", `orchestrator.md:27` "Roadmap scope", and the untouched `AGENTS.md` phase-4 "mark the step complete in the Roadmap") are the conceptual Roadmap (pending-step tracking, scope), not instructions to hand-edit a Markdown status table. These are the acceptable non-overcorrected phrasing (the abstract Roadmap remains the conceptual status home), so their being left as-is is correct, not a miss.

### 2. Status-maintenance correctness (R-1)

All three named locations correctly name `[[step]].status` in the `.plan.toml` as the status SSOT, with edit-plus-re-render and a never-hand-edit-the-generated-table instruction:

- `pack/prompts/implementer.md:5`: "the source of truth for status is each `[[step]].status` in the `<task>.plan.toml`, so set the step's status there and re-render ... Never hand-edit the generated Roadmap table in `<task>.md`: render overwrites it, and `agent-scaffold render --check` catches a stale or hand-edited view."
- `pack/prompts/orchestrator.md:31`: "the status source of truth is each `[[step]].status` in the `<task>.plan.toml`, and the Roadmap table in the generated `<task>.md` is a projection of it that is never hand-edited".
- `pack/AGENTS.md:57`: "each step's status is the `[[step]].status` field in the `<task>.plan.toml`, the single source of truth for step status, and the Roadmap table in the generated `<task>.md` is a projection of it that `render` overwrites (never hand-edited). The implementer keeps the status current by editing `[[step]].status` and re-rendering."

The correction is accurate and not overcorrecting: it distinguishes the SSOT (`[[step]].status`) from the GENERATED table (never hand-edited) while keeping the abstract Roadmap as the conceptual status home ("lives durably in the plan's structured source"). The cross-reference from `implementer.md:5` and `AGENTS.md` to "the plan's Documentation Protocol" points at consistent content: the template's `plan-template.documentation-protocol.md:3` also names `[[step]].status` as the SSOT and the generated view as never-hand-edited, so the reference does not resolve to stale Markdown-table wording.

### 3. Single-sourcing (Q-49)

No NEW verbatim copies of a canonical rule/constant were introduced:

- `planner.md` and `kickoff.md` REFERENCE `AGENTS.md` (phase 2 / "the plan-authoring and render flow is in `AGENTS.md`") for the authoring flow rather than restating it in full.
- `orchestrator.md:31` and `implementer.md:5` contain in-place CORRECTIONS of restatements that already existed pre-sweep (the old "the Roadmap is the single source of truth for step status" / "set the step's Status cell ... the single source of truth for status"). They replace a stale restatement with a current one; they do not add a new independent copy. R-1 explicitly required these two to be corrected, and the status-maintenance rule is operational to the orchestrator's and implementer's own roles.

### 4. README

- New "Rendering the plan" section documents `render`, `render --check`, and `render --check --strict`; the "Validating and projecting workflow state" section documents `validate --source`, `validate --source --workflow`, and `status --source` (plus `--json`). All match the actual CLI in `src/main.rs`: the `Render` subcommand with positional plan path, `check: bool`, and `strict: #[arg(requires = "check")]`; `ValidateArgs.source: Option<PathBuf>` and `workflow: bool` (the README correctly dropped the old "(which requires `--plan`)" qualifier, since `--workflow` no longer has a clap `requires` on `--plan` and a TOML-primary `--source` needs no `--plan`); `StatusArgs.source: Option<PathBuf>`.
- The layout listing (`README.md:20-24`) and the architecture paragraph (`README.md:46`) now describe the `TEMPLATE.plan.toml` skeleton plus sidecars projecting to a generated `TEMPLATE.md`, current with the render/TOML/sidecar architecture.
- The remaining "Markdown plan" mentions (`README.md:210`, `:222`, `:226`, `:232`) are all legitimately about the still-supported `--plan` path (they read "still works when a project keeps a Markdown plan" / "else from the Markdown `--plan`"), not stale authoring instructions.
- The deferred R-11 wording ("W3 states an absolute, omitting the waiver exemption") is correctly left untouched and is not treated as a miss here.

### 5. Style

No em-dash, en-dash, double-hyphen-as-dash, other unicode, or emoji in the changed doc prose (a `grep -P '[^\x00-\x7F]'` over all six changed files returned nothing; the `--flag` tokens are CLI flags, not dash substitutes). No hard-wrapped prose (paragraphs stay single-line). No AI/assistant references introduced.
