//! The advisory `agent-scaffold next` projection (workflow-driver Stage 1).
//!
//! `next` is read-only and stateless: for the single active review loop it
//! recomputes the loop state from the durable files (the plan source, the round
//! log, and the ledger's `## RESUME STATE` block), reports the state plus the
//! valid transitions, and emits ONE filled instruction prompt for the next role to
//! run. It writes nothing and creates no worktree or container; the isolation tier
//! is echoed, not resolved.
//!
//! The forward projection here and the backward `validate --workflow` (W3) check keep
//! their converged-vs-not verdicts in agreement on the same records, in two respects.
//! First, both run the SAME convergence arithmetic: both call
//! `workflow::peak_consecutive_clean` over the same records grouped by the same
//! `round_increment_id`/`round_step_slug` join accessors. Second, both treat a
//! within-increment `risk_class` inconsistency as a non-convergence: W3 reports it as a
//! shortfall and `continue`s, and `next` reports the `risk-class-conflict` state, never
//! `converged`. So a step that `next` reports `converged` is exactly a step W3 finds no
//! shortfall on (the differential property, pinned by `next_agrees_with_w3` below). This
//! is decision B-a: one streak helper, two directions, the same arithmetic and the same
//! data-fault handling.
//!
//! Stage 1 boundary (accepted): the mid-round `awaiting-triage` sub-state is not
//! derivable from the round log (a `round` record is written only after triage), so
//! the states here are derived from completed round records plus the step status; the
//! in-flight sub-state is carried only by the verbatim `## RESUME STATE` block.

use {
	crate::{
		isolation_policy::ISOLATION_POLICY_FRAGMENT,
		metrics::{
			RiskClass,
			Round,
			RoundOutcome,
		},
		plan::{
			self,
			source::{
				PlanToml,
				Principle,
				StepStatus as TomlStepStatus,
			},
		},
		workflow::{
			peak_consecutive_clean,
			round_increment_id,
			round_step_slug,
		},
		workflow_spec::WorkflowSpec,
	},
	serde::Serialize,
	std::{
		collections::BTreeMap,
		path::PathBuf,
	},
};

/// The resolve-the-tier note folded into the writer-isolation reminder when the tier
/// was not supplied on the CLI (`isolation_tier == "unknown"`). The tool never emits a
/// worktree path or a branch name; it points the orchestrator at the policy instead
/// (Principle 18, least authority). Unlike the always-on policy fragment (which fires at
/// every writer state regardless of tier), this note fires only when the tier is still
/// unresolved, so the orchestrator knows a resolution is owed before the spawn.
const TIER_RESOLVE_NOTE: &str =
	"Resolve the isolation tier per the AGENTS.md tier policy before spawning the writer.";

/// The phase -> Project-Principle-NAMES map: for a given `LoopState`, the names of the
/// Project Principles the driver should put in front of the actor at that phase. The
/// driver projects each mapped principle's actual name and text from the plan's
/// `[[principle]]` (by NAME, so it is immune to the AGENTS.md/plan renumbering the design
/// flags), and degrades to the originated imperatives alone for any name the plan does
/// not carry (never a dangling number). Names, not fixed numbers, because two disagreeing
/// numbered principle lists exist and the number is unstable. The mapping is list-valued
/// so a state can invoke several principles later; it is populated CONSERVATIVELY, only
/// where a principle genuinely fits the phase, and most states map to nothing (an empty
/// slice) rather than being forced a fit that would only add noise.
fn phase_principle_names(state: LoopState) -> &'static [&'static str] {
	match state {
		// The planner must validate the approach and raise the open questions with
		// evidence before implementing (matches its base reminder to give a recommendation
		// with reasoning and confirm intent).
		LoopState::ReadyToPlan => &["Ground decisions in evidence"],
		// The human's cap-reached decision must itself be grounded in evidence.
		LoopState::Escalate => &["Ground decisions in evidence"],
		LoopState::Blocked
		| LoopState::AwaitingFirstReview
		| LoopState::AwaitingFixes
		| LoopState::AwaitingReviewers
		| LoopState::Converged
		| LoopState::RiskClassConflict
		| LoopState::Done => &[],
	}
}

/// The whole `next` projection, serialised by `--json`. Every derived part is optional
/// so a missing plan or log yields a partial projection rather than a failure (mirrors
/// `status`'s `Projection`); nothing here is a source of truth.
#[derive(Debug, Serialize)]
pub(crate) struct NextProjection {
	/// The task slug (the `<task>` in `<task>.plan.toml` / `<task>.ledger.md`),
	/// derived from the plan source filename.
	pub(crate) task: String,
	/// The plan source path echoed verbatim (relative, never canonicalised), so the
	/// output is identical on any machine.
	pub(crate) source: String,
	/// The round-log summary, present only when the metrics log was readable.
	pub(crate) metrics: Option<MetricsSummary>,
	/// The single active review loop, or `None` when there is nothing to act on (all
	/// steps complete, every pending step blocked, or no plan source).
	pub(crate) active_loop: Option<ActiveLoop>,
	/// The ledger's `## RESUME STATE` block, extracted verbatim, or `None` when the
	/// ledger is absent or carries no such section.
	pub(crate) resume_state: Option<String>,
	/// Why there is no active loop, for the human renderer. Not serialised (the JSON
	/// contract is exactly the fields above); recomputed each call, never stored.
	#[serde(skip)]
	pub(crate) no_active_loop_reason: Option<String>,
}

/// The round-log half of the projection: a record count, matching `status`.
#[derive(Debug, Serialize)]
pub(crate) struct MetricsSummary {
	/// The number of records (non-blank lines) in the metrics log.
	pub(crate) records: usize,
}

/// The single active review loop: the step (and increment) under review, its derived
/// state, the convergence evidence, and the one filled instruction for the next role.
#[derive(Debug, Serialize)]
pub(crate) struct ActiveLoop {
	/// The active step slug.
	pub(crate) step: String,
	/// The active increment id, when the loop has round records to key it off; `None`
	/// before the first round (`awaiting-first-review`) or for a not-yet-started step.
	pub(crate) increment: Option<String>,
	/// The step's lifecycle status label (`in progress`, `not started`, ...), distinct
	/// from `state` (the review-loop sub-state).
	pub(crate) phase: String,
	/// The derived review-loop state (the transition-table row).
	pub(crate) state: LoopState,
	/// The convergence risk class the required streak is read against, from the round
	/// records; `None` before the first round.
	pub(crate) risk_class: Option<RiskClass>,
	/// The peak consecutive-clean streak reached (the convergence-relevant value W3
	/// checks), `0` before the first round.
	pub(crate) consecutive_clean: u64,
	/// The streak the class requires to converge; `None` before the first round.
	pub(crate) required_streak: Option<u64>,
	/// The count of the active increment's round records.
	pub(crate) total_rounds: u64,
	/// The advisory total-round cap (from the workflow spec); forward guidance, not
	/// enforced (consistent with advisory mode).
	pub(crate) round_cap: u64,
	/// The transitions valid from this state (the next-action vocabulary).
	pub(crate) valid_transitions: Vec<String>,
	/// The isolation tier echoed from the CLI (`worktree`/`container`/`file-safety`),
	/// or `unknown`.
	pub(crate) isolation_tier: String,
	/// The one filled instruction for the role that acts next.
	pub(crate) next_instruction: Instruction,
}

/// One filled instruction prompt: which role runs next, its prompt file, the
/// convention-derived context slots, the phase-keyed principle reminders, and a
/// one-line human summary. The slots are filled from EXISTING conventions (the
/// `.agents/prompts/<role>.md` and `docs/plans/<task>...` path shapes), not from a
/// spec extension; the reminders are control pointers, never manufactured verdicts.
#[derive(Debug, Serialize)]
pub(crate) struct Instruction {
	/// The role that acts next (`planner`, `reviewer`, `implementer`, `orchestrator`).
	pub(crate) role: String,
	/// The role's prompt file, by the `.agents/prompts/<role>.md` convention.
	pub(crate) prompt_path: String,
	/// The convention-derived context slots (a `BTreeMap` so the order is deterministic).
	pub(crate) context: BTreeMap<String, String>,
	/// The phase-keyed reminders: originated workflow-phase guidance (no numeric
	/// citation), plus the projected Project Principle text for the current state's
	/// mapped principle(s), and, at writer states, the generated isolation-policy
	/// fragment. Fixed order.
	pub(crate) principle_reminders: Vec<String>,
	/// A one-line human summary of the filled instruction.
	pub(crate) filled_prompt_summary: String,
}

/// The review-loop state, one variant per transition-table row. Kebab-cased on the
/// wire (`ready-to-plan`, `awaiting-first-review`, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum LoopState {
	/// A ready, unblocked not-started/next step: spawn a planner.
	ReadyToPlan,
	/// A pending step whose blockers are not all complete: resolve them first.
	Blocked,
	/// An in-progress step with no round records yet: spawn the first reviewer.
	AwaitingFirstReview,
	/// The last round produced a new valid finding and the streak has not converged:
	/// address the findings.
	AwaitingFixes,
	/// The last round was clean but the streak has not yet reached the required count:
	/// spawn a fresh reviewer.
	AwaitingReviewers,
	/// The peak streak reached the required count: mark the step complete.
	Converged,
	/// The round cap was reached without converging: escalate to a human.
	Escalate,
	/// The active increment's round records carry inconsistent `risk_class` values, so
	/// the required streak is undefined: a data-integrity fault owed to a human before the
	/// loop can converge. This is the SAME condition W3 flags and `continue`s on, so
	/// reporting it as a non-converged state (never `converged`) keeps the forward and
	/// backward verdicts in agreement (Principle 12 fail-loud, Principle 15 explicit
	/// failure).
	RiskClassConflict,
	/// A complete step: nothing to do, move on. In Stage 1 an all-complete plan is
	/// reported as "no active loop" (the build plan's selection rule 3), so the
	/// projection path never constructs this variant; it is a transition-table row
	/// pinned by `done_row_metadata`. `allow` (not `expect`) because it is constructed
	/// under `cfg(test)` but not in the release build (a cfg-split, test-only use).
	#[cfg_attr(not(test), allow(dead_code))]
	Done,
}

