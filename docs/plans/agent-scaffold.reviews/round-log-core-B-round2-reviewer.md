# Reviewer findings: round-log-core increment B, round 2 (first confirming round)

Scope: full increment-B diff `eba3c99..HEAD` (a9e08b0 reshape + 1fada25 fixes), branch `impl/round-log-core`, HEAD 1fada25, reviewed in worktree `.claude/worktrees/round-log-core`.

Verdict: clean. All four round-1 fixes plus the trivial comment fix landed as described, the core convergence-accounting semantics are preserved, and I found no new defect at any severity.

## Round-1 fixes confirmed

- T1 (high): RESUME STATE comment in `pack/LEDGER.template.md:13` (and identical `.agents/LEDGER.template.md`) now reads as a pointer into the plan's current state (names the active step and artifact under review, refers to the plan's Roadmap and Open Questions queue for their status) plus the non-plan-derivable transient in-flight state (current round number, consecutive-clean streak, locked in-round context such as a pending dismissal re-check or running debate). It explicitly states "not a restatement of the Roadmap or the Open-Questions queue; do not copy step statuses or decisions here." The old "the current step and its status, what is complete, what is next" restatement instruction is gone. Confirmed.
- T2: `pack/AGENTS.md:55` "Preventing relitigation" enumeration now lists, for each round record: what was reviewed, the artifact's risk classification, the reviewers and separate triager with findings-file paths, the verdicts, each round's outcome, and the running consecutive-clean streak. Risk classification and the streak are both present. Confirmed (identical in `AGENTS.md` and `.agents/AGENTS.reference.md`).
- T3: `pack/LEDGER.template.md:9` Round-records comment now includes "whether the artifact changed since the previous round" in the per-round prose enumeration. Confirmed (identical in `.agents/LEDGER.template.md`).
- T4: risk tokens in the template prose comments are natural language ("low-risk", "risky or high-blast-radius"); grep for backticked forms (`low-risk`, `risky`, etc.) returned nothing. The only backticks around "round" refer to the JSONL record type, which is correct. Confirmed.
- Trivial: `src/plan.rs:16-20` module comment no longer implies ledger round parsing. It now says the ledger is a narrative artifact with no machine-parsed tables and the structured round log is handled by `metrics.rs`, "so neither is parsed here." Confirmed.

## Core correctness re-confirmed

- Two-way clean/new_valid partition preserved (orchestrator.md step 2; AGENTS.md convergence section). A round with zero findings counts as clean.
- Per-artifact consecutive-clean streak plus total-round count, both reset to zero on new artifact/step (AGENTS.md "Tracking progress"; orchestrator.md step 3).
- Dismissal-recheck amendment preserved: on overturn, amend the already-written round outcome in the narrative from clean to new-valid, reset streak to zero, send back to fix, spawn another round (orchestrator.md step 2; AGENTS.md backstop paragraph).
- Total-round cap (default five) preserved with escalation and counter-reset-on-resume semantics (AGENTS.md; orchestrator.md step 3).
- Orchestrator counts from the ledger NARRATIVE: stated consistently ("counts the consecutive-clean streak and the total-round count from that narrative", "recomputed from the narrative"). The JSONL is framed strictly as the opt-in "also append" superset ("When instrumentation is on, the orchestrator ALSO appends... the core counting reads the narrative and does not depend on that log").
- Opt-in preserved: instrument.md, {{instrument}}, --instrument are untouched (not among the 8 files in the diff).
- No leftover round-summary/findings-table references: grep across AGENTS.md, pack/AGENTS.md, .agents/AGENTS.reference.md, both templates, and both orchestrator prompts returned nothing. The removed template sections (Artifact classification line, Round summaries table, Findings table) leave no dangling references; classification folded into the Round-records narrative.
- De-dup rule internally consistent: the "references the plan rather than restating", "reference a finding by its file path rather than copying", and "no machine-parsed tables" wording is aligned across AGENTS.md, orchestrator.md, and the template. The three synced pairs are byte-identical: AGENTS.md == .agents/AGENTS.reference.md; pack/LEDGER.template.md == .agents/LEDGER.template.md; pack/prompts/orchestrator.md == .agents/prompts/orchestrator.md.

## Mechanicals

- `just test`: 96 passed, 0 failed.
- `just clippy`: clean, no warnings.
- `just scaffold-self` then `git status --short`: empty (idempotent).
- `validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl`: both valid (54 records; 42 steps, 35 open-questions items).

## Findings by severity

- critical: none.
- high: none.
- medium: none.
- low: none.

No new defects. This confirming round is clean.
