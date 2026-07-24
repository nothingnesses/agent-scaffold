# Build plan: `roles-findings-naming-slots` (step 82, Q-64)

A design pass, written before any code, for the step that was chartered to add generated `{{roles}}` and `{{findings_naming}}` slots to `pack/AGENTS.md`, single-sourced with the values the `next` driver emits. This document validates the step's premise against the actual code, corrects it where it is wrong, and recommends the step's final shape. It is prose for a human and a reviewer to read, grounded in `file:line` evidence, following the human-input-contract-written-to-a-file shape.

Bottom line up front:

- `findings_naming`: a genuine duplicate, worth single-sourcing, but NOT yet drifted in VALUE (the driver and the convention agree today) and NOT the verbatim-`&'static str` case, because the driver's value is a runtime-token-bearing path. Build it, with a token-aware single-source mechanism (a genuine fork on which mechanism, escalated below).
- `roles`: no genuine duplicated prose fragment exists between `pack/AGENTS.md` and `src/next.rs`. The driver's role handling is structural (an FSM-state -> role-identifier map), not a second copy of the AGENTS.md role bullets. Recommend DROPPING the roles half from this step rather than manufacturing a slot for a duplicate that does not exist.

So the step reduces from two slots to one (`findings_naming`), and its title/charter should be corrected accordingly.

---

## 1. Scope: what each source states today (the evidence)

### 1.1 `findings_naming`

The canonical prose convention, `pack/AGENTS.md:67` ("Findings files" paragraph):

> The filenames follow one convention so parallel writers never collide: a reviewer's file is `<step>-<role>-<disambiguator>.md`, where the orchestrator assigns each spawned reviewer a distinct disambiguator (its model, or an index); the triager's is `<step>-triage.md`; and the backstop re-check triager's is `<step>-triage-recheck.md`.

The directory is stated at the head of the same paragraph and again throughout: `docs/plans/<task>.reviews/`.

The driver's independent encoding, `src/next.rs:880-884` (`build_context`):

```rust
let review_findings = format!(
    "docs/plans/{}.reviews/{}-reviewer-<disambiguator>.md",
    context.task, facts.step
);
let triage_findings = format!("docs/plans/{}.reviews/{}-triage.md", context.task, facts.step);
```

These are inserted into the `review_findings` / `triage_findings` context slots for the review and fix states (`src/next.rs:885-892`).

Exact comparison, field by field:

- Reviewer file. Convention: `<step>-<role>-<disambiguator>.md`. Driver: `{step}-reviewer-<disambiguator>.md`. The convention's `<role>` placeholder resolves to the literal `reviewer` for a reviewer's file, so the driver's `reviewer` literal MATCHES the convention in value. It differs only in REPRESENTATION (a filled literal versus the generic `<role>` token). This is not a value drift; it is a second, hand-maintained encoding of the same shape.
- Triager file. Convention: `<step>-triage.md`. Driver: `{step}-triage.md`. Identical.
- Backstop re-check file. Convention: `<step>-triage-recheck.md`. Driver: emits NO recheck path. The driver has no recheck state or action in its FSM (`LoopState`, `src/next.rs:188-219`), so it never produces this path. This is a coverage gap, not a drift: the driver simply does not model that case, so there is nothing to disagree with.
- Directory. Both use `docs/plans/<task>.reviews/`. Identical.

Correction to the orchestrator's re-grounding finding: the premise "findings_naming ... has ALREADY DRIFTED (literal 'reviewer' vs `<role>`, missing `-triage-recheck`)" is only weakly true. The two sources have NOT diverged in the value they produce; the reviewer/triage paths agree, and the missing recheck path is an un-modelled case rather than a conflicting one. What is true and load-bearing: this is a genuine SECOND ENCODING of the naming convention, hand-maintained in `next.rs`, drift-PRONE (a future edit to the convention would not propagate to the driver), and it belongs in one place. The justification for the step is prevention of future drift plus the P8/P1 "kill hand-maintained duplicates" goal, NOT the repair of an existing value bug. The build plan and any reviewer brief should state this honestly so the step is not sold as a bug fix it is not.

