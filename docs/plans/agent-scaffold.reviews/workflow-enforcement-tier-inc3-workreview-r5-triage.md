# `workflow-enforcement-tier-inc3` work review, ROUND 5, TRIAGE

Triaged in worktree `.claude/worktrees/triage-inc3-r5` on branch `triage/inc3-r5` at `a51d62d`, the tip of the branch under review with both round 5 reviewers' findings merged in. `git diff 3f19bd1..a51d62d --stat` touches only the two round 5 review files, so the product I triage is byte-identical to the product both reviewers measured.

Two findings files: `...-r5-reviewer-adversarial.md` (ZERO findings) and `...-r5-reviewer-mutation.md` (TWO findings, `R5M-1` and `R5M-2`, both `low`).

All four prior triage files were read in full before any ruling, along with the increment's specification and the ledger's decision record for `Q-55-existsgate` and `Q-55-emptyroot`. The do-not-relitigate list is treated as settled and nothing below raises or reopens an item on it.

## Method

TOOLCHAIN, confirmed before every build-dependent claim, with no `2>/dev/null` on the export:

```
$ cd <worktree> && direnv allow && eval "$(direnv export bash)" && which cargo && cargo --version
/nix/store/76jaab43a2l7n7fiifxjngp68kk167vm-rust-mixed/bin/cargo
cargo 1.98.0-nightly (a335d47ff 2026-06-26)
```

BASELINE, measured before the first mutation and again after the last: `cargo test --no-fail-fast` 422 passed / 0 failed across nine binaries; `cargo clippy --all-targets -- -D warnings` clean. `TMPDIR` pointed at `<scratch>/tri-r5/tmpdir`, outside any git repository (`git rev-parse --show-toplevel` there reports `not a repository`, checked). `<scratch>` abbreviates the session scratchpad directory; every fixture lives under `<scratch>/tri-r5/`, a directory of my own naming.

TWO BINARIES, both built by me from this worktree: `shipped` (`a51d62d` as it stands) and `declined` (the same tree with mutation `P1` applied, which is the `exists()` design `Q-55-existsgate` declined). Both were reverted out of the tree immediately after the build; their checksums differ and `target/debug/agent-scaffold` was confirmed byte-identical to `shipped` afterwards.

MY UID IS 1000, so the `if opaque` block in `a_round_log_that_cannot_be_checked_is_not_reported_as_missing` DOES execute on my machine, which is what makes the root comparison below a real comparison rather than a simulation. Where the finding turns on uid I ran the same command twice, once as uid 1000 and once under `unshare -Ur` as namespace root. `unshare -Ur` gives uid 0 with `CapEff: 000001ffffffffff`, all capabilities including `CAP_DAC_OVERRIDE` and `CAP_DAC_READ_SEARCH`, confirmed by reading `/proc/self/status` inside the namespace.

No `nix fmt`, no `just scaffold-self`. The main repository at `/home/jessea/Documents/projects/agent-scaffold` was not touched and no other worktree was touched.

---

# Part 1: REPRODUCTION OF THE UNCAUGHT MUTATIONS

Each mutation was applied with the `Edit` tool, measured with `cargo test --no-fail-fast`, and reverted with `git checkout -- .` followed by an empty `git status --porcelain` and an empty `git diff HEAD`, both printed. I reproduced the four mutations the two findings actually rest on, plus the two controls that give them their meaning.

| id | mutation | reviewer says | I measure | verdict |
| --- | --- | --- | --- | --- |
| `M1b` | `Err` message: `metrics_path.display()` replaced by the literal `"SOME/WRONG/LITERAL/PATH"` | NOT CAUGHT | 422 passed, 0 failed | REPRODUCES |
| `M9` | `Err` message: the `({error})` errno dropped (`Err(_)` arm, errno removed from the format string) | NOT CAUGHT | 422 passed, 0 failed | REPRODUCES |
| `M2` | `Err` message: `; pass a --metrics naming this project's log` deleted | NOT CAUGHT | 422 passed, 0 failed | REPRODUCES |
| `M3` | `Ok` message: BOTH remedy clauses deleted | NOT CAUGHT | 422 passed, 0 failed | REPRODUCES |
| `P1` | CONTROL: `try_exists()` -> `Ok(metrics_path.exists())`, the declined design | CAUGHT, exactly 1 test | 421 passed, 1 failed; the single failure is `a_round_log_that_cannot_be_checked_is_not_reported_as_missing` | REPRODUCES |
| `T1` + `P1` | `opaque` forced false AND the declined design applied | NOT CAUGHT | 422 passed, 0 failed | REPRODUCES |

