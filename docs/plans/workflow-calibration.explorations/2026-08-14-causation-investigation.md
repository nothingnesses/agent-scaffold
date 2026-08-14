# Causation investigation, 2026-08-14: why the loop turned

The open half of `Q-76`. Run read-only against `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/causation` at `ef55c22`, full history, 1,079 commits, 2026-07-09 to 2026-08-14.

Headline: **the primary hypothesis H1, as it is written, is falsified.** There is no single cause and no single date. The audit's "roughly 2026-07-19 through 07-31" conflates at least two distinct transitions, with different dates and different candidate causes, plus at least one headline measure that has no inflection at all and whose date is an artifact of binning.

## 1. Pre-registration

**This section was fixed before any measurement.** It is reproduced from `00-prereg-FROZEN.md`, written after inspecting only the repository directory layout, the README's description of the product, the union of JSON field names in `docs/metrics/workflow.jsonl`, the commit count and the commits-per-day histogram. No time series, no audit document, no ledger and no step sidecar had been read at that point. It has not been revised. Section 9 records the one place where I added a measure the pre-registration did not name, and what I would have concluded without it.

### Path taxonomy, fixed first

PRODUCT is `src/**`, `tests/**`, `pack/**`, `Cargo.toml`, `Cargo.lock`, `build.rs`, `justfile`, `flake.nix`, `flake.lock`, `rustfmt.toml`, `README.md`, `CHANGELOG.md`, `LICENSE`. The deliverable is a CLI that ships the `pack/` assets, so `pack/` is product. SELFPLAN is `docs/plans/**`. SELFMETRICS is `docs/metrics/**`. PROCESS is root `AGENTS.md`, `.claude/**`, `.agents/**`. OTHER is reported separately and never folded into another class.

### Operational definition of degenerate

The workflow is degenerate to the extent that its effort and its review attention are consumed by its own process artifacts rather than by the product.

- D1, effort share: `churn(PRODUCT) / churn(PRODUCT + SELF)` per day, churn being insertions plus deletions.
- D2, review subject: the share of `type:"round"` records whose subject is SELF rather than PRODUCT.
- D3, review intensity per unit of product: rounds per 1,000 lines of PRODUCT churn.
- D4, self-reference ratio: `churn(SELF) / churn(PRODUCT)` per day, reported for readability and not as independent evidence.

A transition exists if at least two of D1 to D4 show a changepoint within 3 days of each other. Each measure is reported separately and never averaged into an index.

### Sharpness test

Scan every possible split and report the whole objective profile, not only its maximum. SHARP requires a single clear peak, a peak date stable to within 3 days under all four binnings (per-day, per-3-day, per-5-day, and per-commit-index rather than calendar date), and survival of dropping the single largest-churn commit. GRADUAL, meaning H4, if the profile is flat or multi-peaked, or the peak moves more than 3 days on re-binning, or one commit creates it.

### Hypotheses, with confirm and falsify conditions

The H1 event is defined as the commit range first-to-last touching the `structured-skeleton` step or creating `docs/plans/*.plan.toml`, called S_first to S_last.

- **H1**, the migration created a plan surface the loop then reviewed. Confirmed if the D2 changepoint lands at or after S_first and no later than 5 days after S_last, the plan surface jumps in that range, and the pre-S_first level is product-dominant and not already trending. Falsified if the D2 changepoint precedes S_first, or the SELF share of rounds after S_last is not at least 20 percentage points higher than before S_first, or an equally good changepoint exists at an unconnected date.
- **H2**, reverse causation. Confirmed if D1 or D2 is already moving in the degenerate direction before S_first with a monotone pre-segment trend, and the level immediately before S_first is closer to the post level than to the earliest level. Falsified if the pre-S_first series is flat and product-dominant right up to S_first.
- **H3**, a third factor. Confirmed if enumerating every commit from 2026-07-15 to 2026-08-01 turns up a non-migration change matching the changepoint at least as tightly with a plausible mechanism. Falsified if no co-timed change of comparable scope exists. Enumeration before ranking.
- **H4**, no single cause. Confirmed if the sharpness test returns GRADUAL. Falsified if it returns SHARP on at least two of D1 to D4.
- **H5**, the loop's own rules changed. Mechanics means the convergence rule, round cap, consecutive-clean threshold, increments, reviewer roles, phase set, ledger format, `pack/workflow.toml`, `src/workflow.rs`, `src/workflow_spec.rs`, `pack/prompts/**`. Confirmed if such a diff lands in the window and co-times with the changepoint at least as tightly as the migration does. Falsified if no such diff lands in the window, or the metric change clearly precedes the rule change.

