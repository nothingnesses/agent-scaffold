# Triage: human-onboarding (Q-9)

Artifact: commit `27e6171` (diff `cdcd6c8..27e6171`), the "Getting started, for
the human" section added to `pack/AGENTS.md` and its generated mirrors, plus the
README pointer. Two independent reviews adjudicated: opus (R1) and sonnet (S1,
S2). Judged against the numbered Project Principles (Principle 1: prioritise
correctness, internal coherence, and maintainability; the pack is a shipping
artifact scaffolded by users at arbitrary commits) and the step detail for
`human-onboarding`, `deliberation-mode`, and `human-review-queue`.

Grounding facts confirmed by reading the current pack at this commit:
- `pack/plan-template.md` Open Questions section (lines 37-40) requires, for any
  undecided or blocking item, "the viable approaches, their trade-offs, a
  recommendation, and the reasoning". This grounds the open-questions case.
- `pack/AGENTS.md` Human-requests/intake (lines 100-108): the orchestrator
  reports a recommended routing and "The human decides". This grounds intake.
- Escalation/impasse: `pack/AGENTS.md` line 146 and `pack/prompts/orchestrator.md`
  lines 60-61 say only "escalate to a human with the ledger for a decision". No
  structured options, trade-offs, or recommendation are required at escalation.
- Push-at-checkpoint: neither `pack/prompts/orchestrator.md` nor
  `pack/plan-template.md` contains any "update the queue and push open items to
  the human at each checkpoint" instruction. The plan-template Open Questions
  section is static (author removes resolved entries; nothing pushes them).
- `human-review-queue` (Q-10) owns the push-at-checkpoint machinery;
  `deliberation-mode` (Q-12) owns the full human-input contract (structured
  options at every human-input point, including escalation). Roadmap order:
  `human-onboarding` (next) -> `deliberation-mode` -> `human-review-queue`, both
  machinery steps NOT started and sequenced AFTER this one.

## R1 (opus): undefined "orchestrator"/"checkpoint" jargon in the newcomer section

Verdict: VALID. Severity: low (confirmed).

Reasoning: The section opens "New to this workflow?" (line 9) and its Q-9
validation criterion is that a human with no prior context can start a task and
know their standing duty (plan line 310). It then uses "orchestrator" (lines 16,
20) and "checkpoint" (line 20) as established vocabulary; in top-to-bottom reading
order both first appear here, before the Workflow section defines "orchestrator"
(line 42) and with "checkpoint" never defined for a first-time reader. A no-context
human meets undefined jargon in the exact section meant to onboard them. Impact is
genuinely small: the concrete duty (watch the Open Questions section, decide when
a decision is raised) is stated in plain terms regardless, so the undefined terms
slow comprehension but do not block the task. Low is correct, not medium.

Recommended minimal fix: add a one-clause gloss at first use of "orchestrator",
e.g. "the orchestrator (the agent that drives the workflow)". "checkpoint" needs
no separate gloss if the S1 fix below removes the "pushes at each checkpoint"
clause (see interaction note). Orchestrator-applyable; no sequencing dependency.

## S1 (sonnet): push-at-checkpoint promised but not backed by the pack

Verdict: VALID. Severity: medium (confirmed).

Reasoning: This finding has two parts and both hold.

