# Review ledger: workflow-hardening review

Transient working state for the review loop. Separate from the plan; deleted at
task close. Committed so it survives context loss and travels across sessions.

Task: independently review the workflow-hardening changes (commits `8ce9454`,
`1d15f22`, `8f03a9c`; diff range `f2ca59e..8f03a9c`) for correctness, quality,
and consistency with the project's principles, then address valid findings.

Interrupt requests (received during this task, to fold into the plan via the
planner, not done ad hoc):

- `human-onboarding`: add a "Getting started, for the human" section to the
  canonical `AGENTS.md` with one inline editable kickoff prompt, plus a brief
  README mirror pointing to it (approach B + thin C). Decided by the human.
- `ledger-template`: make the review ledger a scaffolded template asset (lean:
  a reference asset under `.agents/`, copied per task), so the ledger schema is
  pinned rather than invented ad hoc. Recommended; human leaning yes.

- `deliberation-mode`: acknowledge question-driven (Socratic) human input as a
  first-class entry mode; generalise the intake / clarifying-questions /
  Open-Questions machinery so the orchestrator answers a human question with
  options, trade-offs, a recommendation, and reasoning (human decides); bind each
  resolved question to a durable Open-Questions decision; define convergence as
  the human committing a decision to action, guarded by the existing "no re-raise
  without new evidence" rule. LOCKED by the human: approach B plus the thin C
  naming slice (not a new phase or role). Pairs with `human-onboarding`.
- `human-review-queue`: make the plan's "Open Questions, Decisions, Issues and
  Blockers" section a single living human-decision queue, each item with a stable
  id, a one-line ask, a status (open / decided -> folded into <step> / superseded)
  and a context pointer; resolved items marked resolved (not deleted) to prevent
  double-addressing; the orchestrator updates it at every checkpoint as a required
  step. Recommended (reuse Open Questions, no second artifact). Human-interface
  cluster with `deliberation-mode` and `human-onboarding`.

Also to fix in the planner pass: the plan's Open Questions section says "No open
questions remain", which is stale (the convergence fix, the queued feature
designs, and the 14 review findings are all open). Rewrite it as the living queue
above.

All of these are non-trivial (touch the canonical pack assets), so they route to
the planner in a single pass alongside the triager-valid review findings.

Human decisions (this checkpoint): single planner pass covering everything;
`human-review-queue` = approach A (reuse the Open Questions section) upgraded with
B's discipline (stable ids, explicit status, resolved-marked-not-deleted).
Refinement: a new human would not know to check the queue, and a pull-only model
is fragile, so the queue must be PUSH + pull: the orchestrator surfaces open items
at each checkpoint, and `human-onboarding` documents the human's ongoing duty to
decide on them. Planner (opus) dispatched to fold everything into the plan.

Planner returned. Plan now has six new Roadmap steps (`convergence-accounting`,
`workflow-doc-fixes`, `human-onboarding`, `deliberation-mode`,
`human-review-queue`, `ledger-template`), a rewritten living Open-Questions queue
(Q-1..Q-10 with recommendations), and two added Success Criteria. F3 (the "not
committed" contradiction) corrected in the plan's resume-anchor narrative
immediately, rather than waiting for `workflow-doc-fixes`, since it would mislead a
resuming agent. Next: plan review (independent reviewers on the updated plan), then
the human resolves the open decisions to converge the plan, then implement. Open
decisions pushed to the human: Q-1 (convergence-accounting, headline) plus
Q-2..Q-7 (recommendation-backed); Q-8/Q-9/Q-10 already decided.

Human decided Q-1 = approach (b): drop the contested-rounds cap, keep a single
total-round cap (default five). Marked decided in the plan queue. Q-2..Q-7 remain
at their recommended defaults, awaiting confirmation. Round 2 = plan review: two
independent reviewers (A plan-soundness, opus; B consistency/principles, sonnet)
dispatched on the updated plan, told Q-1 = (b). Triager to follow.

## Round summaries

| Round | Artifact          | Changed since prev  | Outcome            | Valid findings | Consecutive clean |
| ----- | ----------------- | ------------------- | ------------------ | -------------- | ----------------- |
| 1     | hardening changes | n/a (first)         | new valid findings | 14 (1H/8M/5L)  | 0                 |
| 2     | updated plan      | folded + 4 features | new valid findings | 12 (1H/5M/6L)  | 0                 |

## Round 2 (plan review) reviewer findings, pre-triage

Artifact: the updated plan (docs/plans/agent-scaffold.md), reviewed with Q-1 = (b).
Reviewer A (soundness, opus) returned. Root theme: Q-1 = (b) was marked decided in
the queue but not propagated into the step body, the core-assets narrative, the
Q-1 decision block body, `workflow-calibration`, F17's rationale, and the Success
Criteria wording, so the plan still carries the two-cap / "contested" scheme in
about five places; and the out-of-band F3 fix is not reconciled in the
`workflow-doc-fixes` step list.

- A-H1 (high). `convergence-accounting` step body (lines 251-253) still reads as
  pending/gated and hedges under approach (a) (incl. the cross-round finding-id
  option); rewrite to the concrete (b) design: drop the contested cap, one total
  cap of five, a two-way {clean, new-valid} partition, no contested state, no
  finding ids.
- A-M1 (med). No step owns removing the two-cap description from the plan's own
  core-assets narrative (line 143); the "stay contested after (default three)"
  clause has no owner.
- A-M2 (med). F17's fix is scoped pre-(b): under (b) there are TWO constants, not
  three, and the narrative's contested cap must be removed, which F17 does not do.
- A-M3 (med). The Q-1 decision block heading says decided but the body still says
  "the human confirms" and "I lean to (b)" (consequence of the partial marker edit).
- A-M4 (med). `workflow-calibration` still lists and plans to calibrate the
  contested-rounds cap (dangling two-cap reference mooted by (b)).
- A-M5 (med). F3 is already fixed in the narrative but `workflow-doc-fixes` still
  lists it as pending, a contradiction that would re-fix a resolved item.
- A-L1 (low). Success Criteria says "escalation caps" (plural); under (b) one cap.
- A-L2 (low). F4/F5/F16 edit the same convergence/clean-round text that
  `convergence-accounting` rewrites first; ordering hazard not noted.
- A-L3 (low). `ledger-template` gating not marked explicitly like
  `convergence-accounting`'s.
- A no-findings: F8/F9 are NOT mooted by (b) (both surviving counters remain and
  are already scoped to them); finding coverage complete and non-overlapping; the
  three human-interface steps match their decided scope; no critical.

Reviewer B (consistency/principles, sonnet) still running; triager on the combined
set after B returns.

New human request (interrupt, during round 2): a structured, machine-parseable
state file for progress/tasks/agents, that the tool can parse and validate (the
orchestrator runs the tool to check it), enabling future visualisation; minimise
sources of truth. Intake done. Recommendation: Approach 1 (derived projection),
prose (Roadmap, decision queue, ledger) stays the single source; the tool gains a
`validate` / `status --json` subcommand that validates the state docs and emits a
JSON projection for viz; exclude authoritative live-agent state (harness owns it;
at most an append-only dispatch/return event log). Build on `ledger-template` +
the Roadmap; unify one schema with `human-review-queue` and `instrument-flag`.
Route to the planner as `Q-11` + a candidate step (e.g. `state-schema`/`validate-command`),
SEQUENCED AFTER the current convergence-accounting and doc-fixes so the in-flight
plan converges first. Awaiting the human's approach lock (now or after plan review
converges).

Reviewer B (consistency/principles, sonnet) returned. Corroborates A's Q-1=(b)
propagation theme and adds several planner artifacts. Merged unique round-2
findings G1-G13 handed to the triager:

- G1 (A-H1,B-H1,B-L1,B-L3): convergence-accounting step body still pending/gated,
  hedges under (a), dead finding-id parenthetical, repeats `next` status label.
- G2 (A-M1,A-M2,B-H2,B-M1): core-assets narrative keeps the two-cap/"three
  constants" scheme; no step owns fixing it; F17's description now wrong under (b).
- G3 (A-M4,B-H3): workflow-calibration still calibrates the dropped contested cap.
- G4 (A-M3,B-M2): Q-1 decision block heading decided but body in pre-decision voice.
- G5 (A-L1,B-M5): Success Criteria says "caps" (plural); one cap under (b).
- G6 (A-L2): F4/F5/F16 edit the same text convergence-accounting rewrites; ordering.
- G7 (A-L3): ledger-template gating not marked explicitly.
- G8 (B-M3): decided queue items embed decision summaries instead of a step pointer
  (contradicts the Documentation Protocol / one source of truth).
