# Review: gate-prompt-clarity (Q-20)

Diff range: `fc30bb3..41cf5ca`. Source of truth is `pack/`; `.agents/` and root `README.md` are generated targets.

---

## S1 - medium

**Routing claim ungrounded in orchestrator.md**

Location: `pack/prompts/clarifying-questions.md` lines 14-16 and `pack/prompts/open-questions-gate.md` lines 14-16 (post-change).

Both gate prompts now end with a routing paragraph:

> If you are a sub-agent without a direct channel to the human, return your questions to the orchestrator, which surfaces them and relays the human's answers.

and:

> If you are a sub-agent without a direct channel to the human, return the open items to the orchestrator, which surfaces them and relays the human's decisions.

These claim the orchestrator "surfaces" the gate output and "relays the human's answers/decisions." That duty is not stated in `pack/prompts/orchestrator.md`. The orchestrator prompt's human-escalation instructions cover two specific situations: reaching the total-round cap (convergence limit) and intake assessment for new human requests. Neither covers the case where a spawned planner returns gate questions without a routing rule for the orchestrator.

`pack/AGENTS.md` lines 16-20 (Getting started, for the human) says "the orchestrator lays out the options... and you decide" and "the orchestrator brings its recommendation to you when a decision is needed," and the orchestrator is instructed to read `AGENTS.md` first. This partly mitigates the gap: an orchestrator reading `AGENTS.md` can infer its role as the human's decision interface. But the inference requires reading a section written for the human, not the orchestrator, and the specific scenario (planner sub-agent returns gate questions, orchestrator surfaces and relays) is never stated as an explicit orchestrator duty. A fresh orchestrator agent reading only `orchestrator.md` finds no instruction covering gate-question relay.

The gap is a Principle 1 coherence problem: the gate prompts describe a behaviour from the orchestrator that the orchestrator's own prompt does not commit to. If this gap is left, an orchestrator that does not independently reason its way to the relay duty could either absorb the gate questions itself (resolving them without human input) or stall. Closing it requires one short sentence in `orchestrator.md` covering what to do when a spawned planner returns unresolved gate questions - for example, a rule to surface the questions and answers to the human before proceeding with the plan phase.

---

## No findings at low, high, or critical

Checked:

- **Remaining first-person references.** Grep for `\bme\b`, `\bI\b`, `\bmy\b` across both gate prompts returns nothing. The earlier "ask me", "for me to review and choose from" instances are fully removed.
- **Single-agent fallback coherence.** The clause "If you are a sub-agent without a direct channel to the human" is conditional. In the single-agent case the same agent plays all roles and does have a direct channel to the human, so the condition is false and the routing paragraph does not fire. The main body text ("raise any clarifying questions... for the human to answer") still applies, so the single-agent path is handled correctly.
- **Who runs these.** `pack/AGENTS.md` describes "Planner (planner.md, with clarifying-questions.md and open-questions-gate.md)" - the gate prompts are planner adjuncts. Addressing the running agent as "you... a sub-agent" is accurate: in the multi-agent workflow the planner is a sub-agent spawned by the orchestrator. No contradiction with AGENTS.md or planner.md.
- **Scope.** The change touches exactly what Q-20 specified: routing wording in both gate prompts, README layout label. No folding into planner.md, no added machinery. The README change from "one prompt per workflow role" to "role prompts and the planner's decision gates" is accurate and does not overclaim (it does not imply the human pastes these prompts). The file labels ("gate: agent asks, the human answers, before starting" and "gate: agent presents options, the human chooses") match the prompts.
- **Pack/generated sync.** `pack/prompts/clarifying-questions.md` and `.agents/prompts/clarifying-questions.md` are byte-identical, as are the two `open-questions-gate.md` copies. The README change is in the root `README.md`; `pack/` carries no separate README so no pack-side README was missed.
- **Principle 2 (minimal).** The change adds one routing paragraph to each gate prompt and rewrites two README lines. No new assets, no new machinery.
- **Term consistency.** "Surfaces" and "relays" are new in these prompts. AGENTS.md uses "lays out" and "brings... to you"; orchestrator.md uses "escalate" and "report back." The meaning is consistent even if the specific words differ; this is not a defect on its own (and would merge with S1 if that is fixed by adding a matching sentence to orchestrator.md using the same vocabulary).
