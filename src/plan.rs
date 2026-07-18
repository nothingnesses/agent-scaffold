//! Plan-structure parsing and validation: read the machine-relevant regions of a
//! Markdown plan and check them against the schema the plan's Documentation
//! Protocol pins.
//!
//! The plan is the single source of truth for status (the Roadmap pipe-table) and
//! for the human-decision queue (the Open Questions list). Those two regions parse
//! deterministically; the free narrative (Step Details, motivations) holds no
//! machine state and is not parsed. This module projects the structured regions
//! and validates their schema and cross-references, so `validate` can hard-fail on
//! a broken table, an unknown status, a duplicate slug, a slug with no Step Detail,
//! or an out-of-set queue status (Principle 5, illegal states caught; Principle 16,
//! one source of truth per region). It parses the plan only; the ledger is a
//! narrative artifact with no machine-parsed tables, and the structured round log
//! (`docs/metrics/workflow.jsonl`) is handled by `metrics.rs`, so neither is parsed
//! here.

use {
	serde::Serialize,
	std::collections::BTreeSet,
};

/// The `<task>.plan.toml` structured-source schema (design B, Q-45/Q-46): the
/// typed skeleton parser and its `validate --source` cross-reference checks. A pure
/// addition for now; nothing in the live pipeline reads it yet (see the module
/// docs). Re-exported so the schema entry points are reached as `plan::parse_toml`
/// and `plan::validate_source`, beside the Markdown-plan functions here.
pub(crate) mod source;

pub(crate) use source::{
	parse_toml,
	validate_source,
};

/// One Roadmap step: a stable slug and its status string, one per data row of the
/// Roadmap pipe-table. The status is kept verbatim so the projection reports
/// exactly what the plan says; schema-checking of it is done in `validate_plan`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Step {
	/// The step's kebab-case slug (the backticked value in the table's first cell).
	pub slug: String,
	/// The status cell verbatim (for example `complete` or `blocked on core-assets`).
	pub status: String,
}

/// One Open Questions queue item: its stable id, the status text inside the
/// parentheses, and the one-line ask that follows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Question {
	/// The item's stable id (for example `Q-24`).
	pub id: String,
	/// The status text inside the parentheses (for example `open` or
	/// `decided -> folded into state-schema`), kept verbatim.
	pub status: String,
	/// The one-line ask following the status parentheses.
	pub ask: String,
}

/// The Roadmap statuses the Documentation Protocol allows, excluding the
/// parametric `blocked on <slug>` form checked separately. Kept in one place so
/// the accepted set and the validator cannot drift (Principle 16). The
/// plan-template drift guard iterates this set, so a status added or renamed here
/// is checked against the template automatically.
///
/// The former `trivial` and `grandfathered` terminal statuses have been RETIRED
/// (`waiver-model`, Q-44): the review-exemption they declared is now a
/// `type:"waiver"` record in the round log, one unified concept (a step that
/// predates logging, or whose review was skipped, is covered by a step-level
/// waiver) rather than two special-case Roadmap statuses. W3 exempts a `complete`
/// step via a covering waiver, not a distinct status. `skipped` STAYS a distinct
/// status: it answers "is this step done?" with "no", which is not an exemption.
const ROADMAP_STATUSES: &[&str] =
	&["not started", "in progress", "complete", "skipped", "next", "optional", "deferred"];

/// The parametric Roadmap status prefix: `blocked on <slug>` names the blocking
/// step. Kept as a named constant so the validator and the drift guard share one
/// spelling (Principle 16). The trailing space is significant: the slug follows it.
const ROADMAP_BLOCKED_PREFIX: &str = "blocked on ";

/// The exact-match Open Questions statuses: the non-parametric queue vocabulary.
/// These match exactly (a near-miss such as `openfoo` is rejected), unlike the
/// parametric fold-into form below. The drift guard iterates this set too.
/// `exploring` is a sub-state of `open`: the item is awaiting a design-space
/// exploration before its options are decidable (see the `exploration-mode`
/// step), distinct from `open` (options ready, awaiting the human's choice).
const QUEUE_EXACT_STATUSES: &[&str] = &["open", "exploring", "superseded"];

