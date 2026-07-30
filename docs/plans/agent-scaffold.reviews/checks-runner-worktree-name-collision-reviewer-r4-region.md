# Reviewer: `checks-runner-worktree-name-collision` (commit `3f49012`, round 4, REGION SWEEP lens)

Reviewed in an isolated worktree at `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/impl-checks-collision/.claude/worktrees/rev93-r4-a`, branch `review/checks-collision-r4a`, HEAD `3f490128ef34c608dac134a313bdb69972e0daf0`. Every mutation and probe below was applied here, built, measured and reverted. `TMPDIR` was exported to this session's scratchpad for every build, test run and binary probe; nothing was created in `/tmp`.

Baseline reconfirmed here, not quoted: `cargo test` 373 + 5 + 1 + 1 + 3 + 1 + 2 = **386 passed, 0 failed**; `cargo clippy --all-targets` 0 warnings, 0 errors.

This lens reviews a REGION as it now stands, not a diff and not a list of fixes. Round 3's zero-finding lens swept over the sentences it had been pointed at rather than the region those sentences live in, and its two misses were one assertion three lines below the one it ruled on and one clause of a sentence whose other clause it ruled on. Two of the four findings below sit in exactly that relationship to round 3's fixes: `RG1` is the line directly underneath the line round 3 pinned, and `RG2` is the surviving half of the sentence round 3 half-deleted and then certified true.

---

# COVERAGE STATEMENT

True bounds established by reading, not taken from the assignment's approximations. Every line in every range below was read in full.

## `src/checks.rs`, swept

| Range | What it is |
| --- | --- |
| `:1-68` | The whole module doc comment, including Invariant A (`:34-36`), **Invariant B (`:37-56`)**, C (`:57-64`) and D (`:65-68`). Invariant B's true extent is `:37-56`, four lines longer than the `:42-51` in the assignment; the reservation/registration/reclamation text runs to the end of the Principle 18 sentence at `:56`. |
| `:91-98` | The `RUNNER_PREFIX` doc comment, which spells the name format and the fixture-prefix distinctness claim. |
| `:280-300` | `RunError::Io` and `RunError::exit_code`, read to adjudicate the exit-code claim in `tests/checks_missing_tmpdir.rs`. |
| `:335-365` | `WorktreeGuard`'s doc comment and its `Drop`. `:340-344` restates Invariant B's registration bound and is neighbouring invariant text under the assignment's item 1. |
| `:411-621` | The reservation machinery in full, its true bounds. `pid_is_alive` + doc (`:411-421`), `NEXT_RUNNER_SEQ` + doc (`:423-429`), `RUNNER_RESERVE_ATTEMPTS` + doc (`:431-436`), `claim_dir` + doc (`:438-454`), `reserve_runner_worktree` + doc (`:456-500`), `reserve_runner_worktree_with` + doc + every inline comment (`:502-555`), `owning_pid` + doc (`:557-565`), `prune_orphan_worktrees` + doc + inline comments (`:567-621`). The assignment's `:420-545` understates this at both ends. |
| `:886-1029` | `run()`'s doc comment (`:886-895`), its body, the prune call and its comment (`:943-946`), **the production reservation call site and its comment (`:952-957`)**, the guard construction and its comment (`:958-964`), the `git worktree add` and its error arm (`:965-972`), and `nanos()` with its doc (`:1012-1029`). The assignment named `:939`; the reservation call is at `:957`. |
| `:1038-1087` | The test helpers `scratch`, `git_ok`, `init_repo`, `write_config`, `worktree_paths`, read because the region's tests cannot be judged without them. |
| `:1612-1882` | Every test touching reservation, claiming, retry, exhaustion, the prune or worktree naming, plus `dead_pid` and its doc. True bounds `:1612` (the `dead_pid` doc) to `:1882` (the closing brace of `mod tests`), against the assignment's `:1600-1800`, which would have cut off `concurrent_reservations_never_share_a_runner_worktree_path` mid-body and missed `a_reserved_path_still_carries_its_owning_pid_as_the_first_component` entirely. Named individually: `dead_pid` `:1612-1619`, `a_startup_prune_reclaims_an_orphaned_runner_worktree` `:1621-1652`, `a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one` `:1654-1685`, `a_directory_claim_is_exclusive` `:1687-1707`, `a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one` `:1709-1731`, `a_claim_that_never_wins_fails_at_the_attempt_bound` `:1733-1764`, `a_claim_error_that_is_not_a_collision_propagates_at_once_and_names_the_path` `:1766-1793`, `concurrent_reservations_never_share_a_runner_worktree_path` `:1795-1858`, `a_reserved_path_still_carries_its_owning_pid_as_the_first_component` `:1860-1881`. |

