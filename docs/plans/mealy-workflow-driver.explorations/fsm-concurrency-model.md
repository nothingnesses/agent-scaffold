# Q-51 exploration: the formal state-machine and concurrency model

Lens: the FORMAL STATE-MACHINE + CONCURRENCY MODEL for the proposed `agent-scaffold` workflow DRIVER. This document works out the state/input/output alphabets, the Mealy machines for the review/convergence loop and the phase sequencing, the concurrency model (a fleet of per-unit FSM instances plus a dependency scheduler over the plan's `blocked_by` DAG), the control-vs-judgment boundary, the escape hatches, and which parts of the workflow are genuinely FSM-shaped versus which resist it. It does not decide whether to build the driver (the skeptic lens owns that), nor the tool-vs-prose single-source question, the AGENTS.md generation, or the concrete I/O and isolation wiring (their own lenses). It supplies the MODEL those lenses build on.

## What already exists (the reconstruction the driver reuses)

The proposal's key enabler is that convergence state is ALREADY reconstructed deterministically from the durable artifacts. `src/workflow.rs` reads `<task>.plan.toml` (the Roadmap steps, their `blocked_by` DAG, their increments and risk classes, their waivers) and `docs/metrics/workflow.jsonl` (the `round`/`escalation`/`decision` events) and recomputes, for each increment, the consecutive-clean streak the outcome sequence implies (`round_log_consistency_problems`: a `clean` adds one, a `new_valid` resets to zero) and the peak streak against the class's required count (`w3_problems`: `low_risk` needs 1, `risky` needs 2). W4 checks decided-question receipts; W5 checks waiver integrity. `main.rs` exposes this as `validate --workflow`, and `status` already PROJECTS the plan and metrics into a summary.

The load-bearing observation for this lens: the checker computes the convergence RELATION over a completed history (was each `complete` step actually converged?). The driver computes the SAME relation forward from the LAST state to emit the next instruction (given where this loop is, what is the next legal move?). It is one arithmetic, evaluated in two directions. So the driver is not a new state model; it is the existing reconstruction plus a transition/output function on top. This is the strongest evidence (Principle 6) that the core loop is genuinely FSM-shaped: a deterministic checker for it already exists and ships.

## The review-loop Mealy machine (the core of the model)

A Mealy machine is (S, s0, Sigma_in, Sigma_out, T: S x Sigma_in -> S, G: S x Sigma_in -> Sigma_out): the output depends on state AND input (which is why Mealy, not Moore: the same state emits `spawn another round` or `converged` depending on the incoming triage outcome). The review loop from AGENTS.md's Convergence section is this machine, and it is the reusable heart of the whole workflow (it runs in plan review, in every step's work review, and, degenerately, in acceptance).

State of one review-loop instance:

- `phase` in {plan_review, work_review} (which artifact class the loop governs; acceptance/standalone review are the degenerate variant below).
- `risk_class` in {low_risk, risky}, FIXED when the loop opens and recorded (AGENTS.md: classify once at loop-open). It sets `required = 1` (low_risk) or `required = 2` (risky).
- `round` (nat): total rounds so far, against the cap (default 5).
- `consecutive_clean` (nat): the streak.
- `awaiting` in {round, recheck, human, none}: which input the machine is blocked on. This is the crucial field that makes the machine total and makes "which units are advanceable now" computable (see Concurrency).
- `status` in {running, converged, escalated, retired}.

Input alphabet Sigma_in (all are JUDGMENT inputs the tool CONSUMES, plus two control triggers):

- `open(risk_class)`: the loop opens on an artifact with its classification. (`risk_class` is a judgment; the tool may suggest a default but consumes the choice.)
- `round_result(outcome in {clean, new_valid}, top_dismissed_severity in {none, low, medium, high, critical})`: the triager's verdict for the round, reduced to the two facts the machine needs: did the round yield new valid findings, and what is the highest severity among DISMISSED findings (for the backstop gate).
- `recheck_result(upheld | overturned)`: the backstop second-triager outcome.
- `human_decision(resume | accept | send_back)`: the human's ruling at an escalation.
- `abandon`: the step is dropped or cancelled.

Output alphabet Sigma_out (the next INSTRUCTION the tool emits; the filled prompt text is a payload on these):

