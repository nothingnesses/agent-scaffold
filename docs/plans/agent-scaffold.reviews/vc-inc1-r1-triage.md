# `validation-constraints-inc1`, round 1: triage

Triager worktree: `.claude/worktrees/tri-inc1-r1`, branch `triage/inc1-r1`, at `a70411c`.
Artifact: `git diff main..HEAD` in that worktree (`.agents/AGENTS.reference.md`, `AGENTS.md`, `CHANGELOG.md`, `pack/instrument.md`, `src/plan/source.rs`, `src/workflow.rs`).
Specification: `docs/plans/agent-scaffold.steps/validation-constraints.md`, increment 1 and the Acceptance section.
Findings adjudicated: `vc-inc1-reviewer-behaviour.md` (`W1A-*`), `vc-inc1-reviewer-edges.md` (`W1B-*`), `vc-inc1-reviewer-contract.md` (`W1C-*`).

EVERY MEASUREMENT BELOW WAS TAKEN BY ME, IN THE TREES NAMED. Nothing is carried from a reviewer's report on trust. Where my figure differs from a reviewer's, I say so.

## Verdict table

| id | verdict | severity | ground |
| --- | --- | --- | --- |
| `W1A-1` | VALID | `medium` | Reproduced: dropping the increment axis of `waiver_covers_round` leaves 382/382 unit tests green, clippy clean and the live plan green, while returning `workflow invariants hold` at exit 0 over an unconverged `risky` increment. A TEST-COVERAGE GAP, not a code defect. |
| `W1A-2` | DUPLICATE OF `W1B-1` | (`medium`, merged) | Same defect from the contract half only. `W1B-1` is the better statement; `W1A-2`'s sharper distinction about WHICH derived step is carried into the merged remedy. |
| `W1B-1` | VALID | `medium` | Reproduced on both substrates and against this repository's own log: `round_step_slug`'s pre-migration fallback lets W5's refusal name a step that is in neither the plan nor the log, so the recorded `src/` defect is not closed by construction and three claims this diff adds are false. |
| `W1B-2` | VALID | `low` | Reproduced against the live log: the no-records sentence prints for an increment id that five `type:"round"` records carry as their `task`. The verdict is right under the Inc 2 identity model; the sentence is not self-checkable by the reader it is written for. |
| `W1B-3` | VALID | `low` | Reproduced: `step_attribution`'s doc gives free-string authoring as the cause of several owners, and a well-formed structured record plus a well-formed pre-migration record produces two owners with no free-string abuse. Same root cause as `W1B-1`, different site. |
| `W1B-4` | VALID | `low` | Confirmed by reading the shipped clause: the escalation join states both its fallbacks verbatim and the round join states none, in text that ships into every scaffolded project. |
| `W1C-1` | DISMISSED | (`low` as raised) | "could not be marked `complete`" is the specification's own framing for this exact increment (`validation-constraints.md:3`, "which cannot go `complete` until they can be written"). A CHANGELOG that matches its own step's language is not a defect. |

FIVE VALID FINDINGS, ONE DUPLICATE, ONE DISMISSED. Ceiling `medium`. NO `high` AND NO `critical` FINDING WAS RAISED OR FOUND, so no backstop re-check is owed on any dismissal.

## Trees and binaries

All work under `<scratch>/tri1`, one `CARGO_TARGET_DIR` per binary, verified distinct:

```
d86426be1950b9caa6e08f7d8f1d6b24  target-prefix/debug/agent-scaffold    (git archive main)
c09658137c8a0dc71d95c2d48bf796a3  target-postfix/debug/agent-scaffold   (git archive HEAD)
9a4ed144fd80bf55343aea091516b9b3  target-m3/debug/agent-scaffold        (HEAD, increment axis dropped)
9bc11e3da6bcf6a3a3607be5dc61d3a6  target-guard/debug/agent-scaffold     (HEAD plus the proposed test fix)
f517804e23158c32a11469b5b24e2ee3  target-m3guard/debug/agent-scaffold   (both)
```

Suite results, `cargo test --bins`, `TMPDIR` outside every repository:

```
prefix   378 passed; 0 failed
postfix  382 passed; 0 failed
m3       382 passed; 0 failed
guard    382 passed; 0 failed
m3guard  381 passed; 1 failed  (w5_flags_an_increment_waiver_whose_increment_has_no_round_records)
```

`cargo test` (all binaries) on postfix: 382 + 20 + 9 + 5 + 4 + 3 + 1 + 1 + 1 = 426 passed, 0 failed. Matches reviewer B's figure.

## `W1A-1`: reproduced, and it is a test-coverage gap

VALID, `medium`. The reviewer's rating is confirmed, and the argument for `high` is rejected below.

The mutation, applied to a scratch copy of `HEAD` at `src/workflow.rs:430-432`:

```
 	waiver.unit == WaiverUnit::Increment
-		&& waiver.increment.as_deref() == Some(round_increment_id(round))
 		&& waiver.step == round_step_slug(round)
 }
```

THE SUITE, THE LINT AND THE LIVE-PLAN ITEM ALL STAY GREEN:

```
cd <scratch>/tri1/m3 && CARGO_TARGET_DIR=<scratch>/tri1/target-m3 cargo test --bins
-> test result: ok. 382 passed; 0 failed; 0 ignored

cd <scratch>/tri1/m3 && CARGO_TARGET_DIR=<scratch>/tri1/target-m3 cargo clippy --all-targets -- -D warnings
-> exit 0

<scratch>/tri1/target-m3/debug/agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
   (run in the worktree, unmodified plan and log)
-> docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold, exit 0
```

W3 FALSE GREEN, my own fixture `<scratch>/tri1/fx2/fx-b2` (Roadmap step `beta` `complete`; increment `beta-incA` `risky` at peak streak 0, needing 2; increment `beta-incB` converged and carrying the only waiver, which names `beta-incB`):

```
prefix   exit=1  docs/plans/t.md vs docs/metrics/workflow.jsonl: Roadmap step `beta` increment `beta-incA` reached a consecutive-clean streak of 0 but its `risky` risk class needs 2
postfix  exit=1  (identical message)
m3       exit=0  docs/plans/t.md vs docs/metrics/workflow.jsonl: workflow invariants hold
```

A REGRESSION AGAINST THE SHIPPED TOOL AND NOT ONLY AGAINST THE FIX, my own fixture `<scratch>/tri1/fx2/fx-regress` (waiver names step `alpha` and increment `alpha-fold`, which does not strip to `alpha`; the log records only a DIFFERENT increment of `alpha`):

```
prefix   exit=1  round log line 3: increment waiver names step `alpha` but increment `alpha-fold` belongs to step `alpha-fold`
postfix  exit=1  round log line 3: increment waiver names increment `alpha-fold`, which has no `type:"round"` records, so the round log attributes it to no step
m3       exit=0  workflow invariants hold
```

The same shape reproduces where a record's `task` is the waived id while its structured `increment` differs (`<scratch>/tri1/fx/fx-taskonly`): postfix refuses, m3 accepts.

THE PROPOSED FIX, VERIFIED BOTH WAYS. Replacing the `&[]` rounds argument in the first half of `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` (`src/workflow.rs:1570`) with a log that is non-empty but lacks the waived increment:

```
guard    (pristine predicate + the fix)  -> 382 passed; 0 failed
m3guard  (m3 + the fix)                  -> 381 passed; 1 failed
                                            workflow::tests::w5_flags_an_increment_waiver_whose_increment_has_no_round_records
```

So the fix costs nothing and closes the hole, exactly as claimed.

CODE DEFECT OR TEST GAP: TEST GAP, STATED PLAINLY. `waiver_covers_round` as shipped is correct on every one of the sixteen fixture trees I ran across three binaries, including both substrates, the pre-migration path, the plural-owner path and the live plan. Nothing in the tool is wrong today. What is missing is anything that pins the increment axis of the one predicate the whole enforcement tier now rests on. The remedy is therefore test-side, and no line of `waiver_covers_round`, `w3_problems` or `w5_problems` needs to change.

