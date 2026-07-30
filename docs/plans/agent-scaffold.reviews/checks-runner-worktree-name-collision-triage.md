# Triage: `checks-runner-worktree-name-collision` (commit `b890c4a`, round 1)

Adjudicated in an isolated worktree at detached HEAD `b890c4a`. Every mutation below was applied, run, and reverted; `git status` is clean at the time of writing apart from this findings directory. Environment: git 2.54.0, cargo/rustc 1.98.0-nightly, Linux, 16 cores. Baseline at `b890c4a`: `cargo test` green (369 + 5 + 1 + 3 + 1 + 2), `cargo clippy --all-targets` clean.

Inputs: `checks-runner-worktree-name-collision-reviewer-adversarial.md` (5 findings: 3 medium, 2 low) and `checks-runner-worktree-name-collision-reviewer-evidence.md` (7 findings, all low). Twelve raw findings, ten after dedup.

## Line-citation audit

Every `file:line` in both findings files was opened and read at that line. **All citations verify except one**, which is off by one line and does not affect the claim:

- Evidence finding 2 cites `src/checks.rs:439-440` for "`{seq}` is what actually separates them: a process-wide atomic counter is unique by construction across all threads and all calls." The sentence actually spans `:440-441`. The quoted text is verbatim and locatable; recorded for the record, not as a dismissal ground.

Verified correct: adversarial `:37-41`, `:15`, `:309`, `:314-318`, `:324-326`, `:334-347`, `:444-449`, `:453-455`, `:460`, `:462`, `:467`, `:468`, `:471-478`, `:514-545`, `:544`, `:816-817`, `:878-887`, `:1561-1562`, `:1591-1594`, `:1645-1660`, `:1680-1691`, `:1690:31`; evidence `:76-82`, `:83`, `:275-284`, `:421`, `:425`, `:481-488`, `:497`, `:867`, `:873-877`, `:876`, `:933-945`, `:1600`, `:1619`, `:1639`, `:1648`, `:1662`, `:1663-1670`, `justfile:29-30`. The evidence lens's ten-hit `RUNNER_PREFIX` enumeration (`:77`, `:83`, `:421`, `:425`, `:461`, `:482`, `:488`, `:497`, `:876`, `:1619`) is exact.

Per the review brief, the two lenses' differing RED-1 magnitudes (25-77 vs 15-72 duplicates per 2000) are treated as directional agreement with machine variance, not as a defect.

## Deduplication

| Triage id | Merged from | Subject |
| --- | --- | --- |
| T1 | adversarial 3 + evidence 1 | The `create_dir` reservation (layer 2) is pinned by no test |
| T2 | adversarial 2 | `create_dir` breaks a missing `TMPDIR`, and the error loses all context |
| T3 | adversarial 1 | Module Invariant B and two more doc statements are now false |
| T4 | evidence 3 | A bypass at the production call site leaves the suite green |
| T5 | adversarial 4 | The `WorktreeGuard` move is unpinned |
| T6 | adversarial 5 + evidence 5 | Fixtures leak reserved directories on assertion failure |
| T7 | evidence 4 | Retry-exhaustion regression prints ~268 KB |
| T8 | evidence 2 | The comment at `:440-441` overstates layer 1 |
| T9 | evidence 6 | Probe numbers differ between commit message and review request |
| T10 | evidence 7 | The property test's guarantee is machine-dependent |

Two merges. T1 merges the adversarial's mutations B/C/D/E with the evidence lens's M1/M2: they are the same claim (neither layer individually pinned) with complementary mutation sets. T6 merges two statements of the same leak, the adversarial adding the two prune fixtures and the property test's thread-panic path to the evidence lens's account.

The two lenses did NOT collide on T2, T3, T4, T5: each is single-lens. Note that the evidence lens's "checked and found clean" section independently confirms T3's *mechanism* ("genuinely unreclaimable by it, exactly as stated") while declining to raise it, because it read the commit message's disclosure as sufficient. The adversarial's point is that the disclosure is in the commit message and not in the code. That distinction survives.

