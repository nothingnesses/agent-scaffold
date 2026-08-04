# `workflow-enforcement-tier-inc2`, work review ROUND 3, ISOLATED REVIEWER, FIX-VERIFICATION AND REGRESSION LENS

ARTIFACT. The round 2 fix pass alone is `git diff HEAD~1..HEAD` at commit `a7e05c3` ("fix: root containment on an anchor that does not exist"); the whole increment is `git diff main..HEAD`. Reviewed in the worktree `.claude/worktrees/r3-fixverify`.

SOURCES READ. Both triages and all six reviewer files under `docs/plans/workflow-enforcement-tier-inc2.reviews/`, the specification `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, and the shipped `src/main.rs`, `src/next.rs`, `README.md`, `CHANGELOG.md` and `tests/unsafe_pairings_are_refused_and_omitted.rs`.

METHOD. Every verdict below is a measurement I produced, never a reading of the fix pass's report. Two binaries were built for the differential work, `bin-prefix` from `HEAD~1` (via `git checkout HEAD~1 -- src/main.rs`, build, copy, revert) and `bin-postfix` from `HEAD`, so every "unchanged" claim is a diff of two real runs rather than an inference. Twenty-four mutations were applied and reverted ONE AT A TIME, each with a full `cargo test` and a `git status --short` check after. Fixtures were built by hand from a shell script rather than by reusing the suite's helpers, and every fixture uses TOP-LEVEL SIBLING projects with no nesting anywhere, so the in-root bound cannot be the explanation for any result here.

BASELINE ESTABLISHED FIRST, with `TMPDIR` pointed outside every repository as specification line 311 requires:

```
cargo build                                -> Finished dev profile
cargo test                                 -> 416 passed, 0 failed
cargo clippy --all-targets -- -D warnings  -> clean
cargo run -- render docs/plans/agent-scaffold.plan.toml --check -> up to date
cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow -> invariants hold, exit 0
```

The integration file `tests/unsafe_pairings_are_refused_and_omitted.rs` went from 15 tests to 16; the suite total went from 415 to 416.

---

## 1. THE FIVE ROUND 2 FINDINGS: VERIFICATION TABLE

**ALL FIVE VERIFIED CLOSED.** One of the five is closed with an unguarded half, filed below as `R3F-1`; the BEHAVIOUR is closed on every surface, the TEST COVERAGE of the note is not.

| Id | Severity | Verdict | Evidence I produced |
| --- | --- | --- | --- |
| G-EMPTYROOT (R2A-1 + FV-1) | `high` | VERIFIED CLOSED on all three surfaces, human and JSON | Two-sibling fixture built by hand; `next`, `status` and `status --resume` all refuse the foreign artifact on a missing anchor, both `--json` reason fields carry a token, and the one-character control gives the SAME verdict. Section 1.1. |
| R2C-1 | `low` | VERIFIED CLOSED | Every clause of the rewritten `README.md:236` checked against a run; the qualification ("that reads a plan", and `validate --workflow` refusing on the no-plan ground) is TRUE of the binary. Section 1.2. |
| FV-2 | `low` | VERIFIED CLOSED | The replacement no longer names one configuration as the trigger and no longer attributes the multi-root rule to a plan-reading `status`/`next`; both statements measured. Section 1.2. One clause of the replacement is itself inaccurate: `R3F-2`, `low`. |
| R2C-3 | `low` | VERIFIED CLOSED | The renamed test asserts BOTH halves its new name claims: `validate` exit 1 at `tests/unsafe_pairings_are_refused_and_omitted.rs:552-555`, and the omission on `status` and `next` at 558-563. |
| R2C-4 | `low` | VERIFIED CLOSED | The reworded doc comment at `tests/unsafe_pairings_are_refused_and_omitted.rs:568-575` matches the body: the ledger question is asked of `status --resume` (596-604) and `next` (606-623), the log question of `next` and `status` (626-644), and `Projection` (`src/main.rs:569-578`) genuinely has no ledger field, so the claim that no single run asks all three the same question is true. |

### 1.1 G-EMPTYROOT, verified closed on all three surfaces

FIXTURE, built by hand. `$R/home` and `$R/alpha` are TOP-LEVEL SIBLINGS. `home` (the directory every run stands in) holds a three-record log and a `## RESUME STATE` block reading "HOME resume state."; `alpha` holds its own two-record log, its own ledger reading "ALPHA resume state.", and a `p.plan.toml` but NO `q.plan.toml`. The attack names `$R/alpha/docs/plans/q.plan.toml` as `--source` and `home`'s own artifacts explicitly.

