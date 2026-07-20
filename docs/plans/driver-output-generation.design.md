# Driver output generation: restate at the point of action without drift

Design pass for the governing decision behind the workflow-driver's output (Q-51, the Mealy-style state machine whose Stage 1 shipped the advisory `agent-scaffold next`). This document decides HOW the driver should emit its instructions and reminders so they are effective at the point of action (inline content, not a bare pointer) WITHOUT drifting from the canonical docs. It is a design pass only: it recommends and flags decisions for the human; it implements nothing.

## The tension

Today the driver POINTS. Its per-state reminders cite Project Principles by number ("Principle 6: verify by running the tests and checks before marking the step complete", `src/next.rs:286-289`), and its isolation reminder points at the policy rather than restating it ("Resolve the isolation tier per the AGENTS.md tier policy before spawning the writer", `src/next.rs:58-59`). Live output confirms this: running `next` on this repo emits `Principle 6` / `Principle 16` citations and `isolation: unknown`.

Pointing is weak because the acting agent may not re-read the pointed-to doc, so the guidance is not present where the decision is made. The obvious fix, restating the guidance inline, risks DRIFT: a hand-copied restatement in the driver can fall out of step with the canonical AGENTS.md / plan text as either side is edited, and nothing catches it.

## The resolving principle: generate, do not hand-copy

The reconciliation is that inline restatement should be GENERATION (projection from a single source), not hand-copying. Generated inline content is present at the point of action AND provably cannot drift, because a drift-guard regenerates it and byte-compares against the committed output; an out-of-step copy fails the guard.

This project already runs this pattern twice, so the mechanism is proven here, not proposed from scratch:

- Stage 0b, the workflow-control fragment. `WorkflowSpec::control_fragment()` (`src/workflow_spec.rs:116-124`) renders a plain-prose statement of the convergence constants from the one `WorkflowSpec`. `build_assets` substitutes it into the `{{workflow_control}}` slot of the scaffolded AGENTS.md (`src/main.rs:257`; the slot is `pack/AGENTS.md:51`). Two drift-guard tests pin it: `the_control_fragment_states_the_spec_constants` (`src/workflow_spec.rs:217-228`) pins the fragment to a spec-derived string, and `the_committed_scaffold_carries_the_generated_fragment` (`src/workflow_spec.rs:230-248`) byte-asserts the committed AGENTS.md contains exactly the generated fragment. So the AGENTS.md prose and the arithmetic the tool runs both read the one spec and cannot diverge; a hand edit of either the constant or the committed fragment fails a test. The surrounding rationale (WHY two clean rounds, WHY a cap) stays hand-authored and refers to the generated values rather than restating them (`src/workflow_spec.rs:108-110`).
- The plan render. `render` generates the whole committed `<task>.md` from the structured `plan.toml` plus its prose sidecars, and `render --check` (`check_render`, `src/plan/render.rs:257`) is a byte-for-byte compare of a fresh in-memory render against the committed view (`src/plan/render.rs:21`). The generated view is never hand-edited; to change it you edit the source and re-render.

Both are instances of the same family: one structured source, many projected views, each pinned by a regenerate-and-compare guard. "Restate via generation" gives the driver inline content with the same guarantee. This aligns with one-source-of-truth (AGENTS.md-committed Principle 16) and structured-data-first (plan Principle 8): keep one authoritative source and derive the rest, rather than hand-authoring prose that is also an input.

## A grounding finding: "Principle N" is a doubly-unstable citation

Before categorizing, one finding sharpens every recommendation, because it shows the current pointer style is not merely weak but already latently incoherent.

The driver cites principles by NUMBER, and the number is unstable in two independent ways:

1. The number is itself GENERATED. The AGENTS.md `## Principles` list is the `{{principles}}` slot, rendered by `render_principles` which enumerates the SELECTED principles in order (`src/pack.rs:185-195`, numbering by `i + 1`). Add, remove, or reorder a principle (a selection or `default_order` change in `pack/principles.toml`) and the list renumbers, while the driver's hardcoded `Principle 6` string does not follow. The citation can silently come to point at the wrong principle. This is the same class of drift the Stage-0b guard exists to prevent, but here there is no guard.
2. There are TWO numbered principle lists, and they disagree. The driver's reminders are keyed to the AGENTS.md-committed generic list (where 5 = independent review, 6 = verify, 7 = cite sources, 12 = fail fast and loudly, 16 = one source of truth; see the committed root `AGENTS.md` Principles section). But the human-input contract judges decisions against "the plan's numbered Project Principles" (`pack/AGENTS.md:41`), which are the plan.toml `[[principle]]` entries, a DIFFERENT list. For this very repo the plan's Principle 5 is "Make illegal states unrepresentable" and Principle 6 is "Ground decisions in evidence" (`docs/plans/agent-scaffold.plan.toml`, the `[[principle]]` block from line 1073), whereas the driver's "Principle 5: the reviewer is independent" (`src/next.rs:275`) refers to the AGENTS.md list's 5. A human who reads the driver's "Principle 5", then opens the plan to check it, lands on the wrong principle.

So "cite Principle N" is fragile at its root: it is a hand-authored paraphrase (drift category 2, below) dressed as a numeric quotation of a generated list, and it references the list the contract does NOT name. Projecting TEXT sidesteps the numbering entirely. This finding is the concrete evidence (evidence-grounded, AGENTS.md Principle 3 / plan Principle 6) that motivates the category-2 recommendation. Note that because two numberings exist, this document judges its own trade-offs against principles by NAME (one-source-of-truth, evidence, document-why, least-authority, minimal), giving the AGENTS.md-committed number only as a locator.

## The driver's outputs, categorized

Each field the driver emits (`Instruction` in `src/next.rs:134-146`, rendered at `src/next.rs:940-969`) falls into one of four categories by its relationship to a canonical source.

### Category 1: content the driver ORIGINATES (no canonical source; cannot drift)

The driver's contextual next-action is the state-specific application of the workflow to the current records. It has no canonical duplicate anywhere, so it cannot drift, and it should be inlined freely.

Members: `next_action` (`src/next.rs:233-247`), `filled_prompt_summary` (`src/next.rs:798-844`), the derived `state`/`valid_transitions`, the `context` path slots built from conventions (`src/next.rs:764-795`), and the resolved facts (streak, required, rounds, cap). The path slots are filled from existing path conventions, not restated from a doc, so they are originated projections of a convention, not copies of prose.

The confirmation the design asks for holds: this content is genuinely originated. "step X converged (streak 2/2); mark the step complete, re-render, and commit" is not a copy of any canonical sentence; it is the machine's application of the transition table to these records. Inline it, keep it terse.

One caveat the finding above raises: some current reminders are ALSO originated content but mislabelled as quotations. "Principle 5: the reviewer is independent; a writer never reviews its own work" is really the driver's own imperative for the awaiting-review state. Its drift risk comes ENTIRELY from the "Principle 5:" prefix, which claims to quote a numbered list it does not track. Stripping the false numeric citation (or reducing it to a non-load-bearing name) turns a mislabelled category-2 paraphrase back into honest category-1 originated guidance. See D-a.

Recommendation: originate-inline. Keep it. This is where the driver's point-of-action value already lives; it does not need a source or a guard because there is nothing for it to diverge from.

### Category 2: content that DUPLICATES a canonical STRUCTURED source (project it drift-free)

Some driver output restates a value or text that already lives in a structured source the driver can read at runtime. It should PROJECT the actual value from that source rather than restate a copy.

- Control constants. `round_cap` and `required_streak` already come from `WorkflowSpec` (`src/next.rs:570`, `616`, `619`), the same source the Stage-0b fragment and the arithmetic read. This is already a correct category-2 projection; keep it.
- Project Principle text. The plan source already parses `[[principle]]` into `{ n, name, text }` (`src/plan/source.rs:66-67`, `379-388`), and `run_next` already parses the plan.toml (`src/main.rs:1112`). So the driver CAN project the authoritative principle name and text by reading the parsed principles, instead of citing a number. Where a reminder wants to invoke a Project Principle, it should project "<name>: <text>" from the plan's `[[principle]]`, self-contained at the point of action, rather than "Principle N".

