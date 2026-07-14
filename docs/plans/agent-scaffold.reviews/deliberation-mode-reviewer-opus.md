# Review: `deliberation-mode` (Q-8 + Q-12 + follow-ups A/B)

Reviewer: independent (Opus). Diff range: `07da2d8..39ab3a3`. Lens: completeness
and correctness against the six deliverables in the step detail
(`docs/plans/agent-scaffold.md`, step `deliberation-mode`), judged against the
plan's numbered Project Principles.

## Verification summary (what passed)

All six deliverables are present and substantively correct:

1. Human-input contract stated ONCE as a named block in `pack/AGENTS.md`
   (lines 122-134, "Human-input contract (how every decision is put to the
   human)."). It covers options/approaches, per-option trade-offs, a
   recommendation, reasoning judged against the numbered Project Principles,
   scale-to-stakes, human-decides, and durable recording in Open Questions,
   matching the Q-12 spec. It also states "Each human-input point refers to this
   contract rather than restating it."
2. Every human-input point references the contract: intake (`pack/AGENTS.md:105`,
   "per the human-input contract below"); the escalation bullet in the
   Convergence section (`pack/AGENTS.md:169`); `pack/prompts/orchestrator.md:62`
   (review-loop escalation step) and `:110-116` (the cross-cutting rule +
   gate-relay paragraph); `pack/prompts/planner.md:14`;
   `pack/prompts/clarifying-questions.md:10`;
   `pack/prompts/open-questions-gate.md:10`. No stale "escalate with the ledger
   for a decision" wording remains (grep for `escalat`/`with the ledger`
   confirms).
3. Socratic mode is a first-class entry mode (`pack/AGENTS.md:136-143`),
   explicitly "alongside the request-driven interrupt above", converging "when
   the human commits a decision to action" under the "no re-raise without new
   evidence" rule, and explicitly "adds no new phase or role, reusing the intake
   and Open-Questions machinery." The Roles list (`pack/AGENTS.md:38`) and Phases
   list (`:66`) are untouched by the diff, so no role/phase was added (Q-8 and
   Principle 2 satisfied).
4. Escalation strengthened in BOTH `pack/AGENTS.md:168-171` and
   `pack/prompts/orchestrator.md:60-63` (both now present structured options via
   the contract, with the ledger as evidence).
5. Gate-relay duty (follow-up B) is present at
   `pack/prompts/orchestrator.md:110-118` and is coherent with the gate prompts'
   existing wording ("return your questions to the orchestrator, which relays them
   to the human and returns the answers", `clarifying-questions.md:15-17`,
   `open-questions-gate.md:16-18`).
6. "impasse" is restored in the "Getting started, for the human" enumeration
   (`pack/AGENTS.md:17-18`, "a question, an impasse, or a trade-off"), inside the
   correct section (heading at line 7).

Generated mirrors are in sync: `diff` of `pack/AGENTS.md` vs `AGENTS.md` shows
only the expected `{{principles}}` -> expanded-principles substitution, and
`.agents/AGENTS.reference.md`, `.agents/prompts/orchestrator.md`,
`clarifying-questions.md`, `open-questions-gate.md`, `planner.md` are byte-for-byte
identical to their `pack/` sources. All changed files are ASCII-clean
(`grep -P '[^\x00-\x7F]'` finds nothing).

## R1: Prose line-wrapping regression in edited paragraphs

Severity: low
Location: `pack/AGENTS.md` (and the mirror `AGENTS.md` / `.agents/AGENTS.reference.md`), lines 18 and 107.

The two lines the edits modified were left unwrapped and now exceed the file's
~80-column prose convention while their surrounding lines stay at ~77-82:

- Line 18 is 89 chars: "the options, their trade-offs, a recommendation, and its
  reasoning, and you decide. These".
- Line 107 is 112 chars: "durable path when the assessment is uncertain. This
  intake is also where the agent gives feedback on the request".

Evidence this is a regression, not pre-existing: at `07da2d8` the intake
paragraph wrapped at 71-82 chars (`git show 07da2d8:pack/AGENTS.md`, lines
105-108 were 71/82/76/69). The edit inserted ", per the human-input contract
below" and rewrapped only partially, leaving line 107 at 112 chars. Same pattern
at line 18. Cosmetic only (no rendered-content change; Markdown reflows), but it
breaks the file's wrapping consistency and would be caught by any line-length
lint. Recommend rewrapping both paragraphs to ~80 columns before commit.

## R2: Minor wording variance "the plan's" vs "the project's" Project Principles

Severity: low
Location: `pack/prompts/orchestrator.md:112` vs `pack/AGENTS.md:125`.

The contract in `AGENTS.md` says reasoning is "judged against the project's
numbered Project Principles", while the orchestrator prompt says "judged against
the plan's numbered Project Principles". In this workflow the plan document is
where the Project Principles live (seeded from `AGENTS.md`), so the two refer to
the same set and the intent is coherent. Flagged only because a reader of a
scaffolded project seeing both phrasings could momentarily wonder whether two
different principle sets are meant. Not a correctness defect; optional to unify
the phrasing for one-source clarity (Principle 1). No change required if the
author judges the current wording clear enough.

## Severity tallies

- critical: none.
- high: none.
- medium: none.
- low: 2 (R1 line-wrapping regression; R2 wording variance).

Both deliverables' Principle checks pass: the contract is stated once and
referenced elsewhere (Principle 1, one-source/coherence), and no new phase or
role was added (Principle 2, minimal). No completeness or correctness gap found
against the six deliverables.
