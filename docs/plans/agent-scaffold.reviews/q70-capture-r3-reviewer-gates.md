# Q-70 capture, round 3, mechanical-detectability lens

A MEASUREMENT, not a defect hunt. The question: of the twenty valid findings rounds 1 and 2 produced, plus the orchestrator defects recorded alongside them, how many could any gate this repository already owns have caught?

THE ANSWER IS ZERO OF TWENTY, and zero of the four orchestrator defects recorded alongside them. Twenty-four defects, no gate. That is demonstrated rather than reasoned: the two historical defect states are reconstructed from the branch's own commits, every gate is run over each of them, and every gate passes. A negative control fires each of those same gates on a defect they should catch, so the rig is falsifiable.

TWO FINDINGS ARE RAISED, `R3C-1` at `medium` and `R3C-2` at `low`. Both are against the item's account of the three detection mechanisms, and both come out of the measurement rather than from reading the prose. NO FINDING AT `high` OR `critical`. No settled finding from rounds 1 or 2 is re-raised.

Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-q70r3-gates`, branch `review/q70r3-gates`, confirmed by `git rev-parse --abbrev-ref HEAD` before the first write. Binary: `target/debug/agent-scaffold` built from this worktree at HEAD. Fixture root: `<S>` for `<scratch>/gates-r3/`. Nothing outside `<S>` was written or deleted, and no fixture was created at mode 000 or 600.

---

## 1. The gate inventory

Enumerated from the repository rather than from the brief. `justfile`, `.agents/checks.toml`, `src/`, `tests/`, and the installed hook set were all read.

| gate | invocation | what it reads | hard or soft |
| --- | --- | --- | --- |
| G1 test suite | `cargo test` (`just test`) | `src/`, `tests/`, `pack/`, the committed `AGENTS.md` and `.agents/` | hard, exit 101 on failure |
| G2 lint | `cargo clippy --all-targets -- -D warnings` (`just clippy` omits `-D warnings`) | `src/`, `tests/` | hard, exit 101 |
| G3 validate, plain | `agent-scaffold validate --source <plan.toml> --metrics <log>` | the JSONL record schema, the TOML source schema and its internal cross-references | hard, exit 1 |
| G4 validate, workflow | the same plus `--workflow` | W3 step convergence, W4 decision receipts, W5 waiver integrity, and the round log's streak arithmetic | hard, exit 1 |
| G5 render check | `agent-scaffold render <plan.toml> --check --strict` | the plan TOML plus its sidecars against the committed generated `<task>.md` | hard ONLY with `--strict`; `--check` alone warns at exit 0 |
| G6 checks runner | `agent-scaffold checks` | `.agents/checks.toml`, which declares exactly one row today, `render-check`, running G5 with `--strict` | hard, exit 1 |
| G7 ASCII sweep | `LC_ALL=C grep -nP '[^\t\x20-\x7e]' <file>` | whatever file it is pointed at | a house rule run by hand; no runner invokes it |

DRIFT GUARDS ARE A SUBSET OF G1, NOT A SEPARATE GATE, and their coverage is stated in the repository rather than inferred. `src/agents_md_drift.rs:40-46` names the guarded set as exactly three comparisons: the committed root `AGENTS.md` against a fresh render, the committed `.agents/AGENTS.reference.md` against a fresh render, and each rendered asset under `PROMPT_DEST_PREFIX` against its committed copy. `:57-61` states the complement as a rule: "Anything else the scaffold emits, or the repo commits, is unguarded by this module", naming the `docs/plans/TEMPLATE` family as an illustration. The single-source guards `src/isolation_policy.rs`, `src/findings_naming.rs`, `src/recommendation_rule.rs` and `src/workflow_spec.rs` each project a fragment into the generated scaffold files, so they reach the same set. NO DRIFT GUARD REACHES `docs/plans/agent-scaffold.plan.toml`, THE LEDGER, OR THE GENERATED PLAN VIEW.

NOT GATES, and excluded deliberately. `agent-scaffold status`, `agent-scaffold next` and `agent-scaffold audit` are advisory by their own `--help` text (`next`: "Advisory ... Read-only and stateless"; `audit`: "Advisory ... never deletes anything; a human decides each candidate"). `nix fmt` is excluded on the standing project instruction that this repo is not formatter-clean at HEAD. NO GIT HOOK IS INSTALLED: `ls $(git rev-parse --git-common-dir)/hooks/` contains only the shipped `.sample` files, so `tests/scaffold_precommit_hook.rs` pins a hook the scaffold can write and this repository does not run.

THE STRUCTURAL FACT THAT DECIDES MOST OF THE TABLE BELOW, measured rather than assumed: NO TEST READS THIS REPOSITORY'S OWN PLAN OR LEDGER. `grep -rn 'docs/plans' --include=*.rs src/ tests/ build.rs` returns only doc comments, `TEMPLATE` asset rows in `src/manifest.rs:591-602`, and per-test fixture paths (`docs/plans/p.plan.toml` and the like) built inside each test's own temporary directory. `agent-scaffold.plan.toml` and `agent-scaffold.ledger.md` appear nowhere in `src/` or `tests/`. G1 and G2 are therefore blind to every finding in this loop by construction, not by accident.

THE LEDGER IS READ BY NO GATE AT ALL. `grep -c orphan_tasks src/workflow.rs` returns 0 and `grep -rln orphan_tasks src/` returns only `src/plan/source.rs` and a testdata fixture, which is the same shape: the ledger has no parser (the `ledger-parse` keystone was skipped, `src/workflow.rs:9-10`), so a defect whose evidence is a ledger passage has no gate that could see it even in principle.

---

## 2. The catchability table, all twenty valid findings

Severities are the triagers' final ones, not the raisers'. Every row is NOT CATCHABLE; the demonstration for the whole set is the two positive controls in section 3, which run every gate over the exact trees that carried these findings. Where a row's ruling turned on something I measured separately, the measurement is named in the last column.

### Round 1, eleven valid findings (`docs/plans/agent-scaffold.reviews/q70-capture-triage.md`)

| id | sev | the defect, in one line | ruling | what would be needed | how I settled it |
| --- | --- | --- | --- | --- | --- |
| R1A-1 | medium | "the convention already exists at three sites" measured four | NOT CATCHABLE | a checker that compares a prose count against the population it names; the sites live in `[[step.waiver]].note`, which no gate joins to anything | ran: POS-A, all gates green |
| R1A-2 | medium | "Two loops hit this" measured six identities across three steps | NOT CATCHABLE | the population is derivable by one `jq` pipeline the item itself carries, but no gate runs it and none compares its result to prose | ran: POS-A green, and the item's own pipeline, which returns the six |
| R1A-4 | low | "THE DURABLE RECORD SAYS FOUR, NOT THREE" is not what the ledger says | NOT CATCHABLE | any gate that reads the ledger; there is none | ran: POS-A green; read: the no-ledger-parser fact above |
| R1A-5 | low | `src/plan/source.rs:791-843` under-cites a block that closes at `:856` | NOT CATCHABLE | a line-citation resolver that checks the cited range covers the construct named, which is a strictly stronger thing than mechanism (3) as the item describes it | ran: POS-A green |
| R1B-1 | medium | the opening `ask` frames the pass at half the mandate `Q-55-entryroute` decided | NOT CATCHABLE | semantic comparison of prose scope against a decision receipt's `chosen`; the receipt is machine-readable, the scope claim is not | ran: POS-A green |
| R1B-2 | high | "WHAT THE PASS OWES BACK" omits four duties the body makes mandatory | NOT CATCHABLE | duty-sentence extraction from prose and set comparison against a list, which is what the round 2 triager did by hand with a script | ran: POS-A green |
| R1C-1 | medium | the declared-increment namespace covers under half the identities, two are undeclarable | NOT CATCHABLE as raised | the underlying facts are measurable and `validate` even enforces the kebab-case rule, but the finding is an OMISSION from the item, and no gate reports what prose fails to say | ran: POS-A green |
| R1C-2 | high | W5 cannot do the declared lookup without widening a shared serialised type | NOT CATCHABLE | design-space pricing; there is no artifact to check it against | ran: POS-A green |
| R1C-3 | high | the item imposes a binary the code and the durable record both exceed | NOT CATCHABLE | as above, and the excluded third direction is named only in ledger prose | ran: POS-A green |
| R1C-4 | medium | a third live waiver-validation path exists, referred to in the past tense | NOT CATCHABLE | as above | ran: POS-A green |
| R1C-5 | medium | the fix's edit surface includes drift-guarded generated files the item names neither | NOT CATCHABLE as raised, BUT ITS CONSEQUENCE IS | G1's `agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render` catches the eventual mis-priced edit, and I demonstrated that (negative control N5, exit 101). The gate fires AFTER a step authored from a compliant proposal ships, which is the cost the finding names | ran: POS-A green, and N5 red |

### Round 2, nine valid findings (`docs/plans/agent-scaffold.reviews/q70-capture-r2-triage.md`)

| id | sev | the defect, in one line | ruling | what would be needed | how I settled it |
| --- | --- | --- | --- | --- | --- |
| R2A-1 | high | the lettered list's completeness guarantee is false and tells the reader not to check | NOT CATCHABLE | as `R1B-2` | ran: POS-B and POS-C, all gates green |
| R2A-2 | low | "It says THREE in two live passages" measured six | NOT CATCHABLE | a ledger reader | ran: POS-B/POS-C green |
| R2A-3 | medium | the "by-product of the membership rule" causal claim contradicts the live plan data | NOT CATCHABLE, AND THE CLOSEST OF THE TWENTY TO MECHANISABLE | the contradicting data is entirely inside the plan TOML the validator already parses (I recomputed it: 45 declared increment ids, 13 named by an increment-unit waiver, 32 by none), but a validator cannot know that prose asserts otherwise | ran: POS-B/POS-C green, and my own `tomllib` recount |
| R2A-4 | medium | "roughly eleven `src/checks.rs` citations" measured fifteen | NOT CATCHABLE | mechanism (3) in a line-citation form produces the true figure, so it would falsify the count indirectly | ran: POS-B/POS-C green, and `grep -oE 'src/checks[.]rs:[0-9]+(-[0-9]+)?' ... \| sort -u \| wc -l` returns 15 |
| R2B-1 | high | the item omits the `Q-55-mechanism` receipted decision that queues a third change to the same schema | NOT CATCHABLE | mechanism (2) lists `Q-55-mechanism` among today's forty dangling receipts, so its routine output would have put the receipt in front of the writer; it would not say the item must register it | ran: POS-B/POS-C green, and the dangling-receipt measurement in section 5 |
| R2B-2 | medium | the deferred-inputs list omits `Q-55-resumecost` and the unowned ledger half | NOT CATCHABLE | as `R2B-1`; `Q-55-resumecost` is also in the dangling forty | ran: POS-B/POS-C green, same measurement |
| R2B-4 | low | the list omits three of the five components the Design explorations rule requires | NOT CATCHABLE | prose-to-prose conformance between a plan item and a pack rule | ran: POS-B/POS-C green |
| R2B-5 | low | "the ledger's current next-action paragraph", twice, names the superseded one | NOT CATCHABLE, AND NOT BY MECHANISM (3) EITHER | both quotations are exact and both resolve today (one at 2 occurrences, one at 3, measured in section 5). What is wrong is the label, not the text, so a resolver would have to model supersession markers | ran: POS-B/POS-C green, and the resolver probe |
| R2B-6 | low | the entry-route ground is relayed without the three Project Principles the record attaches | NOT CATCHABLE | as `R2B-4` | ran: POS-B/POS-C green |

### The orchestrator defects recorded alongside them

| defect | the defect, in one line | ruling | how I settled it |
| --- | --- | --- | --- |
| (19) | the next-action paragraph said `validation-constraints` carries "FOUR things" and it carries at least six | NOT CATCHABLE. A ledger prose count, and no gate reads the ledger. The two missing bodies of work are `Q-55-mechanism` and `Q-55-resumecost`, which is the same pair mechanism (2)'s output would have listed | ran: POS-B/POS-C green; measured the dangling set |
| (20) | the anchor's positional pointer "the one directly below it" stopped resolving when two paragraphs were inserted | NOT CATCHABLE. There is no quoted text to resolve, which is exactly why the cure was to convert positional pointers into quoted-text handles. Mechanism (3) can check the cure; it cannot detect the disease | read: the ledger's own record at `:567` |
| (21) | an orchestrator edit to the ledger deleted a sentence `Q-70` quoted verbatim | NOT CATCHABLE. REPRODUCED IN FULL, section 3. All seven gates pass over the exact state | ran: POS-C, seven gates, all green |
| recurrence of (12) and (17) | the orchestrator relayed two recorded counts into a writer's brief without measuring either | NOT CATCHABLE, and not even in principle: a spawn brief is not a repository artifact and no gate can see it | read: the ledger's record at `:539` |

TOTAL: 0 of 20 valid findings, 0 of 4 orchestrator defects, 0 of 24.

THAT RESULT HAS A PRECEDENT IN THIS PROJECT'S OWN RECORD, which is corroboration rather than evidence: `f15246a`, "docs: record inc4 round 3 detectability findings, 0 of 20 catchable". Two independent loops, two independent lenses, the same figure. The population differs (inc4's twenty were findings against a source-and-docs increment; these twenty are findings against a plan item), so the agreement is not a shared cause, and it does not make either measurement stronger than its own evidence.

---

## 3. The positive controls

Three fixtures, each an exact tree from this branch's own history rather than a synthetic reconstruction. `src/` and `pack/` are byte-identical across all four branch commits (`git rev-parse <c>:src <c>:pack` returns `503d14f...` and `e65f92a...` for every one of `2c2be88`, `58b677f`, `198556e`, `61b1d35`), so no gate result below is confounded by a source difference.

- POS-A = `<S>/posa`, the tree at `58b677f`, THE STATE ROUND 1 REVIEWED. It carries all eleven round 1 valid findings at once.
- POS-B = `<S>/pos`, the tree at `198556e`, THE STATE ROUND 2 REVIEWED. It carries all nine round 2 valid findings plus the dangling quotation of orchestrator defect (21).
- POS-C = `<S>/posc`, the tree at `198556e` with the ledger replaced by `git show 00318ec:docs/plans/agent-scaffold.ledger.md`. THIS IS THE TRUE HISTORICAL MOMENT of defect (21), and building it separately is not pedantry: see the ruling at the end of this section.

### The dangling quotation, established before any gate was run

```
$ git archive 198556e | tar -x -C <S>/pos
$ grep -cF 'the three `agent-scaffold next` defects routed here by an earlier human decision' <S>/pos/docs/plans/agent-scaffold.plan.toml
1
$ grep -o '.\{80\}the ledger.s current next-action paragraph, item (4), "the three `agent-scaffold next` defects routed here by an earlier human decision".' <S>/pos/docs/plans/agent-scaffold.plan.toml
xt` defects are already-diagnosed point defects with NO OPEN DESIGN SPACE"; and the ledger's current next-action paragraph, item (4), "the three `agent-scaffold next` defects routed here by an earlier human decision".
$ grep -o '(d) The `agent-scaffold next` defects routed here by[^.]\{0,120\}' <S>/pos/docs/plans/agent-scaffold.ledger.md
(d) The `agent-scaffold next` defects routed here by the human decision of 2026-07-30
```

