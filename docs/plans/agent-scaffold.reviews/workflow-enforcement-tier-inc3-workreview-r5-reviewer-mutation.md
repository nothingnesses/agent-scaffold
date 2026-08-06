# `workflow-enforcement-tier-inc3` work review, ROUND 5, REVIEWER: MUTATION AND COVERAGE

Reviewed in worktree `.claude/worktrees/rev-inc3-r5-mutation` on branch `review/inc3-r5-mutation` at `3f19bd1`, the tip of the branch under review.

This round asks the inverse of the previous four: not "is the code right?" but "if the code were wrong, would the suite say so?". Every claim below is a measured build, not a reading.

## Method

TOOLCHAIN, confirmed before any build-dependent claim, with no `2>/dev/null` on the export:

```
$ cd <worktree> && direnv allow && eval "$(direnv export bash)" && which cargo
/nix/store/76jaab43a2l7n7fiifxjngp68kk167vm-rust-mixed/bin/cargo
cargo 1.98.0-nightly (a335d47ff 2026-06-26)
```

BASELINE, measured twice, once before the first mutation and once after the last: `cargo test` exit 0, **422 passed, 0 failed** across nine binaries. `TMPDIR` pointed at `<scratch>/r5mut/tmpdir`, outside any git repository (`git rev-parse --show-toplevel` there reports `not a repository`, checked). `<scratch>` abbreviates the session scratchpad directory.

EVERY MUTATION RUN USED `cargo test --no-fail-fast`, so the catching-test column lists every test that fails under a mutation rather than only those in the first failing binary. The first `G1` run was made without it and reported 4 catchers; the recorded 12 is the complete set from the re-run.

EACH MUTATION WAS REVERTED WITH `git checkout -- .` FOLLOWED BY A PROVEN-CLEAN CHECK before the next one was applied: `git status --porcelain` empty AND `git diff HEAD` empty, both printed by the harness after every single run. That is a stronger revert guarantee than re-running the suite (a green suite does not prove the tree is byte-identical to `3f19bd1`; an empty `git diff HEAD` does), and the closing full run re-establishes green as well.

The mutations were applied to `src/main.rs`, the four rendered/shipped prose copies, and the test files, in that order. No `nix fmt`, no `just scaffold-self`.

---

# Part 1: THE MUTATION TABLE

30 mutations. 19 caught, 11 not caught.

## The gate (`src/main.rs:846`)

| id | mutation | caught | catching test(s) |
| --- | --- | --- | --- |
| `G1` | `matches!(metrics_probe, Ok(true))` -> `!matches!(metrics_probe, Ok(true))` | CAUGHT, 12 tests | `a_bare_filename_from_inside_docs_plans_stays_a_miss_and_now_fails_loudly`, `workflow_with_no_metrics_log_hard_errors_instead_of_skipping`, `a_round_log_that_cannot_be_checked_is_not_reported_as_missing`, `plain_validate_and_a_sourceless_run_keep_their_behaviour`, `the_correct_case_prints_the_same_relative_paths_it_always_did`, `validate_workflow_reads_the_plans_own_log_not_the_working_directorys`, `a_divergent_source_and_plan_pairing_is_refused`, `a_dotdot_escape_is_refused_and_one_that_stays_inside_is_not`, `an_explicit_metrics_outside_the_plans_root_is_refused`, `the_refusal_is_scoped_to_the_validator`, `toml_primary_skips_the_markdown_plan_validator_but_markdown_mode_still_fails`, `workflow_on_a_toml_source_runs_without_a_markdown_plan` |
| `G2` | gate -> `metrics_probe.is_ok()` (the `Err` case starts reading a log it cannot stat) | CAUGHT, 3 | `a_bare_filename_from_inside_docs_plans_stays_a_miss_and_now_fails_loudly`, `plain_validate_and_a_sourceless_run_keep_their_behaviour`, `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` |
| `G3` | gate -> `true` | CAUGHT, 4 | the `G2` three plus `a_round_log_that_cannot_be_checked_is_not_reported_as_missing` |
| `G4` | gate -> `false` | CAUGHT, 10 | `plain_validate_and_a_sourceless_run_keep_their_behaviour`, `a_round_log_that_cannot_be_checked_is_not_reported_as_missing`, `the_correct_case_prints_the_same_relative_paths_it_always_did`, `validate_workflow_reads_the_plans_own_log_not_the_working_directorys`, `workflow_on_a_toml_source_runs_without_a_markdown_plan`, and five containment tests |

