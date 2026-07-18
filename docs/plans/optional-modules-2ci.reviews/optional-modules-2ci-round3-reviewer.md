# optional-modules 2c-i (checks) - round 3 (confirming) reviewer findings

Increment: `optional-modules` sub-increment 2c-i, the `checks` subcommand with worktree-isolated execution (`Q-39`). `--staged` / `--with-precommit-hook` are 2c-ii and out of scope.

Scope reviewed: full branch diff `38ce9ec..3f1e247` (branch `impl/inc2ci-checks-cmd`), with focus on the round-2 fix delta `7ea018c..3f1e247` (the pid-liveness gate on the startup orphan prune). Built, tested, and attacked the binary in a throwaway detached worktree at `3f1e247` under the scratch dir; worktree removed after.

## Verdict: CLEAN ROUND. No new findings. The round-2 fix resolves the race and introduces no new one.

The round-2 MEDIUM (startup prune deleting a concurrent same-repo run's worktree) is resolved by gating reclamation on the owning process being dead. I attacked the fix empirically and by inspection and could not break it. Every gate passes.

## What I verified

### The pid-liveness gate resolves the same-repo concurrency race (confirmed empirically)

- 3 overlapping `checks` runs on the SAME repo, each check `sleep 1` to force worktree overlap: all exit 0, all report `2 passed`, no spurious "file not found" / fatal, and `git worktree list` is clean afterward (only the main worktree). Then a heavier stress: 5 parallel x 5 iterations = 25 overlapping runs staggered by 0.1s. All 25 exit 0, no error strings in any output, final worktree list clean. No run clobbered another.
- Invariant B genuine-orphan reclamation still works, tested with a REAL live process (not just the test-process pid): planted one runner worktree owned by a live `sleep 300` pid and one owned by a dead pid (`u32::MAX`), registered to the repo. A run's startup prune left the live-owner worktree intact and reclaimed the dead-owner orphan. This is a stronger check than the in-tree test `a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one`, which models the live owner with the test process's own pid; result matches.

### The gate introduces no new problem (confirmed by inspection + attack)

- Pid parse (`owning_pid`, `src/checks.rs:370-372`): `strip_prefix(RUNNER_PREFIX)` then `split('-').next()` then `parse::<u32>()`. The dir name is `agent-scaffold-checks-run-{pid}-{nanos}`; `pid` is a `u32` decimal and `nanos` a `u128` decimal, neither containing a `-`, so the first segment after the prefix is exactly the pid. Verified empirically for pid `691657` and `4294967295`. Non-numeric or empty first segment parses to `None` -> skip. Sound.
- Conservative default is correct (`src/checks.rs:415-422`): only `Some(pid) if !pid_is_alive(pid)` reclaims; `None` (unparseable) and a live pid both skip. So the worst case is a leaked orphan (reclaimed by a later run), never deletion of a live run's worktree. This is the right direction to fail.
- Bounded to runner worktrees: reclamation requires BOTH `path.starts_with(temp)` (`:407`) AND the `RUNNER_PREFIX` (via `owning_pid`, which returns `None` for any other prefix, `:410-415`). The test-fixture prefix `agent-scaffold-checks-test-` fails `strip_prefix` and is skipped. A non-runner registered worktree cannot be parsed to a pid and reclaimed. Bound holds.
- `/proc/{pid}` liveness (`pid_is_alive`, `src/checks.rs:360-365`) is sound on this target: `/proc` is mounted with no `hidepid` option (checked `/proc/mounts`), so a live pid owned by any user is stat-able and reports alive; a zombie still has a `/proc/{pid}` entry and is conservatively treated as alive (benign delayed reclaim). The `|| !Path::new("/proc").exists()` fallback keeps a `/proc`-less platform conservative (treat unknown as alive). The embedded pid is `std::process::id()` (the thread-group-leader pid), which is exactly what top-level `/proc/{pid}` tracks. Pid reuse is a benign skip, as documented.

### Rest of the isolation core unregressed (A, B, D re-verified empirically; C by test + read)

- Invariant A: a `format` check whose `check` command rewrites a tracked file in place then exits non-zero reports the failure (exit 1) while the live `code.txt` is byte-identical (sha256 unchanged). Live tree not mutated.
- Invariant D exit mapping: all-pass -> 0, one failing check -> 1, not-a-repo -> 2, malformed config -> 1. All as specified. `Io` (unreadable config) -> 2 is pinned by test and by `RunError::exit_code` (`src/checks.rs:266-275`, `src/main.rs:423-434`).
- Invariant B (Drop cleanup on pass/fail and orphan self-heal): confirmed above and by the passing/failing-run tests leaving only the main worktree.
- Invariant C (stash-create captures committed+unstaged tracked state, HEAD when clean, no-commits rejected): unchanged by the round-2 fix; covered by `the_isolated_tree_reflects_unstaged_working_tree_edits`, `a_clean_tree_isolates_head`, `an_empty_repo_with_no_commits_errors`, all green.

### Gates

- `cargo test` run 3x: `149 passed; 0 failed` each time, no flake.
- `cargo clippy --all-targets -- -D warnings`: clean (Finished, no warnings).
- ASCII-clean: `src/checks.rs` and `src/main.rs` contain no non-ASCII bytes.
- Scaffold byte-identical: `git diff --name-only 38ce9ec..3f1e247` lists only `src/checks.rs` and `src/main.rs`; no pack/asset changes.

## Non-findings (noted, not raised)

- hidepid: On a kernel mounting `/proc` with `hidepid=2` (non-default; NOT set on this target), a live process owned by a DIFFERENT user would be invisible in `/proc` and could read as dead, permitting reclamation of that user's worktree. This is out of the same-user concurrent-run contract the fix targets, is not the default, and git's own cross-user worktree permissions would independently block the scenario. Informational only; not a finding on this Linux target.
- The final unconditional `git worktree prune` (`src/checks.rs:427`) and the `WorktreeGuard` Drop prune are unchanged by the round-2 fix and only remove admin entries whose directory is already gone; they did not affect concurrent live runs across 25 overlapping invocations. Pre-existing, not new.
- Settled/deferred items (per-check timeout, `--dir` cwd semantics, the L-1 pre-registration temp-dir residual, CHANGELOG at increment-2 close) were not re-raised; I found no new evidence against any of them.
