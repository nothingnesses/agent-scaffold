# `validation-constraints-inc1`, round 3: reviewer, suite adequacy by mutation

Reviewer worktree: `.claude/worktrees/rev-inc1r3-mutate`, branch `review/inc1r3-mutate`, at `651ff63`.
Artifact: `git diff main..HEAD`, three commits (`0110828` the implementation, `86e00ed` the round 1 fix pass, `651ff63` the round 2 fix pass), touching `.agents/AGENTS.reference.md`, `AGENTS.md`, `CHANGELOG.md`, `pack/instrument.md`, `src/plan/source.rs`, `src/workflow.rs`.
Specification: `docs/plans/agent-scaffold.steps/validation-constraints.md`, increment 1 and the Acceptance section.
Settled: `vc-inc1-r1-triage.md` (five valid, one duplicate, one dismissed) and `vc-inc1-r2-triage.md` (nine valid, two duplicates, none dismissed). Nothing below re-raises a settled finding, and nothing below claims a settled verdict was wrong.

LENS: whether the suite holds the code down, not whether the code is right. The pattern under test is the one two fix passes have now produced, a suite patched mutation by mutation, which is strong exactly where it was attacked. So the battery aims at what neither previous round ran.

## Verdict

TWO FINDINGS, ceiling `medium`. NO `high` AND NO `critical`, stated explicitly rather than implied.

| id | claim | severity |
| --- | --- | --- |
| `W3B-1` | The round 1 fix pass retired acceptance item 4's red half against one of the two options the human DECLINED. A build that reports the unobserved case ONLY WHEN THE LOG IS NON-EMPTY now passes all 386 tests, clippy and the live plan, while returning `workflow invariants hold` at exit 0 on a tree the shipped binary refuses. Round 1's own battery caught this mutation; the tree no longer does. | `medium` |
| `W3B-2` | No fixture makes ONE waiver produce the new ownership problem alongside any other W5 problem, so two mutations that make the new arm suppress, or be suppressed by, a sibling check survive the whole suite. | `low` |

NEITHER IS A DEFECT IN WHAT THE TOOL DOES. Both are test gaps over correct code, the same shape rounds 1 and 2 recorded. On the 32-tree verdict matrix and the 8 targeted fixtures below, the shipped binary matched the expectation I computed by hand from the documented accessors on every tree, 32 of 32 and 8 of 8. I found no input on which it answers wrongly.

## Trees, binaries and fixtures

All under `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/r3mut`, a subdirectory of my own naming. `src/` was mutated ONLY in scratch copies; my worktree carries this file and nothing else (`git status --porcelain` shows only this path). `TMPDIR` pointed at `<scratch>/r3mut/tmp`, outside every git repository, per the Acceptance preamble. No fixture was created with mode 000 or 600, so none needed a chmod back.

ONE `CARGO_TARGET_DIR` PER BINARY, 43 binaries built, 43 distinct md5 sums (`md5sum target-*/debug/agent-scaffold | awk '{print $1}' | sort | uniq -d` returns nothing). Full list in `<scratch>/r3mut/md5.txt`. The two reference builds:

```
d3b4368d6e8d3d3e64984bcb9b8f1bf5  target-head/debug/agent-scaffold    (git archive HEAD, 651ff63)
12d5cd2b75f640b01209f01f4ed39116  target-prefix/debug/agent-scaffold  (git archive main)
```

Mutations were applied by `<scratch>/r3mut/apply.py`, which matches an anchor on its STRIPPED line content, asserts the anchor hits exactly one line, and re-indents with the matched indentation, so a stale anchor aborts loudly rather than silently rebuilding the pristine tree and hard tabs cannot be mangled. Driver: `bash <scratch>/r3mut/run-mutant.sh <id>`.

Control, HEAD unmutated:

```
cargo test --bins                          -> 386 passed; 0 failed
cargo test (9 binaries)                    -> 430 passed; 0 failed
cargo clippy --all-targets -- -D warnings  -> exit 0
git diff main..HEAD | LC_ALL=C grep -cP '[^\t\x20-\x7e]'  -> 0
```

386, not round 2's 385, because `651ff63` added `w5_derives_an_owner_from_an_increment_only_records_task`.

## `W3B-1`: the declined "report only when the log is non-empty" form now passes the whole suite

VALID, `medium`.

THE DECISION. Receipt `Q-70-emptycase` decided the unobserved case as REPORT IT, over staying silent and over reporting only when the log is non-empty. Acceptance item 4 (`validation-constraints.md:122`) requires the decision be pinned by a test and closes "A test that does not distinguish the two forms has not pinned the decision".

THE MUTATION (`RH4rep`), `src/workflow.rs:625`, one added clause:

```
-				if !rounds.iter().any(|round| waiver_covers_round(waiver, round)) {
+				if !rounds.is_empty() && !rounds.iter().any(|round| waiver_covers_round(waiver, round)) {
```

