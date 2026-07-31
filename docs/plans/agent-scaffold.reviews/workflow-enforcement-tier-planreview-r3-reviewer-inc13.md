# `workflow-enforcement-tier` plan review, round 3: REVIEWER, the INC1 / INC3 / risk / ordering lens

Reviewer model: Claude Opus 5, 1M-context variant. Exact model id `claude-opus-5[1m]`.
Harness: `claude-code`.
Date: 2026-08-01.
Worktree: `.claude/worktrees/review-q55-r3b`, branch `review/q55-r3b` at commit `5169ea0` ("docs: apply the round 2 plan review fixes").
Artifact reviewed: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (primary), `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`, `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, and the `[[step]]` / `[[question]]` entries this fold adds or changes in `docs/plans/agent-scaffold.plan.toml`. `src/`, `pack/`, `tests/`, `README.md`, `CHANGELOG.md` and `justfile` are evidence, not artifact.
Lens: a fresh deep read of INC1 and INC3, the risk-classification arguments, and the increment ordering. INC2's content (sidecar lines 168 to 238) was read for context only and is NOT reviewed here, per the brief.
`TMPDIR` was `/tmp/r3b-scratch`, outside any git repository.

## Verdict

SEVEN FINDINGS, ALL `low`. NONE BLOCKS INC1 OR INC3 FROM BEING BUILT FROM THIS TEXT.

The substance of both increments holds up. I re-derived the lexical derivation rule against every layout the file names and it produces the answers the file claims, including the byte-identical no-regression case at check 9 and both accepted costs. The `_` catch-all really does cover exactly the two "metrics missing" variants and nothing else, so inc3's stated three-line code half is accurate. Every `src/`, `pack/`, `tests/`, `README.md` and `justfile` citation in the sections I read reproduces (full list in the enumeration). No existing test flips from pass to fail under inc3. The three increment ids and `risk_class` values in the plan TOML match the sidecar exactly.

What I found is a band of small currency and enumeration defects, four of which are direct continuations of defects earlier rounds already established in this fold and whose fixes did not reach a twin site:

- `R3B-1` is round 1's `EX-5` surviving at a second site the `EX-5` fix's grep could not see. It is the only finding that could mislead a reviewer about what a correct INC1 looks like, and it is the one I would fix first.
- `R3B-2` is the `EX-6` shape again: the INC3 documentation-impact list omits an item the increment's own description requires.
- `R3B-3` and `R3B-6` are the `EX-8` shape: counts stated as current that the work itself, or the fold's own review loop, has already moved.
- `R3B-5` is the `EX-8` vacuity shape on inc3's own acceptance check.
- `R3B-4` and `R3B-7` are the `F-1` / `INC2-6` shape: a stated enumeration that is short by one when re-measured.

I applied the tense rule throughout and say per finding which tense decided it. `R3B-1`, `R3B-3` and `R3B-5` exist ONLY under the forward tense: all three are true, or harmless, as statements about the tree at `5169ea0` and become defects against the tree the increments produce. Two of them (`R3B-3`'s `386` and `R3B-5`'s check 15) were verified as current facts by both previous rounds' triage, which is exactly the miss the brief describes.

I did not raise `INC2-7` or `F-5`; both are confirmed still present and are accepted residuals.

## Findings

| id | severity | one-line summary |
| --- | --- | --- |
| `R3B-1` | low | The ordering argument's premise, that inc3 is the only increment making a previously-green run fail, is falsified by the same file's inc1 and inc2 text; it is round 1's `EX-5` at an unfixed twin site. |
| `R3B-2` | low | The INC3 documentation-impact list omits the `CHANGELOG.md` entry that inc3's own increment description requires, on the increment that changes a CLI exit code. |
| `R3B-3` | low | Check 1 pins `cargo test` at "386 expected", the PRE-change count, so a correct implementation fails the number and one that adds no test passes it. |
| `R3B-4` | low | "The only two places the pack mentions `docs/metrics/workflow.jsonl` outside the instrumentation section" is five places in three files when re-measured. |
| `R3B-5` | low | Check 15's "reports the missing log BY PATH" clause is already true of the pre-fix build, so nothing pins inc3's new problem message, and the literal minimal implementation ships "skipping the workflow check" on a run that exits 1. |
| `R3B-6` | low | Inc3's risk argument says "seventy-seven artifacts ever reviewed"; it is eighty at the commit under review, and one of the three additions is this fold's own review loop. |
| `R3B-7` | low | Inc1's risk paragraph attributes `+163/-18` to "the whole thing including siblings and ledger", but that measurement includes candidate (b), which is inc2's work and which the same file separately sizes at "roughly 80 lines". |

---

## `R3B-1`. Severity `low`. The ordering argument's load-bearing premise is falsified twice by the same file, and it is round 1's `EX-5` standing at a site the `EX-5` fix could not see

TENSE APPLIED: FORWARD. The sentence is a claim about what the three increments will do, not about the tree. Against the tree it is not even evaluable, since no increment has landed.

THE TEXT. `workflow-enforcement-tier.md:290`, in "The three increments, and why in this order", which the brief names as a paragraph to read closely:

> THE ORDER IS inc1 -> inc2 -> inc3, AND EVERY EDGE IS LOAD-BEARING. The first planner pass argued that the path fix must precede the tier policy [...] THAT ARGUMENT STILL BINDS, and the design pass GENERALISES it: the tier policy goes LAST because it is **the only increment that makes a previously-green run fail**, and EVERY escape hatch a user reaches for when it does is closed by an earlier increment.

THE FIRST FALSIFIER IS EIGHT LINES AWAY, IN THE SAME FILE. `:298`, inc2's risk classification, opens:

> `workflow-enforcement-tier-inc2` is RISKY (two consecutive clean rounds), and for reasons that do not overlap inc1's. It INTRODUCES a non-zero exit on validator invocations that succeed today AND withholds output from projection invocations that answer today [...]

An invocation that succeeds today and exits non-zero after inc2 is, by definition, a previously-green run that inc2 makes fail. The acceptance list pins three of them: check 11 (`:318`, "Before inc2 this prints `workflow invariants hold` at exit 0"), check 12 (`:319`) and check 13 (`:320`).

THE SECOND FALSIFIER IS INC1'S OWN BULLET, AS CORRECTED BY ROUND 1. `:274` now reads:

> NO new REFUSAL mechanism: any new non-zero exit comes from the pre-existing W3 check finally running against the right project, which is check 4's whole point.

That clause exists because round 1's `EX-5` measured the flip. Check 4 (`:311`) requires it explicitly: "Give the fixture a log of its OWN with no evidence for that slug and expect the correct RED instead of the absence of a green." Round 1's triage recorded the measurement (`workflow-enforcement-tier-planreview-r1-triage.md:126-136`): the same command line, same files, exits 0 today and exits 1 after inc1. So inc1 also makes a previously-green run fail.

WHY THE `EX-5` FIX DID NOT REACH THIS SITE, WHICH IS THE FINDING'S REAL VALUE AND IS THE `RES-1` SHAPE. Round 1's triage scoped the `EX-5` fix as "Single-site: line 272. (`grep -c "still exits 0"` returns 2 ...)". Line 290 makes the identical claim in different words ("the only increment that makes a previously-green run fail"), which that grep cannot match, so the fix pass applied its stated scope faithfully and inherited its blind spot. Before the fix both statements agreed and were both false; after it they disagree, so the fold now contradicts itself about whether inc1 can produce a new red.

WHAT DOES AND DOES NOT SURVIVE. The ORDERING CONCLUSION IS UNAFFECTED and I attacked it deliberately: inc3 last is independently carried by the rest of the same sentence (every escape hatch is closed by an earlier increment) and by the first pass's workaround argument, both of which hold. What fails is the generalisation the paragraph credits to the design pass. The residual harm is the one round 1 named for `EX-5`: a reviewer of inc1, a `risky` increment needing two clean rounds, who holds "only inc3 makes a green run fail" can read check 4's correct new red as a regression and press for a derivation that suppresses it, which is the self-concealing failure mode the step exists to remove.

WHY `low` AND NOT `medium`, AND IT IS A BORDERLINE CALL. Round 1 rated `EX-5` `medium` because the false claim WAS inc1's stated safety property, so an implementer reading inc1's own bullet was misled. That is closed: `:274` now states the true property unambiguously, and check 4 demands the counterexample in the same list. What is left is a false premise inside a paragraph about ordering whose conclusion stands without it, contradicted on the facing page by `:298`. That is the same band round 2 used when it downgraded `INC2-1` on the reasoning that the operative instruction is now correct and a reviewer of a `risky` increment is reading the file to catch the discrepancy. A triager who weighs the `EX-5` lineage more heavily than the containment could defensibly hold `medium`.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. Single-site, and a DELETION-CLASS form exists. `grep -rn "only increment that makes a previously-green"` over the fold returns exactly one hit, `workflow-enforcement-tier.md:290`. Strike "because it is the only increment that makes a previously-green run fail, and", leaving "the tier policy goes LAST because EVERY escape hatch a user reaches for when it does is closed by an earlier increment", which is the true half and the half the argument actually runs on. If the fix pass prefers to keep a positive claim, the narrow true one is that inc3 is the only increment that makes a run fail for a reason OTHER than a pre-existing check finally seeing the right data, but that authors prose on a sentence that has now produced a finding twice, and this project's calibration data argues against it.

---

## `R3B-2`. Severity `low`. The INC3 documentation-impact list omits the `CHANGELOG.md` entry that inc3's own increment description requires

TENSE APPLIED: FORWARD, and the defect is visible only under it. Nothing about `CHANGELOG.md` is wrong today; the defect is a missing instruction for work inc3 must do.

THE INCREMENT DESCRIPTION REQUIRES IT. `workflow-enforcement-tier.md:276`, verbatim:

> - `workflow-enforcement-tier-inc3`, THE TIER POLICY AND THE `SE-3` DOCUMENTATION HALF. The `_` catch-all at `src/main.rs:999-1003` becomes a reported problem so `--workflow` exits non-zero, plain `validate` untouched; the `run_validate` doc comment corrected; `pack/AGENTS.md:93` qualified and the two deployed copies regenerated; **the README `validate` paragraph and the CHANGELOG entry**. Plus its own red-then-green test.

THE DOCUMENTATION-IMPACT LIST DOES NOT. I read `:358` to `:365` in full. The INC3 list has exactly five bullets: `src/main.rs:791-816` (`:360`), `tests/validate_workflow_toml_source_needs_no_plan.rs:1-13` and `:96-98` (`:361`), `README.md:210` (`:362`), `pack/AGENTS.md:93` (`:363`), and the two deployed copies (`:364`), plus a closing NOT bullet (`:365`). There is no `CHANGELOG.md` bullet.

THE ASYMMETRY IS THE PROOF THIS IS AN OMISSION RATHER THAN A DELIBERATE SILENCE. The INC1 list ends with one (`:346`, "`CHANGELOG.md`, the `## [Unreleased]` section. The section today has `Added` and `Changed` and no `Fixed`; check what a comparable fix did before introducing a new subsection.") and the INC2 list ends with one (`:356`). Only INC3 lacks one, and INC3 is the increment the same file calls out at `:300` as changing "a CLI EXIT CODE, the most externally depended-on contract the tool has". I also checked whether `:356` could be read as covering inc3: it cannot. Its scope is stated in its own words as the new JSON fields and the withheld-output behaviour, and its closing clause ("withheld output at exit 0 versus a non-zero exit") is contrasting inc2's projections against inc2's own containment refusal, not against inc3's tier policy.

