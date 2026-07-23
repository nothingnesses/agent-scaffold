//! The isolation policy fragment: the ONE canonical statement of who runs isolated
//! (every spawned agent) versus what stays the orchestrator's own integration-level
//! work on main, authored once and projected into every view that restates it.
//!
//! This mirrors `WorkflowSpec::control_fragment` (`workflow_spec.rs`): a single
//! prose source, substituted into an `AGENTS.md` render slot (`{{isolation_policy}}`
//! via `build_assets` in `main.rs`) AND emitted verbatim by the driver as the
//! writer-state isolation reminder (`next::build_instruction`), with a byte-guard test
//! pinning the committed scaffold to exactly what this source generates. So the
//! guidance prose and the driver's reminder both read the one fragment and cannot drift.
//!
//! The fragment REFERENCES the capability-tiered tier order that already lives in
//! `AGENTS.md` (container, else worktree, else the file-safety fallback) rather than
//! restating it (one source of truth); it adds only the uniform-isolation
//! clarification (every spawned agent isolates, not only the writers) the tier order
//! left ambiguous. It names the "Writer isolation rule"
//! rather than pointing "above", so it reads correctly BOTH inside `AGENTS.md` (where
//! that rule sits just above) and standalone in the driver reminder (where there is no
//! "above" to point at).

/// The canonical isolation policy prose: the standing directive that every spawned
/// agent runs isolated (the writers and the read-only reviewers, triager, and
/// explorers alike, referencing the tier order in `AGENTS.md`, not restating it),
/// because even a findings or exploration file is a write, DISTINCT from the
/// orchestrator's own integration-level edits on main (step-status flips, increment
/// declarations, round records, ledger anchors), which author no reviewed product
/// content.
///
/// A `&'static str` because the policy takes no computed input (unlike
/// `control_fragment`, which interpolates the spec constants); `build_assets`
/// substitutes it into the `{{isolation_policy}}` slot and the drift-guard test
/// byte-compares it against the committed scaffold.
pub(crate) const ISOLATION_POLICY_FRAGMENT: &str = "Who isolates is settled here once and rendered from this single source, so the copies of this rule cannot drift: every spawned agent runs in the strongest isolation the harness supports, per the capability-tiered tier order in the Writer isolation rule, and the orchestrator integrates its output onto main. This holds for the writers (the planner and the implementer) and for the read-only reviewers, triager, and explorers alike, because even a findings or an exploration file is a write: isolating every spawned agent keeps main pristine until the orchestrator integrates, so a killed or misbehaving agent never touches main. The only edits made directly on main are the orchestrator's own integration-level ones, which author no reviewed product content and so stay the orchestrator's direct job rather than a spawned agent's: flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor.";

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
	fn the_fragment_states_the_uniform_isolation_rule() {
		// The fragment must carry the uniform-isolation rule: every spawned agent
		// runs isolated (the read-only reviewers, triager, and explorers included,
		// because even a findings or exploration file is a write), DISTINCT from the
		// orchestrator's own integration-level edits on main. A reword of the policy
		// fails here directly, so the fragment's content is pinned independent of the
		// scaffold output.
		assert!(
			ISOLATION_POLICY_FRAGMENT.contains("every spawned agent runs in the strongest isolation"),
			"the isolation policy must state that every spawned agent runs isolated"
		);
		assert!(
			ISOLATION_POLICY_FRAGMENT.contains("even a findings or an exploration file is a write"),
			"the isolation policy must justify isolating the read-only reviewers, triager, and explorers"
		);
		assert!(
			ISOLATION_POLICY_FRAGMENT.contains("the orchestrator's own integration-level ones"),
			"the isolation policy must distinguish spawned-agent work from the orchestrator's integration edits"
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
