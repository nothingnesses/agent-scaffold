# Inc 6 Round 3 (holistic final-confirmation) review

Reviewer: independent read-only holistic pass (opus). Scope: the whole `structured-skeleton-inc6` increment on `impl/structured-skeleton-inc6` (worktree `.claude/worktrees/inc6`), diff range `e06a89b..HEAD` (six commits: template+render, `--workflow` relaxation + drift-guard drop + test, doc currency, self-scaffold regen, and the M-1/M-2 fix). Reviewed adversarially against the Inc 6 spec, the reviewer prompt, and the project principles.

Result: essentially clean. One LOW self-consistency finding (R3-1) on the generated `docs/plans/TEMPLATE.md` and the prettier exclude list; no high/critical/medium findings. The M-1 fix is present and correct, the shipped pack is internally coherent, and the acceptance scaffold + `cargo test` + `cargo clippy` are all green.

## Severity counts

- Critical: none.
- High: none.
- Medium: none.
- Low: 1 (R3-1).

## R3-1 (low): the generated `docs/plans/TEMPLATE.md` render artifact is not in the prettier exclude list, so `just fmt` / `just scaffold-self` are not idempotent on it

Evidence:

- `flake.nix:47-50` `settings.global.excludes = ["src/plan/testdata/render-fixture*", "docs/plans/agent-scaffold.md"]`. The comment above it (`flake.nix:41-46`) states the invariant explicitly: render artifacts "are byte-exact `agent-scaffold render` output ... so prettier/taplo must not touch them or `render --check` would diverge", and that "This project's own plan is now a render artifact too ... so prettier must not reflow it". Inc 6 turns `docs/plans/TEMPLATE.md` into a render artifact of exactly this class (generated from `TEMPLATE.plan.toml` + sidecars, do-not-hand-edit banner at `docs/plans/TEMPLATE.md:1`), but it was NOT added to the exclude list.
- Reproduction on clean HEAD in the worktree: `nix fmt` (equivalently `just fmt`, and the `nix fmt` tail of `just scaffold-self` at `justfile:48`) reflows the committed `docs/plans/TEMPLATE.md`: prettier repads the Roadmap GFM table (`| --- | --- |` -> `| -------------- | ----------- | ----- |`) and collapses a blank line after the Open Questions heading. So the committed `TEMPLATE.md` is the raw render form and is not prettier-clean; running the repo's own dogfood recipe leaves an uncommitted diff.
- This contradicts the `scaffold-self` recipe's own stated invariant (`justfile:43-45`: "it leaves the repo at a stable committed fixed point") for `TEMPLATE.md`, and the spirit of the Inc 6 acceptance clause "`just scaffold-self` regenerates ... clean so deployed == pack". Note the acceptance clause enumerates `.agents/` + `AGENTS.md` + `README.md` (not `TEMPLATE.md`), and those three ARE clean, so the literal enumerated criteria pass; this is the un-enumerated generated file.

Why it matters (and why LOW, not higher):

- No active gate fails today. `.agents/checks.toml` gates `render --check --strict` only on `docs/plans/agent-scaffold.plan.toml` (the live plan), not on `TEMPLATE.plan.toml`, and declares no `nix fmt --check` gate. `render --check` on `agent-scaffold.plan.toml` stays green, and `render --check` on the freshly scaffolded `TEMPLATE.plan.toml` passes (raw render matches the raw committed template). The repo already tolerates several non-prettier-clean hand-written markdown files at HEAD (the ledger and review/exploration docs also reflow under `nix fmt`), and there is no CI directory, so nothing is red now.
- The impact is a self-consistency / dogfooding nit plus a latent divergence: a contributor running `just fmt` or `just scaffold-self` gets a spurious `TEMPLATE.md` diff; if committed, `TEMPLATE.md` flips to the prettier form, and IF a `render --check` gate on `TEMPLATE.plan.toml` (a natural extension, mirroring `agent-scaffold.plan.toml`) or a `nix fmt --fail-on-change` gate (present as a commented row in `pack/checks.toml:65`) is ever added, the raw-render and prettier forms can no longer both be satisfied. This is precisely the render-vs-prettier divergence the exclude list was created to prevent (the same lesson already applied to `render-fixture*` and `agent-scaffold.md`).
- The underlying open point the author should settle: is `TEMPLATE.md` meant to be a byte-exact render artifact (add it to the `flake.nix` prettier excludes, like `agent-scaffold.md`, and keep it raw), or a fmt-normalized generated view (like the rendered `AGENTS.md`, in which case the committed `TEMPLATE.md` should be the post-`nix fmt` form, not the raw render currently committed)? It is currently in the inconsistent middle state: committed raw, but not excluded from prettier. Given the do-not-hand-edit banner and the `render --check` model applied to its sibling `agent-scaffold.md`, adding `docs/plans/TEMPLATE.md` to the `flake.nix` exclude list is the consistent fix.

