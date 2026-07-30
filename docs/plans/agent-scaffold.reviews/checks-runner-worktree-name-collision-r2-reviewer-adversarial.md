# Round 2 adversarial review: `checks-runner-worktree-name-collision` (commit `11d60f3`)

Reviewed at detached HEAD `11d60f3` in an isolated worktree. Environment: git 2.54.0, rustc 1.98.0-nightly (f46ec5218 2026-06-30), Linux, 16 cores.

Baseline on the unmutated tree: `cargo test` green (370 + 5 + 1 + 3 + 1 + 2), `cargo clippy --all-targets` clean. Every mutation below was applied, measured, and reverted; `src/checks.rs` is byte-identical to `HEAD` and `git status` is clean at the time of writing apart from this findings file.

**5 findings: 3 medium, 2 low. No high, no critical.**

Headline: I could NOT break the uniqueness property, and I could not make the prune reclaim a live worktree. I COULD make the prune permanently skip a registered dead orphan (Finding 3), and I could remove either of the fix's two layers at its use site and keep the whole suite, clippy, and the agreed `format!` grep green (Findings 1 and 2).

---

## Finding 1: the round-1 `TMPDIR` regression fix is pinned by no test, and removing it again is silent

**Severity: medium.**

Round 1 found a real user-facing regression (triage `T2`): reserving with `fs::create_dir` broke a `TMPDIR` naming a directory that does not exist yet, a case that worked before the reservation existed. `HEAD` fixes it by creating the leading directories before the loop, at `src/checks.rs:501-506`, and the comment at `:495-500` explains why the call is there. Nothing checks that it stays there.

**Mutation.** Delete the `fs::create_dir_all(&temp)` call at `src/checks.rs:501-506` (leaving `let mut last_taken = PathBuf::new();` at `:507` as the first statement after `let temp = ...`). Result: full `cargo test` GREEN, 370 + 5 + 1 + 3 + 1 + 2, identical to baseline. `cargo clippy --all-targets` is also clean (the mutation removes code, so no unused-item warning fires, unlike the `T4` bypass which at least produced three warnings).

**The behaviour that is unguarded, measured A/B with the same fixture repo.** With the mutation:

```
error: could not reserve the runner worktree directory <TMPDIR>/agent-scaffold-checks-run-1328422-1785425001894123979-0: No such file or directory (os error 2)
exit=2
```

Without it, same `TMPDIR` (a two-level path that does not exist):

```
        pass  ok (lint)
checks: 1 passed, 0 failed, 0 skipped
exit=0
```

Reproduce: build the binary, then `TMPDIR=<scratch>/does/not/exist agent-scaffold checks --dir <repo>`.

**Why this is medium and not low.** The regression it guards was rated `medium` in round 1 as a user-facing regression in shipped code, and it was introduced in this same step by a change nobody expected to be behaviour-visible. The identical mistake reintroduced by a later "simplify the reservation" refactor is invisible to every guard the project runs. The repository's own standard, quoted by the round-1 triage, is "Tests must actually exercise the code they claim to"; the code here claims (`:488-490`) that "a `TMPDIR` naming a directory that does not exist yet is legal, and it worked before the reservation existed", and no test executes that claim.

**Proportional fix.** An integration test in the existing style of `tests/checks_staged_hook_env.rs`, which already spawns the built binary with a custom environment (`Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))` plus `.env(...)` at `tests/checks_staged_hook_env.rs:34-41`): create a scratch repo, run `checks` with `TMPDIR` pointing at a nested path that does not exist, assert exit 0. That avoids the unsafe `std::env::set_var` problem a unit test would hit, and it is about 25 lines.

---

## Finding 2: `claim_dir`'s verdict is checked by no test at its only use site, so the decisive uniqueness layer can be dropped while the suite stays green

**Severity: medium.**

Round 1's `T1` fix extracted `claim_dir` (`src/checks.rs:443-449`) and pinned it with `a_directory_claim_is_exclusive` (`:1669-1686`). That test drives `claim_dir` directly and does kill the mutations the round-1 triage measured (`create_dir` to `create_dir_all`; a panic in the `AlreadyExists` arm). It does not reach the consumer. The line that turns a claim into an exclusive reservation is `:524-527`:

```rust
		if claimed {
			return Ok(path);
		}
		last_taken = path;
```