/// The parametric Open Questions status prefix: `decided -> folded into <slug>`
/// names the step the decision was folded into. The trailing space is significant:
/// the backticked target slug follows it, and `validate_plan` cross-references it.
/// Exposed to the crate so `workflow.rs`'s W4 check reuses this one definition
/// rather than carrying a second byte-identical copy that could drift out of sync
/// and silently disable W4.
pub(crate) const QUEUE_FOLD_PREFIX: &str = "decided -> folded into ";

/// Return the backticked value between the first pair of backticks in `text`, or
/// `None` when there is not a complete pair. Used to read a slug from a table cell
/// or a heading.
fn first_backtick(text: &str) -> Option<&str> {
	let start = text.find('`')?;
	let rest = &text[start + 1 ..];
	let end = rest.find('`')?;
	Some(&rest[.. end])
}

/// The lines of the level-2 section headed by `## <heading>`, from the line after
/// the heading up to (but not including) the next `## ` heading or the end of the
/// document. Empty when the heading is absent, so a missing section yields no
/// parsed items rather than an error. Only a level-2 `## ` heading ends the
/// section, so nested `### ` / `#### ` headings inside it do not (there are none in
/// the Roadmap or the queue, but this keeps the boundary well-defined).
fn section_lines<'a>(
	markdown: &'a str,
	heading: &str,
) -> Vec<&'a str> {
	let target = format!("## {heading}");
	let mut in_section = false;
	let mut lines = Vec::new();
	for line in markdown.lines() {
		if in_section {
			if line.starts_with("## ") {
				break;
			}
			lines.push(line);
		} else if line.trim_end() == target {
			in_section = true;
		}
	}
	lines
}

/// Whether the document contains a `## <heading>` level-2 section. `section_lines`
/// alone cannot tell a present-but-empty section from an absent one (both yield no
/// lines), so `validate_plan` uses this to decide whether a missing Roadmap table is
/// a real omission to report or simply an absent section to skip.
fn section_present(
	markdown: &str,
	heading: &str,
) -> bool {
	let target = format!("## {heading}");
	markdown.lines().any(|line| line.trim_end() == target)
}

/// Whether a pipe-table row is a GFM delimiter row: every cell is a run of `-` with
/// optional leading/trailing `:` alignment markers and at least one `-` (for example
/// `| --- | :--: |`). This is the row that separates a table's header from its data
/// rows, which `roadmap_table_problems` requires to be present and well-formed.
fn is_delimiter_row(row: &str) -> bool {
	let cells: Vec<&str> = row.trim().trim_matches('|').split('|').map(str::trim).collect();
	!cells.is_empty()
		&& cells.iter().all(|cell| {
			!cell.is_empty() && cell.bytes().all(|b| b == b'-' || b == b':') && cell.contains('-')
		})
}

