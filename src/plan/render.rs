//! The `render` engine (Inc 3, Q-45/Q-46 design B): a strict, pure
//! `(plan.toml, sidecars) -> <task>.md` projection, plus the `render --check` guard.
//!
//! One direction only. Render reads the typed `<task>.plan.toml` skeleton and its
//! opaque Markdown sidecars and writes exactly one derived file, `<task>.md`. It
//! never parses a sidecar or the generated file back into structure and never writes
//! a sidecar or the TOML, so there is no prose round-trip and nothing to clobber
//! (Principle 16, one source of truth; the sidecars and the TOML are the sources, the
//! `.md` is a projection). The generated fragments render three drift-prone things
//! from data instead of hand-maintained prose: the Status line (the step-status
//! distribution + open-question count + waiver counts, derived, never a stored
//! field), the status-vocabulary fragment (from the `plan::ROADMAP_STATUSES` code
//! constants, closing the B3 drift), and the numbered Project Principles.
//!
//! Strict, fail-loud (Principle 12). Render runs the `validate_source` cross-reference
//! checks FIRST and, on any schema violation or unresolved cross-reference, returns
//! the problems and writes NOTHING; likewise a missing referenced sidecar is a hard
//! failure, not a silent gap, so a broken source can never yield a partial plan an
//! agent then reads as authoritative.
//!
//! The `render --check` mechanism is a BYTE-FOR-BYTE compare of a fresh in-memory
//! render against the committed `<task>.md` (design A of the synthesis), so the
//! committed `.md` is the golden and render never has to write the TOML to keep a
//! stored hash fresh. `[meta].render_sha256` is therefore VESTIGIAL under this
//! increment (it is left in the schema for a later increment to reconcile, per the
//! Inc 3 brief's out-of-scope note); the byte compare catches both a hand-edit of the
//! generated file and a stale render after a source edit, with no second source of
//! truth to keep current.

use {
	super::{
		ROADMAP_BLOCKED_PREFIX,
		ROADMAP_STATUSES,
		parse_toml,
		source::{
			PlanToml,
			Principle,
			Provenance,
			Question,
			QuestionStatus,
			Step,
			StepStatus,
			Waiver,
		},
		validate_source,
	},
	crate::metrics::{
		WaiverReason,
		WaiverUnit,
		question_id_index,
	},
	std::{
		fmt::Write as _,
		fs,
		io,
		path::{
			Path,
			PathBuf,
		},
	},
};

/// The suffix a structured-source file carries; the leading `<task>` is the part
/// before it, and the rendered view is `<task>.md`.
const PLAN_TOML_SUFFIX: &str = ".plan.toml";

/// The result of `render --check`: the committed `<task>.md` either matches a fresh
/// render or does not (a hand-edit or a stale render), carrying enough to report the
/// mismatch. `Mismatch` distinguishes an absent committed file (never rendered) from a
/// present-but-different one, so the caller can word the two cases clearly.
#[derive(Debug)]
pub(crate) enum CheckOutcome {
	/// The committed `<task>.md` is byte-identical to a fresh render.
	Match,
	/// The committed `<task>.md` differs from a fresh render (or is absent).
	Mismatch {
		/// The freshly rendered bytes (what the committed file should contain).
		expected: String,
		/// The committed file's bytes, empty when it does not exist.
		committed: String,
		/// Whether a committed `<task>.md` exists at all.
		committed_exists: bool,
	},
}

/// The `<task>` name a `<task>.plan.toml` path encodes: its file name with the
/// `.plan.toml` suffix removed. A path whose file name does not end with that suffix
/// is a usage error (render is driven by a `.plan.toml`), reported rather than
/// guessed.
fn task_name(plan_path: &Path) -> Result<String, Vec<String>> {
	let name = plan_path
		.file_name()
		.and_then(|n| n.to_str())
		.ok_or_else(|| vec![format!("{}: not a readable file name", plan_path.display())])?;
	match name.strip_suffix(PLAN_TOML_SUFFIX) {
		Some(task) if !task.is_empty() => Ok(task.to_string()),
		_ => Err(vec![format!(
			"{}: a render source must be named `<task>{PLAN_TOML_SUFFIX}`",
			plan_path.display()
		)]),
	}
}

/// The directory a `<task>.plan.toml` sits in, against which its sidecar references
/// and its rendered output are resolved. A path with no parent (a bare file name)
/// resolves against the current directory.
fn base_dir(plan_path: &Path) -> &Path {
	match plan_path.parent() {
		Some(parent) if !parent.as_os_str().is_empty() => parent,
		_ => Path::new("."),
	}
}

/// The path render writes (and `render --check` compares against): `<task>.md`
/// beside the `<task>.plan.toml`. Public so the CLI writes and checks the same path
/// the engine derives (Principle 16).
pub(crate) fn rendered_path(plan_path: &Path) -> Result<PathBuf, Vec<String>> {
	let task = task_name(plan_path)?;
	Ok(base_dir(plan_path).join(format!("{task}.md")))
}

/// Read a referenced sidecar as an opaque byte blob (as UTF-8 text), returning a
/// human-readable problem on a missing or unreadable file. Render never interprets
/// the bytes; a missing sidecar is a hard failure (Principle 12).
fn read_sidecar(path: &Path) -> Result<String, String> {
	fs::read_to_string(path)
		.map_err(|error| format!("missing or unreadable sidecar {}: {error}", path.display()))
}