## `tests/checks_missing_tmpdir.rs`, swept

`:1-86`, the whole file: the 17-line module doc, both helpers, and all four assertions of the single test.

## Deliberately NOT covered, so the next reviewer knows

`src/checks.rs:100-279` (`Kind`, `Check`, `parse`, `ParseError`), `:301-334` (`Display`, `From<io::Error>`), `:367-410` (`strip_git_env`, `git_command`, `git`), `:623-885` (`Isolation`, `isolation_commit`, `glob_match`, `glob_rec`, `any_tracked_matches`, `runnable_for`, `run_command`), `:1089-1611` (the parse, glob, isolation and report tests that touch no reservation). Other test binaries beyond `tests/checks_missing_tmpdir.rs` were not swept.

## Mutations run, all reverted

| id | Change | Site | Full suite | clippy |
| --- | --- | --- | --- | --- |
| N1 | `Err(error) => Err(error)` -> `Err(_) => Ok(false)` | `claim_dir` `:452` | **GREEN, 386 passed, 0 failed** | 0/0 |
| N2 | `Err(error) => Err(error)` -> `Err(_) => Ok(true)` | `claim_dir` `:452` | **GREEN, 386 passed, 0 failed** | 0/0 |
| N3 | `_guard` constructed AFTER the `git worktree add` | `run()` `:961-972` | **GREEN, 386 passed, 0 failed** | 0/0 |
| N4 | `fs::create_dir_all(&temp).map_err(...)?` deleted | `:519-524` | RED, `checks_runs_under_a_tmpdir_that_does_not_exist_yet` FAILED | n/a |
| N5 | `{seq}` dropped from the name format | `:528` | GREEN (the settled `X1` residual; see non-findings) | n/a |
| N6 | `fs::create_dir` -> `fs::create_dir_all` | `claim_dir` `:449` | RED, `a_directory_claim_is_exclusive` FAILED, 372 passed, 1 failed | n/a |
| N7 | `reserve_runner_worktree` passes `\|_\| Ok(true)` (names but never creates) | `:499` | RED, `a_reserved_path_still_carries_its_owning_pid_as_the_first_component` FAILED, 372 passed, 1 failed | n/a |

---

# RG1: `claim_dir`'s own error arm is executed by no test, and both mutations of it are green

**Severity: `medium`.**

## Claim

Round 3's `AD1a` fix pinned the loop's handling of a claim error, at `reserve_runner_worktree_with:533-541`, by injecting a claim that returns `Err`. That test (`a_claim_error_that_is_not_a_collision_propagates_at_once_and_names_the_path`, `:1766-1793`) never calls `claim_dir`. `claim_dir`'s own error arm at `:452` is therefore still executed by nothing, and the two user-visible failure modes round 3 measured as `M1a` and `M1c` remain reachable through it, one line lower than the line that was fixed.

`claim_dir`'s doc comment states three outcomes (`:438-440`): "Returns `Ok(true)` when THIS call created it ..., `Ok(false)` when it already existed ..., and propagates every other error." The test that exists for it asserts two, and its own comment at `:1690-1691` says so in as many words: "Both outcomes matter". The third has no assertion anywhere.

## Reproduced

Each mutation applied alone to the committed tree, full `cargo test` plus `cargo clippy --all-targets`, then reverted.

**N1**, `src/checks.rs:452`, `Err(error) => Err(error)` -> `Err(_) => Ok(false)` (a real error folded into the lost-claim verdict):

```
test result: ok. 373 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s
test result: ok. 5 passed / 1 passed / 1 passed / 3 passed / 1 passed / 2 passed
```

**GREEN, 386 passed, 0 failed**; clippy 0 warnings, 0 errors.

**N2**, same line, `-> Err(_) => Ok(true)` (an errored claim counts as WON): **GREEN, 386 passed, 0 failed**; clippy 0 warnings, 0 errors.

A/B of the built binary against a scratch repo with one trivial `lint` check and `TMPDIR` naming an existing but `chmod 555` directory, quoted verbatim:

```
committed 3f49012:
error: could not reserve the runner worktree directory <TMPDIR>/probe/unwritable/agent-scaffold-checks-run-1794523-1785451104836200280-0: Permission denied (os error 13)
exit=2

N1:
error: could not reserve a unique runner worktree directory after 16 attempts (last tried <TMPDIR>/probe/unwritable/agent-scaffold-checks-run-1791310-1785450951417221223-15)
exit=2

N2:
error: could not set up the isolation worktree: `git worktree add` failed: Preparing worktree (detached HEAD 416bf9e)
fatal: could not create leading directories of '<TMPDIR>/probe/unwritable/agent-scaffold-checks-run-1793658-1785451043702757760-0/.git': Permission denied
exit=2
```

