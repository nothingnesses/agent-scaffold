# Work review, round 2, `workflow-enforcement-tier-inc1`, TRIAGE

Triager: a separate agent from both round-2 reviewers, from the implementer and from the planner. Worktree `.claude/worktrees/triage-inc1-r2`, branch `triage/inc1-r2`, at `f8f2e09`, the exact commit both reviewers read, so every citation below resolves against the same text.

Inputs: `...-workreview-r2-reviewer-residue.md` (fix verification over both round-1 lanes; ZERO findings; both producer disclosures ruled not defects) and `...-workreview-r2-reviewer-claims.md` (claims-versus-behaviour over an 81-claim inventory; four findings, `W2B-1` high, `W2B-2` medium, `W2B-3` medium, `W2B-4` low).

METHOD. Every citation was opened at the cited `file:line` and confirmed against the text. Every reproduction was RUN rather than accepted. Two binaries were built: the increment's own from this worktree, and a PRE-anchoring binary from `69c0525` exported with `git archive` into `/tmp/triage-r2-prev`, outside any git repository, so every regression question is settled against a real pre-change binary rather than by reading the diff. Fixtures were built at `/tmp/triage-r2-fix/tree`, confirmed outside any repository (`git -C /tmp/triage-r2-fix rev-parse --is-inside-work-tree` -> `fatal: not a git repository`), with `home` (3-record log carrying the converged `borrowed-step` round, a TOML-primary `p.plan.toml`, a MARKDOWN-primary `mdprimary.plan.toml`, a `HOME` ledger), `away` (1-record log with no evidence for that slug, a schema-valid Markdown plan `real.md` marking `borrowed-step` complete) and `away2` (5-record log, a TOML-primary plan at `in-progress`, an `AWAY2` ledger). Every log carries a distinct record count, so the printed count identifies the file.

CONTAMINATION TRAP CHECKED, not inherited. The suite was run with `TMPDIR=/tmp/triage-r2-scratch`, outside the repository: 373 + 5 + 1 + 1 + 9 + 3 + 1 + 2 = 395 passed, 0 failed. Before trusting it, `strings target/debug/deps/metrics_and_ledger_anchor_to_the_plan_source-bf4905c55850edca | grep target/debug/agent-scaffold` was confirmed to report this worktree's own path (`.../.claude/worktrees/triage-inc1-r2/target/debug/agent-scaffold`), not a stale tree's. The pre-change tree was exported with `git archive`, which never includes `target/`, so nothing was inherited from a compiled tree.

The residue lens returning clean was set aside entirely while reading the claims lens. Each finding below is judged on its own evidence, and no ruling was weighed against what it implies for the round arithmetic.

## Summary

| id | reviewer severity | triage severity | verdict | owning writer |
| --- | --- | --- | --- | --- |
| `W2B-1` | high | high (confirmed) | VALID, fix required | IMPLEMENTER (8 sites in `src/`) and PLANNER (sidecar record plus inc2 spec amendment) |
| `W2B-2` | medium | medium (confirmed) | VALID, fix required | IMPLEMENTER (`src/`, `tests/`, `CHANGELOG.md`) and PLANNER (sidecar) |
| `W2B-3` | medium | medium (confirmed) | VALID, fix required | IMPLEMENTER (`src/`) and PLANNER (sidecar record) |
| `W2B-4` | low | low (confirmed) | VALID, fix required | IMPLEMENTER (`tests/`) |

Nothing was dismissed and nothing was accepted as residual. The backstop re-check for a dismissed high or critical finding is therefore not triggered by this triage.

Both writer lanes can run in PARALLEL, as they did in round 1. The implementer's 14 edit sites are confined to `src/main.rs`, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs` and `CHANGELOG.md`; the planner's 3 edit sites are confined to `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` plus the mechanical re-render of `docs/plans/agent-scaffold.md`. The two sets share no file. NO ORDERING IS REQUIRED between the lanes.

A NOTE THAT APPLIES TO EVERY EDIT BELOW. Each old-string is quoted exactly as it stands in the file, including its existing line breaks, and several of them end mid-line (the doc comments at `src/main.rs:1165-1168` and `tests/...:370-372` continue into a following sentence that must be kept). Splicing the supplied replacement will re-wrap the surrounding comment lines. That re-wrapping is expected, is mechanical, and is NOT a defect; what must not change is any word outside the supplied strings.

## `W2B-1` (high): VALID, fix required

### The citations, all reproduced

Every cited site was opened and the quoted text confirmed verbatim.

- `src/main.rs:429`, `validate --metrics` help: "So the log a plan is checked against is the plan's own, not whichever log the current directory happens to hold." CONFIRMED.
- `src/main.rs:455`, `status --metrics` help: "So the count summarises the plan's own log, not whichever log the current directory happens to hold." CONFIRMED.
- `src/main.rs:479`, `next --metrics` help: "So the loop is projected from the plan's own round evidence, not from whichever log the current directory happens to hold." CONFIRMED.
- `src/main.rs:795-797`, `run_validate` doc: "so the log a plan is checked against belongs to that plan's project rather than to whichever directory the process happens to be run from." CONFIRMED (line 796 carries the middle of the sentence, as cited).
- `src/main.rs:828`, in-body comment: "Resolving from the plan rather than from the process working directory is what stops a plan being joined to an unrelated project's log." CONFIRMED.
- `src/main.rs:1074`, `run_status` doc: "so the count belongs to the projected plan's own project". CONFIRMED.
- `src/main.rs:1152-1154`, `METRICS_RELATIVE` doc: "so the log a plan is read against belongs to that plan's project rather than to the process working directory." CONFIRMED.
- `README.md:226`: "The round log is resolved FROM THE PLAN, not from the directory you happen to be standing in." CONFIRMED as text. See the citation corrections below for why I do not prescribe an edit there.
- `CHANGELOG.md:22`: the "Previously ... could report `workflow invariants hold` for a plan with no review evidence of its own" clause. CONFIRMED as text. Same caveat.

