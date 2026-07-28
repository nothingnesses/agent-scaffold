### `checks-runner-worktree-name-collision`: make the `checks` runner worktree name unique per call

Deferred. This is a defect fix in `src/`, not a design change: it closes a mechanism verified in the code and observed failing, so it carries no `[[question]]` and no decision receipt. The human's call (2026-07-28) was to TRACK it here rather than schedule it now.

THE MECHANISM, verified in this tree rather than taken from the report.

`run()` names its throwaway worktree from two components, `src/checks.rs:791-792`:

```
let worktree_path =
	std::env::temp_dir().join(format!("{RUNNER_PREFIX}{}-{}", std::process::id(), nanos()));
```

`RUNNER_PREFIX` is the constant `agent-scaffold-checks-run-` (`src/checks.rs:78`), so the full name is `{temp}/agent-scaffold-checks-run-{pid}-{nanos}`. `nanos()` (`src/checks.rs:848-852`) is a bare clock reading with no uniqueness guarantee:

```
std::time::SystemTime::now()
	.duration_since(std::time::UNIX_EPOCH)
	.map_or(0, |elapsed| elapsed.as_nanos())
```

Its own doc comment (`src/checks.rs:845-847`) states the premise that fails: "the process id in the path already provides per-process uniqueness". That is true ACROSS processes and false ACROSS THREADS OF ONE PROCESS. Cargo runs a crate's unit tests as threads of a single test binary, so `std::process::id()` is constant for every test in `checks::tests` and the clock reading is the only thing separating two concurrent `run()` calls.

The clock is not fine-grained enough to carry that alone. Measured on the development machine with a standalone `rustc -O` probe calling the same expression as `nanos()`: consecutive reads on one thread never repeat but advance by a median of 30 ns (minimum 20 ns) over 100000 samples, and two threads sampling concurrently after a shared `Barrier` produce equal values for 8679 of 100000 samples (about 8.7%); at 16 threads it is 568127 of 800000. So the discriminator carries roughly 25 ns of resolution, not 1 ns, and two threads reading it at the same moment agreeing is ordinary rather than exotic.

What a collision then does. Both runs pass the same path to `git worktree add`. Git accepts an EXISTING EMPTY directory as a worktree target and refuses only a non-empty one (verified against git 2.51.2: `git worktree add --detach <pre-created empty dir> HEAD` succeeds; the same command on a directory containing a file fails with `fatal: '<path>' already exists`). Its existence check is not atomic with its creation of the worktree's `.git` pointer file, so two adds issued from DIFFERENT repositories at the same path can both proceed and the later `.git` file overwrites the earlier one. The losing run's isolated tree then points at the winning run's repository, and everything downstream acts on the wrong repo: the tracked-file scan, the check commands, and the `WorktreeGuard` cleanup at `src/checks.rs:329-342`, which unregisters and `remove_dir_all`s the shared directory while the other run is still inside it. The recorded failure text below (both tests citing one path, each error naming the OTHER test's fixture repository) is what that looks like from the outside.

EVIDENCE IT IS REAL, not theoretical.

Observed once in six consecutive runs during the step-92 (`prompt-drift-guard`) round-2 triage, quoted here verbatim because its findings file is transient and will be removed under the commit-before-delete rule (it was `docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md` at commit `9f0966c`, recoverable from history):

> My first `cargo test` in this worktree failed `checks::tests::a_format_check_never_mutates_the_live_tree` and `checks::tests::an_empty_paths_array_runs_unscoped`; five later runs passed. Both failures cited the SAME runner worktree path, `agent-scaffold-checks-run-416707-1785235883764925866`, and each error named the OTHER test's fixture repo.

The shared path is the whole proof: one pid (`416707`), one nanosecond value (`1785235883764925866`), two tests.

Corroboration, and the reason this is not the first sighting. The ledger's RESUME STATE records, from the `code-value-audit-static` inc1 review: "A rare pre-existing MAIN-binary test-parallelism flake was seen once in inc1 review (candidate backlog item on test isolation)." (quoted by text, not line, since the ledger is edited in place and line citations into it rot). That note had a symptom and no mechanism; this finding supplies the mechanism. Which tests failed on that earlier occasion is not recorded, so treat the identification as probable rather than certain; what is certain is that the recorded history now has two sightings, not one, which is the argument against filing this as noise.

SEVERITY, BOUNDED HONESTLY IN BOTH DIRECTIONS.

