# Round-2 review: optional-modules 2c-ii (confirming round)

Reviewer: sonnet (independent confirming review) Branch: `impl/inc2cii-staged-hook` Full diff: `cbf4a15..6e9359b` Fix delta: `49a377f..6e9359b`

Build/test env: throwaway worktree at `6e9359b` under the scratchpad; no edits to main repo.

---

## Verdict: clean round

All round-1 fixes verified. No regressions found. 163 tests pass, Clippy clean, ASCII clean.

---

## Fix verification

**F1 (symlink/lstat - dangling symlink never written through)** Confirmed. `install_precommit_hook` uses `hook.symlink_metadata().is_ok()` (lstat) instead of `hook.exists()` (which follows links). The dangling-symlink test pins this with two assertions: the entry is still a symlink after the call (`link.symlink_metadata().unwrap().file_type().is_symlink()`) AND the missing target was never created (`!missing_target.exists()`). The valid-symlink test additionally asserts the target file is byte-identical after the call. Both tests are genuine, not just no-panic.

**F2 (HookInstall enum, Skipped variant carries reason)** Confirmed. `install_precommit_hook` returns `HookInstall` (not `io::Result<HookInstall>`). All error paths return `Skipped(String)` with human-readable reasons. No `?`-propagation out of the function. The caller in `run_scaffold` matches exhaustively and never propagates to `io::Result`; the scaffold returns `Ok(())` on all three variants.

**F6/F7 (bare repo reported Skipped; redundant caveat dropped from already-exists path)** Confirmed. `is_bare_repo()` detects bare repos via `git rev-parse --is-bare-repository` and returns `HookInstall::Skipped("... bare git repository ...")`. The `install_precommit_hook_skips_a_bare_repo` test asserts the reason contains "bare" and that no hook was written (`!root.join("hooks").join("pre-commit").exists()`). The `print_manual_hook_instructions` function does not include the "check for an existing hook first" caveat - that note lives in `checks-guidance.md` only, which is correct in the already-exists path (the existing hook was just reported).

**sonnet F-1 (README executable field documentation)** Confirmed. `README.md` line 239 adds `executable = true` in the pack-format asset example with an inline comment: "drop with the executable bit set (e.g. a hook script); omit or false otherwise". Self-contained, accurate (defaults to false, only for scripts). The example is in context alongside `ownership` and `render` fields, matching the actual `AssetSpec` struct in `src/manifest.rs`.

---

## New tests verified

All five new/changed tests genuinely assert specific behavioral invariants:

- `install_precommit_hook_never_clobbers_a_valid_symlink` (`src/main.rs`): Asserts the link entry is still a symlink after the call AND the target is byte-identical. Strong.
- `install_precommit_hook_never_writes_through_a_dangling_symlink` (`src/main.rs`): Asserts `!missing_target.exists()`. Directly pins the hole that `exists()` would leave open.
- `install_precommit_hook_skips_a_bare_repo` (`src/main.rs`): Asserts reason contains "bare" AND no hook was written at `hooks/pre-commit`. Covers F6.
- `staged_isolation_works_before_the_first_commit` (`src/checks.rs`): Pins the precondition with `assert!(!head.status.success())`, then proves both the clean-staged-passes and staged-violation-fails paths before any HEAD exists. Covers the primary hook scenario for a new repo.
- `tests/scaffold_precommit_hook.rs` (two tests via the real binary):
  - Coherence exit-2: asserts exit code 2, error message contains "requires --module checks", AND `!out.exists()` (nothing written). Strong.
  - Create-if-absent: asserts exit 0, "Installed the pre-commit hook" in stdout, hook exists, has shebang, delegates to `.agents/hooks/pre-commit`, and is executable (0755 on Unix). Drives the real entry point through `main`.

Additional staged-mode tests in `src/checks.rs` (`staged_isolation_sees_the_index_not_the_working_tree`, `a_staged_run_with_nothing_staged_cleans_up`, `a_staged_format_check_never_mutates_the_live_tree_or_index`) all assert specific invariants beyond no-panic.

