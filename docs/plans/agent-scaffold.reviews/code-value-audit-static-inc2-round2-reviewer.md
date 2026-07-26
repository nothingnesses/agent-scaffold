# Reviewer findings: code-value-audit-static increment 2, review round 2

Fresh adversarial re-review of the round-1 fix commit. Fix range `9a7c54d..cbe0074`
(`src/audit.rs`, `src/main.rs`, `tests/audit_command.rs`); full increment `686d8ca..cbe0074`
for regression context. Round 1 raised 7 triager-valid findings (1 medium, 5 low, 1 nit); this
round verifies each fix closed its finding and sweeps for regressions the fixes introduced.
Read-only against the product; this findings file is the only write.

Severity scale (absolute): Critical / High / Medium / Low.

## Verdict

ZERO findings. Clean round. All 7 round-1 fixes are confirmed closed, and the adversarial
regression sweep (the N2 join-key refactor especially) turned up nothing new. `cargo test`: 366
passed, 0 failed (all 12 `audit::tests::*` green). `cargo clippy --all-targets -- -D warnings`:
exit 0, no warnings.

## Per-fix verification (each closed)

- M1 (`--dir` help text, `src/main.rs:550`): CLOSED. The doc-comment now reads "The source scan
  reads `src/**/*.rs` under it now for suppression and FFI markers; the rustc dead-code and
  cargo-machete harvests (later increments) are its remaining readers." Accurate (`scan_source`
  walks `root/src`, `src/audit.rs:342-343`) and self-contained.

- M2 (untested `dead_code`-in-reason negative, `src/audit.rs:1008-1009,1045-1049`): CLOSED and a
  GENUINE guard, not vacuous. New fixture item
  `#[allow(unused, reason = "silences unused, dead_code, unreachable")]` on `fn
  dead_code_only_in_reason`. The reason carries a comma-delimited `dead_code` token, so removing
  the `args.find("reason")` strip in `lint_list_has_dead_code` (`src/audit.rs:519-520`) would let
  `names.split(',')` find `dead_code` in the reason text, emit a Suppression marker with symbol
  `dead_code_only_in_reason`, and fail the `all(... != "dead_code_only_in_reason")` assertion.
  Verified by tracing: with the guard, `names = "unused, "` -> no match, no marker.

- L1 (untested balanced-paren branch, `src/audit.rs:999-1000,1050-1060`): CLOSED and a genuine
  guard. New fixture `#[allow(dead_code, reason = "kept for foo(bar)")]` on `fn paren_reason`; the
  test asserts the whole reason `kept for foo(bar)` resolves. Without the `b'(' => depth += 1` arm
  (`src/audit.rs:502`), `attr_args` closes at the inner `)` after `bar`, `extract_reason` finds no
  closing quote and returns `None`, so the marker's `reason` would be `None != Some("kept for
  foo(bar)")` and the test fails. Exercises the depth-increment arm.

- N1 (undisclosed false positives, `src/audit.rs:340-347`): CLOSED, disclose-only as prescribed.
  The `scan_source` doc now states it "can both UNDER-report and OVER-report" and names all three
  over-report constructs (block-comment body line not led by `*`, raw string with `extern "C"`,
  and an `#[allow(dead_code)]` commented out on its own line). The scanner was NOT hardened:
  `is_comment` (`src/audit.rs:466-468`) is unchanged (still only `//`, `/*`, `*`), matching the
  triage's Minimal-by-default resolution.

- N2 (`reclassify` join-key collision, `src/audit.rs`): CLOSED structurally. `ScannedMarker` gains
  `item_line` (`src/audit.rs:308-310`); `reclassify` takes `line: u32` and joins on
  `marker.item_line != line` (`src/audit.rs:636-640`); two items in one file have distinct item
  lines, so a same-name collision is now impossible. The `DeclaredReason` span still uses
  `marker.line` (the fence line, `src/audit.rs:603`), so the visible report is unchanged (verified
  live: fences anchor at lines 135/140/80/... exactly as round 1 reported). Both reclassify unit
  tests were updated to the new key and remain meaningful (see the regression sweep note below).

- N3 (symlink-cycle stack overflow, `src/audit.rs:374-382`): CLOSED. `collect_rs_files` now
  `continue`s on `entry.file_type()?.is_symlink()` before the `is_dir()` recursion, so a
  `src/loop -> src` cycle is skipped by construction. `file_type()` on a `DirEntry` does not follow
  the link, so the check is correct; an lstat error propagates as `io::Error` (fail-loud).

- nit (stale `tests/audit_command.rs` comments): CLOSED. All three sites (header line 5-6,
  inline lines 48 and 69) reworded to "the scratch dir has no `src/`, so the source scan runs but
  finds nothing," replacing the "Increment 1 runs no signal" framing.

## Adversarial regression sweep (nothing found)

- N2 mis-join / `line` vs `item_line` population: traced `scan_file` (`src/audit.rs:405-454`).
  `item_line` is set only to `line` at the point `trimmed` is a non-attr, non-comment, non-blank
  item line (both the pending-drain push at 438 and the bare-`extern "C"` push at 450); the fence
  line is carried separately as `line: attr_line` (437). `item_line` is therefore always the
  annotated item's line and never the attribute line. Two markers on one item (the `#[no_mangle]`
  + `extern "C"` dedup, and the hand-built `dual` test) correctly share one `item_line`, which is
  the intended dual-marker-site behavior (FFI precedence resolves it). A dangling attribute at EOF
  drains no marker (pre-existing behavior, unchanged). No mis-join introduced.

- reclassify unit tests not weakened: `reclassify_maps_a_site_to_its_exclusion`
  (`src/audit.rs:1127-1148`) looks each item line up by symbol via `find_marker(...).item_line`, so
  it hard-codes no line numbers, and still asserts Ffi/Ffi/Suppressed/Suppressed plus two `None`
  cases (an unmarked line `9999`, and the same item line in a different file). The old `plain_item`
  / `not_dead_code` symbol cases could not be reused (those items produce no marker, so no
  `item_line` exists to look up), and the `9999`-unmarked-line case exercises the same `None` path
  faithfully. `reclassify_prefers_ffi_when_a_site_carries_both` now builds both markers at
  `item_line: 3` on distinct fence lines 1/2 and calls `reclassify(..., 3)`; still meaningful.

- Report byte-identical: `cargo run -- audit --dir . --json` reports `source_scan: true`,
  `rustc_dead_code: false`, `cargo_machete: false`, and the six `declared-reason` fences
  (checks.rs x2, manifest.rs, metrics.rs, next.rs, pack.rs) anchored on their fence lines, matching
  round 1. Zero `src/audit.rs` rows.

- New fixtures do not self-scan: the new fixture lines live in `concat!` string literals (each
  trimmed line starts with `"`, not `#[`), so the live scan of `src/audit.rs` records no marker
  from them (confirmed: zero `src/audit.rs` rows in the live report).

- House style: no non-ASCII in any added line (grep clean).

## Build reality

- `cargo test`: 366 passed, 0 failed, 0 ignored. All 12 `audit::tests::*` pass, including
  `marker_scan_resolves_each_attribute_form`, `declared_reasons_are_the_suppressions_only`,
  `reclassify_maps_a_site_to_its_exclusion`, `reclassify_prefers_ffi_when_a_site_carries_both`.
- `cargo clippy --all-targets -- -D warnings`: exit 0, no warnings.
