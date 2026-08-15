//! Regression test for `checks` under a `TMPDIR` that does not exist yet.
//!
//! The runner RESERVES its worktree directory by creating it rather than letting
//! `git worktree add` create it, and the claim deliberately creates exactly one
//! level. That made a `TMPDIR` whose leading directories are missing fail with
//! "could not reserve the runner worktree directory ...: No such file or directory
//! (os error 2)", which the add used to handle: a `TMPDIR` naming a directory that
//! does not exist yet is legal, and it worked before the reservation existed. The
//! runner now creates those leading directories itself, once, outside the retry
//! loop; this pins that by running the built binary against a scratch repo with
//! `TMPDIR` set to a nested path that does not exist and asserting the run still
//! completes (exit 0) instead of erroring its worktree setup (exit 2).
//!
//! An integration test rather than a unit one because setting `TMPDIR` in-process
//! needs `std::env::set_var`, which is unsafe and would leak into every other test
//! thread of the same binary. A child process takes the variable safely, and the
//! result is deterministic: one run, no concurrency, no sampling.

use std::{
	fs,
	path::Path,
	process::Command,
};

/// Run a git command in `dir`, asserting it succeeds.
fn git(
	dir: &Path,
	args: &[&str],
) {
	let output = Command::new("git").arg("-C").arg(dir).args(args).output().unwrap();
	assert!(
		output.status.success(),
		"git {args:?} failed: {}",
		String::from_utf8_lossy(&output.stderr)
	);
}

/// Run the built binary's `checks` in `dir` with `TMPDIR` set to `tmpdir`, returning
/// its exit code.
fn checks_with_tmpdir(
	dir: &Path,
	tmpdir: &Path,
) -> Option<i32> {
	Command::new(env!("CARGO_BIN_EXE_agent-flow"))
		.arg("checks")
		.current_dir(dir)
		.env("TMPDIR", tmpdir)
		.output()
		.unwrap()
		.status
		.code()
}

#[test]
fn checks_runs_under_a_tmpdir_that_does_not_exist_yet() {
	let dir =
		std::env::temp_dir().join(format!("agent-flow-missingtmp-{}", std::process::id()));
	let _ = fs::remove_dir_all(&dir);
	fs::create_dir_all(&dir).unwrap();
	git(&dir, &["init", "-q"]);
	git(&dir, &["config", "user.email", "test@example.com"]);
	git(&dir, &["config", "user.name", "Test"]);
	git(&dir, &["config", "commit.gpgsign", "false"]);
	fs::create_dir_all(dir.join(".agents")).unwrap();
	fs::write(
		dir.join(".agents").join("checks.toml"),
		"[[check]]\nname = \"lint\"\nkind = \"lint\"\ncommand = \"true\"\n",
	)
	.unwrap();
	fs::write(dir.join("file.txt"), "clean\n").unwrap();
	git(&dir, &["add", "."]);
	git(&dir, &["commit", "-qm", "init"]);

	// TWO missing levels, so creating only the leaf would not be enough either.
	let missing = dir.join("missing").join("nested");
	assert!(!missing.exists(), "the TMPDIR under test must not exist before the run");

	assert_eq!(
		checks_with_tmpdir(&dir, &missing),
		Some(0),
		"a TMPDIR naming a directory that does not exist yet is legal and must still run"
	);
	assert!(missing.is_dir(), "the run created its temp directory's leading path");

	fs::remove_dir_all(&dir).unwrap();
}
