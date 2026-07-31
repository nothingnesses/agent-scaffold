# `workflow-enforcement-tier` plan review, round 3: TRIAGE

Triager model: Claude Opus 5, 1M-context variant. Exact model id `claude-opus-5[1m]`.
Date: 2026-08-01.
Worktree: `.claude/worktrees/triage-q55-r3`, branch `triage/q55-r3` at commit `5169ea0`, the exact commit both reviewers reviewed.
Artifact triaged against: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (primary), `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`, `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, and the `[[step]]` / `[[question]]` entries this fold adds or changes in `docs/plans/agent-scaffold.plan.toml`. `src/`, `pack/`, `tests/`, `CHANGELOG.md`, `docs/metrics/workflow.jsonl` and `docs/plans/workflow-enforcement-tier.explorations/` are evidence, not artifact.

Findings files triaged: `workflow-enforcement-tier-planreview-r3-reviewer-residue.md` (`R3A-1`, the fix-verification and residue lens, `claude-sonnet-5`) and `workflow-enforcement-tier-planreview-r3-reviewer-inc13.md` (`R3B-1` through `R3B-7`, the inc1 / inc3 / risk / ordering lens, `claude-opus-5[1m]`). Rounds 1 and 2's five files were read as the record of what was already ruled.

`TMPDIR` was `/tmp/triage-r3-scratch`, outside any git repository.

## Result

8 findings triaged. 8 `VALID`, 0 `VALID BUT ACCEPT RESIDUAL`, 0 `DISMISSED`.

Adjusted severity: 0 critical, 0 high, 0 medium, 8 low. Every reviewer rating stands. I considered `medium` for `R3B-1` and for `R3B-5` and took neither; the reasoning is in each entry, and neither was held down to keep a count tidy.

NO FINDING WAS RULED `high` OR `critical` AND DISMISSED, so no backstop re-check is owed by this round.

| id | source file | reviewer severity | adjusted | verdict |
| --- | --- | --- | --- | --- |
| `R3A-1` | `-r3-reviewer-residue.md` | low | low | VALID |
| `R3B-1` | `-r3-reviewer-inc13.md` | low | low | VALID |
| `R3B-2` | `-r3-reviewer-inc13.md` | low | low | VALID |
| `R3B-3` | `-r3-reviewer-inc13.md` | low | low | VALID |
| `R3B-4` | `-r3-reviewer-inc13.md` | low | low | VALID |
| `R3B-5` | `-r3-reviewer-inc13.md` | low | low | VALID |
| `R3B-6` | `-r3-reviewer-inc13.md` | low | low | VALID |
| `R3B-7` | `-r3-reviewer-inc13.md` | low | low | VALID |

## What I re-measured first-hand

Every number either reviewer rested a finding on, re-measured rather than trusted. All of these were run in this worktree against the debug binary built at `5169ea0`.

- `cargo test`: 373 + 5 + 1 + 1 + 3 + 1 + 2 = 386 passed, 0 failed. BOTH reviewers' 386 is exact, and so is check 1's "386 expected" AS A STATEMENT ABOUT TODAY. That is the whole of `R3B-3`.
- `cargo clippy --all-targets -- -D warnings`: clean. `render docs/plans/agent-scaffold.plan.toml --check`: "up to date". `validate --source docs/plans/agent-scaffold.plan.toml --workflow`: 241 records, 95 steps, 69 questions, `workflow invariants hold`, exit 0. The fold is validate-clean and render-clean at this commit.
- `grep -c . docs/metrics/workflow.jsonl`: 241. Both reviewers reported 241; both are right (239 at round 1, 240 at round 2).
- DISTINCT REVIEWED TASKS: `jq -r 'select(.type=="round") | .task' docs/metrics/workflow.jsonl | sort -u | wc -l` returns 80, not the seventy-seven at `:300`. I pinned the drift independently of the reviewer: at `110a8f8` the same query returns 77, and `comm` against today gives exactly the three tasks `R3B-6` names (`checks-runner-worktree-name-collision-inc1`, `decision-folder-currency-inc1`, `workflow-enforcement-tier-fold`). The last is this fold's own review loop.
- THE FULL ROUND DISTRIBUTION, re-derived because `R3B-6`'s argument rests on the median and the ranking surviving: 1 round x16, 2 x37, 3 x10, 4 x6, 5 x7, 6 x2, 7 x1, 9 x1, summing to 80. Positions 40 and 41 both fall in the 2-round bucket, so the MEDIAN IS STILL TWO; the ordered top is 9, 7, 6, 6, so six is STILL JOINT-THIRD. Only the denominator moved. The ledger's own distribution at `docs/plans/agent-scaffold.ledger.md:387` ("1 round x16, 2 x35, 3 x10, 4 x6, 5 x6, 6 x2, 7 x1, 9 x1", summing to 77) reproduces as a historical record and is correctly dated inside a round-record narrative, so it is not a second site.
- THE PACK'S MENTIONS OF `docs/metrics/workflow.jsonl`: `pack/AGENTS.md:61`, `:63`, `pack/LEDGER.template.md:3`, `:9`, `pack/prompts/orchestrator.md:19`, plus `pack/instrument.md:3` (the instrumentation section). FIVE outside the instrumentation section, across THREE files, not two across one. `R3B-4` reproduces exactly. I read all three uncited sites and every one is conditional in the "When instrumentation is on" form the sentence describes, so the conclusion is unaffected.
- ONE FACT NEITHER REVIEWER NOR EITHER EARLIER ROUND ESTABLISHED, and it bears on `R3B-4`: the two files the sentence misses ARE deployed to a non-instrumented scaffold. My fixture rebuild wrote `.agents/LEDGER.template.md` and `.agents/prompts/orchestrator.md` among its 30 files with no `--instrument`. So the miscounted sites are text the very reader defect D is about does receive. This makes the count matter slightly more than the reviewer allowed and makes the argument slightly stronger, since all three are conditional.
- THE `+163/-18` FIGURE: `docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md:474-476` reads, verbatim, "Candidate (a) alone, Route B: +79 / -15 ...", "Candidate (a) via Route A (`value_source`): +96 / -13 ...", "Full build, (a) + (b) + the sibling and ledger extensions: +163 / -18." The line numbers are exact and the attribution `R3B-7` disputes is exactly as the finding states.
- CHECK 11's BEFORE-STATE, run literally: fixture rebuilt (30 changed; `ls $SCRATCH/docs` prints only `plans`; slug `example-step` at `TEMPLATE.plan.toml:34`, status `not-started` at `:36`), then `validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl --workflow` from the repository root gives `241 records, valid`, the fixture plan line, `workflow invariants hold`, exit 0. Reproduces.
- `R3A-1`'s DECISIVE NEGATIVE, run: with an EMPTY, LEGITIMATE, in-root log in the fixture, `validate --source docs/plans/TEMPLATE.plan.toml --workflow` from inside the fixture gives `0 records, valid`, the plan line, `workflow invariants hold`, exit 0. The "holds" verdict is not specific to the log being foreign.
- CHECK 15's BEFORE-STATE, run: from inside a fresh fixture, `validate --source docs/plans/TEMPLATE.plan.toml --workflow` prints on stderr `no metrics log at docs/metrics/workflow.jsonl; nothing to validate` and the skip note, prints the ok line on stdout, exit 0. The path IS reported by the pre-fix build. `R3B-5` reproduces.

## Deduplication, ruled before the findings

THE BRIEF'S QUESTION, ANSWERED PLAINLY: `R3A-1` AND `R3B-5` ARE ONE THEME AND TWO DISTINCT FINDINGS. Neither is merged into the other and both are separately valid.

The shared theme is real and worth naming: A CLAUSE OF AN ACCEPTANCE CHECK IS SATISFIED ON BOTH SIDES OF THE CHANGE, so the check demonstrates less than its own wording claims. Round 1's `EX-8` was the first instance in this fold (check 5's "must NOT print `metrics: 235 records`" was already true of the pre-fix binary), so this genus has now produced findings in rounds 1 and 3.

What separates them, and why merging would lose information:

- WHICH HALF IS WEAK. `R3A-1` is the BEFORE half of check 11: the after half discriminates perfectly (nothing exits non-zero on that input today), so no bad implementation passes; what is lost is only the evidentiary force of the red. `R3B-5` is an AFTER half clause of check 15: because the pre-fix build already names the path, a specific bad implementation both compiles and PASSES the check.
- WHAT THE FIX EDITS. `R3A-1`'s fix changes the FIXTURE the check runs against (add a precondition). `R3B-5`'s fix changes what the check ASSERTS about the output (substitute a clause). Different sites, different files' worth of content, and neither edit implies the other.
- ROOT CAUSE. These are not one root cause in the sense round 1's `EX-3` / `EX-7` were, where one corrected fact resolved both. There is no shared fact here, only a shared failure shape. So they need not be fixed in one pass for consistency, though doing so is cheap.

The rest of the round, checked for overlap:

- `R3B-3` AND `R3B-6` are one theme (a count stated as current that has moved or will move) and two distinct findings. They differ in tense (`R3B-3` is forward, the work itself moves the number; `R3B-6` is present, the number already moved), in file, and in fix. This is round 1's `EX-8` / `F-1` band.
- `R3B-4` AND `R3B-7` are one theme (a stated measurement that is wrong when re-measured against its own source) and two distinct findings. `R3B-4` is a count short by three; `R3B-7` is a correct number under a wrong label.
- `R3B-1` and `R3B-2` overlap nothing, each other or the rest.
- NOTHING RE-RAISES `INC2-7` OR `F-5`. Both reviewers explicitly confirmed each still present and deliberately did not raise it; I confirmed the same (`:234` carries `INC2-7`'s narrowed correlation rule unchanged, and `grep -c validation-constraints` over the primary sidecar returns 4 against a plan TOML with no such slug). Both accepted residuals stand.
- NOTHING RE-LITIGATES A DECIDED ITEM. I checked all eight against the decided list (the enforcement tier, the one-step multi-increment shape, anchor-plus-refusal with identity queued, the conventionless fallback, omit-and-exit-0 on the projections, the serialised reason, both accepted costs, nearest-wins, the open TMPDIR fork) and found no objection to any of them.
- FOUR FINDINGS ARE CONTINUATIONS OF EARLIER ROUNDS AT SITES THE EARLIER FIXES DID NOT REACH, and I verified the lineage claim in each case rather than accepting it: `R3B-1` (round 1's `EX-5`, verified below to be an unfixed pre-existing twin), `R3B-2` (`EX-6`'s shape), `R3B-3` and `R3B-6` (`EX-8`'s shape), `R3B-4` and `R3B-7` (`F-1`'s shape). None re-raises anything dismissed, because rounds 1 and 2 dismissed nothing.

---

## `R3A-1`. `VALID`. Severity `low` (unchanged). Check 11's before-state is a vacuous pass, so the check does not demonstrate the false pass its header names

TENSE APPLIED: PRESENT for the half under objection, and I say so explicitly because this finding and `R3B-3` / `R3B-5` differ on it. Check 11's "Before inc2 this prints `workflow invariants hold` at exit 0" is a claim about TODAY's tree, so today's tree is the right test. It passes that test: the claim is TRUE. The finding is not that it is false; it is that it demonstrates less than the check's own header ("the explicit-relative-`--metrics` false pass is refused") claims. The AFTER half is a forward claim and is untouched by this finding.

QUOTE VERIFIED, `workflow-enforcement-tier.md:318`, word for word as the finding gives it, including the fixture reference `"$SCRATCH/docs/plans/TEMPLATE.plan.toml"` that round 2's `INC2-4` fix put there.

BOTH RUNS REPRODUCE, in both directions, as recorded in the measurement section above: the literal before-command gives `workflow invariants hold` at exit 0 against agent-scaffold's own 241-record log, AND the identical verdict reproduces with an empty, legitimate, in-root log on a fixture with no `complete` step. `src/workflow.rs:17-18` carries the operative constraint verbatim ("W3 checks only `complete` steps; the others (`skipped` and the in-flight statuses) are not checked"), so a fixture whose single step is `not-started` gives W3 nothing to check and "holds" carries no information about whose log was read.

THE EVIDENCE THAT SETTLES THIS AGAINST A DISMISSAL, AND NEITHER REVIEWER USED IT. The obvious defence is that the file's own vocabulary licenses "false pass" for this shape, since `:81` calls even the non-borrowed-slug green "affirmatively wrong". I went to the source the check's header names. `docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md:211-220` is explorer A's "False pass 2, an explicit relative `--metrics`", and its transcript header reads "=== ROUTE B build ((a) only): BORROWED SLUG + EXPLICIT RELATIVE --metrics, from the worktree root ===". A's false pass 2 IS the borrowed-slug case. Check 11 names A's false pass ("the explicit-relative-`--metrics` false pass", and `:290` and `:298` both cite A's second false pass as this input) and then instantiates it WITHOUT the mutation that makes it one. So the shortfall is not a vocabulary quibble; the check drops a precondition its own named source had.

THE ASYMMETRY WITH CHECK 14b IS REAL AND I VERIFIED IT AS A MEASUREMENT. `grep -rno "Before inc2 this prints"` over all three sidecars and the plan TOML returns exactly TWO sites, `:318` (check 11) and `:322` (check 14b). Round 2's `INC2-4` fix gave `:322` the precondition ("with the fixture's single step carrying the borrowed slug `triager-runs-only-on-findings` at `in-progress`") and gave `:318` only the variable and filename half of the same fix. So the fold now carries the two before-state claims at different evidentiary strengths, and the weaker one is the validator's.

WHY `low` AND NOT HIGHER, AND I ATTACKED THE UPGRADE. Check 11's after half is fully discriminating: nothing in the tree exits non-zero on that input, so the check is a genuine red-then-green for inc2's refusal and an implementer who builds the refusal wrong fails it. No bad implementation passes. Every individual sentence in the check is true. What is lost is that a reviewer running check 11's before-command believes they have reproduced A's false pass when they have reproduced something weaker, on a check `:304` designates as one of inc2's three required `Q-66` red cases. That is an evidentiary shortfall in a designated red case, not a false statement and not an admitted defect, which is the bottom of `low`.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. SINGLE-SITE, one clause inside an existing check, and it is a copy of a clause the fold already carries rather than new prose: at `:318`, state the fixture precondition check 14b now states, the borrowed slug `triager-runs-only-on-findings` at `status = "complete"` (the mutation `:87-89` already spells out and check 4 at `:311` already performs). That converts the before-state into the genuine false pass and costs no new vocabulary.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS A LITERAL GREP CANNOT MATCH:

- `Before inc2 this prints`: 2 sites, `:318` and `:322`. `:322` is already correct and MUST NOT CHANGE.
- `Before inc2` (wider): 3 sites, `:318`, `:322`, `:325`. `:325` is check 14e, whose before-state is structural ("Before inc2 neither field exists at all"), fully discriminating, and unaffected.
- THE SIBLING REFUSAL CHECKS, read rather than grepped, because they are the natural twin: checks 12 (`:319`) and 13 (`:320`) state no before-state claim at all, so they do not carry this defect and need no edit. Check 4 (`:311`) already carries the precondition. Check 17 (`:331`) already uses the borrowed-slug fixture.
- `triager-runs-only-on-findings`: 8 sites in the primary sidecar (`:88`, `:90`, `:103`, `:119`, `:264`, `:311`, `:322`, `:331`) and 4 in the plan TOML (`:1057`, `:1069`, `:1760`, `:1762`, all a real unrelated step's slug and a `Q-63` receipt, a pre-existing naming coincidence and out of scope). `:318` is the only acceptance check that states a before-state false pass and does not name the slug.

ONE SITE. Do not widen it.

---

## `R3B-1`. `VALID`. Severity `low` (unchanged). The ordering paragraph's generalisation is false of both earlier increments, and it is round 1's `EX-5` at a twin the `EX-5` grep could not match

TENSE APPLIED: FORWARD. The sentence is a claim about what the three increments will do; against today's tree it is not evaluable at all, since no increment has landed.

QUOTE VERIFIED at `workflow-enforcement-tier.md:290`, in full and not only in the elided form the finding gives. The operative clause is "the tier policy goes LAST because it is the only increment that makes a previously-green run fail, and EVERY escape hatch a user reaches for when it does is closed by an earlier increment".

BOTH FALSIFIERS VERIFIED.

- INC2. `:298` opens "It INTRODUCES a non-zero exit on validator invocations that succeed today AND withholds output from projection invocations that answer today". That is a previously-green run made to fail, in inc2's own words, eight lines below the claim. Checks 11 (`:318`), 12 (`:319`) and 13 (`:320`) each pin one.
- INC1. `:274` now reads "NO new REFUSAL mechanism: any new non-zero exit comes from the pre-existing W3 check finally running against the right project, which is check 4's whole point", and check 4 (`:311`) requires "Give the fixture a log of its OWN with no evidence for that slug and expect the correct RED instead of the absence of a green." Round 1's triage measured the flip directly (`workflow-enforcement-tier-planreview-r1-triage.md:126-136`: the same command line, same files, exit 0 today and exit 1 after inc1). Both citations resolve.

THE `EX-5` LINEAGE CLAIM IS TRUE, AND I PINNED IT TO A COMMIT RATHER THAN INFERRING IT. `git show 6df032c:docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md | grep -n "only increment that makes a previously-green"` returns line 288 with the identical sentence, so the claim PREDATES round 1's fix pass and is not fix-induced. The same grep at `48eb015` returns 1. Round 1's triage scoped `EX-5` as "Single-site: line 272. (`grep -c "still exits 0"` returns 2 ...)", and line 290 makes the identical claim in words that grep cannot match. This is the `RES-1` shape exactly: the fix pass applied its stated scope faithfully and inherited the scope's blind spot. Before the fix both statements agreed and were both false; after it they disagree, so the fold now contradicts itself about whether inc1 can produce a new red.

A CORRECTION TO THE FINDING, AND IT STRENGTHENS THE PRESCRIPTION RATHER THAN WEAKENING IT. The reviewer offers, as a fallback if the fix pass wants to keep a positive claim, "inc3 is the only increment that makes a run fail for a reason OTHER than a pre-existing check finally seeing the right data". THAT FALLBACK IS ALSO FALSE. Inc2's containment refusal is a NEW mechanism (`:275`, "the predicate itself ... `validate --workflow` REFUSES (a problem, exit non-zero)"), not a pre-existing check seeing the right data, and `:298` says so. The reviewer was right to steer away from it on calibration grounds; it is additionally wrong on the facts. TAKE THE DELETION.

WHY `low` AND NOT `medium`, AND I CONSIDERED THE UPGRADE SERIOUSLY BECAUSE THE REVIEWER INVITED IT. Round 1 rated `EX-5` `medium` because the false claim WAS inc1's stated safety property, in inc1's own bullet, so an implementer reading their own increment description was misled. That is closed: `:274` states the true property unambiguously and check 4 demands the counterexample in the same list. What survives is a false premise in a paragraph about ORDERING whose conclusion is independently carried (the escape-hatch half of the same sentence, plus the first pass's workaround argument, both of which hold), contradicted on the facing page by `:298` and `:296`. Round 2 downgraded `INC2-1` to `low` on exactly this reasoning: the operative instruction is now correct, and a reviewer of a `risky` increment with two required clean rounds is reading the file to catch the discrepancy. LINEAGE FROM A `medium` FINDING IS NOT ITSELF A MERIT, any more than the round a finding arrives in is; severity is absolute impact if unfixed, and the containment here is strong. `low` stands, and this is nonetheless the finding I would fix first.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. SINGLE-SITE AND DELETION-CLASS, which is the best available shape. At `:290`, strike "because it is the only increment that makes a previously-green run fail, and", leaving "the tier policy goes LAST because EVERY escape hatch a user reaches for when it does is closed by an earlier increment", which is the true half and the half the argument runs on. Author no replacement clause.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS:

- `only increment that makes a previously-green`: 1 site, `:290`.
- `previously-green` (the wider phrase, in case a second sentence uses it differently): 1 site, `:290`.
- `still exits 0`, the string round 1 used and the most likely differently-worded twin: 1 site, `:321`, inside check 14 ("`validate --source <foreign> --metrics <local log>` still exits 0, since no pairing is asserted"). CORRECT AS WRITTEN. Do not touch it. Round 1's triage already cleared this same site for the same reason.
- `NO new failure`, `no new failure`, `exited 0 before`: 0 sites. The clause round 1 replaced is fully gone.
- THE TWO SENTENCES THE DELETION MUST NOT CONTRADICT, read rather than grepped: `:274` (inc1's corrected property) and `:298` (inc2's stated break) both remain true and both become consistent with `:290` once the clause is struck. No enumeration is headed by the deleted clause and no cross-reference restates it.

ONE SITE.

---

## `R3B-2`. `VALID`. Severity `low` (unchanged). The INC3 documentation-impact list omits the `CHANGELOG.md` entry inc3's own increment description requires

TENSE APPLIED: FORWARD, and the defect exists only under it. Nothing about `CHANGELOG.md` is wrong today; what is missing is an instruction for work inc3 must do.

VERIFIED IN EVERY PART. `:276` requires it verbatim ("the README `validate` paragraph and the CHANGELOG entry"). I read `:358` to `:365` in full: the INC3 list is exactly five content bullets (`src/main.rs:791-816`, the test file, `README.md:210`, `pack/AGENTS.md:93`, the two deployed copies) plus one NOT bullet, and carries no `CHANGELOG.md` item. The INC1 list ends with one at `:346` and the INC2 list ends with one at `:356`. Only INC3 lacks one.

THE ASYMMETRY ARGUMENT HOLDS AND I TESTED THE OBVIOUS DEFENCE. `:356` cannot be read as covering inc3: its stated scope is the new JSON fields and the withheld-output behaviour, and its closing clause contrasts inc2's projections against inc2's own refusal ("withheld output at exit 0 versus a non-zero exit"), not against inc3's tier policy. `:260` supplies the changelog content for inc2's deliberate break and has no inc3 counterpart anywhere in the fold.

THE FACTUAL PREMISES THE MISSING BULLET WOULD REST ON REPRODUCE. `CHANGELOG.md:7` is `## [Unreleased]`, `:9` is `### Added`, `:20` is `### Changed`, and `grep -n '^### Fixed' CHANGELOG.md` returns nothing, so `:346`'s description of the section is accurate and would apply unchanged.

