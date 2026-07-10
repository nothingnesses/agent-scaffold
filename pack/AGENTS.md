# Agent guidance

This is the canonical guidance for agents working in this repository. It is
harness-agnostic: any harness-specific file (for example `CLAUDE.md`) should
point here rather than duplicate it.

## Workflow

1. Front-load context. Read the relevant code and docs before acting.
2. Plan. For non-trivial work, draft a plan under `docs/plans/` from
   `docs/plans/TEMPLATE.md`, and resolve the open questions before implementing.
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

The prompts in `.agents/prompts/` support these steps: clarifying questions
before starting, the open-questions gate before implementing, and adversarial
review of the plan and the work (hand `adversarial-review.md` to each reviewer
sub-agent).

## Principles

Follow these principles. They are numbered for reference, not priority.

{{principles}}
