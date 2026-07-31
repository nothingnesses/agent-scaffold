# Calibration analysis: the round constants, from the four exploration datasets

Analyst pass for `workflow-calibration`. Worktree `.claude/worktrees/cal-analyst`, branch `explore/cal-analyst`, at `4e05404`. I did none of the gathering. I read the four explorer records, re-derived every number I use from the primary sources, and report what the data supports.

Nothing here changes product code, the plan, the ledger or the metrics log. Two of the five recommendations below are "keep the current default", argued rather than defaulted to.

## METHOD

Scripts are in the session scratch directory `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/analyst/`: `oracle.py` (totals, changed/unchanged rates, the enforced-second-round measurement), `oracle2.py` (censoring bookkeeping, cap grid, timestamp stratum, escalation records), `q5_agree.py` and `q5_agree2.py` (inter-rater agreement, exact then fuzzy id matching), `q6_shortfall.py` and `q6b.py` (shortfall, reviewer-array check, the 411-versus-419 check), `q3_replay.py` and `q3b.py` (policy replay), `markov.py` (the sequential framing). Every table below names the script that produced it.

Sources, in the order I trust them:

1. `docs/metrics/workflow.jsonl` is the oracle for anything round-level. I re-derived it rather than quoting anyone.
2. The triage and reviewer files themselves, retrieved from git for the specific claims I adjudicate (Q6).
3. The two mechanical datasets `fix-pass-shape-{a,b}.tsv` and the two interpretive datasets `finding-provenance-{a,b}.tsv`.
4. The explorers' prose records, used for their LIMITS sections and for claims I then checked.

Three disciplines I applied throughout.

**Every claim I repeat, I re-derived.** This found four errors that would otherwise have travelled, listed under LIMITS. The most consequential is that interpretive extractor A's METHOD section 5 states the corpus holds "411 valid findings" over all 204 rounds. The true total is 419. 411 is the sum over `new_valid` rounds only. A's own reconciliation is per-round and is unaffected, but the stated corpus total is wrong and is one digit away from being quoted as the denominator of a rate.

**Censoring is stated per question, not once at the end.** The convergence rule stops the loop at the streak, so the quantity "what would one more round have found" is unobserved by construction wherever the rule fired. I report, for each question, how much of the sample is censored and in which direction the censoring biases the answer.

**Correlated raters.** Every reviewer, triager and extractor in this corpus is a Claude-family model. Interpretive extractor B states `claude-sonnet-5`. Interpretive extractor A does not state its model anywhere in its record; the cross-model claim for the interpretive pass rests on the orchestrator's ledger assertion, not on A's own record. Same-family agreement overstates reliability even when the models differ, so every agreement figure below is an upper bound on what a cross-harness or human check would give.

### Oracle re-derivation (`oracle.py`)

| Quantity | My value | Ledger / explorer value | Match |
| --- | --- | --- | --- |
| Records | 231 (204 round, 22 decision, 5 escalation) | 231 / 204 | Yes |
| `sum(valid_findings)` | 419 | 419 (mech A, mech B) | Yes |
| `sum(len(severities))` | 419, 0 per-round mismatches | 419 | Yes |
| Severities | 1 critical, 12 high, 85 medium, 321 low | same | Yes |
| Outcomes | `clean` 102, `new_valid` 102 | same | Yes |
| Changed / unchanged | 183 / 21 | same | Yes |
| changed -> `new_valid` | 99/183 = 54.1%, Wilson95 [46.9, 61.2] | same | Yes |
| unchanged -> `new_valid` | 3/21 = 14.3%, Wilson95 [5.0, 34.6] | same | Yes |
| Artifacts (task grouping) | 79; 57 `low_risk`, 22 `risky` under any-round-risky | same | Yes |
| `low_risk` rounds | median 2, mean excess 0.98, max 5 | same | Yes |
| `risky` rounds | median 4, mean excess 2.14, max 9 | same | Yes |
| `risky` past cap 5 | 4/22 = 18.2%, Wilson95 [7.3, 38.5] | 4 of 22, 18 percent | Yes |
| Medium-or-worse, any-round-risky rule | `low_risk` 55/250 = 22.0%, `risky` 43/169 = 25.4% | 22 / 25 percent | Yes |
| Medium-or-worse, per-round rule | `low_risk` 61/265 = 23.0%, `risky` 37/154 = 24.0% | 23.0 / 24.0 | Yes |
| Episodes under mech-A's split rule | 89 from 79 tasks | 89 | Yes |
| Mech A's size association | median 761.5 vs 88, U = 143.5, AUC = 0.788, z = 2.548, p = 0.0108 | 761.5 / 88 / 143.5 / 0.788 / 2.55 / ~0.011 | Yes, to the digit |
| Corpus derivation | 296 deleted + 49 live -> 343 union, 89 name-matching triage, 18 overlap | 343 / 89 / 18 | Yes |

Everything the two mechanical explorers published reconciles exactly against my own re-derivation. The 250/169-versus-265/154 split the orchestrator already resolved is confirmed: both rules reproduce, they differ only in artifact-level versus per-round attribution, and the substantive conclusion holds under both.

---

## Q1. Is ONE verification round sufficient after a fix pass, or are TWO required?

**Answer: the data discriminates, and it says one verification round is sufficient at the severity level that matters, with a residual-risk ceiling of 14.9 percent that the decision-maker must be willing to accept. It does NOT say the second round is worthless; it says the second round has never in this corpus found anything above `low`.**

### The measurement, and why it is the right one (`oracle.py`, `oracle2.py`)

The question "does a second clean round pay for itself" is usually censored, because a loop that converges at the streak never runs another round. It is NOT censored for `risky` artifacts, because the bar of 2 forces the second round to run. Every such forced round is an observation of exactly the quantity in dispute.

I extracted every within-episode transition where round K was `clean` with `consecutive_clean == 1` and a round K+1 ran. That is precisely the situation where a bar of 1 would have converged the artifact and a bar of 2 compels one more round.

There are 22 such enforced second rounds. All 22 are on `risky` artifacts (there are zero on `low_risk`, which is the censoring, quantified below).

| Measurement | Value | Wilson 95% |
| --- | --- | --- |
| Enforced second round returns `new_valid` | 3/22 = 13.6% | [4.7, 33.3] |
| ... restricted to an unchanged artifact (pure re-review) | 3/20 = 15.0% | [5.2, 36.0] |
| **Enforced second round finds anything medium-or-worse** | **0/22 = 0.0%** | **[0.0, 14.9]** |

The complete severity inventory of everything all 22 enforced second rounds ever found is `{low: 3}`. Three rounds each found exactly one `low` finding. Nineteen found nothing.

The three are:

| Round index | Task | What it found |
| --- | --- | --- |
| 92 | `waiver-model` | 1 low, artifact unchanged |
| 113 | `structured-skeleton` | 1 low, artifact unchanged ("Inc 6 holistic final") |
| 142 | `driver-output-generation` | 1 low, artifact unchanged; the log's own artifact string says "round 3 fresh review found incidental next.rs reflows" |

**An identity nobody has stated, and it matters.** These three rounds ARE the "3 of 21 unchanged rounds returned `new_valid`" that the ledger uses to argue one clean round is right 86 percent of the time. The 3/21 figure and the 3/22 enforced-second-round figure are the same three events. The corpus therefore contains ONE measurement of the value of the second clean round, not two independent ones, and its entire content is three `low` findings, one of which the log itself describes as incidental formatter reflow.

### Step 93 specifically

Step 93's fix set is three changes, two pure deletions and one narrowing clause, 23 insertions and 12 deletions in `src/checks.rs`, zero production lines.

Three things the data says about it.

**(a) The corpus has no "deletion-class fix pass" to attach a rate to.** Mechanical A located 32 fix passes; zero are line-deletion-only, and among the 27 that changed anything zero have `new_tokens == 0`. The smallest pass in the corpus authored one new word. Mechanical B resolved 18 passes, all `MIXED`. So `P(next round new_valid | deletion-only)` is undefined at any sample size in this corpus, and it is undefined because the category does not exist at the granularity of a PASS. Step 93's "two of three pure deletions" is a per-FIX count with no pass-level equivalent, which is exactly the unit mismatch mechanical A flagged as E1.

**(b) What does exist is a SIZE gradient, and step 93's fix set sits in its safest tercile.** Re-derived from `fix-pass-shape-a.tsv`, n = 27 located changed passes:

