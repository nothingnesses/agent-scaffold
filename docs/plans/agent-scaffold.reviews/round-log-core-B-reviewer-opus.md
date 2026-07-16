# Reviewer findings: `round-log-core` increment B (correctness + consistency lens)

Reviewer: opus. Branch `impl/round-log-core`, HEAD `a9e08b0`, diff range `eba3c99..HEAD`.
Artifact: increment B (ledger template reshape + convergence-accounting prose rewrite + plan/ledger de-dup rule). Risk: risky (rewrites the core convergence-counting rule).

## Summary

Counts: critical 0, high 0, medium 1, low 0. One real inconsistency (the implementer-flagged RESUME STATE tension), confirmed. Q-1 convergence semantics are preserved; opt-in is preserved; mechanicals are green; regenerated files are in sync.

## Findings

### M1 (medium): the RESUME STATE authoring comment contradicts the de-dup rule this increment adds

Location: `pack/LEDGER.template.md:13` and `.agents/LEDGER.template.md:13` (the `## RESUME STATE` section comment).

The increment folds in a new de-dup rule, stated in three places it edited (`AGENTS.md:55` / `pack/AGENTS.md:55`, `pack/prompts/orchestrator.md:31` / `.agents/prompts/orchestrator.md:31`, and the template intro comment at line 3): "its RESUME STATE is a pointer into that durable state plus the non-plan-derivable transient in-flight state, not a restatement of the Roadmap." But the template's own `## RESUME STATE` authoring comment (unchanged by the increment) still instructs the author to write exactly the restatement the rule forbids: "State where the work is: the current step and its status, what is complete, what is next, any open questions awaiting the human ...". Three of those four items are Roadmap/Open-Questions restatements ("the current step and its status", "what is complete, what is next", "any open questions awaiting the human"); only "any workflow rules being applied that are not obvious from the code" is the non-plan-derivable transient the new rule keeps.

This is the residual tension the implementer flagged, and it is real. It is an internal contradiction inside the increment's own deliverable: the template is the operative artifact authors copy, so a contradictory authoring comment means authors keep restating the Roadmap in RESUME STATE, defeating the discipline this increment is meant to establish. It does not break the counting mechanism (hence not high), but it directly negates a stated goal of the increment (hence more than a nit).

Note on scope: the plan (`docs/plans/agent-scaffold.md:599`) folds the de-dup DISCIPLINE into `round-log-core` now, while the full RESUME-STATE slimming and repointing of `resume.md`/`compaction-prep.md` is deferred to `state-queries`. Because the RULE TEXT lands now, the template's authoring comment should be made consistent now (a one-line edit in a file already rewritten this increment), or the deferral should be made explicit. Leaving it as-is is an operative inconsistency, not a stylistic one. The checkpoint procedure at `AGENTS.md:87-89` is already consistent with the new rule (it names the plan's Status line as the resume anchor), which isolates the contradiction to this one template comment.

## Confirmations (no finding)

Q-1 substance preserved (only the SOURCE moved table -> narrative):
- Two-way `clean` / `new valid findings` partition: preserved (`AGENTS.md:47-48` bullets; orchestrator step 2; template Round records comment). "A round where the reviewers report zero findings counts as clean" retained.
- Per-artifact consecutive-clean streak: preserved; "counts the consecutive-clean streak ... from that narrative", "per-artifact, not per-task".
- Reset on new artifact/step: preserved ("both counts reset to zero when the loop moves to a new artifact or step", `AGENTS.md:55`, orchestrator step 3).
- Dismissal-recheck amendment: preserved (orchestrator step 2: "amend the already-written round outcome in the narrative from clean to new-valid, reset the consecutive-clean count to zero ..."; `AGENTS.md:49` backstop paragraph unchanged in substance).
- Total-round cap (default five) escalation, and the "convergence check applies first if a round both caps and converges" rule: preserved (`AGENTS.md:48`, orchestrator step 3).
- Counting from prose is unambiguous: the template pins "For each round record, in prose: ..." (one discrete record per round), so the total = number of round records and the streak = trailing run of `clean`; the orchestrator also records the running streak per round, making it directly readable and self-checking. No new ambiguity.

Opt-in preserved:
- `--instrument`, `{{instrument}}` (`pack/AGENTS.md:104`), and `pack/instrument.md` are untouched (not in the diff; `{{instrument}}` placeholder intact).
- Core counting reads the narrative, not the JSONL: stated consistently ("the core counting reads the narrative and does not depend on that log"; orchestrator step 1: "the counting below reads the narrative, not that log").
- JSONL framed as opt-in structured superset: every mention is conditionally gated ("When instrumentation is on, the orchestrator ALSO appends ..."). No accidental always-on framing. The core prose's conditional forward-reference to `docs/metrics/workflow.jsonl` in the non-instrumented base doc is clearly gated and consistent with `instrument.md`'s `type: "round"` schema (`risk_class` = `low_risk`/`risky` matches the template's `low_risk`/`risky` tokens).

Consistency:
- No leftover table references: grep of `pack/ .agents/ AGENTS.md` for "Round summaries", "round-summary", "Artifact classification", "## Findings", "Consecutive clean", "Valid findings", "Append a row" returns nothing. The only "table" hit is "the status table described in the plan's Documentation Protocol" (the Roadmap status table, correct).
- De-dup rule does not contradict existing sections: it aligns with the Roadmap-single-source-of-status line (`AGENTS.md:55`), the Open-Questions queue (`AGENTS.md:63`), and the checkpoint/resume anchor (`AGENTS.md:87-89`).
- Regeneration in sync: `just scaffold-self` then `git status --short` is empty (`AGENTS.md`, `.agents/AGENTS.reference.md`, `.agents/prompts/orchestrator.md`, `.agents/LEDGER.template.md` match their `pack/` sources).
- The live ledger `docs/plans/agent-scaffold.ledger.md` still contains the `## Round summaries` and `## Findings` tables; this is correctly out of scope for B (increment C, orchestrator-owned, slims the live ledger), not a finding.

Mechanicals:
- `just test`: 96 passed, 0 failed.
- `just clippy`: clean (finished, no warnings).
- `validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl`: exit 0; "54 records, valid" and "42 steps, 35 open-questions items, valid".
