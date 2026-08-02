# `workflow-enforcement-tier` endproperty fold, round 3: the cold-reader lens

I was handed `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` and told to read it as someone who has never seen this review loop and has to act on it: build increment 2, or decide whether to approve it. I read the whole file front to back before looking at any diff, any prior finding, or any ledger entry, formed my own view, then verified every factual claim I could resolve against the source and by running the binary. Only afterwards did I read the six prior review files, to separate what is new from what is already known.

Increment 1 has landed; increments 2 and 3 have not. Everything below was run against a build of the reviewed commit.

TWO FINDINGS, one medium and one low. Neither is a defect in the mechanism the file specifies. The mechanism itself I could build from this file, and every present-state claim I tested about how the tool behaves today is TRUE.

## What I ran, and the environment

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep3-cold`, branch `review/q55-ep3-cold`, built with `cargo build` through `direnv`. `TMPDIR` pointed at a scratch directory outside any git repository, per the file's own instruction at the head of the acceptance check.

Reproduced from the file, all matching what it says:

- The fixture. `scaffold --output-dir "$SCRATCH" --write --force --principles default` prints `Wrote to ... (30 changed, 0 left untouched)` and `ls "$SCRATCH/docs"` prints only `plans`.
- Defect A. From inside the fixture, `validate --source docs/plans/TEMPLATE.plan.toml --workflow` gives exit 0, the ok summary on stdout, and BOTH stderr notes, in the exact words quoted at `:48` to `:51`.
- Check 3. Defect B's original reproduction is dead after inc1: the run from the agent-scaffold root names the FIXTURE's own missing log and exits 0.
- Check 11's pre-change observation. With an explicit relative `--metrics docs/metrics/workflow.jsonl` and the borrowed slug at `complete`, `workflow invariants hold` at exit 0.
- Check 14b's pre-change observation. Same fixture at `in-progress`: `metrics: 255 records`, `state: converged`, `streak: 1/1`, `rounds: 2/5`, `next: mark the step complete, re-render, and commit`, exit 0.
- Check 13b, the case `Q-55-endproperty` exists for. Fixture A markdown-primary with agent-scaffold's own log, fixture B's Markdown Roadmap carrying `triager-runs-only-on-findings` at `complete`: `B/docs/plans/p.md vs A/docs/metrics/workflow.jsonl: workflow invariants hold`, exit 0. The SECOND run, with the `--source` path misspelled so it does not exist, greens identically. This is the file's central new claim and it is correct.
- Check 14g's fourth run, both halves. On the same pairing, `next` echoes A's `## RESUME STATE` block verbatim under B's plan; with B's step at `in-progress` it also emits the full converged instruction from A's log, and `status --json` reports A's 255 records. Exit 0 throughout.
- Accepted cost (iii). `--source <root>/docs/plans/x.plan.toml --plan <root>/notes/p.md --workflow` prints `workflow invariants hold` at exit 0 today against `<root>/docs/metrics/workflow.jsonl`.
- Accepted cost (iv). `status --resume` on that same pair prints the project's own block at exit 0, in BOTH `primary` spellings, and prints it with `--source` alone too.
- The residual at `:277`. Copying agent-scaffold's log into a fixture's own `docs/metrics/` still yields `workflow invariants hold` at exit 0 for a project with no review evidence, and the containment guard cannot fire because the log is under the fixture's root.
- Check 9's property today. From the repository root, `validate --source docs/plans/agent-scaffold.plan.toml --workflow` prints three RELATIVE stdout lines and exits 0.

Checked against the source, by symbol:

- `src/main.rs:project_root_of_source` is the lexical nearest-wins walk the file describes, with `parent.to_path_buf()` as the conventionless fallback. `src/main.rs:resolve_metrics_path` returns an explicit `--metrics` verbatim and otherwise anchors `source.as_ref().or(plan.as_ref())`. `src/main.rs:default_ledger_path` joins `<task>.ledger.md` onto the anchor's parent. `src/next.rs:derive_task` uses the same source-then-plan order.
- `src/main.rs:run_validate`'s `--workflow` block is the four-arm match over `(toml_primary, &plan_contents, &metrics_contents)` described at `:56`, with the `(None, None, _)` arm's comment reading exactly as quoted at `:62` and the `_` catch-all being the `eprintln!` skip. Summaries go to stdout, problems to stderr, exit 1, so the stream labels at `:48` to `:51`, `:78` to `:81` and `:106` are right.
- `src/main.rs:run_status` opens with `if args.resume { return run_resume(&args); }`, before any serialisation, so `:245` and check 14g's closing sentence are right that `status --resume` has no JSON surface.
- `src/main.rs:run_next`'s `else` arm of `metrics_path.exists()` yields `(Vec::new(), None)`, so `NextProjection.metrics` is `None` on an absent log and `src/next.rs:render_human` prints `metrics: no log found`. The "unsafe is not absent" trap at `:197` and the `Some` exactly when `None` rule at `:224` are both consistent with the code.
- `src/next.rs:no_loop_reason`'s three strings are `no plan steps found`, `all steps complete` and `no in-progress or ready step`, and it is only ever called when `select_active_loop` returned `None`, which its own last branch shows happens only with no steps or all-terminal steps. So `:229`'s unreachability claim and `:214`'s `active_loop` doc-comment correction are both right.
- All four doc comments the file says `Q-55-jsonreason` falsifies or leaves incomplete are quoted accurately (`src/next.rs:NextProjection::no_active_loop_reason`, `src/next.rs:NextProjection`, `src/main.rs:Projection`, `src/next.rs:NextProjection::resume_state`), as is the pre-existing fifth at `src/next.rs:NextProjection::active_loop`.
- `#[serde(skip)]` appears exactly once in `src/`, at `src/next.rs:NextProjection::no_active_loop_reason`; no `skip_serializing_if` appears in `src/next.rs` or `src/main.rs` (all thirteen occurrences are in `src/plan/source.rs`). The negative result at `:216` holds.
- `src/plan/source.rs`: `[meta].primary` absent defaults to markdown (the doc at `:71` and the test `primary_defaults_to_markdown_when_absent`), so check 13b's parenthetical is right; `Meta` carries `#[serde(deny_unknown_fields)]` at `:102`; `is_safe_sidecar_ref` is at `:480-495`. `src/plan/render.rs` reads `meta.sidecars` at `:167-169` and `meta.title` at `:296` and NOTHING else, so "the render cost is zero" is exactly stated.
- `src/workflow.rs:180-195` is `check_workflow_toml`; `:448-449` is the bare-slug join. `tests/validate_workflow_toml_source_needs_no_plan.rs` is 132 lines, its module doc is `:1-13`, the pinning test starts at `:89`, and the soft-skip comment is `:96-98`.
- Every cited test name exists: `checks::tests::a_non_repo_target_with_runnable_checks_errors`, `tests::init_plan_defaults_to_git_and_skips_inside_a_repo`, `tests::install_precommit_hook_skips_a_non_repo`, `GOLDEN_JSON`, `golden_json`, `GOLDEN_HUMAN`, `golden_human_text`, and the two drift guards `the_committed_scaffold_matches_a_fresh_render` and `the_committed_role_prompts_match_a_fresh_render` with `normalize_wrapping` in the same module.
- `README.md:210`, `:212-224`, `:228` and `:230-239` all resolve to what the Documentation impact list says they are. `pack/AGENTS.md:93` carries the backstop sentence verbatim, `:61` and `:63` carry the two "When instrumentation is on" clauses, `:116` is `{{instrument}}`. `justfile:46-48` is `scaffold-self` followed by `nix fmt`. `CHANGELOG.md`'s `[Unreleased]` has `Added` and `Changed` and no `Fixed`.
- Backlog orders: `sidecar-ref-empty-string` 63, `sidecar-ref-symlink` 64 (`deferred`), `reviewer-reproducible-evidence` 88, `workflow-enforcement-tier` 94 with three `risky` increments, `test-tmpdir-repo-assumption` 95, `status-resume-ignores-json` 96. All correct.
- Exploration line counts: 521 + 483 + 510 = 1514, matching `:22` exactly.
- The step 92 calibration numbers at `:315`. `prompt-drift-guard` (order 92) has six `type:"round"` records for `prompt-drift-guard-inc1` with `valid_findings` 4, 3, 5, 1, 2, 0, so six rounds and fifteen findings, every severity `low`, and the escalation record's own words are "mechanism verified defect-free across 4 rounds". Exactly as stated.

## `R3B-1` (medium): the acceptance-check set never re-runs the no-regression correct case after inc2, the increment that introduces the canonical resolution and can collapse the split `:175` says must not be collapsed

