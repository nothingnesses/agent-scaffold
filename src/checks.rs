//! The `checks` subcommand: read a project's `.agents/checks.toml`, run its lint
//! and format checks, and report results, executing entirely inside a SEPARATE,
//! TEMPORARY git worktree so that a check running a relative-path linter or
//! formatter cannot mutate the live working tree.
//!
//! This is the runner half of the deterministic-checks module (`--module checks`
//! ships the config schema and the reviewer role; this reads that same schema).
//! The isolation is the point (`Q-39`, human-decided): treefmt v2 (this project's
//! `nix fmt`) has no non-mutating dry-run, so a `kind = "format"` check formats in
//! place and then reports; running it against the live tree would clobber the
//! user's work. Instead every check runs in a throwaway worktree checked out from
//! the current tracked working-tree state, which is removed when the run ends,
//! including on failure or error.
//!
//! Scope of the guarantee (stated honestly for a risky increment). The isolation
//! makes a well-behaved check that operates on RELATIVE paths safe: it runs in the
//! throwaway worktree, so its writes land there and are discarded. It is NOT a
//! security sandbox. A check command that writes an ABSOLUTE path, escapes the
//! temp dir with `../`, or mutates shared git metadata (`git tag`, `git config
//! --local`, `git worktree` state) can still reach outside the worktree; that is
//! trusted-config self-harm and out of contract, since `.agents/checks.toml` is
//! user-authored (Principle 21 draws the trust boundary at external input, and the
//! config is not external). Two more known limitations: there is no per-check
//! timeout yet (a possible future addition; not built here), and a check must not
//! spawn a backgrounded or daemon process that outlives it while holding the
//! captured output pipe, which would make the run hang waiting on that pipe.
//!
//! Commands run at the repository TOP LEVEL (checks are repo-scoped), even when
//! `--dir` points at a subdirectory: a subdirectory-relative command or `paths`
//! glob still sees the repository root, matching how a repo-wide lint or format
//! gate is meant to work.
//!
//! Invariants this module pins (see the tests):
//! - A. A run never mutates the user's live working tree, even when a format check
//!   reformats files (the reformatting happens in the discardable worktree copy).
//!   This holds for the relative-path case the guarantee is bounded to above.
//! - B. The temporary worktree is removed when the run ends normally or on an
//!   internal error, via a `Drop` cleanup guard. A hard kill (SIGKILL) cannot run
//!   `Drop`, so it can orphan a worktree and its temp directory; the next run
//!   reclaims such orphans with a startup prune (see `prune_orphan_worktrees`), so
//!   "always removed" holds across runs rather than unconditionally within one.
//! - C. The isolated content reflects the state the run's mode selects. A plain
//!   run (`Isolation::WorkingTree`) isolates the current tracked WORKING-TREE state
//!   (committed plus unstaged tracked modifications), captured with `git stash
//!   create` (or `HEAD` when the tree is clean); untracked files are not included,
//!   and an empty repository with no commits is rejected with a clear error. A
//!   `--staged` run (`Isolation::Staged`) isolates the INDEX content via `git
//!   write-tree` plus `git commit-tree`, so it sees exactly what a commit would
//!   record and never sees unstaged working-tree edits.
//! - D. The run succeeds (exit 0) iff every check that ran passed; a failing check
//!   makes it exit 1. Usage and environment errors (not a git repo, git missing,
//!   worktree setup failing, an unreadable config) exit 2; a malformed config
//!   exits 1.

use {
	serde::Deserialize,
	std::{
		fmt,
		fs,
		io,
		path::{
			Path,
			PathBuf,
		},
		process::{
			Command,
			Stdio,
		},
	},
};

/// The file-name prefix of a runner's temporary worktree directory under the
/// system temp dir: `agent-scaffold-checks-run-{pid}-{nanos}`. The startup prune
/// (see `prune_orphan_worktrees`) matches this prefix among a repo's registered
/// worktrees to identify a reclaimable runner orphan; the `-run-` segment keeps it
/// distinct from the test fixtures' `agent-scaffold-checks-test-` prefix so the
/// two never collide.
const RUNNER_PREFIX: &str = "agent-scaffold-checks-run-";

/// The kind of a check: a closed enum (parse-don't-validate), so a `kind` value
/// outside this set is rejected at the config boundary rather than mishandled
/// later. `lint` and `format` run in this increment; `test` and `mutation` are
/// owned here as schema but belong to the later test modules, so they are skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
	/// A read-only linter: its `command` detects violations without modifying files.
	Lint,
	/// A formatter: its `command` applies formatting (a writer action, not run here);
	/// its optional `check` is the check-mode command this runner executes.
	Format,
	/// A test suite (reserved for the test-driven module; skipped here).
	Test,
	/// A mutation-testing run (reserved for the mutation module; skipped here).
	Mutation,
}

impl Kind {
	/// The on-disk spelling, for report lines.
	pub fn as_str(self) -> &'static str {
		match self {
			Kind::Lint => "lint",
			Kind::Format => "format",
			Kind::Test => "test",
			Kind::Mutation => "mutation",
		}
	}
}

/// One `[[check]]` entry from `.agents/checks.toml`. Mirrors the schema the
/// shipped `pack/checks.toml` documents; unknown keys are tolerated for
/// forward-compatibility, matching the pack manifest loader.
#[derive(Debug, Clone, Deserialize)]
pub struct Check {
	/// A unique id, shown in report lines and findings.
	pub name: String,
	/// The check's kind (closed enum), which selects how it is run.
	pub kind: Kind,
	/// The primary command. For `lint` this is the detect command run here; for
	/// `format` it is the apply command (a writer action) and is NOT run here.
	pub command: String,
	/// The optional secondary command. For `format` it is the check-mode command
	/// this runner executes (a dry-run where the formatter supports one, or an
	/// in-place-then-report command otherwise, which is safe in the worktree copy).
	#[serde(default)]
	pub check: Option<String>,
	/// Optional globs scoping the check to matching files, so in a polyglot repo a
	/// check runs only over its own language.
	#[serde(default)]
	pub paths: Option<Vec<String>>,
	/// Optional wall-clock budget (mutation only). Parsed but unused in this
	/// increment: `allow` (not `expect`) because it is read only under `cfg(test)`,
	/// so the non-test build would otherwise report it dead while the test build
	/// would report the expectation unfulfilled.
	#[allow(dead_code, reason = "parsed for the schema; used by the later mutation module")]
	#[serde(default)]
	pub budget: Option<String>,
	/// Optional max surviving mutants (mutation only). Parsed but unused in this
	/// increment; `allow` for the same cfg-split reason as `budget` above.
	#[allow(dead_code, reason = "parsed for the schema; used by the later mutation module")]
	#[serde(default)]
	pub threshold: Option<i64>,
}

/// The on-disk shape of `.agents/checks.toml`: a flat array of `[[check]]` tables.
#[derive(Debug, Deserialize)]
struct ChecksFile {
	#[serde(default)]
	check: Vec<Check>,
}

/// A malformed `.agents/checks.toml`: either a TOML/schema parse failure (an
/// unknown `kind`, a missing required field, a wrong type) or a duplicate check
/// name (names must be unique, since they identify a check in findings).
#[derive(Debug)]
pub enum ParseError {
	/// The file was not valid TOML, or a `[[check]]` did not match the schema.
	Toml(toml::de::Error),
	/// Two `[[check]]` entries share a `name`.
	DuplicateName(String),
}

