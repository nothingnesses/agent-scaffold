# Reviewer findings: `code-value-audit-static` Increment 1 (contract / docs / test-honesty / scope lens)

Diff range reviewed: `d482c98..bec8d46` (worktree at `bec8d46`). Spec judged against: `docs/plans/code-value-audit-static.build-plan.md` sections 2, 3, 6, 7-inc1, 8-docs.

Method: read every changed file, cross-checked the CLI contract text and the schema against build-plan sections 2/3/6, ran the audit unit tests (`cargo test --bin agent-scaffold audit::`, 6 passed) and the integration tests (`cargo test --test audit_command`, 4 passed), and ran `cargo clippy --all-targets -- -D warnings` (clean, confirming the cfg-split `allow(dead_code)` correctly suppresses the not-yet-constructed schema enums in the release build). Scanned all changed human-facing text and the commit message for non-ASCII (all clean).

Two findings, both low. No high/critical/medium issues found. The CLI contract text, the schema, the single-sourced caveat, the tests, and scope discipline are all sound; details below.

---

## Finding 1 (low): README `audit` subsection describes signals that Increment 1 does not run, without disclosing the report is currently empty

Evidence: `README.md:239-241` (the "Auditing code value" subsection).

The subsection states the command "builds an advisory, static report of code that may not be earning its keep: dead-code and unused-dependency suspicions, plus author-declared suppression reasons ...". In Increment 1 no signal runs: `run_audit` builds `audit::CodeValueReport::empty(task)` (`src/main.rs:1256`), so the report is the caveat plus four `_None._` sections and the line "Signals run: none (this report analysed nothing yet)." A user consulting the README as the source of truth for what the command does today would run it and get an empty report with no explanation in the docs.

The CHANGELOG handled this correctly: its entry closes with "This first increment ships the schema, the projection, and the caveat with an empty report; the signal harvests ... are later increments" (`CHANGELOG.md:11`). The README has no equivalent "current state" note. This is the documentation-currency gap (the planner's phase-2 duty): the README is ahead of the shipped behavior.

Mitigants that hold the severity to low: the runtime report self-discloses ("Signals run: none (this report analysed nothing yet).", `src/audit.rs:362`), the command is advisory-only, and build-plan section 8 did direct describing the signals in the README (though that is the step-level, all-four-increment doc plan, not the Increment-1 charter). A one-clause note mirroring the CHANGELOG ("the signal harvests are later increments; this first cut emits an empty report") would close it.

## Finding 2 (low): the `--dir` help text claims the flag is "recorded", but nothing records it

Evidence: `src/main.rs:547-549` (the `--dir` arg doc): "The Rust crate root to audit (its `Cargo.toml` and `src/`); defaults to the current directory. The signal harvests read it; this tier only records it."

`run_audit` binds `let _crate_root: &Path = &args.dir;` (`src/main.rs:1253`) and then discards it; it is never read again. `CodeValueReport` has no field for the crate root or dir (`src/audit.rs:41-50`: `task`, `generated_from`, `caveat`, `records` only), and `empty(task)` takes only the task. So `--dir` is accepted-and-ignored in this increment, not "recorded" anywhere in the report or the JSON intermediate. The claim "this tier only records it" is inaccurate; even the charitable "records it rather than walking it" reading is false, because there is no place that records it.

This also disagrees with the `run_audit` doc-comment two lines up, which states it accurately: `--dir` "is accepted into the CLI contract now but not yet walked" (`src/main.rs:1235-1236`). The user-facing help is the one that is wrong. Severity is low because the flag is inert either way and this is help-text wording, but the review lens ("is anything in the help text misleading?") catches it: the documented behavior does not happen. A fix is to say the flag is accepted-but-not-yet-read (matching the doc-comment) rather than "recorded".

---

## Checked and clean (not findings, recorded so the orchestrator sees the coverage)

- CLI contract accuracy: the `Command::Audit` doc-comment (`src/main.rs:376`) correctly states advisory, writes-ONLY-its-own-report (`docs/plans/<task>.code-value-report.md`, or `--out`), never-edits-`src/`/`Cargo.toml`/plan/metrics-log, never-deletes, and `--json` prints the intermediate and writes no file. Matches build-plan sections 2 and 6.
- Flags match build-plan section 2: `--plan`, `--source`, `--dir` (default `.`), `--json`, `--out` all present (`src/main.rs:540-559`), deriving `<task>` via `next::derive_task` (`src/main.rs:1252`, `next.rs:993`) with the `task` fallback, and the default output path `default_report_path` returns `docs/plans/<task>.code-value-report.md` (`src/main.rs:1143`). The `--json`-or-write branch mirrors `run_next` and writes via the atomic `plan::write_rendered` (`src/main.rs:1258-1271`).
- README line-208 staleness fixed correctly: the old "Two read-only subcommands ... never write anything" is replaced with an accurate `validate`/`status` read-only statement plus the `audit` read-mostly qualifier (`README.md:208`).
- Schema faithful to build-plan section 3: `AuditRecord` is a closed enum (`DeadCode`/`UnusedDep`/`DeclaredReason`) carrying only each kind's own evidence, the verdict is derived at projection time not stored, `reason: Option<String>` models the real bare-marker case, and no dead Tier-1 placeholder fields were added (`src/audit.rs:126-238`).
- Single-sourced caveat: one `AUDIT_CAVEAT` const read into both the JSON `caveat` field and the Markdown head (`src/audit.rs:34,60,290`), so the two cannot drift; wording faithful to build-plan section 6.
- Test honesty: the golden projection test pins a hand-written expected string over a fixture touching every bucket (`src/audit.rs:497-506`); the two per-record tests together construct every `Signal`/`Marker`/`Exclusion`/`AuditRecord` variant (which the cfg-split `allow` relies on); the `--json`-writes-nothing integration test asserts the default report path does not exist (`tests/audit_command.rs:54`); all four integration tests hit the real binary via `CARGO_BIN_EXE_agent-scaffold` (`tests/audit_command.rs:31`). No vacuous or self-fulfilling assertions of concern. All 10 tests pass.
- Scope discipline: no signal harvester stub, no `flake.nix` or `Cargo.toml` change, no metrics record, no exclusion engine. The `Exclusion` enum is schema only (no computation). Nothing from Increments 2-4 leaked in; everything Increment 1 promised (schema, projection, empty-report subcommand, single-sourced caveat, `--json`-or-Markdown branch) is present.
- Regenerated `docs/plans/agent-scaffold.md` changed in exactly one hunk (the honest-scope note) matching the step sidecar's clause byte-for-byte; no other drift. The sidecar honest-scope clause faithfully reflects build-plan sections 1 and 10 without overclaiming.
- House style: no em-dash/en-dash/double-hyphen-as-dash, emoji, or unicode symbol in any changed file or the commit message (all ASCII; ARROWS/comparisons use `->`, etc.). No characteristic-AI filler phrasing in the README or CHANGELOG prose.
