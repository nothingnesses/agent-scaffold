# Q-51 exploration: YAGNI / skeptic lens

## The question, through this lens

Q-51 proposes turning `agent-scaffold` into a deterministic state-machine DRIVER: a Mealy-machine REPL (reframed by the orchestrator as a fleet of per-unit control FSMs plus a `blocked_by` dependency scheduler) that parses the current plan/ledger/metrics/codebase state and emits the next INSTRUCTION PROMPT for the agent, so control flow lives in a deterministic Rust program instead of fallible agent memory. The stated goal is to fight AGENT DRIFT as the workflow grows.

My job here is not to design that machine well. It is to ask, honestly, whether we should build it at all, and if so what the smallest version is. I am arguing the case AGAINST the full driver and FOR the smallest useful thing, so the human's decision is not one-sided. I take the anti-drift goal as real and shared; the disagreement is only about the cheapest instrument that reaches it.

The one-line thesis: most of the drift problem is solved by an ADVISORY read-and-remind tool that this project has ALREADY decided to build (the `state-queries` step: `next` plus a `status --resume` slice, Q-28/Q-34, currently `not-started`), and the marginal drift the full driver removes on top of that is small, uncertain, and bought at the price of a major new subsystem that itself becomes a drift and maintenance surface while the workflow it encodes is still molten. Build the advisory tier, measure, and let evidence decide whether authoritative driving is ever worth it.

## What already exists (the honest baseline)

The framing sometimes reads as if the tool must be built from zero to get any state awareness. It does not. Grounding in the current tree:

- `agent-scaffold status` (built, `src/main.rs` `StatusArgs`, projection code from ~L426) already emits a derived summary of the plan's Roadmap steps, open questions, and a metrics-record count, from the TOML source or Markdown plan. It is the "print the current state" half of the proposal, already shipping.
- `agent-scaffold validate --workflow` (built, `src/workflow.rs`, ~1712 lines: the W3 convergence-or-waiver, W4 decision-receipt, and W5 waiver-integrity checks) already RECONSTRUCTS control state from the durable files and reports every illegal state: a `complete` step with no converged rounds, a decided question with no receipt, a waiver that does not join its escalation. This is the post-hoc enforcement stage of the structured-data direction, and it is the part that actually catches control-drift today.
- `state-queries` (Q-28/Q-34, `not-started`, order 46) is ALREADY DECIDED: a thin `next` plus a `status --resume` slice that reads the plan parser (`src/plan.rs`) plus the JSONL round log (`src/metrics.rs`) plus a verbatim RESUME STATE extract, and reuses `workflow-invariants`' next-action transition logic. The decision text (Q-34) explicitly scopes it to the resume and CI/check paths, NOT a wholesale navigator, and warns that per-turn polling is a NEGATIVE token trade during active work. `next` was already slated to "double as the enforcement aid."

So the project already owns three of the four moving parts the driver wants: state reconstruction, illegal-state detection, and a decided-but-unbuilt next-action advisor. The genuinely NEW things Q-51 adds on top are: (1) emitting the filled-in next INSTRUCTION PROMPT (not just naming the next action), (2) AUTHORITATIVE driving (agents run it to be TOLD what to do, rather than to check themselves), (3) migrating convergence/sequencing LOGIC out of AGENTS.md prose into Rust, (4) generating the AGENTS.md workflow section FROM the machine definition, and (5) a parallel dependency scheduler over `blocked_by`. Every one of those five is where the cost and the risk live, and each has a weak marginal-value case, argued below.

## The design space

I lay out four points on the spectrum, from least to most build.

### Option 0: Do nothing new (lighter mitigations only, already queued)

Build nothing for Q-51. Rely on what exists plus what is already decided: `status`, `validate --workflow`, the `state-queries` `next`/`status --resume` slice when it lands, the queued re-grounding + provenance step, and the Q-50 doc-currency guidance. These already attack drift from three angles: `validate --workflow` catches control-drift after the fact, re-grounding re-establishes content context at task entry, and Q-50 keeps the docs true so the agent reads a correct rulebook.

