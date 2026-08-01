# Work review, round 2, increment 1 of `workflow-enforcement-tier`: claims-versus-behaviour lens

Reviewer: independent claims reviewer (`W2B-`). Commit under review: `f8f2e09`. Worktree: `.claude/worktrees/wr2-inc1-b`, branch `wr2/inc1-b`.

## Lens and method

Round 1's three findings were all of one species: a claim in a comment or a doc that did not match the measured behaviour, with zero mechanism defects found by either lens. This round builds an inventory of every claim the increment makes about its own behaviour and checks each one by RUNNING it.

Method. The increment's binary was built from the worktree (`target/debug/agent-scaffold`). The PRE-anchoring binary was built separately from `git archive 69c0525` into `/tmp/wr2b-prev`, so every "before the change" and "byte-identical to the pre-anchoring binary" claim is checked against a real pre-change binary rather than by reading the diff. A fixture tree was built at `/tmp/wr2b-fix/tree`, outside any git repository (`git -C /tmp/wr2b-fix rev-parse --is-inside-work-tree` -> `fatal: not a git repository`), with `home` (3-record log holding the converged `borrowed-step` round), `away` (1-record log with no evidence for that slug), `flat` (conventionless root, 2 records), `outer`/`inner` (nested `docs/plans`, 6 and 4 records), `gitproj` (5 records, with and without a `.git`), and an `away/other` project (7 records) for the `..` cases. Every candidate log carries a distinct record count, so the printed count identifies which file was read.

Tense rule applied. Every claim below is checked as a claim about THIS tree, in which inc1 is built and inc2 and inc3 are not. Claims that are about inc2 or inc3 behaviour are marked NOT CHECKED (unbuilt) and are not findings. Where the tense mattered it is stated on the claim.

Suite state at `f8f2e09`: `TMPDIR=/tmp/wr2b-scratch cargo test` exits 0, four `test result: ok` lines, 9 tests in the new file.

## Verdict

NOT CLEAN. Four findings: one high, two medium, one low. Claim inventory: 81 claims extracted, of which 63 verified true, 12 falsified (the twelve statements that make up the four findings), 4 true with a qualification recorded, and 2 not checked (one counterfactual about a design not taken, one about unbuilt inc2).

The mechanism continues to hold up: every case in the derivation's own matrix (absolute, relative, `./`-prefixed, `..` below the root, `..` climbing out, conventionless fallback, nested nearest-wins, subdirectory under `docs/plans`, inside a repository, inside a nested repository, in an unpacked tarball) resolves exactly as documented, and all four "RED before the change" reproductions were confirmed against a real pre-change binary. What this round found is again in what the increment SAYS about itself, and once again the claims are true of the constructed example and false at an edge.

## Findings

| id | severity | claim site | one line |
| --- | --- | --- | --- |
| W2B-1 | high | `src/main.rs:429`, `:455`, `:479`, `:796`, `:1074`, `:1154`, `:828`, `CHANGELOG.md:22`, `README.md:226` | "the log a plan is checked against is the plan's own" is false when `--source` is not the plan the check reads; a measured run reproduces `workflow invariants hold` for a plan with no evidence of its own, and it is a regression against the pre-anchoring binary in that configuration |
| W2B-2 | medium | `src/main.rs:1166`, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:372` and `:393`, `CHANGELOG.md:22` | "the printed output on a correct run is byte-identical to what it was before anchoring" is false for a `./`-prefixed and for an absolute `--source`, both run from the plan's own project root |
| W2B-3 | medium | `src/main.rs:1284` | "every field of the projected loop ... is derived from those two files" is false: `review_findings` and `triage_findings` in the emitted instruction come from neither file and stay current-directory-relative |
| W2B-4 | low | `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:14` | "Every test builds several projects ... and runs the binary from the WRONG one" is false for 2 of the 9 tests, which build one project and run from the right one |

## W2B-1 (high): the "the plan's own log" consequence claim is false when the anchor and the checked plan diverge, and it costs a false green

### The claims

`src/main.rs:429` (the `validate --metrics` help, rendered identically by `agent-scaffold validate --help`):

> So the log a plan is checked against is the plan's own, not whichever log the current directory happens to hold.

`src/main.rs:796` (the `run_validate` doc comment):

> so the log a plan is checked against belongs to that plan's project rather than to whichever directory the process happens to be run from.

`src/main.rs:1154` (the `METRICS_RELATIVE` doc comment):

> so the log a plan is read against belongs to that plan's project rather than to the process working directory.

`src/main.rs:455` (the `status --metrics` help): "So the count summarises the plan's own log, not whichever log the current directory happens to hold." `src/main.rs:479` (the `next --metrics` help): "So the loop is projected from the plan's own round evidence, not from whichever log the current directory happens to hold." `src/main.rs:828` (an in-body comment in `run_validate`): "Resolving from the plan rather than from the process working directory is what stops a plan being joined to an unrelated project's log."

`README.md:226`:

> The round log is resolved FROM THE PLAN, not from the directory you happen to be standing in.

`CHANGELOG.md:22`:

> Previously both defaults were relative to the current directory, so pointing any of these commands at a plan in another project joined that plan to THIS directory's log and ledger: `validate --workflow` could report `workflow invariants hold` for a plan with no review evidence of its own [...]

### What the code does

`resolve_metrics_path` anchors on `--source` first and `--plan` second, unconditionally. The plan the `--workflow` check actually READS is chosen by a different rule: the TOML `--source` only when it is `[meta].primary = "toml"`, else the Markdown `--plan`. When those two rules select files in different projects, the log is anchored to one project and the plan is read from another, and the increment's consequence claim does not hold.

### The falsifying run

All inputs real and valid: `home/docs/plans/mdprimary.plan.toml` is a schema-valid MARKDOWN-primary source in `home` (3-record log holding a converged round for `borrowed-step`); `away/docs/plans/real.md` is a schema-valid Markdown plan in `away` marking `borrowed-step` `complete`, whose own project's log has no evidence for it. No typo, no missing file. Run from `/tmp`, which is neither project.

Control, the plan alone, which is the correct red:

```
$ cd /tmp && agent-scaffold validate --plan /tmp/wr2b-fix/tree/away/docs/plans/real.md --workflow
/tmp/wr2b-fix/tree/away/docs/plans/real.md vs /tmp/wr2b-fix/tree/away/docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` but has no round records and no covering waiver; [...]
exit=1
```