Important corroborating evidence: every OTHER consumer of the convention already single-sources correctly by POINTER, not by copy. `pack/prompts/reviewer.md:11`, `pack/prompts/triager.md:9`, `pack/prompts/orchestrator.md:7`, and `pack/prompts/checks-reviewer.md:15` all say some form of "the naming convention is in `AGENTS.md`" and let the orchestrator assign the concrete path. `pack/AGENTS.md:65` (Design explorations) reuses it by reference ("the same rule as findings files"). So `pack/AGENTS.md:67` is ALREADY the single prose source of truth that the whole prompt surface defers to; `src/next.rs` is the lone place that re-encodes the shape instead of deferring. That narrows the step to exactly one offender and confirms the single-source target is the `pack/AGENTS.md:67` convention sentence.

One boundary item: `pack/checks-guidance.md:9` names an ADDITIONAL findings-file kind, `<step>-checks.md`, for the checks-reviewer. It is module-specific guidance, not part of the core convention sentence at `pack/AGENTS.md:67`, and it is stated in prose (not re-encoded in the driver). It is out of scope for this slot (see YAGNI, section 6).

### 1.2 `roles`

The AGENTS.md prose, `pack/AGENTS.md:19-23`: five role bullets (Orchestrator, Planner, Reviewers, Triager, Implementer), each a sentence or paragraph describing that role's RESPONSIBILITIES and authority.

The driver's role handling, `src/next.rs`:

- `LoopState::role()` (`src/next.rs:238-249`) maps each FSM state to the role IDENTIFIER that acts next: `"planner"`, `"reviewer"`, `"implementer"`, or `"orchestrator"`. It is a state-to-actor function, not a description of what a role does.
- `build_instruction` (`src/next.rs:857`) uses that identifier structurally to build the prompt path `.agents/prompts/{role}.md`.
- `base_reminders()` (`src/next.rs:319-354`) carries phase-keyed imperatives (for example "The reviewer is independent; a writer never reviews its own work.", `src/next.rs:328`). These are per-PHASE originated guidance, not a restatement of the role bullets, and the module comment at `src/next.rs:307-318` explicitly documents that they carry no canonical source and "nothing to drift from".

There is no prose in `next.rs` that lists or describes the roles the way `pack/AGENTS.md:19-23` does. The overlap is only the role NAMES ("planner", "reviewer", "implementer", "orchestrator"), which the driver uses as structural identifiers to route to prompt files, not as a duplicated description. Note also two structural mismatches that show the two are not the same set: the driver's actor set has no `triager` (the triager is not a "next role" the driver emits; the orchestrator spawns it inside a round), while the AGENTS.md role list does; and the driver adds `orchestrator` as the fallback actor for non-spawn states. The sets are not congruent, so there is no clean fragment to single-source.

The step sidecar (`docs/plans/agent-scaffold.steps/roles-findings-naming-slots.md:3`) cites a roles duplicate "around `:825`" of `next.rs`. That line number is stale (the file has changed since the sidecar was written); the current `src/next.rs:825` sits inside the `build_instruction` doc comment and is a parenthetical "(planner, implementer, or reviewer/triager)" listing which states spawn agents, not a role-description duplicate. No duplicated roles fragment exists at that location or elsewhere in the file.

Correction to the sidecar/charter: the premise that `{{roles}}` kills a duplicate "around `:825`" does not hold. There is no roles duplicate to kill.

---

## 2. The precedent, and why `findings_naming` differs from it

Three single-source members already exist, and they split into two kinds:

Verbatim-prose members (`ISOLATION_POLICY_FRAGMENT` in `src/isolation_policy.rs:33`; `RECOMMENDATION_RULE_FRAGMENT` in `src/recommendation_rule.rs:34`). Each is a `&'static str` with NO computed input. `build_assets` (`src/main.rs:273-288`) substitutes it into the `{{slot}}` and a byte-guard test (`src/isolation_policy.rs:69-85`, `src/recommendation_rule.rs:72-88`) asserts the committed `AGENTS.md` and `.agents/AGENTS.reference.md` contain the exact bytes. The driver emits the SAME `&'static str` verbatim as a reminder (`src/next.rs:853`), so the two copies are byte-identical and cannot drift.

Computed-input member (`WorkflowSpec::control_fragment`, `src/workflow_spec.rs:116-124`). This one INTERPOLATES values via `format!` (`{low}`, `{risky}`, `{cap}`, `{severity}`), but the interpolated values are SCAFFOLD-TIME constants known from the spec. The AGENTS.md slot shows the FILLED result ("1 consecutive clean round ... 2 for a risky ... 5 rounds", `src/workflow_spec.rs:226`), and the driver reads the same spec numbers via `required_streak`/`round_cap`. Byte-guard tests pin both the generated fragment and the committed scaffold (`src/workflow_spec.rs:217-239`).

Why `findings_naming` is a THIRD kind, matching neither cleanly:

- It is not verbatim prose, because the driver's value is a PATH bearing RUNTIME tokens (`task`, `step`) filled per invocation (`src/next.rs:881-882`), plus a `<disambiguator>` token the orchestrator fills even later. So the driver cannot emit a fixed `&'static str`; the isolation_policy pattern does not transfer directly.
- It is not the control_fragment case either, because the tokens are NOT known at scaffold time. `control_fragment` bakes the spec constants into the AGENTS.md text; `findings_naming` cannot bake `<task>`/`<step>` into the AGENTS.md text, so the AGENTS.md slot must render the convention with the tokens UNFILLED (`<task>`, `<step>`, `<role>`, `<disambiguator>`), while the driver fills `task`/`step` and leaves `<disambiguator>`.

So the single source must express a path FORMAT with named tokens that can be rendered two ways: all-tokens-unfilled for the human convention in AGENTS.md, and task/step-filled for the driver's slot. That is what the mechanism options below have to deliver.

---

## 3. Design space for `findings_naming` (the mechanism options)

All three options put the same one slot, `{{findings_naming}}`, in place of the convention sentence at `pack/AGENTS.md:67` and add it to `RESERVED_VARS` (`src/manifest.rs:140-147`) and `build_assets` (`src/main.rs`). They differ in WHERE the one canonical path format lives and how the driver is kept in step with it.

### Option A: verbatim convention fragment for AGENTS.md, plus a consistency guard on the driver

A new `src/findings_naming.rs` holds a `&'static str` convention fragment (isolation_policy-style), stating the reviewer/triage/recheck basenames with tokens (`<step>-<role>-<disambiguator>.md`, `<step>-triage.md`, `<step>-triage-recheck.md`) and the `docs/plans/<task>.reviews/` directory. `build_assets` substitutes it into `{{findings_naming}}`; a byte-guard test pins the committed `AGENTS.md` and reference copy to it (exactly the isolation_policy test). The driver KEEPS its `format!` calls (`src/next.rs:881-884`), and a NEW consistency test asserts that the driver's produced paths, for sample `task`/`step`, match the shapes the convention fragment declares (for example, that `build_context`'s `review_findings` equals `docs/plans/<task>.reviews/<step>-reviewer-<disambiguator>.md` with the tokens filled).

Trade-offs:

- P2 "Minimal by default": strongest. Smallest diff; one const, one slot, two short tests; mirrors the exact proven isolation_policy pattern almost verbatim.
- P6 "Ground decisions in evidence" / P7 "Reproducible": strong. It reuses the precedent that already works and is already tested, so the risk is well understood.
- P1 "Prefer the cleaner long-term architecture over the smallest diff": WEAKEST of the three. The path shape still lives in TWO code encodings, the prose const and the driver's `format!`; the consistency test is a referee between two copies, not the elimination of the copy. The step was chartered precisely to KILL the duplicate ("one source of truth, consumed by both", sidecar line 4), and this option guards the duplicate instead of removing it.
- P8 "Structured data first, project for humans": partial. The AGENTS.md half is projected from a source; the driver half is not projected from that source, it is checked against it.

### Option B: one canonical token-template, rendered two ways (true single source)

A new `src/findings_naming.rs` holds the path FORMAT as the single source: the basename templates as named-token constants (for example `"{step}-reviewer-{disambiguator}.md"`, `"{step}-triage.md"`, `"{step}-triage-recheck.md"`) plus the `docs/plans/{task}.reviews/` directory, with small render helpers. It exposes (1) builder functions the driver calls to produce filled paths (fill `task`/`step`, leave `<disambiguator>` as the literal token), replacing the `format!` calls at `src/next.rs:881-884`; and (2) a `convention_fragment()` that renders the SAME templates with every token left as its `<...>` placeholder, producing the human-readable convention sentence for the `{{findings_naming}}` slot. One byte-guard test pins the committed scaffold to `convention_fragment()`; the driver and the slot both derive from the one set of template constants, so no second encoding exists.

Trade-offs:

- P1 "Prefer the cleaner long-term architecture over the smallest diff": STRONGEST. The path format lives in exactly one place; both consumers derive from it; the duplicate is eliminated, not guarded. This is the architecture the step was chartered to reach.
- P8 "Structured data first, project for humans": strongest. The convention is a structured template projected into both the human AGENTS.md view and the machine path.
- P5 "Make illegal states unrepresentable": moderately better than A. Centralising the builders is a step toward the paths being constructed only one way (fuller typing is Option C).
- P2 "Minimal by default": WEAKER than A. More code: template constants, a partial-fill helper (Rust `format!` cannot leave a named arg unfilled, so the render is done by placeholder substitution, `str::replace` on `{token}`, or an equivalent tiny helper), the driver rewrite, and `convention_fragment()`. Still small and bounded, but more than a single const.
- P7 "Reproducible": equal to A once the byte-guard test is in place.

### Option C: a small typed builder (make the role/kind a type)

Like B, but the findings-file KIND is a typed enum (Reviewer, Triage, TriageRecheck) with builder methods (`reviewer(step, disambiguator)`, `triage(step)`, `triage_recheck(step)`) returning the paths, plus `convention_fragment()` for the slot. The driver calls the typed builder; the slot renders from the same module.

Trade-offs:

- P5 "Make illegal states unrepresentable": strongest. The role/kind is not a free-form string; only the three valid kinds can be constructed, and the disambiguator handling is explicit per kind.
- P1: as strong as B.
- P2 "Minimal by default": WEAKEST. For what is today two `format!` lines with a single caller (`build_context`, `src/next.rs:872`), a typed enum plus methods is more machinery than the current call sites justify. The extra types buy little until a second caller or a fourth kind appears, neither of which exists (`<step>-checks.md` lives in the checks module and is explicitly out of scope).
- P6 "Ground decisions in evidence": the evidence (one caller, three kinds, no growth pressure) does not yet justify the type. This is gold-plating relative to the present need.

---

## 4. Recommendation

### 4.1 `findings_naming` mechanism: Option B (with A as the lighter fallback)

Recommend Option B. Reasoning judged against the principles by name:

- The step exists to KILL a hand-maintained duplicate (sidecar line 4: "one source of truth, consumed by both the AGENTS.md slot and the driver, byte-guarded so the copies cannot drift"). Option B is the only option that literally does that; A guards two copies and C over-builds. P1 "Prefer the cleaner long-term architecture over the smallest diff" and P8 "Structured data first, project for humans" both point at B.
- B's extra cost over A is small and bounded (a placeholder-substitution helper and the driver rewrite), so P2 "Minimal by default" is not badly served; the diff is still a single small module plus a slot plus one guard test.
- Reject C: P2 and P6 both argue against a typed enum for one caller and three fixed kinds with no growth pressure; the type is not yet earned.
- Reject A as the primary: it leaves the very duplicate the step was chartered to remove, downgrading the step to a consistency check. It remains a reasonable FALLBACK if a reviewer or the human judges B's module overkill for two `format!` lines, because A still prevents silent drift and is the closest match to the proven pattern (P6, P7).

This A-versus-B choice is a genuine fork (kill the duplicate with a small module, versus guard two copies with the lighter proven pattern). It is escalated in section 7 for the human to decide, because it trades P1/P8 against P2 and reasonable people could weigh those differently for a duplicate that has not drifted in value.

### 4.2 `roles`: DROP from this step

Recommend dropping the roles half entirely. Reasoning:

- P6 "Ground decisions in evidence": there is no duplicated roles fragment between `pack/AGENTS.md:19-23` and `src/next.rs`. The driver's role handling is a structural FSM-state -> actor-identifier map (`src/next.rs:238-249`), not a copy of the role descriptions, and the two role sets are not even congruent (no `triager` actor; `orchestrator` added as fallback). Manufacturing a `{{roles}}` slot would fabricate a single-source relationship for a duplicate that does not exist.
- P2 "Minimal by default": a slot with no real duplicate behind it is pure addition with no duplicate removed, the opposite of minimal.
- Not a deferral, a drop: there is no latent question here to revisit. If a future need arises to single-source the role VOCABULARY (the four identifier strings), that is a different and much smaller idea and should be raised fresh with its own evidence, not carried as a pending half of this step. So the `{{roles}}` slot idea is dropped entirely, and the step, its title, and its sidecar are corrected to the single `findings_naming` slot.

---

## 5. Recommended final shape of the step

Slot(s) to build: one, `{{findings_naming}}`. Drop `{{roles}}`.

Single-source mechanism (Option B, pending the section-7 decision): a new `src/findings_naming.rs` module holding the canonical path templates and the `convention_fragment()` renderer, consumed by both the driver and the pack.

Exact files to add / change:

- Add `src/findings_naming.rs`: the canonical basename templates (reviewer, triage, triage-recheck) and the `docs/plans/{task}.reviews/` directory as the single source; builder helpers that fill `task`/`step` and leave `<disambiguator>`; `convention_fragment()` rendering the all-tokens-unfilled convention sentence; and its `#[cfg(test)]` byte-guard tests (content assertions plus the committed-scaffold containment test, mirroring `src/isolation_policy.rs:35-86`).
- `src/main.rs`: declare `mod findings_naming;` (near `src/main.rs:14,21`), and in `build_assets` (`src/main.rs:243-296`) insert the `findings_naming` key with `findings_naming::convention_fragment()` (alongside the existing `isolation_policy`/`recommendation_rule` inserts at `src/main.rs:273-288`).
- `src/manifest.rs`: add `"findings_naming"` to `RESERVED_VARS` (`src/manifest.rs:140-147`).
- `pack/AGENTS.md:67`: replace the convention sentence ("The filenames follow one convention ... `<step>-triage-recheck.md`.") with `{{findings_naming}}`, leaving the surrounding paragraph (the directory intro, the merge/read/cleanup prose) hand-authored.
- `src/next.rs:880-884` (`build_context`): replace the two `format!` calls with calls into `findings_naming`, so the driver's `review_findings` and `triage_findings` slots derive from the one source. (No new driver STATE; the recheck path is not modelled by the driver and this step does not add it, see YAGNI.)
- Regenerate the committed scaffold with `just scaffold-self` so the committed root `AGENTS.md` and `.agents/AGENTS.reference.md` carry the rendered `{{findings_naming}}` bytes, which the byte-guard test then pins. Correct the step's sidecar (`docs/plans/agent-scaffold.steps/roles-findings-naming-slots.md`) and title to the single `findings_naming` slot (drop roles); assess doc-currency for any prose that referenced a two-slot step.

If Option A is chosen instead: same files, except `src/findings_naming.rs` holds a verbatim `&'static str` convention fragment (not templates), the driver's `format!` calls at `src/next.rs:881-884` stay, and the driver-side guard is a consistency test asserting `build_context`'s output matches the fragment's declared shapes rather than a rewrite of `build_context`.

Guards (either option): the byte-guard test on the committed scaffold (isolation_policy-style, reading `../AGENTS.md` and `../.agents/AGENTS.reference.md` via `include_str!`); plus, under Option A only, the driver consistency test. Under Option B the driver is guarded structurally by deriving from the one source, so a separate consistency test is optional (a unit test that the builder fills tokens as expected is enough).

Risk classification: low_risk. Reasoning: the change is prose (one AGENTS.md sentence) plus driver path-formatting, with no security, safety, data, or money sensitivity; the blast radius is the driver's advisory `next` output and one guidance sentence; it is easily reversible; and it is covered by byte-guard tests so a mistake fails loudly at `cargo test` and `just scaffold-self`. Under the built-in spec (`src/workflow_spec.rs:52-54`), low_risk requires 1 consecutive clean round to converge. (One caveat that does NOT raise the class: `pack/AGENTS.md` is dogfooded, so this step edits the very guidance the workflow runs under; the byte-guard test and `render --check` catch a stale committed scaffold, keeping the edit safe.)

---

## 6. YAGNI boundary (what NOT to build)

- Do NOT build the `{{roles}}` slot. No duplicate backs it (section 1.2, 4.2). Drop it; do not defer it as a hidden half of this step.
- Do NOT add a driver recheck STATE or a recheck-path emission to model `<step>-triage-recheck.md`. The driver has no recheck action (`src/next.rs:188-219`); the convention fragment names the recheck basename for the human, but the driver need not produce it. Adding a state is out of scope.
- Do NOT fold `<step>-checks.md` (`pack/checks-guidance.md:9`) into the core `{{findings_naming}}` slot. It is checks-module-specific and lives in that module's guidance; keep the module boundary.
- Do NOT generate the WHOLE "Findings files" paragraph (`pack/AGENTS.md:67`). Only the single convention sentence is the single-sourceable unit; the merge/read/cleanup prose stays hand-authored.
- Do NOT fill `<disambiguator>` in the driver. The orchestrator assigns it (`pack/AGENTS.md:67`, `src/next.rs:867`); the driver leaves it as a template token, as it does today.
- Under Option B, do NOT reach for Option C's typed enum. One caller and three fixed kinds do not yet earn the type (P2, P6).
- Do NOT rewrite the prompt files' "the naming convention is in `AGENTS.md`" pointers (`pack/prompts/reviewer.md:11`, `triager.md:9`, `orchestrator.md:7`, `checks-reviewer.md:15`). They already single-source correctly by reference and need no change.

---

## 7. The decision to escalate

One genuine fork the orchestrator should put to the human per the human-input contract:

The `findings_naming` single-source mechanism: Option B (a small `findings_naming` module that KILLS the duplicate, the path format living once and both the driver and the AGENTS.md slot deriving from it) versus Option A (the lighter proven verbatim-fragment-plus-consistency-guard, which single-sources the AGENTS.md copy and GUARDS the driver copy against it rather than removing it). The trade-off is P1 "Prefer the cleaner long-term architecture over the smallest diff" and P8 "Structured data first, project for humans" (favor B, which removes the duplicate the step was chartered to remove) against P2 "Minimal by default" (favor A, the smaller diff and near-exact reuse of the isolation_policy pattern). Recommendation: Option B, because the step's stated purpose is to eliminate the duplicate, not to guard it, and B's extra cost is small and bounded; A is the acceptable fallback if the human judges a new module overkill for a duplicate that has not drifted in value.

The roles drop (section 4.2) is presented as a recommendation with its evidence rather than as an open fork: the evidence (no duplicate exists) points one way, so it is a correction to the charter for the human to confirm, not a balanced choice. If the human disagrees and wants a `{{roles}}` slot regardless, that reopens as its own question with its own evidence.
