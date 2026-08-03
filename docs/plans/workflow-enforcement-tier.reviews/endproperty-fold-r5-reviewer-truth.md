# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 5, REVIEWER: is the fix truthful

Reviewer: independent of the planner, of every prior reviewer and triager on this fold, and of the round 5 fidelity reviewer checking the fix against its instructions. Read-only with respect to the reviewed artifact; this file is the only thing written.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep5-truth`, branch `review/q55-ep5-truth`, at `cda71ee` (the authorised escalation fix landed on top of the round 4 backstop's overturn). Binary built at this commit with `cargo build` (increment 1 in the tree, increment 2 absent, exactly as at every prior round of this fold).

LENS. Not "did the fix match its instructions" (another reviewer's job). This round asks only: is what the document now says TRUE. Every claim below was tested by constructing the fixture the claim describes and running the built binary; where increment 2's own behaviour is the subject (it is not built), the containment test itself is computed by hand from the measured root and the measured resolved artifact, exactly as every prior round in this fold has done, and I say explicitly which half of each verdict is measured and which is reasoned.

## Verdict summary

ZERO FINDINGS. Six target claims, twenty-plus fixtures, no falsifying case found on any of them despite deliberately adversarial construction on each (deeper nesting, reversed nesting, three-project nesting, fallback-rooted nesting, a fifth and an attempted sixth symlink placement, the sibling/nested contrast on the ledger sentence, and a direct code-and-plan-search check of the "no owner" claim). This is a clean result and, per this fold's own instructions, a legitimate one: it merges the artifact.

| claim | verdict |
| --- | --- |
| The in-root bound as a RULE (`:267`) | TRUE |
| Accepted cost (ii) generalised (`:257`) | TRUE, no sixth placement found |
| `ledger-not-this-project` vocabulary (`:229`) | TRUE |
| The `next` bullet's nest/no-nest sentence (`:183`) | TRUE |
| "the LEDGER half... has NO OWNER" (`:269`) | TRUE |
| Check 19's second, log-side layout (`:339`) | TRUE (red today, reasoned green after inc2) |

## Claim 1: the in-root bound as a RULE, `:267`

TEXT UNDER TEST: "CONTAINMENT REFUSES ONLY WHAT LIES OUTSIDE THE CHECKED PLAN'S ROOT SUBTREE, so every foreign artifact inside that subtree is invisible to it: a log copied to this plan's own `docs/metrics/`, and equally a NESTED project's own log and ledger at their own conventional paths, the log then joining by bare slug and the ledger being echoed verbatim."

The rule makes two claims: (a) any foreign log/ledger genuinely inside the checked plan's canonical root subtree is invisible to containment, regardless of how it got there or how deep it sits, and (b) a foreign artifact genuinely outside that subtree is not, even when the two projects are otherwise nested in some looser sense. I built fixtures on both sides of (a) and (b).

FIXTURES AND COMMANDS, all under `TMPDIR=/tmp/claude-1000/rev-ep5-truth`, `AS=.../target/debug/agent-scaffold`, `S=$TMPDIR/fix`:

- `d1`: `outer` (checked, `outer.md`) with NO `docs/metrics/` of its own, `outer/packages/projA` (foreign, Markdown-primary `A.plan.toml`, its own 259-record log, its own ledger). Depth 1.
- `d3`: same shape, foreign project at `outer/a/b/c/projA`. Depth 3.
- `two`: `container` (checked, no log of its own) with TWO foreign projects nested at `container/pkgs/projA` and `container/pkgs/projB`, distinct record counts (259 and 50) so which log was read is unambiguous.
- `rev`: the REVERSE nesting. `projA` (foreign, its own 259-record log at its own conventional path) is the CONTAINER; the CHECKED project sits nested inside it at `projA/inner/checked`.
- `f3`: the conventionless-fallback shape. `myplan.plan.toml` (checked, borrowed-slug `complete`) sits directly at `f3/repo` with no `docs/plans/` of its own at all (so its root is derived through the FALLBACK, not the convention branch); the foreign project sits nested at `f3/repo/vendor/projA`.

MEASUREMENTS:

```
$ "$AS" validate --plan "$S/d1/outer/docs/plans/outer.md" --workflow
no metrics log at .../d1/outer/docs/metrics/workflow.jsonl; nothing to validate   # root = d1/outer
$ "$AS" validate --source "$S/d1/outer/packages/projA/docs/plans/A.plan.toml" --plan "$S/d1/outer/docs/plans/outer.md" --workflow
.../d1/outer/packages/projA/docs/metrics/workflow.jsonl: 259 records, valid
.../d1/outer/docs/plans/outer.md vs .../d1/outer/packages/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

