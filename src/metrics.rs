//! Metrics-log validation: check the workflow's calibration log against the
//! record schema pinned in `pack/instrument.md`.
//!
//! The `--instrument` scaffold has the orchestrator (an LLM) hand-write one JSON
//! object per line (JSON Lines) to `docs/metrics/workflow.jsonl`, so the records
//! are not guaranteed well-formed. This module parses each record and checks it
//! against the schema, reporting every malformed line by number and reason, so
//! the data is deterministically verifiable and bad lines are discardable rather
//! than silently corrupting calibration (Principle 5, illegal states caught;
//! Principle 12, fail loudly). Detection, not prevention: the scaffolded workflow
//! still writes the log directly, with no runtime dependency on this binary.

use serde_json::{
	Map,
	Value,
};

/// One malformed record, located by its 1-based line number in the log with a
/// human-readable reason (a parse error, a missing or wrong-typed field, or an
/// out-of-set enum value).
#[derive(Debug, PartialEq, Eq)]
pub struct LineError {
	/// 1-based line number of the offending record within the log.
	pub line: usize,
	/// Why the record is malformed, phrased for a person reading the log.
	pub reason: String,
}

/// Define an enum-valued field once: the variants and their exact on-disk
/// spellings live in a single place, so the valid set the validator checks and
/// the type that models it cannot drift apart (Principle 16, one source of
/// truth). `VARIANTS` is the ordered list of accepted strings (used verbatim in
/// error messages) and `parse` turns an accepted string into the typed variant.
macro_rules! enum_field {
	($(#[$meta:meta])* $vis:vis $name:ident { $($(#[$vmeta:meta])* $variant:ident => $text:literal),+ $(,)? }) => {
		$(#[$meta])*
		#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
		$vis enum $name {
			$($(#[$vmeta])* #[serde(rename = $text)] $variant),+
		}

		impl $name {
			/// The accepted on-disk spellings, in declaration order.
			const VARIANTS: &'static [&'static str] = &[$($text),+];

			/// Parse an accepted spelling into its variant, or `None` when the
			/// string is not one of the accepted set. `allow` (not `expect`) because
			/// enums whose value arrives already-typed (for example via serde on the
			/// TOML schema) reuse this macro for its `VARIANTS`/drift guard without
			/// ever calling `parse`, so it is legitimately unused for those.
			#[allow(dead_code)]
			$vis fn parse(text: &str) -> Option<Self> {
				match text {
					$($text => Some(Self::$variant),)+
					_ => None,
				}
			}
		}
	};
}

enum_field! {
	/// Which review phase a `round` record belongs to. `review` is a standalone
	/// human-invoked review-entry-mode pass, kept distinct from an implement run's
	/// internal `acceptance` gate so the two populations stay separable in
	/// calibration data.
	Phase {
		PlanReview => "plan_review",
		WorkReview => "work_review",
		Acceptance => "acceptance",
		Review => "review",
	}
}

enum_field! {
	/// The outcome of a review round. `clean` advances the artifact's
	/// consecutive-clean streak by one; `new_valid` (a round that produced a
	/// valid finding) resets it to zero. The cross-reference in `workflow.rs`
	/// reads this to check a step's convergence and the log's internal
	/// consistency, so it is exposed to the crate.
	pub(crate) RoundOutcome {
		Clean => "clean",
		NewValid => "new_valid",
	}
}

enum_field! {
	/// What the human did at a total-round-cap escalation. Exposed to the crate
	/// because `workflow.rs`'s W5 check joins a `record-backed` waiver's evidence
	/// against an escalation record's `human_decision`.
	pub(crate) HumanDecision {
		Decision => "decision",
		Resume => "resume",
	}
}

enum_field! {
	/// The result of a backstop dismissal re-check.
	RecheckResult {
		Upheld => "upheld",
		Overturned => "overturned",
	}
}

enum_field! {
	/// How a human interrupt was classified at intake.
	Classification {
		Trivial => "trivial",
		NonTrivial => "non_trivial",
	}
}

enum_field! {
	/// The convergence risk tier the orchestrator classified a review artifact
	/// into at loop-open, which sets how many clean rounds it takes to converge.
	/// Kept distinct from the intake `Classification` (`trivial`/`non_trivial`)
	/// because it is a different judgement about a different thing. Exposed to
	/// the crate because the `workflow.rs` cross-reference keys the required
	/// convergence streak off it.
	pub(crate) RiskClass {
		LowRisk => "low_risk",
		Risky => "risky",
	}
}

impl RiskClass {
	/// The consecutive-clean streak an artifact of this risk class must reach to
	/// converge: one clean round for `low_risk`, two for `risky`. This is the
	/// count `workflow.rs` checks a `complete` step's rounds against.
	pub(crate) fn required_streak(self) -> u64 {
		match self {
			RiskClass::LowRisk => 1,
			RiskClass::Risky => 2,
		}
	}

	/// The on-disk spelling of this risk class, for problem messages.
	pub(crate) fn label(self) -> &'static str {
		match self {
			RiskClass::LowRisk => "low_risk",
			RiskClass::Risky => "risky",
		}
	}
}

enum_field! {
	/// A finding severity on the four-level scale.
	Severity {
		Low => "low",
		Medium => "medium",
		High => "high",
		Critical => "critical",
	}
}

enum_field! {
	/// Which unit a `waiver` record exempts from the convergence bar: a whole
	/// Roadmap `step` (for a step that predates round-logging or whose review was
	/// skipped) or one `increment` of a step (for an increment accepted below its
	/// required streak). An increment waiver may be a self-declared
	/// `review-skipped`/`predates-logging` exemption OR a record-backed
	/// `accepted-at-escalation` one; the two tiers exist to stop a weak
	/// self-declaration being laundered into a strong claim, not to forbid
	/// self-declaration. Exposed to the crate because `workflow.rs`'s W3 refactor
	/// reads it to decide whether a waiver covers a step-with-no-records or a
	/// short-streak increment.
	pub(crate) WaiverUnit {
		Step => "step",
		Increment => "increment",
	}
}

enum_field! {
	/// Why a `waiver` exempts its unit from the convergence bar. Exposed to the crate
	/// because `workflow.rs`'s W5 check enforces the `reason` <-> `evidence_tier`
	/// pairing (`predates-logging` and `review-skipped` are self-declared;
	/// `accepted-at-escalation` is record-backed).
	pub(crate) WaiverReason {
		PredatesLogging => "predates-logging",
		ReviewSkipped => "review-skipped",
		AcceptedAtEscalation => "accepted-at-escalation",
	}
}

enum_field! {
	/// How strongly a `waiver` is evidenced: a `self-declared` waiver is the author's
	/// word, a `record-backed` waiver points at an independent `type:"escalation"`
	/// record. Kept as two tiers so a cleanup cannot launder a weak self-declaration
	/// into looking as strong as an independent escalation record; W5 enforces which
	/// reason may claim which tier. Exposed to the crate for the W5 join.
	pub(crate) EvidenceTier {
		SelfDeclared => "self-declared",
		RecordBacked => "record-backed",
	}
}

impl WaiverReason {
	/// Every variant in declaration order, iterated by any exhaustive consumer (for
	/// example the render engine's waiver-reason breakdown) so no variant is silently
	/// dropped from a hand-written literal (Principle 16, one source of truth). The
	/// `const _` cross-check below ties this array's length to the macro-generated
	/// `VARIANTS` list, so adding a variant is a compile error until it is listed here.
	pub(crate) const ALL: [WaiverReason; 3] = [
		WaiverReason::PredatesLogging,
		WaiverReason::ReviewSkipped,
		WaiverReason::AcceptedAtEscalation,
	];

	/// The on-disk spelling of this reason, for W5 problem messages.
	pub(crate) fn label(self) -> &'static str {
		match self {
			WaiverReason::PredatesLogging => "predates-logging",
			WaiverReason::ReviewSkipped => "review-skipped",
			WaiverReason::AcceptedAtEscalation => "accepted-at-escalation",
		}
	}

	/// The evidence tier this reason's integrity requires: `predates-logging` and
	/// `review-skipped` are self-declared (the author's word); `accepted-at-escalation`
	/// is record-backed (an independent escalation record). This is the single source
	/// of the `reason` <-> `evidence_tier` pairing rule that both W5 (the round-log
	/// waiver-integrity check) and `plan::source::validate_source` (the TOML waiver
	/// check) enforce, so the two cannot drift (Principle 16).
	pub(crate) fn required_tier(self) -> EvidenceTier {
		match self {
			WaiverReason::PredatesLogging | WaiverReason::ReviewSkipped =>
				EvidenceTier::SelfDeclared,
			WaiverReason::AcceptedAtEscalation => EvidenceTier::RecordBacked,
		}
	}
}

/// Compile-time exhaustiveness guard for `WaiverReason::ALL`: `VARIANTS` is generated
/// by the `enum_field!` macro from the full variant list, so a new variant grows it
/// and this equality fails to compile until the variant is added to `ALL` too. This
/// turns the C4 drift (a variant silently missing from the render breakdown) into a
/// build error rather than a stale hand-written array.
const _: () = assert!(WaiverReason::ALL.len() == WaiverReason::VARIANTS.len());

impl EvidenceTier {
	/// The on-disk spelling of this evidence tier, for W5 problem messages.
	pub(crate) fn label(self) -> &'static str {
		match self {
			EvidenceTier::SelfDeclared => "self-declared",
			EvidenceTier::RecordBacked => "record-backed",
		}
	}
}

