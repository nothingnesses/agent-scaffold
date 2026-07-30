# Adversarial mutation hunt: `checks-runner-worktree-name-collision`, commit `3f49012`, round 4

Reviewed at HEAD `3f490128ef34c608dac134a313bdb69972e0daf0` ("test(checks): pin the reservation's claim-error arm and its exhaustion bound") in an isolated worktree at `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/impl-checks-collision/.claude/worktrees/rev93-r4-b`, branch `review/checks-collision-r4b`.

Baseline reconfirmed here, not quoted from any prior document: `cargo test` 373 + 5 + 1 + 1 + 3 + 1 + 2 = **386 passed, 0 failed**; `cargo clippy --all-targets` produces no warnings (`Checking` / `Finished`, nothing else). Both match the numbers stated in the task brief.

Every mutation below was applied to `src/checks.rs` alone, measured, and reverted with `git checkout -- src/checks.rs` before the next one. Final state verified at the end of this file: `git status --short` empty, `git diff HEAD` empty, and `src/checks.rs` byte-identical (`sha256sum`) to a copy saved before any mutation.

**One new finding: MU1, `medium`.** It answers the task's own attack #2 directly: the new claim-error test injects its own closure and never calls the real `claim_dir`, so `claim_dir`'s own third match arm (documented as "propagates every other error") has no test at all. It can be replaced with `Ok(false)` (report a real error as a lost claim) and the full suite plus clippy stay green, reproducing the identical user-visible misdiagnosis (a permissions fault reported as 16-attempt name-collision exhaustion) that `AD1a` fixed one level up the call stack. I built and measured a one-test fix; it kills the mutation and stays green on the unmutated tree.

Everything else attacked in this round held: the re-run "delete a load-bearing part" sweep now shows 4 of 5 core parts RED (up from round 2's 4-of-5 GREEN and consistent with round 3's report), the two settled residuals still hold for their documented reasons, the forced worst-case concurrent-process attack found 0 shared paths, and the three `checks_missing_tmpdir.rs` environment attacks that "failed" (in the sense of not finding a new defect) are recorded with their mechanism, not asserted.

---

## MUTATION TABLE

All full-suite runs; "unit" figure is the `--bin agent-scaffold` test count out of the shared 386. RED entries list the killing test(s) verbatim from `cargo test` output.