/// Check the `## Roadmap` section is a well-formed GFM table: a header row, a
/// `| --- | --- |` delimiter row, then data rows that each carry a backticked slug
/// and a status cell. Where `parse_roadmap` silently drops a row it cannot read (it
/// is a best-effort projection), this reports the structure instead, so `validate`
/// hard-fails on a missing/malformed delimiter row or a data row that does not parse
/// into a backticked slug plus a status. It is bounded to this one table model, not
/// a general Markdown-table validator: it treats every `|`-led line in the section
/// as part of the single Roadmap table (there is no other pipe content there). An
/// absent Roadmap section yields no problem (a missing section is not a malformed
/// one); a present section whose table is missing or malformed is reported.
fn roadmap_table_problems(markdown: &str) -> Vec<String> {
	if !section_present(markdown, "Roadmap") {
		return Vec::new();
	}
	let mut problems = Vec::new();
	let pipe_lines: Vec<&str> = section_lines(markdown, "Roadmap")
		.into_iter()
		.map(str::trim)
		.filter(|line| line.starts_with('|'))
		.collect();
	// A GFM table needs a header row then a delimiter row; the delimiter must be the
	// second pipe-led line. A present Roadmap without that is not a table.
	if pipe_lines.len() < 2 || !is_delimiter_row(pipe_lines[1]) {
		problems.push(
			"Roadmap section has no well-formed table (a header row then a `| --- | --- |` delimiter row)"
				.to_string(),
		);
		return problems;
	}
	// Every data row (a pipe-led line after the delimiter) must carry a backticked
	// slug in its first cell and a second (status) cell; report one that does not,
	// rather than dropping it as `parse_roadmap` does. The status vocabulary itself is
	// checked from the parsed steps in `validate_plan`, so a present-but-invalid status
	// is not double-reported here.
	for row in &pipe_lines[2 ..] {
		let cells: Vec<&str> = row.trim_matches('|').split('|').map(str::trim).collect();
		if cells.len() < 2 || first_backtick(cells[0]).is_none() {
			problems.push(format!(
				"Roadmap table row is malformed (expected a backticked slug and a status): {row}"
			));
		}
	}
	problems
}

/// Check every confirmed live queue item (a `- `Q-<n>` ...` line) carries a
/// well-formed `(<status>)` group. `parse_questions` drops an item whose status
/// parentheses are missing or malformed (it is a best-effort projection); this
/// reports it instead, so `validate` hard-fails on a live item with a broken status
/// group rather than silently omitting it. Non-item lines and the historical
/// `OQ-<letter>` provenance prose are ignored, exactly as `parse_questions` ignores
/// them, so only a confirmed `Q-<n>` id triggers the requirement.
fn queue_structure_problems(markdown: &str) -> Vec<String> {
	let mut problems = Vec::new();
	for line in section_lines(markdown, "Open Questions, Decisions, Issues and Blockers") {
		let Some(rest) = line.trim_start().strip_prefix("- ") else {
			continue;
		};
		let Some(first) = rest.find('`') else {
			continue;
		};
		let after_open = &rest[first + 1 ..];
		let Some(second) = after_open.find('`') else {
			continue;
		};
		let id = &after_open[.. second];
		if !is_question_id(id) {
			continue;
		}
		// Confirmed a live `Q-<n>` item: it must carry a `(<status>)` group. Parse the
		// same way `parse_questions` does, but report the failure instead of dropping it.
		let after_id = after_open[second + 1 ..].trim_start();
		let well_formed = after_id.strip_prefix('(').is_some_and(|inner| inner.contains(')'));
		if !well_formed {
			problems.push(format!(
				"Open Questions item `{id}` is missing a well-formed `(status)` group"
			));
		}
	}
	problems
}

/// Parse the Roadmap pipe-table into its data rows. Each data row has a backticked
/// slug in the first cell and a status in the second; the header row (`Step`) and
/// the `| --- | --- |` separator have no backticked first cell and are skipped, so
/// they never reach the caller.
pub fn parse_roadmap(markdown: &str) -> Vec<Step> {
	let mut steps = Vec::new();
	for line in section_lines(markdown, "Roadmap") {
		let trimmed = line.trim();
		if !trimmed.starts_with('|') {
			continue;
		}
		let cells: Vec<&str> = trimmed.trim_matches('|').split('|').map(str::trim).collect();
		if cells.len() < 2 {
			continue;
		}
		// A data row has a backticked slug in the first cell; the header and the
		// separator row do not, so they are skipped here.
		let Some(slug) = first_backtick(cells[0]) else {
			continue;
		};
		steps.push(Step {
			slug: slug.to_string(),
			status: cells[1].to_string(),
		});
	}
	steps
}

/// Whether `id` matches the live queue's `Q-<n>` shape (a `Q-` prefix followed by
/// one or more digits). The historical `OQ-<letter>` provenance markers do not
/// match, so they are ignored rather than parsed as queue items.
fn is_question_id(id: &str) -> bool {
	match id.strip_prefix("Q-") {
		Some(digits) => !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()),
		None => false,
	}
}

