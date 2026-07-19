# Q-51 exploration: the tool-vs-prose single-source boundary

Lens: if `agent-scaffold` DRIVES the workflow loop (emits the next instruction from the current state), the workflow's control LOGIC moves from AGENTS.md prose into Rust. Where does the line between executable tool logic and human prose fall, and how does AGENTS.md relate to the state-machine definition afterward, judged against Principle 16 (one source) and Principle 20 (self-contained docs)?

This note deliberately does NOT settle the FSM/concurrency model, the I/O contract, or the advisory-first adoption path (sibling explorers own those). It settles only the source-of-truth boundary and the generation question, because those decide whether the driver deepens the structured-data direction or opens a fresh drift front.

## The question, sharpened

The driver's strongest property, per the Q-51 ask, is that "the tool would become the SOURCE of the workflow LOGIC (convergence/sequencing move from AGENTS.md prose into Rust)." That is also its biggest cost, and it carries a specific hazard the ask names: "the prose and code could themselves drift unless the AGENTS.md workflow section is GENERATED from the machine definition."

So the boundary question has three parts:
1. WHICH AGENTS.md content is a deterministic control rule (a code candidate) versus rationale/role/judgment guidance (stays prose).
2. HOW the two relate afterward: two hand-maintained sources (drift risk), one generated from the other (no drift, but can prose be generated?), or a hybrid spec-plus-sidecars that mirrors the plan.toml/render architecture just built.
3. The DRIFT CLOSURE: today drift is agent-vs-prose; the driver risks code-vs-prose. Which option actually closes both, and at what maintenance cost.

### Grounding evidence: the code-vs-prose duplication ALREADY exists

This is not a hypothetical future drift. The convergence constants are duplicated between prose and Rust TODAY:
- `src/metrics.rs:130` `RiskClass::required_streak()` returns `LowRisk => 1`, `Risky => 2`.
- AGENTS.md line 52 states the same in prose: "one for a trivial or low-risk artifact, two for a risky or high-blast-radius one."
- The total-round cap "default five" is stated in AGENTS.md lines 53 and 59; `validate --workflow` (W3) reads the same streak requirement via `class.required_streak()` at `src/workflow.rs:457`.

So `validate --workflow` already encodes the convergence bar in Rust while AGENTS.md restates it in prose. The "new" code-vs-prose drift the human worries about is partly already present; the render+validate initiatives introduced it. That reframes the decision: generation would close an EXISTING gap, not merely forestall a new one. It also means the marginal cost of the driver's boundary shift is smaller than it looks, because the first and most safety-relevant constants have already crossed into code.

## Where the line falls: control rules vs prose

AGENTS.md interleaves two kinds of content in the same sections. Classify each by a single test: is it a rule a deterministic program can EVALUATE from durable state (a count, a transition, a sequence, a path, a predicate), or is it reasoning/role guidance/format a human or agent must APPLY with judgment? The first is a code candidate; the second stays prose. Concretely:

### Deterministic control rules (candidates for the machine spec)

