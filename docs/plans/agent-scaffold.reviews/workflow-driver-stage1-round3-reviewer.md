# Round 3 Review: workflow-driver-stage1 (commit 37661e6)

Reviewer: independent, adversarial, read-only.
Scope: the R2r-1 fix in commit `37661e6` - conflict-aware `select_active_increment` and the shared `has_risk_class_conflict` predicate.

## Verdict: CLEAN - no findings

All 331 tests pass (`just test`). Clippy is clean (`just clippy`). No defects found.

## Verification record

### 1. Differential guarantee - is it now total?

The guarantee is total: `next` reports `Converged` if and only if W3 finds no shortfall.

`select_active_increment` (`src/next.rs:658-679`) iterates the BTreeMap in id order. For each increment it first calls `has_risk_class_conflict` (line 669); if true, it returns that increment immediately. Only if false does it check `peak_consecutive_clean(records) < required` (lines 672-674). The all-converged fallback (latest-line round, line 678) is reached only when every increment passes BOTH guards.

Consequence: for `next` to report `Converged`, every increment must have no conflict AND peak >= required. Under those same conditions W3 finds no shortfall (it checks the same two conditions over the same records). Contradiction is impossible in either direction.

Additional cases checked:
- Conflicted increment NOT at lowest id: BTreeMap iterates alphabetically; the conflict pre-check fires on the first conflicted increment regardless of position. Confirmed by the `assert_differential` multi-increment case at `src/next.rs:1235-1239`.
- Multiple conflicted increments: the first (alphabetically) is returned active; W3 flags all of them. Both agree it is not converged.
- A conflicted increment whose `records[0].risk_class` would pass the streak check: the conflict pre-check fires BEFORE the peak check (line 669 before lines 672-674), so that increment is never skipped as converged. This is the exact R2r-1 scenario.
- Empty record set: cannot occur - an increment key in the BTreeMap requires at least one round. No panic risk.
- All-converged fallback: the latest-line round's `round_increment_id` identifies the increment. Since all increments converged, this increment also converges, and `build_in_progress_loop` correctly reports `Converged`.

### 2. `has_risk_class_conflict` vs the W3 predicate

W3 at `src/workflow.rs:474-475`:
```
let class = records[0].risk_class;
if records.iter().any(|round| round.risk_class != class) {
```

`has_risk_class_conflict` at `src/next.rs:646-649`:
```
fn has_risk_class_conflict(records: &[&Round]) -> bool {
    let class = records[0].risk_class;
    records.iter().any(|round| round.risk_class != class)
}
```

The logic is byte-for-byte identical: same reference record (index 0), same predicate (`any` with inequality). No mismatch.

The function is called in two places: `build_in_progress_loop` at `src/next.rs:625` and `select_active_increment` at `src/next.rs:669`. Both calls use the same function body, so they cannot drift from each other or from W3.

### 3. Multi-increment fixture non-vacuity (`src/next.rs:1235-1239`)

The fixture:
- inc1: rounds 1 (LowRisk, clean, cc=1) and 2 (Risky, clean, cc=1) - conflicted, peak=1
- inc2: round 3 (LowRisk, clean, cc=1) - clean, peak=1, required=1

W3 analysis: inc1 has inconsistent risk_class -> shortfall; inc2 is clean. One problem total, w3_clean=false.

`next` analysis with the fix: `select_active_increment` sees inc1 first (alphabetical), `has_risk_class_conflict` fires -> returns inc1. `build_in_progress_loop` also detects the conflict -> `RiskClassConflict`. next_converged=false. Both agree: NOT converged.

Without the conflict pre-check (regression scenario): inc1's `records[0].risk_class=LowRisk`, required=1, peak=1, so `peak < required` is false -> skip as converged. inc2: no conflict, peak=1 >= required=1 -> skip. All-converged fallback: latest-line round is 3 (inc2) -> returns inc2. `build_in_progress_loop(inc2)`: no conflict, peak=1 >= required=1 -> `Converged`. next_converged=true, w3_clean=false -> `assert_differential` fails. Fixture is genuinely non-vacuous.

### 4. Non-conflict selection behavior preserved

The conflict pre-check at `src/next.rs:669-671` is a new early-return path that fires only when `has_risk_class_conflict` is true. The existing peak check at lines 672-674 is unchanged and executes on the same path as before for all conflict-free increments. The all-converged fallback at line 678 is also unchanged. No regression in the non-conflict cases.

### 5. Ordering soundness

BTreeMap iteration is lexicographic (deterministic). Within this ordering, a conflicted increment is returned before any later clean increment is examined. This cannot produce a wrong active-loop choice for non-conflict cases: a conflict-free increment is never mistakenly identified as conflicted (the predicate requires at least two records with differing `risk_class`).

### 6. Test and lint

`just test`: 324 unit tests + 7 integration tests, all pass (0 failed).
`just clippy`: clean, no warnings.

Goldens intact: `next::tests::golden_human_text` and `next::tests::golden_json` both pass.
`RiskClassConflict` state metadata (label/role/valid_transitions/next_action/reminders) verified at `src/next.rs:186-302`; all arms present and consistent.

### 7. Style

New code at `src/next.rs:641-679` and `src/next.rs:1219-1239`: no em-dashes, no unicode, no emoji, ASCII only.

The pre-existing `#[cfg_attr(not(test), allow(dead_code))]` on `LoopState::Done` (`src/next.rs:181-182`) uses `allow` rather than `expect` - this is a cfg-split case (constructed only in tests, dead in the release build) where `allow` is correct per the project's convention.
