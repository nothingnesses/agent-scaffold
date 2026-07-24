//! The recommendation-rule fragment: the ONE canonical statement of how the
//! human-input contract presents a decision (the viable options or approaches,
//! the trade-offs of each, a recommendation, and the reasoning judged against the
//! plan's Project Principles by name), authored once and projected into every
//! `AGENTS.md` view that restates it.
//!
//! This mirrors `isolation_policy.rs`: a single prose source, substituted into
//! more than one `AGENTS.md` render slot (`{{recommendation_rule}}` via
//! `build_assets` in `main.rs`, which `render` replaces at EVERY occurrence). The
//! same fragment fills the human-input-contract paragraph (where the contract's
//! presentation format is defined) AND the Preflight (where the orchestrator
//! restates the disciplines it will follow, so the format is concrete at resume
//! time). Because both copies read the one fragment, they cannot drift, and a
//! byte-guard test pins the committed scaffold to exactly what this source
//! generates.
//!
//! The fragment NAMES itself as "the human-input contract's presentation format"
//! rather than pointing "above" or "below", so it reads correctly BOTH inside the
//! contract paragraph (where the contract is defined just before it) and
//! standalone in the Preflight restatement (where there is no contract definition
//! adjacent to point at), exactly as `ISOLATION_POLICY_FRAGMENT` names the "Writer
//! isolation rule" to read standalone.

/// The canonical recommendation-rule prose: the standing directive that the
/// human-input contract presents every decision the orchestrator puts to the
/// human the same way (the viable options or approaches, the trade-offs of each,
/// a recommendation, and the reasoning, with the reasoning judged against the
/// plan's Project Principles BY NAME), settled once and rendered from this
/// single source so its copies cannot drift.
///
/// A `&'static str` because the rule takes no computed input; `build_assets`
/// substitutes it into every `{{recommendation_rule}}` slot and the drift-guard
/// test byte-compares it against the committed scaffold.
pub(crate) const RECOMMENDATION_RULE_FRAGMENT: &str = "The human-input contract's presentation format is settled once and rendered from this single source, so its copies cannot drift: wherever the orchestrator puts a decision to the human, it presents that decision the same way, as the viable options or approaches, the trade-offs of each, a recommendation, and the reasoning, with the reasoning judged against the plan's Project Principles by name.";

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
	fn the_fragment_states_the_recommendation_rule() {
		// The fragment must carry the recommendation-in-options rule: it is the single
		// source of the human-input contract's presentation format, it presents the
		// options with their trade-offs, a recommendation, and the reasoning, and it
		// judges that reasoning against the plan's Project Principles BY NAME. A reword
		// of the rule fails here directly, so the fragment's content is pinned
		// independent of the scaffold output.
		assert!(
			RECOMMENDATION_RULE_FRAGMENT
				.contains("settled once and rendered from this single source"),
			"the recommendation rule must state it is the single canonical source"
		);
		assert!(
			RECOMMENDATION_RULE_FRAGMENT
				.contains("the trade-offs of each, a recommendation, and the reasoning"),
			"the recommendation rule must require the options, their trade-offs, a recommendation, and the reasoning"
		);
		assert!(
			RECOMMENDATION_RULE_FRAGMENT.contains("judged against the plan's Project Principles by name"),
			"the recommendation rule must judge the reasoning against the plan's Project Principles by name"
		);
	}

	#[test]
	fn the_committed_scaffold_carries_the_recommendation_rule_fragment() {
		// Drift guard on the PACK generation path: the committed scaffold output (the
		// dogfooded root `AGENTS.md` and its reference copy) must carry the exact
		// fragment this source generates. This fails on a hand edit of the fragment in
		// the committed output (it no longer matches) and on a stale fragment after a
		// source edit that was not re-scaffolded (the fragment changes while the
		// committed bytes do not). The fix in either case is `just scaffold-self`.
		assert!(
			COMMITTED_AGENTS.contains(RECOMMENDATION_RULE_FRAGMENT),
			"root AGENTS.md is missing the current generated recommendation-rule fragment; run `just scaffold-self`"
		);
		assert!(
			COMMITTED_REFERENCE.contains(RECOMMENDATION_RULE_FRAGMENT),
			".agents/AGENTS.reference.md is missing the current generated recommendation-rule fragment; run `just scaffold-self`"
		);
	}
}
