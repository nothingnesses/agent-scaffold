# Review: `checks-runner-worktree-name-collision` (plan fold, deferred step, order 93)

Artifact: `git diff 9f0966c..5344095` (one `[[step]]` in `docs/plans/agent-scaffold.plan.toml`, the sidecar `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md`, and the regenerated `docs/plans/agent-scaffold.md`).
Reviewed in worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-testiso` at `5344095`.
Judged as a durable RECORD and a future BRIEF; the human's decision to defer rather than fix now is not in scope, and "the fix is not implemented" is not raised.

Four findings: one `medium`, three `low`. No `high` and no `critical`.

## 1. Citation verification (every cited line opened)

| Claim (sidecar line) | Citation | Verdict |
| --- | --- | --- |
| Name construction, quoted verbatim | `src/checks.rs:791-792` | VERIFIED. Lines 791-792 are exactly the quoted two lines. |
| `RUNNER_PREFIX = "agent-scaffold-checks-run-"` | `src/checks.rs:78` | VERIFIED. |
| `nanos()` body, quoted verbatim | `src/checks.rs:848-852` | VERIFIED. 848 is `fn nanos() -> u128 {`, 852 the closing brace; the quoted three lines are 849-851. |
| The false premise "the process id in the path already provides per-process uniqueness" | `src/checks.rs:845-847` | VERIFIED, quoted word for word. |
| `owning_pid` takes the first `-` segment after the prefix, so the pid must stay first | `src/checks.rs:400-405` | VERIFIED. Line 404 is `dir_name.strip_prefix(RUNNER_PREFIX)?.split('-').next()?.parse().ok()`. |
| Doc comment spelling the name format (`owning_pid`) | `src/checks.rs:400-402` | VERIFIED. |
| Doc comment spelling the name format (`RUNNER_PREFIX`) | `src/checks.rs:72-77` | VERIFIED, and it does spell `agent-scaffold-checks-run-{pid}-{nanos}`. |
| Liveness-gated prune, not implicated | `src/checks.rs:407-461` | VERIFIED (doc comment opens at 407, `prune_orphan_worktrees` closes at 461). |
| Benign pid-reuse edge | `src/checks.rs:425-428` | VERIFIED. |
| Dependency discipline ("no libc pulled in just for a `kill(pid, 0)`") | `src/checks.rs:388-392` | VERIFIED, quoted accurately. |
| `WorktreeGuard` cleanup unregisters and `remove_dir_all`s | `src/checks.rs:329-342` | VERIFIED (`impl Drop` 329-342; `worktree remove --force`, `remove_dir_all`, `worktree prune`). |
| Fixture building the name by hand | `src/checks.rs:1462` | VERIFIED (`{RUNNER_PREFIX}{}-{}`, `dead_pid()`, `nanos()`). |
| Fixture building the name by hand | `src/checks.rs:1491` | VERIFIED (`std::process::id()`, `nanos()`). |
| Fixture building the name by hand | `src/checks.rs:1492` | VERIFIED (`dead_pid()`, `nanos()`). |
| "two of them with the live pid" | `src/checks.rs:1462`, `:1491`, `:1492` | NOT VERIFIED. See TI-1. |
| 22 `run()` call sites in the module | `src/checks.rs` tests | VERIFIED. `grep -nE "\brun\(" src/checks.rs` returns the definition at 734 plus exactly 22 call sites (974 ... 1468). |
| Unaffected scratch helpers discriminate by a per-test literal | `src/checks.rs:862-871`, `src/main.rs:1726-1731`, `src/manifest.rs:552-558`, `src/plan/render.rs:638`, `tests/audit_command.rs:20`, `tests/scaffold_precommit_hook.rs:14`, `tests/checks_staged_hook_env.rs:50` | VERIFIED, all seven. Each builds `{distinct-literal-prefix}-{pid}-{name}` with a literal `name`. |
| Three integration sites use `{pid}-{nanos}` but carry distinct literal prefixes | `tests/validate_workflow_toml_source_needs_no_plan.rs:58`, `:90`, `tests/validate_toml_primary_skips_markdown_plan.rs:74` | VERIFIED. Prefixes are `agent-scaffold-validate-toml-only-`, `agent-scaffold-validate-workflow-no-source-`, `agent-scaffold-validate-projection-`: pairwise distinct, so they cannot collide today, as claimed. |
| Verbatim quote of the step-92 round-2 triage | `docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md:198` (present at `9f0966c` and still at `5344095`) | VERIFIED, word for word. |
| Step 85 is a diagnostic-only panic-hook swap in `src/agents_md_drift.rs` that cannot flip a pass/fail, cost is a lost backtrace | `docs/plans/agent-scaffold.steps/drift-guard-test-hook-hygiene.md:3-5` | VERIFIED ("diagnostic-only ... cannot flip any test's pass/fail", "the only impact is backtrace visibility"). |
| Plan Principle 1 = prefer the cleaner long-term architecture; 5 = make illegal states unrepresentable; 6 = ground decisions in evidence | `docs/plans/agent-scaffold.plan.toml` `[[principle]]` n=1, 5, 6 | VERIFIED. The sidecar cites the PLAN's own numbering, which is the correct one. |
| "Production is NOT affected ... each invocation is its own process" | `src/main.rs:709-710` | VERIFIED as far as it goes: `checks::run` has exactly one production call site (`grep -rn "checks::run" src/ tests/`), called once per CLI invocation, so no two `run()` calls share a pid in production. |
| Grep for `agent-scaffold-checks-run` finds no hit in `README.md`, `CHANGELOG.md`, `docs/`, or `pack/` | repo-wide grep | PARTLY NOT VERIFIED: true for `README.md`, `CHANGELOG.md` and `pack/`, false for `docs/`. See TI-4. |

Also checked and correct, though not cited as a line: the observed nanosecond value `1785235883764925866` decodes to late July 2026, consistent with the reported observation date.

## 2. Measured claims

Probes were built and run in a scratch directory outside the repo (`.../scratchpad/ti-probe/`), with `rustc -O` from the project toolchain. The machine has 16 cores (`nproc`) and was under concurrent load from other agents, which if anything inflates the duplicate rates below.

**Clock resolution (median 30 ns, min 20 ns, no repeats over 100k consecutive reads): REPRODUCED.**
A probe calling the same expression as `nanos()` 100000 times in a loop:

```
consecutive: n=100000 zero_deltas=0 min=20 median=21 max=4558
consecutive: n=100000 zero_deltas=0 min=20 median=30 max=6051
```

Min 20 ns and zero repeated consecutive values match exactly; the median is run-dependent (21 and 30 ns on two runs of the same binary). The derived statement "roughly 25 ns of resolution, not 1 ns" is sound.

**Cross-thread duplicate rate (8679/100000 at 2 threads, 568127/800000 at 16): REPRODUCED, same statistic, same magnitude, mine somewhat higher.**
The sidecar's denominators (100000 at 2 threads, 800000 at 16) imply 50000 samples per thread and a count of duplicate values over the pooled samples. Matching that (barrier-released threads, 50000 reads each, counting `total - distinct`):

```
threads=2  per=50000 total=100000 distinct=85320 excess=14680
threads=2  per=50000 total=100000 distinct=89067 excess=10933
threads=2  per=50000 total=100000 distinct=83614 excess=16386
threads=16 per=50000 total=800000 distinct=210495 excess=589505
threads=16 per=50000 total=800000 distinct=205342 excess=594658
threads=16 per=50000 total=800000 distinct=230792 excess=569208
```

16 threads: 569208 to 594658 against the reported 568127, a close match. 2 threads: 10933 to 16386 (11 to 16%) against the reported 8679 (8.7%), the same order and above it. The load-bearing conclusion ("two threads reading it at the same moment agreeing is ordinary rather than exotic") holds with margin.

**`git worktree add --detach` into a pre-created EMPTY directory succeeds on git 2.51.2, a non-empty one is refused: REPRODUCED exactly.**

```
$ git --version
git version 2.51.2
$ git -C repoA worktree add --detach <pre-created empty dir> HEAD
Preparing worktree (detached HEAD f58ad76)
HEAD is now at f58ad76 init          # exit 0
$ git -C repoA worktree add --detach <dir containing file.txt> HEAD
fatal: '<path>' already exists       # exit 128
```

**The corruption mechanism (two adds from DIFFERENT repositories at the same path can both proceed, the later `.git` overwriting the earlier): REPRODUCED, and more directly than the sidecar itself claims.**
200 iterations, each racing `git -C repoA worktree add --detach $P HEAD` against `git -C repoB worktree add --detach $P HEAD` at a fresh shared path:

```
SUMMARY both=25 exactly_one=160 neither=15 both_registered_same_path=25
```

25 of 200 races had BOTH adds return 0, with the path registered in both repositories and one surviving `.git` pointer (owner A in 21 cases, B in 4). 15 races had both fail, which is a second, noisier failure mode (`RunError::WorktreeSetup`) the sidecar does not mention but which does not change any of its conclusions. This is the sidecar's stated mechanism, demonstrated.

## 3. The probabilistic-demonstration section

**Arithmetic: correct.** `(5/6)^6 = 0.334898`, so "about 0.33" is right. `(5/6)^16 = 0.054088` and `(5/6)^17 = 0.045073`, so n = 17 is indeed the first n at which `(5/6)^n <= 0.05`, exactly as the parenthetical states. The assumption is the frequentist one: runs independent and the rate fixed at 1/6, so 17 clean runs is the point at which you can reject "the 1-in-6 mode is still present" at the 5% level. It is not a posterior probability that the defect is gone, and the sidecar's own next sentence ("that 1-in-6 is a single sample from one machine, so the true rate is unknown ... the run-count argument is therefore even weaker than its own arithmetic") makes the right caveat, so the informal phrasing "95% confidence that a 1-in-6 failure mode is gone" is not misleading in context. No finding.

**Is the proposed unit test a valid demonstration, or a proxy? Mostly valid, with two real gaps (TI-2, TI-3).**

What it gets right, and this is the important part: it replaces sampling a rare event with asserting a deterministic property, and it demands the mutation form. Against today's naming the RED direction is not a coin flip. At the exact parameters it proposes (N = 8, M = 1000) my probe found duplicates in every run:

```
threads=8 per=1000 total=8000 distinct=4455 excess=3545
threads=8 per=1000 total=8000 distinct=5466 excess=2534
```

so a `{pid}-{nanos}` generator fails the N*M-distinct assertion with margin, and a by-construction-unique generator passes it deterministically. That is a genuinely better demonstration than any number of green suite runs, and the section's central judgement is correct.

Where it is a proxy:

- It pins the GENERATOR, not the call sites, and the sidecar asserts the unit test "already settles" the claim (TI-3). The claim that matters is that `run()`'s path is unique. A `run()` that keeps the inline `format!` at `src/checks.rs:791-792` alongside a new generator leaves the proposed test green and the defect fully present. The `What "done" looks like` section does independently require one generator used by `run()` and the three fixtures, so the record as a whole carries the requirement; the demonstration section does not, and it is the section the implementer will treat as the acceptance bar.
- Its wording is coupled to the name-entropy candidates and is RED against a correct implementation of candidate (d) alone (TI-2).

## 4. Plan-mechanical checks

- `status = "deferred"`, `order = 93`, `blocked_by = []`, `folds = []`, `increment = []`, `waiver = []`: present and structurally identical to the closest precedent, step 85 (`docs/plans/agent-scaffold.plan.toml:1164-1172` versus `:1262-1270`). No `[step.provenance]`, matching step 85 and step 92, the two other steps with no decision provenance.
- The 91 gap is pre-existing, not introduced here: `git log -S"order = 91" -- docs/plans/agent-scaffold.plan.toml` shows `5fcd020` adding steps 91-92 and `8811ad1` ("docs: demote Q-69 to exploring and drop its premature step") removing 91. Taking 93 rather than back-filling 91 keeps the new step after the in-progress step 92, which is the correct reading of `order`. No duplicate orders anywhere: `grep "^order = " | sort -n | uniq -d` is empty.
- No `[[question]]` and no `type: "decision"` receipt: correct. W4 keys on decided `[[question]]` items (`AGENTS.md:145`, `src/workflow.rs:288`), and this step folds no question, so no receipt is owed; step 85 sets the same precedent (`grep folded_into` matches neither slug).
- No slug collision with step 85, and the sidecar's stated relationship to it is accurate: different file (`src/agents_md_drift.rs` versus `src/checks.rs`), different severity class (85 cannot flip a pass/fail per its own sidecar line 5; this one has), so "same family, different defect, must not be merged" is a fair reading, and the differing evidence burdens are a real reason not to bundle them.
- The title names the actual defect (the `{pid}-{nanos}` path collision and its observed symptom), not a solution.
- `cargo run -- render docs/plans/agent-scaffold.plan.toml --check --strict` -> `up to date`, exit 0.
- `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` -> `216 records, valid` / `92 steps, 69 questions, valid` / `workflow invariants hold`, exit 0. Matches the expected 92 / 69 / 216.
- The regenerated `docs/plans/agent-scaffold.md` updates the Status line to `92 steps (... 19 deferred)` and adds the Roadmap row with an empty third column, consistent with the other two provenance-free steps.
- `grep -P "[^\x00-\x7F]"` over the sidecar: no non-ASCII.
- `cargo test` on this tree: 378 passed, 0 failed (this run was green; see the note below).

## Findings

### TI-1 (`medium`): "two of them with the live pid" is false, and the error inverts which fixture sites have cross-process protection

Sidecar line 49 (rendered `docs/plans/agent-scaffold.md:1328`):

> `src/checks.rs:1462`, `src/checks.rs:1491`, and `src/checks.rs:1492`, test fixtures that build `{RUNNER_PREFIX}{pid}-{nanos}` names by hand in the SAME namespace, two of them with the live pid.

Only ONE of the three uses the live pid. `src/checks.rs:1491` uses `std::process::id()`; `src/checks.rs:1462` and `src/checks.rs:1492` use `dead_pid()`, which is the constant `u32::MAX` (`src/checks.rs:1438-1442`: `let pid = u32::MAX;`). So the three names are:

- `:1462` -> `agent-scaffold-checks-run-4294967295-{nanos}`
- `:1491` -> `agent-scaffold-checks-run-{live pid}-{nanos}`
- `:1492` -> `agent-scaffold-checks-run-4294967295-{nanos}`

Why this matters for the brief rather than being a typo. The whole severity argument of the step turns on the pid being the cross-process discriminator ("Production is NOT affected. Each `agent-scaffold checks` invocation is its own process, so two concurrent runs have different pids"). For `:1462` and `:1492` there is NO pid discriminator at all: the pid component is a compile-time constant, so their only discriminator is the ~25 ns clock both WITHIN a process and ACROSS processes, including two `cargo test` runs in two worktrees at once, which is this project's normal working state. `git_ok` at `src/checks.rs:1463` and `:1493-1494` asserts the `git worktree add` succeeds, so a collision there is a hard test failure, not a silent one; and per the race demonstration in section 2, a same-path add from two repositories can also both succeed and cross-link the trees.

The consequence for the fix: candidate (a) (a process-wide `AtomicU64`) is the sidecar's leading option and, by its own words, "guarantees nothing across processes". Combined with the `What "done" looks like` requirement that all four sites share ONE generator, an implementer who believes the fixtures carry the live pid can pick (a), satisfy every stated done-condition, and leave `:1462` and `:1492` with no cross-process uniqueness whatsoever. Stated correctly, the two constant-pid fixtures are an argument FOR the reservation-based candidate (d), or for the composition of (a) and (d) the sidecar already prefers.

Evidence: `src/checks.rs:1438-1442`, `:1462`, `:1491`, `:1492` (citations settle the factual error). For the cross-process consequence, the git race in section 2 (`both=25 / 200`). I also tried to reproduce an end-to-end cross-process failure directly: 40 paired concurrent runs of `agent_scaffold-<hash> checks::tests::a_startup_prune --test-threads=2` in two processes produced 0 failures. That is NOT evidence against the mechanism, it is the step's own point about sampling a rare event: two independently started processes are far less aligned than two threads released together, so the rate is too low to sample at n = 40. I report it rather than omit it.

Suggested correction: "two of them (`:1462`, `:1492`) with a CONSTANT pid (`u32::MAX`, from `dead_pid()`), so they carry no cross-process discriminator at all, and one (`:1491`) with the live pid, so it shares the production namespace."

### TI-2 (`low`): the prescribed demonstration is RED against a correct implementation of candidate (d) alone

Sidecar line 78: "Extract name generation into a callable unit and assert uniqueness directly: N threads released together on a `std::sync::Barrier`, each generating M names, then assert the collected set holds N * M distinct entries."

Candidate (d) (sidecar line 66) explicitly keeps a clock-derived name and gets uniqueness from the filesystem: "reserve the path with `std::fs::create_dir` ... and retry with a fresh name on collision". Under (d) alone the NAME generator still returns `{pid}-{nanos}` and still produces duplicates at exactly the measured rate, so the prescribed assertion fails against a correct implementation of the option the section deliberately leaves open. My probe at the section's own parameters:

```
threads=8 per=1000 total=8000 distinct=4455 excess=3545
threads=8 per=1000 total=8000 distinct=5466 excess=2534
```

(i.e. a raw `{pid}-{nanos}` generator yields 2534-3545 duplicate values out of 8000 at N=8, M=1000.)

So the demonstration is presented as fix-independent ("Not pre-decided here; the implementer picks one") but is written only for the name-entropy candidates (a) and (c). An implementer who picks (d) sees a red test against correct code and has to work out that the unit under test must be the RESERVING call, not the name string. Cheap correction: define the callable unit as whatever returns the final path (the name generator under (a)/(c), the reserve-and-retry call under (d)), which is also what makes the mutation ("mutate the disambiguator to a constant") meaningful under either.

### TI-3 (`low`): the demonstration pins the generator, not the call sites, yet is described as settling the claim

Sidecar line 80: "More machinery for a claim the unit test already settles; the uniqueness test plus the mutation is the proportional minimum."

The unit test settles "the generator returns distinct values under concurrency". The step's claim is "`run()`'s worktree path is unique per call" (and the three fixtures' too). Nothing in the prescribed test observes `src/checks.rs:791-792`: a build that adds the generator but leaves the inline `format!("{RUNNER_PREFIX}{}-{}", std::process::id(), nanos())` in `run()` passes the unit test with the defect fully intact. That is the shape Principle 11 (tests must actually exercise the code they claim to) names, and it is exactly the state the tree is in TODAY for the three fixtures, which the sidecar itself documents at line 49: hand-built names that no generator-level test would notice.

This is a gap in the demonstration section, not in the record as a whole: `What "done" looks like` (line 56) does require "The name is generated in ONE place and used by both `run()` and the three fixtures". The correction is to carry that into the acceptance bar, and it is cheap and proportional (Q-66 does not demand a test where a command settles it): require the implementer to show a `grep` proving exactly one construction site, or promote the "optional higher-fidelity extra" at line 80, which does force generation through `run()` and so does prove the linkage.

### TI-4 (`low`): the stated grep result does not reproduce for `docs/`

Sidecar line 89: "A grep for `agent-scaffold-checks-run` finds no hit in `README.md`, `CHANGELOG.md`, `docs/`, or `pack/`."

```
$ git -C <worktree> grep -n "agent-scaffold-checks-run" 9f0966c -- README.md CHANGELOG.md docs/ pack/
9f0966c:docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md:198: ... agent-scaffold-checks-run-416707-1785235883764925866 ...
```

The hit exists at the PARENT commit, in the very file the sidecar quotes two paragraphs earlier, and at `5344095` there are three hits under `docs/` (the sidecar, the rendered plan, and the triage file). `README.md`, `CHANGELOG.md` and `pack/` are clean as claimed, and the substantive conclusion (nothing outside the plan documents the name format, so no doc outside `src/` goes stale) holds. Correction is to scope the claim to `README.md`, `CHANGELOG.md`, `pack/`, and non-plan `docs/`.

## Notes (not findings)

- Sidecar line 42 quotes "379 passed" as the figure a green suite reports. `cargo test` on this tree at `5344095` totals 378 passed across the six binaries. The phrase reads as an illustration of what a green run asserts rather than a citation, and the step-92 worktree it was written beside carries an extra test, so I am not raising it; if the sentence is edited for any other reason, dropping the specific number would remove the trap.
- The `owning_pid` constraint (pid first, new components appended) is stated once as a general done-condition at line 57 and restated for candidate (a). It is not repeated for (b), (c), (d), but it does not need to be: it is written as a constraint on any fix, and none of (b), (c), (d) as described would violate it. Not a finding.
- The demonstration's determinism in the GREEN direction depends on the chosen fix being unique by construction. Under candidate (b) (a random suffix) the N*M-distinct assertion is probabilistic in both directions. The sidecar already argues (b) down on other grounds, so this is not load-bearing.
- The race probe also found 15/200 iterations where BOTH concurrent adds failed. In `run()` that path surfaces as `RunError::WorktreeSetup` rather than cross-repo corruption, a second symptom of the same defect that the mechanism section does not mention. It changes no conclusion, so it is a note rather than a finding.

## Tree state

`git status --porcelain` in this worktree reports only this findings file (untracked). No source file, plan file, or sidecar was edited; nothing was committed; no formatter was run. All probes ran under `/tmp/claude-1000/.../scratchpad/ti-probe/`, outside the repository.