### The mechanism, confirmed at the code level

`resolve_metrics_path` (`src/main.rs:1212-1224`) anchors on `source.as_ref().or(plan.as_ref())`, unconditionally. The plan the `--workflow` check READS is chosen by a different rule: the four-arm match at `src/main.rs:968-1014` fires its TOML arm only when `source_plan.as_ref().filter(|source| source.is_toml_primary())` is `Some`, and otherwise fires the Markdown arm on `plan_contents`. When the `--source` parses cleanly but is NOT TOML-primary, and a readable `--plan` is given, the log is anchored to the `--source`'s project and the plan is read from the `--plan`'s.

### Reproduction 1: the false green, run and confirmed

All inputs real and schema-valid, run from `/tmp`, which is neither project.

Control, `--plan` alone, which is the correct red:

```
$ cd /tmp && agent-scaffold validate --plan /tmp/triage-r2-fix/tree/away/docs/plans/real.md --workflow
/tmp/triage-r2-fix/tree/away/docs/plans/real.md vs /tmp/triage-r2-fix/tree/away/docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped
exit=1
```

The same plan with a MARKDOWN-primary `--source` from the other project:

```
$ cd /tmp && agent-scaffold validate --source /tmp/triage-r2-fix/tree/home/docs/plans/mdprimary.plan.toml \
      --plan /tmp/triage-r2-fix/tree/away/docs/plans/real.md --workflow
/tmp/triage-r2-fix/tree/home/docs/metrics/workflow.jsonl: 3 records, valid
/tmp/triage-r2-fix/tree/home/docs/plans/mdprimary.plan.toml: 1 steps, 0 questions, valid
/tmp/triage-r2-fix/tree/away/docs/plans/real.md: 1 steps, 1 open-questions items, valid
/tmp/triage-r2-fix/tree/away/docs/plans/real.md vs /tmp/triage-r2-fix/tree/home/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

REPRODUCES EXACTLY AS DESCRIBED. `away`'s plan declared to hold against `home`'s log, at exit 0, for a step whose own project has no round record. The only difference from the reviewer's run is `1 open-questions items` rather than `0`, a property of my fixture's plan and immaterial.

The sibling commands diverge the same way without `--workflow`:

```
$ cd /tmp && agent-scaffold status --source .../home/docs/plans/mdprimary.plan.toml --plan .../away/docs/plans/real.md
plan: 1 steps (1 complete); 1 open-questions items
metrics: 3 records
$ cd /tmp && agent-scaffold status --plan .../away/docs/plans/real.md
plan: 1 steps (1 complete); 1 open-questions items
metrics: 1 records

$ cd /tmp && agent-scaffold next --source .../home/docs/plans/mdprimary.plan.toml --plan .../away/docs/plans/real.md
task: mdprimary
source: /tmp/triage-r2-fix/tree/away/docs/plans/real.md
metrics: 3 records
```

Same projected plan, two different counts; and on `next`, the `source:` line names `away`'s plan while the count is `home`'s. Both confirmed.

### Reproduction 2: the regression determination, with the pre-change binary's actual output

THE REVIEWER'S CONFIGURATION, CONFIRMED. Pre-anchoring binary, same command, from `/tmp`:

```
$ cd /tmp && /tmp/triage-r2-prev/target/debug/agent-scaffold validate \
      --source /tmp/triage-r2-fix/tree/home/docs/plans/mdprimary.plan.toml \
      --plan /tmp/triage-r2-fix/tree/away/docs/plans/real.md --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
/tmp/triage-r2-fix/tree/home/docs/plans/mdprimary.plan.toml: 1 steps, 0 questions, valid
/tmp/triage-r2-fix/tree/away/docs/plans/real.md: 1 steps, 1 open-questions items, valid
exit=0
```

So in THAT configuration a skip did become an affirmative false green. The reviewer's measurement is correct.

THE MEASUREMENT THE REVIEWER DID NOT MAKE, AND IT CHANGES HOW THE FINDING READS. I ran the pre-anchoring binary on the same divergent pair from `home`, a directory that HOLDS a log:

```
$ cd .../home && /tmp/triage-r2-prev/target/debug/agent-scaffold validate --source docs/plans/mdprimary.plan.toml \
      --plan /tmp/triage-r2-fix/tree/away/docs/plans/real.md --workflow
docs/metrics/workflow.jsonl: 3 records, valid
docs/plans/mdprimary.plan.toml: 1 steps, 0 questions, valid
/tmp/triage-r2-fix/tree/away/docs/plans/real.md: 1 steps, 1 open-questions items, valid
/tmp/triage-r2-fix/tree/away/docs/plans/real.md vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

and the POST-anchoring binary, same command, same directory, produces that output BYTE-FOR-BYTE.

MY REGRESSION DETERMINATION: PARTIAL, AND NARROW. The false green is PRE-EXISTING, not introduced by this increment: the pre-change binary produces it identically whenever the process working directory holds a log with the borrowed slug. What the increment changed is the TRIGGER CONDITION, from "the current directory holds a log" to "the `--source`'s project holds a log". In the one configuration where those two differ in the bad direction (run from a directory with no log), an announced skip became an affirmative false claim, and that is a genuine regression on the assertion surface. The EXIT CODE did not change in either configuration: 0 before and 0 after, so a CI gate reading exit status sees no difference at all. The increment neither introduced the defect class nor closed it for this input shape.

