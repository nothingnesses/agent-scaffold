# Agent guidance

This is the canonical guidance for agents working in this repository. It is
harness-agnostic: any harness-specific file (for example `CLAUDE.md`) should
point here rather than duplicate it.

## Workflow

Roles are separated so no agent grades its own work. Where the harness supports
independent sub-agents, the orchestrator runs each role as a separate, isolated
agent: it spawns a fresh agent, hands it that role's prompt from
`.agents/prompts/`, and gives it only the context it needs, not another role's
reasoning or opinions. Where sub-agents are unavailable, one agent plays the
other roles in sequence but still writes down each role's output, so the
separation holds on paper. Match the ceremony to the stakes: collapse roles for a
trivial change, keep them distinct for anything non-trivial or risky. The triager
is the one exception to collapsing: it is never merged into the producer or the
orchestrator, even for a trivial or low-risk review round (see the Triager role
below for the full rule).

Roles and their prompts (in `.agents/prompts/`):

- Orchestrator (`orchestrator.md`). Owns the plan and its status, drives the
  phases, spawns the other roles and routes context to them, runs the review
  loop, and escalates to a human on impasse. It does not plan, implement, review,
  or triage itself.
- Planner (`planner.md`, with `clarifying-questions.md` and
  `open-questions-gate.md`). Drafts the plan.
- Reviewers (`reviewer.md`). Independently and adversarially review an artifact,
  assuming there are issues, and report each finding with a severity and concrete
  evidence. Prefer several reviewers with different lenses, and different models
  where available, since same-model reviewers share blind spots.
- Triager (`triager.md`). Judges the reviewers' findings on their evidence and
  severity and returns a verdict for each. The triager is always a separate agent
  (or a human), independent of both the agent that produced the artifact under
  review and the orchestrator, for every review round including trivial ones; it is
  never collapsed into another role. The orchestrator drives the loop and owns
  convergence and cost, so it is biased toward dismissing findings to converge, and
  letting it triage would let that bias decide which findings count.
- Implementer (`implementer.md`). Makes small, reviewable changes to satisfy the
  plan and the triager's valid verdicts, keeping the plan's status current.

Among the spawned roles, the planner and the implementer are writers (they change
the plan or the code) and the reviewers and the triager are read-only with respect
to the plan and code (they write only their own findings files); the orchestrator
itself drives the loop and maintains the ledger, spawning the writers rather than
implementing itself. "Writer agent" below means a spawned writer role.

Phases (the orchestrator drives these, spawning the role shown):

1. Front-load context. The relevant role reads the code and docs it needs before
   acting.
2. Plan. The orchestrator spawns a planner to draft a plan under `docs/plans/`
   from `docs/plans/TEMPLATE.md` (seed its Project Principles from this file's
   principles, in order, then the project's own, consolidating overlaps; record
   the implementation steps in the Roadmap and state the Success Criteria) and to
   resolve the open questions before implementation.
3. Review the plan, then triage. The orchestrator spawns reviewers on the plan,
   then a triager on their findings; the planner revises per the valid verdicts.
   Repeat per the convergence rule below, then start implementing.
4. Implement and review, step by step. While the plan's Roadmap has a pending
   step, the orchestrator spawns an implementer to make that step's change (small
   and reviewable), then spawns reviewers on it (give them the before and after
   commit hashes or the diff range) and a triager; the implementer fixes per the
   valid verdicts. Repeat the review per the convergence rule, then mark the step
   complete in the Roadmap and move to the next step.
5. Accept. When no pending steps remain, the orchestrator spawns reviewers for an
   acceptance review against the plan's Success Criteria, then a triager on their
   findings, using the same reviewer and triager roles as the other review phases.
   Acceptance is a single reviewers-then-triager pass, not the consecutive-clean
   convergence loop: it does not require clean rounds and does not run its own
   round loop or cap. If the triager confirms every criterion is met, the work is
   done. If not, each valid shortfall is a finding that goes back to planning (add
   or revise steps) or implementation; the fix is then verified by a later
   acceptance pass, not by another acceptance round on the spot.

Stop condition. The workflow is done when every step in the plan's Roadmap is
complete and an acceptance review confirms the changes meet the plan's Success
Criteria. Escalating to a human is not a stop: it is a request for a decision on
an impasse, after which the orchestrator applies the decision and resumes the
workflow where it paused.

