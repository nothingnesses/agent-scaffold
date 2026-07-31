# Verification lens, region-scoped: commit `596548b`

`test(checks): pin claim_dir's own error arm and delete two false claim-outcome sentences`, on `review/checks-collision-ver-a`, worktree `.claude/worktrees/ver93-a`, parent `bc9908e`. Diff is `src/checks.rs` only, 19 insertions, 9 deletions.

This round is a check on the human-authorised REDUCED fix set (Fix 1 for `RG1`/`MU1`, Fix 2 for `RG2`), not the opening round of a fresh convergence loop. The question answered is narrow: did those two fixes land exactly as authorised, and did they re-seed anything.

**VERDICT: ZERO findings from this lens.**

---

## Coverage statement

True region bounds established by reading outward from the two fix sites until the enclosing item closed, rather than by taking the assignment's named symbols as the bounds. Every line below was read; the CORE ranges are the ones the fix touches or that make a claim about the code the fix touches.

| Range | What it is | Why in region |
| --- | --- | --- |
| `src/checks.rs:33-56` | Module invariant list, Invariant B in full | Invariant B makes claims about the reservation and the prune; `:49-56` is the `RG4` accepted residual, checked for presence, deliberately not raised |
| `:91-98` | `RUNNER_PREFIX` doc and const | Names the fixture-prefix separation the fix's leak argument depends on |
| `:423-429` | `NEXT_RUNNER_SEQ` doc and const | Layer 1 of the uniqueness argument the region documents |
| `:431-436` | `RUNNER_RESERVE_ATTEMPTS` doc and const | Bound named by the region's tests and by the exhaustion error |
| **`:438-454`** | **`claim_dir` doc comment and body** | **CORE. Fix 1's subject; the mutated arm is `:452`** |
| **`:456-500`** | **`reserve_runner_worktree` doc comment and body** | **CORE. `:482-485` is the comment Fix 1's severity argument rests on** |
| **`:502-553`** | **`reserve_runner_worktree_with` doc comment and body** | **CORE. Deletion site 1 is `:502-505`** |
| `:555-563` | `owning_pid` doc and body | Read by the prune from the name the region builds |
| `:565-588` | `prune_orphan_worktrees` doc comment | Bounds the reclamation the region's invariant text describes |
| `:1037-1046` | `scratch` test helper | Load-bearing for ruling on the commit's "adds no new fixture-name class that would leak" claim |
| `:1610-1617` | `dead_pid` and its doc | Neighbouring reservation-test infrastructure |
| `:1619-1650` | `a_startup_prune_reclaims_an_orphaned_runner_worktree` | Neighbouring reservation test; calls the real `claim_dir` through `reserve_runner_worktree` |
| `:1652-1683` | `a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one` | Same |
| **`:1685-1718`** | **`a_directory_claim_is_exclusive`, whole body** | **CORE. Fix 1 lands at `:1704-1716`** |
| **`:1720-1741`** | **`a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one`** | **CORE. Deletion site 2 is `:1722-1724`** |
| `:1743-1774` | `a_claim_that_never_wins_fails_at_the_attempt_bound` | Neighbouring reservation test |
| `:1776-1803` | `a_claim_error_that_is_not_a_collision_propagates_at_once_and_names_the_path` | Neighbouring reservation test; the round-3 `AD1a` fix Fix 1 sits one level below |
| `:1805-1868` | `concurrent_reservations_never_share_a_runner_worktree_path` | Neighbouring reservation test; exercises the real `claim_dir` 2000 times |
| `:1870-1891` | `a_reserved_path_still_carries_its_owning_pid_as_the_first_component` | Last test in `mod tests` (`mod` closes `:1892`); reservation test |

Sweep method, applied to the CORE ranges sentence by sentence and to the rest paragraph by paragraph: for every SENTENCE, is it true of the code as it now is; for every ASSERTION, what does it actually pin and what could change without it noticing. The four clauses this lens's predecessor was faulted for skipping are handled explicitly: both surviving sentences at the deletion sites are decomposed clause by clause below, and the assertions immediately above and below the new block are ruled on individually.

---

## Findings

None. Positive checks, each with its evidence, are recorded below.

---

## What was checked, and how

### C1. Both fixes landed byte-exact against the authorised diff

The authorised text is in `docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage-r4.md`: Fix 1 at `:309-322`, Fix 2 site 1 at `:337-347`, Fix 2 site 2 at `:351-357`. Compared against the landed file:

