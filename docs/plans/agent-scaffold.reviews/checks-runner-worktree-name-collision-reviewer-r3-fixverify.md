# Fix-verification review: `checks-runner-worktree-name-collision`, commit `6a726ed`, round 3

Lens: did the three authorised fixes from the round-2 triage (`checks-runner-worktree-name-collision-r2-triage.md`, "Minimal fix set") land exactly, close what they claim, and manufacture nothing new. Every mutation below was applied to `src/checks.rs` in this worktree, built, measured, and reverted with `git checkout -- src/checks.rs` before the next one; `git status --short` and `git diff HEAD` are both empty at the time of writing (confirmed again at the end of this file).

Baseline reconfirmed at `6a726ed`: `cargo test` 372 + 5 + 1 + 1 + 3 + 1 + 2 = 385 passed, 0 failed. `cargo clippy --all-targets` silent. Both match the commit message's measured claim exactly.

## Fix 1 (X3): `tests/checks_missing_tmpdir.rs` pins `create_dir_all(&temp)`

**Does it pin the call.** Deleted the six lines `fs::create_dir_all(&temp).map_err(...)?;` at `src/checks.rs:521-526`. Result:

```
thread 'checks_runs_under_a_tmpdir_that_does_not_exist_yet' panicked at tests/checks_missing_tmpdir.rs:78:5:
assertion `left == right` failed: a TMPDIR naming a directory that does not exist yet is legal and must still run
  left: Some(2)
 right: Some(0)
```

RED exactly as the commit message claims. Reverted; full suite back to 385/0, clippy silent.

**Precision of the two missing levels.** The test's own comment says two missing levels are needed "so creating only the leaf would not be enough either." Verified: changed `fs::create_dir_all(&temp)` to `fs::create_dir(&temp)` (single level). Same assertion failure at the same line, RED. Reverted. So the two-level construction is doing real work, not decorative.

**Determinism / environment independence.** Probed `std::env::temp_dir()` directly (a tiny standalone `rustc -O` program, no filesystem writes) under three ambient conditions:

```
-- unset --      "/tmp"
-- relative --   "reltmp"                                  (returned as-is, unresolved)
-- symlink --    ".../rev-a-tmp/link_tmpdir"                (returned as-is, unresolved)
```

Then drove the actual built binary (not the test harness) against a scratch repo with `TMPDIR` set to a two-level-missing path reached (a) through a relative value and (b) through a symlink:

```
relative TMPDIR, nested missing:  exit=0, leading dirs created under cwd
symlink TMPDIR,  nested missing:  exit=0, leading dirs created under the symlink target
```

Both succeed. The test itself always builds its own scratch base from `std::env::temp_dir()` plus a fresh, never-yet-created `missing/nested` suffix, and self-checks `assert!(!missing.exists(), ...)` before invoking the binary, so a violated assumption fails loudly rather than passing vacuously. The pattern of deriving the test's own scratch directory from ambient `std::env::temp_dir()` is pre-existing in this codebase (`tests/audit_command.rs:20`, `tests/scaffold_precommit_hook.rs:14`, `tests/checks_staged_hook_env.rs:50` all do the same thing), so it is not something this commit introduced or widened.

**Exact error text claimed in the new file's doc comment.** Reproduced by deleting `create_dir_all` and invoking the built binary directly against a scratch repo:

```
error: could not reserve the runner worktree directory <tmp>/.../agent-scaffold-checks-run-1634579-...-0: No such file or directory (os error 2)
exit=2
```

Matches the quoted text in the doc comment verbatim. Reverted.

**Verdict: Fix 1 lands exactly as claimed. No finding.**

## Fix 2 (X2, X8b): `reserve_runner_worktree_with(pid, claim)` seam and its two tests

Ran every mutation in the triage's table:

| Mutation | Observed | Matches triage table |
| --- | --- | --- |
| `if claimed` -> `if claimed \|\| true` | Both new tests FAILED (`a_claim_that_never_wins_fails_at_the_attempt_bound` panics "must not report a reservation"; `a_lost_claim_retries...` panics `left: 1, right: 3`) | Yes |
| `RUNNER_RESERVE_ATTEMPTS` 16 -> 1 | `a_lost_claim_retries...` FAILED (`AlreadyExists` after 1 attempt); `a_claim_that_never_wins_fails_at_the_attempt_bound` stayed green | Yes, exactly ("RED, the retry test fails", singular) |
| `claim_dir`: `create_dir` -> `create_dir_all` | `a_directory_claim_is_exclusive` FAILED ("a second claim on the same path is lost") | Yes, unchanged from round 2 |
| `fetch_add` -> `load` | Full suite GREEN, 385 passed, 0 failed | Yes, the documented X1 residual |

All four rows reproduce exactly as the commit message and the triage table state. Every mutation was reverted before the next.