impl LoopState {
	/// The kebab-case label, identical to the serialised spelling, for the human text.
	pub(crate) fn label(self) -> &'static str {
		match self {
			LoopState::ReadyToPlan => "ready-to-plan",
			LoopState::Blocked => "blocked",
			LoopState::AwaitingFirstReview => "awaiting-first-review",
			LoopState::AwaitingFixes => "awaiting-fixes",
			LoopState::AwaitingReviewers => "awaiting-reviewers",
			LoopState::Converged => "converged",
			LoopState::Escalate => "escalate",
			LoopState::RiskClassConflict => "risk-class-conflict",
			LoopState::Done => "done",
		}
	}

	/// The role that acts next in this state.
	pub(crate) fn role(self) -> &'static str {
		match self {
			LoopState::ReadyToPlan => "planner",
			LoopState::AwaitingFirstReview | LoopState::AwaitingReviewers => "reviewer",
			LoopState::AwaitingFixes => "implementer",
			LoopState::Blocked
			| LoopState::Converged
			| LoopState::Escalate
			| LoopState::RiskClassConflict
			| LoopState::Done => "orchestrator",
		}
	}

	/// The transitions valid from this state (the transition table's middle column).
	pub(crate) fn valid_transitions(self) -> Vec<String> {
		let raw: &[&str] = match self {
			LoopState::ReadyToPlan => &["start-plan"],
			LoopState::Blocked => &[],
			LoopState::AwaitingFirstReview => &["record-round"],
			LoopState::AwaitingFixes => &["address-findings"],
			LoopState::AwaitingReviewers =>
				&["record-round-clean", "record-round-new-valid", "escalate"],
			LoopState::Converged => &["mark-step-complete"],
			LoopState::Escalate => &["escalate-to-human"],
			LoopState::RiskClassConflict => &["fix-risk-class-records"],
			LoopState::Done => &[],
		};
		raw.iter().map(|transition| (*transition).to_string()).collect()
	}

	/// The recommended next action (the transition table's rightmost column).
	pub(crate) fn next_action(self) -> &'static str {
		match self {
			LoopState::ReadyToPlan => "spawn a planner to draft the step plan",
			LoopState::Blocked => "resolve the unmet blockers before starting (no spawn)",
			LoopState::AwaitingFirstReview => "spawn a reviewer for the first review round",
			LoopState::AwaitingFixes =>
				"spawn an implementer to address the findings, then a fresh reviewer round",
			LoopState::AwaitingReviewers => "spawn a fresh reviewer (diversify the model)",
			LoopState::Converged => "mark the step complete, re-render, and commit",
			LoopState::Escalate => "escalate to a human and present the human-input contract",
			LoopState::RiskClassConflict =>
				"correct the inconsistent risk_class round records before the loop can converge (no spawn)",
			LoopState::Done => "move to the next step",
		}
	}

	/// Whether this state spawns an ISOLATED AGENT: a role that runs in its own isolation
	/// tier because it performs a write, which is what makes the isolation-policy fragment,
	/// the resolved-tier echo, and the resolve-the-tier note relevant. FOUR states qualify:
	/// `ReadyToPlan` (spawns the planner) and `AwaitingFixes` (spawns the implementer), which
	/// both author reviewed product content, and `AwaitingFirstReview` and `AwaitingReviewers`
	/// (spawn REVIEWERS), which author a findings file, and even a findings file is a write.
	/// Q-62 (decided 2026-07-23, option (a)) widened the driver so the always-on isolation
	/// reminder fires at all four spawn states, not only the writer states: under the
	/// uniform-isolation rule (`pack/AGENTS.md`, Writer isolation) reviewers run isolated too,
	/// so the reminder, its tier echo, and its resolve-note now attach wherever an agent is
	/// spawned. The remaining states (marking complete, escalating, resolving blockers) spawn
	/// no agent at all.
	fn spawns_isolated_agent(self) -> bool {
		matches!(
			self,
			LoopState::ReadyToPlan
				| LoopState::AwaitingFixes
				| LoopState::AwaitingFirstReview
				| LoopState::AwaitingReviewers
		)
	}

	/// The phase-keyed reminders for this state: the driver's own ORIGINATED guidance for
	/// the phase, stated as plain imperatives. These no longer carry a "Principle N:"
	/// numeric citation (design decision D-a Option 2): the number was doubly unstable (it
	/// quoted a generated AGENTS.md list that renumbers on reorder, AND it referenced the
	/// list the human-input contract does NOT name, the plan's `[[principle]]`), so it was
	/// a mislabelled paraphrase dressed as a quotation. Stripped of the number, each is
	/// honest originated guidance with no canonical source and nothing to drift from.
	/// Where a Project Principle must actually be put in front of the actor the driver
	/// PROJECTS its real name/text from the plan instead (see the phase -> principle-names
	/// map `phase_principle_names` and `projected_principle_reminders`), rather than citing
	/// a number here. The `escalate` state, and ONLY it, carries the human-input-contract
	/// reminder.
	fn base_reminders(self) -> &'static [&'static str] {
		match self {
			LoopState::ReadyToPlan => &[
				"Raise and resolve the open questions before implementing.",
				"Give a recommendation with reasoning; confirm intent before forging ahead.",
			],
			LoopState::Blocked =>
				&["Do not proceed past an unmet dependency; resolve the named blockers first."],
			LoopState::AwaitingFirstReview => &[
				"The reviewer is independent; a writer never reviews its own work.",
				"Cite the file and line for each finding rather than asserting from memory.",
			],
			LoopState::AwaitingFixes => &[
				"Keep the fix small and reviewable.",
				"Address the findings only; flag any other change rather than folding it in.",
			],
			LoopState::AwaitingReviewers => &[
				"Staff a fresh, independent reviewer and diversify the model.",
				"Cite the file and line for each finding.",
			],
			LoopState::Converged => &[
				"Verify by running the tests and checks before marking the step complete.",
				"Edit the step status in the plan source and re-render; never hand-edit the generated view.",
			],
			LoopState::Escalate => &[
				"Human-input contract (see AGENTS.md): present the options, their trade-offs, an explicit recommendation, and the reasoning judged against the Project Principles, scaled to the stakes. The human decides; advise, never decide for them.",
				"Escalate loudly at the round cap rather than continuing the loop.",
			],
			LoopState::RiskClassConflict => &[
				"Fail loud on the data fault; do not derive a convergence verdict from an ambiguous risk class.",
				"The increment's round records disagree on risk_class; correct them so a single required streak is defined before the loop can converge.",
			],
			LoopState::Done =>
				&["The step is complete; move to the next step rather than expanding scope here."],
		}
	}
}

/// The projected Project Principle reminders for a state: look up the state's mapped
/// principle NAMES in `phase_principle_names`, find each in the plan's `[[principle]]` by
/// NAME, and emit its real name and text (with the plan's own number as a locator), so
/// the actor sees the actual principle to ground its decision in, self-contained at the
/// point of action and drift-free (it is projected live from the same plan.toml the render
/// and the contract read). Looked up by NAME, not by a fixed number, because two
/// disagreeing numbered lists exist and the number renumbers on reorder. A mapped name the
/// plan does not carry is skipped (D-e Option 1: degrade to the originated imperatives
/// alone, never a dangling number); the Markdown substrate, which parses no principles,
/// always degrades to an empty Vec, as does any state the map does not populate.
fn projected_principle_reminders(
	state: LoopState,
	principles: &[Principle],
) -> Vec<String> {
	phase_principle_names(state)
		.iter()
		.filter_map(|name| principles.iter().find(|principle| principle.name == *name))
		.map(|principle| {
			format!(
				"Ground the recommendation in the Project Principle \"{}\" (plan principle {}): {}",
				principle.name, principle.n, principle.text
			)
		})
		.collect()
}

/// A step's lifecycle phase, normalised so the Markdown parametric `blocked on <slug>`
/// status and the TOML `blocked_by` list resolve to the SAME `(NotStarted, blockers)`
/// pair, letting the two substrates give an identical verdict (the parity property).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StepPhase {
	NotStarted,
	InProgress,
	Complete,
	Skipped,
	Next,
	Optional,
	Deferred,
}

impl StepPhase {
	/// The human-readable status label (the space form the Roadmap uses), for the
	/// `phase` field.
	fn label(self) -> &'static str {
		match self {
			StepPhase::NotStarted => "not started",
			StepPhase::InProgress => "in progress",
			StepPhase::Complete => "complete",
			StepPhase::Skipped => "skipped",
			StepPhase::Next => "next",
			StepPhase::Optional => "optional",
			StepPhase::Deferred => "deferred",
		}
	}

	/// Whether a step in this phase is a candidate for the ready frontier (a not-started
	/// or queued-next step that a planner could start).
	fn is_pending(self) -> bool {
		matches!(self, StepPhase::NotStarted | StepPhase::Next)
	}

	/// Whether a step in this phase is terminal (nothing further is owed on it), used to
	/// tell an all-complete plan from one with real remaining work.
	fn is_terminal(self) -> bool {
		matches!(
			self,
			StepPhase::Complete | StepPhase::Skipped | StepPhase::Optional | StepPhase::Deferred
		)
	}
}

