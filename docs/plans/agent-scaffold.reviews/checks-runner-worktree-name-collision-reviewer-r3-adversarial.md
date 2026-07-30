# Reviewer, ADVERSARIAL lens: `checks-runner-worktree-name-collision`, commit `6a726ed`, round 3

Artifact under review: `6a726ed` ("fix(checks): pin the reservation's collision path and the missing-TMPDIR fix"), the round-2 fix pass, reviewed in an isolated worktree at `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev93-r3-b` on branch `review/checks-collision-r3b`, HEAD `6a726ed989c74a3620995bd84ba7474d694f0f96`.

Baseline on the committed tree, measured here, not quoted from the commit message: `cargo test` 372 + 5 + 1 + 1 + 3 + 1 + 2 = **385 passed, 0 failed**; `cargo clippy --all-targets` silent. Every test run in this review used `TMPDIR` pointed at a scratch directory in this session's scratchpad, never `/tmp`.

Three findings: one `medium`, two `low`. Eleven attacks failed and are recorded below, because an attack that could not break the property is evidence about the property.

---

## AD1: the reservation's two ERROR arms are executed by no test, and each can be deleted with a green suite

**Severity: `medium`.**

### Claim

`6a726ed` built a testability seam specifically so the reservation loop's outcomes could be driven at their use site, and drove three of them (the lost-claim verdict, the retry, the exhaustion error). It missed the fourth: the arm that handles a claim failing for a reason that is NOT a collision. It also leaves unexecuted the error arm of the `create_dir_all(&temp)` call the same commit's new integration test was written to pin. Both arms carry an explicit behavioural claim in a comment this commit either wrote or rewrote, and both can be deleted with all 385 tests green and clippy silent.

The two comments that make the claims:

- `src/checks.rs:531-534`: "Any other error (an unwritable temp dir) is not a collision, so it propagates immediately rather than being retried 16 times and then misreported as exhaustion."
- `src/checks.rs:517-520` (rewritten by this commit): "Every failure below is reported with the path it was working on: without that, a bad `TMPDIR` reaches the user as a bare `No such file or directory (os error 2)` naming neither the operation nor the path."

Neither sentence is executed by anything.

### Reproduced evidence

Three mutations, each applied alone to the committed tree, full `cargo test` plus `cargo clippy --all-targets` each time, then reverted.

**Mutation M1a, `src/checks.rs:535-543`, swallow the claim's error:**

```
let claimed = claim(&path).map_err(|error| { ... })?;   ->   let claimed = claim(&path).unwrap_or(false);
```

Result: **GREEN. 372 + 5 + 1 + 1 + 3 + 1 + 2 = 385 passed, 0 failed.** `cargo clippy --all-targets` silent (exit 0, no diagnostics).

The user-visible consequence, measured with the built binary against a scratch repo carrying one trivial `lint` check, with `TMPDIR` naming an existing but UNWRITABLE directory (`chmod 555`):

```
committed 6a726ed:
error: could not reserve the runner worktree directory <TMPDIR>/agent-scaffold-checks-run-1619660-1785446221211190518-0: Permission denied (os error 13)
exit=2

mutated (M1a):
error: could not reserve a unique runner worktree directory after 16 attempts (last tried <TMPDIR>/agent-scaffold-checks-run-1618616-1785446188542932588-15)
exit=2
```

That is exactly the misdiagnosis `:531-534` says the code prevents: a permissions problem reported as a name-collision exhaustion, after 16 pointless `mkdir` syscalls, sending the user hunting for the wrong cause. The suite does not notice.

**Mutation M1b, `src/checks.rs:535-543`, keep the propagation but drop the message wrapping:**

```
let claimed = claim(&path).map_err(|error| { ... })?;   ->   let claimed = claim(&path)?;
```

Result: **GREEN, 385 passed, 0 failed.** Same probe:

```
mutated (M1b):
error: Permission denied (os error 13)
exit=2
```

"naming neither the operation nor the path", which is the precise failure mode `:517-520` says is prevented. This is also the requirement round 1 recorded as `T2` and the round-2 triage re-endorsed under `X7` ("the message now names the operation and the path, which is what `T2` asked for"). Nothing pins it.

**Mutation M17, `src/checks.rs:521-526`, swallow the temp-dir creation error:**

```
fs::create_dir_all(&temp).map_err(|error| { ... })?;   ->   let _ = fs::create_dir_all(&temp);
```

