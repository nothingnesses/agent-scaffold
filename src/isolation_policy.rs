//! The writer-isolation policy fragment: the ONE canonical statement of who
//! counts as a writer (and so runs isolated) versus what stays the orchestrator's
//! own integration-level work on main, authored once and projected into every view
//! that restates it.
//!
//! This mirrors `WorkflowSpec::control_fragment` (`workflow_spec.rs`): a single
//! prose source, substituted into an `AGENTS.md` render slot (`{{isolation_policy}}`
//! via `build_assets` in `main.rs`) and, in a later driver stage, emitted as the
//! writer-state reminder, with a byte-guard test pinning the committed scaffold to
//! exactly what this source generates. So the guidance prose and the driver's
//! reminder both read the one fragment and cannot drift.
//!
//! The fragment REFERENCES the capability-tiered tier order that already lives in
//! `AGENTS.md` (container, else worktree, else the file-safety fallback) rather than
//! restating it (one source of truth); it adds only the writer-classification
//! clarification the tier order left ambiguous.

/// The canonical writer-isolation policy prose: the standing directive that every
/// spawned writer runs isolated while read-only agents need none (referencing the
/// tier order in `AGENTS.md`, not restating it), plus the clarification that a
/// spawned planner and any design or exploration writer is a writer exactly as the
/// implementer is, DISTINCT from the orchestrator's own integration-level edits on
/// main (step-status flips, increment declarations, round records, ledger anchors),
/// which author no reviewed product content.
///
/// A `&'static str` because the policy takes no computed input (unlike
/// `control_fragment`, which interpolates the spec constants); `build_assets`
/// substitutes it into the `{{isolation_policy}}` slot and the drift-guard test
/// byte-compares it against the committed scaffold.
pub(crate) const ISOLATION_POLICY_FRAGMENT: &str = "Who counts as a writer is settled here once and rendered from this single source, so the copies of this rule cannot drift: every spawned writer runs in the strongest isolation the harness supports, per the capability-tiered tier order above, while read-only agents need none. A spawned planner is a writer, and so is any design or exploration writer that authors content: each runs worktree-isolated exactly as the implementer does, and the orchestrator merges its branch back on convergence. This is distinct from the orchestrator's own integration-level edits on main, which author no reviewed product content and so stay the orchestrator's direct job rather than a spawned writer's: flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor.";

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
	fn the_fragment_states_the_writer_classification() {
		// The fragment must carry the planner-isolation clarification: a spawned
		// planner (and any design or exploration writer) is a writer that runs
		// isolated, DISTINCT from the orchestrator's integration-level edits on main.
		// A reword of the policy fails here directly, so the fragment's content is
		// pinned independent of the scaffold output.
		assert!(
			ISOLATION_POLICY_FRAGMENT.contains("A spawned planner is a writer"),
			"the isolation policy must classify a spawned planner as a writer"
		);
		assert!(
			ISOLATION_POLICY_FRAGMENT
				.contains("distinct from the orchestrator's own integration-level edits on main"),
			"the isolation policy must distinguish writer work from the orchestrator's integration edits"
		);
	}

	#[test]
	fn the_committed_scaffold_carries_the_isolation_policy_fragment() {
		// Drift guard on the PACK generation path: the committed scaffold output (the
		// dogfooded root `AGENTS.md` and its reference copy) must carry the exact
		// fragment this source generates. This fails on a hand edit of the fragment in
		// the committed output (it no longer matches) and on a stale fragment after a
		// source edit that was not re-scaffolded (the fragment changes while the
		// committed bytes do not). The fix in either case is `just scaffold-self`.
		assert!(
			COMMITTED_AGENTS.contains(ISOLATION_POLICY_FRAGMENT),
			"root AGENTS.md is missing the current generated isolation-policy fragment; run `just scaffold-self`"
		);
		assert!(
			COMMITTED_REFERENCE.contains(ISOLATION_POLICY_FRAGMENT),
			".agents/AGENTS.reference.md is missing the current generated isolation-policy fragment; run `just scaffold-self`"
		);
	}
}
