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
//!   `pause.md` catch (marked done without review). W3 checks only `complete`
//!   steps; all others (`trivial`, `grandfathered`, `skipped`, and the in-flight
//!   statuses) are not checked.
//! - The round log's internal consistency: within one increment's records, a
//!   `clean` outcome advances the consecutive-clean streak by one and a `new_valid`
//!   resets it to zero, so the logged `consecutive_clean` values are fully
//!   determined by the outcome sequence; a disagreement is reported. That streak is
//!   per loop (per increment): it is one running counter across the different
//!   artifacts an increment's rounds name, not a per-artifact count.

use {
	crate::{
		metrics::{
			self,
			Baseline,
			Decision,
			Round,
			RoundOutcome,
			question_id_index,
		},
		plan::{
			self,
			QUEUE_FOLD_PREFIX,
			Question,
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
///
/// Latent over-strip risk (T3): the strip is purely lexical, so a slug that itself
/// ends `-inc<alnum>` (for example a hypothetical `foo-incidental`, or a Roadmap
/// pair `increment` / `increment-tracker`) would be mis-stripped to its prefix and
/// its rounds misrouted to the wrong step. No current slug hits this, and the
/// alphanumeric run is genuinely needed (`round-log-core` uses `-incA` / `-incB`).
/// Hardening would gate the strip on the remainder matching a known Roadmap slug
/// (an allowlist), removing the ambiguity at the cost of passing the step slugs in;
/// deferred while no live slug is affected.
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
	let questions = plan::parse_questions(plan_markdown);
	let rounds = metrics::parse_rounds(log_contents);
	let decisions = metrics::parse_decisions(log_contents);
	let baselines = metrics::parse_baseline(log_contents);
	let mut problems = round_log_consistency_problems(&rounds);
	problems.extend(w3_problems(&steps, &rounds));
	problems.extend(w4_problems(&questions, &decisions, &baselines));
	problems
}

/// The W4 check: every decided Open-Questions item strictly after the DECLARED
/// baseline cutoff must have a matching `type:"decision"` receipt in the round log.
///
/// W4 is FORWARD-LOOKING and its boundary is an INDEPENDENT DECLARED cutoff, NOT
/// derived from the receipt set. A receipt-derived boundary (the earlier min-index
/// design) is circular: the quantity W4 checks (is a receipt missing?) is the same
/// quantity that would set the boundary, so a forgotten receipt could move its own
/// exemption boundary and slip through silently. The boundary is therefore read
/// from a separate `type:"baseline"` record's `questions_through` cutoff (projected
/// by `metrics::parse_baseline`), which no missing receipt can move.
///
/// Semantics:
///
/// - A baseline IS declared: a decided item is exempt iff its `q_id` index is at or
///   below the cutoff (it predates the mechanism); an item strictly after the
///   cutoff REQUIRES a receipt. Multiple baselines resolve last-one-wins.
/// - NO baseline is declared: every decided item REQUIRES a receipt. The exemption
///   must be DECLARED and visible (the pause.md-catch ethos), never silently
///   inferred, so a fresh project (no pre-existing decided items) needs no baseline
///   and every decision it makes under the mechanism is correctly checked; only a
///   repo migrating with pre-existing decisions declares a baseline to exempt them.
fn w4_problems(
	questions: &[Question],
	decisions: &[Decision],
	baselines: &[Baseline],
) -> Vec<String> {
	// Last-one-wins: a later baseline declaration in file order supersedes an
	// earlier one. `None` means no baseline is declared, so nothing is exempt.
	let cutoff = baselines.last().map(|baseline| baseline.questions_through);
	let mut problems = Vec::new();
	for question in questions {
		// Only decided-and-folded items are in scope; open/exploring/superseded
		// items carry no decision to receipt.
		if !question.status.starts_with(QUEUE_FOLD_PREFIX) {
			continue;
		}
		// An id that does not parse to an index cannot be placed relative to the
		// cutoff, so it is left unchecked (there are none in the live plan).
		let Some(index) = question_id_index(&question.id) else {
			continue;
		};
		// At or below the declared cutoff: predates the mechanism, exempt. With no
		// baseline (`cutoff` is `None`) nothing is exempt, so every decided item is
		// required to carry a receipt.
		if let Some(cutoff) = cutoff {
			if index <= cutoff {
				continue;
			}
		}
		if !decisions.iter().any(|d| d.q_id == question.id) {
			problems.push(format!(
				"Open-Questions item `{}` is decided (folded into a step) but has no matching `type:\"decision\"` receipt in the round log; record a decision receipt with `q_id` `{}`",
				question.id, question.id
			));
		}
	}
	problems
}

/// The round log's internal-consistency check: group records by increment
/// (`task`) alone, then walk each group in file order recomputing the streak the
/// outcome sequence implies (a `clean` adds one, a `new_valid` resets to zero) and
/// report any record whose logged `consecutive_clean` disagrees. The streak spans
/// the different artifacts one increment's rounds name, so it is recomputed per
/// increment, not per artifact. The implied streak is recomputed independently of
/// the logged values, so one wrong record yields exactly one problem rather than
/// cascading into the rest of its group.
fn round_log_consistency_problems(rounds: &[Round]) -> Vec<String> {
	// Group by increment (`task`) only: `consecutive_clean` is a per-loop running
	// streak that spans the different `artifact` values named across one increment's
	// rounds (a change round, then fixes, then verification), so those records share
	// a single streak. Each increment (each full `-inc<x>` task string) is its own
	// review loop, so records for different increments do not share a streak. The
	// counter resets at increment boundaries, which is correct because each is a
	// distinct task string. BTreeMap keeps the report deterministic; each group's
	// Vec stays in file order because the records are pushed in file order.
	let mut groups: BTreeMap<&str, Vec<&Round>> = BTreeMap::new();
	for round in rounds {
		groups.entry(round.task.as_str()).or_default().push(round);
	}

	let mut problems = Vec::new();
	for (task, records) in &groups {
		// Recompute the implied streak across the increment's whole record history
		// in file order. Latent limitation (T4): there is no re-opened-loop boundary,
		// so an increment that legitimately re-opens with a bare `clean` (rather than
		// a `new_valid` reset) would keep climbing and be miscounted. Real re-opens
		// start with `new_valid` (which resets to zero), so current data never hits
		// this.
		let mut implied: u64 = 0;
		for round in records {
			match round.outcome {
				RoundOutcome::Clean => implied += 1,
				RoundOutcome::NewValid => implied = 0,
			}
			if round.consecutive_clean != implied {
				problems.push(format!(
					"round log line {}: task `{}` records consecutive_clean {} but its outcome sequence implies {}",
					round.line, task, round.consecutive_clean, implied
				));
			}
		}
	}
	problems
}

/// The W3 check: for every Roadmap step marked `complete`, its rounds must show
/// convergence. Steps with any other status are skipped, so the terminal statuses
/// (`trivial`, `grandfathered`, `skipped`) and the in-flight ones are not checked.
/// For a `complete` step:
///
/// - Filter round records whose leading slug equals the step slug. No matching
///   records is a violation (the `pause.md` catch: marked complete without review,
///   and not declared `trivial` or `grandfathered`).
/// - Group the matching records by increment (the full `task`). Within each
///   increment the `risk_class` must be consistent, and the increment's peak
///   consecutive-clean streak (over all its records, spanning the artifacts its
///   rounds name) must reach the class's required count (`low_risk` 1, `risky` 2).
///   Grouping per increment, not per step, is what lets a step whose increments
///   converged under different risk classes pass (for example `round-log-core`,
///   `low_risk` at `-incA` and `risky` at `-incB`).
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
			// The streak is per loop (per increment), not per artifact:
			// `consecutive_clean` is one running counter across the different
			// artifacts the increment's rounds name, so take the peak
			// consecutive_clean over ALL of the increment's records and require that
			// single peak to reach the class count.
			//
			// Peak, not terminal (T9): this matches the design's "max
			// consecutive_clean seen" computation and is deliberate, not a bug. In a
			// correctly-run loop the loop stops at convergence, so the peak equals the
			// terminal value; taking the peak is what lets a converged increment pass
			// regardless of any trailing bookkeeping rounds.
			let peak = records.iter().map(|round| round.consecutive_clean).max().unwrap_or(0);
			if peak < required {
				problems.push(format!(
					"Roadmap step `{}` increment `{}` reached a consecutive-clean streak of {} but its `{}` risk class needs {}",
					step.slug,
					increment,
					peak,
					class.label(),
					required
				));
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

	/// Parse a JSONL fixture into decision receipts via the metrics projection.
	fn decisions(jsonl: &str) -> Vec<Decision> {
		metrics::parse_decisions(jsonl)
	}

	/// Parse a JSONL fixture into baseline cutoffs via the metrics projection.
	fn baselines(jsonl: &str) -> Vec<Baseline> {
		metrics::parse_baseline(jsonl)
	}

	/// One minimal `baseline` log line declaring a decided-question cutoff.
	fn baseline_line(questions_through: &str) -> String {
		format!(r#"{{"type":"baseline","task":"t","questions_through":"{questions_through}"}}"#)
	}

	/// Parse an Open-Questions-only plan fixture into questions.
	fn questions(markdown: &str) -> Vec<Question> {
		plan::parse_questions(markdown)
	}

	/// One minimal `decision` receipt log line naming the question id it decides.
	fn decision_line(q_id: &str) -> String {
		format!(
			r#"{{"type":"decision","task":"t","q_id":"{q_id}","options":["A","B"],"recommendation":"A","chosen":"A"}}"#
		)
	}

	/// An Open-Questions plan fixture: one decided-and-folded item per given id.
	fn decided_questions_plan(ids: &[&str]) -> String {
		let mut markdown = String::from("## Open Questions, Decisions, Issues and Blockers\n");
		for id in ids {
			markdown.push_str(&format!("- `{id}` (decided -> folded into `some-step`) an ask.\n"));
		}
		markdown
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
	fn a_multi_artifact_loop_that_converges_across_artifacts_passes() {
		// The real convergence shape: one `risky` increment's review loop runs across
		// three DISTINCT artifacts (change -> fixes -> verification) and the streak is
		// one running counter climbing 0 -> 1 -> 2 across them. The peak (2) meets
		// risky's 2, so the increment converges. Per-artifact grouping would
		// false-flag the `change` artifact (peak 0) and the `fixes` artifact (peak 1);
		// per-loop peak passes it.
		let log = [
			round_line("converge", "converge change", "new_valid", 0, "risky"),
			round_line("converge", "converge fixes", "clean", 1, "risky"),
			round_line("converge", "converge verification", "clean", 2, "risky"),
		]
		.join("\n");
		let problems = w3_problems(&steps(&one_step_plan("converge", "complete")), &rounds(&log));
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn a_multi_artifact_loop_that_never_reaches_the_streak_is_caught() {
		// A `risky` increment whose loop spans two artifacts but whose peak streak is
		// 1 (never 2) is caught exactly once, on the increment as a whole, not once
		// per artifact.
		let log = [
			round_line("short", "short change", "new_valid", 0, "risky"),
			round_line("short", "short fixes", "clean", 1, "risky"),
		]
		.join("\n");
		let problems = w3_problems(&steps(&one_step_plan("short", "complete")), &rounds(&log));
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("reached a consecutive-clean streak of 1")
				&& problems[0].contains("`risky` risk class needs 2"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn a_low_risk_failure_names_the_low_risk_class() {
		// A `low_risk` increment that never logs a clean round (peak 0, needs 1) is
		// caught, and the message must carry the `low_risk` label so a rename of the
		// on-disk spelling cannot diverge from `RiskClass::label` silently.
		let log = round_line("lr", "lr change", "new_valid", 0, "low_risk");
		let problems = w3_problems(&steps(&one_step_plan("lr", "complete")), &rounds(&log));
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("`low_risk` risk class needs 1"), "{}", problems[0]);
	}

	#[test]
	fn an_in_flight_step_with_rounds_is_not_checked() {
		// W3's guard is `status == "complete"`, so an in-flight step (here `in
		// progress`) is not checked even with matching rounds in the log. This pins
		// the guard against a future status-list refactor.
		let log = round_line("wip", "wip change", "new_valid", 0, "risky");
		let problems = w3_problems(&steps(&one_step_plan("wip", "in progress")), &rounds(&log));
		assert!(problems.is_empty(), "{problems:?}");
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
	fn a_streak_spanning_multiple_artifacts_is_consistent() {
		// `consecutive_clean` is one running counter across the increment's artifacts,
		// so a streak that climbs 0 -> 1 -> 2 over three distinct artifacts is
		// internally consistent. Per-(task, artifact) grouping would have recomputed
		// the lone `cc2` verification record as implying 1 and false-flagged it.
		let log = [
			round_line("loop", "loop change", "new_valid", 0, "risky"),
			round_line("loop", "loop fixes", "clean", 1, "risky"),
			round_line("loop", "loop verification", "clean", 2, "risky"),
		]
		.join("\n");
		assert!(round_log_consistency_problems(&rounds(&log)).is_empty());
	}

	#[test]
	fn w4_does_not_flag_a_decided_item_at_or_below_the_baseline_cutoff() {
		// The historical exemption: with a declared baseline cutoff of Q-44, the
		// pre-mechanism decided items at (Q-44) and below (Q-1, Q-40) with no receipt
		// are NOT flagged, because the cutoff is at or above their index.
		let plan = decided_questions_plan(&["Q-1", "Q-40", "Q-44"]);
		let log = baseline_line("Q-44");
		let problems = w4_problems(&questions(&plan), &decisions(&log), &baselines(&log));
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w4_flags_a_decided_item_strictly_above_the_cutoff_without_a_receipt() {
		// Two decided items, Q-44 and Q-45, with a baseline cutoff of Q-44. Q-45 is
		// STRICTLY after the cutoff and has no receipt, so it is flagged; Q-44 (at the
		// cutoff) is exempt. This is the case the derived-min boundary silently missed:
		// no receipt exists for Q-45, yet the missing receipt cannot move the cutoff.
		let plan = decided_questions_plan(&["Q-44", "Q-45"]);
		let log = baseline_line("Q-44");
		let problems = w4_problems(&questions(&plan), &decisions(&log), &baselines(&log));
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("`Q-45` is decided")
				&& problems[0].contains("has no matching `type:\"decision\"` receipt"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w4_passes_a_decided_item_above_the_cutoff_with_a_receipt() {
		// Q-45 is strictly after the Q-44 cutoff, so it is in scope, and it has its
		// matching `type:"decision"` receipt, so it passes.
		let plan = decided_questions_plan(&["Q-45"]);
		let log = [baseline_line("Q-44"), decision_line("Q-45")].join("\n");
		let problems = w4_problems(&questions(&plan), &decisions(&log), &baselines(&log));
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w4_with_no_baseline_requires_a_receipt_for_every_decided_item() {
		// No baseline record: the exemption must be DECLARED, so with none every
		// decided item requires a receipt. Q-1 carries its receipt and passes; Q-44
		// has none and is flagged (the derived-min design would have exempted it).
		let plan = decided_questions_plan(&["Q-1", "Q-44"]);
		let log = decision_line("Q-1");
		let problems = w4_problems(&questions(&plan), &decisions(&log), &baselines(&log));
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("`Q-44` is decided")
				&& problems[0].contains("has no matching `type:\"decision\"` receipt"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w4_with_no_baseline_passes_a_decided_item_that_has_a_receipt() {
		// The companion to the no-baseline case: a decided item with its receipt
		// passes even when no baseline is declared (the receipt satisfies the check).
		let plan = decided_questions_plan(&["Q-1"]);
		let log = decision_line("Q-1");
		let problems = w4_problems(&questions(&plan), &decisions(&log), &baselines(&log));
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w4_ignores_non_decided_queue_items() {
		// Only `decided -> folded into <slug>` items are in scope; an open item after
		// the cutoff carries no decision and is never flagged.
		let plan = concat!(
			"## Open Questions, Decisions, Issues and Blockers\n",
			"- `Q-50` (open) an undecided ask.\n",
		);
		let log = baseline_line("Q-44");
		let problems = w4_problems(&questions(plan), &decisions(&log), &baselines(&log));
		assert!(problems.is_empty(), "{problems:?}");
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