The `tests/checks_staged_hook_env.rs` integration test drives the real binary with `GIT_DIR=.git`, `GIT_INDEX_FILE=.git/index`, `GIT_PREFIX=""` set, asserting exit 1 on a staged violation and exit 0 on clean staged content. This pins the `strip_git_env` / `git_command()` fix; an exit-2 (worktree setup failure) would catch a regression.

---

## Build and test results

Throwaway worktree at `6e9359b`, confirmed with `just test`:

```
test result: ok. 160 passed; 0 failed; 0 ignored (src/main.rs)
test result: ok. 1 passed; 0 failed; 0 ignored (checks_staged_hook_env.rs)
test result: ok. 2 passed; 0 failed; 0 ignored (scaffold_precommit_hook.rs)
```

Total: 163 tests. No failures.

`cargo clippy --all-targets -- -D warnings`: clean.

---

## Code quality checks

**HookInstall enum**: three variants (Installed, Exists, Skipped), all used, all matched exhaustively in `run_scaffold`. No dead code. Variant doc comments are clear and actionable (each explains what the caller should do or report).

**`#[allow]` vs `#[expect]`**: no new `#[allow]` annotations in the diff. The three pre-existing ones in `src/pack.rs` and `src/checks.rs` are cfg-split cases with documented reasons (a field read only in the test build; `expect` would be unfulfilled in the non-test build). Correct.

**ASCII**: all changed files clean (no non-ASCII bytes in `src/`, `tests/`, `pack/`, `README.md`).

**`git_command()` / `strip_git_env`**: used consistently for all git invocations inside the checks runner (`src/checks.rs`). The scaffold-side helpers (`hooks_dir`, `is_bare_repo` in `src/main.rs`) use raw `Command::new("git")`, which is correct - they are called in user-invoked scaffold context, not from within a hook. The `run_command` in checks.rs also strips the git env (`strip_git_env(&mut Command::new("sh"))`) so check commands that shell out to git see the isolated worktree, not the outer hook environment.

**`PRECOMMIT_DELEGATE` content**: verified with `cat -A`. The Rust `\` line continuation correctly strips leading tabs from the string literal; the installed file has no leading whitespace. The `exec "$(git rev-parse --show-toplevel)/.agents/hooks/pre-commit"` line is correct and works from any working directory.

**Staged isolation (parentless commit)**: `git commit-tree` in `Isolation::Staged` has no `-p <parent>` flag, creating a parentless root commit. This is intentional - the commit is a throwaway used only by `git worktree add --detach`. It works uniformly with or without prior commits, as the `staged_isolation_works_before_the_first_commit` test confirms.

**Module-free scaffold**: manual run confirmed 18 assets dropped, no `.agents/hooks/pre-commit`, exit 0. Byte-identical to the pre-increment output.

---

## Minor observations (not blocking findings)

**low - `print_manual_hook_instructions` in the bare-repo Skipped path** (`src/main.rs:829-832`): In the bare-repo case, the printed `ln -s ../../.agents/hooks/pre-commit .git/hooks/pre-commit` command does not apply to a bare repo (no working tree). The message is harmless and the preceding Skipped message already explains why the install was not done, but the manual instructions are not actionable in this specific case. This is cosmetic only; the behavior is otherwise correct (no hook is written, scaffold succeeds).

**low - plan preview always labels the hook line "install"** (`src/main.rs:961`): The line `install  .git/hooks/pre-commit (create-if-absent)` appears in the plan output unconditionally when `--with-precommit-hook` is requested, even if the actual install will be skipped at runtime (e.g., non-repo output dir). The `(create-if-absent)` parenthetical signals conditionality and the skip is explained on stderr after the write. This is acceptable UX, but the static "install" label can appear even when nothing is installed.

Neither observation rises to a blocking finding. Both are edge cases within documented skip behavior, and the behavioral invariants are all correct.

---

## Settled/dismissed items not re-raised

Off-PATH skip default, sonnet F-3 (DISPROVED), F-4 plan-schema-line (increment-2 close), F-6 broad-e2e (now partly addressed by the two new e2e tests): none re-raised. The new e2e tests drive the real binary through both the coherence check and the create-if-absent install path, which constitutes meaningful coverage of the entry point for this increment.