So the defect (19) rewrite turned numbered item "(4) the three `agent-scaffold next` defects routed here by an earlier human decision" into lettered item "(d) The `agent-scaffold next` defects routed here by the human decision of 2026-07-30", and `Q-70` still quoted the old wording verbatim and still attributed it to "the ledger's current next-action paragraph, item (4)". The quotation dangles and the item number is wrong with it.

```
$ git archive 198556e | tar -x -C <S>/posc
$ git show 00318ec:docs/plans/agent-scaffold.ledger.md > <S>/posc/docs/plans/agent-scaffold.ledger.md
$ grep -cF 'the three `agent-scaffold next` defects routed here by an earlier human decision' <S>/posc/docs/plans/agent-scaffold.plan.toml
1
$ grep -cF 'the three `agent-scaffold next` defects routed here by an earlier human decision' <S>/posc/docs/plans/agent-scaffold.ledger.md
0
```

`grep -cF` returns 0, exactly as the ledger records at `:569`.

### Every gate, over POS-C

```
$ agent-scaffold render <S>/posc/docs/plans/agent-scaffold.plan.toml --check --strict
<PLAN>: up to date
EXIT=0

$ agent-scaffold validate --source <S>/posc/docs/plans/agent-scaffold.plan.toml --metrics <S>/posc/docs/metrics/workflow.jsonl
<LOG>: 310 records, valid
<PLAN>: 95 steps, 70 questions, valid
EXIT=0

$ agent-scaffold validate --source <S>/posc/... --metrics <S>/posc/... --workflow
<PLAN> vs <LOG>: workflow invariants hold
EXIT=0

$ LC_ALL=C grep -cP '[^\t\x20-\x7e]' on plan.toml, the generated view, Q-70.md and the ledger
0, 0, 0, 0

$ cd <S>/posc && git init -q && git add -A && git commit -q -m ...
$ agent-scaffold checks
        pass  render-check (lint)
checks: 1 passed, 0 failed, 0 skipped
EXIT=0
```

