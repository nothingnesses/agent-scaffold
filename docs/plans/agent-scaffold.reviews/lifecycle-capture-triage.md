# Triage verdicts: `plan/lifecycle-capture` (245200e..987a253, head 987a253)

Independent triage of the two reviewer findings files. Reviewer B (mechanical lens) reported no findings; I spot-confirmed its key claims (W4 keys on `q_id`, append-only single line, render up to date) against the code and diff and found nothing to overturn. Reviewer A (faithfulness lens) raised two low findings plus one pre-existing out-of-diff note. Each was re-verified against the artifact, not taken on the reviewer's word.

## Finding 1: decision receipt `task` is `lifecycle-capture`, not the folded-into slug

Verdict: VALID (the factual inconsistency is real). Severity: low. Disposition: DEFER (not a must-fix in this pass; backlog it).

Evidence, independently confirmed:
- `docs/metrics/workflow.jsonl` last line (the appended `type:"decision"` for `Q-57`) sets `"task":"lifecycle-capture"`, while `docs/plans/agent-scaffold.plan.toml` Q-57 sets `folded_into = "formatter-reflow-convention"`.
- The 9 prior `type:"decision"` receipts each have `task` equal to their question's `folded_into` slug: Q-45/46/47/48 -> `structured-skeleton`, Q-49 -> `doc-redundancy-cleanup`, Q-50 -> `doc-currency-guidance`, Q-51 -> `workflow-driver`, Q-53 -> `task-entry-regrounding`, Q-54 -> `human-input-gate-reinforce` (verified `folded_into` for each in the branch TOML). So this receipt is genuinely the only one whose `task` names something other than its folded-into step.