| `new_tokens` tercile | Next round `new_valid` | Wilson 95% |
| --- | --- | --- |
| T1 (1 to 93) | 2/9 = 22.2% | [6.3, 54.7] |
| T2 (95 to 708) | 5/9 = 55.6% | [26.7, 81.1] |
| T3 (815 to 12298) | 7/9 = 77.8% | [45.3, 93.7] |

Rank test over all 27: AUC 0.788, two-sided p = 0.0108. **This does not survive Bonferroni at mechanical A's five pre-registered tests (threshold 0.01), and it is one of my six questions plus sub-analyses, so under any honest correction it is suggestive only.** It is also consistent with both the injection and the new-content hypotheses, so it does not discriminate mechanism. A 35-line single-file change lands in T1, where the observed rate is 22.2 percent, but with an interval running to 54.7 percent that is barely narrower than the corpus base rate of 54.1 percent.

**(c) The evidence that cuts AGAINST step 93 is on step 93's own artifact.** The interpretive extractions record four fix-injections inside this very loop, extracted independently by both extractors from different files:

| File | Finding | Severity | Class | Blames |
| --- | --- | --- | --- | --- |
| `-triage-r3.md` | `AD2` | low | test | round 2's fix |
| `-triage-r3.md` | `AD3` | low | prose | round 2's fix |
| `-triage-r4.md` | `RG1/MU1` | **medium** | test | round 3's `AD1a` fix |
| `-triage-r4.md` | `RG2` | low | prose | round 2's fix |

So the artifact under review has already had a fix pass inject a MEDIUM test defect that the next round caught. That is the single strongest argument for a second verification round here, and it comes from the artifact itself rather than from a corpus rate.

**(d) The discriminating detail.** Across both extractions there are 22 rows a triager explicitly ruled `introduced_by_prior_fix` (union, overlap de-duplicated, `union.py`). Twenty of the 22 are `class = prose`: 20/22 = 90.9%, Wilson95 [72.2, 97.5]. Both extractors found this independently (13 of 14 each) and I reproduce it on the union. Every injection in the corpus is attributed to a pass that AUTHORED something, and nine times in ten what it authored was prose. Step 93's fix set is two pure deletions plus ONE narrowing clause, and that clause is prose. **The one non-deletion element in step 93's fix set is exactly the shape that has produced 20 of the 22 recorded injections.**

### What I conclude for Q1

The decision is between (i) apply, one verification round, merge with a waiver, and (ii) apply, reset counters, earn two clean rounds. Note first that option (ii) as usually stated does not work: the ledger's own escalation record spells out that resetting plus one authorised round ends at streak 1 of 2 and converges nothing, so (ii) costs at least two rounds and, at the observed `P(clean | previous round new_valid) = 68.8%` for the first round after a fix, more often three.

The data supports option (i), with these numbers attached:

- P(the verification round itself comes back `new_valid`) is about 22 percent [6.3, 54.7] on the size gradient, against a corpus base rate of 54.1 percent for changed artifacts. If it does come back `new_valid`, the choice re-presents itself and nothing is lost.
- Conditional on that round coming back clean, P(a second round would have found anything) = 3/22 = 13.6% [4.7, 33.3], and P(it would have found anything medium-or-worse) = 0/22 = 0.0% [0.0, 14.9].
- The residual risk being accepted is therefore bounded at 14.9 percent for a `low` finding escaping and 14.9 percent for a medium-or-worse one, with point estimates 13.6 percent and 0 percent respectively.
- The specific counter-evidence is that this artifact's own loop injected a medium at round 4. That risk is concentrated in the authoring part of the fix set, and the fix set is two-thirds deletion.

**A concrete way to spend the difference.** The evidence separates cleanly on fix shape, not on round count. Applying the two pure deletions and the narrowing clause as separate, individually verified changes is not available at this point in the loop, but constraining the verification round's brief to sweep the region the narrowing clause touches, rather than the whole file, targets the round at the 20-of-22 risk. That is a lens-scope instruction, and the ledger already records the round-3 lesson that a lens should be scoped to a REGION rather than to a list of sentences.

**Confidence: MEDIUM.** The severity bound (0/22, upper 14.9 percent) is the load-bearing number and it is a bound, not an estimate. It is not a p-value and so survives multiple-comparison correction trivially, but it rests on 22 observations of which 20 are re-reviews of an unchanged artifact, and it is a DETECTION rate rather than a defect-existence rate. What would raise my confidence: 20 more zero-event enforced rounds would take the upper bound to 8.4 percent. What would lower it: any single medium-or-worse finding on an enforced second round, which would move the point estimate from 0/22 to 1/23 = 4.3% [0.8, 21.0] and remove the clean severity separation entirely.

---

## Q2. Is the `risky` = 2-consecutive-clean bar justified, too strict, or too lenient?

**Answer: it is measurably expensive and its measured yield is three `low` findings. That is a case for lowering it to 1, and the case does not close, because the population that would tell us what a bar of 1 lets through is 100 percent censored.**

### The cost, exactly (`q3_replay.py`)

Replaying each `risky` artifact's observed outcome sequence under each bar, capped at 5:

| Bar | Mean rounds per `risky` artifact | Converged within cap 5 |
| --- | --- | --- |
| 1 | 2.50 (55 rounds over 22 artifacts) | 20/22 |
| 2 | 3.59 (79 rounds over 22 artifacts) | 17/22 |

The second clean round costs **1.09 extra rounds per `risky` artifact**, 24 extra rounds across the 22 artifacts in the corpus. Its measured yield across those 24 rounds is 3 valid findings, all `low`. That is **8 rounds per additional finding, and zero rounds per additional medium-or-worse finding.**

For context, the corpus base rate is 21.6% [17.5, 26.4] of counted findings being medium-or-worse (72/333 over the union extraction). The marginal round found 0 of 3. With n = 3 that comparison is not significant against chance and I am not claiming the marginal round finds systematically lighter material; I am reporting that it has never found heavy material.

### The censoring, exactly (`oracle2.py`)

- 57 `low_risk` episodes converged at streak 1 and never ran a second round. That is 57 fully censored observations of "what would a second round have found", and it is why all 22 enforced second rounds are `risky`.
- 4 `risky` episodes ended `clean` but SHORT of their bar, by human acceptance: `optional-modules-inc2cii`, `waiver-model`, `driver-output-generation`, `prompt-drift-guard-inc1`. The project has therefore already made step 93's choice four times.
- 9 episodes ended on a `new_valid` round; 6 of those are single-round untimestamped backfill artifacts whose loops are simply not fully logged.

The censoring bites in a specific direction that argues AGAINST lowering the bar. The 22 observations come from artifacts that had ALREADY been through the extra fix passes a bar of 2 compels. Under a bar of 1 those loops would have ended earlier, on artifacts that had seen fewer fix passes. The two populations are not exchangeable, and the 0/22 does not transfer to them without an assumption I cannot check.

### The independence question the ledger flagged (`markov.py`)

The ledger wrote that "two consecutive gets to about 98 percent IF THE ROUNDS ARE INDEPENDENT, which is the assumption most at risk". I tested it. Observed within-episode transitions:

| Transition | Count |
| --- | --- |
| `clean` -> `clean` | 19 |
| `clean` -> `new_valid` | 3 |
| `new_valid` -> `clean` | 64 |
| `new_valid` -> `new_valid` | 29 |

| Conditional | Value | Wilson 95% |
| --- | --- | --- |
| P(clean given previous round clean) | 19/22 = 86.4% | [66.7, 95.3] |
| P(clean given previous round `new_valid`) | 64/93 = 68.8% | [58.8, 77.3] |
| P(round 1 clean) | 19/89 = 21.3% | [14.1, 31.0] |

Rounds are NOT independent, and the dependence runs in the direction that FAVOURS the shorter bar: cleanliness persists. But for the specific quantity the ledger was worried about the assumption turns out to be nearly harmless: the unconditional unchanged rate is 14.3 percent and the conditional-on-a-previous-clean-round rate is 15.0 percent. The independence assumption was not the flaw.

The real flaw in the 98 percent is different and worth recording, because it points the wrong way. 14.3 percent is a DETECTION rate, not a defect-existence rate. It is the probability a re-review RAISES something, which is bounded above by the probability that a defect exists AND is where the reviewer looked AND survived triage. The corpus shows plenty of material found late (54.1 percent of changed-artifact rounds find something), so the probability that a converged artifact still carries an undetected defect is HIGHER than 14.3 percent, not lower. "Two consecutive gets to 98 percent" overstates the assurance on both counts.

### Verdict

The bar of 2 is **not too lenient**: nothing in the data suggests it lets material through.

It is **expensive by construction, which is arithmetic and not miscalibration**, and I confirm the ledger's phrasing there.

