# Fix-pass shape, mechanical derivation B

Explorer B, worktree `.claude/worktrees/cal-mech-b`, branch `explore/cal-mech-b`, base commit `12d6a01`.
This is an independent derivation. It was built without looking at explorer A's method or
numbers. Scripts referenced below live in my scratch directory (not in this worktree) so they
are re-runnable from the exact paths given; the key commands are also reproduced inline so the
record is self-contained even if the scratch directory is gone.

Scratch directory: `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/cal-b/`
(`step1_build_loops.py`, `step2_fixpass_diffs.py`, `step3_exploratory_parent.py`, `step5_consolidate_tsv.py`,
plus their JSON intermediates `loops.json`, `fixpass_rows.json`, `fixpass_excluded.json`, `exploratory_parent_rows.json`).

## Headline result, stated first

**The primary pre-registered measurement (P(next round is `new_valid` | fix-pass shape),
tabulated from git diffs) is NOT ACHIEVABLE from this repository's git history at a usable
sample size.** Of 113 candidate same-loop round transitions, the git-commit-boundary method
this task prescribes resolves to a clean, cross-validated, non-contaminated fix-pass diff for
exactly **1** of them. An exploratory extension (described below, lower confidence, not the
prescribed method) recovers 17 more. All 18 total resolved fix passes classify as shape
`MIXED` and all 18 have a following-round outcome of `clean`. There is **zero variance in
either variable** in the resolved sample, so no contrast between shapes, and no `new_valid`
outcome at all, survives in the data this method can extract. This is not "a small effect,
underpowered"; it is "the method cannot see the cases that would decide the question," for a
specific, demonstrated structural reason (below). I did not manufacture a rate from this: I am
reporting that the primary hypothesis is untested by this route.

## METHOD

### Step 1: build the round timeline and cluster into loops

Every `type:"round"` line in `docs/metrics/workflow.jsonl` was matched to the commit that
introduced its current line content via `git blame -l -s docs/metrics/workflow.jsonl` (one
commit hash per line, exact text `git blame -l -s docs/metrics/workflow.jsonl > blame_ls.txt`,
231 lines, one per JSON record of any type). 204 lines have `type:"round"`.

**Loop key.** The brief states "successive rounds sharing a task form one artifact's loop," but
literal task-string equality is not sufficient: several task labels are reused for genuinely
different artifacts at different points in history (`structured-skeleton`, `workflow-driver`,
`plan-fold` each appear as two or more textually-identical-task but semantically distinct review
loops, distinguishable by their `increment` field or by having already converged). I defined:

    loop_key = (task, phase, increment_or_None)

and, within each key's ordered sequence of round records, split a NEW loop instance whenever the
PRECEDING round (same key) had already reached its own risk-class convergence threshold, i.e.
`outcome == "clean" and consecutive_clean >= threshold(risk_class)`, threshold 1 for `low_risk`,
2 for `risky`. This uses only fields on the record itself (no path or prose reading) and is
exhaustive: once a loop converges, any further round sharing its key must be a fresh loop over
new content, since a converged loop is by definition done. I verified this rule against every
case I found where a task label recurs non-adjacently in the round-only sequence
(`structured-skeleton` x2, `workflow-driver` x3 by increment, `plan-fold` x2, plus two tasks
whose rounds are genuinely interleaved at fine grain with another task's rounds,
`prompt-drift-guard-inc1` and `checks-runner-worktree-name-collision`) and it produces the
semantically correct split or merge in every case I checked by hand.

Result: 204 round records group into **91 loops**: 24 singleton (1 round, no possible pair,
e.g. a "capture" round or a loop that converged the instant it was logged) and 67 multi-round
loops, contributing **113 same-loop consecutive round pairs** (candidate fix passes). Loop-length
distribution: 24 loops of 1 round, 40 of 2, 14 of 3, 8 of 4, 4 of 5 (cap reached without an
earlier converge), 1 of 6 (`prompt-drift-guard-inc1`, converged at streak 1 against a `risky`
threshold of 2, i.e. a human-accepted convergence shortfall past the cap). 24 + 67 = 91 loops;
24*1 + 40*2 + 14*3 + 8*4 + 4*5 + 1*6 = 204 rounds. 113 pairs = 204 - 91 (one fewer pair than
rounds, per loop).