/// Render the `<task>.plan.toml` at `plan_path` and its sidecars into the `<task>.md`
/// bytes. PURE with respect to output: it reads the TOML and the sidecars and returns
/// the rendered string, writing nothing (the caller decides whether to write or
/// compare). On ANY schema violation, unresolved cross-reference, or missing sidecar
/// it returns the collected problems and produces no output.
pub(crate) fn render_plan(plan_path: &Path) -> Result<String, Vec<String>> {
	// Derive the task/output name from the path shape FIRST, before touching the
	// filesystem, so a mis-named source is a clear usage error rather than a read error.
	let task = task_name(plan_path)?;
	let base = base_dir(plan_path);

	let contents = fs::read_to_string(plan_path)
		.map_err(|error| vec![format!("{}: {error}", plan_path.display())])?;

	// Strict, fail-loud: run the full cross-reference validation first. On any problem
	// return it and render nothing, so a broken source never yields a partial plan.
	let problems = validate_source(&contents);
	if !problems.is_empty() {
		return Err(problems);
	}
	// A clean validate implies a clean parse; treat a disagreement as a hard error
	// rather than unwrapping.
	let plan =
		parse_toml(&contents).map_err(|error| vec![format!("{}: {error}", plan_path.display())])?;

	// Read every referenced sidecar up front, collecting ALL missing ones so a single
	// run reports the whole gap rather than only the first hole.
	let mut missing: Vec<String> = Vec::new();
	let mut load = |path: &Path| match read_sidecar(path) {
		Ok(text) => text,
		Err(problem) => {
			missing.push(problem);
			String::new()
		}
	};

	let front_blobs: Vec<String> =
		plan.meta.sidecars.front.iter().map(|reference| load(&base.join(reference))).collect();
	let tail_blob: Option<String> =
		plan.meta.sidecars.tail.as_ref().map(|reference| load(&base.join(reference)));

	// Step details and question bodies use the fixed path convention derived from the
	// task name and the slug/id: `<task>.steps/<slug>.md` and `<task>.questions/<id>.md`.
	let steps_dir = base.join(format!("{task}.steps"));
	let questions_dir = base.join(format!("{task}.questions"));

	let mut steps: Vec<&Step> = plan.steps.iter().collect();
	steps.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.slug.cmp(&b.slug)));
	let step_blobs: Vec<(&Step, String)> = steps
		.iter()
		.map(|step| (*step, load(&steps_dir.join(format!("{}.md", step.slug)))))
		.collect();

	let mut questions: Vec<&Question> = plan.questions.iter().collect();
	questions.sort_by_key(|question| question_id_index(&question.id));
	let question_blobs: Vec<(&Question, String)> = questions
		.iter()
		.map(|question| (*question, load(&questions_dir.join(format!("{}.md", question.id)))))
		.collect();

	if !missing.is_empty() {
		return Err(missing);
	}

	Ok(assemble(&plan, &task, &front_blobs, tail_blob.as_deref(), &step_blobs, &question_blobs))
}

impl CheckOutcome {
	/// A concise, one-line description of where a `Mismatch`'s committed file first
	/// diverges from the fresh render, for the `render --check` report. Names the
	/// first differing line (1-based) with both sides trimmed to a readable length, or
	/// the length difference when one side is a prefix of the other. `Match` has no
	/// difference to describe.
	pub(crate) fn difference_summary(&self) -> Option<String> {
		let CheckOutcome::Mismatch {
			expected,
			committed,
			..
		} = self
		else {
			return None;
		};
		for (index, (want, got)) in expected.lines().zip(committed.lines()).enumerate() {
			if want != got {
				return Some(format!(
					"first difference at line {}: expected {:?}, committed {:?}",
					index + 1,
					truncate(want),
					truncate(got)
				));
			}
		}
		// No differing line up to the shorter length. Either one side has extra trailing
		// lines, or the two differ ONLY in trailing whitespace / a trailing newline, which
		// `str::lines()` drops (so both the line-by-line and the count-by-count compare look
		// equal). A byte compare distinguishes the two, so the summary is not the degenerate
		// "the committed file has N line(s); a fresh render has N".
		let committed_lines = committed.lines().count();
		let expected_lines = expected.lines().count();
		if committed_lines != expected_lines {
			Some(format!(
				"the committed file has {committed_lines} line(s); a fresh render has {expected_lines}"
			))
		} else {
			Some(format!(
				"the files differ only in trailing whitespace or a trailing newline (committed {} \
				 bytes, a fresh render {} bytes)",
				committed.len(),
				expected.len()
			))
		}
	}
}

/// Trim a line to a readable length for a diff report, marking truncation.
fn truncate(line: &str) -> String {
	const LIMIT: usize = 80;
	if line.chars().count() <= LIMIT {
		line.to_string()
	} else {
		format!("{}...", line.chars().take(LIMIT).collect::<String>())
	}
}

/// Re-render `plan_path` in memory and compare byte-for-byte against the committed
/// `<task>.md`. Returns the outcome; a render failure (broken source) propagates as
/// the render problems, since there is no golden to check a broken source against.
pub(crate) fn check_render(plan_path: &Path) -> Result<CheckOutcome, Vec<String>> {
	let expected = render_plan(plan_path)?;
	let out_path = rendered_path(plan_path)?;
	match fs::read_to_string(&out_path) {
		Ok(committed) if committed == expected => Ok(CheckOutcome::Match),
		Ok(committed) => Ok(CheckOutcome::Mismatch {
			expected,
			committed,
			committed_exists: true,
		}),
		// Only a genuine absence (`NotFound`) reports the committed file as never-rendered.
		Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(CheckOutcome::Mismatch {
			expected,
			committed: String::new(),
			committed_exists: false,
		}),
		// Any other read failure (invalid UTF-8, a permission error) is a present-but-
		// unreadable file, reported as its own error rather than miscounted as absent.
		Err(error) => Err(vec![format!(
			"{}: committed file present but unreadable: {error}",
			out_path.display()
		)]),
	}
}

