//! Regression tests for `validate --workflow` refusing to report success for a check it
//! never ran, on EITHER of the two inputs that check needs.
//!
//! The file opened on the Inc 6 clap relaxation: `--workflow` no longer `requires`
//! `--plan`, so a TOML-primary project with NO Markdown plan can run
//! `validate --workflow --source <plan.toml>` end to end. Before the relaxation clap
//! rejected the combination with a usage error (exit 2) because `--workflow` was
//! declared `requires = "plan"`. The rule the file pins is the general one rather than a
//! rule about the plan source alone: `--workflow` was explicitly requested, so an input
//! it needs and cannot find is a reported problem at exit 1, never a note at exit 0 that
//! a CI gate reads as a pass.
//!
//! This pins three directions:
//! - Positive: the TOML-primary `--workflow --source` with no `--plan` reaches the
//!   workflow check and passes (exit 0, `workflow invariants hold`).
//! - Negative, THE PLAN SOURCE missing: `--workflow` with no resolvable plan source,
//!   whether no `--source`/`--plan` at all or a typo'd/missing `--source`, hard-errors
//!   (exit 1) naming the missing plan source. This is the false-green regression the
//!   relaxation would otherwise have opened (Inc 6 M-1).
//! - Negative, THE ROUND LOG missing: a plan source that resolves with no round log at
//!   the resolved path hard-errors (exit 1) naming that path, on the TOML arm and the
//!   Markdown arm alike (`workflow-enforcement-tier-inc3`, the tier policy). Plain
//!   `validate` without `--workflow` keeps its stderr note and its exit 0, because
//!   nobody asked for the check there. A path the tool cannot answer that question for gets
//!   the same exit 1 and a different sentence, since asserting absence there would be false.

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

/// A minimal, schema-valid Markdown `--plan` holding one `not started` Roadmap step, so
/// the Markdown arm of the same match reaches the workflow check with nothing to enforce.
/// Only its PRESENCE matters below: the tier policy answers before any check runs.
const PLAN_MD: &str = "\
# A plan

## Roadmap

| Step | Status |
| --- | --- |
| `only-step` | not started |

## Step Detail

### `only-step`: The only step

Body.
";

/// One schema-valid `round` record for `PLAN_TOML`'s only step, so a fixture log built from
/// it is a real round log rather than an empty file.
const ROUND_RECORD: &str = "\
{\"type\":\"round\",\"task\":\"only-step\",\"step\":\"only-step\",\"increment\":\"only-step\",\
\"artifact\":\"a\",\"phase\":\"work_review\",\"changed_since_prev\":true,\"outcome\":\"clean\",\
\"valid_findings\":0,\"severities\":[],\"consecutive_clean\":1,\"risk_class\":\"low_risk\"}
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
	// A present (empty) metrics log, so the only thing missing is the plan source. That is
	// what keeps this test's red attributable: a missing log is now its own hard error (the
	// test below), so a present log is what proves THIS hard error is about the plan source.
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

/// Acceptance checks 15 and 16 (`workflow-enforcement-tier-inc3`, the tier policy): a
/// `--workflow` run whose ROUND LOG is missing reports a problem and exits non-zero, and
/// plain `validate` on the same missing log is untouched.
///
/// RED before the change: all three `--workflow` runs below printed `--workflow has a plan
/// source but the metrics log is missing; skipping the workflow check` on stderr and
/// exited 0, so a CI gate reading the exit status recorded a pass for a project with zero
/// machine enforcement of any workflow invariant.
///
/// The runs are the BOUNDARY, not one case. The `_` catch-all this converts covers both
/// the TOML-source-present and the Markdown-plan-present variants of "log missing", and
/// the policy applies to the resolved path however it was resolved, so a `--metrics` the
/// caller named that is not there is not a weaker case than a default that is not there.
/// The fourth run is the control that keeps the change off the no-`--workflow` path, where
/// nobody asked for the check and an absent log stays a note at exit 0.
#[test]
fn workflow_with_no_metrics_log_hard_errors_instead_of_skipping() {
	let dir = std::env::temp_dir().join(format!(
		"agent-scaffold-validate-workflow-no-log-{}-{}",
		std::process::id(),
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
	));
	fs::create_dir_all(&dir).unwrap();
	fs::write(dir.join("plan.plan.toml"), PLAN_TOML).unwrap();
	fs::write(dir.join("plan.md"), PLAN_MD).unwrap();
	// NO round log anywhere: neither the anchored default (`docs/metrics/workflow.jsonl`
	// under this directory, which is the root the plan's own directory yields) nor any
	// path named below exists.

	// (a) The TOML arm: a TOML-primary `--source` resolves, the anchored default log does
	// not exist. The problem names the path the tool looked for, so a reader can tell a
	// non-instrumented project from a mis-anchored run.
	let (code, stdout, stderr) = validate(&dir, &["--workflow", "--source", "plan.plan.toml"]);
	assert_eq!(
		code,
		Some(1),
		"--workflow with no round log must not exit 0; stdout:\n{stdout}\nstderr:\n{stderr}"
	);
	assert!(
		stderr.contains("no round log at docs/metrics/workflow.jsonl")
			&& stderr.contains("could not run"),
		"expected a problem naming the resolved log and saying the check could not run; stderr:\n{stderr}"
	);

	// (b) The Markdown arm of the same catch-all: a readable `--plan` and no `--source`.
	let (code, stdout, stderr) = validate(&dir, &["--workflow", "--plan", "plan.md"]);
	assert_eq!(
		code,
		Some(1),
		"the Markdown arm must answer the same way; stdout:\n{stdout}\nstderr:\n{stderr}"
	);
	assert!(
		stderr.contains("no round log at docs/metrics/workflow.jsonl")
			&& stderr.contains("could not run"),
		"expected the same problem on the Markdown arm; stderr:\n{stderr}"
	);

	// (c) An EXPLICIT `--metrics` that does not exist, inside the plan's own root so the
	// containment guard is not what answers. Naming a path is not a weaker case than
	// defaulting to one: the check still cannot run.
	let (code, stdout, stderr) =
		validate(&dir, &["--workflow", "--source", "plan.plan.toml", "--metrics", "absent.jsonl"]);
	assert_eq!(
		code,
		Some(1),
		"an explicit --metrics that is not there must not exit 0; stdout:\n{stdout}\nstderr:\n{stderr}"
	);
	assert!(
		stderr.contains("no round log at absent.jsonl") && stderr.contains("could not run"),
		"expected the problem to name the path the caller gave; stderr:\n{stderr}"
	);

	// (d) THE CONTROL: the same missing log with no `--workflow`. Nobody asked for the
	// check, so this keeps its stderr note and its exit 0 (acceptance check 16). This is
	// the half of the tier policy that is easiest to break by accident.
	let (code, stdout, stderr) = validate(&dir, &["--source", "plan.plan.toml"]);
	assert_eq!(
		code,
		Some(0),
		"plain validate must be untouched by the tier policy; stdout:\n{stdout}\nstderr:\n{stderr}"
	);
	assert!(
		stderr.contains("no metrics log at docs/metrics/workflow.jsonl; nothing to validate"),
		"expected the unchanged skip note; stderr:\n{stderr}"
	);
	assert!(
		!stderr.contains("could not run"),
		"the tier policy must not reach a run that did not ask for the check; stderr:\n{stderr}"
	);

	fs::remove_dir_all(&dir).unwrap();
}