/// Parse the Open Questions queue list. Each item is a `- ` list line of the form
/// ``- `Q-<n>` (<status>) <ask>``: the id is the first backticked value, the status
/// is the text inside the first parentheses after it, and the ask is the remainder
/// of the line. Only `Q-<n>` ids are parsed (see `is_question_id`), so the
/// historical `OQ-<letter>` prose and any non-item lines are ignored.
pub fn parse_questions(markdown: &str) -> Vec<Question> {
	let mut questions = Vec::new();
	for line in section_lines(markdown, "Open Questions, Decisions, Issues and Blockers") {
		let Some(rest) = line.trim_start().strip_prefix("- ") else {
			continue;
		};
		// The id is the first backticked value; the text after its closing backtick
		// carries the `(status) ask`.
		let Some(first) = rest.find('`') else {
			continue;
		};
		let after_open = &rest[first + 1 ..];
		let Some(second) = after_open.find('`') else {
			continue;
		};
		let id = &after_open[.. second];
		if !is_question_id(id) {
			continue;
		}
		let after_id = after_open[second + 1 ..].trim_start();
		// The status is the first parenthesised group; its content does not itself
		// contain parentheses, so reading to the first `)` is unambiguous.
		let Some(inner) = after_id.strip_prefix('(') else {
			continue;
		};
		let Some(close) = inner.find(')') else {
			continue;
		};
		questions.push(Question {
			id: id.to_string(),
			status: inner[.. close].trim().to_string(),
			ask: inner[close + 1 ..].trim().to_string(),
		});
	}
	questions
}

/// The set of slugs that have a Step Detail heading (a `###`-or-deeper heading
/// whose text begins with a backticked slug, for example ``### `core-assets`:`` or
/// ``#### `mode-enum`:``). Umbrella headings without a leading backticked slug (for
/// example `### Bring-your-own template`) carry no slug and are excluded, so the
/// set is exactly the slugs a Roadmap row can cross-reference.
pub fn detail_slugs(markdown: &str) -> BTreeSet<String> {
	let mut slugs = BTreeSet::new();
	for line in markdown.lines() {
		let trimmed = line.trim_start();
		if !trimmed.starts_with("###") {
			continue;
		}
		let after = trimmed.trim_start_matches('#').trim_start();
		// Only a heading whose text starts with a backtick names a slug; an umbrella
		// heading (plain prose) does not.
		if after.starts_with('`') {
			if let Some(slug) = first_backtick(after) {
				slugs.insert(slug.to_string());
			}
		}
	}
	slugs
}

/// Whether a Roadmap status is in the allowed set: one of `ROADMAP_STATUSES`, or
/// the parametric `blocked on <slug>` form with a non-empty trailing slug.
fn roadmap_status_ok(status: &str) -> bool {
	ROADMAP_STATUSES.contains(&status)
		|| status.strip_prefix(ROADMAP_BLOCKED_PREFIX).is_some_and(|slug| !slug.trim().is_empty())
}

/// Whether an Open Questions status is in the queue vocabulary: an exact
/// `QUEUE_EXACT_STATUSES` value (`open`, `exploring`, or `superseded`), or the parametric
/// `decided -> folded into <slug>` form (its trailing target slug is cross-referenced
/// separately in `validate_plan`). The exact statuses match exactly, so a near-miss
/// such as `openfoo` is rejected rather than accepted by a loose prefix.
fn question_status_ok(status: &str) -> bool {
	QUEUE_EXACT_STATUSES.contains(&status) || status.starts_with(QUEUE_FOLD_PREFIX)
}