Production is NOT affected. Each `agent-scaffold checks` invocation is its own process, so two concurrent runs have different pids and their names cannot collide while both are alive. The concurrency case that DOES arise in production, two overlapping runs on the same repository, is already handled by the pid-liveness gate on the startup prune (`src/checks.rs:407-461`), which is not implicated here; nor is its benign pid-reuse edge (`src/checks.rs:425-428`).

The test suite IS affected. Every unit test shares one pid, so the only discriminator is the ~25 ns clock. The damage is not a shipped defect, it is that `cargo test` is not deterministically green, which corrodes the evidence base the whole review process rests on: every convergence decision in this repo reads a green suite as a settled fact, and a suite that fails about one run in six makes "379 passed" a claim about luck. Do not escalate this to a user-facing bug, and do not dismiss it as cosmetic; it sits exactly between.

SCOPE: WHERE THE DEFECT LIVES.

The collision surface is any path whose only discriminator is a clock reading. In this tree that is:

- `src/checks.rs:791-792`, the production runner path, exercised concurrently by the `checks::tests` cases that call `run()` (22 call sites in that module).
- `src/checks.rs:1462`, `src/checks.rs:1491`, and `src/checks.rs:1492`, test fixtures that build `{RUNNER_PREFIX}{pid}-{nanos}` names by hand in the SAME namespace, two of them with the live pid. They can collide with each other and with a concurrent real run, so a fix that touches only `run()` leaves the defect half-present.

Checked and NOT affected, so the step does not widen into them: the other scratch helpers discriminate by a per-test literal name rather than by the clock and so are unique by construction (`src/checks.rs:862-871`, `src/main.rs:1726-1731`, `src/manifest.rs:552-558`, `src/plan/render.rs:638`, `tests/audit_command.rs:20`, `tests/scaffold_precommit_hook.rs:14`, `tests/checks_staged_hook_env.rs:50`). Three integration-test sites do use `{pid}-{nanos}` (`tests/validate_workflow_toml_source_needs_no_plan.rs:58` and `:90`, `tests/validate_toml_primary_skips_markdown_plan.rs:74`), but each carries a distinct literal prefix, so they cannot collide today; they are a latent copy-paste hazard to note, not to fix here.

WHAT "DONE" LOOKS LIKE.

- The runner worktree path is unique per call BY CONSTRUCTION, and the argument for why is written in the code comment, since the current comment is precisely where the wrong argument was written down.
- The name is generated in ONE place and used by both `run()` and the three fixtures, so a fixture cannot drift from the production naming (plan Principle 1, prefer the cleaner long-term architecture over the smallest diff: a shared generator, not a patch to the single line that happened to be caught failing).
- `owning_pid` (`src/checks.rs:400-405`) still parses the owning pid. It takes the first `-`-separated segment after the prefix, so the pid must stay the FIRST component and any new component is appended after it; the prune's liveness gate reads nothing else.
- The three doc comments that spell the name format literally are corrected in the same change: `src/checks.rs:72-77` (`RUNNER_PREFIX`), `src/checks.rs:400-402` (`owning_pid`), `src/checks.rs:845-847` (`nanos`, the false premise).
- The uniqueness property is pinned by a test that FAILS without the fix (see the demonstration section, which is the load-bearing part of this step).

CANDIDATE FIXES AND THEIR TRADE-OFFS. Not pre-decided here; the implementer picks one and argues it, or raises the choice if it is a genuine fork.