Depth 1: root `.../d1/outer`, log `.../d1/outer/packages/projA/docs/metrics/workflow.jsonl`, under root, silent, joined. TRUE side of (a).

```
$ "$AS" validate --source "$S/d3/outer/docs/plans/TEMPLATE.plan.toml" --metrics "$S/d3/outer/a/b/c/projA/docs/metrics/workflow.jsonl" --workflow
.../d3/outer/a/b/c/projA/docs/metrics/workflow.jsonl: 259 records, valid
.../d3/outer/docs/plans/TEMPLATE.plan.toml vs .../d3/outer/a/b/c/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

Depth 3: same outcome. The rule does not degrade with depth.

```
$ "$AS" validate --source "$S/two/container/docs/plans/TEMPLATE.plan.toml" --metrics "$S/two/container/pkgs/projA/docs/metrics/workflow.jsonl" --workflow
... 259 records ... workflow invariants hold; exit=0
$ "$AS" validate --source "$S/two/container/docs/plans/TEMPLATE.plan.toml" --metrics "$S/two/container/pkgs/projB/docs/metrics/workflow.jsonl" --workflow
... 50 records ... workflow invariants hold; exit=0
```

Two projects nested in a third: BOTH foreign logs join silently against the container. The rule is about the subtree, not about a single privileged foreign occupant of it.

```
$ "$AS" validate --source "$S/f3/repo/myplan.plan.toml" --workflow
no metrics log at .../f3/repo/docs/metrics/workflow.jsonl; nothing to validate   # root via FALLBACK = f3/repo
$ "$AS" validate --source "$S/f3/repo/myplan.plan.toml" --metrics "$S/f3/repo/vendor/projA/docs/metrics/workflow.jsonl" --workflow
.../f3/repo/vendor/projA/docs/metrics/workflow.jsonl: 259 records, valid
.../f3/repo/myplan.plan.toml vs .../f3/repo/vendor/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