/// Validate the plan's structured regions, returning one human-readable problem
/// per violation (an empty vector means the regions are well-formed). Checks: the
/// Roadmap is a well-formed GFM table with no malformed data rows; every live queue
/// item has a well-formed status group; every Roadmap status is in the allowed set;
/// Roadmap slugs are unique; every Roadmap slug has a matching Step Detail heading
/// and vice versa (every detail block has a Roadmap row); every Open Questions status
/// is in the queue vocabulary; and each `decided -> folded into <slug>` target
/// resolves to a Roadmap slug. These are the schema and cross-reference invariants
/// `validate` hard-fails on: malformed structure is reported, not silently dropped.
pub fn validate_plan(markdown: &str) -> Vec<String> {
	let mut problems = Vec::new();
	let steps = parse_roadmap(markdown);
	let questions = parse_questions(markdown);
	let details = detail_slugs(markdown);

	// Report malformed structure (a broken Roadmap table, a live queue item with a
	// broken status group) rather than letting the best-effort parsers drop it.
	problems.extend(roadmap_table_problems(markdown));
	problems.extend(queue_structure_problems(markdown));

	let roadmap_slugs: BTreeSet<&str> = steps.iter().map(|step| step.slug.as_str()).collect();

	let mut seen: BTreeSet<&str> = BTreeSet::new();
	for step in &steps {
		if !roadmap_status_ok(&step.status) {
			problems.push(format!(
				"Roadmap step `{}` has an unknown status `{}`",
				step.slug, step.status
			));
		}
		if !seen.insert(step.slug.as_str()) {
			problems.push(format!("Roadmap step `{}` appears more than once", step.slug));
		}
		if !details.contains(&step.slug) {
			problems.push(format!(
				"Roadmap step `{}` has no matching `### `{}`` Step Detail heading",
				step.slug, step.slug
			));
		}
	}

	// Reverse cross-reference: every Step Detail slug must have a Roadmap row, so a
	// detail block cannot describe a step the Roadmap does not track (the
	// Documentation Protocol's "every Roadmap slug has a detail block and vice versa").
	for detail in &details {
		if !roadmap_slugs.contains(detail.as_str()) {
			problems.push(format!("Step Detail `{detail}` has no matching Roadmap row"));
		}
	}

	for question in &questions {
		if !question_status_ok(&question.status) {
			problems.push(format!(
				"Open Questions item `{}` has an unknown status `{}`",
				question.id, question.status
			));
		} else if let Some(rest) = question.status.strip_prefix(QUEUE_FOLD_PREFIX) {
			// A `decided -> folded into <slug>` item names the step the decision was
			// folded into; check that target resolves to a Roadmap slug, parallel to the
			// Roadmap-slug -> Step-Detail check above. An empty target is already caught
			// as an unknown status (the trimmed status loses the prefix's trailing space),
			// so here `rest` is non-empty; a target with no backticked slug is reported.
			match first_backtick(rest) {
				Some(target) if roadmap_slugs.contains(target) => {}
				Some(target) => problems.push(format!(
					"Open Questions item `{}` folds into `{}`, which is not a Roadmap step",
					question.id, target
				)),
				None => problems.push(format!(
					"Open Questions item `{}` has a fold-into status with no target slug",
					question.id
				)),
			}
		}
	}

	problems
}

#[cfg(test)]
mod tests {
	use super::*;

	/// A small, well-formed plan exercising both structured regions and a couple of
	/// Step Detail headings (one `###`, one `####`).
	const GOOD_PLAN: &str = concat!(
		"# demo plan\n",
		"\n",
		"## Open Questions, Decisions, Issues and Blockers\n",
		"\n",
		"Historical note: OQ-A is provenance prose, not a live item.\n",
		"\n",
		"- `Q-1` (open) first ask: something to decide. Pointer in `alpha`.\n",
		"- `Q-2` (decided -> folded into `beta`) second ask. Reasoning in `beta`.\n",
		"- `Q-3` (superseded) replaced by a later item.\n",
		"\n",
		"## Roadmap\n",
		"\n",
		"| Step    | Status              |\n",
		"| ------- | ------------------- |\n",
		"| `alpha` | complete            |\n",
		"| `beta`  | in progress         |\n",
		"| `gamma` | blocked on `alpha`  |\n",
		"\n",
		"## Step Details\n",
		"\n",
		"### `alpha`: The first step\n",
		"\n",
		"Some narrative prose here that is not parsed.\n",
		"\n",
		"### Umbrella heading with no slug\n",
		"\n",
		"#### `beta`: The second step\n",
		"\n",
		"More prose.\n",
		"\n",
		"### `gamma`: The third step\n",
		"\n",
		"Prose.\n",
	);