- Cost: near zero (the items are already planned).
- Drift covered: the mechanical, checkable slice (did the required rounds happen, is there a receipt, is a waiver honest) plus, once re-grounding and Q-50 land, the context and doc-truth slices.
- Gap: nothing PROACTIVELY tells the agent the next action at the moment of action; the agent still has to remember to run `validate` and read the rules. This is a real but small gap, and it is exactly what Option 1 closes cheaply.

### Option 1: The advisory MVP (`agent-scaffold next`, read-and-remind only)

The smallest thing that captures most of the value. This is essentially the ALREADY-DECIDED `state-queries` `next`, with a thin advisory framing added, and NOTHING else:

- INPUT: the durable files only (plan TOML via `src/plan.rs`, round log via `src/metrics.rs`, the RESUME STATE extract). Stateless, recomputed each call, so it survives crashes and compaction.
- OUTPUT: a short report: the current step/increment and its convergence state (consecutive-clean count vs the required streak, from the log), the open-questions items awaiting the human, and a ONE-LINE "the rules say the next action is X" derived from the transition logic `workflow-invariants` already encodes. Optionally a periodic Project-Principles reminder line.
- Explicitly NOT: no per-unit FSM type hierarchy, no logic migration out of AGENTS.md, no authoritative driving (the agent still drives; the tool advises), no filled-in instruction-prompt templating, no AGENTS.md generation, no dependency scheduler.

The claim is that this captures the LARGE majority of the anti-drift value. Drift in this workflow is overwhelmingly "the agent forgot a rule applied here" (forgot to spawn a separate triager, forgot the second clean round for a risky artifact, forgot to write a decision receipt). An advisory line surfaced at the point of action ("this artifact is risk-classed risky, so it needs TWO consecutive clean rounds; you have 1") fixes exactly that class, because it re-injects the rule into the agent's working context precisely when it is needed. It does this without asserting a false determinism and without moving a single line of logic out of prose. The event-recording ergonomics (`record-round --outcome clean` appending the log so the agent does not hand-edit JSONL) can be added later as a small, separable convenience; it does not need the FSM either.

If drift is still measurable AFTER this ships and is used, that is the evidence that justifies going further. Absent that evidence, this may simply be sufficient.

### Option 2: Advisory driver with instruction prompts (the middle)

Option 1 plus emitting the filled-in next INSTRUCTION PROMPT (the role prompt with context spliced in), still advisory (the tool suggests, the human/orchestrator can ignore). This is the "advisory-first" tier the ask itself proposes as the adoption on-ramp. It adds prompt-templating machinery and a coupling between the tool and the `.agents/prompts/` files, but stops short of authoritative driving and logic migration.

- Marginal value over Option 1: saves the orchestrator the step of assembling the next role prompt by hand. Convenience, not correctness.
- Marginal cost: a templating layer, and a new coupling that must track every edit to the role prompts (a fresh drift surface between the templater and the prompt files).
- My read: this is where advisory-first should STOP unless Option 1 proves insufficient. It is worth considering only after Option 1 has shown that naming the next action is not enough and agents also fumble ASSEMBLING the prompt, which I doubt is a real failure mode.

### Option 3: The full authoritative driver (the proposal)

Everything: the per-unit FSM fleet, authoritative driving (agents run it to receive instructions), convergence/sequencing logic MIGRATED from AGENTS.md into Rust, the AGENTS.md workflow section GENERATED from the machine definition, and the `blocked_by` dependency scheduler. This is, by the ask's own words, "a MAJOR initiative, larger than `structured-skeleton`."

The rest of this document argues that Option 3's marginal value over Option 1 is small and its cost and risk are large, so it should be gated behind explicit evidence rather than built now.

## Failure modes of the full driver (named concretely)

### The straitjacket

