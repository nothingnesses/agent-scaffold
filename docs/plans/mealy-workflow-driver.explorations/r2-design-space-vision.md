# Q-51 round 2 exploration: design-space breadth, fullest vision, and staging

## The lens

The human has set the FULL DRIVER as the target and asked, before any build, to brainstorm the design space and what its scope COULD encompass at its most ambitious, then structure that into a staged path. This note is the breadth/vision/staging lens. It maps the widest defensible vision of the driver, places every capability on an ambition spectrum, and lays a staged roadmap from the advisory MVP to the full driver with a per-stage EVIDENCE GATE, honouring the skeptic's central objection (do not pour concrete around a workflow that is still changing) by keeping the workflow's control LOGIC data-driven: a spec the tool interprets, not hardcoded Rust. Round 1 already settled the formal FSM/concurrency model (`fsm-concurrency-model.md`), the tool-vs-prose single-source boundary and generation (`tool-vs-prose-boundary.md`), the I/O contract and adoption path (`io-contract-and-adoption.md`), and the YAGNI weighting (`yagni-skeptic.md`), synthesised in `synthesis.md`. This note builds on those and does not re-derive them; it answers a different question: given that we ARE heading for the full driver, what is the whole thing, in what order, and where does it connect to the rest of the system.

Two principle sets are cited below, matching how round 1 cited them. The numbered PROJECT Principles are the eight `[[principle]]` entries in `agent-scaffold.plan.toml` (P1 cleaner-architecture, P2 minimal, P3 safe-on-existing, P4 idempotent, P5 illegal-states-unrepresentable, P6 evidence-first, P7 reproducible, P8 structured-data-first). The generic WORKFLOW Principles are the 1-22 list in `AGENTS.md` (notably W9 durable-notes, W16 one-source-of-truth, W18 least-authority, W19 document-the-why, W20 self-contained-docs). Where a name is unambiguous I use it.

## The north star, stated precisely

The driver is the EXECUTABLE SINGLE SOURCE of the workflow's control logic. Every deterministic control rule (the convergence counts and cap, the phase sequence and stop predicate, the `blocked_by` readiness relation, the isolation-tier resolution, the file-path conventions, the entry-mode routing) lives ONCE, as DATA in a machine-readable workflow spec that the tool interprets. From that one spec three things are derived and cannot disagree: the tool EXECUTES it (emits the next instruction), `validate --workflow` CHECKS against it (the W3/W4/W5 invariants), and the `render` closure PROJECTS its control fragments into the `AGENTS.md` workflow section as generated prose (guarded by `render --check`, exactly as `<task>.md` is guarded today). Process prose and process logic therefore cannot drift, because the prose statement of every control constant is generated from the same spec the tool runs. This is Project Principle 8 (structured data first, project for humans) applied to the PROCESS itself, the natural third stage of the arc the ask names: structured SOURCE (`plan.toml`) -> structured ENFORCEMENT (`validate --workflow`) -> structured DRIVING (this).

The one unifying architectural claim that makes the fullest vision coherent, and keeps it from being "a big new subsystem": the append-only round log (`docs/metrics/workflow.jsonl`) is already an EVENT STORE, and the driver's state is a FOLD over that event stream. The driver is not a new state model bolted on; it is one more PROJECTION in a family the project is already building. `validate --workflow` is the BACKWARD projection (was each completed step actually converged?). `status` is the SUMMARY projection. `next`/the driver is the FORWARD projection (given the last state, what is the next legal move?). `workflow-viz` is the TIMELINE projection. `workflow-calibration` is the AGGREGATE projection (tune the constants from the outcomes). Every one reads the SAME event log plus the SAME spec, so none of them can drift from the others; they share `src/plan.rs` and `src/metrics.rs` parse paths already. Seen this way, the "Mealy driver" is the forward member of a projection family whose backward and summary members already ship. That is the strongest evidence (P6) that the vision is buildable incrementally rather than as one monolith: the reconstruction arithmetic exists (`src/workflow.rs`), and the driver runs it forward.