- G9 (B-M4): Q-1 decision block sits under Open Questions, not folded into the step.
- G10 (B-M6): "Principle 13"/"5" mix the plan's 1-7 numbering with AGENTS.md's.
- G11 (B-L2): "I lean to" AI phrasing (prohibited by the project style).
- G12 (B-L4): push rule duplicated (Documentation Protocol + Open Questions header).
- G13 (B-L5): status line's "next job = codebase review" stale vs Roadmap.

Triager (opus) dispatched on G1-G13 against the current plan.

Round 2 triager verdicts: 12 valid, 1 invalid. Valid: G1 (high); G2, G3, G4, G9,
G13 (medium); G5, G6, G8, G10, G11, G12 (low). Invalid: G7 (ledger-template deps
are already documented in its body and the queue; no real asymmetry). No
high-severity finding dismissed. Judgment calls resolved by the triager: G8 trim
decided queue items to ask + status + pointer; G9 fold the Q-1 decision block into
the `convergence-accounting` step (do not park it in Open Questions); G10 use the
plan's own 1-7 numbering (drop "13"), and sweep lines 80/85 too. Round 2 outcome:
NEW VALID FINDINGS (not clean); consecutive-clean stays 0.

Orchestrator error caught: A-M5 (the F3 fix is applied but its `workflow-doc-fixes`
bullet still lists it pending) was dropped during the hand-merge into G1-G13, so it
was never triaged; it is plainly valid and is folded into the revision. Lesson:
hand-merging findings can silently drop one (supports Q-11).

Planner revision dispatched to fold Q-1 = (b) through and fix the 12 valid findings
plus A-M5. Then a focused convergence check (round 3).

Planner revision returned; all 12 valid findings + A-M5 addressed. Orchestrator
grep verification of the revised plan (objective residual check): no live
contested-rounds cap remains (the only "contested" mentions are the legitimate
triager-debate concept and the step text recording the drop); two round constants,
a single total-round cap; Success Criteria uses one cap; no "Principle 1X"
numbering leakage; no pre-decision "the human confirms" voice; the F3 bullet is
marked done. One out-of-scope residual: `greenfield-flake` still says "the
recommendation leans to (a)" (a pre-existing style nit, unrelated to Q-1; noted for
a later style sweep, not fixed). Round 3 convergence decision pending: a formal
one-reviewer verification round versus accept-as-converged (the round-2 independent
review plus triage already did the judgment; round 3 only verifies mechanical
fixes, which the grep largely confirms). Human decisions: D1 = (a) accept the plan as converged (no round-3 reviewer; match
ceremony to stakes). Q-7 = a four-level severity RATING (low/medium/high/critical);
the dismissed-finding guard triggers on high-or-above (validated against CVSS
practice: an absolute rating, not a relative ranking; prioritisation like EPSS/KEV
is a separate overlay). Q-2/Q-3 = the ledger template is a `.agents/` reference
asset, the per-task copy is a working file. Q-4/Q-5/Q-6 (instrument-flag) and Q-11
(structured state) remain open but non-blocking (deferred). Plan CONVERGED and
accepted.

Implementation begins (workflow phase 4). Step 1, `convergence-accounting`
(Q-1 = (b), single total-round cap; addresses F1/F2/F7/F10): implementer dispatched
to edit the pack sources and regenerate the self-scaffolded copies, then reviewers

- triager on the change.

Step 1 `convergence-accounting` implemented (implementer). Edited pack/AGENTS.md
(convergence bullets now name the single total-round cap; "Two backstops" collapsed
to the dismissed-high-severity guard; ledger paragraph -> single cap),
pack/prompts/orchestrator.md (step 3 escalation -> total-round cap only), README
diagram (both edges -> "round cap reached"), CHANGELOG Unreleased (single cap).
Regenerated the self-scaffold. Verified: no residual contested cap/state (only the
legitimate triager-debate "contested finding"); boundary wording identical across
the three docs ("reach the total-round cap (default five)"); 46 tests pass, clippy
clean, fmt clean, ASCII-clean. Not committed.

NEW FINDING (found during implementation; needs a plan item): `include_dir!` in
src/manifest.rs embeds pack/ but does NOT register those files as cargo rebuild
dependencies, so `just scaffold-self` can regenerate the root AGENTS.md and
`.agents/` from a STALE embedded pack. The bootstrap commit's committed `.agents/`
copies were in fact stale (missing the hardening content); a forced
`cargo clean -p agent-scaffold` + rebuild caught them up in this working tree.
Impact: scaffold-self is unreliable without a forced rebuild, and the planned
golden test would be fooled by the same staleness. Fix candidate: a build.rs
emitting `cargo:rerun-if-changed=pack`. Route to the planner as a new step (e.g.
`pack-rebuild-tracking`), prioritised before the golden test and before relying on
scaffold-self. Surfaced to the human.

