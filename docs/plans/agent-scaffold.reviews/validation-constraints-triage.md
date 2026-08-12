# `validation-constraints` plan review, triage

Artifact: `git diff main..HEAD` on `triage/vcstep` (commit `0402aeb`), the new `[[step]]` `validation-constraints` with six `[[step.increment]]` entries, its sidecar `docs/plans/agent-scaffold.steps/validation-constraints.md`, the `Q-70` fold to `decided`, and the regenerated `docs/plans/agent-scaffold.md`.

Findings triaged: `docs/plans/agent-scaffold.reviews/validation-constraints-reviewer-fidelity.md` (`PR-A-1` to `PR-A-6`) and `docs/plans/agent-scaffold.reviews/validation-constraints-reviewer-checks.md` (`PR-B-1` to `PR-B-7`). Thirteen raw findings.

WHICH TREE I MEASURED IN. Every citation, grep and command below was run against the worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/tri-vcstep` at `HEAD` of `triage/vcstep`, named by absolute path in every invocation. Every build and fixture lives under `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/trivc`, called `<scratch>` below. Nothing outside `<scratch>` was written except this file. I did not read or reuse the checks reviewer's `<scratchpad>/vcs` directory; every binary and fixture below I authored and built myself.

RESULT: 13 raw, 10 valid, 3 duplicates, 0 dismissed. Ceiling `high`, two findings at it. Because I dismissed nothing, no dismissal at `high` or above is owed a second triager's re-check.

---

## The inc1 mutation battery, rebuilt

`PR-B-1`, `PR-B-2`, `PR-B-5` and `PR-B-7` all rest on built mutations, so I rebuilt the battery from scratch rather than reading the reviewer's table.

BUILDS. `<scratch>/build` is `git archive HEAD` of the worktree. `<scratch>/patch.py` writes one variant of `src/workflow.rs` from a pristine copy, asserting on every anchor so a silently-missed edit cannot produce a false result. The variants:

| id | what it is | the `src/workflow.rs` change |
| --- | --- | --- |
| `prefix` | the shipped tool | pristine `HEAD` |
| `ref` | the decided direction (iii), reporting form | shared `waiver_owns_increment(waiver, step_slug, increment)`; W3's covering-waiver match consults it; `w5_problems` widened to take `rounds`; the lexical rule replaced by `rounds.iter().any(\|round\| waiver_owns_increment(waiver, round_step_slug(round), round_increment_id(round)))`; the existing mis-scoping test given round records that join `beta-incB` to `beta` |
| `ref-lax` | `ref`, with the mis-scoping test migrated the OTHER conforming way | identical source; the mis-scoping test's fixture keeps NO round records, so only its asserted message moves |
| `mutE` | THE STEP AXIS DROPPED ON W5'S SIDE | `ref`, except W5 passes `&waiver.step` where `ref` passes `round_step_slug(round)`, so the predicate's step comparison degenerates to `waiver.step == waiver.step` |
| `mutE-lax` | `mutE` under the lax test migration | as `mutE`, tests as `ref-lax` |
| `mutA2` | THE RULE DELETED OUTRIGHT | the whole `if waiver.unit == WaiverUnit::Increment` block removed, the one existing mis-scoping test rewritten to `assert!(problems.is_empty())` |
| `mutC` | ONLY THE TWO WAIVERS THIS REPO NEEDS | the lexical rule kept verbatim, guarded by `&& !increment.ends_with("-fold")` |
| `<base>-pred` | acceptance check 7's own mutation | the shared predicate's increment comparison flipped from `==` to `!=`, applied over `ref` and over `mutE-lax` |

FIXTURES, all built by `<scratch>/mkfx.py`:

- `fx1`: the live `docs/` tree with both fold tokens declared as `[[step.increment]]`, both owed waivers (`-w5`, `-w6`) written as `accepted-at-escalation` / `record-backed`, and `workflow-enforcement-tier` flipped to `complete`. Check 3's fixture.
- `fx2`: the live `docs/` tree plus one `[[step.waiver]]` `probe-w9` naming `increment = "totally-not-a-step-inc1"`. Check 5's fixture.
- `fx3`: a Markdown plan (`alpha`, `beta`, both `complete`) plus a JSONL log carrying a CORRECTLY scoped increment waiver whose increment id `alpha-fold` has no `-inc<alnum>` tail. Check 6's fixture.
- `fx4`: the same shape, but the waiver claims increment `beta-fold` for step `alpha` while both of `beta-fold`'s round records carry `"step":"beta"`. NO ACCEPTANCE CHECK IN THE LIST CORRESPONDS TO THIS FIXTURE.

COMMANDS. `direnv` exported from the worktree, `TMPDIR=<scratch>/tmp` (outside any repository, per the sidecar's preamble at line 112).

```
CARGO_TARGET_DIR=<scratch>/tgt cargo build  --manifest-path <scratch>/build/Cargo.toml
CARGO_TARGET_DIR=<scratch>/tgt cargo test   --manifest-path <scratch>/build/Cargo.toml
CARGO_TARGET_DIR=<scratch>/tgt-clippy cargo clippy --all-targets --manifest-path <scratch>/build/Cargo.toml -- -D warnings
bash <scratch>/run-checks.sh <scratch>/bin-<v> <v>
```

`run-checks.sh` runs checks 2, 3, 5, 6 and 8 verbatim plus the unlisted observed-contradiction case. MEASURED RESULTS:

| check | `prefix` | `ref` | `ref-lax` | `mutE` | `mutE-lax` | `mutA2` | `mutC` |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 `cargo test` | 378 pass | 378 pass | 378 pass | 377 pass, 1 FAIL | 378 pass | 378 pass | 378 pass |
| 1 `cargo clippy --all-targets -- -D warnings` | exit 0 | exit 0 | exit 0 | exit 0 | exit 0 | exit 0 | exit 0 |
| 2 live plan exits 0, `workflow invariants hold` | pass | pass | pass | PASS | PASS | PASS | PASS |
| 3 `fx1` exits 0 | FAIL (exit 1, both ownership refusals) | pass | pass | PASS | PASS | PASS | PASS |
| 5 `fx2`, the message asserts a fact | FAIL | pass | pass | PASS | PASS | pass (vacuously) | FAIL |
| 6 `fx3` exits 0 | FAIL (exit 1) | pass | pass | PASS | PASS | PASS | PASS |
| 7 mutating the predicate reds a W3 test AND a W5 test | n/a | pass | pass | PASS | PASS | fail (no predicate) | fail (no predicate) |
| 8 `render --check --strict` | pass | pass | pass | PASS | PASS | PASS | PASS |
| UNLISTED: `fx4` mis-scoped waiver refused | REFUSED | REFUSED | REFUSED | ACCEPTED at exit 0 | ACCEPTED at exit 0 | ACCEPTED at exit 0 | ACCEPTED at exit 0 |

The exact outputs that decide the verdicts:

```
$ <scratch>/bin-prefix validate --source <scratch>/fx1/docs/plans/agent-scaffold.plan.toml --workflow
... TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`
... TOML waiver `workflow-enforcement-tier-w6`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-endproperty-fold` belongs to step `workflow-enforcement-tier-endproperty-fold`
exit=1