I VERIFIED THE FACTUAL PREMISES THE MISSING BULLET WOULD REST ON. `CHANGELOG.md:7` is `## [Unreleased]`, `:9` is `### Added`, `:20` is `### Changed`, and `grep -n '^### Fixed' CHANGELOG.md` returns nothing, so the INC1 bullet's description of the section is accurate and would apply unchanged.

WHY IT MATTERS RATHER THAN BEING A CROSS-REFERENCE NICETY. `:338`, the section's own preamble, is "All in-repo, and each item travels with the increment that makes it stale rather than being left as a documentation step owed", and `:288` states there is "deliberately NO separate documentation increment". The Documentation impact section is therefore the enumerated contract for what ships with each increment, and an implementer working it literally ships the tool's most externally visible break of the release with no changelog line.

WHY `low`. This is `EX-6`'s band exactly, and for `EX-6`'s reason: it is one missing item in an enumerated list, the requirement is stated correctly elsewhere in the same file (`:276`), and a careful implementer reading their own increment bullet will find it. It does not mislead about behaviour.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND THE PLAN TOML. Single-site: add one bullet to the INC3 list after `:365`, or, cheaper and authoring less, one clause. `grep -rn CHANGELOG` over the fold returns `workflow-enforcement-tier.md:260`, `:275`, `:276`, `:346`, `:351`, `:356`, plus `status-resume-ignores-json.md:111` and `plan.toml:1704`, none of which is an INC3 documentation-impact item. Note `:260` already supplies the content for inc2's changelog break and has no inc3 counterpart, so the new bullet should name the tier policy's exit-code flip in its own terms.

