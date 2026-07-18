# Round-2 confirming review: optional-modules increment 2c-i (`checks` subcommand)

Reviewer: claude-sonnet-4-6 (independent confirming reviewer, round 2) Scope: full branch diff `38ce9ec..7ea018c`; fix delta `76d9b83..7ea018c` Worktree used: `/tmp/claude-1000/-home-jessea-Documents-cv/b1add4df-96ab-4436-ada1-5f3542063be1/scratchpad/wt-7ea018c` at HEAD `7ea018ca`

---

## Build and test

`cargo test`: 148 passed, 0 failed, 0 ignored. `cargo clippy --all-targets`: clean, no warnings. ASCII scan (`python3` over `src/checks.rs`): 0 non-ASCII bytes.

---

## Round-1 fix confirmation

### F-M1 - exit-code mapping single-sourced on `RunError::exit_code()`

Confirmed. `RunError::exit_code()` at `src/checks.rs:260-275` maps `Parse(_)` -> 1 and all other variants (`NotARepo`, `NoCommits`, `GitUnavailable`, `WorktreeSetup`, `Io`) -> 2. The match is exhaustive (the compiler enforces this). The caller in `src/main.rs:426-433` uses a single `Err(error)` arm that calls `error.exit_code()`, so the mapping is not duplicated. The previous `Err(checks::RunError::Io(error)) => return Err(error)` arm that would have propagated `Io` errors through `io::Result` (exit 1, capital "Error:") is gone.

Test `run_error_exit_codes_split_config_from_environment` (`src/checks.rs:1068`) asserts all six variants by constructing each and calling `.exit_code()`, including `Io`. The assertions are direct `assert_eq!` calls, not pattern-only checks.

### F-L1 - absent-config note on stderr

Confirmed. `src/main.rs:440` uses `eprintln!` for "no checks config at ...; nothing to run". Previously `println!`.

### L2 - empty `paths = []` runs unscoped

Confirmed. `src/checks.rs:682` uses `check.paths.as_deref().filter(|patterns| !patterns.is_empty())`, which collapses an empty `Vec` to `None` and takes the `None => run_command(...)` arm. The old code path took `if let Some(patterns)` which would match an empty `Vec`, call `any_tracked_matches` with an empty slice (always returns false), and skip with the message "no tracked file matched paths " (trailing space from an empty join). Test `an_empty_paths_array_runs_unscoped` (`src/checks.rs:1124`) exercises this by committing a file, configuring `paths = []`, and asserting the check runs (and fails, since the command is `false`).

### L5 - `glob_match` strips leading `./`

Confirmed. `src/checks.rs:442`: `let pattern = pattern.strip_prefix("./").unwrap_or(pattern);`. Applies before the directory-prefix suffix check and before `glob_rec`, so both `./src/` and `./src/**/*.rs` are handled. `strip_prefix` returns `Option<&str>` and `unwrap_or` provides the original; this is not a panic-prone call.

### L3/L4 - new tests

Confirmed. Seven tests added in `76d9b83..7ea018c`, bringing the total from 141 to 148:

1. `malformed_config_toml_syntax_error_is_a_parse_error` - calls `parse("this is = = not valid toml [[[")` and pattern-matches `Err(ParseError::Toml(_))`. Covers a genuinely unparseable file, distinct from a schema-shaped TOML file with an unknown `kind`.
2. `run_error_exit_codes_split_config_from_environment` - asserts `.exit_code()` for all six `RunError` variants (see F-M1 above). This is the only unit test for the exit-code contract and it is sufficient.
3. `an_unreadable_config_is_an_environment_error_not_a_check_failure` - makes `.agents/checks.toml` a directory so `read_to_string` errors, asserts `RunError::Io(_)` with `.exit_code() == 2`. Correctly exercises the `?` at `src/checks.rs:611` via `From<io::Error> for RunError`.
4. `a_paths_glob_false_negative_skips_a_root_only_pattern` - commits `src/module.py`, uses pattern `*.py`, and asserts the check is skipped (not run). The command is `false`, so a false match would fail the run. Pins `*` not crossing `/`.
5. `an_empty_paths_array_runs_unscoped` - see L2 above.
6. `a_stdin_reading_check_does_not_hang` - runs `cat` as a check command (reads stdin) and asserts the check passes. The `Stdio::null()` at `src/checks.rs:561` supplies EOF. The test comment notes that in a non-tty test environment, an empty inherited pipe would also return immediately; the null redirection guarantees EOF in every environment, including a tty-inheriting invocation.
7. `a_startup_prune_reclaims_an_orphaned_runner_worktree` - plants a runner-prefixed worktree registered to the test repo (simulating a SIGKILLed run), then calls `run()` and asserts the orphan directory is gone and `git worktree list` shows only one entry. Directly tests the `prune_orphan_worktrees` path.

