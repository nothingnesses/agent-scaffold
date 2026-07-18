# Triage: optional-modules increment 2c-i (`checks` subcommand)

Triager: independent adjudicator. Diff range `38ce9ec..76d9b83`. RISKY increment: needs TWO consecutive clean rounds to converge, so MUST-FIX below means "blocks a clean round".

Inputs read: both reviewer files, the full diff, `src/checks.rs` / `src/main.rs` on the branch, plan `Q-39` and "Increment (2)" (~line 516). Reproductions were done independently by BOTH reviewers (opus built the binary at `76d9b83` and drove throwaway repos; sonnet traced the glob and cross-checked streams); I confirmed the plan wording and the `validate` stream convention (`run_validate` uses `eprintln!` at `src/main.rs:446,468`; `run_checks` uses `println!` for the absent-config note).

Core isolation holds: a format check reformatting in place does not touch the live tree, `../` writes land in the temp-dir parent (not the repo), and `Drop` cleans up on every normal / failing / internal-error path. Nothing at critical or high. The findings are the boundary and contract issues below.

Verdict legend: VALID/INVALID, severity (low/medium/high/critical), owner, and MUST-FIX (blocks convergence) vs DOCUMENT/DEFER.

---

## F-M1 [dedup: opus Finding 1 + sonnet MEDIUM] `RunError::Io` exits 1, not the documented 2

