# `workflow-enforcement-tier-inc4`, round 2, cold complete read

Reviewer lens: a cold read of `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` at `a534d69`, started from the artifact and not from the diff. The question asked of every descriptive sentence: is this true of the tree at `a534d69`?

FIVE VALID FINDINGS: four `medium`, one `low`. Zero `critical`, zero `high`.

None of round 1's eleven valid findings is un-closed. I re-ran the fix for each one that has a mechanical form and all of them hold; the detail is in "Round 1 fix verification" below.

| id | severity | subject |
| --- | --- | --- |
| `R2B-1` | medium | `:157`, "Where NO plan is read there is no root, so the predicate does not fire and every surface behaves as it does today" is false of `status` and `next` |
| `R2B-2` | medium | `:204`, "`active_loop` is `None` ONLY when there are no steps or when every step is terminal" is false; inc2 added a third cause the same file specifies at `:223` |
| `R2B-3` | medium | `:206`, the surviving half of the sweep's negative result, "an `Option::None` serialises as an explicit `null` rather than vanishing", is false for two `NextProjection` fields inc2 added |
| `R2B-4` | medium | `:342`, acceptance check 19's SECOND layout returns the OPPOSITE of its stated result when run as written |
| `R2B-5` | low | `:282` and `:382-387`, the increment's own description and its documentation-impact list both omit the test-file edits the increment made |

## Sections untouched by BOTH the inc4 build pass and its round 1 fix pass

Established from the hunk map, not asserted: `git diff -U0 363ac06 a534d69 -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` changes only lines 44, 46, 52, 102, 129, 133, 139, 173, 189, 195, 201-202, 204, 206, 208, 225, 255, 257, 259, 273, 282, 294, 296, 300, 304, 308-309, 312, 339, 345-348, 352, 356-359, 365-366, 368-369, 374-377 and 382-388 of the current file.

WHOLE SECTIONS NEVER OPENED BY EITHER PASS, which is where I read hardest:

- `## The mechanism, decided rather than chosen here` (`:143-166`). Nothing in it was touched. `R2B-1` is here.
- `### The exact behaviour, per surface` (`:177-188`), except `:189`. `R2B-1`'s supporting sites `:179` and `:182-183` are here.
- `### The field shape and the value vocabulary` (`:210-238`), except `:225`. `:216-224` and `:227-237` are untouched.
- `## Candidate (d) is rejected, with the evidence` (`:239-249`). Entirely untouched. I found nothing wrong in it.
- `## What this step does not fix, and where it goes instead` (`:265-272`), except `:273`.
- The whole `## Acceptance check` list except `:312` (the preamble), `:339` (check 16) and the four checks the pass added at `:345-348`. So checks 1 to 15 and 17 to 20 were never opened by either pass. `R2B-4` is here.
- `## Scope: what this step does not do` (`:389-404`). Entirely untouched. I ran the behavioural claims in it and they all hold.

## `R2B-1` (medium). `:157` says the containment predicate does not fire where no plan is read. It does, on `status` and on `next`.

THE SENTENCE, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:157`, present tense, in the section neither pass opened:

> Where NO plan is read there is no root, so the predicate does not fire and every surface behaves as it does today, which is the answer the no-anchor case above already gets; on `validate --workflow` that case is the match's own `(None, None, _)` arm, already a hard problem for its own reason.

The `validate --workflow` half is true. The "every surface" half is false at `a534d69`. MEASURED, in a fixture outside any repository, on a Markdown-primary `--source` with NO `--plan`, which is exactly the "no plan is read" configuration on `status` and `next`:

```
$ cat mdprim/docs/plans/p.plan.toml      # [meta] primary = "markdown", one step
$ cd mdprim
$ agent-scaffold status --source docs/plans/p.plan.toml --metrics <foreign>/docs/metrics/workflow.jsonl --json
{
  "plan": null,
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project"
}
exit: 0

