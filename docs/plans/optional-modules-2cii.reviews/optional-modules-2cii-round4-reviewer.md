# optional-modules 2c-ii, round 4 (confirming) - independent reviewer

Scope: confirm the round-3 fix (`ff45a25`) closes the pre-commit hook write-escape class. Full branch diff `cbf4a15..ff45a25`, round-3 delta `9ca7ca9..ff45a25`. Built and attacked the binary in a throwaway worktree at `ff45a25`.

## Verdict

CLEAN round for the write-escape class. I could not find any git layout that makes the install write a hook OUTSIDE the output dir or touch another repo's hooks. The general predicate `hooks_dir_inside_output` (src/main.rs:810) plus the canonicalize-nearest-existing-ancestor helper (src/main.rs:786) is sound, and the create-if-absent write path is a correct backstop for the one residual soft spot (dangling-symlink hooks dir).

One NEW, low-severity, NON-escape finding: a preview/action disagreement under a global/system `core.hooksPath` that points outside the output dir. No write escapes in that case; the action still correctly skips. Details below.

## Evidence: attacked layouts, all correctly Skipped, no escape

Ran the built binary (`scaffold --module checks --with-precommit-hook --write`) against:

- Ancestor repo, scaffold into a subdir (round-2 case): Skipped; ancestor `.git/hooks/pre-commit` never created.
- Linked worktree (round-3 case): Skipped; main repo shared `.git/hooks/pre-commit` never created; the worktree's own `worktrees/<name>/hooks` untouched.
- Worktree-of-a-worktree: Skipped; main shared hooks untouched.
- Submodule (`super/mod`, resolves to `super/.git/modules/mod/hooks`): Skipped.
- `core.hooksPath = ../evilhooks` (relative, outside): Skipped; `evilhooks/pre-commit` never created.
- `core.hooksPath = /abs/outside`: Skipped; nothing written outside.
- `core.hooksPath = sub/../../outside/hooks` (dotdot tail): Skipped.
- Symlinked `.git` -> another repo's `.git`: Skipped; other repo's hooks untouched.
- `.git` FILE (`gitdir:` pointer) pointing at an outside gitdir: Skipped.
- `GIT_DIR` env pointing at another repo: Skipped (resolved hooks outside output); neither the other repo nor the target got a hook.
- `GIT_INDEX_FILE` env set: install proceeds inside the target repo (no escape; env only affects the index, not the hooks-dir resolution).

Legit installs that MUST still work, all confirmed installing inside the output dir:

- Fresh dir, default `--vcs git` (init this run): installed at `<out>/.git/hooks/pre-commit`.
- Existing repo whose root IS the output dir: installed.
- `core.hooksPath = .myhooks` (inside): installed at `<out>/.myhooks/pre-commit`.
- `core.hooksPath = .link` where `.link` is a symlink to an inside subdir: installed inside.
- Relative `--output-dir`: installed inside.
- Trailing slash on `--output-dir`: installed inside.
- Symlinked-but-valid output dir (symlink -> real repo root): installed inside.

## The nearest-existing-ancestor false-pass class is closed

The one subtle way the guard could false-pass is a hooks dir whose path does not exist yet, so `canonical_existing_ancestor` climbs to an ancestor that IS inside the output dir while the true target is outside. I probed this directly:

- `core.hooksPath` = a DANGLING symlink to an outside dir (parent exists): the guard PASSES (nearest existing ancestor canonicalizes to the output dir), but the write path is then blocked because `fs::create_dir_all` on the symlink node fails with `EEXIST` (os error 17). Result: Skipped, no escape. src/main.rs:885-888.
- `core.hooksPath` = a DANGLING symlink whose target parent is also nonexistent: same `EEXIST` block, Skipped, no escape.

Reasoning that generalizes the empirical result: for the write to land outside the output dir, `create_dir_all(parent)`/`fs::write` must resolve outside. That resolution can only escape via (a) an EXISTING symlink or dir, which `canonicalize` in the guard resolves and rejects, or (b) a `..` component, which `canonicalize` of the existing ancestor also resolves and rejects (climbing past a `..` lands at `/` or an above-output ancestor, which fails `starts_with`), or (c) a DANGLING symlink node, which `create_dir_all` refuses with `EEXIST`/`ENOENT`. No fourth path exists, so the class is closed. `starts_with` is component-wise, so no `/foo/bar-evil` vs `/foo/bar` string-prefix confusion.

