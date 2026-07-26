# Triage: `code-value-audit-static` Increment 1 (Q-52) review round

Adjudicator: TRIAGER, independent of implementer and orchestrator. Read-only with respect to
the product; I wrote only this file. Worktree at `bec8d46`; change under review is
`git diff d482c98 bec8d46`. Intended scope judged against
`docs/plans/code-value-audit-static.build-plan.md` Increment 1 (section 7): schema plus
Markdown projection plus empty-report `audit` subcommand plus single-sourced caveat; no
signal harvesting. Principles cited by name from `docs/plans/agent-scaffold.plan.toml`
`[[principle]]`.

## Round outcome

- 5 findings judged: 5 VALID, 0 INVALID, 0 severity-adjusted.
- Severities: all 5 low. There is NO high, critical, or medium finding in this round
  (confirmed by reading each cited site, not just trusting the reviewers' labels).
- Because no finding is high/critical, and nothing is being dismissed, NO dismissal
  backstop re-check is needed. All five are accepted as valid low findings for the
  implementer to fix (or the orchestrator to defer at low severity with reasoning).

## Per-finding verdicts

### CORR-1 (two label vocabularies for the same three signals): VALID (low)

Confirmed against the code. `SignalSet::each` (`src/audit.rs:102-108`) names the three
signals "rustc dead-code", "source suppression / FFI scan", "cargo-machete unused
dependencies"; `Signal::label` (`src/audit.rs:242-248`) names the same three "rustc
dead-code", "source scan", "cargo-machete". Two of the three diverge, and the `SignalSet`
booleans map one-to-one onto the `Signal` variants, so this is one oracle spelled two ways.
The golden test (`src/audit.rs:503`) pins both spellings in a single rendered report
("Signals run: ... cargo-machete unused dependencies." in the disclosure line, "(from
cargo-machete)" in a row), so a reader sees the same signal under two names and the sets can
drift as later increments touch one and not the other. Low is the right severity: advisory
output, no correctness impact, purely a consistency/drift concern.

Recommended fix: single-source the per-signal human label, the same discipline already
applied to `AUDIT_CAVEAT`. Key the label off the signal identity once and have both the
`SignalSet` disclosure and the per-row provenance read from it. If the disclosure wants a
longer phrase than the terse per-row label (for example "source suppression / FFI scan" as
the scan's whole-job description versus "source scan" as a row's provenance), derive the
long form from the single short label plus a fixed suffix, so the two cannot disagree on the
oracle's name. This serves "Structured data first, project for humans" (one source, two
projections) and the one-source-of-truth thinking the plan already applies to the caveat.

### CORR-2 (`--json` and `--out` not mutually exclusive; `--out` silently ignored): VALID (low)

Confirmed. `AuditArgs` (`src/main.rs:543-559`) declares `json: bool` and `out:
Option<PathBuf>` with no `conflicts_with`; `run_audit` (`src/main.rs:1262`) takes the
`--json` branch first and never reads `args.out`, so `audit --json --out reports/x.md`
prints JSON to stdout and writes no file, discarding an explicitly-provided flag with a zero
exit and no diagnostic. That is a real "fail fast and loudly" gap. It is unique to `audit`
among the commands: `next --json` has no `--out` to drop, and `render` writes a fixed path.
Low is right: advisory tool, the user still sees output (JSON on stdout), only the named file
is missing.

Recommended fix: add `conflicts_with = "json"` to the `out` arg (`#[arg(long, conflicts_with
= "json")]`) so clap rejects the combination up front. This serves "Make illegal states
unrepresentable" (P5): the impossible request is rejected at parse time rather than silently
half-honoured. Do NOT also add `conflicts_with` to `--dir`, contrary to the reviewer's
parenthetical: `--dir` is a harvest INPUT, not an output selector, and the later harvests
(Increments 3-4) read it under `--json` as much as under the write branch. It is inert under
`--json` only in Increment 1 because no harvest runs at all, so making it conflict with
`--json` would be wrong for the settled design. `--dir` is instead addressed by CONTR-2.

### CORR-3 (`source: Signal` admits provenance-illegal combinations): VALID (low)

Confirmed, and the "confirm intent" the reviewer asked for resolves as follows.
`DeadCode.source: Signal` (`src/audit.rs:138`) and `UnusedDep.source: Signal`
(`src/audit.rs:151`) are each an unconstrained `Signal`, so `UnusedDep { source:
RustcBuildJson }` and `DeadCode { source: CargoMachete }` are representable though
semantically impossible. The module doc's specific claim (`src/audit.rs:111-113`) is only
about CROSS-VARIANT evidence (a dep row cannot carry a symbol span; a dead-code row cannot
carry a machete caveat), and that claim IS true. So the doc is not false, but the schema
admits a class of illegal state that "Make illegal states unrepresentable" (P5) says to
encode out. Low severity: no runtime consequence in Increment 1 (only the tests construct
these variants; there are no producers yet), so this is a schema-hardening opportunity, not
a live bug.

