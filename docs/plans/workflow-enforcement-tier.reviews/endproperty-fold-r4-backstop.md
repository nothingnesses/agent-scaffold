# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 4 BACKSTOP RE-CHECK of the `R4A-1` dismissal

Backstop: independent of the planner, of all seven reviewers, and of all four triagers. READ-ONLY with respect to the reviewed artifact; this file is the only thing written, and no fix is applied.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/backstop-ep4`, branch `backstop/q55-ep4`, at `dd54227`. Binary built here with `cargo build` (increment 1 in the tree, increment 2 absent).

POSTURE, stated because it governs how the rest of this file reads. I was briefed to try to OVERTURN the dismissal, not to confirm it. I rebuilt every fixture from the finding's and the triage's DESCRIPTIONS rather than from their commands, ran every probe myself, and spent most of the work trying to break the triage's three legs one at a time. Two of the three held under everything I could throw at them. The third did not.

## VERDICT: OVERTURNED

The dismissal rests on two legs, and the triage states both: (a) `R4A-1` is round 1's `EX-5` in a new spelling, already ruled `VALID`/`low`, site-corrected and fixed by a deletion that landed; and (b) "NO IN-SCOPE SITE SURVIVES ... I then read every added line of that diff looking for an affirmative claim about the predicate's REACH that this layout falsifies, and found none that has not already been ruled".

LEG (a) HOLDS. Leg (b) DOES NOT. Two affirmative claims that the fold ADDED, neither of them ruled in any of the four rounds, are falsified by this layout, and both are on the LEDGER half of the predicate, which no reviewer in four rounds reached and which the round 4 triage noticed in passing and did not test against the document's text:

- `:183`, the `next` bullet's wholly added closing sentence: "The predicate rooted on the checked plan catches that; an anchor-rooted one cannot, for the same reason it cannot catch the metrics case." On the nested layout it does not catch it. MEASURED below.
- `:229`, the added second member of the `ledger-not-this-project` enumeration: "or, on `next`, a DEFAULT ledger anchored on a `--source` belonging to a different project than the plan being projected". That names a condition which is NOT the containment condition stated in the same sentence's leading clause, and the two come apart exactly when the projects nest. Implemented literally it is the second condition `:161` explicitly REJECTS.

The round 1 remedy could not have covered either site: it was a pure deletion at the inc2 bullet (now `:280`), aimed at a claim about the VALIDATOR closing the end property, and `EX-5` never reached the ledger. So "already ruled" is true of the defect's MECHANISM and false of its in-scope SITES, and a dismissal on the ground that nothing in scope survives falls.

WHAT SURVIVES OF THE TRIAGE'S CORRECTIONS TO THE REVIEWER, in full, because the overturn does not rehabilitate the finding as filed. `R4A-1`'s stated CAUSE is wrong (target 3, reproduced below: the conventionless fallback is not required and is not the mechanism), and its claim that "Both check 11's and check 13b's asserted properties fail on that layout" is wrong (target 2's contrast fixture: check 13b's own fixture refuses correctly). Anyone acting on the overturn should act on the sites above, not on the finding's own suggested remedy of a fifth accepted cost framed on "root depth" and "a conventionless root".

## Method, and the one unbuilt step

Increment 2 does not exist, so its behaviour is derived, not observed. Three parts, two of them built:

1. THE ROOT. `src/main.rs:project_root_of_source`, applied to the canonicalised location of the plan the check reads. Built. Its convention branch tests each ancestor for `is_plans` (`ancestor.file_name()... == Some("plans")`) AND `under_docs` (`ancestor.parent().and_then(Path::file_name)... == Some("docs")`) and returns "the ancestor's grandparent"; its fallback is the last line, `parent.to_path_buf()`. EVERY ROOT BELOW IS MEASURED off the built binary, by asking it where the DEFAULT log is and reading the path out of its `no metrics log at <path>` note. Where a fixture's own log exists and so suppresses that note, I built a log-less probe copy of the same directory shape and measured on it (fixture `F1probe`).
2. THE RESOLVED ARTIFACT. `src/main.rs:resolve_metrics_path` (built: explicit value returned verbatim, else `project_root_of_source(anchor).join(METRICS_RELATIVE)` with the anchor `source.as_ref().or(plan.as_ref())`), and `src/main.rs:default_ledger_path` (built: `anchor.parent()...join(format!("{task}.ledger.md"))`), then `:157`'s "absolutising and canonicalising its longest existing ancestor and re-appending the components below it".
3. THE CONTAINMENT TEST. The only unbuilt step. A path-prefix comparison with no free parameters.

BOTH READINGS OF `:157` ARE EVALUATED AND THEY DO NOT SEPARATE ON ANY FIXTURE HERE. Reading A counts the path itself as its own longest existing ancestor when the leaf exists; reading B canonicalises the deepest existing DIRECTORY and re-appends the leaf lexically. Every fixture below contains ZERO symlinks (verified with `find <fixture> -type l | wc -l`, which returns `0`) and every resolved artifact in them EXISTS, so canonical and lexical coincide and the two readings return the same path. I checked this rather than assuming it, with `realpath` on the root and on both resolved artifacts:

```
$ realpath /tmp/claude-1000/bs-ep4/fix/F1/outer
/tmp/claude-1000/bs-ep4/fix/F1/outer
$ realpath /tmp/claude-1000/bs-ep4/fix/F1/outer/packages/projA/docs/metrics/workflow.jsonl
/tmp/claude-1000/bs-ep4/fix/F1/outer/packages/projA/docs/metrics/workflow.jsonl
$ realpath /tmp/claude-1000/bs-ep4/fix/F1/outer/packages/projA/docs/plans/A.ledger.md
/tmp/claude-1000/bs-ep4/fix/F1/outer/packages/projA/docs/plans/A.ledger.md
```

## The fixtures, built from descriptions, not from anyone's commands

`W` is this worktree, `AS` is `$W/target/debug/agent-scaffold`, `TMPDIR=/tmp/claude-1000/bs-ep4`, `S=$TMPDIR/fix`, `SLUG=triager-runs-only-on-findings`, and every run was made from `$S`, which `git rev-parse --show-toplevel` reports as `fatal: not a git repository` before the fixtures are built.

`F1`, THE CONVENTIONAL NESTED SHAPE (my independent build of the triage's shape 4, and the same shape as round 1's fixture D):

```sh
"$AS" scaffold --output-dir "$S/F1/outer" --write --force --principles default
"$AS" scaffold --output-dir "$S/F1/outer/packages/projA" --write --force --principles default
# outer's CHECKED plan: Markdown, at outer/docs/plans/outer.md, borrowed slug at `complete`,
# Step Detail heading renamed to match. outer-ip.md is the same at `in progress`.
sed -e "s/| \`example-step\` | not started |/| \`$SLUG\` | complete |/" \
    -e "s/### \`example-step\`:/### \`$SLUG\`:/" \
    "$S/F1/outer/docs/plans/TEMPLATE.md" > "$S/F1/outer/docs/plans/outer.md"
