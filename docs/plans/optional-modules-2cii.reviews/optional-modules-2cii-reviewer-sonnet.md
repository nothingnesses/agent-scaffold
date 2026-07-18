# Review: optional-modules increment 2c-ii

Reviewer: claude-sonnet-4-6 (independent) Diff range: cbf4a15..49a377f Date: 2026-07-18

Files inspected: `pack/hooks/pre-commit`, `pack/checks-guidance.md`, `pack/pack.toml`, `src/checks.rs`, `src/main.rs`, `src/manifest.rs`, `tests/checks_staged_hook_env.rs`, `README.md`, `docs/plans/agent-scaffold.md` (pack-format section), `docs/plans/checks-module-config.explorations/checks-config-A.md` (section 5).

shellcheck run: both `pack/hooks/pre-commit` and the `PRECOMMIT_DELEGATE` constant (reconstructed as a .sh file) passed shellcheck 0.11.0 clean.

---

## OFF-PATH DEFAULT: recommendation

The skip-with-note (exit 0) is the correct design. Concrete reasoning:

The checks-reviewer is the authoritative gate and runs on every PR review cycle, independent of any local tool installation. Failing the commit on a missing binary would reliably produce `git commit --no-verify` habits, which bypass ALL hooks in the repo, not just this one. That outcome is strictly worse than a transparent skip of a secondary backstop. The two stderr messages are specific ("install agent-scaffold to run .agents/checks.toml on commit") and actionable.

One limitation worth noting for the orchestrator: stderr output from hooks is invisible in some GUI git clients (GitKraken, VS Code Source Control panel, Tower). A developer using only a GUI client might commit repeatedly without ever seeing the note. This does not change the recommendation (the checks-reviewer gate is still authoritative), but it is worth a sentence in the guidance saying the skip note goes to stderr and may not appear in GUI clients. Not a code change, just a guidance sentence.

---

## Findings

### F-1 (medium): README pack-format section does not document `executable`

`README.md`, lines 222-228. The README is the primary user-facing reference for pack authors using `--template`. Its `[[asset]]` example shows `source`, `dest`, `ownership`, and `render`, and the surrounding prose names those fields. The new `executable = true` field is not mentioned, so a pack author wanting to ship their own executable asset has no user-facing documentation. The `pack/pack.toml` comment covers it for the built-in pack, and `src/manifest.rs` has field-level Rust docs, but neither is user-facing for external pack authors.

Fix: add a commented line to the README's `[[asset]]` example and one sentence in the surrounding prose:

```toml
executable = true     # optional; set the executable bit on Unix (default false)
```

---

### F-2 (medium): No test for `Staged` mode in an empty repo (initial-commit scenario)

`src/checks.rs`, `Isolation::Staged` arm of `isolation_commit`. The module-level doc claims the staged path "does not need a prior commit: an index with staged content produces a tree even in a repository with no `HEAD`." That claim is correct in theory (`git write-tree` + `git commit-tree` with no `-p` works on a bare object store), but no test exercises it. Pre-commit hooks DO run for initial commits (the very first `git commit` on a new repo), so this is a real scenario, not a corner case. If `git worktree add --detach` rejects a root-parentless commit object on some git version, the failure would be a mysterious exit-2 on a user's first commit.

The existing `an_empty_repo_with_no_commits_errors` test is `WorkingTree` only (`run(&dir, Isolation::WorkingTree)`). No staged-mode variant exists that stages files in an empty repo and verifies the run completes without error.

Fix: add a test `a_staged_run_on_an_empty_repo_with_staged_files_succeeds`: init repo, stage a file (no commit), write a config, call `run(&dir, Isolation::Staged)`, assert no error and the check passes.

---

### F-3 (low): `PRECOMMIT_DELEGATE` (the installed hook) lacks `set -eu`

`src/main.rs`, lines 694-712 (the `PRECOMMIT_DELEGATE` constant). The installed `.git/hooks/pre-commit` contains:

```sh
#!/bin/sh
# ...comments...
exec "$(git rev-parse --show-toplevel)/.agents/hooks/pre-commit"
```

There is no `set -eu`. In POSIX sh, if `exec` fails (because the target file is missing or not executable, e.g. the user deleted `.agents/hooks/pre-commit` after installation), the shell continues to the implicit end of the script and exits 0. This silently skips the checks gate. With `set -e`, a failed `exec` would cause a non-zero exit, which correctly fails the commit.

In practice `exec` succeeds because the tracked hook is a reference asset re-scaffolded by `agent-scaffold`, but the silent-pass fallback is contrary to the "fail loudly on unexpected conditions" principle.

Fix: add `set -eu` after the shebang in the `PRECOMMIT_DELEGATE` string. Also add it to the constant's test assertion: `assert!(body.contains("set -eu"), ...)`.

---

### F-4 (low): Plan schema description missing `executable`

`docs/plans/agent-scaffold.md`, line 64. The pack-format description reads:

```
`pack.toml` has `[[asset]]` entries `{ source, dest, ownership = "reference"|"working",
render = true|false, module? }`
```

`executable` is not listed. This is the internal schema reference that is updated whenever fields are added; it was updated for `module` and `render` but not for this new field.

---

### F-5 (low): `print_manual_hook_instructions` "check for existing" note is

confusing in the `Exists` branch

`src/main.rs`, around line 950. When `install_precommit_hook` returns `HookInstall::Exists`, the caller already printed "A pre-commit hook already exists at <path>; leaving it untouched." Then `print_manual_hook_instructions` appends: "(check for an existing .git/hooks/pre-commit first)."

