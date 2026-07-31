# Fix-pass shape, mechanical lens (explorer A)

Corpus: this repository at `12d6a01`, worktree `.claude/worktrees/cal-mech-a`, branch `explore/cal-mech-a`. All 204 `type: "round"` records in `docs/metrics/workflow.jsonl`, and the full commit graph reachable from every ref (759 commits, 739 non-merge).

Every quantity below is derived from git metadata and diffs, never from anyone's prose description of what a fix pass did. Scripts are in the session scratch directory `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/cal-a/` as `01_map_rounds_to_commits.py`, `02_record_type_history.py`, `06_final.py` (the pipeline that writes the dataset), `07_report.py` and `08_extra.py`. The derived dataset `fix-pass-shape-a.tsv` sits beside this file and carries every per-pair number, so the result tables can be recomputed from it without re-running git.

## HEADLINE, stated before the method so it cannot be buried

The pre-registered primary measurement CANNOT BE COMPUTED as specified, because one of its two arms is empty. Under the pre-registered line-based definition (`deletion_only` = the diff removes lines and adds none), **zero of the 32 locatable fix passes are deletion-only**. Every located fix pass that changed anything both added and removed substantive lines. The primary contrast has cell counts `deletion_only` n=0 against `authoring` n=27 (plus 5 no-change pairs), so `P(new_valid | deletion_only)` is undefined and no comparison is possible.

That is not a null result about the hypothesis. It is a finding about the DEFINITION: the project's own five hand-recorded "deletion-only" fix passes are not line-deletion-only either. Spot-checking the one the ledger describes in most detail (`3e4fb6c`, called "pure deletion at BOTH sites, zero words authored") shows a diff of `-3 +2` lines in `src/agents_md_drift.rs`, because deleting a clause from a wrapped doc comment reflows the surviving text. Line-level shape cannot see that. A token-multiset measure can, and reproduces the ledger's own hand-measurement exactly (1 new token, `self-extending.` displacing `self-extending:`).

The one association that does survive is SIZE, not shape: among the 27 located fix passes on changed artifacts, the median new-token count before a `new_valid` round is 761.5 against 88 before a `clean` round (Mann-Whitney AUC 0.788, normal-approximation two-sided p ~ 0.011, n=27). But size predicting `new_valid` is equally consistent with INJECTION and with NEW CONTENT, so it does not separate the two explanations the archaeology was convened to separate.

## METHOD

### M1. Round timeline and its git anchor

`docs/metrics/workflow.jsonl` at `12d6a01` holds 231 records, of which 204 are `type: "round"`. Round index `i` (1-based, file order) is anchored to the FIRST commit whose version of the file contains at least `i` round records:

    git rev-list --reverse HEAD -- docs/metrics/workflow.jsonl        # 117 commits
    git show <commit>:docs/metrics/workflow.jsonl                     # count type=="round" lines

Anchoring BY INDEX rather than by matching the raw JSON line is mandatory, and this is a real trap rather than a hypothetical one. Two commits rewrote already-recorded round records in place:

- `1824e7d` (2026-07-16T13:57), "feat(metrics): require risk_class on round records and backfill the log", rewrote 46 existing round lines (46 insertions, 46 deletions, round count unchanged at 52).
- `7715fa6` (2026-07-20T11:00), "fix: give Q-58 capture its own orphan task", rewrote one round line's `task` field (round count unchanged at 136).

Matching by raw line would have dated 52 rounds to the risk_class backfill commit instead of to their real recording commits.

Integrity check on the anchor: across all 117 commits touching the file the round count is monotone non-decreasing (0 -> 44 -> 45 -> ... -> 204), so no round record was ever deleted and the index anchor is well defined. Verified in `02_record_type_history.py`.

### M2. Artifact grouping and episode splitting

An artifact is a review loop. Grouping is done in two steps.

1. Group all rounds sharing a `task` value. This yields 79 groups and is the grouping the ledger used (see RECONCILIATION).
2. Split a group between round K and K+1 when the log itself says the loop ended or restarted. Split if EITHER
   - `consecutive_clean` continuity fails: `cc(K+1) != (cc(K) + 1 if outcome(K+1) == "clean" else 0)`, OR
   - round K already converged: `cc(K) >= bar(risk_class(K))` with `bar = 1` for `low_risk` and `2` for `risky`.

