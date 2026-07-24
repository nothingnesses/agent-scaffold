//! The findings-file naming convention: the ONE canonical statement of the path
//! FORMAT for a round's findings files (the `docs/plans/<task>.reviews/` directory
//! and the reviewer / triager / backstop-re-check basenames), authored once as
//! named-token templates and projected into every view that restates it.
//!
//! This differs from `isolation_policy.rs` and `recommendation_rule.rs`: those are
//! verbatim `&'static str` fragments with no computed input, byte-identical in the
//! `AGENTS.md` slot and the driver reminder. The findings path is NOT verbatim,
//! because the driver's value is a PATH bearing runtime tokens (`task`, `step`)
//! filled per invocation, plus a `<disambiguator>` token the orchestrator fills even
//! later. So the single source here is a set of angle-bracket-token TEMPLATES,
//! rendered two ways from the one set of constants:
//!
//! - `convention_fragment()` renders the templates with every token left in its
//!   `<...>` human form, producing the convention sentence substituted into the
//!   `{{findings_naming}}` slot in `AGENTS.md` (via `build_assets` in `main.rs`).
//! - the `review_findings_path` / `triage_findings_path` builders fill `<task>` and
//!   `<step>` (and set `<role>` to the literal `reviewer` for a reviewer's file),
//!   leaving `<disambiguator>` as the literal token, producing the exact strings the
//!   `next` driver emits into its `review_findings` / `triage_findings` context slots
//!   (`next::build_context`).
//!
//! Both consumers derive from the one set of template constants, so no second
//! encoding of the shape exists and the two cannot drift; a byte-guard test pins the
//! committed scaffold to `convention_fragment()`, and a unit test pins the builders'
//! filled output.

/// The findings-files directory, a named-token template. `<task>` is filled by the
/// driver builders; the convention sentence names the directory in the hand-authored
/// prose that precedes the `{{findings_naming}}` slot, so it is not rendered here.
const DIR_TEMPLATE: &str = "docs/plans/<task>.reviews";

/// A reviewer's findings-file basename, a named-token template. `<role>` resolves to
/// the literal `reviewer` for a reviewer's file; `<step>` is filled by the driver;
/// `<disambiguator>` is left as a literal token for the orchestrator to assign, so no
/// two parallel reviewers collide.
const REVIEWER_BASENAME: &str = "<step>-<role>-<disambiguator>.md";

/// The triager's findings-file basename, a named-token template.
const TRIAGE_BASENAME: &str = "<step>-triage.md";

/// The backstop re-check triager's findings-file basename, a named-token template.
/// The driver has no re-check state and never produces this path; the convention
/// names it for the human, so it appears only in `convention_fragment()`.
const TRIAGE_RECHECK_BASENAME: &str = "<step>-triage-recheck.md";

/// Fill `<task>` in the directory template and join the given basename onto it,
/// producing a findings-file path from the single-source templates. Named-token
/// substitution (not `format!`) because `format!` cannot leave a named argument
/// unfilled, and the builders must leave `<disambiguator>` (and, for the convention,
/// every token) in place.
fn join_dir(task: &str, basename: &str) -> String {
	let dir = DIR_TEMPLATE.replace("<task>", task);
	format!("{dir}/{basename}")
}

/// The reviewer findings-file path the `next` driver emits: `<task>` and `<step>`
/// filled, `<role>` set to the literal `reviewer`, `<disambiguator>` left as its
/// template token for the orchestrator to assign. Reproduces exactly the string the
/// driver formatted by hand before this module existed.
pub(crate) fn review_findings_path(task: &str, step: &str) -> String {
	let basename = REVIEWER_BASENAME.replace("<step>", step).replace("<role>", "reviewer");
	join_dir(task, &basename)
}

/// The triager findings-file path the `next` driver emits: `<task>` and `<step>`
/// filled. Reproduces exactly the string the driver formatted by hand before this
/// module existed.
pub(crate) fn triage_findings_path(task: &str, step: &str) -> String {
	let basename = TRIAGE_BASENAME.replace("<step>", step);
	join_dir(task, &basename)
}