The same plan with a Markdown-primary `--source` from the other project also passed:

```
$ cd /tmp && agent-scaffold validate --source /tmp/wr2b-fix/tree/home/docs/plans/mdprimary.plan.toml \
      --plan /tmp/wr2b-fix/tree/away/docs/plans/real.md --workflow
/tmp/wr2b-fix/tree/home/docs/metrics/workflow.jsonl: 3 records, valid
/tmp/wr2b-fix/tree/home/docs/plans/mdprimary.plan.toml: 1 steps, 0 questions, valid
/tmp/wr2b-fix/tree/away/docs/plans/real.md: 1 steps, 0 open-questions items, valid
/tmp/wr2b-fix/tree/away/docs/plans/real.md vs /tmp/wr2b-fix/tree/home/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

That last line is `away`'s plan declared to hold against `home`'s log, at exit 0, for a step whose own project has no round record. It is the CHANGELOG's own description of the pre-change defect, produced by the post-change binary.

It is also a REGRESSION in this configuration. The pre-anchoring binary, same command, same directory:

```
$ cd /tmp && /tmp/wr2b-prev/target/debug/agent-scaffold validate \
      --source /tmp/wr2b-fix/tree/home/docs/plans/mdprimary.plan.toml \
      --plan /tmp/wr2b-fix/tree/away/docs/plans/real.md --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
/tmp/wr2b-fix/tree/home/docs/plans/mdprimary.plan.toml: 1 steps, 0 questions, valid
/tmp/wr2b-fix/tree/away/docs/plans/real.md: 1 steps, 0 open-questions items, valid
exit=0
```

Before anchoring the check was SKIPPED because no log was reachable from `/tmp`; after anchoring it reaches into `home` and returns a positive result. A skip became a false green.

`status` and `next` show the same divergence without needing `--workflow`:

```
$ cd /tmp && agent-scaffold status --source .../home/docs/plans/mdprimary.plan.toml --plan .../away/docs/plans/real.md
plan: 1 steps (1 complete); 0 open-questions items
metrics: 3 records
$ cd /tmp && agent-scaffold status --plan .../away/docs/plans/real.md
plan: 1 steps (1 complete); 0 open-questions items
metrics: 1 records
```

Same projected plan, two different counts. And on `next`, where the plan echoed on the `source:` line is `away`'s while the count is `home`'s:

```
$ cd /tmp && agent-scaffold next --source .../home/docs/plans/mdprimary.plan.toml --plan .../away/docs/plans/real.md
task: mdprimary
source: /tmp/wr2b-fix/tree/away/docs/plans/real.md
metrics: 3 records
```

A non-existent `--source` (a typo) reaches the same place, because a missing source is a stderr note and the run continues while the anchor still uses the typed path:

```
$ cd /tmp/wr2b-fix/tree/home && agent-scaffold validate --source docs/plans/typo.plan.toml \
      --plan /tmp/wr2b-fix/tree/away/docs/plans/real.md --workflow
no source plan at docs/plans/typo.plan.toml; nothing to validate
docs/metrics/workflow.jsonl: 3 records, valid
/tmp/wr2b-fix/tree/away/docs/plans/real.md: 1 steps, 0 open-questions items, valid
/tmp/wr2b-fix/tree/away/docs/plans/real.md vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

That one IS literally "whichever log the current directory happens to hold", joined to a foreign plan, which is the exact wording the help string denies.

### Why this is not inc2's absence

Inc2's containment predicate, as specified at `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:262` onward, derives the root from the PLAN SOURCE's canonicalised location and refuses when the resolved log is not under that root. In the falsifying run the resolved log IS under the `--source`'s root: it is `home`'s own log under `home`. The predicate would not fire. For the typo variant the source cannot be canonicalised at all, and the spec says explicitly that the predicate then does not fire. So this is not the known inc1-to-inc2 gap; it survives inc2 as specified.

This finding is about the claims, and the cheapest correct resolution may be to narrow them: state that the anchor is the `--source`/`--plan` path in that order and say nothing about it being the checked plan's project. The alternative, anchoring on the plan the check actually reads, is a behaviour change and belongs to the planner, not to me.

