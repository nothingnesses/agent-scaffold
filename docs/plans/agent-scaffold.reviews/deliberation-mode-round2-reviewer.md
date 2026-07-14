# Round-2 review: `deliberation-mode` (Q-8 + Q-12)

Reviewer: independent (round 2). Diff range reviewed: `07da2d8..360f7b9`. Judged
against the numbered Project Principles in `docs/plans/agent-scaffold.md` and the
step's own acceptance criteria (contract stated once, referenced not duplicated).
`pack/` is source; `AGENTS.md` and `.agents/` are generated mirrors.

## Round-1 fixes: all verified landed

- G1 (gates + planner become pure pointers). Verified. `pack/prompts/open-questions-gate.md:9-11`,
  `pack/prompts/planner.md:13-15`, and `pack/prompts/clarifying-questions.md:9-11`
  each now point to "the human-input contract in `AGENTS.md`" with no inline
  four-element enumeration remaining (grep for "viable approaches"/"trade-offs of
  each" in these three files returns nothing). All three still read `AGENTS.md`
  first (line 3 of each), so the pure pointers are actionable. Coherent and usable.
- G2 (factual-vs-decision carve-out). Verified. `pack/AGENTS.md:129-131` now reads
  "a decision-seeking question the human asks directly. A purely factual question is
  answered directly, not put through the contract; the contract applies where the
  human is asking which way to go, not for a fact."
- G4 (over-long lines reflowed). Verified. The two regressed lines are gone:
  former line 18 (89 cols) and line 107 (112 cols) now wrap at 82 and 82/82 cols
  respectively, consistent with the file's prevailing 81-84 col band. See N1 below
  for a new instance the G1 trim introduced elsewhere.
- G5 ("the plan's numbered Project Principles"). Verified. `pack/AGENTS.md:126` now
  reads "the plan's numbered Project Principles", matching `orchestrator.md:112`.

## Sweep of the whole change

- Contract stated exactly once: the named block at `pack/AGENTS.md:123-137`. No
  second authoritative statement of the format exists.
- Referenced everywhere, and "referenced not duplicated" now honoured: intake
  (`AGENTS.md:106`, pointer), escalation bullet (`AGENTS.md:172`), Socratic
  paragraph (`AGENTS.md:139-146`), `orchestrator.md:60-63` and `:110-118`,
  `planner.md:14`, `clarifying-questions.md:9-10`, `open-questions-gate.md:9-10`.
  The only remaining inline enumerations of the format are the four the triager
  explicitly permitted: the Getting-started human summary (G3, no change), the
  escalation bullet (literal Q-12 deliverable), the Socratic paragraph, and the
  orchestrator's operational restatement. No remaining remote second source.
- No dangling references: every "human-input contract" reference resolves; the
  intake's "contract below" sits above the block at line 123. No stale "escalate
  ... with the ledger for a decision" wording remains anywhere (grep clean).
- Mirrors in sync: `pack/prompts/{orchestrator,planner,clarifying-questions,open-questions-gate}.md`
  are byte-identical to `.agents/prompts/*`; `pack/AGENTS.md` differs from
  `AGENTS.md` and `.agents/AGENTS.reference.md` only by the `{{principles}}`
  expansion.
- No broken sentences from the trims; the three pointer sentences read cleanly.
- The last commit's reflow of the writer-isolation line (`AGENTS.md:269-272`) is
  unrelated tree hygiene, as noted, and is correct (former 126-col line now 82).

## N1: G1 trim reintroduced a line-wrap regression in `clarifying-questions.md`

Severity: low
Location: `pack/prompts/clarifying-questions.md:11` and its mirror
`.agents/prompts/clarifying-questions.md:11`.

The round-1 G1 trim inserted the pointer text ("present it per the human-input
contract in `AGENTS.md`, scaled to a question") and rewrapped lines 9-10 but left
the sentence tail unwrapped, producing a 94-col line:

  "or override rather than reconstruct your thinking. If nothing is unclear, say so and state the"

against the file's other lines at 81-83 cols. Evidence this is a regression, not
pre-existing: at `07da2d8` no line in this file exceeded 85 cols (the equivalent
tail wrapped at ~82); the diff hunk shows the 94-col line as an added (`+`) line.
This is the exact defect class the triage adjudicated as G4 (valid, low) and fixed
in `pack/AGENTS.md`; the G1 fix reintroduced it in the file it was editing.
Cosmetic only (Markdown reflows), but flagged for consistency with the G4 verdict:
reflow line 11 to ~80 cols, e.g. break after "reconstruct your thinking." Apply to
both the pack source and the mirror. Sentence itself is coherent; only the wrap is
off.

## Severity tally (new findings only)

- critical: none.
- high: none.
- medium: none.
- low: 1 (N1, wrap regression in `clarifying-questions.md` introduced by the G1 fix).
