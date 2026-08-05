# `workflow-enforcement-tier-inc2`, work review ROUND 4, ISOLATED REVIEWER, REGRESSION AND CLOSURE LENS

Commit under review: `b54ba3a` ("fix: scope the guessed anchor root and split \"missing\" from \"cannot tell\""), the tip of `main..HEAD`. The increment is the four commits `1c46e7c`, `815fb29`, `b387b4f`, `2b1e39c`, `b54ba3a` above `main`.

Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/r4-regression`. All fixtures under `<S> = /tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/r4reg-*`. `<S>` abbreviates that prefix throughout.

## 0. THE FOUR BINARIES THIS LENS COMPARED AGAINST, AND WHY

Round 3's version of this lens measured three moved cells correctly and then adjudicated them against a neighbouring configuration instead of against the binary that behaved differently. So every "unchanged" and every "still closed" below names its baseline, and every baseline is a BUILT BINARY rather than a neighbouring command line. Four were built from `git archive <rev> | tar -x` into `<S>/r4reg-bin/` and compiled with the project toolchain:

| Label | Revision | What it is the control for |
| --- | --- | --- |
| `as-head` | `b54ba3a` (HEAD) | the artifact under review. |
| `as-pre3` | `2b1e39c` (HEAD~1) | the round 3 PRE-FIX binary. The control for all six round 3 closures. |
| `as-pre2` | `b387b4f` (HEAD~2) | the pre-round-2-fix binary, the one that LEAKED. The control for `G-EMPTYROOT`. |
| `as-main` | `main` | the PRE-INCREMENT binary. The control for "nothing that worked before now refuses" and for `ADV-1`. |

Every control below is checked for the property the round 3 triage demanded: that it CAN show the change. Where a baseline turned out to be incapable of showing a change, that is stated rather than reported as a null result.

Baseline suite: `cargo test` -> 418 passed, 0 failed, across nine binaries (378 + 5 + 1 + 1 + 9 + 3 + 18 + 1 + 2). Re-run after every mutation was reverted: 418 passed, 0 failed.

## 1. THE SIX ROUND 3 FINDINGS

| Id | Sev | Verdict | Evidence |
| --- | --- | --- | --- |
| `R3A-1` | `medium` | CLOSED | C1 is now byte-identical to C0 on all five surfaces, and C2 is byte-identical to the PRE-FIX binary. Section 1.1. |
| `R3F-1` | `medium` | CLOSED | Deleting the `note_missing_anchors` call at `src/main.rs:1153` reds 2 tests. Section 1.2. |
| `R3A-2` | `low` | CLOSED | An anchor on disk under a `0o000` directory now prints `could not be checked`, where the pre-fix binary printed `does not exist`. Section 1.3. |
| `R3A-3` | `low` | CLOSED (doc only, behaviour deliberately unchanged, and I verified the unchangedness) | The soundness paragraph is now scoped to the ARTIFACT and the anchor case is recorded; the single-anchor `..` behaviour is identical on HEAD and HEAD~1. Section 1.4. |
| `R3F-2` | `low` | CLOSED, all THREE sites | No occurrence of "as spelled" or "as you spelled it" survives anywhere outside the review files themselves. Section 1.5. |
| `R3ACC-1` | `low` | CLOSED, and the triage's ADDENDUM site was swept too | Section 1.5. |

### 1.1 `R3A-1`: CLOSED

Fixture `<S>/r4reg-r3a1x`, script `<S>/r4reg-r3a1-exact.sh`. One `alpha` project, Markdown-primary `--source` at `alpha/docs/plans/m.plan.toml` that EXISTS (so this is not the round 2 leak), its own two-record log and its own `m.ledger.md` at their DEFAULT paths, run from a foreign `home`. The triager's own three controls, varying ONLY the `--plan`: C0 supplies none, C1 supplies `<S>/r4reg-r3a1x/beta/docs/plans/s.md` missing, C2 supplies the same path written. Each control captures `next`, `next --json`, `status`, `status --json` and `status --resume` into one file.

```
HEAD C0 vs HEAD C1 (must be identical):            IDENTICAL
HEAD C2 vs PRE-FIX C2 (must be identical):         IDENTICAL
HEAD C1 vs PRE-FIX C1 (the control must differ):   DIFFERS in 29 lines
HEAD C1 vs HEAD C2 (C1 must no longer track C2):   DIFFERS
```

The four corners now read the opposite way from round 3. The baseline CAN show the change: the pre-fix binary's C1 differs from HEAD's C1 in 29 lines, and its own C1 differs from its own C0. The human-readable form (`<S>/r4reg-r3a1.sh`) shows what moved: at HEAD, C1 prints `metrics: 2 records`, `ALPHA resume state.`, `"metrics_absent_reason": null` and `"resume_state_absent_reason": null` exactly as C0 does, while the pre-fix binary printed `metrics: unavailable, ... not under the plan's project root <S>/r4reg-r3a1/beta`, `log-not-this-project` and `ledger-not-this-project` for `alpha`'s OWN artifacts. C2 is unmoved on both binaries: `log-not-this-project`, `ledger-not-this-project` and `nothing to resume`, so the divergent two-anchor pairing that accepted cost (iv) records is untouched and the narrowing is not a general loosening.

