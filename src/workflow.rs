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
//! The checks that live here:
//!
//! - W3, the key invariant: every Roadmap step marked `complete` must have round
//!   records that converge, OR a covering `type:"waiver"` record. A `complete` step
//!   with no matching records and no covering step-waiver is the `pause.md` catch
//!   (marked done without review). W3 checks only `complete` steps; the others
//!   (`skipped` and the in-flight statuses) are not checked. W3 asks only whether a
//!   covering waiver of the right unit and identity exists; whether that waiver is
//!   itself well-evidenced is W5's job, kept orthogonal.
//! - W5, the waiver-integrity check: every waiver must name a real Roadmap step,
//!   an `increment`-unit waiver's `step` must own its `increment`, every
//!   `record-backed` waiver's `evidence` must join to a real `type:"escalation"`
//!   record with `human_decision:"decision"` that is scoped to the waived unit,
//!   and the `reason` <-> `evidence_tier` pairing must be consistent so a
//!   self-declaration cannot claim the strong tier.
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
			question_id_index,
			Baseline,
			Decision,
			Escalation,
			EvidenceTier,
			HumanDecision,
			Round,
			RoundOutcome,
			Waiver,
			WaiverReason,
			WaiverUnit,
		},
		plan::{
			self,
			Question,
			Step,
			QUEUE_FOLD_PREFIX,
		},
	},
	std::collections::{
		BTreeMap,
		BTreeSet,
	},
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
/// Inc 2 retires this risk for NEW data: a `round`/`escalation` record may carry a
/// structured `step`/`increment` id, and `round_step_slug`/`escalation_step_slug`
/// (and their increment counterparts) prefer it, so a record with the field joins
/// without ever reaching this lexical strip. This shim remains only for
/// pre-migration records that omit the structured id.
fn leading_slug(task: &str) -> &str {
	if let Some(marker) = task.rfind(INCREMENT_MARKER) {
		let suffix = &task[marker + INCREMENT_MARKER.len() ..];
		if !suffix.is_empty() && suffix.bytes().all(|b| b.is_ascii_alphanumeric()) {
			return &task[.. marker];
		}
	}
	task
}

// The four join accessors below resolve the STEP axis and the INCREMENT axis
// INDEPENDENTLY. Per the Inc 2 contract the two ids are separately optional ("when
// either is present it must be a non-empty string"), so a record may carry exactly
// one of them; each accessor therefore falls back on its OWN field alone, with no
// coupling to the other. The consequence is a chosen, pinned outcome, not an
// accident: an `increment`-only record still resolves its STEP join through the
// `leading_slug(task)` shim, so if its `task` ends `-inc<alnum>` the residual T3
// over-strip persists on the UNFILLED step axis (pinned by
// `w3_an_increment_only_round_falls_back_to_the_shim_on_the_unfilled_step_axis`);
// symmetrically a `step`-only record groups by its raw `task` on the increment
// axis. We deliberately do NOT force the two to co-occur: that would contradict
// the contract's independent optionality. Principle 19: doc and code agree that a
// partial record falls back per axis rather than being rejected or specially
// cased.

/// The Roadmap step slug a round joins to: its STRUCTURED `step` id when the
/// record carries one (records written from Inc 2 onward), else `leading_slug`
/// of its `task` (the pre-migration shim). Preferring the structured id retires
/// the SE-10/B6 lexical over-strip risk (T3) for new data: a record whose slug
/// itself ends `-inc<alnum>` joins correctly on its declared `step` instead of
/// being mis-stripped to its prefix.
fn round_step_slug(round: &Round) -> &str {
	round.step.as_deref().unwrap_or_else(|| leading_slug(&round.task))
}

/// The increment id a round belongs to: its STRUCTURED `increment` id when the
/// record carries one, else its `task` verbatim (the pre-migration shim). This
/// is the identity the convergence streak is counted per, and Inc 4 will join it
/// to the TOML `[[step.increment]].id`.
fn round_increment_id(round: &Round) -> &str {
	round.increment.as_deref().unwrap_or(&round.task)
}

/// The Roadmap step slug an escalation joins to, mirroring `round_step_slug`:
/// the structured `step` id when present, else `leading_slug(task)`. W5's
/// step-unit scope check keys off this.
fn escalation_step_slug(escalation: &Escalation) -> &str {
	escalation.step.as_deref().unwrap_or_else(|| leading_slug(&escalation.task))
}

