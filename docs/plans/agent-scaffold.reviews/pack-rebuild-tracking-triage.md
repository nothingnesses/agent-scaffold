# Triage: `pack-rebuild-tracking` (`build.rs`)

Artifact: the new untracked `build.rs`, tracking the `pack/` tree so cargo
re-embeds it (`include_dir!("$CARGO_MANIFEST_DIR/pack")`, `src/manifest.rs:26`)
when a pack file changes. Reviewer A raised four low findings, all framed as
robustness gaps rather than live bugs. I verified each against `build.rs`,
`src/manifest.rs`, and `Cargo.toml`.

Context confirmed: `Cargo.toml` has no `[build-dependencies]` and no `build =`
line (std-only, auto-detected build script), matching the reviewer's notes. The
mechanism itself is correct and validated; the findings concern edge cases that
`pack/` (maintainer-controlled, all-ASCII, symlink-free today) does not
currently exercise.

## Finding 1: symlinks in `pack/` are followed (recursion + mis-tracking)

Verdict: VALID, severity low.

Justification: `build.rs:28-34` uses `path.is_dir()`, `std::fs::read_dir`, and
`entry.path()`, all of which follow symlinks and return the link path rather
than the target. A directory symlink pointing at an ancestor would make `track`
recurse until stack overflow (an unbounded hang, worse than the loud panics
elsewhere), and a file symlink would emit the link's path so cargo watches the
link mtime, not the target, reintroducing a stale-embed window. Correctly rated
low: no symlinks exist under `pack/` and nothing untrusted can add one, so the
likelihood is near zero.

Recommendation: accept with a brief rationale comment. Note in the `track`
doc comment that entries are assumed to be regular files/directories because
`pack/` is maintainer-controlled and symlink-free. The fix (an
`entry.file_type()` / `symlink_metadata` check that skips or panics on symlinks)
is cheap and would convert the potential hang into a loud failure per
Principle 12, so it is a reasonable optional hardening; but with no symlinks in
scope it is not required now.

## Finding 2: `path.display()` is lossy for non-UTF8 / newline names

Verdict: VALID, severity low.

Justification: `build.rs:27` emits `path.display()`, which substitutes U+FFFD
for non-UTF8 bytes, so a non-UTF8 name would hand cargo a path that does not
exist on disk (always-rebuild or, worse, a silently untracked real file), and a
newline would break the one-directive-per-line stdout protocol. Correctly low:
a pack file must be valid UTF-8 to be embedded and read at all
(`src/manifest.rs` uses `contents_utf8` / `read_to_string`), and the tree is
ASCII, so such a name cannot reach an embeddable pack.

Recommendation: accept with a brief rationale comment (the UTF-8/ASCII
invariant of `pack/` is the reason `display()` is safe here). A robust form
would check `path.to_str()` and panic on `None`, which is cheap, but given the
embed path already requires UTF-8 this is defensive rather than
correctness-improving; not worth prioritising.

## Finding 3: additions rely on the parent directory's mtime

Verdict: VALID, severity low.

Justification: `build.rs:26-37` catches a pure addition only via the parent
directory's `cargo:rerun-if-changed` line and its mtime; on filesystems that do
not bump directory mtime on entry addition (some network/overlay mounts) an
addition could be missed until an unrelated rebuild. Edits (per-file line) and
removals (directory line plus the vanished per-file line) are covered.
Correctly low: this is inherent to the directory-mtime approach, holds on the
ext4-class filesystems the project targets, and the edit/remove cases that
caused the original stale-embed incident are covered.

Recommendation: accept. This is already documented in the `build.rs:7-11` doc
comment, so the rationale comment the accept path calls for is effectively
present; no change needed. Removing the dependency entirely would mean walking
and hashing rather than relying on mtime, which is disproportionate for a
maintainer-controlled tree.

## Finding 4: `track` root is relative `Path::new("pack")`, not derived from `CARGO_MANIFEST_DIR`

Verdict: VALID, severity low.

Justification: `build.rs:19` starts from the relative `"pack"` while the embed
uses `$CARGO_MANIFEST_DIR/pack` (`src/manifest.rs:26`). These agree only because
cargo runs the build script with cwd at the package root and resolves emitted
`rerun-if-changed` paths relative to it. Behaviour is correct today; the residual
risk is two un-derived references to "the pack directory" (mild tension with
Principle 16, one source of truth): if the embed base moved, the tracker would
silently diverge.

Recommendation: fix now. This is the one cheap, correctness-improving change:
derive the root from `env!("CARGO_MANIFEST_DIR")` (available to build scripts as
the `CARGO_MANIFEST_DIR` env var), e.g.
`track(&Path::new(env!("CARGO_MANIFEST_DIR")).join("pack"))`, so the tracked
tree and the embedded tree are tied to the same base rather than agreeing by
coincidence of cwd. Low effort, removes a silent-divergence footgun, and aligns
the two references with one source.

## Summary of verdicts

| # | Finding | Verdict | Severity | Recommendation |
|---|---------|---------|----------|----------------|
| 1 | Symlinks followed (recursion + mis-track) | VALID | low | Accept with rationale comment; cheap `file_type` guard optional |
| 2 | `path.display()` lossy on non-UTF8/newline | VALID | low | Accept with rationale comment |
| 3 | Additions rely on directory mtime | VALID | low | Accept; already documented in doc comment |
| 4 | Root not derived from `CARGO_MANIFEST_DIR` | VALID | low | Fix now (cheap, correctness-improving) |

The reviewer's "Notes (checked, no defect)" items are all accurate: no
`build =` line is needed (cargo auto-detects `build.rs`), the script is std-only
(no `[build-dependencies]`), the `panic!`s on I/O errors are correct per
Principle 12, and the emitted `rerun-if-changed` set narrows cargo's rerun scope
to `build.rs` plus the `pack/` tree as intended.
