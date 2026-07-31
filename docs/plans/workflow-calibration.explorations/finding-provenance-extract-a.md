# Finding-provenance extraction, slice A (interpretive pass over archived triage records)

Explorer: extractor A, `workflow-calibration` step. Worktree `.claude/worktrees/cal-int-a`, branch `explore/cal-int-a`, at `d916def`. Read-only with respect to product code, the plan, the ledger and the metrics log; the only files written are this record and `finding-provenance-a.tsv` beside it.

Companion dataset: `docs/plans/workflow-calibration.explorations/finding-provenance-a.tsv`, 221 data rows, one row per finding as ruled by a triager.

This record reports counts. It does not compute an injection rate and it draws no conclusion about the INJECTION versus NEW CONTENT hypotheses. A separate analyst does that.

## METHOD

Scripts referenced below live in the session scratchpad at `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/int-a/`. They are reproducible from this worktree.

### 1. The full set of review-directory files that have ever existed: 343

```
git log --diff-filter=D --name-only --pretty=format: -- 'docs/plans/*.reviews/*' 'docs/plans/*review*' 'docs/plans/*triage*' | grep -v '^$' | sort -u   ->  296
git ls-files 'docs/plans/*.reviews/*' | sort -u                                                                                                        ->   49
cat deleted.txt live.txt | sort -u                                                                                                                     ->  343
```

296 deleted plus 49 live is 345; two paths appear in both sets (files deleted and later recreated), so the union is 343. This matches the expected count.

### 2. Filter to `triage` in the path, case-insensitive: 89

```
grep -i 'triage' all343.txt | sort -u | wc -l   ->  89
```

This matches the expected count. Note the filter admits six files that are REVIEWER files, not triage files, because the artifact slug itself contains the substring `triager`:

- `docs/plans/agent-scaffold.reviews/triager-independence-reviewer-opus.md`
- `docs/plans/agent-scaffold.reviews/triager-independence-reviewer-sonnet.md`
- `docs/plans/agent-scaffold.reviews/triager-independence-round2-reviewer.md`
- `docs/plans/agent-scaffold.reviews/triager-on-findings-reviewer-a.md`
- `docs/plans/agent-scaffold.reviews/triager-on-findings-reviewer-b.md`
- `docs/plans/agent-scaffold.reviews/triager-on-findings-reviewer-r2.md`

Three of the six fall in my slice (indices 58, 62, 64). I kept them in the derived corpus, because the derivation rule is fixed and reproducible and changing it would change the 89, but I extracted ZERO rows from them, because the stated discipline is to extract what was RULED and not what was CLAIMED. They are listed in the slice table below with a note. Anyone recomputing the corpus should expect the same six.

### 3. Deterministic split of the sorted 89

Index is 0-based over the sorted list. `index mod 5 == 0` is the shared overlap; of the remainder, even index is mine.

```
awk '{i=NR-1; if (i%5==0) print "overlap"; else if (i%2==0) print "mine"; else print "other"}' triage89.txt
overlap 18   mine 36   other 35   (18 + 36 + 35 = 89)
```

My slice is 18 + 36 = 54 files, as specified. The exact list, with the split index and group:

| idx | group | path |
| --- | --- | --- |
| 0 | overlap | `docs/plans/agent-scaffold.reviews/agent-isolation-triage.md` |
| 2 | mine | `docs/plans/agent-scaffold.reviews/backlog-promotion-triage.md` |
| 4 | mine | `docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-r3-triage.md` |
| 5 | overlap | `docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage.md` |
| 6 | mine | `docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage-r3.md` |
| 8 | mine | `docs/plans/agent-scaffold.reviews/code-value-audit-static-inc1-round2-triage.md` |
| 10 | overlap | `docs/plans/agent-scaffold.reviews/code-value-audit-static-inc2-triage.md` |
| 12 | mine | `docs/plans/agent-scaffold.reviews/consolidate-triage.md` |
| 14 | mine | `docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-r3-triage.md` |
| 15 | overlap | `docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-r4-triage.md` |
| 16 | mine | `docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-r5-triage.md` |
| 18 | mine | `docs/plans/agent-scaffold.reviews/decision-folder-currency-r2-triage.md` |
| 20 | overlap | `docs/plans/agent-scaffold.reviews/decision-fold-triage.md` |
| 22 | mine | `docs/plans/agent-scaffold.reviews/doc-currency-guidance-triage.md` |
| 24 | mine | `docs/plans/agent-scaffold.reviews/driver-output-generation-inc2-round3-triage.md` |
| 25 | overlap | `docs/plans/agent-scaffold.reviews/driver-output-generation-inc2-triage.md` |
| 26 | mine | `docs/plans/agent-scaffold.reviews/exploration-mode-triage.md` |
| 28 | mine | `docs/plans/agent-scaffold.reviews/file-safety-rules-triage.md` |
| 30 | overlap | `docs/plans/agent-scaffold.reviews/gate-prompt-clarity-triage.md` |
| 32 | mine | `docs/plans/agent-scaffold.reviews/human-review-queue-triage.md` |
| 34 | mine | `docs/plans/agent-scaffold.reviews/ledger-template-triage.md` |
| 35 | overlap | `docs/plans/agent-scaffold.reviews/lifecycle-capture-triage.md` |
| 36 | mine | `docs/plans/agent-scaffold.reviews/metrics-fields-triage.md` |
| 38 | mine | `docs/plans/agent-scaffold.reviews/optional-modules-1-triage.md` |
| 40 | overlap | `docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md` |
| 42 | mine | `docs/plans/agent-scaffold.reviews/prompt-drift-guard-r4-triage.md` |
| 44 | mine | `docs/plans/agent-scaffold.reviews/prompt-drift-guard-verify-triage.md` |
| 45 | overlap | `docs/plans/agent-scaffold.reviews/q59-backlog-fold-triage.md` |
| 46 | mine | `docs/plans/agent-scaffold.reviews/q66-q67-plan-triage.md` |
| 48 | mine | `docs/plans/agent-scaffold.reviews/round-log-core-B-triage.md` |
| 50 | overlap | `docs/plans/agent-scaffold.reviews/state-schema-1-triage.md` |
| 52 | mine | `docs/plans/agent-scaffold.reviews/state-schema-3-triage.md` |
| 54 | mine | `docs/plans/agent-scaffold.reviews/step89-triage.md` |
| 55 | overlap | `docs/plans/agent-scaffold.reviews/task-entry-regrounding-inc1-triage.md` |
| 56 | mine | `docs/plans/agent-scaffold.reviews/task-entry-regrounding-inc2-triage.md` |
| 58 | mine | `docs/plans/agent-scaffold.reviews/triager-independence-reviewer-sonnet.md` (REVIEWER file, zero rows) |
| 60 | overlap | `docs/plans/agent-scaffold.reviews/triager-independence-round2-triage.md` |
| 62 | mine | `docs/plans/agent-scaffold.reviews/triager-on-findings-reviewer-a.md` (REVIEWER file, zero rows) |
| 64 | mine | `docs/plans/agent-scaffold.reviews/triager-on-findings-reviewer-r2.md` (REVIEWER file, zero rows) |
| 65 | overlap | `docs/plans/agent-scaffold.reviews/triager-on-findings-triage.md` |
| 66 | mine | `docs/plans/agent-scaffold.reviews/uniform-agent-isolation-r2-triage.md` |
| 68 | mine | `docs/plans/agent-scaffold.reviews/user-prompts-dir-triage.md` |
| 70 | overlap | `docs/plans/agent-scaffold.reviews/workflow-driver-stage1-triage.md` |
| 72 | mine | `docs/plans/optional-modules-2a.reviews/optional-modules-2a-triage.md` |
| 74 | mine | `docs/plans/optional-modules-2cii.reviews/optional-modules-2cii-triage.md` |
| 75 | overlap | `docs/plans/optional-modules-2ci.reviews/optional-modules-2ci-triage.md` |
| 76 | mine | `docs/plans/optional-modules-inc3.reviews/optional-modules-inc3-triage.md` |
| 78 | mine | `docs/plans/reviewer-diversity.reviews/reviewer-diversity-triage.md` |
| 80 | overlap | `docs/plans/structured-skeleton.reviews/inc1-triage.md` |
| 82 | mine | `docs/plans/structured-skeleton.reviews/inc3-triage.md` |
| 84 | mine | `docs/plans/structured-skeleton.reviews/inc6-docsync-plan-triage.md` |
| 85 | overlap | `docs/plans/structured-skeleton.reviews/inc6-r3-triage.md` |
| 86 | mine | `docs/plans/structured-skeleton.reviews/inc6-triage.md` |
| 88 | mine | `docs/plans/workflow-invariants.reviews/triage.md` |

### 4. Retrieval

47 of the 54 are deleted; 7 are live. `fetch.sh` copies live files and, for deleted ones, resolves the deletion commit with `git log --diff-filter=D --format=%H -1 -- <path>` and reads `git show <commit>^:<path>`. All 54 were retrieved with no failure (`fetch-log.txt`: 47 `DEL`, 7 `LIVE`, 0 `MISSING`, 0 `FAIL`). Every file was then read in full; nothing was skimmed or sampled.