A project nested inside a directory that is itself not a project (here, the checked plan's own root is derived through the no-`docs/plans` fallback, and the nested foreign project sits inside a plain `vendor/` directory that is not a project by convention either): the rule still holds, and holds by the SAME general mechanism, with no separate case needed for the fallback. This directly reproduces the round 4 backstop's target 3 finding (the fallback is sufficient to widen the subtree but is not required to), and I built it independently rather than from the backstop's script.

```
$ "$AS" validate --source "$S/rev/projA/inner/checked/docs/plans/TEMPLATE.plan.toml" --workflow
no metrics log at .../rev/projA/inner/checked/docs/metrics/workflow.jsonl; nothing to validate   # root = rev/projA/inner/checked
$ "$AS" validate --source "$S/rev/projA/inner/checked/docs/plans/TEMPLATE.plan.toml" --metrics "$S/rev/projA/docs/metrics/workflow.jsonl" --workflow
.../rev/projA/docs/metrics/workflow.jsonl: 259 records, valid
.../rev/projA/inner/checked/docs/plans/TEMPLATE.plan.toml vs .../rev/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

REVERSED nesting (the checked project sits inside the foreign project's directory, rather than the other way round): today (inc1 only, no guard yet) this still greens, but the hand-computed containment test is decisive and goes the OTHER way from every case above. Root is `.../rev/projA/inner/checked`; the foreign log is at `.../rev/projA/docs/metrics/workflow.jsonl`, which is a SIBLING branch of the root (`projA/docs/...` versus `projA/inner/checked/...`), not a descendant of it. By the rule's own words the log lies OUTSIDE the checked plan's root subtree, so containment would REFUSE it once inc2 is built (REASONED: inc2 itself is not built; the prefix computation is not). This is the (b) side of the rule and it is the sharpest confirmation available that "root subtree" is doing real work and is not a rule that is silent on every nesting relationship: reversing which project contains which flips the verdict.

MEASURED: every root and every resolved-artifact path above, off the built binary and `realpath`, exactly as every prior round in this fold has done. REASONED: the actual post-inc2 refusal/silence outcome, since increment 2 does not exist; the reasoning is a plain path-prefix test on measured, canonical paths with no free parameters, the same computation the round 4 adversarial reviewer, the round 4 triage, and the round 4 backstop all used.

VERDICT: TRUE, on both directions of the rule, at three nesting depths, with two foreign occupants of one subtree, through both root-derivation routes (convention and fallback), and on the deliberately constructed contrast that inverts it.

## Claim 2: accepted cost (ii) generalised, `:257`

TEXT UNDER TEST: "THE COST IS THE DIVERGENCE AND NOT THE LAYOUT: any symlink that makes the canonicalised checked plan and the canonicalised resolved log fall under different roots produces it, on either side, and `docs/plans` is the placement that was MEASURED rather than the population."

Built six placements plus the P0 control, matching the round 4 adversarial reviewer's and the round 4 triage's own fixtures, rebuilt independently rather than reused:

- `P0`: control, no symlink anywhere.
- `P1`: the plan FILE is a symlink to `$S/shared-plans/TEMPLATE.plan.toml`; `docs/plans` itself is a real directory.
- `P2`: `docs/metrics` is a symlink to `$S/shared-metrics` (a sibling directory).
- `P3`: the log FILE itself is a symlink to `$S/shared-log/workflow.jsonl`.
- `P4`: `docs/plans` is a symlink to `$S/P4/elsewhere` (accepted cost (ii)'s own originally-measured layout, kept as the control on my own measurement).
- `P5a`: the whole `docs` directory is a symlink to `$S/shared-docs-notdocs`, a directory NOT itself named `docs`.
- `P5b`: the whole `docs` directory is a symlink to `$S/other/docs`, a directory that IS named `docs` (the mirror-image control).

ALL SIX WORK TODAY. Verbatim for every one of P0 through P5b, `validate --source .../docs/plans/TEMPLATE.plan.toml --workflow` prints its own three lines and exits 0 (shown for P2, representative):

```
$ "$AS" validate --source "$S/P2/docs/plans/TEMPLATE.plan.toml" --workflow
.../P2/docs/metrics/workflow.jsonl: 0 records, valid
.../P2/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
.../P2/docs/plans/TEMPLATE.plan.toml vs .../P2/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

ROOTS AND CANONICAL LOGS, measured off the binary (root, by asking for the default log at the canonical plan location) and `realpath` (the resolved artifact):

```
P0  root=.../P0                log=.../P0/docs/metrics/workflow.jsonl            UNDER root  -> silent (control)
P1  root=.../shared-plans      log=.../P1/docs/metrics/workflow.jsonl            NOT under   -> REFUSE
P2  root=.../P2                log=.../shared-metrics/workflow.jsonl             NOT under   -> REFUSE
P3  root=.../P3                log=.../shared-log/workflow.jsonl                 NOT under   -> REFUSE (decided reading, below)
P4  root=.../P4/elsewhere       log=.../P4/docs/metrics/workflow.jsonl            NOT under   -> REFUSE
P5a root=.../shared-docs-notdocs/plans   log=.../shared-docs-notdocs/metrics/workflow.jsonl   NOT under -> REFUSE
P5b root=.../other/docs/plans  log=.../other/docs/metrics/workflow.jsonl          UNDER root  -> silent (mirror control)
```

`P4` reproduces the design pass's own measurement ("from reading its 37-record log to `exit=1 REFUSED`"; here the record count differs because my fixture used an empty log, but the divergence direction is identical) and is the anchor that validates the rest of the table.

THE READING DECISION AT `:157` IS NOW SETTLED, and this is the one place the fix commit changed the mechanism text rather than the accepted-cost text: "resolve the metrics path by absolutising and canonicalising its longest existing ancestor, THE PATH ITSELF WHEN IT EXISTS, and re-appending the components below it". I confirmed this is the CURRENT wording (not the pre-fix "longest existing ancestor" alone, which the round 4 triage flagged as ambiguous exactly on `P3`). Under this decided reading, `P3`'s log leaf, which exists, counts as its own longest existing ancestor, so `realpath` on the leaf directly gives the canonicalised log (`.../shared-log/workflow.jsonl`), which is NOT under `.../P3`, so `P3` is REFUSED, consistently with `P1`, `P2`, `P4` and `P5a`, and with cost (ii)'s "on either side" (P3 is the log's own leaf, the mirror of P1's plan-side leaf).

THE QUIET HALF IS REAL TODAY, not merely asserted: `status --source` on `P2` today prints `metrics: 5 records` (after I populated `shared-metrics/workflow.jsonl` with five real lines) and `next` prints `metrics: 5 records` too, both reading the log through the symlink exactly as the project would today. That is the round evidence cost (ii) says the projections would lose after inc2 (REASONED for the post-inc2 omission itself, MEASURED for the fact that there is genuine evidence to lose).

IS THERE A SIXTH? I deliberately looked for one beyond the five (`P1`, `P2`, `P3`, `P4`, `P5a`) plus the `P5b` non-divergent control, by symlinking the checked project's own TOP-LEVEL directory (reached via `PWD` through a symlinked ancestor, and separately via an explicit symlinked top-level alias) and confirmed both plan and log canonicalise through the SAME symlink to the same real prefix, so the guard's own root and the log's own location cancel and nothing diverges; this matches the round 4 adversarial reviewer's attack 19 finding on a symlinked current directory. I could not construct a divergent placement beyond the five the generalised text already covers. NEGATIVE RESULT, stated as one: no sixth found, despite trying the one remaining unexplored axis (a shared top-level symlink) that the five known placements do not already exercise.

DOES THE GENERAL WORDING ACTUALLY DESCRIBE EACH? Yes, including `P5a`, which is the placement the round 4 triage found only after the finding as filed named four. `P5a`'s mechanism is subtler than "the plan's own leaf or directory moved": the symlinked `docs` component is on BOTH the plan's path and the log's path at once (a shared ancestor), and the divergence arises because renaming the symlink's target defeats `project_root_of_source`'s own `under_docs` name test, producing a root that is one level too narrow (the `plans` directory itself, rather than its grandparent). The generalised text's "any symlink that makes the canonicalised checked plan and the canonicalised resolved log fall under different roots... on either side" does not name this specific mechanism, and does not need to: it is phrased over the OUTCOME (roots diverge) rather than over which path component moved, which is exactly what keeps it true of a mechanism the triage found only after cost (ii)'s original, narrower wording had already been filed as too narrow. This is the deliberate design the round 4 triage argued for ("Nothing smaller works: ... enumerating the five placements reproduces the stale-enumeration class"), and testing it against the fifth placement found after the fact is the strongest available check that the generalisation, not the enumeration, was the right fix.

VERDICT: TRUE. All five divergent placements are covered by the general wording as written; the mirror control (`P5b`) correctly does not diverge; no sixth placement was found on the one remaining axis tried.

## Claim 3: `ledger-not-this-project`, `:229`

TEXT UNDER TEST: "the resolved ledger is not under the root of the plan this surface reads, which is either an explicit `--ledger-fragment` outside it or, on `next`, a DEFAULT ledger anchored on a `--source` that itself lies outside it. Both members are CONTAINMENT and neither is a project-identity test: a `--source` in a different project reaches this only when that project is not NESTED inside the root."

This is the text the round 4 backstop's site 2 finding was raised against (the PRE-fix wording named "a `--source` belonging to a different project", a project-identity condition that `:161` explicitly rejects, and that condition held on the backstop's own nested fixture where the stated containment clause did not fire). I tested whether the CURRENT wording is both internally consistent (a pure containment test, as it now claims) and empirically correct on both a nested and a sibling pairing.