- Fix 1, `src/checks.rs:1704-1716`: identical to the authorised 13-line block, character for character, including the four comment lines, the `expect_err` message, and the `assert_ne!` arguments.
- Fix 2 site 1, `src/checks.rs:502-505`: identical to the authorised 4-line replacement.
- Fix 2 site 2, `src/checks.rs:1722-1724`: identical to the authorised 3-line replacement.

Independent arithmetic corroboration that the commit is fixes 1 and 2 and NOTHING else. The authorised three-fix set measured `23 insertions(+), 12 deletions(-)` (`triage-r4.md:296`). The dropped Fix 3 for `RG4` measured `4 insertions, 3 deletions` (`triage-r4.md:363-372`). `23 - 4 = 19` and `12 - 3 = 9`, which is exactly `git diff bc9908e..596548b --stat`:

```
 src/checks.rs | 28 +++++++++++++++++++---------
 1 file changed, 19 insertions(+), 9 deletions(-)
```

Zero production lines changed: the whole diff is the doc comment at `:502-505`, the test comment at `:1722-1724`, and test body at `:1704-1716`. No line inside `claim_dir`, `reserve_runner_worktree`, or `reserve_runner_worktree_with`'s body is touched.

### C2. Fix 1 kills both authorised mutations. MEASURED, both RED.

Baseline on the tree as committed, `cargo test` with `TMPDIR` exported to the scratchpad:

```
TOTAL passed=386 failed=0
```

clippy, `cargo clippy --all-targets`: `Finished dev profile`, zero warnings and zero errors emitted.

**MUT-A**, `src/checks.rs:452` `Err(error) => Err(error)` becomes `Err(_) => Ok(false)`. Whole suite `TOTAL passed=372 failed=1`. Observed output:

```
test checks::tests::a_directory_claim_is_exclusive ... FAILED

---- checks::tests::a_directory_claim_is_exclusive stdout ----

thread 'checks::tests::a_directory_claim_is_exclusive' (2274305) panicked at src/checks.rs:1711:14:
a claim that cannot be made is an error, not a verdict: false
```

**MUT-B**, the severity-carrying one, `:452` becomes `Err(_) => Ok(true)`. Whole suite `TOTAL passed=372 failed=1`. Observed output:

```
thread 'checks::tests::a_directory_claim_is_exclusive' (2279075) panicked at src/checks.rs:1711:14:
a claim that cannot be made is an error, not a verdict: true
```

Both RED, at the exact counts and the exact messages the triager recorded. `RG1`/`MU1` is closed: the arm at `:452` is no longer executable-but-unpinned, and under `MUT-B` the reservation can no longer return a path whose claim was never established, which is what `:482-485` asserts makes the returned path exclusively ours.

The `372 passed` under both mutations, against a `386` baseline, is cargo declining to run the 13 integration-binary tests after the unit binary fails, not 13 tests silently disappearing. Confirmed by C5 below, where `--no-fail-fast` yields `385 + 1 = 386`.

### C3. The `assert_ne!` is not dead weight. MEASURED.

`expect_err` alone kills both `MUT-A` and `MUT-B`, since both return `Ok`. To establish that the second assertion pins something of its own rather than riding along, a third mutation was built: `:452` becomes `Err(_) => Err(io::Error::from(io::ErrorKind::AlreadyExists))`, which models `claim_dir` misclassifying a real fault as a lost claim, exactly the failure the assertion names.

**MUT-C** result: `expect_err` is satisfied, and the `assert_ne!` catches it alone.

```
test checks::tests::a_directory_claim_is_exclusive ... FAILED
thread 'checks::tests::a_directory_claim_is_exclusive' (2285235) panicked at src/checks.rs:1712:9:
assertion `left != right` failed: a real error must stay distinguishable from a lost claim: entity already exists
test result: FAILED. 372 passed; 1 failed
```

Exactly one test failed, so `a_directory_claim_is_exclusive` is the only thing in the suite that would notice. `X7` (assert_ne against AlreadyExists rather than an errno equality) is settled and is not reopened here; this measurement only establishes the assertion is load-bearing rather than decorative.

### C4. Fix 2 is a pure deletion at both sites with zero words authored. MEASURED mechanically.

`git diff --word-diff=plain bc9908e..596548b -- src/checks.rs`, restricted to the two prose sites. The complete set of `{+...+}` insertions is:

```
{+injected.+}   {+///+}  {+///+}  {+///+}      (site 1)
{+//+}                                          (site 2)
```