This split rule is derived purely from log fields, not from prose. It fires 10 times and yields 89 episodes. Every split it makes is confirmed by the recording commit's own subject line, checked after the fact: `task-entry-regrounding` r128 -> r129 separates "inc1 round 2 (clean, converged)" from "inc2 round 1"; `driver-output-generation` r139 -> r140 separates inc1 from inc2; `workflow-driver` r118 -> r119 separates stage0a from stage0b; `plan-fold` r59 -> r134 separates two episodes three days apart. Without the split, the pair r59 -> r134 would have had a fix-pass window of 146 commits and 17,708 added lines covering the entire repository's work for three days.

Pairs are formed only WITHIN an episode: 115 pairs from 89 episodes over 204 rounds.

### M3. Fix-pass window

This is where the naive method fails and it is worth stating why, because it changes every number downstream.

Fix passes in this project are authored inside isolated git worktrees. Some are merged, some are rebased onto the main line later, and one loop's are still on an unmerged branch. Main-line ancestry ORDER is therefore not the work order. Concretely, for step 93 the round-1 and round-2 recording commits are adjacent on main with nothing between them but reviewer files, while the fix that ran between those two reviews (`339d26a`, author date 2026-07-30T16:11) sits on the unmerged branch `impl/checks-collision`, rebased onto a commit whose author date is 23:18. A `git diff C_K C_{K+1}` on main finds nothing. Under that definition 27 of 38 windows measured as empty while the log said the artifact had changed.

AUTHOR DATE is the work order, and rebase preserves it. So the window for the pair (round K -> round K+1) is:

    every commit in `git log --all --no-merges` whose AUTHOR DATE lies in [t_K, t_{K+1}),
    excluding stash commits (subject matching ^(On |index on |untracked files on |WIP on )),
    where t_i is the author date of round i's recording commit.

Merges are excluded so branch content is not counted twice; both parents of every merge are themselves in the universe. Commits unreachable from any ref (for example `3e4fb6c`, the pre-rebase copy of `c5b00a7`) are absent from `--all` and so cannot double-count. Exactly two commits in the universe share an (author date, subject) pair (`c0e880a`/`aa771ce`, the pre-rebase pair on `impl/planner-folds-decisions`); neither falls inside any included pair's window, checked explicitly.

The window is left-closed so that a fix bundled into round K's own recording commit is attributed to the pass that follows round K, which is chronologically correct. A sensitivity variant excluding round K's recording commit entirely is carried in the dataset as `shape3_excl_prev`; it changes no cell.

### M4. Artifact paths

The artifact is everything EXCEPT the ledger, the metrics log and the review files:

    git show --format=%n -U0 -M <commit> -- \
      ':(exclude)docs/metrics/workflow.jsonl' \
      ':(exclude)docs/plans/*.ledger.md' \
      ':(exclude)docs/plans/*.reviews/**' \
      ':(exclude)docs/plans/*/*.reviews/**'

Exploration records under `docs/plans/*.explorations/` are NOT excluded, because the brief's exclusion list did not name them and they are plan-adjacent content. This has one measurable consequence, checked rather than assumed: exactly one commit adding exploration files (`cffde3a0`, 352 added lines across two `doc-redundancy.explorations` files) falls inside exactly one included window, the pair r110 -> r111. Recomputing that pair with `docs/plans/*.explorations/**` also excluded gives `subst_added` 367 instead of 565 and `new_tokens` 6167 instead of 12298. Both revised values stay above every value in the `clean` group and inside the same tercile, so no rank crossing occurs and every table in RESULTS, including both Mann-Whitney statistics, is unchanged to the digit. The dataset carries the as-measured values.

### M5. The classifier, exactly

Per window, added lines are the `+` lines of the above diff and deleted lines the `-` lines, with file headers and hunk headers dropped. A line is SUBSTANTIVE when, after stripping leading and trailing whitespace, it is non-empty and is not composed entirely of characters from the set `,;{}()[]. "'` + backtick + `|-+*=&:<>/\!?#`. That is the operationalisation of "adds only whitespace or a terminator character".

SHAPE-3 (the pre-registered line-based classifier), where `A` = substantive added lines and `D` = substantive deleted lines:

- `deletion_only`   : `|D| > 0` and `|A| == 0`
- `authored_pure_insert` : `|A| > 0` and `|D| == 0`
- `mixed`          : `|A| > 0` and `|D| > 0`
- `no_change`      : `|A| == 0` and `|D| == 0`

SHAPE-2 (the collapsed binary the hypothesis actually contrasts): `deletion_only` against `authoring` (= `authored_pure_insert` or `mixed`), with `no_change` kept as its own arm.

TOKEN-SHAPE (the alternative operationalisation, matching what this project itself measured by hand): tokenise each substantive line by whitespace, build the multiset of added tokens `T_A` and deleted tokens `T_D`, and set `new_tokens = sum((T_A - T_D).values())` (multiset difference). A pass with `new_tokens == 0` authored no word that was not already on the page. `zero_new_tokens` against `new_tokens > 0`.

Also recorded per window: `lines_added`, `lines_deleted`, `subst_added`, `subst_deleted`, `files_touched`, `code_files`, `doc_files`, `code_noncomment_added` (substantive added lines in `.rs/.toml/.nix/.sh/.yml/.yaml/.json/.lock/justfile` that are not `//` or `#` comments), `comment_added`, `doc_added` (substantive added lines in `.md/.txt`), and the commit list.

**Explorer B, read this paragraph.** My `deletion_only` is a LINE predicate over the NET window diff. It is strictly narrower than the ledger's phrase "pure deletion", which tolerates the reflow of surviving text. If your method derives shape from what a triager or implementer SAID the fix was, your `deletion_only` will be roughly my `new_tokens <= 1` and will not be my `deletion_only`. Compare against my `token_shape` column, not my `shape3` column, or we will disagree for definitional reasons alone.

### M6. Oracle correction

`changed_since_prev` was written at the time by the participant and is the authority on whether a fix pass existed at all. Where it says `false`, the pair is classified `no_change` regardless of what my window measured. This is carried as `shape_corrected` and it moves exactly one pair (see EXCLUSIONS, round 113).

## RECONCILIATION against the JSONL oracle

Every figure the ledger published from this log re-derives exactly.

| Quantity | This analysis | Ledger's recorded value | Match |
| --- | --- | --- | --- |
| Round records | 204 | 204 | Yes |
| `sum(valid_findings)` | 419 | 419 (250 + 169) | Yes |
| `sum(len(severities))` | 419 | n/a | consistent |
| Rounds where `len(severities) != valid_findings` | 0 of 204 | n/a | consistent |
| Severity totals | critical 1, high 12, medium 85, low 321 | n/a | sums to 419 |
| Medium-or-worse | 98 | 98 (55 + 43) | Yes |
| Outcomes | `new_valid` 102, `clean` 102 | n/a | sums to 204 |
| CHANGED -> `new_valid` | 99/183 = 54.1% (95% Wilson 46.9 to 61.2) | 99/183 = 54.1%, CI 46.9 to 61.2 | Yes |
| UNCHANGED -> `new_valid` | 3/21 = 14.3% (95% Wilson 5.0 to 34.6) | 3/21 = 14.3%, CI 5.0 to 34.6 | Yes |
| Artifacts | 79 | 79 (57 + 22) | Yes |
| `low_risk` artifacts | 57; median 2 rounds, mean excess 0.98, max 5 | 57; median 2, mean excess 0.98, max 5 | Yes |
| `risky` artifacts | 22; median 4 rounds, mean excess 2.14, max 9 | 22; median 4, mean excess 2.14, max 9 | Yes |
| Risky artifacts past the cap of 5 | 4 of 22 = 18.2% | 4 of 22 = 18 percent | Yes |
| Findings medium-or-worse, `low_risk` | 55 of 250 = 22% | 55 of 250 = 22 percent | Yes |
| Findings medium-or-worse, `risky` | 43 of 169 = 25% | 43 of 169 = 25 percent | Yes |

