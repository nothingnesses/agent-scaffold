### `workflow-enforcement-tier`: fail when `--workflow` cannot run, and stop the join reading another project's log (`Q-55`)

Two defects in `validate --workflow`, both reproduced by running the tool on 2026-07-31 rather than quoting the `Q-44` audit, fixed as one step in two increments. Defect A is `Q-55` proper (the `SE-3` two-tier enforcement gap): a project with no metrics log gets a green from a check that never ran. Defect B is not in `Q-55`'s text and is worse: the metrics-log path resolves against the current working directory rather than the plan source, so the check can join one project's plan to another project's log and declare the invariants hold.

Provenance. `Q-55`, decided by the human on 2026-07-31, with two receipts in `docs/metrics/workflow.jsonl`: `type:"decision"` `q_id:"Q-55"` (the enforcement tier) and `type:"decision"` `q_id:"Q-55-scope"` (one step, two increments). Both carry `task:"workflow-enforcement-tier"`.

THE FIXTURE, shared by both reproductions. It is a throwaway non-instrumented project, which is what every new adopter is by definition:

```sh
agent-scaffold scaffold --output-dir "$SCRATCH" --write --force --principles default
```

That drops 30 files ("Wrote to $SCRATCH (30 changed, 0 left untouched)") and NO `docs/metrics/` directory: `ls "$SCRATCH/docs"` prints only `plans`. The instrumentation asset is gated behind `--instrument`, which is off by default (`src/main.rs:416-420`), so the round log the workflow check reads does not exist. Then `validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow` is run two ways, from inside the fixture and from the agent-scaffold repository root.

## Defect A, the false green (`Q-55` proper)

Run from INSIDE the fixture:

```sh
cd "$SCRATCH"
agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
```

Observed, with the streams separated:

```
stdout: docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
stderr: no metrics log at docs/metrics/workflow.jsonl; nothing to validate
stderr: --workflow has a plan source but the metrics log is missing; skipping the workflow check
exit:   0
```

CORRECTION TO `Q-55`'S OWN WORDING, which the implementer should not carry forward. `Q-55` says `validate --workflow` "silently passes". That is an OVERSTATEMENT: the skip IS announced, twice, at `src/main.rs:845` and `src/main.rs:1001-1003`. The correction is sharper than it first looks, because both announcements go to STDERR (both are `eprintln!`), while the only thing on stdout is the ok summary. The operative defect is therefore the EXIT CODE and nothing else: a CI gate reads the exit status, not the stderr log, so `validate --workflow` returns success for a project with zero machine enforcement of any workflow invariant. Write it up as a false green, never as a silent one.

THE MECHANISM. The `--workflow` block is a four-arm `match` at `src/main.rs:958-1004` over `(toml_primary, &plan_contents, &metrics_contents)`. The arm that fires here is the `_` catch-all at `src/main.rs:999-1003`, which prints the skip note and pushes NO problem, so `problems` stays empty and `run_validate` takes the success branch at `src/main.rs:1007-1011`.

THE DECIDED REMEDY (human, 2026-07-31, receipt `q_id:"Q-55"`): FAIL WHEN `--workflow` CANNOT RUN. When `--workflow` is explicitly requested and no metrics log exists at the resolved path, the run exits non-zero and reports why. Plain `validate` (and `validate --source` / `--plan` without `--workflow`) is UNAFFECTED: an absent log there stays a stderr note and exit 0, because nobody asked for the check. The reasoning the human accepted: the user explicitly asked for the workflow check, so skipping it and reporting success is the defect. The policy applies to the resolved metrics path however it was resolved, whether from the `--metrics` default or from an explicit `--metrics` value; "the user named a path that does not exist" is not a weaker case than "the default path does not exist".

