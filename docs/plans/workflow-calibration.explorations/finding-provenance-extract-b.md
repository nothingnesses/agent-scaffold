# Finding-provenance extraction, slice B

Extractor: claude-sonnet-5, worktree `.claude/worktrees/cal-int-b`, branch `explore/cal-int-b` at `d916def`. Interpretive pass for `workflow-calibration`: read triage files and record what triagers explicitly ruled about a finding's provenance. This is a data-extraction record, not an analysis; it does not compute an injection rate or take a position on the injection-versus-new-content question.

## METHOD

### Corpus derivation (reproducible)

Commands run from the worktree root:

```
git log --diff-filter=D --name-only --pretty=format: -- 'docs/plans/*.reviews/*' 'docs/plans/*review*' 'docs/plans/*triage*' \
  | grep -v '^$' | sort -u > deleted_files.txt      # 296 lines
git ls-files 'docs/plans/*.reviews/*' | sort -u > live_files.txt   # 49 lines
cat deleted_files.txt live_files.txt | sort -u > full_corpus.txt  # 343 lines, confirmed
grep -i 'triage' full_corpus.txt > triage_corpus.txt              # 89 lines, confirmed
```

343 and 89 both match the pre-registration exactly.

### Split

```
awk '{print NR-1, $0}' triage_corpus.txt > triage_indexed.txt
awk '$1 % 5 == 0 {print}' triage_indexed.txt > overlap.txt        # 18 files, shared overlap
awk '$1 % 5 != 0 && $1 % 2 == 1 {print}' triage_indexed.txt > mine_odd.txt   # 35 files, mine
awk '$1 % 5 != 0 && $1 % 2 == 0 {print}' triage_indexed.txt > other_even.txt # 36 files, other extractor's
```

18 + 35 = 53, matching the pre-registration's stated slice size for extractor B exactly.

### The exact 53-file slice (0-indexed position in the sorted 89-file list)

```
0  docs/plans/agent-scaffold.reviews/agent-isolation-triage.md
1  docs/plans/agent-scaffold.reviews/backlog-clearing-triage.md
3  docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-r2-triage.md
5  docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage.md
7  docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage-r4.md
9  docs/plans/agent-scaffold.reviews/code-value-audit-static-inc1-triage.md
10 docs/plans/agent-scaffold.reviews/code-value-audit-static-inc2-triage.md
11 docs/plans/agent-scaffold.reviews/compaction-prep-triage.md
13 docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-r2-triage.md
15 docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-r4-triage.md
17 docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-triage.md
19 docs/plans/agent-scaffold.reviews/decision-folder-currency-triage.md
20 docs/plans/agent-scaffold.reviews/decision-fold-triage.md
21 docs/plans/agent-scaffold.reviews/deliberation-mode-triage.md
23 docs/plans/agent-scaffold.reviews/driver-output-generation-inc1-triage.md
25 docs/plans/agent-scaffold.reviews/driver-output-generation-inc2-triage.md
27 docs/plans/agent-scaffold.reviews/file-safety-rules-round2-triage.md
29 docs/plans/agent-scaffold.reviews/findings-files-triage.md
30 docs/plans/agent-scaffold.reviews/gate-prompt-clarity-triage.md
31 docs/plans/agent-scaffold.reviews/human-onboarding-triage.md
33 docs/plans/agent-scaffold.reviews/instrument-flag-triage.md
35 docs/plans/agent-scaffold.reviews/lifecycle-capture-triage.md
37 docs/plans/agent-scaffold.reviews/no-wrap-convention-triage.md
39 docs/plans/agent-scaffold.reviews/pack-rebuild-tracking-triage.md
40 docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md
41 docs/plans/agent-scaffold.reviews/prompt-drift-guard-r3-triage.md
43 docs/plans/agent-scaffold.reviews/prompt-drift-guard-triage.md
45 docs/plans/agent-scaffold.reviews/q59-backlog-fold-triage.md
47 docs/plans/agent-scaffold.reviews/round-log-core-A-triage.md
49 docs/plans/agent-scaffold.reviews/session-preflight-triage.md
50 docs/plans/agent-scaffold.reviews/state-schema-1-triage.md
51 docs/plans/agent-scaffold.reviews/state-schema-2-triage.md
53 docs/plans/agent-scaffold.reviews/step88-triage.md
55 docs/plans/agent-scaffold.reviews/task-entry-regrounding-inc1-triage.md
57 docs/plans/agent-scaffold.reviews/triager-independence-reviewer-opus.md
59 docs/plans/agent-scaffold.reviews/triager-independence-round2-reviewer.md
60 docs/plans/agent-scaffold.reviews/triager-independence-round2-triage.md
61 docs/plans/agent-scaffold.reviews/triager-independence-triage.md
63 docs/plans/agent-scaffold.reviews/triager-on-findings-reviewer-b.md
65 docs/plans/agent-scaffold.reviews/triager-on-findings-triage.md
67 docs/plans/agent-scaffold.reviews/uniform-agent-isolation-triage.md
69 docs/plans/agent-scaffold.reviews/workflow-driver-stage1-round2-triage.md
70 docs/plans/agent-scaffold.reviews/workflow-driver-stage1-triage.md
71 docs/plans/decision-receipt.reviews/decision-receipt-triage.md
73 docs/plans/optional-modules-2b.reviews/optional-modules-2b-triage.md
75 docs/plans/optional-modules-2ci.reviews/optional-modules-2ci-triage.md
77 docs/plans/plan-fold.reviews/plan-fold-triage.md
79 docs/plans/reviewer-harness-field.reviews/reviewer-harness-field-triage.md
80 docs/plans/structured-skeleton.reviews/inc1-triage.md
81 docs/plans/structured-skeleton.reviews/inc2-triage.md
83 docs/plans/structured-skeleton.reviews/inc4-triage.md
85 docs/plans/structured-skeleton.reviews/inc6-r3-triage.md
87 docs/plans/waiver-model.reviews/triage.md
```