New human request (interrupt): make the workflow ALWAYS present, at every point
that needs human input (escalation, intake, open questions, clarifying questions),
the viable options/approaches, the trade-offs of each, a recommendation, and the
reasoning, with the reasoning judged against the current numbered Project
Principles of the project the workflow is used on. Intake: non-trivial (changes the
workflow's human-input contract); routes to the planner. Recommendation: Approach A
(one cross-cutting "human-decision contract" rule in `AGENTS.md`, referenced from
each point, not duplicated) plus a light standard decision-block format, scaled to
stakes; fold into `deliberation-mode` (generalise it from Socratic questions to all
human-input points) and strengthen the escalation step, which currently lacks
structured options. Judge recommendations against the plan's own Principles by
number. Awaiting the human's approach lock.

Human adopted all recommendations: (1) `convergence-accounting` work review = one
focused reviewer; (2) queue the `include_dir` finding as `pack-rebuild-tracking`
(prioritised); (3) structured human-input contract = Approach A + light C,
stakes-scaled, folded into `deliberation-mode`, strengthening escalation; (4)
Q-4/Q-5/Q-6 confirmed at the recommended defaults; (5) Q-11 held (deferred); (6)
`greenfield-flake` nit deferred to a later style sweep. Next: one reviewer on the
convergence-accounting change, then converge/commit/mark it done; then a batched
planner pass to fold `pack-rebuild-tracking` and the human-input contract into the
plan and mark the queue decisions. One reviewer (opus) dispatched on the
convergence-accounting change.

Convergence-accounting work review returned: no critical/high/medium; F1/F2/F7/F10
resolved, no residual contested cap/state, guards and rules preserved. 4 low
wording-consistency nits: (1) "of five" vs "(default five)" in the AGENTS.md ledger
paragraph; (2) README label "round cap reached" vs prose "total-round cap"; (3) the
cap bullet sits under "decides from the triager's verdicts" though it is
round-count-driven; (4) AGENTS.md leaves the converge-before-escalate precedence
implicit (orchestrator.md states it). All clear, undisputed consistency nits;
accepted as valid without formal triage (match ceremony to stakes). Implementer
resumed to apply the four fixes, regenerate (with the cargo-clean rebuild for the
include_dir staleness), and re-verify. Then converge, commit, mark the step done.

Convergence-accounting committed (19d69e5); the four nits landed and re-verified
(46 tests, clippy/fmt clean, ASCII-clean); step marked complete, `workflow-doc-fixes`
now next. The commit also lands the corrected `.agents/` copies (stale-bootstrap
drift fixed in git).

Human decision (triager structure): ALWAYS a separate triager, NEVER collapsed
(Option 3). The triager must be independent of BOTH the producer AND the
orchestrator (the orchestrator drives the loop and owns convergence/cost, so it is
biased toward dismissing findings to converge), for every review round including
trivial ones. This narrows the workflow's "collapse roles for a trivial change"
rule to except the triager. From now on: a separate triager for every review round.

Batched planner pass dispatched to fold into the plan: `pack-rebuild-tracking` (the
include_dir fix, prioritised); the human-input contract (into `deliberation-mode`);
the `triager-independence` / no-collapse rule; the queue decisions (Q-2/Q-3/Q-7
decided, Q-4/Q-5/Q-6 confirmed, Q-11 added as held). Then implement
`workflow-doc-fixes`.

Planner pass returned. Added `pack-rebuild-tracking` (build.rs), `triager-independence`,
`state-schema` (Q-11, deferred); extended `deliberation-mode` (human-input contract)
and `workflow-doc-fixes` (Q-7 four-level rating). Marked Q-2/Q-3/Q-4/Q-5/Q-6/Q-7
decided; added Q-11/Q-12/Q-13. Reviewing the plan change: one reviewer (consistency
lens); a SEPARATE triager adjudicates anything it finds, per the always-separate
rule.

Plan-maintenance reviewer returned: 0 critical, 2 high, 3 medium, 3 low.
- H1: `state-schema` cites "Principle 5, one source of truth", but the plan's P5 is
  make-illegal-states; one-source-of-truth is P1 here (AGENTS.md-numbering leak).
- H2: Q-2/Q-3/Q-4/Q-5/Q-6 marked "decided -> folded into <step>" in the queue, but
  the target steps still read as open recommendations / design sub-questions
  (`instrument-flag` still lists the three as open; `ledger-template` still frames
  Q-2/Q-3 as recommendations). Queue and step contradict; decision detail sits in
  the queue, not the step. (Q-7 folded correctly.)
- M1: Q-11/12/13 and the rewritten Q-2..Q-7 embed full decision detail in the queue
  (against the Documentation Protocol; should be brief ask + pointer; inconsistent
  with the terse Q-1/8/9/10).
- M2: the umbrella says "eight steps" but `state-schema` is a ninth block placed
  inside the run, excluded from the count/enumeration.
- M3: `workflow-doc-fixes` states the Q-7 decision twice (a "recommendation ...
  Q-7" bullet and a "decided" paragraph).
- L1/L2/L3: state-schema repeats its status label; Q-11's status parenthetical
  exceeds the status vocabulary; "prioritised" competes with the Roadmap `next`.
Recurring theme: the queue-vs-step consistency error (same class as round 2 G8/G9),
evidence that the hand-maintained living queue is error-prone and supports the
`state-schema` / validate-command. Separate triager (opus) dispatched to adjudicate,
per the always-separate rule.

Triager verdicts: all 8 valid; re-severitised to 1 high (H2) + 7 low (H1, M1, M2,
M3, L1, L2, L3). H2 (high): the queue asserts Q-2..Q-6 decided while
`ledger-template` / `instrument-flag` still read as open recommendations, with the
decision detail misplaced in the queue. Judgment calls: M1 -> hold to the terse
ask + status + pointer queue form, the decision lives in the step (the "Decided:"
gloss is what let the drift in); L1 -> the leading status-word is a plan-wide
convention, do not single out `state-schema` (leave it). Planner revision
dispatched to fix H2 + H1/M1/M2/M3/L2/L3 (L1 left per the triager). Then
orchestrator grep-verification of the specific fixes; converge if clean (the
independent review + separate triage already did the judgment; a re-review would
only re-verify mechanical fixes).

Planner revision returned and grep-verified clean: no "Decided:" gloss in the
queue; `ledger-template` no longer frames Q-2/Q-3 as recommendations;
`instrument-flag` now states "Decided design (Q-4/Q-5/Q-6)"; the two remaining
"Open sub-questions" belong to `optional-modules` / `tui-authoring` (genuinely
undecided); Principle citations use the plan's 1-7; the umbrella count and the
F4/Q-7 double-statement reconciled. Plan-maintenance review CONVERGED (H2 + lows
fixed). Committing the plan revision, then implementing `workflow-doc-fixes`.

## Findings

| ID  | Round | Severity | Triager verdict | Reasoning | Action |
| --- | ----- | -------- | --------------- | --------- | ------ |

Round 1: two independent reviewers dispatched (R1 correctness/termination lens,
opus; R2 cross-document-consistency/principles lens, sonnet). R1 returned (11
findings: 0 critical, 2 high, 6 medium, 3 low). R2 still running. Triager to be
spawned on the combined, deduplicated findings once R2 returns.

### Round 1 reviewer findings, pre-triage (R1: correctness / termination)

- R1-H1 (high). The recorded per-round outcome is binary (clean vs new valid
  findings), but the contested-rounds cap needs a third "contested" state that is
  never recorded or defined, and findings have no cross-round identity, so a
  re-spawned orchestrator cannot compute the contested cap from the ledger.
  (`orchestrator.md` steps 1-3; `AGENTS.md` convergence bullets and line ~136.)
- R1-H2 (high). The convergence taxonomy is not a partition: "new valid findings"
  and "still-contested valid findings" both describe non-clean rounds with no
  boundary, and the two caps use different numbers (3 vs 5) for identically
  recorded rounds, making the escalation trigger nondeterministic.
- R1-M1 (med). The dismissed-high-severity re-check keys on an undefined severity
  scale; it says "high-severity" but its stated purpose is catching a "critical"
  finding, which a critical-above-high scale would exclude. Severity levels are
  never enumerated in `reviewer.md`/`triager.md`.
- R1-M2 (med). The second-triager re-check is ordered after the round outcome is
  recorded, and the procedure never says what to do if the re-check overturns the
  dismissal (flip outcome, reset streak, refix). The blocking relationship is
  implied, not a step.
- R1-M3 (med). Acceptance review is described "as in the other review phases"
  (implying the convergence loop) but the README diagram draws it as a single pass
  with no loop, cap, or escalation. Prose and diagram disagree.
- R1-M4 (med). Off-by-one between prose ("exceed" / "past" / "after") and the
  diagram ("cap hit"); the boundary phrasings resolve to different round numbers.
- R1-M5 (med). Counts are per-artifact but the ledger is per-task; no rule resets
  the consecutive-clean count and round total when moving to a new artifact/step,
  so a re-spawned orchestrator must infer segmentation that is never specified.
- R1-M6 (med). The escalation-resume path does not reset the round counters, so a
  total-cap escalation can immediately re-fire on resume; the intended semantics
  are undefined.
- R1-L1 (low). "Risky / high-blast-radius" artifact classification is undefined
  yet selects the required consecutive-clean count (one vs two); subjective and
  not recorded.
- R1-L2 (low). The "clean round" definition ("every finding dismissed") does not
  explicitly cover the common zero-findings case.
- R1-L3 (low). The README diagram routes a trivial interrupt into the "implement
  the next step" node, a dead-end when no steps are pending, and mismatches the
  prose (the orchestrator folds a trivial request in directly).

### Round 1 reviewer findings, pre-triage (R2: consistency / principles, sonnet)

- R2-H1 (high). The plan doc (`docs/plans/agent-scaffold.md`, `core-assets`
  narrative) says the ledger is "a durable scratch file (not committed)",
  contradicting `AGENTS.md`, `orchestrator.md`, and `CHANGELOG.md` ("committed
  beside its plan"). In the resume-anchor doc, so it would mislead a resuming
  agent into the opposite design.
- R2-M1 (med). `AGENTS.md` convergence bullets list only the contested-rounds cap;
  the total-round cap lives in the separate "backstops" section, so an agent
  scanning the bullets as a checklist gets an incomplete escalation rule.
- R2-M2 (med). Acceptance "as in the other review phases" is ambiguous about
  whether the convergence loop applies; the diagram shows a single pass. [= R1-M3]
- R2-M3 (med). README intake caption "human decides routing" overstates: the prose
  has the orchestrator fold trivial requests in directly.
- R2-M4 (med). README `intake -> impl` for trivial interrupts is wrong when the
  interrupt arrives during plan review (no pending step). [= R1-L3]
- R2-M5 (med). `AGENTS.md` source line-wrap splits "the review / has converged"
  (cosmetic; Markdown soft-wrap, to be judged).
- R2-M6 (med). `CHANGELOG.md` line-wrap leaves "guarded" at a line end (cosmetic).
- R2-L1 (low). `AGENTS.md` convergence bullet 3 says "a bounded number of rounds"
  not "contested rounds", ambiguous against the total cap. [related R1-M4]
- R2-L2 (low). The plan calls them "three round constants" when one (required
  consecutive clean rounds) has two values.

### Combined unique findings F1-F17 handed to the triager

Merged (deduped): F6 = R1-M3 + R2-M2; F7 = R1-M4 + R2-L1; F12 = R1-L3 + R2-M4.
F1/F2 (deep convergence-rule defects) kept distinct from F10 (bullet omits total
cap). Triager (opus) dispatched to adjudicate all 17 against the current sources.

### Round 1 triager verdicts (14 valid, 3 invalid)

Valid, to fix: F1 (high); F2, F3, F4, F5, F6, F7, F9, F12 (medium); F8, F10, F15,
F16, F17 (low).
Invalid, dismissed: F11 (the diagram caption faithfully matches the prose); F13,
F14 (cosmetic Markdown soft-wrap, renders as continuous text).

No high-severity finding was dismissed, so the dismissed-high-severity
second-triager guard did not trigger. Round 1 outcome: NEW VALID FINDINGS (not a
clean round); consecutive-clean stays 0.

Themes of the valid findings:

- Convergence accounting (F1 high, F2, F7, F10): the "contested" round state the
  contested-rounds cap depends on is never defined or recorded, the round taxonomy
  is not a partition, cap-boundary wording is inconsistent, and the bullets omit
  the total cap. Core defect; needs a design decision on the fix.
- Ledger/plan contradiction (F3): the plan says the ledger is "not committed" vs
  the prompts/CHANGELOG "committed beside its plan".
- Under-specified mechanics (F5 re-check overturn handling, F8 counter reset across
  artifacts, F9 escalation-resume counter reset, F15 risk classification, F16
  zero-findings clean round).
- Severity scale undefined (F4): guard triggers on "high" but aims at "critical".
- Acceptance loop ambiguity (F6): "as in other phases" vs single-pass diagram.
- Diagram vs prose (F12): trivial-interrupt edge routes to impl, a dead-end
  pre-implementation.
- Nit (F17): "three round constants" imprecise (one has two values).

Next: single planner pass folding the 14 valid findings plus the three queued
feature requests into the plan; then plan review, implement, round 2 review.

---

## Post-189b7bd notes (re-added; an implementer's fmt + git checkout clobbered these)

INCIDENT / lesson: the `workflow-doc-fixes` implementer ran repo-wide `just fmt`
(which reformats this ledger and the plan as a side effect), then reverted the
ledger with `git checkout docs/plans/agent-scaffold.ledger.md` to honour "do not
touch the ledger". That reverted the ledger to HEAD (189b7bd), discarding the
uncommitted Q-14 notes. Mitigations going forward: (a) commit the ledger before
spawning an implementer, so its uncommitted state is not at risk; (b) implementers
must NOT run repo-wide `just fmt` or `git checkout` on files they do not own (fmt
only their files; leave incidental reformatting for the orchestrator). A real
workflow finding (candidate plan item); reinforces committing the ledger
frequently and the structured-record / findings-files work.

Q-14 (findings-to-files) DECIDED by the human: adopt. Reviewers and triagers write
their findings to per-agent files that other agents read directly (parallel-safe);
the ledger references them by path and keeps the decision trail; an
orchestrator-owned cleanup step deletes them once the round is fully resolved or at
task close. Compose with `ledger-template` (schema) and `state-schema`, one schema
and validator. Reasoning: Principle 5 (removes the lossy transcription hop that
dropped A-M5) and Principle 1 (findings file = single source, referenced not
copied). Fold into the structured-record cluster at the next planner touch;
non-blocking.

`workflow-doc-fixes` implemented (F4/F5/F6/F8/F9/F12/F15/F16/F17); regenerated,
46 tests/clippy/fmt clean, ASCII-clean; not committed. Reviewing it with TWO
reviewers (A correctness/completeness, opus; B consistency/principles, sonnet),
DOGFOODING Q-14: each reviewer WRITES its findings to a file under
`docs/plans/agent-scaffold.reviews/`, and the separate triager reads those files
directly (no orchestrator transcription). Files cleaned up at round close.

New human request (interrupt): queue the `optional-modules` agent-isolation work
(agent-images / agent-box) to prevent the cross-agent file-conflict class (the
git-checkout ledger clobber). Intake (per the human-input contract). Recommendation:
DECOUPLE the incident from the isolation feature. (1) Now, fix the incident cheaply
with discipline (implementers format only their own files, never `git checkout`
files they do not own; the orchestrator commits the ledger before spawning writers)
as a small workflow-rule plan item, sibling to `pack-rebuild-tracking`. (2) Pursue
isolation deliberately for its real value (parallel implementation, sandboxing),
NOT as the incident fix; when taken up, do WORKTREE isolation first (native to the
harness, proportionate to the file-conflict class) and the heavier `agent-box` /
`agent-images` CONTAINER isolation as a later follow-on (adds sandboxing/environment
isolation, a separate motivation). Reasoning: Principle 2 (minimal; do not
over-build the core for one incident), Principle 6 (evidence before heavy
investment), Principle 1 (isolation is the cleaner structural answer, so pursue it,
but deliberately and worktree-first). Awaiting the human's decision on queuing and
priority.

`workflow-doc-fixes` review (Q-14 findings files, no orchestrator transcription):
- Reviewer A (correctness): 0/0/0/3L -> `docs/plans/agent-scaffold.reviews/workflow-doc-fixes-A.md`.
- Reviewer B (consistency): 0/1H/2M/3L -> `.../workflow-doc-fixes-B.md`. The high:
  `orchestrator.md` omits "acceptance does not require clean rounds" (F6 drift vs
  `AGENTS.md`). Theme: the F6/F15/F5 clauses landed in `AGENTS.md` but were not
  mirrored into `orchestrator.md` (the operational prompt), plus backstop-guard
  wording drifts across AGENTS.md/orchestrator.md/triager.md.
Separate triager (opus, independent of the orchestrator and the implementer)
dispatched to adjudicate; it reads BOTH findings files directly (Q-14).

Agent-isolation decision (human, refining the earlier intake): adopt a
CAPABILITY-TIERED, harness-agnostic rule. Run each WRITER agent in the strongest
isolation the harness supports, preference order: (1) container isolation
(agent-box / agent-images) if available, PREFERRED (isolates filesystem +
environment + security sandbox); (2) worktree isolation if that is what is
available (e.g. claude-code native); (3) discipline-only fallback (scoped fmt, no
cross-owner git checkout, commit the ledger before writers) when the harness has no
isolation. Read-only agents (reviewers reading, triager) need no isolation
(Principle 2). The discipline fix is the always-on BASELINE, not replaced; isolation
is the structural upgrade layered on when available. "Container-preferred" is the
runtime selection rule (in the workflow docs); our own build/adoption order stays
worktree-first (native to claude-code, cheap now), agent-box container integration
later; the rule resolves to worktree for us today anyway. Fold into `optional-modules`
(isolation) plus a small discipline workflow-rule item at the next planner touch.

`workflow-doc-fixes` triage (separate triager, read both findings files per Q-14):
7 unique valid issues, 0 critical/high, 1 medium, 6 low. Medium = the F15 risk
definition + classify-once/record rule is in `AGENTS.md` but missing from
`orchestrator.md`, so the operational checklist re-opens the subjective per-round
1-or-2 classification. Low = the acceptance "no clean rounds" clause, the overturn
step-2 self-containedness (amend the summary line + spawn another round), the
high-or-above guard wording differing across AGENTS.md/orchestrator.md/triager.md,
the escalation reset naming "both counts", and two README diagram simplifications
(trivial-fold edge label-vs-target; acceptance-shortfall edge routes only to plan).
Theme: clauses landed in `AGENTS.md` but were not fully mirrored into
`orchestrator.md`; the nine findings themselves are correctly in place. Round
outcome: new valid findings (not clean). Fix = mirror the clauses into
`orchestrator.md` + the two README nits. Committing the ledger (protect it), then
resuming the implementer WITH the discipline rule (no repo-wide fmt, no git
checkout). Review files kept until the step converges, then cleaned up (Q-14).

`workflow-doc-fixes` round 2 (mirroring fixes) verified: all 7 fixes present in
`orchestrator.md` / `triager.md` / README (initial grep strings just failed across
line wraps). Converged (round 1 already had a full independent review + separate
triage; round-2 mirrorings are verified mechanical). Roadmap: `workflow-doc-fixes`
complete, `pack-rebuild-tracking` next.

MISTAKE: I deleted the Q-14 review files with `rm -rf` while they were UNTRACKED
(never committed), so their raw content is not recoverable from git; the findings
survive only in summary here and in the triager verdicts. Exactly the case the
human's new rule prevents.

New human rule (DECIDED): COMMIT BEFORE DELETING any workflow-managed file (findings
files, the ledger at task close, any transient artifact), so the deletion is a
committed deletion recoverable from git history. Resolves the Q-14
committed-vs-transient sub-decision toward COMMITTED (a findings file is committed at
least once, then removed as a committed deletion). Fold into Q-14's cleanup step,
the ledger delete-at-close lifecycle, and a general workflow rule. Applies from now.

`pack-rebuild-tracking` implemented (build.rs, std-only, recursive
`cargo:rerun-if-changed` for pack/). Validated by the orchestrator: 46 tests pass;
a pack edit re-embeds on a plain `just build` with NO `cargo clean` (marker reached
the scaffolded output, then reverted; tree clean). The implementer was accidentally
killed by the human mid-task but had already reverted its temp edit, so recovery was
just: inspect git status (only `?? build.rs`), re-validate.

Robustness-to-mishaps decision (human asked; per the human-input contract): a killed
or interrupted agent can leave the tree inconsistent. Adopt LAYERED robustness:
(baseline, now) a clean-tree-before-writer invariant (commit pending work before
spawning a writer, so a kill's damage is a visible uncommitted diff); an orchestrator
RECOVERY PROTOCOL on any kill/interrupt (inspect git status/diff, revert stray temp
artifacts, discard or complete partial work, confirm a known-good tree before
continuing); and validation-in-scratch (destructive validations run in a temp dir or
worktree, not the live tree). (structural, later) ISOLATION (already decided) is the
real fix, a killed isolated writer cannot touch the main tree; this is the SECOND
mishap (after the fmt/checkout clobber) that is evidence to prioritise it. Unifying
theme: git is the workflow's durability/recovery substrate. Fold into the
discipline/isolation cluster at the next planner touch.

`pack-rebuild-tracking` review: one reviewer (correctness/robustness of build.rs),
findings to a file per Q-14; separate triager to follow (always-separate rule). The
review file will be committed before cleanup, per the commit-before-delete rule.

Robustness-to-mishaps DECIDED by the human: adopt the layered robustness, baseline
now (clean-tree-before-writer invariant, orchestrator recovery protocol on any
kill/interrupt, validation-in-scratch) and isolation as the structural fix (already
decided; this mishap is added evidence to prioritise it). Fold into the
discipline/isolation cluster at the next planner touch.

`pack-rebuild-tracking` review (Q-14 file): reviewer 0/0/0/4L, all robustness gaps
that cannot currently trigger (pack/ is maintainer-controlled, all-ASCII,
symlink-free): symlink recursion, lossy `path.display()` on non-UTF8 names,
additions relying on directory mtime, and the relative "pack" root not derived from
`CARGO_MANIFEST_DIR`. Findings in `docs/plans/agent-scaffold.reviews/pack-rebuild-tracking-A.md`.
Separate triager dispatched (reads the file per Q-14) to adjudicate and recommend
which to fix vs accept-with-rationale.

New human requests (intake, per the human-input contract; awaiting the human's
decision):
- Q-15: a reusable compaction-prep USER prompt plus a workflow checkpoint/resume
  procedure (before a compaction, flush the plan + ledger + queue current and commit
  so state survives context loss; on resume, reconstruct from AGENTS.md + plan +
  ledger). Recommend adopt: a thin prompt + a short AGENTS.md "Checkpoint / resuming
  after context loss" section that reuses the existing durability rules; keep the
  wording harness-agnostic (plan/ledger/durable notes, not a specific memory).
- Q-16: separate USER prompts from AGENT/role prompts into their own directory (for
  example `.agents/user-prompts/`) holding the kickoff prompt and the compaction-prep
  prompt; `.agents/prompts/` stays the role prompts. Revises `human-onboarding` (the
  kickoff prompt moves to the dir, thin trigger-prompts do not duplicate workflow
  content; AGENTS.md points to it). Recommend adopt.
Both non-trivial -> planner; join the consolidating planner-pass backlog.

`pack-rebuild-tracking` triage (separate triager, read the file per Q-14): 4 valid
low. FIX Finding 4 (derive the `track` root from `CARGO_MANIFEST_DIR` to match the
`include_dir!` base, removing a silent-divergence footgun; one source of truth).
ACCEPT Findings 1 (symlink guard), 2 (lossy path.display), 3 (dir-mtime for
additions, already documented) with brief rationale comments, since pack/ is
maintainer-controlled, all-ASCII, symlink-free. Verdicts in
`docs/plans/agent-scaffold.reviews/pack-rebuild-tracking-triage.md`. Resuming the
implementer for the Finding-4 fix + rationale comments.

`pack-rebuild-tracking` converged: Finding 4 fixed (build.rs derives the pack root
from `CARGO_MANIFEST_DIR`), Findings 1/2/3 accepted with rationale comments; re-
validated (re-embed works on a plain build, 46 tests, clippy clean); src/ fmt
incidental changes reverted by hand (no git checkout). Roadmap: pack-rebuild-tracking
complete, triager-independence next. Committing the step AND the Q-14 review files
(so the review record is in git history), then a follow-up commit deletes the
review files (committed deletion, per commit-before-delete).

Q-15 and Q-16 CONFIRMED by the human at the recommendations. Consolidating planner
pass dispatched to fold the whole backlog of decided workflow rules into the plan
(Roadmap steps + Open-Questions queue, decision detail in the steps, terse pointers
in the queue): file-safety/durability discipline (commit-before-writers,
commit-before-delete, clean-tree-before-writer, no-repo-wide-fmt / no-cross-owner
git-checkout, orchestrator recovery protocol, validation-in-scratch); the
capability-tiered isolation rule (+ optional-modules integration); Q-14
findings-files; Q-15 compaction-prep prompt + checkpoint/resume section; Q-16
user-prompts directory (revising human-onboarding). Then resume implementing from
the updated plan (triager-independence onward).

Consolidating planner pass returned (plan +74/-4): 5 new steps (`file-safety-rules`,
`agent-isolation`, `findings-files`, `user-prompts-dir`, `compaction-prep`), revised
`human-onboarding`, queue Q-14..Q-18 (Q-17 file-safety, Q-18 isolation-tiering), 4
new Success Criteria, resume-anchor status line updated, 2 principle-numbering leaks
fixed proactively. Reviewing the plan change: one reviewer (consistency /
well-formedness, findings to a file per Q-14); a SEPARATE triager adjudicates
anything found (always-separate rule); review files committed before cleanup
(commit-before-delete).

Consolidating-plan review (Q-14 file `consolidate-A.md`): 0/0/2M/2L; the recurring
classes (queue-vs-step, well-formedness, principle numbering, status agreement) all
PASSED this time. Findings: (med) the Success Criteria contradict each other on
where the kickoff prompt lives vs the revised `human-onboarding`; (med) the Q-17
queue item duplicates its step's five-rule content (terse-queue); (low) Q-18
restates the decided tier order (terse-queue); (low) `findings-files` "share one
schema" is loosely worded and its cluster membership disagrees with `state-schema`.
Separate triager dispatched (reads the file per Q-14).

Consolidating-plan triage (separate triager, read the file per Q-14): 4 valid (1
med, 3 low), none dismissed. Med = a Success Criteria bullet still says the
Getting-started section "holds an editable kickoff prompt", contradicting the
revised `human-onboarding` (prompt moves to `.agents/user-prompts/`). Lows = Q-17
and Q-18 queue items restate step detail (terse-queue), and `findings-files` "one
schema" is loose + its cluster membership disagrees with `state-schema`. Verdicts
in `consolidate-triage.md`. Resuming the planner for the 4 fixes; then grep-verify
and commit the consolidated plan.

Consolidating-plan revision verified (grep): the Success Criteria bullet now "points
to" the kickoff prompt in `.agents/user-prompts/`; Q-17 and Q-18 trimmed to terse
pointers; `findings-files` reads "one schema per record type" with cluster
membership aligned to `state-schema`. Consolidated plan CONVERGED. Committing the
plan with the Q-14 review files (`consolidate-A.md`, `consolidate-triage.md`), then
a follow-up commit deletes them (committed deletion). The plan is now authoritative
and compaction-safe: every decided rule is a Roadmap step. Next: implement
`triager-independence`.

`triager-independence` implemented (commit bd483e1; before 0c95ab9): narrowed the
collapse-for-trivial rule so the triager is never collapsed, added
orchestrator-independence (not just producer-independence) to the Triager role
description, the orchestrator prompt's no-sub-agents fallback, and the triager
prompt. Classified LOW-risk (doc-only, easily reversible, not
security/data/money/widely-depended-on) -> one clean round required.

`triager-independence` round 1: two independent reviewers (opus consistency/
correctness, sonnet gaps/edge-cases; different models per the diverse-lens rule),
findings to Q-14 files (`triager-independence-reviewer-opus.md`,
`-reviewer-sonnet.md`; the opus reviewer ran read-only so the orchestrator
transcribed its report verbatim into the file). Reviewers: R1/R2 (opus, 1M/1L);
S1-S5 (sonnet, 2H/1M/2L). SEPARATE triager (always-separate rule) deduped 7 findings
into 5 groups: verdicts in `triager-independence-triage.md`. 3 VALID (Group A low =
AGENTS.md fallback still says "one agent plays THE roles" while orchestrator.md was
tightened to "the OTHER roles" same commit; Group B medium = the "(or a human)"
allowance present in orchestrator.md + both backstops is missing from the primary
triager rule in AGENTS.md's Triager bullet and triager.md; Group D low = the rule is
stated twice in AGENTS.md with slightly different wording). 2 INVALID: Group C (S2,
dismissed HIGH with full auditable reasoning: the single-agent no-human deadlock is
not introduced by this change; that config already cannot run the workflow, which
needs a human for escalation/cap/acceptance; closing an already-illegitimate path in
an unsupported config is not a regression) and Group E (S5, the backstop's unchanged
"independent" is now governed by the general rule, redundant not ambiguous). Round 1
outcome: NEW VALID FINDINGS (not clean); the dismissed-HIGH backstop re-check does
not gate this round because it is not a clean round. Fixes (Groups A/B/D) applied in
pack/ (commit bdebe9d), regenerated + re-reviewed in round 2.

`triager-independence` round 2 (fixes committed bdebe9d): one fresh verification
reviewer (opus, given the ledger so settled findings are not re-raised) confirmed all
three Group A/B/D fixes landed and swept the full text: all four statements
(AGENTS.md opening pointer + Triager bullet, orchestrator.md, triager.md) now agree
the triager is always a separate agent (or a human), independent of both producer and
orchestrator, every review round, never collapsed; generated mirrors byte-identical.
One new LOW cosmetic finding T1 (an orphaned line-wrap from the Group B reflow,
`triager-independence-round2-reviewer.md`). SEPARATE triager (sonnet, always-separate
rule) ruled T1 INVALID, precedented by F11/F13/F14 cosmetic soft-wraps
(`triager-independence-round2-triage.md`). Round 2 outcome: CLEAN (every finding
dismissed). Convergence: LOW-risk artifact needs one clean round; streak = 1 ->
CONVERGED. No dismissed-high/critical in round 2, so the backstop re-check does not
gate; round 1's S2 dismissal was in a new-valid round so it never gated a clean
round. T1 re-wrapped off-cycle (triager blessed it as optional, no new round) and
folded into the step-completion commit. Roadmap: `triager-independence` COMPLETE,
`file-safety-rules` next. Committing the round-2 review files (durable record), then a
follow-up commit deletes all five triager-independence review files (committed
deletion, per commit-before-delete).

`file-safety-rules` implemented (commit d86ec0f; before 742bcae): added an always-on
file-safety/durability section to `pack/AGENTS.md` (git as the recovery substrate:
clean-tree-before-writer, commit-before-delete, format-only-your-own-files,
validate-in-scratch, orchestrator recovery-on-interrupt) and folded the role rules
into `implementer.md` (format-only, validate-in-scratch) and `orchestrator.md`
(clean-tree, commit-before-delete, recovery). Classified LOW-risk (doc-only,
reversible) -> one clean round.

`file-safety-rules` round 1: two independent reviewers (opus completeness, sonnet
coherence/duplication; findings to Q-14 files `file-safety-rules-reviewer-opus.md`,
`-sonnet.md`). R1 (opus, low), S1/S2/S3 (sonnet, 2M/1L). SEPARATE triager
(`file-safety-rules-triage.md`) deduped to 3 groups, all VALID LOW (S1+S2 downgraded
med->low): Group A = "writer agent" is a loose category (never tied to roles) and the
"The rules:" lead-in admits a writer-vs-owner ambiguity; Group B = the implementer
defers "incidental reformatting to the orchestrator" but the orchestrator "does not
implement", so the duty had no permitted owner; Group C = commit-before-delete for the
ledger stated twice (pre-existing ledger paragraph + new rule) in both AGENTS.md and
orchestrator.md, a drift surface. No high/critical, so no backstop re-check. Round 1
outcome: NEW VALID FINDINGS. Fixes: (A) added a Roles-section sentence classifying
writers (planner, implementer) vs read-only (reviewers, triager) and defining "writer
agent", and reworded the lead-in to "each carried out by the role it names"; (B) added
a clause to `orchestrator.md` granting the orchestrator tree-hygiene reformatting
(distinct from implementing a step); (C) dropped the now-subsumed "committing the
deletion" clause from the ledger paragraph in both files, letting the canonical rule
own it. Regenerated; re-reviewed in round 2.

`file-safety-rules` round 2 (fixes committed a3adc50): one fresh verification reviewer
(opus, given the ledger) confirmed all three Group A/B/C fixes landed and mirrors in
sync; one new LOW finding T1 (`file-safety-rules-round2-reviewer.md`): the Group A
classification called reviewers/triager "read-only" though they write findings files.
SEPARATE triager (sonnet, `file-safety-rules-round2-triage.md`) ruled T1 VALID low,
fix now (agent-isolation reuses this carve-out). Round 2 outcome: NEW VALID. Fix
(commit aeea55b): qualified to "read-only with respect to the plan and code (they
write only their own findings files)".

`file-safety-rules` round 3 (fix committed aeea55b): one fresh reviewer (opus, given
the settled findings) verified the T1 fix landed with the exact wording, coherent with
commit-before-delete and the "writer agent" definition, all mirrors in sync, and the
final coherence pass found all five rules owned by the correct role with no unowned
duty / undefined term / contradiction (`file-safety-rules-round3-reviewer.md`). Round 3
outcome: CLEAN (zero findings; no triager needed, nothing to adjudicate). Convergence:
LOW-risk artifact needs one clean round; new-valid (r1), new-valid (r2), clean (r3) ->
streak = 1 -> CONVERGED. No dismissed high/critical, so no backstop re-check. Roadmap:
`file-safety-rules` COMPLETE, `agent-isolation` next. Committing the round-3 review file
(durable record), then a follow-up commit deletes all six file-safety-rules review
files (committed deletion, per commit-before-delete).

`agent-isolation` implemented (commit 032964a; before 57739c3): added the isolation
RULE only (mechanism stays deferred to `optional-modules`). New "Writer isolation
(capability-tiered)" section in `pack/AGENTS.md` (container via agent-box/agent-images
> worktree > file-safety fallback; read-only agents need none; mechanism framed as an
optional module) plus a tier-selection clause in `orchestrator.md`. LOW-risk (doc-only)
-> one clean round.

`agent-isolation` round 1: two independent reviewers (opus correctness, sonnet
coherence; `agent-isolation-reviewer-opus.md`, `-sonnet.md`). Opus: 0/0/0/3L; sonnet:
0/0/1M/1L. SEPARATE triager (`agent-isolation-triage.md`) deduped to 4 groups: 2 VALID
low, 2 INVALID. VALID: R2+S1 (structural-upgrade framing stated in both the file-safety
and isolation sections, a drift surface; sonnet's medium corrected to low) and S2 (the
isolation section re-enumerated role membership instead of reusing the Roles-section
"writer agent"/read-only definition). INVALID: R1 (tier list restated in orchestrator.md
is the established prompt style, AGENTS.md stays the source with a see-AGENTS.md pointer)
and R3 (plan status still `next` in the pre-convergence diff is expected; the orchestrator
flips status at convergence, not in the implementation commit). No high/critical -> no
backstop re-check. Round 1 outcome: NEW VALID. Fixes: (R2+S1) the file-safety sentence
now reads "running writers under isolation builds on it rather than replacing it (see
Writer isolation below)", and the isolation section owns "structural upgrade over the
file-safety baseline"; (S2) dropped the "(the planner and the implementer)" and "(the
reviewers and the triager)" parentheticals, using the defined terms. Regenerated;
re-reviewed in round 2.

`agent-isolation` round 2 (fixes committed ef875bd): one fresh reviewer (opus, given
the settled findings) verified both round-1 fixes landed (no structural-upgrade
duplication remains; the isolation section uses the bare defined terms), mirrors in
sync, tier order and read-only carve-out intact, mechanism still deferred to
optional-modules, no new contradiction (`agent-isolation-round2-reviewer.md`). Round 2
outcome: CLEAN (zero findings; no triager needed). Convergence: LOW-risk artifact needs
one clean round; new-valid (r1), clean (r2) -> streak = 1 -> CONVERGED. No dismissed
high/critical -> no backstop re-check. Roadmap: `agent-isolation` COMPLETE (the
isolation RULE; the MECHANISM stays deferred to `optional-modules`); the human-interface
cluster (`human-onboarding`, `user-prompts-dir`, `compaction-prep`) is next. Committing
the round-2 review file (durable), then a follow-up commit deletes all four
agent-isolation review files (committed deletion).

Sequencing note for the human-interface cluster (raised to the human at this checkpoint,
per the human-input contract): the Roadmap lists `human-onboarding` first, but its
"Getting started" section points to `.agents/user-prompts/`, which `user-prompts-dir`
creates (directory + kickoff/compaction/resume prompts + `pack/pack.toml` entries +
asset-list test). Recommend implementing `user-prompts-dir` FIRST (establish the
directory and moved prompts), then `human-onboarding` (the AGENTS.md pointer + README
mirror), then `compaction-prep` (checkpoint/resume section + the compaction and Q-19
resume prompts). These three are the first steps to touch real code (the manifest and the
`src/` asset-list test), not just docs. Awaiting the human's go-ahead / ordering
confirmation before starting the cluster.

Human confirmed the cluster ordering by asking the orchestrator for its recommendation;
orchestrator recommended and adopted `user-prompts-dir` -> `human-onboarding` ->
`compaction-prep` (build the directory the other two point into first; no forward
reference to a nonexistent path, Principle 1).

`user-prompts-dir` implemented (commit 9c1a88d; before 9f52ef0): created
`.agents/user-prompts/` with a thin kickoff prompt (`pack/user-prompts/kickoff.md`,
ownership reference), added the `pack/pack.toml` `[[asset]]` entry, extended the
`src/manifest.rs` built-in asset-list test (46 pass), and named the directory in the
README layout. First cluster step to touch real code (manifest + test + a shipped
asset). LOW-risk (reversible, test-guarded) -> one clean round.

`user-prompts-dir` round 1: two independent reviewers (opus mechanics, sonnet
content/scope; `user-prompts-dir-reviewer-opus.md`, `-sonnet.md`). Opus: 0 defects
(R1 informational, ownership="reference" justified). Sonnet: S1 medium, S2 low.
SEPARATE triager (`user-prompts-dir-triage.md`): R1 INVALID (not a defect); S1(a) status
still not `complete` INVALID/expected (convergence convention, agent-isolation R3
precedent); S1(b) the cluster reorder not reflected in the Roadmap table VALID low
(unlike R3, the implemented step was not the one marked `next`); S2 the "Getting
started" validation criterion is `human-onboarding`'s, untickable here, VALID low. No
high/critical -> no backstop. Round 1 outcome: NEW VALID (two low plan-bookkeeping
fixes; the implementation artifact itself had zero defects). Both fixes folded into the
completion update per the triager: reordered the Roadmap rows to `user-prompts-dir`
(complete) -> `human-onboarding` (next) -> `compaction-prep`, and moved the "Getting
started" criterion to `human-onboarding`'s ownership in the validate block. Verified
mechanically (the artifact was already found clean by both reviewers; the fixes are the
exact edits the triager prescribed, matching the `workflow-doc-fixes` round-2
verified-mechanical precedent). CONVERGED. Roadmap: `user-prompts-dir` COMPLETE,
`human-onboarding` next. Committing the round-1 review files (durable) with the
completion, then a follow-up commit deletes them (committed deletion).

`human-onboarding` implemented (commit 27e6171; before cdcd6c8): added a "Getting
started, for the human" section to `pack/AGENTS.md` (points to the kickoff prompt in
`.agents/user-prompts/`, distinguishes user-prompts from role prompts, states the
ongoing decision duty + Open Questions queue) and a brief README mirror that points to
it. LOW-risk (doc-only) -> one clean round.