THE SITE. Check 9 at `:331`: "AFTER INC1, NO REGRESSION ON THE CORRECT CASE, which is the Safe on existing projects check: from the agent-scaffold repository root, `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` exits 0 with `workflow invariants hold`, reading this repository's own log, and its three stdout lines are BYTE-IDENTICAL to the pre-fix binary's (A measured this; a relative source must keep a relative printed path)."

That is the only check in the file that asserts the ordinary invocation still produces the ordinary output, and it is scheduled for inc1 alone. Inc2's checks are 11, 12, 13, 13b, 14, 14b, 14c, 14d, 14e, 14f, 14g, 14h, 19 and 19b. I read them as a set and none of them re-runs it:

- 11, 12, 13's first half, 13b's first two runs, 19 and 19b assert REFUSALS.
- 13's second half and 13b's THIRD run are the only positive validator cases. 13b's third run is `--source` AND `--plan` together in TOML-primary mode and asserts only "exits 0 and reads that project's own log"; it says nothing about the printed spelling, and it is deliberately built in the mode where "the checked plan IS the anchor and the rule reduces to the anchor-rooted one".
- 14 asserts scoping (no `--workflow`, and the projections never exiting non-zero), not the correct case.
- 14h is the closest thing on the projections, and it is the MACHINE surface only: "every pre-existing field keeps its name, position and value", which does not reach a printed path because `NextProjection` carries no metrics path.

WHY THIS IS THE INCREMENT THAT NEEDS IT. `:175`: "THE LEXICAL/CANONICAL SPLIT IS DELIBERATE AND MUST NOT BE COLLAPSED. The DEFAULT is lexical so the derived path keeps the spelling the caller typed; the GUARD is canonical so it cannot be spoofed by a symlinked source. A built the canonicalising DEFAULT too and measured what it costs: every resolved path becomes absolute even when the user typed a relative source, so TWO OF THE THREE PRINTED LINES CHANGE ON THE NO-REGRESSION CASE and an absolute machine-specific path lands in output that a pre-commit hook or CI log may be matched against."

Inc1 contains no canonicalisation at all. Inc2 is where it arrives, and `:167` instructs the implementer to "resolve the metrics path by absolutising and canonicalising its longest existing ancestor and re-appending the components below it", which is a resolution OF the metrics path and is the natural thing to do in place. So the one increment able to collapse the split is the one whose check set omits the check that detects the collapse. `:313` classifies inc2 `risky` in part because it "INTRODUCES a non-zero exit on validator invocations that succeed today AND withholds output from projection invocations that answer today", which is exactly the population check 9 protects.

THE SUITE DOES NOT COVER IT EITHER, so check 1's `cargo test` does not stand in. `tests/metrics_and_ledger_anchor_to_the_plan_source.rs` passes every fixture path as an ABSOLUTE string (its helper is `fn arg(path: &Path) -> String { path.to_str().unwrap().to_string() }`, applied to `root.join(...)` results) and asserts which log was read by record count and by exit code, never a relative printed spelling. `GOLDEN_HUMAN` and `GOLDEN_JSON` (`src/next.rs:tests`) are unit tests over `next::project`'s already-resolved inputs, so they cannot observe resolution at all.

MEASURED TODAY, so the property being protected is real and currently holds:

```
$ cd <repo> && ./target/debug/agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 255 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

THE FILE ALREADY HAS THE IDIOM FOR THE FIX, which is why the omission reads as an oversight rather than a judgement. Check 16 at `:346`: "AFTER INC3, plain `validate` is STILL unaffected, which is the other half of the decision and the easiest thing to break by accident: RE-RUN CHECK 10 and expect the same exit 0 and the same stderr note." The document knows how to schedule an earlier check against a later increment, and does it for inc3 and not for inc2.

SEVERITY: MEDIUM. Not high, because the regression it fails to catch is an output-spelling change rather than a wrong verdict, and a wrong-file or wrong-exit regression on the correct case would still be caught by 13b's third run. Not low, because this is a check-SET gap on the one increment the file itself calls "a deliberate, knowing break of Safe on existing projects rather than a strict tightening", and because the specific property left unpinned is the one whose loss decided the lexical/canonical fork in the first place.

MINIMAL FIX, four words, no new check. At `:331`, "9. AFTER INC1, NO REGRESSION ON THE CORRECT CASE" becomes "9. AFTER INC1 AND AGAIN AFTER INC2, NO REGRESSION ON THE CORRECT CASE". This is the same shape as check 16's existing re-run instruction, adds no new fixture, and adds no prose that can go stale independently of the check it sits in.

SITE COUNT MEASURED: 1 authored (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:331`), 1 mechanical. `grep -rn "NO REGRESSION ON THE CORRECT CASE" docs/` returns the sidecar line and its projection in `docs/plans/agent-scaffold.md` and nothing else; `docs/plans/agent-scaffold.plan.toml` carries no occurrence.

