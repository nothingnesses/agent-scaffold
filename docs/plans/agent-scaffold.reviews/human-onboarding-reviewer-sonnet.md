# Review: human-onboarding (Q-9) - Reviewer Sonnet

Diff reviewed: cdcd6c8..27e6171
Lens: forward-references to not-yet-landed machinery, and duplication.

## S1 (medium): Push-at-checkpoint promised but not backed by orchestrator instructions

Location: `pack/AGENTS.md` lines 20-22 (the new "Getting started, for the human" section, second paragraph):

> "These decisions collect in the plan's 'Open Questions' section, the single
> human-decision queue, which the orchestrator raises with you at each checkpoint so
> you resolve the open items there rather than having to hunt for them."

The push-at-checkpoint discipline - "the orchestrator updates the queue at every
checkpoint as a required step and pushes the open items to the human" - is what
`human-review-queue` (Q-10, not started) is slated to add. In the current pack,
`pack/prompts/orchestrator.md` contains no such requirement. The orchestrator has no
instruction to push open items at checkpoints; it has instructions for the review
loop, intake, isolation, and file-safety, but nothing equivalent to "at each
checkpoint, push the Open Questions queue to the human." Similarly,
`pack/plan-template.md`'s Open Questions section is a static section with no push
rule: it tells the plan author to remove resolved entries, not the orchestrator to
raise them.

The consequence: a new human reading the onboarding section will expect to be
proactively notified at checkpoints. The current workflow will not deliver this,
because the orchestrator has no instruction to do so. The human may reasonably
conclude they do not need to check the section themselves, since they have been told
they will not "have to hunt for them." If the orchestrator does not push (and it
currently will not), decisions accumulate silently.

The section partially hedges with "Keeping an eye on that section as the work
proceeds is the main standing thing asked of you," but the "rather than having to
hunt for them" clause in the same sentence actively contradicts the pull-only reality
of the current pack. The two clauses in the paragraph create a mixed message: monitor
it yourself, but also you will be pushed so you do not have to hunt for it.

The claim is not a trivial cluster-reinforcement of existing machinery; the push
direction of the queue does not yet exist anywhere in the pack.

## S2 (low): Human-input contract slightly over-stated for the impasse/escalation case

Location: `pack/AGENTS.md` lines 16-19 (second paragraph of the new section):

> "when the agents reach a question, an impasse, or a trade-off, the orchestrator
> lays out the options, their trade-offs, a recommendation, and its reasoning, and
> you decide."

For open questions (covered by `pack/plan-template.md`'s Open Questions section:
"record the viable approaches, their trade-offs, a recommendation, and the
reasoning") and for intake (covered by the Human requests section: "a recommended
routing; the human decides"), the description is grounded in current pack content.

For the "impasse" case specifically - escalation when the total-round cap fires - the
current pack says only "escalate to a human with the ledger for a decision, then
apply it and resume" (in both `pack/AGENTS.md` Convergence section and
`pack/prompts/orchestrator.md`). No structured options, trade-offs, or
recommendation are required at escalation. The full cross-cutting human-input
contract ("at every point where the workflow needs human input, the agent presents
the viable options or approaches, the trade-offs of each, a recommendation, and the
reasoning") is what `deliberation-mode` / Q-12 formalizes. That step is not started.

Severity is low (not medium) because the primary trigger point the onboarding text
is describing - open questions, where the plan template already requires
options/trade-offs/recommendation/reasoning - is fully covered. The impasse case is
a gap at the edge, not a fabricated claim. The deliberation-mode step will close it.

## No duplication finding

The README change is a pointer ("see the 'Getting started, for the human' section"),
not a restatement. The new section itself summarizes behavior from the human's
perspective rather than restating workflow rules verbatim. Principle 1 is not
violated.

## No scope-overreach finding

The section describes observable outcomes for the human ("you decide," "keep an eye
on that section") without specifying the machinery of `deliberation-mode` or
`human-review-queue` (no new roles, no decision-block format, no queue-item schema).
The over-reach risk is covered by S1 and S2 as accuracy issues, not scope
overreach.

## Usability note (no separate finding)

Paragraph 1 (kickoff, two-directory explanation) is clear and usable. The standing
duty is explicitly stated. The usability concern - that the false push expectation
in S1 could cause a human to under-monitor the Open Questions section - is the
practical consequence of S1 and is captured there.
