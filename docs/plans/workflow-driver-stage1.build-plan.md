# workflow-driver Stage 1 build plan (advisory `agent-scaffold next` MVP)

Durable build record for Stage 1, produced by a read-only Plan pass (2026-07-19) and approved by the human with the recommended scope bundle. This survives compaction; the post-compaction orchestrator briefs an isolated implementer from this file (LOW_RISK, worktree-isolated; the W3 touch in decision B gets extra review scrutiny). Increment id `workflow-driver-stage1`.

## Decided scope (human, 2026-07-19): the recommended bundle A-b, B-a, C-b
- A-b: the filled prompt echoes the ledger `## RESUME STATE` block VERBATIM; populate structured `artifact`/`diff` slots only from an explicit marker if present, never by heuristic prose parsing (Principle 12 fail-loud, Principle 2). Structured artifact/diff field in the ledger template is deferred to Stage 2 (option C there).
- B-a: extract a shared streak helper so `next` (forward) and W3 (backward) provably run the SAME arithmetic. Make `round_step_slug`/`round_increment_id` (`src/workflow.rs:119-129`) `pub(crate)`, and extract `pub(crate) fn peak_consecutive_clean(records: &[&Round]) -> u64` used inside `w3_problems` (`src/workflow.rs:480`). Behaviour-preserving, guarded by existing W3 tests. The larger typed `reconstruct_loop` stays a Stage 2 concern.
- C-b: also build the thin `status --resume` slice (a `--resume` flag on `StatusArgs`) reusing the `extract_resume_state` helper `next` needs, closing the `state-queries` fold at near-zero marginal cost (Principle 8).

## Scope lock
Advisory, read-only, stateless `agent-scaffold next`. For the SINGLE active review loop it recomputes per-unit state from the durable files, reports loop state + valid transitions, and emits ONE filled instruction prompt. Human text + `--json`. Reuses the W3 streak/cap arithmetic so the forward projection agrees with `validate --workflow`'s backward check. NOT the multi-unit fleet or the ready-frontier scheduler (Stage 2/3+).

## State model (recomputed each call; no state store)
Assembled from three durable sources, read exactly as `validate --workflow`/`status` already read them:
- Plan via `--source <plan.toml>` (`plan::parse_toml` -> `PlanToml`, reuse `step_views()`/`question_views()` in `src/plan/source.rs:381-418`), or `--plan <md>` fallback (`plan::parse_roadmap`/`parse_questions`), mirroring the dual-source logic at `src/main.rs:908-959`.
- Rounds via `--metrics` (`metrics::parse_rounds`, `src/metrics.rs:660`; `Round` at `:620-650`).
- Ledger transient via `--ledger-fragment <path>` (default `docs/plans/<task>.ledger.md`): read verbatim, extract the `## RESUME STATE` section (heading convention).

Types (all `Serialize`, mirroring `Projection` at `src/main.rs:444-467`):
- `NextProjection { task, source, metrics, active_loop: Option<ActiveLoop>, resume_state: Option<String> }`.
- `ActiveLoop { step, increment: Option<String>, phase, state: LoopState, risk_class: Option<RiskClass>, consecutive_clean, required_streak: Option<u64>, total_rounds, round_cap, valid_transitions: Vec<String>, isolation_tier, next_instruction: Instruction }`.
- `Instruction { role, prompt_path, context: BTreeMap<String,String>, principle_reminders: Vec<String>, filled_prompt_summary }`.

Streak/risk computation REUSES the W3 accessors (per B-a): join rounds with `round_step_slug`, group by `round_increment_id`; streak = peak `consecutive_clean` over the increment's records vs `spec.required_streak(class)`; `risk_class` from the declared `[[step.increment]].risk_class` when present else the round records' `risk_class`.

## Active-loop selection (deterministic, order-keyed)
1. If >=1 step `status == in-progress`: active = lowest `order` in-progress step; active increment = the increment with round records whose peak streak < required (unconverged), else the increment id of the latest round record for the step. Note other in-progress steps exist (human text) but report the lowest-order one; true multi-loop fanout is Stage 2.
2. Else the next ready step = lowest-order `not-started`/`next` step whose `blocked_by` are all `complete`; next action = start-plan.
3. Else `active_loop = None` (all complete, or all blocked).

## Transition table (convergence checked BEFORE cap, per pack/AGENTS.md)
| Step status | Round evidence (active increment) | state | valid_transitions | Next action (role) |
|---|---|---|---|---|
| not-started/next, blockers clear | - | ready-to-plan | start-plan | spawn planner |
| not-started, blockers unmet | - | blocked | - | resolve blockers (no spawn) |
| in-progress | no rounds yet | awaiting-first-review | record-round | spawn a reviewer (round 1) |
| in-progress | last round new_valid (streak 0) | awaiting-fixes | address-findings | spawn implementer to fix, then a fresh reviewer round |
| in-progress | last round clean, peak < required | awaiting-reviewers | record-round-clean / record-round-new-valid / escalate | spawn a FRESH reviewer (diversify model) |
| in-progress | peak >= required | converged | mark-step-complete | mark complete, render, commit |
| in-progress | total_rounds >= round_cap AND peak < required | escalate | escalate-to-human | escalate, present the human-input contract (Q-54 reminder) |
| complete | - | done | - | move to next step |

Evaluate `converged` before `escalate`. `total_rounds` = count of the active increment's round records; `round_cap` is advisory forward guidance (W3 does not enforce it; consistent with advisory mode ORQ-1).

Known Stage 1 boundary (accepted): the mid-round `awaiting-triage` sub-state is NOT derivable from the JSONL (a `round` record is written only after triage), so Stage 1 derives states from completed round records + step status; the in-flight sub-state is surfaced only via the verbatim `## RESUME STATE`. Refining this (parsing the ledger round narrative) is Stage 2's `driver/reconstruct.rs`.

