# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 2, reviewer: EXECUTABILITY

Reviewed: commit `3354a90` on `review/q55-ep2-exec` (the fix pass over round 1's findings, folding `Q-55-conventionlesscost` and `Q-55-resumepairing` into `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`), diffed against `a9dda1c`.

Lens: is the material that has never had an executability pass (accepted cost (iii) and check 19b, the `status --resume` framing and 14c's third run, 14g's extended fourth run, check 13b's three new fixture clauses, the inc2 increment description) buildable as written, and does what it specifies actually do what it claims. Region: exactly those five areas, read whole rather than by diff line.

## What I ran, and the environment

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep2-exec`, branch `review/q55-ep2-exec`. Binary built at `3354a90` with `cargo build` (inc1 landed, inc2 NOT landed), so every run below is the PRE-INC2 build; post-inc2 statements are derived from the code plus the amendment's stated rule and are marked as such. `TMPDIR` was `/tmp/claude-1000/.../rev-ep2-exec-tmp`, entirely outside any git repository, and every fixture sits under it.

Fixtures built (all under `$TMPDIR`, `BIN` the debug binary):

- `check13b/A`: `docs/plans/p.plan.toml` with no `[meta].primary` (defaults to `markdown`, confirmed against `src/plan/source.rs:primary_defaults_to_markdown_when_absent`), one `not-started` step, plus `docs/metrics/workflow.jsonl` copied verbatim from this repository's own log (253 records at this commit, confirmed carrying converged rounds for `triager-runs-only-on-findings` at lines 175-176 of that log), plus `docs/plans/p.ledger.md` with a `## RESUME STATE` block.
- `check13b/B`: `docs/plans/p.md`, a hand-written Markdown Roadmap with `triager-runs-only-on-findings` at `complete` and its Step Detail heading renamed to match (so `plan::validate_plan` has no cross-reference problem to report).
- `check14g/B`: the same as `check13b/B` but the step is `in progress`, for 14g's fourth run.
- `check19b/root`: `docs/plans/x.plan.toml` (Markdown-primary, one `not-started` step), `notes/p.md` (a valid Markdown Roadmap for the same step), and an empty, valid `docs/metrics/workflow.jsonl`.

Repository guards at the reviewed commit, both green: `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` prints `up to date`; `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` prints `workflow invariants hold` at exit 0.

## `R2B-1` (high): accepted cost (iii) and check 19b both pin a "bound" that is false as written; the true version needs a flag neither of them states

Cost (iii), `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:269`: "`--source <root>/docs/plans/x.plan.toml --plan <root>/notes/p.md` greens today... CARVING THE CASE OUT WAS DECLINED because it would REVERSE `Q-55-noconvention`... THE BOUND, measured: the same layout is ALREADY refused in its no-`--source` spelling, so this removes a rescue rather than introducing a species." Check 19b, `:346`, closes with the same claim: "The same layout in its no-`--source` spelling is refused too, which is what makes this a removed rescue rather than a new species."

Read literally, "the same layout in its no-`--source` spelling" means the stated command with `--source` dropped and nothing else added: `agent-scaffold validate --plan <root>/notes/p.md --workflow`. Built exactly as cost (iii) describes and run that way:

```
$ "$BIN" validate --plan "$TMPDIR/check19b/root/notes/p.md" --workflow
no metrics log at .../check19b/root/notes/docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
.../check19b/root/notes/p.md: 1 steps, 0 open-questions items, valid
exit: 0
```

Not refused, before inc2 and, on the mechanism as specified, not after it either. With no `--source`, `src/main.rs:resolve_metrics_path`'s anchor is `--plan` alone, and `src/main.rs:project_root_of_source` finds no `docs/plans`-shaped ancestor of `notes/p.md`, so it falls back to the plan's own directory (`.../notes`) for BOTH the metrics default and, per the decided rule at `:169` ("root the containment predicate on the plan the check reads"), the containment root. A path built from the same fallback as the root it is checked against is trivially "under" that root regardless of whether the file at the far end exists, so the containment guard structurally cannot fire here; this is the identical reasoning cost (i) already gives at `:265` for the bare-filename case ("the wrong path is still inside the right project: containment is not correctness"). The only mechanism that ever turns this into a non-zero exit is inc3's general "`--workflow` cannot run" policy, which is a different failure (a missing-log hard error, not a `Q-55-endproperty` containment refusal) and does not exist yet at the point 19b is placed in the check list (before check 20's "AFTER INC3" heading).

The claim IS true for a DIFFERENT command that neither cost (iii) nor 19b states: dropping `--source` AND adding an explicit `--metrics` naming the project's own log.

```
$ "$BIN" validate --plan "$TMPDIR/check19b/root/notes/p.md" --metrics "$TMPDIR/check19b/root/docs/metrics/workflow.jsonl" --workflow
.../check19b/root/docs/metrics/workflow.jsonl: 0 records, valid
.../check19b/root/notes/p.md: 1 steps, 0 open-questions items, valid
.../check19b/root/notes/p.md vs .../check19b/root/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

Pre-inc2 this still greens (no containment mechanism exists yet), but once inc2's predicate lands it refuses: the checked plan's root is `.../notes` (same fallback as above, since no `--source`), the explicit `--metrics` path is `.../docs/metrics/workflow.jsonl`, which is NOT under `.../notes`, so the guard fires. This is a genuine `Q-55-endproperty`-style refusal, immediately after inc2, independent of the anchor/checked-plan rooting split (the two coincide here, since there is no `--source`).

This is not a new observation on my part; it traces to round 1's triage of `EX-3` (`docs/plans/workflow-enforcement-tier.reviews/endproperty-fold-r1-triage.md`, the "ONE QUALIFICATION I OWE THE PLANNER" paragraph), which measured exactly the `--metrics`-qualified variant and wrote the bound with that qualifier intact: "with `--plan .../fixC/notes/p.md --metrics .../fixC/docs/metrics/workflow.jsonl` and no `--source`, the anchor IS the plan... and an anchor-rooted inc2 refuses it too." The fix pass's own prescribed text for the new cost (triage's minimal-fix item 4: "...and that the same layout is already refused in its no-`--source` spelling") carried the conclusion into cost (iii) and into 19b without the `--metrics` clause that made it true, so what shipped is a strictly broader and false generalisation of a narrower, correctly-measured claim.

The consequence for executability: an implementer or reviewer running 19b's final clause exactly as written, on a correct and complete inc2 build, gets exit 0 with a silent-miss stderr note, not a refusal; the check's own text does not match what a correct build does. And the "removed rescue rather than a new species" framing, which is offered (alongside the primary `Q-55-noconvention`-reversal ground) as part of why carving out cost (iii) was declined, rests on a bound that does not hold for the plain no-flags-added reading of "the same layout."

MINIMAL FIX. In both places (`:269` and `:346`), name the flag the bound actually needs: "the same layout, with no `--source` but an explicit `--metrics` naming the project's own log, is already refused" (or, more simply, delete the clause in both places, since it is not load-bearing for the decision, `Q-55-noconvention`-reversal is the ground actually doing the work, and a false bound is worse than no bound). If 19b is meant to also assert the plain-no-`--source` silent-miss behaviour, that belongs with accepted cost (i)'s check (18), on the SAME reasoning check 18 already uses to separate "after inc1 alone" from "after inc3", not folded wordlessly into a check about a different cost.

## `R2B-2` (medium): `status --resume`'s new root rule leaves the ledger-fragment-plus-divergence combination unspecified, and no check exercises it

The `status --resume` bullet (`:192`, `Q-55-resumepairing`, human, 2026-08-02) reads: "a `--source` and a `--plan` both named must resolve to the SAME root or the block is omitted, and with one alone the anchor is the root, as today. TWO CASES REACH IT: an explicit `--ledger-fragment` outside that root, and the DEFAULT ledger under a divergent pairing..."

`--ledger-fragment` is independent of `--source`/`--plan` in the CLI: `src/main.rs:StatusArgs::ledger_fragment` carries `#[arg(long, requires = "resume")]`, nothing tying it to either plan flag, so `--source`, `--plan` and `--ledger-fragment` can all be given together, and one reachable combination is never addressed: an explicit `--ledger-fragment` naming a real, valid ledger for one of the two anchors, GIVEN alongside a `--source`/`--plan` pair that itself diverges. I confirmed the combination is a live CLI state today:

```
$ "$BIN" status --resume --source "$TMPDIR/check13b/A/docs/plans/p.plan.toml" \
    --plan "$TMPDIR/check13b/B/docs/plans/p.md" \
    --ledger-fragment "$TMPDIR/check13b/A/docs/plans/p.ledger.md"
## RESUME STATE

Branch: fixture-a-branch
Worktree: /tmp/does-not-matter/fixture-a
In-flight: review round 2 of 5, awaiting reviewer output.
exit: 0
```

Two readings of the text produce different code for this input once inc2 lands. Reading 1 (uniform): "the root" is computed once, from source/plan agreement-or-single-anchor, and used to check WHATEVER ledger path is in play (fragment or default); since A and B diverge here, there is no root, so the block is omitted regardless of what the fragment names, even though the fragment given is A's own, legitimately-rooted ledger. Reading 2 (scoped): the "both must resolve to the same root" sentence answers `Q-55-resumepairing`'s own stated target, "the DEFAULT-ledger LEAK" (the decision's own name), so it governs only the no-fragment case; an explicit `--ledger-fragment` keeps its own, narrower, single-anchor containment check (root of `source.as_ref().or(plan.as_ref())`, unaffected by whether the OTHER flag agrees), and this invocation would print A's block, since the fragment is genuinely under A's root.

Neither check 13b nor 14c exercises this combination: 14c's `--ledger-fragment` clause uses a single-project fixture (no divergence), and its "THIRD RUN" for the divergent pairing explicitly says "with no `--ledger-fragment` at all" (`:335`). So the two readings are not just theoretically different, they are both unfalsifiable against the acceptance-check set as written; a build committing to either one passes every check in the file.

MINIMAL FIX. One sentence at `:192` stating which reading governs: either "the agreement rule applies whether or not `--ledger-fragment` is given" (closing Reading 2), or "an explicit `--ledger-fragment` is checked only against its own anchor's root, independent of whether the other plan flag agrees" (closing Reading 1 out). Whichever is chosen, add one run to check 14c exercising it, since it is currently the only combination among `{--source, --plan} x {agree, diverge} x {fragment, default}` with no check at all.

## `R2B-3` (low): the fix pass's own summary paragraphs were not extended to the two decisions it adds

Both new decisions folded by this pass are absent from the two paragraphs that exist specifically to enumerate what widens inc2's scope.

The inc2 increment description (`:286`) names three decisions by q_id ("`Q-55-endproperty`... `Q-55-refusalscope` settled... the serialised reasons `Q-55-jsonreason` settled") and then states "`status --resume` omits the block" with no citation, even though that behaviour is now `Q-55-resumepairing`'s, not a restatement of the pre-existing per-surface pattern; and cost (iii) / `Q-55-conventionlesscost` is not mentioned in the increment description at all.

The risk-classification paragraph for inc2 (`:309`) is built explicitly around this kind of enumeration ("`Q-55-endproperty` SHARPENS THAT RATHER THAN ADDING A CLASS... `Q-55-refusalscope` ADDS A FACTOR... `Q-55-jsonreason` adds a second... it widens what the two rounds have to cover and a reviewer who checks only `next --json` has checked the guarded half") and stops at three decisions, the same three the increment description names. `Q-55-conventionlesscost` adds a new, previously-unrecorded false-positive layout (pinned by 19b) and `Q-55-resumepairing` adds a rule with its own genuine ambiguity (`R2B-2`), both of which are exactly the shape of thing this paragraph exists to flag ("a reviewer who checks only X has not checked Y"). Neither is named.

This did not cause a wrong build in anything I ran; it is a completeness gap in the document's own self-audit, not a behavioural defect, which is why it is low rather than higher.

MINIMAL FIX. One clause on each paragraph naming the two additional decisions, on the same pattern the existing three use.

## The governing question

COULD A COMPETENT IMPLEMENTER BUILD THE FIVE REGIONS FROM THIS TEXT, AND WOULD WHAT THEY BUILT DO WHAT IT CLAIMS?

For check 13b's three new clauses: YES. I built fixture A (Markdown-primary, absent `[meta].primary`) and fixture B (Roadmap row plus renamed Step Detail heading) exactly as specified and ran all three runs; each reproduces its stated pre-change observation precisely (first run: `workflow invariants hold` at exit 0; second run, typo'd `--source`: the same, `no source plan at ...` on stderr then the green; third run, this repository's own TOML-primary plan against its own Markdown projection: `workflow invariants hold` at exit 0, reading `docs/metrics/workflow.jsonl: 253 records, valid`). I also confirmed, by tracing `resolve_metrics_path` and `project_root_of_source`, that only a checked-plan-rooted implementation (not an anchor-rooted one) would refuse any of the three, which is what the surrounding prose claims.

For 14c's third run and 14g's fourth run: YES. Built on the same A/B fixtures (14g's B variant at `in progress`), both reproduce their stated pre-change reds exactly: `status --resume` with no `--ledger-fragment` prints A's `## RESUME STATE` block verbatim under B's plan at exit 0; `next` (and `next --json`, and `status --json`) on the same pairing at `in-progress` fabricate a full `state: converged` / `mark the step complete` instruction sourced from A's log, at exit 0, with no `metrics_absent_reason` or `no_active_loop_reason` field present at all (since the field does not exist pre-inc2). Both discriminate an anchor-rooted implementation from a checked-plan-rooted one, as claimed.

For cost (iii) and check 19b: PARTIALLY. The with-`--source` half is buildable and does what it claims (`R2B-1`'s first reproduction confirms the pre-inc2 green; the mechanism trace confirms the post-inc2 refusal). The bound offered alongside it, and pinned as 19b's own closing clause, is not something any build can satisfy as literally stated, because it is false (`R2B-1`).

For the `status --resume` framing: MOSTLY. The rule is determinate for every case an acceptance check actually exercises (one anchor alone; both agreeing; both diverging with no fragment), and I confirmed by direct execution that the divergent-default case really does leak pre-inc2 and really would be closed by the stated rule. One reachable combination (fragment plus divergence) is genuinely underdetermined and untested (`R2B-2`).

## Scratch hygiene

Everything ran under a scratch `TMPDIR` created for this review, entirely outside any git repository, and it was removed when the review finished. Directories left in `/tmp`: 0.
