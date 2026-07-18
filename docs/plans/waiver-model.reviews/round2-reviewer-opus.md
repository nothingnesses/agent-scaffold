# Round 2 review: waiver-model (reviewer-opus)

**CLEAN. No new valid defect.** Every round-1 fix is verified closed; the hardened join and O3 predicate do not over-restrict any legitimate case and introduce no new false-pass; the load-bearing invariants hold; all gates are green.

## What I ran

- `cargo test --all-targets`: 211 unit + 1 + 3 integration tests pass, 0 failed.
- `cargo clippy --all-targets -- -D warnings`: clean, no warnings.
- `validate --workflow` on the live repo: `105 records, valid`; `50 steps, 44 open-questions items, valid`; `workflow invariants hold`, rc=0.
- `validate --plan` and `validate --metrics` separately: both rc=0.
- Adversarial end-to-end fixtures for each round-1 defect (below), built and run against HEAD.
- Whole-file re-read of `src/metrics.rs`, `src/workflow.rs`, the `src/plan.rs` diff, and the 15 migration records.

## Round-1 fixes verified CLOSED

- **O1 (unrelated-escalation citation).** New end-to-end repro: `complete` step `victim`, risky increment `victim-incA` at peak 1 (needs 2), escalation `task:"TOTALLY-UNRELATED"` `decision`, record-backed increment waiver citing `evidence:"TOTALLY-UNRELATED"`. Now yields `round log line 4: record-backed waiver cites evidence TOTALLY-UNRELATED but no type:"escalation" record ... is scoped to this waiver's unit`, rc=1. The join at `src/workflow.rs:394-406` ties `waiver.increment == escalation.task` (increment unit) / `leading_slug(escalation.task) == waiver.step` (step unit). Unit test `w5_flags_a_record_backed_waiver_citing_an_unrelated_escalation` covers it. CLOSED.
- **O4 (empty evidence / empty task join).** New repro with `evidence:""` and escalation `task:""`: both are reported malformed (`field task is empty`, `field evidence is empty`) AND the waiver is dropped, so the `alpha-incA` shortfall is re-flagged rather than exempted. Guards confirmed in `check_record` (escalation `task` non-empty `src/metrics.rs:417-419`; waiver `evidence` non-empty `:526`) and in the best-effort projections (`parse_escalations` `:856`, `parse_waivers` `:793`/`:802`). Strict-check and best-effort-parse agree (reported AND dropped). CLOSED.
- **O3 (wrong-step increment waiver).** New repro `{unit:increment, step:alpha, increment:beta-incB}` with `beta` short: `beta` is now flagged (`reached a consecutive-clean streak of 1 ... needs 2`) AND W5 reports `increment waiver names step alpha but increment beta-incB belongs to step beta`. W3 predicate now carries `&& waiver.step == step.slug` (`src/workflow.rs:312`) and W5 the `leading_slug(increment) == step` ownership check (`:372-384`). Unit tests `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment` and `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment` cover both halves. CLOSED.
- **S3 (resume-not-decision escalation).** New repro with a matching-`task` escalation whose `human_decision:"resume"`: W5 reports the record-backed waiver is not scoped/decided; W3 applies the exemption but W5 flags it (orthogonal by design), so overall validate is non-clean. Unit test `w5_flags_a_record_backed_waiver_whose_escalation_resumed_not_decided` present and asserts the scoped-unit message. CLOSED.
- **S1 (migration honesty).** The three b2 waivers (`convergence-accounting`, `pack-rebuild-tracking`, `user-prompts-dir`) now carry `reason:"review-skipped"` / `evidence_tier:"self-declared"` in `docs/metrics/workflow.jsonl`. Each backing round is a single `new_valid`, `consecutive_clean:0`, `low_risk` record (lines 3, 7, 16), so `review-skipped` is the accurate label. CLOSED.
- **O2 (doc softening).** `WaiverUnit` doc (`src/metrics.rs:155-162`) now states an increment waiver "may be a self-declared review-skipped/predates-logging exemption OR a record-backed accepted-at-escalation one", no longer implying escalation-backing is mandatory. CLOSED.
- **S2 (stale baseline comments).** The `baseline` arm and `parse_baseline` doc (`src/metrics.rs:455-474`, `:671-675`) now say the type serves W4 only and that W3's historical exemption is carried by `type:"waiver"` records, not a cutoff. No stray "W3 cutoff on baseline" prediction remains. CLOSED.
- **S4 (bare-slug increment waiver test).** `a_bare_slug_increment_waiver_exempts_a_short_streak` present and passing. CLOSED.
- **O5 (declined).** Confirmed no advisory added; agreed non-blocking.

## New-hole hunt (all probed, none valid)

- **Legit record-backed migration waiver still passes.** `optional-modules-inc2cii` waiver joins to the real `decision` escalation (log line 82); the live `validate --workflow` is green. The hardened join forces `evidence == increment == escalation.task`, exactly the migration convention, so it does not over-restrict it.
- **b2 self-declared increment waivers still exempt.** Live repo passes; each is a low_risk shortfall (peak 0, needs 1) covered by its bare-slug increment waiver, unbroken by the O3 `waiver.step == step.slug` addition (step == increment == bare slug).
- **Legit step-unit record-backed waiver.** Constructed `{unit:step, step:alpha}` backed by escalation `alpha-incQ` `decision`; W5 raises no problem (the only output is an unrelated plan-structure warning from the minimal fixture). The step-unit join arm works.
- **leading_slug over-strip exploit (T3).** Probed two real steps `foo` / `foo-inca` with a round `task:"foo-inca"` and a waiver `{step:foo, increment:foo-inca}`. The strip mis-routes the round to `foo`, but the result FAILS SAFE: step `foo-inca` is flagged (`complete but has no round records and no covering waiver`). No false-pass; the mis-scope always over-reports. This is the pre-existing, documented T3 lexical ambiguity in `leading_slug`, not introduced by the fixes, and no live slug hits it. Over-strip cannot grant an exemption without an actual waiver, and every waiver is still subject to W5.
- **Increment join cannot be split across two escalations.** For the increment unit, `escalation.task == evidence` and `waiver.increment == escalation.task` must both hold for the SAME escalation, forcing `evidence == increment`; two different escalations cannot each satisfy one clause.
- **Fail-safe strict/best-effort split intact.** Walked every waiver malformation (bad `unit`/`reason`/`evidence_tier`, empty `step`, `increment`/`evidence` presence-rule breaks, empties): each is both reported by `check_record` and dropped by `parse_waivers`, so a malformed waiver can never silently exempt.
- **Load-bearing invariants hold.** pause.md catch still fails a `complete` step with no records and no step-waiver, and an increment waiver does NOT rescue the no-records step (`src/workflow.rs:261-263` only consults step-unit waivers). Per-increment convergence unchanged. Risk-class inconsistency is reported before any waiver consultation (`:283-289`, `continue`) so a waiver never suppresses it (`a_risk_class_inconsistency_is_not_suppressed_by_a_waiver`).

## Notes (non-defects, no action)

- `check_record`'s escalation arm re-fetches `task` (`:417`) after the common-field check (`:386`) to test emptiness; harmless redundancy.
- `parse_*` projections are intentionally more lenient than `check_record` on non-semantic fields (e.g. a wrong-typed `ts` is reported by `check_record` but ignored by the projection). This is pre-existing across all record types and does not affect exemption semantics; the malformed line is still reported, so `validate` is non-clean.