$ agent-scaffold next --source docs/plans/p.plan.toml --metrics <foreign>/docs/metrics/workflow.jsonl --json
{
  "task": "p",
  "source": "no plan source",
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
  ...
}
exit: 0
```

`"plan": null` and `"source": "no plan source"` are the tool saying it read no plan. `log-not-this-project` is the predicate firing anyway. The surfaces do NOT behave as they did before inc2 on this input.

THE CODE SAYS SO IN AS MANY WORDS, at `src/main.rs:containment_roots`'s doc comment:

> Where NO plan is read, `checked_plan_root` has nothing to derive from, so the rule SUPPLIES a root from the anchors instead ... `status` and `next` reach that same configuration with a Markdown-primary `--source` and no `--plan`, and without this they fall through with no root at all: both filters go vacuous and an explicit `--metrics` or `--ledger-fragment` naming another project is read with nothing to reject it, while `status --resume` refuses the same ledger on the same inputs.

WHY THIS MATTERS AND IS NOT PEDANTRY. The behaviour `:157` describes is the LEAK inc2's own review rounds found and closed. `workflow-enforcement-tier-w2`'s waiver note records the cause as "`resume_roots`, the root-supply policy, was rewritten four times and EVERY rewrite produced a finding in the next round", and `resume_roots`'s doc comment records that an empty root vector on a supplied anchor "leaked another project's artifact through all three surfaces at exit 0". `:157` is the pre-fix design still stated as current fact, in the file the queued validation-constraints step inherits (`:269`, `:271` route work to it from this very section).

TWO SUPPORTING SITES IN THE SAME FAMILY, both also untouched by both passes, both stating the checked-plan root as the whole rule:

- `:179`: "The trigger is the SAME containment predicate the validator's refusal uses (the canonically-derived root of the plan THAT SURFACE READS, and whether the resolved artifact lives under it)." The predicate IS shared (`is_outside_root`, one function). The parenthetical describes only one of the two root-supply policies `containment_roots` holds.
- `:163`: "`status --resume` is the one surface that reads NO plan". `status` and `next` also read no plan in the configuration above, which is the point `containment_roots` exists to handle.

I checked the whole file for a sentence that DOES describe the anchor-root fallback for `status` and `next`: `grep -n "anchor" <sidecar> | grep -i root` returns 13 lines, and the only one that describes a root SUPPLIED from anchors is `:182`, which scopes it to `status --resume` (`Q-55-resumepairing`). Nothing in the file states it for the other two surfaces.

SEVERITY medium. No behaviour is wrong and the SHIPPED prose gets it right (`README.md:236` and the `CHANGELOG.md` `Changed` entry both state the anchor-root fallback for all three surfaces correctly), so no user is misled. What earns medium rather than low is that the sidecar is the durable design record, the sentence asserts the absence of a guard that is present, and it is the exact configuration whose unguarded form was found twice at `high` inside inc2.

IN SCOPE on the round 1 triage's own ruling: the round-3 triager's condition 3, "a stale claim THE INCREMENT'S OWN CHANGE BROKE is in scope regardless of authorship", which `R1C-3` and `R1C-4` were both admitted on. Inc2 is what broke it.

## `R2B-2` (medium). `:204` states an exhaustive cause list for `active_loop` being `None` that inc2 made short by one, in a paragraph the pass edited.

THE SENTENCE, `:204`:

> `active_loop` is `None` ONLY when there are no steps or when every step is terminal.

MEASURED FALSE. A plan with ONE step at `in-progress` (not terminal, so neither stated cause applies) with an unpairable round log:

```
$ agent-scaffold next --json --source <away>/docs/plans/p.plan.toml --metrics docs/metrics/workflow.jsonl
{
  "task": "p",
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
  "active_loop": null,
  "resume_state": null,
  "resume_state_absent_reason": "ledger-absent",
  "no_active_loop_reason": "metrics-not-this-project"
}
exit: 0
```

THE FILE CONTRADICTS ITSELF NINETEEN LINES LATER. `:223` specifies the third cause as a deliberate addition:

> `metrics-not-this-project`, the NEW case: the round log resolved for this plan is not the plan's own, so no loop state can be derived from it.

And the code agrees with `:223`: `src/next.rs:181-182` now reads "`None` when there is nothing to act on (no steps, every step terminal, or a round log this tool cannot vouch for)".

THE PASS TOUCHED THIS PARAGRAPH AND STOPPED ONE SENTENCE SHORT. `git diff 363ac06 a534d69` on `:204` shows the pass converted "says it is `None`" to "SAID it was `None`" in the sentence before, and left the "ONLY" sentence in the present tense. This is the same shape as `R1A-4`, where the pass re-tensed the fifth bullet of a five-item sweep and stopped, and the same shape as `R1A-2`/`R1C-1`, where a recorded exhaustiveness claim had silently inverted.

NOT THE EXCLUDED ITEM, and I checked before raising. `Q-55-currencyscope` put OUT of scope "`src/next.rs`'s 'Every derived part is optional' with its `active_loop` `None` disjunct", and my brief excludes `src/next.rs:162` and `:181-183`. Those are the CODE's doc comments, and inc2 has since corrected them anyway (quoted above). This finding is about the SIDECAR's own assertion at `:204`, which is a different artifact, is inside the file acceptance check 21 governs, and was falsified by this step's own inc2 rather than predating the step.

SEVERITY medium. It is an exhaustiveness claim ("ONLY") about derived output, which is the class this step's own round-3 sweep ruled must be deleted rather than narrowed, and the file states both the claim and its refutation nineteen lines apart. Not high: nothing computes on it and the code comment is now right.

MINIMAL REMEDY, DELETION-CLASS. Delete the sentence. The paragraph's point (the blocked-steps case is not a cause of `None`, so do not add a variant for it) is carried entirely by the sentence before it and the two after it. Deleting loses nothing and re-seeds nothing, which is the remedy class this project measures as safe.

## `R2B-3` (medium). `:206`'s surviving clause still asserts that nothing vanishes from the JSON. Two fields inc2 added do.

THE SENTENCE AS IT NOW STANDS, `:206`:

> WHAT THE SWEEP FOUND NOTHING OF, stated because a negative result is worth recording. No `skip_serializing_if` appears in either `src/next.rs` or `src/main.rs`, so an `Option::None` serialises as an explicit `null` rather than vanishing (visible in the golden as `"resume_state": null`), and the new field must follow that convention: ALWAYS PRESENT, `null` in the normal case.

The premise is true: `grep -c skip_serializing_if src/next.rs src/main.rs` gives `0` and `0`. The CONCLUSION is false at `a534d69`. `src/next.rs:198-199` and `:202-203` carry two `#[serde(skip)]` fields, both added by inc2:

```
$ grep -rn "serde(skip)" src/
src/next.rs:198:    #[serde(skip)]
src/next.rs:202:    #[serde(skip)]
```

`:199` is `metrics_absent_note: Option<String>`, `:203` is `resume_state_absent_note: Option<String>`, and their own doc comment says "Not serialised: `--json` reports the token". Measured on the binary, both vanish rather than serialising as `null`:

```
$ agent-scaffold next --source docs/plans/TEMPLATE.plan.toml --json
{ "task": ..., "metrics": null, "metrics_absent_reason": "log-absent", "active_loop": {...},
  "resume_state": null, "resume_state_absent_reason": "ledger-absent", "no_active_loop_reason": null }
```

No `metrics_absent_note` key, no `resume_state_absent_note` key, in either the `None` case above or the `Some` case (`#[serde(skip)]` drops both).

THIS IS THE SURVIVING HALF OF THE SENTENCE `R1A-2`/`R1C-1` HALF-FIXED. The diff shows the fix deleted the first clause ("`#[serde(skip)]` appears exactly ONCE in the whole of `src/`, at `src/next.rs:NextProjection::no_active_loop_reason`, so there is no second silently-dropped field anywhere") and kept the rest. Both clauses are the same recorded negative result, "nothing in these two files silently drops a field", and both are falsified by the same two fields. The triage confirmed that finding at `medium` on the ground that "a recorded NEGATIVE RESULT that has silently inverted is worse than a stale positive claim, because its whole function is to let a later reader skip a search". That ground applies unchanged to the half that stayed.

SEVERITY medium, on the triage's own recorded reasoning for the twin. I am not claiming the fix pass failed to do what it was told; it did exactly what `R1A-2`'s minimal remedy prescribed. The finding is that the prescribed cut was one clause short of the claim.