**One definitional detail had to be recovered before the last five rows would reconcile, and it is exactly the class of thing that has burned this project before.** Two tasks carry BOTH risk classes across their rounds (`structured-skeleton`: low, low, low, risky, risky, risky; `task-entry-regrounding`: low, low, risky, risky, risky), both escalating mid-loop. Assigning the artifact its FIRST round's class gives 59 low_risk / 20 risky, `low_risk` max 6, `risky` median 3.5 and mean excess 2.00, and it does NOT reproduce the ledger. Assigning `risky` if ANY round is risky (equivalently, the LAST round's class, since both mixed tasks escalate monotonically) reproduces every published figure to the digit. Anyone recomputing these numbers must use the any-round-risky rule or they will get different answers and think one of us is wrong.

Two further history facts, both checked rather than assumed:

- **The prune.** `ddc7a30` (2026-07-19T10:37, "feat: cut this repo's plan over to the TOML source (structured-skeleton Inc 5)") removed 16 `waiver` records and 1 `baseline` record in one commit. Round count across that commit is unchanged at 107. The stated hazard is confirmed exactly, and it does not touch the round series.
- **The backfill.** The file was created at `eaaf13a` (2026-07-15T18:08, "chore: backfill workflow metrics from the ledger (44 rounds)") already containing 44 rounds. Those 44 rounds have no individual git timestamp, which is the largest single source of exclusion below.

## EXCLUSIONS, with counts

Of 115 within-episode pairs:

| Status | n | Reason |
| --- | --- | --- |
| Included | 32 | Fix-pass window is well defined |
| Excluded | 83 | `shared_recording_commit`: rounds K and K+1 were appended to the log in the SAME commit, so the window is empty by construction and the fix pass between them cannot be separated from the rest of that commit's window |
| Excluded | 0 | `fix_pass_not_in_git`: none. Every pair whose log record says the artifact changed has at least one artifact-touching commit in its window |

That is a 72.2 percent exclusion rate and it is the dominant limit on this analysis. Breakdown of the 83 by the date of the shared recording commit: 2026-07-15 (the 44-round backfill) 22, 07-18 17, 07-19 13, 07-23 9, 07-17 7, 07-16 6, 07-26 5, 07-20 2, 07-24 2. The pattern is chronological and sharp: early practice logged a whole converged loop's rounds in one commit at convergence, and NOT ONE pair is excluded after 2026-07-26, because from 2026-07-27 onward each round gets its own commit. So the included sample is biased LATE, toward the current working style: 16 of the 32 included pairs close on or after 2026-07-27 (12 on 07-28, 3 on 07-30, 1 on 07-31), against 83 of 83 exclusions falling on or before 07-26.

Two further data-quality flags carried in the dataset rather than silently absorbed:

- **Concurrency contamination, 5 of 32 included pairs** (rounds 192, 193, 194, 195, 196). Step 92 (`prompt-drift-guard-inc1`) and step 93 (`checks-runner-worktree-name-collision`) ran interleaved on 2026-07-28, so four commits fall inside two different episodes' windows and are counted in both. An author-date window cannot attribute a commit to one of two simultaneously open loops.
- **One provable boundary mis-attribution, round 113.** The log records `changed_since_prev: false`, but commit `ce0d36c` ("fix: exclude the scaffolded TEMPLATE.md render artifact from treefmt (Inc 6 R3-1)", author date 14:00:53) falls inside the window `[13:37:21, 14:07:02)`. Its subject shows it is the fix for round 3's finding, authored AFTER round 3 ran and bundled into the same commit as round 3's record. The oracle is right and my window is wrong. This is a systematic direction of error: a fix authored between a review finishing and its record being committed is attributed to the previous pair. It is the reason the oracle correction in M6 exists. Rate of detected occurrence: 1 of 32 located pairs (3.1 percent), detectable only because `changed_since_prev` disagreed; an equivalent mis-attribution on a pair where the artifact HAD also genuinely changed would be invisible.

Also excluded from the size analyses, but included in the shape tables: 5 pairs where the artifact did not change at all (`changed_since_prev: false`), leaving 27 located CHANGED fix passes.

## RESULTS

All intervals are 95 percent Wilson. n is small everywhere and several cells are too small to support any conclusion; that is said per-cell below rather than left to the reader.

### R1. PRIMARY: P(next round is `new_valid` | fix-pass shape)

Pre-registered line-based classifier, oracle-corrected, n=32:

| Fix-pass shape | n | next round `new_valid` | 95% Wilson CI |
| --- | --- | --- | --- |
| `deletion_only` | **0** | undefined | undefined |
| `authoring` (`mixed` or `authored_pure_insert`) | 27 | 14 (51.9%) | 34.0 to 69.3 |
| `no_change` (artifact unchanged) | 5 | 2 (40.0%) | 11.8 to 76.9 |

**The `deletion_only` cell is empty. The primary comparison cannot be made.** All 27 authoring passes were `mixed`; not one was `authored_pure_insert` either. This is not an aggregation artefact: restricting to the 19 windows containing exactly ONE commit gives 19 of 19 `mixed`.

Raw (uncorrected) measurement, for completeness: `mixed` 15/28 = 53.6% [35.8, 70.5], `no_change` 1/4 = 25.0% [4.6, 69.9].

The `authoring` rate of 51.9% [34.0, 69.3] is statistically indistinguishable from the corpus-wide changed-artifact rate of 54.1% [46.9, 61.2], which is the sanity check that the 32-pair located subsample is not wildly unrepresentative on the outcome variable.

The `no_change` arm, n=5, is far too small to say anything. Its point estimate (40.0%) sits above the corpus-wide unchanged rate (14.3%), but the CI spans 11.8 to 76.9 and contains both. Of the 5, one is the round-113 boundary case discussed above and one (round 142) is the "incidental reflows" round the log itself flags; two of five is entirely consistent with noise.

### R2. Token-multiset shape, the definition that matches what the project recorded

Among the 27 located changed fix passes, `new_tokens` (words added that were not already present) is:

    1, 1, 2, 3, 12, 15, 51, 88, 93, 95, 110, 132, 147, 231, 372, 509, 575,
    708, 815, 858, 891, 1249, 1862, 2278, 6683, 7408, 12298

No pass has `new_tokens == 0`, so `zero_new_tokens` is also empty and the strict token version of the primary contrast is undefined too. Sweeping the threshold instead:

| Deletion-class defined as | n | next round `new_valid` | 95% CI | Complement | 95% CI |
| --- | --- | --- | --- | --- | --- |
| `new_tokens <= 0` | 0 | undefined | - | 14/27 = 51.9% | 34.0 to 69.3 |
| `new_tokens <= 1` | 2 | 1 (50.0%) | 9.5 to 90.5 | 13/25 = 52.0% | 33.5 to 70.0 |
| `new_tokens <= 2` | 3 | 1 (33.3%) | 6.1 to 79.2 | 13/24 = 54.2% | 35.1 to 72.1 |
| `new_tokens <= 5` | 4 | 1 (25.0%) | 4.6 to 69.9 | 13/23 = 56.5% | 36.8 to 74.4 |
| `new_tokens <= 50` | 6 | 1 (16.7%) | 3.0 to 56.4 | 13/21 = 61.9% | 40.9 to 79.2 |
| `new_tokens <= 100` | 10 | 3 (30.0%) | 10.8 to 60.3 | 11/17 = 64.7% | 41.3 to 82.7 |

**Every deletion-class cell here is too small to support a conclusion.** At the threshold that best matches the project's own usage (`new_tokens <= 1`, meaning at most one new word, which is the reflow or punctuation artefact), n=2 and the interval runs from 9.5 to 90.5 percent. That is no information at all. Reporting "50 percent" from n=2 would be exactly the miscounting failure this brief warns about.

There is a monotone-looking drift in the table as the threshold widens, but it is driven by the threshold sweeping in more of the size gradient in R4, not by shape.

### R3. Direct check of the ledger's five-for-five claim against the outcome proxy