```
cd $R/home && agent-scaffold next --source $R/alpha/docs/plans/q.plan.toml \
    --metrics docs/metrics/workflow.jsonl --ledger-fragment docs/plans/p.ledger.md
exit=0
stdout: metrics: unavailable, the round log docs/metrics/workflow.jsonl is not under the plan's project root $R/alpha, ...
        no active review loop (no plan steps found)
        the ledger docs/plans/p.ledger.md is not under the plan's project root $R/alpha; nothing to resume
stderr: note: --source $R/alpha/docs/plans/q.plan.toml does not exist
```

- `next --json`, same arguments: `"metrics": null`, `"metrics_absent_reason": "log-not-this-project"`, `"resume_state": null`, `"resume_state_absent_reason": "ledger-not-this-project"`, `"active_loop": null`, exit 0. No `null` reason positively asserting the artifact is this plan's.
- `status --source <missing> --metrics docs/metrics/workflow.jsonl`: `metrics: unavailable, ...`, and `status --json` gives `"metrics_absent_reason": "log-not-this-project"`. The three-record count never appears.
- `status --resume --source <missing> --ledger-fragment docs/plans/p.ledger.md`: prints the ledger note, and no line of "HOME resume state." appears anywhere.
- THE ONE-CHARACTER CONTROL (`q.plan.toml` -> `p.plan.toml`, an anchor that IS on disk, everything else identical): the same three surfaces give the SAME verdicts, and stderr is EMPTY. The leak and the refusal are no longer one character apart.
- THE ANCHOR'S OWN ARTIFACTS ARE STILL READ under the derived root: `next --source <missing> --metrics $R/alpha/docs/metrics/workflow.jsonl --ledger-fragment $R/alpha/docs/plans/a.ledger.md` prints `metrics: 2 records` and echoes "ALPHA resume state.". The remedy withholds nothing the anchor's own directory owns.

THE NOTE IS STDERR ONLY, and I checked that with a parser rather than by eye. `next --json ... 2>/dev/null` and `status --json ... 2>/dev/null` both parse as JSON (`json.load` succeeds; keys `['active_loop', 'metrics', 'metrics_absent_reason', 'no_active_loop_reason', 'resume_state', 'resume_state_absent_reason', 'source', 'task']` and `['metrics', 'metrics_absent_reason', 'plan']`). The same stream with `2>&1` FAILS to parse (`JSONDecodeError`), which is the positive control proving the note really is on the other stream. `note_missing_anchors` has exactly two call sites, `src/main.rs:1142` (`run_status`, before the `--resume` split, so both slices report) and `src/main.rs:1576` (`run_next`); `validate` and `audit` are untouched.

THE FIXTURE IS DISCRIMINATING, which I proved rather than assumed. Reverting the one changed line at `src/main.rs:1516` to the pre-fix `.filter_map(|anchor| canonical_project_root(anchor))` turns the suite RED on `an_anchor_that_does_not_exist_still_supplies_a_root` (412 passed, 1 failed), and the same fixture then reads and echoes `home`'s log and block. The whole-binary differential in section 3.2 shows the same thing from the outside.

### 1.2 R2C-1 and FV-2, clause by clause against the binary

Every claim of the rewritten `README.md:236` and `CHANGELOG.md:23`, each checked against a run:

| Claim | Measurement | Verdict |
| --- | --- | --- |
| "Every one of these commands THAT READS A PLAN checks that the log ... lives under the project root of THAT plan". | `validate --source docs/plans/s.plan.toml --plan docs/plans/s.md --workflow --metrics ../beta/docs/metrics/workflow.jsonl` -> exit 1, "would join docs/plans/s.md against ../beta/... which is not under the plan's project root". `next` with a `--plan` in project B and B's artifacts reads them (root is the checked plan's). | TRUE |
| "Where no plan is read, which is always so for `status --resume` and is so for `status` and `next` whenever neither a TOML-primary `--source` nor a readable `--plan` resolves". | `containment_roots` (`src/main.rs:1379`) falls back exactly when `checked_plan_root` is `None`, which is exactly that condition; the twelve-cell enumeration in section 3.3 exercises each branch. | TRUE |
| "every `--source` or `--plan` you gave yields one and the artifact must be under all of them, so a `--source` and a `--plan` naming two different projects reject each other's artifacts". | Configuration K2 (Markdown-primary `--source` in A, missing `--plan` in B): A's OWN explicit log and ledger are both refused, as are B's. | TRUE |
| "An anchor that does not exist still yields a root ... and a `note:` on stderr tells you the anchor is not there". | Section 1.1. | TRUE (the root is not derived "as you spelled it" though; `R3F-2`) |
| "what the anchor's own directory owns is still read, so a plan file you have not written yet still reads its own project's log". | Section 1.1, own-artifacts block; and T5, T12 in section 3.4 (`--metrics docs/metrics/workflow.jsonl` and the DEFAULT log both read under a missing anchor). | TRUE |
| "With NEITHER anchor ... no root is derived, no containment check fires, and the current-directory-relative defaults described above stand". | S4 rows in section 3.3: with no `--source` and no `--plan`, `status` and `next` read the cwd's own three-record log, `status --resume` reports `no ledger at docs/plans/task.ledger.md`, an EXPLICIT foreign `--metrics` and `--ledger-fragment` are still read and echoed, stderr is empty, and every one of those rows is byte-identical to `bin-prefix`. | TRUE |
| "`validate --workflow` has no such fallback and needs none ... it refuses on that ground (`--workflow requested but no plan source resolved`) without ever reaching containment". | Six `validate` configurations (missing `--source`; Markdown-primary `--source` with a missing `--plan`; each with and without a foreign `--metrics`) -> exit 1 with exactly that message, pre-fix and post-fix BYTE-IDENTICAL. | TRUE |

FV-2's specific complaint is closed: the sentence no longer enumerates "a Markdown-primary `--source` and no `--plan`" as though it were the trigger (it now states the condition on `checked_plan_root`), and no longer attributes "under every one of them" to a `status`/`next` that read a plan.

---

## 2. THE NINE ROUND 1 FINDINGS: STILL CLOSED

**ALL NINE STILL CLOSED. Nought reopened by the round 2 fix.** Each row is my own measurement on the round 3 tip.