/// Fetch a required string field: absent is a missing-field error, present but
/// not a JSON string is a wrong-type error.
fn require_str<'a>(
	obj: &'a Map<String, Value>,
	name: &str,
) -> Result<&'a str, String> {
	let value = obj.get(name).ok_or_else(|| format!("missing field `{name}`"))?;
	value.as_str().ok_or_else(|| format!("field `{name}` has wrong type (expected string)"))
}

/// Check a required boolean field is present and a JSON boolean.
fn require_bool(
	obj: &Map<String, Value>,
	name: &str,
) -> Result<(), String> {
	let value = obj.get(name).ok_or_else(|| format!("missing field `{name}`"))?;
	value
		.as_bool()
		.map(|_| ())
		.ok_or_else(|| format!("field `{name}` has wrong type (expected boolean)"))
}

/// Check a required count field is present, a JSON number, and a non-negative
/// integer (so a negative or fractional value is rejected, not silently
/// truncated).
fn require_count(
	obj: &Map<String, Value>,
	name: &str,
) -> Result<(), String> {
	let value = obj.get(name).ok_or_else(|| format!("missing field `{name}`"))?;
	match value {
		Value::Number(number) => number.as_u64().map(|_| ()).ok_or_else(|| {
			format!("field `{name}` value `{number}` is not a non-negative integer")
		}),
		_ => Err(format!("field `{name}` has wrong type (expected non-negative integer)")),
	}
}

/// Check a required enum-valued field is present, a JSON string, and one of the
/// field type's accepted spellings.
fn require_enum(
	obj: &Map<String, Value>,
	name: &str,
	variants: &[&str],
	parse: impl Fn(&str) -> bool,
) -> Result<(), String> {
	let text = require_str(obj, name)?;
	if parse(text) {
		Ok(())
	} else {
		Err(format!("field `{name}` value `{text}` not one of [{}]", variants.join(", ")))
	}
}

/// Check the `round` record's `severities`: a required array whose every element
/// is a string naming an accepted severity.
fn require_severities(
	obj: &Map<String, Value>,
	name: &str,
) -> Result<(), String> {
	let value = obj.get(name).ok_or_else(|| format!("missing field `{name}`"))?;
	let array = value
		.as_array()
		.ok_or_else(|| format!("field `{name}` has wrong type (expected array)"))?;
	for (index, element) in array.iter().enumerate() {
		let text = element
			.as_str()
			.ok_or_else(|| format!("field `{name}`[{index}] has wrong type (expected string)"))?;
		if Severity::parse(text).is_none() {
			return Err(format!(
				"field `{name}`[{index}] value `{text}` not one of [{}]",
				Severity::VARIANTS.join(", ")
			));
		}
	}
	Ok(())
}

/// Check the `round` record's optional `reviewers` attribution: a JSON array
/// whose every element is an object carrying a string `role` and `model` plus
/// non-negative integer `raw_findings` and `valid_findings` counts, and an
/// optional string `harness` naming the CLI the reviewer ran on. This is the
/// per-reviewer breakdown used to calibrate reviewer productivity and whether
/// running multiple models or harnesses earns its cost; the caller only invokes it when the
/// field is present, since it is optional.
fn require_reviewers(
	obj: &Map<String, Value>,
	name: &str,
) -> Result<(), String> {
	let value = obj.get(name).ok_or_else(|| format!("missing field `{name}`"))?;
	let array = value
		.as_array()
		.ok_or_else(|| format!("field `{name}` has wrong type (expected array)"))?;
	// A present-but-empty array cannot describe a real round (every review round
	// has at least one reviewer); a round with no attribution omits the field
	// entirely via the optional path, so an empty array is a malformed record.
	if array.is_empty() {
		return Err(format!("field `{name}` is empty"));
	}
	for (index, element) in array.iter().enumerate() {
		let entry = element
			.as_object()
			.ok_or_else(|| format!("field `{name}`[{index}] has wrong type (expected object)"))?;
		// Reuse the scalar checkers, prefixing their message with the array
		// position so the line-level error still points at the offending element.
		let at = |result: Result<(), String>| {
			result.map_err(|reason| format!("field `{name}`[{index}]: {reason}"))
		};
		at(require_str(entry, "role").map(|_| ()))?;
		at(require_str(entry, "model").map(|_| ()))?;
		at(require_count(entry, "raw_findings"))?;
		at(require_count(entry, "valid_findings"))?;
		// The reviewer's harness (the CLI it ran on, for example "claude-code")
		// is optional, validated as a string only when present so an entry that
		// records only the model still validates.
		if entry.contains_key("harness") {
			at(require_str(entry, "harness").map(|_| ()))?;
		}
	}
	Ok(())
}

/// Check a `round`/`escalation` record's OPTIONAL structured ids (Inc 2): the
/// `step` slug and `increment` id that let W3/W5 join without the lexical
/// `leading_slug` strip. Each is optional (a pre-migration record omits both and
/// falls back to the shim), but when present it must be a non-empty string, so a
/// blank id can never masquerade as a real join key. Absent fields are fine.
fn require_structured_ids(obj: &Map<String, Value>) -> Result<(), String> {
	for name in ["step", "increment"] {
		if obj.contains_key(name) && require_str(obj, name)?.is_empty() {
			return Err(format!("field `{name}` is empty"));
		}
	}
	Ok(())
}

/// Check the `decision` record's `options`: a required NON-EMPTY array whose
/// every element is a string. Modelled on `require_severities` minus the enum
/// step (an option label is any string, not a member of a fixed set). Returns the
/// option strings on success so the caller can check `chosen` membership against
/// them without re-reading the field.
fn require_options<'a>(
	obj: &'a Map<String, Value>,
	name: &str,
) -> Result<Vec<&'a str>, String> {
	let value = obj.get(name).ok_or_else(|| format!("missing field `{name}`"))?;
	let array = value
		.as_array()
		.ok_or_else(|| format!("field `{name}` has wrong type (expected array)"))?;
	// A present-but-empty array cannot describe a real decision (a choice needs at
	// least one option to choose from), so an empty array is a malformed record.
	if array.is_empty() {
		return Err(format!("field `{name}` is empty"));
	}
	let mut options = Vec::with_capacity(array.len());
	for (index, element) in array.iter().enumerate() {
		let text = element
			.as_str()
			.ok_or_else(|| format!("field `{name}`[{index}] has wrong type (expected string)"))?;
		options.push(text);
	}
	Ok(options)
}

/// The numeric index of an Open-Questions id (`Q-<n>` -> `n`), or `None` when the
/// id is not the `Q-<n>` shape. Shared by the `baseline` schema check, the
/// `parse_baseline` projection, and `workflow.rs`'s W4 check so all three agree on
/// what a cutoff id looks like (the historical `OQ-<letter>` provenance markers do
/// not parse and are rejected as a cutoff). W4 compares a decided item's index
/// against the declared cutoff index; both operands come from this one parser so
/// they cannot drift.
pub(crate) fn question_id_index(id: &str) -> Option<u64> {
	id.strip_prefix("Q-").and_then(|digits| digits.parse::<u64>().ok())
}