## The mapped design space (what the full driver could encompass)

Everything the driver could absorb, grouped by the system it connects to. Each item notes what it reads or emits and which existing/planned piece it reuses, so the map doubles as an integration diagram.

### Core control (the irreducible driver)

- The review-loop Mealy machine: streak counting, required-streak-from-risk, the cap, the convergence-before-cap precedence, the backstop re-check gate, the counter resets (round 1's `fsm-concurrency-model.md` specifies it; its transition function IS the `w3_problems`/`round_log_consistency_problems` arithmetic run forward). Instantiated in three sites (plan review, each step's work review, and a degenerate single-pass acceptance/`review`-mode variant).
- The task machine and step machine (nested), the step statuses reusing the shipped `StepStatus` enum, so `complete`-without-convergence is unrepresentable by construction (P5).
- The `blocked_by` dependency scheduler computing the ready frontier (the antichain of `not-started`/`next` steps whose dependencies are all `complete`/`skipped`), which is what may run in parallel now.
- Emission: per-unit state summary + valid transitions + the filled next-instruction prompt (role prompt path + step slug + artifact + diff range + findings/ledger paths + principle reminders) + resolved isolation tier, as JSON and human text.

### The spec-and-generation closure (the single-source half)

- A machine-readable workflow spec (`workflow.toml`, structurally parallel to `<task>.plan.toml`) holding ONLY the deterministic control data, with hand-authored prose sidecars for rationale/roles/contracts/disciplines, per `tool-vs-prose-boundary.md`'s Option C.
- The `render` closure projecting the spec's control fragments into the generated `AGENTS.md` workflow section (as `vocabulary_section()`/`status_line()` already project code constants), guarded by `render --check`. This closes the EXISTING `required_streak`-vs-prose duplication (`src/metrics.rs` returns 1/2 while `AGENTS.md` restates it in prose) and prevents the new code-vs-prose drift by construction.

### Event-sourced state and the write path

- The `record-*` subcommand set (`record-round --outcome clean`, `record-decision`, `record-recheck`, ...) that appends a JSONL event AND advances the affected unit's state atomically, refusing an illegal transition at write time (P5/W13). This is the write half of event sourcing: the log is the event store, the record commands are the only validated appends, and every projection folds the same stream. It reopens the Q-24 "no write-path runtime dependency" stance, so it is gated (see the roadmap).
- New event kinds the store could carry for the fuller vision: agent dispatch/return events (which role ran at which unit, start/end), which `state-schema`/`workflow-viz` already earmarked as a deferred third stream. These are what turns the log from a round record into a full activity event store.

### Driving the optional modules

- The driver knows which modules are enabled (from the pack/config) and emits the module-specific instruction at the right phase: spawn the `checks-reviewer` in the work-review phase (checks module), insert the `test-author` red phase before implementation and hand the checks-reviewer the red/green profile parameter (`test-driven`), fire the diff-scoped mutation gate once after green convergence on risky steps (`mutation`), and echo the resolved container/worktree tier (isolation module). The module guidance already concatenates into the `{{modules}}` render slot; the driver becomes the thing that SEQUENCES those modules' gates rather than the agent remembering to. This makes the driver the sequencer of the whole verification stack (checks -> test-driven -> mutation), each layer verifying the one below.

### Visualisation as a read of the driver

- `workflow-viz` (the deferred live visualiser: a Gantt of dispatch/return, and a `nom`-style streaming in-flight tree) becomes a READ of the driver's per-unit fleet state and the dispatch/return event stream, not a separate subsystem. The driver already computes, per call, every unit's state and `awaiting` field; viz renders that fold plus the timeline. This is why viz was sequenced after the shared schema: it is a projection, and the driver is the projection engine.

### The drift-mitigation neighbours the driver surfaces at the right moment

