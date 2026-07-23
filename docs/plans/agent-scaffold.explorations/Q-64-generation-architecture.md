# Q-64 explorer note: the generation architecture lens

How far can `AGENTS.md` be RENDERED from a single structured workflow source, and what MUST stay hand-authored? This note answers only that lens. It grounds every claim in what the code does today, judges each recommendation against the plan's eight `[[principle]]` by name, and ends with an identity read for Q-65.

Bottom line up front: keep a hand-authored `AGENTS.md` TEMPLATE, but grow the set of GENERATED slots inside it from today's five to cover every structured FACT the prose currently hand-copies (the roles and their prompt paths, the findings/exploration file-naming conventions, the phase/transition machine, the human-input-contract rule text, the status vocabulary), each single-sourced with the code that consumes it and byte-guarded, and add a whole-file `render --check`-style guard. Do NOT whole-file-generate `AGENTS.md`: ~95% of it is authored judgment prose, and forcing that through a structured source buys drift-safety it does not need while paying Principle-2 cost. This is direction A (demote `AGENTS.md` to a generated projection), refined to say the projection is slot-wise, not whole-file, and staged.

## 1. Inventory: the current sources and their generation status

There are TWO distinct generation mechanisms in the tree today, and it matters for the architecture that they are separate:

- Mechanism A, PACK SLOT SUBSTITUTION. `pack/AGENTS.md` is a hand-authored template shipped verbatim as an `[[asset]]` (`pack/pack.toml:28-29` source/dest `AGENTS.md`, and again at `:99-100` to `.agents/AGENTS.reference.md`). At scaffold time `build_assets` (`src/main.rs:237-278`) computes five values and `manifest::substitute` (`src/manifest.rs:316-331`) does a dumb `{{key}}` -> value find-replace. It is not strict, not pure, and has no byte-for-byte re-render check of its own.
- Mechanism B, THE RENDER ENGINE (`src/plan/render.rs`). A strict, pure `(<task>.plan.toml, sidecars) -> <task>.md` projection with a byte-for-byte `render --check` (`check_render`, `:257-280`) that is the golden guard. This engine drives the PLAN, not `AGENTS.md`.

`pack/AGENTS.md` is 119 long-line paragraphs, four `##` headings (Agent guidance / Getting started / Workflow / Principles), and exactly five substitution slots: `{{workflow_control}}` (`:51`), `{{isolation_policy}}` (`:91`), `{{principles}}` (`:114`), `{{instrument}}` (`:116`), `{{modules}}` (`:118`). Everything else is authored prose.

Classifying each substantive section into (a) generatable from a structured source, (b) must stay authored prose, (c) hybrid (structure + an authored rationale slot):