MINIMAL REMEDY, DELETION-CLASS. Cut "so an `Option::None` serialises as an explicit `null` rather than vanishing (visible in the golden as `"resume_state": null`), and" so the sentence reads "No `skip_serializing_if` appears in either `src/next.rs` or `src/main.rs`, and the new field must follow that convention: ALWAYS PRESENT, `null` in the normal case." The instruction to the implementer survives, the falsifiable generalisation goes, and nothing is authored.

## `R2B-4` (medium). Acceptance check 19's SECOND layout gives the opposite of its stated result when built as written.

THE CHECK, `:342`, in the untouched block of the acceptance list:

> 19. ACCEPTED COST (ii) IS PINNED AS EXPECTED BEHAVIOUR ON BOTH SURFACES, NOT FIXED: a layout where `<root>/docs/plans` is a SYMLINK to a sibling directory is REFUSED under `validate --workflow` after inc2 ... A SECOND LAYOUT PINS THE LOG SIDE: `<root>/docs/metrics` a SYMLINK to a sibling directory, with the plan where it belongs, gives the same refusal and the same omission.

The first layout reproduces. The second does not, when "a sibling directory" is read the way the first layout uses it, namely a directory beside `docs` INSIDE the project root (accepted cost (ii) at `:257` spells the first layout out as "`<root>/docs/plans` is a symlink to `<root>/elsewhere`").

MEASURED, three layouts, same binary, all outside any repository:

```
# LAYOUT 1 (control): <root>/docs/plans -> <root>/elsewhere
$ agent-scaffold validate --source <root>/docs/plans/p.plan.toml --workflow
--workflow would join .../sym1/docs/plans/p.plan.toml against .../sym1/docs/metrics/workflow.jsonl,
which is not under the plan's project root .../sym1/elsewhere; ...
exit: 1                                          <- REFUSED, as the check says

# LAYOUT 2a, the check as written: <root>/docs/metrics -> <root>/elsewhere (a sibling INSIDE the root)
$ agent-scaffold validate --source <root>/docs/plans/p.plan.toml --workflow
.../sym2a/docs/plans/p.plan.toml vs .../sym2a/docs/metrics/workflow.jsonl: round log line 1: <a W3 problem>
exit: 1                                          <- NOT refused: the check RAN, this is its verdict
$ agent-scaffold status --source <root>/docs/plans/p.plan.toml
plan: 1 steps (1 complete); 0 open-questions items
metrics: 1 records                               <- NOT omitted, the log was read and counted
exit: 0

# LAYOUT 2b, what the suite actually builds: <proj>/docs/metrics -> a directory OUTSIDE <proj>
$ agent-scaffold validate --source <proj>/docs/plans/p.plan.toml --workflow
--workflow would join ... which is not under the plan's project root .../proj; ...
exit: 1                                          <- REFUSED
$ agent-scaffold status --source <proj>/docs/plans/p.plan.toml
metrics: unavailable, the round log ... is not under the plan's project root ...
exit: 0                                          <- OMITTED
```

Layout 2a's exit 1 is NOT the containment refusal: the message is a W3 verdict from my fixture's own record, which is proof the check ran, and `status` counted the log at `metrics: 1 records`, which is the opposite of "the same omission". The reason is structural rather than incidental: containment compares the canonicalised log against the plan's root, and a symlink target inside the root canonicalises to a path that is still inside the root.

THE FILE ITSELF, THE SUITE AND BOTH SHIPPED DOCS ALL STATE IT CORRECTLY, so check 19 is the odd one out rather than the tree being wrong:

- `:257`, accepted cost (ii): "THE COST IS THE DIVERGENCE AND NOT THE LAYOUT: any symlink that makes the canonicalised checked plan and the canonicalised resolved log fall under different roots produces it, on either side".
- `tests/unsafe_pairings_are_refused_and_omitted.rs:1618-1623`, whose own comment reads "Layout 2, the LOG side: `<root>/docs/metrics` is a symlink out of the root", and which builds the target at `<root>/two-metrics` while the project root is `<root>/two`.
- `README.md:236`: "A layout where `docs/plans` or `docs/metrics` is a symlink pointing somewhere the other one is not under will now be refused".
- `CHANGELOG.md`, `## [Unreleased]` / `Changed`: "a layout in which `docs/plans` or `docs/metrics` is a symlink that lands the plan and the log under different real roots".

