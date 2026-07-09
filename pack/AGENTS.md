# Agent guidance

This is the canonical guidance for agents working in this repository. It is
harness-agnostic: any harness-specific file (for example `CLAUDE.md`) should
point here rather than duplicate it.

## Workflow

1. Front-load context. Read the relevant code and docs before acting.
2. Plan. For non-trivial work, draft a plan under `docs/plans/` from
   `docs/plans/TEMPLATE.md`, and resolve the open questions before implementing.
3. Review the plan. Check it critically, ideally with a second model or a
   separate agent, before building.
4. Implement. Make small, reviewable changes and keep the plan's status current.
5. Review the work. Check the finished work adversarially before accepting it.

The prompts in `.agents/prompts/` support these steps: clarifying questions
before starting, the open-questions gate before implementing, and adversarial
review of the plan and the work.

## Principles

Follow these principles. They are numbered for reference, not priority.

{{principles}}
