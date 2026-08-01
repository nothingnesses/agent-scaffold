# Work review, round 1, `workflow-enforcement-tier-inc1`, TRIAGE

Triager: a separate agent from both reviewers and from the implementer. Worktree `.claude/worktrees/triage-inc1-r1`, branch `triage/inc1-r1`, at `f491c4e` (parent `69c0525`), the exact commit both reviewers read, so every citation below resolves against the same text.

Inputs: `...-workreview-r1-reviewer-fidelity.md` (fidelity, scope, acceptance checks; ZERO findings) and `...-workreview-r1-reviewer-adversarial.md` (adversarial correctness; three findings, `W1A-1` medium, `W1A-2` low, `W1A-3` low).

METHOD. Every citation was opened at the cited `file:line` and every reproduction was RUN, not read. Fixtures were built under `/tmp/triage-inc1-scratch`, outside any git repository. A PRE-change binary was built from `69c0525` in a copy outside the repository so "not a regression" is measured here too rather than inherited from the adversarial reviewer. One coverage claim was upgraded from a grep to a MUTATION run. The worktree's own tree was not modified at any point (`git status --short` shows only the untracked reviews directory); the mutation was applied to a copy at `/tmp/triage-inc1-scratch/mutant`.

A MEASUREMENT ERROR I MADE AND CORRECTED, recorded because it would silently invert a conclusion. My first pre-change run reported `9 passed; 0 failed` for the new test file, apparently contradicting the adversarial reviewer's `2 passed; 7 failed`. The cause was mine, not theirs: I built the pre-change tree by copying an already-compiled tree, and `CARGO_BIN_EXE_agent-scaffold` is baked into the test executable at ITS compile time, so the stale test binary still pointed at the ORIGINAL tree's post-change binary (`strings` confirms the embedded path `/tmp/triage-inc1-scratch/mutant/target/debug/agent-scaffold`). Forcing the test crate to recompile reproduced the reviewer's figure exactly: `2 passed; 7 failed`. Any later round that rebuilds a pre-change binary must recompile the test crate, not just the bin.

## Summary

| id | reviewer severity | triage severity | verdict | owning writer |
| --- | --- | --- | --- | --- |
| `W1A-1` | medium | medium (confirmed) | VALID, fix required | IMPLEMENTER |
| `W1A-2` | low | low (confirmed) | VALID, fix required | PLANNER |
| `W1A-3` (a) count/enumeration | low | low (confirmed) | VALID, fix required | IMPLEMENTER |
| `W1A-3` (b) unpinned ledger arm | low | low (confirmed) | VALID, fix required | IMPLEMENTER |

Both halves of `W1A-3` hold independently. Nothing was dismissed and nothing was accepted as residual.

`W1A-1` and `W1A-2` are ONE claim in TWO artifacts with TWO owners. They are kept as separate findings because they need separate writers, not because they are separate defects.

## `W1A-1` (medium, `src/main.rs:1168-1170`): VALID, fix required

CITATION REPRODUCED. `src/main.rs:1165-1173` reads, in `project_root_of_source`'s doc comment:

```
/// LEXICAL is a deliberate choice, not an omission. The derived path keeps the spelling
/// the caller typed, so a relative `--source` yields a relative log path and the printed
/// output on a correct run is byte-identical to what it was before anchoring; a
/// canonicalising rule would turn every printed path absolute and machine-specific. It
/// also means a `..` component is skipped rather than followed (`Path::file_name` is
/// `None` for it) and the real `docs/plans` above it still matches. "Project root" here is
```

The quoted clause is present, verbatim, at the cited lines.

MEASURED MYSELF, on my own fixture. `trap/other/p.plan.toml` is a plan whose single step `borrowed-step` is `complete`; its own log at `trap/other/docs/metrics/workflow.jsonl` has 14 records and no evidence for that slug, while the unrelated `trap/docs/metrics/workflow.jsonl` has 13 records including a converged round for it. Both runs are made from a third directory and `ls -i` confirms the two spellings name the same inode (`64640564`).

