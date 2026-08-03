# `workflow-enforcement-tier-inc2`: TEST-EVIDENCE lens

LENS: test evidence. The question is not whether the code is right, it is whether the suite would notice if it stopped being right. Method, in order of value: mutation testing, red-then-green verification against the pre-fix product, coverage of the claimed acceptance surface, and the goldens.

ARTIFACT: `git diff main..HEAD` in the review worktree, two commits, `effb637` (`feat: refuse and omit on a round log or ledger the plan cannot vouch for`) and `1543325` (`docs: document the refusal, the omitted parts, and the new JSON reasons`). HEAD is `1543325`.

PRE-FIX REVISION: `feea7ec`, which is `main` and the parent of `effb637`.

SPECIFICATION: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`.

BASELINE, established before any mutation: `cargo test` at HEAD is green, 413 tests across 9 binaries (378 in the bin unit tests, 13 in the new `tests/unsafe_pairings_are_refused_and_omitted.rs`, the rest pre-existing). `cargo clippy --all-targets -- -D warnings` is clean.

## Mutation table

29 mutations run, one at a time, each reverted with `git checkout -- <file>` before the next. Every run was the full suite (`cargo test` under the project's direnv environment, which is what `just test` runs). CAUGHT means at least one test failed. SURVIVED means the whole suite stayed green.

| # | Site | Mutation | Verdict | Caught by |
| --- | --- | --- | --- | --- |
| M1 | `src/main.rs:1355` | `is_outside_root` returns `false` unconditionally (the predicate neutered). | CAUGHT | 11 of the 13 new integration tests. |
| M2 | `src/main.rs:1313` | `checked_plan_root` roots on the ANCHOR (`source.or(plan)`) instead of the checked plan, ignoring `toml_primary`. This is the exact defect `Q-55-endproperty` exists to prevent. | CAUGHT | `a_divergent_source_and_plan_pairing_is_refused`, `the_resume_reasons_separate_and_cover_the_default_ledger`, `accepted_costs_three_and_four_are_pinned`. |
| M3 | `src/main.rs:1534` | `next`'s metrics reason loses the precedence rule: `log-absent` when the unpairable path does not exist. | CAUGHT | `the_machine_surface_separates_the_causes_on_both_commands`. |
| M4 | `src/main.rs:1157` | The same precedence loss on `status`. | SURVIVED | nothing (EVI-3). |
| M5 | `src/main.rs:1449-1461` | `run_resume` checks `!ledger_path.exists()` BEFORE the containment predicate, so an unsafe missing ledger is reported as absent. | SURVIVED | nothing (EVI-4). |
| M6 | `src/main.rs:981` | The refusal removed on `validate` only (`unsafe_pairing` forced false there). | CAUGHT | 6 tests, including `an_explicit_metrics_outside_the_plans_root_is_refused`. |
| M7 | `src/main.rs:1150` | The metrics omission removed on `status` only. | CAUGHT | `status_omits_only_the_unpairable_part`, `the_machine_surface_separates_the_causes_on_both_commands`, and 3 more. |
| M8 | `src/main.rs:1526` | The metrics omission removed on `next` only. | CAUGHT | `next_withholds_the_whole_loop_on_an_unpairable_log` and 4 more. |
| M9 | `src/main.rs:1534` | The trap at specification line 187: `next` treats an unsafe log as ABSENT (`LogAbsent`, empty rounds, the zero-rounds path runs). | CAUGHT | `next_withholds_the_whole_loop_on_an_unpairable_log` and 2 more. |
| M10 | `src/next.rs:677-681` | The same trap inside the pure function: `project` no longer withholds `active_loop`. | CAUGHT | `next::tests::an_unpairable_log_withholds_the_loop_instead_of_projecting_an_empty_one`, `next::tests::the_absent_causes_serialise_distinguishably`. |
| M11 | `src/next.rs:687` | The correlation rule's guard dropped (`unpairable_log` alone, without `!steps_leave_no_loop`). | CAUGHT | `next::tests::a_terminal_plan_reports_the_step_cause_not_the_log_cause`. |
| M12 | `src/next.rs:1085-1089` | `no_loop_reason`'s two variants swapped. | CAUGHT | `next::tests::a_terminal_plan_reports_the_step_cause_not_the_log_cause`, `next::tests::the_absent_causes_serialise_distinguishably`. |
| M13 | `src/next.rs:101` | `#[serde(rename_all = "kebab-case")]` removed from `MetricsAbsentReason` (wire spelling becomes `LogAbsent`). | CAUGHT | `next::tests::the_absent_causes_serialise_distinguishably`. |
| M14 | `src/next.rs:134` | The same removed from `ResumeStateAbsentReason`. | CAUGHT | `next::tests::golden_json`. |
| M15 | `src/next.rs:198` | `#[serde(skip)]` removed from `metrics_absent_note`, so the note leaks onto the wire. | CAUGHT | `next::tests::golden_json`. |
| M16 | `src/next.rs:192` | `#[serde(skip)]` ADDED to `no_active_loop_reason` (the pre-change contract restored). | CAUGHT | `next::tests::golden_json`, `next::tests::the_absent_causes_serialise_distinguishably`. |
| M17 | `src/main.rs:1290` | `canonical_project_root` made LEXICAL (absolutise, no `canonicalize`), collapsing the deliberate lexical/canonical split on the plan side. | CAUGHT | `a_symlinked_source_cannot_borrow_its_neighbours_log`, `accepted_cost_two_the_symlinked_layouts_are_pinned`. |
| M18 | `src/main.rs:1332-1338` | `resolve_for_containment` drops canonicalisation entirely (returns the absolutised path). | CAUGHT | `a_dotdot_escape_is_refused_and_one_that_stays_inside_is_not`, `accepted_cost_two_the_symlinked_layouts_are_pinned`. |
| M19 | `src/main.rs:1332` | `resolve_for_containment` starts the walk at the PARENT (`.ancestors().skip(1)`), so a symlinked log LEAF is never resolved. | SURVIVED | nothing (EVI-1). |
| M20 | `src/main.rs:1355` | `Path::starts_with` replaced by a string-prefix comparison. | CAUGHT | `accepted_cost_two_the_symlinked_layouts_are_pinned` (its `two` root and `two-metrics` sibling happen to be a common-prefix pair). |
| M21 | `src/main.rs:1432` | `resume_roots` drops the `--plan` anchor, so only the `--source` roots `status --resume`. | CAUGHT | `resume_omits_the_default_ledger_under_a_divergent_pairing`, `accepted_costs_three_and_four_are_pinned`. |
| M22 | `src/main.rs:1546-1549` | `next`'s LEDGER containment check removed. | CAUGHT | `the_resume_reasons_separate_and_cover_the_default_ledger`. |
| M23 | `src/main.rs:1451-1457` | `status --resume`'s ledger containment check removed. | CAUGHT | `status_omits_only_the_unpairable_part`, `resume_omits_the_default_ledger_under_a_divergent_pairing`, `accepted_costs_three_and_four_are_pinned`. |
| M24 | `src/main.rs:991` | The refusal message loses its THIRD remedy, "or correct the `--source` and `--plan` pair". | SURVIVED | nothing (EVI-6). |
| M25 | `src/main.rs:985` | The refusal's first slot names the `--source` rather than the plan the check reads. | CAUGHT | `a_divergent_source_and_plan_pairing_is_refused`. |
| M26 | `src/main.rs:995` | The `else` removed, so the four-arm match RUNS BESIDE the refusal and the join is still asserted. | SURVIVED | nothing (EVI-2). |
| M27 | `src/main.rs:1167` | `status` reports `log-not-this-project` for a genuinely absent log. | SURVIVED | nothing (EVI-3). |
| M28 | `src/main.rs:1546-1550` | `next` computes the unsafe-ledger REASON but passes no NOTE, so nothing is printed in place of the block. | SURVIVED | nothing (EVI-5). |
| M29 | `src/main.rs:1355` | `is_outside_root` returns `true` unconditionally (the predicate always fires), the false-positive side. | CAUGHT | 8 tests in the pre-existing inc1 file `tests/metrics_and_ledger_anchor_to_the_plan_source.rs`. |