`human-onboarding` round 1: two independent reviewers (opus correctness, sonnet
forward-references; `human-onboarding-reviewer-opus.md`, `-sonnet.md`). Opus: R1 low.
Sonnet: S1 medium, S2 low. SEPARATE triager (`human-onboarding-triage.md`): all three
VALID, no human sequencing decision required (softening makes the step self-consistent
at its own commit). R1 = "orchestrator"/"checkpoint" jargon used before definition. S1
= the section promised push-at-checkpoint ("raises with you at each checkpoint ... rather
than having to hunt") that the pack does not yet enforce (that machinery is
`human-review-queue`, Q-10, not started, sequenced later), plus an internal pull-vs-push
contradiction; a per-commit Principle 1 break since the pack ships to users who scaffold
mid-task. S2 = the structured-options-at-"impasse" case is ungrounded (current escalation
is only "escalate with the ledger"; the full contract is `deliberation-mode`, Q-12, not
started). Round 1 outcome: NEW VALID. Fixes: glossed "orchestrator" at first use;
replaced the push clause with the pull model the pack enforces today ("check that section
as the work proceeds, and the orchestrator brings its recommendation to you when a
decision is needed"), removing the false promise, the contradiction, and the undefined
"checkpoint"; dropped "an impasse" from the enumeration. Two plan-bookkeeping follow-ups
recorded (not human decisions): re-strengthen the onboarding push language when
`human-review-queue` lands, and restore "an impasse" when `deliberation-mode` lands
(added to those steps' details). Regenerated; re-reviewed in round 2.

`human-onboarding` round 2 (fixes committed e722a14): one fresh reviewer (opus, given
the settled findings) verified all three fixes landed (gloss present; pull-model text
with no push/checkpoint promise or contradiction; "impasse" removed), mirrors in sync,
the softened text accurate to what the pack enforces today, no new forward-reference,
still usable (`human-onboarding-round2-reviewer.md`). Round 2 outcome: CLEAN (zero
findings; no triager needed). Convergence: LOW-risk artifact needs one clean round;
new-valid (r1), clean (r2) -> streak = 1 -> CONVERGED. Roadmap: `human-onboarding`
COMPLETE, `compaction-prep` next (the last human-interface-cluster step). Committing the
round-2 review file (durable), then a follow-up commit deletes all four human-onboarding
review files (committed deletion).

Gate-prompt finding (`Q-20`, from a human question, transcript-recovered). The human
asked whether `pack/prompts/clarifying-questions.md` and `open-questions-gate.md` are
for humans. A compaction had flattened their real nature (agent-RUN, human-ANSWERED
decision GATES: the "me" in them is the human) into "agent-facing role prompts", so the
orchestrator first answered wrongly ("not for humans"). Combed the transcript
(jq/ripgrep over the 48MB session file): design intent is at [07-11T10:17] ("answer at
the gates: clarifying questions before work starts ... you're the decider there") and
the same discussion is where the kickoff prompt was born; git shows the files came from
`a135ff9`, and `704bbae` made open-questions-gate principle-agnostic after an earlier
human worry. Relationship: `kickoff.md` (human-invoked) is the ENTRANCE (human ->
workflow); the gates are where the running workflow comes BACK to the human (workflow ->
human); `human-onboarding`'s "Getting started" section documents both ends. DECIDED
(human): keep the gates as the planner's prompts (do not fold into `planner.md`), fix
the "ask me" routing (sub-agent -> orchestrator -> human), and correct the README's
"one prompt per role" mislabel. New step `gate-prompt-clarity` inserted before
`compaction-prep`. Recording the plan + this finding, then implementing + reviewing.

`gate-prompt-clarity` implemented (commit 41cf5ca; before fc30bb3): reworded both gate
prompts to name the human as decider + state sub-agent -> orchestrator -> human routing;
fixed the README `prompts/` "one prompt per workflow role" mislabel. LOW-risk -> one
clean round.

`gate-prompt-clarity` round 1: two independent reviewers (opus correctness, sonnet
consistency; `gate-prompt-clarity-reviewer-opus.md`, `-sonnet.md`). Opus: R1/R2 low.
Sonnet: S1 medium. SEPARATE triager (`gate-prompt-clarity-triage.md`): all VALID; R1
low (the reflow glued the "Otherwise" else-branch onto the "if so" paragraph), R2 low
("surfaces" as a verb against the maintainer's style preference; house-style claim weak
-> fix), S1 downgraded medium->low and SPLIT OUT (the gate prompts now assert an
orchestrator relay duty that `orchestrator.md` does not name; it is grounded in
`AGENTS.md` which the orchestrator reads, and `deliberation-mode` is already locked to
add the cross-cutting human-input contract touching `orchestrator.md` + the gate prompts,
so defer there rather than duplicate). No high/critical -> no backstop. Round 1 outcome:
NEW VALID. Fixes: R1 restored the blank line before "Otherwise"; R2 replaced "surfaces
them and relays the human's..." with "relays them to the human and returns the
answers/decisions" in both prompts; S1 recorded as a `deliberation-mode` scope note (no
`orchestrator.md` edit here). Regenerated; re-reviewed in round 2.

`gate-prompt-clarity` round 2 (fixes committed 593b71d): one fresh reviewer (opus, given
the settled findings) verified all three fixes (blank line restored; no "surface" verb;
S1 intentionally deferred with the `deliberation-mode` scope note present, orchestrator.md
untouched), mirrors in sync, no new defect (`gate-prompt-clarity-round2-reviewer.md`).
Round 2 outcome: CLEAN (zero findings; no triager needed). Convergence: LOW-risk, one
clean round; new-valid (r1), clean (r2) -> streak = 1 -> CONVERGED. Roadmap:
`gate-prompt-clarity` COMPLETE, `compaction-prep` next. Committing the round-2 review file
(durable), then a follow-up commit deletes all four gate-prompt-clarity review files
(committed deletion).

`compaction-prep` implemented (commit 226ca33; before 082280e): added the "Checkpoint
and resuming after context loss" section to `pack/AGENTS.md` and two human-invoked
prompts under `.agents/user-prompts/` (`compaction-prep.md` flush, `resume.md` pickup =
Q-19), with `pack/pack.toml` entries, asset-list test (46 pass), and README layout.
LOW-risk (reversible, test-guarded) -> one clean round.

`compaction-prep` round 1: two independent reviewers (opus mechanics, sonnet content;
`compaction-prep-reviewer-opus.md`, `-sonnet.md`). Opus: R1 low, R2 medium. Sonnet: S1
medium, S2 low, S3 low. SEPARATE triager (`compaction-prep-triage.md`) deduped to 3
groups, all VALID: Group 1 (R2+S1+S2, medium/low) = both prompts claimed "does not
restate" but their bodies enumerated the AGENTS.md procedure (coherence + a real drift
path: `findings-files` would add a durable artifact and silently stale
compaction-prep.md's list); R1 low = "the clean-tree-before-writer discipline covers
this" over-claims (shared mechanic, not shared trigger); S3 low = the closing note said
"durable notes" but the body's third item is the Open Questions queue. No high/critical
-> no backstop. Round 1 outcome: NEW VALID. Fixes: trimmed both prompt bodies to thin
triggers that name the target AGENTS.md section without enumerating its steps (kept the
"does not restate" claims, now honoured, and the prompt-specific framing); reworded the
over-claim to "the same commit-before-risk durability discipline as before a writer,
applied to a distinct trigger"; aligned the closing note to "the plan, the ledger, and
the plan's Open Questions queue". Regenerated; re-reviewed in round 2.

`compaction-prep` round 2 (fixes committed 16b1260): one fresh reviewer (opus, given the
settled findings) verified all three fixes (prompts now thin triggers naming the AGENTS.md
section, "does not restate" claims true; over-claim reworded; "durable notes" aligned to
the Open Questions queue), mirrors in sync, 46 tests pass, no new defect
(`compaction-prep-round2-reviewer.md`). Round 2 outcome: CLEAN (zero findings; no triager
needed). Convergence: LOW-risk, one clean round; new-valid (r1), clean (r2) -> streak = 1
-> CONVERGED. Roadmap: `compaction-prep` COMPLETE. The human-interface cluster
(`user-prompts-dir`, `human-onboarding`, `gate-prompt-clarity`, `compaction-prep`) is
DONE; `deliberation-mode` is next (it carries two recorded follow-ups: restore the
"impasse" trigger in `human-onboarding`'s Getting-started section, and make the
orchestrator's gate-relay duty explicit in `orchestrator.md`). Committing the round-2
review file (durable), then a follow-up commit deletes all four compaction-prep review
files (committed deletion).

Between-step checkpoint finding (`Q-21`, OPEN, from a human question). At a step
boundary the orchestrator paused with an ad-hoc "carry on or pause?" that was neither a
defined human-input point nor in the human-input-contract format. Two causes: (1) an
operator miss, the human-input contract (`Q-12`, still not in the pack until
`deliberation-mode` lands) was not self-applied, so the decision was a thin binary
instead of options/trade-offs/recommendation/reasoning; (2) a real design gap, the
workflow has the orchestrator drive step-to-step autonomously and defines no between-step
checkpoint. Recorded `Q-21` (open) for the human to decide the cadence (autonomous /
report-only / gate / configurable), recommendation (d)+(b). The FORMAT half is
`deliberation-mode`'s job. Human chose to proceed with `deliberation-mode` now; `Q-21`
stays open in the queue for the human's decision.

## RESUME STATE (compaction checkpoint, read this first)

We are DOGFOODING the role-separated workflow on this repo itself (it is
self-scaffolded: root `AGENTS.md`, `.agents/`, plan under `docs/plans/`). To resume
after a compaction: read `AGENTS.md` (the workflow), `docs/plans/agent-scaffold.md`
(the plan: Roadmap + Open Questions queue + Step Details; its Status line is the
resume anchor), and this ledger. Operate as the ORCHESTRATOR.

Current state: `convergence-accounting`, `workflow-doc-fixes`,
`pack-rebuild-tracking`, `triager-independence`, `file-safety-rules`, and
`agent-isolation`, `user-prompts-dir`, and `human-onboarding` are complete and
committed. The human-interface cluster is being implemented in the order
`user-prompts-dir` (done) -> `human-onboarding` (done); the human-interface cluster
(`user-prompts-dir`, `human-onboarding`, `gate-prompt-clarity`, `compaction-prep`) is
done; `deliberation-mode` is NEXT. The remaining not-started steps (see the Roadmap) implement,
into the pack, the workflow rules we have already ADOPTED and been operating by this
session. The full whole-codebase review is a LATER job, after these steps land, not
the current job.

IMPORTANT, apply these adopted rules when running the workflow even though the pack
`AGENTS.md`/prompts do not yet contain them (implementing them is the remaining
work):
- Always spawn a SEPARATE triager, never collapsed, independent of both the producer
  and the orchestrator (`triager-independence`, `Q-13`; now landed in the pack
  `AGENTS.md`/prompts, so this is no longer a pack-vs-practice gap).
- At every human-input point (escalation, intake, open questions), give options,
  trade-offs, a recommendation, and reasoning judged against the plan's numbered
  Project Principles (human-input contract, `Q-12` / `deliberation-mode`).
- Reviewers and triagers WRITE findings to per-agent files under
  `docs/plans/agent-scaffold.reviews/`; reference them by path, do not transcribe;
  COMMIT the review files before deleting them at round close (`Q-14`
  findings-files; commit-before-delete).
- Keep this ledger current and COMMITTED; commit it (and any pending work) before
  spawning a writer agent; commit any managed file before deleting it
  (`file-safety-rules`, `Q-17`; now landed in the pack `AGENTS.md`/prompts, along
  with the implementer fmt/checkout rule below and the orchestrator recovery
  protocol, so these are no longer pack-vs-practice gaps).
- Implementers must not run repo-wide `just fmt`/`nix fmt` or `git checkout` on files
  they do not own (two incidents clobbered the ledger this way; both recorded above).
  On an agent kill/interrupt, run the recovery protocol: inspect `git status`/`diff`,
  revert stray artifacts, confirm a known-good tree before continuing.
- After editing `pack/`, regenerate with `just scaffold-self` (build.rs now tracks
  pack rebuilds, so no `cargo clean` is needed); verify `just test` and `just clippy`;
  keep everything ASCII-clean; commit per step; do not push unless asked.
- Isolation (writer agents) is capability-tiered (container > worktree > the
  file-safety discipline); the RULE is now landed in the pack `AGENTS.md`/orchestrator
  prompt (`agent-isolation` complete). The MECHANISM (container/worktree integration)
  is still not built; it is deferred to `optional-modules`. Until then the rule
  resolves to the file-safety fallback in practice.

The workflow-hardening review task (this ledger's task) is still OPEN; do not delete
this ledger until the task closes.

New human request (intake): a reusable RESUME user prompt, the pickup counterpart to
`compaction-prep`'s flush. Recommend adopt. Rationale: harness-agnostic (cannot
assume a memory/auto-summary carries the resume), and it encodes "read the plan +
the ledger's RESUME STATE and CONTINUE the in-progress task, do not start fresh"
(the generic kickoff prompt frames a NEW task). Small: `compaction-prep` already
defines the resume PROCEDURE in `AGENTS.md`; this adds the human-facing trigger PROMPT
in `.agents/user-prompts/` (Q-16), paired with the kickoff and compaction-prep
prompts. Record as `Q-19`, fold into `compaction-prep` + `user-prompts-dir` at the
next planner touch. LOCKED by the human.
