//! Regression tests for `workflow-enforcement-tier-inc1`: the metrics log and the review
//! ledger are resolved from the PLAN SOURCE rather than from the process working
//! directory.
//!
//! Before this increment `--metrics` carried a relative clap `default_value` and
//! `default_ledger_path` built `docs/plans/<task>.ledger.md`, both of which resolve
//! against the CWD. Running any of `validate --workflow`, `status`, `status --resume` or
//! `next` against a plan living in another project therefore joined that plan to THIS
//! directory's log and ledger, with four measured consequences: a `workflow invariants
//! hold` on a plan with no review evidence of its own, a fabricated `next: mark the step
//! complete` instruction, a record count belonging to the wrong project, and one project's
//! `## RESUME STATE` block printed as another project's resume anchor.
//!
//! The cross-project tests build several projects in one scratch tree and run the binary
//! from the WRONG one, so which file was read is identified by CONTENT rather than
//! asserted from the path: each project's log carries a different record count, and only
//! `home`'s log has a converged round for `borrowed-step`.

use std::{
	fs,
	path::{
		Path,
		PathBuf,
	},
	process::Command,
};

/// A TOML-primary plan whose single step carries `borrowed-step`, the slug `home`'s log
/// has converged rounds for. Borrowing a slug is what turns "reads the wrong log" into a
/// measurable false pass: W3 joins a round to a step by slug alone.
fn plan_toml(status: &str) -> String {
	format!(
		"[meta]\ntitle = \"A borrowed-slug project\"\nprimary = \"toml\"\n\n\
		 [[step]]\nslug = \"borrowed-step\"\ntitle = \"The only step\"\nstatus = \"{status}\"\norder = 1\n"
	)
}

/// One schema-valid, converged `low_risk` round record for `task` (which is also its
/// structured step and increment id), so a step of that slug reads as converged to both
/// W3 and `next`.
fn round(task: &str) -> String {
	format!(
		"{{\"type\":\"round\",\"task\":\"{task}\",\"step\":\"{task}\",\"increment\":\"{task}\",\
		 \"artifact\":\"a\",\"phase\":\"work_review\",\"changed_since_prev\":true,\"outcome\":\"clean\",\
		 \"valid_findings\":0,\"severities\":[],\"consecutive_clean\":1,\"risk_class\":\"low_risk\"}}\n"
	)
}

/// A round log built from one record per named task. The RECORD COUNT is what identifies
/// the file in the projections' output, so every fixture log below has a distinct length.
fn log(tasks: &[&str]) -> String {
	tasks.iter().map(|task| round(task)).collect()
}

/// A `## RESUME STATE` block naming its own project, so a leaked block is identifiable in
/// the output rather than merely present.
fn resume_block(which: &str) -> String {
	format!("## RESUME STATE\n\n{which} resume state.\n")
}

/// A unique scratch directory for one test, under the system temp dir (which the suite
/// requires to be outside a git repository).
fn scratch(name: &str) -> PathBuf {
	let dir = std::env::temp_dir().join(format!(
		"agent-scaffold-anchor-{}-{}-{}",
		name,
		std::process::id(),
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
	));
	let _ = fs::remove_dir_all(&dir);
	fs::create_dir_all(&dir).unwrap();
	dir
}

/// Write `contents` at `path`, creating its parent directories.
fn write(
	path: &Path,
	contents: &str,
) {
	fs::create_dir_all(path.parent().unwrap()).unwrap();
	fs::write(path, contents).unwrap();
}

/// `path` as a `&str` for the command line.
fn arg(path: &Path) -> String {
	path.to_str().unwrap().to_string()
}

/// The `home` project: a conventional `<root>/docs/plans` layout whose log holds THREE
/// records including the converged round for `borrowed-step`, plus its own plan and
/// ledger for task `p`. This is the directory the tests RUN FROM, standing in for the
/// repository a user happens to be sitting in when they point the tool at another
/// project's plan.
fn build_home(root: &Path) -> PathBuf {
	let home = root.join("home");
	write(
		&home.join("docs").join("metrics").join("workflow.jsonl"),
		&log(&["borrowed-step", "other-step", "third-step"]),
	);
	write(&home.join("docs").join("plans").join("p.plan.toml"), &plan_toml("complete"));
	write(&home.join("docs").join("plans").join("p.ledger.md"), &resume_block("HOME"));
	home
}

/// The `away` project: the same conventional layout and the same task name `p`, its
/// single step carrying the borrowed slug at `status`, its OWN one-record log with no
/// evidence for that slug, and its own resume block.
fn build_away(
	root: &Path,
	status: &str,
) -> PathBuf {
	let away = root.join("away");
	write(&away.join("docs").join("metrics").join("workflow.jsonl"), &log(&["unrelated-step"]));
	write(&away.join("docs").join("plans").join("p.plan.toml"), &plan_toml(status));
	write(&away.join("docs").join("plans").join("p.ledger.md"), &resume_block("AWAY"));
	away
}

