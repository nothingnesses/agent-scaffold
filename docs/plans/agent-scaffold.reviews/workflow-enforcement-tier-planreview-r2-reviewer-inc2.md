# `workflow-enforcement-tier` plan review, round 2, reviewer: INC2 BUILDABILITY lens

Reviewer model: Claude Opus 5, 1M-context variant. Exact model id `claude-opus-5[1m]`.
Date: 2026-07-31.
Worktree: `.claude/worktrees/rev-q55-r2-inc2`, branch `plan/q55-enforcement` at commit `8756578`.
Artifact: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, inc2 in full.

Lens: read inc2 as an implementer who must build it and has nothing but this file. I walked the increment end to end, enumerated the reason vocabulary's input cases one by one, checked the human and machine surfaces against each other, ran every acceptance check that describes current behaviour, and checked inc2's boundary with inc1 and inc3. Round 1's two upheld findings (`EX-1`, the reason carrier, and `EX-2`, the precedence and correlation rules) each received a one-clause fix at this commit, and judging whether those clauses actually closed the problem was the starting point rather than the whole job.

## Result

10 findings: 0 critical, 0 high, 5 `medium`, 5 `low`. Two of the five `medium` findings are the round-1 defects surviving their fix in a changed form; the other eight are new.

ANSWER TO "COULD A COMPETENT IMPLEMENTER NOW EXECUTE INC2 FROM THE SIDECAR ALONE": QUALIFIED NO, for a narrower reason than round 1's. The vocabulary itself is now decidable on every input I could construct except one (R2-7), which is the real gain from the fix pass. What is not closed is the CARRIER: the added clause at line 212 names `run_status` and `run_next` as the assemblers of the human message, and that is true of `run_status` and false of `run_next`, whose entire human output is `render_human`'s return value (R2-1). The triager's statement of the round-1 problem was that a bare token passes every runnable check and would ship; that is still true at this commit, and the clause added to prevent it does not reach the surface it was added for.

## What I ran, so the negative results are grounded

All runs in this worktree, `TMPDIR=/tmp/rev-r2-inc2-scratch` (outside any repository), against `target/debug/agent-scaffold` built from this commit. Fixtures rebuilt by the sidecar's own command at line 28.