TWO FURTHER MEASUREMENTS ON THE SAME FINDING.

The reviewer's typo variant (`--source docs/plans/typo.plan.toml` naming a file that does not exist, plus a foreign `--plan`, run from `home`) is NOT a regression in any configuration. Pre-change and post-change outputs are byte-identical, both printing `workflow invariants hold` at exit 0. The reviewer did not claim it was a regression, but it sits under a heading that reads as regression evidence, so it is worth pinning: that sub-case is purely a pre-existing false green, and the increment leaves it exactly as it found it.

A third candidate route does NOT reach the false green. A `--source` that fails to parse plus a foreign `--plan` exits 1, because `validate_source` reports the malformed source as a problem:

```
$ cd /tmp && agent-scaffold validate --source .../home/docs/plans/broken.plan.toml --plan .../away/docs/plans/real.md --workflow
.../home/docs/plans/broken.plan.toml: malformed `<task>.plan.toml`: TOML parse error at line 1, column 6
exit=1
```

So the divergence needs a `--source` that parses cleanly and is Markdown-primary, or a `--source` that does not exist. That narrows the exposure and is recorded so a later reader does not over-generalise it.

### Is it in inc1's scope to fix, and does inc2 close it?

THE IMPLEMENTATION IS SPEC-CONFORMANT. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:274` fixes the anchor order in as many words: "With BOTH a `--source` and a `--plan`, the anchor follows the source-then-plan order `derive_task` already uses (`src/next.rs:997-999`)". The same bullet forbids a refusal in this increment: "NO new REFUSAL mechanism: any new non-zero exit comes from the pre-existing W3 check finally running against the right project". So the code does what inc1 was told to do, and both candidate behaviour fixes (re-anchoring, or refusing on divergence) are changes to what inc1 was told to do rather than corrections of a deviation from it.

INC2 AS SPECIFIED DOES NOT CLOSE IT. I verified the reviewer's reasoning against the sidecar's own text rather than accepting it. The predicate is specified at `:164`: "derive the plan source's root from its REAL (canonicalised) location, resolve the metrics path by absolutising and canonicalising its longest existing ancestor and re-appending the components below it ..., and push a problem when the resolved log is not under that root." Two readings of "the plan source" are grammatically available, and the sidecar's own text settles which one it means:

- `:158` defines the derivation as starting at "the source's parent directory", and `:274` fixes that source as the `--source`-then-`--plan` anchor. `:164`'s predicate is explicitly the CANONICAL twin of that same derivation ("from its REAL (canonicalised) location"), so its referent is the anchor.
- `:164`'s own exception clause is decisive: "Where the plan source itself cannot be canonicalised there is no root, so the predicate does not fire ..., which is the answer the no-anchor case above already gets." Under a checked-plan reading, a typo'd `--source` with a readable `--plan` WOULD have a root and the predicate WOULD fire, so the exception would not read as it does. It reads as it does only under the anchor reading.

Under the anchor reading, in the falsifying run the resolved log IS under the anchor's root (it is `home`'s own log under `home`), so the predicate does not fire and the false green survives inc2. For the typo variant the anchor cannot be canonicalised at all, and the same clause says explicitly that the predicate then does not fire. THE REVIEWER'S INC2 ANALYSIS IS CORRECT.

INC3 DOES NOT CLOSE IT EITHER. Inc3 converts the `_` catch-all at `src/main.rs:1009-1013` (metrics log missing) into a reported problem. In the falsifying run the log is PRESENT, so that arm never fires.

THE CONSEQUENCE THAT MAKES THIS A HIGH. `:111` states the step's required end property: "`validate --workflow` must never pair a plan source with a metrics log belonging to a different project and report success." As the three increments are specified today, NO increment of this step closes this pairing, so the step would ship believing an end property met that is measurably not met. That, and not inc1's code, is what earns the severity.

SEVERITY: HIGH, CONFIRMED. Not because inc1's mechanism is wrong (it is spec-conformant, and the false green it leaves standing is pre-existing), but because eight sites including three subcommands' rendered `--help` carry a measurably false affirmative claim about the increment's own defining property, and because the routing consequence is that the step's specified mechanism does not reach its own end property. It does not fall to medium: the round-1 precedent set medium for a false claim in ONE internal doc comment, and this is the same defect class multiplied across the user-facing surface and compounded by a specification gap. It does not rise above high: nothing is newly broken for a correct invocation, no exit code changed, and the underlying pairing defect is pre-existing rather than introduced.

### MINIMAL FIX AND SITE COUNT (`W2B-1`)

SITE COUNT: 8 code sites hand-edited (IMPLEMENTER) plus 2 sidecar edits and 1 regeneration (PLANNER). Grepped across `src/`, `tests/`, the three step sidecars (`workflow-enforcement-tier.md`, `test-tmpdir-repo-assumption.md`, `status-resume-ignores-json.md`), `docs/plans/agent-scaffold.plan.toml`, `README.md` and `CHANGELOG.md` for the literal phrasings (`plan's own`, `plans own`, `the plan's project`, `belongs to that plan`, `not whichever log the current directory`, `resolved FROM THE PLAN`) and, semantically, for any restatement that ties the LOG's project to the CHECKED plan's project.

WHY DELETION AND NOT NARROWING, stated plainly because this project's default is to narrow. Three INDEPENDENT measured cases now falsify an affirmative "the log is the plan's own" claim: the escaping-`..` source spelling (measured and recorded at round 1), the bare filename run from inside `docs/plans` (accepted cost (i)), and the divergent anchor measured here. Any narrowed affirmative form has to survive all three, and the first two are already recorded elsewhere as expected behaviour. Narrowing the sentence a fourth time is how this fix pass would manufacture round 3's finding. The MECHANICAL RULE is already stated in full at every one of these sites and is verified true in every case either reviewer or I have measured; deleting the consequence sentence leaves each site stating that rule and nothing falsifiable. FIX CLASS: 8 DELETIONS.

Exact edits, supplied so they are copied rather than composed.

1. `src/main.rs:429`. Delete the sentence, including its trailing space:

```
So the log a plan is checked against is the plan's own, not whichever log the current directory happens to hold. 
```

2. `src/main.rs:438`. A SEMANTIC TWIN NOT IN THE REVIEWER'S SITE LIST, carrying the same gloss on the `--workflow` help. Replace:

```
the round log comes from --metrics, which defaults to the plan's own log (see that flag's help for the rule)
```

with:

```
the round log comes from --metrics (see that flag's help for the rule)
```

3. `src/main.rs:455`. Delete, including its trailing space:

```
So the count summarises the plan's own log, not whichever log the current directory happens to hold. 
```

4. `src/main.rs:479`. Delete, including its trailing space:

```
So the loop is projected from the plan's own round evidence, not from whichever log the current directory happens to hold. 
```

5. `src/main.rs:794-797`. Replace:

```
/// The log is `--metrics` verbatim when given, else `docs/metrics/workflow.jsonl` under
/// the project root derived from the plan source (`resolve_metrics_path`), so the log a
/// plan is checked against belongs to that plan's project rather than to whichever
/// directory the process happens to be run from.
```

with:

```
/// The log is `--metrics` verbatim when given, else `docs/metrics/workflow.jsonl` under
/// the project root derived from the plan source (`resolve_metrics_path`).
```

6. `src/main.rs:826-829`. Replace:

```
	// The log to read: `--metrics` verbatim when given, else the plan source's own
	// `docs/metrics/workflow.jsonl` (see `resolve_metrics_path`). Resolving from the plan
	// rather than from the process working directory is what stops a plan being joined to
	// an unrelated project's log.
```

with:

```
	// The log to read: `--metrics` verbatim when given, else the plan source's own
	// `docs/metrics/workflow.jsonl` (see `resolve_metrics_path`).
```

7. `src/main.rs:1073-1077`. Replace:

```
/// The metrics log is resolved exactly as `validate` resolves it
/// (`resolve_metrics_path`), so the count belongs to the projected plan's own project;
/// with `--resume`, the ledger is resolved beside the plan source (`default_ledger_path`)
/// for the same reason. A projection read from the wrong project's files is not an empty
/// projection, it is a confident wrong one.
```

with:

```
/// The metrics log is resolved exactly as `validate` resolves it
/// (`resolve_metrics_path`); with `--resume`, the ledger is resolved beside the plan source
/// (`default_ledger_path`). A projection read from the wrong project's files is not an empty
/// projection, it is a confident wrong one.
```

8. `src/main.rs:1152-1154`. Replace:

```
/// The conventional round-log path relative to a project root. The defaulted `--metrics`
/// is this joined onto the root derived from the plan source, so the log a plan is read
/// against belongs to that plan's project rather than to the process working directory.
```

with:

```
/// The conventional round-log path relative to a project root. The defaulted `--metrics`
/// is this joined onto the root derived from the plan source.
```

9. PLANNER, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:164`. Append to the END of that paragraph, exactly these three sentences:

```
MEASURED AT WORK REVIEW, AND NOT CLOSED BY THIS PREDICATE AS WRITTEN: the anchor is `--source` first and `--plan` second, while the `--workflow` check reads a TOML-primary `--source` else the Markdown `--plan`, so a Markdown-primary `--source` in one project paired with a `--plan` in another is checked against the FIRST project's log and reports `workflow invariants hold` at exit 0 for a step with no round record of its own. The resolved log IS under the anchor's root, so a predicate rooted on the anchor does not fire, and a typo'd `--source` with a readable `--plan` reaches the same place and is excluded by this paragraph's own no-root clause. Closing it needs either the root to come from the plan the check READS, or a second condition on the anchor and the checked plan resolving to different roots; inc2 owes one of the two, because the END PROPERTY above is otherwise met by no increment of this step.
```

DO NOT choose between the two mechanisms in this edit. Which one inc2 builds is a design question with a live trade-off (re-anchoring changes the resolution rule that `derive_task` and `default_ledger_path` also follow, so it risks making the task name, the ledger and the log disagree; a divergence condition adds a second predicate beside the containment one), and this project puts that class of question to the human through the planner rather than settling it in a triage. RECORD the case, name what is owed, and let inc2's planning pass decide the mechanism.

10. PLANNER, `docs/plans/agent-scaffold.md`. Regenerate; do not hand-edit. `cargo run -- render docs/plans/agent-scaffold.plan.toml`, then commit the sidecar and the generated view together. `render --check` is acceptance check 1 and WILL go red if the sidecar is edited without a re-render.

### Citation and scope corrections inside this finding

Recorded because a later reader comparing the findings file with this ruling must be able to see what moved and why. The finding remains VALID at every point below.

- ADDED SITE: `src/main.rs:438` carries the same false gloss ("defaults to the plan's own log") and is NOT in the reviewer's site list. The reviewer logged it as `C5` NOTE, "inherits W2B-1", rather than as a fix site. It is a fix site.
- REMOVED SITE: `README.md:226`. The cited sentence, "The round log is resolved FROM THE PLAN, not from the directory you happen to be standing in", is LITERALLY TRUE: in the falsifying run the log is resolved from a plan, namely the `--source`. Every operative statement in that paragraph is verified true, including the worked example the reviewer itself passed at `C70` ("checks THEIR plan against THEIR log"), the rule statement, the explicit-verbatim guarantee, the no-anchor fallback, the textual-rule claim and the bare-filename consequence. Deleting a true sentence is not a fix. NO README EDIT IS PRESCRIBED.
- REMOVED SITE: `CHANGELOG.md:22`. The cited clause describes what happened PREVIOUSLY, and I verified it is exactly what the pre-change binary does (the run from `home` above). What is false there is the implicature of a `### Changed` entry, not the text. The entry is in `## [Unreleased]` and inc2 lands before release, so adding a "this case survives inc1" sentence now and removing it at inc2 is churn against a true sentence. NO CHANGELOG EDIT IS PRESCRIBED FOR THIS FINDING. (`CHANGELOG.md:22` IS edited under `W2B-2`, for a different clause of the same entry.)
- SEMANTIC TWIN FOUND AND RULED NO-EDIT: `src/main.rs:1316-1318`, the in-body comment in `run_next`, reads "the round evidence the loop is projected from MUST BE the plan's own, since `next`'s output is an instruction an agent acts on". That is NORMATIVE (a statement of the requirement) rather than DESCRIPTIVE (a claim that the requirement is met), so it is not falsified by the reproduction. It is recorded here so round 3 does not file it as a missed twin.

## `W2B-2` (medium): VALID, fix required

### Citations reproduced

- `src/main.rs:1165-1168`: "LEXICAL is a deliberate choice, not an omission. The derived path keeps the spelling the caller typed, so a relative `--source` yields a relative log path and the printed output on a correct run is byte-identical to what it was before anchoring; a canonicalising rule would turn every printed path absolute and machine-specific." CONFIRMED.
- `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:370-376`, the test doc: "a run made from the plan's own project root, which is the normal invocation and the only one the scaffolded guidance documents, is UNCHANGED, byte for byte." CONFIRMED (`:372` carries "documents, is UNCHANGED, byte for byte", as cited).
- `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:393`, the assertion message: "the correct case's output must be byte-identical to the pre-anchoring binary's". CONFIRMED.
- `CHANGELOG.md:22`: "A run made from the plan's own project root, the normal invocation, is unchanged and still prints the relative paths it always did". CONFIRMED.

### Reproduction

Three spellings of the same correct invocation, all from the plan's own project root, both binaries:

```
$ cd .../home && agent-scaffold validate --source docs/plans/p.plan.toml --workflow
NEW: docs/metrics/workflow.jsonl: 3 records, valid
     docs/plans/p.plan.toml: 1 steps, 0 questions, valid
     docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
OLD: (identical, all three lines)

$ cd .../home && agent-scaffold validate --source ./docs/plans/p.plan.toml --workflow
NEW: ./docs/metrics/workflow.jsonl: 3 records, valid
     ./docs/plans/p.plan.toml: 1 steps, 0 questions, valid
     ./docs/plans/p.plan.toml vs ./docs/metrics/workflow.jsonl: workflow invariants hold
OLD: docs/metrics/workflow.jsonl: 3 records, valid
     ./docs/plans/p.plan.toml: 1 steps, 0 questions, valid
     ./docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold

$ cd .../home && agent-scaffold validate --source /tmp/triage-r2-fix/tree/home/docs/plans/p.plan.toml --workflow
NEW: /tmp/triage-r2-fix/tree/home/docs/metrics/workflow.jsonl: 3 records, valid
     /tmp/triage-r2-fix/tree/home/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
     /tmp/triage-r2-fix/tree/home/docs/plans/p.plan.toml vs /tmp/triage-r2-fix/tree/home/docs/metrics/workflow.jsonl: workflow invariants hold
OLD: docs/metrics/workflow.jsonl: 3 records, valid
     /tmp/triage-r2-fix/tree/home/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
     /tmp/triage-r2-fix/tree/home/docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

REPRODUCES EXACTLY. The bare relative spelling is byte-identical; the `./` spelling changes lines 1 and 3; the absolute spelling changes lines 1 and 3 to absolute machine-specific paths. The `./` case falsifies the doc comment under its own most charitable reading, because `./docs/plans/p.plan.toml` IS "a relative `--source`" and it DOES "yield a relative log path", and the byte-identity promise attached to that premise still fails.

### Is the CHECK or the CLAIM the defect? The CLAIM.

Acceptance check 9 (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:316`) names the EXACT command, with the bare relative spelling, and adds only "a relative source must keep a relative printed path". I verified both halves: the named command is byte-identical, and every relative spelling including `./` does keep a relative printed path. CHECK 9 IS NARROWER THAN THE CLAIM AND IS TRUE. The earlier reviewer's `cmp` pass on it was correct and must not be reopened. The test at `:380` is likewise a correct pin of exactly the spelling check 9 names, and MUST STAY AS WRITTEN. The defect is the generalisation added downstream of the spec, in the doc comment, the test's own doc, the assertion message, the CHANGELOG, and (see below) the sidecar sentence those were derived from.

