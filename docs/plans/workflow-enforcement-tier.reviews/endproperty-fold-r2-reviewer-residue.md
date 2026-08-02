# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 2, REVIEWER: fix verification and fix-induced residue

Reviewer: independent of the planner, of both round 1 reviewers, and of the triager. Read-only with respect to the reviewed artifact; this file is the only thing written.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep2-residue`, branch `review/q55-ep2-residue`, at `3354a90` (the round 1 fix pass). Binary built at that commit with `cargo build`, so INC2 IS NOT LANDED and every run below is a PRE-INC2 measurement; every post-inc2 statement is derived from the code plus the amendment's own specified rule and is labelled as derived. All fixtures under `TMPDIR=/tmp/claude-1000/r2a-scratch`, removed at the end.

Repository guards re-run at the reviewed commit, both green: `render docs/plans/agent-scaffold.plan.toml --check` prints `up to date` (exit 0); `validate --source docs/plans/agent-scaffold.plan.toml --workflow` prints `workflow invariants hold` (exit 0). `grep -nP '[^\x00-\x7F]'` over the changed sidecar returns nothing. So the projection is a faithful regeneration and the sidecar is the only authored site for anything in it.

FIXTURES, all built from `agent-scaffold scaffold --output-dir <d> --write --force --principles default` (which emits `[meta] primary = "toml"`, verified at `base/docs/plans/TEMPLATE.plan.toml:15`):

- `fixT`: ONE project. `docs/plans/x.plan.toml` left TOML-primary, `notes/p.md` a copy of the rendered `TEMPLATE.md`, this repository's 253-record log at `docs/metrics/workflow.jsonl`, and `docs/plans/x.ledger.md` carrying a `## RESUME STATE` block with a unique marker line. This is accepted cost (iii)'s layout, written the way cost (iii) writes it.
- `fixM`: `fixT` with `primary = "markdown"`, the only difference.
- `A13`/`B13`/`B13p`: check 13b built exactly as the fix pass now specifies it. `A13` is Markdown-primary with this repository's log; `B13`'s Markdown Roadmap row AND its Step Detail heading both carry `triager-runs-only-on-findings` at `complete`; `B13p` is `B13` at `in progress`.

## Verdict summary

SEVEN findings: one `high`, four `medium`, two `low`. All six round 1 findings are CLOSED at the sites the triage measured (section "Fix closure" below). Every finding here is fix-induced: five of the seven are in text the fix pass authored on 2026-08-02, and the other two are claims its own deletions or additions falsified.

| id | severity | one line |
| --- | --- | --- |
| `R2A-1` | medium | Line 309's narrowed "only check 13b and 14g's fourth run catch" is falsified by check 14c's new third run, authored in the same pass. |
| `R2A-2` | high | Accepted cost (iii) and check 19b omit the load-bearing precondition (the `--source` must NOT be TOML-primary); as written the layout is not refused after inc2 and 19b does not discriminate. |
| `R2A-3` | medium | Cost (iii)'s and 19b's "ALREADY refused in its no-`--source` spelling" bound is false as written: measured exit 0 today in both no-`--source` spellings. |
| `R2A-4` | medium | The new `status --resume` same-root rule reaches a third case its own "TWO CASES REACH IT" excludes, on a single-project layout that works today and that cost (iii) does not record. |
| `R2A-5` | medium | Line 189's second clause still asserts the universality `EX-4` deleted from its first, and the pass's own deletion at line 173 removed the enumeration entry that clause points at. |
| `R2A-6` | low | Check 14g's new metrics half names THREE invocations and then says "both exit 0". |
| `R2A-7` | low | "Two further human decisions all landed after it" is now five. |

## Fix closure: `EX-1` to `EX-5` and `FI-1`

`EX-1` CLOSED, both prescribed sites, and the new run is a genuine red. Site 1, the triage prescribed extending check 14g's FOURTH RUN with the metrics half on B's step at `in-progress`. Line 339 now carries "THE SAME PAIRING PINS THE METRICS HALF, on B's step at `in-progress` rather than `complete` and still with no explicit `--metrics`: `next` prints none of the `ACTIVE LOOP` block and no record count, `next --json` gives `"metrics_absent_reason": "log-not-this-project"` with `"no_active_loop_reason": "metrics-not-this-project"`, `status --json` on the same pairing gives the same `metrics_absent_reason`, and both exit 0. THIS RUN, NOT CHECK 14b, is what separates an anchor-rooted projection from a checked-plan-rooted one." Every element the triage prescribed is present. I built the run and measured the pre-inc2 red rather than assuming it:

```
$ "$BIN" next --source "$SC/A13/docs/plans/TEMPLATE.plan.toml" --plan "$SC/B13p/docs/plans/TEMPLATE.md"
metrics: 253 records

ACTIVE LOOP
  triager-runs-only-on-findings / triager-runs-only-on-findings-inc1  in progress -> mark-step-complete
  state: converged
  streak: 1/1
  rounds: 2/5
  next: mark the step complete, re-render, and commit
  ...
RESUME STATE (verbatim from the ledger):
## RESUME STATE

FIXTURE-A13-SECRET-RESUME-LINE
exit: 0

$ "$BIN" next --source ... --plan ... --json      # "metrics": { "records": 253 }, full "active_loop", no reason fields
$ "$BIN" status --source ... --plan ... --json    # "metrics": { "records": 253 }
```

No explicit `--metrics` anywhere. This is field-for-field the output line 195 says the fix must make unreachable, so the extended run is a red case rather than a check that passes before the change. Site 2, the narrowing at line 309, landed as prescribed but is now false for a different reason; see `R2A-1`.

`EX-2` CLOSED, and I verified it by BUILDING check 13b the way it is now written rather than by reading it. All three prescribed clauses are present at line 332 ("with its Step Detail heading renamed to match"; "carrying CONVERGED ROUNDS for that slug (this repository's own log does)"; "on a TOML-primary third fixture with its own log, or on this repository itself"). Built that way, 13b's stated pre-change observation is exactly what happens:

```
$ "$BIN" validate --source "$SC/A13/docs/plans/TEMPLATE.plan.toml" --plan "$SC/B13/docs/plans/TEMPLATE.md" --workflow
.../A13/docs/metrics/workflow.jsonl: 253 records, valid
.../A13/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
.../B13/docs/plans/TEMPLATE.md: 1 steps, 0 open-questions items, valid
.../B13/docs/plans/TEMPLATE.md vs .../A13/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The unrelated exit 1 the triage reproduced (`Roadmap step ... has no matching Step Detail heading`) is gone, and the W3 red the triage reproduced from a records-only log is gone. The second run reproduces too (`no source plan at .../TYPO.plan.toml` on stderr, then `workflow invariants hold`, exit 0), and the third run on this repository itself exits 0 reading this repository's own log with the TOML source as the checked plan.

`EX-3` CLOSED at all four prescribed sites. Line 169 now reads "Three things the implementer should carry ... In TOML-primary mode the checked plan IS the anchor and the rule is unchanged there; it differs only when a `--source` is given and is not TOML-primary." The superset sentence and "which is exactly the gap" are gone, and I re-counted the list: three items (one predicate not two; the region statement; the typo'd `--source`), matching "Three things". Check 13b's ", and it is the reason this rule is a superset rather than a replacement" is gone. `grep -rin "superset"` over all sidecars, `agent-scaffold.plan.toml` and the projection returns no `Q-55` site at all now. The ledger correction landed as an APPEND at `docs/plans/agent-scaffold.ledger.md:407` ("THE ORCHESTRATOR'S OWN REASONING WAS FALSIFIED BY `EX-3`..."), with the decision-time record at `:395` left intact, which is what the triage prescribed; it was made in `c10867e`, not in the fix pass, which is why it does not appear in `git diff a9dda1c 3354a90`. The new cost was recorded as cost (iii); its CONTENT is the subject of `R2A-2`, `R2A-3` and `R2A-4`, but the slot itself is filled as prescribed.

`EX-4` CLOSED as prescribed. "in all three cases" is gone from line 189 and `grep -rn "in all three cases" docs/` now returns only the round 1 review files. The residue is in the same sentence's SECOND clause; see `R2A-5`.

`EX-5` CLOSED, pure deletion, as prescribed. Line 286 now reads "(`Q-55-endproperty`, which is what makes the predicate reach a divergent `--source`/`--plan` pairing)". `grep -rn "the step's end property" docs/` returns no sidecar line. Line 315's conditional "the one that decides whether this increment closes the end property" is untouched and still stands, which is what the triage ruled.

`FI-1` CLOSED at all five measured sites. Site 1 (line 185): "and the explicit case is precisely what survives it" deleted. Site 2 (line 293): "Its scope is exactly the case that survives anchoring, an explicit `--metrics` naming a foreign log, and" deleted. Site 3 (line 192): rewritten under `Q-55-resumepairing`; the falsified "The DEFAULT ledger case is already closed by the anchoring in inc1 ... what this rule adds is the residual" sentence is gone. Site 4 (line 334): "the case that survives inc1" is now "a case that survives inc1". Site 5 (`agent-scaffold.plan.toml:1710`): ", the case that survives anchoring being an explicit `--metrics` naming a foreign log," deleted, verified in `git diff a9dda1c 3354a90 -- docs/plans/agent-scaffold.plan.toml`. `grep -rn "survives anchoring\|precisely what survives\|exactly the case that survives" docs/` now returns only the round 1 review files and two frozen ledger records at `:587` and `:591`. I am NOT raising the two ledger lines: the triage ran `grep -rin "surviv"` over a population that explicitly included `agent-scaffold.ledger.md`, measured the site set at five, and separately ruled that decision-time ledger records are corrected by appending rather than rewriting. That is a settled round 1 measurement, not fix-induced residue.

`R2A-4` is the residue of `FI-1`'s site 3 fix rather than a failure of it: the site is closed and the new rule is wider than its own enumeration says.

## `R2A-1` (medium). The narrowing at line 309 is falsified by check 14c's third run, authored in the same pass

Line 309, as the fix pass left it: "`Q-55-endproperty` SHARPENS THAT RATHER THAN ADDING A CLASS: the two resolutions can now start from DIFFERENT FILES, the log from the `--source`-first anchor and the root from the plan the check reads, so rooting the guard on the anchor is a defect that check 11 passes over and only check 13b and 14g's fourth run catch."

The same pass added a third check that catches exactly that. Line 335, check 14c: "A THIRD RUN PINS `status --resume` ON THE DEFAULT LEDGER (`Q-55-resumepairing`), with no `--ledger-fragment` at all: check 13b's divergent pairing, A carrying a `## RESUME STATE` block, must give the same note, no line of A's block, and exit 0, where before inc2 it prints A's block verbatim AND AN INC2 THAT LEFT THIS SURFACE ANCHOR-ROOTED STILL WOULD."