## `R3B-2` (low): the PROVENANCE list is missing `Q-55-resumecost`, the receipt `:263` names as the authority for accepted cost (iv)

THE SITE. `:10` introduces the list: "PROVENANCE. `Q-55`, decided by the human on 2026-07-31, with decision receipts in `docs/metrics/workflow.jsonl` (the last taken on 2026-08-02, after inc1's work review), all carrying `task:"workflow-enforcement-tier"`:". Nine bullets follow at `:12` to `:20`, one per receipt, each with a one-line gloss of what it decided.

There are TEN receipts:

```
$ grep -o '"q_id":"Q-55[^"]*"' docs/metrics/workflow.jsonl | sort -u
"q_id":"Q-55"
"q_id":"Q-55-conventionlesscost"
"q_id":"Q-55-endproperty"
"q_id":"Q-55-jsonreason"
"q_id":"Q-55-mechanism"
"q_id":"Q-55-noconvention"
"q_id":"Q-55-refusalscope"
"q_id":"Q-55-resumecost"
"q_id":"Q-55-resumepairing"
"q_id":"Q-55-scope"
```

`Q-55-resumecost` is the one with no bullet. The receipt is real and is the newest record in the log:

```
$ grep -n 'Q-55-resumecost' docs/metrics/workflow.jsonl
255:{"type":"decision","task":"workflow-enforcement-tier","q_id":"Q-55-resumecost","options":["Accept as (iv), queue the shared cause","Accept as cost (iv), nothing queued","Carve out the conventionless case"],"recommendation":"Accept as (iv), queue the shared cause","chosen":"Accept as (iv), queue the shared cause","ts":"2026-08-02"}
```

The id appears exactly once in the whole document, at `:263`: "Each was measured, each was put to the human, and each was ACCEPTED, (i) and (ii) as part of `Q-55-noconvention`, (iii) as `Q-55-conventionlesscost` and (iv) as `Q-55-resumecost`."

WHY IT MATTERS TO A COLD READER, and it is not the bare bookkeeping it looks like. `:263`'s next sentences are the file's instruction to two audiences: "an implementer must NOT 'fix' them, and a reviewer must NOT raise them as defects". The authority for that instruction is "each was put to the human". The PROVENANCE list is the only place in the document that records WHAT was put and WHAT was chosen; for the other three costs it does, and for (iv) it records nothing. Cost (iv) is also the only one of the four with no inline attribution of its own: (i) and (ii) are attributed at `:263`, (iii) carries "(`Q-55-conventionlesscost`, human, 2026-08-02)" inline at `:269`, and (iv) at `:271` carries neither. A reader who wants to check that (iv) was genuinely decided rather than assumed has one bare id, no gloss, and a provenance list that appears to say no such decision exists. That is precisely the objection this cost was created to answer.

WHY IT HAPPENED, since it bears on whether this is a live gap or a stale one. The `Q-55-resumecost` receipt is written at log line 255, AFTER the round 2 round record at line 254, so the round 2 triage's recount of nine receipts was correct at the time it was made. The round 2 fix pass then added cost (iv) at `:271` and its `:263` citation, and separately a later commit deleted the now-stale count from `:10`, but neither added the bullet. So this is not a claim any prior round passed over; the tenth receipt did not exist when they counted.

SEVERITY: LOW. Nothing asserted is false, no behaviour depends on it, and the receipt is one grep away. It is an incompleteness in the file's own record of what was settled, on the one class of item the file says must not be reopened.

MINIMAL FIX, one bullet in the form of the other nine. After `:20`, add:

`- `q_id:"Q-55-resumecost"`, accept the `status --resume` omission as cost (iv) and queue the shared cause.`

I considered a deletion-shaped alternative and there is none that improves things: the list is a record rather than an argument, and deleting it or its framing would lose the provenance for all ten rather than complete it for one. The bullet is mechanical, mirrors the existing nine, and asserts nothing beyond what the receipt's `chosen` field already says.

