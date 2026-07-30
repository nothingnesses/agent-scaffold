# Triage: `checks-runner-worktree-name-collision` (commit `6a726ed`, round 3)

Adjudicated in an isolated worktree at `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage93-r3`, branch `triage/checks-collision-r3`, HEAD `6a726ed989c74a3620995bd84ba7474d694f0f96`. Every mutation, probe and prescribed fix below was applied here, built, measured, and reverted. `TMPDIR` was exported to this session's scratchpad for every build, test and probe; nothing was created in `/tmp`.

Baseline reconfirmed here, not quoted from either reviewer: `cargo test` 372 + 5 + 1 + 1 + 3 + 1 + 2 = **385 passed, 0 failed**; `cargo clippy --all-targets` produced 0 warnings and 0 errors.

Inputs: `checks-runner-worktree-name-collision-reviewer-r3-fixverify.md` (fix-verification lens, ZERO findings) and `checks-runner-worktree-name-collision-reviewer-r3-adversarial.md` (adversarial lens, `AD1` medium, `AD2` low, `AD3` low). Different models, deliberately.

## Deduplication

| Triage id | From | Subject | Verdict |
| --- | --- | --- | --- |
| AD1a | adversarial AD1, claim-error half | The claim's error arm at `:535-543` is executed by nothing, and three separate mutations of it are green. | VALID, fix required, `medium` |
| AD1b | adversarial AD1, temp-dir-creation half | The error arm of `create_dir_all(&temp)` at `:521-526` is executed by nothing. | VALID BUT ACCEPT RESIDUAL, `low` |
| AD2 | adversarial AD2 | The exhaustion test's bound assertion is satisfiable by the payload's own digits. | VALID, fix required, `low` |
| AD3 | adversarial AD3 | The new `:502-509` doc comment states two things the same commit made false. | VALID, fix required (by DELETION), `low` |

**One split, no merges.** `AD1` is SPLIT because its two halves have different consequences, different fix costs by an order of magnitude, and different verdicts: the claim half is a unit test on a seam that already exists, the temp-dir half needs a whole new integration binary because `std::env::temp_dir()` has no seam. The reviewer's own `medium` rests on the claim half; I am not letting the cheap half carry the expensive one into the fix set, nor the expensive one drag the cheap one out.

**Overlap between the two lenses, and it matters.** `AD2` and the fix-verification lens's "disclosed item" are two different assertions in the SAME test, three lines apart, and `AD3` and the fix-verification lens's second re-seeding note are two different clauses of the SAME sentence. In both cases the fix-verification lens opened the item, ruled on the half it quoted, and stopped inside it. Ruled under "Is the fix-verification zero correct" below.