The cost of projecting principle text is output size: the plan's principle texts run one to several sentences each (`src/plan/source.rs:387`; e.g. plan Principle 8's text is a full paragraph). Emitting the full text of every reminder principle on every `next` call would bloat the output and cut against the driver's token-efficiency rationale (its reminders are deliberately terse control pointers, `src/next.rs:262-264`). The gain is effectiveness and coherence: the reminder is self-contained and cannot mis-reference. The resolution is selective projection, not blanket projection (see D-a and the YAGNI boundary): project text only where the reminder's whole purpose is to invoke a specific Project Principle for the human (the escalate / human-input-contract reminder is the clearest case), and let the workflow-phase reminders be honest category-1 originated guidance without a numeric citation.

Recommendation: project-from-structured-source, selectively. Replace numeric citations with either (a) the driver's own originated imperative (category 1, no citation) for workflow-phase guidance, or (b) a projected `name`/`text` from the plan's `[[principle]]` where the reminder exists specifically to put a Project Principle in front of the human.

### Category 3: content that duplicates a canonical PROSE rule not yet structured

Some guidance the driver should restate lives only as canonical PROSE, with no structured source the driver can project at runtime. The writer-isolation tier policy is the motivating case: it lives as prose in `pack/AGENTS.md:83-89` (the capability-tiered tier order) and `:91-93` (the worktree lifecycle), plus the `isolation-guidance.md` module. The human-input contract (`pack/AGENTS.md:41`) and most role-prompt content are the same shape.

To inline such a rule drift-free the driver needs a single generatable source. Two routes:

- Route A, structure the rule into data. Model the tier order as typed data (as `WorkflowSpec` did for the constants) and render both AGENTS.md's tier list and the driver's reminder from it. Clean for genuinely enumerable rules, but the isolation policy carries prose-rich lifecycle detail (rebase-before-review, merge-back, read-only exemption) that resists full structuring; structuring only the tier ORDER while the lifecycle stays prose splits the rule across two sources and reintroduces drift between them.
- Route B, the Stage-0b generated-fragment pattern. Author the canonical snippet ONCE as a fragment, substitute it into an AGENTS.md render slot AND emit it as the driver's reminder, and byte-guard both against the one fragment. This is exactly `control_fragment` plus its two guards, applied to a prose rule instead of to constants. It needs no full structuring of the rule; it needs only that the shared inline snippet be short and self-contained.

Feasibility for the motivating case is good via Route B: the isolation reminder the driver wants to emit is short (the tier preference order plus the standing directive that writers are always isolated and read-only agents need none), which is a natural fragment. Feasibility GENERALLY is lower: every prose rule turned into a shared fragment adds a render slot, an embedded constant, and a byte-guard test, and bloats the driver output; do this only for rules that are both load-bearing at a specific driver state AND stable enough to be worth a guard (see the YAGNI boundary).

Recommendation for the motivating case: generated-fragment (Route B). Recommendation generally: default to keep-pointer, and promote a prose rule to a generated fragment only when a specific driver state must restate it at the point of action.

### Category 4: role prompts (keep the pointer)

The driver emits `role` plus `prompt_path` (`.agents/prompts/<role>.md`, `src/next.rs:748-749`). The role prompts are large prose (orchestrator.md is ~11.5 KB; reviewer/planner/implementer 1 to 2.4 KB each). Inlining a full prompt on every `next` call would dominate the output for no point-of-action gain (the orchestrator opens the prompt to run the role anyway). Generating a summary would create a second, drift-prone paraphrase of the prompt for no clear benefit.

Recommendation: keep-pointer. The prompt path is a precise, stable handle to content the acting role loads in full; a pointer is the right tool here. This is the one category where pointing wins, and it wins for the reason the design should respect: the pointed-to content is loaded in full by its consumer as a matter of course, so the "may not re-read" failure mode does not apply.

## The motivating case worked concretely: writer-isolation guidance

Goal: the driver should emit the writer-isolation guidance drift-free at the point where it tells the orchestrator to spawn a writer, combining the RESOLVED tier (a projected fact) with the POLICY (a generated fragment shared with AGENTS.md).

Mechanism:

1. Project the resolved tier (category 1/2 fact). The driver already echoes `isolation_tier` (`src/next.rs:727`, `771`). Keep echoing it; it is the concrete, session-specific fact ("worktree" / "container" / "file-safety" / "unknown").
2. Emit a generated isolation-policy fragment (category 3, Route B). Author one canonical `isolation_policy` fragment: the tier preference order (container, else worktree, else file-safety) and the standing directive (writer work always runs isolated where the harness allows; read-only agents need none). Substitute it into an AGENTS.md render slot (a new `{{isolation_policy}}` alongside `{{workflow_control}}`, or a fragment factored out of the existing prose at `pack/AGENTS.md:83-89`), and emit the SAME fragment as the driver's writer-state reminder. Pin both with a byte-guard test in the style of `the_committed_scaffold_carries_the_generated_fragment` (`src/workflow_spec.rs:230-248`). The AGENTS.md tier list and the driver reminder then cannot drift, because both are the one fragment.
3. Fire it on actual WRITER states only. This exposes a real bug the design must fix before an always-on writer-isolation reminder is added: `spawns_writer` (`src/next.rs:252-260`) includes `AwaitingFirstReview` and `AwaitingReviewers`, but those states spawn REVIEWERS (`src/next.rs:204-205`), which are read-only and need no isolation (`pack/AGENTS.md:91`; reviewer.md writes only findings files to the main repo). The orchestrator prompt already draws the correct line ("When you spawn a writer (the planner or the implementer)... Reviewers and the triager are read-only and need no isolation", `.agents/prompts/orchestrator.md:11`). An always-on isolation reminder keyed off the current `spawns_writer` would wrongly attach to reviewer spawns and contradict the policy. The classification must be narrowed to the true writer states: `ReadyToPlan` (planner) and `AwaitingFixes` (implementer).

Result: at a writer state, `next` emits the projected resolved tier plus the generated policy fragment, both at the point of action, neither able to drift. The tier reminder stops being conditional on `tier == "unknown"` (the always-on requirement) and stops being a bare pointer.

Composition with the separately-planned AGENTS.md planner-isolation clarification: the planner is a writer (it edits `plan.toml` and the sidecars, `.agents/prompts/planner.md:5`), but the worktree-lifecycle prose is written around the implementer editing product code while "the orchestrator authors the plan and the ledger on main" (`pack/AGENTS.md:91`), leaving the planner's isolation ambiguous. That clarification is a change to the CANONICAL policy. Under this design it is authored ONCE in the shared `isolation_policy` fragment source, and the generation path propagates it into both AGENTS.md and the driver's reminder automatically; the byte-guard then holds them together. This is the concrete payoff of Route B for this bundle: the clarification cannot land in AGENTS.md and be forgotten in the driver, or vice versa.

## The principle-text-projection case worked concretely

Where a reminder exists to put a Project Principle in front of the human (the human-input contract at escalate is the clearest: it already says "judged against the numbered Project Principles", `src/next.rs:290-293`), the driver projects from the plan's parsed `[[principle]]` rather than citing a number.

Mechanism: thread the parsed principles into `NextInputs` (the plan is already parsed at `src/main.rs:1112`; today only its steps flow into `next` via `steps_from_toml`). Add a lookup that, given a principle `n`, returns its `name` and `text` from the plan's `[[principle]]`. The reminder builder (`base_reminders` / `build_instruction`, `src/next.rs:265-302`, `736-754`) then emits, for a projected principle, "<name>: <text>" drawn live from the plan source. Because the source is the same plan.toml the render and the contract read, the projected text cannot drift from the plan; and because it is projected by name/text, it is immune to the renumbering finding above.

