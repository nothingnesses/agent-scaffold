# Reviewer findings: code-value-audit-static increment 2 (correctness / soundness)

Lens: correctness and soundness of the suppression-marker + FFI source scan
(`scan_source`, `scan_file`, `parse_suppression`, `extract_symbol`, `reclassify`,
`declared_reasons`) and its `run_audit` wiring. Diff range
`686d8ca..79f584b3915f750edc7e92c7aa2d11a89d972598`.

Severity scale (absolute): Critical, High, Medium, Low.

## Verdict

No Critical, High, or Medium findings. The scan is sound on the tested and realistic paths:

- Marker detection is correct for every required form. `#[allow(dead_code)]`,
  `#[expect(dead_code)]`, `#[allow(dead_code, reason = "...")]`,
  `#[cfg_attr(not(test), allow(dead_code))]`, and multi-argument lint lists all match; the
  balanced-paren `attr_args` has no off-by-one (it returns `attr[start..start+offset]`,
  correctly excluding the closing paren) and slices only at ASCII boundaries, so no panic.
  `#[allow(unused)]` / `#[allow(unused_variables)]` correctly do NOT match, and
  `lint_list_has_dead_code` requires an exact `dead_code` token (a `clippy::...` path or a
  `dead_code` mentioned only inside the `reason` text is correctly rejected).
- FFI detection is correct: `#[no_mangle]` (including the `#[unsafe(no_mangle)]` form, since
  it is a `contains` check), `extern "C"` on the item line, and the dedup when both cover one
  site (`recorded_ffi` guard) all behave as the tests assert. I grepped the whole `src/` tree:
  there is no unescaped `extern "C"` or `no_mangle` on any code line outside `src/audit.rs`,
  and inside `src/audit.rs` every occurrence is in a `///` doc comment (skipped) or an
  escaped string literal (`extern \"C\"`, which is not the search string), so the real scan
  produces no false FFI marker.
- Determinism holds: `collect_rs_files` uses `read_dir` (unspecified order) but the caller
  sorts the full `Vec<PathBuf>` (`files.sort()`, line 350) before scanning, and markers are
  appended in line order per file. No `HashMap` is involved anywhere in the scan or
  projection, and record order is preserved into the Markdown. Output is stable across runs.
- File walk is safe on the ordinary error paths: a missing `src/` returns an empty scan
  (`!src.is_dir()` guard, line 344), a non-crate `--dir` likewise, and every `fs` error
  (`read_dir`, `entry?`, `read_to_string`) propagates as `io::Result` up through `run_audit`,
  so failures are loud, not silent. A directory named `foo.rs` is recursed into, not
  mis-collected (the `is_dir()` check precedes the extension check).
- No `unwrap`/`expect`/panicking slice index on a non-test input path. `strip_prefix` uses
  `unwrap_or(file)`, the `as u32` line cast cannot realistically overflow, and every string
  slice in `attr_args`/`extract_reason` is bounded by `find`/balanced-paren offsets at ASCII
  boundaries.
- The `Ffi`-over-`Suppressed` precedence in `reclassify` is intentional and matches the doc
  and the `reclassify_prefers_ffi_when_a_site_carries_both` test; the early `return
  Some(Exclusion::Ffi)` correctly outranks a later suppression at the same site.
- The removal of `CodeValueReport::empty` / `SignalSet::none` is clean: no straggler
  references remain (grep confirms), clippy is green (so they are genuinely unused), and the
  empty-report / all-signals-unrun projection branch is still covered by the `none_report`
  fixture and the two tests that use it (`empty_report_is_caveat_plus_empty_sections`,
  `caveat_is_the_single_sourced_field`). The "Signals run: none" branch is no longer
  reachable in production (the source scan always sets `source_scan: true`), but it remains a
  tested defensive branch, which is fine.
- I verified the smoke test's claimed live sites against the tree: `checks.rs` `budget`
  (attr line 135, item line 137) and `threshold` (140 / 142), and `manifest.rs`
  `description` (`#[expect(dead_code, ...)]` line 80, item line 81). All resolve to the
  asserted symbols. `extract_symbol` correctly handles `pub`, `pub(crate)`, `$vis`, fields,
  and enum variants.

The Low findings below are heuristic limitations that are advisory-tolerable and do not
affect the current tree; I record them because they are real and (for two of them) not
disclosed by the module's own "heuristic" framing.

## Findings

### 1. The line scanner can emit FALSE-POSITIVE markers from block comments, raw strings, and trailing comments; the doc discloses only false negatives

- `src/audit.rs:446-448` (`is_comment`), `src/audit.rs:427` (`extern "C"` item-line check),
  `src/audit.rs:397-409` (`is_attr_line` / suppression push).