Site 1's only insertion is the token `injected.`, which is the surviving word `injected` with the comma after it replaced by a full stop; the three `///` are re-wrap artefacts of the surviving words moving up a line. Site 2's only insertion is a `//` marker. The word `The` at site 2 is shown as surviving text, not as an insertion: the original read `The filesystem will not lose a claim on demand (...), so the claim is injected`, and the deletion takes `filesystem ... so the` out from between `The` and `claim is injected`. So the remainder is built entirely from words already present. No replacement wording was authored, and the past-tense rewrite the implementer was forbidden from writing is absent.

### C5. Both false sites are gone and no third instance survives. MEASURED.

Case-insensitive over the whole repo excluding `.git` for `every real claim`, `claim in this repository`, `claims in this repository`: zero hits in `src/`, `tests/`, `AGENTS.md`, `README.md`. The only hits are in `docs/plans/agent-scaffold.reviews/*` and `docs/plans/agent-scaffold.ledger.md`, which are the historical review record quoting the deleted text, and are correct to retain.

Checked separately for a surviving paraphrase, since a third instance need not reuse the exact words. Over `src/ tests/ AGENTS.md README.md` for each of `one path at a time`, `take theirs sequentially`, `will not lose a claim`, `not produce on demand`, `only way to drive`, `sequentially`, `real claim`: zero hits for every one of the seven. Nothing in live source or documentation still asserts, in any wording, that real claims always win or that the filesystem will not produce the lost outcome on demand.

### C6. Every clause of both surviving sentences is true. CHECKED CLAUSE BY CLAUSE.

This is where the predecessor lens failed, so both surviving sentences are decomposed rather than read whole.

Site 1, `:502-505`:

- "`reserve_runner_worktree` (above) with its claim injected." TRUE; `:498-500` delegates with `claim_dir`.
- "Production passes `claim_dir`". TRUE; `:499`.
- "and is otherwise unchanged". TRUE; `reserve_runner_worktree` is a single delegating expression with no other behaviour.
- "the whole reservation, including the temp-dir creation above the loop, lives here". TRUE; `fs::create_dir_all(&temp)` is at `:517`, inside `reserve_runner_worktree_with`, and the loop starts at `:524`, so "above the loop" is literally correct.
- "so the tests drive the same code the runner does". TRUE, scoped to the reservation, which is what the clause scopes itself to. The injecting tests supply a different claim, which the same sentence has already said.
- No dangling connective. The deletion removed a trailing `, which is ...` and closed the sentence at `injected.`; nothing before or after depends on the removed text.

Site 2, `:1722-1724`:

- "Layer 2's collision handling, driven at the use site rather than only at `claim_dir`." TRUE; the test calls `reserve_runner_worktree_with`, and `a_directory_claim_is_exclusive` is the "only at `claim_dir`" case it contrasts with.
- "The claim is injected". TRUE.
- "this one records every name it is offered". TRUE; `offered.push(path.to_path_buf())` at `:1728` runs on every call before the verdict is computed.
- "and loses the first two". TRUE, and derived from the code rather than from the comment: the verdict is `Ok(offered.len() > 2)` at `:1729`, evaluated after the push, so call 1 sees `len == 1` and loses, call 2 sees `len == 2` and loses, call 3 sees `len == 3` and wins. `assert_eq!(offered.len(), 3, ...)` at `:1734` pins it.
- "which is the state the loop exists for". TRUE.
- No dangling connective. The original ran `..., so the claim is injected: ...`; the deletion removed the reason and the `so`, leaving a well-formed independent sentence. This is the sentence the implementer reported re-wrapping, and it is neither truncated nor left with a false remainder.

### C7. The four new comment lines author no claim that can go stale. CHECKED.

`:1704-1707`, line by line:

- "The THIRD outcome this documents, which neither assertion above reaches". Both halves TRUE. `claim_dir`'s doc at `:438-440` documents three outcomes explicitly, the third being "and propagates every other error", so "this documents" is accurate. The two assertions above at `:1702-1703` both take `Ok` paths and neither can reach the error arm.
- "a claim that fails for a reason OTHER than the path being taken propagates as an error rather than folding into either verdict". TRUE and MEASURED; the two foldings are `MUT-A` and `MUT-B`, both RED.
- "A regular file standing in for a parent directory produces one without needing a permissions fixture." TRUE; `file.join("under-a-file")` has the regular file `file` as its parent, and `fs::create_dir` under it yields ENOTDIR, which is neither `AlreadyExists` nor a permissions fault. Confirmed by `MUT-A`/`MUT-B` reaching the assertion at all.

None of the four lines makes a claim about the shape of the test tree or about the project's history, which is the class of prose that has re-seeded on this step. This was the commit message's own claim about the fix and it holds.

### C8. The leak claim holds. MEASURED empirically, not argued.

