//! Regression tests for `workflow-enforcement-tier-inc2`: the containment predicate and
//! its four consumers.
//!
//! Inc1 changed where the DEFAULT `--metrics` and ledger resolve, which closed the
//! reproduction the step was opened on and left the DEFECT CLASS open: an explicit
//! `--metrics` naming a foreign log, a symlinked source borrowing its neighbours' log, a
//! `..` that climbs out of the plan's root, and a `--source`/`--plan` pair belonging to
//! two different projects all still joined one project's plan to another's evidence. The
//! predicate here asks one question, of the resolved artifact against the CANONICAL root
//! of the plan THAT SURFACE READS, and each surface answers it differently:
//! `validate --workflow` REFUSES (exit non-zero), `status` and `next` OMIT the affected
//! part with a reason at exit 0, and `status --resume` omits its block.
//!
//! Which file was read is identified by CONTENT rather than asserted from the path: every
//! fixture log carries a different record count, and only `home`'s and `away_source`'s
//! logs hold a converged round for `borrowed-step`.
//!
//! The three symlink and cost tests pin ACCEPTED COSTS as expected behaviour. They are
//! not defects to fix: a loud refusal on a layout that works today is the judgement the
//! mechanism was accepted with, and a later "improvement" that quietly removes it should
//! fail here.

use std::{
	fs,
	path::{
		Path,
		PathBuf,
	},
	process::Command,
};

/// A TOML-primary plan whose single step carries `borrowed-step`, the slug the fixture
/// logs below have converged rounds for. Borrowing a slug is what turns "reads the wrong
/// log" into a measurable false pass: W3 joins a round to a step by slug alone.
fn plan_toml(status: &str) -> String {
	format!(
		"[meta]\ntitle = \"A borrowed-slug project\"\nprimary = \"toml\"\n\n\
		 [[step]]\nslug = \"borrowed-step\"\ntitle = \"The only step\"\nstatus = \"{status}\"\norder = 1\n"
	)
}

/// The same plan declared MARKDOWN-primary, so the `--workflow` check reads the Markdown
/// `--plan` instead of this file while the metrics anchor still comes from here. That
/// divergence is the whole point of `Q-55-endproperty`.
fn plan_toml_markdown_primary() -> String {
	"[meta]\ntitle = \"A Markdown-primary project\"\nprimary = \"markdown\"\n\n\
	 [[step]]\nslug = \"its-own-step\"\ntitle = \"Its own step\"\nstatus = \"not-started\"\norder = 1\n"
		.to_string()
}

