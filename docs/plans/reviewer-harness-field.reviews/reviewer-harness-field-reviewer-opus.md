# Reviewer findings: reviewer-harness-field (opus)

Lens: validator correctness, edge cases, backward compatibility. Diff range reviewed: `305ea86..9f87003` (branch `impl/reviewer-harness-field`). Note: the main working tree is checked out at the base commit `305ea86`; the change lives only in `9f87003`. I reviewed the change in a throwaway git worktree at `9f87003` (created and removed under scratch, main repo untouched).

## Verdict

The change is correct, complete for its narrow declared scope, backward compatible, and adequately tested. All 116 tests pass; the real 66-record log validates. No correctness, compatibility, or drift-guard defect found. Findings below are low / informational only.

## Verification performed

- Ran the full suite in the branch worktree: `116 passed; 0 failed`.
- Validated the committed log: `docs/metrics/workflow.jsonl: 66 records, valid` (backward compatibility confirmed against real data; the pre-existing attributed records omit `harness` and still pass).
- Confirmed the reject test produces exactly one error and it is the harness type error (via `one_error`, which asserts `errors.len() == 1`), so the reject test genuinely exercises the new harness branch and not some earlier failure.
- Induced drift in a scratch copy of `pack/instrument.md` (replaced the backtick-wrapped `` `harness` `` mentions): the drift guard `instrument_prose_documents_every_accepted_schema_value` then FAILED with "field `harness` checked by the validator is not documented in pack/instrument.md". So the guard would catch removal of the harness doc. Restored the scratch copy after.
- Checked all four changed files for non-ASCII bytes: clean.

## Correctness of the validator change (src/metrics.rs)

- `harness` is validated string-only-when-present: `require_reviewers` (src/metrics.rs:261-263) gates on `entry.contains_key("harness")` before calling `require_str`. Absent `harness` takes no branch and validates. Correct and matches the plan's "optional, validated as a string when present."
- The four per-reviewer fields stay REQUIRED and unchanged: `role`, `model` (src/metrics.rs:257-258), `raw_findings`, `valid_findings` (src/metrics.rs:259-260) are still unconditional. The `reviewers` array itself stays optional (src/metrics.rs:300-302, `if obj.contains_key("reviewers")`) and still rejects `[]` (src/metrics.rs:245-247). No weakening.
- Error path and message: the new check reuses the `at(...)` position-prefixing closure (src/metrics.rs:254-256), so a bad harness reports `field `reviewers`[0]: field `harness` has wrong type (expected string)`. The reject test asserts exactly this string and passes. Array-position prefix is correct.
- Check ordering: the harness check runs AFTER the four required checks, so a record missing a required field reports that (not the harness error) first. Sensible precedence.

## Drift guard (src/metrics.rs field list)

Sound. `harness` was added to the field list (src/metrics.rs:705 in the branch), the prose carries a backtick-wrapped `` `harness` `` in the reviewers description, and I empirically confirmed the guard fails if that documentation is removed. The plural `harnesses` in the prose is NOT backtick-wrapped, so it does not spuriously satisfy the `` `harness` `` check (the drift test still failed when only the backtick-wrapped forms were removed while plain "harness"/"harnesses" words remained).

## Self-scaffold regeneration

`AGENTS.md` and `.agents/AGENTS.reference.md` were regenerated with the identical harness prose as `pack/instrument.md`; the golden sync test passes as part of the suite. In sync.

## Findings

### Low / informational

- L1 (Principle 11, test strength): The accept test `a_reviewers_element_with_a_valid_harness_is_accepted` (src/metrics.rs, new) does not by itself prove the new harness code runs. Unknown extra fields are permitted (check_record doc, src/metrics.rs:267-268), so a valid string `harness` would validate even if the harness branch did not exist. The Principle-11 load is carried by the reject test (`a_reviewers_element_with_a_non_string_harness_is_reported`): a non-string `harness` can only be rejected by the new branch, otherwise it would be an ignored unknown field. The pair is adequate; noting that the accept test alone is weak.

- L2 (backward-compat coverage): The plan calls out "a reviewer entry without it still validates," but neither NEW test covers the absent-harness reviewer case; it relies on the pre-existing `the_optional_reviewers_field_is_accepted_present_or_absent` test (src/metrics.rs:566, whose reviewers carry no harness) continuing to pass, plus the real-log validation. Coverage is real but indirect. A one-line assertion in the new harness tests (a reviewer with no `harness` validates) would lock the backward-compat intent alongside the harness tests. Not a defect.

- L3 (null handling, informational): An explicit `"harness": null` is REJECTED as wrong-type, not treated as absent, because `contains_key` is true for a present-but-null key and `require_str` then fails. This matches the existing convention for the optional `ts` field (src/metrics.rs:276-278), so it is consistent, not a regression. Flagging only in case a producer might emit `"harness": null` to mean "unknown"; per the prose the correct way to mean unknown is to OMIT the field, so current behavior is defensible.

### Medium / high / critical

None found at any of these severities.

## Non-findings (checked, not defects)

- Empty-string `harness` (`""`) validates. Consistent with `role`/`model`, which are also unconstrained free strings; the schema does not enum-constrain `harness`, and the plan does not ask it to. Fine.
- The drift guard is satisfied by any backtick-wrapped `` `harness` `` anywhere in the prose, not specifically within the reviewers description. This coarseness is pre-existing and applies to every field in the list; it is not introduced or worsened by this change.
- The unique / marginal-valid count is explicitly out of scope (deferred to workflow-calibration) per the plan; its absence is not raised.