Resolution: (b) constrain provenance per variant, in preference to (a) narrow-the-doc, but
scoped more tightly than the reviewer's literal "DeadCodeSource / DepSource" sketch. The
right encoding, confirmed by the intent the tests and the build plan show:

- `UnusedDep.source`: DROP the field entirely. Only `cargo-machete` ever produces an
  unused-dependency row (build plan section 4c), so the field is fully determined by the
  variant. A one-variant `DepSource` would be pointless; removing the field is strictly
  cleaner. This serves "Minimal by default" (P2): a field the variant already implies is
  redundant, and removing it makes the machete-only provenance the only representable state
  for free. The projection (`src/audit.rs:322-326`) then renders the machete label as a
  constant for `UnusedDep` rows.
- `DeadCode.source`: KEEP a source distinction but CONSTRAIN it to a two-variant enum
  (Rustc, SourceScan). DeadCode genuinely has two oracles: the rustc harvest (section 4a)
  AND the source scan for cfg-gated items rustc never compiles (the
  `every_signal_marker...` test at `src/audit.rs:538` deliberately builds `DeadCode { source:
  SourceScan, exclusion: CfgGated }`, and section 5 explains why). It never comes from
  machete. So the field carries real information but must exclude the machete case. This
  serves "Make illegal states unrepresentable" (P5) while keeping the Rustc-versus-SourceScan
  distinction the projection shows the human.

Why (b) over (a), by principle: P5 is a named plan principle whose text is explicit ("encode
[the valid states], rather than admitting bad states and guarding against them"), so
narrowing the prose while leaving the illegal state representable (a) is the exact
anti-pattern P5 names. "Prefer the cleaner long-term architecture over the smallest diff"
(P1) tells you to take the cleaner design unless a concrete limitation prevents it, and none
does here. This is also the cheapest possible moment to do it: Increment 1 is the
schema-only increment, only the module and its tests construct these variants (the whole
point of the cfg-split `allow(dead_code)`), so (b) touches no harvester and costs almost
nothing now versus reworking the producers in Increment 3. A side benefit: after this, the
free-standing `Signal` enum is no longer referenced by any variant field (SignalSet uses
named booleans, not `Signal`), so it can likely be removed, a further "Minimal by default"
win. If the orchestrator wants to minimise Increment-1 churn, (a) (state the doc claim as the
cross-variant-evidence property it actually guarantees) is an acceptable low-cost fallback,
but it leaves the P5 gap for Increment 3 to inherit, so I recommend (b) now.

### CONTR-1 (README describes signals as implemented, no empty-report disclosure): VALID (low)

Confirmed. `README.md:241` describes the dead-code, unused-dependency, and suppression
signals as if they run today, with no note that Increment 1 emits an empty report;
`CHANGELOG.md:11` does disclose it ("This first increment ships the schema, the projection,
and the caveat with an empty report; the signal harvests ... are later increments"). A reader
using the README as the source of truth for today's behaviour runs `audit` and gets an empty
report the README does not explain. This is a documentation-currency gap (the README is ahead
of shipped behaviour). Note this is NOT a scope violation: build plan section 8 directed
describing the signals in the README, so the description is plan-sanctioned; only the
missing current-state disclosure is the defect. Low is right: the runtime report
self-discloses ("Signals run: none (this report analysed nothing yet).", `src/audit.rs:362`),
the command is advisory-only.

Recommended fix: add one clause to the README `audit` subsection mirroring the CHANGELOG,
for example "the signal harvests are later increments; this first cut emits an empty report."
Match the CHANGELOG's disclosure so the two documentation surfaces agree.

### CONTR-2 (`--dir` help says "records it", but nothing records it): VALID (low)

Confirmed. The `--dir` help (`src/main.rs:550`) ends "The signal harvests read it; this tier
only records it." `run_audit` binds `let _crate_root: &Path = &args.dir;`
(`src/main.rs:1259`) and immediately discards it; `CodeValueReport` has no crate-root or dir
field (`src/audit.rs:41-50`: `task`, `generated_from`, `caveat`, `records` only), and
`empty(task)` takes only the task. So `--dir` is accepted-and-ignored, not "recorded"
anywhere in the report or the JSON intermediate; even the charitable "records rather than
walks" reading is false because nothing records it. The help also contradicts the accurate
`run_audit` doc-comment two lines up (`src/main.rs:1252-1254`: "accepted into the CLI contract
now but not yet walked"). Low is right: the flag is inert either way; this is help-text
wording.

Recommended fix: change the `--dir` help to match the doc-comment, for example "The signal
harvests will read it; this tier accepts it into the CLI contract but does not yet read it."
Drop the word "records". This keeps the user-facing help honest and consistent with the
internal doc-comment.

## Note on scope discipline (recorded, not a finding)

Both reviewers independently confirmed Increment 1 stayed in scope (no harvester, no
`flake.nix`/`Cargo.toml` change, no metrics record, no exclusion engine; the schema enums are
schema-only), the single-sourced caveat holds, the projection is total and deterministic, and
all tests pass with clean clippy. None of the five findings disputes that; all five are
low-severity refinements to a change that is otherwise sound and in-scope.
