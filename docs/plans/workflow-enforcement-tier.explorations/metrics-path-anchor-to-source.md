# Exploration: anchor the defaulted `--metrics` path to the plan source's project root (candidate (a))

Explorer model: Claude Opus 5, 1M-context variant. Exact model id `claude-opus-5[1m]`.
Date: 2026-07-31.
Worktree: `.claude/worktrees/explore-metricspath-a`, at base commit `b3bc8e6`.
Brief: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, candidate (a), plus the two scope additions decided after that sidecar was written (the `SE-3` documentation half, and the sibling commands `status`, `next`, `default_ledger_path`).

Everything below was produced by building the mechanism in this worktree and running it. Nothing is asserted from reading code alone; where I did not verify something, it is listed under "What I did not verify".

## Headline

Candidate (a) works, is cheap, and kills the reproduction in the sidecar. It does NOT achieve the end property the sidecar states for this half of the step. I found three distinct false passes; anchoring the default kills one of them and leaves two. Adding candidate (b) as a guard on top of (a) kills all three, at a cost of about 80 more lines. My recommendation is to ship (a) and (b) together as one mechanism, and I would not ship (a) alone.

I also found that the sidecar's stated reason for treating `status` and `next` as separable is factually wrong, with a reproduction. That is the most consequential finding here.

## TMPDIR accounting

`ls /tmp | wc -l` at start: **106**. At end: **106**.

`TMPDIR` was exported to `.scratch` inside the worktree for the CLI work. For the test suite I used a single named directory `/tmp/agent-scaffold-explore-a`, created deliberately and removed at the end, for the reason given under "The full suite" below. No loose directories were left.

For the record, `/tmp` already contains 65 `agent-scaffold-checks-test-*` directories. None are mine: the newest is timestamped `2026-07-30T16:33`, before this session began at `2026-07-31T13:48`.

```
drwxr-xr-x 4 jessea users 4096 2026-07-30T16:33 /tmp/agent-scaffold-checks-test-1375127-worktree-state
...
=== current time ===
2026-07-31T14:10
```

The leak source is `std::env::temp_dir()` in `src/checks.rs:510`, `:589`, `:1038` and `src/main.rs:1725`, which honours `TMPDIR`. So the 32272 directories a previous agent left were caused by not setting `TMPDIR`, not by anything that ignores it. The discipline is sufficient. Note also `justfile:2`, `set tempdir := "/tmp"`, which pins just's own recipe scratch to `/tmp` regardless of `TMPDIR`; I avoided `just` and called `cargo` directly.

## Verifying the sidecar before building on it

The sidecar is accurate on every point I checked. I reproduced the false pass at this worktree's base commit with the pre-fix binary:

```
$ .scratch/agent-scaffold-prefix validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow
docs/metrics/workflow.jsonl: 235 records, valid
.../.scratch/fixture/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
.../.scratch/fixture/docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

235 records, matching the sidecar's count exactly. The fixture was rebuilt from scratch per the sidecar's command and `ls "$SCRATCH/docs"` printed only `plans`, confirming no `docs/metrics/` is scaffolded without `--instrument`.

The control also reproduces, proving the check itself is sound:

```
$ cd "$SCRATCH" && mkdir -p docs/metrics && : > docs/metrics/workflow.jsonl
$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped
exit: 1
```

One correction, in the "Scope: what this step does not do" section. See "The sibling commands" below.

## What I built

All changes are in `src/main.rs`. Four new items plus three call-site changes and three help strings.

New, in the block marked `-- Exploration spike: anchoring the defaulted metrics path to the plan source --`:

- `const METRICS_RELATIVE: &str = "docs/metrics/workflow.jsonl"`, the conventional path relative to a project root.
- `fn project_root_of_source(source: &Path) -> PathBuf`. THE RULE: a purely lexical, nearest-wins upward search. Start at the source's parent directory and walk up; the first ancestor whose own file name is `plans` and whose parent's file name is `docs` identifies `<root>/docs/plans`, and the root is that ancestor's grandparent. If no such ancestor exists, the source's own directory is the root. No filesystem access, no canonicalisation.
- `fn resolve_metrics_path(explicit, source, plan) -> PathBuf`. An explicit `--metrics` is returned verbatim. Otherwise `project_root_of_source(source or plan).join(METRICS_RELATIVE)`. With neither a `--source` nor a `--plan`, the historical CWD-relative path, since there is no plan to pair the log with.
- `fn resolve_as_far_as_possible(path: &Path) -> PathBuf`, `fn metrics_shares_root_with_source(metrics, source) -> bool`, `fn guard_root(source) -> PathBuf`. These are candidate (b), added after measuring that (a) alone is insufficient. `resolve_as_far_as_possible` absolutises and canonicalises the longest existing ancestor, then re-appends the components below it, so a metrics path whose leaf does not exist yet still has its directory prefix resolved. `metrics_shares_root_with_source` derives the root from the source's REAL (canonicalised) location and asks whether the resolved log lives under it.

Changed:

- `ValidateArgs::metrics`, `StatusArgs::metrics`, `NextArgs::metrics`: `PathBuf` with `#[arg(long, default_value = "docs/metrics/workflow.jsonl")]` becomes `Option<PathBuf>` with `#[arg(long)]`, and each help string states the anchoring rule in prose.
- `run_validate`, `run_status`, `run_next` each call `resolve_metrics_path` and use its result instead of `args.metrics`.
- `run_validate`'s `--workflow` block gains the guard, before the four-arm match, pushing a problem when the log is not under the plan's root.
- `fn default_ledger_path(task)` becomes `fn default_ledger_path(task, source, plan)`, resolving `<task>.ledger.md` BESIDE the plan source rather than at a CWD-relative `docs/plans/<task>.ledger.md`. Its two callers in `run_resume` and `run_next` pass the source and plan through.

