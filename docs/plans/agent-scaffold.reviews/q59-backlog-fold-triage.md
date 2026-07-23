# Triage: q59-backlog-fold

Scope adjudicated: diff `dca1e61..0a7e4d6` on `plan/q59-backlog-fold`. Reviewer file: `docs/plans/agent-scaffold.reviews/q59-backlog-fold-reviewer-a.md` (one low finding). Verdicts judged against `AGENTS.md` (four-level `low`/`medium`/`high`/`critical` severity scale) and the plan principles, verified against the artifact directly rather than the write-up.

## Finding 1 (reviewer A, Low): new Q-59 receipt `task` diverges from the convention the same diff documents

Verdict: VALID. Severity: LOW (confirm reviewer's rating). Disposition: ACCEPTABLE (not must-fix before convergence).

- Fact-checked against the artifact and confirmed accurate:
  - The appended receipt at `docs/metrics/workflow.jsonl:163` (last record, 163 total) sets `"task":"q59-backlog-fold"` (the orphan-task name), while `folded_into = "resume-state-currency-signal"` in `docs/plans/agent-scaffold.plan.toml`. So `task` carries the orphan-task name, not the `folded_into` slug.
  - The new step `docs/plans/agent-scaffold.steps/document-receipt-task-convention.md` describes the convention as decision receipts setting `task` to the `folded_into` slug, and itself notes the convention is non-normative: "nothing joins on a decision record's `task`: the W4 receipt check keys on `q_id`, not `task`."
  - The reviewer's precedent claims hold exactly (verified via the decision receipts in `workflow.jsonl` and each question's `folded_into` in the plan): Q-49 (`task=doc-redundancy-cleanup`, `folded_into=doc-redundancy-cleanup`), Q-51 (`workflow-driver`/`workflow-driver`), Q-53 (`task-entry-regrounding`/`task-entry-regrounding`) all use the `folded_into` slug; Q-57 (`task=lifecycle-capture`, `folded_into=formatter-reflow-convention`) and Q-61 (`task=uniform-isolation`, `folded_into=uniform-agent-isolation`) use the orphan-task name. Q-59 follows the Q-57/Q-61 orphan-task precedent, so it is consistent with existing practice, just not with the "task = folded_into slug" pattern the new step describes.
- Why LOW and not higher: cosmetic / documentation-currency only. Nothing joins on a decision record's `task`; the W4 receipt check keys on `q_id`, independently confirmed by `validate --workflow` passing (`workflow invariants hold`) on the artifact. No behaviour, no validation outcome, no join is affected.
- Why ACCEPTABLE (not must-fix): the `task` convention for decision receipts is itself unresolved and inconsistently applied across prior receipts; `document-receipt-task-convention` (added in this same diff, order 71) is queued precisely to decide whether the convention is documented or ruled non-normative. Changing the Q-59 receipt to `resume-state-currency-signal` now would presume that decision resolves toward "task = folded_into slug," pre-empting the very backlog step that exists to settle it. The divergence is self-documenting rather than a defect, and it is already captured as backlog. No convergence blocker.

## Independent spot-check for missed material problems (no new findings)

Verified directly against the artifact; found nothing the reviewer missed.

- Q-59 coherence: `status = "decided"`, `folded_into = "resume-state-currency-signal"` (a real step slug added this diff at `order = 70`), `receipt = "Q-59"`. The appended `ask` DECISION text records Option A faithfully (minimal currency signal; fold into Q-58's structured transient; build only a checkpoint-commit / last-updated currency field; defer the enum + W6 transition-legality check + refuse-while-checkpointed behind a first-recorded-transition-failure evidence gate; no checkpoint/resume commands; state in the Q-58 carrier not the instrument-gated JSONL). No contradiction among the four fields or with the step body.
- Receipt integrity: single clean `+` append (numstat `1 0`, no existing line modified). `chosen` equals `recommendation` equals option A, and `chosen` is one of `options`. Matches the plan's recorded decision.
- Five new steps (orders 70-74): all `status = "deferred"`; orders verified unique, contiguous, and above the prior max of 69 (no duplicate orders anywhere in the plan). `resume-state-currency-signal` carries `[step.provenance]` decisions `Q-59` + the three Q-59 exploration findings; `soften-writer-agent-framing` carries decisions `Q-61`. The three pure documentation-currency folds (`document-receipt-task-convention`, `formatter-reflow-wording-polish`, `reconcile-baseline-doc-drift`) carry no provenance block, which is appropriate: provenance is optional and they defer their own decision rather than applying one. Sidecar bodies are faithful to their titles and, for `resume-state-currency-signal`, cite P6/P2/P1/P3 by name for the build-scope judgement.
- `resume-state-currency-signal.blocked_by = []` while the body says it is gated on Q-58: acceptable; `blocked_by` references step slugs and Q-58 is still an open question with no step to name, so the gate is expressed in prose, as elsewhere.
- `[meta].orphan_tasks`: `"q59-backlog-fold"` inserted in sorted position between `q58-capture` and `uniform-isolation`.
- Independent mechanical re-run on the artifact: `validate --workflow --source docs/plans/agent-scaffold.plan.toml` reports `163 records, valid`, `74 steps, 62 questions, valid`, `workflow invariants hold`.

## Round outcome

CLEAN. One valid low finding, ruled ACCEPTABLE (already captured as the `document-receipt-task-convention` backlog step; nothing joins on a decision record's `task`). No must-fix findings; no other material problem found. No convergence blocker.
