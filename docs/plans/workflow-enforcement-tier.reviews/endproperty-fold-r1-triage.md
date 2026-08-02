# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 1, TRIAGE

Triager: independent of both reviewers and of the planner who authored `c131292`. Read-only with respect to the reviewed artifact; this file is the only thing written.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-ep`, branch `triage/q55-ep`, cut from the reviewed tip `c131292` so every reviewer citation resolves against the reviewed text. Binary built at that commit (`cargo build`, inc2 NOT landed), so every run below is a PRE-INC2 measurement and every post-inc2 statement is derived from the code plus the amendment's specified rule, stated as such. All fixtures under `TMPDIR=/tmp/claude-1000/tri-ep-scratch`, removed at the end.

Repository guards re-run at the reviewed commit and both green: `render docs/plans/agent-scaffold.plan.toml --check` prints `up to date` (exit 0); `validate --source docs/plans/agent-scaffold.plan.toml --workflow` prints `workflow invariants hold` (exit 0). The projection is in sync with the sidecar, so a sidecar fix must be re-rendered but the projection is not an independent authored site.

Population searched for every site count below: all 95 sidecars in `docs/plans/agent-scaffold.steps/`, plus `docs/plans/agent-scaffold.plan.toml`, plus `docs/plans/agent-scaffold.ledger.md`. Site counts separate AUTHORED sites (a human edits them) from MECHANICAL ones (`render` regenerates `docs/plans/agent-scaffold.md`).

## Verdicts

| id | verdict | severity (was) | ground |
| --- | --- | --- | --- |
| `EX-1` | VALID | high (high) | Reproduced: no acceptance check asserts the divergent pairing's METRICS half on a projection, so an anchor-rooted metrics predicate passes the whole file with the fabricated instruction standing. |
| `EX-2` | VALID | medium (medium) | All three fixture gaps reproduced literally; 13b's stated pre-change observation is not what its own preconditions produce. |
| `EX-3` | VALID | high (medium) | Both directions constructed and measured; the superset claim is false, the new false positive is unrecorded, and the false ground is in the record of what was presented to the human. |
| `EX-4` | VALID | low (low) | Citations resolve; the amendment's own sharpening made line 187's parenthetical false for one of the three surfaces it quantifies over. |
| `EX-5` | VALID | low (low) | Nested case reproduced; in scope not against the unamended end property but against the amendment's new "closes the step's end property" claim, so the site is corrected. |
| `FI-1` | VALID | high (medium) | Reproduced on pure default resolution; site set under-measured by the reviewer (2 named, 5 measured), and the fifth site is a BEHAVIOURAL claim that a measured surviving leak falsifies. |

DEDUPLICATED VALID COUNT: 6. SEVERITY LIST: `EX-1` high, `EX-3` high, `FI-1` high, `EX-2` medium, `EX-4` low, `EX-5` low.

No finding was dismissed. Nothing was merged: `EX-4` and `FI-1`'s fifth site both concern `status --resume` and should be fixed in one pass, but they are different claims at different lines with different fixes, so merging them would hide one of the two. No high or critical was dismissed, so NO BACKSTOP RE-CHECK IS OWED.

## What I reproduced, and the fixtures

`BIN` is the debug binary at `c131292`, `SC=/tmp/claude-1000/tri-ep-scratch`, and every run was made from `$SC`, outside every repository involved.

- `fixA`: `scaffold --write --force --principles default`, then `[meta].primary` `"toml"` to `"markdown"`, then this repository's 250-record `docs/metrics/workflow.jsonl` copied to `fixA/docs/metrics/`, plus a `TEMPLATE.ledger.md` carrying a `## RESUME STATE` block with a unique marker line.
- `fixB`: the same scaffold with `example-step` replaced by `triager-runs-only-on-findings` in BOTH the Roadmap row and the Step Detail heading of `docs/plans/TEMPLATE.md`, status `complete`. `fixB2` is `fixB` at `in progress`.
- `fixBa`: `fixB` built the way 13b LITERALLY specifies, Roadmap row only.
- `fixA2`: `fixA` with its log replaced by four records belonging to a different slug.
- `fixC`: one project, Markdown-primary `docs/plans/TEMPLATE.plan.toml`, its own `docs/metrics/workflow.jsonl`, and a Markdown plan at `notes/p.md`.
- `fixD`: `fixB` with `fixA` vendored at `vendor/a` and no log of its own.
- `fixE`: an untouched scaffold, used for 13b's third run.

