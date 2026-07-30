# Triage: `checks-runner-worktree-name-collision`, round 4, commit `3f49012`

Triaged in an isolated worktree at `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage93-r4`, branch `triage/checks-collision-r4`, HEAD `3f490128ef34c608dac134a313bdb69972e0daf0`. Every measurement below was taken here, by me, on this tree. Nothing is quoted on a reviewer's word: each mutation, each candidate fix and each probe was applied, built, run, and reverted in this worktree.

Baseline reconfirmed here before anything was touched:

```
test result: ok. 373 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s
test result: ok. 5 passed; 0 failed; ...
test result: ok. 1 passed; 0 failed; ...
test result: ok. 1 passed; 0 failed; ...
test result: ok. 3 passed; 0 failed; ...
test result: ok. 1 passed; 0 failed; ...
test result: ok. 2 passed; 0 failed; ...
```

373 + 5 + 1 + 1 + 3 + 1 + 2 = **386 passed, 0 failed**. `cargo clippy --all-targets`: 0 warnings, 0 errors.

Mutations used throughout, named once here:

| id | Site | Change |
| --- | --- | --- |
| `MUT-A` | `claim_dir`, `src/checks.rs:452` | `Err(error) => Err(error)` -> `Err(_error) => Ok(false)` (a real error folded into the lost-claim verdict) |
| `MUT-B` | `claim_dir`, `src/checks.rs:452` | `Err(error) => Err(error)` -> `Err(_error) => Ok(true)` (an errored claim counted as WON) |
| `MUT-C` | `run()`, `src/checks.rs:961-972` | the `let _guard = WorktreeGuard {...};` block moved BELOW the failed-`git worktree add` early return |

---

# Deduplication

