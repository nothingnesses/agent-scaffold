# Q-51 round-2 exploration: requirements and scope for the FULL driver

Lens: REQUIREMENTS + SCOPE. Round 1 settled the framing (build advisory-first, defer the full driver behind an evidence gate) and produced the FSM/concurrency model, the tool-vs-prose boundary, the I/O contract, and the skeptic weighing (`fsm-concurrency-model.md`, `tool-vs-prose-boundary.md`, `io-contract-and-adoption.md`, `yagni-skeptic.md`, synthesised in `synthesis.md`). This round-2 pass is different: the human has CHOSEN the full driver as the TARGET (Option B) and wants a deeper design pass on requirements, scope, and user flows before committing. So this note does NOT re-argue whether to build the full driver; it catalogs, rigorously, WHAT the full driver must do (functional), the qualities it must have (non-functional), and where the in-scope / out-of-scope lines fall, honouring the two hard constraints the human carried into this pass: the workflow LOGIC stays DATA-DRIVEN (a spec the tool interprets, not hardcoded Rust), and the tool owns CONTROL, never JUDGMENT.

Numbered principles below are the plan's own Project Principles from `agent-scaffold.plan.toml` (`[[principle]]` n=1..8): P1 cleaner long-term architecture, P2 minimal by default, P3 safe on existing projects, P4 idempotent, P5 illegal states unrepresentable, P6 ground decisions in evidence, P7 reproducible, P8 structured data first / project for humans (one-source-of-truth, which P8 explicitly folds in). Where an AGENTS.md generic workflow principle is the sharper fit and has no P1-P8 twin, it is named explicitly (for example "AGENTS.md WP-9, durable notes that survive context loss"; "AGENTS.md WP-18, least authority").

Each functional requirement is tagged: `[CONTROL]` the tool computes/owns it deterministically from state; `[JUDGMENT-IN]` the tool consumes it as a typed input and never manufactures it; `[GEN]` a generation/derivation duty; `[SPEC]` a duty of the data-driven spec itself. Each also carries a STAGE tag (S1 advisory MVP, S2 write-path + fleet + scheduler, S3 authoritative + generation) so the catalog doubles as a staging map without pre-empting the architecture/staging lens.

## 1. Foundational requirement: the data-driven control spec

The human's carried directive ("keep the workflow LOGIC DATA-DRIVEN so it can evolve without heavy code churn") is not one requirement among many; it is the SUBSTRATE the rest sit on, and it must be a requirement of stage 1, not a stage-3 refactor. The reason is P8 and P6 together: if the constants and the transition graph are first hardcoded in Rust and only later migrated into a spec, the migration is a second big-bang change against a still-moving workflow (the exact concrete-pouring the skeptic warned of), and the interim entrenches a fresh code-vs-prose duplication. Building the spec first is cheaper over the whole path and is the only ordering under which "the tool interprets a spec" is ever tested on real content.