The central factual claim of the amendment reproduces exactly, on all three surfaces, with NO explicit `--metrics` and NO explicit `--ledger-fragment`:

```
$ "$BIN" validate --source "$SC/fixA/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB/docs/plans/TEMPLATE.md" --workflow
.../fixB/docs/plans/TEMPLATE.md vs .../fixA/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0

$ "$BIN" next --source "$SC/fixA/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB2/docs/plans/TEMPLATE.md"
  state: converged / streak: 1/1 / rounds: 2/5 / next: mark the step complete, re-render, and commit
  RESUME STATE (verbatim from the ledger): FIXTURE-A-SECRET-RESUME-LINE ...
exit: 0

$ "$BIN" status --source "$SC/fixA/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB/docs/plans/TEMPLATE.md"
metrics: 250 records
exit: 0
```

The typo'd-`--source` variant reproduces too (`no source plan at .../TYPO.plan.toml` on stderr, then `workflow invariants hold` at exit 0), so the third stated ground of the decision, that the chosen rule reaches the typo'd `--source` while a two-root comparison cannot, HOLDS. The decision receipt is real: `grep -o '"q_id":"Q-55[^"]*"' docs/metrics/workflow.jsonl | sort | uniq -c` returns exactly the seven receipts the amended provenance list names, and `Q-55-endproperty` carries `"ts":"2026-08-02"`, the three options, and `"chosen":"Root on the plan the check reads"`.

## `EX-1` VALID, high (unchanged). The projections' metrics half is unevidenced, and one new sentence over-claims that it is

Reproduced. The check-set survey is accurate against the reviewed file: 13b (line 328) is a `validate` check; 14b (330) and 14c (331) pass an explicit `--metrics`; 14e (333) re-runs those two with `--json`; 14f's (334) unsafe run is 14b's and its fourth run passes an explicit `--metrics`; 14g's (335) fourth run is the only projection check built on the divergent pairing and it asserts the LEDGER only (`ledger-not-this-project`, no line of A's block). All of the explicit-`--metrics` checks fire under an ANCHOR-rooted predicate as well, because the anchor is the fixture source and the named log is outside the fixture (I re-derived this rather than assuming it: `resolve_metrics_path` returns an explicit `--metrics` verbatim, and the anchor root is `project_root_of_source(--source)`, which is the fixture root).

So an implementation that roots the LEDGER predicate on the checked plan and the METRICS predicate on the anchor passes every check in the file while leaving the `next` output measured above standing, minus only the resume block. That output is field-for-field what line 193 says "the fix must make unreachable".

ONE CORRECTION TO THE FINDING, WHICH STRENGTHENS IT. 14g's fourth run inherits 13b's fixture, whose borrowed slug is at `complete`. At `complete`, `next` prints "no active review loop (all steps complete)" and NO `ACTIVE LOOP` block whatever the rooting, so that run cannot exhibit the metrics defect even incidentally. The discriminating case needs the step at `in-progress`, which no divergent-pairing check in the file uses.

ONE CITATION NARROWED. The finding says rooting on the checked plan "needs new plumbing while rooting on the anchor needs none", citing that `src/main.rs:run_next`'s plan selection keeps only a display string (`path.display().to_string()`). The string claim is correct, but `args.source` and `args.plan` are both still in scope at the `resolve_metrics_path(&args.metrics, &args.source, &args.plan)` call, so what is missing is not the paths but the record of WHICH branch was taken. The finding does not depend on the stronger reading.

ONE SITE THE FINDING DID NOT NAME. Line 305, new text from this amendment, says "rooting the guard on the anchor is a defect that check 11 passes over and only check 13b catches". That is true for `validate` and false for `status` and `next`, where nothing catches it. It is the sentence a later reviewer would rely on to conclude the check set covers the rooting.

MINIMAL FIX, MEASURED. Two authored sites, both existing text, no new check number and so no disturbance to the "FOUR red cases" count (see the ruling below).

1. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:335`, check 14g's fourth run. Extend it, roughly 60 new words, with: the same pairing with B's step at `in-progress` instead of `complete` must print none of the `ACTIVE LOOP` block and no record count; `next --json` must give `"metrics_absent_reason": "log-not-this-project"` with `"no_active_loop_reason": "metrics-not-this-project"`; `status --json` on the same pairing must give the same `metrics_absent_reason`; both exit 0; and this run, not 14b, is what separates an anchor-rooted projection from a checked-plan-rooted one. Nothing smaller works: the discriminating case needs the DEFAULT metrics path, a step state that produces a loop, and assertions on both the human and the machine surface, and no existing check has any two of the three together.
2. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:305`. Narrow "only check 13b catches" to name 14g's fourth run beside it. Six words.

SITE COUNT MEASURED: 2 authored, both in `workflow-enforcement-tier.md`; 2 mechanical in `docs/plans/agent-scaffold.md`. `grep -rln "14g" docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.plan.toml` returns one file, and `grep -rn "only check 13b\|check 11 passes over"` over the same population plus the ledger returns one line. No other sidecar and no part of `agent-scaffold.plan.toml` carries either claim.

## `EX-2` VALID, medium (unchanged). 13b's preconditions do not produce 13b's own stated pre-change observation

All three sub-claims reproduced literally.

(a) Built exactly as 13b specifies, Roadmap row only:

```
$ "$BIN" validate --source "$SC/fixA/.../TEMPLATE.plan.toml" --plan "$SC/fixBa/.../TEMPLATE.md" --workflow
.../fixBa/docs/plans/TEMPLATE.md: Roadmap step `triager-runs-only-on-findings` has no matching `### `triager-runs-only-on-findings`` Step Detail heading
.../fixBa/docs/plans/TEMPLATE.md: Step Detail `example-step` has no matching Roadmap row
exit: 1
```

Exit 1 for a reason unrelated to the pairing, before AND after inc2. The comparison the finding draws is exact: defect B's demonstration (lines 86 to 92) DOES spell out the TOML-substrate counterpart, `cp docs/plans/TEMPLATE.steps/example-step.md docs/plans/TEMPLATE.steps/triager-runs-only-on-findings.md`, so the file already knows this class of precondition has to be written down.

(b) With a log carrying records for a different slug, which is all "a real `docs/metrics/workflow.jsonl` with records" requires:

```
.../fixB/docs/plans/TEMPLATE.md vs .../fixA2/docs/metrics/workflow.jsonl: Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records and no covering waiver; ...
exit: 1
```

Exit 1 before and after, so the exit-code half of the check stops discriminating, which is exactly what `Q-66` at line 311 forbids for a red case.

(c) Fixture A is Markdown-primary by 13b's own construction, so it cannot be the third run's TOML-primary source; and an untouched scaffold has no `docs/metrics/` at all, so it cannot be "that project's own log":

```
$ "$BIN" validate --source "$SC/fixE/.../TEMPLATE.plan.toml" --plan "$SC/fixE/.../TEMPLATE.md" --workflow
no metrics log at .../fixE/docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
exit: 0
```

Exit 0, with the workflow check never running, so the run evidences nothing.

MINIMAL FIX, MEASURED. One authored site, `workflow-enforcement-tier.md:328`, three clauses.

1. Fixture B's Step Detail heading is renamed alongside its Roadmap row, mirroring the wording defect B's demonstration already uses. About 12 words.
2. Fixture A's log is one carrying CONVERGED ROUNDS for the borrowed slug, not merely "records" (naming this repository's own log, as defect B's demonstration does). About 8 words.
3. The third run names its fixture: a third project that is TOML-primary AND has its own log, or this repository itself. About 10 words.

SHARED EDIT WITH `EX-3`. The third run's rationale clause, "it is the reason this rule is a superset rather than a replacement", is a site of `EX-3`'s falsified claim and is a DELETION there. Do (3) and that deletion in the same edit: the run keeps its no-regression purpose and drops the property it cannot establish.

SITE COUNT MEASURED: 1 authored, 1 mechanical. Check 13b's text exists in `workflow-enforcement-tier.md` and its projection only; `grep -rn "13b" docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.plan.toml` finds it in no other sidecar and not in the plan source.

## `EX-3` VALID, RE-SEVERITISED to high. The superset claim is FALSE; the region claim is TRUE; the human is owed a correction

I constructed both counterexamples myself rather than reusing the reviewer's, and both hold.

THE REGION CLAIM IS TRUE, and I checked it exhaustively against `run_validate` rather than by example. The checked plan is `toml_primary` when a `--source` parses TOML-primary, else `args.plan`; the anchor is `source.as_ref().or(plan.as_ref())` in `resolve_metrics_path`. With a TOML-primary `--source` both are the source. With no `--source` both are the plan. With a `--source` that is absent, unparseable, or Markdown-primary, the checked plan is `--plan` while the anchor is the `--source`. So the two rootings can differ only in the region the sentence names, and "in TOML-primary mode the checked plan IS the anchor and the rule is unchanged there" is correct.

DIRECTION 1, THE NEW RULE REFUSES A CORRECT SAME-PROJECT PAIRING THE ANCHOR RULE ALLOWS. Fixture C is one project. Today, and under an anchor-rooted inc2:

```
$ "$BIN" validate --source "$SC/fixC/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixC/notes/p.md" --workflow
.../fixC/notes/p.md vs .../fixC/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The checked plan's own root is measurable through the anchored default, since `src/main.rs:project_root_of_source` is the same walk both rootings use:

```
$ "$BIN" validate --plan "$SC/fixC/notes/p.md" --workflow
no metrics log at .../fixC/notes/docs/metrics/workflow.jsonl; nothing to validate
```

`notes/p.md` has no `docs/plans`-shaped ancestor, so `project_root_of_source` takes its "the source's OWN directory is the root" fallback and the checked plan's root is `.../fixC/notes`. The log actually read, `.../fixC/docs/metrics/workflow.jsonl`, is NOT under `.../fixC/notes`, so the amendment's predicate REFUSES a correct single-project invocation that the converged anchor-rooted text allows. The anchor's root is `.../fixC`, so the anchor-rooted predicate does not fire.

ONE QUALIFICATION I OWE THE PLANNER, because it bounds the cost. The same LAYOUT is already refused by the converged anchor-rooted text in its other spelling: with `--plan .../fixC/notes/p.md --metrics .../fixC/docs/metrics/workflow.jsonl` and no `--source`, the anchor IS the plan, its root is `.../fixC/notes`, and an anchor-rooted inc2 refuses it too (measured green today, exit 0, which is the pre-inc2 state). So the amendment does not introduce a new SPECIES of false positive; it removes the rescue that a `--source` inside `docs/plans` currently gives to a `--plan` outside one. That makes the new behaviour more self-consistent and it is still a new hard refusal of a run that works today, which is what "Safe on existing projects" and this step's accepted-cost convention exist to govern.

DIRECTION 2, THE NEW RULE ALLOWS A CORRECT PAIRING THE ANCHOR RULE REFUSES. With fixture B given its own log:

```
$ "$BIN" validate --source "$SC/fixA/.../TEMPLATE.plan.toml" --plan "$SC/fixB/.../TEMPLATE.md" --metrics "$SC/fixB/docs/metrics/workflow.jsonl" --workflow
.../fixB/docs/plans/TEMPLATE.md vs .../fixB/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

B's plan against B's own log, the correct pairing. Checked-plan root `.../fixB` contains the log, so the new rule allows it. Anchor root `.../fixA` does not, so an anchor-rooted predicate refuses it. The new rule is BETTER here.

THE PLAIN STATEMENT ASKED FOR. The property is NOT a strict superset, and it is not true under a qualification that could be added to the sentence: within the region the sentence itself names, the two rootings disagree in BOTH directions, so their refusal sets are incomparable rather than nested. What IS true, and is all that survives, is the region claim: they coincide in TOML-primary mode and where no `--source` is given, and they can differ only when a `--source` is given that is not TOML-primary. "Which is exactly the gap" is also false: the gap is one of at least three distinguishable cases in that region (the divergent cross-project pairing, direction 1, direction 2).

IS THE HUMAN OWED A CORRECTION? YES, and on two counts. The falsified ground is not confined to the plan: `docs/plans/agent-scaffold.ledger.md:395` records it as reasoning point (2) of what was PRESENTED to the human ("It is a STRICT SUPERSET of the converged anchor-rooted text rather than a replacement ... which is exactly the gap"), under a heading that says the reasoning is recorded "because it is what makes the choice checkable rather than a preference". And direction 1 is a measured new false positive on a layout that works today, of the same species as accepted cost (ii), which this step's own convention (line 261: "Both were measured, both were put to the human, and both were ACCEPTED") says is the human's to accept rather than the planner's to assume.

IS A RE-DECISION OWED? NOT ON THIS EVIDENCE, and the reason is measurable rather than a preference. The chosen option was compared against two alternatives. Ground 1 (one predicate, not two layered) is unaffected. Ground 3 (it reaches the typo'd `--source`, which a two-root comparison cannot, because a nonexistent path has no canonical root) is unaffected and I reproduced the typo case above. And direction 1 is NOT avoided by the rejected second-condition alternative: for fixture C the anchor resolves to `.../fixC` and the checked plan to `.../fixC/notes`, two different roots, so "a second condition on the anchor and the checked plan resolving to different roots" fires on exactly the same run. The third option was rejected on scope, not merit, and that is unchanged. So the falsified ground does not flip the comparison; what the human is owed is the correction and the accept-or-carve-out call on the new cost. Routing that is the orchestrator's, not mine.

MINIMAL FIX, MEASURED. Deletion at the two plan sites, a correction note at the ledger, and one recorded cost.

1. `workflow-enforcement-tier.md:167`. DELETE the falsified claim and keep the true region statement: remove "It is a STRICT SUPERSET of the anchor-rooted text rather than a replacement, since" and remove ", which is exactly the gap", leaving "In TOML-primary mode the checked plan IS the anchor and the rule is unchanged there; it differs only when a `--source` is given and is not TOML-primary." Nineteen words deleted, 1 changed for capitalisation, 0 added. The list's framing sentence says "Three properties of the decided rule the implementer should carry"; with the second property demoted to a region statement it should read "Three things", or the count drops to two. That is one word either way.
2. `workflow-enforcement-tier.md:328`, check 13b's third run. DELETE ", and it is the reason this rule is a superset rather than a replacement". About 13 words, 0 added. The run keeps its no-regression purpose; what it cannot do is establish a property that is false, and as `EX-3` notes it was constructed in TOML-primary mode where the two rootings agree by construction, so it never could.
3. `docs/plans/agent-scaffold.ledger.md:395`. Do NOT rewrite reasoning point (2): it is the record of what was presented at decision time and rewriting it destroys the audit trail. APPEND a correction sentence naming the two directions and pointing at this triage file. About 35 words, and nothing smaller works because a correction that does not say WHICH direction is which cannot be checked.
4. THE NEW COST MUST BE RECORDED SOMEWHERE, and where is a human call rather than a triage prescription. The two candidate slots already exist: a third entry in "The two accepted costs" (line 259, which would need retitling, one word), or the "A THIRD BEHAVIOUR CHANGE IS NOT A COST BUT SHOULD BE STATED" slot at line 267. Either way about 40 words: the layout (a same-project `--plan` outside any `docs/plans`, with a `--source` inside one), the measured outcome (exit non-zero under `--workflow` after inc2, omission on the projections), and that the same layout is already refused in its no-`--source` spelling. Nothing smaller works because a cost this step's own convention says must be PUT to the human cannot be recorded by deletion.

SITE COUNT MEASURED: 3 authored (`workflow-enforcement-tier.md:167`, `workflow-enforcement-tier.md:328`, `agent-scaffold.ledger.md:395`) plus 1 new site for the recorded cost; 2 mechanical (`agent-scaffold.md:1562` and `:1723`). A case-insensitive `grep -rin "superset"` over all 95 sidecars, `agent-scaffold.plan.toml`, `agent-scaffold.ledger.md` and the projection is what found site 2 and site 3; a search for the capitalised "STRICT SUPERSET" alone, which is what the finding cites, returns only site 1 and the ledger, and MISSES check 13b's lower-case "a superset rather than a replacement" entirely. That miss is the one that matters operationally, since it is the sentence telling an implementer which run establishes the property. The decision receipt itself carries only the options and the choice, no reasoning, so `docs/metrics/workflow.jsonl` needs no correction.

## `EX-4` VALID, low (unchanged). Line 187 quantifies over three surfaces and is false for one of them

Both citations resolve. Line 187 (amended): "The trigger in all three cases is the SAME containment predicate the validator's refusal uses (the canonically-derived root of the plan THAT SURFACE READS, and whether the resolved artifact lives under it)". The three cases are the bullets at 189, 190 and 191: `status`, `status --resume`, `next`. Line 171 (new): "`status --resume` is the one surface that reads NO plan ... so it has no checked plan to root on and its root falls back to the source-then-plan anchor `default_ledger_path` already uses".

Confirmed against the code by symbol. `src/main.rs:run_resume` reads only `ledger_path`; its task comes from `next::derive_task(&args.source, &args.plan)`, which reads filenames, not contents. `src/main.rs:default_ledger_path` anchors on `source.as_ref().or(plan.as_ref())`. So line 171 is accurate and line 187's parenthetical has no referent for one of its three cases.

IN SCOPE, and the amendment is what made it so: the pre-amendment text read "the canonically-derived plan root", which was merely vague; sharpening it to "the plan THAT SURFACE READS" is what made it false for the plan-less surface. Low, and not lower, because the same sentence's added second clause points at the enumeration that carves the surface out, so a reader following the pointer gets the right answer.

MINIMAL FIX, MEASURED. DELETE "in all three cases" at `workflow-enforcement-tier.md:187`. Four words deleted, 0 added. The sentence then states the rule without claiming to cover all three bullets, and the universality it was carrying is already carried by the next clause, "The predicate is never re-implemented per surface". Nothing smaller works, and nothing added is needed: line 171 already holds the carve-out and line 187 already points at it.

SITE COUNT MEASURED: 1 authored, 1 mechanical (`agent-scaffold.md:1582`). `grep -rn "in all three cases"` over all 95 sidecars, `agent-scaffold.plan.toml` and the projection returns exactly those two lines.

## `EX-5` VALID, low (unchanged), with the SITE CORRECTED from the end property to the amendment's own claim

Reproduced. Fixture D is `fixB` with `fixA` vendored at `vendor/a` and no log of its own:

```
$ "$BIN" validate --plan "$SC/fixD/docs/plans/TEMPLATE.md" --workflow
no metrics log at .../fixD/docs/metrics/workflow.jsonl; nothing to validate      # checked plan's root is .../fixD