## Preview and action agreement (one low finding)

Both the write path and the preview route through `resolve_hook_target` (src/main.rs:835), EXCEPT the preview short-circuits `will_install = true` for `InitPlan::Init` without consulting it (src/main.rs:1049), because at preview time the repo does not exist yet and `resolve_hook_target` would wrongly report "not a git repository".

Finding R4-1 (low, NOT a write-escape, preview accuracy / Principle 16):

- File: src/main.rs:1048-1058 (preview short-circuit) vs src/main.rs:1078-1097 (action).
- Repro: a global or system `core.hooksPath` pointing OUTSIDE the output dir, plus a fresh output dir (default `--vcs git`, so `InitPlan::Init`). The preview prints `install  pre-commit hook (create-if-absent)`, but the actual install is Skipped (the freshly-init'd repo inherits the global `core.hooksPath`, which resolves outside the output dir). Verified with `GIT_CONFIG_GLOBAL` set to a config whose `core.hooksPath` is an outside absolute path: preview said "install", action said "skipping ... resolved git hooks directory (...) is outside ...".
- Impact: cosmetic. NO hook is written outside; the guard correctly skips. The code comment at src/main.rs:1040-1047 claims "preview and action always agree" and "the preview can never promise an install the write path then skips"; this global- hooksPath case is the one residual where that claim is inaccurate.
- Not the escape class under review; flagging for completeness. A precise preview would additionally check the resolved hooks path against the output dir even on the Init branch (the value is queryable before init), but that is optional polish.

## Prior fixes still hold

- F1 lstat never-clobber: unit tests for regular file, valid symlink, and dangling symlink all report Exists and do not write through (all green). The write path uses `symlink_metadata` (src/main.rs:892).
- F2 Skipped-not-fail: confirmed the scaffold exits 0 when the hook install is skipped (ancestor-subdir run returned exit 0); assets still land.
- `--staged` isolation and git-env stripping: `strip_git_env` removes GIT_DIR, GIT_WORK_TREE, GIT_INDEX_FILE, GIT_PREFIX, GIT_COMMON_DIR, GIT_OBJECT_DIRECTORY for every runner git call (src/checks.rs:354-362); the staged-isolation and hook-environment integration tests pass.
- The delegate hook re-resolves `git rev-parse --show-toplevel` at commit time (src/main.rs:706-710), so it locates the tracked hook from within any checkout; `PRECOMMIT_DELEGATE` is the delegating variant, not the pack's inert asset.

## New tests genuinely establish the predicate (Principle 11)

- `install_precommit_hook_does_not_escape_from_a_linked_worktree` (src/main.rs:1672): requires `Skipped` with a reason containing "outside" AND asserts the main repo's shared `.git/hooks/pre-commit` was never created. Absent the guard, `install_precommit_hook` would return `Installed` (writing into the main repo) and hit the test's `panic!` arm, so the test discriminates on the actual invariant.
- `install_precommit_hook_honours_a_custom_hooks_path_inside_the_output_dir` (src/main.rs:1724): requires `Installed` under `.myhooks/pre-commit` inside the output dir, pinning that the general guard does not over-skip a legitimate custom hooks path. My live-binary runs reproduce both behaviors independently.

## Gate checks

- `just test`: 167 green (164 unit + 1 + 2 integration), 0 failed.
- `cargo clippy --all-targets -- -D warnings`: clean (0 warning/error lines).
- Changed files are ASCII-clean (no non-ASCII bytes in the branch diff).
- Module-free rendering unchanged: `modules_slot_renders_empty_for_the_module_free_builtin` passes.

## Not re-raised (settled/dismissed, no new evidence)

Off-PATH skip default; sonnet F-3; plan-schema-line F-4. No new evidence against any.

## Conclusion

The write-escape class this increment has been chasing across rounds 1-3 is CLOSED. Round 4 finds no remaining escape. The single new item (R4-1) is a low-severity preview-accuracy nit under an unusual global `core.hooksPath`, with no safety impact. This is a clean confirming round for the risky increment.
