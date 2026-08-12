# `validation-constraints` plan review: would the acceptance checks catch a wrong implementation?

Lens: adversarial build. Artifact: `git diff main..HEAD` on `review/vcstep-checks`, which adds the `validation-constraints` `[[step]]`, its sidecar `docs/plans/agent-scaffold.steps/validation-constraints.md`, folds `Q-70` to `decided`, and regenerates the view.

Method: build a reference implementation of the decided direction (iii) and four wrong implementations in scratch copies of the tree, then run the sidecar's inc1 acceptance checks 1 to 8 against each one. No file in the worktree outside this findings file was touched. Every scratch build lives under `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/vcs`.

SEVEN FINDINGS: one `high`, five `medium`, one `low`. No `critical`.

## The mutation battery

The variants, all applied by `scratchpad/vcs/patch.py` to a pristine copy of the worktree at `scratchpad/vcs/src-copy`:

| id | what it is | `src/workflow.rs` change |
| --- | --- | --- |
| `ref` | the correct reporting form | shared predicate `waiver_owns_increment(waiver, step_slug, increment)`, W3 consults it at the covering-waiver match, `w5_problems` widened to take `rounds`, ownership rule replaced by `rounds.iter().any(|round| waiver_owns_increment(waiver, round_step_slug(round), round_increment_id(round)))` |
| `mutE` | THE STEP AXIS DROPPED ON W5'S SIDE | identical to `ref` except W5 passes `&waiver.step` where `ref` passes `round_step_slug(round)`, so the predicate's step comparison degenerates to `waiver.step == waiver.step` |
| `mutA2` | THE RULE DELETED OUTRIGHT | the whole `if waiver.unit == WaiverUnit::Increment` block removed, and the one existing mis-scoping unit test rewritten to the silent-form expectation that acceptance check 4 licenses |
| `mutC` | ONLY THE TWO WAIVERS THIS REPO NEEDS | the lexical rule kept verbatim, guarded by `!increment.ends_with("-fold")` |
| `mutE-pred` | acceptance check 7's own mutation | `mutE` with `waiver_owns_increment`'s increment comparison flipped to `!=` |

The fixtures, all built by `scratchpad/vcs/mkfixture.py`, `mkfx2.py` and `mkmd.py`:

- `fx1`: the live `docs/` tree with both fold tokens declared as `[[step.increment]]`, both owed waivers (`-w5`, `-w6`) written, and `workflow-enforcement-tier` flipped to `complete`. This is acceptance check 3's fixture.
- `fx2`: the live `docs/` tree plus one `[[step.waiver]]` naming `increment = "totally-not-a-step-inc1"`. This is acceptance check 5's fixture.
- `fx3`: a Markdown plan plus a JSONL log carrying a CORRECTLY scoped increment waiver whose increment id (`alpha-fold`) has no `-inc<alnum>` tail. This is acceptance check 6's fixture.
- `fx4`: a Markdown plan plus a JSONL log carrying a MIS-SCOPED increment waiver, `step = "alpha"` for an increment whose two round records both carry `step = "beta"`. NO ACCEPTANCE CHECK IN THE LIST CORRESPONDS TO THIS FIXTURE.

