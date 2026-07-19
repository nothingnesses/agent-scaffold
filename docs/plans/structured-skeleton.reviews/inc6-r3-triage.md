# Inc 6 Round 3 (holistic) triage

Triager: independent, read-only except this verdict file. Independent of both the implementer and the orchestrator. Scope: adjudicate the single finding raised by round 3 (holistic, opus) in `inc6-r3-holistic-opus.md`. Rounds 1-2 are settled (M-1/M-2 fixed, round 2 clean); round 3 raised only R3-1. Diff range for the increment: `e06a89b..HEAD` on `impl/structured-skeleton-inc6` (worktree `.claude/worktrees/inc6`).

## R3-1: generated `docs/plans/TEMPLATE.md` render artifact is not in the flake prettier exclude list

Verdict: VALID. Final severity: low (confirmed, unchanged from the reviewer's rating).

### Claims verified in the worktree

- `flake.nix` excludes `agent-scaffold.md` but not `TEMPLATE.md`. Confirmed. `flake.nix:47-50` reads `settings.global.excludes = [ "src/plan/testdata/render-fixture*" "docs/plans/agent-scaffold.md" ]`. `docs/plans/TEMPLATE.md` is absent. The comment at `flake.nix:41-46` states the invariant the exclude enforces: render artifacts are byte-exact `agent-scaffold render` output and prettier/taplo must not touch them or `render --check` would diverge, and this project's own plan is now such an artifact.
- `TEMPLATE.md` is a render artifact. Confirmed. Line 1 carries the generated-file / do-not-hand-edit banner ("Source: TEMPLATE.plan.toml, TEMPLATE.steps/, TEMPLATE.questions/, and the [meta].sidecars prose ... hand edits are overwritten and caught by `agent-scaffold render --check`"). It is produced by the scaffold render loop (`src/main.rs:1349-1375`), not shipped as a manifest asset, exactly like its sibling `agent-scaffold.md`.
- `nix fmt` (prettier) would reflow the committed `TEMPLATE.md`. Confirmed by running the repo's own prettier against a copy, using the repo's `.prettierrc.json` (`proseWrap: never`). Prettier reports `TEMPLATE.md` as not-clean and rewrites it: it collapses a blank line (a `40d39` deletion, the blank after the Open Questions heading region) and repads the Roadmap GFM table separator from `| --- | --- | --- |` to `| -------------- | ----------- | ----- |`. So the committed file is the raw-render form and is not prettier-clean; `just fmt` and the `nix fmt` tail of `just scaffold-self` (`justfile:46-48`) leave an uncommitted `TEMPLATE.md` diff. This reproduces the reviewer's evidence.

### Why low (not higher)

- No active gate fails today. `.agents/checks.toml` gates `render --check --strict` only on the live `docs/plans/agent-scaffold.plan.toml`, not on `TEMPLATE.plan.toml`, and there is no `nix fmt --check` / `--fail-on-change` gate (it exists only as a commented row in `pack/checks.toml:65`) and no CI. So nothing is red at HEAD.
- The impact is a self-consistency / dogfooding defect plus a latent divergence: a contributor running `just fmt` or `just scaffold-self` gets a spurious `TEMPLATE.md` diff, and if it is ever committed, the file flips from the raw-render form to the prettier form, which then cannot both satisfy a future `render --check` on `TEMPLATE.plan.toml` and a future `nix fmt --fail-on-change`. This is exactly the render-vs-prettier divergence the exclude list was created to prevent, already applied to `render-fixture*` and `agent-scaffold.md`. It contradicts the `scaffold-self` recipe's own stated "stable committed fixed point" invariant (`justfile:44-45`) for `TEMPLATE.md`. Real but contained, so low is correct.

### Complete fix

Add `docs/plans/TEMPLATE.md` to the `settings.global.excludes` list in `flake.nix` (the `treefmtEval` block at `flake.nix:47-50`), so the list becomes:

```
settings.global.excludes = [
  "src/plan/testdata/render-fixture*"
  "docs/plans/agent-scaffold.md"
  "docs/plans/TEMPLATE.md"
];
```

This is the entire fix; no re-render or sidecar handling is needed. I checked the residual-sidecar concern explicitly (would excluding `TEMPLATE.md` while its `TEMPLATE.*.md` prose sidecars stay non-excluded leave a divergence, since a sidecar reflow changes what a fresh render produces?):

- Running the repo's prettier (with `.prettierrc.json`, `proseWrap: never`) over all nine `TEMPLATE.*.md` sidecars reports them ALL clean; only `TEMPLATE.md` itself is flagged. So `nix fmt` is a no-op on the sidecars, a fresh render splices already-clean sidecar prose, and the only prettier-induced change is to `TEMPLATE.md`'s render-generated structure (the Roadmap table padding and a blank line), which the exclude prevents.
- This matches the r3 reviewer's own observation that `just scaffold-self` leaves only `TEMPLATE.md` non-clean (the sidecars, `.agents/`, `AGENTS.md`, and `README.md` all regenerate clean), and it mirrors exactly how the sibling `agent-scaffold.md` is handled: its sidecars are likewise not excluded but are prettier-clean, and `agent-scaffold.md` (the raw-render view) is the one excluded.

After the change, `scaffold-self`'s render-then-`nix fmt` ordering is a stable fixed point for `TEMPLATE.md`: render produces the raw-render-of-clean-sidecars form, and `nix fmt` skips the now-excluded `TEMPLATE.md`, so no diff remains. No re-render of `TEMPLATE.md` is required because the committed file is already the raw render of the current (clean) sidecars.

Note: no change to `pack/checks.toml` or `.agents/checks.toml` is required or implied by this fix; adding a `render --check` gate on `TEMPLATE.plan.toml` would be a separate, optional hardening (out of scope for R3-1).

## Other findings

None. Round 3 (holistic) raised only R3-1. Rounds 1-2 are settled: M-1 and M-2 fixed (the r3 pass independently re-confirmed the M-1 hard-error path at `src/main.rs:836-844` and the `tests/validate_workflow_toml_source_needs_no_plan.rs` regression test), and round 2 was clean. The rest of the r3 holistic pass ("Confirmed sound") raised no findings and matches the shipped state.

## Summary

- R3-1: VALID, low. Fix: add `"docs/plans/TEMPLATE.md"` to `settings.global.excludes` in `flake.nix`. No sidecar re-render needed (sidecars verified prettier-clean). Complete.
