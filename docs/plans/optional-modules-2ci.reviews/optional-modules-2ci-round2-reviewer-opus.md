# optional-modules 2c-i (checks subcommand) - round 2 (confirming) - reviewer: opus

Independent confirming review of branch `impl/inc2ci-checks-cmd`, full diff `38ce9ec..7ea018c` (round-1 impl `76d9b83`, round-1 fix `7ea018c`). Built and attacked the binary in a throwaway worktree at `7ea018c`; the worktree is removed after this review. The main repo was not modified.

## Verdict

NOT a clean round. One `medium` finding: the NEW startup-prune logic (`F-M2`) actively destroys a concurrent same-repo run's live worktree, causing that run to fail spuriously. Demonstrated empirically against the built binary. All other round-1 fixes landed correctly and hold up under attack; one `low` confirms the implementer's own flagged residual.

## Round-1 fixes: confirmed

- `F-M1` (exit-code single-sourcing): confirmed in code (`src/checks.rs:266-276`) and empirically. Failing check -> 1; malformed config -> 1; unreadable config (`.agents/checks.toml` a directory) -> `error: Is a directory (os error 21)` exit 2 (NOT the `io::Result` default 1); no-commits -> 2; non-repo -> 2. `main` calls `exit(error.exit_code())` (`src/main.rs:415-427`). Invariant D holds.
- `F-M3a` (`Stdio::null()` on the check, `src/checks.rs:562`): confirmed. Ran the binary with its own stdin bound to a never-closing FIFO writer; a `cat` check still returned in 54ms exit 0, so the check gets EOF regardless of the parent's blocking stdin. No hang.
- `F-DOC` (bounding the guarantee): the subcommand help (`src/main.rs:291`) and module docs (`src/checks.rs:15-31`) accurately bound isolation to the relative-path case and disclaim a security sandbox. Accurate, with ONE exception noted below (the concurrency claim over-states).
- `F-L1` (absent-config note on stderr): confirmed (`src/main.rs:437-443`); empirically stdout empty, note on stderr, exit 0.
- Lows: empty `paths=[]` runs unscoped (`src/checks.rs:682`); leading `./` stripped in glob (`src/checks.rs:442`); new tests for glob false-negative, TOML-syntax error, stdin, orphan-prune all present and passing.

## Invariants re-checked (empirical)

- Invariant A (no live-tree mutation): confirmed. A `kind="format"` check that overwrites a tracked file then exits 1 left the live file byte-identical (sha256 unchanged); the run still reported the failure. The prune's filter (temp-dir path + `RUNNER_PREFIX`) and git's own refusal to remove the main worktree mean the prune cannot endanger the live tree.
- Invariant B (Drop cleanup + startup self-heal): Drop fires on pass/fail/error; a genuinely orphaned registered runner worktree is reclaimed on the next run (verified via the binary: planted orphan gone, `git worktree list` back to 1). See the medium finding for the flip side.
- Invariant C: unstaged tracked edits and the clean-tree-isolates-HEAD paths pass (unit tests).
- Invariant D: exit 0 iff all ran checks pass; mapping consistent and now tested (see F-M1).

## Gates

- `just test` x3: 148 passed each run, 0 failed (prune touches concurrency; no flake observed).
- `just clippy` and `cargo clippy --all-targets -- -D warnings`: clean.
- ASCII: no non-ASCII bytes in `src/checks.rs`/`src/main.rs`; no unicode in added diff lines.
- Scaffold: only `src/checks.rs` and `src/main.rs` changed on the branch; no pack/scaffold assets touched, so the default scaffold is byte-identical.

## Findings

### M-1 (medium) - prune reclaims a CONCURRENT same-repo run's live worktree, failing it

`src/checks.rs:372` (`prune_orphan_worktrees`), doc claim `src/checks.rs:366-371` and module-doc `src/checks.rs:40-41`.

