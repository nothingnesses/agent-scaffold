# Reviewer A findings: consolidating plan-maintenance change

Lens: consistency and well-formedness of the uncommitted change to
`docs/plans/agent-scaffold.md` (diff against HEAD). Decisions themselves were
not re-litigated; only whether the plan is internally consistent and well-formed.

## Critical

- None found.

## High

- None found.

## Medium

- **Success Criteria contradict each other on where the kickoff prompt lives.**
  Location: `## Success Criteria`, line 472 (pre-existing, left unchanged) versus
  line 477 (new). Line 472 states the human-interface layer is present as
  "onboarding (a \"Getting started, for the human\" section with an editable
  kickoff prompt ...)", i.e. the kickoff prompt sits inside the Getting started
  section. Line 477 (new) states "Human-invoked user prompts (the kickoff prompt
  and the compaction-prep prompt) live in their own `.agents/user-prompts/`
  directory ... and stay thin triggers". The revised `human-onboarding` step
  (line 306) is explicit that "the kickoff prompt does NOT live inline in
  `pack/AGENTS.md` ... the \"Getting started\" section becomes a thin trigger that
  points to that directory rather than carrying the prompt text". The planner
  added the new criterion and revised the step but did not update the older
  criterion, so the two Success Criteria now disagree on the design. Why it
  matters: Success Criteria are the acceptance checklist; a literal acceptance
  review of line 472 would mark a correctly-implemented (pointer-only) Getting
  started section as failing, or force the implementer to re-embed the prompt and
  violate the revised design. This is the class of internal-agreement break the
  review was asked to check.

- **`Q-17` queue item duplicates its step's content (queue terseness).**
  Location: Open Questions, `Q-17` (line 95). The item carries an inline
  parenthetical enumerating the five sub-rules: "(clean-tree-before-writer,
  commit-before-delete, own-files-only formatting, orchestrator recovery
  protocol, validation-in-scratch)". These are exactly the five bullets the
  `file-safety-rules` step details (line 345 onward). Every other queue item
  (`Q-1`..`Q-16`) is a terse ask + status + pointer with the detail left to the
  step; `Q-17` breaks that pattern by copying the step's substance into the
  queue. Why it matters: this is the queue-terseness class the review was told to
  watch, and the duplication creates two places to keep the five-rule list in
  sync.

## Low

- **`Q-18` queue item restates the decided tier order (milder queue terseness).**
  Location: Open Questions, `Q-18` (line 96): "capability-tiered isolation
  (container, else worktree, else the file-safety discipline)". The parenthetical
  repeats the decided preference order that the `agent-isolation` step already
  records (line 358: "(1) container ... (2) worktree ... (3) the
  `file-safety-rules` discipline"). Lighter than `Q-17` because it reads as part
  of naming the ask, but it still duplicates step content rather than pointing to
  it.

- **`findings-files` "share ONE record schema" is loosely worded and its cluster
  membership disagrees with `state-schema`.** Location: `findings-files` step,
  line 326, and `state-schema` step, line 338. Line 326 says the structured-record
  cluster is three members ("Grouped with `ledger-template` and `state-schema`
  ... the three share ONE record schema") yet immediately enumerates three
  distinct schemas ("the finding schema ... the round-outcome schema ... and the
  projection `state-schema` validates"), so "share ONE record schema" overstates
  what follows it. Separately, `state-schema` (line 338) names a different
  sharing set of four artifacts, adding `human-review-queue`
  ("shared with `ledger-template` ..., `findings-files` ..., and
  `human-review-queue` (the decision-queue item schema)"), which the
  `findings-files` three-member cluster omits. The two descriptions of the shared
  schema are not aligned. Low severity: both convey the intent (each record type
  defined once and referenced), but the "one schema" claim and the membership
  differ between the two steps.

## Checks that passed (no findings)

- **Queue-vs-step contradiction:** each of `Q-14`..`Q-18` points to a step whose
  detail records the decision as decided ("Decided (`Q-14`/`Q-15`/`Q-16`/`Q-17`/
  `Q-18`, adopted/confirmed by the human)"); no target step still frames its item
  as an open recommendation or open sub-question.
- **Step well-formedness:** all five new slugs (`file-safety-rules`,
  `agent-isolation`, `findings-files`, `user-prompts-dir`, `compaction-prep`)
  appear in the Roadmap table (lines 120-127) each with a status, and each has a
  matching slug-keyed detail block. No new detail block repeats the Roadmap
  status label (each opens with "Decided ...", not "Not started ...").
- **Principle citations:** new citations are Principle 1 (one source of truth:
  `human-onboarding` line 304, `findings-files` line 331, `state-schema` line
  338, `user-prompts-dir` line 369) and Principle 2 (minimal: `agent-isolation`
  line 358). All use the plan's own 1-7 numbering; no citation above 7 and no
  AGENTS.md-numbering leak. Plan Principle 1 = cleaner architecture / one source
  of truth (line 18), Principle 2 = minimal by default (line 19), consistent with
  the citations.
- **Status line / Roadmap / Success Criteria agreement (aside from the medium
  above):** the status line's next step (`triager-independence`) matches the
  Roadmap `next` row; the five new steps named in the status line match the five
  new Roadmap rows and detail blocks; the queue "carries `Q-14` through `Q-18` as
  decided" matches the queue. The four new Success Criteria (lines 474-477) map to
  the four durability/isolation/findings/user-prompts steps.
- **`human-onboarding` revision vs `user-prompts-dir`:** line 306 (kickoff prompt
  moves to `.agents/user-prompts/`, Getting started becomes a thin trigger) and
  line 369 (`user-prompts-dir` REVISES `human-onboarding` the same way) agree,
  and `user-prompts-dir` owns the directory/manifest/moved prompt while
  `human-onboarding` builds against a pointer.
- **Umbrella block:** "Durability, recovery, isolation, and user prompts" (line
  340) is a `###` heading explicitly flagged "not itself a Roadmap entry",
  matching the existing umbrella pattern; it correctly names the four steps below
  and notes `findings-files` sits above with the structured-record cluster.
- **Style:** ASCII throughout (`->`, hyphens, no em/en-dashes or unicode);
  British spelling in the new text ("visualisation", "behavioural",
  "prioritising"); no AI-filler in the new blocks (the earlier
  "surfaces (pushes)" in `human-onboarding` is now "pushes"). The known
  pre-existing "recommendation leans to (a)" in `greenfield-flake` was not
  flagged per instruction.
