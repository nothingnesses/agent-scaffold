# Round 2 verification review: `checks-runner-worktree-name-collision` (commit `11d60f3`, diff `HEAD~1..HEAD`)

Lens: did the four round-1 fixes land, and did they re-seed anything? Every claim below was measured in an isolated worktree at `11d60f3` (detached HEAD). All mutations were reverted; `git status` is clean and `git diff HEAD` is empty at the time of writing.

Baseline before any mutation: `cargo test` green, 370 + 5 + 1 + 3 + 1 + 2 = 382 passed, 0 failed. `cargo clippy --all-targets` emits nothing. `cargo run -- validate` reports `docs/metrics/workflow.jsonl: 228 records, valid`.

## Verdict per fix

| Fix | Closes | Verdict |
| --- | --- | --- |
| 1 | T1 (`claim_dir` extracted, exclusivity test added) | LANDED (layer 2 only; see F1) |
| 2 | T2 (missing `TMPDIR`, path-carrying errors) | LANDED |
| 3 | T3 (three false comments, plus a fourth site) | LANDED, and the fourth site was correct and in scope |
| 4 | T6 (fixture leak on a failing assertion) | LANDED |

## Fix 1: `claim_dir` and `a_directory_claim_is_exclusive` (T1)

`claim_dir` is at `src/checks.rs:443-449`; the load-bearing `fs::create_dir` is at `:444` and the `AlreadyExists` arm at `:446`. The test is `a_directory_claim_is_exclusive`, `src/checks.rs:1669-1686`.

Mutation (a), `:444` `fs::create_dir` -> `fs::create_dir_all`. My output:

```
thread 'checks::tests::a_directory_claim_is_exclusive' panicked at src/checks.rs:1684:9:
a second claim on the same path is lost
test result: FAILED. 29 passed; 1 failed; 0 ignored; 0 measured; 340 filtered out
```

Mutation (b), `panic!` planted in the `:446` `AlreadyExists` arm. My output:

```
thread 'checks::tests::a_directory_claim_is_exclusive' panicked at src/checks.rs:446:71:
MUTATION B: taken branch reached
test result: FAILED. 29 passed; 1 failed; 0 ignored; 0 measured; 340 filtered out
```

Both RED, both killed by the new test and by nothing else (the other 29 `checks::` tests pass under each). Both reverted. Fix 1 lands as claimed.

The single-site claims still hold, re-run at `11d60f3`:

- `grep -n "fs::create_dir(" src/checks.rs` -> exactly one hit, `:444`, inside `claim_dir`.
- `grep -n "RUNNER_PREFIX" src/checks.rs` -> exactly one `format!` building a name, `:510`.
- `grep -rln "agent-scaffold-checks-run" --exclude-dir=.git .` -> `src/checks.rs` plus this plan's own documents only.

## Fix 2: missing `TMPDIR` and error context (T2)

Verified by A/B between a binary built from `HEAD~1:src/checks.rs` and one built from `HEAD`, both against a throwaway git repo carrying a single trivial `lint` check.

| `TMPDIR` condition | pre-fix (`HEAD~1`) | post-fix (`11d60f3`) |
| --- | --- | --- |
| nested path that does not exist | `error: No such file or directory (os error 2)`, exit 2 | `checks: 1 passed, 0 failed, 0 skipped`, exit 0, directories created, nothing left behind |
| exists, `chmod 555` | `error: Permission denied (os error 13)`, exit 2 | `error: could not reserve the runner worktree directory <full path>/agent-scaffold-checks-run-1324781-1785424873417654030-0: Permission denied (os error 13)`, exit 2 |
| unset | not measured | exit 0 |

Both halves of T2 are closed: the missing-`TMPDIR` regression is gone and a genuine failure now names the operation and the path. The `create_dir_all` at `:501-506` runs once, outside the loop, as prescribed. See F2 for the one case where the new message is less accurate than the old one.

## Fix 3: Invariant B and the restatements (T3)

The four sites, read at their current lines:

- `src/checks.rs:42-51`, Invariant B: reclamation is "BOUNDED BY REGISTRATION", and a kill between `reserve_runner_worktree` and the add leaves an empty unregistered directory "which no later run reclaims".
- `src/checks.rs:336-339`, `WorktreeGuard`: "That prune reclaims a REGISTERED worktree only".
- `src/checks.rs:874-877`, `run` doc: "the next run's startup prune reclaims what the kill left REGISTERED, but not the empty directory ... left unregistered".
- `src/checks.rs:925-928`, the `prune_orphan_worktrees` call site inside `run()` (the fourth site the implementer flagged itself): "a SIGKILL that leaked a REGISTERED worktree self-heals on the next run".

