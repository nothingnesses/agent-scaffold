# Q-51 Mealy-machine workflow driver: synthesis

Synthesises the four lens explorations in this directory: `fsm-concurrency-model.md`, `tool-vs-prose-boundary.md`, `io-contract-and-adoption.md`, `yagni-skeptic.md`. Written by the orchestrator to move Q-51 from `exploring` to `open` with options for the human.

## The question

Encode the workflow itself as a deterministic state-machine DRIVER (`agent-scaffold` emits the next instruction prompt from the current state), so control flow lives in a deterministic program rather than fallible agent memory, to fight agent drift as the workflow grows.

## Where the four lenses converge (strong agreement)

1. Build the ADVISORY MVP first, not the full driver. A read-only `next` command that recomputes control state statelessly from `plan.toml` + `workflow.jsonl` + git on every call (no second state store, survives compaction), reuses the convergence reconstruction `src/workflow.rs` already ships (W3/W4/W5), and prints: the current per-unit state, the valid transitions, and the next-action the rules dictate as a filled-in instruction prompt (role prompt + artifact + diff range + findings paths + ledger + Principle reminders), in both JSON and human text.
2. This MVP is largely the ALREADY-PLANNED `state-queries` step (Q-28/Q-34, currently not-started), which was already going to extend `status` over the plan + metrics. The Mealy idea reframes and slightly extends it (the next-action reminder), rather than adding a wholly new subsystem.
3. DEFER the full driver: the per-unit FSM fleet, the `blocked_by` dependency scheduler, atomic event-recording `record-*` write-path subcommands, authoritative/blocking driving, migrating convergence and sequencing logic out of AGENTS.md into Rust, and generating the AGENTS.md workflow section from a machine spec. These are a major subsystem (larger than structured-skeleton) and would harden a workflow that is still changing weekly; the marginal drift reduction over the advisory tier is unproven.
4. The tool stays OUT of worktree/container management (orthogonal to isolation). It reads durable state on main and emits the resolved `isolation_tier` and the spawn instruction; the orchestrator/harness isolates and fans out. This composes with all three isolation tiers and with parallel work unchanged.
5. The driver does NOT subsume the re-grounding + provenance step or Q-50 (doc-currency guidance); it complements them (control-drift vs context-drift vs doc-truth-drift). Q-50 is in fact a prerequisite if the driver is built. So those cheaper items should be un-gated rather than made to wait on this.

## One concrete high-value refinement (tool-vs-prose lens)

The code/prose duplication the human worries about ALREADY exists: `required_streak()` returns 1/2 in `src/metrics.rs` while AGENTS.md restates the same numbers in prose. So single-sourcing the convergence CONSTANTS (one source both `validate --workflow` and the advisory `next` read, and which `render` can project into the AGENTS.md control fragment) closes a live drift gap cheaply, and is the natural first generation step. This is a small, low-regret slice that can ride with the advisory MVP.

## The control-vs-judgment boundary (all lenses agree)

The tool owns only CONTROL transitions computable from state: counting streaks, applying the cap, sequencing phases, scheduling the ready frontier, gating the backstop, resolving the isolation tier, emitting prompts. It CONSUMES agent/human JUDGMENT as typed inputs (a triage verdict {clean|new_valid}, a recheck result, a human decision, a commit) and never manufactures a verdict. Getting this boundary wrong (the tool deciding judgments) is the primary failure mode; the advisory tier keeps the human/agent firmly in the loop.

## Options for the human (control decision)

- Option A (all four explorers recommend): BUILD THE ADVISORY MVP, evidence-gated. Fold/extend `state-queries` into a read-only `agent-scaffold next` (stateless, reuses W3), optionally sliced with single-sourcing the convergence constants (closing the existing `required_streak`/AGENTS.md duplication). Instrument adherence + drift over ~10 step cycles. Un-gate the re-grounding step and Q-50 rather than making them wait. Escalate to the full driver ONLY on evidence (measured control-drift persisting after the advisory tool, a workflow that has stopped changing, real parallel/multi-agent scale, and a proven AGENTS.md-generation closure). Principle 6 (evidence first), Principle 16 (one source, via the constants), lowest regret.
- Option B: BUILD THE FULL DRIVER NOW (per-unit FSM fleet + scheduler + event-recording + AGENTS.md generation). Rejected by all four as premature: a subsystem larger than structured-skeleton, hardening an evolving workflow, with unproven marginal value over the advisory tier. Trades against Principle 6 and Principle 4 (small, reviewable changes).
- Option C: DO NOT BUILD; rely on the lighter drift mitigations only (the re-grounding step, Q-50 doc-currency guidance, better prompts, and the existing `status`/`validate --workflow`). The skeptic's floor; viable if the measured drift is already low, but forgoes the point-of-action reminder that most directly addresses "the agent forgot a rule that applies here."

Orchestrator recommendation: Option A. It captures most of the anti-drift value (drift is mostly "an agent forgot a rule that applies right here," which a point-of-action reminder fixes), reuses machinery already built, is barely new scope beyond the planned `state-queries`, closes a real existing duplication via the constants slice, and keeps the large, irreversible parts (logic migration, generation, authoritative driving) behind an evidence gate.

## YAGNI boundary (what NOT to build under Option A)

No authoritative/blocking driving; no migrating convergence/sequencing prose into Rust beyond the constants; no AGENTS.md-from-machine generator; no per-unit FSM fleet or `blocked_by` scheduler; no `record-*` write-path (keep the tool read-only, honoring the Q-24 no-write-path-runtime-dependency stance); no general process DSL; and the tool encodes no judgment.