/// A ROUND LOG THE TOOL CANNOT ASK ABOUT IS NOT A ROUND LOG THAT IS ABSENT
/// (`Q-55-existsgate`). The gate the policy above hangs on is `Path::exists`, which is
/// `fs::metadata(..).is_ok()` and so answers false both for a log that is not there and for
/// one behind a directory the process cannot traverse. Asserting absence for the second, and
/// then telling the operator to record rounds, sends them to fix a path that is already
/// correct with evidence already on it.
///
/// THE FIXTURE IS MODE 600 ON THE LOG'S OWN DIRECTORY, not 000: the directory stays READABLE
/// and stops being SEARCHABLE, so the log is listable by name while `fs::metadata` on it
/// fails with EACCES. Mode 111 is the matching negative control (searchable, not readable),
/// where the read succeeds and the run is the correct one; the discriminator is search
/// permission on an ancestor and nothing about the log's own mode.
///
/// RED before this commit: the prior build printed `no round log at
/// docs/metrics/workflow.jsonl ... record the project's review rounds there` at exit 1 for
/// the log the closing control then reads clean at exit 0.
#[cfg(unix)]
#[test]
fn a_round_log_that_cannot_be_checked_is_not_reported_as_missing() {
	use std::os::unix::fs::PermissionsExt;

	let dir = std::env::temp_dir().join(format!(
		"agent-scaffold-validate-workflow-opaque-log-{}-{}",
		std::process::id(),
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
	));
	let plans = dir.join("docs").join("plans");
	let metrics = dir.join("docs").join("metrics");
	fs::create_dir_all(&plans).unwrap();
	fs::create_dir_all(&metrics).unwrap();
	fs::write(plans.join("p.plan.toml"), PLAN_TOML).unwrap();
	// A REAL log, not an empty file, so what sits behind the error is round evidence rather
	// than a placeholder: the falsehood is about a project that IS instrumented.
	fs::write(metrics.join("workflow.jsonl"), ROUND_RECORD).unwrap();

	fs::set_permissions(&metrics, fs::Permissions::from_mode(0o600)).unwrap();
	// Whether the mode actually hides the entry, measured rather than assumed: as root it
	// does not, and then the log is simply THERE, which is the correct case and has nothing
	// for this test to say.
	let opaque = fs::metadata(metrics.join("workflow.jsonl")).is_err();
	let (code, stdout, stderr) =
		validate(&dir, &["--workflow", "--source", "docs/plans/p.plan.toml"]);
	// RESTORED BEFORE THE ASSERTIONS, so a failing one cannot leave an undeletable directory.
	fs::set_permissions(&metrics, fs::Permissions::from_mode(0o755)).unwrap();

	if opaque {
		// The exit code is NOT what changes: the check genuinely could not run either way, so
		// non-zero stays right and only the diagnosis was false.
		assert_eq!(
			code,
			Some(1),
			"a check that could not run must still refuse; stdout:\n{stdout}\nstderr:\n{stderr}"
		);
		assert!(
			!stderr.contains("no round log at"),
			"the log is on disk, so this sentence is false; stderr:\n{stderr}"
		);
		assert!(
			stderr.contains("could not be checked"),
			"and the operator is still told the answer is unknown; stderr:\n{stderr}"
		);
		assert!(
			!stderr.contains("record the project's review rounds"),
			"the rounds are already recorded at that path; stderr:\n{stderr}"
		);
	}

	// THE CONTROL, on the same fixture with nothing but the mode changed back: the log was
	// there, readable and valid, for the whole run above.
	let (code, stdout, stderr) =
		validate(&dir, &["--workflow", "--source", "docs/plans/p.plan.toml"]);
	assert_eq!(code, Some(0), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(
		stdout.contains("workflow invariants hold"),
		"the log behind the error was a working one; stdout:\n{stdout}"
	);

	fs::remove_dir_all(&dir).unwrap();
}
