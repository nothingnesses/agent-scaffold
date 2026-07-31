### `test-tmpdir-repo-assumption`: three tests read the ambient repository state through `TMPDIR`, so following this project's own scratch-`TMPDIR` discipline makes a correct tree fail

A suite defect, not a product defect. Three of the 386 tests require the directory `std::env::temp_dir()` returns to be OUTSIDE any git repository. `temp_dir()` honours `TMPDIR`, and this project instructs every spawned agent to point `TMPDIR` at a scratch directory, which agents naturally place inside their worktree, which is a git repository. The three tests then fail on a tree with nothing wrong with it.

This is a FALSE RED, which is the mirror of the false-green family most of this plan is about, and its cost is the same shape one step removed: a suite that fails for reasons unrelated to the change under test teaches its readers to classify failures as spurious, and that habit is how a real failure eventually gets waved through. It also costs agent time directly, measured below.

Scheduled as backlog behind the release gates. It blocks nothing and nothing blocks it; it is deliberately NOT a dependency of `rename-to-agent-flow` or of `workflow-enforcement-tier`. Scheduled by human decision (2026-07-31) against the standing backlog, with no `[[question]]` registered and no decision receipt owed, on the precedent of the 2026-07-30 release-gated resequencing.

## The three tests, and why each assertion is reasonable in itself

None of the three is wrong about the product. Each pins a real behaviour, and the behaviour it pins is one this project deliberately has.

- `checks::tests::a_non_repo_target_with_runnable_checks_errors` (`src/checks.rs:1477-1486`). Asserts `run(&dir, Isolation::WorkingTree)` returns `Err(RunError::NotARepo(_))`. Reasonable: `checks` runs its lint and format commands inside a throwaway git worktree so an in-place formatter can never mutate the live tree, and a target that is not a repository has no worktree to make, so erroring rather than silently running in place is the whole safety property.
- `tests::init_plan_defaults_to_git_and_skips_inside_a_repo` (`src/main.rs:1735-1751`). Its FIRST assertion is the ambient-dependent one: `init_plan(Vcs::Git, &root)` must be `InitPlan::Init` in a fresh non-repo directory. The later assertions (a directory with a `.git`, and a subdirectory of one, both give `SkipExists`) construct their own condition and are unaffected. Reasonable: scaffolding into a populated repo must never re-initialise it (Safe on existing projects), and the `Init` case is the other half of that pair; without it the test would pass while `init_plan` skipped unconditionally.
- `tests::install_precommit_hook_skips_a_non_repo` (`src/main.rs:2324-2335`). Asserts `install_precommit_hook(&root)` returns `HookInstall::Skipped(reason)` with the reason containing "not a git repository". Reasonable: the hook install is create-if-absent and must degrade to a noted skip rather than failing the scaffold, and this is the case that pins the note.

Observed failures, verbatim, from an explorer running with `TMPDIR` inside its worktree:

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

## The mechanism

Two separate scratch helpers, both rooted at the ambient temp directory:

- `src/checks.rs:1037-1046`, `fn scratch(name)`, builds `std::env::temp_dir().join(format!("agent-scaffold-checks-test-{pid}-{name}"))`.
- `src/main.rs:1725-1733`, a second `fn scratch(name)`, builds `std::env::temp_dir().join(format!("agent-scaffold-poc-{pid}-{name}"))`.

`std::env::temp_dir()` honours `TMPDIR` on Unix. The three tests then exercise code whose repository detection walks UP from the given directory, so a `TMPDIR` anywhere inside a worktree, including a linked worktree, resolves to a directory that IS inside a repository and the not-a-repo precondition silently does not hold. Nothing in the tests states the precondition, so the failure presents as a product assertion failing rather than as an environment mismatch, which is why every agent that hits it spends time deciding whether it broke something.

Note the contrast already present in the same file: `checks::tests::an_empty_repo_with_no_commits_errors` (`src/checks.rs:1488-1498`) calls `init_repo(&dir)` to CONSTRUCT the git condition it needs rather than inheriting it. The suite therefore already has the pattern for the positive case; only the negative case was left to ambient state.

## The evidence, and the tension that is the actual defect

Not hypothetical. On 2026-07-31 the metrics-path design pass dispatched three worktree-isolated explorers, each briefed per the project's own discipline to point `TMPDIR` at a scratch directory inside its worktree. ALL THREE HIT THIS, all three correctly isolated it as unrelated to their changes, and all three reported it unprompted; one routed around it by using the session scratchpad, outside any repository, for `cargo test` while keeping the worktree-local directory for build artefacts and fixtures. No result was corrupted, and the cost was three agents' time on a trap the brief set. The ledger records it as an orchestrator defect with the cure "brief a scratch `TMPDIR` OUTSIDE any git repository", and adds that "the collision is also a real property of the suite and is now recorded as such".

