//! The workflow control-constants spec: the deterministic CONTROL CONSTANTS the
//! workflow counts and thresholds against, parsed from `.agents/workflow.toml` (a
//! tool-owned, pack-shipped reference asset) or taken from a built-in default.
//!
//! This is the data-driven boundary of the workflow driver (Stage 0a): a workflow
//! VALUE change (a required streak, the round cap, the backstop severity) is a data
//! edit reviewable like a plan edit, while the transition FUNCTION (the streak/cap
//! arithmetic) stays in Rust. Today this single-sources the one convergence constant
//! W3 checks; `round_cap`/`backstop_severity` are carried and exposed but NOT yet
//! wired into any enforcement (adding a cap/backstop gate would change behaviour and
//! belongs to a later stage).
//!
//! `WorkflowSpec::builtin()` returns today's hardcoded constants, so a project that
//! ships no spec validates byte-for-byte unchanged. A drift-guard test parses the
//! shipped `pack/workflow.toml` and asserts it equals `builtin()`, so the asset and
//! the built-in default cannot diverge.

use {
	crate::metrics::{
		RiskClass,
		Severity,
	},
	serde::Deserialize,
	std::fmt,
};

/// The workflow's control constants, the input both `validate --workflow` and (in a
/// later stage) the driver read. Holds ONLY constants, no logic: the arithmetic that
/// consumes them lives in `workflow.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkflowSpec {
	/// The consecutive-clean streak a `low_risk` increment must reach to converge.
	required_streak_low_risk: u64,
	/// The consecutive-clean streak a `risky` increment must reach to converge.
	required_streak_risky: u64,
	/// The total-round cap (the human-escalation threshold). ADVISORY: exposed via
	/// `round_cap` but not machine-enforced at this stage.
	round_cap: u64,
	/// The severity at or above which a dismissal triggers a re-check (the backstop).
	/// ADVISORY: exposed via `backstop_severity` but not machine-enforced at this
	/// stage.
	backstop_severity: Severity,
}

impl WorkflowSpec {
	/// The built-in default: today's hardcoded constants (`low_risk` 1, `risky` 2,
	/// cap 5, backstop `high`). A project shipping no `.agents/workflow.toml` uses
	/// this, so it validates byte-for-byte unchanged. Pinned equal to the shipped
	/// asset by the drift-guard test.
	pub(crate) fn builtin() -> Self {
		WorkflowSpec {
			required_streak_low_risk: 1,
			required_streak_risky: 2,
			round_cap: 5,
			backstop_severity: Severity::High,
		}
	}

	/// Parse a `workflow.toml` control-constants spec from its TOML source, returning
	/// a typed error on malformed input (a parse failure or a missing/wrong-typed
	/// field).
	pub(crate) fn parse(source: &str) -> Result<Self, WorkflowSpecError> {
		let raw: WorkflowSpecToml = toml::from_str(source).map_err(WorkflowSpecError::Parse)?;
		Ok(WorkflowSpec {
			required_streak_low_risk: raw.convergence.low_risk,
			required_streak_risky: raw.convergence.risky,
			round_cap: raw.rounds.cap,
			backstop_severity: raw.backstop.severity,
		})
	}

	/// The consecutive-clean streak an increment of `class` must reach to converge.
	/// This is the single source W3 checks a `complete` step's rounds against.
	pub(crate) fn required_streak(
		&self,
		class: RiskClass,
	) -> u64 {
		match class {
			RiskClass::LowRisk => self.required_streak_low_risk,
			RiskClass::Risky => self.required_streak_risky,
		}
	}

	/// The total-round cap (the human-escalation threshold). ADVISORY: read by the
	/// generated workflow-control fragment and the tests, but NOT wired into any
	/// enforcement at this stage (a cap gate would change behaviour and belongs to a
	/// later stage).
	pub(crate) fn round_cap(&self) -> u64 {
		self.round_cap
	}

	/// The backstop severity threshold. ADVISORY: read by the generated
	/// workflow-control fragment and the tests, but NOT wired into any enforcement at
	/// this stage (a backstop gate would change behaviour and belongs to a later
	/// stage).
	pub(crate) fn backstop_severity(&self) -> Severity {
		self.backstop_severity
	}