**Mutation.** Replace `if claimed {` at `:524` with `if claimed || true {`, so a LOST claim is handed back to the caller as if it had been won and the retry never runs. Result: full `cargo test` GREEN, 370 + 5 + 1 + 3 + 1 + 2. `a_directory_claim_is_exclusive` still passes (it never calls `reserve_runner_worktree`), and `concurrent_reservations_never_share_a_runner_worktree_path` (`:1688-1751`) still passes because in-process the atomic sequence alone already makes every candidate name distinct, so no claim is ever lost and the mutated branch is never taken.

**Why this is not a re-raise of `T4` or of the settled layer-1 gap.** `T4` (accepted residual) is about `run()` bypassing `reserve_runner_worktree` and building the name inline; its agreed proportional evidence is "exactly one `format!` in `src/checks.rs` builds a `RUNNER_PREFIX` name". That grep still returns exactly one hit under this mutation (`src/checks.rs:510`), `claim_dir` is still called, and `reserve_runner_worktree` is still the only construction site, so the agreed guard does not see this at all. The settled item is that the atomic SEQUENCE (layer 1) is unpinned; this is the other half, layer 2's application, and together they mean the property test fails only when BOTH layers are broken at once, never when either one is.

**What is actually lost under the mutation.** Cross-process uniqueness, which is the only thing layer 2 buys: the code returns to "two processes are separated by a clock reading", which is exactly the defect class this step exists to close, and it is the channel occupied by the two `dead_pid()` fixtures, whose first name segment is the constant `u32::MAX` in every process (`:1597-1601`, used at `:1622` and `:1653`). The step is classified RISKY because two callers on one path means `WorktreeGuard::drop` (`:347-359`) `remove_dir_all`s a directory another live run is inside.

**Proportional fix.** Make the claim injectable so the loser path becomes drivable, for example `fn reserve_with(pid: u32, claim: impl Fn(&Path) -> io::Result<bool>) -> io::Result<PathBuf>` with `reserve_runner_worktree(pid)` calling it with `claim_dir`. Two small tests then pin the whole loop: a claim closure that loses the first two calls asserts the returned path is NOT one of the paths whose claim was lost (which the current code cannot state at all), and a closure that always loses asserts the `AlreadyExists` exhaustion error. That also closes the untested exhaustion path at `:529-536` at no extra cost.

---

## Finding 3: the newly written Invariant B bound is incomplete, and the prune permanently skips a registered dead orphan whenever the recorded worktree path is not under the current `std::env::temp_dir()`

**Severity: medium.** The mechanism is PRE-EXISTING (the prune body is untouched by this step, verified below); the incorrect claim about it is NEW text written by this step.

`HEAD` rewrote Invariant B at `src/checks.rs:42-51` to state the bound on across-runs reclamation, and restated it at `:336-339` (`WorktreeGuard`) and `:874-877` (`run`). The new text says the bound is REGISTRATION: "the prune walks the worktrees git has registered to this repository, so it reclaims an orphan only once `git worktree add` has registered it", with the kill between reservation and add named as the one gap. There is a second gate it does not mention. `prune_orphan_worktrees` reads `let temp = std::env::temp_dir();` at `:573` and skips any registered worktree failing `path.starts_with(&temp)` at `:582`, where `path` is what git recorded. Git records the symlink-RESOLVED path, so when `TMPDIR` resolves through a symlink the gate never matches and a registered, dead-owner orphan is skipped forever.

**Reproduction, four runs of the shipped binary.**

```
mkdir -p $SP/real && ln -s $SP/real $SP/link
ORPH="$SP/link/agent-scaffold-checks-run-4294967295-1785000000000000000-0"
mkdir "$ORPH"
git -C $SP/fix worktree add --detach "$ORPH" HEAD
git -C $SP/fix worktree list --porcelain | grep '^worktree '
# -> worktree <...>/real/agent-scaffold-checks-run-4294967295-1785000000000000000-0   (resolved, not "link")
for i in 1 2 3 4; do TMPDIR=$SP/link agent-scaffold checks --dir $SP/fix; done
ls $SP/real
# -> agent-scaffold-checks-run-4294967295-1785000000000000000-0    (still there after all four runs)
```

The owner is `u32::MAX`, so the liveness gate would reclaim it; the run's own worktree is created and cleaned correctly in the same runs, so nothing else is wrong. Only the prune is defeated.

**Second, sharper symptom: the step's own guard tests fail in that environment.** Running the unit test binary with `TMPDIR` pointing at a symlink:

```
TMPDIR=<symlink> ./target/debug/deps/agent_scaffold-<hash> checks:: --test-threads 4
test checks::tests::a_startup_prune_reclaims_an_orphaned_runner_worktree ... FAILED
test checks::tests::a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one ... FAILED
test result: FAILED. 28 passed; 2 failed
```

with the panics at `src/checks.rs:1631` ("the registered orphan worktree was reclaimed") and `src/checks.rs:1662` ("a dead owner's worktree is reclaimed"), and five leaked directories left in the resolved temp dir. So `cargo test` is environment-dependent in exactly the way this step was opened to stop: a green suite is a statement about the developer's `TMPDIR`, not about the code.

**Pre-existing, verified two ways.** `git diff HEAD~2 HEAD -- src/checks.rs | grep -n "starts_with\|remove_dir_all(path)\|fn prune_orphan_worktrees"` returns nothing, so the prune body is untouched. Building `HEAD~2`'s `src/checks.rs` and running the same two tests under a symlinked `TMPDIR` reproduces both failures identically (panics at the pre-step lines `:1471` and `:1501`).

**What I am asking for.** At minimum, correct the newly written bound so it stops promising more than the code delivers: registration is necessary but not sufficient, and reclamation additionally requires the recorded worktree path to sit under the CURRENT process's `std::env::temp_dir()`, which also fails silently when `TMPDIR` differs between the killed run and the reclaiming one. That is prose only and squarely inside this step's remit, which already committed to correcting every comment that over-promises here. The code fix (compare against a canonicalised temp dir, or match on the file name alone) changes what the prune is willing to delete and belongs in its own step, not this one.

---

## Finding 4: the retry comment's stated cause is contradicted by the leak the same commit documents

**Severity: low.**

`src/checks.rs:427-428` states: "Each attempt draws a fresh sequence value, so a second attempt only ever happens when a DIFFERENT process already holds the name", and `:511-512` repeats it: "A lost one means someone else holds this exact name". "Holds" is wrong for the case this same commit added to Invariant B at `:45-48`: a kill between `create_dir` and `git worktree add` leaves an EMPTY, unregistered directory that no run ever reclaims. That directory is a name held by nothing at all, and a later candidate colliding with it takes a second attempt for a reason both comments exclude. Three pieces of newly written text in one file disagree.

Evidence is the three citations read side by side; no test is owed. The deduction the comment draws (16 consecutive collisions means a broken assumption, not a race worth retrying) is unaffected, because each attempt still draws a fresh sequence value. Fix is a few words in each place, for example "when the name is already taken, by another process or by a directory a killed run left behind".

---

## Finding 5: the new unconditional `create_dir_all(&temp)` creates a directory in the process working directory when `TMPDIR` is relative, and the reservation then guards a path git does not use

**Severity: low.** Requires a malformed `TMPDIR` (POSIX requires an absolute path), which is why this is low and not medium.

`src/checks.rs:501` calls `fs::create_dir_all(&temp)` with whatever `std::env::temp_dir()` returns, unvalidated. Rust returns a relative `TMPDIR` verbatim, and `create_dir` / `create_dir_all` then resolve it against the PROCESS working directory while `git -C <repo> worktree add <relative path>` resolves it against the REPOSITORY. Measured from an empty scratch directory:

```
cd $SP/cwd
TMPDIR=reltmp agent-scaffold checks --dir $SP/fix
# -> checks: 1 passed, exit 0
ls -a $SP/cwd     # -> reltmp        (NEW: created by create_dir_all, left behind)
ls -a $SP/fix     # -> reltmp        (the worktree was created INSIDE the repository)
```

Two consequences. The empty `reltmp` in the caller's working directory is new to this step; the repository-side one is pre-existing (git created it before too). More interesting for this step's own argument: the directory the reservation claimed and the directory git actually used are different directories, so `claim_dir`'s exclusivity covers a path that is not the worktree, and the doc claim at `:934-938` that "no concurrent call can be handed the same path" is true of the return value but no longer says anything about the worktree two concurrent runs occupy.

The cheap fix is to reject a relative `std::env::temp_dir()` with a clear error rather than creating it, which also makes the pre-existing "worktree inside the repository" outcome impossible. Deferring it with a comment is also defensible at this severity.

---

## Attacked and could not break

Recorded so the triager knows these were driven, not assumed.

