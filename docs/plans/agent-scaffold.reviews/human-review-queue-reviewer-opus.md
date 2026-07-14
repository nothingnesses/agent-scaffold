# Reviewer findings: `human-review-queue` (lens: correctness and completeness)

Commit range reviewed: `3599756..fbd3ffa`, pack source only (`pack/AGENTS.md`, `pack/plan-template.md`, `pack/prompts/orchestrator.md`, `pack/user-prompts/kickoff.md`). Generated mirrors under `.agents/`, root `AGENTS.md`, and `docs/plans/TEMPLATE.md` ignored per instructions.

## Decision coverage (all four decisions implemented)

- Q-10 (living human-decision queue): implemented. `pack/AGENTS.md` line 231 ("Checkpoints") defines the queue with stable id, one-line ask, status, context pointer, resolved-marked-not-deleted, and the push-at-checkpoint rule; `pack/plan-template.md` lines 32 and 36 adopt the item format with the exact status set `open` / `decided -> folded into <slug>` / `superseded`. The template's old "removed once resolved" wording (a direct contradiction of mark-not-delete) is replaced; no lingering "remove/delete resolved question" wording survives elsewhere in the pack.
- Q-21 (step-boundary checkpoint cadence): implemented consistently across all three required locations. `pack/AGENTS.md` line 233, `pack/prompts/orchestrator.md` line 97, and `pack/user-prompts/kickoff.md` all state default report-and-continue plus the gate and run-autonomously options, with matching wording.
- Q-9 follow-up (re-strengthen decision-duty to push model): implemented. `pack/AGENTS.md` line 16 replaces the interim pull wording ("check that section as the work proceeds") with the push model ("at each checkpoint the orchestrator raises the open items with you").
- Q-23 (thin pointer to compaction-prep / resume): implemented. `pack/AGENTS.md` line 18 names `compaction-prep.md` (paste before a compaction) and `resume.md` (paste to continue after), without restating the procedure.

Checkpoint definition alignment: `AGENTS.md` line 233 and `orchestrator.md` line 97 both define a checkpoint as the same three cases (step boundary, compaction-prep flush, escalation) and both mandate the queue push; the definitions line up. No contradiction found with the "Preventing relitigation" rule (line 227: individual findings never in Open Questions) since the queue is scoped to "decisions the human owns," nor with the intake/escalation paragraphs or the human-input contract (line 130). No unowned duty or dangling reference found: the orchestrator owns the push and the cadence, and "the compaction checkpoint below" resolves to the "Checkpoint and resuming after context loss" section.

## Findings

### R1 - "checkpoint" used ungloss in the human-facing onboarding (severity: low)

Location: `pack/AGENTS.md` line 16 (Getting started, for the human): "at each checkpoint the orchestrator raises the open items with you."

Problem: this Q-9-follow-up rewording introduces the term "checkpoint" into the human-facing onboarding section. It is the first occurrence of "checkpoint" in the document; the term is only defined ~215 lines later (line 231, "Checkpoints ..."). A newcomer reading Getting started meets an undefined term. This is the same class of issue the `human-onboarding` round-1 review already caught and fixed (per the plan, an "orchestrator"/"checkpoint" jargon gloss), so it is a mild regression against a settled outcome. Impact is minor because the surrounding sentence still conveys the actionable point (the orchestrator raises open items with you) without the reader needing the precise definition.

Principle implicated: 20 (make documentation self-contained).

### R2 - dedicated compaction section is silent on the queue push (severity: low)

Location: `pack/AGENTS.md` lines 277-294 ("Checkpoint and resuming after context loss"), specifically the before-context-loss bullet (lines 282-287), which lists flushing the plan, ledger, and Open Questions queue and committing, but does not mention pushing the queue's open items to the human.

Problem: the new "Checkpoints" paragraph (line 233) asserts "The compaction checkpoint below ... take[s] the same queue push," and `orchestrator.md` line 97 lists a compaction-prep flush as a checkpoint that pushes open items. But the dedicated compaction section itself describes only flush-and-commit, not the push. A reader landing on that section in isolation (the likely case during a real compaction) would not learn the push applies there. The linkage exists via the forward-reference from line 233, and the orchestrator prompt (the actor) is unambiguous, so this is a completeness/cross-linking nit rather than a contradiction.

Principle implicated: 16 (one source of truth) / 20 (self-contained), minor.

## Severities with no findings

- Critical: none.
- High: none.
- Medium: none.

Both findings are low. The change fully implements Q-10, Q-21, Q-9-follow-up, and Q-23, with the cadence stated consistently across the kickoff prompt, AGENTS.md, and orchestrator.md, and no decided item left unimplemented.
