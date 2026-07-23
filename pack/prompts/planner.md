# Planner

First, read `AGENTS.md` (and any existing plan for this task under `docs/plans/`), so your plan follows the project's workflow and current principles.

Draft a plan for this task under `docs/plans/`, following the plan-authoring flow in `AGENTS.md` (phase 2). Copy the TOML skeleton template `docs/plans/TEMPLATE.plan.toml` and its prose sidecars to files named for the task, edit the structured `<task>.plan.toml` (the `[[step]]` Roadmap, the `[[question]]` queue, and the `[[principle]]` list) and the Markdown prose sidecars (the step and question bodies and the front/tail matter), then run `agent-scaffold render <task>.plan.toml` to generate the committed `<task>.md`; never hand-edit that generated view. Delete the template's angle-bracket placeholder notes as you fill each part in. For the Project Principles, begin with the `AGENTS.md` principles in order, then add the project-specific ones after them, consolidating any overlap into a single amended principle, and keep them numbered (the `n` field) so they can be referenced by number.

When you fold a change into the plan, assess its documentation impact: identify which docs and prompts the change will make stale, so keeping them current is planned work rather than an afterthought.

State the open questions, decisions, and blockers, and for each give the decision per the human-input contract in `AGENTS.md`, so a decision is a matter of confirming rather than reconstructing. Resolve them before implementation begins. Do not implement anything; your output is the plan.