All six reproduce exactly as reported. The reviewer's mutation table is accurate on every claim I tested, in both directions: the ones it calls caught are caught, and the ones it calls uncaught are uncaught.

## The reproduction that is STRONGER than the one the reviewer made

`T1` simulates a root machine by editing the test (`.is_err() && false`). I did not need to simulate it. I ran the REAL thing:

```
CONTROL, unmutated tree, suite under `unshare -Ur` (uid 0, all caps):
  passed=422 failed=0

DECLINED DESIGN, source mutation P1 only, NO TEST MUTATION AT ALL,
suite under `unshare -Ur`:
  passed=422 failed=0
```

So on a machine running as root the suite reports 422 passed / 0 failed for the design `Q-55-existsgate` DECLINED, with the test file untouched and the test itself printing `ok`. `T1`'s `&& false` is not a contrivance that manufactures the gap; it is a faithful model of what uid 0 does to that block, and the gap is reachable without touching a single line of test code.

I record that the control matters: the unmutated suite is also 422/0 under `unshare -Ur`, so the green under `P1` is not an artifact of the namespace breaking something unrelated.

---

# Part 2: VERIFICATION OF THE ENOTDIR DISCOVERY

The reviewer's load-bearing claim is that making `docs/metrics` a REGULAR FILE produces `ENOTDIR` from the probe for every uid including root, because a non-directory path component is a structural error that `CAP_DAC_OVERRIDE` does not bypass. Built and run against both binaries at both uids.

FIXTURE: `<scratch>/tri-r5/enotdir/docs/plans/p.plan.toml` present and valid; `<scratch>/tri-r5/enotdir/docs/metrics` a REGULAR FILE.

```
$ agent-scaffold validate --workflow --source docs/plans/p.plan.toml

uid 1000, SHIPPED:   ... the round log at docs/metrics/workflow.jsonl could not be checked
                     (Not a directory (os error 20)): ...                          exit=1
uid 1000, DECLINED:  ... no round log at docs/metrics/workflow.jsonl: ...
                     ... or record the project's review rounds there               exit=1

uid 0 (unshare -Ur, all caps), SHIPPED:   IDENTICAL to uid 1000                    exit=1
uid 0 (unshare -Ur, all caps), DECLINED:  IDENTICAL to uid 1000                    exit=1
```

THE DISCOVERY HOLDS AT BOTH UIDS, exactly as reported, and the shipped design distinguishes itself from the declined one on that fixture at every uid.

## The other half of the premise, which I checked rather than assumed

The finding also needs the EXISTING mode-600 fixture to genuinely degenerate under root. Built and measured on the same shape the test uses (a real one-record log under a mode-600 `docs/metrics`):

```
uid 1000, SHIPPED:   ... could not be checked (Permission denied (os error 13)) ...  exit=1
uid 1000, DECLINED:  ... no round log at ... or record the project's review rounds    exit=1
                     (the two designs are distinguishable here)

uid 0,    SHIPPED:   docs/metrics/workflow.jsonl: 1 records, valid
                     ... workflow invariants hold                                     exit=0
uid 0,    DECLINED:  IDENTICAL, byte for byte                                         exit=0
                     (the two designs are INDISTINGUISHABLE here, and `opaque` is false,
                      so the test's four assertions do not run and it prints `ok`)
```

Both halves of `R5M-2`'s premise are true as measured.

## A BETTER REMEDY THAN THE ONE PROPOSED, measured at both uids against both designs

The reviewer's `ENOTDIR` fixture has one weakness it states honestly: no real log can sit behind a regular-file `docs/metrics`, so it does not reproduce the case `Q-55-emptyroot`'s falsehood argument is actually about. That weakness is removable, and the technique is ALREADY IN THIS REPOSITORY (Part 5).

