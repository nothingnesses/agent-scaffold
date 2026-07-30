# Review: `checks-runner-worktree-name-collision` (adversarial reviewer)

Commit under review: `b890c4a`, range `main..b890c4a`, `src/checks.rs` only (+212/-26). Reviewed in a detached worktree at `b890c4a`; the tree was clean before and after every experiment below.

Environment: git 2.54.0, cargo 1.98.0-nightly, rustc 1.98.0-nightly, Linux. All commands are run from the worktree root with the project toolchain loaded (`direnv allow && eval "$(direnv export bash)"`).

**5 findings: 3 medium, 2 low. No critical and no high findings.**

**Verdict on the property: the uniqueness property holds.** The reserved path is exclusive by construction (`fs::create_dir` succeeds only for the creator), the pid remains the first `-`-separated component, and the property test is genuinely red against the pre-fix construction. The findings below are about what is NOT covered and what the change silently changed around the edges, not about the property failing.

---

## What I verified before looking for defects

These are confirmations, not findings, recorded so the triager does not have to redo them.

1. **The pid stays first, and the prepend mutation really does reclaim a live worktree.** Mutating `src/checks.rs:461` from `format!("{RUNNER_PREFIX}{pid}-{}-{seq}", nanos())` to `format!("{RUNNER_PREFIX}{seq}-{pid}-{}", nanos())` fails two tests:

   ```
   test checks::tests::a_reserved_path_still_carries_its_owning_pid_as_the_first_component ... FAILED
   test checks::tests::a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one ... FAILED
   panicked at src/checks.rs:1684:13: assertion `left == right` failed: the owning pid must
     still parse out of agent-scaffold-checks-run-0-4294967295-1785419865353916487
   panicked at src/checks.rs:1600:9   (assert!(live.exists(), "a live owner's worktree must not be reclaimed"))
   ```

   The implementer's claim is exact: with `seq` first, `owning_pid` returns 0, `/proc/0` does not exist, so `pid_is_alive(0)` is false and the prune at `src/checks.rs:532-537` removes a LIVE run's worktree. Reverted.

2. **One construction site.** `grep -rn "RUNNER_PREFIX" src/ tests/ pack/` shows exactly one `format!` building the name, at `src/checks.rs:461`, and `grep -rn "agent-scaffold-checks-run" .` outside `.git/` hits only `src/checks.rs` and this plan's own documents. The linkage argument in the property test's comment (`src/checks.rs:1618-1620`) holds.

3. **The property test is red against the pre-fix construction.** Replacing the body of `reserve_runner_worktree` with the pre-fix `Ok(temp.join(format!("{RUNNER_PREFIX}{pid}-{}", nanos())))` fails `concurrent_reservations_never_share_a_runner_worktree_path` on 5 of 5 runs, with 25, 30, 35, 37, and 64 shared paths out of 2000. Reverted. This is the load-bearing red-before-green demonstration and it is real.

4. **Determinism, stated with its bound.** 40 consecutive runs of `cargo test --bin agent-scaffold checks::` (29 tests each) were clean, 0 failures. Against the one measured pre-fix rate of 1 failure in 6, `(5/6)^40 = 6.8e-4`, so this supports better than 99.9% confidence that a 1-in-6 mode is gone. This is the weak half of the evidence; item 3 is the strong half.

5. **Cross-process concurrency is clean end-to-end.** 5 rounds of 8 concurrent `agent-scaffold checks` processes on one repo sharing one `TMPDIR`: 0 non-zero exits, 0 leftover `agent-scaffold-checks-run-*` entries, and `git worktree list` back to 1 entry. The pid-liveness gate never reclaimed a live peer.

6. **Retry exhaustion is effectively unreachable and correctly mapped.** Within a process the name is fresh on every attempt (monotonic `seq`), so `AlreadyExists` requires a foreign holder; 16 consecutive such holders is not a realistic state. The exhaustion error at `src/checks.rs:471-478` is self-describing and reaches the user as `error: <message>` with exit 2 through `RunError::Io` (`src/main.rs:718-719`). No finding on the bound itself.

7. **No CHANGELOG entry is owed.** `CHANGELOG.md` carries the whole `checks` module under `## [Unreleased]` -> `### Added`; it has never shipped, so there is no released behaviour to record a `Fixed` against. The implementer's reasoning is correct.

