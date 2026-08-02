# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 2, TRIAGE

Triager: independent of the planner, of both round 2 reviewers, of both round 1 reviewers, and of the round 1 triager. Read-only with respect to the reviewed artifact; this file is the only thing written. No fix is applied here.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-ep2`, branch `triage/q55-ep2`, cut from the reviewed tip so every reviewer citation resolves against the reviewed text. Binary built at `3354a90` with `cargo build`, so INC2 IS NOT LANDED and every run below is a PRE-INC2 measurement. Post-inc2 statements are derived from the code plus the amendment's own stated rule and are labelled as derived. All fixtures under `TMPDIR=/tmp/claude-1000/tri-ep2`, removed at the end.

Repository guards re-run at the reviewed commit, both green, so the projection is a faithful regeneration and every mechanical site in `docs/plans/agent-scaffold.md` is produced by `render` rather than hand-edited:

```
$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date          exit: 0

$ agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 253 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold          exit: 0
```

FIXTURES, all built from `agent-scaffold scaffold --output-dir <d> --write --force --principles default`, which emits `[meta] primary = "toml"` (verified at `base/docs/plans/TEMPLATE.plan.toml:15`):

- `fixT`: ONE project. `docs/plans/TEMPLATE.plan.toml` left TOML-primary as scaffolded, `notes/p.md` a copy of the rendered `TEMPLATE.md`, this repository's 253-record log at `docs/metrics/workflow.jsonl`, and `docs/plans/TEMPLATE.ledger.md` carrying a `## RESUME STATE` block with a unique marker line. This is accepted cost (iii)'s layout, built the way cost (iii)'s own text writes it.
- `fixM`: `fixT` with `primary = "markdown"`, the only difference.
- `A`/`B`: the divergent pairing. `A` is Markdown-primary with this repository's log and a ledger carrying a marked `## RESUME STATE` block; `B` is a second project whose Markdown Roadmap carries `triager-runs-only-on-findings` at `complete`.

## Verdict summary

TEN raw findings from two lenses. `R2A-3` and `R2B-1` are the SAME defect, confirmed and merged below, so NINE distinct findings. All nine are VALID; none is dismissed. Seven require a fix; two are accepted as residual.

DEDUPLICATED VALID COUNT: 9. SEVERITY LIST: 2 high, 3 medium, 4 low.