- The re-grounding + provenance step (CONTEXT drift): the driver emits the re-grounding brief at task entry and at resume-after-compaction, because those are the states where context is stale. The driver does not subsume it (it tells you the move, not the meaning) but it is the natural thing to TRIGGER it at the right state.
- Q-50 doc-currency (DOC-TRUTH drift): the driver surfaces a doc-currency cue at the checkpoint where docs should be reconciled, and DEPENDS on Q-50 (a driver that emits instruction prompts assembled from role-prompt files is only as correct as those files, so doc-currency is a prerequisite). Both stay un-gated and ship on their own schedule; the driver integrates them as timed emissions, it does not replace them.
- The decision-receipt (`Q-44`) and waiver machinery: the driver emits the human-input contract prompt at any `awaiting=human` state and requires the durable receipt on resume (W4 already enforces it), so the receipt pilot and the driver's escape hatches (accept-at-escalation) are the same mechanism.

### The calibration closure (the driver both emits and consumes)

- `workflow-calibration` becomes a CLOSED LOOP with the driver at both ends: the driver (via `record-*`) EMITS the round log with per-finding identity and reviewer/harness attribution (the schema `reviewer-harness-field` and the calibration methodology already earmark), and it CONSUMES the same log to tune the spec's constants (the required-streak-per-risk map, the cap, the reviewer-count-per-risk guideline, a possible severity-trajectory convergence rule). A calibrated constant flows back into `workflow.toml` -> render -> the `AGENTS.md` prose statement, so the whole thing is a self-tuning control plane whose tuning is auditable and single-sourced. This is the most ambitious point: the driver is not just executing the workflow, it is measuring and adjusting the workflow's own constants from real use, with every adjustment landing in the one spec.

### Multi-project / cross-repo driving