The ledger states an unbroken five-for-five record that deletion-only fix passes re-seed nothing. Under the pre-registered OUTCOME proxy (was the next round `new_valid`), that record does not hold: `c5b00a7` (= the ledger's `3e4fb6c`), the fix pass the ledger calls "pure deletion at BOTH sites, zero words authored", was followed by round index 197 (log line 224), which is recorded `new_valid` with 2 low findings.

This is NOT a contradiction of the ledger. The ledger's claim is about PROVENANCE (did the fix create the next finding), and its own record of that round says the two findings were PRE-EXISTING. My claim is about the round OUTCOME, which is the pre-registered quantity, and which cannot tell an injected finding from a pre-existing one. The two measurements are of different things and both are correct. It does mean the outcome proxy is a POOR proxy for injection: on the single case where both measurements exist, they disagree in sign. Whatever `P(new_valid | shape)` would have shown, it would not have been the injection rate.

### R4. SECONDARY: does fix-pass SIZE predict the next round's outcome

This is the only pre-registered association that shows a signal. Among the 27 located changed fix passes:

| Split | next round `new_valid` | 95% CI |
| --- | --- | --- |
| `new_tokens` tercile 1 (1 to 93) | 2/9 = 22.2% | 6.3 to 54.7 |
| `new_tokens` tercile 2 (95 to 708) | 5/9 = 55.6% | 26.7 to 81.1 |
| `new_tokens` tercile 3 (815 to 12298) | 7/9 = 77.8% | 45.3 to 93.7 |
| `subst_added` tercile 1 (1 to 14) | 3/9 = 33.3% | 12.1 to 64.6 |
| `subst_added` tercile 2 (17 to 31) | 4/9 = 44.4% | 18.9 to 73.3 |
| `subst_added` tercile 3 (54 to 565) | 7/9 = 77.8% | 45.3 to 93.7 |

Distribution-free, using all 27 rather than binning: median `new_tokens` is 761.5 before a `new_valid` round (n=14) against 88 before a `clean` round (n=13); Mann-Whitney U = 143.5, AUC = 0.788, normal-approximation z = 2.55, two-sided p ~ 0.011. On `subst_added`: medians 44 against 17, AUC = 0.736, p ~ 0.037.

Adjacent terciles' intervals overlap heavily, so the tercile table alone would not support the claim; the rank test is what carries it. With n=27, one exclusion decision reversed could move it.

`subst_deleted` and `files_touched` show weaker and non-monotone patterns (deleted-lines terciles 22.2%, 66.7%, 66.7%; files terciles 44.4%, 44.4%, 66.7%), consistent with size acting through the ADDED side rather than through churn generally.

### R5. SECONDARY: phase

| Phase | located pairs | next round `new_valid` | 95% CI | median `subst_added` | median `new_tokens` |
| --- | --- | --- | --- | --- | --- |
| `plan_review` | 7 | 5 (71.4%) | 35.9 to 91.8 | 24 | 858 |
| `work_review` | 25 | 11 (44.0%) | 26.7 to 62.9 | 27.5 | 139.5 |

The intervals overlap substantially and n=7 for `plan_review` is too small to conclude a phase difference. The measurable difference between the phases is in the SIZE distribution, not the rate: plan-review fix passes touch a similar number of lines but carry roughly six times the new-token count, which is what "editing prose" looks like mechanically. Given R4, that alone could produce the higher plan-review rate without any phase-specific effect. Confounded; not separable at this n.

Shape does not vary by phase at all, because shape does not vary at all: all 7 plan-review and 20 of 25 work-review located pairs are `authoring`, the other 5 being `no_change`.

### R6. SECONDARY: risk class

Using the artifact-level any-round-risky rule:

| Artifact risk | located pairs | next round `new_valid` | 95% CI |
| --- | --- | --- | --- |
| `low_risk` | 10 | 5 (50.0%) | 23.7 to 76.3 |
| `risky` | 22 | 11 (50.0%) | 30.7 to 69.3 |

Identical point estimates. No association detectable, and no shape contrast to test within either class. Restricting to changed pairs: `low_risk` 5/10, `risky` 9/17 = 52.9% [31.0, 73.8]. Same conclusion.

### R7. SECONDARY: artifacts that ran past the cap of 5

Four artifacts exceeded 5 rounds, all four `risky` under the any-round-risky rule: `workflow-driver` (9), `driver-output-generation` (7), `structured-skeleton` (6), `prompt-drift-guard-inc1` (6). This matches the ledger's "four of 22" exactly.

| Group | located changed fix passes | median `subst_added` | median `new_tokens` | median files | next round `new_valid` |
| --- | --- | --- | --- | --- | --- |
| Artifacts with more than 5 rounds | 12 | 30.5 | 189.0 | 1.5 | 6/12 = 50.0% [25.4, 74.6] |
| Artifacts that converged in exactly 2 rounds | 3 | 5 | 110 | 2 | 0/3 = 0.0% [0.0, 56.2] |

**The comparison is not usable.** Only 3 of the 36 two-round artifacts have a locatable fix pass, because two-round loops converge in one commit and so are precisely the loops that fall into the `shared_recording_commit` exclusion. The exclusion mechanism is correlated with the very quantity being compared, which is the worst possible bias for this question. The 0/3 figure should not be quoted. What can be said is directional and weak: the long-running artifacts' fix passes are not obviously enormous (median 30.5 substantive added lines, usually 1 to 2 files), so "long loops are driven by sprawling fixes" is not supported by what little is visible here.

## LIMITS, stated by me

1. **72 percent of pairs are excluded, non-randomly and chronologically.** The included sample is biased toward late history and toward loops long enough that the orchestrator committed each round separately. Short loops are systematically absent. Anything in R7 comparing long to short loops inherits this directly.
2. **The primary contrast has an empty arm and therefore no result.** No amount of interpretation makes n=0 informative. If the hypothesis is to be tested at all, it needs either a different unit (per-FINDING fix rather than per-PASS) or a different predicate (token-level), and both need more located passes than 32.
3. **The outcome proxy is not the injection rate.** `new_valid` counts findings the round raised, whether the preceding fix created them, they pre-existed, or the reviewer simply looked somewhere new. R3 gives a concrete case where the two diverge. My evidence supports statements about ASSOCIATION between fix-pass shape or size and the next round's outcome, and supports nothing at all about causation. Separating injection from new content is exactly what a per-finding provenance pass over the triage files must do, and it cannot be done from git diffs.
4. **The window is time-based and cannot separate simultaneous loops.** 5 of 32 included pairs share commits with another episode's window. Their measured sizes are over-counted by an unknown amount.
5. **The window boundary leaks post-review fixes backwards.** Demonstrated once (round 113) and detectable only because the oracle disagreed. The undetectable version of the same error inflates the measured size of some passes on changed artifacts.
6. **Artifact paths are global, not per-artifact.** I do not know mechanically which files a given review was actually looking at, so any artifact-path commit in the window counts. On a serialized single-loop period this is right; during the 2026-07-28 interleave it is not.
7. **Size and phase are confounded** (R5), and n=27 cannot separate them. The R4 rank test is computed over pooled phases.
8. **Multiplicity.** Five pre-registered analyses were run (shape, size, phase, risk class, past-cap). One of the five reached conventional significance (size, p ~ 0.011). Under a Bonferroni correction at five tests the threshold is 0.01, which it does not clear. Treat R4 as suggestive, not established.
9. **`new_tokens` uses whitespace tokenisation and a net multiset difference over the whole window.** A pass that deletes the word "foo" in one file and adds it in another nets to zero for that token. At the observed magnitudes this is immaterial, but it is not a per-site measure.
10. **I did not read any reviewer or triage file to derive any number here**, by design. Where I quote the ledger (R3, and the `3e4fb6c` spot check) it is as an independent oracle to check my derivation against, not as an input to it.

## Spot checks, re-derived from source

- Pair 196 -> 197, window commit `c5b00a7`. Pipeline says `subst_added=2, subst_deleted=3, new_tokens=1, files_touched=1, code_noncomment_added=0`. Hand check: `git show -U0 c5b00a7 -- src/agents_md_drift.rs` shows exactly 3 removed and 2 added `//!` doc-comment lines in one file, one hunk. The single new token is `self-extending.` replacing `self-extending:`. The ledger's independent hand-measurement of the same commit (recorded before this analysis existed) says "only added token `self-extending.` displacing `self-extending:`". Agreement, exact.
- Pair 197 -> 198, window commit `a48d8df`. Pipeline says `+1/-1, new_tokens=1`. Hand check: one `//!` line, the word `asset` inserted. Agreement.
- Round 113's window commit `ce0d36c`: `flake.nix`, 5 insertions 1 deletion; pipeline reports `subst_added=5, subst_deleted=1`. Agreement with `git show --stat` (the pipeline's substantive filter drops nothing here).
- Commit reachability: `3e4fb6c` is contained in no branch and absent from `git log --all`, so the rebase duplicate is not double-counted. `339d26a` is contained only in `impl/checks-collision` and `triage/checks-collision-r4`, confirming that the step-93 fix passes are reachable ONLY via `--all` and would be missed by a main-line analysis.
- Round index 197 in this record is file line 224 of `workflow.jsonl`, which is how the ledger numbers records. Verified by counting: the ledger numbers ALL records, this record numbers only `type: "round"` records. Anyone comparing a "record N" in the ledger to a "round N" here must convert.
- The 32 included windows span 41 distinct commits. Every one was scanned for exploration-file content; only `cffde3a0` carries any (M4).