| # | Target | Mutation | Result | Killed by / notes |
| - | ------ | -------- | ------ | ------------------ |
| 1 | `reserve_runner_worktree_with`, claim's error arm | `claim(&path).map_err(...)?` -> `claim(&path).unwrap_or(false)` (M1a) | **RED** | `checks::tests::a_claim_error_that_is_not_a_collision_propagates_at_once_and_names_the_path` (372 passed; 1 failed) |
| 2 | same | `-> claim(&path)?` (M1b, drops message wrapping) | **RED** | same test (372 passed; 1 failed) |
| 3 | same | `-> claim(&path).unwrap_or(true)` (M1c, errored claim counts as WON) | **RED** | same test (372 passed; 1 failed) |
| 4 | `claim_dir`, third match arm | `Err(error) => Err(error)` -> `Err(_error) => Ok(false)` (**MU1**) | **GREEN** | 386 passed, 0 failed; clippy silent. Survivor. See MU1 below. |
| 5 | `reserve_runner_worktree_with`, temp-dir creation | delete `fs::create_dir_all(&temp).map_err(...)?;` entirely | **RED** | `checks_runs_under_a_tmpdir_that_does_not_exist_yet` (integration binary: 0 passed; 1 failed) |
| 6 | `NEXT_RUNNER_SEQ` | `fetch_add(1, Relaxed)` -> `load(Relaxed)` | **GREEN** | 386 passed, 0 failed. Settled `X1` residual, not a new finding. |
| 7 | lost-claim verdict | `if claimed {` -> `if claimed \|\| true {` | **RED** | `a_claim_that_never_wins_fails_at_the_attempt_bound` and `a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one` (371 passed; 2 failed) |
| 8 | `RUNNER_RESERVE_ATTEMPTS` | `16` -> `1` | **RED** | `a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one` (372 passed; 1 failed) |
| 9 | `RUNNER_RESERVE_ATTEMPTS` | `16` -> `3` | **GREEN** | 386 passed, 0 failed. Explicitly settled; not a new finding. |
| 10 | `claim_dir`, second arm | `fs::create_dir(path)` -> `fs::create_dir_all(path)` | **RED** | `a_directory_claim_is_exclusive` (372 passed; 1 failed) |
| 11 | exhaustion message | drop `after {RUNNER_RESERVE_ATTEMPTS} attempts` from the format string (M4) | **RED** | `a_claim_that_never_wins_fails_at_the_attempt_bound` (372 passed; 1 failed). Confirms Fix B (AD2) closed. |
| 12 | retry loop | hoist `let seq = NEXT_RUNNER_SEQ.fetch_add(...)` out of the `for` loop | **GREEN** | 386 passed, 0 failed. Near-equivalent mutant (round 3's `FA2`); not a new finding. |
| 13 | retry loop bound | `for _ in 0 .. RUNNER_RESERVE_ATTEMPTS` -> `0 ..= RUNNER_RESERVE_ATTEMPTS` (off-by-one, one extra attempt) | **RED** | `a_claim_that_never_wins_fails_at_the_attempt_bound` (372 passed; 1 failed) |
| 14 | `owning_pid` | `.split('-').next()` -> `.split('-').last()` | **RED** | `a_reserved_path_still_carries_its_owning_pid_as_the_first_component`, `a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one` (371 passed; 2 failed) |
| 15 | `pid_is_alive` | invert the leading boolean: `Path::new(...).exists()` -> `!Path::new(...).exists()` | **RED** | `dead_pid()`'s own `assert!` panics inside 3 tests (370 passed; 3 failed) |
| 16 | `run()` call site | comment out `prune_orphan_worktrees(&repo);` | **RED**, and clippy also complains | `a_startup_prune_reclaims_an_orphaned_runner_worktree` (372 passed; 1 failed); clippy: `function 'owning_pid' is never used`, `function 'prune_orphan_worktrees' is never used` (3 warnings) |
| 17 (fix) | `claim_dir`, new test | add `claim_dir_propagates_a_non_collision_error_rather_than_reporting_it_taken` (see MU1) | **GREEN** alone (387 passed, 0 failed, clippy silent); **RED** under mutation #4 (373 passed; 1 failed, `claim_dir_propagates_a_non_collision_error_rather_than_reporting_it_taken`) | Built and measured fix for MU1 |

Rows 5, 6, 7, 8, 9, 10 are the "5 load-bearing parts" re-swept (row 9 is the explicitly settled retuning check, included for completeness, not one of the original 5). **Current answer: 4 of 5 are RED (rows 5, 7, 8, 10), 1 stays GREEN (row 6, the accepted `X1` residual).** This is one further than round 3's own re-measurement reported in `checks-runner-worktree-name-collision-reviewer-r3-adversarial.md`'s `FA11` table (which had the retry-bound row as GREEN at `16 -> 3` and RED at `16 -> 1`, the same split I confirm at rows 8/9): nothing regressed, and the claim's-error-arm rows (1, 2, 3) that were GREEN as of round 3's `AD1` finding are now RED, confirming the round-3 fix (`Fix A`) landed as claimed.

---

## MU1: `claim_dir`'s own error-propagation arm is executed by nothing, and the new claim-error test cannot see it

**Severity: `medium`.**

### Claim

`claim_dir`'s doc comment (`src/checks.rs:438-447`) states its contract in three parts: `Ok(true)` when this call created the directory, `Ok(false)` when it already existed, and "propagates every other error." The third clause has zero test coverage through the real function. The new test this round's commit added, `a_claim_error_that_is_not_a_collision_propagates_at_once_and_names_the_path`, answers the task's attack #2 directly: it injects its own closure into `reserve_runner_worktree_with` (`|path| { ...; Err(io::Error::from(io::ErrorKind::PermissionDenied)) }`) and never calls `claim_dir` at all. It correctly pins the CALLER's handling of whatever the claim function returns (mutations 1, 2, 3 in the table above are all now RED), but it establishes nothing about the CALLEE, `claim_dir` itself. `grep -n "claim_dir(" src/checks.rs` shows exactly one other call site, inside `a_directory_claim_is_exclusive`, and that test only drives the first two arms (fresh path: `Ok(true)`; already-existing path: `Ok(false)` via `AlreadyExists`). No test anywhere calls the real `claim_dir` in a way that forces a non-`AlreadyExists` error out of `fs::create_dir`.

### Reproduced evidence

Mutation (row 4 in the table): `src/checks.rs:451-452`

```
Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(false),
Err(error) => Err(error),
```
->
```
Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(false),
Err(_error) => Ok(false),
```

Full suite:

```
test result: ok. 373 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s
test result: ok. 5 passed; 0 failed; ...
test result: ok. 1 passed; 0 failed; ...
test result: ok. 1 passed; 0 failed; ...
test result: ok. 3 passed; 0 failed; ...
test result: ok. 1 passed; 0 failed; ...
test result: ok. 2 passed; 0 failed; ...
```

386 passed, 0 failed. `cargo clippy --all-targets`: `Checking agent-scaffold ... Finished` dev profile, no warnings.

**User-visible consequence, measured with the built binary, A/B against a scratch repo with one trivial `lint` check, `TMPDIR` naming an existing but `chmod 555` (unwritable) directory:**

```
committed 3f49012 (unmutated):
error: could not reserve the runner worktree directory <TMPDIR>/mu1-probe/unwritable/agent-scaffold-checks-run-1831295-1785451679748461012-0: Permission denied (os error 13)
exit=2

mutated (row 4):
error: could not reserve a unique runner worktree directory after 16 attempts (last tried <TMPDIR>/mu1-probe/unwritable/agent-scaffold-checks-run-1830527-1785451629333369024-15)
exit=2
```

This is the identical misdiagnosis class `AD1a` (round 3) fixed one level up: a permissions fault reported as a name-collision exhaustion after 16 pointless `mkdir` syscalls, sending the user hunting for the wrong cause. It is real because the reservation's own doc (`src/checks.rs:480-485`) calls this exact mechanism, `claim_dir` creating or reporting-taken atomically, "what makes the returned path exclusively ours" — under this mutation that sentence is false and nothing notices.

### Why not caught by the new `AD1a` fix

The new test's own comment scopes itself correctly: "The loop's third outcome" is a property of `reserve_runner_worktree_with`'s handling of whatever `claim` returns, not of `claim_dir`'s own implementation. The test is not wrong or falsely worded; it simply does not reach one level deeper. This is a gap the round-3 fix could not have closed by construction, since the seam it added (the `claim: impl Fn(&Path) -> io::Result<bool>` parameter) exists precisely to bypass `claim_dir` for testability. The task's own framing anticipated exactly this ("Does the injection let a test establish something FALSE about the real `claim_dir` path?") — the answer is: not false, but it lets the suite look complete on the claim-error arm while leaving the arm that actually touches the filesystem completely dark.

### Proposed fix, BUILT AND MEASURED

Added directly after `a_directory_claim_is_exclusive` in `src/checks.rs`:

```rust
#[test]
fn claim_dir_propagates_a_non_collision_error_rather_than_reporting_it_taken() {
	// REVIEW CANDIDATE FIX (r4b, MU1). `claim_dir`'s own doc says it "propagates
	// every other error", meaning a non-`AlreadyExists` error from `fs::create_dir`
	// must come back as `Err`, not `Ok(false)` (which would misreport a real error,
	// e.g. a permissions fault, as merely "someone else holds this name" and drive
	// the reservation loop to retry it 16 times and misreport exhaustion). Driven
	// directly through `claim_dir`, not through the injected seam
	// `reserve_runner_worktree_with`'s own tests use, because that seam never calls
	// real `claim_dir` and so cannot see this arm at all.
	let dir = scratch("claim-dir-error");
	let path = dir.join("missing-parent").join("leaf");
	let error = claim_dir(&path)
		.expect_err("a claim under a missing parent directory must fail, not report the path taken");
	assert_ne!(
		error.kind(),
		io::ErrorKind::AlreadyExists,
		"a real error must stay distinguishable from a lost claim: {error}"
	);
	fs::remove_dir_all(&dir).unwrap();
}
```

It forces a real, non-`AlreadyExists` error out of `fs::create_dir` by targeting a path whose immediate parent does not exist (`NotFound`), the same "attack the real function, not the seam" approach `a_directory_claim_is_exclusive` already uses for the other two arms.

Measured:

| State | Result |
| --- | --- |
| `3f49012` + this test | GREEN, **387 passed, 0 failed**; `cargo clippy --all-targets` silent |
| plus mutation #4 (`Err(_error) => Ok(false)`) | **RED**, `claim_dir_propagates_a_non_collision_error_rather_than_reporting_it_taken` fails (373 passed; 1 failed) |

Cost: one test, about 18 lines, no production change, no new seam. It creates nothing that needs later cleanup (the failure happens before any directory is created; the one success-path directory it does create is removed at the end of the test itself).

---

## Attacks that FAILED (recorded because they are evidence about the property)

### FA-R4-1: forced worst-case cross-process uniqueness, constant clock, identical pid, fresh sequence per process, shared directory, no cleanup between rounds

Constructed the worst case the design admits, matching and then exceeding round 3's `FA1` rigor:

- Mutated `nanos()` to return the constant `0` (temporarily, reverted after).
- Added a temporary, bounded probe test (`probe_r4b_worst_case_cross_process`, inert unless `PROBE_ROLE` is set; removed before finishing).
- Ran TWO copies of the `--bin agent-scaffold` test binary as separate OS processes concurrently, both calling `reserve_runner_worktree(u32::MAX)` (the same constant "dead" pid both processes use as a fixture template) into one shared, pre-existing temp directory, 400 reservations requested per process per round, 3 rounds, **deliberately not cleaning up the reserved directories between rounds** (stricter than round 3's approach, which cleaned per round), so later rounds' `seq` values starting again from 0 collide with directories earlier rounds already created and left on disk.

```
round 1: A=400 paths, B=400 paths, 0 errors each
round 2: A=350 paths, B=344 paths, 50/56 errors (exhaustion, from colliding with round 1's leftover directories)
round 3: A=306 paths, B=304 paths, 94/96 errors
```

**2400 reservations attempted, 2104 succeeded, 0 paths shared between any pair of the 2104** (checked via `sort | uniq -d` across the combined output of all 6 process-rounds). The 296 failures were all the documented loud exhaustion error (`AlreadyExists`, "after 16 attempts"), never a silently shared path. This is a harder scenario than round 3's `FA1` (4000 attempted, 3997 succeeded, 0 shared, with per-round cleanup): every one of my 2400 attempts drew candidate names from the SAME clock-and-sequence template as some earlier attempt still sitting on disk, and the property still held.

Mutation and probe test both reverted; `git diff HEAD` confirmed empty afterward.

### FA-R4-2: the new integration test fails identically to its pre-existing sibling under an inherited absolute `GIT_DIR`, and this is not new

Ran `target/debug/deps/checks_missing_tmpdir-*` directly with `GIT_DIR=<this worktree>/.git GIT_WORK_TREE=<this worktree> GIT_INDEX_FILE=<this worktree>/.git/index GIT_PREFIX=` set in the parent (an absolute path to an unrelated repository, more hostile than round 3's `FA5`, which used a relative `GIT_DIR=.git`):

```
thread 'checks_runs_under_a_tmpdir_that_does_not_exist_yet' panicked at tests/checks_missing_tmpdir.rs:31:5:
git ["add", "."] failed: fatal: Unable to create '<worktree>/.git/index.lock': Not a directory
test result: FAILED. 0 passed; 1 failed
```

Ran the pre-existing sibling `tests/checks_staged_hook_env.rs` binary the identical way:

```
thread 'checks_staged_runs_under_a_hook_environment' panicked at tests/checks_staged_hook_env.rs:24:5:
git ["add", "."] failed: fatal: Unable to create '<worktree>/.git/index.lock': Not a directory
test result: FAILED. 0 passed; 1 failed
```

Both fixtures share the identical `git()` helper (`Command::new("git").arg("-C").arg(dir)`, no `.current_dir()`), so both are equally exposed. This reproduces round 3's `FA6` finding class exactly (a hostile inherited git environment breaks the test FIXTURE's own scratch-repo setup, identically in old and new files) and `3f49012` changed zero lines in either test file, so nothing here is attributable to this commit. Not a new finding, per the settled precedent.

### FA-R4-3: relative `TMPDIR` makes the test itself unreliable, but this is the already-settled relative-`TMPDIR` class, not a new production defect

Ran the built binary via the integration test's own harness with the ambient `TMPDIR` for the whole test PROCESS set to a relative path (`reltmp`) from a scratch working directory. The test FAILED, but not on the assertion that the run succeeds (`checks_with_tmpdir(...) == Some(0)` passed): it failed on `assert!(missing.is_dir(), ...)`, because the CHILD's `.current_dir(dir)` combined with a relative `TMPDIR` passed to the child causes the child to resolve the same relative string against a DIFFERENT base than the parent test process did, so the child creates its directory at a different (but still legal) location than the one the parent process checks. Production `run()` still exits 0. This is the settled "relative-`TMPDIR` validation" item on the out-of-scope list, encountered from the test-harness side rather than the production side; not raised as a new finding.

Symlinked `TMPDIR` and unset `TMPDIR` (falls back to `/tmp`) both passed the test correctly and self-cleaned (`0` leftover directories after each).

A read-only-`TMPDIR` variant of this probe was attempted but is a null result by construction: setting the ambient `TMPDIR` for the whole test PROCESS to a `chmod 555` directory breaks the test's OWN scratch-git-repo setup (`fs::create_dir_all(&dir).unwrap()` at `tests/checks_missing_tmpdir.rs:59`) before the child process is ever invoked, so it exercises nothing about the reservation. The read-only-`TMPDIR` scenario against real production code IS covered, directly, by the `MU1` A/B measurement above (a `chmod 555` directory passed as the CHILD's `TMPDIR` while the test's own scratch repo lives elsewhere), which correctly reports `Permission denied (os error 13)` on the unmutated tree.

---

## NON-FINDINGS (settled items revisited only where new evidence surfaced)

- **`fetch_add` -> `load` staying GREEN (row 6).** Reconfirmed GREEN, 386 passed, 0 failed. This is the accepted `X1` residual exactly as documented; not raised.
- **`RUNNER_RESERVE_ATTEMPTS` 16 -> 3 staying GREEN (row 9).** Reconfirmed GREEN, 386 passed, 0 failed. Explicitly settled; not raised.
- **The temp-dir-creation error arm (`fs::create_dir_all(&temp)`'s `map_err`) surviving.** Not independently re-attacked this round (it is the accepted `AD1b` residual, a different call site and a different match arm than `MU1`); noted for completeness only, not raised.
- **Seq draw hoisted out of the retry loop (row 12).** Reconfirmed GREEN, near-equivalent mutant exactly as round 3's `FA2` describes (a call still draws one process-wide-unique `seq` per reservation; only the in-loop retry timing changes). Not raised.

No item on the settled list is disputed.

---

## VERDICT

**Not zero.** One finding: **MU1, `medium`**. It is a genuine gap the round-3 fix could not have closed by construction (the fix's own testability seam bypasses the exact function this finding is about), it reproduces the identical user-visible misdiagnosis class that `AD1a` was rated `medium` for, and a one-test fix is built, measured, and shown to kill it while staying green on the unmutated tree.

Every other attack in this round failed to find anything: the re-run "delete a load-bearing part" sweep shows the suite strictly stronger than at any prior round (4 of 5 core parts RED, versus round 2's 4-of-5 GREEN), the forced worst-case concurrent-process attack found 0 shared paths in 2104 successful reservations under conditions more adversarial than round 3's own worst-case measurement, and the three environment attacks against `tests/checks_missing_tmpdir.rs` either passed correctly or reproduced already-settled, already-documented classes of test-harness fragility that predate this commit and that this commit's diff (which touches only `src/checks.rs`) could not have introduced.

---

## Reverted state and hygiene

**Every mutation was reverted.** Verified after the last revert:

```
$ git status --short
(empty)

$ git diff HEAD
(empty)

$ diff <saved pristine copy of src/checks.rs> src/checks.rs
(empty; sha256sum of both files matches: 8714de16caed99cb1bf89e3d2192796355d305d7b79a08c9fee4d175e84624c8)
```

Re-confirmed on the reverted tree: `cargo test` 373 + 5 + 1 + 1 + 3 + 1 + 2 = 386 passed, 0 failed; `cargo clippy --all-targets` silent.

Mutations applied and reverted, in order: M1a, M1b, M1c (claim's error arm swallowed/dropped/inverted), MU1's own mutation and its candidate fix (applied together and separately), the temp-dir-creation deletion, `fetch_add` -> `load`, `if claimed || true`, `RUNNER_RESERVE_ATTEMPTS` 16 -> 1 and 16 -> 3, `claim_dir`'s `create_dir` -> `create_dir_all`, the exhaustion-message bound drop (M4), the `seq`-hoist, the loop's off-by-one range, `owning_pid`'s `.next()` -> `.last()`, `pid_is_alive`'s inversion, and the `run()` call site's `prune_orphan_worktrees` comment-out. Every one reverted with `git checkout -- src/checks.rs` before the next mutation.

**Directories created in `/tmp` by this review: 0, after one correction.** One Bash call (the `checks_staged_hook_env` sibling-parity check under `FA-R4-2`) omitted the `TMPDIR` export for that specific shell invocation (each tool call starts a fresh shell; the export does not persist across calls), which let one failing test leak `/tmp/agent-scaffold-hookenv-1858741-staged` directly into `/tmp` before its own cleanup could run. Caught during final hygiene audit (`find /tmp -maxdepth 1 -iname "agent-scaffold-*" -newermt "2026-07-30 23:00"`), removed immediately, and reverified empty. `find /tmp -maxdepth 1 -iname "agent-scaffold-checks-run-*"` returns **0**; `find /tmp -maxdepth 1 -iname "agent-scaffold-*"` returns the same **65** pre-existing entries this session's other reviewers have already recorded (none touched, none created by this review); a full `find /tmp -maxdepth 1 -newermt "2026-07-30 23:00"` (not restricted to the `agent-scaffold-*` pattern) returns empty after cleanup.

`TMPDIR` was exported to `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/r4b-tmp` for every `cargo test` / `cargo build` / `cargo clippy` run and for the bulk of direct binary invocations in this review. That directory and its contents (mutation-run scratch git repos, the concurrent worst-case probe's reserved directories, a saved pristine copy of `src/checks.rs` used for the final byte comparison) were removed at the end; `find <scratch TMPDIR> -mindepth 1` returns empty. No exhaustion-path or unbounded probe was run; the largest probe was the 2400-reservation worst-case attack above, bounded and cleaned up as described.