sed 's/| complete |/| in progress |/' "$S/F1/outer/docs/plans/outer.md" > "$S/F1/outer/docs/plans/outer-ip.md"
# projA: MARKDOWN-primary source, its OWN conventional 256-record log, its OWN ledger beside the source.
sed 's/^primary = "toml"/primary = "markdown"/' \
    "$S/F1/outer/packages/projA/docs/plans/TEMPLATE.plan.toml" > "$S/F1/outer/packages/projA/docs/plans/A.plan.toml"
mkdir -p "$S/F1/outer/packages/projA/docs/metrics"
cp "$W/docs/metrics/workflow.jsonl" "$S/F1/outer/packages/projA/docs/metrics/workflow.jsonl"
printf '# Ledger for projA\n\n## RESUME STATE\n\nMARKER-PROJA-SECRET-RESUME-LINE\n\n## Other\n\ntail\n' \
  > "$S/F1/outer/packages/projA/docs/plans/A.ledger.md"
```

Outer carries NO `docs/metrics/` of its own, so nothing here is a copied log at the checked plan's own conventional path. Both projects are laid out conventionally.

`F2`, THE SIBLING CONTRAST: the same two projects side by side (`F2/A` is a copy of `projA`, `F2/B/docs/plans/B.md` is a copy of `outer.md`).

`F3`, THE REVIEWER'S CONVENTIONLESS SHAPE: a TOML-primary `myplan.plan.toml` at `F3/repo` with the borrowed slug at `complete` and no `docs/` of its own, with a copy of `projA` at `repo/vendor/projA`.

`F1probe`, a log-less copy of `F1`'s inner `docs/plans` shape, built only so `projA`'s own root can be read off the binary.

## TARGET 1. Are `EX-5` and `R4A-1` one defect? YES, AND THE TRIAGE IS RIGHT ON THIS

I tried to separate them on three axes and failed on all three.

FIXTURE. Round 1's `EX-5` used "fixture D is `fixB` with `fixA` vendored at `vendor/a` and no log of its own", invoked as `validate --source fixD/vendor/a/docs/plans/TEMPLATE.plan.toml --plan fixD/docs/plans/TEMPLATE.md --workflow`. My `F1` is `outer` with `projA` at `packages/projA` and no log of its own, invoked as `validate --source .../packages/projA/docs/plans/A.plan.toml --plan .../outer/docs/plans/outer.md --workflow`. Those differ in two directory names. Both are conventional on both sides. I rebuilt `F1` without reading either agent's shell and it is fixture D.

MECHANISM. Identical, and I measured both roots rather than deriving one of them:

```
$ "$AS" validate --plan "$S/F1/outer/docs/plans/outer.md" --workflow
no metrics log at /tmp/claude-1000/bs-ep4/fix/F1/outer/docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
/tmp/claude-1000/bs-ep4/fix/F1/outer/docs/plans/outer.md: 1 steps, 0 open-questions items, valid
exit=0

