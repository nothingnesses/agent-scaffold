# Review: optional-modules-2ci, increment 2c-i

Reviewer: sonnet (independent) Diff range: 38ce9ec..76d9b83 Files: src/checks.rs (938 lines new), src/main.rs (+83 lines) Date: 2026-07-18

## Summary

The implementation is correct for all documented use cases. One medium bug (IO errors exit with code 1 instead of the documented 2) should be fixed before merge. Five low findings are below; none is blocking. No critical or high findings.

---

## Findings

### MEDIUM: IO errors exit code 1 instead of documented 2

File: `src/main.rs`, line 424 in the diff (`Err(checks::RunError::Io(error)) => return Err(error)`).

When `run_checks` returns `Err(io::Error)`, it propagates to `main() -> io::Result<()>`. Rust's `Termination` implementation prints `"Error: <os error>"` to stderr and exits with code 1. But `RunError::Io` is documented as "exit 2" in its enum comment, and Invariant D (module doc, line ~31) explicitly lists environment/IO errors as exit 2. Every other environment error in the same match arm calls `std::process::exit(2)`, making `Io` the only case that silently deviates.

Two defects in one:

- Wrong exit code: scripts checking `$?` for environment errors receive 1 instead of 2.
- Inconsistent error format: propagation lets Rust print `"Error: ..."` (capital E), while all other error arms print `"error: ..."` (lowercase, consistent with the project convention).

Fix:

```rust
Err(checks::RunError::Io(error)) => {
    eprintln!("error: {error}");
    std::process::exit(2);
}
```

---

### LOW: Absent-config note goes to stdout; `validate` uses stderr

File: `src/main.rs`, lines printing "no checks config at ...".

`run_validate` uses `eprintln!` for its absent-file note; `run_checks` uses `println!`. The `run_checks` doc comment claims this matches how `validate` treats an absent file, but the stream is wrong. When scripts capture stdout only (e.g., `agent-scaffold checks 2>/dev/null`), the absent-config note is swallowed silently.

Fix: change `println!` to `eprintln!` for the absent-config note.

---

### LOW: `paths = []` reaches `any_tracked_matches` contrary to its comment, and the skip message has a trailing space

File: `src/checks.rs`, lines 394-403 and 570-576.

The comment on `any_tracked_matches` says "An empty `patterns` never reaches here (the caller only calls this when the check declares `paths`)". That is wrong: `if let Some(patterns) = &check.paths` matches `Some(vec![])`, passing an empty slice to `any_tracked_matches`. The function always returns `Ok(false)` for an empty slice, so the check is skipped. The behavior is arguably correct (empty paths matches nothing), but:

1. The comment is factually wrong about reachability.
2. The skip message is `format!("no tracked file matched paths {}", patterns.join(", "))`, which with empty patterns produces `"no tracked file matched paths "` - a trailing space with nothing after it.

Fix: update the comment to reflect reality; format the skip message as `"no tracked file matched paths (none configured)"` when `patterns` is empty, or reject `paths = []` in the parser.

---

### LOW: Missing glob false-negative integration test

File: `src/checks.rs`, `glob_matches_the_documented_patterns` test (line ~703 in the diff).

The unit glob test covers `*.rs` not matching `src/main.rs`, confirming `*` does not cross `/`. But there is no integration test for `any_tracked_matches` verifying that a check with `paths = ["*.py"]` is SKIPPED when the repo contains only `src/module.py` (nested, not at root). The unit test covers the logic; the gap is a test that the paths-scope-skip path through `run()` fires correctly for non-`**` patterns with only nested files. `a_paths_check_with_no_matching_file_is_skipped` uses `**/*.py` with a `.rs` file, which is a different false-negative case.

Fix: add one integration test along these lines:

```rust
// paths = ["*.py"], repo has only src/module.py - no root-level .py, so skip.
fs::create_dir_all(dir.join("src")).unwrap();
fs::write(dir.join("src/module.py"), "x = 1\n").unwrap();
// config: paths = ["*.py"]
// assert: check is skipped (*.py does not cross a directory separator).
```

---

### LOW: `malformed_config_is_a_parse_error` only exercises a schema error, not a syntax error

File: `src/checks.rs`, `malformed_config_is_a_parse_error` test (line ~737 in the diff).

The test writes `kind = "nope"` (a valid-TOML, invalid-schema config) and expects `RunError::Parse`. A bare syntax error - for example an unterminated string or a broken table heading - is also `ParseError::Toml` but exercises a different TOML parser code path. Because the two go through identical Rust code (`toml::from_str` -> `map_err`), the gap is minor, but Principle 11 (test the boundary) calls for at least one syntax-broken variety.

