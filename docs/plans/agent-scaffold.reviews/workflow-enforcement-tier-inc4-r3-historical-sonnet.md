# `workflow-enforcement-tier-inc4`, round 3, historical-truth lens

Reviews `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` at HEAD (`93ee357`)
plus the `Q-55` question record and the three waiver notes in
`docs/plans/agent-scaffold.plan.toml`, against the specific question: is every PAST-TENSE
claim true of the past it describes. This is not a check of present-tense truth (a
different reviewer's lens this round) and not a check of whether any mechanical gate could
catch these defects (the third reviewer's lens). Findings elsewhere in this round may
overlap in citation but not in method: every claim below was checked by building a binary
at the commit the claim describes and running the reproduction, or by reading the git
object at that commit, not by re-reading the current tree.

## Result: no findings

I did not find a single past-tense claim in the reviewed text that was false of the past it
describes. This is stated plainly per the reviewer contract. The rest of this file is the
method and the evidence, so the "nothing found" claim is itself checkable rather than
asserted.

## Commit range and how it was established

`git -C <worktree> log --oneline main..HEAD` gives six commits:

```
93ee357 docs: complete the spec-time sweep and name the test file's impact
7ea9842 docs: apply the inc4 round 2 remedies as class sweeps
80f3dc2 docs: correct the w1 waiver figure to 13 (3, 4, 6)
3801859 docs: apply the inc4 round 1 remedies
c27565c docs: correct three stale comment claims for inc4
7787853 docs: make the step's own claims current and specify inc4
```

Per the metrics log, inc4's own round 1 (11 valid findings) reviewed the tip of `7787853`
alone; the round-1 fix pass is `c27565c` + `3801859` + `80f3dc2` (round 2's reviewed
artifact explicitly names "the corrected waiver figure" as already present, which is
`80f3dc2`'s content); the round-2 fix pass, unreviewed until this round, is `7ea9842` +
`93ee357`. I read `git diff 80f3dc2 93ee357` in full (both the step sidecar and
`agent-scaffold.plan.toml`) as the freshest unreviewed material, and separately read the
whole current file for everything earlier fix passes already touched, since a round-1 or
round-2 finding not actually closed is explicitly in my remit too.

The four increments' merge points, needed to know which "before" to check each defect
against, do NOT resolve to the ff-merge commits' immediate parents (those parents are
mid-branch commits from the same rebased lineage, not the pre-increment baseline). I
established the true baselines by finding each increment's "docs: start/open ..." marker
and confirming no code diff exists between it and the true start:

- pre-inc1: `69c0525` ("docs: start the workflow-enforcement-tier step"), confirmed its
  `src/main.rs` still declares `metrics: PathBuf` with `default_value =
  "docs/metrics/workflow.jsonl"` on `ValidateArgs`, `StatusArgs`, `NextArgs`.
- pre-inc2 / post-inc1: `36e19f0` (the inc1 ledger-recorded merge point), confirmed its
  `run_validate` `--workflow` match still has the `_ => eprintln!(...)` soft-skip arm
  (the diff `36e19f0..1bfd0a8` shows this arm becomes `_ => problems.push(...)` inside
  that range, which is how I discovered `1bfd0a8` is NOT pre-inc3 but already inside
  inc3's own lineage, and corrected course).
- pre-inc3 / post-inc2: `36e19f0` again (same commit serves as both boundaries since inc2
  merged there).
- post-inc1 (for the check-11 red case): `6141549`, the ledger-recorded inc1 merge commit.

I built four separate binaries from `git archive` snapshots (avoiding `git worktree add` on
the shared repo, per the fixture-safety rule) under
`<scratchpad>/rev-inc4-r3-a/src-preinc1`, `src-preinc3b` (at `36e19f0`), `src-postinc1` (at
`6141549`), plus the already-`92c0525`-adjacent one used for the same purpose. All builds
used the worktree's own `direnv`-exported toolchain. Fixtures were scaffolded under
`<scratchpad>/rev-inc4-r3-a/fixture-*` only.

## The four DEFECT STATEMENTS and DEFECT NARRATIVES: all confirmed true of the past they name

### Defect A, the false green (`:5`, `:26-52`)

Built the pre-inc3 binary (`src-preinc3b`, at `36e19f0`) and ran the exact reproduction from
inside a freshly scaffolded fixture:

```
$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
stdout: docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
stderr: no metrics log at docs/metrics/workflow.jsonl; nothing to validate
stderr: --workflow has a plan source but the metrics log is missing; skipping the workflow check
exit:   0
```

Matches the sidecar's claimed output at `:36-42` line for line, including the claim that
both skip announcements are on stderr and stdout carries only the ok summary (`:44`'s
"CORRECTION" paragraph). CONFIRMED TRUE of the pre-inc3 tree.

Also confirmed `pack/AGENTS.md:93` at `36e19f0` (pre-inc3) carries the sentence quoted at
`:136` verbatim and unconditionally (`git show 36e19f0:pack/AGENTS.md | sed -n '93p'`), and
that a fixture scaffolded from that binary without `--instrument` renders the identical
sentence at the same line in the deployed `AGENTS.md` (`grep -n "backstop that the required
reviewed rounds" fixture-preinc3b/AGENTS.md` -> line 93). CONFIRMED TRUE for Defect D
(`:8`, `:131-141`) as well: this is the same evidence, since Defects A and D share the
inc3 boundary.

### Defect B, cross-project contamination (`:6`, `:56-106`)

Built the pre-inc1 binary (`src-preinc1`, at `69c0525`) and reproduced the borrowed-slug
false pass exactly as specified at `:76-86`: scaffolded a fixture, renamed its one step's
slug to `triager-runs-only-on-findings` and its status to `complete`, then ran from the
source tree's own root (which carries agent-scaffold's real 245-record log at that
commit):

```
$ agent-scaffold validate --source <fixture>/docs/plans/TEMPLATE.plan.toml --workflow
docs/metrics/workflow.jsonl: 245 records, valid
<fixture>/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
<fixture>/docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit:   0
```

This is exactly the shape claimed at `:86`: a completed step with no review evidence of its
own passes at exit 0 because a foreign log is being read. CONFIRMED TRUE of the pre-inc1
tree. (Record count differs, 245 here versus 233/235 named in the sidecar, because the log
keeps growing across every project round; the sidecar's own text already accounts for this
"the record count grows as the log accumulates" at `:65`, so this is not a discrepancy.)

Also confirmed explorer C's cited borrowed slug (`agents-md-drift-guard`, `:86`) is real:
it appears in `docs/plans/workflow-enforcement-tier.explorations/metrics-path-independent-map.md`
(explorer C's own record), at lines 51, 69, 257 and 417.

### Defect C, the sibling commands (`:7`, `:108-129`)

Two sub-claims, both reproduced on the pre-inc1 binary.

`next`'s fabrication (`:112-123`), reproduced at the exact commit cited (`3170e3f`, which
predates `69c0525` and carries the same unfixed code, confirmed by `git merge-base
--is-ancestor 3170e3f cda0bab` -> yes, and the pre-inc1 `metrics: PathBuf` default is
present at `3170e3f` too since no code changed between them). With the fixture's step at
`in-progress` under the same borrowed slug, run from the agent-scaffold root with no
`--metrics`:

```
$ agent-scaffold next --source <fixture>/docs/plans/TEMPLATE.plan.toml
metrics: 245 records
ACTIVE LOOP
  ...
  state: converged
  streak: 1/1
  rounds: 2/5
  next: mark the step complete, re-render, and commit
  ...
exit: 0
```

Matches `:117-120`'s quoted output field for field (`state: converged`, `streak: 1/1`,
`rounds: 2/5`, `next: mark the step complete, re-render, and commit`) for a project with
zero rounds of its own. CONFIRMED TRUE.

`status --resume`'s leak (`:125`), reproduced by renaming the fixture's plan to
`agent-scaffold.plan.toml` and running `status --resume --source <fixture>/...` from the
agent-scaffold root: it printed this repository's own `## RESUME STATE` block verbatim,
including branch names, worktree paths and in-flight review-loop state (visible in the raw
output: "committed on branch `plan/decision-folder-currency`, worktree
`.claude/worktrees/plan-decision-folder-currency`"). CONFIRMED TRUE of the pre-inc1 tree,
matching `:125`'s claim exactly.

### Defect D, the SE-3 documentation half (`:8`, `:131-141`)

Covered above under Defect A (shared evidence, shared boundary commit).

## The conditional-perfect sites, `:141` and `:182`-equivalent (current line ~57 and ~79)

Both use "would have" rather than a straight past tense. In both cases I checked (a)
whether the counterfactual described is true, and (b) whether the mood shift is required by
the facts rather than a stylistic choice with a simpler past-tense alternative available.

**Site 1** ("THIS STEP WOULD HAVE MADE THE GAP WORSE BEFORE IT FIXED IT... After the tier
policy landed, that same non-instrumented user would not have got a quiet false green any
more: they would have got a hard failure..."). I pulled the pre-round-3 text
(`git show 4a721ab:.../workflow-enforcement-tier.md`) to see what this replaced: the
original was present-tense ("THIS STEP MAKES THE GAP WORSE... After the tier policy lands
... they get a hard failure"), written at PLAN time as a warning about what would happen if
the two halves were NOT shipped together. Since inc3 in fact shipped the tier-policy code
and the `pack/AGENTS.md:93` qualifier in the SAME increment (confirmed: the plan's own inc3
description at the increment-breakdown section names both in one bullet, and the actual
diff `36e19f0..3d00341` touches both `run_validate`'s match arm and `pack/AGENTS.md` in the
same increment), the scenario described (a hard failure with no qualifier) never actually
happened. A plain past tense ("This step made the gap worse before it fixed it") would
assert that the bad outcome occurred, which is false; the conditional perfect correctly
marks it as a counterfactual that was averted by bundling. RULING: TRUE, and the mood
change is necessary, not just a remedy-class stretch.

**Site 2** ("measured: `status --resume --source A --plan B` printed A's block at exit 0,
and an anchor-rooted inc2 would have kept it"). The first clause is an actual measurement
(pre-inc2 behaviour); the second is a deduction from the design's own stated property,
established two paragraphs earlier in the same file (current `:159`-equivalent: "The
resolved log is DERIVED from the anchor, so it is always under the anchor's root and a
predicate rooted THERE can never fire on that pairing"). I checked the deduction is sound
by tracing `default_ledger_path`'s anchor order (`--source` first, confirmed at
`src/main.rs:run_next`): under an anchor-rooted design the DEFAULT ledger path is derived
FROM the anchor, so it is trivially always "under" the anchor's own root by construction,
meaning an anchor-rooted guard could never fire on the default-ledger divergent-pairing
case. RULING: TRUE. One observation, not raised as a finding because it does not make the
claim false: pairing the word "measured" with a deduced (never-built) counterfactual in one
parenthetical is a little generous about how the second clause was established; a reader
who does not already know the site is a deduction could read both halves as empirically
measured. This is a precision-of-attribution question, not a truth question, and I judge it
below the threshold of a finding under my lens.

## Implementer imperatives and the doc-comment sweep (`:197`-`:236` region)

Checked all four "falsified or incomplete" doc-comment claims and the fifth pre-existing
item against the CURRENT source (since these describe already-shipped inc2 work, the
correct check is that the current comments read as the sidecar says they were corrected
to):

- `src/next.rs:189-192` (`no_active_loop_reason`): now serialised, doc comment says so.
  Matches "BECAME FALSE and had to change".
- `src/next.rs:162-167` (`NextProjection`'s own comment): now enumerates "a missing plan, a
  missing log, or a log that cannot be paired with this plan". Matches "BECAME INCOMPLETE"
  claim being resolved.
- `src/main.rs:561-567` / current `Projection` doc comment: carries the equivalent
  three-cause enumeration. Matches "HAD THE SAME DEFECT" (past).
- `src/next.rs:184-186` (`resume_state`): "absent, carries no such section, or is not this
  plan's" (three causes). Matches "WAS SHORT BY ONE" (past).
- `src/next.rs:181-182` (`active_loop`): now reads "no steps, every step terminal, or a
  round log this tool cannot vouch for", with no "blocked" cause. Matches "SAID it was
  `None` when ... every pending step blocked" being corrected (past "SAID").

Also verified the specific claim that `no_loop_reason`'s third string ("no in-progress or
ready step") WAS UNREACHABLE: read `select_active_loop` (`src/next.rs:711-736`) and
confirmed its three branches cover, in order, any `InProgress` step, any pending step with
blockers met, and any pending step at all; `None` is returned only past all three, i.e.
only when `steps` is empty or every step is terminal. There is no way to reach "non-empty,
not all terminal, yet no active loop", so the pre-fix function's `else` branch
(`git show 8beb1c2^:src/next.rs`, confirmed the string existed there as the `unwrap_or`
fallback) could never actually fire. CONFIRMED TRUE of both the past state (the string
existed and was dead) and the sidecar's present claim about it.

## Waiver figures recomputed from `docs/metrics/workflow.jsonl`

Extracted every `"task":"workflow-enforcement-tier-inc{1,2,3}"` round record
(`grep '"task":"workflow-enforcement-tier"'` and the per-increment task variants) and
recomputed both the per-round counts and the severity ceilings:

- `workflow-enforcement-tier-w1`: rounds carry `valid_findings` 3, 4, 6 -> sum 13. Note
  claims "13 valid findings (3, 4, 6)". MATCHES EXACTLY.
- `workflow-enforcement-tier-w2`: rounds carry 9, 5, 6, 4 -> sum 24. Note claims "24 valid
  findings (9, 5, 6, 4)". MATCHES EXACTLY. Severity ceilings per round (max of each
  `severities` array): high, high, medium, high. Note claims "severity ceiling high, high,
  medium, high". MATCHES EXACTLY.
- `workflow-enforcement-tier-w3`: rounds carry 6, 4, 2, 0, 2 -> sum 14. Note claims "14
  valid findings (6, 4, 2, 0, 2)". MATCHES EXACTLY. Severity ceilings: medium, medium, low,
  none, medium. Note claims "severity ceiling medium, medium, low, none, medium". MATCHES
  EXACTLY. Peak `consecutive_clean` across the five rounds is 1 (round 4). Note claims "the
  peak streak reached 1 of the 2". MATCHES.

All three previously-suspect counts check out precisely against the source of truth. Given
the task's warning that three counts in this increment have already been measured wrong, I
verified this by hand rather than trusting the arithmetic in the note text, re-deriving the
sums independently from the raw `valid_findings` and `severities` fields.

## Round-2 fix pass (`80f3dc2..93ee357`): read in full, no new false claim introduced

Read the complete diff (`git diff 80f3dc2 93ee357` on both the step sidecar and
`agent-scaffold.plan.toml`) rather than only the resulting text, specifically hunting for
authored (non-mechanical) content, since this project's own recorded failure mode is a fix
pass introducing a fresh false claim. Findings:

- The overwhelming majority of the diff is token-level present-to-past re-tensing
  ("comes"->"came", "is"->"was", "gets"->"got", etc.), consistent across both files.
- Two non-tense corrections: `README.md:228`->`README.md:238` (a citation number fix in
  `agent-scaffold.plan.toml`), verified CORRECT by reading `README.md:238` directly, which
  does carry the quoted sentence ("Unlike `validate` it never fails on a missing or
  malformed file..."). And deletion of the clause "which is the increment's one source
  change" from the inc4 documentation-impact bullet about `Projection.plan`'s doc comment,
  replaced by adding a new bullet naming `tests/unsafe_pairings_are_refused_and_omitted.rs`
  as a second source-adjacent change (`Q-55-twinsites`). Verified this deletion is
  warranted: the file's own INC4 documentation-impact list (current `:387`) now names both
  the `main.rs:Projection` doc comment AND the `tests/unsafe_pairings_are_refused_and_omitted.rs`
  comment corrections, so "one source change" would be false again had it survived; its
  removal is correct, not a loss of a true claim.
- No new factual assertion was introduced by this pass; every changed clause either
  re-tenses an already-true fact or corrects a previously-wrong citation/count.

## Round 1 and round 2 fixes spot-checked for actual closure

Per the instruction that a prior-round finding not actually closed is itself a finding, I
spot-checked several MEDIUM findings from round 1's triage (`workflow-enforcement-tier-inc4-r1-triage.md`)
against the current tree:

- `R1A-2`/`R1C-1` (the false "`#[serde(skip)]` appears exactly once" claim): the sentence
  is gone from the current file (confirmed by grep for the quoted fragment: no match).
  CLOSED.
- `R1A-3`/`R1C-2` (the stale "`no_active_loop_reason` IS `#[serde(skip)]`" / "status has no
  reason field" claim): current text reads "WAS `#[serde(skip)]`" and "HAD no reason field
  at all" at the same site. CLOSED.
- `R1A-4` (present-tense bullets `:201`/`:202` describing already-fixed doc comments): the
  current file's equivalent bullets read "BECAME FALSE and had to change" / "BECAME
  INCOMPLETE", past tense throughout. CLOSED.
- `R2C-1`/`R2C-2`/`R2A-3` (the `README.md:228` citation, the "still promises" phrasing, the
  half-re-tensed `Q-55-scope` twin): all three sites read correctly in the current text
  (see the round-2-fix-pass section above for `R2C-1`; the "still promises" wording is gone
  from `Q-55`'s own text, replaced by the conditional-perfect construction covered above).
  CLOSED.

I did not attempt to re-verify every one of round 1's and round 2's roughly 20 combined
valid findings; the four above were chosen because they were the ones most directly
adjacent to the historical-truth question (re-tensed claims whose underlying fact needed
checking), which is my lens.

## What I checked and what I did not reach

Checked, with reproducible evidence above: the five DEFECT STATEMENTS (`:3`-`:8`
equivalents) and four DEFECT NARRATIVES (current `:86`, `:112`, `:125`, `:127`
equivalents) against binaries built at the correct pre-fix commits; the two
conditional-perfect sites; five of the six named IMPLEMENTER IMPERATIVE sites (the
doc-comment sweep) against current source; the "no_loop_reason third string" claim against
`select_active_loop`'s actual branches; all three waiver notes' figures against
`docs/metrics/workflow.jsonl`; the complete unreviewed diff (`80f3dc2..93ee357`); and four
round-1/round-2 findings for actual closure. That is roughly 30 discrete past-tense or
counterfactual claims verified against primary sources (git history, built binaries, or
current code), not merely read for internal consistency.

Not reached: acceptance check 13b's own "before inc2, prints `workflow invariants hold`
... against an inc2 rooted on the anchor too" claim was not independently reproduced with a
built dual-fixture (Markdown-primary A / TOML-primary B) binary; I relied on the
document's internal consistency and the `Q-55-endproperty` reasoning it shares with the
already-verified `status --resume` anchor argument, rather than a fresh build. I did not
re-verify every citation `Q-55-currencyscope`'s check 21b covers (the three sibling
sidecars), since `checks-runner-worktree-name-collision.md`'s citations are explicitly out
of scope for this round. I did not re-derive the full ~64-token re-tensing count the task
mentions; I sampled specific named sites rather than diffing every word. I did not attempt
to verify claims about the design-pass explorers' own build diffs (line counts like
"+79/-15", "+96/-13") against their actual exploration-record diffs, since those are
`docs/plans/workflow-enforcement-tier.explorations/*.md` artifacts from a prior, separately
reviewed pass, not something this round's remit or the task's named sites pointed at.

## Recorded residuals

None of my checks touched the four inc2 residuals, the four inc3 residuals, or the five
settled dismissals named in my brief. I confirm I am not re-raising any of them.

## ASCII check

`LC_ALL=C grep -n '[^ -~]' <this file>` returns 0 hits (verified before commit).