Result: **GREEN, 385 passed, 0 failed.** With `TMPDIR` set two levels under a REGULAR FILE:

```
committed 6a726ed:
error: could not create the temp directory <SCRATCH>/afile/sub: Not a directory (os error 20)
exit=2

mutated (M17):
error: could not reserve the runner worktree directory <SCRATCH>/afile/sub/agent-scaffold-checks-run-1646434-1785447245808294471-0: Not a directory (os error 20)
exit=2
```

The new integration test pins the SUCCESS path of that call (deleting the call outright is RED, reproduced below in FA11) and nothing pins its failure path, so the operation the error names can silently regress to the pre-fix one.

### Why `medium`

This is the unfixed remainder of the finding the round-2 triage carried as `X2` and rated `medium`, in the same function, in the same loop, under the same argument: the module's own doc states the behaviour and nothing executes it. The triage's own rule for the retry half applies verbatim here ("it shares a fix with the claim-verdict half and the group takes the severity of what it protects"). The commit built the seam that makes this arm testable in three lines and then did not use it, so the round closes with the collision path three-quarters driven and the record saying it is driven.

I considered `low`, on the ground that nothing shipped is wrong and the consequence of a regression is a wrong error string rather than data loss. I am not taking it, for two reasons: the standard being applied on this step is "tests must actually exercise the code they claim to" applied to newly written comments, and the misdiagnosis M1a produces is the one a user is most likely to actually meet (a full, read-only or otherwise unusable temp dir is a real environment, whereas 16 consecutive name collisions is not).

### Proposed fix, BUILT AND MEASURED

I implemented it, ran it, and reverted it. It is two tests, no production change.

1. A unit test using the seam this commit already added, next to the two it added (`src/checks.rs`, after `a_claim_that_never_wins_fails_at_the_attempt_bound`): inject a claim that returns `Err(PermissionDenied)`, assert it was offered exactly ONE path (not retried), that `error.kind()` is still `PermissionDenied` (so a real error stays distinguishable from exhaustion), and that the message contains both "could not reserve the runner worktree directory" and the offered path.
2. A second integration test in the existing `tests/checks_missing_tmpdir.rs`, same fixture shape as the one already there: set `TMPDIR` to a path under a regular file, assert exit 2 and that stderr contains "could not create the temp directory" and the path.

Measured, all with `TMPDIR` in the scratchpad:

| State | Result |
| --- | --- |
| `6a726ed` + both tests | GREEN, **387 passed, 0 failed**; `cargo clippy --all-targets` silent |
| `6a726ed` + both tests, 10 consecutive full `cargo test` runs | GREEN 10, RED 0 |
| plus M1a (`claim(&path).unwrap_or(false)`) | RED, `a_claim_error_propagates_at_once_and_names_the_path_it_failed_on` fails (372 passed, 1 failed) |
| plus M1b (`claim(&path)?`) | RED, same test fails (372 passed, 1 failed) |
| plus M17 (`let _ = fs::create_dir_all(&temp);`) | RED, `checks_under_an_unusable_tmpdir_names_the_operation_and_the_path` fails at `tests/checks_missing_tmpdir.rs:121` |

The working patch is 77 added lines across the two files. The triager should re-derive it rather than take my wording, but the shape is measured, not predicted.

---

## AD2: the exhaustion test's bound assertion is satisfied by the path's own digits, so it pins nothing

**Severity: `low`.**

### Claim

`src/checks.rs:1759-1762`:

```rust
		assert!(
			message.contains(&RUNNER_RESERVE_ATTEMPTS.to_string()),
			"the error must name the bound it gave up at: {message}"
		);
```

`RUNNER_RESERVE_ATTEMPTS.to_string()` is the two-character string `"16"`, and the message it is searched in ends with a full filesystem path containing the pid (7 digits), the nanosecond clock reading (19 digits) and the sequence value. The assertion is therefore satisfied by any `"16"` anywhere in that path, and the test's stated subject ("the error must name the bound it gave up at", and its comment at `:1736-1739`, "the error must say how many attempts it made") is not pinned.

### Reproduced evidence

Mutation M4: delete the bound from the exhaustion message at `src/checks.rs:549-556`.

```
"could not reserve a unique runner worktree directory after {RUNNER_RESERVE_ATTEMPTS} attempts (last tried {})"
  ->
"could not reserve a unique runner worktree directory (last tried {})"
```