`cargo test` and `cargo clippy --all-targets -- -D warnings` were run over the POS-B tree, which shares POS-C's `src/` and `pack/` byte for byte:

```
$ CARGO_TARGET_DIR=<S>/target-pos cargo test          # in <S>/pos
CARGO_TEST_EXIT=0
test result: ok. 378 passed; 0 failed; ...   (plus 5, 1, 1, 9, 3, 20, 1, 4 across the integration binaries)
422 tests, 0 failures

$ CARGO_TARGET_DIR=<S>/target-pos cargo clippy --all-targets -- -D warnings
CLIPPY_EXIT=0
```

### Every gate, over POS-A and POS-B

Identical results, run in the same shapes: `render --check --strict` exit 0 "up to date"; `validate` plain exit 0 at 310 records and 95 steps, 70 questions; `validate --workflow` exit 0 "workflow invariants hold"; the ASCII sweep 0 on all four files; `agent-scaffold checks` exit 0, 1 passed.

### The ruling, and one thing worth more than the ruling

THE RECORDED OUTCOME IS CONFIRMED. Every gate this repository owns passes over a plan item carrying a dangling verbatim quotation, and over the two trees that carried twenty valid findings between them. No gate catches it, and the reason is structural rather than incidental: nothing in `src/` or `tests/` reads `docs/plans/agent-scaffold.plan.toml` or the ledger, so there is no gate whose input contains the defect.