All four gate mutations are caught, and the two that only SHIFT the boundary (`G2`, `G3`) are caught as sharply as the two that destroy it. The gate is well pinned.

## The probe (`src/main.rs:845`)

| id | mutation | caught | catching test(s) |
| --- | --- | --- | --- |
| `P1` | `try_exists()` -> `exists()`, spelled `let metrics_probe: io::Result<bool> = Ok(metrics_path.exists());` so the gate is untouched and only the `Err` arm becomes unreachable. THIS IS THE CHANGE `Q-55-existsgate` DECLINED, applied in reverse | CAUGHT, exactly 1 | `a_round_log_that_cannot_be_checked_is_not_reported_as_missing` |
| `P2` | probe evaluated twice (`let _first = metrics_path.try_exists(); metrics_path.try_exists()`) | NOT CAUGHT | none |

`P1` is the single most informative result in the table and it is discussed in `R5M-2`. `P2` is inert absent a concurrent writer, so its non-catch is expected and is not a gap; it is recorded because the brief asked for it.

## The arm (`src/main.rs:1067-1076`)

| id | mutation | caught | catching test(s) |
| --- | --- | --- | --- |
| `A1` | `Ok` and `Err` branches swapped | CAUGHT, 3 | `a_bare_filename_from_inside_docs_plans_stays_a_miss_and_now_fails_loudly`, `a_round_log_that_cannot_be_checked_is_not_reported_as_missing`, `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` |
| `A2` | both branches emit the `Ok` message (the pre-`Q-55-emptyroot` falsehood restored) | CAUGHT, 1 | `a_round_log_that_cannot_be_checked_is_not_reported_as_missing` |
| `A2b` | both branches emit the `Err` message | CAUGHT, 2 | `a_bare_filename_from_inside_docs_plans_stays_a_miss_and_now_fails_loudly`, `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` |
| `A3` | `problems.push` deleted, so the arm computes the message and drops it: a SILENT skip at exit 0 | CAUGHT, 3 | as `A1` |
| `A4` | `problems.push` -> `summaries.push`, so the run REPORTS SUCCESS with the text on stdout | CAUGHT, 3 | as `A1` |
| `A5` | `run_validate`'s failure path `std::process::exit(1)` -> `exit(0)` | CAUGHT, 13 | the whole validate integration surface, including `accepted_costs_three_and_four_are_pinned`, `accepted_cost_two_the_symlinked_layouts_are_pinned`, `workflow_with_no_plan_source_hard_errors_instead_of_skipping`, `a_symlinked_log_leaf_outside_the_root_is_refused_and_omitted`, `a_symlinked_source_cannot_borrow_its_neighbours_log` |

Every behavioural mutation of the arm is caught, including both false-green shapes (`A3` silent skip, `A4` reported success). The increment's central claim, that this arm cannot reach exit 0, is genuinely pinned.

## The messages

| id | mutation | caught | catching test(s) |
| --- | --- | --- | --- |
| `M1` | `Ok` message: resolved path replaced by the literal `"docs/metrics/workflow.jsonl"` | CAUGHT, 1 | `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` (run (c) asserts `no round log at absent.jsonl`) |
| `M1b` | `Err` message: resolved path replaced by the literal `"SOME/WRONG/LITERAL/PATH"` | **NOT CAUGHT** | none |
| `M2` | `Err` message: `; pass a --metrics naming this project's log` deleted (the round 4 known observation) | **NOT CAUGHT** | none |
| `M3` | `Ok` message: BOTH remedy clauses deleted (`; pass a --metrics ..., or record the project's review rounds there`) | **NOT CAUGHT** | none |
| `M5` | `Ok` message: `could not run` -> `was not executed` | CAUGHT, 2 | `a_bare_filename_from_inside_docs_plans_stays_a_miss_and_now_fails_loudly`, `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` |
| `M6` | `Err` message: `could not be checked` -> `was unreadable` | CAUGHT, 1 | `a_round_log_that_cannot_be_checked_is_not_reported_as_missing` |
| `M8` | `Err` message GAINS `, or record the project's review rounds there`, the exact falsehood `Q-55-emptyroot` decided against | CAUGHT, 1 | `a_round_log_that_cannot_be_checked_is_not_reported_as_missing` |
| `M9` | `Err` message: the `({error})` errno dropped entirely | **NOT CAUGHT** | none |
| `M10` | the sentence this increment added to the `--workflow` clap help (`src/main.rs:438`) deleted | **NOT CAUGHT** | none |
| `M10b` | CONTROL: an UNRELATED clap help string (`--plan`, untouched by this increment) replaced wholesale with nonsense | **NOT CAUGHT** | none |

