//! Workflow-invariant cross-reference: check the plan's Roadmap status against
//! the JSONL round log (`docs/metrics/workflow.jsonl`), the `validate --workflow`
//! mode. Detection, not prevention: like `plan.rs` and `metrics.rs`, this reads
//! the two artifacts and reports violations into the `validate` problem list; the
//! scaffolded workflow still writes both directly, with no runtime dependency on
//! this binary.
//!
//! It reuses `plan.rs` (the Roadmap projection) and `metrics.rs` (the round-record
//! projection) rather than parsing either itself; there is no ledger parse (the
//! `ledger-parse` keystone was skipped) and no new record type.
//!
//! Two checks live here:
//!
//! - W3, the key invariant: every Roadmap step marked `complete` must have round
//!   records that converge. A `complete` step with no matching records is the
//!   `pause.md` catch (marked done without review). Steps carrying a review-exempt
//!   status (`trivial`, `grandfathered`, `skipped`) are not checked; W3 checks only
//!   `complete` steps.
//! - The round log's internal consistency: within each artifact's records for one
//!   increment, a `clean` outcome advances the consecutive-clean streak by one and
//!   a `new_valid` resets it to zero, so the logged `consecutive_clean` values are
//!   fully determined by the outcome sequence; a disagreement is reported.

use {
	crate::{
		metrics::{
			self,
			Round,
			RoundOutcome,
		},
		plan::{
			self,
			Step,
		},
	},
	std::collections::BTreeMap,
};

/// The increment suffix marker: a `task` value is a leading step slug optionally
/// followed by `-inc<x>` naming one increment of that step (for example
/// `round-log-core-incA`). The `<x>` token is alphanumeric, not just numeric
/// (`-incA` / `-incB` as well as `-inc1` / `-inc2`), so the strip must accept any
/// run of alphanumerics after the marker.
const INCREMENT_MARKER: &str = "-inc";

/// The leading step slug of a `task`: the value with a trailing `-inc<x>`
/// increment suffix removed, where `<x>` is one or more alphanumeric characters.
/// A `task` with no such suffix (or a `-inc` not followed by an all-alphanumeric
/// run) is returned unchanged. This maps every increment of a step onto the one
/// Roadmap slug W3 keys off, so `round-log-core-incA` and `round-log-core-incB`
/// both resolve to `round-log-core`.
fn leading_slug(task: &str) -> &str {
	if let Some(marker) = task.rfind(INCREMENT_MARKER) {
		let suffix = &task[marker + INCREMENT_MARKER.len() ..];
		if !suffix.is_empty() && suffix.bytes().all(|b| b.is_ascii_alphanumeric()) {
			return &task[.. marker];
		}
	}
	task
}

/// Cross-reference the plan against the round log, returning one human-readable
/// problem per violation (an empty vector means the workflow invariants hold).
/// Combines the round-log internal-consistency check (over every round record)
/// with the W3 step-convergence check (over every `complete` Roadmap step).
pub(crate) fn check_workflow(
	plan_markdown: &str,
	log_contents: &str,
) -> Vec<String> {
	let steps = plan::parse_roadmap(plan_markdown);
	let rounds = metrics::parse_rounds(log_contents);
	let mut problems = round_log_consistency_problems(&rounds);
	problems.extend(w3_problems(&steps, &rounds));
	problems
}

/// The round log's internal-consistency check: group records by increment
/// (`task`) and artifact, then walk each group in file order recomputing the
/// streak the outcome sequence implies (a `clean` adds one, a `new_valid` resets
/// to zero) and report any record whose logged `consecutive_clean` disagrees. The
/// implied streak is recomputed independently of the logged values, so one wrong
/// record yields exactly one problem rather than cascading into the rest of its
/// group.
fn round_log_consistency_problems(rounds: &[Round]) -> Vec<String> {
	// Group by (task, artifact): the streak is per artifact and each increment is
	// its own review loop, so records for different increments or artifacts do not
	// share a streak. BTreeMap keeps the report deterministic; each group's Vec
	// stays in file order because the records are pushed in file order.
	let mut groups: BTreeMap<(&str, &str), Vec<&Round>> = BTreeMap::new();
	for round in rounds {
		groups.entry((round.task.as_str(), round.artifact.as_str())).or_default().push(round);
	}

	let mut problems = Vec::new();
	for ((task, artifact), records) in &groups {
		let mut implied: u64 = 0;
		for round in records {
			match round.outcome {
				RoundOutcome::Clean => implied += 1,
				RoundOutcome::NewValid => implied = 0,
			}
			if round.consecutive_clean != implied {
				problems.push(format!(
					"round log line {}: task `{}` artifact `{}` records consecutive_clean {} but its outcome sequence implies {}",
					round.line, task, artifact, round.consecutive_clean, implied
				));
			}
		}
	}
	problems
}