### Tense

Present tense, this tree. The claims are inc1's, inc1 is built, and the falsification runs against the built binary.

## W2B-2 (medium): "byte-identical to the pre-anchoring binary" is false for a `./`-prefixed and for an absolute `--source`

### The claims

`src/main.rs:1165-1168`:

> LEXICAL is a deliberate choice, not an omission. The derived path keeps the spelling the caller typed, so a relative `--source` yields a relative log path and the printed output on a correct run is byte-identical to what it was before anchoring; a canonicalising rule would turn every printed path absolute and machine-specific.

`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:370-373`:

> Acceptance check 9, the Safe on existing projects pin: a run made from the plan's own project root, which is the normal invocation and the only one the scaffolded guidance documents, is UNCHANGED, byte for byte.

and its assertion message at `:393`: "the correct case's output must be byte-identical to the pre-anchoring binary's".

`CHANGELOG.md:22`: "A run made from the plan's own project root, the normal invocation, is unchanged and still prints the relative paths it always did".

### The falsifying run

Three spellings of the SAME correct invocation, all run from the plan's own project root, post-anchoring binary:

```
$ cd .../home && agent-scaffold validate --source docs/plans/p.plan.toml --workflow
docs/metrics/workflow.jsonl: 3 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold

$ cd .../home && agent-scaffold validate --source ./docs/plans/p.plan.toml --workflow
./docs/metrics/workflow.jsonl: 3 records, valid
./docs/plans/p.plan.toml: 1 steps, 0 questions, valid
./docs/plans/p.plan.toml vs ./docs/metrics/workflow.jsonl: workflow invariants hold

$ cd .../home && agent-scaffold validate --source /tmp/wr2b-fix/tree/home/docs/plans/p.plan.toml --workflow
/tmp/wr2b-fix/tree/home/docs/metrics/workflow.jsonl: 3 records, valid
/tmp/wr2b-fix/tree/home/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
/tmp/wr2b-fix/tree/home/docs/plans/p.plan.toml vs /tmp/wr2b-fix/tree/home/docs/metrics/workflow.jsonl: workflow invariants hold
```

The pre-anchoring binary, same three commands:

```
$ cd .../home && /tmp/wr2b-prev/target/debug/agent-scaffold validate --source docs/plans/p.plan.toml --workflow
docs/metrics/workflow.jsonl: 3 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold

$ ... --source ./docs/plans/p.plan.toml --workflow
docs/metrics/workflow.jsonl: 3 records, valid
./docs/plans/p.plan.toml: 1 steps, 0 questions, valid
./docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold

$ ... --source /tmp/wr2b-fix/tree/home/docs/plans/p.plan.toml --workflow
docs/metrics/workflow.jsonl: 3 records, valid
/tmp/wr2b-fix/tree/home/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
/tmp/wr2b-fix/tree/home/docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

The first spelling is byte-identical, which is what the test at `:380` pins. The `./` spelling is NOT: two of the three lines changed, `docs/metrics/workflow.jsonl` becoming `./docs/metrics/workflow.jsonl`. This falsifies the doc comment under its own most charitable reading, because `./docs/plans/p.plan.toml` IS "a relative `--source`" and it DOES "yield a relative log path": the promise attached to that premise still fails.

The absolute spelling changed the same two lines to absolute, machine-specific paths, which is precisely the outcome the same sentence claims a canonicalising rule would cause and this rule avoids. This project's own agent instructions require absolute paths, so the absolute spelling is not an exotic case here.

The test at `:380` is a good pin and should stay; it is the doc comment, the test's own doc, and the CHANGELOG that generalise beyond what it pins. Note that the increment's spec, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:316` (acceptance check 9), states this narrowly enough to survive: it names the exact command and adds only "a relative source must keep a relative printed path", which is true. The generalisation was added downstream of the spec.

### Tense

Present tense, this tree, both binaries built and run.

## W2B-3 (medium): `next`'s "every field" claim is false, and the exception is an unanchored path in the instruction an agent acts on

### The claim

`src/main.rs:1282-1286`:

> The round log and the ledger are resolved from the PLAN SOURCE, not from the process working directory (`resolve_metrics_path`, `default_ledger_path`). That matters more here than anywhere else: every field of the projected loop, including the instruction and the echoed resume block, is derived from those two files, and the output is consumed by an agent that acts on it.

### The falsifying run

```
$ cd .../home && agent-scaffold next --source /tmp/wr2b-fix/tree/away2/docs/plans/p.plan.toml --isolation-tier worktree
[...]
  isolation: worktree
[...]
  context:
    isolation_tier: worktree
    ledger: /tmp/wr2b-fix/tree/away2/docs/plans/p.ledger.md
    review_findings: docs/plans/p.reviews/borrowed-step-reviewer-<disambiguator>.md
    triage_findings: docs/plans/p.reviews/borrowed-step-triage.md
```

Fields of the projected loop that are derived from neither the round log nor the ledger: `step` and `phase` (from the plan source), `isolation_tier` (from the `--isolation-tier` flag), `round_cap` (from the built-in workflow spec), `role` and `prompt` (constants), and `review_findings` and `triage_findings` (from `findings_naming::review_findings_path(context.task, ...)` at `src/next.rs:881-882`, which builds `docs/plans/<task>.reviews/...` from the task name alone).

