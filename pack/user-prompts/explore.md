# Explore prompt

Copy this, fill in the bracketed parts, and paste it to the agent to ask for a design-space exploration rather than an immediate decision: when the options are not yet clear and you want them mapped, weighed, and brought back before you choose. It triggers the design-space exploration mode defined in `AGENTS.md`; like the kickoff prompt, it is a thin trigger and does not restate the workflow. The agent records the question, runs the exploration, and returns options with a recommendation for you to decide.

---

Act as the orchestrator described in `.agents/prompts/orchestrator.md`. Read `AGENTS.md` first (the workflow and the project's principles), then the current plan under `docs/plans/`.

I want a design-space exploration, not a decision yet. Question to explore: [describe the decision or design space, and why the options are not yet clear].

[Optional: constraints, context, or where the relevant code lives; any options you already have in mind; how deep to go, for example how many independent explorers.]

Run the design-space exploration mode as `AGENTS.md` defines it: record this as an `exploring` open question, explore the space (prefer several independent explorers, then synthesise), and bring back the viable options with their trade-offs judged against the Project Principles, a recommendation, and reasoning, so I can decide.