---

## `R3B-3`. Severity `low`. Check 1 pins the suite at the pre-change test count, so a correct implementation fails it and one that adds no test passes it

TENSE APPLIED: BOTH, AND THEY DISAGREE, WHICH IS THE FINDING. Against the tree the number is exactly right; against the tree the increments produce it is wrong by construction.

I RE-MEASURED IT RATHER THAN TRUSTING THE FILE, as the brief instructs. `cargo test` at `5169ea0` with `TMPDIR=/tmp/r3b-scratch`:

```
test result: ok. 373 passed; 0 failed  (lib)
test result: ok. 5 passed; 0 failed
test result: ok. 1 passed; 0 failed
test result: ok. 1 passed; 0 failed
test result: ok. 3 passed; 0 failed
test result: ok. 1 passed; 0 failed
test result: ok. 2 passed; 0 failed
```

373 + 5 + 1 + 1 + 3 + 1 + 2 = 386. So `:308`'s "(386 expected)" is CURRENT, and both previous rounds' triage were right to confirm it. That is the whole problem: they confirmed it in the present tense.

THE SAME DOCUMENT REQUIRES THE NUMBER TO CHANGE. `:304`, the acceptance section's own preamble: "all four defects are behavioural, so EACH INCREMENT owes at least one test that is RED against the pre-fix build and green after". `:274` requires "the red-then-green tests" for inc1 (plural), `:275` "its own red-then-green tests" for inc2 (plural), `:276` "its own red-then-green test" for inc3. `:332` (check 18) adds "A test pinning this belongs in the suite so a later 'improvement' that turns the default canonical fails loudly", and `:333` (check 19) requires "the tests assert both". So the suite must grow by at least six tests.

THE CHECK IS THEREFORE INVERTED. Read as written, and `:304` says a round is settled by running the checks rather than by reading the diff, check 1 PASSES for an implementation that adds no test at all and FAILS for a correct one. It is check 1's "both clean" clause, not the number, that does the work; the number is the part that cannot survive the change it is checking.

THE SAME NUMBER HAS A TWIN INSIDE THIS FOLD AND THE TWIN IS ALSO AN ACCEPTANCE EXPECTATION. `test-tmpdir-repo-assumption.md:66` is "1. `cargo test` passes with `TMPDIR` set INSIDE a git repository (a worktree-local scratch directory), which is the case that fails today. 386 expected, 0 failed." That step is `order = 95` (`plan.toml:1324`) and `workflow-enforcement-tier` is `order = 94` (`plan.toml:1300`), so by the time step 95 runs, this step's own increments have already moved the count. The two remaining sites, `workflow-enforcement-tier.md:306` ("Three of the suite's 386 tests") and `test-tmpdir-repo-assumption.md:3` ("Three of the 386 tests"), are narrative rather than expectations and drift more gently: the three named tests stay true, only the denominator moves. `plan.toml:1322` carries "3 of 386" in the step title.

WHY `low`. This is `EX-8`'s band. The impact is a number an implementer will obviously adjust and a check whose remaining clause still discriminates, and the failure mode is a puzzled implementer rather than a shipped defect. It is a real check-currency defect and it is in the increments' own evidence list, which is why it is not zero.

MINIMAL FIX AND SITE COUNT, DELETION-CLASS, GREPPED OVER ALL THREE SIDECARS AND THE PLAN TOML. `grep -rn 386` over the fold returns five lines: `workflow-enforcement-tier.md:306`, `:308`, `test-tmpdir-repo-assumption.md:3`, `:66`, `plan.toml:1322`. Only `:308` and `test-tmpdir-repo-assumption.md:66` state it as an EXPECTATION. Delete " (386 expected)" from `:308`, leaving "Suite and lint: `cargo test` and `cargo clippy --all-targets -- -D warnings`, both clean", which is the property that actually settles the check; and delete "386 expected, 0 failed." from `test-tmpdir-repo-assumption.md:66`, whose preceding clause already states the discriminating property. Two deletions, no prose authored. The two narrative sites can stand; if the fix pass wants them exact, "386 at the time of writing" is a one-clause hedge of the kind `:72` already uses for the record count.

---

## `R3B-4`. Severity `low`. The Defect D sweep result is short by three sites in two files

TENSE APPLIED: PRESENT. This is a descriptive claim about the pack as it stands, so the present tense is the right test, and it fails it.

THE TEXT. `workflow-enforcement-tier.md:146`:

> The only two places the pack mentions `docs/metrics/workflow.jsonl` outside the instrumentation section are `pack/AGENTS.md:61` and `:63`, both inside "When instrumentation is on" clauses [...]

RE-MEASURED. `grep -rc "docs/metrics/workflow.jsonl" pack/` at `5169ea0`:

```
pack/prompts/orchestrator.md:1
pack/instrument.md:1
pack/LEDGER.template.md:2
pack/AGENTS.md:2
```

"The instrumentation section" is `pack/instrument.md`, rendered into the `{{instrument}}` slot at `pack/AGENTS.md:116`. Outside it the pack mentions the round log in FIVE places across THREE files, not two across one: `pack/AGENTS.md:61`, `:63`, `pack/LEDGER.template.md:3`, `:9`, and `pack/prompts/orchestrator.md:19`.

THE ARGUMENT SURVIVES AND IS IN FACT STRENGTHENED, WHICH IS WHY THIS IS `low` AND NOT HIGHER. I read all three uncited sites and every one is conditional in the same "When instrumentation is on" form the sentence describes:

- `pack/LEDGER.template.md:3`: "When instrumentation is on, the structured per-round data (the round log the mechanical tooling reads) lives in `docs/metrics/workflow.jsonl`, not here."
- `pack/LEDGER.template.md:9`: "When instrumentation is on, the orchestrator ALSO appends a `round` record for the same round to `docs/metrics/workflow.jsonl`; the core counting reads this narrative, not that log."
- `pack/prompts/orchestrator.md:19`: "When instrumentation is on, ALSO append a `round` record for the same round to `docs/metrics/workflow.jsonl`; the counting below reads the narrative, not that log."

So Defect D's conclusion holds exactly as stated: `pack/AGENTS.md:93` is the sole unconditional promise, and every other mention gates itself. Only the count is wrong.

WHY IT IS WORTH FIXING ANYWAY, AND WHY BOTH PREVIOUS ROUNDS PASSED IT. Round 1's triage confirmed "`docs/metrics/workflow.jsonl` appears exactly twice in `pack/AGENTS.md`, at `:61` and `:63`", which is a NARROWER claim than the sentence makes. The sentence says "the pack", and it says so in a clause that then moves to a different file (`pack/instrument.md`), so the whole-directory reading is the natural one. This project has already ruled the identical shape valid: round 1's `F-1` was `VALID` at `low` for "Four constraint attributes" beside a list of five, on the reasoning that the argument is unaffected and the cost is a reader who greps and finds a different number. `:198`'s "found by sweeping ... rather than by patching the one already known" shows the file treats its stated sweep results as claims a reader may rely on.

WHY `low`. One wrong number in a sentence whose conclusion is correct, in a section that decides no behaviour. Same band as `F-1`.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND THE PLAN TOML. `grep -rn "only two places"` returns one hit, `workflow-enforcement-tier.md:146`. The cheapest true form is a narrowing rather than a re-count, which authors nothing new: change "the pack" to "`pack/AGENTS.md`", making the existing citation pair exact. If the fix pass prefers the wider true statement, "every other mention of the round log in the pack sits inside a 'When instrumentation is on' clause" says more and costs one clause; I would take the narrowing.

---

## `R3B-5`. Severity `low`. Inc3's acceptance check pins a property the pre-fix build already satisfies, so the new failure's message is unpinned and the literal minimal implementation ships a self-contradicting one

TENSE APPLIED: BOTH. The clause is TRUE of today's tree, which is precisely why it cannot detect the change.

THE CHECK. `workflow-enforcement-tier.md:329`:

> 15. AFTER INC3, defect A is closed: from inside the fixture, `agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow` exits NON-ZERO and reports the missing log BY PATH.

MEASURED AGAINST THE PRE-FIX BUILD. Fixture rebuilt by the command at `:28` into `/tmp/r3b-scratch/fixture` ("Wrote to ... (30 changed, 0 left untouched)", `ls docs` prints only `plans`), then from inside it with the debug binary built at `5169ea0`:

```
$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
stdout: docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
stderr: no metrics log at docs/metrics/workflow.jsonl; nothing to validate
stderr: --workflow has a plan source but the metrics log is missing; skipping the workflow check
exit:   0
```

The first stderr line is `src/main.rs:845`, `eprintln!("no metrics log at {}; nothing to validate", metrics_path.display())`, and it fires on the missing-log path independently of `--workflow`. So "reports the missing log BY PATH" IS ALREADY TRUE OF THE PRE-FIX BUILD. Only "exits NON-ZERO" discriminates, and `:51` says as much in the file's own words: "The operative defect is therefore the EXIT CODE and nothing else."

THE CONSEQUENCE IS NOT THAT THE CHECK IS USELESS, IT IS THAT A STATED REQUIREMENT IS UNPINNED. Two other places require the new failure to speak for itself: `:55` ("the run exits non-zero and reports why") and `:256` ("After inc3: a HARD FAILURE naming the path it looked for"). Because `src/main.rs:845` supplies the path unconditionally, an implementation that satisfies neither passes check 15 and check 18.

THE BAD IMPLEMENTATION IS THE LITERAL ONE, WHICH IS WHAT MAKES THIS WORTH RAISING. `:276` instructs "The `_` catch-all at `src/main.rs:999-1003` becomes a reported problem". I read the arm; it is:

```rust
_ => eprintln!(
    "--workflow has a plan source but the metrics log is missing; skipping the workflow check"
),
```

The smallest edit satisfying `:276` is `eprintln!` -> `problems.push(...)` with that string unchanged. The shipped result is a run that exits 1 while its own problem line says "skipping the workflow check", which is false after inc3 and is the opposite of what happened, and it names no path of its own. That output passes check 15, passes check 18, passes check 16 and 17, and passes `cargo test`.

