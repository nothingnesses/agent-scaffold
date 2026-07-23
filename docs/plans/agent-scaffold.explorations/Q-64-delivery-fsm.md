# Q-64 explorer note: the delivery and FSM path

Lens: design the just-in-time DELIVERY mechanism (Q-64 ladder Rung 1) and the incremental FSM build path, reusing today's `next` / `render` / W3 machinery rather than proposing a competing design. This is the concrete Stage-1/Stage-2 realization of the DECIDED Q-51 staged driver (per-unit FSM fleet + `blocked_by` scheduler + data-driven `workflow.toml` + per-state templates + eventual `record-*` write-path), reconciled with the open Q-58 (the structured transient the reconstructor needs).

Principle numbering below is the plan's `[[principle]]` 1-8 by NAME (P1 Prefer-the-cleaner-long-term-architecture, P2 Minimal-by-default, P3 Safe-on-existing-projects, P4 Idempotent, P5 Make-illegal-states-unrepresentable, P6 Ground-decisions-in-evidence, P7 Reproducible, P8 Structured-data-first-project-for-humans).

## 1. What `next` reconstructs and emits TODAY (grounded)

`next` is already the Stage-1 advisory forward projection: read-only, stateless, recomputed from the durable files each call (`src/next.rs:1-25`). Concretely:

- STATE RECONSTRUCTION. `select_active_loop` (`src/next.rs:556-581`) picks ONE active unit deterministically: lowest-order in-progress step, else lowest-order ready pending step (`ReadyToPlan`), else lowest-order blocked pending step (`Blocked`), else `None`. For an in-progress step, `build_in_progress_loop` (`src/next.rs:632-693`) filters the step's rounds by `round_step_slug`, groups them by `round_increment_id` into a `BTreeMap` (`:642-645`), selects the active increment (`select_active_increment`, `:711-732`), and derives the `LoopState` (`derive_in_progress_state`, `:739-757`) from `peak_consecutive_clean` against `spec.required_streak(class)` and `spec.round_cap()`.
- THE SHARED ARITHMETIC IS ALREADY REUSED. `next` imports `peak_consecutive_clean`, `round_increment_id`, `round_step_slug` straight from `workflow.rs` (`src/next.rs:43-48`), and groups by the SAME `round_increment_id` key W3 groups by (`src/workflow.rs:467-469`). The module doc pins the differential property: a step `next` reports `converged` is exactly a step W3 finds no shortfall on (`src/next.rs:9-20`), and `RiskClassConflict` mirrors W3's `continue` on an inconsistent class (`:678-682`, `:699-702`).
- WHAT IT EMITS. `NextProjection` -> `ActiveLoop` (`src/next.rs:109-141`): step, increment, phase, `state`, risk_class, streak, `required_streak`, rounds, `round_cap`, `valid_transitions`, `isolation_tier`, and one `Instruction` (`:148-163`): `role`, `prompt_path`, `context` slots, `principle_reminders`, `filled_prompt_summary`.

So `next` already delivers the CONVERGENCE STATE and the NEXT ACTION (Q-64 Rung-1 items 5-6) well. The delivery GAP is in the other four items the Rung-1 ask names.

## 2. The four delivery gaps (Rung 1 vs today)

Rung 1 wants each `next` to emit "the role prompt filled with context, the phase's rules, the applicable principles, the resolved isolation tier, the convergence state, the next action". Measured against the code:

1. THE ROLE PROMPT is a POINTER, not filled content. `build_instruction` emits `prompt_path: format!(".agents/prompts/{role}.md")` (`src/next.rs:825`) and never opens the file. The pack ships those sources at `pack/prompts/*.md` (planner, reviewer, triager, implementer, orchestrator, plus checks-reviewer/open-questions-gate/clarifying-questions).
2. THE PHASE RULES are ORIGINATED PROSE, a second copy. `base_reminders` (`src/next.rs:294-329`) are hand-written imperatives the driver AUTHORS ("The reviewer is independent; a writer never reviews its own work", etc.). Decision D-a deliberately STRIPPED their `Principle N:` numbers because the number was doubly unstable (`:283-293`). Honest, but they are a SECOND copy of guidance whose canonical form lives in `pack/AGENTS.md` and `pack/prompts/*.md`, and nothing keeps them from drifting from it. This is exactly the drift Q-64 exists to kill (its ask: "two independent copies of the same rules diverging").
3. THE APPLICABLE PRINCIPLES are projected for ONE state only. `projected_principle_reminder` (`src/next.rs:342-349`) looks up the single hardcoded `ESCALATE_PRINCIPLE_NAME` ("Ground decisions in evidence", `:73`) by NAME from the plan's `[[principle]]` and emits its real name/text, degrading gracefully when absent. This is the RIGHT mechanism (project-by-name, drift-free), but it fires only at `Escalate` (`:808-812`). Every other state gets zero projected principles.
4. THE ISOLATION TIER is ECHOED, never RESOLVED. `run_next` maps a missing `--isolation-tier` to the literal `"unknown"` (`src/main.rs:1158-1159`), and the reminder then carries `TIER_RESOLVE_NOTE` (`src/next.rs:63-64, 816-818`). The tool points at the policy instead of resolving the tier. Q-51 emit.rs specifies resolving from `[isolation].tier_order` + the harness capability; `pack/workflow.toml` does not yet carry an `[isolation]` block (it holds only `[convergence]`/`[rounds]`/`[backstop]`).