All four are now TRUE. Measured independently rather than read: with one registered dead-owner worktree (`git worktree add --detach`) and one unregistered dead-owner directory (`mkdir`) planted under a shared `TMPDIR`, then two consecutive `agent-scaffold checks` runs:

```
before:      agent-scaffold-checks-run-4294967295-1785000000000000000-0   (unregistered)
             agent-scaffold-checks-run-4294967295-1785000000000000001-1   (registered)
after run 1: agent-scaffold-checks-run-4294967295-1785000000000000000-0
after run 2: agent-scaffold-checks-run-4294967295-1785000000000000000-0
```

The registered orphan is reclaimed on the first run; the unregistered one survives both. That is exactly what the four statements now say.

**The fourth site was correct and in scope.** The pre-fix text at `:925-928` read "so a SIGKILL leak self-heals on the next run (Invariant B's caveat)". That is over-general in precisely the way T3 names: it is a bare "SIGKILL leak" claim, and the reservation-window leak does not self-heal. Leaving it while qualifying the other three would have left the same defect in the same commit, so qualifying it is the T3 fix applied consistently, not scope creep. It is doc-only, one clause, and it does not widen the prune (which the triager forbade and which the code does not do).

## Fix 4: fixture leak (T6)

`a_reserved_path_still_carries_its_owning_pid_as_the_first_component` is at `src/checks.rs:1753-1774`; the two `fs::remove_dir` calls are now at `:1764-1765`, above the assertion loop at `:1766-1773`.

Measured with the prepend mutation (`{RUNNER_PREFIX}{seq}-{pid}-{nanos}` at `:510`), running that one test with `--exact` under a dedicated empty `TMPDIR`, once against `HEAD~1:src/checks.rs` carrying the same mutation and once against `HEAD`:

```
PRE-FIX  ordering: FAILED, and the isolated TMPDIR afterwards contains
                   agent-scaffold-checks-run-0-4294967295-1785425009096294106
                   agent-scaffold-checks-run-1-1328899-1785425009096494071
POST-FIX ordering: FAILED (same assertion, same message), 0 entries left in the isolated TMPDIR
```

Both runs fail with `assertion left == right failed: the owning pid must still parse out of agent-scaffold-checks-run-0-4294967295-...`, so the diagnostic is unchanged and only the litter is gone. Fix 4 lands.

---

# Findings

Three findings, all `low`. **No `medium`, `high`, or `critical` findings.** Nothing here blocks the round on my reading; F1 is the one the orchestrator has a genuine choice about.

## F1 (`low`): layer 1 is unpinned, the retry loop is unpinned too, and the recorded reason for the first is factually wrong

**Severity `low`**: the code as committed is correct, and correctness is carried by layer 2, which is now pinned. The exposure is regression-only, the same class the triager rated `low` for T5. It is recorded above `nil` because the retry half is a strictly larger hole than the layer-1 half round 1 accepted, and because the reason currently on the record is not what the machine does.

### The implementer's self-report reproduces, but its explanation does not

Mutation `:509` `NEXT_RUNNER_SEQ.fetch_add(1, Ordering::Relaxed)` -> `NEXT_RUNNER_SEQ.load(Ordering::Relaxed)`, full `checks::` module, five consecutive runs:

```
test result: ok. 30 passed; 0 failed  (x5)
```

So the self-report is accurate: layer 1 is not pinned. But the stated mechanism ("`nanos()` still varies and the 16 retries absorb the collisions, so it never exhausts") is not what happens, and neither is triage T1's version of it ("with `seq` constant the raw names duplicate at the clock's rate, the reservation's retry path carries the property").

I instrumented `reserve_runner_worktree` with two counters (total loop iterations, deepest retry depth), printed from the property test, and reverted them afterwards:

| Code state | total attempts | reservations | deepest retry depth |
| --- | --- | --- | --- |
| unmutated, full module, 3 runs | 2021 / 2020 / 2020 | 2000 | 1 / 1 / 1 |
| `seq` pinned (`load`), full module, 3 runs | 2020 / 2020 / 2021 | 2000 | 1 / 1 / 1 |
| `seq` pinned, property test alone, 3 runs | 2000 / 2000 / 2000 | 2000 | 1 / 1 / 1 |

Depth 1 means no reservation ever needed a second attempt. **With `seq` pinned there were zero collisions to absorb**, because the `mkdir` syscall spaces successive `nanos()` readings by microseconds, so the raw names do not in fact duplicate at the clock's rate under this call pattern. The step brief's 2793-to-3354-in-8000 duplicate figure came from a tight-loop name probe with no syscall between readings, which is a different regime. Both measurements are right; only the inference that connects them to this test is wrong.

This matters for the record: the retry path is not what keeps the property test green under the layer-1 mutation, so "layer 2 covers for layer 1" is not an argument the suite supports.

### The retry loop itself is dead under the whole suite