H1, H2, H3 and H5 are not mutually exclusive.

### Review-subject classification rule, fixed before seeing its distribution

Applied to each round record, first match wins. Rule 1, if `artifact`, `task` or `step` contains `plan`, `ledger`, `roadmap`, `skeleton`, `question`, `step`, `receipt`, `workflow-calibration`, `audit`, `exploration`, `doc`, `guidance`, `principle`, `prompt` or `agents-md`, classify SELF. Rule 2, else if it contains `src`, `cli`, `command`, `render`, `validate`, `scaffold`, `pack`, `check`, `test`, `module`, `tui`, `crate` or `release`, classify PRODUCT. Rule 3, else UNCLASSIFIED, reported as its own bucket.

Rule 1 fires first and its list is broader, so **this rule is deliberately biased toward finding SELF**. That makes a null result strong evidence and a positive result weak evidence. Distinct `task` and `artifact` strings are named, not only counted.

### Named failure conditions, so this could come out badly

- If D2's SELF share is already above 50 percent before S_first, H1 is dead and I say so.
- If the changepoint profile is flat, I report no inflection and H1, H2, H3 and H5 all become undetermined by this method, however much narrative exists.
- If the audit's window 07-19 to 07-31 does not contain my computed changepoint, I report the audit as wrong on timing.
- If early records lack dates, I date them by the commit that appended them and state that the date is derived.

### Committed not to do

Not to cite `agent-scaffold.ledger.md`, step sidecars, exploration write-ups or the audit as evidence of events; they generate hypotheses only. Not to revise this pre-registration after seeing data. Not to report a count where the items can be named.

## 2. What changed in the window, enumerated from commits

Enumerated before ranking, per H3. Command:

```
git log --reverse --format='%h|%ad|%s' --date=short --since=2026-07-24 --until=2026-08-02
```

with each commit's `--numstat` classified by the taxonomy above. The full listing is long; what matters is the classification.

**Changes to the loop's own machinery, whole history.** This is the complete set, from:

```
git log --reverse --format='%ad %h %s' --date=short -- \
  pack/prompts pack/workflow.toml src/workflow.rs src/workflow_spec.rs
```

Machinery commits cluster on 07-09 (3), 07-10 (5), 07-11 (2), 07-14 (15), 07-15 (4), 07-16 (4), 07-17 (5), 07-18 (6), 07-19 (15), 07-20 (4), 07-23 (3), **07-26 (1)**, 07-30 (2), 08-12 (5). Between 2026-07-24 and 2026-08-11 inclusive there are exactly three, and only one falls at the review-behaviour changepoint identified in section 3:

- `557fa46`, 2026-07-26, "docs: require reviewer findings to carry reproducible evidence (Q-66)". Edits `pack/prompts/reviewer.md` and `pack/prompts/triager.md`. The reviewer half requires a runnable demonstration for a behavioural or correctness claim, and states that for a doc, design or style claim the reproducible evidence is "an exact command (a grep, a diff, or build or validator output) or a `file:line` citation, not a contrived test", adding "do not manufacture a test where a command or a citation already settles the point". The triager half requires reproducing a finding's evidence and dismissing any testable claim that does not reproduce.
- `bef9084` and `b0d9303`, 2026-07-30, naming the planner as the folder at the remaining decision-folding points.

**Other changes in the window, by kind.** 07-24, four commits, all plan design passes for `code-value-audit-static` (`Q-52`, `Q-58`). 07-26, forty commits, of which four carry substantial PRODUCT churn (`3db79c4` +781, `9a7c54d` +625, `b821b0a` +109, `cbe0074` +136, the advisory `audit` subcommand) and the rest are review findings, triage records and committed deletions of review files. 07-27 to 07-31, almost entirely `docs:` commits recording reviewer findings, triage and "clean up ... review files (committed deletion)" at 3,209, 3,013, 3,558, 1,208 and 617 lines. 07-31 onward, the `workflow-enforcement-tier` fold and its increments.

