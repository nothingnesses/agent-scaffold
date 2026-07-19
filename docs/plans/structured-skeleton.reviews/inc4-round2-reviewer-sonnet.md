<!-- reviewer: confirming round 2 (sonnet) -->
<!-- commit: 87fa8fd on base 3f29a81 (fix range 36a872f..87fa8fd) -->
<!-- scope: verification of round-1 fixes (MSG-LOCATOR, S2/L1 comment, L2, S3) -->

# Inc 4 Round 2 Review: Fix Verification

All round-1 fix targets verified below. No new findings.

---

## Verification checklist

### DOC / COMMENT ACCURACY (round-1 item S2 + L1 comment half)

Four strings required to be corrected. Checked against live `--help` output and source.

**(a) `--workflow` clap help.**

Actual text (from `cargo run -- validate --help`):

```
Cross-reference the plan's Roadmap status against the round log (the workflow
invariants): every `complete` step must have converged round records. Reads the
plan from a TOML source (via --source) when it declares `[meta].primary = "toml"`,
else from the Markdown --plan; the round log comes from --metrics (which defaults).
--plan is still required either way.
```

Accurate. Names the TOML arm. States `--plan` is still required. Does not claim the check works without `--plan`.

**(b) `ValidateArgs --source` help.**

Actual text (from `cargo run -- validate --help`):

```
Path to a `<task>.plan.toml` structured source to validate (its schema and internal
cross-references). When omitted, no source is validated. When it declares
`[meta].primary = "toml"`, it also drives the --workflow check (its steps,
questions, waivers, and baseline) instead of the Markdown --plan.
```

Accurate. States the TOML-primary source drives `--workflow`.

**(c) `run_validate` function doc.**

Relevant excerpt (`src/main.rs` around line 667):

```
With `--workflow` (which still requires `--plan`), the plan status is
cross-referenced against the round log: every `complete` Roadmap step must have
converged round records. When `--source` is TOML-primary (`[meta].primary =
"toml"`) the steps, questions, waivers, and baseline are read from it instead of
the Markdown plan (the Inc 4 swap); otherwise the Markdown `--plan` is used.
Either way the check needs the metrics log and the chosen plan source present;
if a needed file is absent it prints a note and is skipped... `--plan` stays
clap-required for now (the relaxation for a TOML-only project is deferred).
```

Accurate. Says `--plan` still required; names the TOML arm; notes the deferral.

**(d) Branch comment (formerly "TOML-sourced: needs only the metrics log").**

Actual text (`src/main.rs` around line 779):

```
// TOML-sourced: the plan is read from the TOML source, so `plan_contents` is
// ignored here; --plan stays clap-required (present but unused) until the
// deferred relaxation. Needs the metrics log for the rounds/decisions/escalations.
```

Accurate. No longer asserts a capability the CLI does not have. Explicitly states `--plan` is still required and is present but unused.

None of the four strings claims `--workflow` works without `--plan`. None over-claims.

### DEFERRAL INTEGRITY

`requires = "plan"` is present at `src/main.rs:381`:

```rust
#[arg(long, requires = "plan")]
workflow: bool,
```

The relaxation was NOT applied. Confirmed.

### ITEM 3 (stderr note on `status --source` parse failure)

The `toml_source` function (`src/main.rs` around line 847) now has three match arms:

- `Ok(source) if source.is_toml_primary()`: returns `Some` (normal path).
- `Ok(_)`: returns `None` silently; the comment reads "a legitimate fallback to `--plan`, documented on the `--source` help, so no note." A Markdown-primary source falls back silently, as documented.
- `Err(_)`: emits `eprintln!("note: --source {} did not parse as a `<task>.plan.toml`; projecting from --plan", path.display())` then returns `None`.

The note fires only on a genuine parse failure. The text is clear and specific. The doc comment for `toml_source` accurately describes all three cases and explains why the parse-error case gets a note while the Markdown-primary case does not.

### ITEM 4 (test comment: `check_workflow_toml_w5_rejects_a_mis_tiered_waiver`)

Updated comment (`src/workflow.rs` around line 1583):

```
// Un-launderable property (mis-tier): ... The step is `in-progress` so W3 does
// not fire, but the record-backed waiver cites an evidence pointer with no
// matching escalation, so TWO W5 problems fire (the missing evidence join AND the
// reason-tier mismatch); the assertion targets the reason-tier one. The message
// names the waiver by its TOML id, not a JSONL log line.
```

Accurate. Names both problems. The assertion was also strengthened to additionally check for `"TOML waiver `w`"`, pinning the locator format.

### CONSISTENCY / Principle 16

`w5_problems` exists exactly once at `src/workflow.rs:522`. It takes `waivers: &[Waiver]`, and all four W5 format strings use `waiver.locator`. There is no per-substrate fork. Confirmed.

The only remaining `"round log line"` string in `workflow.rs` (line 378) is in the W3 consecutive-clean check, where `round.line` is a genuine JSONL line number for a round record. This is correct and unrelated to waivers.

### MSG-LOCATOR fix (round-1 item 1)

`metrics::Waiver.line` replaced by `metrics::Waiver.locator: String`. `parse_waivers` sets `locator: format!("round log line {}", index + 1)`. `waivers_from_toml` sets `locator: format!("TOML waiver `{}`", waiver.id)`. The `position` counter in `waivers_from_toml` was removed. Both TOML W5 negative tests assert `"TOML waiver `w`"` in the problem message. No stale locator in any W5 message.

### Regression

- `cargo test --all-targets`: 293 + 1 + 3 pass, 0 fail.
- `cargo clippy --all-targets -- -D warnings`: clean.
- Live path (`validate --plan docs/plans/agent-scaffold.md --workflow`): exits 0, "workflow invariants hold".
- `docs/plans/agent-scaffold.md`: untouched by the fix commit (confirmed via `git diff 36a872f 87fa8fd -- docs/plans/agent-scaffold.md` returning empty).

---

## Verdict

CLEAN. All five round-1 findings are correctly addressed. No new findings. The file is authoritative.