**Production path unchanged.** Diffed the pre-commit `reserve_runner_worktree` body (`36aee66`) against the new `reserve_runner_worktree_with` body line by line: the only differences are the added `claim` parameter in the signature and `claim_dir(&path)` -> `claim(&path)`; the temp-dir creation, the retry loop, the sequence/nanos naming, and both error paths are byte-identical. `reserve_runner_worktree(pid)` is now a one-line wrapper (`reserve_runner_worktree_with(pid, claim_dir)`), and every production and fixture call site (`src/checks.rs:959`, `:1642`, `:1672-1673`, `:1799`, `:1840-1841`) still calls the one-argument wrapper. The seam is reachable from tests only via the two-argument function; nothing outside `mod tests` calls it.

**Only test calling `claim_dir` directly.** `grep -n "claim_dir(" src/checks.rs` returns the definition and exactly the two assertions inside `a_directory_claim_is_exclusive`; no other test calls it. This confirms the updated comment there ("no other test executes the LOST one through `claim_dir` itself... nothing else would notice if this stopped reporting a taken path as taken") is accurate.

**Verdict: Fix 2 lands exactly as claimed. No finding**, modulo the disclosed self-referential assertion, ruled below.

## Disclosed item: is `a_claim_that_never_wins_fails_at_the_attempt_bound`'s attempt-bound assertion a defect

The assertion in question: `assert_eq!(offered.len(), RUNNER_RESERVE_ATTEMPTS as usize, ...)`. Under the `RUNNER_RESERVE_ATTEMPTS` 16 -> 1 mutation, both sides read the mutated value, so the assertion is vacuously satisfied and only the sibling test (`a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one`) goes red. Reproduced directly above.

**Two things were checked, not just argued:**

1. The round-2 triage's own "Minimal fix set" section prescribes the test in these words: "assert `Err` with `ErrorKind::AlreadyExists`, that exactly `RUNNER_RESERVE_ATTEMPTS` names were offered, and that the message names the bound and the last path" (`checks-runner-worktree-name-collision-r2-triage.md:227`), i.e. the symbolic constant, not a literal `16`. The triage's own measured table at the same document (`:235`) already recorded "RED, the retry test fails" (singular) for this exact mutation, before this commit existed. So the implementer's test is not a weaker rendering of the prescription; it is the prescription, and the result matches what the triage itself measured when it built and reverted the same fix.
2. Whether the assertion is doing any real work at all, or is fully tautological regardless of what changes: mutated the loop bound independently of the constant (`for _ in 0 .. RUNNER_RESERVE_ATTEMPTS` -> `for _ in 0 .. 1u32`, leaving `RUNNER_RESERVE_ATTEMPTS` itself at 16). Result:

```
test checks::tests::a_claim_that_never_wins_fails_at_the_attempt_bound ... FAILED
  assertion `left == right` failed: the loop tries every attempt it promises and then stops
  left: 1
 right: 16
test checks::tests::a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one ... FAILED
```

Both go RED here, so the assertion is not vacuous in general: it catches a loop bound that diverges from its own named constant (an off-by-N class of bug), it simply cannot catch the constant's own value being retuned, because it reads that same constant to know what to expect. Hardcoding `16` in the test would catch the retuning case too, but that is a strictly different, additional assertion than what was authorised; the triage explicitly discusses proportionality and did not ask for that.

**Ruling: correct as prescribed, not a defect and not merely an accepted residual.** The mutation is killed (by the sibling test), which is what "the fix is pinned" requires under this project's own mutation-testing standard; the specific assertion under scrutiny still does genuine, non-tautological work (verified above); and the implementer's position matches the triage's own prior measurement to the letter, not just in spirit.

## Fix 3 (X4 prose, X8a): Invariant B edit and the `:495-496` (now `:515-516`) comment

**Invariant B's new clauses, checked individually:**

- "the prune additionally requires the worktree path GIT RECORDED to sit under the CURRENT process's `std::env::temp_dir()`" - true by direct reading of `prune_orphan_worktrees` (`src/checks.rs:593, 602`: `let temp = std::env::temp_dir(); ... if !path.starts_with(&temp) { continue; }`), unchanged code confirmed byte-identical to `HEAD~2` per the round-2 triage and reconfirmed here by inspection.
- "git records that path symlink-resolved" - reproduced directly: created a real symlink, added a worktree at a path reached through it, and ran `git worktree list --porcelain`:
  ```
  worktree /tmp/.../rev-a-tmp/symtest_real/agent-scaffold-checks-run-4294967295-...-0
  ```
  reported the resolved target directory (`symtest_real`), never the symlink name (`symtest_link`) used on the command line. True.