The typo is still reported. C1 at HEAD still prints `note: --plan <S>/r4reg-r3a1/beta/docs/plans/s.md does not exist` on stderr, so `Q-55-emptyroot`'s Fail-loudly half survives the narrowing.

GENERALISED BEYOND THE ONE FIXTURE, because a closure measured on one layout is bounded by that layout (`<S>/r4reg-r3a1-general.sh`). Adding a missing `--plan` beside an on-disk `--source` leaves every surface byte-identical to the no-`--plan` run in the conventional layout, in a FLAT project root with no `docs/plans` at all, and in a project VENDORED under another project's `docs/plans`. All three also come out identical to the PRE-INCREMENT binary once the three additive JSON fields are accounted for (see section 3.3).

ONE PRECISION ON THE SCOPE OF THE CLOSURE, measured rather than assumed. With the ROLES SWAPPED (an on-disk `--plan` and a MISSING `--source` elsewhere) C1 does NOT equal C0, and that is not `R3A-1`'s mirror image. `resolve_metrics_path` and `default_ledger_path` are `--source`-first, so a supplied `--source` moves the DEFAULT log and ledger to its own project whether or not it exists. The pre-increment binary does the same thing on the same command line (`<S>/r4reg-swap.sh`: the derived task changes from `p` to `s` on both binaries, and `main` prints `metrics: no log found` with no resume block where HEAD prints `log-not-this-project`). Same withholding, more specific reason, decided by inc1's anchor order. Not a regression and not a residual of `R3A-1`.

### 1.2 `R3F-1`: CLOSED

MUTATION APPLIED: `src/main.rs:1153`, the `note_missing_anchors(&args.source, &args.plan);` call in `run_status`, replaced with a comment.

```
$ cargo test
failures:
    an_anchor_that_cannot_be_checked_is_not_reported_as_missing
    an_anchor_that_does_not_exist_still_supplies_a_root
test result: FAILED. 16 passed; 2 failed
```

Red, on two tests rather than one. Reverted; `git status --short` empty. The two `stderr` assertions the fix added at `tests/unsafe_pairings_are_refused_and_omitted.rs:756` and `:767` are what pins the `status` and `status --resume` slices; before them this mutation left the whole suite green.

### 1.3 `R3A-2`: CLOSED

BEHAVIOUR, `<S>/r4reg-r3a2.sh`. A plan on disk at `<S>/r4reg-r3a2/proj/docs/plans/p.plan.toml` with its parent directory at mode `0o000` (uid 1000, not root, so the mode genuinely hides it: `[ -r "$SRC" ]` answers no).

```
HEAD    : note: --source .../p.plan.toml could not be checked: Permission denied (os error 13)
PRE-FIX : note: --source .../p.plan.toml does not exist
(both exit 0; the anchor is on disk afterwards)
```

The pre-fix line states a falsehood about the filesystem; HEAD's does not.

MUTATION APPLIED: `src/main.rs:1120-1129`, the `match path.try_exists()` reverted to `anchor.as_ref().filter(|path| !path.exists())`. `cargo test` -> 1 failed, `an_anchor_that_cannot_be_checked_is_not_reported_as_missing`. Reverted; tree clean.

### 1.4 `R3A-3`: CLOSED as prescribed (doc comment), behaviour verified unchanged

The soundness paragraph at `src/main.rs:1408-1426` now scopes the "no readable file hides behind a literal `..`" argument to the ARTIFACT ("Every term of that argument is about a path that gets opened, which is why it does not extend past the artifact"), states why it does not carry for an anchor, and records the residual with its bound. That is the prescribed minimum.

BEHAVIOUR UNCHANGED, verified rather than assumed (`<S>/r4reg-r3a3.sh`), because a doc-only fix that silently moved behaviour would be worse than the finding:

```
                                              HEAD                 PRE-FIX(HEAD~1)
A single ghost-.. anchor, explicit own log :  metrics: unavailable  metrics: unavailable
B same file spelled plainly (control)      :  metrics: 1 records    metrics: 1 records
```

Identical on both binaries, so the recorded residual is exactly as recorded and nothing else moved with it. (The `..` residual itself is out of scope and not re-raised.)

THE DOC COMMENT'S OWN MEASURED CLAIM IS TRUE, and it needed a surface that actually reaches `resume_roots` to test, which the naive fixture does not (`<S>/r4reg-ghostbeside.sh`). Claim, at `src/main.rs:1424-1426`: "beside an anchor on disk the ghost anchor now contributes no root and the log is read, while the single-anchor spelling above is unchanged by it."

```
                                        HEAD                 PRE-FIX(HEAD~1)
C0 --source (md-primary, on disk) alone  metrics: 2 records   metrics: 2 records
C1 --source + GHOST --plan               metrics: 2 records   metrics: unavailable, ...
C0 status --resume                       REAL resume state.   REAL resume state.
C1 status --resume + GHOST --plan        REAL resume state.   the ledger ...; nothing to resume
```

Both halves hold.

### 1.5 `R3F-2` and `R3ACC-1`: CLOSED at all five sites, plus the addendum

`grep -rn "as you spelled it\|path as spelled\|as spelled it" --include=*.md --include=*.rs .` returns only the round 3 review files, which are the historical record and not the artifact. The three `R3F-2` sites (`README.md:236`, `CHANGELOG.md:23`, `tests/unsafe_pairings_are_refused_and_omitted.rs:663`) all now read "the path itself, resolved as far as the filesystem allows", which matches `resolve_for_containment`'s actual algorithm.

`R3ACC-1`'s site, `canonical_project_root` at `src/main.rs:1320-1324`, now scopes its parenthetical to its own return ("THIS FUNCTION contributes no root and `checked_plan_root` is `None`. What happens then is the caller's decision and not a claim this function can make").

The triage's ADDENDUM site, `note_missing_anchors`'s doc comment at `src/main.rs:1105-1109`, was swept too: "whether or not it exists" is gone and the three-way rule is stated correctly. `grep -rn "whether or not it exists"` over `src/` and the docs returns nothing.

TWO SITES OF THIS SAME CLASS WERE NOT SWEPT. See `R4R-1`.

## 2. THE PRIOR FOURTEEN FINDINGS

Round 1's nine (`ADV-1`, `ADV-2`, `EVI-1` to `EVI-6`, `TRI-1`) and round 2's five (`R2A-1`/`FV-1` as `G-EMPTYROOT`, `R2C-1`, `FV-2`, `R2C-3`, `R2C-4`).

| Id | Group | Still closed | How verified |
| --- | --- | --- | --- |
| `R2A-1` / `FV-1` | G-EMPTYROOT | YES, and this was the single most important check | Section 3.1. Behaviour measured on four binaries; guard mutation MA reds its test. |
| `ADV-1` | G-ROOT | YES | Section 3.2. Thirteen-configuration surface scan against the pre-increment binary; guard mutation MC reds two tests. |
| `ADV-2` | G-SLOT | out of scope (human-closed) | not re-examined. |
| `EVI-1` | G-COV-PREDICATE | YES | `a_symlinked_log_leaf_outside_the_root_is_refused_and_omitted` present at `tests/...:539` and green; killed by M1 and M29. |
| `EVI-2` | G-COV-PREDICATE | YES | `the_refusal_is_scoped_to_the_validator` present at `tests/...:388` and green; killed by M1. |
| `EVI-3` | G-COV-VOCAB | YES | `the_machine_surface_separates_the_causes_on_both_commands` green; killed by MV1 (the `log-absent` conflation) and by MP2. |
| `EVI-4` | G-COV-VOCAB | YES | `the_resume_reasons_separate_and_cover_the_default_ledger` green; killed by MV2, M2, M21 and MP2. |
| `EVI-5` | G-COV-NOTE | YES | `next_withholds_the_whole_loop_on_an_unpairable_log` green; killed by MV1 and MP2. |
| `EVI-6` | G-COV-NOTE | YES | `a_divergent_source_and_plan_pairing_is_refused` green; killed by M2. |
| `TRI-1` | G-DOC | YES | `src/next.rs:193-197`'s justification is now "Assembled by the CALLER, which holds those paths, because the reason enums above are the machine value and carry none". The false clause ("a machine consumer already holds the paths it passed in") is gone. |
| `R2C-1` | G-NOPLANPROSE | YES | `README.md:236` and `CHANGELOG.md:23` both carry "`validate --workflow` has no such fallback and needs none: with no plan resolved there is nothing for it to check, so it refuses on that ground ... without ever reaching containment", which is the qualification asked for, and the round 3 edit did not disturb it. |
| `FV-2` | G-NOPLANPROSE | YES | The same two lines still scope the no-plan-read case by predicate rather than by enumerating one configuration, and the round 3 edit narrowed rather than re-widened them. |
| `R2C-3` | G-TESTNAME | YES | `a_symlinked_log_leaf_outside_the_root_is_refused_and_omitted` at `tests/...:539`; the old name does not appear. |
| `R2C-4` | G-TESTNAME | YES | `tests/...:573-575` reads "No single run elicits a comparable answer from all three: `status` without `--resume` has no ledger field at all ..., so it is never asked the ledger question." |