## Sub-question 1: distinguishing a defaulted `--metrics` from an explicit one

I built BOTH routes to a working state and measured them. Route A first, then reverted it and built Route B; the Route A source is preserved at `.scratch/main.rs.routeA`.

**Route A, clap's `ArgMatches::value_source`.** Keeps `metrics: PathBuf` with its `default_value` and recovers the distinction from the raw matches. `Cli::parse()` discards the `ArgMatches`, so `main` has to be re-spelled as `Cli::command().get_matches()` plus `Cli::from_arg_matches(&matches)`, and a `bool` has to be threaded into all three command functions. The lookup is `matches.subcommand_matches("validate").and_then(|sub| sub.value_source("metrics"))`.

It works. It is also 17 lines LARGER than Route B for the same functionality (`+96/-13` against `HEAD`, versus `+79/-15` for Route B), and it has two measured defects.

Defect one, the help text lies. Clap prints the `default_value` it was given, and there is no way to keep the value and suppress the display:

```
      --metrics <METRICS>              Path to the JSONL metrics log to validate [default: docs/metrics/workflow.jsonl]
```

After anchoring, that `[default:]` is false. The default is resolved against the plan's root, not the current directory. Route B's help has no `[default:]` at all and states the rule instead.

Defect two, and this is the decisive one. Both the subcommand and the argument are addressed by STRING. I renamed the argument id to `metrics_typo` to simulate a later field rename, and measured what happens. In a DEBUG build clap panics:

```
=== DEBUG build with a typo'd arg id ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
thread 'main' (2340283) panicked at src/main.rs:599:29:
`"metrics_typo"` is not an id of an argument or a group.
Make sure you're using the name of the argument itself and not the name of short or long flags.
exit: 101
```

In a RELEASE build the same code silently answers "not supplied", so the user's explicit `--metrics` is ignored and the anchored path is used instead:

```
=== RELEASE build with the same typo'd arg id ===
    Finished `release` profile [optimized] target(s) in 11.42s
no metrics log at .../.scratch/fixture/docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
.../.scratch/fixture/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
exit: 0
```

The command line said `--metrics docs/metrics/workflow.jsonl`. The release binary read a different file and said nothing. That is a debug/release behaviour divergence guarded only by clap's `debug_assertions`, in a code path whose entire purpose is deciding which file to read.

**Route B, `Option<PathBuf>`.** The field is `None` exactly when the flag was absent. There is no recovery step, no string lookup, and no state where "defaulted" and "explicit" are the same value. `main` is untouched.

**Verdict on the Principle.** The sidecar argues Route B is favoured by Make illegal states unrepresentable, and asked me to test that argument rather than repeat it. It holds, and for a sharper reason than the abstract one. The illegal state Route A admits is not merely representable, it is REACHABLE BY A REFACTOR THAT COMPILES, and it is invisible in exactly the build configuration users run. Route B is also smaller and its help text is honest. I found no compensating advantage for Route A. Route B is what I built on.

## Sub-question 2: what happens when the source is not under `docs/plans/`

