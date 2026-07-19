//! Regression test for the Inc 6 clap relaxation: `--workflow` no longer `requires`
//! `--plan`, so a TOML-primary project with NO Markdown plan can run
//! `validate --workflow --source <plan.toml>` end to end. Before the relaxation clap
//! rejected the combination with a usage error (exit 2) because `--workflow` was
//! declared `requires = "plan"`.
//!
//! This pins two directions: the TOML-primary `--workflow --source` with no `--plan`
//! reaches the workflow check and passes (exit 0, `workflow invariants hold`), and the
//! Markdown path still works when `--plan` is supplied.

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