/// Assemble the rendered `<task>.md` from the parsed plan and the already-loaded
/// opaque sidecar blobs. Pure and deterministic: every ordering is fixed (steps by
/// `order` then slug, questions by `Q-<n>` index, principles by `n`), so rendering
/// twice yields identical bytes.
fn assemble(
	plan: &PlanToml,
	task: &str,
	front_blobs: &[String],
	tail_blob: Option<&str>,
	step_blobs: &[(&Step, String)],
	question_blobs: &[(&Question, String)],
) -> String {
	let mut sections: Vec<String> = Vec::new();
	sections.push(banner(task));
	sections.push(format!("# {} plan", plan.meta.title));
	sections.push(status_line(plan));

	// Front prose sidecars, inlined verbatim in their declared order (each carries its
	// own headings; render stays dumb).
	for blob in front_blobs {
		let trimmed = blob.trim_end();
		if !trimmed.is_empty() {
			sections.push(trimmed.to_string());
		}
	}

	sections.push(principles_section(&plan.principles));
	sections.push(vocabulary_section());
	sections.push(questions_section(question_blobs));
	sections.push(roadmap_section(step_blobs));
	// The two details sections carry only a heading when no item has body prose, so
	// guard them the way the front/tail prose pushes are guarded: an empty section
	// (no step/question bodies) emits NOTHING rather than a bare dangling heading.
	if let Some(section) = step_details_section(step_blobs) {
		sections.push(section);
	}
	if let Some(section) = question_details_section(question_blobs) {
		sections.push(section);
	}

	if let Some(tail) = tail_blob {
		let trimmed = tail.trim_end();
		if !trimmed.is_empty() {
			sections.push(trimmed.to_string());
		}
	}

	// One blank line between sections, one trailing newline at the end of file.
	let mut out = sections.join("\n\n");
	out.push('\n');
	out
}

/// The do-not-hand-edit banner heading the generated file. It names the real sources
/// (the `.plan.toml`, the sidecar directories, and the `[meta].sidecars` prose) by
/// their task-relative names, so the banner is deterministic regardless of where the
/// repository is checked out (no absolute paths).
fn banner(task: &str) -> String {
	format!(
		"<!-- GENERATED FILE - do not hand-edit. Source: {task}{PLAN_TOML_SUFFIX}, {task}.steps/, \
		 {task}.questions/, and the [meta].sidecars prose (front/tail). Regenerate with \
		 `agent-scaffold render {task}{PLAN_TOML_SUFFIX}`; hand edits are overwritten and caught by \
		 `agent-scaffold render --check`. -->"
	)
}

/// The DERIVED Status line: the step-status distribution, the open-question count,
/// and the waiver counts, all read from the TOML (never a stored field). Fixed
/// orderings (`StepStatus::ALL`, then the waiver reasons in a fixed order) keep it
/// deterministic.
fn status_line(plan: &PlanToml) -> String {
	let mut distribution: Vec<String> = Vec::new();
	for status in StepStatus::ALL {
		let count = plan.steps.iter().filter(|step| step.status == status).count();
		if count > 0 {
			distribution.push(format!("{count} {}", status.label()));
		}
	}
	let distribution =
		if distribution.is_empty() { "no steps".to_string() } else { distribution.join(", ") };

	// Both `open` and `exploring` are UNRESOLVED (a decision is still owed; `exploring`
	// is a documented sub-state of open, per AGENTS.md and the schema), so the derived
	// count of outstanding questions includes both. `decided` and `superseded` are
	// resolved and excluded.
	let open_questions = plan
		.questions
		.iter()
		.filter(|question| {
			matches!(question.status, QuestionStatus::Open | QuestionStatus::Exploring)
		})
		.count();

	let waivers: Vec<&Waiver> = plan.steps.iter().flat_map(|step| &step.waivers).collect();
	let waiver_summary = if waivers.is_empty() {
		"0 waivers".to_string()
	} else {
		let mut by_reason: Vec<String> = Vec::new();
		for reason in WaiverReason::ALL {
			let count = waivers.iter().filter(|waiver| waiver.reason == reason).count();
			if count > 0 {
				by_reason.push(format!("{count} {}", reason.label()));
			}
		}
		format!("{} waivers ({})", waivers.len(), by_reason.join(", "))
	};

	format!(
		"Status: {} steps ({distribution}); {open_questions} open questions; {waiver_summary}.",
		plan.steps.len()
	)
}

/// The numbered Project Principles, rendered from the `[[principle]]` array by each
/// entry's own `n` (not a running counter), sorted by `n`.
fn principles_section(principles: &[Principle]) -> String {
	let mut sorted: Vec<&Principle> = principles.iter().collect();
	sorted.sort_by_key(|principle| principle.n);
	let mut out = String::from("## Project Principles\n");
	for principle in sorted {
		let _ = write!(out, "\n{}. {} - {}", principle.n, principle.name, principle.text);
	}
	out
}

/// The generated status-vocabulary fragment: the accepted Roadmap and queue statuses,
/// listed from the code constants (`ROADMAP_STATUSES` + the parametric blocked form,
/// and `QuestionStatus::ALL`) rather than a hand-copied prose list, so the vocabulary
/// cannot drift from what `validate` enforces (B3).
fn vocabulary_section() -> String {
	let roadmap = ROADMAP_STATUSES.join(", ");
	let queue =
		QuestionStatus::ALL.iter().map(|status| status.label()).collect::<Vec<_>>().join(", ");
	format!(
		"## Roadmap Status Vocabulary\n\nGenerated status vocabulary (from the code constants, so it \
		 cannot drift):\n\n- Roadmap statuses: {roadmap}, {ROADMAP_BLOCKED_PREFIX}<slug>.\n- Queue \
		 statuses: {queue}."
	)
}

/// The Open-Questions queue: strictly ONE line per item (id, status, ask, and the
/// resolving `folded_into` / `superseded_by` / `receipt` pointers shown
/// appropriately), so the queue stays scannable. The opaque body sidecars are
/// relocated to `## Question Details` (mirroring `## Step Details`), not inlined here,
/// so a blank line before a body never fragments the Markdown list. Items are already
/// ordered by `Q-<n>` index by the caller.
fn questions_section(question_blobs: &[(&Question, String)]) -> String {
	let mut out = String::from("## Open Questions, Decisions, Issues and Blockers\n");
	for (question, _) in question_blobs {
		let status = question_status_display(question);
		let receipt = match &question.receipt {
			Some(receipt) => format!(" Receipt: `{receipt}`."),
			None => String::new(),
		};
		let _ = write!(out, "\n- `{}` ({status}) {}{receipt}", question.id, question.ask);
	}
	out
}