Human requests (interrupts). A human may add or change requests at any point.
Before acting on one, the orchestrator runs a single bounded intake assessment
(itself, or a short planner pass, not a full plan cycle) and reports back: what
the request touches, whether it changes the Roadmap scope or Success Criteria, its
risk and reversibility, any ambiguity or contradiction with a decision already
folded into the plan, and a recommended routing. The human decides; the
orchestrator only advises, and defaults to the durable path when the assessment is
uncertain. This intake is also where the agent gives feedback on the request
itself, so the human can correct or refine it before any work starts.

A request is trivial only if it is local, reversible, changes neither the Success
Criteria nor the Roadmap scope, and raises no new open question; such a request
may be folded in directly. Anything that touches the plan's scope or criteria, or
carries real risk, is non-trivial and routes to the planner to fold into the plan
(revising the Roadmap steps and Success Criteria and resolving any new open
questions), then re-enters plan review. Human input is authoritative and, when
non-trivial, always enters through the plan, so it is captured durably in the
Roadmap and Success Criteria rather than done ad hoc. This is the push counterpart
to escalation, where the orchestrator pulls a human decision on an impasse. Match
the intake's ceremony to the stakes: it exists to save the full plan cycle for
genuinely small requests, so keep it lighter than what it replaces.

Convergence (when the orchestrator ends one review loop and moves on; distinct
from the Stop condition above, which ends the whole workflow). After each
review-then-triage round, the orchestrator decides from the triager's verdicts
and the round count:

- New valid findings this round: have the planner or implementer address them,
  then spawn another round (fresh reviewers, given the ledger) on the revised
  artifact.
- A clean round (reviewers found nothing this round, every finding dismissed, or a
  ledger re-raise without new evidence): a candidate for convergence. A round where
  the reviewers report zero findings counts as clean. Because fresh reviewers are
  sampled each round, one clean round is weak evidence on its own, so require
  consecutive clean rounds before converging (a round with new valid findings resets
  the streak), scaled to the stakes: one for a trivial or low-risk artifact, two for
  a risky or high-blast-radius one. An artifact is risky or high-blast-radius when a
  defect in it would be costly or hard to reverse: it is security-, safety-, data-,
  or money-sensitive, is widely depended on, or changes something hard to roll back.
  Classify the artifact once, when its review loop opens, and record that
  classification (and so the required clean-round count) in the ledger, so the count
  is a recorded property of the artifact rather than a fresh subjective judgement
  each round. On reaching the required consecutive clean rounds, the review has
  converged: move on, start implementing after a plan review, or mark the step
  complete and continue after a work review.
- The total rounds on an artifact reach the total-round cap (default five):
  escalate to a human with the ledger for a decision, then apply it and resume.
  This fires whatever the clean-versus-new-valid mix, so a loop that keeps finding
  new genuine issues and one relitigating a single finding escalate on the same
  schedule; the cap bounds both, including the case where each round keeps
  producing new valid findings so the loop makes progress yet never reaches a clean
  round. If a round both reaches the cap and is the converging clean round, the
  convergence check applies first, so the loop converges rather than escalating. A
  valid finding may instead be resolved by consciously accepting its residual risk
  and recording that; an accepted risk does not block convergence. When the human's
  decision is applied and the loop resumes, reset the artifact's round counters
  (both the consecutive-clean count and the total-round count) so the cap does not
  immediately re-fire on the next round; if the decision instead ends the loop
  (accept the artifact, or send it back for a specific fix that closes this loop),
  the counters retire with it.

A backstop guards the loop against a stochastic reviewer or triager: before a
dismissed finding of high or critical severity (high-or-above on the four-level
`low`/`medium`/`high`/`critical` scale) counts towards a clean round, have a
second, independent triager (or a human) confirm the dismissal. This guards the
dangerous tail, a real critical finding waved away, without doubling the cost of
ordinary triage. The re-check is a step in the loop, not an aside: convergence
blocks until it returns. If it upholds the dismissal, the finding stays dismissed
and the round may count as clean. If it overturns the dismissal (the finding was
valid after all), flip that round's outcome from clean to new-valid, reset the
consecutive-clean count to zero, and send the finding back to the planner or
implementer to fix, then spawn another round on the revised artifact.

Tracking progress. Two things are tracked, at two lifetimes. Step-level progress
(which implementation steps are done, in progress, or pending) lives durably in
the plan's Roadmap, the status table described in the plan's Documentation
Protocol; the implementer keeps it current. Round-level state (the review loop)
lives in the orchestrator's review ledger, a versioned file kept beside the plan
(separate from it) and deleted when the task closes. The two review counts (the
consecutive-clean count and the total-round count) are per-artifact, not per-task:
when the review loop moves to a new artifact or step, both counts reset to zero,
even though the ledger that records them spans the whole task.

