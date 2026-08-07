# `workflow-enforcement-tier-inc4`, round 1, triage

Adjudicates the three round-1 findings files against the diff `main..HEAD` in worktree `.claude/worktrees/triage-inc4-r1` at `caeab43` (the rebased form of `363ac06..079d63f`). Every fixture was built under `<scratchpad>/triage-inc4-r1/` only. Every mode change was restored and is shown restored below.

## Summary

RAW findings: 17 (A: 8, B: 3, C: 6). DEDUPLICATED: 15. Two pairs are one finding each, confirmed on the evidence and not on the description.

| id | reviewer severity | triage severity | verdict |
| --- | --- | --- | --- |
| `R1A-1` | medium | medium (confirmed) | VALID, fix required |
| `R1A-2` + `R1C-1` | medium / medium | medium (confirmed) | VALID, fix required (ONE finding) |
| `R1A-3` + `R1C-2` | medium / medium | medium (confirmed) | VALID, fix required (ONE finding) |
| `R1A-4` | low | low (confirmed) | VALID, fix required |
| `R1A-5` | low | low (confirmed) | DISMISSED |
| `R1A-6` | low | low (confirmed) | VALID, fix required |
| `R1A-7` | low | low (confirmed) | DISMISSED |
| `R1A-8` | low | low (confirmed) | DISMISSED |
| `R1B-1` | medium | medium (confirmed) | VALID, fix required |
| `R1B-2` | low | low (confirmed) | VALID, fix required |
| `R1B-3` | low | low (confirmed) | DISMISSED, demonstration does not reproduce |
| `R1C-3` | medium | medium (confirmed) | VALID, fix required |
| `R1C-4` | medium | medium (confirmed) | VALID, fix required |
| `R1C-5` | (declined by the reviewer) | medium if ruled in | FACTS ESTABLISHED, HUMAN DECISION OWED |
| `R1C-6` | low | low (confirmed) | VALID, fix required |

VALID, FIX REQUIRED: 10 deduplicated. VALID BUT ACCEPT RESIDUAL: 0. DISMISSED: 4. PENDING A HUMAN DECISION: 1.

SEVERITY MIX OF THE VALID SET: 0 critical, 0 high, 6 medium, 4 low. No severity was corrected in either direction; every reviewer rating held on its stated ground.

NOTHING WAS DISMISSED AT `high` OR ABOVE, so the independent dismissal re-check is NOT triggered. All four dismissals are `low`.

THE ROUND IS NEW-VALID. The streak stays at 0 of the 2 this `risky` increment needs. That outcome is settled by the six `medium` findings alone and does not depend on any `low` ruling or on `R1C-5`, which is stated here so no reader has to work out whether a marginal call moved it.

REPRODUCED FIRST-HAND (evidence re-run, not read): `R1A-1`, `R1A-2`/`R1C-1`, `R1A-3`/`R1C-2`, `R1A-4`, `R1A-5`, `R1A-6`, `R1A-7`, `R1A-8`, `R1B-1`, `R1B-2`, `R1B-3`, `R1C-3`, `R1C-4` (all three claims), `R1C-5` (all seven twins and the rendered contradiction), `R1C-6`. That is every finding. NOTHING was judged on citation alone.

## Per-reviewer attribution, for the round record

| reviewer | model | lens | raw | valid |
| --- | --- | --- | --- | --- |
| A | `claude-opus-5` | citations, quotations, re-measurement | 8 | 5 |
| B | `claude-sonnet-5` | newly authored prose | 3 | 2 |
| C | `claude-opus-5` | completeness and the scope boundary | 6 | 5 |

Per-reviewer `valid_findings` credit each reviewer for its own raised finding, so the shared `R1A-2`/`R1C-1` and `R1A-3`/`R1C-2` are counted in BOTH A's 5 and C's 5 while counting ONCE in the round total of 10. That is the convention `AGENTS.md` states for the `reviewers` array, and the per-reviewer sum (12) is expected to exceed the round-level total (10).

C's count EXCLUDES `R1C-5`. If the human rules `R1C-5` in, C becomes 6 and the round total becomes 11.

## Deduplication, confirmed on evidence

PAIR 1: `R1A-2` and `R1C-1` ARE ONE FINDING. Both name sidecar `:206`, both quote the same sentence, both run the same command (`grep -rn 'serde(skip' src/`) and both reach the same three-clause refutation. The subject, the site, the evidence and the remedy are identical. `R1A-2` adds that the named site's own doc comment now says the opposite; `R1C-1` adds that the file contradicts itself at `:219` thirteen lines later. Those are two supporting observations on one defect, not two defects.

PAIR 2: `R1A-3` and `R1C-2` ARE ONE FINDING. Both name sidecar `:195`, both quote the same sentence, both refute both halves, and both cite the same two sites (`src/next.rs:192` having no skip, `src/main.rs:577` being the reason field). `R1A-3` argues the consistency point from `:189`, `:204`, `:208` and `:225`; `R1C-2` argues it from `:44`, `:46` and `:208`. Same defect, same remedy.

NO OTHER PAIR SURVIVES INSPECTION. `R1A-1` and `R1C-6` both concern `checks-runner-worktree-name-collision.md:55` and are NOT duplicates: `R1A-1` is about the second list in that sentence being short by two `{pid}-{nanos}` sites, `R1C-6` is about the first list's `src/checks.rs` citation having been re-pointed out of scope. Different claims, different remedies, opposite scope directions. They land in one edit region, which the fix pass should know.

---

## `R1A-1` (medium): VALID, fix required

REPRODUCED.

```
$ grep -rn 'as_nanos' tests/
tests/validate_toml_primary_skips_markdown_plan.rs:77
tests/validate_workflow_toml_source_needs_no_plan.rs:100
tests/validate_workflow_toml_source_needs_no_plan.rs:132
tests/validate_workflow_toml_source_needs_no_plan.rs:193
tests/validate_workflow_toml_source_needs_no_plan.rs:290
tests/metrics_and_ledger_anchor_to_the_plan_source.rs:68
tests/unsafe_pairings_are_refused_and_omitted.rs:90
```

SEVEN sites across FOUR files. The sentence the pass rewrote names FIVE across TWO. The two omitted helpers, opened at their ranges:

- `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:63-70`, `fn scratch(name)`, building `agent-scaffold-anchor-{name}-{pid}-{nanos}`.
- `tests/unsafe_pairings_are_refused_and_omitted.rs:85-92`, `fn scratch(name)`, building `agent-scaffold-containment-{name}-{pid}-{nanos}`.

Both files were created by THIS STEP:

```
$ git log --oneline --diff-filter=A -- tests/metrics_and_ledger_anchor_to_the_plan_source.rs tests/unsafe_pairings_are_refused_and_omitted.rs
8beb1c2 feat: refuse and omit on a round log or ledger the plan cannot vouch for   (inc2)
609ddcf fix: anchor the metrics log and the ledger to the plan source              (inc1)
```

The reviewer's second observation also holds and matters for the remedy: both omitted helpers discriminate by a per-test literal name AND by the clock, so they belong to NEITHER of the sentence's two lists. Simply appending them to the second list would make the first list's "rather than by the clock" false of them if a reader assigns them there, and would still leave the sentence's two-way partition wrong.

IN SCOPE. `Q-55-currencyscope` puts the inc1-induced citation drift in this sidecar in scope, and the drift here is drift THIS STEP created by adding two files. The pass moreover AUTHORED this enumeration in this diff: it deleted the stale count word "Three" and expanded the list from three entries to five, and the five it wrote are an under-count on the day it wrote them. That is the increment's own named failure mode ("a pass that re-tenses a false claim can write a NEW false claim in its place", sidecar `:308`) landing inside the pass.

SEVERITY medium CONFIRMED. The paragraph's conclusion survives, since `agent-scaffold-anchor-` and `agent-scaffold-containment-` are distinct literal prefixes and "they cannot collide today" holds over all seven. What earns medium rather than low is that the sentence functions as an exhaustiveness claim bounding the collision surface for the step that owns this file, and that the pass wrote the wrong bound itself.

MINIMAL REMEDY, TWO SHAPES, ONE OF THEM DELETION-CLASS.

