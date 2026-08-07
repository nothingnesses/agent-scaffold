# `workflow-enforcement-tier-inc4`, round 1, completeness lens

Reviewer lens: what the currency pass MISSED, and whether the scope boundary held in both directions. Diff range `363ac06..079d63f`, reviewed at `079d63f` on branch `review/wet-inc4-r1-c`.

Six findings. Severity ceiling `medium`. Nothing at `high` or `critical`: I found no behavioural defect, no broken gate, and no citation whose re-pointing landed on the wrong subject. Every finding is a stale claim that survived the sweep, or one edit that crossed the closed scope boundary.

The headline is that the pass swept the sections it was briefed on and did NOT sweep three others. Two of the three defects below sit in sections the diff never touched at all (`The four accepted costs`, and the `Q-55-jsonreason` problem statement), and one is the THIRD site of the very twin pair `Q-55-twinsites` was raised about, in the same file as one of the two it fixed.

## R1C-1. The `#[serde(skip)]` negative result is false three ways, and inc2 is what falsified it (`medium`)

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:206` states, in unqualified present tense and with no historical framing at all:

> `#[serde(skip)]` appears exactly ONCE in the whole of `src/`, at `src/next.rs:NextProjection::no_active_loop_reason`, so there is no second silently-dropped field anywhere.

All three clauses are now false, and inc2 is what made them false.

```
$ grep -rn 'serde(skip' src/
src/next.rs:198:  #[serde(skip)]
src/next.rs:202:  #[serde(skip)]
```

`src/next.rs:198` is `metrics_absent_note` and `:202` is `resume_state_absent_note`, the two note fields the inc2 implementer added (ledger, judgement call (a), "solved with two `#[serde(skip)]` note fields carried through `NextInputs`"). `no_active_loop_reason` is at `src/next.rs:192` and carries NO `#[serde(skip)]`; the sidecar itself says so thirteen lines later at `:219` ("RETYPED ... and NO LONGER `#[serde(skip)]`"), so the file contradicts itself within one section.

The sentence is not merely stale, it is load-bearing in the wrong direction: "there is no second silently-dropped field anywhere" is a negative result a later implementer or reviewer would rely on when reasoning about the JSON contract, and there are now exactly two silently-dropped fields, both introduced by this step.

The second half of the same sentence is still true and needs no change: `skip_serializing_if` appears only in `src/plan/source.rs`, never in `src/next.rs` or `src/main.rs`, and `"resume_state": null` is still in `GOLDEN_JSON` (`src/next.rs:2117`).

This is inside the decided scope. It is a present-tense sidecar claim about `src/next.rs` that this step's own increments falsified, which is item (1) of `Q-55-currencyscope` and the class the inc4 description at `:282` names verbatim.

## R1C-2. The `Q-55-jsonreason` problem statement still asserts the pre-inc2 state (`medium`)

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:195`:

> `no_active_loop_reason` is `#[serde(skip)]` (`src/next.rs:NextProjection::no_active_loop_reason`) and `status`'s `Projection` has no reason field at all, so under `--json` an omitted part serialises as a bare `null` with nothing distinguishing why.

Both halves are false at `079d63f`. The first is R1C-1's evidence. The second:

```
$ grep -n "struct Projection" -A 9 src/main.rs
569:struct Projection {
...
577-  metrics_absent_reason: Option<next::MetricsAbsentReason>,
```

`status`'s `Projection` carries a reason field, and a live run shows it on the wire:

```
$ agent-scaffold status --json --source "$FIXTURE/docs/plans/TEMPLATE.plan.toml"
{
  "plan": { ... },
  "metrics": null,
  "metrics_absent_reason": "log-absent"
}
```

The preamble "THE PROBLEM, in the form that decided it" is a partial defence, but it is weaker than the ones the pass DID re-tense. `:44` ("That WAS an OVERSTATEMENT: the skip WAS announced") and `:46` ("The arm that FIRED here WAS") are the same shape, decision-time framings of a then-current state, and both were converted. `:208` is the closest parallel of all, the same section, the same "as it stood when the decision was taken" role, and the pass converted "has ... no test" to "HAD no test". Leaving `:195` in the present tense while converting `:208` is an inconsistency inside one increment's own work, not a defensible line.

For the twin of this same sentence in the plan TOML, see R1C-5.

