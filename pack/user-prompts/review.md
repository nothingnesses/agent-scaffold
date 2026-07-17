# Review prompt

Copy this, fill in the bracketed parts, and paste it to the agent to ask for a code review that ends in a findings report, not an implementation. It triggers the review entry mode defined in `AGENTS.md`; like the kickoff and explore prompts, it is a thin trigger and does not restate the workflow. The agent reviews the target you name against the criteria you give, adjudicates the findings, and returns a committed findings report for you to act on. It implements nothing.

---

Act as the orchestrator described in `.agents/prompts/orchestrator.md`. Read `AGENTS.md` first (the workflow and the project's principles), then, if this review is against a plan, that plan under `docs/plans/`.

I want a review and a findings report, not an implementation. Do not fix anything; produce the report and stop.

Target to review: [the whole codebase as it currently stands (the tree at `HEAD`, no baseline), OR the diff between two refs: commit hashes, branches, or tags, or a `<ref-A>..<ref-B>` range].

Criteria to review against: [name a plan whose Success Criteria to check, and/or list specific constraints or conditions to check; leave blank for open-ended correctness and quality against `AGENTS.md` and the project principles].

[Optional: depth and lenses, for example how many independent reviewers and which models, and whether to treat the target as risky (security-, data-, or money-sensitive) so the review is briefed accordingly.]

Run the review entry mode as `AGENTS.md` defines it: resolve the target and criteria, run one reviewers-then-triager pass (the acceptance-review pass, keeping the high/critical dismissal re-check), and give me a committed findings report grouped by severity with evidence, plus which findings are worth turning into a kickoff task. Do not implement any fix.