N1 is the failure mode round 3 recorded as `M1a` and rated the ground for `medium`: a permissions fault reported to the user as name-collision exhaustion, after 16 pointless `mkdir` syscalls. It is not the same line; round 3 closed `M1a` at `:533-541` and it is still reachable at `:452`.

N2 is the failure mode round 3 recorded as `M1c` and called the deciding measurement for the whole round: `reserve_runner_worktree` returns a path whose claim was never established, which is exactly what `src/checks.rs:480-485` calls the thing that makes the returned path exclusively ours. The doc sentence is false under N2 and, as on `6a726ed`, no test observes it.

## Why this is not a reopening of `AD1a` or `AD1b`

`AD1a` was the `map_err(...)?` inside the retry loop; that line is now pinned and I re-derived it (the round-3 test goes RED under `claim(&path).unwrap_or(false)`). `AD1b` was `fs::create_dir_all(&temp)` at `:519-524`, accepted as a residual because `std::env::temp_dir()` has no seam and pinning it costs a spawned-binary test. Neither argument applies here: `claim_dir` takes a `&Path`, so its error arm is drivable from a unit test with no seam, no injection and no new fixture, and the consequence is `M1a` plus `M1c` rather than `AD1b`'s one degraded noun.

## Minimal fix, built and measured

13 lines added inside the existing `a_directory_claim_is_exclusive`, no new test function, no production change.

```rust
		assert!(!claim_dir(&path).unwrap(), "a second claim on the same path is lost");
+		// The THIRD outcome this documents, which neither assertion above reaches: a claim
+		// that fails for a reason OTHER than the path being taken propagates as an error
+		// rather than folding into either verdict. A regular file standing in for a parent
+		// directory produces one without needing a permissions fixture.
+		let file = dir.join("a-regular-file");
+		fs::write(&file, "not a directory\n").unwrap();
+		let error = claim_dir(&file.join("under-a-file"))
+			.expect_err("a claim that cannot be made is an error, not a verdict");
+		assert_ne!(
+			error.kind(),
+			io::ErrorKind::AlreadyExists,
+			"a real error must stay distinguishable from a lost claim: {error}"
+		);
		fs::remove_dir_all(&dir).unwrap();
```

Measured here, `13 insertions(+), 0 deletions(-)` in one file:

| State | Result |
| --- | --- |
| `3f49012` + fix | GREEN, 386 passed, 0 failed; clippy 0 warnings, 0 errors |
| plus N1 (`Err(_) => Ok(false)`) | **RED**, `a claim that cannot be made is an error, not a verdict: false` (372 passed, 1 failed) |
| plus N2 (`Err(_) => Ok(true)`) | **RED**, `a claim that cannot be made is an error, not a verdict: true` (372 passed, 1 failed) |

`assert_ne!` against `AlreadyExists` rather than an equality against a specific errno is deliberate, and it is the `X7` ruling applied rather than reopened: the property that matters is that a real error stays distinguishable from a lost claim, not which errno a given kernel returns for a directory under a regular file. The comment in the fix says only what the assertion does; it authors no claim about history, per round 3's `AD3` remedy.

---

# RG2: the seam's surviving doc sentence is false, measured, and round 3's triage certified it true on a grep that could not see the counterexample

**Severity: `low`.**

## Claim

`src/checks.rs:502-505`:

> `reserve_runner_worktree` (above) with its claim injected, which is the only way to drive the outcome the filesystem will not produce on demand. Every real claim in this repository WINS: production takes one path at a time and the prune fixtures take theirs sequentially.

Both sentences are false against the tree they sit in.

1. **"Every real claim in this repository WINS" is false.** `a_directory_claim_is_exclusive:1705` loses a real claim, through `claim_dir` itself, with no injection, deterministically, and asserts that it loses.
2. **"the only way to drive the outcome the filesystem will not produce on demand" is false.** The filesystem produces that outcome on demand at `:1705`, which is what the assertion there is for.
3. **The enumeration after the colon omits half the real-claim sites, including the only concurrent one.** `grep -n 'reserve_runner_worktree('` gives four call sites: `:957` (production), `:1640`/`:1670`/`:1671` (the prune fixtures), `:1826` (`concurrent_reservations_never_share_a_runner_worktree_path`, 8 threads x 250 = 2000 real claims released together on a `Barrier`) and `:1867`/`:1868` (`a_reserved_path_still_carries_its_owning_pid_as_the_first_component`). "Production takes one path at a time and the prune fixtures take theirs sequentially" names two of four and excludes the 2000-claim concurrent site, which is the opposite of sequential.