| id | verdict | final severity | ground |
| --- | --- | --- | --- |
| `R2A-2` | VALID | high (unchanged) | Measured: with a TOML-primary `--source`, which is what `scaffold` emits, cost (iii)'s layout is NOT refused and check 19b exits 0 both before and after inc2. |
| `R2A-3` / `R2B-1` | VALID (merged) | high (`R2A-3` re-severitised UP from medium to `R2B-1`'s high) | Both no-`--source` spellings measured exit 0 today; the plain spelling is a silent miss no containment predicate can catch in ANY build; the bound is true only under two qualifiers the fix pass dropped. |
| `R2A-1` | VALID | medium (unchanged) | "only check 13b and 14g's fourth run catch" is falsified by check 14c's third run, authored in the same commit. |
| `R2A-4` | VALID | medium (unchanged) | Measured third case: the single-project cost (iii) layout, in BOTH `primary` spellings, prints its own resume block today and would be withheld; not in the "TWO CASES" enumeration and recorded as a cost nowhere. |
| `R2A-5` | VALID | medium (unchanged) | Line 189's second clause is false for `status --resume`, and the same commit deleted the root statement at line 173 that the clause points at. |
| `R2A-6` | VALID | low (unchanged) | Three invocations named, "both exit 0" asserted. |
| `R2A-7` | VALID | low (unchanged) | The count is five; the text says two, and the same pass added two receipts four lines above it. |
| `R2B-2` | VALID BUT ACCEPT AS RESIDUAL | low (re-severitised DOWN from medium) | Reachability and the check gap reproduce; the INDETERMINACY claim does not follow, because line 192's rule sentence is categorical. The enumeration foothold closes under `R2A-4`'s deletion at no extra cost. |
| `R2B-3` | VALID BUT ACCEPT AS RESIDUAL | low (unchanged) | Nothing asserted is false; the prescribed remedy is authored prose in the artifact class this project has measured as re-seeding, and both decisions are already attributed at lines 19, 20, 192 and 269. |

## Deduplication: `R2A-3` and `R2B-1` ARE the same defect. CONFIRMED and MERGED

Both lenses cite the same two sentences (`workflow-enforcement-tier.md:269`, closing cost (iii), and `:346`, closing check 19b), both measure the same claim ("the same layout is ALREADY refused in its no-`--source` spelling"), and both rule it false. `R2B-1` adds the diagnosis `R2A-3` does not have: the claim is true for a DIFFERENT command that neither site states, and the fix pass produced the false generalisation by dropping the `--metrics` qualifier when transcribing the round 1 triage's own bound. `R2A-3` adds what `R2B-1` does not: that the present-tense "ALREADY" is separately false, because pre-inc2 nothing is refused at all.

One defect, two independent measurements of it, counted ONCE. The independent convergence is evidence of validity, not a reason to double-count, and the merged severity takes the HIGHER of the two ratings.

## `R2A-3` / `R2B-1` VALID, RE-SEVERITISED to high. The bound is FALSE as written and true only under TWO named qualifications

I built the fixture and ran BOTH no-`--source` spellings myself rather than reusing either reviewer's.

THE TEXT. `:269`, closing cost (iii): "THE BOUND, measured: the same layout is ALREADY refused in its no-`--source` spelling, so this removes a rescue rather than introducing a species." `:346`, closing check 19b: "The same layout in its no-`--source` spelling is refused too, which is what makes this a removed rescue rather than a new species."

SPELLING 1, the literal reading (drop `--source`, add nothing):

```
$ "$BIN" validate --plan "$SC/fixT/notes/p.md" --workflow
no metrics log at /tmp/claude-1000/tri-ep2/fixT/notes/docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
/tmp/claude-1000/tri-ep2/fixT/notes/p.md: 1 steps, 0 open-questions items, valid
exit: 0
```

NOT REFUSED, and it CANNOT BE, in any build this step ships. With no `--source` the anchor for the default IS the checked plan, so `src/main.rs:resolve_metrics_path` joins `METRICS_RELATIVE` onto the same root the predicate would be checked against, and a path built from the root is trivially under it. The run never reads the project's real log at all. This is cost (i)'s shape, and cost (i)'s own text at `:265` gives exactly this reasoning: "the wrong path is still inside the right project: containment is not correctness". Inc3's tier policy would eventually turn this into a hard failure, but that is a missing-log error, not a `Q-55-endproperty` containment refusal, and check 19b sits before check 20's "AFTER INC3" heading.

SPELLING 2, with an explicit `--metrics` naming the project's real log:

```
$ "$BIN" validate --plan "$SC/fixT/notes/p.md" --metrics "$SC/fixT/docs/metrics/workflow.jsonl" --workflow
/tmp/claude-1000/tri-ep2/fixT/docs/metrics/workflow.jsonl: 253 records, valid
/tmp/claude-1000/tri-ep2/fixT/notes/p.md: 1 steps, 0 open-questions items, valid
/tmp/claude-1000/tri-ep2/fixT/notes/p.md vs .../fixT/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

Green today. Derived for post-inc2: the checked plan is `notes/p.md`, its root is `.../fixT/notes` through `src/main.rs:project_root_of_source`'s `parent.to_path_buf()` fallback, the named log is at `.../fixT/docs/metrics/`, which is not under it, so the predicate fires and the run is refused. Refused under an anchor rooting too, since with no `--source` the anchor and the checked plan are the same file.

THE PLAIN STATEMENT ASKED FOR. The bound is FALSE as written, and TRUE ONLY UNDER TWO QUALIFICATIONS, both of which the fix pass dropped:

1. The no-`--source` spelling must carry an EXPLICIT `--metrics` naming the project's own log. Without it the run is a silent miss, not a refusal, in every build.
2. It is a statement about a POST-INC2 build (either rooting), not about today's binary. The word "ALREADY" and the present-tense "is refused too" both assert something about the shipped tool that is measurably false: today, both spellings exit 0.

Under those two qualifications the SUBSTANCE survives: the species "a `--plan` outside any `docs/plans` paired with the project's own log elsewhere" is refused by BOTH candidate inc2 rootings, and what the checked-plan rooting removes is the rescue that a `--source` inside `docs/plans` gives it. So "this removes a rescue rather than introducing a species" is a true statement about the two candidate rootings and a false statement about the tool as it stands.

WHERE THE FALSE GENERALISATION CAME FROM, since this is fix-induced and the provenance matters for the human correction. The round 1 triage wrote the bound WITH both qualifiers intact (`endproperty-fold-r1-triage.md:144`): "with `--plan .../fixC/notes/p.md --metrics .../fixC/docs/metrics/workflow.jsonl` and no `--source`, the anchor IS the plan ... and an anchor-rooted inc2 refuses it too (measured green today, exit 0, which is the pre-inc2 state)". Its own prescription four items later (`:167`) compressed it to "and that the same layout is already refused in its no-`--source` spelling", and the fix pass transcribed the compression. The compression is where both qualifiers were lost.

WHY HIGH RATHER THAN MEDIUM. `R2A-3` rated this medium; I take `R2B-1`'s high. Two grounds. First, check 19b is an executable instruction whose closing clause, run exactly as written on a correct and complete inc2 build, gives exit 0 with a stderr note rather than a refusal, so it exits 0 both before and after the change and settles nothing. That is precisely the defect `EX-2` was raised about last round, reproduced in the fix for a different finding. Second, the bound was RELAYED TO THE HUMAN as a ground for accepting cost (iii); a falsified decision ground is what took `EX-3` to high last round, and the same reasoning applies with the same force.

IS THE HUMAN OWED A CORRECTION? YES. The falsified bound is not confined to the plan: `docs/plans/agent-scaffold.ledger.md:407` carries it in the same dropped-qualifier form ("the layout is ALREADY refused today in its no-`--source` spelling, so the amendment removes a rescue rather than introducing a new species"), inside the paragraph that records what was put to the human before `Q-55-conventionlesscost` was taken. The human accepted a cost described as "a removed rescue rather than a new species", and that description is true only of a post-inc2 build under an explicit `--metrics`, not of the tool today and not of the plain spelling at all.

IS A RE-DECISION OWED? NO, on measured grounds rather than preference. The decision's own recorded ground at `agent-scaffold.ledger.md:409` is the `Q-55-noconvention` reversal ("Carving it out was declined because it would REVERSE `Q-55-noconvention`"), and the bound is not part of it; the bound appears only in the correction paragraph at `:407`. The substance of the bound survives under the two qualifications, so the comparison the human made is unchanged. What is owed is the correction, not a re-decision. The correction lands in the LEDGER by APPENDING, which is the orchestrator's file and the round 1 precedent for decision-time prose, not the planner's to edit.

MINIMAL FIX, AND IT IS DELETION AT BOTH SITES. `R2A-3` proposed narrowing at `:269`; I decline the narrowing and take `R2B-1`'s deletion alternative, on four grounds. (1) This project's standing remedy for a falsified affirmative claim is deletion, and this round exists because the last pass authored 498 words. (2) The bound is not load-bearing: the decision's recorded ground is the `Q-55-noconvention` reversal, which is already stated in the sentence immediately before it. (3) A narrowed bound must carry BOTH qualifiers to be true, which is more authored words than the deletion. (4) A narrowed claim in this file has now been falsified twice in two rounds (line 189, line 309), and a deleted claim cannot be falsified a third time.

- `:269`: delete "THE BOUND, measured: the same layout is ALREADY refused in its no-`--source` spelling, so this removes a rescue rather than introducing a species." TWENTY-THREE WORDS DELETED, 0 ADDED.
- `:346`: delete "The same layout in its no-`--source` spelling is refused too, which is what makes this a removed rescue rather than a new species." TWENTY-TWO WORDS DELETED, 0 ADDED.

FORTY-FIVE WORDS DELETED, 0 ADDED.

SITE COUNT MEASURED: 2 AUTHORED (`workflow-enforcement-tier.md:269`, `:346`), 2 MECHANICAL (`agent-scaffold.md:1664`, `:1741`, regenerated by `render`), 1 LEDGER (`agent-scaffold.ledger.md:407`, corrected by APPENDING and owned by the orchestrator). `grep -rn "no-\`--source\` spelling"` and `grep -rn "removed rescue\|removes a rescue\|new species\|introducing a species"` over `docs/plans/agent-scaffold.steps/`, `docs/plans/agent-scaffold.plan.toml`, the projection and the ledger return exactly those five lines. `docs/plans/agent-scaffold.plan.toml` carries ZERO occurrences of either phrasing and zero mentions of `Q-55-conventionlesscost` or `Q-55-resumepairing` (measured, `grep -c`).

## `R2A-2` VALID, high (unchanged). The precondition that makes the cost occur is unstated, so as written the layout is not refused and check 19b does not discriminate

Reproduced by building the two fixtures that differ in `[meta].primary` and NOTHING else, and running the check as written.

```
$ "$BIN" validate --source "$SC/fixT/.../TEMPLATE.plan.toml" --plan "$SC/fixT/notes/p.md" --workflow      # primary = "toml"
.../fixT/docs/metrics/workflow.jsonl: 253 records, valid
.../fixT/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
.../fixT/notes/p.md: generated projection of a TOML-primary source; skipping the Markdown plan validator
.../fixT/docs/plans/TEMPLATE.plan.toml vs .../fixT/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0

$ "$BIN" validate --source "$SC/fixM/.../TEMPLATE.plan.toml" --plan "$SC/fixM/notes/p.md" --workflow      # primary = "markdown"
.../fixM/docs/metrics/workflow.jsonl: 253 records, valid
.../fixM/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
.../fixM/notes/p.md: 1 steps, 0 open-questions items, valid
.../fixM/notes/p.md vs .../fixM/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The first slot of the `vs` line names THE PLAN THE CHECK READS, which is what line 167 of the same file says it names. In the TOML-primary spelling that is `<root>/docs/plans/TEMPLATE.plan.toml`, whose root through `project_root_of_source` is `<root>` (the walk finds the `docs/plans` ancestor and returns its grandparent, never reaching the `parent.to_path_buf()` fallback). The log at `<root>/docs/metrics/workflow.jsonl` IS under `<root>`, so the predicate does not fire and check 19b's "exits NON-ZERO after it" is false. In the Markdown-primary spelling the checked plan is `<root>/notes/p.md`, its root is `<root>/notes` through the fallback, and the cost occurs exactly as written.

THE PROJECTIONS HALF IS FALSE IN THE SAME SPELLING, measured rather than derived, since `run_status` and `run_next` make the same selection through `toml_source(&args.source)`:

```
$ "$BIN" status --source "$SC/fixT/.../TEMPLATE.plan.toml" --plan "$SC/fixT/notes/p.md"
plan: 1 steps (1 not started); 0 open-questions items
metrics: 253 records                                                                     exit: 0

$ "$BIN" next --source "$SC/fixT/.../TEMPLATE.plan.toml" --plan "$SC/fixT/notes/p.md"
source: /tmp/claude-1000/tri-ep2/fixT/docs/plans/TEMPLATE.plan.toml
metrics: 253 records

$ "$BIN" next --source "$SC/fixM/.../TEMPLATE.plan.toml" --plan "$SC/fixM/notes/p.md"     # contrast
source: /tmp/claude-1000/tri-ep2/fixM/notes/p.md
metrics: 253 records
```

`next`'s echoed `source:` line names the checked plan directly and confirms the split: the TOML-primary run projects from the source (root `<root>`, log under it, so no omission after inc2), the Markdown-primary run projects from `notes/p.md` (root `<root>/notes`, log not under it, so the omission happens). Cost (iii)'s "`status` and `next` omit their metrics half" holds only in the second spelling.

THE CONCLUSION FOLLOWS FROM THE CITATION AND I CHECKED IT BOTH WAYS. This is not a hypothetical: the shipped skeleton declares `primary = "toml"` (`pack/plan-template.plan.toml:15`, emitted verbatim by `scaffold` and measured on the fixture), and `src/plan/source.rs:primary_defaults_to_markdown_when_absent` confirms the ABSENT case defaults the other way. So a file spelled `x.plan.toml` built the obvious way is TOML-primary and the cost does not occur; a file with `[meta].primary` absent or set to `"markdown"` is Markdown-primary and it does. The round 1 triage measured direction 1 on a MARKDOWN-PRIMARY fixture (`endproperty-fold-r1-triage.md:34`, "`fixC`: one project, Markdown-primary `docs/plans/TEMPLATE.plan.toml`"), and the transcription into cost (iii) dropped that property of the fixture. Fix-induced, in scope.

SEVERITY CONFIRMED AT HIGH, not lowered. Check 19b pins an ACCEPTED COST that the same section says "an implementer must NOT fix" and "a reviewer must NOT raise". An implementer who builds it the obvious way gets exit 0 after inc2 and concludes the guard is broken; the two available repairs are to root on the `--plan` even when the source is TOML-primary (which breaks check 13b's THIRD run, the no-regression side) or to require the two named paths to share a root (which breaks the region claim at line 169). A wrong pinning check here is a route to a real behavioural defect on the one class of item the file forbids touching.

MINIMAL FIX, AND IT IS A NARROWING, NOT AUTHORED RATIONALE. At `:269`, the cost's own heading gains one word: "WITH A `--source` INSIDE ONE" becomes "WITH A MARKDOWN-PRIMARY `--source` INSIDE ONE". At `:346`, check 19b's fixture description gains the same property on `x.plan.toml`, about three words. ABOUT FOUR WORDS ADDED ACROSS TWO SITES, 0 deleted. Nothing smaller works: without the qualifier the run does not exhibit the cost at all, in either direction, and with it the whole rest of both passages is correct as written and needs no other change. Nothing bigger is needed: check 13b already uses this exact wording for the same requirement ("give fixture A a clean MARKDOWN-primary `<task>.plan.toml` (`[meta].primary = "markdown"`, or absent, which defaults to it)"), so this is the file's established vocabulary rather than new prose.

SITE COUNT MEASURED: 2 AUTHORED (`workflow-enforcement-tier.md:269`, `:346`), 2 MECHANICAL (`agent-scaffold.md:1664`, `:1741`). `grep -rn "notes/p.md"` over `docs/plans/agent-scaffold.steps/`, `docs/plans/agent-scaffold.plan.toml`, the projection and the ledger returns those four lines plus `agent-scaffold.ledger.md:407`; the ledger line is the `EX-3` correction paragraph and carries the same under-specification, so it is part of what the orchestrator's APPENDED correction owes the human (below), not a planner edit.

## `R2A-4` VALID, medium (unchanged). The same-root rule reaches a third case, on a single-project layout that works today, and that case is recorded nowhere

Reproduced, and I ran the control and BOTH `primary` spellings rather than reusing the reviewer's single run.

```
$ "$BIN" status --resume --source "$SC/fixT/.../TEMPLATE.plan.toml" --plan "$SC/fixT/notes/p.md"      # primary = "toml"
## RESUME STATE

FIXTURE-fixT-SECRET-RESUME-LINE: internal state of fixT                                   exit: 0

$ "$BIN" status --resume --source "$SC/fixT/.../TEMPLATE.plan.toml"                                   # control, one path alone
## RESUME STATE

FIXTURE-fixT-SECRET-RESUME-LINE: internal state of fixT                                   exit: 0

$ "$BIN" status --resume --source "$SC/fixM/.../TEMPLATE.plan.toml" --plan "$SC/fixM/notes/p.md"      # primary = "markdown"
## RESUME STATE

FIXTURE-fixT-SECRET-RESUME-LINE: internal state of fixT                                   exit: 0
```

One project. The ledger read is `<root>/docs/plans/TEMPLATE.ledger.md`, the project's OWN ledger, beside its own plan source (`src/main.rs:default_ledger_path`, "source.as_ref().or(plan.as_ref())" then the sibling join). Nothing foreign is involved. The two named paths resolve to DIFFERENT roots (`<root>` through the `docs/plans` walk, `<root>/notes` through the fallback, both in `src/main.rs:project_root_of_source`), so line 192's rule ("a `--source` AND a `--plan` BOTH NAMED MUST RESOLVE TO THE SAME ROOT OR THE BLOCK IS OMITTED") withholds the block.

IT IS NEITHER OF THE TWO NAMED CASES. Case 1 is "an explicit `--ledger-fragment` outside that root"; there is no fragment here. Case 2 is "the DEFAULT ledger under a divergent pairing", and this file defines a divergent pairing at line 169 as "a Markdown-primary `--source` in one project paired with a `--plan` in another"; this is one project, and the third run above shows the rule fires on the TOML-primary spelling too, because `status --resume` reads no plan and the rule never consults `primary` at all.

IT IS THE ONE CONSEQUENCE OF COST (iii)'S LAYOUT THAT SURVIVES `R2A-2`, and this interaction is what makes it more than a count error. After `R2A-2`'s narrowing, cost (iii)'s metrics consequences are conditional on a MARKDOWN-primary `--source`. The resume consequence is not: it fires on both spellings. So the resume rule reaches a STRICTLY WIDER population than the cost the human accepted, and folding it into the narrowed cost (iii) without saying so would under-state it.

IS IT A NEW COST THE HUMAN MUST ACCEPT? YES, and I checked the decision record before saying so rather than assuming the planner over-reached. `docs/plans/agent-scaffold.ledger.md:409` records `Q-55-resumepairing`'s framing in the human's own decided terms: "where a surface reads NO plan the two NAMED plans must agree or the block is omitted, which SUPPLIES a root where there is no checked plan rather than inventing a second rule". The RULE at line 192 is therefore a faithful transcription and is NOT the defect. The defect is that the same ledger paragraph states the DEFECT the rule was chosen to close as a cross-project leak ("`status --resume --source A --plan B` prints project A's `## RESUME STATE`"), so what the human was shown and what the general rule reaches are not the same set, and the difference is a new refusal on a single-project layout that works today. Line 263's convention governs exactly this: "Each was measured, each was put to the human, and each was ACCEPTED". This one was measured here for the first time and has not been put. The round 1 precedent is directly on point: `EX-3`'s direction 1 was a measured new false positive on a working layout, was ruled the human's to accept rather than the planner's to assume, and became cost (iii).

SEVERITY HELD AT MEDIUM, and I considered the escalation the reviewer invited. `EX-3` went to high on TWO counts, a falsified ground that had been RELAYED to the human and a new measured false positive. Only the second count is present here: nothing false was relayed about the resume rule, the rule is the human's own words, and the failure direction is over-refusal on a best-effort projection at exit 0 rather than a false assertion. One of two counts is medium, not high. The ROUTING is what matters more than the number, and it is stated above.

MINIMAL FIX, IN TWO PARTS, AND THE SECOND PART IS BLOCKED ON THE HUMAN.

1. DELETION, available now. At `:192`, "TWO CASES REACH IT:" becomes "IT IS REACHED BY", leaving the two named cases standing as the examples they are. THREE WORDS CHANGED, 0 NET ADDED. A deleted count cannot go stale when a third case is found, which is what just happened, and this deletion also closes `R2B-2` (below) at no extra cost.
2. THE COST RECORD, only after the human accepts or carves out. One sentence at `:269` naming the surface and stating that it fires REGARDLESS of the source's `primary`, which is what keeps it consistent with `R2A-2`'s narrowing of the same passage; and about eight words in check 19b at `:346`. ABOUT TWENTY WORDS AT `:269` PLUS EIGHT AT `:346`. Nothing smaller works: the round 1 triage already established that a cost this step's convention says must be PUT cannot be recorded by deletion, and a shorter form that inherited cost (iii)'s new `primary` qualifier would be false, since the resume rule does not carry it. The reviewer's twelve-word estimate is too small for exactly that reason.

SITE COUNT MEASURED: 2 AUTHORED (`workflow-enforcement-tier.md:192`, `:269`, plus `:346` if the cost record is taken, so 2 or 3), 2 or 3 MECHANICAL. `grep -rn "TWO CASES REACH IT"` and `grep -rn "SAME root or the block is omitted"` over `docs/plans/agent-scaffold.steps/`, `docs/plans/agent-scaffold.plan.toml`, the projection and the ledger each return exactly one authored line and one projection line; `grep -rn "SUPPLIES a root"` returns those two plus `agent-scaffold.ledger.md:409`, which is the decision record and is not the planner's to edit. `docs/plans/agent-scaffold.plan.toml` carries no occurrence of any of the three.

## `R2A-1` VALID, medium (unchanged). The narrowing at line 309 was falsified inside the commit that made it

Both citations resolve and the conclusion follows. Line 309: "rooting the guard on the anchor is a defect that check 11 passes over and only check 13b and 14g's fourth run catch." Line 335, check 14c's third run, authored in the same pass: "check 13b's divergent pairing, A carrying a `## RESUME STATE` block, must give the same note, no line of A's block, and exit 0, where before inc2 it prints A's block verbatim AND AN INC2 THAT LEFT THIS SURFACE ANCHOR-ROOTED STILL WOULD."

I verified the discrimination rather than taking the clause's word for it. Under an anchor rooting for `status --resume`, the anchor is A's source, its root is A's root, and the default ledger is beside A's source and therefore under A's root, so the block prints. Under the decided rule the two named plans disagree and the block is omitted. So 14c's third run separates the two rootings, which is what "catch" means in line 309's sentence. Three checks catch it; the sentence names two and says "only". Measured pre-inc2, the divergent pairing does print A's block on the default ledger at exit 0 (run under `R2B-2` below), so the run is a genuine red and not a check that passes before the change.

MINIMAL FIX, AND IT IS A DELETION. At `:309`, delete "and only check 13b and 14g's fourth run catch", leaving "... so rooting the guard on the anchor is a defect that check 11 passes over." NINE WORDS DELETED, 0 ADDED. I take the deletion over a second narrowing on the project's own recorded ground: a narrowed exhaustiveness claim is falsified again by the next check anyone adds, and this one was falsified inside the same commit. Nothing is lost, because each check states its own discriminating role in its own text (13b, "this check, and not check 11, is what separates the two rootings on the validator"; 14g, "THIS RUN, NOT CHECK 14b, is what separates an anchor-rooted projection from a checked-plan-rooted one"; 14c, the clause quoted above).

SITE COUNT MEASURED: 1 AUTHORED (`workflow-enforcement-tier.md:309`), 1 MECHANICAL (`agent-scaffold.md:1704`). `grep -rn "only check 13b"` over `docs/plans/agent-scaffold.steps/`, `docs/plans/agent-scaffold.plan.toml`, the projection and the ledger returns exactly those two lines.

## `R2A-5` VALID, medium (unchanged). The pass narrowed one clause and deleted the referent of its twin

Both citations resolve and both edits are in the fix pass's own diff. Line 189 now reads "The trigger is the SAME containment predicate the validator's refusal uses ... The predicate is never re-implemented per surface (One source of truth), and neither is the selection of the plan to root on: it is the selection each surface already makes, enumerated at the end of the mechanism section." Line 173, the end of the mechanism section, now reads "`status --resume` is the one surface that reads NO plan ... so it has no checked plan to root on."

Confirmed against `git diff a9dda1c 3354a90`: the same commit removed "in all three cases" from line 189's first clause (the `EX-4` fix) AND deleted line 173's continuation "and its root falls back to the source-then-plan anchor `default_ledger_path` already uses, which leaves that surface's rule exactly as it was before this decision". Deleting that continuation was correct, since `Q-55-resumepairing` falsified it. The consequence is that the enumeration line 189 points at now gives NO root for one of the three bullets the section covers, and the rule for that surface exists in exactly one place, the per-surface bullet at line 192, which is the thing line 189's second clause says does not happen. The bullet's own hedge ("so the rule SUPPLIES a root rather than being re-implemented per surface") asserts the conclusion rather than establishing it.

Confirmed against the code by symbol: `src/main.rs:run_resume` reads only the ledger path, its task coming from `next::derive_task(&args.source, &args.plan)`, which reads filenames rather than contents, so line 173 is accurate and line 189's second clause has no referent for that surface.

MINIMAL FIX, AND I TAKE THE DELETION OVER THE REVIEWER'S NARROWING. At `:189`, delete ", and neither is the selection of the plan to root on: it is the selection each surface already makes, enumerated at the end of the mechanism section", leaving "The predicate is never re-implemented per surface (One source of truth)." TWENTY-FOUR WORDS DELETED, 0 ADDED. The reviewer's four-word narrowing ("each surface THAT READS A PLAN") also works and is smaller in words, but this exact sentence has now been falsified in two consecutive rounds, once per clause, and the project's measured remedy for a repeatedly-falsified affirmative claim is deletion. The routing it carried is not lost: line 173 remains the section's plan-selection enumeration, and the bullets at 191 and 193 name `src/main.rs:run_status` and `src/main.rs:run_next` and their `toml_source(&args.source)` branch directly. IF the routing is judged load-bearing, the four-word narrowing is the fallback and I would accept it.

SITE COUNT MEASURED: 1 AUTHORED (`workflow-enforcement-tier.md:189`), 1 MECHANICAL (`agent-scaffold.md:1584`). `grep -rn "enumerated at the end of the mechanism section"` over `docs/plans/agent-scaffold.steps/`, `docs/plans/agent-scaffold.plan.toml`, the projection and the ledger returns exactly those two lines.

## `R2A-6` VALID, low (unchanged). Three invocations, "both exit 0"

Citation resolves. Line 339 names `next`, `next --json` and `status --json`, then closes "and both exit 0". Three invocations, two commands; the antecedent reads equally as either. Practically nothing is unpinned, because check 14 blankets the class ("`status` and `next` NEVER exit non-zero under any of these inputs"), which is why this stays low. It is a defect rather than a nit only because check 14b in the same file makes an explicit point of not leaving exit codes implied.

MINIMAL FIX, A DELETION. Delete "and both exit 0" at `:339`. FOUR WORDS DELETED, 0 ADDED. Nothing is lost, per check 14. The one-word alternative ("all three exit 0") re-states what check 14 already covers and can go stale if a fourth invocation is added.

SITE COUNT MEASURED: 1 AUTHORED, 1 MECHANICAL. `grep -rn "and both exit 0"` over the full population returns exactly those two lines.

## `R2A-7` VALID, low (unchanged). The count is five

Recounted from the receipts rather than from the reviewer's list. `grep -o '"q_id":"Q-55[^"]*"' docs/metrics/workflow.jsonl | sort -u` returns NINE ids, matching line 10's "NINE decision receipts" and the nine provenance bullets at lines 12 to 20 one for one. Line 24 names `Q-55-mechanism` and `Q-55-noconvention` separately as "the design pass", so the "further human decisions" are `Q-55-refusalscope`, `Q-55-jsonreason`, `Q-55-endproperty`, `Q-55-conventionlesscost` and `Q-55-resumepairing`: FIVE, not two.

IN SCOPE, and I checked the four-condition precedent explicitly. Provenance PREDATES the base commit: yes, the sentence is older. No commit in range modifies the line: CONFIRMED, `git diff 45cb6d2 3354a90 -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` contains zero hunks touching "THIS FILE IS THE SECOND PLANNER PASS", and only two commits in the range touch the file at all (`a9dda1c`, `3354a90`). The increment's own change did not falsify it: FAILS, and that is why the finding stands. The fold at `a9dda1c` added `Q-55-endproperty` (taking the true count to three) and the fix pass at `3354a90` added two more receipts to the provenance list FOUR LINES ABOVE the stale count (taking it to five), verified in the diff. No shared fix: not reached, since condition three fails. Three of the four checked; the third defeats the precedent.

MINIMAL FIX, A DELETION. "and two further human decisions" becomes "and further human decisions". ONE WORD DELETED, 0 ADDED. Replacing "two" with "five" is the same size and goes stale on the next decision, which is what has now happened twice.

SITE COUNT MEASURED: 1 AUTHORED (`workflow-enforcement-tier.md:24`), 1 MECHANICAL (`agent-scaffold.md:1419`). `grep -rn "further human decisions"` over the full population returns exactly those two lines.

## `R2B-2` VALID BUT ACCEPT AS RESIDUAL, RE-SEVERITISED DOWN to low. The facts reproduce; the indeterminacy does not follow

THE FACTS REPRODUCE. `src/main.rs:StatusArgs::ledger_fragment` carries `#[arg(long, requires = "resume")]` and nothing tying it to either plan flag, so all three can be given together, and I confirmed the combination is a live CLI state today on the divergent A/B pairing:

```
$ "$BIN" status --resume --source "$SC/A/docs/plans/TEMPLATE.plan.toml" \
    --plan "$SC/B/docs/plans/TEMPLATE.md" \
    --ledger-fragment "$SC/A/docs/plans/TEMPLATE.ledger.md"
## RESUME STATE

FIXTURE-A-SECRET-RESUME-LINE: internal state of A                                        exit: 0

$ "$BIN" status --resume --source "$SC/A/..." --plan "$SC/B/..."       # 14c's third run, the control
## RESUME STATE

FIXTURE-A-SECRET-RESUME-LINE: internal state of A                                        exit: 0
```

The check gap reproduces too, verified against the check text rather than the reviewer's summary: check 14c's fragment clause at `:335` names "a ledger outside the plan's root" with no divergence, its third run says "with no `--ledger-fragment` at all", and check 14g's fragment clauses at `:339` are the same shape on `next`. So no check exercises fragment-plus-divergence.

THE INDETERMINACY CLAIM DOES NOT FOLLOW, and this is the half I reject. Line 192's rule sentence is categorical: "a `--source` AND a `--plan` BOTH NAMED MUST RESOLVE TO THE SAME ROOT OR THE BLOCK IS OMITTED". "Or the block is omitted" states the outcome for disagreement without conditioning it on which ledger path is in play, so an implementer reading the specification sentence builds Reading 1. Reading 2 needs the provenance bullet at line 20 ("close the `status --resume` default-ledger leak in inc2") to narrow the specification sentence, which is not a reading the sentence supports. What DOES give Reading 2 a foothold is the enumeration that follows it: "TWO CASES REACH IT" defines case 1 relative to "that root", which does not exist under divergence, so the enumeration looks like it is carving the fragment case out when it is merely short by one. That is `R2A-4`'s defect, not a separate one, and `R2A-4`'s prescribed deletion of the count removes the foothold at no extra cost.

WHY RESIDUAL RATHER THAN A FIX. The reviewer's remedy is one new sentence at `:192` plus one new run on check 14c. Both are authored prose in the artifact class this project has measured as re-seeding: five of this round's seven fix-required findings are in text the last pass authored. Against that, the combination's answer is already forced by the sentence, a Reading-1 build is what the text produces, and the surface is a best-effort projection at exit 0. Accepting the check-coverage gap costs one untested combination out of eight in the `{--source, --plan} x {agree, diverge} x {fragment, default}` matrix; authoring a run to close it re-opens a paragraph two other findings are already deleting from. RESIDUAL, and recorded here so a later reviewer finds the decision rather than re-raising it.

SITE COUNT MEASURED: 0 additional authored sites. The remedy is `R2A-4`'s deletion at `workflow-enforcement-tier.md:192`, already counted there.

## `R2B-3` VALID BUT ACCEPT AS RESIDUAL, low (unchanged). Nothing asserted is false

The citations resolve. The inc2 increment description at `:286` names `Q-55-endproperty`, `Q-55-refusalscope` and `Q-55-jsonreason` and not the two new decisions; the risk paragraph at `:309` names the same three. Both gaps are real.

BUT NOTHING IN EITHER PARAGRAPH IS FALSE, which is what separates this from `R2A-1` and `R2A-5`. `Q-55-refusalscope` did settle that `status --resume` omits the block (for the explicit-fragment case); `Q-55-resumepairing` widened which cases reach it without falsifying the attribution. Cost (iii) is recorded in the accepted-costs section, pinned by check 19b, and cited by q_id at `:269`; both decisions appear in the provenance list at `:19` and `:20` and in the `status --resume` bullet at `:192`. A reader has the attribution; two summary paragraphs do not repeat it.

WHY RESIDUAL. The remedy is "one clause on each paragraph", which is authored prose added to a risk paragraph that `R2A-1` is simultaneously deleting a clause from, and to an increment description nothing else touches. Adding to `:309` while deleting from it in the same pass is the shape that manufactures the next round's finding. If a fix is taken anyway despite this ruling, the smallest true one is at `:309` only: "accepted cost (ii), the symlinked `docs/plans` directory" becomes "accepted costs (ii) and (iii)", which is TWO WORDS ADDED and one parenthetical narrowed, and I would accept that. I do not prescribe it.

SITE COUNT MEASURED: 2 AUTHORED (`workflow-enforcement-tier.md:286`, `:309`) if a fix is taken, 2 MECHANICAL. No fix prescribed.

## Fix closure on round 1, spot-checked rather than assumed

I did not re-run the whole round 1 closure survey, since that was the residue lens's assignment and it built the fixtures for it. I did spot-check the four closures that bear on this round's findings, each by grep over the full population rather than by reading the reviewer's report:

- `EX-3`: `grep -rin "superset"` over `docs/plans/agent-scaffold.steps/`, `docs/plans/agent-scaffold.plan.toml` and the projection returns no `Q-55` site. The ledger correction landed as an APPEND at `agent-scaffold.ledger.md:407` with the decision-time record at `:395` intact, which is what the round 1 triage prescribed, and it was made in `c10867e` rather than in the fix pass, which is why it is absent from `git diff a9dda1c 3354a90`. CLOSED, and its `:407` text is the site the human correction under `R2A-3`/`R2B-1` must now amend.
- `EX-4`: `grep -rn "in all three cases"` returns no sidecar line. CLOSED; the residue is `R2A-5`.
- `EX-5`: the end-property clause at `:286` is gone and `:315`'s conditional "the one that decides whether this increment closes the end property" stands, which is the round 1 ruling. CLOSED.
- `FI-1`: `grep -rn "survives anchoring\|precisely what survives\|exactly the case that survives"` over the sidecars, `agent-scaffold.plan.toml` and the projection returns nothing; the plan-source site at `agent-scaffold.plan.toml:1710` is confirmed deleted in `git diff a9dda1c 3354a90 -- docs/plans/agent-scaffold.plan.toml`. The two frozen ledger records at `:587` and `:591` still carry the falsified phrasing and I am NOT raising them, for the reason the round 1 triage recorded: decision-time ledger prose is corrected by APPENDING, and re-litigating a measured site set is not this round's job. CLOSED.

None of round 2's findings re-opens a round 1 finding. Every fix-required finding here is in material the fix pass authored on 2026-08-02 or in a claim its own deletions falsified.

## Out of scope, checked and not raised

Nothing was raised by either reviewer from the excluded set, and I confirmed the exclusions still hold rather than assuming it. The `--metrics` relative-default text at `:112`, the `default_ledger_path` current-directory text at `:139`, the "Documentation impact INC1" sub-list at `:355` to `:357` and the two help-string descriptions are all falsified by INCREMENT 1 rather than by this amendment, and all four conditions of the precedent hold for them: provenance predates the base, no commit in range touches the lines (`git diff 45cb6d2 3354a90` on the sidecar has two commits and neither hunk reaches them), the subject is independent of the rooting change, and there is no shared fix with anything above. Deferred to the documentation-currency pass after inc3, as scoped. Accepted costs (i) and (ii), increments 1 and 3, and the two human decisions themselves are untouched here. No line-length or hard-wrapping observation was made by either reviewer or by me.

## Backstop

NO finding rated `high` or `critical` was dismissed. No finding at any severity was dismissed. NO RE-CHECK IS OWED on this round's triage.

## Convergence judgement

CONVERGING ON THE MECHANISM, DIVERGING ON THE PROSE, and the two need to be read separately.

The mechanism, the increment split, the risk classifications, the accepted-cost set and the acceptance-check structure have survived two rounds and four reviewer lenses without a single falsified claim among them. Every one of this round's nine findings is in material added in the last two days by decision folds and fix passes: five in text the round 1 fix pass authored, two in claims its own deletions falsified, and two in summary paragraphs it did not extend. Nothing older than `a9dda1c` was falsified this round.

The prose is diverging in COUNT and converging in COST. The count went 6 to 9. The remedy did not: round 1's fix pass authored roughly 498 words, and this round's prescription DELETES about 106 words and AUTHORS about 4 immediately, plus about 28 more contingent on a human answer. Both `high` findings are in the SAME two lines (`:269` and `:346`), which is one paragraph and one check, both written by the fix pass, and both close by deletion plus a one-word narrowing. That is the project's own measured pattern operating exactly as its risk paragraph predicts: a prose-authoring fix pass manufactures the next round's finding, and a deletion-class pass re-seeds nothing.

THE NEXT ROUND SHOULD NOT BE A PLAIN FIX-PLUS-ROUND. Two of the nine findings need a HUMAN input that no fix pass can supply, and running the fix first means the planner either invents the human's acceptance or writes a cost record it must rewrite next round, which is the re-seeding pattern by another route. The sequence I recommend is ESCALATE, THEN FIX, THEN ROUND 3:

1. ESCALATE to the human, as one item with two parts. Part A, the CORRECTION owed under `R2A-3`/`R2B-1`: the bound they were given for `Q-55-conventionlesscost` is false as stated and true only with an explicit `--metrics` and only of a post-inc2 build; the substance survives, no re-decision is indicated, and the correction lands by APPENDING to `agent-scaffold.ledger.md` beside `:407`. Part B, the ACCEPT-OR-CARVE-OUT call under `R2A-4`: the same-root rule withholds a single project's own resume block on the cost (iii) layout, in both `primary` spellings, which is a wider population than the cost they accepted. `R2A-2`'s measurement belongs in the same escalation as context, since it changes what cost (iii) actually costs.
2. FIX, once Part B is answered. The pass is deletion-dominant by construction: about 106 words deleted across `:24`, `:189`, `:192`, `:269`, `:309`, `:339` and `:346`, about 4 words added for `R2A-2`'s narrowing, and about 28 more only if the human accepts the resume cost. Re-render, and re-run both guards before committing.
3. ROUND 3 on the fixed text. Streak is 0 and this is round 2 of a cap of 5, so the earliest possible convergence is round 4 and there is exactly one spare round.

IF ROUND 3 RETURNS ANOTHER CROP OF FIX-INDUCED PROSE FINDINGS, the evidence at that point supports escalating the ARTIFACT rather than running round 4: three consecutive rounds of prose-only findings on decision folds, with zero mechanism defects across all three, is the step 92 pattern this file's own risk paragraph at `:311` cites as calibration data (six rounds, fifteen findings, all prose, zero mechanism defects). The right response to that pattern is a scope call by the human, not a fourth prose pass.

## Scratch hygiene

Every probe ran with `TMPDIR=/tmp/claude-1000/tri-ep2` and every fixture (`base`, `fixT`, `fixM`, `A`, `B`) was created under it. The directory was removed when this triage finished. DIRECTORIES LEFT IN `/tmp`: 0.
