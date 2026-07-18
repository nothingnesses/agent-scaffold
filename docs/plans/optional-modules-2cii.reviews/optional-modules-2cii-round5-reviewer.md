# Round 5 confirming review: optional-modules sub-increment 2c-ii

Branch: `impl/inc2cii-staged-hook` Range reviewed: `cbf4a15..e101539` (full branch); R4-1 fix delta: `ff45a25..e101539` Reviewer: Claude (claude-sonnet-4-6), independent confirming round

## Summary

CLEAN ROUND. No findings.

## R4-1 fix verification

The fix landed at `e101539` and is correct.

The preview block in `src/main.rs` (around line 1051) now matches on `repo`:

- `InitPlan::Init` arm: prints the conditional label `"install  pre-commit hook (create-if-absent, if the new repo's hooks dir is inside the project)"`. This never asserts a definite install the action might skip.
- `InitPlan::SkipExists | InitPlan::None` arm: calls `resolve_hook_target` directly, the same predicate the install uses, so preview and action agree exactly for the existing-repo cases. This path is unchanged from before the fix.

The new test `scaffold_preview_for_a_fresh_repo_does_not_over_promise_the_hook` (`tests/scaffold_precommit_hook.rs`, line 88):

- Invokes `agent-scaffold scaffold --module checks --with-precommit-hook --dry-run` against a fresh non-repo directory (exercising `InitPlan::Init`).
- Asserts the preview line contains `"if the new repo's hooks dir is inside the project"` (the conditional string).
- Asserts `--dry-run` writes no `.git` dir (no repo initialised).
- This genuinely exercises the Init-case preview path and asserts the conditional form (Principle 11 satisfied).

## Build and test results

- `cargo test`: 168 tests, 0 failed (164 unit + 1 integration + 3 integration). All three tests in `tests/scaffold_precommit_hook.rs` pass, including the new one.
- `cargo clippy --all-targets -- -D warnings`: clean, no warnings.
- ASCII check on branch additions: no non-ASCII bytes in any line added by `cbf4a15..e101539`.

## Write-escape class (spot check)

Five tests covering the closed escape class all pass:

- `install_precommit_hook_does_not_escape_into_an_ancestor_repo`
- `install_precommit_hook_does_not_escape_from_a_linked_worktree`
- `install_precommit_hook_never_writes_through_a_dangling_symlink`
- `install_precommit_hook_never_clobbers_a_valid_symlink`
- `install_precommit_hook_skips_a_bare_repo`

The `hooks_dir_inside_output` predicate and lstat guard are unchanged by the R4-1 fix. The write-escape class remains closed.

## Staged isolation (spot check)

Five unit tests and one integration test covering `--staged` all pass:

- `staged_isolation_sees_the_index_not_the_working_tree`
- `staged_isolation_works_before_the_first_commit`
- `a_staged_run_with_nothing_staged_cleans_up`
- `a_staged_format_check_never_mutates_the_live_tree_or_index`
- `the_isolated_tree_reflects_unstaged_working_tree_edits`
- `checks_staged_runs_under_a_hook_environment` (integration)

## Module-free scaffold

`modules_slot_renders_empty_for_the_module_free_builtin` passes; no regression.

## Settled items not re-raised

Off-PATH skip default, F-3, F-4, and accepted design decisions are not re-raised. The write-escape class was exhaustively confirmed closed in round 4; no new evidence warrants reopening it.

## Verdict

CLEAN ROUND. The R4-1 fix is correct and complete. No regression. Increment 2c-ii is confirmed done.
