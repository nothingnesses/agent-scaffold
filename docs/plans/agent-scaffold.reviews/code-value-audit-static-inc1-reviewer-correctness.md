# Reviewer findings: correctness / soundness (Q-52 `code-value-audit-static`, Increment 1)

Diff range `d482c98..bec8d46`. Lens: correctness and soundness. Read-only review; I
wrote only this file. Build reality verified in the worktree: `cargo test` (360 + 4 + 1 +
3 + 1 + 2, all pass), `cargo clippy --all-targets -- -D warnings` clean.

## Summary of what I confirmed sound (not findings, recorded for the orchestrator)

- Projection totality and determinism: `render_markdown` (`src/audit.rs:285`) matches all
  three `AuditRecord` kinds exhaustively, keeps records in input order, uses no timestamps
  and no map iteration, and has no `unwrap`/`expect`/indexing that can panic on real input.
  The only `unwrap`s are in `#[cfg(test)]` code and the integration test harness.
- Single-sourced caveat: `AUDIT_CAVEAT` (`src/audit.rs:34`) is the only source; the JSON
  field is `report.caveat` (`src/audit.rs:60`) and the Markdown head reads the same field
  (`src/audit.rs:290`), so the two cannot drift. The `caveat_is_the_single_sourced_field`
  test pins this.
- `--json` writes nothing and mirrors `run_next`: `run_audit` (`src/main.rs:1257`) takes
  the JSON branch with only `println!`, no file write; the default branch writes via
  `plan::write_rendered` (atomic temp-then-rename, `src/plan/render.rs:583`). `--out`,
  `derive_task`, and the no-source "task" fallback are all consistent with `next`.
- The cfg-split `#[cfg_attr(not(test), allow(dead_code, ...))]` (`src/audit.rs:119`,
  `184`, `203`, `222`) is correct rather than `expect`: the `LoopState::Done` precedent it
  cites is accurate (`src/next.rs:218`), every variant IS constructed under `cfg(test)`
  (verified: `AuditRecord` x3, `Signal` x3, `Marker` x2, `Exclusion` x4 all appear in the
  test fixtures), and `--all-targets` clippy still enforces `dead_code` in the test build
  (it passed clean, so the enforcement is real). `expect` would be the wrong choice here
  because a later increment adding release-build producers for all variants would make an
  enum-level `expect` unfulfilled and break the `-D warnings` build mid-transition, whereas
  `allow` degrades gracefully.
- The `let _crate_root: &Path = &args.dir;` seam (`src/main.rs:1259`) is a sound, honest,
  documented way to read the clap field so it is not `dead_code`; it hides no real problem.
- The tests' `head[..]` indexing (`tests`/`src/audit.rs:469-472`) is safe: the empty report
  always renders more than three lines, so `take(3)` always yields three elements.

## Findings

### 1. Two independent label vocabularies name the same three signals differently

- Evidence: `SignalSet::each` (`src/audit.rs:102-108`) labels the signals
  "rustc dead-code", "source suppression / FFI scan", "cargo-machete unused dependencies";
  `Signal::label` (`src/audit.rs:242-248`) labels the same three "rustc dead-code",
  "source scan", "cargo-machete". These are two separate, hand-maintained label sets for
  the same three signals (the `SignalSet` booleans map one-to-one onto the `Signal`
  variants).
- Severity: low.
- Why it is a problem: in a single rendered report the disclosure line reads
  "Signals run: ... source suppression / FFI scan, cargo-machete unused dependencies." while
  a row beneath reads "(from source scan)" / "(from cargo-machete)". A reader sees the same
  signal under two different names and cannot tell from the text that "cargo-machete" and
  "cargo-machete unused dependencies" are the same oracle. Because the two label sets are
  independent, they can also drift further as later increments touch one and not the other.
  The `golden` and `every_signal_marker...` tests pin both spellings, so the divergence is
  locked in, not accidental. A fix would single-source the per-signal label (one function
  keyed by the signal identity) so the disclosure and the row provenance agree, the same
  discipline already applied to `AUDIT_CAVEAT`.

### 2. `--json` and `--out` are not mutually exclusive; `--out` is silently ignored

- Evidence: `AuditArgs` (`src/main.rs:540-559`) declares `json: bool` and
  `out: Option<PathBuf>` with no `conflicts_with`; `run_audit` (`src/main.rs:1261-1273`)
  takes the `--json` branch first and never reads `args.out` in it.
- Severity: low.
- Why it is a problem: `agent-scaffold audit --json --out reports/x.md` prints JSON to
  stdout and writes no file, silently discarding the user's explicit `--out`. The user asked
  for a file at a named path and got none, with a zero exit and no diagnostic. That is a
  small "fail fast and loudly" gap: an explicitly-provided flag is dropped without a word.
  A fix would mark `out` (and arguably `dir`, which is also inert under `--json` in this
  increment) as `conflicts_with = "json"` so clap rejects the combination, or at minimum
  note in the `--out` help that it is ignored with `--json`.

### 3. The `source: Signal` field admits provenance-illegal record/signal combinations

- Evidence: `DeadCode.source: Signal` (`src/audit.rs:138`) and `UnusedDep.source: Signal`
  (`src/audit.rs:151`) are each a free `Signal` with no per-variant constraint. Nothing in
  the type prevents `UnusedDep { source: Signal::RustcBuildJson, .. }` or
  `DeadCode { source: Signal::CargoMachete, .. }`, both semantically impossible (rustc does
  not report unused deps; machete does not report dead code).
- Severity: low.
- Why it is a problem: the module doc (`src/audit.rs:111-113`) and the CHANGELOG assert the
  schema makes "illegal states unrepresentable" and that each variant "carries ONLY its own
  evidence". That claim holds for the cross-variant evidence fields (a dep row genuinely
  cannot carry a symbol span, a dead-code row cannot carry a machete caveat), but the
  provenance field is unconstrained, so a class of illegal state IS representable and the
  stated invariant is weaker than advertised. This may be intentional: the
  `every_signal_marker...` test deliberately constructs `DeadCode { source: SourceScan, .. }`
  (a source-scan-discovered item projected as a dead-code exclusion), and the build plan's
  section 3 shows `source: Signal` as a general field. If that generality is wanted, the
  invariant claim in the doc should be narrowed to the cross-variant-evidence property it
  actually guarantees; if it is not wanted, the provenance should be constrained per variant
  (for example a two-variant `DeadCodeSource` / `DepSource`) so the impossible pairings
  cannot be built. Flagging for the orchestrator to confirm intent rather than as a certain
  defect.
