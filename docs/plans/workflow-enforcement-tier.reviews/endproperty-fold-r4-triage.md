# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 4 TRIAGE

Triager: independent of the planner, of both round 4 reviewers, and of the round 1, 2 and 3 triagers. Read-only with respect to the reviewed artifact; this file is the only thing written.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-ep4`, branch `triage/q55-ep4`, at `dd54227`. Binary built here with `cargo build`.

REPRODUCTION POLICY. I accepted no reviewer fixture and no reviewer transcript. Every fixture below was built from the finding's DESCRIPTION, by hand, in my own scratch tree, and every run is mine. For the one unbuilt step (increment 2's containment test) I did not do a shell prefix match: I transcribed `src/main.rs:project_root_of_source` verbatim into a standalone prototype, implemented the sidecar's resolution rule ("absolutising and canonicalising its longest existing ancestor and re-appending the components below it") in BOTH available readings, implemented containment as a component-wise prefix match, and ran it on every fixture. Where a verdict differs between the two readings I say so.

THREE FINDINGS TRIAGED. One DISMISSED (already ruled, twice), two VALID.

## Verdicts

| id | verdict | final severity | ground |
| --- | --- | --- | --- |
| `R4A-1` | DISMISSED (already ruled) | high, counterfactual; BACKSTOP RE-CHECK OWED | The layout and the false green are round 1's `EX-5`, ruled then (`VALID`, `low`), site-corrected to the affirmative closure claim, fixed by a deletion that landed; reproduced again by round 3's cold-read lens, which declined to raise it, and confirmed by the round 3 triage. No affirmative claim in the fold's own added material is falsified that has not already been ruled, and both requirement sentences the finding leans on are untouched by `main..HEAD`. |
| `R4A-2` | VALID | medium (upheld) | Reproduced, and BROADER than filed: FIVE symlink placements produce the refusal, cost (ii) names one, and on the two log-side placements cost (ii)'s stated MECHANISM does not describe what happens at all. |
| `R4A-3` | VALID | low (upheld) | Reproduced. The finding's "two readings" leg is dead (round 3 settled the rule as categorical), but the false omission survives that and is closed by a three-word change. |

DEDUPLICATED VALID COUNT: 2. SEVERITIES: medium, low.

## `R4A-1` DISMISSED. It is round 1's `EX-5` in a new spelling, and its diagnosis of the cause is measurably wrong

### The mechanism claim reproduces, in full, and it is worse than the finding states

I rebuilt all three shapes and added a fourth of my own. Every root below was read off the built binary by asking it where the DEFAULT log is; every containment verdict was computed by the prototype, under both readings.

SHAPE 1, check 11's shape on a conventionless checked plan. A `myplan.plan.toml` I authored at `/tmp/claude-1000/tri-ep4/fix/repo`, one step, slug `triager-runs-only-on-findings`, `status = "complete"`, with a separate scaffolded project at `repo/vendor/projA` carrying a copy of this repository's 256-record log at its own conventional path. The checked plan's own `docs/` does not exist at all.

```
$ agent-scaffold validate --source .../fix/repo/myplan.plan.toml --workflow
no metrics log at /tmp/claude-1000/tri-ep4/fix/repo/docs/metrics/workflow.jsonl; nothing to validate
exit=0                                            # so the derived root is .../fix/repo

$ agent-scaffold validate --source .../fix/repo/myplan.plan.toml \
    --metrics .../fix/repo/vendor/projA/docs/metrics/workflow.jsonl --workflow
.../fix/repo/vendor/projA/docs/metrics/workflow.jsonl: 256 records, valid
.../fix/repo/myplan.plan.toml: 1 steps, 0 questions, valid
.../fix/repo/myplan.plan.toml vs .../fix/repo/vendor/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

Prototype: root `.../fix/repo`, log `.../fix/repo/vendor/projA/docs/metrics/workflow.jsonl`, UNDER root under BOTH readings, so the predicate is silent and increment 2 prints the same three lines at exit 0.

SHAPE 2, check 13b's shape with no explicit flag, and SHAPE 3, the same on `next` at `in progress`: both reproduce exactly as filed, `workflow invariants hold` at exit 0 and then `state: converged` / `streak: 1/1` / `rounds: 2/5` / `next: mark the step complete, re-render, and commit` at exit 0.

