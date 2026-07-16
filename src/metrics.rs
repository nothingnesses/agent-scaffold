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
	($(#[$meta:meta])* $name:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
		$(#[$meta])*
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		enum $name {
			$($variant),+
		}

		impl $name {
			/// The accepted on-disk spellings, in declaration order.
			const VARIANTS: &'static [&'static str] = &[$($text),+];

			/// Parse an accepted spelling into its variant, or `None` when the
			/// string is not one of the accepted set.
			fn parse(text: &str) -> Option<Self> {
				match text {
					$($text => Some(Self::$variant),)+
					_ => None,
				}
			}
		}
	};
}

enum_field! {
	/// Which review phase a `round` record belongs to.
	Phase {
		PlanReview => "plan_review",
		WorkReview => "work_review",
		Acceptance => "acceptance",
	}
}

enum_field! {
	/// The outcome of a review round.
	RoundOutcome {
		Clean => "clean",
		NewValid => "new_valid",
	}
}

enum_field! {
	/// What the human did at a total-round-cap escalation.
	HumanDecision {
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
	/// because it is a different judgement about a different thing.
	RiskClass {
		LowRisk => "low_risk",
		Risky => "risky",
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
/// non-negative integer `raw_findings` and `valid_findings` counts. This is the
/// per-reviewer breakdown used to calibrate reviewer productivity and whether
/// running multiple models earns its cost; the caller only invokes it when the
/// field is present, since it is optional.
fn require_reviewers(
	obj: &Map<String, Value>,
	name: &str,
) -> Result<(), String> {
	let value = obj.get(name).ok_or_else(|| format!("missing field `{name}`"))?;
	let array = value
		.as_array()
		.ok_or_else(|| format!("field `{name}` has wrong type (expected array)"))?;
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
	}
	Ok(())
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
			// Optional calibration fields, validated only when present so the
			// records written before they existed still pass: the artifact's
			// risk tier at loop-open, and the per-reviewer attribution.
			if obj.contains_key("risk_class") {
				require_enum(obj, "risk_class", RiskClass::VARIANTS, |text| {
					RiskClass::parse(text).is_some()
				})?;
			}
			if obj.contains_key("reviewers") {
				require_reviewers(obj, "reviewers")?;
			}
		}
		"escalation" => {
			require_str(obj, "artifact")?;
			require_enum(obj, "human_decision", HumanDecision::VARIANTS, |text| {
				HumanDecision::parse(text).is_some()
			})?;
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
			"field `phase` value `midday` not one of [plan_review, work_review, acceptance]"
		);
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
	fn optional_round_calibration_fields_are_accepted() {
		// A `round` carrying the optional `risk_class` and `reviewers` fields is
		// valid, and a `round` omitting them (the pre-existing shape) is still valid.
		let with = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk","reviewers":[{"role":"reviewer","model":"opus","raw_findings":2,"valid_findings":0}]}"#;
		let without = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1}"#;
		assert_eq!(validate_log(with), Vec::new());
		assert_eq!(validate_log(without), Vec::new());
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
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"reviewers":[{"role":"reviewer","raw_findings":1,"valid_findings":0}]}"#;
		assert_eq!(one_error(line), "field `reviewers`[0]: missing field `model`");
	}

	#[test]
	fn a_reviewers_field_of_wrong_type_is_reported() {
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"reviewers":"opus"}"#;
		assert_eq!(one_error(line), "field `reviewers` has wrong type (expected array)");
	}

	#[test]
	fn a_reviewers_element_with_a_bad_count_is_reported() {
		let line = r#"{"type":"round","task":"demo","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"reviewers":[{"role":"reviewer","model":"opus","raw_findings":-1,"valid_findings":0}]}"#;
		assert_eq!(
			one_error(line),
			"field `reviewers`[0]: field `raw_findings` value `-1` is not a non-negative integer"
		);
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
		for record_type in ["round", "escalation", "dismissal_recheck", "intake"] {
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
			"human_decision",
			"result",
			"classification",
			"replanned",
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