Fix: add one `parse()` unit test with syntactically invalid TOML (e.g., `"[[check\n"`) confirming `Err(ParseError::Toml(_))` is returned.

---

### LOW: Pattern with a leading `./` silently skips all files

File: `src/checks.rs`, `glob_match` function.

If a user writes `paths = ["./src/"]` or `paths = ["./src/**"]`, the matcher never matches any path from `git ls-files`, which never includes a `./` prefix. For `"./src/"`, `strip_suffix('/')` gives prefix `"./src"`, and neither `path == "./src"` nor `path.starts_with("./src/")` can ever be true. The check is silently skipped as if no files matched. No documented example uses `./`, but the schema comment (`pack/checks.toml` line 29) shows path examples without calling out this restriction; a user who types `./src/` by habit will get a check that never runs.

Fix: either strip a leading `./` from patterns before matching, or document in the schema comment that `./`-prefixed patterns are not supported and give no match.

---

## Glob matcher stress-test

Tested by manual trace through `glob_rec` for each case:

| Pattern | Path | Expected | Actual |
| --- | --- | --- | --- |
| `**/*.rs` | `src/a.rs` | match | match (correct) |
| `**/*.rs` | `a.rs` | match | match (correct, zero-segment `**/`) |
| `**/*.rs` | `src/b/c.rs` | match | match (correct) |
| `*.py` | `a/b.py` | no match | no match (correct, `*` stops at `/`) |
| `src/**` | `src/a/b.c` | match | match (correct, bare `**` at end) |
| `**/*.rs` | `.hidden/main.rs` | match | match (correct, `**/` finds `/` at index 7) |
| literal `src/main.rs` | `src/main.rs` | match | match (char-by-char fallthrough) |
| literal `src/main.rs` | `lib/main.rs` | no match | no match (correct) |
| `paths = []` (empty) | any | skip | skip (correct behavior, wrong comment - see LOW above) |

No false positives or false negatives found for the documented patterns. The `**/` zero-segment case (`**/*.rs` matching `a.rs`) works correctly.

One structural note: the bare `**` case (not followed by `/`) scans character-by-character through `s` including `/` bytes. This means `**.rs` would match `a/b.rs` (crossing a slash), which is different from gitignore semantics. This is not a bug for documented patterns (`src/**` uses a trailing bare `**`; `**/*.rs` routes through the `**/` arm), but it is a trap for undocumented patterns.

---

## Responses to the implementer's flagged questions

**1. Exit-code split (0/1/2 vs finer):** Keep the current three-way split. Callers who want to distinguish "check failed" from "parse error" (both exit 1) can inspect stderr for the "malformed" prefix. Adding a fourth exit code (e.g., 2 = parse error, 3 = env error) would complicate the contract without clear benefit. The medium bug above (IO errors exiting 1) should be fixed, but the split itself is sound.

**2. No-commits handling (exit 2):** Exit 2 is correct. A repo with no commits is an environment prerequisite - the user must make an initial commit before `checks` can function. This is the same class as "not a git repo" (also exit 2), not a check failure. The error message ("make an initial commit so there is a tracked state to check") is clear and actionable.

**3. Hand-rolled glob vs `globset`/`glob` dependency:** Keep the hand-rolled matcher for this increment. All documented patterns work correctly (verified above). The implementation is small (55 lines), readable, has no external dependency, and is covered by the test suite. The one documented limitation (no character classes, no brace alternation) is not used by any schema example. If a user needs `paths = ["{src,lib}/**/*.rs"]` in a future increment, migrating to `globset` at that point is the right call. Adding a dependency now for patterns that do not yet exist violates Principle 2.

---

## Other checks

**`#[allow(dead_code)]` on `budget`/`threshold`:** Correct. The cfg-split reason is genuine: `#[expect(dead_code)]` would fail in the test build (the fields ARE read in `cfg(test)`) while `#[allow(dead_code)]` correctly suppresses the lint in the non-test build only. This is the accepted exception.

**`unwrap`/`expect` in production code:** None found. All fallible operations in production paths use `?` or explicit error mapping. `from_utf8_lossy` handles non-UTF8 output without panicking. The `worktree_path.to_string_lossy()` call (for non-UTF8 temp dir paths) is safe on any realistic system.

**clippy / `#[expect]` vs `#[allow]`:** The only `#[allow]` uses are the accepted cfg-split `dead_code` cases. No other `#[allow(clippy::...)]` was introduced.

**ASCII-clean:** No non-ASCII bytes, no emoji, no unicode symbols in the new code or comments.

**CHANGELOG:** Correctly deferred to increment-2 close per the plan's stated intent to write one entry at full-increment close. No entry is needed for this sub-increment.
