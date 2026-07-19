//! The `<task>.plan.toml` structured-source schema: parse a clean-slate plan
//! skeleton into precise types and cross-reference it (`validate --source`).
//!
//! This is the design-B (Q-45/Q-46) source-of-truth schema. It is a PURE
//! ADDITION for increment 1: nothing in the live pipeline reads it yet (the live
//! repo stays Markdown-sourced until the Inc 5 cutover, and W3/W4/W5 keep reading
//! the Markdown plan and the JSONL log). The module exists so the schema, its
//! parser, and its internal-consistency checks land and are tested before any
//! render engine (Inc 3) or enforcement swap (Inc 4) depends on them.
//!
//! Parse, don't validate (Principle 14): `parse_toml` deserialises the TOML into
//! precise typed structs with enums for the status/risk/reason/tier fields, so an
//! out-of-set value is rejected at the boundary rather than carried as a string
//! (Principle 13, illegal states unrepresentable). `validate_source` then runs the
//! cross-reference checks a single record cannot express: unique ids, resolving
//! `blocked_by`/`folds`/`folded_into`/`superseded_by`, a decided question implying
//! a resolving `folded_into`, and the waiver field-presence and `reason` <->
//! `evidence_tier` pairing rules (single-sourced with `metrics`/`workflow`).
//!
//! The status/risk/reason/tier vocabularies for the waiver and increment fields
//! REUSE the `metrics` enums (`RiskClass`, `WaiverUnit`, `WaiverReason`,
//! `EvidenceTier`), so the accepted spellings have one source of truth
//! (Principle 16) across the JSONL log and this TOML schema. The step and question
//! status vocabularies are this schema's own (the Markdown parametric
//! `blocked on <slug>` / `decided -> folded into <slug>` strings are retired in
//! favour of the typed `blocked_by` / `folded_into` cross-reference fields).

use {
	crate::metrics::{
		EvidenceTier,
		RiskClass,
		WaiverReason,
		WaiverUnit,
		question_id_index,
	},
	serde::{
		Deserialize,
		Serialize,
	},
	std::{
		collections::BTreeSet,
		path::{
			Component,
			Path,
		},
	},
};

/// A parsed `<task>.plan.toml` document: the `[meta]` block, the `[[step]]`,
/// `[[question]]`, and `[[principle]]` arrays. `parse_toml` returns this whole
/// document rather than a bare `(steps, questions, meta)` tuple so the principles
/// (also part of the skeleton) are not dropped; it is a strict superset of that
/// tuple with the fields named for the region they carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PlanToml {
	/// The `[meta]` block: title, render/enforcement metadata, and sidecar refs.
	pub(crate) meta: Meta,
	/// The Roadmap steps (`[[step]]`), each with its increments and waivers.
	#[serde(default, rename = "step")]
	pub(crate) steps: Vec<Step>,
	/// The Open-Questions queue items (`[[question]]`).
	#[serde(default, rename = "question")]
	pub(crate) questions: Vec<Question>,
	/// The project principles (`[[principle]]`).
	#[serde(default, rename = "principle")]
	pub(crate) principles: Vec<Principle>,
}

/// The projection kind that owns the plan's status when both substrates are
/// present. `markdown` (the default when `[meta].primary` is absent) keeps the
/// Markdown parser the source; `toml` makes this schema the source. Until the Inc 5
/// cutover the live repo declares `markdown`, so nothing reads the TOML in anger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Primary {
	/// The Markdown plan is the source of truth (the default).
	#[default]
	Markdown,
	/// The `<task>.plan.toml` is the source of truth.
	Toml,
}

/// The front-matter and tail prose sidecars the render engine (Inc 3) splices
/// around the generated Roadmap. `front` is the ordered set of front-matter
/// sidecar refs (intro / motivations / repo-layout); `tail` is the Success-Criteria
/// tail prose ref. Held here so the schema is complete now; render consumes them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct Sidecars {
	/// Ordered front-matter sidecar references, spliced before the Roadmap.
	#[serde(default)]
	pub(crate) front: Vec<String>,
	/// The Success-Criteria tail prose sidecar reference, spliced after the Roadmap.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub(crate) tail: Option<String>,
}

/// The `[meta]` block. Scalar keys come first so a TOML re-serialisation stays
/// valid (the `sidecars` sub-table must follow the scalar keys of its parent).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Meta {
	/// The plan/task title.
	pub(crate) title: String,
	/// The W4 decided-question exemption cutoff (`Q-<n>`), when the project
	/// migrated with pre-existing decisions. Absent for a fresh project.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub(crate) w4_baseline: Option<String>,
	/// Which substrate owns the plan's status (default `markdown`).
	#[serde(default)]
	pub(crate) primary: Primary,
	/// The orphan `task` slugs: tasks that appear in the round log but own no
	/// Roadmap step, declared here so they are visible rather than inferred.
	#[serde(default)]
	pub(crate) orphan_tasks: Vec<String>,
	/// The digest `render --check` (Inc 3) re-computes and compares. Present as a
	/// field now so the schema is stable before render lands; unused in Inc 1.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub(crate) render_sha256: Option<String>,
	/// The front/tail prose sidecars (a sub-table, so it is declared last).
	#[serde(default)]
	pub(crate) sidecars: Sidecars,
}

