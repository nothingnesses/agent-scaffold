# Round 3 reviewer findings: round-log-core increment B

Reviewer: independent confirming reviewer (sonnet, round 3).
Artifact: increment B diff `eba3c99..HEAD` (branch `impl/round-log-core`, HEAD `1fada25`).
Risk classification: risky / high-blast-radius (two consecutive clean rounds required to converge). Round 2 was clean; this is the deciding round.

## Mechanicals

- `just test`: 96 pass, 0 failed.
- `just clippy`: clean (no warnings).
- `just scaffold-self` then `git status --short`: empty; the self-scaffolded copies are in sync with the pack.
- `validate --plan docs/plans/agent-scaffold.md`: 42 steps, 35 open-questions items, valid.
- `validate --metrics docs/metrics/workflow.jsonl`: 54 records, valid.

All mechanicals pass.

## Content review

### 1. Narrative-based convergence rule: unambiguous and followable

The claim to verify: after a compaction, can an orchestrator recompute both the streak and the total-round count from the prose narrative, and are the original Q-1 semantics preserved?

The orchestrator.md step 1 enumerates exactly what each round record must contain, including "the running consecutive-clean streak" and "the outcome (clean, or new valid findings)". Step 3 says "Decide from the counts (recomputed from the narrative)" - "recomputed" makes clear the narrative is the source of truth, parallel to the old table's "Outcome column is authoritative" rule. After a compaction, an orchestrator reads the last round record to get the current streak (explicitly required there) and counts round records for the current artifact to get the total-round count. The artifact's risk classification is required to be noted "when its loop opens", establishing per-artifact boundaries in the narrative.

All Q-1 semantics are present and accounted for:
- Partition: `{clean, new-valid}` - preserved verbatim.
- Per-artifact streak: "The two review counts are per-artifact, not per-task...both counts reset to zero when the loop moves to a new artifact or step" - preserved.
- Reset on new valid findings: "a round with new valid findings resets the streak" - preserved.
- Dismissal-recheck backstop: unchanged text, guards dismissed high-or-critical findings before they count as clean.
- Total-round cap (default five): preserved in all locations.

No semantic drift found.

One note for the record (not a defect): the narrative records the streak redundantly in both the RESUME STATE ("the current round number, the consecutive-clean streak") and each round record. Step 3's "recomputed from the narrative" establishes the round records as the source of truth if they ever disagree. No explicit tie-breaker is stated, but "recomputed" is clear enough in context.

### 2. Opt-in genuinely preserved, no always-on framing

Every reference to the JSONL round log in all modified files is consistently conditional on "When instrumentation is on." This holds across `AGENTS.md`, `pack/AGENTS.md`, `.agents/AGENTS.reference.md`, `pack/prompts/orchestrator.md`, `.agents/prompts/orchestrator.md`, and both `LEDGER.template.md` files. The core narrative counting is explicitly stated as always-present and JSONL logging as the opt-in structured superset.

The `src/plan.rs` comment update correctly says the JSONL "is handled by `metrics.rs`, so neither is parsed here" - accurately framing it as an instrumentation-layer concern.

Opt-in is preserved with no always-on framing anywhere in the increment.

### 3. De-dup rule: internally consistent

The new rule states the plan owns step status (Roadmap) and decisions (Open Questions queue), and the ledger's RESUME STATE is a pointer into that durable state plus non-plan-derivable transient in-flight state (current round number, consecutive-clean streak, any locked in-round context).

Checked against:
- The RESUME STATE template's new wording: consistent ("pointer... plus the non-plan-derivable transient in-flight state... This is a pointer plus transient state, not a restatement of the Roadmap or the Open-Questions queue").
- The AGENTS.md checkpoint/resume procedure: consistent; it reads the plan first ("its Status line first"), then the ledger.
- The user-prompt files (`pause.md`, `compaction-prep.md`, `resume.md`): all defer to AGENTS.md by reference, no contradiction.
- The plan's Documentation Protocol and Roadmap-single-source rule: consistent.

No internal contradiction found.

### 4. Dangling references to removed tables

Checked for residual references to the removed sections ("Round summaries", "Findings" table, "Artifact classification:" line) across all modified pack and prompt files: none found. Specific before/after pairs verified clean:
- Old orchestrator.md "round-summary line" -> new "round outcome in the narrative": done correctly in both `.agents/prompts/orchestrator.md` and `pack/prompts/orchestrator.md`.
- Old "Append a row per finding, and a round-summary line" -> new "Record the round in the ledger's round-records narrative": replaced consistently in both locations.
- Old "amend the already-written round-summary outcome" -> new "amend the already-written round outcome in the narrative": replaced consistently in both locations.

The live ledger (`docs/plans/agent-scaffold.ledger.md`) still has the old `## Round summaries` and `## Findings` tables. This is expected: the rules say "never overwriting a live ledger" and the `round-log-core` step body explicitly assigns live-ledger slimming to increment C ("(C, orchestrator-owned) slim the live `docs/plans/agent-scaffold.ledger.md`...and this plan"). Not a defect in increment B.

Several plan step bodies also contain stale references to "round-summary lines" or "round-summary pipe-table" (at lines 289, 407, 560, 566 of `agent-scaffold.md`). These are all pre-existing (written before `Q-34` was resolved) or historically accurate (describing what was built at the time), and they are in the scope of increment C's plan cleanup. Not a defect in increment B.

### 5. Narrative specification completeness

The LEDGER.template.md's Round records instruction and orchestrator.md step 1 together enumerate all required fields. The template says "the convergence decision (the consecutive-clean streak versus the required count for that risk class, or an escalation when the total-round cap is reached)"; orchestrator.md step 1 says "the running consecutive-clean streak." These are consistent - the streak is the key quantity in the convergence decision. The orchestrator.md phrasing is a concise restatement, not an omission. Nothing is under-specified that would prevent recomputing the counts post-compaction.

## Verdict

No defects found at any severity level.

- Critical: none.
- High: none.
- Medium: none.
- Low: none.

All mechanicals pass. The narrative-based convergence rule is unambiguous and followable. Opt-in is preserved throughout. The de-dup rule is internally consistent. No dangling references to the removed tables appear in the modified files. The narrative specification is sufficient for post-compaction resumption.

This is a clean round.
