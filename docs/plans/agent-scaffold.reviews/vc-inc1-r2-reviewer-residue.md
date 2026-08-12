# `validation-constraints-inc1`, round 2: reviewer, what the fix pass broke or left half-done

Reviewer worktree: `.claude/worktrees/rev-inc1r2-residue`, branch `review/inc1r2-residue`, at `60ee7d0`.
Artifact: `git diff main..HEAD`, two commits (`6ec9f1a` the implementation, `60ee7d0` the round 1 fix pass).
Specification: `docs/plans/agent-scaffold.steps/validation-constraints.md`, increment 1 and the Acceptance section.
Lens: the residue of the fix pass, plus a fresh mutation battery over the code the fix pass added. Round 1's five valid findings, its one duplicate and its one dismissal are settled and are not re-raised; nothing below claims a round 1 verdict was wrong.

Everything below was measured in scratch trees under the session scratchpad. The worktree carries this file and nothing else.

## Summary

FOUR FINDINGS, ALL `low`. NO `critical`, NO `high` AND NO `medium` FINDING WAS FOUND, stated explicitly.

The fix pass executed every remedy the round 1 triage listed, and it changed no verdict: 36 constructed trees give identical W5 ownership verdicts before and after the fix pass, and the head verdict matches an independently computed join relation on all 36. The three surviving defects are in what the new code SAYS and in what PINS it, which is the same shape round 1 recorded.

The three round 1 remedies I re-measured all landed:

- The increment axis of `waiver_covers_round` is now pinned on both sides (`m10` reddens one W3 test and one W5 test where round 1 measured 382 green).
- The non-empty-owners refusal now discloses a derived owner, and the empty-owners refusal now says what "resolves to" means.
- The three shipped rule-text copies now state the round join's per-axis fallback, and acceptance item 7b still reports 0, 0, 0 for the retired wording.

WHAT THE FIX PASS BROKE OR LEFT HALF-DONE: the per-owner marking rule the fix introduced is stated per RECORD in the prose that ships (`W2A-1`) and in two in-code doc comments (`W2A-3`), a new test comment makes a claim about the suite that the fix pass itself falsified in the same commit (`W2A-4`), and three independent mutations of the new provenance map survive the entire suite, each making the refusal state something false about the log (`W2A-2`).

## Findings

### `W2A-1`: the shipped rule text and the CHANGELOG state the derived mark per RECORD, while the code applies it per OWNER and the fix pass's own new test asserts the per-owner rule

Severity: `low`.

Claim. `pack/instrument.md:11`, `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`, all rewritten by `60ee7d0`, say "a step reached through the `leading_slug` fallback is reported as derived". `CHANGELOG.md:32` says "a refusal naming such a step marks it as derived". Neither is true when a second record declares the same step: the mark is computed once per OWNER (`src/workflow.rs:648-652`, `*seen |= declared`), so one declaring record clears the mark for every record that reached the same value through the shim.

Evidence 1, a run. Fixture `<scratch>/r2res/fx/fx1`: Roadmap steps `alpha` and `beta` both `in progress`; one structured record (`"step":"alpha","increment":"alpha-inc1"`) and one pre-migration record (`"task":"alpha-inc1"`, no structured ids) for the same increment; an increment waiver naming step `beta`.

```
head    round log line 4: increment waiver names step `beta` but the round log joins increment `alpha-inc1` to step `alpha`
```

The second record reached `alpha` through `leading_slug("alpha-inc1")`, and the refusal does not mark it. Compare `<scratch>/r2res/fx/fx4`, the same shape with the structured record removed:

```
head    round log line 3: increment waiver names step `beta` but the round log joins increment `alpha-fold` to step `alpha-fold` (derived from a record's `task`)
```

Evidence 2, the diff contradicts itself. `src/workflow.rs:1767`, added by the same commit, is `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it`, whose fixture is exactly `fx1`'s shape and whose assertion is `!problems[0].contains("derived")`. The test states the per-owner rule; the prose states the per-record rule.

Evidence 3, the three copies, one command each:

```
grep -c -F "a step reached through the \`leading_slug\` fallback is reported as derived" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md
   -> 1, 1, 1
```

Why `low`. No verdict is affected and no reader of a REFUSAL is misled: an unmarked owner is always a value some record carries, so the message itself is true in every case reaching it. What is wrong is the prose's account of when the parenthetical appears, in text that ships into every scaffolded project. The argument for `medium` is that this project classifies prose stating an enforcement boundary as its worst measured class (`validation-constraints.md:101`), and the clause is new in this diff rather than inherited.

Remedy, scoped to the class (every site that states the mark's trigger per record): the W5 sentence in `pack/instrument.md` and its two generated copies, which must move together or the drift guard fails, and the `DERIVATION IS NOT FULLY RETIRED` sentence at `CHANGELOG.md:32`. State the trigger as the code applies it, that no record declares the step in a structured `step` id. Acceptance item 7b's fixed-string command must be re-run after the edit, since the replacement wording it greps for is inside the sentence being changed, and the regeneration hazard the step records applies (do not run `just scaffold-self` naively).

### `W2A-2`: three independent mutations of the new provenance map survive the whole suite, and each one makes W5's refusal state something false about the log

Severity: `low`.

Claim. The owners map at `src/workflow.rs:644-653` is new code written to satisfy a round 1 finding. Three of its inputs are unpinned: which records enter the scan, which structured field decides the mark, and whether the merge is order-independent. Each mutation below leaves 385 of 385 unit tests passing, and each produces a refusal that asserts something the log does not say.

Mutation A, the scan's increment axis (`m9`). Replace `round_increment_id(round) == increment` with `round.task == increment` in the owners scan only, leaving `waiver_covers_round` untouched.

```
cargo test --bins   ->  test result: ok. 385 passed; 0 failed
```

Fixture `<scratch>/r2res/fx/fx3`, one record with `"task":"zzz-task","step":"alpha","increment":"alpha-inc1"` and a waiver naming step `beta` for increment `alpha-inc1`:

```
head  round log line 3: increment waiver names step `beta` but the round log joins increment `alpha-inc1` to step `alpha`
m9    round log line 3: increment waiver names increment `alpha-inc1`, which no `type:"round"` record resolves to (a record resolves to its structured `increment` id, or to its `task` when that id is absent), so the round log joins it to no step
```

The `m9` sentence is false of its own fixture: a record does resolve to `alpha-inc1`, by the rule the sentence itself states. No test notices, because every fixture in the suite that reaches this scan has `task` equal to the increment id (`round_line` sets `task` to the increment, and `owning_round_line` passes the increment as the `task` as well, `src/workflow.rs:830-835`).

Mutation B, the mark's axis (`m16`). Replace `let declared = round.step.is_some()` with `round.increment.is_some()`.

```
cargo test --bins   ->  test result: ok. 385 passed; 0 failed
```

Fixture `<scratch>/r2res/fx/fx2`, two increment-only records (structured `increment`, no `step`, different `task` values) for one increment:

```
head  ... the round log joins increment `alpha-fold` to steps `yyy-task` (derived from a record's `task`), `zzz-task` (derived from a record's `task`)
m16   ... the round log joins increment `alpha-fold` to steps `yyy-task`, `zzz-task`
```

The `m16` message presents two `leading_slug` products as steps the records declare, which is the recorded `src/` defect this increment exists to close, and the suite is green.

Mutation C, the merge rule (`m20`). Replace `*seen |= declared` with a no-op, making the mark first-write-wins rather than order-independent.

```
cargo test --bins   ->  test result: ok. 385 passed; 0 failed
```

Fixture `<scratch>/r2res/fx/fx8`, `fx1` with the two records in the opposite file order:

```
head  round log line 4: increment waiver names step `beta` but the round log joins increment `alpha-inc1` to step `alpha`
m20   round log line 4: increment waiver names step `beta` but the round log joins increment `alpha-inc1` to step `alpha` (derived from a record's `task`)
```

The `m20` message marks a step a record DECLARES as derived. The only fixture for the mixed case, `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it` (`src/workflow.rs:1767`), puts the declaring record FIRST, so it cannot distinguish a union from first-write-wins, even though its own comment says "The mark is per OWNER and not per record".

Why `low`, and the argument for `medium`. The shipped code is correct on every tree I ran, and none of the three mutations can change a verdict: the owners map is built inside the `!rounds.iter().any(waiver_covers_round)` branch and feeds only the message, so the problem COUNT is identical under all three. That puts it a tier below round 1's `W1A-1`, where the unpinned axis was verdict-bearing. The argument for `medium` is that mutation B reproduces, green, the exact recorded `src/` defect the plan says inc1 closes, and that message truthfulness is this increment's own review question.

Remedy, scoped to the class (the owners map's three inputs), test-side only. No line of `w5_problems` needs to change.

- One new fixture kills A and B together: a record carrying a structured `increment` and NO `step`, whose `task` differs from the increment id. Assert the refusal names the owner derived from that record's `task` and marks it derived. `w5_marks_an_owner_derived_from_a_pre_migration_records_task` (`src/workflow.rs:1717`) is the natural home, since it already owns the derived-owner case.
- One line kills C: in `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it` (`src/workflow.rs:1766-1795`, whose log puts the declaring record first at `:1782`), assert the same expectation with the two log lines in both orders, or reverse the existing order.

### `W2A-3`: the fix pass's new doc comments frame the step-axis fallback as belonging to a pre-migration record, which contradicts the accessor block above them and makes `step_attribution`'s new "TWO WAYS" enumeration false

Severity: `low`.

Claim. Two doc comments added by `60ee7d0` describe the step-axis fallback as a property of a wholly pre-migration record:

- `src/workflow.rs:551-555`: "SEVERAL OWNERS ARISE TWO WAYS AND NEITHER NEEDS A MALFORMED LOG. Two records for one increment may carry different structured `step` ids ... Or one record may carry a structured `step` while another is pre-migration".
- `src/workflow.rs:586-588`: "a record carrying the structured Inc 2 ids joins on them, while a pre-migration record resolves its increment through `task` and its step through `leading_slug(task)`".

The accessor block at `src/workflow.rs:98-111`, which the second of those two comments cites as its authority ("per the accessor block above"), states the opposite: "each accessor therefore falls back on its OWN field alone, with no coupling to the other ... an `increment`-only record still resolves its STEP join through the `leading_slug(task)` shim". A record can carry the structured `increment` id and still derive its step.

Evidence, a run that is neither of the two enumerated ways. Fixture `<scratch>/r2res/fx/fx2`, two records for increment `alpha-fold`, each carrying a structured `increment` and no `step`, with `task` values `zzz-task` and `yyy-task`:

```
head  round log line 4: increment waiver names step `beta` but the round log joins increment `alpha-fold` to steps `yyy-task` (derived from a record's `task`), `zzz-task` (derived from a record's `task`)
```

Neither record carries a structured `step`, so route one does not apply, and neither record is pre-migration in the sense the comment means, so route two does not apply. This is the third route, and it is the shape the accessor block already documents and pins (`w3_an_increment_only_round_falls_back_to_the_shim_on_the_unfilled_step_axis`).

This is round 1's `W1B-3` re-appearing in the sentence that replaced it: the old comment gave one cause and the fallback supplied a second, the new comment gives two causes and the partial record supplies a third. NEW EVIDENCE IS NOT CLAIMED AGAINST `W1B-3`'s VERDICT, which was VALID and whose remedy was executed; this is the same class landing on the replacement text.

The shipped prose in the three copies is NOT affected: it states the fallback per axis ("its structured `step` slug, or `leading_slug(task)` when that id is absent"), which is correct. `CHANGELOG.md:32` shares the in-code comments' framing ("a pre-migration record carries no `step` id, so `round_step_slug` still derives its step").

Why `low`. In-code documentation plus one CHANGELOG clause; no behaviour and no verdict. It matters because the file's own recurring defect class is an enumeration that bounds a set to its own size (`validation-constraints.md:13`), and "TWO WAYS" is exactly that shape.

Remedy: state the cause per AXIS rather than per record kind, at `src/workflow.rs:551-555` and `:586-588` and in the corresponding `CHANGELOG.md:32` clause, so the enumeration is a property (a record whose `step` id is absent derives its step) rather than a count of record kinds.

### `W2A-4`: the new W3 test's comment says the suite would be green without it, and the same commit's other new fixture catches the same mutation

Severity: `low`.

Claim. `src/workflow.rs:1029-1031`, added by `60ee7d0`: "Without this case a build that dropped `waiver_covers_round`'s increment comparison would report `workflow invariants hold` at exit 0 over an unconverged `risky` increment, with the whole suite green." The clause was true of the round 1 tree. It is false of this commit, because the fix pass ALSO gave `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` a non-empty log (`src/workflow.rs:1648`), and that fixture catches the same mutation.

Evidence, mutation `m15`: drop the increment axis from `waiver_covers_round` AND mark this test `#[ignore]`.

```
test result: FAILED. 383 passed; 1 failed; 1 ignored
failures:
    workflow::tests::w5_flags_an_increment_waiver_whose_increment_has_no_round_records
```

For comparison, `m10` (the same mutation with both tests live) reddens both:

```
test result: FAILED. 383 passed; 2 failed
failures:
    workflow::tests::an_increment_waiver_does_not_exempt_a_sibling_increment_of_the_same_step
    workflow::tests::w5_flags_an_increment_waiver_whose_increment_has_no_round_records
```

Why `low`. A test comment. Its cost is that a maintainer reading it is told this case is the sole guard on the increment axis, when it is one of two, so weakening either one looks safe from the other's comment.

Remedy: `src/workflow.rs:1029-1031`, one clause. Say that this case pins the increment axis on W3's side and that `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` pins it on W5's, which is also the accurate statement of what the round 1 remedy asked for (two additions, one per consumer).

## The mutation battery

Driver: `<scratch>/r2res/apply.py` (exact-string replacement with an occurrence-count assertion, so a stale anchor fails loudly rather than silently rebuilding the pristine tree) and `<scratch>/r2res/run-mutant.sh`. ONE `CARGO_TARGET_DIR` PER BINARY; all 21 built binaries have distinct md5 sums (21 paths, 21 distinct sums) (`md5sum target-*/debug/agent-scaffold | awk '{print $1}' | sort | uniq -d` returns nothing). `TMPDIR` pointed at `<scratch>/r2res/tmp`, outside every git repository, per the Acceptance preamble.

Control, HEAD unmutated: `cargo test --bins` 385 passed, `cargo test` 429 passed across 9 binaries, `cargo clippy --all-targets -- -D warnings` exit 0. Test binary `08b8867`.

None of the mutations below is reused from round 1 or from the implementer, except `m10` and `m11`, which are re-runs of round 1's `W1A-1` mutation and of acceptance item 4b's own mutation, run to settle that the fix landed.

| id | mutation | caught | tests that went red |
| --- | --- | --- | --- |
| `m1` | the owner merge becomes AND (`*seen &= declared`), so an owner is unmarked only when every record declares it | YES, 1 | `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it` |
| `m2` | the owner merge becomes last-write-wins (`*seen = declared`) | YES, 1 | `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it` |
| `m3` | the mark never fires (`declared = true`), so every owner is presented as declared | YES, 1 | `w5_marks_an_owner_derived_from_a_pre_migration_records_task` |
| `m4` | the mark always fires (`declared = false`) | YES, 3 | `w5_does_not_mark_...`, `w5_marks_an_owner_derived_...`, `w5_names_every_step_the_log_joins_a_waived_increment_to` |
| `m5` | the two branches of `step_attribution` swapped, so a declared owner is marked derived | YES, 3 | as `m4` |
| `m6` | the map insertion seeds `true` (`or_insert(true)`) | YES, 1 | `w5_marks_an_owner_derived_from_a_pre_migration_records_task` |
| `m7` | owner ordering reversed in the message (`.iter().rev()`) | YES, 2 | `w5_marks_an_owner_derived_...`, `w5_names_every_step_...` |
| `m8` | the singular/plural boundary broken (`owners.len() == 1` becomes `!owners.is_empty()`) | YES, 2 | as `m7` |
| `m9` | the owners scan keys on the raw `task` instead of `round_increment_id` | NO, 385 passed | none. See `W2A-2` mutation A |
| `m10` | the increment axis dropped from `waiver_covers_round` (round 1's `W1A-1`) | YES, 2 | `an_increment_waiver_does_not_exempt_a_sibling_increment_of_the_same_step`, `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |
| `m11` | the step axis dropped from `waiver_covers_round` (acceptance item 4b) | YES, 6, spanning W3 and W5 | `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment`, `check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step`, `w5_does_not_mark_...`, `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment`, `w5_marks_an_owner_derived_...`, `w5_names_every_step_...` |
| `m12` | the derived mark deleted from the message text | YES, 1 | `w5_marks_an_owner_derived_from_a_pre_migration_records_task` |
| `m13` | the empty-owners branch made unreachable (`if false`) | YES, 1 | `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |
| `m14` | the owners map keyed on `leading_slug(&round.task)` instead of `round_step_slug` | YES, 3 | `check_workflow_toml_w5_refuses_...`, `w5_marks_an_owner_derived_...`, `w5_names_every_step_...` |
| `m15` | `m10` plus the new W3 sibling test ignored | YES, 1 | `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`. See `W2A-4` |
| `m16` | the mark read off the wrong structured axis (`round.increment.is_some()`) | NO, 385 passed | none. See `W2A-2` mutation B |
| `m18` | the owners scan loses its increment filter entirely | YES, 1 | `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` |
| `m20` | the owner merge becomes first-write-wins (order-dependent) | NO, 385 passed | none. See `W2A-2` mutation C |

`m11` is acceptance item 7's demonstration re-run for this round: one edit inside the shared predicate reddens a W3 test and five W5 tests, so the predicate is genuinely shared and not copied.

Commands:

```
bash <scratch>/r2res/run-mutant.sh <id>
#   copies <scratch>/r2res/head, applies apply.py <id>, builds into
#   CARGO_TARGET_DIR=<scratch>/r2res/target-<id>, prints the md5, runs cargo test --bins
```

## Pre-fix versus post-fix

Three binaries from separate `git archive` extracts, each with its own target directory, confirmed distinct:

```
9f3bc6ae1953ee3e35554784961b950e  target-prefix/debug/agent-scaffold  (main)
b16054cb6dd2b6e9d97f5d9cd9738677  target-mid/debug/agent-scaffold     (6ec9f1a, before the fix pass)
08b886755ce4edd28bd6913b3c699125  target-head/debug/agent-scaffold    (60ee7d0)
```

A 36-tree matrix (`<scratch>/r2res/matrix.sh`): two waived increment ids (`alpha-inc1`, which strips, and `alpha-fold`, which does not) x two waiver steps (`alpha`, `beta`) x nine log shapes (no records; structured join to `alpha`; structured join to `beta`; pre-migration; increment-only with a different `task`; step-only; structured with a different `task`; an unrelated pre-migration record; a record whose `task` is the waived id while its structured `increment` is another). The expectation for each tree is computed by hand from the documented accessors, in the script, and NOT read off the tool.

```
head verdict != independently computed expectation:   0 of 36
mid verdict  != head verdict:                          0 of 36
head ACCEPTS where prefix REFUSED:                     5 of 36
head REFUSES where prefix accepted:                    6 of 36
```

The five new acceptances are `alpha-inc1.beta.L2`, `alpha-fold.alpha.L1`, `alpha-fold.alpha.L5`, `alpha-fold.alpha.L6` and `alpha-fold.beta.L2`. Every one is a tree in which a round record joins the waived increment to the waiver's step while the id does not strip to it, which is direction (iii) and the unblocking. THERE IS NO TREE IN WHICH THE NEW BINARY ACCEPTS AND NO RECORD JOINS. The six new refusals are the documented narrowing (nothing resolves to the waived increment, or the records join it elsewhere).

THE FIX PASS ITSELF CHANGED NO VERDICT, measured rather than argued: `mid` and `head` agree on all 36 trees. It changed message text, the owners data structure, three tests and seven prose passages.

Acceptance re-runs against the live plan and log in this worktree, with the `head` binary:

```
validate --source docs/plans/agent-scaffold.plan.toml --workflow   -> workflow invariants hold, exit 0   (item 2)
render --check --strict docs/plans/agent-scaffold.plan.toml        -> up to date, exit 0                 (item 8)
grep -c -F "the increment's leading slug equals the step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md   -> 0, 0, 0   (item 7b)
grep -c -F "some \`round\` record must join that increment to that step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md   -> 1, 1, 1
cargo clippy --all-targets -- -D warnings                          -> exit 0
git diff main..HEAD | LC_ALL=C grep -cP '[^\t\x20-\x7e]'           -> 0
```

## What the fix pass got right, verified rather than assumed

- Every message the new code can emit was checked against a constructed instance. On the empty-owners branch the sentence is true whenever it is reached, since `owners` is empty exactly when no record resolves to the waived increment, and the parenthetical states the resolution rule the code uses. On the non-empty branch an owner can never equal the waiver's own `step` (a record carrying both would have satisfied the predicate), an unmarked owner is always a value some record carries in a structured `step` id, and a marked owner was always computed by `leading_slug` from some record's `task`. I found no input on which a shipped message is false.
- The round 1 remedy for `W1B-2` landed and is measurable: `<scratch>/r2res/fx/fx7`, a record whose `task` is the waived increment while its structured `increment` is another, now returns "which no `type:\"round\"` record resolves to (a record resolves to its structured `increment` id, or to its `task` when that id is absent)".
- The empty-case test kept its second half (the same waiver accepted once its own records are present, `src/workflow.rs:1658-1662`), which the triage's remedy required.
- The two touched test comments that claim an owner is unmarked because the record declares it (`src/workflow.rs:1598-1599` and `:2129-2131`) are accurate of their fixtures, which use `owning_round_line` and so carry a structured `step`.

## Examined and NOT raised

- The two tests just named assert `contains("... to step `beta`")`, which would still pass if a derived mark were appended, so neither pins the unmarkedness its comment asserts. NOT RAISED as its own finding: the property IS pinned, by `w5_names_every_step_the_log_joins_a_waived_increment_to` and `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it`, both of which assert `!contains("derived")`, and mutation `m4` reddens them.
- Owner de-duplication and owner ordering are pinned: `m7` and `m8` are both caught, and `fx1` shows two records resolving to one owner producing the singular "step `alpha`".
- The W3 doc bullet at `src/workflow.rs:451-452` describes the increment-waiver exemption as "`increment == <that increment's task>`", which is looser than `round_increment_id`. PRE-EXISTING: it is not in `git diff main..HEAD`, and pre-existing false doc claims are out of scope for this round.
- W5's ownership can now be satisfied by appending a round record that declares a step, which the retired lexical rule could not be. That is direction (iii) as the human decided it, and round 1 confirmed it implemented as receipted; it is not a defect in this artifact.
- The rounds slice W5 reads is unfiltered by project. That is inc6's subject and the step already records it as inc6's fourth limitation.
- The step's sidecar and the `Q-70` plan entry are known to be left stale by this increment, with a planner updating them after merge.
- The plan-side unblocking (the two `[[step.increment]]` declarations, the two owed waivers, the status flip) is absent from `git diff main..HEAD`; the step assigns those edits to the orchestrator and the planner.
- Line length and prose wrapping, pre-existing import-ordering drift, and anything belonging to increments 2 to 6.

## Round outcome

`new_valid`. Four valid findings, ceiling `low`, no `medium`, no `high` and no `critical`. The artifact is `risky` and needs two consecutive clean rounds, so this round does not advance the streak.