I built the lexical nearest-wins rule described above, then built a case matrix with a DIFFERENT record count in every candidate log location, so the printed count identifies which file was actually read. Fixture builder at `.scratch/mkcases.sh`, runner at `.scratch/matrix.sh`.

Final matrix, lexical rule (`WT` is the worktree root):

```
### resolved metrics log, by case (record count identifies the file)
A normal, abs source, cwd=worktree     -> WT/.scratch/cases/rootA/docs/metrics/workflow.jsonl: 3 records, valid
A normal, rel source, cwd=rootA        -> docs/metrics/workflow.jsonl: 3 records, valid
A normal, ./ prefix, cwd=rootA         -> ./docs/metrics/workflow.jsonl: 3 records, valid
A normal, .. inside path, cwd=rootA    -> docs/metrics/workflow.jsonl: 3 records, valid
A bare filename, cwd=rootA/docs/plans  -> no metrics log at docs/metrics/workflow.jsonl; nothing to validate
B plan at root, abs source             -> WT/.scratch/cases/rootB/docs/metrics/workflow.jsonl: 5 records, valid
B plan at root, cwd=rootB              -> docs/metrics/workflow.jsonl: 5 records, valid
C nested docs/plans (expect 7)         -> WT/.scratch/cases/rootC/docs/plans/vendor/docs/metrics/workflow.jsonl: 7 records, valid
D subdir under docs/plans (expect 13)  -> WT/.scratch/cases/rootD/docs/metrics/workflow.jsonl: 13 records, valid
E symlinked plan file (link at root)   -> WT/.scratch/cases/rootE/docs/metrics/workflow.jsonl: 17 records, valid
E real plan file (expect 17)           -> WT/.scratch/cases/rootE/docs/metrics/workflow.jsonl: 17 records, valid
F nested repo, inner plan (expect 19)  -> WT/.scratch/cases/outer/vendor/inner/docs/metrics/workflow.jsonl: 19 records, valid
G outside any git repo (expect 29)     -> WT/.scratch/cases/nogit/docs/metrics/workflow.jsonl: 29 records, valid
H symlinked docs/plans dir             -> WT/.scratch/cases/rootH/docs/metrics/workflow.jsonl: 37 records, valid
H via the real dir (expect 31)         -> WT/.scratch/cases/rootH/elsewhere/docs/metrics/workflow.jsonl: 31 records, valid
no source at all, cwd=rootA            -> docs/metrics/workflow.jsonl: 3 records, valid
explicit relative --metrics            -> docs/metrics/workflow.jsonl: 235 records, valid
markdown --plan anchor                 -> WT/.scratch/cases/rootA/docs/metrics/workflow.jsonl: 3 records, valid
```

Reading the cases:

- The `docs/plans` convention, in every spelling I could construct (absolute, relative, `./`-prefixed, and with a `..` inside the path), gives the right answer. The `..` case works because `Path::file_name` returns `None` for a `..` component, so the walk skips past it and still finds the real `docs/plans` above.
- Case B is the sub-question proper. A plan at a project root with no `docs/plans` gets the right log from the "source's own directory is the root" fallback, both from that root and from elsewhere. This is the case the sidecar said "has no convention to lean on"; the fallback resolves it correctly, and it is the same answer the CWD-relative default would have given when you happen to be standing in the right place.
- Case C, a second `docs/plans` in the parent chain, resolves nearest-wins to the inner project (7 records, not 11). I think that is right (the innermost project owns the plan), but it is a judgement call the rule makes silently, and it is the only place I chose between two defensible answers without external evidence.
- Cases F and G show the rule is git-agnostic. It never consults `.git`, so a nested repository and a directory outside any repository behave identically and correctly. That is a feature (no VCS dependency, works in a tarball), but it means "project root" here is a filename convention, not a VCS fact.

**Does `default_ledger_path` hold or break in these cases?** The sidecar says it "already assumes the `<root>/docs/plans/<task>.plan.toml` layout". Establishing this by running: it does NOT assume that layout, it assumes the CURRENT DIRECTORY is the root, and it breaks as soon as it is not. Running from inside `docs/plans` with a bare `--source`:

```
=== default_ledger_path layout assumption: run from INSIDE docs/plans with a bare --source ===
--- PRE-FIX ---
no ledger at docs/plans/p.ledger.md; nothing to resume
--- POST-FIX ---
## RESUME STATE

rootA ledger, the right one.
```