| Section (pack/AGENTS.md) | Class | Structured fact it hand-copies (drift surface) |
| --- | --- | --- |
| Header + harness-agnostic bootstrap (`:1-3`) | b | none; this is the irreducible bootstrap floor |
| Getting started / user-prompts (`:5-11`) | c | the set of `.agents/user-prompts/*.md` (kickoff/explore/review/pause/resume) is an enumerable asset set |
| Workflow intro + role-separation (`:13-15`) | b | authored rationale; embeds the Q-63 triager-on-findings rule |
| Roles and their prompts list (`:17-25`) | c | the role -> `.agents/prompts/<role>.md` map, ALSO in `next.rs` `prompt_path` (`src/next.rs:825`) and the pack `prompts/` set: three copies |
| Phases 1-5 (`:27-35`) | a | the state/transition/guard machine; the driver models a subset as `LoopState` (`src/next.rs`); Q-51's widened `workflow.toml` target |
| Human requests / intake (`:37-39`) | b | authored judgment prose |
| Human-input contract (`:41`) | c -> a | the rule text; Q-60 already DECIDED to single-source it and render it into both here and Preflight |
| Socratic / exploration / review entry modes (`:43-47`) | b | authored prose |
| Convergence + `{{workflow_control}}` (`:49-59`) | c (done) | the constants ARE generated (`control_fragment`); the WHY-rationale is authored |
| Backstop (`:59`) | c | backstop severity is a generated constant; the mechanism prose is authored |
| Tracking progress / ledger (`:61-63`) | b | authored |
| Design explorations / findings files / review reports (`:65-69`) | c | the file-naming conventions (`<step>-<role>-<disambiguator>.md`, `<step>-triage.md`, `<step>-triage-recheck.md`) are ALSO `next.rs` format strings (`src/next.rs:848-852`): a duplicate |
| Checkpoints (`:71-73`) | b | authored |
| File safety / durability rules (`:75-81`) | b | authored behavioral rules |
| Writer isolation tier order (`:83-89`) | c | the tier order is authored; the `{{isolation_policy}}` fragment REFERENCES it rather than restating (see `src/isolation_policy.rs:12-19`) |
| `{{isolation_policy}}` (`:91`) | a (done) | generated single-source fragment |
| Worktree lifecycle / merge-back (`:93-95`) | b | authored |
| Checkpoint and resuming (`:97-102`) | b | authored |
| Preflight (`:104`) | c | will host the Q-60 rendered recommendation-rule fragment |
| Task-entry re-grounding (`:106`) | b | authored |
| Prose formatting (`:108`) | b | authored convention (Q-57) |
| Principles heading + `{{principles}}` (`:110-114`) | a (done, with a defect) | generated from `pack/principles.toml`, but numbered by position, see below |
| `{{instrument}}` / `{{modules}}` (`:116-118`) | a (done) | conditional generated includes |

The take: the file is dominated by class (b) authored prose, with a minority of class (a)/(c) structured facts, of which only convergence-constants, isolation-policy, principles, instrument, and modules are generated today. The role list, the file-naming conventions, the phase machine, and the human-input-contract rule text are all class (a)/(c) facts still HAND-COPIED, and each is a live drift surface (a second copy already exists in `next.rs` for the first two).

## 2. The slice already done: the single-source fragment pattern

Two sections are already single-sourced, and they are the pattern to generalize:

- `ISOLATION_POLICY_FRAGMENT` (`src/isolation_policy.rs:33`), a `&'static str`. `build_assets` substitutes it into the `{{isolation_policy}}` slot (`src/main.rs:267-270`); the driver emits the SAME constant as its writer-state reminder (`src/next.rs:821`, `... {ISOLATION_POLICY_FRAGMENT}`). Two `.contains()` drift-guard tests pin that the committed root `AGENTS.md` and `.agents/AGENTS.reference.md` each carry the exact fragment (`src/isolation_policy.rs:70-85`).
- `WorkflowSpec::control_fragment()` (`src/workflow_spec.rs:116-124`), which INTERPOLATES the convergence constants parsed from `pack/workflow.toml` (or `builtin()`). `build_assets` substitutes it into `{{workflow_control}}` (`src/main.rs:259`); the same spec's `required_streak` drives the W3 arithmetic in `workflow.rs`. Drift-guard tests pin the committed scaffold (`src/workflow_spec.rs:230-248`) and that the shipped `workflow.toml` parses equal to `builtin()` (`:201-207`).

The shape of the pattern: one source of truth (a Rust const, or a spec-derived render), consumed by BOTH the `AGENTS.md` slot AND the runtime (the driver / the validator), with a byte-level `.contains()` guard so the two copies cannot drift. `src/isolation_policy.rs:1-10` and `src/workflow_spec.rs:100-115` both describe themselves as members of a "workflow projection family." This is exactly the generalizable unit.

One gap in the pattern as built: the guards are per-fragment `.contains()`, not a whole-file byte-equality check. Nothing in the test suite asserts that a fresh `build_assets` render byte-equals the committed root `AGENTS.md` (regeneration is a `just scaffold-self` step, `justfile:46`, enforced only by the human running it and a clean tree). So a hand-edit of the AUTHORED prose between slots, or a dropped slot, is not caught the way `render --check` catches a hand-edit of `<task>.md`. This is the W4-baseline drift's blast radius (see section 5).