I also confirmed the adjacent claim at `tests/...:374-376`, "changing two of these three lines to absolute, machine-specific paths": exactly lines 1 and 3 change, two of three. TRUE, no edit.

SEVERITY: MEDIUM, CONFIRMED. It stays above low because one of the false statements is a user-facing release note asserting that a normal invocation is unchanged, and because this project's own agent instructions require absolute paths, so the absolute-source spelling is a routine invocation here rather than an exotic one. It does not reach high because the BEHAVIOUR is correct and intended (the derived path keeps the spelling the caller typed, verified at `C27`), the pin is correct, and the acceptance check is correct: nothing executable is wrong.

### MINIMAL FIX AND SITE COUNT (`W2B-2`)

SITE COUNT: 5 hand-edited (4 IMPLEMENTER, 1 PLANNER) plus 1 REGENERATED. Grepped `byte-identical`, `byte for byte`, `still prints the relative paths`, `unchanged and still prints` across `src/`, `tests/`, the three step sidecars, `docs/plans/agent-scaffold.plan.toml`, `README.md` and `CHANGELOG.md`; all other `byte-identical` hits in `src/` belong to the drift guard, the render pin, the manifest and the checks worktree and are unrelated. `README.md` makes no byte-identity claim anywhere.

FIX CLASS: 2 DELETIONS plus 3 NARROWINGS, plus 1 mechanical regeneration.

