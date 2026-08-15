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
		.join(format!("agent-flow-packsource-{}-{name}", std::process::id()));
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
	Command::new(env!("CARGO_BIN_EXE_agent-flow"))
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
	Command::new(env!("CARGO_BIN_EXE_agent-flow"))
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

// -- The symlink shapes --
//
// A pack path can also escape without any `..` and without being absolute, by naming
// a symbolic link the pack itself ships. These three cases are ADDED beside the four
// string-shape cases above rather than replacing any of them: the string shapes and
// the link shapes fail different halves of the rule, and only running both pins that
// the read site applies both.

/// Symlink `link_name` inside `pack` to `target`, replacing any existing entry.
#[cfg(unix)]
fn link(
	pack: &Path,
	link_name: &str,
	target: &Path,
) {
	let at = pack.join(link_name);
	let _ = fs::remove_file(&at);
	std::os::unix::fs::symlink(target, at).unwrap();
}

#[cfg(unix)]
#[test]
fn a_symlinked_source_is_refused_and_reads_nothing_outside_the_pack() {
	let root = scratch("symlink-source");
	let pack = write_pack(&root, "link.md");
	link(&pack, "link.md", &root.join("secret.md"));
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	assert_refused(&scaffold(&pack, &out, "--write"), "link.md", &out);
	assert_refused(&scaffold(&pack, &out, "--dry-run"), "link.md", &out);
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}

#[cfg(unix)]
#[test]
fn a_symlinked_module_guidance_is_refused_and_reads_nothing_outside_the_pack() {
	let root = scratch("symlink-guidance");
	let pack = write_guidance_pack(&root, "link.md");
	link(&pack, "link.md", &root.join("secret.md"));
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	assert_guidance_refused(&scaffold_with_module(&pack, &out, "--write"), "link.md", &out);
	assert_guidance_refused(&scaffold_with_module(&pack, &out, "--dry-run"), "link.md", &out);
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}

#[cfg(unix)]
#[test]
fn a_directory_symlink_cannot_restore_arbitrary_reach() {
	// A DISTINCT shape, not a variant of the two above: one link to a directory lets a
	// path string that is relative and carries no `..` reach anything the link's target
	// contains, which is how the absolute-path refusal is defeated without ever writing
	// an absolute path. The escape target stays inside this test's own root.
	let root = scratch("dir-symlink");
	let pack = write_pack(&root, "up/secret.md");
	link(&pack, "up", &root);
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	assert_refused(&scaffold(&pack, &out, "--write"), "up/secret.md", &out);
	assert_refused(&scaffold(&pack, &out, "--dry-run"), "up/secret.md", &out);
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}

#[cfg(unix)]
#[test]
fn a_pack_internal_symlink_still_scaffolds() {
	// Non-vacuity for the whole run, matching the unit-level pin: the rule is about
	// where a path lands, so a link INSIDE the pack is legitimate and a real scaffold
	// through one must still drop the file with the linked contents.
	let root = scratch("internal-symlink");
	let pack = write_pack(&root, "alias.md");
	fs::create_dir_all(pack.join("sub")).unwrap();
	fs::write(pack.join("sub/real.md"), "REAL BODY\n").unwrap();
	link(&pack, "alias.md", Path::new("sub/real.md"));
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	let output = scaffold(&pack, &out, "--write");
	assert_eq!(output.status.code(), Some(0), "stderr: {}", String::from_utf8_lossy(&output.stderr));
	assert_eq!(fs::read_to_string(out.join("leaked.md")).unwrap(), "REAL BODY\n");
	let _ = fs::remove_dir_all(&root);
}

// -- The literals the tool reads by name --
//
// `principles.toml` and `instrument.md` are read by literal name rather than through
// an `[[asset]]`, and their call sites once discarded every error because the only
// reachable one meant "the pack ships none". The resolved rule made a refusal
// reachable for them too. These cases pin that a refused literal is now reported
// rather than silently treated as absent. ADDED beside the eight cases above; none of
// those is replaced.

/// Write a pack at `<root>/pack` that renders `{{principles}}` and `{{instrument}}`
/// into one asset, with `literal` deployed as a link OUT of the pack. Returns the pack.
#[cfg(unix)]
fn write_literal_pack(
	root: &Path,
	literal: &str,
) -> PathBuf {
	fs::write(root.join("outside.txt"), "id = \"x\"\nname = \"X\"\nsummary = \"s\"\n").unwrap();
	let pack = root.join("pack");
	fs::create_dir_all(&pack).unwrap();
	fs::write(
		pack.join("pack.toml"),
		"[[asset]]\nsource = \"body.md\"\ndest = \"AGENTS.md\"\nownership = \"working\"\nrender = \
		 true\n",
	)
	.unwrap();
	fs::write(pack.join("body.md"), "P:{{principles}}\nI:{{instrument}}\n").unwrap();
	std::os::unix::fs::symlink(root.join("outside.txt"), pack.join(literal)).unwrap();
	pack
}