impl fmt::Display for ParseError {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			ParseError::Toml(error) => write!(f, "{error}"),
			ParseError::DuplicateName(name) =>
				write!(f, "duplicate check name `{name}` (each [[check]] name must be unique)"),
		}
	}
}

impl std::error::Error for ParseError {}

/// Parse `.agents/checks.toml` contents into the check list, rejecting a malformed
/// schema and duplicate names at the boundary (Principle 14, parse-don't-validate).
pub fn parse(contents: &str) -> Result<Vec<Check>, ParseError> {
	let file: ChecksFile = toml::from_str(contents).map_err(ParseError::Toml)?;
	let mut seen: Vec<&str> = Vec::new();
	for check in &file.check {
		if seen.contains(&check.name.as_str()) {
			return Err(ParseError::DuplicateName(check.name.clone()));
		}
		seen.push(&check.name);
	}
	Ok(file.check)
}

/// What became of one check in a run.
#[derive(Debug)]
pub enum CheckStatus {
	/// The command exited 0.
	Passed,
	/// The command exited non-zero; carries its combined stdout+stderr.
	Failed(String),
	/// The check did not run; carries the reason (a reserved kind, a format check
	/// with no `check` command, or no tracked file matched its `paths`).
	Skipped(String),
}

/// One check's outcome in a run: its name and kind plus what became of it.
#[derive(Debug)]
pub struct CheckResult {
	/// The check's `name`.
	pub name: String,
	/// The check's `kind`.
	pub kind: Kind,
	/// What became of the check.
	pub status: CheckStatus,
}

/// The result of a `checks` run: the per-check outcomes and whether a config was
/// present at all (an absent config is not a failure; there is simply nothing to
/// run). Printing and exit-code selection are the caller's job, so the run itself
/// is testable without capturing process exit.
#[derive(Debug)]
pub struct Report {
	/// One entry per configured check, in config order.
	pub results: Vec<CheckResult>,
	/// Whether `.agents/checks.toml` existed; `false` means nothing was run.
	pub config_present: bool,
}

impl Report {
	/// Whether every check that actually ran passed. Skipped checks do not count
	/// against success; a run with no failing checks succeeds (Invariant D).
	pub fn success(&self) -> bool {
		!self.results.iter().any(|result| matches!(result.status, CheckStatus::Failed(_)))
	}

	/// Whether any check actually ran (passed or failed), as opposed to all being
	/// skipped or the config being absent.
	pub fn ran_any(&self) -> bool {
		self.results
			.iter()
			.any(|result| matches!(result.status, CheckStatus::Passed | CheckStatus::Failed(_)))
	}
}

/// An error that prevented the run from completing (as distinct from a check that
/// ran and failed, which is a `CheckStatus::Failed`, not a `RunError`). The caller
/// maps each to an exit code: a malformed config is exit 1, everything else is an
/// environment/usage error (exit 2).
#[derive(Debug)]
pub enum RunError {
	/// The config was present but malformed (exit 1).
	Parse(ParseError),
	/// The target directory is not inside a git repository (exit 2).
	NotARepo(PathBuf),
	/// The repository has no commits and a clean tree, so there is nothing to
	/// isolate (exit 2).
	NoCommits,
	/// The `git` binary was not found (exit 2).
	GitUnavailable,
	/// A git operation setting up the worktree failed; carries a message (exit 2).
	WorktreeSetup(String),
	/// An underlying IO error, for example an unreadable config (exit 2).
	Io(io::Error),
}

impl RunError {
	/// The process exit code this error maps to, single-sourced here so the CLI and
	/// the tests agree: a malformed config is a problem with the project (exit 1,
	/// the same code a failing check yields), and every other variant is an
	/// environment or usage error (exit 2), including an unreadable config (`Io`),
	/// which must NOT propagate as the default `io::Result` exit 1 (Invariant D).
	pub fn exit_code(&self) -> i32 {
		match self {
			RunError::Parse(_) => 1,
			RunError::NotARepo(_)
			| RunError::NoCommits
			| RunError::GitUnavailable
			| RunError::WorktreeSetup(_)
			| RunError::Io(_) => 2,
		}
	}
}

impl fmt::Display for RunError {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			RunError::Parse(error) => write!(f, "malformed .agents/checks.toml: {error}"),
			RunError::NotARepo(dir) => write!(
				f,
				"{} is not inside a git repository; `checks` isolates the run in a temporary \
				 worktree and so needs a git repository",
				dir.display()
			),
			RunError::NoCommits => write!(
				f,
				"the repository has no commits; make an initial commit so there is a tracked state \
				 to check"
			),
			RunError::GitUnavailable =>
				write!(f, "the `git` binary was not found; `checks` needs git to isolate the run"),
			RunError::WorktreeSetup(message) =>
				write!(f, "could not set up the isolation worktree: {message}"),
			RunError::Io(error) => write!(f, "{error}"),
		}
	}
}

impl From<io::Error> for RunError {
	fn from(error: io::Error) -> Self {
		RunError::Io(error)
	}
}

/// A temporary git worktree whose `Drop` removes it, so the worktree is cleaned
/// up when a run ends normally, on a failing check, or on an early return via `?`
/// (Invariant B). Cleanup is best-effort and belt-and-suspenders: it asks git to
/// remove the worktree, removes the directory if it survives, then prunes the
/// stale admin entry. `Drop` cannot run on a hard kill (SIGKILL), which is why the
/// runner also does a startup prune (see `prune_orphan_worktrees`) to reclaim a
/// worktree orphaned by a prior killed run.
struct WorktreeGuard {
	/// The main repository's top level (the `-C` target for git worktree ops).
	repo: PathBuf,
	/// The temporary worktree path under the system temp directory.
	path: PathBuf,
}

impl Drop for WorktreeGuard {
	fn drop(&mut self) {
		let _ = git_command()
			.arg("-C")
			.arg(&self.repo)
			.args(["worktree", "remove", "--force"])
			.arg(&self.path)
			.output();
		// If git left the directory behind (or never registered it), remove it, then
		// prune the admin entry so `git worktree list` is clean.
		let _ = fs::remove_dir_all(&self.path);
		let _ = git_command().arg("-C").arg(&self.repo).args(["worktree", "prune"]).output();
	}
}

/// Strip the git environment variables a parent process may have set, so a child
/// git command acts on the repository through `-C`/`current_dir` alone. This
/// matters most for the pre-commit hook path: git runs a hook with `GIT_DIR`,
/// `GIT_INDEX_FILE`, `GIT_WORK_TREE`, and `GIT_PREFIX` pointing at the committing
/// repository, and if those leak into `git worktree add` it fails (it inherits a
/// relative `GIT_DIR` and the committing index, so the new worktree cannot open its
/// own index). The index content is still captured correctly: `git write-tree` run
/// with `-C repo` and no `GIT_INDEX_FILE` reads `repo/.git/index`, which in a hook
/// is exactly the index being committed. A check command that shells out to git is
/// stripped too, so it sees the isolated worktree, not the outer repository.
fn strip_git_env(command: &mut Command) -> &mut Command {
	command
		.env_remove("GIT_DIR")
		.env_remove("GIT_WORK_TREE")
		.env_remove("GIT_INDEX_FILE")
		.env_remove("GIT_PREFIX")
		.env_remove("GIT_COMMON_DIR")
		.env_remove("GIT_OBJECT_DIRECTORY")
}

/// A fresh `git` command with the inherited git environment stripped (see
/// `strip_git_env`), the single constructor every runner git invocation goes
/// through so none of them leaks a hook's environment.
fn git_command() -> Command {
	let mut command = Command::new("git");
	strip_git_env(&mut command);
	command
}