/// Check one already-parsed record against the schema, returning the first
/// schema violation as a reason string, or `Ok(())` when the record is valid.
/// Unknown extra fields are permitted (forward-compatible); only a missing
/// required field, a wrong JSON type, or an out-of-set enum value is an error.
fn check_record(value: &Value) -> Result<(), String> {
	let obj = value.as_object().ok_or_else(|| "record is not a JSON object".to_string())?;

	// Common fields on every record: a required `type` and `task`, plus an
	// optional `ts` timestamp that, when present, must be a string.
	let record_type = require_str(obj, "type")?;
	require_str(obj, "task")?;
	if obj.contains_key("ts") {
		require_str(obj, "ts")?;
	}

	match record_type {
		"round" => {
			require_str(obj, "artifact")?;
			require_enum(obj, "phase", Phase::VARIANTS, |text| Phase::parse(text).is_some())?;
			require_bool(obj, "changed_since_prev")?;
			require_enum(obj, "outcome", RoundOutcome::VARIANTS, |text| {
				RoundOutcome::parse(text).is_some()
			})?;
			require_count(obj, "valid_findings")?;
			require_severities(obj, "severities")?;
			require_count(obj, "consecutive_clean")?;
			// The artifact's convergence risk tier at loop-open is required: it
			// sets how many clean rounds the round takes to converge, so a round
			// record without it cannot be checked against the convergence rule.
			require_enum(obj, "risk_class", RiskClass::VARIANTS, |text| {
				RiskClass::parse(text).is_some()
			})?;
			// The per-reviewer attribution stays optional, validated only when
			// present so a round with no attribution omits it rather than writing
			// an empty array.
			if obj.contains_key("reviewers") {
				require_reviewers(obj, "reviewers")?;
			}
			// The structured step slug and increment id (Inc 2) are optional: a
			// pre-migration round omits them and joins via `leading_slug`. When
			// present each must be a non-empty string, so a blank id can never
			// masquerade as a real join key.
			require_structured_ids(obj)?;
		}
		"escalation" => {
			// The `task` a record-backed waiver's `evidence` joins to must be non-empty,
			// so an empty-`task` escalation can never satisfy the W5 join.
			if require_str(obj, "task")?.is_empty() {
				return Err("field `task` is empty".to_string());
			}
			require_str(obj, "artifact")?;
			require_enum(obj, "human_decision", HumanDecision::VARIANTS, |text| {
				HumanDecision::parse(text).is_some()
			})?;
			// The same optional structured ids as `round` (Inc 2); non-empty when
			// present so W5's unit-scope join never keys off a blank id.
			require_structured_ids(obj)?;
		}
		"dismissal_recheck" => {
			require_str(obj, "artifact")?;
			require_enum(obj, "result", RecheckResult::VARIANTS, |text| {
				RecheckResult::parse(text).is_some()
			})?;
		}
		"intake" => {
			require_enum(obj, "classification", Classification::VARIANTS, |text| {
				Classification::parse(text).is_some()
			})?;
			require_bool(obj, "replanned")?;
		}
		"decision" => {
			// A human-decision receipt: the question id it decides, the option labels
			// presented, the recommendation, and the human's choice. The one genuinely
			// new cross-field constraint is that `chosen` names one of the presented
			// `options` (choosing Y from {X, Y, Z} inherently records that {X, Y, Z}
			// was shown), which is what makes the receipt evidence rather than a
			// self-certified flag.
			require_str(obj, "q_id")?;
			let options = require_options(obj, "options")?;
			require_str(obj, "recommendation")?;
			let chosen = require_str(obj, "chosen")?;
			if !options.contains(&chosen) {
				return Err(format!(
					"field `chosen` value `{chosen}` is not one of the presented `options` [{}]",
					options.join(", ")
				));
			}
		}
		"baseline" => {
			// A declared historical-exemption cutoff, written once when a repo with
			// pre-existing decisions adopts the mechanism. `questions_through` names the
			// highest decided Open-Questions item that predates the decision-receipt
			// mechanism, so W4 exempts every decided item at or below it and requires a
			// receipt only for items strictly after it. Enforcing the `Q-<n>` shape here
			// (strict validation) keeps the best-effort `parse_baseline` projection and
			// this check agreeing on what a cutoff id is.
			//
			// The `baseline` type serves W4 only (the `questions_through` cutoff); W3's
			// historical exemption is carried by per-unit `type:"waiver"` records, not a
			// cutoff on this type. Only the field this arm consumes is constrained, and
			// unknown extra fields stay permitted (see the doc comment above).
			let cutoff = require_str(obj, "questions_through")?;
			if question_id_index(cutoff).is_none() {
				return Err(format!(
					"field `questions_through` value `{cutoff}` is not a `Q-<n>` id"
				));
			}
		}
		"waiver" => {
			// An authorised waiver of the convergence bar for one unit. The two
			// cross-field constraints are what keep it honest: the `increment` field is
			// present exactly for an `increment`-unit waiver (a step-unit waiver names no
			// increment), and the `evidence` pointer is present exactly for a
			// `record-backed` waiver (a self-declared waiver carries no pointer). A
			// malformed waiver is reported HERE so it cannot silently grant an exemption
			// while the best-effort `parse_waivers` drops it. The `reason` <-> tier
			// pairing (which reason may claim which tier) is W5's job, not this schema
			// check's, so it is not enforced here.
			let unit = require_str(obj, "unit")?;
			let unit_variant = WaiverUnit::parse(unit).ok_or_else(|| {
				format!(
					"field `unit` value `{unit}` not one of [{}]",
					WaiverUnit::VARIANTS.join(", ")
				)
			})?;
			let step = require_str(obj, "step")?;
			if step.is_empty() {
				return Err("field `step` is empty".to_string());
			}
			require_enum(obj, "reason", WaiverReason::VARIANTS, |text| {
				WaiverReason::parse(text).is_some()
			})?;
			let tier = require_str(obj, "evidence_tier")?;
			let tier_variant = EvidenceTier::parse(tier).ok_or_else(|| {
				format!(
					"field `evidence_tier` value `{tier}` not one of [{}]",
					EvidenceTier::VARIANTS.join(", ")
				)
			})?;
			// `increment` presence is tied to `unit`: required (and non-empty) for an
			// increment-unit waiver, forbidden for a step-unit one.
			match unit_variant {
				WaiverUnit::Increment => {
					let increment = require_str(obj, "increment")?;
					if increment.is_empty() {
						return Err("field `increment` is empty".to_string());
					}
				}
				WaiverUnit::Step =>
					if obj.contains_key("increment") {
						return Err(
							"field `increment` is forbidden when `unit` is `step`".to_string()
						);
					},
			}
			// `evidence` presence is tied to `evidence_tier`: required for a
			// record-backed waiver, forbidden for a self-declared one.
			match tier_variant {
				EvidenceTier::RecordBacked =>
					if require_str(obj, "evidence")?.is_empty() {
						return Err("field `evidence` is empty".to_string());
					},
				EvidenceTier::SelfDeclared =>
					if obj.contains_key("evidence") {
						return Err(
							"field `evidence` is forbidden when `evidence_tier` is `self-declared`"
								.to_string(),
						);
					},
			}
		}
		other => return Err(format!("unknown `type` `{other}`")),
	}
	Ok(())
}

/// The number of records in `contents`: non-blank lines, matching the count
/// `validate_log` checks, so a caller reporting "N records" uses the same
/// definition of a record.
pub fn count_records(contents: &str) -> usize {
	contents.lines().filter(|line| !line.trim().is_empty()).count()
}

/// The workflow-relevant projection of one `round` record: the fields the
/// `workflow.rs` cross-reference reads (the log's other record types and the
/// round's calibration-only fields are irrelevant there). Parsed by
/// `parse_rounds`, which reuses this module's schema knowledge rather than
/// introducing a second parser (`workflow.rs` owns no JSON parsing).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Round {
	/// 1-based line number of the record within the log, for problem messages.
	pub(crate) line: usize,
	/// The `task` value verbatim (the leading slug plus an optional `-inc<x>`
	/// increment suffix); when the structured `step`/`increment` ids below are
	/// absent, the join and the increment grouping key off it (the latter
	/// unstripped) via the `leading_slug` shim.
	pub(crate) task: String,
	/// The review artifact this round covers. The convergence streak spans the
	/// different artifacts one increment's rounds name, so it is counted per
	/// increment (the structured `increment` id, or the `task` when it is
	/// absent), not per artifact.
	pub(crate) artifact: String,
	/// Whether the round was clean or produced a new valid finding.
	pub(crate) outcome: RoundOutcome,
	/// The logged consecutive-clean streak after this round.
	pub(crate) consecutive_clean: u64,
	/// The artifact's risk class, which sets the required convergence streak.
	pub(crate) risk_class: RiskClass,
	/// The structured Roadmap step slug this round's increment belongs to, when
	/// the record carries it (records written from Inc 2 onward). When present,
	/// `workflow.rs`'s W3 joins the round to its step on this slug DIRECTLY,
	/// instead of lexically stripping `task` via `leading_slug`, retiring the
	/// SE-10/B6 over-strip risk (T3) for new data. Absent (`None`) on
	/// pre-migration records, which fall back to `leading_slug(task)`.
	pub(crate) step: Option<String>,
	/// The structured increment id this round covers, when the record carries it.
	/// When present, W3 groups the round into its increment on this id instead of
	/// the raw `task` string; absent (`None`) on pre-migration records, which fall
	/// back to `task`. Inc 4 joins this to the TOML `[[step.increment]].id`.
	pub(crate) increment: Option<String>,
}