The commit justifies folding into the existing test rather than adding a new one on the ground that it "reuses the fixture directory the test already creates and so adds no new fixture-name class that would leak on a failing run". Measured directly by inspecting what the RED runs left behind.

`scratch` (`:1037-1046`) builds `agent-scaffold-checks-test-{pid}-{name}` and, importantly, does `let _ = fs::remove_dir_all(&dir);` at `:1043` before creating it, so a leaked fixture is reclaimed by the next run that uses the same name. The nine directories the four RED runs left in the scratchpad are all named `...-claim-dir`, the pre-existing class, and none is a new class. Contents of one from a `MUT-A`/`MUT-B` run:

```
a-regular-file
claim
```

The new fixture file sits INSIDE the directory the test already created, so it leaks only in company with a leak that already existed, and is removed by the same `fs::remove_dir_all(&dir)` at `:1717` on the passing path. A separate test function, as `MU1` proposed, would have introduced a second name and a second leaking class. The stated ground for preferring `RG1`'s form is confirmed.

### C9. The neighbouring coverage sentence is still true after the fix. MEASURED.

`:1689-1692` claims "no other test executes the LOST one through `claim_dir` itself ... so nothing else would notice if this stopped reporting a taken path as taken." The fix added code to this test, so the claim was re-verified rather than assumed.

**MUT-D**, `:451` `Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(false)` becomes `... => Ok(true)`, run under `cargo test --no-fail-fast` so the whole suite executes:

```
test checks::tests::a_directory_claim_is_exclusive ... FAILED
thread 'checks::tests::a_directory_claim_is_exclusive' panicked at src/checks.rs:1703:9
TOTAL passed=385 failed=1
```

`385 + 1 = 386`, so every test in the suite ran, and exactly one caught it. The sentence is TRUE of the code as it now is. This also confirms the `372` counts under `MUT-A` through `MUT-C` are a fail-fast artefact and not a coverage loss.

### C10. `RG4` is present and unfixed, as the human decided. VERIFIED, NOT RAISED.

`:53-56` still reads "Widening the prune to sweep the temp dir by prefix would close that window, at the cost of giving it authority over other repositories' runner directories (Principle 18, least authority), which is not a trade this module makes." The narrowing clause from Fix 3 is absent. This is the accepted residual and is recorded here only so the record shows it was checked for presence rather than overlooked. It is not a finding.

### C11. The rest of the region carries no false sentence

Every remaining sentence in the CORE ranges was checked against the code. Specifically confirmed TRUE: `claim_dir`'s three-outcome contract at `:438-440` against the three arms at `:449-453`; the `create_dir_all` prohibition at `:442-444`; the name format at `:461` against the `format!` at `:526`; the pid-first-component requirement at `:461-464` against `owning_pid` at `:562`; the layer-1 and layer-2 argument at `:471-485` against `NEXT_RUNNER_SEQ` at `:429` and the retry loop at `:524-544`; the create-the-leading-temp-directories consequence at `:492-495` against `:517-522`; the exhaustion error's contents at `:545-552` against what `a_claim_that_never_wins_fails_at_the_attempt_bound` asserts at `:1757-1773`.

### C12. Tree restored and green

Post-revert `cargo test`: `TOTAL passed=386 failed=0`, identical to baseline.

---

## Non-findings

Recorded because this lens returned a wrong zero once on this step, so the near-misses should be visible to the human rather than silently dropped. None of these clears the finding bar, which is a sentence that is FALSE of the code as it now is, or an assertion that pins less than it claims.

### NF1. "both outcomes" at `:446` and `:1689` now undercounts what the test does

This is the closest thing to a re-seed I found, and it does not clear the bar.

`claim_dir`'s doc at `:445-447` says "so both outcomes are pinned directly by a test (`a_directory_claim_is_exclusive`) rather than only through their caller", and the test's own header at `:1689` says "Both outcomes matter". Fifteen lines below that header, the fix's new comment at `:1704` says "The THIRD outcome this documents". So the test now pins three outcomes while two sentences in the region still enumerate two, and one of those two sentences is inside the very test that says "THIRD".

Why this is NOT a finding: neither sentence is false. At `:445-447` the antecedent of "both" is "This exclusivity", the create-or-report-taken distinction, whose outcomes are exactly two, and both of them are in fact pinned directly by that test. At `:1689` the antecedent is the immediately preceding sentence, "the FIRST claim on a path is won, and any later claim on that same path is lost", again exactly two, and "Both outcomes matter" does not assert that only those matter. Both sentences remain true statements about the pair they name; they are incomplete descriptions, not incorrect ones. The commit message relied on exactly this reading when it cited the test "whose own comment already says both outcomes matter" as the reason to fold rather than add a test, and that citation is accurate.

