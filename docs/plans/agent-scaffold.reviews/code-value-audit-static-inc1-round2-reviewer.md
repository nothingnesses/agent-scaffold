# Reviewer findings: `code-value-audit-static` Increment 1, REVIEW ROUND 2 (Q-52)

Fresh, independent, adversarial reviewer. Read-only with respect to the product; I wrote only
this file. Worktree at `d8f2841`. Fix commit under review: `1426e42..d8f2841` (README.md,
src/audit.rs, src/main.rs, tests/audit_command.rs). Full-increment regression context:
`d482c98..d8f2841`. Round 1 raised 5 findings (all low, all triaged VALID); this round (a)
verifies each fix closed its finding and (b) adversarially sweeps the revised artifact for any
NEW problem the fixes introduced.

Build reality verified in this worktree (toolchain via `direnv`):

- `cargo clippy --all-targets -- -D warnings`: CLEAN (finished, no warnings). The `--all-targets`
  release build enforces `dead_code`, so the two-variant `DeadCodeSource` cfg-split
  `allow(dead_code)` is still correct: both variants are constructed only under `cfg(test)`
  and the `not(test)` allow suppresses them in the release build with no producer yet.
- `cargo test`: the audit unit tests (6/6) and the audit integration tests
  (`tests/audit_command.rs`, 5/5, including the new conflict test) pass. Full-suite result:
  see the flakiness note at the end (one rare, pre-existing parallel-run flake in the MAIN
  binary, NOT attributable to this fix; 15 of 16 full runs green).

## Round-1 fixes: all five confirmed CLOSED

### CORR-1 (single-sourced labels): CLOSED

