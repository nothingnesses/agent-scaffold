# Reviewer findings: optional-modules increment 2c-i (`checks` subcommand)

Reviewer lens: isolation correctness, cleanup guarantees, command-execution safety, exit-code correctness. Adversarial. Diff range `38ce9ec..76d9b83`, commit `76d9b83`.

Verification method: read `src/checks.rs` and `src/main.rs` from the branch; built the binary in a throwaway worktree at `76d9b83`; ran `cargo test` (17 checks tests pass), `cargo clippy --all-targets -- -D warnings` (clean); then drove the built binary against purpose-built throwaway git repos to attack each invariant. Every empirical claim below was reproduced.

## Headline

The core isolation guarantee HOLDS for well-behaved checks: a format check that reformats a tracked file in place does not touch the live tree, and a `../` write escapes only into the system temp dir (where the worktree lives), not into the repo. No high or critical isolation-escape and no guaranteed worktree leak on the normal / failing-check / internal-error paths (all covered by the `Drop` guard, confirmed). There is NOTHING at critical or high severity.

The real issues are at medium: an exit-code contract violation for IO/environment errors (documented as 2, actually 1), a worktree leak on SIGINT/SIGKILL (Drop cannot run), and hang vectors (inherited stdin, and the classic `Command::output()` deadlock on a background/grandchild process) that both stall the run and, when a user breaks the stall with Ctrl-C, trigger the leak.

---

## Finding 1 [medium] IO / environment errors exit 1, not the documented 2 (invariant D violation)

`RunError::Io`'s own doc says "An underlying IO error (exit 2)" (`src/checks.rs:222`) and the module-level invariant D (`src/checks.rs:23-25`) says "Usage and environment errors ... exit 2." But `run_checks` routes `RunError::Io` through `return Err(error)` (`src/main.rs:424`), and `fn main() -> io::Result<()>` returning `Err` makes Rust's `Termination` impl print `Error: {e:?}` and exit with `ExitCode::FAILURE` = 1. So every IO error exits 1, contradicting the code's own documentation and invariant D.

Reproduced: a config made unreadable (`chmod 000 .agents/checks.toml`) is an environment problem, but the run exits 1, indistinguishable from "a check failed" (also 1) and "malformed config" (also 1). A CI script that treats 2 as "infra/usage problem, retry or alert" and 1 as "checks genuinely failed" will misclassify an unreadable/permission/filesystem error as a check failure.

```
io-error(unreadable) exit=1   # RunError::Io doc says exit 2
not-a-repo exit=2  no-commits exit=2  malformed exit=1  failing-check exit=1
```

Root cause is the `return Err` shortcut versus the explicit `eprintln!; process::exit(2)` used for the other environment variants. Note the surrounding code already establishes the convention of explicit `exit(2)` for environment errors (`src/main.rs` `run_git_init` and `run_scaffold` use `std::process::exit(2)` for these). To honor the documented mapping, `RunError::Io` should print and `exit(2)` like the sibling environment variants, not fall through to the `io::Result` `Err` path.

Related test gap (Principle 11): NO test exercises the exit-code mapping at all. The 17 module tests assert on `Report` / `RunError` values only; the `RunError`-variant-to-exit-code translation lives entirely in `run_checks` in `main.rs` and is untested. That is exactly why this mismatch slipped through. Invariant D's exit-code half is asserted nowhere.

## Finding 2 [medium] Worktree leak on SIGINT / SIGKILL; no self-healing prune, so leaks accumulate

The cleanup is a `WorktreeGuard` `Drop` (`src/checks.rs:270-283`). `Drop` runs on normal return and on panic-unwind, but NOT on `SIGINT`, `SIGTERM`, or `SIGKILL`. A `checks` run is exactly the kind of long-lived command a user aborts with Ctrl-C (linters, and later `test`/`mutation`).

Reproduced: start a run whose check is `sleep 60`, send `SIGINT` after 2s. Result:

- the temp worktree stays registered in `git worktree list` (a stale `(detached HEAD)` entry pointing into `/tmp`);
- the temp directory survives on disk;
- the `sh`/`sleep` child is orphaned and keeps running.