Items 5-6 (convergence, next action) are delivered. The delivery work is closing 1-4, and the key insight is that 2 and 3 want the SAME single-source-rendered-fragment pattern the project already uses for `ISOLATION_POLICY_FRAGMENT` (`src/isolation_policy.rs:33`) and just DECIDED to generalize in Q-60 (render duplicates from one canonical source into the human-input contract AND the Preflight, not pointers, not hand-copies). Q-64 delivery is that pattern applied to the PHASE RULES and the PER-PHASE PRINCIPLES.

## 3. The per-state instruction template system

The delivered slice at each state is a PROJECTION of three sources, assembled by `build_instruction` (already the assembly seam, `src/next.rs:797-830`):

- SOURCE A, the workflow control spec (`workflow.toml`, widened per Q-51 sec 2.2). Supplies the per-phase RULE FRAGMENTS, the `[roles]` role->prompt-path map, the `[paths]` findings/ledger/exploration templates, `[isolation].tier_order`, and the convergence constants already there. The rule fragments are the SAME text rendered into `pack/AGENTS.md`'s workflow section (Q-51 Stage 0b/4 `{{workflow_control}}` slot), so the driver's emitted rule and the AGENTS.md sentence are two projections of one string and cannot drift. This REPLACES the originated `base_reminders` (gap 2): they stop being authored prose and become spec-rendered fragments keyed by phase.
- SOURCE B, the plan's `[[principle]]` (`plan.toml`, already threaded in via `NextInputs.principles`, `src/next.rs:505`). Supplies the APPLICABLE principles per phase, generalizing `projected_principle_reminder` from an escalate-only single lookup to a phase -> `[principle-names]` map projected by NAME (gap 3). By NAME, not number, for the same reason the escalate case already is (`:337-341`): two disagreeing numbered lists exist and the number renumbers on reorder (P5, P8 one-source-of-truth).
- SOURCE C, the reconstructed unit state + the Q-58 transient. Supplies the CONTEXT SLOTS (`build_context`, `src/next.rs:840-871`): ledger path, tier, and the findings/triage path templates already filled from `[paths]` conventions. The `artifact`/`diff` slots are deliberately NOT filled today (`:836-839`, deferred to Stage 2 per decision A-b) because no structured marker exists in the ledger yet; that is precisely the Q-58 dependency (sec 5).

The ROLE PROMPT itself (gap 1) should stay a POINTER (`prompt_path`), not be inlined. Reasons, judged against the principles: the role prompt is large, judgment-heavy, and belongs to the IRREDUCIBLE FLOOR Q-64 names (judgment-eliciting content the tool delivers but cannot verify); inlining it into every `next` call bloats the output (the exact anti-pattern Q-58 measured, where a 799-byte derived block was buried under a ~149 KB echo) and couples `next` to prompt-file IO for content the harness already loads by default. Deliver the SMALL control slice inline (rules + principles + tier + convergence + next action + filled context); POINT at the large judgment prompt. That is Minimal-by-default (P2) and matches the checkable-vs-promptable partition the Q-64 ask draws.

### Reuse of the render/pack machinery

The rule and principle fragments are generated exactly as `vocabulary_section` / `status_line` / `principles_section` already generate from code constants (`src/plan/render.rs`, per r2-architecture sec 1), and byte-guarded against hand edits like `render --check` and the `ISOLATION_POLICY_FRAGMENT` byte-guard tests (`src/isolation_policy.rs:56-82`). No new generation subsystem: `next` gains a `WorkflowSpec`-backed fragment lookup and a phase->principle-names table; the pack gains the `{{workflow_control}}` slot Q-51 Stage 0b already scopes.

## 4. The shared reconstructor seam

Today `next` and W3 SHARE THE ARITHMETIC (`peak_consecutive_clean` etc.) but DUPLICATE THE GROUPING/SELECTION: `build_in_progress_loop` (`src/next.rs:638-693`) re-implements the group-by-increment loop that `w3_problems` also writes (`src/workflow.rs:467-514`). They provably agree today only because the module doc and the `next_agrees_with_w3` differential test pin it (`src/next.rs:9-20`), not because they call one function.

