# workflow-enforcement-tier inc1, work review round 3, reviewer (claims lens)

Artifact: `fe54995` (increment 1 of `workflow-enforcement-tier`), reviewed in worktree `.claude/worktrees/wr3-inc1-b` on branch `wr3/inc1-b`.
Lens: every claim this increment makes about its own behaviour, checked by running it.
Reviewer id prefix: `W3B-`.

## Verdict

NOT CLEAN. Five findings: one `medium`, four `low`. Zero mechanism defects.

The resolution code itself came through everything I threw at it. Every behavioural claim about what the anchoring DOES resolves correctly across every spelling I constructed (absolute, relative, `./`-prefixed, `..` above and below the matched `docs/plans`, a subdirectory under `docs/plans`, a conventionless root, a nested `docs/plans`, inside a nested git repository, outside any repository, and in a tarball-shaped tree with no `.git` anywhere), all nine of the increment's own tests are genuinely red against the pre-change binary built from `69c0525` where they claim to be and genuinely green here, and the whole suite passes with the test binary verified to be calling this worktree's `agent-scaffold`.

What did not come through is a small residue of DESCRIPTIONS that outrun the behaviour they describe. Three of the five findings are places the round 2 fix pass narrowed one instance of a claim and left its twin standing:

- `W3B-1` and `W3B-2` are the two halves of the Safe-on-existing-projects claim. Round 2 narrowed the CHANGELOG sentence to "with a bare relative `--source`" and narrowed the test's byte-identity claim to "this spelling's output", but the plan's own END PROPERTY still says a run from the plan's own project root must be unchanged "except for the symlinked-`docs/plans` layout", and the sentence round 2 halved still says "the printed path stays relative". Both are false for a `./`-prefixed or absolute `--source` typed from the plan's own project root, which I measured against the pre-change binary.
- `W3B-3` is the ledger twin of a qualifier the sibling test kept. The plan's acceptance check 7 says the ledger stops leaking ON THE DEFAULT PATH and check 14c owes the explicit-`--ledger-fragment` half to inc2; the test doc dropped the qualifier and claims the leak "can no longer" happen. It still can, on both readers.

`W3B-4` is the comment sitting directly above the resolution code, the same site round 2's finding came from: its `..` sentence mispredicts the resolved path for a spelling I ran. `W3B-5` is a documentation-content claim that greps to zero.

Tense applied: all five are false OF THIS TREE. None depends on inc2 or inc3 being unbuilt. `W3B-1` is a requirement sentence rather than a description, and I applied the tense rule to it explicitly: the printed-path change is inc1's own behaviour, the plan's own "LEXICAL/CANONICAL SPLIT MUST NOT BE COLLAPSED" paragraph forecloses any later increment restoring it, so it is not a claim awaiting an unbuilt increment.

I did not re-litigate the two settled behaviours. The `..` that escapes a `docs/plans` and matches it, and the divergent-anchor false green, are both confirmed present and both are OUT of my findings; `W3B-4` is about a different `..` case (one where the matched `docs/plans` lies BELOW the `..`) and is a description defect, not a mechanism one.

## Findings

| id | severity | site | one line |
| --- | --- | --- | --- |
| W3B-1 | medium | `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:111`, `docs/plans/agent-scaffold.md:1506` | The END PROPERTY's list of exceptions to "a run from the plan's own project root must be unchanged" names only the symlink layout; a `./`-prefixed or absolute `--source` from that same root also changed. |
| W3B-2 | low | `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:166`, `docs/plans/agent-scaffold.md:1561` | "The DEFAULT is lexical so the printed path stays relative" is the surviving half of the sentence round 2 halved, and it fails on the same input the deleted half did. |
| W3B-3 | low | `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:262-264` | "one project's `## RESUME STATE` block can no longer be printed as another project's resume anchor" drops acceptance check 7's "on the DEFAULT path"; with an explicit `--ledger-fragment` it still is, on both readers. |
| W3B-4 | low | `src/main.rs:1161-1164` | "the match is against whatever `docs/plans` lies lexically above that `..`" mispredicts the resolved path when the matched `docs/plans` lies below the `..`. |
| W3B-5 | low | `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:371-372` | "the only one the scaffolded guidance documents" describes a `--source` invocation; the scaffolded guidance contains zero occurrences of `--source`. |

## W3B-1 (medium): the END PROPERTY's exception list is incomplete, and it is the twin round 2 narrowed everywhere else

### The claim

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:111` (and identically in the rendered view at `docs/plans/agent-scaffold.md:1506`):

> A run made from the plan's own project root, which is the normal invocation and the only one the scaffolded guidance documents, must be unchanged (Safe on existing projects), except for the symlinked-`docs/plans` layout recorded below as accepted cost (ii).

The exception list has exactly one member: the symlinked `docs/plans` layout of accepted cost (ii). Accepted cost (i), the bare-filename miss, is explicitly NOT an exception to unchangedness (`:256` records it as "NOT a regression (the pre-fix build was identically wrong here)", which I confirmed below).

### Why this is the twin round 2 left

Round 2's fix pass narrowed the same claim in two other places and not here:

- CHANGELOG, `be2c897..fe54995`: "A run made from the plan's own project root, the normal invocation, is unchanged" became "A run made from the plan's own project root **with a bare relative `--source`**, the normal invocation, is unchanged".
- `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:370`, same commit range: "a run made from the plan's own project root, which is the normal invocation" became "a run made from the plan's own project root **with a BARE RELATIVE `--source`**".
- `f8f2e09..fe54995` on the sidecar deleted "and output on the correct case is byte-identical" from `:166`.

Three narrowings of one claim; the sentence they all derive from is unchanged.

### The falsifying run

Pre-change binary built from the increment's base commit into a tree outside the repository, so the comparison is against the actual pre-anchoring build:

```
git archive 69c0525 | tar -x -C /tmp/wr3b-old
cd /tmp/wr3b-old && CARGO_TARGET_DIR=/tmp/wr3b-old-target cargo build
```

Fixture `$B/away` is a conventional `<root>/docs/plans` project holding its own one-record log. Every run below is made FROM `$B/away`, that plan's own project root:

```
for sp in "docs/plans/p.plan.toml" "./docs/plans/p.plan.toml" "$B/away/docs/plans/p.plan.toml"; do
  (cd $B/away && $OLD validate --source "$sp" --workflow)
  (cd $B/away && $NEW validate --source "$sp" --workflow)
