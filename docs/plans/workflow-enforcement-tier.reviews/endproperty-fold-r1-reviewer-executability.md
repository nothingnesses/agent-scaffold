# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 1, reviewer: EXECUTABILITY

Reviewed: commit `c131292` on `plan/q55-endproperty` (a prose-only amendment to `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` plus the regenerated `docs/plans/agent-scaffold.md`), read against `main` at `6632630`.

Lens: could a competent implementer build increment 2 from this sidecar alone, and would what they actually built close the step's stated end property. Region: the whole inc2 specification, its acceptance checks, and the end-property statement.

## What I ran, and the environment

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep-exec`, branch `review/q55-ep-exec`. Binary built at that commit with `cargo build` (inc1 landed, inc2 NOT landed), so every run below is the PRE-INC2 build. `TMPDIR` and every fixture were placed under `/tmp/claude-1000/ex-rev-scratch`.

Fixture setup used by every reproduction below (`W` is the worktree, `BIN=$W/target/debug/agent-scaffold`, `SC=/tmp/claude-1000/ex-rev-scratch`):

```sh
"$BIN" scaffold --output-dir "$SC/fixA" --write --force --principles default
"$BIN" scaffold --output-dir "$SC/fixB" --write --force --principles default
# fixA/docs/plans/TEMPLATE.plan.toml : [meta].primary  "toml" -> "markdown"
# fixB/docs/plans/TEMPLATE.md        : Roadmap row `example-step` | not started -> `triager-runs-only-on-findings` | complete
# fixB/docs/plans/TEMPLATE.md        : Step Detail heading `### `example-step`:` -> `### `triager-runs-only-on-findings`:`
mkdir -p "$SC/fixA/docs/metrics"
cp "$W/docs/metrics/workflow.jsonl" "$SC/fixA/docs/metrics/workflow.jsonl"   # 250 records, includes converged rounds for the borrowed slug
printf '# Ledger for fixture A\n\n## RESUME STATE\n\nFIXTURE-A-SECRET-RESUME-LINE: branch review/q55-ep-exec, worktree /fixtureA\n\n## Other\n\ntail\n' \
  > "$SC/fixA/docs/plans/TEMPLATE.ledger.md"
```

THE DIVERGENT PAIRING REPRODUCES EXACTLY AS THE AMENDMENT DESCRIBES IT, on `validate`, on `next`, and on `status`. Run from `$SC`:

```
$ "$BIN" validate --source "$SC/fixA/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB/docs/plans/TEMPLATE.md" --workflow
/tmp/.../fixA/docs/metrics/workflow.jsonl: 250 records, valid
/tmp/.../fixA/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
/tmp/.../fixB/docs/plans/TEMPLATE.md: 1 steps, 0 open-questions items, valid
/tmp/.../fixB/docs/plans/TEMPLATE.md vs /tmp/.../fixA/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The typo'd-`--source` variant of the same check (13b's second run) reproduces too: with `--source "$SC/fixA/docs/plans/TYPO.plan.toml"` (nonexistent) the run prints `no source plan at ...` on stderr and then the same `workflow invariants hold` at exit 0. So the amendment's central factual claim, that an anchor-rooted predicate could never fire on this pairing and the end property would be met by no increment, is CORRECT and is not the subject of any finding below. The decision receipt is real: `grep 'Q-55-endproperty' docs/metrics/workflow.jsonl` returns one `type:"decision"` record dated `2026-08-02` with the three options and the chosen `Root on the plan the check reads`, and `grep -o '"q_id":"Q-55[^"]*"' docs/metrics/workflow.jsonl | sort | uniq -c` returns exactly the seven receipts the amended provenance list names.

Repository guards at the reviewed commit: `render docs/plans/agent-scaffold.plan.toml --check` prints `up to date` (exit 0), and `validate --source docs/plans/agent-scaffold.plan.toml --workflow` prints `workflow invariants hold` (exit 0). The regenerated projection is in sync with the sidecar.

## `EX-1` (high): no acceptance check pins the divergent-pairing METRICS case on `status` or `next`, so an inc2 that roots the projections on the ANCHOR passes the whole check set with the fabricated `next` instruction still standing