That final clause is a statement that the run discriminates an anchor-rooted inc2 from the decided one, which is precisely what "catch" means in line 309's sentence. Three checks now catch an anchor-rooted guard; line 309 names two and says "only". The triage measured this exact sentence as "the sentence a later reviewer would rely on to conclude the check set covers the rooting", and the standing remedy for an affirmative exhaustiveness claim on this project is deletion rather than a second narrowing, because a narrowed claim is falsified again by the next check anyone adds. This one was falsified inside the same commit.

MINIMAL FIX, AND IT IS A DELETION. At line 309, delete "and only check 13b and 14g's fourth run catch", leaving "... so rooting the guard on the anchor is a defect that check 11 passes over." Nine words deleted, 0 added. Nothing is lost: each check already states its own discriminating role in its own text (13b, "this check, and not check 11, is what separates the two rootings ON THE VALIDATOR"; 14g, "THIS RUN, NOT CHECK 14b, is what separates an anchor-rooted projection from a checked-plan-rooted one"; 14c, the clause quoted above), so the risk paragraph does not need to re-enumerate them and cannot do it without going stale.

SITE COUNT: 1 authored (`workflow-enforcement-tier.md:309`), 1 mechanical (`agent-scaffold.md:1704`). `grep -rn "only check 13b"` over `docs/` returns those two lines plus the round 1 review files.

## `R2A-2` (high). Accepted cost (iii) and check 19b omit the precondition that makes the cost occur, so as written the layout is NOT refused and check 19b does not discriminate

THE TEXT. Line 269: "(iii) A SAME-PROJECT `--plan` OUTSIDE ANY `docs/plans`, WITH A `--source` INSIDE ONE, IS REFUSED (`Q-55-conventionlesscost`, human, 2026-08-02). `--source <root>/docs/plans/x.plan.toml --plan <root>/notes/p.md` greens today against `<root>/docs/metrics/workflow.jsonl`; after inc2 the checked plan's root is `<root>/notes`, through `src/main.rs:project_root_of_source`'s fallback to the checked plan's own directory, and the log is not under it, so `--workflow` exits non-zero and `status` and `next` omit their metrics half." Line 346, check 19b: "`agent-scaffold validate --source <root>/docs/plans/x.plan.toml --plan <root>/notes/p.md --workflow` prints `workflow invariants hold` at exit 0 before inc2 and exits NON-ZERO after it, and `status` and `next` on the same pair omit their metrics half at exit 0."

