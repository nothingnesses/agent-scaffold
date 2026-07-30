# `checks-runner-worktree-name-collision`: reviewer findings (evidence lens)

Reviewed commit `b890c4a` (`fix(checks): reserve the runner worktree path instead of trusting the clock`), a single-file change to `src/checks.rs` (+212 / -26). Reviewed in an isolated worktree at detached HEAD `b890c4a`. Machine: 16 cores, git 2.54.0, cargo 1.98.0-nightly.

Lens: is the demonstration as good as it claims? Every number below was re-measured here rather than taken from the commit message. All code mutations used to produce RED results were reverted; `git status` is clean and `git diff HEAD` is empty at the time of writing, with `cargo test` green (369 + 5 + 1 + 3 + 1 + 2 passed, 0 failed) and `cargo clippy --all-targets` reporting zero warnings.

## Summary

**7 findings: 0 critical, 0 high, 0 medium, 7 low.** Explicitly: no critical, no high and no medium findings were found, and none are invented to fill the scale.

Every load-bearing claim in the commit message reproduces. The demonstration meets the brief's standard: it is a real red/green mutation demonstration on the property, not a green-suite count, and the three RED results reproduce (one of them harder here than reported). The findings are all about what the tests DO NOT pin, not about anything the implementer got wrong.

## Claim-by-claim verification

### Claim 1: the property test exercises the right unit. VERIFIED.

`concurrent_reservations_never_share_a_runner_worktree_path` calls `reserve_runner_worktree(std::process::id())` at `src/checks.rs:1639`. That is the same call that yields the final path in production (`src/checks.rs:878`) and in all three prune fixtures (`src/checks.rs:1561`, `:1591`, `:1592`). It asserts on the returned `PathBuf`s, not on the raw name string (`src/checks.rs:1663-1670`), which is exactly what the brief demanded: a string-level assertion would be RED against a correct `create_dir` reservation. My own measurement confirms the brief's reasoning about why: at 8 threads by 250 with the sequence pinned to a constant and no `mkdir` in the loop, the raw names duplicated 82, 82, 106, 95 and 113 times per 2000, so a string-level assertion would indeed fail against correct code.

### Claim 2 (RED-1): pre-fix expression fails 12 of 12. REPRODUCED, with machine variance in the magnitude.

Method: spliced the exact pre-fix expression back into `reserve_runner_worktree` as an early return (`Ok(std::env::temp_dir().join(format!("{RUNNER_PREFIX}{pid}-{}", nanos())))`), leaving the test untouched, then ran the test 12 times.

My numbers, 12 of 12 FAILED:

```
run 1: 24    run 5: 40    run  9: 55
run 2: 17    run 6: 72    run 10: 34
run 3: 15    run 7: 42    run 11: 19
run 4: 23    run 8: 37    run 12: 49
```

duplicates per 2000. Reported: 12 of 12, 25 to 77. Mine: 12 of 12, 15 to 72. The pass/fail conclusion reproduces exactly; the range is looser at the low end, which is ordinary machine variance and is not a defect. Reverted.

### Claim 3 (RED-2): all disambiguators pinned to a constant fails deterministically via retry exhaustion. REPRODUCED EXACTLY.

Method: pinned `seq` to `0_u64` and the clock component to the literal `12345_u128`, keeping the `create_dir` reservation.

3 of 3 runs FAILED, and the failure is fully deterministic: each run produced byte-identical output (269119 bytes) containing 1999 instances of `could not reserve a unique runner worktree directory after 16 attempts`. One reservation wins, the other 1999 exhaust `RUNNER_RESERVE_ATTEMPTS`. Reverted.

### Claim 4 (RED-3): prepending the disambiguator breaks `owning_pid` and lets the prune reclaim a LIVE worktree, while one prune test still passes. REPRODUCED, and STRONGER than reported.

Method: changed the name to `{RUNNER_PREFIX}{seq}-{pid}-{nanos}`, nothing else.

Full `checks::` module, 3 of 3 runs, identical result: 27 passed, 2 failed.