/// The parenthesised queue status for a rendered item: the plain label for `open` /
/// `exploring`, the parametric `decided -> folded into <slug>` for a decided item
/// (the resolving `folded_into` is guaranteed present by `validate_source`), and
/// `superseded by <slug>` for a superseded one.
fn question_status_display(question: &Question) -> String {
	match question.status {
		QuestionStatus::Open | QuestionStatus::Exploring => question.status.label().to_string(),
		QuestionStatus::Decided =>
			format!("decided -> folded into `{}`", question.folded_into.as_deref().unwrap_or("")),
		QuestionStatus::Superseded => {
			format!("superseded by `{}`", question.superseded_by.as_deref().unwrap_or(""))
		}
	}
}

/// The Roadmap table from `[[step]]` (slug, status, order), with a Notes column that
/// renders each `blocked_by` slug as `blocked on <slug>` and each `[[step.waiver]]`
/// inline (waivers live in the TOML, per Q-46). Rows are in the caller's fixed order.
fn roadmap_section(step_blobs: &[(&Step, String)]) -> String {
	let mut out = String::from("## Roadmap\n\n| Step | Status | Notes |\n| --- | --- | --- |");
	for (step, _) in step_blobs {
		let _ = write!(
			out,
			"\n| `{}` | {} | {} |",
			step.slug,
			step.status.label(),
			escape_cell(&notes_cell(step))
		);
	}
	out
}

/// The Notes cell for a Roadmap row: the `blocked on <slug>` markers (from
/// `blocked_by`), then the inline waiver descriptors, then the provenance fragment
/// (from `[step.provenance]`), joined with `; `. The provenance fragment is appended
/// LAST so the blocked/waived state a reader scans for stays leftmost. Empty when the
/// step is neither blocked, waived, nor provenanced.
fn notes_cell(step: &Step) -> String {
	let mut notes: Vec<String> = Vec::new();
	for blocker in &step.blocked_by {
		notes.push(format!("{ROADMAP_BLOCKED_PREFIX}`{blocker}`"));
	}
	for waiver in &step.waivers {
		notes.push(waiver_note(waiver));
	}
	if let Some(provenance) = &step.provenance {
		notes.push(provenance_note(provenance));
	}
	notes.join("; ")
}

/// The inline "why this step exists" descriptor for a step's provenance: the
/// present sub-lists in the fixed decisions -> findings -> commits order, each in the
/// TOML's source order (a `Vec`, already deterministic). Renders only the non-empty
/// lists, so a provenance carrying only decisions shows only `decisions ...`. Mirrors
/// how a question renders its `receipt`: a TOML-derived per-item fact on the scannable
/// line. `validate_source` guarantees at least one list is non-empty, so the fragment
/// is never a bare `why:` (rendered defensively if it ever is).
fn provenance_note(provenance: &Provenance) -> String {
	let mut parts: Vec<String> = Vec::new();
	if !provenance.decisions.is_empty() {
		parts.push(format!("decisions {}", provenance.decisions.join(", ")));
	}
	if !provenance.findings.is_empty() {
		parts.push(format!("findings {}", provenance.findings.join(", ")));
	}
	if !provenance.commits.is_empty() {
		parts.push(format!("commits {}", provenance.commits.join(", ")));
	}
	format!("why: {}", parts.join("; "))
}

/// The inline descriptor for a waiver in the Roadmap Notes: the unit (a whole step,
/// or a named increment), the reason, and the evidence tier, plus the optional human
/// note. Reads only the TOML `[[step.waiver]]` fields (the Q-46 re-home).
fn waiver_note(waiver: &Waiver) -> String {
	let unit = match (waiver.unit, waiver.increment.as_deref()) {
		(WaiverUnit::Increment, Some(increment)) => format!("increment `{increment}`"),
		// `validate_source` guarantees an increment-unit waiver carries its increment
		// and a step-unit one does not, so the mixed arms below are unreachable in a
		// valid source; render them defensively rather than panicking.
		(WaiverUnit::Increment, None) => "increment".to_string(),
		(WaiverUnit::Step, _) => "step".to_string(),
	};
	let mut note =
		format!("waived: {unit} {} ({})", waiver.reason.label(), waiver.evidence_tier.label());
	if let Some(text) = &waiver.note {
		let _ = write!(note, " - {text}");
	}
	note
}

/// Escape a generated Markdown table cell so a stray `|` cannot break the table and no
/// line ending can split the row. CommonMark treats a lone `\r`, a lone `\n`, and a
/// `\r\n` all as line endings, so every form is neutralised to a space (a `\r\n` pair
/// collapses to a single space). Applies only to generated cell text (the Roadmap
/// Notes), not to opaque sidecar prose, which is never placed in a table.
fn escape_cell(text: &str) -> String {
	text.replace('|', "\\|").replace("\r\n", " ").replace(['\n', '\r'], " ")
}

/// The Step Details section: each step's opaque body sidecar inlined verbatim, in the
/// caller's fixed order (each sidecar carries its own `### <slug>` heading). Returns
/// `None` when no step contributes a non-empty body, so the caller emits no bare
/// heading (a step with no body prose contributes no entry).
fn step_details_section(step_blobs: &[(&Step, String)]) -> Option<String> {
	let mut out = String::from("## Step Details");
	let mut any = false;
	for (_, body) in step_blobs {
		let trimmed = body.trim_end();
		if !trimmed.is_empty() {
			let _ = write!(out, "\n\n{trimmed}");
			any = true;
		}
	}
	any.then_some(out)
}

/// The Question Details section: each question's opaque body sidecar inlined verbatim,
/// in the caller's fixed `Q-<n>` order (each sidecar carries its own prose). Mirrors
/// `step_details_section`, including returning `None` when no question contributes a
/// non-empty body, so the Open-Questions queue stays one line per item and a
/// question-free (or all-empty) plan emits no bare heading.
fn question_details_section(question_blobs: &[(&Question, String)]) -> Option<String> {
	let mut out = String::from("## Question Details");
	let mut any = false;
	for (_, body) in question_blobs {
		let trimmed = body.trim_end();
		if !trimmed.is_empty() {
			let _ = write!(out, "\n\n{trimmed}");
			any = true;
		}
	}
	any.then_some(out)
}