## R1C-3. The third site of the `Q-55-twinsites` claim survives, in the sidecar itself (`medium`)

`Q-55-twinsites` was raised because "`status --json` has no golden and NO TEST ON ITS SERIALISATION AT ALL" survived in `tests/unsafe_pairings_are_refused_and_omitted.rs` after the sidecar was corrected. The human ruled FIX BOTH TWINS, and the pass did: `tests/unsafe_pairings_are_refused_and_omitted.rs:1370` now reads "pinned on BOTH commands because `status --json` has no golden", and sidecar `:208` now reads "has NO golden, and HAD no test on its serialisation at all".

A THIRD site of the identical claim survives, in the same file as one of the two that were corrected. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:304`:

> ... and one of the two commands (`status --json`) has no test on its serialisation at all, so that half is carried by the acceptance check rather than by the suite.

Present tense, and false:

```
$ grep -n '"status", "--json"\|"status",$' tests/unsafe_pairings_are_refused_and_omitted.rs | grep json
403:    vec!["status", "--json", "--source", &away_plan, "--metrics", local],
764:    &["status", "--json", "--source", &missing, "--metrics", "docs/metrics/workflow.jsonl"],
1404:    &["status", "--json", "--source", &away_plan, "--metrics", "docs/metrics/workflow.jsonl"],
1425:  let (code, stdout, stderr) = run(&home, &["status", "--json", "--source", &away_plan]);
1586:    run(&home, &["status", "--json", "--source", &alpha_source, "--plan", &beta_plan]);
1738:    run(&home, &["status", "--json", "--source", "docs/plans/p.plan.toml"]);
```

Six invocations, the same six the ledger records the orchestrator counting when it raised `Q-55-twinsites`. Two of them (`:1404`, `:1425`) are inside `the_machine_surface_separates_the_causes_on_both_commands`, whose own doc comment the pass edited in this very diff to drop the claim. So the pass deleted the sentence at one site, re-tensed it at a second, and left it standing at a third that is eight hundred lines below the second in the same file.

The trailing clause "so that half is carried by the acceptance check rather than by the suite" is false for the same reason and goes with it.

This is the recorded repeat failure mode of this task, at its fourth occurrence. It is inside the decided scope on the same ground `Q-55-twinsites` was ruled in on.

## R1C-4. `The four accepted costs` was never swept, and three of its present-tense claims are falsified (`medium`)

The diff contains no hunk in the section `## The four accepted costs` (`:251-263`). Three claims in it describe behaviour that inc2 and inc3 changed, and each contradicts the acceptance check that pins the same case.

(a) `:255`, cost (i): "A BARE FILENAME RUN FROM INSIDE `docs/plans` REMAINS A SILENT MISS." Reproduced against this worktree:

```
$ cd docs/plans && agent-scaffold validate --source agent-scaffold.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there
exit: 1
```

It is a loud miss, not a silent one. The paragraph's own last sentence says so, but in the future tense ("After the tier policy lands, this case becomes a HARD FAILURE"), and the tier policy landed at `3d00341`. Check 18 states it correctly ("After inc3: a HARD FAILURE naming the path it looked for"), so the file's specification and its prose disagree.

(b) `:257`, cost (ii): "This is a genuine new failure for a layout that works today." That layout does not work today; it has been refused since inc2, which is what `accepted_cost_two_the_symlinked_layouts_are_pinned` (`tests/unsafe_pairings_are_refused_and_omitted.rs`) asserts, and it is what check 19 pins.

(c) `:259`, cost (iii): "`--source <root>/docs/plans/x.plan.toml --plan <root>/notes/p.md` greens today against `<root>/docs/metrics/workflow.jsonl`". Reproduced on a purpose-built fixture:

```
$ cd $S && agent-scaffold validate --source docs/plans/x.plan.toml --plan notes/p.md --workflow
--workflow would join notes/p.md against docs/metrics/workflow.jsonl, which is not under the plan's project root .../cost3/notes; pass a `--metrics` under that root, run against the plan's own log, or correct the `--source` and `--plan` pair
exit: 1
```

Exit 1, not green. Check 19b already states the correct pair ("prints `workflow invariants hold` at exit 0 before inc2 and exits NON-ZERO after it"), and `accepted_costs_three_and_four_are_pinned` asserts `Some(1)` at `tests/unsafe_pairings_are_refused_and_omitted.rs:1701`.