- The test alone, `cargo test --bin agent-scaffold -- --exact checks::tests::a_claim_that_never_wins_fails_at_the_attempt_bound`, **100 trials: 100 spurious passes, 0 failures.**
- Full `cargo test`, **5 trials: GREEN 5, RED 0** (385 passed each time).

The mechanism, captured directly by adding a temporary `eprintln!("PROBE_MSG={message}")` above the assertion under M4 and running with `--nocapture` (probe reverted):

```
PROBE_MSG=could not reserve a unique runner worktree directory (last tried /tmp/.../rev-b-tmp/agent-scaffold-checks-run-1629338-1785446609597928735-15)
PROBE_MSG=could not reserve a unique runner worktree directory (last tried /tmp/.../rev-b-tmp/agent-scaffold-checks-run-1629346-1785446609673765315-15)
PROBE_MSG=could not reserve a unique runner worktree directory (last tried /tmp/.../rev-b-tmp/agent-scaffold-checks-run-1629354-1785446609747342663-15)
```

The "16" is the first two digits of the pid (`1629338`). This machine's pids are currently in the 1.6 million range, which is why the observed rate here is 100%, and I want that stated honestly: the 100/100 is a property of this machine's pid range at this moment, not a universal rate. The assertion is weak regardless of pid: with the pid, the 19-digit clock reading and the sequence contributing roughly 26 two-digit windows, a uniform-digit model puts the spurious-pass rate at about 1 - 0.99^26, that is roughly 23%, even when the pid does not begin with "16". I did not measure that residual rate, because I cannot choose the pid; treat the 23% as arithmetic, not measurement.

The related observation, same root cause: `assert_eq!(offered.len(), RUNNER_RESERVE_ATTEMPTS as usize, ...)` at `:1753-1757` is stated against the constant rather than a literal, so the constant's VALUE is unpinned above 3. Mutation `RUNNER_RESERVE_ATTEMPTS` 16 -> 3: **GREEN, 385 passed, 0 failed** (the retry test needs three offers and gets them; the exhaustion test compares against the mutated constant and agrees with itself). Under that mutation the comment at `:534`, "rather than being retried 16 times", becomes false with nothing noticing. I am not raising the constant as a separate finding, because it is a tuning value and no property depends on it being 16; it is recorded because it is the same weak-assertion shape.

### Proposed fix, UNMEASURED

Assert against a literal fragment rather than a substring that the payload can supply, for example `message.contains("after 16 attempts")` (or `contains(&format!("after {RUNNER_RESERVE_ATTEMPTS} attempts"))`, which keeps the constant coupled but adds the surrounding words so the path cannot satisfy it). I did NOT build or measure this one; do not adopt it on my word. Its cost is one line and its risk is that it re-couples the test to message wording, which this project has previously declined elsewhere (`X5`).

---

## AD3: the new doc comment on `reserve_runner_worktree_with` states, in the present tense, two things the same commit's own tests make false

**Severity: `low`.**

### Claim

`src/checks.rs:502-509`, written by this commit:

> `reserve_runner_worktree` (above) with its claim injected, which is the only way to drive the outcome the filesystem will not produce on demand. Every real claim in this repository WINS: production takes one path at a time and the prune fixtures take theirs sequentially, **so nothing ever exercises the lost-claim verdict, the retry, or the exhaustion error at their use site, and each of those can be deleted with a green suite.**

Both bolded clauses are false about the tree the comment sits in, and false because of the two tests the same commit added 200 lines below it. The paragraph contradicts itself inside four lines: it opens by introducing the injection as "the only way to drive the outcome", then says nothing drives it.

This matters in the concrete way the round-2 triage used to require `X8a`: a wrong action follows from believing it. A maintainer reading "each of those can be deleted with a green suite" directly above the retry loop is being told the loop is dead weight the suite does not defend, which is the opposite of what `6a726ed` just established.

### Reproduced evidence

Mutation: `src/checks.rs:544`, delete the lost-claim verdict, `if claimed {` -> `if claimed || true {`.

