# Inc 1 (structured-skeleton) round-2 confirming review

Reviewer: independent (did not write this code; adversarial verification). Round: 2 (confirming). Increment: `structured-skeleton-inc1`, rebased onto current `main` (which includes merged Inc 2). Full increment reviewed: `git diff 8d98967 27bd647` (base = current `main`, HEAD = `27bd647`); the fix is the second commit `27bd647` ("fix: harden plan.toml validation..."), the original is `0ec8cce`.

Severity scale: `low` / `medium` / `high` / `critical`. ASCII: `->`, `<->`, `>=`, `!=`.

## Verdict: CLEAN

No new defects found after a full adversarial pass. All ten round-1 remediation items (1-9 plus the O4 human decision) landed and are non-vacuous. The rebase onto Inc 2 is semantically sound. The increment converges.

Method: read the whole rebased diff and the full `27bd647:src/plan/source.rs`; cross-read `27bd647:src/metrics.rs` and `27bd647:src/workflow.rs`; re-read the round-1 triage. Built and ran the worktree read-only: `cargo test --all-targets` (262 passed: 258 lib + 1 + 3), `cargo clippy --all-targets -- -D warnings` (clean), and the live path `validate --metrics ... --plan ... --workflow` (all three summaries green, exit 0).

## Remediation confirmations (non-vacuous)

1. [med] question status <-> field integrity: CONFIRMED. `folded_into` is resolved UNCONDITIONALLY (`source.rs:417-424`, outside any status arm), as is `superseded_by` (`426-433`). Status<->field consistency is enforced both ways by two exhaustive matches over the 4-variant closed enum: `Decided` requires `folded_into` / all others forbid it (`435-450`); `Superseded` requires `superseded_by` / all others forbid it (`452-467`). No `_` wildcard, so a future variant forces a compile error here. Tests `a_decided_question_with_a_dangling_folded_into_is_flagged`, `a_non_decided_question_with_a_dangling_folded_into_is_flagged`, `a_non_decided_question_carrying_folded_into_is_flagged`, `a_superseded_question_with_no_superseded_by_is_flagged`, `a_non_superseded_question_carrying_superseded_by_is_flagged`, `a_dangling_superseded_by_is_flagged` all present and targeted.
2. [med] waiver `id` uniqueness + well-formedness: CONFIRMED. `source.rs:365-375` enforces `is_well_formed_token` and plan-wide uniqueness via a `BTreeSet` across all steps' waivers. Tests `a_duplicate_waiver_id_is_flagged` (two steps, shared id) and `a_malformed_waiver_id_is_flagged` (`-bad-`).
3. [low] duplicate `principle.n`: CONFIRMED. `source.rs:378-383`. Test `a_duplicate_principle_n_is_flagged`.
4. [low] self-reference in `blocked_by`/`folds`: CONFIRMED. `source.rs:388-407` flags `target == step.slug` as a self-edge (before the dangling branch). Tests `a_self_blocking_step_is_flagged`, `a_self_folding_step_is_flagged`. Multi-step cycle detection intentionally absent (comment `385-387`), as scoped.
5. [low] orphan token duplicate / equals real slug: CONFIRMED. `source.rs:488-501` rejects duplicates and any token present in `slugs`. Tests `a_duplicate_orphan_task_is_flagged`, `an_orphan_task_equal_to_a_step_slug_is_flagged`.
6. [O4 = STRICT, human decision] `deny_unknown_fields`: CONFIRMED and correctly scoped. Applied to exactly the eight plan.toml schema structs (`PlanToml`, `Sidecars`, `Meta`, `Step`, `Increment`, `Waiver`, `Question`, `Principle`) in `source.rs`; NOT applied to the enums (they use `rename_all`/per-variant rename, which is correct). Test `a_typoed_plan_key_fails_to_parse` proves `blockd_by` now fails at parse. CRITICAL check: `grep deny_unknown_fields src/metrics.rs` returns ZERO hits, so the append-only JSONL log stays forward-compatible. Confirmed independently that the metrics structs (`Round`, `Decision`, `Waiver`, ...) derive only `Debug, Clone, PartialEq, Eq` (no serde) and are parsed by hand from `serde_json::Value`, so the enum serde derives added by `enum_field!` are inert on the JSONL path and consumed only by the TOML schema.
7. [low] lowercase kebab-case for slugs and increment ids, uppercase orphan suffix allowed: CONFIRMED. `is_kebab_case_token` (`source.rs:295-297`) used for step slugs (`328`) and increment ids (`340`); the looser `is_well_formed_token` used for waiver ids (`368`) and orphan tasks (`490`). Tests `an_uppercase_step_slug_is_flagged`, `an_uppercase_increment_id_is_flagged`; the fixture's `round-log-core-incA` orphan token validates clean, proving the split is real. 8/9. [low] new tests for previously-unexercised status variants and dangling targets: CONFIRMED. `the_unexercised_status_variants_round_trip` parses/validates/round-trips `not-started`, `skipped`, `optional`, `deferred`, and `exploring`; the two dangling-`folded_into` tests plus `a_dangling_superseded_by_is_flagged` cover the dangling-target arms. All assert on specific needles or specific parsed values (non-vacuous).