`SIGKILL` behaves the same. Because each run picks a fresh unique temp path and there is no prune/sweep of stale `agent-scaffold-checks-*` entries at startup (the only `git worktree prune` is inside the `Drop` of a run that completes normally), interrupted runs accumulate stale worktree entries indefinitely. Invariant B ("the temporary worktree is ALWAYS removed") is stated without a signal caveat (`src/checks.rs:17-18`), so this is undocumented.

Mitigating: a stale worktree at a unique temp path is harmless to the live tree (no isolation escape) and is reclaimable with `git worktree prune`. Full coverage needs a signal handler, which is a larger change; at minimum invariant B's claim should be qualified ("except on forced termination by signal") and/or the runner should `git worktree prune` and sweep leftover `agent-scaffold-checks-*` dirs at startup so leaks self-heal on the next run. Rated medium, not high, because it is inherent to Drop-based cleanup and does not corrupt or mutate the live tree.

## Finding 3 [medium] Hang vectors: inherited stdin and the `Command::output()` background-process deadlock

`run_command` (`src/checks.rs:452`) runs `sh -c <command>` with `.output()` and no timeout. Two ways a check hangs the whole run:

1. Inherited stdin. `.output()` does not redirect stdin to null; it is inherited from the parent. Reproduced: a check `command = "cat"` blocked until the parent's stdin pipe closed (5s test with a slow pipe; elapsed 5s). In an interactive terminal a stdin-reading check (a linter/formatter that defaults to reading stdin when given no path) blocks indefinitely until the user hits Ctrl-D or Ctrl-C.
2. Background / grandchild holding the captured pipe. `.output()` reads the child's stdout/stderr pipes to EOF, and any process the check backgrounds inherits those pipe write-ends. Reproduced: `command = "sleep 6 & echo started; exit 0"` returned exit 0 immediately, yet the run blocked ~6s until the backgrounded `sleep` died. A check that starts a long-lived daemon (dev server, file watcher) hangs the run for the daemon's entire lifetime, i.e. potentially forever.

Both stall the run holding the worktree open, and breaking the stall with Ctrl-C triggers Finding 2's leak. Fixes: `.stdin(Stdio::null())` so no check can block on stdin, and consider a wall-clock timeout (invariant B already lists a `budget` field, currently unused). Rated medium: within the trusted-config model, but stdin-reading and daemon-spawning checks are plausible, and the failure mode is an unbounded hang plus a leaked worktree on interrupt. Not tested.

## Finding 4 [low] Isolation is working-tree-only: shared git refs / config / objects are NOT isolated

The worktree isolates the working TREE, but shares the main repository's object store, refs, and config. A check that runs ref/config-mutating git commands therefore mutates the user's real repository. Reproduced: a lint `command = "git tag LEAKED-TAG; git config --local leak.key leaked-value; true"` left `LEAKED-TAG` in the MAIN repo's tag list and `leak.key=leaked-value` in the MAIN repo's `.git/config` after the run completed and the worktree was removed.

Invariant A as written ("never mutates the user's live working TREE") strictly still holds; this is a mutation of git metadata, not the working tree. But the module's framing ("the user's live working tree is never touched", `src/checks.rs:2-3`) reads broader than what is delivered. Trusted config, and linters/formatters do not normally run `git`, so low. Worth one sentence in the module doc scoping the guarantee to the working tree specifically.

## Finding 5 [low] A format check writing to an absolute path clobbers the live tree; `../` is safely contained

Reproduced: a format check whose `check` command is `printf 'CLOBBERED\n' > <absolute-path-into-live-repo>; exit 1` overwrote the live tracked file (`victim.txt` became `CLOBBERED`). This is the user explicitly targeting an absolute path outside the isolated cwd, i.e. self-inflicted with trusted config, so low. Invariant A's rationale ("the reformatting happens in the discardable worktree copy", `src/checks.rs:15-16`) tacitly assumes a formatter that writes relative paths; an absolute-path writer is outside that assumption.

Positive counterpart worth recording: the `../` variant is SAFE. Because the worktree lives under `std::env::temp_dir()` (`src/checks.rs:538`), a `check` writing `../ESCAPED_MARKER.txt` landed in `/tmp` (the temp-dir parent), NOT in the repo, and the live tracked file was byte-unchanged. Siting the worktree in the temp dir rather than as a sibling of the repo is a good defensive choice that defeats the relative-path escape.