Whether it is **too strict** is the open question, and the honest answer is that the point estimate says yes and the interval says not yet. Twenty-four rounds bought three `low` findings and zero medium-or-worse, upper bound 14.9 percent. I would want that upper bound below 10 percent, which needs 35 zero-event observations, before changing a safety constant on `risky` artifacts.

**Confidence: MEDIUM.** See RECOMMENDED CONSTANTS for what changes it.

---

## Q3. Is the cap of 5 consistent with the bar?

**Answer: the cap is not the binding constraint, the bar is; the escalation it produces is doing useful work rather than causing friction; and the two loops we can actually observe running past the cap would NOT have converged at a cap of 6. I do not support raising it. This contradicts the conclusion currently in the ledger, and the contradiction is the finding.**

### The escalation rate, three ways (`q3_replay.py`, `q3b.py`)

Replaying every artifact's observed outcome sequence under the actual constants (`low_risk` bar 1, `risky` bar 2, cap 5), an artifact ESCALATES if it exhausts the cap without reaching its bar:

| Population | Escalations | Rate | Wilson 95% |
| --- | --- | --- | --- |
| All 79 artifacts | 12 | 15.2% | [8.9, 24.7] |
| `risky` only (22) | 5 | 22.7% | [10.1, 43.4] |
| Timestamped artifacts only (56) | 6 | 10.7% | [5.0, 21.5] |
| `risky` and timestamped (20) | 5 | 25.0% | [11.2, 46.9] |

The stratum control matters: 6 of the 12 are single-round untimestamped backfill artifacts (`workflow-hardening`, `convergence-accounting`, `plan-maintenance`, `pack-rebuild-tracking`, `consolidate-plan`, `user-prompts-dir`) whose loops predate per-round logging and are not real escalations. The honest figure is the `risky` one: **22.7% [10.1, 43.4]**, or 25.0% [11.2, 46.9] restricted to the timestamped era. The ledger's "4 of 22 ran past the cap" (18.2%) counts artifacts with more than 5 logged rounds; my 5 of 22 counts artifacts that failed to converge within 5, which is one more (`checks-runner-worktree-name-collision-inc1`, currently open at 4 rounds and streak 0). Both are correct measures of different things.

### The five `risky` escalators, and what a cap of 6 would have done

| Artifact | First 5 outcomes | Streak at cap | Round 6 | Converges at cap 6? |
| --- | --- | --- | --- | --- |
| `optional-modules-inc2cii` | N N N N c | 1 | not logged | unknown (censored) |
| `waiver-model` | N N c N c | 1 | not logged | unknown (censored) |
| `structured-skeleton` | N c N N c | 1 | **`new_valid`** | **No, observed** |
| `prompt-drift-guard-inc1` | N N N N N | 0 | **`clean`** (streak 1) | **No, observed** |
| `checks-runner-...-inc1` | N N N N | 0 | not logged | unknown (open loop) |

**Both loops that actually ran a sixth round still failed to converge at a cap of 6.** `structured-skeleton` reached streak 1 at round 5 and its round 6 returned `new_valid`, resetting it. `prompt-drift-guard-inc1` reached streak 1 at round 6, still one short of its bar of 2, and was waived. So on the two observations available, raising the cap to 6 buys two more agent runs and zero convergences. Three of the five are censored: their loops stopped BECAUSE the cap bit, so what round 6 would have found is unobserved.

That is 2 of 2 observed going one way, which is nearly no information as a rate, but it is directly contrary to the projection you get from the corpus-wide conditional. For completeness, the projection: with `P(clean | previous round clean) = 86.4%`, the three streak-1 escalators would each converge with that probability at cap 6, giving an expected 2.41 of 22 escalations = 11.0% (range 9.7 to 13.6 across the interval on 86.4 percent). The projection says a cap of 6 roughly halves escalation. The two observations say it changes nothing. **These are in direct conflict and I am not going to resolve them on n = 2; I report both and recommend against acting on either.**

### The sequential framing, which is the right model (`markov.py`)

Fitting a two-state chain to the observed transitions at the `risky` artifact level (P(round 1 clean) = 3/22 = 13.6%, P(clean given previous clean) = 20/27 = 74.1%, P(clean given previous `new_valid`) = 24/42 = 57.1%):

| Bar | cap 3 | cap 4 | cap 5 | cap 6 | cap 7 | cap 9 |
| --- | --- | --- | --- | --- | --- | --- |
| 2 | 53.3% | 36.2% | **23.4%** | 15.4% | 10.1% | 4.3% |
| 1 | 15.9% | 6.8% | **2.9%** | 1.2% | 0.5% | 0.1% |

The bar-2 cap-5 cell predicts 23.4 percent against an observed 22.7 percent, so the model is well calibrated on the cell we can check. The table's message is unambiguous: **the cap is only "too low" conditional on holding the bar at 2.** Moving the bar from 2 to 1 takes predicted escalation from 23.4 percent to 2.9 percent at the SAME cap of 5, an eightfold reduction, while moving the cap from 5 to 9 at bar 2 gets to 4.3 percent and costs up to four extra rounds on every long loop. The cap and the bar cannot be calibrated separately and calibrating the cap is the expensive way to fix the same number.

Expected rounds to converge under the fitted chain with no cap: bar 1 -> 2.14 rounds, bar 2 -> 3.53, bar 3 -> 5.14. A cap of 5 sits just above the bar-2 mean, which is why it bites as often as it does.

### The reframe the ledger missed: escalation is working

All five `escalation` records in the log carry `human_decision: "decision"`. **Not one is a "resume".** The step's own brief names this as a data question ("whether the human made a real decision or just resumed (useful versus friction)") and the answer is 5 of 5 useful, 0 of 5 friction.

Three of the five escalations produced a substantive decision that changed the workflow itself: `driver-output-generation-inc2`'s record says the round-3 streak reset was "a cosmetic formatter-reflow finding, a class the human simultaneously decided (Q-57)"; `prompt-drift-guard-inc1`'s says the sole open finding was "a low doc-only false coverage claim". Those are exactly the calls a machine should not be making.

So an 18-to-25 percent escalation rate on `risky` artifacts is not obviously a defect. It is the rate at which a hard-coded constant hands a genuinely ambiguous case to a human, and every time it has done so the human has made a real call. Raising the cap would reduce that rate by spending more agent rounds to avoid asking. **The ledger's conclusion that "the CAP OF 5 IS INCONSISTENT WITH IT because it sits below where the distribution still has real mass" reads the escalation as a cost. The escalation records say it is the product.**

### Verdict

The cap of 5 is consistent with the bar of 2 in the only sense that matters: it produces an escalation rate of about 23 percent on `risky` artifacts, every one of those escalations has produced a real human decision, and the two observable attempts to run past it did not converge. The inconsistency the ledger identified is real arithmetic (a bar of 2 needs 3.53 rounds in expectation against a cap of 5, so the tail crosses the cap often), but the remedy it implies is the wrong one.

**Confidence: MEDIUM on "do not raise the cap", HIGH on "the bar is the binding constraint, not the cap".** The second is a structural result from the fitted chain and does not depend on the two contested observations.

---

## Q4. Can INJECTION versus NEW CONTENT be answered from this corpus?

**Answer: No. Not partially, not with a wide interval. The denominator is unobserved and the observed numerator is a sample of triager writing habits rather than of events. Both extractors said this unprompted and both are right.**

### Why (`union.py`)

Union of the two extractions with the 18-file overlap de-duplicated (A authoritative on the overlap; the sensitivity check with B authoritative moves the headline by 0.23 percentage points), 372 rows over 82 contributing triage files:

| Provenance | Rows | Share | Wilson 95% |
| --- | --- | --- | --- |
| `unstated` | 321 | 86.3% | [82.4, 89.4] |
| `pre_existing` | 29 | 7.8% | [5.5, 11.0] |
| `introduced_by_prior_fix` | 22 | 5.9% | [3.9, 8.8] |

Restricted to counted rows (`valid` + `accept_residual`), n = 333: `unstated` 86.5% [82.4, 89.7], `pre_existing` 7.2%, `introduced_by_prior_fix` 6.3%.

Four separate reasons the 5.9 percent is not an injection rate, in descending order of how fatal they are.

**1. The silence is not missing-at-random, and it is measurably concentrated.** Only 28 of the 82 contributing triage files carry a single explicit provenance ruling: 34.1% [24.8, 44.9]. The 51 explicit rows are dominated by a handful of files, with the top eight carrying 26 of the 51. Those files are `decision-folder-currency-plan-r{2,3,4}-triage.md`, `prompt-drift-guard-r{2,3}-triage.md` and `checks-runner-worktree-name-collision-*`, which are precisely the longest, most-scrutinised loops and the ones whose triagers were briefed to check provenance. Where injection was RECORDED is not distinguishable from where injection HAPPENED.