A TRAILING SLASH on a path naming a REGULAR FILE makes `stat` return `ENOTDIR` for every uid, because a trailing slash demands that the final component be a directory, and that is a structural error rather than a permission check. Unlike the regular-file fixture, the file itself is a REAL, VALID, READABLE round log.

```
FIXTURE: docs/plans/p.plan.toml present; docs/metrics/workflow.jsonl a real one-record log.

CONTROL (no slash), SHIPPED, uid 1000 and uid 0:
  docs/metrics/workflow.jsonl: 1 records, valid ... workflow invariants hold      exit=0

$ ... --metrics docs/metrics/workflow.jsonl/          # one character added

uid 1000, SHIPPED:   ... the round log at docs/metrics/workflow.jsonl/ could not be
                     checked (Not a directory (os error 20)): ...                 exit=1
uid 1000, DECLINED:  ... no round log at docs/metrics/workflow.jsonl/: ...
                     ... or record the project's review rounds there              exit=1
uid 0,    SHIPPED:   IDENTICAL to uid 1000                                        exit=1
uid 0,    DECLINED:  IDENTICAL to uid 1000                                        exit=1
```

I then built the remedy as a test addition and measured it in every direction that matters. Three assertions, one extra `validate` run, NO new fixture files, NO source change:

| tree | uid 1000 | uid 0 |
| --- | --- | --- |
| remedy + SHIPPED | GREEN | GREEN |
| remedy + `P1` (declined design) | RED | RED |
| remedy + `M1b` (wrong path literal) | RED | not needed, uid-independent |
| remedy + `M9` (errno dropped) | RED | not needed, uid-independent |

ONE ADDITION CLOSES BOTH FINDINGS AT EVERY UID. That measurement is what makes the remedy question settled rather than a proposal, and it is why neither finding can be defended on cost.

I also measured the reviewer's own regular-file variant as written (green on shipped at both uids, red on declined at both uids, red on `M1b`, red on `M9`), so its claim stands too. The trailing-slash form is simply cheaper and covers the real-log case as well.

---

# Part 3: `R5M-1`

## Verdict: VALID. Severity: `low` (the reviewer's rating, which I agree with).

**What is established.** `M1b`, `M9` and `M2` each leave the suite at 422/0, measured by me. The three assertions guarding the `Err` case are `!stderr.contains("no round log at")`, `stderr.contains("could not be checked")`, and `!stderr.contains("record the project's review rounds")`. Two are negative and the positive one is a four-word phrase. The suite therefore pins WHICH of the two sentences fires and nothing about WHAT it says. The `Ok` branch's path is pinned and `M1` proves that assertion bites; the `Err` branch's path is pinned zero times.

**Why it is a defect in THIS increment and not a general wish.** Two reasons, and the second is an interaction only the triage step can see.

FIRST, THE HOUSE CONVENTION ON THIS EXACT SURFACE IS TO PIN THE PATH BY INTERPOLATION, and this arm is the exception rather than the rule. `tests/unsafe_pairings_are_refused_and_omitted.rs` asserts the analogous note three times with the resolved path interpolated into the expected string (`:973`, `:1069`, `:1223`, all of the form `stderr.contains(&format!("note: --source {source} could not be checked"))`), and the `Ok` branch of this very arm does the same at run (c) of `workflow_with_no_metrics_log_hard_errors_instead_of_skipping`. This is not a new class of guard, which was ground 2 of round 1's `SC-3` dismissal; it is a missing instance of an existing one, which is the opposite.

SECOND, AND DECISIVE FOR VALIDITY: THE PROPERTY LEFT UNPINNED IS THE ONE ROUND 4'S RULING RESTS ON. Round 4 dismissed `R4A-1` because the arm's distinguishability claim is true under the reader-level reading, and its reasoning was explicitly that "THE READER-LEVEL CLAIM IS TRUE, AND NAMING THE PATH IS WHAT MAKES IT TRUE" (r4 triage, Part 1). The residual that triage recorded says the same: "the distinction the arm's comment relies on is drawn by the reader from the named path". So the correctness argument that saved a sentence in round 4 depends entirely on the printed path being the resolved one, and on the `Err` branch nothing in the suite holds that property in place. `M1b` replaces it with a fixed wrong literal and the suite stays green. A message that names a path the tool never probed is not an unhelpful message; on this branch, where a real log may be sitting behind the error, it is a misleading one, and it would ship green.

