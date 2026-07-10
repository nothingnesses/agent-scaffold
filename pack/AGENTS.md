# Agent guidance

This is the canonical guidance for agents working in this repository. It is
harness-agnostic: any harness-specific file (for example `CLAUDE.md`) should
point here rather than duplicate it.

## Workflow

Roles are separated so no agent grades its own work. Where the harness supports
independent sub-agents, run each role as a separate, isolated agent that sees
only the artifacts it needs, not the other roles' reasoning or opinions. Where it
does not, one agent plays the roles in sequence but still writes down each role's
output, so the separation holds on paper. Match the ceremony to the stakes:
collapse roles for a trivial change, keep them distinct for anything non-trivial
or risky.

Roles:

- Orchestrator. Owns the plan and its status, drives the phases, decides what
  context each role receives, enforces the review loop, and escalates to a human
  on impasse. It does not plan, implement, review, or triage itself.
- Planner. Drafts the plan (see phase 2).
- Reviewers. Independently and adversarially review an artifact, assuming there
  are issues, and report each finding with a severity and concrete evidence.
  Prefer several reviewers with different lenses, and different models where
  available, since same-model reviewers share blind spots.
- Triager. Judges the reviewers' findings on their evidence and severity and
  returns a verdict for each (valid or not, and why). The triager must not be the
  agent that produced the artifact under review.
- Implementer. Makes small, reviewable changes to satisfy the plan and the
  triager's verdicts, keeping the plan's status current.

Phases (the orchestrator drives these):

1. Front-load context. Read the relevant code and docs before acting.
2. Plan. For non-trivial work, the planner drafts a plan under `docs/plans/` from
   `docs/plans/TEMPLATE.md`. Seed its Project Principles from this file's
   principles (below), in order, then add the project's own after them,
   consolidating any overlap into a single amended principle. Resolve the open
   questions before implementing.
3. Review the plan, then triage. Reviewers review the plan (give each the plan
   file and the task, and tell them to assume it has problems); the triager
   adjudicates their findings; the planner revises. Loop until the review
   converges (below) before implementing.
4. Implement. The implementer makes the changes.
5. Review the work, then triage. Reviewers review the finished changes (give each
   the means to see exactly what changed, the before and after commit hashes or
   the diff range, plus the task and the relevant files); the triager
   adjudicates; the implementer fixes. Loop until convergence before accepting
   the work.

Review loop, and preventing relitigation. The orchestrator keeps a review ledger
for the task: each finding, the triager's verdict, the reasoning, and the action
taken (fixed in `<commit>`, or dismissed because `<reason>`). On every new round
it gives the reviewers and triager the ledger and this rule: do not re-raise a
settled finding unless you bring new evidence that its verdict was wrong; a
settled finding re-raised without new evidence is dismissed by the ledger, not
re-argued. For a genuinely contested finding, the triager may hold a short
debate, the producer arguing it is invalid and a reviewer arguing it is valid,
before ruling. The loop converges when a round produces no new valid findings; if
findings stay contested after a few rounds, the orchestrator escalates to a human
rather than looping forever. The ledger is transient working state for the task
(keep it as scratch notes, not in the plan) and is discarded when the task
closes; only durable decisions, the ones that change the plan, are folded into
the plan's steps, and individual findings are never kept in the Open Questions
section. The same "new evidence required" rule guards a folded decision: once
recorded with its reasoning, it is reopened only by evidence that beats that
reasoning.

The prompts in `.agents/prompts/` support these phases: clarifying questions
before starting, the open-questions gate before implementing, and adversarial
review (hand `adversarial-review.md` to each reviewer). The triager and
orchestrator follow the roles above.

## Principles

Follow these principles. They are numbered for reference, not priority.

{{principles}}
