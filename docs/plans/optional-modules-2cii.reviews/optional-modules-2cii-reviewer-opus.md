# Reviewer findings: optional-modules 2c-ii (`--staged` + `--with-precommit-hook`)

Reviewer: independent (opus), adversarial lens on never-clobber install, index isolation, executable-bit / manifest change, git-env safety. Range: `cbf4a15..49a377f` (commit `49a377f`). Built and attacked in a throwaway worktree at `49a377f`.

Verification performed: `cargo test` (156 unit + 1 integration = 157, all pass), `cargo clippy --all-targets -D warnings` (clean), ASCII-clean check over every changed file (clean, including `pack/hooks/pre-commit`). Plus binary-level adversarial runs: coherence exit code, real `git commit` gating through the installed hook, staged-vs-unstaged discrimination, `core.hooksPath`, linked worktree, bare repo, and symlink hooks.

## Summary

The core design is sound and most of the flagged attack surface holds up:

- Never-clobber for a REGULAR existing hook: byte-unchanged, reports `Exists` (confirmed empirically and by test).
- Never-clobber for a VALID symlink hook (target present): reports `Exists`, symlink and its target untouched (confirmed).
- `core.hooksPath`: `git rev-parse --git-path hooks` HONORS it and installs into the custom dir (confirmed: returns `.customhooks`, install correctly targets it). Not a bug; the doc comment's claim holds.
- Linked worktree (`.git` is a file): resolves to the shared main hooks dir and installs there correctly (confirmed).
- Coherence: `--with-precommit-hook` without `--module checks` exits 2 with a clear message, checked BEFORE any writing (nothing landed on disk). Confirmed.
- `--staged` index isolation: a real `git commit` through the installed hook blocks a STAGED violation (exit 1) and IGNORES an unstaged working-tree violation (commits). Confirmed end-to-end and by unit test `staged_isolation_sees_the_index_not_the_working_tree`.
- Git-env fix: `checks --staged` runs correctly under a real hook environment and under the synthetic `GIT_DIR`/`GIT_INDEX_FILE`/`GIT_PREFIX` env (integration test + real commit both pass). `write-tree` reads the committing index correctly. `run_command` strips the env for the `sh -c` check runner too.
- Executable bit: the installed `.git/hooks/pre-commit` and the scaffolded `.agents/hooks/pre-commit` are both mode 0o755; the manifest `executable` field defaults false, only the hook asset carries it, and the module-free / non-hook assets are unaffected. Confirmed.
- 2c-i invariants hold in both isolation modes (live tree and index unmutated, worktree cleaned up).

Findings below. No `critical` or `high` findings: I could not produce a destructive clobber of existing hook CONTENT, nor make `--staged` check the wrong content.

## Findings

### F1 (medium) Never-clobber gap: a dangling-symlink hook is misclassified as absent and written THROUGH, outside `.git/hooks/`

`src/main.rs:744` (`install_precommit_hook`): the absence check is `if hook.exists()`. `Path::exists()` FOLLOWS symlinks and stats the target, so a `.git/hooks/pre-commit` that is a symlink to a currently-missing target returns `false` and is treated as "no hook present". The code then falls through to `fs::write(&hook, PRECOMMIT_DELEGATE)` (`src/main.rs:747`), which also follows the symlink and writes the delegate content to the symlink's TARGET, i.e. a location outside `.git/hooks/`, then `make_executable` chmods that target 0o755, and it reports `Installed the pre-commit hook at ./.git/hooks/pre-commit`.

Reproduced: with `.git/hooks/pre-commit -> victim/gone` (victim/ exists, gone does not), the run wrote the full delegate script to `victim/gone` (executable) and printed "Installed ... at ./.git/hooks/pre-commit". The existing symlink hook is silently repurposed to run agent-scaffold, and content lands in an unexpected path.

This is exactly the "existing hook that is a SYMLINK" case the review brief flagged. The valid-symlink case (target exists) is safe because `exists()` is true; only the dangling case slips through. It does not clobber existing CONTENT (the target was missing), so it is not high/critical, but it is a real never-clobber correctness violation plus an out-of-directory write and a misleading success message.

Fix: detect ANY existing directory entry with an lstat, e.g. `if hook.symlink_metadata().is_ok()` instead of `hook.exists()`. That reports `Exists` for a dangling symlink and never writes through it.

### F2 (low) A hook-install write error fails the whole scaffold, contradicting the "install failure does not fail the succeeded scaffold" contract