TOTALS: 29 run, 22 caught, 7 survived (M4, M5, M19, M24, M26, M27, M28), folded into 6 findings.

WHAT THE TABLE SAYS ABOUT THE CENTRAL QUESTION. M2 is the mutation the whole `Q-55-endproperty` decision exists to prevent, and the suite catches it with three independent tests, two of which (`a_divergent_source_and_plan_pairing_is_refused`, `the_resume_reasons_separate_and_cover_the_default_ledger`) are the ones the specification names as the separating cases. M9 and M10 catch the trap at specification line 187 on both the caller and the pure function. M1, M6, M7, M8, M22, M23 confirm all four surfaces are independently guarded: removing the withhold on any one of them fails a test. The predicate's core is well evidenced; the survivors cluster in the REASON VOCABULARY and in the message text, not in the predicate's verdict, with one exception (EVI-1).

## Red-then-green verification

METHOD. `git archive main` was extracted to a scratch copy outside the worktree, the NEW integration test file `tests/unsafe_pairings_are_refused_and_omitted.rs` was copied in unchanged, and `cargo test --test unsafe_pairings_are_refused_and_omitted` was run against the pre-fix product.

RESULT: 12 of 13 RED, 1 green.

```
failures:
    a_correct_run_serialises_the_new_reasons_as_null
    a_divergent_source_and_plan_pairing_is_refused
    a_dotdot_escape_is_refused_and_one_that_stays_inside_is_not
    a_symlinked_source_cannot_borrow_its_neighbours_log
    accepted_cost_two_the_symlinked_layouts_are_pinned
    accepted_costs_three_and_four_are_pinned
    an_explicit_metrics_outside_the_plans_root_is_refused
    next_withholds_the_whole_loop_on_an_unpairable_log
    resume_omits_the_default_ledger_under_a_divergent_pairing
    status_omits_only_the_unpairable_part
    the_machine_surface_separates_the_causes_on_both_commands
    the_resume_reasons_separate_and_cover_the_default_ledger

test result: FAILED. 1 passed; 12 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

THE FOUR OWED DEMONSTRATIONS (specification line 309) ARE ALL REAL, and each fails for the right reason rather than incidentally.

- CHECK 11, the explicit-relative-`--metrics` false pass, is `an_explicit_metrics_outside_the_plans_root_is_refused`. RED, on the exit-code assertion at `tests/unsafe_pairings_are_refused_and_omitted.rs:183`.
- CHECK 13b, the divergent `--source`/`--plan` pairing, is `a_divergent_source_and_plan_pairing_is_refused`. RED, at `:237`.
- CHECK 14b, the fabricated `next` instruction, is `next_withholds_the_whole_loop_on_an_unpairable_log`. RED, and the captured pre-fix stdout in that run carries `"filled_prompt_summary": "step `borrowed-step` increment `borrowed-step` converged (streak 1/1); mark the step complete, re-render, and commit."` plus `"resume_state": "## RESUME STATE\n\nHOME resume state."`, which is the fabricated instruction AND the leaked block from the run-from directory, both reproduced by the new fixture.
- CHECK 14e, the absent reason fields, is `the_machine_surface_separates_the_causes_on_both_commands`. RED, and `the_resume_reasons_separate_and_cover_the_default_ledger` fails at `:617` with `left: "<absent>"`, which is the field not existing at all.

THE ONE GREEN TEST IS CORRECTLY GREEN. `the_refusal_is_scoped_to_the_validator` asserts only exit 0 on five invocations and that a named log is still read. That is a NO-REGRESSION test, not a red-then-green one, and passing pre-fix is what it should do; the increment's risk is that the projections start failing, and this test is the guard against it. It is not evidence for the increment and should not be counted as such.

THE FIVE UNIT TESTS IN `src/next.rs` CANNOT BE RUN AGAINST THE PRE-FIX PRODUCT, and their red is structural rather than behavioural: `MetricsAbsentReason`, `NoActiveLoopReason` and `ResumeStateAbsentReason` do not exist at `feea7ec`, and `NextInputs` has none of the four new fields, so the module does not compile there. That is not a defect, it is what a type-level contract change looks like, but a round report should not claim a behavioural red for them. Their real evidence is M10, M11, M12, M13 and M15, all of which they catch.

## Findings

### EVI-1: the symlinked log LEAF is unguarded, and removing that one clause re-opens a false pass at exit 0

SEVERITY: high.

CLAIM. `resolve_for_containment` (`src/main.rs:1326-1339`) deliberately canonicalises THE PATH ITSELF when it exists, not just its directory prefix, and the doc comment states that ("The path itself is used when it exists"). No test exercises that clause. Starting the walk one component higher leaves every existing test green while re-opening a false pass of exactly the class the increment exists to close: a log whose LEAF is a symlink out of the plan's root is read, joined, and reported as holding.

MUTATION.

```
 fn resolve_for_containment(path: &Path) -> PathBuf {
 ...
-	for ancestor in absolute.ancestors() {
+	for ancestor in absolute.ancestors().skip(1) {
```

SUITE UNDER THE MUTATION, full `cargo test`:

```
=== exit: 0 ===
      1 test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
      1 test result: ok. 378 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
      ... (all 9 binaries ok)
=== failing tests ===
=== compile errors ===
```

THE FALSE PASS IT ADMITS. Fixture: `home/docs/metrics/workflow.jsonl` holds one converged round for `borrowed-step`; `away/docs/plans/p.plan.toml` claims `borrowed-step` `complete` with no evidence of its own; `away/docs/metrics/workflow.jsonl` is a SYMLINK to `home`'s log.

```
$ agent-scaffold validate --source .../leaf/away/docs/plans/p.plan.toml --workflow
.../leaf/away/docs/metrics/workflow.jsonl: 1 records, valid
.../leaf/away/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
.../leaf/away/docs/plans/p.plan.toml vs .../leaf/away/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

The same command on the un-mutated HEAD build:

```
--workflow would join .../leaf/away/docs/plans/p.plan.toml against .../leaf/away/docs/metrics/workflow.jsonl, which is not under the plan's project root .../leaf/away; pass a `--metrics` under that root, run against the plan's own log, or correct the `--source` and `--plan` pair
exit=1
```

WHY THIS IS NOT ACCEPTED COST (ii). Cost (ii) is a FALSE POSITIVE, a legitimate layout being refused, and `accepted_cost_two_the_symlinked_layouts_are_pinned` pins both of its manifestations. This is the opposite direction, a FALSE NEGATIVE, and it is the increment's stated end property ("must never pair a plan source with a metrics log belonging to a different project and report success"). Both existing symlink tests symlink a DIRECTORY (`docs/plans` in layout 1, `docs/metrics` in layout 2), so the leaf clause is never reached by either.

`file:line`: `src/main.rs:1332`, the `for ancestor in absolute.ancestors()` line, and the clause it implements documented at `src/main.rs:1319-1321`.

WHAT WOULD CLOSE IT. One test in `tests/unsafe_pairings_are_refused_and_omitted.rs` with the log's LEAF symlinked out of the plan's root, asserting the refusal on `validate --workflow` and the omission on `status` and `next`. The file already has a `symlink` helper at `:848`.

### EVI-2: nothing pins that the refusal REPLACES the check rather than accompanying it

SEVERITY: medium.

CLAIM. The refusal's own comment (`src/main.rs:984-989`) states the requirement: "REFUSE rather than run the check: joining a log the tool cannot attribute to this plan is the defect, so asserting anything about the pairing (IN EITHER DIRECTION) is what has to stop." Deleting the `else` so the four-arm match runs beside the refusal leaves the suite green. The tests assert the exit code and the absence of `workflow invariants hold` on stdout, and both survive, because `run_validate` suppresses stdout summaries once `problems` is non-empty. Nobody asserts that the refusal is the ONLY problem reported.

MUTATION.

```
-		} else {
+		}
+		{
 			match (toml_primary, &plan_contents, &metrics_contents) {
```

SUITE UNDER THE MUTATION: all 9 binaries ok, 13 of 13 integration tests pass, exit 0.

OBSERVABLE DIFFERENCE. Fixture: `home`'s log holds a round for `other-step` only; `away` claims `borrowed-step` `complete`.

```
$ agent-scaffold validate --source .../away/docs/plans/p.plan.toml --metrics docs/metrics/workflow.jsonl --workflow
--workflow would join .../away/docs/plans/p.plan.toml against docs/metrics/workflow.jsonl, which is not under the plan's project root .../away; pass a `--metrics` under that root, run against the plan's own log, or correct the `--source` and `--plan` pair
.../away/docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped
exit=1
```

HEAD prints only the first line. The mutated build says in one breath that it cannot vouch for the pairing and then reports a verdict on it, which is the negative-direction assertion the comment forbids. The exit code is unchanged, which is why no test notices.

`file:line`: `src/main.rs:995` (the `} else {`), requirement stated at `src/main.rs:984-989`.

WHAT WOULD CLOSE IT. Extend `an_explicit_metrics_outside_the_plans_root_is_refused` with a fixture whose foreign log does NOT satisfy the borrowed slug, and assert the refusal is the only line on stderr (for example `assert!(!stderr.contains("has no round records"))`, or an exact-count assertion on the problem lines).

### EVI-3: `status`'s reason vocabulary is only half pinned, on both its value set and its precedence rule

SEVERITY: medium.

CLAIM. `status --json` gained `metrics_absent_reason` with two possible values and a precedence rule. Only ONE value in ONE direction is asserted anywhere. Two independent mutations survive.

MUTATION A, the precedence rule (`src/main.rs:1154-1157`):

```
 	let (metrics, metrics_absent_reason) = if unpairable_log.is_some() {
-		(None, Some(next::MetricsAbsentReason::LogNotThisProject))
+		let reason = if metrics_path.exists() {
+			next::MetricsAbsentReason::LogNotThisProject
+		} else {
+			next::MetricsAbsentReason::LogAbsent
+		};
+		(None, Some(reason))
```

SUITE: green, exit 0, 13 of 13 integration tests pass.

```
$ agent-scaffold status --json --source .../away/docs/plans/p.plan.toml --metrics docs/metrics/nope.jsonl
  "metrics": null,
  "metrics_absent_reason": "log-absent"        <- mutated build
  "metrics_absent_reason": "log-not-this-project"  <- HEAD build, same command
```

MUTATION B, the `log-absent` value itself (`src/main.rs:1166-1168`):

```
 	} else {
-		(None, Some(next::MetricsAbsentReason::LogAbsent))
+		(None, Some(next::MetricsAbsentReason::LogNotThisProject))
 	};
```

SUITE: green, exit 0.

```
$ agent-scaffold status --json --source docs/plans/p.plan.toml --metrics docs/metrics/nothing.jsonl
  "metrics_absent_reason": "log-not-this-project"   <- mutated build
  "metrics_absent_reason": "log-absent"             <- HEAD build, same command
```

WHY IT MATTERS HERE SPECIFICALLY. Acceptance check 14f's whole content is that the vocabulary SEPARATES the causes, and the specification at line 208 flags `status --json` as the unguarded half by construction (no golden, no serialisation test). The suite does assert `status`'s `log-not-this-project` (in `the_machine_surface_separates_the_causes_on_both_commands:556` and `the_resume_reasons_separate_and_cover_the_default_ledger:689`) and its `null` (in `a_correct_run_serialises_the_new_reasons_as_null:841`), but it never asserts `status`'s `log-absent`, so a consumer joining on the token has evidence for one of the two values it can receive. Under mutation B a `status --json` consumer cannot distinguish case (a) from case (b) at all, which is the exact "the defect has moved rather than closed" condition check 14f names.

`file:line`: `src/main.rs:1154-1168`; the tests that would carry it are `tests/unsafe_pairings_are_refused_and_omitted.rs:549-558` and `:820-844`.

WHAT WOULD CLOSE IT. Two added assertions on existing runs: `status --json` on a project whose OWN log is missing must show `"metrics_absent_reason": "log-absent"`, and `status --json` with an out-of-root `--metrics` naming a nonexistent file must show `log-not-this-project`. The `next` half of both already exists at `:561-592`.

### EVI-4: `status --resume`'s precedence rule is unguarded

SEVERITY: medium.

CLAIM. `run_resume` puts the containment predicate ahead of the existence check with an explicit comment saying why ("so an unsafe ledger is never reported as a missing one (the precedence rule: unsafe is not absent)"). Swapping the two blocks leaves the suite green.

MUTATION (`src/main.rs:1449-1461`): the `if !ledger_path.exists()` block moved ABOVE the `resume_roots(...).find(...)` block.

SUITE: green, exit 0, 13 of 13 integration tests pass.

```
$ agent-scaffold status --resume --source .../away/docs/plans/p.plan.toml --ledger-fragment docs/plans/nope.ledger.md
no ledger at docs/plans/nope.ledger.md; nothing to resume        <- mutated build, exit 0
the ledger docs/plans/nope.ledger.md is not under the plan's project root .../away; nothing to resume   <- HEAD, exit 0
```

The existing coverage uses `docs/plans/p.ledger.md`, which EXISTS in the run-from project (`status_omits_only_the_unpairable_part:455-472`), so the ordering never matters in any test. `next`'s equivalent case IS pinned (`the_resume_reasons_separate_and_cover_the_default_ledger:635-646`, the "Outside the root AND missing" run); `status --resume` has no counterpart.

`file:line`: `src/main.rs:1449-1461`.

WHAT WOULD CLOSE IT. One extra invocation inside `status_omits_only_the_unpairable_part` with a `--ledger-fragment` that is both outside the root and nonexistent, asserting the note rather than `no ledger at`.

### EVI-5: `next`'s human note for an unpairable LEDGER is not pinned end to end

SEVERITY: medium.

CLAIM. Specification line 183 requires that on `next` the `RESUME STATE` echo is omitted "with the same note naming the rejected ledger path in its place that `status --resume` prints". The RENDERER half is pinned by a unit test (`next::tests::an_unpairable_ledger_prints_its_note_in_place_of_the_block`, `src/next.rs:1996-2032`), which builds the projection by hand and supplies the note itself. The CALLER half, `run_next` assembling that note, is pinned by nothing: dropping it leaves the suite green and `next` prints nothing at all where the explanation should be.

MUTATION (`src/main.rs:1546-1550`): `resume_state_absent_note` forced to `None` while the reason is still computed and reported.

SUITE: green, exit 0, 13 of 13 integration tests pass.

OBSERVABLE DIFFERENCE, tail of `next --source <away plan> --ledger-fragment docs/plans/p.ledger.md`:

```
  summary: first review round on step `borrowed-step`: independent reviewer, cite file and line.
                                     <- mutated build: nothing follows, exit 0

  summary: first review round on step `borrowed-step`: independent reviewer, cite file and line.

the ledger docs/plans/p.ledger.md is not under the plan's project root .../away; nothing to resume
                                     <- HEAD build, exit 0
```

The integration tests only ever assert the ABSENCE of the foreign block (`:632` `!stdout.contains("HOME resume state.")`, `:667` `!stdout.contains("ALPHA resume state.")`) and the JSON reason token. The "say why in its place" half of `Q-55-refusalscope` is unasserted on `next`'s human surface, which is the surface an agent's operator reads. Note that the metrics equivalent IS pinned end to end (`next_withholds_the_whole_loop_on_an_unpairable_log:423-428`), so this is an asymmetry between the two artifacts rather than a uniform gap.

`file:line`: `src/main.rs:1546-1550` for the caller, `src/next.rs:1175-1178` for the arm it feeds.

WHAT WOULD CLOSE IT. One assertion in `the_resume_reasons_separate_and_cover_the_default_ledger` that the human `next` stdout contains `the ledger ... is not under the plan's project root`, on both the explicit-fragment run and the default-ledger divergent-pairing run.

### EVI-6: the refusal message's third remedy is unguarded

SEVERITY: low.

CLAIM. The specification at line 157 adds a third remedy member specifically for the divergent-pairing cause, "or correct the `--source` and `--plan` pair", on the stated ground that "neither of A's two remedies names that cause". Removing it leaves the suite green, including the divergent-pairing test that is the only place the third remedy is the RELEVANT one.

MUTATION (`src/main.rs:991`): the message shortened to "pass a `--metrics` under that root, or run against the plan's own log".

SUITE: green, exit 0, 13 of 13 integration tests pass.

The only remedy assertion in the suite is `an_explicit_metrics_outside_the_plans_root_is_refused:198-201`, which checks the FIRST member on the wrong-`--metrics` case. `a_divergent_source_and_plan_pairing_is_refused` asserts the three PATHS (`:239-241`) and no remedy at all.

`file:line`: `src/main.rs:991`.

WHAT WOULD CLOSE IT. One `assert!(stderr.contains("correct the `--source` and `--plan` pair"))` in `a_divergent_source_and_plan_pairing_is_refused`.

## Coverage assessment, per acceptance check owed by inc2

The verdict below is about EVIDENCE, not about behaviour: PINNED means a test fails if the behaviour changes, which was established by mutation where a mutation was available.

- CHECK 11 (explicit relative `--metrics` refused): PINNED. `an_explicit_metrics_outside_the_plans_root_is_refused:165`. Red pre-fix. M6 and M1 both fail it. Its remedy assertion covers only the first of three members (EVI-6).
- CHECK 12 (symlinked source refused): PINNED for the DIRECTORY symlink. `a_symlinked_source_cannot_borrow_its_neighbours_log:281`, and M17 fails it. The LEAF symlink on the log is NOT covered (EVI-1).
- CHECK 13 (`..` escape refused, in-root `..` allowed): PINNED on both halves. `a_dotdot_escape_is_refused_and_one_that_stays_inside_is_not:311`, and the second half asserts the check RUNS by matching W3's own message (`:339`), which is stronger than asserting the absence of a refusal. M18 fails it.
- CHECK 13b (divergent `--source`/`--plan`, typo'd `--source`, no-regression pair): PINNED, and this is the strongest test in the file. `a_divergent_source_and_plan_pairing_is_refused:215` covers all three runs, and M2 (the anchor-rooted predicate) fails it. This is the check the specification says separates the two rootings, and it does.
- CHECK 14 (refusal scoped to the validator): PINNED as a no-regression test. `the_refusal_is_scoped_to_the_validator:355`; M29 shows the over-firing direction is caught, though by inc1's file rather than this one.
- CHECK 14b (`next` withholds the whole loop): PINNED. `next_withholds_the_whole_loop_on_an_unpairable_log:400` asserts all seven block fields absent, no record count, the note present, and exit 0 explicitly. M8 and M9 fail it.
- CHECK 14c (`status` and `status --resume`): PINNED for the three runs the check names, across `status_omits_only_the_unpairable_part:437` and `resume_omits_the_default_ledger_under_a_divergent_pairing:485`. M7, M21 and M23 fail them. The precedence sub-case on `status --resume` is not covered (EVI-4).
- CHECK 14d (unsafe is not absent): PINNED TWICE, at the CLI (`:416-421`, asserting the absence of BOTH `converged` and `awaiting-first-review`) and in the pure function (`src/next.rs:1863`, whose fixture deliberately supplies CONVERGED rounds so both fabrication directions are distinguishable). M9 and M10 fail them. This is the trap the specification calls out and the suite meets it properly.
- CHECK 14e (machine surface on both commands): PINNED on both. `the_machine_surface_separates_the_causes_on_both_commands:526`. Red pre-fix, and the `status --json` half is explicitly asserted at `:556`, which is the half the specification flags as otherwise unguarded.
- CHECK 14f (the vocabulary separates three causes, plus precedence): PINNED ON `next`, HALF-PINNED ON `status`. All three cases plus the precedence run exist for `next` (`:532-592` and `src/next.rs:1913-1953`); for `status` only case (b) is asserted, and neither the `log-absent` value nor the precedence rule is (EVI-3).
- CHECK 14g (resume reasons separate; the default-ledger half of `Q-55-endproperty`): PINNED. `the_resume_reasons_separate_and_cover_the_default_ledger:600` covers all four runs plus the metrics half of the same pairing on `next --json` and `status --json`. M2, M21, M22 all fail it. The human note in place of the block is not asserted (EVI-5).
- CHECK 14h (the correct case unchanged, new fields serialise as `null`): PINNED. `a_correct_run_serialises_the_new_reasons_as_null:820` on both commands, plus `next::tests::golden_json`.
- CHECK 19 (accepted cost (ii), both layouts, both surfaces): PINNED. `accepted_cost_two_the_symlinked_layouts_are_pinned:707` asserts the loud refusal AND the quiet omission for the plan-side and the log-side layouts. M17, M18 and M20 all fail it.
- CHECK 19b (accepted costs (iii) and (iv)): PINNED. `accepted_costs_three_and_four_are_pinned:764` covers cost (iv) in both `primary` spellings and cost (iii) on `validate`, `status` and `next`. M2, M6, M21 and M23 fail it.

### The goldens

`GOLDEN_JSON` (`src/next.rs:2069`) gained exactly three lines and one trailing comma, in struct-field order and nowhere else:

```
+  "metrics_absent_reason": null,      (immediately after the "metrics" object)
-  "resume_state": null
+  "resume_state": null,
+  "resume_state_absent_reason": "ledger-absent",
+  "no_active_loop_reason": null
```

Nothing was reordered, renamed, removed or given a `skip_serializing_if`. `null` is explicit for the two absent-reason fields that are `None`, matching the file's stated convention. That is exactly what check 14h demands.

`GOLDEN_HUMAN` IS UNCHANGED, which is correct and worth stating because the review brief expected it to move: the `no_active_loop_reason` retype maps each old variant back to the exact string it printed, and the golden fixture has an active loop so it never renders the no-loop line at all. The claim that the retype is behaviour-preserving on the human surface is therefore carried by `golden_human_text` for the loop case and by `next::tests::the_absent_causes_serialise_distinguishably` plus `a_terminal_plan_reports_the_step_cause_not_the_log_cause` for the two step-derived strings.

Both golden tests are strict `assert_eq!` byte compares (`src/next.rs:2132-2140`); neither was relaxed.

### Assertions removed or loosened

NONE FOUND. `git diff main..HEAD -- src/` removes exactly one signature line, `fn no_loop_reason(steps: &[StepInfo]) -> String`, replaced by the enum-returning version; no `assert`, no `#[test]`, no golden line is deleted anywhere in the diff. No test was renamed in place of being fixed. No `assert_eq!` was weakened to a `contains`. The new integration tests use `assert_eq!` on exit codes throughout and reserve `contains` for message fragments, which is the right split for text output.

### Two unreachable arms, recorded as observations rather than findings

Neither is a coverage gap because neither can be reached through `project`, which is the only constructor of a `NextProjection` outside the test module.

- `src/next.rs:1196`, `no_loop_text`'s `None` arm returning the historical "no in-progress or ready step". `project` sets a reason whenever there is no loop, so the arm is defensive only, and its own comment says so.
- `src/next.rs:1189-1192`, the fallback to `NoActiveLoopReason::MetricsNotThisProject.human_text()`. `run_next` sets the note and the reason together, so the fallback cannot fire from the CLI.