Edge case to handle explicitly (fail-loud, AGENTS.md-committed Principle 12): the driver's workflow-phase reminders reference GENERIC workflow principles (independent review, cite sources) that a given project's plan may not carry at a fixed number, or at all. So principle-text projection is only safe for reminders that invoke a principle KNOWN to be in the plan, or it must degrade gracefully when the referenced principle is absent (emit the driver's originated imperative alone, not a dangling "Principle N"). This is the reason the category-2 recommendation is selective: project text for the human-input-contract case; keep the workflow-phase reminders as originated category-1 guidance.

## Staging: the writer-isolation-hardening bundle under this design

The near-term bundle is three fixes. Each aligns with a chosen emission strategy; later driver stages are deferred.

1. AGENTS.md planner-isolation clarification (category 3, Route B source edit). Author the clarification in the shared `isolation_policy` fragment source (that planners are writers and are isolated under the same tier order, resolving the `pack/AGENTS.md:91` ambiguity). Generation carries it into both AGENTS.md and the driver reminder; the byte-guard pins them. This is the canonical-source edit the other two fixes derive from.
2. An always-on writer-isolation reminder in `next` (category 3 projection + the category-1 tier fact). Emit the generated `isolation_policy` fragment plus the projected resolved tier at the true writer states, unconditionally (drop the `tier == "unknown"` gate). Requires narrowing `spawns_writer` to `ReadyToPlan` and `AwaitingFixes` first (the reviewer-state bug above), so the always-on reminder does not attach to read-only reviewer spawns.
3. A required orchestrator-prompt writer-spawn preamble (category 4 canonical content; not driver output, but the same generation discipline). The orchestrator prompt already points at the rule (`.agents/prompts/orchestrator.md:11`). If the preamble should be a checklist restated at the spawn point rather than a pointer, and it must not drift from the isolation policy, then it should include the SAME generated `isolation_policy` fragment (the prompts are pack assets rendered through the same substitution path as AGENTS.md, `src/main.rs:build_assets`), not a hand-copied checklist. If instead a pointer is acceptable there (the orchestrator loads its own prompt in full, so category-4 reasoning applies), keep the pointer and rely on fixes 1 and 2. This is D-b's scope for the prompt.

Deferred to later driver stages, explicitly out of scope for this bundle:

- Stage 2 (the typed `src/driver/` multi-loop FSM engine, the reconstructor that would refine the current single-loop projection). The generation strategy here is forward-compatible with it: Stage 2 emits the same categories of output and reuses the same fragment sources.
- Stage 3+ enforcement (machine-gating on the isolation tier, the round cap, or the backstop). All isolation output stays ADVISORY in this bundle, consistent with `next` being read-only and stateless (`src/next.rs:1-8`) and with the constants being carried-but-not-enforced (`src/workflow_spec.rs:84-98`). Generating a reminder is not enforcing a gate.

## Design decisions for the human

Each decision gives the options, their trade-offs, a recommendation, and reasoning judged against the principles by name (with the AGENTS.md-committed number as a locator), so the orchestrator can relay them.

### D-a: Do reminders project `[[principle]]` text, or keep numeric citations?

- Option 1: keep "Principle N" numeric citations as today.
- Option 2: strip numeric citations; make workflow-phase reminders honest originated imperatives (category 1), and project `name`/`text` from the plan's `[[principle]]` ONLY for the reminder that exists to invoke a Project Principle for the human (the escalate / human-input-contract case).
- Option 3: project full `name`/`text` for EVERY reminder principle.

Trade-offs: Option 1 is smallest but is doubly-unstable (the renumbering finding) and already mis-references the list the contract names; it fails one-source-of-truth and self-contained-documentation. Option 3 is maximally self-contained but bloats every `next` call with paragraph-length principle texts, cutting against the driver's terse-control-pointer rationale (`src/next.rs:262-264`) and minimal-by-default (plan Principle 2). Option 2 removes the fragile citation from the many terse reminders (no cost, they become honest originated guidance) and projects real text only where it is load-bearing for a human decision (bounded cost).

Recommendation: Option 2. It resolves the coherence bug (no dangling/mis-referenced numbers), honors one-source-of-truth (AGENTS.md Principle 16) and self-contained documentation for the case that matters, and respects token-efficiency and minimal-by-default by not projecting text everywhere.

### D-b: Generate the isolation-policy fragment inline, or keep a concrete pointer?

- Option 1: keep a pointer (as `TIER_REMINDER` does today, `src/next.rs:58-59`).
- Option 2: generate a shared `isolation_policy` fragment (Route B) into AGENTS.md and the driver reminder, byte-guarded, and emit it inline at writer states.

Trade-offs: Option 1 is zero new machinery but is exactly the weak-pointer failure the human raised (the agent may not re-read AGENTS.md at the spawn point). Option 2 costs one render slot, one embedded constant, and one byte-guard test, and adds a few lines to writer-state output, in exchange for point-of-action content that provably cannot drift, and it makes the planner-isolation clarification (fix 1) propagate automatically to both consumers.

Recommendation: Option 2 for the driver reminder and for AGENTS.md. This is the motivating case; it is the direct application of the proven Stage-0b pattern (evidence-grounded, AGENTS.md Principle 3), and one-source-of-truth is the whole point. For the orchestrator-prompt preamble specifically, see D-c.

### D-c: How far to generalize generation to other prose rules and to role prompts now?

- Option 1: generate only the isolation-policy fragment now; keep pointers for the human-input contract, the other prose rules, and all role prompts; revisit case by case.
- Option 2: also generate a human-input-contract fragment and a per-role prompt-summary fragment now, for uniform inline output.

Trade-offs: Option 2 buys uniformity but pays a real maintenance and size cost (a slot, a constant, and a guard per rule) and, for role prompts, creates a second paraphrase of content the consumer already loads in full (category 4), which cuts against minimal-by-default (plan Principle 2) and adds drift surface for no point-of-action gain. Option 1 promotes a prose rule to a fragment only when a specific driver state must restate it, which is the isolation case and, arguably, the escalate-state human-input contract.

Recommendation: Option 1. Generate the isolation fragment now (it is the motivating, state-local case); keep pointers elsewhere, and keep role prompts as pointers (category 4). For the orchestrator-prompt writer-spawn preamble (bundle fix 3), reuse the isolation fragment IF the preamble is to be a restated checklist; otherwise keep the existing pointer (`.agents/prompts/orchestrator.md:11`). Judged against minimal-by-default and one-source-of-truth: generate where a driver state is the point of action and the rule is stable; point where the consumer loads the content in full anyway.

### D-d: Fix `spawns_writer` to exclude reviewer states?

- Option 1: narrow `spawns_writer` to `ReadyToPlan` and `AwaitingFixes` (the true writer states), matching the orchestrator prompt's "planner or implementer" line and the read-only-agents-need-no-isolation policy.
- Option 2: leave it as is.

Trade-offs: this is a genuine bug (`src/next.rs:252-260` includes the two reviewer states, contradicting `pack/AGENTS.md:91` and `.agents/prompts/orchestrator.md:11`). Today it only over-appends the conditional tier pointer to reviewer instructions; under the always-on reminder (fix 2) it would wrongly attach the whole isolation policy to read-only reviewer spawns. Leaving it would emit a category-error at the point of action.

Recommendation: Option 1, and it is a prerequisite for bundle fix 2. Judged against make-the-common-case-easy / make-illegal-states-unrepresentable (AGENTS.md Principles 18/13 family) and correctness: the isolation output must key off the roles that actually edit files. This is small and reversible; the orchestrator may treat it as decided unless the human wants to weigh it.

### D-e: Where does the driver project principle text FROM when the reminder references a generic workflow principle absent from the plan?

- Option 1: project only from the plan's `[[principle]]`, and degrade to the driver's originated imperative alone when the referenced principle is absent (never emit a dangling number).
- Option 2: also carry the pack's generic principle list in the tool so the driver can project generic workflow-principle text even when the plan omits it.

Trade-offs: Option 2 gives the driver a stable generic source, but the tool cannot know a downstream project's AGENTS.md selection/numbering at `next` time (that numbering is generated per-project at scaffold time), so a carried generic list would be a THIRD principle list that can itself disagree with the project's AGENTS.md, reintroducing the multi-list incoherence this design is removing. Option 1 keeps a single per-project source (the plan) and fails safe.

Recommendation: Option 1. Judged against one-source-of-truth and fail-loud (AGENTS.md Principles 16/12): one authoritative per-project principle source (the plan), and explicit graceful degradation rather than a dangling reference. This reinforces D-a's "selective projection".

## YAGNI boundary (what NOT to build)

- Do not over-generate driver-originated content. The next-action, the summary, the transitions, and the path slots (category 1) have no canonical source; do not invent a "source" to generate them from, and do not add guards for content that cannot drift.
- Do not structure all of AGENTS.md now. Promote a prose rule to a generated fragment only when a specific driver state must restate it at the point of action (the isolation policy qualifies; most of AGENTS.md does not). Route A (full structuring) is not warranted for the isolation lifecycle prose.
- Do not project full principle text into every reminder. Selective projection only (D-a Option 2); the terse workflow-phase reminders stay terse originated guidance.
- Do not inline or summarize role prompts. Keep the `prompt_path` pointer (category 4); the consumer loads the full prompt anyway.
- Do not build enforcement. This bundle keeps all isolation output advisory; machine-gating on the tier is Stage 3+ and out of scope. Generating a reminder is not enforcing a gate.
- Do not add a third principle list in the tool (D-e Option 2). One per-project source (the plan) only.