The Q-51 Stage-2 seam (r2-architecture sec 3.3, sec 6) is to EXTRACT that grouping+fold into one `reconstruct_loop(rounds, spec) -> LoopState` (or a small `LoopFacts`) that BOTH `w3_problems` and `build_in_progress_loop` call, making "one arithmetic, two directions" literal code reuse rather than a tested coincidence (P5, P8, one-source-of-truth). This is the FSM path's load-bearing refactor: the driver's `step` transition function IS the checker's relation run forward (r2-architecture sec 1), so extracting the reconstructor is what lets the typed `ReviewLoop` machine and W3 share a single implementation and be differential-tested against each other.

Recommendation on the seam: extract the reconstructor in Stage 2 (the FSM stage), NOT in Stage 1. Stage 1's delivery gains (sec 6) do not need the extraction; forcing it early would refactor the safety-relevant W3 path for no Stage-1 delivery benefit (P6 evidence-first, P2 minimal). The differential test already guards divergence in the interim.

## 5. Reconciling with Q-58 (the transient the reconstructor needs)

The state reconstructor needs a resume-state INPUT for the sub-state the round log cannot carry: the module doc is explicit that the mid-round `awaiting-triage` sub-state is not derivable from the log (a `round` record is written only AFTER triage), so it is carried only by the verbatim `## RESUME STATE` block (`src/next.rs:22-25`). Q-58 is the open decision on what that input IS:

- Today `next` VERBATIM-ECHOES the block (`extract_resume_state`, `src/next.rs:939-958`; A-b chose verbatim to avoid heuristic prose parsing) and the diff/artifact context slots stay UNFILLED (`:836-839`).
- Q-58 asks whether to PROJECT the transient instead of echo, and whether to STRUCTURE it. Q-59's decision already folded the session-lifecycle currency signal INTO Q-58's carrier (a checkpoint-commit / staleness field the transient carries), so Q-58 is now the single structured-transient carrier.

The dependency for delivery: filling the `artifact`/`diff`/`awaiting` context slots (gap 1's context half, and the FSM's `Await` field, r2-architecture sec 3.2) REQUIRES the Q-58 transient to be STRUCTURED. A structured transient (P8, make-illegal-states-unrepresentable P5) lets `emit` fill those slots drift-free from a typed field rather than heuristically parsing prose (the A-b hazard). So the ordering is: Stage 1 delivery keeps the verbatim echo and the unfilled diff slots (no Q-58 dependency); Stage 2 FSM delivery consumes the structured Q-58 transient to fill `awaiting`/`diff`/`artifact` and to land the `Await` sub-state. Q-58 must be DECIDED before Stage 2, not before Stage 1. This is the clean split: Stage 1 delivery is unblocked; the FSM sub-state delivery is gated on Q-58 structuring.

## 6. The incremental build path

### Stage-1-shippable NOW, reusing today's `next` (read-only, no new subsystem)

Each item is a small edit to the existing `build_instruction` / `build_context` / `run_next` seam, evidence-gated by nothing beyond the existing tests (all read-only, P3 safe):

- D1. WIDEN `workflow.toml` to carry `[roles]`, `[paths]`, `[isolation].tier_order`, and the per-phase RULE fragments (Q-51 sec 2.2 shape; the file today has only `[convergence]`/`[rounds]`/`[backstop]`). Behaviour-preserving; extends the Stage-0a asset that already landed.
- D2. REPLACE the originated `base_reminders` (`src/next.rs:294-329`) with spec-rendered per-phase rule fragments single-sourced with the AGENTS.md `{{workflow_control}}` slot (closes gap 2; the Q-60 pattern generalized). Byte-guard the fragment like `ISOLATION_POLICY_FRAGMENT`.
- D3. GENERALIZE `projected_principle_reminder` (`src/next.rs:342-349`) from the escalate-only single lookup to a phase -> `[principle-names]` map, projecting each applicable principle by NAME from `plan.toml` (closes gap 3). Reuses the existing name-lookup + graceful-degrade code.
- D4. RESOLVE the isolation tier: add an `[isolation].tier_order` lookup in `emit` so a passed harness capability resolves to a concrete tier instead of `"unknown"` (closes gap 4). Keep the resolve-note fallback when the capability is genuinely unknown. Small, self-contained.
- D5. WIDEN the isolation reminder to the reviewer-spawn states (`AwaitingFirstReview`, `AwaitingReviewers`) and flip the pin test, per the already-DECIDED Q-62 (`spawns_writer` sibling predicate). This is decided, reversible driver work, not a new decision.