So the defect is that check 19 reuses one phrase, "a SYMLINK to a sibling directory", for two structurally different placements, and only the first layout is refused under that phrase.

SEVERITY medium. This is an acceptance check whose whole declared function is to PIN an accepted cost so nobody re-raises it, and a reviewer who builds the layout it names measures the opposite of what it asserts. That is the same failure `R1C-4` earned medium for: "A reader sent to verify an accepted cost and finding the opposite behaviour is the failure this section exists to prevent." Not high: the suite's own test builds the right layout and passes, so no gate is broken and no behaviour is wrong.

SCOPE, STATED HONESTLY RATHER THAN ASSUMED. This claim was never true; it is an original error in a check written for inc2, not drift this step's increments caused, so it does not fall under `Q-55-currencyscope` item (1) the way `R2B-1`, `R2B-2` and `R2B-3` do. I raise it because it is a false statement in the artifact under review, because inc4's own review question is "does every claim this step leaves behind match the tree it leaves behind", and because the round 2 brief asks explicitly whether any check is unrunnable as written. The triager should rule on the scope boundary; the fact is measured either way.

MINIMAL REMEDY, TOKEN-LEVEL. In the second layout only, replace "a SYMLINK to a sibling directory" with "a SYMLINK out of the plan's project root", matching the test's own comment. Four words, no new fact, and `:257` already supplies the general statement.

## `R2B-5` (low). The increment's own description and its documentation-impact list both omit the test-file edits the increment made.

`:282` says, of inc4:

> ... `Projection.plan`'s false doc comment at `src/main.rs:Projection` (`Q-55-plandoccurrency`), which is the increment's one source change; and this description.

The increment made THREE comment corrections across TWO files, in a commit whose own subject says so:

```
$ git show --stat --format="%h %s" 218c8c3
218c8c3 docs: correct three stale comment claims for inc4
 src/main.rs                                      | 2 +-
 tests/unsafe_pairings_are_refused_and_omitted.rs | 5 ++---
```

The two test-file edits, confirmed in the diff, are at `tests/unsafe_pairings_are_refused_and_omitted.rs:156` ("the first of inc2's four owed red-then-green cases" lost the count word) and `:1369-1370` (lost "and no test on its serialisation at all", which is the twin the human ruled on under `Q-55-twinsites`).

THE DOCUMENTATION-IMPACT LIST HAS THE SAME GAP, and it reads as exhaustive because it enumerates exclusions. `:382-387` lists THIS FILE, the three sidecars, and `src/main.rs:Projection`'s plan-field doc comment, then "NOT `README.md`, NOT `pack/AGENTS.md` and NOT the deployed `.agents/` copies ... NOT `CHANGELOG.md`". `tests/unsafe_pairings_are_refused_and_omitted.rs` appears in neither the positive list nor the negative one.

CONSEQUENCE, so this is not purely bookkeeping: no acceptance check covers those two edits. Check 21 governs "this file", 21b governs the three named sidecars, 22 governs `Projection.plan`, 23 is the render-and-validate gate. The test-file corrections landed with no check to state them, in an increment whose acceptance list is otherwise complete over its own change set.

THE CLAIM WAS TRUE WHEN WRITTEN, which is what makes it this increment's own failure mode rather than a planning error: `2eb06f5` authored `:282` at 07:09 and `218c8c3` made the test-file edits at 07:45 the same morning, and nothing went back.

SEVERITY low. The edits themselves are correct and I verified both, no reader is misled about the tool, and nothing downstream computes on the count. It is still a finding because "the increment's one source change" is a false enumeration authored by the pass about the pass, which is `:308`'s named failure mode ("a pass that re-tenses a false claim can write a NEW false claim in its place") landing on the pass's self-description.

MINIMAL REMEDY, TOKEN-LEVEL PLUS ONE BULLET. At `:282`, "which is the increment's one source change" becomes "which is the increment's one product-source change". At `:382-387`, add the test file to the list beside the three sidecars, naming the two comment corrections. If the list is not wanted longer, the alternative is DELETION at `:282` of the "one source change" clause, which loses nothing the sentence needs.

## Round 1 fix verification, run rather than read