/// The canonical findings-naming convention sentence, rendered from the same template
/// constants the driver builders fill, but with every token left in its `<...>` human
/// form. `build_assets` substitutes this into the `{{findings_naming}}` slot in
/// `AGENTS.md`, so the human convention and the machine path derive from the one
/// source and cannot drift. Single-line prose (no manual wrapping) so the plain
/// scaffold output equals the canonical formatter output and the drift guard passes.
pub(crate) fn convention_fragment() -> String {
	format!(
		"The filenames follow one convention so parallel writers never collide: a reviewer's file is `{REVIEWER_BASENAME}`, where the orchestrator assigns each spawned reviewer a distinct disambiguator (its model, or an index); the triager's is `{TRIAGE_BASENAME}`; and the backstop re-check triager's is `{TRIAGE_RECHECK_BASENAME}`."
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	/// The committed root `AGENTS.md`, embedded so the drift-guard test reads exactly
	/// the scaffold output the repo ships (dogfooded from the pack).
	const COMMITTED_AGENTS: &str = include_str!("../AGENTS.md");

	/// The committed `.agents/AGENTS.reference.md`, the tool-owned reference copy of
	/// the same generated guidance.
	const COMMITTED_REFERENCE: &str = include_str!("../.agents/AGENTS.reference.md");

	#[test]
	fn the_fragment_states_the_naming_convention() {
		// The fragment must carry the naming convention: the collision-avoidance
		// rationale, the reviewer/triage/recheck filename shapes with their tokens
		// unfilled, and the disambiguator-assignment prose. A reword fails here
		// directly, so the fragment's content is pinned independent of the scaffold
		// output.
		let fragment = convention_fragment();
		assert!(
			fragment.contains("parallel writers never collide"),
			"the convention must state the filenames avoid parallel-writer collisions"
		);
		assert!(
			fragment.contains("`<step>-<role>-<disambiguator>.md`"),
			"the convention must name the reviewer filename shape with tokens unfilled"
		);
		assert!(
			fragment.contains(
				"the orchestrator assigns each spawned reviewer a distinct disambiguator (its model, or an index)"
			),
			"the convention must explain how the disambiguator is assigned"
		);
		assert!(
			fragment.contains("`<step>-triage.md`"),
			"the convention must name the triager filename shape"
		);
		assert!(
			fragment.contains("`<step>-triage-recheck.md`"),
			"the convention must name the backstop re-check filename shape"
		);
	}

	#[test]
	fn the_builders_fill_tokens_as_the_driver_expects() {
		// The driver builders must reproduce the exact strings the `next` driver
		// formatted by hand: `<task>`/`<step>` filled, `<role>` set to `reviewer`,
		// `<disambiguator>` left as its literal token. This pins the byte-for-byte
		// equivalence the driver's golden fixtures also assert.
		assert_eq!(
			review_findings_path("demo", "core-assets"),
			"docs/plans/demo.reviews/core-assets-reviewer-<disambiguator>.md"
		);
		assert_eq!(
			triage_findings_path("demo", "core-assets"),
			"docs/plans/demo.reviews/core-assets-triage.md"
		);
	}

	#[test]
	fn the_committed_scaffold_carries_the_convention_fragment() {
		// Drift guard on the PACK generation path: the committed scaffold output (the
		// dogfooded root `AGENTS.md` and its reference copy) must carry the exact
		// fragment this source generates. This fails on a hand edit of the fragment in
		// the committed output (it no longer matches) and on a stale fragment after a
		// source edit that was not re-scaffolded (the fragment changes while the
		// committed bytes do not). The fix in either case is a scaffold regeneration.
		let fragment = convention_fragment();
		assert!(
			COMMITTED_AGENTS.contains(&fragment),
			"root AGENTS.md is missing the current generated findings-naming fragment; regenerate the scaffold"
		);
		assert!(
			COMMITTED_REFERENCE.contains(&fragment),
			".agents/AGENTS.reference.md is missing the current generated findings-naming fragment; regenerate the scaffold"
		);
	}
}