#[cfg(unix)]
#[test]
fn a_linked_instrument_fragment_is_reported_not_silently_dropped() {
	let root = scratch("literal-instrument");
	let pack = write_literal_pack(&root, "instrument.md");
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	let output = Command::new(env!("CARGO_BIN_EXE_agent-flow"))
		.args(["scaffold", "--template"])
		.arg(&pack)
		.arg("--output-dir")
		.arg(&out)
		.args(["--vcs", "none", "--instrument", "--write"])
		.output()
		.unwrap();
	let stdout = String::from_utf8_lossy(&output.stdout);
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert_ne!(output.status.code(), Some(0), "stdout: {stdout}\nstderr: {stderr}");
	assert!(stderr.contains("instrument.md"), "the message must name the file: {stderr}");
	assert!(!stdout.contains("Wrote to"), "a write was reported: {stdout}");
	assert!(!out.join("AGENTS.md").exists(), "a degraded AGENTS.md was written");
	let _ = fs::remove_dir_all(&root);
}

#[cfg(unix)]
#[test]
fn a_linked_principles_file_is_reported_not_silently_dropped() {
	let root = scratch("literal-principles");
	let pack = write_literal_pack(&root, "principles.toml");
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	let output = Command::new(env!("CARGO_BIN_EXE_agent-flow"))
		.args(["scaffold", "--template"])
		.arg(&pack)
		.arg("--output-dir")
		.arg(&out)
		.args(["--vcs", "none", "--write"])
		.output()
		.unwrap();
	let stdout = String::from_utf8_lossy(&output.stdout);
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert_ne!(output.status.code(), Some(0), "stdout: {stdout}\nstderr: {stderr}");
	assert!(stderr.contains("principles.toml"), "the message must name the file: {stderr}");
	// It must say it could not READ the file, not that it could not parse it: the file
	// never became text.
	assert!(!stderr.contains("could not parse"), "named the wrong step: {stderr}");
	assert!(!out.join("AGENTS.md").exists(), "a degraded AGENTS.md was written");
	let _ = fs::remove_dir_all(&root);
}

#[cfg(unix)]
#[test]
fn a_linked_pack_manifest_is_refused_with_a_message_naming_it() {
	// The `pack.toml` literal goes through `io::Error::from` and carries no field
	// label, so nothing pinned its wording.
	let root = scratch("literal-manifest");
	let pack = root.join("pack");
	fs::create_dir_all(&pack).unwrap();
	fs::write(
		root.join("outside.toml"),
		"[[asset]]\nsource = \"a.md\"\ndest = \"a.md\"\nownership = \"working\"\n",
	)
	.unwrap();
	std::os::unix::fs::symlink(root.join("outside.toml"), pack.join("pack.toml")).unwrap();
	fs::write(pack.join("a.md"), "a\n").unwrap();
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	let output = scaffold(&pack, &out, "--write");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert_ne!(output.status.code(), Some(0), "stderr: {stderr}");
	assert!(stderr.contains("pack.toml"), "the message must name the manifest: {stderr}");
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}

#[cfg(unix)]
#[test]
fn a_pack_shipping_neither_optional_literal_still_scaffolds() {
	// Non-vacuity for the whole run: absence must stay silent. A pack with no
	// `principles.toml` and no `instrument.md` renders both blocks empty at exit 0,
	// which is what `README.md` promises and what an over-tightening would break.
	let root = scratch("literal-absent");
	let pack = root.join("pack");
	fs::create_dir_all(&pack).unwrap();
	fs::write(
		pack.join("pack.toml"),
		"[[asset]]\nsource = \"body.md\"\ndest = \"AGENTS.md\"\nownership = \"working\"\nrender = \
		 true\n",
	)
	.unwrap();
	fs::write(pack.join("body.md"), "P:{{principles}}\nI:{{instrument}}\n").unwrap();
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	let output = Command::new(env!("CARGO_BIN_EXE_agent-flow"))
		.args(["scaffold", "--template"])
		.arg(&pack)
		.arg("--output-dir")
		.arg(&out)
		.args(["--vcs", "none", "--instrument", "--write"])
		.output()
		.unwrap();
	assert_eq!(
		output.status.code(),
		Some(0),
		"stderr: {}",
		String::from_utf8_lossy(&output.stderr)
	);
	assert_eq!(fs::read_to_string(out.join("AGENTS.md")).unwrap(), "P:\nI:\n");
	let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_template_that_is_not_a_directory_is_reported_against_the_flag() {
	// A failure of the `--template` root must name the flag and the path, not the
	// first file inside the pack a run happens to read. Nothing pinned this message
	// before, so nothing is replaced.
	let root = scratch("template-not-a-dir");
	let not_a_pack = root.join("pack.toml");
	fs::write(&not_a_pack, "[[asset]]\n").unwrap();
	let out = root.join("out");
	fs::create_dir_all(&out).unwrap();

	let output = scaffold(&not_a_pack, &out, "--write");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert_ne!(output.status.code(), Some(0), "stderr: {stderr}");
	assert!(stderr.contains("--template"), "the message must name the flag: {stderr}");
	assert!(
		stderr.contains(not_a_pack.to_str().unwrap()),
		"the message must name the path: {stderr}"
	);
	assert!(
		!stderr.contains("principles.toml"),
		"a root failure must not be reported against a file inside the pack: {stderr}"
	);
	assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "the output directory must stay empty");
	let _ = fs::remove_dir_all(&root);
}