**`RG1` and `MU1` are ONE finding.** Confirmed, not assumed. Both name the same line (`src/checks.rs:452`, `claim_dir`'s third match arm), both reproduce it with the same mutation (`RG1`'s `N1` and `MU1`'s table row 4 are byte-identical changes), both derive the same user-visible consequence from the same fixture shape (a `chmod 555` `TMPDIR` passed to the built binary), and both trace the same cause: round 3's `AD1a` fix pinned the CALLER's `map_err` through the injected `claim` seam, and the seam by construction never calls the real `claim_dir`. Two models on two different lenses reached the same line independently. They are merged below as **`RG1`/`MU1`**, severity `medium`.

I verified the shared coverage claim myself rather than accepting either reviewer's grep:

```
$ grep -rn "claim_dir" --include=*.rs .
src/checks.rs:448:fn claim_dir(path: &Path) -> io::Result<bool> {
src/checks.rs:499:	reserve_runner_worktree_with(pid, claim_dir)
src/checks.rs:1704:		assert!(claim_dir(&path).unwrap(), "the first claim on a fresh path is won");
src/checks.rs:1705:		assert!(!claim_dir(&path).unwrap(), "a second claim on the same path is lost");
(remaining hits are doc/comment prose)
```

Two test-side call sites, both in `a_directory_claim_is_exclusive`, driving `Ok(true)` and `Ok(false)`. Nothing drives `Err`.

`RG2`, `RG3` and `RG4` are each distinct from every other finding this round and from each other.

**Checked against the settled list, and none of these reopens it.** `RG1`/`MU1` is a different call site and a different match arm from `AD1b` (`fs::create_dir_all(&temp)` at `:519-524`), which stays an accepted residual and which I did not re-attack. `RG3` touches the guard ordering in `run()`, not the `X1` sequence residual, not `RUNNER_RESERVE_ATTEMPTS`, not `nanos()`. `RG4` is about the SENTENCE describing a declined alternative; it asks for no behaviour change and takes no position on whether the prune should be widened, so the routed-out symlink/`temp_dir()` behaviour stays routed out. `X5`, `X7`, the (a)+(d) design choice, the SIGKILL-leak trade and the uniqueness property itself are untouched. Both reviewers' non-findings sections re-confirmed `fetch_add -> load` and `16 -> 3` as green; I did not re-run either and neither is raised.

---

# `RG1`/`MU1`: `claim_dir`'s own error arm is executed by nothing

**Verdict: VALID. Severity `medium`. Fix required.**

## Reproduced

Each mutation applied alone to the committed tree, full `cargo test` plus `cargo clippy --all-targets`, then reverted.

| Mutation | Suite | clippy |
| --- | --- | --- |
| `MUT-A` | **GREEN, 386 passed, 0 failed** | 0 warnings, 0 errors |
| `MUT-B` | **GREEN, 386 passed, 0 failed** | 0 warnings, 0 errors |

Two independent mutations of the same line, both survive the entire suite. That is the strongest evidence form available and it is met.

User-visible consequence, measured with three separately built binaries against a scratch repo with one trivial `lint` check and `TMPDIR` naming an existing but `chmod 555` directory. One run each, quoted verbatim (paths shortened to `<ROOT>`):

```
committed 3f49012:
error: could not reserve the runner worktree directory <ROOT>/unwritable/agent-scaffold-checks-run-1893998-1785454367750023411-0: Permission denied (os error 13)
exit=2

MUT-A:
error: could not reserve a unique runner worktree directory after 16 attempts (last tried <ROOT>/unwritable/agent-scaffold-checks-run-1894515-1785454382872086828-15)
exit=2

MUT-B:
error: could not set up the isolation worktree: `git worktree add` failed: Preparing worktree (detached HEAD 9c33d51)
fatal: could not create leading directories of '<ROOT>/unwritable/agent-scaffold-checks-run-1895337-1785454438677121741-0/.git': Permission denied
exit=2
```

`MUT-A` is a permissions fault reported to the user as name-collision exhaustion after 16 pointless `mkdir` syscalls, which is the failure mode round 3 recorded as `M1a` and rated the ground for `medium`. `MUT-B` returns a path whose claim was never established, which `src/checks.rs:482-485` calls "what makes the returned path exclusively ours"; round 3 recorded that as `M1c` and called it the deciding measurement of its round. Both are still reachable at `:452`, one line below the line round 3 closed at `:533-541`.

The severity is `medium` on the same ground round 3 used for the identical consequences, reached through a different line. I considered arguing it down to `low` on the reasoning that the mutation is hypothetical (nothing is wrong in the shipped binary today) and rejected it: that reasoning would equally have downgraded `AD1a`, which this same record rated `medium` and fixed one round ago, and the trigger is a real environment fault (an unwritable or full temp dir), not an exotic one.

## Which reviewer's evidence and which reviewer's fix is stronger

**Evidence: `RG1`'s is stronger.** `MU1` measured one mutation (`Ok(false)`) and one binary A/B. `RG1` measured two mutations covering both directions of the arm and derived both of round 3's own recorded failure modes (`M1a` and `M1c`) from them, and it additionally established, by instrumenting the lost-claim arm, that the neighbouring comment's claim "no other test executes the LOST one through `claim_dir` itself" is true. Both reviewers' central measurement reproduced exactly here; neither overstated anything.

**Fix: `RG1`'s is stronger, measured.** I built both.

| Candidate | Diffstat, measured | Suite alone | clippy | under `MUT-A` | under `MUT-B` |
| --- | --- | --- | --- | --- | --- |
| `RG1`'s (13 lines inside `a_directory_claim_is_exclusive`) | `13 insertions(+), 0 deletions(-)` | GREEN, 386 passed, 0 failed | 0/0 | **RED**, 372 passed, 1 failed | **RED**, 372 passed, 1 failed |
| `MU1`'s (new test `claim_dir_propagates_a_non_collision_error_rather_than_reporting_it_taken`) | `22 insertions(+), 0 deletions(-)` | GREEN, 387 passed, 0 failed | 0/0 | **RED**, 373 passed, 1 failed | **RED**, 373 passed, 1 failed |

Failure text, verbatim:

```
RG1's, under MUT-A:
thread 'checks::tests::a_directory_claim_is_exclusive' panicked at src/checks.rs:1713:14:
a claim that cannot be made is an error, not a verdict: false

RG1's, under MUT-B:
a claim that cannot be made is an error, not a verdict: true

MU1's, under MUT-A:
thread 'checks::tests::claim_dir_propagates_a_non_collision_error_rather_than_reporting_it_taken' panicked at src/checks.rs:1722:14:
a claim under a missing parent directory must fail, not report the path taken: false

MU1's, under MUT-B:
a claim under a missing parent directory must fail, not report the path taken: true
```

Both work. `RG1`'s wins on three measured grounds:

1. **Size.** 13 lines against 22. `MU1` advertised its fix as "about 18 lines"; measured here it is 22, the fifth instance on this task of a prescription's advertised size not surviving being built by someone else. The gap is small and I am recording it as a calibration note, not as a mark against the finding, whose diagnosis was exactly right.
2. **No new fixture-name class.** `MU1`'s test calls `scratch("claim-dir-error")`, which creates `agent-scaffold-checks-test-{pid}-claim-dir-error` in the shared temp-dir root and removes it AFTER its assertions, so every RED run leaks one directory under a name that does not exist today. Measured directly: my four RED runs of `MU1`'s fix left four `agent-scaffold-checks-test-*-claim-dir-error` directories in my scratch `TMPDIR`. `RG1`'s fix reuses the `dir` the existing test already created, so its eight RED runs left directories only under the existing `claim-dir` name. The region reviewer's own count puts the module at 18 distinct fixture-name classes; `MU1`'s fix makes 19, `RG1`'s keeps 18.
3. **Coherence with the function's documented contract.** `claim_dir`'s doc at `:438-440` states three outcomes and `a_directory_claim_is_exclusive`'s own comment at `:1690-1691` says "Both outcomes matter". Putting the third assertion in that test makes one test cover one function's whole contract and makes the comment true of what follows it.

Against those, `MU1`'s advantage is a separately named test, so a failure names the arm rather than the function. That is real but small: `RG1`'s assertion message ("a claim that cannot be made is an error, not a verdict") names the arm in the failure output, as quoted above.

`MU1`'s comment also carries a marker (`REVIEW CANDIDATE FIX (r4b, MU1)`) that would have to be stripped, and it authors a claim about the seam ("that seam never calls real `claim_dir` and so cannot see this arm at all") which is a claim about the tree of exactly the class this project has measured re-seeding five times. `RG1`'s comment says only what its own assertion does. That is not the deciding factor but it points the same way.

---

# `RG2`: the seam's surviving doc sentence is false, and round 3's triage certified it on a grep that could not see the counterexample

**Verdict: VALID. Severity `low`. Fix required (deletion), with the site list EXTENDED on my own evidence.**

## The sentence is false, measured

`src/checks.rs:502-505`: "`reserve_runner_worktree` (above) with its claim injected, which is the only way to drive the outcome the filesystem will not produce on demand. Every real claim in this repository WINS: production takes one path at a time and the prune fixtures take theirs sequentially."

`claim_dir`'s lost-claim arm instrumented with `eprintln!("TRIAGE_PROBE_REAL_CLAIM_LOST {}", path.display())`, whole unit suite under `--nocapture`, five runs, probe then reverted:

```
run 1: real lost claims in whole unit suite = 1
run 2: real lost claims in whole unit suite = 1
run 3: real lost claims in whole unit suite = 1
run 4: real lost claims in whole unit suite = 1
run 5: real lost claims in whole unit suite = 1
```

The path identifies it, and isolating the test confirms it:

```
TRIAGE_PROBE_REAL_CLAIM_LOST <TMPDIR>/agent-scaffold-checks-test-1910093-claim-dir/claim

$ cargo test --bin agent-scaffold a_directory_claim_is_exclusive -- --nocapture
TRIAGE_PROBE_REAL_CLAIM_LOST <TMPDIR>/agent-scaffold-checks-test-1910874-claim-dir/claim
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 372 filtered out
```

5/5, deterministic, not sampled: exactly one real claim in this repository loses, and it is the one `a_directory_claim_is_exclusive:1705` deliberately loses through the real `claim_dir`. So "Every real claim in this repository WINS" is false, and "the only way to drive the outcome the filesystem will not produce on demand" is false, since the filesystem produces that outcome on demand eight lines above the tests that inject.

The enumeration is wrong too, and its own grep shows it: `grep -rn "reserve_runner_worktree" --include=*.rs .` lists `:1826`, inside `concurrent_reservations_never_share_a_runner_worktree_path`, which releases 8 threads x 250 reservations on a shared `Barrier`. "Production takes one path at a time and the prune fixtures take theirs sequentially" names two of the four real-claim sites and omits the one that is the opposite of sequential.

I considered the charitable reading, that "real claim" means a claim made at the reservation's own use site, and it does not save the sentence: under that reading the CONCLUSION happens to be true (my instrumented probe shows none of the 2000 concurrent claims loses) but the REASON given is still wrong, because those 2000 claims win by `{seq}` uniqueness and not by being taken sequentially. One reading makes the claim false, the other makes its stated ground false.

## The triager-error allegation holds

Verified verbatim. `docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage-r3.md:297` reads:

> "Every real claim in this repository WINS: production takes one path at a time and the prune fixtures take theirs sequentially" (verified true: `grep -n "reserve_runner_worktree" src/checks.rs` shows the production call and the fixtures all take the one-argument wrapper, sequentially)

The counterexample is a direct call to `claim_dir` at `:1705`. The pattern `reserve_runner_worktree` cannot match a `claim_dir` call, so the grep could not have seen it whatever the tree contained; and `:1826`, which contradicts "sequentially", is IN that grep's own output and was read past. The sentence's history confirms the counterexample was already there:

```
$ git log --oneline -S 'production takes one path at a time' -- src/checks.rs
14692f3 fix(checks): pin the reservation's collision path and the missing-TMPDIR fix

$ git log --oneline -S 'a_directory_claim_is_exclusive' -- src/checks.rs
339d26a fix(checks): pin the worktree claim, restore a missing TMPDIR, correct the prune's stated bound
```

`339d26a` (round 1) precedes `14692f3` (round 2). The test that falsifies the sentence landed before the sentence; it was false the day it was written and stayed false through two subsequent rounds.

**Record this as a lesson, at the triager level.** The project has already recorded the orchestrator-level form: "a grep returning zero is evidence about the PATTERN as much as about the repo". Round 3's error is the twin case, a grep returning NON-zero: a grep can only certify a claim whose counterexamples the pattern is capable of matching, and a certification must state which counterexample shapes the pattern can see. Here the pattern could see neither shape that falsifies the sentence, and one of the shapes it COULD see was in the output and unread. That makes `RG2` the fifth measured instance of a fix pass's prose manufacturing the next round's finding, and the second where the manufacturing step was a triager's certification rather than an implementer's authorship.

## I extended the fix's site list

`RG2` cites only `:502-505`. The identical false claim exists a second time:

```
$ grep -rni "real claim\|will not lose a claim\|not produce on demand" --include=*.rs .
src/checks.rs:503:/// to drive the outcome the filesystem will not produce on demand. Every real claim
src/checks.rs:1712:		// `claim_dir`. The filesystem will not lose a claim on demand (every real claim
```

`src/checks.rs:1711-1714`, inside `a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one`: "The filesystem will not lose a claim on demand (every real claim in this repository wins), so the claim is injected". That is the same false statement, and it sits SEVEN LINES below `a_directory_claim_is_exclusive`'s own comment at `:1691-1694` asserting that this very test executes the lost outcome through `claim_dir` itself. Fixing only `:502-505` would leave the duplicate to become round 5's finding, which is precisely the re-seeding pattern. Both sites are in the fix.

---

# `RG3`: nothing pins that the guard takes the reserved directory before the add

**Verdict: VALID BUT ACCEPT RESIDUAL. Severity `low`. No fix required this round. The reviewer's recommendation is upheld in its conclusion and CORRECTED in its stated ground.**

## The gap is real, reproduced

`MUT-C`, the `let _guard = WorktreeGuard {...};` block moved below the failed-add early return: **GREEN, 386 passed, 0 failed**; `cargo clippy --all-targets` 0 warnings, 0 errors. The ordering at `src/checks.rs:958-964`, which the comment there states and which Invariant B's opening clause at `:37-38` depends on, is executed by every run and asserted by nothing.

## I did not inherit the cost figure, and it does not survive measurement

The reviewer built a 78-line standalone integration test and recommended ACCEPT RESIDUAL on the ground that 78 lines exceeds the roughly 40 that got `AD1b` declined. I built two forms.

| Form | Cost, measured | On `3f49012` | Under `MUT-C` |
| --- | --- | --- | --- |
| Standalone new file `tests/checks_failed_worktree_add.rs`, written as small as I could | **72 lines**, one new test binary | GREEN, 387 passed, 0 failed; clippy 0/0 | **RED**, `a failed add must leave no reserved directory: ["agent-scaffold-checks-run-1914275-1785454956865741475-0"]` |
| Folded into the existing `tests/checks_missing_tmpdir.rs`, reusing its `git()` and `checks_with_tmpdir()` helpers | **`40 insertions(+)`, no new test binary**, plus an unmeasured module-doc rewrite | GREEN, that binary 2 passed, 0 failed | **RED**, `a failed add must leave no reserved directory: ["agent-scaffold-checks-run-1926228-1785455488882790585-0"]` |

Both forms assert that the child's dedicated `TMPDIR` is EMPTY after the run, which needs no copy of `RUNNER_PREFIX` in a second file. The reviewer's standalone 78 is honest for its form (mine came in at 72 writing it as tightly as I could), but the cheapest form is 40 lines and adds no test binary at all, which makes it CHEAPER than the `AD1b` fix that was declined at roughly 40 lines plus a second spawned-binary fixture. **The proportionality argument as the reviewer stated it does not hold.** Round 5 should not inherit "78 lines" as this finding's cost.

## Why it is still a residual

On VALUE, not on cost, and I am stating the ground explicitly because the cost ground has just failed.

What the mutation produces is one EMPTY, unregistered directory under the temp dir, in a run that is already exiting 2 on a `git worktree add` that has already failed. That is character-for-character the residual Invariant B at `:45-48` already documents, accepts, and hands to the operating system's temp-dir cleanup for the SIGKILL window; only the trigger differs. Nothing shipped is wrong: the code is correct, the comment at `:958-960` is true, and I re-derived that the reservation genuinely precedes the guard which genuinely precedes the add. What goes unpinned is a currently-true ordering whose violation yields an already-accepted leak class.

Compare the two coverage gaps this round on the same axis. `RG1`/`MU1`'s mutations put a WRONG DIAGNOSIS in front of a user on a real environment fault, and destroy the module's decisive exclusivity property. `RG3`'s mutation leaks an empty directory in an already-failing run. Same class of gap, materially different consequence, and the record has already calibrated the line here by declining `AD1b` for a lower-value pin.

The other half of the reviewer's reasoning I checked and it DOES hold: a unit-level pin is unavailable by construction, not by preference. `run()` reads `std::env::temp_dir()` with no seam, setting `TMPDIR` in-process needs `unsafe` `set_var` and would leak across the thread-shared test binary, the crate is a pure binary so no integration test can call `run()` directly, and any scan of the shared temp dir from a unit test races `concurrent_reservations_never_share_a_runner_worktree_path`'s 2000 in-flight directories. A spawned binary with its own `TMPDIR` is the only route.

**This is recorded, with both A/B measurements above, so round 5 does not re-derive it.** A reviewer arguing against its own finding deserved the scrutiny; the scrutiny found the conclusion sound and the stated ground wrong, which is worth having on the record in both directions.

---

# `RG4`: Invariant B's stated remedy would not close the window it names

**Verdict: VALID. Severity `low`. Fix required (a narrowing clause, the only authored prose in the fix set).**

## The over-claim is real

`src/checks.rs:49-56`. The first sentence names two causes of a registered orphan going unreclaimed: a `TMPDIR` reached through a symlink, and a run killed under a DIFFERENT `TMPDIR`. The second sentence says "Widening the prune to sweep the temp dir by prefix would close that window".

The claim is about a rejected alternative, so there is nothing to mutate; the check is on the referent, and it fails. A sweep of THIS process's temp dir reaches a directory that is physically in this process's temp dir. Cause 1 qualifies: `std::env::temp_dir()` returns `TMPDIR` unresolved, `read_dir` follows the link, so the sweep sees an orphan whose git-recorded path was symlink-resolved out from under the containment gate. Cause 2 does not: an orphan created under `TMPDIR=/A` is at `/A`, and no sweep of `/B`, at any prefix width, reaches `/A`.

I checked the alternative reading, that "sweep the temp dir by prefix" might mean filtering the REGISTERED worktree list by name prefix instead of by path containment, under which cause 2 would in fact be reclaimed. The sentence's own cost clause rules it out: filtering the registered list stays repo-scoped and could not give the prune "authority over other repositories' runner directories", which only a filesystem walk of the shared temp dir can. The cost clause and the remedy clause therefore describe the same filesystem-walk alternative, and that alternative reaches one of the two causes.

## Not a reopening

The settled list routes the BEHAVIOUR (canonicalising `temp_dir()`, the symlink-resolved skip, relative-`TMPDIR` validation) to a future roadmap step. This finding changes no behaviour and takes no position on whether the prune should be widened. It is that the sentence recording the declined alternative overstates what the alternative would buy, and that sentence is exactly what the routed-out step will inherit as its starting record. Same species as round 3's `AD3` and round 2's `X8b`, both of which this record ruled valid.

---

# What I checked and found HOLDING or NOT a finding

## The clock-rate discrepancy: the region reviewer's restraint was RIGHT, and no record correction is needed

The region reviewer declined to raise a measurement of `SystemTime::now()`'s rate that contradicted the step brief (it measured 0/100000 equal at two threads against the brief's 8679/100000, and 0.5% at sixteen threads against the brief's 71%), on the ground that three prior measurements disagree with it and its own method plausibly explains the gap. The task asked me to assess that restraint, so I arbitrated it rather than leaving two contradicting numbers on the record.

Standalone `rustc -O` probe, one process, three runs, four methods, bounded and self-limiting:

```
run 1:
A. one thread, n=100000: back-to-back repeats=0 min=50 p50=60 p90=100
B. two threads, barrier PER ROUND, rounds=100000: equal pairs=0 (0.00%)
B2. two threads, ONE barrier then tight loop, n=100000: non-unique readings=14860/200000 (7.43%)
C. 16 threads, barrier PER ROUND, 800000 samples: non-unique=6533 (0.82%)
C2. 16 threads, ONE barrier then tight loop, 800000 samples: non-unique=411766 (51.47%)

run 2:
A. min=50 p50=60 p90=61, repeats=0
B. 0 (0.00%)   B2. 13752/200000 (6.88%)   C. 7502 (0.94%)   C2. 440235 (55.03%)

run 3:
A. min=50 p50=60 p90=100, repeats=0
B. 0 (0.00%)   B2. 17463/200000 (8.73%)   C. 16520 (2.06%)   C2. 537037 (67.13%)
```

The brief records 8.7 percent at two threads and 71 percent at sixteen. **Method B2/C2 reproduces both: 6.88 to 8.73 percent at two threads (the brief's 8.7 sits inside my range) and 51 to 67 percent at sixteen.** Method B/C, which waits on a shared barrier once per sample, gives 0.00 percent and 0.8 to 2.1 percent, which is what the region reviewer measured.

So the two measurements are not in conflict about the machine; they are two different experiments. The brief's figures stand, the module's doc claims that rest on them stand ("advances in steps of tens of nanoseconds": measured p50 = 60 ns; "two threads sampling it at the same moment routinely read the same value": 7 percent at two threads, over half at sixteen), and **no record correction is due**. The reviewer's restraint was correct and so was its stated reason (that a per-sample barrier staggers the threads through a futex wake and that this alone drives the rate to zero); it simply attributed that method to itself as a doubt rather than identifying it as the actual difference. Recording the arbitration here so round 5 inherits one number and not two.

## Re-derived and holding

- **The commit under review changed zero production lines.** `git show --stat 3f49012` gives `src/checks.rs | 35 +++++----`, `31 insertions(+), 4 deletions(-)`, and the diff is one new test, one strengthened assertion, and one deleted doc clause.
- **Round 3's `AD1a` fix landed and works.** The three caller-side mutations both reviewers list as rows 1 to 3 are RED, and the new test `a_claim_error_that_is_not_a_collision_propagates_at_once_and_names_the_path` is what kills them. The finding above is one level below that fix, not a claim that it failed.
- **Round 3's `AD2` fix is sound as landed.** `message.contains(&format!("after {RUNNER_RESERVE_ATTEMPTS} attempts"))` searches for words the payload path cannot supply, unlike the bare `"16"` it replaced.
- **Round 3's `AD3` fix is a clean deletion** and did not re-seed by itself; what re-seeded was the certification written ABOUT the surviving clause, which is `RG2`.
- **No candidate in my fix set exceeds 100 columns**, checked with `awk 'length($0)>100'` over `src/checks.rs` with the whole set applied: no hits.

---

# Minimal fix set

Three fixes, all in `src/checks.rs`, no production line changed, no new test binary. **Two of the three are pure deletions.** Built together and measured as one set, then reverted.

Whole set, measured on this tree:

```
$ git diff --stat
 src/checks.rs | 35 +++++++++++++++++++++++------------
 1 file changed, 23 insertions(+), 12 deletions(-)
```

| State | Suite | clippy |
| --- | --- | --- |
| `3f49012` + the whole fix set | **GREEN, 386 passed, 0 failed** | 0 warnings, 0 errors |
| plus `MUT-A` | **RED**, `a claim that cannot be made is an error, not a verdict: false` (372 passed, 1 failed) | n/a |
| plus `MUT-B` | **RED**, `a claim that cannot be made is an error, not a verdict: true` (372 passed, 1 failed) | n/a |

## Fix 1, for `RG1`/`MU1`: assert `claim_dir`'s third documented outcome

`RG1`'s form, adopted over `MU1`'s on the measured grounds above. 13 insertions, 0 deletions, inside the existing `a_directory_claim_is_exclusive`, after `:1705`:

```rust
		assert!(!claim_dir(&path).unwrap(), "a second claim on the same path is lost");
+		// The THIRD outcome this documents, which neither assertion above reaches: a claim
+		// that fails for a reason OTHER than the path being taken propagates as an error
+		// rather than folding into either verdict. A regular file standing in for a parent
+		// directory produces one without needing a permissions fixture.
+		let file = dir.join("a-regular-file");
+		fs::write(&file, "not a directory\n").unwrap();
+		let error = claim_dir(&file.join("under-a-file"))
+			.expect_err("a claim that cannot be made is an error, not a verdict");
+		assert_ne!(
+			error.kind(),
+			io::ErrorKind::AlreadyExists,
+			"a real error must stay distinguishable from a lost claim: {error}"
+		);
		fs::remove_dir_all(&dir).unwrap();
```

Measured alone: `13 insertions(+), 0 deletions(-)`; GREEN 386 passed, 0 failed, clippy 0/0; RED under `MUT-A` and RED under `MUT-B`, both at 372 passed, 1 failed, with the failure text quoted earlier.

`assert_ne!` against `AlreadyExists` rather than an equality against a specific errno is the `X7` ruling applied, not reopened: what must hold is that a real error stays distinguishable from a lost claim, not which errno a kernel picks for a directory under a regular file. The four comment lines say only what the assertion does and author no claim about the tree or its history.

## Fix 2, for `RG2`: delete the false sentences at both sites

**Pure deletion, zero words authored**, which is the `AD3` remedy applied to `AD3`'s own species. Only surviving words are re-wrapped.

Site 1, `src/checks.rs:502-507`:

```
-/// `reserve_runner_worktree` (above) with its claim injected, which is the only way
-/// to drive the outcome the filesystem will not produce on demand. Every real claim
-/// in this repository WINS: production takes one path at a time and the prune
-/// fixtures take theirs sequentially. Production passes `claim_dir` and is otherwise
-/// unchanged; the whole reservation, including the temp-dir creation above the loop,
-/// lives here so the tests drive the same code the runner does.
+/// `reserve_runner_worktree` (above) with its claim injected. Production passes
+/// `claim_dir` and is otherwise unchanged; the whole reservation, including the
+/// temp-dir creation above the loop, lives here so the tests drive the same code the
+/// runner does.
```

Site 2, `src/checks.rs:1711-1714`:

```
 		// Layer 2's collision handling, driven at the use site rather than only at
-		// `claim_dir`. The filesystem will not lose a claim on demand (every real claim
-		// in this repository wins), so the claim is injected: this one records every name
-		// it is offered and loses the first two, which is the state the loop exists for.
+		// `claim_dir`. The claim is injected: this one records every name it is offered
+		// and loses the first two, which is the state the loop exists for.
```

Net for this fix: `-3` lines, no new words.

**I deliberately declined the region reviewer's own proposed replacement wording** ("A claim can be made to lose directly (`a_directory_claim_is_exclusive` does), but not at THIS use site: the loop draws a fresh, unpredictable name each attempt..."), and the reviewer itself offered it as replaceable and asked that the diagnosis be treated as the finding. Its content is true, but it is authored prose that names another test and describes what that test does, which is the exact class of claim that has become the next round's finding five times on this step, `RG2` included. What is lost by deleting instead is the seam's justification, and it is not lost from the file: the surviving first sentence at `:1711` ("Layer 2's collision handling, driven at the use site rather than only at `claim_dir`") states why the tests inject at the use site rather than testing `claim_dir` alone, and it is true.

## Fix 3, for `RG4`: narrow the remedy claim to the case it reaches

The only authored prose in the fix set, one clause, `src/checks.rs:53-56`:

```
 //!   under a different `TMPDIR`) is never reclaimed either. Widening the prune to
-//!   sweep the temp dir by prefix would close that window, at the cost of giving it
-//!   authority over other repositories' runner directories (Principle 18, least
-//!   authority), which is not a trade this module makes.
+//!   sweep the temp dir by prefix would close the symlink case, though not an orphan
+//!   left under a different `TMPDIR`, at the cost of giving it authority over other
+//!   repositories' runner directories (Principle 18, least authority), which is not a
+//!   trade this module makes.
```

I considered pure deletion of the whole remedy sentence first, per the deletion-over-authorship rule, and rejected it: unlike `AD3`'s clause and unlike `RG2`'s sentences, this one carries something no surviving sentence does, the Principle 18 reason the module declines to widen the prune, which is the record the routed-out roadmap step inherits. The edit therefore narrows an existing claim rather than adding a new one, and what it asserts ("`/A` is not under `/B`") cannot go stale as the tree changes.

Measured only as a comment edit admits: the whole set compiles, the suite is GREEN at 386 passed, 0 failed, and clippy is silent. `cargo test` runs no doc tests for a binary crate, so there is nothing stronger to measure on prose.

## What is NOT in the fix set, and why

- **`RG3`'s integration test**, in either the 72-line or the 40-line form. Accepted residual, reasoning and both A/B measurements under `RG3`.
- **`MU1`'s separate test function.** Superseded by Fix 1, which kills the identical mutations for 13 lines instead of 22 and adds no fixture-name class.
- **The region reviewer's replacement wording for the seam doc.** Declined in favour of deletion, above.
- **Anything on the settled list.** Untouched, and no measurement here disturbs any of it.

---

# Routed out to a new roadmap step

Nothing new. `RG3` is an accepted residual on this step's record rather than a routed-out item: it needs no design decision and no risk class of its own, only a note that the ordering is unpinned and what the leak is if it breaks, which this file now carries with the mutation and the two A/B measurements.

One item for the EXISTING routed-out step to inherit, not a new one: when that step revisits the prune's temp-dir gate, `RG4` is the reason its starting record now says the prefix sweep closes the symlink case only. Both causes still need a decision there; only the description of the declined alternative is corrected here.

---

# ROUND OUTCOME

**Round 4 is NEW_VALID.** Three findings require a fix: one `medium` (`RG1`/`MU1`, deduplicated from two reviewers) and two `low` (`RG2`, `RG4`). One finding is valid and accepted as a residual (`RG3`, `low`). Nothing was dismissed.

**Cap arithmetic, stated for the orchestrator.** The step is classified `risky` and needs TWO CONSECUTIVE CLEAN ROUNDS to converge. Its streak entering round 4 was 0. Round 4 is NEW_VALID, so **the streak stays 0** and a fix pass must land. Round 5 is the last round permitted by the cap of 5. Even if round 5 is clean, the streak reaches only 1, which is short of the 2 required. **This step CANNOT converge within its cap.** The orchestrator owes the human an escalation decision.

**On shading.** I record explicitly that the convergence pressure was visible to me throughout and that it did not move any verdict. `RG1`/`MU1` alone decides the round: two independent mutations of `src/checks.rs:452` survive the full 386-test suite, and the built binary reports a permissions fault as 16-attempt name-collision exhaustion under one of them. That measurement does not become less true because round 4 was the round that had to be clean. Because the round was already NEW_VALID on that finding, the marginal cost of also fixing `RG2` and `RG4` against the cap is zero, and I have checked that I would rule the same way with the cap removed: both are false or over-claiming statements in the artifact's own doc comments, the same species this record has twice ruled valid, and two of the three fixes are pure deletions that re-seed nothing. In the other direction, I declined to manufacture work on `RG3` even though its cheapest fix turned out to cost less than the reviewer claimed, because what it pins is a currently-true ordering whose violation produces a leak class Invariant B already accepts.

---

# Reverted state

**Everything I applied is reverted.** Measured after the last revert and before this file was written:

```
$ git rev-parse HEAD
3f490128ef34c608dac134a313bdb69972e0daf0

$ git status --short
?? docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-reviewer-r4-mutation.md
?? docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-reviewer-r4-region.md

$ git diff HEAD
(empty)
```

The only untracked entries are the two reviewer files that were placed in this worktree for me, plus this triage file, written after those commands were run. No tracked file differs from `3f49012`. Baseline re-confirmed on the reverted tree: `cargo test` 373 + 5 + 1 + 1 + 3 + 1 + 2 = 386 passed, 0 failed; `cargo clippy --all-targets` 0 warnings, 0 errors.

Applied and reverted, in order: `MUT-A`, `MUT-B` (`claim_dir`'s error arm as lost / as won), three binary builds for the `chmod 555` A/B, `RG1`'s candidate fix alone and under both mutations, `MU1`'s candidate fix alone and under both mutations, the `claim_dir` lost-arm `eprintln!` instrumentation, `MUT-C` (guard after the add), the whole fix set alone and under both mutations, `RG3`'s standalone candidate test file (created and deleted), and `RG3`'s folded candidate test in `tests/checks_missing_tmpdir.rs` alone and under `MUT-C`. Reverts were `git checkout -- src/checks.rs tests/checks_missing_tmpdir.rs` and `rm` of the file I created. This triage file is left uncommitted for the orchestrator to collect; I committed nothing.

# Temp-directory hygiene

- **Directories created in `/tmp`: 0.** `ls -d /tmp/agent-scaffold-* | wc -l` returned **65** before I touched anything and **65** after my last revert, and a `diff` of the two full listings is IDENTICAL, so none of the 65 is mine and none was touched or deleted. `find /tmp -maxdepth 1 -name 'agent-scaffold-checks-run-*' | wc -l` returns **0**. `find /tmp -mindepth 1 -maxdepth 1 -newermt "2026-07-31 00:30"`, unrestricted by name, returns nothing but the session's own `claude-1000` scratch root.
- `TMPDIR` was exported to `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/triage-r4-tmp` in every single Bash call, including the ones that only read git history, since each call is a fresh shell.
- Probes were bounded: 5 instrumented suite runs for the lost-claim count, 3 runs of a single-process clock probe (100000 + 200000 + 200000 + 800000 + 800000 samples per run, self-limiting), one binary run per A/B arm, and single suite runs per mutation. No exhaustion-path probe, no multi-process reservation probe, and nothing scaled up the 2000-reservation concurrency test.
- Fixtures created inside the scratch `TMPDIR` and removed at the end: three built binaries, three saved patches, one standalone clock probe and its source, one probe shell script, two `/tmp` listings, three scratch git repos with a `chmod 555` directory each (restored to 755 before deletion), and the fixture directories left behind by deliberately RED runs (eight `agent-scaffold-checks-test-*-claim-dir`, four `agent-scaffold-checks-test-*-claim-dir-error` from `MU1`'s candidate, one `agent-scaffold-addfail-*`). The scratch `TMPDIR` was emptied after this file was written.