Every round 1 valid finding with a mechanical form, re-checked at `a534d69`. All eleven hold; none is re-raised.

- `R1A-1`: `checks-runner-worktree-name-collision.md:55` now carries no `{pid}-{nanos}` enumeration. I re-ran the underlying claim rather than only the deletion: `grep -rn "agent-scaffold-" tests/*.rs` gives eleven sites with ELEVEN distinct literal prefixes (`e2ehook`, `hookenv`, `audit`, `validate-toml-only`, `validate-workflow-no-source`, `validate-workflow-no-log`, `validate-workflow-opaque-log`, `validate-projection`, `anchor`, `missingtmp`, `containment`), so "each carries a distinct literal prefix, so they cannot collide today" is true of the current set, not just of the set it was written about.
- `R1A-2`/`R1C-1`: the `#[serde(skip)]` clause is gone from `:206`. The rest of that sentence is `R2B-3`.
- `R1A-3`/`R1C-2`: `:195` now reads "WAS `#[serde(skip)]`" and "HAD no reason field at all".
- `R1A-4`: `:201` and `:202` now read "HAD THE SAME DEFECT" and "WAS SHORT BY ONE".
- `R1A-6`: check 16's quoted command line is now copyable. Both spellings reproduce verbatim at uid 1000: mode-600 `docs/metrics` gives `Permission denied (os error 13)` under `--workflow` at exit 1 and the absent-log note at exit 0 without it; the trailing slash gives `Not a directory (os error 20)` the same way. The directory was restored to 755 and verified.
- `R1B-1`: `:308` reads 13, and `docs/plans/agent-scaffold.plan.toml:1330` (`-w1`) reads "13 valid findings (3, 4, 6)". Reconciled against the log: the three `workflow-enforcement-tier-inc1` round records carry `valid_findings` 3, 4 and 6. The two sibling notes still match their own logs (`-w2` 9+5+6+4 = 24, `-w3` 6+4+2+0+2 = 14), and `prompt-drift-guard` is 4+3+5+1+2+0 = 15 across six rounds, so `:306` and `:308`'s other figure hold too.
- `R1B-2`: "The check is mechanical rather than a reading." is gone from check 21.
- `R1C-3`: `:304` no longer carries the `status --json` no-test claim.
- `R1C-4`: `:255`, `:257` and `:259` are re-tensed. I re-ran (a) and (c) rather than trusting the edit: from inside `docs/plans` a bare `--source` now hard-fails at exit 1 naming the path it looked for, and the conventionless `--source`/`--plan` pair exits 1 with the containment refusal.
- `R1C-5`: the `Q-55` record's seven claims are re-tensed in place and `-w1` carries 13. See the observation below on the one member whose falsity was not a tense.
- `R1C-6`: check 21b now carries "THE EXCLUSION IS THE REPLACED-SUBJECT CLASS ONLY".

## Acceptance checks re-run first-hand

Run against `target/debug/agent-scaffold` built from `a534d69`, in fixtures under my own scratch subdirectory, all outside any git repository.

| check | result |
| --- | --- |
| 1 | `cargo test` exit 0 (`TMPDIR` outside the repo); `render --check` up to date |
| 2 | scaffold gives "30 changed, 0 left untouched"; `ls docs` prints only `plans` |
| 5, 6 | `next` and `status` on the non-instrumented fixture report the fixture's own absent log |
| 12 | symlinked source with a full log beside the symlink: exit 1, refusal naming the real project root |
| 14 (first half) | no `--workflow`, foreign `--source` with a local `--metrics`: exit 0, no refusal |
| 14b, 14c, 14f (precedence) | `next` prints no `ACTIVE LOOP` line and the `no active review loop (<reason>)` form; `status` prints `metrics: unavailable, <note>`; `status --resume` with a foreign `--ledger-fragment` prints the rejected-ledger note and no block; all exit 0. An explicit `--metrics` outside the root naming a file that does not exist gives `log-not-this-project`, not `log-absent` |
| 15 | exit 1, problem names the resolved log |
| 16 | both spellings, both surfaces, as above |
| 17 | empty log plus borrowed slug at `complete`: exit 1 with the W3 message quoted at `:96` |
| 18 | exit 1, a hard failure naming the path |
| 19 | layout 1 passes, layout 2 is `R2B-4` |
| 20 | the non-instrumented fixture's `AGENTS.md` carries the instrumentation qualifier and the exit-code sentence |
| 22 | `status --json --source <TOML-primary>` with no `--plan` serialises a populated `plan` object; `src/main.rs:570` no longer says "present only when a readable `--plan` was given" |
| 23 | render up to date; `validate --source ... --workflow` exit 0, `workflow invariants hold`, 289 records |

