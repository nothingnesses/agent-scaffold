# Triage: workflow-driver Stage 1

Independent triager verdicts on the two reviewers' findings. Read-only pass over the worktree
`.claude/worktrees/wf-driver-stage1` (diff `main..impl/wf-driver-stage1`). Each finding judged on
evidence + severity against the build plan `docs/plans/workflow-driver-stage1.build-plan.md`.

Build state re-confirmed from the reviews: `just test` 323 passed, `just clippy` clean.

## Verdict table

| id   | verdict          | actionable this round? | one-line fix |
|------|------------------|------------------------|--------------|
| R1-1 | VALID            | Yes                    | Guard `next` against mixed-class records so it cannot report `Converged`; add the mixed-class fixture to `next_agrees_with_w3`. |
| R1-2 | VALID-BUT-DEFER  | No                     | Stage 2 `reconstruct_loop` joins the declared increment id; no Stage 1 change. |
| R2-1 | VALID            | Yes                    | Add `#[arg(long, requires = "resume")]` to `StatusArgs.ledger_fragment` (`src/main.rs:421`). |
| R2-2 | VALID-BUT-DEFER  | No                     | Non-blocking test-quality nit; optionally drop or rename the within-process check. |

---

## R1-1 (medium) -> VALID, ACTIONABLE THIS ROUND

The divergence is real and the fixture is reachable. Confirmed against code:

- W3 (`src/workflow.rs:471-481`): per increment, `let class = records[0].risk_class;` then
  `if records.iter().any(|round| round.risk_class != class) { problems.push(...); continue; }`. A
  mixed-class increment NEVER reaches the peak/threshold check, so W3 ALWAYS reports it as a
  shortfall (not-clean).
- `next` (`src/next.rs:594-599`): `let class = records[0].risk_class;` then derives the state with
  NO consistency check. `select_active_increment` (`src/next.rs:626-628`) likewise keys off
  `records[0].risk_class` with no check.
- Trace verified. `required_streak(LowRisk) = 1`, `Risky = 2`, `cap = 5` (`src/workflow_spec.rs:52-54`,
  pinned by the test at `:195-197`). For `round(1, Clean, cc=1, LowRisk)` + `round(2, Clean, cc=1, Risky)`
  in one increment: `next` class = LowRisk, required = 1, peak = max(1,1) = 1, `1 >= 1` ->
  `derive_in_progress_state` returns `Converged` (`src/next.rs:648-649`) and advises
  `mark-step-complete`. W3 sees `any(risk_class != LowRisk)` true (round 2 is Risky) -> pushes the
  inconsistency problem and `continue`s -> not-clean. So `next_converged = true` while
  `w3_clean = false`: a forward/backward disagreement.
- The differential test is vacuous in this dimension: `next_agrees_with_w3` (`src/next.rs:1137-1150`)
  feeds only single-class fixtures (`assert_differential` uses one increment `a-inc`; every `round(...)`
  in a call shares one `RiskClass`), so `assert_eq!(next_converged, w3_clean)` at `:1131` never
  exercises a mixed-class increment and would still pass while the property is false.
- The module doc claim is overstated: `src/next.rs:10-15` asserts "a step that `next` reports
  `converged` is exactly a step W3 finds no shortfall on ... provably identical." That is literally
  false on this reachable input, and the input is not rejected upstream (`parse_rounds` accepts
  mixed-class records; it is a W3 validate-time fault, not a parse error).

Why VALID and actionable, not deferrable: the differential guarantee IS the central correctness
claim of this increment (decision B-a, the whole reason for the shared `peak_consecutive_clean`
helper), and the build-plan test plan item 2 calls the differential the "key acceptance" test. A
reviewer produced a concrete counterexample to a claim the code states as total. On the reachable
input, the advisory forward projection is actively wrong: it advises marking a data-integrity-faulted
step complete. This is capped at medium (not high) because the ENFORCED gate is untouched: no step
passes `validate --workflow` falsely (`w3_problems` still flags it), so the false `Converged` is
caught before it can promote a bad step. But "the enforced gate catches it" does not make the
increment's own stated property true, and closing it is small.

Smallest correct fix (restores the property AND makes the doc claim true):
1. In `build_in_progress_loop` (`src/next.rs:594-599`), before deriving the state, apply the SAME
   guard W3 uses: when `records.iter().any(|r| r.risk_class != class)`, do NOT report `Converged`.
   The differential only needs `next_converged == w3_clean`, and on a mixed-class increment
   `w3_clean` is false, so `next` must not be `Converged`. Routing the mixed-class case to
   `LoopState::Escalate` (the existing non-converged human-gate state) is the most honest choice: it
   is a data fault owed to a human, and it keeps the differential total without adding a new state
   (Stage-2 typed FSM stays out of scope). Any non-converged state satisfies the property; Escalate
   is the recommended one.
