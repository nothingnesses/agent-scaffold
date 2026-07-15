# Review ledger: <task>

<The review loop's transient working state, separate from the plan. Keep it in a file tracked in version control beside the plan (`docs/plans/<task>.ledger.md`) and commit it, so it survives the orchestrator losing context and travels across sessions; delete it (a committed deletion) when the task closes. Durable decisions do not live here; they fold into the plan. Prose is not hard-wrapped.>

Task: <what is under review, with the diff range or the before and after commit hashes where relevant>.

Artifact classification: <for each artifact reviewed, record low-risk (one clean round to converge) or risky / high-blast-radius (two clean rounds), classified once when its review loop opens; this fixes the required consecutive-clean count for that artifact>.

## Round summaries

<One row per review round for an artifact. Outcome is exactly one of `clean` or `new valid findings` (a round with zero findings counts as clean). The total-round count and the consecutive-clean streak are countable from this table; both reset to zero when the loop moves to a new artifact or step. There is no cross-round finding identity: a round is scored by its outcome, not by tracking a finding across rounds. The Consecutive clean column is a convenience view derived from the Outcome column, which is authoritative.>

| Round | Artifact | Changed since prev | Outcome | Valid findings | Consecutive clean |
| --- | --- | --- | --- | --- | --- |
| <n> | <what was reviewed> | <yes / no / n.a.> | <clean \| new valid findings> | <count, e.g. 2 (1 medium, 1 low)> | <streak> |

## Findings

<One entry per finding raised in a round, for the relitigation record. Severity is the four-level scale (`low` / `medium` / `high` / `critical`). Verdict is the triager's (`valid` / `invalid`). Action is `fixed in <commit>`, `dismissed because <reason>`, or `accepted residual risk`. The id is a within-round label for reference (for example R1, S2, F3), not a cross-round identity. The full findings live in each agent's file under `docs/plans/<task>.reviews/`; cite those file paths in the Round records section below rather than copying their text, so this table stays a compact index.>

| ID  | Round | Severity | Triager verdict | Reasoning | Action |
| --- | ----- | -------- | --------------- | --------- | ------ |

## Round records

<Per-round narrative, appended as the loop runs: what was reviewed, which reviewers and which separate triager ran (with their findings-file paths), the verdicts and the round outcome, and the convergence decision (streak versus the required consecutive-clean count, or an escalation at the total-round cap). Keep this current and committed; it is the audit trail a re-spawned orchestrator reads.>

## RESUME STATE (compaction checkpoint, read this first)

<The first thing to read on resume. State where the work is: the current step and its status, what is complete, what is next, any open questions awaiting the human, and any workflow rules being applied that are not obvious from the code. Flush and commit this before a compaction (see the checkpoint procedure in `AGENTS.md`).>
