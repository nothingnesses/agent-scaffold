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
//! one source of truth per region). It parses the plan only; the ledger's
//! round-summary narrative is out of scope for this increment.

use {
	serde::Serialize,
	std::collections::BTreeSet,
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
/// the accepted set and the validator cannot drift (Principle 16).
const ROADMAP_STATUSES: &[&str] =
	&["not started", "in progress", "complete", "skipped", "next", "optional", "deferred"];

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
		|| status.strip_prefix("blocked on ").is_some_and(|slug| !slug.trim().is_empty())
}

/// Whether an Open Questions status is in the queue vocabulary: it starts with
/// `open`, `decided -> folded into `, or `superseded` (the statuses the
/// Documentation Protocol defines).
fn question_status_ok(status: &str) -> bool {
	status.starts_with("open")
		|| status.starts_with("decided -> folded into ")
		|| status.starts_with("superseded")
}

/// Validate the plan's structured regions, returning one human-readable problem
/// per violation (an empty vector means the regions are well-formed). Checks: every
/// Roadmap status is in the allowed set; Roadmap slugs are unique; every Roadmap
/// slug has a matching Step Detail heading; and every Open Questions status is in
/// the queue vocabulary. These are the schema and cross-reference invariants
/// `validate` hard-fails on.
pub fn validate_plan(markdown: &str) -> Vec<String> {
	let mut problems = Vec::new();
	let steps = parse_roadmap(markdown);
	let questions = parse_questions(markdown);
	let details = detail_slugs(markdown);

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

	for question in &questions {
		if !question_status_ok(&question.status) {
			problems.push(format!(
				"Open Questions item `{}` has an unknown status `{}`",
				question.id, question.status
			));
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
}