done
```

```
########## spelling: docs/plans/p.plan.toml
--- OLD ---
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` but has no round records and no covering waiver; ...
--- NEW ---
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` but has no round records and no covering waiver; ...
########## spelling: ./docs/plans/p.plan.toml
--- OLD ---
./docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` ...
--- NEW ---
./docs/plans/p.plan.toml vs ./docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` ...
########## spelling: /tmp/wr3b-fx/t1/away/docs/plans/p.plan.toml
--- OLD ---
/tmp/wr3b-fx/t1/away/docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` ...
--- NEW ---
/tmp/wr3b-fx/t1/away/docs/plans/p.plan.toml vs /tmp/wr3b-fx/t1/away/docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` ...
```

The bare relative spelling is byte-identical, as the narrowed claims say. The `./`-prefixed spelling and the absolute spelling are NOT: the printed metrics path changed from `docs/metrics/workflow.jsonl` to `./docs/metrics/workflow.jsonl` and to an absolute machine-specific path. Same file read, same exit code, changed output. Neither of those two runs involves a symlinked `docs/plans`, so neither is covered by the stated exception.

That the printed path is part of what "unchanged" means here is the plan's own position, not mine: accepted cost (ii) at `:258` rejects the canonicalising variant because it "turns every printed metrics path absolute, changing output on the correct case", and acceptance check 9 at `:316` requires the three stdout lines to be "BYTE-IDENTICAL to the pre-fix binary's".

### Why medium

The absolute spelling is the one a script or a CI job is most likely to use (`--source "$PWD/docs/plans/x.plan.toml"`), and the concern the plan itself raises for the canonicalising variant, "an absolute machine-specific path lands in output that a pre-commit hook or CI log may be matched against", lands on the LEXICAL default too for exactly this input. The record currently says that cost was avoided. It was narrowed, not avoided, and the plan is the surface the acceptance review and inc2's implementer will read. Acceptance check 9 is safe as written because it names its exact command; the END PROPERTY is what a reviewer checks Safe on existing projects against, and as written it will produce either a false pass or a false defect.

Note the fix is a narrowing of the sentence, not a change to the code: the lexical default is the decided mechanism (`:166` says the split "MUST NOT BE COLLAPSED"), so the exception list needs the case added, not the behaviour changed.

## W3B-2 (low): "the printed path stays relative" is the surviving half of a halved sentence and fails on the same input

### The claim

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:166` (and `docs/plans/agent-scaffold.md:1561`):

> THE LEXICAL/CANONICAL SPLIT IS DELIBERATE AND MUST NOT BE COLLAPSED. The DEFAULT is lexical so the printed path stays relative; the GUARD is canonical so it cannot be spoofed by a symlinked source.

Before round 2 this read "so the printed path stays relative and output on the correct case is byte-identical". The byte-identity conjunct was deleted at `f8f2e09..fe54995`. The two conjuncts fail on the same input, so deleting one left the other asserting the property the deletion was meant to remove.

### The falsifying run

From the same comparison above, the absolute-`--source` case:

```
(cd $B/away && $NEW validate --source /tmp/wr3b-fx/t1/away/docs/plans/p.plan.toml --workflow)
```

```
/tmp/wr3b-fx/t1/away/docs/plans/p.plan.toml vs /tmp/wr3b-fx/t1/away/docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` but has no round records ...
```

The printed default metrics path is `/tmp/wr3b-fx/t1/away/docs/metrics/workflow.jsonl`. It did not stay relative.

### The correct wording already exists in the code

`src/main.rs:1159-1160` says it accurately: "The derived path keeps the spelling the caller typed, so a relative `--source` yields a relative log path". The plan sentence is the loose twin of the accurate one, and the accurate one is the fix.

## W3B-3 (low): the ledger test drops acceptance check 7's "on the DEFAULT path" qualifier that its sibling test kept

### The claim

`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:262-264`:

> Acceptance check 7: the ledger resolves BESIDE the plan source, so one project's `## RESUME STATE` block can no longer be printed as another project's resume anchor. Both readers are covered, since `next` echoes the same block `status --resume` prints.

"can no longer" plus "both readers are covered" is a closure claim over the whole surface.

The acceptance check it cites does not make that claim. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:314`, check 7: "AFTER INC1, the ledger stops leaking **on the DEFAULT path**". Check 14c at `:323` owes the rest to inc2: "And for `status --resume` with an explicit `--ledger-fragment` naming a ledger outside the plan's root: a note naming the rejected path, no line of the block, exit 0." The sibling test at `:182` kept its equivalent qualifier ("on the default path", plus "THE EXPLICIT-`--metrics` CASE IS STILL OPEN HERE BY DESIGN"). This one did not.

### The falsifying run

`$B/home` holds a `HOME resume state.` ledger; the plan pointed at belongs to `$B/away`:

```
(cd $B/home && $NEW status --resume --source $B/away/docs/plans/p.plan.toml --ledger-fragment docs/plans/p.ledger.md)
```

```
## RESUME STATE