The pattern is exact and worth stating plainly. The suite pins WHICH of the two sentences fires (`A1`, `A2`, `A2b`, `M5`, `M6`, `M8` all caught) and, on the `Ok` branch only, THAT THE PATH IS THE RESOLVED ONE (`M1`). It pins nothing else about either message. `M10b` is the control that keeps `M10` from being read as this increment's defect: no clap help string anywhere in the binary is pinned by anything, so `M10` is a project-wide property and not a gap this increment opened.

## The shipped prose

| id | mutation | caught | catching test(s) |
| --- | --- | --- | --- |
| `D1` | the `no round log yet` qualifier reverted in root `AGENTS.md` ALONE | CAUGHT, 1 | `agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render` |
| `D2` | the same reverted in `pack/AGENTS.md` (the TEMPLATE) ALONE | CAUGHT, 1 | same |
| `D3` | the same reverted in `.agents/AGENTS.reference.md` ALONE | CAUGHT, 1 | same |
| `D4` | the same reverted CONSISTENTLY in all three at once | **NOT CAUGHT** | none |
| `D5` | `README.md` and `CHANGELOG.md` reverted wholesale to their `main` text, removing every sentence this increment added to them | **NOT CAUGHT** | none |

`D1` to `D4` together characterise the drift guard exactly: it pins CONSISTENCY between the template and its two deployed copies, and pins NOTHING about content. Any claim can be edited out of the scaffolded guidance and the suite stays at 422/0 provided all three copies move together. This is discussed under check 20 rather than raised, for the reason given there.

## The tests themselves

| id | mutation | caught | catching test(s) |
| --- | --- | --- | --- |
| `T1` | `let opaque = ... .is_err() && false;` in `a_round_log_that_cannot_be_checked_is_not_reported_as_missing`, simulating a machine where the mode-600 fixture does not hide the entry, COMBINED WITH source mutation `P1` | **NOT CAUGHT** | none |
| `T2` | run (a) of `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` weakened to trivially true (`assert_ne!(code, Some(999))` and `stderr.contains("")`) | **NOT CAUGHT** | none |
| `T3` | VACUITY PROBE: control run (d)'s expected exit code corrupted `Some(0)` -> `Some(7)` | CAUGHT, 1 | its own test, `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` |

`T2`'s non-catch is the EXPECTED answer and is not a gap: these are independent integration tests, and an assertion only its own test depends on is fine, which is what the brief allows for. `T3` is the useful direction: corrupting an assertion's EXPECTED VALUE and confirming the owning test goes red proves the assertion actually executes. Control run (d), the half of the tier policy the test file itself calls "the easiest to break by accident", is non-vacuous. `T1` is the one test-side result that is a genuine gap, and it is `R5M-2`.

---

# Part 2: THE ACCEPTANCE-CHECK COVERAGE TABLE

Checks 15, 17, 18 and 20 of `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`. Check 16's vacuous pass is already recorded and scheduled and is excluded by name; check 19 is outside this brief.