- Convergence counts and cap (AGENTS.md "Convergence", lines 49-53). The required consecutive-clean streak per risk class (1 low-risk, 2 risky), the total-round cap (default 5), the streak-reset-on-new-valid rule, and the counter-reset-on-resume-after-escalation rule. These are pure arithmetic over the ledger's round outcomes. Already half in code (`required_streak`, W3).
- The backstop gate (lines 54-55). "Before a dismissed high/critical finding counts toward a clean round, a second triager confirms": this is a REQUIRED-TRANSITION rule (a clean round with a high/critical dismissal is not reachable until a `dismissal_recheck` records `upheld`). The tool can refuse to advance the state; the human/agent still makes the re-check judgment.
- Phase sequencing and the stop predicate (lines 27-35). Plan -> plan-review-loop -> per-step (implement -> work-review-loop -> mark complete) -> acceptance. "Done when every Roadmap step is complete and acceptance confirms the Success Criteria" is a computable predicate over `[[step]].status` plus an acceptance record.
- Dependency readiness (the ORCHESTRATOR REFRAMING in the ask). What may run now is a topological function of the plan's `blocked_by` DAG plus each step's status. Pure graph computation.
- Isolation-tier RESOLUTION (lines 79-85, 98). Given the harness's capability flags (container? worktree? neither?), the resolved tier is a lookup down the fixed preference order. The tool cannot PERFORM isolation, but it can COMPUTE and emit the resolved tier as part of the instruction, exactly as the ask suggests.
- File and path CONVENTIONS (lines 61-65). The findings-file names (`<step>-<role>-<disambiguator>.md`, `<step>-triage.md`, `<step>-triage-recheck.md`), the exploration paths, the ledger path, the report path. These are string templates the tool can emit exactly, removing a transcription-error surface.
- Entry-mode dispatch (lines 42-47). Which machinery a kickoff/explore/review/socratic entry reuses is a fixed routing, though the CHOICE of mode is a human input.

### Rationale, roles, contracts, and judgment (stays prose)

- WHY each rule holds. "Fresh reviewers are sampled each round, so one clean round is weak evidence" (line 52); "the orchestrator is biased toward dismissing findings to converge, so letting it triage would let that bias decide" (line 22). This is Principle 19 (document the why) content; an FSM cannot express it and should not try.
- Role definitions and separation guidance (lines 17-25). The never-collapse-the-triager rule is a constraint the tool can ENFORCE (refuse an instruction that names the same agent as producer and triager), but the guidance on WHY and on matching ceremony to stakes is prose.
- The human-input contract (line 41). The presentation FORMAT for a decision (options, trade-offs, recommendation, Principle-judged reasoning) is a prose template for a human to read, explicitly "not a machine-parsed schema" in spirit; the tool can REMIND the agent to use it but not generate the reasoning.
- The file-safety disciplines (lines 71-78). "Format only your own files", "validate in scratch", "recover on interrupt": these are behaviours the agent must carry out, not states the tool computes. The tool can gate on a clean tree (a computable precondition) but cannot itself keep the tree clean.
- The judgment INPUTS the tool consumes but never makes: whether a finding is valid (triage), whether an artifact is risky (the risk classification that SETS the required streak), whether a request is trivial, what a human decides. The ask is explicit: the tool "must own only CONTROL transitions and consume agent/human JUDGMENT as inputs, never make the judgments itself."

The boundary is therefore clean and already latent in the codebase: the counted/sequenced/pathed rules are the machine spec; the reasoning, roles, contracts, and disciplines are prose that the generated control fragments sit inside.

## The design space: how AGENTS.md relates to the spec

### Option A: two hand-authored sources (AGENTS.md prose + a separate machine spec)

The tool gets its own state-machine definition (a Rust table, or a TOML the tool loads); AGENTS.md keeps its hand-written control prose unchanged.

- Principle 16: FAILS. The convergence constants, the cap, the sequencing, and the path conventions each live twice, maintained by hand in two places. This is the drift the whole initiative is meant to fight, reintroduced one layer down. It also entrenches the existing `required_streak`-vs-prose duplication rather than closing it.
- Principle 20: fine (prose stays self-contained), but at the cost of Principle 16.
- Drift closure: closes agent-vs-prose (the agent now runs the tool) but OPENS code-vs-prose as a standing hazard. Net: trades one drift for another.
- Cost to change the workflow: change it in two places every time, and nothing forces them to agree. Worst option on the very axis that motivates the project.

Rejected. It is the anti-pattern Principle 16 names.

### Option B: the machine spec is the single source; the whole AGENTS.md workflow section is generated from it

One spec drives the tool AND renders the entire "## Workflow" section of AGENTS.md.

