# Review: Q-58 capture (next-output projection finding)

Branch: impl/next-output-q, HEAD dd5d920, diff main (5bdca7b)..HEAD.

Result: CLEAN. No findings.

## What was verified

1. SCHEMA: All three validation commands green.
   - `validate --source docs/plans/agent-scaffold.plan.toml`: 67 steps, 58 questions, valid.
   - `validate --workflow --source docs/plans/agent-scaffold.plan.toml`: workflow invariants hold.
   - `render --check docs/plans/agent-scaffold.plan.toml`: up to date.
   - Q-58 is unique, follows Q-57 (confirmed by grep on plan.toml at line 1225). Status is `open`. No `folded_into`, `receipt`, or `superseded_by` fields present.
   - Sidecar docs/plans/agent-scaffold.questions/Q-58.md is 0 bytes, matching the empty-sidecar convention used by all other Q-*.md files.

2. ACCURACY: Every factual claim in the ask is grounded in real artifacts.
   - Q-51 is real (status `decided`, folded into `workflow-driver`). Its ask confirms Stage 1 is the advisory read-only stateless `agent-scaffold next` MVP. The Q-58 description "driver's Stage-1 forward projection, Q-51" is accurate.
   - Q-52 is real (status `exploring`). Its ask defines the falsifiable-deletion method ("if I REMOVE this, does anything observably notice?"). The Q-58 claim "Q-52's falsifiable-deletion method applied to output CONTENT" is accurate and the composition with Q-52 is apt.
   - The "A-b Stage-1 decision deliberately chose verbatim-echo to avoid heuristic prose parsing" is confirmed by docs/plans/workflow-driver-stage1.build-plan.md line 6: "A-b: the filled prompt echoes the ledger `## RESUME STATE` block VERBATIM; populate structured `artifact`/`diff` slots only from an explicit marker if present, never by heuristic prose parsing (Principle 12 fail-loud, Principle 2)." The reference is real and the paraphrase is accurate.
   - The bloat measurement (150145 bytes / 148 lines, 799-byte ACTIVE LOOP block, ~149 KB echo of 27 anchors, 1 live) is presented as a historical observation; no overstatement.
   - The source-side fix (~6 KB after bounding to the live anchor) is described accurately.
   - The three candidate directions (tool projection; structured transient per Principle 8; empirical ablation composing with Q-52) are accurately enumerated.

3. CLASSIFICATION: `open` is correct and consistent with Q-55 (the comparator). Both Q-55 and Q-58 have enumerable candidate directions, no design pass owed, and a "Do not build until decided" guard. `exploring` would require a design pass owed first; no pass is owed here since the candidate directions are already enumerated.

4. NO COLLATERAL: Only three files changed (docs/plans/agent-scaffold.plan.toml, docs/plans/agent-scaffold.md, docs/plans/agent-scaffold.questions/Q-58.md). No src/, ledger, or metrics changes. Q-44 is untouched (confirmed by diffing Q-44 in plan.toml: no hits in the diff output).

5. STYLE: No em-dashes, en-dashes, double-hyphens as dashes, unicode symbols, or emoji in Q-58's ask or commit message. ASCII only throughout. Prose is not hard-wrapped (no line-length issues raised).