WHY IT IS NOT A CROSS-REFERENCE NICETY. `:338` states the section's own contract ("each item travels with the increment that makes it stale rather than being left as a documentation step owed") and `:288` states there is "deliberately NO separate documentation increment". The list is therefore the enumerated shipping contract per increment, and inc3 is the increment `:300` calls out as changing "a CLI EXIT CODE, the most externally depended-on contract the tool has".

WHY `low`. This is `EX-6`'s band exactly and for `EX-6`'s reason: one missing item in an enumerated list, with the requirement stated correctly elsewhere in the same file (`:276`), misleading about an enumeration rather than about behaviour. Round 1 downgraded `EX-6` from `medium` to `low` on that reasoning and I hold the same line.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. SINGLE-SITE. Nothing can be deleted or narrowed to close an omission, and no numeral or citation edit reaches it, so this is the one finding in the round that needs new text; keep it to ONE CLAUSE. Add a `CHANGELOG.md` bullet to the INC3 list naming the exit-code flip and the non-instrumented population it breaks.

WHY MY PRESCRIPTION IS SMALLER THAN THE FINDING'S, AND SMALLER THAN THE PRECEDENT BULLETS. DO NOT COPY `:346`'s OR `:356`'s SHAPE. Both restate the section's structure ("The section today has `Added` and `Changed` and no `Fixed`; check what a comparable fix did before introducing a new subsection"), and a third restatement of a fact already in the fold twice is exactly the prose-authoring this project has measured as manufacturing the next round's finding. The new bullet needs the file, the section, and what the entry must say; it needs nothing about `Fixed`.