- Principle 16: satisfied for the control rules. One source, tool and prose derived.
- Principle 20: FAILS at the edges. Most of the workflow section is rationale, role guidance, and contract prose (the "why", the trade-off reasoning, the ceremony-to-stakes judgment). An FSM spec cannot generate readable, self-contained explanations of WHY the triager is never collapsed or WHY one clean round is weak evidence. Forcing all of it through generation either strips the rationale (Principle 19 and 20 loss) or bloats the spec with free-text prose fields that are "generated" only in the trivial sense of being copied through, which is not single-sourcing, it is just relocating the prose into a less readable container.
- Drift closure: closes both drifts for the generated content, but only by over-scoping generation to prose that has no machine meaning.
- Cost: high authoring friction (every rationale tweak edits a spec file), and the spec becomes a general documentation format, scope creep beyond a control machine.

Rejected as stated. But its single-source instinct is right; it just draws the generation boundary in the wrong place (all of the section, rather than only the control fragments).

### Option C (recommended): hybrid, mirroring the plan.toml + sidecars + render architecture

Apply the EXACT pattern the project just built for plans to the process itself:
- A machine-readable workflow spec (call it `workflow.toml`, structurally parallel to `<task>.plan.toml`) holds ONLY the deterministic control data: the risk-class -> required-streak map, the total-round cap, the phase/transition graph, the isolation-tier preference order, the file-path templates, the entry-mode routing. This is the single source the DRIVER executes.
- Hand-authored prose sidecars hold the rationale, the role guidance, the human-input contract, and the file-safety disciplines, exactly as the plan's `.steps/` and `.questions/` sidecars hold opaque body prose.
- The same render closure just built projects a GENERATED AGENTS.md workflow section: small generated fragments state the control constants (the streak counts, the cap, the vocabulary, the path conventions) exactly as `vocabulary_section()` and `status_line()` already generate the status vocabulary and status distribution from code constants today (`src/plan/render.rs:351,410`), and the hand-authored sidecars supply the surrounding rationale verbatim. A `render --check` byte-compare guards the generated section against hand edits, precisely as it guards `<task>.md`.

- Principle 16: satisfied precisely. Each control constant lives once in `workflow.toml`; the tool executes it and the prose statement of it is generated from it. The `required_streak`/prose duplication that exists today is closed: the constant would live in the spec, `validate --workflow` and the driver both read it, and the AGENTS.md sentence stating it is rendered from it.
- Principle 20: satisfied. The rationale sidecars stay hand-authored and self-contained; generation touches only the mechanical constant statements, which render already proves it can do readably (the vocabulary fragment is a working precedent). The generated fragments are the WHAT; the sidecars keep the WHY (Principle 19).
- Principle 8 (structured data first, project for humans): this IS that principle applied to the process. The workflow is data (the control graph and constants), projected to a human view (the AGENTS.md section). It is the "third stage" the ask names, and it reuses the machinery of stage two rather than inventing a parallel one.
- Drift closure: closes BOTH drifts. Agent-vs-prose closes because the agent runs the tool instead of recalling the rule. Code-vs-prose closes because the prose statement of every control constant is GENERATED from the same spec the tool executes, so they cannot disagree (the byte-compare check fails CI if they do). The rationale prose has no executable counterpart, so it has nothing to drift AGAINST, it is single-sourced by construction.
- Cost: changing a control constant now means editing `workflow.toml` and re-rendering, not editing prose. This is the real price, and it is the SAME price the project already accepted for plans (to change the plan, edit the TOML and re-render). Generation MITIGATES the pain: the author still edits one human-readable source (the spec plus its sidecars) and the prose follows automatically, rather than editing Rust. Only adding a genuinely NEW KIND of control rule (a new transition type, not a new constant value) touches Rust, which is correct: a new rule kind is a code change, a new constant value is a data edit.

Option C is Option B with the generation boundary drawn at the control fragments instead of the whole section, which is exactly the line render already draws for the plan (generated Status line and vocabulary and principles; hand-authored sidecar bodies inlined verbatim).

## The determinism boundary (what the tool encodes vs what stays human)

