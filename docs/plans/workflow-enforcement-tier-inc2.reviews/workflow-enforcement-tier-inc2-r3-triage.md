# `workflow-enforcement-tier-inc2`, work review ROUND 3, ISOLATED TRIAGE

ARTIFACT. `git diff main..HEAD` at commit `3cb7f45` ("fix: root containment on an anchor that does not exist"), triaged in the worktree `.claude/worktrees/triage-inc2-r3`. The three reviewers name the same commit as `a7e05c3`; the trees are identical, the hash differs because each review worktree carries its own history. The round 2 fix alone is `git diff HEAD~1..HEAD` (`HEAD~1` is `63ce26d`). Specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`.

SOURCE FILES TRIAGED.

- `workflow-enforcement-tier-inc2-r3-reviewer-fixverify.md` (R3F-1 `medium`, R3F-2 `low`).
- `workflow-enforcement-tier-inc2-r3-reviewer-adversarial.md` (R3A-1 `medium`, R3A-2 `low`, R3A-3 `low`).
- `workflow-enforcement-tier-inc2-r3-reviewer-acceptance.md` (R3ACC-1 `low`).

Rounds 1 and 2 reviewer files and both triages in the same directory were read first, and nothing settled there is re-litigated below.

METHOD. Every verdict carries a command I ran and output I observed. Two binaries were used: `target/release/agent-scaffold` built at `3cb7f45`, and a pre-fix binary built by reverting the single fix line at `src/main.rs:1516` and copied out before reverting the mutation. Three mutations were applied one at a time, each measured with a full `cargo test` and reverted with `git checkout -- src/main.rs` and a `git status --short` check. All fixtures were built by hand under `<S>/tri3-*` (`<S>` abbreviates the session scratchpad), never inside a repository, and every project fixture is a set of TOP-LEVEL SIBLINGS except the one nested arrangement built deliberately as the in-root discriminating control.

BASELINE, measured before any mutation: `cargo test` 416 passed, 0 failed across 9 binaries (378 + 5 + 1 + 1 + 9 + 3 + 16 + 1 + 2); `cargo clippy --all-targets -- -D warnings` clean; `render docs/plans/agent-scaffold.plan.toml --check` reports up to date.

ONE SIGNAL BEFORE THE FINDINGS. There is no `high` and no `critical` in this round's raw set, where round 1 carried a `high` and round 2 carried a `high`. Nothing was dismissed or downgraded from `high` here, so no backstop re-check is owed. The severity ceiling falling for two consecutive rounds while three independent lenses attacked the same mechanism is itself evidence that the mechanism is converging; what has not converged is the POLICY the mechanism implements, which is the subject of section 1.

---

## 1. THE RULING ON THE FIX-VERIFICATION VERSUS ADVERSARIAL CONTRADICTION

### 1.1 The ruling in one paragraph

**NEITHER LENS MISMEASURED. THE CONTRADICTION IS NOT A CONTRADICTION OF FACT, IT IS A DISAGREEMENT ABOUT WHICH CONTROL THE VERDICT IS TAKEN AGAINST, AND THE FIX-VERIFICATION LENS PICKED THE ONE THAT CANNOT SHOW THE CHANGE.** R3A-1 REPLAYS EXACTLY, byte for byte against the reviewer's own script. The fix-verification lens's sentence "no configuration refuses anything the same configuration did not already refuse with the plan present" is ALSO TRUE, and I reproduced it. Both hold at once because they compare the attack to different baselines: fix-verification compares "`--plan` supplied and missing" against "`--plan` supplied and present at that same path", and those two agree; the adversarial lens compares "`--plan` supplied and missing" against "NO `--plan` SUPPLIED AT ALL", and those two disagree. The second baseline is the one an operator who typed a stale path actually had, and it is the cell the six-layout enumeration never contains. **R3A-1 IS VALID.**

### 1.2 The discriminator I built, and its output

One fixture, one `--source` (a Markdown-primary `alpha/docs/plans/m.plan.toml` that EXISTS, so it is not the round 2 leak), alpha's own log and own ledger left to default, three controls, both binaries. `<S>` abbreviates the scratchpad path. Script at `<S>/tri3-disc/disc.sh`.

```
########## POST-FIX (HEAD 3cb7f45) ##########
--- C0: NO --plan supplied at all (the operator's real baseline)
  "metrics": {                     "metrics_absent_reason": null,
  "resume_state": "## RESUME STATE\n\nALPHA PRIVATE RESUME STATE.",
                                   "resume_state_absent_reason": null,
--- C1: --plan supplied, file DOES NOT EXIST (the attack)
note: --plan <S>/tri3-disc/f1/beta/docs/plans/s.md does not exist
  "metrics": null,                 "metrics_absent_reason": "log-not-this-project",
  "resume_state": null,            "resume_state_absent_reason": "ledger-not-this-project",
--- C2: --plan supplied, SAME PATH now WRITTEN (fix-verification's control)
  "metrics": null,                 "metrics_absent_reason": "log-not-this-project",
  "resume_state": null,            "resume_state_absent_reason": "ledger-not-this-project",

########## PRE-FIX (HEAD~1 equivalent) ##########
--- C0: identical to post-fix C0.
--- C1: "metrics": { ... }  "metrics_absent_reason": null
        "resume_state": "## RESUME STATE\n\nALPHA PRIVATE RESUME STATE."
--- C2: identical to post-fix C2.
```

Read the four corners. C1 post-fix equals C2 post-fix, which is exactly the fix-verification lens's claim, TRUE. C1 post-fix differs from C1 pre-fix and from C0, which is exactly the adversarial lens's claim, ALSO TRUE. C0 is the cell that decides it and it is missing from the fix-verification enumeration.

### 1.3 What the six layouts actually varied, and why that matters for the next round

Section 3.3 of the fix-verification file defines each of K1 to K6 by a `--source` AND a `--plan`, and runs each "TWICE, once with the `--plan` absent (the two-root case) and once with the identical path written (the one-root control)". The varied dimension is therefore WHETHER THE NAMED FILE EXISTS, on a command line that always carries a `--plan`. The dimension WHETHER A `--plan` IS SUPPLIED AT ALL is never varied inside that enumeration, so no layout can produce C0 and no layout can show the change.

This was NOT a measurement miss, and calling it one would teach the next round the wrong lesson. The lens's own section 3.2 records the change plainly: "Three configurations where a supplied but missing `--plan` now contributes a SECOND root and so refuses artifacts the other anchor owns." It measured the differential against the pre-fix binary, SAW those three cells move, and then adjudicated them against the plan-present control instead of against the pre-fix behaviour it had just measured. Its K2 row states the outcome outright ("A's own log and ledger are refused in both") and rules it "IDENTICAL to the control".

WHAT A COVERAGE CLAIM IS WORTH HERE, stated for round 4. A coverage claim over N layouts is worth exactly the dimensions the layouts CROSS, and a lens that enumerates a shape defined by a predicate (here "two roots", which REQUIRES both anchors supplied) can never contain the configuration where the predicate is false. When the finding under test is "supplying X made things worse", the control must be "X not supplied", and an enumeration of the space where X IS supplied is structurally incapable of producing it. The acceptance lens hit the same behaviour independently in its PART 3 (a `--metrics` "under `<alpha>`'s own derived root yet still omitted") and read it as the fix working correctly. So the behaviour was seen by all three lenses and classified as a defect by one.

### 1.4 Why the behaviour is a defect and not a decided cost

Three grounds, in the order I weigh them.

1. THE HUMAN'S OWN DECISION TEXT RULED OUT THIS CONSEQUENCE. `Q-55-emptyroot` (`docs/plans/agent-scaffold.ledger.md:403`) declined the "unpairable" option because it "would also omit an artifact legitimately belonging to the anchor's own directory, losing its own log for a run against a plan file not yet written". C1 loses alpha's own log and alpha's own ledger for exactly that population. The consequence the human paid to avoid arrives through the option that was chosen.
2. THE NARROWNESS THE DECISION WAS GRANTED ON IS FALSIFIED. The same decision records "the only invocations whose output changes are ones CURRENTLY LEAKING or currently reading an artifact outside the anchor's project". Pre-fix C1 was doing neither: it read alpha's own log, under the `--source` anchor's own project root, and it leaked nothing. Its output changed.
3. IT IS NOT ABSORBED BY ANY RECORDED BOUND, WHICH I CHECKED RATHER THAN ASSUMED. It is not the in-root bound: the refused artifact is the project's OWN and the direction is refusal, not a foreign read, and the fixture is two top-level siblings with no containment relation. It is not accepted cost (iii) or (iv): specification lines 259 to 261 define both on a `--plan` that EXISTS, and line 271 attributes their shared root cause to `project_root_of_source`'s parent-directory fallback, which `beta/docs/plans/NOSUCH.md` never reaches because it matches the `docs/plans` convention walk. The adversarial lens drew that distinction itself and supplied the same-project `alpha/notes/missing.md` variant separately as the one a triager may rule absorbed; that discrimination is correct and I adopt it.

WHAT THE FINDING IS NOT. It is not a leak and not content injection. The direction is a false REFUSAL at exit 0 with a `note:` naming the missing anchor, which is fail-safe, and it lands next to accepted cost (iv), where the human has already accepted `status --resume` omitting a project's own block under a divergent two-anchor pairing. That is why the disposition question in section 8 is genuinely open, and why I do not rate it above `medium`.

### 1.5 The follow-on: is the surface disagreement a new instance of round 1's ADV-1 class?

**NO. IT IS THE MIRROR IMAGE OF ADV-1, NOT A NEW INSTANCE OF IT, AND THE TWO CANNOT BE THE SAME DEFECT CLASS BECAUSE THEIR CAUSES ARE OPPOSITE.**

Round 1's ADV-1 (`workflow-enforcement-tier-inc2-reviewer-adversarial.md:16`, triaged `high`) was "`next` echoes ANOTHER PROJECT's `## RESUME STATE` block where `status --resume` refuses it", and round 1's triage fixed its cause precisely: "`next` has NO ROOT AT ALL and the predicate never fires, at any distance". That is a surface with TOO FEW roots admitting FOREIGN content.

R3A-1's block E is a surface with ENOUGH roots refusing OWN content. I reproduced it: with a TOML-primary `--source` in alpha and a nonexistent `--plan` in beta, `next` prints alpha's `ACTIVE LOOP` and alpha's own block, `validate --workflow` greens on alpha's own log, and `status --resume` refuses alpha's own ledger naming root `<...>/beta`. Nothing foreign appears anywhere; `next`'s predicate FIRES (root `[alpha]`, the checked TOML plan's) and PASSES. ADV-1's cause is structurally unreachable now: every supplied anchor yields a root, and the neither-anchor case is decided.

IS THE ADVERSARIAL LENS'S STRICTNESS PROOF CONSISTENT WITH ITS OWN R3A-1? YES, and I verified the proof two ways rather than accepting it.

By reading: `containment_roots` (`src/main.rs:1379`) is `[checked_plan_root]` when a plan is read and `resume_roots` otherwise. `canonical_project_root` returns `Some` only for a path that canonicalises, and for a path that exists `resolve_for_containment` IS `fs::canonicalize`, so that root is literally one of the elements `resume_roots` produces for the same anchor. Containment requires the artifact under EVERY root, so a superset of roots is never weaker. `run_next` (`src/main.rs:1638`) and `run_resume` (`src/main.rs:1539`) derive the ledger path from the identical `default_ledger_path(&task, &args.source, &args.plan)` call, so the comparison is on one file.

By measurement, twelve configurations on one fixture (`<S>/tri3-inroot/g.sh`), counting whether each surface echoes a `## RESUME STATE` body:

```
P1  toml src alone                  : next-echo=1 resume-echo=1
P2  md src alone                    : next-echo=1 resume-echo=1
P3  toml src + EXISTING beta plan   : next-echo=1 resume-echo=0
P4  toml src + MISSING beta plan    : next-echo=1 resume-echo=0
P5  md src + EXISTING beta plan     : next-echo=0 resume-echo=0
P6  md src + MISSING beta plan      : next-echo=0 resume-echo=0
P7  MISSING src + EXISTING beta pl  : next-echo=0 resume-echo=0
P8  beta plan alone                 : next-echo=1 resume-echo=1
P9  toml src + explicit beta ledger : next-echo=0 resume-echo=0
P10 md src + explicit beta ledger   : next-echo=0 resume-echo=0
P11 beta plan + explicit alpha led  : next-echo=0 resume-echo=0
P12 no anchors + explicit beta led  : next-echo=1 resume-echo=1
```

NO ROW HAS `resume-echo` GREATER THAN `next-echo`. The strictness runs ONE WAY: `status --resume` is always at least as strict as `next` and `status`, never looser.

IS THAT DIRECTION ACCEPTABLE? YES, and it is already decided rather than merely tolerable. P3 is accepted cost (iv) itself, present pre-fix, on an EXISTING `--plan`; P4 is the same split on a `--plan` that does not exist, which is the new population R3A-1 files. The surface that accepts is rooted on the plan the run actually reads (`Q-55-endproperty`); the surface that refuses reads no plan and roots on both anchors (`Q-55-resumepairing`). Two decided policies, and the divergence between them can only ever produce a refusal on the stricter surface, never an acceptance. A disagreement that can only fail closed is a usability cost, not a safety hole, and the specification already records that cost with a note that its population is the wider one. I re-confirmed accepted cost (iv) directly in acceptance check 19b below: `status --resume` refuses the project's own ledger in BOTH `primary` spellings while `next` projects in the TOML spelling.

---

## 2. VERDICT TABLE

| Raw id | Lens | Valid? | Reviewer severity | My severity | Dedup group | One-line prescription |
| --- | --- | --- | --- | --- | --- | --- |
| R3A-1 | adversarial | VALID | `medium` | `medium` | G-ANCHORROOT | A nonexistent anchor should contribute a veto root only where no supplied anchor exists; `src/main.rs:1509-1518`, authored logic, roughly four lines, OR a human accept-and-record. See section 8. |
| R3F-1 | fix-verification | VALID | `medium` | `medium` | G-NOTECOV | Two `stderr` assertions in the runs that already exist at `tests/unsafe_pairings_are_refused_and_omitted.rs:740` and `:750`; test-only, no product change. |
| R3A-2 | adversarial | VALID | `low` | `low` | G-NOTETRUTH | `Path::try_exists` with a separate error arm at `src/main.rs:1115`; small authored logic, roughly five lines. |
| R3A-3 | adversarial | VALID | `low` | `low` | G-ANCHORROOT | Narrow the soundness paragraph at `src/main.rs:1402-1404` to the ARTIFACT use; doc-comment change. Behavioural half optional, see section 4.4. |
| R3F-2 | fix-verification | VALID | `low` | `low` | G-STALECLAIM | Replace "as you spelled it" at `README.md:236`, `CHANGELOG.md:23` AND `tests/unsafe_pairings_are_refused_and_omitted.rs:663`, which the lens missed; one clause each. |
| R3ACC-1 | acceptance | VALID | `low` | `low` | G-STALECLAIM | Narrow `canonical_project_root`'s first-paragraph parenthetical at `src/main.rs:1308-1310` to this function's own return; one clause. |

DEDUPLICATION NOTES. Six raw findings, six valid, NONE collapse into one another. R3A-1 and R3A-3 are grouped as G-ANCHORROOT because both are consequences of the same new trust ("a root derived from a path with nothing behind it is used as if it were a project root") and a fix pass should hold them in one head; they stay separate findings because their mechanisms differ (an intersection of two roots versus a single root that is not a directory) and their fixes touch different functions. R3F-2 and R3ACC-1 are grouped as G-STALECLAIM because both are text that the fix pass left behind while editing the paragraph around it, and both should be fixed in one sweep; they stay separate because they are different files, different claims, and neither edit implies the other. THE ANSWER TO THE BRIEF'S QUESTION IS THEREFORE: TWO FINDINGS, ONE SWEEP, and R3F-2 has THREE sites rather than the two it names.

---

## 3. THE LARGE POSITIVE RESULTS, SPOT-CHECKED

The brief asked for at least `G-EMPTYROOT`, the central anchor-rooting mutation, and two acceptance checks. All four verified, plus the in-root discriminating control.

### 3.1 G-EMPTYROOT closure and its guard: CONFIRMED

The behaviour: R3A-1's own script, block A versus the pre-fix binary, plus the R3ACC-1 construction in section 4.6, all show a supplied anchor that does not exist now yielding a root and refusing a foreign artifact where the pre-fix binary read it.

The guard: mutation MA, reverting the fix line `src/main.rs:1516` from `.map(|anchor| project_root_of_source(&resolve_for_containment(anchor)))` back to `.filter_map(|anchor| canonical_project_root(anchor))`.

```
cargo test -> 415 passed, 1 failed
---- an_anchor_that_does_not_exist_still_supplies_a_root stdout ----
```

Reverted; `git status --short` empty. The fix-verification lens's row is correct.

### 3.2 The central anchor-rooting mutation M2: CONFIRMED CAUGHT

`checked_plan_root` (`src/main.rs:1351`) rooted on the anchor instead of the selection: `let checked = if toml_primary { source.as_ref() } else { plan.as_ref() }?;` replaced by `let checked = source.as_ref().or(plan.as_ref())?;`.

```
cargo test -> 13 passed, 3 failed (of 16 in the containment file)
---- a_divergent_source_and_plan_pairing_is_refused stdout ----
---- accepted_costs_three_and_four_are_pinned stdout ----
---- the_resume_reasons_separate_and_cover_the_default_ledger stdout ----
```

The same three tests rounds 1, 2 and 3 all report. Reverted; tree clean. The increment's end property is still guarded after the third rewrite of the root supply.

### 3.3 Acceptance check 11: PASS

Fixture built by `scaffold --output-dir <F>/c11 --write --force --principles default` (`ls <F>/c11/docs` prints `plans` only), the single step's `slug` set to `triager-runs-only-on-findings` and its `status` to `complete`, run from the worktree root so the relative `--metrics` resolves to this repository's own log, which genuinely carries a converged round for that slug. That is the sharpened false-pass precondition the specification names.

```
$ agent-scaffold validate --source <F>/c11/docs/plans/TEMPLATE.plan.toml \
      --metrics docs/metrics/workflow.jsonl --workflow
--workflow would join <F>/c11/docs/plans/TEMPLATE.plan.toml against docs/metrics/workflow.jsonl,
which is not under the plan's project root <F>/c11; pass a `--metrics` under that root, run
against the plan's own log, or correct the `--source` and `--plan` pair
exit=1
```

I added a control the acceptance lens did not run, to show the refusal is what produces the non-zero exit rather than something incidental: the same plan against its OWN (absent) log exits 0 with "`--workflow` has a plan source but the metrics log is missing; skipping the workflow check". So without the refusal this configuration falls through, which is the false pass the check exists to catch. Matches specification line 323.

### 3.4 Acceptance check 19b: PASS, with one imprecision in the reviewer's table

Fixture: `<F>/c19b/docs/plans/x.plan.toml` (one `in-progress` step), `<F>/c19b/notes/p.md` (a real Roadmap plus Step Detail), `<F>/c19b/docs/metrics/workflow.jsonl` copied from this repository, and `<F>/c19b/docs/plans/x.ledger.md` seeded with its own block. Exit codes measured directly with no pipeline in the way.

```
primary=markdown  validate=1  status=0  next=0  resume=0
primary=toml      validate=0  status=0  next=0  resume=0
```

The refusal names `<R>/notes/p.md`, `<R>/docs/metrics/workflow.jsonl` and root `<R>/notes`; `status` and `next` omit the metrics half with the same reason; `status --resume` prints "the ledger `<R>/docs/plans/x.ledger.md` is not under the plan's project root `<R>/notes`; nothing to resume" in BOTH spellings, byte-identical, and no line of the block appears. That is the specification's actual requirement ("`status --resume` omits its block in EITHER `primary` spelling") and it holds.

THE IMPRECISION, recorded because a later reader will otherwise carry it forward. The acceptance file's table row for 19b reads "`validate --workflow`, `status`, `next`, `status --resume` in both `primary` spellings" against exits "1 / 0 / 0 / 0". Only `status --resume` gives the same answer in both spellings. In the TOML spelling `validate --workflow` exits 0 and reports invariants hold, correctly, because the checked plan is then the TOML `--source` under `<R>/docs/plans` and the log IS under `<R>`. The file's own narrative scopes the both-spellings claim to `status --resume` and is right; the table row's phrasing is loose. This is a precision note on a review file, not a finding against the increment.

### 3.5 The in-root discriminating control: the adversarial lens's judgement is CORRECT

I rebuilt the G-series control rather than accepting it. One fixture shape, `--source <G>/nested/alpha/docs/plans/../../../q.plan.toml`, run twice with only beta's location changed:

```
NESTED   (beta a sibling of alpha under the derived root)
  metrics: 3 records ; BETA PRIVATE RESUME STATE.
DISJOINT (identical anchor shape, beta outside the derived root)
  metrics: unavailable, the round log <G>/disjointbeta/beta/docs/metrics/workflow.jsonl is not
  under the plan's project root <G>/nested2, so its records cannot be paired with this plan
  the ledger <G>/disjointbeta/beta/docs/plans/b.ledger.md is not under the plan's project root
  <G>/nested2; nothing to resume
```

The disjoint arrangement does not reproduce, so the construction is the in-root bound and the lens was right not to file it. I apply the same test to R3A-1 in section 1.4 and it comes out the other way, which is why one is filed and the other is not.

### 3.6 The two headline claims, judged

"ALL FIVE ROUND 2 FINDINGS CLOSED, ALL NINE ROUND 1 FINDINGS STILL CLOSED, 23 OF 24 MUTATIONS CAUGHT" is SOUND on everything I sampled: MA caught, M2 caught with the same three tests, the one reported survivor reproduced exactly as a survivor, and the baseline is the 416 the lens states. I did not re-run the other twenty-one mutations; the three I sampled all agreed with the file, including the negative one, which is the sample most likely to expose an over-claim.

"ALL EIGHT ACCEPTANCE CHECKS AND ALL 18 SUB-RUNS PASS" is SOUND on the two I re-ran from scratch (11 and 19b), with one loose table phrase noted above and no substantive error. Neither large positive result is a false "everything passes".

---

## 4. PER-FINDING RULINGS

### 4.1 R3A-1: VALID, `medium`, G-ANCHORROOT

WHAT I RAN. The reviewer's own `repro.sh` verbatim (`<S>/tri3-r3a1/repro.sh`), then my own three-control discriminator on both binaries (section 1.2), then the twelve-configuration surface scan (section 1.5), then the in-root discriminating test (section 1.4 ground 3).

WHAT I OBSERVED. Every block of the reproduction replays: block B omits alpha's own log and own ledger, block C reports `log-not-this-project` and `ledger-not-this-project` at exit 0, block D is byte-identical with alpha's own artifacts named explicitly, and block E splits the three surfaces on one command line. The pre-fix binary reads both artifacts on the identical command line.

REASONING. Section 1.4 in full. In short: the behaviour is real, new at this commit, reachable by a stale path with no privileges, not absorbed by the in-root bound or by any accepted cost, and it delivers the specific consequence the human's own decision text declined an option to avoid. Held at `medium` rather than raised: the direction is a refusal, the operator gets a `note:`, and the harm shape already exists as accepted cost (iv) in an adjacent population. Held at `medium` rather than lowered: it is live today, a typo is its trigger, and the machine surface tells an agent that the project's own log is not the project's.

PRESCRIBED FIX. `src/main.rs:1509-1518`, `resume_roots`. AUTHORED LOGIC, roughly four lines: derive roots from the supplied anchors that EXIST, and fall back to the partially-resolved derivation over all supplied anchors only when NONE of them exists. Every closure holds under that rule (a lone missing anchor still yields its root, so G-EMPTYROOT stays closed; a missing anchor beside an existing one defers to the existing one, which is C0), and R3A-1 disappears. THIS IS A POLICY CHANGE AND NOT PURELY A REPAIR, so section 8 routes it rather than prescribing it unilaterally.

### 4.2 R3F-1: VALID, `medium`, G-NOTECOV

WHAT I RAN. Deleted `note_missing_anchors(&args.source, &args.plan);` at `src/main.rs:1142` and ran the full suite.

WHAT I OBSERVED.

```
cargo test -> 378 + 5 + 1 + 1 + 9 + 3 + 16 + 1 + 2 passed, 0 failed
```

The EXACT green baseline, binary for binary. Reverted; `git status --short` empty. I then read the pinning test: `tests/unsafe_pairings_are_refused_and_omitted.rs:704` asserts the note for the `next` run, and the `status --resume` run at `:740` and the `status --json` run at `:750` both BIND `stderr` and assert nothing about it.

REASONING. `medium`, following round 1's own calibration, which the round 1 triage set explicitly: an unpinned load-bearing clause is `medium`, `high` is reserved for a defect a user can hit today. This is the sharper end of that class, for the reason the brief names: the note is the second half of `Q-55-emptyroot`, attached because the chosen option was "the weakest of the three against Fail loudly", and two of its three call sites are unguarded. Two thirds of a decided behaviour rests on nothing. The counter the lens offers itself (the three surfaces share one function, so a partial deletion is unlikely) is real but is an argument about likelihood, and the suite is what converts likelihood into a guarantee.

PRESCRIBED FIX. TEST-ONLY, NO PRODUCT CHANGE. Two assertions inside the existing `an_anchor_that_does_not_exist_still_supplies_a_root`, in the same shape as the one at `tests/unsafe_pairings_are_refused_and_omitted.rs:704`: assert `note: --source <missing> does not exist` on the `stderr` already bound by the `status --resume` run at `:740` and by the `status --json` run at `:750`. Both runs exist; only the assertions are missing.

### 4.3 R3A-2: VALID, `low`, G-NOTETRUTH

WHAT I RAN.

```sh
mkdir -p "$F/proj/docs/plans"; printf '[meta]\ntitle = "x"\nprimary = "markdown"\n' > "$F/proj/docs/plans/p.plan.toml"
chmod 000 "$F/proj/docs/plans"; agent-scaffold next --source "$F/proj/docs/plans/p.plan.toml"; chmod 755 ...; ls ...
```

WHAT I OBSERVED. `note: --source <F>/proj/docs/plans/p.plan.toml does not exist`, and `ls` then prints `p.plan.toml`. The file the note says is absent is on disk.

REASONING. `Path::exists` collapses "not there" and "metadata unreadable" into one `false`; `Path::try_exists` separates them. The population is small (a directory the caller cannot traverse) and the projection degrades in that configuration regardless. It is valid rather than dismissible because the note is the entire Fail-loudly half of the remedy, and a loud line that states a falsehood about the filesystem is worse than a quiet one: the operator is told to go fix a path that is already correct.

PRESCRIBED FIX. `src/main.rs:1115`, inside `note_missing_anchors`. SMALL AUTHORED LOGIC, roughly five lines: replace the `.filter(|path| !path.exists())` guard with a `match path.try_exists()`, printing the existing sentence on `Ok(false)` and a separately-phrased line on `Err`, doing nothing on `Ok(true)`. Pin whichever arm the fix pass adds, since R3F-1 already shows this function's coverage is thin.

### 4.4 R3A-3: VALID, `low`, G-ANCHORROOT. The doc comment IS now over-wide

WHAT I RAN. The reviewer's construction, plus the same command against the pre-fix binary, which the reviewer did not run on THIS fixture.

WHAT I OBSERVED.

```
--- ATTACK  (post-fix): --source <F>/a3/proj/ghost/../q.plan.toml --metrics <F>/a3/proj/docs/metrics/workflow.jsonl
note: --source <F>/a3/proj/ghost/../q.plan.toml does not exist
metrics: unavailable, the round log <F>/a3/proj/docs/metrics/workflow.jsonl is not under the plan's
         project root <F>/a3/proj/ghost/.., so its records cannot be paired with this plan
--- CONTROL (post-fix): the same file, spelled without ghost/..
metrics: 1 records
--- ATTACK  (PRE-FIX binary, identical command)
metrics: 1 records
```

`<F>/a3/proj/ghost/../q.plan.toml` and `<F>/a3/proj/q.plan.toml` name the same file and get opposite verdicts, and the printed root `<F>/a3/proj/ghost/..` is not a directory.

MY JUDGEMENT ON THE DOC COMMENT, which the brief asked for directly. THE ARGUMENT IS NOW OVER-WIDE. `resolve_for_containment`'s comment (`src/main.rs:1402-1404`) justifies keeping a literal `..` with "it can only survive when a directory ABOVE it is missing, and a path whose intermediate directory is missing cannot be opened either, so no readable file hides behind one". Every term in that argument is about a path that gets OPENED. It is sound for the ARTIFACT, which is opened, and the adversarial lens attacked it four ways there and could not falsify it, which I accept. As of `3cb7f45` the same function is applied to an ANCHOR, which is never opened, so the `..` does not fall out at the open; it survives into `project_root_of_source` and becomes the root every artifact is compared against. The comment is the stated guarantee for two uses and only covers one. That is a real defect in the artifact under review, introduced by this commit, not a pre-existing wart.

ONE CORRECTION TO THE REVIEWER'S SEVERITY REASONING, which does not change the severity. The lens rates it `low` partly on "the direction is safe: at `HEAD~1` this same command printed `metrics: 3 records` off another project's log, so the fix moved this case from a leak to a refusal". On a fixture where the named log is the ANCHOR'S OWN project's, the pre-fix binary read it (`metrics: 1 records`, measured above) and the post-fix binary refuses it. So the direction is safe on the foreign-log population and is a false refusal on the own-log population. `low` still holds: the population is a `..` traversing a directory that does not exist, which is narrow, and this is the same harm shape as R3A-1 arriving through a different door.

PRESCRIBED FIX. MINIMUM, DOC-COMMENT CHANGE: narrow the soundness paragraph at `src/main.rs:1402-1404` to say the argument holds for the ARTIFACT because the artifact is opened, and state what the anchor use relies on instead. OPTIONAL BEHAVIOURAL HALF: normalise `..` components in the re-appended remainder. That is authored logic in the most-touched function of the increment and I do not prescribe it at round 3; see section 8.

### 4.5 R3F-2: VALID, `low`, G-STALECLAIM, and it has THREE sites, not two

WHAT I RAN.

```sh
ln -s "$F/f2/alpha" "$F/f2/link-to-alpha"
agent-scaffold status --source "$F/f2/link-to-alpha/docs/plans/nope.plan.toml" --metrics "$F/f2/beta/docs/metrics/workflow.jsonl"
agent-scaffold status --source "$F/f2/link-to-alpha/docs/plans/nope.plan.toml" --metrics "$F/f2/link-to-alpha/docs/metrics/workflow.jsonl"
```

WHAT I OBSERVED. The first prints "not under the plan's project root `<F>/f2/alpha`" for a path spelled `link-to-alpha`. The second prints `metrics: 1 records`, which is the load-bearing half: under a root taken literally "as you spelled it" that own log would canonicalise to `<F>/f2/alpha/...`, fail `starts_with("<F>/f2/link-to-alpha")` and be REFUSED. The clause does not merely read imprecisely, it predicts the opposite verdict from the one the binary gives.

REASONING. Valid as filed, and the code's own comment at `src/main.rs:1495-1497` states the truth, so the two documents disagree with each other as well.

THE THIRD SITE, WHICH ALL THREE LENSES MISSED. `grep -n "as spelled\|as you spelled"` over the tree returns `README.md:236`, `CHANGELOG.md:23` AND `tests/unsafe_pairings_are_refused_and_omitted.rs:663`, the doc comment of the very test that pins the remedy: "the root is derived from the anchor's path as spelled, not withheld". Same false clause, in the file whose job is to say what the behaviour is.

PRESCRIBED FIX. ONE CLAUSE ON EACH OF THREE LINES. `README.md:236` and `CHANGELOG.md:23` as the lens prescribes ("derived from the path itself, resolved as far as the filesystem allows" or similar), and the same correction at `tests/unsafe_pairings_are_refused_and_omitted.rs:663`. The intended contrast (the root comes from the PATH, not from a file that was read) survives all three edits.

### 4.6 R3ACC-1: VALID, `low`, G-STALECLAIM

WHAT I RAN.

```sh
agent-scaffold next --json --source "$F/acc/alpha/docs/plans/nonexistent.plan.toml" --metrics "$F/acc/beta/foreign.jsonl"
```

One anchor, supplied, not on disk; no `--plan` at all, so the comment's own words "that plan does not exist" and "nothing was read" both apply.

WHAT I OBSERVED.

```
note: --source <F>/acc/alpha/docs/plans/nonexistent.plan.toml does not exist
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
```

The containment predicate fired and rejected the foreign log, against a comment that says it does not fire.

REASONING. Valid. `canonical_project_root`'s first-paragraph parenthetical (`src/main.rs:1308-1310`) makes a claim about the SYSTEM ("there is no root and the containment predicate below does not fire") on a condition that now routes through `containment_roots` to `resume_roots`, which supplies roots and fires. The same fix commit ADDED a correct paragraph to this same comment ("A PLAN, NOT AN ANCHOR ...") and left the stale sentence above it, and `containment_roots`'s comment three lines below states the opposite for the same condition. `low` is right: no behaviour is wrong and the class matches R2C-1 and FV-2, both held at `low`.

PRESCRIBED FIX. ONE CLAUSE at `src/main.rs:1308-1310`: narrow the parenthetical to what this FUNCTION does (it contributes no root, so `checked_plan_root` is `None` and the caller decides what to do about it) and drop the claim about the predicate not firing.

ADDENDUM TO FOLD INTO THE SAME SWEEP, not filed as a seventh finding. The adversarial lens recorded but did not file a third instance of this exact class: `note_missing_anchors`'s doc comment (`src/main.rs:1099-1101`) says "the containment rule roots them on the anchor whether or not it exists", where "them" is `status` and `next`. With a TOML-primary `--source` a missing `--plan` contributes NO root to those two surfaces (only to `status --resume`), which R3A-1's block E demonstrates directly. Its judgement that this was too small to file on its own is correct; a sweep that is already correcting two doc-scope claims should correct the third rather than leave it for round 4 to find.

---

## 5. WHAT ALL THREE LENSES MISSED

1. THE THIRD "AS SPELLED" SITE at `tests/unsafe_pairings_are_refused_and_omitted.rs:663`. R3F-2 names two of three, and the one it omits is in the test that pins the remedy. Section 4.5.
2. R3A-3'S DIRECTION IS NOT UNIFORMLY SAFE. On a fixture whose named log belongs to the anchor's OWN project, the pre-fix binary read it and the post-fix binary refuses it. The lens's "the direction is safe" rests on a fixture with a foreign log. Section 4.4.
3. R3A-1 AND R3A-3 SHARE ONE CAUSE, and no lens connected them. Both are the increment now trusting a root derived from a path with nothing behind it: R3A-1 trusts it enough to let it VETO a root derived from a path that does exist, R3A-3 trusts it enough to use a string that is not a directory. A fix pass that treats them as two unrelated items will touch `resume_roots` and `resolve_for_containment` in two passes when one design question governs both.
4. THE FIX-VERIFICATION AND ACCEPTANCE LENSES BOTH BUILT R3A-1'S BEHAVIOUR AND BOTH READ IT AS CORRECT. The acceptance lens's PART 3 construction ("The metrics file genuinely lives under `<alpha>`'s own derived root ... yet is still omitted") IS R3A-1, presented as proof that the fix works. Three lenses saw it; one filed it. That is a calibration fact worth carrying: when a lens's job is to confirm a fix, the fix's own framing is the control it reaches for by default.
5. THE ACCEPTANCE FILE'S 19b TABLE ROW implies `validate --workflow` exits 1 in both `primary` spellings; measured, the TOML spelling exits 0. Its narrative is correctly scoped. Section 3.4. Recorded so it is not carried forward, not filed against the increment.

---

## 6. ROUND OUTCOME

**NEW VALID FINDINGS. THE ROUND IS NOT CLEAN.**

VALID COUNT AFTER DEDUP: **6**, being `medium` 2 (R3A-1, R3F-1) and `low` 4 (R3A-2, R3A-3, R3F-2, R3ACC-1). No `high`, no `critical`, nothing dismissed from `high`, so no backstop re-check is owed.

INVALID: NONE. Every raw finding in this round reproduced against the shipped binary or the shipped text on my own fixtures. That is unusual and is itself worth recording: rounds 1 and 2 each carried findings the triage rejected, and this round carries none.

SHAPE OF THE REMAINING WORK. One test-only fix (two assertions in an existing test), four documentation or doc-comment clauses across five sites, one small `try_exists` change, and ONE POLICY QUESTION that is not the implementer's to answer. Excluding the policy question, the fix pass is the cheap shape: no product logic changes except R3A-2's five lines, and nothing that touches the containment predicate or the root supply.

---

## 7. THE CONVERGENCE PICTURE, STATED PLAINLY AND KEPT OUT OF THE VERDICTS

The clean-round streak stays at 0 of the 2 this `risky` increment needs, and this is round 3 of a cap of 5. Rounds 4 and 5 would both have to be clean. No verdict above was decided by that; each is on the evidence, and I would file the same six against a fresh increment on day one.

What the arithmetic DOES bear on is DISPOSITION, and I state my reading rather than acting on it. The one item with real risk of costing round 4 its cleanliness is R3A-1's prescribed code change: `resume_roots` has now been rewritten twice, and each rewrite produced a finding in the next round (the round 2 leak, then R3A-1). A third rewrite at round 3 of 5 is the highest-variance action available. Everything else on the list is text and assertions, which is the shape this project has clean-round evidence for.

---

## 8. ITEMS NEEDING A HUMAN DECISION

### 8.1 R3A-1's policy: should a SUPPLIED anchor that DOES NOT EXIST veto a root derived from an anchor that DOES?

This is a policy question `Q-55-emptyroot` did not answer. That decision conditions on a `--source` and a `--plan` that both exist (`Q-55-resumepairing`'s framing), and the fix extended the rule to anchors that do not without the decision extending with it. The human's ground for choosing the option that was built explicitly declined "omitting an artifact legitimately belonging to the anchor's own directory", and that is what the built form does when a second anchor is supplied.

- OPTION A, ACCEPT AND RECORD as a fifth accepted cost, in the same form as (i) to (iv), pinned by a test so it cannot silently change. COST: ships a behaviour whose stated ground the decision text rejects, and leaves `log-not-this-project` asserted about a project's own log. BENEFIT: no change to the function that has produced a finding in each of the last two rounds, so round 4 is a text-and-assertions round.
- OPTION B (RECOMMENDED), NARROW `resume_roots`: contribute a partially-resolved root for a nonexistent anchor only when NO supplied anchor exists. Roughly four lines at `src/main.rs:1509-1518`. COST: authored logic in the increment's most sensitive function at round 3 of 5, needing new tests in both directions (a lone missing anchor still yields its root; a missing anchor beside an existing one defers). BENEFIT: closes R3A-1 and restores the narrowness the decision was granted on, while preserving every closure G-EMPTYROOT bought, which I verified by walking each case.
- OPTION C, DEFER to the queued project-identity step. COST: that step owns the LOG half of the in-root bound and does not own the anchor-root policy, so this would be filed against a step that has no mechanism for it; the specification already records that the LEDGER half of the in-root bound has no owner, and this would add a second ownerless item.

I recommend OPTION B, judged against the plan's own principles. "Safe on existing projects" is the principle the decision was argued on, and it is the one Option A gives up for the typo population; "One source of truth" favours the anchor that is actually on disk supplying the root over the one that is not; and "Minimal by default" is satisfied at four lines. The honest counterweight is section 7: Option B is the only item on this round's list that could cost round 4 its cleanliness, and a human who weighs reaching the cap above closing a fail-safe refusal should take Option A with the record written in full.

### 8.2 R3A-2's and R3A-3's behavioural halves: fix or accept?

Both are `low`, both have narrow populations, and both have a cheap partial. R3A-2's fix is five lines and self-contained (`try_exists`), and I would take it. R3A-3 splits: the DOC-COMMENT correction is a clause and should be taken regardless, since it is the stated guarantee for a use it does not cover; the BEHAVIOURAL half (normalising `..` in the re-appended remainder) is authored logic in `resolve_for_containment`, which the containment predicate, the artifact resolution and now the anchor resolution all depend on, and I would NOT take it at round 3. Recommended split: fix R3A-2 in full, fix R3A-3's comment, record R3A-3's behaviour beside whatever is decided in 8.1, since it is the same cause.

---

`git status --short` in this worktree shows only this file. All three mutations were applied one at a time, measured, reverted, and the tree confirmed clean after each.
