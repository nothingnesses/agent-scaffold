# Q-51 Mealy driver, round-2 synthesis (full-driver design + staged build plan)

Synthesises the four round-2 lens explorations (`r2-requirements-scope.md`, `r2-user-flows.md`, `r2-design-space-vision.md`, `r2-architecture-build-path.md`) plus round 1. The human chose the FULL DRIVER as the target and asked for a deeper design pass before building. This is the material to decide against.

## The unifying picture (all four agree)

The driver is the FORWARD member of a projection family over two sources: the append-only event log `workflow.jsonl` and a data-driven control spec. `validate --workflow` is the BACKWARD check (did the past obey the rules), `status` is the SUMMARY, and the driver (`next`) is the FORWARD execution (given the state, what the rules dictate next). All three are folds over the same log, so the driver is not a new subsystem bolted on, it is the missing forward member of a family whose backward and summary members already ship. `render` extends the family to prose: it generates the AGENTS.md control fragments from the same spec, so process logic and process prose cannot drift.

## The data-driven boundary (resolves the skeptic's objection precisely)

The spec (`.agents/workflow.toml`) holds only CONTROL CONSTANTS and orderings: the required clean-round streak per risk class, the total-round cap, the backstop severity threshold, the phase sequence, the isolation-tier order, the role -> prompt-path map, and the findings/ledger path templates. The transition FUNCTION stays Rust (the streak/cap arithmetic the W3 checker already computes, run forward). So a workflow VALUE change is a data edit; only a new rule KIND is code. That is the answer to "keep it data-driven so the workflow evolves without pouring concrete": the parts that change often (the numbers, the sequences, the templates) are data; the fixed interpreter is small code. Both `validate --workflow` and the driver read one `WorkflowSpec`, which single-sources today's hardcoded `RiskClass::required_streak` (currently duplicated against AGENTS.md prose).

## The control-vs-judgment boundary (enforced by types)

The tool owns CONTROL (counting, sequencing, scheduling, gating, prompt emission); it CONSUMES agent/human JUDGMENT as typed `Input` variants (a triage verdict {clean|new_valid}, a recheck result, a human decision, a commit, an override). The transition function never manufactures a verdict. This is the load-bearing invariant; get it wrong and the tool either straitjackets or hallucinates decisions.

## The interaction model (from the user flows)

Semi-authoritative event-recording, non-blocking: `agent-scaffold next` reads durable state and emits the next instruction plus a pre-filled `next_record_command`; `agent-scaffold record-*` validates the transition before appending the log (an illegal transition exits non-zero); NEITHER blocks the orchestrator, so the workflow never deadlocks if the tool is skipped, but `validate --workflow` flags the gap after the fact. Fully-blocking harness-hook enforcement is a far, gated stage.

## The staged build path (the spine of the proposal)

Committed near-term ordering, each a reviewable increment; later stages behind EVIDENCE GATES so no increment hardens the still-evolving workflow:

- Stage 0a (RISKY, no deps): add `WorkflowSpec` + `.agents/workflow.toml` (constants) + `WorkflowSpec::builtin()`; single-source the convergence constants into the W3 check; add a `--workflow-spec` flag. Behaviour-preserving (a `builtin()`-equals-old-constants assertion guards it). VALUE: closes the live `required_streak`/AGENTS.md duplication now, and lays the spec foundation. This is a real bug-closing win independent of the driver.
- Stage 0b (RISKY, deps 0a): generate the AGENTS.md control-constant fragment from the spec via a pack `{{workflow_control}}` render slot, byte-compare guarded. Proves the prose-generation closure on the smallest content.
- Stage 1 (LOW_RISK, deps 0a): the advisory, read-only, stateless `agent-scaffold next` MVP, largely the already-planned `state-queries` step (Q-28/Q-34) plus a next-action reminder, forward-projecting the W3 reconstruction for the active review loop. Ships the point-of-action anti-drift value and is the EVIDENCE SOURCE for every later gate (adherence rate, drift reduction over ~10 step cycles).
- Stage 2 (RISKY, deps 1): type the FSM engine in `src/driver/` (a spec-parameterized `ReviewLoop` Mealy machine, nested step/task machines, a stateless `reconstruct` layer shared with the checker so the driver and W3 provably run the same arithmetic).
- Stage 3+ (GATED): the `blocked_by` ready-frontier scheduler (gate: real parallelism); generation-widening beyond constants; the `record-*` write-path (gate: advisory adoption evidence + reopening Q-24's no-write-path stance); authoritative/blocking driving (gate: a measured-low override rate); and the far corner (calibration closure, workflow-viz as a read, multi-repo).

## Open decisions the human owns (surfaced, not decided here)

- ORQ-1: the terminal authority, advisory-and-recording (recommended destination) vs authoritative-blocking. Shapes the far stages, not the near ones.
- ORQ-2: does the driver ever gate commits/merges (ties to Q-24's no-write-path-runtime-dependency stance)?
- ORQ-3: single-project vs multi-project/cross-repo driving.
- ORQ-4: the ceremony for changing the workflow spec (it is now load-bearing config).
- ORQ-5: reminder cadence (every `next` call vs periodic).
None of these block Stages 0a/0b/1; they gate the later stages they concern.

## Build-detail gaps the flows found (for the implementer, not human decisions)

A bootstrap "planning complete" signal; a `record-step-start` event (per-unit diff-base, so the ledger becomes purely human narrative and supports parallel in-flight units); a loop-reset marker after a send-back so total rounds are not double-counted; extending W5 to accept `record-override` as completion evidence (parallel to waivers); pre-computing findings-file paths in the `next` output; and defaulting an omitted risk class to `risky` with a warning (fail-safe).

## Orchestrator recommendation

Fold this into the plan as a new staged initiative (a `workflow-driver` umbrella with the stage ladder above, mirroring the `structured-skeleton` umbrella-plus-increments shape), and BUILD STAGE 0a FIRST: it is low-regret, closes a real existing duplication bug, lays the data-driven-spec foundation the whole driver rests on, and commits no concrete to the driver's shape. Then 0b and the Stage 1 advisory MVP, gathering the adoption/drift evidence that gates every later stage. The full driver remains the committed destination; the staging is how we reach it without hardening a workflow that is still changing weekly. Resolve ORQ-1..5 as we approach the stages they gate, not up front.
