# Round 5 (confirming) review: unified waiver model (Q-44 pilot 2)

Reviewer: independent fresh read, sonnet. HEAD 7be5c2a on `impl/waiver-model`, based on main 506f472.

## Summary

CLEAN. No valid defect found. The round-4 low finding (empty-task escalation guard untested) is correctly fixed by the new test in 7be5c2a. All 15 waivers are honest, the Roadmap maps 1:1, prose and code agree on every surface, and all gates pass.

## What I verified

### Migration honesty (all 15 waivers)

Derived each waiver independently from the pre-waiver round records in docs/metrics/workflow.jsonl.

**11 predates-logging step waivers** (core-assets, file-dropper, idempotency-safety, selection-ui, mode-enum, tag-selection, available-filter, pack-manifest, external-packs, pack-owned-principles, init-vcs): confirmed zero round records for every one of these tasks before the waiver records appear. No `type:"round"` line with any of these task values exists. The waiver type (step-unit, predates-logging, self-declared) is the correct form for a step with no review records.

**3 review-skipped increment waivers** (convergence-accounting, pack-rebuild-tracking, user-prompts-dir): each has exactly one round record with `outcome:"new_valid"`, `consecutive_clean:0`, `risk_class:"low_risk"`. Low-risk requires a streak of 1; these have peak 0. W3 takes the short-streak increment path (not the zero-records path), so an increment waiver is correct. The `increment` token is the bare step slug (no `-inc` suffix), which matches the round record's `task` value; `leading_slug` returns the full string for bare slugs, satisfying both the W3 match and the W5 slug-ownership check. The reason `review-skipped` is the best available vocabulary for "the change round was logged but no clean round followed."

**1 accepted-at-escalation record-backed increment waiver** (optional-modules-inc2cii): 5 rounds for this task (`outcome` sequence: new_valid, new_valid, new_valid, new_valid, clean with `consecutive_clean:1`), `risk_class:"risky"`, peak streak 1 (needs 2). The backing escalation record exists: `{"type":"escalation","task":"optional-modules-inc2cii","human_decision":"decision"}`. The waiver's `evidence` field points to `"optional-modules-inc2cii"`, which equals the escalation's `task`. W5 scope check (increment-unit): `escalation.task == waiver.increment` holds. The reason/tier pairing (`accepted-at-escalation` / `record-backed`) is the one valid combination for this reason. Honest.

Roadmap count: 14 rows flipped from `grandfathered` to `complete`, 1 row flipped from `in progress` to `complete` (`optional-modules`). Total 15 rows. 15 waivers. 1:1 mapping. `waiver-model` row stays `next`.

### Plan scope constraint

The entire diff of docs/plans/agent-scaffold.md is table rows inside the Roadmap table. Every non-table-row diff line is a header or separator, confirmed by filtering the diff. The Status line, Step Details blocks, and Open Questions section are untouched.

### Consistency

- AGENTS.md and .agents/AGENTS.reference.md: byte-identical (diff produces no output).
- docs/plans/TEMPLATE.md and pack/plan-template.md: byte-identical.
- pack/instrument.md documents the `waiver` record type with all fields (`unit`, `step`, `increment`, `reason`, `evidence_tier`, `evidence`), the three reason values, the two tier values, and the two unit values, matching the code's enum VARIANTS. The schema drift-guard test (`instrument_prose_documents_every_accepted_schema_value` in metrics.rs:1438) iterates `WaiverUnit::VARIANTS`, `WaiverReason::VARIANTS`, and `EvidenceTier::VARIANTS` and asserts each appears backtick-quoted in pack/instrument.md; this passes.
- CHANGELOG.md: retires `trivial` and `grandfathered` in the right entry (same unreleased cycle), describes W3 as convergence-OR-waiver, and describes W5 as the orthogonal integrity check.
- src/plan.rs: `ROADMAP_STATUSES` no longer includes `trivial` or `grandfathered`; the plan validator test is updated to assert their rejection and pin that `skipped` stays accepted.
- src/workflow.rs module doc: W3 described as convergence-OR-waiver; W5 described as orthogonal. Matches implementation.
- No stale `baseline-cutoff` reference in any public prose surface.
- Residual `trivial`/`grandfathered` in the codebase are all legitimate: test fixture strings testing their rejection (plan.rs:529-530), the CHANGELOG retirement entry, historical narrative prose in the plan's Status line (not parsed by any validator), and the `Classification` enum in metrics.rs (which is for the `intake` record's classification field, entirely distinct from Roadmap statuses).

### Tests

**New test** (`an_escalation_with_an_empty_task_is_reported`, metrics.rs:1316-1322): asserts `one_error` returns `"field \`task\` is empty"` for an escalation with `"task":""`. This is non-vacuous: without the empty-check guard at metrics.rs:417-419, `require_str` would return the empty string without error and `one_error` would panic (it asserts exactly one error). The test directly pins the guard added in the main feature commit.

**W3 coverage** (confirmed complete): zero-records step caught without waiver; zero-records step exempt with step-unit waiver; waiver for wrong step does not exempt; step-unit waiver does not cover short-streak increment (units are distinct); increment-unit waiver covers short-streak increment; risk_class inconsistency not suppressed by waiver (continue before waiver lookup); mis-scoped increment waiver (step mismatch) does not exempt; bare-slug increment waiver exempts correctly.

**W5 coverage** (confirmed complete): nonexistent step; increment waiver naming wrong step (leading_slug mismatch); record-backed waiver with no matching escalation; record-backed waiver with a matching decision escalation (passes); `resume` escalation does not satisfy decision requirement; step-unit join via leading_slug (passes and fails with different step); unrelated escalation does not satisfy scope check; all three invalid reason/tier pairings flagged; all three valid pairings accepted. End-to-end test mirrors the live optional-modules migration shape.

All 214 unit tests, 1 checks test, and 3 scaffold tests pass. `validate --workflow` on the live plan and log exits 0 with "workflow invariants hold."

### ASCII cleanliness

`git diff 506f472..HEAD` contains no non-ASCII bytes (confirmed with LC_ALL=C grep).

### Files touched

Exactly the 11 intended files: .agents/AGENTS.reference.md, AGENTS.md, CHANGELOG.md, docs/metrics/workflow.jsonl, docs/plans/agent-scaffold.md, docs/plans/TEMPLATE.md, pack/instrument.md, pack/plan-template.md, src/metrics.rs, src/plan.rs, src/workflow.rs. No unintended files.
