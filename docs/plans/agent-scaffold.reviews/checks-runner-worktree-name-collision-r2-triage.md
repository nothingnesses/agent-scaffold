# Triage: `checks-runner-worktree-name-collision` (commit `11d60f3`, round 2)

Adjudicated in an isolated worktree at detached HEAD `11d60f3`. Every mutation and every prescribed fix below was applied, built, measured, and reverted; `git status` is clean and `git diff HEAD` is empty at the time of writing, apart from this findings directory. All test runs were pointed at a scratch `TMPDIR` under this session's scratchpad, not `/tmp`.

Baseline at `11d60f3`: `cargo test` green, 370 + 5 + 1 + 3 + 1 + 2 = 382 passed, 0 failed. `cargo clippy --all-targets` silent.

Inputs: `checks-runner-worktree-name-collision-r2-reviewer-verification.md` (3 findings, all `low`) and `checks-runner-worktree-name-collision-r2-reviewer-adversarial.md` (5 findings, 3 `medium` + 2 `low`). Eight raw findings, seven after dedup and one split.

## Deduplication

| Triage id | Merged from | Subject |
| --- | --- | --- |
| X1 | verification F1 (layer-1 half) | The atomic sequence is unpinned, and the reason on the record for accepting that is factually wrong. |
| X2 | adversarial F2 + verification F1 (retry half) | The reservation loop's collision handling, both the `if claimed` verdict and the retry, is executed by no test. |
| X3 | adversarial F1 | The `create_dir_all(&temp)` fix that restored a missing `TMPDIR` is pinned by no test. |
| X4 | adversarial F3 | The newly written Invariant B bound omits the prune's temp-dir gate, which silently skips a registered dead orphan. |
| X5 | adversarial F4 | The retry comments name a cause the same commit's own Invariant B text contradicts. |
| X6 | adversarial F5 | A relative `TMPDIR` makes the reservation guard a path git does not use. |
| X7 | verification F2 | `create_dir_all` reports a less specific errno than the pre-fix path did. |
| X8 | verification F3 | Two comments written by this commit state something false. |

**One merge and one split.** X2 merges adversarial F2 with the retry half of verification F1: they are two mutations of the same three lines (`:515-527`), they fail for the same reason (nothing in the repository ever makes a claim lose), and one fix closes both. Verification F1 is SPLIT because its two halves are different properties with different fixes and different verdicts: the layer-1 half (X1) is about `{seq}`, the retry half is about what the loop does when a claim is lost.

The two lenses did not collide on X3, X4, X5, X6, X7 or X8; each is single-lens.

---

# The through-line, judged as a pattern

The orchestrator asked whether the suite fails only when several layers are broken at once. **It does, and the picture is worse than either lens reported.** Measured here, each on the committed tree, each reverted:

| Mutation | What it removes | Full `cargo test` | `clippy --all-targets` |
| --- | --- | --- | --- |
| Delete `fs::create_dir_all(&temp)` at `:501-506` | the missing-`TMPDIR` fix | GREEN, 382 | silent |
| `fetch_add` -> `load` at `:509` | layer 1, the atomic sequence | GREEN, 382 | silent |
| `if claimed` -> `if claimed \|\| true` at `:524` | layer 2's verdict at its only use site | GREEN, 382 | silent |
| `RUNNER_RESERVE_ATTEMPTS` 16 -> 1 | the retry loop and the exhaustion error | GREEN, 382 | silent |
| Whole body -> the PRE-FIX `format!` | layer 1 AND layer 2 together | RED, 8 of 8 runs | n/a |

So four of the five load-bearing parts of this step's fix can each be deleted on their own with a green suite and a clean clippy. The suite goes red only when the entire construction is replaced at once. That is the state round 1 diagnosed for `T1` and ruled `medium`; the fix that landed closed the `claim_dir` FUNCTION and left the loop that uses it in exactly the same condition.