- One driver instance per repo is the default (it reads that repo's `plan.toml` + log). The ambitious extension is a driver that tracks SEVERAL plans (a portfolio view: what is the ready frontier across N projects, which needs a human decision). This is the outer edge of the vision and stays speculative until a second real project is driven.

### The interaction shape (REPL vs one-shot)

- The ask frames it as a "Mealy-machine REPL." Two realizations: (a) a persistent interactive process holding FSM state in memory across a session, or (b) repeated one-shot `next` calls, each recomputing state statelessly from the durable files. The vision KEEPS the REPL user-EXPERIENCE (run it, be told the next move, do it, run it again) but realizes it as (b): stateless one-shot calls. A persistent process is rejected because in-memory state is a second source of truth that a crash or compaction loses, defeating the whole point (W9, W16, and the `io-contract` lens's Option C rejection). "REPL" describes the loop the human runs, not a resident daemon.

## The ambition spectrum

Two orthogonal axes organise the whole design space. Placing a capability is a matter of picking a point on each.

AXIS A, AUTHORITY (how binding the tool's output is):

- A0 Observe: post-hoc detection only. `status` + `validate --workflow` (SHIPPED). The tool reports what happened.
- A1 Advise: `next` prints the next instruction; the agent is free to deviate; a deviation is just not-following. Read-only.
- A2 Record: `record-*` appends the validated event and advances state atomically; the tool now mediates the WRITE path and refuses illegal transitions, but still does not compel the next action.
- A3 Author: the tool emits the fully filled instruction prompt (templating) and generates the `AGENTS.md` control fragments from the spec; the tool is now the source of both the instruction and its prose statement.
- A4 Block / drive authoritatively: agents run the tool to be TOLD what to do and cannot proceed on an illegal move; overrides are first-class RECORDED transitions (`record-override --to <state> --reason`), never bypasses.

AXIS B, SINGLE-SOURCE SCOPE (how much of the control logic is one data-driven source):

- B0 Duplicated: constants live in both `src/metrics.rs` and prose (TODAY's state; a live drift gap).
- B1 Constants single-sourced: the required-streak map and the cap live once in the spec; `validate` and `next` both read it; render projects the constant fragment into prose.
- B2 Full control spec: the phase graph, path templates, tier order, and entry-mode routing all become spec DATA the tool interprets; render projects each as a generated fragment.
- B3 Generated prose closure: the whole control-fragment set of the `AGENTS.md` workflow section is render-generated and `render --check`-guarded; only rationale/role/contract prose stays hand-authored in sidecars.
- B4 Calibration closure: the spec's constants are TUNED from the event log the driver emits, so the single source is not just authored once but continuously, auditably adjusted from real use.

Placing the adjacent capabilities on the grid: the advisory MVP is (A1, B1). Event-recording is (A2, B1/B2). Instruction-authoring + generation is (A3, B3). The fleet + scheduler + module-driving is an A3 capability that adds BREADTH (more unit types driven) rather than more authority. Authoritative/blocking driving is (A4, B3). The calibration closure, viz-as-a-read, and multi-repo are (A4, B4), the far corner. The skeptic's floor (do-nothing-new) is (A0, B0). The full driver the human targets is the (A4, B4) corner, and the value of the two-axis map is that it shows the corner is reached by a PATH, not a leap: each stage below advances one or two cells, each independently useful, each gated.

## The staged roadmap: MVP -> full driver, with per-stage evidence gates

The organising discipline, honouring the skeptic: the workflow's control logic is DATA (a spec the tool interprets), so a workflow change is a data edit and a re-render, NOT a Rust edit and a review cycle. Adding a new constant VALUE, a new phase, a new path template is a spec edit. Only adding a genuinely NEW KIND of rule (a new transition type the interpreter does not have) is a code change. This is what lets the workflow keep evolving while it is encoded, and it is the concrete form of "do not pour concrete around a moving design": the concrete is the small fixed INTERPRETER, and the road surface (the rules) stays editable data on top of it.

### Stage 0 (SHIPPED): Observe

`status` (projection) + `validate --workflow` (W3/W4/W5 reconstruction). The baseline. It already reconstructs control state and detects every illegal state post-hoc.

- Value: catches control-drift after the fact; the reconstruction the driver reuses.
- This stage is the honest floor the skeptic names: much of the anti-drift value is already here.

### Stage 1 (the MVP, BUILD FIRST): Advisory `next` + single-source the convergence constants -> (A1, B1)

A read-only, stateless `agent-scaffold next` that recomputes per-unit control state from `plan.toml` + `workflow.jsonl` + the RESUME STATE ledger extract (+ git), reusing the W3/W4/W5 reconstruction, and prints the current per-unit state, the valid transitions, and the next-action as a filled-in instruction reminder, in JSON and human text. Sliced with it: lift the convergence CONSTANTS (the required-streak-per-risk map and the cap) into a single source both `validate --workflow` and `next` read, and project that one fragment into the `AGENTS.md` control section via render.

- Adds: the FORWARD projection (the point-of-action reminder that most directly fixes "the agent forgot a rule that applies right here"), and closes the existing `required_streak`-vs-prose duplication.
- Reuses: `src/plan.rs`, `src/metrics.rs`, `src/workflow.rs` transition logic, the `render` closure, the `status` projection. This is largely the already-planned `state-queries` step (Q-28/Q-34) plus a next-action reminder and the constants fragment.
- Risk: LOW. Read-only, additive, no write path, no Q-24 reopen, no logic migration beyond the constants. Small and reviewable (P4).
- Depends on: `round-log-core` (done), `workflow-invariants`, `render`.
- EVIDENCE GATE to Stage 2: (1) agents actually RUN `next` at decision points (adoption rate high; if they ignore it, no deeper tier helps until that is fixed). (2) A control-drift BASELINE is captured (required rounds skipped, missing receipts, wrong clean-counts, caught only by `validate`) and shows drift PERSISTS after `next` is in routine use, i.e. naming the next action was not enough. Absent persistent drift, stop here: the MVP may simply be sufficient, and that is a real and acceptable outcome.

### Stage 2: Event-recording write path + widen the spec -> (A2, B2)

Add `record-round` and siblings that atomically append the validated JSONL event and advance state, refusing an illegal transition at write time (P5/W13). Widen the workflow spec from just the constants to the full control DATA: the phase/transition graph, the file-path templates, the isolation-tier preference order, the entry-mode routing, each interpreted by the tool and each projected into `AGENTS.md` via a generated fragment guarded by `render --check`.

- Adds: structural enforcement that hand-writing cannot give (illegal transitions unrepresentable at the boundary, not just detected after), and the B2/B3 generation closure on the widened control set.
- Risk: MEDIUM. Reopens the Q-24 "no write-path runtime dependency" trade (the binary becomes load-bearing during active runs; the hand-write fallback must stay possible). The generation must be proven readable on the constants fragment first.
- EVIDENCE GATE to Stage 3: the Stage-1 gate met (drift persists, adoption high) AND the constants-fragment generation is demonstrated to produce correct, readable prose (so W16/P8 is actually satisfied, not nominally) AND agents run `next` consistently enough that a tool-mediated write path is not a friction cliff.

### Stage 3: The fleet + scheduler + instruction authoring + module driving -> (A3, breadth)

The per-unit FSM fleet (task/step/review-loop instances, recomputed statelessly), the `blocked_by` scheduler emitting the ready frontier, the FILLED instruction-prompt emission (templated from `.agents/prompts/`), and DRIVING the optional modules (emit the checks-reviewer spawn, the test-author red phase and red/green profile, the mutation gate, the resolved isolation tier at the right states).

- Adds: parallel-aware driving, the concrete next-prompt (not just the next action named), and sequencing of the whole verification stack (checks -> test-driven -> mutation) so the agent does not have to remember which module gate fires when.
- Risk: MEDIUM-HIGH. The fleet + scheduler is parallelism machinery; it earns its keep only at real concurrency. The prompt-templating adds a coupling to the prompt files that must track their edits (a fresh drift surface unless generated).
- EVIDENCE GATE to Stage 4: real PARALLELISM is actually happening (multiple concurrent units run often enough that a scheduler and per-unit fleet are exercised, not hypothetical) AND the workflow spec has gone a meaningful stretch (multiple tasks / weeks) with no STRUCTURAL change (a new rule KIND, not a constant value), so encoding the fuller machine is not concreting a moving design.

### Stage 4: Authoritative / blocking driving + overrides as recorded transitions -> (A4, B3)

The tool refuses to advance on an illegal move; documentation instructs agents to run it to RECEIVE instructions rather than to check themselves; every escape (override, abandon, accept-at-escalation) is a first-class RECORDED transition into the total transition function's defined states, so a forced move is never an unaudited bypass.

- Adds: the control flow genuinely LIVES in the deterministic program rather than agent memory (the ask's headline goal), with a complete audit trail.
- Risk: HIGH. Requires reliable binary availability across harnesses and an onboarding change ("never proceed without consulting the tool"). The straitjacket failure mode (a judgment-saturated workflow forced through a rigid path) is the danger; the override rate is the honesty check.
- EVIDENCE GATE to Stage 5: measured drift STILL persists after advisory + record + authoring (so blocking is warranted), the OVERRIDE RATE under authoritative driving is low (the machine models reality, is not a cage), and binary availability is guaranteed in the harnesses in use.

### Stage 5: The calibration closure + viz-as-a-read + multi-repo -> (A4, B4)

The driver consumes the round log it emits to TUNE the spec constants (`workflow-calibration`: required-streak, cap, reviewer-count-per-risk, a possible severity-trajectory convergence rule), each tuned constant landing back in `workflow.toml` -> render -> prose. `workflow-viz` renders the fleet state and the dispatch/return timeline as a projection. Optionally, multi-plan portfolio driving.

- Adds: a self-tuning, auditable control plane; the live visualiser as a free read; the outer portfolio edge.
- Risk: HIGH and DATA-BOUND. Calibration needs enough round volume to be statistically meaningful (the methodology notes tiny-n and correlated-LLM-blind-spot caveats); viz needs the dispatch/return event stream built; multi-repo needs a second real project.
- EVIDENCE GATE (to consider each sub-capability): calibration only when the log has enough records for a constant's estimate to carry a usable confidence interval; viz only when the event stream has a consumer and the fleet state is stable; multi-repo only when a second project is actually being driven.

## Trade-offs judged against the Principles

The vision is defensible only if each stage earns its principle case; the far corner is defensible only conditionally. Judged honestly:

- P6 (evidence-first) and W3/W6 (ground in evidence, verify): the WHOLE staging exists to serve P6. Each gate demands measured evidence (adoption, persistent drift, generation-readable, real parallelism, low override rate, enough data) before the next, irreversible-ish stage. Building the (A4, B4) corner before Stage 1 has produced its measurement inverts P6; building Stage 1 first honours it. This is the decisive principle and it points at the MVP-first path unambiguously.
- P8 (structured data first, project for humans) and W16 (one source of truth): the north star IS P8 applied to the process. The spec is the single source; validate, next, render, viz, and calibration are projections. But W16 is DOUBLE-EDGED before the generation closure exists: a spec plus a hand-authored prose section is TWO sources until render generates the prose and `render --check` guards it. So P8/W16 SUPPORT the driver only once B3 (generated prose) is proven, which is a reason to sequence (constants fragment first, at Stage 1), not to leap.
- P1 (cleaner long-term architecture) and W17: the projection-family framing (one event store + one spec, many folds) is the cleaner architecture, and it is why the driver is not a bolt-on. But "long-term cleaner" assumes a settled target; while the workflow changes weekly, the data-driven spec (a small fixed interpreter over editable rule DATA) is what keeps the clean architecture from becoming premature concrete. P1 is served by the data-driven discipline, not by hardcoding the rules in Rust.
- P5 (illegal states unrepresentable) and W13: the nested fleet makes `complete`-without-convergence unrepresentable by construction, and the `record-*` write path rejects illegal transitions at the boundary (Stage 2). This is the driver's best principled argument. The honest discount: `validate --workflow` already DETECTS every illegal control state post-hoc, so the marginal gain is "reject earlier" not "reject at all." Real but not transformative.
- P2 (minimal by default): the MVP is minimal (one read-only subcommand, existing parsers, no new required files); the far corner is maximal. P2 is the tension that the gates manage: each stage must prove its keep before the next adds surface. The fleet/scheduler and calibration are the items most at risk of violating P2 (machinery for a scale the solo dogfood may not reach), which is why they sit behind the parallelism and data-volume gates.
- W18 (least authority): the whole design gives the tool authority to COUNT and SEQUENCE but never to JUDGE. Every stage consumes triage verdicts, risk classes, recheck results, human decisions, and commits as typed INPUTS; none manufactures a verdict. Getting this boundary wrong in the permissive direction (the tool inventing judgments) is the primary failure mode, and advisory-first (A1) keeps the human in the loop while the boundary is validated.
- W9 (durable notes / survive context loss): stateless recompute from the durable files, at every stage, is what makes the driver survive a crash or compaction. This forbids the persistent-REPL and the `.fsm-state` file at every point on the roadmap, not just the MVP.
- W19/W20 (document-the-why / self-contained docs): the generation boundary is drawn at the CONTROL fragments only; the rationale, roles, human-input contract, and file-safety disciplines stay hand-authored prose sidecars. An FSM cannot generate "why one clean round is weak evidence"; forcing it to would strip the why (W19) and the self-containment (W20). This is the boundary that keeps B3 honest.

Net: the principles that are about EVIDENCE, MINIMALISM, and STABILITY (P6, P2, W3, W10) all point at building Stage 1 and gating hard. The principles that are about CLEAN STRUCTURE and ONE SOURCE (P1, P5, P8, W13, W16) point at the full north star but each carries a precondition (once the generation closure is proven, once the workflow stabilises, once real parallelism exists) that the gates encode. The vision is principle-aligned as a DESTINATION reached by a gated path; it is principle-VIOLATING as a thing built all at once now.

## Recommended north star and first stages

- NORTH STAR (adopt as the target architecture): the driver as the forward projection of a projection family over one append-only event store (`workflow.jsonl`) and one data-driven control spec (`workflow.toml` + prose sidecars). The spec is the single source; `validate` checks it, `next`/the driver executes it, `render` generates the `AGENTS.md` control fragments from it, and (later) `viz` and `calibration` read/tune it. Control logic is DATA the tool interprets, so the workflow evolves by editing data and re-rendering, not by editing Rust. Prose and logic cannot drift because the prose is generated from the spec and `render --check`-guarded.
- FIRST STAGE (build now): Stage 1, the advisory `next` at (A1, B1), sliced with single-sourcing the convergence constants. It is on the critical path to the full driver, it is independently valuable even if the driver never deepens, it is barely more than the already-planned `state-queries` step, it closes a real existing duplication, and it is the instrument that PRODUCES the evidence every later gate needs. Un-gate the re-grounding step and Q-50 in parallel (the driver complements them and depends on Q-50; do not make them wait).
- SECOND STAGE (only on evidence): Stage 2 (event-recording + widen the spec + prove the generation closure), gated on the Stage-1 measurement (persistent drift, high adoption, readable generation). Everything beyond Stage 2 (the fleet/scheduler, authoritative blocking, calibration closure, viz, multi-repo) stays behind its named gate. The human targets the (A4, B4) corner; the recommendation is to COMMIT to that destination in the plan while building only Stage 1 now and letting each gate authorise the next stage. That reconciles the human's full-driver direction with the skeptic's objection: the target is the full driver, the concrete poured today is only the small interpreter and one read command.

## The YAGNI / speculative boundary (tempting, but OUT)

Out for the foreseeable future (some possibly permanently), because they are speculative, judgment-encoding, or premature against the gates above:

- A single global product FSM or a full Harel statechart with history states. The nested fleet with stateless recompute has the needed power with less machinery, and history states retain state that fights crash/compaction survival (W9). Rejected by `fsm-concurrency-model.md`.
- A persisted `.fsm-state` file or any persistent REPL daemon holding in-memory FSM state. A second source of truth that drifts and does not survive compaction (W9, W16). The "REPL" is realized as repeated one-shot stateless `next` calls.
- The tool making any JUDGMENT: auto-triage, auto risk-classification (beyond suggesting a default), auto "small and reviewable," auto ceremony-collapse, auto-acceptance. Determinism ends at the verdict; the tool consumes verdicts, never computes them (W18). This boundary is permanent.
- A general or Turing-complete workflow DSL that can express arbitrary processes. The spec encodes THIS workflow's fixed control-rule kinds (streak counts, cap, phase graph, path templates, tier order, entry-mode routing), not a process language.
- Generating the WHOLE `AGENTS.md` workflow section. Generate only the control-constant fragments; the rationale, roles, human-input contract, and file-safety disciplines stay hand-authored sidecars. LLM-synthesizing English from a state table is a different, harder, unproven artifact; render only splices verbatim prose and templates constants, it never synthesizes descriptions of logic.
- Increment-level DAG scheduling. Increments carry no `blocked_by`; keep the scheduler at step granularity until evidence shows finer scheduling is needed.
- Per-turn polling of `next` during active work. The token trade is negative there (Q-28/Q-34); scope to cold resume, CI/hook, and the orchestrator's decision-point checkpoints.
- The far-corner integrations before their gates: the calibration closure before the log has statistically usable volume; `workflow-viz` before the dispatch/return event stream is built and has a consumer; multi-repo/portfolio driving before a second real project is actually driven; driving the optional modules before those modules themselves ship and are in real use.
- Authoritative/blocking driving before advisory + record measurement shows drift persists AND the override rate is low. Shipping the cage before proving the machine models reality is the worst-regret move; it adds ceremony to reach the same override.

The burden of proof runs one direction at every step: the next stage must show, with data from the stage below, that the cheaper tier left drift on the table. Until a gate's evidence is in hand, the stage below it is the right and sufficient stopping point, and that is true even though the agreed DESTINATION is the full driver.