## Confirmed sound (holistic pass, no findings)

- Integration coherence: the shipped pack hangs together. `pack/pack.toml` maps the new `plan-template.plan.toml` + all nine prose sidecars + `steps/example-step.md` + the two `.gitkeep` files to `docs/plans/TEMPLATE.*` (working, create-if-absent, verbatim); the deleted `pack/plan-template.md` -> `TEMPLATE.md` asset row is gone; `TEMPLATE.md` is produced by the post-drop `render` loop (`src/main.rs:1349-1375`), not shipped as an asset. The pack prompts/docs and the code now describe the same flow: `pack/prompts/{orchestrator,planner,implementer}.md`, `pack/user-prompts/kickoff.md`, `pack/AGENTS.md`, and `README.md` all move from "author a Markdown plan from `TEMPLATE.md`" to the TOML skeleton + sidecars + `agent-scaffold render` + never-hand-edit-the-generated-view flow. No active file (`src`, `pack`, `README.md`, `AGENTS.md`, `.agents`) references the deleted `pack/plan-template.md`; the only remaining `plan-template.md` hits are in historical exploration/review docs (archival, correctly untouched).
- No stale `TEMPLATE.md`-as-authoring references remain in `pack/prompts/` or `pack/user-prompts/kickoff.md`; the surviving `TEMPLATE.md` mentions in `README.md` and `pack/pack.toml` correctly describe it as the generated view.
- Acceptance re-confirmed by running it (temp dir, cleaned up):
  - `cargo run -- scaffold --output-dir <tmp> --write --principles default --instrument` drops a coherent `docs/plans/TEMPLATE.plan.toml` + the nine sidecars + `steps/example-step.md` + `steps/.gitkeep` + `questions/.gitkeep`, and renders `docs/plans/TEMPLATE.md` (with the do-not-hand-edit banner and the derived "1 steps (1 not started); 0 open questions; 0 waivers" status line).
  - `validate --source docs/plans/TEMPLATE.plan.toml` -> exit 0, "1 steps, 0 questions, valid".
  - `validate --source docs/plans/TEMPLATE.plan.toml --workflow` (no `--plan`) -> exit 0; with a present (empty) metrics log it reaches the check and prints "workflow invariants hold" (relaxation works end to end); with no metrics log it soft-skips (exit 0), the legitimate missing-log skip, not a false green.
  - `render --check` and `render --check --strict` on the scaffolded skeleton -> "up to date" (exit 0); a one-line hand-edit of the generated `TEMPLATE.md` makes `render --check --strict` exit 1 naming the drift.
  - No active `w4_baseline`: the template declares none (only a commented schema note at `pack/plan-template.plan.toml:10`), so a fresh project's decisions require a receipt, as specified.