SHAPE 4 IS MINE AND IT IS THE ONE THAT MATTERS. `R4A-1` states the predicate is vacuous "whenever the checked plan's derived root is a directory that contains other projects, WHICH IS EXACTLY WHAT `Q-55-noconvention`'S FALLBACK PRODUCES for a plan at a repository root", and its "why this is not any of the recorded residuals" section rests on that: it argues costs (iii) and (iv)'s shared fallback root cause is recorded only in its OVER-refusal direction and that this is the same fallback making the root too WIDE. THE FALLBACK IS NOT REQUIRED AND HAS NOTHING TO DO WITH IT. I built check 13b's exact invocation shape with BOTH projects conventionally laid out, no fallback anywhere on either side, the foreign project simply nested one directory down:

```
outer/docs/plans/outer.md                              (checked plan, CONVENTIONAL, root = outer)
outer/packages/projA/docs/plans/A.plan.toml            (Markdown-primary --source, CONVENTIONAL)
outer/packages/projA/docs/metrics/workflow.jsonl       (256 records)

$ agent-scaffold validate --source outer/packages/projA/docs/plans/A.plan.toml \
    --plan outer/docs/plans/outer.md --workflow
.../outer/docs/plans/outer.md vs .../outer/packages/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

Prototype: root `.../outer`, log `.../outer/packages/projA/docs/metrics/workflow.jsonl`, UNDER root under both readings. And the LEDGER half goes with it, which the finding does not reach: on the same pairing `next` echoes project A's block, `RESUME STATE (verbatim from the ledger): ## RESUME STATE / MARKER-PROJA-BLOCK`, because A's ledger is also under the outer root, so `ledger-not-this-project` is silent too.

The contrast that isolates the real rule: two CONVENTIONAL projects side by side rather than nested (`sib/A` and `sib/B`) give root `.../sib/B` and log `.../sib/A/...`, NOT under root, REFUSE, under both readings.

SO THE ACTUAL BOUND IS SIMPLER AND WIDER THAN THE FINDING SAYS: containment refuses a foreign log exactly when it lies OUTSIDE the checked plan's root subtree. Any foreign log INSIDE that subtree is invisible to it. The conventionless fallback is one way to make the subtree large; a monorepo package, a vendored dependency, an `examples/` scaffold or a nested fixture tree is another, and needs no fallback at all. ANYONE ACTING ON `R4A-1` AS FILED WOULD QUEUE THE WRONG ROOT CAUSE, folding this into `project_root_of_source`'s fallback at `:271` where it does not belong.

### Why it is nonetheless dismissed

IT IS ROUND 1'S `EX-5`, ON AN ALL-BUT-IDENTICAL FIXTURE. `endproperty-fold-r1-triage.md` records `EX-5`'s reproduction as "fixture D is `fixB` with `fixA` vendored at `vendor/a` and no log of its own", run as `validate --source fixD/vendor/a/docs/plans/TEMPLATE.plan.toml --plan fixD/docs/plans/TEMPLATE.md --workflow`, giving `workflow invariants hold` at exit 0, with the triager's own words: "The log read is at the VENDORED project's own `docs/metrics/` path, so it belongs to a different project by the same filename convention the whole mechanism uses, and it IS under the checked plan's root, so the predicate does not fire. This is sharper than the copied-log residual." That is my shape 4, one round of review and three commits ago, and `fixD` is conventional exactly as my `outer` is. `R4A-1`'s own "IT IS NOT THE COPIED-LOG RESIDUAL" argument was therefore already made, and accepted, in round 1.

THE ROUND 1 RULING WAS ON THE SITE, NOT THE CLAIM, AND IT LANDED. Round 1 ruled `EX-5` `VALID`, `low`, corrected the site from the end property to the amendment's own affirmative claim ("and so what makes this increment close the step's end property rather than half of it"), and prescribed a pure deletion of that clause with an explicit instruction: "Do NOT take the finding's own prescription of adding a containment clause to the end property at line 112 ... A deleted claim cannot be falsified at an edge." I verified the deletion landed: `:280`'s inc2 bullet now reads "(`Q-55-endproperty`, which is what makes the predicate reach a divergent `--source`/`--plan` pairing)" and stops there.