- FAIL `checks::tests::a_reserved_path_still_carries_its_owning_pid_as_the_first_component`.
- FAIL `checks::tests::a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one`, panicking at `src/checks.rs:1600` with `a live owner's worktree must not be reclaimed`. That is the destructive failure mode, caught by name.
- PASS `checks::tests::a_startup_prune_reclaims_an_orphaned_runner_worktree`.

So the reported shape is exact. One addition the implementer did not report: when the two prune tests are run in isolation (`cargo test --bin agent-scaffold checks::tests::a_startup_prune`), the orphan-reclaim test ALSO fails, at `src/checks.rs:1570` with `the registered orphan worktree was reclaimed`, because with the sequence prepended the parsed "owner" is a small integer that happens to be a live pid on this system. The "one prune test still passed" detail is therefore a function of which sequence values the run happens to draw, not a property of the mutation. This makes the demonstration stronger than claimed, not weaker. Reverted.

### Claim 5: exactly one `format!` builds a `RUNNER_PREFIX` name. VERIFIED.

```
grep -rn "RUNNER_PREFIX\|agent-scaffold-checks-run" --include=*.rs .
```

returns 10 hits in `src/checks.rs` and nothing elsewhere in `src/` or `tests/`. Exactly one is a name-building `format!`: `src/checks.rs:461`, inside `reserve_runner_worktree`. The other nine are the constant itself (`:83`), `owning_pid`'s `strip_prefix` (`:488`), and doc or code comments (`:77`, `:421`, `:425`, `:482`, `:497`, `:876`, `:1619`).

Reconciled against the four sites the brief's scope section enumerates: `run()` (was `:791-792`) now calls the generator at `src/checks.rs:878`; the three fixtures (was `:1462`, `:1491`, `:1492`) now call it at `src/checks.rs:1561`, `:1591` and `:1592`. No inline construction survives beside the shared generator.

### Claim 6: cross-process measurement. REPRODUCED; the conclusion it is used for is supported.

Method: an independent 8-process probe, released on a shared wall-clock instant, at the constant-pid template (`agent-scaffold-checks-run-4294967295-{nanos}-{seq}`), 2000 draws per process, 16000 per run. Written from scratch rather than reusing the implementer's.

Candidate (a) alone (atomic sequence in the name, no reservation), 8 of 8 runs produced duplicates: 73, 67, 58, 53, 45, 60, 67, 68 per 16000.

Candidate (a) + (d) (the shipped shape), 8 of 8 runs: 0 duplicates per 16000, 0 exhaustions, 0 leftover directories, with `create_dir` rejecting a name 2 times across the 128000 reservations.

Reported: (a)-alone non-zero in every run; (a)+(d) 0 per 16000 with `create_dir` firing twice in 80000. Both reproduce. The conclusion the commit draws from this, that the cross-process channel at the constant-pid template is real rather than theoretical and that layer 2 is what closes it and demonstrably fires, is supported. The commit states the probe setup explicitly ("an 8-process probe at that constant-pid template") and does not present the probe rate as a field rate, which would have been wrong: the probe hammers 2000 reservations per process under a synchronised release, whereas the real fixtures make one dead-pid reservation each per `cargo test` process. See finding 6 for the one number I could not reconcile.

### Claim 7: `create_dir` fires 0 retries in 2000 in-process even with the sequence pinned. REPRODUCED, and it does weaken the "both layers earn their place" framing.

Method: an in-process probe, 8 threads on a `Barrier`, 250 reservations each, sequence pinned to a constant so the clock is the only name-level discriminator.

- With the `create_dir` reservation: 5 of 5 runs gave 0 duplicates and 0 retries in 2000.
- Same pinning, `create_dir` removed (control): 86, 82, 106, 95, 113 duplicates per 2000.
- Extended to 64 threads by 250 (16000 per run) with the reservation: 0, 0, 1, 0, 0 retries, i.e. 1 retry in 80000 reservations.

So the reported number is right and the mechanism the implementer gives for it is right: the `mkdir` syscall spreads the clock reads apart, and the control run at the same pinning proves the clock itself has not improved. See finding 2 for what this implies about layer 1.

