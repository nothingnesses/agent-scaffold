# Round 5 (confirming) review: unified waiver model (Q-44 pilot 2)

Reviewer: independent adversarial read, opus. HEAD 7be5c2a on `impl/waiver-model`, based on main 506f472.

## Summary

CLEAN. No valid defect found. The enforcement backstop is sound: I could not construct a case where `validate --workflow` exits 0 while a `complete` step lacks both convergence and a legitimate, declared exemption. The four load-bearing invariants hold, the migration is honest and mechanically valid, and all gates pass. The one round-4 low finding (empty-`task` escalation guard) is fixed and its new test is non-vacuous.

## What I verified

### Enforcement backstop (false-pass class)

- pause.md catch (W3, no-records path). A `complete` step with no matching round records is exempt ONLY by a step-unit waiver (`unit == step && step == slug`); an increment waiver does not satisfy this branch (workflow.rs:258-270). Confirmed in code and empirically (crafted `ghost`/empty-log fixture exits 1 with the catch message).
- Per-increment convergence (W3, short-streak path). Records are grouped by full `task`; each increment's peak `consecutive_clean` must reach the class streak (`low_risk` 1, `risky` 2) UNLESS an increment waiver covers it with `unit == increment && increment == <full task> && step == slug` (workflow.rs:302-324). A step waiver does NOT cover a short increment and a mis-scoped increment waiver (wrong `step`) does not exempt (tests at workflow.rs:658-672, 1145-1166).
- Risk-class inconsistency never suppressed. The inconsistency branch pushes its problem and `continue`s BEFORE any waiver is consulted (workflow.rs:283-289); no waiver can mask it (test at 674-688).
- Strict-check / best-effort-parse fail-safe. `check_record`'s waiver arm (metrics.rs:475-537) and `parse_waivers` (metrics.rs:760-820) enforce the same rules (valid `unit`/`reason`/`evidence_tier` enums, non-empty `step`, the `increment`-iff-increment-unit presence rule, the `evidence`-iff-record-backed presence rule). `parse_waivers` is never looser than `check_record`, so any waiver it projects would also pass the schema check. A malformed waiver is therefore BOTH reported by `validate_log` (forcing exit 1) AND dropped from the projection W3 reads, so it can never silently exempt. Confirmed empirically: a bad-`evidence_tier` waiver and a self-declared+evidence laundering attempt each produced the schema error AND the W3 pause.md catch simultaneously, exit 1.
- CLI wiring (main.rs:532-634): `validate_log` errors and `check_workflow` problems are merged into one `problems` list in a single invocation; any non-empty list forces `std::process::exit(1)`. So "reported" always means non-zero exit; a reported-but-still-projected waiver can never yield a green run.
- W5 unit-scoped escalation join. A record-backed waiver's `evidence` must join to a `type:"escalation"` record with `human_decision:"decision"` whose `task` equals the evidence AND is scoped to the waived unit (increment: `task == increment`; step: `leading_slug(task) == step`) (workflow.rs:390-414). W3 stays orthogonal (exempts on unit+identity), but a bad join is still reported by W5, so the net result of a mis-evidenced waiver is exit 1, not a pass. The reason<->tier pairing (self-declared reasons must be self-declared tier; `accepted-at-escalation` must be record-backed) is enforced at workflow.rs:416-429.
- Empty-`task` escalation. Guarded in `check_record` (metrics.rs:417-419) and `parse_escalations` (metrics.rs:856); combined with `parse_waivers` requiring non-empty `evidence`, an empty-task escalation can never satisfy the W5 join. The new test `an_escalation_with_an_empty_task_is_reported` (metrics.rs:1316-1322) is non-vacuous: without the guard the record would validate and `one_error` would panic on "expected exactly one error".

### Migration honesty and mechanical validity

15 waiver records added; 15 Roadmap rows flipped to `complete` (14 `grandfathered` -> `complete`, plus `optional-modules` `in progress` -> `complete`). Each flipped unit has exactly one covering waiver:

- 11 step-unit `predates-logging`/self-declared waivers for the early steps (core-assets, file-dropper, idempotency-safety, selection-ui, mode-enum, tag-selection, available-filter, pack-manifest, external-packs, pack-owned-principles, init-vcs). None of these 11 have any round records in the log, so W3 takes the no-records path and a step waiver is the correct instrument. Honest.
- 3 increment-unit `review-skipped`/self-declared waivers (convergence-accounting, pack-rebuild-tracking, user-prompts-dir), each with `increment` == the bare step slug. Each of these steps DOES have exactly one round record (a lone `new_valid` change round, `consecutive_clean` 0, `low_risk`, peak 0 < required 1), so they take W3's short-streak path and correctly need an increment waiver, not a step waiver. The "review-skipped" reason matches the data (a change round with no clean rounds logged). Self-declared increment waivers exempting a short streak without an escalation is the documented design (the tiers stop laundering a weak claim as strong, not to forbid self-declaration), so this is intended, not a hole.
- 1 increment-unit `accepted-at-escalation`/record-backed waiver for optional-modules-inc2cii: 5 rounds, peak streak 1 (risky needs 2), with a real `type:"escalation" human_decision:"decision"` record at the increment's `task` (line 82). The record-backed join and unit scoping are satisfied. Honest.

No Roadmap table row still carries a retired `grandfathered`/`trivial` status (the 22 lexical hits in the plan are narrative prose about the retirement, which is not parsed). `plan.rs` retires both statuses and `validate --plan` rejects them (test at plan.rs:517-544).

### Gates (all pass)

- `cargo test --all-targets`: 214 + 1 + 3 tests, 0 failed.
- `cargo clippy --all-targets -- -D warnings`: clean, no warnings.
- `cargo run -- validate --metrics ...`: 105 records, valid, exit 0.
- `cargo run -- validate --plan ... --metrics ...`: 50 steps, 44 open-questions items, valid, exit 0.
- `cargo run -- validate --workflow --plan ... --metrics ...`: workflow invariants hold, exit 0.
- Adversarial fixtures (scratchpad): pause.md catch, malformed-waiver report+drop, and self-declared+evidence laundering all correctly exit 1.

### Tests

Spot-checked for vacuity: the W5 flag tests assert specific message substrings and problem counts; the W3 exemption tests distinguish step vs increment units and mis-scoped steps; the fail-safe is exercised through `parse_waivers`/`parse_escalations` (the same projections W3/W5 read) so a malformed record genuinely never reaches the checked functions. The new empty-task test is load-bearing (see above). No vacuous test found.

I did not run `nix fmt` (to avoid reflowing unrelated tracked docs); formatting is not a correctness concern and clippy is clean. I modified no tracked files; only this review file (untracked) was written.