8. `cargo clippy --all-targets` is clean at `b890c4a`.

---

## Finding 1: the module's Invariant B, and two doc comments that restate it, are now false for the orphan shape this change creates

**Severity: medium**

The change creates a directory at `src/checks.rs:462` before registering it at `src/checks.rs:887`. A SIGKILL in that window leaves a temp directory that is NOT a registered worktree. `prune_orphan_worktrees` (`src/checks.rs:514-545`) only ever iterates `git worktree list --porcelain` output, so it cannot see such a directory, and the trailing `git worktree prune` (`src/checks.rs:544`) only removes admin entries, never unregistered directories. That orphan is therefore never reclaimed, by this run or any later one.

Three doc statements assert the opposite and were not updated:

- `src/checks.rs:37-41` (module Invariant B): "A hard kill (SIGKILL) cannot run `Drop`, so it can orphan a worktree **and its temp directory**; the next run reclaims such orphans with a startup prune (see `prune_orphan_worktrees`), so 'always removed' holds across runs rather than unconditionally within one."
- `src/checks.rs:324-326` (`WorktreeGuard`): "`Drop` cannot run on a hard kill (SIGKILL), which is why the runner also does a startup prune (see `prune_orphan_worktrees`) to reclaim a worktree orphaned by a prior killed run."
- `src/checks.rs:816-817` (`run`): "removed on every return via the `Drop` guard (a hard kill is the one gap, reclaimed by the startup prune on the next run)."

The leak itself was acknowledged in the step's discussion, but it is written down nowhere in the code, and the three claims above are left unqualified in a module whose header explicitly promises honest scope statements ("Scope of the guarantee (stated honestly for a risky increment)", `src/checks.rs:15`).

**Reproduction** (no code change needed; `$SP` is any scratch dir, `$W` the worktree root):

```bash
cd "$SP" && rm -rf demo tmpL && mkdir -p demo tmpL && cd demo
git init -q . && mkdir -p .agents
printf '[[check]]\nname = "ok"\nkind = "lint"\ncommand = "true"\n' > .agents/checks.toml
printf 'x\n' > file.txt && git add . && git -c user.email=a@b -c user.name=t commit -qm init
# the SIGKILL-between-create_dir-and-add shape: unregistered, dead owner (u32::MAX)
mkdir "$SP/tmpL/agent-scaffold-checks-run-4294967295-1785000000000000000-0"
# the shape the prune does handle: registered, dead owner
git worktree add --detach "$SP/tmpL/agent-scaffold-checks-run-4294967295-1785000000000000001-1" HEAD
TMPDIR="$SP/tmpL" "$W/target/debug/agent-scaffold" checks --dir "$SP/demo"
ls "$SP/tmpL"
```

Observed:

```
before:  ...-1785000000000000000-0   ...-1785000000000000001-1
after:   ...-1785000000000000000-0
```

The registered orphan is reclaimed; the unregistered one survives, and survives every subsequent run.

**Assessment of the trade the implementer argued.** The trade is sound in direction: an unregistered empty directory is a smaller artifact than a registered worktree holding a full checkout, and the window is a few microseconds against a whole run. But it is not a strictly smaller leak, because the new one is permanent while the old one self-healed, so it accumulates without bound across a machine's lifetime. Either half of the fix would settle it:

- Qualify the three statements above so the invariant stops over-promising, or
- Give the prune a second, dead-owner-gated pass over `temp_dir()` entries matching `RUNNER_PREFIX` that are not in the registered set. It is safe for the same reason the existing gate is safe: a concurrent reservation is owned by a live pid, so it is skipped, and `reserve_runner_worktree` is now the only producer of these names.

At minimum the doc must stop claiming what the code no longer does.

---

## Finding 2: `create_dir` turns a `TMPDIR` that does not exist from working into a hard failure, and the failure now reports no context

**Severity: medium**

`reserve_runner_worktree` uses `fs::create_dir` (`src/checks.rs:462`), which does not create leading directories. `git worktree add` does (it runs the equivalent of `mkdir -p`). So a `TMPDIR` pointing at a directory that does not exist used to work and now fails the run. Its doc comment claims otherwise, at `src/checks.rs:453-455`: "so reserving first costs one `mkdir` and **changes nothing downstream**."