The amendment specifies the projections correctly in prose. Line 187 of the sidecar: "The trigger in all three cases is the SAME containment predicate the validator's refusal uses (the canonically-derived root of the plan THAT SURFACE READS, and whether the resolved artifact lives under it)." The acceptance checks do not evidence it.

Every check that exercises an unsafe METRICS pairing on `status` or `next` uses an EXPLICIT `--metrics` naming a log outside the anchor's root, which an ANCHOR-rooted predicate catches just as well:

- check 14b: `next --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl`, run from the agent-scaffold root.
- check 14c: the same shape for `status`.
- check 14e: "Re-run check 14b with `--json`" and "Re-run check 14c with `--json`".
- check 14f: run (b) is "the unsafe pairing", that is 14b's; the fourth run is "an explicit `--metrics` outside the plan's root naming a file that does not exist".
- check 14g: runs 1 to 3 use an explicit `--ledger-fragment`. The FOURTH run is the only projection check that uses the divergent pairing at all, and it asserts only the LEDGER: "gives `ledger-not-this-project` and prints no line of A's block".

Check 13b, the one check built on the divergent pairing, is a `validate` check. So the sidecar's own summary at line 305, "rooting the guard on the anchor is a defect that check 11 passes over and only check 13b catches", is true for the validator and false for the projections: on `status` and `next` the anchor rooting is caught by NOTHING in this file.

The consequence is not hypothetical. With fixture B's borrowed slug at `in progress` instead of `complete` (call it `fixB2`) and NO explicit `--metrics`, the pre-inc2 build emits, at exit 0:

```
$ "$BIN" next --source "$SC/fixA/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB2/docs/plans/TEMPLATE.md"
task: TEMPLATE
source: /tmp/.../fixB2/docs/plans/TEMPLATE.md
metrics: 250 records

ACTIVE LOOP
  triager-runs-only-on-findings / triager-runs-only-on-findings-inc1  in progress -> mark-step-complete
  state: converged
  streak: 1/1
  rounds: 2/5
  ...
  next: mark the step complete, re-render, and commit
  ...
  summary: step `triager-runs-only-on-findings` increment `triager-runs-only-on-findings-inc1` converged (streak 1/1); mark the step complete, re-render, and commit.

RESUME STATE (verbatim from the ledger):
## RESUME STATE

FIXTURE-A-SECRET-RESUME-LINE: branch review/q55-ep-exec, worktree /fixtureA
exit: 0
```