WHY `medium` AND NOT `high`. Severity is absolute impact if left unfixed. Left unfixed, the shipped tool gives the right answer on every input; the exposure is that a later edit ships green. That is `medium`. The argument for `high`, that the demonstrated failure mode is a false green over an unconverged `risky` increment, describes the mutant's impact and not the artifact's, and the four-level scale rates the finding, not the mutation used to expose it.

REMEDY, SCOPED TO THE CLASS (the increment axis of the shared predicate is unpinned on both consumers):

- `src/workflow.rs:1570`, in `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`: give the FIRST assertion a non-empty log that lacks the waived increment, so the test distinguishes "no records for THIS increment" from "no records at all". Keep the second half (the same waiver with its own records present, accepted) unchanged. Verified above.
- `src/workflow.rs`, the `w3_problems` tests: ADD one case in which a `complete` step carries two increments, one short and one covered by a correctly-scoped increment waiver naming the OTHER increment, asserting the short one is still reported. No such case exists today, which is the W3 half of the same hole. This is a new test, not an edit to an existing one.
- `src/workflow.rs:426-433`, `waiver_covers_round`: NO EDIT. Correct as shipped.
- `src/workflow.rs`, `a_step_waiver_does_not_exempt_a_short_streak_increment`: NO EDIT. It pins the UNIT axis and keeps its subject.
- `src/workflow.rs`, `a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment`: NO EDIT. It pins the STEP axis and keeps its subject.
- `docs/plans/agent-scaffold.steps/validation-constraints.md`, Acceptance item 4b: NOT EDITED BY THIS INCREMENT. Item 4b pins the step axis and has no increment-axis sibling, which is why the mutation passes the whole list. The sidecar is deliberately left stale here and a planner updates it after merge; RECORD FOR THAT PLANNER that the list needs an increment-axis item, because the two test additions above satisfy the gap in the suite but leave the acceptance list itself asymmetric.

## `W1B-1` (with `W1A-2` merged): reproduced, on both substrates and on the live log

VALID, `medium`. This is the first matter I was asked to settle.

THE FACT HOLDS. `round_step_slug` (`src/workflow.rs:119-121`) resolves a record's step axis to its structured `step` when present and to `leading_slug(&round.task)` otherwise. `w5_problems` builds `owners` from that accessor (`src/workflow.rs:612-616`), so on a pre-migration record the step W5's refusal names is COMPUTED FROM A `task` STRING and is not read from the log.

Fixtures are mine, under `<scratch>/tri1/fx` and `<scratch>/tri1/fx4`, one project root each. Command in every case: `agent-scaffold validate --plan docs/plans/t.md --workflow` (or `--source docs/plans/t.plan.toml --workflow` for the TOML case).

Case 1, the derived step is in neither the plan nor the log (`fx-premig`; Roadmap has `alpha` and `beta`; one pre-migration round with `"task":"alpha-fold"`):

```
postfix  round log line 3: increment waiver names step `alpha` but the round log attributes increment `alpha-fold` to step `alpha-fold`
```

Case 2, the derived step is a literal substring of the increment id and appears nowhere in the log (`fx-substr`; one pre-migration round with `"task":"gamma-incidental"`):

```
postfix  round log line 3: increment waiver names step `alpha` but the round log attributes increment `gamma-incidental` to step `gamma`
```

Case 3, the derived step is an unrelated record's raw `task` (`fx-othertask`; a record with a structured `increment` of `alpha-fold`, NO `step`, and `"task":"zzz-task"`):

```
postfix  round log line 3: increment waiver names step `alpha` but the round log attributes increment `alpha-fold` to step `zzz-task`
```

Case 4, THE TOML SUBSTRATE, which is the path the recorded `src/` defect was scoped to (`fx4/toml-premig`; step `alpha` declares increment `alpha-fold` and nests the waiver on it, and the log carries one pre-migration round for `alpha-fold`):

```
prefix   TOML waiver `w`: increment waiver names step `alpha` but increment `alpha-fold` belongs to step `alpha-fold`
postfix  TOML waiver `w`: increment waiver names step `alpha` but the round log attributes increment `alpha-fold` to step `alpha-fold`
```

