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
			Baseline,
			Decision,
			Escalation,
			EvidenceTier,
			HumanDecision,
			Round,
			RoundOutcome,
			Waiver,
			WaiverUnit,
			question_id_index,
		},
		plan::{
			self,
			QUEUE_FOLD_PREFIX,
			Question,
			Step,
			source::PlanToml,
		},
		workflow_spec::WorkflowSpec,
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
pub(crate) fn round_step_slug(round: &Round) -> &str {
	round.step.as_deref().unwrap_or_else(|| leading_slug(&round.task))
}

/// The increment id a round belongs to: its STRUCTURED `increment` id when the
/// record carries one, else its `task` verbatim (the pre-migration shim). This
/// is the identity the convergence streak is counted per, and Inc 4 will join it
/// to the TOML `[[step.increment]].id`.
pub(crate) fn round_increment_id(round: &Round) -> &str {
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

/// Cross-reference the MARKDOWN-sourced plan against the round log, returning one
/// human-readable problem per violation (an empty vector means the workflow
/// invariants hold). This is the default source: the steps, questions, waivers, and
/// baseline all come from the Markdown plan and the JSONL log, exactly as before Inc
/// 4. The `check_workflow_toml` sibling reads the same checks from a TOML source; both
/// funnel into `run_checks`, so the four checks stay one implementation and only their
/// input source differs (Principle 16). A repo with no `<task>.plan.toml`, or one whose
/// `[meta].primary` is `markdown`, uses this path and is byte-for-byte unaffected.
pub(crate) fn check_workflow(
	spec: &WorkflowSpec,
	plan_markdown: &str,
	log_contents: &str,
) -> Vec<String> {
	run_checks(
		spec,
		&plan::parse_roadmap(plan_markdown),
		&plan::parse_questions(plan_markdown),
		&metrics::parse_rounds(log_contents),
		&metrics::parse_decisions(log_contents),
		&metrics::parse_baseline(log_contents),
		&metrics::parse_waivers(log_contents),
		&metrics::parse_escalations(log_contents),
	)
}

/// Cross-reference a TOML-sourced plan (`[meta].primary == "toml"`) against the round
/// log, the Inc 4 (Q-46) source swap. The steps, questions, WAIVERS, and the W4
/// baseline are projected from the `<task>.plan.toml` (`step_views`/`question_views`,
/// `waivers_from_toml`, `baseline_from_toml`); the rounds, decisions, and escalations
/// still come from the JSONL log, since those are genuine append-only events that keep
/// a JSONL home. W5's record-backed join is therefore now CROSS-SUBSTRATE: a mutable
/// TOML `[[step.waiver]]` joined to the immutable JSONL `escalation` it cites. The
/// checks themselves are unchanged (`run_checks`); only the waiver/baseline/step/
/// question inputs are re-sourced, so the pause.md catch and the un-launderable
/// two-tier property hold identically across substrates.
pub(crate) fn check_workflow_toml(
	spec: &WorkflowSpec,
	plan: &PlanToml,
	log_contents: &str,
) -> Vec<String> {
	run_checks(
		spec,
		&plan.step_views(),
		&plan.question_views(),
		&metrics::parse_rounds(log_contents),
		&metrics::parse_decisions(log_contents),
		&baseline_from_toml(plan),
		&waivers_from_toml(plan),
		&metrics::parse_escalations(log_contents),
	)
}

/// Run the four cross-reference checks over already-sourced inputs, so the same check
/// logic serves both the Markdown+JSONL and the TOML substrates (Principle 16, one
/// implementation): the round-log internal-consistency check (over every round
/// record), W3 (step convergence OR a covering waiver), W4 (decided-item decision
/// receipts), and W5 (waiver integrity, incl. the cross-substrate escalation join).
#[expect(
	clippy::too_many_arguments,
	reason = "the single check funnel takes each already-sourced input plus the control spec; grouping them into a struct would only relocate the same fields"
)]
fn run_checks(
	spec: &WorkflowSpec,
	steps: &[Step],
	questions: &[Question],
	rounds: &[Round],
	decisions: &[Decision],
	baselines: &[Baseline],
	waivers: &[Waiver],
	escalations: &[Escalation],
) -> Vec<String> {
	let mut problems = round_log_consistency_problems(rounds);
	problems.extend(w3_problems(spec, steps, rounds, waivers));
	problems.extend(w4_problems(questions, decisions, baselines));
	problems.extend(w5_problems(waivers, steps, rounds, escalations));
	problems
}

/// Flatten a TOML plan's nested `[[step.waiver]]` entries into the flat
/// `metrics::Waiver` shape W3/W5 consume, so the same waiver checks run over either
/// substrate (Principle 16). Each waiver's `step` is supplied by the step it nests on
/// (the nesting replaces the JSONL `step` field). This mirrors `metrics::parse_waivers`
/// best-effort PRESENCE filtering: a waiver breaking the `increment` (present iff
/// `unit == increment`) or `evidence` (present iff `record-backed`) rule is DROPPED,
/// exactly as the JSONL projection drops it, so a malformed TOML waiver can never
/// silently grant a W3 exemption (`validate --source` REPORTS these; the projection the
/// checks read DROPS them, the synthesis invariant). The `reason` <-> `evidence_tier`
/// pairing is NOT filtered here (it is W5's job to report, matching `parse_waivers`).
/// The `locator` field carries the waiver's `[[step.waiver]].id`: a TOML waiver has no
/// JSONL log line, so its W5 message names it by that stable id (`TOML waiver <id>`)
/// instead of asserting a false `round log line N`, while a JSONL waiver keeps its log
/// line (one `w5_problems`, correct per substrate).
fn waivers_from_toml(plan: &PlanToml) -> Vec<Waiver> {
	let mut waivers = Vec::new();
	for step in &plan.steps {
		for waiver in &step.waivers {
			let increment = waiver.increment.as_deref().filter(|token| !token.is_empty());
			let increment = match (waiver.unit, increment) {
				(WaiverUnit::Increment, Some(token)) => Some(token.to_string()),
				(WaiverUnit::Increment, None) => continue,
				(WaiverUnit::Step, None) => None,
				(WaiverUnit::Step, Some(_)) => continue,
			};
			let evidence = waiver.evidence.as_deref().filter(|pointer| !pointer.is_empty());
			let evidence = match (waiver.evidence_tier, evidence) {
				(EvidenceTier::RecordBacked, Some(pointer)) => Some(pointer.to_string()),
				(EvidenceTier::RecordBacked, None) => continue,
				(EvidenceTier::SelfDeclared, None) => None,
				(EvidenceTier::SelfDeclared, Some(_)) => continue,
			};
			waivers.push(Waiver {
				locator: format!("TOML waiver `{}`", waiver.id),
				unit: waiver.unit,
				step: step.slug.clone(),
				increment,
				reason: waiver.reason,
				evidence_tier: waiver.evidence_tier,
				evidence,
			});
		}
	}
	waivers
}