That build reports the unobserved case when the log carries some other increment's records and stays silent when the log carries no `type:"round"` record at all, which is precisely the declined option.

THE SUITE, THE LINT AND THE LIVE PLAN ALL STAY GREEN:

```
cd <scratch>/r3mut/m-RH4rep && CARGO_TARGET_DIR=<scratch>/r3mut/target-RH4rep cargo test --bins
-> test result: ok. 386 passed; 0 failed; 0 ignored
```

FALSE GREEN AT THE COMMAND LEVEL, my fixtures `<scratch>/r3mut/fx/*.L0` (Roadmap `alpha` and `beta` both `in progress`, so only W5 can speak; log carries one `decision` escalation scoped to the waived increment and one increment-unit `record-backed` waiver, and NO `type:"round"` record):

```
head     alpha-inc1.alpha.L0  exit=1  ... increment waiver names increment `alpha-inc1`, which no `type:"round"` record resolves to ..., so the round log joins it to no step
RH4rep   alpha-inc1.alpha.L0  exit=0  ... workflow invariants hold
head     alpha-fold.alpha.L0  exit=1  (same refusal for `alpha-fold`)
RH4rep   alpha-fold.alpha.L0  exit=0  workflow invariants hold
head     alpha-inc1.beta.L0   exit=1
RH4rep   alpha-inc1.beta.L0   exit=0  workflow invariants hold
```

ROUND 1 CAUGHT THIS EXACT MUTATION AND THE TREE NO LONGER DOES, which is what makes it new evidence rather than a re-raise. Round 1's battery recorded `m6`, "report only when the log is non-empty (the option the human DECLINED)", as CAUGHT by `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` (`vc-inc1-reviewer-behaviour.md:128`). At the implementation commit that test passed an EMPTY rounds slice, which is the only fixture that can distinguish the two forms:

```
git show 0110828:src/workflow.rs | grep -n "w5_flags_an_increment_waiver_whose_increment_has_no_round_records" -A 26
->  1570:  let problems = w5_problems(&waivers(&waiver), &steps, &[], &escalations);

git show 86e00ed:src/workflow.rs | grep -n "w5_flags_an_increment_waiver_whose_increment_has_no_round_records" -A 30
->  1648:  let other = rounds(&owning_round_line("alpha", "alpha-other"));
->  1649:  let problems = w5_problems(&waivers(&waiver), &steps, &other, &escalations);
```

Round 1's triage remedy asked for exactly that substitution, to close the increment-axis gap (`vc-inc1-r1-triage.md:109`), and neither it nor the fix pass kept an empty-log case beside it. The substitution was correct for the axis it aimed at, and it was the only fixture holding the empty-log axis, so closing one gap opened another. Nine `w5_problems` call sites still pass `&[]`, but every one of them carries a STEP-unit waiver, which never reaches this arm; that is proved by the mutant itself, since an increment-unit waiver with an empty rounds slice anywhere in the suite would have reddened it.

`RH4sil`, the OTHER declined option (stay silent on the unobserved case), is still CAUGHT, 1 red (`w5_flags_an_increment_waiver_whose_increment_has_no_round_records`). So item 4 still distinguishes the decided form from one declined alternative and no longer distinguishes it from the other.

CODE DEFECT OR TEST GAP: TEST GAP, STATED PLAINLY. `w5_problems` as shipped implements the decided reporting form on every tree I ran, including all four `.L0` trees. No line of `src/` needs to change.

WHY `medium` AND NOT `low`. Severity is absolute impact if left unfixed. Left unfixed the shipped tool is right, and the exposure is that a later edit implementing a form the human explicitly declined ships green with a false `workflow invariants hold` at exit 0 over a waiver nothing evidences. That is structurally identical to round 1's `W1A-1`, which the triage rated `medium` on the same reasoning, and it additionally leaves an acceptance item's own stated requirement unmet.

WHY NOT `high`. The affected population is a repository whose log carries no `type:"round"` record at all, which is narrower than `W1A-1`'s, and no verdict on the live plan or on any other tree moves.

REMEDY, SCOPED TO THE CLASS (the empty-log axis of the unobserved case is pinned on neither consumer):

- `src/workflow.rs:1645-1650`, `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`: ADD a third assertion, or a sibling test, passing an EMPTY rounds slice and asserting the same message. Do NOT revert the non-empty fixture, which is round 1's remedy and catches the increment axis; the two fixtures pin different axes and the test needs both. One added assertion reddens `RH4rep`.
- `src/workflow.rs:625`, the `!rounds.iter().any(...)` guard: NO EDIT. Correct as shipped.
- FOR THE POST-MERGE PLANNER, not a finding against this artifact: acceptance item 4's wording already demands a test that distinguishes the two forms, so the list itself needs no change; what it lacks is any statement that a fixture substitution can retire a red half. The sidecar is known stale and I do not raise it.

