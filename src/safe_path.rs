//! The ONE containment predicate for a free-string path a manifest or a plan source
//! supplies and the tool then joins onto a base directory.
//!
//! Two callers join such a string onto a directory they own: `plan::source` joins a
//! `[meta].sidecars` front/tail ref (and a `[step.provenance].findings` ref) onto the
//! plan directory to READ it, and `manifest` joins an `[[asset]].dest` onto the
//! `--output-dir` to WRITE it. Both inputs are external (a `.plan.toml` or a
//! `--template` pack the user may have fetched from anywhere), and both joins escape
//! their base on the same two shapes, so the rule is authored here once rather than
//! copied per caller (Principle 1, prefer the cleaner long-term architecture over the
//! smallest diff).

use std::path::{
	Component,
	Path,
};

/// Whether `reference` stays inside the directory it is joined onto: a relative path
/// with no root and no `..` (parent-directory) component. An absolute reference
/// discards the base entirely on `Path::join`, and a `..` component walks out of it,
/// so both are refused at the boundary where the string enters (Principle 21, validate
/// external input where it enters; Principle 18, least authority). A `.` component is
/// accepted, since it names the base itself.
///
/// This is a check on the STRING, so it holds whether or not the referenced path
/// exists and needs no filesystem access: `render`, `render --check`,
/// `validate --source`, and a `scaffold` dry run all refuse exactly what a write
/// refuses.
pub fn is_contained_relative(reference: &str) -> bool {
	let path = Path::new(reference);
	!path.is_absolute()
		&& path
			.components()
			.all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn a_task_relative_reference_is_contained() {
		assert!(is_contained_relative("AGENTS.md"));
		assert!(is_contained_relative("docs/plans/TEMPLATE.plan.toml"));
		assert!(is_contained_relative("./nested/file.md"));
	}

	#[test]
	fn an_absolute_or_parent_bearing_reference_is_not_contained() {
		// An absolute path discards the base on `join`.
		assert!(!is_contained_relative("/etc/passwd"));
		// A `..` component walks out of the base, leading or interior.
		assert!(!is_contained_relative("../escaped.md"));
		assert!(!is_contained_relative("../../escaped.md"));
		assert!(!is_contained_relative("a/../../escaped.md"));
		// A `..` that a purely textual normalisation would cancel is still refused: the
		// check is on the components, not on the resolved result, so a symlinked `a`
		// cannot make the cancellation a lie.
		assert!(!is_contained_relative("a/../b.md"));
	}
}