/// Atomically write the rendered `<task>.md`: write the bytes to a temp file in the
/// SAME directory, then `fs::rename` it into place. `fs::rename` within a directory is
/// atomic on the platforms this runs on, so an interrupted write (disk full, a kill,
/// a permission change mid-write) leaves a previously-valid `<task>.md` intact rather
/// than truncated or partial, extending the increment's "never a partial plan" intent
/// to the success path. Public so the CLI writes through the same atomic path.
pub(crate) fn write_rendered(
	out_path: &Path,
	rendered: &str,
) -> io::Result<()> {
	let dir = out_path.parent().filter(|parent| !parent.as_os_str().is_empty());
	let file_name = out_path.file_name().and_then(|name| name.to_str()).ok_or_else(|| {
		io::Error::other(format!("{}: not a writable file name", out_path.display()))
	})?;
	// A same-directory temp sibling, so the final `rename` stays on one filesystem (a
	// cross-device rename is not atomic and would fail). The pid keeps concurrent renders
	// from colliding on the temp name.
	let temp_name = format!(".{file_name}.{}.tmp", std::process::id());
	let temp_path = match dir {
		Some(parent) => parent.join(&temp_name),
		None => PathBuf::from(&temp_name),
	};
	// Write the bytes, then atomically rename into place. On ANY failure remove the
	// temp sibling so none leaks: a partial write (disk-full, an I/O error, a kill
	// mid-write) leaves a `.<task>.md.<pid>.tmp` dotfile the `?` would otherwise skip,
	// and a failed rename leaves the written temp behind. Either way the pre-existing
	// `<task>.md` is untouched, since it is never opened until the rename.
	let write_then_rename =
		fs::write(&temp_path, rendered).and_then(|()| fs::rename(&temp_path, out_path));
	if write_then_rename.is_err() {
		let _ = fs::remove_file(&temp_path);
	}
	write_then_rename
}

#[cfg(test)]
mod tests {
	use {
		super::*,
		std::path::PathBuf,
	};

	/// The testdata directory holding the render fixture (`<task>.plan.toml` + its
	/// sidecar tree + the golden `<task>.md`), resolved from the crate root so tests
	/// operate on the real on-disk fixture files render reads.
	fn testdata() -> PathBuf {
		PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/plan/testdata")
	}

	/// The render fixture's `<task>.plan.toml` path.
	fn fixture_plan() -> PathBuf {
		testdata().join("render-fixture.plan.toml")
	}

	/// The committed golden `<task>.md`, embedded so the byte-stability assertion does
	/// not depend on a filesystem read.
	const GOLDEN: &str = include_str!("testdata/render-fixture.md");

	/// A scratch directory for a single test that writes files, kept off the live tree.
	fn scratch(name: &str) -> PathBuf {
		let dir = std::env::temp_dir()
			.join(format!("agent-scaffold-render-{}-{name}", std::process::id()));
		let _ = fs::remove_dir_all(&dir);
		fs::create_dir_all(&dir).unwrap();
		dir
	}

	/// Copy the whole render fixture (the `.plan.toml`, the sidecar directories, and
	/// the front/tail prose sidecars, but NOT the golden `.md`) into `dest`, so a
	/// mutation test can edit an isolated copy without touching the committed fixture.
	fn copy_fixture_sources(dest: &Path) {
		let src = testdata();
		fs::copy(src.join("render-fixture.plan.toml"), dest.join("render-fixture.plan.toml"))
			.unwrap();
		for reference in [
			"render-fixture.intro.md",
			"render-fixture.motivations.md",
			"render-fixture.repo-layout.md",
			"render-fixture.success-criteria.md",
		] {
			fs::copy(src.join(reference), dest.join(reference)).unwrap();
		}
		for dir in ["render-fixture.steps", "render-fixture.questions"] {
			let from = src.join(dir);
			let to = dest.join(dir);
			fs::create_dir_all(&to).unwrap();
			for entry in fs::read_dir(&from).unwrap() {
				let entry = entry.unwrap();
				fs::copy(entry.path(), to.join(entry.file_name())).unwrap();
			}
		}
	}

	#[test]
	fn render_is_deterministic_and_matches_the_golden() {
		let first = render_plan(&fixture_plan()).expect("fixture renders");
		let second = render_plan(&fixture_plan()).expect("fixture renders again");
		// Byte-stable: two renders of the same source are identical.
		assert_eq!(first, second, "render must be deterministic");
		// The committed golden matches the fresh render (so `render --check` is green).
		assert_eq!(first, GOLDEN, "the committed golden must match a fresh render");
	}

	#[test]
	fn the_committed_golden_passes_check() {
		match check_render(&fixture_plan()).expect("check runs") {
			CheckOutcome::Match => {}
			other => panic!("expected the committed golden to match, got {other:?}"),
		}
	}