## `W3B-2`: the new ownership arm is never exercised beside another W5 check on the same waiver

VALID, `low`.

W5 runs four per-waiver checks: the Roadmap-step check, the NEW round-log ownership check, the record-backed evidence join, and the reason/tier pairing. The suite exercises the evidence join and the pairing TOGETHER on one waiver (`w5_flags_each_inconsistent_reason_tier_pairing`, whose own comment says "hence two problems"), and that combination is pinned: mutation `N9`, which suppresses the pairing check whenever an earlier problem exists, is caught by 2 tests. NO fixture pairs the new ownership problem with any sibling, and both mutations that exploit that survive.

MUTATION `N8`, the evidence join suppressed by any earlier problem:

```
-			if waiver.evidence_tier == EvidenceTier::RecordBacked {
+			if waiver.evidence_tier == EvidenceTier::RecordBacked && problems.is_empty() {
```

386 passed, 0 failed. My fixture `<scratch>/r3mut/fx2/n8` (Roadmap `alpha` and `beta` both `in progress`; one record joins `alpha-inc1` to `beta`; one increment waiver names `alpha` and cites evidence `no-such-pointer`):

```
head  exit=1  round log line 2: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
              round log line 2: `record-backed` waiver cites evidence `no-such-pointer` but no `type:"escalation"` record with `human_decision` `decision` is scoped to this waiver's unit
N8    exit=1  round log line 2: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
```

MUTATION `N10`, the ownership arm suppressed by a failed Roadmap-step check:

```
-		if waiver.unit == WaiverUnit::Increment {
+		if waiver.unit == WaiverUnit::Increment && slugs.contains(waiver.step.as_str()) {
```

386 passed, 0 failed. My fixture `<scratch>/r3mut/fx3/n10` (Roadmap `beta` only; one record joins `alpha-inc1` to `beta`; an increment waiver naming ghost step `ghost`):

```
head  exit=1  round log line 3: `type:"waiver"` names step `ghost`, which is not a Roadmap step
              round log line 3: increment waiver names step `ghost` but the round log joins increment `alpha-inc1` to step `beta`
N10   exit=1  round log line 3: `type:"waiver"` names step `ghost`, which is not a Roadmap step
```

BOTH ARE VERDICT-NEUTRAL, measured rather than argued: the tree is refused at exit 1 under `head` and under both mutants, because the surviving problem still fires. What the mutants lose is the SECOND reason, so a user fixes one fault, re-runs, and meets the next one instead of both at once.

IN SCOPE THOUGH THE MUTATED LINES ARE NOT `+` LINES. `if waiver.evidence_tier == EvidenceTier::RecordBacked {` and `if waiver.unit == WaiverUnit::Increment {` are both unchanged context (`git diff main..HEAD` adds neither). But the BEHAVIOUR the mutations remove is the new arm's own contribution to the report, and the missing fixture is one that exercises the new arm, so the round 2 out-of-scope precedent's third condition (the subject is independent) fails. The retired lexical rule occupied the same position, so this gap predates the diff in form; what the diff changes is which arm sits there and what it says.

WHY `low`. No verdict moves anywhere, and neither mutant makes a shipped message FALSE, which is where round 2 set its `low` bar for `W2A-2`.