/// The normalised view of one step the projection reads, built from either substrate.
/// Holds only what the active-loop selection needs: the slug, the order, the phase, and
/// the resolved blockers.
///
/// The declared `[[step.increment]].risk_class` is deliberately NOT carried: the
/// convergence class comes from the round records (`records[0].risk_class`), the SAME
/// value W3 reads, so the forward and backward verdicts cannot diverge on the threshold
/// (the differential property), and the Markdown substrate (which declares no increments)
/// produces an identical projection to the TOML one (the parity property). This departs
/// from the build plan's "declared class when present" wording in favour of those two
/// structural guarantees.
#[derive(Debug, Clone)]
pub(crate) struct StepInfo {
	slug: String,
	order: u64,
	phase: StepPhase,
	blocked_by: Vec<String>,
}

/// Normalise the TOML steps into the projection's `StepInfo` view: the order and the
/// typed `blocked_by` carry over directly, and the typed `StepStatus` maps to the
/// matching phase.
pub(crate) fn steps_from_toml(plan: &PlanToml) -> Vec<StepInfo> {
	plan.steps
		.iter()
		.map(|step| StepInfo {
			slug: step.slug.clone(),
			order: step.order,
			phase: phase_from_toml_status(step.status),
			blocked_by: step.blocked_by.clone(),
		})
		.collect()
}

/// Normalise the Markdown Roadmap steps into the projection's `StepInfo` view. The order
/// is the table position (the Markdown Roadmap has no explicit order field); the
/// parametric `blocked on <slug>` status resolves to `NotStarted` plus that blocker, so
/// a blocked Markdown step matches the equivalent TOML `not-started` + `blocked_by` step
/// (the parity property). The Markdown substrate declares no increments.
pub(crate) fn steps_from_markdown(steps: &[plan::Step]) -> Vec<StepInfo> {
	steps
		.iter()
		.enumerate()
		.map(|(index, step)| {
			let (phase, blocked_by) = phase_from_markdown_status(&step.status);
			StepInfo {
				slug: step.slug.clone(),
				order: index as u64,
				phase,
				blocked_by,
			}
		})
		.collect()
}

/// Map a typed TOML `StepStatus` to the projection's phase. The TOML schema retired the
/// `blocked` status (blockedness is the typed `blocked_by` list), so there is no blocked
/// arm here.
fn phase_from_toml_status(status: TomlStepStatus) -> StepPhase {
	match status {
		TomlStepStatus::NotStarted => StepPhase::NotStarted,
		TomlStepStatus::InProgress => StepPhase::InProgress,
		TomlStepStatus::Complete => StepPhase::Complete,
		TomlStepStatus::Skipped => StepPhase::Skipped,
		TomlStepStatus::Next => StepPhase::Next,
		TomlStepStatus::Optional => StepPhase::Optional,
		TomlStepStatus::Deferred => StepPhase::Deferred,
	}
}

/// Map a Markdown Roadmap status cell to a phase plus any blocker it names. A
/// `blocked on <slug>` cell resolves to `NotStarted` with that slug as a blocker (with
/// any surrounding backticks stripped), so it matches the equivalent TOML step. An
/// unrecognised status is treated as `NotStarted` (a best-effort projection, consistent
/// with the Markdown parsers dropping what they cannot read).
fn phase_from_markdown_status(status: &str) -> (StepPhase, Vec<String>) {
	if let Some(rest) = status.strip_prefix("blocked on ") {
		let slug = rest.trim().trim_matches('`').to_string();
		return (StepPhase::NotStarted, vec![slug]);
	}
	let phase = match status.trim() {
		"in progress" => StepPhase::InProgress,
		"complete" => StepPhase::Complete,
		"skipped" => StepPhase::Skipped,
		"next" => StepPhase::Next,
		"optional" => StepPhase::Optional,
		"deferred" => StepPhase::Deferred,
		// "not started" and anything unrecognised.
		_ => StepPhase::NotStarted,
	};
	(phase, Vec::new())
}

/// The inputs `project` reads, gathered by the caller from the durable files. Borrowing
/// the steps/rounds/spec keeps `project` allocation-light and the caller the single
/// owner of the parsed data.
pub(crate) struct NextInputs<'a> {
	pub(crate) task: String,
	pub(crate) source: String,
	pub(crate) steps: &'a [StepInfo],
	pub(crate) rounds: &'a [Round],
	pub(crate) spec: &'a WorkflowSpec,
	pub(crate) metrics_records: Option<usize>,
	pub(crate) ledger_path: String,
	pub(crate) resume_state: Option<String>,
	pub(crate) isolation_tier: String,
	/// The plan's parsed `[[principle]]` list, projected by NAME into the escalate
	/// human-input-contract reminder. Empty for a Markdown-source projection (the Markdown
	/// substrate carries no principles), which degrades the projection gracefully.
	pub(crate) principles: &'a [Principle],
}

/// The instruction-assembly context threaded through the builders: the path/tier facts
/// the filled prompt echoes.
struct LoopContext<'a> {
	task: &'a str,
	ledger_path: &'a str,
	isolation_tier: &'a str,
	/// The plan's principles, threaded to the escalate-state reminder projection.
	principles: &'a [Principle],
}

/// The loop facts the instruction and the summary read, so the many scalars are passed
/// as one value rather than a long parameter list.
struct LoopFacts {
	step: String,
	increment: Option<String>,
	peak: u64,
	required: Option<u64>,
	total_rounds: u64,
	round_cap: u64,
	blockers: Vec<String>,
}

/// Build the whole `next` projection from the gathered inputs.
pub(crate) fn project(inputs: NextInputs) -> NextProjection {
	let context = LoopContext {
		task: &inputs.task,
		ledger_path: &inputs.ledger_path,
		isolation_tier: &inputs.isolation_tier,
		principles: inputs.principles,
	};
	let active_loop = select_active_loop(inputs.steps, inputs.rounds, inputs.spec, &context);
	let no_active_loop_reason =
		if active_loop.is_none() { Some(no_loop_reason(inputs.steps)) } else { None };
	NextProjection {
		task: inputs.task,
		source: inputs.source,
		metrics: inputs.metrics_records.map(|records| MetricsSummary { records }),
		active_loop,
		resume_state: inputs.resume_state,
		no_active_loop_reason,
	}
}

/// Select the single active loop, deterministically and order-keyed:
/// 1. the lowest-order in-progress step, if any (its state derives from its rounds);
/// 2. else the lowest-order pending step whose blockers are all complete (ready-to-plan);
/// 3. else, if any pending step exists (all are blocked), the lowest-order one (blocked);
/// 4. else `None` (all steps terminal, or no plan source).
fn select_active_loop(
	steps: &[StepInfo],
	rounds: &[Round],
	spec: &WorkflowSpec,
	context: &LoopContext,
) -> Option<ActiveLoop> {
	if let Some(step) =
		steps.iter().filter(|step| step.phase == StepPhase::InProgress).min_by_key(|step| step.order)
	{
		return Some(build_in_progress_loop(step, rounds, spec, context));
	}
	if let Some(step) = steps
		.iter()
		.filter(|step| step.phase.is_pending() && blockers_met(step, steps))
		.min_by_key(|step| step.order)
	{
		return Some(build_pending_loop(step, LoopState::ReadyToPlan, Vec::new(), spec, context));
	}
	if let Some(step) =
		steps.iter().filter(|step| step.phase.is_pending()).min_by_key(|step| step.order)
	{
		let blockers = unmet_blockers(step, steps);
		return Some(build_pending_loop(step, LoopState::Blocked, blockers, spec, context));
	}
	None
}

/// Whether every one of a step's blockers names a `complete` step. An unresolved blocker
/// slug (naming no step, or a step in any non-complete phase) counts as unmet.
fn blockers_met(
	step: &StepInfo,
	steps: &[StepInfo],
) -> bool {
	step.blocked_by.iter().all(|blocker| is_complete(blocker, steps))
}

/// The subset of a step's blockers that are not yet complete, for the blocked report.
fn unmet_blockers(
	step: &StepInfo,
	steps: &[StepInfo],
) -> Vec<String> {
	step.blocked_by.iter().filter(|blocker| !is_complete(blocker, steps)).cloned().collect()
}

/// Whether `slug` names a `complete` step in the plan.
fn is_complete(
	slug: &str,
	steps: &[StepInfo],
) -> bool {
	steps.iter().any(|step| step.slug == slug && step.phase == StepPhase::Complete)
}

/// Build the active loop for a not-started/next step (either `ready-to-plan` or
/// `blocked`): no rounds, no increment, no streak.
fn build_pending_loop(
	step: &StepInfo,
	state: LoopState,
	blockers: Vec<String>,
	spec: &WorkflowSpec,
	context: &LoopContext,
) -> ActiveLoop {
	let facts = LoopFacts {
		step: step.slug.clone(),
		increment: None,
		peak: 0,
		required: None,
		total_rounds: 0,
		round_cap: spec.round_cap(),
		blockers,
	};
	assemble(step.phase, state, None, 0, facts, context)
}