ROUND 3 REPRODUCED IT AND DECLINED, AND THE ROUND 3 TRIAGE CONFIRMED. The cold-read lens wrote it out in full ("IS THE STEP'S END PROPERTY, AS STATED, ACTUALLY MET BY THE INCREMENTS AS SPECIFIED? No, and this is already ruled on rather than new"), listed "THE COPIED-LOG AND VENDORED-LOG GREENS versus the end property" under "Checked deliberately and NOT raised", and the round 3 triage confirmed it by name.

NO IN-SCOPE SITE SURVIVES. I checked this myself rather than inheriting it. `git diff main HEAD -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` contains no added or removed line carrying either requirement sentence the finding leans on: not `:104`'s "must never pair a plan source with a metrics log belonging to a different project and report success", and not `:185`'s "This is the specific output the fix must make unreachable". Both predate the fold and are untouched by it. I then read every added line of that diff looking for an affirmative claim about the predicate's REACH that this layout falsifies, and found none that has not already been ruled: `:280`'s reach claim is true (the checked-plan rooting does make the predicate reach the divergent pairing, on the sibling layout that check 13b actually specifies); `:159`'s "rooted on the anchor, the END PROPERTY above would have been met by no increment of this step" is a claim about the ANCHOR rooting and remains true, as round 1 already noted; `:309`'s "the one that decides whether this increment closes the end property" is conditional and was expressly allowed to stand in round 1. Check 13b itself specifies A and B as SIBLINGS (`--source A/docs/plans/p.plan.toml --plan B/docs/plans/p.md`), so its asserted refusal is correct for the fixture it names; the finding's claim that "check 13b's asserted property fails" reads a check heading as a universal quantifier over layouts its own body does not cover.

MY BRIEF ALSO LISTS IT OUT OF SCOPE by name, as "the end-property/copied-log tension", among the already-ruled residuals. I would have dismissed it on the record above without that instruction; I record the agreement rather than leaning on it.

### The plain statement asked for

DOES INCREMENT 2 AS SPECIFIED CARRY AN UNRECORDED FALSE GREEN? It carries a real one, and it is RECORDED ONLY THROUGH ONE INSTANCE. The mechanism is exactly as `R4A-1` describes and I reproduced every shape of it, including one stronger than any it filed. What the document records is the instance (a log COPIED into the checked plan's own `docs/metrics/`, `:267`) with the general reasoning attached to it ("the guard passes (the log IS under the fixture's root)", "Neither the anchor nor the refusal touches this", and the queued identity work at `:269` with C's measurement that identity is "the ONLY mechanism that separates two projects LEGITIMATELY SHARING ONE MERGED LOG, which no path mechanism can address"). A reader who generalises that paragraph arrives at the bound. A reader who does not, does not.

Whether that recording is ADEQUATE is a question two triagers have now answered yes and I am not entitled to re-answer on no new evidence, and I have none: round 1's fixture already covered the conventional nested case. But I do have one new datum, and the human should have it: THREE INDEPENDENT LENSES IN THREE SEPARATE ROUNDS HAVE NOW EACH REDISCOVERED THIS BOUND FROM SCRATCH (`EX-5` in round 1, the cold-read lens in round 3, the adversarial lens in round 4), and the third one got its cause wrong. Each individual re-raise was correctly ruled out of scope. The pattern is not about any one of them. A residual that costs a review round to rediscover every round is under-recorded whatever the scope rules say about each rediscovery, and the recording is instance-bound in a way that a two-sentence generalisation would close permanently. That is an option for the escalation below, not a finding here.

## `R4A-2` VALID, medium (upheld). Cost (ii)'s stated scope is one placement; the mechanism reaches five, and on two of them cost (ii)'s stated mechanism describes nothing

### In scope, stated plainly because the provenance test would say otherwise

Cost (ii)'s paragraph at `:257` and check 19 at `:339` are CONTEXT lines in `git diff main HEAD`, not added ones: they predate this fold. Under the four-condition out-of-scope precedent the earlier triages built, that would put them out. My brief overrides it explicitly ("A mismatch between a cost's stated scope and the mechanism's actual reach IS in scope"), and I rule on the brief. I record the tension so the human can see the ruling rests on an instruction rather than on the precedent.