## Finding 6 [low] `--dir <subdir>`: config read location and command cwd diverge

`run` reads the config at `dir/.agents/checks.toml` (`src/checks.rs:493`) but runs every check with cwd set to the worktree ROOT, which is the repository top level (`git rev-parse --show-toplevel`, `src/checks.rs:528`), and `paths` globs are matched against repo-root-relative `git ls-files` output. Reproduced: with `--dir <repo>/sub` and a config under `sub/.agents/`, a check probing its cwd reported the worktree root (repo top), and `test -f subfile.txt` (a file that exists at `sub/subfile.txt`) failed because cwd is the repo root, not `sub`.

So a subdir-scoped config's relative commands and `paths` silently operate against the repo root instead of the subdir. Since `--dir` defaults to `.` and the common invocation is at the repo root, this is an edge case; low. If subdir invocation is intended to be meaningful it should either run in the subdir's worktree-relative path or reject a subdir target; if not intended, a note that `--dir` is resolved to the repo top level would prevent surprise.

## Invariant C note [low, informational] stash-create semantics are correct and as documented

Verified `git stash create` captures both staged-but-uncommitted and unstaged tracked changes: a check requiring a file that was `git add`ed but never committed saw it in the isolated tree and passed. A clean tree correctly isolates HEAD. Untracked files are excluded, which is documented (`src/checks.rs:19-22, 304-305`) and acceptable per the increment scope. No defect. One consequence worth a sentence for users: because a skipped check never counts against success (invariant D by design), a check that depends on an untracked-only file, or a `paths` glob that under-matches, is silently skipped rather than failing, so it can mask a real problem. This is a design property, not a bug, but the hand-rolled `glob_match` widens the surface for an accidental under-match that turns into a silent skip.

## Things I tried to break and could NOT (positive confirmations)

- Core isolation holds: the in-place format-then-report check left the live tracked file byte-identical (`a_format_check_never_mutates_the_live_tree` reproduced independently); `../` writes are contained in the temp dir; the live tree is untouched across pass, fail, and format-reformat runs.
- Drop cleanup works on all covered paths: passing, failing, and format-reformat runs each leave exactly one worktree (the main one); no leftover temp dir. The guard's belt-and-suspenders (`worktree remove --force` then `remove_dir_all` then `worktree prune`) is sound. The leak is ONLY on signal (Finding 2), not on any normal return or `?` early-return (the guard is constructed immediately after a successful `worktree add`, with no `?` between add-success and guard construction, so there is no leak window).
- Concurrency / temp uniqueness: temp path is `agent-scaffold-checks-<pid>-<nanos>` (`src/checks.rs:538-542`); distinct across processes via pid, and a single CLI process runs one `run`. Two concurrent runs against the same repo both exited 0 with no leftover worktree; `git`'s own index lock serializes the concurrent `worktree add`, and `stash create` only writes objects, so the worst realistic case is a transient lock failure surfacing as a clean `WorktreeSetup` error (exit 2), not corruption. No race in the naming. No finding.
- Exit-code mapping for the git/environment cases is correct: not-a-repo = 2, no-commits = 2, malformed = 1, failing check = 1 (only the IO case is wrong, Finding 1).
- Combined-output capture is correct: stdout then stderr, each trimmed; empty output yields `(no output; exit N)`; a signal-killed check reports `signal`.
- Reserved/checkless-only configs touch git not at all and work in a non-repo directory (the worktree is created only when something is runnable, `src/checks.rs:507-525`).
- clippy `-D warnings` clean; source is ASCII-only; all 17 module tests pass.

## Verdict

Ship-able after addressing Finding 1 (a clear contract bug with a one-line fix and no test) and deciding on Findings 2 and 3 (either fix `stdin(null)` plus a startup prune, or explicitly document the signal/hang limitations of Drop-based cleanup). Findings 4-6 are low and can be handled with a doc sentence scoping the guarantee. The increment's central claim, that a `checks` run does not mutate the live working tree, is upheld for the checks it is designed for.