The last pair is the one that matters rather than being pedantry. In the run above, `ledger:` correctly points into `away2`, and one line later `review_findings:` points at `docs/plans/p.reviews/...` relative to whatever directory the process is standing in, which here is `home`. The same emitted instruction therefore tells an agent to read a foreign project's ledger and to write its findings into THIS project's tree. That is the shape of the defect the increment exists to remove, still present in a sibling field, and the "every field" claim is exactly what would stop a reader noticing it.

Anchoring the report paths is arguably outside inc1's declared scope, which is the metrics log and the ledger, so the minimum correct action here may be to narrow the claim to the two fields it is actually true of and to record the report paths as still unanchored. That call is the planner's.

### Tense

Present tense, this tree. `next` is fully built at inc1 and the run is against the built binary.

## W2B-4 (low): the test module's "Every test" claim is false for 2 of the 9 tests

### The claim

`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:14-17`:

> Every test builds several projects in one scratch tree and runs the binary from the WRONG one, so which file was read is identified by CONTENT rather than asserted from the path: each project's log carries a different record count, and only `home`'s log has a converged round for `borrowed-step`.

### What the file does

`the_correct_case_prints_the_same_relative_paths_it_always_did` at `:380` builds ONE project (`let home = build_home(&root);` at `:382`, no second `build_*` call) and runs the binary from `home`, which is the plan's own project, not the wrong one. Its assertion at `:388-394` is an exact comparison of the whole stdout INCLUDING the paths, which is the opposite of identifying the file by content.

`a_bare_filename_from_inside_docs_plans_stays_a_silent_miss` at `:460` builds ONE project (`let away = build_away(&root, "complete");` at `:462`) and runs from `away/docs/plans`, inside that same project. Its assertion at `:469-472` is `stderr.contains("no metrics log at docs/metrics/workflow.jsonl")`, which is again asserted from the path.

The mapping of every test to its fixtures and its run directory:

```
$ grep -n "^fn \|build_home\|build_away\|run(&" tests/metrics_and_ledger_anchor_to_the_plan_source.rs
[...]
380:fn the_correct_case_prints_the_same_relative_paths_it_always_did() {
382:	let home = build_home(&root);
385:		run(&home, &["validate", "--source", "docs/plans/p.plan.toml", "--workflow"]);
[...]
460:fn a_bare_filename_from_inside_docs_plans_stays_a_silent_miss() {
462:	let away = build_away(&root, "complete");
466:		run(&plans_dir, &["validate", "--source", "p.plan.toml", "--workflow"]);
```

Two further partial cases: `a_source_with_no_docs_plans_ancestor_falls_back_to_its_own_directory` at `:306` runs its second invocation from `flat`, the plan's own root, and `plain_validate_and_a_sourceless_run_keep_their_behaviour` at `:408` runs two of its three invocations with no anchor at all, so there is no "wrong project" involved in them either.

Both single-project tests are correct as written and must stay that way: they are deliberately not cross-project. It is the "Every test" generalisation in the module doc that is wrong, and it would mislead a reader into believing the byte-identity pin carries a cross-project discrimination it does not.

### Tense

Present tense, this tree.

## Claim inventory

77 claims. `PASS` means checked and true, `FAIL` means falsified with the evidence shown above, `NOTE` means true with a qualification recorded, `N/C` means not checked and why.

### Clap help strings (checked against the RENDERED `--help`, not only the source)

