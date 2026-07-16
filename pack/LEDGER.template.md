# Review ledger: <task>

<The review loop's transient working state, separate from the plan. Keep it in a file tracked in version control beside the plan (`docs/plans/<task>.ledger.md`) and commit it, so it survives the orchestrator losing context and travels across sessions; delete it (a committed deletion) when the task closes. Durable decisions do not live here; they fold into the plan. This ledger is a narrative and resume artifact only; it holds no machine-parsed tables. When instrumentation is on, the structured per-round data (the round log the mechanical tooling reads) lives in `docs/metrics/workflow.jsonl`, not here. Prose is not hard-wrapped.>

Task: <what is under review, with the diff range or the before and after commit hashes where relevant>.

## Round records

<Per-round narrative, appended as the loop runs. For each round record, in prose: what was reviewed; the artifact's risk classification (`low_risk` needs one clean round to converge, `risky` / high-blast-radius needs two), noted when the artifact's review loop opens; which reviewers and which separate triager ran, with their findings-file paths under `docs/plans/<task>.reviews/`; the verdicts; the round outcome (`clean`, or new valid findings); and the convergence decision (the consecutive-clean streak versus the required count for that risk class, or an escalation when the total-round cap is reached). The orchestrator counts the streak and the total-round count from this narrative, so keep it current and committed; it is the audit trail a re-spawned orchestrator reads, and it survives a compaction. When instrumentation is on, the orchestrator ALSO appends a `round` record for the same round to `docs/metrics/workflow.jsonl`; the core counting reads this narrative, not that log.>

## RESUME STATE (compaction checkpoint, read this first)

<The first thing to read on resume. State where the work is: the current step and its status, what is complete, what is next, any open questions awaiting the human, and any workflow rules being applied that are not obvious from the code. Flush and commit this before a compaction (see the checkpoint procedure in `AGENTS.md`).>
