# Triage: `deliberation-mode` (Q-8 + Q-12)

Artifact: commit range `07da2d8..39ab3a3`. Adjudicated against the plan step
`deliberation-mode` (Q-8, Q-12, and the two cluster follow-ups), the numbered
Project Principles (1-7), and the current pack (`pack/AGENTS.md`,
`pack/prompts/orchestrator.md`, `planner.md`, `clarifying-questions.md`,
`open-questions-gate.md`).

Two independent reviews adjudicated: `deliberation-mode-reviewer-opus.md` (R1, R2)
and `deliberation-mode-reviewer-sonnet.md` (S1-S5). Deduplication applied:
R1 and S5 are one wrap-regression group; R2 and S4 are one wording-variance group.
Five finding-groups below.

All fixes below apply to the `pack/` sources; the generated mirrors (`AGENTS.md`,
`.agents/AGENTS.reference.md`, `.agents/prompts/*`) must stay in sync (the Opus
reviewer confirmed they are byte-identical to their `pack/` sources except the
`{{principles}}` expansion, so re-run the pack build / mirror step after any edit).

---

## Group 1 (S1): reference-plus-restatement hybrid in the gate prompts and planner

Covered ids: S1.

Verdict: VALID.
Severity: medium for `open-questions-gate.md` and `planner.md` (confirming the
reviewer); low for `clarifying-questions.md`.