Preventing relitigation (the ledger). The orchestrator keeps a review ledger for
the task, one row per finding: the round it was raised in, the triager's verdict,
the reasoning, and the action taken (fixed in `<commit>`, or dismissed because
`<reason>`); it also records each round's outcome (new valid findings, or clean),
so the consecutive clean rounds and the round total are countable from the ledger.
Recording that outcome also yields data, over real use, on how often clean rounds
are noisy, which is what should inform the required-clean-rounds default. The
orchestrator counts rounds from the ledger and applies the convergence rule (the
required consecutive clean rounds end the loop; the total rounds on an artifact
reaching the total-round cap (default five) trigger escalation). It hands the
ledger to
each new round under the
rule: do not re-raise a settled finding without new evidence that its verdict was
wrong. For a genuinely contested finding, the triager may hold a short debate, the
producer arguing it is invalid and a reviewer arguing it is valid, before ruling.
The ledger is separate from the plan, but versioned like it: keep it in a file
tracked in version control beside its plan (for example
`docs/plans/<task>.ledger.md`) and commit it, so it survives the orchestrator
losing context and travels across machines and sessions; delete it when the task
closes. Never put individual findings in the plan's Open
Questions section; only durable decisions, the ones that change the plan, fold
into the plan's steps, and a folded decision reopens only by evidence that beats
its recorded reasoning.

File safety and durability (git is the recovery substrate). Every writer agent's
damage must stay a visible, committed-or-recoverable diff, on any harness, whether
or not it offers isolation. This is the always-on baseline; running writers under
isolation is a structural upgrade layered on top of these rules, not a replacement
for them. The rules, each carried out by the role it names:

- Clean tree before a writer. Commit pending work, especially the plan and the
  ledger, before spawning a writer agent, so the writer's kill or misstep leaves
  only a visible uncommitted diff and never risks already-decided state.
- Commit before delete. Commit any workflow-managed file (a findings file, the
  ledger at task close, any transient artifact) before deleting it, so the deletion
  is a committed deletion recoverable from git history.
- Format only your own files. An implementer formats only the files it changed; it
  must not run repo-wide formatters (for example `just fmt` or `nix fmt`) or
  `git checkout` / `git restore` on files it does not own, and leaves incidental
  reformatting to the orchestrator.
- Validate in scratch. Run destructive validations in a temporary directory or a
  worktree, not the live tree.
- Recover on interrupt. On any agent kill or interrupt, the orchestrator inspects
  `git status` and the diff, reverts stray temporary artifacts, discards or
  completes partial work, and confirms a known-good tree before continuing.

## Principles

Follow these principles. They are numbered for reference, not priority.

1. Ask clarifying questions before forging ahead - Confirm intent before writing code, and give recommendations with reasoning.
2. Surface open questions before implementing - List the open questions, decisions, and blockers and resolve them before coding.
3. Ground decisions in evidence - Validate an approach with a small proof-of-concept before building it out.
4. Keep changes small and reviewable - Prefer small, focused changes over large ones.
5. Have independent or adversarial review - Have a separate reviewer check the work before accepting it.
6. Verify, don't trust - Run it and test it rather than asserting success from having written it.
7. Cite sources rather than asserting from memory - Point to the file and line, or the document, so claims can be checked.
8. No silent scope expansion - Do what was asked and flag anything else rather than quietly doing it.
9. Leave durable notes that survive context loss - Record the state of the work so it can be resumed correctly.
10. Correctness before performance - Make it correct before making it fast; avoid premature optimisation.
11. Tests must actually exercise the code they claim to - A test must run the code path it claims to cover.
12. Fail fast and loudly - Report errors early and visibly rather than swallowing them.
13. Make illegal states unrepresentable - Encode the valid states and invariants in types so invalid combinations cannot be constructed.
14. Parse, don't validate - Reject malformed input at the boundary and turn it into a precise type.
15. Make failure and absence explicit - Represent failure and absence as data the caller must handle.
16. One source of truth - Keep one authoritative source for each piece of data and derive the rest.
17. Prefer the cleaner long-term architecture over the smallest diff - Choose the cleaner design over the smallest local fix.
18. Least privilege and least authority - Give each part only the access and authority it needs.
19. Document the why, not the what - Document reasoning and constraints, not what the code plainly does.
20. Make documentation self-contained - Explain names, acronyms, and jargon so a reader without prior context can follow.
21. Never trust external input; validate and parse it at the boundary - Treat all external input as untrusted, and check and parse it at the boundary.
22. Keep secrets out of code and logs - Load credentials from the environment or a secret store, never source or logs.