```
thread 'checks::tests::a_claim_that_never_wins_fails_at_the_attempt_bound' (1643119) panicked at src/checks.rs:1746:10:
thread 'checks::tests::a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one' (1643124) panicked at src/checks.rs:1726:9:
test result: FAILED. 370 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

So the lost-claim verdict CANNOT be deleted with a green suite, and the two tests that stop it are exactly the ones the comment says do not exist. (Mutation reverted.)

Contrast with `src/checks.rs:1693-1697`, the comment this same commit correctly rewrote in `a_directory_claim_is_exclusive`, which is careful to scope its claim ("no other test executes the LOST one **through `claim_dir` itself**") and which I verified true in FA3 below. The new `:502-509` comment did not get the same care.

### Proposed fix, UNMEASURED (documentation only)

Put the two clauses in the past tense, or scope them to real claims, so they read as the motivation for the seam rather than a statement about the current tree. Something of the shape "so BEFORE this seam nothing exercised the lost-claim verdict, the retry, or the exhaustion error at their use site, and each could be deleted with a green suite; the two tests below drive them through an injected claim." I have not built this; it is one sentence, and the triager should judge whether it is worth opening the comment at all, given this project's recorded evidence that prose-heavy fix passes manufacture the next round's findings.

---

# Attacks that FAILED

Recorded because they are evidence about the property.

## FA1 (the headline attack): forced worst-case cross-process uniqueness. Layer 2 held, 0 shared paths in 3997 reservations

The uniqueness argument's weakest admitted point is the constant-pid fixture namespace: `dead_pid()` is `u32::MAX` in every process, `NEXT_RUNNER_SEQ` starts at 0 in every process, so two concurrent processes share the entire name template and have only the clock between them. I forced that to its extreme rather than sampling it:

- Mutated `nanos()` to return the constant `0`, removing the clock as a discriminator entirely.
- Added a temporary test (inert unless `PROBE_OUT` is set) that calls `reserve_runner_worktree(u32::MAX)` 400 times and records every returned path.
- Ran TWO copies of the test binary concurrently against a SHARED, freshly created temp dir, five rounds.

Both processes therefore generated byte-identical candidate name sequences from seq 0 upward, into one directory, at the same time.

```
round 1: A=400 paths, B=400 paths, SHARED=0, A-ERRORS 0 B-ERRORS 0
round 2: A=400 paths, B=400 paths, SHARED=0, A-ERRORS 0 B-ERRORS 0
round 3: A=400 paths, B=399 paths, SHARED=0, A-ERRORS 0 B-ERRORS 1
round 4: A=399 paths, B=399 paths, SHARED=0, A-ERRORS 1 B-ERRORS 1
round 5: A=400 paths, B=400 paths, SHARED=0, A-ERRORS 0 B-ERRORS 0
```

**4000 reservations attempted, 3997 returned a path, 0 paths shared between the two processes**, and the 3 failures were the documented exhaustion error at the 16-attempt bound, which is the loud failure the design says a hostile namespace should produce. This is the first measurement I can find on the record that executes the exhaustion error through the PRODUCTION `claim_dir` rather than an injected one. Layer 2 behaves exactly as `src/checks.rs:481-485` claims: a collision becomes a retry, and an unwinnable namespace becomes an error, never a shared path.

It also confirms the `nanos()` doc's claim at `src/checks.rs:1024-1026` that "a constant returned here would still be correct, only slower": correct held (0 shared paths), and the cost was 3 failed reservations in 4000 under a load no real run produces.

Mutation and probe test both reverted.

## FA2: hoisting the sequence draw out of the retry loop is NOT caught, but it is a near-equivalent mutant, so I am not raising it

Mutation: move `let seq = NEXT_RUNNER_SEQ.fetch_add(1, Ordering::Relaxed);` from inside the `for` at `src/checks.rs:529` to above it, so all 16 attempts of one reservation share one sequence value and differ only by the clock.

Result: **GREEN, 20 consecutive full `cargo test` runs, 20 GREEN 0 RED.**

I worked out whether that breaks anything before raising it, and it does not. Each CALL to `reserve_runner_worktree_with` still draws exactly one process-wide-unique sequence value, so two concurrent calls in one process still have disjoint candidate-name sets, which is the in-process channel the whole step exists to close. Within one call, the retry needs only a name different from the one just lost, and there is a `mkdir` syscall (microseconds) between two `nanos()` readings whose resolution is about 25 ns, so consecutive attempts get distinct clock values in practice. The only thing that becomes false is the descriptive sentence at `src/checks.rs:432-434` ("Each attempt draws a fresh sequence value"), which is true of the committed code. A comment that is true is not a finding.

I am recording this in full because my first measurement of it was WRONG and I want the correction on the record: I initially ran `cargo test --lib`, which this crate has no target for, so `cargo` exited 101 every time and I read 40/40 "failures" as a kill. The crate is a binary; the unit tests live in `--bin agent-scaffold`. Every earlier-cited number in this file was re-derived with a valid target.

## FA3: `a_directory_claim_is_exclusive`'s new comment is TRUE at its boundary

The comment this commit rewrote at `src/checks.rs:1693-1697` claims "nothing else would notice if this stopped reporting a taken path as taken". Mutation at `src/checks.rs:451`, `Err(error) if error.kind() == AlreadyExists => Ok(false)` -> `Ok(true)`:

```
thread 'checks::tests::a_directory_claim_is_exclusive' (1631029) panicked at src/checks.rs:1707:9:
test result: FAILED. 371 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Exactly one test, and it is the one the comment names. The claim holds. Reverted.