/// The increment id an escalation belongs to, mirroring `round_increment_id`:
/// the structured `increment` id when present, else `task` verbatim. W5's
/// increment-unit scope check keys off this.
fn escalation_increment_id(escalation: &Escalation) -> &str {
	escalation.increment.as_deref().unwrap_or(&escalation.task)
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
	let waivers = metrics::parse_waivers(log_contents);
	let escalations = metrics::parse_escalations(log_contents);
	let mut problems = round_log_consistency_problems(&rounds);
	problems.extend(w3_problems(&steps, &rounds, &waivers));
	problems.extend(w4_problems(&questions, &decisions, &baselines));
	problems.extend(w5_problems(&waivers, &steps, &escalations));
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

/// The round log's internal-consistency check: group records by increment (the
/// structured `increment` id when a record carries one, else its `task` via
/// `round_increment_id`) alone, then walk each group in file order recomputing the streak the
/// outcome sequence implies (a `clean` adds one, a `new_valid` resets to zero) and
/// report any record whose logged `consecutive_clean` disagrees. The streak spans
/// the different artifacts one increment's rounds name, so it is recomputed per
/// increment, not per artifact. The implied streak is recomputed independently of
/// the logged values, so one wrong record yields exactly one problem rather than
/// cascading into the rest of its group.
fn round_log_consistency_problems(rounds: &[Round]) -> Vec<String> {
	// Group by increment only (the structured `increment` id, or the `task` when a
	// record omits it): `consecutive_clean` is a per-loop running streak that spans
	// the different `artifact` values named across one increment's rounds (a change
	// round, then fixes, then verification), so those records share a single streak.
	// Each increment (each structured id, or each full `-inc<x>` task string on a
	// pre-migration record) is its own review loop, so records for different
	// increments do not share a streak. The counter resets at increment boundaries,
	// which is correct because each is a distinct increment id. BTreeMap keeps the
	// report deterministic; each group's Vec stays in file order because the records
	// are pushed in file order.
	let mut groups: BTreeMap<&str, Vec<&Round>> = BTreeMap::new();
	for round in rounds {
		groups.entry(round_increment_id(round)).or_default().push(round);
	}

	let mut problems = Vec::new();
	for (increment, records) in &groups {
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
					"round log line {}: increment `{}` records consecutive_clean {} but its outcome sequence implies {}",
					round.line, increment, round.consecutive_clean, implied
				));
			}
		}
	}
	problems
}