Separately, the failure is now reported with no context at all. A non-`AlreadyExists` error is returned raw at `src/checks.rs:468`, becomes `RunError::Io` through `From<io::Error>` (`src/checks.rs:314-318`), and `Display` for that variant is a bare `write!(f, "{error}")` (`src/checks.rs:309`). The same condition previously surfaced as `RunError::WorktreeSetup`, which names the operation and carries git's stderr including the full path.

**A/B evidence.** Same scratch repo, same command, only `src/checks.rs` swapped between `main` and `b890c4a`:

| `TMPDIR` condition | pre-fix (`main`) | post-fix (`b890c4a`) |
| --- | --- | --- |
| nested path that does not exist | exit 0, checks run, git creates the directories | `error: No such file or directory (os error 2)`, exit 2 |
| exists but is read-only (`chmod 555`) | `error: could not set up the isolation worktree: ``git worktree add`` failed: fatal: could not create leading directories of '<full path>': Permission denied`, exit 2 | `error: Permission denied (os error 13)`, exit 2 |

Reproduce with:

```bash
# post-fix (tree as committed)
cargo build
rm -rf "$SP/tmpB"; TMPDIR="$SP/tmpB/sub" ./target/debug/agent-scaffold checks --dir "$SP/demo"; echo "exit=$?"
mkdir -p "$SP/roA" && chmod 555 "$SP/roA"
TMPDIR="$SP/roA" ./target/debug/agent-scaffold checks --dir "$SP/demo"; echo "exit=$?"
# pre-fix, for comparison; restore afterwards with: git checkout b890c4a -- src/checks.rs
git checkout main -- src/checks.rs && cargo build
rm -rf "$SP/tmpA"; TMPDIR="$SP/tmpA/sub" ./target/debug/agent-scaffold checks --dir "$SP/demo"; echo "exit=$?"
TMPDIR="$SP/roA" ./target/debug/agent-scaffold checks --dir "$SP/demo"; echo "exit=$?"
git checkout b890c4a -- src/checks.rs && cargo build
```

The trigger is narrow (`/tmp` exists everywhere; it takes an explicitly set `TMPDIR` naming a missing directory, which does happen with per-session or CI-provided temp paths). The impact when it does trigger is a shipped CLI that stops working with a message naming neither the path, nor the operation, nor the fact that a temp directory was involved. Both halves are one-line fixes: `fs::create_dir_all(&temp)` once before the loop (the leaf `create_dir` must stay `create_dir`, that is where the exclusivity lives), and mapping the reservation failure through `RunError::WorktreeSetup` or an `io::Error` that carries the path. The comment at `:453-455` needs correcting either way. Nothing in the test suite covers either behaviour.

---

## Finding 3: neither layer of the fix is pinned by a test; the reservation can be removed outright and the whole suite stays green

**Severity: medium**

The fix's own doc comment names the `create_dir` reservation as the thing that "makes the returned path exclusively ours" and as "the ONLY cross-process discriminator where the pid is a fixture's dead constant" (`src/checks.rs:444-449`). Nothing tests it.

**Mutation D (the realistic regression).** Change `src/checks.rs:462` from `fs::create_dir(&path)` to `fs::create_dir_all(&path)`. That is exactly the change a later reader makes to silence a spurious `AlreadyExists`, and it destroys the guarantee: `create_dir_all` succeeds on an existing directory, so two callers with the same name both proceed and share it. Full `cargo test`:

```
test result: ok. 369 passed; 0 failed; ...
test result: ok. 5 passed; ...   (and the four other integration targets, all ok)
```

Green. Reverted.

**Mutation E (the retry branch is never executed).** Replace the arm at `src/checks.rs:467` with `panic!("MUTATION PROBE: retry branch reached")`. Full `cargo test` is green, so no test in the repository ever drives `create_dir` to `AlreadyExists`. By the same argument the exhaustion return at `src/checks.rs:471-478` is unreachable in the suite. Reverted.

**Mutation B (the sequence is unpinned too).** Change `src/checks.rs:460` from `NEXT_RUNNER_SEQ.fetch_add(1, Ordering::Relaxed)` to `NEXT_RUNNER_SEQ.load(Ordering::Relaxed)`, so `seq` is a constant 0. `cargo test --bin agent-scaffold checks::` is green on 3 of 3 runs. Reverted.