NO CHECK OTHER THAN 19 IS UNRUNNABLE OR WRONG AS WRITTEN, on the ones I ran. Checks 3, 4, 7 to 11, 13, 13b, 14d, 14e, 14g, 14h, 19b and 21 to 21b I did not run end to end; see the limits section.

## Citation and quotation sweep, done independently

I re-derived the sweep rather than inheriting round 1's. Every `file:line` citation in the sidecar opened at its range and shown to hold its named subject:

`src/workflow.rs:180-195` (`check_workflow_toml`), `:448-449` (the `round_step_slug(round) == step.slug` filter); `src/plan/source.rs:102` (`#[serde(deny_unknown_fields)]` on `Meta`), `:480-495` (`is_safe_sidecar_ref` with its doc comment); `src/plan/render.rs:296` (`plan.meta.title`), `:167-169` (both `plan.meta.sidecars` accesses); `src/findings_naming.rs:52-55` (`join_dir`, which is where the task name enters the relative path); `tests/validate_workflow_toml_source_needs_no_plan.rs:127-171` (`workflow_with_no_plan_source_hard_errors_instead_of_skipping`, the test pinning the `(None, None, _)` arm); `README.md:210`, `:212-232`, `:238`, `:242-260` (all four exact); `pack/AGENTS.md:61`, `:63`, `:93`, `:116`; `justfile:46-48` (`scaffold-self`, the render then `nix fmt`). Symbol citations all resolve: every named `src/main.rs` and `src/next.rs` item in the file, plus `src/metrics.rs`'s `count_records` and `parse_rounds`.

Structural claims re-measured rather than read: `run_validate`'s `--workflow` block IS a four-arm match over `(toml_primary, &plan_contents, &metrics_contents)` with the four arm patterns the file names (`:46`, `:163`); `project_root_of_source` implements the lexical nearest-wins walk exactly as `:151` describes it; `resolve_metrics_path` anchors `--source` first then `--plan` (`:159`); `--metrics` carries no clap `[default:]` in `validate --help` (`:356`); `pack/AGENTS.md` mentions `docs/metrics/workflow.jsonl` at exactly `:61` and `:63` outside the instrument slot (`:139`); `CHANGELOG.md`'s `## [Unreleased]` has `Added` and `Changed` and no `Fixed` (`:360`, `:370`); `README.md` documents no `next` section (`:366`, `:369`); the explorations directory holds the three named files at 521 + 510 + 483 = 1514 lines (`:12`); the backlog orders 60, 63, 64, 78, 88, 92, 93, 95 and 96 all match `docs/plans/agent-scaffold.plan.toml`; `src/workflow.rs` has no commit from this step, so `:400`'s "It does not change any check logic in `src/workflow.rs`" holds.

Quotations run as literal searches against the file each is attributed to. Every quotation that SHOULD resolve does; every quotation that should NOT resolve is in a sentence whose verb is past or prospective. Two literal-search misses are artifacts rather than staleness and I checked both by hand: `run_validate`'s `(None, None, _)` comment (`:52`) is wrapped across four comment lines in the source, so a single-line search fails while the text matches word for word; and `pack/instrument.md`'s closing line and `README.md:238`'s never-fails sentence miss only if backticks are stripped from the query.

## Three observations that are NOT findings

Recorded so a later reader does not have to rediscover that they were looked at and ruled on.

