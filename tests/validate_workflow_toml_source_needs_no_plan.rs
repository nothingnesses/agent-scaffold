//! Regression test for the Inc 6 clap relaxation: `--workflow` no longer `requires`
//! `--plan`, so a TOML-primary project with NO Markdown plan can run
//! `validate --workflow --source <plan.toml>` end to end. Before the relaxation clap
//! rejected the combination with a usage error (exit 2) because `--workflow` was
//! declared `requires = "plan"`.
//!
//! This pins two directions:
//! - Positive: the TOML-primary `--workflow --source` with no `--plan` reaches the
//!   workflow check and passes (exit 0, `workflow invariants hold`).
//! - Negative: `--workflow` with no resolvable plan source, whether no `--source`/`--plan`
//!   at all or a typo'd/missing `--source`, hard-errors (exit 1) naming the missing plan
//!   source rather than silently skipping the check and exiting 0. This is the false-green
//!   regression the relaxation would otherwise have opened (Inc 6 M-1).

use std::{
	fs,
	path::Path,
	process::Command,
};

/// A minimal TOML-primary `<task>.plan.toml` that validates clean and holds the
/// workflow check: no `complete` step (so W3 has nothing to enforce) and no decided
/// question (so W4 has nothing to enforce), so an empty round log is enough for
/// `workflow invariants hold`.
const PLAN_TOML: &str = "\
[meta]
title = \"TOML-only project\"
primary = \"toml\"

[[step]]
slug = \"only-step\"
title = \"The only step\"
status = \"not-started\"
order = 1
";

/// Run the built binary's `validate` with the given args in `dir`, returning
/// `(exit_code, stdout, stderr)`.
fn validate(
	dir: &Path,
	args: &[&str],
) -> (Option<i32>, String, String) {
	let output = Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))
		.arg("validate")
		.args(args)
		.current_dir(dir)
		.output()
		.expect("run agent-scaffold validate");
	(
		output.status.code(),
		String::from_utf8_lossy(&output.stdout).into_owned(),
		String::from_utf8_lossy(&output.stderr).into_owned(),
	)
}

#[test]
fn workflow_on_a_toml_source_runs_without_a_markdown_plan() {
	let dir = std::env::temp_dir().join(format!(
		"agent-scaffold-validate-toml-only-{}-{}",
		std::process::id(),
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
	));
	fs::create_dir_all(&dir).unwrap();
	fs::write(dir.join("plan.plan.toml"), PLAN_TOML).unwrap();
	// An empty (present) round log: the workflow check needs a metrics file present,
	// and an empty one is 0 records, valid, with nothing to enforce.
	fs::write(dir.join("workflow.jsonl"), "").unwrap();

	// The relaxation under test: `--workflow --source` with NO `--plan`. Before Inc 6
	// clap rejected this with exit 2; now it reaches the workflow check and passes.
	let (code, stdout, stderr) = validate(
		&dir,
		&["--metrics", "workflow.jsonl", "--workflow", "--source", "plan.plan.toml"],
	);
	assert_eq!(
		code,
		Some(0),
		"a TOML-primary --workflow --source with no --plan should pass; stderr:\n{stderr}\nstdout:\n{stdout}"
	);
	assert!(
		stdout.contains("workflow invariants hold"),
		"expected the workflow invariants to hold; stdout:\n{stdout}"
	);

	fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn workflow_with_no_plan_source_hard_errors_instead_of_skipping() {
	let dir = std::env::temp_dir().join(format!(
		"agent-scaffold-validate-workflow-no-source-{}-{}",
		std::process::id(),
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
	));
	fs::create_dir_all(&dir).unwrap();
	// A present (empty) metrics log, so the only thing missing is the plan source. This
	// isolates the regression: with a source present but metrics missing the tool still
	// soft-skips, so a present metrics log proves the hard error is about the plan source.
	fs::write(dir.join("workflow.jsonl"), "").unwrap();

	// (a) No --source and no --plan at all. Before the M-1 fix this fell into the `_`
	// catch-all: stderr note + exit 0, green-passing while validating nothing.
	let (code, stdout, stderr) = validate(&dir, &["--metrics", "workflow.jsonl", "--workflow"]);
	assert_ne!(
		code,
		Some(0),
		"--workflow with no plan source must not exit 0; stdout:\n{stdout}\nstderr:\n{stderr}"
	);
	assert!(
		stderr.contains("no plan source resolved"),
		"expected a problem naming the missing plan source; stderr:\n{stderr}"
	);

	// (b) A typo'd/missing --source path (the realistic CI failure mode): the path does
	// not resolve to a TOML-primary plan, and there is no --plan, so the workflow gate has
	// nothing to check. This must hard-error, not silently pass a misconfigured gate.
	let (code, stdout, stderr) = validate(
		&dir,
		&["--metrics", "workflow.jsonl", "--workflow", "--source", "typo.plan.toml"],
	);
	assert_ne!(
		code,
		Some(0),
		"--workflow with a typo'd --source must not exit 0; stdout:\n{stdout}\nstderr:\n{stderr}"
	);
	assert!(
		stderr.contains("no plan source resolved"),
		"expected a problem naming the missing plan source; stderr:\n{stderr}"
	);

	fs::remove_dir_all(&dir).unwrap();
}