### 5. The oracle

`docs/metrics/workflow.jsonl` holds 231 records: 204 `round`, 22 `decision`, 5 `escalation`. Round records carry no explicit round number, so round number is derived as the ordinal position within a `(task, phase)` group in file order (`oracle2.jq`). This model is corroborated by the artifact strings, which frequently name the round ("after the round-2 fix pass") and agree with the derived ordinal in every case I checked.

Totals over all 204 rounds: 102 `clean`, 102 `new_valid`, 411 valid findings.

### 6. Extraction rules I applied

- One row per finding as the TRIAGER ids it. Where a triage merges reviewer findings under one triage id, that is one row. Where a triage SPLITS one reviewer finding into sub-claims with different verdicts (`AD1` -> `AD1a`/`AD1b`; `I1-2` -> sub-claims A and B; `O5` -> self-reference and cycle detection; `G` -> doc facet and perf facet), each sub-claim is its own row, because each carries its own verdict.
- Findings that a later round CONFIRMS CLOSED are not rows. They are prior rounds' findings, already counted there.
- `verdict` maps as: `VALID`/`VALID (fix required)` -> `valid`; `VALID BUT ACCEPT RESIDUAL`, `VALID-BUT-DEFER`, `VALID-NON-ACTIONABLE`, `ACCEPTABLE residual`, `accepted non-blocking residual`, `valid-as-observation, out of scope` -> `accept_residual`; `INVALID`, `DISMISSED`, `WONTFIX`, `not a defect` -> `dismissed`.
- `severity_raised` and `severity_ruled` record what the triage says, including its own non-standard tokens (`nit`, `very low`, `low-to-medium`) and including compound forms when two reviewers rated differently (`low/medium`). Normalisation happens only inside the reconciliation script, never in the TSV.
- `class`: `prose` is documentation or comment content, including Rust doc comments, commit messages and plan/step prose; `code` is executable behaviour; `test` is test code or test coverage; `config` is structured data files (`plan.toml`, `workflow.jsonl`, `pack.toml`, `flake.nix`).
- `provenance` is `unstated` unless the triage states the causation in words I could quote. My two positive labels are defined as:
  - `introduced_by_prior_fix`: the triage explicitly attributes the defect to an earlier round's fix pass or fix commit.
  - `pre_existing`: the triage explicitly states the defect existed before the artifact or change under review, or is inherited from an upstream artifact (the spec, the build plan, an earlier step).
  Every row with a positive label carries a verbatim `provenance_quote`. Rows with `unstated` carry an empty quote. 30 rows carry a quote (24 of them counted, plus 6 on dismissed rows).
- `escaped` is `yes` only where the triage explicitly says an earlier round had the defect available and did not raise it; `no` only where the triage explicitly says the defect did not exist earlier or was not previously available; `unknown` on silence.

## RECONCILIATION AGAINST THE JSONL ORACLE

51 of my 54 files are triage files and each maps to exactly one round record. The mapping is asserted in `reconcile.py` and was established by reading each triage file's own header (task, commit range, stated round) and matching it to the round record's `task`, `phase` and `artifact` text. Full output is in `reconcile.out`.

Comparison rule: my per-round count is the number of rows with `verdict` in {`valid`, `accept_residual`}; my severity multiset is those rows' `severity_ruled`, with `nit` and `very low` normalised to `low` and `low-to-medium` to `medium` for comparison only.

**43 MATCH, 8 MISMATCH out of 51.**

### The 8 mismatches, by round identity

| triage file | oracle round record | task / phase / round | outcome | oracle `valid_findings` | my counted rows | delta |
| --- | --- | --- | --- | --- | --- | --- |
| `optional-modules-2a-triage.md` | line 69 | `optional-modules-inc2a` work_review R1 | new_valid | 9 (3 medium, 6 low) | 11 (3 medium, 8 low) | +2 low |
| `workflow-driver-stage1-triage.md` | line 123 | `workflow-driver` work_review R6 (stage 1 round 1) | new_valid | 2 (2 medium) | 4 (2 medium, 2 low) | +2 low |
| `task-entry-regrounding-inc2-triage.md` | line 129 | `task-entry-regrounding` work_review R3 (inc2 round 1) | new_valid | 2 (medium, low) | 3 (medium, 2 low) | +1 low |
| `driver-output-generation-inc2-triage.md` | line 140 | `driver-output-generation` work_review R4 (inc2 round 1) | new_valid | 1 (low) | 2 (2 low) | +1 low |
| `lifecycle-capture-triage.md` | line 144 | `lifecycle-capture` work_review R1 | clean | 0 | 2 (2 low) | +2 low |
| `uniform-agent-isolation-r2-triage.md` | line 146 | `uniform-isolation` work_review R2 | clean | 0 | 2 (2 low) | +2 low |
| `q59-backlog-fold-triage.md` | line 148 | `q59-backlog-fold` work_review R1 | clean | 0 | 1 (low) | +1 low |
| `doc-currency-guidance-triage.md` | line 149 | `doc-currency-guidance` work_review R1 | clean | 0 | 1 (low) | +1 low |