Three module constants now hold the canonical per-signal labels (`src/audit.rs:42-44`:
`LABEL_RUSTC_DEAD_CODE = "rustc dead-code"`, `LABEL_SOURCE_SCAN = "source scan"`,
`LABEL_CARGO_MACHETE = "cargo-machete"`). Every projection site reads from them:
`SignalSet::each` (`src/audit.rs:112-118`), `DeadCodeSource::label` (`src/audit.rs:256-260`),
and the `UnusedDep` projection (`src/audit.rs:335`, an inline `{LABEL_CARGO_MACHETE}`). No
second independent label vocabulary remains: the old `Signal::label` function is gone with the
enum, and grep for the old long forms ("source suppression / FFI scan", "cargo-machete unused
dependencies") returns nothing in `src/`, `tests/`, `README.md`, or `CHANGELOG.md`. The golden
and disclosure tests were updated to the single spelling (`src/audit.rs:488,513,522,583-585`),
so the two surfaces cannot drift. (The implementer collapsed to the short label everywhere
rather than the triage's optional "short label plus fixed suffix" derivation; that is a valid
way to close CORR-1, since the core requirement was one oracle, one spelling.) The `AUDIT_CAVEAT`
prose still names the signals descriptively ("source suppression markers", "cargo-machete's
source-grep heuristic"); that is a deliberate explanatory register, was not part of CORR-1's two
label-function scope, and is unchanged by the fix, so it is not a regression.

### CORR-2 (`--json`/`--out` conflict): CLOSED

`--out` now carries `#[arg(long, conflicts_with = "json")]` (`src/main.rs:556-557`), so clap
rejects `audit --json --out ...` at parse time. `--dir` was correctly NOT gated
(`src/main.rs:551`: `#[arg(long, default_value = ".")]`, no `conflicts_with`), consistent with
the triage (it is a harvest input read under `--json` in later increments). The new test
`json_and_out_conflict_is_rejected` (`tests/audit_command.rs:92-115`) is not vacuous: it asserts
non-zero exit (`assert!(!output.status.success(), ...)`), that stderr names BOTH flags
(`stderr.contains("--out") && stderr.contains("--json")`), and that no file was written
(`assert!(!dir.join("custom/report.md").exists())`). Verified passing against the real binary.

### CORR-3 (provenance constrained): CLOSED

`UnusedDep.source` is dropped entirely (`src/audit.rs:156-166`: the variant now carries only
`crate_name`, `manifest`, `caveat`); `DeadCode.source` is narrowed to a two-variant
`DeadCodeSource { Rustc, SourceScan }` (`src/audit.rs:206-211`, field at `:151-152`). The
provenance-illegal combinations (a dep row from rustc, a dead-code row from machete) are no
longer constructible. The free-standing `Signal` enum is removed: grep for `Signal::`,
`enum Signal`, finds nothing in `src/` or `tests/` (one stale doc reference remains, see the
finding below). The module-doc invariant (`src/audit.rs:121-127`) now states exactly what the
types guarantee (dep row has no `symbol`/`source`; dead-code row has no `caveat`; `DeadCode.source`
is `DeadCodeSource`, never machete) with no overclaim or underclaim. The projection stays total
and deterministic over the new types (the `DeadCode` arm matches on the 2-variant
`DeadCodeSource`, exhaustive; records keep input order; no timestamps). All fixtures/tests updated
(`Signal::RustcBuildJson -> DeadCodeSource::Rustc`, `Signal::SourceScan -> DeadCodeSource::SourceScan`,
`UnusedDep` no longer sets `source`).

### CONTR-1 (README disclosure): CLOSED

The README `audit` subsection (`README.md:241`) now ends "This first increment ships the schema,
the projection, and the caveat with an empty report; the signal harvests are later increments.",
mirroring the CHANGELOG entry (`CHANGELOG.md:11`).

### CONTR-2 (`--dir` help): CLOSED

The `--dir` help (`src/main.rs:550`) now reads "The signal harvests will read it; this tier accepts
it into the CLI contract but does not yet read it.", dropping the false "records it" and matching
the accurate `run_audit` doc-comment ("accepted into the CLI contract now but not yet walked",
`src/main.rs:1254`).

## New issue introduced by the fixes

### F1 (low): a doc comment still names the now-removed `Signal` type

- Evidence: `src/audit.rs:78` (the `SignalSet` type doc): "... Named booleans rather than a
  `Vec<Signal>` so the \"ran vs not run\" state is explicit and cannot carry a duplicate."
- Severity: low.
- Why it is a problem: the CORR-3 fix removed the free-standing `Signal` enum (renamed to
  `DeadCodeSource` and narrowed to two variants with a different meaning). Before the fix, `Signal`
  existed with three variants mapping one-to-one onto `SignalSet`'s three booleans, so `Vec<Signal>`
  was a coherent "rejected alternative" the comment could point at. After the fix, `Signal` is not a
  type in the codebase at all, and the nearest surviving type (`DeadCodeSource`) has only two
  variants and does not correspond to `SignalSet`'s three signals, so the comment names a type that
  no longer exists and could not sensibly represent this struct's state. This is exactly the
  "a doc still naming the removed `Signal` enum" class the schema change was supposed not to leave
  behind: a reader who greps `Signal` to understand the reference finds only this dangling mention
  and no definition. No compile or runtime effect (comment only), and the design rationale it
  conveys (named booleans over a free collection that could carry a duplicate) is still valid in
  spirit, so it is low. A one-word fix (for example "a `Vec` of signal enums" or "a free list")
  removes the dangling type name.

## Adversarial sweep: everything else checked and clean (not findings)

- No other stale reference to the removed enum or the dropped field: grep for the old labels,
  `Signal::`, `enum Signal`, and an `UnusedDep`-with-`source` all come back empty across `src/`,
  `tests/`, `README.md`, and `CHANGELOG.md`. The CHANGELOG's audit entry does not name `Signal` or
  a per-row `source` field, so it needed no change and has none. (The build-plan sketch at
  `docs/plans/code-value-audit-static.build-plan.md:107-127` still shows the old `Signal` enum, but
  that is a historical design doc, not a shipped artifact, and is out of the fix's scope.)
- The label single-sourcing changed the disclosure line's user-visible text
  ("source suppression / FFI scan, cargo-machete unused dependencies" -> "source scan, cargo-machete").
  Every test that pinned the old spelling was updated in the same commit (the `golden`,
  `all_signals_run_disclosure...`, and `every_signal_marker...` tests, plus the empty-report
  contains-checks); no test was left asserting the old text, and no constant is unused (all three
  `LABEL_*` are read).
- The cfg-split `#[cfg_attr(not(test), allow(dead_code, ...))]` reason text is still accurate after
  the variant-set change: both `DeadCodeSource` variants (`Rustc`, `SourceScan`) are constructed
  under `cfg(test)` (`populated_report` and the `every_signal_marker...` fixture), `Marker` and
  `Exclusion` unchanged, and `--all-targets -D warnings` stays clean.
- No new panic path, non-determinism, or fail-fast gap: the projection adds no `unwrap`/`expect`/
  index on real input; the new conflict rejection is a clap parse-time refusal (fail fast and
  loudly) with no file write; the new test's `scratch("json-out-conflict")` dir is unique per test
  (keyed by pid plus a distinct name) and only sets the CHILD process cwd, so it adds no cross-test
  interference.
- House style: no em-dash/en-dash/double-hyphen-as-dash, emoji, or unicode in any changed line or in
  the commit message (all ASCII; comparisons/arrows use `->` etc.). Commit message uses a
  conventional `fix:` prefix and no agent attribution.

## Note (not a finding against this fix): rare pre-existing full-suite flake

In 16 full `cargo test` runs, one run reported `FAILED. 358 passed; 2 failed; 0 filtered out` in
the MAIN unittest binary (360 tests); the other 15 runs were green, and `cargo test --bin
agent-scaffold` in isolation passes 360/360 every time. The flake is in the main binary, NOT in the
new `tests/audit_command.rs` conflict test, and is consistent with pre-existing parallel-run
filesystem contention among integration binaries rather than anything this fix introduced (the fix
adds no shared mutable state to the main binary's unit tests). Recorded for the orchestrator's
awareness of suite hygiene; it is not attributable to Increment 1's changes and is not a round-2
finding.

## Verdict

All five round-1 fixes are confirmed closed. One new low finding (F1: a dangling `Vec<Signal>` doc
reference the schema rename left behind). No high/critical/medium issue. `cargo clippy --all-targets
-- -D warnings` clean; the audit tests pass.
