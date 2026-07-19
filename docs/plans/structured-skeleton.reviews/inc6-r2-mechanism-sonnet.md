# Inc 6 Round 2 mechanism re-review (confirming round)

Reviewer: independent, read-only, fresh eyes. Scope: verify the fix for Inc 6 M-1
(silent false-green when `--workflow` resolved no plan source) and M-2 (regression test
overstated coverage). Fix commit under review: `3b5ed44` "fix: reject --workflow with no
plan source instead of silently passing (Inc 6 M-1/M-2)" on `impl/structured-skeleton-inc6`,
rebased onto main `e06a89b`.

## Verdict: CLEAN (zero findings)

No findings at any severity: no critical, no high, no medium, no low. The fix resolves
M-1 and M-2 correctly and completely, does not break the metrics-missing soft-skip, and
introduces no new false-green or false-RED. Details below.

## M-1 verified fixed

`src/main.rs:841-844` adds the `(None, None, _)` arm that pushes a hard problem into
`problems` (which prints to stderr and `std::process::exit(1)` at `src/main.rs:858-862`)
whenever `--workflow` is requested but neither a TOML-primary `--source` (`toml_primary`)
nor a readable Markdown `--plan` (`plan_contents`) resolved. Reproduced in the worktree
(binary built from HEAD); verbatim runs are in the reply. All three failure shapes now exit
non-zero with the message "--workflow requested but no plan source resolved: pass a
TOML-primary --source or a Markdown --plan":
- (a) no `--source`/`--plan` at all: exit 1.
- (b) typo'd/missing `--source` path: exit 1.
- (c) Markdown-primary `--source` with no `--plan`: exit 1.
The legitimate case `validate --workflow --source <toml-primary>` (no `--plan`) still exits
0 and runs the check ("plan.plan.toml vs workflow.jsonl: workflow invariants hold").

## Full match-arm truth table (independent re-check)

The match at `src/main.rs:804-850` is on `(toml_primary, &plan_contents, &metrics_contents)`,
each `Some`/`None`. Enumerating all 8 combinations (T = Some, N = None), top-down:
- (T,T,T) -> arm1 (`(Some,_,Some)`): TOML check. Correct.
- (T,T,N) -> arm4 (`_`): metrics missing, plan source present, soft-skip exit 0. Intended.
- (T,N,T) -> arm1: TOML check. Correct.
- (T,N,N) -> arm4: metrics missing, TOML source present, soft-skip. Intended.
- (N,T,T) -> arm2 (`(None,Some,Some)`): Markdown check. Correct.
- (N,T,N) -> arm4: metrics missing, Markdown plan present, soft-skip. Intended.
- (N,N,T) -> arm3 (`(None,None,_)`): hard error, exit 1. Correct (M-1 fix).
- (N,N,N) -> arm3: hard error, exit 1. Correct (M-1 fix).

Findings from the table:
- Arm ordering is correct: arm3 `(None, None, _)` precedes the `_` catch-all, so both
  metrics-present and metrics-absent no-plan-source cases are caught by the hard error
  rather than swallowed by the soft-skip. Verified in reproduction (e) and (f): a missing
  metrics log with no plan source still hard-errors (exit 1), the plan-source problem taking
  priority, which is the more fundamental misconfiguration.
- No residual false-green: the only exit-0-without-running path is arm4, which fires
  strictly when a plan source IS present (`toml_primary` Some, or `plan_contents` Some) and
  only the metrics log is absent. That is the pre-existing metrics-missing soft-skip, not new
  and not the M-1 shape. Verified preserved in reproduction (d): TOML source present, metrics
  absent -> "the metrics log is missing; skipping" + exit 0.
- No false-RED: arm3 fires only when there is genuinely no plan source to read, where
  erroring is correct. Legitimate TOML-primary and Markdown-plan cases route to arm1/arm2 and
  still run.
- The reworded `_`-arm message ("--workflow has a plan source but the metrics log is
  missing; skipping the workflow check", `src/main.rs:847-849`) is accurate for every arm4
  case, since all three (T,T,N)/(T,N,N)/(N,T,N) have a plan source present and metrics
  absent. No misleading skip note.

## M-2 verified fixed

`tests/validate_workflow_toml_source_needs_no_plan.rs:88-132` adds
`workflow_with_no_plan_source_hard_errors_instead_of_skipping`, a real negative case that
runs the binary with (a) no source/plan and (b) a typo'd `--source`, asserting
`code != Some(0)` and stderr containing "no plan source resolved". This test fails against
the pre-fix `_`-catch-all (which exited 0), so it would catch a M-1 regression (Principle 11,
tests exercise the code they claim to). The isolation is sound: an empty-but-present
`workflow.jsonl` is written, so the missing thing is only the plan source, distinguishing the
hard error from the metrics-missing soft-skip. The module doc comment (lines 7-13) now
describes exactly the positive and negative directions the test bodies exercise, no longer
overstating coverage. It does not claim to cover the Markdown-primary-source case, and the
test does not; the doc matches the code, so Principle 11 holds.

## Considered and not raised

- The metrics-missing soft-skip (arm4) also exits 0 while running no check, the same shape
  as M-1. Not raised: it predates Inc 6 (not introduced by this fix), round-1's mechanism
  lens examined this code and did not flag it, the confirming-round scope is the M-1/M-2 fix,
  and I have no new evidence the round-1 verdict was wrong. `--metrics` defaults to a present
  path in normal use, and a missing metrics log is the project's "nothing to validate yet"
  state (matching the "nothing to validate" notes for absent plan/source/metrics elsewhere).

## Tooling (run in the worktree, this round)

- `cargo test`: all suites green. Unit: 292 passed, 0 failed. Integration suites all
  "test result: ok" including `validate_workflow_toml_source_needs_no_plan` (2 passed).
- `cargo clippy --all-targets`: clean, no warnings.