	/// The generated workflow-control fragment: a plain-prose statement of the three
	/// convergence constants (the required clean-round count per risk class, the
	/// total-round cap, and the backstop severity threshold), derived from this spec.
	///
	/// This is the value the pack substitutes into the `{{workflow_control}}` slot of
	/// the scaffolded `AGENTS.md` (see `build_assets` in `main.rs`), the forward
	/// prose-generating member of the workflow projection family: the fragment and
	/// the arithmetic the tool runs both read the one spec, so the process prose and
	/// the process logic cannot drift. The surrounding rationale in `AGENTS.md` (WHY
	/// two clean rounds, WHY a cap) stays hand-authored and refers to these values
	/// rather than restating them.
	///
	/// The built-in pack renders this from `WorkflowSpec::builtin()`, which the
	/// shipped `pack/workflow.toml` is pinned equal to (see the drift-guard test), so
	/// for the default scaffold the fragment states the same constants the shipped
	/// spec holds.
	pub(crate) fn control_fragment(&self) -> String {
		format!(
			"These control constants are generated from the workflow spec, so this statement and the arithmetic the tool runs from that spec cannot drift: converging a review loop takes {low} consecutive clean round for a trivial or low-risk artifact and {risky} for a risky or high-blast-radius one; the total-round cap that escalates the loop to a human is {cap} rounds; and the backstop re-check covers a dismissed finding whose severity is {severity} or above.",
			low = self.required_streak(RiskClass::LowRisk),
			risky = self.required_streak(RiskClass::Risky),
			cap = self.round_cap(),
			severity = self.backstop_severity().label(),
		)
	}
}

/// The on-disk shape of a `workflow.toml`: the three constant groups. A missing
/// section or field, or a wrong-typed value, fails the parse (there is no default),
/// so a malformed spec is reported rather than silently falling back.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkflowSpecToml {
	convergence: Convergence,
	rounds: Rounds,
	backstop: Backstop,
}

/// The `[convergence]` table: the required clean-round streak per risk class.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Convergence {
	low_risk: u64,
	risky: u64,
}

/// The `[rounds]` table: the total-round cap.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Rounds {
	cap: u64,
}

/// The `[backstop]` table: the severity threshold, parsed into the typed `Severity`
/// (so an out-of-set spelling fails the parse).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Backstop {
	severity: Severity,
}

/// An error parsing a `workflow.toml` control-constants spec.
#[derive(Debug)]
pub(crate) enum WorkflowSpecError {
	/// The TOML was malformed, or a required constant was missing or wrong-typed.
	Parse(toml::de::Error),
}

impl fmt::Display for WorkflowSpecError {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			WorkflowSpecError::Parse(error) => write!(f, "malformed workflow spec: {error}"),
		}
	}
}

impl std::error::Error for WorkflowSpecError {}

#[cfg(test)]
mod tests {
	use super::*;

	/// The shipped reference asset, embedded so the drift-guard test reads exactly
	/// what the pack ships.
	const SHIPPED: &str = include_str!("../pack/workflow.toml");

	#[test]
	fn builtin_equals_the_old_hardcoded_constants() {
		// The built-in default must equal the constants Stage 0a single-sourced out of
		// `RiskClass::required_streak` and the workflow prose, so an un-migrated project
		// validates byte-for-byte unchanged.
		let spec = WorkflowSpec::builtin();
		assert_eq!(spec.required_streak(RiskClass::LowRisk), 1);
		assert_eq!(spec.required_streak(RiskClass::Risky), 2);
		assert_eq!(spec.round_cap(), 5);
		assert_eq!(spec.backstop_severity(), Severity::High);
	}

	#[test]
	fn the_shipped_asset_parses_to_exactly_the_builtin() {
		// Drift guard: the pack-shipped `workflow.toml` and `WorkflowSpec::builtin()`
		// cannot diverge, so editing one without the other fails here.
		let parsed = WorkflowSpec::parse(SHIPPED).expect("the shipped workflow.toml must parse");
		assert_eq!(parsed, WorkflowSpec::builtin());
	}