## FA4: the new integration test is deterministic here, 50/50

`cargo test --test checks_missing_tmpdir`, 50 consecutive runs: **pass=50, fail=0**, and `0` leftover `agent-scaffold-missingtmp-*` directories in the scratch `TMPDIR` afterwards.

## FA5: the new integration test survives an inherited hook-shaped git environment

The test does not clear the parent environment before spawning the binary, and this project's own scaffolded pre-commit hook runs `checks`, so a developer could plausibly run the suite with git's hook variables exported. Running the test binary with `GIT_DIR=.git GIT_INDEX_FILE=.git/index GIT_WORK_TREE=. GIT_PREFIX=` set in the parent:

```
test checks_runs_under_a_tmpdir_that_does_not_exist_yet ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

Unaffected.

## FA6: the new integration test IS sensitive to a hostile global git config, but so is the pre-existing sibling, identically

With `GIT_CONFIG_GLOBAL` pointing at a config whose `core.excludesFile` ignores `.agents/` and `*.txt`, the fixture's `git add .` stages nothing and `git commit` fails:

```
--- NEW test ---
failures:
    checks_runs_under_a_tmpdir_that_does_not_exist_yet
test result: FAILED. 0 passed; 1 failed

--- pre-existing sibling tests/checks_staged_hook_env.rs ---
failures:
    checks_staged_runs_under_a_hook_environment
test result: FAILED. 0 passed; 1 failed
```

The exposure is the repository's existing integration-fixture convention (the new file is a near copy of `tests/checks_staged_hook_env.rs`, including the `let _ = fs::remove_dir_all(&dir);` at the top and the unguarded `fs::remove_dir_all(&dir).unwrap();` at the bottom that leaks the scratch tree on failure). `6a726ed` did not introduce it and closing it is not this step's business. Not a finding.

## FA7: the test puts its `TMPDIR` INSIDE the repository under test, and that is not masking anything

`missing` is `dir/missing/nested` where `dir` is the scratch repo root, so the runner's worktree is created inside the repository's own working tree, which the production call site's comment at `src/checks.rs:954` describes as "A temp path OUTSIDE the repository". I checked whether the in-repo placement is what makes the test pass, by running the built binary with `TMPDIR` set to a two-level missing path OUTSIDE the repo:

```
--- (2) TMPDIR nested-missing OUTSIDE the repo: <SCRATCH>/outside/missing/nested ---
        pass  lint (lint)