- Fix sanity (M-1) present at HEAD: `src/main.rs:836-844` makes `--workflow` with no resolvable plan source a hard `problems.push` ("no plan source resolved") -> exit 1, instead of the old silent green. Verified live: `validate --workflow` with no source (metrics present) exits 1; `validate --workflow --source <typo>.plan.toml` exits 1; the TOML-primary `--workflow --source` with no `--plan` passes. The clap `requires = "plan"` on `--workflow` is removed (`src/main.rs:750`). The four-arm match at `src/main.rs:804-850` is exhaustive and correct: TOML-primary+metrics -> check; Markdown+metrics -> check; no-plan-source -> hard error; catch-all -> the pre-existing soft-skip for a present-source-but-missing-metrics case. The regression test `tests/validate_workflow_toml_source_needs_no_plan.rs` pins both the positive and the two negative directions and asserts the exact behaviors it claims.
- Drift-guard drop: `plan_template_documents_every_accepted_status` is removed from `src/plan.rs` (it referenced the deleted `pack/plan-template.md`); the status vocabulary is now render-generated from the code constants into `TEMPLATE.md` ("not started, in progress, complete, skipped, next, optional, deferred, blocked on <slug>" and the queue statuses), so the B3 drift risk it guarded is subsumed by render, as the spec directs. No dangling reference to the removed test or the deleted template.
- `AGENTS.md`, `.agents/AGENTS.reference.md`, and `pack/AGENTS.md` receive identical phase-2 and Tracking-progress edits (the `[[step]].status` SSOT + never-hand-edit-the-Roadmap-projection reconciliation, per Q-48), so the three copies stay consistent; `manifest.rs`'s dest-list test is updated to the full new asset set and passes.
- Self-scaffold: the root `docs/plans/TEMPLATE.plan.toml` is byte-identical to `pack/plan-template.plan.toml`; `.agents/`, `AGENTS.md`, and `README.md` regenerate clean under `just scaffold-self` (no git diff on them). The only non-clean regenerated file is `TEMPLATE.md` (R3-1).
- Principles: no violation found. Principle 12 (fail loudly) is honored, the scaffold render failure path exits 2 with a per-problem `eprintln` (`src/main.rs:1361-1367`) and the M-1 hard error; Principle 11 (tests drive real behavior) holds for the new regression test; Principle 16 (single source of truth) is advanced (status vocabulary render-generated, `[[step]].status` named as the SSOT across all prompt/doc copies).

## Commands run (verbatim)

Acceptance scaffold (`cargo run -- scaffold --output-dir /tmp/inc6-r3-<pid> --write --principles default --instrument`), tail:

```
          create  docs/plans/TEMPLATE.principles-note.md
          ... (all 29 assets) ...
          render  docs/plans/TEMPLATE.md
Wrote to /tmp/inc6-r3-1836692 (29 changed, 0 left untouched).
```

Acceptance validations (in the scaffolded dir):

```
=== validate --source ===
docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
exit=0
=== validate --workflow --source (empty metrics present, no --plan) ===
docs/metrics/workflow.jsonl: 0 records, valid
docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
=== render --check / render --check --strict ===
docs/plans/TEMPLATE.plan.toml: up to date
exit=0 (both)
=== render --check --strict after a hand-edit (expect non-zero) ===
error: docs/plans/TEMPLATE.md differs from a fresh render (a hand-edit, or a stale render after a source edit) (the committed file has 57 line(s); a fresh render has 55)
exit=1
=== M-1: --workflow with NO plan source (metrics present) ===
--workflow requested but no plan source resolved: pass a TOML-primary --source or a Markdown --plan
exit=1
=== M-1: --workflow with typo'd --source ===
no source plan at docs/plans/nope.plan.toml; nothing to validate
--workflow requested but no plan source resolved: pass a TOML-primary --source or a Markdown --plan
exit=1
```

`cargo test` (all suites green):

```
test result: ok. 292 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (validate_workflow_toml_source_needs_no_plan: both directions pass)
```

`cargo clippy --all-targets -- -D warnings` (after `touch src/main.rs` to force a fresh pass):

```
   Compiling agent-scaffold v0.0.1 (.../.claude/worktrees/inc6)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.72s
```

(No warnings, no errors.)

Note: the worktree was dirtied during review by `nix fmt` / `just scaffold-self` runs used to reproduce R3-1; those changes were stashed (not committed, not discarded) so the worktree is back at clean HEAD. Two review stashes remain on the stash list.
