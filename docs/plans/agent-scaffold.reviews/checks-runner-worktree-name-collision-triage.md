# Triage: `checks-runner-worktree-name-collision` (plan fold, deferred step, order 93)

Artifact: `git diff 9f0966c..5344095` (one `[[step]]` in `docs/plans/agent-scaffold.plan.toml`, the sidecar `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md`, and the regenerated `docs/plans/agent-scaffold.md`).
Reviewer findings adjudicated: `docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-reviewer.md` (TI-1 `medium`, TI-2 / TI-3 / TI-4 `low`).
Triaged in worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-testiso` at `5344095`, independent of both the planner that wrote the step and the orchestrator driving the loop.

The human's 2026-07-28 decision to DEFER rather than fix now is out of scope and is not re-litigated. What is in scope is whether the record is correct, because a deferred step's only value is being right when someone picks it up.

**Standard applied.** This is a durable RECORD and a future BRIEF, not transient prose, so I weight accuracy of the stated facts more heavily than I would in a work review. That changed two rulings, and I say so at each: it kept TI-4 as VALID rather than an accepted residual, and it held TI-1 at `medium` rather than dropping it to `low` after I established that the residual defect it points at is rare in practice.

## Verdict summary

| Finding | Reviewer severity | Verdict | My severity | Evidence reproduced |
| --- | --- | --- | --- | --- |
| TI-1: "two of them with the live pid" is false and inverts the risk | `medium` | VALID | `medium` (confirmed) | Yes |
| TI-2: prescribed demonstration is RED against a correct candidate (d) | `low` | VALID | `low` (confirmed) | Yes |
| TI-3: demonstration pins the generator, not the call sites | `low` | VALID | `low` (confirmed) | Yes |
| TI-4: the stated grep result does not reproduce for `docs/` | `low` | VALID | `low` (confirmed) | Yes |
| TR-1 (triager-raised): the both-fail symptom is missing from the mechanism | n/a | VALID | `low` | Yes |

Valid findings to fix: 4 reviewer findings plus 1 triager-raised, all in one edit pass on the sidecar (and its regenerated view). Dismissed: none. Accepted residuals: none. Out of scope: none.

**Backstop status: not triggered.** Nothing was dismissed at any severity, so there is no dismissal at or above the `high` backstop threshold (`AGENTS.md:51`, `:59`) for a second triager to re-check. No `high` or `critical` finding was raised or created by this triage.

## TI-1 (`medium`, VALID): "two of them with the live pid" is false, and the error hides a cross-process collision channel

**Evidence: REPRODUCED.** I opened all four cited locations myself rather than taking the reviewer's table:

- `src/checks.rs:1438-1442`: `fn dead_pid() -> u32 { let pid = u32::MAX; assert!(!pid_is_alive(pid), ...); pid }`.
- `src/checks.rs:1462`: `format!("{RUNNER_PREFIX}{}-{}", dead_pid(), nanos())`.
- `src/checks.rs:1491`: `format!("{RUNNER_PREFIX}{}-{}", std::process::id(), nanos())`.
- `src/checks.rs:1492`: `format!("{RUNNER_PREFIX}{}-{}", dead_pid(), nanos())`.

So sidecar line 49 ("two of them with the live pid") is false: ONE carries the live pid, TWO carry the compile-time constant `u32::MAX`. The same sentence's tail ("They can collide with each other and with a concurrent real run") is true only of `:1491`; `:1462` and `:1492` cannot collide with a real `run()` in the same process at all, because their pid segment is a constant that no live process can hold.

**Ruling on the consequence.** I do not accept the reviewer's framing whole, but I accept its substance. Set out the collision channels explicitly:

- A. Within-process, live-pid namespace (`run()` against `run()`, `run()` against `:1491`). High rate. My own probe (below) puts two barrier-released threads at 12-14% duplicate clock reads; the recorded observation is one suite failure in six runs. Candidate (a) closes it.
- B. Within-process, constant-pid namespace (`:1462` against `:1492`, in two tests that run as concurrent threads). Same rate as A, same mechanism. Candidate (a) closes it.
- C. Cross-process, live-pid namespace. Not a risk while both processes are alive (distinct pids); the only edge is the pid-reuse case the prune already treats as benign (`src/checks.rs:425-428`).
- D. Cross-process, constant-pid namespace (`:1462`/`:1492` in one `cargo test` process against `:1462`/`:1492` in a concurrent one, which is this project's normal state with several worktrees in play). Rate far lower than A or B, because two independently started processes are not aligned the way barrier-released threads are. Candidate (a) does NOT close it, by the sidecar's own words: an `AtomicU64` "guarantees nothing across processes", and both processes start their counter at the same value.

Channel D is the one the record denies exists. The denial is not confined to line 49: the severity section's load-bearing sentence is "Production is NOT affected. Each `agent-scaffold checks` invocation is its own process, so two concurrent runs have different pids", and candidate (a)'s trade-off bullet dismisses its cross-process gap on the grounds that it "leans on the pid exactly as today for that half". Both of those are sound where the pid is live and unsound at `:1462` and `:1492`, where the pid discriminates nothing.

**Is it a one-line correction, or do the done-conditions have to change?** Neither extreme. It is more than one line, and it does not require the done-conditions to be rewritten. The gap the corrected fact opens is that an implementer can pick candidate (a) alone, satisfy every one of the five stated done-conditions (unique by construction, one generator used by `run()` and the three fixtures, pid stays first for `owning_pid`, the three doc comments corrected, a test that fails without the fix), pass the prescribed demonstration, and close the step with channel D untouched.

There is also a structural constraint the record does not state anywhere, and it is the reason this is not just a typo: `:1462` and `:1492` MUST carry a dead pid. `dead_pid()` asserts `!pid_is_alive(pid)` because those tests exist to plant an orphan owned by a dead owner so the prune reclaims it. Those two sites can therefore never acquire a live-pid discriminator in the first segment, and `owning_pid` (`src/checks.rs:400-405`, verified: `strip_prefix(RUNNER_PREFIX)?.split('-').next()?.parse()`) requires the pid to stay first. So any discriminator that closes channel D there has to be appended after the constant pid: a `create_dir` reservation (candidate (d)), or entropy, or the live pid as a LATER component, for instance `{RUNNER_PREFIX}{dead_pid}-{live_pid}-{seq}`, which still parses as a dead owner. I name that only to show the constraint is cheap to satisfy; the fix stays not pre-decided, as the section intends.

**What the fix must achieve** (wording is the planner's):

1. Line 49 states the true fixture facts: one live-pid site (`:1491`), two constant-pid sites (`:1462`, `:1492`, `u32::MAX` via `dead_pid()`), and that the constant-pid pair therefore carries no cross-process discriminator.
2. The record says why those two are constant by design (they need a dead owner), so a future implementer does not "fix" it by making them live.
3. Candidate (a)'s trade-off bullet stops resting on "leans on the pid exactly as today", or is qualified to say that argument does not hold at the two constant-pid sites, so the choice between (a) alone and the (a)+(d) composition is made on true premises.

Point 3 is the load-bearing one. Correcting the fact without correcting the trade-off leaves the trap intact.

**Severity: `medium`, confirming the reviewer.** I considered `low` and rejected it, and I considered `high` and rejected it.

Against `high`: nothing is broken today, the step is deferred, and channel D is rare. The reviewer's own disclosed counter-evidence (0 failures in 40 paired cross-process runs) is the honest report of that rarity.

Against `low`: the false clause is not decorative, it feeds the candidate trade-off that decides the fix, and the failure mode if it survives is that the step gets closed as done while the symptom it was opened for can still occur. Because this is a durable record whose whole value is being correct months from now, I do not discount an inaccuracy on the grounds that it is only read later; that is exactly when it will be read, by someone with no access to this round's reasoning. On transient prose I would have called this `low`.

**On the reviewer's counter-evidence (0/40 cross-process reproductions).** Its reasoning is sound and I accept it, with one correction to how much it should move the reader. A collision in channel D needs two independently launched processes to execute the same `format!` within roughly 25 ns of each other; with startup jitter of even tens of milliseconds, the per-pair probability is on the order of 1e-6, so the expected number of hits in 40 pairs is around 1e-4. A 0/40 result is what you get whether or not the mechanism exists, so it is not counter-evidence. It does, however, cut against the reviewer's rhetoric: "INVERTS the risk" and "leaves the defect half-present" overstate it, because candidate (a) alone would still close channels A and B, which are the high-rate ones and the ones that produced the observed failure. What (a) alone leaves is a rare channel plus a record that says that channel is impossible. That is the real cost, and it is why the finding is valid at `medium` rather than higher: the harm is mostly to the next person diagnosing a failure the record told them could not happen.

## TI-2 (`low`, VALID): the prescribed demonstration is RED against a correct implementation of candidate (d) alone

**Evidence: REPRODUCED.** I built my own probe outside the repo (`rustc -O`, the same expression as `nanos()`), at the section's own parameters:

```
consecutive: n=100000 zero_deltas=0 min=20 median=30 max=16841
threads=2  per=50000 total=100000 distinct=87841 excess=12159
threads=2  per=50000 total=100000 distinct=85675 excess=14325
threads=16 per=50000 total=800000 distinct=239917 excess=560083
threads=8  per=1000  total=8000   distinct=5207 excess=2793
threads=8  per=1000  total=8000   distinct=4646 excess=3354
threads=8  per=1000  total=8000   distinct=4699 excess=3301
```

At N = 8, M = 1000 a raw `{pid}-{nanos}` generator yields 2793-3354 duplicates out of 8000 in every run, so the prescribed "assert the collected set holds N * M distinct entries" fails against it deterministically in practice. Candidate (d) (sidecar line 66) keeps a clock-derived name and takes uniqueness from `std::fs::create_dir` plus retry, so under (d) alone the raw name generator still duplicates at exactly that rate and the prescribed assertion is RED against correct code. The finding holds.

**Ruling.** A real defect in the brief, not acceptable imprecision, but a small one. The section explicitly promises to be fix-independent ("Not pre-decided here; the implementer picks one") and then writes an acceptance test that only works for the name-entropy candidates (a) and (c). The cost if unfixed is bounded: an implementer who picks (d) sees a red test against correct code and has to work out for themselves that the unit under test is the reserving call, not the name string, or is nudged into adding entropy they do not need in order to satisfy a test the brief prescribed. The record's preferred composition (a)+(d) does pass the test as written, which is why this is `low` and not higher.

**What the fix must achieve.** The demonstration defines its unit under test by the PROPERTY (the call that yields the final worktree path returns a distinct path every time, under N threads released together) rather than by the string generator, so it is green against any of (a), (b), (c), (d) implemented correctly and red against today's code. The mutation step ("mutate the disambiguator to a constant") then remains meaningful under each.

## TI-3 (`low`, VALID): the demonstration pins the generator, not the call sites, yet is described as settling the claim

**Evidence: REPRODUCED (citation).** `src/checks.rs:791-792` is verbatim as quoted, and nothing in the prescribed test observes it. Sidecar line 80 does say "a claim the unit test already settles". Sidecar line 56 does independently require "The name is generated in ONE place and used by both `run()` and the three fixtures". `AGENTS.md:120` principle 11 is "Tests must actually exercise the code they claim to - A test must run the code path it claims to cover", so the reviewer's citation is accurate.

**Ruling: valid, and it is a separate issue from TI-2 with a shared root cause.** The orchestrator asked whether these are one issue or two. They are two symptoms of one root cause and take one correction. The root cause is that the demonstration section names its unit under test syntactically ("name generation") instead of by the property that matters (the path `run()` and the fixtures actually use is distinct per call). TI-2 is the false-negative symptom (red against correct code under (d)); TI-3 is the false-positive symptom (green while the defect is fully present, if the inline `format!` at `:791-792` survives alongside a new generator). I record them separately because they fail in opposite directions and a fix that addresses only one is possible, but the correction in TI-2 above, stated as a property over the call that yields the path used by `run()` and the fixtures, closes both. Count them as two findings and one edit.

Of the two, TI-3 is the more consequential: a demonstration that can be green with the defect intact is exactly the shape principle 11 names, and it is the state the tree is in TODAY for the three fixtures, which is not hypothetical. TI-2 only costs the implementer time.

I hold it at `low` rather than raising it, because the record as a whole does carry the linkage requirement at line 56, so an implementer who reads the whole step is not misled; only one who treats the demonstration section as the acceptance bar in isolation is. That is a real risk but a mitigated one.

**What the fix must achieve.** The demonstration section's acceptance bar states the linkage, not just the property: the generator is the only construction site, shown by a command (a `grep` proving exactly one `format!` building a `RUNNER_PREFIX` name), or by promoting the "optional higher-fidelity extra" at line 80, which forces generation through `run()`. A command settles it, so no extra test is owed (Q-66's proportionality).

## TI-4 (`low`, VALID): the stated grep result does not reproduce for `docs/`

**Evidence: REPRODUCED.** I re-ran it myself:

```
$ git grep -n "agent-scaffold-checks-run" 9f0966c -- README.md CHANGELOG.md docs/ pack/
9f0966c:docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md:198: ... agent-scaffold-checks-run-416707-1785235883764925866 ...
$ git grep -c "agent-scaffold-checks-run" 5344095 -- docs/
docs/plans/agent-scaffold.md:3
docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md:1
docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md:3
```

So sidecar line 89 ("A grep for `agent-scaffold-checks-run` finds no hit in `README.md`, `CHANGELOG.md`, `docs/`, or `pack/`") is false for `docs/` at the parent commit and at the step's own commit. `README.md`, `CHANGELOG.md` and `pack/` are clean as claimed.

**Ruling: VALID, not a transient-file artifact.** The orchestrator's framing was that the hit lives in a transient file. That is true of one of the three hits. The other two are permanent: the sidecar itself and the rendered `docs/plans/agent-scaffold.md` both contain the string and will keep containing it as long as the step exists. So the stated grep will never reproduce for anyone who runs it, including the implementer who picks the step up. This is precisely where I weight record accuracy above transient prose: the claim is written as a verification result (a command that was run and returned nothing), it is offered as the basis for "no documentation step is owed", and it fails to reproduce on the first attempt. A reader who checks the record's cheapest checkable claim and finds it wrong has reason to distrust the expensive ones, which on this sidecar are the measured numbers I have just confirmed are right. That is the cost, and it is worth one clause to avoid.

The substantive conclusion is unaffected and I confirm it: outside the plan's own documents, the name format is written down only in `src/`, so no documentation step is owed.

**What the fix must achieve.** The claim is scoped so that it is true and re-checkable, for instance stating that `README.md`, `CHANGELOG.md` and `pack/` are clean and that the only `docs/` hits are the plan's own record of this defect, so nothing outside `src/` goes stale when the name format changes.

## TR-1 (`low`, VALID, triager-raised): the mechanism section omits the both-fail symptom

The orchestrator asked me to rule on this, which the reviewer flagged as a note and did not raise.

**Evidence: REPRODUCED at second hand and consistent with the code.** The reviewer's 200 paired races reported `both=25 exactly_one=160 neither=15`: 15 of 200 collisions had BOTH `git worktree add` calls fail. I did not re-run the race (the reviewer's `both=25` result is the one the sidecar's stated mechanism depends on and it is not disputed), but the code path is plain: `src/checks.rs:795-800` turns a failed add into `RunError::WorktreeSetup`, and at the fixtures a failed add is a `git_ok` assertion failure, so a collision there is a hard, loud test failure rather than the silent cross-linking the record describes.

**Ruling: VALID at `low`, fix in the same pass.** The mechanism section (sidecar line 26) describes exactly one symptom, cross-repository `.git` overwriting, and the evidence section records exactly one sighting shaped like that. A future failure that presents as a bare `git worktree add` failure or a `git_ok` assertion, with no cross-linking, would not match the record, and the person diagnosing it would not connect it to this step. The record's stated purpose is to carry a mechanism forward, so a second symptom of the same mechanism belongs in it. Cost is one clause; I would not open an edit pass for it alone, and TI-1 and TI-4 already open one.

## Rulings on the reviewer's other notes

- **"379 passed" (sidecar line 42) is 378 on this tree.** I measured it independently: `cargo test` on `5344095` summed across the six binaries gives `passed=378 failed=0`. NOT a separate finding, and I agree with the reviewer's decision not to raise it: the number is illustrative inside a rhetorical sentence, not a citation, and any specific number there is wrong the moment a test is added. Fold it in opportunistically since the sidecar is being edited anyway: drop the numeral rather than correct it, so the sentence cannot go stale again.
- **The `owning_pid` constraint not being restated for (b), (c), (d).** Agreed, not a finding. It is written as a constraint on any fix.
- **The green direction being probabilistic under candidate (b).** Agreed, not a finding, and it is subsumed by the TI-2 correction: a property-level unit under test makes the assertion deterministic for (a), (c) and (d), and (b) is already argued down.

## Things neither the reviewer nor the step raised

- **`src/checks.rs:789-790`.** The comment immediately above the defect reads "A unique temp path OUTSIDE the repository ... The `RUNNER_PREFIX` (with the embedded pid) is what the startup prune recognises." It asserts the uniqueness that is currently false, and the sidecar's documentation-impact list (line 58) names three doc comments (`:72-77`, `:400-402`, `:845-847`) and presents that list as complete. Not raised as a finding: done-condition line 55 already requires the uniqueness argument to be written into the code comment, and that lands at `:789-790` because it is the comment on the naming site itself, which the implementer edits regardless. Worth adding to the list if the sidecar is being edited; not worth an edit on its own.
- **Plan mechanics re-checked independently** and clean, so nothing here is contested: `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` reports `216 records, valid`, `92 steps, 69 questions, valid`, `workflow invariants hold`; `cargo run -- render ... --check --strict` reports `up to date`; `grep -cE "\brun\(" src/checks.rs` returns 23, that is the definition at `:734` plus the 22 call sites the sidecar claims; the plan's own `[[principle]]` 1, 5 and 6 are the ones the sidecar cites, under the plan's numbering, which is the correct one; and the probability arithmetic checks out ((5/6)^6 = 0.3349, (5/6)^16 = 0.0541, (5/6)^17 = 0.0451, so 17 is the first n at which it drops to 5%).
- **No finding about the fix being unimplemented**, and none about the deferral. Both are out of scope by the human's decision.

## Tree state

`git status --porcelain` in this worktree reports two untracked files: the reviewer's findings file (copied in as input) and this triage file. No plan file, sidecar, or source file was edited; nothing was committed; no formatter was run. The probe was built and run under the session scratchpad at `.../scratchpad/tri-probe/`, outside the repository.