NESTED (reusing `d1` above, with a ledger added to the nested `projA`):

```
$ "$AS" next --source "$S/d1/outer/packages/projA/docs/plans/A.plan.toml" --plan "$S/d1/outer/docs/plans/outer-ip.md"
...
context:
    ledger: .../d1/outer/packages/projA/docs/plans/A.ledger.md
...
RESUME STATE (verbatim from the ledger):
## RESUME STATE

MARKER-PROJA-SECRET-RESUME-LINE
exit=0
```

Root (outer, measured) is `.../d1/outer`; the resolved ledger is `.../d1/outer/packages/projA/docs/plans/A.ledger.md`, which IS under that root. `--source` (projA, a different project) does NOT "lie outside" the root here, because it is nested inside it, so containment is silent and the ledger echoes, exactly as the sentence's closing clause predicts for the nested case.

SIBLING (`sib/A`, Markdown-primary source with its own ledger, beside `sib/B`, the checked plan; NOT nested):

```
$ "$AS" next --source "$S/sib/A/docs/plans/A.plan.toml" --plan "$S/sib/B/docs/plans/B.md"
...
    ledger: .../sib/A/docs/plans/A.ledger.md
...
RESUME STATE (verbatim from the ledger):
## RESUME STATE

MARKER-SIBA-SECRET-RESUME-LINE
exit=0            # today; the hand-computed containment test below decides the post-inc2 case
```