**Nothing settled was reopened.** The out-of-scope list (temp-dir canonicalisation, the prune's symlink gate, relative-`TMPDIR` validation, the `X1` `fetch_add` residual, `X5`, `X7`, the (a)+(d) choice, `nanos()`, the SIGKILL-leak trade, the no-CHANGELOG call, the uniqueness property) is untouched by all three findings and by all three fixes. I have no disagreement with any of those exclusions and am raising no clearly-marked section against them.

---

# AD1a: the claim's error arm is executed by nothing, and one mutation of it returns a path that was never claimed

**Verdict: VALID (fix required). Final severity: `medium`** (confirming the reviewer's rating, on different and stronger evidence than the reviewer gave).

## Reproduced: the reviewer's two mutations

Each applied alone to the committed tree, full `cargo test` plus `cargo clippy --all-targets`, then reverted.

**M1a**, `src/checks.rs:535-543`, `claim(&path).map_err(...)?` -> `let claimed = claim(&path).unwrap_or(false);`

```
test result: ok. 372 passed; 0 failed
test result: ok. 5 passed  /  1 passed  /  1 passed  /  3 passed  /  1 passed  /  2 passed
```

**GREEN, 385 passed, 0 failed**; clippy 0 warnings, 0 errors. A/B of the built binary against a scratch repo with one trivial `lint` check, `TMPDIR` naming an existing but `chmod 555` directory:

```
committed 6a726ed:
error: could not reserve the runner worktree directory <TMPDIR>/probe/unwritable/agent-scaffold-checks-run-1706239-1785448382588013220-0: Permission denied (os error 13)
exit=2

M1a:
error: could not reserve a unique runner worktree directory after 16 attempts (last tried <TMPDIR>/probe/unwritable/agent-scaffold-checks-run-1705579-1785448351412603030-15)
exit=2
```

Reproduces the reviewer's claim exactly: a permissions fault reported as name-collision exhaustion, after 16 pointless `mkdir` syscalls.

**M1b**, same lines, `-> let claimed = claim(&path)?;` **GREEN, 385 passed, 0 failed**; clippy silent. Same probe:

```
M1b:
error: Permission denied (os error 13)
exit=2
```

"naming neither the operation nor the path", which is the failure mode `:517-520` says is prevented and the requirement round 1 recorded as `T2` and round 2 re-endorsed under `X7`.

## The mutation NEITHER lens tried, which is why this is `medium` and not `low`

The reviewer argued `medium` from message quality and conceded it considered `low`. Message quality alone would not carry `medium` here under this project's rubric: severity rates the consequence of what stays unfixed, and a wrong noun on a run that already exits 2 is not what made round 2's `X2` a `medium`. So I looked for the mutation of this arm that touches CORRECTNESS rather than diagnosis, and it exists.

**M1c**, `src/checks.rs:535-543`, `-> let claimed = claim(&path).unwrap_or(true);` (an errored claim counts as WON):

```
test result: ok. 372 passed; 0 failed
... 5 / 1 / 1 / 3 / 1 / 2 ...
clippy warnings+errors: 0
```

**GREEN, 385 passed, 0 failed**, clippy silent. Under M1c the function returns a path whose claim it never established, which is precisely the outcome the module's own doc calls decisive at `src/checks.rs:483-485`: "`claim_dir` (above) creates the directory or reports it already taken, atomically, and that outcome (not an entropy argument) is what makes the returned path exclusively ours." That sentence is false under M1c and no test observes it.

That is the same class round 2 rated `medium` under `X2` (layer 2's verdict at its use site), it is in the same three lines, and it is not covered by `X2`'s fix: `X2` pinned what happens when a claim LOSES, and says nothing about what happens when a claim ERRORS. Round 2's table recorded this arm as "not tried". So `medium` is the right rating and it does not rest on the reviewer's message-quality argument.

## Not a reopening

Round 2's `X2` covered the `if claimed` verdict and the retry bound; both are now RED (measured below and by both lenses). The error arm was never diagnosed, never prescribed, never declined. Round 2's refusal of "a test per branch" was a refusal of the four mutations in ITS table, all of which are now resolved (two fixed, one residual with a corrected reason, one fixed). This is new evidence, not a re-argument.

---

# AD1b: the temp-dir creation's error arm

**Verdict: VALID BUT ACCEPT RESIDUAL. Final severity: `low`.**

**M17**, `src/checks.rs:521-526`, `fs::create_dir_all(&temp).map_err(...)?;` -> `let _ = fs::create_dir_all(&temp);` **GREEN, 385 passed, 0 failed**, clippy 0 warnings, 0 errors. A/B of the built binary with `TMPDIR` set two levels under a regular file:

```
committed 6a726ed:
error: could not create the temp directory <SCRATCH>/probe/afile_parent/afile/sub: Not a directory (os error 20)
exit=2

M17:
error: could not reserve the runner worktree directory <SCRATCH>/probe/afile_parent/afile/sub/agent-scaffold-checks-run-1709899-1785448441466086604-0: Not a directory (os error 20)
exit=2
```

The mutation is real and the suite does not see it. I am accepting it, for three measured reasons rather than one asserted one.

1. **What degrades is one noun.** Both messages name a path, both name the same errno, both exit 2, and the mutated path CONTAINS the temp dir the committed message names. Compare M1a, which points the user at an entirely different cause, and M1b, which names nothing. This is a materially smaller consequence than AD1a's and the reviewer's file does not separate them.
2. **The pin costs a whole new integration binary.** `std::env::temp_dir()` has no seam and cannot be redirected from a unit test without the unsafe `std::env::set_var` the new integration test's own doc comment explains away. So closing M17 means a second spawned-binary test with its own scratch git repo, roughly 40 lines, to pin a noun.
3. **The alternative is the move round 2 already declined.** Injecting the temp dir would be a SECOND testability parameter on `reserve_runner_worktree`, which is exactly what round 2 refused for the clock parameter under plan Principle 2 (minimal by default) and `Q-66` proportionality. Taking it here for a weaker payoff than the clock parameter offered would contradict that ruling.

Recorded as a residual with the A/B above so the next round does not re-derive it. `Q-66` proportionality, plan Principle 2.

---

# AD2: the exhaustion test's bound assertion is satisfiable by the payload's own digits

**Verdict: VALID (fix required). Final severity: `low`** (confirming the reviewer's rating; CORRECTING its "pins nothing" framing to "pins it at a machine-dependent rate", which is a different and in one way worse defect).

`src/checks.rs:1760-1763` searches the error message for `RUNNER_RESERVE_ATTEMPTS.to_string()`, the two characters `"16"`, in a string that ends with a path carrying a 7-digit pid, a 19-digit nanosecond reading, a sequence value and the whole `TMPDIR` string.

## Reproduced, and the rate is NOT what the reviewer measured

**M4**, `src/checks.rs:549-556`, drop the bound from the exhaustion message.

The reviewer measured 100 spurious passes in 100 trials and attributed it to this machine's pids beginning `"16"`. That is no longer true of this machine: pids here are now in the 1.71 million range (`1705579`, `1706239`, `1709899`, `1713571`, observed above and below), and the scratch `TMPDIR` string contains zero occurrences of `"16"` (`grep -o "16" | wc -l` -> `0`). So I re-measured rather than reproducing a stale number.

- Full `cargo test` under M4: **RED**, `a_claim_that_never_wins_fails_at_the_attempt_bound` FAILED, 371 passed, 1 failed.
- The test alone, run as a fresh process 100 times: **PASS=10, FAIL=90.** So it is caught 90 times in 100 and spuriously passes 10 times in 100.
- Mechanism, captured with a temporary `eprintln!("PROBE_MSG={message}")` above the assertion under M4, 100 fresh processes, probe reverted: **100 messages collected, 16 contain `"16"`.** Two spurious-pass samples:

```
PROBE_MSG=could not reserve a unique runner worktree directory (last tried .../agent-scaffold-checks-run-1713571-1785448609044186163-15)
PROBE_MSG=could not reserve a unique runner worktree directory (last tried .../agent-scaffold-checks-run-1713643-1785448609096161592-15)
```

and a kill:

```
PROBE_MSG=could not reserve a unique runner worktree directory (last tried .../agent-scaffold-checks-run-1713565-1785448609041498793-15)
```

The `"16"` is supplied by the 19-digit nanosecond reading (`...044186163...`, `...096161592...`), with the pid contributing nothing on this machine. Two independent 100-trial samples of the same Bernoulli give 10/100 and 16/100, so about 13% here.

**This is the honest statement of the defect, and it is not the reviewer's.** The assertion does not "pin nothing"; it pins the bound with a probability that depends on the pid range, the clock digits and the `TMPDIR` string. Observed at 0% kill on the reviewer's machine and 87 to 90% kill on mine, from the same code. A mutation guard whose kill rate is a property of the developer's pid counter is a defect of exactly the species this whole step exists to remove: it makes a test result a statement about the environment. That is what justifies the fix, more than the reviewer's flat claim did.

## Why `low` and why fix anyway

`low`, because what stays unfixed is a test-quality problem in a guard that already works most of the time in the environment where it was measured, and nothing shipped is wrong. Fixed anyway, because the remedy is ONE LINE, it authors no new prose, and it converts a probabilistic guard into a deterministic one. Measured below.

## The related observation, deliberately NOT raised as a finding

`RUNNER_RESERVE_ATTEMPTS` 16 -> 3: **GREEN, 385 passed, 0 failed.** So the constant's VALUE is unpinned above 3 by the whole test, not just by one assertion. I confirm the reviewer's decision not to raise it and agree with its reason: it is a tuning value and no property depends on it being 16. Under the prescribed fix it stays GREEN (measured), and I am explicitly not widening the fix to change that; see the interaction ruling below.

---

# AD3: the new doc comment states two things the same commit made false

**Verdict: VALID (fix required). Final severity: `low`** (confirming). **The remedy is DELETION, not the reviewer's proposed rewrite.**

`src/checks.rs:502-509`, written by `6a726ed`:

> ... so nothing ever exercises the lost-claim verdict, the retry, or the exhaustion error at their use site, and each of those can be deleted with a green suite.

All three of the "each of those can be deleted with a green suite" cases are false against the tree the sentence sits in, measured here:

| Deletion the sentence says is green | Measured on `6a726ed` |
| --- | --- |
| the lost-claim verdict (`if claimed` -> `if claimed \|\| true`) | **RED**, `a_claim_that_never_wins_fails_at_the_attempt_bound` and `a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one` both FAILED, 370 passed, 2 failed |
| the retry (`RUNNER_RESERVE_ATTEMPTS` 16 -> 1) | **RED**, `a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one` FAILED, 371 passed, 1 failed |
| the exhaustion error | asserted directly by `a_claim_that_never_wins_fails_at_the_attempt_bound` (`expect_err`, `ErrorKind::AlreadyExists`, the message) |

The first clause ("nothing ever exercises ... at their use site") has a defensible reading, the one the fix-verification lens took: scoped by the preceding "Every real claim in this repository WINS", it means "nothing with a REAL claim exercises them". The second clause does not have that reading. Deletability with a green suite is a statement about the whole suite, and the whole suite now contains the two tests the same commit added 200 lines below. It is false, flatly, and the paragraph contradicts itself inside four lines: it opens by introducing the injection as "the only way to drive the outcome" and then says nothing drives it.

**Required, on round 2's own `X8b` precedent, not on a new standard.** `X8b` was a comment made false by tests landing in the same commit ("Both outcomes matter, and neither was executed by any other test"), and round 2 required its correction as a consequence of Fix 2. This is the identical species, in the identical commit, and it was missed. Declining it now would be inconsistent with the ruling that produced the very fix that made it false.

**The remedy is deletion and this is the point of the finding.** `AD3` exists because `6a726ed`'s prose pass authored a motivation paragraph that its own tests falsified; the reviewer's proposed remedy is to author a replacement sentence in the past tense, which is another authored claim that the next commit can falsify again. This project has measured four times that prose-authoring fix passes manufacture the next round's finding. The false clause is pure motivation-history and carries nothing the surviving sentences do not: the first sentence already states why the seam exists ("the only way to drive the outcome the filesystem will not produce on demand"), and "Every real claim in this repository WINS: production takes one path at a time and the prune fixtures take theirs sequentially" already states why. **Delete the clause, write nothing.** Measured below: the resulting paragraph is coherent and every remaining sentence is true.

---

# Is the fix-verification lens's zero correct

**Ruling: CORRECT on the three authorised fixes, and its disclosed-item ruling is SOUND and reproduced. INCOMPLETE on its own "manufacture nothing new" clause: it missed AD2 and the false half of AD3, and both misses are inside items it opened and then stopped short within.**

**What I reproduced and confirm.** Its decisive independent mutation for the disclosed item: `for _ in 0 .. RUNNER_RESERVE_ATTEMPTS` -> `for _ in 0 .. 1u32` with the constant left at 16.

```
test checks::tests::a_claim_that_never_wins_fails_at_the_attempt_bound ... FAILED
test checks::tests::a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one ... FAILED
test result: FAILED. 370 passed; 2 failed
```

So `assert_eq!(offered.len(), RUNNER_RESERVE_ATTEMPTS as usize, ...)` does genuine, non-tautological work: it pins the loop bound against its own named constant. The lens's conclusion, that the self-reference is correct as prescribed rather than a defect, is right, and its second ground (the round-2 triage prescribed the symbolic constant in those words and measured the identical mutation itself) is right. I am not disturbing it.

**The interaction the orchestrator asked about, measured.** Both assertions live in `a_claim_that_never_wins_fails_at_the_attempt_bound`, and their blind spots are complementary, not identical:

- `offered.len() == RUNNER_RESERVE_ATTEMPTS` is SELF-REFERENTIAL: it reads the constant to know what to expect, so it cannot see the constant being retuned. It does see the loop bound diverging from the constant.
- `message.contains("16")` is PAYLOAD-SATISFIABLE: it can be satisfied by digits the path supplies, so it sees the bound being dropped from the message only at a machine-dependent rate.

Together they mean the test's two "the bound" assertions both pass under `RUNNER_RESERVE_ATTEMPTS` 16 -> 3 (measured GREEN, 385). That is the interaction, and it is real. **I am NOT requiring a fix for it**, because the fix-verification lens's ruling on the symbolic form is correct and the constant is a tuning value no property depends on; the prescribed AD2 fix deliberately keeps the symbolic coupling and leaves 16 -> 3 green (measured GREEN, 373 unit tests, under the fix). Recorded so it is not re-found.

**Where the zero fell short.** The lens states its scope as "did the three authorised fixes land exactly, close what they claim, and manufacture nothing new", and it ran a "Re-seeding: every new or changed sentence" sweep. Two gaps, both one step further into things it had already opened:

1. It ruled on ONE of the two bound assertions in the new test and did not examine the other, three lines below it. An assertion introduced by the fix that does not reliably pin what its own failure message says it pins is squarely inside "manufacture nothing new".
2. On `:502-509` it quoted and ruled on the first clause and did not quote the second. Its own note concedes the sentence "reads best as 'nothing outside the injected seam' rather than 'nothing at all'", which is a charitable-reading defence of the first clause only; the second clause is not rescued by it and is false, as measured above.

Its sweep was, by its own account, over SENTENCES. The two things it missed are one ASSERTION and one CLAUSE. That is the shape of the gap, and it is a real miss rather than a difference of judgement, since neither item was weighed and declined.

**What this does not mean.** The zero is not wrong about the three fixes: I re-derived its Fix 1, Fix 2 and Fix 3 mutation rows (the `create_dir_all` deletion, `if claimed || true`, `RUNNER_RESERVE_ATTEMPTS` 16 -> 1, `fetch_add` -> `load`) and every row reproduces as it reported. The disagreement between a zero and a three is explained by scope and by depth, not by one lens being unsound: the fix-verification lens verified the prescription was executed, the adversarial lens attacked what the execution left, and the two findings it did not reach are precisely the ones that need an attack rather than a comparison against the prescription.

---

# Minimal fix set

Three fixes. Each was implemented here, measured green on `6a726ed` and RED under the mutation it claims to kill, then reverted. **No production code changes at all**: one new test, one changed assertion line, one deleted clause. Combined diff measured at `31 insertions, 4 deletions` in `src/checks.rs`, one file.

Combined state with all three applied: `cargo test` 373 + 5 + 1 + 1 + 3 + 1 + 2 = **386 passed, 0 failed**; `cargo clippy --all-targets` 0 warnings, 0 errors; **10 consecutive full `cargo test` runs: GREEN 10, RED 0**.

## Fix A (closes AD1a). One unit test on the existing seam, no production change.

Add after `a_claim_that_never_wins_fails_at_the_attempt_bound`. This is the exact text I built and measured; use it rather than re-wording it, so the fix pass authors no prose of its own (this is the anti-re-seeding measure, and `AD3` is why it is stated).

```rust
	#[test]
	fn a_claim_error_that_is_not_a_collision_propagates_at_once_and_names_the_path() {
		// The loop's third outcome: an error that is NOT a lost claim (an unwritable temp
		// dir) propagates on the first attempt rather than being retried to exhaustion and
		// misreported as a collision, and it reaches the user naming the operation and the
		// path it failed on.
		let offered = std::cell::RefCell::new(Vec::new());
		let error = reserve_runner_worktree_with(std::process::id(), |path| {
			offered.borrow_mut().push(path.to_path_buf());
			Err(io::Error::from(io::ErrorKind::PermissionDenied))
		})
		.expect_err("a claim error must fail the reservation");

		let offered = offered.into_inner();
		assert_eq!(offered.len(), 1, "a non-collision error is not retried");
		assert_eq!(
			error.kind(),
			io::ErrorKind::PermissionDenied,
			"a real error stays distinguishable from exhaustion, which is AlreadyExists"
		);
		let message = error.to_string();
		assert!(
			message.contains("could not reserve the runner worktree directory"),
			"the error must name the operation it failed at: {message}"
		);
		let tried = offered[0].display().to_string();
		assert!(message.contains(&tried), "the error must name the path it failed on: {message}");
	}
```

Measured, one test killing all three mutations of the arm:

| State | Result |
| --- | --- |
| `6a726ed` + Fix A/B/C | GREEN, 386 passed, 0 failed; clippy 0/0 |
| plus M1a (`claim(&path).unwrap_or(false)`) | **RED**, `assertion left == right failed: a non-collision error is not retried, left: 16, right: 1` (372 passed, 1 failed) |
| plus M1b (`claim(&path)?`) | **RED**, `the error must name the operation it failed at: permission denied` (372 passed, 1 failed) |
| plus M1c (`claim(&path).unwrap_or(true)`) | **RED**, `a claim error must fail the reservation: "<TMPDIR>/agent-scaffold-checks-run-1730335-1785449049379705605-0"` (372 passed, 1 failed) |

It creates no directories (the injected claim errors before touching the filesystem), so it leaks nothing and needs no cleanup.

## Fix B (closes AD2). One line.

`src/checks.rs:1761`:

```
message.contains(&RUNNER_RESERVE_ATTEMPTS.to_string()),
  ->
message.contains(&format!("after {RUNNER_RESERVE_ATTEMPTS} attempts")),
```

Keeps the symbolic constant, which is what the round-2 triage prescribed and what the fix-verification lens correctly ruled sound; adds the surrounding words from the production format string, which a path cannot supply. No new prose: both fragments already exist in the file.

Measured:

| State | The exhaustion test, 100 fresh processes |
| --- | --- |
| `6a726ed` + M4 (bound dropped from the message) | PASS=10, FAIL=90 (spuriously green 10% here, 100% on the adversarial reviewer's machine) |
| `6a726ed` + Fix A/B/C + M4 | **PASS=0, FAIL=100** |

Full `cargo test` under Fix + M4: RED, `a_claim_that_never_wins_fails_at_the_attempt_bound` FAILED, 372 passed, 1 failed. Full `cargo test` under Fix + `RUNNER_RESERVE_ATTEMPTS` 16 -> 3: GREEN, 373 unit tests passed, 0 failed, which is the settled symbolic-coupling behaviour left deliberately unchanged.

## Fix C (closes AD3). Deletion only, nothing authored.

`src/checks.rs:502-509`, delete the false clause and close the sentence:

```
/// in this repository WINS: production takes one path at a time and the prune
/// fixtures take theirs sequentially, so nothing ever exercises the lost-claim
/// verdict, the retry, or the exhaustion error at their use site, and each of those
/// can be deleted with a green suite. Production passes `claim_dir` and is otherwise
  ->
/// in this repository WINS: production takes one path at a time and the prune
/// fixtures take theirs sequentially. Production passes `claim_dir` and is otherwise
```

Net `-2` lines, zero words written. The surviving paragraph reads: the seam is "the only way to drive the outcome the filesystem will not produce on demand"; "Every real claim in this repository WINS: production takes one path at a time and the prune fixtures take theirs sequentially" (verified true: `grep -n "reserve_runner_worktree" src/checks.rs` shows the production call and the fixtures all take the one-argument wrapper, sequentially); "Production passes `claim_dir` and is otherwise unchanged; the whole reservation, including the temp-dir creation above the loop, lives here so the tests drive the same code the runner does" (verified true by the fix-verification lens's line-by-line body diff, which I did not re-derive).

**Do not put the deleted clause back in the past tense.** That is the reviewer's proposal and I am declining it explicitly: it manufactures a new authored claim about the history of a tree, for zero information the surviving sentences do not already carry, on a step whose record shows four separate instances of authored fix-pass prose becoming the next round's finding. Deletion re-seeds nothing.

## Not required, and deliberately so

- **AD1b's integration test for M17.** Accepted as a residual, reasons and measurements under AD1b. Roughly 40 lines and a second spawned-binary fixture to pin one noun in an error that already names the path and the errno.
- **Pinning `RUNNER_RESERVE_ATTEMPTS`'s value with a literal.** Measured GREEN at 16 -> 3 both with and without the fix set. It is a tuning value, no property depends on it, the fix-verification lens's ruling that the symbolic form is the prescribed one is sound, and hardcoding `16` would be a strictly additional assertion nobody authorised.
- **Anything on the settled list.** Untouched.

---

# Route to a NEW ROADMAP STEP

**Nothing new.** The one roadmap step round 2 opened (the raw, uncanonicalised, unvalidated `std::env::temp_dir()` shared by the reservation at `:514` and the prune's gate at `:593`) still covers everything that belongs outside this step. AD1a, AD1b, AD2 and AD3 are all inside `reserve_runner_worktree_with` and its two tests, which is this step's own surface.

---

# Round outcome for the orchestrator

**Round 3 is NEW_VALID. The streak stays at 0.**

Three fixes required: AD1a at `medium`, AD2 at `low`, AD3 at `low`. AD1b is a recorded accepted residual at `low`. Total cost: one 27-line unit test, one changed assertion line, one deleted clause, all in `src/checks.rs`, zero production code changed.

**Severity movement.** No raises, no drops against the reviewer's ratings. `AD1` splits, and its `medium` follows the half I am requiring; the reviewer's stated ground for `medium` (message quality) would not have carried it, and the ground that does is M1c, which neither lens tried.

**I did not shade toward convergence, and the cost is real.** The step is on round 3 of a 5-round cap with a 2-consecutive-clean requirement, so declaring NEW_VALID here means rounds 4 and 5 must BOTH come back clean or the step hits its cap. I considered accepting all three as residuals to buy a clean round, and rejected it. The deciding measurement is M1c: on the committed tree, a mutation that makes the reservation return a path whose claim was never established leaves 385 tests green and clippy silent, on a step classified `risky` precisely because a shared runner path ends at `WorktreeGuard::drop` calling `remove_dir_all` on a directory another live run is inside. Round 2 refused to converge on exactly that argument, with the seam not yet built; the seam is now built, the test is 27 lines, and declining it would be the shading. AD2 and AD3 travel with it at a cost of one line and minus two lines, and both were required by precedents already on this step's record (`Q-66`'s reproducible-evidence standard for AD2, round 2's `X8b` for AD3).

**Prescription tested, not trusted.** The adversarial lens's AD1 fix was described as two tests, 77 added lines, 387 passed. I built it myself and it is smaller than described: ONE test, 27 lines, kills all three mutations of the claim arm including one the reviewer did not have, and the second test it proposed is the part I measured as disproportionate and declined. Its AD2 and AD3 proposals were both explicitly unbuilt; AD2's is right in substance and I measured the exact form above, AD3's is wrong in method (authoring replacement prose where deletion suffices) and I replaced it. So the pattern this task has recorded three times holds a fourth time in a weaker form: the diagnoses were sound, and none of the three prescriptions was adoptable exactly as written.

---

# Worktree and temp-directory state

**Everything was reverted.** Final state, measured after the last revert:

```
$ git rev-parse HEAD
6a726ed989c74a3620995bd84ba7474d694f0f96

$ git status --short
?? docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-reviewer-r3-adversarial.md
?? docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-reviewer-r3-fixverify.md

$ git diff HEAD
(empty)
```

The two untracked files are the reviewer inputs copied in by the orchestrator; this triage file is written after those commands and is left uncommitted. Verification on the reverted tree: `cargo test` 372 + 5 + 1 + 1 + 3 + 1 + 2 = 385 passed, 0 failed; `cargo clippy --all-targets` 0 warnings, 0 errors.

Mutations applied and reverted, in order: M1a, M1b (claim error swallowed / unwrapped), M17 (temp-dir creation error swallowed), M4 (exhaustion bound dropped from the message) plus a temporary `eprintln!` probe, `RUNNER_RESERVE_ATTEMPTS` 16 -> 3, the loop bound decoupled from the constant (`0 .. 1u32`), `if claimed || true`, `RUNNER_RESERVE_ATTEMPTS` 16 -> 1, M1c (`unwrap_or(true)`), and the Fix A/B/C candidate, alone and in combination with M1a, M1b, M1c, M4, 16 -> 3 and M17. All reverted with `git checkout -- src/checks.rs`.

**Temp-directory hygiene.**

- **Directories created in `/tmp`: 0.** `find /tmp -maxdepth 1 -iname "agent-scaffold-*" | wc -l` returned **65** before this triage and **65** after; all 65 are `agent-scaffold-checks-test-*` predating this session, none are mine, none were touched. `find /tmp -maxdepth 1 -iname "agent-scaffold-checks-run-*" | wc -l` returns **0**.
- `TMPDIR` was exported to `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/triage-tmp` for every build, test run, probe and binary A/B.
- Probes were bounded: 100 single-test processes for the M4 rate, 100 more for the message capture, 10 full suite runs for determinism. No exhaustion-path or high-volume reservation probe was run.
- Fixtures created and removed inside the scratch `TMPDIR`: one scratch git repo, one `chmod 555` directory (restored to 755 before deletion), one regular-file parent, one saved patch, one saved source copy, one probe-message file. **The scratch `TMPDIR` is empty at the time of writing** (`ls -A` returns nothing).