I CONSIDERED AND REJECTED A CHEAPER FORM: appending a clause to `:362`, the existing `README.md:210` bullet, which is the other user-facing release-documentation item. It is one rung cheaper on the deletion-first ladder, but it buries a file item inside another file's bullet and so partially reproduces the omission the finding is about, against a list whose INC1 and INC2 halves are strictly one file per bullet. Take the bullet.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS. `grep -rn CHANGELOG` over all three sidecars and the plan TOML returns 8 sites: `workflow-enforcement-tier.md:260`, `:275`, `:276`, `:346`, `:351`, `:356`; `status-resume-ignores-json.md:111`; `docs/plans/agent-scaffold.plan.toml:1704`. NONE is an INC3 documentation-impact item. I also swept for the twin ENUMERATION rather than the twin string: the fold contains exactly two enumerations of what inc3 ships, the increment bullet at `:276` (complete, names the CHANGELOG) and the Documentation impact INC3 list at `:358-365` (short by one). The `:276` half is correct and must not change. No numeral heads the list, so there is no count to keep in step with the added bullet.

ONE SITE.

---

## `R3B-3`. `VALID`. Severity `low` (unchanged). Check 1 pins the pre-change test count, which the increments are required to move

TENSE APPLIED: BOTH, AND THEY DISAGREE, WHICH IS THE FINDING. I state this explicitly because the brief asks and because it is the difference from `R3A-1`. Against TODAY's tree the number is exactly right, and I measured it (386, as 373 + 5 + 1 + 1 + 3 + 1 + 2). Against the tree the increments produce it is wrong BY CONSTRUCTION, because the same document requires new tests.