So each layer individually can be neutralised with the suite green; only removing BOTH is caught. That is inherent to belt-and-braces and is not itself wrong, but the consequence is concrete: `concurrent_reservations_never_share_a_runner_worktree_path` is billed in its own comment as covering "the production runner and the prune fixtures alike", and it does not detect the loss of the only mechanism that protects the two `dead_pid()` fixtures against a second concurrent `cargo test` process. This is the shape AGENTS.md names as "tests must actually exercise the code they claim to", applied to the fix's own load-bearing mechanism on a step classified RISKY.

Note the one place a layer-2 removal is caught today is accidental and useless as a signal. Removing the `create_dir` call entirely (mutation C: `match Ok::<(), io::Error>(())`) fails one test, but only on a cleanup line:

```
panicked at src/checks.rs:1690:31: called `Result::unwrap()` on an `Err` value:
  Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

That is `fs::remove_dir(&dead).unwrap()` tripping over a directory that was never created. It names nothing about reservations, and it does not fire for mutation D, which is the mutation that actually matters.

A direct test is cheap: assert that a reserved path exists and is a directory on return, and that a second `fs::create_dir` on the same path fails with `AlreadyExists`. That pins layer 2 in three lines and would kill mutations C, D, and E.

---

## Finding 4: the `WorktreeGuard` move is load-bearing and pinned by no test

**Severity: low**

Moving the guard construction to before `git worktree add` (`src/checks.rs:879-885`) is the one change beyond the naming line, and it is correct and necessary. Both halves demonstrated:

**It is necessary.** With the commit argument at `src/checks.rs:887` forced to `"no-such-commit-xyz"` so the add fails:

- Guard in its committed position (before the add): `error: could not set up the isolation worktree: ... fatal: invalid reference: no-such-commit-xyz`, and the temp dir is empty. Nothing leaks.
- Guard moved back to its pre-change position (after the `if !added.status.success()` block): same error, and the temp dir contains `agent-scaffold-checks-run-1212838-1785420615511409222-0`. That is a permanent leak, unregistered and so not reclaimable by the prune (see Finding 1).

**It is untested.** With the commit argument restored and the ONLY change being the guard moved back to its old position, the full `cargo test` is green: `369 passed; 0 failed` plus all five integration targets. Reverted; `git status` clean.

So a later refactor that moves the guard back, or that inserts a fallible statement between `src/checks.rs:878` and `src/checks.rs:882`, reintroduces the leak with CI green. A test that forces the add to fail and asserts the temp directory is gone would pin it. Rated low because the code is correct as committed; the exposure is regression risk on a path the change itself just made leak-prone.

---

## Finding 5: the fixtures now create a temp directory before their first fallible step, so a failing assertion leaks an unreclaimable directory

**Severity: low**

Pre-change the fixtures only built a path string; nothing existed on disk until `git worktree add`. Now `reserve_runner_worktree` creates the directory first, and the cleanup runs only after the assertions:

- `src/checks.rs:1680-1691`: `reserve_runner_worktree` twice, then an assertion loop, then `fs::remove_dir(&dead).unwrap()` and `fs::remove_dir(&live).unwrap()`. A failing `assert_eq!` at `:1684` leaks both directories.
- `src/checks.rs:1561-1562` and `src/checks.rs:1591-1594`: a `git_ok` panic between the reservation and the `worktree add` leaks the reserved directory.
- `src/checks.rs:1645-1654`: the property test reserves up to 2000 directories and only removes them after `taker.join().expect(...)`; a panicking thread leaks every path from the remaining threads.

Each leak is an empty directory that nothing reclaims (Finding 1), so it is temp-dir litter that only accumulates on already-failing runs. The property test already gets this right for its normal failure path by removing before asserting (`src/checks.rs:1656-1660`); the same ordering in `a_reserved_path_still_carries_its_owning_pid_as_the_first_component` closes the worst of it.

---

## Explicitly not raised

Per the review scope: line length and prose wrapping; the severity of the original defect; retaining `nanos()`; the choice of (a)+(d); the three integration-test sites building `{pid}-{nanos}` names under distinct literal prefixes; the absent CHANGELOG entry (checked and agreed, see confirmation 7).