AND ONE THING I DID NOT EXPECT, WHICH IS A DESIGN INPUT FOR THE PASS RATHER THAN A GATE RESULT. Run the same substring resolution over POS-B, the tree as it exists on the branch today:

```
$ grep -cF 'the three `agent-scaffold next` defects routed here by an earlier human decision' <S>/pos/docs/plans/agent-scaffold.ledger.md
1
$ grep -nF ... | cut -c1-70
569:ORCHESTRATOR DEFECT (21), CAUSED BY THE FIX FOR DEFECT (19) AND FOUND BY
```

The only occurrence is inside the ledger's own post-mortem OF the deletion, added by `4351e6c`, which quotes the deleted sentence in order to record that it was deleted. A resolver that asks "does this quoted string occur in the cited file" returns TRUE on POS-B and reports green on the very defect it exists to catch. That is not hypothetical: POS-B is the tree on this branch right now, and my resolver probe (section 5) goes green on that quotation there and red on it in POS-C. This is the single most consequential measurement in this review and it drives `R3C-1`.

---

## 4. The negative controls

Seven, one per gate, so that no green above can be a rig failure. Each is a defect the named gate SHOULD catch. Every one fires.

| id | injected defect | gate | result |
| --- | --- | --- | --- |
| N1 | one word changed in the GENERATED `docs/plans/agent-scaffold.md` ("6 open questions" to "7 open questions") | G5 | `error: ... differs from a fresh render (a hand-edit, or a stale render after a source edit) (first difference at line 5 ...)`, EXIT=1 |
| N1b | the same tree, `--check` without `--strict` | G5 soft form | `warning: ...`, EXIT=0 |
| N2 | `Q-70`'s `status` set to `explorng` | G3 | `unknown variant \`explorng\`, expected one of \`open\`, \`exploring\`, \`decided\`, \`superseded\``, EXIT=1 |
| N3 | the owed `workflow-enforcement-tier-w5` waiver injected under its step | G4 | two problems on one waiver, EXIT=1 |
| N4 | a U+2014 em-dash inserted into the `Q-70` ask | G7 | `1895:THE COUPLING HYPOTHESIS <em-dash> THE PASS MUST SETTLE ...`, grep EXIT=0 (match found) |
| N5 | the W5 clause in `pack/instrument.md` changed alone | G1 drift guard | `test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... FAILED`, `4 passed; 1 failed`, EXIT=101 |
| N6 | `pub(crate) fn negative_control_probe(values: &Vec<String>)` appended to `src/workflow.rs` | G2 | `error: writing \`&Vec\` instead of \`&[_]\` involves a new object where a slice will do`, EXIT=101 |
| N7 | N1's tree, committed, run through the checks runner | G6 | `fail  render-check (lint)`, `checks: 0 passed, 1 failed`, EXIT=1 |