$ "$BIN" validate --source "$SC/fixD/vendor/a/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixD/docs/plans/TEMPLATE.md" --workflow
.../fixD/docs/plans/TEMPLATE.md vs .../fixD/vendor/a/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The log read is at the VENDORED project's own `docs/metrics/` path, so it belongs to a different project by the same filename convention the whole mechanism uses, and it IS under the checked plan's root, so the predicate does not fire. This is sharper than the copied-log residual line 271 already concedes, because there the log had been moved INTO the project's own directory.

THE CITATION IS RIGHT AND THE SITE IS WRONG, so I correct it rather than dismissing. The end property at line 112 is unamended, its residual is already recorded in a dedicated section ("What this step does not fix, and where it goes instead", line 269, whose line 271 gives the same "the guard passes (the log IS under the fixture's root)" reasoning) and again in the scope list at line 379. Against line 112 alone, the four-condition out-of-scope precedent would apply: provenance predates the base commit (checked: line 112 unmodified in `git diff main HEAD`), no commit in the range modifies the lines (checked: one commit in range, neither 112 nor 271 in any hunk), and the nested case behaves identically under BOTH rootings (checked: anchor root `.../fixD/vendor/a` also contains the log), so the subject is independent of the rooting change.

The third condition FAILS, and that is why the finding stands. This amendment added, at line 282, "`Q-55-endproperty`, which is what makes the predicate reach a divergent `--source`/`--plan` pairing AND SO WHAT MAKES THIS INCREMENT CLOSE THE STEP'S END PROPERTY rather than half of it". Read against line 112 as written, the nested case falsifies that. Line 167's neighbouring sentence does NOT need the same fix: "rooted on the anchor, the END PROPERTY above would have been met by no increment of this step" is a claim about the anchor rooting, and it remains true.

MINIMAL FIX, MEASURED, AND IT IS A PURE DELETION. At `workflow-enforcement-tier.md:282`, delete "and so what makes this increment close the step's end property rather than half of it", leaving "(`Q-55-endproperty`, which is what makes the predicate reach a divergent `--source`/`--plan` pairing)". Sixteen words deleted, 0 added. Do NOT take the finding's own prescription of adding a containment clause to the end property at line 112: that authors new qualifying prose onto the step's definition of done, at the one site most exposed to being falsified at an edge in a later round, to say something sections 269 and 379 already say. A deleted claim cannot be falsified at an edge.