Why it is only low and only a defer, not a fix that blocks convergence:
- Nothing joins on `task` for a decision receipt. W4 reads decisions through `metrics::parse_decisions`, whose `Decision` projection carries exactly `{ line, q_id }` (`src/metrics.rs:718-724`); the join in `w4_problems` matches purely on `d.q_id == question.id` (`src/workflow.rs:337`). Confirmed the W4 baseline itself is sourced from the TOML `[meta].w4_baseline`, not from `task` (`src/workflow.rs:216` -> `baseline_from_toml(plan)`). Enforcement passes regardless of the `task` value (Reviewer B's `validate --workflow` pass corroborates).
- `task` is not a documented field on a decision receipt at all. The instrument schema for `type:"decision"` (`AGENTS.md:145`, `pack/instrument.md:9`) lists only `q_id`, `options`, `recommendation`, `chosen`. There is no written rule that a decision receipt's `task` must equal its folded-into slug, so the "convention" is a de-facto pattern, not a documented contract.
- The pattern held 9/9 only because in every prior case the pass that RECORDED the decision was the same as the step it folded into. Here they legitimately diverge for the first time: Q-57 was decided at the `driver-output-generation-inc2` escalation (see the `type:"escalation"` line immediately above it) and was formalised in a separate `lifecycle-capture` bookkeeping pass, which is a registered `[meta].orphan_tasks` entry. Read as "the work unit that recorded this" (the meaning `task` carries on `round`/`escalation` records), `lifecycle-capture` is accurate. So there are two defensible semantics (recording-pass vs folded-into-step), and the writer picked the accurate-to-recording-pass one.

Debate (producer vs reviewer), ruled: the reviewer is right that the value breaks the observed pattern and that a step-slug filter will miss this receipt; the producer is right that the value is accurate, the field is undocumented, and no machine reads it. Ruling: the observation is VALID and worth capturing, but it is not a defect that must be fixed before convergence, because "fixing" it presumes an answer to an undocumented convention question. Backlog item: decide and DOCUMENT what `task` means on a decision receipt (recording-pass vs folded-into slug), then normalise if the folded-into reading wins. Do not silently rewrite this receipt in-place under an unstated rule.

## Finding 2: reconciliation is silent on the "leaves incidental reformatting to the orchestrator" clause

Verdict: VALID (a real incompleteness), but the text is NOT contradictory under its natural reading. Severity: low. Disposition: DEFER (optional wording polish; not a convergence blocker).

Evidence, independently confirmed:
- Rule 79 (`AGENTS.md:79`): "Format only your own files. An implementer formats only the files it changed; it must not run repo-wide formatters ... or `git checkout` / `git restore` on files it does not own, and leaves incidental reformatting to the orchestrator."
- New reconciliation (`AGENTS.md:108`, regenerated from `pack/AGENTS.md`): "... This is distinct from the 'Format only your own files' file-safety rule above, which still holds that a writer does not proactively run a repo-wide formatter; this convention governs only the incidental reflow that appears anyway, so the two do not conflict."

Analysis:
- Rule 79 has two clauses: (a) no repo-wide formatter / no `checkout`/`restore` on files-not-owned, and (b) "leaves incidental reformatting to the orchestrator." The reconciliation explicitly addresses (a) but never names (b), which is the clause a reader is most likely to read as in tension, since the new convention has the writer KEEP incidental reflow in its own files rather than leave it to the orchestrator.
- Whether (b) actually conflicts depends on reading. Narrow reading (the whole sentence scoped to files-not-owned): no conflict, since the new convention governs the writer's OWN changed/regenerated files, a disjoint file set; the reconciliation's phrase "governs only the incidental reflow that appears anyway" implicitly relies on exactly this scoping. Broad reading (clause (b) standalone: all incidental reformatting is the orchestrator's job): the new convention refines/overrides (b) for own-files, which the text should say rather than assert bare "the two do not conflict."
- So the text is not contradictory under the natural (narrow) reading, which is the one context supports; it is merely less explicit than ideal because it leans on that reading without stating it and without naming clause (b). That is a completeness nit, not a defect.

Ruling: VALID as an incompleteness, low severity, defer as optional polish (a one-clause addition to the reconciliation naming clause (b) would close it). It does not block convergence: the natural reading yields no conflict, and the faithfulness of the convention to the human's decision is not in question (Reviewer A's own clean-list confirms the wording encodes the decision without overreach).

## Pre-existing note: `AGENTS.md` baseline described as a JSONL `type:"baseline"` record

Verdict: CONFIRMED pre-existing; OUT OF SCOPE for this pass. Not a finding against this change.

Evidence:
- The live W4 baseline mechanism for this repo is the TOML `[meta].w4_baseline = "Q-44"` (`docs/plans/agent-scaffold.plan.toml:3`), consumed via `baseline_from_toml(plan)` (`src/workflow.rs:216`), per the Q-46 cutover that moved the baseline to TOML and pruned the historical JSONL `baseline` line (Q-46 `ask`, plan TOML). The schema text at `AGENTS.md:146` still documents `type:"baseline"` as the baseline mechanism, so the drift the reviewer describes is real (minor line-number slip: the baseline text is line 146; line 145 is the `decision` schema).
- This diff does not touch it: `git diff 245200e..987a253 -- AGENTS.md` has zero `baseline` hunks. The drift predates the change, so it cannot be a finding against this round. It is a separate backlog item (update the pack instrument schema to describe the TOML `[meta].w4_baseline` for a `primary="toml"` plan), not a fix owed here. Reviewer A correctly filed it as an aside, not a finding.

## Reviewer-missed check

None material. I re-verified the load-bearing mechanical claims (append-only single decision line; W4 join key; `folded_into` back-reference; `order = 68` uniqueness via the diff; `orphan_tasks` alphabetical insertion; regeneration parity of the three AGENTS files) and found nothing the two reviewers missed that is wrong with the diff.

## Round outcome

CLEAN. No new valid IN-SCOPE finding requires a fix before convergence. Both of Reviewer A's findings are low-severity and defensible as-shipped (Finding 1's value is accurate to the recording pass and joins on nothing; Finding 2 is non-contradictory under its natural reading); both are DEFERRED as backlog/optional-polish items, and the pre-existing baseline-doc drift is out of scope.