- Verdict: VALID. Severity: medium. Owner: implementer. MUST-FIX.
- Reasoning: `RunError::Io`'s own doc (`src/checks.rs:227`) and invariant D (`src/checks.rs:23-25`) both say environment/IO errors exit 2, and every sibling environment variant calls `std::process::exit(2)`. But `run_checks` routes `Io` through `return Err(error)` (diff `src/main.rs:424`); `fn main() -> io::Result<()>` returning `Err` makes Rust's `Termination` print `Error: {e:?}` and exit 1. Reproduced (chmod 000 config -> exit 1). Two defects: wrong exit code (a CI script cannot distinguish an infra/permission error from a genuine check failure, both 1) and inconsistent format (`Error:` capital vs the project's `error:` lowercase). Medium, not high: only the IO/environment path is affected and nothing is corrupted, but it is a flat contract violation against the code's own documentation.
- Fix: replace the `Io` arm with `eprintln!("error: {error}"); std::process::exit(2);`, matching the sibling arms.
- Test (required, Principle 11): the finding slipped precisely because the variant-to-exit-code mapping in `run_checks` is untested. Add coverage. Proportionate option: extract a pure `fn exit_code(&RunError) -> i32` (or equivalent) and unit-test the mapping including `Io -> 2`, or drive the built binary. The code fix AND at least one exit-code test are both required to converge.

## F-M2 [opus Finding 2] Worktree leak on SIGINT/SIGKILL; no self-healing prune

- Verdict: VALID. Severity: medium. Owner: implementer. MUST-FIX (startup prune + wording), signal handler NOT required.
- Reasoning: cleanup is a `Drop` (`src/checks.rs:276-289`), which does not run on SIGINT/SIGTERM/SIGKILL. Reproduced: SIGINT during a `sleep 60` check leaves a registered stale `(detached HEAD)` worktree, an on-disk temp dir, and an orphaned child. Because each run uses a unique temp path and the only `git worktree prune` lives inside a normally-completing run's `Drop`, interrupted runs accumulate stale `agent-scaffold-checks-*` entries indefinitely. The plan `Q-39` explicitly promises the worktree is "removed when the run finishes (including on failure/interrupt)", and the code invariant B says the worktree is "ALWAYS removed when the run ends" (`src/checks.rs:23`) with no signal caveat. Both overclaim.
- Is "always removed" (invariant B) achievable? No, not literally: SIGKILL is uncatchable, so no in-process mechanism can guarantee it. A SIGINT handler would help only the catchable case and adds async-signal-safety and disposition-restore complexity. A signal handler is therefore NOT required and would not make the claim true anyway.
- Required, proportionate resolution: (a) at startup, sweep stale `agent-scaffold-checks-*` temp dirs (by naming prefix) and run `git worktree prune` so orphans self-heal on the next run; (b) soften invariant B and the module framing to acknowledge the hard-kill caveat ("removed when the run ends, and reclaimed on the next run after a forced signal termination"). Medium, not high: a stale worktree at a unique temp path never touches or corrupts the live tree and is reclaimable. An acceptable weaker resolution would be wording-softening plus a documented `git worktree prune` recovery step, but given the plan's explicit interrupt-cleanup promise and how cheap the sweep is, the self-healing prune is the minimum I will pass.

## F-M3a [opus Finding 3, part 1] Inherited stdin hangs a stdin-reading check

- Verdict: VALID. Severity: medium. Owner: implementer. MUST-FIX.
- Reasoning: `run_command` (`src/checks.rs:454`) runs `sh -c` with `.output()` and does not redirect stdin, so stdin is inherited. Reproduced: `command = "cat"` blocks until the parent's stdin closes. A linter/formatter that defaults to reading stdin when given no path blocks the run indefinitely in an interactive terminal, holding the worktree open; breaking it with Ctrl-C then triggers F-M2's leak. The fix is one line and unambiguously correct.
- Fix: `.stdin(Stdio::null())` on the check command (a check is non-interactive by contract).

## F-M3b [opus Finding 3, part 2] No per-check timeout; `Command::output()` deadlocks on a backgrounded/grandchild process

- Verdict: VALID. Severity: low. Owner: implementer (doc note now) + orchestrator (schedule the feature). DOCUMENT now, DEFER the timeout feature.
- Reasoning: `.output()` reads the child's stdout/stderr to EOF; a process the check backgrounds inherits those pipe write-ends, so the run blocks until it dies. Reproduced: `sleep 6 & echo started; exit 0` returned 0 immediately yet blocked ~6s; a daemon-spawning check blocks forever. Real, but: it needs an unusual check (spawning a persistent background process) within an already trusted-config model, and a correct fix is genuinely non-trivial (a wall-clock timeout alone does not reap grandchildren holding the pipe; it needs process-group kill / pipe handling). The `budget` field exists but is reserved for the mutation module, not this increment. Not a proportionate blocker for 2c-i.
- Resolution: add a one-line known-limitation note (a check must not spawn a persistent background process holding the output pipe; there is no per-check timeout yet). Defer the timeout feature to a later increment for the orchestrator to schedule.

## F-DOC [opus Finding 4 + Finding 5] Isolation is working-tree-only: git metadata and absolute-path writes are not sandboxed

- Verdict: VALID (both). Severity: low. Owner: implementer. MUST-FIX as documentation only (no code change).
- Reasoning: both are trusted-config self-harm reproduced by the reviewer. Finding 4: a check running `git tag .../git config --local ...` mutates the SHARED main repo, because the worktree isolates the working TREE but shares the object store, refs, and config. Finding 5: a `check` writing to an ABSOLUTE path into the live repo clobbers the live tracked file (cwd isolation only constrains relative paths; the `../` variant is safely contained in the temp-dir parent, a good defensive siting). Neither violates the `Q-39` guarantee as decided: `Q-39` and invariant A are about the live WORKING TREE and an in-place FORMATTER reformatting it, not a filesystem/process sandbox against a user who deliberately writes outside cwd or runs ref-mutating git. Checks are arbitrary user-authored shell commands; a user who writes `git tag` or an absolute-path clobber into their own `checks.toml` owns that.
- Why must-fix-as-doc: the isolation claim is the entire point of this RISKY increment, and the module framing "the user's live working tree is never touched" (`src/checks.rs:2-3`) reads broader than what is delivered. Honest invariants require bounding it. Add one or two sentences scoping the guarantee to working-tree isolation of well-behaved (relative-path) checks: the shared object store / refs / config are NOT isolated, and a check that writes outside its cwd (absolute paths) or mutates git metadata is the user's responsibility. Cheap; no code change. This is the only reason it blocks: an unbounded isolation claim on a RISKY isolation increment is itself the defect.

## F-6 [opus Finding 6] `--dir <subdir>`: config read location and command cwd diverge

- Verdict: VALID. Severity: low. Owner: implementer (doc) with an orchestrator design note. DOCUMENT; not must-fix.
- Reasoning: `run` reads the config at `dir/.agents/checks.toml` (`src/checks.rs:499`) but runs every check with cwd = worktree root = repo top level (`src/checks.rs:534-538`), and matches `paths` against repo-root-relative `git ls-files`. Reproduced: with `--dir <repo>/sub`, a check's cwd was the repo top and `test -f subfile.txt` (present at `sub/subfile.txt`) failed. Running checks at the repository top level is the CORRECT default (linters/formatters are conventionally repo-scoped and discover their own config from the root); the defect is the DIVERGENCE between where the config is located (`--dir` subdir) and where it runs (repo root). It is an edge case: `--dir` defaults to `.`, and at the repo root `dir` equals the toplevel so there is no divergence.
- Resolution: document that checks run at the repository top level regardless of `--dir` (which only locates the config, and whose `paths` are repo-root-relative). Optionally the orchestrator may later decide to reject a non-toplevel `--dir` or resolve everything subdir-relative; flag for the orchestrator but not required for convergence.

## F-L1 [sonnet LOW] Absent-config note goes to stdout; `validate` uses stderr

- Verdict: VALID. Severity: low. Owner: implementer. MUST-FIX (cheap self-consistency).
- Reasoning: verified. `run_validate` prints its absent-file notes with `eprintln!` (`src/main.rs:446,468`); `run_checks` prints "no checks config at ..." with `println!` (diff `src/main.rs`), yet its doc comment claims it matches how `validate` treats an absent file. So the code contradicts its own stated parity, and a script capturing stdout only (`checks 2>/dev/null`) silently loses the note. One-line fix. It blocks not on severity but because the code and its doc disagree; pick one side.
- Fix: change the absent-config `println!` to `eprintln!` (match `validate`). If instead stdout is intended, correct the doc comment; matching `validate` is the cheaper correct choice.

## F-L2 [sonnet LOW] `paths = []` reaches `any_tracked_matches`; wrong comment + trailing-space skip message

- Verdict: VALID. Severity: low. Owner: implementer. DOCUMENT/CLEANUP; not must-fix.
- Reasoning: `if let Some(patterns) = &check.paths` matches `Some(vec![])`, so an empty slice does reach `any_tracked_matches`, contradicting its comment "An empty `patterns` never reaches here" (`src/checks.rs:398-401`). The behavior is correct (empty -> `Ok(false)` -> skip), but the comment is factually wrong and the skip message `"no tracked file matched paths "` (empty `join`) has a dangling trailing space. Cosmetic, not harmful.
- Resolution (recommended, not blocking): correct the comment, and either special-case the empty-patterns message (`"paths is empty; nothing to match"`) or reject `paths = []` in `parse` (the latter is more in the parse-don't-validate spirit, Principle 14, since an always-skipped check is a silent config mistake). Low; fold into a cleanup pass.

## F-L3 [sonnet LOW] Missing glob false-negative integration test for a single-`*` pattern

- Verdict: VALID (test gap). Severity: low. Owner: implementer. DEFER/NICE-TO-HAVE; not must-fix.
- Reasoning: the glob logic is unit-tested (`glob_matches_the_documented_patterns` covers `*` not crossing `/`), and there is an integration skip test for `**/*.py`. The gap is an integration test that `paths = ["*.py"]` is skipped when only nested `src/module.py` exists. Worth adding for boundary assurance (Principle 11) but the underlying logic is already covered; not blocking.

## F-L4 [sonnet LOW] `malformed_config` test covers only a schema error, not a TOML syntax error

- Verdict: VALID (minor test gap). Severity: low. Owner: implementer. DEFER/NICE-TO-HAVE; not must-fix.
- Reasoning: both a bad schema (`kind = "nope"`) and a raw syntax error go through the identical `toml::from_str -> map_err(ParseError::Toml)` path, so, as the reviewer notes, the gap is minor. Add one `parse()` unit test with syntactically invalid TOML if convenient; not blocking.

## F-L5 [sonnet LOW] Leading `./` patterns never match (silent skip)

- Verdict: VALID. Severity: low. Owner: implementer. DOCUMENT (or cheap strip); not must-fix.
- Reasoning: traced and correct. `paths = ["./src/"]` yields prefix `"./src"`, which never matches `git ls-files` output (no `./` prefix), so the check is silently skipped as if nothing matched. The dangerous mode is the silent skip masking a real check, but `./` is an undocumented typo-class pattern under trusted config. Resolution: either strip a leading `./` before matching (cheap, removes the trap) or document that `./`-prefixed patterns are unsupported. Not blocking; the cheap strip is the better option if a cleanup pass happens.

## Invariant C note [opus, informational]

- No finding. `git stash create` semantics are correct and documented (captures committed + unstaged tracked changes; untracked excluded; clean tree isolates HEAD). The "a skipped check never counts against success, so an under-matching `paths` glob or an untracked-only dependency is silently skipped" observation is a documented design property, not a defect. No action.

---

## Implementer questions

1. Exit-code split (keep 0/1/2, fix `Io` to emit 2): CONFIRMED. Keep the three-way split; fix `Io` (see F-M1). A finer split (separate code for parse vs env) is not worth the added contract surface; both reviewers agree.
2. No-commits = exit 2: CONFIRMED correct. A repo with no commits is an environment prerequisite, the same class as "not a git repo" (also 2), not a check failure. The message is clear and actionable.
3. Hand-rolled glob vs `globset`: CONFIRMED keep for this increment. All documented patterns verified (sonnet's stress table found no false pos/neg); the matcher is ~55 lines, dependency-free, and adding a dep for brace/char-class patterns that do not yet exist violates Principle 2. Migrate to `globset` if/when such patterns are introduced. (One trap worth a comment: bare `**` not followed by `/` crosses `/`, unlike gitignore; not used by any documented pattern.)

---

## Convergence checklist

MUST-FIX to converge (blocks a clean round):

- F-M1: `Io` -> print `error:` and `exit(2)`; add an exit-code mapping test.
- F-M2: add a startup sweep of stale `agent-scaffold-checks-*` dirs + `git worktree prune`; soften invariant B / plan-cleanup wording for the hard-kill caveat. (No signal handler required.)
- F-M3a: `.stdin(Stdio::null())` on the check command.
- F-DOC (opus 4+5): scope the module/invariant-A wording to working-tree isolation of relative-path checks; state the shared object store/refs/config and absolute-path writes are out of contract. (Doc only.)
- F-L1: absent-config note -> `eprintln!` (or fix the doc claim); resolve the code/doc contradiction.

DOCUMENT or DEFER (not blocking):

- F-M3b (timeout / background-process deadlock): document the limitation now; defer the feature.
- F-6 (`--dir` subdir divergence): document; optional orchestrator design decision.
- F-L2 (`paths = []` comment + message), F-L3 (glob false-negative integration test), F-L4 (TOML syntax-error test), F-L5 (leading `./`): cheap cleanups / nice-to-haves, recommended not required.

Out of scope (not findings): `--staged` / `--with-precommit-hook` (2c-ii); CHANGELOG (orchestrator-owned, deferred to increment-2 close); prose line length / wrapping.