The predecessor lens's two misses were a flatly false clause and an unpinned arm. This is neither, and treating an undercount as equivalent would be manufacturing a finding.

Prescription if the human wants it closed anyway, **UNMEASURED** and noted as such under rule Q-66. The only form that authors zero words is a deletion at `:1689`: remove "Both outcomes matter, and " and capitalise the following "no", leaving "No other test executes the LOST one through `claim_dir` itself: ...". The two outcomes are already stated in full by the preceding sentence, so nothing is lost. `:445-447` has no clean zero-word form; every candidate either authors a word or deletes the pointer to `a_directory_claim_is_exclusive`, which is load-bearing. I did not build or measure either, because it is a comment-only change that cannot alter a test outcome and measuring it would produce a GREEN that proves nothing. I record it as UNMEASURED rather than implying it was verified.

Against this project's own history, in which 20 of 22 recorded fix-induced defects are prose and the human dropped `RG4` from the set for precisely that reason, my view is that closing NF1 would cost more than it buys.

### NF2. "a loud retry" at `:488` is loose, pre-existing, and untouched

`:487-489` reads "it turns any future mistake in layer 1 into a loud retry rather than two runs quietly sharing one worktree". A single retry emits nothing: `:524-544` sets `last_taken` and loops with no logging. Only exhaustion at `:545-552` is loud. Under a literal reading "loud" is wrong for the single-collision case and right in the limit; under the contrastive reading the sentence sets up, "loud" means visible in behaviour rather than silently wrong, which holds. Pre-existing, not in the diff, `low` at most, and the same species of pre-existing `low` prose over-claim the human just accepted as `RG4`. Not raised.

### NF3. "This is the ONLY place a `RUNNER_PREFIX` name is built" at `:457`

The `format!` that builds the name is at `:526`, inside `reserve_runner_worktree_with`, not inside the function this doc comment is attached to. Verified by grep that `:526` is the sole construction site; `:562` consumes the prefix and `:98` defines it. Round 3's introduction of the `_with` seam is what moved the construction one function down. The claim is true of the code path and `reserve_runner_worktree_with`'s own doc at `:503-505` covers the delegation explicitly ("the whole reservation ... lives here"). Not false, pre-existing, not in the diff. Not raised.

### NF4. `assert_ne!` strength

`X7` is settled and is not reopened. Noted only to record that C3 measured the assertion to be load-bearing rather than decorative, so even the settled question has no live residue here.

---

## Hygiene accounting

**Mutations reverted: YES, all four.** Four mutations were applied and reverted one at a time (`MUT-A`, `MUT-B`, `MUT-C`, `MUT-D`), each at `src/checks.rs:451` or `:452`. After the final revert:

```
$ git status --short
(no output)

$ git diff HEAD
(no output)
```

Both empty. The working tree is identical to `596548b`, and `cargo test` on the restored tree returns `386 passed, 0 failed`, matching baseline. The only file this review adds is this findings file, left uncommitted for the orchestrator.

**`/tmp` count: 0.** `TMPDIR` was exported to `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/ver-a` in every one of the fresh shells that ran cargo. `find /tmp -maxdepth 1 -newermt "2026-07-31 00:00"`, excluding `/tmp` itself and `claude-1000`, returns exactly one entry, `/tmp/RustDesk`, which is an unrelated running application and not mine. The 65 `agent-scaffold-*` directories in `/tmp` all carry mtimes of `2026-07-30 16:33` or earlier, from sessions preceding this one; `find /tmp -maxdepth 1 -name "agent-scaffold*" -mmin -180` returns 0. The five `/tmp/agent-scaffold-checks-test-*-claim-dir` directories date from `2026-07-30 16:06` to `16:16` and are likewise pre-existing. This review created nothing in `/tmp`. The nine `...-claim-dir` fixtures the RED runs leaked are inside the scratchpad `TMPDIR`, where they are expected, and are the evidence for C8.

---

## Verdict

**ZERO findings from this lens.** Fix 1 landed byte-exact and kills both authorised mutations, measured RED at the recorded messages, and its second assertion independently kills a third mutation the first would miss. Fix 2 landed byte-exact at both sites, authored zero words as proven by word-diff, and left no third instance or paraphrase anywhere in live source. `RG4` is present and unfixed as decided. No sentence in the swept region is false of the code as it now is, and the one adjacent coverage sentence the fix could have invalidated was re-measured and still holds. Nothing in this commit re-seeds.
