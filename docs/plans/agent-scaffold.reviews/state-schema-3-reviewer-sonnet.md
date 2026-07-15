# Review: state-schema increment 3 (sonnet)

Diff range: `61ba68f..fdd3774`. Files changed: `src/main.rs`, `src/plan.rs` (new).

Lens: design consistency and completeness. Reviewed against the `state-schema` Design-pass decisions, the Documentation Protocol (plan and `pack/plan-template.md`), the project Principles, and the decided hard-fail/best-effort split.

## Verified clean

- Roadmap status vocabulary in `ROADMAP_STATUSES` (`src/plan.rs:48`) exactly matches the Documentation Protocol enumeration (`not started`, `in progress`, `complete`, `skipped`, `next`, `optional`, `deferred`) plus the parametric `blocked on <slug>` form, which matches both `docs/plans/agent-scaffold.md` ("blocked on <slug>") and `pack/plan-template.md`. No vocabulary drift between code and prose as of this commit.
- Queue status vocabulary in `question_status_ok` (`src/plan.rs:196-200`) covers `open`, `decided -> folded into <slug>`, and `superseded`, matching the Documentation Protocol in both the plan and the template exactly.
- `validate --plan` hard-fails (exits 1, prints to stderr) on any violation: unknown Roadmap status, duplicate slug, orphaned slug (no Step Detail heading), or unknown queue status. Consistent with the decided hard-fail model.
- `status` best-effort: a missing `--plan` or a missing metrics log leaves that part of the projection empty with no non-zero exit. Consistent with the decided best-effort model.
- `--metrics` default (`docs/metrics/workflow.jsonl`) is the same in both `validate` and `status` (`src/main.rs:324`, `src/main.rs:338`).
- `--plan` is optional in both `validate` and `status`. Consistent.
- `about` string: "Scaffold the agent workflow into a project, and validate or project its state." All three verbs (`scaffold`, `validate`, `status`) now exist, so the expanded description is accurate. The increment-1 narrowing was correct then; the reversal is correct now.
- `status --json` JSON shape: `plan` (nullable) wraps `steps` (`[{slug, status}]`) and `open_questions` (`[{id, status, ask}]`); `metrics` (nullable) wraps `records`. Verified with a live run against `docs/plans/agent-scaffold.md` - valid JSON, correct field set. Nullable plan/metrics are appropriate for a best-effort partial projection.
- `ask` field is present in the JSON question objects and contains the full remaining ask text. No missing key fields.
- 72 tests pass; clippy clean (verified via the test run in this review).
- `status` without `--json` prints a step-count breakdown grouped by status plus open-questions count plus a metrics record count: coherent, readable, not redundant with `--json` (which exposes the full list for downstream processing).

## Findings

### F1 - No drift-guard test for plan/queue vocabulary (low)

**Evidence:** `src/plan.rs:44-48` defines `ROADMAP_STATUSES` as a static array. `src/plan.rs:196-200` defines `question_status_ok` by string prefix. The accepted values are also stated in `pack/plan-template.md:28` (prose) and in the Documentation Protocol section of `docs/plans/agent-scaffold.md`. There is no test that cross-checks the validator's accepted set against the template prose.

**Contrast:** `src/metrics.rs:398-470` has `instrument_prose_documents_every_accepted_schema_value()`, which iterates the validator's own `VARIANTS` arrays and asserts every accepted value appears verbatim in `pack/instrument.md`. If someone updates `pack/instrument.md` without updating the validator (or vice versa), that test fails.

No equivalent guard exists for the plan vocabulary. If someone adds a new status to `pack/plan-template.md`'s Documentation Protocol prose (say `cancelled`) without updating `ROADMAP_STATUSES`, the validator silently rejects valid plans but no test catches the divergence. This is the schema-drift class of defect the metrics module explicitly guards against (Principle 16, one source of truth; Principle 11, tests must exercise the code they claim to).

**Severity:** low. The two sources currently agree; the risk is future drift without a failing test to surface it. Whether to build a guard now or note it for later is a judgment call. The metrics precedent makes "note it" slightly harder to defend since the same problem was solved there.

### F2 - `question_status_ok` over-permissive on `open` and `superseded` (low)

**Evidence:** `src/plan.rs:196-200`:

```rust
fn question_status_ok(status: &str) -> bool {
    status.starts_with("open")
        || status.starts_with("decided -> folded into ")
        || status.starts_with("superseded")
}
```

The Documentation Protocol defines exactly three queue statuses: `open`, `decided -> folded into <slug>`, and `superseded` (`pack/plan-template.md:24`; `docs/plans/agent-scaffold.md` Documentation Protocol). The parametric form `decided -> folded into <slug>` legitimately needs `starts_with`. However, `open` and `superseded` are exact terminal values with no trailing slug. Using `starts_with` for them accepts `opened`, `open (TBD)`, `superseded by Q-5`, etc., which the protocol does not define.

Contrast with `roadmap_status_ok` (`src/plan.rs:191-195`), which uses an exact-match `ROADMAP_STATUSES.contains(&status)` for the enum values and reserves `strip_prefix` only for the genuinely parametric `blocked on <slug>` form.

**Severity:** low. Practical risk is small since the mismatched forms are not natural to write, but the over-permissiveness is an inconsistency with how `roadmap_status_ok` handles the same pattern, and means a mislabeled item like `(open - pending review)` passes without a warning.

### F3 - README does not document `status` or `validate --plan` (low)

**Evidence:** `README.md:116-215` (the Usage section). This section covers the `scaffold` subcommand with examples, flag reference, and interactive-selector key bindings. It does not mention `validate` or `status` at all, despite both now being live subcommands. This is the final increment of `state-schema`; the step is marked "in progress" and this is the review that gates marking it complete.

`pack/instrument.md` does mention `agent-scaffold validate` in its closing line, but that is an injected rendering fragment, not user-facing README prose. A user reading the README Installation and Usage sections finds no indication that `validate` or `status` exist, what their arguments are, or when to use them.

The `--help` output is self-documenting (`agent-scaffold --help` lists all subcommands), so discoverability is not absent, but README coverage is the expected place for usage examples for any non-trivial subcommand.

**Severity:** low. Self-documenting help text covers discovery. No incorrect documentation, only missing documentation.

## No findings at medium, high, or critical

The implementation is consistent with the decided design. The hard-fail vs. best-effort split is correctly realised. Vocabulary is accurate against the plan's Documentation Protocol. JSON shape is sensible and stable. All 72 tests pass.