That is field-for-field the output line 193 says "the fix must make unreachable". An implementer who roots the LEDGER predicate on the checked plan (check 14g's fourth run forces them to) and the METRICS predicate on the anchor gets a green run of every check in the file and leaves this output intact, minus only the resume block.

The anchor rooting is the likely mistake rather than a contrived one. `src/main.rs:resolve_metrics_path` takes `(&args.metrics, &args.source, &args.plan)` and hands back a path; the anchor pair is right there at the call site in `src/main.rs:run_next`. The path of the plan the surface actually reads is NOT in scope at that point: `run_next`'s plan selection (`if let Some(source_plan) = toml_source(&args.source)?`) keeps only a display STRING (`path.display().to_string()`), not a `Path`, so rooting on the checked plan needs new plumbing while rooting on the anchor needs none.

`status` has the same hole. Same fixtures, no explicit `--metrics`:

```
$ "$BIN" status --source "$SC/fixA/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB/docs/plans/TEMPLATE.md"
plan: 1 steps (1 complete); 0 open-questions items
metrics: 250 records
exit: 0
```

This is squarely against the file's own stated standard for inc2's red cases at line 311: "all four are owed because one predicate with several consumers on two surfaces is not evidenced by testing one consumer on one surface".

MINIMAL FIX. Extend check 14g's fourth run, or add a sibling check, so that check 13b's divergent pairing is asserted on `next` and on `status` for the METRICS half as well as the ledger half: with fixture B's borrowed slug at `in-progress` and NO explicit `--metrics`, `next` must print none of the `ACTIVE LOOP` block and no record count, `next --json` must give `"metrics_absent_reason": "log-not-this-project"` with `"no_active_loop_reason": "metrics-not-this-project"`, `status --json` must give the same `metrics_absent_reason`, and both exit 0. State that this run, and not check 14b, is what separates an anchor-rooted projection from a checked-plan-rooted one.

## `EX-2` (medium): check 13b's fixture preconditions, executed literally, do not produce check 13b's own stated pre-change observation

Check 13b says: "Before inc2 it prints `workflow invariants hold` at exit 0". Built exactly as the check specifies, it does not. Three measured gaps, all in the check's own preconditions.

(a) THE MARKDOWN FIXTURE NEEDS ITS STEP DETAIL HEADING RENAMED, WHICH THE CHECK DOES NOT SAY. The check asks only for "a SECOND fixture B whose MARKDOWN Roadmap carries the borrowed slug `triager-runs-only-on-findings` at `complete`". Doing exactly that and nothing else:

```
$ "$BIN" validate --source "$SC/fixA/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB/docs/plans/TEMPLATE.md" --workflow
/tmp/.../fixB/docs/plans/TEMPLATE.md: Roadmap step `triager-runs-only-on-findings` has no matching `### `triager-runs-only-on-findings`` Step Detail heading
/tmp/.../fixB/docs/plans/TEMPLATE.md: Step Detail `example-step` has no matching Roadmap row
exit: 1
```

`validate_plan` runs on a Markdown `--plan` whenever the `--source` is not TOML-primary, which is exactly the mode 13b requires, so the run exits 1 for a reason unrelated to the pairing both before AND after inc2. Note that the file already handles the analogous requirement for the TOML substrate in defect B's demonstration, which spells out `cp docs/plans/TEMPLATE.steps/example-step.md docs/plans/TEMPLATE.steps/triager-runs-only-on-findings.md`; 13b's Markdown fixture is missing the counterpart instruction.

(b) "A REAL `docs/metrics/workflow.jsonl` WITH RECORDS" IS NOT ENOUGH; THE LOG MUST CARRY CONVERGED ROUNDS FOR THE BORROWED SLUG. With fixture A's log replaced by four valid records that belong to a different slug (`grep 'agents-md-drift-guard' ... | head -20`), and everything else per 13b:

```
$ "$BIN" validate --source "$SC/fixA2/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB/docs/plans/TEMPLATE.md" --workflow
/tmp/.../fixB/docs/plans/TEMPLATE.md vs /tmp/.../fixA2/docs/metrics/workflow.jsonl: Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records and no covering waiver; ...
exit: 1
```

Pre-inc2 exit 1, post-inc2 exit 1. The exit-code half of the check stops discriminating entirely, which matters because `Q-66` (invoked at line 311) requires the round report to state which pre-fix revision produced the red.

(c) THE THIRD RUN NEEDS A FIXTURE STATE NEITHER A NOR B IS IN. "A THIRD RUN PINS THE NO-REGRESSION SIDE ...: `--source` and `--plan` naming the same project's two substrates, with the source TOML-primary, exits 0 and reads that project's own log". Fixture A is MARKDOWN-primary by the check's own construction, so it cannot supply the TOML-primary source. Fixture B is an untouched scaffold, which has no `docs/metrics/` at all (check 2 of this same file records that), so it cannot supply "that project's own log":

```
$ "$BIN" validate --source "$SC/fixB/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB/docs/plans/TEMPLATE.md" --workflow
no metrics log at /tmp/.../fixB/docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
...
exit: 0
```

Exit 0, but the workflow check never ran, so the run evidences nothing about the rooting.

MINIMAL FIX. Add to 13b's fixture description: fixture B's Step Detail heading must be renamed to the borrowed slug alongside its Roadmap row; fixture A's log must be one carrying CONVERGED ROUNDS for `triager-runs-only-on-findings` (naming agent-scaffold's own log as the source, as defect B's demonstration does); and name the fixture the third run uses, either a third fixture that is TOML-primary WITH its own log or the agent-scaffold repository itself.

## `EX-3` (medium): the "STRICT SUPERSET" claim is false in both directions, and one direction is a new hard refusal of a correct same-project invocation that no accepted cost records

Line 167 claims: "It is a STRICT SUPERSET of the anchor-rooted text rather than a replacement, since in TOML-primary mode the checked plan IS the anchor and the rule is unchanged there; it differs only when a `--source` is given and is not TOML-primary, which is exactly the gap."

The region named is right. The relation is not a superset, and the region is not "exactly the gap": inside it the two rootings disagree in BOTH directions, and only one of the two disagreements is the divergent pairing.

DIRECTION 1, THE NEW RULE REFUSES A CORRECT SAME-PROJECT PAIRING THE ANCHOR RULE ALLOWS. Fixture C is one project: a Markdown-primary `docs/plans/TEMPLATE.plan.toml`, its own `docs/metrics/workflow.jsonl`, and a Markdown plan at `notes/p.md`. Today this reads its own log and greens:

```
$ "$BIN" validate --source "$SC/fixC/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixC/notes/p.md" --workflow
/tmp/.../fixC/docs/metrics/workflow.jsonl: 250 records, valid
/tmp/.../fixC/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
/tmp/.../fixC/notes/p.md: 1 steps, 0 open-questions items, valid
/tmp/.../fixC/notes/p.md vs /tmp/.../fixC/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The root the amendment's rule would derive from the CHECKED plan is measurable today through the anchored default, since `src/main.rs:project_root_of_source` is the same walk:

```
$ "$BIN" validate --plan "$SC/fixC/notes/p.md" --workflow
no metrics log at /tmp/.../fixC/notes/docs/metrics/workflow.jsonl; nothing to validate
```

so the checked plan's root is `.../fixC/notes` (the no-`docs/plans` fallback to the plan's own directory), while the anchor's root is `.../fixC` (first line of the previous run). `.../fixC/docs/metrics/workflow.jsonl` is NOT under `.../fixC/notes`, so the amendment's predicate REFUSES a correct single-project invocation that the converged anchor-rooted text allows. That is a new false positive of the same species as accepted cost (ii) and it appears nowhere in "The two accepted costs", in the risk classification, or in any acceptance check.