No prior round raised any symlink placement beyond the `docs/plans` directory: a `grep -in symlink` over all nine earlier review and triage files returns three hits, none of them a placement finding. This subject is new to the loop.

### Reproduced, with the model's own control, and a fifth placement the finding missed

Six fixtures, each a scaffolded project with an empty log of its own, each verified GREEN AT EXIT 0 TODAY under `validate --source <P>/docs/plans/TEMPLATE.plan.toml --workflow` before any predicate reasoning. Roots for the plan-side fixtures were read off the binary against the canonical plan location; containment computed by the prototype under both readings.

```
P0  control, no symlink        root .../P0             log .../P0/docs/metrics/workflow.jsonl        SILENT   (both readings)
P1  plan FILE symlinked out    root .../shared-plans   log .../P1/docs/metrics/workflow.jsonl        REFUSE   (both readings)
P2  docs/metrics symlinked     root .../P2             log .../shared-metrics/workflow.jsonl         REFUSE   (both readings)
P3  log FILE symlinked         root .../P3             log .../shared-log/workflow.jsonl             REFUSE under reading A, SILENT under reading B
P4  docs/plans symlinked       root .../P4/elsewhere   log .../P4/docs/metrics/workflow.jsonl        REFUSE   (both readings)   <- cost (ii) itself
P5a docs symlinked, target
    NOT named `docs`           root .../shared-docs/plans  log .../shared-docs/metrics/workflow.jsonl  REFUSE (both readings)
P5b docs symlinked, target
    ALSO named `docs`          root .../other          log .../other/docs/metrics/workflow.jsonl     SILENT   (both readings)
```

P4 IS THE CONTROL ON MY MODEL, INDEPENDENTLY OF THE REVIEWER'S. It is accepted cost (ii)'s own layout and my prototype gives it the verdict the design pass MEASURED for it ("from reading its 37-record log to `exit=1 REFUSED`"). P1, P2, P5a and P3-under-reading-A get the identical verdict from the identical computation.

P5a IS MINE AND THE FINDING DOES NOT HAVE IT. `R4A-2` claims "there are four such placements and cost (ii) names one". There are at least five: a symlinked `docs` whose target is not itself named `docs` defeats the `plans`-under-`docs` test in `src/main.rs:project_root_of_source` ("the first ancestor whose own file name is `plans` and whose parent's file name is `docs`"), so the root falls back to the plans directory itself and the log falls outside it. P5b shows the mirror case cancelling correctly, which is what proves the rule is about DIVERGENCE and not about placement. THAT IS THE POINT: enumerating placements is the wrong frame, and it is the frame both cost (ii) and this finding use.

THE QUIET HALF IS MEASURED TOO: `status --source <P2 or P3 plan>` prints `metrics: <n> records` today and loses that half at exit 0 after inc2, with `next` losing the whole `ACTIVE LOOP` block. That is cost (ii)'s own "more expensive half", recorded for one placement only.

### Ruling

VALID, medium, upheld. Cost (ii) exists to tell two audiences something ("an implementer must NOT 'fix' them, and a reviewer must NOT raise them as defects", `:253`), and it cannot do that for four of the five placements it reaches. On P2 and P3 its stated mechanism ("the lexical default and the canonical guard disagree about which project the PLAN belongs to") is not merely narrow but inapplicable: the plan's project is unambiguous and it is the LOG that canonicalises out of the root. The concrete risk is not confusion but a wrong repair: an implementer meeting P2 and reading cost (ii) as plan-side-only has every reason to "fix" it by dropping canonicalisation on the log side, which reopens the symlinked-log spoof the guard exists to defeat.

NOT HIGH. It produces no wrong answer at runtime and falsifies no affirmative correctness claim; it is a scope statement that under-describes a knowingly accepted false positive. NOT LOW, because `:255`'s own standard ("a check that pins an accepted cost is worth more than a note asking people to remember it") is met for one placement out of five, and because the whole symlink class is otherwise framed in this document only as an ATTACK to be defeated (`:165`, "the GUARD is canonical so it cannot be spoofed by a symlinked source"), so nothing in the file tells a reader the same mechanism refuses the mirror-image legitimate layouts.