/// Build the active loop for an in-progress step: join its rounds, group them by
/// increment, pick the active increment, and derive the state from that increment's
/// convergence evidence.
fn build_in_progress_loop(
	step: &StepInfo,
	rounds: &[Round],
	spec: &WorkflowSpec,
	context: &LoopContext,
) -> ActiveLoop {
	let matching: Vec<&Round> =
		rounds.iter().filter(|round| round_step_slug(round) == step.slug).collect();
	// Group the step's rounds by increment, exactly as W3 does, so the same records back
	// the same peak. `BTreeMap` keeps the selection deterministic.
	let mut increments: BTreeMap<&str, Vec<&Round>> = BTreeMap::new();
	for round in &matching {
		increments.entry(round_increment_id(round)).or_default().push(round);
	}

	let Some(active) = select_active_increment(&increments, &matching, spec) else {
		// No round records yet: the first review round is owed.
		let facts = LoopFacts {
			step: step.slug.clone(),
			increment: None,
			peak: 0,
			required: None,
			total_rounds: 0,
			round_cap: spec.round_cap(),
			blockers: Vec::new(),
		};
		return assemble(step.phase, LoopState::AwaitingFirstReview, None, 0, facts, context);
	};

	let records = &increments[active];
	// The risk class the convergence bar is read against is the round records' class,
	// the SAME value W3 uses (`records[0].risk_class`), so the forward and backward
	// verdicts cannot diverge on the threshold (the differential property). The declared
	// `[[step.increment]].risk_class` is not substituted here for that reason; when it
	// disagrees with the logged class that is a data fault W3 owns, not one `next` papers
	// over.
	let class = records[0].risk_class;
	let required = spec.required_streak(class);
	let peak = peak_consecutive_clean(records);
	let total_rounds = records.len() as u64;
	let round_cap = spec.round_cap();
	// A within-increment `risk_class` inconsistency leaves the required streak undefined.
	// W3 flags exactly this and `continue`s (never a no-shortfall increment), so `next`
	// must not read `records[0].risk_class` and possibly report `converged` here: it
	// reports the non-converged `RiskClassConflict` state instead, keeping the forward
	// `converged` verdict and the backward `no-shortfall` verdict in agreement.
	let state = if has_risk_class_conflict(records) {
		LoopState::RiskClassConflict
	} else {
		derive_in_progress_state(records, peak, required, total_rounds, round_cap)
	};
	let facts = LoopFacts {
		step: step.slug.clone(),
		increment: Some(active.to_string()),
		peak,
		required: Some(required),
		total_rounds,
		round_cap,
		blockers: Vec::new(),
	};
	assemble(step.phase, state, Some(class), peak, facts, context)
}

/// Whether an increment's round records disagree on `risk_class`, leaving its required
/// streak undefined. This is the SAME predicate the line-625 guard and W3
/// (`workflow::w3_problems`) apply, single-sourced here so the forward `next` verdict and
/// the backward W3 verdict cannot drift on the data-fault condition. `records` is non-empty.
fn has_risk_class_conflict(records: &[&Round]) -> bool {
	let class = records[0].risk_class;
	records.iter().any(|round| round.risk_class != class)
}

/// Pick the active increment among an in-progress step's increments: the first
/// conflicted-or-unconverged one, in id order; else, when every increment has converged,
/// the increment of the latest round record. A `risk_class`-conflicted increment is never
/// treated as converged (W3 flags it as a shortfall), so it is returned as active ahead of
/// letting a later clean increment read as converged; the `RiskClassConflict` guard in
/// `build_in_progress_loop` then fires on it. `None` when the step has no round records at
/// all. Returns the increment key borrowed from the map.
fn select_active_increment<'a>(
	increments: &BTreeMap<&'a str, Vec<&'a Round>>,
	matching: &[&'a Round],
	spec: &WorkflowSpec,
) -> Option<&'a str> {
	if increments.is_empty() {
		return None;
	}
	// `BTreeMap` iterates in id order, so the first conflicted-or-unconverged increment is
	// chosen deterministically.
	for (increment, records) in increments {
		if has_risk_class_conflict(records) {
			return Some(increment);
		}
		let required = spec.required_streak(records[0].risk_class);
		if peak_consecutive_clean(records) < required {
			return Some(increment);
		}
	}
	// Every increment converged: report the one the latest round record covers.
	matching.iter().max_by_key(|round| round.line).map(|round| round_increment_id(round))
}

/// Derive the state of an in-progress step's active increment from its round evidence,
/// evaluating `converged` BEFORE `escalate` (the pack/AGENTS.md order: a round that both
/// converges and reaches the cap converges). Below the cap, the last round's outcome
/// splits `awaiting-fixes` (a new valid finding) from `awaiting-reviewers` (clean but not
/// yet converged). `records` is non-empty and in file (chronological) order.
fn derive_in_progress_state(
	records: &[&Round],
	peak: u64,
	required: u64,
	total_rounds: u64,
	round_cap: u64,
) -> LoopState {
	if peak >= required {
		return LoopState::Converged;
	}
	if total_rounds >= round_cap {
		return LoopState::Escalate;
	}
	let last = records.last().expect("an in-progress increment has at least one round record");
	match last.outcome {
		RoundOutcome::NewValid => LoopState::AwaitingFixes,
		RoundOutcome::Clean => LoopState::AwaitingReviewers,
	}
}

/// Assemble the `ActiveLoop` from the derived state and facts.
fn assemble(
	phase: StepPhase,
	state: LoopState,
	risk_class: Option<RiskClass>,
	consecutive_clean: u64,
	facts: LoopFacts,
	context: &LoopContext,
) -> ActiveLoop {
	let next_instruction = build_instruction(state, &facts, context);
	ActiveLoop {
		step: facts.step,
		increment: facts.increment,
		phase: phase.label().to_string(),
		state,
		risk_class,
		consecutive_clean,
		required_streak: facts.required,
		total_rounds: facts.total_rounds,
		round_cap: facts.round_cap,
		valid_transitions: state.valid_transitions(),
		isolation_tier: context.isolation_tier.to_string(),
		next_instruction,
	}
}

/// Build the filled instruction for a state: the role and its prompt path, the
/// convention-derived context slots, the reminders, and the one-line summary.
///
/// The reminders are assembled in three parts, in a fixed order: (1) the state's
/// originated base reminders; (2) the projected Project Principle text for the current
/// state's mapped principle(s) per `phase_principle_names` (when the plan carries them;
/// today ReadyToPlan and Escalate both map to "Ground decisions in evidence"); (3) at an
/// AGENT-spawn state (planner, implementer, or reviewer/triager),
/// the always-on agent-isolation reminder, unconditionally emitting the generated
/// `ISOLATION_POLICY_FRAGMENT` (the single source shared with AGENTS.md, so the policy is
/// inline at the point of action and cannot drift) plus the resolved tier, and, when the
/// tier is still `unknown`, the resolve-the-tier note. This replaces the old
/// tier-only-when-unknown pointer: the policy fires at every agent spawn regardless of
/// tier (D-b, the motivating case; Q-62 widened the scope from writer spawns to all four
/// agent-spawn states).
fn build_instruction(
	state: LoopState,
	facts: &LoopFacts,
	context: &LoopContext,
) -> Instruction {
	let role = state.role();
	let mut principle_reminders: Vec<String> =
		state.base_reminders().iter().map(|reminder| (*reminder).to_string()).collect();
	// The current state's mapped Project Principle(s) projected by name from the plan
	// (not a number); a mapped principle absent from the plan is skipped, and a state the
	// phase map does not populate contributes nothing (no dangling reference).
	principle_reminders.extend(projected_principle_reminders(state, context.principles));
	// The always-on agent-isolation reminder: the generated policy fragment (drift-free,
	// shared with AGENTS.md) plus the resolved tier, at every agent-spawn state (Q-62 widened
	// this from the writer states to include the reviewer/triager-spawn states too).
	if state.spawns_isolated_agent() {
		let lead = if context.isolation_tier == "unknown" {
			format!("Agent isolation (tier not yet resolved). {TIER_RESOLVE_NOTE}")
		} else {
			format!("Agent isolation (resolved tier: {}).", context.isolation_tier)
		};
		principle_reminders.push(format!("{lead} {ISOLATION_POLICY_FRAGMENT}"));
	}
	Instruction {
		role: role.to_string(),
		prompt_path: format!(".agents/prompts/{role}.md"),
		context: build_context(state, facts, context),
		principle_reminders,
		filled_prompt_summary: filled_prompt_summary(state, facts),
	}
}

/// The convention-derived context slots for a state. Always the ledger path and the
/// isolation tier; plus the review findings-file templates for the review states, the
/// triage findings path for the fix state, and the unmet blockers for the blocked state.
/// The concrete reviewer `<disambiguator>` is left as a template token for the
/// orchestrator to assign (writers never collide on findings-file names). The
/// `artifact`/`diff` slots are NOT populated: no structured marker exists in the ledger
/// yet (deferred to Stage 2 per decision A-b), so the orchestrator reads the verbatim
/// `resume_state` instead of a heuristically parsed slot (Principle 12, fail loud).
fn build_context(
	state: LoopState,
	facts: &LoopFacts,
	context: &LoopContext,
) -> BTreeMap<String, String> {
	let mut slots = BTreeMap::new();
	slots.insert("ledger".to_string(), context.ledger_path.to_string());
	slots.insert("isolation_tier".to_string(), context.isolation_tier.to_string());
	let review_findings = format!(
		"docs/plans/{}.reviews/{}-reviewer-<disambiguator>.md",
		context.task, facts.step
	);
	let triage_findings = format!("docs/plans/{}.reviews/{}-triage.md", context.task, facts.step);
	match state {
		LoopState::AwaitingFirstReview | LoopState::AwaitingReviewers => {
			slots.insert("review_findings".to_string(), review_findings);
			slots.insert("triage_findings".to_string(), triage_findings);
		}
		LoopState::AwaitingFixes => {
			slots.insert("triage_findings".to_string(), triage_findings);
		}
		LoopState::Blocked => {
			slots.insert("blocked_by".to_string(), facts.blockers.join(", "));
		}
		LoopState::ReadyToPlan
		| LoopState::Converged
		| LoopState::Escalate
		| LoopState::RiskClassConflict
		| LoopState::Done => {}
	}
	slots
}

