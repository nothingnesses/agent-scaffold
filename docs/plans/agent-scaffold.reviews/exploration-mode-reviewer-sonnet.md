# Review: exploration-mode (Q-29) - commit 9757a64

Reviewer: sonnet (documentation quality, clarity, completeness lens) Artifact: commit 9757a64 - files changed: `pack/AGENTS.md`, `pack/plan-template.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`, `docs/plans/TEMPLATE.md`, `src/plan.rs` Reference: `pack/AGENTS.md` (Design-space exploration intake paragraph + Design explorations artifact-convention paragraph), `pack/plan-template.md` (exploring description), `docs/plans/agent-scaffold.md` (exploration-mode step detail).

---

## Finding 1 - Medium: AGENTS.md intake paragraph attributes writing of design-notes artifact to the orchestrator, contradicting the Design explorations paragraph

Location: `pack/AGENTS.md` line 45 (Design-space exploration paragraph) vs. `pack/AGENTS.md` line 59 (Design explorations paragraph). Same in `AGENTS.md` and `.agents/AGENTS.reference.md`.

The intake paragraph reads: "the orchestrator records the question as an Open-Questions item with status `exploring` (a design pass is owed), runs the exploration, and writes a design-notes artifact (see Design explorations below)." Grammatically, the orchestrator is the subject of "runs the exploration" and "writes a design-notes artifact."

The Design explorations paragraph (line 59) says the opposite: "prefer several independent explorers with different lenses or models and synthesise their proposals, rather than one unchecked take" and "The orchestrator owns cleanup" - cleanup is all that belongs to the orchestrator; the writing belongs to the explorers.

The plan step detail (`docs/plans/agent-scaffold.md` ~line 574) also says "spawns the explorer(s)" and "the multi-independent-explorer-plus-synthesis setup (which produced this very decision) is the adversarial analog," consistent with the Design explorations paragraph, not the intake paragraph.

The inconsistency within AGENTS.md itself is the real defect: a reader who only reads the intake paragraph (the more prominent, forward-facing paragraph) concludes the orchestrator writes the file. A reader who continues to the Design explorations paragraph sees the opposite. The "(see Design explorations below)" reference is present but a reader might not follow it for what seems like a procedural detail.

Impact: an orchestrator following the intake paragraph literally writes the design-notes file itself, bypassing the independent-explorer guidance and breaking role separation.

---

## Finding 2 - Medium: pack/plan-template.md omits the orchestrator-initiated trigger for exploring status

Location: `pack/plan-template.md` line 24 (Open Questions template instruction). Same in `docs/plans/TEMPLATE.md`.

The plan-template says: "`exploring` is a sub-state of `open`: a design-space exploration is owed before the item's options are decidable (the orchestrator runs an exploration and writes a design-notes artifact, then moves the item to `open` with options ready); use it when the human asks to deliberate rather than to decide."

`pack/AGENTS.md` line 45 gives two triggers: "the human asks to deliberate rather than to decide, or the orchestrator finds a decision's design space genuinely open." The second trigger (orchestrator-initiated, where no human request prompted the exploration) is absent from the plan-template.

The plan-template is the inline instruction agents read when filling in the Open Questions section of a live plan. Omitting the orchestrator-initiated case means an orchestrator that judges the design space too open to present options will not recognize `exploring` as the right status and may fall back to leaving the item `open` (options ready, awaiting choice) or producing options prematurely.

Additionally, the plan-template also repeats the misleading "the orchestrator runs an exploration and writes a design-notes artifact" phrasing from Finding 1.

---

## Finding 3 - Medium: Explorer is an unnamed writer role; file-safety and isolation rules do not cover it

Location: `pack/AGENTS.md` roles paragraph (the writer/reader classification) and the File safety section. `pack/AGENTS.md` Design explorations paragraph (line 59).

AGENTS.md defines: "Among the spawned roles, the planner and the implementer are writers (they change the plan or the code)... 'Writer agent' below means a spawned writer role." The file-safety rules ("Clean tree before a writer," "Format only your own files") and the writer isolation rules (container > worktree > file-safety fallback) all use "writer agent" as their subject.

An explorer writes `docs/plans/<task>.explorations/<question>.md` to the filesystem - it is a writer. But "explorer" is not named in the roles list, not given a prompt in `pack/prompts/`, and not listed as a writer in the writer/reader classification. A reader following the file-safety rules would not know to apply "clean tree before a writer" or writer isolation when spawning explorers.

The Design explorations paragraph says "prefer several independent explorers with different lenses or models" but gives no guidance on what prompt they receive, whether they need isolation, or how collisions on the output file are prevented when multiple explorers write in parallel (findings files avoid collisions by disambiguating filenames; the explorations convention only names the file for the question, not for each explorer, raising the question of whether explorers synthesise into a single file or write separate ones that the orchestrator then synthesises).

The plan step detail mentions "spawns the explorer(s)" without saying what role prompt to hand them - the gap is structural, not incidentally missed.

---

## Finding 4 - Medium: orchestrator.md not updated; its existing Socratic handling can suppress exploration mode

Location: `pack/prompts/orchestrator.md` (unchanged by this commit); `.agents/prompts/orchestrator.md` (identical, also unchanged).

The orchestrator prompt's question-handling instruction reads: "If the human drives by asking a question rather than giving a task, answer with the same contract and record the resolved answer as a durable Open-Questions decision."