### The prescribed minimal fix

THIS IS A PROSE FIX, NOT A MECHANISM CHANGE, at sites 1 and 2. Site 3 is a specification decision and I flag it as one.

SITE 1, `:257`, the cost itself. Replace the LAYOUT framing with the RULE, keeping the measured layout as the example. Two edits and one added clause:

- heading: "A SYMLINKED `docs/plans` DIRECTORY BECOMES A FALSE POSITIVE ON THE PREDICATE" to "A SYMLINK ON THE PLAN'S OR THE LOG'S PATH BECOMES A FALSE POSITIVE ON THE PREDICATE". 5 words changed, net +2.
- mechanism clause: "disagree about which project the plan belongs to" to "land under different roots". 4 words changed, net -4. This is what makes it cover the log-side placements.
- one added clause after the measured sentence, stating the rule with NO enumeration so it cannot go stale when a sixth placement is found: that the cost is the DIVERGENCE and not the layout, that any symlink making the canonicalised checked plan and the canonicalised resolved log fall under different roots produces it on either side, and that the `docs/plans` directory is the placement that was MEASURED rather than the population. About 40 words.

AUTHORED WORD COUNT, SITE 1: about 40 net added, 9 changed. Nothing smaller works: deleting the layout noun without the added clause leaves a cost whose only example is plan-side, and enumerating the five placements reproduces the stale-enumeration class rounds 2 and 3 both spent findings on.

SITE 2, `:339`, check 19. Add ONE pinned log-side layout (a symlinked `docs/metrics`, which is reading-independent and so pins cleanly) beside the existing `docs/plans` one, asserting the same refusal and the same quiet omission. About 25 words. This is the site that matters most, by the file's own standard at `:253`.

SITE 3, `:157`, the resolution rule, and IT IS A DECISION. "absolutising and canonicalising its longest existing ancestor" does not say whether the path ITSELF counts as its own longest existing ancestor, and P3 flips on the answer: under reading A a symlinked log file is refused, under reading B the tool reads a log whose real content lives outside the project and says nothing. Explorer A's own record (`metrics-path-anchor-to-source.md:67`) repeats the same wording and does not settle it. MINIMAL FIX: insert ", the path itself when it exists," after "longest existing ancestor". 6 words. MY RECOMMENDATION IS READING A, because it is the same reason already decided for the source side (`:165`, the guard is canonical so it cannot be spoofed by a symlink) applied to the log side, and because the alternative is a silent false green rather than a loud false refusal (Fail loudly). But it decides behaviour, so it is the human's to take, not mine to prescribe.

SITE COUNT MEASURED. Authored: 3, all in `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (`:157`, `:257`, `:339`). Mechanical: 1 file, `docs/plans/agent-scaffold.md`, regenerated by `cargo run -- render docs/plans/agent-scaffold.plan.toml` and never hand-edited, at `:1552`, `:1652` and `:1734`. NOT a site: `:303`, the inc2 risk paragraph's "(accepted cost (ii), the symlinked `docs/plans` directory)", which reads correctly as an example once the cost states a rule. NOT a site: `docs/plans/agent-scaffold.plan.toml`, which carries `Q-55-mechanism` and `Q-55-noconvention` question records but no symlink text, and `docs/plans/agent-scaffold.ledger.md`, whose only symlink-adjacent line is the design-pass narrative. Searches run over the whole tracked plan population: `grep -rn "SYMLINKED \`docs/plans\` DIRECTORY\|is a SYMLINK to a sibling directory\|symlinked \`docs/plans\`"` and `grep -rn "longest existing ancestor"` across `docs/plans/agent-scaffold.steps/`, `agent-scaffold.plan.toml`, `agent-scaffold.md` and `agent-scaffold.ledger.md`.

## `R4A-3` VALID, low (upheld). A nonexistent `--plan` suppresses a correctly located resume block, on a flag inert today

### In scope, and half the finding is already answered

The `Q-55-resumepairing` bullet at `:182` IS in the fold's added material, so this attacks a sentence the fold authored. Squarely in scope.

The finding presents TWO readings and asks the triager to pick. THAT LEG IS DEAD AND I RULE IT SO. Round 3's cold-read lens built this same rule against a nonexistent `--source` and reasoned it out: "`:192`'s sentence is categorical, 'must resolve to the SAME root OR THE BLOCK IS OMITTED', and a path with no canonical root does not resolve to the same root, so the block is omitted. That is determinate ... The round 2 triage relied on the same categorical reading to reject `R2B-2`'s indeterminacy claim, so it is a consistent reading of the file rather than a charitable one." There is ONE reading, not two, so `R4A-3`'s second leg (that the lexical reading would introduce a second lexical/canonical split on one surface) does not arise. What that ruling also establishes is the finding's own consequence: under the settled reading the block IS omitted.

The cold-read lens then declined to raise it, on a one-line judgement that omission "is the safe direction on a best-effort surface at exit 0". No triager has ruled on it. So it is open on the merits and I rule it.

### Reproduced

```
$ agent-scaffold status --resume --source .../P0/docs/plans/TEMPLATE.plan.toml \
    --plan .../P0/docs/plans/does-not-exist.md
