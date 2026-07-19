# Workflow-driver Stage 1: Round 2 triage (R2r-1)

Read-only adjudication of the single Round 2 finding. I read the reviewer file, `src/next.rs`,
`src/workflow.rs` `w3_problems`, and the Stage 1 build plan; I traced the reviewer's repro
through the code and inspected the real round log. I edited no code, plan, or ledger.

## Verdict: VALID. Actionable THIS round. Smallest fix: option (i).

R2r-1 is a real defect, not an artefact of the reviewer's setup, and it is fixable within
Stage 1's single-loop scope with a ~3-line change. It is NOT a scope decision to escalate: the
correct fix does not touch the deferred multi-unit/ready-frontier machinery.

## Why VALID (the repro is sound)

Traced the reviewer's repro against the code:

- `select_active_increment` (`src/next.rs:646-664`) iterates increments in `BTreeMap` id order
  and, per increment, skips it when `peak_consecutive_clean(records) >= spec.required_streak(records[0].risk_class)`
  (`:656-661`). A conflicted increment whose `records[0]` class threshold is still met by its
  peak is therefore treated as converged and SKIPPED.
- The line-625 guard (`src/next.rs:625`) inspects ONLY the returned active increment's records,
  so it never runs on a skipped conflicted increment.
- When every increment clears its `records[0]` threshold, the fallback (`:662-663`) returns the
  latest-line increment; if that one is clean, `build_in_progress_loop` derives `Converged`.
- W3 (`src/workflow.rs:471-481`) iterates ALL of a complete step's increments and flags the
  conflicted one as a shortfall.

Reviewer's fixture (`inc1` id-orders before `inc2`): `inc2` = [Clean/low_risk peak1, Clean/risky],
`records[0]`=low_risk needs 1, peak 1 >= 1 -> skipped as converged despite the internal class
conflict; `inc1` = [Clean/low_risk] converged; fallback returns `inc1` (latest line) -> `Converged`.
W3 flags `inc2`'s "inconsistent risk_class values". `next_converged=true`, `w3_clean=false`.
Divergence confirmed by reading the code paths; the reviewer also verified it empirically.

The module doc's totality claim is genuinely false as written: `src/next.rs:16-18` asserts a step
`next` reports `converged` is "exactly" a step W3 finds no shortfall on, and
`next_agrees_with_w3` (`src/next.rs:1187-1212`) exercises only single-increment fixtures, so the
claim is untested at the multi-increment boundary where it breaks.

Adversarial checks against the finding (it survives all three):
- "Advisory only, W3 catches it downstream." True, and the reviewer concedes it. But the
  `Converged` state's sole purpose is to green-light `mark-step-complete`, and R1-1's stated
  purpose plus the module doc make the differential agreement a claimed safety property. A
  false green here defeats exactly that claim.
- "Unreachable." Multi-increment steps are a first-class, supported scenario: W3 groups per
  increment precisely so a step can converge under two classes (`src/workflow.rs:429-431`), and
  the real log already has one (`state-schema` -> increments `state-schema-inc1`/`state-schema-inc2`).
  A conflicted-but-threshold-passing increment is constructible (low_risk `records[0]` needs 1,
  one clean round gives peak 1).
- "The only divergence is unconditional." No: I confirmed the conflict is the ONLY divergence
  source. If any increment has peak < its required, `select_active_increment` returns it first
  and `next` reports a non-converged state, agreeing with W3. `next` reports `Converged` ONLY
  when every increment clears its threshold, which matches W3's peak condition exactly; the sole
  residual gap is a risk_class conflict on a non-selected increment. That narrowness is why it
  is medium, not the reason to dismiss it.

Real-data status: LATENT, not currently live. All real round records keep risk_class consistent
within each task, so no conflict exists today; the multi-increment structure that would trigger
it does exist. So this is a correctness/claim defect to fix now, not a live false green.

## Smallest correct fix: option (i)

Make `select_active_increment` conflict-aware so a conflicted increment can never be silently
skipped as "converged". Add a conflict pre-check inside the existing id-order loop in
`select_active_increment` (`src/next.rs:656`), BEFORE the convergence-skip test:

```rust
for (increment, records) in increments {
    let class = records[0].risk_class;
    if records.iter().any(|round| round.risk_class != class) {
        return Some(increment); // a conflicted increment is never treated as converged
    }
    let required = spec.required_streak(class);
    if peak_consecutive_clean(records) < required {
        return Some(increment);
    }
}
```

This returns the first non-converged-OR-conflicted increment in id order. The existing line-625
guard then fires on the returned conflicted increment automatically (it recomputes
`records[0].risk_class` and checks `any(risk_class != class)`), so `build_in_progress_loop` needs
NO further change and `RiskClassConflict` is reported against the actually-conflicted increment
(the summary/reminders point at the right one). Behaviour for conflict-free inputs is unchanged:
the pre-check returns nothing and the loop falls through to the current logic.

Then extend the differential test: add a multi-increment fixture to `next_agrees_with_w3`
(`src/next.rs:1187`) with a conflicted non-latest increment plus a clean latest one (the
reviewer's shape), asserting `next_converged == w3_clean` (both false). This pins the totality
and fails if the pre-check regresses. The module doc at `src/next.rs:16-18` can keep its
"exactly" wording, which becomes true for any number of increments.

Size/risk: ~3 added lines in one function plus one test; LOW risk, fully within Stage 1's
single-active-loop scope. It is still one loop, one step; it only makes the active-increment
selection over the increments the code already groups conflict-aware. It does NOT add multi-unit
fanout, a ready-frontier scheduler, or any deferred machinery, so it does not cross the build
plan's YAGNI line (`docs/plans/workflow-driver-stage1.build-plan.md:79-80`).

## Why not option (ii)

Option (ii) (narrow the doc/test to the single active increment, defer step-level agreement to
Stage 2) is about the same size as (i) but strictly weaker: it PERMANENTLY accepts a false-green
`mark-step-complete` path for a data-integrity fault, which contradicts Principle 12 (fail loud
on data faults) and re-opens exactly the false-green convergence hole R1-1 was created to close.
Since (i) costs no more and keeps the strong, honest guarantee, prefer (i). Option (iii)
(combination) is unnecessary: (i) alone makes both the code and the doc claim correct.
