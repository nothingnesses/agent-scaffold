# Reviewer findings: code-value-audit-static increment 2 (contract lens)

Artifact: Q-52 increment 2, the suppression-marker + FFI source scan.
Diff range: `686d8ca..79f584b3915f750edc7e92c7aa2d11a89d972598`.
Changed files: `src/audit.rs`, `src/main.rs`.
Lens: test honesty, scope discipline, output/report contract, house style.

Severity scale (absolute): Critical / High / Medium / Low.

## Summary

Three findings: two Medium, one Low. No new crate dependency was added (`Cargo.toml`
untouched; only `std::{fs, io, path}` pulled in; no `syn`, no walk crate). The report
contract is honest: a run of `audit --dir .` flags `source_scan: true`, keeps
`rustc_dead_code` and `cargo_machete` false with their two Markdown sections empty
(`_None._`), and populates the six live author-declared fences. Clippy is clean at
`-D warnings`; the 12 audit tests pass. The scan of `src/audit.rs` itself does NOT pick up
the fixture's own attribute lines or the module's multi-line `cfg_attr` markers (verified:
the live report contains zero `src/audit.rs` rows). No inc3/inc4 work (rustc harvest,
cargo-machete, flake.nix) leaked in; the caller-less `reclassify` hook is by-design for inc3.

## Finding 1: stale `--dir` help text claims the flag is not read (it is, now)

- File:line: `src/main.rs:550`
- Severity: Medium

The `AuditArgs::dir` doc-comment still reads: "The signal harvests will read it; this tier
accepts it into the CLI contract but does not yet read it." Increment 2 now reads `--dir`:
`run_audit` calls `audit::scan_source(&args.dir)` (`src/main.rs:1260`), which walks
`args.dir/src/**/*.rs`. This `///` line is the clap `--help` text for `--dir`, so the
inaccuracy is user-facing, and it directly contradicts the increment's central behavior
change. The inc2 diff updated the `run_audit` doc-comment (`src/main.rs:1248-1254`) and the
inline comment (`src/main.rs:1257-1259`) but left this arg doc untouched (confirmed: line 550
is not in the diff). Fix: update it to say the source scan reads `src/**/*.rs` under this dir
now, and that the rustc/machete harvests are the later readers.

## Finding 2: the `dead_code`-in-reason-string negative case is untested

- File:line: `src/audit.rs:965` (fixture) and `src/audit.rs:498` (`lint_list_has_dead_code`)
- Severity: Medium

`lint_list_has_dead_code` has an explicit guard so that a `dead_code` appearing only inside a
`reason = "..."` string does not count as a suppressed lint: it truncates `args` at the first
`"reason"` before splitting on commas (`src/audit.rs:499-503`). The behavior is asserted in
the `parse_suppression` doc-comment ("a `dead_code` mentioned only inside a `reason` string
does not match", `src/audit.rs:453-454`). No test exercises this branch. `marker_fixture`
(`src/audit.rs:965`) covers the `#[allow(unused)]` negative (`not_dead_code`) but has no item
whose only `dead_code` occurrence is inside the reason text (for example
`#[allow(unused, reason = "replaces dead_code detection")]`). Removing the `args.find("reason")`
truncation would not fail any existing test: for `allow_with_reason` the lint name still
precedes the comma-split first token either way. This is a genuine test-honesty gap on a
guard whose regression would emit a false-positive `DeclaredReason` fence for an item that is
not a dead-code suppression. The review brief enumerated this exact case as required coverage;
it is the one required fixture form that is missing. Fix: add one fixture item of that form and
assert it produces no marker.

## Finding 3: the balanced-parenthesis reason branch is untested

- File:line: `src/audit.rs:474` (`attr_args`)
- Severity: Low

`attr_args` counts nested parentheses (`b'(' => depth += 1`, `src/audit.rs:480`) so that a
`reason` string containing parentheses does not close the argument list early; the doc-comment
claims exactly this (`src/audit.rs:472-473`). No fixture or unit test has a reason (or any
`allow`/`expect` argument after the keyword) containing an inner `(`, so the depth-increment
arm never executes under test. The `#[cfg_attr(not(test), allow(dead_code))]` fixture does not
reach it, because `attr_args` starts scanning only after the `allow(` keyword and the inner
`not(test)` sits before it. Defensive parsing code with no covering test. Lower severity than
Finding 2 because the tree has no such reason today, so no live misclassification results.

## Checked, not findings

- No new dependency; `Cargo.toml` unchanged; no `syn`.
- FFI path is exercised despite no in-tree instance: the fixture's `exported` (`#[no_mangle]` +
  `extern "C"`, deduped to one marker) and `other_ffi` (bare `extern "C"`) drive
  `marker_scan_resolves_each_attribute_form` and `reclassify_maps_a_site_to_its_exclusion`.
- Ffi-over-Suppressed precedence is genuinely tested: `reclassify_prefers_ffi_when_a_site_carries_both`
  places the suppression first in iteration order, so the early `return Some(Exclusion::Ffi)`
  is what makes FFI win; a broken precedence would fail this test.
- No brittle live-tree total is pinned: `source_scan_finds_known_live_suppression_sites`
  asserts three known sites by symbol via existence checks and pins no count; it exercises the
  real `scan_source` entry point (file walk + read), not just `scan_file`.
- Self-scan safety holds: fixture lines start with `"` (not `#[`), and the escaped `extern \"C\"`
  in the fixture does not match the `contains("extern \"C\"")` real-quote probe; the live report
  has zero `src/audit.rs` rows, confirming the module's own multi-line `cfg_attr` markers and its
  fixture strings are not picked up.
- Removal of `CodeValueReport::empty` / `SignalSet::none` is honest: the all-unrun projection is
  now a test-only fixture (`none_report`, `src/audit.rs`), and the tests that used `empty` were
  repointed to it. No shipped doc/comment still claims the report is empty; the `run_audit` and
  module doc-comments were updated to describe the scan.
- House style: no em-dash / en-dash / double-hyphen-as-dash / emoji / non-ASCII in the diff or
  the commit message; no characteristic-AI filler in the added comments.
- README / step-86 sidecar not made stale: they describe the whole step (which includes the
  source scan), so inc2 implementing it does not contradict them; neither file is in the diff.