This is the MIRROR IMAGE of round 1's `SC-3` ground 3 rather than a repeat of it. `SC-3` was dismissed because its remedy would have frozen a claim that a valid finding in the same round proved false. Here the claim is not in dispute: four rounds have now argued the wording of these two messages to a settled state (`R3B-1`, `R4A-1`, `Q-55-emptyroot`), the shipped text is the settled text, and the assertion would pin a property the project has already ruled load-bearing. The contingency `SC-3` was held open for ("if `T-2` is fixed and the wording is settled, a guard on the settled sentence becomes a coherent proposal, and it should be raised then") has ARRIVED, and this is that round.

**Severity: `low`, and I decline to raise it.** The shipped message is correct on every input measured by four reviewer sets and by me. No behaviour, no exit code and no user-facing surface is wrong, and the gap alone produces no false green: on a non-root machine the branch IS exercised, just not its contents. Calibrated against this increment's own scale, that is `low`, the band round 4 assigned to a comment-only issue, and below the `medium` band round 1 used for a message that actually stated a falsehood to a user.

**The smallest remedy.** Subsumed entirely by `R5M-2`'s. The trailing-slash run in Part 2 asserts `round log at {with_slash} could not be checked (` in one `format!`, which kills `M1b` (the path) and `M9` (a parenthesised errno is present) together, measured red on both. `M2` and `M3`, the remedy clauses, I do NOT uphold as part of this: the reviewer declines to press them, round 4 recorded the `Ok` clauses as live one per population without asking for an assertion, and pinning a remedy sentence is a judgement about prose rather than about the property the arm exists to carry. One `format!` in one existing test is the whole fix.

---

# Part 4: `R5M-2`

## Verdict: VALID. Severity: `medium`, RAISED from the reviewer's `low`, for the reason stated below.

**What is established, by me, at the strongest available level.** Under `unshare -Ur` as namespace root with all capabilities, with the source reverted to the design `Q-55-existsgate` declined and WITH NO TEST MUTATION WHATSOEVER, `cargo test --no-fail-fast` reports 422 passed / 0 failed. The unmutated control under the same namespace is also 422/0. `P1` on my own uid is caught by exactly one test, and every assertion in that test that does the catching sits inside `if opaque`. As root, `opaque` is false, the block is skipped, the test prints `ok`, and the shipped design is indistinguishable from the declined one.

**Why the population is real rather than theoretical.** This repository has NO CI configuration at all (no `.github`, no `.gitlab-ci.yml`, no `.circleci`, checked), so the justfile is the gate and the suite runs on whatever machine a developer or an agent happens to be on. Nothing in `justfile`, `AGENTS.md` or `README.md` states a uid requirement for the suite (grepped, nothing). And this is a project whose own workflow is executed by agents, in harnesses that commonly run as container root. The class of machines on which the suite silently stops testing this branch is not an edge case here; it is a plausible default.

## MY EXPLICIT ANSWER TO THE SHARPEST QUESTION

**Is a suite that cannot distinguish a human-decided design from the declined alternative materially different from an ordinary coverage gap? YES, and by a wide margin.**

An ordinary coverage gap is a branch nobody wrote a test for. It is visible: you grep, you find nothing, you know where you stand. What is here is different in kind on three axes.

FIRST, IT IS A FALSE POSITIVE REPORT AND NOT AN ABSENCE. A test exists, it is NAMED for the branch, its doc comment asserts a red-then-green demonstration ("RED before this commit: the prior build printed `no round log at ...` at exit 1 for the log the closing control then reads clean at exit 0"), and it prints `ok` on machines where it demonstrates nothing at all. A reader auditing coverage finds a passing test with the right name and stops. That is worse than a gap, because a gap can be found and this cannot: the failure is invisible on exactly the machine where it occurs.

SECOND, IT IS THE INCREMENT'S OWN DEFECT CLASS, TURNED ON ITSELF. The entire content of `workflow-enforcement-tier-inc3` is the rule that a check which did not run must not report success, written into the source comment as "a skip that reports success is read by a CI gate as a pass over a project with no enforcement at all". The suite does to the developer precisely what the increment forbids the tool to do to the operator: it skips a check and reports `ok`. The specification's own words for this are "a check that passes before the change pins nothing", and `Q-66` requires evidence proportional to the claim. The claim here is that a HUMAN chose one design over another; the evidence is a block gated on the running uid.

