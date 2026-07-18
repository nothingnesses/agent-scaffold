# optional-modules 2c-ii, round 3 (confirming, independent reviewer)

Scope: confirm the round-2 fix (commit `9ca7ca9`) closes the ancestor-repo write-escape without opening a new one. Re-attacked the hook install against the release binary built in a throwaway worktree at `9ca7ca9`. Reviewed the full branch diff `cbf4a15..9ca7ca9` and the round-2 delta `6e9359b..9ca7ca9` (main.rs only).

Verdict: NOT a clean round. The ancestor escape is closed, but the repo-root guard checks the wrong property and a NEW write-escape of the SAME class remains for a git LINKED WORKTREE. Rated `high`.

## Result summary

- Build/test/clippy: `cargo build --release` ok; tests 165 green (162 lib + 1 + 2 integration); `cargo clippy --all-targets -- -D warnings` clean.
- Round-2 delta touches `src/main.rs` only (guard + preview + 2 tests); the 2c-i invariants, `--staged` isolation, and git-env stripping are unchanged since round 2 confirmed them.
- ASCII-clean: `src/main.rs`, `pack/hooks/pre-commit` clean.
- Module-free scaffold: no `checks.toml`, no `.agents/hooks`, no checks content in `AGENTS.md` (the lone "checks" hit is unrelated prose). Consistent with prior byte-identity confirmation.

## Finding H1 (high, residual write-escape): the repo-root guard misses a linked worktree

`src/main.rs:792` (`is_repo_root`), `src/main.rs:844` (the guard call), `src/main.rs:744` (`hooks_dir`).

The guard decides "install here" by comparing canonicalized `git -C <output_dir> rev-parse --show-toplevel` to canonicalized `output_dir`. For a git LINKED worktree (created by `git worktree add`, whose `.git` is a FILE), `--show-toplevel` returns the linked worktree's OWN directory, so it equals `output_dir` and the guard passes. But git shares hooks across worktrees: `git rev-parse --git-path hooks` (what `hooks_dir` uses) resolves to the MAIN repo's `.git/hooks`, which is OUTSIDE `output_dir`. So the install writes the delegate hook into the MAIN repo, a repo the user never named, and the delegate is broken there (the main repo has no `.agents/hooks/pre-commit`). This is the exact round-2 symptom, reached by a different path the guard does not cover.

Reproduced end-to-end with the `9ca7ca9` release binary:

```
git init mainrepo; (cd mainrepo && git commit --allow-empty -m init)
git -C mainrepo worktree add ../linkedwt
agent-scaffold scaffold --module checks --with-precommit-hook --write \
  --principles default --output-dir linkedwt
# stdout: "install  pre-commit hook (create-if-absent)"
#         "Installed the pre-commit hook at .../mainrepo/.git/hooks/pre-commit."
# -> the hook landed in mainrepo/.git/hooks (OUTSIDE output_dir linkedwt)
# mainrepo/.agents/hooks/pre-commit does NOT exist (mainrepo was not scaffolded)

cd mainrepo && echo x > f && git add f && git commit -m x
# .git/hooks/pre-commit: line 5: .../mainrepo/.agents/hooks/pre-commit: No such file or directory
# git commit exit code = 1   -> commits in the MAIN repo are BLOCKED
```

The main repo (and every sibling worktree lacking the scaffolded assets) can no longer commit, which is the HIGH write-escape round 2 was meant to eliminate.

Why the guard mismatches its own intent: the plan's git-init policy the fix mirrors is "a dir that is not its own new repo gets no hook of its own." `init_plan` already treats a linked worktree as `SkipExists` (the scaffold prints "skip (exists) git repository" for it), i.e. NOT its own new repo, yet `is_repo_root` returns true and the hook installs anyway. The guard tests toplevel IDENTITY, but the property that actually matters is "the hooks dir git will write to is inside / owned solely by `output_dir`."

Suggested fix (reviewer, not applied): guard on the resolved hooks dir rather than the toplevel, e.g. require canonicalized `hooks_dir(output_dir)` to be under canonicalized `output_dir` (equivalently, reject when `git rev-parse --git-common-dir` resolves outside `output_dir`, which distinguishes a linked worktree from a real repo root). That single check subsumes both the ancestor case and this linked-worktree case. The preview at `src/main.rs:1014-1015` should use the same predicate so it does not mispreview "install".

Test gap (Principle 11): the two new tests (`install_precommit_hook_does_not_escape_into_an_ancestor_repo` at `src/main.rs:1607`, `install_precommit_hook_installs_at_the_repo_root` at `src/main.rs:1635`) genuinely establish the ancestor guard and the repo-root happy path, but neither covers a linked worktree, so the suite passes while the escape stands. A regression test should scaffold into a linked worktree and assert the main repo's `.git/hooks/pre-commit` stays absent.

## What the fix DOES close (attacks that now pass correctly)

- Ancestor escape, DEFAULT vcs: `--output-dir <subdir-of-existing-repo>` reports `skip  pre-commit hook (output dir is not its own git repo root)` and the ancestor's `.git/hooks/pre-commit` stays absent; exit 0. No nested `.git` in the subdir.
- Ancestor escape, `--vcs none`: same, Skipped with the explaining reason; ancestor untouched.
- Normal case, fresh dir + default init-vcs: previews and reports `install`; executable delegate written at `<out>/.git/hooks/pre-commit`.
- Normal case, existing repo whose root IS the output dir: installs correctly.
- Symlinked output dir (repo behind a symlink): `canonicalize` resolves both sides; installs into the real repo, no false skip.
- Relative `--output-dir`, trailing slash, and repo root reached via `..`: all install at the true repo root, no false skip.
- `core.hooksPath` set to a custom dir at the repo root: `--git-path hooks` honours it; the hook installs into `<out>/.githooks/pre-commit` (inside the output dir), and the never-clobber lstat still applies to that resolved path. Correct, no issue.

## Prior fixes re-checked, still holding

- F1 lstat never-clobber via the binary: a DANGLING symlink in `.git/hooks/pre-commit` is reported `Exists` and left in place; the delegate is NOT written through the link to the dangling target. Regular/valid-symlink/empty covered by the (passing) unit tests.
- F2 Skipped-not-fail via the binary: unwritable hooks dir -> `Skipped` (write permission denied), exit 0, assets still landed. Non-repo and bare-repo covered by passing tests.
- Preview accuracy (L1): for the ancestor and `--vcs none` subdir cases the preview line reads `skip ... (output dir is not its own git repo root)`, matching the post-write report. (For the linked-worktree case in H1 the preview wrongly reads `install`, same root cause.)

## Not re-raised (settled)

Off-PATH skip default, sonnet F-3, plan-schema-line F-4: no new evidence, left as settled.