DIRECTION 2, THE NEW RULE ALLOWS A CORRECT PAIRING THE ANCHOR RULE REFUSES. With fixture B given its own log, and an explicit `--metrics` naming it:

```
$ "$BIN" validate --source "$SC/fixA/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB/docs/plans/TEMPLATE.md" --metrics "$SC/fixB/docs/metrics/workflow.jsonl" --workflow
/tmp/.../fixB/docs/plans/TEMPLATE.md vs /tmp/.../fixB/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

B's plan against B's own log, the correct pairing. The checked plan's root is `.../fixB`, so the new rule allows it. The anchor's root is `.../fixA` (measured above as the default it derives), so an anchor-rooted predicate would REFUSE it. The new rule is better here, and it is still not a superset.

This is not a wording quibble in a lens that asks whether an implementer builds the right thing. The superset sentence is what tells the implementer and the next reviewer that the converged no-regression story survives the amendment unexamined; direction 1 shows it does not, and the acceptance checks have no run that would find it, because 13b's third run deliberately picks TOML-primary mode where the two rootings agree by construction.

MINIMAL FIX. Replace the superset sentence with the accurate statement: the two rootings coincide in TOML-primary mode and where no `--source` is given, and they differ whenever a `--source` is given that is not TOML-primary, in both directions, the divergent cross-project pairing being one of the two. Then either record the direction-1 case (a same-project `--plan` outside the project's `docs/plans` and outside the directory chain above `docs/metrics`) as a third accepted cost beside (i) and (ii) with its exit code stated, or add it to 13b's third run with a decided expected outcome.

## `EX-4` (low): line 187 says the root comes from "the plan THAT SURFACE READS" in "all three cases", which line 171 contradicts for `status --resume`

Line 187: "The trigger in all three cases is the SAME containment predicate the validator's refusal uses (the canonically-derived root of the plan THAT SURFACE READS, and whether the resolved artifact lives under it)." The three cases are the three bullets beneath it: `status`, `status --resume`, `next`.

Line 171: "`status --resume` is the one surface that reads NO plan (`src/main.rs:run_resume` derives `<task>` from the source-or-plan filename and reads only the ledger), so it has no checked plan to root on and its root falls back to the source-then-plan anchor `default_ledger_path` already uses".

Confirmed against the code: `src/main.rs:run_resume` reads only the ledger, and `src/main.rs:default_ledger_path` anchors `source.as_ref().or(plan.as_ref())`. An implementer working from line 187 alone has no root to derive for `status --resume`; line 171, sixteen paragraphs earlier, is where the answer is. They will find it, which is why this is low rather than higher, but the two sentences say different things about the same surface and only one of them is beside the bullet list that specifies the behaviour.

MINIMAL FIX. Add the carve-out to line 187's parenthetical, or to the `status --resume` bullet: for that surface, which reads no plan, the root comes from the source-then-plan anchor.

## `EX-5` (low): the end property is stated without qualification while the mechanism is containment-based, and a NESTED-project divergent pairing still greens after inc2 as specified

The end property at line 112 is unamended and unqualified: "`validate --workflow` must never pair a plan source with a metrics log belonging to a different project and report success."

Inc2's predicate is containment, so a divergent pairing whose ANCHOR project is nested inside the CHECKED project survives it. Fixture D is fixture B with fixture A vendored at `vendor/a`:

```
$ "$BIN" validate --plan "$SC/fixD/docs/plans/TEMPLATE.md" --workflow
no metrics log at /tmp/.../fixD/docs/metrics/workflow.jsonl; nothing to validate       # checked plan's root is .../fixD

