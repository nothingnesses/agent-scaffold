# optional-modules 2c-ii, round 2 (confirming) - reviewer opus

Independent confirming review of the RISKY hook-install increment. Full branch diff `cbf4a15..6e9359b` (fix delta `49a377f..6e9359b`). Built, tested, and attacked the binary in a throwaway worktree at `6e9359b`.

## Verdict

NOT a clean round. Round-1 fixes (F1, F2, F6/F7, sonnet F-1) all land correctly and the never-clobber/isolation cases I re-attacked hold. BUT re-attacking the install found a NEW, previously-unraised write-escape: with `--with-precommit-hook`, scaffolding into a SUBDIRECTORY of an existing git repo installs a broken hook into the ANCESTOR repo's `.git/hooks/`, outside the named output dir, and that hook then blocks every commit in the ancestor repo. Reachable through the default flow. Rated HIGH.

## Build/test status (all green)

- `cargo test`: 163 passed, 0 failed (160 unit + 1 `checks_staged_hook_env` + 2 `scaffold_precommit_hook`).
- `cargo clippy --all-targets -- -D warnings`: clean.
- ASCII-clean across all changed files (`src/main.rs`, `src/checks.rs`, `src/manifest.rs`, `pack/hooks/pre-commit`, `pack/pack.toml`, `pack/checks-guidance.md`, both new test files, `README.md`).
- Module-free scaffold excludes the hook asset (verified via the real binary: no `.agents/hooks/pre-commit` without `--module checks`; manifest test `builtin_checks_module_adds_its_five_assets` pins it).

## Round-1 fixes confirmed

- F1 (lstat never-clobber): confirmed by attack. `src/main.rs:810` uses `hook.symlink_metadata().is_ok()`. Regular file, VALID symlink, DANGLING symlink, and empty file all report `Exists` and are left byte-untouched; a dangling symlink's out-of-hooks target is NOT created (no write-through). See attack matrix below.
- F2 (`HookInstall::Skipped`, no `?`-propagation): confirmed. `src/main.rs:794` returns `HookInstall` (not `io::Result`); call site `src/main.rs:982` has no `?`. Non-repo, bare repo, and an unwritable hooks dir (chmod 000) each exit 0 with a clear note, assets already landed. No `?`-propagation of an install error remains.
- F6/F7: bare repo reported `Skipped` (`src/main.rs:798-803`), not "Installed"; the redundant "check for an existing hook" note is gone from `print_manual_hook_instructions` (`src/main.rs:829-832`).
- sonnet F-1: `executable` field documented in the README pack-format reference (`README.md:235-236`).
- The `HookInstall` refactor did not change create-if-absent semantics or the coherence exit-2 check (`src/main.rs:838-844`; e2e test `scaffold_with_precommit_hook_without_checks_module_exits_2` passes, exit 2, nothing written).

## Never-clobber / Skipped attack matrix (real binary, all confirmed)

| Attack | Result | Escape/clobber? |
| --- | --- | --- |
| Regular-file hook exists | `Exists`, byte-unchanged, exit 0 | No |
| VALID symlink hook | `Exists`, still a symlink, target byte-unchanged | No |
| DANGLING symlink (target outside `.git/hooks/`) | `Exists`, target NOT created | No write-through |
| Existing empty file | `Exists`, size stays 0 | No |
| Custom `core.hooksPath` w/ existing hook | `git rev-parse --git-path hooks` HONORS it, resolves to `myhooks`, reports `Exists`, custom hook unchanged, nothing in default `.git/hooks/` | No |
| Absent (fresh repo) | `Installed`, executable (0o755), delegates to `.agents/hooks/pre-commit` | correct |
| Non-repo (`--vcs none`) | `Skipped(... not a git repository)`, exit 0, assets landed | correct |
| Bare repo | `Skipped(... bare ...)`, exit 0, no hook written in bare hooks dir | correct |
| Unwritable hooks dir (chmod 000) | `Skipped(could not write ...: Permission denied)`, exit 0, assets landed | correct |
| Dry-run (no `--write`) | preview line only, nothing installed | correct |

## Isolation re-attack (all confirmed)

- Initial-commit (no HEAD) staged isolation via the REAL binary: repo with staged content and no HEAD, clean staged content exits 0, a staged violation exits 1, the live worktree file is unchanged, and no worktree is left behind (`git worktree list` == 1). Matches `isolation_commit` `Staged` arm (`src/checks.rs:522-555`: `write-tree` then `commit-tree` with a fixed `-c user.name/email` identity, no parent needed).
- git-env stripping under a hook env: `tests/checks_staged_hook_env.rs` genuinely sets `GIT_DIR`/`GIT_INDEX_FILE`/`GIT_PREFIX` and asserts exit 1 (staged violation) then exit 0 (clean), not exit 2 (worktree-setup error). Exercises `strip_git_env` (`src/checks.rs:354-362`). Principle 11 satisfied.
- 2c-i invariants (live tree never mutated, worktree cleanup, pid-liveness prune) unchanged by this fix delta; the staged tests assert `worktree_paths(&dir).len() == 1` cleanup.

## Tests genuinely exercise the behaviors (Principle 11)