All eight run the same direction: I have MORE rows than the log records, and in every case the surplus rows are findings the triager ruled valid but resolved without a fix in that round (`VALID BUT ACCEPT RESIDUAL`, `VALID-BUT-DEFER`, `VALID-NON-ACTIONABLE`, `ACCEPTABLE`, `accepted non-blocking residual`, orchestrator-owned or next-increment-owned items).

The log is NOT consistent about this. Accepted residuals ARE counted in `valid_findings` elsewhere in my slice:

- `decision-folder-currency-fold` plan_review R5 (line 188): outcome `clean`, `valid_findings: 1`, and the only finding (`R5-1`) is `VALID BUT ACCEPT RESIDUAL`.
- `planner-folds-decisions-inc1` work_review R1 (line 183): outcome `clean`, `valid_findings: 1`, and the only finding (`F1`) is `VALID-BUT-ACCEPT-RESIDUAL`.
- `checks-runner-worktree-name-collision-inc1` work_review R1 (line 201): `valid_findings: 8`, of which four (`T4`, `T5`, `T7`, `T10`) are `VALID BUT ACCEPT RESIDUAL`.
- `optional-modules-inc2ci` work_review R1 (line 74) and `optional-modules-inc2cii` work_review R1 (line 78) both count their deferred and residual items.

So `valid_findings` in the round log does not have a single stable meaning across the 51 rounds I extracted. In 43 rounds it equals "findings the triager ruled valid, including accepted residuals"; in 8 it equals "findings that required a fix this round". I am reporting this as a fact about the instrument, not resolving it.

### Severity-vocabulary mismatches (count agrees, token differs)

Four rows use a token the log's `severities` array does not carry. In each case the log records `low` (or, for the hedged one, `medium`). The TSV keeps the triage's own token.

| triage file | finding | triage token | oracle token |
| --- | --- | --- | --- |
| `code-value-audit-static-inc2-triage.md` | non-numbered stale-comment note | `nit` | `low` |
| `exploration-mode-triage.md` | `V6` | `nit` | `low` |
| `optional-modules-2cii-triage.md` | `opus F4` | `very low` | `low` |
| `task-entry-regrounding-inc1-triage.md` | `I1-1` | `low-to-medium` (the triage declines to pick) | `medium` |

### One internal disagreement inside a triage file that the log resolves against the triage

`structured-skeleton.reviews/inc1-triage.md` reconciles at 11 findings (2 medium, 9 low) against oracle line 98, but only if `O4` (`deny_unknown_fields`, deferred to a human decision) is counted. The triage's own Tally section says "medium: 2, low: 8", ten in total, and explicitly excludes `O4` ("Deferred (human decides, not counted above)"). The log counted it. I include `O4` as an `accept_residual` row, so my rows reconcile with the log and disagree with the triage's own summary line by one. Recorded as stated in both sources; not corrected.

## SHORTFALL ROUNDS

The framing "89 triage files against 102 `new_valid` rounds" needs an adjustment before the shortfall can be read off it: triage files are also written for CLEAN rounds. Eight of my 51 triage files map to a round the log records as `clean` (`decision-fold-q60-q62` R1, `doc-currency-guidance` R1, `lifecycle-capture` R1, `q59-backlog-fold` R1, `planner-folds-decisions-inc1` R1, `decision-folder-currency-fold` R5, `triager-independence` R2, `uniform-isolation` R2). So only 43 of my 51 files cover a `new_valid` round, and the corpus-wide shortfall is larger than 13.

Restricted to the 42 tasks my slice touches, the log records 65 `new_valid` rounds. Ten of them have NO triage file anywhere in the 89-file corpus. Identified by matching each round record against the full 343-file list, not only the 89:

| oracle line | task | phase | round | `valid_findings` | severities | reviewer files that DO survive |
| --- | --- | --- | --- | --- | --- | --- |
| 75 | `optional-modules-inc2ci` | work_review | R2 | 1 | medium | `optional-modules-2ci-round2-reviewer-opus.md`, `-round2-reviewer-sonnet.md` |
| 79 | `optional-modules-inc2cii` | work_review | R2 | 2 | high, low | `optional-modules-2cii-round2-reviewer-opus.md`, `-round2-reviewer-sonnet.md` |
| 80 | `optional-modules-inc2cii` | work_review | R3 | 1 | high | `optional-modules-2cii-round3-reviewer.md` |
| 81 | `optional-modules-inc2cii` | work_review | R4 | 1 | low | `optional-modules-2cii-round4-reviewer.md` |
| 94 | `structured-skeleton` | plan_review | R1 | 11 | high, 3 medium, 7 low | `structured-skeleton-plan.reviews/reviewer-opus.md`, `-sonnet.md`, `confirm-reviewer.md` |
| 101 | `structured-skeleton-inc3` | work_review | R2 | 4 | 4 low | `inc3-round2-reviewer-opus.md`, `inc3-round2-reviewer-sonnet.md` |
| 102 | `structured-skeleton-inc3` | work_review | R3 | 1 | low | `inc3-round3-reviewer.md` |
| 116 | `workflow-driver` | work_review | R1 (stage 0a) | 3 | 3 low | none under this name |
| 191 | `checks-runner-worktree-name-collision` | plan_review | R1 | 5 | medium, 4 low | `checks-runner-worktree-name-collision-reviewer.md` |
| 202 | `checks-runner-worktree-name-collision-inc1` | work_review | R2 | 8 | 3 medium, 5 low | `-r2-reviewer-adversarial.md`, `-r2-reviewer-verification.md` |

Two observations I am reporting rather than absorbing:

1. None of these ten is explained by the two known historical causes. `Q-14` (rounds predating the findings-files convention) and `Q-63` (early rounds collapsing triage into the producer or orchestrator) both concern the EARLIEST rounds. These ten sit at oracle lines 75 to 202 out of 204, that is across the middle and the very end of the log, including step 93 (`checks-runner-worktree-name-collision`), which is among the last work in the log. In eight of the ten the reviewer files for the same round DO survive in the 343, so the round ran a normal reviewer pass and its triage file specifically is absent from history.
2. `optional-modules-inc2cii` has four consecutive `new_valid` rounds (R1 to R4) with a triage file for R1 only. Rounds R2 and R3 both carry a `high`. Those are the only two `high` severities in my slice's shortfall set, and no triager ruling on them survives.

One naming inference underpins two rows of the shortfall table, and I checked it rather than assuming it. The `checks-runner-worktree-name-collision` family uses two naming conventions in the same directory: `-rN-triage.md` for PLAN review round N, and `-triage-rN.md` for INC1 WORK review round N. I confirmed this by reading the header of `checks-runner-worktree-name-collision-r2-triage.md` (not in my slice, read only to resolve the ambiguity): "# Triage, plan review round 2". So the plan review has triage files for R2, R3, R4 but not R1, and the inc1 work review has them for R1, R3, R4 but not R2.

## DESCRIPTIVE COUNTS

221 rows over 50 files (54 slice files, minus 3 reviewer files that yield no rows, minus `decision-fold-triage.md` where both reviewers reported zero findings so there was nothing to adjudicate).

### Verdicts

| verdict | rows |
| --- | --- |
| `valid` | 156 |
| `accept_residual` | 41 |
| `dismissed` | 24 |
| total | 221 |

Counted rows (`valid` + `accept_residual`): 197.

### Class (what the finding is about)

| class | rows | share |
| --- | --- | --- |
| `prose` | 126 | 57.0% |
| `code` | 56 | 25.3% |
| `test` | 34 | 15.4% |
| `config` | 5 | 2.3% |
| `unknown` | 0 | 0% |

### Provenance, all 221 rows

| provenance | rows | share |
| --- | --- | --- |
| `unstated` | 191 | 86.4% |
| `pre_existing` | 16 | 7.2% |
| `introduced_by_prior_fix` | 14 | 6.3% |

### Provenance, counted rows only (197)

| provenance | rows | share |
| --- | --- | --- |
| `unstated` | 173 | 87.8% |
| `introduced_by_prior_fix` | 13 | 6.6% |
| `pre_existing` | 11 | 5.6% |

20 of the 50 contributing files carry at least one non-`unstated` provenance. 30 files carry none.

### Class by provenance (all 221 rows)

| class | `introduced_by_prior_fix` | `pre_existing` | `unstated` | total |
| --- | --- | --- | --- | --- |
| `prose` | 13 | 13 | 100 | 126 |
| `code` | 0 | 2 | 54 | 56 |
| `test` | 1 | 1 | 32 | 34 |
| `config` | 0 | 0 | 5 | 5 |
| total | 14 | 16 | 191 | 221 |