- (a) A process-wide atomic counter, `{pid}-{nanos}-{seq}` from a `static AtomicU64`. Zero dependencies, unique by construction within a process, smallest diff, keeps the pid first. Weakness: it guarantees nothing across processes, so it leans on the pid exactly as today for that half; a stale orphan from a dead process whose pid was reused could in principle share a name, which the existing prune already treats as benign (`src/checks.rs:425-428`).
- (b) A random suffix. There is no public RNG in std, so this means adding `rand` for a filename suffix. That is out of proportion for this repo's dependency discipline, which the code itself states at `src/checks.rs:388-392` (no libc pulled in just for a `kill(pid, 0)`); the std workaround (`RandomState` / `DefaultHasher`) is obscure for what it buys. Cheapest to write, worst cost/benefit.
- (c) `std::thread::current().id()`. Zero dependencies and it targets the observed case directly. But `ThreadId` exposes no stable textual form (`as_u64` is unstable, leaving `Debug`'s `ThreadId(3)`, which is neither documented as stable nor filename-safe without sanitising), and it discriminates THREADS, not CALLS, so two calls on one thread fall back on the clock again. Strictly weaker than (a) at the same cost.
- (d) Let the filesystem enforce it: reserve the path with `std::fs::create_dir` (one `mkdir`, which fails with `AlreadyExists` atomically rather than racing) and retry with a fresh name on collision, then hand the reserved empty directory to `git worktree add`. Strongest guarantee, since uniqueness stops depending on any entropy argument and covers the cross-process and pid-reuse cases too, and zero new dependencies. Verified compatible with git: `git worktree add --detach` into a pre-created EMPTY directory succeeds (git 2.51.2), which is the objection that would otherwise sink this option. Costs a retry loop and one extra syscall. A `tempfile`-crate version of the same idea adds a dependency and buys nothing over the std form here.

(a) and (d) compose: a counter for the name plus `create_dir` as the reservation gives both a cheap unique name and a loud failure the day the uniqueness argument is wrong (plan Principle 5, make illegal states unrepresentable, applied to a path namespace).

THE DEMONSTRATION PROBLEM. This is the hardest part of the step and the part most likely to be skipped.

The reproducible-evidence rule (`Q-66`, step 88) is live, and this claim is behavioural, so the implementer owes a runnable demonstration, ideally the mutation form: break the code and show the test still passes. Plan Principle 6 (ground decisions in evidence) is the standard the demonstration is judged against, and it is not met by a green suite here, for the reason below.

"I ran `cargo test` and it was green" is exactly the non-evidence that rule exists to reject. Work the arithmetic on the one rate that was measured, one failure in six runs. Under UNCHANGED code the chance of six consecutive green runs is (5/6)^6 = 0.33, so a six-run green streak is roughly a coin flip and proves nothing. To reach 95% confidence that a 1-in-6 failure mode is gone takes 17 consecutive clean runs ((5/6)^n <= 0.05 first holds at n = 17). And that 1-in-6 is a single sample from one machine, so the true rate is unknown and varies with core count and clock granularity; the run-count argument is therefore even weaker than its own arithmetic. If a run-count claim is made at all, it must state the number of runs and the confidence bound it actually supports.

The way out is to stop sampling the probabilistic event and test the deterministic property underneath it:

- Extract name generation into a callable unit and assert uniqueness directly: N threads released together on a `std::sync::Barrier`, each generating M names, then assert the collected set holds N * M distinct entries. This is deterministic in the direction that matters, because the measured cross-thread duplicate rate (8.7% at two threads) makes a collision effectively certain at, say, N = 8 and M = 1000 against today's generator, and impossible against a by-construction-unique one.
- Show it RED before green: run that test against the unfixed generator, or mutate the fix's disambiguator to a constant, and show it fails; then restore. That mutation is what separates a test that pins the property from a test that merely passes, and it is the form `Q-66` names as strongest.
- Optional higher-fidelity extra, if the unit-level property is judged too indirect: drive two `run()` calls from two threads with the name generation forced to a common value, and show the pre-fix build produces the cross-repository `.git` corruption while the post-fix build does not. More machinery for a claim the unit test already settles; the uniqueness test plus the mutation is the proportional minimum.
- Report the measured numbers, not the word "fixed".

RELATION TO STEP 85 (`drift-guard-test-hook-hygiene`). Same family, different defect, and they must not be merged into one step.

Step 85 is a process-global panic-hook swap in `src/agents_md_drift.rs`: diagnostic-only, it cannot change any test's pass or fail, and its cost is a lost backtrace. This step is a shared-path collision in `src/checks.rs` that HAS changed test outcomes, twice in the recorded history. They touch disjoint files and neither blocks the other, so doing both in one sitting is cheap and sensible. Doing them as ONE step is not: their evidence burdens differ, since 85's fix is unobservable from test outcomes while this one owes the red/green uniqueness demonstration above, and bundling them lets 85's easy "no behaviour change" story stand in for the harder one. If both are scheduled, this one goes first, because it is the one that makes the suite lie.

DOCUMENTATION IMPACT: in-code only, and it is named above.

The three doc comments at `src/checks.rs:72-77`, `:400-402`, and `:845-847` go stale with any change to the name format and are corrected by the same implementer, so no separate documentation step is owed. Nothing scaffolded changes: the temp path is an internal implementation detail, not part of the CLI, the output, or the pack contract, so `AGENTS.md`, `pack/`, and the deployed `.agents/` copies are untouched and no regeneration is required. A grep for `agent-scaffold-checks-run` finds no hit in `README.md`, `CHANGELOG.md`, `docs/`, or `pack/`. A CHANGELOG `Fixed` entry is likely right, but the `Unreleased` section currently carries only `Added` and `Changed`, so check what a comparable internal-only fix did before assuming either way.
