# optional-modules 2c-i (checks) - round 4 (final confirming) reviewer findings

Increment: `optional-modules` sub-increment 2c-i, the `checks` subcommand with worktree-isolated execution (`Q-39`). `--staged` / `--with-precommit-hook` are 2c-ii and out of scope.

Scope reviewed: full branch diff `38ce9ec..3f1e247` (branch `impl/inc2ci-checks-cmd`), holistic read of both changed files (`src/checks.rs`, `src/main.rs`). Built, tested, and attacked the binary in a throwaway detached worktree at `3f1e247` under the scratch dir; worktree removed after.

## Verdict: CLEAN ROUND. No new findings.

This is the second consecutive clean round on a risky increment. The increment is sound and ready to converge.

## What I verified

### Gates

- `just test`: 149 passed, 0 failed, 0 ignored. Run once (round 3 ran it 3x with no flakes; not repeated here).
- `cargo clippy --all-targets -- -D warnings`: clean.
- ASCII-clean: `grep -Pq '[^\x00-\x7F]'` on both files returns no match.
- Scaffold byte-identical: `git diff --name-only 38ce9ec..3f1e247` lists only `src/checks.rs` and `src/main.rs`.

### All four invariants verified empirically

A (no live-tree mutation): a format check whose `check` command overwrites a tracked file then exits 1 reports `fail` with exit code 1 while the live file is byte-identical to before the run. Confirmed with a quick shell script.

B (worktree cleanup on pass and fail): `git worktree list | wc -l` = 1 (only the main worktree) after both a passing and a failing run. Confirmed.

C (isolated tree = working-tree state): a lint check that greps for `VERSION=2` in a file that has `VERSION=1` committed but `VERSION=2` unstaged passes (the isolated tree carries the unstaged edit), while the live file is still `VERSION=2` after the run. Confirmed.

D (exit codes 0/1/2): all five cases confirmed empirically - 0 on all-pass, 1 on one-failing-check, 1 on malformed config, 2 on not-a-repo, 0 on absent config.

### Holistic code pass - areas checked

**Config parse -> dispatch -> run -> report -> exit-code path**: The `run` -> `parse` -> `runnable_for` -> `any_tracked_matches` -> `run_command` chain is internally consistent. Early-exit paths (absent config, all-skipped) return correct `Report` values without touching git.

**Error message quality**: all `RunError::Display` arms are self-contained and actionable. `NotARepo` names the directory and explains why git is needed. `NoCommits` tells the user what to do. `WorktreeSetup` includes the git command that failed and its stderr. `GitUnavailable` and the `sh`-not-found case are similarly specific.

**`#[allow]` vs `#[expect]` uses** (`src/checks.rs:131,136`): `budget` and `threshold` are correctly `#[allow(dead_code)]` rather than `#[expect]`. Both fields are accessed only in `#[cfg(test)]` assertions, so the non-test build sees them as dead and the test build sees no lint to satisfy; `#[expect]` would fail the test build. The documented cfg-split exception in the project's CLAUDE.md applies. Correct as-is.

**`glob_rec` correctness**: traced `**/*.py` against `a.py` (zero directories), `pkg/sub/a.py` (two directories), and the negative case `a.rs`. The `**/` branch tries `glob_rec(after, s)` first (zero-directory match), which resolves `*.py` against `a.py` via the single-`*` branch consuming one non-`/` char at a time. All cases compute correctly; the test assertions at lines 852-865 are sound.

**`run_checks` output routing** (`src/main.rs:440-476`): pass/skip lines and the summary line go to stdout via `println!`. The per-check fail label also goes to stdout. The captured command output (the diagnostic evidence) goes to stderr via `eprintln!`. This separation is correct: a caller can capture stdout for the structured report and let stderr carry diagnostics.

**`Report::success` and `ran_any` edge cases**: an empty `checks.toml` (zero entries) produces `config_present: true`, `results: vec![]`, `ran_any() = false`, `success() = true`. The CLI then prints `checks: nothing to run (0 skipped)` and exits 0. Sensible.

**`Runnable::Run(_) => CheckStatus::Skipped(String::new())` unreachable arm** (`src/checks.rs:659-660`): the empty-reason `Skipped` is the fallback under the `!anything_runnable` guard. The arm is unreachable by logic (the guard means no `Runnable::Run` reached this point) and the comment says so. Using a silent default rather than `unreachable!()` is defensive (no panic on hostile input). Acceptable; not a finding.

**Public surface**: `Kind`, `Check`, `ParseError`, `parse`, `CheckStatus`, `CheckResult`, `Report`, `RunError`, `run` are the items used by `main.rs`. Everything internal (`ChecksFile`, `WorktreeGuard`, `git`, `pid_is_alive`, `owning_pid`, `prune_orphan_worktrees`, `isolation_commit`, `any_tracked_matches`, `runnable_for`, `run_command`, `nanos`) is private. Surface is correctly scoped.

**No dead code or unclosed TODOs**: no `TODO`, `FIXME`, or `HACK` comments in either file. No unused `pub` items.

**Multi-pattern `paths`**: a check with `paths = ["**/*.py", "**/*.rs"]` runs when any tracked file matches either pattern. Confirmed empirically.

**`run_checks` exit path through `io::Result`**: `std::process::exit(1)` is called directly rather than propagating through `io::Result`, which would produce a "Error:" banner and exit 1 via the default `io::Error` handler. The comment at `src/main.rs:423-434` documents this distinction correctly.

### Prior round settled/deferred items

None re-raised. No new evidence against any settled or deferred item was found.
