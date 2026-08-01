# Work review, round 3, `workflow-enforcement-tier-inc1`, TRIAGE

Triager: a separate agent from both round-3 reviewers, from the implementer and from the planner. Worktree `.claude/worktrees/triage-inc1-r3`, branch `triage/inc1-r3`, at `fe54995`, the exact commit both reviewers read, so every citation below resolves against the same text.

Inputs: `...-workreview-r3-reviewer-residue.md` (fix verification over both round-2 lanes plus rulings on two disclosed defects; two findings, `W3A-1` low and `W3A-2` medium) and `...-workreview-r3-reviewer-claims.md` (a fresh 118-claim inventory; five findings, `W3B-1` medium and `W3B-2` through `W3B-5` low).

METHOD. Every cited `file:line` was opened and the quoted text confirmed against the file. Every reproduction was RUN rather than accepted, including the two the residue lens offered as its own evidence. Two binaries: this worktree's own, and a PRE-anchoring binary built from `69c0525` exported with `git archive` into `/tmp/triage-r3-old` with `CARGO_TARGET_DIR=/tmp/triage-r3-old-target`, so nothing is inherited from a compiled tree. Fixtures at `/tmp/triage-r3-fx/t1`, confirmed outside any git repository (`git -C /tmp/triage-r3-fx rev-parse --is-inside-work-tree` -> `fatal: not a git repository`): `home` (3-record log with a converged `borrowed-step` round, `HOME resume state.` ledger) and `away` (1-record log with no evidence for that slug, same task name `p`, `AWAY resume state.` ledger). Distinct record counts identify which file was read. Every site sweep below was run CASE-INSENSITIVELY, because round 2's own triage grepped `every field` case-sensitively against a capitalised `Every field` and that missed site became `W3A-2`.

GUARDS RE-RUN HERE, not inherited. `TMPDIR=/tmp/triage-r3-scratch cargo test`: 373 + 5 + 1 + 1 + 9 + 3 + 1 + 2 = 395 passed, 0 failed. CONTAMINATION TRAP CHECKED BEFORE ANY FIGURE WAS TRUSTED: the anchor test binary was deleted and force-rebuilt, and `strings target/debug/deps/metrics_and_ledger_anchor_to_the_plan_source-bf4905c55850edca` reports `CARGO_BIN_EXE_agent-scaffold` baked to `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-inc1-r3/target/debug/agent-scaffold`, this worktree's own path. `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` -> `up to date`, exit 0. `cargo clippy --all-targets -- -D warnings` -> no output. `git status --short` shows only the untracked reviews directory: the worktree's tracked tree was not modified at any point.

Each finding is judged on its own evidence. No ruling was weighed against what it implies for the round arithmetic, and the out-of-scope precedent below was settled before the in-scope set was totalled. That ordering matters and is noted again in its own section: the precedent does not decide this round.

## Summary