/// The W3 check: for every Roadmap step marked `complete`, its rounds must show
/// convergence. Steps with any other status are skipped, so the review-exempt
/// terminal statuses (`trivial`, `grandfathered`, `skipped`) and the in-flight
/// ones are not checked. For a `complete` step:
///
/// - Filter round records whose leading slug equals the step slug. No matching
///   records is a violation (the `pause.md` catch: marked complete without review,
///   and not declared `trivial` or `grandfathered`).
/// - Group the matching records by increment (the full `task`). Within each
///   increment the `risk_class` must be consistent, and each artifact's
///   consecutive-clean streak must reach the class's required count (`low_risk` 1,
///   `risky` 2). Grouping per increment, not per step, is what lets a step whose
///   increments converged under different risk classes pass (for example
///   `round-log-core`, `low_risk` at `-incA` and `risky` at `-incB`).
fn w3_problems(
	steps: &[Step],
	rounds: &[Round],
) -> Vec<String> {
	let mut problems = Vec::new();
	for step in steps {
		if step.status != "complete" {
			continue;
		}
		let matching: Vec<&Round> =
			rounds.iter().filter(|round| leading_slug(&round.task) == step.slug).collect();
		if matching.is_empty() {
			problems.push(format!(
				"Roadmap step `{}` is `complete` but has no round records; if the review loop was deliberately skipped mark it `trivial`, if it predates round-logging mark it `grandfathered`",
				step.slug
			));
			continue;
		}

		// Group the step's records by increment (the full `task`), so each
		// increment's convergence is judged on its own terms.
		let mut increments: BTreeMap<&str, Vec<&Round>> = BTreeMap::new();
		for round in &matching {
			increments.entry(round.task.as_str()).or_default().push(round);
		}
		for (increment, records) in &increments {
			// The `risk_class` must be consistent within the increment; without a
			// single class the required streak is undefined, so report and move on.
			let class = records[0].risk_class;
			if records.iter().any(|round| round.risk_class != class) {
				problems.push(format!(
					"Roadmap step `{}` increment `{}` logs inconsistent risk_class values",
					step.slug, increment
				));
				continue;
			}
			let required = class.required_streak();
			// The streak is per artifact: take each artifact's peak
			// consecutive_clean within the increment and require it to reach the
			// class count.
			let mut peak: BTreeMap<&str, u64> = BTreeMap::new();
			for round in records {
				let entry = peak.entry(round.artifact.as_str()).or_default();
				*entry = (*entry).max(round.consecutive_clean);
			}
			for (artifact, reached) in &peak {
				if *reached < required {
					problems.push(format!(
						"Roadmap step `{}` increment `{}` artifact `{}` reached a consecutive-clean streak of {} but its `{}` risk class needs {}",
						step.slug,
						increment,
						artifact,
						reached,
						class.label(),
						required
					));
				}
			}
		}
	}
	problems
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Parse a Roadmap-only plan fixture into steps (W3 reads only the Roadmap).
	fn steps(markdown: &str) -> Vec<Step> {
		plan::parse_roadmap(markdown)
	}

	/// Parse a JSONL fixture into rounds via the metrics projection.
	fn rounds(jsonl: &str) -> Vec<Round> {
		metrics::parse_rounds(jsonl)
	}

	/// Build one minimal `round` log line carrying only the fields the projection
	/// reads, so fixtures stay small.
	fn round_line(
		task: &str,
		artifact: &str,
		outcome: &str,
		consecutive_clean: u64,
		risk_class: &str,
	) -> String {
		format!(
			r#"{{"type":"round","task":"{task}","artifact":"{artifact}","outcome":"{outcome}","consecutive_clean":{consecutive_clean},"risk_class":"{risk_class}"}}"#
		)
	}

	/// A Roadmap fixture with one row of the given slug and status.
	fn one_step_plan(
		slug: &str,
		status: &str,
	) -> String {
		format!(
			concat!(
				"## Roadmap\n",
				"| Step | Status |\n",
				"| ---- | ------ |\n",
				"| `{}` | {} |\n",
			),
			slug, status
		)
	}

	#[test]
	fn leading_slug_strips_alphanumeric_increment_suffixes() {
		// The `<x>` token is alphanumeric, not only numeric, so `-incA` / `-incB`
		// strip just like `-inc1` / `-inc2`.
		assert_eq!(leading_slug("round-log-core-incA"), "round-log-core");
		assert_eq!(leading_slug("round-log-core-incB"), "round-log-core");
		assert_eq!(leading_slug("state-schema-inc1"), "state-schema");
		assert_eq!(leading_slug("state-schema-inc12"), "state-schema");
		// No suffix: unchanged. A `-inc` not followed by an all-alphanumeric run is
		// not an increment marker. A bare slug that merely contains `inc` (no leading
		// hyphen) is untouched.
		assert_eq!(leading_slug("round-log-core"), "round-log-core");
		assert_eq!(leading_slug("instrument-flag"), "instrument-flag");
		assert_eq!(leading_slug("foo-inc"), "foo-inc");
		assert_eq!(leading_slug("foo-inc-bar"), "foo-inc-bar");
	}

	#[test]
	fn a_trivial_step_is_exempt() {
		// A `trivial` step needs no round records: W3 checks only `complete` steps.
		let problems = w3_problems(&steps(&one_step_plan("skip-me", "trivial")), &[]);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn a_grandfathered_step_is_exempt() {
		let problems = w3_problems(&steps(&one_step_plan("legacy", "grandfathered")), &[]);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn a_skipped_step_is_exempt() {
		let problems = w3_problems(&steps(&one_step_plan("dropped", "skipped")), &[]);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn a_complete_step_with_no_records_is_caught() {
		// The `pause.md` catch: marked `complete` with no matching rounds.
		let problems = w3_problems(&steps(&one_step_plan("no-review", "complete")), &[]);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("`no-review` is `complete` but has no round records"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn per_increment_grouping_passes_a_step_that_converged_across_two_risk_classes() {
		// `round-log-core` converged as `-incA` (low_risk, streak 1) and `-incB`
		// (risky, streak 2). Per-increment grouping must PASS this; a per-step
		// aggregate would see an inconsistent risk_class and never accept it. This
		// also exercises the alphanumeric `-incA` / `-incB` strip.
		let log = [
			round_line("round-log-core-incA", "src/metrics.rs", "clean", 1, "low_risk"),
			round_line("round-log-core-incB", "src/metrics.rs", "new_valid", 0, "risky"),
			round_line("round-log-core-incB", "src/metrics.rs", "clean", 1, "risky"),
			round_line("round-log-core-incB", "src/metrics.rs", "clean", 2, "risky"),
		]
		.join("\n");
		let problems =
			w3_problems(&steps(&one_step_plan("round-log-core", "complete")), &rounds(&log));
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn a_complete_increment_that_never_reaches_the_streak_is_caught() {
		// A risky increment that only ever reaches streak 1 (needs 2) is flagged.
		let log = [
			round_line("stall-incA", "AGENTS.md", "new_valid", 0, "risky"),
			round_line("stall-incA", "AGENTS.md", "clean", 1, "risky"),
		]
		.join("\n");
		let problems = w3_problems(&steps(&one_step_plan("stall", "complete")), &rounds(&log));
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("reached a consecutive-clean streak of 1")
				&& problems[0].contains("`risky` risk class needs 2"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn a_risk_class_inconsistency_within_one_increment_is_caught() {
		// Two records for the SAME increment disagreeing on risk_class is a
		// violation (distinct from the two-increment case, which is fine).
		let log = [
			round_line("mixup-incA", "AGENTS.md", "new_valid", 0, "low_risk"),
			round_line("mixup-incA", "AGENTS.md", "clean", 1, "risky"),
		]
		.join("\n");
		let problems = w3_problems(&steps(&one_step_plan("mixup", "complete")), &rounds(&log));
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("inconsistent risk_class"), "{}", problems[0]);
	}

	#[test]
	fn a_round_log_consecutive_clean_inconsistency_is_caught() {
		// A `clean` round after a `new_valid` should log consecutive_clean 1, not 2.
		let log = [
			round_line("some-task", "AGENTS.md", "new_valid", 0, "low_risk"),
			round_line("some-task", "AGENTS.md", "clean", 2, "low_risk"),
		]
		.join("\n");
		let problems = round_log_consistency_problems(&rounds(&log));
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("records consecutive_clean 2 but its outcome sequence implies 1"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn a_consistent_round_log_has_no_consistency_problems() {
		// A clean streak that increments correctly, then a new_valid reset, is fine.
		let log = [
			round_line("t", "a", "clean", 1, "low_risk"),
			round_line("t", "a", "clean", 2, "low_risk"),
			round_line("t", "a", "new_valid", 0, "low_risk"),
			round_line("t", "a", "clean", 1, "low_risk"),
		]
		.join("\n");
		assert!(round_log_consistency_problems(&rounds(&log)).is_empty());
	}

	#[test]
	fn check_workflow_catches_the_pause_pattern_and_passes_round_log_core() {
		// End to end over both checks: a `pause.md` step (complete, no rounds) is
		// caught, while the per-increment `round-log-core` pattern in the same log
		// is not false-flagged.
		let plan = concat!(
			"## Roadmap\n",
			"| Step             | Status   |\n",
			"| ---------------- | -------- |\n",
			"| `round-log-core` | complete |\n",
			"| `pause`          | complete |\n",
			"| `declared`       | trivial  |\n",
		);
		let log = [
			round_line("round-log-core-incA", "src/metrics.rs", "clean", 1, "low_risk"),
			round_line("round-log-core-incB", "src/metrics.rs", "new_valid", 0, "risky"),
			round_line("round-log-core-incB", "src/metrics.rs", "clean", 1, "risky"),
			round_line("round-log-core-incB", "src/metrics.rs", "clean", 2, "risky"),
		]
		.join("\n");
		let problems = check_workflow(plan, &log);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("`pause` is `complete` but has no round records"),
			"{}",
			problems[0]
		);
	}
}