The pre-fix build looked for `rootA/docs/plans/docs/plans/p.ledger.md` and found nothing, for a ledger that exists. It needs no root derivation at all: the ledger lives BESIDE the plan, so the source's own directory is the whole rule. That is simpler than the metrics rule and has fewer cases that can go wrong (no upward walk, no fallback branch, no convention to miss), and it is what I built.

### The lexical / canonical fork

I also built a canonicalising variant of `project_root_of_source` and ran the same matrix. It is a real fork with a measured trade in both directions.

Canonicalising FIXES two cases. Case A-bare (a bare relative filename run from inside `docs/plans`) goes from "no metrics log" to correctly reading the 3-record log, because canonicalisation gives the bare filename back its real location. And a plan symlinked out of its project resolves to the real project.

Canonicalising BREAKS output stability on the correct case. Every resolved path becomes absolute even when the user typed a relative source:

```
=== CANONICAL: no-regression case, own project from worktree root ===
/home/jessea/.../explore-metricspath-a/docs/metrics/workflow.jsonl: 235 records, valid
docs/plans/agent-scaffold.plan.toml: 93 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs /home/jessea/.../explore-metricspath-a/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

It reads the same file, but two of the three printed lines change, and they now embed an absolute machine-specific path in output that a pre-commit hook or CI log may be matched against. It also flips case H (a symlinked `docs/plans` DIRECTORY) from 37 records to 31, following the physical location rather than the path the user typed; there I think the lexical answer is the better one.

I resolved the fork by keeping the DEFAULT lexical (so display stays relative and stable) and making the GUARD canonical (so it cannot be spoofed). That gets the safety benefit of canonicalisation without the output change. It leaves case A-bare unfixed; see "Where the mechanism is still wrong" below.

## The false pass is dead

Three distinct false passes exist. I reproduced each, and measured the mechanism against each.

**False pass 1, the sidecar's: default path, foreign plan.** Killed by (a) alone.

```
=== scaffolded fixture, borrowed slug, default metrics ===
no metrics log at .../.scratch/fixture/docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
.../.scratch/fixture/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
exit: 0
```

No `workflow invariants hold`. Exit stays 0, which is defect A and out of scope for this increment by design. When the foreign project HAS a log of its own, the anchored default produces not merely the absence of a green but the correct red:

```
$ agent-scaffold validate --source "$C/rootA/docs/plans/p.plan.toml" --workflow
.../rootA/docs/plans/p.plan.toml vs .../rootA/docs/metrics/workflow.jsonl: Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records and no covering waiver; ...
exit: 1
```

**False pass 2, an explicit relative `--metrics`.** NOT killed by (a). This is the case the mandate told me to hunt, and it is a full false pass, not a wrong-file warning:

```
=== ROUTE B build ((a) only): borrowed slug + EXPLICIT RELATIVE --metrics, from the worktree root ===
docs/metrics/workflow.jsonl: 235 records, valid
.../rootA/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
.../rootA/docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

**False pass 3, a symlinked plan source.** NOT killed by (a), and not killed by a lexically-rooted (b) either. A symlink at `cases/away/p.plan.toml` pointing at rootA's plan, with a full log sitting beside the SYMLINK:

```
=== symlinked source + DEFAULT metrics + a full log beside the SYMLINK ===
.../cases/away/docs/metrics/workflow.jsonl: 235 records, valid
.../cases/away/p.plan.toml: 1 steps, 0 questions, valid
.../cases/away/p.plan.toml vs .../cases/away/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

rootA's real project has 3 records and no evidence for that slug. The symlink carries its lexical parents with it, so the anchor and a lexical guard agree with each other and both are wrong.

**All three, against the final (a)+(b) build with a canonically-rooted guard:**

```
=== B. symlink + default metrics (was a false pass) ===
--workflow would join .../cases/away/p.plan.toml against .../cases/away/docs/metrics/workflow.jsonl, which is not under the plan's project root .../cases/rootA; pass a --metrics under that root, or run against the plan's own log
exit: 1

=== C. explicit relative --metrics ===
--workflow would join .../cases/rootA/docs/plans/p.plan.toml against docs/metrics/workflow.jsonl, which is not under the plan's project root .../cases/rootA; pass a --metrics under that root, or run against the plan's own log
exit: 1