- `cargo build` clean. `cargo test` gives 386 passed, 0 failed. `cargo clippy --all-targets -- -D warnings` clean. `render docs/plans/agent-scaffold.plan.toml --check` reports "up to date". Check 1's "386 expected" is current.
- The fixture reproduces: `ls "$SCRATCH/docs"` prints only `plans`.
- `docs/metrics/workflow.jsonl` now holds 240 records, not the 235 the historical passages quote and not the 239 round 1 measured. The fix pass removed both stale literals from checks 5 and 14b (round 1's `EX-8`), so nothing in the acceptance list depends on the count any more. Confirmed by `grep -c . docs/metrics/workflow.jsonl`.
- CHECK 11's RED REPRODUCES: `validate --source <fixture plan> --metrics docs/metrics/workflow.jsonl --workflow` from the repository root prints `docs/metrics/workflow.jsonl: 240 records, valid` and `workflow invariants hold` at exit 0.
- CHECK 12's RED REPRODUCES, and round 1 did not run it. A symlink at `o/docs/plans/TEMPLATE.plan.toml` pointing at project `p`'s plan (borrowed slug `triager-runs-only-on-findings` at `complete`, no log of its own), with a copy of this repository's log at `o/docs/metrics/workflow.jsonl`, run from `o`: `workflow invariants hold`, exit 0.
- CHECK 13's RED REPRODUCES, and round 1 did not run it. From inside `p`, `--metrics docs/metrics/../../../o/docs/metrics/workflow.jsonl` prints `240 records, valid` and `workflow invariants hold` at exit 0, while the control that check 13 also specifies (a `..` staying inside the root, `docs/plans/../metrics/workflow.jsonl`) gives the correct W3 red at exit 1.
- CHECK 14's PRE-FIX HALF REPRODUCES: the same foreign pairing without `--workflow` exits 0 and asserts nothing.
- CHECK 15's RED REPRODUCES: from inside a fresh fixture, `validate --source docs/plans/TEMPLATE.plan.toml --workflow` prints the two stderr notes and exits 0.
- CHECK 19's PRE-FIX BEHAVIOUR REPRODUCES, and round 1 did not run it. A layout where `docs/plans` is a symlink to `<root>/elsewhere` reads its own log and exits 0 today, which is what accepted cost (ii) says becomes a refusal after inc2.
- CHECK 14e's RED REPRODUCES: `next --json` on the unsafe pairing serialises exactly `task`, `source`, `metrics`, `active_loop`, `resume_state`; `no_active_loop_reason` is absent and neither reason field exists. `status --json` serialises exactly `plan` and `metrics`.
- CHECK 14c's RED REPRODUCES: `status --source <fixture plan> --metrics docs/metrics/workflow.jsonl` prints `metrics: 240 records` from the foreign log at exit 0.
- CHECK 14b's RED REPRODUCES ONLY WITH THE BORROWED SLUG, WHICH THE CHECK DOES NOT ASK FOR. See R2-4; both measurements are quoted there.

## The enumeration, offered instead of a coverage claim

Round 1's lesson is that naming a region is not evidence of having checked it, so this is the list itself.

REASON-VOCABULARY INPUT CASES I CONSTRUCTED, and the answer the sidecar gives each:

1. Metrics log absent at a path INSIDE the root: `log-absent`. Unambiguous.
2. Metrics log present, OUTSIDE the root: `log-not-this-project`. Unambiguous.
3. Metrics log absent AND outside the root: the new precedence rule at line 231 decides it (unsafe wins). Decidable in the text, pinned by nothing; see R2-3.
4. No `--metrics` and no plan source: the CWD-relative default stands (line 158); absent gives `log-absent`. Unambiguous.
5. Metrics path exists but is unreadable (a directory, or permissions): NOT a projection state at all. `fs::read_to_string` is behind `?` at `src/main.rs:1091` and `:1201`, so the command returns `Err` before a projection is built. No variant is owed and none is missing. Not raised.
6. Plan with an in-progress step plus an unsafe log: `no_active_loop_reason` is `metrics-not-this-project`. Unambiguous.
7. EMPTY plan plus an unsafe log: over-determined. See R2-7.
8. ALL-TERMINAL plan plus an unsafe log: over-determined. See R2-7.
9. All-terminal plan where NO step is `complete` (one `deferred`, one `skipped`): `all-steps-complete`. Reachable and measured. See R2-8.
10. No plan source at all versus a plan source with zero steps: both give `no-plan-steps`, and a consumer separates them by the `source` field, which the projection already carries (`no plan source` versus the path). Measured both. Acceptable, not raised.
11. A `--source` naming a file that does not exist: measured at exit 0 today with `source: no plan source`; the guard's input is undefined. See R2-9.
12. A pending step with unmet blockers: yields an ACTIVE LOOP (`LoopState::Blocked`), not a `no_active_loop_reason` case. The sidecar states this correctly at line 204, which is round 1's `EX-3` fix; the residual is what the fix instructs the implementer to WRITE, see R2-2.
13. Ledger absent at the anchored default: `ledger-absent`. Unambiguous.
14. Ledger present with no `## RESUME STATE` block: `no-resume-section`. Unambiguous.
15. Explicit `--ledger-fragment` outside the root, file EXISTS: `ledger-not-this-project`. Unambiguous.
16. Explicit `--ledger-fragment` outside the root, file ABSENT: the precedence rule decides it. This is the exact overlap round 1's `EX-2` sub-claim 2 raised, and it is now closed in the text.
17. Ledger unsafe AND metrics unsafe in the same run: all three fields set, no collision.
18. `status --resume` with an unsafe `--ledger-fragment`: a note and exit 0, no JSON owed (line 235 is explicit and correct).
19. `next`'s HUMAN surface with an unsafe ledger: nothing is printed at all. See R2-5.
20. The normal case on `--json`: three nulls on a real run, but NOT in the golden fixture, whose `resume_state` is `None`. See R2-10.

SECTIONS I WALKED LINE BY LINE: "One predicate, two responses" (lines 168 to 190), "The exact behaviour, per surface" (178 to 190), "The machine surface: a serialised reason" (192 to 208), "The field shape and the value vocabulary" (210 to 237), the inc2 increment description (line 274), both placement arguments (281 and 283), the cost of the placement (285), the ordering claim (289), inc2's risk classification (297), acceptance checks 11 to 14h and 19 (317 to 327, 332), the INC2 documentation-impact list (347 to 355), and the scope bullets that touch inc2 (371 to 375).

WHAT I DID NOT COVER, stated rather than implied: I did not re-verify the three exploration records against the sidecar's quotations of them, the six `Q-55*` decision receipts, the `[[step]]`/`[[question]]` TOML entries, or the two backlog sidecars; round 1 checked all four and the triager reproduced them. I did not review inc1's or inc3's internals except where inc2 depends on them. I did not check the README or `pack/AGENTS.md` wording that inc3 owns.

---

## R2-1. `medium`. The new carrier clause resolves `status` and does not reach `next`, whose entire human output is `render_human`'s return value

THE FIX UNDER TEST. Line 212, added at this commit: "THE ENUM IS THE MACHINE VALUE ONLY: the paths a human message names are not carried on it, so the CALLER assembles that message from the paths it already holds (`run_status` and `run_next`), which is what lets a unit variant serialise as a bare token while the human text still names the log and the root."

IT IS TRUE OF `run_status`. Its human print is inline at `src/main.rs:1125-1128`, inside `run_status`, with `args.metrics` in scope. That function can print `metrics: unavailable, <reason>` naming the resolved log and the derived root exactly as line 182 requires.

IT IS FALSE OF `run_next`. The whole of `next`'s human output is one statement, `src/main.rs:1239`:

```
		println!("{}", next::render_human(&projection));
```

Nothing else is printed on that path (`src/main.rs:1235-1241`). `render_human` is `pub(crate) fn render_human(projection: &NextProjection) -> String` at `src/next.rs:1017`, and `NextProjection` (`src/next.rs:99-118`) carries `task`, `source`, `metrics`, `active_loop`, `resume_state`, `no_active_loop_reason` and no path or root of any kind. The two lines the sidecar requires to name paths are produced inside that function and nowhere else:

- the metrics line, `src/next.rs:1021-1024`, which line 184 requires to become `metrics: unavailable, <reason>` "as for `status`";
- the `no active review loop ({reason})` line, `src/next.rs:1032`, which line 184 requires to carry "a reason naming the unsafe pairing" and line 223 requires to be "Printed with a reason naming the resolved log and the derived root".

So the clause names `run_next` as the assembler of two lines `run_next` does not assemble. The implementer is left to choose an unstated mechanism (change `render_human`'s signature; add a `#[serde(skip)]` detail field to the projection; have `run_next` suppress and replace part of `render_human`'s output), and the cheapest option remains the one the round-1 triager identified as the shipping outcome: print the bare token. That option still passes every runnable check, because check 14b (line 321) asks only for "a reason naming the unsafe pairing", check 14e (line 324) asserts the bare token deliberately, and check 14h (line 327) pins the JSON. The user-visible result would be `next` printing `no active review loop (metrics-not-this-project)` and a metrics line naming nothing, while `status` names both paths for the same input, on the surface `Q-55-jsonreason`'s own reasoning says agents consume.

WHAT SHOULD CHANGE. One clause naming the mechanism for `next` specifically, since the asymmetry between the two commands is the whole difficulty and the current clause asserts symmetry. Any of the three options above works; whichever is chosen, say it, and if it adds a field to `NextProjection`, reconcile check 14h's "exactly the added fields and nothing else" with it in the same clause.

---

## R2-2. `medium`. The target text the fix pass wrote for `src/next.rs:108-109` is falsified by inc2's own change, so following line 353 still writes a fresh false statement into shipped code

THE FIX UNDER TEST. Round 1's `EX-3` was that the sidecar's diagnosis of the `src/next.rs:108-109` doc comment was wrong. The fix corrected the diagnosis and added a target text at line 204: "`active_loop` is `None` ONLY when there are no steps or when every step is terminal ... THAT is the target text for the reconciled comment." Line 353, in the INC2 documentation-impact list, now instructs: "reconcile the comment to the target text given above rather than adding a variant to satisfy the comment."

THE TARGET TEXT IS TRUE ONLY UNTIL INC2 LANDS, AND INC2 IS THE INCREMENT THAT WRITES IT. Inc2 adds a THIRD cause of `active_loop: None` in the same change:

- line 184: on an unsafe pairing "the whole `ACTIVE LOOP` block is omitted";
- line 223: `metrics-not-this-project` exists precisely because "the round log resolved for this plan is not the plan's own, so no loop state can be derived from it";
- check 14e, line 324, pins it literally: "`next --json` must show `"active_loop": null` WITH `"no_active_loop_reason": "metrics-not-this-project"`".

So the enum specified at lines 221 to 223 enumerates THREE causes of a `None` loop while the target text 17 lines earlier says there are TWO, and the documentation-impact list tells the implementer to write the two-cause version into the doc comment during the increment that adds the third. An implementer executing line 353 literally ships a comment that is false on the day it lands, which is the identical failure `EX-3` was raised about, reintroduced by its own fix.

WHAT SHOULD CHANGE. Extend the target text by one clause so it enumerates what inc2 leaves behind: no steps, every step terminal, or a metrics pairing the tool cannot vouch for. That is a correction to line 204, not an addition, and it makes the doc comment agree with the enum specified below it.

---

## R2-3. `medium`. The new precedence rule is pinned by no acceptance check, and the shape of the existing code leads an implementer straight to the answer it forbids

THE FIX UNDER TEST. Line 231, added at this commit: "THE PRECEDENCE RULE, on both path fields: where an absent cause and an unsafe cause both apply, THE UNSAFE VARIANT WINS, because unsafe is not absent (above) and a bare absence is exactly the conflation this vocabulary exists to prevent." The fix pass also removed the existence qualifier from line 217 (it read "a file exists at the resolved path, but it is not under the plan's project root" at `6df032c`), which is what makes the two variants genuinely overlap and makes the rule necessary rather than decorative.

THE RULE IS CORRECT AND UNTESTED. The acceptance list is the sidecar's own standard for settling a round (line 302: "Every claim below is a command with an expected exit code, so a round is settled by running it rather than by reading the diff"), and check 14f (line 325), whose entire stated purpose is that "THE VOCABULARY ACTUALLY SEPARATES THE CAUSES", runs exactly three non-overlapping cases: (a) a genuinely absent log, (b) an existing foreign log, (c) no plan source. Round 1's `EX-2` asked for a fourth run pinning the overlap and the fix pass did not add one. No other check in the file exercises an input where both causes hold.

THE DEFAULT IMPLEMENTATION GETS IT BACKWARDS. Today's code tests existence FIRST, at `src/main.rs:1090` (`let metrics = if args.metrics.exists() {`) and `src/main.rs:1200` (`let (rounds, metrics_records) = if args.metrics.exists() {`). An implementer extending those branches in place writes `if !exists -> log-absent, else if !contained -> log-not-this-project`, which reports `log-absent` for a `--metrics` outside the root whose leaf does not exist. That input is squarely inside the omit's trigger, because line 180 makes containment the trigger and line 164 resolves a non-existent leaf through its longest existing ancestor. The reported result would be a bare absence for a log the tool cannot vouch for, which line 188 names as the exact conflation the vocabulary exists to prevent ("UNSAFE IS NOT ABSENT"), and every acceptance check in the file passes.

WHAT SHOULD CHANGE. Add the fourth run to check 14f: an explicit `--metrics` outside the plan's root naming a file that does not exist must serialise `log-not-this-project`, not `log-absent`. One clause in an existing check, no new prose section. The same applies to `resume_state_absent_reason`'s overlap (an explicit `--ledger-fragment` outside the root that does not exist), which check 14g at line 326 likewise runs only on its three non-overlapping cases.

---

## R2-4. `medium`. Check 14b's stated red depends on a fixture mutation the check does not state, and the acceptance list uses three different spellings for the fixture it means

THE CHECK. Line 321: "From the agent-scaffold root, `agent-scaffold next --source "$FIXTURE/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl`. Before inc2 this prints `state: converged`, `streak: 1/1`, `rounds: 2/5` and `next: mark the step complete, re-render, and commit` at exit 0." Line 303 designates this as one of inc2's three required red cases under `Q-66`, and `Q-66` requires the round report to state which mutation produced the red.

MEASURED, BOTH READINGS. Fixture built by the command at line 28, then the single step set to `in-progress`, which is the only precondition check 14d (line 323) states for 14b:

```
$ agent-scaffold next --source "$S/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl
task: TEMPLATE
metrics: 240 records
ACTIVE LOOP
  example-step  in progress -> record-round
  state: awaiting-first-review
  streak: 0/?
  rounds: 0/5
  next: spawn a reviewer for the first review round
exit: 0
```

That is NOT check 14b's stated red. It is, word for word, the output check 14d forbids as the post-fix answer ("the output must NOT be the zero-rounds projection (`state: awaiting-first-review`, `next: spawn a reviewer for the first review round`)"). With the borrowed slug applied as well, the stated red does appear:

```
$ sed -i 's/^slug = "example-step"/slug = "triager-runs-only-on-findings"/' ...
$ agent-scaffold next --source "$S/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl
  triager-runs-only-on-findings / triager-runs-only-on-findings-inc1  in progress -> mark-step-complete
  state: converged
  streak: 1/1
  rounds: 2/5
  next: mark the step complete, re-render, and commit
exit: 0
```

The borrowed slug is stated in the defect B narrative (lines 83 to 93) and in check 4 (line 310, where the step is `complete`, not `in-progress`), and nowhere in 14b, 14c, 14d or 14e. WHICH fixture `$FIXTURE` names is itself undefined: line 28 and checks 2 to 6 use `$SCRATCH` and produce `TEMPLATE.plan.toml`; checks 7, 11, 14b and 14c use `$FIXTURE`, which the file never introduces; and checks 11 and 18 name a plan file `p.plan.toml` that no fixture in the file produces. An implementer who rebuilds per check 2 (as check 2 tells them to) and applies 14d's one qualifier gets a fixture on which the increment's primary human-surface red does not reproduce, and the sharper risk is a test written against that fixture, which would pass trivially both before and after the change.

WHAT SHOULD CHANGE. State 14b's fixture in the check: the borrowed slug `triager-runs-only-on-findings` at `in-progress`. Then either define `$FIXTURE` once at line 28 beside `$SCRATCH` or use one name throughout, and reconcile `p.plan.toml` in check 11 with the fixture's actual `TEMPLATE.plan.toml`.

---

## R2-5. `medium`. On `next`'s human surface an unsafe ledger is omitted with NO reason printed, which contradicts the decision the increment implements

THE DECISION AND THE PER-SURFACE RULE. Line 172: "When the pairing is nonetheless UNSAFE, they OMIT THE WORKFLOW-DERIVED FIELDS, say WHY in their place, and EXIT 0." Line 183 applies that to `status --resume` in as many words: "Print a note naming the ledger path that was rejected and why, and print NO part of the block." Line 184, for `next`, says only: "the `RESUME STATE` echo is omitted when the ledger is the unsafe artifact". No note, no reason.

WHAT THAT PRODUCES. `render_human` prints the resume block only when `resume_state` is `Some` (`src/next.rs:1036-1041`) and prints nothing whatsoever when it is `None`. Measured, with an explicit `--ledger-fragment` naming a file that does not exist:

```
task: allterm
source: /tmp/rev-r2-inc2-scratch/vocab/docs/plans/allterm.plan.toml
metrics: 240 records

no active review loop (all steps complete)
exit: 0
```

There is no line at all for the absent block. An implementer executing line 184 literally therefore ships `next` producing byte-identical output for "there is no resume state" and "I rejected your ledger fragment because it is not this project's", which is the same conflation line 188 forbids for the metrics half ("absent means 'this project has no rounds', unsafe means 'this tool cannot tell you anything about this project's rounds'"), applied to the third artifact. The machine surface does say why (check 14g, line 326, pins `ledger-not-this-project`), so `next --json` and `next` disagree about whether anything happened, and no acceptance check covers `next`'s human surface for the ledger case: 14c covers `status --resume`, 14g covers `--json`.

WHAT SHOULD CHANGE. One clause in line 184 giving `next` the same note line 183 gives `status --resume`, and one clause in check 14c or 14g asserting it. If the deliberate answer is that `next`'s human surface stays silent, say that instead, so it reads as a decision rather than as an omission from the rule at line 172.

---

## R2-6. `low`. The inc2 documentation-impact list misses `src/next.rs:111-112`, whose two-cause enumeration inc2 makes short by one, and the sweep that produced the list is asserted as complete

THE CLAIM. Line 198: "Three doc claims are falsified or made incomplete by it, found by SWEEPING `src/next.rs` and the `status` projection for exhaustiveness claims rather than by patching the one already known." Line 353 lists them: `src/next.rs:114-115`, `src/next.rs:95-97`, `src/main.rs:561-564`.

THE MISS. `src/next.rs:111-112` is the doc comment on `resume_state`:

```
	/// The ledger's `## RESUME STATE` block, extracted verbatim, or `None` when the
	/// ledger is absent or carries no such section.
```

That is an exhaustive enumeration of the causes of `None`, in the same struct, of exactly the class the sweep says it looked for, and inc2 adds a third cause to it. The sidecar proves the point itself at line 225, in the paragraph that specifies the new field: "after inc2, `next --json` can omit the resume block for a THIRD reason". Three causes are then named at lines 227 to 229 while the comment names two.

WHY IT IS SPECIFICALLY THIS COMMENT AND NOT ITS NEIGHBOURS, which I checked and am not raising: `src/next.rs:106` ("present only when the metrics log was readable") and `src/main.rs:569` ("present only when the metrics log exists") are necessary-condition claims, so they survive an extra cause of absence unscathed. `src/next.rs:111-112` is the only uncited one phrased as a complete list.

WHAT SHOULD CHANGE. Add `src/next.rs:111-112` to the line 353 list and correct "Three doc claims" at line 198 to four. This authors no prose; it adds a citation and a numeral.

---

## R2-7. `low`. No precedence is stated for `no_active_loop_reason` when the loop's absence is over-determined, and the narrowed correlation rule does not decide it

THE NARROWING UNDER TEST. Line 233 now reads: "`no_active_loop_reason` is `metrics-not-this-project` WHEN the loop's absence is metrics-derived rather than step-derived." That closes round 1's `EX-2` sub-claim 3, where the rule was stated absolutely and could not be met absolutely.

THE RESIDUAL. The narrowing assumes the absence has ONE derivation, and two real inputs make it have two at once. Both measured:

```
$ agent-scaffold next --source .../allterm.plan.toml        # one `deferred` step, one `skipped`
no active review loop (all steps complete)

$ agent-scaffold next --source .../nosteps.plan.toml        # a plan with no [[step]] at all
no active review loop (no plan steps found)
```

Add an unsafe `--metrics` to either and the absence is BOTH step-derived and metrics-derived: the plan would have had no loop anyway, and the unsafe pairing independently requires the block to be omitted (line 184). Line 233 does not say which wins, and line 231's precedence rule is scoped explicitly to "both PATH fields", which `no_active_loop_reason` is not. The implementer chooses silently between reporting `all-steps-complete` (leaving a consumer that trusts the shared `not-this-project` token to correlate the fields, per line 233's own justification, without its correlate) and reporting `metrics-not-this-project` (asserting the log is why there is no loop when the plan has no work). Check 14f's case (b) does not cover it, because check 14d requires that fixture's step to be at `in-progress`.

WHAT SHOULD CHANGE. Extend line 231's scope from "both path fields" to all three reason fields, or add the half-sentence to line 233 saying which derivation wins when both hold. One clause either way.

---

## R2-8. `low`. `all-steps-complete` is minted as a machine token for a case where no step is complete, and the sidecar names the same condition correctly 18 lines earlier

MEASURED. A plan whose only steps are one `deferred` and one `skipped`, with nothing `complete` anywhere:

```
$ agent-scaffold next --source /tmp/rev-r2-inc2-scratch/vocab/docs/plans/allterm.plan.toml
no active review loop (all steps complete)
```

The condition in the code is `is_terminal`, which is `Complete | Skipped | Optional | Deferred` (`src/next.rs:421-426`), and the sidecar states it correctly at line 204: "`active_loop` is `None` ONLY when there are no steps or when EVERY STEP IS TERMINAL". Line 222 then names the variant `all-steps-complete`.

WHY IT MATTERS DESPITE THE HUMAN STRING BEING PRE-EXISTING. The human string is out of scope and I am not asking for it to change; line 219 is right that mapping each variant back to today's exact string keeps the retype behaviour-preserving. What is new here is the MACHINE contract: a closed enum whose stated purpose is that "a consumer can TELL THE CAUSES APART" (line 212), fixed in this file against widening or renaming without a new decision (line 373). A consumer reading `all-steps-complete` on a plan with deferred work left in it is told something false, and the file's own words for the condition are available two paragraphs up.

WHAT SHOULD CHANGE. Either name the variant for the condition it tests (`all-steps-terminal`, still printed as today's "all steps complete", which keeps the human surface and its golden unchanged), or add the half-sentence recording that the token deliberately mirrors the legacy human string even though `is_terminal` covers skipped, optional and deferred. Either is one edit; leaving it unlabelled is what the section's own governing rule at line 204 forbids.

---

## R2-9. `low`. The containment predicate's root derivation is unspecified when the plan source cannot be canonicalised, on two commands that must not fail

THE INSTRUCTION. Line 164: "derive the plan source's root from its REAL (canonicalised) location, RESOLVE THE METRICS PATH by absolutising and canonicalising ITS LONGEST EXISTING ANCESTOR and re-appending the components below it (so a log whose leaf does not exist yet still has its directory prefix resolved)". The resolve-as-far-as-possible treatment is spelled out for the metrics path and NOT for the source, in the same sentence, so the asymmetry reads as deliberate and the source is assumed to exist. Line 180 makes the same predicate the trigger on all three surfaces.

THE INPUT THAT BREAKS THE ASSUMPTION. `status` and `next` tolerate a `--source` that does not exist and must keep exiting 0. Measured today:

```
$ agent-scaffold next --source /tmp/rev-r2-inc2-scratch/does-not-exist/docs/plans/x.plan.toml --metrics docs/metrics/workflow.jsonl
task: x
source: no plan source
metrics: 240 records
exit: 0
```

`toml_source` returns `Ok(None)` for a non-existent path at `src/main.rs:1035-1037`, so this is a normal, supported invocation. `std::fs::canonicalize` returns `Err` on a missing path, and `run_next` and `run_status` both return `io::Result<()>`, so the shortest Rust spelling of line 164 propagates with `?` and turns exit 0 into a non-zero exit. That breaks check 14's "`status` and `next` NEVER exit non-zero under any of these inputs" (line 320) and `README.md:226`'s never-fails contract, and no acceptance check covers a non-existent source. (`validate --workflow` is unaffected in outcome, since the same input already exits 1 through the `(None, None, _)` arm at `src/main.rs:995-998`, which I confirmed, but the guard runs BEFORE that arm per line 164, so its behaviour there is undefined too.)

WHAT SHOULD CHANGE. One clause: when the plan source cannot be canonicalised there is no root, so the containment predicate does not fire and the projections behave as they do today. That also keeps the predicate consistent with line 158's treatment of the no-anchor case.

---

## R2-10. `low`. The reasons cannot be computed inside `project`, the sidecar does not say the caller supplies them, and the one spelling that compiles inside `project` makes a byte-compare golden depend on the filesystem

THE GAP. Lines 214, 219 and 225 specify three fields ON THE PROJECTION and line 225 notes the causes "are already distinguished IN THE CODE at `src/main.rs:1208-1212`", which is in the CALLER. The projection is built by `next::project` from `NextInputs` (`src/next.rs:525-539`), which carries `metrics_records: Option<usize>` and `resume_state: Option<String>` and no path for either, so `project` cannot derive either reason from what it is given. The sidecar never says the reasons become new `NextInputs` fields, and check 14h's "exactly the added fields and nothing else" is about the JSON, not about the input struct.

WHY THE GAP HAS A BAD ANSWER AVAILABLE. `NextInputs` DOES carry `ledger_path: String`, so `project` can compute `resume_state_absent_reason` by calling `Path::new(&inputs.ledger_path).exists()`. That compiles, satisfies the specified invariant, and makes `GOLDEN_JSON` depend on the test process's working directory, because `golden_projection` (`src/next.rs:1662-1678`) calls `project` with `ledger_path: "docs/plans/demo.ledger.md"` and `resume_state: None`. A golden asserted by byte-compare (`src/next.rs:1762`) that consults the filesystem is a flaky test in the increment whose reviewable artifact is that golden's diff.

A SECOND, SMALLER CONSEQUENCE OF THE SAME FIXTURE. Because `golden_projection` has `resume_state: None`, the invariant "`Some` exactly when `resume_state` is `None`" (line 225) forces the golden to gain a NON-NULL reason value, while check 14h (line 327) describes the new reasons as serialising "as `null` ... the same way `"resume_state": null` appears in the golden today". The two statements are about different runs and do not contradict, but an implementer reading 14h and then seeing a non-null value in the golden diff has no way to tell which of the two is the mistake.

WHAT SHOULD CHANGE. One clause saying the reasons are computed by the caller and passed through `NextInputs` alongside `metrics_records` and `resume_state`, so `project` stays a pure function of its inputs and the golden stays deterministic. That also matches line 225's own observation about where the causes are distinguished.

---

## What I checked and found nothing wrong with

Recorded so a later round does not re-run it, and because a negative result is worth having.

- THE ROUND-1 FIXES OTHER THAN THE TWO UNDER TEST ALL LANDED CORRECTLY, as far as inc2 is concerned. `no-ready-step` is gone from line 223's variant list and line 219 now records the collapse to two answers (`EX-7`). The `235` literals are gone from checks 5 and 14b while the six historical mentions are untouched (`EX-8`). The `src/main.rs:560-563` citation is now `:561-564` and matches the comment I read at those lines (`F-2`). The decoy instruction is gone from check 7 (`EX-4`). Inc1's line 273 now carries the narrow safety claim and both `EX-10` clauses.
- THE OMIT-VERSUS-REFUSE SPLIT IS STATED CONSISTENTLY EVERYWHERE I FOUND IT: lines 172, 182 to 184, 274, 320, 321, 371 and 372 all say the projections exit 0 and only the validator refuses. I found no place where inc2 misrepresents `Q-55-refusalscope` or `Q-55-jsonreason`.
- THE `no_loop_reason` COLLAPSE IS BUILDABLE. `fn no_loop_reason(steps) -> ...` at `src/next.rs:953-961` has three branches, and dropping the third leaves a total function (`if steps.is_empty() { .. } else { .. }`) that is correct given the unreachability argument at line 204, which I re-derived: `StepPhase` has seven variants (`src/next.rs:388-396`), `is_pending` and `is_terminal` cover six, `InProgress` is arm 1 of `select_active_loop`, and every non-terminal phase reaches a `Some`. No panic is needed and none is invited.
- INC2'S BOUNDARY WITH INC1 AND INC3 HOLDS UNDER SIMULATION. Line 285's stated cost is exact: checks 11, 12, 13 and 14b all turn on an EXPLICIT `--metrics` or a canonicalisation, neither of which inc1's lexical anchor touches, so all four genuinely survive inc1, which is why they belong in inc2. Line 289's ordering argument is consistent with what I measured: the two workarounds an inc3 refusal would push a user towards (standing elsewhere, passing `--metrics` by hand) are closed by inc1 and inc2 respectively, and I reproduced both as live today. Accepted cost (i) is correctly described as structurally uncatchable by the guard (the wrong path stays inside the right root), and accepted cost (ii)'s pre-fix behaviour reproduces.
- CHECK 14f'S THREE CASES ARE EACH REACHABLE AND MUTUALLY DISTINGUISHABLE on the JSON alone, which is its stated acceptance condition. I constructed all three.
- `status --resume` OWES NOTHING ON THE MACHINE SURFACE, as line 235 says: `run_status` returns from `run_resume` at `src/main.rs:1067-1069` before any serialisation, which I read and confirmed.
- EVERY `file:line` CITATION IN THE INC2 REGION THAT I OPENED RESOLVES: `src/next.rs:95-97`, `:99-118`, `:108-109`, `:114-115`, `:116`, `:187-189`, `:388-396`, `:415-417`, `:421-426`, `:589-614`, `:607-611`, `:953-961`, `:1017-1043`, `:1705`, `:1762`, and `src/main.rs:438-440`, `:561-564`, `:1067-1069`, `:1104`, `:1200-1205`, `:1208-1212`, plus `README.md:210`, `:212-224`, `:226`, `:228-237`. I found no misattribution in inc2's region.
- NOTHING IN THIS FILE RE-LITIGATES A DECIDED ITEM. I raise no objection to the enforcement tier, the increment shape, the anchor-plus-refusal mechanism, the conventionless fallback, omit-and-exit-0, the serialised reason, either accepted cost, or nearest-wins, and I do not re-raise `F-5`.
