//! The containment rules for a free-string path a manifest or a plan source supplies
//! and the tool then joins onto a base directory.
//!
//! The strings are external (a `.plan.toml` or a `--template` pack the user may have
//! fetched from anywhere), so the rules are authored here once rather than copied per
//! caller (Principle 1, prefer the cleaner long-term architecture over the smallest
//! diff).
//!
//! ONE CALLER USES THE LEXICAL RULE WITHOUT JOINING ANYTHING. A
//! `[step.provenance].findings` ref is shape-checked with `is_contained_relative` and
//! is never joined onto a directory and never read: `render` puts it on the Roadmap
//! Notes line as text. That difference matters for a later change rather than for this
//! one, so it is recorded here: `resolved_within` must NOT be applied to a findings
//! ref, because it requires the path to exist, while `plan::source` deliberately does
//! not existence-check one (a findings file is committed and then deleted at task
//! close, so a valid historical pointer can name an absent path).
//!
//! TWO RULES, at two strengths, because a caller that can touch the filesystem can
//! answer a question a caller that cannot must leave open:
//!
//! - `is_contained_relative` is LEXICAL. It decides what a path string NAMES, needs no
//!   filesystem access, and so holds for a path that does not exist. It cannot decide
//!   where a path LANDS, because a symbolic link makes those two different things.
//! - `resolved_within` is RESOLVED. It decides where a path LANDS by canonicalising
//!   both ends, so a symbolic link cannot disguise an outside file as an inside one.
//!   It requires the path to exist and touches the filesystem.
//!
//! A read boundary that must be airtight uses the lexical rule as a fail-fast and then
//! the resolved one. A boundary that must answer without touching disk (the plan-side
//! `validate --source` and `render --check` refusals) has only the lexical rule
//! available, and is lexical for that reason rather than by preference.

use std::{
	fs,
	io,
	path::{
		Component,
		Path,
		PathBuf,
	},
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

/// Why `reference` fails `is_contained_relative`, as a phrase for a refusal message,
/// or `None` when it passes the lexical rule. Derived from the string, so a caller
/// that refused on the RESOLVED rule gets `None` here and supplies its own phrase.
///
/// This exists so a refusal states the rule and the specific cause rather than
/// asserting a property of the input: `a/../b.md` is refused for carrying a `..`
/// component, and saying it "leaves the directory" would be false of it.
pub fn lexical_failure(reference: &str) -> Option<&'static str> {
	let path = Path::new(reference);
	if path.is_absolute() {
		Some("it is an absolute path")
	} else if path.components().any(|component| matches!(component, Component::ParentDir)) {
		Some("it carries a `..` component")
	} else {
		None
	}
}

/// Where `rel` LANDS when joined onto `root`, if that is inside `root`: `Ok(Some(real))`
/// with the canonical path when it is contained, `Ok(None)` when it resolves outside,
/// and `Err` when either end cannot be canonicalised (most often because the file does
/// not exist).
///
/// The RESOLVED rule. `is_contained_relative` decides what a string names and cannot
/// decide this, because a symbolic link makes the two differ: a bare `link.md` names
/// something inside and can land anywhere. Both ends are canonicalised, so the
/// comparison is between two real locations rather than between a real one and a
/// textual guess, and `Path::starts_with` compares whole components, so a sibling
/// directory whose name merely shares a prefix with the root is not mistaken for a
/// child.
///
/// This TOUCHES THE FILESYSTEM: it stats and follows links to answer. Only a caller
/// that is about to open the path anyway can use it, which is why the lexical rule
/// stays available beside it for the callers that must answer without disk access.
/// It does not read the file's contents, so a refused path is still never read.
pub fn resolved_within(
	root: &Path,
	rel: &str,
) -> io::Result<Option<PathBuf>> {
	let real_root = fs::canonicalize(root)?;
	let real = fs::canonicalize(root.join(rel))?;
	Ok(real.starts_with(&real_root).then_some(real))
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

	#[test]
	fn the_lexical_rule_names_the_component_that_failed_it() {
		assert_eq!(lexical_failure("/etc/passwd"), Some("it is an absolute path"));
		assert_eq!(lexical_failure("../escaped.md"), Some("it carries a `..` component"));
		assert_eq!(lexical_failure("a/../b.md"), Some("it carries a `..` component"));
		// A path the lexical rule accepts has no lexical cause to report, even when a
		// caller went on to refuse it on the resolved rule.
		assert_eq!(lexical_failure("link.md"), None);
		assert_eq!(lexical_failure("./nested/file.md"), None);
	}

	/// A scratch root for one test, holding a `pack` subdirectory and an `outside.md`
	/// beside it, so an escape target stays inside the test's own directory.
	fn scratch(name: &str) -> PathBuf {
		let root = std::env::temp_dir()
			.join(format!("agent-scaffold-safepath-{}-{name}", std::process::id()));
		let _ = fs::remove_dir_all(&root);
		fs::create_dir_all(root.join("pack/sub")).unwrap();
		fs::write(root.join("outside.md"), "outside\n").unwrap();
		fs::write(root.join("pack/inside.md"), "inside\n").unwrap();
		fs::write(root.join("pack/sub/real.md"), "real\n").unwrap();
		root
	}

	#[test]
	fn the_resolved_rule_follows_links_and_answers_where_a_path_lands() {
		let root = scratch("resolved");
		let pack = root.join("pack");
		// A plain contained path lands inside and comes back canonicalised.
		let inside = resolved_within(&pack, "inside.md").unwrap().expect("inside is contained");
		assert!(inside.ends_with("pack/inside.md"), "{inside:?}");
		// A link whose target is INSIDE the pack is contained: the rule is about where
		// the path lands, not about whether a link was involved.
		#[cfg(unix)]
		{
			std::os::unix::fs::symlink("sub/real.md", pack.join("alias.md")).unwrap();
			assert!(
				resolved_within(&pack, "alias.md").unwrap().is_some(),
				"a pack-internal link must stay contained"
			);
			// A link whose target is OUTSIDE the pack is refused, though its string is
			// relative and carries no `..`, which is exactly what the lexical rule cannot
			// see.
			std::os::unix::fs::symlink("../outside.md", pack.join("link.md")).unwrap();
			assert!(is_contained_relative("link.md"), "the lexical rule accepts the string");
			assert_eq!(
				resolved_within(&pack, "link.md").unwrap(),
				None,
				"a link landing outside must be refused"
			);
			// A directory link restores arbitrary reach from a relative, `..`-free string.
			std::os::unix::fs::symlink(&root, pack.join("up")).unwrap();
			assert_eq!(resolved_within(&pack, "up/outside.md").unwrap(), None);
		}
		// A path that does not exist cannot be resolved, and is an error rather than a
		// silent refusal, so a caller reports the missing file as a missing file.
		assert!(resolved_within(&pack, "absent.md").is_err());
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn a_sibling_sharing_a_name_prefix_is_not_mistaken_for_a_child() {
		// `Path::starts_with` compares whole components, so `pack-evil` is not inside
		// `pack`. A string prefix test would get this wrong.
		let root = scratch("sibling");
		let pack = root.join("pack");
		fs::create_dir_all(root.join("pack-evil")).unwrap();
		fs::write(root.join("pack-evil/x.md"), "x\n").unwrap();
		#[cfg(unix)]
		{
			std::os::unix::fs::symlink(root.join("pack-evil"), pack.join("nearby")).unwrap();
			assert_eq!(resolved_within(&pack, "nearby/x.md").unwrap(), None);
		}
		fs::remove_dir_all(&root).unwrap();
	}
}
