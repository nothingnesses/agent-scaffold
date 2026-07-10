# Agent guidance

This is the canonical guidance for agents working in this repository. It is
harness-agnostic: any harness-specific file (for example `CLAUDE.md`) should
point here rather than duplicate it.

## Workflow

1. Front-load context. Read the relevant code and docs before acting.
2. Plan. For non-trivial work, draft a plan under `docs/plans/` from
   `docs/plans/TEMPLATE.md`. Seed its Project Principles from this file's
   principles (below), in order, then add the project's own after them,
   consolidating any overlap into a single amended principle. Resolve the open
   questions before implementing.
3. Review the plan. Before building, have it reviewed adversarially. Where the
   harness supports sub-agents, spawn separate, independent reviewers rather than
   reviewing it yourself: give each the plan file and the task, and tell them to
   assume the plan has problems and to find them. Do not share your own views or
   conclusions, so each reviewer investigates and concludes on its own.
4. Implement. Make small, reviewable changes and keep the plan's status current.
5. Review the work. Before accepting it, have the finished changes reviewed the
   same way. Where the harness supports sub-agents, spawn separate, independent
   reviewers: give each the means to see exactly what changed (the before and
   after commit hashes, or the diff range), the task, and the relevant files, and
   tell them to assume there are issues and to hunt for them. Do not offer your
   own opinions; let each reviewer investigate and reach its own conclusions.

Acting on review findings (steps 3 and 5): when the reviewers return, judge each
finding on its evidence and severity, not on who raised it or on your own
confidence. Address the valid ones, revising the plan after a plan review or
fixing the code (and updating the plan's status) after a work review, and record
why any finding is dismissed so no concern is silently dropped. If the artifact
changed materially, review it again with fresh reviewers, iterating until the
findings are resolved. Only move on (to implementation after a plan review, or to
accepting the work after a work review) once every finding is resolved or a
remaining risk is consciously accepted and noted. Surface genuine disagreements
or unresolved blockers for a human decision rather than settling them
unilaterally.

The prompts in `.agents/prompts/` support these steps: clarifying questions
before starting, the open-questions gate before implementing, and adversarial
review of the plan and the work (hand `adversarial-review.md` to each reviewer
sub-agent).

## Principles

Follow these principles. They are numbered for reference, not priority.

{{principles}}
