# Review: q59-backlog-fold (reviewer A)

Scope: diff `dca1e61..0a7e4d6` on `plan/q59-backlog-fold`. Applies the human Q-59 decision (Option A, minimal currency signal) and folds five deferred backlog steps. Reviewed against `.agents/prompts/reviewer.md`, the plan principles, and the change brief.

## Summary

The change is coherent and mechanically clean. Q-59's `status` / `folded_into` / `receipt` / appended `ask` decision all agree with each other, with the new `resume-state-currency-signal` step, and with Option A as briefed (minimal currency signal; fold into Q-58; defer the richer lifecycle behind an evidence gate; no checkpoint/resume commands; state in the Q-58 carrier, not the JSONL). The `workflow.jsonl` receipt is a single clean append with `chosen` present in `options` and equal to `recommendation`. All four mechanical checks pass (74 steps, 62 questions). One low-severity coherence observation, no medium/high/critical findings.

## Findings

### Low: new Q-59 receipt `task` diverges from the convention the same diff documents

- Evidence: the appended receipt in `docs/metrics/workflow.jsonl:163` sets `"task":"q59-backlog-fold"` (the orphan-task name), while the newly-added step `docs/plans/agent-scaffold.steps/document-receipt-task-convention.md` describes the convention as "decision receipts set their `task` field to the question's `folded_into` slug" (which for Q-59 would be `resume-state-currency-signal`).
- Why it is low, not higher: the convention is already non-normative and already inconsistently applied. Nothing joins on a decision record's `task` (the W4 receipt check keys on `q_id`, confirmed by `validate --workflow` passing), and prior receipts already split both ways: Q-49/Q-51/Q-53 use the `folded_into` slug, while Q-57 (`task:lifecycle-capture`, `folded_into:formatter-reflow-convention`) and Q-61 (`task:uniform-isolation`, `folded_into:uniform-agent-isolation`) use the orphan-task name instead. The new Q-59 receipt follows the Q-61 orphan-task precedent, so it is consistent with existing practice, just not with the "nine receipts" pattern the new step describes. This is the exact ambiguity that `document-receipt-task-convention` is queued to resolve, so it is self-documenting rather than a defect.
- Severity: low (cosmetic/currency; no behaviour, no validation, no join affected).

## Non-findings checked (all clear)

- Q-59 coherence: `status = "decided"`, `folded_into = "resume-state-currency-signal"` (a real step slug added in this diff, order 70), `receipt = "Q-59"` (matches the `receipt = q_id` convention used by Q-51/53/54/57/61), and the appended `ask` DECISION text records Option A faithfully: minimal currency signal, fold into Q-58's structured transient, build only a checkpoint-commit / last-updated field projected as "current / N commits stale / tree dirty", defer the `session_state` enum + transition-legality check + refuse-while-checkpointed behind an evidence gate (first recorded transition failure, mirroring Q-51), no checkpoint/resume commands, state in the Q-58 carrier not the instrument-gated JSONL. No contradiction among the four fields or with the step body.
- Receipt integrity: `chosen` = `"A: minimal currency signal (fold into Q-58, defer the rest behind an evidence gate)"` is one of `options` and identical to `recommendation`; `recommendation` present; `chosen` matches the plan's recorded decision. `git diff dca1e61..0a7e4d6 -- docs/metrics/workflow.jsonl` is a single `+` line (numstat `1 0`), no existing line modified.
- Five new steps: all `status = "deferred"`; `order` values 70-74 are each unique and contiguous above the prior max (69), no collisions with any existing order. Entries are well-formed (`blocked_by`/`folds`/`increment`/`waiver` empty arrays). `resume-state-currency-signal` carries `[step.provenance]` decisions `Q-59` + the three Q-59 exploration findings; `soften-writer-agent-framing` carries `[step.provenance]` decisions `Q-61`. Sidecars present for all five and render into the roadmap.
- Faithful bodies: `resume-state-currency-signal.md` records the deferred-behind-evidence-gate scope exactly (currency field only; enum/W6-check/refuse-while-checkpointed deferred behind a first recorded transition failure; no write-command pair; Q-58 carrier home) and cites P6, P2, P1, P3 by name for its judgements. `soften-writer-agent-framing.md` captures the accepted round-2 residual (writer-only framing spots in `AGENTS.md` + `pack/isolation-guidance.md` lines 3/30/37 to universal "spawned agent" wording).
- Principle citation: the one step making a substantive build-scope judgement (`resume-state-currency-signal`) cites principles by name. The other four folds are low-priority documentation/polish placeholders that make no principle-level judgement (they defer the decision itself), so absence of a named principle there is appropriate, not a gap.
- `resume-state-currency-signal.blocked_by = []` while the body says it is gated on Q-58: acceptable, because `blocked_by` references step slugs and Q-58 is still an open question with no step to name; the gating is expressed in prose as with other exploration-gated steps.
- `[meta].orphan_tasks` includes `"q59-backlog-fold"` (inserted in sorted position between `q58-capture` and `uniform-isolation`).
- No schema drift, dangling reference, or internal inconsistency observed. A `decided` question folding into a `deferred` (not complete) step is permitted and validates.

## Mechanical checks (command tails)

`cargo run -- validate --source docs/plans/agent-scaffold.plan.toml`:

```
docs/metrics/workflow.jsonl: 163 records, valid
docs/plans/agent-scaffold.plan.toml: 74 steps, 62 questions, valid
```

`cargo run -- validate --workflow --source docs/plans/agent-scaffold.plan.toml`:

```
docs/metrics/workflow.jsonl: 163 records, valid
docs/plans/agent-scaffold.plan.toml: 74 steps, 62 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

`cargo run -- render --check docs/plans/agent-scaffold.plan.toml`:

```
docs/plans/agent-scaffold.plan.toml: up to date
```

`cargo test`:

```
test result: ok. 342 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
test result: ok. 1 passed; 0 failed; ...
test result: ok. 3 passed; 0 failed; ...
test result: ok. 1 passed; 0 failed; ...
test result: ok. 2 passed; 0 failed; ...
```

All four checks pass; step/question counts match the expected 74 / 62.
