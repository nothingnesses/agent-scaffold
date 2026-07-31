# `workflow-enforcement-tier` plan review, round 2: TRIAGE

Triager model: Claude Opus 5, 1M-context variant. Exact model id `claude-opus-5[1m]`.
Date: 2026-07-31.
Worktree: `.claude/worktrees/triage-q55-r2`, branch `plan/q55-enforcement` at commit `8756578`.
Artifact triaged against: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (primary), `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`, `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, and the `[[step]]`/`[[question]]` entries this fold adds or changes in `docs/plans/agent-scaffold.plan.toml`. `src/` is evidence, not artifact.

Findings files triaged: `workflow-enforcement-tier-planreview-r2-reviewer-residue.md` (the FIX-INDUCED RESIDUE lens, `claude-sonnet-5`) and `workflow-enforcement-tier-planreview-r2-reviewer-inc2.md` (the INC2 BUILDABILITY lens, `claude-opus-5[1m]`). Round 1's three files were read as context.

## The id collision, resolved first

Both reviewers independently used the id `R2-1` for different findings. That is a defect in the brief, not in either reviewer. The disjoint namespaces used throughout this file:

| this file | source file | source id | the finding, in one line |
| --- | --- | --- | --- |
| `RES-1` | `-r2-reviewer-residue.md` | `R2-1` | The stale `src/main.rs:1150` citation twin left in the plan TOML by F-4's scoping. |
| `INC2-1` | `-r2-reviewer-inc2.md` | `R2-1` | The carrier clause resolves `status` and does not reach `next`. |
| `INC2-2` | `-r2-reviewer-inc2.md` | `R2-2` | The target text EX-3's fix wrote is falsified by inc2's own third cause. |
| `INC2-3` | `-r2-reviewer-inc2.md` | `R2-3` | The new precedence rule is pinned by no acceptance check. |
| `INC2-4` | `-r2-reviewer-inc2.md` | `R2-4` | Check 14b's red needs an unstated fixture mutation; three fixture spellings. |
| `INC2-5` | `-r2-reviewer-inc2.md` | `R2-5` | `next`'s human surface omits an unsafe ledger with no reason printed. |
| `INC2-6` | `-r2-reviewer-inc2.md` | `R2-6` | The inc2 documentation-impact list misses `src/next.rs:111-112`. |
| `INC2-7` | `-r2-reviewer-inc2.md` | `R2-7` | No precedence for an over-determined `no_active_loop_reason`. |
| `INC2-8` | `-r2-reviewer-inc2.md` | `R2-8` | `all-steps-complete` is minted for a condition that is `is_terminal`. |
| `INC2-9` | `-r2-reviewer-inc2.md` | `R2-9` | The root derivation is unspecified when the source cannot be canonicalised. |
| `INC2-10` | `-r2-reviewer-inc2.md` | `R2-10` | The reasons cannot be computed inside `project`, and the one spelling that can makes the golden filesystem-dependent. |

## Result

11 findings triaged. 10 `VALID`, 1 `VALID BUT ACCEPT RESIDUAL`, 0 `DISMISSED`.

Adjusted severity: 0 critical, 0 high, 3 medium, 8 low.

NO FINDING WAS RULED `high` OR `critical` AND DISMISSED, so no backstop re-check is owed by this round.

Severity changes from the reviewers' ratings: `INC2-1` DOWNGRADED `medium` -> `low`; `INC2-5` DOWNGRADED `medium` -> `low`. Every other rating stands.

| id | reviewer severity | adjusted | verdict |
| --- | --- | --- | --- |
| RES-1 | low | low | VALID |
| INC2-1 | medium | low | VALID, strong claim narrowed |
| INC2-2 | medium | medium | VALID |
| INC2-3 | medium | medium | VALID |
| INC2-4 | medium | medium | VALID |
| INC2-5 | medium | low | VALID |
| INC2-6 | low | low | VALID |
| INC2-7 | low | low | VALID BUT ACCEPT RESIDUAL |
| INC2-8 | low | low | VALID |
| INC2-9 | low | low | VALID |
| INC2-10 | low | low | VALID |

## What I reproduced first-hand

All runs in this worktree with `TMPDIR=/tmp/triage-r2-scratch` (outside any repository), against `target/debug/agent-scaffold` built from `8756578`.

- `cargo build` clean. `cargo test` gives 386 passed, 0 failed (373 + 5 + 1 + 1 + 3 + 1 + 2), so check 1's "386 expected" is current. `cargo clippy --all-targets -- -D warnings` clean. `render docs/plans/agent-scaffold.plan.toml --check` reports "up to date". Both reviewers' claims on all four confirmed.
- `grep -c . docs/metrics/workflow.jsonl` returns 240, as the inc2 lens reports and not the 239 round 1 measured.
- The fixture rebuilds by the command at sidecar line 28 ("30 changed"), and `ls "$SCRATCH/docs"` prints only `plans`.
- CHECK 14b's RED, BOTH READINGS, measured. With only the precondition check 14d states (the single step set to `in-progress`, slug left at `example-step`): `state: awaiting-first-review`, `next: spawn a reviewer for the first review round`, exit 0. With the borrowed slug `triager-runs-only-on-findings` applied as well: `state: converged`, `streak: 1/1`, `rounds: 2/5`, `next: mark the step complete, re-render, and commit`, exit 0. Both exactly as the inc2 lens reports.
- CHECK 11's RED reproduces: `240 records, valid` then `workflow invariants hold`, exit 0.
- CHECK 14e's RED reproduces: `next --json` serialises exactly `task`, `source`, `metrics`, `active_loop`, `resume_state`; `no_active_loop_reason` is absent. `status --json` serialises exactly `plan` and `metrics`.
- The all-terminal plan (one `deferred`, one `skipped`, nothing `complete`) prints `no active review loop (all steps complete)`; the no-steps plan prints `no active review loop (no plan steps found)`; both exit 0.
- `next` with an explicit `--ledger-fragment` naming a file that does not exist prints NO line at all about the ledger, exit 0.
- `next --source <a path that does not exist> --metrics docs/metrics/workflow.jsonl` prints `task: x`, `source: no plan source`, `metrics: 240 records`, exit 0; the same input to `validate --workflow` exits 1 through the `(None, None, _)` arm.
- CODE CITATIONS OPENED AND CONFIRMED: `src/next.rs:95-97`, `:99-118`, `:106`, `:108-109`, `:111-112`, `:114-115`, `:116`, `:388-396`, `:415-417`, `:421-426`, `:525-539`, `:953-961`, `:1017`, `:1021-1024`, `:1032`, `:1036-1041`, `:1662-1678`, `:1705`, `:1762`; `src/main.rs:438-440`, `:558-571`, `:1031-1037`, `:1067-1069`, `:1090`, `:1104`, `:1125-1128`, `:1136-1138`, `:1180-1198`, `:1200-1212`, `:1235-1241`. Every one resolves as cited.
- THE DECISIVE NEGATIVE FOR `INC2-1`: `awk` over `src/main.rs:1160-1242` returns exactly two print statements inside `run_next`, at `:1237` (`println!("{json}")`) and `:1239` (`println!("{}", next::render_human(&projection))`). There is no third. The whole of `next`'s human output is `render_human`'s return value, with no exception.
- THE DECISIVE POSITIVE FOR `INC2-2`: `git show 8756578` on the primary sidecar shows the doc-impact bullet changed from "reconcile the comment to what the code distinguishes rather than adding a variant to satisfy the comment" to "reconcile the comment TO THE TARGET TEXT GIVEN ABOVE rather than adding a variant to satisfy the comment", in the same commit that added the two-cause target text at line 204. Both halves quoted in full under `INC2-2`.

## The two lenses contradict each other, and the residue lens is wrong

The residue lens reports NO fix-induced prose residue in the region it swept, including an explicit per-finding clearance of EX-3's fix at its line 27 ("EX-3 (diagnosis correction + target text): present at `:204`, and I independently re-read the cited code ... and confirmed every citation resolves exactly as the target text states"). `INC2-2` says the target text that EX-3's fix WROTE is falsified by inc2's own third cause of `active_loop: None`. Both positions cannot stand.

`INC2-2` HOLDS. THE RESIDUE LENS MISSED A DEFECT INSIDE A REGION IT EXPLICITLY CLAIMS TO HAVE SWEPT. The grounds are in the finding below; the calibration point is here, because the project needs it separately from the fix.

WHY THE MISS HAPPENED, mechanically rather than as a rebuke. The residue lens tested the target text the way you test a DESCRIPTIVE claim: it opened `src/next.rs:108-109`, `:388-396`, `:415-417`, `:421-426`, `:589-614` and `:607-611` and confirmed the sentence is true of the code as it stands. It is. I re-derived it independently and reached the same result: `StepPhase` has seven variants, `is_pending` covers two, `is_terminal` covers four, `InProgress` is arm 1 of `select_active_loop`, so `active_loop` is `None` today only for an empty or all-terminal step list. The sentence is not the artifact's claim about today, though. It is designated (line 204) and pointed at (line 353) as the TEXT TO WRITE INTO SHIPPED CODE DURING INC2, and inc2 adds a third cause of `None` in the same change. The right test for a forward-looking instruction is against what the increment does, not against what the tree contains, and that test was not run.

A SECOND, INDEPENDENT SIGN THE SWEEP WAS INCOMPLETE, found inside the residue file itself. Its headline at line 14 enumerates what it cleared as "the three one-clause prose additions (EX-1, EX-2, EX-10)" and "the three narrowings (EX-2's qualifier deletion and correlation-rule narrowing, EX-5's replacement clause, EX-9's replacement clause)". EX-3's fix is in NEITHER list, and the diff shows it authored a new two-sentence passage ("`active_loop` is `None` ONLY when ... THAT is the target text for the reconciled comment."), which is a prose addition of exactly the class the file says it concentrated on. So the lens's own taxonomy of the fix pass's prose is short by one, and the item it omitted is the one that carried the residue. The clearance at its line 27 is real but was made under the wrong test; the clearance at its line 14 does not cover the item at all.

THIS IS THE SECOND DEMONSTRATED GAP IN A STATED COVERAGE CLAIM IN THIS REVIEW. Round 1's fidelity lens named order 93 among the cross-references it had checked and had missed the wrong status label on it; round 2's residue lens named EX-3's target text among the fixes it had verified and had verified it against the wrong baseline. Both gaps were found by a lens with a different question, not by a re-run of the same lens. The recurring shape is that ENUMERATING WHAT WAS CHECKED IS NOT THE SAME AS STATING THE TEST APPLIED, and both misses sit precisely where the test applied was the wrong one for the claim's tense. That is worth carrying into the reviewer brief: a review of a plan is a review of INSTRUCTIONS, and an instruction is checked against the state its increment will produce.

WHAT THE RESIDUE LENS GOT RIGHT AND SHOULD KEEP CREDIT FOR. Its fourteen per-finding relocations are all correct (I spot-checked EX-1 at `:212`, EX-2 at `:217`/`:231`/`:233`, EX-4's deletion at `:313`, EX-7's at `:219-223`, EX-8's at `:311`/`:321` with the six historical `235` mentions intact at `:72`, `:75`, `:81`, `:122`, `:164`, `:263`, F-2 at `:202`/`:353`, F-3 at `:59`, F-4 at `:174`, EX-6 at `:342`). Its single finding, `RES-1`, is valid and is the round's cleanest scope lesson. Its negative result on the other five prose edits survives this triage: I attacked EX-1's, EX-2's, EX-5's, EX-9's and EX-10's clauses independently and found no residue in any of them beyond what `INC2-1` and `INC2-7` raise, and both of those are residuals of the original findings rather than new falsehoods authored by the fix.

## Did round 1's EX-1 fix fail? No

`INC2-1`'s overall conclusion (a QUALIFIED NO on inc2's buildability) rests entirely on the claim that the EX-1 fix DID NOT WORK. I attacked that in both directions and it does not survive in the form given. The detail is under `INC2-1`; the ruling is here because the brief asks for it plainly.

THE CLAIM. Line 212's added clause names `run_status` and `run_next` as the callers that assemble the human message; that is true of the first and false of the second, because the whole of `next`'s human output is `render_human`'s return value at `src/main.rs:1239`, a pure function of a projection that carries no paths.

WHAT REPRODUCES. The structural half is exact, and I confirmed it more strongly than the finding did: `run_next` contains exactly two print statements in its whole body, both inside the `if args.json` branch at `:1237` and `:1239`. `render_human` is `pub(crate) fn render_human(projection: &NextProjection) -> String` at `src/next.rs:1017`; `NextProjection` at `:99-118` carries `task`, `source`, `metrics`, `active_loop`, `resume_state`, `no_active_loop_reason` and no path or root; the two lines that must name paths are produced at `:1021-1024` and `:1032`, inside that function. `run_status`'s human print really is inline at `src/main.rs:1125-1128` with `args.metrics` in scope. The asymmetry between the two commands is real and the finding states it accurately.

WHAT DOES NOT SURVIVE. The clause is a DIRECTIVE, not a description, and the finding grades it as a description. Nothing about an unsafe-pairing message exists in the tree today, so "the CALLER assembles that message" is not true of `run_status` either as a statement about current code; it is true of `run_status` only in the sense that `run_status` is already positioned to do it. Read as what it is, the clause instructs both callers to supply the human text from paths they hold, and `run_next` DOES hold them: `args.metrics` at `src/main.rs:1200` and the source label at `:1180-1184`. Round 1's triager had already established that nothing in the sidecar constrains `render_human`'s signature and listed three routes that reach the surface, and the fix pass wrote the clause the triager specified, almost verbatim. A fix that does exactly what triage asked, and whose target property (the enum carries the machine token, the caller carries the paths) is achievable on both commands, did not fail.

WHAT SURVIVES, AND IT IS SMALLER THAN ROUND 1'S. Two things. First, the parenthetical asserts a symmetry between the two callers that does not hold structurally: `run_status` needs no restructuring and `run_next` must either change `render_human`'s signature, add a non-serialised detail field, or print outside it, and the sidecar acknowledges none of that. Second, and this is the operative residual, THE PATH-NAMING REQUIREMENT IS PINNED BY NO CHECK. Lines 184 and 223 both require `next`'s human text to name the resolved log and the derived root, but check 14b asks only for "a reason naming the unsafe pairing", so the bare-token outcome still passes everything runnable. That is true, it was true in round 1, and the EX-1 fix did not address it because round 1's triager did not ask it to. It is a check gap, not a carrier gap, and its fix is a phrase substitution inside an existing check rather than a third pass of prose over line 212.

CONSEQUENCE FOR THE LENS'S ANSWER. "The clause added to prevent it does not reach the surface it was added for" is over-strong and I am not carrying it. The finding is valid at `low`, and the QUALIFIED NO that rests on it does not stand on that ground. My own answer on buildability is at the end of this file and is different.

---

## RES-1. `VALID`. Severity `low` (unchanged). F-4's fix corrected one citation and left its identical twin stale in the plan TOML

REPRODUCED EXACTLY. `grep -rn 'main.rs:1150' docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.steps/` returns exactly two hits: `workflow-enforcement-tier.md:174`, now correctly `src/main.rs:1150-1151`, and `docs/plans/agent-scaffold.plan.toml:1702`, still `src/main.rs:1150`. The TOML hit is inside the `Q-55` question's `ask` field, in the `Q-55-refusalscope` paragraph, making the same argument in near-identical words ("and `run_resume`'s doc comment at `src/main.rs:1150` matches it, so a log that does not belong to this plan is exactly a part that is not available for this projection"). I read `src/main.rs:1147-1151` and confirm line 1150 ends "...exits 0, since" and line 1151 carries "`status` is a best-effort projection, not a validator", so the bare `:1150` is exactly the imprecision F-4 named.

THE DIAGNOSIS OF CAUSE IS CORRECT AND IS THE FINDING'S REAL VALUE. Round 1's F-4 stated its site count as "`grep -rn "src/main.rs:1150"` over the steps directory returns one hit", which by construction cannot see the plan TOML. The fix pass applied the triager's stated scope faithfully and inherited its blind spot. Before the fix both citations agreed and were equally imprecise; after it they disagree, so the fold now contradicts itself about where one doc comment lives. This is residue of the fix's BOUNDARY, not of its wording, which is the distinction the residue lens draws and it is the right one.

MINIMAL FIX AND SITE COUNT, GREPPED OVER THE WHOLE FOLD. Single-site: widen `docs/plans/agent-scaffold.plan.toml:1702` from `` `src/main.rs:1150` `` to `` `src/main.rs:1150-1151` ``. A number edit, authors no prose. I re-ran the residue lens's twin sweep over the whole fold for the other strings the round 1 fixes touched (`235`, `no-ready-step`, `every pending step blocked`, `560-563`, `995-998`, `four constraint`, `:441`, `w3_problems`, `next.rs:1339`, `still exits 0`, `decoy`) and found no second twin, so this is the only one.

THE SCOPE LESSON, WHICH OUTLIVES THE FIX. Every site count in THIS file was taken over all three sidecars AND `docs/plans/agent-scaffold.plan.toml`, not over the steps directory. The plan TOML carries decision prose that paraphrases the sidecars, so any sidecar sentence that restates a decision has a candidate twin there by construction.

---

## INC2-1. `VALID`. Severity DOWNGRADED `medium` -> `low`. The carrier clause asserts a symmetry the two commands do not have, and the requirement it carries is pinned by no check

REPRODUCED, and the structural half is stronger than the finding claims. `run_next` (`src/main.rs:1171-1242`) contains exactly two print statements in its entire body, at `:1237` and `:1239`, both inside the `if args.json` branch; I confirmed this by scanning every `println!`/`eprintln!` in the range rather than by reading the tail. So `next`'s human output is `render_human(&projection)` and nothing else, `render_human` (`src/next.rs:1017`) is a function of `NextProjection` alone, and `NextProjection` (`:99-118`) carries no path or root. The two lines the sidecar requires to name paths are `:1021-1024` (the metrics line) and `:1032` (the no-active-loop line), both inside that function. `run_status`'s human print really is inline at `src/main.rs:1125-1128`. Every code fact holds.

THE STRONG CLAIM DOES NOT SURVIVE, FOR THE REASON GIVEN IN THE RULING ABOVE. The clause is prescriptive. Judged as a description it is false of `run_next` AND of `run_status`, since neither assembles an unsafe-pairing message today; judged as an instruction it is achievable on both, since `run_next` holds `args.metrics` (`:1200`) and the source label (`:1180-1184`). Round 1's triager had already found three routes that no sidecar clause closes, and the fix pass wrote the clause the triager asked for. "Does not reach the surface it was added for" overstates a real but narrower gap.

WHAT SURVIVES, IN TWO PARTS.

- THE PARENTHETICAL ASSERTS A SYMMETRY THAT DOES NOT HOLD. `run_status` is already the assembler of its own human text; `run_next` assembles none of its own and would become an assembler for the first time, or would have to change `render_human`'s contract. The sidecar names neither consequence, and the asymmetry between the two commands is, as the finding says, the whole difficulty in this section.
- THE REQUIREMENT IS PINNED BY NO CHECK, AND THIS IS THE OPERATIVE HALF. Lines 184 (the metrics line "as for `status`") and 223 ("Printed with a reason naming the resolved log and the derived root") both require `next`'s human text to name both paths. Check 14b (line 321) asks only for "a reason naming the unsafe pairing". Check 14e (line 324) pins the bare token on the machine surface deliberately, and check 14h (line 327) pins the JSON. So an implementation that prints `no active review loop (metrics-not-this-project)` with a pathless metrics line violates lines 184 and 223 and passes every runnable check. The sidecar's own standard (line 303: a round is settled by running the checks) is therefore not met for this requirement.

WHY `low` AND NOT `medium`. Severity is absolute impact if left unfixed. Round 1 rated this `medium` when the sidecar named NO carrier at all, so the requirement was arguably unsatisfiable as written and an implementer could reasonably conclude the spec was self-defeating. That is closed: the carrier is named, the requirement at lines 184 and 223 is unambiguous, and a bare token is now a plainly visible spec violation that a reviewer of a `risky` increment with two required clean rounds is reading the sidecar to catch. What remains is one unpinned requirement plus some implementer latitude among three viable routes, on one line of one command's human output. That is the same band as EX-6's stale help string, not the same band as an unsatisfiable specification.

MINIMAL FIX AND SITE COUNT, GREPPED OVER THE WHOLE FOLD. Two independent options, both single-site, and I would take the first.

- PREFERRED, AND IT IS A SUBSTITUTION RATHER THAN NEW PROSE: at check 14b, line 321, replace "must print a reason naming the unsafe pairing in their place" with a clause requiring the reason to name the resolved log and the derived root, matching line 223's words. `grep -c 'naming the unsafe pairing'` over the whole fold returns 2, at lines 184 and 321; only 321 is a check and only 321 needs to change, since line 184 is the behaviour rule and is already correct by its cross-reference to `status`. This converts the bare-token outcome from a passing build into a failing check, which is precisely what the finding says is missing, and it commits the plan to no mechanism.
- DELETION-ONLY, AND IT IS OPTIONAL: strike the parenthetical `` (`run_status` and `run_next`) `` at line 212. `grep -c 'run_status` and `run_next'` over the whole fold returns 1. The surrounding sentence stays true and route-neutral without it. This removes the false symmetry at zero prose cost but closes nothing on its own.

DO NOT ANSWER THIS WITH A THIRD PASS OF PROSE OVER LINE 212. That sentence has now been authored once and reviewed twice; a clause naming a mechanism for `next` would commit the plan to one of three routes an implementer is entitled to choose between, and this project's calibration data says the added prose is where the next round's finding comes from.

---

## INC2-2. `VALID`. Severity `medium` (unchanged). EX-3's own fix replaced an open instruction with a closed one that inc2 falsifies, which is fix-induced residue in the region the residue lens cleared

REPRODUCED, AND THE DIFF MAKES IT WORSE THAN THE FINDING STATES. `git show 8756578` on the primary sidecar shows the doc-impact bullet changing as follows.

Before (`6df032c`): "...whose "every pending step blocked" names a distinction `no_loop_reason` (`src/next.rs:953-961`) does not draw; reconcile the comment TO WHAT THE CODE DISTINGUISHES rather than adding a variant to satisfy the comment."

After (`8756578`, line 353): "...whose "every pending step blocked" names a case that yields an active loop rather than a `None`; reconcile the comment TO THE TARGET TEXT GIVEN ABOVE rather than adding a variant to satisfy the comment."

The pre-fix instruction was OPEN and pointed at the code the implementer would be looking at, which at inc2 time is code with three causes. The post-fix instruction is CLOSED and points at a fixed sentence with two. The fix therefore did not merely leave a defect standing, it converted a correct-by-deferral instruction into an incorrect-by-specification one. That is the sharpest form of the failure mode the residue lens exists to catch, and it is in the commit the residue lens read.

THE THIRD CAUSE IS INC2'S OWN, AND THE SIDECAR PINS IT LITERALLY. Line 184: on an unsafe pairing "the whole `ACTIVE LOOP` block is omitted". Line 223: `metrics-not-this-project` exists because "the round log resolved for this plan is not the plan's own, so no loop state can be derived from it". Check 14e, line 324: "`next --json` must show `"active_loop": null` WITH `"no_active_loop_reason": "metrics-not-this-project"`". So after inc2, `active_loop` is `None` for three causes. Line 204's target text says "ONLY when there are no steps or when every step is terminal", and lines 221 to 223, seventeen lines below it, enumerate three. The artifact contradicts itself across those seventeen lines, and line 353 tells the implementer to resolve the contradiction in favour of the wrong half.

THE UNDERLYING FACT IS CORRECT AND MUST NOT BE TOUCHED. Line 204's derivation is true of today's code and I re-derived it independently: `StepPhase` has seven variants (`src/next.rs:388-396`), `is_pending` is `NotStarted | Next` (`:415-417`), `is_terminal` is `Complete | Skipped | Optional | Deferred` (`:421-426`), `InProgress` is arm 1 of `select_active_loop` (`:589-614`), and the blocked case returns `Some(build_pending_loop(..., LoopState::Blocked, ...))` at `:607-611`. That derivation is also load-bearing for EX-7's unreachability argument, which line 219's collapse of `no_loop_reason` to two answers depends on. The defect is the DESIGNATION of that sentence as the text to ship, not the sentence.

WHY `medium` AND NOT `low`. Consistency with round 1's calibration of the identical defect: EX-3 was rated `medium` on the reasoning that the sidecar's instruction "would write a NEW false statement into shipped code", and this instruction does exactly that, on the same doc comment, in the same increment, now with the extra aggravation that it was introduced by EX-3's own fix. It stays below `high` because the artefact produced is a doc comment rather than behaviour, and a reviewer of inc2 is reading the enum's three variants on the facing page.

MINIMAL FIX AND SITE COUNT, GREPPED OVER THE WHOLE FOLD. `grep -c 'target text'` over the three sidecars and the plan TOML returns 2, both in the primary sidecar, at lines 204 and 353. No twin anywhere else. Two forms, and I recommend the first.

- DELETION-CLASS, PREFERRED: strike the sentence "THAT is the target text for the reconciled comment." from line 204, and strike "to the target text given above" from line 353 so it reads "reconcile the comment rather than adding a variant to satisfy the comment". Two deletions, no prose authored, and the result is the pre-fix instruction with EX-3's corrected diagnosis left standing above it, which is strictly better than either endpoint: the implementer reconciles against the code they are writing, and the wrong premise EX-3 was raised about is gone from line 204 regardless.
- ADDITIVE, IF THE FIX PASS PREFERS EXPLICITNESS: extend line 204's designation by one clause so the target text enumerates inc2's third cause alongside the two. One clause, one site. This authors prose on a sentence that has already produced a finding, which is the shape the project's data warns about, so I would take it only if the orchestrator judges the open instruction too weak.

FIX THIS WITH `INC2-6`. Both are the same root: the artifact enumerates the causes of a `None` field as of TODAY, in a document specifying an increment that adds a cause. One pass over both sites, or the second will look inconsistent with the first.

---

## INC2-3. `VALID`. Severity `medium` (unchanged). The precedence rule the fix pass added is pinned by no check, and the existing code's shape leads straight to the answer it forbids

REPRODUCED IN EVERY PART. Line 231 carries the new rule ("THE PRECEDENCE RULE, on both path fields: where an absent cause and an unsafe cause both apply, THE UNSAFE VARIANT WINS"). Line 217 no longer carries the existence qualifier round 1 quoted from `6df032c`, so the two variants genuinely overlap and the rule is load-bearing rather than decorative. Check 14f (line 325) runs three cases, (a) a genuinely absent log, (b) an existing foreign log, (c) no plan source, none of which is the overlap; check 14g (line 326) likewise runs three non-overlapping ledger cases. I read the whole acceptance list and no other check exercises an input where both causes hold.

THE CODE-SHAPE ARGUMENT REPRODUCES. `src/main.rs:1090` is `let metrics = if args.metrics.exists() {` and `src/main.rs:1200` is `let (rounds, metrics_records) = if args.metrics.exists() {`. Existence is tested first at both sites, and an implementer extending those branches in place writes existence-first, which reports `log-absent` for an explicit `--metrics` outside the root whose leaf does not exist. Line 180 makes containment the trigger and line 164 resolves a non-existent leaf through its longest existing ancestor, so the omit fires on that input and the reported pair is `metrics_absent_reason: log-absent` beside `no_active_loop_reason: metrics-not-this-project`. That pair is internally contradictory on the machine surface and is exactly the conflation line 188 names as the thing the vocabulary exists to prevent.

WHY THIS IS NOT A RE-RAISE OF EX-2. Round 1's triager wrote the fix scope as "One contiguous section (sidecar lines 214-232) PLUS OPTIONALLY check 14f at line 324". The fix pass took the mandatory half and declined the optional half, correctly within its brief. This finding is the declined half coming back with evidence it should not have been optional: the rule's only enforcement is a reader's attention, on the one input where the default implementation shape disagrees with it. It is a continuation of EX-2, not a duplicate of it, and it is not a re-raise of anything dismissed.

WHY `medium`. If unfixed and the implementer follows the code's existing shape, the shipped tool reports a bare absence for a log it cannot vouch for, on the machine surface `Q-55-jsonreason` exists to serve, and no test fails. That is a behavioural defect in the delivered product rather than a documentation one, which is what separates this band from the `low` set.

MINIMAL FIX AND SITE COUNT, GREPPED OVER THE WHOLE FOLD. `grep -c 'both path fields'` returns 1, at line 231, and the rule's text needs no change. The fix is a fourth run appended to check 14f at line 325: an explicit `--metrics` outside the plan's root naming a file that does not exist must serialise `log-not-this-project`, not `log-absent`. One clause inside an existing check, no new section. The same clause form applies to check 14g at line 326 for the `--ledger-fragment` overlap, which is the input round 1's EX-2 sub-claim 2 raised and which the text now decides at line 231 and no check exercises; that is a second site and I would take both in the same edit.

---

## INC2-4. `VALID`. Severity `medium` (unchanged). Check 14b's stated red does not reproduce on the fixture the check tells you to build, and the acceptance list names its fixture three different ways

BOTH MEASUREMENTS REPRODUCE, EXACTLY AS THE FINDING REPORTS THEM. I rebuilt the fixture by the command at line 28 and applied only the precondition check 14d states (the single step at `in-progress`):

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

That is not check 14b's stated red. It is, word for word, the output check 14d names as the forbidden POST-fix answer ("the output must NOT be the zero-rounds projection (`state: awaiting-first-review`, `next: spawn a reviewer for the first review round`)"). Applying the borrowed slug as well produces the stated red:

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

The borrowed slug appears in the defect B narrative (lines 83 to 93) and in check 4 (line 310, where the step is `complete`, not `in-progress`), and in none of 14b, 14c, 14d or 14e.

THE THREE SPELLINGS REPRODUCE. `$SCRATCH` is introduced by the command at line 28 and used by checks 2, 3, 5 and 6. `$FIXTURE` is used at lines 313, 317, 321 and 322 (checks 7, 11, 14b, 14c) and is never introduced anywhere in the file; `grep -c FIXTURE` returns 5 lines, of which line 309's is the English word rather than the variable. `p.plan.toml` appears at lines 255, 317 and 331; the uses at 255 and 331 are the generic accepted-cost example and are fine, but check 11 at line 317 names `"$FIXTURE/docs/plans/p.plan.toml"` while the fixture the file builds produces `TEMPLATE.plan.toml`.

WHY `medium`. Line 303 designates check 14b as one of inc2's three required red cases under `Q-66`, which requires the round report to state which mutation produced the red. An implementer who rebuilds per check 2 and applies 14d's single qualifier gets a fixture on which the increment's primary human-surface red does not exist, and a test written against that fixture passes trivially before and after the change, which is the self-concealing shape this whole step exists to remove. That is an executability defect in the increment's own evidence, not a wording slip. It stops short of `high` because the borrowed slug is stated twice elsewhere in the same file and a careful implementer will find it.

MINIMAL FIX AND SITE COUNT. Three edits, all single-site, all inside existing checks and none authoring narrative prose.

- Line 321: name 14b's fixture in the check, the borrowed slug `triager-runs-only-on-findings` at `in-progress`. One clause.
- Lines 313, 317, 321, 322: use one variable name. The cheapest form is substituting `$SCRATCH` for `$FIXTURE` at all four, which authors nothing and needs no new definition line. `grep -c FIXTURE` over the whole fold returns 5 lines in the primary sidecar and 0 elsewhere.
- Line 317: reconcile `p.plan.toml` with the fixture's `TEMPLATE.plan.toml`. One name.

---

## INC2-5. `VALID`. Severity DOWNGRADED `medium` -> `low`. `next`'s human surface omits an unsafe ledger silently, which the general rule at line 172 forbids

REPRODUCED BY READING AND BY RUNNING. `render_human` prints the resume block only inside `if let Some(resume) = &projection.resume_state` (`src/next.rs:1036-1041`) and prints nothing at all when it is `None`. Measured with an explicit `--ledger-fragment` naming a file that does not exist:

```
$ agent-scaffold next --source .../allterm.plan.toml --ledger-fragment /tmp/.../nope.ledger.md
task: allterm
source: /tmp/triage-r2-scratch/vocab/docs/plans/allterm.plan.toml
metrics: 240 records

no active review loop (all steps complete)
exit: 0
```

There is no line for the absent block, so `next`'s human output is byte-identical for "there is no resume state" and "your ledger fragment was rejected".

THE ARTIFACT CONTRADICTS ITSELF, WHICH IS WHY THIS IS NOT DISMISSED. Line 172 states the decided behaviour in the human's own terms: the projections "OMIT THE WORKFLOW-DERIVED FIELDS, SAY WHY IN THEIR PLACE, and EXIT 0", and `docs/plans/agent-scaffold.plan.toml:1702` records `Q-55-refusalscope` in the same words. Line 183 applies the say-why half to `status --resume` explicitly. Line 184's rule for `next` says only that "the `RESUME STATE` echo is omitted when the ledger is the unsafe artifact", with no note and no reason. The per-surface section is titled "The exact behaviour, per surface", so the absence there reads as the specification, and it does not produce the decided behaviour.

WHY `low` AND NOT `medium`, AND IT IS A BORDERLINE CALL. Three things bound the impact. The machine surface DOES say why and is pinned (check 14g, line 326, `ledger-not-this-project`), and `Q-55-jsonreason`'s own recorded reasoning is that `--json` is the surface agents consume and the reason this decision exists. `status --resume`, the surface a human uses for this artifact, is covered explicitly at line 183 and at check 14c. And the silence is not a regression: `next` prints nothing for an absent resume block today, so the defect is a missing addition rather than a new conflation reaching a user. The input also requires an explicit `--ledger-fragment` pointing outside the plan's root, which is a deliberate act rather than an accident of where the user stood. As with round 1's EX-6, the rating does not change what the fix pass does: fix it regardless.

MINIMAL FIX AND SITE COUNT. Single-site: one clause at line 184 giving `next` the note line 183 gives `status --resume`, or, if the deliberate answer is that `next`'s human surface stays silent, one clause saying so, which is the deletion-adjacent form and is equally acceptable. `grep -c 'RESUME STATE` echo'` over the whole fold returns 1, at line 184. A second clause in check 14g asserting it is optional and I would not require it, since the machine half is already pinned there.

---

## INC2-6. `VALID`. Severity `low` (unchanged). The inc2 documentation-impact list misses `src/next.rs:111-112`, whose two-cause enumeration inc2 makes short by one

REPRODUCED. `src/next.rs:111-112` is the `resume_state` doc comment: "The ledger's `## RESUME STATE` block, extracted verbatim, or `None` when the ledger is absent or carries no such section." That is an enumeration of the causes of `None`, phrased exactly like `:108-109`, which the sidecar does treat as one. Inc2 adds a third cause, and the sidecar proves the point itself at line 225 ("after inc2, `next --json` can omit the resume block for a THIRD reason") before naming three at lines 227 to 229. Line 353's list names `src/next.rs:114-115`, `:95-97`, `src/main.rs:561-564` and `:108-109`, and not `:111-112`. Line 198 asserts the list was produced "by SWEEPING `src/next.rs` and the `status` projection for exhaustiveness claims rather than by patching the one already known", so the omission falsifies a stated coverage claim as well as leaving a site unfixed.

THE FINDING'S DISCRIMINATION AGAINST THE NEIGHBOURS IS CORRECT AND I CHECKED IT. `src/next.rs:106` ("present only when the metrics log was readable") and `src/main.rs:569` ("present only when the metrics log exists") are necessary-condition claims of the form "present implies X", which survive an extra cause of ABSENCE unscathed. `:111-112` is the only uncited comment in either struct phrased as a list of causes.

WHY `low`. The impact if unfixed is one doc comment in `src/next.rs` left short by one cause, in the same struct whose other three comments the increment corrects. It is the same band as EX-6, and for the same reason: it misleads about an enumeration, not about behaviour.

MINIMAL FIX AND SITE COUNT, GREPPED OVER THE WHOLE FOLD. Two sites, both in the primary sidecar, neither authoring prose. Add `src/next.rs:111-112` to the list at line 353 (`grep -c 'next.rs:114-115'` returns 2, at lines 200 and 353; only 353 is the list), and change "Three doc claims" to "Four doc claims" at line 198 (`grep -c 'Three doc claims'` returns 1). A citation and a numeral, which is the shape round 1 classed as authoring nothing.

SHARES A ROOT WITH `INC2-2`. Both are the artifact enumerating a field's `None` causes as of today inside a document that specifies an increment adding one. Fix them in the same pass.

---

## INC2-7. `VALID BUT ACCEPT RESIDUAL`. Severity `low` (unchanged). The narrowed correlation rule is ambiguous rather than silent on the over-determined case, and both readings yield a true value

BOTH INPUTS REPRODUCE. A plan whose only steps are one `deferred` and one `skipped` prints `no active review loop (all steps complete)`; a plan with no `[[step]]` at all prints `no active review loop (no plan steps found)`. Add an unsafe `--metrics` to either and the loop's absence is both step-derived (the plan has no work) and metrics-derived (line 184 requires the block omitted regardless).

THE FINDING'S FRAMING NEEDS NARROWING, WHICH CHANGES THE DISPOSITION. "No precedence is stated" is not quite right. Line 233's condition is "WHEN the loop's absence is metrics-derived RATHER THAN step-derived", and on an input where it is both, the condition as written is not met, so the rule does not fire and the step-derived value stands. That is a complete reading and it decides the case. The trouble is that "X rather than Y" also reads as "prefer X over Y", which decides it the opposite way, and the reviewer's own two proposed fixes are the two opposite answers, which is the best available demonstration that the sentence does not force one. So the defect is real and is AMBIGUITY, not silence. Line 231's precedence rule is genuinely scoped to "both PATH fields" and does not reach this field; I confirmed the wording.

WHY THE RESIDUAL IS ACCEPTED RATHER THAN FIXED, and I want this reasoning on the record because accepting a residual is the disposition an independent triager should be slowest to reach. Three things weigh together. FIRST, NEITHER READING PRODUCES A FALSE STATEMENT. On the over-determined input, `all-steps-complete` is true (every step is terminal) and `metrics-not-this-project` is true by its own definition at line 223 ("no loop state can be derived from it"), so whichever the implementer picks, no consumer is told something untrue. SECOND, THE SECTION'S STATED PROPERTY IS STILL MET. Line 212's requirement is that a consumer can tell the causes apart, and the unsafe cause is reported unconditionally on `metrics_absent_reason` (`log-not-this-project`) on this input regardless of which way `no_active_loop_reason` falls. What degrades is line 233's convenience claim that a consumer "needs no lookup table to correlate them", on one input class. THIRD, AND DECISIVELY, THE FIX IS PROSE ON A SENTENCE ALREADY EDITED ONCE FOR THIS EXACT INPUT. There is no deletion-only form: striking the WHEN clause reinstates round 1's EX-2 sub-claim 3 verbatim, and widening line 231's "both path fields" to cover this field creates a fresh contradiction with line 233's remaining clause. So the only fixes are an added clause in one direction or the other, on the section that has now produced findings in two consecutive rounds, to decide between two true answers on an input requiring an empty-or-all-terminal plan PLUS an explicit foreign `--metrics`. On this project's own measured rule, that trade is the wrong way round.

DEDUPLICATION. This is the residual of round 1's EX-2 sub-claim 3, on the identical input. Round 1's defect (a rule stated absolutely that could not be met absolutely, so a consumer meets either a violated invariant or a false assertion) is genuinely closed; what is left is smaller in kind, not merely in degree.

IF THE FIX PASS TOUCHES LINE 233 FOR ANY OTHER REASON, take this at the same time: one half-sentence saying the step-derived reason stands when both derivations hold, which is the reading the current words most nearly support and the one that costs the correlation claim least.

---

## INC2-8. `VALID`. Severity `low` (unchanged). `all-steps-complete` is minted as a machine token for a condition the file itself names correctly eighteen lines earlier

REPRODUCED. A plan whose only steps are one `deferred` and one `skipped`, with nothing `complete` anywhere, prints `no active review loop (all steps complete)`. The condition in the code is `is_terminal`, which is `Complete | Skipped | Optional | Deferred` (`src/next.rs:421-426`), and `no_loop_reason` (`:953-961`) reaches that string through `steps.iter().all(|step| step.phase.is_terminal())`. The sidecar states the condition correctly at line 204 ("when EVERY STEP IS TERMINAL") and names the variant `all-steps-complete` at line 222.

ONE PIECE OF EVIDENCE THE FINDING DOES NOT USE, AND IT IS THE ONE THAT SETTLES THIS AGAINST A DISMISSAL. The obvious defence is that the tokens deliberately mirror the legacy human strings, so the new token merely inherits a pre-existing inaccuracy that the file chose to preserve. That defence fails on the file's own text: the sibling token at line 221 is `no-plan-steps`, while the string it prints is "no plan steps found". The tokens are therefore already chosen INDEPENDENTLY of the strings rather than derived from them, so `all-steps-complete` was a free choice and is not forced by line 219's behaviour-preservation rule, which constrains only what `render_human` PRINTS. This is not a finding that a different option would be better; it is a finding that a new machine contract asserts something false on reachable input when the file's own words for the condition were available two paragraphs up.

WHY IT MATTERS DESPITE THE HUMAN STRING BEING OUT OF SCOPE. Line 373 fixes the variant sets in this file and forbids widening or renaming them "without a new decision", so the token is cheap now and expensive later. Line 212's stated purpose for the enum is that a consumer can tell the causes apart; a consumer told `all-steps-complete` about a plan with deferred work left in it has been told the wrong cause.

WHY `low`. The impact if unfixed is a misleading token on the machine surface for plans with deferred, skipped or optional terminal steps. It misleads about a classification, not about whether there is work to do (there is none either way), and the human string it mirrors has carried the same inaccuracy for as long as the command has existed.

MINIMAL FIX AND SITE COUNT, GREPPED OVER THE WHOLE FOLD. Single-site: `grep -c 'all-steps-complete'` over the three sidecars and the plan TOML returns 1, at line 222. Rename the token to `all-steps-terminal`, keeping "printed as today's 'all steps complete'" unchanged, which leaves the human surface and its golden untouched. One word. The alternative the finding offers (a half-sentence recording that the token deliberately mirrors the legacy string) authors prose and, given the `no-plan-steps` precedent above, would be recording a rationale the file does not actually follow, so I would not take it.

---

## INC2-9. `VALID`. Severity `low` (unchanged). The containment predicate's root derivation is unspecified when the plan source cannot be canonicalised, on two commands that must not fail

REPRODUCED BY READING AND BY RUNNING. `toml_source` returns `Ok(None)` for a path that does not exist (`src/main.rs:1031-1037`, the `if !path.exists() { return Ok(None); }` guard), so a `--source` naming a missing file is a normal, supported invocation. Measured:

```
$ agent-scaffold next --source /tmp/triage-r2-scratch/does-not-exist/docs/plans/x.plan.toml --metrics docs/metrics/workflow.jsonl
task: x
source: no plan source
metrics: 240 records

no active review loop (no plan steps found)
exit: 0
```

Line 164 instructs deriving the plan source's root "from its REAL (canonicalised) location" and spells out resolve-as-far-as-possible for the METRICS path in the same sentence and not for the source, so the asymmetry reads as deliberate and the source is assumed to exist. `std::fs::canonicalize` returns `Err` on a missing path, `run_next` and `run_status` both return `io::Result<()>`, and the shortest Rust spelling of line 164 propagates with `?`, turning exit 0 into a non-zero exit. That would break check 14's "`status` and `next` NEVER exit non-zero under any of these inputs" (line 320) and `README.md:226`'s never-fails contract, and no acceptance check covers a non-existent source. The finding's parenthetical about `validate --workflow` also checks out: the same input already exits 1 through the `(None, None, _)` arm, which I ran, but line 164 places the guard before that arm so its behaviour there is undefined too.

ONE ABRIDGEMENT IN THE FINDING'S TRANSCRIPT, NOT AN ERROR. The quoted output omits the blank line and the `no active review loop (no plan steps found)` line the command actually prints. Every load-bearing fact (`task: x`, `source: no plan source`, exit 0) reproduces exactly, so the finding is unaffected.

WHY `low`. This is round 1's EX-10 class exactly: a resolution input left to the implementer, with an obvious right answer that the implementer will probably reach anyway (the projections already treat a missing source as no source, and line 158 establishes the no-anchor treatment for the neighbouring case). Round 1 rated that class `low` on that reasoning and I am holding the same line. It stays a real gap because the one wrong answer available breaks a documented contract rather than degrading an output.

MINIMAL FIX AND SITE COUNT, GREPPED OVER THE WHOLE FOLD. Single-site: one clause at line 164 (`grep -c 'longest existing ancestor'` returns 1) saying that when the plan source cannot be canonicalised there is no root, so the predicate does not fire and the projections behave as they do today. That is the same answer line 158 already gives the no-anchor case, so the clause is a cross-reference rather than a new rule.

---

## INC2-10. `VALID`. Severity `low` (unchanged). The reasons cannot be computed inside `project`, and the one spelling that can makes a byte-compare golden depend on the filesystem

REPRODUCED IN FULL. `NextInputs` (`src/next.rs:525-539`) carries `metrics_records: Option<usize>`, `ledger_path: String` and `resume_state: Option<String>`, and no metrics path, so `project` cannot derive `metrics_absent_reason` from what it is given. The sidecar specifies all three fields ON THE PROJECTION (lines 214, 219, 225) and never says the reasons arrive as new `NextInputs` fields; check 14h's "exactly the added fields and nothing else" is about the serialised JSON, not the input struct. Line 225's own observation that the causes "are already distinguished IN THE CODE at `src/main.rs:1208-1212`" points at the caller, which is a hint rather than an instruction.

THE BAD ANSWER IS AVAILABLE AND COMPILES. Because `NextInputs` does carry `ledger_path`, `project` can compute `resume_state_absent_reason` as `Path::new(&inputs.ledger_path).exists()`. That satisfies line 225's invariant literally and makes `GOLDEN_JSON` depend on the test process's working directory, because `golden_projection` (`src/next.rs:1662-1678`) passes `ledger_path: "docs/plans/demo.ledger.md"` with `resume_state: None`, and `golden_json` (`:1762`) asserts by byte-compare against `GOLDEN_JSON` (`:1705`). A golden that consults the filesystem is a flaky test in the increment whose reviewable artifact is that golden's diff, which is the property line 208 is relying on.

THE SECOND HALF ALSO REPRODUCES AND IS SMALLER. `golden_projection` has `resume_state: None`, so line 225's "`Some` exactly when `resume_state` is `None`" forces the golden to gain a non-null reason, while check 14h describes the new reasons serialising "as `null`" for a correct run against this repository's own plan. The two statements are about different runs and both are correct (this repository's ledger has a `## RESUME STATE` block, so a real run gives `resume_state: Some` and a null reason), so this is a legibility trap rather than a contradiction, and the finding says so.

WHY `low`. If unfixed, an implementer either reaches the right answer unaided (the caller already supplies `metrics_records` and `resume_state` by exactly this route, so the pattern is in front of them) or ships a golden that consults the filesystem, which the increment's own two clean rounds are well placed to catch since the golden diff is the stated review artifact. Real, cheap, and bounded.

MINIMAL FIX AND SITE COUNT, GREPPED OVER THE WHOLE FOLD. Single-site: one clause at line 214 or 225 (`grep -c 'Some` exactly when'` returns 2, at lines 214 and 225, and one clause covering both fields belongs at the first) saying the reasons are computed by the caller and passed through `NextInputs` alongside `metrics_records` and `resume_state`, so `project` stays a pure function of its inputs. That is a statement the sidecar's own line 225 already half-makes, so it is closer to completing a sentence than to adding an argument.

---

## Deduplication

- NO FINDING IS RAISED BY BOTH LENSES. The residue lens's single finding and the inc2 lens's ten are disjoint, so nothing in this round carries the corroboration of a doubly-raised finding. Each stands on its own evidence, all of which I reproduced.
- NOTHING RE-RAISES `F-5`. Neither file mentions the dangling `validation-constraints` reference except the residue lens's explicit note that it correctly remains untouched. The accepted residual stands and its right disposition is still entering the human-decided step as its own plan item, which is an orchestrator action rather than a fix-pass one.
- NOTHING RE-LITIGATES A DECIDED ITEM. I checked all eleven against the decided list (the enforcement tier, the one-step multi-increment shape, anchor-plus-refusal with identity queued, the conventionless fallback, omit-and-exit-0 on the projections, the serialised reason, both accepted costs, nearest-wins, the open TMPDIR fork) and found no objection to any of them. `INC2-5` and `INC2-8` come closest to a preference and both are ruled on self-contradiction and on a false machine token respectively, not on a better option.
- TWO FINDINGS ARE FIX-INDUCED. `RES-1` by the round 1 fix's stated SCOPE (a grep bounded to the steps directory), and `INC2-2` by the round 1 fix's CONTENT (an open instruction replaced by a closed one). The fix pass produced exactly two residue findings out of fifteen items and one high-severity deletion, which is a good result and is evidence the deletion-first constraint is working, not evidence against it.
- THREE FINDINGS ARE CONTINUATIONS OF ROUND 1 RATHER THAN NEW DEFECTS. `INC2-1` continues EX-1's residual (narrowed from a carrier gap to a check gap), `INC2-3` is the OPTIONAL half of EX-2's fix that the fix pass declined, and `INC2-7` is EX-2 sub-claim 3's residual on the same input. None re-raises something dismissed, because round 1 dismissed nothing.
- `INC2-2` AND `INC2-6` ARE ONE ROOT CAUSE SEEN TWICE, exactly as EX-3 and EX-7 were in round 1: the artifact enumerates the causes of a `None` field as of today, inside a document specifying the increment that adds a cause. They are separate sites and neither subsumes the other, but one pass must take both.
- `INC2-4`, `INC2-5`, `INC2-8`, `INC2-9` AND `INC2-10` ARE NEW, on text the fix pass did not touch. None of them was reachable from round 1's two lenses, which is the case for having run a third.

## Errors inside the findings files

Checked because this project has repeatedly caught misnumbered citations inside findings files, in both reviewers, in round 1.

- THE INC2 LENS CITES LINE 302 FOR A SENTENCE AT LINE 303. In `INC2-3` it attributes "Every claim below is a command with an expected exit code, so a round is settled by running it rather than by reading the diff" to line 302; `grep -n` puts it at 303. The same finding then cites 303 correctly for the `Q-66` designation two clauses later in the same paragraph. Not load-bearing, and the paragraph is where it says it is.
- THE INC2 LENS'S TRANSCRIPTS ARE ABRIDGED IN THREE PLACES AND NOWHERE MISLEADING. `INC2-4`'s two runs drop `isolation:`, `role:`, `prompt:`, `context:`, `reminders:` and `summary:`; `INC2-9`'s drops the blank line and the `no active review loop` line. I re-ran all three and every quoted line reproduces verbatim in the right order. This is elision, not error, though a future round would be better served by a marked elision.
- EVERY OTHER LINE NUMBER THE INC2 LENS CITES RESOLVES. I re-derived lines 28, 158, 164, 172, 180, 182, 183, 184, 188, 198, 204, 212, 214, 217, 219, 221, 222, 223, 225, 227, 228, 229, 231, 233, 235, 274, 279, 281, 283, 285, 289, 297, 303, 310, 317, 320, 321, 322, 323, 324, 325, 326, 327, 332, 347, 353, 371, 373 in the sidecar, plus every `src/` citation in its closing list, and found no other misattribution. Its record-count claim (240), its 386-test claim, its clippy-clean claim and its render-check claim all reproduce.
- THE RESIDUE LENS'S CLASSIFICATION OF THE FIX PASS'S PROSE IS SHORT BY ONE, which is the substantive error and is treated under the contradiction ruling above rather than here. Its `235` site list (`:72`, `:75`, `:81`, `:122`, `:164`, `:263`) is exactly right post-fix, its fourteen relocations are all correct, and its `RES-1` evidence reproduces line for line.

## Guidance for the fix pass

- FIX SET, IN DESCENDING ORDER OF WHAT IT BUYS: `INC2-2` (a false statement instructed into shipped code, and the only fix-induced content defect in the round), then `INC2-4` (the increment's primary red does not reproduce on the fixture the file builds), then `INC2-3` (an unpinned rule the code's shape contradicts), then the mechanical set (`RES-1`, `INC2-6`, `INC2-8`, `INC2-1`'s check tightening), then the three one-clause additions (`INC2-5`, `INC2-9`, `INC2-10`). `INC2-7` is accepted and is not in the set.
- SEVEN OF THE TEN FIXES AUTHOR NO NARRATIVE PROSE, which is the shape the calibration data favours: `RES-1` (a number), `INC2-2` (two deletions in its preferred form), `INC2-6` (a citation and a numeral), `INC2-8` (one word), `INC2-1` (a phrase substitution inside check 14b, plus an optional deletion at line 212), `INC2-4` (three edits inside existing checks) and `INC2-3` (a clause appended to two existing checks). ONLY THREE NEED A NEW CLAUSE OF PROSE, and each is one clause: `INC2-5` (line 184), `INC2-9` (line 164), `INC2-10` (line 214). Keep them to the smallest true statement.
- `INC2-2` AND `INC2-6` MUST LAND IN THE SAME PASS. They are one root cause and a fix to either alone will read as inconsistent with the other.
- DO NOT AUTHOR A THIRD PASS OF PROSE OVER LINE 212 OR LINE 233. Both have been written once and reviewed twice. Line 212's residual is answered inside check 14b; line 233's is accepted.
- EVERY SITE COUNT IN THIS FILE WAS TAKEN OVER ALL THREE SIDECARS AND `docs/plans/agent-scaffold.plan.toml`, per `RES-1`'s lesson. Only `RES-1` itself has a site outside the primary sidecar; every other fix is confined to `workflow-enforcement-tier.md` and I found no twin for any of them in the TOML.
- AFTER EDITING ANY SIDECAR, RE-RENDER. `docs/plans/agent-scaffold.md` is a generated projection that must never be hand-edited; `render docs/plans/agent-scaffold.plan.toml --check` reports "up to date" at this commit, so it will catch a missed re-render. `RES-1`'s fix edits the TOML's `Q-55` `ask` field, which the generated view also carries, so that one needs the re-render as much as the sidecar edits do.

## My own answer on inc2's buildability

QUALIFIED YES, which is a different answer from the inc2 lens's and rests on a different reading of the same evidence.

The reason vocabulary is decidable on every input either of us constructed except two, and both exceptions are small: the over-determined `no_active_loop_reason` (`INC2-7`, accepted, both answers true), and the absent-and-unsafe overlap on the path fields, which the text DECIDES at line 231 and no check pins (`INC2-3`). The carrier question round 1 raised is closed in the text: the enum is the machine value, the caller holds the paths, and lines 184 and 223 state unambiguously that `next`'s human text must name the resolved log and the derived root. An implementer has three viable routes to that on `next` and is entitled to choose among them.

WHAT WOULD ACTUALLY STOP A COMPETENT IMPLEMENTER IS `INC2-4`, not `INC2-1`. An implementer who follows check 2 to build the fixture and check 14d for its one stated precondition cannot reproduce the increment's primary human-surface red case at all, and the file names that fixture three different ways. That is the one place in inc2 where the instructions, followed literally, do not produce the state they describe. It is also the cheapest of the three mediums to fix.

The remaining risk is not executability but VERIFICATION COVERAGE: two specified rules (the path-naming requirement at lines 184 and 223, and the precedence rule at line 231) are stated and unpinned, against a file whose own standard at line 303 is that a round is settled by running the checks. Both fixes are clauses inside checks that already exist. With `INC2-2`, `INC2-3` and `INC2-4` taken, I would expect inc2 to be executable from the sidecar alone.