| id | claim | site | test | result |
| --- | --- | --- | --- | --- |
| C1 | `validate --metrics`: "An explicit value is used verbatim" | `src/main.rs:429` | `status --source <away> --metrics docs/metrics/workflow.jsonl` from `home` -> `metrics: 3 records` (the explicit path, not the anchored one) | PASS |
| C2 | `validate --metrics`: the rule (nearest `<root>/docs/plans/` ancestor of `--source`, else `--plan`, else the source's own directory) | `:429` | eight resolutions measured: absolute, relative, `./`, `..`-below-root, `..`-out-of-root, conventionless fallback, nested nearest-wins, subdirectory under `docs/plans` | PASS |
| C3 | `validate --metrics`: "So the log a plan is checked against is the plan's own, not whichever log the current directory happens to hold" | `:429` | divergent-anchor run | FAIL (W2B-1) |
| C4 | `validate --metrics`: no-anchor keeps the current-directory-relative path | `:429` | `agent-scaffold validate` from `home` -> `docs/metrics/workflow.jsonl: 3 records, valid`, exit 0 | PASS |
| C5 | `--workflow`: "the round log comes from --metrics, which defaults to the plan's own log (see that flag's help for the rule)" | `:438` | pointer target is C2/C3; the pointer is accurate, the "plan's own" gloss inherits C3 | NOTE (inherits W2B-1) |
| C6 | `--workflow`: "Requesting --workflow with neither a TOML-primary --source nor a --plan is an error" | `:438` | `agent-scaffold validate --workflow` -> `--workflow requested but no plan source resolved: ...`, exit 1 | PASS |
| C7 | `status --metrics`: "An explicit value is used verbatim" | `:455` | as C1 | PASS |
| C8 | `status --metrics`: the rule | `:455` | as C2 | PASS |
| C9 | `status --metrics`: "So the count summarises the plan's own log, not whichever log the current directory happens to hold" | `:455` | divergent-anchor `status` run: same projected plan, 3 records vs 1 | FAIL (W2B-1) |
| C10 | `status --metrics`: no-anchor keeps the current-directory-relative path | `:455` | `agent-scaffold status` from `home` -> `metrics: 3 records` | PASS |
| C11 | `status --resume`: "from --ledger-fragment, or `<task>.ledger.md` beside the plan source" | `:461` | `status --resume --source <away>/docs/plans/p.plan.toml` from `home` -> `AWAY resume state.` | PASS |
| C12 | `status --resume`: "Exits 0 with a note when the ledger or the section is absent" | `:461` | ledger absent -> `no ledger at .../z.ledger.md; nothing to resume`, exit 0; section absent -> `.../q.ledger.md: no `## RESUME STATE` block found`, exit 0 | PASS |
| C13 | `status --ledger-fragment`: "Defaults to `<task>.ledger.md` BESIDE the plan source, where `<task>` is derived from that source's filename" | `:464` | as C11, plus `--plan <away>/docs/plans/p.md` -> `AWAY resume state.` (task `p` from `p.md`) | PASS |
| C14 | `status --ledger-fragment`: "no root derivation is involved" | `:464` | ledger resolves to the source's own directory in every case run, never to `<root>/docs/plans` | PASS |
| C15 | `status --ledger-fragment`: no-anchor keeps `docs/plans/<task>.ledger.md` relative to the current directory | `:464` | `agent-scaffold status --resume` from `home` -> `no ledger at docs/plans/task.ledger.md; nothing to resume`, exit 0 | PASS |
| C16 | `status --ledger-fragment`: "Requires --resume" | `:464` | `status --ledger-fragment docs/plans/p.ledger.md` -> clap `error: the following required arguments were not provided: --resume`, exit 2 | PASS |
| C17 | `status --ledger-fragment` explicit is used verbatim | `:464` | `status --resume --source <away> --ledger-fragment docs/plans/p.ledger.md` from `home` -> `HOME resume state.` | PASS |
| C18 | `next --metrics`: "An explicit value is used verbatim" | `:479` | `next --source <away2> --metrics docs/metrics/workflow.jsonl` from `home` -> `metrics: 3 records` | PASS |
| C19 | `next --metrics`: the rule | `:479` | as C2 | PASS |
| C20 | `next --metrics`: "So the loop is projected from the plan's own round evidence" | `:479` | divergent-anchor `next` run: `source:` names `away`'s plan, `metrics: 3 records` is `home`'s | FAIL (W2B-1) |
| C21 | `next --metrics`: no-anchor keeps the current-directory-relative path | `:479` | `agent-scaffold next` from `home` -> `metrics: 3 records`, exit 0 | PASS |
| C22 | `next --ledger-fragment`: beside-the-plan default, and no-anchor fallback | `:482` | `next --source <away2>` -> `AWAY2 resume state.`; explicit fragment -> `HOME resume state.` | PASS |
| C23 | The rendered help no longer advertises a stale `[default: docs/metrics/workflow.jsonl]` on any of the three subcommands | rendered | `validate --help`, `status --help`, `next --help`: no `[default:` on `--metrics` anywhere | PASS |

### Doc comments in `src/main.rs`

| id | claim | site | test | result |
| --- | --- | --- | --- | --- |
| C24 | `METRICS_RELATIVE`: "The defaulted `--metrics` is this joined onto the root derived from the plan source" | `:1152` | true of the anchored branch; the trailing "belongs to that plan's project" clause fails | FAIL (W2B-1) |
| C25 | `project_root_of_source`: start at the source's parent, first ancestor named `plans` whose parent is named `docs` wins, root is that ancestor's grandparent | `:1157-1163` | every resolution in C2 agrees with the described walk | PASS |
| C26 | `project_root_of_source`: "When no such ancestor exists the source's OWN directory is the root, so a plan sitting at a project root with no `docs/plans` still reads that root's log instead of being rejected" | `:1161` | `status --source <flat>/myplan.plan.toml` from `home` -> `metrics: 2 records`; from `flat` itself -> `metrics: 2 records` | PASS |
| C27 | "The derived path keeps the spelling the caller typed" | `:1165-1166` | relative in -> relative out, `./` in -> `./` out, absolute in -> absolute out | PASS |
| C28 | "so a relative `--source` yields a relative log path and the printed output on a correct run is byte-identical to what it was before anchoring" | `:1166-1167` | three spellings versus the pre-anchoring binary | FAIL (W2B-2) |
| C29 | "a canonicalising rule would turn every printed path absolute and machine-specific" | `:1167-1168` | counterfactual about a design not taken; consistent with the measured absolute-source output | N/C (counterfactual) |
| C30 | "a `..` component is skipped rather than followed (`Path::file_name` is `None` for it), so the match is against whatever `docs/plans` lies lexically above that `..`, which is the plan's own only when the `..` does not climb out through one" | `:1168-1171` | `.../away/docs/plans/sub/../p.plan.toml` -> `metrics: 1 records` (`away`'s own); `.../away/docs/plans/../../other/p.plan.toml` -> `metrics: 1 records` while `.../away/other/p.plan.toml` -> `metrics: 7 records` | PASS |
| C31 | "the rule never consults `.git`, so it behaves identically inside a nested repository, outside any repository, and in an unpacked tarball" | `:1172-1174` | same 5-record project read as `metrics: 5 records` in all three: `git init`ed, with a second `git init` at `docs/plans`, and as a `.git`-free tar copy; the whole fixture tree is outside any repository | PASS |
| C32 | "NEAREST-WINS on a nested `docs/plans` ... resolves to the INNER root ... is a JUDGEMENT, recorded as one" | `:1176-1179` | `status --source <outer>/docs/plans/vendor/docs/plans/inner.plan.toml` from `home` -> `metrics: 4 records` (inner), not 6 (outer) | PASS |
| C33 | inner comment: "`<root>` is empty for a relative `docs/plans/...`, which is what keeps the joined default equal to the historical `docs/metrics/workflow.jsonl`" | `:1187-1189` | bare relative source from `home` prints `docs/metrics/workflow.jsonl`, matching the pre-anchoring binary exactly | PASS |
| C34 | `resolve_metrics_path`: "`--source` first, then `--plan`, the same order `next::derive_task` resolves them in" | `:1201` | both use `source.as_ref().or(plan.as_ref())` (`src/main.rs:1220`, `src/next.rs:997`); `status --source <away> --plan docs/plans/p.md` from `home` -> `metrics: 1 records`, the source anchor | PASS |
| C35 | `resolve_metrics_path`: "With NEITHER a source nor a plan ... the historical current-directory-relative path stands unchanged" | `:1202-1203` | C4, C10, C21 | PASS |
| C36 | `resolve_metrics_path`: the `Option<PathBuf>`-with-default-applied-here rationale, "`None` is 'not supplied' by construction" | `:1205-1211` | field is `Option<PathBuf>` under a bare `#[arg(long)]`; no `value_source` call anywhere in `src/`; rendered help carries no default | PASS |
| C37 | `resolve_metrics_path`: "An explicit value is honoured verbatim, so a caller who names a path gets the file they named" | `:1210-1211` | C1, C7, C18 | PASS |
| C38 | `default_ledger_path`: "No root derivation and no upward walk, unlike the metrics log" | `:1230-1232` | code takes `anchor.parent()` only; measured ledger paths are always the source's own directory | PASS |
| C39 | `default_ledger_path`: "With NEITHER a `--source` nor a `--plan` ... the historical current-directory-relative `docs/plans/<task>.ledger.md` stands, the same case in which the metrics default keeps its own historical path" | `:1233-1236` | C15 plus C4 measured in the same run | PASS |
| C40 | `run_validate`: "The log is `--metrics` verbatim when given, else `docs/metrics/workflow.jsonl` under the project root derived from the plan source" | `:794-795` | C1, C2 | PASS |
| C41 | `run_validate`: "so the log a plan is checked against belongs to that plan's project rather than to whichever directory the process happens to be run from" | `:795-797` | divergent-anchor run | FAIL (W2B-1) |
| C42 | `run_validate`: "An absent file ... prints a note to stderr and is skipped rather than hard-failing" (pre-existing, re-checked because the anchored path changes WHICH file is absent) | `:799-802` | `validate --source <nolog>/docs/plans/p.plan.toml` from `home` -> `no metrics log at <nolog path>; nothing to validate`, exit 0, and no `3 records` anywhere | PASS |
| C43 | in-body comment: "Resolving from the plan rather than from the process working directory is what stops a plan being joined to an unrelated project's log" | `:828-829` | divergent-anchor run joins exactly such a pair | FAIL (W2B-1) |
| C44 | `run_status`: "The metrics log is resolved exactly as `validate` resolves it (`resolve_metrics_path`)" | `:1073-1074` | one function, identical argument triple at `:830`, `:1108`, `:1319`; measured counts agree across the three commands on every fixture | PASS |
| C45 | `run_status`: "so the count belongs to the projected plan's own project" | `:1074` | divergent-anchor `status` run | FAIL (W2B-1) |
| C46 | `run_status`: "with `--resume`, the ledger is resolved beside the plan source (`default_ledger_path`)" | `:1075-1076` | C11 | PASS |
| C47 | in-body comment: "single-sourced in `resolve_metrics_path` so the two commands cannot drift" | `:1106-1107` | true of `validate` and `status`, the two under discussion; `next` is a third caller of the same function, so the enumeration is local rather than wrong | NOTE |
| C48 | `run_resume`: "The ledger path is `--ledger-fragment` or the `<task>.ledger.md`-beside-the-plan-source default (with `<task>` derived from that source's filename). A missing ledger or absent section prints a note and exits 0" | `:1254-1258` | C11, C12, C13, C17 | PASS |
| C49 | `run_next`: "The round log and the ledger are resolved from the PLAN SOURCE, not from the process working directory" | `:1282-1283` | `next --source <away2>` from `home` -> `metrics: 1 records` and `AWAY2 resume state.` | PASS (the exception is C20/W2B-1) |
| C50 | `run_next`: "every field of the projected loop, including the instruction and the echoed resume block, is derived from those two files" | `:1284-1285` | `next --json` field-by-field | FAIL (W2B-3) |
| C51 | in-body comment: "The same anchored resolution `validate` and `status` use" | `:1316-1318` | same function, same argument triple | PASS |

### The test file

| id | claim | site | test | result |
| --- | --- | --- | --- | --- |
| C52 | module doc: "Before this increment `--metrics` carried a relative clap `default_value` and `default_ledger_path` built `docs/plans/<task>.ledger.md`, both of which resolve against the CWD" | `:5-7` | pre-anchoring binary reads `docs/metrics/workflow.jsonl` and `docs/plans/p.ledger.md` from the CWD on every foreign-source run | PASS |
| C53 | module doc: the four measured consequences | `:9-12` | all four reproduced on the pre-anchoring binary: `workflow invariants hold` at exit 0 for `away`'s evidence-free plan; `next` printing `state: converged`, `streak: 1/1`, `next: mark the step complete, re-render, and commit`; `metrics: 3 records` for a foreign plan; `HOME resume state.` printed for `away2` | PASS |
| C54 | module doc: "Every test builds several projects in one scratch tree and runs the binary from the WRONG one, so which file was read is identified by CONTENT rather than asserted from the path" | `:14-16` | fixture and run-directory map of all nine tests | FAIL (W2B-4) |
| C55 | module doc: "each project's log carries a different record count, and only `home`'s log has a converged round for `borrowed-step`" | `:16-17` | counts are 3 (`home`), 1 (`away`), 2 (`flat`), 6 (`outer`), 4 (`inner`), all distinct within each test; only `build_home` writes a `borrowed-step` round | PASS |
| C56 | module doc: "Several of the tests are pins rather than red-then-green cases, marked as such on each" | `:19` | tests at `:306`, `:344`, `:380`, `:408`, `:460` each carry an explicit pin marking in their own doc | PASS |
| C57 | test name `validate_workflow_reads_the_plans_own_log_not_the_working_directorys` and its "RED before the change" | `:141-145` | pre-anchoring binary: `docs/metrics/workflow.jsonl: 3 records, valid` plus `<away plan> vs docs/metrics/workflow.jsonl: workflow invariants hold` at exit 0, exactly as written | PASS |
| C58 | test at `:145` doc heading "Acceptance checks 3 and 4" | `:138` | the test implements check 4's shape (a fixture with its own evidence-free log, expecting the correct red). Check 3's shape (`--source <fixture> --workflow` with the fixture having NO log, expecting the stderr note naming the fixture's missing log and exit 0) is not asserted by this test or any other in the file; the nearest, at `:418`, omits `--workflow` and is check 10's first half | NOTE (partial coverage, not raised: the acceptance checks are the acceptance phase's to run) |
| C59 | test name `next_projects_the_loop_from_the_plans_own_log` and its RED | `:182-188` | pre-anchoring binary printed `metrics: 3 records`, `state: converged`, `streak: 1/1`, `next: mark the step complete, re-render, and commit`, exit 0 | PASS |
| C60 | test name `status_counts_the_plans_own_log_from_either_anchor` and its RED "`metrics: 3 records` on all three invocations" | `:223-228` | pre-anchoring binary printed `metrics: 3 records` on all three; post-anchoring prints `metrics: 1 records` on all three | PASS |
| C61 | test name `the_ledger_resolves_beside_the_plan_source` and its RED "both commands print `HOME resume state.`" | `:262-267` | pre-anchoring `status --resume` and `next` both printed `HOME resume state.` for the `away2` source | PASS |
| C62 | test name `a_source_with_no_docs_plans_ancestor_falls_back_to_its_own_directory` and its RED | `:297-304` | C26; pre-anchoring the from-elsewhere run printed `metrics: 3 records` | PASS |
| C63 | test name `a_nested_docs_plans_resolves_to_the_inner_project` and its RED | `:334-342` | C32; pre-anchoring printed `metrics: 3 records` | PASS |
| C64 | test doc at `:370-373`: "a run made from the plan's own project root ... is UNCHANGED, byte for byte", and the assertion message at `:393` | `:372`, `:393` | three spellings versus the pre-anchoring binary | FAIL (W2B-2) |
| C65 | test doc at `:374-376`: a canonicalising "improvement" would change "two of these three lines to absolute, machine-specific paths" | `:374-376` | the absolute-source spelling changes exactly lines 1 and 3, two of three, corroborating the count | PASS |
| C66 | test name `plain_validate_and_a_sourceless_run_keep_their_behaviour` and its RED "before the change this read this directory's three-record log and printed it as valid" | `:399-406` | pre-anchoring `validate --source <nolog plan>` printed `docs/metrics/workflow.jsonl: 3 records, valid` at exit 0 | PASS |
| C67 | test name `a_bare_filename_from_inside_docs_plans_stays_a_silent_miss` and "This is not a regression (the pre-change build was identically wrong here)" | `:450-458` | pre and post binaries produce byte-identical output for `cd away/docs/plans && validate --source p.plan.toml --workflow`: the two stderr notes, `p.plan.toml: 1 steps, 0 questions, valid`, exit 0 | PASS |

### README and CHANGELOG

| id | claim | site | test | result |
| --- | --- | --- | --- | --- |
| C68 | "The round log is resolved FROM THE PLAN, not from the directory you happen to be standing in" | `README.md:226` | divergent-anchor run | FAIL (W2B-1) |
| C69 | the rule statement, including "so a plan at a project root with no `docs/plans` still reads that root's log" | `README.md:226` | C2, C26 | PASS |
| C70 | "`agent-scaffold validate --source /elsewhere/docs/plans/their-task.plan.toml --workflow` checks THEIR plan against THEIR log" | `README.md:226` | `validate --source <away>/docs/plans/p.plan.toml --workflow` from `home` -> exit 1, the failure names `<away>`'s own log, W3's `borrowed-step` red, and no `3 records` on either stream | PASS |
| C71 | "`status`, `status --resume` and `next` resolve the same way, and the ledger those two read is `<task>.ledger.md` beside the plan source" | `README.md:226` | the ledger half is C11/C22. On the log half, `status --resume` returns from `run_resume` before any metrics resolution, so it resolves no log at all; nothing observable contradicts the sentence, but it reads as covering three commands where two resolve a log | NOTE |
| C72 | "An explicit `--metrics` (or `--ledger-fragment`) is used verbatim" | `README.md:226` | C1, C17 | PASS |
| C73 | "a run with neither `--source` nor `--plan` ... keeps the current-directory-relative `docs/metrics/workflow.jsonl`" | `README.md:226` | C4 | PASS |
| C74 | "The rule is textual: it never consults `.git`, so it works the same in a nested repository, outside a repository, and in an unpacked tarball" | `README.md:226` | C31 | PASS |
| C75 | "a bare filename run from inside `docs/plans` ... looks for `docs/metrics/workflow.jsonl` beneath `docs/plans` and reports that it found no log" | `README.md:226` | `cd away/docs/plans && validate --source p.plan.toml --workflow` -> `no metrics log at docs/metrics/workflow.jsonl; nothing to validate` and `--workflow has a plan source but the metrics log is missing; skipping the workflow check`, exit 0 | PASS |
| C76 | `README.md:213` "Validate the default metrics log (docs/metrics/workflow.jsonl)" above a bare `agent-scaffold validate`: a claim that was true before the increment and had to stay true after it | `README.md:213-214` | C4 | PASS |
| C77 | CHANGELOG: the rule statement, the four "Previously" consequences, "the round log joins a step by slug alone, so a borrowed slug was enough", "the derivation is textual and consults no VCS", "An explicit `--metrics` or `--ledger-fragment` is still used verbatim", "a run with neither `--source` nor `--plan` ... keeps the current-directory-relative paths" | `CHANGELOG.md:22` | C2, C53, C31, C1, C17, C4; the borrowed-slug mechanism confirmed by `home`'s `borrowed-step` round satisfying `away`'s plan on the pre-anchoring binary | PASS, EXCEPT the two clauses in W2B-1 ("could report `workflow invariants hold` ... Previously") and W2B-2 ("still prints the relative paths it always did") |

### Plan sidecar, the two lines the increment changed

Tense note: the sidecar carries claims about inc2 and inc3, which are unbuilt. Only the sentences this increment ADDED or CHANGED, and only their inc1-tense content, are checked.

| id | claim | site | test | result |
| --- | --- | --- | --- | --- |
| C78 | "with a `..` that stays below the project's own `docs/plans` ... reaches that project's `docs/plans` above it" | `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:162`, mirrored at `docs/plans/agent-scaffold.md:1557` | `status --source <away>/docs/plans/sub/../p.plan.toml` -> `metrics: 1 records`, `away`'s own | PASS |
| C79 | "a `..` that climbs OUT through a `docs/plans` matches THAT directory, so `<root>/docs/plans/../../other/p.plan.toml` and `<root>/other/p.plan.toml` are the same file read against two different logs" | same two lines | `metrics: 1 records` versus `metrics: 7 records` for the two spellings of the same file | PASS |
| C80 | "it is not a regression (before anchoring, both spellings read the current directory's log)" | same two lines | pre-anchoring binary printed `metrics: 3 records` for both spellings | PASS |
| C81 | "it is the canonical guard in inc2, whose root comes from the source's REAL location, that rejects it" | same two lines | inc2 is unbuilt | N/C (inc2 tense, not a finding) |

Note on the count: the inventory runs C1 to C81. 63 are PASS, 12 are FAIL (C3, C9, C20, C24, C28, C41, C43, C45, C50, C54, C64, C68, which are the twelve statements the four findings are made of), 4 are NOTE (C5, C47, C58, C71: true with a qualification recorded rather than raised), and 2 are N/C (C29, a counterfactual about a design not taken, and C81, which is inc2 tense and unbuilt). C77 bundles the CHANGELOG's remaining clauses and passes except for the two already counted under W2B-1 and W2B-2.

## What a clean round here would have rested on

Recording the negatives, since a clean round on a `risky` increment is half of convergence. The derivation itself was exercised across eleven distinct path shapes and three filesystem contexts and matched its documentation every time. All four pre-change reproductions in the module doc and all seven per-test "RED before the change" claims were confirmed against a binary built from `69c0525` rather than inferred from the diff. The `Option<PathBuf>` design rationale, the `requires = "resume"` constraint, the explicit-verbatim guarantee on both flags, the no-anchor fallbacks on all four surfaces, the exit codes, and the stderr note wording all hold as written. Nothing in the mechanism was found wanting; all four findings are about what the increment says, and three of the four are fixable by narrowing a sentence.