WHAT WAS REJECTED, AND WHY. Make round-logging CORE rather than opt-in: rejected as forcing instrumentation on users who did not ask for it, and a substantially bigger change than the defect warrants (Minimal by default; the `--instrument` opt-in is the two-tier design working as intended, not the bug). Warn but keep exit 0: rejected because a warning with exit 0 is still green in CI, which is the actual failure mode being fixed; it would leave the defect in place and add a message beside it.

THE PRECEDENT THIS FOLLOWS, ALREADY IN THE TREE, so this is closing the second half of a hole whose first half is already closed. The sibling arm at `src/main.rs:995-998` handles "`--workflow` requested but no plan source resolved" and pushes a hard problem, with the comment stating the identical reasoning: "`--workflow` was explicitly requested, so skipping would green-pass while checking nothing; make it a hard problem instead." That arm was itself a false-green fix (Inc 6, finding M-1) and is pinned by `tests/validate_workflow_toml_source_needs_no_plan.rs:89-132`. That test's own comment at `:96-98` records the gap this increment closes: "with a source present but metrics missing the tool still soft-skips". Defect A is the same defect on the other input, and the fix is the same shape.

A CASE THAT IS NOT PART OF THIS, named so the increment does not creep into it. A metrics log that is PRESENT but EMPTY is not a false green and needs no change: the check runs, and W3 fails on any `complete` step with no round records (demonstrated in the acceptance check below). The missing-file case is different in kind because the check does not run at all.

## Defect B, cross-project contamination

Not in `Q-55`'s text, found only by running the tool, and a straight bug independent of whichever tier policy was chosen. Run from the agent-scaffold repository root against the fixture's plan:

```sh
cd /path/to/agent-scaffold
agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow
```