REMEDY, SCOPED TO THE CLASS (nothing pins the new arm's independence from its three siblings):

- `src/workflow.rs`, beside `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment`: ONE fixture in which a single increment-unit waiver names a step the Roadmap does not carry AND whose increment the log joins elsewhere AND whose evidence joins to nothing, asserting all three problems. One fixture reddens `N8` and `N10` together. `<scratch>/r3mut/fx3/n10` and `fx2/n8` are worked instances.
- `src/workflow.rs:603-704`, `w5_problems`: NO EDIT. Correct as shipped.

## The mutation battery

41 mutations. NONE is reused from round 1 or round 2 except the seven marked "re-run", which are there to settle that the fixes landed and that two acceptance red halves are still red. Every one names its target, whether the suite caught it, which tests went red, and for survivors what the build gets wrong or why it is equivalent.

### Group A: the two join accessors and the lexical shim, which no round has mutated

| id | mutation | caught | tests that went red |
| --- | --- | --- | --- |
| `A1` | `round_step_slug` ignores the structured `step` id and always uses `leading_slug(task)` | YES, 6 | `check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step`, `w3_a_round_carrying_a_structured_step_joins_without_the_lexical_strip`, `w3_a_step_only_round_joins_on_its_structured_step_and_falls_back_on_the_increment_axis`, `w5_accepts_an_increment_waiver_whose_id_does_not_strip_to_its_step`, `w5_marks_an_owner_derived_from_a_pre_migration_records_task`, `w5_names_every_step_the_log_joins_a_waived_increment_to` |
| `A2` | `round_step_slug`'s fallback stops stripping (`unwrap_or(&round.task)`) | YES, 16 | spanning W3 and W5, incl. `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment`, `check_workflow_catches_the_pause_pattern_and_passes_round_log_core`, `w3_a_pre_migration_round_still_joins_its_step_via_leading_slug`, `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it` |
| `A3` | `round_increment_id` ignores the structured `increment` id and always uses `task` | YES, 4 | `w5_derives_an_owner_from_an_increment_only_records_task`, plus `next::tests::awaiting_fixes_row`, `next::tests::golden_human_text`, `next::tests::golden_json` |
| `A4` | `round_increment_id`'s fallback strips (`leading_slug(task)` instead of `task`) | YES, 6 | `an_increment_waiver_does_not_exempt_a_sibling_increment_of_the_same_step`, `a_short_streak_increment_with_a_covering_increment_waiver_passes`, `check_workflow_catches_the_pause_pattern_and_passes_round_log_core`, `check_workflow_passes_the_optional_modules_migration_shape`, `check_workflow_toml_passes_the_optional_modules_accepted_at_escalation_waiver`, `per_increment_grouping_passes_a_step_that_converged_across_two_risk_classes` |
| `A5` | `leading_slug` drops the all-alphanumeric guard on the suffix | YES, 1 | `leading_slug_strips_alphanumeric_increment_suffixes` |
| `A6` | `leading_slug` never strips | YES, 20 | spanning W3, W5 and the escalation join |
| `A7` | `leading_slug` takes the FIRST `-inc` marker (`find`) instead of the last (`rfind`) | NO, 386 passed | none. SURVIVOR, see below |
| `A8` | the predicate's step axis accepts only a record that DECLARES its step (`Some(waiver.step) == round.step`) | YES, 5 | `a_bare_slug_increment_waiver_exempts_a_short_streak`, `a_short_streak_increment_with_a_covering_increment_waiver_passes`, `check_workflow_passes_the_optional_modules_migration_shape`, `check_workflow_toml_passes_the_optional_modules_accepted_at_escalation_waiver`, `check_workflow_toml_passes_the_waiver_model_self_referential_waiver` |

### Group B: the escalation join, which shares the same accessor design and which no round has mutated

| id | mutation | caught | tests that went red |
| --- | --- | --- | --- |
| `B1` | `escalation_step_slug` ignores the structured `step` id | YES, 1 | `w5_a_record_backed_waiver_joins_via_the_escalations_structured_step` |
| `B2` | `escalation_increment_id` ignores the structured `increment` id | NO, 386 passed | none. SURVIVOR, see below |
| `B3` | the evidence join's two `WaiverUnit` arms swapped | YES, 9 | incl. `w5_a_record_backed_waiver_joins_via_the_escalations_structured_step`, `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment`, `w5_passes_a_step_unit_record_backed_waiver_joined_by_leading_slug` |
| `B4` | the `human_decision != Decision` clause dropped | YES, 1 | `w5_flags_a_record_backed_waiver_whose_escalation_resumed_not_decided` |
| `B5` | the `escalation.task != evidence` clause dropped | NO, 386 passed | none. SURVIVOR, see below |
| `B6` | `escalation_step_slug`'s fallback stops stripping | YES, 2 | `w5_passes_a_step_unit_record_backed_waiver_joined_by_leading_slug`, `w5_without_the_structured_step_the_escalation_over_strips_and_is_missed` |

### Group C: the W3 grouping and filters that feed the shared predicate

| id | mutation | caught | tests that went red |
| --- | --- | --- | --- |
| `C1` | W3's step filter uses `leading_slug(task)` instead of `round_step_slug` | YES, 2 | `w3_a_round_carrying_a_structured_step_joins_without_the_lexical_strip`, `w3_a_step_only_round_joins_on_its_structured_step_and_falls_back_on_the_increment_axis` |
| `C2` | W3's increment grouping keys on the raw `task` instead of `round_increment_id` | NO, 386 passed | none. SURVIVOR, see below |
| `C3` | W3 asks the predicate of EVERY round instead of the increment's own records | YES, 1 | `an_increment_waiver_does_not_exempt_a_sibling_increment_of_the_same_step` |
| `C4` | W3's step-level exemption drops its `unit == Step` check | NO, 386 passed | none. SURVIVOR, see below |
| `C5` | `round_log_consistency_problems` groups on the raw `task` instead of `round_increment_id` | NO, 386 passed | none. SURVIVOR, see below |

### Group D: the predicate's two callers, mutated at the CALL SITE rather than inside it

| id | mutation | caught | tests that went red |
| --- | --- | --- | --- |
| `D1` | W5's ownership arm runs for every waiver unit (`if true`) | NO, 386 passed | none, and it is an EQUIVALENT MUTANT, see below |
| `D2` | W5 drops the STEP axis at the call site (accepts if any record resolves to the increment) | YES, 6 | `check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step`, `w5_derives_an_owner_from_an_increment_only_records_task`, `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it`, `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment`, `w5_marks_an_owner_derived_from_a_pre_migration_records_task`, `w5_names_every_step_the_log_joins_a_waived_increment_to` |
| `D2b` | W3 drops the STEP axis at the call site | YES, 1 | `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment` |
| `D3` | W5 drops the INCREMENT axis at the call site | YES, 1 | `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |

### Group E: the singular and plural formatting arms

| id | mutation | caught | tests that went red |
| --- | --- | --- | --- |
| `E1` | the singular MARKED arm names its own slug inside the mark, like the plural one | YES, 2 | `w5_derives_an_owner_from_an_increment_only_records_task`, `w5_marks_an_owner_derived_from_a_pre_migration_records_task` |
| `E2` | the plural MARKED arm marks the whole owner list instead of the derived subset | YES, 1 | `w5_marks_an_owner_derived_from_a_pre_migration_records_task` |
| `E3` | the derived filter inverted (declared owners marked, derived ones not) | YES, 4 | `w5_derives_an_owner_from_an_increment_only_records_task`, `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it`, `w5_marks_an_owner_derived_from_a_pre_migration_records_task`, `w5_names_every_step_the_log_joins_a_waived_increment_to` |
| `E4` | the singular/plural boundary widened (`owners.len() <= 1`) | NO, 386 passed | none, and it is an EQUIVALENT MUTANT, see below |

### Group N: the new arm's interaction with the rest of W5, and determinism

| id | mutation | caught | tests that went red |
| --- | --- | --- | --- |
| `N7b` | the owners map becomes a `HashMap`, so owner order is per-process random | YES, 1 to 2, FLAKY | `w5_marks_an_owner_derived_from_a_pre_migration_records_task` and `w5_names_every_step_the_log_joins_a_waived_increment_to`; over 20 runs of those two tests alone, 5 runs had 2 red, 13 had 1 red, 2 were fully green |
| `N8` | the evidence join skipped when an earlier problem exists | NO, 386 passed | none. See `W3B-2` |
| `N9` | the pairing check skipped when an earlier problem exists | YES, 2 | `check_workflow_toml_w5_rejects_a_mis_tiered_waiver`, `w5_flags_each_inconsistent_reason_tier_pairing` |
| `N10` | the ownership arm skipped when the waiver's step is not a Roadmap step | NO, 386 passed | none. See `W3B-2` |
| `N11` | the empty-owners and non-empty-owners branches swapped | YES, 7 | incl. `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`, `w5_names_every_step_the_log_joins_a_waived_increment_to` |

### Re-runs: the three targeted tests' own mutations and the acceptance red halves

| id | mutation | caught | tests that went red |
| --- | --- | --- | --- |
| `T1` | the INCREMENT axis dropped from `waiver_covers_round` (round 1 `W1A-1`, round 2 `m10`) | YES, 2 | `an_increment_waiver_does_not_exempt_a_sibling_increment_of_the_same_step`, `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |
| `T2` | the owners scan keys on the raw `task` (round 2 `W2A-2` mutation A, `m9`) | YES, 1 | `w5_derives_an_owner_from_an_increment_only_records_task` |
| `T3` | the mark reads `round.increment` (round 2 `W2A-2` mutation B, `m16`) | YES, 1 | `w5_derives_an_owner_from_an_increment_only_records_task` |
| `T4` | the owner merge becomes first-write-wins (round 2 `W2A-2` mutation C, `m20`) | YES, 1 | `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it` |
| `RH4b` | acceptance item 4b's own mutation: W5 passes the waiver's own `step`, so the step comparison degenerates to comparing a value with itself, W3 untouched | YES, 6 | `check_workflow_toml_w5_refuses_...`, `w5_derives_an_owner_...`, `w5_does_not_mark_...`, `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment`, `w5_marks_an_owner_derived_...`, `w5_names_every_step_...` |
| `RH4b2` | the STEP axis dropped from the predicate itself (round 2 `m11`) | YES, 7 | the six above plus `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment` |
| `RH7` | acceptance item 7's demonstration: the predicate's step comparison inverted | YES, 19, spanning W3 and W5 | W3 side incl. `a_bare_slug_increment_waiver_exempts_a_short_streak`, `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment`, `a_short_streak_increment_with_a_covering_increment_waiver_passes`; W5 side incl. `w5_accepts_an_increment_waiver_whose_id_does_not_strip_to_its_step`, `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |
| `RH4rep` | acceptance item 4's first declined option: report only when the log is non-empty (round 1 `m6`) | NO, 386 passed | none. See `W3B-1` |
| `RH4sil` | acceptance item 4's second declined option: stay silent on the unobserved case (round 1 `m6b`) | YES, 1 | `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |

## The survivors, one by one

Eight mutations survived. Two are equivalent mutants, two are the `W3B-2` finding, and four are pre-existing gaps in lines this diff does not touch. Each is settled here rather than left in the table.

### Equivalent, not gaps

- `D1`. Dropping W5's `unit == Increment` guard changes nothing because both waiver projections enforce the presence rule: `metrics::parse_waivers` DROPS a step-unit waiver carrying an `increment` (`src/metrics.rs:885-890`, `(WaiverUnit::Step, Some(_)) => continue`), and `waivers_from_toml` does the same (`src/workflow.rs:246`). So a step-unit `Waiver` reaching `w5_problems` always has `increment: None`, the inner `if let Some(increment)` never binds, and the arm is unreachable for it. This is the same ground round 1 gave for its `m4`.
- `E4`. `owners.len() <= 1` and `owners.len() == 1` agree because the arm is reached only from the `else` of `if owners.is_empty()`, so `owners` is never empty there.

### Pre-existing, and out of scope by the round 2 precedent

For each of the four below I checked all four conditions of the out-of-scope precedent recorded at `vc-inc1-r2-triage.md:90-98`, and all four hold. Condition 2 is the load-bearing one and I measured it: `git diff main..HEAD -- src/workflow.rs` contains NO `+` line and NO `-` line for any of these anchors, so no commit in range modifies them. Condition 3 holds because each subject is an identity or a join that predates the ownership rule and is not what inc1's review question asks about. Condition 4 holds because each remedy is a new test that shares nothing with the ownership predicate's tests. THEY ARE REPORTED HERE, NOT RAISED AS FINDINGS, and the strongest is called out for routing rather than dropped.

- `B5`, THE STRONGEST OF THE FOUR AND WORTH ROUTING. Dropping the `escalation.task != evidence` clause from W5's record-backed join makes the `evidence` pointer decorative: any `decision` escalation scoped to the waived unit backs any pointer. Fixture `<scratch>/r3mut/fx2/b5` (Roadmap `alpha` `in progress`; a `decision` escalation with `task` `alpha`; a step-unit record-backed waiver for `alpha` citing evidence `no-such-pointer`): `head` exits 1 with "`record-backed` waiver cites evidence `no-such-pointer` but no `type:\"escalation\"` record ... is scoped to this waiver's unit", `B5` exits 0 with `workflow invariants hold`. A FALSE GREEN IN THE ENFORCEMENT TIER. No test pins the clause: the three tests that look like they would (`w5_flags_a_record_backed_waiver_citing_an_unrelated_escalation`, `w5_flags_a_record_backed_waiver_citing_an_escalation_for_another_step`, `w5_flags_a_record_backed_waiver_with_no_matching_escalation`) all fail their join on the UNIT SCOPE or on an empty escalation slice, never on the pointer equality.
- `B2`. `escalation_increment_id` ignoring the structured `increment` id survives because no test gives an escalation a structured `increment` that differs from its `task`; the step-axis counterpart IS pinned, by `w5_a_record_backed_waiver_joins_via_the_escalations_structured_step`, which caught `B1` and `B6`. Fixture `<scratch>/r3mut/fx2/b2`: `head` exits 0, `B2` exits 1 with "cites evidence `e1` but no `type:\"escalation\"` record ... is scoped to this waiver's unit". A false red, on the migration shape Inc 2 exists to serve.
- `C2` and `C5`. Both grouping sites key on `round_increment_id`, and no test gives one structured `increment` two records with DIFFERENT `task` values. Fixture `<scratch>/r3mut/fx2/c2` (Roadmap `gamma` `complete`; two records `{"task":"r1"}` and `{"task":"r2"}` both carrying `"increment":"gamma-incA"`, `"step":"gamma"`, `risky`, streaks 1 then 2): `head` exits 0; `C2` exits 1 with "Roadmap step `gamma` increment `r1` reached a consecutive-clean streak of 1 but its `risky` risk class needs 2"; `C5` exits 1 with "round log line 2: increment `r2` records consecutive_clean 2 but its outcome sequence implies 1". Two false reds on a valid tree, from two different checks.
- `A7`. `leading_slug` taking the first `-inc` marker instead of the last differs only for a `task` carrying two markers, because the all-alphanumeric guard then rejects the longer suffix and the value is returned unstripped. Fixture `<scratch>/r3mut/fx2/a7` (Roadmap `alpha-incA` `complete`; one pre-migration record with `"task":"alpha-incA-incB"`): `head` exits 0; `A7` exits 1 with "Roadmap step `alpha-incA` is `complete` but has no round records and no covering waiver". `leading_slug_strips_alphanumeric_increment_suffixes` does not cover a double-marker task.

### The `W3B-2` pair

`N8` and `N10` are the finding above.

## The three targeted tests, verified

Each was added by a fix pass to close a named mutation. I re-applied each named mutation and ran the named test ALONE, so nothing else can account for the red, and read the panic message to check it fails for the reason it claims.

- `an_increment_waiver_does_not_exempt_a_sibling_increment_of_the_same_step` (round 1, W3's side of the increment axis). Against `T1`: `panicked at src/workflow.rs:1032:9: assertion left == right failed: [] left: 0 right: 1`. The problem list is EMPTY, so the sibling waiver did exempt the short increment, which is exactly the axis the test names. Passes on HEAD.
- `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` (round 1, W5's side of the same axis). Against `T1`: `panicked at src/workflow.rs:1636:9: assertion left == right failed: [] left: 0 right: 1`, that is the FIRST assertion, on the non-empty log that lacks the waived increment. Its second half (the same waiver accepted once its own records are present) still passes under `T1`, correctly, so the red comes from the half that pins the axis. Passes on HEAD. ITS EMPTY-LOG HALF IS THE ONE `W3B-1` IS ABOUT.
- `w5_derives_an_owner_from_an_increment_only_records_task` (round 2, mutations A and B together). Against `T2` it fails with the EMPTY-OWNERS message, "increment waiver names increment `alpha-fold`, which no `type:\"round\"` record resolves to ...", so it genuinely pins that the scan reaches the record by the structured `increment` id and not by `task`. Against `T3` it fails with "the round log joins increment `alpha-fold` to step `zzz-task`", the mark ABSENT, so it genuinely pins that the mark reads the absent `step` and not the present `increment`. Two distinct failure modes from one test, which is what its comment claims.
- `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it`, the both-orders loop (round 2, mutation C). Against `T4` the panic prints the log that failed, and it is the DERIVES-FIRST ordering, ending "... joins increment `alpha-inc1` to step `alpha` (derived from a record's `task`)". So the red comes from the ordering the loop was added for, not from the pre-existing one. The declared-first ordering still passes under `T4`, which is the point.

NONE of the four is passing for an unrelated reason, and none is vacuous.

## Acceptance items re-run, with their red halves

`TMPDIR` outside every repository throughout.

| item | green half | red half | status |
| --- | --- | --- | --- |
| 1 | `cargo build` clean; `cargo test` 430 passed, 0 failed across 9 binaries; `cargo clippy --all-targets -- -D warnings` exit 0 | n/a | MET |
| 1 (migration obligation) | `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment` keeps its subject: its fixture carries `owning_round_line("beta", "beta-incB")` and it still asserts a mis-scoping | `RH4b` reddens it | MET |
| 2 | `validate --source docs/plans/agent-scaffold.plan.toml --workflow` on the unmodified live plan and log: `321 records valid, 96 steps, 70 questions, workflow invariants hold`, exit 0 | none by design, and I confirmed it: the PREFIX binary returns the same on the same tree | MET, and it demonstrates nothing on its own, as the item itself says |
| 3 | the plan-side unblocking is not in this diff (the two `[[step.increment]]` declarations, the two waivers and the status flip are assigned to the orchestrator and the planner), so the equivalent is matrix tree `alpha-fold.alpha.L1` | prefix exits 1 with "increment waiver names step `alpha` but increment `alpha-fold` belongs to step `alpha-fold`"; head exits 0 with `workflow invariants hold` | MET in the form this tree can carry |
| 4 | the REPORTING form is taken and asserted by `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` | `RH4sil` (declined option 2) RED, 1 test; `RH4rep` (declined option 1) GREEN, 386 passed | HALF-MET. See `W3B-1` |
| 4b | the refusal fires on both substrates (`w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment` and `check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step`) | `RH4b` RED, 6 tests | MET |
| 5 | on `alpha-fold.alpha.L0` head says "increment waiver names increment `alpha-fold`, which no `type:\"round\"` record resolves to ..., so the round log joins it to no step" and names NO step | prefix on the same tree says "increment waiver names step `alpha` but increment `alpha-fold` belongs to step `alpha-fold`", a step that is in neither the plan nor the log | MET |
| 6 | Markdown plus JSONL acceptance on `alpha-fold.alpha.L1`: head exit 0, prefix exit 1 | carried by 4b, which is red on the same substrate | MET |
| 7 | `RH7`, one edit inside the shared predicate, reddens 19 tests spanning W3 (3 tests) and W5 (16), so the predicate is shared and not copied | the mutation IS the red half | MET |
| 7b | at HEAD, `grep -c -F "the increment's leading slug equals the step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md` returns 0, 0, 0 and the replacement wording returns 1, 1, 1 | at `main` the same two commands return 1, 1, 1 and 0, 0, 0 | MET |
| 8 | `render --check --strict docs/plans/agent-scaffold.plan.toml` reports "up to date", exit 0; `validate --source ... --workflow` exit 0 | n/a | MET |

## Pre-fix versus current verdict surface

32 trees (`<scratch>/r3mut/fx`), each its own project root with `docs/plans/t.md` and `docs/metrics/workflow.jsonl`. Two waived increment ids (`alpha-inc1`, which strips to `alpha`; `alpha-fold`, which does not strip) x two waiver steps (`alpha`, `beta`) x eight log shapes: L0 no `type:"round"` record at all; L1 structured record joining the increment to `alpha`; L2 the same joining to `beta`; L3 pre-migration record whose `task` is the increment id; L4 increment-only record whose `task` is `zzz-task`; L5 step-only record declaring `alpha`; L6 a record for a DIFFERENT increment only; L7 two structured records joining the increment to `alpha` and to `beta`.

Both Roadmap steps are `in progress` in every tree, so W3 is skipped and W5's ownership rule is the only check that can speak. Every waiver's evidence joins to a scoped `decision` escalation, so the evidence arm is satisfied everywhere and a non-zero exit is always the ownership refusal.

THE EXPECTATION IS COMPUTED IN `<scratch>/r3mut/mkfx.py`, from the documented accessors, and is NOT read off either binary: the new build must accept iff some record `r` has `(r.increment else r.task) == waiver.increment` and `(r.step else leading_slug(r.task)) == waiver.step`; the old build must accept iff `leading_slug(waiver.increment) == waiver.step`.

```
head verdict  != independently computed expectation:   0 of 32
prefix verdict != independently computed expectation:  0 of 32
head ACCEPTS where prefix REFUSED:                     6 of 32
head REFUSES where prefix ACCEPTED:                    4 of 32
```

THE SIX NEW ACCEPTANCES, each checked against the documented unblocking:

- `alpha-inc1.beta.L2` and `alpha-inc1.beta.L7`: a record declares `beta` for `alpha-inc1` and the waiver names `beta`. The id strips to `alpha`, so the retired rule refused it; the log says `beta`.
- `alpha-fold.alpha.L1`, `alpha-fold.alpha.L5` and `alpha-fold.alpha.L7`: `alpha-fold` never strips, so the retired rule could never admit any waiver for it; a record joins it to `alpha` and the waiver names `alpha`. THIS IS THE `workflow-enforcement-tier-fold` SHAPE, the unblocking direction (iii) exists for.
- `alpha-fold.beta.L2`: the same, joined to `beta`.

THERE IS NO TREE IN WHICH THE NEW BINARY ACCEPTS AND NO RECORD JOINS THE WAIVED INCREMENT TO THE WAIVER'S STEP. Every new acceptance is explained by the documented unblocking, so I raise none of them.

THE FOUR NEW REFUSALS are the documented narrowing, all on `alpha-inc1.alpha`, where the id strips to the step and the retired rule therefore accepted whatever the log said: L0 (no round records at all) and L6 (records, none for this increment) are `Q-70-emptycase`'s reporting form; L2 (records join it to `beta`) is the observed contradiction the change exists to close; L4 (an increment-only record whose step derives to `zzz-task`) is the same contradiction reached through the documented per-axis fallback, and its refusal marks the owner derived.

## Examined and NOT raised

- The rounds slice W5 reads is unfiltered by project. That is inc6's subject and the step already records it as inc6's fourth limitation.
- Owner ordering and de-duplication are pinned: `N7b` reddens the two ordering assertions, and round 2's `m7` and `m8` did too.
- `A2`, `A6` and `RH7` each redden 16 to 20 tests, which is a healthy blast radius for the accessors the whole tier now rests on, and is evidence that the accessor tests round 2 did not run are in fact strong.
- The step's sidecar and the `Q-70` plan entry are known to be left stale, with a planner updating them after merge, so I do not raise the absence of an increment-axis or empty-log item from the acceptance list. `W3B-1`'s remedy records what that planner needs.
- The plan-side unblocking (two `[[step.increment]]` declarations, two owed waivers, the status flip) is absent from `git diff main..HEAD` by design.
- Line length and prose wrapping, pre-existing import-ordering drift at `src/workflow.rs:35,44,51`, pre-existing false doc claims, and anything belonging to increments 2 to 6.
- `commit 651ff63` is prefixed `docs:` while changing 150 lines of `src/workflow.rs` including one new test and one rewritten assertion loop. NOT RAISED: a commit-message prefix is not in the artifact's behaviour or its claims, and the repository's own convention file scopes prefixes to the message rather than to a diff-content rule.

## Round outcome

`new_valid`. Two valid findings, ceiling `medium`, carried by `W3B-1`. No `high` and no `critical`, found or raised. The artifact is `risky` and needs two consecutive clean rounds, so this round does not advance the streak.

IF THE ORCHESTRATOR WANTS THE ONE-LINE VERSION: the code is right and the battery could not break it, but the suite lost a red half between round 1 and now, and the loss was caused by round 1's own remedy. That is the cost of patching a suite mutation by mutation, measured on this artifact rather than argued.