**One structural change just before the window.** `013e6fc`, 2026-07-20, "docs: promote the SE/backlog items into structured plan entries", whose own message reads "Promote the loose backlog that lived only in ledger prose and inside the `Q-44` ask into individually-revisitable plan entries", creating ten deferred steps at orders 57 to 66. Eighteen further steps were created on 07-23. This matters to section 4.

## 3. The measurements

Every command is given so it can be re-run. Scripts referenced are in the scratch directory beside this file.

### M-A. Daily churn by path class

```
git log --reverse --format='C|%H|%ad|%s' --date=format:'%Y-%m-%d %H:%M:%S' --numstat
```
classified by the taxonomy, then aggregated per day (`churn.py daily`).

Product share by day: 1.000, 1.000 on 07-09 and 07-10; 0.210 on 07-11; 0.102 on 07-14; then oscillating between 0.002 and 0.470 for the rest of the project. The product-dominant era is the first three days, before `docs/plans/` existed. `docs/plans` first appears 2026-07-11 in `f484208`, "chore: dogfood the workflow by scaffolding agent-scaffold onto itself".

### M-B. Changepoint scan on D1, four binnings

`changepoint.py all`. Objective is the reduction in sum of squared error from a single split, scanned at every candidate boundary, whole profile reported.

| Binning | Rank-1 split | Mean before | Mean after |
| --- | --- | --- | --- |
| per calendar day | 2026-07-14 | 0.737 | 0.153 |
| per 3-day bin | 2026-07-21 | 0.405 | 0.085 |
| per 5-day bin | 2026-07-24 | 0.483 | 0.099 |
| per 50-commit bin | 2026-07-20 | 0.378 | 0.123 |
| per 100-commit bin | 2026-07-20 | 0.365 | 0.103 |

The rank-1 date moves across a ten-day span, 07-14 to 07-24, against a pre-registered stability threshold of 3 days. The daily profile decays monotonically from its 07-14 maximum with no local peak at 07-19. **D1 is GRADUAL by the pre-registered test.**

### M-C. Net product delivery per day

```
git log --reverse --format='C|%ad' --date=short --numstat -- 'src/*.rs'
```

Net `src/*.rs` by day: 07-18 +3,745 and **07-19 +5,430, the two highest days in the project**. Delivery continues afterwards: 07-26 +1,272, 08-03 +636, 08-12 +465, 08-13 +1,061. Weekly net `src` totals: +2,495, +12,475, +2,867, +635, +814, +1,542.

### M-D. Dating the metrics records

`workflow.jsonl` is append-only and 46 of 347 records carry no `ts`. Each line is dated by the first commit at which the file reached that length (`metrics.py`, cache `linedates.json`). These dates are **derived, not recorded**. The first 44 records were backfilled in one commit, `eaaf13a`, 2026-07-15, "chore: backfill workflow metrics from the ledger (44 rounds)", so those 44 are a transcription of the ledger and are not independent of it. They are used only for gross ordering.

### M-E. Review subject, first pass, and its defect

For each round record, the churn in the commit range from the previous round's commit to its own, classified. First pass gave a product share of the reviewed interval collapsing to 0.030 at round 180 (2026-07-26). **That measure is wrong and I discarded it.** The interval includes `docs/plans/<task>.reviews/**`, which is the review's own output, not its input. Counting a round's findings and triage files as material under review makes the ratio near-tautological.

### M-F. Review subject, decontaminated

`subject2.py`, `subject3.py`. Material under review is CODE (`src/**`, `tests/**`, `.rs`), PACK, PRODUCT_OTHER, PLAN, PROCESS and LEDGER. REVIEW_OUTPUT (`*.reviews/**`) and EXPLORATION are excluded from the denominator and reported separately.