```
$ agent-scaffold validate --source $F/trap/other/p.plan.toml --workflow
.../trap/other/p.plan.toml vs .../trap/other/docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` but has no round records and no covering waiver; ...
exit=1

$ agent-scaffold validate --source $F/trap/docs/plans/../../other/p.plan.toml --workflow
.../trap/docs/metrics/workflow.jsonl: 13 records, valid
.../trap/docs/plans/../../other/p.plan.toml: 1 steps, 0 questions, valid
.../trap/docs/plans/../../other/p.plan.toml vs .../trap/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

`next` on the same pair, with the step moved to `in-progress`: the correct spelling gives `metrics: 14 records`, `state: awaiting-first-review`, `next: spawn a reviewer for the first review round`; the escaping spelling gives `metrics: 13 records`, `state: converged`, `streak: 1/1`, `rounds: 1/5`, `next: mark the step complete, re-render, and commit`, with a `summary` line, at exit 0. That is the output shape the sidecar names at line 186 as "the specific output the fix must make unreachable".

THE CLAIM IS FALSE AS WRITTEN. The load-bearing word is "real". What the walk matches after skipping a `..` is whatever `docs/plans` lies lexically above that `..`, which is the plan's OWN `docs/plans` only when the `..` does not climb out through one. In the reproduction the plan's real project (`trap/other`) has no `docs/plans` at all, so "the real `docs/plans` above it" does not exist and the sentence's promise cannot hold. The clause also sits inside a paragraph whose whole purpose is to defend LEXICAL as deliberate, which is the most load-bearing possible place for a false reassurance about the one input class that produces a silent false green.

THE MECHANICAL HALF IS TRUE and I verified it separately: a `..` that does not escape resolves correctly. `$F/away/docs/plans/sub/../p.plan.toml` and `$F/away/sub/../docs/plans/p.plan.toml` both report `metrics: 11 records`, `away`'s own log, run from a 3-record directory.

SEVERITY: MEDIUM, CONFIRMED. It does not reach high because nothing executable is wrong, no test encodes the claim, and no other document depends on it mechanically. It does not fall to low because the falsehood is a specific affirmative claim of correctness, about the exact case that produces a self-concealing false green, in the doc comment of the function that produces it, in an increment classified `risky` precisely for that failure mode.

A NOTE FOR ROUND 2 SO THE TWO REVIEWS ARE NOT READ AS CONFLICTING. The fidelity review's line 47 says it ran a live `..` test and confirmed the doc comment's claim at `src/main.rs:1169-1170`. I reproduced that run: `status --source docs/plans/agent-scaffold.steps/../agent-scaffold.plan.toml` prints `metrics: 245 records`, identical to the plain spelling. That spelling's `..` stays BELOW this project's `docs/plans`, so it exercises only the half of the clause that is true. The fidelity reviewer's measurement is correct and does not bear on the escaping case; the two reviews do not disagree. A round-2 reviewer must not treat a non-escaping `..` as re-confirming the clause.

### MINIMAL FIX AND SITE COUNT (`W1A-1`)

SITE COUNT: 1. Grepped across `src/`, `tests/`, the three step sidecars (`workflow-enforcement-tier.md`, `test-tmpdir-repo-assumption.md`, `status-resume-ignores-json.md`), `docs/plans/agent-scaffold.plan.toml`, `README.md` and `CHANGELOG.md` for the literal claim (`still finds the real`, `still matches`, `skipped rather than followed`, `skips past it`) and, semantically, for any restatement: every backticked `..` in `src/` and `tests/`, and the words `climbs`, `escape`, `dot-dot`, `parent component`. In CODE the claim exists once, at `src/main.rs:1169-1170`. No help string, no test, no README sentence and no CHANGELOG sentence restates it: I read the README anchoring paragraph and the CHANGELOG entry in full and neither mentions `..` at all. The `..` rules in `src/plan/source.rs` (lines 481-483, 659, 751-763, 1033-1050, 1384-1391) are the unrelated sidecar-ref containment rule, untouched by this diff and not a twin.