	/// The committed root `AGENTS.md`, embedded so the drift-guard test reads exactly
	/// the scaffold output the repo ships (dogfooded from the pack).
	const COMMITTED_AGENTS: &str = include_str!("../AGENTS.md");

	/// The committed `.agents/AGENTS.reference.md`, the tool-owned reference copy of
	/// the same generated guidance.
	const COMMITTED_REFERENCE: &str = include_str!("../.agents/AGENTS.reference.md");

	#[test]
	fn the_control_fragment_states_the_spec_constants() {
		// The generated fragment must state the spec's values (1 clean round for
		// low-risk, 2 for risky, a 5-round cap, backstop severity high). A change to
		// the wording or to a `builtin()` constant fails here directly, so the
		// fragment's content is pinned to a spec-derived string independent of the
		// scaffold output.
		assert_eq!(
			WorkflowSpec::builtin().control_fragment(),
			"These control constants are generated from the workflow spec, so this statement and the arithmetic the tool runs from that spec cannot drift: converging a review loop takes 1 consecutive clean round for a trivial or low-risk artifact and 2 for a risky or high-blast-radius one; the total-round cap that escalates the loop to a human is 5 rounds; and the backstop re-check covers a dismissed finding whose severity is high or above."
		);
	}

	#[test]
	fn the_committed_scaffold_carries_the_generated_fragment() {
		// Drift guard on the PACK generation path: the committed scaffold output (the
		// dogfooded root `AGENTS.md` and its reference copy) must carry the exact
		// fragment `WorkflowSpec::builtin()` generates. This fails on a hand edit of
		// the fragment in the committed output (it no longer matches) and on a stale
		// fragment after a `builtin()` constant edit that was not re-scaffolded (the
		// freshly generated fragment changes while the committed bytes do not). The
		// fix in either case is `just scaffold-self`.
		let fragment = WorkflowSpec::builtin().control_fragment();
		assert!(
			COMMITTED_AGENTS.contains(&fragment),
			"root AGENTS.md is missing the current generated workflow-control fragment; run `just scaffold-self`"
		);
		assert!(
			COMMITTED_REFERENCE.contains(&fragment),
			".agents/AGENTS.reference.md is missing the current generated workflow-control fragment; run `just scaffold-self`"
		);
	}

	#[test]
	fn a_valid_spec_round_trips_its_constants() {
		let spec = WorkflowSpec::parse(
			"[convergence]\nlow_risk = 3\nrisky = 4\n\n[rounds]\ncap = 9\n\n[backstop]\nseverity = \"critical\"\n",
		)
		.unwrap();
		assert_eq!(spec.required_streak(RiskClass::LowRisk), 3);
		assert_eq!(spec.required_streak(RiskClass::Risky), 4);
		assert_eq!(spec.round_cap(), 9);
		assert_eq!(spec.backstop_severity(), Severity::Critical);
	}

	#[test]
	fn a_malformed_spec_is_a_typed_error() {
		// A missing required constant fails the parse rather than defaulting silently.
		let result = WorkflowSpec::parse("[convergence]\nlow_risk = 1\n");
		assert!(matches!(result, Err(WorkflowSpecError::Parse(_))));

		// An out-of-set severity spelling also fails.
		let result = WorkflowSpec::parse(
			"[convergence]\nlow_risk = 1\nrisky = 2\n\n[rounds]\ncap = 5\n\n[backstop]\nseverity = \"extreme\"\n",
		);
		assert!(matches!(result, Err(WorkflowSpecError::Parse(_))));
	}

	#[test]
	fn a_typoed_or_extra_key_fails_to_parse() {
		// `deny_unknown_fields` makes a typo'd or extra key a hard parse error rather
		// than a silent no-op, so an edit that misspells a constant cannot quietly
		// leave the convergence bar at its old value.
		let result = WorkflowSpec::parse(
			"[convergence]\nlow_risk = 1\nrisky = 2\nmedium_risk = 3\n\n[rounds]\ncap = 5\n\n[backstop]\nseverity = \"high\"\n",
		);
		assert!(matches!(result, Err(WorkflowSpecError::Parse(_))));
	}
}