## 3. THE REGRESSION SECTION

### 3.0 What bounds every negative result below

DIMENSIONS VARIED (`<S>/r4reg-sweep.sh`, 45 cells, and `<S>/r4reg-adv1.sh`, 13 configurations x 2 binaries):

- D1, THE LAYOUT OF THE ANCHOR'S PROJECT, which is the dimension round 2's 216-invocation matrix never varied: conventional `docs/plans`; a project ROOT with no `docs/plans` at all; a same-project directory OFF the convention (`notes/`); a project VENDORED under another project's `docs/plans` (the nearest-wins case); a SYMLINKED `docs/plans`.
- D2, THE SPELLING OF THE ANCHOR: absolute; relative to the directory the command runs in; a `..` that climbs out through `docs/plans`; a bare filename run from inside `docs/plans`.
- D3, WHICH ANCHORS ARE SUPPLIED: `--source` only; `--plan` only; both in ONE project; both in TWO projects; NEITHER. The last is the cell round 3's fix-verification enumeration structurally could not contain, and it is present here.
- D4, WHETHER THE NAMED ANCHOR IS ON DISK: written; missing; and, separately, present but with unreadable metadata.
- D5, THE ARTIFACT: the DEFAULT log and ledger; an explicit one INSIDE the anchor's root; an explicit one OUTSIDE it.
- D6, THE SURFACE: `validate --workflow`, `status`, `status --json`, `next`, `next --json`, `status --resume`.

CONTROLS COMPARED AGAINST, each named with what it is capable of showing:

- For "nothing that worked before now refuses": the PRE-INCREMENT binary `as-main`. It shows 21 of 45 sweep cells moving, so it is demonstrably capable of showing a change.
- For `G-EMPTYROOT`: `as-pre2` (HEAD~2), which reads the foreign artifacts in exactly the rows HEAD refuses them. Demonstrably capable.
- For `ADV-1`: `as-main`, which leaks in 6 of 13 configurations. Demonstrably capable.
- For every round 3 closure: `as-pre3` (HEAD~1), the PRE-FIX BINARY, never a neighbouring command line.

### 3.1 `G-EMPTYROOT` (round 2 `high`) is STILL CLOSED

`<S>/r4reg-emptyroot.sh`. Run from a foreign `home` whose log holds 3 records and whose ledger holds `HOME resume state.`, with an explicit `--metrics <home>/docs/metrics/workflow.jsonl` and an explicit `--ledger-fragment <home>/docs/plans/p.ledger.md`, across five anchor configurations and four binaries. `records3` is whether the foreign 3-record count was read, `homeblock` whether the foreign `## RESUME STATE` was echoed.

```
########## HEAD ##########
  A --source MISSING only     next:records3=n homeblock=n | status:records3=n | resume:homeblock=n | json m="log-not-this-project" l="ledger-not-this-project"
  B --plan MISSING only       next:records3=n homeblock=n | status:records3=n | resume:homeblock=n | json m="log-not-this-project" l="ledger-not-this-project"
  C both anchors MISSING      next:records3=n homeblock=n | status:records3=n | resume:homeblock=n | json m="log-not-this-project" l="ledger-not-this-project"
  D --source EXISTS (control) next:records3=n homeblock=n | status:records3=n | resume:homeblock=n | json m="log-not-this-project" l="ledger-not-this-project"
  E NEITHER anchor supplied   next:records3=Y homeblock=Y | status:records3=Y | resume:homeblock=Y | json m=null l=null
########## PRE-FIX-R2(HEAD~2) ##########
  A --source MISSING only     next:records3=Y homeblock=Y | status:records3=Y | resume:homeblock=Y | json m=null l=null
  B --plan MISSING only       next:records3=Y homeblock=Y | status:records3=Y | resume:homeblock=Y | json m=null l=null
  C both anchors MISSING      next:records3=Y homeblock=Y | status:records3=Y | resume:homeblock=Y | json m=null l=null
  D --source EXISTS (control) next:records3=n homeblock=n | status:records3=n | resume:homeblock=n | json m="log-not-this-project" l="ledger-not-this-project"
  E NEITHER anchor supplied   next:records3=Y homeblock=Y | status:records3=Y | resume:homeblock=Y | json m=null l=null
```