1. IMPLEMENTER, `src/main.rs:1165-1168`. DELETION. Replace:

```
/// LEXICAL is a deliberate choice, not an omission. The derived path keeps the spelling
/// the caller typed, so a relative `--source` yields a relative log path and the printed
/// output on a correct run is byte-identical to what it was before anchoring; a
/// canonicalising rule would turn every printed path absolute and machine-specific.
```

with:

```
/// LEXICAL is a deliberate choice, not an omission. The derived path keeps the spelling
/// the caller typed, so a relative `--source` yields a relative log path; a canonicalising
/// rule would turn every printed path absolute and machine-specific.
```

2. IMPLEMENTER, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:370-372`. NARROWING. Replace:

```
/// Acceptance check 9, the Safe on existing projects pin: a run made from the plan's own
/// project root, which is the normal invocation and the only one the scaffolded guidance
/// documents, is UNCHANGED, byte for byte.
```

with:

```
/// Acceptance check 9, the Safe on existing projects pin: a run made from the plan's own
/// project root with a BARE RELATIVE `--source`, which is the normal invocation and the
/// only one the scaffolded guidance documents, is UNCHANGED, byte for byte.
```

3. IMPLEMENTER, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:393`. NARROWING. Replace:

```
		"the correct case's output must be byte-identical to the pre-anchoring binary's"
```