HOME resume state.
exit=0
```

```
(cd $B/home && $NEW next --source $B/away/docs/plans/p.plan.toml --ledger-fragment docs/plans/p.ledger.md | tail -4)
```

```
RESUME STATE (verbatim from the ledger):
## RESUME STATE

HOME resume state.
```

One project's `## RESUME STATE` block, printed as another project's resume anchor, on both readers. The behaviour is correct for inc1 (an explicit `--ledger-fragment` is documented as verbatim and the containment predicate is inc2's); the claim of closure is what is wrong, and the plan explicitly warns against it at `:286`: "An implementer must not read inc1's acceptance checks as evidence that defect C is closed."

## W3B-4 (low): the `..` sentence above the resolution code mispredicts the resolved path

### The claim

`src/main.rs:1161-1164`, in `project_root_of_source`'s doc comment:

> It also means a `..` component is skipped rather than followed (`Path::file_name` is `None` for it), so the match is against whatever `docs/plans` lies lexically above that `..`, which is the plan's own only when the `..` does not climb out through one.

Stated as an unconditional consequence for a path containing `..`: the match is against a `docs/plans` lying lexically ABOVE the `..`.

### The falsifying run

Two spellings of one file, where the `..` sits ABOVE the matched `docs/plans` rather than below it (`$B/away` holds a one-record log; `$B/home` holds a three-record log and is where the process runs):

```
(cd $B/home && $NEW status --source $B/away/docs/plans/p.plan.toml   | grep metrics)
(cd $B/home && $NEW status --source $B/away/other2/../docs/plans/p.plan.toml | grep metrics)
```

```
spelling A (plain):      metrics: 1 records
spelling B (other2/..):  metrics: 1 records
```

Spelling B contains a `..` and resolves correctly, but not by the stated mechanism. The walk starts at the parent `.../away/other2/../docs/plans`, which matches on its FIRST step; no `..` is ever skipped, and the matched `docs/plans` lies lexically BELOW the `..`, not above it. There is no `docs/plans` above the `..` at all, so the sentence as written predicts no match and hence the conventionless fallback to the source's own directory, which would have looked for:

```
ls /tmp/wr3b-fx/t1/away/other2/../docs/plans/docs
ls: cannot access '/tmp/wr3b-fx/t1/away/other2/../docs/plans/docs': No such file or directory
```

that is, `metrics: no log found`. The sentence therefore mispredicts the resolved path, not merely the route to it. The same over-reach shows on the plainer `--source ../../away/docs/plans/p.plan.toml` run from `$B/home/docs`, which also matches below both `..` components and also prints `metrics: 1 records`.

The operative clause ("which is the plan's own only when the `..` does not climb out through one") is correct, and the escaping case it warns about is the settled one I am not re-litigating. What is wrong is the universal mechanism clause in front of it. The accurate statement is narrower: a `..` the walk actually REACHES is skipped rather than followed, so the search continues above it.

## W3B-5 (low): "the only one the scaffolded guidance documents" describes an invocation the scaffolded guidance never shows

### The claim

`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:370-372`:

> Acceptance check 9, the Safe on existing projects pin: a run made from the plan's own project root with a BARE RELATIVE `--source`, which is the normal invocation and the only one the scaffolded guidance documents, is UNCHANGED, byte for byte.

Round 2 inserted "with a BARE RELATIVE `--source`" into the subject, so the relative clause now asserts that the scaffolded guidance documents a `--source` invocation, and only that one.

### The falsifying count

The scaffolded guidance is what `scaffold` writes: the `pack/` sources and this project's own rendered `AGENTS.md` and `.agents/`.

```
grep -ro -- "--source" pack/ | wc -l
0
grep -rc -- "--source" .agents/ | grep -v ":0"
(no output)
grep -c -- "--source" AGENTS.md
0
```

Zero occurrences in the entire pack, including `pack/AGENTS.md`, every role prompt under `pack/prompts/`, `pack/instrument.md` and the plan template. The only `agent-scaffold validate` invocation the guidance carries is bare:

```
grep -rn "agent-scaffold validate" pack/ AGENTS.md
pack/instrument.md:13:The log can be checked against this schema with `agent-scaffold validate`, ...
AGENTS.md:149:The log can be checked against this schema with `agent-scaffold validate`, ...
```

The substantive half of the claim is TRUE and I verified it (see the inventory, C.7): that spelling's three stdout lines are byte-identical to the pre-anchoring binary's. Only the parenthetical attribution is wrong.

For completeness on the alternative reading: `README.md:220` does document `agent-scaffold validate --source docs/plans/my-task.plan.toml --workflow`, so the claim would be true if "the scaffolded guidance" meant agent-scaffold's own README. It does not; the guidance is the thing scaffolded INTO a project, which is why the same phrase reads correctly at sidecar `:111` where the subject is "a run made from the plan's own project root" with no `--source` in it. Only the round-2 edited copy is affected.

## Claim inventory

Every claim I built, how I tested it, and the result. Negatives are listed in full: they are what a clean-round decision would have rested on. 118 claims across six surfaces. Round 2 built 81; mine is larger mostly because I enumerated every assertion message and every `--help` clause separately rather than by help string, and because I added the pre-change binary as an oracle for the eleven RED-before claims.