THIRD, WHAT IS AT RISK IS NOT AN IMPLEMENTATION DETAIL BUT THE DECISION ITSELF. `Q-55-existsgate` was a human decision with a recorded receipt, taken over a specific named alternative, for recorded reasons. The ledger records that the declined `try_exists()?` change was "NOT rejected on merit" and was declined only because it widened a surface this increment promised not to touch; it is therefore exactly the kind of change a later reader might reintroduce in good faith, believing it equivalent. In this project's own vocabulary, a decision is protected by its pin: the ledger's own standard, set for `Q-55-emptyroot`, is that "both halves are pinned by separate tests: reverting the containment line reds one, reverting the human-line note reds the other". Reverting `Q-55-existsgate`'s design reds ONE test ON SOME MACHINES AND NOTHING ON OTHERS. That is a weaker pin than the project has already required of itself for a decision of the same kind.

I have weighed the honest counter-argument, which is that the shipped behaviour is correct and every reviewer across five rounds agrees. It is correct, and it is why this is `medium` and not `high`. But "the code is right today" is not the property a decision pin exists to protect. The pin protects the decision against the NEXT edit, and on a root machine there is no pin.

## Why the reviewer's own mitigation does not survive measurement, and turns into an aggravator

The reviewer rated this `low` on one mitigation, stated fairly and explicitly: the `if opaque` pattern is a pre-existing house convention introduced at `tests/unsafe_pairings_are_refused_and_omitted.rs:958-967`, so this increment followed the established pattern rather than inventing a weaker one. I checked that and IT IS ONLY HALF THE CONVENTION. See Part 5: the same commit that introduced the `if opaque` test ALSO introduced an UNCONDITIONAL, uid-independent technique for the identical `Err` class, twice, with a comment saying in terms that it exists so the fixture cannot pass vacuously. The increment took the code precedent from that commit and left the test precedent behind. Its position is therefore not "followed the house convention" but "used the half of the convention the project had already improved on, at the same surface, for the same reason, when the better half cost one character".

That is the single fact that moves my severity. Remove it and `low` would be defensible; with it, the gap is not an inherited limitation but an available and rejected improvement, on the only guard standing over a human decision.

**Severity: `medium`.** Above `low` because the consequence class is a silent false green over a decided design, on a machine population this project actually runs on, and because the cheaper unconditional alternative was already in the tree. Below `high` because no shipped behaviour is wrong, no user is affected, and the remedy is one argument in one existing test. `medium` is below the backstop severity of `high`, so NO BACKSTOP RE-CHECK IS TRIGGERED.

**The smallest remedy, measured rather than proposed.** Add to `a_round_log_that_cannot_be_checked_is_not_reported_as_missing`, AFTER its existing control and OUTSIDE any `if`, one `validate` run with `--metrics <the same real log>/` and three assertions (exit 1, `round log at {with_slash} could not be checked (`, and not `no round log at`). Measured GREEN on shipped at uid 1000 and uid 0, RED on the declined design at uid 1000 and uid 0, RED on `M1b`, RED on `M9`. No new fixture file, no source change, and it composes `R5M-1`'s missing assertion into the same three lines. The existing mode-600 fixture STAYS: it is the only one where the operator's OWN log sits behind a PERMISSION error, and the trailing-slash floor goes beneath it rather than replacing it.

The remedy is NOT `Q-55-existsgate`'s declined `try_exists()?` gate change and does not touch `src/` at all. The reviewer says so explicitly and I confirm it: everything above is a test addition.

I record the reviewer's cheap alternative (an `else` on the existing `if opaque` printing a warning that the discriminator did not engage) and do NOT recommend it. It makes the degeneration loud, which is better than silent, but it leaves the decided design unpinned on the machines where the pin is missing, and it costs about as much as the fix that actually closes it.

---

# Part 5: WHAT THE REVIEWERS MISSED

Three, and the first is the one that changed a severity.