with:

```
		"this spelling's output must be byte-identical to the pre-anchoring binary's"
```

4. IMPLEMENTER, `CHANGELOG.md:22`. NARROWING. Replace:

```
A run made from the plan's own project root, the normal invocation, is unchanged and still prints the relative paths it always did
```

with:

```
A run made from the plan's own project root with a bare relative `--source`, the normal invocation, is unchanged and still prints the relative paths it always did
```

5. PLANNER, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:166`. A SEMANTIC TWIN NOT IN THE REVIEWER'S SITE LIST, and the UPSTREAM SOURCE of the phrasing the four sites above inherited. DELETION. Replace:

```
The DEFAULT is lexical so the printed path stays relative and output on the correct case is byte-identical; the GUARD is canonical so it cannot be spoofed by a symlinked source.
```

with:

```
The DEFAULT is lexical so the printed path stays relative; the GUARD is canonical so it cannot be spoofed by a symlinked source.
```

Nothing is lost by the deletion: the very next sentence in that paragraph already states the cost precisely and truly ("every resolved path becomes absolute EVEN WHEN THE USER TYPED A RELATIVE SOURCE, so two of the three printed lines change on the no-regression case"), which is the claim the deleted clause was compressing.

6. PLANNER, `docs/plans/agent-scaffold.md:1561`. REGENERATED, not hand-edited. Same re-render as `W2B-1` item 10; one render covers all three sidecar edits.

DO NOT edit `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:316` (acceptance check 9). It is narrow and true, and narrowing it further would weaken a correct acceptance criterion.

## `W2B-3` (medium): VALID, fix required

### Citations reproduced

`src/main.rs:1282-1286`: "The round log and the ledger are resolved from the PLAN SOURCE, not from the process working directory (`resolve_metrics_path`, `default_ledger_path`). That matters more here than anywhere else: every field of the projected loop, including the instruction and the echoed resume block, is derived from those two files, and the output is consumed by an agent that acts on it." CONFIRMED verbatim.

`src/next.rs:881-882`: `let review_findings = findings_naming::review_findings_path(context.task, &facts.step);` and the `triage_findings` line beneath it. CONFIRMED at the cited lines. `findings_naming::join_dir` (`src/findings_naming.rs:52-55`) builds the path from `DIR_TEMPLATE` with `<task>` substituted, i.e. from the task name alone.

### Reproduction

```
$ cd .../home && agent-scaffold next --source /tmp/triage-r2-fix/tree/away2/docs/plans/p.plan.toml --isolation-tier worktree
task: p
source: /tmp/triage-r2-fix/tree/away2/docs/plans/p.plan.toml
metrics: 5 records
...
  isolation: worktree
  next: spawn a reviewer for the first review round
  role: reviewer
  prompt: .agents/prompts/reviewer.md
  context:
    isolation_tier: worktree
    ledger: /tmp/triage-r2-fix/tree/away2/docs/plans/p.ledger.md
    review_findings: docs/plans/p.reviews/borrowed-step-reviewer-<disambiguator>.md
    triage_findings: docs/plans/p.reviews/borrowed-step-triage.md