All three are present-tense sidecar claims about the tool that this step's own increments falsified, so all three are inside `Q-55-currencyscope` item (1). Cost (iv) at `:261` is fine and needs no change.

## R1C-5. The plan TOML's `Q-55` record is a whole cluster of the same falsified claims, and the rendered view now contradicts itself (`medium`)

Every passage the pass re-tensed in the sidecar has a twin in the `Q-55` question record at `docs/plans/agent-scaffold.plan.toml:1713-1736`, which renders verbatim into `docs/plans/agent-scaffold.md:152-170`. None of the twins was touched. Naming each with its corrected sidecar counterpart:

| plan TOML claim | corrected sidecar counterpart | why false now |
| --- | --- | --- |
| "`no_active_loop_reason` is `#[serde(skip)]` ... and `status`'s projection has no reason field" (`:1734`) | `:195` (uncorrected, R1C-2) | `src/next.rs:192` has no skip; `src/main.rs:577` is the reason field |
| "`README.md:228` says "Unlike `validate` it never fails on a missing or malformed file"" (`:1732`) | `:173`, changed from `:228` to `:238` by this pass | `README.md:228` is now a comment line inside a code fence; the sentence is at `:238` |
| "the `--workflow has a plan source but the metrics log is missing; skipping the workflow check` note" (`:1722`) | `:44`, which gained "that inc3 replaced with a reported problem" | `grep -rn "skipping the workflow check" src/` returns nothing |
| "The metrics-log path resolves against the CURRENT WORKING DIRECTORY (`src/main.rs:ValidateArgs::metrics`)" (`:1724`) | `:102`, re-tensed to "WAS declared with a RELATIVE default" | `--metrics` is `Option<PathBuf>`; `resolve_metrics_path` anchors it |
| "`status` ... `next` ... and the derived ledger path ... carry the identical CWD-relative defect" and "`next` emits `state: converged` ... at exit 0" (`:1728`) | `:110`, re-tensed to "IT DID NOT ... and BROKE" | closed by inc1 and inc2; check 5 and check 14b pin it |
| "reads an unconditional promise of the `validate --workflow` backstop (`pack/AGENTS.md:93`)" (`:1728`) | `:139`, re-tensed to "it WAS UNCONDITIONAL" | `pack/AGENTS.md:93` now reads "when instrumentation is on, the deterministic `validate --workflow` check is the backstop ..." |
| "a BARE FILENAME run from inside `docs/plans` remains a silent miss" (`:1736`) | `:255` (uncorrected, R1C-4a) | see R1C-4a's reproduction |

The rendered view is where this bites hardest, which is the point of lens (D). A reader of `docs/plans/agent-scaffold.md` alone meets `:168` saying `no_active_loop_reason` IS `#[serde(skip)]` and `status`'s projection HAS no reason field, and then `:1614` saying it is "NO LONGER `#[serde(skip)]`" and `:1609` specifying `metrics_absent_reason` on both projections. `render --check` passes, so nothing mechanical catches it: the generated file matches a render of a source that disagrees with itself.

SCOPE NOTE, stated rather than assumed. `Q-55-currencyscope` named the sidecar and three other sidecars; it did not name the plan TOML. But these claims were falsified by THIS step's own increments, not by anything predating it, so they are not in the declined "pre-existing false doc claims that predate this step" class, and the round-3 triager's condition 3 that `Q-55-twinsites` was ruled in on ("a stale claim THE INCREMENT'S OWN CHANGE BROKE is in scope regardless of authorship") reaches them directly. Against that, a `[[question]]` `ask` is a decision receipt and there is a real argument that receipts are frozen history that must read as written at decision time. I do not think a reviewer should settle that; it is a human decision of the same shape as `Q-55-twinsites`, and it should be put rather than assumed in either direction. What is NOT arguable is that the rendered view now contradicts itself on a fact, whichever way the receipt question goes.

## R1C-6. The pass re-pointed a `src/checks.rs` citation that check 21b, written in the same commit, declares out of scope (`low`)

Scope boundary, the "pulled in something outside" direction. In `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md:55` the pass changed:

```
-(`src/checks.rs:862-871`, `src/main.rs:1726-1731`, ...)
+(`src/checks.rs:1037-1046`, `src/main.rs:2280-2285`, ...)
```