FIX CLASS: NARROWING. One clause is replaced inside one existing sentence. No new sentence, no test change, no behaviour change.

Replace `and the real `docs/plans` above it still matches` with:

```
, so the match is against whatever `docs/plans` lies lexically above that `..`, which is
the plan's own only when the `..` does not climb out through one
```

The wording is supplied so it is copied rather than composed. Do NOT add an inc2 reference here: the fidelity review verified that nothing in this diff's documentation mentions inc2 behaviour that has not landed, and adding one would break that. Incidental re-wrapping of the surrounding doc-comment lines is expected and is not a defect.

## `W1A-2` (low, sidecar line 162): VALID, fix required

CITATION REPRODUCED. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:162` is the paragraph headed `WHAT THE DERIVATION WAS MEASURED AGAINST`, and it contains, verbatim:

```
The `docs/plans` convention resolves correctly in every spelling constructed: absolute, relative, `./`-prefixed, and with a `..` inside the path (that last works because `Path::file_name` returns `None` for a `..` component, so the walk skips past it and still finds the real `docs/plans` above).
```

The reviewer's ruling is right and its reasoning is right. The sentence is a report of a MEASURED case matrix ("in every spelling constructed"), the escaping `..` was not in that matrix, and the parenthetical states the mechanism accurately and then draws a conclusion the mechanism does not support: `Path::file_name` returning `None` for `..` is exactly WHY a `docs/plans` the file does not live under can win. An unmeasured case presented inside a measured list inherits weight it was never given, and this file is what inc2's implementer reads as the authority on what the derivation already handles.

SEVERITY: LOW, CONFIRMED. It stays at low rather than rising to the code copy's medium because the same file already carries a counterweight at line 164: the refusal is described as "Measured to close both of A's self-found false passes and to resist a `..` escape that climbs out of a fixture root to reach agent-scaffold's own 235-record log". A reader of the whole file is not told the case is closed by inc1. That containment is real, but it is not precise enough to make the finding go away: line 164's escape is an explicit `--metrics` climbing out (acceptance check 13's case at line 320), not a `--source` spelled with an escaping `..`, so the source-spelling case is genuinely unrecorded.

### MINIMAL FIX AND SITE COUNT (`W1A-2`)

SITE COUNT: 2 files, of which 1 is hand-edited and 1 is REGENERATED.

1. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:162`, the source of record. Hand-edited.
2. `docs/plans/agent-scaffold.md:1557`, which carries the identical sentence. This is the GENERATED plan view (`<!-- GENERATED FILE - do not hand-edit ... Regenerate with `agent-scaffold render agent-scaffold.plan.toml`; hand edits are overwritten and caught by `agent-scaffold render --check` -->`). I confirmed `render --check` prints `docs/plans/agent-scaffold.plan.toml: up to date` at `f491c4e`, so it is green today and WILL go red if the sidecar is edited without a re-render. Acceptance check 1 runs `render --check`. This site is not named in the adversarial findings file; it is the twin a literal grep of the reviewer's named scope would miss because it is outside `src/`, `tests/` and the sidecars.

ONE FURTHER TWIN, FOUND AND DELIBERATELY EXCLUDED, recorded so round 2 does not re-find it and file it: `docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md:150` carries the ancestor of the same sentence ("The `..` case works because `Path::file_name` returns `None` for a `..` component, so the walk skips past it and still finds the real `docs/plans` above"). It must NOT be edited. Exploration files are, per `AGENTS.md:65`, dated first-person advisory design notes an explorer wrote at a point in time; editing one rewrites a historical measurement report rather than correcting a live claim. The planner's fold into the sidecar is the live claim, and that is the copy being corrected.

FIX CLASS: 1 NARROWING plus 1 RECORDED CONSEQUENCE (2 sentences of new prose), plus 1 mechanical regeneration. The prose here is genuinely unavoidable and I say so plainly: deletion alone would leave the sidecar silent on a case that inc2's implementer needs, and silence in the specification is what produced this finding in the first place. Exact text is supplied below so the planner copies rather than composes, which is what keeps a fix pass from manufacturing the next round's finding.