N3's output reproduces `Q-70`'s own recorded fixture byte for byte, which is a second, independent confirmation that the item's two quoted problem strings are accurate:

```
$ agent-scaffold validate --source <S>/neg3/docs/plans/agent-scaffold.plan.toml --metrics <S>/neg3/docs/metrics/workflow.jsonl --workflow
<PLAN>: waiver `workflow-enforcement-tier-w5` on step `workflow-enforcement-tier` names increment `workflow-enforcement-tier-fold`, which is not one of the step's increments
<PLAN> vs <LOG>: TOML waiver `workflow-enforcement-tier-w5`: increment waiver names step `workflow-enforcement-tier` but increment `workflow-enforcement-tier-fold` belongs to step `workflow-enforcement-tier-fold`
EXIT=1
```

N4 CARRIES A SECOND HALF THAT MATTERS FOR THE MEASUREMENT. The same em-dashed file passes `validate --workflow` at exit 0:

```
$ agent-scaffold validate --source <S>/neg4/... --metrics <S>/neg4/... --workflow  >/dev/null 2>&1
N4b_VALIDATE_EXIT=0
```

So the ONE mechanical property of the `Q-70` ask that any gate can see is its byte set, and even that is seen only by a hand-run grep with no runner behind it. The plan TOML passes through G3, G4, G5 and G6 as a string the tools carry and do not read.