/// Run the built binary with `args` in `dir`, returning `(exit_code, stdout, stderr)`.
fn run(
	dir: &Path,
	args: &[&str],
) -> (Option<i32>, String, String) {
	let output = Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))
		.args(args)
		.current_dir(dir)
		.output()
		.expect("run agent-scaffold");
	(
		output.status.code(),
		String::from_utf8_lossy(&output.stdout).into_owned(),
		String::from_utf8_lossy(&output.stderr).into_owned(),
	)
}

/// Acceptance checks 3 and 4: the cross-project false pass is dead, and what replaces it
/// is the CORRECT red rather than merely the absence of a green.
///
/// RED before the change: `docs/metrics/workflow.jsonl: 3 records, valid` plus
/// `<away plan> vs docs/metrics/workflow.jsonl: workflow invariants hold` at exit 0, for a
/// plan whose project has no review evidence for the step it marks `complete`.
#[test]
fn validate_workflow_reads_the_plans_own_log_not_the_working_directorys() {
	let root = scratch("validate");
	let home = build_home(&root);
	let away = build_away(&root, "complete");
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));
	let away_log = arg(&away.join("docs").join("metrics").join("workflow.jsonl"));

	let (code, stdout, stderr) = run(&home, &["validate", "--source", &away_plan, "--workflow"]);

	assert!(
		!stdout.contains("workflow invariants hold"),
		"a foreign plan must never be declared to hold against this directory's log; stdout:\n{stdout}"
	);
	assert_eq!(
		code,
		Some(1),
		"the check now runs against the plan's own log and must fail it; stdout:\n{stdout}\nstderr:\n{stderr}"
	);
	// The problem line is prefixed `<source> vs <metrics>`, so this asserts WHICH log was
	// read, not merely that something failed.
	assert!(
		stderr.contains(&away_log),
		"expected the failure to name the plan's own log {away_log}; stderr:\n{stderr}"
	);
	assert!(
		stderr.contains("`borrowed-step` is `complete` but has no round records"),
		"expected W3's correct red for the borrowed slug; stderr:\n{stderr}"
	);
	// `home`'s log is three records; nothing may report it.
	assert!(
		!stdout.contains("3 records") && !stderr.contains("3 records"),
		"this directory's log must not be read at all; stdout:\n{stdout}\nstderr:\n{stderr}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 5: `next` no longer fabricates an instruction from a foreign log on
/// the default path. The step is at `in-progress` so the loop is derived from round
/// records, which is the case that fabricates.
///
/// RED before the change: `metrics: 3 records`, `state: converged`, `streak: 1/1`, and
/// `next: mark the step complete, re-render, and commit`, at exit 0, for a project with
/// zero rounds of its own.
#[test]
fn next_projects_the_loop_from_the_plans_own_log() {
	let root = scratch("next");
	let home = build_home(&root);
	let away = build_away(&root, "in-progress");
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));

	let (code, stdout, stderr) = run(&home, &["next", "--source", &away_plan]);

	assert_eq!(code, Some(0), "`next` is a projection; stderr:\n{stderr}");
	assert!(
		!stdout.contains("state: converged"),
		"a foreign log must not converge this plan's loop; stdout:\n{stdout}"
	);
	assert!(
		!stdout.contains("mark the step complete"),
		"the fabricated completion instruction must be unreachable; stdout:\n{stdout}"
	);
	// The count identifies the file: `away`'s own log is one record, `home`'s is three.
	assert!(
		stdout.contains("metrics: 1 records"),
		"expected the plan's own log to be summarised; stdout:\n{stdout}"
	);
	// The state that FOLLOWS from the right log, asserted positively: `away`'s log has no
	// round for this slug, so the first review round is what is owed. Asserting only the
	// absence of the wrong answer would pass against a build that read no log at all.
	assert!(
		stdout.contains("state: awaiting-first-review"),
		"expected the state implied by the plan's own log; stdout:\n{stdout}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 6: `status` counts the plan's own log, whether the anchor comes from
/// `--source` or from `--plan`, and `--source` wins when both are given (the
/// source-then-plan order `next::derive_task` already uses).
///
/// RED before the change: `metrics: 3 records` on all three invocations.
#[test]
fn status_counts_the_plans_own_log_from_either_anchor() {
	let root = scratch("status");
	let home = build_home(&root);
	let away = build_away(&root, "in-progress");
	write(&away.join("docs").join("plans").join("p.md"), "# away plan\n");
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));
	let away_markdown = arg(&away.join("docs").join("plans").join("p.md"));

	let (code, stdout, stderr) = run(&home, &["status", "--source", &away_plan]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("metrics: 1 records"),
		"expected the --source anchor; stdout:\n{stdout}"
	);

	// The Markdown `--plan` anchors identically, so one rule covers both substrates.
	let (code, stdout, stderr) = run(&home, &["status", "--plan", &away_markdown]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("metrics: 1 records"), "expected the --plan anchor; stdout:\n{stdout}");

	// With BOTH, `--source` wins: the `--plan` here is `home`'s own three-record project,
	// so a count of 1 is the source anchor and a count of 3 would be the plan anchor.
	let (code, stdout, stderr) =
		run(&home, &["status", "--source", &away_plan, "--plan", "docs/plans/p.md"]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("metrics: 1 records"),
		"--source must win over --plan as the anchor; stdout:\n{stdout}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 7: the ledger resolves BESIDE the plan source, so one project's
/// `## RESUME STATE` block can no longer be printed as another project's resume anchor ON
/// THE DEFAULT PATH. Both readers are covered, since `next` echoes the same block
/// `status --resume` prints.
///
/// RED before the change: both commands print `HOME resume state.`, this directory's
/// internal resume state, as the anchor for an unrelated project.
#[test]
fn the_ledger_resolves_beside_the_plan_source() {
	let root = scratch("ledger");
	let home = build_home(&root);
	let away = build_away(&root, "in-progress");
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));

	let (code, stdout, stderr) = run(&home, &["status", "--resume", "--source", &away_plan]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("AWAY resume state."),
		"expected the plan's own ledger; stdout:\n{stdout}"
	);
	assert!(
		!stdout.contains("HOME resume state."),
		"this directory's resume state must not leak into a foreign project's brief; stdout:\n{stdout}"
	);

	let (code, stdout, stderr) = run(&home, &["next", "--source", &away_plan]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("AWAY resume state."),
		"`next` echoes the same block and must read the same ledger; stdout:\n{stdout}"
	);
	assert!(!stdout.contains("HOME resume state."), "stdout:\n{stdout}");

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 8 (`Q-55-noconvention`): a plan at a project root with NO `docs/plans`
/// directory falls back to the source's own directory as the root, reading that root's own
/// `docs/metrics/workflow.jsonl`, both from elsewhere and from that root itself.
///
/// RED before the change on the from-elsewhere run (`metrics: 3 records`, this directory's
/// log). The from-its-own-root run is a pin: the fallback gives the same answer the
/// historical CWD-relative default gave from the right directory, which is the reason it
/// was chosen over a hard error.
#[test]
fn a_source_with_no_docs_plans_ancestor_falls_back_to_its_own_directory() {
	let root = scratch("noconvention");
	let home = build_home(&root);
	let flat = root.join("flat");
	write(
		&flat.join("docs").join("metrics").join("workflow.jsonl"),
		&log(&["flat-one", "flat-two"]),
	);
	write(&flat.join("myplan.plan.toml"), &plan_toml("complete"));
	let flat_plan = arg(&flat.join("myplan.plan.toml"));

	let (code, stdout, stderr) = run(&home, &["status", "--source", &flat_plan]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("metrics: 2 records"),
		"expected the conventionless root's own log; stdout:\n{stdout}"
	);

	let (code, stdout, stderr) = run(&flat, &["status", "--source", "myplan.plan.toml"]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("metrics: 2 records"),
		"the same answer from the plan's own root; stdout:\n{stdout}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// The nested-`docs/plans` case: a project vendored under another project's plan directory
/// resolves NEAREST-WINS to the inner root.
///
/// This pins a JUDGEMENT, not a measurement. The rule has to answer this input somehow and
/// the answer must be deterministic, so it is asserted here where a reader can see it;
/// nothing outside this test establishes that the inner project is the right answer, and a
/// later reader with evidence may change both.
///
/// RED before the change: `metrics: 3 records`, this directory's log, for either reading.
#[test]
fn a_nested_docs_plans_resolves_to_the_inner_project() {
	let root = scratch("nested");
	let home = build_home(&root);
	let outer = root.join("outer");
	let inner = outer.join("docs").join("plans").join("vendor");
	write(
		&outer.join("docs").join("metrics").join("workflow.jsonl"),
		&log(&["o1", "o2", "o3", "o4", "o5", "o6"]),
	);
	write(
		&inner.join("docs").join("metrics").join("workflow.jsonl"),
		&log(&["i1", "i2", "i3", "i4"]),
	);
	write(&inner.join("docs").join("plans").join("inner.plan.toml"), &plan_toml("complete"));
	let inner_plan = arg(&inner.join("docs").join("plans").join("inner.plan.toml"));

	let (code, stdout, stderr) = run(&home, &["status", "--source", &inner_plan]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("metrics: 4 records"),
		"nearest-wins selects the inner project's log (4), not the outer's (6); stdout:\n{stdout}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 9, the Safe on existing projects pin: a run made from the plan's own
/// project root with a BARE RELATIVE `--source`, which is the normal invocation, is
/// UNCHANGED, byte for byte. The whole stdout is compared rather than searched, because the
/// property is that the printed paths stay RELATIVE: an "improvement" that canonicalised
/// the default would still read the right file and still pass a `contains` assertion while
/// changing two of these three lines to absolute, machine-specific paths.
///
/// A pin, not a red-then-green case: it passes identically before and after the change.
#[test]
fn the_correct_case_prints_the_same_relative_paths_it_always_did() {
	let root = scratch("noregression");
	let home = build_home(&root);

	let (code, stdout, stderr) =
		run(&home, &["validate", "--source", "docs/plans/p.plan.toml", "--workflow"]);

	assert_eq!(code, Some(0), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert_eq!(
		stdout,
		"docs/metrics/workflow.jsonl: 3 records, valid\n\
		 docs/plans/p.plan.toml: 1 steps, 0 questions, valid\n\
		 docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold\n",
		"this spelling's output must be byte-identical to the pre-anchoring binary's"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 10: plain `validate` (no `--workflow`) is unaffected by the tier
/// policy and still exits 0 with a stderr note on a missing log, and a bare `validate`
/// with NO plan source has nothing to anchor to and keeps the historical
/// current-directory-relative path.
///
/// The anchored-but-missing case is red-then-green (before the change this read this
/// directory's three-record log and printed it as valid); the bare-`validate` case is a
/// pin on the no-anchor rule.
#[test]
fn plain_validate_and_a_sourceless_run_keep_their_behaviour() {
	let root = scratch("plain");
	let home = build_home(&root);
	let nolog = root.join("nolog");
	write(&nolog.join("docs").join("plans").join("p.plan.toml"), &plan_toml("not-started"));
	let nolog_plan = arg(&nolog.join("docs").join("plans").join("p.plan.toml"));
	let nolog_log = arg(&nolog.join("docs").join("metrics").join("workflow.jsonl"));

	// Anchored, and the anchored log does not exist: a note and exit 0, because nobody
	// asked for the workflow check.
	let (code, stdout, stderr) = run(&home, &["validate", "--source", &nolog_plan]);
	assert_eq!(code, Some(0), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(
		stderr.contains(&format!("no metrics log at {nolog_log}")),
		"expected the note to name the plan's own missing log; stderr:\n{stderr}"
	);
	assert!(
		!stdout.contains("3 records"),
		"this directory's log must not stand in for the plan's; stdout:\n{stdout}"
	);

	// No `--source` and no `--plan`: nothing to anchor to, so the historical
	// `docs/metrics/workflow.jsonl` relative to the current directory stands.
	let (code, stdout, stderr) = run(&home, &["validate"]);
	assert_eq!(code, Some(0), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert_eq!(
		stdout, "docs/metrics/workflow.jsonl: 3 records, valid\n",
		"a sourceless run keeps the current-directory-relative path"
	);

	// No `--source` and no `--plan` on the ledger side either: `<task>` falls back to
	// `task` and the historical `docs/plans/<task>.ledger.md` stands.
	let (code, stdout, stderr) = run(&home, &["status", "--resume"]);
	assert_eq!(code, Some(0), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert_eq!(
		stdout, "no ledger at docs/plans/task.ledger.md; nothing to resume\n",
		"a sourceless resume keeps the current-directory-relative ledger path"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Accepted cost (i), pinned as EXPECTED behaviour rather than fixed (acceptance check
/// 18's inc1 half): a bare filename run from inside `docs/plans` has no parents to walk,
/// falls back to the source's own directory, and looks for `docs/metrics/workflow.jsonl`
/// beneath it, which does not exist. The project's real log is never read.
///
/// This is not a regression (the pre-change build was identically wrong here) and the fix
/// is NOT to canonicalise the default: doing so would turn the printed paths of the
/// correct case absolute, which is what the test above pins. This test exists so that
/// change fails loudly here too.
#[test]
fn a_bare_filename_from_inside_docs_plans_stays_a_silent_miss() {
	let root = scratch("barefilename");
	let away = build_away(&root, "complete");
	let plans_dir = away.join("docs").join("plans");

	let (code, stdout, stderr) =
		run(&plans_dir, &["validate", "--source", "p.plan.toml", "--workflow"]);

	assert_eq!(code, Some(0), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(
		stderr.contains("no metrics log at docs/metrics/workflow.jsonl"),
		"expected the miss note naming the path it looked for; stderr:\n{stderr}"
	);
	assert!(
		!stdout.contains("records, valid"),
		"the project's real log must not be reached from here; stdout:\n{stdout}"
	);

	let _ = fs::remove_dir_all(&root);
}