Observed (at commit `fe62ca1`; the record count grows as the log accumulates, and the orchestrator's first reproduction two commits earlier read 233):

```
stdout: docs/metrics/workflow.jsonl: 235 records, valid
stdout: $SCRATCH/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
stdout: $SCRATCH/docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit:   0
```

Those 235 records are AGENT-SCAFFOLD'S OWN log, joined against a foreign one-step plan, and the join was declared to hold. The green is not merely empty, it is affirmatively wrong: it asserts an invariant over a pairing of two unrelated projects.

THE SHARPER DEMONSTRATION, which turns "wrong pairing" into a measurable FALSE PASS and is the red case the increment's test should pin. Give the fixture's single step a slug that has round records in agent-scaffold's log, and mark it complete:

```sh
cd "$SCRATCH"
# in docs/plans/TEMPLATE.plan.toml, on the single [[step]]:
#   slug   = "example-step"    -> "triager-runs-only-on-findings"
#   status = "not-started"     -> "complete"
cp docs/plans/TEMPLATE.steps/example-step.md docs/plans/TEMPLATE.steps/triager-runs-only-on-findings.md
```

The fixture now claims a completed, reviewed step and has no review evidence of its own whatsoever. From the agent-scaffold root, `agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow` still prints `workflow invariants hold` and exits 0. Agent-scaffold's own rounds for its step 78 satisfied a foreign project's convergence claim.

THE CONTROL, which proves the check itself is sound and the defect is purely in which file it reads. From inside the fixture, with an empty log so the check can run against the RIGHT project:

```sh
cd "$SCRATCH" && mkdir -p docs/metrics && : > docs/metrics/workflow.jsonl
agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
```

```
stderr: docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped
exit:   1
```

THE MECHANISM IN THE CODE. `--metrics` is declared with a RELATIVE default, `#[arg(long, default_value = "docs/metrics/workflow.jsonl")]` at `src/main.rs:429-431`, and every use of it (`metrics_path.exists()` and `fs::read_to_string` at `src/main.rs:823-847`) resolves that relative path against the PROCESS working directory. The `--source` path is taken as given. Nothing anywhere reconciles the two, so the pairing of plan and log is an accident of where the user happened to stand. `check_workflow_toml` (`src/workflow.rs:180-195`) then joins whatever it is handed, and W3 matches a round to a step by SLUG alone (`round_step_slug(round) == step.slug`, `src/workflow.rs:448-449`), with no project identity anywhere in the record or the join, which is why a borrowed slug is enough.

THE REQUIRED END PROPERTY, which is what "done" means for this half regardless of the mechanism chosen: `validate --workflow` must never pair a plan source with a metrics log belonging to a different project and report success. Where the tool cannot establish that the two belong together, it must say so and exit non-zero rather than proceed. A run made from the plan's own project root, which is the normal invocation and the only one the scaffolded guidance documents, must be unchanged (Safe on existing projects).

CANDIDATE MECHANISMS AND THEIR TRADE-OFFS. NOT pre-decided here: the human decided the tier policy and the increment split, not this. The implementer picks one and argues it in the commit, or raises the choice if it reads as a genuine fork. Note that (a) and (d) are the same idea at two scales, and that all of (a), (b), (c) leave the round record itself carrying no project identity, so they fix the path and not the data.

- (a) ANCHOR THE DEFAULT to the plan source's project root: when `--metrics` was not supplied, resolve `docs/metrics/workflow.jsonl` relative to the root derived from the `--source` (or `--plan`) path rather than to the CWD. The `<root>/docs/plans/<task>.plan.toml` layout this derives from is the one the scaffolder drops and the one `default_ledger_path` (`src/main.rs:1133-1138`) already assumes. Cheapest fix that makes the common case right by construction, and a no-op when the user already stands in the plan's root. Two things must be got right: distinguishing "the user passed `--metrics`" from "the default fired" (clap's `ArgMatches::value_source`, or better, make the field `Option<PathBuf>` and apply the default after resolution, which makes the distinction representable rather than recovered, per Make illegal states unrepresentable), and deciding what happens when the source is NOT under `docs/plans/` (for example `--source myplan.plan.toml` at a repository root), where the derivation has no convention to lean on.
- (b) DETECT AND REFUSE: keep the CWD-relative default, but make the `--workflow` join fail when the resolved metrics path and the plan source do not share a project root. Loudest and changes the least, but it still needs the same root derivation as (a), so it does not avoid the hard part, and it leaves the user to pass `--metrics` by hand on every cross-directory run.
- (c) REQUIRE `--metrics` EXPLICITLY whenever the plan source is not under the current directory. Cheapest to write and impossible to get subtly wrong, but it pushes the work onto the user and does nothing for the case where both paths are relative and the pairing is wrong anyway.
- (d) LET THE PLAN DECLARE ITS OWN LOG: a `[meta]` field naming the metrics path relative to the source, defaulting to today's convention. Structured data first, project for humans, applied properly: the pairing becomes data the plan owns rather than a CLI convention the user must reproduce. Costs a schema field, its validation, its render, and a migration story for every existing plan that omits it. Larger than this step, and a candidate for the queued validation-constraints step rather than here; naming it matters because it is where the cleaner long-term architecture points (Prefer the cleaner long-term architecture over the smallest diff).

## The two increments, and why in this order

The human decided one step, two increments (receipt `q_id:"Q-55-scope"`), chosen over a separate step ahead of `Q-55` and over folding this into the validation-constraints step. Both defects live in the same twenty lines of `run_validate` and produce the same user-facing green, so separate steps would mean reviewing the same region twice; separate INCREMENTS keep the path fix reviewable on its own terms.

- `workflow-enforcement-tier-inc1`, THE PATH FIX (defect B). Anchor the metrics log to the plan source per the chosen mechanism, plus the red-then-green test that pins the false pass above. No exit-code policy change: after inc1 alone, a `--workflow` run whose anchored log is missing still soft-skips, which is defect A, still open by design.
- `workflow-enforcement-tier-inc2`, THE TIER POLICY (defect A). Turn the missing-log case at `src/main.rs:999-1003` into a reported problem so `--workflow` exits non-zero, leaving plain `validate` untouched, plus its own red-then-green test and the documentation updates below.

INC1 GOES FIRST, and the order is load-bearing rather than arbitrary. If the tier policy landed first, a user whose `--workflow` run started failing could "fix" it by running from a directory that happens to contain a log, which is exactly the contaminated green of defect B; the fix would hand users a workaround that walks them into the worse bug. Landing the path fix first means that by the time missing-log becomes an error, the log being looked for is the right one.

## Risk classification

Both increments are `risky` (two consecutive clean rounds each). The classifications are argued separately because the reasons differ.

`workflow-enforcement-tier-inc1` is `risky`. It changes WHICH FILE the validator reads on every invocation that does not pass `--metrics`, and the failure mode of a wrong anchor is not a crash but a confident wrong answer, which is the same class of defect the increment exists to remove and is self-concealing in exactly the same way. It also compounds forward: once inc2 lands, a mis-derived anchor stops being "reads the wrong file" and becomes "hard-fails a correctly instrumented project", so an inc1 defect escalates into a broken gate rather than staying a quiet one. Relative-path handling is also this tree's recorded weak spot rather than a neutral area: `is_safe_sidecar_ref` (`src/plan/source.rs:480-495`) exists because of it, and two backlog steps sit in the same family (`sidecar-ref-empty-string`, order 63, and `sidecar-ref-symlink`, order 64). The counter-argument, that the eventual diff is small and there is a deterministic test for it, loses to the fact that the blast radius is every future invocation of the project's main validation surface.

`workflow-enforcement-tier-inc2` is `risky`. It changes a CLI EXIT CODE, which is the most externally depended-on contract the tool has, and it is intended to flip a currently-passing gate to failing for every non-instrumented project, which is precisely the population that Safe on existing projects protects. That principle is not overridden lightly here; what authorises the break is Principle 8 (Structured data first, project for humans), whose own text records the human's 2026-07-18 decision that at this pre-adoption stage the best long-term design wins over backwards-compatibility and that it beats Principle 3 when the two conflict. An authorised break is a reason to make the change, not a reason to review it less. The boundary is also easy to get subtly wrong in a way tests may not catch by accident: the `_` catch-all at `src/main.rs:999-1003` covers both the TOML-source-present and the Markdown-plan-present variants of "metrics missing", and the fix must convert exactly those to problems without capturing a case that should stay a skip, and without touching the no-`--workflow` path at all. The counter-argument, that the in-tree precedent at `src/main.rs:995-998` makes this a three-line change, is the argument this project has already recorded losing: the size of a diff is not the size of its blast radius.

## Acceptance check

Every claim below is a command with an expected exit code, so the round is settled by running it rather than by reading the diff. `Q-66` (reproducible evidence proportional to the claim, step 88) applies: both defects are behavioural, so each increment owes a test that is RED against the pre-fix build and green after, and the round report states which mutation or which pre-fix revision produced the red.

1. Build: `cargo build`. Suite and lint: `cargo test` and `cargo clippy --all-targets -- -D warnings`, both clean. Plan render pinned: `cargo run -- render docs/plans/agent-scaffold.plan.toml --check`.
2. Rebuild the fixture from scratch (the command at the top of this file) and confirm `docs/metrics/` is absent.
3. Defect A closed, after inc2: from inside the fixture, `agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow` exits NON-ZERO and reports the missing log by path.
4. Plain `validate` unaffected, which is the other half of the decision and the easiest thing to break by accident: from inside the fixture, `agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml` (no `--workflow`) still exits 0 and still prints `no metrics log at docs/metrics/workflow.jsonl; nothing to validate` on stderr. Same for bare `agent-scaffold validate`.
5. Defect B closed, after inc1: from the agent-scaffold root, `agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow` does NOT read agent-scaffold's own `docs/metrics/workflow.jsonl` and does NOT print `workflow invariants hold`. With inc2 also landed it exits non-zero naming the fixture's own missing log.
6. The false pass is dead: rerun the borrowed-slug demonstration above (fixture step `complete` with slug `triager-runs-only-on-findings`) from the agent-scaffold root. Before the fix it exits 0 with `workflow invariants hold`; after, no green under any invocation that is not looking at the fixture's own log.
7. The control still works, proving the fix removed a wrong answer rather than the check: put an empty `docs/metrics/workflow.jsonl` in the borrowed-slug fixture and run from inside it. Expect exit 1 and the W3 message quoted above naming `triager-runs-only-on-findings`.
8. No regression on the correct case, which is the Safe on existing projects check: from the agent-scaffold repository root, `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` still exits 0 with `workflow invariants hold`, reading this repository's own log exactly as before.

## Documentation impact

All in-repo; the implementer updates every item in the same change rather than leaving a documentation step owed.

- `src/main.rs:791-816`, the `run_validate` doc comment, which states the superseded policy in as many words ("An absent file (the metrics log, or a `--plan` path) is not a validation failure ... a missing file prints a note to stderr and is skipped rather than hard-failing (the same treatment for both, so the behaviour is consistent)"). After inc2 the treatment is deliberately NOT the same for both, and the comment must say why.
- `src/main.rs:429-431` (the `--metrics` help, which after inc1 no longer describes a plain CWD-relative default) and `src/main.rs:438-440` (the `--workflow` help, whose closing sentence enumerates the error cases and gains one).
- `tests/validate_workflow_toml_source_needs_no_plan.rs:1-13` (the module doc, which frames the false-green rule as being about the plan source only) and `:96-98` (a comment that asserts the soft-skip this step removes). Inc2's new case is a sibling of the two already there and belongs in this file; inc1's needs a two-project fixture and is likely a new file.
- `README.md:210`, the `validate` paragraph in "Validating and projecting workflow state", which currently says only that it "exits non-zero if any exist, so it can gate a commit or run in CI" without stating that a `--workflow` run which cannot see a log is itself a failure. The example block at `README.md:212-224` is where a note about the log being resolved relative to the plan source belongs.
- `CHANGELOG.md`, the `## [Unreleased]` section. This is a user-visible behaviour change, so it is at minimum a `### Changed` entry; the section currently has `Added` and `Changed` and no `Fixed`, so check what a comparable fix did before introducing a new subsection.
- NOT the pack, on current reading: `pack/instrument.md:13` says `validate` "exits non-zero and reports any malformed record", which stays true, and `pack/AGENTS.md:93` describes the workflow check as the backstop without claiming an exit code. If the implementer does find pack text that goes stale, the deployed `.agents/` copies must be regenerated in the same change or the drift guards fail.

## Scope: what this step does not do

- It does not make round-logging core. That was the rejected option, not a follow-up.
- It does not close the DOCUMENTATION half of `SE-3`. `Q-55`'s text also records that the two-tier split is undocumented in the scaffolded AGENTS.md, so a user reading the role prompt expects the same guarantees whether or not they instrumented. The decision covered the enforcement tier only, and this step does not silently widen into the guidance; the gap is raised for the human rather than assumed either way.
- It does not touch `status` or `next`, which carry the identical CWD-relative metrics default (`src/main.rs:455-457` and `src/main.rs:479-481`), nor `default_ledger_path` (`src/main.rs:1133-1138`), which builds a CWD-relative `docs/plans/<task>.ledger.md` the same way. They are named here so the implementer recognises the family and does NOT widen the increment into it; whether they are in scope is raised for the human. Both are best-effort projections rather than validators, so a wrong path there yields an empty projection rather than a false assertion, which is why they are separable.
- It does not change any check logic in `src/workflow.rs`. W3, W4 and W5 are correct; they were handed the wrong log.
- It does not add project identity to the round record. Anchoring a path stops the accidental pairing, but the record still carries nothing that ties it to a project and W3 still joins on a bare slug (`src/workflow.rs:448-449`), so a deliberately or accidentally shared log remains joinable. That is a data-model question for the queued validation-constraints step, and mechanism (d) above is its natural home.