Mutation R: replace `last_taken = path;` at `src/checks.rs:527` with an immediate `return Err(...)`, so a lost claim becomes a hard failure instead of a retry. Full `cargo test`:

```
test result: ok. 370 passed; 0 failed
test result: ok. 5 passed / 1 passed / 3 passed / 1 passed / 2 passed
```

382 passed, 0 failed, with the retry deleted. Reverted. This was not measured in round 1 and it is a bigger hole than the accepted layer-1 residual: deleting the retry would turn a recoverable name collision into a user-visible `RunError::Io` failure of the shipped `checks` command, and no test in the repository would notice. `a_directory_claim_is_exclusive` pins `claim_dir`'s `Ok(false)` return but not the caller's handling of it, and the exhaustion `Err` at `:529-536` is likewise never reached by any test.

Note that `RUNNER_RESERVE_ATTEMPTS`'s own doc at `:426-431` ("a second attempt only ever happens when a DIFFERENT process already holds the name") is confirmed by the depth-1 measurement, so the code's own comments are accurate here. The gap is in the tests, not the prose.

### A proportionate pin exists, and I measured it

The implementer says pinning layer 1 needs either name-level assertions (which the property test's own comment at `:1702-1706` rules out) or an injectable clock, and treats the latter as disproportionate. I think it considered only the global-state form of an injectable clock. A clock **parameter** is smaller, has no process-global state, and needs no `cfg(test)` branch in the hot path:

```rust
fn reserve_runner_worktree(pid: u32) -> io::Result<PathBuf> {
	reserve_runner_worktree_with(pid, nanos)
}

fn reserve_runner_worktree_with(pid: u32, clock: fn() -> u128) -> io::Result<PathBuf> { ... }
```

with `clock()` replacing `nanos()` at `:510`, and one test that takes four reservations with `|| 0` as the clock and asserts they all succeed and are distinct. Measured here:

- Against the committed code: `test result: ok. 31 passed; 0 failed`.
- Under `fetch_add` -> `load`: RED, `reservations failed with a frozen clock: ["could not reserve a unique runner worktree directory after 16 attempts (last tried /tmp/agent-scaffold-checks-run-1370607-0-0)", ...]`, 30 passed, 1 failed.

Both reverted; the experiment is not in the tree. Cost is four lines of production code and about a dozen of test. It pins layer 1 deterministically, with no concurrency and no sampling, and as a side effect it is the only thing in the repository that would ever execute the retry loop and the exhaustion error, which closes the mutation-R hole above at the same time.

### My judgement

I would take the pin, but I would not block the round on it.

The argument for taking it: the step's own "WHAT DONE LOOKS LIKE" says the uniqueness property is pinned by a test that fails without the fix. The shipped fix is (a) plus (d), and the test fails without (d) only. Half of what was shipped, and specifically the half that closes the observed in-process defect cheaply, is unpinned; and the retry loop that makes (d) safe is unpinned outright. Leaving it means a future edit can quietly return the module to relying on the clock, which is the exact reliance this step exists to remove. Plan Principle 5 (make illegal states unrepresentable) is what the (a)+(d) composition was argued on, and an unexecuted retry loop is that composition half-observed.

The argument against, which is why this is `low` and not `medium`: nothing shipped is wrong today, the invariant that matters (no two callers share a path) is carried by layer 2 and is now pinned directly, and Q-66 proportionality plus plan Principle 2 (minimal by default) both cut toward stopping. An accepted residual here is legitimate.

What I would not accept is carrying the current *reason* forward. If the orchestrator accepts the residual, the recorded justification should be "layer 1 is an optimisation and the correctness guarantee is pinned at layer 2", not "the retries absorb the collisions". The latter is measurably false, and a future reader who trusts it will believe the retry path is exercised when nothing exercises it.

## F2 (`low`): the new `create_dir_all` reports a less accurate errno when `TMPDIR` is not a directory

`src/checks.rs:501-506`. Measured, pre-fix binary versus post-fix binary, same fixture repo:

| `TMPDIR` condition | pre-fix | post-fix |
| --- | --- | --- |
| existing regular FILE | `error: Not a directory (os error 20)`, exit 2 | `error: could not create the temp directory <path>: File exists (os error 17)`, exit 2 |
| dangling symlink | `error: No such file or directory (os error 2)`, exit 2 | `error: could not create the temp directory <path>: File exists (os error 17)`, exit 2 |

This is inherent to `fs::create_dir_all`, which retries `mkdir` and falls back on `path.is_dir()`; when that is false it returns the `EEXIST` from the `mkdir`, not the more descriptive `ENOTDIR`. Both cases still fail correctly (exit 2, no worktree created), and the post-fix message names the operation and the path, which is what T2 asked for, so this is a net improvement rather than a regression. **No fix required.** Recorded so that a future report of "File exists" against a `TMPDIR` that is a plain file is diagnosed rather than treated as a new defect.

Related non-finding, checked because the brief asked: the error-context rewrite uses `io::Error::new(kind, msg)`, which preserves `kind()` but drops `raw_os_error()`. `grep -rn "raw_os_error\|\.source()\|downcast" src/ tests/` returns nothing, so no code in this tree observes it, and the OS error number survives textually inside the wrapped message (`(os error 13)` above). Not a defect.

## F3 (`low`): two comments written by this commit state something false

Same class as T3, much smaller. Both are prose introduced by `11d60f3`.

1. `src/checks.rs:495-496`: "`claim_dir` deliberately creates exactly ONE level (creating parents is what would destroy its exclusivity)". Creating parents is not what would destroy the exclusivity; tolerating an already-existing LEAF is. `claim_dir`'s own doc gets this right 60 lines earlier at `:437-439` ("`create_dir_all` succeeds when the directory already exists, so every claim would report `Ok(true)`"), and my mutation (a) above confirms the leaf is the load-bearing part. Fix, if a pass is open anyway: "creating parents" -> "tolerating a directory that already exists".

2. `src/checks.rs:1673`: "Both outcomes matter, and neither was executed by any other test." The LOST outcome was executed by no other test, and my depth-1 measurement above confirms that (`claim_dir` never returned `Ok(false)` in any test run I instrumented). The WON outcome is executed roughly 2020 times per `checks::` run by every other reservation; it was merely never asserted. Fix, if a pass is open anyway: "neither was executed" -> "the lost outcome was executed by no other test, and the won one was never asserted".

**Neither requires a fix on its own.** I am recording them because the T3 finding was precisely "comments that say something the code does not do", and a fix pass that re-seeds two smaller instances of that is worth knowing about before the next round rather than after it. If the orchestrator judges this prose churn, the T8 precedent supports dismissing both.

---

# Checked and NOT findings

- **The new test leaks its scratch directory when it fails.** `a_directory_claim_is_exclusive` (`:1669-1686`) calls `fs::remove_dir_all(&dir)` after its assertions, so mutation (a) left `agent-scaffold-checks-test-<pid>-claim-dir` behind. This is not a T6-class leak: `scratch()` at `:1021-1030` names the directory by pid plus a per-test literal and calls `fs::remove_dir_all` on entry, so the next run of that same test reclaims it. It is the module's standard pattern and is bounded at one directory. No action.
- **`create_dir_all(&temp)` called unconditionally, outside the loop.** Costs one extra `mkdir` per run against an existing `TMPDIR`; with `TMPDIR` unset the run is green (measured). Pre-fix, `git worktree add` created the same leading directories, so the authority the process exercises is unchanged: it creates only the temp directory the user pointed it at, never anything wider. No Principle 18 concern.
- **Documentation currency outside `src/`.** `grep -rniE "tmpdir|temp dir|temporary worktree" README.md CHANGELOG.md pack/ .agents/ AGENTS.md` returns nothing, and `agent-scaffold-checks-run` appears outside `src/checks.rs` only in this plan's own documents. Nothing scaffolded or shipped goes stale. Consistent with the settled no-CHANGELOG-entry call, which I did not reopen.
- **Guards.** `cargo test` 382 passed / 0 failed; `cargo clippy --all-targets` silent; `cargo run -- validate` valid. No new warnings from the `claim_dir` extraction.
- **32272 empty `agent-scaffold-checks-run-*` directories appeared in `/tmp` mid-review; they are not from the committed code.** They carry one live pid (`1375127`), a contiguous seq range `0..32271`, and mtimes inside a single 0.46 s window at 16:33, which is 8 threads times 4000 takes plus the other tests' reservations, i.e. a scaled-up copy of the property test rather than the committed `THREADS = 8, PER_THREAD = 250`. `git worktree list` shows a second round-2 reviewer worktree (`rev93r2-b`) running against the same `/tmp`, and none of the directories is registered to this repository. The committed code does not leak: a full `cargo test` under a dedicated empty `TMPDIR` leaves 0 `agent-scaffold-checks-run-*` and 0 `agent-scaffold-checks-test-*` entries. I left them in place rather than deleting another session's working state.

# Settled findings not re-raised

T4, T5, T7, T10 (accepted residuals), T8 and T9 (dismissed), the (a)+(d) design choice, retaining `nanos()`, the uniqueness property itself, the red-before-green demonstration, and the no-CHANGELOG call. I found no new evidence against any of them. I did not propose widening the prune to sweep the temp directory by prefix.

# Worktree state

All mutations reverted. `git status --short` empty, `git diff HEAD` empty, HEAD still `11d60f3`. `ls /tmp | grep -c agent-scaffold-checks-run` returns 0. All experiment fixtures were built under the session scratchpad, not in the repository.
