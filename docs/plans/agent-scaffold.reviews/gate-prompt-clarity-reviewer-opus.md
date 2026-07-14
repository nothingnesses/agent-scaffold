# Reviewer findings: `gate-prompt-clarity` (Q-20)

Reviewer: independent, opus. Diff range: `fc30bb3..41cf5ca`.

Scope reviewed: `pack/prompts/clarifying-questions.md`, `pack/prompts/open-questions-gate.md`,
the generated mirrors under `.agents/prompts/`, and the `README.md` layout block. Checked
against the decided intent in the `gate-prompt-clarity` step (plan lines 388-397), `Q-20`
(plan line 98), the routing wording in `pack/AGENTS.md` and `pack/prompts/orchestrator.md`,
and the numbered Project Principles (esp. Principle 1).

## Intent verification (passed)

The change meets the three decided requirements. Confirmed, not assumed:

- Human named as decider. `clarifying-questions.md:14` "The human is the decider here.";
  `open-questions-gate.md:14` "The human decides which options to take."
- Sub-agent -> orchestrator -> human routing stated. `clarifying-questions.md:14-16`
  "If you are a sub-agent without a direct channel to the human, return your questions to
  the orchestrator, which surfaces them and relays the human's answers.";
  `open-questions-gate.md:14-16` the parallel wording for open items.
- Single-agent case not contradicted. The routing note is conditional
  ("If you are a sub-agent without a direct channel to the human"), so in the single-agent
  case (the agent runs the gate and is the orchestrator with a direct human channel) it
  simply does not apply. The body still reads correctly: "raise any clarifying questions
  ... for the human to answer".
- Original gate function preserved. clarifying-questions still requires asking before any
  code is written, converting assumptions into questions, giving a recommendation +
  reasoning, and stating assumptions if nothing is unclear. open-questions-gate still
  requires recording open items into the plan's "Open Questions, Decisions, Issues and
  Blockers" section with viable approaches, trade-offs, a recommendation, and reasoning,
  and the "otherwise proceed" branch. Nothing material dropped.
- README label corrected. `README.md:37` now reads "role prompts and the planner's decision
  gates" (was "one prompt per workflow role"); the two gate line-descriptions
  (`README.md:40-41`) accurately name the human as answerer/chooser and match the prompt
  bodies. No stale "workflow role" text remains (grep clean).
- Routing consistent with `pack/AGENTS.md` (the orchestrator is the human's interface and
  lays out options: AGENTS.md:16-22, 26-30) and `orchestrator.md` (spawns roles, escalates
  to the human).
- Generated mirrors in sync: `diff` of `pack/prompts/*` against `.agents/prompts/*` is
  identical for both files. Both files and README are ASCII-clean (grep for non-ASCII
  returned nothing).

No critical findings. No high findings. No medium findings.

## R1 (low): open-questions-gate merges the "if/otherwise" branches into one paragraph

Location: `pack/prompts/open-questions-gate.md:8-12` (and the generated mirror).

Evidence: the diff removed the blank line that previously separated the "If so ..." branch
from the "Otherwise, state that none are open ..." branch. The two branches now sit in one
paragraph, with the branch-specific instruction "Prefer the approaches most consistent with
the project's principles." wedged between them:

    ... for the human to review and choose
    from. Prefer the approaches most consistent with the project's principles.
    Otherwise, state that none are open and proceed with the next steps.

The gate is a two-way decision (open items exist, or none do). The prior paragraph break
made that if/otherwise structure visually explicit; gluing the else-branch onto the end of
the if-branch, after a sentence that belongs only to the if-branch, slightly muddies it.
This is a readability regression, not a correctness error, and it was not required by the
Q-20 intent (which only asked to fix the routing wording, not to reflow this paragraph).
Suggest restoring a blank line before "Otherwise" so the branch structure reads cleanly.

## R2 (low): "surfaces" used as a verb, against the maintainer's stated style preference

Location: `pack/prompts/clarifying-questions.md:15`, `pack/prompts/open-questions-gate.md:15`
(and mirrors).

Evidence: both new routing sentences use "which surfaces them and relays ...". The
maintainer's documented writing-style preference is to avoid "surface" as a verb (prefer
"raise"/"show"/"report"). Caveats that make this minor: (a) the decided intent text in the
step itself uses "surfaces them" (plan line 394), so this matches the recorded wording;
(b) the pack already uses "surface" as a verb elsewhere (`pack/principles.toml:544` "surface
staleness"), so it is existing house style, not a new inconsistency. Flagged only for
awareness; downgrading or dismissing this is reasonable. If changed, "passes them on"/"shows
them" reads equivalently.