/// Project every well-formed `round` record in `contents` into a `Round`,
/// preserving file order. A line that is blank, not JSON, not a `round`, or a
/// `round` missing one of the projected fields is skipped here: schema
/// violations are the job of `validate_log`, which reports them, so this
/// projection is a best-effort read for the cross-reference and does not
/// re-report them. Non-`round` record types (`intake`, `escalation`,
/// `dismissal_recheck`) carry no convergence data and are skipped.
pub(crate) fn parse_rounds(contents: &str) -> Vec<Round> {
	let mut rounds = Vec::new();
	for (index, line) in contents.lines().enumerate() {
		if line.trim().is_empty() {
			continue;
		}
		let Ok(value) = serde_json::from_str::<Value>(line) else {
			continue;
		};
		let Some(obj) = value.as_object() else {
			continue;
		};
		if obj.get("type").and_then(Value::as_str) != Some("round") {
			continue;
		}
		let (Some(task), Some(artifact)) =
			(obj.get("task").and_then(Value::as_str), obj.get("artifact").and_then(Value::as_str))
		else {
			continue;
		};
		let Some(outcome) =
			obj.get("outcome").and_then(Value::as_str).and_then(RoundOutcome::parse)
		else {
			continue;
		};
		let Some(consecutive_clean) = obj.get("consecutive_clean").and_then(Value::as_u64) else {
			continue;
		};
		let Some(risk_class) =
			obj.get("risk_class").and_then(Value::as_str).and_then(RiskClass::parse)
		else {
			continue;
		};
		// The structured step slug and increment id are optional (present only on
		// records written from Inc 2 onward); an empty string is treated as absent
		// so a blank field falls back to the `leading_slug`/`task` shim rather than
		// joining to an empty slug.
		let step = obj.get("step").and_then(Value::as_str).filter(|s| !s.is_empty());
		let increment = obj.get("increment").and_then(Value::as_str).filter(|s| !s.is_empty());
		rounds.push(Round {
			line: index + 1,
			task: task.to_string(),
			artifact: artifact.to_string(),
			outcome,
			consecutive_clean,
			risk_class,
			step: step.map(str::to_string),
			increment: increment.map(str::to_string),
		});
	}
	rounds
}

/// The workflow-relevant projection of one `decision` record: the fields the
/// `workflow.rs` W4 cross-reference reads. A decision record carries more (the
/// options, the recommendation, the choice), but W4 only needs the question id it
/// decides and the record's line, so the projection is deliberately narrow,
/// parallel to `Round`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Decision {
	/// 1-based line number of the record within the log, for problem messages.
	pub(crate) line: usize,
	/// The Open-Questions id this receipt decides (for example `Q-44`).
	pub(crate) q_id: String,
}

/// Project every well-formed `decision` record in `contents` into a `Decision`,
/// preserving file order. A line that is blank, not JSON, not a `decision`, or a
/// `decision` missing its projected `q_id` is skipped here: schema violations are
/// the job of `validate_log`, which reports them, so this projection is a
/// best-effort read for the W4 cross-reference and does not re-report them
/// (parallel to `parse_rounds`).
pub(crate) fn parse_decisions(contents: &str) -> Vec<Decision> {
	let mut decisions = Vec::new();
	for (index, line) in contents.lines().enumerate() {
		if line.trim().is_empty() {
			continue;
		}
		let Ok(value) = serde_json::from_str::<Value>(line) else {
			continue;
		};
		let Some(obj) = value.as_object() else {
			continue;
		};
		if obj.get("type").and_then(Value::as_str) != Some("decision") {
			continue;
		}
		let Some(q_id) = obj.get("q_id").and_then(Value::as_str) else {
			continue;
		};
		decisions.push(Decision {
			line: index + 1,
			q_id: q_id.to_string(),
		});
	}
	decisions
}

/// The workflow-relevant projection of one `baseline` record: the declared
/// decided-question cutoff W4 consults for its historical exemption. The `baseline`
/// type serves W4 only; W3's historical exemption is carried by per-unit
/// `type:"waiver"` records, not a cutoff on this type. The projection is
/// deliberately narrow (only the cutoff index W4 needs), parallel to `Decision`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Baseline {
	/// 1-based line number of the record within the log, for problem messages.
	pub(crate) line: usize,
	/// The decided-question cutoff index (`Q-<n>` -> `n`): every decided item at or
	/// below it predates the mechanism and is exempt from W4.
	pub(crate) questions_through: u64,
}

/// Project every well-formed `baseline` record in `contents` into a `Baseline`,
/// preserving file order. A line that is blank, not JSON, not a `baseline`, or a
/// `baseline` whose `questions_through` is missing or not a `Q-<n>` id is skipped
/// here: schema violations are `validate_log`'s job, so this is a best-effort read
/// for the W4 cross-reference (parallel to `parse_decisions`). A malformed cutoff
/// is therefore treated as NO declared exemption rather than a silent pass, the
/// safe direction (W4 then requires receipts, so a broken baseline cannot silently
/// exempt anything).
///
/// Multiple `baseline` records resolve LAST-ONE-WINS: the caller takes the last
/// element (`.last()`), so a later declaration in file order supersedes an earlier
/// one. This is deterministic and lets a repo re-declare its cutoff by appending a
/// new record rather than editing a past line (the log is append-only).
pub(crate) fn parse_baseline(contents: &str) -> Vec<Baseline> {
	let mut baselines = Vec::new();
	for (index, line) in contents.lines().enumerate() {
		if line.trim().is_empty() {
			continue;
		}
		let Ok(value) = serde_json::from_str::<Value>(line) else {
			continue;
		};
		let Some(obj) = value.as_object() else {
			continue;
		};
		if obj.get("type").and_then(Value::as_str) != Some("baseline") {
			continue;
		}
		let Some(questions_through) =
			obj.get("questions_through").and_then(Value::as_str).and_then(question_id_index)
		else {
			continue;
		};
		baselines.push(Baseline {
			line: index + 1,
			questions_through,
		});
	}
	baselines
}

/// The workflow-relevant projection of one `waiver` record: an authorised
/// exemption from the convergence bar that `workflow.rs`'s W3 (does a covering
/// waiver exist?) and W5 (is the waiver well-evidenced?) read. Projected by
/// `parse_waivers`, parallel to `Decision`/`Baseline`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Waiver {
	/// A substrate-appropriate locator naming this waiver in a W5 problem message:
	/// for a JSONL waiver its 1-based log line (`round log line N`), for a TOML
	/// waiver its `[[step.waiver]].id`. Each parser fills it correctly for its own
	/// substrate, so a W5 message points a reader at the real record rather than
	/// asserting a false JSONL line for a TOML waiver. W5 prints it verbatim.
	pub(crate) locator: String,
	/// Whether the waiver covers a whole `step` or one `increment` of it.
	pub(crate) unit: WaiverUnit,
	/// The Roadmap step slug the waiver covers (W5 checks it names a real step).
	pub(crate) step: String,
	/// The increment token this waiver covers, present exactly for an
	/// `increment`-unit waiver (matched against a round's full `task` in W3).
	pub(crate) increment: Option<String>,
	/// Why the waiver exempts its unit (W5 pairs it against the evidence tier).
	pub(crate) reason: WaiverReason,
	/// How strongly the waiver is evidenced (W5 pairs it against the reason and,
	/// for the record-backed tier, joins the evidence against an escalation record).
	pub(crate) evidence_tier: EvidenceTier,
	/// The backing-record pointer, present exactly for a `record-backed` waiver;
	/// for an `accepted-at-escalation` waiver it names the escalation record by its
	/// `task`.
	pub(crate) evidence: Option<String>,
}

/// Project every well-formed `waiver` record in `contents` into a `Waiver`,
/// preserving file order. Best-effort, parallel to `parse_baseline`: a line that is
/// blank, not JSON, not a `waiver`, or a waiver that violates the schema (a bad
/// enum value, a missing `step`, or an `increment`/`evidence` presence rule broken)
/// is skipped here. Schema violations are `validate_log`'s job to REPORT, so a
/// malformed waiver is both reported there and DROPPED here, which is the load-
/// bearing property: a malformed waiver can never silently grant an exemption,
/// because W3 only ever sees the well-formed ones this projects.
pub(crate) fn parse_waivers(contents: &str) -> Vec<Waiver> {
	let mut waivers = Vec::new();
	for (index, line) in contents.lines().enumerate() {
		if line.trim().is_empty() {
			continue;
		}
		let Ok(value) = serde_json::from_str::<Value>(line) else {
			continue;
		};
		let Some(obj) = value.as_object() else {
			continue;
		};
		if obj.get("type").and_then(Value::as_str) != Some("waiver") {
			continue;
		}
		let Some(unit) = obj.get("unit").and_then(Value::as_str).and_then(WaiverUnit::parse) else {
			continue;
		};
		let Some(step) = obj.get("step").and_then(Value::as_str).filter(|s| !s.is_empty()) else {
			continue;
		};
		let Some(reason) = obj.get("reason").and_then(Value::as_str).and_then(WaiverReason::parse)
		else {
			continue;
		};
		let Some(evidence_tier) =
			obj.get("evidence_tier").and_then(Value::as_str).and_then(EvidenceTier::parse)
		else {
			continue;
		};
		// The `increment` presence rule (present and non-empty iff the unit is an
		// increment) is enforced here too, so a waiver that breaks it is dropped rather
		// than projected as a mis-scoped exemption.
		let increment = obj.get("increment").and_then(Value::as_str).filter(|s| !s.is_empty());
		let increment = match (unit, increment) {
			(WaiverUnit::Increment, Some(token)) => Some(token.to_string()),
			(WaiverUnit::Increment, None) => continue,
			(WaiverUnit::Step, None) => None,
			(WaiverUnit::Step, Some(_)) => continue,
		};
		// The `evidence` presence rule (present iff record-backed) is likewise enforced
		// here, so a laundered or missing pointer drops the waiver instead of exempting.
		let evidence = obj.get("evidence").and_then(Value::as_str).filter(|s| !s.is_empty());
		let evidence = match (evidence_tier, evidence) {
			(EvidenceTier::RecordBacked, Some(pointer)) => Some(pointer.to_string()),
			(EvidenceTier::RecordBacked, None) => continue,
			(EvidenceTier::SelfDeclared, None) => None,
			(EvidenceTier::SelfDeclared, Some(_)) => continue,
		};
		waivers.push(Waiver {
			locator: format!("round log line {}", index + 1),
			unit,
			step: step.to_string(),
			increment,
			reason,
			evidence_tier,
			evidence,
		});
	}
	waivers
}

