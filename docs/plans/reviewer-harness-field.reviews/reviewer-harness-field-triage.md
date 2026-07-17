# Triage: reviewer-harness-field

Adjudicated against `git diff 305ea86..9f87003` (change lives only in `9f87003`; main checked out at base). Verified each claim against the actual code in the branch.

Summary: no implementer-owned defect blocks convergence. One valid orchestrator-owned finding (CHANGELOG, applied at merge). Two valid low implementer-owned readability/consistency nits (both one-line edits in `src/metrics.rs`) that may be batched into an optional single cleanup round; they do not block convergence. All three opus findings are acceptable-as-is (no action).

Distinct findings after dedup: 6 (sonnet M-1, L-1, L-2; opus L1, L2, L3). No overlap between the two reviewers.

---

## sonnet M-1: missing CHANGELOG entry

- Verdict: VALID
- Severity: medium (as a class-of-change judgement); non-blocking at this stage
- Reasoning: `CHANGELOG.md` has an actively maintained `[Unreleased]` section, and it already carries a sibling entry for the "models or harnesses" guidance generalisation. This commit adds `harness` to the `reviewers[]` schema that ships verbatim in every scaffolded `AGENTS.md`, so it is a user-visible schema/doc addition of the same class and is tagged `feat:`. An entry is warranted. Confirmed no `[Unreleased]` entry exists for this commit.
- Owner: ORCHESTRATOR. Under REVIEW-IN-WORKTREE / MERGE-ON-CONVERGENCE the CHANGELOG is orchestrator-owned on main and its entry is applied at merge time. Its absence in the worktree diff is by design, not an implementer defect.
- Round needed: No implementer round. Resolved by the orchestrator applying the `[Unreleased]` entry at merge.

## sonnet L-1: drift-guard field array ordering

- Verdict: VALID (cosmetic)
- Severity: low
- Reasoning: Confirmed in the branch the array is `... "model", "harness", "raw_findings", "valid_findings" ...`, i.e. `harness` inserted between `model` and `raw_findings`. The validator order in `require_reviewers` is `role, model, raw_findings, valid_findings`, then optional `harness` last (src/metrics.rs:257-266), and the prose phrases `harness` as the trailing "plus an optional string `harness`". So the array's placement does not match either the validator's order or the prose. Note the array is already a flat, de-duplicated set of field names (round-level `valid_findings` appears once and stands in for the per-reviewer one too), so it was never a strict structural mirror; the comment says it "mirrors what `check_record` matches on and requires", a loose grouping claim. The test checks presence, not order, so it still passes. Impact is limited to readability of a test comment for a future reader. Real but trivial.
- Owner: IMPLEMENTER (`src/metrics.rs`).
- Round needed: Not on its own. If an implementer round is spawned for L-2, move `"harness"` to after `"valid_findings"` in the same edit. Does not block convergence.

## sonnet L-2: `require_reviewers` doc-comment purpose clause not updated

- Verdict: VALID
- Severity: low
- Reasoning: Confirmed. The doc comment's field description was extended to mention the optional `harness` string (src/metrics.rs:228-230), but the purpose clause still reads "calibrate reviewer productivity and whether running multiple models earns its cost" (src/metrics.rs:231-232), while the corresponding `pack/instrument.md` prose was updated to "running multiple models or harnesses can be calibrated". A genuine, minor code-doc-vs-prose inconsistency: the function now validates harness attribution but its stated purpose omits it. One-line fix ("models or harnesses").
- Owner: IMPLEMENTER (`src/metrics.rs`).
- Round needed: If fixed, an implementer edit spawns one review round. Recommend batching with L-1 into a single low-severity cleanup round. Non-blocking; the orchestrator may also converge and defer both as accepted lows.

## opus L1: accept test alone does not prove the new branch runs

- Verdict: VALID as an observation; NOT a defect. No action.
- Severity: informational
- Reasoning: Correct that `a_reviewers_element_with_a_valid_harness_is_accepted` would pass even without the new branch, since unknown extra fields are permitted. But the Principle-11 proof is carried by the reject test `a_reviewers_element_with_a_non_string_harness_is_reported`: a non-string `harness` can only be rejected by the new branch (otherwise it is an ignored unknown field), and opus independently confirmed via `one_error` that the reject test produces exactly the harness type error. The pair is adequate. Acceptable as-is.
- Owner: n/a. Round needed: No.

## opus L2: no explicit NEW absent-harness test

- Verdict: VALID as an observation; NOT a defect. No action required.
- Severity: informational
- Reasoning: The "reviewer entry without harness still validates" intent is covered indirectly by the pre-existing `the_optional_reviewers_field_is_accepted_present_or_absent` test (whose reviewers carry no harness) plus the real 66-record log validating. Coverage is real, if indirect. A one-line absent-harness assertion in the new tests would localise the backward-compat intent but is a nice-to-have, not a defect, and out of the decided narrow scope.
- Owner: IMPLEMENTER only if the orchestrator chooses to strengthen coverage. Round needed: No (optional; not required for convergence).

## opus L3: explicit `"harness": null` rejected as wrong-type

- Verdict: VALID as an observation; NOT a defect (working as intended). No action.
- Severity: informational
- Reasoning: `contains_key` is true for a present-but-null key, so `require_str` fails and `"harness": null` is reported as wrong-type rather than treated as absent. This matches the existing convention for the optional `ts` field, so it is consistent, not a regression. The prose states the correct way to mean "unknown" is to omit the field. Defensible and intended.
- Owner: n/a. Round needed: No.

---

## Out of scope (not raised as findings)

- Unique / marginal-valid reviewer count: explicitly deferred per the plan; its absence is not a valid finding. Neither reviewer raised it; both correctly noted it as out of scope.

## Disposition

- Convergence is not blocked by any implementer-owned defect.
- Orchestrator action at merge: add the `[Unreleased]` CHANGELOG entry (M-1).
- Optional single implementer cleanup round may address L-1 (move `"harness"` after `"valid_findings"` in the field array) and L-2 (update the doc-comment purpose clause to "models or harnesses"); both are one-line edits in `src/metrics.rs`. If taken, it is one review round covering both. These are accepted lows and may instead be deferred.
- opus L1/L2/L3 require no action.