AGENTS.md is saturated with JUDGMENT that resists encoding as deterministic transitions. It says "match the ceremony to the stakes," "collapse roles for a trivial change," classify an artifact as risky "when a defect in it would be costly or hard to reverse," scale the clean-round requirement "to the stakes," hold "a short debate" for a contested finding, and route a human interrupt by "a single bounded intake assessment." It supports at least four distinct ENTRY MODES (request-driven, Socratic, exploration, review) plus escalation, each re-using shared machinery in judgment-dependent ways. A Mealy machine that emits THE next instruction has to either (a) hard-code one path and force every exception through an escape hatch, in which case the machine is mostly escape hatches and the "determinism" is a fiction, or (b) try to encode the judgment and get it wrong. The ask concedes "ESCAPE HATCHES for exceptions and human overrides are mandatory or the tool becomes a cage." A workflow whose defining feature is "scale to the stakes" is close to the worst possible fit for a rigid driver, and a driver that is all overrides is worse than the advisory tool because it adds ceremony to reach the same override.

### Tool-vs-prose drift (a NEW drift surface to fight drift)

The proposal fights agent-memory drift by moving logic into Rust, then notices this creates prose-vs-code drift, then proposes to fix THAT by generating AGENTS.md from the machine definition. Follow the cost: you now need a generator that turns an FSM definition into correct, readable English workflow guidance. That is a hard, unproven artifact. The `render` closure the ask cites as precedent generates a plan VIEW by splicing verbatim prose sidecars; it does not synthesize English descriptions of control logic from a state table, which is a different and much harder problem. Two outcomes: either the generator is not built and you maintain logic in two places (the exact drift you set out to kill, now between code and docs instead of between docs and memory), or the generator IS built and it is a second major subsystem whose output quality is a fresh review burden every time the machine changes. Either way the driver does not eliminate a drift surface; it relocates it to a place that is harder to inspect.

### The control-vs-judgment boundary going wrong

The ask is right that the tool must own only CONTROL transitions and consume judgment as inputs. But apply that boundary honestly and see what is LEFT for the tool to own. The valuable, drift-prone decisions are the judgment ones: is this artifact risky (sets the clean-round bar), is this finding valid (sets clean-vs-new-valid), is this request trivial (sets the routing), has the human decided. The tool cannot make any of these; it can only bookkeep after a human/agent supplies them: count clean rounds, check DAG readiness, verify a receipt exists. That mechanical residue is PRECISELY what `validate --workflow` and `status` already compute. So the boundary, drawn correctly, leaves the driver thin, doing what two shipped commands already do; drawn incorrectly, it has the tool guessing judgments and being confidently wrong (mis-classifying risk, declaring a round clean the triager would not). Thin-but-correct converges on Option 1; thick-but-wrong is a liability.

### Over-engineering for the actual team

This is a solo/small-team dogfooding project (per the project memory: mid-dogfooding its own workflow). The codebase is already ~10.5k lines of Rust for the scaffold tool. A per-unit FSM fleet plus a `blocked_by` dependency SCHEDULER computing what may run in parallel is optimizing for concurrent multi-agent execution that a solo dogfood rarely exercises; it is machinery for a scale the project is not at (Principle 10, avoid premature optimization). The scheduler in particular solves "parallel steps," but the current workflow runs steps sequentially and the plan is authored by one orchestrator; the parallelism is hypothetical. Building and maintaining a scheduler for a DAG that is walked one node at a time is close to a textbook YAGNI.

### The cost of encoding a still-molten workflow in code

This is the strongest skeptical point. The workflow is NOT stable; it is under heavy active development. The plan's own question log shows the workflow being edited constantly and recently: Q-45/Q-46 (structured skeleton), Q-47/Q-48 (doc currency), and Q-51 itself, all dated within the last days. Encoding convergence and sequencing in Rust means every future workflow change (and there is a steady stream of them) now needs a code edit plus its review cycle, where today it is a prose edit. You would be pouring concrete around a design that is still being poured. Principle 17 favors the cleaner LONG-TERM architecture, but "long-term" presumes the thing has settled; while the workflow is molten, PROSE is the cleaner substrate precisely because it is cheap to change, and code-as-source-of-workflow-logic is premature by Principle 3 (ground in evidence) and Principle 10 (correctness and stability before optimization). Encode the machine only once the machine has stopped moving.

