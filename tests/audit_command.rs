//! End-to-end tests for `agent-scaffold audit` through the real binary entry point
//! (`main` -> `run_audit`), covering the two behaviours a unit test cannot observe: that
//! `--json` prints the machine intermediate and writes NO file, and that the report path
//! is derived from the plan-source filename (`docs/plans/<task>.code-value-report.md`) or
//! overridden by `--out`. Increment 1 emits an EMPTY report, so these assert the caveat
//! and the empty record list, not any harvested candidate.

use std::{
	fs,
	path::{
		Path,
		PathBuf,
	},
	process::Command,
};

/// A unique scratch directory under the system temp dir for one test.
fn scratch(name: &str) -> PathBuf {
	let dir =
		std::env::temp_dir().join(format!("agent-scaffold-audit-{}-{}", std::process::id(), name));
	let _ = fs::remove_dir_all(&dir);
	dir
}

/// Run the built binary's `audit` with the given args in `dir`, returning
/// `(success, stdout)`.
fn audit(
	dir: &Path,
	args: &[&str],
) -> (bool, String) {
	let output = Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))
		.arg("audit")
		.args(args)
		.current_dir(dir)
		.output()
		.unwrap();
	(output.status.success(), String::from_utf8_lossy(&output.stdout).to_string())
}

#[test]
fn json_prints_the_intermediate_and_writes_no_file() {
	let dir = scratch("json");
	fs::create_dir_all(&dir).unwrap();
	// The plan source need not exist: `<task>` is derived from its filename only.
	let (ok, stdout) = audit(&dir, &["--source", "docs/plans/demo.plan.toml", "--json"]);
	assert!(ok, "audit --json should exit 0");
	// The typed intermediate is on stdout: the task, the single-sourced caveat, and an
	// empty record list (Increment 1 runs no signal).
	assert!(stdout.contains("\"task\": \"demo\""), "stdout: {stdout}");
	assert!(stdout.contains("\"caveat\":"), "stdout: {stdout}");
	assert!(stdout.contains("\"records\": []"), "stdout: {stdout}");
	assert!(stdout.contains("\"rustc_dead_code\": false"), "stdout: {stdout}");
	// `--json` writes nothing: the default report path was never created (nor was its dir).
	assert!(!dir.join("docs/plans/demo.code-value-report.md").exists());
	let _ = fs::remove_dir_all(&dir);
}

#[test]
fn default_out_path_is_derived_from_the_source_filename() {
	let dir = scratch("default-out");
	fs::create_dir_all(dir.join("docs/plans")).unwrap();
	let (ok, stdout) = audit(&dir, &["--source", "docs/plans/demo.plan.toml"]);
	assert!(ok, "audit should exit 0");
	let report = dir.join("docs/plans/demo.code-value-report.md");
	assert!(report.exists(), "the derived report path should be written");
	assert!(stdout.contains("wrote docs/plans/demo.code-value-report.md"), "stdout: {stdout}");
	let body = fs::read_to_string(&report).unwrap();
	// The report leads with its title and the mandatory caveat, and its record sections are
	// empty in Increment 1.
	assert!(body.starts_with("# Code-value audit: demo\n"), "body: {body}");
	assert!(body.contains("This report is advisory."), "body: {body}");
	assert!(body.contains("## Candidates: dead code\n\n_None._"), "body: {body}");
	let _ = fs::remove_dir_all(&dir);
}

#[test]
fn out_override_writes_the_given_path_and_not_the_default() {
	let dir = scratch("out-override");
	fs::create_dir_all(dir.join("docs/plans")).unwrap();
	fs::create_dir_all(dir.join("custom")).unwrap();
	let (ok, _stdout) =
		audit(&dir, &["--source", "docs/plans/demo.plan.toml", "--out", "custom/report.md"]);
	assert!(ok, "audit --out should exit 0");
	assert!(dir.join("custom/report.md").exists(), "the --out path should be written");
	assert!(
		!dir.join("docs/plans/demo.code-value-report.md").exists(),
		"the default path should not be written when --out is given"
	);
	let _ = fs::remove_dir_all(&dir);
}

#[test]
fn no_plan_source_falls_back_to_the_task_slug() {
	let dir = scratch("no-source");
	fs::create_dir_all(dir.join("docs/plans")).unwrap();
	// With neither --source nor --plan, `<task>` falls back to `task` (the same fallback
	// `next` uses), so the default report path is `docs/plans/task.code-value-report.md`.
	let (ok, _stdout) = audit(&dir, &[]);
	assert!(ok, "audit with no plan source should exit 0");
	assert!(dir.join("docs/plans/task.code-value-report.md").exists());
	let _ = fs::remove_dir_all(&dir);
}