| Era | CODE | PACK | PROD_OTHER | PLAN | PROCESS | LEDGER | under review | **code share** |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1, to 07-18 | 24,192 | 3,592 | 3,326 | 3,299 | 3,498 | 3,076 | 40,983 | **0.590** |
| 2, 07-19 to 07-25 | 10,908 | 393 | 473 | 4,890 | 313 | 248 | 17,225 | **0.633** |
| 3, 07-26 on | 9,907 | 36 | 148 | 5,718 | 60 | 1,360 | 17,229 | **0.575** |

Changepoint scan on code share of reviewed material: rank-1 split is `r130@2026-07-20` under 10-round bins and `r220@2026-08-04` under 20-round bins, a fifteen-day move, with SSE reductions of 0.268 and 0.068. **No stable changepoint. The code share of what the loop reviewed is flat across the whole project.**

### M-G. Findings per round, and the phase decomposition

Per-day and per-round-bin findings per round, scanned at every split (`metrics.py series`, then `scan`). Rank-1 splits: 2026-07-27 daily, 2026-07-30 under 20-round bins, 2026-07-30 under 10-round bins. SSE reduction at `r120-139@2026-07-19` is 1.82 and at `r100-119@2026-07-18` is 0.15, against 29.77 at the maximum. **There is no changepoint in findings per round at 07-19.**

Decomposed by review phase:

| Era | plan_review | work_review | all | plan_review share of rounds |
| --- | --- | --- | --- | --- |
| 1, to 07-18 | 5.50 | 2.30 | 2.54 | 0.073 |
| 2, 07-19 to 07-25 | 0.75 | 0.64 | 0.65 | 0.127 |
| 3, 07-26 on | 5.64 | 4.12 | 4.64 | 0.342 |

Holding the era-1 phase mix fixed, era 3 is 4.24 against a raw 4.64. **The rise is not a mix effect.** It happens inside both phases. The series is U-shaped, not a step: era 2 is far below era 1.

### M-H. The overhead measure, and its changepoint

Review prose produced per line of material reviewed (`overhead.py`). Per 20-round bin:

```
r0@07-15 0.63   r40@07-15 3.04   r60@07-17 2.11   r80@07-18 0.28
r100@07-18 0.96  r120@07-19 1.25  r140@07-23 2.57  r160@07-23 1.22
r180@07-26 10.66 r200@07-30 6.84  r220@08-04 7.67  r240@08-13 2.43
```

Rank-1 changepoint:

| Binning | Rank-1 split | Mean before | Mean after |
| --- | --- | --- | --- |
| 10-round bins | **r180 @ 2026-07-26** | 1.880 | 7.714 |
| 20-round bins | **r180 @ 2026-07-26** | 1.507 | 6.899 |
| 30-round bins | **r180 @ 2026-07-26** | 1.530 | 5.902 |
| 10-round, insertions only | **r180 @ 2026-07-26** | 1.339 | 5.154 |
| 20-round, insertions only | **r180 @ 2026-07-26** | 1.045 | 4.690 |

The same bin is rank 1 in every binning, and under insertions only, which removes the double-count from the committed deletions of review files. **This is the only measure that meets the pre-registered SHARP criterion, and its date is 2026-07-26, outside the two dates the audit's table gives for its own sharpest measures.**

### M-I. The matched-exposure test at the 07-26 boundary

`ordering.py`. `557fa46` is commit #654 of 1,079. Rounds are split in **commit order**, not by calendar date, at that commit. Forty rounds either side:

| | reviewed (lines) | review prose | prose/round | findings/round | reviewer passes | raw findings/pass | valid/raw |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 40 rounds before | **6,078** | 10,017 | 250 | 0.88 | 42 | 0.90 | 0.947 |
| 40 rounds after | **6,063** | 47,934 | 1,198 | 4.30 | 77 | 2.44 | 0.963 |

Total material under review is matched to within 0.2 percent. Review prose multiplies 4.8x, findings per round 4.9x, raw findings per reviewer pass 2.7x. **The triage validity rate is unchanged**, 0.947 to 0.963, so the change is in what reviewers raised, not in what the triager allowed. Reviewer passes per round rose 1.45 to 1.93, which accounts for about 1.3x of the 4.8x; prose per reviewer pass roughly tripled, from a range of 119 to 370 lines before to 517 to 956 lines after.

### M-J. Step productivity, with the censoring control