NEITHER SAYS THE `--source` MUST NOT BE TOML-PRIMARY, and the checked plan is the `--source` whenever it is. The shipped skeleton declares `primary = "toml"` (`pack/plan-template.plan.toml:15`) and `scaffold` emits it verbatim (measured on the fixture: `base/docs/plans/TEMPLATE.plan.toml:15`), so a file spelled `x.plan.toml` built the obvious way is TOML-primary and the cost does not occur. Measured, the two fixtures differing in that one field and nothing else:

```
$ "$BIN" validate --source "$SC/fixT/docs/plans/x.plan.toml" --plan "$SC/fixT/notes/p.md" --workflow      # primary = "toml"
.../fixT/docs/metrics/workflow.jsonl: 253 records, valid
.../fixT/docs/plans/x.plan.toml: 1 steps, 0 questions, valid
.../fixT/notes/p.md: generated projection of a TOML-primary source; skipping the Markdown plan validator
.../fixT/docs/plans/x.plan.toml vs .../fixT/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0

$ "$BIN" validate --source "$SC/fixM/docs/plans/x.plan.toml" --plan "$SC/fixM/notes/p.md" --workflow      # primary = "markdown"
.../fixM/docs/metrics/workflow.jsonl: 253 records, valid
.../fixM/docs/plans/x.plan.toml: 1 steps, 0 questions, valid
.../fixM/notes/p.md: 1 steps, 0 open-questions items, valid
.../fixM/notes/p.md vs .../fixM/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The first slot of the `vs` line names THE PLAN THE CHECK READS, which is what line 167 of this same file says it names ("the first slot names THE PLAN THE CHECK READS"). In the TOML-primary spelling that is `<root>/docs/plans/x.plan.toml`, not `<root>/notes/p.md`. Its root through `src/main.rs:project_root_of_source` is `<root>`, because the walk finds the `docs/plans` ancestor and returns "the ancestor's grandparent" rather than reaching the `parent.to_path_buf()` fallback. The log is `<root>/docs/metrics/workflow.jsonl`, which IS under `<root>`, so the containment predicate as specified does not fire and check 19b's "exits NON-ZERO after it" is false. This is not a derivation about a hypothetical implementation: it is what line 169 of the same file asserts, "In TOML-primary mode the checked plan IS the anchor and the rule is unchanged there".

The projections half is false in the same spelling, and I measured that too rather than deriving it, because `run_status` and `run_next` make the same selection (`src/main.rs:run_status`, "if let Some(source) = toml_source(&args.source)?", and the identical binding in `src/main.rs:run_next`):

```
$ "$BIN" status --source "$SC/fixT/docs/plans/x.plan.toml" --plan "$SC/fixT/notes/p.md"
plan: 1 steps (1 not started); 0 open-questions items
metrics: 253 records
exit: 0

$ "$BIN" next --source "$SC/fixT/docs/plans/x.plan.toml" --plan "$SC/fixT/notes/p.md"
task: x
source: /tmp/claude-1000/r2a-scratch/fixT/docs/plans/x.plan.toml
metrics: 253 records
...
```

Both project from the TOML source, root `<root>`, log under it, so after inc2 they keep printing the count and cost (iii)'s "omit their metrics half" does not happen either.

WHY THIS IS HIGH RATHER THAN A TERSENESS. Check 19b is an executable instruction with a stated exit code, pinning an ACCEPTED COST that the same section says "an implementer must NOT fix" and "a reviewer must NOT raise". An implementer who builds it the obvious way gets exit 0 after inc2, concludes the guard is broken, and the available "fixes" are to root on the `--plan` even when the source is TOML-primary, or to require the two named paths to share a root: the first breaks check 13b's THIRD run (the no-regression side) and the second breaks the region claim at line 169. So a wrong pinning check here is a route to a real behavioural defect, on the one class of item the file says must not be touched. It is also the written record of what the human accepted under `Q-55-conventionlesscost`; the DECISION is unaffected (the case is real, and the triager measured it on a Markdown-primary fixture), but the record of it names a layout that is not refused.

MINIMAL FIX, AND IT IS A NARROWING. Two words at each of the two sites: at line 269, "`--source <root>/docs/plans/x.plan.toml`" becomes "a MARKDOWN-primary `--source <root>/docs/plans/x.plan.toml`"; at line 346, the same in check 19b's command sentence. Nothing bigger is needed and nothing smaller works: without the qualifier the run does not exhibit the cost, and with it the whole rest of both passages is correct as written. Check 13b already uses exactly this phrasing for the same requirement ("give fixture A a clean MARKDOWN-primary `<task>.plan.toml`"), so this is the file's own established wording rather than new vocabulary.

SITE COUNT: 2 authored (`workflow-enforcement-tier.md:269` and `:346`), 2 mechanical in `agent-scaffold.md`. `grep -rn "notes/p.md" docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.plan.toml` returns only those two sidecar lines.

## `R2A-3` (medium). The "ALREADY refused in its no-`--source` spelling" bound is false as written, and check 19b's version of it does not discriminate

THE TEXT. Line 269, closing cost (iii): "THE BOUND, measured: the same layout is ALREADY refused in its no-`--source` spelling, so this removes a rescue rather than introducing a species." Line 346, closing check 19b: "The same layout in its no-`--source` spelling is refused too, which is what makes this a removed rescue rather than a new species."

Measured, both no-`--source` spellings of that layout, today:

```
$ "$BIN" validate --plan "$SC/fixT/notes/p.md" --workflow
no metrics log at .../fixT/notes/docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
.../fixT/notes/p.md: 1 steps, 0 open-questions items, valid
exit: 0