/// Run a git command in `repo` and return its captured output, translating a
/// missing `git` binary into `RunError::GitUnavailable`.
fn git(
	repo: &Path,
	args: &[&str],
) -> Result<std::process::Output, RunError> {
	git_command().arg("-C").arg(repo).args(args).output().map_err(|error| {
		if error.kind() == io::ErrorKind::NotFound {
			RunError::GitUnavailable
		} else {
			RunError::Io(error)
		}
	})
}

/// Whether process `pid` is currently alive, tested by the existence of its
/// `/proc/{pid}` entry. This is the dependency-free liveness check on this
/// project's Linux target (no libc crate is pulled in just for a `kill(pid, 0)`);
/// on a non-Linux platform without `/proc` it conservatively reports "alive", so
/// the prune skips reclamation rather than risk deleting a live run's worktree.
fn pid_is_alive(pid: u32) -> bool {
	// On Linux `/proc/{pid}` exists iff the process (or a thread group leader) is
	// live. Where `/proc` is absent, `exists()` is false, so we OR in the platform
	// guard to stay conservative (treat unknown as alive).
	Path::new(&format!("/proc/{pid}")).exists() || !Path::new("/proc").exists()
}

/// Parse the owning pid out of a runner worktree directory name of the form
/// `agent-scaffold-checks-run-{pid}-{nanos}`. Returns `None` when the name does not
/// carry a parseable pid, so the caller can skip reclamation conservatively.
fn owning_pid(dir_name: &str) -> Option<u32> {
	dir_name.strip_prefix(RUNNER_PREFIX)?.split('-').next()?.parse().ok()
}

/// Reclaim runner worktrees orphaned by a prior hard-killed run (SIGKILL, which
/// `Drop` cannot catch), called at startup before creating a fresh worktree.
///
/// Reclamation is REPO-SCOPED and gated on the owning process being DEAD. It lists
/// the worktrees registered to THIS `repo` (`git worktree list --porcelain`) and,
/// for each whose path is a runner temp directory (under the system temp dir,
/// `RUNNER_PREFIX` naming), parses the owning pid embedded in that name and only
/// reclaims it when that pid is not alive (see `pid_is_alive`): unregistering it
/// (`git worktree remove --force`), deleting it (`remove_dir_all`), then pruning
/// admin entries whose directory is already gone.
///
/// The two gates together make this safe under concurrency, including two
/// overlapping `checks` runs on the SAME repository: run B lists run A's live
/// worktree, but A's pid is alive, so B skips it (a bare repo-scope filter would
/// have deleted A's worktree out from under it). A genuine orphan's owner is a
/// dead pid, so it is reclaimed. Repo-scoping additionally means another
/// repository's worktree is never touched (Principle 18, least authority), and the
/// current run's own worktree is not yet listed (the prune runs before it is
/// created). Pid reuse is a benign edge: if a new process happens to reuse a dead
/// owner's pid, its orphan is merely skipped and reclaimed by a later run, never a
/// live-tree or safety issue. An unparseable pid is treated as "skip", the same
/// conservative default. Entirely best-effort: every step ignores errors, since a
/// failure to reclaim an orphan must not fail the run.
fn prune_orphan_worktrees(repo: &Path) {
	let temp = std::env::temp_dir();
	if let Ok(listed) = git(repo, &["worktree", "list", "--porcelain"]) {
		if listed.status.success() {
			let text = String::from_utf8_lossy(&listed.stdout);
			for line in text.lines() {
				let Some(path) = line.strip_prefix("worktree ") else {
					continue;
				};
				let path = Path::new(path);
				if !path.starts_with(&temp) {
					continue;
				}
				let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
					continue;
				};
				// Reclaim only a runner worktree whose owning process is dead; a live
				// owner is a concurrent run and must be left alone.
				match owning_pid(name) {
					Some(pid) if !pid_is_alive(pid) => {
						let _ =
							git(repo, &["worktree", "remove", "--force", &path.to_string_lossy()]);
						let _ = fs::remove_dir_all(path);
					}
					_ => {}
				}
			}
		}
	}
	// Prune admin entries for worktrees whose directories are already gone.
	let _ = git(repo, &["worktree", "prune"]);
}

/// Which state the isolation worktree checks out: the working tree, or the index.
/// A plain `checks` run isolates the WORKING-TREE state; `checks --staged`
/// isolates the STAGED/index content, so the pre-commit hook checks exactly what
/// will be committed without seeing unstaged edits. The downstream path (worktree
/// add, guard, run, cleanup, prune) is identical for both; only the isolated
/// commit differs, so the mode is a single parameter to `run`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Isolation {
	/// Isolate the current tracked working-tree state (committed plus unstaged
	/// tracked modifications), the default for a plain `checks` run.
	WorkingTree,
	/// Isolate the index (staged) content, for `checks --staged` and the hook, so
	/// the run sees exactly what a commit would record.
	Staged,
}

/// The commit whose tree the isolation worktree checks out, selected by `mode`.
///
/// `WorkingTree`: the current tracked working-tree state (committed plus unstaged
/// tracked modifications) via `git stash create`, or `HEAD` when the tree is clean
/// (stash create prints nothing). A repository with no commits (so neither a stash
/// commit nor a resolvable `HEAD`) is rejected. Untracked files are not captured;
/// this is the documented limitation of the primitive.
///
/// `Staged`: the INDEX content, via `git write-tree` (a tree object of the index)
/// then `git commit-tree` (a commit-ish `git worktree add` can check out, since it
/// needs a commit, not a bare tree). The throwaway commit is given a fixed identity
/// with `-c user.name/user.email` so it succeeds even when the repository has no
/// committer identity configured. This path does not need a prior commit: an index
/// with staged content produces a tree even in a repository with no `HEAD`.
fn isolation_commit(
	repo: &Path,
	mode: Isolation,
) -> Result<String, RunError> {
	match mode {
		Isolation::WorkingTree => {
			// Resolve HEAD first: a repository with no commits has no HEAD, and `git
			// stash create` itself errors there ("You do not have the initial commit
			// yet"), so detecting the no-commits case up front gives a clear error
			// rather than a generic stash failure.
			let head = git(repo, &["rev-parse", "--verify", "HEAD"])?;
			if !head.status.success() {
				return Err(RunError::NoCommits);
			}
			let head_sha = String::from_utf8_lossy(&head.stdout).trim().to_string();

			// With at least one commit, capture the current tracked working-tree state.
			// `stash create` returns a commit of the dirty tracked state, or nothing when
			// the tree is clean, in which case HEAD is the state to isolate.
			let created = git(repo, &["stash", "create"])?;
			if !created.status.success() {
				return Err(RunError::WorktreeSetup(format!(
					"`git stash create` failed: {}",
					String::from_utf8_lossy(&created.stderr).trim()
				)));
			}
			let sha = String::from_utf8_lossy(&created.stdout).trim().to_string();
			Ok(if sha.is_empty() { head_sha } else { sha })
		}
		Isolation::Staged => {
			// A tree object of the current index (the staged content).
			let tree_out = git(repo, &["write-tree"])?;
			if !tree_out.status.success() {
				return Err(RunError::WorktreeSetup(format!(
					"`git write-tree` failed: {}",
					String::from_utf8_lossy(&tree_out.stderr).trim()
				)));
			}
			let tree = String::from_utf8_lossy(&tree_out.stdout).trim().to_string();
			// Wrap the tree in a commit `git worktree add --detach` can check out. The
			// fixed identity via `-c` keeps this working in a repository with no
			// committer identity configured (the throwaway commit is never published).
			let commit_out = git(
				repo,
				&[
					"-c",
					"user.name=agent-scaffold",
					"-c",
					"user.email=agent-scaffold@invalid",
					"commit-tree",
					&tree,
					"-m",
					"agent-scaffold checks --staged: index snapshot",
				],
			)?;
			if !commit_out.status.success() {
				return Err(RunError::WorktreeSetup(format!(
					"`git commit-tree` failed: {}",
					String::from_utf8_lossy(&commit_out.stderr).trim()
				)));
			}
			Ok(String::from_utf8_lossy(&commit_out.stdout).trim().to_string())
		}
	}
}

