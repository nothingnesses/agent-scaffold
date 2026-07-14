# Round 2 review: human-onboarding (Q-9)

Reviewer: independent (round 2). Diff range reviewed: `cdcd6c8..e722a14`
(cumulative step diff). Files: `pack/AGENTS.md` (source), `AGENTS.md` and
`.agents/AGENTS.reference.md` (generated mirrors), `README.md` (pointer), plus
plan/ledger bookkeeping. Scope: verify the three round-1 fixes landed, then a
fresh coherence pass for any new defect. Judged against the numbered Project
Principles in `docs/plans/agent-scaffold.md`.

## Round-1 fixes: all verified

R1 (jargon gloss). Fixed. `pack/AGENTS.md` line 15-16 now reads "the orchestrator
(the agent that drives the workflow)", glossing the term at first use in the
newcomer section. "checkpoint" no longer appears in the section (removed by the S1
fix), so no undefined-jargon term remains.

S1 (push-at-checkpoint promise + internal contradiction). Fixed. The decision-duty
sentence is now a coherent pull model: "These decisions collect in the plan's
'Open Questions' section, the single human-decision queue; check that section as
the work proceeds, and the orchestrator brings its recommendation to you when a
decision is needed. Reviewing that section is the main standing thing asked of
you." The "at each checkpoint" push promise, the "rather than having to hunt"
clause, and the undefined "checkpoint" term are all gone. The pull-vs-push
contradiction is resolved: the two clauses (monitor the section; a recommendation
accompanies a decision when one is needed) are complementary, not conflicting.

S2 (ungrounded "impasse" trigger). Fixed. The decision-trigger enumeration now
reads "when the agents reach a question or a trade-off"; "an impasse" is removed.

## Coherence pass

Accuracy to what the pack enforces today. The retained decision-block claim ("the
orchestrator lays out the options, their trade-offs, a recommendation, and its
reasoning, and you decide") is grounded: `pack/plan-template.md` Open Questions
section (lines 37-40) requires "the viable approaches, their trade-offs, a
recommendation, and the reasoning" for any undecided/blocking item, and the
`pack/AGENTS.md` intake section (lines ~99-105) has the orchestrator report a
"recommended routing" and "The human decides". The two remaining triggers
(question, trade-off) are both covered by those mechanisms. The interim clause
"the orchestrator brings its recommendation to you when a decision is needed"
describes the decision-block content presented when a decision arises, not a
checkpoint-cadence push; it is the exact wording the round-1 triage approved as
the pull-model interim, and it does not reintroduce a push guarantee.

No remaining forward-reference to unlanded machinery. The human-review-queue push
("at each checkpoint") and the deliberation-mode impasse contract are both absent
from the section. The plan follow-ups recorded in `docs/plans/agent-scaffold.md`
(restore the push language under `human-review-queue`; restore "impasse" under
`deliberation-mode`) match the softened interim text, so the upgrade path is
consistent.

Usability and Q-9 intent (interim level). The section remains usable for a
no-context human: kickoff instructions are concrete (copy `kickoff.md`, fill the
brackets, paste), the user-prompts-vs-role-prompts distinction is stated, and the
standing decision duty is explicit ("Reviewing that section is the main standing
thing asked of you"). Q-9's intent is honoured at the interim (pull) level.

README mirror is still a pointer. `README.md` lines 60-63 direct the reader to the
"Getting started, for the human" section of the scaffolded `AGENTS.md` and give
only a one-clause gloss; no workflow content is restated (Principle 1 intact).

Mechanical checks. The "Getting started" section is byte-identical across
`pack/AGENTS.md`, `AGENTS.md`, and `.agents/AGENTS.reference.md` (generated mirrors
in sync via `just scaffold-self`). All four changed files are ASCII-clean (no
em-dashes, emoji, or unicode symbols). The diff touches only the expected files;
no scope creep.

No new defect found. The edits removed the round-1 issues without introducing a
new over-claim, contradiction, or forward reference.

## Severity roll-up (new findings)

- critical: none.
- high: none.
- medium: none.
- low: none.

Fixes verified; no new findings.
