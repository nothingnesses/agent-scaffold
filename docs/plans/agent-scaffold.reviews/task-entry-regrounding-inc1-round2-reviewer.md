# Round 2 re-review: task-entry-regrounding-inc1 (Q-53 Part A)

Reviewer: fresh independent adversarial reviewer, Round 2 (post-fix).
Scope: diff main (9b1ea51) .. impl/ter-inc1 HEAD (ba64e89); two commits a1b1262 (add) + ba64e89 (fix). Five files: pack/AGENTS.md, pack/prompts/orchestrator.md, and the three generated copies (AGENTS.md, .agents/AGENTS.reference.md, .agents/prompts/orchestrator.md).

## Verdict: CLEAN

No findings. Both Round 1 findings are correctly fixed, no regression, no new duplication, tests pass, style compliant.

## What I probed

1. I1-1 fix (orchestrator pointer duplication). pack/prompts/orchestrator.md:23 dropped the parenthetical four-element list "(what it is, why it exists, the cited evidence, and what you are about to do)" that duplicated the AGENTS.md subsection. The pointer still names the discipline ("task-entry re-grounding in `AGENTS.md`"), the trigger ("Before starting each step (and again on resume)"), the sourcing ("brief the step from durable artifacts"), and the gate ("push it for a go/no-go per the human-input contract, scaled to stakes, rather than restating that discipline here"). No necessary meaning lost; the element list now lives only in the AGENTS.md subsection (pack/AGENTS.md:104). No self-contradiction.

2. I1-2 fix (dangling receipt reference). pack/AGENTS.md:104 now cites "a decision by `q_id` (the resolved decision recorded in the plan's Open Questions section, and, when instrumentation is on, its `type: "decision"` round-log record carrying the human's `chosen`)". The always-present part points at the Open Questions resolved decision, which exists in every scaffold (Open Questions queue is always present per the Checkpoints rule; pack/AGENTS.md:61 "Decisions live in the plan's Open Questions queue"), so a non-instrumented scaffold has no dangling reference. The round-log receipt is now qualified "when instrumentation is on", matching the existing pattern at pack/AGENTS.md:61/:63/:69 ("When instrumentation is on ... ALSO appends a structured `round` record ..."). The `type: "decision"` record and its `chosen` field are real (defined at pack/instrument.md:9: q_id, options, recommendation, chosen; also src/workflow.rs W4 check), so the qualified reference is accurate, not merely non-dangling. The Instrumentation heading is NOT named (rejected sub-claim correctly omitted).

3. REFERENCE-NOT-RESTATE (Principle 8). The subsection and pointer cite the human-input contract, checkpoint, durability, and receipt machinery by handle rather than duplicating it. The fix removed a duplication (I1-1) and introduced none. No new second source.

4. Dogfood regen consistency. Changed passage on pack/AGENTS.md:104 is byte-identical to AGENTS.md:104 and .agents/AGENTS.reference.md:104 (diff empty). pack/prompts/orchestrator.md is byte-identical to .agents/prompts/orchestrator.md (full-file diff empty). Regen ran cleanly; no partial or stale copy.

5. Regression. `just test` passes: 324 passed / 0 failed in the bins suite plus all integration suites (0 failed across the run). No checks::tests worktree index.lock flake observed. Only the expected 5 files changed (git diff --name-only main..HEAD); no unrelated files.

6. Style. Added lines are ASCII-only (no non-ASCII match); no em-dashes, en-dashes, or double-hyphen-as-dash. Bullets/prose unchanged in structure; prose not hard-wrapped (not raised as a finding per instruction). Line length not raised.