/// The workflow-relevant projection of one `escalation` record: the fields
/// `workflow.rs`'s W5 join reads. The full `escalation` schema is validated by
/// `check_record`; this narrow projection only carries the `task` a record-backed
/// waiver's `evidence` points at and the `human_decision` W5 requires to be a
/// `decision`, parallel to the other `parse_*` projections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Escalation {
	/// 1-based line number of the record within the log, for problem messages.
	pub(crate) line: usize,
	/// The `task` value verbatim; a record-backed waiver's `evidence` joins to it,
	/// and, when the structured ids below are absent, W5's unit-scope check keys
	/// off it via the `leading_slug` shim.
	pub(crate) task: String,
	/// What the human did at the escalation; W5 requires a `decision`.
	pub(crate) human_decision: HumanDecision,
	/// The structured Roadmap step slug this escalation belongs to, when the
	/// record carries it (records written from Inc 2 onward). When present, W5's
	/// step-unit scope check joins on this slug DIRECTLY instead of stripping
	/// `task` via `leading_slug`; absent (`None`) on pre-migration records, which
	/// fall back to `leading_slug(task)`.
	pub(crate) step: Option<String>,
	/// The structured increment id this escalation belongs to, when the record
	/// carries it. When present, W5's increment-unit scope check joins on this id
	/// instead of the raw `task`; absent (`None`) on pre-migration records, which
	/// fall back to `task`. Inc 4 joins this to the TOML `[[step.increment]].id`.
	pub(crate) increment: Option<String>,
}

/// Project every well-formed `escalation` record in `contents` into an
/// `Escalation`, preserving file order. Best-effort, parallel to `parse_rounds`: a
/// line that is blank, not JSON, not an `escalation`, or an `escalation` missing one
/// of the projected fields is skipped (its reporting is `validate_log`'s job).
pub(crate) fn parse_escalations(contents: &str) -> Vec<Escalation> {
	let mut escalations = Vec::new();
	for (index, line) in contents.lines().enumerate() {
		if line.trim().is_empty() {
			continue;
		}
		let Ok(value) = serde_json::from_str::<Value>(line) else {
			continue;
		};
		let Some(obj) = value.as_object() else {
			continue;
		};
		if obj.get("type").and_then(Value::as_str) != Some("escalation") {
			continue;
		}
		let Some(task) = obj.get("task").and_then(Value::as_str).filter(|s| !s.is_empty()) else {
			continue;
		};
		let Some(human_decision) =
			obj.get("human_decision").and_then(Value::as_str).and_then(HumanDecision::parse)
		else {
			continue;
		};
		// The structured step slug and increment id are optional (present only on
		// records written from Inc 2 onward); an empty string is treated as absent
		// so a blank field falls back to the `leading_slug`/`task` shim.
		let step = obj.get("step").and_then(Value::as_str).filter(|s| !s.is_empty());
		let increment = obj.get("increment").and_then(Value::as_str).filter(|s| !s.is_empty());
		escalations.push(Escalation {
			line: index + 1,
			task: task.to_string(),
			human_decision,
			step: step.map(str::to_string),
			increment: increment.map(str::to_string),
		});
	}
	escalations
}

/// Validate a JSONL metrics log, returning one `LineError` per malformed record.
/// Blank lines are ignored; every other line is parsed as JSON and checked
/// against the record schema. An empty result means the log is valid. Line
/// numbers are 1-based and count blank lines, so they match the file as a person
/// or editor sees it.
pub fn validate_log(contents: &str) -> Vec<LineError> {
	let mut errors = Vec::new();
	for (index, line) in contents.lines().enumerate() {
		if line.trim().is_empty() {
			continue;
		}
		let reason = match serde_json::from_str::<Value>(line) {
			Ok(value) => check_record(&value).err(),
			Err(error) => Some(format!("invalid JSON: {error}")),
		};
		if let Some(reason) = reason {
			errors.push(LineError {
				line: index + 1,
				reason,
			});
		}
	}
	errors
}

#[cfg(test)]
mod tests {
	use super::*;

	/// A valid log with one record of each type, exercising the optional `ts`
	/// field and an unknown extra field (both accepted).
	const VALID_LOG: &str = concat!(
		r#"{"type":"round","task":"demo","artifact":"AGENTS.md","phase":"plan_review","changed_since_prev":true,"outcome":"new_valid","valid_findings":3,"severities":["high","low"],"consecutive_clean":0,"risk_class":"risky","reviewers":[{"role":"reviewer","model":"opus","raw_findings":4,"valid_findings":2},{"role":"reviewer","model":"sonnet","raw_findings":3,"valid_findings":1}],"ts":"2026-07-15T12:00:00Z"}"#,
		"\n",
		r#"{"type":"escalation","task":"demo","artifact":"AGENTS.md","human_decision":"resume","note":"extra field ok"}"#,
		"\n",
		r#"{"type":"dismissal_recheck","task":"demo","artifact":"AGENTS.md","result":"upheld"}"#,
		"\n",
		r#"{"type":"intake","task":"demo","classification":"non_trivial","replanned":false}"#,
		"\n",
	);

	#[test]
	fn a_fully_valid_multi_record_log_passes() {
		assert_eq!(validate_log(VALID_LOG), Vec::new());
		assert_eq!(count_records(VALID_LOG), 4);
	}

	#[test]
	fn blank_lines_are_ignored_and_do_not_shift_line_numbers() {
		// Blank and whitespace-only lines are skipped but still counted, so the
		// error on line 4 keeps its true 1-based position.
		let log = concat!(
			"\n",
			"   \n",
			r#"{"type":"intake","task":"demo","classification":"trivial","replanned":false}"#,
			"\n",
			r#"{"type":"intake","task":"demo","replanned":false}"#,
			"\n",
		);
		let errors = validate_log(log);
		assert_eq!(errors.len(), 1);
		assert_eq!(errors[0].line, 4);
		assert!(
			errors[0].reason.contains("missing field `classification`"),
			"{}",
			errors[0].reason
		);
	}

	/// Validate a single-line log and return the first error's reason.
	fn one_error(line: &str) -> String {
		let errors = validate_log(line);
		assert_eq!(errors.len(), 1, "expected exactly one error for: {line}");
		assert_eq!(errors[0].line, 1);
		errors[0].reason.clone()
	}

	#[test]
	fn bad_json_is_reported() {
		assert!(one_error("{not json").contains("invalid JSON"));
	}

	#[test]
	fn a_non_object_record_is_reported() {
		assert!(one_error("[1, 2, 3]").contains("not a JSON object"));
	}

