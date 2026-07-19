# Reviewer (opus) findings: workflow-driver Stage 1

Lens: W3 behaviour-preservation + convergence/transition correctness (Reviewer 1 of 2).
Artifact: `main..impl/wf-driver-stage1` (HEAD `2b8f535`), reviewed in the worktree.
Build state: `just test` -> 323 passed, 0 failed; `just clippy` -> clean.

## Summary
- 1 medium: the differential guarantee (`next` converged <-> W3 no-shortfall) is NOT total; it breaks on within-increment `risk_class` inconsistency, and the `next_agrees_with_w3` test is vacuous in exactly that dimension.
- 1 low: `next` ignores the declared `[[step.increment]].risk_class` entirely; the deviation is correct for the differential, but a declared-vs-logged mismatch is silently invisible to both directions.
- The W3 extraction itself (decision B, the safety-relevant touch) is genuinely behaviour-preserving. The `converged`-before-`escalate` order and the state boundaries are correct.

---

## R1-1 (medium): the differential guarantee drifts on within-increment risk_class inconsistency; the differential test does not cover it

Evidence:
- `src/workflow.rs:474-481` (W3): `let class = records[0].risk_class;` then `if records.iter().any(|round| round.risk_class != class) { problems.push("... inconsistent risk_class ..."); continue; }`. W3 treats an increment whose records disagree on `risk_class` as a data-integrity fault: it pushes a problem and `continue`s, so it NEVER reaches the peak/threshold check. Such an increment is ALWAYS a W3 shortfall (not-clean).
- `src/next.rs:594-599` (`build_in_progress_loop`): `let class = records[0].risk_class;` then `required = spec.required_streak(class)`, `peak = peak_consecutive_clean(records)`, `state = derive_in_progress_state(...)`. There is NO consistency check. `next` silently keys off `records[0].risk_class` and can report `Converged`.
- `src/next.rs:626-628` (`select_active_increment`) likewise uses `records[0].risk_class` per increment with no consistency check.

Divergent fixture (single in-progress step `a`, single increment `a-inc`, two rounds):
- `round(1, Clean, consecutive_clean=1, LowRisk)`
- `round(2, Clean, consecutive_clean=1, Risky)`

Trace:
- `next`: class = `records[0]` = LowRisk; required = 1 (`workflow_spec.rs:52`); peak = max(1,1) = 1; `1 >= 1` -> `derive_in_progress_state` returns `Converged` (`src/next.rs:648-649`). `next` advises `mark-step-complete`.
- `w3_problems`: class = LowRisk; `records.iter().any(risk_class != LowRisk)` is true (round 2 is Risky) -> pushes the inconsistency problem and `continue`s -> `problems` non-empty -> not-clean.

So `next` reports `Converged` (forward) while W3 reports a shortfall (backward). This is the exact forward/backward disagreement the lens asked me to construct.

Why the test misses it: `next_agrees_with_w3` (`src/next.rs:1134-1151`) only feeds increments whose records share one `risk_class`, so it is vacuous in this dimension and would still pass while the property is false. The module doc (`src/next.rs:10-15`) and the build plan's test-plan item 2 both claim the equivalence is total ("a step that `next` reports `converged` is exactly a step W3 finds no shortfall on"); that claim is overstated given this gap.

Why it matters: the reachable input is a metrics log with mixed `risk_class` within one increment. `parse_rounds` does not reject this (it is precisely a W3 validate-time fault, not a parse error), so the log CAN contain it. On such data `next` gives a false `Converged` verdict and advises marking the step complete, papering over an integrity fault. The enforced W3 gate is unchanged and still catches it at `validate --workflow` time (so no step passes the ENFORCED gate falsely; the Converged reminder Principle 6 at `src/next.rs:269-272` also points at running the checks), which is why this is medium rather than high, but the advisory forward projection is wrong and the claimed structural equivalence does not hold. Cheapest fix that restores the property: have `next` apply the SAME inconsistency guard W3 does before deriving the state (report a non-converged/fault state when `records.iter().any(|r| r.risk_class != class)`), and add that fixture to `next_agrees_with_w3`.

## R1-2 (low): declared `[[step.increment]].risk_class` is ignored entirely; a declared-vs-logged mismatch is invisible to both directions

Evidence: `src/next.rs:333-339` and `:588-593` document (correctly) that the declared class is deliberately not carried, so the threshold comes from `records[0].risk_class`, the same value W3 reads. This is the right call for the differential property and for TOML/Markdown parity, and deviation #2 is consistent between the two directions (so it does NOT cause a forward/backward divergence).

The residual: because BOTH `next` and W3 read only the logged class, a step whose declared increment is `risky` (needs 2) but whose rounds are logged `low_risk` converges at a single clean round, and neither the forward projection nor the backward W3 check notices the declared/logged disagreement. This is a latent data-fault that is out of Stage 1 scope (Stage 2 `reconstruct_loop` joins the declared id), and it does not break the differential, so it is low/informational, not a blocker. Flagging so it is not lost when Stage 2 joins the declared increment.

---

## Cleared in my lens (checked, no defect)

- W3 extraction behaviour-preservation (`src/workflow.rs:407-408`, `:491`): the extracted `peak_consecutive_clean(records: &[&Round])` body is `records.iter().map(|round| round.consecutive_clean).max().unwrap_or(0)`, byte-identical in effect to the old inline expression at the call site. The call site passes `&Vec<&Round>` which coerces to `&[&Round]`; iteration yields `&&Round` and auto-derefs to `consecutive_clean` exactly as before. Empty slice -> `unwrap_or(0)` -> 0 (unchanged). Peak-vs-terminal semantics preserved. Visibility widening (`round_step_slug`/`round_increment_id`/`w3_problems`/`peak_consecutive_clean` -> `pub(crate)`) changes no behaviour. No other caller of the old inline expression exists.
- Convergence before cap (`src/next.rs:648-653`): `peak >= required` -> `Converged` is checked BEFORE `total_rounds >= round_cap` -> `Escalate`, matching pack/AGENTS.md and the build plan's transition table. Pinned by `a_converging_round_at_the_cap_converges_not_escalates` (`src/next.rs:1099-1110`).
- State boundaries (`src/next.rs:641-659`, `derive_in_progress_state`): peak >= required -> converged; else cap reached -> escalate; else last round `NewValid` -> awaiting-fixes, `Clean` -> awaiting-reviewers. No-rounds -> awaiting-first-review (`:573-585`). Matches the table row-for-row; each row has a passing test.
- Differential direction (non-converged): for a consistent-class increment, `next` reports non-converged iff the selected active increment has peak < required, which is exactly a W3 shortfall; when every increment converges `next` reports converged and W3 finds no shortfall. The only divergence is R1-1's inconsistency case.
- `select_active_increment` (`src/next.rs:616-634`): deterministic (BTreeMap id order), picks the first unconverged increment else the latest-line record's increment; matches the build plan's active-increment rule.