Shared setup for everything below. `$NEW` is `target/debug/agent-scaffold` in this worktree (verified by `strings` to contain only this worktree's path and no other worktree's). `$OLD` is the pre-change binary built from `69c0525` via `git archive` into `/tmp/wr3b-old` with `CARGO_TARGET_DIR=/tmp/wr3b-old-target`, so no contamination path exists between them. Fixtures live under `/tmp/wr3b-fx/t1`, outside any git repository: `home` (3-record log, its own plan and `HOME resume state.` ledger), `away` (1-record log, same task name `p`, `AWAY resume state.` ledger), `flat` (2-record log, plan at the root with no `docs/plans`), `outer`/`inner` (6 and 4 records, nested `docs/plans`), `nolog` (plan, no log), `noSect` (ledger with no `## RESUME STATE` block). Distinct record counts are what identify which file was read.

### A. Rendered `--help` (13 claims, 0 findings)

Tested by running `$NEW validate --help`, `$NEW status --help`, `$NEW next --help` and reading the rendered output, not the source strings.

1. `validate --metrics`: "An explicit value is used verbatim." TEST: `validate --metrics docs/metrics/workflow.jsonl --source <away plan>` from `home`. RESULT: TRUE, printed `docs/metrics/workflow.jsonl: 3 records, valid`, home's log, the named file.
2. `validate --metrics`: when omitted, the log is under the root derived from "the nearest `<root>/docs/plans/` ancestor of --source (else of --plan)". TEST: seven spellings (absolute, relative, `./`, `..` below, `..` above, subdirectory under `docs/plans`, `--plan` only). RESULT: TRUE in all seven.
3. `validate --metrics`: "or the source's own directory when it has no such ancestor". TEST: `status --source $B/flat/myplan.plan.toml` from `home`. RESULT: TRUE, `metrics: 2 records`, flat's own log.
4. `validate --metrics`: "With neither --source nor --plan ... the path stays `docs/metrics/workflow.jsonl` relative to the current directory." TEST: bare `validate` from `home`. RESULT: TRUE, `docs/metrics/workflow.jsonl: 3 records, valid`.
5. `validate --workflow`: "the round log comes from --metrics (see that flag's help for the rule)". TEST: read the `--metrics` help; it states the rule. RESULT: TRUE, the cross-reference resolves.
6. `validate --workflow`: "A TOML-primary --source needs no --plan". TEST: `validate --source docs/plans/p.plan.toml --workflow` from `away`. RESULT: TRUE, ran the check, exit 1 with W3's message.
7. `validate --workflow`: "the Markdown path still needs --plan present". TEST: `validate --source docs/plans/md.plan.toml --workflow` with `primary = "markdown"` and no `--plan`. RESULT: TRUE, refused.
8. `validate --workflow`: "Requesting --workflow with neither a TOML-primary --source nor a --plan is an error." TEST: bare `validate --workflow`. RESULT: TRUE, exit 1, `--workflow requested but no plan source resolved`.
9. `status --metrics`: same three clauses as 1, 2 and 4. TEST: `status --source`, `status --plan`, both, and bare. RESULT: TRUE (`1`, `1`, `1`, `3` records respectively).
10. `status --resume`: "from --ledger-fragment, or `<task>.ledger.md` beside the plan source". TEST: both routes. RESULT: TRUE.
11. `status --resume`: "Exits 0 with a note when the ledger or the section is absent." TEST: both halves separately, missing ledger and a ledger with no section. RESULT: TRUE, `no ledger at ...; nothing to resume` and `<path>: no `## RESUME STATE` block found`, exit 0 both.
12. `status --ledger-fragment`: "Requires --resume". TEST: `status --ledger-fragment <path>` with no `--resume`. RESULT: TRUE, clap error, exit 2.
13. `next --metrics` and `next --ledger-fragment`: same clauses as 1, 2, 4 and 10, and `next --ledger-fragment` correctly does NOT claim to require anything. TEST: ran each. RESULT: TRUE.

Also checked and TRUE: the sidecar's documentation-impact prediction that "the `[default:]` disappears from `--help`". No `[default:` appears anywhere in the three rendered outputs.

### B. `src/main.rs` doc comments and inline comments (28 claims, 2 findings)