**2. The vocabulary discards the most common explicit ruling.** Extractor A found 14 cases where a triager ruled, in quotable words, that a defect is NEW CONTENT of the artifact under review, neither pre-existing nor attributable to a prior fix, and correctly coded all 14 `unstated` because the schema had nowhere to put them. So the 86.3 percent overstates true silence, AND the new-content hypothesis has been getting explicit triager support all along that the instrument threw away. The forward vocabulary needs four values, not three.

**3. The obvious proxy is broken and must not be rebuilt.** Mechanical A demonstrated that `new_valid` is a poor proxy for injection on the one case where both signals exist: commit `c5b00a7` (the ledger's `3e4fb6c`), the flagship deletion-class pass, was followed by a round recorded `new_valid`, while the ledger's own text records those two findings as pre-existing. The two measurements disagree in sign. I re-derived the pipeline figures for that pair (`subst_added` 2, `subst_deleted` 3, `new_tokens` 1) and they reproduce. No shape-versus-outcome table could ever have measured the injection rate.

**4. Severity does not separate the injected findings from the rest.** Among the 22 `introduced_by_prior_fix` rows: 18 low, 3 medium, 1 high. Medium-or-worse 4/22 = 18.2% [7.3, 38.5], against a base rate over all counted union rows of 72/333 = 21.6% [17.5, 26.4]. The intervals overlap almost entirely. Injected findings are not more or less serious than other findings, so severity cannot be used as a proxy either.

### What the corpus DOES support about injection

Two descriptive facts, both robust because both extractors found them independently on disjoint slices and I reproduce them on the union.

- **Injections are overwhelmingly prose.** 20 of 22 union rows are `class = prose`: 90.9% [72.2, 97.5]. Extractors A and B each independently found 13 of 14. The two non-prose rows are `AD2` (a test assertion the round-2 fix added, step 93) and `RG1/MU1` (a test defect from round 3's fix, step 93). Zero of 22 are production code. This is the first corpus-wide support for a pattern the ledger had recorded as five anecdotes, and it is a claim about the CLASS of recorded injections, not about their rate.
- **Two signals point weakly away from injection being the dominant mechanism.** The `new_valid` rate does not decay with loop length (38/47/43/59/54 percent across 1/2/3/4-5/6+ round artifacts, all 204 rounds, no exclusions), and `prompt-drift-guard-inc1` returned `new_valid` on rounds 4 and 5 after fix passes of 95 and 1 new tokens. Both are mechanical A's, both use no exclusions, and both are weak.

**Confidence: HIGH that the question cannot be answered from this corpus.** This is the one conclusion here I hold without reservation, and it is unanimous across four independent explorers who were not told each other's findings.

---

## Q5. Inter-rater agreement between extractors A and B on the 18-file overlap

**Answer: high on the load-bearing column and good on the rest, but the disagreement is concentrated rather than scattered, and it is same-family agreement so it is an upper bound.**

### Row-set agreement (`q5_agree.py`, `q5_agree2.py`)

A extracted 75 rows on the 18 overlap files, B extracted 74. **17 of the 18 files have identical row counts.** The single exception is `lifecycle-capture-triage.md`, where A extracted 3 rows and B 2.

Matching on exact `(file, finding_id)` gives a misleadingly low Jaccard of 0.795, because the two used different id spellings for the same findings (`I1-2 sub-claim A` versus `I1-2subA`, `O5 (self-reference)` versus `O5a(self-reference)`, `F-DOC` versus `F-DOC(opus4+5)`, `R4-1` versus `R4-1(merges H4-2)`, `O3/S2 (waiver id)` versus `O3(waiver)+S2`, and A's `NOTE-stale-comment` versus B's `NIT-1`). After normalising parentheticals and merge markers:

| Metric | Value |
| --- | --- |
| Matched findings | 72 |
| Only in A | 2 (`lifecycle-capture` "Pre-existing note"; `inc1-triage` `O5 (self-reference)`) |
| Only in B | 2 (`inc1-triage` `O5a`, `O5b`, where B split what A kept as one plus one) |
| **Jaccard on the finding set** | **0.947** |

The only substantive row-set difference is that A extracted a third row from `lifecycle-capture-triage.md` (a "Pre-existing note", coded `dismissed` / `pre_existing`) that B did not extract at all.

### Per-column agreement on the 72 matched findings

| Column | Raw agreement | Wilson 95% | Cohen kappa | Weighted kappa |
| --- | --- | --- | --- | --- |
| `verdict` | 66/72 = 91.7% | [83.0, 96.1] | 0.822 | n/a (nominal) |
| `class` | 66/72 = 91.7% | [83.0, 96.1] | 0.876 | n/a (nominal) |
| **`provenance`** | **70/72 = 97.2%** | **[90.4, 99.2]** | **0.897** | n/a (nominal) |
| `severity_ruled` | 69/72 = 95.8% | [88.5, 98.6] | 0.892 | 0.897 (linear, ordinal) |
| All four columns agree | 58/72 = 80.6% | [70.0, 88.0] | | |

Severity is ordinal, so the linear-weighted kappa is the correct statistic; it is 0.897 against an unweighted 0.892, meaning the few disagreements are not adjacent-category slips but vocabulary differences (below).

### Every disagreement, by finding id

**`verdict`, 6 disagreements, ALL IN ONE FILE.** In `optional-modules-2ci-triage.md`: `F-6`, `F-L2`, `F-L3`, `F-L4`, `F-L5`, `F-M3b`. In all six A coded `accept_residual` and B coded `valid`. This is not scattered rater noise; it is one systematic judgement call about one triage file's disposition vocabulary, applied consistently by each rater. B's own record flags exactly this hazard: it mapped `DEFER`, `ACCEPTABLE`, `VALID BUT OUT OF SCOPE`, `VALID (design)` and `VALID as an observation; NOT a defect` all onto `accept_residual` and warned that "`accept_residual` in this dataset is not a single homogeneous category". The verdict column's 91.7 percent is therefore better read as "one file's coding rule differs" than as an 8 percent per-row error rate.

**`class`, 6 disagreements, scattered.** `NOTE-stale-comment`/`NIT-1` (A `test`, B `prose`), `decision-folder-currency-plan-r4` `H4-3` (A `test`, B `prose`), `state-schema-1` `F-1` (A `code`, B `prose`), `optional-modules-2ci` `F-L2` (A `prose`, B `code`), `inc1-triage` `S3` (A `prose`, B `code`), `inc1-triage` `S4` (A `code`, B `config`). Both extractors independently warned that `class` is their own judgement and should be treated as lower-confidence than the other three columns. The data agrees: it has the lowest raw agreement of the four and its disagreements are genuinely scattered across five files.

**`provenance`, 2 disagreements only.**

| File | Finding | A | B |
| --- | --- | --- | --- |
| `agent-isolation-triage.md` | `R1` | `pre_existing` | `unstated` |
| `decision-folder-currency-plan-r4-triage.md` | `H4-3` | `unstated` | `pre_existing` |

Both are `pre_existing`-versus-`unstated` calls at the threshold of "did the triager state it in words I could quote". Neither is a disagreement about `introduced_by_prior_fix`.

**`severity_ruled`, 3 disagreements, all vocabulary rather than substance.** `agent-isolation` `R1` (A `none`, B `low (downgraded to non-issue)`), `triager-independence-round2` `T1` (A `low`, B `n/a`), `inc1-triage` `O4` (A `low`, B `unstated(deferred to human)`). All three are rows with no severity to carry (dismissed or deferred to a human) where the two raters chose different null tokens. Substantive severity agreement on rows that carry a severity is 72/72.

### The number that matters most

On the load-bearing column, restricted to the rows where either rater gave a positive (non-`unstated`) label:

- Rows where either rater gave a positive label: 10.
- Rows where both gave the SAME positive label: 8.
- **Positive-label agreement: 8/10 = 80.0%, Wilson 95% [49.0, 94.3].**

And on the sub-column that the injection question actually depends on: **both raters independently identified exactly the same six `introduced_by_prior_fix` rows on the overlap** (`decision-folder-currency-plan-r4` `R4-1`, `R4-2`, `R4-3`; `prompt-drift-guard-r2` `V2-1`, `A2-2`; `triager-independence-round2` `T1`). Six of six, no additions, no omissions. That is the strongest single reliability signal in this dataset.

### Correlated raters

B states `claude-sonnet-5`. **A does not state its model anywhere in its record**; the claim that the interpretive pass was cross-model rests on the orchestrator's ledger, which I cannot verify from the artifacts themselves. Even taking the ledger at its word, both raters are Claude-family, so the kappas above are upper bounds on what a cross-harness or human check would yield. The pattern of disagreement supports treating them as correlated: the two raters disagree where the SOURCE is ambiguous (one file's disposition vocabulary, null-severity tokens, prose-versus-code boundaries) and agree essentially perfectly where the source is explicit. That is the signature of two raters sharing a reading strategy, and it means the 97.2 percent on `provenance` reflects that explicit provenance statements are unambiguous, not that these raters would catch each other's blind spots.

**Confidence: HIGH on the agreement figures themselves (they are direct computations on a complete overlap). MEDIUM on what they imply about extraction reliability, for the correlated-rater reason.**

---

## Q6. Which rounds lack a surviving triage file? Adjudicate the disagreement, report the true shortfall and its cause.

**Answer: most of the apparent disagreement is that the two extractors answered different questions. Where they genuinely conflict, EACH has exactly one false positive, and I verified both. A's supporting naming inference is also wrong in a way the ledger's adjudication did not catch, and correcting it changes the answer for step 93 in both directions. The true corpus-wide shortfall is about three times the pre-registered estimate.**

### The disagreement is mostly a population difference

A restricted itself to the 42 tasks its own slice touches and asked which `new_valid` rounds in those tasks have no triage file anywhere in the 343: 10 answers. B asked which TASKS have no triage file at all anywhere: 9 tasks covering 12 rounds, and separately verified its own slice had no internal gaps. These are different populations that barely intersect. The lists are not in conflict except at one point each.

### A's false positive, verified independently (`q6b.py`, direct git retrieval)

A's shortfall table lists round-idx 202 (`checks-runner-worktree-name-collision-inc1` work review R2) as having no triage file. **This is false.** `docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-r2-triage.md` is that round's triage. I did not take the ledger's word for it. I retrieved the file and matched it to the oracle:

- Its header reads "# Triage: `checks-runner-worktree-name-collision` (commit `11d60f3`, round 2)".
- Its stated inputs are `-r2-reviewer-verification.md` ("3 findings, all `low`") and `-r2-reviewer-adversarial.md` ("5 findings, 3 `medium` + 2 `low`").
- Round-idx 202's `reviewers` array is exactly `reviewer-verification` (raw 3, valid 3) and `reviewer-adversarial` (raw 5, valid 5).

Exact match on both reviewer roles and both counts. The orchestrator's conclusion is right.

### B's false positive, which nobody caught

B lists `user-prompts-dir` among the tasks with no triage file anywhere. **This is false.** `docs/plans/agent-scaffold.reviews/user-prompts-dir-triage.md` is in the corpus at index 68 of the 89, and it is in extractor A's own slice (A lists it in its slice table). B also describes the task as having "no `ts` field", which is correct.

So each extractor has exactly one verified false positive, and the ledger's framing (A has a false positive, B is right on this point) is true of the one point it examined but should not be read as B being the reliable list.

### A's naming inference is wrong, and correcting it changes the step-93 answer

A supports its round-202 entry with an inference: that this family uses `-rN-triage.md` for PLAN review round N and `-triage-rN.md` for INC1 WORK review round N, and states it confirmed this by reading `-r2-triage.md`'s header as "# Triage, plan review round 2". **That header does not say that.** I read all five step-93 triage files:

| File | Header says | Belongs to | Verified against |
| --- | --- | --- | --- |
| `-triage.md` | "(commit `b890c4a`, round 1)" | inc1 **work** review R1 (round-idx 201) | "Twelve raw findings, ten after dedup" vs reviewers 5+7 raw, 5+5 valid |
| `-r2-triage.md` | "(commit `11d60f3`, round 2)" | inc1 **work** review R2 (round-idx 202) | reviewer roles and counts, exact |
| `-r3-triage.md` | "# **Plan review round 3**, triage" | **plan** review R3 (round-idx 194) | adjudicates `-r3-reviewer.md` |
| `-triage-r3.md` | "(commit `6a726ed`, round 3)" | inc1 **work** review R3 (round-idx 203) | fixverify ZERO findings + adversarial AD1/AD2/AD3 vs reviewers 0 and 3 raw |
| `-triage-r4.md` | "round 4, commit `3f49012`" | inc1 **work** review R4 (round-idx 204) | commit, round |

So the family uses BOTH slots for BOTH loops: `-r2-triage.md` is a WORK review file while `-r3-triage.md` is a PLAN review file. A's rule is exactly backwards for one of the two files it names.

Three consequences.

1. A's claim that "the plan review has triage files for R2, R3, R4 but not R1" is arithmetically impossible: only five triage files exist in this family and four of them are work-review files. The plan review has exactly one, for R3.
2. **A under-reported.** Step 93's plan review ran four rounds: round-idx 191 (R1, 5 findings including a medium), 193 (R2, 3 low), 194 (R3, 2 low), 195 (R4, clean). R3 has a triage file; R4 was clean so under `Q-63` no triager runs; **R1 and R2 both have none.** A listed R1 and missed R2. A is wrong in both directions on this one family.
3. **The ledger's account of the cause is incomplete.** It says "step 93's four triage files" and diagnoses "the round-2 file puts its round token in a DIFFERENT SLOT". There are five files, not four, and the fifth belongs to a different review loop. The real mechanism is that TWO DISTINCT REVIEW LOOPS on the same slug (the plan review of the step sidecar, and the work review of inc1) share one flat directory with overlapping round numbers, so an ad-hoc per-file disambiguator had to be invented. That is a namespace collision, which is precisely what step 93 is itself about.

### The true shortfall

**A per-round point figure is not derivable, and the reason is the same defect at corpus scale.** The file-to-round join cannot be computed from paths: the increment token lives in the directory for `structured-skeleton.reviews/inc1-triage.md`, in the basename for `optional-modules-2ci-triage.md`, as a bare number for `state-schema-1-triage.md`, and in either slot for the step-93 family. My own token-based join collapses `structured-skeleton-inc1` through `-inc6` onto one task and is unusable. The join exists only in the two extractors' hand-built mappings.

What CAN be derived is a bound, from the accounting (`q6b.py`):

- 89 files match the case-insensitive `triage` filter. Six are reviewer files caught by the substring `triager`; both extractors named the same six independently and I re-derived the list with a `triager-*-reviewer*` pattern. **83 genuine triage files.**
- The two slices cover all 83: A 54 files minus 3 reviewer false positives = 51 genuine; B 53 minus 3 = 50 genuine; overlap 18, all genuine (no false-positive index is divisible by 5). 51 + 50 - 18 = 83. Complete coverage, verified.
- **108 rounds carry at least one valid finding** and so require a triager under `Q-63`. (102 `new_valid` plus 6 recorded `clean` with `valid_findings > 0`. Zero rounds are `new_valid` with `valid_findings == 0`.)
- Triage files documenting a round the log gives ZERO findings: A reports 8 of its 51 = 15.7% [8.1, 28.3]. Scaled to all 83, about 13 files [7, 24].
- So finding-bearing rounds with a surviving triage file: about 70 [59, 76].

**Estimated shortfall: about 38 rounds, range [32, 49], which is roughly 35 percent of all finding-bearing rounds.** The pre-registration expected "roughly 13". That estimate was low by about a factor of three because it compared 89 (which includes 6 reviewer files) against 102 (which omits the 6 clean-with-findings rounds) and ignored that triage files are also written for zero-finding rounds.

**A hard lower bound needing no join at all: 21 rounds.** Eight tasks have zero triage files anywhere in the corpus (B's nine minus the verified `user-prompts-dir` false positive), covering 11 finding-bearing rounds; seven of the eight have no review file of ANY kind, while `agents-md-drift-guard` has six reviewer files and no triage. Add A's nine verified entries plus the plan-review R2 it missed: 10 more.

### Causes, in order of size

1. **Multi-round loops with one triage file for several finding-bearing rounds. This is the largest bucket and it fits neither pre-registered cause.** `optional-modules-inc2cii` has four `new_valid` rounds (78 to 81) and one triage file; rounds 79 and 80 both carry a `high`, and no triager ruling on either survives. `optional-modules-inc2ci` has two finding-bearing rounds and one file. `structured-skeleton-inc3` has three and one. Step 93's plan review has three and one. Reviewer files for the missing rounds DO survive in most cases, so the round ran a normal reviewer pass and its triage file specifically is absent.
2. **The pre-`Q-14` stratum.** Four tasks (`workflow-hardening`, `convergence-accounting`, `plan-maintenance`, `workflow-doc-fixes`) covering 5 finding-bearing rounds, all with no `ts` field at all. This is the cause the pre-registration named and it accounts for a small minority.
3. **Contemporaneous tasks with no known cause.** `agents-md-drift-guard` (3 rounds, `ts` 2026-07-23), `principle-by-name-projection`, `driver-isolation-reminder-scope`, `single-source-recommendation-rule`. These carry timestamps contemporaneous with well-triaged tasks, so "predates the convention" does not explain them. B correctly declined to force-fit these and I agree; they remain unexplained.
4. **The naming collision**, which cost A one false positive and one omission on a single family. Small in round count, but it is the only cause that actively produces WRONG answers rather than missing ones.

**Confidence: HIGH on the two adjudications (both verified against retrieved files and the oracle). HIGH on the corrected step-93 mapping. MEDIUM on the ~38-round shortfall estimate, which extrapolates A's 15.7 percent slice rate to B's slice.**

---

## Verification of the `valid_findings` inconsistency claim

The brief asked me to verify extractor A's claim that `valid_findings` is inconsistent about whether accepted residuals count, quantify it, and say what it does and does not invalidate.

**The claim is CONFIRMED, from three independent directions.**

**Direction 1, oracle-internal.** Six rounds are recorded `outcome: "clean"` with `valid_findings > 0`, which is only possible if accepted residuals ARE counted: round-idx 164 (`agents-md-drift-guard`), 166 (`principle-by-name-projection`), 181 (`reviewer-reproducible-evidence-inc1`), 183 (`planner-folds-decisions-inc1`), 188 (`decision-folder-currency-fold`), 200 (`decision-folder-currency-inc1`). Five carry 1 low, one carries 3 low.

**Direction 2, cross-model confirmation on shared files.** Both extractors, working independently on different models, found the SAME mismatch on the SAME two overlap files: `lifecycle-capture-triage.md` (round-idx 144) and `q59-backlog-fold-triage.md` (round-idx 148). Both rounds are recorded `clean` with `valid_findings: 0`, and in both the triage explicitly rules findings `VALID` (with dispositions `DEFER` and `ACCEPTABLE`). Since these files are in the mod-5 overlap, this is a genuine independent replication rather than one rater's reading.

**Direction 3, magnitude.** I re-checked every one of A's 8 claimed mismatch rounds against the oracle. A's stated oracle value is correct in all 8 cases. The rate is 8 of A's 51 reconciled rounds = 15.7% [8.1, 28.3]. B independently reports 3 of 50 on its slice, of which 2 are the shared ones above and 1 (`structured-skeleton-inc1`, round-idx 98) runs in the OTHER direction and is unexplained by either extractor. Both A and B extracted 13 rows from that file with identical verdict distributions (10 valid, 1 accept_residual, 2 dismissed, counted = 11), which matches the oracle's 11 exactly, so the disagreement there is between the triage document's own tally section and its enumerated findings, not between the extractors.

### A fourth inconsistency neither extractor reported

147 of 204 rounds carry a `reviewers` array. In 22 of those 147, the sum of the per-reviewer `valid_findings` does not equal the round's `valid_findings`. Nineteen are LOWER than the reviewer sum, which cross-reviewer dedup explains. **Three are HIGHER, which dedup cannot explain:** round-idx 51 (`session-preflight`, round 8 vs reviewers 7), 191 (step 93 plan R1, 5 vs 4), 193 (step 93 plan R2, 3 vs 2). In all three the round-level count includes findings attributed to no listed reviewer. This is a second, independent instrumentation defect in the same field.

### What it invalidates and what it does not

**Invalidated: anything that sums or compares finding COUNTS across rounds.** That includes the 419 total, the medium-or-worse severity mix by risk class (whether stated as 250/169 or 265/154), and any per-finding rate whose denominator is a sum of `valid_findings`. Those figures are reproducible but they are counts of a quantity whose definition changes across rounds, so their precision is false. The 22-versus-25 percent severity comparison should be read as "indistinguishable" rather than as two measured percentages.

**NOT invalidated: the outcome variable, and therefore every Q1, Q2 and Q3 result above.** `outcome` is recorded independently of `valid_findings`, and the six clean-with-findings rounds prove it is not derived from the count. All my convergence results are outcome-based: the 3/22 and 0/22 enforced-second-round figures, the 22 transitions, the transition matrix, the replay, and the fitted chain. None of them reads `valid_findings`.

**One caveat I must state rather than bury.** If a round is sometimes recorded `clean` when its triager ruled findings `VALID`, then `P(clean)` is overstated and my 3/22 UNDERSTATES the marginal yield of the second round. One of my 22 first-clean rounds is directly affected: round-idx 146 (`uniform-isolation` work review R2) is recorded `clean` with `valid_findings: 0` while A extracted two `low` findings the triager ruled valid. That does not change the K+1 outcome I measured, and it does not remove the transition from the set (the streak field is what the workflow acted on, and calibrating the workflow means measuring what the workflow did). But it means the 0/22 medium-or-worse bound is a bound on what got RECORDED, and the recording is known to be lossy in the direction of under-counting. **This is the single largest threat to the Q1 recommendation and it is why my confidence there is MEDIUM rather than HIGH.**

---

## RECOMMENDED CONSTANTS

| Constant | Current | Recommendation | Confidence |
| --- | --- | --- | --- |
| `low_risk` consecutive clean rounds | 1 | **Keep 1** | MEDIUM-HIGH |
| `risky` consecutive clean rounds | 2 | **Keep 2** | MEDIUM |
| Total-round cap | 5 | **Keep 5** | MEDIUM |
| Step 93's escalation specifically | open | **One verification round, merge with a waiver**, with the round's lens scoped to the region the narrowing clause touches | MEDIUM |
| Risk-scaled reviewer count | soft "prefer several" | **No recommendation; not measured** | n/a |

Three of five are "keep". That is a real recommendation here rather than a default, and the arguments are below. The one place the data clearly says the current design is mis-analysed is Q3, and the fix there is not to change a constant.

### `low_risk` bar = 1: KEEP. Confidence MEDIUM-HIGH.

**This is the pure "inconclusive, keep the safe default" case the step's brief anticipates, and I record it as such.** All 57 `low_risk` artifacts converged at streak 1 and none ever ran a second round, so the corpus contains ZERO direct observations of what a second `low_risk` round would find. 57 of 57 censored.

The keep is not purely a default, because there is a transfer argument that runs the right way: the 22 enforced second rounds we DO have are all on `risky` artifacts, which by construction are the artifacts where a second look should pay off most, and they found 3 `low` findings and nothing worse. If the second round adds nothing on the harder class it is unlikely to add more on the easier one. Against that, the transfer is an assumption I cannot test, and the fitted chain shows `low_risk` artifacts have a HIGHER `P(clean | previous new_valid)` (78.4% versus 57.1%), so they are genuinely different loops.

**What would change it:** a forward experiment running one extra round past convergence on `low_risk` artifacts (see FORWARD INSTRUMENTATION). If 35 such rounds return zero medium-or-worse findings, the bound drops below 10 percent and this becomes HIGH confidence. If any of the first ten returns a medium-or-worse, the bar should go to 2 immediately.

### `risky` bar = 2: KEEP. Confidence MEDIUM. This is the closest call here.

The case for lowering to 1 is real and I want it on the record rather than dismissed. Twenty-four extra rounds across 22 artifacts bought three `low` findings and zero medium-or-worse; that is 8 rounds per finding at the lowest severity the scale has. The escalation rate would fall from 22.7 percent observed (23.4 percent modelled) to 2.9 percent modelled, eliminating most of the escalation traffic. And the population that provides the evidence is exactly the population the bar governs, so the usual censoring objection does not apply.

I recommend keeping 2 anyway, for four reasons in descending weight.

1. **The 14.9 percent upper bound is above what I would accept as a silent escape rate on artifacts classified `risky` in the first place.** The point estimate is 0 percent. The bound is what you must be willing to live with, and on 22 `risky` artifacts a 14.9 percent bound means up to about 3 medium-or-worse defects could have escaped undetected.
2. **The 0/22 is a DETECTION result, and the detection instrument is known lossy.** Round-idx 146 is recorded `clean` while its triager ruled two `low` findings valid. Under-recording biases 0/22 downward by an unmeasured amount.
3. **The populations are not exchangeable.** All 22 observations come from artifacts that had already been through the extra fix passes a bar of 2 compels. A bar of 1 would end loops on artifacts that have seen fewer passes, which is a different and less-reviewed population.
4. **The escalation the bar produces is not waste.** All 5 escalation records carry `human_decision: "decision"`, none a resume, and three produced decisions that changed the workflow itself. Lowering the bar would remove most of that traffic along with the cost.

**What would change it:** 20 more zero-event enforced second rounds (total 0/42, upper bound 8.4 percent) would make me recommend lowering to 1, because at that point the accepted risk is comparable to ordinary review variance and the 1.09-rounds-per-artifact saving is large. Conversely, one medium-or-worse finding on any enforced second round settles the question in favour of keeping 2 permanently.

### Cap = 5: KEEP. Confidence MEDIUM.

The ledger currently concludes that the cap is inconsistent with the bar and sits below where the distribution has mass. The arithmetic behind that is right (expected rounds to converge at bar 2 is 3.53 against a cap of 5, so the tail crosses often) but the conclusion does not follow, for three reasons.

1. **Both loops that actually ran a sixth round still failed to converge at a cap of 6.** `structured-skeleton` returned `new_valid` at round 6; `prompt-drift-guard-inc1` returned `clean` at round 6, reaching only streak 1 of 2. On the two observations available, a cap of 6 buys two more agent runs and zero convergences. The other three escalators are censored.
2. **The bar, not the cap, is the binding constraint.** At bar 1 the same fitted chain gives 2.9 percent escalation at cap 5; at bar 2 it gives 23.4 percent at cap 5 and still 10.1 percent at cap 7. Raising the cap is the expensive way to move a number the bar moves eightfold for free.
3. **Escalation is the product, not the friction.** 5 of 5 escalations produced a real human decision.

I am not confident this is right, and I say so: the fitted chain does predict that a cap of 6 halves escalation (23.4 percent to 15.4 percent), and it is well calibrated at the cell I can check. The model and the two observations disagree, and n = 2 cannot settle it.

**What would change it:** the forward experiment below includes running the next two or three cap-reached `risky` loops one round past the cap under an explicit authorisation and recording the outcome. Five such observations, if three or more converge, would justify a cap of 6. If the escalation records ever start carrying `human_decision: "resume"` at a material rate, that is the signal that the cap is firing too early and creating friction rather than decisions, and it should trigger an immediate re-look.

### Step 93: one verification round plus a waiver. Confidence MEDIUM.

Stated in full at Q1. The load-bearing numbers: the enforced-second-round marginal yield is 3/22 = 13.6% [4.7, 33.3] for anything and 0/22 = 0.0% [0.0, 14.9] for medium-or-worse; the fix set is 35 changed lines in one file with zero production lines, which sits in the size tercile with a 22.2% [6.3, 54.7] next-round `new_valid` rate; and the project has already made this exact call four times (`optional-modules-inc2cii`, `waiver-model`, `driver-output-generation`, `prompt-drift-guard-inc1`), each time with a recorded human decision.

The counter-evidence, which is why this is MEDIUM and not higher: step 93's own loop has already had a fix pass inject a MEDIUM test defect (`RG1/MU1`, blamed on round 3's `AD1a` fix), and one of the three authorised fixes is a prose narrowing clause, which is the shape responsible for 20 of the 22 recorded injections corpus-wide.

**What would change it:** if the fix set were re-scoped to the two pure deletions only, deferring the narrowing clause to a separate change, I would raise this to HIGH, because the entire residual risk in the corpus attaches to the authored element. That may not be available at this point in the loop and is a decision for the human, not for me.

### What I am NOT recommending

**Reviewer count and diversity.** The step's design inputs ask for a risk-scaled reviewer-count guideline. Nothing in this corpus measures marginal unique-valid yield per reviewer, because there is no per-finding identity linking the same finding across reviewers. The `reviewers` arrays give per-reviewer counts, and I found 22 of 147 rounds where those counts do not sum to the round total, so even the counts are not a reliable basis. This question is blocked on the schema change, exactly as the sidecar says.

**A severity-trajectory convergence rule.** The sidecar floats converging after N rounds with no finding above severity X. The data is suggestive in its favour, since 0 of 22 enforced second rounds found anything above `low` and 5 of the 6 clean-with-findings rounds carry exactly one `low`. But `valid_findings` and `severities` are the two fields I just showed to be inconsistently written, and building a convergence rule on top of them without fixing them first would encode the defect into the gate. Fix the instrument, then revisit.

---

## FORWARD INSTRUMENTATION

Ordered so a planner can schedule them. Items 1 and 2 are prerequisites for most of the rest.

### 1. Per-finding identity, and the four-value provenance vocabulary. One schema change, not two.

The sidecar already names per-finding identity as the enabling change and the first design question. Two additions this analysis forces.

**The provenance vocabulary must have four values, not three:** `introduced_by_prior_fix`, `pre_existing`, `new_content_of_artifact`, `unstated`. Extractor A found 14 cases where a triager explicitly ruled a defect to be new content of the artifact under review, quotable in words, and had to code every one `unstated` because the three-value schema had nowhere to put them. A three-value vocabulary would silently discard the category triagers record most often, and `new_content_of_artifact` IS the new-content hypothesis.

**`introduced_by_prior_fix` needs a scope qualifier.** Extractor B flagged two rows where the triager attributes causation to a DIFFERENT, EARLIER STEP rather than to a prior round of the same loop (`decision-folder-currency-plan-triage.md` `T-3a` blames step 89; `prompt-drift-guard-triage.md` `FN-2`/`FN-3` blame an earlier step). An injection rate computed per review loop must not pool these with within-loop causation. Record the blamed round as a structured field (`provenance_round`), with a distinguished value for "a prior step, not this loop".

Fields per finding: stable finding id, round identity (step, increment, phase, round number), severity as raised, severity after triage, verdict, class (prose / code / test / config), provenance (four values), provenance_round, whether the finding escaped an earlier round, and reviewer attribution (role, model, harness).

**Make provenance a required field on the triager's output, not a habit.** Only 28 of 82 triage files in this corpus state provenance at all (34.1% [24.8, 44.9]), and both triagers who wrote a provenance section noted they were doing something the reviewers had not. As long as it is optional, the recorded cases will cluster in whichever loops happened to have a diligent triager, which is exactly the bias that makes Q4 unanswerable. `unstated` should mean "the triager considered it and could not determine it", not "nobody asked".

### 2. Fix `valid_findings`, and make the fix testable.

Decide and document one rule: does `valid_findings` count findings the triager ruled valid (including accepted residuals), or findings that required a fix this round? The corpus contains both meanings, at a rate of about 15.7% [8.1, 28.3] of rounds.

Recommended: record BOTH as separate fields (`valid_findings` for all triager-valid findings, `fix_required_findings` for the subset requiring a fix this round), because the two are genuinely different quantities and the workflow uses both. `severities` should be the severities of `valid_findings`, and the existing invariant `len(severities) == valid_findings` (currently 204 of 204) should become a `W`-level deterministic check rather than a coincidence.

Add a second check: `sum(reviewers[].valid_findings) >= valid_findings`. This currently fails on three rounds (round-idx 51, 191, 193) where the round total exceeds the reviewer sum with no possible dedup explanation. That is a cheap deterministic guard on a defect that took a dedicated analysis pass to notice.

### 3. Record the fix-pass commit range on the round record.

Both mechanical explorers hit the same wall from opposite sides, and the cause is this workflow's own writer-isolation rule: fix passes are authored in isolated worktrees, some rebased, some squashed, and merge in a late undifferentiated batch that lands AFTER the loop's round-recording commits. 83 of 115 pairs (72.2 percent) are unresolvable for explorer A and 112 of 113 for explorer B. Mechanical A recovered the true work order using author date over `git log --all --no-merges` (rebase preserves author date), which brought its 27-of-38 empty-window disagreement to zero, but that is archaeology and it does not scale.

Record on each round record: `fix_pass_commits` (the list of commit hashes constituting the fix pass that preceded this round) and `fix_pass_branch`. The orchestrator knows both at the moment it dispatches the implementer. This is a two-field change that makes every mechanical question answerable prospectively.

### 4. Measure fix-pass shape at the TOKEN level and at the per-FIX granularity.

Two findings force this.

**Token, not line.** Under a line-based classifier zero of 32 located fix passes are deletion-only, and this is not an aggregation artefact (19 of 19 single-commit windows are `mixed` too). The cause is that deleting a clause from an unwrapped doc comment reflows the surviving text. Mechanical A re-measured `c5b00a7` at minus 3 plus 2 lines but exactly ONE new token, reproducing this project's own hand-count exactly. The no-hard-wrap convention this project adopted for prose is what makes line-level diff classification useless on its own artifacts. Record `new_tokens` (the multiset difference of added over deleted whitespace tokens) alongside line counts.

**Per-fix, not per-pass.** Zero deletion-only passes exist because passes BUNDLE several fixes and one authored fix contaminates the whole pass. Step 93's own fix set illustrates it: "two of three pure deletions" is a per-fix count with no pass-level equivalent. If the workflow wants a "deletion-class fix" category with a rate attached, the unit must be one finding's remedy. Since item 1 gives every finding an id, record shape per (finding id, fix commit) rather than per pass.

### 5. The forward experiment on the clean-round constant, which is the only thing that fixes the censoring.

This is the item the sidecar already anticipates ("that needs a FORWARD EXPERIMENT, occasionally running one extra round past convergence and recording what it finds, which should be built into the calibration step rather than discovered later"). Concrete design:

- **Sampling.** On a randomly selected 1 in 3 of artifacts that CONVERGE, run one additional review round past convergence, under an explicit `experiment: "post_convergence_probe"` marker on the round record so it is excluded from streak accounting and from the cap. Randomise rather than choosing, or the sample inherits the orchestrator's judgement about which convergences look shaky.
- **Stratify by risk class**, because `low_risk` is currently 57 of 57 censored and is the stratum with no evidence at all. Weight the sample toward `low_risk`.
- **The measured quantity** is P(a post-convergence probe finds a valid finding) and, separately, P(it finds one that is medium-or-worse). The second is the decision-relevant one.
- **Sample size, computed.** From a current position of 0/22 with a 95 percent Wilson upper bound of 14.9 percent: 20 more zero-event probes takes the bound to 8.4 percent; 35 total zero-event probes takes it below 10 percent; 50 more (72 total) takes it to 5.1 percent. At roughly 4 converging artifacts per working day in the current corpus, a 1-in-3 sampling rate reaches 20 additional probes in about three weeks.
- **The stopping rule, pre-registered now.** If any probe returns a medium-or-worse finding, stop and keep both bars where they are. If 20 consecutive probes return no medium-or-worse finding, lower the `risky` bar to 1. If the probes are stratified and `low_risk` accumulates 35 zero-event probes, keep `low_risk` at 1 with HIGH confidence.

### 6. The cap probe, which is cheaper and can run alongside.

On the next three `risky` loops that reach the cap without converging, authorise one round past the cap before escalating, marked `experiment: "post_cap_probe"`, and record whether it converges. Current evidence is 2 observations, both negative. Five observations would settle whether a cap of 6 is worth anything. This costs at most three extra agent runs.

### 7. Record the escalation outcome, not just that one happened.

`human_decision` currently takes `"decision"` and (presumably) `"resume"`. All 5 records say `"decision"`, which is the strongest single piece of evidence that escalation is useful, and it is one field away from being uninterpretable. Extend it: record whether the decision RESUMED the loop (counters reset), ENDED it by acceptance (waiver), or ENDED it by sending back a specific fix. Also record whether a `"trivial"` human interrupt later had to be re-planned, which the sidecar names and which nothing currently captures. Also record whether a dismissed high-severity finding was upheld or overturned by the second triager; zero such records exist and the sidecar names it as the one measurement that genuinely fits a two-rater agreement statistic.

### 8. Fix the review-file namespace, which is now a measured cost.

The shipped convention `<step>-<role>-<disambiguator>` is stated but not enforced, and this analysis shows it now costs correctness rather than tidiness: it produced one false positive and one omission in an extractor's shortfall list, and it makes the file-to-round join non-derivable at corpus scale. The specific mechanism is that two distinct review loops on the same slug (plan review of a step sidecar, work review of an increment) share one flat directory with overlapping round numbers.

Minimum fix: put phase and round in fixed slots, `<step>-<phase>-r<N>-<role>.md`, and add a deterministic check that every round record with `valid_findings > 0` has a review directory entry matching its own (step, phase, round). That check would have caught the shortfall as it accumulated rather than 38 rounds later.

### 9. Do not delete review files at step close, or record the round key inside them before deleting.

The entire archaeology exists because triage files are committed and then deleted at step close, and every join has to be rebuilt from prose headers. If deletion must continue, require each triage file to carry a machine-readable header line naming its round key (step, increment, phase, round number), so that a future extraction is a parse rather than an interpretation. This costs one line per file and removes the largest single source of interpretive error in this whole exercise.

---

## LIMITS

**1. The clean-round question remains censored and my answer inherits it.** 57 of 57 `low_risk` artifacts never ran a second round. All 22 enforced second rounds are `risky`. Nothing I computed observes what an unrun round would have found, and item 5 above is the only remedy.

**2. The 0/22 medium-or-worse bound is the load-bearing number for two of my five recommendations, and it is a bound on RECORDED findings from an instrument I have just shown to be lossy.** Round-idx 146 is recorded `clean` while its triager ruled two `low` findings valid. The bias runs toward under-counting, so 0/22 may be optimistic by an unmeasured amount.

**3. Multiple comparisons.** I ran six pre-registered questions plus roughly a dozen sub-analyses. The only conventional significance test anywhere in this corpus is mechanical A's size association (p = 0.0108, which I reproduce exactly), and it does NOT clear Bonferroni at that explorer's own five tests, let alone at mine. Everything else here is an estimate with an interval or a bound, and none of it should be read as a hypothesis test. The results I would defend under any correction are the ones that are direct enumerations rather than inferences: the 22 enforced second rounds and their severity inventory, the transition counts, the agreement figures, and the two Q6 adjudications.

**4. Correlated raters throughout.** Every reviewer, triager and extractor in this corpus is a Claude-family model. Interpretive extractor A does not state its model in its own record, so the cross-model claim for the interpretive pass rests on the ledger rather than on the artifact. The kappas in Q5 are upper bounds.

**5. Four errors I found in inputs, recorded so they do not travel.**

- Interpretive extractor A's METHOD section 5 states the corpus holds "411 valid findings" over 204 rounds. The true total is 419; 411 is the sum over `new_valid` rounds only. A's own reconciliation is per-round and unaffected.
- Interpretive extractor A's naming inference for the step-93 family is backwards for one of the two files it cites, and its stated header quotation ("# Triage, plan review round 2") does not match the file, which reads "(commit `11d60f3`, round 2)". This produced its round-202 false positive AND caused it to miss step 93's plan-review round 2.
- Interpretive extractor B lists `user-prompts-dir` as having no triage file anywhere. `docs/plans/agent-scaffold.reviews/user-prompts-dir-triage.md` exists and is in extractor A's slice. Nobody caught this before.
- The ledger's step-93 adjudication says "step 93's four triage files". There are five, and the fifth belongs to a different review loop. The conclusion it reached (A has a false positive) is correct; the supporting account of the cause is incomplete.

**6. The 38-round shortfall estimate extrapolates.** It applies extractor A's slice rate of 8 of 51 triage files documenting zero-finding rounds (15.7% [8.1, 28.3]) to extractor B's slice, which I did not verify file by file. The hard lower bound of 21 rounds needs no extrapolation.

**7. I did not re-read the 89 triage files.** I retrieved and read five (the step-93 family) to adjudicate Q6, and I checked specific claims against the oracle throughout. Everything else about finding-level content comes from the two extractions, whose reliability I measured in Q5 and did not independently re-derive.

**8. The episode-splitting rule is mechanical A's, and I adopted it.** It splits between rounds K and K+1 when `consecutive_clean` continuity fails or round K already converged, using only log fields. It produces 89 episodes from 79 tasks. Mechanical B used a different rule (loop key on task, phase and increment) and got 91 loops and 113 pairs against A's 115. The difference does not affect any figure I report, because my Q1/Q2 measurements are within-episode transitions that both rules preserve, but a reader recomputing with B's rule will see 113 pairs where I see 115.

**9. The fitted Markov chain is a two-state approximation and its transition estimates are themselves conditioned on the loop continuing.** `P(clean | previous clean)` is estimable only on loops that did NOT converge at streak 1, which is to say `risky` loops. The chain's cap-5 prediction (23.4 percent) matches the observed 22.7 percent, which is the only calibration check available, and it is one cell.

**10. I recommend keeping three constants and my evidence for two of the three keeps is a bound rather than a measurement.** If a reader wants to read this record as "the analyst found the second clean round has never caught anything serious and recommended keeping it anyway", that reading is fair, and the reasons are argued at RECOMMENDED CONSTANTS rather than assumed. The forward experiment at item 5 is designed specifically to close that gap, and it has a pre-registered stopping rule so that the next analyst does not have to make the same judgement call on the same 22 observations.