	#[test]
	fn a_well_formed_plan_parses_and_validates() {
		let steps = parse_roadmap(GOOD_PLAN);
		assert_eq!(
			steps,
			vec![
				Step {
					slug: "alpha".to_string(),
					status: "complete".to_string()
				},
				Step {
					slug: "beta".to_string(),
					status: "in progress".to_string()
				},
				Step {
					slug: "gamma".to_string(),
					status: "blocked on `alpha`".to_string()
				},
			]
		);

		let questions = parse_questions(GOOD_PLAN);
		assert_eq!(questions.len(), 3);
		assert_eq!(questions[0].id, "Q-1");
		assert_eq!(questions[0].status, "open");
		assert_eq!(questions[0].ask, "first ask: something to decide. Pointer in `alpha`.");
		assert_eq!(questions[1].id, "Q-2");
		assert_eq!(questions[1].status, "decided -> folded into `beta`");
		assert_eq!(questions[2].status, "superseded");

		let slugs = detail_slugs(GOOD_PLAN);
		assert!(slugs.contains("alpha"));
		assert!(slugs.contains("beta"));
		assert!(slugs.contains("gamma"));
		// The umbrella heading contributes no slug.
		assert_eq!(slugs.len(), 3);

		assert!(validate_plan(GOOD_PLAN).is_empty());
	}

	#[test]
	fn the_blocked_on_status_form_is_accepted() {
		// The `gamma` row uses `blocked on <slug>`, which is a valid parametric
		// status and must not be reported.
		assert!(roadmap_status_ok("blocked on `alpha`"));
		assert!(roadmap_status_ok("blocked on core-assets"));
		// A bare `blocked on` with no trailing slug is not valid.
		assert!(!roadmap_status_ok("blocked on "));
		assert!(!roadmap_status_ok("blocked on"));
	}