Every `introduced_by_prior_fix` row but one is `prose`. The single non-prose one is `AD2` in `checks-runner-worktree-name-collision-triage-r3.md`, a test assertion the round-2 fix pass added.

### Verdict by provenance (all 221 rows)

| verdict | `introduced_by_prior_fix` | `pre_existing` | `unstated` | total |
| --- | --- | --- | --- | --- |
| `valid` | 12 | 8 | 136 | 156 |
| `accept_residual` | 1 | 3 | 37 | 41 |
| `dismissed` | 1 | 5 | 18 | 24 |

### Severity as ruled by the triager

| `severity_ruled` | rows |
| --- | --- |
| `critical` | 1 |
| `high` | 2 |
| `medium` | 37 |
| `low` | 160 |
| `nit` | 2 |
| `very low` | 1 |
| `low-to-medium` | 1 |
| `none` (dismissed, no severity carried) | 17 |

### Severity as raised by the reviewer, where the triage states it

| `severity_raised` | rows |
| --- | --- |
| `high` | 2 |
| `medium/high` | 1 |
| `medium` | 39 |
| `low/medium` | 6 |
| `low` | 100 |
| `nit` | 1 |
| `unstated` | 72 |

The triage states the reviewer's rating for 149 of 221 rows (67.4%).

### `escaped`

| `escaped` | rows |
| --- | --- |
| `unknown` | 207 |
| `no` | 9 |
| `yes` | 5 |

The five `yes` rows: `H4-1`, `H4-4`, `H4-5` (`decision-folder-currency-plan-r4-triage.md`), `RD4-1` (`prompt-drift-guard-r4-triage.md`), `RD-V1` (`prompt-drift-guard-verify-triage.md`).

### The 14 `introduced_by_prior_fix` rows, listed

| file | finding | fix round blamed | verdict |
| --- | --- | --- | --- |
| `checks-runner-worktree-name-collision-r3-triage.md` | `T3-1` | 1 and 2 | valid |
| `checks-runner-worktree-name-collision-r3-triage.md` | `T3-2` | 2 | valid |
| `checks-runner-worktree-name-collision-triage-r3.md` | `AD2` | 2 | valid |
| `checks-runner-worktree-name-collision-triage-r3.md` | `AD3` | 2 | valid |
| `decision-folder-currency-plan-r3-triage.md` | `DEC-1` | 2 | valid |
| `decision-folder-currency-plan-r3-triage.md` | `R3-2` | 2 | valid |
| `decision-folder-currency-plan-r4-triage.md` | `R4-1` | 3 | valid |
| `decision-folder-currency-plan-r4-triage.md` | `R4-2` | 3 | valid |
| `decision-folder-currency-plan-r4-triage.md` | `R4-3` | 3 | valid |
| `decision-folder-currency-plan-r5-triage.md` | `R5-1` | 4 | accept_residual |
| `prompt-drift-guard-r2-triage.md` | `V2-1` | 1 | valid |
| `prompt-drift-guard-r2-triage.md` | `A2-2` | 1 | valid |
| `prompt-drift-guard-r4-triage.md` | `RD4-1` | 2 | valid |
| `triager-independence-round2-triage.md` | `T1` | 1 (the Group B fix) | dismissed |

They come from only 6 distinct tasks, and 13 of the 14 are round 2 or later of a loop. `triager-independence` `T1` is the one case where a triager recorded fix-induced provenance and DISMISSED the finding anyway.

### The 16 `pre_existing` rows, listed

`agent-isolation-triage.md` `R1`; `decision-folder-currency-plan-r4-triage.md` `H4-1`, `H4-4`, `H4-5`; `decision-folder-currency-r2-triage.md` coldread `F1`; `file-safety-rules-triage.md` Group B; `ledger-template-triage.md` `V2`; `lifecycle-capture-triage.md` pre-existing note; `metrics-fields-triage.md` `D-2`; `optional-modules-2a-triage.md` `E`; `prompt-drift-guard-verify-triage.md` `RD-V1`, `RD-V2`; `step89-triage.md` `F1`; `task-entry-regrounding-inc1-triage.md` `I1-2` sub-claim A; `task-entry-regrounding-inc2-triage.md` `I2-2`; `uniform-agent-isolation-r2-triage.md` Finding 4.

## AMBIGUITIES I HAD TO JUDGE, AND THE RULE FOR EACH