`steps.py`. A step's creation date is the first commit adding its slug under `docs/plans`; what it changed is the churn of every commit whose message names the slug. Restricted to steps that are `complete` and have at least one commit naming the slug, which removes both the never-built censoring and the early-step blind spot where old commits do not name slugs:

| Steps created | n | produced any product churn | share |
| --- | --- | --- | --- |
| on or before 2026-07-18 | 31 | 28 | **90%** |
| on or after 2026-07-19 | 26 | 5 | **19%** |

By creation day: 07-14 16/16, 07-16 5/6, 07-17 1/1, 07-18 2/3, then **07-19 0/4, 07-20 1/1, 07-23 2/14**, 07-26 1/2, 07-27 0/2, 07-28 0/1, 07-31 1/1, 08-13 0/1.

### M-K. Human involvement

`metrics.py human`. The `decision` record type does not exist before 2026-07-18, so no comparison spans the migration. Decisions per active day: 4, 5, 8, 3, 2, 0, 6, 4, 3, 5, 3, 1, 4, 6, 1, 3, 5, 8, 8, 8. Escalations are flat at 0 to 2. There is no step at 07-26. The rise to 8 per day falls on 08-12 to 08-14, which is the audit period itself.

## 4. Hypothesis by hypothesis

### H1, the migration created a plan surface the loop then reviewed: FALSIFIED

Decided by M-F. The pre-registered falsifier was that the SELF share of rounds after S_last is not at least 20 percentage points above the level before S_first. Measured on the material actually under review, the code share is 0.590 before the migration, 0.633 immediately after it, and 0.575 after 07-26. It moves by less than 6 points across the whole project and in the wrong direction immediately after the migration. The changepoint scan finds no binning-stable split on this measure, the rank-1 date moving fifteen days between binnings.

Two further measurements point the same way. The single highest product share of any reviewed interval in the project is 0.629 at bin `r80`, first day 2026-07-18, which is the migration itself. And 07-18 and 07-19 are the two largest net `src/*.rs` days ever recorded, +3,745 and +5,430.

H1 is falsified specifically on its stated mechanism, that **review effort redirected from product to plan**. It did not. The loop went on reviewing roughly 60 percent code for the rest of the project.

### H1-adjacent, the migration changed the work queue: SUPPORTED, moderate confidence

This is not H1 and I am not permitted to score it as H1. It is the hypothesis the data suggests in H1's place, and it is added here rather than in the pre-registration, which section 9 records.

M-J shows a sharp step at exactly the migration boundary: complete steps created on or before 07-18 produced product 90 percent of the time, those created from 07-19 on, 19 percent. It survives the censoring control, because the 07-23 batch was largely built rather than left deferred, 14 complete steps of which 2 produced product.

The mechanism is visible in a diff and a commit message rather than inferred. `013e6fc`, 07-20, one day after the migration, promoted "the loose backlog that lived only in ledger prose and inside the `Q-44` ask" into ten individually-revisitable plan entries, and eighteen more steps followed on 07-23. Before the migration there was no structured step entry to promote into. Each promoted item then became eligible for its own review loop.

Confidence is moderate, not high, for one reason stated plainly: **the backlog items pre-existed the migration as prose notes.** The migration did not create them. It changed their status from informal notes to formal steps that each drew a review loop. Whether that changed behaviour or merely recorded an existing drift is the crux, and the counterfactual, where the backlog stayed as prose, is not observable.

### H2, reverse causation: NOT SUPPORTED for the 07-26 transition, PARTIALLY LIVE for the queue

The pre-registered falsifier was that the pre-event series is flat and product-dominant right up to the event. For the 07-26 transition it is better than flat, it is quiet: findings per round in era 2 is 0.65 against 2.54 in era 1, raw findings per reviewer pass drops to 0.42 to 1.10 from 2.15 to 2.76, and the outcome mix at `r140-159` is 16 clean rounds against 4 with findings, the quietest stretch in the project. The loop was not degenerating immediately before 07-26. It was converging fast on small items.

For the queue transition, H2 stays partially live on the point already conceded above: the backlog items existed before the migration.

### H3, a third factor: FALSIFIED as stated, on the enumeration