Command: `nix shell nixpkgs#python3 --command python3 step1_build_loops.py` (reads
`workflow.jsonl` + `blame_ls.txt`, writes `loops.json`).

### Step 2: locate and classify the fix pass between round K and round K+1

For each of the 113 pairs, let `c_K` = round K's blame commit, `c_{K+1}` = round K+1's blame
commit (this is literally what the brief specifies: "the commits touching the reviewed artifact
between those two round-recording commits").

- If `c_K == c_{K+1}`: excluded as **CO_RECORDED** (see EXCLUSIONS; the window is empty by
  construction, since both round entries were appended by the identical commit).
- Else, verify `git merge-base --is-ancestor c_K c_{K+1}` (history must be linear for the window
  to mean "between"; checked, never violated: 0 non-ancestor pairs found).
- Else, compute the restricted diff:

      git diff --numstat c_K c_{K+1} -- . \
        ':(exclude)docs/plans/*.reviews/*' \
        ':(exclude)docs/plans/agent-scaffold.ledger.md' \
        ':(exclude)docs/metrics/workflow.jsonl'

  (the brief's three named exclusions: reviewer/triage files, the ledger, the metrics log
  itself; everything else touched between the two commits counts as "the artifact," since
  narrowing further would require reading which specific file the round's prose says is under
  review, which is exactly the prose-dependent guessing this task forbids).

**Classifier (exact rule).** From the restricted diff: `deleted_lines` = numstat deletions
summed across included paths; `added_lines_raw` = numstat additions summed. From the same diff
at `--unified=0`, `added_nontrivial` = count of added (`+`) lines whose stripped content is
neither empty nor matches `^[.,;:)\]}({\[]{1,3}$` (a bare terminator/punctuation/brace addition
of three characters or fewer, e.g. a stray closing paren or added comma, not real content).

    shape = NO_OP            if added_lines_raw == 0 and deleted_lines == 0
          = DELETION_ONLY     if deleted_lines > 0 and added_nontrivial == 0
          = AUTHORED          if added_nontrivial > 0 and deleted_lines == 0
          = MIXED             if added_nontrivial > 0 and deleted_lines > 0
          = TRIVIAL_ADD_ONLY  otherwise (only trivial/whitespace additions, nothing deleted)

Doc-vs-code split: `.rs` files bucketed as code, everything else (this repo's non-Rust content
is prose/config, not a second programming language) as doc/other; both totals are in the TSV.

Command: `nix shell nixpkgs#python3 --command python3 step2_fixpass_diffs.py` (reads
`loops.json`, writes `fixpass_rows.json` for resolvable pairs and `fixpass_excluded.json` for
excluded ones).

### Cross-validation against the oracle (this is what broke the naive result)

Every round record independently carries `changed_since_prev`. If it is `false`, no fix pass
happened by definition (nothing changed) and the restricted diff between `c_K` and `c_{K+1}`
MUST be empty; if it is `true`, a real fix happened and the diff should be non-empty and plausibly
attributable to this loop's own files. I checked every one of the 30 non-co-recorded pairs
against this and found the naive commit-window diff **contradicts or fails to confirm** the
oracle field in the overwhelming majority of cases (detailed under EXCLUSIONS). This is the
reconciliation step that turned an apparently-usable 30-row table into a 1-row table, and it is
exactly the kind of check this task's brief demands ("reconcile against an independent oracle
wherever one exists").

## RECONCILIATION against the JSONL oracle

    nix shell nixpkgs#jq --command jq -s '[.[]|select(.type=="round")] as $r
      | {total: ($r|length),
         changed_true: [$r[]|select(.changed_since_prev==true)]|length,
         changed_false: [$r[]|select(.changed_since_prev==false)]|length,
         changed_new_valid: [$r[]|select(.changed_since_prev==true and .outcome=="new_valid")]|length,
         unchanged_new_valid: [$r[]|select(.changed_since_prev==false and .outcome=="new_valid")]|length,
         total_findings: ([$r[].valid_findings]|add), tasks: ([$r[].task]|unique|length)}' \
      docs/metrics/workflow.jsonl

Result: `total=204, changed_true=183, changed_false=21, changed_new_valid=99,
unchanged_new_valid=3, total_findings=419, tasks=79`.

This reproduces the ledger's pre-registered numbers **exactly**: 204 rounds, 183 changed / 21
unchanged, 99/183 = 54.1% (Wilson 95% CI 46.9-61.2), 3/21 = 14.3% (Wilson 95% CI 5.0-34.6),
computed independently here as:

    99/183 = 54.1%  95% CI [46.9, 61.2]
    3/21   = 14.3%  95% CI [5.0, 34.6]

(formula: standard Wilson score interval, z=1.96; script inline in `step*.py` comments, also
computed ad hoc and checked against the ledger's stated intervals, which match to the tenth of a
percentage point). Total findings 419 matches severities-array length summed across all 204
records exactly (`[$r[].severities[]]|length` = 419), so no record has a `severities` array
whose length disagrees with its own `valid_findings` count.

**One reconciliation that does NOT match, flagged rather than explained away.** Grouping
findings by each round's own `risk_class` field gives `low_risk`: 118 rounds, 265 findings, 61
medium-or-worse; `risky`: 86 rounds, 154 findings, 37 medium-or-worse (61+37=98, matching the
oracle's total medium+high+critical count of 1+12+85=98 exactly). The ledger's prose states
"`low_risk` 55 of 250... `risky` 43 of 169" for the same medium-or-worse breakdown: the totals
agree (55+43=98) but the low_risk/risky **split disagrees by 15 findings/15 rounds in both
directions** (265 vs 250, 154 vs 169). I have a plausible but UNVERIFIED explanation: two tasks
(`structured-skeleton`, `task-entry-regrounding`) carry an inconsistent `risk_class` value across
their own rounds (some rounds `low_risk`, some `risky` for the same nominal task), so a
per-round tally (mine) and a per-artifact tally (the ledger's, if it classified each whole task
once) would disagree exactly on rounds like these. I checked the two tasks' own per-round-class
splits and the magnitude does not obviously reconcile to a clean single-direction reclassification,
so I am **not** claiming this explains it, only noting it as the most likely mechanism and
leaving it open. This discrepancy does not affect the primary reconciliation (rounds, changed/
unchanged rates, total findings), which all match exactly.

## EXCLUSIONS, with counts and reasons

Total candidate pairs: 113 (from 91 loops; 24 singleton loops, contributing 0 pairs, are excluded
by construction, not tallied below since they were never candidates).

| bucket | pairs | reason |
|---|---|---|
| CO_RECORDED, backfill-rewrite cluster | 22 | round K and K+1 both blame to commit `1824e7d87226fa288bd1f10ff809cb0670c397b3`, "feat(metrics): require risk_class on round records and backfill the log". This commit REWROTE (not appended) all 46 pre-existing round records to add the newly-required `risk_class` field, so `git blame` attributes every one of those lines' current content to this rewrite, not to whatever commit originally appended each one (the true origin commits, `eaaf13a...` for the first 44 and two more shortly after, are masked). A single commit cannot disambiguate 22 different transitions across many unrelated tasks; not attempted. |
| CO_RECORDED, other | 61 | round K and K+1 share an identical (non-rewrite) blame commit. Distinct shared commits: 24 cover exactly 1 pair, 8 cover 2 pairs, 3 cover 3 pairs, 3 cover 4 pairs (24x1+8x2+3x3+3x4=61). This is this project's dominant logging convention discovered during this analysis (see below): a loop's rounds are frequently appended to the JSONL together, in one "docs: converge X; log rounds A-B" commit, well after the actual review-and-fix activity happened, rather than one commit per round. |
| NON_ANCESTOR | 0 | none found; history between round-recording commits used in this analysis is linear. |
| Non-co-recorded, window computed | 30 | see next table. |

Of the 30 non-co-recorded pairs, cross-validated against each pair's own `changed_since_prev`:

| status | pairs | meaning |
|---|---|---|
| resolved (clean, non-contaminated) | 1 | `optional-modules-inc3` (single loop under this task label), lines 84->85: `changed_since_prev=true`, diff non-empty (5 files: `CHANGELOG.md`, `docs/plans/agent-scaffold.md`, `pack/isolation-guidance.md`, `pack/pack.toml`, `src/manifest.rs`), file set plausible for the task, no lineno-gap suggesting an interleaved task landed in the window. Shape MIXED (added_nontrivial=64, deleted=1). Following round (round K+1) outcome: `clean`. |
| resolved_no_fix_needed (correct control) | 4 | `changed_since_prev=false` and diff correctly empty: `workflow-driver-stage1` 137->138, `task-entry-regrounding-inc2` 142->143, `driver-output-generation-inc1` 150->151, `driver-output-generation-inc2` 153->154. These are genuine "re-review of unchanged artifact" rounds, consistent with the oracle, contributing no shape data (no fix happened) but confirming the method is not simply broken. |
| unresolved_empty_window | 23 | `changed_since_prev=true` but the restricted diff between `c_K` and `c_{K+1}` is EMPTY. A real fix happened (the oracle says so) but it is not visible in this commit window: it landed on `main` either before `c_K` or after `c_{K+1}`. Traced by hand for `workflow-driver-stage1` (see LIMITS): the actual `feat`/`fix` commits for that loop's four rounds land AFTER the loop's own convergence-recording commit, in a batch, consistent with worktree-isolated development merged in one lump once the whole loop finished. |
| contaminated (proven) | 2 | `structured-skeleton|work_review|structured-skeleton-inc6#1` 118->119 (`changed_since_prev=false` yet the window diff is 44 files / 542 lines: this is the Inc-6 branch's entire batch merge landing inside the window, not a round-2-to-3 fix; proven by the contradiction with `changed_since_prev`) and `prompt-drift-guard-inc1` 218->222 (`changed_since_prev=true`, diff non-empty, but the touched files are `docs/plans/agent-scaffold.md`, `.plan.toml`, and `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md`: this is the interleaved `checks-runner-worktree-name-collision` plan-fold, whose own rounds are genuinely interleaved with `prompt-drift-guard-inc1`'s at the round-log level, lines 217/219/220/221 sit between 218 and 222). |

1+4+23+2 = 30, matches.

**A structural finding, not a pre-registered hazard, discovered while building this pipeline:**
this project's git history does not, in general, place a loop's fix-pass commits between its own
round-recording commits. Two distinct patterns produce this: (a) round entries for a whole loop
(or several rounds of it) are frequently appended together, well after the fact, in a single
bookkeeping commit ("log rounds 47-48" style; 61 of 83 co-recorded pairs, excluding the rewrite
cluster); (b) even where each round IS individually recorded, the underlying code/plan commits
for the whole loop often land on `main` as a late, undifferentiated batch (consistent with
worktree-isolated implementer branches merged in one lump at the end of a loop, per this
project's documented workflow), landing AFTER the loop's last round-recording commit rather than
between any two of them (verified directly for `workflow-driver-stage1`: `git log --oneline
--reverse --ancestry-path 38a39c05..HEAD | head -6` shows five `feat`/`fix` commits landing
immediately after round 4's recording commit `38a39c05`, none of them between any pair of the
loop's own four round-recording commits, whose pairwise diffs are all empty). Both patterns mean
the brief's prescribed commit-window method is structurally blind to the majority of this
project's fix passes, not merely noisy about them.

## EXPLORATORY: parent-1 heuristic (not the primary method, lower confidence, separately labeled)

To see whether ANY more signal was recoverable, I tried one further, clearly-lower-confidence
heuristic on the CO_RECORDED bucket, excluding the 22-pair rewrite cluster and excluding any
shared commit covering more than one pair (ambiguous which of several transitions a single
commit belongs to; 61 - 24 = 37 pairs excluded from this attempt for that reason, leaving 24
singleton-group pairs attempted). For each, I took the immediate first parent of the shared
commit and computed its OWN restricted diff (`git show`, same exclusions), applying the same
`changed_since_prev` cross-check.

Command: `nix shell nixpkgs#python3 --command python3 step3_exploratory_parent.py` (reads
`fixpass_excluded.json`, writes `exploratory_parent_rows.json`).

Result: 24 examined. 18 valid (cross-check-consistent): 17 with a non-empty, plausible diff, all
classifying as shape `MIXED`; 1 correctly empty (no fix needed). 6 excluded: 4 where
`changed_since_prev=false` but the parent commit has a diff anyway (contamination: the parent
commit belongs to some other task, e.g. `structured-skeleton-inc5` 112->113 pulls in
`src/main.rs`/`src/plan.rs`/`src/plan/migrate.rs` totalling 973 deleted lines against a round
that changed nothing); 2 still empty even one parent back (`plan-fold` 58->59, `q64-q65-fold`
181->182).

**A decisive selection-bias finding in this exploratory sample, reported rather than used as a
result.** All 18 valid rows have `round_k1_outcome == "clean"`. Zero have `new_valid`. I checked
whether co-recorded pairs generally correspond to a loop's FINAL (converging) transition: of all
83 co-recorded pairs, 55 (66%) are the last transition of their loop, 28 (34%) are a middle
transition. The parent-1 heuristic, restricted to singleton-commit groups (small, quickly-
converging loops), appears to draw even more heavily from final/converging transitions than the
co-recorded population as a whole (every one of the 24 attempted came back with round K+1 =
clean; I did not find a single new_valid case to test). This means the exploratory extension
cannot supply the missing contrast either: it can only ever recover "the fix worked" cases, by
the very selection rule that makes it resolvable at all (a commit that bundles a loop's last one
or two rounds together, appended once the loop is already known to have converged). Reporting a
rate from this subset (e.g. "100% of MIXED fix passes are followed by clean") would misstate what
the data supports; it is 100% by construction of which cases this method can even see, not
evidence about shape.

## RESULTS

Combining the one clean primary-method row with the 17 non-trivial exploratory rows (excluding
the "no fix needed" controls, which carry no shape): **18 resolved fix passes total.**

| shape | n | round K+1 outcome |
|---|---|---|
| DELETION_ONLY | 0 | n/a |
| AUTHORED (pure addition) | 0 | n/a |
| MIXED | 18 | all 18 `clean`, 0 `new_valid` |
| TRIVIAL_ADD_ONLY | 0 | n/a |

P(new_valid \| shape) is **undefined for every shape except MIXED** (no observations) and, for
MIXED, is 0/18 (Wilson 95% CI 0.0-17.6), which is not a usable estimate of anything: it reflects
which fix passes this method could locate, not the true rate, per the selection-bias finding
above. **I am not reporting this as evidence for or against either the INJECTION or NEW CONTENT
hypothesis.** The pre-registered primary measurement is untested.

Cell counts, exactly as found, for the record:

    resolved MIXED, next round clean:      18
    resolved MIXED, next round new_valid:   0
    resolved DELETION_ONLY (any outcome):    0
    resolved AUTHORED (any outcome):         0
    unresolved (fix pass exists per oracle, shape unrecoverable): 23 (primary window) + up to 37 (co-recorded groups >1, not attempted)
    contaminated / discarded:                6 (2 primary + 4 exploratory)
    no-fix-needed controls (changed_since_prev=false, correctly empty): 5 (4 primary + 1 exploratory)

### Secondary pre-registered questions

- **Fix-pass size vs outcome, independent of shape**: not assessable; only 18 resolved rows, all
  one outcome.
- **plan_review vs work_review**: of the 18 resolved rows, 5 are `plan_review`
  (`structured-skeleton` 98->99, `backlog-promotion` 144->145, `plan-fold`#2 146->147,
  `q66-q67-fold` 205->206, plus the primary `optional-modules-inc3` row is `work_review`) and 13
  are `work_review`. Not assessable for outcome contrast (all clean).
- **low_risk vs risky**: of the 18, round_k_risk_class is `low_risk` for most (backfill-era rounds
  predate consistent risky/low_risk tagging in places); not assessable for outcome contrast for
  the same reason.
- **The 5 loops that ran to or past the round-5 cap, vs 2-round loops**: this question does not
  need git diffs, only the oracle round records, and IS answerable. Mechanically, from
  `loops.json` (no git diffing): loop lengths are 24x1, 40x2, 14x3, 8x4, 4x5, 1x6. The four
  5-round loops are `optional-modules-inc2cii` (new_valid,new_valid,new_valid,new_valid,clean),
  `waiver-model` (new_valid,new_valid,clean,new_valid,clean: a clean round followed by a NEW
  new_valid, i.e. a regression after an apparent convergence point, `risky` streak reset), `structured-skeleton-inc3`
  (new_valid,new_valid,new_valid,clean,clean), `decision-folder-currency-fold`
  (new_valid,new_valid,new_valid,new_valid,clean); the one 6-round loop is
  `prompt-drift-guard-inc1` (five new_valid then one clean, converging at streak 1 against a
  `risky` threshold of 2, a recorded convergence-shortfall waiver). All five long-running loops
  show new_valid on every round but their last, i.e. no interior clean-then-regress streak break
  except `waiver-model`'s round 3. This is descriptive only (no shape data attached to any of
  these transitions in my resolved set) and should not be read as more than "these loops kept
  finding things until they didn't."

## LIMITS (stated by me)

- **The primary hypothesis is untested, not merely underpowered.** 113 candidate pairs collapse
  to 18 resolved, all one shape, all one outcome. This is a negative result about what this
  repository's git history can support, not a small-sample estimate to be treated cautiously; it
  is zero information about the shape/outcome relationship.
- **The exploratory extension is provably selection-biased** toward "the fix worked" transitions
  (see above) and additionally only captures fix passes that fit in a SINGLE parent commit; a
  fix pass spanning multiple commits before the log entry would be undercounted (only the last
  commit's diff captured), biasing toward smaller apparent fix passes. I did not attempt to walk
  back more than one parent, since doing so without an independent stopping rule would require
  judgement calls (how far back is "this fix pass" vs "the previous transition's fix pass")
  indistinguishable from reading prose to decide, which this task forbids.
- **I did not attempt the 37 co-recorded pairs in groups of size 2-4** (one shared commit bundling
  multiple transitions): assigning sub-ranges of a single commit's parent chain to specific
  transitions without an ordering assumption I can verify would not be mechanical.
  the 22-pair backfill-rewrite cluster is unresolvable in principle from this repository alone
  (the true origin commits are masked by a later field-backfill rewrite); a prior version of the
  file (before the rewrite) might still recover them via `git log -p` on the pre-rewrite blob,
  which I did not attempt given the yield-per-effort at this point in the analysis.
- **Contamination from interleaved tasks is real and was directly observed twice** (proven, not
  hypothesized) in the 30-pair primary bucket. Any commit-window method on this history needs a
  cross-check against an independent oracle field or it will silently misattribute one task's
  work to another's round transition; I only had one such field available (`changed_since_prev`)
  and it is binary (it cannot catch contamination where a genuine fix pass co-occurs in the same
  window as unrelated changes without net-zero cancellation, only outright contradictions).
- **The severity-by-risk-class reconciliation gap** (265/154 mine vs 250/169 ledger's, same
  98-finding total) is unresolved and flagged, not explained away; it should not be trusted for
  any risk-class-conditioned claim until reconciled.
- This analysis says nothing about causation (whether a specific finding was caused by a
  preceding fix); it was never positioned to, per the brief, and the sample that would let it
  even attempt an association is not there.

## Reproducing this

    cd /home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/cal-mech-b
    git blame -l -s docs/metrics/workflow.jsonl > blame_ls.txt   # commit-per-line oracle
    nix shell nixpkgs#python3 --command python3 <scratch>/step1_build_loops.py
    nix shell nixpkgs#python3 --command python3 <scratch>/step2_fixpass_diffs.py
    nix shell nixpkgs#python3 --command python3 <scratch>/step3_exploratory_parent.py
    nix shell nixpkgs#python3 --command python3 <scratch>/step5_consolidate_tsv.py

Dataset: `docs/plans/workflow-calibration.explorations/fix-pass-shape-b.tsv`, 137 rows (113
primary-method pairs + 24 exploratory-method attempts), one row per (loop, round K, round K+1)
with its method, resolution status, oracle fields, raw diff numbers, and classified shape.
