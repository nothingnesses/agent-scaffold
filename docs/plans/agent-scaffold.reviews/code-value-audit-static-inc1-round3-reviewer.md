# Reviewer findings: `code-value-audit-static` Increment 1, REVIEW ROUND 3 (Q-52)

Fresh, independent, adversarial reviewer. Read-only with respect to the product; I wrote only
this file. Worktree at `ef657d0`. Fix commit under review: `ef657d0` (a one-line doc-comment
change in `src/audit.rs`, closing round-2 finding F1). Full-increment regression context:
`d482c98..ef657d0`. This is the converging round: I (a) confirm F1 closed and (b) re-check the
whole increment with fresh eyes for any regression the fix introduced or anything prior rounds
missed.

## Outcome: ZERO findings (clean round)

F1 is confirmed closed and I found no new issue at any severity. Details below.

## F1 confirmed CLOSED

- The `SignalSet` doc comment (`src/audit.rs:76-79`) now reads: "... Named booleans rather than
  a free `Vec` of signal flags so the \"ran vs not run\" state is explicit and cannot carry a
  duplicate." The dangling `` `Vec<Signal>` `` type name is gone; the replacement is exactly the
  wording the round-2 triager prescribed ("a free `Vec` of signal flags") and keeps the original
  rationale intact (named booleans over a free collection that could carry a duplicate).
- `grep -nE "\bSignal\b" src/audit.rs` returns nothing (exit 1): there is no standalone `Signal`
  token anywhere in the file. `SignalSet` (the surviving type) is not matched by `\bSignal\b`
  because it is a single word, so the grep confirms the removed type is no longer named.
- No regression from the edit: the surrounding sentence still reads correctly with no truncation,
  the type it documents (`SignalSet` with three named `bool` fields) is unchanged, and the
  comment's claim still matches the code (three booleans, fixed order via `each()`).

## Adversarial sweep: everything else checked and clean (not findings)

- Illegal states unrepresentable: `DeadCode.source` is the two-variant `DeadCodeSource`
  (`Rustc`, `SourceScan`) at `src/audit.rs:206-211`, so a machete-sourced dead-code row is not
  constructible; `UnusedDep` (`:159-166`) carries no `source` field, so a non-machete dep row is
  not constructible either. `DeadCodeSource::label` matches both variants exhaustively
  (`:256-261`). The module-doc invariant (`:121-127`) states exactly what the types guarantee,
  with no over- or under-claim.
- Single-sourced labels are consistent between the disclosure and per-row provenance: the three
  `LABEL_*` consts (`:42-44`) are the only label vocabulary; `SignalSet::each` (`:112-118`),
  `DeadCodeSource::label` (`:256-260`), and the inline `{LABEL_CARGO_MACHETE}` in the `UnusedDep`
  projection (`:335`) all read from them. The golden test (`:512-515`) pins one spelling per
  signal in a single rendered report, so the disclosure line and the row provenance cannot drift.
- `--out conflicts_with json`: `#[arg(long, conflicts_with = "json")]` on `AuditArgs::out`
  (`src/main.rs:558-559`); `--dir` is correctly NOT gated (`:550-551`), consistent with the
  triage (a harvest input read under `--json` in later increments). The integration test
  `json_and_out_conflict_is_rejected` (`tests/audit_command.rs:92-115`) asserts non-zero exit,
  that stderr names BOTH flags, and that no file was written.
- Projection total and deterministic: `render_markdown` (`src/audit.rs:298-361`) matches all
  three `AuditRecord` variants exhaustively and the `DeadCode` `exclusion` on `None`/`Some`; it
  adds no `unwrap`/`expect`/index on real input; records keep input order; there is no timestamp
  or other non-determinism. The golden test proves the exact byte output of a fixture touching
  every bucket.
- Caveat single-sourced: `AUDIT_CAVEAT` (`:34`) is written into `CodeValueReport::caveat` by
  `empty()` (`:70`) and the Markdown head reads the field (`:303`), so the JSON `caveat` and the
  rendered head cannot diverge. The `caveat_is_the_single_sourced_field` test pins this.
- Docs accurate: the README `audit` subsection (`README.md:238-254`) discloses the empty report
  ("This first increment ships the schema, the projection, and the caveat with an empty report;
  the signal harvests are later increments.") and the read-mostly contract; the `--dir` help
  (`src/main.rs:550`) reads "accepts it into the CLI contract but does not yet read it"
  (no false "records it"); the `Command::Audit` doc-comment (`:376`) and the `run_audit`
  doc-comment (`:1244-1252`) match the shipped behaviour; the CHANGELOG entry describes the
  schema, the derived verdict, the `--json`/write branches, and the empty-report increment
  accurately. The step-86 sidecar honest-scope clause (`docs/plans/agent-scaffold.md:1119`) is
  accurate (rustc yields zero here; machete plus the kept report are load-bearing; rustc is the
  near-free step-87 baseline).
- Tests genuinely exercise the code: 6 unit tests cover the empty report, the single-sourced
  caveat, the total golden projection, the all-signals-ran disclosure, every marker/exclusion
  label path, and the JSON discriminators; 5 integration tests drive the real binary for
  `--json` (prints, writes no file), the derived default path, the `--out` override, the
  `--json`/`--out` conflict, and the no-source `task` fallback. None is vacuous.
- cfg-split `allow(dead_code)`: both `DeadCodeSource` variants, all `Marker`/`Exclusion`
  variants, and every `AuditRecord` variant are constructed under `cfg(test)`; the
  `not(test)` allow (with an accurate reason) suppresses the no-producer-yet warning in the
  `--all-targets` release build, which clippy `-D warnings` confirms clean.
- House style: the round-3 changed line (and the commit message) are ASCII only, backticks and
  regular hyphens only, no em/en dash, no emoji, no unicode. Conventional `docs:` prefix, no
  agent attribution.

## Build reality (this worktree, toolchain via direnv)

- `cargo test`: green (exit 0). The audit unit tests pass 6/6
  (`agent_scaffold` unittests: `test result: ok. 6 passed; ... 354 filtered out`) and the audit
  integration tests pass 5/5 (`tests/audit_command.rs`). No failure observed in this round; the
  rare pre-existing main-binary parallelism flake noted in round 2 did not recur here and is not
  attributable to this increment.
- `cargo clippy --all-targets -- -D warnings`: clean (exit 0, "Finished" with no warnings).

## Verdict

F1 confirmed closed. Zero new findings at any severity: a clean round. `cargo test` green (audit
6/6 unit + 5/5 integration), `cargo clippy --all-targets -- -D warnings` clean.