HEAD's rows A, B and C are identical to HEAD~1's and refuse on all three surfaces with both `--json` reasons populated. HEAD~2 leaks on exactly those three rows, so the control shows the change. Row E is the DECIDED neither-anchor case and is identical on all four binaries including `as-main`.

The narrowing cannot re-open this, and the reason is structural rather than only measured: `resume_roots` (`src/main.rs:1562-1572`) takes `deciding = if on_disk.is_empty() { &supplied } else { &on_disk }`, and `supplied` is non-empty whenever an anchor was supplied, so `deciding` is non-empty in both branches. A lone missing anchor is still the only anchor and still supplies its root. I also re-checked the erring branch: `try_exists().unwrap_or(true)` puts an unreadable-metadata anchor into `on_disk`, which is also non-empty, so no third path reaches an empty vector.

GUARD: mutation MA at `src/main.rs:1571`, `.map(|anchor| project_root_of_source(&resolve_for_containment(anchor)))` reverted to `.filter_map(|anchor| canonical_project_root(anchor))`. `cargo test` -> 1 failed, `an_anchor_that_does_not_exist_still_supplies_a_root`. Reverted; tree clean.

### 3.2 Round 1's `ADV-1` is STILL CLOSED

`<S>/r4reg-adv1.sh`, thirteen configurations on one fixture, on HEAD and on `as-main`, counting which project's `## RESUME STATE` body each surface echoes.

At HEAD, `FOREIGN-in-next` is `no` in twelve of thirteen configurations. The thirteenth is Q12 (NO anchors supplied, explicit foreign `--ledger-fragment`), where `next` AND `status --resume` both echo it: the two surfaces AGREE, which is the decided neither-anchor case and is identical to `as-main`. `ADV-1`'s shape, `next` echoing a block that `status --resume` refuses, appears nowhere.

The one configuration where the two surfaces split at HEAD is Q10 (TOML-primary `--source` in one project beside a Markdown `--plan` in another): `next` echoes `AWAY`'s OWN block and `status --resume` refuses it. That is accepted cost (iv), it echoes the project's own block rather than a foreign one, and it is the divergence `Q-55-resumepairing` decided.

The control shows the change: `as-main` leaks a foreign block through BOTH surfaces in six of the thirteen (Q2, Q4, Q5, Q7, Q8, Q12).

GUARD: mutation MC at `src/main.rs:1398-1399`, `containment_roots`'s `.map_or_else(|| resume_roots(source, plan), ...)` replaced by `.map_or_else(Vec::new, ...)`, which is `ADV-1`'s original cause. `cargo test` -> 2 failed, `a_surface_that_reads_no_plan_is_supplied_a_root` and `an_anchor_that_does_not_exist_still_supplies_a_root`. Reverted; tree clean.

### 3.3 The full sweep: 45 cells, 21 differing, every difference attributed

`<S>/r4reg-sweep.sh`, HEAD against `as-main`. Every differing cell falls into one of five buckets, and NONE is unattributed:

1. THE STATED BREAK, an explicit `--metrics` or `--ledger-fragment` naming a log outside the anchor's own root: 10 cells (every layout, anchor present and anchor missing). This is the increment's whole point and the CHANGELOG says so.
2. ACCEPTED COST (ii), the symlinked `docs/plans` layout: 5 cells, all `L5`. Pinned by `accepted_cost_two_the_symlinked_layouts_are_pinned`.
3. ACCEPTED COST (iii), a plan off the `docs/plans` convention deriving the root from its own directory: 3 cells, all `L3`. Spec line 271 attributes this to `project_root_of_source`'s fallback.
4. ACCEPTED COST (iv), the divergent two-anchor pairing on `status --resume`: 1 cell.
5. THE ADDITIVE `--json` FIELDS: 2 cells. `metrics_absent_reason`, `resume_state_absent_reason` and `no_active_loop_reason` do not exist at `main` at all, so any cell that populates one differs by construction. Verified additive by diff (`<S>/r4reg-swap.sh`): the pre-existing fields keep their pre-existing values and only new keys appear.

