# Review: fold driver-output-generation hardening bundle (plan-authoring fold, low risk)

Branch: `impl/fold-bundle`, main `8ba5621`, HEAD `65624c5`.
Reviewer: adversarial / independent. READ-ONLY.

## Probes and results

1. Schema and validation: `validate --source`, `validate --workflow`, `render --check` all exit 0. Step count increments from 66 to 67; status distribution changes from `3 not-started, 3 skipped, 14 deferred` to `4 not-started, 4 skipped, 13 deferred` (one deferred retired to skipped, one not-started added). Validated clean.

2. `folds` field semantics (src/plan/source.rs): the field is defined at `source.rs:150`, validated at `source.rs:612-617` (must name a real step, no self-reference). There is no production use of `step.folds` anywhere in `src/plan/render.rs` or the rest of the codebase beyond validation - confirmed by exhaustive grep. The field is structural metadata stored in the TOML and checked for consistency but NOT projected into the rendered `agent-scaffold.md`. The planner's claim is correct.

3. `folds` + `skipped` combination: the two mechanisms are not redundant or contradictory - they operate at different layers. `folds = ["agents-worktree-planner-scope"]` on the new step is a typed cross-reference in the structured source, kept consistent by validation; it does not appear in the rendered roadmap. The `status = "skipped"` plus the absorption-naming title on `agents-worktree-planner-scope` is what makes the human-readable roadmap coherent (the roadmap table and step-detail section read correctly). Using both is sound. Recommendation: keep both.

4. `agents-worktree-planner-scope` absorption: the step exists, the slug resolves, validation passes. The rendered roadmap table shows the step as `skipped` with no provenance note, and the step detail opens with "Skipped. Absorbed into..." - correct for a human reader.

5. Stale trailing word: the absorbed sidecar (`agent-scaffold.steps/agents-worktree-planner-scope.md:3`) ends with the word "Deferred." from the original text while the new opening line says "Skipped." The sidecar's framing ("preserved below for the record") is clear, so the original text is not authoritative, but the terminal "Deferred." is stale. See F-1.

6. Sidecar accuracy (`driver-output-generation.md`): the five-bullet decided scope maps correctly to design decisions D-a through D-e in `driver-output-generation.design.md`. Bullet 1 (isolation_policy fragment) = D-b Option 2 + design doc fix 1. Bullet 2 (spawns_writer fix) = D-d Option 1 (prerequisite for fix 2). Bullet 3 (always-on reminder) = design doc fix 2. Bullet 4 (selective principle-text projection) = D-a Option 2 + D-e Option 1. Bullet 5 (orchestrator preamble) = D-c Option 1 + design doc fix 3. The sidecar counts five discrete changes; the design doc's staging section groups the same work as "three fixes" with spawns_writer embedded as a prerequisite and D-a handled as a separate decision. Both descriptions cover the same scope; the sidecar's five-point count is more granular but not inaccurate. The deferred items (role-prompt inlining, generating other prose rules, Stage 3+ enforcement, Stage 2 multi-loop FSM) match the design doc's deferred list. Sidecar correctly points at the design doc ("the full design...lives in driver-output-generation.design.md") rather than restating it.

7. Provenance: `decisions = ["Q-51"]` - Q-51 exists at `agent-scaffold.plan.toml:1184`, status `decided`. `findings = ["driver-output-generation.design.md"]` - the file exists at `docs/plans/driver-output-generation.design.md`, and the ref is a valid task-relative path (no absolute root, no `..` component, passes `is_safe_sidecar_ref` as confirmed by clean validation). Convention matches the other provenance entries that use task-relative refs. Both are apt: Q-51 is the governing workflow-driver decision this step follows up; the design doc is the approved spec.

8. Collateral: four files changed, all expected (`agent-scaffold.md`, `agent-scaffold.plan.toml`, `agent-scaffold.steps/agents-worktree-planner-scope.md`, `agent-scaffold.steps/driver-output-generation.md`). No `src/` changes. Q-44 and all other questions/steps outside the two intended are untouched in the diff.

9. Style: all four changed files are ASCII-clean (verified byte-by-byte). No em-dashes, en-dashes, double-hyphen-as-dash, unicode symbols, or emoji in the diff. Prose is not hard-wrapped.

---

## Findings

**F-1** | severity: low | `docs/plans/agent-scaffold.steps/agents-worktree-planner-scope.md:3`

The original sidecar text ends with the word "Deferred." after the new absorption notice and the "preserved below for the record" framing. The step is now `skipped`, so the terminal word is stale. The context makes the intent clear (the original scope is preserved verbatim, not updated), but a reader scanning only the end of the section sees "Deferred." while the step is retired as skipped. Low severity: the leading "Skipped. Absorbed into..." is unambiguous, and the record-preservation framing is explicit. Fix is one word ("Deferred." -> "Deferred (original scope, preserved for the record)." or simply drop the word). Not a blocker.

---

## Summary

One finding, low severity. All validation passes clean. The `folds`-plus-skipped combination is correct and necessary: `folds` is structural metadata (validated, not rendered); `skipped` with an absorption title is what the human-readable roadmap requires. Keep both. The new step's scope summary is accurate against the design doc. Provenance is well-formed and apt. No collateral changes.
