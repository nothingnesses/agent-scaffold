# Triage: `gate-prompt-clarity` (Q-20)

Diff range: `fc30bb3..41cf5ca`. Adjudicated against the numbered Project Principles
(esp. Principle 1 coherence / one-source-of-truth, Principle 2 minimal) and the
`gate-prompt-clarity` step detail. Verdicts below; the producer did not raise these
and I am independent of both the producer and the orchestrator.

Two reviewers, three findings (R1, R2 from opus; S1 from sonnet). No high/critical
finding was raised, so no dismissal needs second-triager re-check.

---

## R1: open-questions-gate glues the if/otherwise branches into one paragraph

Verdict: VALID. Severity: low (confirmed).

Reasoning. The diff removed the blank line that previously separated the "If so ..."
branch from the "Otherwise, state that none are open ..." branch. The branch-specific
sentence "Prefer the approaches most consistent with the project's principles." (which
belongs only to the if-branch) now sits immediately before "Otherwise", inside one
paragraph, so the two-way gate decision (open items exist / none do) is no longer
visually explicit. This is a real readability regression and it was not required by the
Q-20 intent: the step scope asks only to fix the routing wording and the README label,
not to reflow this paragraph. It is low severity because the meaning is still
recoverable on a careful read; it is purely a presentation defect, not a correctness one.
It is in scope for this step (the change introduced it).

Recommended minimal fix. Restore a blank line before "Otherwise" in
`pack/prompts/open-questions-gate.md` (and re-render the `.agents/` mirror), so the
if-branch and the else-branch are separate paragraphs again:

    ... for the human to review and choose
    from. Prefer the approaches most consistent with the project's principles.

    Otherwise, state that none are open and proceed with the next steps.

---

## R2: "surfaces" used as a verb, against the maintainer's stated style preference

Verdict: VALID. Severity: low (confirmed).

Reasoning. Both new routing sentences use "which surfaces them and relays ...". The
maintainer's documented global style preference lists "surface" as a verb among the
phrasings to avoid (prefer "raise" / "show" / "report" / "relay"). The two mitigations
offered do not hold up as reasons to dismiss:

- The "existing house style" defence is weak. A grep finds "surface" as a prose verb
  shipped in exactly one place (`pack/principles.toml:544`, "surface staleness"); the
  `surface-open-questions-before-implementing` id/name on line 38-39 is an identifier,
  not prose usage. One shipped instance is not an established house style; if anything
  line 544 is itself a later-cleanup candidate. So these two new instances are new
  drift toward a disfavoured verb, not conformity to a norm.
- The plan-intent text (step detail) using "surfaces them" is descriptive prose
  describing the routing, not a binding requirement to use that exact word. The step's
  decided requirement is that the routing be stated, not that it be worded with
  "surface".

Low severity: purely a wording preference, no behavioural impact. It is in scope for
this step, which owns "the routing wording" in both gate prompts. Dismissal would be
defensible (these prompts are agent-facing and the intent text uses the word), but
because the fix is trivial and aligns with a documented, active preference, fixing is
the better call.

Recommended minimal fix. In both `pack/prompts/clarifying-questions.md:15` and
`pack/prompts/open-questions-gate.md:15` (and the `.agents/` mirrors), replace the
"surfaces them and relays the human's ..." construction with wording that drops the
verb, reusing "relay" which is already in the sentence, for example:

- clarifying-questions: "... return your questions to the orchestrator, which relays
  them to the human and returns the answers."
- open-questions-gate: "... return the open items to the orchestrator, which relays
  them to the human and returns the decisions."

(`pack/principles.toml:544` is out of scope for this step; leave it, or clean it up
separately.)

---

## S1: routing claim ungrounded in orchestrator.md