| id | reviewer severity | triage severity | verdict | owning writer |
| --- | --- | --- | --- | --- |
| `W3A-1` | low | low (confirmed) | VALID, OUT OF SCOPE for inc1; does NOT reset the streak | backlog (a new step), not inc1 |
| `W3A-2` | medium | medium (confirmed) | VALID, fix required | PLANNER |
| `W3B-1` | medium | medium (confirmed) | VALID, fix required | PLANNER |
| `W3B-2` | low | low (confirmed) | VALID, fix required | PLANNER |
| `W3B-3` | low | low (confirmed) | VALID, fix required | IMPLEMENTER |
| `W3B-4` | low | low (confirmed) | VALID, fix required | IMPLEMENTER |
| `W3B-5` | low | low (confirmed) | VALID, fix required | IMPLEMENTER (test) + PLANNER (the `:111` half, folded into `W3B-1`'s edit) |

Nothing was dismissed and nothing was accepted as residual. The backstop re-check for a dismissed high or critical finding is not triggered.

DEDUPLICATION. `W3A-2` and the `W3B` set do NOT overlap. `W3A-2` (sidecar `:186`) and `W3B-2` (sidecar `:166`) are twenty lines apart in one file and are both over-reaching descriptions, but they assert different propositions about different subjects (`ActiveLoop` field derivation; printed-path relativity) and neither fix touches the other's text. `W3B-1` and `W3B-5` are different claims that land on ONE shared edit site, `:111`; they are counted separately and prescribed as a single edit, for the reason set out under `W3B-5`. Both `W3A-2` and `W3B-1` are MISSED SITES of round-2 findings rather than fresh defects, which is recorded per finding and is the subject of the pattern section.

## THE OUT-OF-SCOPE PRECEDENT: does a valid finding that predates the increment reset the convergence streak?

This is the question the round turns on procedurally, it is not covered by either half of the project's existing rule ("a new valid finding resets the streak", "an accepted residual counts clean"), and it is settled here as a precedent rather than as a one-off.

### The ruling

A VALID FINDING THAT IS OUT OF SCOPE FOR THE INCREMENT DOES NOT RESET THE CONVERGENCE STREAK. It is still valid, still recorded, still owed a fix somewhere, and it is still reported in the round totals as its own category. What the exemption denies is only the inference "this round found a defect, therefore this artifact has not yet converged".

### Why, in terms that generalise

The streak is not a tally of defects found while reviewing. It is EVIDENCE ABOUT ONE ARTIFACT: that an increment classified `risky` has stopped yielding defects under independent attack. This step's own risk paragraph (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:296`) says exactly what the two rounds are buying: inc1's failure mode is self-concealing, "A's own build, correct on the reported reproduction, was still emitting `workflow invariants hold` at exit 0 on two other inputs until A attacked it deliberately", so a second clean round is a second independent failure to find one. A defect that was in the tree before the increment opened, in lines the increment did not write, about a subject the increment does not touch, carries no information about that question. Counting it would make an increment's convergence a function of the total defect density of every file any reviewer happens to open, which no amount of work on the increment can reduce. A gate that cannot be passed by doing the work correctly is not a gate.

### The test, which must be applied with commands rather than argument

All FOUR conditions must hold. Each of the first two and the fourth is established by a command whose output is quoted in the triage; the third is a judgement and must be argued with the mechanism named.

1. PROVENANCE. The text predates the increment's base commit. Established by `git log -S <exact string> -- <file>` returning only commits outside the increment's range, plus `git log -1 --format=%ci` on the introducing commit. Not by reading a date in a comment.
2. UNTOUCHED. No commit in the increment's range modifies the lines carrying the claim. Established by `git diff <base> <head> -- <file>` producing no hit on the claim's text, and by `git blame` at the artifact commit attributing those lines outside the range.
3. INDEPENDENT SUBJECT. The claim is not about the property the increment changed, AND the increment's change is not what falsified it. This is the condition that does the real work. A stale claim that the increment's own change made false is the increment's defect even if the increment never opened the file, and it is IN scope no matter who wrote the sentence or when.
4. NO SHARED FIX. Correcting it requires no edit to any line the increment wrote, so it routes to a separate step without blocking or re-opening this one.

If any condition fails, the finding is IN SCOPE and resets the streak.

### The guards on the exemption, because it is the easiest available lever for manufacturing a clean round

- The classification is the TRIAGER's and is binding, and the four commands must be run and their output quoted. An out-of-scope ruling asserted without the commands is not one.
- The finding is recorded with its minimal fix even though the fix is not prescribed for this round, so a later reader can re-open it without rediscovering it.
- A round whose ONLY findings are out of scope must report that explicitly in its totals ("clean, with N out-of-scope findings recorded"), never as a bare "clean". An orchestrator reading the ledger must be able to see that a clean round was reached with findings on the table.
- A reviewer who disagrees with an out-of-scope ruling may carry the finding forward in the next round; the exemption is not a bar on re-raising.

### Applying it to `W3A-1`

1. PROVENANCE: PASS. Verified by me, not accepted from the reviewer.

```
$ git log --oneline --all -S "clap-required for now" -- src/main.rs
8017a2c fix: substrate-correct W5 locator and accurate TOML-swap docs
$ git log --oneline --all -S "still requires" -- src/main.rs
8017a2c fix: substrate-correct W5 locator and accurate TOML-swap docs
$ git log -1 --format="%ci %H %s" 8017a2c
2026-07-19 09:14:12 +0100 8017a2c9398a429888cd1f43b125ac90d2b83fcc fix: substrate-correct W5 locator ...
$ git log -1 --format="%ci %H %s" 69c0525
2026-08-01 15:16:58 +0100 69c0525046a47dcdeecf8185b2c79024beee9c5b docs: start the workflow-enforcement-tier step
$ git log --oneline 69c0525..fe54995
fe54995 / b18a0a8 / f8f2e09 / be2c897 / f491c4e     (five commits; 8017a2c is in none of them)
```

2. UNTOUCHED: PASS, with one fact the residue lens did not report and which I record because it is the strongest available argument against this ruling.

```
$ git diff 69c0525 fe54995 -- src/main.rs | grep -c "clap-required\|still requires"
0
$ git blame -L 805,815 fe54995 -- src/main.rs        # :806 and :808-814 -> 8017a2c; :805,:807 -> 88356ad
```

THE FACT AGAINST MY OWN RULING: inc1 DID edit the same doc BLOCK. `git diff 69c0525 fe54995 -- src/main.rs` carries a hunk at `@@ -788,8 +788,11 @@` which rewrites `run_validate`'s opening paragraph and inserts a whole new paragraph about metrics resolution at `:794-795`, about ten lines above the false clauses. The residue lens wrote "no inc1 commit ever touches these lines", which is true of the LINES and not of the BLOCK. My ruling stands anyway, and the reason is the test's own wording: condition 2 is about the lines carrying the claim, deliberately, because adjacency is not authorship. Treating "edited somewhere in the same doc comment" as scope would make an increment's review scope a function of how long the surrounding comment happens to be, which is arbitrary. What would move it into scope is condition 3, not condition 2.

3. INDEPENDENT SUBJECT: PASS. The claim is about clap argument relationships between `--workflow` and `--plan`. The increment changed which FILE the defaulted `--metrics` resolves to. I established what actually falsified the claim rather than inferring it:

```
$ git show 8017a2c:src/main.rs | sed -n '381p'
	#[arg(long, requires = "plan")]                       # on `workflow`: the doc was TRUE when written
$ git show f230f80:src/main.rs | sed -n '382p'
	#[arg(long)]                                           # relaxed, same day, 2026-07-19 14:00:53 +0100
$ git merge-base --is-ancestor 8017a2c f230f80 && echo "doc written BEFORE the relaxation"
doc written BEFORE the relaxation
```

`f230f80` rewrote the `--workflow` help string in the same commit and left the `run_validate` doc block behind. That is `f230f80`'s residue miss, roughly two weeks before this step opened. Nothing in inc1's diff bears on it.

4. NO SHARED FIX: PASS. The fix is two deletions inside `src/main.rs:806-807` and `:813-814`, plus one clause at `CHANGELOG.md:14` (a twin I found and the residue lens did not; see the finding). `CHANGELOG.md` is a file inc1 edited, but at line 22, a different entry in a different section; line 14 is untouched by the increment.

RULING ON `W3A-1`: VALID, LOW, OUT OF SCOPE for inc1, DOES NOT RESET THE STREAK, routed to a new backlog step.

### What this implies for THIS round, stated plainly

NOTHING. Round 3 carries six valid IN-SCOPE findings, two of them medium, so the round is not clean regardless of how `W3A-1` is classified. The precedent above was therefore settled in the one circumstance in which a precedent is worth most: where the ruling could not buy the outcome. A later reader should weigh it accordingly, and should also note the converse, that it has not yet been exercised in the case that matters, which is a round whose only findings are out of scope.

## `W3A-1` (low): VALID, out of scope, not prescribed for this round

### Citations reproduced

`src/main.rs:806-807`, inside `run_validate`'s doc block (`:791-819`): "With `--workflow` (which still requires / `--plan`), the plan status is cross-referenced against the round log". CONFIRMED verbatim.

`src/main.rs:813-814`, same block: "`--plan` stays / clap-required for now (the relaxation for a TOML-only project is deferred)." CONFIRMED verbatim.

### Verified false, three ways, all re-run here

1. THE STRUCT. `ValidateArgs.plan` (`:432-434`) carries a bare `#[arg(long)]`. `ValidateArgs.workflow` (`:438-440`) carries a bare `#[arg(long)]`. The only `requires` anywhere in `ValidateArgs` is `requires = "workflow"` on `workflow_spec` (`:442`). Read at the lines, not grepped.

2. THE HELP STRING FOR THE SAME FLAG, WHICH IS ALREADY CORRECT. `src/main.rs:438`: "A TOML-primary --source needs no --plan (a TOML-only project has no Markdown plan); the Markdown path still needs --plan present. Requesting --workflow with neither a TOML-primary --source nor a --plan is an error." True, and it directly contradicts the doc block at `:806-807` and `:813-814` in the same file.

3. A FRESH REPRO, outside any git repository, no `--plan` given:

```
$ cd /tmp/triage-r3-fx/t1/away && .../agent-scaffold validate --metrics docs/metrics/workflow.jsonl --workflow --source docs/plans/p.plan.toml
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` but has no round records ...
exit=1
```

Exit 1 is W3's correct red, not clap's usage error (which would be exit 2). The check ran to completion with no `--plan`.

### A TWIN THE RESIDUE LENS DECLARED ABSENT

`CHANGELOG.md:14`, in `## [Unreleased]` / `### Added`: "It requires `--plan` and reuses the same metrics log as the rest of `validate`." Same claim, same falsity, same cause. Introduced 2026-07-17 by `5a4effc` (`git log -S "It requires \`--plan\` and reuses the same metrics log" -- CHANGELOG.md`), true when written, stranded by the same `f230f80` relaxation, and an ancestor of it (`git merge-base --is-ancestor 5a4effc f230f80` -> yes). The residue lens's twin sweep grepped `clap-required` and `still requires`, neither of which matches `It requires`. This is the same site-count failure mode the pattern section names, and it is worth recording that it occurred in a lens whose declared job was the sweep.

The finding remains valid, out of scope and low with the twin added; a third stale site does not change the class.

### Severity

LOW, CONFIRMED. The behaviour is right, is pinned by a dedicated regression test (`tests/validate_workflow_toml_source_needs_no_plan.rs`, 2 passed), and the correct statement already sits in the same file at `:438`. Nothing depends on the false text. It sits where round 1's `W1A-3` sat: an inaccuracy that misleads only about plumbing, with no safety or correctness consequence.

### MINIMAL FIX AND SITE COUNT (`W3A-1`), RECORDED, NOT PRESCRIBED FOR THIS ROUND

SITE COUNT: 3, in 2 files. Swept case-insensitively across `src/`, `tests/`, the three step sidecars (`workflow-enforcement-tier.md`, `test-tmpdir-repo-assumption.md`, `status-resume-ignores-json.md`), `docs/plans/agent-scaffold.plan.toml`, `README.md`, `CHANGELOG.md` and the generated `docs/plans/agent-scaffold.md`, on `clap-required` (1 hit, the finding), `still requires` (3 hits: the finding plus `status-resume-ignores-json.md:119` and its generated mirror `:1976`, both the unrelated and correct `--ledger-fragment still requires --resume`), `requires --plan` and ``requires `--plan` `` (1 hit, `CHANGELOG.md:14`), and `required either way` (0).

FIX CLASS: 3 DELETIONS. A deleted claim cannot be wrong, and every one of these deletes a sentence whose true content is already stated accurately elsewhere in the same file.

1. `src/main.rs:806-807`. Delete the parenthetical, leaving "With `--workflow`, the plan status is cross-referenced against the round log".
2. `src/main.rs:813-814`. Delete the sentence "`--plan` stays clap-required for now (the relaxation for a TOML-only project is deferred)." entirely; the preceding sentence ends the paragraph correctly at "skipped, the same treatment a missing file gets elsewhere here."
3. `CHANGELOG.md:14`. Delete "It requires `--plan` and", leaving "It reuses the same metrics log as the rest of `validate`."

ROUTE: a new backlog step of its own, a sibling of `test-tmpdir-repo-assumption` (order 95) and `status-resume-ignores-json` (order 96), both of which are pre-existing defects this same step's review surfaced and held elsewhere. It must NOT be added to inc1's fix pass: doing so would make inc1's next artifact contain edits no round-3 finding against inc1 required, which is precisely the coupling the out-of-scope ruling exists to prevent.

## `W3A-2` (medium): VALID, fix required, PLANNER

### Citation reproduced

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:186`, and identically at `docs/plans/agent-scaffold.md:1581`: "Every field of `ActiveLoop` is derived from the rounds, including `role`, `prompt`, `context`, `reminders` and `filled_prompt_summary`, so the block goes as a unit; suppressing only the `next:` line would leave an instruction assembled from foreign evidence in the surrounding fields." CONFIRMED verbatim at both.

### Reproduced live, on a fixture built for this ruling

A TOML-primary project with one `in-progress` step and a ZERO-RECORD log, `next --json`, `--isolation-tier container`:

```
"total_rounds": 0,
"round_cap": 5,
"isolation_tier": "container",
"next_instruction": {
  "role": "reviewer",
  "context": {
    "isolation_tier": "container",
    "ledger": "docs/plans/p.ledger.md",
    "review_findings": "docs/plans/p.reviews/only-step-reviewer-<disambiguator>.md",
    "triage_findings": "docs/plans/p.reviews/only-step-triage.md"
  },
```

Zero rounds, yet `round_cap: 5` (the workflow spec's), `isolation_tier: "container"` (exactly the CLI value), and both report paths built from the task name `p` and the step slug `only-step`. Confirmed against the struct's own doc comments, read at the lines: `src/next.rs:151-153` ("The advisory total-round cap (from the workflow spec)"), `:156-158` ("The isolation tier echoed from the CLI"), and `build_context` at `:873-900`, whose `review_findings`/`triage_findings` come from `findings_naming`, which substitutes `<task>` into a template (`src/findings_naming.rs:52-55`).

### Self-contradicted by text the planner wrote in the round-2 fix pass

Sidecar `:382`, added at `fe54995`: "`review_findings` and `triage_findings` are built from the task name alone". `:186` says `context` is derived from the rounds. Two sentences in one file, added roughly two hundred lines and one fix pass apart, giving opposite answers about the same map. Confirmed by reading both.

### A further looseness in the same sentence, recorded but not separately filed

`role`, `prompt`, `context`, `reminders` and `filled_prompt_summary` are not fields of `ActiveLoop` at all. They are fields of `Instruction`, reached through `ActiveLoop.next_instruction`, and two of the five are spelled differently there (`prompt_path`, `principle_reminders`). `src/next.rs:130-161` lists `ActiveLoop`'s twelve fields and none of the five is among them. The human `ACTIVE LOOP` block does print all five, so the loose usage is intelligible and I do not file it; it is noted because a fix that only swaps a quantifier would leave it.

### This is a MISSED SITE of `W2B-3`, not a fresh defect

Round 2's `W2B-3` fix deleted the CODE copy at `src/main.rs:1282-1286`. Its site count declares grepping `every field`, and this sentence reads `Every field`, so a case-sensitive grep missed it. The claim class was ruled MEDIUM in round 2 on reasoning that transfers unchanged: an exhaustiveness claim, unusually easy to falsify, whose falsity is what would stop a reader noticing a non-round-derived value inside an argument that everything in the block is safe because it is all round-derived.

### The load-bearing constraint, weighed

The planner's position is that `:186`'s sentence constrains inc2's implementer ("the block goes as a unit; suppressing only the `next:` line would leave an instruction assembled from foreign evidence"), and that constraint must survive. IT DOES, AND ON A MUCH SMALLER PREMISE. The conclusion needs only that ENOUGH of the block is round-derived that blanking `next:` alone leaves round-tainted output standing. `state`, `increment`, `risk_class`, `consecutive_clean`, `required_streak`, `total_rounds` and `valid_transitions` are round-derived, and `role`, `prompt_path`, `principle_reminders` and `filled_prompt_summary` follow from `state`; each is confirmed at its own doc comment in `src/next.rs`. That is an EXISTENTIAL claim, and the argument never needed a universal one. This is the substantive difference between my prescription and the reviewer's offered shape, which re-enumerates both sides and so re-exposes the same falsification surface.

SEVERITY: MEDIUM, CONFIRMED. Matches `W2B-3` exactly, and the self-contradiction with `:382` is what keeps it there rather than at low: a reader of the sidecar today gets two opposite answers about the same fields, and one of them is the answer that would let a foreign-evidence field through.

### MINIMAL FIX AND SITE COUNT (`W3A-2`)

SITE COUNT: 1 hand-edited plus 1 regenerated. Swept case-insensitively across `src/`, `tests/`, the three step sidecars, `docs/plans/agent-scaffold.plan.toml`, `README.md`, `CHANGELOG.md` and `docs/plans/agent-scaffold.md`, on `every field` (4 hits: sidecar `:186`, its generated mirror `:1581`, and `src/metrics.rs:1664` and `:1699`, both about validator schema field names and not derivation claims), `all fields` (0), `each field` (1, `src/metrics.rs:1545`, unrelated), `derived from the rounds` (2, the same pair) and `goes as a unit` (2, the same pair). NO TWIN.

FIX CLASS: 1 DELETION of the universal premise, replaced by an EXISTENTIAL restatement, plus 1 mechanical regeneration.

PLANNER, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:186`. Replace:

```
Every field of `ActiveLoop` is derived from the rounds, including `role`, `prompt`, `context`, `reminders` and `filled_prompt_summary`, so the block goes as a unit; suppressing only the `next:` line would leave an instruction assembled from foreign evidence in the surrounding fields.
```

with:

```
The block goes as a unit: suppressing only the `next:` line would leave `state`, `streak` and `rounds` standing above it, which is the same foreign evidence in a quieter form. NOT every field of `ActiveLoop` is round-derived, and this argument does not need them to be: `round_cap` comes from the workflow spec, `isolation_tier` is echoed from the CLI, and the `review_findings`/`triage_findings` slots in `context` are built from the task name, as the last bullet of "Scope: what this step does not do" records.
```

Then regenerate `docs/plans/agent-scaffold.md` with `cargo run -- render docs/plans/agent-scaffold.plan.toml` and commit both together. Never hand-edit the generated file; `render --check` is acceptance check 1 and is green today.

The three named fields in the replacement are the three the SAME paragraph already quotes two sentences earlier as the measured bad output (`state: converged`, `streak: 1/1`, `rounds: 2/5`), so the correction points at evidence the paragraph already carries rather than introducing new referents.

## `W3B-1` (medium): VALID, fix required, PLANNER

### Citation reproduced

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:111`, and identically at `docs/plans/agent-scaffold.md:1506`: "A run made from the plan's own project root, which is the normal invocation and the only one the scaffolded guidance documents, must be unchanged (Safe on existing projects), except for the symlinked-`docs/plans` layout recorded below as accepted cost (ii)." CONFIRMED verbatim at both.

### The falsifying run, made here against a pre-change binary

Three spellings of one file, every run made FROM `away`, that plan's own project root:

```
########## docs/plans/p.plan.toml
OLD: docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: ...                    exit=1
NEW: docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: ...                    exit=1
########## ./docs/plans/p.plan.toml
OLD: ./docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: ...                  exit=1
NEW: ./docs/plans/p.plan.toml vs ./docs/metrics/workflow.jsonl: ...                exit=1
########## /tmp/triage-r3-fx/t1/away/docs/plans/p.plan.toml
OLD: <abs plan> vs docs/metrics/workflow.jsonl: ...                                exit=1
NEW: <abs plan> vs /tmp/triage-r3-fx/t1/away/docs/metrics/workflow.jsonl: ...      exit=1
```

REPRODUCES EXACTLY. The bare relative spelling is byte-identical. The `./`-prefixed and absolute spellings are not: same file read, same exit code, changed printed path, and neither run involves a symlinked `docs/plans`, so neither is covered by the stated exception.

### That the printed path is part of "unchanged" is the plan's own position

Checked against the sidecar rather than assumed. Accepted cost (ii) at `:258` rejects the canonicalising variant because it "turns every printed metrics path absolute, changing output on the correct case", and acceptance check 9 at `:316` requires the three stdout lines to be "BYTE-IDENTICAL to the pre-fix binary's". Both make the printed path the measured quantity.

### This is the twin round 2 left, and it was left by vocabulary

Round 2's `W2B-2` narrowed the same claim at `CHANGELOG.md:22` and at `tests/...:370` by inserting "with a bare relative `--source`", and deleted a conjunct at sidecar `:166`. Its declared site-count greps were `byte-identical`, `byte for byte`, `still prints the relative paths` and `unchanged and still prints`. `:111` states the same claim in none of those words, so it was outside the search rather than outside the scope: the round-2 triage's declared scope explicitly includes the three step sidecars. My own sweep on the CLAIM's vocabulary finds all four copies: `the normal invocation` (case-insensitive) gives exactly `CHANGELOG.md:22`, `tests/...:371`, sidecar `:111` and generated `:1506`.

### The behaviour is not the defect

The lexical default is the decided mechanism (`:166`, "MUST NOT BE COLLAPSED"), and keeping the caller's spelling is what it is for. The `./` and absolute spellings are inherent consequences of it, not deviations from it. The CLAIM is what needs correcting, exactly as in `W2B-2`.

SEVERITY: MEDIUM, CONFIRMED. It is the step's stated END PROPERTY, which is the surface an acceptance review checks "Safe on existing projects" against and which inc2's implementer reads as the requirement. Round 2 rated the identical claim medium in the CHANGELOG and the test; the sidecar is the upstream of both, so it does not fall below them. It does not rise to high: nothing executable is wrong, the exit code and the file read are unchanged in every measured spelling, and acceptance check 9 (which names its exact command) is correctly scoped and stays as written.

### MINIMAL FIX AND SITE COUNT (`W3B-1`)

SITE COUNT: 1 hand-edited plus 1 regenerated. Swept case-insensitively across the same eight targets on `the normal invocation` (4 hits, two already narrowed by round 2, two being this finding and its mirror), `must be unchanged` (2, the pair), `except for the symlink` (2, the pair) and `Safe on existing projects` (many, all the principle's name rather than the claim). NO FURTHER TWIN.

FIX CLASS: 1 NARROWING plus 1 DELETION (the deletion is `W3B-5`'s half; see there for why they must land together), plus the shared regeneration.

PLANNER, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:111`. Replace:

```
A run made from the plan's own project root, which is the normal invocation and the only one the scaffolded guidance documents, must be unchanged (Safe on existing projects), except for the symlinked-`docs/plans` layout recorded below as accepted cost (ii).
```

with:

```
A run made from the plan's own project root with a bare relative `--source`, which is the normal invocation, must be unchanged (Safe on existing projects), except for the symlinked-`docs/plans` layout recorded below as accepted cost (ii).
```

The six inserted words are COPIED from the text round 2 already landed at `CHANGELOG.md:22`, so the three surviving copies of this claim finally agree. WHAT MAKES THIS NARROWING DIFFERENT FROM THE ONE THAT FAILED IN ROUND 2, stated plainly rather than asserted: the narrowed proposition is exactly what `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:380` asserts by whole-stdout comparison against a byte-literal, so if it ever stops being true a test goes red before a reviewer has to notice. The round-2 narrowing that produced `W3B-5` had no such guard. That is a real difference in risk, and it is the only one I claim.

## `W3B-2` (low): VALID, fix required, PLANNER

### Citation reproduced

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:166`, and at `docs/plans/agent-scaffold.md:1561`: "THE LEXICAL/CANONICAL SPLIT IS DELIBERATE AND MUST NOT BE COLLAPSED. The DEFAULT is lexical so the printed path stays relative; the GUARD is canonical so it cannot be spoofed by a symlinked source." CONFIRMED verbatim at both.

### The falsifying run

From the same comparison above, the absolute-`--source` case run from the plan's own root prints the defaulted metrics path as `/tmp/triage-r3-fx/t1/away/docs/metrics/workflow.jsonl`. It did not stay relative.

### A MEASUREMENT THAT DIFFERS FROM THE REVIEWER'S, and it narrows the finding without dissolving it

The reviewer writes that the deleted byte-identity conjunct and the surviving relativity conjunct "fail on the same input". They do not fail on the same input SET. The `./`-prefixed spelling falsifies byte-identity while leaving relativity TRUE: the printed path becomes `./docs/metrics/workflow.jsonl`, which is relative. Only the ABSOLUTE spelling falsifies both. The finding stands on that one spelling; its stated framing overstates the overlap.

### Why this is not dismissed, since it is the marginal one

There is a reading on which the clause is true. The very next sentence in the paragraph frames the cost of the rejected variant as "every resolved path becomes absolute EVEN WHEN THE USER TYPED A RELATIVE SOURCE", which supplies a relative-source condition the clause itself omits. I do not dismiss on that reading, for one decisive reason: ROUND 2 ALREADY RULED ON THIS EXACT SENTENCE, deleting its other conjunct as false because it was unconditional. It would be incoherent to delete conjunct A for being unconditional and then defend conjunct B by supplying a condition from context. The sentence is also written as a shouty directive to future implementers ("MUST NOT BE COLLAPSED"), which is the form most likely to be quoted alone.

SEVERITY: LOW, CONFIRMED. It is one clause in a design-rationale paragraph whose next sentence states the same fact correctly and with the measurement behind it, no executable check depends on it, and the falsifying spelling is narrower than `W3B-1`'s.

### MINIMAL FIX AND SITE COUNT (`W3B-2`)

SITE COUNT: 1 hand-edited plus 1 regenerated. Swept case-insensitively across the same eight targets on `stays relative` (2 hits, the pair) and `printed path` (7 hits: `tests/...:373` and `:456` and `src/main.rs:1161`, all true counterfactuals about the REJECTED canonicalising variant and none of them a claim about the default; sidecar `:316` and generated `:1711`, which is acceptance check 9, narrow and true and explicitly not to be edited per the round-2 triage; and the pair being fixed). NO TWIN.

FIX CLASS: 1 SUBSTITUTION of a CONSEQUENCE claim by the MECHANISM claim, copied from text already in the tree, plus the shared regeneration.

PLANNER, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:166`. Replace:

```
The DEFAULT is lexical so the printed path stays relative;
```

with:

```
The DEFAULT is lexical so the derived path keeps the spelling the caller typed;
```

The replacement clause is COPIED from `src/main.rs:1159-1160`, which the round-3 claims lens verified TRUE (its claim 19) and which I re-measured on all three spellings above: bare relative gives relative, `./` gives `./`, absolute gives absolute. This is the substitution that removes the falsification class rather than narrowing it: a CONSEQUENCE claim about what the output looks like must hold over every input spelling, while a MECHANISM claim describes what the rule does and has no input on which it differs. Nothing is lost, for the reason round 2 gave when it deleted the other conjunct: the next sentence already states the cost precisely and truly.

## `W3B-3` (low): VALID, fix required, IMPLEMENTER

### Citation reproduced

`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:262-264`: "Acceptance check 7: the ledger resolves BESIDE the plan source, so one project's `## RESUME STATE` block can no longer be printed as another project's resume anchor. Both readers are covered, since `next` echoes the same block `status --resume` prints." CONFIRMED verbatim.

The acceptance check it cites, sidecar `:314`: "AFTER INC1, the ledger stops leaking on the DEFAULT path". CONFIRMED, and the qualifier is present there. Check 14c at `:323` owes the explicit-`--ledger-fragment` half to inc2. CONFIRMED.

### The falsifying run

```
$ cd .../home && agent-scaffold status --resume --source .../away/docs/plans/p.plan.toml --ledger-fragment docs/plans/p.ledger.md
## RESUME STATE

HOME resume state.
exit=0

$ cd .../home && agent-scaffold next --source .../away/docs/plans/p.plan.toml --ledger-fragment docs/plans/p.ledger.md
RESUME STATE (verbatim from the ledger):
## RESUME STATE

HOME resume state.

$ cd .../home && agent-scaffold status --resume --source .../away/docs/plans/p.plan.toml    # control, default path
## RESUME STATE

AWAY resume state.
```

One project's block printed as another project's resume anchor, on both readers, with the default path correct beside it. The BEHAVIOUR is right for inc1: an explicit `--ledger-fragment` is documented as verbatim and the containment predicate is inc2's. The CLOSURE claim is the defect, and the plan warns against exactly this reading at `:286`: "An implementer must not read inc1's acceptance checks as evidence that defect C is closed."

### A CITATION CORRECTED inside an otherwise-valid finding

The reviewer writes that the sibling test at `:182` kept "on the default path, plus 'THE EXPLICIT-`--metrics` CASE IS STILL OPEN HERE BY DESIGN'". Only the first half is in the test. `tests/...:182-183` reads "Acceptance check 5: `next` no longer fabricates an instruction from a foreign log on the default path", and a case-insensitive sweep for `still open` across `src/`, `tests/` and the sidecar returns zero hits in `tests/`: that sentence is sidecar `:312`, acceptance check 5's own text. The correction does not touch the ruling; the sibling test did keep the qualifier this one dropped.

SEVERITY: LOW, CONFIRMED. An over-claim in an internal test doc, on a residual the plan already schedules to inc2 and already warns readers about, with the behaviour correct and the default-path half genuinely closed and pinned.

### MINIMAL FIX AND SITE COUNT (`W3B-3`)

SITE COUNT: 1. Swept case-insensitively across the same eight targets on `can no longer` (1 hit, the finding), `no longer be printed` (1, the same), `stops leaking` (2, sidecar `:314` and generated `:1709`, both already carrying "on the DEFAULT path", no edit) and `resume anchor` (the module doc at `tests/...:12` and `CHANGELOG.md:22` and sidecar `:132`, all describing the PRE-change consequence and all correct). NO TWIN.

FIX CLASS: 1 NARROWING, four words, copied from the acceptance check the doc already cites.

IMPLEMENTER, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:262-263`. Replace:

```
/// `## RESUME STATE` block can no longer be printed as another project's resume anchor.
```

with:

```
/// `## RESUME STATE` block can no longer be printed as another project's resume anchor ON
/// THE DEFAULT PATH.
```

The following sentence ("Both readers are covered ...") then inherits the default-path scope from the sentence before it and needs no edit. Incidental re-wrapping of the doc comment is expected and is not a defect.

OPTIONAL AND NOT PRESCRIBED, offered because the plan asks for it at `:286` and because the sidecar's own check 5 carries the equivalent: a following sentence pointing at where the residual lives, in the form "THE EXPLICIT-`--ledger-fragment` CASE IS STILL OPEN HERE BY DESIGN and is check 14c, after inc2." I do not prescribe it, because the four-word qualifier alone makes the claim true and adding composed prose is the risk class this loop keeps paying for. If the implementer adds it, it must be copied from that shape and not composed.

## `W3B-4` (low): VALID, fix required, IMPLEMENTER

### Citation reproduced

`src/main.rs:1161-1164`, in `project_root_of_source`'s doc comment: "It also means a `..` component is skipped rather than followed (`Path::file_name` is `None` for it), so the match is against whatever `docs/plans` lies lexically above that `..`, which is the plan's own only when the `..` does not climb out through one." CONFIRMED verbatim. This is round 1's own prescribed replacement text, landed at `be2c897`.

### The falsifying runs, constructed and checked rather than accepted

The finding asserts a specific resolved path, so I constructed it. Trace the code first (`src/main.rs:1173-1190`): for `<away>/other2/../docs/plans/p.plan.toml` the walk starts at the parent `<away>/other2/../docs/plans`, which matches on its FIRST ancestor, so the returned root is `<away>/other2/..` and the joined default is `<away>/other2/../docs/metrics/workflow.jsonl`. The `..` is never reached by the walk and the matched `docs/plans` lies lexically BELOW it, so the sentence's stated mechanism does not apply; read as a universal it predicts no match at all, hence the conventionless fallback to the source's own directory and a look for `<away>/other2/../docs/plans/docs/metrics/workflow.jsonl`, which does not exist.

```
$ ls /tmp/triage-r3-fx/t1/away/other2/../docs/plans/docs
ls: cannot access '...': No such file or directory     # what the sentence predicts: no log found
$ cd .../home && agent-scaffold status --source .../away/other2/../docs/plans/p.plan.toml | grep metrics
metrics: 1 records                                      # away's own log, the right answer
```

A MEASUREMENT THAT DIFFERS FROM THE REVIEWER'S. That run reproduces only when `away/other2` EXISTS as a directory. My first attempt, before creating it, printed `metrics: no log found`, because the kernel cannot stat a path through a nonexistent component. That outcome coincidentally matches the sentence's prediction, by a different mechanism, so an unwary re-run of the reviewer's spelling can look like a confirmation of the doc. The reviewer's SECOND spelling has no such dependency and is the cleaner falsifier:

```
$ cd .../home/docs && agent-scaffold status --source ../../away/docs/plans/p.plan.toml | grep metrics
metrics: 1 records                                      # sentence predicts no log found
```

Both `..` components exist as real directories, the walk matches `../../away/docs/plans` on its first step, and the root is `../../away`. The sentence mispredicts the RESOLVED PATH, not merely the route to it.

THE CASE THE SENTENCE IS ACTUALLY ABOUT IS TRUE, verified so the correction does not overshoot: `<away>/docs/plans/sub/../p.plan.toml` gives `metrics: 1 records`, the walk having skipped the `..` and matched the `docs/plans` above it. And the settled escaping case, which I do not re-litigate, is confirmed present: `<away>/docs/plans/../../other/p.plan.toml` gives `metrics: 1 records` while `<away>/other/p.plan.toml` gives `metrics: no log found`, the same file read against two different logs.

SEVERITY: LOW, CONFIRMED. The operative clause (the escaping-`..` warning) is correct and is the load-bearing half; the mispredicted case resolves CORRECTLY anyway, so the inaccuracy leads a reader to expect a worse outcome than occurs; and the complete rule, which entails the right answer for every `..` case, is already stated correctly in the paragraph immediately above at `:1151-1157`. It does not rise: it is in the doc comment of the function that produces the false green, and it is the third round in which this one sentence has been found wrong.

### MINIMAL FIX AND SITE COUNT (`W3B-4`)

SITE COUNT: 1. Swept case-insensitively across the same eight targets on `skipped rather than followed` (1 hit, the finding), `lies lexically above` (1, the finding), `climb out` (1, the finding) and `climbs out` (6: sidecar `:162`, `:164`, `:320` and their generated mirrors, all correct, `:162` being round 2's own correction of this same fact). NO TWIN. `docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md` carries the ancestor of this sentence and is OUTSIDE the swept scope and excluded by the round-1 precedent: exploration files are dated first-person records and are not edited.

FIX CLASS: 1 SUBSTITUTION, landing on text already in the tree and already verified twice.

IMPLEMENTER, `src/main.rs:1161-1164`. Replace:

```
It also means a `..`
/// component is skipped rather than followed (`Path::file_name` is `None` for it), so the
/// match is against whatever `docs/plans` lies lexically above that `..`, which is the
/// plan's own only when the `..` does not climb out through one.
```

with:

```
It also means the walk
/// matches on the components AS SPELLED (`Path::file_name` is `None` for a `..`, so a `..`
/// the walk REACHES never matches and the search continues above it), which is why a `..`
/// that climbs OUT through a `docs/plans` matches THAT directory:
/// `<root>/docs/plans/../../other/p.plan.toml` and `<root>/other/p.plan.toml` are the same
/// file read against two different logs.
```

The consequence half is COPIED from sidecar `:162`, which round 2 prescribed, which the round-2 and round-3 claims lenses each verified TRUE, and which I re-measured above. The mechanism half keeps `Path::file_name` (a maintainer reading the function needs it) and adds the one scoping word the sentence was missing, "REACHES", which is the reviewer's own formulation and which I measured on the `sub/..` case. Incidental re-wrapping of the surrounding doc-comment lines is expected and is not a defect; the following sentence ("Project root" here is a FILENAME convention ...) must be kept.

## `W3B-5` (low): VALID, fix required, IMPLEMENTER and PLANNER

### Citation reproduced

`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:370-372`: "Acceptance check 9, the Safe on existing projects pin: a run made from the plan's own project root with a BARE RELATIVE `--source`, which is the normal invocation and the only one the scaffolded guidance documents, is UNCHANGED, byte for byte." CONFIRMED verbatim. Round 2 inserted "with a BARE RELATIVE `--source`" into the subject, so the relative clause now asserts that the scaffolded guidance documents a `--source` invocation.

### The falsifying counts, re-run here

```
$ grep -ro -- "--source" pack/ | wc -l
0
$ grep -rc -- "--source" .agents/ | grep -v ":0"
(no output)
$ grep -c -- "--source" AGENTS.md
0
$ grep -rn "agent-scaffold validate" pack/ AGENTS.md
pack/instrument.md:13:... with `agent-scaffold validate`, which exits non-zero and reports any malformed record.
AGENTS.md:149:... with `agent-scaffold validate`, which exits non-zero and reports any malformed record.
```

Zero across the whole pack, the deployed `.agents/` and the rendered root `AGENTS.md`. The only `validate` invocation the scaffolded guidance carries is bare. `README.md:220` does show a `--source` invocation, but the README is agent-scaffold's own documentation, not the guidance scaffolded INTO a project, and the reviewer's reasoning on that distinction is correct.

### THE INTERACTION NEITHER REVIEWER HAD, AND IT IS THE MOST IMPORTANT LINE IN THIS FILE

This finding EXISTS because round 2's fix inserted a qualifier into a subject and thereby made an adjacent, previously-true relative clause false. The identical relative clause sits at sidecar `:111`, where it is TRUE TODAY (its subject is "a run made from the plan's own project root", with no `--source` in it, and a bare `agent-scaffold validate` is indeed run from a project root). `W3B-1` prescribes inserting "with a bare relative `--source`" into exactly that subject. APPLYING `W3B-1`'s NARROWING AT `:111` WITHOUT DELETING THE ATTRIBUTION CLAUSE THERE WOULD MANUFACTURE ROUND 4's FINDING BY THE IDENTICAL MECHANISM THAT MANUFACTURED THIS ONE. That is why the `:111` edit prescribed under `W3B-1` already carries both changes as one replacement, and why the two findings must not be routed to different passes.

SEVERITY: LOW, CONFIRMED. An attribution clause in an internal test doc; the substantive half of the claim (byte-identity for that spelling) is TRUE and is pinned by the assertion the doc sits above.

### MINIMAL FIX AND SITE COUNT (`W3B-5`)

SITE COUNT: 2 hand-edited (1 IMPLEMENTER, 1 PLANNER, the latter folded into `W3B-1`'s single `:111` replacement) plus the shared regeneration. Swept case-insensitively across the same eight targets on `scaffolded guidance` (4 hits: `tests/...:372`, the finding; sidecar `:111` and generated `:1506`, true today and made false by `W3B-1`'s narrowing, hence the combined edit; and `docs/plans/agent-scaffold.plan.toml:1298`, the step's own TITLE, "document the two enforcement tiers in the scaffolded guidance", unrelated and correct, DO NOT EDIT).

FIX CLASS: 2 DELETIONS. A deleted claim cannot be wrong; this is the safest class available and it is the whole prescription here.

IMPLEMENTER, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:371-372`. Delete " and the / only one the scaffolded guidance documents", leaving:

```
/// Acceptance check 9, the Safe on existing projects pin: a run made from the plan's own
/// project root with a BARE RELATIVE `--source`, which is the normal invocation, is
/// UNCHANGED, byte for byte.
```

The continuation ("The whole stdout is compared rather than searched ...") must be kept; incidental re-wrapping is expected and is not a defect.

PLANNER, sidecar `:111`. Already contained in the single replacement prescribed under `W3B-1`; do not apply it twice.

## The pattern this loop has now demonstrated three times, and what I did about it

Round 1's fix wrote a sentence that is itself wrong (`W3B-4`). Round 2's fix narrowed two of three twins and left the third (`W3B-1`), and separately made a true adjacent clause false (`W3B-5`). Two mechanisms produce all of it.

MECHANISM A: SITE COUNTS GREPPED IN THE FINDING'S VOCABULARY RATHER THAN THE CLAIM'S PROPOSITION. `W2B-3` searched `every field` and missed `Every field` at sidecar `:186` (now `W3A-2`). `W2B-2` searched `byte-identical`, `byte for byte`, `still prints the relative paths` and `unchanged and still prints`, and missed sidecar `:111`, which states the same claim in none of those words (now `W3B-1`). Both missed sites were INSIDE the round-2 triage's own declared scope. The residue lens repeated it in this very round, declaring `W3A-1` twin-free after grepping `clap-required` and `still requires`, which do not match `CHANGELOG.md:14`'s `It requires`.

MECHANISM B: A CORRECTION THAT ASSERTS A NEW CONSEQUENCE. Round 1 replaced a false clause with composed text that states a mechanism, and the composed text is false (`W3B-4`). Round 2 inserted a qualifier that changed the referent of the clause beside it (`W3B-5`).

WHAT IS DIFFERENT ABOUT THESE PRESCRIPTIONS, said honestly per class rather than claimed in general:

- `W3B-5` and `W3A-1`: DELETIONS. Zero residual risk. A deleted claim cannot be wrong.
- `W3B-2`, `W3B-4`, `W3B-3`: COPIES of wording already landed in this tree and verified independently (respectively `src/main.rs:1159-1160`, sidecar `:162`, and acceptance check 7 at sidecar `:314`). The residual risk is that the copied text is itself wrong; I re-measured each before prescribing it and the runs are above. `W3B-2` and `W3B-4` additionally change the KIND of claim, from a CONSEQUENCE (what the output looks like, which must hold over the whole input space) to a MECHANISM (what the rule does, which has no input on which it differs). That removes the falsification mode rather than narrowing it.
- `W3A-2`: the one composed prescription, and it carries the same risk class as round 1's composition. What is different is only that it replaces a UNIVERSAL claim with an EXISTENTIAL one: a universal is falsified by any member the writer forgot, while an existential is falsified only by its named members being wrong, and each of the three named here is established by a struct doc comment plus the live `next --json` run above. That does not remove the risk that I have a fact wrong; it removes the risk of an omission.
- `W3B-1`: a NARROWING, the class that failed in round 2. The difference I claim, and the only one: the narrowed proposition is exactly what `tests/...:380` asserts by whole-stdout comparison against a byte literal, so a future regression goes red in the suite rather than waiting for a reviewer. The round-2 narrowing that produced `W3B-5` had no guard behind it.
- Every site sweep in this file was run CASE-INSENSITIVELY and against the claim's proposition, with the patterns listed so the next round can audit them rather than repeat them.

SHOULD A CLASS BE DELETED RATHER THAN CORRECTED? YES, ONE, AND I NAME IT. AFFIRMATIVE EXHAUSTIVENESS CLAIMS ABOUT DERIVED OUTPUT should be deleted from this step's artifacts rather than narrowed. The evidence is four data points inside this one step: `W1A-3` ("Four of the tests are pins"), `W2B-4` ("Every test builds several projects"), `W2B-3` ("every field of the projected loop"), and now `W3A-2` ("Every field of `ActiveLoop`"). Every one was falsifiable at an edge, and in every case the argument the claim supported never needed the universal. Round 2 already deleted the code copy of `W2B-3`; `W3A-2` is the sidecar copy of the same sentence, which is why the prescription above deletes the premise rather than re-enumerating it. I do NOT extend the recommendation to the byte-identity and unchangedness family (`W3B-1`, `W3B-2`): there the claim IS the requirement, acceptance check 9 pins it, and deleting it would delete a criterion rather than a description.

## Measurements that differed from the reviewers'

Recorded so a later round can see what was re-measured rather than inherited.

- `W3A-1`, A TWIN THE RESIDUE LENS DECLARED ABSENT: `CHANGELOG.md:14`, "It requires `--plan` and reuses the same metrics log as the rest of `validate`". Same claim class, same cause, introduced 2026-07-17 by `5a4effc`, an ancestor of the `f230f80` relaxation. Adds a third site to the backlog fix; changes no ruling.
- `W3A-1`, A FACT AGAINST THE OUT-OF-SCOPE RULING THAT NEITHER REVIEWER RAISED: inc1 DID edit `run_validate`'s doc block (the `@@ -788,8 +788,11 @@` hunk), six lines above the false clauses. The lines carrying the claim are untouched, which is what condition 2 asks, but the block-level adjacency is real and is recorded rather than omitted.
- `W3B-2`: the deleted byte-identity conjunct and the surviving relativity conjunct do NOT fail on the same input set. The `./`-prefixed spelling falsifies the first and leaves the second true (`./docs/metrics/workflow.jsonl` is relative). Only the absolute spelling falsifies both.
- `W3B-4`: the reviewer's `other2/..` spelling reproduces ONLY when `away/other2` exists as a directory. Without it the run prints `metrics: no log found`, which coincidentally matches the doc's prediction by a different mechanism (a stat failure through a nonexistent component, not the conventionless fallback), so an unwary re-run can read as a confirmation of the doc. The reviewer's other spelling, `--source ../../away/docs/plans/p.plan.toml` from `home/docs`, needs no synthetic directory and is the falsifier I rely on.
- `W3B-3`, A CITATION CORRECTED: "THE EXPLICIT-`--metrics` CASE IS STILL OPEN HERE BY DESIGN" is sidecar `:312`, acceptance check 5, not the sibling test at `:182`. A case-insensitive sweep for `still open` across `tests/` returns zero hits. The test kept only "on the default path"; the finding is unaffected.
- `W3A-2`, A FURTHER LOOSENESS: `role`, `prompt`, `context`, `reminders` and `filled_prompt_summary` are not `ActiveLoop` fields at all but `Instruction` fields reached through `next_instruction`, and two are spelled differently there. Recorded, not separately filed, and the prescribed replacement does not inherit it.
- `W3A-2` AND `W3B-1` ARE MISSED SITES, NOT FRESH DEFECTS. Both are copies of claims round 2 ruled on and fixed elsewhere, left standing because the site-count greps used each finding's own wording. They are counted as valid round-3 findings, because the false text is in the tree at the reviewed commit, but a later reader calibrating this artifact family should count them as one fix-pass omission each rather than as two new discoveries.

## Round totals

- RAW findings across both lenses: 7 (2 residue lens, 5 claims lens).
- DEDUPLICATED: 7. No two findings assert the same proposition at the same site. `W3B-1` and `W3B-5` share one EDIT SITE (`:111`) and are prescribed as one replacement, but they are different claims with different falsifiers and are counted separately.
- VALID: 7.
- OUT OF SCOPE (a category ruled here, with its precedent set out above): 1, `W3A-1`.
- VALID AND IN SCOPE FOR INC1: 6.
- ACCEPT RESIDUAL: 0.
- DISMISSED: 0.
- SEVERITY MIX OF THE FULL VALID SET: 0 critical, 0 high, 2 medium (`W3A-2`, `W3B-1`), 5 low.
- SEVERITY MIX OF THE IN-SCOPE SET, which is what the round turns on: 2 medium, 4 low.
- FIX-CLASS BREAKDOWN, IN SCOPE: 6 hand-edited sites plus 1 mechanical regeneration. 2 DELETIONS (`W3B-5`'s clause at the test; `W3A-2`'s universal premise), 3 SUBSTITUTIONS COPIED FROM LANDED TEXT (`W3B-2` from `src/main.rs:1159-1160`, `W3B-4` from sidecar `:162`, `W3B-3` from acceptance check 7), 1 NARROWING WITH AN EXECUTABLE GUARD (`W3B-1`), and about 70 words of composed prose in total, all of it in `W3A-2`'s replacement and all supplied verbatim.
- FIX-CLASS BREAKDOWN, OUT OF SCOPE: 3 DELETIONS, recorded and routed, not prescribed for this round.
- OWNERSHIP: PLANNER takes 3 sites, all in `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (`:111`, `:166`, `:186`), plus one re-render of `docs/plans/agent-scaffold.md` covering all three. IMPLEMENTER takes 3 sites (`src/main.rs:1161-1164`, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:262-263` and `:371-372`). The lanes share no file and may run in PARALLEL with NO ORDERING between them.
- NO CODE BEHAVIOUR CHANGE IS PRESCRIBED. All six in-scope edits are comment or prose. ZERO MECHANISM DEFECTS were found by either round-3 lens, and I found none while reproducing: every claim about what the resolution DOES held on every input I constructed, including the pre-change comparison on three spellings, the `..` cases above and below the matched `docs/plans`, the conventionless fallback and the explicit-flag paths.

### MY RULING ON THE ROUND'S OUTCOME

ROUND 3 IS NOT CLEAN. Six valid in-scope findings, two of them medium, four of the six in artifacts the round-2 fix pass itself edited or created. The consecutive-clean streak stays at 0 of the 2 that `risky` requires. `W3A-1` is valid but out of scope and does not bear on this ruling; the ruling is unchanged if it is set aside entirely.

### Routing recommendation

1. A TWO-LANE FIX PASS IN PARALLEL, planner and implementer, strictly COPYING the supplied text. Every replacement above is given in full for exactly this reason: this artifact family's recorded way of spending its remaining rounds is a fix pass that composes rather than copies.
2. `W3B-1` AND `W3B-5` MUST LAND AS ONE EDIT AT `:111`. This is the single highest-value instruction in this file. Applying the narrowing without the deletion reproduces the exact mechanism that created `W3B-5`.
3. `W3A-1` GOES TO A NEW BACKLOG STEP (3 deletion sites: `src/main.rs:806-807`, `:813-814`, `CHANGELOG.md:14`), a sibling of `test-tmpdir-repo-assumption` (order 95) and `status-resume-ignores-json` (order 96). It must NOT be folded into inc1's fix pass.
4. THE CAP ARITHMETIC, WHICH THE ORCHESTRATOR NEEDS BEFORE ROUND 4 IS SPAWNED AND NOT AFTER. This was round 3 of a cap of 5, and the streak is 0 of 2. Rounds 4 and 5 must BOTH come back clean for the increment to converge inside the cap. THERE IS ZERO SLACK: any valid in-scope finding in round 4 makes convergence within the cap arithmetically impossible, and round 5 then reaches the cap and forces the human escalation. I recommend the orchestrator put that to the human NOW, with the calibration data below, rather than discovering it at round 5.
5. THE CALIBRATION DATA THAT DECISION SHOULD BE INFORMED BY, stated as data and not as a recommendation to waive. Across three rounds this increment has produced 14 valid findings (3, 4, 7), of which 13 are in scope, and EVERY ONE of them is an inaccurate DESCRIPTION of correct behaviour. An adversarial lens running 51 attacks, an 81-claim inventory, a 118-claim inventory, a mutation run and every acceptance check re-run against a pre-anchoring binary have found ZERO defects in what the code does. Two of round 3's six in-scope findings are copies the round-2 fix pass missed rather than new discoveries. What the two-clean-round gate is currently measuring on this artifact is prose accuracy in the surrounding documentation, not the self-concealing wrong-file failure the `risky` classification was argued from. That is a fact about the evidence, and the human is the one who decides what follows from it.