14. `METRICS_RELATIVE`: "The conventional round-log path relative to a project root." TRUE by inspection and by every resolved path above.
15. `METRICS_RELATIVE`: "The defaulted `--metrics` is this joined onto the root derived from the plan source." TRUE where an anchor exists. In the no-anchor case no root is derived and the constant is used directly (`src/main.rs:1214`); the resulting path is identical, so nothing observable is wrong. NOT RAISED: the same headline shape appears in the CHANGELOG and the module doc, each carrying the carve-out elsewhere, and round 2 already narrowed this sentence once.
16. `project_root_of_source`: "Start at the source's parent and walk up". TRUE, `src/main.rs:1174-1175`.
17. `project_root_of_source`: "the first ancestor whose own file name is `plans` and whose parent's file name is `docs` ... and the root is that ancestor's grandparent". TRUE, `:1176-1186`, and confirmed by the nested case selecting the inner root.
18. `project_root_of_source`: "When no such ancestor exists the source's OWN directory is the root, so a plan sitting at a project root with no `docs/plans` still reads that root's log instead of being rejected." TRUE, `flat` reads `metrics: 2 records` both from elsewhere and from its own root.
19. `project_root_of_source`: "The derived path keeps the spelling the caller typed, so a relative `--source` yields a relative log path." TRUE, measured on the bare relative and `./` spellings.
20. `project_root_of_source`: "a canonicalising rule would turn every printed path absolute and machine-specific." Counterfactual about the rejected variant, attributed to explorer A. NOT TESTABLE HERE, accepted as attributed.
21. `project_root_of_source`: "a `..` component is skipped rather than followed (`Path::file_name` is `None` for it), so the match is against whatever `docs/plans` lies lexically above that `..`". FALSE as written. FINDING W3B-4.
22. `project_root_of_source`: "which is the plan's own only when the `..` does not climb out through one." TRUE, confirmed on both sides: `<root>/docs/plans/../../other/p.plan.toml` reads `<root>`'s log while `<root>/other/p.plan.toml` finds none, and a `..` that stays below resolves to the plan's own.
23. `project_root_of_source`: "the rule never consults `.git`, so it behaves identically inside a nested repository, outside any repository, and in an unpacked tarball." TRUE, measured all three: `metrics: 1 records` outside any repo, the same after `git init` in both the running and the target project, the same for a repo nested inside the target project reading its own inner log, and the same for a `.git`-free copy read from `/tmp`.
24. `project_root_of_source`: nearest-wins on a nested `docs/plans` "is a JUDGEMENT, recorded as one ... No measurement settled it". TRUE as a record: the behaviour is real (`metrics: 4 records`, inner, not 6, outer) and the sidecar records it as a judgement in the same terms.
25. Inline at `:1180-1182`: "`<root>` is empty for a relative `docs/plans/...`, which is what keeps the joined default equal to the historical `docs/metrics/workflow.jsonl`." TRUE for the spelling it names. NOT RAISED: a `./docs/plans/...` source yields root `.` and a `./`-prefixed path, but the comment names the literal `docs/plans/...` spelling.
26. `resolve_metrics_path`: "an explicit `--metrics` verbatim". TRUE on all three commands.
27. `resolve_metrics_path`: "`--source` first, then `--plan`, the same order `next::derive_task` resolves them in". TRUE, `src/next.rs:993-1003` is `source.as_ref().or(plan.as_ref())`, the same expression as `:1213`; and measured, `--source` wins when both are given.
28. `resolve_metrics_path`: "With NEITHER a source nor a plan there is nothing to pair a log with, so the historical current-directory-relative path stands unchanged." TRUE, measured on all three commands.
29. `resolve_metrics_path`: the `Option<PathBuf>` rationale, that `value_source` would give "a debug panic and a silent release-build misread". Attributed to explorer A's measured comparison; not re-measured. The design consequence IS visible: `None` is not supplied by construction, `:1210`.
30. `resolve_metrics_path`: "(Principle 13, make illegal states unrepresentable)". TRUE. `AGENTS.md:126` is "13. Make illegal states unrepresentable", and `src/plan/source.rs:14` already cites the same number for the same principle, so the citation matches in-tree precedent. NOT a finding despite the plan's own list numbering it 5, because the plan's list has 8 entries and cannot be what a "13" refers to.
31. `resolve_metrics_path`: "An explicit value is honoured verbatim, so a caller who names a path gets the file they named." TRUE, measured on `validate`, `status` and `next`, including `next --metrics $B/flat/...` printing `metrics: 2 records`.
32. `default_ledger_path`: "`<task>.ledger.md` BESIDE the plan source". TRUE, measured from `--source`, from `--plan` only, and from a bare-filename source (which correctly gives `p.ledger.md` in the current directory, not `docs/plans/p.ledger.md`).
33. `default_ledger_path`: "No root derivation and no upward walk". TRUE by inspection, `:1234-1237`.
34. `default_ledger_path`: "unlike the metrics log (which lives in a SIBLING `docs/metrics/`, so it needs the root to get there)". TRUE in substance, loosely worded (`metrics` is the sibling of `plans`, not `docs/metrics` of the source). Not raised.
35. `default_ledger_path`: "With NEITHER a `--source` nor a `--plan` there is no directory to sit beside, so the historical current-directory-relative `docs/plans/<task>.ledger.md` stands". TRUE, `status --resume` with no anchor prints `no ledger at docs/plans/task.ledger.md; nothing to resume`.
36. `default_ledger_path`: "the same case in which the metrics default keeps its own historical path". TRUE, both no-anchor fallbacks measured together.
37. `run_validate`: "The log is `--metrics` verbatim when given, else `docs/metrics/workflow.jsonl` under the project root derived from the plan source (`resolve_metrics_path`)." TRUE.
38. `run_validate`: "An absent file (the metrics log, or a `--plan` path) is not a validation failure." TRUE, anchored-but-missing log gives the stderr note and exit 0.
39. Inline at `:824-825`: "`--metrics` verbatim when given, else the plan source's own `docs/metrics/workflow.jsonl`". TRUE.
40. `run_status`: "The metrics log is resolved exactly as `validate` resolves it (`resolve_metrics_path`)". TRUE, both call sites pass the identical argument triple (`:826` and `:1103`).
41. `run_status`: "with `--resume`, the ledger is resolved beside the plan source (`default_ledger_path`)". TRUE.
42. `run_status`: "A projection read from the wrong project's files is not an empty projection, it is a confident wrong one." A rationale, and demonstrably the pre-change behaviour (`$OLD status --source <away plan>` printed `metrics: 3 records`, a confident wrong number). TRUE.
43. Inline at `:1101-1102`: "single-sourced in `resolve_metrics_path` so the two commands cannot drift". TRUE, one function, three call sites.
44. `run_resume`: "The ledger path is `--ledger-fragment` or the `<task>.ledger.md`-beside-the-plan-source default (with `<task>` derived from that source's filename)." TRUE.
45. `run_resume`: "A missing ledger or absent section prints a note and exits 0, since `status` is a best-effort projection, not a validator." TRUE, both halves measured, exit 0 both.
46. `run_next`: "The round log and the ledger are resolved from the PLAN SOURCE, not from the process working directory (`resolve_metrics_path`, `default_ledger_path`)." FALSE in the no-anchor case, measured: bare `next` from `home` prints `metrics: 3 records`, the current directory's log, and reads `docs/plans/task.ledger.md`. NOT RAISED: this is a headline summary of the shape used identically in the CHANGELOG, the README and the test module doc, each of which carves out the no-anchor case in the same paragraph, and the two functions it names carry the carve-out in their own docs. Round 2 edited this exact sentence and kept the head clause. I record it as a checked over-reach rather than a finding, since narrowing it would be a wording preference, not a correction of a misleading statement.
47. `run_next`: "That matters more here than anywhere else, because the output is consumed by an agent that acts on it." A rationale, consistent with the plan's ASYMMETRY paragraph. TRUE.
48. Inline at `:1307-1309`: "The same anchored resolution `validate` and `status` use: the round evidence the loop is projected from must be the plan's own". Normative ("must be") rather than a description of outcome, and the resolution IS the same. TRUE as written; noted as the nearest surviving relative of the "the plan's own log" claims round 2 deleted, but it does not assert what those asserted.