For files that no longer exist in the working tree, content was recovered with `git log --diff-filter=D --format=%H -1 -- <path>` to find the deletion commit, then `git show <commit>^:<path>`. All 53 files recovered content successfully; every dump had non-zero length.

### A corpus-construction defect found while reading the slice, not before

The case-insensitive `triage` substring match is not a clean proxy for "triager ruling." The word "triager" itself contains the substring "triage" (`triage` + `r`), so any file about the *topic* of triager independence, written by a *reviewer*, gets swept into the 89-file corpus even though it is exactly the kind of document the corpus is supposed to exclude (a reviewer's claim, not a triager's ruling).

Three files in my slice are this false-positive class:

- index 57, `triager-independence-reviewer-opus.md`: opus's raw review of the `triager-independence` step (findings R1, R2). Confirmed by reading it: it opens "Reviewer: claude-opus-4-8 (independent)."
- index 59, `triager-independence-round2-reviewer.md`: a round-2 reviewer file (finding T1) for the same step.
- index 63, `triager-on-findings-reviewer-b.md`: Reviewer B's zero-findings report for `triager-runs-only-on-findings`.

I read all three in full to confirm their role before deciding what to do with them. I extracted **zero finding rows** from these three files: their content is reviewer *claims*, and the actual triager *rulings* on those same review rounds are separately present in my slice under their correct names (`triager-independence-triage.md`, `triager-independence-round2-triage.md`, `triager-on-findings-triage.md`), which I did extract from. So no round is lost; the same review rounds are represented once, correctly, through their real triage files.

This means my slice is 53 files by the letter of the pre-registered rule, but only **50 of them are genuine triage documents**; 3 are misclassified reviewer files that happen to satisfy the substring match. I flag this because the pre-registration's own corpus count (89 triage vs 254 reviewer) was produced by the identical grep and therefore inherits the identical defect; the "89 triage files" figure itself likely includes a small number of reviewer files under the same mechanism (at minimum the 6 I can name across both extractors' slices: the 3 above plus `triager-independence-reviewer-sonnet.md`, `triager-on-findings-reviewer-a.md`, `triager-on-findings-reviewer-r2.md`, all visible in the full 89-line listing and all following the same `triager-*-reviewer*` pattern). I did not go back and re-derive the 89/254 split with a corrected filter; that would have changed the pre-registered corpus after the fact, which the task instructs against. I record it here as a limit on the corpus, not something I corrected.