WHY `low`. The band is round 2's `INC2-1` ("THE REQUIREMENT IS PINNED BY NO CHECK, AND THIS IS THE OPERATIVE HALF"), rated `low` there for the same reason: the requirement is stated plainly elsewhere in the file, and a reviewer of a `risky` increment with two required clean rounds is reading the sidecar to catch a message that contradicts its own exit code. It stops well short of `medium` because the discriminating half of check 15 works and defect A really is closed by it.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND THE PLAN TOML. Single-site, and it is a substitution inside an existing check rather than new narrative, which is the shape round 2's triage preferred for the identical problem. `grep -rn "BY PATH"` over the fold returns one hit, `workflow-enforcement-tier.md:329`. Replace "reports the missing log BY PATH" with a clause requiring the REPORTED PROBLEM (not merely the run's stderr) to name the resolved path and to state that the check could not run, so the stale "skipping" wording fails the check. That commits the plan to no mechanism and converts the currently-passing bad outcome into a failing one.

---

## `R3B-6`. Severity `low`. Inc3's risk argument states a denominator the fold's own review loop has already moved

TENSE APPLIED: PRESENT, since the sentence describes the project's calibration data as it stands.

THE TEXT. `workflow-enforcement-tier.md:300`, the last factor in inc3's risk classification:

> [...] step 92 spent six rounds and fifteen findings on one such claim, all of them prose, zero mechanism defects, joint-third of **seventy-seven** artifacts ever reviewed against a project median of two rounds [...]

WHAT REPRODUCES EXACTLY, AND I CHECKED EVERY NUMBER IN THE SENTENCE RATHER THAN THE ONE I SUSPECTED. Step order 92 is `prompt-drift-guard` (`plan.toml:1256-1259`). Over `docs/metrics/workflow.jsonl` at `5169ea0`:

```
$ jq -c 'select(.type=="round" and .step=="prompt-drift-guard") | {outcome,valid_findings}'
{"outcome":"new_valid","valid_findings":4}
{"outcome":"new_valid","valid_findings":3}
{"outcome":"new_valid","valid_findings":5}
{"outcome":"new_valid","valid_findings":1}
{"outcome":"new_valid","valid_findings":2}
{"outcome":"clean","valid_findings":0}
```

SIX rounds, 4+3+5+1+2+0 = FIFTEEN valid findings. Both exact.

WHAT DOES NOT. Grouping round records by `task`, which is the grouping the ledger used (its distribution at `docs/plans/agent-scaffold.ledger.md:387` is "1 round x16, 2 x35, 3 x10, 4 x6, 5 x6, 6 x2, 7 x1, 9 x1", summing to 77, and my measurement matches it on six of the eight buckets), the count at `5169ea0` is EIGHTY, not seventy-seven:

```
$ jq -r 'select(.type=="round") | .task' docs/metrics/workflow.jsonl | sort -u | wc -l
80
```

I pinned the drift to the commit rather than asserting it. `git log -S "Across all 77 artifacts" -- docs/plans/agent-scaffold.ledger.md` returns `110a8f8`, and at that commit the same query returns 77. The three tasks added since are `checks-runner-worktree-name-collision-inc1`, `decision-folder-currency-inc1` and `workflow-enforcement-tier-fold`. The last of those is THIS FOLD'S OWN REVIEW LOOP, so the artifact's own reviewing is one of the three things that falsified its number.

THE ARGUMENT IS UNAFFECTED AND I CONFIRMED BOTH LOAD-BEARING HALVES INDEPENDENTLY. The current distribution is 1 x16, 2 x37, 3 x10, 4 x6, 5 x7, 6 x2, 7 x1, 9 x1. Positions 40 and 41 of 80 both fall in the 2-round bucket, so the MEDIAN IS STILL TWO. The ordered top is 9, 7, 6, 6, so `prompt-drift-guard`'s six is STILL JOINT-THIRD. Only the denominator moved.

WHY `low`, AND WHY I RAISED IT AT ALL. The impact is a reader who re-derives the figure and finds a different one, which is seconds, and the risk classification does not move. I raise it because the brief asked for count claims to be re-measured rather than trusted, because the sentence carries no hedge of the kind `:72` gives the record count ("the record count grows as the log accumulates"), and because this project has twice ruled the same shape valid (`F-1`, `EX-8`). If the fix pass judges a running total not worth chasing, the honest resolution is a hedge rather than a new number, since the next round moves it again.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND THE PLAN TOML. `grep -rn seventy-seven` returns one hit, `workflow-enforcement-tier.md:300`. Two forms, both one word or one clause: substitute "eighty", or delete "of seventy-seven" so it reads "joint-third of the artifacts ever reviewed", which is the deletion-class form, stays true as the log grows, and loses nothing the argument uses. I would take the deletion.

---

## `R3B-7`. Severity `low`. Inc1's risk paragraph attributes a measurement that includes candidate (b) to inc1's scope, and the same file separately sizes candidate (b) at "roughly 80 lines"

TENSE APPLIED: PRESENT, since this is a fidelity claim about what a committed exploration record measured.

THE TEXT. `workflow-enforcement-tier.md:296`, the counter-argument inc1's risk classification rejects:

> The counter-argument, that the eventual diff is small (A measured the anchor at `+79/-15` and **the whole thing including siblings and ledger** at `+163/-18`, mostly comments) and that there is a deterministic test for it, loses to the blast radius [...]

WHAT THE SOURCE ACTUALLY SAYS. `docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md:474-476`, verbatim:

```
- Candidate (a) alone, Route B: **+79 / -15**, and most of the additions are doc comments. The executable rule is about 20 lines.
- Candidate (a) via Route A (`value_source`): **+96 / -13**, for the same behaviour plus the debug/release hazard.
- Full build, (a) + (b) + the sibling and ledger extensions: **+163 / -18**.
```

`+163/-18` is `(a) + (b) + the sibling and ledger extensions`. Candidate (b) is the containment guard, which `:275` assigns to INC2 and `:274` explicitly excludes from inc1 ("NO new REFUSAL mechanism"). The sidecar's phrase enumerates inc1's parts exactly (anchor, siblings, ledger) while quoting a number that includes a fourth part belonging to a different increment.

THE FILE'S OWN ARITHMETIC AGREES WITH THE EXPLORATION AND DISAGREES WITH `:296`. `:298` closes inc2's risk classification with "The counter-argument, that it is roughly 80 lines sharing inc1's derivation". 163 - 79 = 84, which is where "roughly 80" comes from. So the same document uses `+163` as inc1-plus-inc2 in one paragraph and labels it as inc1 alone two paragraphs earlier; if both readings were right, inc2 would be zero lines.

WHY `low`, AND I CONSIDERED NOT RAISING IT. Both statements sit inside counter-arguments their own paragraphs reject, so an inflated inc1 number only strengthens the rejection and changes no conclusion. What keeps it above zero is that an implementer sizing inc1 against `+163/-18` is being pointed at a diff that includes work `:274` forbids inc1 from doing, on a step whose entire structure is increment separation. `:274`'s scope statement is unambiguous enough that a careful implementer will not act on the number, which is why this is the smallest of the seven.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND THE PLAN TOML. `grep -rn '163/-18'` returns one hit, `workflow-enforcement-tier.md:296`. Two deletion-class forms: strike ", and the whole thing including siblings and ledger at `+163/-18`" so only inc1's own `+79/-15` is cited, or keep the number and correct its label to name candidate (b) as A's exploration does. The first authors nothing and I would take it, since `:298` already carries the inc2 half of the arithmetic.

---

## Enumeration: what I swept, with the negative results

This section lists what I read, what I reproduced, and what I checked and did NOT raise, because round 2's triage established that enumerating coverage is what makes a miss attributable.

### Sections of the primary sidecar read in full, under my lens

`:1-21` (title, four-defect summary, provenance, second-pass notice); `:23-31` (the fixture); `:33-61` (Defect A); `:63-113` (Defect B); `:115-136` (Defect C); `:138-148` (Defect D); `:150-166` (The mechanism, decided rather than chosen here); `:240-250` (Candidate (d) is rejected); `:252-260` (The two accepted costs); `:262-266` (What this step does not fix); `:268-290` (The three increments, and why in this order, including the four-call-sites paragraph at `:278`, the predicate paragraph at `:280`, the omit-placement paragraph at `:282`, the JSON-placement paragraph at `:284`, the cost-of-placement paragraph at `:286`, the documentation paragraph at `:288` and the ordering paragraph at `:290`); `:292-300` (Risk classification, all three arguments); `:302-334` (the whole acceptance list, with checks 1 to 10, 15 to 20 read as mine); `:336-365` (Documentation impact, all three lists); `:367-381` (Scope).

`:168-238` (the `Q-55-refusalscope` and `Q-55-jsonreason` sections, INC2's content) was read for context only and is not reviewed here, per the brief. `test-tmpdir-repo-assumption.md` and `status-resume-ignores-json.md` were read for the cross-references this fold makes to them.

### Commands run, with results

- `cargo test` -> 373 + 5 + 1 + 1 + 3 + 1 + 2 = 386 passed, 0 failed. `cargo build` clean.
- `agent-scaffold render docs/plans/agent-scaffold.plan.toml --check` -> "docs/plans/agent-scaffold.plan.toml: up to date", exit 0. The fold is render-clean at `5169ea0`.
- `grep -c . docs/metrics/workflow.jsonl` -> 241 (240 at round 2, 239 at round 1, as the brief notes).
- Fixture rebuild by the command at `:28` -> "Wrote to /tmp/r3b-scratch/fixture (30 changed, 0 left untouched)"; `ls "$SCRATCH/docs"` -> `plans` only. Check 2 reproduces.
- DEFECT A reproduces exactly as `:44-49` states, with 241 in place of the historical count: stdout one ok line, stderr both notes, exit 0.
- DEFECT B reproduces exactly as `:74-79` states: `docs/metrics/workflow.jsonl: 241 records, valid`, then the fixture's plan line, then `... vs docs/metrics/workflow.jsonl: workflow invariants hold`, exit 0.
- CHECK 7'S RED reproduces first-hand: with a fixture plan copied to `agent-scaffold.plan.toml`, `status --resume --source "$SCRATCH/docs/plans/agent-scaffold.plan.toml"` run from the repository root printed this repository's own `## RESUME STATE (compaction checkpoint, read this first)` block verbatim, exit 0.
- ACCEPTED COST (i) reproduces as `:256` describes: `cd docs/plans && validate --source agent-scaffold.plan.toml --workflow` -> exit 0, stderr "no metrics log at docs/metrics/workflow.jsonl; nothing to validate" plus the skip note, and the repository's real 241-record log is never read.
- SELF-RENDER COMPARISON, run into a scratch directory rather than in place: `scaffold --output-dir /tmp/r3b-scratch/selfrender --write --force --principles default --instrument`, then `diff` against the committed copies -> ZERO differing lines for both root `AGENTS.md` and `.agents/AGENTS.reference.md`. So `:364`'s regeneration instruction is correct AND its diff will be exactly the qualifier, not a whole-file reflow.
- `jq` measurements over `docs/metrics/workflow.jsonl` for `R3B-6`, plus the same query at `110a8f8` for the baseline.

### Code and asset citations opened and checked, all in the sections under my lens

EVERY ONE BELOW RESOLVES AS THE SIDECAR STATES IT, unless noted.

`src/main.rs`: `:416-420` (the `--instrument` gate, off by default); `:429-431`, `:455-457`, `:479-481` (the three `--metrics` declarations with the relative `default_value`); `:438-440` (the `--workflow` help); `:461`, `:464-466`, `:482-484` (the three ledger-default prose sites, including the `:461` that `EX-6` added); `:465` (`requires = "resume"`); `:791-816` (the `run_validate` doc comment; I checked the ellipsised quote at `:360` word for word against `:794-797` and it is accurate, and I note the range also covers `:808-811`, a second sentence inc3 falsifies, so the citation is complete); `:823-847` (the metrics read) and `:845` (the path-naming stderr note); `:939` (the `if args.workflow` gate, which is what makes checks 10 and 16 hold); `:958-1004` (the four-arm match, and I enumerated the residue reaching the `_` arm: exactly `(Some(_), _, None)` and `(None, Some(_), None)`, so `:300`'s "covers BOTH the TOML-source-present and the Markdown-plan-present variants" is exactly right and nothing else falls through); `:992-998` (the sibling arm plus its comment; `F-3`'s widened citation is correct and the quote at `:59` is verbatim); `:995-998`; `:999-1003`; `:1007-1011` (the success branch); `:1090` (`status`'s metrics existence test); `:1133-1138` and `:1136-1138` (`default_ledger_path`); `:1147-1151` and `:1150-1151` (`run_resume`; `F-4`'s widened citation is correct); `:1154` and `:1207` (the two `default_ledger_path` call sites `:274` names, and `grep` confirms there is no third); `:1200-1205`; `:1208-1212`.

`src/workflow.rs`: `:180-195` (`check_workflow_toml`, exactly the function); `:448-449` (the bare-slug W3 join).

`src/plan/source.rs`: `:480-495` (`is_safe_sidecar_ref`, doc comment plus body); `:102` (`#[serde(deny_unknown_fields)]` on `Meta`).

`src/plan/render.rs`: `:296` (`meta.title`) and `:167-169` (`meta.sidecars`). I also ran `grep -n '\.meta\.' src/plan/render.rs` and those are the ONLY three, so `:248`'s "render reads only `meta.title` and `meta.sidecars`" is a complete claim, not a sample.

`src/next.rs`: `:997-999` (`source.as_ref().or(plan.as_ref())`, which is the source-then-plan order `:274` anchors on); `:730` (the doc comment `EX-9`'s fix redirected to, and it does support the statement it is now attached to); `:1038` (the `RESUME STATE (verbatim from the ledger):` echo check 7 names).

`src/agents_md_drift.rs`: `:403-404`, `:408-409`, `:451-452` all pass BOTH sides through `normalize_wrapping`, so `:364`'s "the drift guard passes both sides through `normalize_wrapping` before comparing" is true, and skipping `nix fmt` really does cost nothing.

`tests/validate_workflow_toml_source_needs_no_plan.rs`: `:1-13` (the module doc does frame the false-green rule as being about the plan source only); `:89-132` (the test pinning the sibling arm); `:96-98` (the soft-skip comment, quoted verbatim and correctly).

`pack/AGENTS.md`: `:61`, `:63` (both "When instrumentation is on"); `:93` (the backstop sentence, present verbatim and unconditional, in the "Worktree lifecycle and merge-back" paragraph); `:116` (`{{instrument}}`); `:79` ("it must not run repo-wide formatters (for example `just fmt` or `nix fmt`)", which is what `:364`'s "which the pack forbids an implementer" rests on, and it holds).

`pack/instrument.md`: the `validate` sentence at `:13` and the closing line, both quoted accurately at `:146` and `:365`.

`justfile:46-48`: `scaffold-self` is the render followed by `nix fmt`, as `:364` says.

`README.md`: `:210` (the `validate` paragraph; the quoted clause "exits non-zero if any exist, so it can gate a commit or run in CI" is verbatim and it does not mention a `--workflow` run that cannot see a log); `:212-224` (the example block); `:226` (the `status` paragraph, both contract halves verbatim); `:228-237` (the `status` example block).

`CHANGELOG.md`: `## [Unreleased]` at `:7` with `### Added` at `:9` and `### Changed` at `:20`; `grep -n '^### Fixed'` returns nothing.

`docs/plans/agent-scaffold.plan.toml`: `workflow-enforcement-tier` at `:1297`, `order = 94`, with the three `[[step.increment]]` blocks at `:1308-1318` carrying ids `workflow-enforcement-tier-inc1/2/3` and `risk_class = "risky"` on all three, matching `:296`, `:298` and `:300` exactly. Cross-referenced step orders all confirmed: `sidecar-ref-empty-string` 63 `deferred`, `sidecar-ref-symlink` 64 `deferred`, `reviewer-reproducible-evidence` 88 `complete`, `test-tmpdir-repo-assumption` 95, `status-resume-ignores-json` 96.

`docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md`: `:82` and `:474-476` (the three diff measurements), which is where `R3B-7` comes from.

The three test function names at `:306` all exist: `src/checks.rs:1478`, `src/main.rs:1736`, `src/main.rs:2325`.

### Negative results: checked under the forward tense, no finding

- NO EXISTING TEST FLIPS FROM PASS TO FAIL UNDER INC3, and I checked this rather than assuming it, because inc3 is designed to make a passing gate fail. `grep -rn '"--workflow"' tests/ src/` returns exactly four invocations: `validate_workflow_toml_source_needs_no_plan.rs:73`, `:103`, `:119` and `validate_toml_primary_skips_markdown_plan.rs:95`. I read all four fixtures. Every one writes a PRESENT (empty) metrics log before invoking (`:67`, `:99`, `:84` respectively), so none reaches the `_` catch-all inc3 converts. Check 1's "both clean" survives inc3 on the pre-existing suite.
- INC3'S PACK EDIT CANNOT BREAK THE INSTRUMENT GATING TEST. I checked whether a qualifier at `pack/AGENTS.md:93`, which renders into BOTH tiers, could trip a negative assertion about the non-instrumented render. `src/main.rs:1943` asserts only that the off render lacks the string "Instrumentation (metrics logging)" and `:1944` that it lacks `{{instrument}}`; `:1958`'s `docs/metrics/workflow.jsonl` assertion is on the ON render and is positive. No test asserts the off render lacks the log path, so the qualifier is free to name instrumentation.
- CHECK 9'S BYTE-IDENTICAL PROPERTY SURVIVES THE DERIVATION, which I derived rather than trusted. For `--source docs/plans/agent-scaffold.plan.toml`, the parent is `docs/plans`, whose own file name is `plans` and whose parent's is `docs`, so the rule matches and the root is that ancestor's grandparent, the EMPTY path. Joining `docs/metrics/workflow.jsonl` onto the empty path yields `docs/metrics/workflow.jsonl` unchanged, so all three printed lines stay byte-identical and a relative source keeps a relative printed path, exactly as `:316` requires.
- ACCEPTED COST (i)'S "STRUCTURALLY CANNOT CATCH IT" CLAIM HOLDS UNDER INC2 AS WELL AS INC1, which the file needs since check 18 spans both. For `cd <root>/docs/plans && validate --source p.plan.toml`, the lexical default falls back to the source's own directory and looks under the CWD, while the canonical guard derives `<root>` from the canonicalised source; the wrong path `<root>/docs/plans/docs/metrics/workflow.jsonl` is still under `<root>`, so the guard passes and the miss stays silent until inc3 makes it loud. Consistent at every stage.
- THE `..`-COMPONENT EXPLANATION AT `:162` IS CORRECT. `Path::file_name` returns `None` for a path terminating in `..`, so an ancestor ending in `..` fails the "own file name is `plans`" test and the walk continues past it to the real `docs/plans` above. The stated mechanism is the actual one.
- `:274`'s ANCHOR ORDER MATCHES THE PRECEDENT IT CITES. `derive_task` is `source.as_ref().or(plan.as_ref())` at `src/next.rs:997-999`, so source-then-plan is the existing behaviour and inc1 reuses it rather than inventing an order. This closes round 1's `EX-10` sub-claim 2 and I found no residue of it.
- INC2-4'S FIXTURE-NAMING FIX LANDED CLEANLY, AND I RE-MEASURED IT RATHER THAN ASSUMING IT. `grep -c '\$FIXTURE'` over the primary sidecar returns 0, so the undefined variable is gone. `grep -n 'FIXTURE'` still returns two hits, `:310` and `:314`, and I read both: they are the English word in capitals ("naming the FIXTURE's own missing log path", "must print the FIXTURE's ledger"), which is the same class round 2's triage already cleared at `:309`, not a surviving variable. `$SCRATCH` is now used at `:28`, `:31`, `:38`, `:69`, `:76`, `:77`, `:86`, `:93`, `:98`, `:309`, `:310`, `:312`, `:313`, `:314`, `:318`, `:322`, `:323`; `p.plan.toml` survives only at `:256` and `:332`, both the generic accepted-cost example, which is correct.
- `:278`'S "FOUR CALL SITES" IS COHERENT AND I DID NOT RAISE IT. I counted the actual sites: three `--metrics` declarations (`src/main.rs:430`, `:456`, `:480`) and two `default_ledger_path` calls (`:1154`, `:1207`), and `grep` confirms there is no fourth `--metrics` default anywhere in `src/main.rs`. "Four" therefore means the four SURFACES `Q-55-mechanism` names (validate, status, next, the ledger path), and while "They share one function" strictly covers only the first three, the same paragraph then argues the ledger in separately and explicitly on its own grounds ("a genuinely DIFFERENT rule ... but it is roughly ten lines"). The enumeration is loose, not wrong.
- `:274`'s "any new non-zero exit comes from the pre-existing W3 check" IS NARROWER THAN THE TRUTH AND I DID NOT RAISE IT. A correctly-anchored log that is malformed exits 1 through `src/main.rs:835-841`, and W4 or W5 could also newly fire, so W3 is not the only route. The property the clause asserts (inc1 adds no new refusal mechanism, and every new red is a pre-existing check finally seeing the right data) is true of all of them, so naming W3 narrows the example without weakening the claim.
- THE "FIVE RETROSPECTIVE AND ONE PROSPECTIVE CONFIRMATION" CLAIM AT `:363` IS ARGUABLY UNDERCOUNTED NOW AND I DID NOT RAISE IT. Round 2's triage ruled `INC2-2` fix-induced by the round 1 fix's prose content, which is a further confirmation of the same hypothesis. But the sentence is an inventory rather than an exhaustive count, it says nothing that a sixth confirmation falsifies, the calibration it supports is unchanged, and I could not pin down from the ledger which prediction "the one prospective" names. A finding here would not survive triage on its conclusion.
- `:365`'s "NOT the role prompts: no prompt states where the log is resolved from" HOLDS despite `pack/prompts/orchestrator.md:19` naming `docs/metrics/workflow.jsonl`. That line tells an orchestrator where to WRITE the record inside a project, which is unchanged; it states no resolution rule for an invocation from elsewhere.
- `:365`'s "NOT `pack/instrument.md`" HOLDS. Its `validate` sentence at `:13` stays true under inc3 (it is about malformed records, not about a missing file), and its log-path references describe where the log lives inside a project, which the anchor does not move.
- ACCEPTED RESIDUALS CONFIRMED PRESENT AND DELIBERATELY NOT RAISED: `INC2-7` (the over-determined `no_active_loop_reason`, whose narrowed correlation rule is at `:234`) and `F-5` (the dangling `validation-constraints` handle; `grep -c validation-constraints` over the primary sidecar returns 4 and the plan TOML still holds no such slug).

### One thing I could not settle and am flagging rather than raising

`:290`'s "Inc1 precedes inc2 because the guard reuses inc1's root derivation" sits beside `:280`'s "it deliberately uses a DIFFERENT resolution from the default" and `:166`'s "THE LEXICAL/CANONICAL SPLIT IS DELIBERATE AND MUST NOT BE COLLAPSED". I worked through whether those contradict and concluded they do not: the guard reuses the same lexical RULE applied to a CANONICALISED input, so the rule is shared and the input is not. That reading makes all three sentences true simultaneously. I record it because a reviewer arriving at those three sentences in a different order could read a contradiction where there is none, and because if a future round does raise it, this is the reasoning that dismisses it.