| Id | Kind | Verdict | Evidence I produced on this tip |
| --- | --- | --- | --- |
| ADV-1 | product | STILL CLOSED | The round 1 spelling rebuilt by hand (Markdown-primary `--source` in `alpha`, NO `--plan`, `beta`'s ledger and log named explicitly): `next` prints the unpairable-ledger note and no line of BETA's block, `status --resume` prints the same note, `status` reports `metrics: unavailable`. All three agree; exit 0 throughout. Five separate mutations (M7, M8, M22, M23, M28) each turn `a_surface_that_reads_no_plan_is_supplied_a_root` RED, so the guard is live. |
| EVI-1 | test-only | STILL CLOSED | M19 (`resolve_for_containment` -> `.ancestors().skip(1)`, `src/main.rs:1403`) -> RED, 412 passed, 1 failed: `a_symlinked_log_leaf_outside_the_root_is_refused_and_omitted`. |
| EVI-2 | test-only | STILL CLOSED | M26 (the refusal ACCOMPANIES the check instead of replacing it: `} else {` -> `}` plus a bare block at `src/main.rs:995`) -> RED, 412 passed, 1 failed: `an_explicit_metrics_outside_the_plans_root_is_refused`. |
| EVI-3 | test-only | STILL CLOSED | BOTH halves. M27 (`LogAbsent` -> `LogNotThisProject` at `src/main.rs:1197`) -> RED, 1 failed: `the_machine_surface_separates_the_causes_on_both_commands`. M4 (the precedence rule at `src/main.rs:1184` weakened to `unpairable_log.is_some() && !metrics_path.exists()`) -> RED, 8 failed including `status_omits_only_the_unpairable_part` and the same machine-surface test. |
| EVI-4 | test-only | STILL CLOSED | M5 (`run_resume` tests existence BEFORE containment, `src/main.rs:1539-1549`) -> RED, 412 passed, 1 failed: `status_omits_only_the_unpairable_part`. |
| EVI-5 | test-only | STILL CLOSED | M28 (`resume_state_absent_note: None` at `src/main.rs:1673`, reason still computed) -> RED, 410 passed, 3 failed: `a_surface_that_reads_no_plan_is_supplied_a_root`, `an_anchor_that_does_not_exist_still_supplies_a_root`, `the_resume_reasons_separate_and_cover_the_default_ledger`. Stronger than round 2, which caught it with two. |
| EVI-6 | test-only | STILL CLOSED | M24 (the third remedy dropped from the refusal message at `src/main.rs:991`) -> RED, 412 passed, 1 failed: `a_divergent_source_and_plan_pairing_is_refused`. |
| TRI-1 | doc | STILL CLOSED | `grep -rn "already holds the paths" src/` returns nothing (exit 1). The retained text at `src/next.rs:193-197` is true on every path I drove: the caller does hold both paths, the enums carry none, `Some` tracks `LogNotThisProject`, and the note is absent from `--json` on every invocation I made. |
| ADV-2 | decided residual | UNCHANGED, NOT RE-RAISED | `next --source docs/plans/agent-scaffold.plan.toml --ledger-fragment /tmp/foreign.ledger.md` prints `ledger: /tmp/foreign.ledger.md` inside the `ACTIVE LOOP` block above the rejection note, and the output is IDENTICAL from `bin-prefix` and `bin-postfix`. The round 2 fix neither closed it nor disturbed it, which is what `q_id:"Q-55-ledgerslot"` requires. |

---

## 3. REGRESSION

### 3.1 The mutation set: twenty-three of twenty-four still caught, one SURVIVES

Every mutation was applied to a clean tree, measured with a full `cargo test`, and reverted with `git checkout -- src/main.rs`; `git status --short` was empty after every one of the twenty-four.

| Mutation | Site | Result |
| --- | --- | --- |
| M2, `checked_plan_root` roots on the ANCHOR (`source.as_ref().or(plan.as_ref())`), ignoring `toml_primary`. THE CENTRAL ONE. | `src/main.rs:1351` | STILL CAUGHT. 3 failed: `a_divergent_source_and_plan_pairing_is_refused`, `accepted_costs_three_and_four_are_pinned`, `the_resume_reasons_separate_and_cover_the_default_ledger`. The same three as rounds 1 and 2. |
| M1, `is_outside_root` returns `false` unconditionally. | `src/main.rs:1426` | STILL CAUGHT. 14 tests failed, of the 16 in the containment file. |
| M29, the false-positive direction, `is_outside_root` returns `true`. | `src/main.rs:1426` | STILL CAUGHT. 8 failed, all in the inc1 file `metrics_and_ledger_anchor_to_the_plan_source.rs`. |
| M20, `Path::starts_with` replaced by a string-prefix comparison. | `src/main.rs:1426` | STILL CAUGHT. `accepted_cost_two_the_symlinked_layouts_are_pinned`. |
| M17, `canonical_project_root` made LEXICAL. | `src/main.rs:1328` | STILL CAUGHT. `a_symlinked_source_cannot_borrow_its_neighbours_log`, `accepted_cost_two_the_symlinked_layouts_are_pinned`. |
| M18, `resolve_for_containment` drops canonicalisation entirely. | `src/main.rs:1403-1409` | STILL CAUGHT. 3 failed. |
| M19, `resolve_for_containment` skips the path itself. | `src/main.rs:1403` | STILL CAUGHT. 1 failed (EVI-1's test). |
| M21, `resume_roots` drops the `--plan` anchor. | `src/main.rs:1513` | STILL CAUGHT. `resume_omits_the_default_ledger_under_a_divergent_pairing`, `accepted_costs_three_and_four_are_pinned`. The two-anchor intersection is still guarded after the root supply changed a second time. |
| M6, the refusal removed on `validate` only. | `src/main.rs:981` | STILL CAUGHT. 7 failed. |
| M7, the metrics omission removed on `status` only. | `src/main.rs:1180-1183` | STILL CAUGHT. 8 failed, up from 7. |
| M8, the metrics omission removed on `next` only. | `src/main.rs:1618-1621` | STILL CAUGHT. 8 failed, up from 7. |
| M9, `next` treats an unpairable log as ABSENT (the specification line 187 trap). | `src/main.rs:1626` | STILL CAUGHT. 5 failed. |
| M22, `next`'s LEDGER containment check removed. | `src/main.rs:1638-1641` | STILL CAUGHT. 3 failed, up from 2. |
| M23, `status --resume`'s ledger containment check removed. | `src/main.rs:1539-1545` | STILL CAUGHT. 5 failed, up from 4. |
| M24, M26, M27, M4, M5, M28 (the six round 1 coverage findings). | see section 2 | ALL STILL CAUGHT. |
| MA, the round 2 fix itself reverted (`.map(project_root_of_source o resolve_for_containment)` -> `.filter_map(canonical_project_root)`). | `src/main.rs:1516` | CAUGHT. 1 failed: `an_anchor_that_does_not_exist_still_supplies_a_root`. The fix is guarded. |
| MB-NOTE-NEXT, the `note_missing_anchors` call deleted from `run_next`. | `src/main.rs:1576` | CAUGHT. 1 failed: `an_anchor_that_does_not_exist_still_supplies_a_root`. |
| MB-NOTE-STDOUT, the note printed with `println!` instead of `eprintln!`. | `src/main.rs:1116` | CAUGHT. 1 failed: same test. |
| **MB-NOTE-STATUS, the `note_missing_anchors` call deleted from `run_status`.** | `src/main.rs:1142` | **SURVIVES. 416 passed, 0 failed, the exact green baseline.** Filed as `R3F-1`. |

NO MUTATION THAT WAS CAUGHT IN ROUND 1 OR ROUND 2 SURVIVES IN ROUND 3, and several are caught by MORE tests than before. The round 2 fix weakened no existing guard.

### 3.2 The whole-binary differential

`bin-prefix` (`HEAD~1`) and `bin-postfix` (`HEAD`) were driven over 140 invocations each in two scripted matrices (122 and 18 rows), plus six `validate --workflow` configurations, and diffed. **EVERY verdict change is in the direction of REFUSAL or OMISSION. Nothing that was refused before is now read, and nothing that was read before is now refused except where an anchor was supplied that the run could not have vouched for.** The changed cells are:

- Four leak closures where a MISSING anchor previously contributed no root: `--source` missing alone (foreign log read at 4 records -> unpairable; foreign ledger echoed -> unpairable), `--plan` missing alone (same), both anchors missing (same), and, on `status --resume` only, a missing `--source` beside a PRESENT `--plan` in another project (BETA's block echoed -> refused).
- Three configurations where a supplied but missing `--plan` now contributes a SECOND root and so refuses artifacts the other anchor owns. Each of them is analysed in section 3.3 and each matches the verdict the SAME configuration already gave when that `--plan` existed.
- The stderr `note:` line, on every run with a missing anchor. No stdout byte changed on account of it.

### 3.3 THE TWO-ROOT INTERSECTION: the complete enumeration

`containment_roots` (`src/main.rs:1379`) returns `resume_roots` exactly when `checked_plan_root` is `None`. `checked_plan_root` is `None` exactly when the source is not TOML-primary AND the `--plan` is absent or does not canonicalise. `toml_source` (`src/main.rs:1070`) returns `Some` only for an EXISTING, parseable, `primary = "toml"` source. So the vector holds TWO roots in exactly ONE shape:

**a `--source` supplied and NOT TOML-primary (missing, or not a `.plan.toml`, or Markdown-primary, or unparseable), TOGETHER WITH a `--plan` supplied that does NOT exist.**

Everything else holds at most one: a TOML-primary `--source` or an existing `--plan` gives the single checked-plan root; one anchor alone gives one; neither gives none. Before the fix this shape gave ONE root (the `--plan` was dropped), or none when the `--source` was also missing.

I enumerated the shape across six layouts crossed with three artifact sets and three surfaces, each run TWICE, once with the `--plan` absent (the two-root case) and once with the identical path written (the one-root control), on both binaries. `A` and `B` are top-level siblings; every run stands in a third sibling `home`.

| Layout | `--source` | `--plan` (missing) | Two roots | Post-fix verdict versus the plan-PRESENT control |
| --- | --- | --- | --- | --- |
| K1 | `A/docs/plans/s.plan.toml`, Markdown-primary | `A/docs/plans/s.md` | {A, A} | IDENTICAL in all nine cells. A's own artifacts read, B's refused. The ordinary "the Markdown has not been rendered yet" case loses nothing. |
| K2 | `A/docs/plans/s.plan.toml` | `B/docs/plans/s.md` | {A, B} | IDENTICAL on `status --resume` and on the DEFAULT and OWN-A sets; A's own log and ledger are refused in both, because the run named a plan in B. Differs from the control only on FOREIGN-B for `status`/`next`, where the control READS B's artifacts because a plan in B was actually read. That is the decided two-policy split (specification lines 182 and 183), not a new rule. |
| K3 | `A/docs/plans/s.plan.toml` | `A/notes/s.md`, same project, off the `docs/plans` convention | {A, A/notes} | IDENTICAL in all nine cells, including the control. A's own default log is refused in BOTH, because a plan at `A/notes/s.md` derives the root `A/notes` whether or not it exists. Pre-existing convention (`R2A-2`'s class, absorbed by specification line 271), not something this fix introduced. |
| K4 | `A/docs/plans/nope.plan.toml`, missing | `A/docs/plans/s2.md` | {A, A} | IDENTICAL to the control. DEFAULT and OWN-A artifacts still READ (2 records, ALPHA's block); only FOREIGN-B refused, which is the leak closure. |
| K5 | `A/docs/plans/nope.plan.toml`, missing | `B/docs/plans/s2.md` | {A, B} | IDENTICAL to the control on `status --resume` and on OWN-A. Differs only on FOREIGN-B for `status`/`next`, the same decided split as K2. |
| K6 | `A/notes/n.plan.toml`, Markdown-primary, OFF the convention | `A/docs/plans/s3.md` | {A/notes, A} | Verdicts BYTE-IDENTICAL to `bin-prefix` in all nine cells. The `--source` exists, so it already supplied `A/notes` before the fix; the new second root changes nothing. |

**NO CONFIGURATION REFUSES OR OMITS ANYTHING LEGITIMATE THAT IT DID NOT ALREADY REFUSE.** Where a two-root configuration now refuses an artifact, the SAME configuration with the `--plan` present refuses it too (K2 DEFAULT and OWN-A, K3 everywhere), pre-fix and post-fix alike; the fix made the missing-`--plan` case agree with the existing-`--plan` case rather than diverging from it. The only cells where the two-root case is STRICTER than its one-root control are K2 and K5 with FOREIGN-B on `status`/`next`, and there the control reads B's artifacts precisely because it READ B's plan; with no plan read there is nothing to prefer B over the other anchor, and `status --resume` gives the strict answer in both.

THE NEITHER-ANCHOR CASE IS UNCHANGED, measured rather than argued. With no `--source` and no `--plan`: `status` and `next` read the current directory's own `docs/metrics/workflow.jsonl` (3 records), `status --resume` reports `no ledger at docs/plans/task.ledger.md`, an EXPLICIT foreign `--metrics` and `--ledger-fragment` are still read and echoed at exit 0, and stderr is empty. All six of those rows are byte-identical between `bin-prefix` and `bin-postfix`, which is the `README.md:235` contract intact.

### 3.4 DIMENSIONS I VARIED, AND DIMENSIONS I DID NOT

A negative result is bounded by what the fixture varied, so both lists are stated.

VARIED:

- Anchor existence: `--source` present/missing, `--plan` present/missing, both, neither.
- Anchor kind: TOML-primary `.plan.toml`, Markdown-primary `.plan.toml`, a plain `.md`, and a path with no file behind it.
- WHERE THE ANCHOR SITS RELATIVE TO `docs/plans` (the dimension round 2's matrix missed, on both anchors independently): a `--source` under `<root>/docs/plans`, a `--source` under `<root>/notes` with no `docs/plans` ancestor (K6), a `--plan` under `<root>/docs/plans`, and a `--plan` under `<root>/notes` (K3).
- Anchor spelling: ABSOLUTE, RELATIVE from the project root, a BARE FILENAME run from inside `docs/plans`, a path with `..` that climbs out through a `docs/plans`, and a path through a SYMLINKED directory with a missing leaf.
- Artifact sets: none (defaults), the anchor's own explicit `--metrics` and `--ledger-fragment`, a foreign project's, and a foreign artifact reached through a symlink.
- Surfaces: `status`, `status --json`, `status --resume`, `next`, `next --json`, `validate --workflow`.
- Project topology: TOP-LEVEL SIBLINGS only, deliberately, so no result here can be attributed to the in-root bound.
- Pre-fix versus post-fix binaries over every cell.

NOT VARIED, and no claim above extends to them:

- NESTED projects (one project's tree inside another's). That is the in-root bound, recorded and not closed by human decision, and out of scope.
- File PERMISSIONS: an anchor or artifact that exists but cannot be read. `checked_plan_root` tests canonicalisation, not readability, so the README's word "readable" is an approximation I did not probe.
- An anchor that is a DIRECTORY, or a DANGLING symlink (where `Path::exists` is false and the note would fire on a path that does exist as a link).
- Non-UTF8 paths, paths with newlines, and non-Unix platforms.
- Concurrent writes to the log or ledger while a command reads it, and logs large enough to matter.
- `audit`, `render`, `checks` and `scaffold`, which never reach `containment_roots`.
- `--workflow-spec` variants; every `validate` run used the built-in constants.

---

## 4. NEW FINDINGS

### R3F-1: `note_missing_anchors` is UNGUARDED on `status` and on `status --resume`; deleting it leaves the suite fully green

SEVERITY: `medium`. The behaviour is correct on all three surfaces; the COVERAGE is present on one.

`q_id`: none (new).

THE MUTATION AND THE OBSERVED OUTPUT. Delete the call at `src/main.rs:1142`:

```
 fn run_status(args: StatusArgs) -> io::Result<()> {
-	note_missing_anchors(&args.source, &args.plan);
 	// The thin `status --resume` slice: ...
```

```
cargo test  ->  416 passed, 0 failed
```

That is the EXACT green baseline, name for name. Compare the same deletion in `run_next` at `src/main.rs:1576`, which turns `an_anchor_that_does_not_exist_still_supplies_a_root` RED, and the `eprintln!` -> `println!` change at `src/main.rs:1116`, which also turns it RED. The fix pass added two call sites and pinned one.

WHAT THE MUTANT DOES, behaviourally, on the section 1.1 fixture:

```
cd $R/home && agent-scaffold status --source $R/alpha/docs/plans/q.plan.toml --metrics docs/metrics/workflow.jsonl
exit=0
stdout: plan: not provided
        metrics: unavailable, the round log ... is not under the plan's project root $R/alpha, ...
stderr: (EMPTY)
```

and `status --resume --source <missing> --ledger-fragment docs/plans/p.ledger.md` likewise prints its refusal with an EMPTY stderr. Both slices go back to deriving a containment root from a name with nothing behind it and saying nothing about it, which is the exact condition the remedy was accepted with; the test that pins the `next` half says so in its own words at `tests/unsafe_pairings_are_refused_and_omitted.rs:704` ("Fail loudly, the condition the remedy was accepted with"). Nothing else on either surface distinguishes a typo'd anchor from a correct one: `status` prints `plan: not provided` for a `--source` that is missing and for no `--source` at all alike.

WHY `medium` RATHER THAN `low`. Round 1 rated an unpinned load-bearing clause `medium` five times over (EVI-1 through EVI-5) on exactly this shape: shipped behaviour is right, one mutation shows nothing holds it. This one is the guard on a condition attached to a `high` finding's remedy, on two of the three surfaces that leak was found on, and the mutation that removes it is a one-line deletion of the kind a later refactor makes casually. A triager may reasonably prefer `low` on the ground that the `next` half IS pinned and the three surfaces share one function, so a partial deletion is unlikely; I would answer that "unlikely" is what the suite is for, and that the mutation took one line.

PRESCRIPTION (test-only, no product change). Two assertions inside the existing `an_anchor_that_does_not_exist_still_supplies_a_root`, beside the `next` assertion already there at line 704: assert the same `note: --source <path> does not exist` on `stderr` for the `status --resume` run at `tests/unsafe_pairings_are_refused_and_omitted.rs:740` and for the `status --json` run at line 750. Both runs already exist in the test and already bind `stderr`; only the assertions are missing.

### R3F-2: the new README and CHANGELOG clause "derived from the path as you spelled it" is false; the root is the path RESOLVED through its longest existing ancestor

SEVERITY: `low`. Documentation accuracy, in a clause this fix pass authored.

`file:line`: `README.md:236` ("An anchor that does not exist still yields a root, derived from the path as you spelled it") and `CHANGELOG.md:23` ("still yields a root, derived from the path as spelled").

THE COMMAND AND THE OBSERVED OUTPUT. `$R/link-to-alpha` is a symlink to `$R/alpha`; the anchor's leaf does not exist; the log named is a foreign project's, so the tool prints the root it derived:

```
cd $R && agent-scaffold status --source $R/link-to-alpha/docs/plans/nope.plan.toml \
    --metrics $R/beta/docs/metrics/workflow.jsonl
note: --source $R/link-to-alpha/docs/plans/nope.plan.toml does not exist
plan: not provided
metrics: unavailable, the round log $R/beta/docs/metrics/workflow.jsonl is not under the plan's
         project root $R/alpha, so its records cannot be paired with this plan
```

The path spelled was `link-to-alpha`; the root reported is `alpha`, a directory name the caller never typed. The clause is not merely imprecise, it predicts the wrong verdict: under an as-spelled root of `$R/link-to-alpha`, the anchor's OWN log resolves (through `resolve_for_containment`, which canonicalises the artifact) to `$R/alpha/docs/metrics/workflow.jsonl`, which does not start with `$R/link-to-alpha` and would be REFUSED. The tool reads it:

```
cd $R && agent-scaffold status --source $R/link-to-alpha/docs/plans/nope.plan.toml \
    --metrics $R/link-to-alpha/docs/metrics/workflow.jsonl
metrics: 2 records
```

The plainest case shows it too, with no symlink involved: with `--source docs/plans/nope.plan.toml` run from the project root, the reported root is the ABSOLUTE `$R/alpha`, not the relative spelling.

The code's own doc comment gets this right at `src/main.rs:1495-1497` ("resolved by the same `resolve_for_containment` the predicate already resolves the ARTIFACT with: absolutise, canonicalise the longest existing ancestor, re-append the rest"), so the two documents disagree with each other as well as one of them with the binary. The distinction is load-bearing in the same paragraph, which four sentences earlier promises "resolving both through their real on-disk locations so a symlink cannot disguise one as the other".

PRESCRIPTION. ONE CLAUSE on each line. Replace "derived from the path as you spelled it" with something like "derived from the path itself, resolved as far as the filesystem allows" (`README.md:236`), and the same on `CHANGELOG.md:23`. The intended contrast (the root comes from the path, not from a file that was read) survives; the false claim that no resolution happens does not.

---

## 5. WHAT I ATTACKED AND FOUND NOTHING IN

- THE SECOND CHANGE OF ROOT DERIVATION did not weaken the central property. M2 still fails the same three tests it failed in rounds 1 and 2, and M21 still fails the two that pin the two-anchor intersection on `status --resume`.
- THE VOCABULARY AND PRECEDENCE RULES are intact under mutation on both commands (M27, M4, M9, M5) and on both the human and the machine surface.
- `validate --workflow` IS GENUINELY UNTOUCHED. Six configurations, byte-identical between the two binaries, and the no-plan-resolved cases refuse on that ground without reaching containment, exactly as the new README sentence claims.
- THE `..` AND SYMLINK CLAUSES survive on a MISSING anchor, which is new ground: a `..` that climbs out through a `docs/plans` derives its root from the climbed-to directory, and a symlinked anchor prefix resolves to the real root (which is what makes `R3F-2` a documentation defect rather than a behavioural one).
- THE DEFAULT ARTIFACTS AND THE DERIVED ROOT NEVER CONTRADICT EACH OTHER in any configuration I built: wherever the default log or ledger was refused, the anchors genuinely disagreed about the project, and the plan-present control refused it too.
- NO TEST'S MEANING CHANGED in the round 2 fix pass. `git diff --numstat HEAD~1..HEAD -- tests/` is `168 2`; the two deletions are the renamed function signature and the reworded doc-comment line, findings R2C-3 and R2C-4 themselves, and no assertion was edited, relaxed or removed.
- THE GUARDS ARE GREEN on the unmutated tip: `cargo test` 416 passed, `cargo clippy --all-targets -- -D warnings` clean, `render --check` up to date, `validate --workflow` reports invariants hold at exit 0 against the repository's own 270-record log.

---

## 6. ROUND OUTCOME FROM THIS LENS

The five round 2 findings are CLOSED and the nine round 1 findings are STILL CLOSED, all verified independently. The round 2 fix broke nothing: no previously caught mutation survives, the whole-binary differential moves only in the direction of refusal, the newly reachable two-root configurations refuse nothing that the same configuration did not already refuse with the `--plan` present, and the neither-anchor case is byte-identical.

TWO NEW FINDINGS: `R3F-1` (`medium`, test-only, one missing pair of assertions in a test that already makes the runs) and `R3F-2` (`low`, one clause in each of `README.md:236` and `CHANGELOG.md:23`). Neither is a behavioural defect in the shipped binary.

`git status --short` in this worktree shows only this file. Every mutation was reverted and the tree checked clean after each.