The `src/main.rs` half is inc1-induced drift and is squarely in scope. The `src/checks.rs` half is not. `src/checks.rs` has not been touched since `09a027c` (2026-07-31), while this step's first source commit is `609ddcf` (2026-08-01), so no increment of `workflow-enforcement-tier` moved a line in that file and the drift has another cause, the owning step at order 93.

Check 21b, added by this same commit, says the opposite of what the commit did: "every `src/main.rs` and `tests/` citation in the three is opened at its cited range and shown to hold its named subject", "AND ONLY THOSE", and then "its `src/checks.rs` citations point at code the fix deliberately replaced ... That is the owning step's closure work, not this one's; pulling it in would widen a scope the human closed (`Q-55-currencyscope`)."

The edit is harmless in effect: `src/checks.rs:1037-1046` is `fn scratch(name)`, the correct subject, and `test-tmpdir-repo-assumption.md` already cited the same range for the same helper before this pass. Rated `low` for that reason. It is still a breach of a boundary the human closed and it makes check 21b's own text untrue of the commit that introduced it, so the remedy is either to revert that one citation or to widen check 21b's wording to admit it. Note that the sibling `src/checks.rs:400-405` citation in the same file, cited for `owning_pid`, still resolves to `fn git`; per the review brief I am NOT raising that as a defect to fix, only recording that it was correctly left alone while its neighbour was not.

## The scope boundary in the other direction: what held

Checked and clean, so the pass did not creep:

- `run_validate`'s "`--plan` still clap-required" doc claims: `src/main.rs` shows one changed line in the whole diff, `:570`, the `Projection.plan` comment. Untouched.
- `src/next.rs:162` and `:181-183`: `src/next.rs` is not in the diff's file list at all.
- The Status narrative at `docs/plans/agent-scaffold.md:7`: unchanged; the rendered diff contains no hunk before `:152`.
- `README.md`, `pack/AGENTS.md`, `CHANGELOG.md` and the deployed `.agents/` copies: none in the diff, matching the inc4 "NOT" bullet at `:387`.

## Does the step close (lens E)

Not yet, and the blocker is the increment's own acceptance check rather than anything external.

- Check 21 ("EVERY CITATION AND EVERY QUOTATION IN THIS FILE RESOLVES") fails on R1C-1 and R1C-2: `src/next.rs:NextProjection::no_active_loop_reason` is cited twice as the site of a `#[serde(skip)]` that is not there.
- The inc4 description at `:282` claims the pass covers "This file's own descriptions of `src/main.rs`, `src/next.rs`, the test suite ... re-tensed ... or deleted". R1C-1, R1C-3 and R1C-4 are exactly such descriptions, unswept, so that sentence is itself a claim the increment leaves behind that does not match the tree it leaves behind, which is the increment's own stated review question.
- Checks 21b, 22 and 23 are satisfied. I re-ran 22 and 23 first-hand (below), and confirmed 21b's three sidecars resolve on their `src/main.rs` and `tests/` citations.
- No check is unrunnable as written. Check 16's newly widened text reproduces exactly (below).

MECHANICAL GATES, all four run first-hand in this worktree at `079d63f`, all four clean:

```
$ cargo test                                     -> exit 0, all suites pass
$ cargo clippy --all-targets -- -D warnings      -> exit 0, no diagnostics
$ cargo run -- render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date          exit 0
$ cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 286 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold   exit 0
```

`TMPDIR` was outside the repository for the suite run, per the acceptance check's preamble.

## What I exercised, and what I did not

DIMENSIONS I VARIED. I did not start from the diff. I read all 404 lines of the sidecar as it now stands and asked of each descriptive sentence whether it is true of the tree at `079d63f`.