1. The `Q-55` record at `docs/plans/agent-scaffold.plan.toml:1732` still cites `README.md:228` for the never-fails sentence, while the sidecar's parallel at `:173` was corrected to `README.md:238` by this pass. I checked whether the re-tensed form is false and it is NOT: `git log -S'README.md:228'` puts the citation's arrival at `6141549` on 2026-08-02, and the sentence sat at line 228 from `609ddcf` (08-01) until `b236b10` (08-03), so "`README.md:228` said ..." is historically true. It is a stale-but-true citation, and the two artifacts now disagree on the line number for the same sentence in the rendered view. I raise no finding: the sentence is not false, and `Q-55-receiptcurrency` authorised a tense change only.
2. `tests/unsafe_pairings_are_refused_and_omitted.rs:1372-1373` still reads "`no_active_loop_reason` is `#[serde(skip)]`, and `status`'s `Projection` has no reason field", the present-tense twin of the sidecar `:195` claim `R1A-3` had re-tensed. I do not raise it: the sentence opens "RED against the parent commit:" in the same breath, which is a tighter historical frame than `:195`'s, and re-raising a near-twin of a settled ruling needs better evidence than I have.
3. Check 21's quotation rule is scoped to "source, test, `README.md` or `pack/AGENTS.md` text", so the file's many quotations of the ledger, of the first planner pass and of `Q-55` are outside it. I verified the ledger-attributed ones anyway (`docs/plans/agent-scaffold.ledger.md` holds all six, one differing only in capitalisation) and found nothing. The scoping looks deliberate rather than an omission.

## What this review varied, and what it did NOT reach

VARIED. Plan substrate (TOML-primary, Markdown-primary, no plan resolved at all). `--metrics` state (anchored default, explicit relative, explicit absolute, explicit outside the root, explicit naming a nonexistent file, trailing slash). `--ledger-fragment` present and absent. Layout (conventional, `docs/plans` symlinked inside the root, `docs/metrics` symlinked inside the root, `docs/metrics` symlinked outside the root, plan symlinked out of its project, bare filename from inside `docs/plans`). Working directory (project root, `docs/plans`, a foreign project's root). Probe failure class (EACCES from a mode-600 directory, ENOTDIR from a trailing slash, and the healthy control). Surface (`validate`, `validate --workflow`, `status`, `status --json`, `status --resume`, `next`, `next --json`). Step status (`not-started`, `in-progress`, `complete`).

WHAT I DID NOT REACH, so do not read this round as covering it.

- THE RED HALVES. I built no binary from any parent commit and re-ran no pre-increment reproduction. Every "before inc2 this prints ..." clause in checks 3 to 15 is unverified by me. A red-then-green claim that never went red would survive this round, exactly as it survived round 1.
- UID. Everything at uid 1000 only. Check 16's "at every uid including root" clause for the trailing-slash spelling is unverified here; round 1's part C reports it clean and I did not re-run it under `unshare -Ur`.
- ONE PLATFORM, ONE PROFILE. Linux, local filesystem, debug build, one binary. No concurrency and no TOCTOU case.
- CHECKS 13b, 14e, 14g, 14h AND 19b BY HAND. I exercised those only through `cargo test`, which passes. I built no two-fixture divergent `--source`/`--plan` pairing myself, so a defect the suite's own fixtures happen not to construct would survive.
- THE LEDGER'S OWN CURRENCY. `docs/plans/agent-scaffold.ledger.md` is orchestrator narrative and outside the pass's scope; I read it for evidence but did not sweep it for the same stale claims.
- THE RENDERED VIEW AS A READER MEETS IT. That is another reviewer's lens this round. I confirmed `render --check` reports up to date and did not read `docs/plans/agent-scaffold.md` end to end.
- THE EIGHT RECORDED RESIDUALS (inc2's four and inc3's four). I checked each of my five findings against that list before raising and none is a re-raise, but I did not re-examine the residuals themselves.
- ROUND 1'S FOUR DISMISSALS (`R1A-5`, `R1A-7`, `R1A-8`, `R1B-3`). Not re-raised, and I found no new evidence against any of the four verdicts.

WHAT THE FIVE FINDINGS COVER, stated so the round's product is not overread. I read all 404 lines. Four of the five are in blocks NEITHER earlier pass opened (`:157`, `:206`'s survivor, `:282`, `:342`), and the fifth (`:204`) is one sentence past where a fix pass stopped. That is the pattern, and it is the fourth consecutive round in which a count or an exhaustiveness claim in this file has been found wrong. I would not treat the current figure as exhaustive either.