=== D. scaffolded fixture, borrowed slug, default ===
no metrics log at .../.scratch/fixture/docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
.../.scratch/fixture/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
exit: 0
```

The guard also resists a `..` escape. Reaching agent-scaffold's own 235-record log by climbing out of rootA with `../../../../../`:

```
=== .. SPOOF: escape the root via .. to reach agent-scaffold's own 235-record log ===
--workflow would join .../rootA/docs/plans/p.plan.toml against .../rootA/docs/metrics/../../../../../docs/metrics/workflow.jsonl, which is not under the plan's project root .../rootA; ...
exit: 1
```

## No regression on the correct case

From the worktree root, byte-identical to the pre-fix binary:

```
=== POST-FIX: own project, from worktree root ===
docs/metrics/workflow.jsonl: 235 records, valid
docs/plans/agent-scaffold.plan.toml: 93 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0

=== PRE-FIX same command ===
docs/metrics/workflow.jsonl: 235 records, valid
docs/plans/agent-scaffold.plan.toml: 93 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The plan render is also unaffected:

```
$ cargo run -q -- render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
exit: 0
```

Plain `validate` without `--workflow` does not trigger the guard, which is correct because no pairing is asserted:

```
$ agent-scaffold validate --source "$C/rootA/docs/plans/p.plan.toml" --metrics docs/metrics/workflow.jsonl
docs/metrics/workflow.jsonl: 235 records, valid
.../rootA/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
exit: 0
```

I also confirmed the existing integration tests are unaffected by construction: all five `--metrics` uses in `tests/` pass an explicit path, so the anchoring never fires for them.

## The full suite

```
$ cargo test
running 373 tests
test result: ok. 373 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s
test result: ok. 5 passed; 0 failed; ...
test result: ok. 1 passed; 0 failed; ...
test result: ok. 1 passed; 0 failed; ...
test result: ok. 3 passed; 0 failed; ...
test result: ok. 1 passed; 0 failed; ...
test result: ok. 2 passed; 0 failed; ...
=== totals ===
passed: 386  failed: 0
```

386 passed, 0 failed.

```
$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.10s
```

Clean, no warnings.

**An environment finding that cost me time.** With `TMPDIR` pointed at `.scratch` INSIDE the worktree, as the TMPDIR discipline asks, three tests fail:

```
failures:
    checks::tests::a_non_repo_target_with_runnable_checks_errors
    tests::init_plan_defaults_to_git_and_skips_inside_a_repo
    tests::install_precommit_hook_skips_a_non_repo
test result: FAILED. 370 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s
```

with, for example:

```
thread 'tests::init_plan_defaults_to_git_and_skips_inside_a_repo' panicked at src/main.rs:1885:9:
assertion `left == right` failed
  left: SkipExists
 right: Init
```

These three tests build a scratch directory under `std::env::temp_dir()` and require it NOT to be inside a git repository. A worktree-internal `TMPDIR` is inside one. The failures are entirely an artifact of the temp directory's location and have nothing to do with this change; with `TMPDIR` set to a directory outside any repository, all 386 pass. Anyone following the TMPDIR discipline on this repository will hit this, so either the discipline needs an exception for `cargo test`, or those three tests need to stop depending on the ambient repository state.

## Where the mechanism is still wrong, and what did not break

I hunted deliberately. Found:

**1. A bare relative filename run from inside `docs/plans` resolves to a path that does not exist, and the run greens.** The guard does not catch it, because the wrong path is still INSIDE the right project:

```
$ cd "$C/rootA/docs/plans" && agent-scaffold validate --source p.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
p.plan.toml: 1 steps, 0 questions, valid
exit: 0
```

rootA's own 3-record log is never read. This is not a regression (the pre-fix build was identically wrong here) but the mechanism does not fix it, and the guard structurally cannot: containment is not correctness. The canonicalising variant DOES fix it, at the output-stability cost described above. After inc2 lands this case becomes a hard failure rather than a silent green, which is arguably the right outcome anyway.

**2. A symlinked `docs/plans` directory is now REFUSED.** Case H, where `rootH/docs/plans` is a symlink to `rootH/elsewhere`, goes from reading the 37-record log to `exit=1 REFUSED`. The lexical default and the canonical guard disagree about which project the plan belongs to, and the guard wins. This is a genuine new failure for a layout that previously worked, and it is the main false-refusal risk in the mechanism. I judge a loud refusal better than a silent wrong file, but it is a real cost and it is not hypothetical.

**3. A deliberately shared log stays refused, which may or may not be wanted.** Any setup that points one project's `--metrics` at a log outside its own root now exits 1. The sidecar's stated end property requires this, but it is a behaviour break for anyone doing it on purpose.