	#[test]
	fn the_retired_review_exempt_statuses_are_rejected_but_skipped_stays_accepted() {
		// `trivial` and `grandfathered` are RETIRED (their review-exemption is now a
		// `type:"waiver"` record, not a status), so `validate --plan` must REJECT them as
		// unknown statuses. `skipped` STAYS a distinct, accepted status.
		assert!(!roadmap_status_ok("trivial"));
		assert!(!roadmap_status_ok("grandfathered"));
		assert!(roadmap_status_ok("skipped"));
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status        |\n",
			"| ------- | ------------- |\n",
			"| `alpha` | trivial       |\n",
			"| `beta`  | grandfathered |\n",
			"## Step Details\n",
			"### `alpha`: a\n",
			"### `beta`: b\n",
		);
		let problems = validate_plan(plan);
		assert!(
			problems.iter().any(|p| p.contains("`alpha` has an unknown status `trivial`")),
			"{problems:?}"
		);
		assert!(
			problems.iter().any(|p| p.contains("`beta` has an unknown status `grandfathered`")),
			"{problems:?}"
		);
	}

	#[test]
	fn a_bad_roadmap_status_is_reported() {
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status  |\n",
			"| ------- | ------- |\n",
			"| `alpha` | done    |\n",
			"## Step Details\n",
			"### `alpha`: a\n",
		);
		let problems = validate_plan(plan);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("unknown status `done`"), "{}", problems[0]);
	}

	#[test]
	fn a_duplicate_roadmap_slug_is_reported() {
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status   |\n",
			"| ------- | -------- |\n",
			"| `alpha` | complete |\n",
			"| `alpha` | complete |\n",
			"## Step Details\n",
			"### `alpha`: a\n",
		);
		let problems = validate_plan(plan);
		assert!(
			problems.iter().any(|p| p.contains("`alpha` appears more than once")),
			"{problems:?}"
		);
	}

	#[test]
	fn a_slug_with_no_step_detail_is_reported() {
		let plan = concat!(
			"## Roadmap\n",
			"| Step     | Status   |\n",
			"| -------- | -------- |\n",
			"| `alpha`  | complete |\n",
			"| `orphan` | complete |\n",
			"## Step Details\n",
			"### `alpha`: a\n",
		);
		let problems = validate_plan(plan);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("`orphan` has no matching"), "{}", problems[0]);
	}

	#[test]
	fn a_bad_queue_status_is_reported() {
		let plan = concat!(
			"## Open Questions, Decisions, Issues and Blockers\n",
			"- `Q-1` (pending) an ask.\n",
			"## Roadmap\n",
			"| Step | Status |\n",
			"| ---- | ------ |\n",
		);
		let problems = validate_plan(plan);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("`Q-1` has an unknown status `pending`"), "{}", problems[0]);
	}

	#[test]
	fn the_historical_oq_prose_is_not_parsed_as_a_queue_item() {
		let plan = concat!(
			"## Open Questions, Decisions, Issues and Blockers\n",
			"- `OQ-A` (this is prose) not a live item, must be ignored.\n",
			"- `Q-1` (open) a real item.\n",
			"## Roadmap\n",
		);
		let questions = parse_questions(plan);
		assert_eq!(questions.len(), 1);
		assert_eq!(questions[0].id, "Q-1");
	}

	#[test]
	fn absent_sections_yield_empty_not_a_crash() {
		let plan = "# a plan with no structured regions\n\nJust prose.\n";
		assert!(parse_roadmap(plan).is_empty());
		assert!(parse_questions(plan).is_empty());
		assert!(detail_slugs(plan).is_empty());
		assert!(validate_plan(plan).is_empty());
	}

	#[test]
	fn a_roadmap_with_a_missing_delimiter_row_is_reported() {
		// A header row followed straight by a data row, with no `| --- | --- |`
		// delimiter, is not a well-formed table and must be reported, not silently
		// read as if the delimiter were there.
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status   |\n",
			"| `alpha` | complete |\n",
			"## Step Details\n",
			"### `alpha`: a\n",
		);
		let problems = validate_plan(plan);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("no well-formed table"), "{}", problems[0]);
	}

	#[test]
	fn a_malformed_roadmap_data_row_is_reported() {
		// A data row whose first cell is not a backticked slug does not parse; it must
		// be reported rather than dropped (as the best-effort `parse_roadmap` drops it).
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status   |\n",
			"| ------- | -------- |\n",
			"| `alpha` | complete |\n",
			"| plain   | text     |\n",
			"## Step Details\n",
			"### `alpha`: a\n",
		);
		let problems = validate_plan(plan);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(problems[0].contains("Roadmap table row is malformed"), "{}", problems[0]);
	}

	#[test]
	fn a_queue_item_with_a_broken_status_group_is_reported() {
		// A confirmed `Q-<n>` item with no `(status)` group is dropped by the
		// best-effort parser; the validator must report it instead.
		let plan = concat!(
			"## Open Questions, Decisions, Issues and Blockers\n",
			"- `Q-1` open, but with no parentheses group.\n",
			"## Roadmap\n",
			"| Step | Status |\n",
			"| ---- | ------ |\n",
		);
		let problems = validate_plan(plan);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("`Q-1` is missing a well-formed `(status)` group"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn terminal_queue_statuses_match_exactly() {
		// The exact statuses match exactly, so a near-miss with a trailing suffix is
		// rejected, while the parametric fold-into keeps its prefix match.
		assert!(question_status_ok("open"));
		assert!(question_status_ok("exploring"));
		assert!(question_status_ok("superseded"));
		assert!(!question_status_ok("openfoo"));
		assert!(!question_status_ok("exploringxyz"));
		assert!(!question_status_ok("supersededxyz"));
		assert!(question_status_ok("decided -> folded into `beta`"));
	}

	#[test]
	fn a_fold_into_target_that_does_not_resolve_is_reported() {
		// The fold-into target slug must resolve to a Roadmap step; a dangling target
		// is reported, parallel to the Roadmap-slug -> Step-Detail check.
		let plan = concat!(
			"## Open Questions, Decisions, Issues and Blockers\n",
			"- `Q-1` (decided -> folded into `ghost`) an ask.\n",
			"## Roadmap\n",
			"| Step    | Status   |\n",
			"| ------- | -------- |\n",
			"| `alpha` | complete |\n",
			"## Step Details\n",
			"### `alpha`: a\n",
		);
		let problems = validate_plan(plan);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("folds into `ghost`, which is not a Roadmap step"),
			"{}",
			problems[0]
		);
	}

	#[test]
	fn a_detail_block_with_no_roadmap_row_is_reported() {
		// The reverse cross-reference: a Step Detail slug with no Roadmap row is a
		// detail describing a step the Roadmap does not track, and must be reported.
		let plan = concat!(
			"## Roadmap\n",
			"| Step    | Status   |\n",
			"| ------- | -------- |\n",
			"| `alpha` | complete |\n",
			"## Step Details\n",
			"### `alpha`: a\n",
			"### `stray`: a detail block with no Roadmap row\n",
		);
		let problems = validate_plan(plan);
		assert_eq!(problems.len(), 1, "{problems:?}");
		assert!(
			problems[0].contains("Step Detail `stray` has no matching Roadmap row"),
			"{}",
			problems[0]
		);
	}

	/// Schema drift-guard: the plan's status vocabulary lives in two places, this
	/// validator (the source of truth) and the human-readable placeholders in
	/// `pack/plan-template.md`. This test asserts every Roadmap status and every queue
	/// status the validator accepts is documented verbatim in that template, so
	/// changing the vocabulary on one side without the other fails here (Principle 16,
	/// one source of truth; Principle 11, the test drives the real accepted sets). The
	/// statuses are iterated from the validator's own constants rather than
	/// re-hardcoded, so renaming one in code re-points the check at its new spelling
	/// and the template must then document that spelling. The match forms are anchored,
	/// not weak substrings: the Roadmap statuses appear in the template's comma-list
	/// (`not started, in progress, ...`) so each is matched with its trailing comma,
	/// and the queue statuses appear backticked (`` `open` ``) so each is matched with
	/// its backticks, both of which actually catch a removed status.
	#[test]
	fn plan_template_documents_every_accepted_status() {
		let template = include_str!("../pack/plan-template.md");

		// Every exact Roadmap status: documented in the template's `Statuses:` list as
		// a comma-separated run, so anchor on the trailing comma each carries there.
		for status in ROADMAP_STATUSES {
			assert!(
				template.contains(&format!("{status},")),
				"Roadmap status `{status}` accepted by the validator is not documented in pack/plan-template.md"
			);
		}
		// The parametric `blocked on <slug>` form, documented as the list's last entry.
		assert!(
			template.contains(&format!("{ROADMAP_BLOCKED_PREFIX}<slug>")),
			"parametric Roadmap status `{ROADMAP_BLOCKED_PREFIX}<slug>` is not documented in pack/plan-template.md"
		);

		// Every exact queue status: documented backticked, so anchor on the backticks.
		for status in QUEUE_EXACT_STATUSES {
			assert!(
				template.contains(&format!("`{status}`")),
				"queue status `{status}` accepted by the validator is not documented in pack/plan-template.md"
			);
		}
		// The parametric fold-into form (its trailing space precedes the target slug).
		assert!(
			template.contains(QUEUE_FOLD_PREFIX.trim_end()),
			"parametric queue status `{}<slug>` is not documented in pack/plan-template.md",
			QUEUE_FOLD_PREFIX
		);
	}
}