- "a registered orphan recorded outside this process's temp dir ... is never reclaimed" - reproduced end to end: registered a dead-pid (`u32::MAX`) orphan worktree under a symlinked path, then ran the built `checks` binary three times with `TMPDIR` pointing at that symlink. The orphan survived all three runs, still both registered (`git worktree list --porcelain`) and present on disk afterward. True.

**The `:515-516` comment** ("tolerating a directory that already exists is what would destroy its exclusivity", replacing the old "creating parents is what would destroy its exclusivity"): reproduced by changing `claim_dir`'s `fs::create_dir` to `fs::create_dir_all`. `a_directory_claim_is_exclusive` fails specifically on the SECOND claim ("a second claim on the same path is lost"), i.e. exactly because the already-existing leaf is tolerated on the retry, not because of anything to do with missing parent directories. This is the same mechanism `claim_dir`'s own doc at `:438-439` already states correctly. True, and the old wording's misattribution (parents, not the existing leaf) is the correction the round-2 triage's X8a demanded.

**Verdict: Fix 3's prose is entirely true against the code as it stands. No finding.**

## Scope

Checked `git diff 36aee66..6a726ed` against every forbidden item:

- `src/checks.rs:336-339` and `:874-877` (base-commit line numbers): both regions still exist, byte-identical in content, just shifted a few lines by the unrelated insertions above them (`:341-344`, `:892-897` in the new tree; diffed content matches exactly).
- No prune widening: `prune_orphan_worktrees`'s body untouched; `grep -n "canonicalize"` returns nothing.
- No clock parameter added: `reserve_runner_worktree_with`'s signature is `(pid: u32, claim: impl Fn(&Path) -> io::Result<bool>)`, not a clock.
- No `temp_dir()` canonicalisation added.
- No relative-`TMPDIR` validation added: `grep -n "is_absolute"` returns nothing.
- X5 (retry comment wording), X6 (relative-`TMPDIR` validation), X7 (errno specificity) are all untouched, matching "not required, and deliberately so" in the triage.

**Verdict: the implementer stayed exactly inside the authorised fix set. No finding.**

## Re-seeding: every new or changed sentence

Read every new/changed line in the diff (module doc, the two function doc comments, the `claim_dir`-adjacent comment, the `a_directory_claim_is_exclusive` doc update, both new tests' inline comments, and the full new integration test file including its module doc). Each factual claim checked above reproduced true. Two additional things checked that did not turn into findings:

- The new `reserve_runner_worktree_with` doc says "nothing ever exercises the lost-claim verdict, the retry, or the exhaustion error at their use site" as the MOTIVATION for the seam. Read literally this could seem contradicted by the two tests added in the same commit, which do exercise exactly that; but the sentence is scoped to what happens through a REAL claim (production and the prune fixtures, which always win), and the two new tests exercise the lost/exhaustion paths only via the injected closure this same paragraph introduces. Not false, just worth flagging that it reads best as "nothing outside the injected seam" rather than "nothing at all."
- The new integration test's doc comment says setting `TMPDIR` in-process "needs the unsafe `std::env::set_var`." Checked the actual std source shipped with this toolchain (`rustc 1.98.0-nightly`, `library/std/src/env.rs:359`): `set_var` is declared `pub unsafe fn`, confirming the claim; the fact that this crate's `edition = "2021"` (`Cargo.toml:4`) lets a bare call compile without a literal `unsafe { }` block (verified: a standalone `rustc --edition 2021` probe compiled and ran a `set_var` call with no error or warning) does not make the "unsafe" framing false, since the function's own signature and documented soundness requirement ("the only sound option is to not use `set_var` ... in multi-threaded programs at all") are exactly what the comment is pointing at. Not a finding.

No sentence found false or over-broad.

## Verdict

**Zero findings from the fix-verification lens on round 3.** All three authorised fixes land exactly as the triage prescribed, close what they claim to close (reproduced by mutation in every case), stayed inside the authorised scope, and introduced no new false or over-broad prose. The one disclosed item (the self-referential attempt-bound assertion) is ruled correct as prescribed, not a defect, on the strength of an independent mutation (decoupling the loop bound from the constant) that shows the assertion is not tautological in general, plus the triage's own prior measurement of the identical mutation.

## Reverted state and hygiene

`git status --short`: empty. `git diff HEAD`: empty. Confirmed with a final run after all probes:

```
cargo test: 372 + 5 + 1 + 1 + 3 + 1 + 2 = 385 passed, 0 failed
cargo clippy --all-targets: silent (Finished, no warnings)
```

`/tmp` directories named `agent-scaffold-*` created by this review: **0**. `find /tmp -maxdepth 1 -iname "agent-scaffold-*"` returns the same 65 pre-existing entries before and after this session (all predating this session, none touched); every scratch artifact this review created (probe binaries, scratch git repos, symlink fixtures, mutation-triggered leftovers) was created under and removed from `$TMPDIR` (`/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/rev-a-tmp`), which is empty at the time of writing.
