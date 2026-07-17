# reviewer-diversity review (opus, correctness / consistency / completeness)

Diff range reviewed: `c45e36e..959ab7a` (commit `959ab7a`, branch `impl/reviewer-diversity`). Lens: correctness of the generalization, self-consistency, and completeness of coverage across the pack and repo.

Scope confirmed touched: `pack/AGENTS.md`, `AGENTS.md`, `.agents/AGENTS.reference.md` only (three files, `git diff --name-only c45e36e..959ab7a`). Nothing else changed.

## Summary of verification (things that are correct, stated so they are not re-litigated)

- Both diversity clauses in `pack/AGENTS.md` are generalized, consistently:
  - Reviewers role line (`pack/AGENTS.md:21`): "different models or harnesses where available, since same-model and same-harness reviewers share blind spots".
  - Design-explorations explorer line (`pack/AGENTS.md:61`): "different lenses, models, or harnesses".
- Self-scaffold has no drift: `AGENTS.md:21`/`:61` and `.agents/AGENTS.reference.md:21`/`:61` carry byte-identical changed clauses to the pack source (verified via `git show 959ab7a:<file>`). The only remaining differences between the generated files and `pack/AGENTS.md` are the expected `{{principles}}` and `{{instrument}}` template expansions, not the diversity text. Single-source-of-truth holds.
- Rationale is logically sound. The rule prescribes differing on at least one axis ("different models OR harnesses"); the rationale names the worst case where both axes are identical ("same-model AND same-harness reviewers share blind spots"). The `or` (prescription) and `and` (failure case) are consistent, not contradictory: identical-on-both is the case to avoid, differing-on-either is the fix.
- ASCII-clean: only `or harnesses` / `and same-harness` were added; no unicode, em-dashes, or special characters.
- Leaving `pack/prompts/reviewer.md` and `pack/prompts/triager.md` unchanged is CORRECT (see below), not a gap.

## Findings

### 1. Missed location: `pack/user-prompts/review.md` still cues only "which models" (severity: low)

Evidence: `pack/user-prompts/review.md:15`: `[Optional: depth and lenses, for example how many independent reviewers and which models, and whether to treat the target as risky ...]`

This is the human-facing review-request prompt (review entry mode). Its optional depth/lenses cue is the one user-facing surface that invites the human to specify reviewer diversity, and after this change it still frames that diversity as "which models" only, not "which models or harnesses". The step's stated purpose (plan `docs/plans/agent-scaffold.md:643`) is precisely to guide "a user who runs multiple CLIs ... to spread reviewers across harnesses too", so this prompt is arguably the most on-point place for the harness axis to appear, and it was left inconsistent with the newly-generalized canonical rule.

Why low, not higher: the field is an optional, bracketed, free-text example; the human is not constrained by it, and the canonical guidance is now correct in `AGENTS.md`, which the orchestrator reads regardless. Impact if unfixed is a minor consistency wart. Note it is outside the plan step's literal named-target list (which only named `pack/AGENTS.md`, `reviewer.md`, `triager.md`), so this is a genuine completeness gap the step scope itself did not anticipate. Suggested fix: "which models or harnesses". If changed, the regenerated `.agents/user-prompts/review.md` would also need to stay in sync (it is a scaffolded asset), so this would grow the change by one more file-pair.

### 2. Plan step detail will misstate the change once the step is marked complete (severity: low)

Evidence: `docs/plans/agent-scaffold.md:643` describes the edit as touching "`pack/AGENTS.md` ..., `pack/prompts/reviewer.md`, and `pack/prompts/triager.md`, then regenerate the self-scaffold". The implementer correctly did NOT touch `reviewer.md` or `triager.md` (they carry no diversity clause; verified by reading both files in full: `pack/prompts/reviewer.md` and `pack/prompts/triager.md` contain no model/harness/diversity language). The plan's durable record therefore still asserts two file targets that were intentionally not edited.

This is not a defect in the code change itself, and the Roadmap status is still `not started` (`docs/plans/agent-scaffold.md:165`), so the plan has not yet been reconciled to the completed work; that reconciliation is the orchestrator's post-merge bookkeeping. Raising it so it is not lost: when the step is closed, correct the step-detail file list (or add an Outcome note) to record that `reviewer.md`/`triager.md` were deliberately left unchanged because the diversity rule is an orchestrator-spawning concern canonical in `AGENTS.md` (Principle 16, one source of truth). Otherwise the plan carries a stale, contradicted claim.

### 3. No CHANGELOG entry, against existing precedent (severity: low)

Evidence: `CHANGELOG.md` Unreleased `### Changed` (`CHANGELOG.md:16`) already tracks a very similar scaffolded-guidance doc change: "Hardened the scaffolded workflow guidance (`AGENTS.md` and the role prompts) after a design review ...". The scaffolded `AGENTS.md` is the shipped product, and a user scaffolding a new project now gets materially different guidance (harness diversity), so by the project's own precedent this user-visible guidance change is CHANGELOG-worthy and is currently missing.

Why low: it is a one-clause edit, the release is still Unreleased, and the cleanest resolution is to fold one clause into the existing "Hardened the scaffolded workflow guidance" Unreleased entry rather than add a new bullet. Judgment call; flagged with reasoning so the triager/human can decide whether to require it.

### 4. Instrumentation prose still frames review diversity as model-only (severity: low, informational)

Evidence: `pack/instrument.md` (rendered into the generated `AGENTS.md` `{{instrument}}` section) describes the `reviewers[].model` field as existing "so per-reviewer productivity and the value of running multiple models can be calibrated". After generalizing the diversity rule to harnesses, this calibration description (and the `model` schema field) still frame diversity purely as "multiple models".

I am NOT recommending a fix now: this is a metrics-schema concern, the step is explicitly docs-only with no code, and the cross-harness spawn/calibration machinery is deliberately deferred to `optional-modules` increment 3 (`docs/plans/agent-scaffold.md:524`). Recording it only so the incompleteness is known and tracked to that deferred step rather than silently forgotten.

## Explicit no-findings

- self-contained "harness" / Principle 20: No finding. "harness" is established, pervasively-used terminology in `AGENTS.md` (line 3 introduces it with the `CLAUDE.md` example: "harness-agnostic: any harness-specific file (for example `CLAUDE.md`)"), and the new "models or harnesses" usage is consistent with that. No new gloss is needed.
- `reviewer.md` / `triager.md` left unchanged: No finding; this is the correct call. Neither prompt states a diversity/model preference, so there is nothing to generalize; adding one would duplicate the canonical AGENTS.md rule and violate one-source-of-truth.
- Phrasing asymmetry between the two clauses (reviewers line carries the "share blind spots" rationale, explorer line does not; different list grouping): No finding. Pre-existing structure, untouched intent, stylistic only.
- critical: none. high: none. medium: none.