### C. Test file (51 claims, 2 findings)

Every test name, every doc comment and every assertion message. All nine tests were run in this worktree (`9 passed; 0 failed`) and the test binary was confirmed to bake `CARGO_BIN_EXE_agent-scaffold` to this worktree's path, so no stale-binary false green.

Module doc:

49. "the metrics log and the review ledger are resolved from the PLAN SOURCE rather than from the process working directory." TRUE as a headline; the file's own last test pins the no-anchor carve-out.
50. "Before this increment `--metrics` carried a relative clap `default_value` and `default_ledger_path` built `docs/plans/<task>.ledger.md`, both of which resolve against the CWD." TRUE, confirmed against `69c0525`'s source and against `$OLD`'s behaviour.
51. The four measured consequences (false `workflow invariants hold`, fabricated `mark the step complete`, wrong record count, leaked `## RESUME STATE`). TRUE, all four reproduced against `$OLD` in these exact fixtures. Count matches: four named, four tests.
52. "each project's log carries a different record count, and only `home`'s log has a converged round for `borrowed-step`." TRUE, home 3, away 1, flat 2, outer 6, inner 4, and only home's log names `borrowed-step`.
53. "Several of the tests are pins rather than red-then-green cases, marked as such on each." TRUE. Four are pins or contain a pinned half and each says so: `:378`, `:302-304`, `:405-406`, `:456-458`. NOT RAISED, but noted: round 1 added a third invocation to `plain_validate_and_a_sourceless_run_keep_their_behaviour` and the doc comment above it still enumerates two cases; the added case is a pin and its inline comment says "the historical ... stands", so it is marked, just not in the doc comment's own two-item split.

Test names (each a claim):

54. `validate_workflow_reads_the_plans_own_log_not_the_working_directorys`. TRUE for what the test constructs. The general form is false in the settled divergent-anchor case; excluded from findings by scope, and the sidecar records the gap at `:164`.
55. `next_projects_the_loop_from_the_plans_own_log`. Same. TRUE for the constructed case.
56. `status_counts_the_plans_own_log_from_either_anchor`. TRUE, both anchors measured.
57. `the_ledger_resolves_beside_the_plan_source`. TRUE.
58. `a_source_with_no_docs_plans_ancestor_falls_back_to_its_own_directory`. TRUE.
59. `a_nested_docs_plans_resolves_to_the_inner_project`. TRUE, 4 records not 6.
60. `the_correct_case_prints_the_same_relative_paths_it_always_did`. TRUE for the bare relative spelling the test runs; see W3B-1 for the spellings it does not run.
61. `plain_validate_and_a_sourceless_run_keep_their_behaviour`. TRUE.
62. `a_bare_filename_from_inside_docs_plans_stays_a_silent_miss`. TRUE.

Test doc comments:

63. Test 1: "RED before the change: `docs/metrics/workflow.jsonl: 3 records, valid` plus `<away plan> vs docs/metrics/workflow.jsonl: workflow invariants hold` at exit 0". TRUE, `$OLD` produced exactly those two lines at exit 0.
64. Test 1: "for a plan whose project has no review evidence for the step it marks `complete`". TRUE, away's log holds one `unrelated-step` record.
65. Test 2: "`next` no longer fabricates an instruction from a foreign log on the default path." TRUE, and correctly qualified "on the default path".
66. Test 2: "The step is at `in-progress` so the loop is derived from round records, which is the case that fabricates." TRUE.
67. Test 2: "RED before the change: `metrics: 3 records`, `state: converged`, `streak: 1/1`, and `next: mark the step complete, re-render, and commit`, at exit 0". TRUE, `$OLD` printed all four verbatim.
68. Test 3: "`status` counts the plan's own log, whether the anchor comes from `--source` or from `--plan`, and `--source` wins when both are given (the source-then-plan order `next::derive_task` already uses)." TRUE, all three measured.
69. Test 3: "RED before the change: `metrics: 3 records` on all three invocations." TRUE, `$OLD` printed `metrics: 3 records` three times.
70. Test 4: "the ledger resolves BESIDE the plan source, so one project's `## RESUME STATE` block can no longer be printed as another project's resume anchor." FALSE as a closure claim. FINDING W3B-3.
71. Test 4: "Both readers are covered, since `next` echoes the same block `status --resume` prints." TRUE that both read the same path; false as a closure claim, same finding.
72. Test 4: "RED before the change: both commands print `HOME resume state.`". TRUE, `$OLD` printed it on both.
73. Test 5: "a plan at a project root with NO `docs/plans` directory falls back to the source's own directory as the root, reading that root's own `docs/metrics/workflow.jsonl`, both from elsewhere and from that root itself." TRUE, 2 records both ways.
74. Test 5: "RED before the change on the from-elsewhere run (`metrics: 3 records`)." TRUE. "The from-its-own-root run is a pin." TRUE, `$OLD` and `$NEW` both print 2 records from `flat`.
75. Test 6: "a project vendored under another project's plan directory resolves NEAREST-WINS to the inner root." TRUE.
76. Test 6: "This pins a JUDGEMENT, not a measurement ... nothing outside this test establishes that the inner project is the right answer." TRUE as a record, and consistent with the sidecar and the doc comment.
77. Test 6: "RED before the change: `metrics: 3 records`, this directory's log, for either reading." TRUE, `$OLD` printed 3.
78. Test 7: "a run made from the plan's own project root with a BARE RELATIVE `--source` ... is UNCHANGED, byte for byte." TRUE for that spelling, verified by diffing `$OLD` and `$NEW` stdout (identical).
79. Test 7: "which is the normal invocation and the only one the scaffolded guidance documents". FALSE. FINDING W3B-5.
80. Test 7: "an 'improvement' that canonicalised the default would still read the right file and still pass a `contains` assertion while changing two of these three lines". TRUE by construction; the two path-bearing lines are lines 1 and 3.
81. Test 7: "A pin, not a red-then-green case: it passes identically before and after the change." TRUE.
82. Test 8: "plain `validate` (no `--workflow`) is unaffected by the tier policy and still exits 0 with a stderr note on a missing log." TRUE.
83. Test 8: "a bare `validate` with NO plan source has nothing to anchor to and keeps the historical current-directory-relative path." TRUE.
84. Test 8: "The anchored-but-missing case is red-then-green (before the change this read this directory's three-record log and printed it as valid)." TRUE, `$OLD` printed `docs/metrics/workflow.jsonl: 3 records, valid`.
85. Test 9: "a bare filename run from inside `docs/plans` has no parents to walk, falls back to the source's own directory, and looks for `docs/metrics/workflow.jsonl` beneath it, which does not exist. The project's real log is never read." TRUE.
86. Test 9: "This is not a regression (the pre-change build was identically wrong here)." TRUE, `$OLD` and `$NEW` produce byte-identical output for that invocation.
87. Test 9: "the fix is NOT to canonicalise the default ... This test exists so that change fails loudly here too." TRUE as a design statement, consistent with the plan.

Assertion messages (each a claim about what the assertion means):

88. "a foreign plan must never be declared to hold against this directory's log". TRUE for the default path.
89. "the check now runs against the plan's own log and must fail it". TRUE.
90. "expected the failure to name the plan's own log {away_log}". TRUE, the problem line is prefixed `<source> vs <metrics>` and names away's log.
91. "expected W3's correct red for the borrowed slug". TRUE, W3's message names `borrowed-step`.
92. "this directory's log must not be read at all". TRUE, `3 records` appears in neither stream.
93. "`next` is a projection and never fails". TRUE for these inputs, exit 0.
94. "a foreign log must not converge this plan's loop". TRUE.
95. "the fabricated completion instruction must be unreachable". TRUE on the default path.
96. "expected the plan's own log to be summarised". TRUE, `metrics: 1 records`.
97. "expected the state implied by the plan's own log". TRUE, `state: awaiting-first-review`.
98. Inline: "Asserting only the absence of the wrong answer would pass against a build that read no log at all." TRUE, and the positive assertion is present.
99. "expected the --source anchor" / "expected the --plan anchor". TRUE, both give 1 record.
100. Inline: "The Markdown `--plan` anchors identically, so one rule covers both substrates." TRUE, one code path.
101. "--source must win over --plan as the anchor". TRUE.
102. "expected the plan's own ledger" / "this directory's resume state must not leak into a foreign project's brief". TRUE on the default path.
103. "expected the conventionless root's own log" / "the same answer from the plan's own root". TRUE.
104. "nearest-wins selects the inner project's log (4), not the outer's (6)". TRUE.
105. "this spelling's output must be byte-identical to the pre-anchoring binary's". TRUE, verified by diff against `$OLD`. Correctly narrowed by round 2 to "this spelling's".
106. "expected the note to name the plan's own missing log" / "this directory's log must not stand in for the plan's". TRUE.
107. "a sourceless run keeps the current-directory-relative path" / "a sourceless resume keeps the current-directory-relative ledger path". TRUE, both byte-compared.
108. "expected the miss note naming the path it looked for" / "the project's real log must not be reached from here". TRUE.

### D. README (10 claims, 0 findings)

109. "The round log is resolved FROM THE PLAN, not from the directory you happen to be standing in." TRUE as a headline; the same paragraph carves out the explicit-flag and no-anchor cases.
110. "the nearest `<root>/docs/plans/` ancestor of `--source` (else of `--plan`), or the source's own directory when it has no such ancestor, so a plan at a project root with no `docs/plans` still reads that root's log." TRUE.
111. "`agent-scaffold validate --source /elsewhere/docs/plans/their-task.plan.toml --workflow` checks THEIR plan against THEIR log, rather than joining their plan to yours." TRUE, measured with an absolute foreign source.
112. "`status`, `status --resume` and `next` resolve the same way". `status --resume` resolves NO round log: `run_status` returns from `run_resume` at `src/main.rs:1078-1080` before `resolve_metrics_path` at `:1103`, and `status --resume --metrics /nonexistent/nope.jsonl` still prints the block at exit 0. NOT RAISED: the sentence reads naturally as "anchor to the plan source the same way", which is true of `status --resume` for its ledger, and the very next clause is about the ledger. Recorded as an ambiguity, not a falsehood.
113. "the ledger those two read is `<task>.ledger.md` beside the plan source." TRUE.
114. "An explicit `--metrics` (or `--ledger-fragment`) is used verbatim." TRUE, both measured.
115. "a run with neither `--source` nor `--plan` has nothing to anchor to, so it keeps the current-directory-relative `docs/metrics/workflow.jsonl`." TRUE.
116. "The rule is textual: it never consults `.git`, so it works the same in a nested repository, outside a repository, and in an unpacked tarball." TRUE, all three measured.
117. "a bare filename run from inside `docs/plans` ... looks for `docs/metrics/workflow.jsonl` beneath `docs/plans` and reports that it found no log". TRUE, and the exact command in the README reproduces it: stderr `no metrics log at docs/metrics/workflow.jsonl; nothing to validate` plus `--workflow has a plan source but the metrics log is missing; skipping the workflow check`, exit 0.
118. "run it from the project root instead." TRUE, doing so reads the real log.