Verdict: VALID. Severity: low (downgraded from the reviewer's medium).

Reasoning. Both gate prompts now assert an orchestrator behaviour, "the orchestrator
surfaces them and relays the human's answers/decisions", and this is a Principle 1
cross-reference: for the assertion to be coherent, the duty must live somewhere
authoritative that the orchestrator reads. The change opened this loop by replacing the
old direct "ask me" wording with explicit sub-agent -> orchestrator -> human routing, so
the finding is a genuine coherence gap the change created, not a pre-existing one. That
makes it VALID.

I downgrade the severity to low because the gap is thinner than the reviewer frames it:

- The duty is largely already grounded, just not in `orchestrator.md`. The orchestrator
  is instructed to read `AGENTS.md` first (orchestrator.md:4), and `AGENTS.md:16-22`
  states, as orchestrator behaviour, that "when the agents reach a question or a
  trade-off, the orchestrator lays out the options ... and you decide", with those
  decisions collecting in the plan's Open Questions queue. That directly covers the
  open-questions-gate case and reasonably covers the clarifying-questions case. Under
  Principle 1 (state a rule once, do not duplicate), it is acceptable, even preferable,
  for this duty to live in canonical `AGENTS.md` rather than be copied into the role
  prompt. The residual gap is only that `AGENTS.md` states it in the human-facing
  section and frames it around the Open Questions queue, so the clarifying-questions
  relay is covered by inference rather than named explicitly.
- The failure mode (an orchestrator absorbing clarifying questions itself and bypassing
  the human decider) is bounded by the whole workflow being built around the human as
  decider and by the orchestrator's stated "does not plan/implement/review/triage
  itself" role. It is a documentation-coherence weakness, not a functional break, which
  is a low, not medium, rating.

In-scope-or-split call: SPLIT OUT (defer to `deliberation-mode`). Reasons:

1. `gate-prompt-clarity`'s declared file scope is exactly the two gate prompts and the
   README label; `orchestrator.md` is deliberately not in it. Pulling an
   `orchestrator.md` edit into this step widens its scope against Principle 2.
2. The proper durable home already exists and is a pending step. `deliberation-mode`
   (Q-8/Q-12, roadmap status "not started", sequenced immediately after this step) is
   LOCKED to generalize every human-input point, and its scope text names "an open
   question, or a clarifying question" explicitly, into ONE cross-cutting human-input
   contract stated once in `pack/AGENTS.md` and referenced from each point, and it
   explicitly touches `orchestrator.md` and both gate prompts. The orchestrator-side
   relay/"present options" duty for gate questions is squarely that step's job. Adding a
   bespoke sentence to `orchestrator.md` now would pre-empt that cross-cutting rule and
   likely be reworded when `deliberation-mode` lands, which is the opposite of Principle
   1 (one source of truth) and Principle 2 (minimal).

Recommended minimal fix. Do not edit any pack file for S1 in this step. Instead record
a one-line scope note now, folded into the `gate-prompt-clarity` completion entry and
into the `deliberation-mode` step detail, that `deliberation-mode` must make the
orchestrator's duty explicit: when a spawned role returns clarifying or open questions,
the orchestrator relays them to the human and returns the human's answers/decisions
before proceeding. This keeps the gate prompts' promise, closes the Principle 1 loop at
the layer that owns it, and does not lose the finding.

If the orchestrator instead judges the transient gap must be closed inside this step
(defensible, since this change opened it), the acceptable minimal in-place fix is a
single sentence in `orchestrator.md` in the escalation/routing area, e.g. "When a
spawned role returns clarifying or open questions, relay them to the human and return
the human's answers before continuing." My recommendation is the deferral, because
`AGENTS.md` already carries the substance and `deliberation-mode` is the coherent home.

---

## Summary of verdicts

- R1: VALID, low. Restore the blank line before "Otherwise" in `open-questions-gate.md`.
- R2: VALID, low. Drop "surfaces" as a verb in both gate prompts (reuse "relays").
- S1: VALID, low (downgraded from medium). Split out; defer the orchestrator relay duty
  to `deliberation-mode`, recorded as a scope note now. No `orchestrator.md` edit in
  this step.
