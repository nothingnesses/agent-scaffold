# Reviewer findings: `workflow-invariants` (commit `04e26b1`)

Lens: code quality, test completeness, consistency.

`cargo test`: 108 passed, 0 failed, clean. `cargo clippy --all-targets -- -D warnings`: clean, no warnings.

---

## MEDIUM

### M1: README.md stale - `--workflow` flag is not documented

**File:** `README.md:187-195`

The "Validating and projecting workflow state" section describes `validate` as checking the metrics log and, with `--plan`, the plan structure. It shows no `--workflow` example and gives no hint the flag exists. A user reading the README has no way to discover `validate --workflow`. The `--help` output is complete; the README is not.

The description at line 187 should be extended to mention `--workflow` and a usage example added (parallel to the `--plan` example at line 194).

### M2: `leading_slug` strips an embedded `-inc<word>` that is part of the slug, not an increment marker

**File:** `src/workflow.rs:52-60`

`rfind("-inc")` finds the _last_ occurrence of the literal `-inc` in the task string. A task whose slug naturally contains `-inc` followed by alphanumeric chars (e.g., a step named `foo-incidental`) would have `leading_slug("foo-incidental")` return `"foo"` instead of the intended `"foo-incidental"`, misrouting its rounds to step `foo`. The test suite covers `foo-inc` (unchanged, no alphanumeric suffix) and `foo-inc-bar` (unchanged, non-alphanumeric suffix) but not a slug that actually ends in `-inc<alphanumeric>`.

Concretely: if the Roadmap has both `increment` (complete) and `increment-tracker` (complete), a round logged under task `increment-tracker` would have `leading_slug` return `"increment"`, attributing the round to the wrong step and potentially causing the wrong step to pass or fail W3.

The `rfind` approach is correct for typical task names (the last `-inc<x>` is the increment marker). The vulnerability is a slug that itself ends with that pattern. No current step slug in the plan contains this pattern, so the risk is future-facing, but it is a silent correctness hazard.

---

## LOW

### L1: CHANGELOG not updated

**File:** `CHANGELOG.md` (Unreleased section)

The `[Unreleased]` section has an existing entry but no mention of: the two new Roadmap statuses (`trivial`, `grandfathered`), the `--workflow` flag, or the new `src/workflow.rs` module. These are user-visible additions that belong in the changelog.

### L2: `RiskClass::label` duplicates the `enum_field!` on-disk spellings with no direct test for the `low_risk` case

**File:** `src/metrics.rs:128-133`

`label` re-spells `"low_risk"` and `"risky"` in a `match`, duplicating the strings from the `enum_field!` macro. If `"low_risk"` were renamed in the macro, `label` would silently diverge, and the problem message ("its `low_risk` risk class needs N") would report the old spelling. The `risky` branch is exercised by `a_complete_increment_that_never_reaches_the_streak_is_caught`; the `low_risk` branch is not exercised in any problem-message assertion. Adding a test that fails a `low_risk` step and checks the message text would close both the duplication risk and the coverage gap.

### L3: W3 uses peak `consecutive_clean` rather than the terminal value

**File:** `src/workflow.rs:166-184`

`w3_problems` takes the `max` consecutive_clean seen for each artifact rather than the final logged value. In a correctly-run workflow (the loop stops when the streak is reached), peak equals terminal, so the check is correct in practice. In an anomalous case where the orchestrator logged rounds after convergence and a `new_valid` reset the streak, the peak would still be at the required level while the terminal value would not, and W3 would pass a step that has a later unresolved finding. The spec phrase "must reach the required count" is ambiguous (peak vs. terminal), so this is a design judgment rather than a clear error; noting it as an edge case worth a comment or a deliberate decision would be sufficient.

### L4: Module-level comment in `workflow.rs` calls `skipped` "review-exempt", which conflates two different exemption reasons

**File:** `src/workflow.rs:17-18`

The comment reads: "Steps carrying a review-exempt status (`trivial`, `grandfathered`, `skipped`) are not checked". `trivial` and `grandfathered` are genuinely review-exempt (they declare something about the review history). `skipped` means the step was dropped, not that its review was exempted. The code is correct (W3 checks only `complete`, so all non-complete statuses are excluded), but describing `skipped` as "review-exempt" is inaccurate. A clearer phrasing: "W3 checks only `complete` steps; all others (`trivial`, `grandfathered`, `skipped`, in-flight statuses) are not checked".

### L5: Missing test for an in-flight status (e.g., `in progress`) being exempt from W3

**File:** `src/workflow.rs` tests

The three "exempt" tests cover `trivial`, `grandfathered`, and `skipped`. These are the new or terminal statuses, but the core exemption is `step.status != "complete"`. No test verifies that an `in progress` or `not started` step with matching rounds in the log is not checked. Given the implementation is a single `!= "complete"` comparison, a test exercising one in-flight status would confirm the guard and future-proof against an accidental status-list approach.

### L6: Missing test for multi-artifact convergence where one artifact passes and one fails within the same increment

**File:** `src/workflow.rs` tests

The streak-check code iterates `peak` by artifact and emits one problem per failing artifact. No test exercises an increment with two artifacts where one converges (reaches the required streak) and the other does not. The test `per_increment_grouping_passes_a_step_that_converged_across_two_risk_classes` uses a single artifact per increment. A fixture with two artifacts in one increment, one passing and one failing, would confirm the per-artifact loop reports the failing artifact only.

---

## Not findings

- Non-ASCII: none found in the new files.
- `#[allow]` vs `#[expect]`: no new `#[allow]` attributes in the changed files; the pre-existing one in `src/pack.rs` is out of scope.
- `unwrap`/`panic` on untrusted input: none in the new code.
- Clippy: clean.
- Drift-guard coverage: the `plan_template_documents_every_accepted_status` test anchors each status with a trailing comma (`"trivial,"`, `"grandfathered,"`), so removing either from the template would cause a test failure. The guard is genuine.
- `enum_field!` visibility-token change: adding `$vis:vis` handles the empty-visibility case correctly (existing private enums expand to `enum Name`, unchanged).
- `parse_rounds` vs `validate_log` duplication: both iterate the same JSONL and key off the same field names, but `parse_rounds` is an intentional best-effort projection rather than a validator, and it delegates all parsing of enum values back to the same `RoundOutcome::parse` / `RiskClass::parse` methods, so there is no independent definition of the accepted values.
- `RiskClass::required_streak` constants (1, 2): defined once in code; the comment hardcoding in `w3_problems` and in the module-level doc is prose-only and does not affect logic.
- `--workflow` help text: accurate; it correctly states `complete` steps are checked (not `trivial`/`grandfathered`, which are not `complete`) and correctly notes it requires `--plan`.
- Problem-message format: consistent with the `{path}: {message}` style used by `--plan`; the `{plan} vs {metrics}: {message}` form for `--workflow` is appropriate since a violation is a disagreement between two files.
- AGENTS.md: already anticipated `validate --workflow` at line 83; the line 134 mention ("The log can be checked against this schema with `agent-scaffold validate`") is scoped to the metrics-schema check and does not need updating.
