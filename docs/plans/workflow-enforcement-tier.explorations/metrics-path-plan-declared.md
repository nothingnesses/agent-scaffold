# Exploration: candidate (d), let the plan declare its own log

- Explorer model: Opus 5 (1M context), exact model id `claude-opus-5[1m]`.
- Date: 2026-07-31.
- Worktree: `.claude/worktrees/explore-metricspath-b`, branched from `main` at `9f50929`.
- Brief: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, candidate (d).
- Mandate: build candidate (d) and measure it, adversarially, including against itself.

Everything below was run in that worktree with a throwaway build. The code that produced it is UNCOMMITTED and is reported at the end.

## Summary of the result, stated first

Candidate (d) was built, including the sibling extension to `status`, `next`, and the ledger. It works. It also does not do what it was proposed to do.

Two findings decide it.

1. The declared field is not what kills the false pass. The changed DEFAULT is, and that default is candidate (a). Every plan in existence omits the field (measured), so on the day (d) ships, 100 percent of its effect on the false pass comes from resolving the absent-field case against the plan directory. Holding the absent-field case at today's CWD default and shipping only the field leaves the false pass reproducing byte for byte.
2. The declared field is a new way to reintroduce the exact same false pass, in a worse form. A plan can declare a path that walks out to another project's log; the false pass then reproduces, exits 0, prints `workflow invariants hold`, is INDEPENDENT of the working directory, is committed to the repository, and passes `validate --source` clean. Today's false pass is a transient property of where the user happens to stand. Candidate (d)'s is a durable property of a committed file that reproduces in CI.

The validation rule cannot close (2), for a structural reason: the conventional log lives OUTSIDE the plan directory (`docs/plans/` to `docs/metrics/`), so the source-relative spelling of the default itself needs a `..` component. The project's existing containment rule, `is_safe_sidecar_ref` (`src/plan/source.rs:489-495`), exists precisely to forbid `..`, and it cannot be reused here. The strongest rule available for this field is "not absolute", which stops nothing.

Recommendation, argued in the last section: ship candidate (a) for the path fix in this step, and do NOT ship a `[meta].metrics` path field either here or in the queued step. What the queued validation-constraints step should carry instead is project IDENTITY in the round record and in the plan, which is the thing the sidecar's own closing bullet already names.

## Verification of the brief, before building anything

The brief's reproduction and control were re-run rather than quoted. Both hold exactly as written.

Fixture, rebuilt from scratch:

```
$ ./target/debug/agent-scaffold scaffold --output-dir "$SCRATCH" --write --force --principles default
          create  .agents/user-prompts/resume.md
          render  docs/plans/TEMPLATE.md
Wrote to /home/.../.scratch/fixture (30 changed, 0 left untouched).
$ ls "$SCRATCH/docs"
plans
```

30 files, no `docs/metrics/`, as the brief states.

Borrowed-slug mutation applied (`slug` to `triager-runs-only-on-findings`, `status` to `complete`, step sidecar copied). The FALSE PASS, at the pre-change build, run from the worktree root:

```
$ ./target/debug/agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow
stdout: docs/metrics/workflow.jsonl: 235 records, valid
stdout: /home/.../.scratch/fixture/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
stdout: /home/.../.scratch/fixture/docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
stderr: (empty)
exit: 0
```

235 records, matching the brief's count at `fe62ca1`. The stderr stream was captured separately and is empty, confirming the brief's correction that the only thing wrong here is the exit code plus the affirmative green.

THE CONTROL, from inside the fixture with an empty log of its own:

```
$ cd "$SCRATCH" && mkdir -p docs/metrics && : > docs/metrics/workflow.jsonl
$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped
exit: 1
```

The check is sound; the defect is purely which file it reads. Nothing in the brief was found to be wrong. One thing it does not say is added below as finding 6 (the Markdown-plan path), and one defect it does not name is added as finding 7 (the ledger leak).

## What I built

Three files changed, all in `src/`, all throwaway.

`src/plan/source.rs`:

- Two new `[meta]` fields on `Meta`: `metrics: Option<String>` and `ledger: Option<String>`, both `#[serde(default, skip_serializing_if = "Option::is_none")]`, placed among the scalar keys before the `sidecars` sub-table, following the existing `w4_baseline` pattern. Both are paths RELATIVE TO THE DIRECTORY HOLDING THE PLAN FILE.
- `DEFAULT_METRICS_REL = "../metrics/workflow.jsonl"` and `DEFAULT_LEDGER_REL_SUFFIX = ".ledger.md"`, the source-relative spellings of today's conventions.
- `is_declared_log_ref`, the validation predicate: non-empty and not absolute. It deliberately permits `..`, because the default itself needs one. Wired into `validate_source` alongside the existing sidecar-ref checks.
- `Meta::metrics_rel()` and `Meta::ledger_rel()` accessors.

