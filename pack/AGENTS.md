# Agent guidance

This is the canonical guidance for agents working in this repository. It is
harness-agnostic: any harness-specific file (for example `CLAUDE.md`) should
point here rather than duplicate it.

## Workflow

Roles are separated so no agent grades its own work. Where the harness supports
independent sub-agents, the orchestrator runs each role as a separate, isolated
agent: it spawns a fresh agent, hands it that role's prompt from
`.agents/prompts/`, and gives it only the context it needs, not another role's
reasoning or opinions. Where sub-agents are unavailable, one agent plays the
roles in sequence but still writes down each role's output, so the separation
holds on paper. Match the ceremony to the stakes: collapse roles for a trivial
change, keep them distinct for anything non-trivial or risky.

Roles and their prompts (in `.agents/prompts/`):

- Orchestrator (`orchestrator.md`). Owns the plan and its status, drives the
  phases, spawns the other roles and routes context to them, runs the review
  loop, and escalates to a human on impasse. It does not plan, implement, review,
  or triage itself.
- Planner (`planner.md`, with `clarifying-questions.md` and
  `open-questions-gate.md`). Drafts the plan.
- Reviewers (`reviewer.md`). Independently and adversarially review an artifact,
  assuming there are issues, and report each finding with a severity and concrete
  evidence. Prefer several reviewers with different lenses, and different models
  where available, since same-model reviewers share blind spots.
- Triager (`triager.md`). Judges the reviewers' findings on their evidence and
  severity and returns a verdict for each. The triager must not be the agent that
  produced the artifact under review.
- Implementer (`implementer.md`). Makes small, reviewable changes to satisfy the
  plan and the triager's valid verdicts, keeping the plan's status current.

Phases (the orchestrator drives these, spawning the role shown):

1. Front-load context. The relevant role reads the code and docs it needs before
   acting.
2. Plan. The orchestrator spawns a planner to draft a plan under `docs/plans/`
   from `docs/plans/TEMPLATE.md` (seed its Project Principles from this file's
   principles, in order, then the project's own, consolidating overlaps; record
   the implementation steps in the Roadmap and state the Success Criteria) and to
   resolve the open questions before implementation.
3. Review the plan, then triage. The orchestrator spawns reviewers on the plan,
   then a triager on their findings; the planner revises per the valid verdicts.
   Repeat per the convergence rule below, then start implementing.
4. Implement and review, step by step. While the plan's Roadmap has a pending
   step, the orchestrator spawns an implementer to make that step's change (small
   and reviewable), then spawns reviewers on it (give them the before and after
   commit hashes or the diff range) and a triager; the implementer fixes per the
   valid verdicts. Repeat the review per the convergence rule, then mark the step
   complete in the Roadmap and move to the next step.
5. Accept. When no pending steps remain, the orchestrator spawns reviewers for an
   acceptance review against the plan's Success Criteria. If the changes meet
   every criterion, the work is done. If not, each shortfall is a finding that
   goes back to planning (add or revise steps) or implementation.

Stop condition. The workflow is done when every step in the plan's Roadmap is
complete and an acceptance review confirms the changes meet the plan's Success
Criteria. Escalating to a human is not a stop: it is a request for a decision on
an impasse, after which the orchestrator applies the decision and resumes the
workflow where it paused.

Convergence (when the orchestrator ends one review loop and moves on; distinct
from the Stop condition above, which ends the whole workflow). After each
review-then-triage round, the orchestrator decides from the triager's verdicts:

- New valid findings this round: have the planner or implementer address them,
  then spawn another round (fresh reviewers, given the ledger) on the revised
  artifact.
- No new valid findings this round (every finding was dismissed, or was a ledger
  re-raise without new evidence): the review has converged. Move on, start
  implementing after a plan review, or mark the step complete and continue after
  a work review.
- Still-contested valid findings after a bounded number of rounds (default
  three): escalate to a human with the ledger for a decision, then apply it and
  resume. A valid finding may instead be resolved by consciously accepting its
  residual risk and recording that; an accepted risk does not block convergence.

Tracking progress. Two things are tracked, at two lifetimes. Step-level progress
(which implementation steps are done, in progress, or pending) lives durably in
the plan's Roadmap, the status table described in the plan's Documentation
Protocol; the implementer keeps it current. Round-level state (the review loop)
lives in the orchestrator's review ledger, which is transient.

Preventing relitigation (the ledger). The orchestrator keeps a review ledger for
the task, one row per finding: the round it was raised in, the triager's verdict,
the reasoning, and the action taken (fixed in `<commit>`, or dismissed because
`<reason>`). The orchestrator counts rounds from the ledger and applies the
convergence rule (a clean round ends the loop; the default of three contested
rounds triggers escalation). It hands the ledger to each new round under the
rule: do not re-raise a settled finding without new evidence that its verdict was
wrong. For a genuinely contested finding, the triager may hold a short debate, the
producer arguing it is invalid and a reviewer arguing it is valid, before ruling.
The ledger is transient working state, keep it as scratch notes (not in the
plan), discard it when the task closes, and never put individual findings in the
plan's Open Questions section; only durable decisions, the ones that change the
plan, fold into the plan's steps, and a folded decision reopens only by evidence
that beats its recorded reasoning.

## Principles

Follow these principles. They are numbered for reference, not priority.

{{principles}}