- `spawn_round`: emit the reviewer + separate-triager prompt for a fresh round on the current artifact, with the ledger and the diff range filled in.
- `require_recheck`: emit the independent-second-triager prompt for the high/critical dismissal; convergence blocks until it returns.
- `route_fix`: emit the planner/implementer prompt to address this round's valid findings, then continue.
- `converged`: the loop is done; the caller may move on (start implementing after a plan review, or mark the step complete after a work review).
- `escalate`: emit the human-input-contract prompt (options, trade-offs, recommendation, Principle-judged reasoning, ledger as evidence).
- `retire`: the loop closes (abandoned unmerged, or accepted at escalation).

Transition + output function (T and G together). Let `required = required(risk_class)`, `cap = 5`.

- From `status=running, awaiting=round`, on `round_result(new_valid, _)`: `round += 1`, `consecutive_clean = 0`. If `round >= cap` -> `awaiting=human`, output `escalate`. Else -> `awaiting=round`, output `route_fix` then `spawn_round`.
- From `status=running, awaiting=round`, on `round_result(clean, sev)` where `sev >= high`: `awaiting=recheck`, output `require_recheck`. The round outcome is PROVISIONAL; the streak does not advance until the re-check returns (AGENTS.md: the re-check is a step in the loop, convergence blocks on it).
- From `status=running, awaiting=round`, on `round_result(clean, sev)` where `sev < high`: `round += 1`, `consecutive_clean += 1`. If `consecutive_clean >= required` -> `status=converged`, output `converged` (the convergence check applies BEFORE the cap, per AGENTS.md's "if a round both reaches the cap and is the converging clean round, convergence first"). Else if `round >= cap` -> `awaiting=human`, output `escalate`. Else -> `awaiting=round`, output `spawn_round`.
- From `awaiting=recheck`, on `recheck_result(upheld)`: apply the clean branch just above (advance the streak, then the same convergence/cap test). On `recheck_result(overturned)`: flip the round to new_valid (`round += 1`, `consecutive_clean = 0`), then the same new_valid branch (route_fix + spawn_round, or escalate at the cap).
- From `awaiting=human`, on `human_decision(resume)`: RESET both counters (`round = 0`, `consecutive_clean = 0`, per AGENTS.md, so the cap does not immediately re-fire), `awaiting=round`, output `spawn_round`. On `human_decision(accept)`: `status=retired`, output `converged` (and the caller records a `accepted-at-escalation` waiver, which the schema already supports). On `human_decision(send_back)`: `status=retired`, output `route_fix`.
- From any `status=running`, on `abandon`: `status=retired`, output `retire`.

Every (state, input) pair has a defined image, so the machine is TOTAL. This matters for the escape hatches below and for Principle 5: an undefined transition is an illegal state, and a total function has none. Note the transition function is exactly the arithmetic in `w3_problems`/`round_log_consistency_problems` run forward: this machine is not new logic, it is the checker's relation with a next-move projection.

## Acceptance and standalone review: a degenerate variant

Acceptance (phase 5) and the standalone review entry mode are NOT the consecutive-clean loop: AGENTS.md says acceptance is a single reviewers-then-triager pass, no round loop and no cap, but it KEEPS the high/critical dismissal re-check. Model this as a degenerate two-state machine: `open -> single_pass`, on `round_result`: if a high/critical finding was dismissed, `awaiting=recheck` first; then on the settled verdict, either `met` (output `done`/`report`) or `shortfall` (output `route_fix` back to planning or implementation, which re-enters the task machine rather than looping in place). Reusing the review-loop's input alphabet and its re-check sub-state, with the round/streak fields pinned unused, keeps this one definition rather than a second machine (Principle 16).

## Phase sequencing: the task machine and the step machine (nested)

Two granularities sit above the review loop, and they NEST.

The TASK machine (one per plan): states {planning, plan_review, implementing, accepting, done, escalated}. `planning` (planner drafts) -> on plan drafted, open a plan_review review-loop instance -> on that instance's `converged`, enter `implementing` -> while any pending step remains, drive step machines (below) -> on no pending steps, enter `accepting` (the degenerate acceptance instance) -> on `met`, `done`; on `shortfall`, route back to `planning`/`implementing`.

The STEP machine (one per Roadmap step): states {not-started, next, in-progress, in-review, complete, skipped, abandoned}. These map directly onto the `StepStatus` enum the plan schema already defines (`not-started`, `in-progress`, `complete`, `skipped`, `next`, plus `optional`/`deferred` as not-yet-scheduled variants). `next/not-started -> in-progress` (spawn implementer) -> on commit, `in-review` which EMBEDS a work_review review-loop instance -> on that instance's `converged`, `complete`. `abandon` -> `abandoned` (worktree removed, branch deleted unmerged).

The nesting is the important structural claim: the step machine's `in-review` state CONTAINS a review-loop FSM instance, and the task machine's `plan_review` and `accepting` states each contain one too. The review-loop machine is defined ONCE and instantiated in three sites. This is a hierarchical FSM (a state machine whose states can contain sub-machines), not a flat one, and not a single global machine. It keeps one definition of convergence (Principle 16) and makes the illegal cross-states (a step `complete` while its work-review loop is still `running`) unrepresentable by construction (Principle 5): `complete` is reachable only through the embedded instance's `converged`.

## Human-input points as one uniform await-state

AGENTS.md deliberately routes every human decision through ONE human-input contract: escalation on an impasse, intake of a new/changed request, an open or clarifying question, and a directly-asked decision question all use the same format (options, trade-offs, recommendation, Principle-judged reasoning). Model this as a single input class `human_decision` and a single `awaiting=human` state that ANY machine in the fleet can enter. The tool's control job at such a state is uniform: detect the blocked unit, emit the contract-shaped prompt, and, on resume, require the durable receipt (which `validate --workflow`'s W4 already enforces for decided items). The tool CANNOT generate the options or the reasoning (that is judgment); it emits the prompt that asks the agent to prepare the contract and consumes the recorded `decision`. So the human-input points are FSM-shaped in their CONTROL (a blocking await plus a required receipt) and judgment-shaped in their CONTENT, which is exactly the boundary this model draws everywhere.