/// Match `path` (a forward-slash relative path from `git ls-files`) against a
/// single glob `pattern`. Patterns are repository-root-relative; a leading `./` is
/// normalised away first (`git ls-files` never emits one, so `./src/` would
/// otherwise silently never match). A pattern ending in `/` is a directory prefix
/// (`src/` matches any path under `src/`). Otherwise `**` matches any run including
/// `/` (and `**/` matches zero or more leading directories), `*` matches any run
/// not crossing `/`, `?` matches one non-`/` character, and every other byte is
/// literal. Deliberately small: enough for the documented `paths` examples
/// (`**/*.py`, `src/**/*.rs`, `tests/`), not a full gitignore engine.
fn glob_match(
	pattern: &str,
	path: &str,
) -> bool {
	let pattern = pattern.strip_prefix("./").unwrap_or(pattern);
	if let Some(prefix) = pattern.strip_suffix('/') {
		return path == prefix || path.starts_with(&format!("{prefix}/"));
	}
	glob_rec(pattern.as_bytes(), path.as_bytes())
}

/// The recursive core of `glob_match`, matching pattern bytes `p` against path
/// bytes `s` from the front.
fn glob_rec(
	p: &[u8],
	s: &[u8],
) -> bool {
	match p.first() {
		None => s.is_empty(),
		Some(b'*') if p.get(1) == Some(&b'*') => {
			let rest = &p[2 ..];
			if rest.first() == Some(&b'/') {
				// `**/` matches zero or more leading directory segments.
				let after = &rest[1 ..];
				if glob_rec(after, s) {
					return true;
				}
				(0 .. s.len()).any(|i| s[i] == b'/' && glob_rec(after, &s[i + 1 ..]))
			} else {
				// Bare `**` matches any suffix, including the empty one.
				(0 ..= s.len()).any(|i| glob_rec(rest, &s[i ..]))
			}
		}
		Some(b'*') => {
			// A single `*` matches any run that does not cross a `/`.
			let rest = &p[1 ..];
			if glob_rec(rest, s) {
				return true;
			}
			for (i, &byte) in s.iter().enumerate() {
				if byte == b'/' {
					break;
				}
				if glob_rec(rest, &s[i + 1 ..]) {
					return true;
				}
			}
			false
		}
		Some(b'?') =>
			matches!(s.first(), Some(&byte) if byte != b'/') && glob_rec(&p[1 ..], &s[1 ..]),
		Some(&pc) => matches!(s.first(), Some(&sc) if sc == pc) && glob_rec(&p[1 ..], &s[1 ..]),
	}
}

/// Whether at least one tracked file in the worktree matches one of `patterns`.
/// Reads the worktree's tracked files with `git ls-files`, so it reflects exactly
/// the isolated content. An empty `patterns` never reaches here: the caller only
/// calls this for a check whose `paths` is present AND non-empty (an empty `paths`
/// is treated as no path restriction, so the check runs unscoped).
fn any_tracked_matches(
	worktree: &Path,
	patterns: &[String],
) -> Result<bool, RunError> {
	let listed = git(worktree, &["ls-files"])?;
	if !listed.status.success() {
		return Err(RunError::WorktreeSetup(format!(
			"`git ls-files` failed: {}",
			String::from_utf8_lossy(&listed.stderr).trim()
		)));
	}
	let files = String::from_utf8_lossy(&listed.stdout);
	Ok(files
		.lines()
		.filter(|line| !line.is_empty())
		.any(|file| patterns.iter().any(|pattern| glob_match(pattern, file))))
}

/// The command a check runs, and why it might not run at all. Encodes the
/// per-kind dispatch: a `lint` runs its `command`; a `format` runs its `check`
/// command, or is skipped when it declares none (running the mutating `command`
/// is an apply action, not a check); `test`/`mutation` are skipped as reserved.
enum Runnable<'a> {
	/// Run this shell command string.
	Run(&'a str),
	/// Do not run; report this skip reason.
	Skip(String),
}

/// Decide what a check runs, before considering `paths` (Invariant: the selection
/// is a pure function of the check's kind and its commands).
fn runnable_for(check: &Check) -> Runnable<'_> {
	match check.kind {
		Kind::Lint => Runnable::Run(&check.command),
		Kind::Format => match &check.check {
			Some(command) => Runnable::Run(command),
			None => Runnable::Skip(
				"format check has no `check` command; its `command` applies formatting (a writer \
				 action), so there is nothing to check"
					.to_string(),
			),
		},
		Kind::Test => Runnable::Skip("`test` checks run in the later test module".to_string()),
		Kind::Mutation =>
			Runnable::Skip("`mutation` checks run in the later mutation module".to_string()),
	}
}

/// Run one shell command string in `worktree`, returning its outcome. The command
/// is interpreted by `sh -c`, matching how the config's commands are written (they
/// use shell features such as pipes and `!`); its stdout and stderr are captured
/// and combined for the failure report. Its stdin is `/dev/null`, so a command
/// that reads stdin gets EOF immediately rather than blocking the run forever on
/// the inherited terminal. There is deliberately no per-check timeout yet: a check
/// that hangs (or backgrounds a process holding the captured output pipe) hangs
/// the run; a timeout is a possible future addition, called out in the module doc.
fn run_command(
	worktree: &Path,
	command: &str,
) -> Result<CheckStatus, RunError> {
	let output = strip_git_env(&mut Command::new("sh"))
		.arg("-c")
		.arg(command)
		.current_dir(worktree)
		.stdin(Stdio::null())
		.output()
		.map_err(|error| {
			if error.kind() == io::ErrorKind::NotFound {
				RunError::WorktreeSetup(
					"the `sh` shell was not found to run check commands".to_string(),
				)
			} else {
				RunError::Io(error)
			}
		})?;
	if output.status.success() {
		Ok(CheckStatus::Passed)
	} else {
		let mut combined = String::new();
		let stdout = String::from_utf8_lossy(&output.stdout);
		let stderr = String::from_utf8_lossy(&output.stderr);
		if !stdout.trim().is_empty() {
			combined.push_str(stdout.trim_end());
		}
		if !stderr.trim().is_empty() {
			if !combined.is_empty() {
				combined.push('\n');
			}
			combined.push_str(stderr.trim_end());
		}
		let code = output.status.code().map_or_else(|| "signal".to_string(), |c| c.to_string());
		if combined.is_empty() {
			combined = format!("(no output; exit {code})");
		}
		Ok(CheckStatus::Failed(combined))
	}
}

