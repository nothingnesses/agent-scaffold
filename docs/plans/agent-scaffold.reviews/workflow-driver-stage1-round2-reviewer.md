# Workflow-driver Stage 1: Round 2 re-review (adversarial)

Re-review of the fix round on branch `impl/wf-driver-stage1` (fix commits `9bd8603`,
`e5c8fda`) over the base feat commit `d2180e7`. Read-only: I ran the suite, the binary,
and a scratch-copy experiment; I edited no tracked files.

## Verdict

One finding (medium). The R2-1 and R2-2 fixes are correct and complete. The R1-1 fix is
correct for the single-increment case it targeted, but it does NOT make the claimed
differential guarantee TOTAL: a multi-increment step can still make `next` report
`Converged` while W3 reports a shortfall. Verified empirically.

## Verification of the three fixes

- R2-1 (`9bd8603`, `--ledger-fragment` now `requires = "resume"`): VERIFIED.
  `src/main.rs:421` adds `#[arg(long, requires = "resume")]`. Built the binary and ran it:
  `status --ledger-fragment /tmp/x.md` (no `--resume`) errors with "the following required
  arguments were not provided: --resume" and exits 2; `status --ledger-fragment ... --resume`
  runs and exits 0. Gated as intended. The diff is minimal and touches only the arg
  attribute plus its doc line.

- R2-2 (`e5c8fda`, determinism test renamed): VERIFIED. The test is now
  `the_renderers_are_idempotent_within_a_call` (`src/next.rs:1393-1401`) with a doc comment
  (`src/next.rs:1387-1392`) that explicitly says it is a within-call idempotence check and
  NOT a cross-run determinism check (that being owned by the `BTreeMap` ordering, the
  wall-clock-free paths, and the golden byte-compares). No longer misleadingly named; passes.

- R1-1 (`e5c8fda`, `RiskClassConflict` variant + guard): PARTIALLY effective; see R2r-1.
  Sub-checks:
  - (a) Guard condition matches W3 EXACTLY. `src/next.rs:615,625`:
    `let class = records[0].risk_class; ... records.iter().any(|round| round.risk_class != class)`.
    `src/workflow.rs:474-475`: identical `let class = records[0].risk_class; if records.iter().any(|round| round.risk_class != class)`. Same predicate over the same
    per-increment record grouping. Confirmed.
  - (c) The new state is coherent. `label` -> `risk-class-conflict` (`src/next.rs:196`),
    `role` -> orchestrator (`:210`), `valid_transitions` -> `["fix-risk-class-records"]`
    (`:226`), `next_action` present (`:243-244`), `base_reminders` cite Principle 12/15
    (`:294-297`), `build_context` maps it to the no-extra-slots arm (`:776`),
    `filled_prompt_summary` handled (`:823-826`), and `spawns_writer` correctly EXCLUDES it
    (`:252-260`, so no writer/tier reminder). No dead match arms; `just clippy` is clean.
  - (d) The de-vacuumed fixtures genuinely bite. `risk_class_conflict_row`
    (`src/next.rs:1123-1137`) and the mixed-class case appended to `next_agrees_with_w3`
    (`:1208-1211`) both fail if the guard is removed (I verified the guard's effect by
    building a scratch copy of the crate; with the guard the single-increment conflict
    yields `RiskClassConflict`, without it it would read `Converged`). Non-vacuous.
  - (b), (e): NOT satisfied. See R2r-1.

- Regression sweep: `just test` -> 324 + integration tests pass, 0 failed. `just clippy
  --all-targets` clean. Golden human/JSON byte-compares pass. Behaviour-preservation of the
  W3 extraction (`peak_consecutive_clean`) intact. No new defect from the two commits'
  diffs (R2-1 touches only the one arg attribute; R1-1 touches only `src/next.rs`).

## Findings

### R2r-1 (medium): the differential guarantee is still NOT total; a multi-increment step can make `next` report `Converged` while W3 reports a shortfall