THE TENSION IS THE DEFECT, and it is why the cure alone is not the fix. The scratch-`TMPDIR` discipline exists for a measured reason: an earlier reviewer left 32272 directories in `/tmp`, and the standing rule that followed is to brief every agent to use a scratch `TMPDIR` and report its `/tmp` count. So the project has two disciplines that contradict each other on the same variable: use a scoped scratch `TMPDIR` so agents cannot litter, and do not put `TMPDIR` anywhere the suite can see a repository. Every agent briefed for this repository has to be told about the collision by hand, which is a rule that lives in prose and gets forgotten rather than in the thing that enforces it. Removing the tension means the suite stops depending on where `TMPDIR` points at all.

## Fix options, not pre-decided

The implementer picks one and argues it in the commit, or raises the choice if it reads as a genuine fork. Both are viable and they trade differently.

- (A) MAKE THE PRECONDITION DETERMINISTIC rather than ambient. Have the three tests construct a directory that is provably outside any repository instead of assuming one, for example by asserting the precondition explicitly and failing with an environment message when it does not hold, or by giving git an explicit boundary (`GIT_CEILING_DIRECTORIES`, or a `.git` sentinel arrangement) so the upward walk stops before it leaves the scratch directory. Keeps all three assertions intact and has the in-file precedent noted above (the positive case already constructs its condition). The risk to watch is that a boundary applied carelessly makes the not-a-repo condition hold BY CONSTRUCTION in a way that would also hold if the product stopped detecting repositories at all, which turns a real test into a vacuous one.
- (B) CHANGE WHAT THE TESTS ASSERT, dropping or weakening the not-a-repo cases. Cheapest, and wrong on current reading: each assertion pins a real safety property (do not run checks outside a worktree, do not re-initialise an existing repo, do not fail the scaffold over an uninstallable hook), so weakening them buys suite determinism by deleting coverage. Recorded so the option is visibly considered and visibly rejected rather than silently unconsidered.
- (C) LEAVE THE TESTS AND FIX THE BRIEF, that is, make the scratch-`TMPDIR` rule say "outside any repository" everywhere it appears. This is what is being done today by hand. It is not a fix: it keeps a suite whose result depends on an environment variable, and it relies on prose that every future brief must remember, which is precisely the class of rule this project keeps finding drifts.

Recommendation: (A), with (B) rejected on coverage and (C) rejected as a workaround already in force. State which of (A)'s two routes was taken and why.

## Risk classification

`test-tmpdir-repo-assumption-inc1` is `low_risk` (one clean review round). The change is confined to `#[cfg(test)]` code in two files, touches no product behaviour, ships nothing to a scaffolded project, and is reversible in a single revert. It differs in kind from the other test-isolation step in this plan, `checks-runner-worktree-name-collision` (order 93, `risky`), which changes how `src/checks.rs` NAMES worktrees and so touches production code.

The one failure mode that would justify a second round is a test made to pass by construction rather than by the property holding, which is the false-green shape this project cares most about, appearing inside the suite itself. It is foreclosed cheaply rather than by an extra round: `Q-66` already requires evidence proportional to the claim, and the proportionate evidence here is a MUTATION DEMONSTRATION, breaking each of the three product behaviours in turn and showing the corresponding test goes RED under the new arrangement. A round report that shows three reds and three greens has answered the only question the classification turns on, and a second clean round would add nothing to it. If the implementer cannot produce those three reds, the classification was wrong and the step should be re-classified before it converges rather than waived afterwards.

## Acceptance check

1. `cargo test` passes with `TMPDIR` set INSIDE a git repository (a worktree-local scratch directory), which is the case that fails today. 386 expected, 0 failed.
2. `cargo test` still passes with `TMPDIR` set OUTSIDE any repository, and with `TMPDIR` unset, so the fix removes the dependency rather than inverting it.
3. `cargo test` passes with `TMPDIR` set inside a LINKED worktree specifically, not only inside a primary checkout, since that is the arrangement every isolated agent on this project actually runs in.
4. The mutation demonstration, one per test, each producing a named RED against the new arrangement: make `run` accept a non-repository target, make `init_plan` skip unconditionally, and make `install_precommit_hook` report something other than the not-a-repository skip. Each corresponding test must fail. The round report names the mutation and quotes the failure.
5. `cargo clippy --all-targets -- -D warnings` clean.
6. No production behaviour changed: the diff touches only `#[cfg(test)]` code, or, if it does not, the step is re-classified before converging.
7. Neither scratch helper leaves directories behind: check the `/tmp` (or `TMPDIR`) entry count before and after a full run and report it, which is the discipline whose collision with the suite this step exists to remove.

## Scope

- It does not change `src/checks.rs`'s worktree NAMING, which is the separate defect held by `checks-runner-worktree-name-collision` (order 93, `deferred`). The two are both test-isolation defects and are deliberately separate: that one is a real race in production-reachable code, this one is an environment assumption in test setup.
- It does not consolidate the two `scratch` helpers into one. They live in different modules and differ in their prefix and their cleanup, and merging them is a refactor this defect does not require (Minimal by default). If the chosen fix makes a shared helper natural, say so and argue it rather than doing it silently.
- It does not change the project's scratch-`TMPDIR` discipline or any agent brief. Those stay as they are; what changes is that following them stops breaking the suite.