- **Two callers on one path, in process.** `concurrent_reservations_never_share_a_runner_worktree_path` is genuinely RED against the pre-fix construction: replacing the body of `reserve_runner_worktree` with `Ok(std::env::temp_dir().join(format!("{RUNNER_PREFIX}{pid}-{}", nanos())))` gives `28 passed; 2 failed`, the property test failing on duplicate paths. The demonstration the brief demanded holds.
- **Two callers on one path, cross-process.** 60 concurrent runs of the unit test binary (5 rounds x 12 processes, `checks::` with `--test-threads 8`), all sharing one temp dir and therefore contending on the constant `u32::MAX` fixture template: 0 non-zero exits, 60 x "30 passed; 0 failed", and 0 leftover `agent-scaffold-checks-run-*` directories afterwards. Layer 2 holds under real contention.
- **Prune reclaiming a LIVE worktree.** I could not construct one. The pid stays the first component (pinned by `a_reserved_path_still_carries_its_owning_pid_as_the_first_component`, `:1753-1774`), `owning_pid` (`:545-547`) reads only that segment, and `dead_pid()` asserts its own premise (`:1594-1601`). The only route I found needs a PID namespace boundary crossed while the repository and temp dir are shared, which is unchanged by this step and not reproducible here.
- **`WorktreeGuard` removing what it does not own, or double-removing.** It owns the reserved path from `:943-946`, before the add, and drops once. Moving the guard before the add strictly REDUCES a pre-existing race: `git worktree prune` (run from `Drop` at `:358` and from `:602`) prunes an admin entry whose worktree directory is missing, and the reservation means the directory now exists before git registers it, so the window where a registered worktree has no directory is gone.
- **The 16-attempt bound and its exhaustion error.** Forced by making every claim lose: the message renders correctly, `error: could not reserve a unique runner worktree directory after 16 attempts (last tried <path>)`, exit 2 through `RunError::Io` (`:285-294`). The `\`-continuation in the format string at `:532-533` does not eat the space. The path itself leaks nothing on this route in unmutated code, because a lost claim means the directory is someone else's. The exhaustion branch is executed by no test, which is folded into Finding 2's fix rather than raised separately.
- **The error-context rewrite dropping `raw_os_error()`.** Nothing depends on it: `grep -rn "raw_os_error" src/ tests/` returns zero hits, and the only kind-sensitive consumer, `git()` at `:393-404`, is on a different path. `kind()` is preserved, and every wrapped message names the operation and the full path (measured for a `TMPDIR` that is a regular file, a dangling symlink, and an unwritable directory: all three give a path-carrying message and exit 2).
- **The three test fixtures.** All three now take their path from `reserve_runner_worktree` (`:1622`, `:1652`, `:1653`), exactly two keep the dead pid (`:1622` and `:1653` pass `dead_pid()`), and both prune tests still assert what they claim: they fail correctly when the prune is defeated (see Finding 3's symlink run, and the MUT-1 run above), so they are not vacuous.
- **One construction site.** `grep -n 'format!("{RUNNER_PREFIX}' src/checks.rs` returns exactly one hit, `:510`. `grep -rn "agent-scaffold-checks-run"` outside `.git/` reaches only `src/checks.rs` and this plan's own documents; `README.md`, `CHANGELOG.md`, `AGENTS.md` and `pack/` carry none, so nothing outside `src/` went stale.
- **Empty `TMPDIR`.** Falls back to `/tmp` on this toolchain; no relative-path or empty-prefix behaviour, so the `starts_with` gate is not degraded that way.

## Cleanup note for the orchestrator

The mutation that forced the exhaustion path (every claim lost, with the directory still created) left 32272 EMPTY `agent-scaffold-checks-run-*` directories in `/tmp`, all from that one run: 32240 under pid `1375127` and 32 under `4294967295`. They are unregistered, so no prune and no test can see them, and they are harmless, but they should be removed. My delete was refused by the permission classifier, so it needs a human:

```
find /tmp -maxdepth 1 -name 'agent-scaffold-checks-run-1375127-*' -type d -empty -delete
find /tmp -maxdepth 1 -name 'agent-scaffold-checks-run-4294967295-*' -type d -empty -delete
```

Separately and NOT mine: 65 `agent-scaffold-checks-test-*` directories in `/tmp` predate this session, the oldest by about twelve days. That is `scratch()` litter from failed runs in earlier sessions, out of this step's scope.