/// Run the project's checks under `dir`, isolated in a temporary git worktree of
/// the state selected by `mode` (the working tree, or the index for
/// `Isolation::Staged`). Reads `dir/.agents/checks.toml`; an absent config is not
/// a failure (returns a `Report` with `config_present: false`). Only `lint` and
/// `format` kinds run in this increment; `test` and `mutation` are skipped. The
/// worktree is created only when at least one check is actually runnable, and is
/// removed on every return via the `Drop` guard (a hard kill is the one gap,
/// reclaimed by the startup prune on the next run).
pub fn run(
	dir: &Path,
	mode: Isolation,
) -> Result<Report, RunError> {
	let config_path = dir.join(".agents").join("checks.toml");
	if !config_path.exists() {
		return Ok(Report {
			results: Vec::new(),
			config_present: false,
		});
	}
	let contents = fs::read_to_string(&config_path)?;
	let checks = parse(&contents).map_err(RunError::Parse)?;

	// Decide each check's command up front. If nothing is runnable (all reserved or
	// format-without-check), report the skips without touching git at all.
	let dispatch: Vec<(&Check, Runnable)> =
		checks.iter().map(|check| (check, runnable_for(check))).collect();
	let anything_runnable = dispatch.iter().any(|(_, run)| matches!(run, Runnable::Run(_)));
	if !anything_runnable {
		let results = dispatch
			.into_iter()
			.map(|(check, run)| CheckResult {
				name: check.name.clone(),
				kind: check.kind,
				status: match run {
					Runnable::Skip(reason) => CheckStatus::Skipped(reason),
					// Unreachable: `anything_runnable` is false here.
					Runnable::Run(_) => CheckStatus::Skipped(String::new()),
				},
			})
			.collect();
		return Ok(Report {
			results,
			config_present: true,
		});
	}

	// Resolve the repository top level; this also rejects a non-repository target.
	// All commands then run at this top level, so checks are repo-scoped even when
	// `--dir` pointed at a subdirectory.
	let toplevel_out = git(dir, &["rev-parse", "--show-toplevel"])?;
	if !toplevel_out.status.success() {
		return Err(RunError::NotARepo(dir.to_path_buf()));
	}
	let repo = PathBuf::from(String::from_utf8_lossy(&toplevel_out.stdout).trim());

	// Reclaim any worktree orphaned by a prior hard-killed run before creating a new
	// one, so a SIGKILL leak self-heals on the next run (Invariant B's caveat).
	prune_orphan_worktrees(&repo);

	// The commit capturing the isolated state: the working tree (or HEAD when clean)
	// for a plain run, or the index for `--staged`.
	let commit = isolation_commit(&repo, mode)?;

	// A unique temp path OUTSIDE the repository; git worktree add creates it. The
	// `RUNNER_PREFIX` (with the embedded pid) is what the startup prune recognises.
	let worktree_path =
		std::env::temp_dir().join(format!("{RUNNER_PREFIX}{}-{}", std::process::id(), nanos()));
	let added =
		git(&repo, &["worktree", "add", "--detach", &worktree_path.to_string_lossy(), &commit])?;
	if !added.status.success() {
		return Err(RunError::WorktreeSetup(format!(
			"`git worktree add` failed: {}",
			String::from_utf8_lossy(&added.stderr).trim()
		)));
	}
	// From here the guard owns cleanup: every return path below removes the worktree.
	let _guard = WorktreeGuard {
		repo: repo.clone(),
		path: worktree_path.clone(),
	};

	let mut results = Vec::with_capacity(dispatch.len());
	for (check, run) in dispatch {
		let status = match run {
			Runnable::Skip(reason) => CheckStatus::Skipped(reason),
			Runnable::Run(command) => {
				// Honour `paths`: run only when a tracked file in the isolated tree matches
				// a glob, else skip as not-applicable. We do not pass files to the command;
				// it uses its own config and arguments. A present-but-empty `paths` is
				// treated as no restriction (run unscoped), so an empty array is not a
				// perpetual skip and `any_tracked_matches` only ever sees a non-empty set.
				match check.paths.as_deref().filter(|patterns| !patterns.is_empty()) {
					Some(patterns) =>
						if any_tracked_matches(&worktree_path, patterns)? {
							run_command(&worktree_path, command)?
						} else {
							CheckStatus::Skipped(format!(
								"no tracked file matched paths {}",
								patterns.join(", ")
							))
						},
					None => run_command(&worktree_path, command)?,
				}
			}
		};
		results.push(CheckResult {
			name: check.name.clone(),
			kind: check.kind,
			status,
		});
	}

	Ok(Report {
		results,
		config_present: true,
	})
	// `_guard` drops here, removing the worktree.
}

/// Nanoseconds since the epoch, for a unique temp path. Falls back to a fixed
/// value if the clock is before the epoch (which cannot happen on a sane system);
/// the process id in the path already provides per-process uniqueness.
fn nanos() -> u128 {
	std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.map_or(0, |elapsed| elapsed.as_nanos())
}

#[cfg(test)]
mod tests {
	use {
		super::*,
		std::process::Command,
	};

	/// A unique scratch directory under the system temp dir for one test.
	fn scratch(name: &str) -> PathBuf {
		let dir = std::env::temp_dir().join(format!(
			"agent-scaffold-checks-test-{}-{}",
			std::process::id(),
			name
		));
		let _ = fs::remove_dir_all(&dir);
		fs::create_dir_all(&dir).unwrap();
		dir
	}

	/// Run a git command in `dir`, asserting it succeeds.
	fn git_ok(
		dir: &Path,
		args: &[&str],
	) {
		let status = Command::new("git").arg("-C").arg(dir).args(args).output().unwrap();
		assert!(
			status.status.success(),
			"git {args:?} failed: {}",
			String::from_utf8_lossy(&status.stderr)
		);
	}

	/// Initialise a git repo in `dir` with a deterministic identity and no signing,
	/// so commits work in a bare CI environment.
	fn init_repo(dir: &Path) {
		git_ok(dir, &["init", "-q"]);
		git_ok(dir, &["config", "user.email", "test@example.com"]);
		git_ok(dir, &["config", "user.name", "Test"]);
		git_ok(dir, &["config", "commit.gpgsign", "false"]);
	}

	/// Write `.agents/checks.toml` under `dir` with the given body.
	fn write_config(
		dir: &Path,
		body: &str,
	) {
		let agents = dir.join(".agents");
		fs::create_dir_all(&agents).unwrap();
		fs::write(agents.join("checks.toml"), body).unwrap();
	}

	/// The current worktree paths git knows about for `dir`, one per worktree.
	fn worktree_paths(dir: &Path) -> Vec<String> {
		let out =
			Command::new("git").arg("-C").arg(dir).args(["worktree", "list"]).output().unwrap();
		String::from_utf8_lossy(&out.stdout).lines().map(str::to_string).collect()
	}

	#[test]
	fn parse_reads_all_fields_and_rejects_bad_kind() {
		let ok = parse(
			"[[check]]\nname = \"lint-a\"\nkind = \"lint\"\ncommand = \"true\"\n\n\
			 [[check]]\nname = \"fmt\"\nkind = \"format\"\ncommand = \"apply\"\ncheck = \"verify\"\n\
			 paths = [\"**/*.rs\"]\n\n\
			 [[check]]\nname = \"mut\"\nkind = \"mutation\"\ncommand = \"m\"\nbudget = \"10m\"\n\
			 threshold = 0\n",
		)
		.unwrap();
		assert_eq!(ok.len(), 3);
		assert_eq!(ok[0].kind, Kind::Lint);
		assert_eq!(ok[1].kind, Kind::Format);
		assert_eq!(ok[1].check.as_deref(), Some("verify"));
		assert_eq!(ok[1].paths.as_deref(), Some(&["**/*.rs".to_string()][..]));
		assert_eq!(ok[2].kind, Kind::Mutation);
		assert_eq!(ok[2].budget.as_deref(), Some("10m"));
		assert_eq!(ok[2].threshold, Some(0));

		// An out-of-set `kind` is rejected at the boundary (parse-don't-validate).
		let bad = parse("[[check]]\nname = \"x\"\nkind = \"bogus\"\ncommand = \"c\"\n");
		assert!(matches!(bad, Err(ParseError::Toml(_))));
	}