- FR-SPEC-1 `[SPEC][CONTROL]` A single machine-readable control spec (round 1's `workflow.toml` candidate, structurally parallel to `<task>.plan.toml`) is the ONE source of the deterministic control DATA: the risk-class -> required-streak map, the total-round cap, the phase/transition graph, the per-status legal-transition table, the isolation-tier preference order, the file/path templates (findings, ledger, exploration, report), and the entry-mode routing table. The driver INTERPRETS this spec; it does not embed the constants in Rust. (P8, P5, AGENTS.md WP-16 one-source.)
- FR-SPEC-2 `[SPEC]` The spec distinguishes a RULE KIND (a transition type, a new gate) from a RULE VALUE (a constant, a threshold, an added edge). Changing a value is a data edit reviewable as a plan edit; adding a rule KIND is a code change to the interpreter. This split is what makes "evolve without code churn" true for the common case (tuning constants, adding a phase edge) while keeping novel mechanics honest as code. (P1, P2.)
- FR-SPEC-3 `[SPEC]` The interpreter parses the spec at the boundary into precise types and rejects a malformed or internally-inconsistent spec loudly (an edge to an unknown state, a streak map missing a risk class, a path template with an unbound slot), rather than driving from a half-valid spec. (P5, AGENTS.md WP-14 parse-don't-validate, WP-21 validate at the boundary.)
- FR-SPEC-4 `[SPEC]` The spec is the SAME source `validate --workflow` reads for its convergence constants, so the enforcement checker (W3/W4/W5) and the driver cannot disagree about the bar. This closes the live `RiskClass::required_streak()`-vs-AGENTS.md duplication (round 1's concrete finding: `src/metrics.rs` returns 1/2 while AGENTS.md line 52 restates it in prose). (P8, AGENTS.md WP-16.)

Contested point (spec format): a dedicated `workflow.toml` versus reusing the plan schema versus a Rust-const table exported to data. Recommendation: a dedicated spec file, because the workflow control graph is a distinct, project-lifetime artifact (one per repo, not one per task) whereas the plan is per-task; conflating them would put per-task status data and process-definition data in one file against P8's single-source-per-concept reading. Judged against P1 (cleaner long-term architecture) and P8, the dedicated spec wins; the Rust-const-table option fails FR-SPEC-2 (a value change becomes a code change).

## 2. Functional requirements: state reconstruction

- FR-REC-1 `[CONTROL]` S1. Reconstruct all control state on every call, statelessly, from the durable files: `<task>.plan.toml` (Roadmap step statuses, `blocked_by` DAG, increments, risk classes, waivers, Open-Questions items) via `src/plan.rs`; `docs/metrics/workflow.jsonl` (the `round`/`escalation`/`dismissal_recheck`/`intake`/`decision`/`baseline`/`waiver` events) via `src/metrics.rs`; and `git` for tree-state and commit-range facts. This is the SAME reconstruction `src/workflow.rs` already ships for W3/W4/W5, run forward instead of as a post-hoc check. (P4, P7, AGENTS.md WP-9.)
- FR-REC-2 `[CONTROL]` S1. Read the current task ledger's `## RESUME STATE` block verbatim (the round-1 `--ledger-fragment` input) for the transient in-flight facts not derivable from plan + log alone: the artifact under review, the diff range, the current-round narrative pointer. The tool reads it; it does not own or write it (that stays the orchestrator's, per `round-log-core`). (P8, AGENTS.md WP-9.)
- FR-REC-3 `[CONTROL]` S1. Derive, per increment, the consecutive-clean streak the outcome sequence implies (`clean` adds one, `new_valid` resets to zero) and the peak against the risk class's required count, exactly as `round_log_consistency_problems` / `w3_problems` compute today. The driver reuses this arithmetic; it does not reimplement it. (P8, AGENTS.md WP-16.)
- FR-REC-4 `[CONTROL]` S1. Be robust to a partially-written or pre-migration log: a record missing the optional `step`/`increment` join fields falls back to the `task`-strip join exactly as `validate --workflow` does, so the driver's view equals the checker's view on the same files. (P4, P3.)

## 3. Functional requirements: per-unit control-state computation (the fleet)

- FR-FLEET-1 `[CONTROL]` S2. Model the workflow as a FLEET of per-unit FSM instances (round 1's hierarchical model), not one global machine: one task machine, one step machine per Roadmap step (statuses reuse the existing `StepStatus` enum), and one review-loop instance per active loop (plan review, each in-review step's work review, the acceptance/standalone pass as the degenerate variant). Each instance carries an id (reusing plan identifiers: step slug, `Increment.id`, and a (phase, artifact) loop id), a current state, and an `awaiting` field. (P5, P1.)
- FR-FLEET-2 `[CONTROL]` S2. Compute each instance's state as a projection of the durable files; there is no separate FSM-state store (that would be a second source of truth that drifts, against P8 and FR-SPEC-4). (P8, AGENTS.md WP-16, WP-9.)
- FR-FLEET-3 `[CONTROL]` S2. Enforce the nesting invariant by construction: a step reaches `complete` only through its embedded work-review instance's `converged`; a `complete` step whose loop never converged is unrepresentable, not merely rejected at runtime. (P5.)
- FR-FLEET-4 `[CONTROL]` S1 (single-unit) / S2 (fleet). Classify each unit as READY (its only outgoing transitions are control transitions the tool can take from state alone, or it just received a supplied judgment input) or BLOCKED (awaiting an unavailable input, or predecessors unfinished), and report per unit which input it is blocked on. (P5, P15-equivalent make-absence-explicit via AGENTS.md WP-15.)

Note on staging: at S1 (advisory MVP) the tool computes and reports one unit's state at a time (the current step / loop), which is the already-decided `state-queries` scope. The full FLEET (many units tracked at once) is an S2 requirement; it earns its keep only when parallel units actually run (see FR-SCHED and the non-functional multi-agent requirement), so the MVP does not need it.

## 4. Functional requirements: next-instruction emission

- FR-EMIT-1 `[CONTROL]` S1. For each ready unit, emit the next INSTRUCTION as a filled role prompt: the role prompt path from `.agents/prompts/`, the step slug and title, the artifact path and diff range (from the ledger fragment), the ledger and plan paths, the findings-file paths from the naming convention, and the phase-keyed Principle reminders. This is parameterized ASSEMBLY of a fixed slot structure, not prose synthesis. (P8, AGENTS.md WP-16 for path templates.)
- FR-EMIT-2 `[CONTROL]` S1. Emit the current per-unit state summary AND the valid transitions from that state, alongside the next instruction, in BOTH machine JSON (`--json`) and human text. (P8.)
- FR-EMIT-3 `[CONTROL]` S1. Emit the resolved `isolation_tier` as a field of each instruction: echo an orchestrator/harness-supplied `--isolation-tier`, or emit `unknown` with a reminder to resolve per the AGENTS.md tier policy when none is supplied. The tool computes/echoes the tier; it never performs isolation (see the out-of-scope line). (P2, AGENTS.md WP-18.)
- FR-EMIT-4 `[CONTROL]` S2. Emit periodic reminders on a defined cadence (the Project Principles, phase-specific disciplines), keyed to the phase and the unit's state so the reminder is relevant at the point of action (the core anti-drift mechanism: re-inject the rule when it applies). (P6 for the value proposition; AGENTS.md WP-9.)
- FR-EMIT-5 `[GEN]` S1. Instruction prompts are DERIVED from the role-prompt files, not copied into the driver; when a role prompt changes, the emitted instruction follows. Do not create a second copy of the prompt text inside the driver (that is the templater-vs-prompt drift surface the skeptic named). (P8, AGENTS.md WP-16.)

## 5. Functional requirements: driving the review / convergence loop

These are the heart, and they are the arithmetic `workflow.rs` already computes, run forward (round-1 FSM lens, section "The review-loop Mealy machine"). All are `[CONTROL]` except the inputs they consume.

- FR-LOOP-1 `[CONTROL]` S1. Count the streak and reset it on `new_valid`; compute `required` from `risk_class` via the spec map; detect convergence (`consecutive_clean >= required`) and emit `converged`. (P8.)
- FR-LOOP-2 `[CONTROL]` S1. Detect the total-round cap (default 5, from the spec) and emit `escalate`; apply the convergence-before-cap precedence (a round that both hits the cap and is the converging clean round converges, per AGENTS.md line 53). (P5.)
- FR-LOOP-3 `[CONTROL]` S1. Gate a clean round behind the backstop re-check when the round's top DISMISSED severity is high or critical: emit `require_recheck`, hold the streak PROVISIONAL, and advance only on a recorded `recheck_result(upheld)`; on `overturned`, flip the round to new_valid and reset the streak. (P5, AGENTS.md WP-5.)
- FR-LOOP-4 `[CONTROL]` S1. Reset both counters after a `human_decision(resume)` at an escalation so the cap does not immediately re-fire, and retire them when a decision ends the loop (accept, or send-back). (P5, matches AGENTS.md convergence text.)
- FR-LOOP-5 `[JUDGMENT-IN]` S1. CONSUME, never produce: the triager's round `outcome` (clean vs new_valid), the `risk_class` at loop-open (defaultable, not decidable), the backstop `recheck_result`, and the human decision at an escalation. The tool blocks awaiting these; it never invents a verdict. (AGENTS.md WP-18, WP-5; this is the no-judgment invariant applied.)
- FR-LOOP-6 `[CONTROL]` S1. Reuse the SAME loop definition for the degenerate acceptance / standalone-review single pass (no round loop, no cap, keeps the high/critical re-check), with the round/streak fields pinned unused. One definition, three instantiation sites. (P8, AGENTS.md WP-16.)

## 6. Functional requirements: phase sequencing

- FR-PHASE-1 `[CONTROL]` S1/S2. Sequence the task machine: planning -> plan-review loop -> implementing (drive step machines) -> accepting -> done, with a shortfall at acceptance routing back to planning/implementing. (P5.)
- FR-PHASE-2 `[CONTROL]` S1/S2. Sequence the step machine: not-started/next -> in-progress (spawn implementer) -> in-review (embed work-review loop) -> complete, with abandon -> abandoned. Statuses map onto the existing `StepStatus` enum. (P5.)
- FR-PHASE-3 `[CONTROL]` S1. Compute the STOP predicate: done iff every Roadmap step is complete/skipped and an acceptance record confirms the Success Criteria. (P5.)

## 7. Functional requirements: dependency scheduling over `blocked_by`

- FR-SCHED-1 `[CONTROL]` S2. Compute the READY FRONTIER over the plan's `blocked_by` DAG: a step is schedulable iff every slug in its `blocked_by` is `complete` or `skipped`; the frontier is the antichain of not-started/next steps with all dependencies satisfied. (P5.)
- FR-SCHED-2 `[CONTROL]` S2. PROPOSE the frontier as the set that MAY run in parallel now; the orchestrator (which owns isolation and the concurrency budget) decides how many to actually spawn. The tool schedules readiness; it does not fan out. (P2, AGENTS.md WP-18.)
- FR-SCHED-3 `[CONTROL]` S2. Keep scheduling at STEP granularity (increments carry no `blocked_by`; the DAG is step-level). Increment ordering within a step stays the orchestrator's sequential choice. (P2; do not invent finer scheduling than the data supports.)

Contested point (is the scheduler in scope at all): the skeptic's strongest live objection is that a solo/small-team dogfood walks the DAG one node at a time, so a parallel scheduler is machinery for a scale not yet reached (P2, and premature per P6). Against that, the human has named the full driver as the target and the fleet+scheduler is its defining concurrency capability. Recommendation: the scheduler is IN scope for the full driver but is an S2 requirement gated on the multi-agent non-functional requirement below actually being exercised; ship the single-unit advisory reconstruction (S1) with the scheduler stubbed to "the one ready unit" until parallel runs occur. This keeps P6 (build the scale machinery when the scale is real) without dropping it from the target.

## 8. Functional requirements: human-input points

The workflow routes EVERY human decision through one contract (options, trade-offs, recommendation, Principle-judged reasoning). The driver models the CONTROL of this uniformly and never its CONTENT.

- FR-HUMAN-1 `[CONTROL]` S1. Detect any unit in the single `awaiting=human` state (escalation, intake of a new/changed request, an open/clarifying question, a directly-asked decision) and emit the contract-shaped PROMPT that asks the agent to prepare the contract. (P5.)
- FR-HUMAN-2 `[JUDGMENT-IN]` S1. CONSUME the recorded human `decision` (and its options/recommendation/chosen fields); never generate the options or the reasoning. On resume, require the durable receipt that `validate --workflow`'s W4 already enforces for decided items. (AGENTS.md WP-18; the no-judgment invariant.)
- FR-HUMAN-3 `[CONTROL]` S1. Report the Open-Questions queue's open items so the checkpoint queue-push has a computed, current list to present. The tool surfaces the queue; the human decides each item. (P8.)
- FR-HUMAN-4 `[JUDGMENT-IN]` S2. CONSUME the intake CLASSIFICATION (trivial vs non-trivial) and the "match ceremony to stakes" call as inputs; the tool records and routes on them but does not classify. (AGENTS.md WP-18.)

## 9. Functional requirements: recording transitions (the write path)

This is the S2 boundary and the one that reopens Q-24 (the deliberate "no runtime write-path dependency on the binary" stance). Every item here is a GUARDED write, and the whole group is behind the evidence gate.

- FR-REC-W-1 `[CONTROL+JUDGMENT-IN]` S2. Provide `record-*` subcommands (`record-round`, `record-decision`, `record-recheck`, `record-escalation`, ...) that ATOMICALLY append the JSONL event AND let the next recompute reflect it. Each records a supplied JUDGMENT (the input) and advances the affected instance's state; it does not compute the judgment. (P5, P8.)
- FR-REC-W-2 `[CONTROL]` S2. Validate the transition is LEGAL from the current computed state BEFORE appending (reject a round record on a step not in-review, reject a clean advance past an already-met streak), exiting non-zero and printing the actual state. This is the structural enforcement (P5) that hand-writing JSONL cannot give. (P5.)
- FR-REC-W-3 `[CONTROL]` S2. Preserve graceful degradation: the log stays hand-writable, so a harness without the binary can still run the workflow (honouring Q-24's fallback). The write-path is a convenience-plus-enforcement layer, not a hard runtime dependency. (P3, Q-24.)
- FR-REC-W-4 `[CONTROL]` S2. Keep append-only semantics: never rewrite past log lines (the log accumulates across tasks and is committed). (P4, P7, AGENTS.md WP-9.)

Contested point (Q-24 reopening): Q-24 declined the validated-append writer to avoid coupling the workflow to the binary. Recommendation: the write-path is IN scope for the full driver but STRICTLY behind the advisory-evidence gate (round-1 synthesis condition), and it must keep FR-REC-W-3's degradation so Q-24's core concern (the workflow must survive without the binary) is preserved. Judged against P6, do not ship the write-path until the advisory tier shows agents run the tool consistently; judged against P5, once shipped it is the only place illegal transitions become truly unrepresentable rather than post-hoc detected.

## 10. Functional requirements: escape hatches, overrides, abandonment

- FR-ESC-1 `[CONTROL]` S2. Model every escape as a FIRST-CLASS RECORDED transition, not a bypass: `record-override --unit <id> --to <state> --reason <text>` forces a unit to a defined target state and logs that a human overrode the rules, by whom and why. Because the transition function is total, the forced move lands in a defined state, not a hole. (P5, AGENTS.md WP-9.)
- FR-ESC-2 `[CONTROL]` S1. Support abandonment as an existing first-class transition (to `abandoned`/`skipped`), matching AGENTS.md's abandoned-before-convergence path. (P5.)
- FR-ESC-3 `[CONTROL]` S1. Inherit accept-at-escalation as the already-modelled `accepted-at-escalation` waiver, record-backed by its `escalation` (W5 checks the join). (P8.)
- FR-ESC-4 `[CONTROL]` S1. Provide the ADVISORY deviation hatch as the master escape at every stage: in advisory mode the tool SUGGESTS and the agent may deviate; a deviation is recorded (an override note), so the OVERRIDE RATE is the measured evidence (P6) that decides whether to deepen toward authoritative-blocking driving. A workflow the machine mis-models shows up as a high override rate, not as silent friction. (P6.)

## 11. Functional requirements: generating the AGENTS.md control fragments (the ambitious end)

- FR-GEN-1 `[GEN]` S3. Generate ONLY the control-constant fragments of the AGENTS.md workflow section from the spec (the streak-per-risk-class map, the cap, the path-convention vocabulary, the phase order), via the same `render` closure that already generates the plan view (`vocabulary_section()` / `status_line()` are the working precedents). Guard with `render --check` byte-compare. (P8, AGENTS.md WP-16.)
- FR-GEN-2 `[GEN]` S3. Do NOT generate the rationale, role guidance, human-input contract, or file-safety disciplines; those stay hand-authored prose sidecars (parallel to the plan's `.steps/`/`.questions/` bodies). The generated fragments carry the WHAT; the sidecars keep the WHY. (AGENTS.md WP-19 document-the-why, WP-20 self-contained.)
- FR-GEN-3 `[GEN]` S3. Draw the generation boundary at the control/judgment line, so the prose statement of every control constant is generated from the same spec the driver executes and the two cannot disagree; rationale prose has no executable counterpart and so is single-sourced by construction. (P8.)

Contested point (generation feasibility): the skeptic's sharpest doubt is that "generate readable English workflow guidance from an FSM table" is a hard, unproven artifact, and if it is not built you keep two hand-maintained sources (the very drift you set out to kill). The tool-vs-prose lens answers by narrowing generation to CONSTANT fragments (which `render` already does for the vocabulary), not whole-section prose synthesis. Recommendation: FR-GEN is IN scope for the full driver but is the LAST stage (S3) and must be PROVEN on the smallest fragment (the convergence constants) before any wider control prose is generated; until proven, keep the hand-authored prose and let the spec drive only the tool (P6). This is the round-1 "start by generating only the convergence constants" slice, kept as the generation on-ramp.

## 12. Non-functional requirements

- NFR-DET Determinism. Same inputs (files) -> same output, on any machine, every call. The transition function is TOTAL (every state,input pair has a defined image), so there are no undefined transitions and no nondeterministic branches. (P4, P5, P7.)
- NFR-STATELESS Statelessness. All state is recomputed from the durable files each call; no hidden in-memory or on-disk FSM store. This is what makes the tool survive a crash or compaction and is a hard constraint, not an optimization. (P8, AGENTS.md WP-9, WP-16.)
- NFR-RESUME Resumability. After a compaction or a lost session, the next call reconstructs the full fleet state from plan + log + ledger-fragment + git and continues from where the files say the work left off. (AGENTS.md WP-9.)
- NFR-PORT Cross-harness portability. The tool reads committed files and emits text/JSON; it names no harness API. Isolation-tier and fan-out are EMITTED as data for the orchestrator to act on, so the same binary works under any harness (container, worktree, or file-safety fallback). (P7, P2.)
- NFR-PARALLEL Multi-agent / parallel safety. Concurrent units accumulate independent records in the append-only log; a recompute reads all records and reports each unit's state independently, with no inter-call memory. The write-path (S2) must be safe under concurrent appends (append-only, no rewrite). (P4, AGENTS.md WP-9.)
- NFR-PERF Performance. Fast enough to run at every orchestrator decision point (cold resume, end of a review round, escalation, step boundary) but NOT per-turn during active work (Q-28/Q-34 found per-turn polling is a negative token trade). Recompute at plan scale is negligible; the design must not create a per-turn incentive. (P6 for the measured scope; matches the Q-28 decision.)
- NFR-AUDIT Auditability. Every transition is traceable to a durable record: a round, a decision receipt, an escalation, an override, a waiver. The log plus the plan is a complete account of what happened and why, so a reviewer can reconstruct the control history. (AGENTS.md WP-9, P8.)
- NFR-AUTH Least authority. The tool is READ-ONLY by default (advisory), with the write-path a strictly guarded, opt-in, append-only, transition-validated extension. It never edits the plan, never writes the ledger, never performs isolation, never spawns agents, never rewrites the log. It has exactly the authority to count, sequence, schedule, gate, emit, and (guardedly) append. (AGENTS.md WP-18, P3.)
- NFR-NOJUDGE The no-judgment invariant. The tool consumes every judgment (triage outcome, risk class, recheck result, human decision, intake classification, commit) as a typed INPUT and never computes one. Getting this wrong in the permissive direction makes the tool hallucinate verdicts; in the restrictive direction it straitjackets the agent. This invariant is the primary correctness property and overrides any convenience that would have the tool infer a judgment. (AGENTS.md WP-18, P5.)
- NFR-SPEC-STABILITY Spec evolvability. A constant/threshold/edge change is a spec (data) edit reviewed like a plan edit; only a new rule KIND touches interpreter code. This keeps the still-moving workflow cheap to change (the human's carried constraint) and is a measurable property: count how many past workflow changes would have been data-only vs code. (P1, P2, P6.)

## 13. Scope: in vs out for "the full driver"

IN scope (the full driver target, staged S1 -> S3):

- The data-driven control spec (FR-SPEC-1..4), authored FIRST. (S1.)
- Stateless reconstruction reusing the W3/W4/W5 machinery (FR-REC-1..4). (S1.)
- The review/convergence loop driver, phase sequencing, stop predicate (FR-LOOP, FR-PHASE). (S1.)
- Next-instruction emission with filled role prompts, JSON + text, isolation-tier echo, cadenced reminders (FR-EMIT). (S1.)
- The advisory deviation hatch, abandonment, accept-at-escalation (FR-ESC-2..4). (S1.)
- Human-input CONTROL: await-state detection, contract-prompt emission, receipt consumption, queue reporting (FR-HUMAN-1..3). (S1.)
- The per-unit FSM fleet and the `blocked_by` ready-frontier scheduler (FR-FLEET, FR-SCHED). (S2, gated on real parallelism.)
- The guarded `record-*` write-path with transition validation and preserved degradation (FR-REC-W, FR-ESC-1 override). (S2, gated on advisory evidence + reopening Q-24.)
- Generation of the AGENTS.md control-constant fragments via `render --check` (FR-GEN). (S3, gated on the smallest-fragment proof.)
- Authoritative-blocking driving (the tool refuses to advance without the recorded event). (S3, gated on measured advisory adoption.)

OUT of scope (or a separate concern):

- Performing isolation. The tool emits the resolved tier; the orchestrator/harness creates worktrees and containers. The isolation MECHANISM is the separate `optional-modules` behavioural module and the agent-box/agent-images projects, not the driver. (AGENTS.md WP-18.)
- Computing any judgment: no auto-triage, no auto risk-classification beyond suggesting a default, no auto "small and reviewable", no auto ceremony-collapse, no auto acceptance, no trivial/non-trivial decider. (NFR-NOJUDGE.)
- A general/Turing-complete process DSL. The spec encodes THIS workflow's fixed control rules, not arbitrary processes. (P2.)
- A persisted FSM-state store. Recompute always; a second store reintroduces the drift the initiative fights. (NFR-STATELESS.)
- Increment-level DAG scheduling (increments carry no `blocked_by`). (FR-SCHED-3.)
- Generating the RATIONALE/role/contract/discipline prose (only the control-constant fragments are generated). (FR-GEN-2.)
- Merge/conflict authoring or reconciliation (already routed to an isolated implementer and re-reviewed; authored content, not control). (AGENTS.md worktree lifecycle.)
- Per-turn polling during active work (a negative token trade). (NFR-PERF.)
- Subsuming the re-grounding + provenance step or Q-50 doc-currency guidance. The driver COMPLEMENTS them (control-drift vs context-drift vs doc-truth-drift) and DEPENDS on Q-50 (a driver that emits prompts assembled from role-prompt files is only as correct as those files). Those un-gate and ship on their own schedule. (Round-1 synthesis point 5.)

Dependencies on existing / planned pieces:

- `state-queries` (Q-28/Q-34, `not-started`, order 46): the natural home for S1; the driver EXTENDS it with instruction generation and the spec. Coordinate to avoid duplicating the `status --resume` slice. `state-queries` depends on `round-log-core` (complete).
- `workflow-invariants` / `validate --workflow` (`src/workflow.rs`, complete): the reconstruction the driver reuses and shares constants with (FR-SPEC-4, FR-REC-3).
- `render` (`src/plan/render.rs`, from `structured-skeleton`, complete): the closure FR-GEN reuses for the AGENTS.md fragments and `--check` guard.
- `checks` module (`src/checks.rs`; `optional-modules` inc2, worktree-isolated `agent-scaffold checks`): a sibling deterministic gate the driver can point at but does not own; the checks-reviewer runs in the review phase the driver sequences.
- `isolation` module (`optional-modules` inc3, agent-box/agent-images): the MECHANISM the driver emits a tier for but never invokes.
- `test-driven` (Q-30) and `mutation` (Q-31) modules (`not-started`): future review-phase participants the driver sequences like any reviewer; no direct coupling.
- `workflow-viz` (`deferred`): a projected live view that could consume the driver's JSON output; not a dependency of the driver, a potential consumer.

## 14. Open requirements questions for the human

- ORQ-1 Advisory vs authoritative terminal. Does the full driver's END STATE remain advisory-with-recorded-deviation (the agent keeps the wheel, the tool suggests and measures), or does it become authoritative-blocking (the tool refuses to proceed without the recorded event)? Trade-off: authoritative gives the strongest P5 guarantee but requires reliable binary availability in every harness and an onboarding change ("never proceed without consulting the tool"); advisory is portable and reversible but relies on adherence. Recommendation: reach authoritative-CAPABLE (the write-path validates transitions) but keep advisory the DEFAULT, flipping to blocking only per-repo once the override rate is measured low. Reasoning: P6 (flip on evidence) and P3 (portable default). The human owns this because it changes the tool's authority (NFR-AUTH).
- ORQ-2 Does the driver GATE commits / merges? Round 1 kept the tool out of the merge path (the orchestrator integrates converged work; conflict authoring is re-reviewed). Should the driver instead block a step-complete or a merge until its recorded rounds exist (a stronger version of what `validate --workflow` checks post-hoc)? Trade-off: gating catches an unreviewed merge at the moment it happens (P5) but couples the commit path to the binary (against Q-24). Recommendation: NO commit-gating in the tool; keep `validate --workflow` as the CI/pre-commit backstop (its existing role) and let the driver's write-path be the point-of-action enforcement. The human owns the Q-24 reopening.
- ORQ-3 Single-project vs multi-project. The spec is per-repo and the log is per-repo-accumulating-across-tasks. Is a single driver instance ever expected to drive multiple concurrent PROJECTS (multiple plans / repos), or strictly one repo's task set at a time? Recommendation: single-repo scope; multi-repo is a YAGNI until a real multi-repo workflow exists. The human confirms whether multi-repo is a target.
- ORQ-4 Spec ownership and change-review. When the control spec changes (a cap tweak, a new phase edge), does that change go through the normal plan-review loop, or a lighter data-edit path? Recommendation: treat a spec VALUE change as a normal reviewed change (it alters workflow behaviour) but a low-ceremony one; a spec RULE-KIND change is a code change with full review. The human confirms the ceremony.
- ORQ-5 Reminder cadence. FR-EMIT-4's "periodic reminders" needs a defined trigger (every call? every N rounds? phase-entry only?) to avoid noise that trains agents to ignore it. Recommendation: phase-entry and escalation only, not every call. The human sets the cadence once there is usage data.

## 15. Recommendation on the scope lines

Adopt the staged IN/OUT lines in section 13, with three load-bearing commitments:

1. Author the DATA-DRIVEN SPEC first (section 1), so the constants and transition graph never live in Rust that later needs migrating. This is the single change that makes "the full driver" affordable against a still-moving workflow, and it is the human's own carried constraint. (P8, P6.)
2. Keep the tool READ-ONLY by default with a strictly guarded, opt-in, append-only write-path (NFR-AUTH), and keep authoritative-blocking and the write-path behind the round-1 advisory-evidence gate. This preserves Q-24's degradation guarantee and the least-authority invariant while still reaching the full driver's enforcement power. (AGENTS.md WP-18, P3, P6.)
3. Stage generation LAST and prove it on the smallest fragment (the convergence constants) before widening. (P6.)

The recommended scope line for "the full driver" is therefore: everything in section 13's IN list is the TARGET, but the build order is S1 (advisory MVP on the spec, the natural `state-queries` increment) -> S2 (fleet + scheduler + guarded write-path, gated on real parallelism and advisory-adoption evidence, and on reopening Q-24) -> S3 (fragment generation + authoritative-blocking, gated on the generation proof and the measured override rate). The scope is the full driver; the GATES are what keep each large, less-reversible stage honest against P6.

## 16. YAGNI boundary

- Do NOT hardcode the constants/transition graph in Rust and migrate later; author the spec first (FR-SPEC). A migrate-later plan is a second big-bang against a moving workflow.
- Do NOT build the fleet + scheduler before parallel units actually run; ship S1 with a single-unit "the one ready unit" scheduler and expand when NFR-PARALLEL is exercised. (FR-SCHED contested point.)
- Do NOT build the `record-*` write-path before the advisory tier shows agents run the tool consistently, and preserve hand-writable JSONL when you do (FR-REC-W-3). Do not reopen Q-24 speculatively.
- Do NOT generate whole-section AGENTS.md prose; generate only control-constant fragments, proven smallest-first (FR-GEN).
- Do NOT flip to authoritative-blocking before the override rate is measured low (ORQ-1).
- Do NOT compute any judgment, ever (NFR-NOJUDGE): no auto-triage, no auto risk class beyond a default, no auto acceptance, no trivial/non-trivial decider.
- Do NOT persist FSM state, poll per-turn, perform isolation, schedule increments, author merges, or subsume the re-grounding step / Q-50 (section 13 OUT list).
- Do NOT build a general process DSL; the spec encodes THIS workflow only.