Narrowing, inside the existing sentence: replace

```
and with a `..` inside the path (that last works because `Path::file_name` returns `None` for a `..` component, so the walk skips past it and still finds the real `docs/plans` above)
```

with

```
and with a `..` that stays below the project's own `docs/plans` (that last works because `Path::file_name` returns `None` for a `..` component, so the walk skips past it and reaches that project's `docs/plans` above it)
```

Recorded consequence, appended to the END of that same paragraph, exactly these two sentences:

```
NOT IN THAT MATRIX, AND MEASURED AT WORK REVIEW: a `..` that climbs OUT through a `docs/plans` matches THAT directory, so `<root>/docs/plans/../../other/p.plan.toml` and `<root>/other/p.plan.toml` are the same file read against two different logs. It follows from the lexical rule rather than being a defect in it, it is not a regression (before anchoring, both spellings read the current directory's log), and it is the canonical guard in inc2, whose root comes from the source's REAL location, that rejects it.
```

DO NOT promote this to a third accepted cost in the "The two accepted costs" section (lines 252-258). An accepted cost is something that survives the step; this does not (inc2 closes it), and moving it there would be a scope change rather than a correction.

AFTER EDITING, RE-RENDER: `cargo run -- render docs/plans/agent-scaffold.plan.toml`, and commit the sidecar and `docs/plans/agent-scaffold.md` together. Never hand-edit the generated file.

## `W1A-3` (low, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:19-22`): VALID on BOTH claims, fix required for both

The two claims are separable and I ruled them separately, as instructed. BOTH HOLD.

CITATION REPRODUCED, with one correction. Lines 19-22 are cited correctly and read:

```
//! Four of the tests are pins rather than red-then-green cases, marked as such on each:
//! the correct-case no-regression check, the two no-anchor cases that keep the historical
//! path, and the accepted-cost bare-filename miss. They pass identically before and after
//! the change, which is the property they exist to hold.
```

The findings file quotes this with `that keep the historical path` elided and the lines re-flowed. The elision changes nothing about the claim or the ruling, but the quote is not verbatim and a reader comparing them should know that.

### Claim (a), the count is wrong: VALID

Reproduced the reviewer's mechanical count and then enumerated the whole file myself. There are 14 `run(...)` invocations in the file. Exactly ONE passes neither `--source` nor `--plan`:

```
$ grep -n 'run(&[a-z_]*, &\[' tests/metrics_and_ledger_anchor_to_the_plan_source.rs | grep -v -- '--source' | grep -v -- '--plan'
434:	let (code, stdout, stderr) = run(&home, &["validate"]);
```

The reviewer's reading of which pin the doc is miscounting is also right: the fourth is the from-its-own-root run at line 327 inside `a_source_with_no_docs_plans_ancestor_falls_back_to_its_own_directory`, which passes `--source` on BOTH of its runs and is the no-CONVENTION case, a different rule with a different fallback (`project_root_of_source`'s final `parent.to_path_buf()`, not `resolve_metrics_path`'s `None` arm).

A THIRD INACCURACY IN THE SAME SENTENCE, WHICH THE REVIEWER DID NOT NAME AND WHICH CHANGES THE FIX. "They pass identically before and after the change" is false for two of the four items the sentence enumerates. MEASURED, against a binary built from `69c0525` with the test crate forced to recompile:

```
test result: FAILED. 2 passed; 7 failed
failures:
    a_nested_docs_plans_resolves_to_the_inner_project
    a_source_with_no_docs_plans_ancestor_falls_back_to_its_own_directory
    next_projects_the_loop_from_the_plans_own_log
    plain_validate_and_a_sourceless_run_keep_their_behaviour
    status_counts_the_plans_own_log_from_either_anchor
    the_ledger_resolves_beside_the_plan_source
    validate_workflow_reads_the_plans_own_log_not_the_working_directorys
```