The pre-registered falsifier was that the enumeration turns up no co-timed change of comparable scope. Between 2026-07-24 and 2026-08-11 the complete set of changes to the loop's machinery is three commits, and only `557fa46` on 07-26 falls at the changepoint; the other two are on 07-30 and rename a role in a decision-folding step. No other candidate of comparable scope is co-timed. The enumeration was performed before ranking.

### H4, no single cause, the fortnight is partly a binning artefact: SUPPORTED

Confirmed on its pre-registered condition for D1, whose rank-1 split moves 07-14 to 07-24 across binnings against a 3-day threshold, and whose daily profile decays monotonically with no local peak at 07-19.

It is confirmed more sharply on one of the audit's own headline measures. Documentation-to-code by bytes, measured on the tracked tree at the last commit of each day:

```
07-14 1.50:1   07-18 2.47:1   07-19 3.23:1   07-20 3.22:1
07-26 3.40:1   07-31 4.98:1   08-14 4.62:1
```

The series climbs monotonically from the start of dogfooding and has no inflection at 07-19. Any split would show before below after. The largest jump is 07-26 to 07-31, not 07-19.

So H4 is right about part of the picture and wrong about the rest: the effort-share and documentation-ratio measures are genuinely gradual and their dates are artifacts, but two behavioural measures do have real, binning-stable changepoints, and they fall on different dates from each other.

### H5, the loop's own rules changed: SUPPORTED for the 07-26 transition, moderate confidence

The pre-registered confirm condition was that a machinery diff lands in the window and co-times with the changepoint at least as tightly as the migration does. It does, and more tightly: `557fa46` is on 2026-07-26, and the overhead changepoint is at the round bin beginning 2026-07-26 under every binning tested, while the migration is seven to eleven days earlier and carries an SSE reduction of 1.82 against 29.77 at the maximum on findings per round.

The mechanism is specific and matches the measurement. The rule made a `file:line` citation or a grep sufficient evidence for a doc, design or style claim, while requiring a runnable demonstration, "the strongest form a mutation", for a behavioural or correctness claim. That lowers the cost of a prose finding and raises the cost of a behaviour finding. What is measured after it: raw findings per reviewer pass up 2.7x, prose per reviewer pass roughly tripled, triage validity rate unchanged, exposure matched to within 0.2 percent in total.

Confidence is moderate, not high, and the reason is a named residual confound. Exposure is matched in total but **not in shape**: median material reviewed per round falls from 47 lines to 16, with 3 zero-exposure rounds before and 7 after and the upper quartile falling from 147 to 51. A few large rounds hold the totals equal. So part of the per-unit overhead rise is denominator-driven. The measures that are not denominator-driven, prose per round 250 to 1,198, prose per reviewer pass roughly tripled, and raw findings per pass 0.90 to 2.44, all move in the same direction, which is why the hypothesis survives at all. One honest limit on the size of the effect: raw findings per pass after the rule, 2.44, is close to the earliest era's 2.15 to 2.76, so on that measure 07-26 is a **return to the early baseline** after an anomalously quiet fortnight, not an unprecedented level. What is genuinely unprecedented is the prose volume per pass.

## 5. Verdict

**H1 is falsified as stated.** Review effort did not redirect from product to plan. The material the loop reviewed stayed about 60 percent code from the first week to the last, with no binning-stable changepoint.

**There is no single cause and no single date.** The audit's fortnight contains at least two distinct transitions, and at least one of its headline measures has no inflection at all.

- **Transition A, the work queue, 2026-07-19 to 07-20.** Complete steps created before the migration produced product 90 percent of the time; those created after, 19 percent. Co-timed with `structured-skeleton` and with `013e6fc`, which one day later promoted the prose backlog into ten formal steps, followed by eighteen more on 07-23. Supported at **moderate confidence**, limited by the fact that the backlog items pre-existed as prose, so the migration formalised them rather than creating them.
- **Transition B, review intensity, 2026-07-26.** Review prose per line of material reviewed steps from about 1.5 to about 6.9, rank 1 at the same bin under four binnings including insertions-only. Matched-exposure comparison across the boundary commit shows 4.8x the prose and 2.7x the raw findings per reviewer pass on 6,078 against 6,063 lines of material, with the triage validity rate unchanged. The only co-timed change to the loop's machinery is `557fa46`, whose content predicts exactly this. Supported at **moderate confidence**, limited by the exposure-shape confound.
- **Neither transition is at 07-31**, and the audit's two 07-19 datings do not survive re-measurement.