/// A one-line human summary of the filled instruction.
fn filled_prompt_summary(
	state: LoopState,
	facts: &LoopFacts,
) -> String {
	let increment = facts
		.increment
		.as_ref()
		.map(|increment| format!(" increment `{increment}`"))
		.unwrap_or_default();
	let required = facts.required.map_or_else(|| "?".to_string(), |required| required.to_string());
	match state {
		LoopState::ReadyToPlan => format!(
			"plan step `{}`: draft the plan, resolve the open questions, and state the Success Criteria.",
			facts.step
		),
		LoopState::Blocked => format!(
			"step `{}` is blocked on: {}. Resolve the blockers before starting.",
			facts.step,
			facts.blockers.join(", ")
		),
		LoopState::AwaitingFirstReview => format!(
			"first review round on step `{}`{increment}: independent reviewer, cite file and line.",
			facts.step
		),
		LoopState::AwaitingFixes => format!(
			"address the triaged findings on step `{}`{increment}, then request a fresh review round.",
			facts.step
		),
		LoopState::AwaitingReviewers => format!(
			"fresh review round on step `{}`{increment} (streak {}/{required}).",
			facts.step, facts.peak
		),
		LoopState::Converged => format!(
			"step `{}`{increment} converged (streak {}/{required}); mark the step complete, re-render, and commit.",
			facts.step, facts.peak
		),
		LoopState::Escalate => format!(
			"step `{}`{increment} reached the round cap ({}/{}) without converging; escalate to a human per the contract.",
			facts.step, facts.total_rounds, facts.round_cap
		),
		LoopState::RiskClassConflict => format!(
			"step `{}`{increment} logs inconsistent risk_class values; correct the round records so a single required streak is defined before the loop can converge.",
			facts.step
		),
		LoopState::Done => format!("step `{}` is complete; move to the next step.", facts.step),
	}
}

/// The reason there is no active loop, for the human renderer.
fn no_loop_reason(steps: &[StepInfo]) -> String {
	if steps.is_empty() {
		"no plan steps found".to_string()
	} else if steps.iter().all(|step| step.phase.is_terminal()) {
		"all steps complete".to_string()
	} else {
		"no in-progress or ready step".to_string()
	}
}

/// Extract the ledger's `## RESUME STATE` section VERBATIM: from its heading line
/// (which starts with `## RESUME STATE`, allowing a trailing parenthetical) through the
/// line before the next level-2 `## ` heading or the end of the document, with trailing
/// blank lines trimmed. `None` when the section is absent (Principle 15, absence is
/// explicit). Only a level-2 `## ` heading terminates the section, so a nested `### `
/// heading inside it does not; this mirrors `plan::section_lines`.
pub(crate) fn extract_resume_state(fragment: &str) -> Option<String> {
	let mut in_section = false;
	let mut block: Vec<&str> = Vec::new();
	for line in fragment.lines() {
		if in_section {
			if line.starts_with("## ") {
				break;
			}
			block.push(line);
		} else if line.starts_with("## RESUME STATE") {
			in_section = true;
			block.push(line);
		}
	}
	if in_section {
		Some(block.join("\n").trim_end().to_string())
	} else {
		None
	}
}

/// Derive the task slug from the plan source filename: the `<task>` in
/// `<task>.plan.toml`, `<task>.md`, or `<task>.toml`. Prefers `--source`, then `--plan`;
/// a `next` with neither plan source falls back to `task`.
pub(crate) fn derive_task(
	source: &Option<PathBuf>,
	plan: &Option<PathBuf>,
) -> String {
	source
		.as_ref()
		.or(plan.as_ref())
		.and_then(|path| path.file_name())
		.and_then(|name| name.to_str())
		.map_or_else(|| "task".to_string(), task_from_filename)
}

/// Strip the plan-source extension off a filename to recover the task slug.
fn task_from_filename(name: &str) -> String {
	name.strip_suffix(".plan.toml")
		.or_else(|| name.strip_suffix(".md"))
		.or_else(|| name.strip_suffix(".toml"))
		.unwrap_or(name)
		.to_string()
}

/// Render the projection as the deterministic human-readable text: a `task`/`source`/
/// `metrics` header, then either the `ACTIVE LOOP` block or a `no active review loop`
/// line. No trailing newline (the caller adds one); no timestamps; paths echoed verbatim.
pub(crate) fn render_human(projection: &NextProjection) -> String {
	let mut out = String::new();
	out.push_str(&format!("task: {}\n", projection.task));
	out.push_str(&format!("source: {}\n", projection.source));
	match &projection.metrics {
		Some(metrics) => out.push_str(&format!("metrics: {} records\n", metrics.records)),
		None => out.push_str("metrics: no log found\n"),
	}
	out.push('\n');
	match &projection.active_loop {
		None => {
			let reason = projection
				.no_active_loop_reason
				.as_deref()
				.unwrap_or("no in-progress or ready step");
			out.push_str(&format!("no active review loop ({reason})\n"));
		}
		Some(active) => render_active_loop(&mut out, active),
	}
	if let Some(resume) = &projection.resume_state {
		out.push('\n');
		out.push_str("RESUME STATE (verbatim from the ledger):\n");
		out.push_str(resume);
		out.push('\n');
	}
	out.trim_end_matches('\n').to_string()
}

/// Render the `ACTIVE LOOP` block into `out`.
fn render_active_loop(
	out: &mut String,
	active: &ActiveLoop,
) {
	let unit = match &active.increment {
		Some(increment) => format!("{} / {}", active.step, increment),
		None => active.step.clone(),
	};
	let transition = active.valid_transitions.first().map_or("-", String::as_str);
	let required =
		active.required_streak.map_or_else(|| "?".to_string(), |required| required.to_string());
	out.push_str("ACTIVE LOOP\n");
	out.push_str(&format!("  {unit}  {} -> {transition}\n", active.phase));
	out.push_str(&format!("  state: {}\n", active.state.label()));
	out.push_str(&format!("  streak: {}/{required}\n", active.consecutive_clean));
	out.push_str(&format!("  rounds: {}/{}\n", active.total_rounds, active.round_cap));
	out.push_str(&format!("  isolation: {}\n", active.isolation_tier));
	out.push_str(&format!("  next: {}\n", active.state.next_action()));
	out.push_str(&format!("  role: {}\n", active.next_instruction.role));
	out.push_str(&format!("  prompt: {}\n", active.next_instruction.prompt_path));
	out.push_str("  context:\n");
	for (key, value) in &active.next_instruction.context {
		out.push_str(&format!("    {key}: {value}\n"));
	}
	out.push_str("  reminders:\n");
	for reminder in &active.next_instruction.principle_reminders {
		out.push_str(&format!("    - {reminder}\n"));
	}
	out.push_str(&format!("  summary: {}\n", active.next_instruction.filled_prompt_summary));
}

#[cfg(test)]
mod tests {
	use {
		super::*,
		crate::metrics::{
			Round,
			RoundOutcome,
		},
	};

	/// Build a `round` record fixture with the fields the projection reads.
	fn round(
		line: usize,
		outcome: RoundOutcome,
		consecutive_clean: u64,
		risk_class: RiskClass,
		step: &str,
		increment: &str,
	) -> Round {
		Round {
			line,
			task: step.to_string(),
			artifact: "a".to_string(),
			outcome,
			consecutive_clean,
			risk_class,
			step: Some(step.to_string()),
			increment: Some(increment.to_string()),
		}
	}

	/// Build a `StepInfo` fixture directly (the fields are module-private).
	fn test_step(
		slug: &str,
		order: u64,
		phase: StepPhase,
		blocked_by: &[&str],
	) -> StepInfo {
		StepInfo {
			slug: slug.to_string(),
			order,
			phase,
			blocked_by: blocked_by.iter().map(|blocker| (*blocker).to_string()).collect(),
		}
	}

	/// Project a fixture with the given steps, rounds, and echoed isolation tier.
	fn project_fixture(
		steps: &[StepInfo],
		rounds: &[Round],
		isolation_tier: &str,
	) -> NextProjection {
		let spec = WorkflowSpec::builtin();
		project(NextInputs {
			task: "demo".to_string(),
			source: "docs/plans/demo.plan.toml".to_string(),
			steps,
			rounds,
			spec: &spec,
			metrics_records: Some(rounds.len()),
			ledger_path: "docs/plans/demo.ledger.md".to_string(),
			resume_state: None,
			isolation_tier: isolation_tier.to_string(),
			principles: &[],
		})
	}

	/// Project a fixture, additionally supplying the plan's `[[principle]]` list so the
	/// escalate human-input-contract reminder can project a real principle by name.
	fn project_fixture_with_principles(
		steps: &[StepInfo],
		rounds: &[Round],
		isolation_tier: &str,
		principles: &[Principle],
	) -> NextProjection {
		let spec = WorkflowSpec::builtin();
		project(NextInputs {
			task: "demo".to_string(),
			source: "docs/plans/demo.plan.toml".to_string(),
			steps,
			rounds,
			spec: &spec,
			metrics_records: Some(rounds.len()),
			ledger_path: "docs/plans/demo.ledger.md".to_string(),
			resume_state: None,
			isolation_tier: isolation_tier.to_string(),
			principles,
		})
	}

	/// Build a `[[principle]]` fixture (fields are `pub(crate)`, constructible in-crate).
	fn test_principle(
		n: u64,
		name: &str,
		text: &str,
	) -> Principle {
		Principle {
			n,
			name: name.to_string(),
			text: text.to_string(),
		}
	}