/// Project the TOML `[meta].w4_baseline` cutoff into the `metrics::Baseline` shape W4
/// consumes: at most one baseline (W4 resolves last-one-wins, so a single element is
/// equivalent). An absent or non-`Q-<n>` cutoff yields NO baseline, so W4 then requires
/// a decision receipt for every decided item, the same safe direction as a fresh repo
/// with no baseline record. The `line` field is unused by W4 (it reads only the cutoff)
/// so it carries a placeholder.
fn baseline_from_toml(plan: &PlanToml) -> Vec<Baseline> {
	plan.meta
		.w4_baseline
		.as_deref()
		.and_then(question_id_index)
		.map(|questions_through| Baseline {
			line: 0,
			questions_through,
		})
		.into_iter()
		.collect()
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

/// The peak consecutive-clean streak over a set of round records: the maximum
/// logged `consecutive_clean` value, or `0` when the slice is empty. This is the
/// convergence arithmetic W3 checks a `complete` increment against, extracted so the
/// forward `agent-scaffold next` projection and this backward check run the SAME
/// computation over the same records rather than two copies that could drift
/// (Principle 16). `consecutive_clean` is one running per-loop counter spanning the
/// artifacts an increment's rounds name, so the peak (not the terminal value) is
/// taken: in a correctly-run loop the loop stops at convergence so the peak equals
/// the terminal value, and taking the peak lets a converged increment pass
/// regardless of any trailing bookkeeping rounds.
pub(crate) fn peak_consecutive_clean(records: &[&Round]) -> u64 {
	records.iter().map(|round| round.consecutive_clean).max().unwrap_or(0)
}

/// Whether `waiver` exempts the increment `round` belongs to, as the ROUND LOG
/// attributes it: an increment-unit waiver whose `increment` is the round's increment
/// id and whose `step` is the step that round joins to (`round_increment_id` and
/// `round_step_slug`, so a record carrying the structured Inc 2 ids joins on them and a
/// pre-migration record falls back per axis).
///
/// W3 and W5 both consult this one implementation, so the two cannot drift on what "this
/// step owns this increment" means (Principle 16). W3 asks it of a `complete` step's own
/// records, to decide whether a covering waiver exempts a short streak; W5 asks it of
/// EVERY round, to decide whether any record backs the ownership the waiver asserts.
///
/// It takes the round rather than a step slug and an increment id the caller supplies,
/// so the step axis can only come from the record. A caller cannot pass the waiver's own
/// `step` and collapse the comparison into comparing a value with itself, which is the
/// mutation acceptance check 4b exists to catch (Principle 13).
fn waiver_covers_round(
	waiver: &Waiver,
	round: &Round,
) -> bool {
	waiver.unit == WaiverUnit::Increment
		&& waiver.increment.as_deref() == Some(round_increment_id(round))
		&& waiver.step == round_step_slug(round)
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
pub(crate) fn w3_problems(
	spec: &WorkflowSpec,
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
			let required = spec.required_streak(class);
			// The streak is per loop (per increment), not per artifact:
			// `consecutive_clean` is one running counter across the different
			// artifacts the increment's rounds name, so take the peak
			// consecutive_clean over ALL of the increment's records and require that
			// single peak to reach the class count.
			//
			// Peak, not terminal (T9): see `peak_consecutive_clean`, which owns this
			// computation so `agent-scaffold next` runs the identical arithmetic.
			let peak = peak_consecutive_clean(records);
			if peak < required {
				// Exempt this increment iff an increment-level waiver covers it, judged by
				// the shared `waiver_covers_round` predicate over the increment's own
				// records: the waiver must name this increment AND the step those records
				// join to, so a mis-scoped waiver pointing at a real-but-wrong step exempts
				// nothing. Every record in the group carries this increment and this step,
				// so asking any one of them asks the group. W3 checks only unit and
				// identity; W5 judges the waiver's evidence.
				let covered = waivers
					.iter()
					.any(|waiver| records.iter().any(|round| waiver_covers_round(waiver, round)));
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

/// The phrase naming the steps the round log joins an increment to, for W5's ownership
/// refusal: "step `a`" for one owner and "steps `a`, `b`" for several. Each owner maps to
/// whether ANY record declared it in a structured `step` id; one no record declared is
/// marked as derived from a record's `task`, because `round_step_slug` computed it
/// through the `leading_slug` shim and it need not be a Roadmap step or occur anywhere in
/// the log. Without that mark the refusal would present a computed value as something the
/// records carry.
///
/// SEVERAL OWNERS ARISE TWO WAYS AND NEITHER NEEDS A MALFORMED LOG. Two records for one
/// increment may carry different structured `step` ids, which the JSONL substrate permits
/// because a record's `step` is a free string. Or one record may carry a structured `step`
/// while another is pre-migration, in which case the second resolves through
/// `leading_slug(task)` and can land on a different value.
fn step_attribution(owners: &BTreeMap<&str, bool>) -> String {
	let list = owners
		.iter()
		.map(|(slug, declared)| {
			if *declared {
				format!("`{slug}`")
			} else {
				format!("`{slug}` (derived from a record's `task`)")
			}
		})
		.collect::<Vec<_>>()
		.join(", ");
	if owners.len() == 1 {
		format!("step {list}")
	} else {
		format!("steps {list}")
	}
}

/// The W5 check: every `type:"waiver"` record must be well-formed as an exemption,
/// independent of whether W3 currently relies on it. Reports one problem per
/// violation:
///
/// - The waiver's `step` must name a real Roadmap step slug (a waiver for a step the
///   Roadmap does not track is dangling).
/// - An `increment`-unit waiver's `step` must own its `increment`, judged against the
///   ROUND LOG through the shared `waiver_covers_round` predicate: some `type:"round"`
///   record must resolve to that increment id AND join to that step, so a waiver naming a
///   real-but-wrong step is reported rather than silently mis-scoped. Q-70 decided this
///   relation against the log rather than against the WAIVED INCREMENT ID's leading slug.
///   BOTH AXES STILL DEGRADE PER RECORD, per the accessor block above: a record carrying
///   the structured Inc 2 ids joins on them, while a pre-migration record resolves its
///   increment through `task` and its step through `leading_slug(task)`. So the step a
///   refusal names is READ from a record's `step` id where one exists and is DERIVED
///   otherwise, and a derived value need not be a Roadmap step or occur anywhere in the
///   log. The message marks which, rather than presenting either as what the records
///   carry; retiring the derivation itself would mean changing `round_step_slug`, which
///   W3 shares, and no decision has asked for that.
///   An increment NO record resolves to is REPORTED too (receipt `Q-70-emptycase`): the
///   log joins it to no step, so nothing evidences the ownership the waiver asserts. That
///   NARROWS what a waiver may cover against the retired lexical rule, which accepted such
///   a waiver silently.
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
	rounds: &[Round],
	escalations: &[Escalation],
) -> Vec<String> {
	let slugs: BTreeSet<&str> = steps.iter().map(|step| step.slug.as_str()).collect();
	let mut problems = Vec::new();
	for waiver in waivers {
		// The waiver must name a real Roadmap step.
		if !slugs.contains(waiver.step.as_str()) {
			problems.push(format!(
				"{}: `type:\"waiver\"` names step `{}`, which is not a Roadmap step",
				waiver.locator, waiver.step
			));
		}
		// An increment-unit waiver's `step` must own its `increment`, evidenced by the
		// round log rather than by the WAIVED INCREMENT ID's leading slug (Q-70). A
		// mis-scoped waiver naming a real-but-wrong step is reported here (and refused by
		// W3), and so is a waiver no round record resolves to, which owns nothing yet.
		if waiver.unit == WaiverUnit::Increment {
			if let Some(increment) = waiver.increment.as_deref() {
				if !rounds.iter().any(|round| waiver_covers_round(waiver, round)) {
					// Each step the log DOES join this increment to, mapped to whether any
					// record DECLARED that step in a structured `step` id. `round_step_slug`
					// prefers that id and falls back to `leading_slug(round.task)` for a
					// pre-migration record, so an owner no record declares was computed by
					// the join rather than carried by the log. The message marks the
					// difference, so the refusal never presents a computed step as one the
					// records state.
					let mut owners: BTreeMap<&str, bool> = BTreeMap::new();
					for round in
						rounds.iter().filter(|round| round_increment_id(round) == increment)
					{
						let declared = round.step.is_some();
						owners
							.entry(round_step_slug(round))
							.and_modify(|seen| *seen |= declared)
							.or_insert(declared);
					}
					if owners.is_empty() {
						problems.push(format!(
							"{}: increment waiver names increment `{}`, which no `type:\"round\"` record resolves to (a record resolves to its structured `increment` id, or to its `task` when that id is absent), so the round log joins it to no step",
							waiver.locator, increment
						));
					} else {
						problems.push(format!(
							"{}: increment waiver names step `{}` but the round log joins increment `{}` to {}",
							waiver.locator,
							waiver.step,
							increment,
							step_attribution(&owners)
						));
					}
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
						"{}: `record-backed` waiver cites evidence `{}` but no `type:\"escalation\"` record with `human_decision` `decision` is scoped to this waiver's unit",
						waiver.locator, evidence
					));
				}
			}
		}
		// The reason must be paired with the tier its integrity requires. The pairing
		// rule is single-sourced on `WaiverReason::required_tier`, shared with the TOML
		// waiver check in `plan::source`, so the two cannot drift (Principle 16).
		let pairing_ok = waiver.reason.required_tier() == waiver.evidence_tier;
		if !pairing_ok {
			problems.push(format!(
				"{}: waiver reason `{}` must not carry evidence tier `{}`",
				waiver.locator,
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

	/// One `round` record joining `increment` to `step` on the Inc 2 structured ids: the
	/// evidence W5's round-log ownership rule (Q-70) reads for an increment-unit waiver.
	/// That rule consults only the two join axes, so the outcome, streak and risk class
	/// are filler here, and the structured ids state the ownership the fixture means
	/// rather than leaving it to the `leading_slug` shim.
	fn owning_round_line(
		step: &str,
		increment: &str,
	) -> String {
		structured_round_line(increment, step, increment, "clean", 1, "low_risk")
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("dropped", "skipped")),
			&[],
			&[],
		);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn a_complete_step_with_no_records_but_a_covering_step_waiver_passes() {
		// A `complete` step with no rounds is exempt when a step-level waiver covers it
		// (the retired `grandfathered`/`trivial` cases, now one waiver notion). W3 keys
		// only on the unit and the step identity, not the waiver's reason or tier.
		let waivers = waivers(&step_waiver_line("legacy", "predates-logging", "self-declared"));
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("legacy", "complete")),
			&[],
			&waivers,
		);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn a_complete_step_with_no_records_and_no_covering_waiver_is_caught() {
		// The `pause.md` catch: marked `complete` with no matching rounds and no
		// covering step-waiver still fails. A waiver for a DIFFERENT step does not cover
		// it, so the exemption stays scoped to the named step.
		let waivers = waivers(&step_waiver_line("other", "predates-logging", "self-declared"));
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("no-review", "complete")),
			&[],
			&waivers,
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("round-log-core", "complete")),
			&rounds(&log),
			&[],
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("stall", "complete")),
			&rounds(&log),
			&[],
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("stall", "complete")),
			&rounds(&log),
			&waivers,
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("stall", "complete")),
			&rounds(&log),
			&waivers,
		);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("reached a consecutive-clean streak of 1"), "{}", problems[0]);
	}

	#[test]
	fn an_increment_waiver_does_not_exempt_a_sibling_increment_of_the_same_step() {
		// THE INCREMENT AXIS OF THE SHARED PREDICATE, on W3's side (round 1, `W1A-1`). One
		// `complete` step carries two increments: `stall-incB` converged and carries the
		// only waiver, `stall-incA` is short. The waiver names the WRONG increment of the
		// RIGHT step, so it must exempt nothing and the shortfall must still be reported.
		//
		// Its siblings pin the other two axes and neither reaches this one:
		// `a_step_waiver_does_not_exempt_a_short_streak_increment` pins the UNIT axis, and
		// `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment` pins the
		// STEP axis. Without this case a build that dropped
		// `waiver_covers_round`'s increment comparison would report `workflow invariants
		// hold` at exit 0 over an unconverged `risky` increment, with the whole suite green.
		let log = [
			round_line("stall-incA", "AGENTS.md", "new_valid", 0, "risky"),
			round_line("stall-incA", "AGENTS.md", "clean", 1, "risky"),
			round_line("stall-incB", "AGENTS.md", "clean", 1, "risky"),
			round_line("stall-incB", "AGENTS.md", "clean", 2, "risky"),
		]
		.join("\n");
		let waivers = waivers(&increment_waiver_line("stall", "stall-incB", "stall-incB"));
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("stall", "complete")),
			&rounds(&log),
			&waivers,
		);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("increment `stall-incA` reached a consecutive-clean streak of 1"),
			"{}",
			problems[0]
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("mixup", "complete")),
			&rounds(&log),
			&waivers,
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("converge", "complete")),
			&rounds(&log),
			&[],
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("short", "complete")),
			&rounds(&log),
			&[],
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("lr", "complete")),
			&rounds(&log),
			&[],
		);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("`low_risk` risk class needs 1"), "{}", problems[0]);
	}

	/// A `workflow.toml` spec that raises the `low_risk` streak to 2, used by the
	/// spec-driven-bar tests to prove the convergence constant is genuinely read from
	/// the spec rather than hardcoded.
	fn low_risk_needs_two_spec() -> WorkflowSpec {
		WorkflowSpec::parse(
			"[convergence]\nlow_risk = 2\nrisky = 2\n\n[rounds]\ncap = 5\n\n[backstop]\nseverity = \"high\"\n",
		)
		.unwrap()
	}

	#[test]
	fn an_altered_required_streak_raises_the_bar_w3_applies() {
		// A `low_risk` increment with a single clean round (peak 1) converges under the
		// built-in spec (needs 1) but FLAGS under a spec whose `low_risk` streak is
		// raised to 2, proving W3's bar is genuinely spec-driven, not hardcoded.
		let log = round_line("lr", "lr change", "clean", 1, "low_risk");
		let plan = one_step_plan("lr", "complete");

		// Built-in bar: one clean round converges the low_risk increment.
		assert!(
			w3_problems(&WorkflowSpec::builtin(), &steps(&plan), &rounds(&log), &[]).is_empty(),
			"one clean round should converge low_risk under the built-in spec"
		);

		// Raised bar: the same single clean round now falls short of 2.
		let problems = w3_problems(&low_risk_needs_two_spec(), &steps(&plan), &rounds(&log), &[]);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("reached a consecutive-clean streak of 1")
				&& problems[0].contains("`low_risk` risk class needs 2"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn check_workflow_threads_the_spec_end_to_end() {
		// The full Markdown check honours the spec threaded through it: a low_risk
		// increment that converges at the built-in bar flags under the raised bar, so
		// the `--workflow-spec` plumbing genuinely reaches W3.
		let log = round_line("lr", "lr change", "clean", 1, "low_risk");
		let plan = one_step_plan("lr", "complete");
		assert!(
			check_workflow(&WorkflowSpec::builtin(), &plan, &log).is_empty(),
			"the built-in spec converges the low_risk increment"
		);
		let problems = check_workflow(&low_risk_needs_two_spec(), &plan, &log);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("`low_risk` risk class needs 2"), "{}", problems[0]);
	}

	#[test]
	fn an_in_flight_step_with_rounds_is_not_checked() {
		// W3's guard is `status == "complete"`, so an in-flight step (here `in
		// progress`) is not checked even with matching rounds in the log. This pins
		// the guard against a future status-list refactor.
		let log = round_line("wip", "wip change", "new_valid", 0, "risky");
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("wip", "in progress")),
			&rounds(&log),
			&[],
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("mixup", "complete")),
			&rounds(&log),
			&[],
		);
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
		let problems = check_workflow(&WorkflowSpec::builtin(), plan, &log);
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
		let problems = w5_problems(&waivers, &steps, &[], &[]);
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
		// flagged (the strong tier must be backed by a real human decision). The round
		// records establish the waiver's ownership, so the ONE problem asserted below is
		// the evidence join and not Q-70's ownership rule.
		let steps = steps(&one_step_plan("optional-modules", "complete"));
		let waivers = waivers(&increment_waiver_line(
			"optional-modules",
			"optional-modules-inc2cii",
			"optional-modules-inc2cii",
		));
		let log = owning_round_line("optional-modules", "optional-modules-inc2cii");
		let problems = w5_problems(&waivers, &steps, &rounds(&log), &[]);
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
		// `decision` escalation, and whose increment the round log joins to the waived
		// step, passes W5.
		let steps = steps(&one_step_plan("optional-modules", "complete"));
		let waivers = waivers(&increment_waiver_line(
			"optional-modules",
			"optional-modules-inc2cii",
			"optional-modules-inc2cii",
		));
		let log = owning_round_line("optional-modules", "optional-modules-inc2cii");
		let escalations = escalations(&escalation_line("optional-modules-inc2cii"));
		let problems = w5_problems(&waivers, &steps, &rounds(&log), &escalations);
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
		let problems = w5_problems(&waivers(bad_predates), &steps, &[], &[]);
		assert!(
			problems.iter().any(|p| p.contains(
				"reason `predates-logging` must not carry evidence tier `record-backed`"
			)),
			"{problems:?}"
		);
		// `review-skipped` may not be record-backed either.
		let bad_review = r#"{"type":"waiver","task":"t","unit":"step","step":"s","reason":"review-skipped","evidence_tier":"record-backed","evidence":"x"}"#;
		let problems = w5_problems(&waivers(bad_review), &steps, &[], &[]);
		assert!(
			problems.iter().any(|p| p
				.contains("reason `review-skipped` must not carry evidence tier `record-backed`")),
			"{problems:?}"
		);
		// `accepted-at-escalation` may not be self-declared.
		let bad_escalation = r#"{"type":"waiver","task":"t","unit":"step","step":"s","reason":"accepted-at-escalation","evidence_tier":"self-declared"}"#;
		let problems = w5_problems(&waivers(bad_escalation), &steps, &[], &[]);
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
		// The increment-unit waiver among them also needs its ownership evidenced, so the
		// round log joins `s-inc1` to `s`.
		let steps = steps(&one_step_plan("s", "complete"));
		let escalations = escalations(&escalation_line("s-inc1"));
		let rounds = rounds(&owning_round_line("s", "s-inc1"));
		let log = [
			step_waiver_line("s", "predates-logging", "self-declared"),
			step_waiver_line("s", "review-skipped", "self-declared"),
			increment_waiver_line("s", "s-inc1", "s-inc1"),
		]
		.join("\n");
		let problems = w5_problems(&waivers(&log), &steps, &rounds, &escalations);
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
		let problems = check_workflow(&WorkflowSpec::builtin(), plan, &log);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w5_flags_a_record_backed_waiver_whose_escalation_resumed_not_decided() {
		// S3: an escalation exists with the matching `task` but `human_decision:"resume"`
		// (not `decision`), so the record-backed join is not satisfied and the waiver is
		// still flagged. `escalation_line` only emits `decision`, so build the raw line here.
		// The round log owns the increment, so the ONE problem asserted is that join and
		// not Q-70's ownership rule.
		let steps = steps(&one_step_plan("optional-modules", "complete"));
		let waivers = waivers(&increment_waiver_line(
			"optional-modules",
			"optional-modules-inc2cii",
			"optional-modules-inc2cii",
		));
		let resume = r#"{"type":"escalation","task":"optional-modules-inc2cii","artifact":"a","human_decision":"resume"}"#;
		let log = owning_round_line("optional-modules", "optional-modules-inc2cii");
		let problems = w5_problems(&waivers, &steps, &rounds(&log), &escalations(resume));
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
		let problems = w5_problems(&waivers(waiver), &steps, &[], &escalations);
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
		let problems = w5_problems(&waivers(waiver), &steps, &[], &escalations);
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
		// The round log owns the increment, so the ONE problem asserted is that join and
		// not Q-70's ownership rule.
		let steps = steps(&one_step_plan("optional-modules", "complete"));
		let waiver =
			increment_waiver_line("optional-modules", "optional-modules-inc2cii", "unrelated-task");
		let log = owning_round_line("optional-modules", "optional-modules-inc2cii");
		let escalations = escalations(&escalation_line("unrelated-task"));
		let problems = w5_problems(&waivers(&waiver), &steps, &rounds(&log), &escalations);
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
		// O3, restated against the round log (Q-70): an increment-unit waiver's `step`
		// must be the step the log joins its `increment` to. A waiver naming a
		// real-but-wrong step (`alpha`) for an increment the records join to `beta` is
		// reported, so a mis-scoped waiver cannot hide behind a real slug. `beta` goes
		// unmarked in the message because the record DECLARES it in a structured `step`.
		//
		// THE OBSERVED CONTRADICTION, so the fixture carries the round records that
		// establish the true owner rather than leaving the increment unlogged. Without
		// them this would test the unobserved case (which
		// `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` owns) and a
		// build that compared the waiver's `step` with itself would still pass it.
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status   |\n",
			"| ------- | -------- |\n",
			"| `alpha` | complete |\n",
			"| `beta`  | complete |\n",
		);
		let waiver = increment_waiver_line("alpha", "beta-incB", "beta-incB");
		let log = owning_round_line("beta", "beta-incB");
		let escalations = escalations(&escalation_line("beta-incB"));
		let problems = w5_problems(&waivers(&waiver), &steps(plan), &rounds(&log), &escalations);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("increment waiver names step `alpha`")
				&& problems[0].contains("the round log joins increment `beta-incB` to step `beta`"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w5_flags_an_increment_waiver_whose_increment_has_no_round_records() {
		// Q-70-emptycase, the unobserved case, decided by the human as REPORT IT over
		// staying silent and over reporting only when the log is non-empty. An
		// increment-unit waiver whose increment NO round record resolves to is reported:
		// the log joins the increment to no step, so nothing evidences the ownership the
		// waiver asserts. This is the deliberate NARROWING the round-log rule ships,
		// since the retired lexical rule accepted such a waiver whenever the id happened to
		// strip to the step slug.
		//
		// The message must assert a fact the records carry, so it names no step at all: the
		// retired rule reported a step derived from the id, which need not exist in the
		// plan.
		//
		// THE LOG IS NON-EMPTY AND SIMPLY LACKS THIS INCREMENT, which is what makes the
		// case the INCREMENT axis rather than an absent log. An empty slice cannot tell
		// "no record for this increment" from "no records at all", so a build that dropped
		// the increment axis from `waiver_covers_round` and compared the step alone would
		// still pass it (round 1, `W1A-1`).
		let steps = steps(&one_step_plan("alpha", "complete"));
		let waiver = increment_waiver_line("alpha", "alpha-inc1", "alpha-inc1");
		let escalations = escalations(&escalation_line("alpha-inc1"));
		let other = rounds(&owning_round_line("alpha", "alpha-other"));
		let problems = w5_problems(&waivers(&waiver), &steps, &other, &escalations);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains(
				"increment waiver names increment `alpha-inc1`, which no `type:\"round\"` record resolves to (a record resolves to its structured `increment` id, or to its `task` when that id is absent), so the round log joins it to no step"
			),
			"{}",
			problems[0]
		);
		// The same waiver with the increment's records present is accepted, so the report
		// above is about the missing evidence and not about the waiver's shape.
		let log = owning_round_line("alpha", "alpha-inc1");
		let problems = w5_problems(&waivers(&waiver), &steps, &rounds(&log), &escalations);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w5_accepts_an_increment_waiver_whose_id_does_not_strip_to_its_step() {
		// THE UNBLOCKING (Q-70). This is the shape the retired lexical rule made
		// unwritable: an increment id that does not end `-inc<alnum>`, so
		// `leading_slug` returns it unchanged and it can never equal the step slug, while
		// the round log joins it to that step. The waiver is now accepted on the evidence
		// of the records. `workflow-enforcement-tier-fold` is the live instance.
		assert_eq!(
			leading_slug("beta-fold"),
			"beta-fold",
			"the fixture must use an id the shim leaves unstripped"
		);
		let steps = steps(&one_step_plan("beta", "complete"));
		let waiver = increment_waiver_line("beta", "beta-fold", "beta-fold");
		let log = owning_round_line("beta", "beta-fold");
		let escalations = escalations(&escalation_line("beta-fold"));
		let problems = w5_problems(&waivers(&waiver), &steps, &rounds(&log), &escalations);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w5_names_every_step_the_log_joins_a_waived_increment_to() {
		// The refusal reports what the records say, so an increment the log joins to SEVERAL
		// steps names all of them rather than picking one. This is the first of the two
		// routes to several owners: two records carrying DIFFERENT structured `step` ids,
		// which the JSONL substrate permits because a record's `step` is a free string. Both
		// owners are declared, so neither is marked derived.
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status   |\n",
			"| ------- | -------- |\n",
			"| `alpha` | complete |\n",
			"| `beta`  | complete |\n",
			"| `gamma` | complete |\n",
		);
		let waiver = increment_waiver_line("alpha", "shared-inc1", "shared-inc1");
		let log =
			[owning_round_line("beta", "shared-inc1"), owning_round_line("gamma", "shared-inc1")]
				.join("\n");
		let escalations = escalations(&escalation_line("shared-inc1"));
		let problems = w5_problems(&waivers(&waiver), &steps(plan), &rounds(&log), &escalations);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0]
				.contains("the round log joins increment `shared-inc1` to steps `beta`, `gamma`"),
			"{}",
			problems[0]
		);
		assert!(!problems[0].contains("derived"), "{}", problems[0]);
	}

	#[test]
	fn w5_marks_an_owner_derived_from_a_pre_migration_records_task() {
		// PROVENANCE IN THE REFUSAL (round 1, `W1B-1`). A pre-migration record carries no
		// structured `step`, so `round_step_slug` derives its step with
		// `leading_slug(task)`. That value need not be a Roadmap step and need not occur
		// anywhere in the log: here it is `alpha-fold`, which the Roadmap does not carry.
		// The message must therefore mark it as derived rather than present it as a step the
		// records state, which is the half of the recorded `src/` message defect that the
		// empty-owners branch does not reach.
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status      |\n",
			"| ------- | ----------- |\n",
			"| `alpha` | in progress |\n",
			"| `beta`  | in progress |\n",
		);
		let premigration = round_line("alpha-fold", "a", "clean", 1, "risky");
		let waiver = increment_waiver_line("beta", "alpha-fold", "alpha-fold");
		let escalations = escalations(&escalation_line("alpha-fold"));
		let problems =
			w5_problems(&waivers(&waiver), &steps(plan), &rounds(&premigration), &escalations);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains(
				"the round log joins increment `alpha-fold` to step `alpha-fold` (derived from a record's `task`)"
			),
			"{}",
			problems[0]
		);
		assert_eq!(
			steps(plan).iter().filter(|step| step.slug == "alpha-fold").count(),
			0,
			"the fixture's point is that the derived owner is not a Roadmap step"
		);

		// THE SECOND ROUTE TO SEVERAL OWNERS, which needs no free-string abuse (`W1B-3`):
		// one structured record and one pre-migration record for the SAME increment resolve
		// to different steps, and only the derived one is marked.
		let log = [owning_round_line("alpha", "alpha-fold"), premigration].join("\n");
		let problems = w5_problems(&waivers(&waiver), &steps(plan), &rounds(&log), &escalations);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains(
				"the round log joins increment `alpha-fold` to steps `alpha`, `alpha-fold` (derived from a record's `task`)"
			),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it() {
		// The mark is per OWNER and not per record: an owner one record declares in its
		// structured `step` is a value the log carries, whatever route a second record took
		// to the same value. So a declared record plus a pre-migration record that derives
		// the SAME step yields one unmarked owner, and the refusal does not tell the reader
		// to distrust a step the log states.
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status      |\n",
			"| ------- | ----------- |\n",
			"| `alpha` | in progress |\n",
			"| `beta`  | in progress |\n",
		);
		let waiver = increment_waiver_line("beta", "alpha-inc1", "alpha-inc1");
		let log = [
			owning_round_line("alpha", "alpha-inc1"),
			round_line("alpha-inc1", "a", "clean", 1, "risky"),
		]
		.join("\n");
		let escalations = escalations(&escalation_line("alpha-inc1"));
		let problems = w5_problems(&waivers(&waiver), &steps(plan), &rounds(&log), &escalations);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("the round log joins increment `alpha-inc1` to step `alpha`")
				&& !problems[0].contains("derived"),
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
		let problems = w3_problems(&WorkflowSpec::builtin(), &steps(plan), &rounds(&log), &waivers);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("bare-step", "complete")),
			&rounds(&log),
			&waivers,
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("foo-incidental", "complete")),
			&rounds(&log),
			&[],
		);
		assert!(problems.is_empty(), "{problems:?}");
	}

	#[test]
	fn w3_the_same_task_without_the_structured_step_over_strips_and_is_missed() {
		// The companion showing the structured id, not a change to the shim, is what
		// fixes the join: the SAME `foo-incidental` task WITHOUT the field falls back to
		// `leading_slug`, which strips it to `foo`, so the `complete` `foo-incidental`
		// step sees no matching rounds and is caught by the pause.md catch.
		let log = round_line("foo-incidental", "a", "clean", 1, "low_risk");
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("foo-incidental", "complete")),
			&rounds(&log),
			&[],
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("state-schema", "complete")),
			&rounds(&log),
			&[],
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("foo-incidental", "complete")),
			&rounds(log),
			&[],
		);
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
		let problems = w3_problems(
			&WorkflowSpec::builtin(),
			&steps(&one_step_plan("foo-incidental", "complete")),
			&rounds(log),
			&[],
		);
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
		let problems = w5_problems(&waivers(waiver), &steps, &[], &escalations(escalation));
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
		let problems = w5_problems(&waivers(waiver), &steps, &[], &escalations(escalation));
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("is scoped to this waiver's unit"), "{}", problems[0]);
	}

	// -- Inc 4: the TOML-sourced path (`[meta].primary == "toml"`) --
	//
	// These drive the SAME W3/W4/W5 + pause.md checks over a `<task>.plan.toml` source
	// instead of the Markdown plan + JSONL waiver/baseline records, via
	// `check_workflow_toml`. The fixtures are inline `concat!` TOML strings (like the
	// `plan::source` tests), so no new on-disk fixture file exists for taplo/prettier to
	// touch; the rounds/decisions/escalations still come from the JSONL log.

	/// Parse an inline `<task>.plan.toml` fixture into a `PlanToml`, panicking on a
	/// parse error (the fixtures are hand-authored and expected to parse).
	fn toml_plan(source: &str) -> PlanToml {
		plan::parse_toml(source).expect("fixture `<task>.plan.toml` parses")
	}

	#[test]
	fn check_workflow_toml_catches_the_pause_pattern() {
		// The pause.md catch survives the source swap: a `complete` TOML step with no
		// matching rounds and no covering waiver still FAILS W3, identically to the
		// Markdown path.
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\n",
			"[[step]]\nslug = \"pause\"\ntitle = \"P\"\nstatus = \"complete\"\norder = 1\n",
		);
		let problems = check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), "");
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0]
				.contains("`pause` is `complete` but has no round records and no covering waiver"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn check_workflow_toml_converges_a_clean_complete_step() {
		// The happy path over TOML: a `complete` step whose single low-risk increment
		// converged (peak streak 1) passes both W3 and the round-log consistency check.
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\n",
			"[[step]]\nslug = \"done\"\ntitle = \"D\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.increment]]\nid = \"done-inc1\"\nrisk_class = \"low_risk\"\n",
		);
		let log = round_line("done-inc1", "a", "clean", 1, "low_risk");
		assert!(check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), &log).is_empty());
	}

	#[test]
	fn check_workflow_toml_passes_the_optional_modules_accepted_at_escalation_waiver() {
		// Migration shape (a), expressed in TOML: `optional-modules` is `complete` with a
		// risky increment accepted at ONE clean round (peak 1, needs 2), unstuck by a
		// record-backed `[[step.waiver]]` whose `evidence` joins CROSS-SUBSTRATE to the
		// increment's real `decision` escalation in the JSONL. W3 accepts it (covering
		// increment waiver) and W5 accepts it (backed by the scoped escalation).
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\n",
			"[[step]]\nslug = \"optional-modules\"\ntitle = \"OM\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.increment]]\nid = \"optional-modules-inc2cii\"\nrisk_class = \"risky\"\n",
			"[[step.waiver]]\nid = \"om-w1\"\nunit = \"increment\"\nincrement = \"optional-modules-inc2cii\"\n",
			"reason = \"accepted-at-escalation\"\nevidence_tier = \"record-backed\"\nevidence = \"optional-modules-inc2cii\"\n",
		);
		let log = [
			round_line("optional-modules-inc2cii", "a", "new_valid", 0, "risky"),
			round_line("optional-modules-inc2cii", "a", "clean", 1, "risky"),
			escalation_line("optional-modules-inc2cii"),
		]
		.join("\n");
		assert!(
			check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), &log).is_empty(),
			"{:?}",
			check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), &log)
		);
	}

	#[test]
	fn check_workflow_toml_passes_the_waiver_model_self_referential_waiver() {
		// Migration shape (b), the self-referential dogfooding case: the `waiver-model`
		// step's own increment `waiver-model` (a bare token equal to the step slug, no
		// `-inc` suffix) is accepted below its streak by a record-backed waiver whose
		// `evidence` points at the `waiver-model` escalation that accepted it. W3 and W5
		// both pass, joining the increment id and the escalation across substrates.
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\n",
			"[[step]]\nslug = \"waiver-model\"\ntitle = \"WM\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.increment]]\nid = \"waiver-model\"\nrisk_class = \"risky\"\n",
			"[[step.waiver]]\nid = \"wm-w1\"\nunit = \"increment\"\nincrement = \"waiver-model\"\n",
			"reason = \"accepted-at-escalation\"\nevidence_tier = \"record-backed\"\nevidence = \"waiver-model\"\n",
		);
		let log = [
			round_line("waiver-model", "a", "new_valid", 0, "risky"),
			round_line("waiver-model", "a", "clean", 1, "risky"),
			escalation_line("waiver-model"),
		]
		.join("\n");
		assert!(
			check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), &log).is_empty(),
			"{:?}",
			check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), &log)
		);
	}

	#[test]
	fn check_workflow_toml_w5_rejects_a_mis_tiered_waiver() {
		// Un-launderable property (mis-tier): a `self-declared` reason dressed as
		// `record-backed` (with an evidence pointer so the presence filter keeps it) trips
		// W5's `reason` <-> `evidence_tier` pairing over the TOML source. The step is
		// `in-progress` so W3 does not fire, but the record-backed waiver cites an evidence
		// pointer with no matching escalation, so TWO W5 problems fire (the missing evidence
		// join AND the reason-tier mismatch); the assertion targets the reason-tier one. The
		// message names the waiver by its TOML id, not a JSONL log line.
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\n",
			"[[step]]\nslug = \"s\"\ntitle = \"S\"\nstatus = \"in-progress\"\norder = 1\n",
			"[[step.waiver]]\nid = \"w\"\nunit = \"step\"\n",
			"reason = \"predates-logging\"\nevidence_tier = \"record-backed\"\nevidence = \"x\"\n",
		);
		let problems = check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), "");
		assert!(
			problems.iter().any(|problem| problem.contains("TOML waiver `w`")
				&& problem.contains(
					"waiver reason `predates-logging` must not carry evidence tier `record-backed`"
				)),
			"{problems:?}"
		);
	}

	#[test]
	fn check_workflow_toml_w5_rejects_a_wrong_escalation_waiver() {
		// Un-launderable property (wrong escalation): a correctly-tiered record-backed
		// increment waiver whose `evidence` cites an escalation NOT scoped to the waived
		// unit (a `decision` escalation for an unrelated task) is still flagged by W5, so
		// an unrelated human decision cannot back a TOML waiver across the substrate split.
		// The round log owns the waived increment, so the flag asserted is the evidence
		// join and not Q-70's ownership rule.
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\n",
			"[[step]]\nslug = \"s\"\ntitle = \"S\"\nstatus = \"in-progress\"\norder = 1\n",
			"[[step.increment]]\nid = \"s-inc1\"\nrisk_class = \"risky\"\n",
			"[[step.waiver]]\nid = \"w\"\nunit = \"increment\"\nincrement = \"s-inc1\"\n",
			"reason = \"accepted-at-escalation\"\nevidence_tier = \"record-backed\"\nevidence = \"unrelated-task\"\n",
		);
		let log = [owning_round_line("s", "s-inc1"), escalation_line("unrelated-task")].join("\n");
		let problems = check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), &log);
		assert!(
			problems.iter().any(|problem| problem.contains("TOML waiver `w`")
				&& problem.contains("cites evidence `unrelated-task`")
				&& problem.contains("is scoped to this waiver's unit")),
			"{problems:?}"
		);
	}

	#[test]
	fn check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step() {
		// Q-70 on the TOML substrate: a `[[step.waiver]]` inherits its `step` from the step
		// it nests on (`waivers_from_toml`), so the contradiction is authored by nesting the
		// waiver on `alpha` while the round log joins its increment to `beta`. W5 refuses it
		// on what the records say, and its message names `beta` unmarked because the record
		// DECLARES it in a structured `step`, not because the id strips to it. Both steps are
		// `in-progress`, so only W5 speaks.
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\n",
			"[[step]]\nslug = \"alpha\"\ntitle = \"A\"\nstatus = \"in-progress\"\norder = 1\n",
			"[[step.increment]]\nid = \"shared-inc1\"\nrisk_class = \"risky\"\n",
			"[[step.waiver]]\nid = \"w\"\nunit = \"increment\"\nincrement = \"shared-inc1\"\n",
			"reason = \"accepted-at-escalation\"\nevidence_tier = \"record-backed\"\nevidence = \"shared-inc1\"\n",
			"[[step]]\nslug = \"beta\"\ntitle = \"B\"\nstatus = \"in-progress\"\norder = 2\n",
		);
		let log =
			[owning_round_line("beta", "shared-inc1"), escalation_line("shared-inc1")].join("\n");
		let problems = check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), &log);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("TOML waiver `w`")
				&& problems[0].contains(
					"increment waiver names step `alpha` but the round log joins increment `shared-inc1` to step `beta`"
				),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn check_workflow_toml_w4_reads_the_meta_baseline_cutoff() {
		// W4 over TOML reads its cutoff from `[meta].w4_baseline`: a decided item at the
		// cutoff (Q-44) with no receipt is exempt, while one strictly above it (Q-45) with
		// no receipt is flagged. The step is `in-progress` so only W4 speaks.
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\nw4_baseline = \"Q-44\"\n",
			"[[step]]\nslug = \"s\"\ntitle = \"S\"\nstatus = \"in-progress\"\norder = 1\n",
			"[[question]]\nid = \"Q-44\"\nstatus = \"decided\"\nask = \"a\"\nfolded_into = \"s\"\n",
			"[[question]]\nid = \"Q-45\"\nstatus = \"decided\"\nask = \"b\"\nfolded_into = \"s\"\n",
		);
		let problems = check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), "");
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("`Q-45` is decided")
				&& problems[0].contains("has no matching `type:\"decision\"` receipt"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn check_workflow_toml_w4_passes_a_decided_item_with_a_receipt() {
		// The companion: a decided item above the cutoff whose `type:"decision"` receipt
		// is present in the JSONL passes W4 over the TOML source (the receipt still lives
		// in the log, per Q-46's 3(c): only genuine events keep a JSONL home).
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\nw4_baseline = \"Q-44\"\n",
			"[[step]]\nslug = \"s\"\ntitle = \"S\"\nstatus = \"in-progress\"\norder = 1\n",
			"[[question]]\nid = \"Q-45\"\nstatus = \"decided\"\nask = \"b\"\nfolded_into = \"s\"\nreceipt = \"Q-45\"\n",
		);
		let log = decision_line("Q-45");
		assert!(check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), &log).is_empty());
	}

	#[test]
	fn check_workflow_toml_w4_with_no_baseline_requires_a_receipt() {
		// With no `[meta].w4_baseline`, W4 requires a receipt for EVERY decided item (the
		// exemption must be declared), so a decided item with no receipt is flagged.
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\n",
			"[[step]]\nslug = \"s\"\ntitle = \"S\"\nstatus = \"in-progress\"\norder = 1\n",
			"[[question]]\nid = \"Q-1\"\nstatus = \"decided\"\nask = \"a\"\nfolded_into = \"s\"\n",
		);
		let problems = check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), "");
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("`Q-1` is decided"), "{}", problems[0]);
	}

	#[test]
	fn check_workflow_toml_drops_a_malformed_waiver_so_it_grants_no_exemption() {
		// The best-effort drop mirrors the JSONL path: a step-unit waiver that also carries
		// an `increment` (a presence-rule violation) is DROPPED by `waivers_from_toml`, so
		// it does NOT cover the `complete` step-with-no-rounds, which the pause.md catch
		// then flags. A malformed TOML waiver can never silently grant a W3 exemption.
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\n",
			"[[step]]\nslug = \"s\"\ntitle = \"S\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.increment]]\nid = \"s-inc1\"\nrisk_class = \"low_risk\"\n",
			"[[step.waiver]]\nid = \"w\"\nunit = \"step\"\nincrement = \"s-inc1\"\n",
			"reason = \"predates-logging\"\nevidence_tier = \"self-declared\"\n",
		);
		let problems = check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), "");
		assert!(
			problems.iter().any(|problem| problem
				.contains("`s` is `complete` but has no round records and no covering waiver")),
			"{problems:?}"
		);
	}

	#[test]
	fn a_toml_step_waiver_covers_a_complete_step_with_no_rounds() {
		// The step-unit exemption over TOML: a `complete` step with no rounds is exempt
		// when a well-formed step-unit `[[step.waiver]]` covers it (the retired
		// predates-logging/review-skipped cases), so the pause.md catch does not fire.
		let source = concat!(
			"[meta]\ntitle = \"t\"\nprimary = \"toml\"\n",
			"[[step]]\nslug = \"legacy\"\ntitle = \"L\"\nstatus = \"complete\"\norder = 1\n",
			"[[step.waiver]]\nid = \"w\"\nunit = \"step\"\n",
			"reason = \"predates-logging\"\nevidence_tier = \"self-declared\"\n",
		);
		assert!(check_workflow_toml(&WorkflowSpec::builtin(), &toml_plan(source), "").is_empty());
	}
}