**Confidence statement.** High confidence that H1 as written is false, because the falsifying measurement is direct, pre-registered, and robust to binning. High confidence that the transition is not single or sharp in the way the audit's table presents it. **Moderate confidence** on each of the two proposed causes: both are co-timed to within a day, both have a mechanism visible in a diff rather than inferred from prose, and neither is established by anything stronger than co-timing plus mechanism, which is not proof of causation. I would not describe either as established. Low confidence on their relative sizes, which I did not attempt to partition.

Reported as pre-registered: **the evidence settles the falsification and does not settle the attribution.**

## 6. What would settle it

**Transition B is settleable and the material still exists.** It needs a differential test rather than an observational one: run the reviewer prompt at `557fa46^` and at `557fa46` against the same artifact, several times each, and compare findings per pass and prose per pass. That converts co-timing into a controlled comparison and directly tests whether the rule text causes the behaviour. `workflow-audit-followups.md` records that the triage files are recoverable with `git show <commit>:docs/plans/agent-scaffold.reviews/<file>` at `281f0ad`, `d7c9fb0`, `306d13a`, `01333f3`, `8993642`, `7ed86f8` and `e82e303`, so real artifacts of known difficulty are available as inputs. This is the single highest-value missing measurement.

**Transition A is not settleable from this repository.** It needs the counterfactual in which the backlog stayed as prose notes, which no record contains.

**One schema gap blocks a sharper test of transition B.** The log records aggregate `raw_findings` and `valid_findings` per reviewer pass and never which pass raised which finding, and never the finding's kind. So I cannot measure whether the additional findings after 07-26 are of a different kind, which is the mechanism's central prediction. The audit's methods reference already records this gap for M6, capture-recapture. It is recoverable only for the loops whose findings files still exist in history, and only by reading them by hand.

**Exposure shape.** The confound in M-I could be removed by matching rounds pairwise on material reviewed rather than matching totals, at the cost of a much smaller sample. With 40 rounds a side and a skewed distribution, I judged the matched-pairs version underpowered rather than informative, and I did not run it. That judgement could be wrong and the test is cheap to run.

## 7. What the audit got wrong

Re-measured, with the command in each case. Two of its claims reproduce and are noted as such.

**7.1. "Steps generated by the process itself, 8.3% to 54.2%, monotonic" is confounded with the introduction of the field that records it.**

`[step.provenance]` first appears in `b949a1c` on **2026-07-20**, one day after the migration:

```
git log --reverse --format='%ad %h %s' --date=short -S'[step.provenance]' -- docs/plans
```

No step created before the plan became TOML can carry it, and none was backfilled: 0 of 45 steps at orders 1 to 45 have a decision provenance, against 29 of 54 at orders 46 and above, which is **53.7%**. The audit's after figure is 54.2%. The near-identity indicates the audit read this field. A measure that is structurally zero before a schema exists and about half after it will look monotone under era binning whatever the underlying behaviour did. `Q-76` already records that this measure "is marked MONOTONIC rather than dated, so it does not turn at 07-19 and cannot be used to date a cause there"; the stronger statement is that it cannot date anything, because its zero era is an artifact.

**7.2. "Net Rust delivered per week, +3,733 before, +48 after, turn 07-19" does not reproduce.**

Searching every window of 7 days or more across the whole history for a net `src/*.rs` rate anywhere near +48 per week returns **exactly one** window in the range 0 to +120, namely 2026-08-05 to 08-11 at +107 per week. Under three scopes, `src`, `src tests`, and `src tests pack`, restricted to `.rs`, the post-07-19 rate is +1,577 to +3,867 per week. Weekly net `src` totals are +2,495, +12,475, +2,867, +635, +814, +1,542. 2026-07-19 is the **highest** net `src/*.rs` day in the project at +5,430, and 07-18 is second at +3,745. A reader of the audit's table would conclude that Rust delivery stopped at 07-19. It did not.