D1-D5 reuse the shipped `next`, `WorkflowSpec`, render/pack, and W3 arithmetic; none needs the reconstructor extraction or the FSM types. This is the Stage-1 DELIVERY increment.

### Evidence-gated later (the FSM path, Q-51 Stages 2+)

- Stage 2 (gated on Q-58 structuring decision + Stage-1 adoption evidence): extract the shared `reconstruct_loop` (sec 4), type the `ReviewLoop`/step/task fleet (r2-architecture sec 3.2), and fill the `awaiting`/`diff`/`artifact` slots from the structured Q-58 transient. Delivers the full per-unit sub-state and makes complete-without-convergence unrepresentable by construction (P5).
- Stage 3+ (gated as Q-51 already sets): `blocked_by` ready-frontier scheduler (gate: real parallelism), `record-*` write-path (gate: advisory-adoption evidence + reopening Q-24), authoritative/blocking driving (gate: measured residual drift). Unchanged from the Q-51 decision; delivery does not move these gates.

### Escape hatches (the driver must not become a cage)

Stage-1 delivery is READ-ONLY and ADVISORY: `next` suggests, the orchestrator acts, nothing blocks (Q-51 ORQ-1/ORQ-5, r2-synthesis sec on interaction). The tool ECHOES/RESOLVES the tier but never isolates (Q-51 concern 1: the tool instructs, the orchestrator isolates). The role prompt stays a POINTER so a human/agent can always read and override the full judgment content, not just the tool's slice. When the FSM write-path lands (far, gated), `record-override` is a FIRST-CLASS logged transition (r2-architecture sec 4), so a forced move is audited, never a silent bypass. The delivered principles are projected as REMINDERS/control-pointers, never manufactured verdicts (`src/next.rs:146-147` already states this boundary): the tool delivers the judgment-eliciting slice but never makes the judgment (the irreducible floor).

## 7. Judgement against the principles

- P8 (Structured-data-first): the whole delivery direction IS this principle on the PROCESS. The phase rules and applicable principles become projections of one structured source (`workflow.toml` + `plan.toml`), delivered by `next` and rendered into AGENTS.md, so the agent-vs-prose and code-vs-prose drifts both close. Strongest argument FOR.
- P1 (Cleaner-long-term-architecture): FOR. Delivery reuses the render/pack single-source pattern and the W3 arithmetic rather than adding a parallel mechanism; the reconstructor extraction (Stage 2) makes the driver/checker agreement structural, not coincidental.
- P5 (Make-illegal-states-unrepresentable): the escalate principle-by-name lookup and the structured Q-58 transient both remove a heuristic-parse / drifting-number failure surface; the typed fleet (Stage 2) makes complete-without-convergence unrepresentable.
- P6 (Ground-decisions-in-evidence / adopt-in-stages) and P2 (Minimal-by-default): the reason Stage-1 delivery (D1-D5) ships now and the FSM extraction/typing waits. D1-D5 are read-only projections reusing shipped code; the reconstructor refactor, the typed engine, and the write-path stay gated so no stage hardens the still-evolving workflow. Keeping the role prompt a pointer (not inlined) is P2 at the delivery boundary.
- P3 (Safe-on-existing-projects): D1-D5 are read-only and `WorkflowSpec::builtin()`-backed, so an un-migrated project is byte-for-byte unaffected until it scaffolds the widened `workflow.toml`.

Net: P8/P1/P5 pull toward building the delivery projections; P2/P6 pull toward staging the FSM machinery behind the Stage-1 evidence. D1-D5 satisfy both: they are the drift-closing projection slice with no new subsystem.

## Identity read (for Q-65)

The delivery trajectory makes the driver identity central, not peripheral. Once `next` delivers the filled control slice (rules + principles + resolved tier + convergence + next action) at each state as a PROJECTION of the structured source, the agent asks the TOOL what to do now rather than holding AGENTS.md in context (the Q-64 thesis). At that point the tool's ONGOING value is the plan/metrics/convergence machinery plus the just-in-time driver channel; `validate --workflow` ENFORCES, `render` PROJECTS, `next` DRIVES, and `scaffold` is reduced to ONE bootstrap subcommand. That is a harness-agnostic STRUCTURED WORKFLOW ENGINE / control plane, not a scaffolding tool: the scaffold generator is the setup step, the driver is the product. Note the precise limit (guards against over-claiming for the rename): it is NOT a meta-harness, agents INVOKE it and it never runs the LLM loop. Rename implication: a `scaffold`-rooted name mis-sells a workflow driver and imposes a standing comprehension tax (P8 project-for-humans, P1 the public name should project the real identity), so a rename is warranted and a DESCRIPTIVE driver/flow name (over a metaphor) fits this project's clarity values; the delivery conclusion here supports letting the name follow the settled driver identity per Q-65's own framing.
