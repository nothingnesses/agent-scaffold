# Review: `pack-rebuild-tracking` (lens: correctness and robustness of `build.rs`)

Reviewer A. Artifact under review: the new untracked `build.rs` (git status: `?? build.rs`) and its interaction with the `include_dir!("$CARGO_MANIFEST_DIR/pack")` embed in `src/manifest.rs:26`. Spec: the `pack-rebuild-tracking` step at `docs/plans/agent-scaffold.md:273`.

Context confirmed while reviewing: `pack/` currently holds only UTF-8/ASCII files, one subdirectory (`pack/prompts/`), and no symlinks; `Cargo.toml` has no `[build-dependencies]` and no `build =` line; there is no `.cargo/` config.

## Critical

- None found.

## High

- None found.

## Medium

- None found.

## Low

- **Symlinks in `pack/` are followed, so a symlink cycle causes unbounded recursion, and symlinked content is mis-tracked.** Location: `build.rs:26-37` (`track`), specifically `path.is_dir()` (line 28) and `std::fs::read_dir(path)` / `entry.path()` (lines 29-34). `Path::is_dir()` and `read_dir` both follow symlinks, and `entry.path()` returns the link path, not its target. Two consequences if a symlink ever appears under `pack/`: (a) a directory symlink that points at an ancestor (for example `pack/self -> .`) makes `track` recurse forever until the build script overflows the stack or hangs, which is worse than the clean, loud failure the panics elsewhere aim for; (b) a symlink to a file emits `cargo:rerun-if-changed=pack/<link>`, so cargo tracks the link's own mtime, not the target's, meaning an edit to the target that does not touch the link's mtime would not force a rebuild, reintroducing the exact stale-embed bug this step exists to fix. Why low, not higher: `pack/` is entirely maintainer-controlled content with no symlinks today and no path by which untrusted input enters it, so the practical likelihood is near zero. Worth a `symlink_metadata`/`entry.file_type()` check (skip or reject symlinks) to keep the recursion total, matching Principle 12 (fail fast and loudly) rather than hang.

- **`path.display()` is a lossy rendering, so a non-UTF8 or newline-containing pack file name would emit a `cargo:rerun-if-changed` line that does not match the real path.** Location: `build.rs:27` (`println!("cargo:rerun-if-changed={}", path.display())`). `Path::display()` substitutes U+FFFD for non-UTF8 bytes, so cargo would be handed a path that does not exist on disk; cargo would then either rebuild every time (treating the missing path as always-changed) or, worse, silently fail to track the real file (stale embed). A path containing a newline would also break the one-directive-per-line stdout protocol. Why low: a pack file must be UTF-8 to be embedded and read at all (`src/manifest.rs` `contents_utf8`, `read_to_string`), and the whole `pack/` tree is ASCII markdown/TOML, so a name that triggers this cannot currently reach an embeddable pack. The robust form would check `path.to_str()` and fail loudly (or use the OS-string bytes) rather than emit a lossy string; cargo's own docs note `rerun-if-changed` cannot represent such paths.

- **Additions to a directory rely solely on that directory's mtime, which is filesystem-dependent.** Location: `build.rs:26-37` plus the doc comment at `build.rs:7-11`. A newly added file is not in the previous build's emitted line set, so only the parent directory's `cargo:rerun-if-changed` line catches it, via the directory mtime changing. On filesystems or setups where adding an entry does not bump the containing directory's mtime (some network/overlay filesystems), a pure addition could be missed until an unrelated rebuild. Removals are double-covered (directory mtime plus the vanished per-file line), and edits are covered by the per-file line, so only additions carry this single point of failure. Why low: this is inherent to the directory-mtime approach, is explicitly acknowledged in the doc comment, and holds on the ext4-class filesystems the project targets; edit/remove (the cases that produced the original stale-embed incident) are covered.

- **`track` starts from the relative `Path::new("pack")` (`build.rs:19`) while the embed uses `$CARGO_MANIFEST_DIR/pack` (`src/manifest.rs:26`); these agree only because cargo runs the build script with cwd set to the package root.** This is correct for cargo today (build scripts run in `CARGO_MANIFEST_DIR`) and emitted `rerun-if-changed` paths are resolved relative to the package root, so the tracked tree and the embedded tree are the same directory. The residual risk is that the two references to "the pack directory" are not derived from one source (mild tension with Principle 16, one source of truth): if the embed base ever moved, the tracker would silently diverge. Deriving `track`'s root from `env!("CARGO_MANIFEST_DIR")` (available to build scripts as the `CARGO_MANIFEST_DIR` env var) would tie them together. Why low: no divergence exists now and the current behavior is correct.

## Notes (checked, no defect)

- **No `build = "build.rs"` line is needed in `Cargo.toml`, and none is present (confirmed).** Cargo auto-detects a `build.rs` at the package root, so the omission is correct, not a gap.
- **No build dependency is added (confirmed std-only).** `build.rs` imports only `std::path::Path` and uses `std::fs`; `Cargo.toml` has no `[build-dependencies]`. Matches the spec's "std only" requirement.
- **`panic!` on `read_dir`/entry errors (`build.rs:30, 33`) is the right behavior.** Failing the build loudly on an I/O error while enumerating pack inputs is correct per Principle 12 (fail fast and loudly); a swallowed error here would risk a silent stale embed, the very failure mode being fixed. The messages include both the offending path and the underlying error, which is adequate for diagnosis.
- **Emitting explicit `rerun-if-changed` lines correctly narrows, not breaks, cargo's rerun behavior.** Once any `rerun-if-changed` is emitted, cargo tracks only the emitted paths instead of the whole package; here that set is `build.rs` plus the `pack/` tree, which is the intended scope. The self-tracking line at `build.rs:18` keeps the script re-running when it is itself edited. The build script does not run on every build; it re-runs only when a tracked path changes.
- **Empty pack directory and missing pack directory are handled without panic.** An empty `pack/` yields the single directory line and an empty loop. A missing `pack/` makes `is_dir()` false, so `track` emits `cargo:rerun-if-changed=pack` and returns; the real build would then fail at `include_dir!` regardless, so the build script not panicking here is acceptable.