1. Unbacked forward reference. Lines 20-22 promise the orchestrator "raises with
   you at each checkpoint so you resolve the open items there rather than having to
   hunt for them". Confirmed: no push-at-checkpoint instruction exists anywhere in
   the pack at this commit (orchestrator.md has the review-loop, intake,
   isolation, and file-safety instructions but nothing that pushes the queue at
   checkpoints; plan-template's Open Questions section is static). That machinery
   is `human-review-queue` (Q-10), not started, sequenced after this step. So the
   onboarding text describes a discipline the pack does not yet enforce.

2. Internal contradiction, independent of the forward reference. The same
   paragraph says "Keeping an eye on that section as the work proceeds is the main
   standing thing asked of you" (a pull/monitor duty) alongside "rather than having
   to hunt for them" (a push guarantee that removes the need to monitor). The two
   clauses give a mixed message and, worse, the push clause undercuts the section's
   own purpose: the section exists to make the human watch the queue, and the
   "rather than hunt" clause tells them they need not. This self-defeating tension
   is what lifts the finding above low.

Why this is a real defect NOW under Principle 1, not an acceptable intra-task
forward reference: the pack is a shipping artifact (published to crates.io;
scaffolded by users at arbitrary commits, not only at task end). A user who
scaffolds at this commit gets an AGENTS.md that promises push-at-checkpoint and an
orchestrator.md that never delivers it, so the shipped pack is internally
incoherent, and the false promise directly causes the failure mode the section is
meant to prevent (the human under-monitors the queue because told they will be
pushed to; decisions accumulate silently; the workflow stalls). Principle 1's
"internal coherence" is a per-commit property for a shipping pack, so the
"fulfilled at task end" defense (option b) is weak: it leaves the pack shipping a
false promise for the whole duration of the human-interface cluster. Medium is
correct: the impact is a functional break in the primary human-interface, bounded
to the window before `human-review-queue` lands and to users who scaffold in that
window, which is why it is not high.

Recommended minimal fix: option (a), SOFTEN now to describe only what the pack
enforces today, and let `human-review-queue` STRENGTHEN it when the push machinery
lands. Concretely, replace the "which the orchestrator raises with you at each
checkpoint so you resolve the open items there rather than having to hunt for
them" clause with a pull-model statement, e.g. "which you should check as the work
proceeds; the orchestrator brings its recommendation to you when a decision is
needed." Keep the final "Keeping an eye on that section ... is the main standing
thing asked of you" sentence, which then becomes fully consistent. This removes
both the unbacked promise and the internal contradiction in one edit, and also
deletes the undefined "checkpoint" term (helping R1).

Reconciling with Q-9's design intent: Q-9 asks the section to state that "the human
decides on them when the orchestrator pushes them at a checkpoint", so the push
language is what Q-9 wanted. The resolution is that Q-9's intent is fulfilled at the
CLUSTER level (human-onboarding + human-review-queue + deliberation-mode), not at
this single commit. Stating a promise the pack cannot yet keep violates Principle
1's per-commit coherence; softening now and restoring the push language when
`human-review-queue` makes it true honours both Q-9's eventual intent and per-commit
coherence. Option (a) serves Principle 1 and the mid-cluster scaffolding user
strictly better than option (b).

## S2 (sonnet): impasse/escalation structured-options claim slightly over-stated

Verdict: VALID. Severity: low (confirmed).

Reasoning: Lines 16-19 promise that "when the agents reach a question, an impasse,
or a trade-off, the orchestrator lays out the options, their trade-offs, a
recommendation, and its reasoning, and you decide." Confirmed grounded for the
question and trade-off cases (plan-template Open Questions requires exactly
approaches/trade-offs/recommendation/reasoning) and for intake (AGENTS.md intake
reports a recommendation, the human decides). NOT grounded for the "impasse" case:
current escalation says only "escalate to a human with the ledger for a decision"
(AGENTS.md line 146; orchestrator.md lines 60-61), with no structured options. The
full structured-options-at-every-input-point contract is `deliberation-mode`
(Q-12), not started. Low is correct: the primary trigger the text describes (open
questions) is fully covered, so this is an over-claim at one edge case, not a
fabricated promise.

Recommended minimal fix: for per-commit coherence consistent with S1, remove "an
impasse" from the enumerated list so the structured-options promise covers only the
grounded cases, i.e. "when the agents reach a question or a trade-off". This is
lower priority than S1: `deliberation-mode` is the IMMEDIATE next step and, when it
lands, it both makes escalation produce structured options and re-adds "impasse" to
this enumeration with the full contract. The orchestrator may reasonably either
apply this one-word soften now or accept it as a single-step forward reference given
deliberation-mode lands next; I recommend the soften for consistency with S1.
Orchestrator-applyable; no sequencing dependency.

## Sequencing note

No finding forces a Roadmap reorder. Softening (S1 option a, and the S2 soften)
makes `human-onboarding` self-consistent at its own commit without requiring
`human-review-queue` or `deliberation-mode` to land first, so `human-onboarding`
does NOT hard-depend on those steps and all three fixes are orchestrator-applyable
now. There is no genuine human sequencing decision required.

Two light plan-bookkeeping follow-ups should be recorded so the softened text is
upgraded when the machinery lands (otherwise the pack permanently under-describes
the intended model):
- Add to `human-review-queue` (Q-10) scope: strengthen the onboarding section to
  restore the push-at-checkpoint guarantee once the orchestrator's push rule
  exists.
- Add to `deliberation-mode` (Q-12) scope: restore "impasse" to the onboarding
  enumeration once escalation requires structured options.

These are plan edits the orchestrator can apply; they do not need a human decision.
The only path that would need a human decision is the alternative to softening,
reordering the Roadmap so `human-review-queue`/`deliberation-mode` precede
`human-onboarding`, which I recommend against: softening is a single clause, keeps
every commit coherent, and avoids reworking a sequence the plan already committed
to.
