# `workflow-enforcement-tier` plan review, round 3: FIX-INDUCED RESIDUE AND FIX VERIFICATION

Reviewer model: Claude Sonnet 5. Exact model id `claude-sonnet-5`.
Date: 2026-08-01.
Worktree: `.claude/worktrees/review-q55-r3a`, branch `review/q55-r3a`, based on commit `5169ea0`.
Commit under review: `5169ea0` (the round 2 fix pass), diffed against its parent `48eb015` (round 1's fix pass, the pre-fix baseline for round 2's ten findings).
Artifact reviewed: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (primary), `test-tmpdir-repo-assumption.md`, `status-resume-ignores-json.md`, and the `[[step]]`/`[[question]]` entries this fold adds or changes in `docs/plans/agent-scaffold.plan.toml`, plus the generated `docs/plans/agent-scaffold.md`.

## Summary verdict

NOT CLEAN. One low-severity finding (`R3A-1`).

All ten of round 2's prescribed fixes (`RES-1`, `INC2-1` through `INC2-6`, `INC2-8`, `INC2-9`, `INC2-10`) landed exactly as their triaged minimal fix prescribed, and a fold-wide twin sweep on every edited string found no unfixed twin and no fix-induced falsehood. `INC2-7` and round 1's `F-5` were correctly left untouched, as instructed. The one deliberate deviation (`INC2-6`'s four-site fix against a two-site prescription) is verified correct and necessary: the prescribed two-site fix would have left the artifact self-contradictory (a "Four doc claims" heading over a three-bullet list, and a numeral twin at the near-duplicate documentation-impact bullet that the triager's own grep string did not match). Of the three items the planner flagged as deliberately unfixed, two are ruled clean on independent verification (the line 205/bullet 203 juxtaposition is not a genuine ambiguity; the metrics-log growth from 240 to 241 records does not disturb the "386 expected" test-count claim or any other reproducible claim). The third, check 11's before-state, reproduces a genuine weakness: the fixture it uses passes W3 vacuously regardless of the metrics log's content, so the check's own "false pass is refused" framing overclaims what it demonstrates. That is `R3A-1`, rated `low`.

## Findings

| id | severity | one-line summary |
| --- | --- | --- |
| R3A-1 | low | Check 11's "before" state prints `workflow invariants hold` because its fixture's single step is `not-started` (W3 checks nothing), not because a genuine cross-project false pass was avoided; the same output reproduces with an empty, legitimate log, so the check demonstrates the refusal's path-containment trigger but not the false-pass closure its own wording claims. |

## R3A-1. `low`. Check 11's before-state relies on a vacuous W3 pass, so its "false pass" framing overclaims what the check demonstrates