THE REQUIREMENT TO GROW IS THE DOCUMENT'S OWN, AND EVERY CITATION RESOLVES. `:304` ("EACH INCREMENT owes at least one test that is RED against the pre-fix build and green after"), `:274` ("the red-then-green tests", plural), `:275` ("its own red-then-green tests", plural), `:276` ("its own red-then-green test", singular), `:332` (check 18, "A test pinning this belongs in the suite"), `:333` (check 19, "the tests assert both"). `grep -rno red-then-green` over the fold returns exactly 3 sites, `:274`, `:275`, `:276`, matching the finding. The precise floor the reviewer infers ("at least six") is an inference rather than a measurement and I am not carrying the number, but the direction is certain: the suite must grow, so 386 cannot survive.

THE CHECK IS INVERTED AS THE FINDING STATES. `:304` says a round is settled by running the checks rather than by reading the diff, so read as written check 1 PASSES for an implementation that adds no test and FAILS for a correct one. Its "both clean" clause is what does the work; the number is the part that cannot survive the change it is checking.

WHY THIS IS A STRONGER CASE THAN ROUND 1'S `EX-8`, WHICH IS WORTH RECORDING FOR THE FIX PASS. `EX-8`'s stale literal drifted because an external log accumulated; this one is stale because THE WORK ITSELF MOVES IT. That is why the answer is a deletion and not a hedge: no hedge ("386 at the time of writing") makes an expectation useful when the expectation is guaranteed wrong on the day it is checked.