1. **A third causation category the schema cannot hold.** The most common EXPLICIT causation ruling in this corpus is neither of the two positive schema values. Triagers repeatedly rule, in quotable words, that a defect is NEW CONTENT of the artifact under review: not pre-existing, and (in a round-1 loop) not attributable to any prior fix. I found 14 such rulings and recorded every one as `provenance = unstated`, because the schema has no value for them. They are:
   - `gate-prompt-clarity-triage.md` `R1`: "It is in scope for this step (the change introduced it)."
   - `gate-prompt-clarity-triage.md` `S1`: "so the finding is a genuine coherence gap the change created, not a pre-existing one."
   - `human-review-queue-triage.md` `R2`: "This incompleteness is introduced by this change ..., so it is a new-valid finding rather than pre-existing."
   - `human-review-queue-triage.md` `S3`: "All three phrasings were authored in this same change ... a real, self-introduced consistency gap."
   - `decision-folder-currency-r2-triage.md` coldread `F2`: "the routing itself is new in these three clauses, so the incompleteness arrives with the change."
   - `driver-output-generation-inc2-triage.md` `D2s-1`: "they are committed in 36ed42a."
   - `prompt-drift-guard-r2-triage.md` `A2-1`: "The header text is NOT inherited ... the overclaim is text this artifact wrote."
   - `state-schema-1-triage.md` `F-1`: "Note the diff regressed here."
   - `task-entry-regrounding-inc1-triage.md` `I1-1`: "The implementer went beyond the plan by adding the four-element list."
   - `task-entry-regrounding-inc2-triage.md` `I2-1`: "The increment newly introduced this false rationale."
   - `uniform-agent-isolation-r2-triage.md` Finding 1: "not a pre-existing unrelated gap: before this change a read-only agent did not hit tier 3 at all."
   - `uniform-agent-isolation-r2-triage.md` Finding 2 (via the round outcome): "VALID residuals of this change's own goal, not pre-existing unrelated gaps."
   - `structured-skeleton.reviews/inc6-triage.md` `M-1`: "so this is a regression, not a pre-existing soft-skip."
   - `code-value-audit-static-inc2-triage.md` stale-comment note: "inc2's behavior change is what staled these comments."
   Recorded as `unstated` per the rule. If the forward instrumentation this step designs uses a three-value vocabulary, it will silently discard the category triagers most often record.

2. **"Pre-existing" said about the SITE versus about the DEFECT.** Several triages use the word "pre-existing" for the text or the drift while ruling that the DEFECT is created by the change. My rule: the label attaches to the defect, not to the site. Cases decided against a positive label on this rule:
   - `driver-output-generation-inc2-triage.md` `D2s-1`: "Verified the reflows are a fmt 'fix' of PRE-EXISTING drift". The drift pre-dates; the committed out-of-scope hunks do not. Recorded `unstated`.
   - `round-log-core-B-triage.md` `T1`: the offending template comment is "not touched by the increment", but the triage rules the defect "an internal contradiction inside B's own deliverable". Recorded `unstated`.
   - `triager-on-findings-triage.md` `F1`: line 69 is a "leftover always-run implication" but the contradiction is created by the change. Recorded `unstated`.
   The clearest case of a triager separating the two is `prompt-drift-guard-r2-triage.md` `V2-1`, which explicitly says the false sentence is inherited from step 80 AND that "`V2-1` did not exist before the fix; the fix manufactured it." I took the second, because it is the ruling about the finding.

3. **"Inherited from the design" counted as `pre_existing`.** Two rows are labelled `pre_existing` on a statement that the defect came from an upstream artifact rather than from the codebase's prior state: `file-safety-rules-triage.md` Group B ("The gap originates in the decided spec wording") and `task-entry-regrounding-inc1-triage.md` `I1-2` sub-claim A ("the instrumentation-conditional gap is inherited from the design, not an implementer deviation", written under a heading the triager itself titled "Note on provenance of the defect"). Both are explicit provenance rulings, but they point at the spec rather than at the tree. An analyst who wants tree-provenance only should exclude these two.

4. **A near-miss I ruled `unstated` and want on the record.** `code-value-audit-static-inc1-round2-triage.md` `F1` says "a genuine dangling reference the rename left behind", where the rename is "the CORR-3 rename". The same file says "All five round-1 fixes confirmed closed". `CORR-3` looks like a round-1 correctness finding, which would make this an `introduced_by_prior_fix` case, and the round-1 log record does carry 5 findings. But the triage never says `CORR-3` was a round-1 fix, so I did not label it. This is the single row where the strict rule most plausibly cost a real signal.

