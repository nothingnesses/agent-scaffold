//! Regression test for the upgrade that destroyed a hand-edited plan view.
//!
//! At v0.0.1 `docs/plans/TEMPLATE.md` was a manifest asset with
//! `ownership = "working"`, so a re-scaffold printed `skip (exists)` and left a user's
//! edits alone. At 0.0.2 it is not an asset at all: it is a view regenerated from
//! `TEMPLATE.plan.toml` after the assets land, and that regeneration ran
//! unconditionally. A user who scaffolded with 0.0.1, edited their template and then
//! upgraded lost the file, with one `render` line among twenty-nine to show for it.
//!
//! The scaffold now leaves a pre-existing view alone whenever its bytes are not what
//! this version generates. This test fails if that check is removed: without it the
//! second scaffold overwrites the marker and the first assertion goes.

use std::{
	fs,
	path::{
		Path,
		PathBuf,
	},
	process::Command,
};

/// A unique scratch root for one test, removed and recreated so a rerun starts clean.
fn scratch(name: &str) -> PathBuf {
	let dir = std::env::temp_dir()
		.join(format!("agent-scaffold-keepview-{}-{name}", std::process::id()));
	let _ = fs::remove_dir_all(&dir);
	fs::create_dir_all(&dir).unwrap();
	dir
}

/// Run `scaffold --write` into `out`, returning stdout and stderr joined.
fn scaffold(out: &Path) -> (bool, String) {
	let output = Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))
		.args(["scaffold", "--output-dir"])
		.arg(out)
		.args(["--vcs", "none", "--write"])
		.output()
		.unwrap();
	let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
	text.push_str(&String::from_utf8_lossy(&output.stderr));
	(output.status.success(), text)
}

#[test]
fn an_edited_plan_view_is_kept_and_the_run_says_what_to_do() {
	let out = scratch("edited");
	let view = out.join("docs/plans/TEMPLATE.md");

	// A first scaffold generates the view.
	let (ok, _) = scaffold(&out);
	assert!(ok, "the first scaffold must succeed");
	let generated = fs::read_to_string(&view).unwrap();

	// The user edits it, as they legitimately could when it was a working file.
	let edited = format!("{generated}\n\nHAND WRITTEN MARKER\n");
	fs::write(&view, &edited).unwrap();

	// Scaffolding again must not overwrite it.
	let (ok, text) = scaffold(&out);
	assert!(ok, "the second scaffold must still succeed: {text}");
	assert_eq!(
		fs::read_to_string(&view).unwrap(),
		edited,
		"a pre-existing view whose bytes differ from a fresh render must be left untouched"
	);
	// And it must say so, and say what to do about it.
	assert!(text.contains("docs/plans/TEMPLATE.md"), "the run must name the file: {text}");
	assert!(text.contains("left untouched"), "the run must say it was kept: {text}");
	assert!(
		text.contains("agent-scaffold render docs/plans/TEMPLATE.plan.toml"),
		"the run must give the command that produces the current view: {text}"
	);
	let _ = fs::remove_dir_all(&out);
}

#[test]
fn an_unedited_plan_view_is_still_regenerated() {
	// Non-vacuity: the check must fire only on a view that differs. A view this
	// version generated is byte-identical to a fresh render, so an ordinary
	// re-scaffold must still render rather than reporting a refusal.
	let out = scratch("unedited");
	let (ok, _) = scaffold(&out);
	assert!(ok, "the first scaffold must succeed");
	let generated = fs::read_to_string(out.join("docs/plans/TEMPLATE.md")).unwrap();

	let (ok, text) = scaffold(&out);
	assert!(ok, "the second scaffold must succeed: {text}");
	assert!(text.contains("render  docs/plans/TEMPLATE.md"), "it must still render: {text}");
	assert!(!text.contains("left untouched.\n"), "no refusal on an unedited view: {text}");
	assert_eq!(
		fs::read_to_string(out.join("docs/plans/TEMPLATE.md")).unwrap(),
		generated,
		"the regenerated view must be unchanged"
	);
	let _ = fs::remove_dir_all(&out);
}