This handles Socratic input (answer immediately with the contract). Exploration mode differs precisely here: the orchestrator should NOT answer with the contract immediately, but instead record an `exploring` item, spawn explorers, synthesise, and only then present options. A deliberation request from the human ("I want to think through whether to X") looks like a question, so the orchestrator prompt's instruction triggers for it and routes it to Socratic mode.

The orchestrator prompt tells the agent to read `AGENTS.md`, but the explicit, more proximate Socratic instruction in orchestrator.md is a competing signal. An agent following orchestrator.md strictly would answer immediately with the contract rather than entering exploration mode.

The Roadmap's plan step for intake-mode (line 574 in `docs/plans/agent-scaffold.md`) says to update `pack/AGENTS.md`; it does not name `pack/prompts/orchestrator.md`. But the commit result is that the behaviour the new mode requires is not reinforced in the prompt the orchestrator actually executes. Since the AGENTS.md intake paragraph refers to "the orchestrator" as the actor, and the orchestrator's own prompt is where orchestrator behaviour is reinforced, the omission is a gap in this step's deliverables.

---

## Finding 5 - Low: lifecycle is silent on what happens when an exploration itself surfaces new open questions

Location: `pack/AGENTS.md` lines 45-59 (Design-space exploration and Design explorations paragraphs).

The stated lifecycle is: record `exploring` -> run exploration -> write design-notes -> present options via human-input contract -> move item to `open`. But an exploration may reveal that the question depends on subsidiary questions that are themselves undecidable without further exploration. No guidance on this case: do nested sub-explorations get their own `exploring` items and files? Does the main item stay `exploring` until all subsidiaries resolve? Does the orchestrator present what it has and flag the open threads?

This is a real edge case in design work. The lifecycle's silence means the orchestrator must improvise, which is the kind of disposition-based fallback the broader workflow aims to avoid.

---

## Finding 6 - Low: README workflow diagram shows intake but not exploration mode; discoverability gap for onboarding

Location: `README.md` lines 83-86 (Mermaid diagram) and lines 50-87 (workflow description).

The README diagram has an `intake` node for human requests but no node or annotation for design-space exploration or the `exploring` status. The Socratic input mode is also absent from the README, which suggests the README intentionally operates at a higher level and defers to AGENTS.md for detail - consistent with the commit's "no new phase" constraint.

However, the `exploring` status is now a recognized vocabulary item in `validate --plan` (enforced by the code change in `src/plan.rs`). A new user who reads only the README and the plan template's Open Questions instruction would not know this mode exists. The README could at minimum note that AGENTS.md documents additional entry modes (Socratic, design-space exploration) beyond the diagrammed request flow, without adding a diagram node. This is a discoverability gap, not a defect.

---

## Finding 7 - Low: "exploring" lifecycle transition to `open` is not specified as guarded by the human deciding

Location: `pack/AGENTS.md` line 45 (Design-space exploration paragraph).

The intake paragraph says: "only then does it present the options through the human-input contract and move the item to `open`." The phrasing implies the orchestrator both presents options and moves the item to `open` as one action. But the human-input contract requires the human to decide; the item should move to `open` (options ready, awaiting the human's choice) after the exploration, not after the human decides - `open` means "options ready, awaiting choice," and moving from `exploring` to `open` is a natural transition. Moving to `decided -> folded into <slug>` comes after the human decides.

This is a minor ambiguity in what `open` means in this transition. The plan-template's description of `exploring` says "then moves the item to `open` with options ready" - which aligns with the reading that `open` is the post-exploration state and the human's actual choice then moves it to `decided`. So the lifecycle is probably correct, but the intake paragraph's phrasing ("present the options...and move the item to `open`") could be read as skipping the human's choice before closing the `exploring` status. A clarifying phrase like "and moves the item from `exploring` to `open` so the human can decide" would remove the ambiguity.

This is a nit.

---

## Finding 8 - Nit: cleanup timing for explorations is less defined than for findings files

Location: `pack/AGENTS.md` line 59 (Design explorations paragraph) vs. `pack/AGENTS.md` line 61 (Findings files paragraph).

Findings files: "when a round is fully resolved, or at task close, it commits the findings files at least once and then deletes them." Explicit triggers.

Explorations: "The orchestrator owns cleanup on the same commit-before-delete rule as findings files, unless the exploration is worth keeping as a durable design record." No trigger defined; the decision to keep vs. delete is left to the orchestrator with no stated criteria or timing. "Worth keeping as a durable design record" is a disposition, not a rule. This is intentional design flexibility, but the analogy to findings files (which does have triggers) is slightly misleading.

---

## Scope creep check

No scope creep found. The plan explicitly excluded: a new workflow phase parallel to Plan/Implement/Accept, a machine-parsed schema, and an `explore` subcommand. None of these appear. The changes are documentation plus one code line. No new role prompt was added. The optional light review reuses existing reviewer/triager roles. The YAGNI boundary was honoured.

---

## Summary

- Critical: 0
- High: 0
- Medium: 4 (Findings 1-4)
- Low: 3 (Findings 5-7)
- Nit: 1 (Finding 8)

The four medium findings are interconnected: (1) the intake paragraph and plan-template both attribute writing to the orchestrator rather than spawned explorers, (2) the plan-template omits the orchestrator-initiated trigger, (3) explorers are unnamed as writers so file-safety and isolation rules don't cover them, and (4) the orchestrator prompt wasn't updated so its Socratic handling competes with the new mode. These collectively mean an orchestrator following the documentation as written may handle exploration requests incorrectly.
