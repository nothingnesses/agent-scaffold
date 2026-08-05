# workflow-enforcement-tier-inc2, round 4, CLAIM-ACCURACY lens

Commit reviewed: `b54ba3a` (the round 3 fix pass, `git diff HEAD~1..HEAD` against `2b1e39c`;
whole increment is `git diff main..HEAD`).

Working directory: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/r4-claims`.
Binary built at `target/debug/agent-scaffold` (and `target/release/agent-scaffold`) from this
worktree's checkout of `b54ba3a`. All fixtures live under
`/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/r4a-*`
and `r4claims-*`.

## Findings

### R4C-1: `run_status`'s inline comment claims a missing anchor "still supplies a containment root" unconditionally; round 3's own fix made that conditional

Severity: medium.

Claim, `src/main.rs:1150-1152` (a `//` comment inside `run_status`, immediately before its
`note_missing_anchors` call):

> Before the `--resume` split, so BOTH slices report a typo'd anchor. A missing anchor
> still supplies a containment root, and the note is the only place the projection says
> the name behind that root is not on disk.

This sentence is unchanged since round 2's fix (`2b1e39c`, `fix: root containment on an anchor
that does not exist`), confirmed byte-identical at `HEAD~1` and `HEAD`:

```
$ git show HEAD~1:src/main.rs | grep -n "still supplies a containment root"
1140:	// still supplies a containment root, and the note is the only place the projection says
```

Before round 3, "a missing anchor still supplies a containment root" was true unconditionally
(every supplied anchor, existing or not, contributed a root in `resume_roots`). Round 3
(`b54ba3a`) added the narrowing: `resume_roots` now filters to `on_disk` anchors and only falls
back to the full `supplied` set when `on_disk` is empty. A missing anchor beside an anchor that
exists now contributes **no root at all**. The comment was not touched by round 3's diff, so it
still asserts the pre-round-3 universal.

Evidence, run against the built binary:

```
$ SC=.../scratchpad/r4a-statuscomment
$ mkdir -p "$SC/home/docs/plans" "$SC/home/docs/metrics" "$SC/alpha/docs/plans" "$SC/alpha/docs/metrics"
# home/docs/metrics/workflow.jsonl: one record (a different project's log, standing in for the
# CWD-relative default were this to fall through to it)
# alpha/docs/plans/m.plan.toml: [meta] primary = "markdown", one step "s"
# alpha/docs/metrics/workflow.jsonl: two records ("alpha-one", "alpha-two")
$ cd "$SC/home"
$ agent-scaffold status --json --source "$SC/alpha/docs/plans/m.plan.toml" \
    --plan "$SC/beta/docs/plans/s.md"   # beta/... does not exist anywhere
note: --plan /tmp/.../r4a-statuscomment/beta/docs/plans/s.md does not exist
{
  "plan": null,
  "metrics": {
    "records": 2
  },
  "metrics_absent_reason": null
}
```

`--plan` is a missing anchor, reported by the note exactly as the comment describes. But
`metrics_absent_reason` is `null` and `records` is `2`: alpha's own log is read, meaning the
missing `--plan` anchor supplied no containment root at all (only `--source`'s root, which
alpha's own log is trivially inside, ever entered the vector). Had the missing anchor "still
supplie[d] a containment root" as the comment claims, containment would have required the
artifact under both roots, and alpha's own log (not under the fictitious root the missing
`--plan` would have derived) would have been withheld with `log-not-this-project`, as it was
before this round's fix (see the sibling test `a_missing_anchor_does_not_overrule_an_anchor_that_exists`,
C1, which is RED against the round 3 tip on exactly this shape).

Narrowest correction: qualify the clause, e.g.

> A missing anchor still supplies a containment root only when no supplied anchor is on disk
> (`resume_roots`); either way, the note is the only place the projection says the anchor's
> name is not on disk.

### R4C-2: `run_next`'s inline comment makes the same now-conditional claim, also untouched by the fix

Severity: medium.

Claim, `src/main.rs:1628-1629` (a `//` comment inside `run_next`, immediately before its
`note_missing_anchors` call):

> The same typo'd-anchor note `status` prints, for the same reason: `next` roots
> containment on an anchor that does not exist rather than falling through with none.

Also byte-identical at `HEAD~1` and `HEAD`:

```
$ git show HEAD~1:src/main.rs | sed -n '1573,1576p'
fn run_next(args: NextArgs) -> io::Result<()> {
	// The same typo'd-anchor note `status` prints, for the same reason: `next` roots
	// containment on an anchor that does not exist rather than falling through with none.
	note_missing_anchors(&args.source, &args.plan);
```

Same defect as R4C-1, on `next` instead of `status`. Evidence:

```
$ cd "$SC/home"
$ agent-scaffold next --json --source "$SC/alpha/docs/plans/m.plan.toml" \
    --plan "$SC/beta/docs/plans/s.md"
note: --plan /tmp/.../r4a-statuscomment/beta/docs/plans/s.md does not exist
{
  "task": "m",
  "source": "no plan source",
  "metrics": { "records": 2 },
  "metrics_absent_reason": null,
  ...
}
```

`--plan` (missing) contributes no root; `next` roots containment on `--source` (the anchor that
DOES exist) and correctly reads alpha's own two-record log. "`next` roots containment on an
anchor that does not exist" is not what happened on this input: it rooted on the anchor that
does exist, and the missing one dropped out.

Narrowest correction:

> The same typo'd-anchor note `status` prints, for the same reason: `next` can root
> containment on an anchor that does not exist (when no supplied anchor is on disk) rather
> than falling through with none.

## Claim surfaces swept (no finding beyond R4C-1/R4C-2)

- **`resume_roots` doc comment** (`src/main.rs:1515-1571`), every sentence added or reworded by
  the round 3 diff. Checked by: (a) two-missing-anchors run (`next --source <missing-A>
  --plan <missing-B>`, different projects) confirming both missing anchors contribute roots and
  the artifact is checked against both, consistent with "every case where an anchor IS supplied
  yields AT LEAST ONE root"; (b) an anchor whose `try_exists` errors (`chmod 000` on an
  intermediate directory, confirmed running as non-root via `whoami`) beside an anchor that
  exists, confirming the erroring one is grouped into the deciding set (counts as existing, adds
  a root, matching "AN ANCHOR WHOSE EXISTENCE CANNOT BE DETERMINED COUNTS AS EXISTING ... only
  this one can add a root rather than remove one"); (c) the existing suite's
  `an_anchor_that_does_not_exist_still_supplies_a_root` and
  `a_missing_anchor_does_not_overrule_an_anchor_that_exists` (`cargo test --test
  unsafe_pairings_are_refused_and_omitted`, 18/18 pass).
- **`containment_roots` doc comment** (`src/main.rs:1368-1399`). Checked by reasoning against
  the code (`checked_plan_root(...).map_or_else(|| resume_roots(...), |root| vec![root])`) plus
  the `validate --workflow` refusal run below, which exercises `checked_plan_root` exclusively
  (this function's other arm) and confirms "the predicate is not re-implemented and not
  widened" holds for that surface too.
- **`checked_plan_root` and `canonical_project_root` doc comments** (`src/main.rs:1320-1358`),
  not touched by the round 3 diff. Confirmed unaffected: neither calls `resume_roots`, and
  `validate --workflow`'s refusal (which uses only these two) still produces exactly the
  message the README quotes:
  `agent-scaffold validate --source <away plan> --metrics docs/metrics/workflow.jsonl --workflow`
  -> `--workflow would join <away plan> against docs/metrics/workflow.jsonl, which is not under
  the plan's project root <away>; pass a \`--metrics\` under that root, run against the plan's
  own log, or correct the \`--source\` and \`--plan\` pair`, exit 1. Also ran
  `validate --workflow` with neither anchor -> `--workflow requested but no plan source
  resolved: pass a TOML-primary --source or a Markdown --plan`, exit 1.
- **`note_missing_anchors` doc comment and its three-way body** (`src/main.rs:1095-1130`).
  Checked by: the `chmod 000` run above (Err arm prints `note: --source <path> could not be
  checked: Permission denied (os error 13)`, never "does not exist"); the existing
  `an_anchor_that_cannot_be_checked_is_not_reported_as_missing` test (passes); confirmed the
  "no root can be inferred from this line" claim by inspecting every `note:` line printed across
  all runs in this sweep, none of which name a derived root.
- **`resolve_for_containment` doc comment, the `..` residual paragraph** (`src/main.rs:1402-1426`).
  This documents the deliberately-unfixed single-anchor `..` residual (out of scope to raise as
  a behavior defect), so I verified only that the documented BOUND is accurate, by reproducing
  both sides of it:
  - Sole ghost anchor (`--source .../proj/ghost/../docs/plans/q.plan.toml`, `ghost` never
    created) with an explicit `--metrics` pointing at proj's real, on-disk log: refused, with
    the root printed literally as `.../proj/ghost/..` (`metrics: unavailable, ... is not under
    the plan's project root .../proj/ghost/..`); this matches "the anchor's own log is refused
    under a root printed as `<proj>/ghost/..`, which is not a directory."
  - The same file, spelled directly (no `ghost/..`), same explicit `--metrics`: accepted
    (`metrics: 2 records`), confirming "opposite verdicts" for the two spellings of one file.
  - Added a second, on-disk anchor in the same project (`--plan` pointing at a real
    `other.plan.toml` beside the ghost anchor's project, tested via `status --resume` so no
    plan-read side-channel intervenes) beside the sole ghost anchor: the residual disappeared
    and the project's own real ledger was read correctly (`## RESUME STATE` / `PROJ resume
    state.` printed instead of a refusal), confirming "beside an anchor on disk the ghost anchor
    now contributes no root and the log is read," the exact narrowing boundary the doc claims.
- **`README.md`, the "Anchoring changes..." paragraph** (line 236) and **`CHANGELOG.md`, the
  `validate --workflow` REFUSES... bullet** (line 23), both rewritten in the round 2 and round 3
  fix passes. Checked every clause against a run: on-disk-anchors-always-decide (existing test
  `a_missing_anchor_does_not_overrule_an_anchor_that_exists` C2, both anchors exist, still
  rejects); not-on-disk-yields-a-root-only-when-none-on-disk (the two-missing-anchors run above,
  and the existing `an_anchor_that_does_not_exist_still_supplies_a_root` test); beside-an-anchor-
  on-disk-yields-nothing (the `status --json`/`next --json` runs in R4C-1/R4C-2's evidence, and
  the existing C0/C1 checks in `a_missing_anchor_does_not_overrule_an_anchor_that_exists`);
  neither-anchor-no-root (existing test, `for command in ["status", "next"]` block with no
  anchors at all). No finding: every clause matched a real run.
- **Test names/comments for the two anchor-precedence tests**
  (`an_anchor_that_does_not_exist_still_supplies_a_root` and
  `a_missing_anchor_does_not_overrule_an_anchor_that_exists`). Checked that the first test's
  added scoping paragraph ("EVERY RUN HERE SUPPLIES ONE ANCHOR AND THAT ANCHOR IS THE MISSING
  ONE") is true of the test body by grepping the function for a second `"--plan"` argument
  (`awk` range extraction + `grep '"--plan"'`, no match), so its unqualified title remains
  accurate precisely because the test never exercises the two-anchor case that would falsify it.
  Checked the second test's title and assertions agree (C0/C1 both show alpha's own 2-record
  log and `ALPHA resume state.` at exit 0 with no refusal; C2, the same `--plan` path once
  written, still refuses) by reading the full test body and cross-checking against `cargo test`
  output (passes).
- **`an_anchor_that_cannot_be_checked_is_not_reported_as_missing`** (test name/doc). Confirmed
  its `opaque` guard is exercised as intended: `whoami` on this worktree returns a non-root user,
  so `fs::metadata` on the permission-000 file fails and the `if opaque` block's assertions
  actually run; `cargo test` shows it passing.
- **User-facing message strings**: the `--workflow` refusal and its three remedies
  (`pass a \`--metrics\` under that root, run against the plan's own log, or correct the
  \`--source\` and \`--plan\` pair`), the no-plan-source refusal, `unpairable_log_note` and
  `unpairable_ledger_note`'s doc comments (used by `status`/`next` in the metrics line, the
  no-loop reason, and `status --resume` respectively; confirmed the no-loop-reason reuse by
  reading `next.rs:1184-1198`'s `no_loop_text`, which prefers `projection.metrics_absent_note`
  exactly when the reason is `MetricsNotThisProject`), and the three-way `note_missing_anchors`
  text including the `Err` arm. All exercised by direct runs quoted above and unaffected by the
  round 3 diff (none of these strings appear in `git diff HEAD~1..HEAD` except the doc comments
  already covered).
- **Other tests that supply two anchors, checked for interaction with the round 3 narrowing**:
  `a_surface_that_reads_no_plan_is_supplied_a_root` (only ever supplies one, existing, anchor;
  read the full body, no second `--plan`/`--source` anywhere), `resume_omits_the_default_ledger_under_a_divergent_pairing`,
  `the_machine_surface_separates_the_causes_on_both_commands`, and
  `the_resume_reasons_separate_and_cover_the_default_ledger` (all use two anchors that are BOTH
  written to disk before being referenced, so the narrowing's `on_disk`-vs-`supplied` branches
  never diverge for them). Confirmed by reading each full test body; none needed updating and
  none make a claim the round 3 change falsifies.
- **Full regression suite**: `cargo test --test unsafe_pairings_are_refused_and_omitted` in this
  worktree, 18/18 passed, confirming no test's own assertions (as opposed to its prose) were
  broken by anything in the diff.

## Out of scope, not raised

Per the task brief: the in-root bound, the single-anchor `..` residual's behavior (only its
documentation was checked, per above), ADV-2/R2A-2/R2C-2, the stale "FOUR owed red-then-green"
count, the `Q-55-emptyroot` fix site, the four accepted costs, project identity, line length,
and prose hard-wrapping.