`src/main.rs:938`: `match install_precommit_hook(&args.output_dir)? { ... }`. The `?` propagates any `io::Error` from `install_precommit_hook` (from `fs::create_dir_all` or `fs::write`). The stated contract (doc comment at `src/main.rs:733-737` and `:933-936`) is that only a non-repo output dir is swallowed (`NotARepo`, skipped with a note) so the install never fails a scaffold whose assets already landed. But a genuine write failure escapes that.

Reproduced (same root cause as F1): with `.git/hooks/pre-commit -> /no/such/parent/target`, the run printed `Wrote to . (23 changed, 0 left untouched)`, then died with `Error: Os { code: 2, kind: NotFound }` and process exit code 1, after all assets had landed. The scaffold reports asset success and then exits nonzero with a raw io error.

This is low because it needs a write to genuinely fail (unwritable hooks dir, or the F1 dangling-symlink-into-a-missing-parent case), and surfacing a real permission error is arguably acceptable. But it is inconsistent with the design's promise. If F1 is fixed with lstat, the most likely trigger disappears; consider also mapping a write error to a printed note rather than `?` so the flag never turns a landed scaffold into a hard failure.

### F3 (low) Test gap: the never-clobber suite does not cover the symlink case the design calls out

`src/main.rs` tests cover create-if-absent, never-clobber for a regular file (`install_precommit_hook_never_clobbers_an_existing_hook`), and non-repo skip. There is no test for an existing SYMLINK hook (valid or dangling), which is exactly where F1 lives. Per Principle 11 the never-clobber guarantee should be pinned for the symlink case too (assert `Exists` and byte-unchanged for both a valid and a dangling symlink). Adding that test would have caught F1.

### F4 (very low, informational) Bare-repo install is a harmless no-op reported as success

Installing into a bare repo writes `<bare>/hooks/pre-commit` and prints `Installed`. A bare repo never runs `pre-commit` (no commits happen there), so this is inert, but the "Installed" message is slightly misleading. Not worth changing; noting for completeness since the brief asked about bare repos.

## Design question: skip-with-note vs fail-closed when `agent-scaffold` is off PATH

The installed/asset hook exits 0 with a stderr note when the binary is not on PATH (`pack/hooks/pre-commit:45-49`), rather than failing the commit.

I think skip-with-note is the right default here, for these reasons:

- The hook is explicitly SECONDARY. The authoritative gate is the review-integrated checks-reviewer (stated repeatedly in the exploration section 5 and the plan). A local hook failing on a missing convenience binary should not be able to block work that the real gate already covers.
- Fail-closed would block commits for anyone who has the repo but not the tool (a collaborator, CI committing, a machine where PATH regressed) for a hook they may not know exists. The natural escape is `git commit --no-verify`, which disables ALL hooks, a strictly worse outcome than one visible skipped gate. The script's own comment makes this argument and it is correct.
- The skip is not silent: two stderr lines say the gate was skipped and how to enable it. git shows hook stderr, so it is visible and actionable.

One improvement worth offering the human: add an opt-in strict mode (e.g. an env var `AGENT_SCAFFOLD_HOOK_STRICT=1` that makes the missing-binary branch `exit 1`) for a solo developer who wants a hard local gate on their own machine. That preserves the safe default while letting someone who set the hook up deliberately choose fail-closed. Not required for this increment.

## Items checked and found clean (no finding)

- `--staged` checks the index, not the working tree, in every scenario tried (staged violation blocks, unstaged violation ignored, staged-clean-with-unstaged-dirty commits).
- The throwaway commit identity (`-c user.name/user.email`) lets `commit-tree` succeed with no configured committer; the commit is never published.
- `strip_git_env` removes `GIT_DIR`, `GIT_WORK_TREE`, `GIT_INDEX_FILE`, `GIT_PREFIX`, `GIT_COMMON_DIR`, `GIT_OBJECT_DIRECTORY` on every runner git call (via `git_command`) and on the `sh -c` check runner; a real `git commit` through the hook works, so no additional exported var breaks `worktree add`.
- Worktree cleanup and orphan prune intact in both isolation modes; no leftover worktree after staged runs (asserted by the new tests and observed).
- Executable field defaults false; only the hook asset is executable; module-free and non-hook assets byte- and mode-unchanged (existing byte-identity tests still pass).
- Coherence checked before any write; nothing lands on the incoherent path.
- ASCII-clean across all changed files including the shell script; clippy `-D warnings` clean.