/// One Roadmap step (`[[step]]`). Scalar and value keys come before the
/// array-of-table keys (`increment` / `waiver`) so a TOML re-serialisation stays
/// valid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Step {
	/// The step's kebab-case slug (its stable id, cross-referenced by others).
	pub(crate) slug: String,
	/// The human-readable step title.
	pub(crate) title: String,
	/// The step's status (its convergence state; `complete` is the one W3 keys on).
	pub(crate) status: StepStatus,
	/// The step's Roadmap order.
	pub(crate) order: u64,
	/// The slugs of the steps that block this one (typed, replacing the Markdown
	/// `blocked on <slug>` parametric status).
	#[serde(default)]
	pub(crate) blocked_by: Vec<String>,
	/// The slugs of the steps this step folds in (subsumes).
	#[serde(default)]
	pub(crate) folds: Vec<String>,
	/// The step's increments (`[[step.increment]]`).
	#[serde(default, rename = "increment")]
	pub(crate) increments: Vec<Increment>,
	/// The step's convergence-bar waivers (`[[step.waiver]]`).
	#[serde(default, rename = "waiver")]
	pub(crate) waivers: Vec<Waiver>,
}

/// The step statuses this schema accepts. The Markdown parametric
/// `blocked on <slug>` status is retired: blockedness is now the typed `blocked_by`
/// list, so a step is never `blocked`. `next`, `optional`, and `deferred` carry
/// over from the Markdown Roadmap vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum StepStatus {
	/// Not started.
	NotStarted,
	/// In progress.
	InProgress,
	/// Complete (the status W3 checks for converged rounds or a covering waiver).
	Complete,
	/// Skipped (answered "is this step done?" with "no"; not an exemption).
	Skipped,
	/// Queued next.
	Next,
	/// Optional.
	Optional,
	/// Deferred.
	Deferred,
}

impl StepStatus {
	/// Every variant in a fixed canonical order, so the render engine's derived
	/// status distribution is deterministic (Principle 16, one ordering).
	pub(crate) const ALL: [StepStatus; 7] = [
		StepStatus::NotStarted,
		StepStatus::InProgress,
		StepStatus::Complete,
		StepStatus::Skipped,
		StepStatus::Next,
		StepStatus::Optional,
		StepStatus::Deferred,
	];

	/// The human-readable Roadmap label, the space form the Markdown Roadmap uses
	/// (`not started`, `in progress`, ...). Every value here is a member of
	/// `plan::ROADMAP_STATUSES` (a test pins that), so the TOML enum and the Markdown
	/// vocabulary constant cannot drift (Principle 16).
	pub(crate) fn label(self) -> &'static str {
		match self {
			StepStatus::NotStarted => "not started",
			StepStatus::InProgress => "in progress",
			StepStatus::Complete => "complete",
			StepStatus::Skipped => "skipped",
			StepStatus::Next => "next",
			StepStatus::Optional => "optional",
			StepStatus::Deferred => "deferred",
		}
	}
}

/// Compile-time exhaustiveness guard for `StepStatus::ALL`: a wildcard-free match over
/// every variant, so adding a `StepStatus` makes this fail to compile until the new
/// variant is also listed in `ALL` above (a stale `ALL` would silently drop it from the
/// render engine's status distribution; Principle 16).
const _: () = match StepStatus::NotStarted {
	StepStatus::NotStarted
	| StepStatus::InProgress
	| StepStatus::Complete
	| StepStatus::Skipped
	| StepStatus::Next
	| StepStatus::Optional
	| StepStatus::Deferred => (),
};

/// One increment of a step (`[[step.increment]]`): a structured id and its
/// convergence risk class. The id retires the lexical `-inc<x>` strip (Inc 2), so
/// a round record can join to this id directly instead of by prefix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Increment {
	/// The increment's structured id (for example `structured-skeleton-inc1`).
	pub(crate) id: String,
	/// The increment's convergence risk class (sets its required clean-round streak).
	pub(crate) risk_class: RiskClass,
}

/// One convergence-bar waiver nested on the step it exempts (`[[step.waiver]]`).
/// This is the Q-46 re-home of the JSONL `type:"waiver"` record onto the TOML step;
/// the fields mirror that record (minus `task`/`step`, which the nesting supplies).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Waiver {
	/// The waiver's stable id.
	pub(crate) id: String,
	/// Whether the waiver covers the whole step or one of its increments.
	pub(crate) unit: WaiverUnit,
	/// The increment id this waiver covers, present exactly for an `increment`-unit
	/// waiver (checked against the step's increments by `validate_source`).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub(crate) increment: Option<String>,
	/// Why the waiver exempts its unit.
	pub(crate) reason: WaiverReason,
	/// How strongly the waiver is evidenced.
	pub(crate) evidence_tier: EvidenceTier,
	/// The backing-record pointer, present exactly for a `record-backed` waiver
	/// (for an `accepted-at-escalation` waiver it names the JSONL escalation record
	/// by its `task`; the cross-substrate join itself is Inc 4's job).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub(crate) evidence: Option<String>,
	/// An optional human note explaining the waiver.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub(crate) note: Option<String>,
}