	#[test]
	fn parse_rejects_duplicate_names() {
		let result = parse(
			"[[check]]\nname = \"dup\"\nkind = \"lint\"\ncommand = \"a\"\n\n\
			 [[check]]\nname = \"dup\"\nkind = \"lint\"\ncommand = \"b\"\n",
		);
		match result {
			Err(ParseError::DuplicateName(name)) => assert_eq!(name, "dup"),
			other => panic!("expected DuplicateName, got {other:?}"),
		}
	}

	#[test]
	fn glob_matches_the_documented_patterns() {
		// Directory-prefix patterns.
		assert!(glob_match("src/", "src/main.rs"));
		assert!(glob_match("tests/", "tests/it.rs"));
		assert!(!glob_match("src/", "lib/main.rs"));
		// `**/*.py` matches at the root and nested.
		assert!(glob_match("**/*.py", "a.py"));
		assert!(glob_match("**/*.py", "pkg/sub/a.py"));
		assert!(!glob_match("**/*.py", "a.rs"));
		// `*` does not cross a directory separator.
		assert!(glob_match("*.rs", "main.rs"));
		assert!(!glob_match("*.rs", "src/main.rs"));
		// `src/**/*.rs` is nested-only under src.
		assert!(glob_match("src/**/*.rs", "src/a/b.rs"));
		assert!(glob_match("src/**/*.rs", "src/b.rs"));
		assert!(!glob_match("src/**/*.rs", "lib/b.rs"));
		// `?` matches a single non-separator character.
		assert!(glob_match("a?c.rs", "abc.rs"));
		assert!(!glob_match("a?c.rs", "a/c.rs"));
	}