### F-DOC - isolation guarantee bounded

Confirmed. Module doc (`src/checks.rs:15-31`) adds a "Scope of the guarantee" paragraph stating the guarantee covers well-behaved checks using relative paths, and naming the out-of-contract cases: absolute paths, `../` escapes, and git metadata mutations. The CLI help string at `src/main.rs:288` adds "(this is isolation, not a security sandbox: a check that writes an absolute path or mutates git metadata is trusted-config self-harm and out of contract)". The `run_checks` function doc (`src/main.rs:415-422`) adds the same bound. The claims match the implementation and are self-contained.

### F-M3a/F-M3b/F-6 - stdin null, no-timeout doc, repo-top-level cwd

- Stdin null: `Stdio::null()` at `src/checks.rs:561`.
- No per-check timeout: documented at `src/checks.rs:550-553` and in the module doc (`src/checks.rs:24-26`).
- Repo-top-level cwd: documented at `src/checks.rs:28-31`.

All three confirmed.

---

## Supplementary checks

### `#[allow]` vs `#[expect]`

`src/checks.rs:131,136` use `#[allow(dead_code, reason = "...")]` for `budget` and `threshold`. Both fields are accessed only in `cfg(test)` (the test at line 793 reads `.budget` and `.threshold`). Using `#[expect]` would cause an unfulfilled-expectation warning in the test build (where the fields are not dead) while a non-test build would need `#[allow]` (where they are dead). The cfg-split `#[allow]` exception in the project conventions applies here. Correct.

### No new `unwrap`/`expect` on hostile input

Production code uses `unwrap_or` at `src/checks.rs:442` (infallible by design, for `strip_prefix` on an `Option`), which is not panic-prone. All `.unwrap()` calls in the file are inside `mod tests`. No new panic-prone calls on user-provided data.

### Glob `./` stripping regression check

The existing `glob_matches_the_documented_patterns` test (`src/checks.rs:814`) continues to pass with the `strip_prefix` line present. The stripping applies before directory-prefix detection and before `glob_rec`, so previously-passing patterns (`src/`, `**/*.py`, `src/**/*.rs`, `*.rs`, `a?c.rs`) are unaffected (none begin with `./`). No regression.

### Worktree naming and prune scoping

The runner prefix `RUNNER_PREFIX = "agent-scaffold-checks-run-"` (`src/checks.rs:74`) is distinct from the test fixture prefix `"agent-scaffold-checks-test-"` used in `scratch()` (`src/checks.rs:728`). The prune at `prune_orphan_worktrees` (`src/checks.rs:355-399`) checks `path.file_name().starts_with(RUNNER_PREFIX)` and `path.starts_with(&temp)`, so test fixture directories and non-runner worktrees are never touched. The test `a_startup_prune_reclaims_an_orphaned_runner_worktree` creates the orphan with the runner prefix and confirms it is reclaimed; a test-prefixed directory would not be.

---

## Conclusion

This is a clean round. All seven round-1 fixes are confirmed landed. No regressions found in the glob engine, exit-code dispatch, or existing tests. Build and tests are green (148/148). Clippy is clean. Docs are accurate and self-contained. No new `unwrap`/`expect` on hostile input. No new Unicode. No unsettled or deferred items re-raised.