```

REPRODUCES EXACTLY. `ledger:` correctly anchored into `away2`; one line later `review_findings:` and `triage_findings:` are relative to `home`, the directory the process is standing in. The same emitted instruction tells an agent to read one project's ledger and to write its findings into another project's tree.

The claim is false more broadly than the reviewer states, which strengthens rather than weakens it: `isolation: worktree` comes from the `--isolation-tier` flag, `rounds: 0/5` from the built-in `WorkflowSpec`, `role`/`prompt` from constants, and `borrowed-step in progress` from the PLAN SOURCE, which is not one of "those two files" either. The reviewer's judgement that the report paths are the pair that MATTERS is correct: they are the only non-derived fields that are also cross-project inconsistent.

### Whose scope is the behaviour?

NOT INC1'S, and not any increment of this step. Inc1's declared scope (`:274`) is `--metrics` becoming `Option<PathBuf>`, the lexical derivation, `resolve_metrics_path`, `default_ledger_path` and their call sites, "plus the help strings and doc comments that describe resolution, and the red-then-green tests". Inc2 is the containment predicate and its consumers; inc3 is the tier policy and the `SE-3` documentation half. The report paths appear in none of the three, and the step's "Scope: what this step does not do" section (`:368-381`) does not mention them. So the BEHAVIOUR is out of scope for the step and the CLAIM is the defect, exactly as the reviewer suggests.

SEVERITY: MEDIUM, CONFIRMED. The claim is an exhaustiveness claim ("every field"), which this project has calibration data on as unusually easy to falsify, and its falsity is precisely what would stop a reader noticing a cross-project path in an instruction an agent acts on. It does not rise to high because nothing here is a regression, the behaviour predates the increment, and the increment neither introduced nor was asked to fix it.

### MINIMAL FIX AND SITE COUNT (`W2B-3`)

SITE COUNT: 1 code site hand-edited (IMPLEMENTER) plus 1 sidecar record (PLANNER, covered by the same re-render). Grepped `every field`, `those two files`, `from the durable files`, `derived from the round log` across `src/`, `tests/`, the three step sidecars, `docs/plans/agent-scaffold.plan.toml`, `README.md` and `CHANGELOG.md`. The other `from the durable files` hits (`src/next.rs:4`, `src/next.rs:522`, `src/main.rs:370`, `src/main.rs:1278`) name the inputs without claiming exhaustiveness over the output fields and are NOT twins. `review_findings`/`triage_findings` appear nowhere else in `src/main.rs`, `README.md`, `CHANGELOG.md` or the sidecar except the unrelated template comment at `src/main.rs:295-296`.

FIX CLASS: 1 DELETION plus 1 RECORDED CONSEQUENCE (1 bullet of new prose), plus the shared regeneration.

1. IMPLEMENTER, `src/main.rs:1282-1286`. DELETION. Replace:

```
/// The round log and the ledger are resolved from the PLAN SOURCE, not from the process
/// working directory (`resolve_metrics_path`, `default_ledger_path`). That matters more
/// here than anywhere else: every field of the projected loop, including the instruction
/// and the echoed resume block, is derived from those two files, and the output is
/// consumed by an agent that acts on it.
```

with:

```
/// The round log and the ledger are resolved from the PLAN SOURCE, not from the process
/// working directory (`resolve_metrics_path`, `default_ledger_path`). That matters more
/// here than anywhere else, because the output is consumed by an agent that acts on it.
```

2. PLANNER, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, appended as a new bullet at the END of the "Scope: what this step does not do" list. Exactly this bullet:

```
- It does not anchor the report paths `next` emits. `review_findings` and `triage_findings` are built from the task name alone (`src/findings_naming.rs:52-55`, via `src/next.rs:881-882`) and stay relative to the process working directory, so `next --source <a foreign plan>` emits one instruction whose `ledger:` is anchored into that project while its `review_findings:` is not. Measured at work review on inc1. It is a different rule from the metrics log and the ledger, it is not caused by this step, and no increment here changes it.
```

Whether that behaviour warrants its own backlog step is a plan-level judgement for the orchestrator and the planner, not a review ruling; recording it here is what the review owes.

## `W2B-4` (low): VALID, fix required

### Citations reproduced

`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:14-17`: "Every test builds several projects in one scratch tree and runs the binary from the WRONG one, so which file was read is identified by CONTENT rather than asserted from the path: each project's log carries a different record count, and only `home`'s log has a converged round for `borrowed-step`." CONFIRMED verbatim.

The file contains exactly 9 `#[test]` functions (`grep -c '^#\[test\]'` reports 9), at `:145`, `:190`, `:229`, `:269`, `:306`, `:344`, `:380`, `:408`, `:460`.

BOTH COUNTEREXAMPLES CONFIRMED, each read at its own lines rather than taken from the reviewer's grep:

- `the_correct_case_prints_the_same_relative_paths_it_always_did` (`:380`) builds ONE project (`let home = build_home(&root);` at `:382`, and there is no second `build_*` call in the function), runs from `home`, which IS the plan's own project, and asserts an exact whole-stdout comparison INCLUDING the paths (`:388-394`), which is the opposite of identifying the file by content.
- `a_bare_filename_from_inside_docs_plans_stays_a_silent_miss` (`:460`) builds ONE project (`let away = build_away(&root, "complete");` at `:462`), runs from `away/docs/plans`, inside that same project, and asserts from the path (`stderr.contains("no metrics log at docs/metrics/workflow.jsonl")` at `:469-472`).

The reviewer's two "further partial cases" also check out: `a_source_with_no_docs_plans_ancestor_falls_back_to_its_own_directory` (`:306`) runs its second invocation from `flat`, the plan's own root (`:324`), and `plain_validate_and_a_sourceless_run_keep_their_behaviour` (`:408`) runs two of its three invocations with no anchor at all (`:431`, `:440`).