WHY `low`. `EX-8`'s band. The impact is a number an implementer will obviously adjust and a check whose remaining clause still discriminates; the failure mode is a puzzled implementer, not a shipped defect. Real, because it sits in the increments' own evidence list.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. DELETION-CLASS, TWO SITES, and my full measurement is FIVE so the fix pass does not have to re-derive it.

`grep -rn 386` over the fold returns exactly FIVE lines:

| site | text | is it an expectation | disposition |
| --- | --- | --- | --- |
| `workflow-enforcement-tier.md:306` | "Three of the suite's 386 tests (...)" | no, narrative denominator | may stand |
| `workflow-enforcement-tier.md:308` | "`cargo test` (386 expected)" | YES | DELETE " (386 expected)" |
| `test-tmpdir-repo-assumption.md:3` | "Three of the 386 tests require ..." | no, narrative denominator | may stand |
| `test-tmpdir-repo-assumption.md:66` | "386 expected, 0 failed." | YES | DELETE "386 expected, 0 failed." |
| `docs/plans/agent-scaffold.plan.toml:1322` | step title, "a false red, 3 of 386" | no, step title | may stand |

Both deletions leave the discriminating property behind: `:308` becomes "Suite and lint: `cargo test` and `cargo clippy --all-targets -- -D warnings`, both clean", and `test-tmpdir-repo-assumption.md:66` keeps "`cargo test` passes with `TMPDIR` set INSIDE a git repository (a worktree-local scratch directory), which is the case that fails today". No prose is authored at either.

THE SECOND EXPECTATION SITE IS IN A DIFFERENT FILE AND IS THE ONE A STEPS-DIRECTORY-ONLY SWEEP WOULD HAVE FOUND BUT A PRIMARY-SIDECAR-ONLY SWEEP WOULD HAVE MISSED. `test-tmpdir-repo-assumption` is `order = 95` (`plan.toml:1324`) and `workflow-enforcement-tier` is `order = 94` (`plan.toml:1300`), both verified, so by the time step 95 runs its own acceptance check this step's increments have already moved the count.