/// One Open-Questions queue item (`[[question]]`). The queue's `options`/`chosen`
/// live ONLY in the JSONL `type:"decision"` receipt (decided sub-question 3(c)) and
/// are deliberately NOT carried here; `receipt` points at that receipt by `q_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Question {
	/// The item's stable id (`Q-<n>`).
	pub(crate) id: String,
	/// The item's status.
	pub(crate) status: QuestionStatus,
	/// The one-line ask.
	pub(crate) ask: String,
	/// The step slug this decision was folded into, present (and required) when the
	/// item is `decided`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub(crate) folded_into: Option<String>,
	/// The question id that supersedes this one, present when it is `superseded`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub(crate) superseded_by: Option<String>,
	/// A pointer (by `q_id`) to the JSONL `type:"decision"` receipt for this item.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub(crate) receipt: Option<String>,
}

/// The Open-Questions statuses this schema accepts. The Markdown parametric
/// `decided -> folded into <slug>` status is retired: a decided item is the
/// `decided` status plus the typed `folded_into` cross-reference field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum QuestionStatus {
	/// Open (options ready, awaiting the human's choice).
	Open,
	/// Exploring (a design pass is owed before the options are decidable).
	Exploring,
	/// Decided (folded into a step, named by `folded_into`).
	Decided,
	/// Superseded (replaced by a later item, named by `superseded_by`).
	Superseded,
}

impl QuestionStatus {
	/// Every variant in a fixed canonical order, so the render engine's generated
	/// queue-status vocabulary fragment is deterministic.
	pub(crate) const ALL: [QuestionStatus; 4] = [
		QuestionStatus::Open,
		QuestionStatus::Exploring,
		QuestionStatus::Decided,
		QuestionStatus::Superseded,
	];

	/// The human-readable queue label (`open`, `exploring`, `decided`, `superseded`).
	/// The parametric target (`folded_into` / `superseded_by`) is appended by render,
	/// not carried here.
	pub(crate) fn label(self) -> &'static str {
		match self {
			QuestionStatus::Open => "open",
			QuestionStatus::Exploring => "exploring",
			QuestionStatus::Decided => "decided",
			QuestionStatus::Superseded => "superseded",
		}
	}
}

/// Compile-time exhaustiveness guard for `QuestionStatus::ALL`: a wildcard-free match
/// over every variant, so adding a `QuestionStatus` makes this fail to compile until
/// the new variant is also listed in `ALL` above (a stale `ALL` would silently drop it
/// from the render engine's generated queue-status vocabulary; Principle 16).
const _: () = match QuestionStatus::Open {
	QuestionStatus::Open
	| QuestionStatus::Exploring
	| QuestionStatus::Decided
	| QuestionStatus::Superseded => (),
};

/// One project principle (`[[principle]]`): its number, name, and text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Principle {
	/// The principle number (for reference, not priority).
	pub(crate) n: u64,
	/// The principle's short name.
	pub(crate) name: String,
	/// The principle's text.
	pub(crate) text: String,
}

/// Parse a `<task>.plan.toml` into the typed `PlanToml`. Enums for the
/// status/risk/reason/tier fields mean an out-of-set value (for example a step
/// status of `done`) is rejected here at the boundary as a deserialisation error,
/// and a missing required field (for example a step with no `slug`) likewise.
/// The schema structs carry `#[serde(deny_unknown_fields)]`, so an unknown or
/// misspelled key (for example `blockd_by` for `blocked_by`) is a loud parse error
/// rather than a silently dropped field. This is deliberately stricter than the
/// round log's forward-compatible JSONL stance: the plan.toml is hand-authored and
/// versioned in-repo, so a typo that silently drops a correctness-bearing
/// cross-reference is the worse failure to tolerate.
pub(crate) fn parse_toml(contents: &str) -> Result<PlanToml, toml::de::Error> {
	toml::from_str(contents)
}