Root (`sib/B`, measured: `no metrics log at .../sib/B/docs/metrics/workflow.jsonl`) is `.../sib/B`. The resolved ledger is `.../sib/A/docs/plans/A.ledger.md`, a SIBLING branch of the root, not a descendant of it. `--source` here genuinely "lies outside" the root, so containment would fire (REASONED for the post-inc2 refusal itself, MEASURED for the two paths and their non-containment relationship).

EXPLICIT `--ledger-fragment` OUTSIDE ROOT (the first enumerated member):

```
$ "$AS" next --source "$S/P0/docs/plans/TEMPLATE.plan.toml" --ledger-fragment "$S/shared-plans/foreign.ledger.md"
...
RESUME STATE (verbatim from the ledger):
## RESUME STATE

MARKER-FOREIGN-LEDGER-FRAGMENT
```

Root (`P0`) is `.../P0`; the named fragment resolves to `.../shared-plans/foreign.ledger.md`, not under it. Today it echoes (no guard yet); by the stated rule it would be caught, and there is no project-identity content in this branch at all, only a path.

VERDICT: TRUE. The vocabulary entry, as now worded, is a pure containment predicate on both its enumerated members, and the closing clause's nested/not-nested distinction is exactly the distinction the two fixtures produce.

## Claim 4: the `next` bullet's nest/no-nest sentence, `:183`

TEXT UNDER TEST: "The predicate rooted on the checked plan catches that WHERE THE TWO PROJECTS DO NOT NEST; an anchor-rooted one cannot, for the same reason it cannot catch the metrics case, and where they DO nest neither rooting catches it (the IN-ROOT BOUND below)."

Three sub-claims, tested separately:

(a) CHECKED-PLAN-ROOTED CATCHES IT WHEN NOT NESTED. This is the `sib/A`/`sib/B` measurement directly above: root(`sib/B`) does not contain the resolved ledger at `sib/A`, so the checked-plan-rooted predicate fires (REASONED for the actual refusal, MEASURED for the two paths).

(b) AN ANCHOR-ROOTED PREDICATE CANNOT CATCH IT, EVER. This is a claim about a hypothetical alternative design (rooting on the `--source`/`--plan` anchor instead of on the checked plan) that was considered and rejected under `Q-55-endproperty` and is not built in any form, so there is nothing to run. I derive it from the two functions that would feed it: `default_ledger_path` (`src/main.rs`) returns `anchor.parent().join(format!("{task}.ledger.md"))`, i.e. a path directly beside the anchor, and `project_root_of_source` (`src/main.rs`) either returns `ancestor.parent().parent()` for the nearest `docs/plans` ancestor of `anchor.parent()` (in which case `anchor.parent()` is, by construction, a descendant of that root, since the `plans` ancestor the walk matched is itself an ancestor-or-equal of `anchor.parent()`) or falls back to `parent.to_path_buf()`, i.e. `anchor.parent()` itself. In BOTH branches, `default_ledger_path(anchor, ...)`'s result is trivially under `project_root_of_source(anchor)`: the ledger sits beside the anchor, and the anchor's own root is always an ancestor of (or equal to) the anchor's parent. So a predicate rooted on the anchor would find the ledger under the anchor's root on EVERY input, nested or not, sibling or not; it is a tautology of the two functions' definitions, not a property of any particular fixture. This is the SAME argument `:159` already makes for the metrics case ("The resolved log is DERIVED from the anchor, so it is always under the anchor's root and a predicate rooted THERE can never fire on that pairing"), and it transfers to the ledger without modification because `default_ledger_path` has the identical shape (derive from the anchor, place directly beside it). REASONED, not measured: there is no anchor-rooted build to run.