Evidence: `src/next.rs:646-664` (`select_active_increment`) and `src/next.rs:625` (the guard),
vs `src/workflow.rs:471-481` (W3's per-increment loop). Module doc claim:
`src/next.rs:16-18`.

What is wrong: the `RiskClassConflict` guard added by the fix lives ONLY in
`build_in_progress_loop` (`src/next.rs:625`), and it inspects ONLY the SINGLE increment that
`select_active_increment` returns as active. But `select_active_increment`
(`src/next.rs:646-664`) does not check for a risk_class conflict when it decides which
increments count as "converged": for each increment it computes
`required = spec.required_streak(records[0].risk_class)` and skips it when
`peak_consecutive_clean(records) < required` is false (`:656-661`). A conflicted increment
whose `records[0]` class threshold is still met by its peak is therefore treated as
converged and SKIPPED, so the guard never runs on it. When EVERY increment passes that
per-`records[0]` threshold, the all-converged fallback returns the increment of the
latest-line round (`:662-663`); if that latest increment is a genuinely clean
(non-conflicted) one, `build_in_progress_loop` derives `Converged` for it and the conflicted
increment is never examined. W3 (`src/workflow.rs:471-481`), by contrast, iterates ALL of a
complete step's increments and flags the conflicted one. The two verdicts diverge.

The module doc at `src/next.rs:16-18` claims totality: "a step that `next` reports
`converged` is exactly a step W3 finds no shortfall on (the differential property, pinned by
`next_agrees_with_w3`)". That "exactly" is not supported: `next_agrees_with_w3`
(`src/next.rs:1186-1212`) exercises only SINGLE-increment fixtures, so the multi-increment
divergence is untested and the doc claim is dishonest as written.

Reproduced empirically (scratch copy of the crate, guard unchanged; I did not modify any
tracked file). Rounds on complete-in-review step `a`:
```
line 1: increment inc2, Clean, cc=1, risk_class=low_risk
line 2: increment inc2, Clean, cc=1, risk_class=risky      <- inc2 is internally inconsistent
line 3: increment inc1, Clean, cc=1, risk_class=low_risk   <- inc1 converged, latest line
```
inc2's `records[0]` is low_risk (needs 1) and its peak cc is 1, so `select_active_increment`
treats inc2 as converged and skips it; inc1 also converges; the fallback returns inc1 (max
line). Result observed:
```
active_increment=Some("inc1") state=Converged next_converged=true w3_clean=false
```
`next` says `Converged`; `w3_problems` reports inc2's "inconsistent risk_class values". A
real false-green convergence path.

Why it matters: `next` is advisory and the `Converged` state's reminder cites Principle 6
(run the tests/checks first), and `validate --workflow` (W3) would catch the fault
downstream, so this is not an unconditional silent green. But R1-1's stated purpose was to
make the forward/backward verdicts agree TOTALLY, and the module doc now asserts that
totality; both are false for multi-increment steps, which the codebase treats as a real,
supported scenario (W3 groups per increment precisely so a step can converge under two
classes across `-incA`/`-incB`, `src/workflow.rs:429-431`). The narrow trigger (a conflicted
increment that is not the latest-line one, with all increments over their per-`records[0]`
threshold) makes this lower-probability than a blanket false-green, hence medium; note the
review rubric's wording ("a remaining false-green convergence path ... is high/critical")
would support escalating it. A minimal fix would move (or duplicate) the conflict check into
`select_active_increment` so a conflicted increment is never counted as converged, and/or
have `build_in_progress_loop` report `RiskClassConflict` when ANY of the step's increments
is internally inconsistent, not just the selected active one; the `next_agrees_with_w3`
fixture should gain a multi-increment case to pin it.

## Settled findings not re-raised

- R1-2 (declared `[[step.increment]].risk_class` ignored) is correctly deferred to Stage 2;
  not re-raised. The code's choice to read the logged class over the declared one
  (`src/next.rs:350-360,615`) is consistent with the differential-property design.
