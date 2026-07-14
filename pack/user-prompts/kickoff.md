# Kickoff prompt

Copy this, fill in the bracketed parts, and paste it to the agent to start a new task under the workflow. It is a thin trigger: the workflow and its rules live in `AGENTS.md`, so this prompt only points the agent there and states your task; it deliberately does not restate the workflow.

---

Act as the orchestrator described in `.agents/prompts/orchestrator.md`. Read `AGENTS.md` first (the workflow and the project's principles), then the current plan under `docs/plans/`; if there is no plan for this task yet, start one from `docs/plans/TEMPLATE.md`.

Task: [describe what you want done].

[Optional: constraints, context, or where the relevant code lives.]

[Optional: set the checkpoint cadence. By default, at each step boundary the orchestrator reports what completed and what is next, then continues (report-and-continue). Say "gate at each step boundary" to be asked for a go-ahead between steps, or "run autonomously through to acceptance" to be interrupted only for decisions that need you.]

Then drive the workflow to completion as `AGENTS.md` defines it.