## RECONCILIATION against the jsonl oracle

`docs/metrics/workflow.jsonl` holds 204 `type:"round"` records. For every round my 50 real triage files address, I compared my per-finding row count and severity multiset (valid + accept_residual rows, excluding dismissed) against that round's `valid_findings` and `severities`.

**47 of 50 files reconcile exactly** (row count and severity multiset both match the jsonl round record). Three do not:

1. **`lifecycle-capture-triage.md`** (task `lifecycle-capture` / step `formatter-reflow-convention`, increment `formatter-reflow-convention-inc1`). The triage explicitly rules two findings `VALID` (Finding 1: receipt `task` divergence, low; Finding 2: reconciliation-clause incompleteness, low), both with `Disposition: DEFER`. The matching jsonl round (`"artifact":"lifecycle-capture pass: Q-57 formalization + Q-59/Q-60 capture"`) records `"outcome":"clean","valid_findings":0,"severities":[]`. The triage's own two low `VALID` findings are not reflected in the round's finding count at all.
2. **`q59-backlog-fold-triage.md`** (task `q59-backlog-fold`). The triage rules Finding 1 (`Q-59` receipt `task` divergence) `VALID`, severity low, `Disposition: ACCEPTABLE (not must-fix before convergence)`, and its own "Round outcome" line reads "CLEAN. One valid low finding, ruled ACCEPTABLE." The matching jsonl round records `"outcome":"clean","valid_findings":0,"severities":[]`. Same pattern as (1): one triage-ruled `VALID` finding, zero jsonl-recorded findings.
3. **`structured-skeleton.reviews/inc1-triage.md`** (task `structured-skeleton-inc1`). The triage's own tally section states the total explicitly: "medium: 2 ... low: 8" (and separately numbers ten concrete remediation items, 1 through 10), i.e. 10 valid findings. The matching jsonl round records `"valid_findings":11,"severities":["medium","medium","low","low","low","low","low","low","low","low","low"]`, 2 medium + 9 low. The jsonl has one more low-severity finding than the triage document's own stated tally supports. I could not find an eleventh named finding anywhere in the file to account for the extra low; the document is internally consistent at 10 and disagrees with the oracle at 11.