$ "$BIN" validate --plan "$SC/fixT/notes/p.md" --metrics "$SC/fixT/docs/metrics/workflow.jsonl" --workflow
.../fixT/docs/metrics/workflow.jsonl: 253 records, valid
.../fixT/notes/p.md: 1 steps, 0 open-questions items, valid
.../fixT/notes/p.md vs .../fixT/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

Neither is refused. Plainly spelled, the run never reads the project's real log at all (the anchored default lands at `<root>/notes/docs/metrics/workflow.jsonl`, since `src/main.rs:resolve_metrics_path` joins `METRICS_RELATIVE` onto the root of "source.as_ref().or(plan.as_ref())" and that root is the plan's own directory), which is accepted cost (i)'s silent-miss shape and not a refusal. With an explicit `--metrics` it greens. What the triage actually measured is narrower and has two qualifiers the fix pass dropped: WITH AN EXPLICIT `--metrics`, an inc2 rooted EITHER way refuses it, and today it is green. The word "ALREADY" and the present-tense "is refused too" both assert something about today's binary that is measurably false.

The check 19b version is worse than the cost (iii) version, because it is an acceptance check: as written its no-`--source` run gives exit 0 before inc2 and exit 0 after it, so it settles nothing, which is exactly what line 315 says `Q-66` forbids of a red case and exactly the defect `EX-2` was raised about.

MINIMAL FIX. At line 346, DELETE the final sentence "The same layout in its no-`--source` spelling is refused too, which is what makes this a removed rescue rather than a new species." Twenty-two words deleted, 0 added; it is rationale rather than an assertion the check can discriminate, and cost (iii) is where the bound belongs. At line 269, narrow rather than delete, because the bound is what the human was given: "the same layout is ALREADY refused" becomes "an anchor-rooted inc2 refuses the same layout, with an explicit `--metrics`," which keeps the point (this removes a rescue rather than introducing a species) and drops the false present tense. About 9 words changed, 0 net added.

SITE COUNT: 2 authored (`workflow-enforcement-tier.md:269` and `:346`), 2 mechanical.

## `R2A-4` (medium). The new `status --resume` rule reaches a third case, on a single-project layout that works today, which its own "TWO CASES" enumeration excludes and which cost (iii) does not record

THE TEXT, line 192, authored by this pass: "`status --resume`. Print a note naming the ledger path that was rejected and why, and print NO part of the block. EXIT 0. THIS SURFACE READS NO PLAN, so the rule SUPPLIES a root rather than being re-implemented per surface (`Q-55-resumepairing`, human, 2026-08-02): A `--source` AND A `--plan` BOTH NAMED MUST RESOLVE TO THE SAME ROOT OR THE BLOCK IS OMITTED, and with one alone the anchor is the root, as today. TWO CASES REACH IT: an explicit `--ledger-fragment` outside that root, and the DEFAULT ledger under a divergent pairing, which the anchoring in inc1 closes only on a single project."

A third case reaches it, and it is neither of the two named. "A divergent `--source`/`--plan` pairing" is defined by this file at line 169 as "a Markdown-primary `--source` in one project paired with a `--plan` in another". Accepted cost (iii)'s layout is ONE project and its `--source` can be TOML-primary, yet its two named paths resolve to DIFFERENT roots (`<root>` through the `docs/plans` walk, `<root>/notes` through the fallback, both in `src/main.rs:project_root_of_source`), so the same-root rule omits the block. Today it prints it:

```
$ "$BIN" status --resume --source "$SC/fixT/docs/plans/x.plan.toml" --plan "$SC/fixT/notes/p.md"
## RESUME STATE

FIXTURE-fixT-SECRET-RESUME-LINE: internal state of fixT
exit: 0

$ "$BIN" status --resume --source "$SC/fixT/docs/plans/x.plan.toml"          # control, one path alone
## RESUME STATE

FIXTURE-fixT-SECRET-RESUME-LINE: internal state of fixT
exit: 0
```

The ledger read is `<root>/docs/plans/x.ledger.md`, the project's OWN ledger, beside its own plan source (`src/main.rs:default_ledger_path`, "source.as_ref().or(plan.as_ref())" then the sibling join). There is nothing foreign about it. So the new rule withholds a project's own resume block from a run of its own that works today, which is a new refusal on a legitimate layout of exactly the species this step's own convention governs: line 263, "Each was measured, each was put to the human, and each was ACCEPTED". This one was not put, and it is not in cost (iii)'s list of what the layout loses ("so `--workflow` exits non-zero and `status` and `next` omit their metrics half", which is silent on the resume block) nor in check 19b's ("and `status` and `next` on the same pair omit their metrics half at exit 0").

It is also the ONE consequence of cost (iii)'s layout that survives `R2A-2`: the metrics half needs a Markdown-primary `--source`, while the same-root rule fires whatever the `--source` declares, since it never consults `primary` at all.