2. Add the mixed-class fixture (`round(1, Clean, 1, LowRisk, "a", "a-inc")`,
   `round(2, Clean, 1, Risky, "a", "a-inc")`) to `next_agrees_with_w3` so the test is no longer
   vacuous in this dimension.

Minimum-viable alternative if the guard is judged out of scope: narrow the `src/next.rs:10-15` doc
claim to say the equivalence holds for CONSISTENT-class increments only, and add the same fixture as
a documented known-divergence test. I do NOT recommend this alone: it leaves `next` emitting a false
`Converged` / `mark-step-complete` on reachable input, which is a genuine advisory defect, and it
weakens the increment's stated guarantee rather than honoring it. The guard is the smaller net
change in claimed-behaviour terms.

## R1-2 (low) -> VALID-BUT-DEFER, NOT ACTIONABLE THIS ROUND

The observation is accurate: both `next` (`src/next.rs:588-594`) and W3 read only the LOGGED
`records[0].risk_class`, so a step whose DECLARED `[[step.increment]].risk_class` is `risky` (needs 2)
but whose rounds are logged `low_risk` converges at one clean round, and neither direction notices the
declared-vs-logged mismatch.

Correctly deferred:
- Using the logged class (not the declared one) is deliberate and is the RIGHT call for Stage 1: it is
  what keeps `next` and W3 reading the same value, which is the differential property itself. Reviewer
  1 explicitly blessed this (`opus` review, R1-2 body). Note this deviates from build-plan line 24
  ("declared ... when present else the round records'"), but the deviation is justified and better than
  the plan text: had `next` used the declared class while W3 uses the logged one, the two directions
  would diverge on the threshold. So this is an improvement over the plan, documented in-code at
  `src/next.rs:588-593`.
- The residual (no one joins declared vs logged) is squarely Stage 2 work: the build plan defers the
  typed `reconstruct_loop` join to Stage 2 (build plan lines 7, 39, 45, and the YAGNI list line 80),
  and this does NOT break the differential (both directions agree). Low/informational; flag-forward
  only, no Stage 1 change.

## R2-1 (medium) -> VALID, ACTIONABLE THIS ROUND

Confirmed against code and precedent:
- `StatusArgs.ledger_fragment` (`src/main.rs:420-422`) is annotated `#[arg(long)]` with no
  `requires`. Its help text carries "(with --resume)".
- The sibling flag `StatusArgs.resume` (`src/main.rs:418-419`) is the intended dependency, and the
  established precedent in the SAME file is `ValidateArgs.workflow_spec` (`src/main.rs:398`):
  `#[arg(long, requires = "workflow")]`, with a doc comment giving exactly this rationale ("the flag
  is meaningless without it, and would otherwise leave a malformed spec unparsed and exit 0").
- Consequence: `status --ledger-fragment X` without `--resume` is silently accepted, exits 0, and
  ignores the flag (reviewer 2 reproduced this by running the binary). This violates fail-loud
  (Principle 12) and diverges from the file's own precedent.

Smallest correct fix: change `src/main.rs:421` from `#[arg(long)]` to
`#[arg(long, requires = "resume")]`. clap resolves `requires` by the field id, and `resume` is the
bool flag field on the same struct (the `workflow` precedent requires a bool flag the same way), so
this is correct as written. One-line change; no test churn required (an added negative CLI test is
optional).

## R2-2 (low) -> VALID-BUT-DEFER, NOT ACTIONABLE THIS ROUND

The observation is accurate: `identical_inputs_give_identical_bytes` (`src/next.rs:1328-1333`) calls
the pure `golden_projection()` twice in ONE process and asserts equality. A pure function with no I/O,
randomness, or wall-clock is deterministic within a process by construction, so the assertion is a
tautology and cannot catch cross-run non-determinism, which is what build-plan test item 6 ("same
inputs -> identical bytes across two runs") intends.

Why defer rather than block:
- It is not a code defect. The assertion is TRUE and the test passes; it is a weak proxy, not a wrong
  proxy.
- The real determinism properties are covered elsewhere: structurally (`BTreeMap` context ordering, no
  wall-clock, `.display().to_string()` verbatim paths) and by the golden byte-compare tests
  `golden_human_text` / `golden_json` (reviewer 2 verified these are real `assert_eq!` against
  hardcoded constants). So the vacuous test adds no risk and removing it loses no coverage.

Optional, non-blocking improvement (not required for convergence): either drop the test (the golden
tests own the real property) or rename it to reflect that it checks within-process idempotence of the
renderers, not cross-run determinism. No implementer action needed before convergence.

---

## Summary

- Actionable before convergence: R1-1 (guard `next` against mixed-class records + de-vacuum the
  differential test) and R2-1 (one-line `requires = "resume"`).
- Deferred / non-blocking: R1-2 (Stage 2 declared-vs-logged join) and R2-2 (test-quality nit).
- No finding was judged INVALID; both reviewers' medium findings are real and both low findings are
  accurate observations that are correctly non-blocking for this advisory MVP.
