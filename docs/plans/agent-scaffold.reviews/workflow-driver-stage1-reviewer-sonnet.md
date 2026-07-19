# Review: workflow-driver-stage1 (Reviewer 2 of 2)

Lens: CLI/API surface, output-contract determinism, test coverage, scaffold consistency, style.
Diff: `main..impl/wf-driver-stage1` (main `0167972`, branch `2b8f535`).
Key files reviewed: `src/next.rs`, `src/main.rs`, `src/workflow.rs`.

## Findings

### R2-1 (medium): Missing `requires = "resume"` on `StatusArgs.ledger_fragment`

`src/main.rs:420-422`

```rust
/// Path to the ledger fragment to read the `## RESUME STATE` block from (with --resume). ...
#[arg(long)]
ledger_fragment: Option<PathBuf>,
```

The `--ledger-fragment` flag on `status` carries a "(with --resume)" note in its help text but has no `#[arg(long, requires = "resume")]` annotation. Running `agent-scaffold status --ledger-fragment some.md` (without `--resume`) is accepted silently, exits 0, and prints the normal status projection with `--ledger-fragment` unused. The user gets no error and no indication the flag was ignored.

The established pattern in the same file is `#[arg(long, requires = "workflow")]` at line 398 for `workflow_spec` in `ValidateArgs`, with an explicit note: "the flag is meaningless without it, and would otherwise leave a malformed spec unparsed and exit 0." The same reasoning applies here.

Confirmed by running the binary:
```
$ agent-scaffold status --ledger-fragment nonexistent.md
plan: not provided
metrics: 134 records
```
Exit 0. The flag was silently ignored.

Fix: add `#[arg(long, requires = "resume")]` to `StatusArgs.ledger_fragment`, matching the `workflow_spec` precedent.

### R2-2 (low): `identical_inputs_give_identical_bytes` is tautological as a cross-run determinism test

`src/next.rs:1328-1333`

```rust
fn identical_inputs_give_identical_bytes() {
    let one = serde_json::to_string_pretty(&golden_projection()).unwrap();
    let two = serde_json::to_string_pretty(&golden_projection()).unwrap();
    assert_eq!(one, two);
    assert_eq!(render_human(&golden_projection()), render_human(&golden_projection()));
}
```

`golden_projection()` is a pure function with no I/O, no randomness, and no wall-clock reads. Calling a deterministic pure function twice in the same process and asserting equality is trivially true and cannot catch any real non-determinism bug. The build plan says the test intent is "same inputs -> identical bytes across two runs" (cross-run determinism), but the test only checks within a single invocation.

The real non-determinism risks are addressed structurally (`BTreeMap` for context ordering, no wall-clock, paths echoed verbatim) and the golden tests (`golden_human_text`, `golden_json`) provide the meaningful coverage by asserting exact byte output against hardcoded constants. The `identical_inputs_give_identical_bytes` test adds no additional assurance beyond what the golden tests already give.

This is low severity because the actual determinism properties ARE guaranteed structurally and via the golden tests; the vacuous test is just a weak proxy for the real claim.

## Clean areas (no findings)

The following items were specifically checked and found sound.

**Transition table coverage:** All 8 rows are covered by tests with the named `_row` pattern. The `done_row_metadata` test correctly pins the `Done` variant's metadata rather than exercising it through the selection path (which never produces it, as intended).

**Q-54 human-input-contract reminder:** The reminder text is in `LoopState::Escalate.base_reminders()` only (`src/next.rs:273-276`). The test `the_human_input_contract_reminder_is_present_only_at_escalate` verifies it is present at `Escalate` and absent at `AwaitingReviewers`. Since `base_reminders()` is a pure match on state, the single non-escalate check is sufficient.

**`extract_resume_state` test coverage:** Three tests cover the three required scenarios: verbatim extraction (`resume_state_is_extracted_verbatim`), absence returns `None` (`resume_state_absent_is_none`), and termination at the next `## ` heading only (`resume_state_terminates_at_the_next_level_two_heading_only`).

**Golden tests:** `golden_human_text` and `golden_json` are real `assert_eq!` byte-compares against hardcoded string constants. They cover the output-contract determinism requirement meaningfully.

**Dual-source parity test:** `toml_and_markdown_sources_give_the_same_verdict` asserts `step`, `increment`, and `state` equality. For this fixture all other fields (`risk_class`, `consecutive_clean`, `required_streak`, `total_rounds`) are derived from the same round fixtures and cannot differ between sources. The assertion is sufficient for the fields that can vary.

**Differential test:** `next_agrees_with_w3` calls `assert_differential` with five distinct round sets covering converged and unconverged cases for both `low_risk` and `risky` classes. The assertion `next_converged == w3_clean` is non-tautological and exercises the shared `peak_consecutive_clean` arithmetic correctly.

**`w3_problems` pub(crate) deviation:** The differential test at `src/next.rs:1129` calls `crate::workflow::w3_problems` directly. Without `pub(crate)` the test could not compile. The deviation from the build plan (which specified only `peak_consecutive_clean` as pub) is justified by the test requirement and leaves no dead code.

**`blocked` state reachability:** `select_active_loop` in `src/next.rs:482-507` has a third branch (lines 500-504) that fires when pending steps exist but none have blockers met, correctly producing `LoopState::Blocked`. The `blocked_row` test covers it.

**`#[cfg_attr(not(test), allow(dead_code))]` on `Done` variant:** `src/next.rs:169`. In the test build `Done` is constructed by `done_row_metadata` so `dead_code` does not fire there, and no suppression is needed. In the non-test build `Done` would trigger `dead_code`, so `allow` (not `expect`) is correct per the project convention for cfg-split test-only constructions. The code comment explains this explicitly.

**No `#[allow]` used where `#[expect]` is required:** The only non-test `allow` in new code is the cfg-split case above. `run_checks` in `src/workflow.rs:202` correctly uses `#[expect(clippy::too_many_arguments, ...)]`.

**No em-dashes, unicode, or emoji:** No disallowed characters were found in new code, comments, or user-visible strings.

**Output-contract determinism structural checks:** `context` is `BTreeMap<String, String>` (deterministic iteration order); no wall-clock or timestamps; paths echoed with `.display().to_string()` (verbatim, not canonicalized); reminders are from a fixed `&[&str]` array (fixed order).

**`run_next` mirrors `run_status`:** Both resolve the plan source with the same `toml_source`/`steps_from_*` logic, use the same `derive_task` for the ledger path default, and handle missing files as best-effort (no hard failure).

## Summary

2 findings total: 1 medium, 1 low.

R2-1 (medium): `status --ledger-fragment` accepts and silently ignores the flag without `--resume` (missing `requires = "resume"` annotation). Exit 0 with false confidence.

R2-2 (low): `identical_inputs_give_identical_bytes` test is a within-invocation tautology; does not exercise the cross-run determinism the build plan specifies.