- DELETION (preferred on this project's own recorded cure): delete the parenthetical enumeration of `{pid}-{nanos}` sites and keep the conclusion, which does not need the list. "Integration-test sites do use `{pid}-{nanos}`, but each carries a distinct literal prefix, so they cannot collide today." The count word "Three" was already deleted for being a count that moves; an explicit list is that same count in another spelling, and it moved for the same reason. A deleted claim cannot be falsified at an edge.
- AUTHORED (if the list is wanted): the sentence must be restructured into a three-way rather than a two-way split, because the two new helpers use a literal name AND the clock. This is composed prose, not an append, and it re-exposes the same falsification surface at the next test file this project adds.

## `R1A-2` + `R1C-1` (medium): VALID, fix required. ONE FINDING.

REPRODUCED.

```
$ grep -rn 'serde(skip' src/
src/next.rs:198:    #[serde(skip)]
src/next.rs:202:    #[serde(skip)]
```

Sidecar `:206` says, in unqualified present tense and untouched by the pass (confirmed: the diff has no hunk on that line):

> `#[serde(skip)]` appears exactly ONCE in the whole of `src/`, at `src/next.rs:NextProjection::no_active_loop_reason`, so there is no second silently-dropped field anywhere.

Three clauses, three refutations, opened at the lines in `src/next.rs`:

1. It appears TWICE, not once.
2. Neither occurrence is on `no_active_loop_reason`. That field is at `:192`, carries no attribute, and its own doc comment at `:189-191` reads "Serialised: `--json` is what an agent reads".
3. The two occurrences ARE second and third silently-dropped fields: `metrics_absent_note` (`:199`) and `resume_state_absent_note` (`:203`), both added by inc2.

The sentence's SECOND HALF is still TRUE and must stay: `grep -rn 'skip_serializing_if' src/` hits only `src/plan/source.rs` (10 sites), never `src/next.rs` or `src/main.rs`.

The file contradicts itself inside one section: `:219` says `no_active_loop_reason` is "RETYPED from `Option<String>` to a closed enum and NO LONGER `#[serde(skip)]`", thirteen lines below `:206`.

IN SCOPE, item (1) of `Q-55-currencyscope`, and it is the exact case acceptance check 21 promises does not remain.

SEVERITY medium CONFIRMED. Not higher: no behaviour is wrong, no gate is broken, and the correcting statement sits thirteen lines below. Not lower: a recorded NEGATIVE RESULT that has silently inverted is worse than a stale positive claim, because its whole function is to let a later reader skip a search.

MINIMAL REMEDY: DELETION. Delete the first clause through "anywhere", keeping the paragraph's opening and its `skip_serializing_if` half, which is true. The paragraph stays coherent as a negative result about `skip_serializing_if`. Nothing is authored. A re-tensing alternative exists ("APPEARED exactly ONCE at the time of the sweep") and would be historically true, but it preserves a claim falsifiable at an edge for no reader benefit, which is what this project's own round-3 deletion sweep ruled against.

## `R1A-3` + `R1C-2` (medium): VALID, fix required. ONE FINDING.

REPRODUCED. Sidecar `:195`, untouched by the pass (confirmed against the diff's changed-line list):

> `no_active_loop_reason` is `#[serde(skip)]` (`src/next.rs:NextProjection::no_active_loop_reason`) and `status`'s `Projection` has no reason field at all, so under `--json` an omitted part serialises as a bare `null` with nothing distinguishing why.

Both halves are false in the tree.

- First half: see the finding above. `src/next.rs:189-192`.
- Second half: `src/main.rs:575-577` carries `metrics_absent_reason: Option<next::MetricsAbsentReason>`.

Measured on the binary rather than read, in a fixture outside any repository:

```
$ agent-scaffold status --json --source docs/plans/p.plan.toml
{
  "plan": { "steps": [ { "slug": "only-step", "status": "not started" } ], "open_questions": [] },
  "metrics": { "records": 1 },
  "metrics_absent_reason": null
}
exit: 0
```

THE PARTIAL DEFENCE AND WHY IT DOES NOT CARRY. The paragraph opens "THE PROBLEM, in the form that decided it", so it is a decision-time framing. Both reviewers answer it the same way and the answer reproduces: the pass applied the OPPOSITE standard to every neighbouring paragraph of the same shape. I confirmed against the diff's changed-line list that the pass re-tensed `:44`, `:46`, `:102`, `:129`, `:139`, `:189`, `:204`, `:208` and `:225`, and that `:195` is the one paragraph in that run left in the present tense. `:208` is the closest parallel of all: same section, same decision-time role, and the pass converted "has no test" to "HAD no test" there. A standard applied to nine neighbours and withheld from the one whose present-tense content is flatly wrong is not a defensible line.

SEVERITY medium CONFIRMED, on the same reasoning as the finding above.

MINIMAL REMEDY: RE-TENSE, token-level, matching what the pass already did nine times in this file. "is `#[serde(skip)]`" becomes "WAS `#[serde(skip)]`" and "has no reason field at all" becomes "HAD no reason field at all". Two token substitutions. NOT authored prose: no new fact is introduced, and the surrounding sentence already carries its own decision-time framing.

## `R1A-4` (low): VALID, fix required

REPRODUCED. Neither quotation is in the tree:

```
$ grep -Fn 'Every part is optional so a missing plan or metrics file' src/main.rs   -> exit 1, no output
$ grep -Fn 'None` when the ledger is absent or carries no such section' src/next.rs -> exit 1, no output
```

The current texts, opened at their lines: `src/main.rs:561-567` reads "a missing plan, a missing metrics file, or a metrics file that cannot be paired with this plan"; `src/next.rs:184-186` reads "absent, carries no such section, or is not this plan's". Both defects the bullets describe are fixed.

Sidecar `:201` and `:202` keep present-tense verbs for that fixed state ("HAS THE SAME DEFECT", "IS SHORT BY ONE IN THE SAME WAY"), while `:199` and `:200` use prospective verbs that are correct as a specification of owed work ("BECOMES FALSE", "BECOMES INCOMPLETE"). The pass re-tensed `:204`, the fifth item of the same sweep, and stopped. Confirmed against the changed-line list: `:204` changed, `:199` to `:202` did not.

SEVERITY low CONFIRMED. A reader reaches the right facts either way, since the bullets are visibly a record of work now done, and the two quotations do not resolve so nobody can be misled about the current code.

MINIMAL REMEDY: DELETION is available and is the cheaper class. The four bullets specify work that inc2 completed and that acceptance checks 14a to 14h now pin, so deleting `:201` and `:202` removes two false present-tense claims and loses nothing a reader needs. If the record is wanted, RE-TENSE is token-level ("HAS" to "HAD", "IS SHORT BY ONE" to "WAS SHORT BY ONE"), matching `:204`. Either way no prose is authored.

## `R1A-5` (low): DISMISSED

REPRODUCED, and the reproduction is what dismisses it.

```
$ grep -Fn 'PathBuf::from(format!("docs/plans/{task}.ledger.md"))' src/main.rs
1544:        || PathBuf::from(format!("docs/plans/{task}.ledger.md")),
```

The fragment IS present, as the reviewer says. But the reviewer's own evidence answers its own objection. `src/main.rs:1535-1537`, the three lines DIRECTLY ABOVE the cited function, read: "With NEITHER a `--source` nor a `--plan` there is no directory to sit beside, so the historical current-directory-relative `docs/plans/<task>.ledger.md` stands, the same case in which the metrics default keeps its own historical path." The reviewer quotes this and then concludes that a reader "has to work out unaided" what the fragment now does. A reader who follows the citation lands on a function whose own doc comment explains the fallback in the sentence immediately preceding it. The unaided-reader premise does not survive its own citation.

The finding also concedes its subject claim: "the substantive claim (the function no longer assumes it in general) is true". Sidecar `:129` is a historical paragraph by construction ("CORRECTION TO THE FIRST PASS'S CLAIM ABOUT `default_ledger_path`. That pass said..."), and its past-tense sentence is TRUE of the pre-fix state. Acceptance check 21's standard is that every citation resolves and every quotation matches or correctly fails to match; this citation resolves and this quotation matches. Check 21 PASSES on this line.

The proposed remedy is an added clause on a true sentence, to forestall a misreading the cited site already prevents. That is the AUTHORED move this project has five retrospective and one prospective measurement against.

DISMISSED. Severity would be `low` in any case, so the high/critical backstop is not engaged.

## `R1A-6` (low): VALID, fix required

REPRODUCED on a purpose-built fixture (`docs/plans/p.plan.toml` from `PLAN_TOML`, a real one-record `docs/metrics/workflow.jsonl` from `ROUND_RECORD`), confirmed outside any repository (`git rev-parse --is-inside-work-tree` -> `fatal: not a git repository`).

Acceptance check 16 says, of BOTH `Err`-arm spellings:

> MEASURED at uid 1000 on both spellings: plain `validate --source docs/plans/p.plan.toml` exits 0 with `no metrics log at <the path as given>; nothing to validate`

Run literally for the trailing-slash spelling:

```
$ agent-scaffold validate --source docs/plans/p.plan.toml
docs/metrics/workflow.jsonl: 1 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
exit: 0
```

A GREEN RECORD COUNT, not the quoted absence note, because the trailing slash lives entirely in a `--metrics` the quoted command omits and the anchored default resolves to the real readable log. The command that does reproduce:

```
$ agent-scaffold validate --source docs/plans/p.plan.toml --metrics docs/metrics/workflow.jsonl/
no metrics log at docs/metrics/workflow.jsonl/; nothing to validate
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
exit: 0
```

CONTROL, so the finding is bounded correctly: for the MODE-600 spelling the quoted command line IS complete and DOES reproduce, because there the failure is a property of the fixture rather than of an argument.

```
$ chmod 600 docs/metrics
$ agent-scaffold validate --source docs/plans/p.plan.toml
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
exit: 0
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow
--workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked (Permission denied (os error 13)): ...
exit: 1
$ chmod 755 docs/metrics
drwxr-xr-x 2 jessea users 4096 docs/metrics        # restored
```

So the defect is exactly one cell wide, which is why it is `low` and not higher.

SEVERITY low CONFIRMED. The check's own earlier sentence names `--metrics docs/metrics/workflow.jsonl/` when it builds the fixture, and "on both spellings" plus "<the path as given>" carry the argument for a careful reader. It is a compression, not a false claim. It is still a finding because check 16 is the one cell the check's own framing tells a reviewer to settle by running a quoted command line, and that command line cannot be copied.

MINIMAL REMEDY: AUTHORED, four words. Insert the `--metrics` into the quoted command or split the sentence into the two command lines it is compressing. There is no deletion form that keeps the check's measurement.

## `R1A-7` (low): DISMISSED

REPRODUCED, all three routes, on a fixture outside any repository, all at exit 0:

```
$ agent-scaffold status --json --source docs/plans/p.plan.toml        # TOML-primary, no --plan
"plan": { "steps": [ { "slug": "only-step", ... } ], "open_questions": [] }

$ agent-scaffold status --json --source docs/plans/md.plan.toml       # Markdown-primary, no --plan
"plan": null

$ agent-scaffold status --json --source docs/plans/broken.plan.toml   # fails to parse, no --plan
note: --source docs/plans/broken.plan.toml did not parse as a `<task>.plan.toml`; projecting from --plan
"plan": null
```

The reproduction refutes the finding. THE TOOL'S OWN STDERR ON THE THIRD ROUTE NAMES THE CAUSE: "projecting from --plan", and there is no `--plan`. So the cause of `plan: null` on the parse-failure route IS a missing plan, exactly as the struct comment says. The `--source` being present and malformed is what sends the projection to the `--plan` route; it is not itself the cause of the null. The reviewer printed this note in its own evidence and did not follow it through, which is the adjudication failure mode of asking whether what remains looks right rather than what the change removed.

TWO INDEPENDENT GROUNDS BESIDES.

1. OUT OF THE SCOPE THE HUMAN CLOSED. `Q-55-currencyscope` names `Projection.plan`'s doc comment, which is the FIELD comment at `src/main.rs:570-571`. The pass corrected it and acceptance check 22 measures the correction, which I re-ran above as route 1. The STRUCT-level comment at `:561-567` is a different comment, is not on the closed list, and its current text was written by inc2 (it names "a metrics file that cannot be paired with this plan", which is inc2's containment case), so it is not stale drift this pass owed.
2. THE COMMENT'S ACTUAL CLAIM IS MEASURED TRUE. It promises "a partial projection rather than a failure", and all three routes give a partial projection at exit 0. The remedy would narrow an enumeration of causes, and this step's own human-authorised round-3 sweep ruled that affirmative exhaustiveness claims about derived output are DELETED rather than narrowed, on four data points inside this step.

DISMISSED. `low`, so no backstop.

## `R1A-8` (low): DISMISSED

REPRODUCED:

```
$ chmod 000 docs/plans/p.md
$ agent-scaffold status --json --plan docs/plans/p.md
Error: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
exit: 1
$ chmod 644 docs/plans/p.md
-rw-r--r-- 1 jessea users 139 docs/plans/p.md        # restored
```

The behaviour is real and the reviewer offers it as information rather than as a defect. It falsifies nothing in scope, and the reviewer says so itself: "The corrected sentence does not assert what an unreadable `--plan` does, so it is not falsified". It is pre-existing, it is not on any list `Q-55-currencyscope` closed in, and it is the same shape as the plain-`validate` mode-000 residual the ledger already routed to the validation-constraints step.

DISMISSED as a finding against this increment. RECORDED so the routing survives the deletion of these files: `status --json --plan <mode-000 file>` propagates a raw `Error: Os { code: 13, ... }` out of `run_status` from `src/main.rs:1211`'s `fs::read_to_string(path)?`, against `README.md:238`'s "Unlike `validate` it never fails". It belongs with the two pre-existing defects already routed to the validation-constraints step. `low`, so no backstop.

## `R1B-1` (medium): VALID, fix required

The reviewer's arithmetic REPRODUCES, and I ran the reconciliation it did not.

THE FIGURE UNDER REVIEW, sidecar `:308` (newly authored by this pass), rendered at `docs/plans/agent-scaffold.md:1703`:

> Inc1 OF THIS VERY STEP spent THREE rounds and TWENTY valid findings, EVERY ONE an inaccurate description of correct behaviour and zero defects in what the code does.

THE ROUND RECORDS:

```
$ jq -r 'select(.type=="round") | [.task, (.valid_findings|tostring)] | @tsv' docs/metrics/workflow.jsonl | grep inc1
workflow-enforcement-tier-inc1   3
workflow-enforcement-tier-inc1   4
workflow-enforcement-tier-inc1   6
```

3 + 4 + 6 = 13. `grep -n '"workflow-enforcement-tier-inc1"' docs/metrics/workflow.jsonl` returns exactly lines 246 to 249: three `round` records and one `escalation`, and no `dismissal_recheck`. Summing the `reviewers` sub-arrays instead gives 3 + 4 + 7 = 14. There is no tally in the log that reaches 20.

THE OUT-OF-SCOPE RECONCILIATION, TESTED RATHER THAN ASSUMED. The hypothesis is that out-of-scope valid findings were deliberately excluded from `valid_findings`, so the ledger narrative and the round log would legitimately disagree. THE HYPOTHESIS IS CORRECT ABOUT THE CONVENTION AND DOES NOT REACH 20. The ledger's own round-3 record (`docs/plans/agent-scaffold.ledger.md:873`) reads: "7 raw, 7 valid, 0 dismissed, ONE RULED OUT OF SCOPE leaving 6 in scope", and the round record carries 6. So `valid_findings` IS the in-scope count, exactly as the hypothesis predicts. But inc1 had ONE out-of-scope finding in three rounds (`W3A-1`, confirmed in the round-3 triage recoverable at `bb3d10f~1`), so the adjudicated-valid total is 14. The exclusion explains a gap of ONE, not of SEVEN.

THE CONTROL THE REVIEWER DID NOT RUN, AND IT SETTLES WHAT THE SENTENCE'S OWN WORDING CLAIMS. This project has three sibling waiver notes using the identical phrase, and two of them match the log exactly:

- `docs/plans/agent-scaffold.plan.toml:1339` (`-w2`): "Four work-review rounds, 24 valid findings (9, 5, 6, 4)". Log: 9, 5, 6, 4. EXACT.
- `docs/plans/agent-scaffold.plan.toml:1348` (`-w3`): "Five work-review rounds, 14 valid findings (6, 4, 2, 0, 2)". Log: 6, 4, 2, 0, 2. EXACT.
- `docs/plans/agent-scaffold.plan.toml:1330` (`-w1`): "Three work-review rounds, 20 valid findings". Log: 3, 4, 6 = 13. DOES NOT MATCH.

And within the sentence under review itself, the OTHER figure is the round-record sum: "Step 92 (`prompt-drift-guard`) spent SIX rounds and FIFTEEN valid findings" reconciles exactly (4+3+5+1+2+0 = 15 across six round records) and is independently corroborated at `docs/plans/agent-scaffold.md:322`. So the sentence uses "valid findings" in the round-record sense for one of its two figures. That is what its own wording claims, and in that sense the answer is 13.

CONCLUSION: 20 is unreconcilable with any tally this repository holds. Under the round-record reading it is 13; under the adjudicated-valid reading it is 14. The finding is VALID either way, which is why it does not turn on which reading is preferred.

THE THIRD SITE, WHICH THE BRIEF ASKED ABOUT: YES. `docs/plans/agent-scaffold.plan.toml:1330`, the `workflow-enforcement-tier-w1` waiver note, carries the same claim and RENDERS at `docs/plans/agent-scaffold.md:324`. Full census of the claim: FOUR source sites, TWO rendered mirrors.

| site | class | in inc4's closed scope |
| --- | --- | --- |
| `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:308` | newly authored by this pass | YES |
| `docs/plans/agent-scaffold.plan.toml:1330` (`-w1` waiver note) | decision receipt, pre-existing | NO, see `R1C-5` |
| `docs/plans/agent-scaffold.ledger.md:535` | orchestrator narrative | NO |
| `docs/plans/agent-scaffold.ledger.md:885` | orchestrator narrative (the escalation record) | NO |

Rendered mirrors: `docs/plans/agent-scaffold.md:1703` (of the sidecar) and `:324` (of the waiver note).

RECORDED, NOT ASSERTED, as the likely contamination source: the ledger paragraph immediately preceding the escalation record (`:883`) says of the citation conversion that "about TWENTY OF THEM WERE ALREADY WRONG", a count of stale citations, not of valid findings.

SEVERITY medium CONFIRMED. The figure is wrong by 54 percent, it sits in the paragraph whose job is to justify a `risky` classification on measured calibration data, and it is in an increment whose whole subject is claim currency. The classification itself does not fall: the distinguishing property the argument actually rests on ("EVERY ONE an inaccurate description of correct behaviour and zero defects in what the code does") is true at 13 as at 20, and 13 is not even this step's worst round total (inc2 scored 24). Not `high` because nothing downstream computes on the number.

MINIMAL REMEDY: TOKEN SUBSTITUTION, "TWENTY" -> "13", at sidecar `:308`, then re-render. No prose is authored and no sentence is restructured, so this re-seeds nothing.

TWIN-SITE WARNING FOR THE ORCHESTRATOR. Correcting only `:308` leaves the plan TOML's waiver note and both ledger paragraphs saying 20, which manufactures a fresh disagreement between this project's own artifacts, and rendered `docs/plans/agent-scaffold.md` would then carry BOTH figures, at `:324` and `:1703`. That is the twin-site failure mode this task has been bitten by four times. The waiver-note site is a decision receipt of the same class as `R1C-5`'s subject, so it should ride with `R1C-5`'s human decision rather than being settled separately.

## `R1B-2` (low): VALID, fix required

REPRODUCED.

```
$ grep -F "not a validation failure ... a missing file prints a note" src/main.rs                     -> exit 1
$ git show 6b1c847~1:src/main.rs | grep -F "not a validation failure ... a missing file prints a note" -> exit 1
```

The elision is the plan author's, not the source's, so the literal search cannot succeed against ANY revision. The reviewer's uniqueness claim also reproduces: a search for a mid-quote ellipsis inside double quotes across the whole file returns exactly ONE hit, at `:374`.

```
$ grep -cn '"[^"]* \.\.\. [^"]*"' docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
1
```

WHAT SURVIVES AND WHAT DOES NOT. Check 21's PROCEDURE is not defective: its no-match branch ("a quotation with no match in the tree is either RE-TENSED ... or DELETED") reaches the right answer here, since `:374` is correctly re-tensed ("which STATED the superseded policy"). The reviewer concedes this. What IS falsified is check 21's affirmative claim that "The check is mechanical rather than a reading", which is untrue for this quotation and, on inspection, for every re-tensed quotation, because the no-match branch requires reading the sentence for tense.

SEVERITY low CONFIRMED. No outcome changes, on this quotation or any other. It is an inaccurate claim in prose this pass authored, which is the increment's own subject, which is why it is not dismissed.

MINIMAL REMEDY: DELETION. Strike "The check is mechanical rather than a reading." from check 21 at `:345`. The instruction it restates is already carried by the imperative that precedes it ("run each quoted fragment ... as a literal search"), so the deletion removes an overstatement and loses no instruction. This is the class this project measures as re-seeding nothing.

## `R1B-3` (low): DISMISSED, the demonstration does not reproduce

THE LITERAL-OVERLAP HALF REPRODUCES. Check 23's two commands do appear verbatim in earlier checks: check 1 at `:316` carries `cargo run -- render docs/plans/agent-scaffold.plan.toml --check`, and check 9 at `:324` carries `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` with the same expected `workflow invariants hold`.

THE LOAD-BEARING PREMISE DOES NOT. The finding rests on "neither command exercises anything inc4 changed: inc4's only source change is a doc comment on an unrelated struct field, and its sidecar edits do not touch `docs/plans/agent-scaffold.plan.toml`'s renderable content". I RAN THE CONTROL the finding did not, on a scratch copy of `docs/` outside the worktree.

```
=== CONTROL: unmutated copy ===
$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
exit: 0

=== MUTATION: edit ONE sidecar line, leave the rendered view untouched ===
(workflow-enforcement-tier.md: "appears exactly ONCE" -> "appears exactly TWICE")

$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
warning: docs/plans/agent-scaffold.md differs from a fresh render (a hand-edit, or a stale
render after a source edit) (first difference at line 1601: ...); re-render with
`agent-scaffold render docs/plans/agent-scaffold.plan.toml`
exit: 0
```

The sidecars ARE renderable content: they reach `docs/plans/agent-scaffold.md` through `[meta].sidecars`, and inc4 changed four of them plus the rendered view. `render --check` goes red on exactly the failure mode a documentation-currency pass has, which is a sidecar edit not carried into the rendered view. The premise that the commands exercise nothing inc4 changed is false.

CHECK 23 ALSO CARRIES INFORMATION CHECK 1 DOES NOT. `render --check` reports the divergence as a WARNING at EXIT 0, measured above. Check 1 states the command with no expected output; check 23 states the expected OUTPUT ("reports up to date"). Given the exit code does not move, that difference is load-bearing: a round reading only exit codes passes a stale render under check 1 and fails it under check 23.

The check-9 half of the finding stands on its own terms (check 23's validate half is a subset of check 9's assertion), but that half alone does not support the finding's conclusion that check 23 "adds nothing".

DISMISSED under the standing rule that a testable claim whose demonstration does not reproduce is dismissed. `low`, so no backstop. Recorded for the orchestrator, not as a finding: the `render --check` warning-at-exit-0 behaviour is a separate observation about the tool, out of this increment's scope, and I raise no finding on it.

## `R1C-3` (medium): VALID, fix required

REPRODUCED. Sidecar `:304`, present tense, untouched by the pass:

> ... and one of the two commands (`status --json`) has no test on its serialisation at all, so that half is carried by the acceptance check rather than by the suite.

```
$ grep -n '"status", "--json"' tests/unsafe_pairings_are_refused_and_omitted.rs
403:  vec!["status", "--json", "--source", &away_plan, "--metrics", local],
764:  &["status", "--json", "--source", &missing, "--metrics", "docs/metrics/workflow.jsonl"],
1404: &["status", "--json", "--source", &away_plan, "--metrics", "docs/metrics/workflow.jsonl"],
1425: let (code, stdout, stderr) = run(&home, &["status", "--json", "--source", &away_plan]);
1586: run(&home, &["status", "--json", "--source", &alpha_source, "--plan", &beta_plan]);
1738: run(&home, &["status", "--json", "--source", "docs/plans/p.plan.toml"]);
```

The six are the same six the ledger records the orchestrator counting when it raised `Q-55-twinsites`. I opened two of them to confirm they assert on the SERIALISATION and not merely on the exit code:

```
1427: assert!(stdout.contains("\"metrics_absent_reason\": \"log-absent\""), "stdout:\n{stdout}");
1740: assert!(stdout.contains("\"metrics_absent_reason\": null"), "stdout:\n{stdout}");
```

THE THIRD-SITE CLAIM HOLDS AND THE DIFF PROVES IT. The pass fixed the two twins the human ruled on, in this same diff: `tests/unsafe_pairings_are_refused_and_omitted.rs:1370` lost "and no test on its serialisation at all", and sidecar `:208` became "has NO golden, and HAD no test on its serialisation at all". Sidecar `:304` carries the identical claim in the present tense, ninety-six lines below `:208` in the file the pass was sweeping.

IN SCOPE. `Q-55-twinsites` was ruled in on the round-3 triager's condition 3, "a stale claim THE INCREMENT'S OWN CHANGE BROKE is in scope regardless of authorship", and inc2 is what added the `status --json` assertions. The same condition reaches the third site by the same route.

SEVERITY medium CONFIRMED. The sidecar now says opposite things at `:208` and `:304` about the same fact, and this is the fourth occurrence of the recorded twin-site failure mode, on a claim the human ruled on hours before the pass ran. Not `high`: it is a claim in a closed increment's risk paragraph, nothing acts on it.

MINIMAL REMEDY: DELETION, and the human has ALREADY PRESCRIBED THIS EXACT REMEDY CLASS for this exact claim. `Q-55-twinsites` decided "FIX BOTH TWINS, DELETION ONLY" and recorded "Both remedies are PURE DELETIONS, which is the class this project measures as re-seeding nothing." Delete "and one of the two commands (`status --json`) has no test on its serialisation at all, so that half is carried by the acceptance check rather than by the suite" from `:304`. The sentence's preceding clause about the JSON contract change and the broken golden stands alone.

## `R1C-4` (medium): VALID, fix required. ALL THREE CLAIMS REPRODUCED.

THE PREMISE HOLDS. `git diff main..HEAD -U0` on the sidecar has NO hunk between `:226` and `:272`, so the section `## The four accepted costs` (`:251-263`) was never opened by this pass. Confirmed against the hunk map, not asserted.

(a) `:255`, cost (i): "A BARE FILENAME RUN FROM INSIDE `docs/plans` REMAINS A SILENT MISS."

```
$ cd docs/plans && agent-scaffold validate --source agent-scaffold.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run,
so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record
the project's review rounds there
exit: 1
```

A LOUD FAILURE, not a silent miss. The paragraph's own closing sentence predicts this in the FUTURE tense ("After the tier policy lands, this case becomes a HARD FAILURE"), and the tier policy landed at `3d00341`. Acceptance check 18 states it correctly ("After inc3: a HARD FAILURE naming the path it looked for"), so the file's specification and its prose disagree.

(b) `:257`, cost (ii): "This is a genuine new failure for a layout that works today." REPRODUCED on a purpose-built symlink layout (`<root>/docs/plans` a symlink to `<root>/elsewhere`), which the reviewer argued rather than ran:

```
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow
--workflow would join docs/plans/p.plan.toml against docs/metrics/workflow.jsonl, which is not under
the plan's project root .../cost2/elsewhere; pass a `--metrics` under that root, ...
exit: 1

$ agent-scaffold status --json --source docs/plans/p.plan.toml
{ "plan": { ... }, "metrics": null, "metrics_absent_reason": "log-not-this-project" }
exit: 0
```

The layout does not work today; it is refused, loudly on `validate --workflow` and quietly on `status`, which is what check 19 pins.

(c) `:259`, cost (iii): "`--source <root>/docs/plans/x.plan.toml --plan <root>/notes/p.md` greens today against `<root>/docs/metrics/workflow.jsonl`". REPRODUCED on a purpose-built two-substrate fixture:

```
$ agent-scaffold validate --source docs/plans/x.plan.toml --plan notes/p.md --workflow
--workflow would join notes/p.md against docs/metrics/workflow.jsonl, which is not under the plan's
project root .../cost3/notes; pass a `--metrics` under that root, ...
exit: 1
```

Exit 1, not green. Check 19b already states the correct pair.

Cost (iv) at `:261` is present tense and TRUE post-inc2; the reviewer is right to leave it.

IN SCOPE. All three are present-tense sidecar claims about the tool that this step's own increments falsified, which is item (1) of `Q-55-currencyscope` verbatim.

SEVERITY medium CONFIRMED for the compound. Taken alone (b) and (c) are `low`, because each paragraph's next clause states the correct post-change behaviour. (a) carries the medium on its own: its capitalised headline claim is INVERTED (a silent miss where the tool now hard-fails), and the section's declared purpose is instruction to future readers, "an implementer must NOT 'fix' them, and a reviewer must NOT raise them as defects". A reader sent to verify an accepted cost and finding the opposite behaviour is the failure this section exists to prevent.

MINIMAL REMEDY: RE-TENSE, token-level, three sentences. No new fact needs authoring, because checks 18, 19 and 19b already state the current behaviour and the paragraphs already carry the post-change clause. (a) "REMAINS A SILENT MISS" -> "WAS A SILENT MISS" and "becomes a HARD FAILURE" -> "became a HARD FAILURE"; (b) "a layout that works today" -> "a layout that worked before inc2"; (c) "greens today" -> "greened before inc2". Not deletion class, but not authored prose either: every substitution is a verb tense on an existing clause.

## `R1C-5`: FACTS ESTABLISHED, HUMAN DECISION OWED. I do not settle the policy question and neither should the orchestrator.

The reviewer declined to rule whether a decision record is frozen history. I decline too, and by design: it is a scope question of the same shape as `Q-55-twinsites`, which the human decided rather than a writer. Below is only what I measured.

### FACT 1: the self-contradiction in the rendered view is CONFIRMED. Both lines, quoted with their line numbers.

`docs/plans/agent-scaffold.md:168`, rendered from the `Q-55` question record's `ask` at `docs/plans/agent-scaffold.plan.toml:1734`:

> THE MACHINE SURFACE (human, 2026-07-31, receipt `q_id:"Q-55-jsonreason"`): ADD A SERIALISED REASON ... The planner raised this rather than assuming it: `no_active_loop_reason` is `#[serde(skip)]` at `src/next.rs:NextProjection::no_active_loop_reason` and `status`'s projection has no reason field ...

`docs/plans/agent-scaffold.md:1614`, rendered from sidecar `:219`:

> `no_active_loop_reason`, on `NextProjection`, RETYPED from `Option<String>` to a closed enum and NO LONGER `#[serde(skip)]`.

And `docs/plans/agent-scaffold.md:1609`, rendered from sidecar `:214`, specifies `metrics_absent_reason` "on BOTH `NextProjection` and `status`'s `Projection`". The tree settles which side is right: `src/next.rs:192` carries no attribute, and `src/main.rs:577` is the reason field.

CONFIRMED. `render --check` reports "up to date" at exit 0, so nothing mechanical catches it: the generated file faithfully renders a source that disagrees with itself.

### FACT 2, WHICH THE REVIEWER DID NOT ESTABLISH AND WHICH DECIDES HOW SEPARABLE THIS IS. Fixing the in-scope findings does NOT remove the contradiction.

`docs/plans/agent-scaffold.md` currently carries THREE lines contradicting `:1614`, from TWO different sources:

| rendered line | source | in inc4's closed scope |
| --- | --- | --- |
| `:1590` | sidecar `:195` | YES, this is `R1A-3`/`R1C-2` |
| `:1601` | sidecar `:206` | YES, this is `R1A-2`/`R1C-1` |
| `:168` | plan TOML `:1734`, the `Q-55` `ask` | NO, this is the open question |

So the in-scope fix pass removes two of the three. `:168` survives it, and the rendered view still contradicts itself afterwards. The plan TOML question is genuinely separable and does not resolve as a side effect.

### FACT 3: the twin count is SEVEN, across SIX lines of the `Q-55` `ask`. Verified individually.

| # | plan TOML line | the claim | why false now, measured |
| --- | --- | --- | --- |
| 1 | `:1734` | `no_active_loop_reason` is `#[serde(skip)]`; `status`'s projection has no reason field | `src/next.rs:192` carries no attribute; `src/main.rs:577` is `metrics_absent_reason` |
| 2 | `:1732` | "`README.md:228` says 'Unlike `validate` it never fails on a missing or malformed file'" | `README.md:228` is now a comment line inside a code fence (`# --workflow would join ...`); the sentence is at `:238`. The pass corrected this same citation to `:238` at sidecar `:173` |
| 3 | `:1722` | the "`--workflow has a plan source but the metrics log is missing; skipping the workflow check`" note in `src/main.rs:run_validate` | `grep -rn "skipping the workflow check" src/` returns NOTHING; the only tree hit is a historical test doc comment at `tests/validate_workflow_toml_source_needs_no_plan.rs:178` |
| 4 | `:1724` | "The metrics-log path resolves against the CURRENT WORKING DIRECTORY (`src/main.rs:ValidateArgs::metrics`)" | `src/main.rs:431` is `metrics: Option<PathBuf>` with no default; `resolve_metrics_path` at `:1344` anchors it |
| 5 | `:1728` | `status`, `next` and the derived ledger path "carry the identical CWD-relative defect", and `next` emits `state: converged` at exit 0 | closed by inc1 and inc2; checks 5 and 14b pin it, and the suite asserts it |
| 6 | `:1728` | a non-instrumented user "reads an unconditional promise of the `validate --workflow` backstop (`pack/AGENTS.md:93`)" | `pack/AGENTS.md:93` now reads "when instrumentation is on, the deterministic `validate --workflow` check is the backstop ... and on a project with no round log yet, which every project scaffolded without `--instrument` remains, that check exits non-zero reporting that it could not run rather than passing". The unqualified form is absent (`grep -F`, exit 1) |
| 7 | `:1736` | "a BARE FILENAME run from inside `docs/plans` remains a silent miss" | reproduced under `R1C-4`(a): exit 1, a hard failure |

SEVEN twins. I verified SIX directly by command; the seventh (row 5) rests on the acceptance checks and the passing suite rather than on a hand-built multi-project fixture, and I say so rather than claiming a run I did not make.

The `Q-55` record spans `docs/plans/agent-scaffold.plan.toml:1713-1736` and renders at `docs/plans/agent-scaffold.md:152-170`, both confirmed.

RELATED, AND IT RIDES WITH THIS DECISION: `R1B-1`'s "20 valid findings" has a twin at `docs/plans/agent-scaffold.plan.toml:1330`, the `workflow-enforcement-tier-w1` waiver note, which renders at `docs/plans/agent-scaffold.md:324`. A waiver note is a decision receipt of the same class as a question `ask`, so whichever way the human rules on the `Q-55` record governs it too. That makes the decision cover EIGHT twin claims, not seven.

### FACT 4: this project's OWN recorded convention, quoted.

`docs/plans/agent-scaffold.ledger.md:695`:

> ... left because it is a decision record of what was decided AT THE TIME, and this project's convention, the one the `EX-3` ledger prescription just established, is to APPEND a correction rather than rewrite a decision record.

That paragraph records the inc4 PLANNER declining to touch `Q-55-noconvention`'s "TWO ACCEPTED COSTS" claim on exactly this ground, and the orchestrator recorded it as a judgement "a round-2 reviewer should CHECK rather than accept". So the convention exists, it was applied by this very pass, and it is on the table for checking.

THE CONVENTION HAS A PRECEDENT INSIDE THE `Q-55` RECORD ITSELF. `:1722` opens "CORRECTION TO THIS ITEM'S OWN WORDING, recorded because the reproduction contradicted it", and `:1724` opens "A SECOND DEFECT, NOT IN THE TEXT ABOVE AND WORSE". Both are appended corrections to earlier paragraphs of the same `ask`, added without rewriting what they correct. The append form is not hypothetical here; it is the record's own established shape.

THE COUNTER-GROUND, stated so the human sees both. The round-3 triager's condition 3, which `Q-55-twinsites` was ruled in on, is "a stale claim THE INCREMENT'S OWN CHANGE BROKE is in scope regardless of authorship". All seven twins were falsified by THIS STEP's own increments, not by anything predating it, so condition 3 reaches them on its face. `Q-55-currencyscope` named four sidecars and did not name the plan TOML, so including them would widen a closed scope, which is why `Q-55-twinsites` was PUT rather than assumed.

### THE OPTIONS AND WHAT EACH COSTS. No recommendation; the orchestrator puts this to the human.

1. APPEND A CORRECTION PARAGRAPH to the `Q-55` `ask` (and to the `-w1` waiver note). COST: one authored paragraph, which is the class this project has six measurements against for manufacturing the next round's finding, and the paragraph must enumerate seven or eight corrections without asserting a count that moves. The rendered view then carries BOTH the false claims and their correction, so `:168` still reads false in isolation to a reader who stops there. BENEFIT: the receipt still reads as written at decision time, and the convention is applied rather than broken.
2. REWRITE THE SEVEN (OR EIGHT) TWINS IN PLACE, re-tensing each. COST: breaks the recorded convention and destroys the decision-time wording of a human decision receipt, which is the thing the convention exists to protect; the edits are token-level re-tensings, so the authoring risk is low. BENEFIT: the rendered view stops contradicting itself, and no reader meets a false claim at all.
3. LEAVE IT, RECORD IT AS A RESIDUAL. COST: the generated view ships self-contradictory on a fact, and the step closes with `docs/plans/agent-scaffold.md` asserting at `:168` what it denies at `:1614`; a future currency pass inherits all eight. BENEFIT: zero writing, zero re-seeding, and the scope the human closed stays closed.
4. SPLIT: fix only the twins whose falsity is a CITATION rather than a claim (rows 2 and 6 point at line numbers and a file that moved). COST: an arbitrary line that leaves the self-contradiction standing, since row 1 is the contradicting claim. BENEFIT: smallest possible edit.

IF THE HUMAN RULES IT IN, this becomes a `medium` valid finding and the round total goes from 10 to 11; reviewer C's `valid_findings` goes from 5 to 6. The round is NEW-VALID either way.

## `R1C-6` (low): VALID, fix required

REPRODUCED, on the dates the finding turns on.

```
$ git log -1 --format="%h %ad %s" --date=short 09a027c -- src/checks.rs
09a027c 2026-07-31 test(checks): pin claim_dir's own error arm and delete two false claim-outcome sentences
$ git log -1 --format="%h %ad %s" --date=short 609ddcf
609ddcf 2026-08-01 fix: anchor the metrics log and the ledger to the plan source
```

`src/checks.rs` has not been touched since 2026-07-31; this step's FIRST source commit is 2026-08-01. No increment of `workflow-enforcement-tier` moved a line in that file, so the `src/checks.rs:862-871` -> `:1037-1046` re-point at `checks-runner-worktree-name-collision.md:55` repairs drift with ANOTHER CAUSE, the owning step at order 93.

Acceptance check 21b, added by this same commit, says the opposite of what the commit did: "every `src/main.rs` and `tests/` citation in the three is opened at its cited range and shown to hold its named subject", "AND ONLY THOSE", and then routes the `src/checks.rs` citations to the owning step because "pulling it in would widen a scope the human closed (`Q-55-currencyscope`)".

THE EDIT IS HARMLESS IN EFFECT, confirmed: `src/checks.rs:1037-1046` IS `fn scratch(name)` building `agent-scaffold-checks-test-{pid}-{name}`, the correct subject, and `test-tmpdir-repo-assumption.md:35` already cited that exact range for that exact helper BEFORE this pass (it is context in the diff, not a changed line). The sibling `src/checks.rs:400-405` was correctly left alone; it now resolves to `fn git`, its `owning_pid` subject having been replaced.

SEVERITY low CONFIRMED, on the harmlessness. It is still a finding because it makes check 21b's own "AND ONLY THOSE" untrue of the commit that introduced it, and because a scope the human closed was crossed without disclosure.

A DISTINCTION THE REMEDY SHOULD USE, which neither the check nor the finding draws. There are TWO classes of `src/checks.rs` citation in that file, and check 21b's exclusion reasoning describes only the second: citations whose SUBJECT MOVED (`:862-871`, `fn scratch`, which still exists) and citations whose SUBJECT WAS REPLACED (`:400-405`, `owning_pid`, which no longer exists). Check 21b's stated ground is "point at code the fix deliberately replaced ... several of its named subjects no longer exist at all", which is true of the second class and false of the first.

MINIMAL REMEDY, TWO OPTIONS, AND I DO NOT CHOOSE BETWEEN THEM because it is a scope-boundary call the orchestrator owns.

- REVERT the one citation to `src/checks.rs:862-871`. DELETION-class (a revert authors nothing), but it restores a citation that is stale for another reason, so the file gets worse for a reader in exchange for the boundary holding.
- NARROW check 21b's exclusion clause to the class it actually describes, admitting the moved-subject citation. AUTHORED, one clause, in an acceptance check this pass wrote.

## Mechanical gates, run first-hand in this worktree

```
$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date                                    exit 0

$ agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 286 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold   exit 0
```

Both re-run on a scratch copy of `docs/` as the `R1B-3` control, so the "up to date" result is a measured green and not an inherited one.

```
$ cargo test          # TMPDIR outside any repository, per the acceptance check's preamble
cargo test exit: 0
378 + 20 + 9 + 5 + 4 + 3 + 1 + 1 + 1 passed, 0 failed, across nine binaries
```

The exit code was captured directly rather than through a pipe, because `tail` would have reported its own status and masked a failure.

## What this triage varied, and what it held fixed

VARIED: probe failure class (EACCES from a mode-600 ancestor, ENOTDIR from a trailing slash, EACCES on a mode-000 plan file, and the healthy control); `--metrics` explicit and anchored-default; `--source` kind (TOML-primary, Markdown-primary, parse-failure); `--plan` state (absent, readable, mode-000); layout (conventional, symlinked `docs/plans`, plan outside any `docs/plans`); working directory (project root and inside `docs/plans`); a live mutation of a sidecar against `render --check`.

HELD FIXED: one platform (Linux, local filesystem), one build profile (debug), one binary (`caeab43`), uid 1000 only. I did NOT re-run reviewer A's uid-0 cells under `unshare -Ur`, because no finding turns on them and reviewer A's part C reports them clean; a uid-dependent defect in check 16's root cell would survive this triage. I ran no concurrency or TOCTOU case. I re-raised none of inc2's four or inc3's four recorded residuals, and I checked each finding against that list before ruling: none of the fifteen is a re-raise.

I did NOT re-derive the whole citation sweep or the whole completeness sweep. My job was the seventeen findings, and a defect no reviewer raised would survive this round.