SITE COUNT MEASURED: 1 authored, 1 mechanical. `grep -rn "end property\|END PROPERTY"` over all 95 sidecars and `agent-scaffold.plan.toml` returns six lines, all in `workflow-enforcement-tier.md` (18, 112, 167, 267, 282, 311). Of those, 282 is the only affirmative claim that the increment CLOSES it; 311's "the one that decides whether this increment closes the end property" is conditional and stands.

## `FI-1` VALID, RE-SEVERITISED to high. Reproduced, and the site set is 5, not 2

The reproduction holds and I re-ran both halves: the metrics false pass and the ledger leak both occur with `--source` and `--plan` alone, no explicit override anywhere on the command line (see the three runs at the top of this file). So a second, independent way the gap survives inc1's anchoring exists, and it needs no explicit flag. Both cited paragraphs are affirmative exhaustiveness claims that this establishes as false, and the fidelity reviewer's Part A verifications all re-checked out against the tree by symbol (`run_validate`'s four arms, `resolve_metrics_path`'s source-first `or`, `default_ledger_path`'s identical anchor, `run_status` and `run_next`'s shared `toml_source` selection, `run_resume` reading no plan).

IN SCOPE on the third condition of the out-of-scope precedent: the amendment's own new material (line 167, the `next` bullet at 191, checks 13b and 14g) is what establishes the falsifying case, so the increment's change did falsify it. The other three conditions would have held.

THE SITE SET IS UNDER-MEASURED, which is what raises the severity. Grepping the phrasings across all 95 sidecars, `agent-scaffold.plan.toml` and `agent-scaffold.ledger.md` returns FIVE authored sites, not two:

1. `workflow-enforcement-tier.md:183`, rationale: "and the explicit case is precisely what survives it". Named by the finding.
2. `workflow-enforcement-tier.md:289`, rationale: "Its scope is exactly the case that survives anchoring, an explicit `--metrics` naming a foreign log". Named by the finding.
3. `workflow-enforcement-tier.md:190`, THE `status --resume` BULLET, which is BEHAVIOURAL SPECIFICATION and not rationale: "The DEFAULT ledger case is already closed by the anchoring in inc1 (explorer B measured the post-fix run printing "no ledger at <fixture path>; nothing to resume"); what this rule adds is the residual, an explicit `--ledger-fragment` naming a ledger outside the plan's root." NOT named by the finding, and MEASURED FALSE.
4. `workflow-enforcement-tier.md:330`, check 14b: "THE OMIT BEHAVIOUR (`Q-55-refusalscope`), which is THE case that survives inc1". A definite-article uniqueness claim, now shared with 13b's case. Not named by the finding.
5. `agent-scaffold.plan.toml:1710`, the `Q-55-refusalscope` decision record in the PLAN SOURCE: "when the pairing is nonetheless unsafe, THE CASE THAT SURVIVES ANCHORING BEING AN EXPLICIT `--metrics` NAMING A FOREIGN LOG, they OMIT the workflow-derived fields". Not named by the finding, and it is in the structured source rather than a sidecar, so a search confined to the reviewed file could not have found it.

SITE 3 IS THE ONE THAT CARRIES THE SEVERITY, and I measured it rather than inferring it:

```
$ "$BIN" status --resume --source "$SC/fixA/docs/plans/TEMPLATE.plan.toml" --plan "$SC/fixB/docs/plans/TEMPLATE.md"
## RESUME STATE

FIXTURE-A-SECRET-RESUME-LINE: this is project A internal state
exit: 0
```

No `--ledger-fragment`. This is the DEFAULT ledger path, and it is exactly the behaviour DEFECT C names at line 7 ("`status --resume` prints one project's internal resume state into another project's agent brief"). It SURVIVES INC2 AS SPECIFIED: `default_ledger_path` anchors on the `--source`, so the ledger read is `.../fixA/docs/plans/TEMPLATE.ledger.md`; line 171 decides that `status --resume`, reading no plan, roots on that same source-then-plan anchor; and `.../fixA/docs/plans/TEMPLATE.ledger.md` is under `.../fixA`, so the predicate does not fire and the block is printed. Line 190 says this default case "is already closed by the anchoring in inc1". Explorer B's quoted measurement was made on a single-project fixture, where it is true; it does not carry to the divergent pairing.

This is the exact pattern the finding itself names ("narrows one member of a pair, leaves the other"): the amendment updated the `next` bullet at 191 with its "SECOND WAY TO BE UNSAFE" sentence and left the `status --resume` bullet directly above it asserting the narrower, now-falsified scope. Medium was right for two rationale paragraphs. Five sites, one of which is a behavioural specification that a measured surviving instance of a NAMED defect contradicts, is high.