### Claim 8: numbers that do not reproduce.

All behavioural numbers reproduce, within machine variance where variance is expected. One documentary inconsistency is recorded as finding 6.

## Findings

### Finding 1: neither uniqueness layer is individually pinned by any test. Severity: low.

The commit describes the fix as "two independent layers" (`src/checks.rs:432-455`). Neither layer is pinned on its own. I removed each in turn.

**Mutation M1, layer 1 removed** (delete the atomic sequence, keep the reservation): name becomes `{RUNNER_PREFIX}{pid}-{nanos}` while `fs::create_dir` still reserves. The whole `checks::` module is GREEN, 5 of 5 runs, 29 passed / 0 failed each time. No test notices.

**Mutation M2, layer 2 removed** (delete the `create_dir` reservation, keep the sequence in the name, i.e. exactly candidate (a) alone which the commit itself argues is insufficient): 28 passed, 1 failed. The failing test is NOT the property test. `concurrent_reservations_never_share_a_runner_worktree_path` PASSES under M2, because the atomic sequence alone makes in-process names distinct. The only test that fails is `a_reserved_path_still_carries_its_owning_pid_as_the_first_component`, and it fails incidentally on its cleanup call:

```
panicked at src/checks.rs:1690:31:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

That message says nothing about a reservation having been removed. A future author reading it would most likely "fix" the test by relaxing the cleanup to `let _ = fs::remove_dir(...)`, at which point the reservation is unpinned entirely.

What the property test actually pins is "at least one disambiguator survives", not "both layers survive". Both RED results the commit reports are consistent with that weaker property: RED-1 removes both layers, RED-2 pins both to constants.

Severity is low rather than medium because neither single-layer removal reintroduces the observed in-process defect: M1 keeps the reservation's hard guarantee, M2 keeps the sequence's in-process guarantee, and M2's residual exposure is only the cross-process channel at the two constant-pid fixtures, which the brief itself estimates at order 1e-6 per pair in the field. This is defence-in-depth erosion, not a live defect. Both mutations reverted.

### Finding 2: the "both layers earn their place" argument is not backed by the measurements, and one doc sentence overstates layer 1. Severity: low.

The implementer honestly reported the counter-intuitive number (0 `create_dir` retries in 2000 with the sequence pinned), and it reproduces. I extended it: 1 retry in 80000 at 64 threads by 250. So once layer 2 is present, layer 1 buys nothing measurable in-process on this machine. Combined with finding 1's M1 result (suite green with layer 1 deleted), the honest position is that layer 2 supplies the entire correctness guarantee and layer 1 is a cheap optimisation plus defence in depth.

The doc comment at `src/checks.rs:439-440` says of the sequence: "`{seq}` is what actually separates them: a process-wide atomic counter is unique by construction across all threads and all calls." Read as a statement about the NAME that is true; read as a statement about what separates two reservations in the shipped function it overstates layer 1, because the reservation in the same function separates them too, measured at 0 duplicates and 0 retries with the sequence pinned. The rest of the comment is accurate and does correctly attribute the guarantee to layer 2 ("that outcome (not an entropy argument) is what makes the returned path exclusively ours", `src/checks.rs:447-448`).

This is NOT a re-argument of the settled (a)+(d) choice: layer 1 is one atomic increment, it keeps the retry path cold so the test stays deterministic on a slower filesystem, and it preserves the guarantee if layer 2 is ever removed. Those are good reasons to keep it. The finding is that the code says layer 1 is doing something the measurements say layer 2 is doing.

Suggested fix: soften `src/checks.rs:439-440` to say the sequence is what makes the NAME unique in-process, so that the reservation almost never has to retry, rather than that it is what separates two reservations.

### Finding 3: a bypass of the shared generator at the production call site leaves the suite green. Severity: low.

**Mutation M3**: replaced `reserve_runner_worktree(std::process::id())?` at `src/checks.rs:878` with the pre-fix inline `std::env::temp_dir().join(format!("{RUNNER_PREFIX}{}-{}", std::process::id(), nanos()))`, leaving `reserve_runner_worktree` and both new tests fully intact. This is precisely the shape the brief warns about: "A test that pins a new generator while the inline `format!` at `src/checks.rs:791-792` ... survives beside it is green with the defect fully present."

Result: `checks::` module GREEN, 6 of 6 runs, 29 passed / 0 failed each time. The defect is fully present in `run()` and no test notices.

Mitigating factor, which I looked for and found: under M3, `cargo clippy --all-targets` reports three new warnings, `static NEXT_RUNNER_SEQ is never used`, `constant RUNNER_RESERVE_ATTEMPTS is never used`, and `function reserve_runner_worktree is never used`, because `run()` is the generator's only non-`cfg(test)` caller. That is a real standing signal against this specific regression. Its limits: it is a warning and not an error (`just clippy` is `cargo clippy --all-targets` with no `-D warnings`, `justfile:29-30`), and it vanishes the moment any second non-test caller exists.

The brief settled that a grep is proportional evidence for the linkage and that no extra test is owed, so this is reported as a characterisation of the standing guard rather than a demand for a new test. The grep in claim 5 is correct today; what does not exist is anything that keeps it correct tomorrow, beyond the clippy warning above. Reverted.

### Finding 4: a retry-exhaustion regression prints up to 2000 full error strings. Severity: low.

`src/checks.rs:1662`:

```rust
assert!(failures.is_empty(), "reservations failed: {failures:?}");
```

Under RED-2 each failing run emitted 269119 bytes to stdout, containing 1999 copies of the same message. It was large enough that piping it through `head`/`cut` in my shell failed with `Argument list too long`. A failure count plus the first message would diagnose the same regression. Cosmetic; no correctness impact.

### Finding 5: both new tests leak reserved directories on failure. Severity: low.

`a_reserved_path_still_carries_its_owning_pid_as_the_first_component` cleans up at `src/checks.rs:1690-1691`, after its assertion loop, so an assertion failure leaks the two reserved directories. `concurrent_reservations_never_share_a_runner_worktree_path` cleans up before asserting (`src/checks.rs:1656-1660`), which is the right order, but a panic inside a reserving thread propagates through `taker.join().expect(...)` at `src/checks.rs:1648` before the cleanup loop, leaking up to 2000.

Observed, not theoretical: my RED-3 runs left 16 directories under `/tmp` matching `agent-scaffold-checks-run-*`, six of them containing a `.git` file. I removed all of them; `find /tmp -maxdepth 1 -name "agent-scaffold-checks-run-*" | wc -l` now returns 0. On a green run both tests clean up correctly, and the leaked directories are unregistered so the repo-scoped prune cannot mistake them for anything. Purely hygiene.

### Finding 6: the (a)-alone probe numbers differ between the commit message and the report given to the review. Severity: low.

The commit message records "5, 9, 16, 56 and 157 duplicate paths per 16000 across five runs". The review request describes the same measurement as "duplicates in 8 of 8 runs (4 to 157 per 16000)". Five runs versus eight, and a low bound of 5 versus 4. Both are consistent with the underlying behaviour and with my own re-measurement (8 of 8, 45 to 73), so nothing here is wrong about the world, but only the commit message is durable and the two accounts of one measurement do not match. Worth a one-line reconciliation from the implementer so the record has a single number. I cannot resolve it from the repository.

### Finding 7: the property test's guarantee is machine-dependent in one direction the comment does not mention. Severity: low.

`concurrent_reservations_never_share_a_runner_worktree_path` can only fail in two ways: duplicate paths (impossible while the atomic sequence exists) or a reservation error (only reachable through 16 consecutive `create_dir` collisions). On this machine the retry path is essentially never taken, 1 retry in 80000 even with the sequence pinned. That is fine and is the intended design, but it means the test is a guard against removal of BOTH disambiguators and nothing weaker (finding 1), and it means the test's runtime cost is 2000 real `mkdir` plus 2000 `rmdir` on every `cargo test`. Measured cost here: the whole `checks::` module still finishes in 0.08 to 0.12 s, so the cost is not a problem, but on a slow or network-backed `TMPDIR` this test becomes the module's dominant cost and, if `create_dir` there is slower than the clock granularity assumption, the retry path could start firing. Nothing to change now; recording it so a future `TMPDIR`-related flake is diagnosed quickly rather than rediscovered.

## Checked and found clean, so raised as nothing

- **The four comments the brief required corrected** are all corrected: `RUNNER_PREFIX` (`src/checks.rs:76-82`), `owning_pid` (`:481-486`), the naming site in `run()` (`:873-877`), and `nanos` (`:933-945`). The `nanos` comment now explicitly retracts the false premise.
- **Doc currency outside `src/`**: `grep -rn "agent-scaffold-checks-run" --include="*.md" .` returns hits only in `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md` and `docs/plans/agent-scaffold.md`, and every one is inside this plan's own record of the pre-fix defect, where the old format is the correct historical text. Nothing stale. `README.md`, `CHANGELOG.md`, `AGENTS.md` and `pack/` carry no occurrence.
- **The CHANGELOG argument holds.** `git show --stat 3f1e247` (`fix: gate checks orphan prune on owning-process liveness`) touched only `src/checks.rs`; `7ea018c` (`fix: harden checks isolation`) touched only `src/checks.rs` and `src/main.rs`. Neither added a CHANGELOG entry. The `checks` module is described under `[Unreleased] -> Added` in `CHANGELOG.md` and has never shipped (`[0.0.1] - 2026-07-10` does not mention it). `AGENTS.md` contains no CHANGELOG rule. The reasoning is right.
- **`git worktree add` into a pre-reserved empty directory**, re-verified here against git 2.54.0 rather than the brief's 2.51.2: `git worktree add --detach <pre-created empty dir> HEAD` succeeds; the same on a directory containing one file fails with `fatal: '<path>' already exists`. The reservation is compatible with the add.
- **Moving `WorktreeGuard` before the add** (`src/checks.rs:879-885`) is a strict improvement, not a regression. On a failing add the guard's `Drop` (`src/checks.rs:334-347`) runs `git worktree remove --force` on an unregistered path (fails, ignored), then `remove_dir_all` on the reserved directory, then `worktree prune`. Previously a partially-succeeding add returned `WorktreeSetup` with no guard in scope and leaked. It is also now safe in a way it was not: `remove_dir_all` can no longer delete a concurrently-created directory belonging to another run, because the reservation makes the path exclusively this call's.
- **Error classification**: `reserve_runner_worktree` returns `io::Result`, `?` converts through `impl From<io::Error> for RunError` (`src/checks.rs:314-318`) to `RunError::Io`, which maps to exit code 2 (`src/checks.rs:275-284`), the environment class. An unwritable `TMPDIR` is correctly not reported as a config error. The non-`AlreadyExists` branch propagates immediately (`src/checks.rs:468`) rather than burning 16 attempts, which is right.
- **The "channels left open" paragraph in the commit message is accurate.** `prune_orphan_worktrees` (`src/checks.rs:514-545`) walks only `git worktree list --porcelain` for this repo, so an unregistered reserved directory left by a SIGKILL between the `create_dir` and the add is genuinely unreclaimable by it, exactly as stated, and widening it would indeed give it authority over other repositories' directories.
- **The prune ordering is still correct**: `prune_orphan_worktrees(&repo)` runs at `src/checks.rs:867`, before the reservation at `:878`, so a run can never prune its own directory, and a concurrent run's registered worktree is protected by the liveness gate as before.
- **No `nix fmt` was run**, per this repo's known state of not being formatter-clean at HEAD.

## Verdict

The demonstration meets the brief's standard. It tests the path-yielding call rather than the name generator, it is RED before green in three independent ways, all three RED results reproduce here (RED-3 more strongly than reported), the linkage claim is exactly true, and the cross-process measurement supports the narrow conclusion it is used for. The seven low findings are about coverage the tests do not provide and one documentation overstatement, not about anything the implementer claimed and got wrong.
