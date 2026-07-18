//! End-to-end tests for `scaffold --with-precommit-hook` through the real binary
//! entry point (`main` -> `run_scaffold`), not the `install_precommit_hook` helper
//! alone. They cover the coherence usage error (which calls `process::exit(2)`, so
//! it can only be observed from a subprocess) and the create-if-absent install.

use std::{
	fs,
	path::PathBuf,
	process::Command,
};

/// A unique scratch directory under the system temp dir for one test.
fn scratch(name: &str) -> PathBuf {
	let dir = std::env::temp_dir().join(format!(
		"agent-scaffold-e2ehook-{}-{}",
		std::process::id(),
		name
	));
	let _ = fs::remove_dir_all(&dir);
	dir
}

#[test]
fn scaffold_with_precommit_hook_without_checks_module_exits_2() {
	// The coherence usage error: requesting the hook without `--module checks` is a
	// usage error (exit 2), and nothing is written.
	let out = scratch("coherence");
	let status = Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))
		.args([
			"scaffold",
			"--with-precommit-hook",
			"--write",
			"--principles",
			"default",
			"--vcs",
			"none",
			"--output-dir",
		])
		.arg(&out)
		.output()
		.unwrap();
	assert_eq!(status.status.code(), Some(2), "the incoherent combination exits 2");
	let stderr = String::from_utf8_lossy(&status.stderr);
	assert!(stderr.contains("requires --module checks"), "stderr explains the error: {stderr}");
	assert!(!out.exists(), "nothing is written on the usage error");
}

#[test]
fn scaffold_with_precommit_hook_installs_it_create_if_absent() {
	// The happy path through the real entry point: `--module checks
	// --with-precommit-hook --write` initialises the repo, drops the assets, and
	// installs an executable, delegating `.git/hooks/pre-commit` (none existed).
	let out = scratch("install");
	let result = Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))
		.args([
			"scaffold",
			"--module",
			"checks",
			"--with-precommit-hook",
			"--write",
			"--principles",
			"default",
			"--output-dir",
		])
		.arg(&out)
		.output()
		.unwrap();
	assert!(result.status.success(), "the scaffold succeeds (exit 0)");
	let stdout = String::from_utf8_lossy(&result.stdout);
	assert!(stdout.contains("Installed the pre-commit hook"), "the install is reported: {stdout}");

	let hook = out.join(".git").join("hooks").join("pre-commit");
	assert!(hook.exists(), "the hook was installed");
	let body = fs::read_to_string(&hook).unwrap();
	assert!(body.starts_with("#!/bin/sh"), "the hook has a shebang");
	assert!(body.contains(".agents/hooks/pre-commit"), "the hook delegates to the tracked asset");
	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;
		let mode = fs::metadata(&hook).unwrap().permissions().mode();
		assert!(mode & 0o111 != 0, "the installed hook is executable");
	}

	fs::remove_dir_all(&out).unwrap();
}