SITE COUNT MEASURED: 1 authored (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:20`, the insertion point), 1 mechanical. `grep -rn 'q_id:"Q-55-resumepairing"' docs/` returns the sidecar line and its projection in `docs/plans/agent-scaffold.md`; `docs/plans/agent-scaffold.plan.toml` carries no provenance bullet list.

## The governing questions, answered

WOULD A COLD READER BE MISLED ANYWHERE ABOUT HOW `agent-scaffold` BEHAVES TODAY, OR ABOUT WHAT INCREMENT 2 WILL DO? No. I tested every present-state claim I could reduce to a command and all of them reproduced, including the two that decide the whole `Q-55-endproperty` amendment (the divergent `--source`/`--plan` pairing greening at exit 0, and its typo'd-`--source` variant doing the same). I found no sentence in the file that is false about the current binary and none that misdescribes what inc2 will do. `R3B-1` is a gap in what the checks establish, not a false statement; `R3B-2` is an omission from a record, not a false one.

DO THE FOUR ACCEPTED COSTS HONESTLY DESCRIBE WHAT A USER OF THOSE LAYOUTS WILL EXPERIENCE? Yes, including (iii) and (iv), which I built and ran rather than reasoned about.

For (iii), `--source <root>/docs/plans/x.plan.toml --plan <root>/notes/p.md --workflow` greens today at exit 0 against `<root>/docs/metrics/workflow.jsonl`, and after inc2 the checked plan is the Markdown `--plan`, whose root is `<root>/notes` through `src/main.rs:project_root_of_source`'s fallback, and the log is not under it. Refused. The stated cause is the actual cause and the markdown-primary precondition in the title is load-bearing and correct: with the source TOML-primary the checked plan IS the source and the rule reduces to the anchor-rooted one, so the metrics half is not refused.

For (iv), `status --resume` on the same pair prints the project's OWN block today, at exit 0, in BOTH `primary` spellings, which I ran separately rather than inferring:

```
$ status --resume --source <root>/docs/plans/x.plan.toml --plan <root>/notes/p.md    # primary = "markdown"
## RESUME STATE
FIXTURE-R-RESUME-LINE: this is project R own block.
exit: 0
$ ... same with primary = "toml"                                                      -> identical
$ ... same with --source alone, no --plan                                             -> identical
```

Under `:192`'s rule the two named paths resolve to `<root>` and `<root>/notes`, so the block is omitted whatever the source declares, which is exactly what "in EITHER `primary` spelling, so its population is WIDER than (iii)'s" says. Both costs describe genuine, currently-working invocations, both say plainly that the user loses something real, and (ii)'s text is candid that its quiet manifestation on the projections is "the more expensive half".

DO THE ACCEPTANCE CHECKS, READ AS A SET, ESTABLISH THAT INCREMENT 2 DID ITS JOB? Almost. The positive side, the negative side, both surfaces, the vocabulary's separating power, the precedence rule, the correlation rule and both accepted costs are all covered, and 14d and 14f are specifically built so that a check passing against a wrong implementation is not enough. The one thing the set does not establish is that inc2 left the CORRECT case alone in the way `:175` requires, which is `R3B-1`. Nothing else in inc2's owed content is unpinned by a runnable check except the documentation items (the four doc comments, the `--workflow` help enumeration, the README additions, the CHANGELOG entry), and those are not behavioural, are enumerated exhaustively in "Documentation impact", and are not the kind of thing the file's "a command with an expected exit code" convention can pin. I do not raise them.

IS THE STEP'S END PROPERTY, AS STATED, ACTUALLY MET BY THE INCREMENTS AS SPECIFIED? No, and this is already ruled on rather than new. `:114` says "`validate --workflow` must never pair a plan source with a metrics log belonging to a different project and report success". I reproduced the counterexample the file itself records at `:277`: copy agent-scaffold's log into a fixture's own `docs/metrics/` and the fixture's borrowed-slug `complete` step still gets `workflow invariants hold` at exit 0, and the containment guard structurally cannot fire because the log IS under the fixture's root. `:265`'s own words apply, "containment is not correctness". So the end property as literally worded is met by no increment of this step.

`EX-5` raised this in round 1 and the round 1 triage ruled the SITE, not the claim: it corrected the site from the end property to the affirmative "and so what makes this increment close the step's end property rather than half of it" at what was then `:282`, deleted that clause, and wrote explicitly "Do NOT take the finding's own prescription of adding a containment clause to the end property at line 112 ... A deleted claim cannot be falsified at an edge." I checked that the deletion landed: `:290`'s inc2 bullet now reads "(`Q-55-endproperty`, which is what makes the predicate reach a divergent `--source`/`--plan` pairing)" with nothing after it, and the only surviving mention is `:319`'s conditional "the one that decides whether this increment closes the end property", which the triage explicitly allowed to stand. The residual is recorded in two other places, the "What this step does not fix, and where it goes instead" section at `:277` and the scope bullet at `:388` that cross-references it. I therefore do NOT raise it, and record it here as the honest answer to the question rather than as a finding.

## Checked deliberately and NOT raised

- THE COPIED-LOG AND VENDORED-LOG GREENS versus the end property. Reproduced; ruled by the round 1 triage on `EX-5` as above. Not re-raised.
- `:189`'s "The predicate is never re-implemented per surface (One source of truth)" against `:192`'s same-root rule for `status --resume`. The round 2 residue reviewer wrote out this exact tension ("the bullet's own hedge ... asserts the conclusion rather than establishing it"), the round 2 triage saw it, and its prescribed fix deleted the sentence's SECOND clause while explicitly "leaving 'The predicate is never re-implemented per surface (One source of truth).'" That is a settled verdict on the surviving sentence and I have no new evidence that it was wrong.
- `status --resume`'s same-root rule on a `--source` that does not exist. I built it and it is live today: with A's plan source moved aside and `--plan` naming B, `status --resume --source A/docs/plans/p.plan.toml --plan B/docs/plans/p.md` prints A's `## RESUME STATE` at exit 0. My first reading was that `:171`'s own observation ("a path that does not exist yields no canonical root and a two-root comparison then has nothing to compare") leaves the rule undefined here. It does not: `:192`'s sentence is categorical, "must resolve to the SAME root OR THE BLOCK IS OMITTED", and a path with no canonical root does not resolve to the same root, so the block is omitted. That is determinate and it is the safe direction on a best-effort surface at exit 0. The round 2 triage relied on the same categorical reading to reject `R2B-2`'s indeterminacy claim, so it is a consistent reading of the file rather than a charitable one. No finding.
- A SAME-PROJECT PAIR OUTSIDE (iii)'s AND (iv)'s STATED SHAPES. A project with no `docs/plans` anywhere, source at `<root>/x.plan.toml` and Markdown plan at `<root>/plans/x.md`, greens today and loses both its `--workflow` green and its resume block after inc2, by the same `project_root_of_source` fallback, while matching neither (iii)'s "MARKDOWN-PRIMARY `--source` INSIDE" a `docs/plans` nor (iv)'s "ON THE SAME PAIR". I reproduced it. I do not raise it: `R2A-4` established this class in round 2, the human ruled it with `Q-55-resumecost`'s "Accept as (iv), QUEUE THE SHARED CAUSE", and `:281` records the shared cause precisely ("`src/main.rs:project_root_of_source`'s fallback to the plan's own parent") and states that the response is queueing "rather than accumulating a fresh accepted cost on every new surface". Widening (iii)'s and (iv)'s population sentences would be exactly the authored-prose fix class this project has measured as re-seeding, against a decision that already covers the class.
- THE FIVE EXCLUDED RESIDUALS. I confirmed each is still what it was and raised none: the `--metrics` relative-`default_value` text at `:112`, the `default_ledger_path` current-directory text at `:139`, the "Documentation impact INC1" sub-list at `:359` to `:363` and the two help-string descriptions (all falsified by increment 1, which I verified: `ValidateArgs::metrics`, `StatusArgs::metrics` and `NextArgs::metrics` are `Option<PathBuf>` with anchoring help text in the tree today, and `src/main.rs:default_ledger_path`'s doc comment now says "BESIDE the plan source"); the `--ledger-fragment` interaction with the `status --resume` rule; and the increment summary paragraphs naming three decisions rather than five. Accepted costs (i) and (ii), increments 1 and 3, and the five human decisions themselves are untouched here.
- NO LINE-LENGTH OR HARD-WRAPPING OBSERVATION was made, and none would have been.

## Scratch hygiene

All probes ran under a single scratch `TMPDIR` at `/tmp/claude-1000/r3b-coldread-scratch`, created for this review and removed at the end. Nothing was written to bare `/tmp`: its entry count is unchanged from before this review. DIRECTORIES LEFT IN `/tmp`: 0.