5. **Merges, splits and duplicates.** `uniform-agent-isolation-r2-triage.md` Finding 3 is ruled "DUPLICATE of Finding 2 ... Not counted as a second finding", so it gets no row. `decision-folder-currency-plan-r4-triage.md` `R4-1` merges `H4-2`, giving one row with `severity_raised = low/medium`. `optional-modules-2ci-triage.md` `F-DOC` is one verdict over two reviewer findings (opus 4 and 5) and is one row; that choice is what makes the round reconcile at 11.

6. **Severity of a dismissed finding.** Where a triage gives a hypothetical rating for a dismissed finding ("Severity if it had been valid: low") I record that token; where it says N/A or gives none I record `none`. Dismissed rows never enter the reconciliation.

7. **`escaped` under-fires by construction.** I required an explicit statement that an earlier round had the defect and did not raise it. Timelines that IMPLY an escape (a defect present since round 1 first raised at round 3) were not enough. This is why 207 of 221 rows are `unknown`.

## DISCREPANCIES IN THE SOURCES, RECORDED NOT CORRECTED

1. `structured-skeleton.reviews/inc1-triage.md`: the Tally section counts 10 valid findings and explicitly excludes `O4`; the round record counts 11. Per-finding rows reconcile with the log only if `O4` counts.
2. `decision-folder-currency-r2-triage.md`, verification `F1`: the triage reproduces and rules on the fact that `docs/plans/agent-scaffold.ledger.md:413` records a word count as "45 words to 43" where the true counts are 46 and 42. This is the propagated miscount named in my brief. The triage ruled it `VALID BUT ACCEPT RESIDUAL` and out of scope, on the ground that the ledger is the orchestrator's own transient record.
3. `structured-skeleton.reviews/inc3-triage.md` records a non-finding: the implementer claimed 276 tests where the triager re-ran and measured 275.
4. `checks-runner-worktree-name-collision-triage-r3.md` records that the round-3 adversarial reviewer measured a mutation-kill rate of 100/100 on its own machine while the triager re-measured 10/100 and 16/100 on the triage machine, and rules the reviewer's framing ("pins nothing") wrong in a way that made the defect look different from what it is. This is the headline-100/100-versus-10/100 case named in my brief; the triage file records both numbers.
5. `prompt-drift-guard-r4-triage.md` records that the round-4 reviewer's evidence contained a transcription error (claiming `.agents/hooks/pre-commit` is committed when it is not), which the triager corrected without changing the verdict.

## LIMITS I STATE MYSELF

1. **This dataset cannot separate the two hypotheses on its own.** 87.8% of counted rows have `unstated` provenance. The 13 `introduced_by_prior_fix` rows come from 6 tasks and are heavily concentrated in two loops (`decision-folder-currency` and `prompt-drift-guard`), which are also the two loops that ran the most rounds and whose triagers happened to write provenance sections. Whether that concentration reflects where injection HAPPENED or where triagers CHOSE TO RECORD IT is not recoverable from these files.
2. **Recording provenance is a triager habit, not a workflow requirement.** 30 of 50 contributing files carry no provenance statement at all. Nothing in the corpus suggests triagers were ever asked for one. Two triagers wrote a section under an explicit heading ("PROVENANCE, which I checked because it bears on scope and neither reviewer established it"; "Note on provenance of the defect"), and both note they were doing something the reviewers had not.
3. **The severity ceiling in my slice is low.** 160 of 204 severity-carrying rows are `low`; there is exactly one `critical` and two `high`. Any per-severity provenance analysis over my slice will be underpowered.
4. **My round-number derivation is positional.** Round records carry no round field, so I derived round number as position within `(task, phase)`. Where a triage states its own round I checked the two agree, and they did in every case; where a triage does not state a round (for example `agent-isolation-triage.md`) `round_number` is `unknown` in the TSV and only the oracle mapping carries the number.
5. **My oracle mapping is asserted, not machine-derived.** The 51 file-to-round-record assignments in `reconcile.py` were made by reading headers and matching commit ranges and artifact descriptions. A misassignment would appear as a reconciliation mismatch, and 43 of 51 match exactly, which bounds but does not eliminate the risk.
6. **The corpus derivation can miss renames.** `git log --diff-filter=D` finds deletions. A triage file that was RENAMED rather than deleted, and whose final path no longer contains `triage`, would not appear. I did not test for this, and it is one possible partial explanation for the ten shortfall rounds.
7. **Class is my judgement, not the triage's.** No triage file assigns a class. `prose` versus `code` for a finding about a Rust doc comment is a call I made by asking what the prescribed fix touches; another extractor could reasonably split those differently, which would move a substantial number of the 126 `prose` rows.
8. **The three reviewer files in my slice are unextracted.** If the other extractor extracted rows from the reviewer files in ITS slice, the two datasets are not directly unionable without filtering on file type.