1. **THIS REPOSITORY ALREADY OWNS AN UNCONDITIONAL, UID-INDEPENDENT TECHNIQUE FOR THE `Err` CLASS, AND IT ARRIVED IN THE VERY COMMIT `Q-55-existsgate` CITES AS ITS PRECEDENT.** The mutation reviewer cites `tests/unsafe_pairings_are_refused_and_omitted.rs:958-967` as the house convention and reads it as a mitigation. Two tests further down THE SAME FILE, `an_uncheckable_plan_anchor_does_not_remove_the_other_anchors_root` (`:1069`) and `an_uncheckable_source_anchor_does_not_remove_the_other_anchors_root` (`:1223`), assert `could not be checked` with NO `if` guard of any kind, reaching the `Err` class by putting a TRAILING SLASH on a path that names a regular file. Only two `opaque` bindings exist in the whole test tree (`grep`ed): the one at `:962` and this increment's at `:305`. Everything else on that surface is unconditional.

   The file says why, in the doc comment above `:1041`: "the `could not be checked` assertion below is what keeps the fixture from passing vacuously if the anchor ever stopped landing in that class." That is `R5M-2`'s exact concern, written into this repository, before this increment, by the author of the pattern.

   PROVENANCE, checked rather than assumed. `git log -S "uncheckable_plan = format!"` puts those tests in `36e19f0`, "fix: classify an anchor that cannot be checked as not on disk", and `git merge-base --is-ancestor` confirms `36e19f0` predates this increment; `git diff main...HEAD` does not touch that file at all. `36e19f0` is the commit `Q-55-existsgate`'s recorded reasoning names when it says the change "matches the `try_exists` precedent this same file set one increment earlier". So the decision took the CODE precedent from `36e19f0` and the increment left the TEST precedent sitting beside it, unused.

   This matters three ways: it converts the reviewer's mitigation into an aggravator; it makes the remedy one character rather than one fixture; and it removes any argument that the fix would be a new class of guard, which was ground 2 of round 1's `SC-3` dismissal.

2. **THE TRAILING-SLASH FIXTURE COVERS THE CASE THE REVIEWER SAID ITS REMEDY COULD NOT.** The reviewer keeps the mode-600 fixture on the ground that "it is the only one where a REAL LOG sits behind the error, which is the case `Q-55-emptyroot`'s falsehood argument is actually about, and `ENOTDIR` does not reproduce that". True of a regular-file `docs/metrics`, false of a trailing slash: measured in Part 2, the same fixture reads `1 records, valid ... workflow invariants hold` at exit 0 without the slash and reports `could not be checked (Not a directory (os error 20))` with it, at BOTH uids. So a real, valid, readable log DOES sit behind the error, at every uid. I still recommend keeping the mode-600 fixture, but for a narrower reason than the reviewer gives: it is the only one where the error is a PERMISSION error, which is the `EACCES` class round 1's `T-1` established as the one reachable class with a real log behind it. The claim that no uid-independent fixture can put a real log behind the error is not correct.

3. **THE ADVERSARIAL REVIEWER'S ZERO IS SOUND ON ITS OWN TERMS, AND ITS BLIND SPOT IS STRUCTURAL RATHER THAN A MISS.** I spot-checked its two load-bearing structural claims and both hold: exactly one `println!` exists inside `run_validate` (`src/main.rs:1083`) and it is gated on `problems.is_empty()`, so a run with any problem has an empty stdout; and `--workflow` carries no `requires` on `ValidateArgs` while `workflow_spec` does. Its coverage-gaps section is honest and complete about what it did not do. What it could not see is that its whole method, running inputs against a built binary and comparing, cannot detect a suite that fails to test a correct binary. Its zero and the mutation reviewer's two are not in tension; they are answers to different questions, and this round is a clean demonstration of why the second question is worth a lens of its own.

I conducted no independent adversarial pass of my own, which is not this role's job. Everything above came from testing the two reports' claims.

---

# Tally

VALID FINDINGS: 2.

| Severity | Count | Findings |
| --- | --- | --- |
| critical | 0 | |
| high | 0 | |
| medium | 1 | `R5M-2` |
| low | 1 | `R5M-1` |

RE-RATING I MADE: `R5M-2` from `low` to `medium`, on the ground stated in Part 4 (the reviewer's sole mitigation for keeping it at `low` does not survive Part 5's measurement).