$ "$BIN" validate --source "$SC/fixD/vendor/a/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixD/docs/plans/TEMPLATE.md" --workflow
/tmp/.../fixD/vendor/a/docs/metrics/workflow.jsonl: 250 records, valid
/tmp/.../fixD/docs/plans/TEMPLATE.md vs /tmp/.../fixD/vendor/a/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The log actually read, `.../fixD/vendor/a/docs/metrics/workflow.jsonl`, IS under the checked plan's root `.../fixD`, so the predicate does not fire and fixD's `complete` step keeps its green on a vendored project's evidence. This is the same class the file already concedes at line 271 ("A log COPIED into a project's own `docs/metrics/` still joins by bare slug and still greens ... the guard passes (the log IS under the fixture's root)") and it is the residual queued to project identity, so the mechanism is not the thing to change. What is worth changing is the sentence the increments are measured against, since the amendment now argues explicitly at line 167 that the rooting is what makes the end property met by an increment.

MINIMAL FIX. One clause on the end property recording that it is met by CONTAINMENT, so a log inside the checked plan's own root is out of its reach whether it got there by copying or by nesting, and that the residual is the queued project-identity work.

## The governing question

COULD A COMPETENT IMPLEMENTER BUILD INC2 FROM THIS SIDECAR ALONE? Yes for the validator, and yes in substance for the projections. I walked the specification and wrote out the code I would produce, and it is determinate at every branch that matters: `src/main.rs:run_validate` already binds `toml_primary` immediately above the four-arm match (verified in the tree, line 165's claim is accurate), so `checked = if toml_primary.is_some() { args.source } else { args.plan }` is available before the match exactly as the text says; `src/main.rs:run_status` and `src/main.rs:run_next` make the same selection through `toml_source(&args.source)` with `--plan` as fallback (verified, line 171's claim is accurate); and the typo'd-`--source` case resolves the way line 167 says it does (measured above: root from the `--plan` that WAS read, log from the lexical derivation on the path that was not). I found no branch where two readings of the text produce different code, other than `EX-4`'s `status --resume` root, which the text answers elsewhere.

WOULD WHAT THEY BUILT CLOSE THE END PROPERTY? For `validate --workflow`, yes, subject to `EX-5`'s containment qualification: rooted on fixture B, the resolved log `.../fixA/docs/metrics/workflow.jsonl` is not under `.../fixB`, the predicate fires, and both of check 13b's red runs turn non-zero. Check 13b is genuinely RED against the current binary (measured: exit 0 with `workflow invariants hold`), it distinguishes the two rootings as it claims to, and it is not a check that would pass before the change, once its fixture is corrected per `EX-2`.

For the AGENT-FACING half, no, not reliably. The prose specifies it correctly and the check set does not evidence it (`EX-1`), so a conforming implementation can pass every acceptance check in the file while `next` still prints `state: converged` and `next: mark the step complete, re-render, and commit` for a foreign project on the default path. On this file's own reasoning at line 135, that is the more urgent half.

Nothing in the amended region instructs the implementer to do something the project's rules forbid elsewhere. The amendment adds no commands; check 13b's "From anywhere" is compatible with the `TMPDIR`-outside-a-repository preamble (all runs above were made from a scratch directory outside every repository involved), and no new text touches the `just scaffold-self` or `nix fmt` prohibitions.

## Scratch hygiene

Everything ran under `/tmp/claude-1000/ex-rev-scratch`, which was removed when the review finished. Directories left in `/tmp`: 0.