## Reproduced

`claim_dir`'s lost-claim arm instrumented with `eprintln!("PROBE_REAL_CLAIM_LOST {}", path.display())`, full unit suite under `--nocapture`, five runs, probe then reverted:

```
run 1: real lost claims across the whole unit suite = 1
run 2: real lost claims across the whole unit suite = 1
run 3: real lost claims across the whole unit suite = 1
run 4: real lost claims across the whole unit suite = 1
run 5: real lost claims across the whole unit suite = 1
```

The path, and the isolation that identifies it:

```
PROBE_REAL_CLAIM_LOST <TMPDIR>/agent-scaffold-checks-test-1828389-claim-dir/claim

$ cargo test --bin agent-scaffold a_directory_claim_is_exclusive -- --nocapture
PROBE_REAL_CLAIM_LOST <TMPDIR>/agent-scaffold-checks-test-1829172-claim-dir/claim
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 372 filtered out
```

So exactly one real claim in the repository loses, every run, and it is the one `a_directory_claim_is_exclusive` deliberately loses. 5/5, deterministic, not sampled.

## Why round 3 did not see it, which is the part worth recording

The sentence predates `3f49012`: `git log -S 'production takes one path at a time'` gives `14692f3` (round 2's fix), and `git log -S 'a_directory_claim_is_exclusive'` gives `339d26a` (round 1's fix). The test that falsifies it therefore landed BEFORE the sentence that denies it; the sentence was false the day it was written.

Round 3's `AD3` deleted the second half of this same sentence and explicitly certified the surviving half, at `checks-runner-worktree-name-collision-triage-r3.md:297`: "Every real claim in this repository WINS ... (verified true: `grep -n "reserve_runner_worktree" src/checks.rs` shows the production call and the fixtures all take the one-argument wrapper, sequentially)". That grep searches for `reserve_runner_worktree`. The counterexample is a direct call to `claim_dir` at `:1705`, which the grep cannot match, and the concurrent site at `:1826` is in the grep's own output and was not read as contradicting "sequentially". This is `AD3`'s species surviving `AD3`'s fix, in the clause next to the deleted one.

## Minimal fix, unmeasured beyond a compile

Prose only; no code changes and no test changes, so there is nothing to measure but a green suite. `AD3`'s precedent says delete rather than author, and pure deletion does not work here because the first sentence carries the load (it says why the seam exists) and is itself false. The narrowest true replacement of `:502-505` is to say what the injection is actually needed for, which is the loop's use site rather than the filesystem:

```
/// `reserve_runner_worktree` (above) with its claim injected. A claim can be made to
/// lose directly (`a_directory_claim_is_exclusive` does), but not at THIS use site:
/// the loop draws a fresh, unpredictable name each attempt, so no arranged directory
/// can be sitting at it. Production passes `claim_dir` and is otherwise unchanged; ...
```