Both single-project tests are CORRECT AS WRITTEN and must stay that way. The defect is the "Every test" generalisation, and its cost is that a reader would credit the byte-identity pin with a cross-project discrimination it does not carry.

SEVERITY: LOW, CONFIRMED. A false enumeration in an internal module doc, with no behavioural consequence and no other document depending on it.

### MINIMAL FIX AND SITE COUNT (`W2B-4`)

SITE COUNT: 1. Grepped `Every test`, `from the WRONG one`, `identified by CONTENT` across `src/`, `tests/`, the three step sidecars, `README.md` and `CHANGELOG.md`: the phrasing exists exactly once, at the cited lines. No semantic twin: no other module doc or sidecar sentence characterises this file's fixture strategy.

FIX CLASS: 1 NARROWING. Owned by the IMPLEMENTER.

`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:14-17`. Replace:

```
//! Every test builds several projects in one scratch tree and runs the binary from the
//! WRONG one, so which file was read is identified by CONTENT rather than asserted from
//! the path: each project's log carries a different record count, and only `home`'s log
//! has a converged round for `borrowed-step`.
```

with:

```
//! The cross-project tests build several projects in one scratch tree and run the binary
//! from the WRONG one, so which file was read is identified by CONTENT rather than
//! asserted from the path: each project's log carries a different record count, and only
//! `home`'s log has a converged round for `borrowed-step`.
```

## Measurements that differed from the reviewers'

Recorded so a later round can see what was re-measured rather than inherited.

- `W2B-1`, THE REGRESSION. The reviewer measured the pre-change binary only from `/tmp`, a directory with no log. Running it from `home`, a directory that holds one, produces the false green BYTE-FOR-BYTE identically to the post-change binary. The false green is therefore pre-existing and the regression is confined to the run-from-a-logless-directory configuration, with no change of exit code. The reviewer's stated measurement is correct; its characterisation as "a regression" without that second run overstates what changed.
- `W2B-1`, THE TYPO VARIANT. Measured on both binaries: byte-identical output, `workflow invariants hold` at exit 0 in both. Not a regression in any configuration.
- `W2B-1`, A THIRD ROUTE RULED OUT. An unparseable `--source` plus a foreign `--plan` exits 1 rather than green-passing, because `validate_source` reports the malformed source. The exposure needs a cleanly-parsing Markdown-primary `--source`, or a `--source` that does not exist.
- `W2B-1`, INC2 SCOPE. Verified the reviewer's reasoning against `:164`'s own exception clause rather than accepting it. The clause is only coherent under the anchor reading, which is what makes the predicate not fire. Confirmed. Also confirmed inc3 does not fire (the log is present), so the case survives the whole step as specified.
- SUITE AND CONTAMINATION. 395 tests green with `TMPDIR` outside the repository, and the test binary's embedded `CARGO_BIN_EXE_agent-scaffold` confirmed via `strings` to be this worktree's own before any figure was trusted.

Two immaterial defects in the findings file itself, noted rather than filed. Line 281 of `...-r2-reviewer-claims.md` opens the inventory with "77 claims" while line 17 and line 391 both give 81 (and 63 + 12 + 4 + 2 = 81); 77 is a stale figure. And the reviewer's `W2B-1` site list logs `src/main.rs:438` as a NOTE rather than as a site, which this triage corrects above. Neither affects a ruling.

## Round totals

- RAW findings across both lenses: 4 (0 from the residue lens, 4 from the claims lens).
- DEDUPLICATED: 4. The two lenses overlap nowhere, and no two of the four findings share a claim, a site or a fix.
- VALID: 4.
- ACCEPT RESIDUAL: 0.
- DISMISSED: 0.
- SEVERITY MIX OF THE VALID SET: 1 high (`W2B-1`), 2 medium (`W2B-2`, `W2B-3`), 1 low (`W2B-4`).
- FIX-CLASS BREAKDOWN: 11 DELETIONS, 4 NARROWINGS, 2 RECORDED CONSEQUENCES (about 185 words of new prose in total, all supplied verbatim above), 1 mechanical regeneration. NO CODE BEHAVIOUR CHANGE IS PRESCRIBED IN INC1.
- OWNERSHIP: 14 sites to the IMPLEMENTER (`src/main.rs` x 10, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs` x 3, `CHANGELOG.md` x 1). 3 sites to the PLANNER, all in `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, plus one re-render of `docs/plans/agent-scaffold.md` covering all three. The lanes share no file and may run in parallel with NO ORDERING between them.

### Routing recommendation

ROUND 2 IS NOT CLEAN. The consecutive-clean streak stays at 0 of the 2 that `risky` requires. Rounds 1 and 2 both carried findings, so rounds 3 and 4 must BOTH come back clean, against a total-round cap of 5. That leaves exactly one round of slack before the cap forces the human escalation, which is the reason to keep this fix pass strictly to the supplied text: a fix pass that composes rather than copies is the recorded way this artifact family spends its slack.

ROUTE: a two-lane fix pass in parallel (implementer and planner), then round 3 with two fresh reviewers.

ONE ITEM DOES NOT BELONG TO THE FIX PASS AND MUST NOT BE SILENTLY CARRIED. `W2B-1` establishes that the step's END PROPERTY at `:111` is met by no increment of this step as currently specified, and closing it requires a design choice between two mechanisms for inc2's predicate. The fix pass RECORDS the case (item 9 above) and states what inc2 owes; it does not choose. The mechanism choice should be put to the human by the planner before inc2 is built, not resolved inside a review round.