$ "$AS" validate --source "$S/F1probe/outer/packages/projA/docs/plans/A.plan.toml" \
    --plan "$S/F1probe/outer/packages/projA/docs/plans/A.md" --workflow
no metrics log at /tmp/claude-1000/bs-ep4/fix/F1probe/outer/packages/projA/docs/metrics/workflow.jsonl; nothing to validate
```

so root(checked plan) is `.../F1/outer` and root(`--source`) is `.../packages/projA`, both through the CONVENTION branch of `src/main.rs:project_root_of_source` and not through its fallback. The resolved log is `.../F1/outer/packages/projA/docs/metrics/workflow.jsonl`, which is under `.../F1/outer` under both readings, so the predicate is silent. That is `EX-5`'s sentence word for word: "The log actually read ... IS under the checked plan's root ... so the predicate does not fire".

CONSEQUENCE. `R4A-1`'s consequence set is strictly LARGER than `EX-5`'s (`EX-5` reached the validator only; `R4A-1` adds `next`'s `ACTIVE LOOP`, and neither reached the ledger), but a larger consequence set from one predicate on one layout is one defect with more consumers, not two defects. I looked for a mechanism difference that would make them two and there is none.

SO LEG (a) OF THE DISMISSAL STANDS. What does NOT follow from it, and is where the dismissal overreaches, is that the round 1 RULING disposed of the sites `R4A-1`'s wider consequence set touches. It could not have: `EX-5` is a validator finding, its corrected site was the inc2 bullet's end-property claim, and its remedy was a deletion there.

## TARGET 2. DOES ANY IN-SCOPE AFFIRMATIVE CLAIM SURVIVE THAT THIS FALSIFIES? YES, TWO

### How in-scope was decided, and it was decided on the diff rather than on either agent's account

`git diff main HEAD -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` at `-U0` carries 29 added lines. I read all 29 and then took the four candidate sites to `--word-diff` so that a rewritten line cannot pass off predating text as added text. Two of the triage's own scope claims re-checked out exactly:

```
$ git diff -U0 main HEAD -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md \
    | grep -c "must never pair a plan source"