### E. CHANGELOG (9 claims, 0 findings)

Same nine assertions as A, B and D above, checked individually: the PLAN SOURCE headline (with its own carve-out later in the entry), the derivation with all three arms, the ledger rule, the four pre-change consequences, the "bare relative `--source`" no-regression claim (byte-compared against `$OLD`), "the derivation is textual and consults no VCS", "an explicit `--metrics` or `--ledger-fragment` is still used verbatim", the no-anchor carve-out, and the scope list "on `validate`, `status` and `next`". The scope list is exhaustive and correct: `--metrics` exists on exactly those three subcommands and nowhere else (`scaffold`, `checks`, `render` and `audit` have no such flag), and `default_ledger_path` has exactly two callers.

### F. Plan sidecar and rendered view (14 claims, 2 findings)

Checked in `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` and mirrored in `docs/plans/agent-scaffold.md`.

- THE DERIVATION paragraph (`:158`): lexical, nearest-wins, start at the parent, root is the grandparent, no filesystem access, no canonicalisation, no-anchor keeps the historical path. ALL TRUE, measured.
- THE CONVENTIONLESS CASE (`:160`): fallback to the source's own directory, "the same answer the CWD-relative default would have given from the right directory". TRUE, `flat` gives 2 records from its own root under both binaries.
- WHAT THE DERIVATION WAS MEASURED AGAINST (`:162`), the spelling list: absolute, relative, `./`-prefixed, and "a `..` that stays below the project's own `docs/plans`". ALL TRUE, each re-run. The `..` clause is correctly narrowed by round 2.
- Same paragraph: "A subdirectory under `docs/plans` resolves to the project root." TRUE, `away/docs/plans/sub/q.plan.toml` reads away's log.
- Same paragraph: the nested-`docs/plans` judgement is "not evidenced" and "must not be treated as settled by measurement". TRUE as a record.
- Same paragraph, round 2's addition: "a `..` that climbs OUT through a `docs/plans` matches THAT directory, so `<root>/docs/plans/../../other/p.plan.toml` and `<root>/other/p.plan.toml` are the same file read against two different logs." TRUE, measured: the first spelling reads away's log (1 record), the second finds none. Settled behaviour, correctly described.
- THE REFUSAL paragraph, round 2's addition at `:164`: the divergent-anchor gap. TRUE as described; out of scope by instruction and confirmed present rather than re-litigated.
- THE LEXICAL/CANONICAL SPLIT (`:166`): "the printed path stays relative". FINDING W3B-2.
- THE REQUIRED END PROPERTY (`:111`): "must be unchanged ... except for the symlinked-`docs/plans` layout". FINDING W3B-1.
- Accepted cost (i) (`:256`): the bare-filename silent miss "is NOT a regression (the pre-fix build was identically wrong here)". TRUE, byte-identical under both binaries.
- Acceptance check 9 (`:316`): the byte-identity requirement names its exact command (`--source docs/plans/agent-scaffold.plan.toml`), so it is correctly scoped and needs no narrowing. TRUE.
- Acceptance checks 3 to 10 for inc1: each corresponds to a test in the new file and each was re-run by hand as well. ALL PASS.
- "What this step does not fix" bullet added in round 2: "`review_findings` and `triage_findings` are built from the task name alone (`src/findings_naming.rs:52-55`, via `src/next.rs:881-882`)". TRUE, and both line references are accurate against THIS tree: `:52-55` is `join_dir`, which substitutes only `<task>` into `docs/plans/<task>.reviews`, and `:881-882` are the two builder calls. The described behaviour reproduces: `next --source <foreign plan>` emits an anchored `ledger:` beside the foreign plan and an unanchored `review_findings:`.
- Stale line references NOT raised as findings: inc1 moved code, so some pre-existing references into `src/main.rs` no longer point where they say, including `Q-55-refusalscope`'s "`run_resume`'s doc comment at `src/main.rs:1150-1151`" (now `:1247-1251`) and acceptance check 14g's "`src/main.rs:1067-1069` returns before serialisation" (now `:1078-1080`). The quoted CONTENT is still accurate in both cases, the plan carries many such of-their-time references by convention, and renumbering the plan is not this increment's job. Recorded here so the next reader does not have to rediscover it.

### Guards run

- `TMPDIR=/tmp/wr3b-scratch just test`: exit 0, all four test result lines `ok`, zero failures. `TMPDIR` was set outside every git repository, so the three repository-sensitive tests behaved.
- `cargo test --test metrics_and_ledger_anchor_to_the_plan_source`: 9 passed, 0 failed.
- Contamination check: `strings` on `target/debug/agent-scaffold` shows only this worktree's path, and `strings` on the test binary shows `CARGO_BIN_EXE_agent-scaffold` baked to this worktree's `target/debug/agent-scaffold`. No stale binary from another tree.
