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
	let beta = root.join("beta");
	write(&beta.join("docs").join("plans").join("p.md"), &plan_markdown("in progress"));
	let beta_plan = arg(&beta.join("docs").join("plans").join("p.md"));

	let (code, stdout, stderr) =
		run(&home, &["next", "--source", &alpha_source, "--plan", &beta_plan]);
	assert_eq!(code, Some(0), "stderr:\n{stderr}");
	assert!(!stdout.contains("ALPHA resume state."), "no line of the block; stdout:\n{stdout}");
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