The prune identifies a reclaimable orphan purely by "registered to THIS repo + path under the temp dir + `RUNNER_PREFIX` name" (`src/checks.rs:382-390`). A live worktree belonging to a _concurrent_ `checks` run on the SAME repo is indistinguishable from a dead orphan under that filter: it is registered to the same repo, under the temp dir, with the same prefix. The prune therefore `git worktree remove --force`s it and `remove_dir_all`s it out from under the running process.

Demonstrated against the built binary. Run A ran a check `sleep 4; cat f.txt >/dev/null` (exit code faithful to whether its worktree survives). While A slept, run B started on the same repo; B's prune removed A's live runner worktree (`git worktree list` dropped to the main worktree only). Run A then failed spuriously:

```
*** run A exit=1 ***
        fail  slow (lint)
--- slow (lint) output ---
cat: f.txt: No such file or directory
```

Blast radius: the concurrent run gets a false check failure (exit 1) that has nothing to do with the code under check. If two runs start near-simultaneously each can prune the other. Worst case is a retryable spurious exit 1, not live-tree corruption (Invariant A still holds), which is why this is medium and not high.

This is not exotic for this module: 2c-ii adds `--with-precommit-hook`, so a hook-driven run overlapping a manual or CI run on the same repo is a realistic collision. The round-1 fix traded the blanket-temp-sweep race (cross-repo / parallel-test, different repos) for this narrower same-repo race; the repo-scoping is a real improvement, but the residual is undocumented and un-flagged, unlike the SIGKILL-before-register residual which IS flagged.

Two coupled issues:

1. Behaviour: the prune cannot tell a live concurrent worktree from a dead orphan. The cheap fix is already latent in the design: `RUNNER_PREFIX` embeds the owning pid (`agent-scaffold-checks-run-{pid}-{nanos}`, `src/checks.rs:657`). Parse that pid and reclaim only when the owner process is NOT alive (e.g. `kill(pid, 0)` / `/proc/<pid>` absent). A genuine SIGKILL orphan has a dead pid; a concurrent run has a live one. (A per-run lock file is an alternative but heavier.)
2. Docs: `src/checks.rs:366-371` claims the prune "makes the runner safe under concurrency". That is only proven for parallel TESTS, which use distinct scratch repos; it is false for concurrent runs on one repo. Narrow the claim to "different repositories / parallel tests", or fix (1) and keep it.

### L-1 (low) - unregistered-temp-dir leak before `git worktree add` registers (confirming flagged residual)

`src/checks.rs:656-659`. The runner temp path is created by `git worktree add` itself; if the process is hard-killed mid-`add` after the directory exists but before registration completes, the dir is not in `.git/worktrees/`, so the repo-scoped, registration-based prune never reclaims it (a permanent temp-dir leak, not a correctness issue). The implementer flagged this honestly. Severity low: narrow window, disk-space only, and OS temp cleaning eventually reaps it. Acceptable as-is; a belt-and-braces option is to also prune stale unregistered `RUNNER_PREFIX` dirs older than some age, but that reintroduces exactly the cross-run filesystem-sweep hazard the fix removed, so leaving it is the right call.

## Soundness of the filter (no false positives on user worktrees)

The "temp-dir + `RUNNER_PREFIX`" filter cannot catch a user's legitimately-named worktree: it would have to live under the system temp dir AND be named `agent-scaffold-checks-run-*`, which is self-inflicted and absurd. Ignoring all prune errors (best-effort) is acceptable: every step is reclamation of a leak, and a failure just defers self-heal to a later run. Cross-repo isolation confirmed empirically (a runner-prefixed orphan registered to repo1 was untouched by a run in repo2).

## Not re-raised (settled/deferred, no new evidence)

Per-check timeout (`F-M3b`), `--dir` cwd vs top-level (`F-6`), and the increment-2-close CHANGELOG are deferred by prior rounds and I found no new evidence to reopen them. Nothing new was introduced by the fix beyond M-1's pre-existing-but-now-relevant concurrency gap.