That trailing note is redundant (the user was just told one exists) and could read as contradictory. The `NotARepo` branch calls the same helper and the note is appropriate there. Consider either: (a) splitting the helper into two versions, or (b) removing the "check first" sentence from the shared helper and putting it only in the guidance doc, or (c) using a different phrasing that is coherent in both contexts, e.g. "If you later remove the existing hook, the one-liner above will activate the scaffolded gate."

---

### F-6 (low): No end-to-end `run_scaffold` test for `--with-precommit-hook`

`src/main.rs`, tests module. The unit tests cover `install_precommit_hook` and `precommit_coherent` in isolation. But the code paths inside `run_scaffold` that are guarded by `args.with_precommit_hook` are not tested end-to-end: the coherence `process::exit(2)` branch, the preview-line print (dry-run path), the `HookInstall::Installed` print, the `HookInstall::Exists` print + instructions, and the `HookInstall::NotARepo` print + instructions. A test that calls `run_scaffold` directly (or via `main()` through `Command::new(env!("CARGO_BIN_EXE_..."))`) with `--module checks --with-precommit-hook` and asserts the hook file is created and executable would close this gap.

---

## No findings at these levels

The following areas were specifically scrutinized and are clean:

- **Shell correctness and portability** (`pack/hooks/pre-commit`): `#!/bin/sh`, `set -eu`, `if ! command -v ... >/dev/null 2>&1` is correct under `set -e` (condition context exempts the command from errexit), `exec` delegation is correct. shellcheck 0.11.0 is clean.
- **`set -e` / `set -u` pitfalls in the hook**: no variable references so `set -u` is inert; the `if !` condition correctly exempts `command -v` from `set -e`. No word-splitting issues (no unquoted expansions).
- **`strip_git_env` completeness and consistency**: all production git invocations in `src/checks.rs` go through `git_command()` or the `git()` helper (which calls `git_command()`); the `sh` runner for check commands also strips via `strip_git_env`. The `WorktreeGuard::drop` impl was correctly updated to use `git_command()`. Test helpers use bare `Command::new("git")` appropriately (not a hook path).
- **Symlink path correctness**: `ln -s ../../.agents/hooks/pre-commit .git/hooks/pre-commit` - from `.git/hooks/`, two `../` levels reach the repo root, so the target is `<root>/.agents/hooks/pre-commit`. Correct.
- **Guidance docs (`pack/checks-guidance.md`)**: both activation paths are clearly documented (symlink one-liner with "check for an existing hook first" caveat, and `--with-precommit-hook`); the symlink path is correct; the create-if-absent behaviour is described; Principle 20 self-contained. The paragraph is clear.
- **Coherence error message** (`--with-precommit-hook without --module checks`): clear, actionable, exits 2.
- **Never-clobber semantics**: `hook.exists()` guard correctly returns `Exists` without writing; the unit test `install_precommit_hook_never_clobbers_an_existing_hook` pins it at the byte level.
- **`executable` manifest field design**: default false, `#[serde(default)]`, clean parse-don't-validate. `make_executable` sets `0o755` absolutely (correct for a newly written file); the `#[cfg(not(unix))]` no-op is noted in its doc comment as a platform limitation. `pack/pack.toml` comment documents the field adequately for the built-in pack.
- **`PRECOMMIT_DELEGATE` content**: Rust's string continuation escape (`\` + newline) strips leading tab whitespace, so the installed file has no spurious tab indentation. `git rev-parse --show-toplevel` is correct in a hook context (CWD is repo root, git resolves correctly). The double-quoted `exec` handles paths with spaces.
- **Staged isolation correctness**: `git write-tree` captures index content; `git commit-tree` without `-p` creates a root commit (correct, the throwaway commit is never published and will be GC'd); stripping `GIT_INDEX_FILE` means `git write-tree -C repo` reads `<repo>/.git/index`, which in a hook context is exactly the index being committed (correct, as the code comment explains).
- **`Isolation::Staged` tests**: three tests covering index-not-working-tree, clean-index cleanup, and format isolation (mutation of the worktree copy does not reach the live index).
- **Regression test (`checks_staged_hook_env.rs`)**: pins the key failure mode (`GIT_DIR=.git` + `GIT_INDEX_FILE=.git/index` leaking into `git worktree add`); covers both the fail (staged violation, expect exit 1) and pass (clean staged content, expect exit 0) arms. Uses `env!("CARGO_BIN_EXE_agent-scaffold")` to exercise the real binary.
- **`#[allow]` vs `#[expect]`**: no `#[allow(lint)]` in the diff.
- **ASCII-clean**: no non-ASCII characters in any changed file.
- **CHANGELOG note**: increment-2 CHANGELOG entry is deferred to increment-2 close (the planned single-entry-at-close approach). Its absence here is not a finding.

---

## Summary table

| ID | Severity | Area | File |
| --- | --- | --- | --- |
| F-1 | medium | README missing `executable` field docs | `README.md:222` |
| F-2 | medium | No test for `Staged` + empty repo (init) | `src/checks.rs` tests |
| F-3 | low | Delegate hook lacks `set -eu` | `src/main.rs:694` (PRECOMMIT_DELEGATE) |
| F-4 | low | Plan schema missing `executable` | `docs/plans/agent-scaffold.md:64` |
| F-5 | low | Redundant "check first" note in `Exists` | `src/main.rs` (run_scaffold) |
| F-6 | low | No end-to-end `run_scaffold` hook test | `src/main.rs` tests |

No high or critical findings.