	/// The active loop of a fixture projection, panicking when there is none.
	fn active(projection: &NextProjection) -> &ActiveLoop {
		projection.active_loop.as_ref().expect("fixture has an active loop")
	}

	// -- Transition-table coverage (one test per row) --

	#[test]
	fn ready_to_plan_row() {
		let steps = [test_step("a", 0, StepPhase::NotStarted, &[])];
		let projection = project_fixture(&steps, &[], "worktree");
		let loop_ = active(&projection);
		assert_eq!(loop_.state, LoopState::ReadyToPlan);
		assert_eq!(loop_.valid_transitions, vec!["start-plan"]);
		assert_eq!(loop_.next_instruction.role, "planner");
		assert_eq!(loop_.phase, "not started");
		assert_eq!(loop_.increment, None);
	}

	#[test]
	fn blocked_row() {
		// The only pending step is blocked by `dep`, which does not resolve to a complete
		// step, so no ready step exists and the blocked step is reported.
		let steps = [test_step("a", 0, StepPhase::NotStarted, &["dep"])];
		let projection = project_fixture(&steps, &[], "worktree");
		let loop_ = active(&projection);
		assert_eq!(loop_.state, LoopState::Blocked);
		assert!(loop_.valid_transitions.is_empty());
		assert_eq!(loop_.next_instruction.role, "orchestrator");
		assert_eq!(loop_.next_instruction.context.get("blocked_by").map(String::as_str), Some("dep"));
	}

	#[test]
	fn awaiting_first_review_row() {
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let projection = project_fixture(&steps, &[], "worktree");
		let loop_ = active(&projection);
		assert_eq!(loop_.state, LoopState::AwaitingFirstReview);
		assert_eq!(loop_.valid_transitions, vec!["record-round"]);
		assert_eq!(loop_.next_instruction.role, "reviewer");
		assert_eq!(loop_.increment, None);
		assert_eq!(loop_.consecutive_clean, 0);
		assert_eq!(loop_.required_streak, None);
	}

	#[test]
	fn awaiting_fixes_row() {
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let rounds = [round(1, RoundOutcome::NewValid, 0, RiskClass::LowRisk, "a", "a-inc")];
		let projection = project_fixture(&steps, &rounds, "worktree");
		let loop_ = active(&projection);
		assert_eq!(loop_.state, LoopState::AwaitingFixes);
		assert_eq!(loop_.valid_transitions, vec!["address-findings"]);
		assert_eq!(loop_.next_instruction.role, "implementer");
		assert_eq!(loop_.increment.as_deref(), Some("a-inc"));
	}

	#[test]
	fn awaiting_reviewers_row() {
		// Risky needs 2 clean; a single clean round is short, so a fresh reviewer is owed.
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let rounds = [round(1, RoundOutcome::Clean, 1, RiskClass::Risky, "a", "a-inc")];
		let projection = project_fixture(&steps, &rounds, "worktree");
		let loop_ = active(&projection);
		assert_eq!(loop_.state, LoopState::AwaitingReviewers);
		assert_eq!(
			loop_.valid_transitions,
			vec!["record-round-clean", "record-round-new-valid", "escalate"]
		);
		assert_eq!(loop_.next_instruction.role, "reviewer");
		assert_eq!(loop_.consecutive_clean, 1);
		assert_eq!(loop_.required_streak, Some(2));
	}

	#[test]
	fn converged_row() {
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let rounds = [round(1, RoundOutcome::Clean, 1, RiskClass::LowRisk, "a", "a-inc")];
		let projection = project_fixture(&steps, &rounds, "worktree");
		let loop_ = active(&projection);
		assert_eq!(loop_.state, LoopState::Converged);
		assert_eq!(loop_.valid_transitions, vec!["mark-step-complete"]);
		assert_eq!(loop_.next_instruction.role, "orchestrator");
	}

	#[test]
	fn escalate_row() {
		// Five new-valid rounds reach the built-in cap (5) without ever converging.
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let rounds: Vec<Round> = (1 ..= 5)
			.map(|line| round(line, RoundOutcome::NewValid, 0, RiskClass::LowRisk, "a", "a-inc"))
			.collect();
		let projection = project_fixture(&steps, &rounds, "worktree");
		let loop_ = active(&projection);
		assert_eq!(loop_.state, LoopState::Escalate);
		assert_eq!(loop_.valid_transitions, vec!["escalate-to-human"]);
		assert_eq!(loop_.total_rounds, 5);
		assert_eq!(loop_.round_cap, 5);
	}

	#[test]
	fn risk_class_conflict_row() {
		// One increment whose rounds disagree on risk_class: the required streak is
		// undefined, so `next` reports the data-integrity fault rather than a verdict.
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let rounds = [
			round(1, RoundOutcome::Clean, 1, RiskClass::LowRisk, "a", "a-inc"),
			round(2, RoundOutcome::Clean, 1, RiskClass::Risky, "a", "a-inc"),
		];
		let projection = project_fixture(&steps, &rounds, "worktree");
		let loop_ = active(&projection);
		assert_eq!(loop_.state, LoopState::RiskClassConflict);
		assert_eq!(loop_.valid_transitions, vec!["fix-risk-class-records"]);
		assert_eq!(loop_.next_instruction.role, "orchestrator");
		assert_eq!(loop_.state.label(), "risk-class-conflict");
	}

	#[test]
	fn done_row_metadata() {
		// `done` is a complete step's state; selection never picks a complete step, so the
		// row is pinned by its state metadata directly.
		assert_eq!(LoopState::Done.label(), "done");
		assert!(LoopState::Done.valid_transitions().is_empty());
		assert_eq!(LoopState::Done.role(), "orchestrator");
		assert_eq!(LoopState::Done.next_action(), "move to the next step");
	}

	// -- Convergence check evaluated before the escalate cap --

	#[test]
	fn a_converging_round_at_the_cap_converges_not_escalates() {
		// Low-risk converges at 1; a fifth round that is the converging clean round hits the
		// cap and converges (convergence checked BEFORE escalate).
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let mut rounds: Vec<Round> = (1 ..= 4)
			.map(|line| round(line, RoundOutcome::NewValid, 0, RiskClass::LowRisk, "a", "a-inc"))
			.collect();
		rounds.push(round(5, RoundOutcome::Clean, 1, RiskClass::LowRisk, "a", "a-inc"));
		let projection = project_fixture(&steps, &rounds, "worktree");
		assert_eq!(active(&projection).state, LoopState::Converged);
	}

	// -- The differential (key acceptance) test --

	/// For a set of rounds on one in-progress increment, `next`'s forward `converged`
	/// verdict must agree with `w3_problems`' backward "no shortfall" verdict on the same
	/// records (both call the shared `peak_consecutive_clean` + join accessors).
	fn assert_differential(rounds: Vec<Round>) {
		let spec = WorkflowSpec::builtin();
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let projection = project_fixture(&steps, &rounds, "worktree");
		let next_converged =
			projection.active_loop.as_ref().is_some_and(|loop_| loop_.state == LoopState::Converged);

		// W3 checks a COMPLETE step; feed it the same records and the same built-in spec.
		let w3_steps = [crate::plan::Step {
			slug: "a".to_string(),
			status: "complete".to_string(),
		}];
		let w3_clean = crate::workflow::w3_problems(&spec, &w3_steps, &rounds, &[]).is_empty();

		assert_eq!(next_converged, w3_clean, "rounds: {rounds:?}");
	}

	#[test]
	fn next_agrees_with_w3() {
		// Converged cases: next says converged, W3 finds no shortfall.
		assert_differential(vec![round(1, RoundOutcome::Clean, 1, RiskClass::LowRisk, "a", "a-inc")]);
		assert_differential(vec![
			round(1, RoundOutcome::NewValid, 0, RiskClass::Risky, "a", "a-inc"),
			round(2, RoundOutcome::Clean, 1, RiskClass::Risky, "a", "a-inc"),
			round(3, RoundOutcome::Clean, 2, RiskClass::Risky, "a", "a-inc"),
		]);
		// Shortfall cases: next says not-converged, W3 reports a shortfall.
		assert_differential(vec![round(1, RoundOutcome::NewValid, 0, RiskClass::LowRisk, "a", "a-inc")]);
		assert_differential(vec![round(1, RoundOutcome::Clean, 1, RiskClass::Risky, "a", "a-inc")]);
		assert_differential(
			(1 ..= 5)
				.map(|line| round(line, RoundOutcome::NewValid, 0, RiskClass::LowRisk, "a", "a-inc"))
				.collect(),
		);
		// Mixed-class case: one increment whose rounds disagree on risk_class. This would
		// read as `converged` if `next` naively took `records[0].risk_class` (LowRisk needs
		// 1, peak is 1), but W3 treats the inconsistency as a shortfall. Both directions must
		// agree it is NOT converged, so this fixture fails if the RiskClassConflict guard
		// regresses.
		assert_differential(vec![
			round(1, RoundOutcome::Clean, 1, RiskClass::LowRisk, "a", "a-inc"),
			round(2, RoundOutcome::Clean, 1, RiskClass::Risky, "a", "a-inc"),
		]);
		// Multi-increment case: a step with two increments where inc1's rounds disagree on
		// risk_class (conflicted) and inc2 is genuinely clean and owns the latest-line round.
		// Without the conflict pre-check in `select_active_increment`, inc1 clears its
		// `records[0]` bar (LowRisk needs 1, peak is 1) and is skipped as converged, the
		// all-converged fallback returns the latest-line clean inc2, and `next` would read
		// `Converged` while W3 still flags inc1's inconsistency: a step-level false green. The
		// pre-check returns the conflicted inc1 as the active loop instead, so `next` reports
		// RiskClassConflict and agrees with W3. This fixture fails if that pre-check regresses.
		assert_differential(vec![
			round(1, RoundOutcome::Clean, 1, RiskClass::LowRisk, "a", "inc1"),
			round(2, RoundOutcome::Clean, 1, RiskClass::Risky, "a", "inc1"),
			round(3, RoundOutcome::Clean, 1, RiskClass::LowRisk, "a", "inc2"),
		]);
	}