	#[test]
	fn absent_config_is_not_a_failure() {
		let dir = scratch("absent");
		init_repo(&dir);
		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(!report.config_present);
		assert!(report.results.is_empty());
		assert!(report.success());
		assert!(!report.ran_any());
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn malformed_config_is_a_parse_error() {
		let dir = scratch("malformed");
		init_repo(&dir);
		write_config(&dir, "[[check]]\nname = \"x\"\nkind = \"nope\"\ncommand = \"c\"\n");
		match run(&dir, Isolation::WorkingTree) {
			Err(RunError::Parse(_)) => {}
			other => panic!("expected Parse error, got {other:?}"),
		}
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_passing_lint_check_succeeds_and_leaves_no_worktree() {
		// Invariant B (clean-run cleanup) and D (exit 0 on pass).
		let dir = scratch("pass");
		init_repo(&dir);
		fs::write(dir.join("file.txt"), "hello\n").unwrap();
		git_ok(&dir, &["add", "."]);
		write_config(&dir, "[[check]]\nname = \"ok\"\nkind = \"lint\"\ncommand = \"true\"\n");
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(report.success());
		assert!(report.ran_any());
		assert!(matches!(report.results[0].status, CheckStatus::Passed));
		assert_eq!(worktree_paths(&dir).len(), 1, "only the main worktree remains");
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_failing_check_reports_and_leaves_no_worktree() {
		// Invariant B (cleanup after a failing run) and D (a failing check -> not success).
		let dir = scratch("fail");
		init_repo(&dir);
		fs::write(dir.join("file.txt"), "hello\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"boom\"\nkind = \"lint\"\ncommand = \"echo nope >&2; exit 3\"\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(!report.success());
		match &report.results[0].status {
			CheckStatus::Failed(output) => assert!(output.contains("nope"), "output was: {output}"),
			other => panic!("expected Failed, got {other:?}"),
		}
		assert_eq!(worktree_paths(&dir).len(), 1, "no leftover worktree after a failing run");
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_format_check_never_mutates_the_live_tree() {
		// Invariant A: a format check that reformats a tracked file in place (like
		// treefmt's --fail-on-change) rewrites only the worktree copy; the live file
		// is byte-unchanged and the run still reports the failure.
		let dir = scratch("format-isolation");
		init_repo(&dir);
		let tracked = dir.join("code.txt");
		fs::write(&tracked, "MISFORMATTED\n").unwrap();
		// The `check` command overwrites the file then exits non-zero, exactly as an
		// in-place formatter reporting a change would.
		write_config(
			&dir,
			"[[check]]\nname = \"fmt\"\nkind = \"format\"\ncommand = \"apply\"\n\
			 check = \"printf 'reformatted\\n' > code.txt; exit 1\"\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(!report.success(), "the in-place format check reports a change as a failure");
		// The LIVE file is byte-identical to before the run.
		assert_eq!(fs::read_to_string(&tracked).unwrap(), "MISFORMATTED\n");
		assert_eq!(worktree_paths(&dir).len(), 1);
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn the_isolated_tree_reflects_unstaged_working_tree_edits() {
		// Invariant C: the worktree captures committed plus unstaged tracked edits.
		// Commit "A", then edit to "B" without staging; a lint that greps for "B"
		// passes only if the isolated tree has the working-tree content, not HEAD.
		let dir = scratch("worktree-state");
		init_repo(&dir);
		let tracked = dir.join("value.txt");
		fs::write(&tracked, "A\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"has-b\"\nkind = \"lint\"\ncommand = \"grep -q B value.txt\"\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);
		// Unstaged edit to the tracked file.
		fs::write(&tracked, "B\n").unwrap();

		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(report.success(), "the isolated tree should carry the unstaged edit (B)");
		// The live edit is still there and untouched.
		assert_eq!(fs::read_to_string(&tracked).unwrap(), "B\n");
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn staged_isolation_sees_the_index_not_the_working_tree() {
		// Invariant C, staged mode (the mirror of the working-tree test above). Commit
		// "A", stage "B", then edit the working tree to "C" WITHOUT staging. A staged
		// run isolates the index (B), so a check that greps for the staged "B" passes
		// and one that greps for the unstaged "C" fails: the index is seen, the
		// unstaged working-tree edit is not.
		let dir = scratch("staged-index");
		init_repo(&dir);
		let tracked = dir.join("value.txt");
		fs::write(&tracked, "A\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"has-b\"\nkind = \"lint\"\ncommand = \"grep -q B value.txt\"\n\n\
			 [[check]]\nname = \"no-c\"\nkind = \"lint\"\ncommand = \"! grep -q C value.txt\"\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);
		// Stage "B", then make a further unstaged edit to "C".
		fs::write(&tracked, "B\n").unwrap();
		git_ok(&dir, &["add", "value.txt"]);
		fs::write(&tracked, "C\n").unwrap();

		let report = run(&dir, Isolation::Staged).unwrap();
		assert!(
			report.success(),
			"staged mode sees the index (B) and not the unstaged working-tree edit (C)"
		);
		// The live working tree still carries the unstaged "C", untouched by the run.
		assert_eq!(fs::read_to_string(&tracked).unwrap(), "C\n");
		// And the index still holds "B" (the run did not mutate it).
		let staged =
			Command::new("git").arg("-C").arg(&dir).args(["show", ":value.txt"]).output().unwrap();
		assert_eq!(String::from_utf8_lossy(&staged.stdout), "B\n", "the index is untouched");
		assert_eq!(worktree_paths(&dir).len(), 1, "no leftover worktree after a staged run");
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_staged_run_with_nothing_staged_cleans_up() {
		// With a clean index (nothing staged beyond HEAD), a staged run writes the
		// HEAD tree, runs the check against it, and cleans up its worktree. This pins
		// that the write-tree/commit-tree path handles the no-staged-changes case.
		let dir = scratch("staged-clean");
		init_repo(&dir);
		fs::write(dir.join("value.txt"), "committed\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"has-committed\"\nkind = \"lint\"\n\
			 command = \"grep -q committed value.txt\"\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		let report = run(&dir, Isolation::Staged).unwrap();
		assert!(report.success());
		assert_eq!(worktree_paths(&dir).len(), 1, "the staged worktree is cleaned up");
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_staged_format_check_never_mutates_the_live_tree_or_index() {
		// Invariant A, staged mode: an in-place format check run under `--staged`
		// rewrites only the discardable worktree copy. The live working file AND the
		// staged index content are both byte-unchanged after the run.
		let dir = scratch("staged-format-isolation");
		init_repo(&dir);
		let tracked = dir.join("code.txt");
		fs::write(&tracked, "STAGED\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"fmt\"\nkind = \"format\"\ncommand = \"apply\"\n\
			 check = \"printf 'reformatted\\n' > code.txt; exit 1\"\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);
		// Stage a new value; the working tree matches the index here.
		fs::write(&tracked, "STAGED\n").unwrap();
		git_ok(&dir, &["add", "code.txt"]);

		let report = run(&dir, Isolation::Staged).unwrap();
		assert!(!report.success(), "the in-place format check reports a change as a failure");
		// The live file is byte-identical to before the run.
		assert_eq!(fs::read_to_string(&tracked).unwrap(), "STAGED\n");
		// The index is byte-identical too (the format check touched only the worktree).
		let staged =
			Command::new("git").arg("-C").arg(&dir).args(["show", ":code.txt"]).output().unwrap();
		assert_eq!(String::from_utf8_lossy(&staged.stdout), "STAGED\n");
		assert_eq!(worktree_paths(&dir).len(), 1);
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn staged_isolation_works_before_the_first_commit() {
		// The primary hook scenario for a new repository: files are staged but there is
		// no HEAD yet (the initial commit). `write-tree` still writes the index tree and
		// `commit-tree` makes a parentless root commit, so a staged run works with no
		// prior commit: it passes on clean staged content and fails on a staged
		// violation. Unlike working-tree mode, which rejects a no-commits repo.
		let dir = scratch("staged-initial-commit");
		init_repo(&dir);
		fs::write(dir.join("value.txt"), "clean\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"no-nope\"\nkind = \"lint\"\n\
			 command = \"! grep -q NOPE value.txt\"\n",
		);
		// Stage everything, but make NO commit: the repository has no HEAD.
		git_ok(&dir, &["add", "."]);
		let head = Command::new("git")
			.arg("-C")
			.arg(&dir)
			.args(["rev-parse", "--verify", "HEAD"])
			.output()
			.unwrap();
		assert!(!head.status.success(), "the repository must have no HEAD for this test");

		// Clean staged content passes.
		let report = run(&dir, Isolation::Staged).unwrap();
		assert!(
			report.success(),
			"staged mode works before the first commit (clean content passes)"
		);
		assert_eq!(worktree_paths(&dir).len(), 1, "the worktree is cleaned up");

		// A staged violation fails, still before any commit.
		fs::write(dir.join("value.txt"), "NOPE\n").unwrap();
		git_ok(&dir, &["add", "value.txt"]);
		let report = run(&dir, Isolation::Staged).unwrap();
		assert!(!report.success(), "a staged violation fails before the first commit too");
		assert_eq!(worktree_paths(&dir).len(), 1);
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_clean_tree_isolates_head() {
		// With a clean tree, stash create is empty and the run isolates HEAD.
		let dir = scratch("clean-tree");
		init_repo(&dir);
		fs::write(dir.join("value.txt"), "committed\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"has-committed\"\nkind = \"lint\"\n\
			 command = \"grep -q committed value.txt\"\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(report.success());
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn reserved_and_checkless_kinds_are_skipped_without_a_worktree() {
		// A config of only test/mutation/format-without-check runs nothing and never
		// touches git, so it works even in a non-repo directory.
		let dir = scratch("all-skipped");
		fs::create_dir_all(&dir).unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"t\"\nkind = \"test\"\ncommand = \"cargo test\"\n\n\
			 [[check]]\nname = \"m\"\nkind = \"mutation\"\ncommand = \"cargo mutants\"\n\n\
			 [[check]]\nname = \"f\"\nkind = \"format\"\ncommand = \"apply\"\n",
		);
		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(report.config_present);
		assert!(!report.ran_any());
		assert!(report.success());
		assert!(report.results.iter().all(|r| matches!(r.status, CheckStatus::Skipped(_))));
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_paths_check_with_no_matching_file_is_skipped() {
		let dir = scratch("paths-skip");
		init_repo(&dir);
		fs::write(dir.join("main.rs"), "fn main() {}\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"py\"\nkind = \"lint\"\ncommand = \"false\"\npaths = [\"**/*.py\"]\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		// The command is `false` (would fail if run), but no `.py` file matches, so it
		// is skipped and the run still succeeds.
		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(report.success());
		match &report.results[0].status {
			CheckStatus::Skipped(reason) => assert!(reason.contains("no tracked file")),
			other => panic!("expected Skipped, got {other:?}"),
		}
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_paths_check_with_a_matching_file_runs() {
		let dir = scratch("paths-run");
		init_repo(&dir);
		fs::write(dir.join("mod.py"), "x = 1\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"py\"\nkind = \"lint\"\ncommand = \"true\"\npaths = [\"**/*.py\"]\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(report.success());
		assert!(matches!(report.results[0].status, CheckStatus::Passed));
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_non_repo_target_with_runnable_checks_errors() {
		let dir = scratch("non-repo");
		write_config(&dir, "[[check]]\nname = \"ok\"\nkind = \"lint\"\ncommand = \"true\"\n");
		match run(&dir, Isolation::WorkingTree) {
			Err(RunError::NotARepo(_)) => {}
			other => panic!("expected NotARepo, got {other:?}"),
		}
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn an_empty_repo_with_no_commits_errors() {
		let dir = scratch("no-commits");
		init_repo(&dir);
		write_config(&dir, "[[check]]\nname = \"ok\"\nkind = \"lint\"\ncommand = \"true\"\n");
		match run(&dir, Isolation::WorkingTree) {
			Err(RunError::NoCommits) => {}
			other => panic!("expected NoCommits, got {other:?}"),
		}
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn malformed_config_toml_syntax_error_is_a_parse_error() {
		// A TOML syntax error (not just an out-of-set enum) is rejected as a parse
		// error, so the boundary covers a genuinely unparseable file, not only a
		// schema-shaped one.
		match parse("this is = = not valid toml [[[") {
			Err(ParseError::Toml(_)) => {}
			other => panic!("expected Toml parse error, got {other:?}"),
		}
	}

	#[test]
	fn run_error_exit_codes_split_config_from_environment() {
		// The exit-code mapping is single-sourced on `RunError::exit_code`: a
		// malformed config is exit 1 (a project problem, like a failing check), and
		// every environment/usage error, INCLUDING an unreadable config (`Io`), is
		// exit 2 rather than the default `io::Result` exit 1.
		let toml_err = toml::from_str::<ChecksFile>("x = =").unwrap_err();
		assert_eq!(RunError::Parse(ParseError::Toml(toml_err)).exit_code(), 1);
		assert_eq!(RunError::NotARepo(PathBuf::from(".")).exit_code(), 2);
		assert_eq!(RunError::NoCommits.exit_code(), 2);
		assert_eq!(RunError::GitUnavailable.exit_code(), 2);
		assert_eq!(RunError::WorktreeSetup("x".to_string()).exit_code(), 2);
		assert_eq!(RunError::Io(io::Error::other("boom")).exit_code(), 2);
	}

	#[test]
	fn an_unreadable_config_is_an_environment_error_not_a_check_failure() {
		// A config path that exists but cannot be read as a file (here it is a
		// directory) is an `Io` error -> exit 2, distinct from a failing check
		// (exit 1). This pins that the unreadable-config path takes the
		// environment-error branch rather than propagating as the default exit 1.
		let dir = scratch("unreadable-config");
		init_repo(&dir);
		// Make `.agents/checks.toml` a directory, so `read_to_string` errors.
		fs::create_dir_all(dir.join(".agents").join("checks.toml")).unwrap();
		match run(&dir, Isolation::WorkingTree) {
			Err(error @ RunError::Io(_)) => assert_eq!(error.exit_code(), 2),
			other => panic!("expected Io error with exit code 2, got {other:?}"),
		}
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_paths_glob_false_negative_skips_a_root_only_pattern() {
		// A root-level `*.py` does not cross a directory separator, so a repo whose
		// only Python file is nested (`src/module.py`) matches nothing and the check
		// is skipped. The command is `false`, so a mistaken run would fail the run.
		let dir = scratch("paths-false-negative");
		init_repo(&dir);
		fs::create_dir_all(dir.join("src")).unwrap();
		fs::write(dir.join("src").join("module.py"), "x = 1\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"py\"\nkind = \"lint\"\ncommand = \"false\"\npaths = [\"*.py\"]\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(report.success(), "a nested .py must not match a root-only *.py");
		match &report.results[0].status {
			CheckStatus::Skipped(reason) => assert!(reason.contains("no tracked file")),
			other => panic!("expected Skipped, got {other:?}"),
		}
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn an_empty_paths_array_runs_unscoped() {
		// A present-but-empty `paths` is treated as no restriction, so the check runs
		// (and here fails), never producing the trailing-space "no tracked file
		// matched paths " skip that the empty array used to yield.
		let dir = scratch("empty-paths");
		init_repo(&dir);
		fs::write(dir.join("file.txt"), "x\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"e\"\nkind = \"lint\"\ncommand = \"false\"\npaths = []\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(!report.success(), "an empty paths array runs the check unscoped");
		assert!(matches!(report.results[0].status, CheckStatus::Failed(_)));
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_stdin_reading_check_does_not_hang() {
		// A check that reads stdin gets EOF from /dev/null instead of blocking on an
		// inherited terminal. `cat` copies stdin to stdout and exits 0 on EOF; if
		// stdin were inherited and empty in a non-tty test, this would still return,
		// but the null redirection is what guarantees EOF in every environment.
		let dir = scratch("stdin-null");
		init_repo(&dir);
		fs::write(dir.join("file.txt"), "x\n").unwrap();
		write_config(
			&dir,
			"[[check]]\nname = \"reads-stdin\"\nkind = \"lint\"\ncommand = \"cat\"\n",
		);
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(report.success(), "a stdin-reading check gets EOF and exits cleanly");
		fs::remove_dir_all(&dir).unwrap();
	}

	/// A pid that is not alive, for the dead-owner orphan cases. `u32::MAX` is far
	/// above any real pid, so `/proc/{pid}` does not exist; asserted here so the
	/// test fails loudly rather than silently if that ever stops holding.
	fn dead_pid() -> u32 {
		let pid = u32::MAX;
		assert!(!pid_is_alive(pid), "expected pid {pid} to be dead");
		pid
	}

	#[test]
	fn a_startup_prune_reclaims_an_orphaned_runner_worktree() {
		// Invariant B's self-heal: an orphan left by a prior hard-killed run (a
		// runner-prefixed temp worktree still registered to this repo, owned by a now
		// dead pid) is reclaimed on the next run, leaving `git worktree list` with
		// only the main worktree. The reclamation is repo-scoped, so this holds even
		// under parallel tests: only this repo's orphan is touched.
		let dir = scratch("prune-orphan");
		init_repo(&dir);
		fs::write(dir.join("file.txt"), "x\n").unwrap();
		write_config(&dir, "[[check]]\nname = \"ok\"\nkind = \"lint\"\ncommand = \"true\"\n");
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		// Plant the orphan: a registered worktree under a runner-prefixed temp path
		// owned by a DEAD pid, exactly the shape a SIGKILLed run leaves behind
		// (registered but never removed by its `Drop`).
		let orphan =
			std::env::temp_dir().join(format!("{RUNNER_PREFIX}{}-{}", dead_pid(), nanos()));
		git_ok(&dir, &["worktree", "add", "--detach", &orphan.to_string_lossy(), "HEAD"]);
		assert!(orphan.exists());
		// Two worktrees now: main plus the planted orphan.
		assert_eq!(worktree_paths(&dir).len(), 2);

		let report = run(&dir, Isolation::WorkingTree).unwrap();
		assert!(report.success());
		// The orphan directory is gone and only the main worktree remains.
		assert!(!orphan.exists(), "the registered orphan worktree was reclaimed");
		assert_eq!(worktree_paths(&dir).len(), 1, "git worktree list is clean after the prune");
		fs::remove_dir_all(&dir).unwrap();
	}

	#[test]
	fn a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one() {
		// The liveness gate: a runner worktree owned by a LIVE process (a concurrent
		// run, modelled here by this test process's own pid) is left untouched, while
		// one owned by a DEAD pid is reclaimed. This is what stops two overlapping
		// same-repo runs from deleting each other's worktree.
		let dir = scratch("prune-liveness");
		init_repo(&dir);
		fs::write(dir.join("file.txt"), "x\n").unwrap();
		git_ok(&dir, &["add", "."]);
		git_ok(&dir, &["commit", "-q", "-m", "init"]);

		// A live-owner worktree (current pid) and a dead-owner worktree, both
		// registered to this repo and both matching the runner-prefix filter.
		let live =
			std::env::temp_dir().join(format!("{RUNNER_PREFIX}{}-{}", std::process::id(), nanos()));
		let dead = std::env::temp_dir().join(format!("{RUNNER_PREFIX}{}-{}", dead_pid(), nanos()));
		git_ok(&dir, &["worktree", "add", "--detach", &live.to_string_lossy(), "HEAD"]);
		git_ok(&dir, &["worktree", "add", "--detach", &dead.to_string_lossy(), "HEAD"]);
		assert!(live.exists() && dead.exists());

		prune_orphan_worktrees(&dir);

		// The live owner survives; the dead owner is reclaimed.
		assert!(live.exists(), "a live owner's worktree must not be reclaimed");
		assert!(!dead.exists(), "a dead owner's worktree is reclaimed");

		// Clean up the deliberately-left live worktree.
		git_ok(&dir, &["worktree", "remove", "--force", &live.to_string_lossy()]);
		fs::remove_dir_all(&dir).unwrap();
	}
}
