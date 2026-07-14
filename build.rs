//! Register the embedded pack as a cargo rebuild dependency.
//!
//! `src/manifest.rs` embeds `pack/` into the binary at compile time via
//! `include_dir!`. That macro reads the files during compilation but does not
//! tell cargo they are build inputs, so cargo will not rebuild when a pack file
//! changes and a plain `cargo build` can carry a stale embedded pack. Emitting a
//! `cargo:rerun-if-changed` line for the pack directory and for every file under
//! it makes cargo re-run this build script, and so recompile and re-embed, on
//! any change: an edit to a tracked file, or an added or removed file (a change
//! to a directory's entry list changes that directory's mtime, which the
//! directory-level line catches). Kept to std so the crate gains no build
//! dependency.

use std::path::Path;

fn main() {
	// Re-run when the build script itself changes.
	println!("cargo:rerun-if-changed=build.rs");
	// Derive the pack root from CARGO_MANIFEST_DIR so it matches the base
	// `include_dir!` embeds (`$CARGO_MANIFEST_DIR/pack`) and cannot silently
	// diverge from it (for example if the build runs from another directory).
	track(&Path::new(env!("CARGO_MANIFEST_DIR")).join("pack"));
}

/// Emit a `cargo:rerun-if-changed` line for `path`, and for each entry when it
/// is a directory, recursing so every pack file is tracked. Emitting the line
/// for a directory catches additions and removals within it; emitting it for
/// each file catches edits to that file.
///
/// Symlinks are not handled specially and `path.display()` (which is lossy on
/// non-UTF-8 paths) is used rather than a lossless encoding, because `pack/` is
/// maintainer-controlled: it is all-ASCII and symlink-free, and the
/// `include_dir!` embed already requires UTF-8 paths, so neither gap can trigger
/// today. Revisit if `pack/` ever gains symlinks or non-UTF-8 names.
fn track(path: &Path) {
	println!("cargo:rerun-if-changed={}", path.display());
	if path.is_dir() {
		let entries = std::fs::read_dir(path).unwrap_or_else(|error| {
			panic!("failed to read pack directory {}: {error}", path.display())
		});
		for entry in entries {
			let entry = entry.unwrap_or_else(|error| {
				panic!("failed to read entry in {}: {error}", path.display())
			});
			track(&entry.path());
		}
	}
}