$ <scratch>/bin-mutE validate --source <scratch>/fx1/docs/plans/agent-scaffold.plan.toml --workflow
... 316 records, valid
... 96 steps, 70 questions, valid
... workflow invariants hold
exit=0

$ <scratch>/bin-ref validate --plan <scratch>/fx4/docs/plans/fixture.md --workflow
... round log line 4: increment waiver names step `alpha` but no round record joins increment `beta-fold` to that step
exit=1

$ <scratch>/bin-prefix validate --plan <scratch>/fx4/docs/plans/fixture.md --workflow
... round log line 4: increment waiver names step `alpha` but increment `beta-fold` belongs to step `beta-fold`
exit=1

$ <scratch>/bin-mutE validate --plan <scratch>/fx4/docs/plans/fixture.md --workflow
... 4 records, valid
... 2 steps, 0 open-questions items, valid
... workflow invariants hold
exit=0
```

CHECK 7 RUN AS THE STEP SPECIFIES IT, over both bases:

```
$ patch.py ref-pred        && cargo test --bin agent-scaffold
test result: FAILED. 368 passed; 10 failed
  W3 side: a_short_streak_increment_with_a_covering_increment_waiver_passes, a_bare_slug_increment_waiver_exempts_a_short_streak
  W5 side: w5_passes_a_record_backed_waiver_with_a_matching_escalation, w5_flags_a_record_backed_waiver_with_no_matching_escalation,
           w5_flags_a_record_backed_waiver_citing_an_unrelated_escalation, w5_flags_a_record_backed_waiver_whose_escalation_resumed_not_decided,
           w5_accepts_the_three_valid_reason_tier_pairings