TENSE APPLIED: the "before inc2" half of check 11 is an explicit claim about the CURRENT tree (inc1/inc2 are not yet implemented, so "before inc2" is today's behaviour), so I tested it directly against the working tree at `5169ea0`. The "after inc2" half is a claim about the increment's future output and is not disturbed by this finding; the objection is to the evidentiary strength of the "before" half as evidence for the defect the check exists to close.

QUOTE, current text at line 318: "11. AFTER INC2, the explicit-relative-`--metrics` false pass is refused: from the agent-scaffold root, `agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl --workflow` exits NON-ZERO with the refusal naming both paths and the derived root. Before inc2 this prints `workflow invariants hold` at exit 0."

This is one of the two sites the round 2 fix pass touched for `INC2-4` (rewriting `"$FIXTURE/docs/plans/p.plan.toml"` to `"$SCRATCH/docs/plans/TEMPLATE.plan.toml"`, closing the undefined-variable and wrong-filename defects that finding raised). The fixed reference now names a real, buildable fixture, which makes the check runnable for the first time and exposes a further property the fix pass did not touch: `$SCRATCH` is the FRESH fixture built by the command at the top of the file, whose single step is `slug = "example-step"`, `status = "not-started"`, per `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:88` and the scaffold output. `src/workflow.rs:17-18` states the operative constraint: "W3 checks only `complete` steps; the others (`skipped` and the in-flight statuses) are not checked." A fixture with no `complete` step gives W3 nothing to check, so "workflow invariants hold" is not a false pass on this input, it is a vacuous pass true of ANY metrics log, foreign or local.

REPRODUCED BY RUNNING, in both directions.

Building the fixture and running check 11's literal before-command, from the agent-scaffold root, against agent-scaffold's own 241-record log:

```
$ ./target/debug/agent-scaffold scaffold --output-dir "$SCRATCH" --write --force --principles default
$ grep -n '^slug\|^status' "$SCRATCH/docs/plans/TEMPLATE.plan.toml"
34:slug = "example-step"
36:status = "not-started"
$ ./target/debug/agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl --workflow
docs/metrics/workflow.jsonl: 241 records, valid
/tmp/.../TEMPLATE.plan.toml: 1 steps, 0 questions, valid
/tmp/.../TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit:0
```

That reproduces check 11's stated before-state exactly.

THE DECISIVE NEGATIVE: the same "holds" verdict reproduces with an EMPTY, LEGITIMATE, in-root log, which removes both candidate causes (cross-project content, out-of-root path) at once and leaves only "no complete step to check" as the explanation:

```
$ ./target/debug/agent-scaffold scaffold --output-dir "$SCRATCH" --write --force --principles default
$ mkdir -p "$SCRATCH/docs/metrics" && touch "$SCRATCH/docs/metrics/workflow.jsonl"
$ (cd "$SCRATCH" && agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow)
docs/metrics/workflow.jsonl: 0 records, valid
docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit:0
```

An empty log belonging to the fixture's own project, checked against a fixture with no complete step, also "holds". Nothing about check 11's before-state is specific to agent-scaffold's log being foreign; it is specific to the fixture having no complete step, full stop.

CONTRAST WITH CHECK 14b, WHICH GOT THE STRONGER FIX. The same fix pass, addressing the same finding (`INC2-4`), added exactly the missing precondition to check 14b: "with the fixture's single step carrying the borrowed slug `triager-runs-only-on-findings` at `in-progress`" (line 322). That is precisely the kind of precondition check 11 is missing: `INC2-4`'s own prescribed fix (three edits: name 14b's fixture explicitly, unify `$FIXTURE`/`$SCRATCH`, reconcile `p.plan.toml`/`TEMPLATE.plan.toml`) treated the variable-naming defect and the missing-precondition defect as one bundle for 14b, but the variable-naming half of that same bundle, applied to check 11, was not accompanied by a precondition addition there. Defect B's own narrative earlier in the file (`workflow-enforcement-tier.md:83-93`, "THE SHARPER DEMONSTRATION") already establishes exactly the precondition needed (borrowed slug, `status = "complete"`) and check 4 already exercises it for inc1; check 11 is the inc2 (refusal) analogue of that same scenario and does not carry it forward.

WHY THIS DOES NOT INVALIDATE THE CHECK, AND WHY IT IS `low` RATHER THAN HIGHER. The AFTER-inc2 half of check 11 is unaffected: the refusal is a path-containment predicate, independent of step status (as `src/workflow.rs:437` and the mechanism section make clear), so "exits NON-ZERO with the refusal naming both paths and the derived root" will hold on this fixture exactly as stated once inc2 lands, and an implementer who builds the refusal correctly will pass check 11 regardless of this gap. Checks 12 and 13 (the symlink and `..`-escape refusals) share the same path-only structure and are not specific to check 11. What is missing is only the EVIDENTIARY force of the BEFORE half: as written, an implementer cannot use check 11 to confirm that the refusal prevents a genuinely wrong verdict, only that it fires on an out-of-root path. That gap is closeable with the same one-clause precondition 14b already received, so it is cheap, and it does not misstate anything (every individual claim in check 11 reproduces exactly as written); it only overclaims, in its own header ("the ... false pass is refused"), what its instantiation demonstrates.

MINIMAL FIX, IF TAKEN: add the same clause 14b now carries, at check 11: run it against the fixture with the borrowed slug `triager-runs-only-on-findings` at `status = "complete"` (the same mutation check 4 already performs), so the before-state is the genuine false pass ("THE SHARPER DEMONSTRATION" reproduced under `--metrics`) rather than a vacuous one. One clause, no new section, the same shape as the fix already applied to 14b.

## The INC2-6 deviation, ruled on independently