## Trade-offs against the numbered Project Principles

The principles cut both ways; I give each side honestly.

- Principle 3 (ground decisions in evidence) and Principle 6 (verify, don't trust): decisively FOR starting at the advisory MVP. The ask itself concedes adoption "should be EVIDENCE-FIRST ... start ADVISORY ... measure whether drift drops." Building Option 3 before Option 1 has produced that measurement inverts the principle. There is currently NO measured control-drift rate to justify the larger build.
- Principle 4 (small, reviewable changes): FOR Option 1. It is a bounded read command reusing existing parsers. Option 3 is "larger than structured-skeleton," the biggest initiative the project has taken, and hard to land in small reviewable pieces because the logic migration and the AGENTS.md generator are entangled.
- Principle 13 (make illegal states unrepresentable): the driver's BEST principled argument, and I concede it. Encoding valid transitions as types genuinely serves P13. But note the honest discount: `validate --workflow` already DETECTS every illegal control state post-hoc, so the driver's marginal P13 gain is "reject earlier" not "reject at all," and P13 is about representability, which the enforcement layer already approximates. It is a real but marginal, not transformative, gain.
- Principle 16 (one source of truth): DOUBLE-EDGED, and I think it actually cautions AGAINST Option 3 today. Right now AGENTS.md prose is the single source of workflow logic. Option 3 splits that into a Rust machine plus a generated prose view, which is one source ONLY IF the generator is perfect and always run; until then it is two sources, the opposite of P16. P16 supports the driver only AFTER the generation closure is built and proven, which is a reason to sequence, not to start.
- Principle 17 (prefer the cleaner long-term architecture over the smallest diff): the pull toward Option 3, and legitimate IF the workflow stabilizes. But "long-term cleaner" assumes a stable target; against a workflow still being redesigned week to week, the smaller diff that keeps the workflow editable in prose is the more defensible reading. P17 does not license building the concrete overpass before the road is surveyed.
- Principle 8 (no silent scope expansion): FOR keeping this exploration tight. The human explicitly queued Q-51, so it is not silent, but the exploration should resist the gravity of "since we are building a machine, also build the scheduler and the generator." Each of those is separately justifiable or not on its own evidence.
- Principle 10 (correctness before performance): FOR Option 1. The dependency scheduler is a performance/parallelism feature for a problem the project does not yet have.

Net: the principles that are about EVIDENCE, SMALL STEPS, and STABILITY (3, 4, 6, 10) point at the advisory MVP; the principles that are about CLEAN STRUCTURE (13, 16, 17) point at the full driver but each carries a "once it is stable / once the generator exists" precondition that is not met today.

## Subsumption: does the driver replace the re-grounding step and Q-50?

No. It COMPLEMENTS them; it does not subsume either, and treating it as a replacement would be a mistake. The three attack DIFFERENT drift axes:

- The driver attacks CONTROL drift: which step/round/action comes next.
- The re-grounding + provenance step attacks CONTEXT drift: at task entry, re-establishing WHAT this task is and WHY the plan decided what it decided, from durable artifacts. A machine that says "spawn the reviewer next" does not re-load the reasoning behind the plan; it tells you the move, not the meaning.
- Q-50 (doc-currency as core cross-cutting guidance) attacks DOC-TRUTH drift: keeping the documents the agent reads actually true. If anything the driver DEPENDS on Q-50: a driver that emits instruction prompts assembled from role-prompt files is only as correct as those files, so doc-currency is a prerequisite for the driver, not a casualty of it.

Recommendation on sequencing so we do not build overlapping things: do NOT block the re-grounding step or Q-50 on this exploration or on any Q-51 build. They are cheaper, independently valuable, and cover drift the driver never touches. Ship them on their own schedule. The only genuine overlap is the single ADVISORY read-and-remind surface: `next`/`status --resume` (state-queries), the re-grounding brief, and a Q-50 doc-currency check could all be delivered through the SAME advisory command rather than three separate tools, which is an argument for building state-queries first and letting re-grounding and Q-50 ride on it, NOT an argument for the full FSM. Concretely: the human directed queueing this exploration before folding Q-50 / re-grounding; my skeptical counsel is that the exploration should not gate them, because the exploration's likely outcome (build the advisory tier, measure) does not conflict with shipping the cheaper anti-drift items now.

## Recommendation

1. Build Option 1, the advisory `agent-scaffold next`, which is essentially the already-decided `state-queries` step (Q-28/Q-34) with a thin advisory-reminder framing. Do this FIRST and do not expand its scope. It reuses `src/plan.rs`, `src/metrics.rs`, and the `workflow-invariants` transition logic; it is a small, reviewable change (Principle 4).
2. Do NOT build the full driver (Option 3) now. Do not migrate convergence/sequencing logic out of AGENTS.md, do not generate AGENTS.md from a machine definition, and do not build the dependency scheduler, until the evidence gate below is met.
3. Un-gate the lighter anti-drift work: let the re-grounding + provenance step and Q-50 proceed on their own schedule rather than waiting on this design pass, since the driver neither subsumes nor conflicts with them, and Q-50 is in fact a prerequisite if the driver is ever built.
4. Instrument for the decision: while running under the advisory tool, record control-drift incidents (a required round skipped, an unreviewed merge, a wrong clean-count, a missing receipt caught only by `validate`). This is the evidence that would later justify or refute escalation, mirroring the receipt/waiver pilots the project already ran.

Reasoning, in one line: the project has already decided to build the cheap 80-percent solution and has not yet built it; building the expensive 100-percent solution before the cheap one has shipped and been measured violates the project's own evidence-first principle (3, 6) and its small-step principle (4), and encodes a workflow that is still being actively redesigned (against 10, and prematurely for 16/17).

## The YAGNI boundary: what NOT to build, and when

Do NOT build, now:

- Authoritative driving (agents running the tool to be TOLD what to do rather than to check themselves). Advisory only. The agent keeps the wheel.
- Migration of convergence/sequencing LOGIC from AGENTS.md prose into Rust. The prose stays the source of workflow logic while the workflow is still changing.
- Generation of the AGENTS.md workflow section from a machine definition. This is a hard, unproven generator and a second major subsystem; do not build it to fix a drift the advisory tool did not have.
- The `blocked_by` dependency SCHEDULER and the per-unit FSM FLEET. This is parallelism machinery for a scale (many concurrent units) the solo/small-team project does not currently reach (Principle 10).
- Instruction-prompt TEMPLATING (Option 2) unless Option 1 demonstrably fails because agents fumble assembling prompts, which is not an observed failure mode.

DO build, now: the advisory `next` (Option 1), scoped to resume and CI/check paths as Q-34 already decided, with an optional periodic Principle reminder, and optionally the separable `record-round` append convenience.

Escalate from the MVP to the full driver ONLY when ALL of these hold (the evidence gate):

- MEASURED DRIFT: the instrumentation shows control-drift incidents continuing at a material rate AFTER the advisory tool is in routine use, i.e. naming the next action was not enough. Absent this, stop at Option 1.
- WORKFLOW STABILITY: the AGENTS.md workflow definition has gone a meaningful stretch (multiple tasks / weeks) with no structural change, so encoding it in code is not pouring concrete around a moving design. Today it is changing weekly, so this is not met.
- TEAM/CONCURRENCY SCALE: multiple agents or humans are actually running units in parallel often enough that a scheduler and a per-unit FSM fleet earn their maintenance, rather than being built for a hypothetical.
- GENERATOR PROVEN: before any logic moves into Rust, the AGENTS.md-from-machine generation closure is demonstrated on a small slice and shown to produce correct, readable guidance, so Principle 16 is actually satisfied rather than nominally claimed.

Until every one of those is true, the advisory MVP is the right and sufficient stopping point, and the burden of proof is on the full driver to show, with data, that the cheaper tool left drift on the table.