**4. `--source` pointing at a DIRECTORY produces a nonsense path in a stderr note.** `--source rootA/docs/plans` derives `rootA/docs/docs/metrics/workflow.jsonl`. The run still fails with the pre-existing `Error: Os { code: 21, kind: IsADirectory }` and exit 1, exactly as pre-fix, so this is cosmetic noise before an unchanged failure, not a new failure mode.

Tried and did NOT break it:

- A relative `--source` from anywhere: anchors correctly and keeps the relative display.
- A `..` inside the source path (`docs/plans/../plans/p.plan.toml`): correct, because `file_name` returns `None` for `..` and the walk continues past it.
- A `..` inside an explicit `--metrics` that stays within the project: allowed, correct W3 red.
- A `..` inside an explicit `--metrics` that escapes the project: refused (shown above).
- A symlinked CURRENT DIRECTORY with a relative source: no false refusal, correct W3 red. `std::env::current_dir` returns the physical path, so both sides of the comparison agree.
- A source outside any git repository, and a source inside a nested git repository: both correct.
- A source whose parent chain contains a second `docs/plans`: nearest-wins, defensible.
- A typo'd `--source` that does not exist: unchanged, still the pre-existing hard error `--workflow requested but no plan source resolved`.
- A `--plan` (Markdown) anchor with no `--source`: anchors off `--plan` correctly.
- No `--source` and no `--plan` at all: falls back to the CWD-relative path, unchanged.

## The sibling commands

The sidecar says of `status`, `next` and `default_ledger_path` (line 145): "Both are best-effort projections rather than validators, so a wrong path there yields an empty projection rather than a false assertion, which is why they are separable."

**That is wrong, and the separability argument built on it does not survive.** `next` reads the round log to project the ACTIVE LOOP: the round number, the convergence state, the clean-round streak, the role to spawn next, and the instruction. Given a foreign log it does not degrade to empty, it emits a confident and completely fabricated instruction. Same plan, same command, pre-fix and post-fix, on a project whose own log is EMPTY:

```
=== next on a project with an EMPTY log of its own ===
--- PRE-FIX (cwd=worktree, reads agent-scaffold's 235-record log) ---
task: n
source: .../cases/rootN/docs/plans/n.plan.toml
metrics: 235 records

ACTIVE LOOP
  triager-runs-only-on-findings / triager-runs-only-on-findings-inc1  in progress -> mark-step-complete
  state: converged
  streak: 1/1
  rounds: 2/5
  isolation: unknown
  next: mark the step complete, re-render, and commit
  role: orchestrator
  prompt: .agents/prompts/orchestrator.md

--- POST-FIX (reads rootN's own empty log) ---
task: n
source: .../cases/rootN/docs/plans/n.plan.toml
metrics: 0 records

ACTIVE LOOP
  triager-runs-only-on-findings  in progress -> record-round
  state: awaiting-first-review
  streak: 0/?
  rounds: 0/5
  isolation: unknown
  next: spawn a reviewer for the first review round
  role: reviewer
  prompt: .agents/prompts/reviewer.md
```

`state: converged`, `rounds: 2/5`, `next: mark the step complete, re-render, and commit`, for a project with zero rounds. This is worse than the `validate` defect, not milder. `validate --workflow` prints a green a human may not read; `next` hands an agent a direct instruction to close an unreviewed step, and an agent WILL act on it. The mechanism must extend to `next`, and the same anchoring rule is the right treatment. I found no reason to give the projections different treatment: they want the same right answer, and the argument for a weaker rule rested on a claim that does not hold.

`status` is milder but still wrong, reporting a foreign record count:

```
--- PRE-FIX, cwd=worktree ---   plan: 1 steps (1 complete); 0 open-questions items / metrics: 235 records
--- POST-FIX, cwd=worktree ---  plan: 1 steps (1 complete); 0 open-questions items / metrics: 3 records
```

`status --resume` is a third case, and it does not merely count wrong, it prints another project's content verbatim. With a decoy `docs/plans/p.ledger.md` in the current directory:

```
=== PRE-FIX status --resume ===
## RESUME STATE

WORKTREE ledger, the WRONG one.

=== POST-FIX same ===
## RESUME STATE

rootA ledger, the right one.
```

I did NOT extend the (b) guard to `status` and `next`. For those two the anchoring alone is right: they are genuinely best-effort, an explicit `--metrics` there is a user pointing at a file they want counted, and a hard refusal in a projection command would be a worse trade than in a validator. That IS a real difference between the validator and the projections, and it is a different one from the reason the sidecar gives.

## The SE-3 documentation half