The fix pass took four sites for `INC2-6` against the triager's two-site prescription ("Add `src/next.rs:111-112` to the list at line 353 ... and change 'Three doc claims' to 'Four doc claims' at line 198"). I ruled on this independently rather than accepting the planner's own reasoning at face value.

WHAT ACTUALLY CHANGED, verified against the current file (`workflow-enforcement-tier.md`):

1. Line 198: "Three doc claims" -> "Four doc claims" (the prescribed numeral edit).
2. Line 203 (NEW): a fourth bullet, `` `src/next.rs:111-112`, `resume_state`'s doc comment, IS SHORT BY ONE IN THE SAME WAY: ... `` (not explicitly prescribed).
3. Line 205: "A FOURTH ITEM FOUND BY THE SWEEP" -> "A FIFTH ITEM FOUND BY THE SWEEP" (not explicitly prescribed; a consequence of #2).
4. Line 354: "THE THREE DOC COMMENTS ... all three ..." -> "THE FOUR DOC COMMENTS ... all four ...", with the `next.rs:111-112` citation added (the prescribed list edit, but the numeral half of this same sentence was not called out separately by the triager's "Two sites" count).

THE REASONING HOLDS UP. Line 198's sentence ("Four doc claims are falsified or made incomplete by it") directly introduces the bulleted list at 200-203; if the numeral is bumped to "Four" without a fourth bullet, the sentence contradicts the three items under it. The triager's own two-site count did not name this as a site, but achieving it requires either the bullet (site 2) or leaving the numeral at "Three" (which would then be false, since `INC2-6`'s own finding establishes a fourth doc-comment defect exists and is in scope to fix). Adding the bullet is therefore not optional once the numeral moves, and I verified its content against source: `src/next.rs:111-112` reads exactly "The ledger's `## RESUME STATE` block, extracted verbatim, or `None` when the ledger is absent or carries no such section", which is what the new bullet quotes verbatim. Site 3 (FOURTH -> FIFTH) is a forced renumbering once a fourth bullet exists above it; leaving "A FOURTH ITEM" after four bullets would misnumber it as the fourth rather than fifth thing the sweep found. Site 4's numeral half ("THE THREE" -> "THE FOUR", "all three" -> "all four") is in the exact sentence the triager cited as "the list at line 353" for the citation add; the triager's own site-count grep (`grep -c 'Three doc claims'`) is a literal-string match that does not hit "THE THREE DOC COMMENTS" (a different string), so the prescription's "Two sites" undercounted a numeral that lives inside the same sentence being edited for the citation. The planner's diagnosis that "the triager's grep never reached a numeral twin" is accurate for this specific reason.

I swept the whole fold for both the old and new phrasing (`grep -n 'Three doc claims\|THE THREE DOC COMMENTS'` and `grep -n 'Four doc claims\|THE FOUR DOC COMMENTS'` over all three sidecars and the plan TOML) and found zero remaining occurrences of the old numeral anywhere, and both new numerals appearing exactly once each, at the two sites that need them. `src/next.rs:111-112` is cited at exactly the two sites the fix touches (203 and 354) and nowhere else. No twin, no contradiction, no residue.

VERDICT: the widening was correct, necessary for internal consistency, and landed accurately. Not a finding.

## Two flagged items ruled clean

### Line 205's "A FIFTH ITEM" and the new bullet at line 203

Read together (lines 198-205 in the current file), the two items are clearly demarcated rather than overlapping on close reading: line 198 introduces bullets 200-203 as consequences ("falsified or made incomplete by") of the `Q-55-jsonreason` change, and line 205 explicitly opens with "IS PRE-EXISTING AND IS NOT A CONSEQUENCE OF THIS CHANGE" before describing a different defect in a different doc comment (`src/next.rs:108-109`, which lists a WRONG cause rather than being merely short one cause). The phrase "SHORT BY ONE" is used only for bullet 203 (and, in matching form, at line 354 for two of the other bullets); line 205 never uses it, and its defect (an enumerated cause that yields `Some`, not `None`, i.e. a wrong item rather than a missing one) is textually distinct from "short by one". I read both paragraphs as a fresh reader would and did not find a place where an implementer could plausibly conflate the two or mis-apply one paragraph's instruction to the other's target; line 205's own closing sentence ("Do NOT add a blocked-steps variant to satisfy the comment") is unambiguous and self-contained. No finding.

### The metrics log's growth from 240 to 241 records

`docs/metrics/workflow.jsonl` now has 241 records (`grep -c . docs/metrics/workflow.jsonl` -> 241), up from the 240 the round 2 triage measured. I re-measured rather than trusted the sidecar's claims that touch record counts:

- `cargo test` (with `TMPDIR` outside any repository): 373 + 5 + 1 + 1 + 3 + 1 + 2 = 386 passed, 0 failed. The sidecar's "386 expected" (line 308) is still exact; test count is a property of the code, not the growing log, and I confirmed no test in `tests/` or `src/` hardcodes a specific record count against agent-scaffold's own live log (`grep -rn 'records, valid' tests/ src/` finds only two unrelated comments about an EMPTY log, and the one live formatting site at `src/main.rs:829`).
- `cargo clippy --all-targets -- -D warnings`: clean.
- `render docs/plans/agent-scaffold.plan.toml --check`: "up to date".
- The sidecar's own record-count narrative (233, then 235, "the second... and explorers A, B and C all reproduced 235 independently at their own base commits", 235-record log") is explicitly framed as a point-in-time historical reproduction ("the record count grows as the log accumulates"), not a live claim, so 241 today does not falsify it.

No finding.

## Enumeration: what I swept

BUILD AND TEST REPRODUCTION, all under `TMPDIR=/tmp/r3a-scratch` (outside any git repository):
- `cargo build` clean.
- `cargo test`: 386 passed (373+5+1+1+3+1+2), 0 failed.
- `cargo clippy --all-targets -- -D warnings`: clean.
- `cargo run -- render docs/plans/agent-scaffold.plan.toml --check`: "up to date".
- `grep -c . docs/metrics/workflow.jsonl`: 241.

DIFF READ IN FULL: `git diff 48eb015 5169ea0` across all three changed files (`docs/plans/agent-scaffold.md`, `docs/plans/agent-scaffold.plan.toml`, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`); confirmed `docs/plans/agent-scaffold.md`'s diff is byte-for-byte the same prose delta as the sidecar's and the TOML's (the render is a faithful projection, consistent with `render --check` reporting up to date). Confirmed via `git diff --name-only` that no `src/` or `tests/` file changed in this commit (a pure plan-document fix pass, as expected for a fix-pass round).

PER-FINDING FIX VERIFICATION, ten of ten, each checked against its triaged "MINIMAL FIX AND SITE COUNT":
- `RES-1`: `src/main.rs:1150` -> `src/main.rs:1150-1151` landed in BOTH the sidecar (line 174) and the plan TOML (`Q-55-refusalscope`, line 1702). Fold-wide sweep for bare `main.rs:1150` (without `-1151`) found zero remaining occurrences.
- `INC2-1`: took the DELETION-ONLY option (struck `` (`run_status` and `run_next`) ``) at the field-shape paragraph, AND the PREFERRED option (check 14b's reason clause reworded from "naming the unsafe pairing" to "naming the resolved log and the derived root") landed too. Fold-wide sweep: `naming the unsafe pairing` now appears exactly once, at the line-184 behaviour rule that the triage said must NOT change; the parenthetical string is gone with zero occurrences fold-wide.
- `INC2-2`: took the DELETION-CLASS option, both halves. "THAT is the target text for the reconciled comment." struck from the FIFTH-ITEM paragraph; "to the target text given above" struck from the documentation-impact bullet. Fold-wide sweep for `target text`: zero occurrences remaining anywhere.
- `INC2-3`: fourth run added to check 14f (metrics precedence) and a parallel clause added to check 14g (ledger precedence), both as prescribed ("I would take both in the same edit"). `both path fields` (line 232, the rule itself) is unchanged, as required.
- `INC2-4`: all three prescribed edits landed. `$FIXTURE` -> `$SCRATCH` at all four sites (checks 7, 11, 14b, 14c); fold-wide sweep for `$FIXTURE` found zero remaining occurrences. `p.plan.toml` -> `TEMPLATE.plan.toml` at check 11; the two remaining `p.plan.toml` occurrences (lines 256, 332) are the deliberately-generic accepted-cost example the triage said were fine to leave. The borrowed-slug precondition was added to check 14b.
- `INC2-5`: single clause added to the `next` bullet ("with the same note naming the rejected ledger path in its place that `status --resume` prints"). Appears exactly once, at the one site prescribed.
- `INC2-6`: landed via the four-site widening, ruled correct above; content verified against `src/next.rs:111-112` directly.
- `INC2-8`: `all-steps-complete` -> `all-steps-terminal`. Fold-wide sweep: old token zero occurrences, new token exactly one occurrence (the single site prescribed).
- `INC2-9`: single clause added to "THE REFUSAL" paragraph ("Where the plan source itself cannot be canonicalised there is no root, so the predicate does not fire and every surface behaves as it does today, which is the answer the no-anchor case above already gets"). Verified the cross-reference is accurate by reading the referenced paragraph directly (the earlier "THE DERIVATION" paragraph: "With neither a `--source` nor a `--plan` there is nothing to anchor to, so the historical CWD-relative path stands unchanged").
- `INC2-10`: single clause added after `metrics_absent_reason`'s definition ("This reason and `resume_state_absent_reason` below are computed by the CALLER and passed through `NextInputs` alongside `metrics_records` and `resume_state`, so `project` stays a pure function of its inputs"). Appears exactly once.

TWIN SWEEPS RUN, each over all three sidecars (`workflow-enforcement-tier.md`, `test-tmpdir-repo-assumption.md`, `status-resume-ignores-json.md`) AND `agent-scaffold.plan.toml`, per the `RES-1` lesson (never the steps directory alone):
- `main.rs:1150` (bare): 0 remaining.
- `naming the unsafe pairing`: 1 (correctly unchanged site).
- `` `run_status` and `run_next` `` parenthetical: 0.
- `target text`: 0.
- `both path fields`: 1 (unchanged, correct).
- `$FIXTURE`: 0.
- `p.plan.toml`: 2 (both deliberately-untouched generic examples).
- `all-steps-complete`: 0. `all-steps-terminal`: 1.
- `Three doc claims` / `THE THREE DOC COMMENTS`: 0. `Four doc claims` / `THE FOUR DOC COMMENTS`: 1 each.
- `next.rs:111-112`: 2 (both correct sites, 203 and 354).
- `A FOURTH ITEM` / `A FIFTH ITEM`: 0 / 1.
- `longest existing ancestor`, `cannot be canonicalised`, `REAL (canonicalised) location`: all 1, single site, no twin.
- `computed by the CALLER and passed through`: 1, single site, no twin.
- `triager-runs-only-on-findings` (the borrowed slug): appears in the sidecar's own narrative/checks (unrelated to this round's fixes, pre-existing) AND, separately, in the plan TOML as a REAL, unrelated step's slug and a `Q-63` decision receipt. This is a pre-existing naming coincidence, not fix-induced, and out of this round's scope.

CHECK-NUMBER SEQUENCE VERIFIED: extracted every acceptance-check line (`1.` through `20.`, plus `14b`-`14h`) and confirmed no duplicate, no gap, and every cross-reference between checks (14d's reference to 14b's precondition, 14e's re-run of 14b/14c, 16's re-run of check 10) still resolves to the right check after the fix pass's edits.

SIDECARS CONFIRMED UNTOUCHED BY THIS FIX PASS: `test-tmpdir-repo-assumption.md` and `status-resume-ignores-json.md` both show zero diff lines between `48eb015` and `5169ea0`, consistent with none of the ten findings targeting them; the twin sweeps above nonetheless covered both files and found no stray content requiring a fix.

NEGATIVE RESULTS, explicitly: no fix left an edited string's twin unedited elsewhere; no fix's replacement text was found false against the code it cites; the one deviation from prescription (`INC2-6`'s widened scope) is justified and landed correctly; two of the three planner-flagged unfixed items are non-issues on independent verification; the third (check 11) is real and is `R3A-1`.

## Ruled out of scope

- `INC2-7` (over-determined `no_active_loop_reason`, correlation rule ambiguity): confirmed untouched (line 234's "WHEN the loop's absence is metrics-derived rather than step-derived" is byte-identical to the pre-fix text), correctly left as an accepted residual per the triage. Not re-raised.
- Round 1's `F-5` (dangling `validation-constraints` reference): not mentioned by either this fix pass's diff or the round 2 triage as re-touched; not re-raised.