**Two corrections to the record, both measured, both stronger than the reviewers' versions.**

First, why the layer-1 mutation stays green. The implementer's recorded reason ("the 16 retries absorb the collisions") and round 1's triage `T1` reason ("with `seq` constant the raw names duplicate at the clock's rate, the reservation's retry path carries the property") are BOTH false. The verification lens showed this with counters; I confirmed it with a stronger probe. I applied the layer-1 mutation (`fetch_add` -> `load`) AND planted `panic!` on the lost-claim branch, so any retry at all aborts the run, then ran the whole `checks::` module three times:

```
test result: ok. 30 passed; 0 failed  (x3)
```

Zero claims lost, so zero retries, so there is nothing for the retries to absorb. The reason the mutation stays green is that `nanos()` alone still separates the names, because the `mkdir` syscall inside the loop spaces successive clock readings by microseconds. Neither layer of the fix is what keeps that mutation green.

Second, and following from it: **the property test's red/green signal is carried mainly by the presence of a syscall in the loop, not by either uniqueness layer.** `concurrent_reservations_never_share_a_runner_worktree_path` is RED against the pre-fix construction (8 of 8 runs) because that mutation removes the `mkdir` and returns the loop to a tight clock read. The margin is thin: 1, 2, 3, 4, 4, 6, 8, 9, 15 duplicates per 2000 across nine runs, against the 8.7% the step brief predicted for a tight-loop probe. The test is a legitimate end-to-end property assertion and I am not disturbing it, but it should not be read as evidence that either layer works, and its redness is probabilistic rather than deterministic.

**What this does NOT justify.** It does not justify a test per branch. Three of the four silent mutations matter for different reasons and at different severities, and I am requiring pins for two of them, refusing one, and taking the fourth as a documented residual. The proportionality line I drew is in the fix set below: after it, five of the six mutations above are RED and the one that stays green has a true reason on the record instead of a false one.

---

# Verdicts

## X1 (verification F1, layer-1 half): the atomic sequence is unpinned, and the recorded reason is wrong

