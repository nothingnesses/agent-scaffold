# Review: Q-64 / Q-65 fold, validity and conventions lens

Scope: `git diff main HEAD` on branch folding Q-64 and Q-65 into `docs/plans/agent-scaffold.plan.toml`, six new step sidecars, two decision receipts, one orphan task, and the regenerated `docs/plans/agent-scaffold.md`.

## Result

Zero findings. All three validators pass.

Validator output (verbatim evidence):

- `docs/plans/agent-scaffold.plan.toml: 84 steps, 65 questions, valid`
- `docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold`
- `docs/plans/agent-scaffold.plan.toml: up to date`

(from `validate`, `validate --workflow`, and `render --check` respectively; all exit 0.)

## Checks performed

- Validators (item 1): all three pass; `render --check` reports `up to date`, so the committed `.md` is a fresh projection (not stale).
- TOML conventions (item 2): all six new `[[step]]` blocks carry `slug`/`title`/`status`/`order`/`blocked_by`/`folds`/`waiver`, and `increment = []` (nothing built yet). Each has a `[step.provenance]` with a `decisions` array (`generated-projection`, `agents-md-drift-guard`, `principle-by-name-projection`, `roles-findings-naming-slots`, `workflow-toml-rule-fragments` -> `["Q-64"]`; `rename-to-agent-flow` -> `["Q-65", "Q-64"]`). Step statuses used (`in-progress`, `next`, `deferred`) are all from the valid hyphenated enum. Question statuses set to `decided` are valid. `folded_into` + `receipt` fields on Q-64/Q-65 match the exact convention of prior folded-and-decided questions Q-60/Q-62/Q-63 (`plan.toml:1531-1533`, `1546-1548`).
- Orders (item 3): the six new steps occupy 79-84 uniquely; the full order set 73..84 is contiguous with no duplicate and no gap (checked all orders in file). Orders match the slug intent stated in the sidecars.
- Sidecars (item 4): all 84 step slugs have a same-named sidecar and there are no orphan sidecars (84 slugs = 84 files, bidirectional cross-check clean). The two flipped steps (`single-source-recommendation-rule`, `driver-isolation-reminder-scope`) retain their sidecars (unchanged by the diff). New sidecar headings follow the existing `### \`slug\`: title (\`Q-XX\`)` convention.
- Receipts (item 5): the two appended `workflow.jsonl` lines are valid single-object JSON with field shape identical to prior `type:"decision"` records (`type`, `task`, `q_id`, `options`, `recommendation`, `chosen`, `ts`); `ts` is `"2026-07-23"` in the file's date format. `recommendation != chosen` on the Q-65 record is legitimate and matches prior records (e.g. Q-45) and the question prose (orchestrator leaned flowgate/agentflow, human chose agent-flow).
- Orphan task (item 6): `q64-q65-fold` added to `[meta].orphan_tasks` (`plan.toml:17`), correctly alphabetically ordered between `q64-capture` and `q65-capture`.
- W4 (item 7): Q-64 and Q-65 are past `w4_baseline` (Q-44); both are now `status = "decided"` with matching `type:"decision"` records (`q_id:"Q-64"`, `q_id:"Q-65"`) at `workflow.jsonl:179-180`. `validate --workflow` passes; these two receipts are the records that satisfy the past-baseline-decision-needs-receipt invariant (absent them the workflow validator would fail).
- Text rules (item 8): scanned all added lines across the entire diff (TOML, sidecars, jsonl, generated `.md`). No non-ASCII characters. The only `--` token is `--check` (a CLI flag), not a dash substitute. No em/en dashes, emoji, or unicode arrows/math/bullets.
- Hard-wrap (item 9): long single-line prose is present and correct; not flagged.

Cross-check of the regenerated `.md` status line is internally consistent: `84 steps (2 not started, 2 in progress, 52 complete, 4 skipped, 5 next, 3 optional, 16 deferred)` sums to 84; the 5 `next` = 2 flipped (Q-60/Q-62 steps) + 3 new endorsed-core steps; `deferred` stays 16 (two flipped out, two new folded in); open questions drop 7 -> 5 as Q-64/Q-65 leave `exploring`.