Commands (with `direnv` exported from the worktree and `TMPDIR` outside any repository, per the sidecar's preamble at line 112):

```
CARGO_TARGET_DIR=$S/tgt-<v> cargo test  --manifest-path $S/build-<v>/Cargo.toml
CARGO_TARGET_DIR=$S/tgt-<v> cargo clippy --all-targets --manifest-path $S/build-<v>/Cargo.toml -- -D warnings
bash $S/run-checks.sh $S/bin-<v> <v>
```

`run-checks.sh` runs checks 2, 3, 5, 6 and 8 verbatim plus the unlisted mis-scoping case. Results:

| check | PRE-FIX (HEAD) | `ref` | `mutE` | `mutA2` | `mutC` |
| --- | --- | --- | --- | --- | --- |
| 1 `cargo test` (378 tests, drift guard included) | pass | pass | PASS | PASS | PASS |
| 1 `cargo clippy --all-targets -- -D warnings` | exit 0 | exit 0 | exit 0 | exit 0 | exit 0 |
| 2 live plan exits 0, `workflow invariants hold` | PASS | pass | PASS | PASS | PASS |
| 3 `fx1` exits 0 | FAIL (exit 1, both ownership refusals) | pass | PASS | PASS | PASS |
| 4 the narrowing pinned as a decision | n/a | pass (reporting) | PASS (reporting) | PASS (silent) | PASS (reporting) |
| 5 the message asserts a fact | FAIL | pass | PASS | PASS | FAIL |
| 6 both substrates agree (`fx3`) | FAIL (exit 1) | pass | PASS | PASS | PASS |
| 7 W3 and W5 consult one implementation | n/a | pass | PASS | fail (no predicate) | fail (no predicate) |
| 8 `render --check --strict`, `validate --workflow` | pass | pass | PASS | PASS | PASS |
| UNLISTED: `fx4` mis-scoped waiver refused | refused | refused | ACCEPTED at exit 0 | ACCEPTED at exit 0 | ACCEPTED at exit 0 |

`mutE` passes EVERY ONE of the eight inc1 acceptance checks and accepts a waiver the round log contradicts.

## `PR-B-1` (severity: `high`)

CLAIM. No acceptance check requires that a mis-scoped increment-unit waiver, one whose increment's round records join it to a DIFFERENT step, is still refused. A build that accepts it passes all eight of inc1's checks, so the step can converge on an implementation that removes the ownership rule's entire reason for existing.

THE MUTATION. `mutE`, one argument changed on the W5 side of the join:

```
-					waiver_owns_increment(waiver, round_step_slug(round), round_increment_id(round))
+					waiver_owns_increment(waiver, &waiver.step, round_increment_id(round))
```

The predicate is genuinely shared with W3, so this is not a facade. The step comparison inside it becomes `waiver.step == waiver.step`, and the rule collapses to "does any round record anywhere carry this increment id".

EVIDENCE, each check run against `scratchpad/vcs/bin-mutE`:

- CHECK 1. `cargo test`: `test result: ok. 378 passed; 0 failed`, byte-identical to `ref`. `cargo clippy --all-targets -- -D warnings`: exit 0. The pre-existing mis-scoping test `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment` (`src/workflow.rs:1437-1458`) does NOT catch it, because its fixture carries NO round records, so after the fix it exercises the unobserved case rather than mis-scoping. That is the vacuity: the one test the repository already had for this rule stops testing the rule the moment the rule becomes structural.
- CHECK 2. Live plan, exit 0, `workflow invariants hold`.
- CHECK 3. `fx1`, exit 0, `workflow invariants hold`, against a pre-fix exit 1 quoting both ownership refusals. The red half is real and `mutE` turns it green.
- CHECK 4. Reporting form. `mutE` emits the same message as `ref` for an increment with no round records: "TOML waiver `probe-w9`: increment waiver names step `workflow-enforcement-tier` but no round record joins increment `totally-not-a-step-inc1` to that step". A test asserting that message passes on both.
- CHECK 5. `fx2`, no derived step in the message, line-start anchored control `LC_ALL=C grep -c '^slug = "totally-not-a-step"$'` returns 0.
- CHECK 6. `fx3`, exit 0, against a pre-fix exit 1. `mutE` accepts the correctly scoped Markdown/JSONL waiver exactly as `ref` does.
- CHECK 7. THE ONE CHECK BUILT TO CATCH THIS CLASS, AND IT DOES NOT. Mutating the shared predicate (`mutE-pred`, `== Some(increment)` flipped to `!= Some(increment)`) reds ten tests, both W3 and W5: `a_short_streak_increment_with_a_covering_increment_waiver_passes` and `a_bare_slug_increment_waiver_exempts_a_short_streak` on the W3 side, `w5_passes_a_record_backed_waiver_with_a_matching_escalation`, `w5_flags_a_record_backed_waiver_with_no_matching_escalation`, `w5_flags_a_record_backed_waiver_citing_an_unrelated_escalation`, `w5_flags_a_record_backed_waiver_whose_escalation_resumed_not_decided` and `w5_accepts_the_three_valid_reason_tier_pairings` on the W5 side. Check 7's demonstration therefore SUCCEEDS on `mutE`. It pins that the predicate is shared, not that W5 quantifies over the right thing, and the step's defect is entirely in the quantification.
- CHECK 8. `render --check --strict` and `validate --workflow` both exit 0.
- THE STATE NO CHECK EXAMINES. `bin-mutE validate --plan $S/fx4/docs/plans/fixture.md --workflow` prints "workflow invariants hold" at exit 0 over a log in which `beta-fold`'s two round records both carry `step = "beta"` while the waiver claims it for step `alpha`. `ref` refuses the same tree: "round log line 5: increment waiver names step `alpha` but no round record joins increment `beta-fold` to that step". The PRE-FIX binary also refuses it. So `mutE` is a REGRESSION against the shipped tool that every acceptance check calls a pass.

WHY IT MATTERS HERE SPECIFICALLY. The sidecar's risk classification (line 96) says inc1's failure mode is "a confident wrong answer in either direction: a false green admits a waiver that grants an exemption nothing evidences". Checks 2, 3, 5 and 6 all pin the green direction (something that used to be refused is now accepted) and check 4 pins the unobserved boundary; NOTHING pins the red direction on an observed contradiction. A step that exists to remove a false green from a validator therefore has an acceptance list that a false-green build passes.

REMEDY (stated as the missing check, not as a direction): add a check of the form "an increment-unit waiver whose increment's round records join it to a different step is REPORTED, on both substrates, and the round report shows the fixture and the message", with the red half being the mutation above rather than the pre-fix binary (the pre-fix binary refuses this case too, by accident of the lexical rule, so it cannot supply the red).

## `PR-B-2` (severity: `medium`)

CLAIM. Acceptance check 4's SILENT branch is satisfied, assertion for assertion, by deleting the ownership rule outright. Its three obligations are all true of a build that has no rule at all, so the check cannot distinguish "silent on the unobserved case" from "silent always".

EVIDENCE. Check 4's silent branch (sidecar line 117) requires that such a waiver "PASSES W5 and the test asserts that `src/plan/source.rs`'s membership check still catches the undeclared case, plus the residual that a DECLARED and never-logged increment passes". `mutA2` deletes the block at `src/workflow.rs:562-574` entirely and rewrites the one existing test to `assert!(problems.is_empty())`. Measured:

- `cargo test`: `test result: ok. 378 passed; 0 failed`. `cargo clippy --all-targets -- -D warnings`: exit 0.
- The membership check still fires on `fx2`: "waiver `probe-w9` on step `workflow-enforcement-tier` names increment `totally-not-a-step-inc1`, which is not one of the step's increments", from `src/plan/source.rs:807-811`. Obligation two: satisfied.
- A declared-and-never-logged increment passes. Obligation three: satisfied trivially.
- Checks 2, 3, 5, 6 and 8 all PASS (table above). Check 3 goes green because deleting the rule is by itself a complete unblocking, which the `Q-70` item already measured (`docs/plans/agent-scaffold.plan.toml:1933`, direction (iv)'s "with that one rule disabled in a scratch build of the same tree, the same fixture returns `workflow invariants hold` at exit 0").

Only check 7 stands between total deletion and a green step, and check 7 is a procedural demonstration the implementer performs on itself. Under the silent form there is no obligation anywhere in the list to emit a refusal at all, so the implementer has nothing to point check 7 at except an "accepts" test.

This is `PR-B-1`'s finding reached from the other side: the same missing observed-contradiction check would close both, because under the silent form that case is the ONLY case in which the rule fires.

## `PR-B-3` (severity: `medium`)

CLAIM. No acceptance check verifies that the shipped rule text matches the shipped behaviour. All three drift-guarded prose copies can keep asserting the retired lexical rule and every check in the list passes, including check 1's explicitly named drift guard.

EVIDENCE. The clause lives in three files:

```
grep -rn "must own its .increment." pack/instrument.md AGENTS.md .agents/AGENTS.reference.md
```

returns `pack/instrument.md:11`, `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`, each saying "an `increment`-unit waiver's `step` must own its `increment` (the increment's leading slug equals the step)". Every one of my four variants changed the behaviour and changed NONE of those three files, and `cargo test` reported 378 passed in each, `the_committed_scaffold_matches_a_fresh_render` included. The guard compares the committed generated files against a fresh render of `pack/`; it cannot see `src/`, so it detects a PARTIAL prose edit and never detects a MISSING one.

Check 1 names that guard ("all clean, including the drift guard `the_committed_scaffold_matches_a_fresh_render`", line 114), which reads as though it covers the rule text. It does not. The Documentation impact section (line 134) correctly identifies the three files as owed work, but owed work with no check is exactly the class the sidecar's own risk note calls this project's worst measured one ("this project's calibration on prose that states an enforcement boundary is its worst measured class", line 96), and it is text that ships into every scaffolded project.

REMEDY: a check that reads the shipped clause and asserts it states the observed relation, for example a fixed-string grep asserting the retired wording "the increment's leading slug equals the step" is absent from all three copies, run as a command with an expected exit code like the rest of the list.

## `PR-B-4` (severity: `medium`)

CLAIM. `inc6` acquires a dependency on a choice `inc1` makes, and the step does not record it. `inc1` makes `w5_problems` a consumer of the `rounds` slice; `inc6` filters that same slice. Under `inc1`'s default reporting form, `inc6`'s filter therefore gains the power to turn every increment-unit waiver red, which the step's `inc6` limitation list does not mention.

EVIDENCE. `run_checks` (`src/workflow.rs:206-221`) hands `rounds` to `w3_problems` today and, after `inc1`, to `w5_problems` as well. `inc6` is described as "Project identity on `Round` and on `[meta]` with the join filtered in `check_workflow_toml`" (sidecar line 70), and `check_workflow_toml` (`src/workflow.rs:180-195`) is precisely where `metrics::parse_rounds(log_contents)` is passed into `run_checks`. The three recorded limitations at line 70 are that the mechanism is opt-in, that a filter inside `run_checks` does not cover `next`, and that `decisions` and `escalations` are read unfiltered. NONE of them names W5.

The concrete hazard follows from the first limitation combined with `inc1`'s default: identity is opt-in, so on a merged log the plan's own pre-migration rounds carry no `project` field. A filter that keeps only rounds whose `project` matches the plan drops them, and under the reporting form W5 then reports EVERY increment-unit waiver in the plan, because no surviving round joins any waived increment. That is a false red on a correct plan, produced by two increments that the step says are independent apart from ordering.

This is a boundary defect, not a direction defect: the sidecar's line 72 argues `inc6` is last because of the one-deliberate-edit constraint, and separately argues at line 43 that direction (iii) "ADDS NO FIELD TO ANY STRUCT" and so is not bound by that constraint. Both remain true. What is missing is that `inc1` widens the set of checks `inc6`'s filter governs, so `inc6`'s entry gate (line 72) and its review question (line 70, "does an identity that is absent change nothing") must cover W5 as well as W3 and `next`.

## `PR-B-5` (severity: `medium`)

CLAIM. Check 6 pins only ACCEPTANCE on the Markdown and JSONL substrate, and that is the one substrate on which the step's own `inc2` argument says the mis-scoped state is authorable at all. So the substrate where the rule genuinely has work to do gets no red case.

EVIDENCE. Check 6 (line 119) reads in full: "Run the same shape on a Markdown plan plus a JSONL `type:"waiver"` record. The pre-fix binary refuses a correctly-scoped waiver there and the fixed binary accepts it, so the fix is not TOML-only." Both halves are about acceptance. Measured on `fx3`: pre-fix exit 1, `ref` exit 0, `mutE` exit 0, `mutA2` exit 0, `mutC` exit 0. Every variant including total deletion passes it.

The `inc2` entry (line 66) states why this matters: on the TOML path a waiver "inherits its `step` from the containing `[[step]]`" and the typed struct "is `deny_unknown_fields` with no `step` field at all", so the rule "cannot fire there"; the JSONL substrate is where "`step` is a free string and the state IS authorable". `fx4` is that authorable state, and it is exactly the fixture no check names. A red case on this substrate costs nothing extra: `fx4` is `fx3` with a second step and one changed field.

## `PR-B-6` (severity: `medium`)

CLAIM. The Acceptance check section opens "Every claim below is a command with an expected exit code, so a round is settled by running it rather than by reading the diff" (line 110). That is false of items 9, 11, 12 and 13, and item 11 contains a criterion that cannot be settled by any command.

EVIDENCE, quoting the items:

- Item 9 (line 122): "a reader of the rule's documentation can predict, for each substrate, whether the rule can fire". A property of a reader, not a command.
- Item 11 (line 124): "its output is proportionate rather than a verbatim ledger dump". "Proportionate" has no threshold, so two reviewers can reach opposite verdicts with no way to settle between them. The other two clauses in item 11 are checkable; this one is not.
- Item 12 (line 125): "state the treatment taken, and show that the answer `next` gives about which step is next follows from the structured source rather than from ledger prose". "Follows from" is an argument, not an exit code.
- Item 13 (line 126): "the round report states which surfaces the filter placement does and does not cover". A reporting obligation, not a command.

The section already has the right escape: it says "THE LIST IS A FLOOR AND NOT A TOTAL". The defect is the opening sentence's universal claim, which a reviewer will take as licence to treat the list as mechanically settled when four of thirteen items are not. Given that the step's whole subject is a check whose green cannot be trusted, an acceptance section that overstates its own mechanisation is the same failure one level up.

## `PR-B-7` (severity: `low`)

CLAIM. Check 2 is labelled "the no-regression check and the migration proof" (line 115). It is a correct no-regression check and it is not a proof of anything about the migration, because it is green on the UNMODIFIED pre-fix binary.

EVIDENCE. `target/debug/agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow` at HEAD exits 0 with "workflow invariants hold" (measured; it is the first row of the table above). The same command exits 0 on `ref`, `mutE`, `mutA2` and `mutC`. A check that is green before the change, after the change, and after the change is deleted has no red half and distinguishes nothing. The `Q-66` obligation the section itself invokes at line 110 ("a behavioural claim owes a demonstration that is RED against the pre-fix build") is not met by this item and is not claimed to be, so the fix is one word: drop "and the migration proof", or state that its value is only that the population it covers is non-empty.

## What I could not fault

Recorded so the next round does not re-derive it.

- CHECK 3 HAS A GENUINE RED HALF AND IT REPRODUCES EXACTLY AS DESCRIBED. On `fx1` the pre-fix binary exits 1 with precisely two problems, both ownership refusals, and nothing else: "TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`" and the same for `-w6`. The sidecar's claim at line 49 that declaring plus fixing together return exit 0 is confirmed against `ref`.
- CHECK 5'S CONTROL-GREP WARNING IS CORRECT AND LOAD-BEARING. `LC_ALL=C grep -c '^slug = "totally-not-a-step"$'` over `fx2`'s plan returns 0 while the unanchored form returns a non-zero count whose hits are the `Q-70` item's own quotations. A reviewer using the unanchored form would conclude the step exists.
- CHECK 5 IS THE ONLY CHECK THAT CATCHES `mutC`. The allowlist variant (`!increment.ends_with("-fold")`) passes the full 378-test suite, clippy, and checks 2, 3, 4, 6 and 8, and is caught by check 5 alone, which reports "increment `totally-not-a-step-inc1` belongs to step `totally-not-a-step`". Check 5 is doing real work.
- THE SIDECAR'S CLAIM ABOUT THE MEASURED EDIT SURFACE (line 59) IS CORRECT AND I CONFIRMED THE FIGURE. Under the reporting form, five existing W5 unit tests need a round-log fixture rather than a mechanical empty slice: `w5_flags_a_record_backed_waiver_with_no_matching_escalation`, `w5_passes_a_record_backed_waiver_with_a_matching_escalation`, `w5_accepts_the_three_valid_reason_tier_pairings`, `w5_flags_a_record_backed_waiver_whose_escalation_resumed_not_decided` and `w5_flags_a_record_backed_waiver_citing_an_unrelated_escalation`. Under the silent form, none of them does. The sidecar predicts exactly this asymmetry and it holds.
- CHECK 2 IS TRUE OF THE LIVE PLAN UNDER THE REPORTING FORM. `bin-ref validate --source <live plan> --workflow` exits 0 with "workflow invariants hold", so the narrowing costs no edit to any committed waiver and no edit to the append-only log, as line 55 claims. I re-established it rather than relying on the explorer's measurement, as line 55 instructs.
- THE INCREMENT BOUNDARIES ARE OTHERWISE BUILDABLE. inc1 is satisfiable without touching anything inc2 to inc5 need: inc2's subject is the sibling rule at `src/workflow.rs:553`, which no variant of inc1 had to touch, and inc3 to inc5 touch `src/main.rs`, `src/next.rs` and plan content. `PR-B-4` is the one exception I found, and it runs the other way (inc6 depending on inc1).

## Gates run in the worktree

All at `review/vcstep-checks`, `TMPDIR` outside any repository.

- `cargo test`: green, 378 unit tests plus every integration binary, 0 failed.
- `cargo clippy --all-targets -- -D warnings`: exit 0.
- `agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow`: exit 0, "316 records, valid", "96 steps, 70 questions, valid", "workflow invariants hold".
- `agent-scaffold render docs/plans/agent-scaffold.plan.toml --check --strict`: exit 0.
- `git status --porcelain`: clean apart from this file.