Offered as the shortest correct statement, not as prescribed wording. I did not measure it beyond confirming the two facts it asserts (`:1705` loses a real claim, 5/5 above; the loop's candidate name is `{pid}-{nanos}-{seq}` at `:528`). The triager should treat the diagnosis as the finding and the wording as replaceable.

---

# RG3: nothing pins that the guard takes the reserved directory BEFORE the add, and Invariant B rests on that ordering

**Severity: `low`.**

## Claim

`src/checks.rs:958-960`: "The guard takes ownership of the reserved directory BEFORE the add, so a failing or half-finished `git worktree add` leaves nothing behind: from here every return path below removes the worktree." That sentence is TRUE of the code (`_guard` is constructed at `:961`, the add is at `:965`), and Invariant B's opening clause at `:37-38` ("The temporary worktree is removed when the run ends normally or on an internal error, via a `Drop` cleanup guard") depends on it, since a failed `git worktree add` is an internal error that returns at `:968`. Nothing asserts it. Moving the guard below the add leaves the whole suite green and leaks the reserved directory on every failed add.

## Reproduced

**N3**, `src/checks.rs:961-972`, the `let _guard = WorktreeGuard {...};` block moved below the `if !added.status.success()` early return: **GREEN, 386 passed, 0 failed**; clippy 0 warnings, 0 errors.

A/B of the built binary. The add is made to fail after a successful reservation by writing a regular file at `<repo>/.git/worktrees`, so git cannot create its admin directory:

```
committed 3f49012:
error: could not set up the isolation worktree: `git worktree add` failed: Preparing worktree (detached HEAD fe91471)
fatal: could not create leading directories of '.git/worktrees/agent-scaffold-checks-run-1834338-1785451804761841074-0': Not a directory
exit=2
leftover runner dirs under the run's TMPDIR: 0

N3:
error: could not set up the isolation worktree: `git worktree add` failed: Preparing worktree (detached HEAD 50350ce)
fatal: could not create leading directories of '.git/worktrees/agent-scaffold-checks-run-1833649-1785451766541148170-0': Not a directory
exit=2
leftover runner dirs under the run's TMPDIR: 1
/tmp/.../pg/rt/agent-scaffold-checks-run-1833649-1785451766541148170-0
```

## Severity reasoning, argued down rather than up

`low`, not `medium`, and the argument against a higher rating is on the record already. What the leak produces is an EMPTY, unregistered directory under the temp dir that no later run reclaims, which is character-for-character the residual Invariant B `:45-48` already documents and accepts for the SIGKILL window and hands to the operating system's temp-dir cleanup. The trigger differs (a failing add rather than a hard kill) and the run is already exiting 2 either way. Nothing shipped is wrong today; what stays unfixed is that a reordering nobody would notice turns a stated invariant into a leak.

## Fix, built and measured, and its cost is the reason I am not pressing it

The assertion has to be "no runner directory survives under the temp dir", and under `cargo test` every unit test is a thread of one process while `concurrent_reservations_never_share_a_runner_worktree_path` is creating and removing 2000 runner directories in that same temp dir, so the scan is racy from a unit test. This crate is a pure binary with no lib target, so an integration test cannot call `run()` either. That leaves a spawned-binary test with its own `TMPDIR`, the same shape as `tests/checks_missing_tmpdir.rs`.

I built it: `tests/checks_failed_worktree_add_leaks_nothing.rs`, **78 lines**, one new test binary.

| State | Result |
| --- | --- |
| `3f49012` + the test | GREEN, `a_failed_worktree_add_leaves_no_reserved_directory_behind ... ok`; full suite 387 passed, 0 failed |
| plus N3 | **RED**, `a failed add must leave no reserved directory: ["agent-scaffold-checks-run-1842609-1785452059712455724-0"]` |

78 lines and a new test binary to pin an ordering whose failure leaks an empty directory is close to the cost that got `AD1b` accepted as a residual (roughly 40 lines and a second spawned-binary fixture, declined). The measured number is above that, not below it, and I am reporting it that way rather than advertising a cheap fix: on this step's record, four prescriptions in a row were corrected when a triager measured them, and three of the four were corrected for cost. My recommendation is that this is a legitimate ACCEPT-AS-RESIDUAL on the same reasoning as `AD1b`, recorded with the A/B above so round 5 does not re-derive it. The finding is that it is currently unrecorded, not that it must be fixed.

---

# RG4: Invariant B's stated remedy would not close the window it names

**Severity: `low`.**

## Claim

`src/checks.rs:49-56`, the last two sentences of Invariant B:

> Registration is NECESSARY BUT NOT SUFFICIENT: the prune additionally requires the worktree path GIT RECORDED to sit under the CURRENT process's `std::env::temp_dir()`, and git records that path symlink-resolved, so a registered orphan recorded outside this process's temp dir (**a `TMPDIR` reached through a symlink, or a run killed under a different `TMPDIR`**) is never reclaimed either. **Widening the prune to sweep the temp dir by prefix would close that window**, at the cost of giving it authority over other repositories' runner directories (Principle 18, least authority), which is not a trade this module makes.

The first sentence names two causes. The second says a prefix sweep of the temp dir would close "that window". It would close the first cause and cannot close the second. An orphan left under `TMPDIR=/A` is not under `/B`; sweeping `/B` by prefix, however wide the prefix, never reaches `/A`. The sub-case is the module's own, named two lines earlier, and the sentence promises a remedy that does not reach it.

For completeness on the half that does work: `std::env::temp_dir()` returns `TMPDIR` unresolved, and `read_dir` on a symlinked directory follows the link, so a prefix sweep does see an orphan whose git-recorded path was symlink-resolved out from under the gate. That is why this is an over-claim on one sub-case rather than a wholly false sentence.

## Evidence

`file:line` citation and the argument above; the claim is about a rejected alternative implementation, so there is no code to mutate. No probe can add anything to "`/A` is not under `/B`".

## Not a reopening

The settled list routes the BEHAVIOUR (canonicalising `std::env::temp_dir()`, the prune skipping an orphan recorded under a symlink-resolved path, relative-`TMPDIR` validation) to a future roadmap step. This finding changes no behaviour, asks for no code, and takes no position on whether the prune should be widened. It is that the sentence describing the declined alternative overstates what the alternative would buy, which is a prose defect of the same species as round 3's `AD3` and round 2's `X8b`. The narrowest fix is to scope the promise: "would close the symlink half of that window". Unmeasured beyond being a doc edit.

---

# What I checked in the region and found HOLDING

Stated positively, because a zero from a region lens was wrong last round and "I found nothing" is not the same as "I looked".

- **`claim_dir`'s `create_dir` is genuinely load-bearing, and its comment's claim about itself is true.** `:442-447` says a change to `create_dir_all` would report every claim as won, and that asserting `is_dir()` from the test instead would stay green under it. **N6** (`fs::create_dir` -> `fs::create_dir_all`, `:449`): **RED**, `a_directory_claim_is_exclusive` FAILED on "a second claim on the same path is lost", 372 passed, 1 failed. The comment is accurate and the test it defends does the work it claims.
- **"no other test executes the LOST one through `claim_dir` itself" (`:1691-1693`) is true, measured.** The instrumentation under `RG2` found exactly one lost real claim per whole-suite run, 5/5, and it is this test's. The three reservation collision tests do inject rather than call `claim_dir`, as the comment says.
- **The reservation genuinely CREATES the directory, and that is pinned, incidentally but really.** **N7** (`reserve_runner_worktree` passes `|_| Ok(true)`, so the path is named but never created): **RED**, `a_reserved_path_still_carries_its_owning_pid_as_the_first_component` FAILED at `:1871`, which is its `fs::remove_dir(&dead).unwrap()`. That cleanup line, added in round 2 for litter reasons, is what pins creation; `concurrent_reservations_...` uses `let _ = fs::remove_dir(path)` at `:1846` and would not have caught it.
- **The exhaustion test's bound assertion, round 3's `AD2` fix, is sound as landed.** `message.contains(&format!("after {RUNNER_RESERVE_ATTEMPTS} attempts"))` at `:1759` searches for the words "after 16 attempts", which the payload path cannot supply; it is not satisfiable by the pid, the nanosecond reading or the `TMPDIR` string the way the bare `"16"` was. Read against the production format string at `:549-553`, both fragments exist there. No residual weakness found in that assertion.
- **`a_claim_that_never_wins_fails_at_the_attempt_bound`'s other assertions hold their weight.** `error.kind() == AlreadyExists` (`:1748-1751`), `offered.len() == RUNNER_RESERVE_ATTEMPTS` (`:1752-1756`, non-tautological per round 3's re-derivation of the `0 .. 1u32` mutation) and `message.contains(&last)` (`:1763`) each pin a distinct property. I found nothing three lines from anything.
- **`a_lost_claim_retries_...` (`:1709-1731`) pins name freshness, not just the return value.** `!offered[..2].contains(&reserved)` at `:1727` goes RED under any change that reuses one candidate name across attempts (drawing `nanos()` and `seq` once above the loop), because all three offered paths would then be equal.
- **`tests/checks_missing_tmpdir.rs` is sound in both directions.** **N4** (`fs::create_dir_all(&temp).map_err(...)?` deleted at `:519-524`): the integration test goes **RED**, `a TMPDIR naming a directory that does not exist yet is legal and must still run`. The binary probe under N4 produces `error: could not reserve the runner worktree directory <TMPDIR>/pm/missing/nested/agent-scaffold-checks-run-1846893-1785452173444428021-0: No such file or directory (os error 2)`, `exit=2`, which matches the message its doc comment quotes at `:6-7` word for word and errno for errno. Its two assertions are also complementary rather than redundant: `Some(0)` alone could be satisfied by a run that returned early without touching the temp dir, and `missing.is_dir()` at `:83` closes that. `RunError::Io` maps to exit 2 at `:290-298`, so the "(exit 2)" in its doc is right (the variant is `Io`, not `WorktreeSetup`; "erroring its worktree setup" reads as the phase, and I am not raising it).
- **`RUNNER_PREFIX`'s doc claim that the name is built in exactly one place is true.** `grep -rn 'agent-scaffold-checks-run' --include=*.rs .` gives only `src/checks.rs`, and within it the only construction is the `format!` at `:528`; `:564` consumes it via `strip_prefix`. The doc attributes it to `reserve_runner_worktree` where the `format!` is one level down in `reserve_runner_worktree_with`, which is delegation, not an error. The distinctness claim against `agent-scaffold-checks-test-` also holds; the file now has other fixture prefixes (`agent-scaffold-missingtmp-` in the integration test) but none begins with `RUNNER_PREFIX`, so `owning_pid` cannot mistake any of them and the safety claim is unaffected.
- **`owning_pid`'s first-segment contract is pinned.** `a_reserved_path_still_carries_its_owning_pid_as_the_first_component` fails under a name whose pid is not first, which is the damaging way to get this wrong (the prune could reclaim a live run's worktree).
- **Invariant B's other sentences are true as written.** The `Drop` guard at `:352-364`, the registration bound, the empty-unregistered-directory window between reservation and add, and `WorktreeGuard`'s restatement at `:340-344` all match the code. `run()`'s doc at `:892-895` restates the same bound consistently. Only the final remedy sentence over-claims (`RG4`).
- **`run()`'s ordering claims are true.** The prune runs before the reservation (`:946` before `:957`), so the current run's own worktree is not in the list it walks, as `:583-584` says; and every return path below `:961` drops the guard, as `:959-960` says.

