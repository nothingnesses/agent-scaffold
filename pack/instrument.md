## Instrumentation (metrics logging)

Instrumentation is enabled for this project (it was scaffolded with `--instrument`). While running the workflow, the orchestrator records calibration metrics so the round constants can be tuned from real use. Append one JSON object per event, one object per line (JSON Lines), to `docs/metrics/workflow.jsonl`, creating `docs/metrics/` if it does not exist; the file is committed and accumulates across tasks, so never rewrite past lines. Each record has a `type` field plus the fields for that type. Every record also carries `task` (a string naming the plan/ledger task, so records can be grouped per task) and may carry `ts` (a timestamp string, ISO 8601 recommended):

- `type: "round"` (one per review round): `artifact`, `phase` (`plan_review`, `work_review`, or `acceptance`), `changed_since_prev` (boolean), `outcome` (`clean` or `new_valid`), `valid_findings` (count), `severities` (list of severity names on the four-level `low`/`medium`/`high`/`critical` scale, for example `["high", "low"]`), and `consecutive_clean` (the streak after this round).
- `type: "escalation"` (one per total-round-cap escalation): `artifact` and `human_decision` (`decision` if the human changed course, `resume` if they just resumed).
- `type: "dismissal_recheck"` (one per re-checked high or critical dismissal): `artifact` and `result` (`upheld` or `overturned`).
- `type: "intake"` (one per human interrupt): `classification` (`trivial` or `non_trivial`) and `replanned` (boolean: whether a trivial call later had to be re-planned, that is, a misclassification).

The log can be checked against this schema with `agent-scaffold validate`, which exits non-zero and reports any malformed record.

This section is present only because instrumentation was enabled; a scaffold without `--instrument` omits it entirely.