	#[test]
	fn the_rendered_document_carries_every_generated_fragment() {
		let out = render_plan(&fixture_plan()).expect("renders");
		// The do-not-edit banner heads the file and names the real sources.
		assert!(out.starts_with("<!-- GENERATED FILE - do not hand-edit."), "{out}");
		assert!(out.contains("render-fixture.plan.toml"), "banner names the TOML source");
		// The title heading from `[meta].title`.
		assert!(out.contains("# Render fixture plan"), "{out}");
		// The derived Status line: the full distribution (all seven statuses), the open +
		// exploring unresolved-question count (Q-1 open, Q-9 exploring, Q-10 open = 3), and
		// the waiver counts.
		assert!(
			out.contains(
				"Status: 7 steps (1 not started, 1 in progress, 1 complete, 1 skipped, 1 next, 1 \
				 optional, 1 deferred); 3 open questions; 2 waivers (1 predates-logging, 1 \
				 accepted-at-escalation)."
			),
			"{out}"
		);
		// The generated vocabulary fragment, under its own heading (renamed off the live
		// plan's `## Documentation Protocol`), from the code constants (B3).
		assert!(out.contains("## Roadmap Status Vocabulary"), "{out}");
		assert!(out.contains("Roadmap statuses: not started, in progress, complete"), "{out}");
		assert!(out.contains("blocked on <slug>."), "{out}");
		assert!(out.contains("Queue statuses: open, exploring, decided, superseded."), "{out}");
		// The Roadmap renders `blocked_by` as `blocked on <slug>` and a waiver inline.
		assert!(
			out.contains(
				"| `beta` | in progress | blocked on `alpha`; waived: step predates-logging \
				 (self-declared) |"
			),
			"{out}"
		);
		assert!(
			out.contains("waived: increment `alpha-inc1` accepted-at-escalation (record-backed)"),
			"{out}"
		);
		// The provenance fragment renders LAST in `alpha`'s Notes cell (after the waiver),
		// showing the present sub-lists in decisions -> findings -> commits order. `alpha`
		// carries all three, so the full "why: ..." fragment appears.
		assert!(
			out.contains(
				"why: decisions Q-2; findings render-fixture.findings/alpha.md; commits abc1234"
			),
			"{out}"
		);
		// The queue shows the decided fold target and the receipt pointer.
		assert!(out.contains("- `Q-2` (decided -> folded into `alpha`)"), "{out}");
		assert!(out.contains("Receipt: `Q-2`."), "{out}");
		// A superseded item shows its superseding id.
		assert!(out.contains("- `Q-3` (superseded by `Q-1`)"), "{out}");
		// The spliced sidecar prose is present verbatim (front, step, and tail).
		assert!(out.contains("This is the render fixture intro prose."), "{out}");
		assert!(out.contains("### `alpha`: The first step"), "{out}");
		assert!(out.contains("## Success Criteria"), "{out}");
		// F1: question bodies live in a dedicated `## Question Details` section, not inline
		// in the queue, so the queue stays one line per item. The Q-1 body sits AFTER the
		// section heading, which itself sits after the queue.
		let queue_at = out.find("## Open Questions").expect("queue heading");
		let details_at = out.find("## Question Details").expect("question details heading");
		let q1_body_at = out.find("The Q-1 body.").expect("Q-1 body prose");
		assert!(queue_at < details_at, "the queue precedes Question Details");
		assert!(details_at < q1_body_at, "the Q-1 body sits under Question Details, not the queue");
	}

	#[test]
	fn render_writes_exactly_one_file_and_check_is_green_after() {
		let dir = scratch("write-one");
		copy_fixture_sources(&dir);
		let plan = dir.join("render-fixture.plan.toml");
		// Snapshot every source file's bytes BEFORE the render, so the no-clobber invariant
		// (the increment's principal guarantee: the sources are never rewritten) is asserted,
		// not just commented.
		let before = snapshot_dir(&dir);
		let out_path = write_rendered_via_engine(&plan);
		// Exactly one file was produced, `<task>.md`.
		assert_eq!(out_path, dir.join("render-fixture.md"));
		assert!(out_path.exists());
		// Every pre-existing source is byte-identical after the render.
		let after = snapshot_dir(&dir);
		for (path, bytes) in &before {
			assert_eq!(
				after.get(path).map(Vec::as_slice),
				Some(bytes.as_slice()),
				"render clobbered a source file: {}",
				path.display()
			);
		}
		// The only new file is `<task>.md`; no temp sibling or extra file is left behind.
		let created: Vec<&PathBuf> =
			after.keys().filter(|path| !before.contains_key(*path)).collect();
		assert_eq!(
			created,
			vec![&out_path],
			"render created a file other than <task>.md: {created:?}"
		);
		match check_render(&plan).expect("check") {
			CheckOutcome::Match => {}
			other => panic!("a freshly written render must check clean, got {other:?}"),
		}
		let _ = fs::remove_dir_all(&dir);
	}

	/// Render `plan` and write it through the engine's atomic `write_rendered`, returning
	/// the output path. Used by the no-clobber test so it exercises the real write path.
	fn write_rendered_via_engine(plan: &Path) -> PathBuf {
		let rendered = render_plan(plan).expect("renders");
		let out_path = rendered_path(plan).expect("path");
		write_rendered(&out_path, &rendered).expect("write");
		out_path
	}

	/// Snapshot every regular file under `dir` (recursively) as a path -> bytes map, so a
	/// test can assert which files a render touched or created.
	fn snapshot_dir(dir: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
		let mut out = std::collections::BTreeMap::new();
		let mut stack = vec![dir.to_path_buf()];
		while let Some(current) = stack.pop() {
			for entry in fs::read_dir(&current).unwrap() {
				let path = entry.unwrap().path();
				if path.is_dir() {
					stack.push(path);
				} else {
					let bytes = fs::read(&path).unwrap();
					out.insert(path, bytes);
				}
			}
		}
		out
	}

	#[test]
	fn a_failed_atomic_write_leaves_no_temp_file_behind() {
		// L2: on a write/rename failure the atomic write must remove its temp sibling
		// rather than leak a `.<task>.md.<pid>.tmp` dotfile. A rename onto an existing
		// DIRECTORY fails (EISDIR on Linux) AFTER the temp file is written, exercising the
		// error path.
		let dir = scratch("write-fail");
		let out_path = dir.join("target.md");
		fs::create_dir(&out_path).unwrap();
		let result = write_rendered(&out_path, "some rendered content");
		assert!(result.is_err(), "a rename onto an existing directory must fail");
		// No temp sibling remains anywhere in the directory.
		let leaked: Vec<PathBuf> = fs::read_dir(&dir)
			.unwrap()
			.map(|entry| entry.unwrap().path())
			.filter(|path| {
				path.file_name()
					.and_then(|name| name.to_str())
					.is_some_and(|name| name.ends_with(".tmp"))
			})
			.collect();
		assert!(leaked.is_empty(), "the temp file leaked on the error path: {leaked:?}");
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn empty_details_sections_emit_no_bare_heading() {
		// N1: a plan with no questions (and no step body prose) must not emit a bare
		// `## Question Details` / `## Step Details` heading with nothing beneath it.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
		);
		let plan = parse_toml(source).expect("parses");
		let empty_steps: Vec<(&Step, String)> =
			plan.steps.iter().map(|step| (step, String::new())).collect();
		let out = assemble(&plan, "t", &[], None, &empty_steps, &[]);
		assert!(
			!out.contains("## Question Details"),
			"no questions must emit no Question Details heading: {out}"
		);
		assert!(
			!out.contains("## Step Details"),
			"empty step bodies must emit no Step Details heading: {out}"
		);
		// Non-vacuous: a non-empty step body keeps the heading.
		let bodied_steps: Vec<(&Step, String)> =
			plan.steps.iter().map(|step| (step, "### a\n\nbody".to_string())).collect();
		let out2 = assemble(&plan, "t", &[], None, &bodied_steps, &[]);
		assert!(
			out2.contains("## Step Details"),
			"a non-empty step body keeps the heading: {out2}"
		);
	}