## EXPLORATORY (NOT pre-registered)

Flagged clearly: none of the following was decided before looking at the data, and each was prompted by something the pre-registered questions ran into.

**E1. The pre-registered shape predicate does not exist in this corpus, and that is itself the most useful finding.** Zero pure-deletion fix passes in 32 located passes, and zero zero-new-token passes in 27 changed ones. The smallest observed pass authored one new word. If the workflow wants a "deletion-class fix pass" category with a testable rate attached, the category has to be defined at the level of an individual FIX (one finding's remedy), not at the level of a PASS (everything the implementer did between two reviews), because passes bundle several fixes and one authored fix contaminates the whole pass. Step 93's own escalation illustrates this: its fix set is described as "two of three pure deletions", which is a per-fix count and has no fix-pass-level equivalent.

**E2. A rate that does exist, at the artifact level, using all 79 artifacts and no exclusions.** Rounds are not equally productive across loop lengths:

| Artifact length | artifacts | rounds | rounds `new_valid` | rate | findings |
| --- | --- | --- | --- | --- | --- |
| 1 round | 16 | 16 | 6 | 38% | 26 |
| 2 rounds | 36 | 72 | 34 | 47% | 159 |
| 3 rounds | 10 | 30 | 13 | 43% | 50 |
| 4 to 5 rounds | 13 | 58 | 34 | 59% | 142 |
| more than 5 rounds | 4 | 28 | 15 | 54% | 42 |

The rate does not decay with loop length. Long loops are not grinding out an exhausted artifact; round 6 of a long loop is about as likely to find something as round 2 of a short one. This uses all 204 rounds and no fix-pass location, so it carries none of the exclusion bias above. It is the strongest evidence in this record that the cap of 5 sits below where the distribution still has mass, and it is consistent with the ledger's existing conclusion, but it says nothing about WHY (the injection-versus-new-content question is untouched by it).

**E3. Fix-pass size collapses across a loop's life while the finding rate does not.** All 12 located changed fix passes inside the four artifacts that ran past the cap, in loop order, as `new_tokens` -> next round's outcome:

- `structured-skeleton`: 12298 -> `new_valid`, 372 -> `clean`.
- `workflow-driver`: 509 -> `new_valid`, 231 -> `clean`.
- `driver-output-generation`: 147 -> `clean`, 3 -> `clean`, 15 -> `clean`.
- `prompt-drift-guard-inc1`: 575 -> `new_valid`, 6683 -> `new_valid`, 95 -> `new_valid`, 1 -> `new_valid`, 1 -> `clean`.

Fix passes shrink by three to four orders of magnitude as a loop proceeds (one exception, prompt-drift-guard's second pass, which grew). Yet `prompt-drift-guard-inc1` returned `new_valid` on rounds 4 and 5 after fix passes of 95 and 1 new tokens respectively. If the R4 size association were causal in the INJECTION direction, a collapsing fix-pass size should drag the `new_valid` rate down along a loop, and in the one artifact where the collapse is starkest it does not. Combined with E2, that is a point AGAINST injection being the dominant mechanism. It rests on 12 fix passes across 4 artifacts, is far too little to conclude anything, and is offered only as a direction for the interpretive pass to check against the triage files, where the provenance of those late findings is recorded.

**E4. Recoverable coverage, if someone wants the excluded 83 back.** The 83 excluded pairs are excluded because their rounds share a recording commit, not because their fix passes are missing from git. In the bundled windows the intervening fix commits ARE individually present (for example `67ba852..0af1bcb` contains one build commit and four fix commits against five bundled rounds). Aligning them requires assuming one commit per pass, which is a guess I declined to make. A per-finding extraction from the triage files, which is what explorer B and the interpretive pass are doing, can date each round from the reviewer-file commits in later history and would not need that assumption.