ONE GATE PROPERTY WORTH RECORDING, found by N1b and not previously stated in this loop's record: `render --check` ALONE IS NOT A HARD GATE. It warns and exits 0. Only `--strict` fails, and the project gets the hard form because `.agents/checks.toml`'s single row spells it `agent-scaffold render --check docs/plans/agent-scaffold.plan.toml --strict`. Anyone running `render --check` by hand, as several records in this loop describe doing, is running the soft form. This is not raised as a finding against `Q-70`, which never claims otherwise.

---

## 5. Evidence on the three mechanisms

`Q-70` at `:1893` scopes three detection mechanisms, "in the buildability order round 3 of inc4 recorded". Each was measured against this repository's live records.

### Mechanism (1), the W6 waiver-note breakdown join

Implemented as the item describes it, checking each `[[step.waiver]].note` breakdown against the `valid_findings` of the round records the same command already reads:

```
workflow-enforcement-tier-w1  increment=workflow-enforcement-tier-inc1
   stated total 13  note breakdown [3, 4, 6] (sum 13)
   round-record valid_findings [3, 4, 6]  -> MATCH
workflow-enforcement-tier-w2  ...  [9, 5, 6, 4] vs [9, 5, 6, 4]        -> MATCH
workflow-enforcement-tier-w4  ...  [11, 9, 6, 4, 5] vs [11, 9, 6, 4, 5] -> MATCH
workflow-enforcement-tier-w3  ...  [6, 4, 2, 0, 2] vs [6, 4, 2, 0, 2]   -> MATCH
```

Four sites, four agreements, zero red. Built today it catches NOTHING, and it caught none of the twenty-four. It is a regression guard on a convention currently held correctly by hand.

### Mechanism (2), dangling decision-receipt detection

```
distinct decision q_ids: 62   registered [[question]] ids: 70   DANGLING: 40
Q-55-mechanism dangling?  True
Q-55-resumecost dangling? True
Q-55-entryroute dangling? True
non-Q-55- dangling ids: []
```

Forty red today, and the item's "dominated by `Q-55-<suffix>`" is if anything conservative: all forty are. THE PART THAT BEARS ON THIS LOOP: `Q-55-mechanism` and `Q-55-resumecost` are the two receipts whose omission produced `R2B-1` (`high`) and `R2B-2` (`medium`), and the same pair is what orchestrator defect (19) undercounted. Mechanism (2) would not have raised those findings, since a list of unregistered `q_id`s says nothing about what a plan item owes; but its standing output is a list on which both receipts appear by name, which is one query away from the finding.

### Mechanism (3), the quotation resolver

A naive substring resolver was built and run over `Q-70`'s own quotations in each state. Extraction is a parity split of the `ask` on the quote character (118 quote characters at HEAD, even, so they balance), keeping spans of five or more words, then counting occurrences outside the `Q-70` entry across the ledger, the plan, the log, the cited `src/` files, `pack/AGENTS.md`, `AGENTS.md` and the cited step sidecar.

| tree | quotations | resolve | red |
| --- | --- | --- | --- |
| POS-A (`58b677f`) | 8 | 6 | 2 |
| POS-C (true defect (21) state) | 13 | 10 | 3 |
| POS-B (`198556e` as it sits on this branch) | 13 | 11 | 2 |
| HEAD (`61b1d35`) | 20 | 18 | 2 |

THREE RESULTS COME OUT OF THAT TABLE.

FIRST, THE RESOLVER CATCHES DEFECT (21) AND NOTHING ELSE DOES. POS-C's third red is exactly `the three \`agent-scaffold next\` defects routed here by an earlier human decision`, and POS-C is the state at which all seven gates pass.

SECOND, THE RESOLVER FALSE-GREENS ON DEFECT (21) THE MOMENT THE LEDGER RECORDS IT. POS-B differs from POS-C only in the ledger, and the difference is the post-mortem paragraph that quotes the deleted sentence. Red count drops from 3 to 2 and the dangling quotation resolves. A resolver that does not scope to LIVE passages, excluding a record's own post-mortems and round records, reports green on the defect class it was built for. This repository has now measured that decay twice independently: round 2's triage recorded that "teaching W5 the structured step association W3 already uses" resolves to 2 because the ledger's own round 1 record quotes it, and my probe reproduces that count at HEAD.

THIRD, THE RESOLVER'S OPENING RED LIST IS NOT ONLY THE STALE `src/checks.rs` CITATIONS. In every state, two of the reds are the validator problem strings `Q-70` quotes from its own fixture: `waiver \`workflow-enforcement-tier-w5\` on step ... which is not one of the step's increments` and `TOML waiver \`workflow-enforcement-tier-w5\`: increment waiver names step ...`. Neither is a document quotation. Both are EXPECTED TOOL OUTPUT and both are correct: negative control N3 reproduces them byte for byte at exit 1. A resolver with no way to mark a quotation as expected output goes red on the item's most valuable evidence.