The cells that do NOT differ are the load-bearing half of this result: every conventional-layout run against its OWN default log and OWN default ledger, in all four spellings of D2, in all five layouts, with the anchor present or missing, is byte-for-byte what `main` produced. In particular `L1 src ON DISK + MISSING L1B plan` does not differ from `as-main`, which is `R3A-1`'s closure showing up independently in the sweep.

WHAT THIS NEGATIVE RESULT IS BOUNDED BY, stated because round 2's was not. It is bounded by D1 to D6 above and by the `as-main` control. It does NOT cover: Windows path semantics; anchors that are directories rather than files; concurrent modification of the fixture between the two binaries' runs; `validate`'s non-`--workflow` paths; or any surface outside the four.

### 3.4 The structural result: the two tests pull in opposite directions. CONFIRMED, both halves

MUTATION N1, over-applying the narrowing so only on-disk anchors EVER decide (`src/main.rs:1570`, `let deciding = &on_disk;`):

```
$ cargo test
    an_anchor_that_does_not_exist_still_supplies_a_root
test result: FAILED. 17 passed; 1 failed
```

MUTATION N2, reverting the narrowing so every supplied anchor decides again (`src/main.rs:1570`, `let deciding = &supplied;`):

```
$ cargo test
    a_missing_anchor_does_not_overrule_an_anchor_that_exists
test result: FAILED. 17 passed; 1 failed
```

Exactly one failure each, and the failures are DISJOINT: each test catches its own mutation and stays green under the other's. The two together pin an interval rather than a direction, which is the first time in this increment that `resume_roots`'s policy has been pinned on both sides. Both reverted; tree clean after each.

### 3.5 The mutation sample

Twelve mutations beyond N1 and N2, spanning the predicate, the root supply, the reason vocabulary and the precedence rules. ALL CAUGHT. Each was applied with the Edit tool, run under `cargo test`, then reverted by restoring a pristine copy of `src/main.rs`; `git status --short` was empty after each.

