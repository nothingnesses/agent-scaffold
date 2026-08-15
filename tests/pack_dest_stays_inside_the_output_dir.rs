//! Regression test for a pack `[[asset]].dest` that leaves `--output-dir`.
//!
//! `dest` is documented as relative to the output directory and is joined onto it
//! verbatim, so a `..` component walked out of the directory and an absolute path
//! discarded it entirely. Both wrote the file outside, at exit 0, while the run
//! reported "Wrote to <output-dir>". The manifest loader now refuses both shapes.
//!
//! An integration test rather than a unit one because the claim under test is about
//! the whole run: the exit status, what the plan preview printed BEFORE any write, and
//! that no file landed anywhere. A unit test over `manifest::load` (in `src/manifest.rs`)
//! pins the loader's own refusal; this pins the observable behaviour the reproduction
//! described.

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
		.join(format!("agent-flow-packdest-{}-{name}", std::process::id()));
	let _ = fs::remove_dir_all(&dir);
	fs::create_dir_all(&dir).unwrap();
	dir
}

/// Write a one-asset pack at `<root>/pack` whose single asset lands at `dest`.
fn write_pack(
	root: &Path,
	dest: &str,
) -> PathBuf {
	let pack = root.join("pack");
	fs::create_dir_all(&pack).unwrap();
	fs::write(pack.join("escape.md"), "x\n").unwrap();
	fs::write(
		pack.join("pack.toml"),
		format!("[[asset]]\nsource = \"escape.md\"\ndest = \"{dest}\"\nownership = \"working\"\n"),
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
	Command::new(env!("CARGO_BIN_EXE_agent-flow"))
		.args(["scaffold", "--template"])
		.arg(pack)
		.arg("--output-dir")
		.arg(out)
		.args(["--vcs", "none", mode])
		.output()
		.unwrap()
}

/// Assert the run refused: a non-zero exit, a message naming `dest`, no plan preview
/// line and no write report, and no file at `escaped`.
fn assert_refused(
	output: &Output,
	dest: &str,
	escaped: &Path,
) {
	let stdout = String::from_utf8_lossy(&output.stdout);
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert_ne!(output.status.code(), Some(0), "stdout: {stdout}\nstderr: {stderr}");
	assert!(stderr.contains(dest), "the message must name the dest: {stderr}");
	// Refused at load, so the preview never ran: no `create` line, no write report.
	assert!(!stdout.contains("create"), "a preview line was printed: {stdout}");
	assert!(!stdout.contains("Wrote to"), "a write was reported: {stdout}");
	assert!(!escaped.exists(), "a file landed outside the output directory: {}", escaped.display());
}

#[test]
fn a_parent_dir_dest_is_refused_and_writes_nothing() {
	let root = scratch("dotdot");
	let pack = write_pack(&root, "../../escaped-outside.md");
	let out = root.join("work/out");
	fs::create_dir_all(&out).unwrap();
	let escaped = root.join("escaped-outside.md");

	assert_refused(&scaffold(&pack, &out, "--write"), "../../escaped-outside.md", &escaped);
	// The preview refuses on the same ground, so a dry run cannot promise a write the
	// action would refuse.
	assert_refused(&scaffold(&pack, &out, "--dry-run"), "../../escaped-outside.md", &escaped);
	// Nothing landed inside the output directory either.
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}

#[test]
fn an_absolute_dest_is_refused_and_writes_nothing() {
	let root = scratch("absolute");
	let escaped = root.join("escaped-absolute.md");
	let dest = escaped.to_str().unwrap().to_string();
	let pack = write_pack(&root, &dest);
	let out = root.join("work/out");
	fs::create_dir_all(&out).unwrap();

	assert_refused(&scaffold(&pack, &out, "--write"), &dest, &escaped);
	assert_refused(&scaffold(&pack, &out, "--dry-run"), &dest, &escaped);
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}
