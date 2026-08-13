//! Regression test for a pack `[[asset]].source` that leaves the pack directory.
//!
//! `source` is documented as a path within the pack and is joined onto the pack root
//! to READ, so a `..` component walked out of the pack and an absolute path discarded
//! it entirely. The run exited 0, printed its normal `create <dest>` plan line, and
//! copied a file from outside the pack into the scaffolded project. The manifest
//! loader now refuses both shapes, the same rule it applies to `dest`.
//!
//! An integration test rather than a unit one because the claim under test is about
//! the whole run: the exit status, that no plan preview promised the drop, and that
//! the outside file's contents never landed. A unit test over `manifest::load` (in
//! `src/manifest.rs`) pins the loader's own refusal.

use std::{
	fs,
	path::{
		Path,
		PathBuf,
	},
	process::{
		Command,
		Output,
	},
};

/// A unique scratch root for one test, removed and recreated so a rerun starts clean.
fn scratch(name: &str) -> PathBuf {
	let dir = std::env::temp_dir()
		.join(format!("agent-scaffold-packsource-{}-{name}", std::process::id()));
	let _ = fs::remove_dir_all(&dir);
	fs::create_dir_all(&dir).unwrap();
	dir
}

/// Write a secret file OUTSIDE the pack, plus a one-asset pack at `<root>/pack` whose
/// single asset reads `source` and drops it at `leaked.md`. Returns the pack root.
fn write_pack(
	root: &Path,
	source: &str,
) -> PathBuf {
	fs::write(root.join("secret.md"), "SECRET-OUTSIDE-THE-PACK\n").unwrap();
	let pack = root.join("pack");
	fs::create_dir_all(&pack).unwrap();
	fs::write(
		pack.join("pack.toml"),
		format!("[[asset]]\nsource = \"{source}\"\ndest = \"leaked.md\"\nownership = \"working\"\n"),
	)
	.unwrap();
	pack
}

/// Run `scaffold` from the built binary against `pack` into `out`, with `mode` being
/// `--write` or `--dry-run`.
fn scaffold(
	pack: &Path,
	out: &Path,
	mode: &str,
) -> Output {
	Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))
		.args(["scaffold", "--template"])
		.arg(pack)
		.arg("--output-dir")
		.arg(out)
		.args(["--vcs", "none", mode])
		.output()
		.unwrap()
}

/// Assert the run refused: a non-zero exit, a message naming `source` as a source, no
/// plan preview line and no write report, and no leaked file in `out`.
fn assert_refused(
	output: &Output,
	source: &str,
	out: &Path,
) {
	let stdout = String::from_utf8_lossy(&output.stdout);
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert_ne!(output.status.code(), Some(0), "stdout: {stdout}\nstderr: {stderr}");
	assert!(stderr.contains(source), "the message must name the source: {stderr}");
	assert!(stderr.contains("asset source"), "the message must name the FIELD: {stderr}");
	// Refused at load, so the preview never ran: no `create` line, no write report.
	assert!(!stdout.contains("create"), "a preview line was printed: {stdout}");
	assert!(!stdout.contains("Wrote to"), "a write was reported: {stdout}");
	assert!(!out.join("leaked.md").exists(), "a file from outside the pack landed in the output");
}

#[test]
fn a_parent_dir_source_is_refused_and_reads_nothing_outside_the_pack() {
	let root = scratch("dotdot");
	let pack = write_pack(&root, "../secret.md");
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	assert_refused(&scaffold(&pack, &out, "--write"), "../secret.md", &out);
	// The preview refuses on the same ground, so a dry run cannot promise a drop the
	// action would refuse.
	assert_refused(&scaffold(&pack, &out, "--dry-run"), "../secret.md", &out);
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}

#[test]
fn an_absolute_source_is_refused_and_reads_nothing_outside_the_pack() {
	let root = scratch("absolute");
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();
	let source = root.join("secret.md").to_str().unwrap().to_string();
	let pack = write_pack(&root, &source);

	assert_refused(&scaffold(&pack, &out, "--write"), &source, &out);
	assert_refused(&scaffold(&pack, &out, "--dry-run"), &source, &out);
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}