	#[test]
	fn missing_type_is_reported() {
		assert_eq!(one_error(r#"{"task":"demo"}"#), "missing field `type`");
	}

	#[test]
	fn missing_task_is_reported() {
		assert_eq!(
			one_error(r#"{"type":"intake","classification":"trivial","replanned":false}"#),
			"missing field `task`"
		);
	}

	#[test]
	fn unknown_type_is_reported() {
		assert_eq!(one_error(r#"{"type":"mystery","task":"demo"}"#), "unknown `type` `mystery`");
	}

	#[test]
	fn a_missing_per_type_field_is_reported() {
		// A `round` without its `outcome` field.
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"plan_review","changed_since_prev":true,"valid_findings":0,"severities":[],"consecutive_clean":0}"#;
		assert_eq!(one_error(line), "missing field `outcome`");
	}

	#[test]
	fn a_wrong_typed_field_is_reported() {
		// `changed_since_prev` should be a boolean, not a string.
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"plan_review","changed_since_prev":"yes","outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":0}"#;
		assert_eq!(one_error(line), "field `changed_since_prev` has wrong type (expected boolean)");
	}

	#[test]
	fn a_bad_enum_value_is_reported() {
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"midday","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":0}"#;
		assert_eq!(
			one_error(line),
			"field `phase` value `midday` not one of [plan_review, work_review, acceptance, review]"
		);
	}

	#[test]
	fn a_review_phase_round_is_accepted() {
		// The `review` phase (a standalone review-entry-mode pass) is an accepted
		// `phase` value, so a round record logging one validates.
		let line = r#"{"type":"round","task":"review-demo","artifact":"src/","phase":"review","changed_since_prev":false,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","reviewers":[{"role":"reviewer","model":"opus","raw_findings":2,"valid_findings":0}]}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn a_bad_severity_element_is_reported() {
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"acceptance","changed_since_prev":false,"outcome":"clean","valid_findings":0,"severities":["low","fatal"],"consecutive_clean":0}"#;
		assert_eq!(
			one_error(line),
			"field `severities`[1] value `fatal` not one of [low, medium, high, critical]"
		);
	}

	#[test]
	fn a_negative_count_is_reported() {
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"acceptance","changed_since_prev":false,"outcome":"clean","valid_findings":-1,"severities":[],"consecutive_clean":0}"#;
		assert_eq!(
			one_error(line),
			"field `valid_findings` value `-1` is not a non-negative integer"
		);
	}

	#[test]
	fn the_optional_reviewers_field_is_accepted_present_or_absent() {
		// `reviewers` is the only optional `round` calibration field: a round
		// carrying it (alongside the now-required `risk_class`) is valid, and a
		// round omitting it (but still carrying `risk_class`) is equally valid.
		let with = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","reviewers":[{"role":"reviewer","model":"opus","raw_findings":2,"valid_findings":0}]}"#;
		let without = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk"}"#;
		assert_eq!(validate_log(with), Vec::new());
		assert_eq!(validate_log(without), Vec::new());
	}

