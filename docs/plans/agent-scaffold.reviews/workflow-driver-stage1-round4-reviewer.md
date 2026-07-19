# workflow-driver Stage 1 - Round 4 review (fresh independent, adversarial)

Reviewer: fresh Round 4 sampling. Branch `impl/wf-driver-stage1`, HEAD `cd07173`, base main `4fee243`. Artifact unchanged since Round 3.

## Verdict: CLEAN (zero findings)

Independent confirmation. The increment converges the RISKY loop (Round 3 clean + Round 4 clean). I did a genuinely separate pass rather than re-running Round 3; no defect found.

## What I probed (and why each holds)

1. Differential guarantee (`next` forward-converged <-> W3 backward no-shortfall). Total across the cases I tried, including ones the suite did not spell out:
   - Multi-increment step, conflict at a NON-first id-order increment while an earlier increment converged: `select_active_increment` (`src/next.rs:658-679`) checks `has_risk_class_conflict` before the streak on each increment in BTreeMap id order, so a converged inc1 is skipped and the conflicted inc2 is returned as active; `build_in_progress_loop` (`:625-629`) then reports `RiskClassConflict`, never `converged`. W3 (`src/workflow.rs:471-514`) flags inc2's inconsistency as a shortfall. Both non-converged: agree.
   - All-converged fallback (`src/next.rs:677-678`) is reached ONLY when every increment is non-conflicted and peak >= required, which is exactly the condition under which W3 finds no arithmetic shortfall; the active increment then chosen (latest line) is itself converged. So `state == Converged` iff W3 empty. The safety-critical direction (next-converged => no real shortfall) is thus structural.
   - Empty/single record sets: empty `matching` -> `select_active_increment` returns `None` -> `AwaitingFirstReview` (`:594-606`); `records[0]` / `has_risk_class_conflict` are only ever called on non-empty per-increment slices. No panic path.
   - The `next_agrees_with_w3` test (`src/next.rs:1201-1240`) calls the real `workflow::w3_problems` on a `complete` twin step with the identical records, and includes both the single-increment mixed-class fixture and the multi-increment conflict-at-latest fixture; it is non-vacuous (asserts `next_converged == w3_clean`).
   - Waivers: `next` does NOT read waivers (no field on `NextInputs`; `run_next` parses none), so it correctly does not claim to. A waiver only makes W3 MORE lenient, so it can never turn a `next` non-converged into a real shortfall; no false-green is reachable via waivers. The module doc's phrase "exactly a step W3 finds no shortfall on" (`src/next.rs:17`) is scoped by "on the same records" to the arithmetic/conflict verdict; the reverse direction (a human-waivered unconverged step, which W3-if-complete would pass) is `next` being conservatively advisory, not a false green. Judged acceptable, not a defect.

2. W3 extraction behaviour-preserving. `peak_consecutive_clean` (`src/workflow.rs:407-409`) is the old inline `records.iter().map(|round| round.consecutive_clean).max().unwrap_or(0)` moved verbatim (diff confirms the expression is identical); call site `:491` unchanged in meaning. `round_step_slug`/`round_increment_id` only changed visibility to `pub(crate)`. 324 lib unit tests pass, including the existing W3 tests that guard this. No W3 verdict changed.

3. State machine. `derive_in_progress_state` (`src/next.rs:686-704`) evaluates `converged` before `escalate` (`a_converging_round_at_the_cap_converges_not_escalates`, `:1166-1177`, pins this). The `RiskClassConflict` guard fires before any streak verdict (`:625-629`). Each `LoopState` has coherent label/role/transitions/next_action/reminders; `Done` is unreachable in the projection path and pinned by `done_row_metadata` (`:1154-1162`), and its `#[cfg_attr(not(test), allow(dead_code))]` is the correct cfg-split exception (constructed only under `cfg(test)`), so `allow` over `expect` is right here.

4. CLI wiring. Verified on the built binary: `status --ledger-fragment X` without `--resume` errors (exit 2, `requires = "resume"` enforced). `next` with no sources yields a clean partial projection (exit 0). `status --resume` prints the `## RESUME STATE` block verbatim and reports an explicit note when the section is absent. Dual-source `--source`/`--plan` handling mirrors `status` (`src/main.rs` run_next). `extract_resume_state` (`src/next.rs:863-882`) is verbatim, None-when-absent, terminates at the next `## ` (not `### `) heading; all three behaviours are tested (`:1297-1317`) and confirmed live. Output determinism: BTreeMap context, no wall-clock, paths echoed verbatim; golden human + JSON byte-compares present (`:1338-1412`).

5. Test honesty (Principle 11). All 9 transition rows covered (one test each, plus the `done` metadata test). The Q-54 human-input-contract reminder is asserted present at `escalate` and ABSENT at `awaiting-reviewers` (`:1252-1267`); `base_reminders` (`src/next.rs:265-302`) emits it only on the `Escalate` arm. No tautological or mislabelled assertions found; the differential and idempotence tests assert what they claim.

6. Scope discipline. No `src/driver/`, no `reconstruct_loop`, no scheduler/multi-unit fanout, no write-path/JSONL writing, no `WorkflowSpec` extension. Read-only and stateless. R1-2 remains deferred to Stage 2 (not re-raised).

7. Style. No non-ASCII / em-dash / unicode-arrow in `src/next.rs`, `src/main.rs`, `src/workflow.rs` (scanned). The one `allow` is the documented cfg-split case.

8. Regression. `just test` green (324 lib + integration suites all pass). `just clippy --all-targets` clean.

## Prior findings status
R1-1/R2r-1 (false-green convergence) fixed and now total incl. multi-increment; R2-1 (`--ledger-fragment` requires `--resume`) enforced; R2-2 (tautological test) not present; R1-2 correctly deferred. Nothing to re-raise.