THAT LAST PAIR SETTLES IT. The ledger records the defect as "W5's ownership message ASSERTS AN OWNERSHIP FACT THAT NEED NOT BE TRUE OF THE PLAN, naming a Roadmap step that does not exist, ON EVERY FIRING UNDER `primary = \"toml\"`" (`docs/plans/agent-scaffold.ledger.md:571`). Post-fix, on `primary = "toml"`, the message still names `alpha-fold`, which the plan does not contain.

REPRODUCED AGAINST THIS REPOSITORY'S OWN LOG, in my worktree. I copied the live `docs/metrics/workflow.jsonl` to a scratch project and appended one `type:"waiver"` record naming the real step `structured-skeleton` for increment `q70-capture`:

```
postfix  round log line 319: increment waiver names step `structured-skeleton` but the round log attributes increment `q70-capture` to step `q70-capture`

grep -c '^slug = "q70-capture"$' docs/plans/agent-scaffold.plan.toml   -> 0
grep -c '"step":"q70-capture"' docs/metrics/workflow.jsonl             -> 0
```

`q70-capture` is a step in no plan and a `step` value on no record. The tool derived it lexically, from the shim the fix set out to retire.

POPULATION, MEASURED IN MY WORKTREE:

```
grep -c '"type":"round"' docs/metrics/workflow.jsonl                     -> 236
grep '"type":"round"' docs/metrics/workflow.jsonl | grep -c '"step":'    -> 123
jq -r 'select(.type=="round") | if .step then "structured" else "premigration" end' docs/metrics/workflow.jsonl | sort | uniq -c
   -> 113 premigration
      123 structured
```

113 of 236 confirmed, exactly as reported. Every round record of every project that adopted the tool before Inc 2 reaches the message through the fallback.

I ALSO MEASURED WHICH LIVE FOLD TOKENS ARE AFFECTED, because it bears on the unblocking:

```
jq -r 'select(.type=="round" and ((.increment // .task)|test("fold$"))) | [(.increment // .task), (.step // "NO-STEP")] | @tsv' docs/metrics/workflow.jsonl | sort -u
   workflow-enforcement-tier-fold              workflow-enforcement-tier
   workflow-enforcement-tier-endproperty-fold  workflow-enforcement-tier
   decision-folder-currency-fold               decision-folder-currency
   plan-fold                                   NO-STEP
   q59-backlog-fold                            NO-STEP
   q64-q65-fold                                NO-STEP
   q66-q67-fold                                NO-STEP
   vc-step-fold                                NO-STEP
```

THE TWO BLOCKING TOKENS CARRY A STRUCTURED `step`, so the unblocking this increment exists to deliver is unaffected. Five other logged fold tokens do not, so the defect's live surface is real but is not the unblocking.

WHICH CLAIMS ARE FALSIFIED. I make it THREE in the diff, not four, plus one in the sidecar. My correction to the reviewer's list is on the test comment.

1. FALSIFIED. `src/workflow.rs:610-611`, added by this diff: "The steps the log DOES join this increment to, so the refusal names what the records say instead of a step derived from the id." In `fx-substr` the refusal names `gamma`, which no record contains and which IS derived from the id.
2. FALSIFIED, AND THIS IS THE CLEANEST REFUTATION. `src/workflow.rs:566`, added by this diff: "the refusal states a fact the records carry instead of a substring of the id." In `fx-substr` the refusal states `gamma`, a literal substring of `gamma-incidental` and a fact no record carries.
3. FALSIFIED. `CHANGELOG.md:32`: "A refusal now names the step or steps the records actually attribute the increment to."
4. NOT FALSIFIED, BUT MISLEADING BY IMPLICATION, AND REWORKED IN THE SAME SENTENCE. `CHANGELOG.md:32`: "its refusal named a step derived from the id, which need not be a Roadmap step", given as the RETIRED rule's defect. True of the retired rule. It reads as a defect the new rule cures, and the new rule cures it only on the empty-owners branch.
5. NOT FALSIFIED, NO EDIT OWED. `src/workflow.rs:1564-1566`, the test comment on `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`: "The message must assert a fact the records carry, so it names no step at all: the retired rule reported a step derived from the id, which need not exist in the plan." Scoped to the EMPTY-OWNERS branch, which is the branch that test owns, every word of this is true, and that branch genuinely IS closed by construction: it names no step. I dissent from the reviewer here.
6. FALSIFIED, OUTSIDE THIS INCREMENT'S EDIT SCOPE. `docs/plans/agent-scaffold.steps/validation-constraints.md:23`: the recorded `src/` defect "is closed BY CONSTRUCTION by inc1 and is not separately scheduled". It is closed on the empty-owners branch and live on the non-empty branch. The sidecar is deliberately left stale by this increment and a planner updates it after merge, so its staleness is not a finding, but THIS SPECIFIC FACT IS RECORDED FOR THAT PLANNER BECAUSE THE PASS WOULD OTHERWISE DELETE A LIVE DEFECT FROM THE PLAN.