	#[test]
	fn a_one_byte_hand_edit_of_the_generated_file_is_a_mismatch() {
		let dir = scratch("hand-edit");
		copy_fixture_sources(&dir);
		let plan = dir.join("render-fixture.plan.toml");
		let rendered = render_plan(&plan).expect("renders");
		let out_path = rendered_path(&plan).expect("path");
		// Write the correct render, then flip a single byte (a hand-edit).
		fs::write(&out_path, format!("{rendered}x")).unwrap();
		match check_render(&plan).expect("check") {
			CheckOutcome::Mismatch {
				committed_exists, ..
			} => assert!(committed_exists),
			CheckOutcome::Match => panic!("a hand-edited generated file must not match"),
		}
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn a_stale_render_after_a_source_edit_is_a_mismatch() {
		let dir = scratch("stale-source");
		copy_fixture_sources(&dir);
		let plan = dir.join("render-fixture.plan.toml");
		// Commit a correct render, then edit a SOURCE (a step sidecar) without re-rendering.
		let rendered = render_plan(&plan).expect("renders");
		fs::write(rendered_path(&plan).expect("path"), &rendered).unwrap();
		let sidecar = dir.join("render-fixture.steps").join("alpha.md");
		fs::write(&sidecar, "### `alpha`: The first step\n\nEdited body after the render.\n")
			.unwrap();
		match check_render(&plan).expect("check") {
			CheckOutcome::Mismatch {
				committed_exists, ..
			} => assert!(committed_exists),
			CheckOutcome::Match => panic!("a stale render after a source edit must not match"),
		}
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn a_stale_render_after_a_toml_edit_is_a_mismatch() {
		let dir = scratch("stale-toml");
		copy_fixture_sources(&dir);
		let plan = dir.join("render-fixture.plan.toml");
		let rendered = render_plan(&plan).expect("renders");
		fs::write(rendered_path(&plan).expect("path"), &rendered).unwrap();
		// Change a status in the TOML source without re-rendering the committed `.md`.
		let toml = fs::read_to_string(&plan).unwrap();
		let edited = toml.replace("status = \"next\"", "status = \"complete\"");
		assert_ne!(toml, edited, "the replacement must actually change the TOML");
		fs::write(&plan, edited).unwrap();
		match check_render(&plan).expect("check") {
			CheckOutcome::Mismatch {
				..
			} => {}
			CheckOutcome::Match => panic!("a stale render after a TOML edit must not match"),
		}
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn an_absent_committed_file_is_a_mismatch() {
		let dir = scratch("absent");
		copy_fixture_sources(&dir);
		let plan = dir.join("render-fixture.plan.toml");
		// No `<task>.md` was ever written.
		match check_render(&plan).expect("check") {
			CheckOutcome::Mismatch {
				committed_exists, ..
			} => assert!(!committed_exists),
			CheckOutcome::Match => panic!("an absent generated file must not match"),
		}
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn a_missing_sidecar_fails_and_writes_nothing() {
		let dir = scratch("missing-sidecar");
		copy_fixture_sources(&dir);
		let plan = dir.join("render-fixture.plan.toml");
		// Remove a referenced step sidecar.
		fs::remove_file(dir.join("render-fixture.steps").join("gamma.md")).unwrap();
		let problems = render_plan(&plan).expect_err("a missing sidecar must fail render");
		assert!(
			problems.iter().any(|problem| problem.contains("gamma.md")),
			"the missing sidecar must be named: {problems:?}"
		);
		// Nothing was written: no `<task>.md` exists.
		assert!(!dir.join("render-fixture.md").exists(), "render must write nothing on failure");
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn an_unresolved_cross_reference_fails_and_writes_nothing() {
		let dir = scratch("dangling-xref");
		copy_fixture_sources(&dir);
		let plan = dir.join("render-fixture.plan.toml");
		// Point a `blocked_by` at a step that does not exist.
		let toml = fs::read_to_string(&plan).unwrap();
		let edited = toml.replace("blocked_by = [\"alpha\"]", "blocked_by = [\"ghost\"]");
		assert_ne!(toml, edited, "the replacement must actually change the TOML");
		fs::write(&plan, edited).unwrap();
		let problems =
			render_plan(&plan).expect_err("an unresolved cross-reference must fail render");
		assert!(
			problems.iter().any(|problem| problem.contains("ghost")),
			"the dangling reference must be reported: {problems:?}"
		);
		assert!(!dir.join("render-fixture.md").exists(), "render must write nothing on failure");
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn a_schema_invalid_toml_fails_and_writes_nothing() {
		let dir = scratch("schema-invalid");
		copy_fixture_sources(&dir);
		let plan = dir.join("render-fixture.plan.toml");
		// An out-of-set step status is rejected at parse (parse, don't validate).
		let toml = fs::read_to_string(&plan).unwrap();
		let edited = toml.replace("status = \"complete\"", "status = \"done\"");
		assert_ne!(toml, edited, "the replacement must actually change the TOML");
		fs::write(&plan, edited).unwrap();
		let problems = render_plan(&plan).expect_err("a schema-invalid TOML must fail render");
		assert!(
			problems.iter().any(|problem| problem.contains("malformed")),
			"a malformed source must be reported: {problems:?}"
		);
		assert!(!dir.join("render-fixture.md").exists(), "render must write nothing on failure");
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn a_non_plan_toml_path_is_a_usage_error() {
		// A path not named `<task>.plan.toml` cannot derive a task/output name.
		let problems = render_plan(Path::new("not-a-plan.toml")).expect_err("must fail");
		assert!(
			problems.iter().any(|problem| problem.contains(".plan.toml")),
			"the naming requirement must be reported: {problems:?}"
		);
	}

	#[test]
	fn the_status_line_counts_open_and_exploring_but_not_decided_or_superseded() {
		// C1: `open` and `exploring` are both unresolved (a decision is still owed);
		// `decided` and `superseded` are resolved. The derived count is 2, not 1 or 4.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"[[question]]\nid = \"Q-1\"\nstatus = \"open\"\nask = \"o\"\n",
			"[[question]]\nid = \"Q-2\"\nstatus = \"exploring\"\nask = \"e\"\n",
			"[[question]]\nid = \"Q-3\"\nstatus = \"decided\"\nask = \"d\"\nfolded_into = \"a\"\n",
			"[[question]]\nid = \"Q-4\"\nstatus = \"superseded\"\nask = \"s\"\nsuperseded_by = \"Q-1\"\n",
		);
		assert!(validate_source(source).is_empty(), "{:?}", validate_source(source));
		let plan = parse_toml(source).expect("parses");
		let line = status_line(&plan);
		assert!(
			line.contains("2 open questions"),
			"open + exploring are counted, not decided/superseded: {line}"
		);
	}

	#[test]
	fn a_trailing_newline_only_mismatch_reports_a_trailing_newline_difference() {
		// C2: `str::lines()` drops a final newline, so a trailing-`\n`-only mismatch has
		// equal lines AND equal counts; the summary must word it from a byte compare, not
		// the degenerate "N line(s) vs N".
		let outcome = CheckOutcome::Mismatch {
			expected: "one\ntwo\n".to_string(),
			committed: "one\ntwo".to_string(),
			committed_exists: true,
		};
		let summary = outcome.difference_summary().expect("a mismatch has a summary");
		assert!(
			summary.contains("trailing"),
			"the trailing-newline case is worded explicitly: {summary}"
		);
		assert!(
			!summary.contains("has 2 line(s); a fresh render has 2"),
			"not the degenerate N-vs-N message: {summary}"
		);
	}

	#[test]
	fn a_present_but_unreadable_committed_file_is_an_error_not_absent() {
		// C3: invalid UTF-8 is present on disk but unreadable as text; it must be its own
		// error, not collapsed into `committed_exists: false` ("does not exist").
		let dir = scratch("unreadable");
		copy_fixture_sources(&dir);
		let plan = dir.join("render-fixture.plan.toml");
		let out_path = rendered_path(&plan).expect("path");
		fs::write(&out_path, [0xff, 0xfe, 0x00]).unwrap();
		let problems = check_render(&plan)
			.expect_err("a present-but-unreadable committed file must be an error");
		assert!(
			problems.iter().any(|problem| problem.contains("unreadable")),
			"the present-but-unreadable file must be reported as such: {problems:?}"
		);
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn escape_cell_neutralizes_carriage_returns_and_newlines() {
		// R2: a lone `\r`, a lone `\n`, and a `\r\n` are all CommonMark line endings; none
		// may pass into a table cell. A `|` stays escaped.
		assert_eq!(escape_cell("a\rb"), "a b");
		assert_eq!(escape_cell("a\r\nb"), "a b");
		assert_eq!(escape_cell("a\nb"), "a b");
		assert_eq!(escape_cell("a|b"), "a\\|b");
	}

	#[test]
	fn ordering_is_numeric_for_questions_and_slug_tiebroken_for_equal_order_steps() {
		// C5: pin the increment's central ordering claims against a lexical-sort regression.
		let out = render_plan(&fixture_plan()).expect("renders");
		// Numeric question order: Q-1, Q-9, Q-10 (a lexical sort would put Q-10 after Q-1).
		let q1 = out.find("- `Q-1`").expect("Q-1 in queue");
		let q9 = out.find("- `Q-9`").expect("Q-9 in queue");
		let q10 = out.find("- `Q-10`").expect("Q-10 in queue");
		assert!(q1 < q9 && q9 < q10, "questions sort numerically (Q-1, Q-9, Q-10): {out}");
		// Equal-order steps break the tie by slug: `epsilon` before `zeta` (both order 5),
		// though the TOML declares `zeta` first, so a stable sort without the tiebreak fails.
		let epsilon = out.find("| `epsilon` |").expect("epsilon row");
		let zeta = out.find("| `zeta` |").expect("zeta row");
		assert!(epsilon < zeta, "equal-order steps sort by slug (epsilon before zeta): {out}");
		// The skipped / optional / deferred Status-line buckets are exercised.
		assert!(
			out.contains("1 skipped") && out.contains("1 optional") && out.contains("1 deferred"),
			"{out}"
		);
	}

	#[test]
	fn provenance_note_renders_only_the_present_sub_lists_in_order() {
		// Only the non-empty sub-lists render, in the fixed decisions -> findings ->
		// commits order. A decisions-only provenance yields exactly `why: decisions ...`
		// with no stray `findings`/`commits` label (the live exemplars exercise this case).
		let decisions_only = Provenance {
			decisions: vec!["Q-53".to_string()],
			findings: Vec::new(),
			commits: Vec::new(),
		};
		assert_eq!(provenance_note(&decisions_only), "why: decisions Q-53");
		// Findings + commits, no decisions: the decisions label is absent and the order
		// is preserved.
		let findings_and_commits = Provenance {
			decisions: Vec::new(),
			findings: vec!["notes/x.md".to_string()],
			commits: vec!["abc1234".to_string(), "def5678".to_string()],
		};
		assert_eq!(
			provenance_note(&findings_and_commits),
			"why: findings notes/x.md; commits abc1234, def5678"
		);
	}

	#[test]
	fn every_step_status_label_is_an_accepted_roadmap_status() {
		// Drift guard: the render label for each `StepStatus` must be a member of the
		// `ROADMAP_STATUSES` vocabulary constant, so the TOML enum and the Markdown
		// vocabulary cannot diverge (Principle 16, one source of truth).
		for status in StepStatus::ALL {
			assert!(
				ROADMAP_STATUSES.contains(&status.label()),
				"StepStatus label `{}` is not in ROADMAP_STATUSES",
				status.label()
			);
		}
	}
}