## Concurrency: a fleet of per-unit FSM instances plus a dependency scheduler

The orchestrator runs parallel steps, increments, and reviewers, so a single sequential machine is wrong: a single machine is in exactly one state, and parallelism needs a PRODUCT of states, which explodes combinatorially. The correct model is a FLEET: a SET of independently-tracked FSM instances, each with an id, its current state, and its `awaiting` field.

- The set: one task machine; one step machine per Roadmap step; one review-loop instance per active review loop (the plan review, each in-review step's work review, the acceptance pass). Instance ids reuse the plan's own identifiers: step slug, increment id (`Increment.id`), and a loop id derived from (phase, artifact).
- Recompute, do not persist. Each invocation re-derives EVERY instance's state from `plan.toml` + `workflow.jsonl` (+ `git` for the tree/commit facts), exactly as `workflow.rs` already re-derives convergence state. There is no separate FSM-state file (see the design space: a second store would be a second source of truth that drifts, against Principles 8 and 16). This is what makes the driver survive a crash or compaction (Principle 9) and keeps it stateless.
- Which units are advanceable now. A unit is READY iff (a) its machine's only outgoing transitions from its current state are CONTROL transitions the tool can take from state alone (then the tool advances it and emits the instruction), OR (b) it is `awaiting` a judgment input that has just been supplied (via a `record-*` subcommand). A unit is BLOCKED iff it is `awaiting` an input not yet available, or the scheduler says its predecessors are unfinished.
- The dependency scheduler over `blocked_by`. The plan's `Step.blocked_by: Vec<String>` is a typed DAG (its edges are validated non-self-referential and pointing at real steps by `validate_source`). A step is SCHEDULABLE (may enter `in-progress` now) iff every slug in its `blocked_by` is `complete` or `skipped`. The scheduler computes the READY FRONTIER: the antichain of steps that are `not-started`/`next` with all dependencies satisfied. That frontier is what may run in parallel now, capped by the orchestrator's concurrency/worktree budget (the tool proposes the frontier; the orchestrator, which owns isolation, decides how many to actually spawn). This is a topological-readiness / workqueue scheduler, not a further FSM.

So the tool's per-call output is: for each ready-frontier step not yet running, an instruction to start it; for each running loop that just received a judgment, its next instruction; for each unit `awaiting` an input, a note that it is blocked on that input. The fleet advances by data availability (judgments arriving) and by dependency satisfaction (predecessors completing), which is the dataflow shape the human's reframing describes.

Increment granularity: the schema models increments as a flat list under a step (`Increment` carries only `id` and `risk_class`, no `blocked_by`), so the DAG is at the STEP level only. The model keeps scheduling at step granularity; increment ordering within a step stays the orchestrator's sequential choice (see YAGNI).

## The control-vs-judgment boundary (where determinism ends)

The tool OWNS (computable from state, deterministic control transitions):

- Counting the streak from the outcome sequence and resetting it on `new_valid` (already in `workflow.rs`).
- Computing `required` from `risk_class`, detecting convergence (`consecutive_clean >= required`), detecting the cap (`round >= cap`) and emitting `escalate`, and applying the convergence-before-cap precedence.
- Gating convergence behind the backstop re-check when a dismissed finding is high/critical.
- Resetting both counters after a `resume` decision.
- Computing the ready frontier from `blocked_by` and step statuses.
- Selecting and filling the next instruction PROMPT TEMPLATE and attaching the periodic reminders (for example the Project Principles).

The tool CONSUMES but never PRODUCES (judgment inputs; determinism ends here):

- The triager's per-finding verdict, and thus the round `outcome` (clean vs new_valid). The tool cannot decide whether a finding is valid.
- The `risk_class` at loop-open (a judgment about blast radius; defaultable but not decidable by the tool).
- The backstop `recheck_result`.
- The human decision at any `awaiting=human` state, and the reasoning content of the human-input contract.
- The acceptance judgment (does the artifact meet the Success Criteria).
- The commit itself (the implementer's diff) and whether a change is "small and reviewable."
- The intake classification (trivial vs non-trivial) and the "match ceremony to stakes" call.

The interface between the two is the `record-*` subcommand set: each records a judgment AND advances the affected instance's state ATOMICALLY (append the log line and let the next recompute reflect it). Modeling the judgments as the machine's INPUT ALPHABET, never as computed transitions, is the whole discipline: it is Principle 5 (the tool cannot represent a state where it invented a verdict) and Principle 18 (the tool has only the authority to count and sequence, not to judge). Get this boundary wrong in the permissive direction and the tool hallucinates decisions; get it wrong in the restrictive direction and it straitjackets the agent. The boundary is drawn at exactly the line AGENTS.md already draws between the orchestrator (drives the loop) and the triager/human (judge).

## Escape hatches: overrides as recorded transitions

A machine that refuses non-standard moves is a cage; the design must accept exceptions WITHOUT leaving the fleet in an unrepresented or unaudited state. The mechanism: every escape is a FIRST-CLASS, RECORDED transition, so the machine stays the single source of truth for what happened even when a human forces a move the normal rules would not take.

- Override: a `record-override --unit <id> --to <state> --reason <text>` transition (a logged event) forces a unit to a target state and records that a human overrode the rules. The next recompute reflects the forced state; the log shows it was forced, by whom, and why. Because the transition function is total, the override lands in a defined state, not a hole.
- Abandon: `abandon` is already a first-class transition (to the `abandoned`/`skipped` terminal), matching AGENTS.md's "if a step is abandoned before it converges" and the `Skipped` status.
- Accept-at-escalation: already formalized in the schema as the `accepted-at-escalation` waiver reason, record-backed by the `escalation` it cites (W5 checks the join). This IS an escape hatch already expressed as a recorded transition; the model inherits it.
- The unanticipated: the total transition function means every input has a landing state, but for genuinely novel situations the master hatch is the ADVISORY adoption stance. In advisory mode the tool SUGGESTS the next instruction and the agent may deviate; a deviation is recorded (a `record-override` or a note), so the tool measures how often it is overridden. That measurement is the evidence (Principle 6) that decides whether to deepen toward authoritative driving, and it keeps the tool honest: a workflow the machine mis-models shows up as a high override rate rather than as silent friction.

The invariant across all four: an override is a TRANSITION IN THE MACHINE, not a bypass AROUND it. This keeps the durable log a complete account (Principle 9) and keeps the recompute total and correct.

## What is genuinely FSM-shaped, and what resists it

Genuinely FSM-shaped (deterministic, tool-ownable), and thus the model's proper scope:

- The convergence/review loop: streak counting, the cap, required-streak-from-risk, the backstop gate, the counter resets. PROVEN FSM-shaped because `workflow.rs` already reconstructs it.
- Phase sequencing (plan -> plan_review -> implement -> accept) and the per-step status lifecycle (already an enum).
- The dependency scheduler over `blocked_by` (topological readiness is a pure function of the DAG and the statuses).
- The CONTROL half of every human-input point (a blocking await plus a required receipt on resume).

Resists FSM modeling (judgment-shaped; must stay agent/human INPUTS, not states the tool computes):

- Whether a finding is valid (the core triage judgment).
- Risk classification (a blast-radius judgment; defaultable, not decidable).
- Whether a change is "small and reviewable," and the "match ceremony to stakes" collapse-or-keep-roles call (AGENTS.md deliberately leaves these to judgment).
- The CONTENT of a review (what to actually scrutinize) and the CONTENT of the human-input contract (the options and Principle-judged reasoning). The tool fills a template; it cannot author the substance.
- Intake of a NOVEL human request (what it touches, whether it is trivial).
- Design-space exploration itself (this document): open-ended, not a state sequence. The tool can only track the STATUS transition (`exploring -> open` once a synthesis is owed and produced), not the thinking.
- Semantic merge-conflict resolution (already routed to an implementer and re-reviewed; authored content, not control).

The conclusion this lens reaches: the workflow is a HYBRID, a deterministic control SKELETON (counting, sequencing, scheduling, gating, prompt emission) wrapping JUDGMENT HOLES that only agents and humans fill. The driver's correct scope is the skeleton, consuming judgment as typed inputs, which is precisely the "detection, not prevention / the tool consumes judgment, never makes it" philosophy `workflow.rs` and the human's reframing already state.

## The design space (how to formalize) and Principle-judged trade-offs

Two axes: the FLEET TOPOLOGY, and the STATE SOURCE.

Topology options:

- Option A: a single global monolithic FSM whose state is the entire workflow. Rejected. The state space is the product of every step's and every loop's state, which explodes combinatorially and is riddled with illegal combinations (Principle 5 violated: a single product type admits states that cannot occur). Worse, a single machine is in exactly one state, so concurrency (parallel steps) is UNREPRESENTABLE without the product. Fails Principle 1 (not the cleaner architecture) and Principle 5.
- Option B: a flat fleet of independent per-unit FSMs, coordinated only by the scheduler reading `blocked_by`. Viable and simple; matches the reframing; each instance is small and analyzable (Principle 5). Weakness: the relationship between a step machine and the review-loop it contains is IMPLICIT (coordination via shared recomputed state), so nothing structurally prevents a `complete` step whose loop never converged; that invariant would live only in a check, not in the type. Partial Principle 5.
- Option C: a HIERARCHICAL fleet (recommended). A task machine embeds step machines; each step's `in-review` and the task's `plan_review`/`accepting` embed a SINGLE reusable review-loop sub-machine; the scheduler over `blocked_by` picks the ready frontier of step machines. Strengths: one definition of the convergence loop instantiated three places (Principle 16, one source of truth; Principle 8, structured-first); the nesting makes `complete`-without-convergence unrepresentable by construction (Principle 5); the scheduler is a clean separate concern (Principle 1). Cost: marginally more modeling machinery than B (the nesting relation). Best against Principles 1, 5, 16, 8.
- Option D: a full Harel statechart with orthogonal regions (for concurrency) and history states (for resume). Expressive enough to hold everything in one formalism, but heavyweight: orthogonal-region signalling and history semantics are more than the workflow needs and awkward in plain Rust, and history states RETAIN machine state, which fights the stateless-recompute requirement that gives crash/compaction survival (Principle 9). The nested fleet (C) plus stateless recompute gets the same expressive power with less machinery. Rejected as over-general (Principle 2, minimal by default; YAGNI).

State-source options:

- Stateless recompute from the durable files each call (recommended). Re-derives every instance's state from `plan.toml` + `workflow.jsonl` + `git`, exactly as `workflow.rs` already does. Survives crash and compaction (Principle 9), keeps one source of truth (Principle 16, Principle 8), and reuses the shipped reconstruction (Principle 6, evidence: it already works). Cost: recompute per call, which is negligible at plan scale.
- A persisted `.fsm-state` file (rejected). Faster, but it is a SECOND source of truth that can drift from the plan and the log, against Principles 8 and 16, and it would need its own reconciliation on resume, adding exactly the drift surface the whole initiative sets out to remove.

## Recommendation

Model the driver as a HIERARCHICAL FLEET of per-unit Mealy machines, recomputed statelessly from the durable artifacts:

- A single reusable REVIEW-LOOP Mealy machine with the state/input/output alphabets and total transition function specified above, whose transition function IS the convergence arithmetic `workflow.rs` already computes, run forward. Instantiate it in three sites: plan review, each step's work review, and (a degenerate single-pass variant) acceptance and standalone review.
- One TASK machine and one STEP machine per Roadmap step (the step statuses reuse the existing `StepStatus` enum), with the review-loop instances NESTED inside their review states, so illegal cross-states (`complete` without convergence) are unrepresentable.
- A DEPENDENCY SCHEDULER that computes the ready frontier over the plan's `blocked_by` DAG (a step is schedulable iff all its `blocked_by` are `complete`/`skipped`); the ready antichain is what may run in parallel now, capped by the orchestrator's isolation budget. The tool proposes the frontier; the orchestrator owns how many to spawn (isolation is orthogonal).
- A hard CONTROL/JUDGMENT interface: the tool owns counting, sequencing, scheduling, gating, and prompt emission; it consumes triage outcome, risk class, recheck result, human decision, and commit as typed INPUTS via `record-*` subcommands that append the log and advance state atomically. The tool never manufactures a verdict.
- ESCAPE HATCHES as recorded transitions: override, abandon, and accept-at-escalation are first-class logged transitions into the total transition function's defined states, so a forced move never leaves the fleet unrepresented or unaudited. Advisory-first adoption is the master hatch and the evidence source (Principle 6): the tool suggests, the agent may deviate, deviations are recorded and measured, and the override rate decides whether to deepen toward authoritative driving.

Reasoning against the Principles: Option C + stateless recompute is the cleanest long-term architecture (Principle 1), keeps one definition of convergence and one source of state (Principles 16, 8), makes the dangerous illegal states unrepresentable (Principle 5), survives context loss (Principle 9), gives the tool least authority (Principle 18, it counts and sequences but never judges), and reuses a reconstruction that already ships and passes tests (Principle 6). It stays minimal against the over-general statechart (Principle 2).

## What NOT to build (the YAGNI boundary for this lens)

- Do NOT build a single global product FSM (Option A) or a full Harel statechart with history states (Option D). The nested fleet with stateless recompute has the needed power with less machinery.
- Do NOT persist FSM state in a separate file. Recompute from `plan.toml` + `workflow.jsonl` + `git` every call; a second store is a second source of truth that reintroduces drift.
- Do NOT let the tool make judgments: no auto-triage, no auto risk-classification beyond suggesting a default, no auto "small and reviewable," no auto ceremony-collapse, no auto acceptance. Determinism ends at the verdict; the tool consumes verdicts, it does not compute them.
- Do NOT model the open-ended entry modes (design exploration, novel-request intake, the substance of a review or a human-input contract) as FSM states. Model only their CONTROL skeleton: the status transition (`exploring -> open`), the blocking await, and the required receipt. The thinking is an input, not a state.
- Do NOT model increment-level DAG scheduling. Increments carry no `blocked_by` (the DAG is at the step level); keep the scheduler at step granularity and leave increment ordering to the orchestrator until evidence shows finer scheduling is needed.
- Do NOT have the driver author or reconcile merges, run isolation, or hold any state that is not a projection of the files. Isolation is orthogonal: the tool instructs, the orchestrator isolates (that wiring is the I/O-contract lens's concern, not this model's).
- Do NOT ship authoritative driving before advisory-mode evidence shows drift actually drops. This lens specifies the MODEL; the model must be validated advisory-first (Principle 6), mirroring the receipt and waiver pilots. Whether the driver is worth building at all, and whether it subsumes the queued drift-mitigation work, are the skeptic lens's questions, not this one's.
