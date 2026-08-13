//! Regression tests for a pack path that leaves the pack directory (`F4b`).
//!
//! A pack path is joined onto the pack root to READ, so a `..` component walked out
//! of the pack and an absolute path discarded it entirely. TWO pack-controlled fields
//! reach that join, and both are covered here because a fix for one alone does not
//! close the defect:
//!
//! - An `[[asset]]`'s `source`. The run exited 0, printed its normal `create <dest>`
//!   plan line, and copied the outside file into the scaffolded project.
//! - A `[[module]]`'s `guidance`. The run exited 0 and spliced the outside file into
//!   the scaffolded `{{modules}}` slot, with no plan line naming it at all, since
//!   guidance is not an asset and is rendered into another file's body.
//!
//! Both are now refused at `PackSource::read`, before either file is opened.
//!
//! Integration tests rather than unit ones because the claim under test is about the
//! whole run: the exit status, that no plan preview promised the drop, and that the
//! outside file's contents never landed. Unit tests over `manifest::load` and
//! `manifest::module_guidance` (in `src/manifest.rs`) pin each refusal at its caller,
//! and one over `PackSource::read` pins the shared site itself.

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

/// Write a secret file OUTSIDE the pack, plus a pack at `<root>/pack` declaring a
/// module whose `guidance` is `guidance`, and one rendered asset carrying the
/// `{{modules}}` slot that the guidance would be spliced into. Returns the pack root.
fn write_guidance_pack(
	root: &Path,
	guidance: &str,
) -> PathBuf {
	fs::write(root.join("secret.md"), "SECRET-OUTSIDE-THE-PACK\n").unwrap();
	let pack = root.join("pack");
	fs::create_dir_all(&pack).unwrap();
	fs::write(
		pack.join("pack.toml"),
		format!(
			"[[module]]\nname = \"evil\"\ndescription = \"d\"\nguidance = \"{guidance}\"\n\n\
			 [[asset]]\nsource = \"body.md\"\ndest = \"body.md\"\nownership = \"working\"\nrender = \
			 true\n"
		),
	)
	.unwrap();
	fs::write(pack.join("body.md"), "before\n{{modules}}\nafter\n").unwrap();
	pack
}

/// Run `scaffold --module evil` from the built binary, with `mode` being `--write` or
/// `--dry-run`.
fn scaffold_with_module(
	pack: &Path,
	out: &Path,
	mode: &str,
) -> Output {
	Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))
		.args(["scaffold", "--template"])
		.arg(pack)
		.arg("--output-dir")
		.arg(out)
		.args(["--vcs", "none", "--module", "evil", mode])
		.output()
		.unwrap()
}

/// Assert the guidance run refused: a non-zero exit, a message naming the module and
/// the guidance path, no write report, and no scaffolded file carrying the secret.
fn assert_guidance_refused(
	output: &Output,
	guidance: &str,
	out: &Path,
) {
	let stdout = String::from_utf8_lossy(&output.stdout);
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert_ne!(output.status.code(), Some(0), "stdout: {stdout}\nstderr: {stderr}");
	assert!(stderr.contains(guidance), "the message must name the guidance path: {stderr}");
	assert!(stderr.contains("evil"), "the message must name the module: {stderr}");
	// A refusal, not a failed read: nothing was opened, so nothing "could not be read".
	assert!(!stderr.contains("could not be read"), "worded as a failed read: {stderr}");
	assert!(!stdout.contains("Wrote to"), "a write was reported: {stdout}");
	let body = out.join("body.md");
	assert!(!body.exists(), "the rendered asset landed: {}", body.display());
}

#[test]
fn a_parent_dir_module_guidance_is_refused_and_reads_nothing_outside_the_pack() {
	let root = scratch("guidance-dotdot");
	let pack = write_guidance_pack(&root, "../secret.md");
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	assert_guidance_refused(&scaffold_with_module(&pack, &out, "--write"), "../secret.md", &out);
	assert_guidance_refused(&scaffold_with_module(&pack, &out, "--dry-run"), "../secret.md", &out);
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}

#[test]
fn an_absolute_module_guidance_is_refused_and_reads_nothing_outside_the_pack() {
	let root = scratch("guidance-absolute");
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();
	let guidance = root.join("secret.md").to_str().unwrap().to_string();
	let pack = write_guidance_pack(&root, &guidance);

	assert_guidance_refused(&scaffold_with_module(&pack, &out, "--write"), &guidance, &out);
	assert_guidance_refused(&scaffold_with_module(&pack, &out, "--dry-run"), &guidance, &out);
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}