**Verdict: VALID BUT ACCEPT RESIDUAL, with the recorded reason CORRECTED. Final severity: `low`** (confirming the reviewer's rating).

The mutation reproduces (`fetch_add` -> `load`, full suite green), and the false-mechanism claim reproduces as described above, more decisively than with counters.

**I built the reviewer's proposed pin and measured it, because a prescription is not a diagnosis.** Its clock parameter is exactly as described: `reserve_runner_worktree(pid)` delegating to `reserve_runner_worktree_with(pid, clock: fn() -> u128)`, with `clock()` at `:510`, plus a frozen-clock test taking four reservations with `|| 0`. Measured:

- Against `11d60f3`: `cargo test` green, 383 passed. `cargo clippy --all-targets` silent.
- Under `fetch_add` -> `load`: RED, `reservations failed with a frozen clock: ["could not reserve a unique runner worktree directory after 16 attempts (last tried .../agent-scaffold-checks-run-1424368-0-0)", ...]`, 30 passed, 1 failed.

So it works for what it directly claims. **But its advertised side-benefit is FALSE, and that is why I am not requiring it.** The reviewer writes that "as a side effect it is the only thing in the repository that would ever execute the retry loop and the exhaustion error, which closes the mutation-R hole above at the same time". Measured with the pin in the tree:

| Mutation, with the clock pin applied | Result |
| --- | --- |
| `RUNNER_RESERVE_ATTEMPTS` 16 -> 1 | GREEN, 371 + 5 + 1 + 3 + 1 + 2 |
| `if claimed` -> `if claimed \|\| true` | GREEN, 371 + 5 + 1 + 3 + 1 + 2 |

On CORRECT code the frozen-clock test never loses a claim, because `{seq}` still advances, so it never enters the retry loop either. The loop is executed only in the mutant state, which is the state where the test is already red. The pin closes layer 1 and nothing else. This is the third time in this task that a reviewer's prescription failed under measurement while its diagnosis held; the lesson round 1 recorded applies again.

**Why I accept the residual rather than take the pin anyway.** With Fix 2 below in place, layer 2 is pinned end to end (the claim function, the verdict at its use site, the retry, and the exhaustion error). A future edit that breaks `{seq}` therefore degrades the module to "collisions become retries", not to "two callers share a path": the correctness guarantee is carried by a layer that is then fully tested. Adding a SECOND testability parameter to `reserve_runner_worktree` to guard an optimisation, on a step already carrying one, is where plan Principle 2 (minimal by default) and `Q-66` proportionality bite.

**The reason on the record MUST change, and this is not optional.** Whatever the orchestrator decides about the pin, the following are measurably false and must not be carried forward:

- "the 16 retries absorb the collisions, so it never exhausts" (the implementer's round-2 explanation).
- "with `seq` constant the raw names duplicate at the clock's rate, the reservation's retry path carries the property" (round 1 triage, `T1`).
- "layer 1 is an optimisation that keeps the retry path cold" (round 1 triage, `T1` residual). The retry path is cold with or without layer 1; the `mkdir` latency is what keeps it cold.

The true reason, which is what an accept-residual here must carry: **layer 1 is an optimisation on the NAME. Correctness is carried by layer 2, which after Fix 2 is pinned at the function, at its use site, on the retry, and on the exhaustion error. Removing layer 1 turns collisions into retries, not into shared paths, and no test observes that difference.**

## X2 (adversarial F2 + verification F1, retry half): the reservation loop's collision handling is executed by no test

**Verdict: VALID (fix required). Final severity: `medium`** (confirming the adversarial lens; RAISING the verification lens's `low` for the retry half, which shares the fix and takes the group's severity).

Both mutations reproduce exactly on the committed tree, full `cargo test`:

- `if claimed` -> `if claimed || true` at `:524`: GREEN, 382 passed, 0 failed. `clippy --all-targets` silent.
- `RUNNER_RESERVE_ATTEMPTS` 16 -> 1 (a lost claim becomes an immediate exhaustion error instead of a retry): GREEN, 382 passed, 0 failed. `clippy --all-targets` silent.

(I used the attempt-bound form rather than the reviewer's `return Err` at `:527`; the reviewer's own `break`-shaped variant trips clippy's `never_loop`, and the bound form is silent under clippy, which makes the hole strictly larger than reported.)

**This is not a re-raise of a settled item, and it is not the layer-1 residual.** `T4`'s agreed guard is "exactly one `format!` in `src/checks.rs` builds a `RUNNER_PREFIX` name"; I re-ran it and it returns exactly one hit (`:510`) under both mutations, so the settled evidence does not see this. `a_directory_claim_is_exclusive` pins `claim_dir`'s two return values and never calls `reserve_runner_worktree`. Round 1 ruled this species (`T1`, "the `create_dir` reservation is pinned by no test") VALID at `medium` and required a fix; the fix pinned the function and left its consumer untouched, and round 1 never mutated the consumer. This is the unfixed remainder of a `medium` finding, newly measured, not a new argument about a closed one.

**Why `medium` and not `low`.** What the mutation removes is the only thing standing between the module and the outcome the step was classified RISKY for: two callers handed one path, and `WorktreeGuard::drop` (`:347-359`) calling `remove_dir_all` on a directory another live run is inside. Layer 2 is what closes the cross-process channel, which is the channel the two `dead_pid()` fixtures occupy, since their first name segment is the constant `u32::MAX` in every process. The code's own doc (`:475-480`) says that outcome, "not an entropy argument", is what makes the returned path exclusively ours, and nothing checks it. That is the AGENTS.md standard "tests must actually exercise the code they claim to" applied to the decisive mechanism of the step, which is the ground round 1 used to put this at `medium`.

**Why not `high`.** Nothing shipped is wrong today, and the channel layer 2 closes needs two independently launched processes to reach the same `format!` within about 25 ns.

**I implemented the adversarial lens's prescribed fix and measured it; it works.** See Fix 2 below for the measurements.

## X3 (adversarial F1): the `TMPDIR` regression fix is pinned by no test

**Verdict: VALID (fix required). Final severity: `medium`** (confirming).

Reproduced end to end. With `fs::create_dir_all(&temp)` at `:501-506` deleted: full `cargo test` GREEN, 382 passed, `clippy --all-targets` silent. A/B of the built binary against a throwaway repo carrying one trivial `lint` check, with `TMPDIR` naming a two-level path that does not exist:

```
mutated:   error: could not reserve the runner worktree directory <TMPDIR>/agent-scaffold-checks-run-1408573-1785426640541666520-0: No such file or directory (os error 2)
           exit=2
committed:         pass  lint (lint)
           checks: 1 passed, 0 failed, 0 skipped
           exit=0, and the leading directories were created
```

`grep -rn "TMPDIR" tests/ src/` reaches only two comments in `src/checks.rs`; no test in the repository sets `TMPDIR` at all.

**Why `medium` stands.** This is the one finding whose probability is DEMONSTRATED rather than argued: the regression it guards is not hypothetical, it happened in this very step, and round 1 rated it `medium` as a user-facing regression in shipped CLI code. The code states the claim explicitly at `:488-490` ("a `TMPDIR` naming a directory that does not exist yet is legal, and it worked before the reservation existed") and nothing executes it. The fix costs no production change at all, so the usual proportionality objection to pinning does not apply. Of the three fixes I am requiring, this is the one I would keep if only one could be taken.

## X4 (adversarial F3): the newly written Invariant B bound is incomplete, and the prune permanently skips a registered dead orphan

**Verdict: VALID (fix required for the PROSE; the CODE fix is correctly out of scope and goes to a NEW STEP). Final severity: `medium`** (confirming the reviewer's rating for the mechanism).

Reproduced with four runs of the built binary. Git records the symlink-resolved path, so `path.starts_with(&temp)` at `:582` never matches when `TMPDIR` reaches the temp dir through a symlink:

```
worktree .../scratchpad/real/agent-scaffold-checks-run-4294967295-1785000000000000000-0   (resolved, not "link")
run 1 exit=0 / run 2 exit=0 / run 3 exit=0 / run 4 exit=0
ls .../real  ->  agent-scaffold-checks-run-4294967295-1785000000000000000-0   (still there)
```

The owner is `u32::MAX`, so the liveness gate would reclaim it; only the temp-dir gate defeats the prune. The sharper symptom reproduces too, running the unit binary with `TMPDIR` pointing at a symlink:

```
test checks::tests::a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one ... FAILED
test checks::tests::a_startup_prune_reclaims_an_orphaned_runner_worktree ... FAILED
test result: FAILED. 28 passed; 2 failed; 0 ignored; 0 measured; 340 filtered out
```

with panics at `src/checks.rs:1662` and `src/checks.rs:1631`, and five leaked entries in the resolved temp dir.

**Pre-existing, verified independently and more strictly than the reviewer did.** I extracted the whole `prune_orphan_worktrees` body from `HEAD~2` and from `HEAD` and diffed them: **byte-identical**. This step did not touch the mechanism.

**The reviewer's split is RIGHT, and I endorse it.** Assessing it as asked:

- The prose half is squarely in scope. The over-promising sentence is text THIS commit wrote, the step's own documentation-impact section commits to correcting every comment that over-promises here, and round 1's `T3` (the same sentence, a larger error) was ruled VALID at `medium` and fixed. The new text says the bound is registration, calls it "real, not theoretical", and names the reservation-window kill as THE gap, which reads as exhaustive. It is not: reclamation additionally requires the path git recorded to sit under the CURRENT process's `std::env::temp_dir()`.
- The code half is genuinely out of scope. The prune body is untouched by the step; the step brief scopes the work to the name generator, the call site, the three fixtures and four named comments, and it has an explicit "checked and NOT affected, so the step does not widen into them" section establishing the project's norm of keeping steps narrow. Above all, changing the temp-dir gate changes WHAT THE PRUNE IS WILLING TO DELETE, which is destructive-cleanup authority on a step already classified RISKY and already on round 2 of a 5-round cap. Widening into it here would need its own risk classification and its own red-before-green demonstration, which is the definition of a separate step. The step brief's own "RELATION TO STEP 85" section rules against exactly this kind of same-family merge.

The tempting counter-argument, that this step exists to stop the suite lying and two of its tests fail under a symlinked `TMPDIR`, does not carry: that environment dependence is pre-existing, orthogonal to clock-based name collisions, and it is the strongest single item to put in the new step's brief rather than a reason to widen this one.

**Severity `medium` confirmed for the mechanism**, which is what stays unfixed after this step: silent, unbounded accumulation of registered full checkouts, plus a `cargo test` result that depends on the developer's `TMPDIR`. The in-scope remedy is one clause; the rest is routed.

## X5 (adversarial F4): the retry comments name a cause the same commit contradicts

**Verdict: VALID BUT ACCEPT RESIDUAL. Final severity: `low`** (confirming).

The three citations read as quoted. `:427-428` and `:511-512` both say a second attempt means another PROCESS holds the name, while `:45-48` (written by this same commit) documents an empty unregistered directory a killed run leaves behind, which is a name held by nothing. The observation is correct.

**No fix required, because nothing follows from it.** For a leaked directory to cause a retry, a later candidate must reproduce the same `{pid}-{nanos}-{seq}` triple, which needs pid reuse plus a nanosecond-exact clock match plus a matching sequence value. The reviewer concedes the deduction the comment draws is unaffected. Round 1's `T8` precedent (dismissing a comment reworded to pre-empt a misreading) and `T7`'s "a one-line courtesy, but I am not requiring it" both cover this. This project has explicit recorded evidence that prose-heavy fix passes manufacture the next round's findings, and this is the item on the list most likely to do that. If the implementer is editing that doc comment for another reason, correcting it is free and welcome; it must not become a reason to open it.

## X6 (adversarial F5): a relative `TMPDIR` makes the reservation guard a path git does not use

**Verdict: VALID BUT ACCEPT RESIDUAL here; ROUTED to the new step with X4. Final severity: `low`** (confirming).

Reproduced exactly, and I checked the attribution the reviewer made, which it did not measure. Running the built binary from an empty working directory with `TMPDIR=reltmp`:

```
committed (11d60f3):  exit 0;  <cwd>/reltmp created (NEW);  <repo>/reltmp created
pre-step  (HEAD~2):   <cwd> untouched;                      <repo>/reltmp created
```

So the reviewer's split is right: the directory in the process working directory is new to this step, the one inside the repository is pre-existing (git resolved the relative path against the repo before too). Its more interesting consequence also holds: the directory the reservation claims and the directory git uses are different, so this step's exclusivity covers a path that is not the worktree.

**Low, and routed rather than fixed here.** It needs a malformed `TMPDIR` (POSIX requires an absolute path), and no statement this commit wrote is falsified by it: `:934-938` claims uniqueness of the RETURN VALUE, which remains true. It shares a root cause with X4 (`std::env::temp_dir()` consumed raw, unvalidated and uncanonicalised, at `:494` and `:573`), so one new step closes both. No comment is owed here; the roadmap entry is the record.

## X7 (verification F2): `create_dir_all` reports a less specific errno

**Verdict: VALID as a RECORD CORRECTION; no code fix. Final severity: `low`** (confirming the reviewer's rating and its "no fix required" call on the code).

Both rows reproduce against the built binary:

```
TMPDIR is a regular file:     error: could not create the temp directory <path>: File exists (os error 17)   exit=2
TMPDIR is a dangling symlink: error: could not create the temp directory <path>: File exists (os error 17)   exit=2
TMPDIR unwritable:            error: could not reserve the runner worktree directory <path>: Permission denied (os error 13)   exit=2
```

The reviewer's analysis is right and its conclusion is right: this is inherent to `fs::create_dir_all`, all three still fail correctly, and the message now names the operation and the path, which is what `T2` asked for. No code fix.

**I am raising one thing neither lens noticed, which is why this is not simply a non-finding.** The commit message for `11d60f3` states, as a measured result: "A `TMPDIR` under a regular file now reports `could not create the temp directory <path>: Not a directory (os error 20)`". The binary reports `File exists (os error 17)`. Two independent measurements (the verification lens's and mine) agree against the commit message. On a step whose entire burden is reproducible evidence (`Q-66`, plan Principle 6), a quoted measurement in the durable record that the artifact does not produce is worth correcting. Round 1's `T9` precedent settles the remedy: a commit message already in history is not fixable, so this is a ledger correction, not a code change.

The related non-finding is confirmed: `grep -rn "raw_os_error\|\.source()\|downcast" src/ tests/` returns nothing, so dropping `raw_os_error()` is observed by nothing.

## X8 (verification F3): two comments written by this commit state something false

**Verdict: VALID (fix required), both. Final severity: `low`** (confirming).

- **X8a, `src/checks.rs:495-496`**: "`claim_dir` deliberately creates exactly ONE level (creating parents is what would destroy its exclusivity)". Creating parents is not the cause; tolerating an already-existing LEAF is, which `claim_dir`'s own doc gets right at `:437-439`. **Required, unlike X5, because a wrong action follows from believing it**: a maintainer who reads "creating parents" as the hazard can conclude that `create_dir_all` is safe wherever the parents already exist, which is precisely the mutation the step's own new test exists to kill (measured RED under `create_dir` -> `create_dir_all`). Six words.
- **X8b, `src/checks.rs:1673`**: "Both outcomes matter, and neither was executed by any other test." My panic probe settles it directly: the LOST outcome is executed by no other test (true), while the WON outcome runs about 2020 times per `checks::` run. **Required as a consequence of Fix 2**, not on its own merits: once the two new tests land, the lost outcome IS executed elsewhere, so leaving the sentence makes it newly and more visibly wrong. It travels with Fix 2 at no extra cost.

---

# Minimal fix set

Three fixes. Each was implemented here, measured green on `11d60f3` and RED under the mutation it claims to kill, then reverted. Combined state after all three: `cargo test` 385 passed, 0 failed; `cargo clippy --all-targets` silent.

**Fix 1 (closes X3). Test only, no production change. New file, about 55 lines.**

An integration test in the existing style of `tests/checks_staged_hook_env.rs`, which already spawns the built binary with a custom environment (`Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))` plus `.env(...)`, `tests/checks_staged_hook_env.rs:33-46`). Create a scratch repo with one trivial `lint` check, run `checks` with `TMPDIR` pointing at a nested path that does not exist, assert exit 0. This avoids the unsafe `std::env::set_var` a unit test would need.

Measured: green against `11d60f3`; RED with `:501-506` deleted, `assertion left == right failed: a TMPDIR naming a directory that does not exist yet is legal and must still run, left: Some(2), right: Some(0)`. Deterministic, no concurrency, no sampling.

Single-site: `grep -n "create_dir_all(&temp)" src/checks.rs` returns one hit, `:501`. `grep -rn "TMPDIR" tests/` returns zero, so nothing duplicates it.

**Fix 2 (closes X2 and X8b). Production seam of about 8 lines plus two tests of about 35.**

`reserve_runner_worktree(pid)` delegates to `reserve_runner_worktree_with(pid, claim_dir)`, where the second parameter is `claim: impl Fn(&Path) -> io::Result<bool>`, and `:515` calls `claim(&path)` instead of `claim_dir(&path)`. Two tests:

1. A claim closure that records every path it is offered and loses the first two: assert the reservation succeeds, that exactly three names were offered, that the returned path is the third, and that neither lost name is handed back. This is the assertion the current code cannot state at all.
2. A claim closure that always loses: assert `Err` with `ErrorKind::AlreadyExists`, that exactly `RUNNER_RESERVE_ATTEMPTS` names were offered, and that the message names the bound and the last path.

Measured with Fix 1 also present:

| State | Result |
| --- | --- |
| `11d60f3` unmutated | GREEN, 385 passed; clippy silent |
| `if claimed` -> `if claimed \|\| true` | RED, both new tests fail |
| `RUNNER_RESERVE_ATTEMPTS` 16 -> 1 | RED, the retry test fails |
| `claim_dir` `create_dir` -> `create_dir_all` | RED, `a_directory_claim_is_exclusive` fails (unchanged) |
| `fetch_add` -> `load` | GREEN (the X1 residual, unchanged and now correctly explained) |

Update the `:1673` comment in `a_directory_claim_is_exclusive` in the same edit (X8b).

Single-site: `grep -n "if claimed" src/checks.rs` returns one hit, `:524`; `grep -n "claim_dir" src/checks.rs` returns one production call, `:515`, plus the definition and the existing test. `grep -n "reserve_runner_worktree" src/checks.rs` confirms the production call site is `:939` and the three fixtures are `:1622`, `:1652`, `:1653`, all of which keep calling the unchanged one-argument wrapper.

**Fix 3 (closes X4's prose half and X8a). Documentation only, two clauses.**

- Invariant B at `src/checks.rs:42-51`: add that reclamation additionally requires the worktree path GIT RECORDED to sit under the current process's `std::env::temp_dir()`, that git records it symlink-resolved, and that an orphan recorded outside it is never reclaimed. Do NOT touch `:336-339` or `:874-877`: those say registration is necessary, which stays true, and only Invariant B states the bound as if it were exhaustive. Do NOT widen the prune.
- `src/checks.rs:495-496`: "creating parents is what would destroy its exclusivity" becomes "tolerating a directory that already exists is what would destroy its exclusivity".

**Not required, and deliberately so.** The clock parameter for X1: measured, it does what it claims and nothing more, and its advertised side-benefit is false. Adding a second testability parameter to guard an optimisation whose failure mode is now bounded by a fully pinned layer 2 is where Principle 2 and `Q-66` stop this. X5's comment rewording. X7's errno behaviour. X6's relative-`TMPDIR` validation.

# Route to a NEW ROADMAP STEP

**One new step, closing X4's code half and X6.** Shared root cause: `std::env::temp_dir()` is consumed raw, uncanonicalised and unvalidated, at `src/checks.rs:494` (the reservation) and `src/checks.rs:573` (the prune's gate). Scope it as: compare the prune's gate against a canonicalised temp dir (or match on the file name alone), and reject a relative temp dir with a clear error rather than creating it. It must carry its own risk classification, because it changes what the prune is willing to delete.

Evidence to carry into its brief, all reproduced here: four consecutive runs of the shipped binary leave a registered dead-owner orphan in place under a symlinked `TMPDIR`; `a_startup_prune_reclaims_an_orphaned_runner_worktree` and `a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one` both FAIL in that environment (28 passed, 2 failed, panics at `:1631` and `:1662`, five leaked entries), which makes `cargo test` a statement about the developer's `TMPDIR`; the prune body is byte-identical at `HEAD~2` and `HEAD`, so the mechanism predates this step; and a relative `TMPDIR` puts an empty directory in the caller's working directory while git puts the worktree inside the repository.

# Record corrections owed, independent of the fix set

1. The layer-1 accept-residual reason. Replace "the 16 retries absorb the collisions", "the raw names duplicate at the clock's rate, the reservation's retry path carries the property", and "layer 1 is an optimisation that keeps the retry path cold" with the measured reason given under X1. All three are false: under the layer-1 mutation there are ZERO lost claims and ZERO retries across three full-module runs with a panic planted on the lost-claim branch.
2. The commit message for `11d60f3` quotes `Not a directory (os error 20)` for a `TMPDIR` under a regular file; the binary produces `File exists (os error 17)` (X7).

# Advice to the orchestrator: NEW_VALID

**This round is NEW_VALID, and the streak stays at 0.** Three fixes are required: two at `medium` (X2 and X3) and one whose mechanism is `medium` with an in-scope prose remedy (X4). Two of the three are pure tests or pure prose; only Fix 2 touches production code, by 8 lines.

I did not shade toward convergence and I want the reasoning on the record. The step is on round 2 of a 5-round cap and needs two consecutive clean rounds, so requiring a fix here costs one round out of three remaining. Against that: round 1 ruled this exact species VALID at `medium`, the fix closed half of it, and the reason recorded for accepting the other half is measurably false. Declaring clean now would convert a half-closed `medium` finding plus a false record into a converged step, on a step classified RISKY because its failure mode runs through `remove_dir_all`. That is a worse outcome than a third round.

Severity movement: one raise, no drops. The retry half of verification F1 moves from `low` into X2's `medium`, because it shares a fix with the claim-verdict half and the group takes the severity of what it protects. Everything else keeps its reviewer's rating, including X4 at `medium` (I considered lowering it, since the in-scope remedy is one clause, but severity is an absolute rating of what stays unfixed, and what stays unfixed is the mechanism).

Two prescriptions were tested rather than trusted, per the lesson round 1 recorded. The verification lens's clock parameter WORKS for layer 1 but does NOT close the retry hole it claims to close as a side effect; measured green under both `RUNNER_RESERVE_ATTEMPTS = 1` and `if claimed || true` with the pin applied. Do not adopt it on the reviewer's stated grounds. The adversarial lens's injectable-claim prescription works exactly as described and is what Fix 2 is built from.

Nothing settled was reopened. I did not touch the (a)+(d) choice, `nanos()` being retained, the `T4` bypass, the `T5` guard move, the SIGKILL-leak trade, the no-CHANGELOG call, or the uniqueness property itself, and I did not propose widening the prune to sweep the temp dir by prefix. No `high` or `critical` finding was raised or dismissed, so the dismissal backstop does not apply. No finding was dismissed outright.

# Worktree and temp-directory state

`git status --short` shows only this reviews directory; `git diff HEAD` is empty; HEAD is still `11d60f3`. Every mutation, probe and prescribed fix was reverted with `git checkout -- src/checks.rs`, and the one file I added (`tests/checks_missing_tmpdir.rs`) was removed.

Temp-directory hygiene. Every test run was pointed at a scratch `TMPDIR` inside this session's scratchpad, never `/tmp`. The runner reservations there are created and removed by the tests themselves (about 2020 per `checks::` run, across roughly twenty runs). **Persistent directories I created and removed: 3** (two `agent-scaffold-checks-test-*-claim-dir` scratch directories left by deliberately-failing mutation runs, and one `agent-scaffold-missingtmp-*` left by the deliberately-failing Fix 1 red run), plus six fixture trees (`fix`, `cwd`, `missing`, `real`, `link`, and the scratch `TMPDIR` itself), all deleted. **Directories I created in `/tmp`: 0.**

`/tmp` at the time of writing holds 0 `agent-scaffold-checks-run-*` directories, so the 32272 the previous reviewer reported have already been cleaned. It still holds 65 `agent-scaffold-checks-test-*` directories; the newest are from pid `1375127` at 16:33, which is the other round-2 reviewer's process, and the rest predate this session. None are mine and I left them alone.