The tool encodes only what is computable from durable state:
- Counts and thresholds: the consecutive-clean streak, the total-round count, the required streak per risk class, the cap.
- Transitions and gates: advance a loop only when its precondition holds (a clean round with a high/critical dismissal is unreachable until a re-check records `upheld`; a step is not `complete` until its loop converged; the never-same-agent-as-triager constraint).
- Sequences and predicates: the phase order, the stop predicate, the DAG readiness set.
- Derivations: the resolved isolation tier from capability flags, the exact findings/ledger/exploration/report paths from the naming convention.

It consumes, but never makes, every judgment:
- The triage verdict (valid/dismissed) is an INPUT, recorded via an event subcommand.
- The risk classification that SETS the required streak is an INPUT the orchestrator supplies at loop-open; the tool then computes the streak from it, it does not classify.
- The human decision, the trivial/non-trivial intake call, and whether a finding's re-raise carries new evidence are all INPUTS.

The prose must therefore still fully cover the judgment side: how a triager weighs a finding, what makes an artifact risky, how the human-input contract is presented, how the file-safety disciplines are carried out. The tool tells the agent WHICH role to run next and hands it the filled prompt; the prose (and the role prompt) tells the agent HOW to do that role's judgment. If the tool ever tried to encode a judgment (decide a finding is invalid, classify risk itself), it would "hallucinate decisions" the ask warns against. The boundary is exactly the control/judgment line the ask draws, and Option C keeps the judgment content in prose where an agent and human can read and apply it.

## Recommendation

Adopt Option C: a machine-readable `workflow.toml` control spec plus hand-authored prose sidecars, with the AGENTS.md workflow section's control fragments GENERATED by the same render closure and guarded by `render --check`. Draw the boundary at the control/judgment line: the counted, sequenced, pathed, and derived rules become spec data the driver executes and render projects; the rationale, roles, human-input contract, and file-safety disciplines stay hand-authored prose sidecars. This closes the existing `required_streak`-vs-prose duplication, prevents the new code-vs-prose drift by construction, and reuses the plan.toml/render architecture rather than inventing a parallel one (Principles 16, 8, 20, 19).

Sequence it evidence-first (Principle 6), matching the receipt/waiver pilots: START by generating only the smallest, highest-value fragment, the convergence constants (the streak-per-risk-class map and the cap), from a single source both `validate --workflow` and the (advisory) driver read. That one step closes a duplication that already exists, is independently useful even if the full driver is never built, and proves the generation approach on real content before the boundary is widened to sequencing, paths, and tiers. Only deepen to authoritative driving once the advisory pilot shows drift actually drops.

## YAGNI boundary (what NOT to build)

- Do NOT generate the whole AGENTS.md workflow section (Option B). Generate only the control-constant fragments; keep the rationale, role, contract, and discipline prose hand-authored in sidecars. Rationale has no machine meaning and generating it loses the "why".
- Do NOT build a general workflow DSL that can express arbitrary processes. The spec encodes THIS workflow's fixed control rules (streak counts, cap, phase graph, path templates, tier order), not a Turing-complete process language.
- Do NOT try to encode any judgment in the spec: no finding-validity rule, no risk-classifier, no trivial/non-trivial decider. These are INPUTS; the moment the tool computes one it either straitjackets or hallucinates.
- Do NOT keep two hand-maintained sources (Option A) as a "simpler" interim. It reintroduces the exact drift the initiative fights and entrenches the existing duplication.
- Do NOT reconcile `[meta].render_sha256` or add a second guard mechanism; the byte-compare `render --check` already guards generated content, and the workflow section reuses it unchanged.
- Do NOT couple this to the FSM/concurrency model, the event-subcommand I/O contract, or the isolation integration; those are separate explorer lenses. This note commits only to the SOURCE boundary and the generation approach, which stand whatever concurrency model the driver adopts.
- Do NOT migrate all control constants at once. Start with the convergence constants (already duplicated, already safety-relevant), measure, then widen. A big-bang spec extraction is the opposite of the evidence-first adoption the ask mandates.