- `install_precommit_hook_never_writes_through_a_dangling_symlink` (`src/main.rs:1485`) creates a real dangling symlink, asserts `!link.exists()` up front (proving the old `exists()` hole), then asserts `Exists` + the escaped target is not created. Genuine.
- `install_precommit_hook_never_clobbers_a_valid_symlink` (`src/main.rs:1457`): asserts still-a-symlink and target byte-unchanged. Genuine.
- `staged_isolation_works_before_the_first_commit` (`src/checks.rs:1181`) asserts `!head.status.success()` ("must have no HEAD") before running, then checks pass-on-clean and fail-on-violation. Genuine no-HEAD coverage.
- `install_precommit_hook_skips_a_bare_repo` (`src/main.rs:1527`): real `git init --bare`, asserts `Skipped(reason.contains("bare"))` and no hook file. Genuine.

## NEW finding

### H1 (high) `--with-precommit-hook` into a subdir of an existing repo installs a broken hook into the ANCESTOR repo's `.git/hooks/`, outside the output dir, and blocks all its commits

`src/main.rs:795` resolves the hooks dir with `git -C <output_dir> rev-parse --git-path hooks`, which WALKS UP the directory tree. When `output_dir` is a subdirectory that is not itself a repo but is nested inside an ancestor git repo, this resolves to the ANCESTOR's hooks dir (`../.git/hooks`), so the hook is installed OUTSIDE the named `output_dir`, into a repository the user never named.

This is reachable through the DEFAULT flow, not just `--vcs none`: the plan's own git-init policy (`docs/plans/agent-scaffold.md:282`) is that scaffolding into a subdirectory of an existing repo SKIPS repo init ("skip (exists) git repository"), so no `.git` is created in `output_dir` and the hooks-dir resolution falls through to the ancestor.

The escaped hook is also BROKEN, so it does not just misplace the hook, it breaks the ancestor repo. The delegate execs `"$(git rev-parse --show-toplevel)/.agents/hooks/pre-commit"` (`src/main.rs:710`). From the ancestor root, `--show-toplevel` is the ancestor root, but the `.agents/hooks/pre-commit` asset was dropped into `<output_dir>/.agents/`, not `<ancestor-root>/.agents/`. The exec target does not exist, so every `git commit` anywhere in the ancestor repo fails.

Reproduced end to end (default vcs):

```
mono/           <- git repo, no pre-commit hook
mono/project/   <- scaffold target (--output-dir mono/project --with-precommit-hook)

$ agent-scaffold scaffold --module checks --with-precommit-hook --write --principles default --output-dir mono/project
   skip (exists)  git repository
Installed the pre-commit hook at mono/project/../.git/hooks/pre-commit   # <- ANCESTOR .git/hooks, note the ../

# asset landed at mono/project/.agents/hooks/pre-commit
# hook installed at  mono/.git/hooks/pre-commit  (delegate target: mono/.agents/hooks/pre-commit -- MISSING)

$ (in mono, unrelated file) git commit -m try
.git/hooks/pre-commit: line 5: .../mono/.agents/hooks/pre-commit: No such file or directory
commit-exit=1     # every commit in the ancestor repo now blocked
```

Never-clobber still protects an EXISTING ancestor hook (an ancestor with a pre-commit hook reports `Exists`, unchanged), so this only fires when the ancestor has no pre-commit hook, but that is the common case, and adding checks guidance to a subdirectory of an existing project is a natural use of `--module checks --with-precommit-hook`. The only user-visible hint is the `../` in the "Installed" path, which most users will not parse as "wrote into a different repo".

Severity high: it writes an executable file outside the named output dir into an unrelated repo (the increment's own bar treats a write-escape as high/critical), and it actively blocks all commits in that repo until the user finds and deletes the hook. It does not clobber (never-clobber holds) and needs the subdir-of-a-repo layout, which is why it is high rather than critical.

Suggested fix direction (for the implementer/planner, not applied here): only install when the resolved repo toplevel equals `output_dir`, i.e. compare `git -C output_dir rev-parse --show-toplevel` against `output_dir` and return `Skipped("<output_dir> is inside repository <root>; not installing a hook outside the output directory; symlink it yourself from <root>")` when they differ. That keeps the delegate's `.agents/` path and the hook in the same repo, and never escapes the output dir.

### L1 (low, tied to H1) install preview line hardcodes `.git/hooks/pre-commit`

`src/main.rs:961` always previews `install  .git/hooks/pre-commit (create-if-absent)`, even when the resolved target is a custom `core.hooksPath` (attack showed the real install went to `myhooks/pre-commit`) or, per H1, an ancestor repo's hooks dir. The preview does not reflect the resolved hooks dir, so the plan output can disagree with where the hook actually lands. Cosmetic on its own; it also removes the one place a user might have caught H1 before writing. Fixing H1 as above makes the preview accurate for the remaining (in-output-dir) case.

## Not re-raised (settled, no new evidence)

Off-PATH skip default (fail-open, sonnet argued and opus concurred in round 1); sonnet F-3 delegate `set -eu` (disproved); plan-schema-line F-4 (orchestrator-owned at increment-2 close). `core.hooksPath` was flagged by round-1 opus as honored-not-a-bug and I confirm that holds.