(c) WHERE THEY NEST, NEITHER CATCHES IT. The checked-plan-rooted half is the `d1` nested measurement above (MEASURED: the ledger echoes). The anchor-rooted half follows from (b) directly, since (b) holds unconditionally regardless of nesting (REASONED).

VERDICT: TRUE on all three legs: two measured (sibling catches, nested does not, both on the checked-plan rooting), one reasoned from the two functions' definitions (anchor-rooted never catches, in either case). This is the exact sentence the round 4 backstop's site 1 finding was raised against on its PRE-fix wording ("The predicate rooted on the checked plan catches that; an anchor-rooted one cannot..." with no nest/no-nest qualifier at all, falsified by the backstop's own `F1` fixture); the current wording adds precisely the qualifier the falsification required, and it is now true of both the case it always was true of and the case it was not.

## Claim 5: "the LEDGER half... has NO OWNER in this plan today", `:269`

TEXT UNDER TEST: "THE QUEUED STEP OWNS THE LOG HALF OF THE IN-ROOT BOUND ONLY: filtering `Round` records cannot change which ledger file `src/main.rs:run_next` opens, so the LEDGER half of that bound has NO OWNER in this plan today, recorded here rather than scheduled."

CODE, read directly (`src/main.rs:run_next`): the round-derived state (`rounds`, `metrics_records`) is computed from `metrics_path` (line ~1311-1317), and the ledger's own path (`ledger_path`, line ~1319-1322) is computed independently from `args.ledger_fragment` or `default_ledger_path(&task, &args.source, &args.plan)`. Nothing in the ledger-path computation reads `rounds`, `metrics_records`, or any per-record field. The queued mechanism explorer C built (`project: Option<String>` on `Round`, pre-filtering rounds in `check_workflow_toml`) operates entirely on the `Round` list; it has no path anywhere in it and cannot, by construction, alter which file `default_ledger_path` or an explicit `--ledger-fragment` names. This is a code-reading confirmation of the document's own reasoning, not a new argument.

PLAN SEARCH, for any OTHER step or question that might reach it. I grepped `docs/plans/agent-scaffold.plan.toml` for every step title containing "ledger", "resume", "identity" or "project", and for every open question mentioning "identity", "nested", "containment", "in-root" or "bound". The hits are: `ledger-template` (order 27, template placement/refresh policy, decided, unrelated to path resolution), `state-schema` (order 28, the plan-state JSON projection, unrelated), `repoint-resume-prompts` (order 65, deferred, about which PROMPT text points at `next`/`status --resume`, not about path resolution), `resume-state-currency-signal` (order 70, deferred, folds into `Q-58`/`Q-59`'s SESSION-LIFECYCLE checkpoint/resume state, a different kind of "resume" entirely, confirmed by reading `docs/plans/agent-scaffold.explorations/Q-59-data-model.md`), and `sidecar-ref-symlink` (order 64, deferred, about symlink escapes in `[meta].sidecars` refs specifically, a different field from anything `run_next` reads for its ledger). `validation-constraints`, the step name `Q-55-mechanism` and `Q-55-jsonreason` both name as the queue target for project identity, does not appear as a `slug` anywhere in `docs/plans/agent-scaffold.plan.toml` at all: it is referenced only as a future step name inside decision-provenance prose, not (yet) a scheduled `[[step]]` entry. No open question anywhere in the file addresses ledger routing, project identity, or the containment predicate's reach. MEASURED (a grep over the tracked plan population), not inferred.

VERDICT: TRUE, on both legs the claim rests on: the code-structural argument (confirmed by reading `run_next` directly) and the "nothing else in the plan reaches it either" implication (confirmed by search). I note, without treating it as a finding since it is outside this round's six named targets and touches the METRICS half's owner rather than the ledger half's absence of one, that the metrics half's own named owner ("the validation-constraints step") is itself not yet a scheduled entry in the plan; this does not make the ledger-half claim false, since a same-named future step failing to exist yet is consistent with "no owner... today", not evidence against it.

## Claim 6: check 19's second, log-side layout, `:339`

TEXT UNDER TEST: "A SECOND LAYOUT PINS THE LOG SIDE: `<root>/docs/metrics` a SYMLINK to a sibling directory, with the plan where it belongs, gives the same refusal and the same omission."

This is exactly `P2` above: `docs/metrics` is a symlink to `shared-metrics` (a sibling of the project directory), and the plan is untouched at its normal conventional path.

```
$ "$AS" validate --source "$S/P2/docs/plans/TEMPLATE.plan.toml" --workflow
.../P2/docs/metrics/workflow.jsonl: 5 records, valid
.../P2/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
.../P2/docs/plans/TEMPLATE.plan.toml vs .../P2/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

RED TODAY: running this against the built binary (increment 1 only) gives exit 0 and a green summary; an assertion that this layout is REFUSED, run against today's build, fails, which is what "red before increment 2" means for a check whose entire subject is a currently-working layout that the document says becomes a refusal. MEASURED.

REASONED GREEN AFTER: root (`.../P2`, measured) and the canonicalised log (`.../shared-metrics/workflow.jsonl`, measured via `realpath`) do not share a containment relationship (the log is not under the root), so the containment predicate specified at `:157` and `:165` would fire once built, converting this to a refusal under `validate --workflow` and to the omission-plus-reason under `status`/`next`. REASONED, since increment 2 is not built; the two paths it would compare are measured.

VERDICT: TRUE. Check 19's added second run pins a layout that is genuinely red today and is a genuine instance of the general divergence rule (claim 2) rather than a restatement of the first (plan-side) layout under a different name: the mechanism is different (the LOG canonicalises out, not the plan), which is exactly the gap the round 4 triage's `R4A-2` ruling identified in cost (ii)'s ORIGINAL, pre-fix wording and check 19's original, single-layout form.

## What was measured and what was reasoned, summarised

MEASURED, on the built binary at `cda71ee` (increment 1 landed, increment 2 absent): every root (via the binary's own `no metrics log at <path>` note against a canonical plan location), every resolved log and ledger path (via the binary's explicit-flag echo and via `realpath`), every "does this layout work today" claim, and every prefix/containment comparison between two measured, canonical paths (a computation with no free parameters, performed by eye on paths the binary and `realpath` produced, the same method every prior round of this fold used for the one step increment 2 has not built).

REASONED, because increment 2 does not exist and one alternative design (anchor-rooted) was considered and rejected without ever being built: (i) that a positive containment result would, once increment 2 is built, actually produce a refusal or an omission rather than some other behaviour, in every place this report says "REASONED" or "once inc2 is built"; (ii) the anchor-rooted counterfactual in claim 4(b), derived from the definitions of `project_root_of_source` and `default_ledger_path` rather than from a build. Both kinds of reasoning are the same shape the document's own mechanism sections use, and neither substitutes for a fixture: every reasoned claim in this report is anchored to measured paths, never to a description of what a fixture would show.

## Direct answer to the round's question

The document, as it now stands after `cda71ee`, describes the in-root bound truthfully and completely enough that a reader building any of the fixtures above would get the same answer the text predicts, on both the RULE itself (containment is blind inside the checked plan's root subtree, at any depth, through either root-derivation route, for a log or a ledger, but is NOT blind to a foreign artifact that genuinely lies outside that subtree even when the two projects are nested the other way round) and on the two sentences the round 4 backstop found specifically false in the prior wording (`:183`'s nest/no-nest qualifier and `:229`'s pure-containment restatement). Accepted cost (ii)'s generalisation holds up against a placement (`P5a`) that was not part of the evidence base it was written against, which is the strongest test available for a rule written specifically to avoid staleness. The "no owner" claim is confirmed by the code it cites and by a search that found no other candidate owner anywhere in the tracked plan. I could not construct a case, across nesting depth, nesting direction, shared containers, fallback rooting, or symlink placement, where the document's current claims and the binary's (or the hand-computed predicate's) behaviour came apart. Four earlier readers getting an earlier version of this wrong is not evidence this version is also wrong; it is the reason this round existed, and the version it tested is the one the fourth reader's overturn produced.

## Scratch hygiene

All fixtures were built under `TMPDIR=/tmp/claude-1000/rev-ep5-truth`, created for this round. Removed after the evidence above was captured. Directories left in `/tmp`: 0 (the harness-provided session scratch tree under `/tmp/claude-1000/` is not counted; nothing was written to bare `/tmp`).