I did **not** force any of these three into agreement by reinterpreting verdicts. Cases (1) and (2) share a visible pattern worth naming without over-claiming it as an explanation: both are dispositions the triage itself labels `DEFER` / `ACCEPTABLE`, language distinct from `ACCEPT RESIDUAL`, which elsewhere in the corpus (e.g. `checks-runner-worktree-name-collision-triage.md`'s T4/T5/T7/T10, all `VALID BUT ACCEPT RESIDUAL`) *does* get counted into `valid_findings`. A plausible reading is that this project's jsonl-writing convention counts "this is a real defect in the artifact whose risk we accept" but not "this observation is correct but doesn't belong to this artifact's own spec, and is punted to a different, unstarted step"; and `reviewer-harness-field-triage.md`'s opus L1/L2/L3 ("VALID as an observation; NOT a defect") give a third, cleaner data point for the same asymmetry: the jsonl's own per-reviewer breakdown for that round records `opus: raw_findings:3, valid_findings:0` verbatim, confirming that "valid-as-observation-but-not-a-defect" is a category this project's own logging already zeroes out at the reviewer level, not just at my reading of it. I present this as a candidate explanation, not a resolved one; case (3) does not fit this pattern at all (all ten of its findings are ordinary `VALID`/`VALID BUT ACCEPT RESIDUAL`, not deferred-as-out-of-scope), so whatever produced the extra low in the jsonl there is a different, unexplained cause. All three are reported as mismatches; none are silently absorbed.

## SHORTFALL

The pre-registration states roughly 13 of the 102 `new_valid` rounds have no triage file anywhere in the corpus. I checked every task that appears in my 53-file slice for gaps in its own round sequence (i.e., a jsonl round for that same task with no triage file at all, under any name, by either extractor) and found **none**. Every multi-round task I touched had a triage file for every round with any reviewer-raised content: `checks-runner-worktree-name-collision` (rounds 1, 2, 4 mine; round 3 exists as two files, `-r3-triage.md` and `-triage-r3.md`, both belonging to the other extractor, not missing), `decision-folder-currency-fold` (5 plan-review rounds, all 5 have files), `decision-folder-currency` work-review (2 rounds, both have files), `prompt-drift-guard` (5 non-clean rounds have 5 files: base/r2/r3/r4/verify), `workflow-driver-stage1` (2 non-clean rounds, 2 files), `structured-skeleton` (inc1-4 and inc6 all have files; inc5's two rounds are both genuinely clean with zero raw findings from every reviewer, which by this project's own `triager-runs-only-on-findings` rule means no triager runs at all, not a gap but a designed absence), and `triager-independence` (2 rounds, 2 files).

Since my slice has no internal shortfall, I went looking for where the corpus-wide ~13-round gap actually lives, by checking whether task names with `new_valid` (or clean-but-`valid_findings`>0) jsonl rounds have *any* triage file, under any name, anywhere in the full 343-file corpus (not just the 89 filtered ones, in case a triage file exists but doesn't contain the string "triage"). None of the following do, and none of them are in my slice or (by construction, since they have no triage file to split) the other extractor's:

- `workflow-hardening`: 2 rounds, 14 and 12 findings, no `ts` field (pre-dates timestamping, consistent with `Q-14`/pre-convention).
- `convergence-accounting`: 1 round, 4 findings, no `ts`.
- `plan-maintenance`: 1 round, 8 findings, no `ts`. Notably, this task's rulings (`H1`, `H2`, `M1`) are *cited by slug* as precedent inside `plan-fold-triage.md` and `no-wrap-convention-triage.md` in my own slice, so the rulings clearly existed and were communicated forward even though no dedicated file survives in the corpus.
- `workflow-doc-fixes`: 1 round with findings (7), no `ts`. Its rulings (`F11`, `F13`, `F14`) are likewise cited by slug in `triager-independence-round2-triage.md`.
- `user-prompts-dir`: 1 round, 2 findings, no `ts`.
- `agents-md-drift-guard`: 3 rounds with nonzero `valid_findings` (2, 2, and 1 on a `clean`-outcome round), `ts:"2026-07-23"`.
- `principle-by-name-projection`: 1 round, `valid_findings:1` on a `clean`-outcome round, `ts:"2026-07-23"`.
- `driver-isolation-reminder-scope`: 1 round, 5 findings, `ts:"2026-07-23"`.
- `single-source-recommendation-rule`: 1 round, 2 findings, `ts:"2026-07-24"`.

That is 12 rounds across 9 tasks, close to the pre-registered "roughly 13." The first four tasks fit the pre-registration's stated `Q-14` cause cleanly (no `ts` field at all, i.e. from before timestamping existed). The last four do **not** fit either stated cause cleanly: they carry `ts` values contemporaneous with many well-triaged tasks in my slice (`decision-folder-currency-fold` and `prompt-drift-guard` both have rounds dated `2026-07-27`/`2026-07-28`; these four are `2026-07-23`/`24`), so "predates the convention" does not explain them, and I found no textual evidence in any file I read of `Q-63`'s "collapsed into producer/orchestrator" pattern for these four specifically (no other triage file names their rulings by slug, unlike `plan-maintenance` and `workflow-doc-fixes` above). I report this as an open, unexplained residual rather than force-fitting it to one of the two named causes.

## DESCRIPTIVE COUNTS

225 finding rows extracted from 49 of the 53 slice files (`decision-fold-triage.md` contributed 0 rows, being a genuine zero-finding CLEAN round; the 3 misclassified reviewer files contributed 0 rows by design, see METHOD).

Verdict:

| verdict | count |
| --- | --- |
| valid | 166 |
| accept_residual | 36 |
| dismissed | 23 |

Class (what the finding is about):

| class | count |
| --- | --- |
| prose | 125 |
| code | 56 |
| test | 28 |
| config | 16 |

Provenance (the load-bearing column):

| provenance | count |
| --- | --- |
| unstated | 193 |
| pre_existing | 18 |
| introduced_by_prior_fix | 14 |

86 percent of rows are `unstated`. That is the headline descriptive fact of this extraction: explicit provenance is the exception, not the rule, even across triagers who write extensively (several of the richest files below produced 5-9 explicit-provenance rows each out of many more `unstated` ones in the same document).

Cross-tabulation, class by provenance:

| class | unstated | pre_existing | introduced_by_prior_fix |
| --- | --- | --- | --- |
| code | 52 | 4 | 0 |
| config | 16 | 0 | 0 |
| prose | 100 | 12 | 13 |
| test | 25 | 2 | 1 |

Every `introduced_by_prior_fix` row but one is `class:prose` (13 of 14): every explicit "the fix pass created this" statement I found was a triager pointing at a doc/comment claim that a previous fix pass introduced or altered, not at production code or a test. This is a real pattern in what I extracted, not a claim about the general rate: my slice's richest multi-round tasks for this question (`checks-runner-worktree-name-collision`, `prompt-drift-guard`, `decision-folder-currency-fold`) are all documentation-and-comment-heavy artifacts (a Rust module's doc comments, a plan's prose, an `AGENTS.md`-family guidance file), so the corpus I was handed may simply contain more prose-fix-pass provenance discussion than code-fix-pass discussion, rather than prose being intrinsically more injection-prone.

Escaped (triage explicitly says this finding was raised in an earlier round and missed):

| escaped | count |
| --- | --- |
| no | 197 |
| unknown | 23 |
| yes | 5 |

The 5 `yes` rows are: `checks-runner-worktree-name-collision-r2-triage.md` X2 and X3 (round 2, both explicitly tied to round 1's `T1`); `checks-runner-worktree-name-collision-triage-r4.md` RG2 (round 4, explicitly traced through round 2's certification failure back to round 2's own fix); `prompt-drift-guard-r3-triage.md` RD-2 and RD-3 (round 3, both explicitly framed as "fix incompleteness against a requirement round 2 set" or "carried through the consolidation, no scope question").

Provenance by verdict (do explicit-provenance findings skew toward a particular disposition):

| verdict | unstated | pre_existing | introduced_by_prior_fix |
| --- | --- | --- | --- |
| valid | 141 | 12 | 13 |
| accept_residual | 30 | 6 | 0 |
| dismissed | 22 | 0 | 1 |

## The explicit-provenance rows, listed in full

Because these are the load-bearing data points and there are only 32 of them (18 `pre_existing` + 14 `introduced_by_prior_fix`), here is every one, by source file and finding id, so the analyst does not have to re-derive them from the TSV:

- **`checks-runner-worktree-name-collision-r2-triage.md`** (round 2): X2 `pre_existing` (round 1's T1, "unfixed remainder"), X3 `pre_existing` (round 1 already rated it), X4 `pre_existing` (byte-identical since `HEAD~2`, this step never touched it).
- **`checks-runner-worktree-name-collision-triage-r4.md`** (round 4): RG1/MU1 `introduced_by_prior_fix` (round 3's `AD1a` fix), RG2 `introduced_by_prior_fix` (round 2's fix authored the false sentence, per `git log -S`).
- **`prompt-drift-guard-triage.md`** (round 1): FN-2 `pre_existing` (the precondition was built in a different, earlier step), FN-3 `pre_existing` (restates an already-accepted residual, `H4-3`).
- **`prompt-drift-guard-r2-triage.md`** (round 2): V2-1 `introduced_by_prior_fix` (round 1's fix commit added the contradicting text), A2-2 `introduced_by_prior_fix` (text is part of round 1's `CT-1` fix).
- **`prompt-drift-guard-r3-triage.md`** (round 3): V3-1/RD-1 `introduced_by_prior_fix` (round 2's consolidation), RD-2 `pre_existing`+escaped (round 2's triage required a fix that was never applied), RD-3 `pre_existing`+escaped (this step's own mechanism commit, before round 1), V3-2 `introduced_by_prior_fix` (round 2's consolidation rewrote correct text into false text), RD-4 `pre_existing` (a different, earlier step, `git log -S` confirms untouched).
- **`decision-folder-currency-plan-r2-triage.md`** (round 2): NEW-1 `introduced_by_prior_fix` (round 1's fix pass authored the flawed claim), R2-1 `introduced_by_prior_fix` (round 1's fix pass copied a stale citation), R2-2 `introduced_by_prior_fix` (round 1's fix pass added new text that contradicts old, untouched text), R2-3 `pre_existing` (describes completed, earlier work).
- **`decision-folder-currency-plan-triage.md`** (round 1): T-3a `introduced_by_prior_fix` (attributed to a *different, earlier step*, step 89, not this task's own round), T-4 `pre_existing` (an established convention already present in converged artifacts).
- **`decision-folder-currency-plan-r4-triage.md`** (round 4): H4-1/H4-4/H4-5 `pre_existing` (explicitly framed as "new-lens findings on text no prior round had read," i.e. old text, not new), R4-1/R4-2/R4-3 `introduced_by_prior_fix` (explicitly labelled "fix-induced residue from the round-3 split"), H4-3 `pre_existing` (a frozen, already-converged sibling step's content).
- **`triager-independence-round2-triage.md`** (round 2): T1 `introduced_by_prior_fix` ("introduced by the Group B fix rather than being pre-existing"; the triage's own words, distinguishing the two explicitly).
- **`task-entry-regrounding-inc1-triage.md`** (round 1): I1-2subA `pre_existing` (inherited from the build-plan design document, not an implementer deviation).
- **`uniform-agent-isolation-triage.md`** (round 1): Finding1 and Finding3 both `pre_existing` (both confirmed untouched by the diff under review via `git diff --stat`).

Two of these deserve a second look because they are cross-task rather than cross-round: `decision-folder-currency-plan-triage.md`'s T-3a and `prompt-drift-guard-triage.md`'s FN-2/FN-3 attribute causation to a **different, earlier step's** fix pass, not to an earlier round of the *same* review loop. I coded them `introduced_by_prior_fix`/`pre_existing` because the triager's own words draw that causal line explicitly and by name (a specific commit or a specific prior step), which is the standard the rule sets; but the two-value provenance scheme this task specifies (round-scoped `introduced_by_prior_fix` vs `pre_existing`) does not cleanly distinguish "introduced by the immediately preceding round's fix within this loop" from "introduced by a wholly different, already-converged step." Both read as legitimate applications of the rule to me, but an analyst computing an injection rate *per review loop* should decide whether cross-step causation belongs in the same bucket as within-loop causation before aggregating.

## AMBIGUITIES and the rules I used

- **Round number when not stated in prose.** Several files title themselves only by task (no "round N" sentence) but sit in a family with an explicit `-r2`/`-round2` sibling (e.g. `code-value-audit-static-inc1-triage.md` next to `-inc1-round2-triage.md`, which belongs to the other extractor). I treated the unsuffixed file as round 1 by this naming convention, not by inferring it from jsonl order. Where no sibling and no explicit round text existed at all (e.g. `agent-isolation-triage.md`, `compaction-prep-triage.md`), I recorded `round_number:unknown`. This inference is about round *labelling*, never about `provenance`, so it does not touch the one rule that matters most.
- **`escaped` defaults to `unknown`, not `no`, when a file simply never mentions round history.** I used `no` only when the file is explicitly a task's first round (stated "round 1," or structurally the first entry in that task's jsonl sequence with no earlier round to escape from) or when a provenance statement pins the finding to a specific round that rules out escaping. Otherwise `unknown`.
- **Merged findings (one triage verdict covering two or more reviewer ids, e.g. `R2+S1`, `V3-1/RD-1`, `MSG-LOCATOR (C1+L3+S1)`) are one row.** This is not my convention; it is how each triage file itself counts for its own "valid findings" tally, and reconciling against the jsonl oracle requires matching that convention (a merged triage verdict is one entry in the round's `severities` array, confirmed repeatedly by exact-count reconciliation across the corpus).
- **Dispositions with no exact fit in `valid`/`accept_residual`/`dismissed`.** This corpus uses `DEFER`, `ACCEPTABLE`, `VALID BUT OUT OF SCOPE`, `VALID (design)`, and `VALID as an observation; NOT a defect` as distinct dispositions, and (per RECONCILIATION above) at least two of these track differently against the jsonl oracle than plain `ACCEPT RESIDUAL` does. I mapped all of these to `accept_residual` for the TSV's controlled vocabulary, which is a lossy collapse I want the analyst to know about rather than discover independently: `accept_residual` in this dataset is not a single homogeneous category.
- **Severity fields that are themselves disputed between two reviewers or two rounds.** Where the file gives two different raised severities (e.g. `low(opus)/medium(sonnet)`), I recorded both, joined, in `severity_raised` rather than picking one, so the column stays a faithful transcription rather than my own summary judgement.
- **`class` (prose/code/test/config/unknown) is my own categorical judgement**, not something the triage files label directly. My rule: `test` when the finding is fundamentally about missing or wrong test coverage; `code` when it is about production-code behavior; `prose` when it is about documentation, comments, prompts, or plan text; `config` when it is about a structured non-code file (`checks.toml`, `pack.toml`, `flake.nix`, jsonl field values). Boundary cases (e.g. a YAML rule file's comment, a CLI help string) were judged individually and could reasonably be classed differently by another reader; this column should be treated as lower-confidence than `verdict`, `severity`, or `provenance`.

## LIMITS

- **I did not compute, and this record does not support, an injection rate.** That is the analyst's job, deliberately kept separate from extraction.
- **`unstated` is the overwhelming majority (86 percent) of provenance values.** Whatever the analyst concludes about injection versus new-content, it will rest on the 32 rows (14 percent) where a triager wrote down the causal chain explicitly, not on the full 225. Any weighting or rate calculation must reckon with that small a base, and with the fact that the 32 are not a random sample: they cluster heavily in a handful of multi-round, high-scrutiny tasks (`checks-runner-worktree-name-collision`, `prompt-drift-guard`, `decision-folder-currency-fold`) where the triager was explicitly instructed to check provenance, rather than being spread evenly across the corpus.
- **This is one of two independent extractions over an overlapping but not identical slice.** 18 files (the mod-5 overlap) were also extracted by extractor A on a different model. I did not look at extractor A's output before or during this extraction, and I did not try to guess or match it. Disagreement between the two on the 18 shared files is the intended signal, not an error to reconcile.
- **Three reconciliation mismatches are reported, not resolved** (see RECONCILIATION). I have a candidate explanation for two of the three (a `DEFER`/`ACCEPTABLE` disposition that this project's own jsonl logging appears not to count) and no explanation for the third (`structured-skeleton-inc1`, off by one low finding against the triage document's own stated tally). Do not treat my candidate explanation as confirmed; it is one plausible reading of a pattern seen three times across the corpus (twice in mismatches, once in a reviewer-level `raw_findings`-vs-`valid_findings` split inside a file that otherwise reconciled), not a verified rule.
- **The `class` column is a judgement call**, flagged above; treat it as directional, not authoritative.
- **The corpus itself has a construction defect** (the `triager` substring collision, see METHOD) that most likely affects the 89/254 split system-wide, not only my slice's 3 instances. I did not attempt to fix this for the whole corpus; I only handled my own 3 instances correctly.
- **I did not re-verify the reviewers' or triagers' underlying technical claims** (whether a cited line number is really where a triager says it is, whether a `git log -S` really returns what's quoted). I extracted what the triage documents assert, including about their own provenance reasoning, on trust that the triagers' own stated verification (which is extensive and itself a recurring subject in this corpus) is accurate. Cross-checking that would be a different, much larger task.
