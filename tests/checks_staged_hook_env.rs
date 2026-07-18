//! Regression test for `checks --staged` under a pre-commit hook environment.
//!
//! Git runs a hook with `GIT_DIR`, `GIT_INDEX_FILE`, `GIT_WORK_TREE`, and
//! `GIT_PREFIX` set to the committing repository. If those leak into the runner's
//! `git worktree add`, it fails ("index file open failed: Not a directory"),
//! making `--staged` unusable in the very hook it exists for. The runner strips
//! the git environment before shelling out; this test pins that by invoking the
//! built binary with a hook-shaped environment and asserting it runs the staged
//! checks (exit 1 on a staged violation, exit 0 when clean) rather than failing to
//! set up its worktree (exit 2).

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

/// Run the built binary's `checks --staged` in `dir` with a pre-commit hook's git
/// environment set, returning its exit code.
fn checks_staged_under_hook_env(dir: &Path) -> Option<i32> {
	Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))
		.args(["checks", "--staged"])
		.current_dir(dir)
		// The variables git exports to a hook. A relative GIT_DIR plus GIT_INDEX_FILE
		// is what previously broke `git worktree add`.
		.env("GIT_DIR", ".git")
		.env("GIT_INDEX_FILE", ".git/index")
		.env("GIT_PREFIX", "")
		.output()
		.unwrap()
		.status
		.code()
}

#[test]
fn checks_staged_runs_under_a_hook_environment() {
	let dir = std::env::temp_dir().join(format!(
		"agent-scaffold-hookenv-{}-{}",
		std::process::id(),
		"staged"
	));
	let _ = fs::remove_dir_all(&dir);
	fs::create_dir_all(&dir).unwrap();
	git(&dir, &["init", "-q"]);
	git(&dir, &["config", "user.email", "test@example.com"]);
	git(&dir, &["config", "user.name", "Test"]);
	git(&dir, &["config", "commit.gpgsign", "false"]);
	fs::create_dir_all(dir.join(".agents")).unwrap();
	fs::write(
		dir.join(".agents").join("checks.toml"),
		"[[check]]\nname = \"no-nope\"\nkind = \"lint\"\n\
		 command = \"! grep -rq NOPE --include=*.txt .\"\n",
	)
	.unwrap();
	fs::write(dir.join("file.txt"), "clean\n").unwrap();
	git(&dir, &["add", "."]);
	git(&dir, &["commit", "-qm", "init"]);

	// Stage a violation, exactly what git presents to a pre-commit hook.
	fs::write(dir.join("file.txt"), "NOPE\n").unwrap();
	git(&dir, &["add", "file.txt"]);
	assert_eq!(
		checks_staged_under_hook_env(&dir),
		Some(1),
		"a staged violation must fail the check (exit 1), not error the worktree setup (exit 2)"
	);

	// Stage the fix; the same hook environment now passes.
	fs::write(dir.join("file.txt"), "clean\n").unwrap();
	git(&dir, &["add", "file.txt"]);
	assert_eq!(
		checks_staged_under_hook_env(&dir),
		Some(0),
		"clean staged content passes under a hook environment"
	);

	fs::remove_dir_all(&dir).unwrap();
}