## RESUME STATE

MARKER-P0-BLOCK
exit=0                                  # identical to the same run with no --plan at all
```

The inertness is confirmed by symbol rather than by inference: `src/main.rs:run_resume` computes the task with `next::derive_task(&args.source, &args.plan)` and the path with `default_ledger_path(&task, &args.source, &args.plan)`, and both resolve the anchor as `source.as_ref().or(plan.as_ref())`, so the `--plan` value is never consulted when a `--source` is given. Increment 2 gives that inert flag the power to suppress the output, for a project whose source is good and whose ledger is exactly where it belongs.

### Ruling

VALID, low, upheld. The consequence is real and unrecorded: a currently-working invocation stops printing a correct block, silently, at exit 0, and the agent that invokes `status --resume` loses the resume anchor the whole defect-C narrative is about. It is not covered by cost (iv), which is `status --resume` on the (iii) PAIR with both paths existing, and it is not covered by `Q-55-resumecost`'s "Accept as (iv), queue the shared cause", because the shared cause queued at `:271` is `src/main.rs:project_root_of_source`'s FALLBACK and no fallback is reached here: a path that does not exist yields no canonical root at all.

There is a sharper ground the finding does not give, and it is what keeps this from being dismissed as noise. On the VALIDATOR the same input class is handled deliberately and defensibly: `:159` records that the rule "covers the TYPO'D `--source`, a nonexistent `--source` beside a readable `--plan`, where the root comes from the plan that WAS read while the log still comes from the lexical derivation on the path that was not", and refusing there is right because the LOG really did come from the wrong derivation. On `status --resume` the mirror input discards a good anchor because a bad one was also named, and nothing about the ledger is wrong. The same input class gets a reasoned answer on one surface and an unreasoned one on the other.

LOW AND NOT MEDIUM: the trigger is a typo or an unrendered Markdown projection, the direction is withholding rather than fabricating, the exit code is 0, and the user recovers by fixing the path.

### The prescribed minimal fix

At `:182`, change "a `--source` and a `--plan` BOTH NAMED must resolve to the SAME root or the block is omitted" to "a `--source` and a `--plan` THAT BOTH EXIST must resolve to the SAME root or the block is omitted". THREE WORDS CHANGED, ONE NET ADDED.

That is the whole fix, and it is worth stating what it buys. It preserves today's behaviour on the narrow case; it leaves cost (iv) and check 14c's third run untouched, since those have both paths existing; it makes the agreement test well-defined by construction, because canonicalisation now always succeeds for both operands, which removes the very ambiguity round 3 had to resolve by appeal to categoricality; and it leaves the no-leak property intact in the mirror case, since with a nonexistent `--source` the ledger is derived beside that nonexistent path and `run_resume` prints `no ledger at <path>; nothing to resume` rather than another project's block.

IT IS STILL A SPECIFICATION DECISION AND I SAY SO. It decides what the rule means for an input the rule did not consider, rather than correcting a description of settled behaviour. The alternative worth naming is to resolve a nonexistent anchor as far as possible (the rule `:157` already gives the metrics path), which would print the block for a same-directory typo and omit it for a typo that also changes directory. I prefer the three-word change: it is smaller, it is strictly behaviour-preserving against today, and reusing `:157`'s rule for a root derivation extends that rule to a job it was not specified for.

SITE COUNT MEASURED. Authored: 1, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:182`. Mechanical: 1, `docs/plans/agent-scaffold.md:1577`, regenerated by render. `grep -rn "must resolve to the SAME root"` over `docs/plans/agent-scaffold.steps/`, `agent-scaffold.plan.toml`, `agent-scaffold.md` and `agent-scaffold.ledger.md` returns exactly those two lines; the `Q-55-resumepairing` receipt in `docs/metrics/workflow.jsonl` carries options and a choice only, no restated rule, so nothing is owed there.