---

# NON-FINDINGS, recorded so round 5 does not re-derive them

## The clock numbers: I could NOT reproduce the step brief's rates, and I am explicitly not raising it

`reserve_runner_worktree:473-476` says `SystemTime::now()` "advances in steps of tens of nanoseconds, so two threads sampling it at the same moment routinely read the same value", and `nanos():1019` says it "carries roughly 25 ns of resolution". The step brief records the original measurement: median 30 ns, minimum 20 ns over 100000 samples; two threads at a shared `Barrier` equal for 8679 of 100000 (8.7%); 16 threads 568127 of 800000 (71%).

Measured here with a standalone `rustc -O` probe calling the same expression, matching the brief's method as closely as I could read it:

```
A. one-thread consecutive-distinct step, n=100000: min=50 p50=60 p90=100
   readings that REPEATED back to back: 0
B. two threads at a shared Barrier, n=100000: equal readings = 0 (0.00%)
C. 16 threads at a shared Barrier, 800000 samples: duplicate readings = 4102 (0.5%)
```

And in the configuration the module actually cares about, **N5** (`{seq}` dropped from the name at `:528`, `claim_dir`'s lost arm instrumented), the 2000-reservation concurrency test: **0 lost claims, 5 runs of 5, 10000 reservations total.**

I am raising none of this as a finding, for three reasons and I want them on the record rather than the numbers alone. First, the step brief's numbers were independently reproduced twice already (during plan review and by a round-1 reviewer), so three prior measurements disagree with mine and the most likely explanation is my method, not the machine: with two threads, `Barrier::wait` releases the last arriver immediately and wakes the other through a futex, which staggers them by microseconds against a 60 ns clock step, and that alone would drive B to 0. My one-thread step of 50 to 60 ns is bounded below by the call's own latency, so it cannot distinguish "the clock's resolution is 50 ns" from "the resolution is 1 ns and the call costs 50 ns", which means it does not refute "roughly 25 ns" either. Second, the conclusion the prose draws from the number is right whatever the number is: `nanos()` gives no uniqueness guarantee and correctness rides on layer 2, which both doc sites say plainly. Third, `nanos()` being retained is on the settled list. Reporting a contradicting measurement as a finding on this evidence would be the exact error this task has recorded four times, a diagnosis whose measurement does not survive being built by someone else, and I would rather leave the discrepancy visible than convert it into a finding it will not support.

## `{seq}` dropped from the name is the settled `X1` residual, not a new one

N5 above is functionally the `fetch_add` -> `load` mutation in a different spelling (every name collapses to `{pid}-{nanos}`), and it stays green for the documented reason: layer 1 is a name optimisation and correctness rides on layer 2. Not raised.

## `RUNNER_RESERVE_ATTEMPTS` and the `AD1b` residual

Untouched. I did not re-run 16 -> 3, did not re-open the symbolic-constant ruling, and did not re-derive `AD1b`'s `M17`.

## Test-fixture litter on a RED run is module-wide, not regional

Three RED runs of `a_directory_claim_is_exclusive` each left their `agent-scaffold-checks-test-{pid}-claim-dir` scratch directory behind, because its `fs::remove_dir_all(&dir)` sits after its assertions, while `concurrent_reservations_...` (`:1843-1847`) and `a_reserved_path_still_carries_...` (`:1869-1872`) deliberately clean up BEFORE asserting and say why in comments. I checked whether this is a regional inconsistency and it is not: classifying the 65 pre-existing `/tmp/agent-scaffold-checks-test-*` directories by their test-name suffix gives 18 distinct fixtures (`prune-liveness` 9, `prune-orphan` 6, `stdin-null` 5, `fail` 5, `claim-dir` 5, `paths-false-negative` 4, and 12 more), so cleanup-after-assert is the module's prevailing convention across the whole test suite and the two pre-cleaning tests are the deviation, taken because they create directories in the shared temp-dir ROOT rather than in a scratch subdirectory. Out of this region's scope and not raised.

---

# VERDICT

**NOT zero. Four findings from this lens: `RG1` `medium`, `RG2` `low`, `RG3` `low`, `RG4` `low`.**

One is a correctness-coverage gap with two green mutations and a 13-line measured fix (`RG1`). Two are false or over-claiming sentences in the region's prose (`RG2`, `RG4`). One is an unpinned ordering that Invariant B depends on, reported with a measured fix cost that I recommend accepting as a residual rather than paying (`RG3`).

---

# Worktree and temp-directory state

**Every mutation reverted.** Measured after the last revert and before this file was written:

```
$ git rev-parse HEAD
3f490128ef34c608dac134a313bdb69972e0daf0

$ git status --short
(empty)

$ git diff HEAD
(empty)
```

This findings file is written after those commands and is left uncommitted, as instructed. The candidate fix for `RG1` and the candidate test for `RG3` were both reverted out of the tree (`git checkout -- src/checks.rs`, `rm tests/checks_failed_worktree_add_leaks_nothing.rs`) and kept only in the session scratchpad.

Mutations applied and reverted, in order: N1, N2 (`claim_dir`'s error arm swallowed as lost / as won), the `RG1` candidate fix alone and with N1 and N2, N3 (guard after the add) alone and with the `RG3` candidate test, N4 (temp-dir creation deleted), the `claim_dir` lost-arm `eprintln!` instrumentation, N5 (`{seq}` dropped), N6 (`create_dir_all`), N7 (claim always wins, never creates). All reverted with `git checkout -- src/checks.rs`.

**Temp-directory hygiene.**

- **Directories created in `/tmp`: 0.** `ls -d /tmp/agent-scaffold-* | wc -l` returned **65** at the start of this review, and a `diff` of that listing against the same listing taken after my last mutation was reverted is IDENTICAL, so not one of the 65 is mine. All 65 are `agent-scaffold-checks-test-*` predating this session; none was created, touched or deleted. `find /tmp -maxdepth 1 -name 'agent-scaffold-checks-run-*' | wc -l` returns **0**.
- **One entry appeared during the review that is NOT mine, and I left it in place.** `/tmp/agent-scaffold-hookenv-1858741-staged`, mtime `2026-07-31 00:03:39`, taking the count to 66. It is the fixture of `tests/checks_staged_hook_env.rs:50`, and it is incomplete (it holds `.agents/` and `file.txt` but no `.git`), so its run did not finish. Attribution settled by experiment rather than asserted: `direnv export` does NOT set or override `TMPDIR` here (checked directly, before and after are the same string), and running that exact test binary under my exported `TMPDIR` leaves the `/tmp` count unchanged at 66 and leaves 0 entries in my scratch `TMPDIR`, so my runs demonstrably do not reach `/tmp`. The sibling review worktree `rev93-r4-b` exists alongside mine and its timestamps line up. I left the directory alone rather than deleting another session's working state, on round 2's precedent for the 32272-directory case.
- `TMPDIR` was exported to `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/r4a-tmp` for every build, test run, binary A/B and probe.
- Probes were bounded: 5 instrumented suite runs for the lost-claim count, 5 concurrency-test runs under N5, 100000-sample clock probes (one process, self-limiting), and single runs for each binary A/B. No exhaustion-path or high-volume reservation probe was run, and nothing scaled the 2000-reservation concurrency test up.
- Fixtures created and removed inside the scratch `TMPDIR`: three scratch git repos (an unwritable-`TMPDIR` probe, a failed-add probe, a missing-`TMPDIR` probe), one `chmod 555` directory (restored to 755 before deletion), two standalone clock probes, one saved patch, one saved source copy, one saved candidate test. Leftovers from the deliberately RED runs (three `agent-scaffold-checks-test-*-claim-dir`, one `agent-scaffold-addfail-*`, one `agent-scaffold-missingtmp-*`) were removed at the end; the scratch `TMPDIR` is empty at the time of writing.
