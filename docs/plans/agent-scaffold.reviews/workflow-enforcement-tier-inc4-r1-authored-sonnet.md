# workflow-enforcement-tier-inc4, round 1, lens: newly authored prose

Reviewer: sonnet, lens B (the newly authored prose and only that). Scope per the
orchestrator's brief: the inc4 increment description and its risk-classification
paragraph; acceptance checks 21, 21b, 22 and 23; the widened acceptance check 16's
prose; the three reconciled sentences (Documentation impact opener, increment-order
sentence, no-separate-documentation-increment passage); the re-tensed sentences.

Diff range reviewed: 363ac06..079d63f (two commits, seven files).

## What was exercised

- Read the full diff (`git diff 363ac06..079d63f`) line by line, isolating every
  hunk that adds new sentences as opposed to deleting or re-tensing existing ones.
- Read `docs/plans/agent-scaffold.ledger.md` from the paragraph beginning "THE INC4
  LOOP IS OPEN (2026-08-07" through the four paragraphs after it, plus the earlier
  paragraphs those four paragraphs point back to (the inc1 escalation-merge record,
  the citation-conversion record, the Q-55-twinsites record).
- Computed the actual round counts and valid-finding totals for
  `workflow-enforcement-tier-inc1` and `prompt-drift-guard-inc1` directly from
  `docs/metrics/workflow.jsonl` with `jq`, rather than trusting any prose summary
  of it.
- Ran literal `grep -F` searches, against both the current tree and the pre-inc3
  historical revision (`git show 6b1c847~1:src/main.rs`), for every double-quoted
  fragment in the step file that is attributed to a real file (source, test,
  README, or pack/AGENTS.md), to test acceptance check 21's own claim that such a
  search is mechanical.
- Cross-checked the four re-pointed citations the inc4 planner pass claims to have
  verified first-hand (`src/main.rs:2289`, `:2878`, `:2279-2287`, `:257-258`)
  against the current source by line number, and checked the `{pid}-{nanos}`
  citations in `checks-runner-worktree-name-collision.md`.
- Compared acceptance checks 1, 9 and 23 for literal command overlap.
- Read `src/main.rs:Projection`'s corrected doc comment and `run_status`'s
  `toml_source` branch to confirm check 22's own claim before treating it as
  settled ground.
- Did NOT re-verify every citation and quotation in the file end to end (that is
  the "citations resolve / measured claims reproduce" reviewer's lens; I would be
  duplicating it). Did NOT re-hunt for code defects the pass may have missed
  (the third reviewer's lens).

## Findings

### R1B-1 (medium): inc1's "twenty valid findings" figure does not match the metrics log

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:308` (new paragraph,
also rendered verbatim at `docs/plans/agent-scaffold.md:1703`) reads:

> Inc1 OF THIS VERY STEP spent THREE rounds and TWENTY valid findings, EVERY ONE
> an inaccurate description of correct behaviour and zero defects in what the
> code does.

The "THREE rounds" half is correct: `docs/metrics/workflow.jsonl` has exactly
three `"type":"round"` records for `"task":"workflow-enforcement-tier-inc1"`
(lines 246-248), followed by one escalation record (line 249) and nothing else.
But their `valid_findings` fields are 3, 4 and 6, which sum to 13, not 20.

Reproduction:

```
$ grep '"task":"workflow-enforcement-tier-inc1"' docs/metrics/workflow.jsonl | grep '"type":"round"' | grep -o '"valid_findings":[0-9]*'
"valid_findings":3
"valid_findings":4
"valid_findings":6

$ nix shell nixpkgs#jq --command bash -c '
  grep "\"task\":\"workflow-enforcement-tier-inc1\"" docs/metrics/workflow.jsonl \
    | jq "select(.type==\"round\") | .valid_findings" | jq -s "add"'
13
```

No other record in the file carries this task name or this increment name (a
plain `grep -n '"workflow-enforcement-tier-inc1"' docs/metrics/workflow.jsonl`
returns exactly the four lines 246-249), and there is no `dismissal_recheck`
record that would add to the tally. Every alternate way of summing the
`reviewers` sub-arrays instead of the round-level (post-dedup) total tops out at
14, never 20.

For comparison, the SAME sentence's other figure is correct: "Step 92
(`prompt-drift-guard`) spent SIX rounds and FIFTEEN valid findings" sums exactly
(4+3+5+1+2+0=15 across six round records at lines 215, 216, 218, 222, 224, 225 of
the metrics log), and matches the independently-recorded "Six review rounds, 15
valid findings" at `docs/plans/agent-scaffold.md:322`. So the sentence gets one
of its two central calibration numbers right and the other wrong by roughly 54
percent.

This "20" is not a number inc4 invented from nothing: the ledger's own
pre-existing "INC1 IS MERGED UNDER AN ESCALATION" paragraph (before "THE INC4
LOOP IS OPEN") already asserts "three rounds, 20 valid findings" for the same
merge. But that does not make the newly authored sentence under review true, and
it is exactly the kind of figure my lens was asked to check against the raw log
rather than against another prose summary. The sentence is load-bearing: it is
part of the argument for why inc4 is classified `risky` (two clean rounds
required) rather than `low_risk`, appealing to this project's own worst
calibration class, so a reader taking the "20" at face value overstates how bad
that specific precedent was by more than half.

Fix shape (not prescribing wording): correct "TWENTY" to "13" (or however the
implementer wants to round-total it), or drop the exact number if it cannot be
reconciled, following this project's own stated preference for deletion over a
second unverified number.

### R1B-2 (low): acceptance check 21's "run each quoted fragment as a literal search" is not executable for at least one quote in the same file

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:345` (new
acceptance check 21) reads, in part:

> ... run each quoted fragment of source, test, `README.md` or `pack/AGENTS.md`
> text as a literal search against the file it is attributed to ... The check is
> mechanical rather than a reading.

Line 374 of the same file (an existing, only lightly re-tensed bullet under
"Documentation impact / INC3") attributes this quote to `src/main.rs:run_validate`:

> "An absent file (the metrics log, or a `--plan` path) is not a validation
> failure ... a missing file prints a note to stderr and is skipped rather than
> hard-failing (the same treatment for both, so the behaviour is consistent)"

The `...` in the middle is an elision the plan's own author inserted for
brevity; it is not text that ever appeared in the source. A literal search for
this fragment therefore cannot succeed against any revision of the file,
including the one it is quoting from:

```
$ grep -F "not a validation failure ... a missing file prints a note" src/main.rs
$ echo $?
1

$ git show 6b1c847~1:src/main.rs | grep -F "not a validation failure ... a missing file prints a note"
$ echo $?
1
```

(`6b1c847~1` is the commit immediately before inc3's own fix landed, i.e. the
last revision where `run_validate`'s doc comment actually read this way; the
real text there is "... is not a validation failure: not every project
instruments, and a plan is validated only on request, so a missing file prints a
note ..." per `git show 6b1c847~1:src/main.rs` lines 804-807.)

So for this quotation, "run it as a literal search" gives a "no match" result
regardless of whether the citation is accurate, stale, or anything in between:
the method supplies zero discriminating signal for the one case it is describing
here, which is exactly the class of instruction my lens was told to distrust ("a
check that cannot fail is worse than no check"). Check 21's own fallback logic
("a quotation with no match ... is either RE-TENSED ... or DELETED") still lets a
human reviewer reach the right answer for this quote (it is legitimately
historical, correctly re-tensed, and the elision is visually obvious), but that
requires reading the sentence for tense and intent, not the "mechanical" literal
search the check advertises as sufficient on its own. I found exactly one quote
in the file constructed this way (a `grep -n` for a mid-quote `...` inside
double quotes across the whole file turns up only this one instance), so I am
not claiming the check is broadly unusable, only that its stated method has a
narrow, demonstrated gap.

### R1B-3 (low): acceptance check 23 adds nothing that checks 1 and 9 do not already cover

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:348` (new
acceptance check 23) reads:

> AFTER INC4, the plan still renders and still validates: `cargo run -- render
> docs/plans/agent-scaffold.plan.toml --check` reports up to date, and `cargo run
> -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` exits 0
> with `workflow invariants hold`, so the step's own closing pass is measured by
> the mechanism the step built.

Both commands are already present, verbatim, in two earlier checks in the same
file:

- Check 1, line 316: "Plan render pinned: `cargo run -- render
  docs/plans/agent-scaffold.plan.toml --check`."
- Check 9, line 324: "... from the agent-scaffold repository root, `cargo run --
  validate --source docs/plans/agent-scaffold.plan.toml --workflow` exits 0 with
  `workflow invariants hold` ..."

Check 23 is framed as inc4's own dedicated closing verification ("the step's own
closing pass is measured by the mechanism the step built"), but as written it is
the union of two checks that any round already has to run regardless of whether
inc4 did anything at all, and that would pass identically on the pre-inc4 tree
(neither command exercises anything inc4 changed: inc4's only source change is a
doc comment on an unrelated struct field, and its sidecar edits do not touch
`docs/plans/agent-scaffold.plan.toml`'s renderable content or its workflow
invariants). Per the lens question ("does check 23 add anything check 1 does not
already cover"): no, and it also does not add anything check 9 does not already
cover. Running it produces no information a round would not already have from
checks 1 and 9.

Fix shape: either delete check 23 and note that checks 1 and 9 already cover it,
or, if a dedicated post-inc4 sanity check is wanted, state what it verifies that
1 and 9 do not (for example, that inc4's own edits did not silently reintroduce
a render or workflow-check regression in a way that is distinct from the
standing checks 1 and 9 already assert every round).

## Areas checked with no finding

- **Lens A, the increment description and risk-classification paragraph beyond
  the two figures above.** The step-92 order number (92) and its "prompt-drift-
  guard" name check out against `docs/plans/agent-scaffold.md:322` and
  `docs/metrics/workflow.jsonl`. "Five retrospective and one prospective
  measurement" is not new to this diff at the other site it appears
  (`workflow-enforcement-tier.md:377`, only the surrounding clause was
  re-tensed) and the figure is independently corroborated at three separate
  pre-existing ledger paragraphs (the ones beginning "THE FIX SHAPE IS
  MEASURED", "STANDING DISCIPLINES LEARNED", and "THE PREDICTION AND THE
  OUTCOME"), so I treat it as settled and did not re-derive it. The "instrument-
  magic-filename (order 60)" and "checks-runner-worktree-name-collision (order
  93)" citations in the new inc4 bullet both check out against
  `docs/plans/agent-scaffold.plan.toml` and the roadmap table.
- **Lens B, checks 21b and 22.** The four re-pointed citations the inc4 planner
  claims to have verified (`src/main.rs:2289`, `2878`, `2279-2287`, `257-258`)
  all resolve exactly to the named subjects (`init_plan_defaults_to_git_and_
  skips_inside_a_repo`, `install_precommit_hook_skips_a_non_repo`, the
  `agent-scaffold-poc-` scratch helper, and the `instrument.md` read,
  respectively), checked directly against `src/main.rs` by line number. The
  `{pid}-{nanos}` citations in `checks-runner-worktree-name-collision.md`
  (`tests/validate_workflow_toml_source_needs_no_plan.rs:97,129,190,287` and
  `tests/validate_toml_primary_skips_markdown_plan.rs:74`) all land on the
  `std::env::temp_dir().join(format!(` line as claimed. `nanos()`'s doc comment
  in `src/checks.rs` does state the opposite of a per-process-uniqueness
  premise, and `reserve_runner_worktree` exists, matching the ledger's claim
  about that sidecar's now-stale subjects. Check 22 is checkable and correct:
  the removed clause ("present only when a readable `--plan` was given") is
  gone from the new `src/main.rs:Projection.plan` doc comment, and `run_status`
  does populate `plan` from a TOML-primary `--source` with no `--plan` given
  (its `toml_source(&args.source)` branch), matching the correction.
- **Lens C, the widened check 16.** The prose does distinguish the specified,
  tested divergence (plain `validate` versus `--workflow` on the same EACCES /
  ENOTDIR fixture, arm-scoped by `Q-55-existsgate`, which the check must verify
  still holds) from the separate, pre-existing, out-of-scope inconsistency
  (mode-000 file versus unsearchable directory under plain `validate`), which is
  explicitly and unambiguously marked "a RECORDED RESIDUAL ... an implementer
  must not 'fix' it here and a reviewer must not raise it" and cross-referenced
  to the established "in the manner of checks 18, 19 and 19b" convention. I did
  not find a version of this check text that would lead a reader to "fix" the
  residual.
- **Lens D, the three reconciled sentences.** The Documentation-impact opener
  now explicitly names inc4 as "a documentation step OWED" rather than hiding
  the tension the old wording denied, and grounds the exception in a measured
  fact (inc1's citation-conversion pass was itself falsified by inc2 and inc3,
  which the ledger's citation-conversion and INC1-merge paragraphs both
  corroborate). The increment-order sentence's "INC4'S EDGE ... WAS MEASURED
  RATHER THAN ARGUED" claim is backed by the same evidence. The "no separate
  documentation increment" passage's new exception clause is consistent with
  both. I read all three as honest reconciliations rather than restatements of
  the same tension in safer language.
- **Lens E, re-tensed sentences.** Checked the "silently passes" / "OVERSTATEMENT"
  paragraph's quoted stderr messages against the pre-fix source
  (`git show 6b1c847~1:src/main.rs`): both `eprintln!` messages exist verbatim
  at that revision, so no qualifier was dropped in the re-tensing. Checked the
  `status --json` "has NO golden, and HAD no test on its serialisation at all"
  split: `status --json` still has no golden today (no `GOLDEN` identifier
  mentions `status` anywhere in `src/main.rs` or `src/next.rs`), which is why
  that half correctly stayed present tense while only the "no test at all" half
  (falsified by inc2's added `status --json` assertions, per the ledger's
  Q-55-twinsites paragraph) was moved to past tense. Checked the `active_loop`
  "FIFTH ITEM" re-tensing against the current `src/next.rs`: the field doc
  comment now reads "or `None` when there is nothing to act on (no steps, every
  step terminal, or a round log this tool cannot vouch for)", matching the
  corrected description in the past-tense sidecar text; the still-present-tense,
  out-of-scope `NextProjection`-level "Every derived part is optional" comment
  at `src/next.rs:162` is a different comment at a different site and was
  correctly left untouched (it is on the declined `Q-55-currencyscope` list).
  The `default_ledger_path` and `AGENTS.md` backstop-sentence re-tensings both
  read as accurate descriptions of the pre-fix state on inspection of the
  current, post-fix code and prose. I found no case of a re-tensed sentence
  losing a qualifier the way the recorded historical case did.
- **The twin-site deletions (`Q-55-twinsites`).** Both sites named in the
  ledger's Q-55-twinsites record are fixed as described: `tests/
  unsafe_pairings_are_refused_and_omitted.rs:156` no longer says "FOUR owed",
  and `:1370` no longer claims "no test on its serialisation at all" (only "has
  no golden" remains).

## Severity summary

- R1B-1: medium.
- R1B-2: low.
- R1B-3: low.
- No `high` or `critical` findings in this lens.