/// A minimal, schema-valid Markdown plan whose Roadmap carries `borrowed-step` at
/// `status`, with the matching Step Detail heading.
fn plan_markdown(status: &str) -> String {
	format!(
		"# A plan\n\n## Roadmap\n\n| Step | Status |\n| --- | --- |\n| `borrowed-step` | {status} |\n\n\
		 ## Step Detail\n\n### `borrowed-step`: The only step\n\nBody.\n"
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
		"agent-scaffold-containment-{}-{}-{}",
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
/// records including the converged round for `borrowed-step`. This is the directory the
/// tests RUN FROM, standing in for the repository a user happens to be sitting in when
/// they point the tool at another project's plan, and its log is the foreign evidence an
/// explicit `--metrics` reaches for.
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
/// single step carrying the borrowed slug at `status`, and NO log of its own, so any
/// record count in the output belongs to another project.
fn build_away(
	root: &Path,
	status: &str,
) -> PathBuf {
	let away = root.join("away");
	write(&away.join("docs").join("plans").join("p.plan.toml"), &plan_toml(status));
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

/// Acceptance check 11, the validator's response and the first of inc2's four owed
/// red-then-green cases.
///
/// RED against the parent commit (inc1 landed, inc2 not): `home`'s three-record log is
/// read verbatim from the explicit relative `--metrics`, its converged `borrowed-step`
/// round satisfies `away`'s completion claim, and the run prints
/// `<away plan> vs docs/metrics/workflow.jsonl: workflow invariants hold` at exit 0.
/// Anchoring alone cannot reach this: an explicit value is honoured verbatim by design.
#[test]
fn an_explicit_metrics_outside_the_plans_root_is_refused() {
	let root = scratch("explicit");
	let home = build_home(&root);
	let away = build_away(&root, "complete");
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));

	let (code, stdout, stderr) = run(
		&home,
		&[
			"validate",
			"--source",
			&away_plan,
			"--metrics",
			"docs/metrics/workflow.jsonl",
			"--workflow",
		],
	);

	assert_eq!(code, Some(1), "an unpairable log is refused; stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(
		!stdout.contains("workflow invariants hold"),
		"nothing may be asserted about the pairing; stdout:\n{stdout}"
	);
	// The refusal names all three: the checked plan, the log, and the derived root.
	assert!(stderr.contains(&away_plan), "the refusal names the checked plan; stderr:\n{stderr}");
	assert!(
		stderr.contains("docs/metrics/workflow.jsonl"),
		"the refusal names the resolved log; stderr:\n{stderr}"
	);
	assert!(
		stderr.contains(&arg(&away)),
		"the refusal names the derived project root; stderr:\n{stderr}"
	);
	assert!(
		stderr.contains("pass a `--metrics` under that root"),
		"the refusal tells the user what to do; stderr:\n{stderr}"
	);

	// THE REFUSAL REPLACES THE FOUR-ARM MATCH RATHER THAN ACCOMPANYING IT. Asserting
	// anything about the pairing IN EITHER DIRECTION is what has to stop, so the refusal
	// must be the only problem reported. A foreign log whose records do NOT satisfy the
	// borrowed slug is what separates the two readings: run the match beside the refusal
	// and W3 reports a verdict on the very pairing just declared unvouchable, at the same
	// exit code, which is why the exit code alone cannot see the difference.
	write(&home.join("docs").join("metrics").join("other.jsonl"), &log(&["other-step"]));
	let (code, stdout, stderr) = run(
		&home,
		&[
			"validate",
			"--source",
			&away_plan,
			"--metrics",
			"docs/metrics/other.jsonl",
			"--workflow",
		],
	);
	assert_eq!(code, Some(1), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(stderr.contains("is not under the plan's project root"), "stderr:\n{stderr}");
	assert!(
		!stderr.contains("has no round records"),
		"no W3 verdict on a pairing the tool just said it cannot vouch for; stderr:\n{stderr}"
	);
	assert_eq!(
		stderr.lines().count(),
		1,
		"the refusal is the ONLY problem reported; stderr:\n{stderr}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 13b, the case `Q-55-endproperty` exists for and the second owed
/// red-then-green case. It is what SEPARATES a predicate rooted on the checked plan from
/// one rooted on the metrics anchor: `alpha`'s log is under `alpha`'s root by
/// construction, so an anchor-rooted predicate can never fire here.
///
/// RED against the parent commit: `beta`'s Markdown plan is checked against `alpha`'s
/// log (the anchor is the `--source`, the checked plan is the `--plan`) and prints
/// `workflow invariants hold` at exit 0 for a step with no round record of its own.
#[test]
fn a_divergent_source_and_plan_pairing_is_refused() {
	let root = scratch("divergent");
	let home = build_home(&root);

	// `alpha`: a MARKDOWN-primary source beside a log that does hold the converged round.
	let alpha = root.join("alpha");
	write(&alpha.join("docs").join("plans").join("p.plan.toml"), &plan_toml_markdown_primary());
	write(
		&alpha.join("docs").join("metrics").join("workflow.jsonl"),
		&log(&["borrowed-step", "alpha-two"]),
	);
	let alpha_source = arg(&alpha.join("docs").join("plans").join("p.plan.toml"));
	let alpha_log = arg(&alpha.join("docs").join("metrics").join("workflow.jsonl"));

	// `beta`: the Markdown plan that is actually CHECKED, claiming the borrowed step
	// complete with no evidence of its own anywhere.
	let beta = root.join("beta");
	write(&beta.join("docs").join("plans").join("p.md"), &plan_markdown("complete"));
	let beta_plan = arg(&beta.join("docs").join("plans").join("p.md"));

	let (code, stdout, stderr) =
		run(&home, &["validate", "--source", &alpha_source, "--plan", &beta_plan, "--workflow"]);
	assert_eq!(code, Some(1), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(!stdout.contains("workflow invariants hold"), "stdout:\n{stdout}");
	assert!(stderr.contains(&beta_plan), "the refusal names the CHECKED plan; stderr:\n{stderr}");
	assert!(stderr.contains(&alpha_log), "the refusal names the log; stderr:\n{stderr}");
	assert!(stderr.contains(&arg(&beta)), "the root is beta's; stderr:\n{stderr}");
	// The third remedy, added for exactly this cause: neither of the other two names a
	// `--source` and a `--plan` that belong to different projects.
	assert!(stderr.contains("or correct the `--source` and `--plan` pair"), "stderr:\n{stderr}");

	// A TYPO'D `--source`: nothing is read from it, so the root comes from the `--plan`
	// that WAS read while the log still comes from the lexical derivation on the path that
	// was not. A two-root comparison could not reach this at all (a path that does not
	// exist yields no canonical root to compare).
	let typo = arg(&alpha.join("docs").join("plans").join("p-typo.plan.toml"));
	let (code, stdout, stderr) =
		run(&home, &["validate", "--source", &typo, "--plan", &beta_plan, "--workflow"]);
	assert_eq!(code, Some(1), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(stderr.contains(&alpha_log), "stderr:\n{stderr}");

	// The NO-REGRESSION side: one project's two substrates, with the source TOML-primary,
	// so the checked plan IS the anchor and the rule reduces to the anchor-rooted one.
	write(&home.join("docs").join("plans").join("p.md"), &plan_markdown("complete"));
	let (code, stdout, stderr) = run(
		&home,
		&[
			"validate",
			"--source",
			"docs/plans/p.plan.toml",
			"--plan",
			"docs/plans/p.md",
			"--workflow",
		],
	);
	assert_eq!(code, Some(0), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(
		stdout.contains("workflow invariants hold"),
		"a same-project pair still reads its own log; stdout:\n{stdout}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 12: a symlink to a project's plan, placed outside that project with a
/// full log sitting beside the SYMLINK, cannot borrow its neighbours' evidence. This is
/// why the guard canonicalises: a LEXICAL comparison would see the symlink's own spelling
/// and accept the join.
#[test]
fn a_symlinked_source_cannot_borrow_its_neighbours_log() {
	let root = scratch("symlinksource");
	let home = build_home(&root);

	let proj = root.join("proj");
	write(&proj.join("docs").join("plans").join("p.plan.toml"), &plan_toml("complete"));
	// The project's own log is empty, so the ONLY converged evidence anywhere near this
	// run is the log beside the symlink.
	write(&proj.join("docs").join("metrics").join("workflow.jsonl"), "");

	let away = root.join("away");
	write(&away.join("docs").join("metrics").join("workflow.jsonl"), &log(&["borrowed-step"]));
	let link = away.join("p.plan.toml");
	symlink(&proj.join("docs").join("plans").join("p.plan.toml"), &link);

	let (code, stdout, stderr) = run(&home, &["validate", "--source", &arg(&link), "--workflow"]);
	assert_eq!(code, Some(1), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(!stdout.contains("workflow invariants hold"), "stdout:\n{stdout}");
	assert!(
		stderr.contains(&arg(&proj)),
		"the root is the symlink TARGET's project; stderr:\n{stderr}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 13: a `..` that climbs OUT of the plan's root is refused, while a `..`
/// that stays INSIDE it is allowed and produces the correct W3 result. The second half is
/// what keeps this a containment rule rather than a ban on a path component.
#[test]
fn a_dotdot_escape_is_refused_and_one_that_stays_inside_is_not() {
	let root = scratch("dotdot");
	let home = build_home(&root);
	let away = build_away(&root, "complete");
	write(&away.join("docs").join("metrics").join("workflow.jsonl"), &log(&["unrelated"]));
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));

	// Out of `away` and into `home`'s log, which does hold the converged borrowed round.
	let escape = arg(&away
		.join("docs")
		.join("metrics")
		.join("..")
		.join("..")
		.join("..")
		.join("home/docs/metrics/workflow.jsonl"));
	let (code, stdout, stderr) =
		run(&home, &["validate", "--source", &away_plan, "--metrics", &escape, "--workflow"]);
	assert_eq!(code, Some(1), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(!stdout.contains("workflow invariants hold"), "stdout:\n{stdout}");

	// A `..` that stays inside: the same file the default would have found, spelled the
	// long way round. It is allowed, and the check RUNS, giving W3's correct red.
	let inside =
		arg(&away.join("docs").join("plans").join("..").join("metrics").join("workflow.jsonl"));
	let (code, stdout, stderr) =
		run(&home, &["validate", "--source", &away_plan, "--metrics", &inside, "--workflow"]);
	assert_eq!(code, Some(1), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(
		stderr.contains("`borrowed-step` is `complete` but has no round records"),
		"the check must RUN here, not be refused; stderr:\n{stderr}"
	);
	assert!(
		!stderr.contains("is not under the plan's project root"),
		"an in-root `..` is not a containment breach; stderr:\n{stderr}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 14: the REFUSAL is the validator's alone. Without `--workflow` no
/// pairing is asserted, so there is nothing to refuse; and the projections never exit
/// non-zero on any of these inputs, which is the half of `Q-55-refusalscope` that a
/// reviewer meeting the new refusal might otherwise read as a bug.
#[test]
fn the_refusal_is_scoped_to_the_validator() {
	let root = scratch("scope");
	let home = build_home(&root);
	let away = build_away(&root, "complete");
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));
	let local = "docs/metrics/workflow.jsonl";

	let (code, stdout, stderr) =
		run(&home, &["validate", "--source", &away_plan, "--metrics", local]);
	assert_eq!(code, Some(0), "no --workflow, no pairing, no refusal; stderr:\n{stderr}");
	assert!(stdout.contains("3 records, valid"), "the named log is still read; stdout:\n{stdout}");

	for args in [
		vec!["status", "--source", &away_plan, "--metrics", local],
		vec!["next", "--source", &away_plan, "--metrics", local],
		vec!["status", "--json", "--source", &away_plan, "--metrics", local],
		vec!["next", "--json", "--source", &away_plan, "--metrics", local],
		vec![
			"status",
			"--resume",
			"--source",
			&away_plan,
			"--ledger-fragment",
			"docs/plans/p.ledger.md",
		],
	] {
		let (code, stdout, stderr) = run(&home, &args);
		assert_eq!(code, Some(0), "{args:?} must never fail; stdout:\n{stdout}\nstderr:\n{stderr}");
	}

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance checks 14b and 14d, the projections' response in human text and the third
/// owed red-then-green case.
///
/// RED against the parent commit: `metrics: 3 records`, `state: converged`, `streak: 1/1`,
/// `rounds: 1/1` and `next: mark the step complete, re-render, and commit` at exit 0, for
/// a project with zero rounds of its own.
///
/// The trap at 14d is asserted too: the output must NOT be the zero-rounds projection
/// either. Treating an unsafe log as an absent one is the cheap implementation and it
/// fabricates in the other direction, so a test that only asserts the absence of "mark the
/// step complete" passes against the defect.
#[test]
fn next_withholds_the_whole_loop_on_an_unpairable_log() {
	let root = scratch("nextomit");
	let home = build_home(&root);
	let away = build_away(&root, "in-progress");
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));

	let (code, stdout, stderr) =
		run(&home, &["next", "--source", &away_plan, "--metrics", "docs/metrics/workflow.jsonl"]);

	assert_eq!(code, Some(0), "asserted explicitly: `next` never refuses; stderr:\n{stderr}");
	// The block goes as a UNIT. Suppressing only the action would leave the same foreign
	// evidence standing in a quieter form.
	for field in ["state:", "streak:", "rounds:", "next:", "role:", "prompt:", "summary:"] {
		assert!(!stdout.contains(field), "`{field}` must not be emitted; stdout:\n{stdout}");
	}
	assert!(!stdout.contains("3 records"), "no record count either; stdout:\n{stdout}");
	// UNSAFE IS NOT ABSENT: neither the converged instruction nor the zero-rounds one.
	assert!(!stdout.contains("converged"), "stdout:\n{stdout}");
	assert!(
		!stdout.contains("awaiting-first-review"),
		"an unsafe log must not degrade to an absent one; stdout:\n{stdout}"
	);
	// The reason names the resolved log and the derived root, in their place.
	assert!(
		stdout.contains("metrics: unavailable, the round log docs/metrics/workflow.jsonl is not under the plan's project root"),
		"stdout:\n{stdout}"
	);
	assert!(stdout.contains(&arg(&away)), "the derived root is named; stdout:\n{stdout}");
	assert!(stdout.contains("no active review loop ("), "stdout:\n{stdout}");

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 14c: the same predicate on `status` and on `status --resume`. The plan
/// half is untouched and only the unpairable part is left out, which is what makes this an
/// omission rather than a failure.
#[test]
fn status_omits_only_the_unpairable_part() {
	let root = scratch("statusomit");
	let home = build_home(&root);
	let away = build_away(&root, "in-progress");
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));

	let (code, stdout, stderr) =
		run(&home, &["status", "--source", &away_plan, "--metrics", "docs/metrics/workflow.jsonl"]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("plan: 1 steps (1 in progress); 0 open-questions items"),
		"the plan half is unaffected; stdout:\n{stdout}"
	);
	assert!(stdout.contains("metrics: unavailable, the round log"), "stdout:\n{stdout}");
	assert!(!stdout.contains("3 records"), "stdout:\n{stdout}");

	// An explicit `--ledger-fragment` naming a ledger outside the plan's root: a note
	// naming the rejected path, and NO line of the block.
	let (code, stdout, stderr) = run(
		&home,
		&[
			"status",
			"--resume",
			"--source",
			&away_plan,
			"--ledger-fragment",
			"docs/plans/p.ledger.md",
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("the ledger docs/plans/p.ledger.md is not under the plan's project root"),
		"stdout:\n{stdout}"
	);
	assert!(!stdout.contains("HOME resume state."), "no line of the block; stdout:\n{stdout}");
	assert!(!stdout.contains("## RESUME STATE"), "stdout:\n{stdout}");

	// THE PRECEDENCE RULE on this surface, which the `next` equivalent already pins: a
	// fragment both outside the root AND missing reports the unsafe cause. Reporting the
	// absent one instead would tell a user there is no ledger for a ledger that exists in
	// another project.
	let (code, stdout, stderr) = run(
		&home,
		&[
			"status",
			"--resume",
			"--source",
			&away_plan,
			"--ledger-fragment",
			"docs/plans/nope.ledger.md",
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout
			.contains("the ledger docs/plans/nope.ledger.md is not under the plan's project root"),
		"stdout:\n{stdout}"
	);
	assert!(!stdout.contains("no ledger at"), "unsafe is not absent; stdout:\n{stdout}");

	let _ = fs::remove_dir_all(&root);
}

/// The log's LEAF is a symlink out of the plan's root, which is the clause that makes
/// `resolve_for_containment` resolve THE PATH ITSELF rather than only its directory prefix.
/// No other test in the suite reaches that clause: resolving only the prefix leaves every
/// other test green while this layout goes back to a false pass, because the log then sits
/// at the project's own conventional path by every test but the one that follows the link.
#[test]
fn a_symlinked_log_leaf_outside_the_root_is_refused_and_omitted() {
	let root = scratch("symlinkleaf");
	let home = build_home(&root);
	let away = build_away(&root, "complete");
	// `away` has no evidence of its own; the only converged `borrowed-step` round anywhere
	// is `home`'s, reached through the link at `away`'s own conventional log path.
	symlink(
		&home.join("docs").join("metrics").join("workflow.jsonl"),
		&away.join("docs").join("metrics").join("workflow.jsonl"),
	);
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));

	// The LOUD manifestation, on the DEFAULT resolution: no explicit `--metrics` is needed.
	let (code, stdout, stderr) = run(&home, &["validate", "--source", &away_plan, "--workflow"]);
	assert_eq!(code, Some(1), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(!stdout.contains("workflow invariants hold"), "stdout:\n{stdout}");
	assert!(stderr.contains("is not under the plan's project root"), "stderr:\n{stderr}");

	// The QUIET one, on both projections.
	for command in ["status", "next"] {
		let (code, stdout, stderr) = run(&home, &[command, "--source", &away_plan]);
		assert_eq!(code, Some(0), "{command}: stderr:\n{stderr}");
		assert!(stdout.contains("metrics: unavailable,"), "{command}: stdout:\n{stdout}");
		assert!(!stdout.contains("3 records"), "{command}: stdout:\n{stdout}");
	}

	let _ = fs::remove_dir_all(&root);
}

/// `status` and `next` READ NO PLAN with a Markdown-primary `--source` and no `--plan`,
/// which is the configuration `Q-55-resumepairing` decided for `status --resume`. The rule
/// SUPPLIES them a root from the anchors there, exactly as it supplies one to
/// `status --resume`, so the two LEDGER readers (`next` and `status --resume`) agree with
/// each other and the two LOG readers (`next` and `status`) agree with each other on
/// identical anchors. No single run elicits a comparable answer from all three: `status`
/// without `--resume` has no ledger field at all (`Projection`, `src/main.rs`), so it is
/// never asked the ledger question.
///
/// RED against the round 1 tip: `checked_plan_root` returns `None` here, both containment
/// filters go vacuous, and `next` echoes `home`'s `## RESUME STATE` block verbatim at exit
/// 0 with `"resume_state_absent_reason": null` on `--json`, while `status --resume` refuses
/// the same ledger on the same anchors.
#[test]
fn a_surface_that_reads_no_plan_is_supplied_a_root() {
	let root = scratch("noplanread");
	let home = build_home(&root);

	// A MARKDOWN-primary source with no `--plan`: nothing is read as a plan, so there is no
	// checked plan to root on. `alpha` and `home` are top-level siblings, so no containment
	// relationship of any kind holds between them.
	let alpha = root.join("alpha");
	write(&alpha.join("docs").join("plans").join("p.plan.toml"), &plan_toml_markdown_primary());
	let alpha_source = arg(&alpha.join("docs").join("plans").join("p.plan.toml"));
	let fragment = "docs/plans/p.ledger.md";

	// What `status --resume` answers on these anchors, which is the answer the other two
	// surfaces must agree with rather than contradict.
	let (code, stdout, stderr) = run(
		&home,
		&["status", "--resume", "--source", &alpha_source, "--ledger-fragment", fragment],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("the ledger docs/plans/p.ledger.md is not under the plan's project root"),
		"stdout:\n{stdout}"
	);

	let (code, stdout, stderr) =
		run(&home, &["next", "--source", &alpha_source, "--ledger-fragment", fragment]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(!stdout.contains("HOME resume state."), "no line of the block; stdout:\n{stdout}");
	assert!(!stdout.contains("RESUME STATE (verbatim from the ledger)"), "stdout:\n{stdout}");
	assert!(
		stdout.contains("the ledger docs/plans/p.ledger.md is not under the plan's project root"),
		"the same note `status --resume` prints; stdout:\n{stdout}"
	);

	let (code, stdout, stderr) =
		run(&home, &["next", "--json", "--source", &alpha_source, "--ledger-fragment", fragment]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("\"resume_state\": null"), "stdout:\n{stdout}");
	assert!(
		stdout.contains("\"resume_state_absent_reason\": \"ledger-not-this-project\""),
		"a `null` reason here would positively assert the block is this plan's; stdout:\n{stdout}"
	);

	// The LOG half of the same configuration, on both commands.
	for command in ["next", "status"] {
		let (code, stdout, stderr) = run(
			&home,
			&[
				command,
				"--json",
				"--source",
				&alpha_source,
				"--metrics",
				"docs/metrics/workflow.jsonl",
			],
		);
		assert_eq!(code, Some(0), "{command}: stderr:\n{stderr}");
		assert!(
			stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
			"{command}: stdout:\n{stdout}"
		);
		assert!(!stdout.contains("\"records\": 3"), "{command}: stdout:\n{stdout}");
	}

	let _ = fs::remove_dir_all(&root);
}

/// AN ANCHOR THAT DOES NOT EXIST STILL SUPPLIES A ROOT, which is the clause that separates
/// "no plan was read" from "no root could be derived". The test above only ever writes the
/// anchor it then passes, so nothing in the suite saw this: with a `--source` that is not on
/// disk the anchor was DROPPED, the root vector went empty, every containment quantifier
/// over it went vacuous, and `status`, `next` and `status --resume` alike read an explicit
/// foreign `--metrics` and echoed another project's `## RESUME STATE` block verbatim at exit
/// 0 with both `--json` reason fields `null`.
///
/// RED against the round 2 tip on the ATTACK block below: one character of the `--source`
/// (`p.plan.toml` -> `q.plan.toml`, everything else identical) is the whole difference
/// between the refusal the CONTROL gets and the leak, so the two runs are kept side by side
/// here rather than in separate tests.
///
/// The OWN-ARTIFACT block is the shape check on the remedy (`Q-55-emptyroot`): the root is
/// derived from the anchor's path itself, resolved as far as the filesystem allows, rather
/// than withheld, so an anchor whose file has not been written yet still reads ITS OWN
/// project's log and ledger. A remedy that treated an underivable root as unpairable would
/// omit those too, and this block would go red.
///
/// EVERY RUN HERE SUPPLIES ONE ANCHOR AND THAT ANCHOR IS THE MISSING ONE, which is the
/// configuration the guessed root is scoped to (`resume_roots`). The narrowing that keeps a
/// missing anchor from overruling one on disk is pinned separately, in
/// `a_missing_anchor_does_not_overrule_an_anchor_that_exists`.
#[test]
fn an_anchor_that_does_not_exist_still_supplies_a_root() {
	let root = scratch("missinganchor");
	let home = build_home(&root);

	// `alpha` is a top-level sibling of `home` with a real `docs/plans` directory, its own
	// TWO-record log and its own ledger, but NO `q.plan.toml`. No containment relationship
	// of any kind holds between `alpha` and `home` in either direction.
	let alpha = root.join("alpha");
	write(
		&alpha.join("docs").join("metrics").join("workflow.jsonl"),
		&log(&["alpha-one", "alpha-two"]),
	);
	write(&alpha.join("docs").join("plans").join("a.ledger.md"), &resume_block("ALPHA"));
	let missing = arg(&alpha.join("docs").join("plans").join("q.plan.toml"));
	let fragment = "docs/plans/p.ledger.md";

	// THE ATTACK, on both ledger readers and both log readers, human surface first.
	let (code, stdout, stderr) = run(
		&home,
		&[
			"next",
			"--source",
			&missing,
			"--metrics",
			"docs/metrics/workflow.jsonl",
			"--ledger-fragment",
			fragment,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("metrics: unavailable,"), "stdout:\n{stdout}");
	assert!(!stdout.contains("3 records"), "home's log is not this anchor's; stdout:\n{stdout}");
	assert!(!stdout.contains("HOME resume state."), "no line of the block; stdout:\n{stdout}");
	assert!(
		stdout.contains("the ledger docs/plans/p.ledger.md is not under the plan's project root"),
		"stdout:\n{stdout}"
	);
	// Fail loudly, the condition the remedy was accepted with: the operator is TOLD the
	// anchor is a typo, on stderr so `--json` on stdout stays parseable.
	assert!(
		stderr.contains(&format!("note: --source {missing} does not exist")),
		"the typo is reported; stderr:\n{stderr}"
	);

	let (code, stdout, stderr) = run(
		&home,
		&[
			"next",
			"--json",
			"--source",
			&missing,
			"--metrics",
			"docs/metrics/workflow.jsonl",
			"--ledger-fragment",
			fragment,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"stdout:\n{stdout}"
	);
	assert!(
		stdout.contains("\"resume_state_absent_reason\": \"ledger-not-this-project\""),
		"a `null` reason here would positively assert the block is this plan's; stdout:\n{stdout}"
	);
	assert!(stdout.contains("\"resume_state\": null"), "stdout:\n{stdout}");
	assert!(!stdout.contains("\"records\": 3"), "stdout:\n{stdout}");

	// `status --resume`, whose hole is independent: it calls `resume_roots` directly rather
	// than through `containment_roots`, so a fix that closed only the other two would leave
	// this one leaking.
	let (code, stdout, stderr) =
		run(&home, &["status", "--resume", "--source", &missing, "--ledger-fragment", fragment]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("the ledger docs/plans/p.ledger.md is not under the plan's project root"),
		"stdout:\n{stdout}"
	);
	assert!(!stdout.contains("HOME resume state."), "stdout:\n{stdout}");
	// The note is `Q-55-emptyroot`'s Fail-loudly half and it is owed on EVERY surface that
	// roots on a name with nothing behind it, not only on `next`. Deleting the
	// `note_missing_anchors` call in `run_status` left the whole suite green, so the two
	// `status` slices are pinned here exactly as the `next` run above is.
	assert!(
		stderr.contains(&format!("note: --source {missing} does not exist")),
		"`status --resume` owes the typo too; stderr:\n{stderr}"
	);

	let (code, stdout, stderr) = run(
		&home,
		&["status", "--json", "--source", &missing, "--metrics", "docs/metrics/workflow.jsonl"],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"stdout:\n{stdout}"
	);
	assert!(!stdout.contains("\"records\": 3"), "stdout:\n{stdout}");
	assert!(
		stderr.contains(&format!("note: --source {missing} does not exist")),
		"and on stderr, not stdout, so `--json` stays parseable; stderr:\n{stderr}"
	);

	// THE ONE-CHARACTER CONTROL: the same anchor spelled correctly, which already refused
	// before this fix. Both spellings must now give the SAME verdict, and only the missing
	// one carries the note.
	write(&alpha.join("docs").join("plans").join("p.plan.toml"), &plan_toml_markdown_primary());
	let present = arg(&alpha.join("docs").join("plans").join("p.plan.toml"));
	let (code, stdout, stderr) = run(
		&home,
		&[
			"next",
			"--source",
			&present,
			"--metrics",
			"docs/metrics/workflow.jsonl",
			"--ledger-fragment",
			fragment,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("metrics: unavailable,"), "stdout:\n{stdout}");
	assert!(!stdout.contains("HOME resume state."), "stdout:\n{stdout}");
	assert!(
		!stderr.contains("does not exist"),
		"no note for an anchor that is there; stderr:\n{stderr}"
	);

	// THE ANCHOR'S OWN ARTIFACTS, named explicitly, are still READ under the derived root.
	let own_log = arg(&alpha.join("docs").join("metrics").join("workflow.jsonl"));
	let own_ledger = arg(&alpha.join("docs").join("plans").join("a.ledger.md"));
	let (code, stdout, stderr) = run(
		&home,
		&["next", "--source", &missing, "--metrics", &own_log, "--ledger-fragment", &own_ledger],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("metrics: 2 records"), "the anchor's OWN log; stdout:\n{stdout}");
	assert!(stdout.contains("ALPHA resume state."), "the anchor's OWN ledger; stdout:\n{stdout}");

	// THE NEITHER-ANCHOR CASE IS UNTOUCHED (`README.md`, the anchoring paragraph): with no
	// `--source` and no `--plan` there is nothing to pair against, so no root is derived, no
	// containment check fires, and the current-directory-relative defaults stand.
	for command in ["status", "next"] {
		let (code, stdout, stderr) = run(&home, &[command]);
		assert_eq!(code, Some(0), "{command}: stderr:\n{stderr}");
		assert!(
			stdout.contains("metrics: 3 records"),
			"{command}: the current directory's own log; stdout:\n{stdout}"
		);
		assert!(stderr.is_empty(), "{command}: no anchor, so no note; stderr:\n{stderr}");
	}

	let _ = fs::remove_dir_all(&root);
}

/// A MISSING ANCHOR DOES NOT OVERRULE AN ANCHOR THAT EXISTS (`Q-55-anchorveto`). The root a
/// missing anchor supplies is a guess about a name with nothing behind it, and the test above
/// scopes that guess to the case that motivated it: nothing else to go on. Where a supplied
/// anchor IS on disk, only the anchors on disk decide.
///
/// THREE CONTROLS ON ONE FIXTURE, varying ONLY the `--plan`, because the defect is invisible
/// against the wrong one. C0 supplies no `--plan` at all, which is the baseline an operator
/// who then types a stale path actually had; C1 supplies a `--plan` that does not exist; C2
/// supplies the SAME PATH written. C1 must answer like C0, not like C2. An enumeration that
/// always passes a `--plan` and varies only whether the file is there contains C1 and C2 and
/// cannot contain C0, so it reports C1 as correct.
///
/// RED against the round 3 tip on C1: the missing `--plan` contributed a second root in
/// `beta`, containment requires the artifact under EVERY root, and `alpha`'s OWN default log
/// and OWN default ledger were withheld at exit 0 with `log-not-this-project` and
/// `ledger-not-this-project` asserted about the project the `--source` names.
///
/// C2 IS PINNED AS UNCHANGED, so the narrowing cannot be mistaken for a general loosening:
/// two anchors that both exist in different projects still reject each other's artifacts,
/// which is the divergent pairing `Q-55-resumepairing` decided and accepted cost (iv)
/// records.
#[test]
fn a_missing_anchor_does_not_overrule_an_anchor_that_exists() {
	let root = scratch("anchorveto");
	let home = build_home(&root);

	// `alpha` is a top-level sibling of `home`, Markdown-primary so no plan is ever read from
	// it, with its own TWO-record log and its own `<task>.ledger.md` at their DEFAULT paths.
	// Both artifacts are therefore derived from the `--source` itself, not named on the
	// command line: there is nothing here for the operator to have pointed anywhere.
	let alpha = root.join("alpha");
	write(&alpha.join("docs").join("plans").join("m.plan.toml"), &plan_toml_markdown_primary());
	write(&alpha.join("docs").join("plans").join("m.ledger.md"), &resume_block("ALPHA"));
	write(
		&alpha.join("docs").join("metrics").join("workflow.jsonl"),
		&log(&["alpha-one", "alpha-two"]),
	);
	let source = arg(&alpha.join("docs").join("plans").join("m.plan.toml"));
	let beta_plan = arg(&root.join("beta").join("docs").join("plans").join("s.md"));

	// C0 and C1 must agree on all four of these, and each is measured on both the human and
	// the machine surface plus the independent `status --resume` slice.
	for (label, extra) in
		[("C0 no --plan", Vec::new()), ("C1 --plan missing", vec!["--plan", &beta_plan])]
	{
		let mut argv = vec!["next", "--source", &source];
		argv.extend_from_slice(&extra);
		let (code, stdout, stderr) = run(&home, &argv);
		assert_eq!(code, Some(0), "{label}: stderr:\n{stderr}");
		assert!(
			stdout.contains("metrics: 2 records"),
			"{label}: alpha's OWN log; stdout:\n{stdout}"
		);
		assert!(
			stdout.contains("ALPHA resume state."),
			"{label}: alpha's OWN ledger; stdout:\n{stdout}"
		);

		let mut argv = vec!["next", "--json", "--source", &source];
		argv.extend_from_slice(&extra);
		let (code, stdout, stderr) = run(&home, &argv);
		assert_eq!(code, Some(0), "{label}: stderr:\n{stderr}");
		assert!(
			stdout.contains("\"metrics_absent_reason\": null"),
			"{label}: a project's own log is this project's; stdout:\n{stdout}"
		);
		assert!(
			stdout.contains("\"resume_state_absent_reason\": null"),
			"{label}: and so is its own ledger; stdout:\n{stdout}"
		);
		assert!(stdout.contains("ALPHA resume state."), "{label}: stdout:\n{stdout}");

		let mut argv = vec!["status", "--resume", "--source", &source];
		argv.extend_from_slice(&extra);
		let (code, stdout, stderr) = run(&home, &argv);
		assert_eq!(code, Some(0), "{label}: stderr:\n{stderr}");
		assert!(
			stdout.contains("ALPHA resume state."),
			"{label}: the slice that calls `resume_roots` directly; stdout:\n{stdout}"
		);
		assert!(
			!stdout.contains("nothing to resume"),
			"{label}: no refusal of alpha's own ledger; stdout:\n{stdout}"
		);
	}

	// Reading the artifacts does NOT cost the typo its note: the anchor is still not there and
	// C1 still says so, which is the half of `Q-55-emptyroot` that Fail loudly bought.
	let (_, _, stderr) = run(&home, &["next", "--source", &source, "--plan", &beta_plan, "--json"]);
	assert!(
		stderr.contains(&format!("note: --plan {beta_plan} does not exist")),
		"the stale path is still reported; stderr:\n{stderr}"
	);

	// C2: the SAME `--plan` path, now written. The divergent pairing is untouched.
	write(&root.join("beta").join("docs").join("plans").join("s.md"), &plan_markdown("complete"));
	let (code, stdout, stderr) = run(&home, &["next", "--source", &source, "--plan", &beta_plan]);
	assert_eq!(code, Some(0), "C2: stderr:\n{stderr}");
	assert!(stdout.contains("metrics: unavailable,"), "C2: stdout:\n{stdout}");
	assert!(!stdout.contains("ALPHA resume state."), "C2: stdout:\n{stdout}");
	let (code, stdout, stderr) =
		run(&home, &["status", "--resume", "--source", &source, "--plan", &beta_plan]);
	assert_eq!(code, Some(0), "C2: stderr:\n{stderr}");
	assert!(stdout.contains("nothing to resume"), "C2: stdout:\n{stdout}");

	let _ = fs::remove_dir_all(&root);
}

/// AN ANCHOR THE TOOL CANNOT ASK ABOUT IS NOT AN ANCHOR THAT IS MISSING. `Path::exists`
/// answers `false` both for a path that is not there and for one whose metadata cannot be
/// read, so a plan sitting on disk inside a directory the caller cannot traverse was reported
/// as not existing. The note is the whole Fail-loudly half of `Q-55-emptyroot`'s remedy, and
/// a loud line that states a falsehood about the filesystem is worse than a quiet one: it
/// sends the operator to fix a path that is already correct. `try_exists` splits the two and
/// each gets its own line.
///
/// RED against the round 3 tip: the run below printed `does not exist` for a file the final
/// assertion then finds on disk.
#[cfg(unix)]
#[test]
fn an_anchor_that_cannot_be_checked_is_not_reported_as_missing() {
	use std::os::unix::fs::PermissionsExt;

	let root = scratch("uncheckable");
	let home = build_home(&root);
	let plans = root.join("proj").join("docs").join("plans");
	write(&plans.join("p.plan.toml"), &plan_toml_markdown_primary());
	let source = arg(&plans.join("p.plan.toml"));

	fs::set_permissions(&plans, fs::Permissions::from_mode(0o000)).unwrap();
	// Whether the mode actually hides the file, measured rather than assumed: as root it does
	// not, and then the anchor is simply THERE, which is a third case with its own (correct)
	// answer and nothing for this test to say.
	let opaque = fs::metadata(plans.join("p.plan.toml")).is_err();
	let (code, _, stderr) = run(&home, &["status", "--source", &source]);
	fs::set_permissions(&plans, fs::Permissions::from_mode(0o755)).unwrap();

	assert_eq!(code, Some(0), "a note is not an error; stderr:\n{stderr}");
	if opaque {
		assert!(
			!stderr.contains("does not exist"),
			"the anchor is on disk, so this sentence is false; stderr:\n{stderr}"
		);
		assert!(
			stderr.contains(&format!("note: --source {source} could not be checked")),
			"and the operator is still told the answer is unknown; stderr:\n{stderr}"
		);
	}
	assert!(plans.join("p.plan.toml").exists(), "the anchor was there the whole time");

	let _ = fs::remove_dir_all(&root);
}

/// The fixture the two `Err`-anchor tests below share: THREE TOP-LEVEL SIBLINGS under
/// `root`, with no containment relation between any two of them. `home` is the directory the
/// runs are made from and carries its own THREE-record log; `alpha` is the project the
/// MISSING anchor names, with a real `docs/plans`, a TWO-record log and its own block;
/// `beta` is the project the UNCHECKABLE anchor names, with a FOUR-record log, its own
/// block, and the two files a trailing slash is put on. Beta's log and ledger are the
/// artifacts named explicitly on every command line below, so the record count and the block
/// name in the output say which file was read rather than being inferred from the path.
#[cfg(unix)]
fn build_err_anchor_fixture(root: &Path) -> (PathBuf, PathBuf, PathBuf) {
	let home = build_home(root);

	let alpha = root.join("alpha");
	write(
		&alpha.join("docs").join("metrics").join("workflow.jsonl"),
		&log(&["alpha-one", "alpha-two"]),
	);
	write(&alpha.join("docs").join("plans").join("a.ledger.md"), &resume_block("ALPHA"));

	let beta = root.join("beta");
	write(
		&beta.join("docs").join("metrics").join("workflow.jsonl"),
		&log(&["beta-one", "beta-two", "beta-three", "beta-four"]),
	);
	write(&beta.join("docs").join("plans").join("b.ledger.md"), &resume_block("BETA"));
	write(&beta.join("docs").join("plans").join("b.md"), &plan_markdown("complete"));
	write(&beta.join("docs").join("plans").join("b.plan.toml"), &plan_toml_markdown_primary());

	(home, alpha, beta)
}

/// AN ANCHOR WHOSE EXISTENCE CANNOT BE DETERMINED DOES NOT REMOVE THE OTHER ANCHOR'S ROOT,
/// with the uncheckable anchor in the `--plan` slot. `try_exists` errs for every `stat`
/// failure that is not an absence, and counting such an anchor as ON DISK made `on_disk`
/// non-empty; a non-empty `on_disk` suppresses the fallback to the supplied anchors, so the
/// MISSING `--source`'s root was dropped and the uncheckable `--plan`'s project decided
/// alone, although no supplied anchor was on disk at all.
///
/// RED against the round 4 tip: `beta`'s four-record log was counted and its private
/// `## RESUME STATE` block echoed verbatim by `next`, `next --json`, `status --json` and
/// `status --resume` alike, at exit 0 with both machine reason fields `null`. That is the
/// same output shape `an_anchor_that_does_not_exist_still_supplies_a_root` closed, reached
/// through the other branch of the same filter, which is why the membership rule is pinned
/// here and not only the deciding rule.
///
/// THE TRIGGER IS A TRAILING SLASH ON A FILE THAT EXISTS (`ENOTDIR`), chosen because it
/// needs no permission manipulation and behaves the same for any uid. The `Err`
/// CLASSIFICATION rather than that spelling is what this pins: `ELOOP`, `ENAMETOOLONG` and a
/// directory the process cannot traverse all reach the same branch, and the `could not be
/// checked` assertion below is what keeps the fixture from passing vacuously if the anchor
/// ever stopped landing in that class.
///
/// THE ONE-CHARACTER CONTROL is the same `--plan` WITHOUT the slash, run on `status
/// --resume` so that both runs reach `resume_roots` directly rather than one of them being
/// answered by `checked_plan_root`. That anchor IS on disk, so it still decides alone and
/// beta's own block still prints; the refusal above therefore follows from the anchor's stat
/// class and not from the layout.
#[cfg(unix)]
#[test]
fn an_uncheckable_plan_anchor_does_not_remove_the_other_anchors_root() {
	let root = scratch("errplan");
	let (home, alpha, beta) = build_err_anchor_fixture(&root);

	// The MISSING anchor: a plan in `alpha` that has not been written (`Ok(false)`).
	let missing_source = arg(&alpha.join("docs").join("plans").join("ghost.plan.toml"));
	// The UNCHECKABLE anchor and, one character shorter, its control.
	let on_disk_plan = arg(&beta.join("docs").join("plans").join("b.md"));
	let uncheckable_plan = format!("{on_disk_plan}/");
	let beta_log = arg(&beta.join("docs").join("metrics").join("workflow.jsonl"));
	let beta_ledger = arg(&beta.join("docs").join("plans").join("b.ledger.md"));

	let (code, stdout, stderr) = run(
		&home,
		&[
			"next",
			"--source",
			&missing_source,
			"--plan",
			&uncheckable_plan,
			"--metrics",
			&beta_log,
			"--ledger-fragment",
			&beta_ledger,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stderr.contains(&format!("note: --plan {uncheckable_plan} could not be checked")),
		"the anchor must land in the `Err` class or this test measures nothing; stderr:\n{stderr}"
	);
	assert!(
		stderr.contains(&format!("note: --source {missing_source} does not exist")),
		"and the anchor beside it is the missing one; stderr:\n{stderr}"
	);
	assert!(stdout.contains("metrics: unavailable,"), "stdout:\n{stdout}");
	assert!(!stdout.contains("4 records"), "beta's log is not this pairing's; stdout:\n{stdout}");
	assert!(!stdout.contains("BETA resume state."), "no line of the block; stdout:\n{stdout}");
	assert!(
		stdout.contains(&format!("the ledger {beta_ledger} is not under the plan's project root")),
		"stdout:\n{stdout}"
	);

	let (code, stdout, stderr) = run(
		&home,
		&[
			"next",
			"--json",
			"--source",
			&missing_source,
			"--plan",
			&uncheckable_plan,
			"--metrics",
			&beta_log,
			"--ledger-fragment",
			&beta_ledger,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"stdout:\n{stdout}"
	);
	assert!(
		stdout.contains("\"resume_state_absent_reason\": \"ledger-not-this-project\""),
		"a `null` reason here would positively assert beta's block is this plan's; stdout:\n{stdout}"
	);
	assert!(stdout.contains("\"resume_state\": null"), "stdout:\n{stdout}");
	assert!(!stdout.contains("\"records\": 4"), "stdout:\n{stdout}");

	let (code, stdout, stderr) = run(
		&home,
		&[
			"status",
			"--json",
			"--source",
			&missing_source,
			"--plan",
			&uncheckable_plan,
			"--metrics",
			&beta_log,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"stdout:\n{stdout}"
	);
	assert!(!stdout.contains("\"records\": 4"), "stdout:\n{stdout}");

	let (code, stdout, stderr) = run(
		&home,
		&[
			"status",
			"--resume",
			"--source",
			&missing_source,
			"--plan",
			&uncheckable_plan,
			"--ledger-fragment",
			&beta_ledger,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		!stdout.contains("BETA resume state."),
		"the slice that calls `resume_roots` directly; stdout:\n{stdout}"
	);
	assert!(stdout.contains("nothing to resume"), "stdout:\n{stdout}");

	// THE ONE-CHARACTER CONTROL, on that same slice.
	let (code, stdout, stderr) = run(
		&home,
		&[
			"status",
			"--resume",
			"--source",
			&missing_source,
			"--plan",
			&on_disk_plan,
			"--ledger-fragment",
			&beta_ledger,
		],
	);
	assert_eq!(code, Some(0), "control: stderr:\n{stderr}");
	assert!(
		stdout.contains("BETA resume state."),
		"control: an anchor that IS on disk still decides alone; stdout:\n{stdout}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// THE SAME RULE WITH THE SLOTS SWAPPED: the uncheckable anchor is the `--source` and the
/// missing one the `--plan`. The orientation is not a formality and this test is not a
/// duplicate of the one above.
///
/// With the uncheckable anchor in the `--source` slot the TASK NAME is derived from that
/// anchor, so beta's log and beta's block read as beta's own and the leak looks like a
/// correct read. Asking whether the admitted artifact belongs to the surviving anchor's own
/// project answers "yes" here and "no" in the other orientation, on identical behaviour; the
/// question that separates them is WHAT THE REMOVED ROOT PREVIOUSLY REFUSED. It refused this
/// pairing: two anchors naming two different projects must reject each other's artifacts
/// (`Q-55-resumepairing`, and accepted cost (iv)), which is what
/// `resume_omits_the_default_ledger_under_a_divergent_pairing` pins for two anchors that
/// both exist. Dropping one anchor because the other could not be `stat`ed silently exempted
/// the pairing from that rule.
///
/// RED against the round 4 tip on every assertion below, exactly as in the `--plan`
/// orientation, and the only visible difference between the two leaks is the task name.
#[cfg(unix)]
#[test]
fn an_uncheckable_source_anchor_does_not_remove_the_other_anchors_root() {
	let root = scratch("errsource");
	let (home, alpha, beta) = build_err_anchor_fixture(&root);

	// The MISSING anchor: a Markdown plan in `alpha` that has not been written (`Ok(false)`).
	let missing_plan = arg(&alpha.join("docs").join("plans").join("ghost.md"));
	// The UNCHECKABLE anchor and, one character shorter, its control. This one is
	// Markdown-primary, so no plan is read from it in either spelling and the two runs differ
	// only in the stat class of the anchor.
	let on_disk_source = arg(&beta.join("docs").join("plans").join("b.plan.toml"));
	let uncheckable_source = format!("{on_disk_source}/");
	let beta_log = arg(&beta.join("docs").join("metrics").join("workflow.jsonl"));
	let beta_ledger = arg(&beta.join("docs").join("plans").join("b.ledger.md"));

	let (code, stdout, stderr) = run(
		&home,
		&[
			"next",
			"--source",
			&uncheckable_source,
			"--plan",
			&missing_plan,
			"--metrics",
			&beta_log,
			"--ledger-fragment",
			&beta_ledger,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stderr.contains(&format!("note: --source {uncheckable_source} could not be checked")),
		"the anchor must land in the `Err` class or this test measures nothing; stderr:\n{stderr}"
	);
	assert!(
		stderr.contains(&format!("note: --plan {missing_plan} does not exist")),
		"and the anchor beside it is the missing one; stderr:\n{stderr}"
	);
	assert!(stdout.contains("metrics: unavailable,"), "stdout:\n{stdout}");
	assert!(!stdout.contains("4 records"), "stdout:\n{stdout}");
	assert!(
		!stdout.contains("BETA resume state."),
		"the derived task is beta's, which does not make the block this pairing's; stdout:\n{stdout}"
	);
	assert!(
		stdout.contains(&format!("the ledger {beta_ledger} is not under the plan's project root")),
		"stdout:\n{stdout}"
	);

	let (code, stdout, stderr) = run(
		&home,
		&[
			"next",
			"--json",
			"--source",
			&uncheckable_source,
			"--plan",
			&missing_plan,
			"--metrics",
			&beta_log,
			"--ledger-fragment",
			&beta_ledger,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"stdout:\n{stdout}"
	);
	assert!(
		stdout.contains("\"resume_state_absent_reason\": \"ledger-not-this-project\""),
		"stdout:\n{stdout}"
	);
	assert!(stdout.contains("\"resume_state\": null"), "stdout:\n{stdout}");
	assert!(!stdout.contains("\"records\": 4"), "stdout:\n{stdout}");

	let (code, stdout, stderr) = run(
		&home,
		&[
			"status",
			"--json",
			"--source",
			&uncheckable_source,
			"--plan",
			&missing_plan,
			"--metrics",
			&beta_log,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"stdout:\n{stdout}"
	);
	assert!(!stdout.contains("\"records\": 4"), "stdout:\n{stdout}");

	let (code, stdout, stderr) = run(
		&home,
		&[
			"status",
			"--resume",
			"--source",
			&uncheckable_source,
			"--plan",
			&missing_plan,
			"--ledger-fragment",
			&beta_ledger,
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		!stdout.contains("BETA resume state."),
		"the slice that calls `resume_roots` directly; stdout:\n{stdout}"
	);
	assert!(stdout.contains("nothing to resume"), "stdout:\n{stdout}");

	// THE ONE-CHARACTER CONTROL, on that same slice.
	let (code, stdout, stderr) = run(
		&home,
		&[
			"status",
			"--resume",
			"--source",
			&on_disk_source,
			"--plan",
			&missing_plan,
			"--ledger-fragment",
			&beta_ledger,
		],
	);
	assert_eq!(code, Some(0), "control: stderr:\n{stderr}");
	assert!(
		stdout.contains("BETA resume state."),
		"control: an anchor that IS on disk still decides alone; stdout:\n{stdout}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 14c's third run (`Q-55-resumepairing`): `status --resume` on the
/// DEFAULT ledger under a divergent pairing, with no `--ledger-fragment` at all. This
/// surface reads no plan, so the rule SUPPLIES it a root: two anchors that both exist must
/// resolve to the same one.
///
/// RED against the parent commit: `alpha`'s block is printed verbatim at exit 0, and an
/// inc2 that left this surface anchor-rooted would keep it.
#[test]
fn resume_omits_the_default_ledger_under_a_divergent_pairing() {
	let root = scratch("resumepairing");
	let home = build_home(&root);

	let alpha = root.join("alpha");
	write(&alpha.join("docs").join("plans").join("p.plan.toml"), &plan_toml_markdown_primary());
	write(&alpha.join("docs").join("plans").join("p.ledger.md"), &resume_block("ALPHA"));
	let alpha_source = arg(&alpha.join("docs").join("plans").join("p.plan.toml"));

	let beta = root.join("beta");
	write(&beta.join("docs").join("plans").join("p.md"), &plan_markdown("complete"));
	let beta_plan = arg(&beta.join("docs").join("plans").join("p.md"));

	let (code, stdout, stderr) =
		run(&home, &["status", "--resume", "--source", &alpha_source, "--plan", &beta_plan]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(!stdout.contains("ALPHA resume state."), "no line of the block; stdout:\n{stdout}");
	assert!(
		stdout.contains("is not under the plan's project root"),
		"the note says why; stdout:\n{stdout}"
	);

	// One anchor alone is the root, exactly as before: `alpha`'s own resume still prints.
	let (code, stdout, stderr) = run(&home, &["status", "--resume", "--source", &alpha_source]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("ALPHA resume state."), "stdout:\n{stdout}");

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance checks 14e and 14f, the machine surface and the fourth owed red-then-green
/// case, pinned on BOTH commands because `status --json` has no golden and no test on its
/// serialisation at all.
///
/// RED against the parent commit: none of the three fields exists. `no_active_loop_reason`
/// is `#[serde(skip)]`, and `status`'s `Projection` has no reason field, so an omitted
/// part serialises as a bare `null` that reads the same for every cause.
///
/// The three inputs must give THREE distinct answers; if two agreed, the vocabulary would
/// be under-specified and the defect would have moved rather than closed.
#[test]
fn the_machine_surface_separates_the_causes_on_both_commands() {
	let root = scratch("json");
	let home = build_home(&root);
	let away = build_away(&root, "in-progress");
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));

	// (b) The unsafe pairing, on `next` and on `status`.
	let (code, stdout, stderr) = run(
		&home,
		&["next", "--json", "--source", &away_plan, "--metrics", "docs/metrics/workflow.jsonl"],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("\"metrics\": null"), "stdout:\n{stdout}");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"stdout:\n{stdout}"
	);
	assert!(stdout.contains("\"active_loop\": null"), "stdout:\n{stdout}");
	assert!(
		stdout.contains("\"no_active_loop_reason\": \"metrics-not-this-project\""),
		"stdout:\n{stdout}"
	);

	let (code, stdout, stderr) = run(
		&home,
		&["status", "--json", "--source", &away_plan, "--metrics", "docs/metrics/workflow.jsonl"],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("\"metrics\": null"), "stdout:\n{stdout}");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"the unguarded half is asserted here; stdout:\n{stdout}"
	);

	// (a) The plan's own log genuinely absent: a DERIVED loop, and a different reason.
	let (code, stdout, stderr) = run(&home, &["next", "--json", "--source", &away_plan]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("\"metrics_absent_reason\": \"log-absent\""), "stdout:\n{stdout}");
	assert!(stdout.contains("\"no_active_loop_reason\": null"), "stdout:\n{stdout}");
	assert!(
		!stdout.contains("\"active_loop\": null"),
		"an absent log still projects; stdout:\n{stdout}"
	);

	// (a) on `status --json` too. Its `log-absent` value is the half with no golden, so
	// nothing else separates check 14f's case (a) from case (b) on this command.
	let (code, stdout, stderr) = run(&home, &["status", "--json", "--source", &away_plan]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("\"metrics_absent_reason\": \"log-absent\""), "stdout:\n{stdout}");

	// (c) No plan source at all.
	let (code, stdout, stderr) = run(&home, &["next", "--json"]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("\"no_active_loop_reason\": \"no-plan-steps\""), "stdout:\n{stdout}");

	// The PRECEDENCE rule: an explicit `--metrics` outside the root naming a file that does
	// not exist reports the unsafe cause, not the absent one.
	let (code, stdout, stderr) = run(
		&home,
		&[
			"next",
			"--json",
			"--source",
			&away_plan,
			"--metrics",
			"docs/metrics/does-not-exist.jsonl",
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"unsafe wins over absent; stdout:\n{stdout}"
	);

	// The same precedence rule on `status --json`, the command whose serialisation has no
	// golden. Swapping the two tests there makes the two commands disagree on one input.
	let (code, stdout, stderr) = run(
		&home,
		&[
			"status",
			"--json",
			"--source",
			&away_plan,
			"--metrics",
			"docs/metrics/does-not-exist.jsonl",
		],
	);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"unsafe wins over absent; stdout:\n{stdout}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 14g: the resume reasons separate too, including the DEFAULT-ledger
/// half of `Q-55-endproperty`, which needs no `--ledger-fragment` at all.
#[test]
fn the_resume_reasons_separate_and_cover_the_default_ledger() {
	let root = scratch("resumereasons");
	let home = build_home(&root);
	let away = build_away(&root, "in-progress");
	let away_plan = arg(&away.join("docs").join("plans").join("p.plan.toml"));

	let reason = |stdout: &str| {
		stdout
			.lines()
			.find(|line| line.contains("resume_state_absent_reason"))
			.unwrap_or("<absent>")
			.trim()
			.to_string()
	};

	// Absent ledger.
	let (_, stdout, _) = run(&home, &["next", "--json", "--source", &away_plan]);
	assert_eq!(reason(&stdout), "\"resume_state_absent_reason\": \"ledger-absent\",");

	// Present ledger with no `## RESUME STATE` block.
	let bare = away.join("docs").join("plans").join("bare.ledger.md");
	write(&bare, "# not a resume ledger\n");
	let (_, stdout, _) =
		run(&home, &["next", "--json", "--source", &away_plan, "--ledger-fragment", &arg(&bare)]);
	assert_eq!(reason(&stdout), "\"resume_state_absent_reason\": \"no-resume-section\",");

	// An explicit fragment outside the plan's root.
	let (_, stdout, _) = run(
		&home,
		&["next", "--json", "--source", &away_plan, "--ledger-fragment", "docs/plans/p.ledger.md"],
	);
	assert_eq!(reason(&stdout), "\"resume_state_absent_reason\": \"ledger-not-this-project\",");
	assert!(!stdout.contains("HOME resume state."), "stdout:\n{stdout}");

	// The HUMAN surface on the same input. `Q-55-refusalscope` is an OMIT plus SAY WHY
	// decision, and the note is assembled by the CALLER, so the JSON above leaves the say-why
	// half of the agent-facing text unpinned on its own.
	let (_, stdout, _) = run(
		&home,
		&["next", "--source", &away_plan, "--ledger-fragment", "docs/plans/p.ledger.md"],
	);
	assert!(
		stdout.contains("the ledger docs/plans/p.ledger.md is not under the plan's project root"),
		"the note names the rejected ledger path in the block's place; stdout:\n{stdout}"
	);
	assert!(!stdout.contains("HOME resume state."), "stdout:\n{stdout}");

	// Outside the root AND missing: the unsafe cause wins over the absent one.
	let (_, stdout, _) = run(
		&home,
		&[
			"next",
			"--json",
			"--source",
			&away_plan,
			"--ledger-fragment",
			"docs/plans/nope.ledger.md",
		],
	);
	assert_eq!(reason(&stdout), "\"resume_state_absent_reason\": \"ledger-not-this-project\",");

	// The DEFAULT ledger under a divergent pairing: `next` projects `beta`'s steps while
	// the ledger default anchors on the `--source` in `alpha`. This run, not the explicit
	// fragment above, is what separates an anchor-rooted projection from a checked-plan
	// rooted one.
	let alpha = root.join("alpha");
	write(&alpha.join("docs").join("plans").join("p.plan.toml"), &plan_toml_markdown_primary());
	write(&alpha.join("docs").join("plans").join("p.ledger.md"), &resume_block("ALPHA"));
	write(
		&alpha.join("docs").join("metrics").join("workflow.jsonl"),
		&log(&["borrowed-step", "alpha-two"]),
	);
	let alpha_source = arg(&alpha.join("docs").join("plans").join("p.plan.toml"));
	let alpha_ledger = arg(&alpha.join("docs").join("plans").join("p.ledger.md"));
	let beta = root.join("beta");
	write(&beta.join("docs").join("plans").join("p.md"), &plan_markdown("in progress"));
	let beta_plan = arg(&beta.join("docs").join("plans").join("p.md"));

	let (code, stdout, stderr) =
		run(&home, &["next", "--source", &alpha_source, "--plan", &beta_plan]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(!stdout.contains("ALPHA resume state."), "no line of the block; stdout:\n{stdout}");
	// The note stands in the block's place here too. The LEDGER phrasing is asserted rather
	// than the shared "not under the plan's project root" clause, which the metrics note on
	// this same pairing also carries.
	assert!(
		stdout.contains(&format!("the ledger {alpha_ledger} is not under the plan's project root")),
		"stdout:\n{stdout}"
	);
	// The METRICS half of the same pairing, still with no explicit `--metrics`.
	assert!(!stdout.contains("2 records"), "no record count; stdout:\n{stdout}");
	for field in ["state:", "streak:", "rounds:", "next:", "role:", "summary:"] {
		assert!(!stdout.contains(field), "`{field}`; stdout:\n{stdout}");
	}

	let (_, stdout, _) =
		run(&home, &["next", "--json", "--source", &alpha_source, "--plan", &beta_plan]);
	assert_eq!(reason(&stdout), "\"resume_state_absent_reason\": \"ledger-not-this-project\",");
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"stdout:\n{stdout}"
	);
	assert!(
		stdout.contains("\"no_active_loop_reason\": \"metrics-not-this-project\""),
		"stdout:\n{stdout}"
	);

	let (_, stdout, _) =
		run(&home, &["status", "--json", "--source", &alpha_source, "--plan", &beta_plan]);
	assert!(
		stdout.contains("\"metrics_absent_reason\": \"log-not-this-project\""),
		"stdout:\n{stdout}"
	);

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 19, ACCEPTED COST (ii) pinned as EXPECTED behaviour on BOTH surfaces,
/// not fixed. Any symlink that makes the canonicalised checked plan and the canonicalised
/// resolved log fall under different roots produces it; the two layouts here are the plan
/// side and the log side of the same divergence.
///
/// This is the mechanism's one known false positive. The judgement taken is that a loud
/// refusal beats a silent wrong file, and the quiet half (a projection that stops
/// reporting round evidence the project legitimately has) is the more expensive part of
/// the cost rather than a separate defect. A change that removes either manifestation
/// should fail here and be argued, not made silently.
#[test]
fn accepted_cost_two_the_symlinked_layouts_are_pinned() {
	let root = scratch("symlinkcost");
	let home = build_home(&root);

	// Layout 1, the PLAN side: `<root>/docs/plans` is a symlink.
	let one = root.join("one");
	fs::create_dir_all(one.join("docs")).unwrap();
	fs::create_dir_all(one.join("elsewhere")).unwrap();
	write(&one.join("docs").join("metrics").join("workflow.jsonl"), &log(&["borrowed-step"]));
	write(&one.join("elsewhere").join("p.plan.toml"), &plan_toml("complete"));
	symlink(&one.join("elsewhere"), &one.join("docs").join("plans"));

	// Layout 2, the LOG side: `<root>/docs/metrics` is a symlink out of the root.
	let two = root.join("two");
	write(&two.join("docs").join("plans").join("p.plan.toml"), &plan_toml("complete"));
	write(&root.join("two-metrics").join("workflow.jsonl"), &log(&["borrowed-step"]));
	fs::create_dir_all(two.join("docs")).unwrap();
	symlink(&root.join("two-metrics"), &two.join("docs").join("metrics"));

	for plan in [
		one.join("docs").join("plans").join("p.plan.toml"),
		two.join("docs").join("plans").join("p.plan.toml"),
	] {
		let plan = arg(&plan);
		// The LOUD manifestation.
		let (code, stdout, stderr) = run(&home, &["validate", "--source", &plan, "--workflow"]);
		assert_eq!(code, Some(1), "{plan}: stdout:\n{stdout}\nstderr:\n{stderr}");
		assert!(
			stderr.contains("is not under the plan's project root"),
			"{plan}: stderr:\n{stderr}"
		);

		// The QUIET one: the same layout has its metrics half omitted at exit 0.
		for command in ["status", "next"] {
			let (code, stdout, stderr) = run(&home, &[command, "--source", &plan]);
			assert_eq!(code, Some(0), "{command} {plan}: stderr:\n{stderr}");
			assert!(
				stdout.contains("metrics: unavailable,"),
				"{command} {plan}: stdout:\n{stdout}"
			);
			assert!(!stdout.contains("1 records"), "{command} {plan}: stdout:\n{stdout}");
		}
	}

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 19b, ACCEPTED COSTS (iii) and (iv) pinned as EXPECTED behaviour, not
/// fixed. ONE project, whose Markdown plan sits outside any `docs/plans` while its
/// TOML source sits inside one: the checked plan's root falls back to `<root>/notes` and
/// the project's own log is not under it.
///
/// Carving this out was DECLINED because it would reverse `Q-55-noconvention`, which
/// decided the containment refusal is layered ON TOP of that fallback rather than
/// suppressed by it. Cost (iv) is wider than (iii): `status --resume` omits the block in
/// EITHER `primary` spelling, since that surface never consults `primary` at all.
#[test]
fn accepted_costs_three_and_four_are_pinned() {
	let root = scratch("conventionless");
	let home = build_home(&root);

	let solo = root.join("solo");
	write(&solo.join("docs").join("metrics").join("workflow.jsonl"), &log(&["borrowed-step"]));
	write(&solo.join("docs").join("plans").join("x.ledger.md"), &resume_block("SOLO"));
	write(&solo.join("notes").join("p.md"), &plan_markdown("complete"));
	let notes_plan = arg(&solo.join("notes").join("p.md"));
	let source = solo.join("docs").join("plans").join("x.plan.toml");

	for primary in ["markdown", "toml"] {
		write(
			&source,
			&format!(
				"[meta]\ntitle = \"Solo\"\nprimary = \"{primary}\"\n\n\
				 [[step]]\nslug = \"its-own-step\"\ntitle = \"Its own step\"\nstatus = \"not-started\"\norder = 1\n"
			),
		);
		let source_arg = arg(&source);

		// Cost (iv): the block is omitted in EITHER spelling.
		let (code, stdout, stderr) =
			run(&home, &["status", "--resume", "--source", &source_arg, "--plan", &notes_plan]);
		assert_eq!(code, Some(0), "stderr:\n{stderr}");
		assert!(!stdout.contains("SOLO resume state."), "{primary}: stdout:\n{stdout}");
	}

	// Cost (iii) is the Markdown-primary spelling only, since a TOML-primary source is
	// itself the checked plan and roots the predicate in `docs/plans`.
	write(
		&source,
		"[meta]\ntitle = \"Solo\"\nprimary = \"markdown\"\n\n\
		 [[step]]\nslug = \"its-own-step\"\ntitle = \"Its own step\"\nstatus = \"not-started\"\norder = 1\n",
	);
	let source_arg = arg(&source);
	let (code, stdout, stderr) =
		run(&home, &["validate", "--source", &source_arg, "--plan", &notes_plan, "--workflow"]);
	assert_eq!(code, Some(1), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert!(!stdout.contains("workflow invariants hold"), "stdout:\n{stdout}");
	assert!(stderr.contains(&arg(&solo.join("notes"))), "the root is `notes`; stderr:\n{stderr}");

	for command in ["status", "next"] {
		let (code, stdout, stderr) =
			run(&home, &[command, "--source", &source_arg, "--plan", &notes_plan]);
		assert_eq!(code, Some(0), "{command}: stderr:\n{stderr}");
		assert!(stdout.contains("metrics: unavailable,"), "{command}: stdout:\n{stdout}");
	}

	let _ = fs::remove_dir_all(&root);
}

/// Acceptance check 14h: the CORRECT case is unchanged on the machine surface except for
/// the new always-present fields. They serialise as explicit `null`s rather than
/// vanishing, matching the struct's existing no-`skip_serializing_if` convention.
#[test]
fn a_correct_run_serialises_the_new_reasons_as_null() {
	let root = scratch("nullfields");
	let home = build_home(&root);
	write(&home.join("docs").join("plans").join("p.ledger.md"), &resume_block("HOME"));
	// An in-progress step so there is a loop to project.
	write(&home.join("docs").join("plans").join("p.plan.toml"), &plan_toml("in-progress"));

	let (code, stdout, stderr) =
		run(&home, &["next", "--json", "--source", "docs/plans/p.plan.toml"]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("\"metrics_absent_reason\": null"), "stdout:\n{stdout}");
	assert!(stdout.contains("\"resume_state_absent_reason\": null"), "stdout:\n{stdout}");
	assert!(stdout.contains("\"no_active_loop_reason\": null"), "stdout:\n{stdout}");
	assert!(
		stdout.contains("\"records\": 3"),
		"the pre-existing fields keep their values; stdout:\n{stdout}"
	);

	let (code, stdout, stderr) =
		run(&home, &["status", "--json", "--source", "docs/plans/p.plan.toml"]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(stdout.contains("\"metrics_absent_reason\": null"), "stdout:\n{stdout}");

	let _ = fs::remove_dir_all(&root);
}

/// Create a symlink at `link` pointing at `target`. Unix-only, like the rest of the
/// symlink fixtures here; the crate's supported platforms are Unix.
fn symlink(
	target: &Path,
	link: &Path,
) {
	fs::create_dir_all(link.parent().unwrap()).unwrap();
	std::os::unix::fs::symlink(target, link).unwrap();
}