SEVERITY CEILING: `medium`. No `high` and no `critical` was raised by either lens and I found none. `medium` is below the backstop severity of `high`, so NO BACKSTOP RE-CHECK IS TRIGGERED.

REMEDY SHAPE, since this project measures it: BOTH findings are closed by ONE addition to ONE existing test, about six lines, adding one `validate` run and three assertions, with NO source change and NO new fixture file. I measured it green on the shipped design at uid 1000 and uid 0, and red on the declined design at uid 1000 and uid 0. Nothing in either finding asks for `Q-55-existsgate`'s declined `try_exists()?` gate change, and nothing asks for a line of `src/` to move.

FOR THE ORCHESTRATOR, stated plainly because the arithmetic is close and must not be inferred from prose: ROUND 5 HAS TWO VALID FINDINGS AND IS NOT CLEAN. I record that I was told the full convergence arithmetic before ruling, including that any valid finding here takes the loop to the cap and escalates it to a human. I record also that I looked hard for the dismissal, and that the two grounds most likely to supply it, round 1's `SC-3` precedent and the reviewer's own house-convention mitigation, BOTH TURNED OUT TO POINT THE OTHER WAY when I checked them rather than accepted them: `SC-3`'s three grounds are absent or inverted here, and the house convention contains the unconditional technique the increment did not use. Had either held I would have dismissed on it and the round would have been clean.

WHAT I WOULD PUT TO THE HUMAN, since escalation is where this lands. The product question is small and the remedy is measured and cheap; the question worth a human is the one in Part 4, whether a decided design may ship with a pin that holds only on some machines, because that is a standard for this project and not a fact about this arm.

# Relitigation and constraints check

Nothing above raises or reopens the four standing residuals (the in-root bound, the single-anchor `..` case, the earlier increment's rejected-ledger context slot, the off-convention `--source` surface); accepted costs (i) to (iv), which appear only as pinned expected behaviour and as catching tests; round 1's `ADV-4` or `SC-3`, which is CITED AS A PRECEDENT AND DISTINGUISHED, not reopened, and whose own text holds the contingency open for exactly this case; round 2's `R2A-4`, `R2B-2` or `R2B-3`; round 3's `R3A-1` or `R3A-3`; round 4's `R4A-1`, whose RULING I rely on as settled and do not disturb; the pre-existing plain-`validate` inconsistency; the pre-existing containment TOCTOU; or the check-16 vacuous pass, which is recorded and scheduled and which I neither raise nor use as an argument. Neither upheld finding proposes `Q-55-existsgate`'s declined `try_exists()?` gate change; both remedies are test-side and touch no source. No line-length, prose-wrapping or comment-raggedness observation appears anywhere in this file.

# TREE STATE: NO SOURCE CHANGE REMAINS

Every mutation applied during this triage was reverted with `git checkout -- .` in the same command that ran its suite, and the empty status was printed each time. The two binaries built for the differential were built, copied out to the scratchpad, and the tree reverted immediately; `target/debug/agent-scaffold` was confirmed byte-identical (`md5sum`) to the shipped build afterwards. The candidate remedies I measured were measured and reverted, twice.

Final state, measured after the last mutation and before this file was written:

```
$ cargo test --no-fail-fast      # TMPDIR outside any git repository
passed=422 failed=0

$ cargo clippy --all-targets -- -D warnings
clean

$ git status --porcelain
(empty)
$ git diff HEAD --stat
(empty)
$ git rev-parse HEAD
a51d62d82d90a22c034b44cfcbeb3287cc9bc8d2
```

The tree carries NO source changes, NO test changes and NO prose changes. The only file this triage authors is this one.

FIXTURE HYGIENE: every fixture lives under `<scratch>/tri-r5/`, a directory of my own naming. Nothing outside it was written or deleted and nothing was written into bare `/tmp`. The one restrictive fixture I created (`<scratch>/tri-r5/mode600/docs/metrics` at mode 600) was chmodded back to 0755 in the same command that used it, in both the uid-1000 and the namespace-root runs. The closing sweep for restrictive directories, mode-000 files and FIFOs under my subdirectory returns nothing, and the three fixture `docs/metrics` entries are at 755, 644 and 755.