$ patch.py mutE-lax-pred   && cargo test --bin agent-scaffold
test result: FAILED. 368 passed; 10 failed        (the same ten, both sides)
```

So check 7's demonstration SUCCEEDS identically on the `mutE` base. It pins that the predicate is shared, not that W5 quantifies over the right thing.

THE ONE PLACE MY REPRODUCTION DIVERGES FROM THE REVIEWER, and it matters. The reviewer reports `mutE` passing `cargo test` at 378 byte-identically. That reproduces under `mutE-lax` and NOT under `mutE`. The difference is entirely in how the ONE existing test for this rule, `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment` (`src/workflow.rs:1437-1458`), is migrated when the rule becomes structural:

- FAITHFUL migration (mine): the fixture gains round records that join `beta-incB` to step `beta`, so the test still states the observed contradiction its own name claims. `ref` is 378 green; `mutE` is `377 passed; 1 failed`, failing exactly that test.
- LAX migration (the reviewer's): the fixture keeps no round records, so after the fix the test exercises the UNOBSERVED case and only its asserted message moves. `ref-lax` and `mutE-lax` are both `378 passed; 0 failed`, byte-identically.

Both migrations conform to the step as written, because NOTHING IN THE ACCEPTANCE LIST SAYS WHICH. That correction does not weaken `PR-B-1`; it sharpens it. The finding is not "mutE necessarily passes"; it is "no check in the list forces a build that accepts an observed contradiction to fail, so whether the class is caught at all is left to an unforced implementer choice about one test fixture". I rule on the finding in that corrected form.

---

## Verdicts

### `PR-B-1` (checks lens): no acceptance check requires a mis-scoped increment-unit waiver to be refused

VERDICT: VALID. SEVERITY: `high` (confirming the reviewer's rating).

EVIDENCE REPRODUCED. The table and outputs above, all first-hand. `mutE` passes checks 2, 3, 5, 6 and 8; check 7's own demonstration succeeds on it; and it accepts `fx4` at exit 0 printing `workflow invariants hold` over a log in which `beta-fold`'s round records both carry `"step":"beta"` while the waiver claims it for `alpha`. BOTH the pre-fix binary AND `ref` refuse that same tree. So `mutE` is a REGRESSION AGAINST THE SHIPPED TOOL that every listed check calls a pass, on a build where the shared predicate is genuinely shared and not a facade.

CORRECTION TO THE FINDING AS WRITTEN. Its check 1 row holds only under the lax test migration (measured above). Under the faithful migration check 1 catches `mutE`. This is a contingency, not a check: the step's list contains no obligation that produces the faithful migration, so the finding's substance survives intact.

WHY `high` AND NOT LOWER. The step exists to remove a false green from the validator that the project's whole enforcement tier rests on. Its own risk note (sidecar line 96) names the failure mode as "a false green admits a waiver that grants an exemption nothing evidences". Checks 2, 3, 5 and 6 all pin the GREEN direction and check 4 pins the unobserved boundary; nothing pins the red direction on an OBSERVED contradiction, which is the only case in which the rule does any work at all once it is structural. This is the last adversarial pass before an implementer builds, and the artifact is `risky` by the step's own classification.

WHY NOT `critical`. It is a gap in a plan's acceptance list rather than shipped code; the round report still reaches a human; and one of the two conforming test migrations catches the specific mutation by accident.

REMEDY, scoped to the class over the whole Acceptance check section rather than to one item.

1. ADD A CHECK, between the present items 4 and 5, of the form: "AFTER INC1, AN OBSERVED CONTRADICTION IS REFUSED. An increment-unit waiver whose increment's round records join it to a DIFFERENT step is REPORTED, on BOTH substrates, and the round report quotes the fixture and the message. THE RED HALF IS A MUTATION, NOT THE PRE-FIX BINARY: the pre-fix binary refuses this case too, by accident of the lexical rule, so it cannot supply the red. The mutation is the step axis dropped on W5's side of the shared predicate. This obligation is INDEPENDENT of the item 4 fork and holds under both the reporting form and the silent form." A worked fixture already exists in this triage (`fx4`), and it is `fx3` with a second step and one changed field.
2. AMEND ITEM 4 (sidecar line 117), the WHOLE item including both branches. Its silent branch's three obligations are satisfied by a build with no rule at all (measured: `PR-B-2` below), so state that item 4 pins the UNOBSERVED case only, and that the observed case is pinned by the new item whichever form is taken.
3. AMEND ITEM 6 (sidecar line 119), the whole item. It currently pins only ACCEPTANCE on the Markdown and JSONL substrate, which by inc2's own argument (sidecar line 66) is the one substrate where the mis-scoped state is authorable. Give it the red half from item 1, or state that its red half is carried by the new item.
4. AMEND ITEM 1 (sidecar line 114) to name the migration obligation explicitly: the existing test `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment` must keep its subject, so its post-fix fixture must carry round records that establish the increment's true owner. Without this the whole class rests on taste.
5. IN-PLACE VERDICTS on the sites the reviewer named and I am leaving alone: item 7 (line 120) is CORRECT AS WRITTEN and stays, but the amended item 1 should note that check 7 pins sharing and not quantification, so it is not the catch for this class. Items 2, 3, 5 and 8 stay unchanged; the reviewer does not fault them for this class and `mutC` shows item 5 doing real work.

### `PR-B-2` (checks lens): check 4's silent branch is satisfied by deleting the rule outright

VERDICT: DUPLICATE OF `PR-B-1`. SEVERITY (were it standalone): `medium`.

EVIDENCE REPRODUCED, and it reproduces in full. I built `mutA2` (the whole `if waiver.unit == WaiverUnit::Increment` block deleted, the one existing test rewritten to `assert!(problems.is_empty())`). Measured: `378 passed; 0 failed`; `cargo clippy --all-targets -- -D warnings` exit 0; checks 2, 3, 6 and 8 all green; `fx1` returns `workflow invariants hold` at exit 0, so deleting the rule is by itself a complete unblocking; `fx2` still reports the membership problem from `src/plan/source.rs:807-811`, quoted in full in the run log as "waiver probe-w9 on step workflow-enforcement-tier names increment totally-not-a-step-inc1, which is not one of the step's increments" (the tool prints each identifier inside backticks), satisfying item 4's second obligation; a declared-and-never-logged increment passes, satisfying the third; and `fx4` is ACCEPTED at exit 0.

WHY DUPLICATE RATHER THAN SEPARATELY VALID. The reviewer states it themselves: under the silent form the observed contradiction is the ONLY case in which the rule fires, so the single missing check closes both. `PR-B-1`'s remedy item 2 above names check 4's silent branch as a site, so the planner cannot close `PR-B-1` while leaving this open. Its independent evidence stands as corroboration and its site is carried.

### `PR-B-3` (checks lens): no check verifies that the shipped rule text matches the shipped behaviour

VERDICT: VALID. SEVERITY: `medium` (confirming).

EVIDENCE REPRODUCED. `grep -rn "must own its .increment." pack/instrument.md AGENTS.md .agents/AGENTS.reference.md` returns exactly `pack/instrument.md:11`, `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`, each stating "an `increment`-unit waiver's `step` must own its `increment` (the increment's leading slug equals the step)". Every one of my six behaviour-changing variants (`ref`, `ref-lax`, `mutE`, `mutE-lax`, `mutA2`, `mutC`) left all three files untouched and every one reported 378 passing tests, `the_committed_scaffold_matches_a_fresh_render` included. Reading `src/agents_md_drift.rs:375-400` confirms why: the guard compares the committed `AGENTS.md` and `.agents/AGENTS.reference.md` against a FRESH RENDER OF THE PACK, so it detects a PARTIAL prose edit and is structurally blind to a MISSING one, because it never reads `src/`.

Check 1 (sidecar line 114) names that guard, which reads as though it covers the rule text. It does not. The Documentation impact section (line 134) correctly books the three files as owed work; owed work with no check is the class the step's own risk note (line 96) calls this project's worst measured one, and this text ships into every scaffolded project.

REMEDY, scoped to the class of "prose that states an enforcement boundary, changed by this increment, with no check" over the whole Acceptance check section and the whole INC1 Documentation impact block.

1. ADD a check item: a fixed-string command asserting the RETIRED wording "the increment's leading slug equals the step" is absent from all three copies and that the replacement wording is present in all three, with an expected exit code like the rest of the list. State it as one command over the three paths so a partial edit fails it.
2. AMEND check 1 (line 114) so that naming the drift guard does not imply rule-text coverage: say what the guard does cover (the pack-to-committed generation path) and that the rule text's currency is the separate item above.
3. AMEND the INC1 Documentation impact bullet at line 134, the whole bullet, to point at the new check rather than leaving the three files as unchecked owed work.
4. IN-PLACE VERDICT on the sites left alone: the REGENERATION HAZARD bullet (line 135) and the `NOT README.md` / `NOT pack/plan-template.plan.toml` exclusions (line 139) are correct and stay; I re-ran `grep -c "must own its" README.md` and it returns 0.

### `PR-B-4` (checks lens): inc6 acquires a dependency on inc1 that the step does not record

VERDICT: VALID. SEVERITY: `medium` (confirming).

EVIDENCE REPRODUCED. `src/workflow.rs:185-192` is `check_workflow_toml` computing `metrics::parse_rounds(log_contents)` at ONE site and handing it to `run_checks`; `grep -n "metrics::parse_rounds"` over `src/` returns exactly four sites, `src/workflow.rs:162` (the Markdown twin), `:189` (this one), `:634` (a test helper) and `src/main.rs:1739` (the `next` path, which is the recorded "a filter inside `run_checks` does not cover `next`" limitation). After inc1, `w5_problems` becomes a consumer of that same slice, so a filter applied at `:189` governs W5 as well as W3. The three limitations recorded at sidecar line 70 are opt-in, `next` uncovered, and `decisions`/`escalations` unfiltered. NONE names W5.

THE HAZARD IS CONCRETE AND FOLLOWS FROM THE FIRST LIMITATION. Identity is opt-in, so on a merged log the plan's own pre-migration rounds carry no `project`. A filter keeping only rounds whose `project` matches the plan drops them all, and under inc1's DEFAULT reporting form W5 then reports EVERY increment-unit waiver in the plan, because no surviving round joins any waived increment. `grep -c '^unit = "increment"$' docs/plans/agent-scaffold.plan.toml` returns 13, so that is 13 false reds on a correct plan, produced by two increments the step says are independent apart from ordering.

This is a boundary defect and not a direction defect. Sidecar line 43's "direction (iii) ADDS NO FIELD TO ANY STRUCT" and line 72's one-deliberate-edit ordering argument both remain true and are not reopened.

REMEDY, scoped to the whole `validation-constraints-inc6` bullet (sidecar line 70), its ordering paragraph (line 72) and its acceptance item (line 126).

1. ADD A FOURTH RECORDED LIMITATION to the inc6 bullet: after inc1, W5 reads the same `rounds` slice, so the identity filter's placement governs W5's ownership verdict as well as W3's convergence verdict, and under the reporting form an over-broad filter turns every increment-unit waiver red. Cite `src/workflow.rs:189` and `:206-221`.
2. AMEND INC6'S REVIEW QUESTION in the same bullet. "Does an identity that is absent change nothing" must now be answered for W5 as well as for W3 and `next`, because absence is exactly the state that produces the false red.
3. AMEND ACCEPTANCE ITEM 13 (line 126), which asks the round report to state "which surfaces the filter placement does and does not cover, `next` included": name W5 alongside `next`, and require the absent-identity case to be exercised against a plan carrying at least one increment-unit waiver.
4. RECORD THE MEASURED FACT THAT MAKES THE ORDERING RIGHT, from `docs/plans/validation-constraints.explorations/Q-70-architecture.md:296` and `:336`: today W5 reads no rounds so an identity filter would not protect its verdict, and under direction (iii) it would, so inc1 STRICTLY IMPROVES the coverage of the queued identity edit. This runs in favour of the recorded inc1-before-inc6 ordering, not against it, and saying so prevents a later reader reading limitation 4 as a reason to re-order.
5. IN-PLACE VERDICT on the site left alone: line 72's entry gate to a human before inc6 stays as written and is the right place for the note-join question; it needs no change beyond the limitation above being visible to whoever answers it.

### `PR-B-5` (checks lens): check 6 pins only acceptance on the substrate where the mis-scoped state is authorable

VERDICT: DUPLICATE OF `PR-B-1`. SEVERITY (were it standalone): `medium`.

EVIDENCE REPRODUCED. Check 6's text (sidecar line 119) is acceptance on both halves. Measured on `fx3`: `prefix` exit 1, and `ref`, `ref-lax`, `mutE`, `mutE-lax`, `mutA2` and `mutC` ALL exit 0. Every variant including total deletion passes it. Inc2's own entry (line 66) is what makes this matter: on the TOML path a waiver inherits its `step` from the containing `[[step]]` and the typed `Waiver` struct is `deny_unknown_fields` with no `step` field, so the rule "cannot fire there", while on the JSONL substrate `step` is a free string and the state IS authorable.

WHY DUPLICATE. `PR-B-1`'s remedy already requires the new red case "on BOTH substrates", and its remedy item 3 names check 6 (line 119) as a site to amend. One added red case closes both findings. Carried, not separately counted.

### `PR-B-6` (checks lens): the Acceptance check section's opening universal claim is false

VERDICT: VALID. SEVERITY: `low`, CORRECTED DOWN from the reviewer's `medium`.

EVIDENCE REPRODUCED, by reading the items at the cited lines. Line 110 opens "Every claim below is a command with an expected exit code, so a round is settled by running it rather than by reading the diff". Item 9 (line 122) asks that "a reader of the rule's documentation can predict, for each substrate, whether the rule can fire", a property of a reader. Item 11 (line 124) requires that `next`'s "output is proportionate rather than a verbatim ledger dump", with no threshold, so two reviewers can reach opposite verdicts with nothing to settle between them; its other two clauses are checkable. Item 12 (line 125) asks the implementer to "show that the answer `next` gives ... FOLLOWS FROM the structured source rather than from ledger prose", an argument. Item 13 (line 126) is a reporting obligation. Four of thirteen.

WHY `low` AND NOT `medium`. The impact if left unfixed is a plan-prose overstatement plus one unsettleable criterion on inc4, in a section that already carries its own hedge two sentences later ("THE LIST IS A FLOOR AND NOT A TOTAL"). No product behaviour follows from it, and the unsettleable clause costs at most a round of argument on a later increment. It is real, and it is small.

REMEDY, scoped to the opening paragraph (line 110) and to the four items as a class, not to the quoted fragments.

1. REWRITE THE OPENING SENTENCE of the Acceptance check paragraph so it states what is true of the list: the items that are commands are settled by running them, and the items that are round-report obligations are settled by the report. Keep the `Q-66` red-then-green sentence that follows it; it is correct and load-bearing.
2. MARK THE FOUR ITEMS (9, 11, 12, 13) as report obligations rather than commands, in place, so the distinction is visible at each item rather than only in the opening.
3. GIVE ITEM 11'S "proportionate" CLAUSE A SETTLEABLE FORM over the whole item, for example a stated output bound with the measurement command, or delete the clause and let the "no verbatim ledger dump" half carry it. The other two clauses in item 11 stay.
4. IN-PLACE VERDICT on the sites left alone: items 1 to 8 and 10 are genuine commands and stay as written.

### `PR-B-7` (checks lens): check 2 is mislabelled "the migration proof"

VERDICT: VALID. SEVERITY: `low` (confirming).

EVIDENCE REPRODUCED. Check 2 (sidecar line 115) is labelled "the no-regression check and the migration proof". Measured: `<scratch>/bin-prefix validate --source <worktree>/docs/plans/agent-scaffold.plan.toml --workflow` exits 0 with `316 records, valid`, `96 steps, 70 questions, valid`, `workflow invariants hold`. The same command exits 0 on `ref`, `ref-lax`, `mutE`, `mutE-lax`, `mutA2` and `mutC`. A check that is green before the change, after the change, and after the change is deleted has no red half and distinguishes nothing. The `Q-66` obligation the section invokes at line 110 is not met by this item.

REMEDY, over the whole of check 2.

1. DROP "and the migration proof" from the label, or state what the item's value actually is: it establishes that the population the narrowing affects is EMPTY on this plan, which is a precondition claim and not a demonstration of the fix.
2. IN-PLACE VERDICT: the item's substance (exits 0, no edit to any waiver, no edit to the append-only log) is correct and confirmed against `ref`, and stays.

### `PR-A-1` (fidelity lens): inc5's `low_risk` classification rests on a false premise

VERDICT: VALID. SEVERITY: `high` (confirming the reviewer's rating).

EVIDENCE REPRODUCED, all of it first-hand, and the orchestrator's `src/plan/render.rs:480` is one of four sites, not the only one.

THE READERS, each read at the line in this worktree:

- `src/plan/render.rs:478-488`, the `notes_cell` function, whose first loop is `for blocker in &step.blocked_by` and which pushes one `ROADMAP_BLOCKED_PREFIX` note per blocker. It writes the Notes cell of every Roadmap row in the generated `<task>.md`.
- `src/next.rs:720-724`, `.filter(|step| step.phase.is_pending() && blockers_met(step, steps))`, which selects the pending step `next` recommends, and `:726-732`, which returns a `LoopState::Blocked` loop when no pending step has its blockers met.
- `src/next.rs:737-751`, `blockers_met` and `unmet_blockers`, and `:1012`, which fills the `blocked_by` context slot of the emitted instruction.
- `src/plan/source.rs:599-608`, the `validate_source` cross-reference that flags a self-reference or a dangling slug.

DEMONSTRATION 1, THE RENDERED VIEW CHANGES. Against a scratch copy of `docs/` from this branch, adding one blocker to the very step this diff introduces:

```
$ <scratch>/bin-prefix render <scratch>/bb/docs/plans/agent-scaffold.plan.toml --check
warning: .../agent-scaffold.md differs from a fresh render (a hand-edit, or a stale render after a source edit)
(first difference at line 350: expected "| `validation-constraints` | not started | blocked on `workflow-enforcement-tier...",
 committed "| `validation-constraints` | not started | why: decisions Q-55, Q-70 |")