MINIMAL FIX, MEASURED. Four deletions and one decision.

1. Site 1: delete "and the explicit case is precisely what survives it". Nine words, 0 added. The rest of the sentence (anchoring does nothing to an explicit `--metrics` naming a foreign log) stays true.
2. Site 2: delete "Its scope is exactly the case that survives anchoring, an explicit `--metrics` naming a foreign log, and". About 18 words, 0 added, letting "no lexical test separates an explicit `--metrics` naming a foreign log from an explicit `--metrics` naming the plan's own log spelled differently" stand as the example it always was. The paragraph's actual argument is untouched.
3. Site 4: change "the case" to "a case". One word.
4. Site 5: delete ", the case that survives anchoring being an explicit `--metrics` naming a foreign log,". About 13 words, 0 added. The decision record's operative content (they omit, say why, exit 0) is untouched.
5. Site 3 is NOT closeable by deletion alone, and I am not prescribing which way it goes because it is a decision. Deleting the falsified sentence (about 45 words, 0 added) removes the falsity and leaves the bullet's operative specification intact. It also leaves a surviving instance of defect C unrecorded in a step that lists defect C among the four it closes and does not name it in "Scope: what this step does not do" (checked: lines 378 to 390, and line 390 is the file's own precedent for recording exactly this kind of measured surviving gap). THE QUESTION FOR THE HUMAN is the mirror of one already put and answered: `Q-55-endproperty`'s second decision extended the checked-plan rooting to `next`'s DEFAULT ledger path inside inc2. Whether `status --resume`'s default ledger under a divergent pairing is in scope for the same treatment, or is recorded as out of scope with its reason, was never put. If it is recorded as out of scope, that is about 35 new words in the scope list; if it is brought in scope, it changes what inc2 builds. I flag it and route it rather than choosing.

SITE COUNT MEASURED: 5 authored (3 in `workflow-enforcement-tier.md`, 1 in `agent-scaffold.plan.toml`, and site 4 also in the sidecar), plus 1 conditional new site in the scope list; 4 mechanical in `agent-scaffold.md`. Searches run over the full population: `grep -rin "surviv"`, `grep -rn "explicit \`--metrics\` naming a foreign log"`, `grep -rn "the residual"`, and `grep -rn "precisely what survives\|exactly the case that survives"`.

## The 14g versus "FOUR red cases" question, deferred to me by the fidelity reviewer

RULING: THE FIDELITY REVIEWER WAS RIGHT NOT TO RAISE IT. No finding, and no change to the count.

Three grounds, checked rather than asserted.

FIRST, the sentence at line 311 is a literal count of the items it names in the same breath, and it names four: check 11 (the validator's response), check 13b (the case an anchor-rooted predicate cannot reach), check 14b (the projections in human text), check 14e (the same on the machine surface). Four items, "FOUR". Self-consistent.

SECOND, the exclusions are PRINCIPLED, not arbitrary, which is the part that decides whether the list claims exhaustiveness. Its stated organising rule is "one predicate with several consumers on two surfaces is not evidenced by testing one consumer on one surface": one red per consumer-and-surface. 14c is `status` in human text, redundant with 14b's surface; 14f is the vocabulary separating, which reuses 14b's case. The pre-amendment "THREE" excluded both on that rule, and the amendment added 13b on a second, stated rule (the case that decides whether the increment closes the end property). So the list is a curated set with two visible selection criteria, not an enumeration of every red-green pair in the file.

THIRD, and this is what makes it safe against my own `EX-1` prescription, the file consistently treats "Nth run" extensions as parts of their parent check and never counts them separately: 13b has a second and a third run, 14f has a fourth run, 14g has four. None of those appears in the red-case list. `EX-1`'s fix lands on 14g's fourth run, an existing run of an existing check, so it does not disturb the count either. Had I prescribed a new numbered check instead, the count sentence would have needed revisiting, which is a second reason to prefer extending 14g.

## Scratch hygiene

Every probe ran with `TMPDIR=/tmp/claude-1000/tri-ep-scratch` and every fixture was created under it. The directory was removed when the triage finished. DIRECTORIES LEFT IN `/tmp`: 0.
