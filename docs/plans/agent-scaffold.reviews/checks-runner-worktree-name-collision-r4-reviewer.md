# Plan review round 4, reviewer: `checks-runner-worktree-name-collision` (deferred step, order 93)

Reviewed in worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev4-testiso`, detached at `3a7b944`, independent of the planner that wrote the fix and of the orchestrator driving the loop. Artifacts: `git diff HEAD~1..HEAD` (the round-3 fix commit `3a7b944`) and the whole fold `git diff 86b0e5c..3a7b944`.

I did not see rounds 1 to 3 as they ran; I read the three triage files as the authoritative record of what was settled and judged the artifact myself. I did not re-open the (b)/(c) "implemented correctly" wording, the channel-D question, or the step title. The deferral is not re-litigated and no verdict below rests on the fix not being implemented.

## Findings

**ZERO findings, at every severity.** Stated explicitly rather than by omission: no `critical`, no `high`, no `medium`, no `low`.

Four items I checked, considered, and deliberately did NOT raise are recorded in the last section, so a later round does not re-find them and treat them as new.

## 1. Round-3 repairs: both CLOSED

`git diff HEAD~1..HEAD` is exactly two files (the sidecar and the regenerated `docs/plans/agent-scaffold.md`), 2 insertions and 2 deletions, two hunks per file, each a pure within-line deletion. That is the shape the round-3 triage prescribed, and nothing else changed.

**`T3-1`: CLOSED.** `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md:92` now reads `DOCUMENTATION IMPACT: in-code only.` The false, circular clause is gone, and the repair took the triage's recommended form (delete the pointer, not re-aim it), so there is nothing left at that line for a later edit to falsify.

**`T3-2`: CLOSED.** `:62` now reads "Every comment the documentation-impact section below names is corrected in the same change." The noun now matches the membership of the section it delegates to (`:94`, "The change must correct four comments in `src/checks.rs`"). I re-read all four cited ranges in the source rather than accepting the round-3 finding:

- `src/checks.rs:72` `/// The file-name prefix of a runner's temporary worktree directory under the` -> `///`.
- `src/checks.rs:400` `/// Parse the owning pid out of a runner worktree directory name of the form` -> `///`.
- `src/checks.rs:845` `/// Nanoseconds since the epoch, for a unique temp path. Falls back to a fixed` -> `///`.
- `src/checks.rs:789` `// A unique temp path OUTSIDE the repository; git worktree add creates it. The` -> `//`, an ordinary line comment in the body of `run()`.

So three of the four are doc comments and one is not, which is exactly why the restrictive noun had to go.

**Both repairs landed in the rendered view too.** `diff` of the sidecar against `docs/plans/agent-scaffold.md:1280-1374` is empty apart from the section-separating blank line the renderer adds, and `render docs/plans/agent-scaffold.plan.toml --check --strict` exits 0 with "up to date". No fix landed in only one of the two.

**`plan.toml` untouched.** `git diff --name-status HEAD~1..HEAD` lists only `docs/plans/agent-scaffold.md` and the sidecar.

## 2. The round-3 prediction, tested directly

The round-3 triage predicted that after two deletions the count of fragile internal cross-references is zero. I enumerated the population myself rather than re-checking its table.

**My census is larger than the triage's nine. I find twelve.** Three references the triage's table did not list are below, marked NEW; all three are sound.