checks: 1 passed, 0 failed, 0 skipped
exit=0
created=yes
```

Same outcome. The test's configuration is not load-bearing for its result.

## FA8: I could not make the integration test pass while the behaviour it pins is broken

Deleting `fs::create_dir_all(&temp)` at `src/checks.rs:521-526` entirely:

```
thread 'checks_runs_under_a_tmpdir_that_does_not_exist_yet' (1633984) panicked at tests/checks_missing_tmpdir.rs:78:5:
test result: FAILED. 0 passed; 1 failed
```

Caught, and caught by that test alone (the 372 unit tests and the other five integration binaries all stayed green). The `assert!(missing.is_dir(), ...)` second assertion also forecloses the escape I looked for, where the run exits 0 without ever reaching the reservation: nothing else in the binary creates `std::env::temp_dir()` (`grep -n "create_dir_all\|temp_dir()" src/*.rs` shows the runner path's only production site is `:521`), so the directory's existence after the run cannot come from anywhere else.

## FA9: I attacked Invariant B's new clause at its edges and could not falsify it

`src/checks.rs:48-56`. Each claim checked against `prune_orphan_worktrees` at `:592-623`:

- "the prune additionally requires the worktree path GIT RECORDED to sit under the CURRENT process's `std::env::temp_dir()`" matches `if !path.starts_with(&temp) { continue; }` at `:601`. TRUE.
- `Path::starts_with` is component-wise, so `/tmp/x` does not prefix-match `/tmp/xy/...`; no false positive at that boundary.
- A trailing slash or a trailing `.` in `TMPDIR` is normalised away by `Components`, so the gate still matches the paths the reservation builds. No edge found.
- "git records that path symlink-resolved" was measured by the round-2 triage and I did not re-derive it.

The one wobble is "is never reclaimed either", which is unconditional while one of its two examples is not: an orphan left under `TMPDIR=/x` is skipped by a run under `TMPDIR=/y` but would be reclaimed by a later run under `/x`. The sentence's subject is scoped ("outside THIS PROCESS'S temp dir"), which makes the charitable reading correct, and the wording is the round-2 triage's own prescribed clause. I am not raising it; see the non-findings section.

## FA10: the rewritten `claim_dir` comment at `src/checks.rs:515-520` is accurate

"`claim_dir` deliberately creates exactly ONE level (tolerating a directory that already exists is what would destroy its exclusivity), so the temp dir's own leading directories are created here, once, outside the retry loop." `fs::create_dir` creates one level; the parenthetical now names the real hazard (which is what `X8a` asked for); the "so" still follows from the one-level property, which is still stated. "Every failure below is reported with the path it was working on" is true of both failure sites below it (`:535-543` and `:549-556`). It is unexecuted (AD1), but it is not false.

## FA11: the "delete a load-bearing part" sweep, round 2 versus round 3

Round 2 established that four of five load-bearing parts could each be deleted alone with 382 tests green. Re-run against `6a726ed`, each mutation alone, full `cargo test`:

| Part deleted or neutered | Round 2 | This commit |
| --- | --- | --- |
| `fs::create_dir_all(&temp)` (`:521-526`) | GREEN | **RED** (the new integration test) |
| the lost-claim verdict, `if claimed` (`:544`) | GREEN | **RED** (both new unit tests) |
| the retry bound, `RUNNER_RESERVE_ATTEMPTS` 16 -> 1 | GREEN | RED (per commit message; I did not re-derive, I measured 16 -> 3 instead, which is GREEN) |
| `claim_dir`'s `create_dir` exclusivity | RED | RED (unchanged, one test) |
| `fetch_add` -> `load` | GREEN | GREEN (the accepted `X1` residual, out of scope) |
| **the claim's error arm (`:535-543`)** | not tried | **GREEN (AD1)** |
| **the temp-dir creation's error arm (`:521-526`)** | not tried | **GREEN (AD1)** |
| `seq` draw hoisted out of the loop | not tried | GREEN, near-equivalent mutant (FA2) |

The new tests did change the answer, on the two rows the commit set out to change. The two rows they did not reach are both error arms, and one of them sits inside the very loop the commit's seam was built for.

---

# NON-FINDINGS

Items I considered and am deliberately NOT raising, listed so the triager can see they were weighed rather than missed.

1. **Invariant B's "is never reclaimed either" is unconditional where one of its two examples is recoverable** (FA9). It is the round-2 triage's own prescribed wording, the sentence's subject is already scoped to "this process's temp dir", and re-opening a comment to qualify a word is exactly the move this project has recorded as manufacturing the next round's findings. Not raised.
2. **Invariant B names one additional prune requirement and reads as exhaustive, while the prune also gates on the owning pid being dead.** The liveness gate is documented at `:569-591` and its pid-reuse edge is explicitly self-healing ("reclaimed by a later run"), so it is not part of the permanent bound Invariant B is stating. Not raised.
3. **The integration test does not assert stdout, so it cannot distinguish "ran the check" from "ran zero checks".** Its subject is the reservation, the `missing.is_dir()` assertion forces the reservation to be reached, and asserting the check output would duplicate coverage that already exists in `checks::tests`. Not raised.
4. **The new integration test leaks its scratch tree when it fails** (observed under FA8: `agent-scaffold-missingtmp-1633982` left behind). Identical to the pre-existing sibling, and the `let _ = fs::remove_dir_all(&dir);` at the top makes the next run with the same pid self-clean. Convention, not a defect of this commit.
5. **`TMPDIR` two levels under a regular file reports `Not a directory (os error 20)`, whereas the round-2 triage recorded `File exists (os error 17)` for `TMPDIR` being the regular file itself.** Different setups, both correct, both exit 2. Not a discrepancy, and `X7` is out of scope anyway.

## On the declared out-of-scope list

I agree with all six exclusions and am not disputing any of them. Two notes on evidence rather than scope:

- The `fetch_add` -> `load` residual (`X1`) is out of scope and I did not raise it, but FA1 supplies a measurement the accept-residual record did not have: with the clock also removed, two concurrent processes marching through identical name sequences produced 0 shared paths and 3 exhaustion errors in 4000 reservations. That is direct evidence for the recorded reason ("removing it turns collisions into retries rather than shared paths") rather than an argument for it.
- The symlink-resolved prune gate stays out of scope; my only contact with it was verifying the prose that describes it (FA9).

---

# VERDICT

**This round does NOT have zero findings from the adversarial lens.** Three findings: **AD1 `medium`**, **AD2 `low`**, **AD3 `low`**.

AD1 is the one I would keep if only one could be taken: the commit built a seam precisely so the reservation's outcomes could be driven, drove three of four, and left the fourth deletable with 385 green tests and a measured user-visible misdiagnosis. Its fix is two tests, no production change, and I built and measured it (387 passed, clippy silent, RED under all three mutations). AD2 and AD3 are both `low` and both are the "holds in the centre, fails at the boundary" class this step's sibling work has repeatedly produced: one assertion that a payload substring satisfies for free, one comment that is false about the tree it was written into.

The headline property survived every attack I could construct, including the worst case the design admits with the clock removed entirely. I found nothing wrong with the uniqueness argument, nothing wrong with the new integration test's determinism or its environment dependence relative to the repository's existing convention, nothing wrong with Invariant B's rewritten clause, and nothing wrong with the rewritten `claim_dir` comment.

---

# Worktree and temp-directory state

**Every mutation was reverted.** Final state, measured after the last revert:

```
$ git rev-parse HEAD
6a726ed989c74a3620995bd84ba7474d694f0f96

$ git status --short
(no output)

$ git diff HEAD
(no output)
```

Both are empty apart from this findings file, which is written after those commands and is left uncommitted for the orchestrator. Verification on the reverted tree: `cargo clippy --all-targets` silent; `cargo test` 372 + 5 + 1 + 1 + 3 + 1 + 2 = 385 passed, 0 failed.

Mutations applied and reverted, in order: M1a (claim error swallowed), M1b (claim error message dropped), M2 (seq hoisted), M4 (exhaustion bound removed from message) plus a temporary `eprintln!` probe, M5 (`claim_dir` reports taken as won), M15 (`RUNNER_RESERVE_ATTEMPTS` 16 -> 3), M17 (temp-dir creation error swallowed), P1 (`create_dir_all(&temp)` deleted), the AD3 verdict mutation (`if claimed || true`), the FA1 probe (constant `nanos()` plus a temporary test), and the AD1 candidate fix (two tests across `src/checks.rs` and `tests/checks_missing_tmpdir.rs`). All reverted with `git checkout --`.

**Temp-directory hygiene.** `TMPDIR` was exported to the scratchpad path for every test run, build, probe and A/B in this review.

- **Directories I created in `/tmp`: 0.** `/tmp` held 65 `agent-scaffold-*` entries when I started and holds 65 now, all of them `agent-scaffold-checks-test-*` predating this session; none are mine and I touched none of them. `/tmp` holds **0** `agent-scaffold-checks-run-*` directories.
- No exhaustion-path or high-volume probe was run against `/tmp`. FA1 was bounded at 400 reservations per process, 2 processes, 5 rounds, all inside a directory under the scratch `TMPDIR` that the probe script deleted at the end of each round and again at the end.
- Persistent entries left in the scratch `TMPDIR` by deliberately-failing mutation runs: 3 (`agent-scaffold-missingtmp-1633982`, `agent-scaffold-hookenv-1641805-staged`, `agent-scaffold-badtmp-1683487`), all deleted. The scratch `TMPDIR` is empty at the time of writing.