I verified the gap by running rather than reading the pack. The NON-instrumented scaffold's `AGENTS.md` still contains, verbatim:

```
$ grep -o "the deterministic \`validate --workflow\` check, once built, is the backstop[^.]*\." "$SCRATCH/AGENTS.md"
the deterministic `validate --workflow` check, once built, is the backstop that the required reviewed rounds happened before a step is marked complete.
```

and mentions `workflow.jsonl` twice, in "when instrumentation is on" clauses. So a user who scaffolds without `--instrument` reads a promise of a deterministic backstop, gets a `docs/metrics/` directory that does not exist, and gets exit 0 from the check that was supposed to provide it. The gap is real and reproduced.

This mechanism adds a second documentation obligation on the same surface: nothing in the scaffolded guidance or the README states where the tool looks for the log. That did not matter while the answer was "the current directory"; it matters now that the answer is "the plan's project root", and it matters a great deal once the refusal exists, because the refusal message is a new error a user can hit. `README.md:210-224` is where the `--workflow` examples live and where the anchoring note and the refusal belong. `pack/AGENTS.md:93` is where the backstop promise needs its "only when instrumented" qualifier. None of the pack's `docs/metrics/workflow.jsonl` references go stale: the log still lives at that path inside the project, and only the resolution rule for invocations from elsewhere changes.

## What this does NOT fix

The sidecar says anchoring "leaves the round record still carrying no project identity, so a deliberately or accidentally shared log remains joinable". **That is true of my build**, established by copying agent-scaffold's log into rootA's own `docs/metrics/`:

```
=== agent-scaffold's log COPIED into rootA's own docs/metrics (a deliberately shared log) ===
.../rootA/docs/metrics/workflow.jsonl: 235 records, valid
.../rootA/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
.../rootA/docs/plans/p.plan.toml vs .../rootA/docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

The guard passes (the log IS under rootA), W3 joins `triager-runs-only-on-findings` by bare slug against a record that belongs to another project, and the invariants are declared to hold for a project with no review evidence at all. Neither (a) nor (b) touches this. It needs project identity in the record and in the join, which is mechanism (d) and the queued validation-constraints step. Both (a) and (b) are path fixes; the data model is untouched.

Also unfixed, by design: the exit code stays 0 when the anchored log is missing (defect A, inc2's job).

## How big the real diff is

One file, `src/main.rs`. Measured against `HEAD`:

- Candidate (a) alone, Route B: **+79 / -15**, and most of the additions are doc comments. The executable rule is about 20 lines.
- Candidate (a) via Route A (`value_source`): **+96 / -13**, for the same behaviour plus the debug/release hazard.
- Full build, (a) + (b) + the sibling and ledger extensions: **+163 / -18**.

```
$ git diff --stat
 src/main.rs | 181 ++++++++++++++++++++++++++++++++++++++++++++++++++++++------
 1 file changed, 163 insertions(+), 18 deletions(-)