| Check | What it asserts | Pinned by a test? | Evidence |
| --- | --- | --- | --- |
| 15 | `validate --source <plan> --workflow` on a project with no round log exits NON-ZERO, and the problem names the RESOLVED log path and says the check COULD NOT RUN | **PINNED**, on the behaviour, by `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` runs (a), (b), (c) | Every one of `G2`, `G3`, `A1`, `A2b`, `A3`, `A4`, `A5`, `M1`, `M5` is caught by that test. All three claims are separately pinned: the exit code (`A3`, `A4`), the resolved path (`M1`), and the `could not run` clause (`M5`). The MANUAL residue is only the check's fixture: no test runs `agent-scaffold init` and then `validate --workflow` on its output, so "a scaffolded project has no log at the resolved path" holds by construction of a hand-built fixture rather than of a scaffolded one. The behaviour under test is identical either way, so I do not count this against the check |
| 17 | THE CONTROL: an empty `docs/metrics/workflow.jsonl` beside a `complete` step still produces the W3 message at exit 1, proving the change removed a wrong answer rather than the check | **PINNED**, twice end to end plus unit coverage | `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:168` and `tests/unsafe_pairings_are_refused_and_omitted.rs:372` both assert `` `borrowed-step` is `complete` but has no round records `` through the built binary, and `src/workflow.rs` carries four more W3 assertions at the unit level (`:1543`, `:1584`, `:1664`, `:1850`). This check needs no human |
| 18 | ACCEPTED COST (i) pinned as EXPECTED behaviour: the bare-filename run from inside `docs/plans` is a HARD FAILURE naming the path it looked for, and the fix is never to canonicalise the default | **PINNED**, by the test named for it | `a_bare_filename_from_inside_docs_plans_stays_a_miss_and_now_fails_loudly` catches `G1`, `G2`, `A1`, `A2b`, `A3`, `A4`, `A5` and `M5`. The check's own words ask that "a test pinning this belongs in the suite so a later improvement that turns the default canonical fails loudly", and that test exists and bites |
| 20 | The `SE-3` documentation half: the scaffolded `AGENTS.md` carries the instrumentation qualifier, a reader of that sentence can predict check 15's exit code, AND the deployed copies are regenerated | **SPLIT.** The REGENERATION half is pinned; the CONTENT half is MANUAL ONLY | `D1`, `D2`, `D3` are each caught by `agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render`, so no copy can drift from the template. `D4` deletes the qualifier from the template and both deployed copies together and the suite stays at 422/0, so nothing pins that the sentence says anything in particular. The check's own text matches this split: it words the content half as a `grep` a human runs and cites `cargo test` only for the regeneration half. So the check is not overclaiming, and I raise no finding on it |

---

# Part 3: FINDINGS

Two, both `low`. Both are coverage gaps rather than defects: the shipped behaviour at `3f19bd1` is correct in every case I measured, and neither finding claims otherwise.

## `R5M-1`: the `Err` branch's message carries no pinned information at all

**Severity: `low`.**

**The uncaught mutations.** Three, all in the same six lines, all leaving the suite at 422 passed / 0 failed:

- `M1b`: the resolved path in the `Err` message replaced by the literal `"SOME/WRONG/LITERAL/PATH"`.
- `M9`: the `({error})` errno dropped from the `Err` message.
- `M2`: the `; pass a --metrics naming this project's log` remedy deleted from the `Err` message (the observation round 4's triager recorded and did not raise).

`M3` is the same family on the other branch: deleting BOTH remedy clauses from the `Ok` message is also uncaught.

**Why the gap is material.** The three assertions guarding the `Err` case are `!stderr.contains("no round log at")`, `stderr.contains("could not be checked")`, and `!stderr.contains("record the project's review rounds")`. Two are negative and the positive one is a four-word phrase. So the suite pins that the `Err` branch says the right KIND of thing and nothing about WHAT it says. Everything the message exists to carry can be corrupted silently: the path the probe failed on, the errno that says why, and the one action available to the operator.

The asymmetry inside the increment is what makes this more than a wish. The `Ok` branch's path IS pinned, three times over, and `M1` proves it bites. The `Err` branch's path is pinned zero times, and it is the branch where naming the path matters MORE, not less: in the `Ok` case the operator already knows no log is there, whereas in the `Err` case a real log may be sitting behind the error and the printed path is the only thing that tells them which file the tool could not reach. A wrong path there is not an unhelpful message, it is a misleading one, and it would ship green.

There is a second reason specific to this artifact. Four rounds have now argued about the wording of these two messages (`R3B-1` on an exhaustiveness word, `R4A-1` on the distinguishability claim, `Q-55-emptyroot` on the remedy clause). `M8` shows the suite does defend the one conclusion that was written as an assertion. Nothing defends the rest, so the next edit to this message can undo any of those conclusions without a single test going red.

**The smallest remedy.** One assertion, added to the existing `if opaque` block in `a_round_log_that_cannot_be_checked_is_not_reported_as_missing`, next to the three already there. Something of the form:

```
assert!(
    stderr.contains(&format!("round log at {} could not be checked (", metrics_display)),
    "the Err message must name the resolved path and the errno; stderr:\n{stderr}"
);
```

That single line kills `M1b` and `M9` together (it pins the path and the presence of a parenthesised errno), costs one assertion, and needs no new fixture. `M2` and `M3`, the remedy clauses, are a judgement call I do not press: round 4 established that both `Ok` clauses are live one per population, so an assertion on them would be defensible, but the message text is longer than the behaviour it guards and I would not object to leaving those two unpinned deliberately.

## `R5M-2`: the whole `Err` branch is guarded by one conditional block that degenerates silently

**Severity: `low`.** I say below why I did not rate it higher and what would move it.

**The uncaught mutation.** `T1`: set `opaque` to false in `a_round_log_that_cannot_be_checked_is_not_reported_as_missing` and apply source mutation `P1` (`try_exists()` -> `exists()`) at the same time. Suite: 422 passed, 0 failed.

**Why the gap is material.** `P1` is caught by EXACTLY ONE test, and every assertion in that test that does the catching sits inside `if opaque`. So the entire coverage of this increment's `Err` branch, which is to say the entire coverage of the design `Q-55-existsgate` chose over the `exists()` alternative it declined, rests on a runtime condition. When that condition is false the block is skipped, the test prints `ok`, the suite prints 422 passed, and the shipped design is indistinguishable from the declined one.

The condition is false whenever mode 600 on a directory does not stop the process reading through it, which is exactly the case for uid 0 and for any process holding `CAP_DAC_READ_SEARCH`. The test's own comment says so ("as root it does not, and then the log is simply THERE"). This repository has no CI configuration at all (`.github/workflows` does not exist; the justfile is the gate), so the suite runs wherever a developer or an agent runs it, and agent harnesses commonly run as root in a container. On the machine I measured on the block DOES execute, which is how `P1`, `A2`, `M6` and `M8` were caught; the objection is that nothing tells a reader when it stops executing.

This is the project's own named failure mode, from the brief: a check that passes before the change pins nothing. Here it is narrower and stranger, a check that pins something on some machines and silently pins nothing on others, while reporting the same `ok` either way.

**What I weighed against it, honestly.** The `if opaque` pattern is NOT this increment's invention. It is a pre-existing house convention, introduced for the anchor surface at `tests/unsafe_pairings_are_refused_and_omitted.rs:958-967` (`fs::set_permissions(&plans, 0o000)`, then `let opaque = ...is_err()`, then `if opaque`). This increment followed the established pattern rather than inventing a weaker one, and the test measures the condition rather than assuming it, which is better than the alternative it could have chosen. What is new here, and what I think justifies raising it anyway, is that in this increment the conditional block is the ONLY guard on a decided design choice, which is a sharper position than the same pattern occupies at the anchor surface.

**The smallest remedy, and I measured it rather than proposing it blind.** Add a SECOND fixture to the same test, with NO `if` guard, that produces an `Err` from the probe structurally rather than by permission: make `docs/metrics` a REGULAR FILE, so `stat("docs/metrics/workflow.jsonl")` returns `ENOTDIR` for every uid including root, since a non-directory component is not a permission check and `CAP_DAC_OVERRIDE` does not apply to it. Built and run against both designs:

```
FIXTURE: <root>/docs/plans/p.plan.toml present, <root>/docs/metrics is a regular FILE
$ agent-scaffold validate --workflow --source docs/plans/p.plan.toml

  SHIPPED (3f19bd1):
  --workflow requested but the round log at docs/metrics/workflow.jsonl could not be
  checked (Not a directory (os error 20)): the workflow check could not run, ...     exit=1

  DECLINED DESIGN (mutation P1, exists()):
  --workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow
  check could not run, ... or record the project's review rounds there               exit=1
```

One unconditional `assert!(stderr.contains("could not be checked"))` on that fixture separates the two on every machine, and it composes with `R5M-1`'s assertion (the same fixture pins the path and the errno at the same time). The existing mode-600 fixture should STAY: it is the only one where a REAL LOG sits behind the error, which is the case `Q-55-emptyroot`'s falsehood argument is actually about, and `ENOTDIR` does not reproduce that. The proposal is to add an unconditional floor beneath it, not to replace it.

If a fix is not wanted at round 5, the alternative that costs almost nothing is to make the degeneration loud rather than silent: an `else` branch on the existing `if opaque` that prints a warning saying the discriminator did not engage on this machine. That leaves the coverage exactly as it is but stops the suite from reporting an untested branch as tested.

## What I did NOT raise, and why

- `M10` (the `--workflow` clap help sentence is unpinned): control `M10b` shows no clap help string in the binary is pinned by anything. Project-wide property, not this increment's gap.
- `D4` (the `AGENTS.md` qualifier's CONTENT is unpinned): check 20 words the content half as a human `grep` and claims `cargo test` only for regeneration, which is exactly what `D1` to `D3` confirm it delivers. The check is not overclaiming.
- `D5` (`README.md` and `CHANGELOG.md` prose unpinned): hand-authored release prose outside any render pipeline. Pinning it is not a thing this project does or should start doing here.
- `P2` (probe evaluated twice) and `T2` (an assertion weakened with no cross-test catch): both expected non-catches, neither a gap.
- The `Err` branch's remedy clause (`M2`) as a finding of its own: already recorded as a round 4 observation, and folded into `R5M-1` as one member of a family rather than re-raised.

---

# Tally

| Severity | Count |
| --- | --- |
| critical | 0 |
| high | 0 |
| medium | 0 |
| low | 2 |

`R5M-1` and `R5M-2`, both `low`, both coverage gaps, neither claiming a defect in the shipped behaviour.

I record for the triager that I was told this artifact is at round 5 of a cap of 5 with a streak of 1, and that a valid finding here prevents convergence before the cap. Neither finding was raised for the sake of raising one and neither was withheld for the sake of converging. Both rest on a mutation that leaves the suite at 422/0, both name the smallest remedy, and for `R5M-2` I built and ran the proposed remedy's fixture against both designs rather than asserting it would work. If the triager judges that a `low` coverage gap on a branch whose shipped behaviour is correct does not warrant reopening the increment, that is a reasonable reading of both, and `R5M-2`'s cheap variant (make the degeneration loud) exists partly so that reading has somewhere to land.

# Relitigation and constraints check

Nothing above raises or reopens the four standing residuals; accepted costs (i) to (iv), which appear only as pinned expected behaviour and as catching tests; round 1's `ADV-4` or `SC-3`; round 2's `R2A-4`, `R2B-2` or `R2B-3`; round 3's `R3A-1` or `R3A-3`; round 4's `R4A-1`; the pre-existing plain-`validate` inconsistency; the pre-existing containment TOCTOU; or the check-16 vacuous pass, which Part 2 excludes by name. `R5M-2` explicitly does NOT propose changing the gate to `try_exists()?`: its remedy is a test fixture and touches no source. No line-length or prose-wrapping observation appears anywhere in this file.

# TREE STATE: NO SOURCE CHANGE REMAINS

Stated explicitly because it is the most important safety property of this run.

Every one of the 30 mutations was reverted with `git checkout -- .` immediately after its test run, and the harness printed a proven-clean check after each one; all 30 printed `TREE CLEAN AT 3f19bd1`. The two mutations applied outside the harness (the `ENOTDIR` experiments) were reverted in the same command that ran them, with the empty status printed.

Final state, measured after the last mutation and before this file was written:

```
$ git status --porcelain
(empty)
$ git diff HEAD --stat
(empty)
$ git rev-parse HEAD
3f19bd1e5a18b877f4298fcad094f3fb97442246
$ cargo test          # TMPDIR outside any git repository
422 passed, 0 failed
```

The tree carries NO source changes, no test changes, and no prose changes. The only file this run authors is this one. The main repository at `/home/jessea/Documents/projects/agent-scaffold` was not touched, and no other worktree was touched.

FIXTURE HYGIENE: every fixture lives under `<scratch>/r5mut/`, a directory of my own naming. Nothing outside it was written or deleted, and nothing was written into bare `/tmp`. I created no 000 or 600 fixture at any point; the mode-600 directory in `a_round_log_that_cannot_be_checked_is_not_reported_as_missing` is the suite's own and the test restores it to 0755 before its assertions. The closing sweep for restrictive directories, mode-000 files and FIFOs under my subdirectory returns nothing.