/// The W3 check: for every Roadmap step marked `complete`, its rounds must show
/// convergence, OR a covering `type:"waiver"` record must exempt the shortfall.
/// Steps with any other status are skipped, so `skipped` and the in-flight statuses
/// are not checked. For a `complete` step:
///
/// - Filter round records whose step slug equals the step slug, via
///   `round_step_slug` (the record's structured `step` id from Inc 2 when present,
///   else `leading_slug(task)`). No matching records is a violation (the `pause.md`
///   catch: marked complete without review), UNLESS a STEP-level waiver covers the
///   step (`unit == step`, `step == slug`), which exempts it (a step that predates
///   logging or whose review was skipped).
/// - Group the matching records by increment (`round_increment_id`: the structured
///   `increment` id when present, else the full `task`). Within each
///   increment the `risk_class` must be consistent, and the increment's peak
///   consecutive-clean streak (over all its records, spanning the artifacts its
///   rounds name) must reach the class's required count (`low_risk` 1, `risky` 2),
///   UNLESS an INCREMENT-level waiver covers that increment (`unit == increment`,
///   `increment == <that increment's task>`), which exempts the shortfall.
///   Grouping per increment, not per step, is what lets a step whose increments
///   converged under different risk classes pass (for example `round-log-core`,
///   `low_risk` at `-incA` and `risky` at `-incB`).
///
/// W3 consults ONLY the waiver's unit and identity (does a covering waiver exist?);
/// it does NOT inspect `reason` or `evidence_tier`, which is W5's job, so the two
/// checks stay orthogonal. The `risk_class`-inconsistency error within an increment
/// is a data-integrity fault and is NOT suppressed by any waiver.
fn w3_problems(
	steps: &[Step],
	rounds: &[Round],
	waivers: &[Waiver],
) -> Vec<String> {
	let mut problems = Vec::new();
	for step in steps {
		if step.status != "complete" {
			continue;
		}
		let matching: Vec<&Round> =
			rounds.iter().filter(|round| round_step_slug(round) == step.slug).collect();
		if matching.is_empty() {
			// Exempt iff a step-level waiver covers this step. W3 asks only about the
			// unit and identity; W5 judges whether the waiver is well-evidenced.
			let covered = waivers
				.iter()
				.any(|waiver| waiver.unit == WaiverUnit::Step && waiver.step == step.slug);
			if !covered {
				problems.push(format!(
					"Roadmap step `{}` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:\"waiver\"` for it if it predates logging or its review was skipped",
					step.slug
				));
			}
			continue;
		}

		// Group the step's records by increment (the full `task`), so each
		// increment's convergence is judged on its own terms.
		let mut increments: BTreeMap<&str, Vec<&Round>> = BTreeMap::new();
		for round in &matching {
			increments.entry(round_increment_id(round)).or_default().push(round);
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
				// Exempt this increment iff an increment-level waiver covers it (its
				// `increment` token equals this increment's full `task` AND its `step`
				// names this step, so a mis-scoped waiver pointing at a real-but-wrong
				// step exempts nothing). W3 checks only unit and identity; W5 judges the
				// waiver's evidence.
				let covered = waivers.iter().any(|waiver| {
					waiver.unit == WaiverUnit::Increment
						&& waiver.increment.as_deref() == Some(*increment)
						&& waiver.step == step.slug
				});
				if !covered {
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
	}
	problems
}

/// The W5 check: every `type:"waiver"` record must be well-formed as an exemption,
/// independent of whether W3 currently relies on it. Reports one problem per
/// violation:
///
/// - The waiver's `step` must name a real Roadmap step slug (a waiver for a step the
///   Roadmap does not track is dangling).
/// - An `increment`-unit waiver's `step` must own its `increment`
///   (`leading_slug(increment) == step`), so a waiver naming a real-but-wrong step
///   is reported rather than silently mis-scoped.
/// - A `record-backed` waiver's `evidence` must join to an existing
///   `type:"escalation"` record whose `human_decision` is `decision`, whose `task`
///   equals the evidence pointer, AND that is scoped to the waived unit (the
///   escalation's increment id equals the waived `increment`, or its step slug
///   equals the waived `step`, each preferring the escalation's structured Inc 2
///   id and falling back to the `leading_slug`/`task` shim), so a self-declaration
///   cannot cite an unrelated escalation to earn the strong tier.
/// - The `reason` <-> `evidence_tier` pairing must be consistent so a
///   self-declaration cannot claim the strong tier: `predates-logging` and
///   `review-skipped` MUST be `self-declared`; `accepted-at-escalation` MUST be
///   `record-backed`. Any other pairing is flagged.
///
/// W5 is orthogonal to W3: W3 asks "does a covering waiver exist?", W5 asks "is a
/// waiver well-evidenced?". A waiver can therefore be flagged by W5 while still
/// covering a step in W3 (the exemption is applied but its integrity is reported),
/// which keeps a malformed-but-present waiver visible rather than silently trusted.
fn w5_problems(
	waivers: &[Waiver],
	steps: &[Step],
	escalations: &[Escalation],
) -> Vec<String> {
	let slugs: BTreeSet<&str> = steps.iter().map(|step| step.slug.as_str()).collect();
	let mut problems = Vec::new();
	for waiver in waivers {
		// The waiver must name a real Roadmap step.
		if !slugs.contains(waiver.step.as_str()) {
			problems.push(format!(
				"round log line {}: `type:\"waiver\"` names step `{}`, which is not a Roadmap step",
				waiver.line, waiver.step
			));
		}
		// An increment-unit waiver's `step` must own its `increment`: the increment's
		// leading slug must equal the waiver's `step`. A mis-scoped waiver naming a
		// real-but-wrong step is reported here (and refused by W3).
		if waiver.unit == WaiverUnit::Increment {
			if let Some(increment) = waiver.increment.as_deref() {
				if leading_slug(increment) != waiver.step {
					problems.push(format!(
						"round log line {}: increment waiver names step `{}` but increment `{}` belongs to step `{}`",
						waiver.line,
						waiver.step,
						increment,
						leading_slug(increment)
					));
				}
			}
		}
		// A record-backed waiver's evidence must join to a real decision escalation
		// that is ALSO scoped to the waived unit, so a self-declaration cannot cite an
		// unrelated escalation to earn the strong tier: for an increment-unit waiver
		// the escalation's `task` must equal the waived `increment`; for a step-unit
		// waiver its leading slug must equal the waived `step`.
		if waiver.evidence_tier == EvidenceTier::RecordBacked {
			// `parse_waivers` guarantees `evidence` is present for the record-backed
			// tier, so a `None` here would already have been dropped; guard anyway.
			if let Some(evidence) = waiver.evidence.as_deref() {
				let backed = escalations.iter().any(|escalation| {
					if escalation.task != evidence
						|| escalation.human_decision != HumanDecision::Decision
					{
						return false;
					}
					// Tie the joined escalation to the unit the waiver exempts, preferring
					// the escalation's structured ids (Inc 2) over the `leading_slug`/`task`
					// shim when it carries them.
					match waiver.unit {
						WaiverUnit::Increment =>
							waiver.increment.as_deref() == Some(escalation_increment_id(escalation)),
						WaiverUnit::Step => escalation_step_slug(escalation) == waiver.step,
					}
				});
				if !backed {
					problems.push(format!(
						"round log line {}: `record-backed` waiver cites evidence `{}` but no `type:\"escalation\"` record with `human_decision` `decision` is scoped to this waiver's unit",
						waiver.line, evidence
					));
				}
			}
		}
		// The reason must be paired with the tier its integrity requires.
		let pairing_ok = match waiver.reason {
			WaiverReason::PredatesLogging | WaiverReason::ReviewSkipped =>
				waiver.evidence_tier == EvidenceTier::SelfDeclared,
			WaiverReason::AcceptedAtEscalation =>
				waiver.evidence_tier == EvidenceTier::RecordBacked,
		};
		if !pairing_ok {
			problems.push(format!(
				"round log line {}: waiver reason `{}` must not carry evidence tier `{}`",
				waiver.line,
				waiver.reason.label(),
				waiver.evidence_tier.label()
			));
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

	/// Parse a JSONL fixture into waivers via the metrics projection, so the tests
	/// exercise the same best-effort parse W3/W5 read (a malformed waiver never
	/// reaches these functions).
	fn waivers(jsonl: &str) -> Vec<Waiver> {
		metrics::parse_waivers(jsonl)
	}

	/// Parse a JSONL fixture into escalations via the metrics projection.
	fn escalations(jsonl: &str) -> Vec<Escalation> {
		metrics::parse_escalations(jsonl)
	}

	/// One minimal step-unit `waiver` log line for the given step, reason, and tier.
	fn step_waiver_line(
		step: &str,
		reason: &str,
		evidence_tier: &str,
	) -> String {
		format!(
			r#"{{"type":"waiver","task":"t","unit":"step","step":"{step}","reason":"{reason}","evidence_tier":"{evidence_tier}"}}"#
		)
	}

	/// One minimal increment-unit `waiver` log line naming its increment and the
	/// `evidence` pointer its record-backed tier requires.
	fn increment_waiver_line(
		step: &str,
		increment: &str,
		evidence: &str,
	) -> String {
		format!(
			r#"{{"type":"waiver","task":"t","unit":"increment","step":"{step}","increment":"{increment}","reason":"accepted-at-escalation","evidence_tier":"record-backed","evidence":"{evidence}"}}"#
		)
	}

	/// One minimal `escalation` log line with a `decision` outcome for the task.
	fn escalation_line(task: &str) -> String {
		format!(
			r#"{{"type":"escalation","task":"{task}","artifact":"a","human_decision":"decision"}}"#
		)
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

	/// Build a `round` log line carrying the Inc 2 structured `step`/`increment`
	/// ids, so the join tests can exercise the structured path (a record that joins
	/// without the lexical `leading_slug` strip).
	fn structured_round_line(
		task: &str,
		step: &str,
		increment: &str,
		outcome: &str,
		consecutive_clean: u64,
		risk_class: &str,
	) -> String {
		format!(
			r#"{{"type":"round","task":"{task}","artifact":"a","outcome":"{outcome}","consecutive_clean":{consecutive_clean},"risk_class":"{risk_class}","step":"{step}","increment":"{increment}"}}"#
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
	fn a_skipped_step_is_exempt() {
		let problems = w3_problems(&steps(&one_step_plan("dropped", "skipped")), &[], &[]);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn a_complete_step_with_no_records_but_a_covering_step_waiver_passes() {
		// A `complete` step with no rounds is exempt when a step-level waiver covers it
		// (the retired `grandfathered`/`trivial` cases, now one waiver notion). W3 keys
		// only on the unit and the step identity, not the waiver's reason or tier.
		let waivers = waivers(&step_waiver_line("legacy", "predates-logging", "self-declared"));
		let problems = w3_problems(&steps(&one_step_plan("legacy", "complete")), &[], &waivers);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn a_complete_step_with_no_records_and_no_covering_waiver_is_caught() {
		// The `pause.md` catch: marked `complete` with no matching rounds and no
		// covering step-waiver still fails. A waiver for a DIFFERENT step does not cover
		// it, so the exemption stays scoped to the named step.
		let waivers = waivers(&step_waiver_line("other", "predates-logging", "self-declared"));
		let problems = w3_problems(&steps(&one_step_plan("no-review", "complete")), &[], &waivers);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains(
				"`no-review` is `complete` but has no round records and no covering waiver"
			),
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
			w3_problems(&steps(&one_step_plan("round-log-core", "complete")), &rounds(&log), &[]);
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
		let problems = w3_problems(&steps(&one_step_plan("stall", "complete")), &rounds(&log), &[]);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("reached a consecutive-clean streak of 1")
				&& problems[0].contains("`risky` risk class needs 2"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn a_short_streak_increment_with_a_covering_increment_waiver_passes() {
		// The `optional-modules-inc2cii` shape: a risky increment accepted at ONE clean
		// round (peak 1, needs 2) at an escalation. An increment-level waiver naming its
		// full `task` exempts the shortfall, so W3 does not flag it.
		let log = [
			round_line("stall-incA", "AGENTS.md", "new_valid", 0, "risky"),
			round_line("stall-incA", "AGENTS.md", "clean", 1, "risky"),
		]
		.join("\n");
		let waivers = waivers(&increment_waiver_line("stall", "stall-incA", "stall-incA"));
		let problems =
			w3_problems(&steps(&one_step_plan("stall", "complete")), &rounds(&log), &waivers);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn a_step_waiver_does_not_exempt_a_short_streak_increment() {
		// The waiver units are distinct: a STEP-level waiver does not cover a short-streak
		// INCREMENT (that needs an increment-level waiver), so the shortfall still fails.
		let log = [
			round_line("stall-incA", "AGENTS.md", "new_valid", 0, "risky"),
			round_line("stall-incA", "AGENTS.md", "clean", 1, "risky"),
		]
		.join("\n");
		let waivers = waivers(&step_waiver_line("stall", "predates-logging", "self-declared"));
		let problems =
			w3_problems(&steps(&one_step_plan("stall", "complete")), &rounds(&log), &waivers);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("reached a consecutive-clean streak of 1"), "{}", problems[0]);
	}

	#[test]
	fn a_risk_class_inconsistency_is_not_suppressed_by_a_waiver() {
		// The risk_class-inconsistency error is a data-integrity fault: an increment-level
		// waiver covering the increment does NOT suppress it, so it still fails.
		let log = [
			round_line("mixup-incA", "AGENTS.md", "new_valid", 0, "low_risk"),
			round_line("mixup-incA", "AGENTS.md", "clean", 1, "risky"),
		]
		.join("\n");
		let waivers = waivers(&increment_waiver_line("mixup", "mixup-incA", "mixup-incA"));
		let problems =
			w3_problems(&steps(&one_step_plan("mixup", "complete")), &rounds(&log), &waivers);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("inconsistent risk_class"), "{}", problems[0]);
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
		let problems =
			w3_problems(&steps(&one_step_plan("converge", "complete")), &rounds(&log), &[]);
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
		let problems = w3_problems(&steps(&one_step_plan("short", "complete")), &rounds(&log), &[]);
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
		let problems = w3_problems(&steps(&one_step_plan("lr", "complete")), &rounds(&log), &[]);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("`low_risk` risk class needs 1"), "{}", problems[0]);
	}

	#[test]
	fn an_in_flight_step_with_rounds_is_not_checked() {
		// W3's guard is `status == "complete"`, so an in-flight step (here `in
		// progress`) is not checked even with matching rounds in the log. This pins
		// the guard against a future status-list refactor.
		let log = round_line("wip", "wip change", "new_valid", 0, "risky");
		let problems =
			w3_problems(&steps(&one_step_plan("wip", "in progress")), &rounds(&log), &[]);
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
		let problems = w3_problems(&steps(&one_step_plan("mixup", "complete")), &rounds(&log), &[]);
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
			"| `declared`       | skipped  |\n",
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

	#[test]
	fn w5_flags_a_waiver_naming_a_nonexistent_step() {
		// A waiver whose `step` does not resolve to a Roadmap slug is dangling.
		let steps = steps(&one_step_plan("real", "complete"));
		let waivers = waivers(&step_waiver_line("ghost", "predates-logging", "self-declared"));
		let problems = w5_problems(&waivers, &steps, &[]);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("names step `ghost`, which is not a Roadmap step"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w5_flags_a_record_backed_waiver_with_no_matching_escalation() {
		// A record-backed waiver whose `evidence` joins to no `decision` escalation is
		// flagged (the strong tier must be backed by a real human decision).
		let steps = steps(&one_step_plan("optional-modules", "complete"));
		let waivers = waivers(&increment_waiver_line(
			"optional-modules",
			"optional-modules-inc2cii",
			"optional-modules-inc2cii",
		));
		let problems = w5_problems(&waivers, &steps, &[]);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("cites evidence `optional-modules-inc2cii`")
				&& problems[0].contains("no `type:\"escalation\"` record"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w5_passes_a_record_backed_waiver_with_a_matching_escalation() {
		// The migration shape: an increment waiver whose evidence joins to a real
		// `decision` escalation passes W5.
		let steps = steps(&one_step_plan("optional-modules", "complete"));
		let waivers = waivers(&increment_waiver_line(
			"optional-modules",
			"optional-modules-inc2cii",
			"optional-modules-inc2cii",
		));
		let escalations = escalations(&escalation_line("optional-modules-inc2cii"));
		let problems = w5_problems(&waivers, &steps, &escalations);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w5_flags_each_inconsistent_reason_tier_pairing() {
		// The three forbidden pairings: a self-declared reason forced to record-backed,
		// and the escalation reason forced to self-declared. Each is flagged; the three
		// valid pairings are accepted below.
		let steps = steps(&one_step_plan("s", "complete"));
		// `predates-logging` may not be record-backed. `parse_waivers` requires an
		// `evidence` pointer for the record-backed tier, so include one; W5 then flags the
		// pairing (and, with no escalation, the missing evidence join, hence two problems).
		let bad_predates = r#"{"type":"waiver","task":"t","unit":"step","step":"s","reason":"predates-logging","evidence_tier":"record-backed","evidence":"x"}"#;
		let problems = w5_problems(&waivers(bad_predates), &steps, &[]);
		assert!(
			problems.iter().any(|p| p.contains(
				"reason `predates-logging` must not carry evidence tier `record-backed`"
			)),
			"{problems:?}"
		);
		// `review-skipped` may not be record-backed either.
		let bad_review = r#"{"type":"waiver","task":"t","unit":"step","step":"s","reason":"review-skipped","evidence_tier":"record-backed","evidence":"x"}"#;
		let problems = w5_problems(&waivers(bad_review), &steps, &[]);
		assert!(
			problems.iter().any(|p| p
				.contains("reason `review-skipped` must not carry evidence tier `record-backed`")),
			"{problems:?}"
		);
		// `accepted-at-escalation` may not be self-declared.
		let bad_escalation = r#"{"type":"waiver","task":"t","unit":"step","step":"s","reason":"accepted-at-escalation","evidence_tier":"self-declared"}"#;
		let problems = w5_problems(&waivers(bad_escalation), &steps, &[]);
		assert!(
			problems.iter().any(|p| p.contains(
				"reason `accepted-at-escalation` must not carry evidence tier `self-declared`"
			)),
			"{problems:?}"
		);
	}

	#[test]
	fn w5_accepts_the_three_valid_reason_tier_pairings() {
		// `predates-logging`/self-declared, `review-skipped`/self-declared, and
		// `accepted-at-escalation`/record-backed (with its escalation) are all accepted.
		let steps = steps(&one_step_plan("s", "complete"));
		let escalations = escalations(&escalation_line("s-inc1"));
		let log = [
			step_waiver_line("s", "predates-logging", "self-declared"),
			step_waiver_line("s", "review-skipped", "self-declared"),
			increment_waiver_line("s", "s-inc1", "s-inc1"),
		]
		.join("\n");
		let problems = w5_problems(&waivers(&log), &steps, &escalations);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn check_workflow_passes_the_optional_modules_migration_shape() {
		// End to end mirroring this repo's migration: `optional-modules` is `complete`
		// with a risky increment accepted at ONE clean round (peak 1, needs 2), unstuck by
		// a record-backed increment waiver whose evidence joins to the increment's real
		// `decision` escalation. W3 accepts it (covering waiver) and W5 accepts it (backed
		// by the escalation), so the whole cross-reference is green.
		let plan = concat!(
			"## Roadmap\n",
			"| Step               | Status   |\n",
			"| ------------------ | -------- |\n",
			"| `optional-modules` | complete |\n",
		);
		let log = [
			round_line("optional-modules-inc2cii", "a", "new_valid", 0, "risky"),
			round_line("optional-modules-inc2cii", "a", "clean", 1, "risky"),
			escalation_line("optional-modules-inc2cii"),
			increment_waiver_line(
				"optional-modules",
				"optional-modules-inc2cii",
				"optional-modules-inc2cii",
			),
		]
		.join("\n");
		let problems = check_workflow(plan, &log);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w5_flags_a_record_backed_waiver_whose_escalation_resumed_not_decided() {
		// S3: an escalation exists with the matching `task` but `human_decision:"resume"`
		// (not `decision`), so the record-backed join is not satisfied and the waiver is
		// still flagged. `escalation_line` only emits `decision`, so build the raw line here.
		let steps = steps(&one_step_plan("optional-modules", "complete"));
		let waivers = waivers(&increment_waiver_line(
			"optional-modules",
			"optional-modules-inc2cii",
			"optional-modules-inc2cii",
		));
		let resume = r#"{"type":"escalation","task":"optional-modules-inc2cii","artifact":"a","human_decision":"resume"}"#;
		let problems = w5_problems(&waivers, &steps, &escalations(resume));
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("cites evidence `optional-modules-inc2cii`")
				&& problems[0].contains("is scoped to this waiver's unit"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w5_passes_a_step_unit_record_backed_waiver_joined_by_leading_slug() {
		// The step-unit join branch: a `step`-unit `accepted-at-escalation`/`record-backed`
		// waiver whose `evidence` names a `decision` escalation whose `leading_slug(task)`
		// equals the waived `step` satisfies the record-backed join and passes W5. (The
		// migration only exercises the increment-unit branch, so pin the step-unit one here.)
		let steps = steps(&one_step_plan("optional-modules", "complete"));
		let waiver = r#"{"type":"waiver","task":"t","unit":"step","step":"optional-modules","reason":"accepted-at-escalation","evidence_tier":"record-backed","evidence":"optional-modules-inc1"}"#;
		let escalations = escalations(&escalation_line("optional-modules-inc1"));
		let problems = w5_problems(&waivers(waiver), &steps, &escalations);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w5_flags_a_step_unit_record_backed_waiver_whose_escalation_names_a_different_step() {
		// The step-unit join is unit-scoped: a `decision` escalation whose `leading_slug(task)`
		// names a DIFFERENT step does not back a `step`-unit waiver, so the waiver is flagged
		// even though the escalation is a real human decision. This mirrors the increment-unit
		// unrelated-escalation test for the step-unit branch.
		let steps = steps(&one_step_plan("optional-modules", "complete"));
		let waiver = r#"{"type":"waiver","task":"t","unit":"step","step":"optional-modules","reason":"accepted-at-escalation","evidence_tier":"record-backed","evidence":"other-step-inc1"}"#;
		let escalations = escalations(&escalation_line("other-step-inc1"));
		let problems = w5_problems(&waivers(waiver), &steps, &escalations);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("cites evidence `other-step-inc1`")
				&& problems[0].contains("is scoped to this waiver's unit"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w5_flags_a_record_backed_waiver_citing_an_unrelated_escalation() {
		// Group A (O1): the joined escalation must be scoped to the waived unit. A
		// record-backed increment waiver whose `evidence` names an escalation for a
		// DIFFERENT task is flagged, even though that escalation is a real `decision`, so
		// an unrelated decision cannot launder a weak self-declaration into the strong tier.
		let steps = steps(&one_step_plan("optional-modules", "complete"));
		let waiver =
			increment_waiver_line("optional-modules", "optional-modules-inc2cii", "unrelated-task");
		let escalations = escalations(&escalation_line("unrelated-task"));
		let problems = w5_problems(&waivers(&waiver), &steps, &escalations);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("cites evidence `unrelated-task`")
				&& problems[0].contains("is scoped to this waiver's unit"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment() {
		// O3: for an increment-unit waiver `leading_slug(increment)` must equal `step`. A
		// waiver naming a real-but-wrong step (`alpha`) for an increment belonging to
		// `beta` is reported, so a mis-scoped waiver cannot hide behind a real slug.
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status   |\n",
			"| ------- | -------- |\n",
			"| `alpha` | complete |\n",
			"| `beta`  | complete |\n",
		);
		let waiver = increment_waiver_line("alpha", "beta-incB", "beta-incB");
		let escalations = escalations(&escalation_line("beta-incB"));
		let problems = w5_problems(&waivers(&waiver), &steps(plan), &escalations);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("increment waiver names step `alpha`")
				&& problems[0].contains("increment `beta-incB` belongs to step `beta`"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment() {
		// O3 in W3: an increment waiver whose `step` names a real-but-wrong step no longer
		// exempts the shortfall, because W3 now cross-checks `waiver.step == step.slug`.
		// `alpha` is `skipped` (not W3-checked) so only the `beta` shortfall can be flagged.
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status   |\n",
			"| ------- | -------- |\n",
			"| `alpha` | skipped  |\n",
			"| `beta`  | complete |\n",
		);
		let log = [
			round_line("beta-incB", "a", "new_valid", 0, "risky"),
			round_line("beta-incB", "a", "clean", 1, "risky"),
		]
		.join("\n");
		let waivers = waivers(&increment_waiver_line("alpha", "beta-incB", "beta-incB"));
		let problems = w3_problems(&steps(plan), &rounds(&log), &waivers);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("reached a consecutive-clean streak of 1"), "{}", problems[0]);
	}

	#[test]
	fn a_bare_slug_increment_waiver_exempts_a_short_streak() {
		// S4: the migration's b2 shape uses an `increment` equal to the bare step slug (no
		// `-inc` suffix), matching a `task` with no suffix that `leading_slug` returns
		// whole. Pin it: a short risky increment `bare-step` is exempted by an increment
		// waiver whose `step` and `increment` are both the bare slug `bare-step`.
		let log = [
			round_line("bare-step", "a", "new_valid", 0, "risky"),
			round_line("bare-step", "a", "clean", 1, "risky"),
		]
		.join("\n");
		let waivers = waivers(&increment_waiver_line("bare-step", "bare-step", "bare-step"));
		let problems =
			w3_problems(&steps(&one_step_plan("bare-step", "complete")), &rounds(&log), &waivers);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w3_a_round_carrying_a_structured_step_joins_without_the_lexical_strip() {
		// Inc 2 acceptance (structured path): `foo-incidental` ends `-inc<alnum>`, so
		// `leading_slug` over-strips it to `foo` (the T3 risk). A round carrying the
		// structured `step`/`increment` ids joins to the `complete` `foo-incidental`
		// step on the declared slug directly, so the step converges with no problem.
		assert_eq!(leading_slug("foo-incidental"), "foo", "the shim would misroute this task");
		let log = structured_round_line(
			"foo-incidental",
			"foo-incidental",
			"foo-incidental",
			"clean",
			1,
			"low_risk",
		);
		let problems =
			w3_problems(&steps(&one_step_plan("foo-incidental", "complete")), &rounds(&log), &[]);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w3_the_same_task_without_the_structured_step_over_strips_and_is_missed() {
		// The companion showing the structured id, not a change to the shim, is what
		// fixes the join: the SAME `foo-incidental` task WITHOUT the field falls back to
		// `leading_slug`, which strips it to `foo`, so the `complete` `foo-incidental`
		// step sees no matching rounds and is caught by the pause.md catch.
		let log = round_line("foo-incidental", "a", "clean", 1, "low_risk");
		let problems =
			w3_problems(&steps(&one_step_plan("foo-incidental", "complete")), &rounds(&log), &[]);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("has no round records and no covering waiver"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w3_a_pre_migration_round_still_joins_its_step_via_leading_slug() {
		// Inc 2 acceptance (fallback path preserved): a pre-migration round (no
		// structured ids) for `state-schema-inc1` still joins to the `complete`
		// `state-schema` step via the `leading_slug` shim and converges.
		let log = round_line("state-schema-inc1", "a", "clean", 1, "low_risk");
		let problems =
			w3_problems(&steps(&one_step_plan("state-schema", "complete")), &rounds(&log), &[]);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w3_an_increment_only_round_falls_back_to_the_shim_on_the_unfilled_step_axis() {
		// O1 (single-field join, unfilled STEP axis): the `step`/`increment` ids are
		// independently optional, so a round may carry `increment` WITHOUT `step`. On
		// the unfilled step axis `round_step_slug` falls back to the `leading_slug`
		// shim, which over-strips `foo-incidental` to `foo` (T3). Pin this as the
		// CHOSEN outcome, not a bug: the `complete` `foo-incidental` step sees no
		// matching rounds (the round joined to `foo`) and is caught by the pause.md
		// catch, exactly as a fieldless record would be. The present `increment` id
		// does nothing for the step axis; the axes fall back independently.
		assert_eq!(leading_slug("foo-incidental"), "foo", "the shim over-strips this task");
		let log = r#"{"type":"round","task":"foo-incidental","artifact":"a","outcome":"clean","consecutive_clean":1,"risk_class":"low_risk","increment":"foo-incidental"}"#;
		let problems =
			w3_problems(&steps(&one_step_plan("foo-incidental", "complete")), &rounds(log), &[]);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("has no round records and no covering waiver"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w3_a_step_only_round_joins_on_its_structured_step_and_falls_back_on_the_increment_axis() {
		// O1 (single-field join, filled STEP axis): the mirror case, a round carrying
		// `step` WITHOUT `increment`. On the filled step axis `round_step_slug` uses
		// the structured `foo-incidental` id and joins to the `complete`
		// `foo-incidental` step directly (no over-strip), so the step converges. On
		// the unfilled increment axis `round_increment_id` falls back to the raw
		// `task` (`foo-incidental`) as its grouping key. Pin that the filled axis uses
		// the structured id while the unfilled axis independently uses the `task`
		// shim, with no coupling between the two.
		let log = r#"{"type":"round","task":"foo-incidental","artifact":"a","outcome":"clean","consecutive_clean":1,"risk_class":"low_risk","step":"foo-incidental"}"#;
		let problems =
			w3_problems(&steps(&one_step_plan("foo-incidental", "complete")), &rounds(log), &[]);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w5_a_record_backed_waiver_joins_via_the_escalations_structured_step() {
		// Inc 2 acceptance for escalation (structured path): a step-unit
		// `accepted-at-escalation` waiver for `foo-incidental` (a slug `leading_slug`
		// over-strips to `foo`). Its escalation carries the structured `step` id, so
		// W5's step-unit scope join matches the declared slug directly and passes.
		let steps = steps(&one_step_plan("foo-incidental", "complete"));
		let waiver = r#"{"type":"waiver","task":"t","unit":"step","step":"foo-incidental","reason":"accepted-at-escalation","evidence_tier":"record-backed","evidence":"foo-incidental"}"#;
		let escalation = r#"{"type":"escalation","task":"foo-incidental","artifact":"a","human_decision":"decision","step":"foo-incidental","increment":"foo-incidental"}"#;
		let problems = w5_problems(&waivers(waiver), &steps, &escalations(escalation));
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w5_without_the_structured_step_the_escalation_over_strips_and_is_missed() {
		// The escalation companion: the same escalation WITHOUT the structured `step`
		// falls back to `leading_slug("foo-incidental") == "foo"`, which does not equal
		// the waived step `foo-incidental`, so the record-backed join is not satisfied
		// and W5 flags the waiver as unscoped.
		let steps = steps(&one_step_plan("foo-incidental", "complete"));
		let waiver = r#"{"type":"waiver","task":"t","unit":"step","step":"foo-incidental","reason":"accepted-at-escalation","evidence_tier":"record-backed","evidence":"foo-incidental"}"#;
		let escalation = r#"{"type":"escalation","task":"foo-incidental","artifact":"a","human_decision":"decision"}"#;
		let problems = w5_problems(&waivers(waiver), &steps, &escalations(escalation));
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("is scoped to this waiver's unit"), "{}", problems[0]);
	}
}