0
$ git diff -U0 main HEAD -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md \
    | grep -c "specific output the fix must make unreachable"
0
```

So `:104`'s end property and `:185`'s "the specific output the fix must make unreachable" are untouched by the fold and are out of scope, exactly as the triage says, and exactly as round 1 ruled `:104`. The round 1 deletion also landed: HEAD's inc2 bullet stops at "which is what makes the predicate reach a divergent `--source`/`--plan` pairing" and main has no such parenthetical at all, the whole `Q-55-endproperty` clause being the fold's own addition.

### SITE 1, `:183`. The `next` bullet's closing sentence, WHOLLY ADDED, and falsified

`git diff --word-diff` on that line shows the entire block as an insertion with no removed text inside it:

```
- `next`. ... In place of the loop, print the existing "no active review loop (`<reason>`)" line with a reason naming the unsafe pairing. {+THE LEDGER HAS A SECOND WAY TO BE UNSAFE ON `next`, and it follows from the root decision above rather than from a new rule: `default_ledger_path` is anchored `--source` first while the steps come from a TOML-primary `--source` else the Markdown `--plan` (...), so a Markdown-primary `--source` in one project beside a `--plan` in another resolves the ledger in the FIRST project while projecting the SECOND project's steps, and echoes one project's `## RESUME STATE` under another's plan on the DEFAULT ledger path. The predicate rooted on the checked plan catches that; an anchor-rooted one cannot, for the same reason it cannot catch the metrics case.+}
```

The sentence names a CONFIGURATION ("a Markdown-primary `--source` in one project beside a `--plan` in another"), states the consequence, and then asserts that the checked-plan-rooted predicate CATCHES it. `F1` is that configuration, and the predicate does not catch it.

```
$ "$AS" next --source "$S/F1/outer/packages/projA/docs/plans/A.plan.toml" \
             --plan "$S/F1/outer/docs/plans/outer-ip.md"
task: A
source: /tmp/claude-1000/bs-ep4/fix/F1/outer/docs/plans/outer-ip.md
metrics: 256 records

ACTIVE LOOP
  triager-runs-only-on-findings / triager-runs-only-on-findings-inc1  in progress -> mark-step-complete
  state: converged
  streak: 1/1
  rounds: 2/5
  isolation: unknown
  next: mark the step complete, re-render, and commit
  role: orchestrator
  prompt: .agents/prompts/orchestrator.md
  context:
    isolation_tier: unknown
    ledger: /tmp/claude-1000/bs-ep4/fix/F1/outer/packages/projA/docs/plans/A.ledger.md
  reminders:
    - Verify by running the tests and checks before marking the step complete.
    - Edit the step status in the plan source and re-render; never hand-edit the generated view.
  summary: step `triager-runs-only-on-findings` increment `triager-runs-only-on-findings-inc1` converged (streak 1/1); mark the step complete, re-render, and commit.

RESUME STATE (verbatim from the ledger):
## RESUME STATE

MARKER-PROJA-SECRET-RESUME-LINE
exit=0
```

THE ONE UNBUILT STEP ON THE LEDGER, computed by hand and stated so it can be checked by eye:

- root of the plan `next` reads (`outer-ip.md`, measured above, convention branch): `/tmp/claude-1000/bs-ep4/fix/F1/outer`
- resolved ledger, from `src/main.rs:run_next`'s `.unwrap_or_else(|| default_ledger_path(&task, &args.source, &args.plan))` beneath its `ledger_fragment`, with `default_ledger_path` returning the `<task>.ledger.md` BESIDE the anchor: `/tmp/claude-1000/bs-ep4/fix/F1/outer/packages/projA/docs/plans/A.ledger.md`, which the run itself prints on its `ledger:` context line
- the ledger IS under the root, under both readings, so `ledger-not-this-project` is silent, so `next` echoes `MARKER-PROJA-SECRET-RESUME-LINE` under outer's plan after increment 2 exactly as it does before it.

That is `projA`'s internal resume state injected into another project's agent brief, which is defect C's third case in the step's own words ("`status --resume` is not a wrong boolean at all but CONTENT INJECTION into an instruction that the receiving agent has been told is authoritative and to read first"), surviving the increment whose bullet says the predicate catches it.

NO ROUND HAS RULED THIS SENTENCE AGAINST THIS LAYOUT. `grep -n "catches that" *.md` over all eleven prior review and triage files returns nothing. The sentence WAS assessed once, in round 1, by the fidelity reviewer, and blessed: "The operative behavioural specification elsewhere in the SAME document (the `next` bullet's 'SECOND WAY TO BE UNSAFE' sentence, checks 13b and 14g) is correct and complete", endorsed by the round 1 triage as the correct member of the pair `FI-1` split. But that assessment was made on `FI-1`'s subject (the "case that survives anchoring" scope claim) against a SIBLING pairing, in the same round in which `EX-5` was measuring the nested one, and neither finding was run against the other's fixture. A blessing in another subject's ruling is not a ruling on this one, and the round 4 triage's site sweep names only `:280`, `:159` and `:309` and does not reach `:183`.

### SITE 2, `:229`. The `ledger-not-this-project` enumeration, ADDED, and internally inconsistent on this layout

Word-diff of that bullet, main to HEAD:

```
- `ledger-not-this-project`: {+the resolved ledger is not under the root of the plan this surface reads, which is either+} an explicit `--ledger-fragment`[-resolves-] outside [-the plan's-]{+it or, on `next`, a DEFAULT ledger anchored on a `--source` belonging to a different+} project [-root.-]{+than the plan being projected.+}
```

Main's bullet had ONE cause and it was a containment cause. The fold added the general rule plus a two-member enumeration, and the members are not the same kind of thing. The first ("an explicit `--ledger-fragment` outside it") carries the containment qualifier. The second ("a DEFAULT ledger anchored on a `--source` belonging to a different project than the plan being projected") is a PROJECT-IDENTITY condition with no containment qualifier at all. On `F1` the second member's stated condition holds in full and the variant does not fire, so the bullet contradicts its own leading clause.

THIS IS AN IMPLEMENTATION HAZARD AND NOT A WORDING PREFERENCE, which is what keeps it above pedantry. An implementer who builds the second member as written builds "the anchor's root and the checked plan's root differ", and `:161`, also added by this fold, REJECTS exactly that: "A SECOND CONDITION, on the anchor and the checked plan resolving to different roots: rejected as two conditions where one does the work". So the added vocabulary specifies a trigger the added mechanism section forbids, and the layout that makes the two disagree is one project nested inside another.

### The corroborating observation, flagged as adjacent and NOT filed as a finding

On `F1`'s inputs the fold's own added material gives TWO DIFFERENT ANSWERS FOR ONE LEDGER FILE. `:182`'s `Q-55-resumepairing` rule for `status --resume` is a two-root AGREEMENT test ("a `--source` and a `--plan` both named must resolve to the SAME root or the block is omitted"), and the two roots here differ (`.../packages/projA` versus `.../outer`, both measured above), so `status --resume` OMITS. `:183`'s rule for `next` is containment, which is silent, so `next` ECHOES. Same two flags, same `default_ledger_path` result, opposite answers:

```
$ "$AS" status --resume --source "$S/F1/outer/packages/projA/docs/plans/A.plan.toml" \
                        --plan "$S/F1/outer/docs/plans/outer-ip.md"
## RESUME STATE

MARKER-PROJA-SECRET-RESUME-LINE
exit=0            # today. After inc2: OMITTED, by the agreement rule.
```

`src/main.rs:run_resume` and `src/main.rs:run_next` resolve the identical path (`next::derive_task(&args.source, &args.plan)` then `default_ledger_path(&task, &args.source, &args.plan)` in the first, the same call beneath `ledger_fragment` in the second), so the divergence is entirely in the specified rules. It sits against `:179`'s claim that "The trigger is the SAME containment predicate ... The predicate is never re-implemented per surface (One source of truth)" and `:182`'s "the rule SUPPLIES a root rather than being re-implemented per surface": supplying a root and testing two roots for agreement are different predicates. I record this because it is what makes site 1's falsity consequential rather than academic (the stronger rule is already written down one bullet above the weaker one), and I do not file it: filing is a reviewer's job and this backstop's remit is the dismissal.

### Why neither site is any recorded residual

- NOT THE COPIED-LOG RESIDUAL (`:267`). That residual is a log copied INTO the checked plan's own `docs/metrics/`, and `F1`'s outer has no `docs/metrics/` at all. More to the point, `:267`'s whole mechanism is W3 joining "by bare slug" against the round records, and a ledger has no slug, no records and no join.
- NOT THE QUEUED IDENTITY WORK (`:269`), AND THIS IS THE PART THAT MATTERS MOST. That work is described in the document itself as "an optional `project: Option<String>` on `Round` (`src/metrics.rs`) and on `[meta]` (`src/plan/source.rs`), with `check_workflow_toml` pre-filtering the rounds when the plan declares an id". Filtering round records cannot change which FILE `src/main.rs:run_next` opens for the ledger, so project identity structurally cannot close the ledger half of this bound. The residual as recorded therefore has no queued owner for one of its two halves, and the round 4 triage's fallback position ("it is ALREADY there, as the queued project-identity work at `:269`") does not hold for the ledger.
- NOT COSTS (iii)/(iv) (`:271`). Those are over-refusals through the fallback. Target 3 shows no fallback is involved here, and the direction is the opposite one.

## TARGET 3. IS THE CONVENTIONLESS FALLBACK IRRELEVANT? YES. THE TRIAGE IS RIGHT AND THE REVIEWER IS WRONG

I built both shapes and the false green appears in both, so the fallback is sufficient and not necessary.

WITH NO FALLBACK ANYWHERE (`F1`, both projects conventional, both roots measured through the convention branch above), check 13b's invocation shape:

```
$ "$AS" validate --source "$S/F1/outer/packages/projA/docs/plans/A.plan.toml" \
                 --plan "$S/F1/outer/docs/plans/outer.md" --workflow
/tmp/claude-1000/bs-ep4/fix/F1/outer/packages/projA/docs/metrics/workflow.jsonl: 256 records, valid
/tmp/claude-1000/bs-ep4/fix/F1/outer/packages/projA/docs/plans/A.plan.toml: 1 steps, 0 questions, valid
/tmp/claude-1000/bs-ep4/fix/F1/outer/docs/plans/outer.md: 1 steps, 0 open-questions items, valid
/tmp/claude-1000/bs-ep4/fix/F1/outer/docs/plans/outer.md vs /tmp/claude-1000/bs-ep4/fix/F1/outer/packages/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

`workflow invariants hold` for outer's `complete` step, which has zero review evidence of its own, satisfied by a nested package's log. Root `.../F1/outer`, log under it, predicate silent under both readings.

WITH THE FALLBACK (`F3`, the reviewer's shape), the same green appears, so the reviewer's fixture is real and only its causal claim is wrong:

```
$ "$AS" validate --source "$S/F3/repo/myplan.plan.toml" --workflow
no metrics log at /tmp/claude-1000/bs-ep4/fix/F3/repo/docs/metrics/workflow.jsonl; nothing to validate   # root = .../F3/repo, via the fallback
...
$ "$AS" validate --source "$S/F3/repo/myplan.plan.toml" \
                 --metrics "$S/F3/repo/vendor/projA/docs/metrics/workflow.jsonl" --workflow
/tmp/claude-1000/bs-ep4/fix/F3/repo/vendor/projA/docs/metrics/workflow.jsonl: 256 records, valid
/tmp/claude-1000/bs-ep4/fix/F3/repo/myplan.plan.toml: 1 steps, 0 questions, valid
/tmp/claude-1000/bs-ep4/fix/F3/repo/myplan.plan.toml vs /tmp/claude-1000/bs-ep4/fix/F3/repo/vendor/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

THE CONTRAST THAT ISOLATES THE RULE (`F2`, the same two projects side by side rather than nested):

```
$ "$AS" validate --plan "$S/F2/B/docs/plans/B.md" --workflow
no metrics log at /tmp/claude-1000/bs-ep4/fix/F2/B/docs/metrics/workflow.jsonl; nothing to validate   # root = .../F2/B
$ "$AS" validate --source "$S/F2/A/docs/plans/A.plan.toml" --plan "$S/F2/B/docs/plans/B.md" --workflow
... /tmp/claude-1000/bs-ep4/fix/F2/A/docs/metrics/workflow.jsonl ... workflow invariants hold
exit=0
```

Log `.../F2/A/...` against root `.../F2/B`: NOT under it, so the predicate FIRES and increment 2 refuses. So the bound is the one the triage states, and I reached it independently: CONTAINMENT REFUSES A FOREIGN ARTIFACT EXACTLY WHEN IT LIES OUTSIDE THE CHECKED PLAN'S ROOT SUBTREE, and nesting is what puts it inside. The fallback is one way to widen the subtree and needs nothing of the sort to happen.

TWO CONSEQUENCES THE HUMAN WILL NEED. First, `F2` is check 13b's own fixture shape and it behaves as check 13b asserts, so `R4A-1`'s "check 13b's asserted property fails" does not hold and the triage's rebuttal of it is correct. Second, on the queued-work question the two agents disagreed about: the reviewer says the shared fallback root cause at `:271`, the triage says the project-identity work at `:269`. NEITHER IS RIGHT FOR THE WHOLE BOUND. The metrics half belongs with the identity work, as the triage says. The ledger half belongs with neither, for the reason given under target 2: identity fields on round records cannot change which ledger file `run_next` opens.

## What I tried in order to overturn, that did NOT work

Recorded with the same care as what did, since these are what make the parts of the dismissal that stand safe to rely on.

1. SEPARATING `EX-5` FROM `R4A-1` ON THE FIXTURE. Rebuilt round 1's fixture D from its description and got my `F1`. Two directory names apart. Failed.
2. SEPARATING THEM ON THE MECHANISM. Looked for any way the vendored case and the monorepo-package case could take different branches of `src/main.rs:project_root_of_source`. They take the same one, and I measured both roots to confirm rather than reading the code. Failed.
3. SHOWING THE FALLBACK IS REQUIRED, which would have rehabilitated `R4A-1`'s cause and its queued-owner claim. `F1` has no fallback on either side (both roots measured through the convention branch) and greens anyway. Failed, decisively.
4. SHOWING CHECK 11 OR CHECK 13b ACTUALLY FAILS AS FILED. `F2` is 13b's own layout and refuses correctly. Check 11's fixture is the scaffolded `$SCRATCH` against the agent-scaffold root, which are not nested. The finding reads a check heading as a universal quantifier over layouts its body does not cover, which is what the triage said. Failed.
5. ATTACKING THE THREE SITES THE TRIAGE DID NAME. `:280`'s "which is what makes the predicate reach a divergent `--source`/`--plan` pairing" reads as an enabling claim about why the rooting decision was taken, is true of the layout check 13b specifies, and round 1 deliberately preserved it while deleting the closure clause beside it; `:159`'s "rooted on the anchor, the END PROPERTY above would have been met by no increment" is a claim about the ANCHOR rooting and stays true; `:309`'s "the one that decides whether this increment closes the end property" is conditional. I could not falsify any of the three without reading them harder than their grammar supports. Failed, three times.
6. LOOKING FOR THE END PROPERTY OR `:185` IN THE FOLD'S DIFF, which would have put the finding's own cited sentences in scope. Both are absent from the `-U0` diff entirely. Failed, and the triage's claim re-checked out exactly.

What worked was none of the above: it was reading the 29 added lines for affirmative claims about the LEDGER rather than about the metrics log, which is the half `R4A-1` itself never reached and the half the triage measured and then set aside as "which the finding does not reach".

## SEPARATE JUDGEMENT, kept apart from the verdict: is recording one instance rather than the rule adequate?

NO, AND I HOLD THIS INDEPENDENTLY OF THE VERDICT ABOVE. It is a judgement, so I label it one and give the evidence rather than the conclusion alone.

The document records this bound through one instance (`:267`, a log COPIED into the checked plan's own `docs/metrics/`) with general reasoning attached to it ("the guard passes (the log IS under the fixture's root)"). Two triagers judged that adequate. The evidence against it is now four items long, and only the first is the rediscovery count the round 4 triage raised:

1. FOUR INDEPENDENT READERS HAVE REDISCOVERED THE BOUND FROM SCRATCH IN FOUR ROUNDS: `EX-5` in round 1, the cold-read lens in round 3, the adversarial lens in round 4, and this backstop. Each cost a lens or a round.
2. THE INSTANCE MISLEADS ABOUT THE CAUSE, MEASURABLY. The recorded instance's mechanism is a COPY into the plan's own conventional path; the general mechanism is a nested root subtree, which needs no copy. A reader who generalises from the instance gets the copy; the round 4 reviewer, who did generalise, reached for the only other root-related thing the document records (the fallback at `:271`) and got the cause wrong. That is not a careless reading, it is what the recorded material supports.
3. THE INSTANCE MISASSIGNS THE OWNER. Because it is recorded as a W3 bare-slug join problem, it is queued to project identity at `:269`, and identity cannot reach the LEDGER half at all. Recording the RULE ("the guard's reach ends at the checked plan's root subtree") would have made the ledger half visible as part of the same bound; recording the instance hid it for four rounds.
4. THE CONSEQUENCE SET KEPT GROWING AND EACH READER FOUND A DIFFERENT PART OF IT. Round 1 found the validator green. Round 4's reviewer added `next`'s `ACTIVE LOOP`. Round 4's triager added the ledger echo and recorded it as an aside. This backstop found that the ledger echo falsifies added text and that the same inputs get opposite answers on `next` and `status --resume`. A recording that leaves each reader to rediscover a different consequence of one bound is not doing the job the document assigns it at `:253`, "an implementer must NOT 'fix' them, and a reviewer must NOT raise them as defects".

The document's own standard, quoted at `:253` and again at `:255`, is that "a check that pins an accepted cost is worth more than a note asking people to remember it". By that standard the current recording is the weaker of the two forms and is instance-bound on top of it. The round 4 triage's option (3) is the right direction; on this evidence it needs a third clause beyond the two it proposes, naming the LEDGER half and the fact that the queued identity work does not own it.

## What the overturn leaves owed, stated as scope and not as a prescription

The triage's own escalation arithmetic is untouched by this: round 4 was already `new_valid`, the streak was already 0, and the cap is still 5. What changes is the finding count and one of the escalation options. `R4A-1` is in scope on two added sites (`:183` and `:229`), it is NOT in scope on the end property or on checks 11 and 13b, and its stated cause is wrong. Whether the remedy is a qualification, a deletion in the shape round 1 preferred, or the rule-for-instance replacement of the triage's option (3) extended to the ledger, is a decision for the fix pass and the human, not for this backstop.

## Scratch hygiene

Every fixture was built under `/tmp/claude-1000/bs-ep4/`, created for this re-check and removed after the evidence above was captured. Nothing was written to bare `/tmp`. DIRECTORIES LEFT IN `/tmp`: 0 (the harness-provided session scratch tree under `/tmp/claude-1000/` is not counted).