## Filled instruction prompt (slot-fill, no prose generation; slots from EXISTING conventions, not a spec extension)
- role -> prompt_path: `.agents/prompts/<role>.md` (convention); hardcoded role->path mapping in Rust, NOT added to `WorkflowSpec` (role/path templates deferred to generation-widening).
- ledger path: `docs/plans/<task>.ledger.md` or `--ledger-fragment`.
- findings_path: reviewer `docs/plans/<task>.reviews/<step>-<role>-<disambiguator>.md`, triager `<step>-triage.md` (emit the template string; orchestrator assigns the concrete disambiguator).
- artifact/diff: per A-b, from a structured marker if present, else absent (the orchestrator reads the verbatim `resume_state`). No heuristic prose parsing.
- principle_reminders: phase-keyed hardcoded Rust table keyed by state/role, citing Project Principles by number (source `[[principle]]` in the plan TOML). Reminders are control pointers, never manufactured verdicts.
- Q-54 human-input-contract reminder: emitted ONLY when `state == escalate` (the awaiting-human-decision gate): "present the options, their trade-offs, an explicit recommendation, and Principle-judged reasoning (naming the Principle numbers)", referencing (not restating) the AGENTS.md contract. The driver's first concrete anti-drift reminder.
- isolation_tier: echo `--isolation-tier <worktree|container|file-safety>`; default "unknown" with a "resolve per the AGENTS.md tier policy" reminder. The tool never emits worktree paths/branch names.

## Output contract (deterministic)
- `--json`: `serde_json::to_string_pretty(&NextProjection)` (as `run_status`).
- Human text: `task`/`source` header, an `ACTIVE LOOP` block (`step [/ increment]  <phase> -> <transition>`, `state`, `streak N/required`, isolation tier, `next:` action, `prompt:`, `context:`, `reminders:`, and at a gate the human-input-contract note). When `active_loop` is None, print "no active review loop (...)".
- Determinism: echo CLI paths verbatim (relative, never canonicalize); no wall-clock/timestamps; `BTreeMap` context; fixed reminder ordering; identical bytes on any machine; idempotent.

## state-queries reconciliation
Build `next` as the core deliverable + the thin `status --resume` slice (C-b). Mark the `state-queries` `[[step]]` as `superseded` (`superseded_by = workflow-driver`) in `plan.toml` and re-render, so there is no duplicate step. Repointing `.agents/user-prompts/resume.md`/`compaction-prep.md` at `next`/`status --resume` is an optional pack-prose follow-up.

## Test plan
1. Transition coverage: one test per table row (ready-to-plan, awaiting-first-review, awaiting-fixes, awaiting-reviewers, converged, escalate, done, blocked), asserting state, valid_transitions, role/next-action.
2. DIFFERENTIAL test (key acceptance): for fixtures, assert `next`'s forward projection agrees with `w3_problems`' backward verdict on the same files (converged <-> no shortfall; awaiting/escalate <-> shortfall). Structural because both call the shared `peak_consecutive_clean` + join accessors (B-a).
3. Q-54 gate reminder: a cap-reached unconverged fixture asserts `state == escalate` and the human-input-contract cue present; a non-gate fixture asserts it absent.
4. RESUME STATE extractor: a ledger with the section returns it verbatim; one without returns None; the section terminates at the next `## ` heading.
5. Golden output: golden human-text and golden `--json` for one fixture (byte-compare).
6. Determinism/idempotence: same inputs -> identical bytes across two runs; paths echoed verbatim.
7. Dual-source parity: `next` from a TOML-primary `--source` and an equivalent Markdown `--plan` give the same active-loop verdict.

## Files to add/change
- ADD `src/next.rs`: active-loop selection, transition function, instruction assembly, `extract_resume_state`, the `Serialize` types, human-text renderer, and tests. Do NOT create `src/driver/` (Stage 2).
- CHANGE `src/main.rs`: `mod next;`; add `Next(NextArgs)` to `Command` and dispatch; `NextArgs { source, plan, metrics, ledger_fragment, isolation_tier, json }` mirroring `StatusArgs`/`ValidateArgs`; `run_next`. Add `--resume` to `StatusArgs` + handling in `run_status`.
- CHANGE `src/workflow.rs`: `round_step_slug`/`round_increment_id` -> `pub(crate)`; extract `pub(crate) fn peak_consecutive_clean` used in `w3_problems`. Behaviour-preserving; guarded by existing W3 tests. (This is the safety-relevant touch; review scrutinizes it.)
- CHANGE `docs/plans/agent-scaffold.plan.toml` (+ re-render): mark `state-queries` superseded; declare the `workflow-driver-stage1` increment (`low_risk`) at close (orchestrator/planner-owned plan edit, done at close not by the implementer).

## YAGNI (do NOT build)
No `src/driver/` typed FSM; no `reconstruct_loop` full typed extraction (Stage 2); no ready-frontier scheduler / multi-unit fanout (Stage 3+); no `record-*` write-path or JSONL writing (read-only, Q-24); no `WorkflowSpec` extension for role/path/reminder tables (derive from conventions); no AGENTS.md-from-FSM generation; no worktree/container creation (emit the tier only); no persisted state; no per-turn polling.

## Risk note
Overall LOW_RISK (read-only, reuses shipped parsers), but decision B touches the safety-relevant W3 `w3_problems`/join functions. Treat the W3 extraction with risky-grade scrutiny in review (behaviour-preservation + the differential test), even though the increment as a whole converges at one clean round unless the review classifies it risky at loop-open.