Additional round-1 items also confirmed: the W5 `required_tier()` refactor is single-sourced (`metrics.rs:209-215`, called from both `workflow.rs:475` and `source.rs:564`); it is a keep-and-document item (no code defect).

## Rebase soundness: SOUND

Inc 1 and Inc 2 both edited `src/metrics.rs` and `src/workflow.rs`; the textually conflict-free rebase is also semantically sound.

- `metrics.rs`: the `enum_field!` serde derives + `WaiverReason::required_tier()` coexist with Inc 2's changes; the enum derives do not touch the JSONL parse path (hand-written from `serde_json::Value`), so no behavior change to the log. No double-application, no dead code.
- `workflow.rs`: the W5 pairing now reads `waiver.reason.required_tier() == waiver.evidence_tier` (`475`); the `WaiverReason` type import is correctly dropped (only a method call remains, no type reference), while `EvidenceTier` (used at `445`) and `question_id_index` (used at `203`, Inc 2's join accessor) stay imported and used. Imports are exactly what the body needs; nothing unused, nothing missing.
- Both Inc-2 W5 tests pass (`w5_passes_a_step_unit_record_backed_waiver_joined_by_leading_slug`, `w5_flags_each_inconsistent_reason_tier_pairing`), so Inc 2's join-by-leading-slug logic and Inc 1's pairing refactor combine correctly. No lost check.

## Adversarial checks that came up empty (ruled out)

- Over-strict rejection: none found. Free-text fields (`title`, `ask`, `text`, `name`, `note`) are not tokenised. `receipt` and `w4_baseline` are shape-checked (`Q-<n>`) but not required to resolve inside the TOML, which is correct since a `receipt` points at a JSONL decision record, not a TOML question. Forbidding `superseded_by` on a `decided` question is correct under the single-status model (a question is one status, not both).
- Never-firing checks: none. Every problem branch is reached by a passing test needle or by the clean-fixture negative (the fixture exercises every region and validates empty).
- Vacuous tests: none. `assert_flags` asserts a specific substring; `a_typoed_plan_key_fails_to_parse` additionally asserts `parse_toml(...).is_err()`; the round-trip tests assert specific parsed enum values and struct equality.
- Waiver `increment` scoping: correctly checked against the step's own increments (`source.rs:510-511`), not the plan-wide set, so a waiver cannot borrow another step's increment id. Correct.
- `main.rs` `--source` wiring: additive and mirrors `--plan` (absent path -> note to stderr, skipped; problems feed the shared exit code). The clean-path re-parse-for-counts has a documented `Err(_) -> "valid"` fallback that cannot fire after a clean validate. Not a defect.

## Tally

- critical: 0
- high: 0
- medium: 0
- low: 0

Clean confirming round. Increment converges.