FOURTH, THE RESOLVER DOES NOT CATCH `R2B-5`. Both of that finding's quotations resolve at HEAD, one at 2 occurrences and one at 3. The text is exact; the label "current" on a paragraph published under a supersession notice is what is wrong. Detecting it needs supersession modelling, which is a different mechanism.

### What this does to the recorded order

THE RECORDED ORDER IS NOT CONTRADICTED, AND I SAY SO PLAINLY. The item labels it a BUILDABILITY order, and on buildability the evidence supports it. Mechanism (1) reads the waiver notes and round records `check_workflow_toml` already holds. Mechanism (2) reads the `type:"decision"` records and the registered `[[question]]` ids, both already parsed by W4. Mechanism (3) needs quotation extraction from arbitrary prose, cross-file resolution, live-passage scoping and an expected-output escape, none of which any existing code does. Cheapest to dearest, in that order.

THE YIELD ORDER IS ITS EXACT INVERSE, and it is measured rather than argued: mechanism (3) catches the one defect nothing else caught and would have falsified `R2A-4`'s figure; mechanism (2) surfaces the two receipts behind `R2B-1`, `R2B-2` and orchestrator defect (19); mechanism (1) catches nothing, today or in this loop. The item carries the buildability axis and no yield axis at all, and it is the sole input to the pass that will rule, under its own letter (f), whether mechanisms 2 and 3 are designed or only bounded. That is `R3C-2`.

---

## 6. Findings

### R3C-1. The item records one design constraint on the quotation resolver, and this loop measured two more, one of which is a false green on the exact defect the resolver exists for

SEVERITY: `medium`. Absolute impact if left unfixed: a proposal scopes mechanism (3) as substring resolution, the pass and the human accept it, and the built check reports green on the next instance of orchestrator defect (21) as soon as the ledger records the previous one, which is this project's invariable practice.