```

DEMONSTRATION 2, `next`'S ANSWER FOLLOWS THE FIELD. Two copies of the repository's own fixture `src/plan/testdata/skeleton.plan.toml`, differing in exactly one line (`diff` output: `22c22`, `blocked_by = ["beta"]` against `blocked_by = []`), with `alpha` and `beta` both `not-started` so the pending-selection rule at `src/next.rs:722` is the one that decides:

```
$ <scratch>/bin-prefix next --source <scratch>/nx/docs/plans/c.plan.toml --json | jq -c '{step: .active_loop.step, state: .active_loop.state}'
{"step":"beta","state":"ready-to-plan"}
$ <scratch>/bin-prefix next --source <scratch>/nx/docs/plans/d.plan.toml --json | jq -c '{step: .active_loop.step, state: .active_loop.state}'
{"step":"alpha","state":"ready-to-plan"}
```

The recommended step flips on the field alone.

THE CLAIM SITES, and the distinction the reviewer draws is correct and is the heart of the finding. The ledger's routing of this member records the field as UNPOPULATED and states its consequence in terms of what `next` ADVISES; the step converted that into UNREAD and priced the increment on the converted claim. Sidecar `:69` ("THE UNUSED `blocked_by` FIELD"), `:104` ("Populating `blocked_by` changes no product behaviour today, because nothing reads the field, so a wrong value is inert and reversible in one revert") and `:146` ("a typed field that nothing currently reads"). The emptiness half of `:69` is TRUE and I re-measured it: `grep -c '^\[\[step\]\]$'` and `grep -c '^blocked_by = \[\]$'` both return 96. The unread half is false.

THE ESCAPE HATCH CANNOT FIRE FOR THE TREATMENT THE RECORD ASKS FOR. `:104` says re-classify to risky "if the chosen treatment ALSO teaches `next` to honour the field or retires the field from the schema". `next` already honours it, so that trigger is unreachable; only the retirement trigger is reachable. The "populate it" treatment, the one the ledger queues, fires neither, so inc5 converges at one clean round on a false ground.

A FIFTH SITE THE REVIEWER DID NOT NAME, AND IT IS THE ONE THE TOOL READS. `docs/plans/agent-scaffold.plan.toml:1415-1416` declares `id = "validation-constraints-inc5"` with `risk_class = "low_risk"`. That is the STRUCTURED source W3 reads to set the required streak; the sidecar prose is a projection of the judgement, not its enforcement. A remedy that corrects only the prose leaves the machine-read classification wrong.

WHY `high`. AGENTS.md's convergence rule makes the classification "a recorded property of the artifact rather than a fresh subjective judgement each round", so later rounds inherit it rather than re-derive it. The consequence chain ends where the step itself rates inc4 `risky`: `next` emits an INSTRUCTION an agent acts on, so a wrong answer is a wrong action, and the ledger records that exact failure having already occurred once. The premise is falsifiable in one command, and this is the last adversarial pass before an implementer builds.

THE COUNTER-ARGUMENT FOR `medium`, recorded so the ruling is auditable. inc5 is scoped to plan content; a dangling or self-referential blocker IS caught by `validate_source`; the change is reversible in one revert; and the practical delta is one clean round rather than two. I judged that insufficient because the uncaught case, a blocker naming a REAL but WRONG step, passes every gate this repository owns and silently changes what `next` tells the next agent to do.

REMEDY, scoped to the class "the step asserts the field is UNREAD where the record says UNPOPULATED, and prices an increment on that assertion", over every enclosing sentence and paragraph.

1. SIDECAR `:104`, THE WHOLE `validation-constraints-inc5` RISK PARAGRAPH. Rewrite it. The ground must not be "nothing reads the field". Either classify inc5 `risky`, or state the classification PER TREATMENT and defer it to the treatment decision, since the fork at `:69` contains no treatment that leaves both the rendered view and `next`'s answer untouched. Replace the escape hatch entirely: its `next`-honouring trigger is unreachable as written, so it must not be the thing that carries the re-classification.
2. `docs/plans/agent-scaffold.plan.toml:1416`, `risk_class` for `validation-constraints-inc5`. It must move with the prose, or the enforced streak stays `low_risk` whatever the sidecar says. This is the site the finding does not name and the only one the tool reads.
3. SIDECAR `:69`, THE WHOLE `validation-constraints-inc5` BULLET. Restate the member as the UNPOPULATED field, keep the re-measured emptiness figures (they hold), and add the reader inventory by `file:line` so the treatment fork is stated against what the field already does: `src/plan/render.rs:478-488`, `src/next.rs:720-732`, `:737-751`, `:1012`, `src/plan/source.rs:599-608`.
4. SIDECAR `:146`, THE WHOLE INC4-AND-INC5 DOCUMENTATION-IMPACT BULLET. I read `blocked_by`'s doc comment at `src/plan/source.rs:144-145`; it says only that the field carries "the slugs of the steps that block this one (typed, replacing the Markdown `blocked on <slug>` parametric status)" and makes NO claim about readers. So "a typed field that nothing currently reads" is the sidecar's own gloss, not a quotation of a false comment, and the owed doc work for inc5 must be restated from what the comment actually says. There is no `src/` defect here to route.
5. IN-PLACE VERDICT on the site left alone: acceptance item 12 (`:125`), which asks the implementer to "show that the answer `next` gives about which step is next follows from the structured source", is the CORRECT half and stays. Once `:104` is rewritten it no longer contradicts it. (Item 12's separate "follows from" wording is `PR-B-6`'s subject, not this one's.)

### `PR-A-2` (fidelity lens): two ledger-routed members are absent and the step closes the set at "TWO"

VERDICT: VALID. SEVERITY: `medium` (confirming).

EVIDENCE REPRODUCED. Each anchored grep over `docs/plans/agent-scaffold.ledger.md` returns exactly 1 hit and lands on its intended paragraph:

- `grep -c '^ONE ITEM IS QUEUED BY THIS DECISION RATHER THAN FIXED'` = 1, the plain-`validate` mode-000 file versus unsearchable directory inconsistency. CARRIED, as inc3.
- `grep -c '^THE PRE-EXISTING CONTAINMENT TOCTOU IS CONFIRMED PRE-EXISTING AND IS ROUTED'` = 1. CARRIED, as inc3. Its own text reads "It routes to the validation-constraints step beside `R2A-4` and `R3A-3`", naming the other two.
- A line-start anchored grep for the paragraph opening "R2A-4 WAS ACCEPTED AS A RESIDUAL" (the id is backticked in the file) = 1 hit: "the pre-existing `no metrics log at <path>` note still prints one line above the corrected sentence ... Routed into the queued validation-constraints item alongside the pre-existing plain-`validate` inconsistency." NOT CARRIED.
- A line-start anchored grep for the paragraph opening "R3A-3 IS OUT OF SCOPE FOR THIS INCREMENT AND IS ROUTED" (same backticking) = 1 hit: a mode-600 `docs/plans` yielding "no source plan at X" with a remedy aimed at someone who already passed one. "IT GOES TO THE VALIDATION-CONSTRAINTS STEP, beside `R2A-4` and the queued plain-`validate` inconsistency. Same family, NOT the same defect: that one is about the LOG input's exit codes, this one about the PLAN-SOURCE input's message." NOT CARRIED.

MEASURED ABSENCE, over the sidecar and the plan TOML: `grep -cF 'R2A-4'` = 0 in both, `grep -cF 'R3A-3'` = 0 in both, and `grep -cF 'no source plan'`, `'mode-600'` and `'no metrics log'` all return 0 over the sidecar.

WHY THE COUNT MAKES IT WORSE, and I confirmed the tension is inside one file. The members section opens "NO COUNT IS STATED HERE AND NONE MUST BE ADDED ... a maintained count of a moving set is this project's most repeated defect" (`:11`), then states "THE TWO PRE-EXISTING `validate` DEFECTS" two bullets later (`:15`) and again at `:67`. The ledger's own routing paragraph is explicit that an earlier form saying "FOUR THINGS" "WAS UNDERCOUNTED BY AT LEAST TWO, WHICH IS RECORDED AS ORCHESTRATOR DEFECT (19)". inc3's stated review question at `:67` reaches neither omitted member: `R3A-3` is about the PLAN-SOURCE input's message, which that question does not touch at all.

IMPACT. The findings files are commit-deleted and the ledger is deleted at task close, so this step is the durable home for both. They are lost when the ledger goes, and the affirmative "TWO" tells a re-deriver the set is complete.

REMEDY, scoped to the class "a closed count over a set the same file says is open", across every site that states one.

1. SIDECAR `:15`, THE WHOLE BULLET. Remove the count. Restate the member as the `validate`-input defect family and enumerate the members present, with the ledger anchor for each, in the same style the other bullets use. ADD `R2A-4` and `R3A-3` with their anchored ledger handles, preserving `R3A-3`'s recorded distinction that it is the same family and NOT the same defect.
2. SIDECAR `:67`, THE WHOLE `validation-constraints-inc3` BULLET. Remove the count from its opening. WIDEN OR SPLIT the review question: as written ("does the tool report the same thing about a log it cannot read, whichever way it cannot read it, and can the containment guard still be defeated between the check and the use") it does not reach the PLAN-SOURCE message defect, so an inc3 reviewer working from this brief has no prompt to look for it. If the increment-is-one-question rule makes `R3A-3` a poor fit for inc3, declare it as its own increment rather than dropping it.
3. SIDECAR `:11`, the members paragraph. It is CORRECT and stays; the fix is to make the bullets below it obey it. Consider adding the standing instruction that a bullet states members and never a figure, so the next author does not re-introduce one.
4. ACCEPTANCE ITEM 10 (`:123`), which names "the two `validate` defects ... with the fixtures the ledger records": it inherits the same count and must move with the bullets, naming the fixtures for whatever set inc3 ends up carrying.
5. IN-PLACE VERDICT on the sites left alone: the member-by-member enumeration the reviewer supplies is otherwise accurate; I independently re-ran every anchored ledger grep it uses and each returns exactly 1 hit. Members (a), (b), (d), (e) and (f), the two `src/` defects, and the ownerless `run_next` ledger half are all carried correctly and need no change.

### `PR-A-3` (fidelity lens): explorer B's negative result, routed here by name, is not carried

VERDICT: VALID. SEVERITY: `low` (confirming).

EVIDENCE REPRODUCED. `grep -c "B's negative result belongs to the same queued step" docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` returns 1, and the sentence reads: "B's negative result belongs to the same queued step and should be carried into it: declaring the log's LOCATION does not establish that the log BELONGS to the plan, and building a path field first would make the identity work harder, because the identity check would then have to reconcile itself against a declared path that may disagree with it." Over the new sidecar, `grep -ciF 'negative result'`, `grep -ciF 'path field'` and `grep -ciF 'declared path'` all return 0.

The step DOES inherit the three project-identity limitations from the same source paragraph correctly, so the omission is specific to this one sentence.

REMEDY, over the whole `validation-constraints-inc6` bullet (`:70`).

1. CARRY THE CONSTRAINT into the inc6 bullet as an ordering constraint with its provenance: do not build a declared log-path field before the identity work, because declaring the log's location does not establish that the log belongs to the plan, and a declared path that disagrees with the identity check makes the identity work harder. Cite the `workflow-enforcement-tier` sidecar as the source.
2. IN-PLACE VERDICT on the sites left alone: the three inherited limitations at `:70` are carried verbatim and correctly and stay; the entry gate at `:72` stays. Note that this remedy and `PR-B-4`'s both land in the same bullet, so they should be authored together.

### `PR-A-4` (fidelity lens): the TOML `title` and the sidecar heading disagree, and only the narrower reaches the reader

VERDICT: VALID. SEVERITY: `low` (confirming).

EVIDENCE REPRODUCED. `docs/plans/agent-scaffold.plan.toml:1388` reads "state W5's waiver-ownership rule against the round log so the two owed waivers become writable (`Q-70`, direction (iii)), then treat the validator-cluster defects, THE `agent-scaffold next` DEFECTS, PROJECT IDENTITY AND THE DETECTION MECHANISMS the record routes here". `docs/plans/agent-scaffold.steps/validation-constraints.md:1` reads "... then treat the validator-cluster defects the record routes here", dropping three of the four named member classes. `docs/plans/agent-scaffold.md:2033` carries the sidecar heading verbatim, and `docs/plans/agent-scaffold.md:350` is the Roadmap row whose three cells are the backticked slug `validation-constraints`, the status "not started", and the note "why: decisions Q-55, Q-70". There is no title cell at all.

THE SCOPE LINE IS DRAWN CORRECTLY BY THE REVIEWER AND I AM HOLDING IT. The pre-existing tool behaviour (the TOML `title` is not projected into the rendered view) is recorded in the ledger's accepted-residuals item (3), which I confirmed at 1 anchored hit, and it is NOT raised. What is raised is the AUTHORED divergence, which is in this diff, so the out-of-scope precedent's condition 1 (provenance predates the base commit) fails and the finding stays in scope.

REMEDY, over both one-line descriptions as a pair.

1. BRING THE TWO INTO AGREEMENT by widening the SIDECAR HEADING (`:1`) to name the same member classes the TOML title names, since the heading is the line a reader of the rendered view actually sees. Shortening the TOML title to match the heading would be the wrong direction: recorded orchestrator defect (19) was precisely an UNDER-description of this step's scope.
2. IN-PLACE VERDICT on the site left alone: `docs/plans/agent-scaffold.plan.toml:1388` stays as written; it is the more complete of the two. The rendered view is regenerated from the sidecar, so `render --check` closes the loop with no further site.

### `PR-A-5` (fidelity lens): a ground held by one explorer is attributed to two

VERDICT: VALID. SEVERITY: `low` (confirming).

EVIDENCE REPRODUCED, by reading all three exploration files at the cited lines. The sidecar at `:57` says "Two REPORT it, on the ground that it is the only thing that catches a typo'd increment id on the Markdown and JSONL substrates, where no declared set exists to catch it." The report-versus-silent split is CORRECT: `Q-70-architecture.md:203` and `:286` report the unobserved case, `Q-70-evidence.md:208-213`'s `owned` predicate reports it, and `Q-70-minimal.md:90-91` leaves it silent ("An increment absent from the round log is left unchecked here (nothing to attribute yet)"). But only the architecture lens gives the stated ground; `Q-70-architecture.md:286` reads "because it is the only thing that catches a typo'd increment id on the Markdown and JSONL substrate, where no declared set exists to catch it". The evidence lens gives a DIFFERENT ground at `Q-70-evidence.md:165`: "a waiver that covers an increment with no round records grants nothing in W3 anyway (W3's increment loop is built from the records, so an increment with none never enters it) and reporting it turns a dead waiver into a visible one, which is Make illegal states unrepresentable applied to a waiver."

REMEDY, over the whole OPEN POINT paragraph at `:57`.

1. SPLIT THE ATTRIBUTION: state that two prototypes report the unobserved case, on TWO INDEPENDENT GROUNDS, and give both, each attributed to its lens by file. Independent corroboration by two arguments is a stronger record than one argument credited twice, so this strengthens the default rather than weakening it.
2. IN-PLACE VERDICT on the sites left alone: the report-versus-silent split, the silent lens's honestly-measured residual, and the "carried with a test either way (acceptance check 4)" close are all correct and stay. The step's DEFAULT to report is not changed by this remedy.

### `PR-A-6` (fidelity lens): inc6's limitations are stated from the pre-inc1 world

VERDICT: DUPLICATE OF `PR-B-4`. SEVERITY (were it standalone): `low`.

EVIDENCE REPRODUCED and it holds: `Q-70-architecture.md:336` and `:296` measure that `check_workflow_toml` computes `metrics::parse_rounds(log_contents)` at one site (`:189`) and hands it to `run_checks`, so a filter applied there is inherited by every check that reads `rounds`; today W5 reads none, so an identity filter would not protect W5's ownership verdict, and under direction (iii) it would. Confirmed at the source: `src/workflow.rs:185-192` is that single site, and `:206-221` today passes `waivers, steps, escalations` to `w5_problems` with no `rounds`.

WHY DUPLICATE. It is the same missing fact, at the same site (the inc6 bullet at `:70` and acceptance item 13 at `:126`), as `PR-B-4`, reached from the fidelity side rather than the hazard side. `PR-B-4`'s remedy items 1, 3 and 4 above carry both this finding's citation and its check-13 observation, so one authored change closes both. I kept `PR-B-4` as the primary because it states the concrete failure mode (the false red on a correct plan) and therefore rates `medium` rather than `low`.

---

## Dedup map

| raw id | verdict | severity | one-line ground |
| --- | --- | --- | --- |
| `PR-B-1` | VALID | `high` | Rebuilt: `mutE` passes checks 2, 3, 5, 6, 8 and check 7's own demonstration, and accepts `fx4` at exit 0 where both `prefix` and `ref` refuse it. |
| `PR-B-2` | DUPLICATE OF `PR-B-1` | `medium` | Rebuilt `mutA2`: 378 green, checks 2, 3, 6, 8 green, `fx4` accepted; closed by `PR-B-1`'s remedy, whose site list names check 4. |
| `PR-B-3` | VALID | `medium` | Six behaviour-changing variants left all three prose copies stating the retired lexical rule and all reported 378 passing tests; the drift guard never reads `src/`. |
| `PR-B-4` | VALID | `medium` | `src/workflow.rs:189` is the one `parse_rounds` site feeding `run_checks`; after inc1 W5 reads it, and inc6's three recorded limitations name only W3 and `next`. |
| `PR-B-5` | DUPLICATE OF `PR-B-1` | `medium` | Check 6 is acceptance-only and every variant including total deletion passes it; closed by `PR-B-1`'s remedy, which requires the red case on both substrates. |
| `PR-B-6` | VALID | `low` (down from `medium`) | Items 9, 11, 12 and 13 read at the line are not commands, and item 11's "proportionate" has no threshold. |
| `PR-B-7` | VALID | `low` | Check 2 is green on `prefix` and on all six post-fix variants, so it has no red half. |
| `PR-A-1` | VALID | `high` | Four reader sites read at the line, plus two runnable demonstrations: one blocker rewrites a Roadmap row, and `next`'s recommendation flips on the field alone. |
| `PR-A-2` | VALID | `medium` | Four anchored ledger paragraphs route the family here, the step carries two and states "TWO"; `R2A-4` and `R3A-3` return 0 hits in both the sidecar and the plan TOML. |
| `PR-A-3` | VALID | `low` | The routing sentence returns 1 hit in the `workflow-enforcement-tier` sidecar and 0 of its three key phrases appear in the new one. |
| `PR-A-4` | VALID | `low` | `plan.toml:1388` names four member classes, the sidecar heading names one, and the heading is what `agent-scaffold.md:2033` carries. |
| `PR-A-5` | VALID | `low` | `Q-70-architecture.md:286` gives the quoted ground; `Q-70-evidence.md:165` gives a different one. |
| `PR-A-6` | DUPLICATE OF `PR-B-4` | `low` | Same missing fact, same two sites (`:70`, `:126`); `PR-B-4`'s remedy carries this finding's citation. |

SEVERITY CHANGES FROM THE REVIEWERS: one, `PR-B-6` from `medium` to `low`. Every other rating confirmed.

DISMISSALS: none, at any severity. No second triager's confirmation is owed on a dismissal for this round.

OUT-OF-SCOPE PRECEDENT: applied to none. The step is newly authored, so condition 1 (provenance predates the base commit) fails for every finding. The one candidate, `PR-A-4`, is raised against the AUTHORED divergence rather than the pre-existing tool behaviour, and the reviewer already excluded the latter.

DECIDED DIRECTION: not reopened, and I do not believe I have evidence that beats `Q-70`'s recorded reasoning. Every finding above is about the step's checks, its risk classifications, or its fidelity to the record. `PR-B-1`'s remedy is a missing check, not a different direction; `PR-B-4`'s is a missing limitation, and it runs IN FAVOUR of the recorded inc1-before-inc6 ordering.

## `src/` defects found, for the orchestrator to route

NONE NEW. Everything I touched in `src/` behaved as the step and the explorations describe it. Two things worth recording that are NOT defects:

- `blocked_by`'s doc comment at `src/plan/source.rs:144-145` makes no claim about readers, so `PR-A-1`'s third claim site is the sidecar's own gloss and there is nothing to fix in `src/`.
- The one existing test for the W5 ownership rule, `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment` (`src/workflow.rs:1437-1458`), is correct today. It becomes the single point of failure for the whole class the moment the rule becomes structural, which is why `PR-B-1`'s remedy item 4 pins its migration rather than leaving it to the implementer.

## Overall assessment

THE ROUND'S REAL RESULT. Both lenses found real defects and neither invented one. The round produced ten valid findings after dedup, at a `high` ceiling, with nothing dismissed. The two `high`s are independent and land in different parts of the step: one says the acceptance list would ratify a regression in the code, the other says a recorded risk classification would let a plan-content increment converge in one round on a premise that two commands falsify. Both are exactly the failure this step exists to remove, one level up: a green that does not mean what it says.

The step's factual spine is otherwise strong. I re-ran the ledger anchors, the exploration citations, the source citations and the gates, and I could not falsify anything the fidelity lens listed as confirmed. The step's discipline of stating properties rather than counts is right, and the one place it breaks its own rule (`PR-A-2`) is the one place a member went missing, which is the discipline earning its keep by its own violation.

IS THE STEP SAFE TO BUILD FROM ONCE THE REMEDIES LAND? YES FOR INC1, CONDITIONALLY. With `PR-B-1`'s five remedy items in place (the new observed-contradiction check on both substrates, item 4 narrowed to the unobserved case, item 6 given a red half, item 1 pinning the mis-scoping test's migration, and check 7 relabelled as a sharing check), inc1's list acquires a red half on the only case where the fixed rule does any work. With `PR-B-3`'s rule-text check, the prose that ships into every scaffolded project can no longer silently contradict the shipped behaviour. Those two together are what make inc1 buildable.

NOT YET SAFE FOR INC5 UNTIL `PR-A-1` LANDS IN BOTH ITS PROSE AND ITS STRUCTURED FORM. The `risk_class = "low_risk"` at `docs/plans/agent-scaffold.plan.toml:1416` is what `validate --workflow` enforces; correcting the sidecar alone would leave the tool requiring one clean round while the plan's prose says two. This is the one remedy in the set that must touch the TOML and not only a sidecar.

INC3 SHOULD NOT BE ENTERED UNTIL `PR-A-2` LANDS, because two human-routed defects would otherwise be lost when the ledger is deleted at task close, and the step's own affirmative count would tell the next reader nothing is missing.

WOULD INC1'S CHECKS, AFTER MY REMEDIES, CATCH EACH MUTATION THE BATTERY RAN?

| mutation | caught before the remedies? | caught after them, and by what |
| --- | --- | --- |
| `mutE`, step axis dropped on W5's side | ONLY BY ACCIDENT: check 1 catches it under the faithful test migration and misses it under the lax one, and the list does not say which to write | YES, DETERMINISTICALLY. The new observed-contradiction check is red on `mutE` and green on `ref` (measured: `fx4` accepted at exit 0 by `mutE`, refused at exit 1 by `ref`), and remedy item 4 pins check 1's fixture so the existing test catches it too. Two independent catches. |
| `mutA2`, the rule deleted outright | PARTLY: check 7 alone, and check 7 is a demonstration the implementer performs on itself | YES. The new check is red on `mutA2` (`fx4` accepted at exit 0), which does not depend on the implementer having a predicate to point at, plus check 7 and the pinned check 1 as before. |
| `mutC`, the lexical rule kept behind a `-fold` allowlist | YES, by check 5 alone (measured: `fx2` still reports the derived step `totally-not-a-step`) | YES, by check 5 AND the new check (`mutC` accepts `fx4` at exit 0). Two catches instead of one, so check 5 stops being a single point of failure. |
| `mutE-pred`, the predicate's increment comparison flipped | This is check 7's instrument, not a candidate build; it reds ten tests on every base | YES, by check 1. Unchanged. |
| `ref` and `ref-lax`, correct builds | pass | `ref` passes everything. `ref-lax` no longer conforms, because remedy item 4 requires the mis-scoping test to keep its subject. That is the intended effect. |

ONE RESIDUAL I AM RECORDING RATHER THAN RAISING AS A FINDING. The new check I am prescribing pins the OBSERVED contradiction. The UNOBSERVED case stays a fork the implementer decides under item 4, and my remedy deliberately does not settle it: the design pass left it open, the human has not decided it, and the step's default to REPORT is argued. What the remedy does change is that the fork can no longer swallow the observed case, which is what made `PR-B-1` and `PR-B-2` the same defect seen twice.
