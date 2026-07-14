# Triage verdicts: consolidating plan-maintenance change

Adjudicated against the current (uncommitted) `docs/plans/agent-scaffold.md`.
Each finding verified directly in the file. The Documentation Protocol requires
Open-Questions queue items to be a terse ask + status + pointer, with decision
detail living in the STEP (not duplicated in the queue); Project Principles are
numbered 1-7.

## Finding 1 (reviewer: Medium) - Success Criteria contradict on where the kickoff prompt lives

Verdict: VALID (medium).

Justification: Line 472 lists onboarding as "a \"Getting started, for the human\"
section with an editable kickoff prompt", which reads as the prompt being embedded
in that section. The new line 477 and the revised `human-onboarding` step (lines
304-308, and `user-prompts-dir` at line 306) explicitly decide the opposite: the
kickoff prompt "does NOT live inline in `pack/AGENTS.md`", moves to
`.agents/user-prompts/`, and the "Getting started" section "becomes a thin trigger
that points to that directory". Line 472 was left unchanged and now disagrees with
the adopted design. Because Success Criteria are the acceptance checklist, a literal
acceptance check of line 472 would either fail a correctly-implemented pointer-only
section or push the implementer to re-embed the prompt and break the revised design.
Medium is correct: this is a live acceptance-checklist contradiction with a
functional consequence, not mere prose polish.

Fix: revise line 472 so onboarding is described as the "Getting started" section
pointing to the editable kickoff prompt in `.agents/user-prompts/`, aligning it with
lines 304-308 and 477.

## Finding 2 (reviewer: Medium) - Q-17 queue item duplicates its step's content

Verdict: VALID (low; corrected down from medium).

Justification: Q-17 (line 95) carries the inline parenthetical "(clean-tree-before-writer,
commit-before-delete, own-files-only formatting, orchestrator recovery protocol,
validation-in-scratch)", which enumerates exactly the five bullets the
`file-safety-rules` step details (Clean-tree-before-writer, Commit-before-delete,
Format only your own files, Orchestrator recovery protocol, Validation-in-scratch).
Every other queue item (Q-1..Q-16) is a terse ask + status + pointer with detail
left to the step, so Q-17 breaks the Documentation Protocol by copying the step's
substance into the queue, creating two places to keep the five-rule list in sync.
I correct the severity to low: the queue item still ends with an accurate pointer
("Decision and reasoning in `file-safety-rules`"), so the authoritative source is
unambiguous and the realistic harm is bounded doc-drift rather than a functional
defect, which sits below the Finding 1 acceptance-checklist contradiction.

Fix: trim Q-17 to the terse ask ("the git-durability-and-recovery baseline for
writer agents") plus status and pointer; drop the five-rule enumeration and let it
live only in the `file-safety-rules` step.

## Finding 3 (reviewer: Low) - Q-18 restates the decided tier order

Verdict: VALID (low).

Justification: Q-18 (line 96) carries "(container, else worktree, else the
file-safety discipline)", which restates the decided preference order that the
`agent-isolation` step already records ("(1) container ... (2) worktree ... (3) the
`file-safety-rules` discipline"). This is the same queue-terseness class as Q-17:
decided detail leaking into the queue rather than living only in the step. It is
milder because the parenthetical is short and partly reads as defining the term
"capability-tiered", and the pointer to `agent-isolation` is intact, so low is
correct.

Fix: optionally trim the parenthetical to keep the ask terse (e.g., "isolation
tiering: whether to run writer agents under capability-tiered isolation"), leaving
the tier order to the `agent-isolation` step; low priority.

## Finding 4 (reviewer: Low) - `findings-files` "share ONE record schema" is loose and its cluster membership disagrees with `state-schema`

Verdict: VALID (low).

Justification: `findings-files` (line 326) says the three-member cluster
(`findings-files`, `ledger-template`, `state-schema`) "share ONE record schema" but
then enumerates three distinct schemas (the finding schema, the round-outcome
schema, and the projection `state-schema` validates), so "ONE record schema"
overstates what follows. Separately, `state-schema` (line 338) names a different
set for the shared schema, "shared with `ledger-template` (the round-outcome
schema), `findings-files` (the finding schema), and `human-review-queue` (the
decision-queue item schema)", adding `human-review-queue`, which the `findings-files`
three-member cluster omits. The two descriptions of the shared-schema family are not
aligned. Low is correct: both convey the same intent (each record type defined once
and referenced), so the impact is wording/consistency, not a design defect.

Fix: reword `findings-files` line 326 to state the cluster shares the discipline of
"one schema per record type, defined once and referenced" (not one shared schema),
and align the membership between the two steps (either name `human-review-queue`
consistently or scope each list to the schemas it actually references).

## Summary

- Finding 1: VALID, medium.
- Finding 2: VALID, low (corrected from medium).
- Finding 3: VALID, low.
- Finding 4: VALID, low.

Four valid, none dismissed. No high/critical dismissals, so no backstop re-check is
triggered.