Citations opened at their cited range or resolved by symbol, all of which hold unless named above: `src/main.rs` `ScaffoldArgs::instrument`, `ValidateArgs::metrics`, `ValidateArgs::workflow`, `StatusArgs::metrics`, `StatusArgs::resume`, `StatusArgs::ledger_fragment`, `NextArgs::metrics`, `NextArgs::ledger_fragment`, `AuditArgs::out`, `Projection`, `run_validate` (the four-arm match and all four arm patterns), `run_status`, `run_next`, `run_resume`, `default_ledger_path`, `project_root_of_source`; `src/next.rs` `NextProjection` and its `metrics`, `active_loop`, `resume_state`, `no_active_loop_reason` fields, `tests::GOLDEN_JSON`, `tests::golden_json`, `GOLDEN_HUMAN`, `golden_human_text`, `LoopState`, `derive_task`, `build_context`, `has_risk_class_conflict`, `select_active_loop`'s `build_pending_loop(step, LoopState::Blocked, ...)` branch, the three `no_loop_reason` strings; `src/workflow.rs:180-195` and `:448-449`; `src/plan/render.rs:296` and `:167-169`; `src/plan/source.rs:102` and `:480-495`; `src/findings_naming.rs:52-55`; `src/metrics.rs` `count_records` and `parse_rounds`; `tests/validate_workflow_toml_source_needs_no_plan.rs:127-171`; `README.md:210`, `:212-232`, `:238`, `:242-260` (all four exact); `pack/AGENTS.md:61`, `:63`, `:93`, `:116`; `pack/instrument.md`; `CHANGELOG.md`'s `## [Unreleased]` subsections; and the plan-TOML orders 60, 63, 64, 88, 93, 95, 96.

Quotations run as literal searches against the file each is attributed to: the pack backstop sentence (old and new), `pack/instrument.md`'s closing line and its `validate` sentence, `run_validate`'s `(None, None, _)` comment, the two stderr notes, the `README.md:238` never-fails sentence, `run_validate`'s superseded doc comment, `metrics: unavailable, <reason>`, `no active review loop (<reason>)`, `no plan steps found`, `all steps complete`, `no in-progress or ready step`, and `"resume_state": null` in the golden.

Runs, all at uid 1000 in a scratch fixture outside any repository: the non-instrumented scaffold rebuild (check 2, "30 changed, 0 left untouched", `ls docs` prints only `plans`); check 15; check 20 (the fixture's `AGENTS.md` carries the qualifier); check 22; check 23; accepted cost (i) from inside `docs/plans`; accepted cost (iii) on a hand-built two-substrate fixture; and BOTH spellings of check 16's probe-that-cannot-answer, the mode-600 directory (`Permission denied (os error 13)`) and the trailing slash (`Not a directory (os error 20)`), each against plain `validate` (exit 0, absent-log note) and against `--workflow` (exit 1, the could-not-be-checked problem). The mode-600 directory was restored to 755 before I finished.

Twin sweeps: I grepped the whole tree (excluding `target/` and other worktrees) for each of the twenty corrected passages, by phrase and by paraphrase, across `src/`, `tests/`, `README.md`, `pack/`, `.agents/`, `CHANGELOG.md`, `AGENTS.md`, every sidecar under `docs/plans/agent-scaffold.steps/`, `docs/plans/agent-scaffold.plan.toml` and the rendered `docs/plans/agent-scaffold.md`. Case-insensitively, so the capitalised-first-word miss recorded earlier in this task cannot repeat.

WHAT I DID NOT REACH, so do not read this round as covering it.

- The RED halves of checks 3 to 14h. I built no binary from a parent commit and re-ran no pre-increment reproduction, so I checked only the post-state. A red-then-green claim that never actually went red would survive this round.
- The multi-project fixtures behind checks 5 to 14h and 19: I exercised those only through `cargo test`, not by hand. In particular I did not build a symlinked `docs/plans` or `docs/metrics` layout myself and relied on `accepted_cost_two_the_symlinked_layouts_are_pinned` passing.
- Uid was not varied. Check 16's "at every uid including root" claim for the trailing-slash spelling is verified here at uid 1000 only.
- The quality of the newly authored inc4 prose as prose (another reviewer's lens this round), and the citation-resolution sweep as an independent exercise (likewise). Where those overlap my findings I opened the range myself rather than relying on the other lens.
- `docs/plans/agent-scaffold.ledger.md`'s own currency. It is orchestrator narrative, it is not in the pass's scope, and I did not sweep it for the same stale claims, though R1C-5's cluster suggests it would repay one.
- The four inc2 and four inc3 recorded residuals, which I did not re-examine and have not re-raised.
- The `src/checks.rs` citation staleness in `checks-runner-worktree-name-collision.md`, which check 21b assigns to the owning step. I confirmed only that the pass declared it correctly, and that it nonetheless edited one of them (R1C-6).
