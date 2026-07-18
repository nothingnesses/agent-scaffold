# Round 3 review (reviewer: opus) - unified waiver model

CLEAN. No valid defect found. A fresh whole-artifact read of the change at HEAD
`f24810c` (branch `impl/waiver-model`, base `506f472`), all gates run, and a
battery of adversarial fixtures aimed at the false-pass class all behaved
correctly. Two pre-existing, already-documented latent risks are noted below as
non-defects for the record; neither is triggered by live data.

## Verdict

The enforcement backstop holds: I could not make `validate --workflow` exit 0
while a `complete` step genuinely lacks convergence and lacks a legitimate
exemption.

## What I verified

### Gates (all green)

- `cargo test --all-targets`: 213 + 1 + 3 tests pass, 0 failed.
- `cargo clippy --all-targets -- -D warnings`: clean, no warnings or errors.
- `cargo run -- validate --workflow --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl`: exit 0, "workflow invariants hold" (105 records, 50 steps, 44 open-questions items).
- `validate --plan` and `validate --metrics` standalone: both exit 0.
- `nix fmt` leaves `src/metrics.rs`, `src/workflow.rs`, `src/plan.rs` unchanged (committed Rust is already formatted). (Note: running `nix fmt` reformatted 7 unrelated tracked files in the worktree; I set those aside with `git stash push` to restore the worktree to HEAD. The stash is left in place. The three reviewed source files were not among the reformatted files.)

### False-pass hunt (the enforcement backstop) - all correctly rejected

- Complete step, no records, no waiver (`pause.md` catch): FAILS (exit 1), "has no round records and no covering waiver". Correct.
- Short risky increment (`peak 1`, needs 2) with an increment-waiver naming a real-but-WRONG step (`step:"alpha"`, `increment:"beta-incB"`): W3 refuses the exemption (`waiver.step != step.slug`) AND W5 flags the mis-scope (`leading_slug(increment) != step`). Both fire, exit 1. Correct.
- Record-backed increment-waiver citing an escalation for an UNRELATED task: W3 exempts on unit+identity, but W5 reports "no `type:\"escalation\"` record ... scoped to this waiver's unit", so the union of problems is non-empty, exit 1. This confirms the orthogonal-but-union design: a present-but-ill-evidenced waiver is exempted by W3 yet reported by W5, so it cannot silently pass.
- Risk-class inconsistency within an increment: `w3_problems` reports and `continue`s before any waiver check, so it is never suppressed by any waiver (verified in code and by the `a_risk_class_inconsistency_is_not_suppressed_by_a_waiver` test).

### `leading_slug` unit-scope exploit - not reachable

- W3 increment coverage requires `waiver.increment == <the matched round's full task>` and `waiver.step == step.slug`, while the round matched only because `leading_slug(round.task) == step.slug`. W5 additionally requires `leading_slug(waiver.increment) == waiver.step`. All three tie back to the same `leading_slug` of the real round task, so there is no cross-step mismatch to exploit: a waiver can only cover the increment whose leading slug is its own step.
- The documented T3 over-strip risk (a step slug that itself ends `-inc<alnum>` collapsing onto a shorter slug) requires such a slug to exist. I enumerated all Roadmap step slugs: none self-over-strip. The only `-inc`-bearing round tasks are `round-log-core-inc{A,B}`, `state-schema-inc{1,2,3}`, `optional-modules-inc*`, all of whose leading slugs are the correct distinct steps. Latent, not triggered.

### Strict-check / best-effort-parse fail-safe

- `check_record`'s `waiver` arm and `parse_waivers` enforce the same load-bearing presence rules (valid `unit`/`reason`/`evidence_tier`, non-empty `step`, `increment` present iff increment-unit, `evidence` present iff record-backed). A waiver breaking any of these is both reported by `check_record` and dropped by `parse_waivers`, so it can never reach W3 as an exemption.
- One asymmetry exists but is not exploitable: `parse_waivers` (like `parse_decisions`/`parse_baseline`) does not re-check the common `task` field that `check_record` requires, so it would project a `task`-less waiver that `check_record` rejects. This does NOT produce a false pass because `run_validate` (src/main.rs:541) ALWAYS runs `metrics::validate_log` over the metrics file in every mode, including `--workflow`; I confirmed empirically that a `task`-less waiver makes both `--metrics` and `--workflow` exit 1 with "missing field `task`". The safety conclusion ("can never silently exempt") therefore holds via the CLI, even though the parse-side half of the belt-and-suspenders is looser than the module doc-comment implies. Flagging as an observation, not a defect: no caller runs `check_workflow` without `validate_log`, and any illegitimate-content waiver must still clear W5 regardless.

### Load-bearing invariants

- `pause.md` catch (complete + no records + no waiver -> FAIL): confirmed empirically and by test.
- Per-increment convergence unchanged: `round-log-core` (low_risk incA / risky incB) passes via per-increment grouping; peak-not-terminal streak is deliberate (T9).
- A step-level waiver does not exempt a short increment, and an increment-level waiver does not satisfy the no-records branch (that needs a step waiver): both confirmed.

### Migration honesty

- 15 waiver records (lines 91-105): 11 step-unit `predates-logging`/self-declared, 3 increment-unit `review-skipped`/self-declared (`convergence-accounting`, `pack-rebuild-tracking`, `user-prompts-dir`), 1 increment-unit `accepted-at-escalation`/record-backed (`optional-modules-inc2cii`).
- 14 Roadmap rows flipped `grandfathered -> complete` (no `trivial` rows existed). The 11 step-waived steps each have zero round records (verified), so they take the step-waiver no-records branch. The 3 review-skipped increment steps each have exactly one `new_valid` round (peak 0/streak short), so they legitimately need an increment waiver whose `increment` equals the bare step slug (none of the three slugs contains `-inc`, so `leading_slug` returns them whole and W5 ownership holds).
- `optional-modules-inc2cii` is the ONLY short `optional-modules` increment: inc1/inc2a/inc2b/inc3 reach low_risk streak 1, inc2ci reaches risky streak 2, inc2cii reaches risky streak 1 (needs 2) and is unstuck by the record-backed waiver whose evidence joins to the real `decision` escalation at line 82. Confirmed.
- The retired `trivial`/`grandfathered` statuses are now rejected by `validate --plan` (plan.rs test updated accordingly), and the whole live artifact passes all three validate modes.

### Tests

Spot-checked the new tests for vacuity: the W5 pairing test exercises all three forbidden and all three valid pairings; the mis-scope and unrelated-escalation tests assert on the specific message text; the parse-drop tests assert exact projected vectors. No vacuous or wrong-asserting test found.