I am rating this medium rather than high because the affected surface is a best-effort projection at exit 0 and the failure direction is over-refusal (withholding the project's own block) rather than a false assertion. A triager who judges that line 263's convention binds a new refusal to be PUT rather than assumed has the `EX-3` precedent for re-severitising it, which is the same argument that took `EX-3` from medium to high.

MINIMAL FIX, two edits, both shrinking a claim rather than adding one. At line 192, DELETE the count: "TWO CASES REACH IT:" becomes "IT IS REACHED BY", leaving the two named cases standing as the examples they are (a deleted count cannot go stale when a third case is found, which is what just happened). Three words changed, 0 added. At line 269, extend cost (iii)'s existing consequence list by naming the surface: "so `--workflow` exits non-zero and `status` and `next` omit their metrics half" becomes "... and, once `Q-55-resumepairing`'s same-root rule applies, `status --resume` omits the block". About 12 words, which is the smallest true statement of an accepted cost that the file says must not be raised as a defect later.

SITE COUNT: 2 authored (`workflow-enforcement-tier.md:192` and `:269`), 2 mechanical. `grep -rn "TWO CASES REACH IT"` over `docs/plans/agent-scaffold.steps/`, `agent-scaffold.plan.toml` and the projection returns the sidecar line and its projection only.

## `R2A-5` (medium). Line 189's second clause still carries the universality `EX-4` deleted from its first, and the pass's own deletion at line 173 removed what it points at

THE PAIR. Line 189 after the `EX-4` fix: "The trigger is the SAME containment predicate the validator's refusal uses ... The predicate is never re-implemented per surface (One source of truth), AND NEITHER IS THE SELECTION OF THE PLAN TO ROOT ON: IT IS THE SELECTION EACH SURFACE ALREADY MAKES, ENUMERATED AT THE END OF THE MECHANISM SECTION."

Follow the pointer for the surface `EX-4` carved out. Line 173, the end of the mechanism section, as the fix pass left it: "`status --resume` is the one surface that reads NO plan (`src/main.rs:run_resume` derives `<task>` from the source-or-plan filename and reads only the ledger), so it has no checked plan to root on."

Before the fix pass that sentence continued "and its root falls back to the source-then-plan anchor `default_ledger_path` already uses, which leaves that surface's rule exactly as it was before this decision" (`git diff a9dda1c 3354a90`, the hunk at line 33 of the diff). Deleting it was correct, since `Q-55-resumepairing` falsified it. The consequence is that the enumeration line 189 points at now gives NO root for one of the three bullets it quantifies over, and the rule for that surface exists in exactly one place, the per-surface bullet at line 192, which is the thing line 189 says does not happen. The bullet's own hedge ("so the rule SUPPLIES a root rather than being re-implemented per surface") asserts the conclusion rather than establishing it: the same-root rule is stated nowhere else in the file.

This is the same shape as `EX-4` itself, one clause narrowed and its twin left standing, in the same sentence, and it is fix-induced twice over (the pass edited line 189 and deleted the referent at line 173 in the same commit). Medium rather than low because the practical effect is misrouting: an implementer looking up `status --resume`'s root where line 189 sends them reads "it has no checked plan to root on" and concludes the predicate does not fire there, which is the OPPOSITE of what `Q-55-resumepairing` decided.

MINIMAL FIX, A FOUR-WORD NARROWING. At line 189, "it is the selection each surface already makes" becomes "it is the selection each surface THAT READS A PLAN already makes". The pointer then resolves correctly for `status` and `next` and stops claiming to cover the surface line 173 says has no such selection. The deletion-shaped alternative, dropping the whole clause, also works and loses the routing for the two surfaces that do have it, so the narrowing is the smaller change here.

SITE COUNT: 1 authored (`workflow-enforcement-tier.md:189`), 1 mechanical (`agent-scaffold.md:1584`). `grep -rn "enumerated at the end of the mechanism section" docs/` returns those two lines.

## `R2A-6` (low). Check 14g's new metrics half names three invocations and then asserts "both exit 0"

Line 339, the text this pass authored: "THE SAME PAIRING PINS THE METRICS HALF ... `next` prints none of the `ACTIVE LOOP` block and no record count, `next --json` gives `"metrics_absent_reason": "log-not-this-project"` with `"no_active_loop_reason": "metrics-not-this-project"`, `status --json` on the same pairing gives the same `metrics_absent_reason`, AND BOTH EXIT 0."

Three invocations are named (`next`, `next --json`, `status --json`); "both" names two. The antecedent is unrecoverable from the sentence: it reads equally as the two commands and as the last two runs. This matters slightly more than a wording nit in this file specifically, because check 14b makes a point of asserting exit codes explicitly rather than leaving them implied ("The exit code is asserted explicitly, not left implied: a later reviewer who knows inc2 introduced a refusal could otherwise read exit 0 here as a bug").

MINIMAL FIX, A DELETION. Delete "and both exit 0" (four words, 0 added). Check 14 already pins the whole class ("`status` and `next` NEVER exit non-zero under any of these inputs"), so nothing is lost. If the count is wanted instead of the deletion, "all three exit 0" is one word.

SITE COUNT: 1 authored, 1 mechanical.

## `R2A-7` (low). "Two further human decisions all landed after it" is now five

Line 24: "THIS FILE IS THE SECOND PLANNER PASS. The first pass scoped two defects and two increments, correctly for what was then known. Two human scope additions (defects C and D), the design pass, and TWO FURTHER HUMAN DECISIONS all landed after it."

Counted from this file's own provenance list at lines 12 to 20 and confirmed against the receipts. Decisions that landed after the first planner pass: `Q-55-mechanism` and `Q-55-noconvention` (which the same sentence names separately as "the design pass"), then `Q-55-refusalscope`, `Q-55-jsonreason`, `Q-55-endproperty`, `Q-55-conventionlesscost` and `Q-55-resumepairing`. That is FIVE "further" decisions, not two. The receipts are real and dated:

```
$ grep '"q_id":"Q-55' docs/metrics/workflow.jsonl | grep -o '"q_id":"[^"]*"\|"ts":"[^"]*"'
... Q-55 2026-07-31 / Q-55-scope 2026-07-31 / Q-55-mechanism 2026-07-31 / Q-55-noconvention 2026-07-31 /
    Q-55-refusalscope 2026-07-31 / Q-55-jsonreason 2026-07-31 / Q-55-endproperty 2026-08-02 /
    Q-55-conventionlesscost 2026-08-02 / Q-55-resumepairing 2026-08-02
```

The amendment took the count from two to three and the fix pass took it to five, while editing the receipt count and the receipt list four lines below it. Low, because nothing executable depends on it, and it is a framing paragraph rather than a specification.

MINIMAL FIX, A DELETION. "and two further human decisions" becomes "and further human decisions" (one word deleted, 0 added), which is the form that cannot go stale on the next decision. Replacing "two" with "five" is the same size and will be wrong again the next time the human decides something on this step.

SITE COUNT: 1 authored (`workflow-enforcement-tier.md:24`), 1 mechanical (`agent-scaffold.md:1419`). `grep -rn "further human decisions" docs/` returns exactly those two lines.

## Counts and enumerations I re-counted myself

| claim | where | my count | verdict |
| --- | --- | --- | --- |
| "NINE decision receipts" | line 10 | `grep -o '"q_id":"Q-55[^"]*"' docs/metrics/workflow.jsonl \| sort -u \| wc -l` returns 9, all carrying `"task":"workflow-enforcement-tier"`, last three dated 2026-08-02 | correct |
| the provenance bullet list | lines 12 to 20 | 9 bullets, one per receipt, ids matching exactly | correct |
| "Three things the implementer should carry" | line 169 | 3 (one predicate not two; the TOML-primary region; the typo'd `--source`) | correct |
| "The three accepted costs" / "PINS all three" | lines 261, 263 | 3 costs (i) (ii) (iii); 3 pinning checks 18, 19, 19b | correct |
| "TWO CASES REACH IT" | line 192 | at least 3 | FALSE, `R2A-4` |
| "and both exit 0" | line 339 | 3 invocations named | FALSE, `R2A-6` |
| "only check 13b and 14g's fourth run catch" | line 309 | 3 checks catch it | FALSE, `R2A-1` |
| "two further human decisions" | line 24 | 5 | FALSE, `R2A-7` |
| "for inc2 there are FOUR" red cases | line 315 | 4 (checks 11, 13b, 14b, 14e) | correct, and undisturbed: the `EX-1` fix landed on an existing run of an existing check, which is the ruling the triage already made |
| "The three increments" | line 279 | 3 bullets | correct |
| "Four defects, one family" | line 3 | 4 bullets (A, B, C, D) | correct |
| "THE FOUR DOC COMMENTS `Q-55-jsonreason` FALSIFIES" | line 367 | 4 named, matching the 4 bullets at 209 to 212, with the fifth at 214 correctly excluded as pre-existing | correct |
| "WHY THE FOUR CALL SITES ARE ONE INCREMENT" | line 289 | 4 (`validate`, `status`, `next`, the ledger path) | correct |
| "Three runs, three distinct outputs" (14f) | line 338 | 3 ((a), (b), (c)) | correct |
| "Two of the three causes are already distinguished IN THE CODE" | line 235 | 3 causes listed at 237 to 239 | correct |
| "1514 lines: 521, 483, 510" | line 22 | 521 + 483 + 510 = 1514 | correct |
| "The two accepted costs" reference in Documentation impact | line 364 | now says "The three accepted costs" and the deliberate break is still the last paragraph of that section | correct |

## What I checked and did NOT raise

- The two frozen ledger records at `agent-scaffold.ledger.md:587` and `:591` still carry `FI-1`'s falsified "the case that survives anchoring" phrasing. The triage searched a population that included the ledger, measured five sites, and separately ruled that decision-time ledger prose is corrected by appending rather than rewriting. Not fix-induced, and re-litigating a measured site set is not this lens's job.
- `agent-scaffold.plan.toml` records six of the nine `Q-55` decisions in its step prose and not `Q-55-endproperty`, `Q-55-conventionlesscost` or `Q-55-resumepairing`. The round 1 fidelity reviewer checked `agent-scaffold.plan.toml` for exactly this and reported "No other sidecar or `docs/plans/agent-scaffold.plan.toml` reference to this step's receipt count, red-case count, or increment scope needed updating", and the ledger at `:411` records that no new `[[question]]` is owed for a sub-decision under an already-decided question. Settled at round 1.
- The consequential deletions the two decisions forced all check out and I confirmed each removed rather than relocated its claim: "THAT IT DOES NOT HAVE ON `status --resume`" (line 193), "on `next` ALONE" (line 239), "for an explicit `--ledger-fragment` outside the root" (line 286), and "which leaves that surface's rule exactly as it was before this decision" (line 173, whose residue is `R2A-5` rather than the deletion being wrong). Each was falsified by `Q-55-resumepairing` and each deletion is the right response.
- "A THIRD BEHAVIOUR CHANGE IS NOT A COST" was renamed to "A FURTHER BEHAVIOUR CHANGE", which removes the collision with the new cost (iii) rather than creating one. Correct.
- Check 14c's third run and check 14g's fourth run both build on "check 13b's divergent pairing" and both state the step status they need (14g's metrics half explicitly moves B to `in-progress`, which is the triage's `EX-1` correction applied). No fixture contradiction between them.
- `status --resume` having no JSON surface is stated consistently at lines 245, 335 and 339 and none of the new text contradicts it.
- Line 315's "the one that decides whether this increment closes the end property" is scoped to `validate --workflow`, which is what the end property at line 114 is about, so `EX-5`'s deletion did not falsify it.

## Scratch hygiene

Every probe ran with `TMPDIR=/tmp/claude-1000/r2a-scratch` and every fixture (`base`, `fixT`, `fixM`, `A13`, `B13`, `B13p`) was created under it. The directory was removed when this review finished. DIRECTORIES LEFT IN `/tmp`: 0.