| Line | Reference | Status |
| --- | --- | --- |
| `:26` | "The recorded failure text below (both tests citing one path, each error naming the OTHER test's fixture repository)" | NEW. Sound. Points down to `:34`, whose quote reads "Both failures cited the SAME runner worktree path ... and each error named the OTHER test's fixture repo". Direction and content both correct. |
| `:28` | "the cross-registered take-over above" | NEW. Sound. The take-over is described at `:26`. |
| `:42` | "(see the scope section)" | Sound. Direction-free, no membership claim; the scope section at `:46-55` does cover the constant-pid fixtures. |
| `:44` | "the only discriminator is the ~25 ns clock" | Sound. The 25 ns figure is established at `:24` and reproduces (section 4). |
| `:62` | "the documentation-impact section below names" | Sound after the `T3-2` repair. Section at `:92-94` names four comments; the bullet's noun is now "comment". |
| `:63` | "(see the demonstration section, which is the load-bearing part of this step)" | Sound. Direction-free, no membership claim. |
| `:67` | "the (a) + (d) composition below" | Sound. The composition is at `:72`. |
| `:76` | "for the reason below" | Sound. The arithmetic is at `:78`. |
| `:83` | "the sites the scope section enumerates" | Sound. An instruction to reconcile, not a membership claim. |
| `:85` | "the linkage command above" / "the requirements above" | Sound. `:83`, and `:82-84`. |
| `:85` | "(or the `WorktreeSetup` failure, which is the more common shape)" | NEW. Sound. `:28` establishes the loud failure as the common shape, and my own 200-trial race reproduces that direction (section 4). |
| `:90` | "the red/green uniqueness demonstration above" | Sound. `:84`. |
| `:92` | (pointer deleted by `T3-1`) | No claim remains. |

**The class is exhausted.** Twelve internal cross-references, twelve sound, zero defective. The triage's census was under-inclusive by three, but every reference it missed is one of the correct ones, so its conclusion holds under the wider population as well as its own. I found no cross-reference the triage missed that is defective, which was the one outcome that would have been an important finding.

## 3. Fix-induced residue sweep

Both deletions REMOVE a claim rather than move one, so there is no new synchronisation point. I checked what each could have made stale and found nothing.

- Deleting "and it is named above" at `:92`: nothing else in the document asserts that the documentation impact appears above that line, and `:62` points DOWN to the section, which is the surviving and correct direction. The header still stands as a true claim on its own (see note 3 below on the CHANGELOG sentence).
- Deleting "doc " at `:62`: the bullet's set is now identical to `:94`'s set, so it is satisfied by whatever membership that section ever carries. It also removed the last conflict with `:22`, which correctly calls `src/checks.rs:845-847` a doc comment; `:22` is unaffected because `:62` no longer asserts a category at all.

Cross-section agreement, re-checked pairwise:

- Severity (`:42-44`) against mechanism (`:5-28`) and scope (`:46-55`): `:42`'s "Read that as a statement about the SHIPPED path only ... it separates nothing at the two constant-pid test fixtures" agrees with `:51` and `:53`, and with candidate (a)'s qualified trade-off at `:67`. No residue from round 1's channel-D correction.
- Rate figures: `:32` "once in six consecutive runs", `:44` "about one run in six", `:78` "one failure in six runs". Consistent throughout.
- Fixture counts: `:51` names three fixtures (`:1462`, `:1491`, `:1492`), `:60` requires one generator "used by both `run()` and the three fixtures", `:67` names the two constant-pid ones, `:83` says the three fixtures are the ones in the un-linked state today. All agree at three, one live-pid and two constant-pid.
- Done conditions (`:59-63`) against demonstration (`:74-86`): `:63`'s "a test that FAILS without the fix" is `:84`'s red-before-green; `:60`'s single-generator requirement is `:83`'s linkage command; `:85`'s "the requirements above are the proportional minimum" reaches all three of `:82`, `:83`, `:84`. No count drifts against any other count. I ran the round-2 triage's own suggested consistency pass (grep the sidecar for "three" and "four" and for "proportional minimum") and every numeral matches the list it counts.
- Documentation impact (`:92-94`) against done conditions (`:62`): one enumeration, one delegation, no second count.

## 4. Substance spot-check (the expensive claims a future implementer will not re-derive)

Probes built and run under the session scratchpad OUTSIDE the repository (`.../scratchpad/r4probe/`), `rustc -O`, on this 16-core machine.

**Clock resolution and cross-thread duplicates. Reproduced.** Same expression as `nanos()`:

```
consecutive: n=100000 zero_deltas=0 min=20 median=30 max=9257
threads=2  per=50000 total=100000 distinct=85961 excess=14039
threads=2  per=50000 total=100000 distinct=87104 excess=12896
threads=2  per=50000 total=100000 distinct=84107 excess=15893
threads=16 per=50000 total=800000 distinct=223759 excess=576241
threads=8  per=1000  total=8000   distinct=5189  excess=2811
threads=8  per=1000  total=8000   distinct=4831  excess=3169
threads=8  per=1000  total=8000   distinct=4895  excess=3105
```

- `:24` "consecutive reads on one thread never repeat but advance by a median of 30 ns (minimum 20 ns) over 100000 samples": exact, on all three figures.
- `:24` "8679 of 100000 (about 8.7%)" for two threads, corrected in the same sentence to "read 8.7% as a floor": my 12896-15893 is above 8679 and inside the 10933-16386 range the sentence quotes. The floor framing is right and is the conservative direction.
- `:24` "at 16 threads it is 568127 of 800000": mine is 576241 of 800000, ordinary run-to-run noise at the same rate.
- `:82` "measured at 2793 to 3354 duplicates in 8000 at N = 8, M = 1000": mine is 2811 to 3105, inside the stated range. The claim that a string-level assertion is RED against a correct candidate (d) holds.

**Confidence arithmetic. Reproduced exactly.** `(5/6)^6 = 0.3349` (sidecar `:78` says 0.33), `(5/6)^16 = 0.0541`, `(5/6)^17 = 0.0451`, and the first n with `(5/6)^n <= 0.05` is n = 17, which is `:78`'s "17 consecutive clean runs".

**Git's directory handling. Reproduced, on a NEWER git than the one the record names.** The tree's toolchain now provides git 2.54.0; the record says "verified against git 2.51.2". Both halves still hold:

```
$ mkdir emptydir && git -C repo-a worktree add --detach .../emptydir HEAD
Preparing worktree (detached HEAD 6dcc7ba) ... EMPTY_EXIT=0
$ mkdir nonempty && touch nonempty/x && git -C repo-a worktree add --detach .../nonempty HEAD
fatal: '.../nonempty' already exists          NONEMPTY_EXIT=128
```

So `:26`'s parenthetical and candidate (d)'s "Verified compatible with git" at `:70` both still stand.

**The 200-race split and the cross-registration claim. Mechanism reproduced; proportions differ, as a race's proportions must.** I ran my own paired race: two independent repos, two barrier-released threads, one shared path, 200 trials, on git 2.54.0.

```
trials=200 both=68 exactly_one=115 neither=17 both_registered=68
```