## 3. How the render engine works (the projection engine to reuse)

`src/plan/render.rs` is the existing structured-source -> prose projection engine and the architectural model for where `AGENTS.md` generation should head:

- `render_plan` (`:135-195`) is PURE with respect to output: it validates first (`validate_source`), fails loud on any schema violation, unresolved cross-reference, or missing sidecar, and writes nothing on failure (Principle 5, Principle 6 fail-loud). `assemble` (`:286-333`) concatenates GENERATED fragments (the derived Status line `:352`, the code-constant status VOCABULARY `:411-420`, the numbered Principles `:397-405`) with OPAQUE authored sidecar blobs inlined verbatim (`:301-306`, `:315-327`). It never parses a sidecar back into structure: the sidecars and the TOML are the sources, the `.md` is a one-way projection.
- `check_render` (`:257-280`) is the golden guard: a fresh in-memory render byte-compared against the committed `<task>.md`. A hand-edit of the generated file, a stale render after a source edit, or an absent file are all `Mismatch` (tests `:874-946`).

The key transferable idea is `assemble`'s split: GENERATED fragments for structured facts, OPAQUE verbatim blobs for authored prose, one deterministic order, one byte guard. `AGENTS.md` is the same shape as `<task>.md`, just with a much larger opaque-prose fraction and a much smaller generated-fragment fraction.

Note a numbering inconsistency the render engine exposes: `render.rs` numbers the plan's Principles by each entry's own `n` (`principles_section`, `:397-405`, "not a running counter"), while the `AGENTS.md` `{{principles}}` slot is numbered by POSITION, `i + 1` over the selection, in `render_principles` (`src/pack.rs:191-192`). Two numbering schemes for the "same" principle list. That is the root of the principle-numbering drift bug (section 5).

## 4. Reconciliation with Q-51 (the decided workflow-driver target)

Q-51 is DECIDED and folded into the `workflow-driver` umbrella (plan.toml `[[question]]` Q-51). The decided target: keep the workflow LOGIC DATA-DRIVEN (a `workflow.toml` spec of states, transitions, guards, and templates that the tool INTERPRETS, transition function in Rust, data in TOML), and GENERATE the `AGENTS.md` workflow section from that same machine definition, "the same render closure just built for the plan." Stage 0a (the convergence CONSTANTS spec, `src/workflow_spec.rs`) and Stage 0b (generate the `AGENTS.md` control fragment from the spec, byte-guarded) are the parts already landed; the FSM engine, the scheduler, and the write-path are staged behind evidence gates.

The generation architecture proposed here is the natural continuation of Stage 0b, and it must fit Q-51's frame exactly:

- Q-51's `workflow.toml` WIDENS from today's three constant tables (`[convergence]`, `[rounds]`, `[backstop]`, `pack/workflow.toml`) to also hold the roles, the phases/states, the transitions, the guards, and the file-naming conventions. That widened spec is the SINGLE source the driver interprets AND the `AGENTS.md` workflow-section slots render from.
- `control_fragment` is the seed of a FAMILY of spec -> fragment renderers, one per generated slot (a roles fragment, a naming fragment, a phase-machine fragment, alongside today's control fragment). Each is the forward-prose member of the projection family Q-51 names: `validate` = backward check, `status` = summary, `next` = forward instruction, `render`/these fragments = forward prose. All read the one spec, so the process prose and the process logic cannot drift.

This means the generation architecture is NOT a separate initiative from Q-51; it is Stage 0b generalized. Nothing here should invent a second spec format or a second engine.

## 5. The concrete generation architecture

### 5.1 The single source(s)

1. The widened `workflow.toml` (Q-51): the machine definition of roles, phases, transitions, guards, convergence constants (already there), and the workflow-managed file-naming conventions. The driver interprets it; a family of `spec.<x>_fragment()` renderers project prose from it.
2. `pack/principles.toml`: already the source for `{{principles}}`. Reconcile the two numbering schemes (`pack.rs` `i+1` vs `render.rs` `n`) onto one, and prefer projecting principles BY NAME where a citation is load-bearing (this is what `driver-output-generation.design.md:97` already did for the driver's escalate reminder, `projected_principle_reminder` in `src/next.rs:342`, immune to renumbering).
3. A small set of single-source RULE fragments for the class-(c) rules whose text is duplicated across locations: the isolation policy (done, `isolation_policy.rs`), and the Q-60 recommendation-in-options rule (decided to be rendered into both the human-input contract and Preflight). These can live as Rust consts (like `ISOLATION_POLICY_FRAGMENT`) or migrate into a `[rules]` table in `workflow.toml`; the Rust-const home is the proven one and is fine to keep.

### 5.2 Slots vs whole-file generation: slots, and here is why

RECOMMEND: keep `AGENTS.md` as a hand-authored TEMPLATE and grow the GENERATED-slot set. Reject whole-file generation. Reasoning judged against the principles:

- Whole-file generation would demand that every class-(b) authored paragraph (~95% of the file) live in the structured source as an opaque sidecar blob, exactly `render.rs`'s sidecar pattern applied to `AGENTS.md`. That is buildable, but it inverts the cost/benefit: it adds a `plan.toml`-scale skeleton + sidecar tree for a file whose drift risk lives ENTIRELY in the small generated-fragment minority, not in the authored prose. That cuts against `Minimal-by-default` (a large new mechanism for no marginal drift-safety over slots) with no `Prefer-the-cleaner-long-term-architecture` win, because the slot model ALREADY is "authored template + generated fragments," which is the same clean split `assemble` uses, just with the ratio flipped.
- The slot model isolates the drift surface precisely: the ONLY things that can drift are the generated fragments and the structured facts the prose hand-copies. Generate exactly those; leave the judgment prose authored in place. This satisfies `Structured-data-first-project-for-humans` (structured facts are single-sourced and projected) without over-reaching it (Principle 8 says derive human views by projection, not that every authored sentence must have a machine parent).

So the workflow section resolves to a HANDFUL of slots, today's five plus roughly four new ones:

- `{{roles}}`: the role -> prompt-path list, rendered from the spec's role set (kills the three-copy role/prompt-path drift with `next.rs:825` and the pack `prompts/` set).
- `{{findings_naming}}`: the findings / exploration / review-report naming conventions, rendered from the spec (kills the duplicate with `next.rs:848-852`).
- `{{workflow_phases}}`: the phase/transition machine prose, rendered from the widened spec (the Q-51 "generate the AGENTS.md workflow section from the machine definition" clause). This is the largest and last to land; it needs the widened spec to exist first.
- `{{recommendation_rule}}`: the Q-60 single-sourced recommendation-in-options rule, rendered into both the human-input-contract paragraph and Preflight.

Each new slot follows the proven fragment pattern: one source, consumed by the slot AND the runtime that also states it, byte-guarded.

### 5.3 Add the whole-file byte guard

Regardless of slots-vs-whole-file, add a `render --check`-equivalent for `AGENTS.md`: a test asserting a fresh `build_assets` render byte-equals the committed root `AGENTS.md` and `.agents/AGENTS.reference.md`. Today only per-fragment `.contains()` guards exist (`isolation_policy.rs:70-85`, `workflow_spec.rs:230-248`), which catch a stale fragment but not a hand-edit of the authored prose or a dropped slot. The whole-file guard makes "committed `AGENTS.md` diverges from what the pack generates" unrepresentable in a green build (`Make-illegal-states-unrepresentable`), the same guarantee `check_render` gives `<task>.md`. This is the cheapest high-value item and is independent of the slot-widening.

### 5.4 What drift each generated section kills (the two real bugs)

The project has already hit both drift bugs Q-64 cites, and each maps to a generated section that would have prevented it:

- The DRIVER PRINCIPLE-NUMBERING bug. The driver hard-coded a "Principle 6" citation while the `AGENTS.md` `## Principles` list is generated by `render_principles` numbering by `i+1` (`src/pack.rs:191-192`); adding/removing/reordering a principle renumbers the list but not the citation, so it silently points at the wrong principle (`docs/plans/driver-output-generation.design.md:28-31`; the ledger records prior "AGENTS.md-numbering leak" incidents, `docs/plans/architecture-audit.explorations/audit-data-model.md:57`). Fixed by projecting the principle BY NAME/TEXT from the plan source (`projected_principle_reminder`, `src/next.rs:342`), which is immune to renumbering. Lesson for the architecture: generate the citation from the same source as the list, and cite by a STABLE handle (name/id), never a positional number. Reconciling the `pack.rs` `i+1` vs `render.rs` `n` split is the remaining unfinished piece of this.
- The W4-BASELINE doc drift. The `AGENTS.md` prose describing the W4 baseline-exemption mechanism diverged from the live `[meta].w4_baseline` behavior in code, requiring a dedicated reconcile step (plan.toml step `reconcile-baseline-doc-drift`, order 73, complete). This is precisely the class a generated `{{workflow_phases}}` / mechanism slot kills: when the prose that describes a mechanism is RENDERED from the same spec the code reads, the prose cannot describe a mechanism the code does not implement. It is also the class the whole-file byte guard (5.3) would have caught earlier.

### 5.5 What authored prose remains, and why

The irreducible authored residue for THIS lens is the class-(b) set: the harness-agnostic bootstrap header; all WHY-rationale (why two clean rounds, why a cap, why role separation); and all judgment/behavioral guidance (adversarial review, sound triage, front-loading context, the file-safety behavioral rules, prose conventions). These are PROMPTABLE, not CHECKABLE: they are content the tool delivers but whose adherence it cannot verify, so by Q-64's own test they become tool-delivered prose and are never reduced to enforced structure. They CAN be relocated (into `workflow.toml` as opaque template text the tool emits, mirroring `render.rs` sidecars) but they cannot be GENERATED from more primitive structure, they are authored either way. The only real decision about them is WHERE they are authored (the pack template, which is where they are today and where Principle 2 says to leave them), not WHETHER they are generated. They stay authored; the generation effort targets only the structured facts they sit beside.

### 5.6 Staging (Principle 6, evidence-first)

The two landed fragments are the proof-of-concept. Widen incrementally, each slot its own low-risk step with a byte guard, cheapest-first: (1) the whole-file byte guard; (2) `{{recommendation_rule}}` (Q-60 is already decided and waiting); (3) `{{roles}}` and `{{findings_naming}}` (small spec additions, kill existing `next.rs` duplicates); (4) reconcile the principle numbering onto one scheme; (5) `{{workflow_phases}}` last, gated on the widened `workflow.toml` from Q-51's FSM stage. Do not build the whole ladder speculatively; each step removes a named drift surface and is independently valuable.

## Identity read (for Q-65)

From the generation lens, the project has crossed from SCAFFOLDING tool to structured WORKFLOW ENGINE / DRIVER. The evidence is directional, not incidental: the tool is becoming the SOURCE of the workflow DEFINITION (the widened `workflow.toml` machine spec that the driver interprets AND that `AGENTS.md` renders from), with `AGENTS.md` demoted to a projection of it. A scaffolder emits files once and is done; here the ongoing value is that the tool OWNS the workflow's structured definition and projects every view (validate/status/next/render/the AGENTS.md fragments) from it. The `scaffold` verb becomes the one-shot BOOTSTRAP delivery of a projection, one subcommand of the engine, not the identity.

One honest qualifier this lens adds: the generation architecture shows the tool will forever DELIVER a large authored-prose residue it does not itself generate (the judgment/behavioral floor). So it is a workflow engine that also ships a human contract, not a pure logic engine, which is why `AGENTS.md` survives (as a projection) rather than disappearing. That supports a rename toward a DESCRIPTIVE workflow-driver name (the plan's `agentflow` / `flowgate` direction) over keeping a "scaffold" name that mis-sells the ongoing driver identity, consistent with `Structured-data-first-project-for-humans` and `Prefer-the-cleaner-long-term-architecture` arguing the public name should project the real identity.