THE OUT-OF-SCOPE PRECEDENT DOES NOT APPLY, and I checked all four conditions. (1) Provenance: `round_step_slug`'s fallback predates the base commit and this diff does not touch it, so condition 1 holds. (2) No commit in range modifies the claim's lines: FAILS. Every claim listed above is ADDED by `a70411c`; `git diff main..HEAD -- src/workflow.rs CHANGELOG.md` shows lines 566, 610-611 and the `Fixed` entry as additions. (3) THE SUBJECT IS INDEPENDENT: FAILS, and this is the condition that does the real work. The subject is the increment's own headline promise, and the increment's own review question is "does the ownership rule now assert a fact the round log records". (4) No shared fix with an in-scope finding: FAILS; the fix is to the claims this diff adds. Three of four conditions fail, so the finding is squarely in scope.

I DO NOT REOPEN DIRECTION (iii) OR `Q-70-emptycase`, and I have no evidence beating either. Direction (iii) is confirmed implemented as chosen: I verified `Q-70` and `Q-70-emptycase` in `docs/metrics/workflow.jsonl` and the reporting branch fires unconditionally (`fx-regress` and `fx-taskonly` both report with a non-empty log). This finding is about what the change SAYS, not about which relation it uses.

REMEDY, SCOPED TO THE CLASS (every site that promises the refusal names a step the RECORDS carry, when a pre-migration record's step is derived by `leading_slug(round.task)`). THE VERDICT LOGIC MUST NOT CHANGE: `waiver_covers_round`, `round_step_slug` and the `owners` membership are all correct and are what the receipted direction (iii) decided. The remedy is the message plus the claims.

- `src/workflow.rs:624-629`, the non-empty-owners format string: MAKE THE MESSAGE HONEST ABOUT ITS PROVENANCE. I RULE FOR TREATING THE MESSAGE RATHER THAN ONLY WEAKENING THE CLAIMS, because only that closes the recorded `src/` defect the plan says inc1 closes, it costs one clause plus one test, and it sits inside inc1's own review question. Disclose the derivation on the fallback path, for example by naming the record the owner came from, or by distinguishing an owner read from a record's `step` from one derived from its `task`. The implementer chooses the wording and pins it with a fixture built on a pre-migration record; what is NOT acceptable is a message that continues to present a `leading_slug` product as what the records attribute.
- `src/workflow.rs:610-611`, the inline comment on the `owners` computation: restate over the whole comment, not the quoted fragment, so it says what the accessor does (structured `step` when present, `leading_slug(task)` otherwise).
- `src/workflow.rs:562-570`, the `w5_problems` doc comment's ownership bullet: same restatement over the whole bullet, including the "instead of a substring of the id" clause at `:566`.
- `CHANGELOG.md:32`, the whole `### Fixed` paragraph: rework the two sentences at issue together, so the retired rule's defect and the new rule's guarantee are stated with the same scope. The narrowing sentence and the "no waiver committed to this project's own plan is affected" sentence are both correct and stay.
- `src/workflow.rs:1564-1566`, the empty-case test comment: NO EDIT. Accurate as scoped to its own branch (see item 5 above).
- `src/workflow.rs:411-425`, `waiver_covers_round`'s doc comment: NO EDIT. It already says "a pre-migration record falls back per axis", which is the honest statement, and is the reason this is a claims defect rather than a design defect.
- `src/workflow.rs:104-113`, the four-accessor comment block: NO EDIT. Pre-existing, accurate, and the source of truth the other comments must be made to agree with.
- `docs/plans/agent-scaffold.steps/validation-constraints.md:23`: NOT EDITED HERE, recorded above for the post-merge planner.

## `W1B-2`: valid at `low`

The empty-owners branch prints "increment waiver names increment `X`, which has no `type:\"round\"` records". `round_increment_id` prefers the structured `increment`, so a record whose `task` IS `X` while its `increment` is something else does not count.

Reproduced against the live log in my worktree, same method as above with a waiver for increment `backlog-clearing`:

```
postfix  round log line 319: increment waiver names increment `backlog-clearing`, which has no `type:"round"` records, so the round log attributes it to no step

grep -c '"task":"backlog-clearing"' docs/metrics/workflow.jsonl  -> 5

jq -r 'select(.type=="round" and .task=="backlog-clearing") | [.task, (.step // "NO-STEP"), (.increment // "NO-INCREMENT")] | @tsv' docs/metrics/workflow.jsonl
   backlog-clearing  document-receipt-task-convention          document-receipt-task-convention-inc1
   backlog-clearing  formatter-reflow-wording-polish           formatter-reflow-wording-polish-inc1
   backlog-clearing  reconcile-baseline-doc-drift              reconcile-baseline-doc-drift-inc1
   backlog-clearing  soften-writer-agent-framing               soften-writer-agent-framing-inc1
   backlog-clearing  acceptance-doc-currency-phrasing-polish   acceptance-doc-currency-phrasing-polish-inc1
```

Also reproduced minimally at `<scratch>/tri1/fx/fx-taskonly`, one record with `"task":"alpha-inc1","increment":"alpha-inc2"`.

I RULE THE SENTENCE TRUE AND THE MESSAGE STILL DEFECTIVE. Read against the Inc 2 identity model, "the increment has no round records" is correct, and no verdict is wrong. The defect is that the message's only purpose is to tell an author what to do, and the first thing an author does is grep the log for the id, which contradicts it. `low` is right.

REMEDY: `src/workflow.rs:618-621`, the empty-owners format string, over the whole sentence: say that no record RESOLVES to that increment id, so the reader knows the identity being matched is the structured `increment` (falling back to `task`) rather than any occurrence of the string. This lands in the same edit as `W1B-1`'s message treatment and the two must be worded together. No other site.

## `W1B-3`: valid at `low`

`src/workflow.rs:544-545`: "Several is authorable on the JSONL substrate, where a record's `step` is a free string." That is one route to several owners. The other needs no free-string abuse: one record carrying a structured `step` and one pre-migration record for the same increment.

Reproduced, `<scratch>/tri1/fx/fx-plural`, exactly that pair, waiver naming `beta`:

```
postfix  round log line 4: increment waiver names step `beta` but the round log attributes increment `alpha-fold` to steps `alpha`, `alpha-fold`
prefix   round log line 4: increment waiver names step `beta` but increment `alpha-fold` belongs to step `alpha-fold`
```

`alpha-fold` is not a Roadmap step, and the second owner exists only because the second record resolves through the fallback.

Same root cause as `W1B-1`. KEPT AS ITS OWN FINDING RATHER THAN RULED A DUPLICATE, deliberately: it is a different sentence in a different function, and a fix pass that treats only `W1B-1`'s sites leaves it stating a cause that is no longer the only one.

REMEDY: `src/workflow.rs:543-545`, `step_attribution`'s whole doc comment: state both routes, the free-string `step` and the per-axis fallback. No other site.

## `W1B-4`: valid at `low`

Confirmed by reading the shipped clause in my worktree. The escalation join spells out both of its fallbacks:

```
... the escalation's structured `increment` id, or its `task` when that id is absent, equals the waived increment;
or its structured `step` slug, or `leading_slug(task)` when that id is absent, equals the waived step ...
```

and the new ownership rule states none:

```
... an `increment`-unit waiver's `step` must own its `increment` (the round log must join that increment to that step,
so an increment with no round records at all is reported) ...
```

A reader of a scaffolded project's `AGENTS.md` is told one join degrades and left to infer the other does not. Both do. This text ships into every project the tool scaffolds, which is why it is a finding rather than an omission.

REMEDY, over the whole `type: "waiver"` bullet's W5 sentence, in ALL THREE COPIES, which must move together or the drift guard fails:

- `pack/instrument.md`, the pack source.
- `AGENTS.md`, the generated copy.
- `.agents/AGENTS.reference.md`, the generated copy.

CARRY THE REGENERATION HAZARD THE STEP RECORDS: do not run `just scaffold-self` naively, because its second line runs `nix fmt` over a tree that is not formatter-clean at HEAD. Run the render half alone, or regenerate and commit only the three rule files. Acceptance item 7b's fixed-string command must be re-run after the edit, since the replacement wording it greps for is the sentence being changed.

## `W1C-1`: dismissed

The claim is that `CHANGELOG.md:32`'s "the step carrying it could not be marked `complete`" overstates, because nothing mechanically ties step status to `validate --workflow`'s exit code.

I confirmed the mechanism half of the reviewer's evidence in my worktree: there is no `.github` directory, the only YAML in the tree is `pack/checks/ast-grep/sgconfig.yml` and `pack/checks/ast-grep/rules/no-dbg-macro.yml`, and `pack/hooks/pre-commit` ends in `exec agent-scaffold checks --staged`, not `validate --workflow`. So no mechanism prevents typing `status = "complete"`.

DISMISSED ANYWAY, on a ground the reviewer did not reach. The specification for this exact increment uses the identical framing, in the same words, at `docs/plans/agent-scaffold.steps/validation-constraints.md:3`:

```
Two are owed on `workflow-enforcement-tier`, which cannot go `complete` until they can be written.
```

A CHANGELOG entry that describes an increment in the language of that increment's own step is not making a false mechanical claim; it is using this project's established meaning of "marked complete", which is "marked complete with the deterministic gate agreeing". The reviewer's own mitigating paragraph reaches the same place from the `### Changed` precedent at `CHANGELOG.md:23`. The specification citation settles it.

This dismissal is `low`, well below the backstop severity, so it needs no second triager.

## Ruling: the `CHANGELOG` placement recommendation

The contract reviewer recommended, as a recommendation and not a verdict, that the new `### Fixed` section instead amend the existing `Added` bullet at `CHANGELOG.md:13`, on one-source-of-truth grounds, since the check has never shipped in a tagged release and `Q-55-changelog` chose no separate entry for an in-cycle correction.

I VERIFIED THE REVIEWER'S FACTS IN MY WORKTREE AND THEY HOLD:

```
git log main --oneline -S'### Fixed' -- CHANGELOG.md   -> no output (never on main)
git log --all --oneline -S'### Fixed' -- CHANGELOG.md  -> a70411c, fe5b31a (this change on two branches only)
```

Only `[0.0.1]` is released and it predates W5. The `Added` bullet at `:13` is mechanism-agnostic ("an increment-unit waiver's `step` owns its `increment`") and was never false.

I RULE AGAINST THE RECOMMENDATION. KEEP THE `### Fixed` SECTION. Four grounds, strongest first.

1. THE PRECEDENT DOES NOT REACH THIS CASE ON ITS OWN RECORDED REASONING. `Q-55-changelog` (`docs/plans/agent-scaffold.ledger.md:711`, receipt record 305) chose "No entry, and do not re-home the exclusion" for five `--help` strings, and its recorded ground is explicit: "every one said something FALSE and now says something TRUE, WITH NO BEHAVIOUR ALTERED, which the implementer verified byte-identical before and after, so these are documentation fixes rather than behaviour changes". Inc1 alters behaviour. I measured verdict changes myself in both directions: `fx-regress` and `fx-taskonly` go from accept to refuse, and the reviewer's own matrix records ten trees changing verdict over forty-five. Applying a precedent past its own stated ground is a misuse this project's ledger already records as a recurring class.
2. THE SPECIFICATION NAMES THE SITE AND STATES THE OBLIGATION, AND THE IMPLEMENTER MET IT. `validation-constraints.md:145`, INC1's Documentation impact: "`CHANGELOG.md`, the `## [Unreleased]` section, WHICH HAS `Added` AND `Changed` AND NO `Fixed`. The narrowing is a behaviour change to `validate --workflow` and the population it affects must be named." The clause noting the absent `Fixed` reads as the gap this entry fills, and nothing in the step directs an amendment to `Added`. A triager overturning a placement the step itself points at needs more than a preference.
3. PRINCIPLE 16'S RISK IS SMALL HERE. The two entries do not state one fact twice: `Added` states what the W5 check IS, mechanism-agnostically, and `Fixed` states a mechanism change and a narrowing. A future edit to the mechanism has ONE site to reach, not two, precisely because `Added` names no mechanism. The two-sources-for-one-fact problem the principle warns against is not the shape in front of us.
4. AMENDING WOULD COST THE NARROWING ITS VISIBILITY OR THE `Added` BULLET ITS SCOPE. The spec requires the affected population be named. Folding "an increment with no round records at all is now reported" into a bullet that describes the check's introduction either buries a behaviour change in a feature description or forces that bullet to carry both the old and the new mechanism, which is worse than two entries.

THE REVIEWER'S REAL CONCERN IS RECORDED AND ROUTED RATHER THAN DISMISSED. A `Fixed` entry saying "previously compared the increment id's leading slug" describes a state no released reader ever had, and when `[Unreleased]` is cut to a release that reads oddly. THAT IS A RELEASE-CUT CONCERN AND NOT A DEFECT IN THIS DIFF: the correct moment to collapse or merge `[Unreleased]` entries that describe an in-cycle transition is when the release is cut, with the whole section in view. RECORDED HERE FOR WHOEVER CUTS THE NEXT RELEASE, since no other artifact carries it.

## Overall assessment

THE ROUND'S REAL RESULT: NOT CLEAN. Five valid findings, ceiling `medium`, no `high` and no `critical`. The round outcome is `new_valid` and the consecutive-clean streak stays at 0 against the two required for a `risky` artifact.

IS THE SHIPPED BEHAVIOUR CORRECT EVEN WHERE ITS TESTS AND CLAIMS ARE NOT? YES, AND I MEASURED IT RATHER THAN ARGUING IT. Across sixteen fixture trees on both substrates, three binaries each, plus the unmodified live plan and log, I found NO tree on which the shipped build returns a wrong verdict: no false green and no false red beyond the narrowing `Q-70-emptycase` decided. Direction (iii) is implemented as receipted, the reporting form of the unobserved case fires unconditionally, W3's verdict is unchanged, the two blocking fold tokens carry structured `step` ids so the unblocking works, and the three shipped rule-text copies match acceptance item 7b's fixed-string check (0, 0, 0 for the retired wording and 1, 1, 1 for the replacement). `cargo test` is 426 green and clippy is clean.

EVERY VALID FINDING IS IN WHAT THE CHANGE SAYS OR IN WHAT PINS IT, NOT IN WHAT IT DOES. `W1A-1` is a test-coverage gap over correct code. `W1B-1`, `W1B-2`, `W1B-3` and `W1B-4` are four sites where the diff, or the text it ships, describes the join more strongly than the code guarantees. The single root cause under three of them is `round_step_slug`'s documented per-axis fallback, which is correct, deliberate and pinned, and which the new prose forgot.

SAFE TO MERGE ONCE THE REMEDIES LAND? YES. No remedy changes a verdict, `waiver_covers_round` needs no edit, and the whole fix surface is two test additions, three comment restatements, two format strings and one sentence in three prose copies. The one substantive judgement inside the remedy is whether W5's non-empty refusal discloses its derivation, which I ruled it must, and which is inside inc1's own review question rather than a new design decision.

TWO THINGS THE ORCHESTRATOR STILL OWES, NEITHER A FINDING AGAINST THIS ARTIFACT. Acceptance item 3, the plan-side unblocking (the two `[[step.increment]]` declarations, the two owed waivers, the `workflow-enforcement-tier` status flip), is absent from `git diff main..HEAD`; the step assigns those edits to the orchestrator and the planner, so the increment cannot settle item 3 by itself. And the sidecar's line 23 must be re-scheduled rather than deleted at the post-merge planner pass, per `W1B-1`.