Reasoning. Q-12 is explicit that the contract is "stated once in `pack/AGENTS.md`
... referenced from each human-input point rather than duplicated (Principles 1 and
2)", and the contract block itself closes with "Each human-input point refers to
this contract rather than restating it." `open-questions-gate.md` and `planner.md`
add the pointer AND still enumerate the full four-element format ("the viable
approaches, the trade-offs of each, a recommendation, and the reasoning behind
it"). That is a direct self-contradiction of the rule introduced in the same
commit, and it is the exact drift vector the single-source rule exists to close: if
the format later gains or loses an element, these two inline copies lag the
canonical block silently. This is a real coherence miss against Principle 1 and
against the step's own acceptance criterion, hence medium rather than cosmetic.

`clarifying-questions.md` is the least affected: it keeps only a partial, scaled
form ("recommendation and the reasoning behind it ... scaled to a question"), which
is the contract's own sanctioned lighter form for a low-stakes point, not a
restatement of the full four-element format. Valid but low.

Key call (trim vs soften). Trim, do not soften. Softening the contract's "rather
than restating it" clause to permit inline restatement would gut the Q-12
deliverable (one format, one source, referenced not duplicated) to accommodate the
very drift it was written to prevent. The standalone-prompt concern does not
justify softening: both gate prompts already instruct the agent to read `AGENTS.md`
first, so a pure pointer is fully actionable. Trim the two full-format restatements
to a pure pointer; lightly trim `clarifying-questions.md` for consistency (optional,
because its partial is already the contract's lighter form).

Recommended minimal fix (concrete wording):

- `pack/prompts/open-questions-gate.md` (and mirror). Replace
  "and for each present the viable approaches, the trade-offs of each, a
  recommendation, and the reasoning behind it (the human-input contract in
  `AGENTS.md`), for the human to review and choose from."
  with
  "and for each present the decision per the human-input contract in `AGENTS.md`,
  for the human to review and choose from."
  (Keep the following "Prefer the approaches most consistent with the project's
  principles." sentence.)

- `pack/prompts/planner.md` (and mirror). Replace
  "and for each give the viable approaches, their trade-offs, a recommendation, and
  the reasoning (the human-input contract in `AGENTS.md`), so a decision is a matter
  of confirming rather than reconstructing."
  with
  "and for each give the decision per the human-input contract in `AGENTS.md`, so a
  decision is a matter of confirming rather than reconstructing."

- `pack/prompts/clarifying-questions.md` (and mirror; optional, low). Replace
  "For each, give your recommendation and the reasoning behind it (the human-input
  contract in `AGENTS.md`, scaled to a question), so the human can confirm or
  override rather than reconstruct your thinking."
  with
  "For each, present it per the human-input contract in `AGENTS.md`, scaled to a
  question, so the human can confirm or override rather than reconstruct your
  thinking."

Note (not a new finding, flagged for the author's awareness): the parenthetical
four-element enumerations inside `pack/AGENTS.md` itself (the escalation bullet and
the Socratic paragraph) and in `orchestrator.md` are borderline by the same rule,
but they are acceptable as written: the two in `AGENTS.md` sit adjacent to the
contract and reinforce it rather than acting as a remote second source, and the
escalation enumeration is the literal Q-12 deliverable (escalation must now present
structured options). No change required there.

---

## Group 2 (S2): Socratic contract applied to purely factual questions

Covered ids: S2.

Verdict: VALID.
Severity: low (correcting the reviewer's medium down).

Reasoning. The defect is real but narrower and lower-impact than rated. The Socratic
paragraph and the orchestrator sentence both frame the mode as the human "driv[ing]
the work by asking a question rather than giving a task", and the paragraph's
convergence clause ("converges when the human commits a decision to action") already
scopes it to decision-driving questions: a factual lookup has no "decision to
action" to commit, so the framing mostly self-limits. The genuine over-reach is the
contract block's enumeration of covered points, which lists "a question the human
asks directly" with no qualifier. Read literally, a plain factual question ("what
does the triager do?", "how many rounds have we had?") is "a question the human asks
directly" and would be pushed through the full options/trade-offs/recommendation
format, manufacturing artificial option sets. Scale-to-stakes mitigates but does not
resolve this: it scales a decision's weight, it does not tell the agent that a fact
is not a decision at all. Impact if unfixed is a minor, self-correcting inefficiency,
hence low, not medium.

This matches Q-8's intent. Q-8 is the human DRIVING the work by asking a question
(the Socratic sense), i.e. a decision-seeking question that stands in for a task, not
a request for a fact. The carve-out below aligns the text with that intent.

Recommended minimal fix (concrete wording). One clause at the single over-broad
point, the contract block's covered-points sentence in `pack/AGENTS.md` (and mirror).
Replace
"an open question or a clarifying question raised before or during the work, and a
question the human asks directly."
with
"an open question or a clarifying question raised before or during the work, and a
decision-seeking question the human asks directly. A purely factual question is
answered directly, not put through the contract; the contract applies where the
human is asking which way to go, not for a fact."

The Socratic paragraph and the orchestrator sentence need no change (their
"drive the work" / "commits a decision to action" framing already implies the
decision sense); the single carve-out at the contract enumeration is sufficient.

---

## Group 3 (S3): Getting started restates the four elements

Covered ids: S3.

Verdict: VALID as an observation, but NOT a defect requiring a fix (acceptable
audience-appropriate summary).
Severity: low (informational).

Reasoning. The overlap is real (the Getting started prose and the contract block
both list options, trade-offs, recommendation, reasoning), but this is not the
duplication Principle 1 targets. The Getting started section was created
deliberately by `human-onboarding` (Q-9) to orient a NEWCOMER HUMAN who has not yet
read the workflow body; its audience and purpose differ from the contract block,
which is the agent-facing prescription. Crucially, the onboarding prose DESCRIBES to
the human what they will receive; it does not PRESCRIBE a format to any agent. The
agent still derives the format solely from the contract block, so there is no second
authoritative source of the contract for the machinery. The two are co-located in
one file and mutually consistent, so the residual drift risk (a future fifth element)
is minimal and visible to anyone editing either. Weighing Principle 1's coherence
concern against the onboarding section's stated purpose, the onboarding purpose wins:
forcing a newcomer to jump to the agent-facing contract block to learn "what will I
be asked to decide" defeats the reason the section exists.

Recommended minimal fix. None. Keep the enumeration as an audience-appropriate
summary. A soft pointer ("(the human-input contract below)") could be added but is
not recommended; it adds clutter for a newcomer without removing the drift risk it
purports to address (the enumeration stays either way). Do not trim to a bare
pointer.

---

## Group 4 (R1 + S5): prose line-wrap regression on edited paragraphs

Covered ids: R1, S5.

Verdict: VALID.
Severity: low (confirming both reviewers).

Reasoning. Confirmed by measurement: `pack/AGENTS.md` line 18 is 89 columns and line
107 is 112 columns, against surrounding lines at 77-83, and both are lines the diff
touched (the edits inserted text and rewrapped only partially). `git show
07da2d8:pack/AGENTS.md` shows the pre-edit paragraph wrapped at 71-82, so this is a
regression, not pre-existing. Cosmetic only (Markdown reflows, no rendered change),
but it breaks the file's ~80-column convention and would be caught by any
line-length lint. Same defect in the two mirrors.

Recommended minimal fix. Reflow both edited paragraphs to ~80 columns. Concrete
example wraps (downstream lines in each paragraph reflow to absorb the shift):

- Lines ~16-19 (the "Your part does not end at kickoff" paragraph): break line 18
  after "and you", e.g.
  "the options, their trade-offs, a recommendation, and its reasoning, and you"
  / "decide. These decisions collect in the plan's \"Open Questions\" section, the
  single human-decision queue; ..." and let the rest of the paragraph reflow.

- Lines ~105-108 (the intake paragraph): break line 107 after "also where the",
  e.g.
  "durable path when the assessment is uncertain. This intake is also where the"
  / "agent gives feedback on the request itself, so the human can correct or refine
  it" / "before any work starts."

Apply to `pack/AGENTS.md` and both mirrors.

---

## Group 5 (R2 + S4): "the project's" vs "the plan's" numbered Project Principles

Covered ids: R2, S4.

Verdict: VALID.
Severity: low (confirming both reviewers).

Reasoning. `pack/AGENTS.md`'s contract block says reasoning is "judged against the
project's numbered Project Principles"; `pack/prompts/orchestrator.md` says "the
plan's numbered Project Principles." Both point at the same set, so this is a wording
variance, not a correctness defect, but a reader seeing both phrasings could
momentarily wonder whether two different principle sets are meant. Principle 1
(coherence, one source) favors unifying.

Which is correct: "the plan's". The numbered, referenceable-by-number list is
established and lives in the plan's "Project Principles" section: `planner.md`
instructs the planner to seed it from `AGENTS.md`, add project-specific ones, and
"keep the list numbered so it can be referenced by number." That numbered list, in
the plan, is the concrete artifact an agent judges reasoning against. "The plan's
numbered Project Principles" names that location and removes the "two lists?"
ambiguity by consistently pointing at one document; "the project's" names ownership
but not the location the agent actually reads. Standardize on "the plan's".

Recommended minimal fix. In `pack/AGENTS.md` (and mirror), change the contract
block's "with the reasoning judged against the project's numbered Project
Principles" to "... the plan's numbered Project Principles". `orchestrator.md`
already uses "the plan's" and needs no change.

Sub-point (S4, shorthand "Principle-judged reasoning" appearing before its
definition): INVALID. In document order the contract block (which expands the form)
precedes the escalation bullet and the Socratic paragraph that use the "Principle-
judged reasoning" shorthand, so the shorthand is defined before it is used. The
reviewer's concern holds only for an agent reading strictly out of order, which the
workflow does not require. No change.

---

## Severity tally (adjudicated)

- critical: none.
- high: none.
- medium: 1 (S1, for `open-questions-gate.md` and `planner.md`).
- low: S1 (`clarifying-questions.md`), S2, S3 (informational), R1/S5 wrap group,
  R2/S4 wording group.

No high-or-above dismissals, so no second-triager re-check is required. S3 is the
only finding recommended for no code change; every other valid finding has a
concrete minimal fix above.