	#[test]
	fn a_round_missing_risk_class_is_reported() {
		// `risk_class` is required on a `round` record: a round without it (the
		// pre-backfill shape) is now a missing-field error.
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1}"#;
		assert_eq!(one_error(line), "missing field `risk_class`");
	}

	#[test]
	fn a_bad_risk_class_is_reported() {
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"medium"}"#;
		assert_eq!(
			one_error(line),
			"field `risk_class` value `medium` not one of [low_risk, risky]"
		);
	}

	#[test]
	fn a_reviewers_element_missing_a_field_is_reported() {
		// A reviewer entry without `model`; the error locates the array position.
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","reviewers":[{"role":"reviewer","raw_findings":1,"valid_findings":0}]}"#;
		assert_eq!(one_error(line), "field `reviewers`[0]: missing field `model`");
	}

	#[test]
	fn a_reviewers_field_of_wrong_type_is_reported() {
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","reviewers":"opus"}"#;
		assert_eq!(one_error(line), "field `reviewers` has wrong type (expected array)");
	}

	#[test]
	fn an_empty_reviewers_array_is_reported() {
		// A present `reviewers` array must have at least one entry; a round with no
		// attribution omits the field instead (the optional path).
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","reviewers":[]}"#;
		assert_eq!(one_error(line), "field `reviewers` is empty");
	}

	#[test]
	fn a_non_object_reviewers_element_is_reported() {
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","reviewers":[42]}"#;
		assert_eq!(one_error(line), "field `reviewers`[0] has wrong type (expected object)");
	}

	#[test]
	fn a_reviewers_element_with_a_bad_count_is_reported() {
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","reviewers":[{"role":"reviewer","model":"opus","raw_findings":-1,"valid_findings":0}]}"#;
		assert_eq!(
			one_error(line),
			"field `reviewers`[0]: field `raw_findings` value `-1` is not a non-negative integer"
		);
	}

	#[test]
	fn a_reviewers_element_with_a_valid_harness_is_accepted() {
		// The optional `harness` string names the CLI a reviewer ran on; a reviewer
		// entry carrying it validates.
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","reviewers":[{"role":"reviewer","model":"opus","harness":"claude-code","raw_findings":2,"valid_findings":0}]}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn a_reviewers_element_with_a_non_string_harness_is_reported() {
		// When present, `harness` must be a string; a number is reported, and the
		// error locates the offending array element.
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","reviewers":[{"role":"reviewer","model":"opus","harness":7,"raw_findings":2,"valid_findings":0}]}"#;
		assert_eq!(
			one_error(line),
			"field `reviewers`[0]: field `harness` has wrong type (expected string)"
		);
	}

	#[test]
	fn a_valid_decision_record_passes() {
		// A well-formed decision receipt: q_id, a non-empty options array, a
		// recommendation, and a chosen value that is a member of options.
		let line = r#"{"type":"decision","task":"agent-scaffold","q_id":"Q-44","options":["A","B","C"],"recommendation":"B","chosen":"B","ts":"2026-07-18T00:00:00Z"}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn a_decision_with_chosen_not_in_options_is_rejected() {
		// The one genuinely new cross-field constraint: `chosen` must name one of the
		// presented `options`.
		let line = r#"{"type":"decision","task":"t","q_id":"Q-44","options":["A","B"],"recommendation":"A","chosen":"Z"}"#;
		assert_eq!(
			one_error(line),
			"field `chosen` value `Z` is not one of the presented `options` [A, B]"
		);
	}

	#[test]
	fn a_decision_missing_q_id_is_reported() {
		let line =
			r#"{"type":"decision","task":"t","options":["A"],"recommendation":"A","chosen":"A"}"#;
		assert_eq!(one_error(line), "missing field `q_id`");
	}

	#[test]
	fn a_decision_missing_options_is_reported() {
		let line =
			r#"{"type":"decision","task":"t","q_id":"Q-1","recommendation":"A","chosen":"A"}"#;
		assert_eq!(one_error(line), "missing field `options`");
	}

	#[test]
	fn a_decision_missing_recommendation_is_reported() {
		let line = r#"{"type":"decision","task":"t","q_id":"Q-1","options":["A"],"chosen":"A"}"#;
		assert_eq!(one_error(line), "missing field `recommendation`");
	}

	#[test]
	fn a_decision_missing_chosen_is_reported() {
		let line =
			r#"{"type":"decision","task":"t","q_id":"Q-1","options":["A"],"recommendation":"A"}"#;
		assert_eq!(one_error(line), "missing field `chosen`");
	}

	#[test]
	fn a_decision_with_an_empty_options_array_is_reported() {
		let line = r#"{"type":"decision","task":"t","q_id":"Q-1","options":[],"recommendation":"A","chosen":"A"}"#;
		assert_eq!(one_error(line), "field `options` is empty");
	}

	#[test]
	fn a_decision_with_a_non_string_option_is_reported() {
		let line = r#"{"type":"decision","task":"t","q_id":"Q-1","options":["A",7],"recommendation":"A","chosen":"A"}"#;
		assert_eq!(one_error(line), "field `options`[1] has wrong type (expected string)");
	}

	#[test]
	fn a_decision_with_a_non_array_options_is_reported() {
		// `options` must be a JSON array; a bare string (a non-array value) is a
		// wrong-type error, mirroring the sibling `reviewers` wrong-type test.
		let line = r#"{"type":"decision","task":"t","q_id":"Q-1","options":"A,B","recommendation":"A","chosen":"A"}"#;
		assert_eq!(one_error(line), "field `options` has wrong type (expected array)");
	}

	#[test]
	fn a_valid_baseline_record_passes() {
		// A well-formed baseline: a `questions_through` cutoff naming a `Q-<n>` id.
		let line = r#"{"type":"baseline","task":"decision-receipt","questions_through":"Q-44","ts":"2026-07-18"}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn a_baseline_missing_questions_through_is_reported() {
		let line = r#"{"type":"baseline","task":"decision-receipt"}"#;
		assert_eq!(one_error(line), "missing field `questions_through`");
	}

	#[test]
	fn a_baseline_with_a_wrong_typed_questions_through_is_reported() {
		// The cutoff must be a JSON string, not a number.
		let line = r#"{"type":"baseline","task":"decision-receipt","questions_through":44}"#;
		assert_eq!(one_error(line), "field `questions_through` has wrong type (expected string)");
	}

	#[test]
	fn a_baseline_with_a_non_question_id_cutoff_is_reported() {
		// Present and a string, but not the `Q-<n>` shape the cutoff must take.
		let line = r#"{"type":"baseline","task":"decision-receipt","questions_through":"OQ-a"}"#;
		assert_eq!(one_error(line), "field `questions_through` value `OQ-a` is not a `Q-<n>` id");
	}

	#[test]
	fn parse_baseline_projects_valid_records_and_resolves_last_one_wins() {
		// Best-effort projection: a well-formed baseline is projected to (line,
		// cutoff index); a baseline with a non-`Q-<n>` cutoff and a non-baseline
		// record are skipped silently. Two valid baselines are both projected in file
		// order, so the caller's `.last()` (Q-50) supersedes the earlier one (Q-44).
		let log = concat!(
			r#"{"type":"baseline","task":"t","questions_through":"Q-44"}"#,
			"\n",
			r#"{"type":"decision","task":"t","q_id":"Q-45","options":["A"],"recommendation":"A","chosen":"A"}"#,
			"\n",
			r#"{"type":"baseline","task":"t","questions_through":"nope"}"#,
			"\n",
			r#"{"type":"baseline","task":"t","questions_through":"Q-50"}"#,
			"\n",
		);
		let baselines = parse_baseline(log);
		assert_eq!(
			baselines,
			vec![
				Baseline {
					line: 1,
					questions_through: 44,
				},
				Baseline {
					line: 4,
					questions_through: 50,
				},
			]
		);
		assert_eq!(baselines.last().map(|b| b.questions_through), Some(50));
	}

	#[test]
	fn parse_decisions_projects_well_formed_records_and_skips_malformed_ones() {
		// Best-effort projection: a well-formed decision is projected to (q_id, line);
		// a decision missing `q_id`, a non-decision record, and a malformed line are
		// all skipped silently (their reporting is validate_log's job).
		let log = concat!(
			r#"{"type":"decision","task":"t","q_id":"Q-44","options":["A","B"],"recommendation":"A","chosen":"A"}"#,
			"\n",
			r#"{"type":"round","task":"t","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk"}"#,
			"\n",
			r#"{"type":"decision","task":"t","options":["A"]}"#,
			"\n",
			"{not json",
			"\n",
			r#"{"type":"decision","task":"t","q_id":"Q-50","options":["X"],"recommendation":"X","chosen":"X"}"#,
			"\n",
		);
		let decisions = parse_decisions(log);
		assert_eq!(
			decisions,
			vec![
				Decision {
					line: 1,
					q_id: "Q-44".to_string(),
				},
				Decision {
					line: 5,
					q_id: "Q-50".to_string(),
				},
			]
		);
	}

	#[test]
	fn a_valid_step_waiver_passes() {
		// A well-formed step-unit waiver: a self-declared `predates-logging` exemption
		// for a whole step, carrying no `increment` and no `evidence`.
		let line = r#"{"type":"waiver","task":"t","unit":"step","step":"core-assets","reason":"predates-logging","evidence_tier":"self-declared","ts":"2026-07-18"}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn a_valid_increment_waiver_passes() {
		// A well-formed increment-unit waiver: a record-backed `accepted-at-escalation`
		// exemption for one increment, carrying both `increment` and `evidence`.
		let line = r#"{"type":"waiver","task":"t","unit":"increment","step":"optional-modules","increment":"optional-modules-inc2cii","reason":"accepted-at-escalation","evidence_tier":"record-backed","evidence":"optional-modules-inc2cii"}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn a_waiver_with_a_bad_unit_is_reported() {
		let line = r#"{"type":"waiver","task":"t","unit":"module","step":"s","reason":"predates-logging","evidence_tier":"self-declared"}"#;
		assert_eq!(one_error(line), "field `unit` value `module` not one of [step, increment]");
	}

	#[test]
	fn a_waiver_with_a_bad_reason_is_reported() {
		let line = r#"{"type":"waiver","task":"t","unit":"step","step":"s","reason":"because","evidence_tier":"self-declared"}"#;
		assert_eq!(
			one_error(line),
			"field `reason` value `because` not one of [predates-logging, review-skipped, accepted-at-escalation]"
		);
	}

	#[test]
	fn a_waiver_with_a_bad_evidence_tier_is_reported() {
		let line = r#"{"type":"waiver","task":"t","unit":"step","step":"s","reason":"predates-logging","evidence_tier":"strong"}"#;
		assert_eq!(
			one_error(line),
			"field `evidence_tier` value `strong` not one of [self-declared, record-backed]"
		);
	}

	#[test]
	fn a_waiver_with_an_empty_step_is_reported() {
		let line = r#"{"type":"waiver","task":"t","unit":"step","step":"","reason":"predates-logging","evidence_tier":"self-declared"}"#;
		assert_eq!(one_error(line), "field `step` is empty");
	}

	#[test]
	fn an_escalation_with_an_empty_task_is_reported() {
		// The `task` a record-backed waiver's `evidence` joins to must be non-empty,
		// so an escalation with `task:""` can never satisfy the W5 join and is rejected.
		let line = r#"{"type":"escalation","task":"","artifact":"a","human_decision":"decision"}"#;
		assert_eq!(one_error(line), "field `task` is empty");
	}

	#[test]
	fn a_step_waiver_with_an_increment_is_reported() {
		// The `increment` field is forbidden on a step-unit waiver.
		let line = r#"{"type":"waiver","task":"t","unit":"step","step":"s","increment":"s-inc1","reason":"predates-logging","evidence_tier":"self-declared"}"#;
		assert_eq!(one_error(line), "field `increment` is forbidden when `unit` is `step`");
	}

	#[test]
	fn an_increment_waiver_without_an_increment_is_reported() {
		// The `increment` field is required on an increment-unit waiver.
		let line = r#"{"type":"waiver","task":"t","unit":"increment","step":"s","reason":"accepted-at-escalation","evidence_tier":"record-backed","evidence":"s-inc1"}"#;
		assert_eq!(one_error(line), "missing field `increment`");
	}

	#[test]
	fn a_self_declared_waiver_with_evidence_is_reported() {
		// The `evidence` pointer is forbidden on a self-declared waiver.
		let line = r#"{"type":"waiver","task":"t","unit":"step","step":"s","reason":"predates-logging","evidence_tier":"self-declared","evidence":"e"}"#;
		assert_eq!(
			one_error(line),
			"field `evidence` is forbidden when `evidence_tier` is `self-declared`"
		);
	}

	#[test]
	fn a_record_backed_waiver_without_evidence_is_reported() {
		// The `evidence` pointer is required on a record-backed waiver.
		let line = r#"{"type":"waiver","task":"t","unit":"increment","step":"s","increment":"s-inc1","reason":"accepted-at-escalation","evidence_tier":"record-backed"}"#;
		assert_eq!(one_error(line), "missing field `evidence`");
	}

	#[test]
	fn parse_waivers_projects_well_formed_records_and_drops_malformed_ones() {
		// Best-effort projection: two well-formed waivers (a step-unit and an
		// increment-unit) are projected in file order; a waiver with a bad `unit`, one
		// with an `increment` on a step-unit (a broken presence rule), and a non-waiver
		// record are all dropped silently, so a malformed waiver never reaches W3.
		let log = concat!(
			r#"{"type":"waiver","task":"t","unit":"step","step":"core-assets","reason":"predates-logging","evidence_tier":"self-declared"}"#,
			"\n",
			r#"{"type":"waiver","task":"t","unit":"module","step":"s","reason":"predates-logging","evidence_tier":"self-declared"}"#,
			"\n",
			r#"{"type":"waiver","task":"t","unit":"step","step":"s","increment":"s-inc1","reason":"predates-logging","evidence_tier":"self-declared"}"#,
			"\n",
			r#"{"type":"round","task":"t","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk"}"#,
			"\n",
			r#"{"type":"waiver","task":"t","unit":"increment","step":"optional-modules","increment":"optional-modules-inc2cii","reason":"accepted-at-escalation","evidence_tier":"record-backed","evidence":"optional-modules-inc2cii"}"#,
			"\n",
		);
		let waivers = parse_waivers(log);
		assert_eq!(
			waivers,
			vec![
				Waiver {
					locator: "round log line 1".to_string(),
					unit: WaiverUnit::Step,
					step: "core-assets".to_string(),
					increment: None,
					reason: WaiverReason::PredatesLogging,
					evidence_tier: EvidenceTier::SelfDeclared,
					evidence: None,
				},
				Waiver {
					locator: "round log line 5".to_string(),
					unit: WaiverUnit::Increment,
					step: "optional-modules".to_string(),
					increment: Some("optional-modules-inc2cii".to_string()),
					reason: WaiverReason::AcceptedAtEscalation,
					evidence_tier: EvidenceTier::RecordBacked,
					evidence: Some("optional-modules-inc2cii".to_string()),
				},
			]
		);
	}

	#[test]
	fn parse_escalations_projects_well_formed_records_and_skips_others() {
		// Best-effort projection: an escalation record is projected to (line, task,
		// human_decision); a non-escalation record is skipped.
		let log = concat!(
			r#"{"type":"escalation","task":"optional-modules-inc2cii","artifact":"a","human_decision":"decision"}"#,
			"\n",
			r#"{"type":"waiver","task":"t","unit":"step","step":"s","reason":"predates-logging","evidence_tier":"self-declared"}"#,
			"\n",
		);
		let escalations = parse_escalations(log);
		assert_eq!(
			escalations,
			vec![Escalation {
				line: 1,
				task: "optional-modules-inc2cii".to_string(),
				human_decision: HumanDecision::Decision,
				step: None,
				increment: None,
			}]
		);
	}

	#[test]
	fn a_round_with_structured_step_and_increment_ids_is_accepted() {
		// The Inc 2 structured ids are optional: a round carrying both `step` and
		// `increment` (records written from Inc 2 onward) validates.
		let line = r#"{"type":"round","task":"foo-incidental","artifact":"a","phase":"work_review","changed_since_prev":false,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","step":"foo-incidental","increment":"foo-incidental"}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn a_round_with_only_a_structured_step_is_accepted() {
		// S3/O1 (independent optionality): `step` and `increment` are separately
		// optional, so a round carrying `step` WITHOUT `increment` validates. Pins
		// that `require_structured_ids` checks each field on its own and never
		// requires the pair to co-occur.
		let line = r#"{"type":"round","task":"foo-incidental","artifact":"a","phase":"work_review","changed_since_prev":false,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","step":"foo-incidental"}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn a_round_with_only_a_structured_increment_is_accepted() {
		// The mirror of the above: `increment` WITHOUT `step` also validates.
		let line = r#"{"type":"round","task":"foo-incidental","artifact":"a","phase":"work_review","changed_since_prev":false,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","increment":"foo-incidental"}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn an_escalation_with_only_a_structured_step_is_accepted() {
		// The escalation side of the same independent-optionality contract: `step`
		// present, `increment` absent, accepted.
		let line = r#"{"type":"escalation","task":"foo-incidental","artifact":"a","human_decision":"decision","step":"foo-incidental"}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn an_escalation_with_only_a_structured_increment_is_accepted() {
		// The mirror: `increment` present, `step` absent, accepted.
		let line = r#"{"type":"escalation","task":"foo-incidental","artifact":"a","human_decision":"decision","increment":"foo-incidental"}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn a_round_with_an_empty_structured_step_is_reported() {
		// Present but blank is rejected, so a blank id can never masquerade as a real
		// join key (the field is optional, but not optionally-empty).
		let line = r#"{"type":"round","task":"a","artifact":"a","phase":"work_review","changed_since_prev":false,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","step":""}"#;
		assert_eq!(one_error(line), "field `step` is empty");
	}

	#[test]
	fn an_escalation_with_structured_ids_is_accepted() {
		let line = r#"{"type":"escalation","task":"foo-incidental","artifact":"a","human_decision":"decision","step":"foo-incidental","increment":"foo-incidental"}"#;
		assert_eq!(validate_log(line), Vec::new());
	}

	#[test]
	fn an_escalation_with_an_empty_structured_increment_is_reported() {
		let line = r#"{"type":"escalation","task":"t","artifact":"a","human_decision":"decision","increment":""}"#;
		assert_eq!(one_error(line), "field `increment` is empty");
	}

	#[test]
	fn parse_rounds_projects_optional_structured_ids() {
		// A round carrying the structured `step`/`increment` ids projects them; a round
		// omitting them projects `None`, the pre-migration fallback the join shim reads.
		let log = concat!(
			r#"{"type":"round","task":"foo-incidental","artifact":"a","outcome":"clean","consecutive_clean":1,"risk_class":"low_risk","step":"foo-incidental","increment":"foo-incidental"}"#,
			"\n",
			r#"{"type":"round","task":"bar-inc1","artifact":"a","outcome":"clean","consecutive_clean":1,"risk_class":"low_risk"}"#,
			"\n",
		);
		let rounds = parse_rounds(log);
		assert_eq!(rounds.len(), 2);
		assert_eq!(rounds[0].step.as_deref(), Some("foo-incidental"));
		assert_eq!(rounds[0].increment.as_deref(), Some("foo-incidental"));
		assert_eq!(rounds[1].step, None);
		assert_eq!(rounds[1].increment, None);
	}

	#[test]
	fn parse_escalations_projects_optional_structured_ids() {
		// The escalation projection mirrors `parse_rounds`: present ids are carried, an
		// absent one is `None` (the `leading_slug`/`task` fallback W5 reads).
		let log = concat!(
			r#"{"type":"escalation","task":"foo-incidental","artifact":"a","human_decision":"decision","step":"foo-incidental","increment":"foo-incidental"}"#,
			"\n",
			r#"{"type":"escalation","task":"bar-inc1","artifact":"a","human_decision":"decision"}"#,
			"\n",
		);
		let escalations = parse_escalations(log);
		assert_eq!(escalations.len(), 2);
		assert_eq!(escalations[0].step.as_deref(), Some("foo-incidental"));
		assert_eq!(escalations[0].increment.as_deref(), Some("foo-incidental"));
		assert_eq!(escalations[1].step, None);
		assert_eq!(escalations[1].increment, None);
	}

	#[test]
	fn an_optional_ts_of_wrong_type_is_reported() {
		// `ts` is optional, but when present it must be a string.
		let line = r#"{"type":"intake","task":"demo","classification":"trivial","replanned":false,"ts":123}"#;
		assert_eq!(one_error(line), "field `ts` has wrong type (expected string)");
	}

	/// Schema drift-guard: the metrics schema lives in two places, this validator
	/// (the source of truth) and the human-readable prose in `pack/instrument.md`.
	/// This test asserts every value the validator accepts is documented verbatim
	/// in that prose, so changing the schema on one side without the other fails
	/// here (Principle 16, one source of truth; Principle 11, the test exercises
	/// the real accepted set). The enum spellings are iterated from each type's own
	/// `VARIANTS` array rather than re-hardcoded, so renaming a variant in code
	/// automatically re-points the check at the new spelling, and the prose must
	/// then document that new spelling or this test fails. The record-type and
	/// field lists mirror what `check_record` matches on and requires; if a field
	/// is added to or removed from the validator, update this list to match.
	#[test]
	fn instrument_prose_documents_every_accepted_schema_value() {
		let prose = include_str!("../pack/instrument.md");

		// Every record-type name `check_record` accepts (its `match record_type`).
		// Anchor on the quoted form the prose uses (`type: "round"`), not a bare
		// substring: `round`/`escalation` also occur as plain words elsewhere, so an
		// unanchored match would not catch the deletion of a type's documentation.
		for record_type in
			["round", "escalation", "dismissal_recheck", "intake", "decision", "baseline", "waiver"]
		{
			assert!(
				prose.contains(&format!("\"{record_type}\"")),
				"record type `{record_type}` accepted by the validator is not documented in pack/instrument.md"
			);
		}

		// Every field name the validator requires or checks: the common fields on
		// every record, then the per-type fields. Mirrors `check_record`.
		for field in [
			"type",
			"task",
			"ts",
			"artifact",
			"phase",
			"changed_since_prev",
			"outcome",
			"valid_findings",
			"severities",
			"consecutive_clean",
			"risk_class",
			"reviewers",
			"role",
			"model",
			"raw_findings",
			"harness",
			"human_decision",
			"result",
			"classification",
			"replanned",
			"q_id",
			"options",
			"recommendation",
			"chosen",
			"questions_through",
			"unit",
			"step",
			"increment",
			"reason",
			"evidence_tier",
			"evidence",
		] {
			// Anchor with backticks: the prose writes every field as `field`, so a
			// backtick-wrapped match avoids a false positive from a short name
			// appearing as a substring of another word (for example `ts` in `tasks`).
			assert!(
				prose.contains(&format!("`{field}`")),
				"field `{field}` checked by the validator is not documented in pack/instrument.md"
			);
		}

		// Every accepted enum spelling, driven from the validator's own `VARIANTS`
		// so a code-side rename is automatically checked at its new spelling.
		for (enum_name, variants) in [
			("Phase", Phase::VARIANTS),
			("RoundOutcome", RoundOutcome::VARIANTS),
			("HumanDecision", HumanDecision::VARIANTS),
			("RecheckResult", RecheckResult::VARIANTS),
			("Classification", Classification::VARIANTS),
			("RiskClass", RiskClass::VARIANTS),
			("Severity", Severity::VARIANTS),
			("WaiverUnit", WaiverUnit::VARIANTS),
			("WaiverReason", WaiverReason::VARIANTS),
			("EvidenceTier", EvidenceTier::VARIANTS),
		] {
			for variant in variants {
				// Backtick-anchored for the same reason as the field checks: the
				// prose writes every enum value as `value`.
				assert!(
					prose.contains(&format!("`{variant}`")),
					"enum `{enum_name}` value `{variant}` accepted by the validator is not documented in pack/instrument.md"
				);
			}
		}
	}
}