---

## T1 (adversarial 3 + evidence 1): the `create_dir` reservation is pinned by no test

**Verdict: VALID (fix required). Final severity: medium** (raised from the evidence lens's `low`, confirming the adversarial's `medium`).

All four mutations reproduce exactly:

| Mutation | What it does | Result here |
| --- | --- | --- |
| D | `:462` `fs::create_dir` -> `fs::create_dir_all` | full `cargo test` GREEN (369 + 5 + 1 + 3 + 1 + 2) |
| E | `:467` `AlreadyExists` arm -> `panic!` | full `cargo test` GREEN; the retry arm is never executed anywhere in the repository |
| B | `:460` `fetch_add` -> `load` (seq pinned to 0) | `checks::` GREEN 3 of 3 |
| M1 | seq dropped from the name entirely | `checks::` GREEN 5 of 5 |
| C / M2 | reservation removed (`match Ok::<(), io::Error>(())`) | 28 passed, 1 failed, at `src/checks.rs:1690:31` with `called Result::unwrap() on an Err value: Os { code: 2, kind: NotFound }` |

The mutation-C message is byte-for-byte what the adversarial reported, including the column. It names nothing about reservations, and the evidence lens's reading is right: a future author meeting it would relax the cleanup to `let _ = fs::remove_dir(...)` and unpin the reservation entirely.

Mutation B additionally settles the evidence lens's mechanism claim: with `seq` constant the raw names duplicate at the clock's rate, the reservation's retry path carries the property, and the test still passes. Layer 2 supplies the correctness guarantee.

**Why medium and not low.** The evidence lens rated this low on the ground that no single-layer removal reintroduces the *observed in-process* defect. That is true, and it is why this is not high. But three things push it above low:

1. Mutation D silently removes the only cross-process discriminator at the two `dead_pid()` fixtures, whose first name segment is the constant `u32::MAX` in every process. Both lenses measured candidate (a) alone producing duplicates in every probe run. The brief states that several concurrent `cargo test` processes is this project's normal state, so this is the channel the project actually occupies.
2. The step is classified RISKY precisely because the failure mode runs through destructive cleanup: two callers on one path means `WorktreeGuard::drop` (`:334-347`) `remove_dir_all`s a directory another live run is inside.
3. The fix's own doc comment (`:444-449`) names the reservation as what "makes the returned path exclusively ours", and nothing checks it. This is the AGENTS.md principle "Tests must actually exercise the code they claim to" applied to the load-bearing mechanism of the step, which is the standard the brief set.

**The adversarial's prescribed fix does not work, and I verified that.** It proposes "assert that a reserved path exists and is a directory on return, and that a second `fs::create_dir` on the same path fails with `AlreadyExists`", claiming this "would kill mutations C, D, and E". I wrote exactly that test. Against unmutated code it passes; **under mutation D the whole `checks::` module is still green (30 passed, 0 failed)**. It cannot kill D, because the second `fs::create_dir` in the test exercises `std`, not the production call site, and it never drives the retry arm so it cannot kill E either. It kills only C. Do not implement it as written.

**Minimal fix that does work, verified.** Extract the single claim step so it has a testable surface, and keep `reserve_runner_worktree` calling it (an unused helper would pin nothing, the exact shape the brief warns about):

```rust
/// Claim `path` exclusively: `Ok(true)` when THIS call created it, `Ok(false)` when
/// another holder already had it. `create_dir` (never `create_dir_all`) is what makes
/// the claim exclusive: it is the one that fails atomically on an existing directory.
fn claim_dir(path: &Path) -> io::Result<bool> {
	match fs::create_dir(path) {
		Ok(()) => Ok(true),
		Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(false),
		Err(error) => Err(error),
	}
}
```

with the loop body at `:462-469` becoming `if claim_dir(&path)? { return Ok(path); }` followed by `last_taken = path;`, and one test:

```rust
#[test]
fn claiming_a_directory_another_holder_already_created_reports_it_taken() {
	let dir = scratch("claim");
	let path = dir.join("held");
	assert!(claim_dir(&path).unwrap(), "a fresh path is claimed by this caller");
	assert!(!claim_dir(&path).unwrap(), "a path another holder created is reported taken");
	fs::remove_dir_all(&dir).unwrap();
}
```

Measured here: green against `b890c4a` (30 passed); RED under mutation D at `a path another holder created is reported taken`; RED under mutation E at the probe panic. It also kills C, because the reservation would not compile without the helper. Single-site confirmed: `grep -n "fs::create_dir(" src/checks.rs` returns exactly one production hit, `:462`.

Residual accepted: mutations B and M1 (layer 1 removed) stay green. That is inherent to belt-and-braces, layer 1 is an optimisation that keeps the retry path cold, and pinning it would need a clock or sequence seam that costs more than it buys.

---

## T2 (adversarial 2): `create_dir` turns a missing `TMPDIR` into a hard failure, and the failure reports no context

**Verdict: VALID (fix required). Final severity: medium** (confirming the adversarial's rating).

Both halves reproduce exactly, A/B against `HEAD~1` with only `src/checks.rs` swapped:

| `TMPDIR` condition | pre-fix (`HEAD~1`) | post-fix (`b890c4a`) |
| --- | --- | --- |
| nested path that does not exist | `checks: 1 passed`, exit 0; git created the directories (confirmed by `ls -d`) | `error: No such file or directory (os error 2)`, exit 2 |
| exists, read-only (`chmod 555`) | `error: could not set up the isolation worktree: git worktree add failed: fatal: could not create leading directories of '<full path>': Permission denied`, exit 2 | `error: Permission denied (os error 13)`, exit 2 |

So a shipped CLI subcommand stops working in a case that used to work, and the error path loses the operation, the path, and any hint a temp directory was involved. The doc comment at `:453-455` asserting that reserving "changes nothing downstream" is false as written.

The missing-`TMPDIR` trigger is narrow, which is why this is not high. The message regression is not narrow: it applies to the entire environment-error class for this subcommand (missing temp dir, unwritable temp dir, full filesystem), and the module's own error design (`RunError::WorktreeSetup` naming the operation) is the standard it fell below. Both together are a user-facing regression in shipped code, which is where medium sits.

**Minimal fix, verified.** In `reserve_runner_worktree`, before the loop:

```rust
fs::create_dir_all(&temp).map_err(|error| {
	io::Error::new(error.kind(), format!("could not create the temp directory {}: {error}", temp.display()))
})?;
```

and at `:468` replace the bare `return Err(error)` with the same wrap carrying `path.display()`. The leaf `create_dir` at `:462` must stay `create_dir`; that is where the exclusivity lives (T1). Correct the `:453-455` sentence in the same edit.

Measured with that fix applied: the nested-missing-`TMPDIR` case returns to exit 0, the read-only case reports `error: could not reserve the runner worktree directory <full path>: Permission denied (os error 13)`, and the full suite stays green. `RunError::Io` -> exit 2 is unchanged and correct.

---

## T3 (adversarial 1): module Invariant B, and two further doc statements, are now false

**Verdict: VALID (fix required). Final severity: medium** (confirming the adversarial's rating; it sits at the low/medium boundary and the required fix is one qualifying clause).

The reproduction is exact. With one unregistered dead-owner directory and one registered dead-owner worktree planted under a shared `TMPDIR`:

```
before:  ...-1785000000000000000-0   ...-1785000000000000001-1
after:   ...-1785000000000000000-0
second run: ...-1785000000000000000-0
```

The registered orphan is reclaimed; the unregistered one survives that run and every later one. `prune_orphan_worktrees` (`:514-545`) iterates only `git worktree list --porcelain`, and the trailing `git worktree prune` (`:544`) removes admin entries, never directories.

All three quoted doc statements say what the adversarial claims, verbatim: `:37-41` (Invariant B, "it can orphan a worktree **and its temp directory**; the next run reclaims such orphans with a startup prune"), `:324-326` (`WorktreeGuard`), `:816-817` (`run`). The module header at `:15` does commit to "Scope of the guarantee (stated honestly for a risky increment)", and Invariant B is listed under "Invariants this module pins (see the tests)" while the only test covering it (`a_startup_prune_reclaims_an_orphaned_runner_worktree`) exercises the registered case alone.

The trade itself is sound and I do not reopen it: an empty unregistered directory is a smaller artifact than a registered full checkout, and the window is one `git worktree add` long. The defect is that the commit message discloses the trade and the code does not, which is exactly the record-versus-code drift this project's process exists to catch.

**Minimal fix:** qualify the three statements so they stop over-promising, for example adding to Invariant B that a kill landing between the reservation and the `worktree add` leaves an empty unregistered directory the repo-scoped prune cannot see. Prose only, no code change.

**Do NOT implement the adversarial's second option** (a dead-owner-gated sweep of `temp_dir()` by prefix). It gives the prune authority over other repositories' runner directories, which the commit message already argues against and which cuts against plan Principle 18-equivalent least authority. The residual leak is accepted.

---

## T4 (evidence 3): a bypass at the production call site leaves the suite green

**Verdict: VALID BUT ACCEPT RESIDUAL. Final severity: low** (confirming).

Reproduced precisely. Replacing `:878` with the pre-fix inline `std::env::temp_dir().join(format!("{RUNNER_PREFIX}{}-{}", std::process::id(), nanos()))` and leaving `reserve_runner_worktree` and both new tests intact: `checks::` green 6 of 6, 29 passed each time, with the defect fully present in `run()`. The mitigating signal reproduces too: `cargo clippy --all-targets` then emits exactly the three warnings reported (`static NEXT_RUNNER_SEQ is never used`, `constant RUNNER_RESERVE_ATTEMPTS is never used`, `function reserve_runner_worktree is never used`), and `justfile:29-30` confirms `just clippy` carries no `-D warnings`, so they are warnings and not errors.

No fix required, because the brief already settled this: "A command settles this and no extra test is owed (`Q-66` proportionality): show that exactly one `format!` in `src/checks.rs` builds a `RUNNER_PREFIX` name." That command holds today (verified independently: one production hit at `:461`, and `grep -rn "agent-scaffold-checks-run"` outside `.git/` reaches only `src/checks.rs` and this plan's own documents). The reviewer explicitly framed this as characterising the standing guard rather than demanding a test, and I agree. Reopening it would relitigate a settled proportionality call.

---

## T5 (adversarial 4): the `WorktreeGuard` move is load-bearing and pinned by no test

**Verdict: VALID BUT ACCEPT RESIDUAL. Final severity: low** (confirming).

Both halves reproduce. Necessity: with the guard moved back to its pre-change position (verified against `HEAD~1`, where it sat after the `if !added.status.success()` block) and the add forced to fail, the run leaves `agent-scaffold-checks-run-1273220-1785422209423881633-0` behind; with the guard in its committed position and the same forced failure, the temp dir is empty. Unpinnedness: with the guard moved back and nothing else changed, the full `cargo test` is green (369 + 5 + 1 + 3 + 1 + 2).

No fix required. The code is correct as committed, the exposure is regression-only, and pinning it needs failure injection into `run()`'s `git worktree add` (the guard's placement, not its `Drop`, is the property). That machinery is out of proportion to a regression risk on a path that also carries an explanatory comment (`:879-881`), against plan Principle 2 (minimal by default) and `Q-66` proportionality. Recorded so a later reader knows the placement is deliberate and unguarded.

---

## T6 (adversarial 5 + evidence 5): the fixtures leak reserved directories on assertion failure

**Verdict: VALID (fix required). Final severity: low** (confirming).

Observed, not argued. Running the prepend mutation (`{RUNNER_PREFIX}{seq}-{pid}-{nanos}`) left exactly two directories under `/tmp` matching `agent-scaffold-checks-run-*` after the failing run, both from `a_reserved_path_still_carries_its_owning_pid_as_the_first_component`, whose cleanup at `:1690-1691` runs after the assertion loop at `:1682-1689`. I removed them. Because they are unregistered they are also unreclaimable (T3), so they accumulate.

That run also reproduced RED-3 exactly, both failures with the reported lines and messages: `:1684` `the owning pid must still parse out of agent-scaffold-checks-run-0-4294967295-...` and `:1600` `a live owner's worktree must not be reclaimed`.

**Minimal fix:** in `a_reserved_path_still_carries_its_owning_pid_as_the_first_component` only, move the two `fs::remove_dir` calls above the assertion loop, matching the ordering the property test already uses at `:1656-1660`. Roughly a four-line reorder with no change to the assertions or their diagnostics.

Residual accepted for the other two sites the adversarial names: a `git_ok` panic between the reservation and the add in the two prune fixtures (`:1561-1562`, `:1591-1594`), and a panicking reserving thread propagating through `taker.join()` at `:1648` before the cleanup loop. Both need scope guards or `catch_unwind` for litter that only appears on an already-failing run under investigation. Not worth the machinery.

---

## T7 (evidence 4): the retry-exhaustion regression prints ~268 KB

**Verdict: VALID BUT ACCEPT RESIDUAL. Final severity: low** (confirming).

Reproduced. Pinning `seq` to `0_u64` and the clock to the literal `12345_u128` while keeping the reservation, the single failing assertion at `:1662` emitted 268495 bytes containing exactly 1999 copies of `could not reserve a unique runner worktree directory after 16 attempts` (reported: 269119 bytes, 1999 copies; the byte difference is pid and path width). The failure is deterministic, as claimed.

No fix required. It degrades output only under a deliberate mutation that will never be committed, and it changes no shipped behaviour. If the implementer is already editing this test for T6 it is a one-line courtesy to report `failures.len()` plus the first message instead of `{failures:?}`, but I am not requiring it, and it must not become a reason to touch the test if T6 is handled elsewhere.

---

## T8 (evidence 2): the comment at `:440-441` overstates layer 1

**Verdict: DISMISSED. Final severity: low.**

The measurement behind it is sound and I confirmed it (mutation B: with `seq` constant the reservation alone carries the property and the suite stays green, so layer 2 supplies the correctness guarantee). But the doc claim does not follow. The quoted sentence sits inside a numbered list item explicitly headed "1. The name.", so "what actually separates them" is scoped to the name by its own heading. Item 2 in the same comment then attributes the guarantee to the reservation in terms the reviewer itself calls accurate: "that outcome (not an entropy argument) is what makes the returned path exclusively ours" (`:447-448`), followed by "Layer 2 is what closes the cross-process channel that layer 1 leaves open" (`:451-452`). The reviewer concedes "Read as a statement about the NAME that is true", and the surrounding structure forces that reading.

Rewording an accurate comment to pre-empt a misreading its own heading rules out is prose churn on a step whose brief warns that fix passes authoring lots of prose manufacture the next round's findings. Dismissed. (Citation also off by one, see the audit above.)

---

## T9 (evidence 6): probe numbers differ between the commit message and the review request

**Verdict: DISMISSED. Final severity: low.**

Not reproducible from the repository, which the reviewer states itself ("I cannot resolve it from the repository"). I searched: the "8 of 8 runs (4 to 157 per 16000)" phrasing appears nowhere in `docs/plans/agent-scaffold.ledger.md` or the plan; it came from the orchestrator's transient review-request prompt, which is not the artifact under review. The durable record, the commit message, is self-consistent: five numbers presented as five runs, all non-zero, and the only conclusion drawn from them (that candidate (a) alone leaves a real cross-process channel at the constant-pid template) is independently corroborated by both lenses' own re-measurements. Nothing in the artifact is wrong, and there is no proportional fix to a commit message already in history. Dismissed as not a defect in the artifact.

---

## T10 (evidence 7): the property test's guarantee is machine-dependent

**Verdict: VALID BUT ACCEPT RESIDUAL. Final severity: low** (confirming; the reviewer itself proposes no change).

Accurate and useful as a note. The cost claim reproduces directionally: the whole `checks::` module runs in 0.12 to 0.19 s here against the reported 0.08 to 0.12 s, so the 2000 `mkdir` plus 2000 `rmdir` are not a problem on a local `TMPDIR`. The observation that the property test can only fail through duplicate paths or 16 consecutive `create_dir` collisions is correct and is the same fact as T1's residual. No action; recorded so a future `TMPDIR`-related flake is diagnosed rather than rediscovered.

---

## Minimal fix set

Three fixes, all inside `src/checks.rs`. One is a shared fix.

**Fix 1 (closes T1), production plus one test.** Extract `claim_dir` as given under T1 and call it from the loop at `:462-469`; add the four-line exclusivity test. Verified here: green against `b890c4a`, RED under `create_dir` -> `create_dir_all`, RED under a panic planted in the taken branch. Single-site confirmed by `grep -n "fs::create_dir(" src/checks.rs` (one production hit, `:462`).

**Fix 2 (closes T2), same function.** Add `fs::create_dir_all(&temp)` with path-carrying error context before the loop, wrap the non-`AlreadyExists` return at `:468` with the path, and correct the "changes nothing downstream" sentence at `:453-455`. Verified here: restores the missing-`TMPDIR` case to exit 0, restores a path-naming message on the read-only case, full suite green. Fixes 1 and 2 touch the same function and should land in one edit.

**Fix 3 (closes T3), doc only.** Qualify `:37-41`, `:324-326`, and `:816-817` so they stop claiming the startup prune reclaims every SIGKILL orphan. Do not widen the prune.

**Fix 4 (closes T6), test only.** Move the two `fs::remove_dir` calls in `a_reserved_path_still_carries_its_owning_pid_as_the_first_component` above the assertion loop.

Nothing else is required. T4, T5, T7, T10 are accepted residuals and T8, T9 are dismissed. No CHANGELOG entry is owed; both lenses confirmed the reasoning independently and I do not reopen it. No `nix fmt`, per this repo's known non-formatter-clean state at HEAD.

## Advice to the orchestrator

**This round is NEW_VALID.** Four fixes are required, three of them medium: a load-bearing mechanism with no test (T1), a user-facing behavioural and diagnosability regression in shipped CLI code (T2), and a module invariant the change falsified without saying so anywhere in the code (T3). The consecutive-clean count should stay at zero.

Two independent lenses converged on T1 and I confirmed every mutation behind it, including one the adversarial did not have: **its own prescribed fix does not close the defect**, and implementing it as written would produce a green round that pins nothing. The prescription under T1 is the one I verified RED-then-green; use that.

The severity movement is one raise and no drops. T1 goes from the evidence lens's `low` to `medium`, on the ground that mutation D removes the only cross-process discriminator at the two `dead_pid()` fixtures and this project routinely runs concurrent `cargo test` processes across worktrees, and that the consequence runs through `remove_dir_all`, which is why the step was classified RISKY. Everything else keeps its reviewer rating.

No high or critical finding was raised or dismissed, so the dismissal backstop does not apply to T8 or T9.

The property itself holds and I do not disturb it: the reservation is exclusive by construction, the pid stays first, and the red-before-green demonstration is real (I reproduced RED-1 at 25-64 duplicates per 2000, RED-2 as a deterministic 1999-way exhaustion, and RED-3 with both named failures). The fix set above is about what the tests do not pin and what the change altered at the edges, not about the fix being wrong.
