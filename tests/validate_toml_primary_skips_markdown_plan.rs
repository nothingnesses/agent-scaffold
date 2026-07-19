//! Regression test for the Inc 5 cutover fix: in TOML-primary mode the Markdown
//! `--plan` is a GENERATED projection (rendered from the `<task>.plan.toml`, pinned
//! to it by `render --check`), so `validate` must NOT run the hand-authored-Markdown
//! validator (`validate_plan`) against it. The live cutover surfaced this: the
//! rendered plan writes a superseded queue item as `(superseded by `Q-x`)`, which the
//! Markdown queue-status vocabulary (only the bare `superseded`) rejects, so the
//! standalone Markdown validator fails on the projection even though the TOML source
//! is valid.
//!
//! This pins both directions: a TOML-primary `validate --source <toml> --plan <md>
//! --workflow` passes (exit 0) on a `--plan` md that WOULD fail the standalone
//! Markdown validator, and a Markdown-mode `validate --plan <md>` on that same md
//! still fails (exit 1), so the skip is scoped to the projection case only.

use std::{
	fs,
	path::Path,
	process::Command,
};

/// A minimal TOML-primary `<task>.plan.toml` that validates clean and holds the
/// workflow check: no `complete` step (so W3 has nothing to enforce) and no decided
/// question past a baseline (so W4 has nothing to enforce), so an empty round log is
/// enough for `workflow invariants hold`.
const PLAN_TOML: &str = "\
[meta]
title = \"TOML-primary projection test\"
primary = \"toml\"

[[step]]
slug = \"only-step\"
title = \"The only step\"
status = \"not-started\"
order = 1

[[question]]
id = \"Q-1\"
status = \"open\"
ask = \"An open ask still awaiting a decision.\"
";

/// A Markdown `--plan` whose one queue item carries a `superseded by `Q-1`` status.
/// The standalone Markdown validator only accepts the bare `superseded` token, so it
/// reports an unknown status here (exit 1). It stands in for the generated projection.
const PLAN_MD: &str = "\
# Projection test plan

## Open Questions, Decisions, Issues and Blockers

- `Q-2` (superseded by `Q-1`) an ask whose Markdown status the standalone validator rejects.
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
fn toml_primary_skips_the_markdown_plan_validator_but_markdown_mode_still_fails() {
	let dir = std::env::temp_dir().join(format!(
		"agent-scaffold-validate-projection-{}-{}",
		std::process::id(),
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
	));
	fs::create_dir_all(&dir).unwrap();
	fs::write(dir.join("plan.plan.toml"), PLAN_TOML).unwrap();
	fs::write(dir.join("plan.md"), PLAN_MD).unwrap();
	// An empty (present) round log: the workflow check needs a metrics file present,
	// and an empty one is 0 records, valid, with nothing to enforce.
	fs::write(dir.join("workflow.jsonl"), "").unwrap();

	// TOML-primary: the bad `--plan` md is a projection, so `validate_plan` is skipped
	// and the run passes even though that md would fail the standalone validator.
	let (code, stdout, stderr) = validate(
		&dir,
		&[
			"--metrics",
			"workflow.jsonl",
			"--plan",
			"plan.md",
			"--workflow",
			"--source",
			"plan.plan.toml",
		],
	);
	assert_eq!(
		code,
		Some(0),
		"TOML-primary validate should pass; stderr:\n{stderr}\nstdout:\n{stdout}"
	);
	assert!(
		stdout.contains("workflow invariants hold"),
		"expected the workflow invariants to hold; stdout:\n{stdout}"
	);
	assert!(
		stdout.contains("skipping the Markdown plan validator"),
		"expected a note that the Markdown plan validator was skipped; stdout:\n{stdout}"
	);

	// Markdown mode (no --source): the SAME md now goes through `validate_plan` and
	// fails on the unknown superseded status, so the skip is scoped to the projection.
	let (code, stdout, stderr) =
		validate(&dir, &["--metrics", "workflow.jsonl", "--plan", "plan.md"]);
	assert_eq!(
		code,
		Some(1),
		"Markdown-mode validate should fail on the bad md; stdout:\n{stdout}"
	);
	assert!(
		stderr.contains("unknown status") && stderr.contains("Q-2"),
		"expected the standalone Markdown validator to reject the superseded status; stderr:\n{stderr}"
	);

	fs::remove_dir_all(&dir).unwrap();
}