	// -- Q-54 gate reminder --

	fn reminders_cite_the_contract(loop_: &ActiveLoop) -> bool {
		loop_
			.next_instruction
			.principle_reminders
			.iter()
			.any(|reminder| reminder.to_lowercase().contains("human-input contract"))
	}

	#[test]
	fn the_human_input_contract_reminder_is_present_only_at_escalate() {
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];

		let escalate_rounds: Vec<Round> = (1 ..= 5)
			.map(|line| round(line, RoundOutcome::NewValid, 0, RiskClass::LowRisk, "a", "a-inc"))
			.collect();
		let escalate = project_fixture(&steps, &escalate_rounds, "worktree");
		assert_eq!(active(&escalate).state, LoopState::Escalate);
		assert!(reminders_cite_the_contract(active(&escalate)));

		let review_rounds = [round(1, RoundOutcome::Clean, 1, RiskClass::Risky, "a", "a-inc")];
		let review = project_fixture(&steps, &review_rounds, "worktree");
		assert_eq!(active(&review).state, LoopState::AwaitingReviewers);
		assert!(!reminders_cite_the_contract(active(&review)));
	}

	// -- The always-on writer-isolation reminder --

	/// Whether any reminder carries the generated isolation-policy fragment (the single
	/// source shared with AGENTS.md). Matching the whole fragment, not a paraphrase, pins
	/// that the driver emits the shared const and not a hand-copy.
	fn reminders_carry_the_isolation_fragment(loop_: &ActiveLoop) -> bool {
		loop_
			.next_instruction
			.principle_reminders
			.iter()
			.any(|reminder| reminder.contains(ISOLATION_POLICY_FRAGMENT))
	}

	/// Whether any reminder carries the resolve-the-tier note (fired only when the tier is
	/// still `unknown` at a writer state).
	fn reminders_carry_the_resolve_note(loop_: &ActiveLoop) -> bool {
		loop_
			.next_instruction
			.principle_reminders
			.iter()
			.any(|reminder| reminder.contains(TIER_RESOLVE_NOTE))
	}

	/// The `ready-to-plan` (planner) writer state, with the given echoed tier.
	fn ready_to_plan_loop(isolation_tier: &str) -> NextProjection {
		let steps = [test_step("a", 0, StepPhase::NotStarted, &[])];
		project_fixture(&steps, &[], isolation_tier)
	}

	/// The `awaiting-fixes` (implementer) writer state, with the given echoed tier.
	fn awaiting_fixes_loop(isolation_tier: &str) -> NextProjection {
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let rounds = [round(1, RoundOutcome::NewValid, 0, RiskClass::LowRisk, "a", "a-inc")];
		project_fixture(&steps, &rounds, isolation_tier)
	}

	#[test]
	fn a_writer_state_emits_the_isolation_fragment_for_any_tier() {
		// The policy fragment fires UNCONDITIONALLY at writer states, whether the tier is
		// known or unknown, at both writer states (planner and implementer). This is the
		// motivating case: the policy is inline at the point of action and cannot drift.
		for state in [ready_to_plan_loop("worktree"), ready_to_plan_loop("unknown")] {
			assert_eq!(active(&state).state, LoopState::ReadyToPlan);
			assert!(reminders_carry_the_isolation_fragment(active(&state)));
		}
		for state in [awaiting_fixes_loop("container"), awaiting_fixes_loop("unknown")] {
			assert_eq!(active(&state).state, LoopState::AwaitingFixes);
			assert!(reminders_carry_the_isolation_fragment(active(&state)));
		}
	}

	#[test]
	fn a_writer_state_echoes_the_resolved_tier_in_the_isolation_reminder() {
		let known = ready_to_plan_loop("worktree");
		assert!(
			active(&known)
				.next_instruction
				.principle_reminders
				.iter()
				.any(|reminder| reminder.contains("resolved tier: worktree"))
		);
		// A known tier does NOT carry the resolve-the-tier note; only an unknown tier does.
		assert!(!reminders_carry_the_resolve_note(active(&known)));
	}

	#[test]
	fn an_unknown_tier_adds_the_resolve_note_at_a_writer_state() {
		let unknown = awaiting_fixes_loop("unknown");
		assert!(reminders_carry_the_resolve_note(active(&unknown)));
		// The fragment is still present alongside the resolve note.
		assert!(reminders_carry_the_isolation_fragment(active(&unknown)));
	}

	#[test]
	fn the_reviewer_states_now_carry_the_isolation_reminder() {
		// Q-62 (decided 2026-07-23, option (a)) widened `spawns_isolated_agent` to include
		// the two reviewer/triager-spawn states, so the always-on isolation reminder (policy
		// fragment plus tier echo) DOES attach to a reviewer spawn: under the
		// uniform-isolation rule reviewers isolate too, since even a findings file is a
		// write. Option (c) (keep the tier echo writer-only) was rejected, so the full
		// reminder fires here. Test both review states; the fragment fires at any tier, and
		// an unknown tier additionally carries the resolve-the-tier note.
		let first_review_steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		for tier in ["worktree", "unknown"] {
			let first_review = project_fixture(&first_review_steps, &[], tier);
			assert_eq!(active(&first_review).state, LoopState::AwaitingFirstReview);
			assert!(reminders_carry_the_isolation_fragment(active(&first_review)));
			assert_eq!(reminders_carry_the_resolve_note(active(&first_review)), tier == "unknown");

			let review_rounds = [round(1, RoundOutcome::Clean, 1, RiskClass::Risky, "a", "a-inc")];
			let awaiting_reviewers = project_fixture(&first_review_steps, &review_rounds, tier);
			assert_eq!(active(&awaiting_reviewers).state, LoopState::AwaitingReviewers);
			assert!(reminders_carry_the_isolation_fragment(active(&awaiting_reviewers)));
			assert_eq!(
				reminders_carry_the_resolve_note(active(&awaiting_reviewers)),
				tier == "unknown"
			);
		}
	}

	// -- De-numbered terse reminders + projected escalate principle (CHANGE C) --

	#[test]
	fn no_terse_reminder_carries_a_numeric_principle_citation() {
		// D-a Option 2: the workflow-phase reminders are honest originated imperatives with
		// no "Principle N:" numeric citation. Sweep every state's base reminders.
		for state in [
			LoopState::ReadyToPlan,
			LoopState::Blocked,
			LoopState::AwaitingFirstReview,
			LoopState::AwaitingFixes,
			LoopState::AwaitingReviewers,
			LoopState::Converged,
			LoopState::Escalate,
			LoopState::RiskClassConflict,
			LoopState::Done,
		] {
			for reminder in state.base_reminders() {
				assert!(
					!reminder.contains("Principle "),
					"state {state:?} still cites a numbered principle: {reminder}"
				);
			}
		}
	}

	/// The grounding principle's name, mapped to both the escalate and ready-to-plan states
	/// by `phase_principle_names`. A test-local literal, since the production const was
	/// generalised into the phase map.
	const GROUNDING_PRINCIPLE_NAME: &str = "Ground decisions in evidence";

	#[test]
	fn the_escalate_reminder_projects_a_real_plan_principle_by_name() {
		// The escalate human-input-contract reminder projects the actual name and text of
		// the plan's "Ground decisions in evidence" principle (D-e Option 1), self-contained
		// for the human, not a numeric citation.
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let escalate_rounds: Vec<Round> = (1 ..= 5)
			.map(|line| round(line, RoundOutcome::NewValid, 0, RiskClass::LowRisk, "a", "a-inc"))
			.collect();
		let principles = [
			test_principle(6, GROUNDING_PRINCIPLE_NAME, "validate an approach with evidence."),
			test_principle(2, "Minimal by default", "the core does one thing well."),
		];
		let projection =
			project_fixture_with_principles(&steps, &escalate_rounds, "worktree", &principles);
		assert_eq!(active(&projection).state, LoopState::Escalate);
		let projected = active(&projection)
			.next_instruction
			.principle_reminders
			.iter()
			.find(|reminder| reminder.contains(GROUNDING_PRINCIPLE_NAME))
			.expect("the escalate reminder projects the grounding principle by name");
		// The projected reminder carries the real text and the plan's own number, never a
		// hardcoded AGENTS.md number.
		assert!(projected.contains("validate an approach with evidence."));
		assert!(projected.contains("plan principle 6"));
	}

	#[test]
	fn the_ready_to_plan_reminder_projects_the_grounding_principle_by_name() {
		// The phase map now also puts the grounding principle in front of the planner
		// (D3 generalisation): the ready-to-plan state projects the same "Ground decisions
		// in evidence" principle by name, so the planner validates its approach with
		// evidence. Same projection form as the escalate case.
		let steps = [test_step("a", 0, StepPhase::NotStarted, &[])];
		let principles = [
			test_principle(6, GROUNDING_PRINCIPLE_NAME, "validate an approach with evidence."),
			test_principle(2, "Minimal by default", "the core does one thing well."),
		];
		let projection = project_fixture_with_principles(&steps, &[], "worktree", &principles);
		assert_eq!(active(&projection).state, LoopState::ReadyToPlan);
		let projected = active(&projection)
			.next_instruction
			.principle_reminders
			.iter()
			.find(|reminder| reminder.contains(GROUNDING_PRINCIPLE_NAME))
			.expect("the ready-to-plan reminder projects the grounding principle by name");
		assert!(projected.contains("validate an approach with evidence."));
		assert!(projected.contains("plan principle 6"));
	}

	#[test]
	fn an_unmapped_state_projects_no_principle() {
		// A state the phase map does not populate (here: awaiting-fixes) projects no Project
		// Principle reminder even when the plan carries the grounding principle: the map is
		// populated conservatively, not forced onto every state.
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let rounds = [round(1, RoundOutcome::NewValid, 0, RiskClass::LowRisk, "a", "a-inc")];
		let principles =
			[test_principle(6, GROUNDING_PRINCIPLE_NAME, "validate an approach with evidence.")];
		let projection = project_fixture_with_principles(&steps, &rounds, "worktree", &principles);
		assert_eq!(active(&projection).state, LoopState::AwaitingFixes);
		assert!(
			!active(&projection)
				.next_instruction
				.principle_reminders
				.iter()
				.any(|reminder| reminder.contains(GROUNDING_PRINCIPLE_NAME))
		);
	}

	#[test]
	fn the_escalate_reminder_degrades_when_the_principle_is_absent() {
		// When the plan does not carry the grounding principle (here: no principles at all,
		// as for a Markdown source), the reminder degrades to the originated human-input
		// contract alone, never a dangling number (D-e Option 1). The base contract reminder
		// is still present; no reminder mentions the missing principle name.
		let steps = [test_step("a", 0, StepPhase::InProgress, &[])];
		let escalate_rounds: Vec<Round> = (1 ..= 5)
			.map(|line| round(line, RoundOutcome::NewValid, 0, RiskClass::LowRisk, "a", "a-inc"))
			.collect();
		let projection = project_fixture(&steps, &escalate_rounds, "worktree");
		assert_eq!(active(&projection).state, LoopState::Escalate);
		assert!(reminders_cite_the_contract(active(&projection)));
		assert!(
			!active(&projection)
				.next_instruction
				.principle_reminders
				.iter()
				.any(|reminder| reminder.contains(GROUNDING_PRINCIPLE_NAME))
		);
	}

	// -- RESUME STATE extractor --

	#[test]
	fn resume_state_is_extracted_verbatim() {
		let fragment = "## Round 1\nnoise\n\n## RESUME STATE (checkpoint)\nline one\nline two\n\n## Next section\nother\n";
		let extracted = extract_resume_state(fragment).expect("section present");
		assert_eq!(extracted, "## RESUME STATE (checkpoint)\nline one\nline two");
	}

	#[test]
	fn resume_state_absent_is_none() {
		let fragment = "## Round 1\nnoise\n\n## Other\ntext\n";
		assert_eq!(extract_resume_state(fragment), None);
	}

	#[test]
	fn resume_state_terminates_at_the_next_level_two_heading_only() {
		// A nested `### ` heading inside the block does NOT terminate it; the next `## `
		// does.
		let fragment = "## RESUME STATE\nbody\n### nested\nmore body\n## After\ntrailing\n";
		let extracted = extract_resume_state(fragment).expect("section present");
		assert_eq!(extracted, "## RESUME STATE\nbody\n### nested\nmore body");
	}

	// -- Golden output (byte-compare) --

	fn golden_projection() -> NextProjection {
		let steps = [test_step("core-assets", 0, StepPhase::InProgress, &[])];
		let rounds = [round(1, RoundOutcome::Clean, 1, RiskClass::Risky, "core-assets", "core-assets-inc1")];
		let spec = WorkflowSpec::builtin();
		project(NextInputs {
			task: "demo".to_string(),
			source: "docs/plans/demo.plan.toml".to_string(),
			steps: &steps,
			rounds: &rounds,
			spec: &spec,
			metrics_records: Some(1),
			ledger_path: "docs/plans/demo.ledger.md".to_string(),
			resume_state: None,
			isolation_tier: "worktree".to_string(),
			principles: &[],
		})
	}

	const GOLDEN_HUMAN: &str = "\
task: demo
source: docs/plans/demo.plan.toml
metrics: 1 records