## Backstop

I DISMISSED ONE FINDING, `R4A-1`, AND I RATE IT `high`. Had it been in scope and unruled I would have upheld the reviewer's `high`: it is a false green on the two routes the increment exists to close, it defeats three of inc2's specified consumer behaviours (the validator's refusal, the metrics omission, the ledger omission), and it needs no unusual layout beyond one project nested inside another. A BACKSTOP RE-CHECK IS THEREFORE OWED, and I state it explicitly rather than leaving it implied.

WHAT THE RE-CHECK MUST TARGET, so it is not a re-run of my reasoning by someone who has already read it. Three questions, each answerable by reading rather than by building, since the mechanism half is not in dispute:

1. Are `EX-5` (round 1) and `R4A-1` the same defect? Compare `endproperty-fold-r1-triage.md`'s fixture D against my shape 4 above and decide independently whether the vendored-nested green and the nested-package green are one thing.
2. Was the round 1 site correction right, and is the deletion at `:280` the whole in-scope fix? In particular, re-check my claim that no added line in `git diff main HEAD -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` carries an affirmative reach claim this layout falsifies.
3. Independently confirm or refute my measurement that the conventionless fallback is NOT required, since that is the one substantive correction I make to the reviewer and it is what stops the bound being queued to the wrong root cause.

## The escalation, which is now certain

THE ARITHMETIC, stated so the human does not have to derive it. The streak was 0 of 2 required going into round 4. Round 4 is `new_valid` (two findings), so the streak stays 0. Round 5 is the cap. Even a perfectly clean round 5 reaches 1 of 2. CONVERGENCE WITHIN THE CAP IS IMPOSSIBLE and the shortfall is exactly one clean round.

WHAT THE ESCALATION IS ABOUT, on my rulings: two small fixes (about 65 authored words for `R4A-2`, four for `R4A-3`, across four authored sites in one file plus one regenerated projection), one specification decision inside each of them, a one-round convergence shortfall, and one standing question the human may want to close deliberately.

THE OPTIONS AS I SEE THEM.

(1) FIX THE PROSE SCOPE ONLY, run round 5, waive the one-round shortfall. Cost: one round, about 69 authored words, one waiver record and its escalation receipt. Buys: the fold closes. Risk: inc2 is `risky` and the two-round bar exists because its correctness property is a negative; against that, seven independent lenses across four rounds have now found ZERO defects in the mechanism itself and every valid finding in the loop has been in prose. The residual risk is prose risk, and prose risk is what the round would be re-testing.

(2) FIX THE PROSE SCOPE AND KEEP LOOPING TO THE CAP without a waiver. Cost: at least one round and realistically two, and it cannot succeed: the cap is 5 and a clean round 5 reaches a streak of 1. This option does not exist as a route to convergence; it only defers the same waiver. I name it so it can be ruled out explicitly rather than by omission.

(3) FIX THE PROSE SCOPE AND ALSO GENERALISE THE IN-ROOT BOUND (the `R4A-1` subject), as a deliberate human decision rather than a triage ruling. Concretely: at `:267`, replace the copied-log paragraph's INSTANCE framing with its RULE, that the containment guard's reach ends at the checked plan's root subtree, so any foreign log inside it, whether copied to the plan's own conventional path or sitting at a NESTED project's own conventional path, passes the guard and joins by bare slug; and state, at `:271` or beside the queued identity work at `:269`, that this direction belongs to the QUEUED IDENTITY work and NOT to costs (iii) and (iv)'s fallback root cause. About 45 authored words, 1 authored site, 1 regenerated projection. Buys: it stops a bound that three independent lenses in three rounds have each rediscovered from scratch, and it corrects the record before an implementer inherits the wrong cause from `R4A-1`. Risk: this project has measured that a fix pass which AUTHORS prose manufactures the next round's finding; against that, this particular edit REPLACES an instance with a rule rather than widening an enumeration, which is the same deletion-shaped move rounds 2 and 3 both endorsed, and a rule has no edge to be falsified at.

(4) CHANGE THE MECHANISM INSIDE INC2, replacing containment with a same-project-root test (derive the log's own project root and require equality with the checked plan's). Cost: it reopens a predicate the human decided under `Q-55-endproperty` and `Q-55-mechanism`, it needs its own measurement and probably its own decision, and it has at least one known collision, check 13's "a `..` that stays INSIDE the root is allowed and produces the correct W3 result", plus any legitimate in-root log at a non-conventional path. NOT RECOMMENDED, and I flag it as the one option here that is a MECHANISM change rather than a prose change, which is a materially different thing to put to a human than the others.

(5) SPLIT THE IN-ROOT DIRECTION INTO A LATER INCREMENT OR INTO THE VALIDATION-CONSTRAINTS STEP. Mostly a no-op: it is ALREADY there, as the queued project-identity work at `:269`, with explorer C's build as its evidence and C's own measurement that identity is the only mechanism that can separate two projects sharing one log. The only live content in this option is the CORRECTION in option (3)'s second half, that it is queued under identity and not under the fallback.

MY RECOMMENDATION: (1) PLUS (3), taken together, then one final round, then waive the one-round shortfall if that round is clean.

The reasoning, judged against the plan's own principles. Fail loudly and One source of truth both point at (3): a residual recorded through one instance is a source of truth that three separate readers have failed to read, and the cheapest thing that cannot be misread is the rule itself. Minimal by default points at keeping (3) to a replacement rather than an addition, which is why I priced it at 45 words in one paragraph rather than as a fifth accepted cost. Safe on existing projects is what makes `R4A-2` worth fixing at all, since four of the five refused layouts are legitimate ones that work today. And the argument for waiving rather than looping is the loop's own evidence: the bar exists because inc2's correctness property is a negative established only by adversarial construction, round 4 SPENT a lens on exactly that construction, and it returned no mechanism defect, only two prose-scope defects and one re-raise. A fifth round buys another prose sweep of an artifact that has had six of them.

WHAT I WOULD NOT DO: take `R4A-1`'s own suggested remedy of a fifth accepted cost stating the bound as a function of "the checked plan's root depth" and a "conventionless root". I measured that framing wrong, and recording it would put a false cause into the file at the site an implementer reads first.

## Out of scope, confirmed out

- LINE LENGTH, wording, counts, citations, enumeration staleness: none raised by either round 4 lens, none raised here, and the round 4 verification lens re-counted all ten enumerations and returned zero.
- The present-tense `src/main.rs` claims increment 1 falsified: deferred to the post-inc3 documentation-currency pass, unchanged.
- Accepted costs (i) and (ii) THEMSELVES and (iii) and (iv) AS STATED: untouched. Only cost (ii)'s stated SCOPE is ruled on, which the brief permits explicitly.
- Increments 1 and 3: untouched.
- Already-ruled residuals: `R2B-2` (the `--ledger-fragment` interaction, distinct from `R4A-3`, which has no fragment at all and a different trigger), `R2B-3` (the summary paragraphs naming three decisions), the end-property versus copied-log tension (`R4A-1`, above), and the dismissed `R3B-1`. None re-raised.
- The six human decisions: settled, and none re-opened. `Q-55-resumepairing`'s CHOSEN option ("Close it in inc2") is untouched by `R4A-3`, which is about what the closing rule says for an input it did not consider.

## Scratch hygiene

Every fixture was built under `/tmp/claude-1000/tri-ep4/`, created for this triage and removed after the evidence above was captured; the prototype and its scripts live in the harness-provided session scratchpad under `/tmp/claude-1000/`. Nothing was written to bare `/tmp`. DIRECTORIES LEFT IN `/tmp`: 0.