WHAT I SWEPT FOR SEMANTIC TWINS. The count is spelled as the numeral 386 at all five sites and nowhere as a word; `grep -rn "three hundred"` returns nothing. No numeral I am deleting heads an enumeration (unlike round 2's `INC2-6`), so no list falls out of step. The three narrative sites are NOT numeral twins of the two expectations in the sense that matters here: deleting an expectation creates no contradiction with a narrative denominator. THEY ARE IN THE SAME DRIFT CLASS, though, and unlike round 1's protected `235` sites they are NOT framed as historical, so they will be wrong of the post-increment tree. The same deletion works at all three ("Three of the suite's tests", "Three tests", "a false red, 3 tests"). I am scoping the FINDING to the two expectations, because those are what it establishes; the fix pass may take all five in one deletion pass and I would not call that over-scope. If it takes only two, take those two.

TWO SITES MANDATORY, FIVE MEASURED.

---

## `R3B-4`. `VALID`. Severity `low` (unchanged). The Defect D sweep result is short by three sites in two files

TENSE APPLIED: PRESENT. This is a descriptive claim about the pack as it stands, so the present tense is the right test, and it fails it.

QUOTE VERIFIED at `workflow-enforcement-tier.md:146`, word for word.

RE-MEASURED INDEPENDENTLY. `grep -rn "docs/metrics/workflow.jsonl" pack/` returns six lines: `pack/AGENTS.md:61`, `:63`, `pack/LEDGER.template.md:3`, `:9`, `pack/prompts/orchestrator.md:19`, and `pack/instrument.md:3`. "The instrumentation section" is `pack/instrument.md`, rendered into the `{{instrument}}` slot at `pack/AGENTS.md:116` only under `--instrument`. Outside it the pack mentions the round log in FIVE places across THREE files, not two across one.

THE ARGUMENT SURVIVES, WHICH IS WHY THIS IS `low`. I read all three uncited sites and every one gates itself in the same "When instrumentation is on" form the sentence describes. Defect D's conclusion is therefore exactly as stated: `pack/AGENTS.md:93` is the SOLE unconditional promise, and every other mention is conditional. Only the count is wrong, and the true count strengthens the argument.

I TESTED THE AVAILABLE DISMISSAL AND IT FAILS. The defence would be that "the pack" means `pack/AGENTS.md` in context. It cannot: the same sentence then moves to `pack/instrument.md` as another part of the same pack, and "outside the instrumentation section" is a qualifier that only means anything directory-wide. The whole-directory reading is the only coherent one.

WHY BOTH EARLIER ROUNDS PASSED IT, VERIFIED. Round 1's triage recorded "`docs/metrics/workflow.jsonl` appears exactly twice in `pack/AGENTS.md`, at `:61` and `:63`" (`workflow-enforcement-tier-planreview-r1-triage.md:49`), which is a NARROWER claim than the sentence makes and is true. The sentence was checked against a paraphrase of itself.

ONE FACT I ADD, WHICH THE REVIEWER DID NOT HAVE. The two missed files ARE deployed to the non-instrumented population defect D is about: my fixture rebuild wrote `.agents/LEDGER.template.md` and `.agents/prompts/orchestrator.md` among 30 files with no `--instrument`. So the miscount is about text the affected reader receives. This does not move the severity, because the sentence's conclusion is right and its error is a number, but it removes the "the missed sites do not ship anyway" defence before anyone reaches for it.

WHY `low`. One wrong number in a sentence whose conclusion is correct, in a section that decides no behaviour. Round 1's `F-1` band exactly (four constraint attributes beside a five-item list), which round 1 downgraded to `low` on the reasoning that the argument is unaffected and the cost is a reader who greps and finds a different number.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. SINGLE-SITE, and the form is a NARROWING rather than a re-count, which authors nothing: at `:146`, change "the pack" to "`pack/AGENTS.md`", making the existing citation pair exact and the sentence true. Do NOT take the wider re-count form ("every other mention of the round log in the pack sits inside a 'When instrumentation is on' clause"): it says more, it costs a clause, and it puts a fresh sweep claim into a file whose sweep claims have now produced two findings.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS:

- `only two places`: 1 site, `:146`.
- `two places` (the wider phrase): 1 site, `:146`.
- `AGENTS.md:61`: 2 sites, `:146` and `:363`. `:363` says "the existing 'When instrumentation is on' clauses at `pack/AGENTS.md:61` and `:63` are the established phrasing for this conditional and the qualifier should match them". That is a DIFFERENT claim (those two are the established phrasing), it states no count, it is TRUE, and it MUST NOT CHANGE. This is the twin the narrowing must not disturb, and the narrowing does not reach it.
- No count of pack mentions is spelled as a word anywhere else in the fold; `grep -rn "two mentions"` and `grep -rn "twice"` over the fold return nothing bearing on this.

ONE SITE.

---

## `R3B-5`. `VALID`. Severity `low` (unchanged). Check 15's path clause is satisfied by the pre-fix build, so inc3's new problem message is pinned by nothing and the literal minimal implementation ships a self-contradicting one

TENSE APPLIED: BOTH, AND THE FINDING IS THAT THE CLAUSE IS TRUE OF TODAY'S TREE, WHICH IS PRECISELY WHY IT CANNOT DETECT THE CHANGE. This is the mirror of `R3A-1`'s tense (there the weak half is the BEFORE state; here it is an AFTER clause), and I say so because the brief asks the two to be distinguished.

QUOTE VERIFIED at `workflow-enforcement-tier.md:329`, word for word.

MEASURED AGAINST THE PRE-FIX BUILD, and it reproduces exactly: from inside a fresh fixture, `validate --source docs/plans/TEMPLATE.plan.toml --workflow` prints `no metrics log at docs/metrics/workflow.jsonl; nothing to validate` and the skip note on stderr, the ok line on stdout, exit 0. `src/main.rs:845` is `eprintln!("no metrics log at {}; nothing to validate", metrics_path.display());`, verified by opening it, and it fires on the missing-log path independently of `--workflow`. So "reports the missing log BY PATH" IS ALREADY TRUE of the pre-fix build; only "exits NON-ZERO" discriminates, and `:51` says as much in the file's own words ("The operative defect is therefore the EXIT CODE and nothing else").

THE BAD IMPLEMENTATION IS THE LITERAL ONE, AND I READ THE ARM RATHER THAN TAKING THE QUOTE. `src/main.rs:999-1003` is a two-line comment plus `_ => eprintln!("--workflow has a plan source but the metrics log is missing; skipping the workflow check"),` at `:1001-1003`. `:276` instructs that this "becomes a reported problem". The smallest edit satisfying that is `eprintln!` -> `problems.push(...)` with the string unchanged, which ships a run that EXITS 1 while its own problem line says "skipping the workflow check", names no path of its own, and passes checks 15, 16, 17 and 18 and `cargo test`.

THE UNPINNED REQUIREMENT IS STATED TWICE ELSEWHERE, both verified: `:55` ("the run exits non-zero and reports why") and `:256` ("After inc3: a HARD FAILURE naming the path it looked for"). Because `src/main.rs:845` supplies the path unconditionally, an implementation that satisfies neither passes everything runnable. That is the `INC2-1` residual shape ("THE REQUIREMENT IS PINNED BY NO CHECK, AND THIS IS THE OPERATIVE HALF"), against a file whose own standard at `:304` is that a round is settled by running the checks.

WHY `low` AND NOT `medium`, AND I WEIGHED THE UPGRADE AGAINST ROUND 2's `INC2-3`. `INC2-3` was rated `medium` because the shipped tool would report a WRONG CAUSE on the machine surface an agent acts on, which is a wrong-answer defect. Here the exit code is correct, the enforcement is correct, and defect A really is closed; what ships is a human-readable message that contradicts its own exit code. That is a message-quality defect, one band below a wrong answer. It stays above zero because it is a stated requirement with no enforcement and the failing implementation is the LITERAL reading of `:276`, not a careless one. Round 2 rated `INC2-1` `low` for the identical structure.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. SINGLE-SITE, and it is a SUBSTITUTION INSIDE AN EXISTING CHECK rather than new narrative, which is the shape round 2's triage preferred for the identical problem. At `:329`, replace "reports the missing log BY PATH" with a clause requiring THE REPORTED PROBLEM (not merely the run's stderr) to name the resolved path and to state that the check could not run. That converts the stale "skipping the workflow check" wording from a passing outcome into a failing one, and commits the plan to no mechanism.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS:

- `BY PATH`: 1 site, `:329`.
- `naming the path it looked for`, THE SEMANTIC TWIN A LITERAL SWEEP OF `BY PATH` WOULD MISS: 2 sites, `:256` and `:332` (check 18). Both assert the same weakly-discriminating property, and I considered whether the fix must reach them. IT MUST NOT. Both are the accepted-cost-(i) claim, whose point is WHICH path the failure names (the wrong-but-inside-root one under `docs/plans/docs/metrics`), and that is discriminating for its own purpose. More decisively, once `:329` requires the reported problem to explain itself, the implementation that ships "skipping the workflow check" fails check 15 and never reaches check 18 as a passing build, so editing `:332` closes nothing that `:329` has not already closed and would author prose on a check that is correct for its own job.
- `skipping the workflow check`, the string the bad implementation would keep: 0 sites in the fold (it appears only in `src/main.rs`), so there is no artifact site quoting it that would fall out of step.

ONE SITE. I swept two more and am deliberately excluding them; that exclusion is a ruling, not an oversight.

---

## `R3B-6`. `VALID`. Severity `low` (unchanged). Inc3's risk argument states a denominator this fold's own review loop has already moved

TENSE APPLIED: PRESENT, since the sentence describes the project's calibration data as it stands.

QUOTE VERIFIED at `workflow-enforcement-tier.md:300`, word for word.

WHAT REPRODUCES EXACTLY, and I checked every number in the sentence rather than only the one under suspicion. Step order 92 is `prompt-drift-guard`. Over `docs/metrics/workflow.jsonl` at `5169ea0`, its round records are six, with `valid_findings` 4 + 3 + 5 + 1 + 2 + 0 = FIFTEEN. "six rounds and fifteen findings" is exact. "all of them prose, zero mechanism defects" matches the ledger's own diagnosis at `:387`. "a project median of two rounds" is still true (see below). "joint-third" is still true (see below).

WHAT DOES NOT. `jq -r 'select(.type=="round") | .task' docs/metrics/workflow.jsonl | sort -u | wc -l` returns EIGHTY, not seventy-seven. I pinned the drift to a commit rather than asserting it: at `110a8f8` the same query returns 77, and the three tasks added since are `checks-runner-worktree-name-collision-inc1`, `decision-folder-currency-inc1` and `workflow-enforcement-tier-fold`. The last is THIS FOLD'S OWN REVIEW LOOP, so the artifact's own reviewing is one of the three things that falsified its number.

THE ARGUMENT IS UNAFFECTED AND I CONFIRMED BOTH LOAD-BEARING HALVES INDEPENDENTLY RATHER THAN TAKING THE REVIEWER'S. The current distribution is 1 x16, 2 x37, 3 x10, 4 x6, 5 x7, 6 x2, 7 x1, 9 x1, summing to 80. Positions 40 and 41 of 80 both fall in the 2-round bucket, so THE MEDIAN IS STILL TWO. The ordered top is 9, 7, 6, 6, so six is STILL JOINT-THIRD. Only the denominator moved, which is why the deletion form loses nothing.

WHY `low`. `F-1` and `EX-8`'s band: a reader who re-derives the figure finds a different one, which costs seconds, and the risk classification does not move. It stays above zero because the sentence carries no hedge of the kind `:72` gives the record count, and because this project has twice ruled the identical shape valid.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. SINGLE-SITE AND DELETION-CLASS. At `:300`, strike "of seventy-seven" so it reads "joint-third of the artifacts ever reviewed against a project median of two rounds". That stays true as the log grows, loses nothing the argument uses, and authors no word. DO NOT substitute "eighty": a running total substituted today is wrong again next round, which is how this finding arose.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS:

- `seventy-seven`: 1 site, `:300`.
- `77` as a numeral, in case the same count is spelled differently: 2 sites, neither related. `test-tmpdir-repo-assumption.md:13` is inside the citation `src/checks.rs:1477-1486`; `docs/plans/agent-scaffold.plan.toml:1044` is `order = 77`. Neither is a twin.
- `eighty`: 0 sites.
- `joint-third`: 1 site, `:300`. `median of two`: 1 site, `:300`. Both are in the sentence being edited, both stay TRUE after the deletion, and neither needs a change. This is the check the round-2 lesson demands: the numeral I am removing does not head an enumeration and the two claims that share its sentence survive it.
- OUT OF SCOPE AND NOT A SITE: `docs/plans/agent-scaffold.ledger.md:387` carries "Across all 77 artifacts ever reviewed in this log the distribution is ...". The ledger is not this review's artifact, and that sentence sits inside a dated round-record narrative, which is the historical framing round 1's `EX-8` triage correctly protected. Do not touch it.

ONE SITE.

---

## `R3B-7`. `VALID`. Severity `low` (unchanged). Inc1's risk paragraph labels a measurement that includes candidate (b) as inc1's own scope

TENSE APPLIED: PRESENT, since this is a fidelity claim about what a committed exploration record measured.

QUOTE VERIFIED at `workflow-enforcement-tier.md:296` ("A measured the anchor at `+79/-15` and the whole thing including siblings and ledger at `+163/-18`, mostly comments").

THE SOURCE OPENED AND VERIFIED. `docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md:474-476` reads, verbatim and at exactly those line numbers, "Candidate (a) alone, Route B: **+79 / -15** ...", "Candidate (a) via Route A (`value_source`): **+96 / -13** ...", "Full build, (a) + (b) + the sibling and ledger extensions: **+163 / -18**." So `+163/-18` is `(a) + (b) + the sibling and ledger extensions`, and candidate (b) is the containment guard, which `:275` assigns to INC2 and `:274` explicitly excludes from inc1 ("NO new REFUSAL mechanism"). The sidecar's phrase enumerates inc1's parts exactly (anchor, siblings, ledger) while quoting a number that includes a fourth part belonging to a different increment.

THE FILE'S OWN ARITHMETIC AGREES WITH THE SOURCE AND DISAGREES WITH `:296`. `:298` closes inc2's risk classification with "The counter-argument, that it is roughly 80 lines sharing inc1's derivation". 163 - 79 = 84. So the same document uses `+163` as inc1-plus-inc2 in one paragraph and labels it inc1 alone two paragraphs earlier; if both readings were right, inc2 would be zero lines.

WHY `low`, AND IT IS THE SMALLEST OF THE EIGHT. Both statements sit inside counter-arguments their own paragraphs reject, so an inflated inc1 number only strengthens the rejection and changes no conclusion. What keeps it above zero is that an implementer sizing inc1 against `+163/-18` is pointed at a diff containing work `:274` forbids inc1 from doing, on a step whose entire structure is increment separation. `:274`'s scope statement is unambiguous enough that a careful implementer will not act on the number.

MINIMAL FIX AND SITE COUNT, GREPPED OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`. SINGLE-SITE AND DELETION-CLASS. At `:296`, strike ", and the whole thing including siblings and ledger at `+163/-18`" so only inc1's own `+79/-15` is cited. Do NOT take the relabel form ("(a) + (b) + the sibling and ledger extensions"): it keeps a number the paragraph has no use for and authors a clause, while `:298` already carries the inc2 half of the arithmetic in the place it belongs.

WHAT I SWEPT, INCLUDING SEMANTIC TWINS:

- `163/-18`: 1 site, `:296`.
- `79/-15`: 2 sites, `:156` and `:296`. `:156` uses it correctly and for a different purpose (the route comparison, "`value_source` is `+96/-13` against `Option<PathBuf>`'s `+79/-15` for identical behaviour"). It MUST NOT CHANGE, and the deletion at `:296` does not reach it.
- `roughly 80 lines`: 1 site, `:298`. It stands on its own words ("it is roughly 80 lines sharing inc1's derivation") without citing the arithmetic, so deleting `+163/-18` leaves it intact and unmoored from nothing. `roughly ten lines` at `:278`, the ledger's size, is unrelated.
- No numeral I am deleting heads an enumeration and no cross-reference restates the `+163` figure anywhere in the fold.

ONE SITE.

---

## Citations I corrected, and measurements that came out differently

Recorded because this project has repeatedly caught misnumbered citations inside findings files, and because a correction inside a valid finding is a success rather than a demerit.

- `R3A-1` CITES `workflow-enforcement-tier.md:88` FOR TWO FACTS THAT SPAN TWO LINES. `:88` is `#   slug   = "example-step"    -> "triager-runs-only-on-findings"` and `:89` is `#   status = "not-started"     -> "complete"`. The correct citation is `:88-89`. NOT LOAD-BEARING: the finding also cites the scaffold output, which I reproduced directly (slug at `TEMPLATE.plan.toml:34`, status at `:36`).
- `R3A-1`'s `src/workflow.rs:437` DOES NOT SUPPORT THE CLAIM ATTACHED TO IT, and this is the one citation in the round that reproduces to something other than what it is said to show. The finding writes "the refusal is a path-containment predicate, independent of step status (as `src/workflow.rs:437` and the mechanism section make clear)". `src/workflow.rs:437` is `pub(crate) fn w3_problems(`, a function signature about steps and rounds, which says nothing about a containment predicate. No code citation COULD support it: the predicate is inc2's work and does not exist in the tree. The claim is nonetheless TRUE and is established by the sidecar text the finding also cites (`:164`, the refusal's definition as a path predicate, and `:180`, "the SAME containment predicate ... the canonically-derived plan root, and whether the resolved artifact lives under it"). The finding survives with the code citation struck; the conclusion does not depend on it.
- NO MEASUREMENT CAME OUT DIFFERENTLY FROM EITHER REVIEWER'S. I re-ran or re-derived every one: 386, 241, 80 distinct tasks, the 16/37/10/6/7/2/1/1 distribution, the 77 baseline at `110a8f8` and the three tasks added since, the five pack mentions across three files, the five `386` sites, the exploration's `+79/-15` / `+96/-13` / `+163/-18` at `:474-476`, the six `prompt-drift-guard` rounds and fifteen findings, check 11's before-state, check 15's before-state, and `R3A-1`'s empty-log negative. Every one matches the reviewer that reported it.
- ONE FACT I ADDED THAT NEITHER REVIEWER HAD, under `R3B-4`: `.agents/LEDGER.template.md` and `.agents/prompts/orchestrator.md` are both written by a non-instrumented scaffold, so the three uncited pack sites do reach the population defect D is about.
- ONE PIECE OF EVIDENCE I ADDED THAT SETTLES `R3A-1` AGAINST A DISMISSAL, under that entry: explorer A's "false pass 2, an explicit relative `--metrics`" (`metrics-path-anchor-to-source.md:211-220`) was measured on a BORROWED-SLUG fixture, so check 11 names A's false pass and instantiates it without the mutation that makes it one.
- `R3B-1`'s PROPOSED FALLBACK IS FALSE, recorded under that entry: the "narrow true" positive form the reviewer offers as an alternative to deletion is itself falsified by inc2's containment refusal, which is a new mechanism rather than a pre-existing check seeing the right data. The deletion is the only correct form.
- BOTH REVIEWERS' COVERAGE CLAIMS HELD UP WHERE I SAMPLED THEM. The residue lens's ten-of-ten fix verification and its twin sweeps I spot-checked at `$FIXTURE` (0), `target text` (0), `all-steps-complete` (0) / `all-steps-terminal` (1), `Three doc claims` (0) / `Four doc claims` (1), `next.rs:111-112` (2), and its `INC2-6` deviation ruling, which I re-derived and agree with. The inc1/inc3 lens's negative results I spot-checked at the `render --check` result, the clippy result, and the step orders 94 / 95 / 96 at `plan.toml:1300` / `:1324` / `:1337`. Neither lens's stated coverage showed a gap this round, which breaks the two-round run recorded in the round-2 triage.

## Totals

- RAW FINDINGS: 8 (`R3A-1`; `R3B-1` through `R3B-7`).
- AFTER DEDUPLICATION: 8. Nothing merged. Three THEMES span pairs (`R3A-1` with `R3B-5`; `R3B-3` with `R3B-6`; `R3B-4` with `R3B-7`), and in each pair the two findings are distinct sites with distinct fixes and neither subsumes the other. The brief's specific question is answered above: `R3A-1` and `R3B-5` are ONE THEME AND TWO DISTINCT FINDINGS.
- VALID: 8.
- VALID BUT ACCEPT RESIDUAL: 0.
- DISMISSED: 0.
- SEVERITY MIX OF THE VALID SET: 0 critical, 0 high, 0 medium, 8 low. No reviewer rating changed. I considered `medium` for `R3B-1` (round 1's `EX-5` lineage) and for `R3B-5` (a shipped self-contradicting message) and held `low` on both, on the ground that severity is absolute impact if unfixed and that neither lineage nor round number is a merit.
- TOTAL SITES TO EDIT: NINE. `R3A-1` 1, `R3B-1` 1, `R3B-2` 1, `R3B-3` 2 (of 5 measured), `R3B-4` 1, `R3B-5` 1, `R3B-6` 1, `R3B-7` 1. All nine are in `workflow-enforcement-tier.md` except one of `R3B-3`'s, which is `test-tmpdir-repo-assumption.md:66`. No site is in `status-resume-ignores-json.md` or in `docs/plans/agent-scaffold.plan.toml`, and I swept both for every string above.

## Guidance for the fix pass

- FIX SET, IN DESCENDING ORDER OF WHAT IT BUYS: `R3B-1` (the only finding that could mislead a reviewer about what a correct inc1 looks like, and a deletion), then `R3B-5` and `R3A-1` (the two check-strength findings, which may as well land together), then `R3B-3`, `R3B-2`, `R3B-4`, `R3B-6`, `R3B-7`.
- EIGHT OF THE NINE SITES AUTHOR NO NARRATIVE PROSE, which is the shape this project's calibration data favours: FIVE ARE DELETIONS ACROSS FOUR FINDINGS (`R3B-1`, `R3B-3` at two sites, `R3B-6`, `R3B-7`), ONE IS A NARROWING (`R3B-4`, "the pack" -> "`pack/AGENTS.md`"), and TWO ARE SUBSTITUTIONS OR CLAUSES INSIDE EXISTING CHECKS (`R3A-1` at `:318`, `R3B-5` at `:329`). ONLY `R3B-2` NEEDS NEW TEXT, and it is one bullet of one clause. Keep it to the smallest true statement, and specifically do NOT restate the `Added` / `Changed` / no-`Fixed` structure a third time.
- WHERE A FINDING'S OWN PROPOSED FIX AUTHORS MORE THAN MINE, I HAVE PRESCRIBED THE SMALLER ONE AND SAID WHY, at `R3B-2` (one-clause bullet, not a copy of `:346`'s two-sentence form), `R3B-4` (narrowing, not the wider re-count), `R3B-6` (deletion, not the substitution "eighty"), and `R3B-7` (deletion, not the relabel).
- MY SITE COUNTS ARE MEASUREMENTS AND EACH ENTRY STATES WHAT I SWEPT FOR, including the semantic twins a literal grep cannot match. THREE ENTRIES CARRY A DELIBERATE EXCLUSION rather than an omission, and each says so: `R3B-3` measures five `386` sites and scopes the finding to two; `R3B-5` finds the twin property at `:256` and `:332` and rules them out of the fix; `R3B-4` finds `pack/AGENTS.md:61` cited a second time at `:363` and rules that site correct as written. Read those exclusions as rulings. NO NUMERAL PRESCRIBED FOR EDIT IN THIS ROUND HEADS AN ENUMERATION, which is the round-2 `INC2-6` trap, and I checked each one for it.
- DO NOT REOPEN `INC2-7` OR `F-5`. Both are deliberately-accepted residuals, both were confirmed present and correctly untouched by both reviewers and by me, and neither is restated by any round-3 finding.
- AFTER EDITING ANY SIDECAR, RE-RENDER. `docs/plans/agent-scaffold.md` is a generated projection that must never be hand-edited; `render docs/plans/agent-scaffold.plan.toml --check` reports "up to date" at this commit, so it will catch a missed re-render. No fix in this round edits the plan TOML, so no `Q-55` question body changes, but the sidecar text is carried in the generated view and the re-render is still owed.