CLAIM. `Q-70` at `:1893` describes mechanism (3) as "A QUOTATION RESOLVER, automating what acceptance check 21 already instructs", and records exactly one caveat about it: that it "would immediately go red on the `src/checks.rs` citations `Q-55-check21b` deliberately left stale", with the opening red list being all of them rather than a subset. That is accurate and I re-measured it (15 distinct citations, and the item's own reproduction command returns 15). Two further constraints on the same mechanism were produced by this loop's own record and are not in the item.

EVIDENCE, constraint one, THE SELF-QUOTING RECORD. Same fixture, one file different:

```
$ grep -cF 'the three `agent-scaffold next` defects routed here by an earlier human decision' <S>/posc/docs/plans/agent-scaffold.ledger.md
0                       # ledger at 00318ec: the defect is visible
$ grep -cF 'the three `agent-scaffold next` defects routed here by an earlier human decision' <S>/pos/docs/plans/agent-scaffold.ledger.md
1                       # ledger at 4351e6c: the only hit is the post-mortem OF the deletion
$ grep -nF ... <S>/pos/docs/plans/agent-scaffold.ledger.md | cut -c1-70
569:ORCHESTRATOR DEFECT (21), CAUSED BY THE FIX FOR DEFECT (19) AND FOUND BY
```

The resolver probe reflects it: red on that quotation in POS-C, green in POS-B. The ledger recording a defect using the defective words is what retires the detection. This is not a one-off: round 2's triage independently measured `teaching W5 the structured step association W3 already uses` at 2 occurrences for the same reason, and recorded that "UNIQUENESS DECAYS WHILE RESOLVABILITY DOES NOT", but it recorded that against the find-by-quoted-text CONVENTION, not against the MECHANISM that has to implement it. `Q-70` is where the mechanism is scoped and it carries neither measurement.

EVIDENCE, constraint two, EXPECTED TOOL OUTPUT IS NOT A DOCUMENT QUOTATION. Of the twenty quotations of five or more words in the `Q-70` ask at HEAD, eighteen resolve and two do not, and the two are validator problem strings the item quotes from its own scratch fixture. Both are correct: negative control N3 reproduces them byte for byte at exit 1. A resolver with no expected-output escape reports the item's own measured evidence as dangling. The item's recorded caveat covers the `src/checks.rs` citations and does not reach this class.

WHY IT IS IN SCOPE AND NOT A RE-RAISE. Mechanism (3) is inside the pass's scope by the item's own heading at `:1893`, and letter (f) at `:1901` makes the DESIGNED-versus-BOUNDED ruling the pass's. Either way a measured failure mode of the mechanism is a pass input. No round 1 or round 2 finding touches mechanism (3)'s design; `R2A-4` is about the count in the same sentence and its remedy (round 2 remedy I site 3) is a count deletion, which landed and which I verified.

WHY `medium` AND NOT `high`. The item does not assert that substring resolution is sufficient; it names no implementation at all. The gap is an omission of measured evidence, and an explorer who builds the mechanism against this repository's records meets the self-quoting case on their first run, because forty ledger paragraphs quote other ledger paragraphs. It is not `low` because the false green is silent, it lands on the highest-consequence defect class in the set, and the item is the sole input to the pass.

### R3C-2. The three mechanisms are ordered on buildability alone, with no evidence about which catches anything

SEVERITY: `low`. Absolute impact if left unfixed: the pass rules mechanism (1) in and mechanisms (2) and (3) merely bounded, on an order that was never about yield, and the project builds the only one of the three that has nothing to catch.

CLAIM. `Q-70` at `:1893` presents the three mechanisms "in the buildability order round 3 of inc4 recorded" and carries no statement of what any of them would detect. The label is honest and I do not fault it: measured against the source, the buildability order holds. What the item lacks is the other axis, in the one paragraph the pass will read to decide priority under letter (f).

EVIDENCE. Measured against the live records, in section 5:

- Mechanism (1): four `[[step.waiver]].note` breakdowns, all four already agreeing with their round records. Zero red today. Zero of this loop's twenty-four.
- Mechanism (2): forty dangling receipts, two of them (`Q-55-mechanism`, `Q-55-resumecost`) the exact receipts behind `R2B-1` (`high`), `R2B-2` (`medium`) and orchestrator defect (19).
- Mechanism (3): catches orchestrator defect (21), which all seven gates miss (section 3), and in a line-citation form falsifies `R2A-4`'s figure.

The order on yield is (3), (2), (1). The order in the item is (1), (2), (3).

WHY IT IS A FINDING AT ALL. The item requires every proposal to price what it does not build ("must say what the other mechanism costs under that choice", `:1895`) and to rule on the scope of mechanisms 2 and 3 (letter (f)). Pricing a mechanism needs what it catches, and the item supplies a buildability ranking that reads as a priority ranking to any reader who does not notice the qualifier. Recording the yield evidence costs three sentences and closes it.

WHY `low`. The qualifier "buildability order" is present and accurate, the evidence above is now in the durable record whether or not the item carries it, and an explorer told to price mechanisms will measure them. Nothing is asserted falsely.

---

## 7. Defects in `src/`

NONE FOUND, and none looked for beyond what the gates exercised. The seven negative controls all produced the behaviour their gates document. One gate PROPERTY is recorded in section 4 for the orchestrator to note rather than route: `render --check` without `--strict` warns at exit 0, so the hard form of the render gate is reached only through `.agents/checks.toml` or by spelling `--strict` by hand. That is documented behaviour (`agent-scaffold render --help`), not a defect.

---

## 8. What I settled by running and what by reading

RAN: the gate inventory's baseline over the branch (all seven, green); POS-A, POS-B and POS-C over all seven gates; the seven negative controls; `cargo test` and `cargo clippy --all-targets -- -D warnings` on both a positive and a negative fixture; the quotation-resolver probe over four trees; the mechanism (1) note-to-round-record join over the live plan and log; the mechanism (2) dangling-receipt set difference; the declared-increment recount behind `R2A-3`'s row; every one of `Q-70`'s own reproduction commands (the blocker pipeline returns the six identities, the breakdown grep returns the four sites, the `src/checks.rs` citation census returns 15, the three-next-defects grep returns six lines, all as the item describes); the `docs/plans` reachability grep over `src/`, `tests/` and `build.rs`; and the installed-hook listing.

READ: the two triage files in full, both round 2 reviewer files' verdict sections, `AGENTS.md`, `.agents/prompts/reviewer.md`, the `Q-70` entry at `docs/plans/agent-scaffold.plan.toml:1880-1903`, `src/agents_md_drift.rs:40-100` for the guarded set, `src/workflow.rs:1-32` for the check enumeration, and the ledger paragraphs recording orchestrator defects (19), (20) and (21).

Every ruling in section 2 is settled by running the gates over a tree that actually carried the finding. The two indirect attributions to mechanisms (2) and (3) are reasoned from measurements, and are labelled as indirect where they appear.

Nothing above is presented as measured that was not run.

FIXTURE HYGIENE. Everything under `<S>/gates-r3/` and nothing else. No file was created at mode 000 or 600, so nothing needed chmod back. Nothing outside that directory was written or deleted.
