# deliberation-mode review (sonnet)

Diff reviewed: `07da2d8..39ab3a3`

---

## S1 (medium): Reference-plus-restatement hybrid in open-questions-gate.md and planner.md

Location: `pack/prompts/open-questions-gate.md` lines 9-11; `pack/prompts/planner.md` lines 13-15

The contract block in `pack/AGENTS.md` closes with: "Each human-input point refers to this contract rather than restating it." Yet the two gate prompts and the planner add only a parenthetical pointer to the contract while keeping the full inline format:

- `open-questions-gate.md`: "present the viable approaches, the trade-offs of each, a recommendation, and the reasoning behind it (the human-input contract in `AGENTS.md`)"
- `planner.md`: "give the viable approaches, their trade-offs, a recommendation, and the reasoning (the human-input contract in `AGENTS.md`)"

Both fully enumerate the four-element contract format AND add the pointer, producing a reference-plus-restatement hybrid that contradicts "referenced rather than restated." This is the exact drift risk the single-source rule is meant to prevent: if the format acquires or loses an element, the inline text in these two prompts will lag the canonical contract.

`clarifying-questions.md` partially restates ("recommendation and the reasoning behind it") with the qualifier "scaled to a question", which is lighter and better motivated by the scaling provision in the contract. Still a partial restatement, but less concerning.

The clean fix for `open-questions-gate.md` and `planner.md` is to trim the inline format entirely, replacing the enumeration with a pure pointer: something like "for each present the decision per the human-input contract in `AGENTS.md`." This respects Principle 1 (prefer the cleaner long-term architecture) and fulfils the contract's own stated design intent.

---

## S2 (medium): Socratic paragraph applies decision-format contract to all "questions," including factual ones

Location: `pack/AGENTS.md` lines 136-143

The Socratic paragraph states: "a human may drive the work by asking a question rather than giving a task. When they do, the orchestrator answers with the same contract (the options, their trade-offs, a recommendation, and Principle-judged reasoning)."

The contract format - viable options, trade-offs, a recommendation, and Principle-judged reasoning - is structured for decisions where multiple approaches exist. It is not applicable to factual questions ("What does the triager do?", "How many review rounds have we had?") where there is one correct answer and no set of options to weigh. Directing the orchestrator to apply the full decision format to every question will produce artificial option sets and unnecessary deliberation for straightforward informational queries.

The contract block itself lists "a question the human asks directly" as a covered human-input point alongside escalation and intake, but gives no guidance on distinguishing fact-seeking from decision-seeking questions. The convergence criterion in the Socratic paragraph ("converges when the human commits a decision to action") also only makes sense for decision questions; a factual answer has no "decision to action."

The paragraph needs to distinguish question types: factual questions should be answered directly; decision-seeking or design questions ("What approach should we take for X?") apply the full contract. Without that distinction the orchestrator will misapply the format.

---

## S3 (low): Getting Started section restates the contract format within the same document

Location: `pack/AGENTS.md` lines 17-18

The Getting Started section (which is for the human, not the agent) says: "the orchestrator lays out the options, their trade-offs, a recommendation, and its reasoning, and you decide." The contract block at lines 123-125 then states the canonical form: "the viable options or approaches, the trade-offs of each, a recommendation, and the reasoning, with the reasoning judged against the project's numbered Project Principles."

Both enumerate the same four elements (options, trade-offs, recommendation, reasoning). The audiences differ (human-facing summary vs. agent-facing prescription) and the contexts differ (what the human receives vs. what the agent must present), so there is a genuine justification for the overlap. Co-location within the same file also means a reader changing one is likely to see the other. Still, if the format evolves - for example if "success criteria alignment" is added as a fifth element - two places in `pack/AGENTS.md` need updating. This is the same "stated twice" pattern the contract is designed to prevent, applied within a single document.

---

## S4 (low): "project's" vs. "plan's" Project Principles wording inconsistency

Location: `pack/AGENTS.md` line 125 vs. `pack/prompts/orchestrator.md` line 112

The contract block says: "the reasoning judged against the project's numbered Project Principles."

The new paragraph in `orchestrator.md` says: "reasoning judged against the plan's numbered Project Principles."

These refer to the same list (the plan's Project Principles section is seeded from `AGENTS.md`'s principles), but the wording is inconsistent. An agent reading both could wonder whether "the project's principles" (from AGENTS.md) and "the plan's principles" (from the plan document) are the same list or different lists. Use one term consistently; "the plan's numbered Project Principles" is more precise because it points to the numbered list an agent can actually reference by number.

The same inconsistency appears within AGENTS.md: the escalation bullet in the Convergence section uses the shorthand "Principle-judged reasoning" (line 170) while the contract block expands it to "the reasoning judged against the project's numbered Project Principles" (line 125). These are consistent in meaning but the shorthand appears without prior definition in AGENTS.md itself (it is defined only by the expanded form; an agent reading the escalation bullet first sees the shorthand before the definition).

---

## S5 (low): Unflowed long line in intake paragraph after local edit

Location: `pack/AGENTS.md` line 107 (approximately)

The intake paragraph was edited to insert "per the human-input contract below;" into the middle of a sentence, but the remainder of the paragraph was not re-wrapped. The result is a run-on line: "durable path when the assessment is uncertain. This intake is also where the agent gives feedback on the request" that is substantially longer than the surrounding lines, which wrap at roughly 70-75 characters. Minor cosmetic inconsistency with the file's line-wrapping style.

---

## No findings at high or critical severity.

Coherence check (gate-relay handoff): the orchestrator's new gate-relay duty ("you are the relay: present them to the human per the contract and return the human's answers to the role") matches the gate prompts' wording ("return your questions to the orchestrator, which relays them to the human and returns the answers"). The two ends of the handoff are consistent.

Scope check: no over-reach (no new phase, role, or machinery beyond the contract, the Socratic mode, the escalation strengthening, the gate-relay duty, and the impasse-trigger restoration). Human-input point coverage is complete: escalation, intake, open-questions gate, clarifying-questions gate, planner open questions, and orchestrator all reference the contract. The impasse trigger in Getting Started is restored. No human-input point found without a contract reference.