- `:28` "with the path registered in both repositories in every one of the 25": reproduced at 68 of 68. This is the load-bearing half, and it is the claim the whole take-over mechanism rests on.
- `:28` "So the loud failure is the common shape and the silent cross-linking is the minority": reproduced in direction (132 of 200 loud, 68 silent) though not in proportion (the record's own run was 175 loud, 25 silent).
- The stated split itself (25 / 160 / 15) is presented as one measurement from one probe, not as an invariant, so a different split on a different git and machine does not falsify it.

**The `owning_pid` pid-first constraint. Verified.** `src/checks.rs:403-404`: `dir_name.strip_prefix(RUNNER_PREFIX)?.split('-').next()?.parse().ok()`. It reads the first `-`-separated segment and nothing else, so `:61` and `:53` are exactly right that any new component must be appended AFTER the pid.

**The dead-pid design constraint. Verified.** `src/checks.rs:1438-1442` is `fn dead_pid() -> u32 { let pid = u32::MAX; assert!(!pid_is_alive(pid), ...); pid }`, so `:53`'s "Those two sites MUST keep a dead pid; it is a design constraint" is accurate.

**The four `RUNNER_PREFIX` construction sites and their live/dead split. Verified.**

```
$ grep -n 'format!("{RUNNER_PREFIX}' src/checks.rs
792:  ... std::process::id(), nanos()   (production, run())
1462: ... dead_pid(), nanos()
1491: ... std::process::id(), nanos()
1492: ... dead_pid(), nanos()
```

Exactly four sites, exactly one fixture on the live pid (`:1491`), exactly two on the compile-time constant. `:51` is right, including the consequence that the constant-pid pair carries no cross-process discriminator.

**Other code claims I checked rather than assumed:**

- `:50` "22 call sites in that module": `grep -cE "\brun\(" src/checks.rs` is 23; the definition is at `:734` and the other 22 lines are all between `:974` and `:1468`, inside `mod tests` which opens at `:855`. Accurate, including "in that module".
- `:55`'s not-affected list: every one of the seven resolves to a helper keyed on a per-test literal `name` (`src/checks.rs:862-871`, `src/main.rs:1726-1731`, `src/manifest.rs:552-558`, `src/plan/render.rs:638`, `tests/audit_command.rs:20`, `tests/scaffold_precommit_hook.rs:14`, `tests/checks_staged_hook_env.rs:50`).
- `:55`'s three latent `{pid}-{nanos}` integration sites resolve and do carry that shape with distinct literal prefixes: `agent-scaffold-validate-toml-only-` (`tests/validate_workflow_toml_source_needs_no_plan.rs:58`), `agent-scaffold-validate-workflow-no-source-` (`:90`), `agent-scaffold-validate-projection-` (`tests/validate_toml_primary_skips_markdown_plan.rs:74`), each with `SystemTime::now()...as_nanos()` as the second component. "A latent copy-paste hazard to note, not to fix here" is the correct call.
- `:68`'s dependency-discipline citation: `src/checks.rs:390` is "no libc crate is pulled in just for a `kill(pid, 0)`", inside the cited `:388-392`.
- `:42`'s prune citations: `prune_orphan_worktrees` spans `:407-461` (doc from `:407`, closing brace at `:461`) and the benign pid-reuse passage is at `:425-428`. Both exact.
- `:26`'s `WorktreeGuard` citation: the `Drop` impl is `:329-342` and it does `worktree remove --force` then `remove_dir_all` then `worktree prune`. Exact.
- `:14`'s two code quotations are verbatim against `src/checks.rs:791-792` and `:849-851`, and `RUNNER_PREFIX` is the const at `:78`.
- `:90`'s characterisation of step 85 checks out: `docs/plans/agent-scaffold.steps/drift-guard-test-hook-hygiene.md:3` describes the `precondition_rejects` panic-hook swap in `src/agents_md_drift.rs`.

**Both external quotations are verbatim.** `git show 9f0966c:docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md` line 198 carries `:34`'s quote word for word, including the path `agent-scaffold-checks-run-416707-1785235883764925866`. `docs/plans/agent-scaffold.ledger.md:367` carries `:38`'s quote word for word ("A rare pre-existing MAIN-binary test-parallelism flake was seen once in inc1 review (candidate backlog item on test isolation).").

## 5. Mechanical

Citations: every `file:line` in the sidecar resolves, and I re-read each range rather than trusting a prior round's table. This includes the plan-internal ones, which is where this project produced misnumbered citations this week:

- `:76` "The reproducible-evidence rule (`Q-66`, step 88)": `docs/plans/agent-scaffold.plan.toml` has slug `reviewer-reproducible-evidence` at `order = 88`, titled for `Q-66`. Correct on both the number and the id.
- `:88` "RELATION TO STEP 85 (`drift-guard-test-hook-hygiene`)": slug `drift-guard-test-hook-hygiene` is at `order = 85`. Correct.
- `:60` "plan Principle 1, prefer the cleaner long-term architecture over the smallest diff", `:72` "plan Principle 5, make illegal states unrepresentable", `:76` "Plan Principle 6 (ground decisions in evidence)": the plan's `[[principle]]` 1, 5 and 6 are "Prefer the cleaner long-term architecture over the smallest diff", "Make illegal states unrepresentable" and "Ground decisions in evidence". Correct, and under the plan's numbering, which is the right one.
- `:83` quotes the AGENTS.md principle by TEXT, not by number: "Tests must actually exercise the code they claim to" is `AGENTS.md:124` verbatim. Quoting by text is what keeps this one from rotting.

No `[[question]]`, no decision receipt, as `:3` claims: `grep -c 'folded_into = "checks-runner-worktree-name-collision"'` on `plan.toml` returns 0, and the three `workflow.jsonl` lines mentioning the slug are all `"type":"round"` (records 217, 219, 220 for the three review rounds). No `type:"decision"` record exists for it, and none is owed.

Validators, both green, exit 0:

```
$ cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 220 records, valid
docs/plans/agent-scaffold.plan.toml: 92 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
$ cargo run -- render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
```

92 steps and 69 questions as expected. 220 records is at or above the expected floor, and another loop is appending concurrently, so it is not an error.

## 6. Checked, considered, and deliberately NOT raised

Recorded so a later round does not re-find these and treat them as new.

1. **`:28`'s "because `git worktree add` refuses a non-empty directory" is one of several loud-failure texts on git 2.54.0.** Over the 149 failed adds in my 200 paired races, 73 carried `fatal: '<path>' already exists`, 59 carried `fatal: Unable to create '<repo>/.git/worktrees/<name>/index.lock': File exists`, and 17 carried something else. Not raised: everything the sentence asserts as CONSEQUENCE is correct and reproduces (the add fails loudly; in `run()` that is `RunError::WorktreeSetup` at `src/checks.rs:795-800`; at a fixture it is a `git_ok` assertion failure), the record names the git version it verified against, and the lock-file variant is the same collision caught by a lock rather than a different mechanism, so it is further support for the record rather than a correction to it. A diagnoser matching a future sighting is looking for a loud `WorktreeSetup` failure, which is what the record tells them to expect.
2. **The race split's proportions differ on this machine and git** (68 / 115 / 17 against the recorded 25 / 160 / 15). Not raised: the sidecar states it as one measurement of 200 collisions, not as an invariant, and both conclusions it draws from it reproduce.
3. **`:92` "DOCUMENTATION IMPACT: in-code only" alongside `:94` "A CHANGELOG `Fixed` entry is likely right".** Considered as a possible internal tension and rejected. "Documentation impact" is this project's term of art for what a change makes STALE (`AGENTS.md:30`: "it identifies which docs and prompts the change will make stale"), and adding a CHANGELOG entry is a new addition under a release-notes convention, not a staleness repair. It is also pre-existing: both sentences have been in the file since the step's first commit `bcf3dbb`, so the round-3 deletion neither created nor worsened it.
4. **The two items the planner reported rather than fixed.** `:62`'s forward pointer resolves and is accurate (checked in section 2), and `:24`'s unpointed "an independent probe during this step's plan review" quotes its numbers inline, all of which I reproduced in section 4 with the stated 8.7% holding as a floor. Neither is factually wrong, so neither is raised.

## 7. Convergence read (advisory, the orchestrator owns the decision)

The round-3 triage made a checkable prediction and it survived a test on a wider population than it was made on: twelve internal cross-references, all sound. The class the last two rounds lived in is closed, and this round's own lens (fix-induced residue) found nothing, which is the first time that lens has come back empty. The expensive claims, the ones a future implementer will rely on and never re-derive, all reproduce: the clock measurements to the digit on three of four figures and conservatively on the fourth, the confidence arithmetic exactly, the cross-registration in every both-succeed trial, and all four construction sites with their live/dead pid split.

The record is correct as it stands, and it is correct in the parts that matter most for a reader months out with none of this context.

## Tree state

`git status --porcelain` in this worktree reports only this findings file, untracked. No plan file, no sidecar, and no source file was edited; nothing was committed; no formatter was run. The two probes (`probe.rs`, `race.rs`) and their scratch repositories were built and run under the session scratchpad at `.../scratchpad/r4probe/`, outside the repository; nothing was created inside the repo tree beyond cargo's own `target/`.