- Severity: Low.
- `is_comment` recognises only lines whose trimmed start is `//`, `/*`, or `*`. That misses
  three constructs, each of which yields a spurious marker:
  - A block-comment body line that does NOT start with `*` (e.g. a hand-formatted
    `/* ... */` whose middle line reads `with extern "C" here`) is treated as an item line;
    if it contains `extern "C"` it records a spurious `Ffi` marker (symbol taken from that
    comment text).
  - A raw string literal on an item line, e.g. `let s = r#"extern "C""#;`, contains the
    unescaped substring `extern "C"` and records a spurious `Ffi` marker for that item. (An
    ordinary `"extern \"C\""` string is safe because the inner quotes are escaped; only raw
    strings defeat this.)
  - A commented-out attribute on its own line inside a block comment, e.g.
    `/*\n#[allow(dead_code)]\n*/`, passes `is_attr_line` (it starts with `#[`) and is parsed
    as a real suppression, attaching a phantom fence to the next item.
  The module doc frames the heuristic only as false NEGATIVES ("A marker split across
  lines ... or an unusually written attribute may be missed"); it does not disclose that the
  scan can also over-report. None of these constructs exist in the current tree, so there is
  no live effect today, but the tool accepts an arbitrary `--dir`, so on another crate this
  can inject a spurious FFI/suppression marker (and, via `reclassify` in inc3, hide a real
  dead-code candidate). Advisory-tolerable; worth either a one-line doc addition ("may also
  over-report from raw strings / trailing comments / block-comment bodies") or a cheap guard.

### 2. `reclassify` keyed on `(file, symbol)` collides when one file has two items with the same leading-identifier symbol

- `src/audit.rs:610-628` (the `marker.file ... || marker.symbol != symbol` match, lines
  616-617).
- Severity: Low.
- `reclassify` matches a candidate to any marker in the same file with the same `symbol`
  string, where `symbol` is the leading-identifier heuristic (e.g. `fn new` -> `"new"`). If a
  file contains two items that reduce to the same symbol, one carrying a suppression/FFI
  marker and the other genuinely dead (for example two `fn new` in different `impl` blocks,
  or a shadowed name), a dead-code candidate for the UNmarked item is reclassified as
  excluded (`Suppressed`/`Ffi`) purely because a same-named sibling elsewhere in the file is
  marked. That is a false negative in the tool's core direction (it hides a real candidate).
  The implementer flagged this key choice as a deviation. My judgement: it is acceptable for
  an advisory tool at this stage, and the key is in fact forced, because the stored marker
  line is the ATTRIBUTE line while a rustc diagnostic reports the ITEM line, so `line` cannot
  be part of the key. But it is a real, undocumented false-negative source. It has no live
  caller in increment 2 (increment 3 wires it), so it is safe to land now; I recommend
  documenting the collision on `reclassify` and revisiting it in increment 3 when the caller
  and the rustc-symbol-matching land (that is where a `(file, item-line, symbol)` or
  qualified-path key could be reconstructed).

### 3. `collect_rs_files` follows symlinks via `is_dir()`, so a symlink cycle under `src/` recurses unbounded to a stack-overflow abort

- `src/audit.rs:369-370` (`if path.is_dir() { collect_rs_files(&path, out)? }`).
- Severity: Low.
- `path.is_dir()` follows symlinks, and the recursion has no visited-set or depth bound, so a
  symlink cycle inside the scanned `src/` (e.g. `src/loop -> src`) causes unbounded recursion
  and a stack-overflow abort rather than a clean `io::Error`. This is a pathological,
  effectively self-inflicted input for a tool run against one's own crate, and no real crate
  ships such a cycle, so the practical risk is low; I record it because the walk is not
  cycle-safe and aborts (rather than failing loudly with an error) on that input. A
  `symlink_metadata`/`file_type().is_symlink()` skip, or `read_link` cycle detection, would
  make it robust if that is ever wanted.

## Minor note (not a numbered finding)

`tests/audit_command.rs` still describes increment-1 behaviour in its comments (header
"Increment 1 emits an EMPTY report", and the inline "Increment 1 runs no signal" at lines
5-6, 48, 69). The assertions themselves remain correct (the scratch dirs have no `src/`, so
the scan is empty and `records` is `[]`, `rustc_dead_code` is `false`), and the file was not
touched by this increment, but the comments now misdescribe what runs (the source scan does
run; it simply finds nothing). This is a stale-comment / prose issue, not a
correctness/soundness defect, so I leave it as a note.

## Build reality

- `cargo test`: 366 unit tests passed, 0 failed (including all 12 `audit::tests::*`:
  `marker_scan_resolves_each_attribute_form`, `declared_reasons_are_the_suppressions_only`,
  `reclassify_maps_a_site_to_its_exclusion`, `reclassify_prefers_ffi_when_a_site_carries_both`,
  `extract_symbol_reads_the_annotated_item_name`, `source_scan_finds_known_live_suppression_sites`,
  and the six projection tests). Integration binaries (`audit_command` 5, plus the checks /
  scaffold / validate suites) all passed. No failures, no discrepancy with the implementer's
  claims.
- `cargo clippy --all-targets -- -D warnings`: exit 0, no warnings. The `cfg_attr(not(test),
  allow(dead_code))` on `AuditRecord`, `DeadCodeSource`, `Exclusion`, and `reclassify` hold
  (the release build has no producer for the `DeadCode`/`UnusedDep` variants and no live
  caller for `reclassify` yet, both by design for later increments).
</content>
</invoke>