| Mutation | Site | Result |
| --- | --- | --- |
| M2, THE CENTRAL ONE: `checked_plan_root` roots on the ANCHOR (`source.as_ref().or(plan.as_ref())`), ignoring `toml_primary`. | `src/main.rs:1364` | CAUGHT. 4 failed: `a_divergent_source_and_plan_pairing_is_refused`, `accepted_costs_three_and_four_are_pinned`, `the_resume_reasons_separate_and_cover_the_default_ledger`, and now `a_missing_anchor_does_not_overrule_an_anchor_that_exists`. The same three as rounds 1, 2 and 3, PLUS one: the new test strengthens the guard on the central mutation rather than merely coexisting with it. |
| M1, `is_outside_root` returns `false` unconditionally. | `src/main.rs:1456` | CAUGHT. 15 of the 18 containment tests failed (14 of 16 at round 3). |
| M29, the false-positive direction, `is_outside_root` returns `true`. | `src/main.rs:1456` | CAUGHT. 8 failed, all in the inc1 file. Unchanged from round 3. |
| M21, `resume_roots` drops the `--plan` anchor. | `src/main.rs:1566-1567` | CAUGHT. 3 failed (2 at round 3): `resume_omits_the_default_ledger_under_a_divergent_pairing`, `accepted_costs_three_and_four_are_pinned`, `a_missing_anchor_does_not_overrule_an_anchor_that_exists`. |
| MA, the round 2 fix reverted. | `src/main.rs:1571` | CAUGHT. 1 failed: `an_anchor_that_does_not_exist_still_supplies_a_root`. |
| MC, `containment_roots` drops the anchor-root fallback (`ADV-1`'s cause). | `src/main.rs:1398-1399` | CAUGHT. 2 failed. |
| MV1, reason vocabulary: `next` reports an unpairable log as `LogAbsent`. | `src/main.rs:1680` | CAUGHT. 5 failed. |
| MV2, reason vocabulary: `next` reports an unpairable ledger as `LedgerAbsent`. | `src/main.rs:1697` | CAUGHT. 3 failed. |
| MP1, precedence: `run_resume` checks ABSENCE before containment. | `src/main.rs:1593-1603` | CAUGHT. 1 failed: `status_omits_only_the_unpairable_part`. |
| MP2, precedence: `run_next` checks EXISTENCE before containment, so an unpairable log that happens to be on disk is read. | `src/main.rs:1676-1686` | CAUGHT. 9 failed. |
| MR3F1, the `note_missing_anchors` call deleted from `run_status`. | `src/main.rs:1153` | CAUGHT. 2 failed. |
| MR3A2, `try_exists` reverted to `Path::exists`. | `src/main.rs:1120-1129` | CAUGHT. 1 failed. |

### 3.6 The NEITHER-ANCHOR case is unchanged

Run from a project root with no `--source` and no `--plan` and no explicit artifact (`<S>/r4reg-adv1.sh`, final block), on all four binaries:

```
  HEAD                 status=[metrics: 3 records] resume=[no ledger at docs/plans/task.ledger.md; nothing to resume] next=[metrics: 3 records]
  PRE-FIX-R3(HEAD~1)   status=[metrics: 3 records] resume=[no ledger at docs/plans/task.ledger.md; nothing to resume] next=[metrics: 3 records]
  PRE-FIX-R2(HEAD~2)   status=[metrics: 3 records] resume=[no ledger at docs/plans/task.ledger.md; nothing to resume] next=[metrics: 3 records]
  PRE-INCREMENT(main)  status=[metrics: 3 records] resume=[no ledger at docs/plans/task.ledger.md; nothing to resume] next=[metrics: 3 records]
```

Byte-identical across all four. No root is derived, the paths stay current-directory-relative, and the ledger default is still the relative `docs/plans/task.ledger.md`. Sweep row E in section 3.1 makes the same point with explicit foreign artifacts: the neither-anchor case reads them at HEAD exactly as at `main`, which is the decided behaviour and not a leak this increment introduced.

### 3.7 No test's MEANING changed across the four commits

`git diff main..HEAD -- src/ tests/ | grep -E "^-" | grep -E "assert|#\[test\]|fn [a-z_]+\("` returns exactly one line, `-fn no_loop_reason(steps: &[StepInfo]) -> String {`, which is product code (the function survives at `src/next.rs:1084` with a typed `NoActiveLoopReason` return). NO assertion and NO test function was removed anywhere in the increment.

The only test DATA line removed in the whole increment is `-  "resume_state": null` in `src/next.rs`'s golden JSON fixture, and the diff shows it re-added as `+  "resume_state": null,` with two new keys following it. Every other removed non-comment line in `src/main.rs` and `src/next.rs` is product code, which I enumerated and read. `tests/unsafe_pairings_are_refused_and_omitted.rs` is 1405 insertions with zero deletions, so it is a new file.

Suite size went 416 (round 3 baseline, per the round 3 fix-verification file) -> 418. Coverage grew by two tests; nothing was reinterpreted.

## 4. NEW FINDINGS

### `R4R-1`: two call sites of `note_missing_anchors` still assert the PRE-NARROWING rule. `low`

The round 3 sweep corrected the five prescribed `G-STALECLAIM` sites and the triage's addendum site, which is `note_missing_anchors`'s own doc comment. It did not correct the two comments that JUSTIFY CALLING that function, and both state the rule the narrowing replaced.

`file:line` 1, `src/main.rs:1150-1152`, in `run_status`:

```rust
	// Before the `--resume` split, so BOTH slices report a typo'd anchor. A missing anchor
	// still supplies a containment root, and the note is the only place the projection says
	// the name behind that root is not on disk.
```

`file:line` 2, `src/main.rs:1628-1629`, in `run_next`:

```rust
	// The same typo'd-anchor note `status` prints, for the same reason: `next` roots
	// containment on an anchor that does not exist rather than falling through with none.
```

Both are false in the configuration the round 3 fix created, and it is the configuration the fix exists FOR. Command (`<S>/r4reg-r3a1.sh`, control C1), run from `<S>/r4reg-r3a1/home`:

```
$ agent-scaffold status --source <S>/r4reg-r3a1/alpha/docs/plans/m.plan.toml \
      --plan <S>/r4reg-r3a1/beta/docs/plans/s.md
note: --plan <S>/r4reg-r3a1/beta/docs/plans/s.md does not exist
metrics: 2 records
```

`alpha`'s own two-record log is read, so the missing `--plan` supplied NO containment root, and `next` on the identical command line prints `metrics: 2 records` and `ALPHA resume state.` for the same reason. Comment 1's "A missing anchor still supplies a containment root" and comment 2's "`next` roots containment on an anchor that does not exist" are both contradicted by the run the comment sits above. `resume_roots` (`src/main.rs:1570`) is what makes them false: `deciding` is `on_disk` whenever any supplied anchor is on disk.

The function's OWN doc comment, at `src/main.rs:1105-1109`, now says the opposite and says it correctly. So the increment ships a function whose contract is stated three times and disagrees with itself twice.

WHY THIS IS NOT ABSORBED. It is not `R3F-2` (a different claim, "as spelled", at different files). It is not `R3ACC-1` (a different function's return-value scope). It is not the triage addendum's site (that one was fixed). It is the same CLASS as all three, at two sites nobody enumerated, which is the pattern section 5 of the round 3 triage recorded and asked round 4 to stop repeating.

SEVERITY `low`: comment-only, no behavioural consequence, and the same rating the project gave `R3ACC-1` and `R3A-3`'s doc half. FIX: one clause each, in one sweep with `R4R-2`.

### `R4R-2`: `resume_roots`'s justification for the `try_exists` error direction is falsified by its own two-anchor case. `low`

`file:line`: `src/main.rs:1557-1560`.

```rust
/// AN ANCHOR WHOSE EXISTENCE CANNOT BE DETERMINED COUNTS AS EXISTING (`try_exists` erring,
/// which is a directory above it the caller cannot traverse, not an absence). Guessing the
/// other way would drop its root on the strength of an error, and of the two directions only
/// this one can add a root rather than remove one.
```

The last clause is false. Because an anchor that cannot be checked is placed in `on_disk`, it makes `on_disk` non-empty, which REMOVES every other supplied anchor's root from `deciding`. That is a removal, not an addition, and it happens in the configuration "one anchor uncheckable, the other genuinely missing".

Measured, `<S>/r4reg-errdir.sh`. `proj/docs/plans` is set to mode `0o000`, so `--source proj/docs/plans/p.plan.toml` is uncheckable; `--plan <S>/r4reg-errdir/beta/docs/plans/s.md` is in another project and genuinely missing; the explicit `--metrics` is `proj`'s OWN four-record log. Run from `<S>/r4reg-errdir/home`:

```
CASE B: --source uncheckable + --plan MISSING in another project, explicit own log
    note: --source .../proj/docs/plans/p.plan.toml could not be checked: ...
    note: --plan .../beta/docs/plans/s.md does not exist
    metrics: 4 records
CASE D (the same shape with the uncheckable anchor merely MISSING instead)
    metrics: unavailable, the round log ... is not under the plan's project root ...
```

B and D differ only in whether the first anchor is uncheckable or absent. Counting it as existing turned a refusal into a read, which it could only do by REMOVING `beta`'s root from the vector. The claim "only this one can add a root rather than remove one" therefore does not hold, and the paragraph is the stated justification for the design choice rather than incidental prose.

THE BEHAVIOUR ITSELF IS FINE and I am not asking for it to change: case B reads the anchor's OWN log, which is what `Q-55-emptyroot` wanted and what `Q-55-anchorveto` decided. The defect is that the reason given for it is not the reason it works. FIX: one clause, saying that an uncheckable anchor is treated as on-disk so it decides rather than defers, and that in the one configuration where the other anchor is missing this removes that anchor's root, which is the same deference the narrowing already applies.

SEVERITY `low`: doc-only, narrow configuration, safe direction.

## 5. WHAT I VERIFIED AS GENUINELY CLOSED, STATED PLAINLY

All six round 3 findings are closed, each against the PRE-FIX BINARY rather than a neighbouring configuration. All fourteen prior findings that were in scope remain closed, including both `high`s: `G-EMPTYROOT` refuses on all three surfaces in every anchor configuration where an anchor was supplied, and `ADV-1`'s split-surface leak appears in none of thirteen configurations. `resume_roots`'s fourth rewrite is, for the first time, pinned on BOTH sides by two tests that catch each other's mutations. Twelve further mutations across the predicate, the root supply, the reason vocabulary and the precedence rules are all caught, three of them by MORE tests than at round 3. A 45-cell sweep against the pre-increment binary produced 21 differences and no unattributed one. No test's meaning changed anywhere in the increment.

The two findings above are both `low`, both comment-only, both in one sweep, and neither touches behaviour.

## 6. WORKTREE STATE

Every mutation was reverted by restoring pristine copies of `src/main.rs`, `src/next.rs` and `tests/unsafe_pairings_are_refused_and_omitted.rs` taken before any edit; `git status --short` was checked empty after each. Final state: `cargo test` -> 418 passed, 0 failed, and `git status --short` shows only this findings file.