/// Whether `token` is a well-formed id token: non-empty, made of ASCII
/// alphanumerics and hyphens, and not starting or ending with a hyphen. Covers
/// waiver ids and orphan-task tokens (which may carry an uppercase `-incA`
/// suffix, so this is not lowercase-only). Step slugs and increment ids are
/// held to the stricter lowercase `is_kebab_case_token`.
fn is_well_formed_token(token: &str) -> bool {
	!token.is_empty()
		&& !token.starts_with('-')
		&& !token.ends_with('-')
		&& token.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

/// Whether `token` is a well-formed lowercase kebab-case token: `is_well_formed_token`
/// with no ASCII uppercase. Step slugs and increment ids are kebab-case by contract
/// (for example `structured-skeleton-inc1`), so they take this stricter rule; the
/// `[meta]` orphan-task tokens keep the looser `is_well_formed_token` to allow their
/// uppercase increment suffix (for example `round-log-core-incA`).
fn is_kebab_case_token(token: &str) -> bool {
	is_well_formed_token(token) && !token.bytes().any(|b| b.is_ascii_uppercase())
}

/// Whether a `[meta].sidecars` front/tail reference is safe to join onto the plan
/// directory: a task-relative path with no absolute root and no `..` (parent-dir)
/// component. The render engine joins these free-string refs straight onto the base
/// directory, so an absolute ref would discard the base and a `..`-bearing ref would
/// escape it, letting a crafted `.plan.toml` read a file OUTSIDE the plan directory and
/// splice its bytes into `<task>.md`. Rejecting both at the boundary (Principle 21,
/// validate external input where it enters; Principle 18, least authority) does not
/// depend on the referenced file existing, so `render`, `render --check`, and
/// `validate --source` all refuse the same thing.
fn is_safe_sidecar_ref(reference: &str) -> bool {
	let path = Path::new(reference);
	!path.is_absolute()
		&& path
			.components()
			.all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}

/// Validate a `<task>.plan.toml`'s schema (types and enums, via `parse_toml`) and
/// its internal cross-references, returning one human-readable problem per
/// violation (an empty vector means the source is well-formed). A malformed source
/// that will not even parse is reported as the single parse problem; on a clean
/// parse the cross-reference checks run:
///
/// - unique step slugs, increment ids, question ids, and waiver ids, each
///   well-formed (slugs and increment ids lowercase kebab-case), and unique
///   `principle.n` numbers;
/// - `blocked_by` / `folds` resolve to real steps and are not self-references;
///   `folded_into` resolves to a real step whenever present; `superseded_by`
///   resolves to a real question whenever present; a `decided` question carries
///   exactly a resolving `folded_into` and a `superseded` question exactly a
///   resolving `superseded_by` (and no other status carries either); `receipt` and
///   `w4_baseline` are well-formed `Q-<n>` ids;
/// - the `[meta]` orphan-task list is well-formed, unique, and disjoint from the
///   step slugs; and
/// - the waiver field-presence and `reason` <-> `evidence_tier` pairing rules
///   (moved from the round log's `check_record` waiver arm and the W5 pairing).
pub(crate) fn validate_source(contents: &str) -> Vec<String> {
	let plan = match parse_toml(contents) {
		Ok(plan) => plan,
		Err(error) => return vec![format!("malformed `<task>.plan.toml`: {error}")],
	};
	let mut problems = Vec::new();

	// Step slugs: well-formed and unique. Collect the set for the cross-references.
	let mut slugs: BTreeSet<&str> = BTreeSet::new();
	for step in &plan.steps {
		if !is_kebab_case_token(&step.slug) {
			problems.push(format!("step slug `{}` is not a well-formed kebab-case id", step.slug));
		}
		if !slugs.insert(step.slug.as_str()) {
			problems.push(format!("step slug `{}` appears more than once", step.slug));
		}
	}

	// Increment ids: well-formed and unique across the whole plan.
	let mut increment_ids: BTreeSet<&str> = BTreeSet::new();
	for step in &plan.steps {
		for increment in &step.increments {
			if !is_kebab_case_token(&increment.id) {
				problems.push(format!(
					"increment id `{}` is not a well-formed kebab-case id",
					increment.id
				));
			}
			if !increment_ids.insert(increment.id.as_str()) {
				problems.push(format!("increment id `{}` appears more than once", increment.id));
			}
		}
	}

	// Question ids: `Q-<n>` shaped and unique. Collect the set for `superseded_by`.
	let mut question_ids: BTreeSet<&str> = BTreeSet::new();
	for question in &plan.questions {
		if question_id_index(&question.id).is_none() {
			problems.push(format!("question id `{}` is not a `Q-<n>` id", question.id));
		}
		if !question_ids.insert(question.id.as_str()) {
			problems.push(format!("question id `{}` appears more than once", question.id));
		}
	}

	// Waiver ids: well-formed and unique across the whole plan (Inc 4's read path
	// joins on waiver identity, so a duplicate id would be an ambiguous join).
	let mut waiver_ids: BTreeSet<&str> = BTreeSet::new();
	for step in &plan.steps {
		for waiver in &step.waivers {
			if !is_well_formed_token(&waiver.id) {
				problems.push(format!("waiver id `{}` is not a well-formed id", waiver.id));
			}
			if !waiver_ids.insert(waiver.id.as_str()) {
				problems.push(format!("waiver id `{}` appears more than once", waiver.id));
			}
		}
	}

	// Principle numbers: unique across the plan.
	let mut principle_ns: BTreeSet<u64> = BTreeSet::new();
	for principle in &plan.principles {
		if !principle_ns.insert(principle.n) {
			problems.push(format!("principle `{}` appears more than once", principle.n));
		}
	}

	// Step cross-references: `blocked_by` and `folds` must name real steps and must
	// not name the step itself (a step cannot block or fold itself; multi-step cycle
	// detection is deferred to the increment that orders steps).
	for step in &plan.steps {
		for target in &step.blocked_by {
			if target == &step.slug {
				problems.push(format!("step `{}` is blocked_by itself", step.slug));
			} else if !slugs.contains(target.as_str()) {
				problems.push(format!(
					"step `{}` is blocked_by `{}`, which is not a step",
					step.slug, target
				));
			}
		}
		for target in &step.folds {
			if target == &step.slug {
				problems.push(format!("step `{}` folds itself", step.slug));
			} else if !slugs.contains(target.as_str()) {
				problems
					.push(format!("step `{}` folds `{}`, which is not a step", step.slug, target));
			}
		}
	}

	// Question cross-references and status <-> field consistency. Cross-references
	// resolve whenever present, regardless of status; the status-consistency rules
	// then make the illegal states unrepresentable both ways (a `decided` question
	// has exactly a resolving `folded_into`, a `superseded` question has exactly a
	// resolving `superseded_by`, and no other status carries either field).
	for question in &plan.questions {
		// `folded_into` resolves to a real step whenever present (not only when
		// `decided`), so a dangling target on any question is flagged.
		if let Some(target) = &question.folded_into {
			if !slugs.contains(target.as_str()) {
				problems.push(format!(
					"question `{}` folds into `{}`, which is not a step",
					question.id, target
				));
			}
		}
		// `superseded_by` resolves to a real question whenever present.
		if let Some(target) = &question.superseded_by {
			if !question_ids.contains(target.as_str()) {
				problems.push(format!(
					"question `{}` is superseded_by `{}`, which is not a question",
					question.id, target
				));
			}
		}
		// `folded_into` presence is tied to the `decided` status, both ways.
		match question.status {
			QuestionStatus::Decided =>
				if question.folded_into.is_none() {
					problems.push(format!(
						"question `{}` is `decided` but has no resolving `folded_into`",
						question.id
					));
				},
			QuestionStatus::Open | QuestionStatus::Exploring | QuestionStatus::Superseded =>
				if question.folded_into.is_some() {
					problems.push(format!(
						"question `{}` is not `decided` but carries `folded_into`",
						question.id
					));
				},
		}
		// `superseded_by` presence is tied to the `superseded` status, both ways.
		match question.status {
			QuestionStatus::Superseded =>
				if question.superseded_by.is_none() {
					problems.push(format!(
						"question `{}` is `superseded` but has no resolving `superseded_by`",
						question.id
					));
				},
			QuestionStatus::Open | QuestionStatus::Exploring | QuestionStatus::Decided =>
				if question.superseded_by.is_some() {
					problems.push(format!(
						"question `{}` is not `superseded` but carries `superseded_by`",
						question.id
					));
				},
		}
		if let Some(receipt) = &question.receipt {
			if question_id_index(receipt).is_none() {
				problems.push(format!(
					"question `{}` has a receipt pointer `{}` that is not a `Q-<n>` id",
					question.id, receipt
				));
			}
		}
	}

	// The `[meta]` W4 baseline cutoff, when present, must be a `Q-<n>` id.
	if let Some(cutoff) = &plan.meta.w4_baseline {
		if question_id_index(cutoff).is_none() {
			problems.push(format!("meta.w4_baseline `{cutoff}` is not a `Q-<n>` id"));
		}
	}

	// The `[meta].sidecars` front/tail refs are joined onto the plan directory by the
	// render engine, so an absolute or `..`-bearing ref would read a file outside it.
	// Reject both at the boundary (Principle 21) rather than at the render read.
	for reference in &plan.meta.sidecars.front {
		if !is_safe_sidecar_ref(reference) {
			problems.push(format!(
				"meta sidecar front ref `{reference}` must be a task-relative path (no absolute path, no `..` component)"
			));
		}
	}
	if let Some(reference) = &plan.meta.sidecars.tail {
		if !is_safe_sidecar_ref(reference) {
			problems.push(format!(
				"meta sidecar tail ref `{reference}` must be a task-relative path (no absolute path, no `..` component)"
			));
		}
	}

	// The `[meta]` orphan-task list must be well-formed task tokens, unique, and
	// genuinely orphan: an orphan token equal to a declared step slug contradicts
	// the field's own definition (a task that owns no Roadmap step).
	let mut orphan_tasks: BTreeSet<&str> = BTreeSet::new();
	for task in &plan.meta.orphan_tasks {
		if !is_well_formed_token(task) {
			problems.push(format!("meta orphan task `{task}` is not a well-formed task token"));
		}
		if !orphan_tasks.insert(task.as_str()) {
			problems.push(format!("meta orphan task `{task}` appears more than once"));
		}
		if slugs.contains(task.as_str()) {
			problems.push(format!(
				"meta orphan task `{task}` is also a step slug, so it is not orphan"
			));
		}
	}

	// Waiver integrity, per step. These rules are moved from the round log's
	// `check_record` waiver arm (the `increment`/`evidence` presence rules) and the
	// W5 `reason` <-> `evidence_tier` pairing. The pairing is single-sourced via
	// `WaiverReason::required_tier`; the presence rules are re-stated here byte-
	// faithfully because they run against these typed structs rather than the JSON
	// map `check_record` reads (one behaviour, two data representations).
	for step in &plan.steps {
		let step_increments: BTreeSet<&str> =
			step.increments.iter().map(|increment| increment.id.as_str()).collect();
		for waiver in &step.waivers {
			// `increment` presence is tied to `unit`: required (and one of the step's
			// increments) for an increment-unit waiver, forbidden for a step-unit one.
			match waiver.unit {
				WaiverUnit::Increment => match &waiver.increment {
					None => problems.push(format!(
						"waiver `{}` on step `{}` has unit `increment` but no `increment` field",
						waiver.id, step.slug
					)),
					Some(increment) if increment.is_empty() => problems.push(format!(
						"waiver `{}` on step `{}` has an empty `increment`",
						waiver.id, step.slug
					)),
					Some(increment) if !step_increments.contains(increment.as_str()) => problems
						.push(format!(
							"waiver `{}` on step `{}` names increment `{}`, which is not one of the step's increments",
							waiver.id, step.slug, increment
						)),
					Some(_) => {}
				},
				WaiverUnit::Step =>
					if waiver.increment.is_some() {
						problems.push(format!(
							"waiver `{}` on step `{}` has unit `step` but carries an `increment`",
							waiver.id, step.slug
						));
					},
			}
			// `evidence` presence is tied to `evidence_tier`: required for a
			// record-backed waiver, forbidden for a self-declared one.
			match waiver.evidence_tier {
				EvidenceTier::RecordBacked => match &waiver.evidence {
					None => problems.push(format!(
						"waiver `{}` on step `{}` is `record-backed` but has no `evidence`",
						waiver.id, step.slug
					)),
					Some(evidence) if evidence.is_empty() => problems.push(format!(
						"waiver `{}` on step `{}` has empty `evidence`",
						waiver.id, step.slug
					)),
					Some(_) => {}
				},
				EvidenceTier::SelfDeclared =>
					if waiver.evidence.is_some() {
						problems.push(format!(
							"waiver `{}` on step `{}` is `self-declared` but carries `evidence`",
							waiver.id, step.slug
						));
					},
			}
			// The `reason` must be paired with the tier its integrity requires
			// (single-sourced with W5 via `WaiverReason::required_tier`).
			if waiver.reason.required_tier() != waiver.evidence_tier {
				problems.push(format!(
					"waiver `{}` on step `{}` reason `{}` must not carry evidence tier `{}`",
					waiver.id,
					step.slug,
					waiver.reason.label(),
					waiver.evidence_tier.label()
				));
			}
		}
	}

	problems
}

#[cfg(test)]
mod tests {
	use super::*;

	/// The fixture skeleton: a well-formed `<task>.plan.toml` exercising every
	/// region, including nested `[[step.increment]]` and `[[step.waiver]]` entries.
	const SKELETON: &str = include_str!("testdata/skeleton.plan.toml");

	#[test]
	fn the_fixture_skeleton_parses_and_round_trips_including_nested_waivers() {
		let plan = parse_toml(SKELETON).expect("fixture parses");

		// The nested waiver entries are read into typed structs.
		let alpha = plan.steps.iter().find(|step| step.slug == "alpha").expect("alpha step");
		assert_eq!(alpha.status, StepStatus::Complete);
		assert_eq!(
			alpha.increments,
			vec![Increment {
				id: "alpha-inc1".to_string(),
				risk_class: RiskClass::LowRisk,
			}]
		);
		assert_eq!(
			alpha.waivers,
			vec![Waiver {
				id: "alpha-w1".to_string(),
				unit: WaiverUnit::Increment,
				increment: Some("alpha-inc1".to_string()),
				reason: WaiverReason::AcceptedAtEscalation,
				evidence_tier: EvidenceTier::RecordBacked,
				evidence: Some("alpha-inc1".to_string()),
				note: Some("Accepted below its streak at a human escalation.".to_string()),
			}]
		);
		let beta = plan.steps.iter().find(|step| step.slug == "beta").expect("beta step");
		assert_eq!(
			beta.waivers,
			vec![Waiver {
				id: "beta-w1".to_string(),
				unit: WaiverUnit::Step,
				increment: None,
				reason: WaiverReason::PredatesLogging,
				evidence_tier: EvidenceTier::SelfDeclared,
				evidence: None,
				note: None,
			}]
		);
		assert_eq!(plan.meta.primary, Primary::Toml);
		assert_eq!(plan.questions.len(), 4);
		assert_eq!(plan.principles.len(), 2);

		// Round-trip: serialising the parsed document and re-parsing yields an equal
		// document, so the schema is stable through a full write/read cycle.
		let reserialised = toml::to_string(&plan).expect("serialise");
		let reparsed = parse_toml(&reserialised).expect("re-parse");
		assert_eq!(plan, reparsed);
	}

	#[test]
	fn the_fixture_skeleton_validates_clean() {
		assert!(validate_source(SKELETON).is_empty(), "{:?}", validate_source(SKELETON));
	}

	/// Assert `validate_source(source)` flags at least one problem containing
	/// `needle`, returning nothing (the assertion is the point).
	fn assert_flags(
		source: &str,
		needle: &str,
	) {
		let problems = validate_source(source);
		assert!(
			problems.iter().any(|problem| problem.contains(needle)),
			"expected a problem containing `{needle}`, got {problems:?}"
		);
	}

	#[test]
	fn an_injected_bad_status_is_flagged() {
		// A step status outside the enum is rejected at parse (parse, don't validate),
		// surfaced by validate_source as the single malformed-source problem.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"done\"\norder = 1\n",
		);
		assert_flags(source, "malformed `<task>.plan.toml`");
	}

	#[test]
	fn a_dangling_blocked_by_is_flagged() {
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"blocked_by = [\"ghost\"]\n",
		);
		assert_flags(source, "blocked_by `ghost`, which is not a step");
	}

	#[test]
	fn a_decided_question_with_no_folded_into_is_flagged() {
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[question]]\nid = \"Q-1\"\nstatus = \"decided\"\nask = \"a\"\n",
		);
		assert_flags(source, "`Q-1` is `decided` but has no resolving `folded_into`");
	}

	#[test]
	fn a_record_backed_waiver_missing_evidence_is_flagged() {
		// `accepted-at-escalation` pairs correctly with `record-backed`, so ONLY the
		// missing-evidence presence rule fires.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.waiver]]\nid = \"w\"\nunit = \"step\"\n",
			"reason = \"accepted-at-escalation\"\nevidence_tier = \"record-backed\"\n",
		);
		assert_flags(source, "is `record-backed` but has no `evidence`");
	}

	#[test]
	fn a_self_declared_waiver_dressed_as_record_backed_is_flagged() {
		// `predates-logging` must be `self-declared`; dressing it as `record-backed`
		// (with an evidence pointer so the presence rule is satisfied) trips the
		// `reason` <-> `evidence_tier` pairing check.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.waiver]]\nid = \"w\"\nunit = \"step\"\n",
			"reason = \"predates-logging\"\nevidence_tier = \"record-backed\"\nevidence = \"x\"\n",
		);
		assert_flags(
			source,
			"reason `predates-logging` must not carry evidence tier `record-backed`",
		);
	}

	#[test]
	fn an_increment_waiver_naming_an_absent_increment_is_flagged() {
		// The increment-unit waiver names `a-inc2`, but the step only declares `a-inc1`.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.increment]]\nid = \"a-inc1\"\nrisk_class = \"risky\"\n",
			"[[step.waiver]]\nid = \"w\"\nunit = \"increment\"\nincrement = \"a-inc2\"\n",
			"reason = \"accepted-at-escalation\"\nevidence_tier = \"record-backed\"\nevidence = \"a-inc2\"\n",
		);
		assert_flags(source, "names increment `a-inc2`, which is not one of the step's increments");
	}

	#[test]
	fn a_duplicate_step_slug_is_flagged() {
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A2\"\nstatus = \"complete\"\norder = 2\n",
		);
		assert_flags(source, "step slug `a` appears more than once");
	}

	#[test]
	fn a_dangling_superseded_by_is_flagged() {
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[question]]\nid = \"Q-1\"\nstatus = \"superseded\"\nask = \"a\"\n",
			"superseded_by = \"Q-99\"\n",
		);
		assert_flags(source, "superseded_by `Q-99`, which is not a question");
	}

	#[test]
	fn a_traversal_sidecar_front_ref_is_flagged() {
		// A `..`-bearing front ref would escape the plan directory when render joins it,
		// reading a file outside the plan dir; validate rejects it at the boundary.
		let source =
			concat!("[meta]\ntitle = \"t\"\n", "[meta.sidecars]\nfront = [\"../outside.md\"]\n",);
		assert_flags(source, "meta sidecar front ref `../outside.md` must be a task-relative path");
	}

	#[test]
	fn an_absolute_sidecar_tail_ref_is_flagged() {
		// An absolute tail ref would discard the plan directory entirely when joined.
		let source =
			concat!("[meta]\ntitle = \"t\"\n", "[meta.sidecars]\ntail = \"/etc/passwd\"\n",);
		assert_flags(source, "meta sidecar tail ref `/etc/passwd` must be a task-relative path");
	}

	#[test]
	fn a_nested_traversal_sidecar_ref_is_flagged() {
		// A `..` component anywhere in the ref (not only at the front) is rejected.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[meta.sidecars]\nfront = [\"sub/../../escape.md\"]\n",
		);
		assert_flags(source, "must be a task-relative path");
	}

	#[test]
	fn a_task_relative_sidecar_ref_validates_clean() {
		// A plain task-relative ref (and a `./`-prefixed one) is safe and not flagged.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[meta.sidecars]\nfront = [\"intro.md\", \"./motivations.md\"]\ntail = \"success.md\"\n",
		);
		let problems = validate_source(source);
		assert!(
			!problems.iter().any(|problem| problem.contains("task-relative path")),
			"a task-relative ref must not be flagged: {problems:?}"
		);
	}

	#[test]
	fn primary_defaults_to_markdown_when_absent() {
		// When `[meta].primary` is absent the default is `markdown`, so the Markdown
		// parser stays the source (the live repo stays Markdown-sourced pre-cutover).
		let source = "[meta]\ntitle = \"t\"\n";
		let plan = parse_toml(source).expect("parses");
		assert_eq!(plan.meta.primary, Primary::Markdown);
	}

	#[test]
	fn a_non_decided_question_carrying_folded_into_is_flagged() {
		// `folded_into` is legal only on a `decided` question; an `open` one carrying
		// it is an illegal state.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"[[question]]\nid = \"Q-1\"\nstatus = \"open\"\nask = \"a\"\nfolded_into = \"a\"\n",
		);
		assert_flags(source, "`Q-1` is not `decided` but carries `folded_into`");
	}

	#[test]
	fn a_decided_question_with_a_dangling_folded_into_is_flagged() {
		// Proves the unconditional `folded_into` resolution: a `decided` question whose
		// `folded_into` names an absent step is flagged as a dangling cross-reference.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[question]]\nid = \"Q-1\"\nstatus = \"decided\"\nask = \"a\"\nfolded_into = \"ghost\"\n",
		);
		assert_flags(source, "`Q-1` folds into `ghost`, which is not a step");
	}

	#[test]
	fn a_non_decided_question_with_a_dangling_folded_into_is_flagged() {
		// `folded_into` now resolves regardless of status, so a dangling target on an
		// `exploring` question is caught (as well as the status-consistency violation).
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[question]]\nid = \"Q-1\"\nstatus = \"exploring\"\nask = \"a\"\nfolded_into = \"ghost\"\n",
		);
		assert_flags(source, "`Q-1` folds into `ghost`, which is not a step");
	}

	#[test]
	fn a_superseded_question_with_no_superseded_by_is_flagged() {
		// A `superseded` question must name the item that replaced it.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[question]]\nid = \"Q-1\"\nstatus = \"superseded\"\nask = \"a\"\n",
		);
		assert_flags(source, "`Q-1` is `superseded` but has no resolving `superseded_by`");
	}

	#[test]
	fn a_non_superseded_question_carrying_superseded_by_is_flagged() {
		// `superseded_by` is legal only on a `superseded` question.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[question]]\nid = \"Q-1\"\nstatus = \"open\"\nask = \"a\"\n",
			"[[question]]\nid = \"Q-2\"\nstatus = \"open\"\nask = \"b\"\nsuperseded_by = \"Q-1\"\n",
		);
		assert_flags(source, "`Q-2` is not `superseded` but carries `superseded_by`");
	}

	#[test]
	fn a_duplicate_waiver_id_is_flagged() {
		// Two waivers sharing an id would be an ambiguous cross-substrate join for
		// Inc 4, so a duplicate waiver id fails plan-wide like the other ids.
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.waiver]]\nid = \"dup\"\nunit = \"step\"\n",
			"reason = \"predates-logging\"\nevidence_tier = \"self-declared\"\n",
			"[[step]]\nslug = \"b\"\ntitle = \"B\"\nstatus = \"complete\"\norder = 2\n",
			"[[step.waiver]]\nid = \"dup\"\nunit = \"step\"\n",
			"reason = \"predates-logging\"\nevidence_tier = \"self-declared\"\n",
		);
		assert_flags(source, "waiver id `dup` appears more than once");
	}

	#[test]
	fn a_malformed_waiver_id_is_flagged() {
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.waiver]]\nid = \"-bad-\"\nunit = \"step\"\n",
			"reason = \"predates-logging\"\nevidence_tier = \"self-declared\"\n",
		);
		assert_flags(source, "waiver id `-bad-` is not a well-formed id");
	}

	#[test]
	fn a_duplicate_principle_n_is_flagged() {
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[principle]]\nn = 1\nname = \"One\"\ntext = \"first\"\n",
			"[[principle]]\nn = 1\nname = \"Also one\"\ntext = \"clash\"\n",
		);
		assert_flags(source, "principle `1` appears more than once");
	}

	#[test]
	fn a_self_blocking_step_is_flagged() {
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"blocked_by = [\"a\"]\n",
		);
		assert_flags(source, "step `a` is blocked_by itself");
	}

	#[test]
	fn a_self_folding_step_is_flagged() {
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"folds = [\"a\"]\n",
		);
		assert_flags(source, "step `a` folds itself");
	}

	#[test]
	fn a_duplicate_orphan_task_is_flagged() {
		let source = "[meta]\ntitle = \"t\"\norphan_tasks = [\"x\", \"x\"]\n";
		assert_flags(source, "meta orphan task `x` appears more than once");
	}

	#[test]
	fn an_orphan_task_equal_to_a_step_slug_is_flagged() {
		// An orphan token that names a declared step contradicts the field's own
		// definition (a task that owns no Roadmap step).
		let source = concat!(
			"[meta]\ntitle = \"t\"\norphan_tasks = [\"a\"]\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
		);
		assert_flags(source, "meta orphan task `a` is also a step slug");
	}

	#[test]
	fn an_uppercase_step_slug_is_flagged() {
		// Step slugs are lowercase kebab-case; an uppercase slug is rejected even
		// though it is otherwise a well-formed token (orphan tokens keep uppercase).
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"Alpha\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
		);
		assert_flags(source, "step slug `Alpha` is not a well-formed kebab-case id");
	}

	#[test]
	fn an_uppercase_increment_id_is_flagged() {
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.increment]]\nid = \"a-incA\"\nrisk_class = \"low_risk\"\n",
		);
		assert_flags(source, "increment id `a-incA` is not a well-formed kebab-case id");
	}

	#[test]
	fn a_typoed_plan_key_fails_to_parse() {
		// `blockd_by` is a typo for `blocked_by`; with `deny_unknown_fields` on the
		// plan.toml structs the schema rejects it at parse rather than silently
		// dropping the blocking edge (the O4 human decision: plan.toml is strict).
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"complete\"\norder = 1\n",
			"blockd_by = [\"ghost\"]\n",
		);
		assert!(parse_toml(source).is_err(), "a typoed key should fail to parse");
		assert_flags(source, "malformed `<task>.plan.toml`");
	}

	#[test]
	fn the_unexercised_status_variants_round_trip() {
		// The `not-started`, `skipped`, `optional`, `deferred` step statuses and the
		// `exploring` question status are otherwise unexercised: parse, validate clean,
		// and round-trip them (Principle 11).
		let source = concat!(
			"[meta]\ntitle = \"t\"\n",
			"[[step]]\nslug = \"a\"\ntitle = \"A\"\nstatus = \"not-started\"\norder = 1\n",
			"[[step]]\nslug = \"b\"\ntitle = \"B\"\nstatus = \"skipped\"\norder = 2\n",
			"[[step]]\nslug = \"c\"\ntitle = \"C\"\nstatus = \"optional\"\norder = 3\n",
			"[[step]]\nslug = \"d\"\ntitle = \"D\"\nstatus = \"deferred\"\norder = 4\n",
			"[[question]]\nid = \"Q-1\"\nstatus = \"exploring\"\nask = \"an exploring ask\"\n",
		);
		let plan = parse_toml(source).expect("parses");
		assert_eq!(plan.steps[0].status, StepStatus::NotStarted);
		assert_eq!(plan.steps[1].status, StepStatus::Skipped);
		assert_eq!(plan.steps[2].status, StepStatus::Optional);
		assert_eq!(plan.steps[3].status, StepStatus::Deferred);
		assert_eq!(plan.questions[0].status, QuestionStatus::Exploring);
		assert!(validate_source(source).is_empty(), "{:?}", validate_source(source));

		let reserialised = toml::to_string(&plan).expect("serialise");
		let reparsed = parse_toml(&reserialised).expect("re-parse");
		assert_eq!(plan, reparsed);
	}
}