```

So (b) roughly doubles the diff. No new dependencies, no schema change, no new module, no changes to `src/workflow.rs`. A shippable version would add tests (I wrote none, see below) and the documentation updates, which I would expect to be a comparable amount again.

## What I did not verify

Stated plainly, because the evidence rule requires it:

- **I wrote no tests.** Every measurement here is a manual command run against a hand-built fixture. The red-then-green test the increment owes does not exist, and I have not checked that the cases above are expressible as tests in this repo's harness.
- **I did not run the drift guards or the checks gate** beyond `cargo test` and `cargo clippy`. I did not run `agent-scaffold checks`, and I did not regenerate `.agents/` or run `scaffold-self`.
- **I did not touch `README.md`, `CHANGELOG.md`, `pack/`, or the `run_validate` doc comment.** The documentation impact is analysed above but nothing was written. The `run_validate` doc comment at `src/main.rs:791-816` is now stale with respect to my change and I left it stale.
- **I did not test on Windows or macOS.** The rule compares path components as `OsStr` against `"plans"` and `"docs"`, which is case-sensitive; on a case-insensitive filesystem a `Docs/Plans` layout would not match. Untested.
- **I did not test a non-UTF-8 path**, nor a path containing a newline, nor a very deep path.
- **I did not measure performance.** The guard calls `canonicalize` up to a few times per `--workflow` run; I did not time it, though it is plainly negligible against reading a 235-record log.
- **I did not check what happens under a git worktree whose `.git` is a file** in a way distinguishable from the ordinary case, because the rule never consults git at all. I state that it is git-agnostic from having read the rule I wrote and from cases F and G behaving identically, not from an exhaustive git-layout sweep.
- **I did not resolve case C (nested `docs/plans`) against any real-world usage.** Nearest-wins is my judgement, not an evidenced choice.
- **I did not verify the sidecar's claims about `Q-55`'s provenance records** or the two decision receipts it cites; I took those on trust as they are not behavioural.
- **I did not build mechanism (d)**, so my comparison of (a)+(b) against (d) rests on the sidecar's description of (d) and on the shared-log reproduction above, not on a built (d).

## Recommendation

**Ship (a), but not alone. Ship (a) and (b) together as one mechanism.**

The reasoning, against the evidence above.

Candidate (a) is correct as far as it goes and it is cheap. It makes the common case right by construction, it is a genuine no-op on the correct invocation (byte-identical output), and it fixes the reported reproduction. Build it with `Option<PathBuf>`, not `value_source`: that choice is settled by measurement, not preference, because the `value_source` route is larger, prints a help string that is now false, and silently ignores the user's flag in release builds if anyone ever renames the field.

But (a) alone does not deliver the property the sidecar calls "what done means for this half": "`validate --workflow` must never pair a plan source with a metrics log belonging to a different project and report success." I built (a), and then produced two further false passes against my own build, one with an explicit relative `--metrics` and one through a symlinked source. Both print `workflow invariants hold` and exit 0 for a foreign project with no review evidence. That is the same defect the step exists to remove, reached by a slightly different route. Shipping (a) alone would close the reproduction in the sidecar and leave the defect class open, which is the worst outcome available here, because the next person will reasonably believe it was fixed.

The guard (b) closes both. The sidecar treats (a) and (b) as alternatives and notes that (b) "still needs the same root derivation as (a), so it does not avoid the hard part". Having built both, I think that framing is what led to them being posed as a choice: they are not alternatives, they are the two halves of one fix. (a) decides which log to read; (b) decides whether the log you were told to read is allowed. Neither substitutes for the other, and because they share `project_root_of_source` the second is cheap once the first exists (about 80 lines, half of them comments). The sidecar's objection to (b) standing alone, that it "leaves the user to pass `--metrics` by hand on every cross-directory run", is answered by (a) doing the anchoring, which is precisely why they belong together.

Conditions I would put on shipping it:

1. **The `docs/plans` symlink refusal is accepted, or the fork is reopened.** Case H is a real behaviour break for a layout that works today. I think a loud refusal beats a silent wrong file (it is what Fail loudly asks for), but this is a judgement about a user-visible break and it should be made deliberately rather than discovered.
2. **The lexical/canonical choice is made deliberately.** I chose lexical default plus canonical guard, which keeps output stable and leaves the bare-filename-from-`docs/plans` case unfixed. Canonicalising the default fixes that case and changes the printed metrics path to absolute on every run. Both are defensible; the second is the kind of change that should not happen as a side effect.
3. **The sibling commands go in the same change, not a later one.** This is the point I would press hardest. The sidecar separated them on the grounds that a wrong path there "yields an empty projection rather than a false assertion", and that is false: `next` told me `state: converged`, `rounds: 2/5`, `next: mark the step complete` for a project with zero rounds, reading another project's log. A validator's false green misleads a human who may not be looking. `next`'s false instruction is consumed by an agent that will act on it. If anything the projections are the more urgent half, and leaving them for later means the path fix lands while the tool continues to instruct agents to close unreviewed steps. `default_ledger_path` belongs in the same change and is the easiest of the three: the ledger lives beside the plan, so it needs no root derivation at all.
4. **The documentation goes in the same change.** The anchoring rule and the new refusal are user-visible and currently undocumented anywhere. The `SE-3` half (`pack/AGENTS.md:93` promising a backstop a non-instrumented project does not have) is a separate gap that this change does not create, but it sits on the same surface and the `--instrument` qualifier is a one-sentence fix while that text is being touched.
5. **The three environment-sensitive tests are dealt with.** `cargo test` fails 3 of 386 when `TMPDIR` is inside a git repository. That is a trap for anyone following the TMPDIR discipline on this repo, and it is unrelated to this change.

What I would NOT do: reach for mechanism (d) now. It is the better long-term architecture and it is the only thing that fixes the shared-log case reproduced above, but that case requires someone to put a foreign log inside their own project, whereas the three false passes here happen by standing in the wrong directory. (a)+(b) is the right size for the defect at hand, and it does not foreclose (d).