Only `the_correct_case_prints_the_same_relative_paths_it_always_did` and `a_bare_filename_from_inside_docs_plans_stays_a_silent_miss` pass pre-change. The other two enumerated items are MIXED tests: each contains a pin RUN alongside a red-then-green run, so the TEST does not pass identically before and after even though the run inside it does. A fix that only changes the numeral, or only relabels "the two no-anchor cases", leaves this clause false and re-opens the finding in round 2.

A CORRECTION TO THE REVIEWER'S PROPOSED FIX, measured rather than argued. The findings file says adding the missing assertion "makes the module doc's 'two no-anchor cases' true as written". It does not. Adding the run to `plain_validate_and_a_sourceless_run_keep_their_behaviour` puts two no-anchor RUNS inside ONE test, so "four of the TESTS" still does not add up (1 + 1 + 1 = 3), and that test still fails pre-change, so "they pass identically" stays false for it. Claim (a) and claim (b) are independent and both are owed.

### Claim (b), `default_ledger_path`'s no-anchor arm is pinned by nothing: VALID

Established statically and then confirmed dynamically.

STATIC, and it is exhaustive rather than a sample. `default_ledger_path` is a private function in the binary crate (`src/main.rs:1235`), referenced only at its two call sites `run_resume` (`:1263`) and `run_next` (`:1329`) plus doc comments at `:1075` and `:1282`; no `#[cfg(test)]` module calls it (`src/next.rs`'s unit tests at `:1137`, `:1160`, `:1673` supply `ledger_path` to `project` as a fixture STRING and never reach this function). Integration tests can therefore only reach it by driving the binary. I enumerated every `Command::new(env!("CARGO_BIN_EXE_agent-scaffold"))` in `tests/`: they live in `audit_command.rs`, `checks_missing_tmpdir.rs`, `checks_staged_hook_env.rs`, `scaffold_precommit_hook.rs`, `validate_toml_primary_skips_markdown_plan.rs`, `validate_workflow_toml_source_needs_no_plan.rs`, and the new anchor file. Only the anchor file runs `status` or `next` at all, and every one of its `status --resume` (line 278) and `next` (lines 199, 289) runs passes `--source`. The `map_or_else` default arm is unreachable from the suite.

DYNAMIC, which is the upgrade over the reviewer's two greps. I mutated the arm in a copy outside the repository, from `docs/plans/{task}.ledger.md` to `MUTANT/{task}.ledger.md`, and ran the whole suite:

```
$ TMPDIR=/tmp/triage-inc1-scratch/tmp cargo test
cargo exit=0
suites: 8  passed: 395  failed: 0
```

The mutation is live in that binary and observable (`status --resume` with no anchor prints `no ledger at MUTANT/task.ledger.md; nothing to resume`), the test executable is confirmed to point at the mutated binary, and the suite is still entirely green. 395/0 also independently confirms the adversarial reviewer's suite total. The arm is asserted by nothing.

The behaviour itself is CORRECT, which I verified against the shipped binary: `status --resume` with no anchor prints `no ledger at docs/plans/task.ledger.md; nothing to resume` at exit 0 (`next::derive_task` falls back to the slug `task` with neither anchor, `src/next.rs:993-1002`). This is a missing pin on correct behaviour that inc1 explicitly specifies, plus a module doc that says the pin is there. The reviewer's citation of that specification is correct: sidecar line 274 reads "with NEITHER, the ledger keeps today's `docs/plans/<task>.ledger.md`, as the metrics rule keeps its CWD-relative path for the same case".

SEVERITY: LOW, CONFIRMED, for both halves. No user-visible defect exists, the behaviour is right, and the doc inaccuracy misleads only about test coverage. It is not lower than low: the increment is `risky`, three help strings and the CHANGELOG all promise this exact fallback, and the mutation shows the promise is unguarded.

### MINIMAL FIX AND SITE COUNT (`W1A-3`)

SITE COUNT: 1 file, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs`, with 2 edit locations inside it.

The claim exists nowhere else. Grepped `src/`, `tests/`, the three sidecars, `agent-scaffold.plan.toml`, `README.md` and `CHANGELOG.md` for `no-anchor`, `no anchor`, and `pin`: the only hits are line 20 (the finding) and line 409 (`pin on the no-anchor rule`, singular, which is ACCURATE and must be left alone). No sidecar names the pin set, so there is no plan-side twin and no re-render is triggered by this fix.

Claim (a). FIX CLASS: DELETION or one-clause NARROWING, no authored prose either way. Replace lines 19-22 with the single sentence:

```
//! Several of the tests are pins rather than red-then-green cases, marked as such on each.
```

Deleting the paragraph outright is equally acceptable and equally safe. Nothing is lost in either case: "marked as such on each" is verifiably true (each pin-bearing test states its own pin status in its own doc comment at lines 305-307, 340-345, 381, 408-409, 444-452), so the module-level enumeration is redundant with the per-test comments it summarises. This single edit closes all three inaccuracies at once (the count, the mislabelled no-convention case, and the false "pass identically" clause). A numeral-only edit does NOT and must not be used.

Claim (b). FIX CLASS: NEW TEST. This one cannot be a deletion or a narrowing, and I am not going to pretend otherwise: the gap is missing coverage, and only code closes it. It is the smallest possible code addition: no new fixture, no new test function, six lines appended to the existing no-anchor test `plain_validate_and_a_sourceless_run_keep_their_behaviour` (lines 411-442), immediately after the existing bare-`validate` block and before the `remove_dir_all`:

```rust
	// No `--source` and no `--plan` on the ledger side either: `<task>` falls back to
	// `task` and the historical `docs/plans/<task>.ledger.md` stands.
	let (code, stdout, stderr) = run(&home, &["status", "--resume"]);
	assert_eq!(code, Some(0), "stdout:\n{stdout}\nstderr:\n{stderr}");
	assert_eq!(
		stdout, "no ledger at docs/plans/task.ledger.md; nothing to resume\n",
		"a sourceless resume keeps the current-directory-relative ledger path"
	);
```

I measured that exact output from the shipped binary; the implementer should still confirm the byte form from a run rather than trusting my transcription. `home` already exists in that test and writes `docs/plans/p.ledger.md`, not `task.ledger.md`, so the absent-ledger note is the deterministic result. This assertion is red under the mutation above and green on the shipped tree, which is the property that was missing. A stronger variant (write a `docs/plans/task.ledger.md` with a tagged block and assert the block is printed, proving the file is READ and not merely named) is available at the cost of one extra fixture write; the cheaper form is sufficient because the note names the resolved path, so the file is still identified by output content rather than asserted from the path.

## The `..` escape behaviour itself: I CONCUR that it is not a defect against inc1

Not before me as a defect, and I do not reopen it. I checked the four grounds rather than accepting them, and two of them I re-measured.

1. Inherent to a settled decision. Sidecar line 158 fixes the rule as "purely LEXICAL, NEAREST-WINS ... No filesystem access and no canonicalisation". A non-normalising lexical rule cannot distinguish two spellings of one file. Confirmed against the implementation: `project_root_of_source` (`src/main.rs:1179-1196`) makes no `fs::` call and no `canonicalize()` call.
2. The obvious in-increment fix is unsound. Lexical `..` collapsing is wrong across symlinks (`a/b/../c` is not `a/c` when `b` is a symlink), so it would trade a narrow wrong answer for a broader one. Accepted as reasoning, not re-measured.
3. NOT A REGRESSION, RE-MEASURED BY ME on the pre-change binary. Run from a directory whose own log carries the borrowed slug, BOTH spellings print `docs/metrics/workflow.jsonl: 13 records, valid` and `workflow invariants hold` at exit 0. Run from a directory whose log does not, both go red. Either way the pre-change metrics path is the CWD-relative clap `default_value` (`69c0525:src/main.rs:430`, `:456`, `:480`) and does not depend on the source spelling at all, so the two spellings were indistinguishable before. Post-change one of the two is fixed and the other reads a different wrong log. No input got worse.
4. inc2 closes it. Checked against the specification rather than a build, since inc2 does not exist: the guard's root comes from the source's canonicalised location (sidecar line 164), which for the trap spelling is `trap/other`, while the resolved log canonicalises to `trap/docs/metrics/workflow.jsonl`, which is not under it, so the predicate fires. Note for the inc2 planner and implementer: acceptance check 13 (line 320) covers an escaping `--metrics`, NOT an escaping `--source` spelling, so check 13 alone does not exercise this case. That is a gap in inc2's check set rather than a defect in inc1, and it is why `W1A-2`'s recorded-consequence sentence is worth its prose.

## ROUTING

`W1A-1` -> IMPLEMENTER. `src/main.rs` doc comment. Code artifact.

`W1A-3` (a) and (b) -> IMPLEMENTER. `tests/metrics_and_ledger_anchor_to_the_plan_source.rs`, module doc plus one new assertion. Code artifact. Both halves are one file and should land in one pass.

`W1A-2` -> PLANNER. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` is reviewed product content in the plan, which by this project's precedent is the planner's artifact, not the implementer's.

DOES ANY FIX REQUIRE ONE WRITER TO TOUCH THE OTHER'S ARTIFACT? No. The implementer's two edits are confined to `src/` and `tests/`; the planner's is confined to the plan sidecar and its generated view. The one thing to watch is the regeneration: `docs/plans/agent-scaffold.md` is a plan artifact, not code, so it stays in the planner's lane. The planner should run `cargo run -- render docs/plans/agent-scaffold.plan.toml` in its own worktree and commit both files together. If the planner's worktree cannot build the binary, the orchestrator may run the render at integration; that is a mechanical regeneration of a generated file, not authored content, so it does not cross the role boundary either way. What must NOT happen is the sidecar landing without the re-render: `render --check` is acceptance check 1, it is green today, and it would go red, opening round 2 with a self-inflicted finding.

CAN THE TWO HALVES PROCEED INDEPENDENTLY? YES, in parallel, with no ordering constraint. The files are disjoint and neither fix's correctness depends on the other's text. The only coupling is substantive consistency: `W1A-1` and `W1A-2` state the same corrected fact about the escaping `..`, and both writers must not drift. That coupling is discharged here rather than by sequencing, because the exact replacement text for both is supplied above and is to be copied, not composed. Both must land before round 2 opens, since a `risky` increment needs two consecutive clean rounds over ONE artifact set and round 2 must review a single tree.

## ROUND TOTALS

- RAW FINDINGS: 3 (fidelity reviewer 0, adversarial reviewer 3).
- DEDUPLICATED: 3. No two findings are the same defect in the same artifact. `W1A-1` and `W1A-2` are one root claim in two artifacts with two owners, and are counted separately because they need separate writers and separate edits; `W1A-3` is one finding with two separable claims, both upheld.
- VALID, FIX REQUIRED: 3 (four edits, since `W1A-3` splits into a doc edit and a test).
- VALID BUT ACCEPT RESIDUAL: 0.
- DISMISSED: 0.
- SEVERITY MIX OF THE VALID SET: 0 critical, 0 high, 1 medium (`W1A-1`), 2 low (`W1A-2`, `W1A-3`).
- ROUND 1 IS NOT CLEAN. The increment is `risky` and needs two consecutive clean rounds, so the streak stands at 0 and round 2 begins after both writers land.
- FIX-CLASS BREAKDOWN: 1 deletion (`W1A-3` (a)), 2 narrowings (`W1A-1`, `W1A-2`'s clause), 2 sentences of new prose (`W1A-2`'s recorded consequence, exact text supplied, genuinely unavoidable), 1 new test (`W1A-3` (b), six lines in an existing test), 1 mechanical regeneration (`docs/plans/agent-scaffold.md` via `render`). No finding requires a new sentence in code.
- ZERO MECHANISM DEFECTS were found by either reviewer, and I found none while reproducing. Nothing in this triage asks for a behaviour change.