ACTIVE LOOP
  core-assets / core-assets-inc1  in progress -> record-round-clean
  state: awaiting-reviewers
  streak: 1/2
  rounds: 1/5
  isolation: worktree
  next: spawn a fresh reviewer (diversify the model)
  role: reviewer
  prompt: .agents/prompts/reviewer.md
  context:
    isolation_tier: worktree
    ledger: docs/plans/demo.ledger.md
    review_findings: docs/plans/demo.reviews/core-assets-reviewer-<disambiguator>.md
    triage_findings: docs/plans/demo.reviews/core-assets-triage.md
  reminders:
    - Staff a fresh, independent reviewer and diversify the model.
    - Cite the file and line for each finding.
    - Agent isolation (resolved tier: worktree). __ISOLATION_FRAGMENT__
  summary: fresh review round on step `core-assets` increment `core-assets-inc1` (streak 1/2).";

	const GOLDEN_JSON: &str = r#"{
  "task": "demo",
  "source": "docs/plans/demo.plan.toml",
  "metrics": {
    "records": 1
  },
  "active_loop": {
    "step": "core-assets",
    "increment": "core-assets-inc1",
    "phase": "in progress",
    "state": "awaiting-reviewers",
    "risk_class": "risky",
    "consecutive_clean": 1,
    "required_streak": 2,
    "total_rounds": 1,
    "round_cap": 5,
    "valid_transitions": [
      "record-round-clean",
      "record-round-new-valid",
      "escalate"
    ],
    "isolation_tier": "worktree",
    "next_instruction": {
      "role": "reviewer",
      "prompt_path": ".agents/prompts/reviewer.md",
      "context": {
        "isolation_tier": "worktree",
        "ledger": "docs/plans/demo.ledger.md",
        "review_findings": "docs/plans/demo.reviews/core-assets-reviewer-<disambiguator>.md",
        "triage_findings": "docs/plans/demo.reviews/core-assets-triage.md"
      },
      "principle_reminders": [
        "Staff a fresh, independent reviewer and diversify the model.",
        "Cite the file and line for each finding.",
        "Agent isolation (resolved tier: worktree). __ISOLATION_FRAGMENT__"
      ],
      "filled_prompt_summary": "fresh review round on step `core-assets` increment `core-assets-inc1` (streak 1/2)."
    }
  },
  "resume_state": null
}"#;

	// The golden fixtures carry the reviewer-state isolation reminder (Q-62 widened the
	// driver to emit it here). The long policy fragment is single-sourced from
	// `ISOLATION_POLICY_FRAGMENT`: the golden holds a `__ISOLATION_FRAGMENT__` placeholder,
	// substituted here, so the snapshot cannot drift from the shared const and there is no
	// hand-copied duplicate to keep in step.
	fn golden_with_fragment(golden: &str) -> String {
		golden.replace("__ISOLATION_FRAGMENT__", ISOLATION_POLICY_FRAGMENT)
	}

	#[test]
	fn golden_human_text() {
		assert_eq!(render_human(&golden_projection()), golden_with_fragment(GOLDEN_HUMAN));
	}

	#[test]
	fn golden_json() {
		let json = serde_json::to_string_pretty(&golden_projection()).expect("serialises");
		assert_eq!(json, golden_with_fragment(GOLDEN_JSON));
	}

	// -- Renderer idempotence --

	/// The renderers are idempotent within a call: rendering the same projection twice
	/// yields identical bytes. This guards against a renderer that reads shared mutable
	/// state or otherwise varies between invocations; it is NOT a cross-run determinism
	/// check (that is owned structurally, by the `BTreeMap` context ordering and the
	/// wall-clock-free paths, and by the `golden_human_text` / `golden_json` byte-compares).
	#[test]
	fn the_renderers_are_idempotent_within_a_call() {
		let projection = golden_projection();
		assert_eq!(
			serde_json::to_string_pretty(&projection).unwrap(),
			serde_json::to_string_pretty(&projection).unwrap()
		);
		assert_eq!(render_human(&projection), render_human(&projection));
	}

	// -- Dual-source parity --

	#[test]
	fn toml_and_markdown_sources_give_the_same_verdict() {
		let toml = "\
[meta]
title = \"Demo\"
primary = \"toml\"

[[step]]
slug = \"alpha\"
title = \"Alpha\"
status = \"in-progress\"
order = 0

[[step]]
slug = \"beta\"
title = \"Beta\"
status = \"not-started\"
order = 1
";
		let parsed = crate::plan::parse_toml(toml).expect("valid toml");
		let toml_steps = steps_from_toml(&parsed);

		let markdown = "\
## Roadmap

| Step | Status |
| --- | --- |
| `alpha` | in progress |
| `beta` | not started |
";
		let markdown_steps = steps_from_markdown(&crate::plan::parse_roadmap(markdown));

		let rounds = [round(1, RoundOutcome::Clean, 1, RiskClass::LowRisk, "alpha", "alpha-inc1")];

		let from_toml = project_fixture(&toml_steps, &rounds, "worktree");
		let from_markdown = project_fixture(&markdown_steps, &rounds, "worktree");

		let toml_loop = active(&from_toml);
		let markdown_loop = active(&from_markdown);
		assert_eq!(toml_loop.step, markdown_loop.step);
		assert_eq!(toml_loop.increment, markdown_loop.increment);
		assert_eq!(toml_loop.state, markdown_loop.state);
		assert_eq!(toml_loop.state, LoopState::Converged);
	}
}