`src/main.rs`:

- `--metrics` changed from `PathBuf` with a `default_value` to `Option<PathBuf>` on all three of `ValidateArgs`, `StatusArgs`, `NextArgs`, so "the user named a path" is representable rather than recovered (the brief's own note under candidate (a), Principle "Make illegal states unrepresentable").
- `resolve_metrics_path(cli, source_path, source_plan) -> Result<PathBuf, String>`. The rule: with no plan source there is nothing to anchor to, so an explicit `--metrics` or today's CWD default fires unchanged; with a plan source that parses, the declared `[meta].metrics` (or, under the anchor policy, the convention) is joined onto the plan file's directory; a source that is missing or unparseable falls back to the first case.
- `resolve_ledger_path(...)`, the same rule for `[meta].ledger`.
- `source_meta()`, which parses a `--source` for its `[meta]` regardless of `primary`. `status` and `next` must NOT route this through the existing `toml_source()`, which returns `None` for a Markdown-primary source: the log pairing is a property of the plan file, not of which substrate owns the status.
- `lexical_normalise` and `same_file`, needed so that `--metrics <the same file, spelled differently>` is not reported as a conflict.
- `run_validate` restructured so the `--source` is READ AND PARSED BEFORE the metrics log, because it now decides which log is read. Its summary and problem reporting stay in their original position, so stdout ordering is unchanged (confirmed: the full suite passes without touching a single test).
- Two measurement switches, so both open policy questions could be measured on one build rather than argued. `AS_ABSENT_FIELD=cwd` makes an ABSENT field keep today's CWD-relative default; unset (`anchor`) makes it resolve against the plan directory. `AS_CONFLICT=error` makes a declared-versus-flag disagreement a hard error; unset (`cli-wins`) lets the flag win. A production version picks one of each; these exist only so the choice is decided by evidence.

`src/plan.rs`: one re-export line.

## Measurement 1: the migration cost

The brief's stated cost for (d) is "a schema field, its validation, its render, and a migration story for every existing plan that omits the field". Measured, that list is wrong in one place and incomplete in another.

RENDER COST IS ZERO. `render` reads only `meta.title` (`src/plan/render.rs:296`) and `meta.sidecars` (`:167-169`); no other `[meta]` scalar reaches the generated Markdown. Adding the field to this repository's own plan and re-running `render --check`:

```
$ ./target/debug/agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
exit: 0
```

That is WITH `metrics = "../metrics/workflow.jsonl"` declared in `[meta]`. The plan was then restored and `git diff --stat docs/` printed nothing, confirming the round trip. There is no render story to write.

PACK TEMPLATE COST IS ONE LINE, AND BREAKS NOTHING. Adding the declaration to `pack/plan-template.plan.toml` and re-running the suite:

```
test result: ok. 373 passed; 0 failed; ...
test result: ok. 5 passed; 0 failed; ...
(five further integration binaries, all 0 failed)
$ ./target/debug/agent-scaffold scaffold --output-dir .scratch/fixture-tpl --write --force --principles default
Wrote to .scratch/fixture-tpl (30 changed, 0 left untouched).
$ grep -n metrics .scratch/fixture-tpl/docs/plans/TEMPLATE.plan.toml
16:metrics = "../metrics/workflow.jsonl"
```

The drift guard (`src/agents_md_drift.rs`) covers `AGENTS.md`, `.agents/AGENTS.reference.md`, and the role prompts, not the plan template, so no regeneration is owed for this change. The pack change was reverted afterwards; it is not in the reported diff.

EXISTING PLANS NEED NO EDIT, under the anchor policy. Measured, the field is absent everywhere it could be:

```
$ grep -c "^metrics = " docs/plans/agent-scaffold.plan.toml pack/plan-template.plan.toml
docs/plans/agent-scaffold.plan.toml:0
pack/plan-template.plan.toml:0
```

THE COST THE BRIEF DOES NOT NAME, and the one that actually matters: `Meta` carries `#[serde(deny_unknown_fields)]` (`src/plan/source.rs:102`), so the field is a HARD VERSION FENCE. A plan that declares it is `malformed` to any binary predating it. Demonstrated with an unknown `[meta]` key as the proxy for an older binary meeting a newer plan:

```
$ ./target/debug/agent-scaffold validate --source .scratch/unknown.plan.toml
/home/.../.scratch/unknown.plan.toml: malformed `<task>.plan.toml`: TOML parse error at line 16, column 1
   |
16 | metrics_v2 = "../metrics/workflow.jsonl"
   | ^^^^^^^^^^
unknown field `metrics_v2`, expected one of `title`, `w4_baseline`, `primary`, `metrics`, `ledger`, `orphan_tasks`, `render_sha256`, `sidecars`

exit: 1
```

So the field is not a free addition. A project that adopts the declaration cannot be validated by an older `agent-scaffold`, and there is no version negotiation in the schema to soften that.

A COSMETIC REGRESSION that a production version must fix. The anchored path is displayed unnormalised:

```
$ ./target/debug/agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/plans/../metrics/workflow.jsonl: 235 records, valid
docs/plans/agent-scaffold.plan.toml: 93 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/plans/../metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

`docs/metrics/workflow.jsonl` has become `docs/plans/../metrics/workflow.jsonl` in every message. No test asserts on it today (the suite is green), but 82 files in the tree name the literal path, of which the live surface is `README.md`, `justfile`, `AGENTS.md`, `.agents/AGENTS.reference.md`, `.agents/prompts/orchestrator.md`, `.agents/LEDGER.template.md`, `pack/AGENTS.md`, `pack/instrument.md`, `pack/LEDGER.template.md`, and `pack/prompts/orchestrator.md`. A production version must lexically normalise before printing so that surface stays true. This cost is IDENTICAL for candidate (a), so it is not a differentiator, but it is real and neither candidate's write-up mentions it.

Note that all of this applies to candidate (a) as well, minus the schema field and the version fence. The measured MARGINAL cost of (d) over (a) is: 62 lines of schema and validation (for two fields), one pack template line, and a permanent plan-format version fence.

## Measurement 2: does it actually solve the problem

Answered adversarially, as instructed. The answer is no, twice over.

### 2a. The declared field contributes nothing on the day it ships

The same borrowed-slug fixture, run under both absent-field policies on the same build:

```
$ ./target/debug/agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow
no metrics log at /home/.../.scratch/fixture/docs/plans/../metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
/home/.../.scratch/fixture/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
exit: 0

$ AS_ABSENT_FIELD=cwd ./target/debug/agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow
docs/metrics/workflow.jsonl: 235 records, valid
/home/.../.scratch/fixture/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
/home/.../.scratch/fixture/docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The first run is the anchored default: the tool stops reading this repository's log, stops printing `workflow invariants hold`, and reduces the case to defect A, which inc2 converts to a hard failure. That is the fix working. The second run is the declared-field-only build: the false pass survives unchanged, because the fixture declares no field and neither does any other plan.

Every line of the difference between those two runs is candidate (a). The plan file is byte-identical across them.

### 2b. The declared field reopens the false pass, in a worse form

The mandate asked for the copy-paste and bad-relative-path case to be constructed and run. Constructed: a fixture plan declaring a path that walks out of its own project into this repository's log.

```toml
[meta]
title = "<title>"
primary = "toml"
metrics = "../../../../docs/metrics/workflow.jsonl"
```

```
$ ./target/debug/agent-scaffold validate --source "$W/.scratch/fixture-borrowed/docs/plans/TEMPLATE.plan.toml" --workflow
/home/.../.scratch/fixture-borrowed/docs/plans/../../../../docs/metrics/workflow.jsonl: 235 records, valid
/home/.../.scratch/fixture-borrowed/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
/home/.../.scratch/fixture-borrowed/docs/plans/TEMPLATE.plan.toml vs /home/.../.scratch/fixture-borrowed/docs/plans/../../../../docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The false pass is fully reconstructed through the mechanism that was supposed to prevent it. The plan validates clean, so nothing warns. And it is now WORSE than the defect it replaces, in three specific ways.

- It no longer depends on the working directory. Run from inside the fixture, where today's defect would not fire at all:

```
$ cd "$W/.scratch/fixture-borrowed"
$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
docs/plans/../../../../docs/metrics/workflow.jsonl: 235 records, valid
docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
docs/plans/TEMPLATE.plan.toml vs docs/plans/../../../../docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

- It is committed. Today's false pass lives in the invocation and disappears when the user stands somewhere else. This one lives in a tracked file and reproduces on every machine and in CI.
- Validation cannot refuse it. The declared ref must permit `..`, because `../metrics/workflow.jsonl` IS the convention. `is_safe_sidecar_ref` forbids exactly the component this field requires, so the project's existing containment rule is unusable here and the strongest available rule, "not absolute", rejects nothing that matters. This is a property of the repository layout (plans and metrics are siblings, not nested), not of my implementation, and no amount of care in the validator changes it.

The copy-paste vector is the realistic one. Copy a plan between two projects at different depths and the declared path either dangles (harmless) or lands on the source project's log (a false pass). Nothing in the tool can tell those apart, because a plan legitimately declaring a shared log is indistinguishable from a plan accidentally naming a foreign one.

### 2c. What the mechanism would need in order to work

The reason (d) is attractive under Principle "Structured data first, project for humans" is that the pairing becomes data the plan owns. The finding here is that a PATH is the wrong data to own. A path is a pointer, and a pointer with no identity on the other end cannot be checked. The check that would actually work is project IDENTITY: an id in the plan, the same id on each round record, and a join that requires them to match. Then a wrong path is DETECTED (the records do not belong to this project) instead of silently obeyed, and the residual W3 slug-only join (`src/workflow.rs:448-449`) closes at the same time. That is the sidecar's own closing bullet, and it is a data-model change, not a path change.

## Measurement 3: the conflict case

Built both rules and ran both. Setup: a fixture whose plan declares `metrics = "../metrics/workflow.jsonl"` and which has its own empty log there, invoked with `--metrics docs/metrics/workflow.jsonl` (this repository's log) from this repository's root.

Precedence rule, flag wins:

```
$ ./target/debug/agent-scaffold validate --source "$W/.scratch/fixture-declared/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl --workflow
docs/metrics/workflow.jsonl: 235 records, valid
/home/.../fixture-declared/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
/home/.../fixture-declared/docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

Error rule, same invocation:

```
$ AS_CONFLICT=error ./target/debug/agent-scaffold validate --source "$W/.scratch/fixture-declared/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl --workflow
--metrics docs/metrics/workflow.jsonl disagrees with the plan's `[meta].metrics` (/home/.../fixture-declared/docs/plans/../metrics/workflow.jsonl); pass one or the other
exit: 2
```

Same file named a different way, which must NOT error:

```
$ AS_CONFLICT=error ./target/debug/agent-scaffold validate --source "$W/.scratch/fixture-declared/docs/plans/TEMPLATE.plan.toml" --metrics "$W/.scratch/fixture-declared/docs/metrics/workflow.jsonl" --workflow
/home/.../fixture-declared/docs/plans/TEMPLATE.plan.toml vs /home/.../fixture-declared/docs/plans/../metrics/workflow.jsonl: Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped
exit: 1
```

DECISION, with reasoning: THE CONFLICT MUST BE AN ERROR, and precedence is indefensible. The first run is the same false pass a third time, reached through the CLI flag; a precedence rule means the mechanism can always be talked out of its own answer, and the thing it can be talked into is the exact defect it exists to remove. The error rule costs one thing, the same-file comparison, and the third run shows why that is required rather than optional: without it, every legitimate explicit invocation that names the declared log by a different spelling would be rejected.

One scoping detail that made the suite pass untouched: the error fires only when the plan declares `[meta].metrics` EXPLICITLY, not when the anchored default merely differs from an explicit flag. `tests/validate_workflow_toml_source_needs_no_plan.rs:73` and `:103` and `:119` all pass `--metrics workflow.jsonl` against plans that declare nothing, and they stay green. Widening the error to cover the undeclared case would break them and would also break every legitimate "point this at a different log" invocation, so the narrow rule is the right one.

## Measurement 4: the absent-field case

Every plan in existence omits the field, including this repository's (measured above) and the one the scaffolder drops (measured above). Three policies are possible; two were built.

- ANCHOR (built, the default in my build): an absent field resolves against the plan directory by the convention. This is the only one of the three that both fixes the defect and honours Principle "Safe on existing projects": no existing plan changes, and the normal invocation from the plan's own project root is a no-op, verified below in measurement 6. It is also, precisely, candidate (a).
- CWD (built): an absent field keeps today's default. Safe on existing projects trivially, and fixes nothing, as measurement 2a shows.
- REQUIRE THE FIELD (not built): an absent field is an error. I did not build this, and I did not need to: since the field is absent from this repository's plan and from the scaffolded template, this policy makes `validate --workflow` fail for every project that exists on the day it ships, including this one. That is a direct violation of Principle "Safe on existing projects" with no compensating benefit, since the anchor policy gets the same correctness without it. I state this as reasoning from a measured fact (the field is absent everywhere), not as a run.

So the safe absent-field behaviour is the anchor policy, and the anchor policy is candidate (a). Candidate (d) is only safe on existing projects to the exact extent that it is candidate (a) underneath.

## Measurement 5: no regression on the correct case

From the worktree root, this repository's own plan, unchanged behaviour and its own log:

```
$ ./target/debug/agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/plans/../metrics/workflow.jsonl: 235 records, valid
docs/plans/agent-scaffold.plan.toml: 93 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/plans/../metrics/workflow.jsonl: workflow invariants hold
exit: 0

$ ./target/debug/agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
exit: 0
```

The same plan from an unrelated directory, which is the actual point of the change, now reads the right log instead of whatever is under the caller's feet:

```
$ cd .scratch
$ agent-scaffold validate --source /home/.../explore-metricspath-b/docs/plans/agent-scaffold.plan.toml --workflow
/home/.../docs/plans/../metrics/workflow.jsonl: 235 records, valid
/home/.../docs/plans/agent-scaffold.plan.toml: 93 steps, 69 questions, valid
/home/.../docs/plans/agent-scaffold.plan.toml vs /home/.../docs/plans/../metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The no-source paths are untouched, which matters because they are the ones with nothing to anchor to:

```
$ ./target/debug/agent-scaffold validate
docs/metrics/workflow.jsonl: 235 records, valid
exit: 0

$ cd .scratch && agent-scaffold validate
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
exit: 0
```

## Measurement 6: the coverage gap the brief does not name

Candidate (d) can only anchor when there is a TOML to read a declaration from. The Markdown-plan path has no TOML, so it is entirely untouched. Fixture re-rendered so its Markdown Roadmap genuinely carries the borrowed slug at `complete`, then run from this repository's root against the FIXED build:

```
$ grep -n "triager-runs-only-on-findings" "$SCRATCH/docs/plans/TEMPLATE.md"
45:| `triager-runs-only-on-findings` | complete |  |

$ ./target/debug/agent-scaffold validate --plan "$SCRATCH/docs/plans/TEMPLATE.md" --workflow
docs/metrics/workflow.jsonl: 235 records, valid
/home/.../.scratch/fixture/docs/plans/TEMPLATE.md: 1 steps, 0 open-questions items, valid
/home/.../.scratch/fixture/docs/plans/TEMPLATE.md vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The full false pass, surviving candidate (d) completely. This is not a gap in my build; it is inherent. A Markdown plan has no `[meta]` to declare anything in.

Candidate (a), as the brief words it, derives the root from "the `--source` (or `--plan`) path", so it covers BOTH substrates with one derivation. That is a straightforward architectural advantage of (a) over (d), and it points the other way from the brief's framing, which treats (a) as the cheap fix and (d) as the principled one. On coverage, (a) is the more complete mechanism and (d) is the partial one.

## Measurement 7: the siblings, and a defect not in the brief

`status` and `next` carry the identical defect and are fixed by the same resolution. Today's behaviour reproduced with `AS_ABSENT_FIELD=cwd`, which restores the old resolution exactly (with no declaration the code path is identical to the pre-change one):

```
$ AS_ABSENT_FIELD=cwd agent-scaffold status --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml"
plan: 1 steps (1 complete); 0 open-questions items
metrics: 235 records

$ agent-scaffold status --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml"
plan: 1 steps (1 complete); 0 open-questions items
metrics: no log found
```

`next` behaves the same way (`metrics: 235 records` becoming `metrics: no log found`). The brief calls these separable because they are best-effort projections, so a wrong path yields an empty projection rather than a false assertion. That reasoning is correct for the record count. It is NOT correct for the ledger.

A DEFECT NOT IN THE BRIEF, found by extending to `default_ledger_path`. The ledger path is `docs/plans/<task>.ledger.md` with `<task>` derived from the plan source FILENAME, so any foreign project whose plan file happens to be named `agent-scaffold.plan.toml` collides. Fixture renamed accordingly, run from this repository's root:

```
$ AS_ABSENT_FIELD=cwd agent-scaffold status --resume --source "$W/.scratch/fixture-ledger/docs/plans/agent-scaffold.plan.toml"
## RESUME STATE (compaction checkpoint, read this first)

PENDING FOLDS: none. The task-entry re-grounding and structured per-step provenance items once staged here have SHIPPED: ...

CURRENT TRANSIENT STATE (updated 2026-07-28; this is the LIVE anchor ...). SESSION RESUMED after a compaction; ... Resolved writer-isolation tier: 2, git worktree ...

IN FLIGHT (committed on branch `plan/decision-folder-currency`, worktree `.claude/worktrees/plan-decision-folder-currency`; NOT merged, NOT converged ...)
exit: 0
```

That is this repository's entire internal RESUME STATE block, including branch names, worktree paths, and in-flight review state, printed as the resume anchor for an unrelated project. `next` echoes the same block into the instruction it hands an agent. This is a different KIND of defect from the metrics one: not a wrong boolean, but foreign content injected verbatim into an agent's prompt, where the receiving agent has been told the block is authoritative and to read it first. On the same build with the ledger anchored:

```
$ agent-scaffold status --resume --source "$W/.scratch/fixture-ledger/docs/plans/agent-scaffold.plan.toml"
no ledger at /home/.../fixture-ledger/docs/plans/agent-scaffold.ledger.md; nothing to resume
exit: 0
```

The brief's "best-effort projections, so a wrong path yields an empty projection" reasoning should be narrowed: it holds for the record count and does not hold for `--resume`.

DOES A PLAN-DECLARED PATH GENERALISE TO THE LEDGER? Yes, and better than it does to the log, for one structural reason: the ledger IS beside the plan (`docs/plans/<task>.ledger.md`), so its source-relative default is a bare filename with no `..`, which means `is_safe_sidecar_ref` CAN be applied to it unchanged and the declared ref CAN be confined to the plan directory. The containment argument that fails for `[meta].metrics` succeeds for `[meta].ledger`. Both the anchored default and an explicit declaration were built and run:

```
$ agent-scaffold status --resume --source "$W/.scratch/fixture-ledger/docs/plans/agent-scaffold.plan.toml"   # with ledger = "my-ledger.md"
## RESUME STATE

fixture ledger, its own.
exit: 0
```

Does that make the mechanism more attractive or merely larger? Merely larger, for THIS step. The ledger's defect is fixed entirely by anchoring the default; the declaration adds a capability (a ledger somewhere other than beside the plan) that nobody has asked for. It is worth recording that if a declared path is ever wanted, the ledger is the field where it is defensible and the metrics log is the field where it is not, which is the opposite of the priority the brief's candidate (d) implies.

## Measurement 8: the full suite and lint

Run with `TMPDIR` outside any git repository (see the note below):

```
$ cargo test
test result: ok. 373 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.65s
=== CLIPPY EXIT: 0 ===
```

386 tests, zero failures, clippy clean. Clippy caught exactly one thing on the first pass (`needless_borrow` at `src/main.rs:882`, a leftover from the `run_validate` restructure), fixed before the run above.

A SEPARATE FINDING, worth passing on because it costs time. With `TMPDIR` set INSIDE a git repository, which is what the worktree-local `.scratch` directory is, three tests fail:

```
---- checks::tests::a_non_repo_target_with_runnable_checks_errors stdout ----
expected NotARepo, got Ok(Report { results: [CheckResult { name: "ok", kind: Lint, status: Passed }], config_present: true })
---- tests::init_plan_defaults_to_git_and_skips_inside_a_repo stdout ----
assertion `left == right` failed
  left: SkipExists
 right: Init
---- tests::install_precommit_hook_skips_a_non_repo stdout ----
assertion failed: reason.contains("not a git repository")
test result: FAILED. 370 passed; 3 failed
```

They are not related to this change: all three assert "this directory is NOT a git repository", and a temp directory inside the worktree is inside one. Establishing that was a run, not a reading: the same build with `TMPDIR` outside any repository gives 373 passed, 0 failed. The tests are correct and the environment was wrong, but anyone told to keep `TMPDIR` inside the worktree will hit this and should know it is spurious.

## The two in-scope additions

THE SE-3 DOCUMENTATION HALF. `pack/AGENTS.md:93` tells every scaffolded project that "the deterministic `validate --workflow` check, once built, is the backstop that the required reviewed rounds happened before a step is marked complete", with no qualification, while `pack/instrument.md` (which is only rendered under `--instrument`) is where the round log is described at all. A non-instrumented reader is therefore promised a backstop that cannot run for them. Under candidate (d) the documentation debt GROWS rather than shrinks: the two-tier caveat is still owed, and a second concept is added on top of it, namely that the plan file declares where its log lives and that this declaration is trusted. Under candidate (a) the only new sentence is that the log is resolved relative to the plan source, which is what a reader would have assumed anyway. I did not write either documentation change; I measured what each would owe.

THE SIBLING COMMANDS. Covered in measurement 7. Both `status` and `next` and both ledger call sites (`run_resume` and `run_next`) were extended and measured. The extension is the same function in all four places, so the marginal cost of covering the siblings is small under either candidate; what differs is that candidate (a) needs no schema field to do it and candidate (d) needs two.

## The honest size of the change

```
$ git diff --stat
 src/main.rs        | 234 ++++++++++++++++++++++++++++++++++++++++++++++-------
 src/plan.rs        |   1 +
 src/plan/source.rs |  62 ++++++++++++++
 3 files changed, 268 insertions(+), 29 deletions(-)
```

Breaking that down honestly. Of the 205 added lines in `src/main.rs`, 53 are comments or blank, and 63 are the measurement scaffolding that a production version would delete (the two policy switches, `lexical_normalise`, and `same_file`, though a production version keeping the conflict-as-error rule must keep the last two, about 40 lines). The mechanism proper is roughly 60 lines of `src/main.rs`, 62 of `src/plan/source.rs` for two fields, and one re-export.

What a production version adds ON TOP of this, estimated rather than measured:

- Tests. At minimum: a two-project red-then-green integration test for the false pass (the brief already says this needs a new file), a conflict-as-error test, a same-file-different-spelling test, an absent-field test, a declared-relative-path test, and unit tests for the new validation predicate. Call it one new integration file of 120 to 180 lines plus 4 to 6 unit tests in `src/plan/source.rs`. This is the largest single item and it is the same for candidate (a) minus the field-specific cases.
- Path normalisation before display, plus a sweep of the 10 live files that name the literal path. Small in lines, tedious in checking.
- Documentation: the `run_validate` doc comment, both flag help strings (already updated in the experiment), `README.md:210-224`, `CHANGELOG.md`, and the SE-3 caveat in `pack/AGENTS.md` with its two deployed copies regenerated so the drift guards stay green.
- Pack template plus a comment explaining the field, if the field ships at all: about 5 lines.
- The schema documentation for two new `[meta]` fields wherever the TOML schema is described for users.

Rough total for a production candidate (d): 400 to 500 lines across roughly 12 files. For candidate (a): 250 to 350 lines across roughly 10 files, with no schema field, no version fence, and coverage of the Markdown path that (d) does not have.

## What I did NOT verify

Stated explicitly, per Q-66.

- I did not build or run the "require the field" absent-field policy. The conclusion about it in measurement 4 is reasoning from a measured fact (the field is absent from every existing plan), not from a run.
- I did not run an actual older `agent-scaffold` binary against a plan declaring `[meta].metrics`. The version-fence result uses an unknown `[meta]` key on the current binary as a proxy for that; it establishes that `deny_unknown_fields` rejects the key, which is the mechanism, but it is a proxy and not the literal cross-version run.
- I did not test symlinked paths. `same_file` canonicalises when both files exist, so a symlinked log would compare equal, but I did not construct that case, and `lexical_normalise` explicitly does not resolve symlinks.
- I did not test Windows or any non-Unix path behaviour, nor a plan file passed with no parent component at all (a bare `plan.toml` in the current directory falls to `Path::new(".")` in my build, which I did not exercise).
- I did not run `nix fmt` or any formatter; the repository is not formatter-clean at HEAD and the instruction forbade it. The experiment code is hand-formatted to match the surrounding style and may not match what `rustfmt` would produce.
- I did not measure the audit or TUI surfaces, which may also read a CWD-relative path; I only inspected `validate`, `status`, `next`, and the two ledger call sites.
- I did not write the documentation changes described in "the two in-scope additions"; I measured what they would owe and stopped there.
- The `AS_ABSENT_FIELD=cwd` runs are asserted to reproduce today's behaviour exactly. That is true by construction (with no declaration the resolver returns the CLI value or the CWD default, which is what the pre-change code did), and it is corroborated by the pre-change run at the top of this record producing the identical output, but I did not diff the two builds' outputs mechanically on every command.

## TMPDIR discipline

`ls /tmp | wc -l` at the start: 106. At the end: 107. The single new entry is `/tmp/agent-scaffold-explore-a`, created by the parallel explore-a agent, not by this run. `TMPDIR` was set for every cargo invocation, first to `<worktree>/.scratch` and then, after the three git-repo-detection tests were found to fail under a repo-internal TMPDIR, to the session scratchpad at `/tmp/claude-1000/.../scratchpad/tmp`, which is nested and therefore adds nothing to the top-level count and is outside any git repository. All fixtures live under `<worktree>/.scratch`, which is untracked.

## RECOMMENDATION

Two separable questions, answered separately.

### (i) Is this the right mechanism?

NO, not for defect B. Ship candidate (a).

The case, in the order the evidence made it.

- It does not solve the problem it was proposed for. All of its false-pass-killing effect comes from anchoring the ABSENT-FIELD case to the plan directory, which is candidate (a) exactly. Held at today's default, the field alone changes nothing, because no plan declares it (measurement 2a, both runs on one build with a byte-identical plan file).
- Its distinctive contribution is a regression. A declared path reconstructs the same false pass, and the reconstructed one is worse in three measured respects: working-directory-independent, committed, and accepted by `validate --source` (measurement 2b). The mechanism ships a supported, validated way to write down the defect.
- The regression is not fixable by a better validator. The conventional log sits outside the plan directory, so the declared ref must permit `..`, which is the exact component `is_safe_sidecar_ref` exists to forbid. There is no containment rule available for this field. This is layout, not implementation.
- It covers less than (a) does. The Markdown-plan path has no `[meta]` and stays fully broken under (d) (measurement 6), while (a)'s root derivation covers both substrates from one rule.
- It costs a permanent plan-format version fence via `deny_unknown_fields` (measurement 1), which is a real price for a capability that, on the evidence above, is a liability.

The brief's argument for (d) is Principle "Structured data first, project for humans" and Principle "Prefer the cleaner long-term architecture over the smallest diff". I take both seriously and I think they point the other way here. "Structured data first" says the pairing should be DATA rather than a CLI convention, and I agree; what this build shows is that a PATH is not the data that principle wants, because a path is an unverifiable pointer. The data that satisfies the principle is project IDENTITY, present in both the plan and each round record and required to match, because that is checkable. And "prefer the cleaner long-term architecture" does not favour a mechanism that is measurably less complete than the alternative (the Markdown gap) and that reintroduces the defect it was chosen to remove.

One narrow thing is worth keeping from this build, and it is not the metrics field: `[meta].ledger`. The ledger sits beside the plan, so a declared ledger ref CAN be confined by the existing safe-ref rule, and the ledger defect found here (measurement 7) is the more serious of the two, since it injects one project's internal state into another project's agent prompt. Even so, that defect is closed entirely by anchoring the default, so the declaration is a capability question and not a correctness one, and I would not ship it now.

### (ii) Does it belong in THIS step or the queued validation-constraints step?

NEITHER, as a path field. The brief's suspicion that (d) belongs in the queued step is half right: it is right that (d) does not belong in this step, and it is wrong about what should go in the queued one.

- THIS STEP, inc1: candidate (a). Anchor the default to the root derived from the plan source, covering `--source` and `--plan`, and extend the same resolution to `status`, `next`, and both ledger call sites. The sibling extension should be IN, not deferred, because the ledger leak is a live defect of the same family and the fix is the same function called in four more places. The brief's stated reason for separating the siblings, that they are best-effort projections whose failure mode is an empty projection, does not survive measurement 7 and should be corrected in the step file.
- THE QUEUED VALIDATION-CONSTRAINTS STEP: project identity in the round record and in the plan, with W3 joining on it. That is what makes the pairing checkable rather than merely declared, and it closes the residual the brief's own last bullet names (W3 still joins on a bare slug, so a shared or copied log stays joinable no matter how the path is resolved). A `[meta].metrics` path field is not a prerequisite for it and would not help it; if a project ever genuinely needs a log outside the convention, `--metrics` already serves that and does so without committing the wrong answer to a tracked file.

The one thing I would carry forward from this exploration into the queued step is the negative result, stated plainly: declaring the log's LOCATION in the plan does not establish that the log BELONGS to the plan, and building the field first would make the identity work harder, because the identity check would then have to reconcile itself against a declared path that may disagree with it.