**7.3. "Documentation to code by bytes, 1.28:1 to 5.17:1, turn 07-19" describes the endpoints of a monotone climb, not a turn.**

Measured on the tracked tree: 07-14 1.50:1, 07-18 2.47:1, 07-19 3.23:1, 07-20 3.22:1, 07-26 3.40:1, 07-31 4.98:1, 08-14 4.62:1. There is no inflection at 07-19, and the largest jump in the sampled points is 07-26 to 07-31. Dating this measure to 07-19 is a binning artefact of the kind the audit's own methods reference warns about under M2, "the proxy is chosen after seeing the data ... it happens to careful people".

**7.4. The audit's claim that five auditors independently identified the same fortnight is weaker evidence than it reads as.** Independent agents measuring overlapping quantities on one monotone series will converge on similar split dates whether or not an inflection exists. Sections 7.1 to 7.3 show that at least three of the six measures in the audit's table cannot date a cause. Agreement among them is not corroboration.

**Claims that do reproduce.** "74.3% of 1,022 commits touched neither product code nor tests": I measure **74.5%**, 804 of 1,079, at a later HEAD, using the pre-registered PRODUCT taxonomy. Convergence: the audit's 76 of 100 loops against my 80 of 98, 81.6%, grouping by `increment` else `task`; the difference is grouping, and the audit documents its grouping.

**One error of my own, recorded because it is the same class.** My first review-subject measure counted each round's own findings and triage files as material under review, and produced an apparent collapse to 3 percent product at 07-26. That is near-tautological, since the review's output scales with its findings. It is corrected in M-F, and the corrected measure falsifies the collapse. Had I stopped at the first version, I would have reported H1 confirmed with a changepoint at 07-26.

## 8. Prevention

Only what follows from what was actually established. No general list.

**From transition B, which is the better-supported of the two, and only at moderate confidence.** The rule in `557fa46` set two different evidence prices in one sentence: a citation or a grep for a prose claim, a runnable demonstration or a mutation for a behavioural one. Whatever else it did, it made the cheap class cheaper than the expensive class at the moment the project's remaining work was mostly prose. If that rule is kept, the prevention that follows from the measurement is to **price the two classes so the cheaper one is not the one that pays**, and to instrument the split so the effect is visible next time: record each finding's class alongside its severity. The project already has the field for it, since severities are recorded per round.

**From the measurement method rather than from either cause, because this one is established rather than moderate.** Three of the audit's six headline measures cannot date a cause, one of them because the field it reads did not exist before the date it was used to date. Before a measure is used to date a transition, **check when its own recording mechanism was introduced**, and do not date a cause with a measure whose zero era is a schema gap. This follows directly from 7.1, which is a re-measurement and not a judgement.

**What does not follow, stated so it is not inferred.** Nothing here justifies reverting `557fa46`, because the rule's effect on finding quality was not measured, only its effect on volume, and the audit separately records that the loop's genuine catches were concentrated in behavioural defects that this rule's expensive class is designed to produce. Nothing here justifies a change to the plan-as-data migration, whose stated hypothesis is falsified. And no prevention follows for transition A at all, because its cause is not established, only co-timed.

## 9. Deviation from the pre-registration

One, recorded as required.

The pre-registration named D1 to D4 and did not name the overhead measure, review prose produced per line of material reviewed, which section 3 M-H uses and which is the only measure meeting the SHARP criterion. I added it after seeing that D2's decontamination in M-F left the review-subject question answered and the intensity question open.

**What I would have concluded under the original pre-registration alone.** D1 is GRADUAL, D2 shows no stable changepoint, D3 and D4 are functions of the same two series. On D1 to D4 only, the verdict would have been: H1 falsified, H4 supported, and H2, H3 and H5 all undetermined for want of any measure with a locatable changepoint. The addition of the overhead measure is what moves H5 from undetermined to supported at moderate confidence. A reader who regards a post hoc measure as inadmissible should read H5 as **undetermined** and the verdict as "H1 falsified, no cause established", which is a legitimate reading of this evidence and is weaker than the verdict in section 5 rather than different in direction.
