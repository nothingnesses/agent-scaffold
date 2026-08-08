# `workflow-enforcement-tier-inc4`, round 3: triage

Triager worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-inc4-r3`, branch `triage/wet-inc4-r3`, at `cb5e6ac`. `git diff --stat 93ee357 HEAD` returns only the three round-3 findings files, so this tree is byte-identical to the tree all three reviewers judged and every sidecar line number below is theirs unchanged.

All fixtures under `<scratchpad>/triage-inc4-r3/` only. Nothing written to bare `/tmp`, nothing deleted outside that subdirectory, no `chmod` used anywhere so none is owed a restore. `git status --short` in this worktree prints nothing but this file. The plan TOML and log mutations below were run against COPIES of `docs/` in the scratchpad, never against the worktree.

## Counts

| quantity | value |
| --- | --- |
| RAW findings | 10 |
| DEDUPLICATED | 10 (no duplicates across the three lenses) |
| VALID | 10 |
| IN-SCOPE VALID | 6 |
| OUT-OF-SCOPE VALID | 4 |
| DISMISSED | 0 |
| dismissed at `high` or above | 0, SO NO BACKSTOP RE-CHECK IS OWED |

Severity distribution after correction: `medium` 7, `low` 3, `high` 0, `critical` 0. I changed no severity in either direction; every reviewer rating survived its ground. I corrected four factual characterisations, listed under "Corrections to the reviewers" below, none of which moves a severity.

Per reviewer:

| lens | file | raw | valid | in scope | out of scope |
| --- | --- | --- | --- | --- | --- |
| historical-truth (sonnet) | `...-r3-historical-sonnet.md` | 0 | 0 | 0 | 0 |
| still-true (opus) | `...-r3-stilltrue-opus.md` | 5 | 5 | 5 | 0 |
| detectability (opus) | `...-r3-detectability-opus.md` | 5 | 5 | 1 | 4 |

REPRODUCED FIRST-HAND: `R3B-1`, `R3B-2`, `R3B-3`, `R3B-4`, `R3B-5`, `R3C-2`, `R3C-3`, `R3C-4`, `R3C-5`. Nine of ten in full. `R3C-1` is REPRODUCED IN PART: I re-ran the baseline gate set, `M1` in both render states, `M2`, `PC1`, `PC2` and the zero-mutation corroboration, and I verified the mechanism the other four mutations rest on (no test or fixture reads the step sidecars at all, `w5_problems` never reads `waiver.note`, `render --check` is a copy comparison). I did not re-run `M3`, `M4`, `M6` or `M7`, and I say so rather than implying a fuller reproduction than I ran. NOTHING was judged on citation alone.

THE ROUND IS NEW-VALID, AND NEW-VALID ON IN-SCOPE FINDINGS ALONE: six in-scope valid findings. The streak stays 0 of 2, round 3 of a cap of 5.

## Verdict table

| id | reviewer severity | final severity | scope | verdict | remedy class |
| --- | --- | --- | --- | --- | --- |
| `R3B-1` | medium | medium (confirmed) | IN SCOPE | VALID, fix required | DELETION (two sites) |
| `R3B-2` | medium | medium (confirmed) | IN SCOPE | VALID, fix required | TOKEN |
| `R3B-3` | medium | medium (confirmed) | IN SCOPE | VALID, fix required | TOKEN (two sites) |
| `R3B-4` | low | low (confirmed) | IN SCOPE | VALID, fix required | TOKEN |
| `R3B-5` | low | low (confirmed) | IN SCOPE | VALID, fix required | DELETION (one word) |
| `R3C-1` | medium | medium (confirmed) | OUT OF SCOPE | VALID, backlog step, minimal fix recorded | new mechanism |
| `R3C-2` | medium | medium (confirmed) | OUT OF SCOPE | VALID, backlog step, minimal fix recorded | new mechanism |
| `R3C-3` | medium | medium (confirmed) | OUT OF SCOPE | VALID, backlog step, minimal fix recorded | new mechanism |
| `R3C-4` | medium | medium (confirmed) | IN SCOPE | VALID, fix required | AUTHORED PROSE (two clauses) |
| `R3C-5` | low | low (confirmed) | OUT OF SCOPE | VALID, process note, minimal fix recorded | AUTHORED PROSE (one line, outside the plan) |

## The zero-finding lens, read first

`...-r3-historical-sonnet.md` reports NO findings and its stated coverage is what makes that worth anything, so I sampled it rather than accepting it.

I re-derived all three waiver figures independently from `docs/metrics/workflow.jsonl`, extracting only the TOP-LEVEL `valid_findings` (the `reviewers` array carries its own per-reviewer `valid_findings`, which is how a naive grep over-counts; inc1 returns nine numbers that way and three the right way):

```
workflow-enforcement-tier-inc1 per round: 3 4 6                (sum 13)
workflow-enforcement-tier-inc2 per round: 9 5 6 4              (sum 24)  ceilings high, high, medium, high
workflow-enforcement-tier-inc3 per round: 6 4 2 0 2            (sum 14)  ceilings medium, medium, low, none, medium
workflow-enforcement-tier-inc4 per round: 11 9
```

Every figure the three waiver notes claim matches, exactly as the lens reports. The inc4 line also confirms the "twenty in-scope valid findings" that `R3C-1` is measured against (11 + 9).

The trap the lens records is real and I needed it: the fast-forward merges' immediate parents are mid-branch commits from the same rebased lineage, not pre-increment baselines. Where I touched history (`R3B-1`'s and `R3B-3`'s provenance, `R3C-4`'s provenance) I used `git log -S` and `git log -L` against the file rather than a merge parent, which sidesteps it.

The lens's one self-reported non-finding, that `:182`'s "measured" parenthetical pairs a real measurement with a deduced counterfactual, is a precision-of-attribution point and I agree it is below the threshold. I raise nothing on it.

## `R3B-1` (medium confirmed): VALID, fix required. IN SCOPE.

REPRODUCED. The fixture, run from this worktree's root:

```
$ ./target/debug/agent-scaffold status --json --source "$S/r1/away/p.plan.toml" --metrics docs/metrics/workflow.jsonl
{
  "plan": null,
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project"
}
exit=0

$ ./target/debug/agent-scaffold next --json --source "$S/r1/away/p.plan.toml" --metrics docs/metrics/workflow.jsonl --ledger-fragment docs/plans/agent-scaffold.ledger.md
{
  "task": "p",
  "source": "no plan source",
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
  "active_loop": null,
  "resume_state": null,
  "resume_state_absent_reason": "ledger-not-this-project",
  "no_active_loop_reason": "no-plan-steps"
}
exit=0
```

Byte-for-byte the reviewer's output. `"plan": null` and `"source": "no plan source"` are the tool reporting it read no plan; `log-not-this-project` and `ledger-not-this-project` are the predicate firing anyway, on a root supplied from the anchors. So `:163`'s "`status --resume` is the one surface that reads NO plan" is FALSE, and `:179`'s parenthetical, which states the trigger as "the canonically-derived root of the plan THAT SURFACE READS", is FALSE as a statement of the trigger.

The code says so at `src/main.rs:1406-1417`, `containment_roots`' doc comment, titled "One predicate, TWO ROOT-SUPPLY POLICIES, and this is the one place they meet", and again at `src/main.rs:1717-1718` in `run_next`: "rooted on the plan `next` itself projects from, or on the anchors where it projects from no plan at all". I opened both.

NEITHER SITE HAS EVER BEEN TOUCHED. `git log -S` on each clause returns exactly one commit, `52725e3` ("docs: root the containment predicate on the plan the check reads"), which is the pre-inc2 design pass. No round-1 or round-2 fix commit modified either.

SEVERITY medium CONFIRMED, on the same ground round 2 confirmed `R2B-1` at medium: no behaviour is wrong and the shipped prose gets it right, so no user is misled, but the sidecar is the durable design record the queued `validation-constraints` step inherits and these sentences state a rule the tree does not implement.

MINIMAL REMEDY: DELETION, at both sites, and deletion IS available here, which is the class this project measures as re-seeding nothing.

- `:163`: delete the two words "the one", leaving "`status --resume` reads NO plan (`src/main.rs:run_resume` derives `<task>` from the source-or-plan filename and reads only the ledger), so it has no checked plan to root on." True as written.
- `:179`: delete the parenthetical "(the canonically-derived root of the plan THAT SURFACE READS, and whether the resolved artifact lives under it)", leaving "The trigger is the SAME containment predicate the validator's refusal uses. The predicate is never re-implemented per surface (One source of truth)." True: `is_outside_root` is one function at `src/main.rs:1493`, called from eight sites.

A RESIDUAL THE DELETION LEAVES, stated so the fix pass does not quietly author past it. After both deletions the file still nowhere states the anchor-supplied-root policy for `status` and `next`; `:182` states it for `status --resume` alone, which is true but partial. Writing it would be AUTHORED PROSE and is not the minimal remedy, and `containment_roots`' doc comment already carries the full rule for a reader who follows the citation. I recommend taking the deletion and ACCEPTING the residual, and I record it here so a round-4 reviewer raising it is met with a decision rather than a surprise.

SCOPE. Condition 3 fails on both limbs, so the finding is IN SCOPE: the claim is about what the increment changed (inc4's declared subject is this file's stale claims, and `Q-55-currencyscope` item (1) as re-derived by the planner covers everything inc2 and inc3 falsified), and the falsifying change is this step's own inc2. That reading is settled in this task by two triage applications and one human decision (`Q-55-twinsites`) and I follow it rather than re-litigating it. It is also, more simply, the unclosed remainder of an already-confirmed round-2 finding.

### THE PROCESS QUESTION, WHICH MATTERS MORE THAN THE FIX

The brief asks me to rule whether the defect is that the TRIAGE under-scoped its remedy relative to its own reviewer's evidence, or that the CLASS-SCOPED SWEEP RULE is bounded to a paragraph when the class is not. It is the FIRST, and the evidence is decisive rather than balanced.

WHAT ROUND 2'S REVIEWER WROTE. `...-r2-coldread-opus.md` raised `R2B-1` and named three sites, its primary at `:157` and then, under the heading "TWO SUPPORTING SITES IN THE SAME FAMILY, both also untouched by both passes, both stating the checked-plan root as the whole rule", `:179` and `:163`, each quoted verbatim with the reason it is false. It even recorded the negative search that proves the gap: "I checked the whole file for a sentence that DOES describe the anchor-root fallback for `status` and `next` ... the only one ... is `:182`, which scopes it to `status --resume`."

WHAT ROUND 2'S TRIAGE WROTE. I searched the whole triage file:

```
$ grep -n '`:163`\|`:179`\|:163\b\|:179\b' docs/plans/agent-scaffold.reviews/workflow-enforcement-tier-inc4-r2-triage.md
(no output)
```

ZERO MENTIONS. The two sites appear nowhere in the triage: not in the remedy, not in a dismissal, not as a recorded residual. They were not judged and rejected; they were dropped. The triage's MINIMAL REMEDY entry re-tenses `:157`'s sentence only and adds a paragraph arguing why deletion is unavailable AT `:157`, which reads as a complete treatment of the finding and is not one.

WHY THE SWEEP RULE CANNOT CARRY THE BLAME. `:157`, `:163` and `:179` are in three different paragraphs, and `:179` is in a different SECTION:

```
157: THE REFUSAL (candidate (b), layered on top). ...
158: <blank>
159: WHY THE ROOT COMES FROM THE CHECKED PLAN ...
160: <blank>
161: BOTH ALTERNATIVES WERE PUT AND REJECTED ...
162: <blank>
163: WHICH PLAN EACH SURFACE READS IS ALREADY DECIDED IN THE TREE ...
...
167: ## One predicate, two responses: ...
177: ### The exact behaviour, per surface
179: The trigger is the SAME containment predicate ...
```

A rule scoped to "the whole enclosing sentence and paragraph" could not reach either, and the writer applied it correctly. The rule's job is to reach sites NO FINDING NAMED, and round 2's own record shows it doing that job: the class-scoped brief "FOUND AND FIXED TWO IN-PARAGRAPH INSTANCES NO FINDING HAD NAMED". These two sites did not need discovering. They needed CARRYING.

I also decline the tempting general cure of widening the sweep bound from the paragraph to the file. That would make every remedy a file-wide authoring pass, which is the cost this project has six measurements of: an authoring fix pass manufactures the next round's finding while a deletion-class pass re-seeds nothing.

THE RULE THAT WOULD HAVE CAUGHT IT, in the form a fix brief can carry:

> A TRIAGER'S MINIMAL REMEDY MUST ACCOUNT FOR EVERY SITE THE FINDING'S REVIEWER NAMED. Where the remedy reaches fewer sites than the reviewer enumerated, the triage states, IN PLACE, one of three verdicts for each dropped site: it is false too and is included in the remedy; it is true and the reviewer was wrong about it, with the evidence; or it is valid but out of scope, recorded as a residual. A SITE THAT APPEARS IN A FINDINGS FILE AND IN NO VERDICT IS A LOST FINDING, and the fix brief inherits the loss silently, because the writer has no signal that anything is missing.

AND ITS COMPANION FOR THE BRIEF:

> THE FIX BRIEF'S SITE LIST IS THE UNION OF THE TRIAGE'S REMEDY SITES AND THE SITES THE UNDERLYING FINDINGS NAMED. Where the triage names fewer, the brief carries the reviewer's list too and asks the writer to report each site it did not change and why.

This is the missing half of a rule the project already has on the writer side, ledger `:851`: "a triager's site count is a MEASUREMENT, not an instruction, and a producer finding it short should widen and report". That cure assumes the writer can measure the count short. Here it could not, because the two extra sites were invisible from the triage. The gap is on the triage side, and it is my own role's failure mode, which is why I state it plainly rather than routing it elsewhere.

## `R3B-2` (medium confirmed): VALID, fix required. IN SCOPE.

REPRODUCED. `docs/plans` a real directory, `docs/metrics` a symlink out of the root, bare relative `--source`, run from the project root, which is exactly the invocation `:104` promises must be unchanged:

```
$ ls -l docs/
lrwxrwxrwx metrics -> ../../elsewhere
drwxr-xr-x plans

$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
--workflow would join docs/plans/TEMPLATE.plan.toml against docs/metrics/workflow.jsonl, which is not under the plan's project root <S>/symlog/root; pass a `--metrics` under that root, run against the plan's own log, or correct the `--source` and `--plan` pair
exit=1

$ agent-scaffold status --source docs/plans/TEMPLATE.plan.toml
plan: 1 steps (1 not started); 0 open-questions items
metrics: unavailable, the round log docs/metrics/workflow.jsonl is not under the plan's project root <S>/symlog/root, so its records cannot be paired with this plan
exit=0
```

`:104`'s exception clause names only "the symlinked-`docs/plans` layout". This layout is not that one. `:257` widened the cost to the population ("THE COST IS THE DIVERGENCE AND NOT THE LAYOUT: any symlink that makes the canonicalised checked plan and the canonicalised resolved log fall under different roots produces it, on either side"), and check 19 at `:342` pins this exact second layout as expected behaviour ("A SECOND LAYOUT PINS THE LOG SIDE: `<root>/docs/metrics` a SYMLINK out of the plan's project root ... gives the same refusal and the same omission"). I read all three sites.

THE STRONGEST FRAMING, and it is stronger than the reviewer's. This is not a tense defect and it does not depend on the spec-time exemption at all. Read `:104` as a pure requirement written at spec time and it STILL contradicts check 19 in the same file: the requirement would forbid what the check pins as expected. It is a document whose definition of done contradicts its own acceptance criterion, which is the class the ledger records at `:825` as the single most productive defect class in this artifact.

SEVERITY medium CONFIRMED. No behaviour is wrong and no user is misled; `README.md:236` states the symlink cost without narrowing it. What earns medium is that `:104` is the sentence a later reader checks the step against, and as written it makes an accepted, pinned cost read as an unmet end property, which is precisely the outcome `:253` exists to prevent.

MINIMAL REMEDY: TOKEN. "except for the symlinked-`docs/plans` layout recorded below as accepted cost (ii)" becomes "except for the symlink divergence recorded below as accepted cost (ii)". Four words out, two in, and "the symlink divergence" is drawn verbatim from `:257`'s own widening sentence, so no fact is authored.

SCOPE: IN SCOPE and it is not close. `:104` is in this file, which is the increment's declared main artifact (`:384`), and the widening at `:257` that made it wrong landed at `a5786ae` inside this step. Condition 3 fails on both limbs.

## `R3B-3` (medium confirmed): VALID, fix required. IN SCOPE.

REPRODUCED, all three parts.

The twin that WAS fixed, `run_resume`'s doc comment at `src/main.rs:1629-1636`, carries three causes: "A missing ledger, an absent section, or a ledger that is not this plan's all print a note and exit 0".

The two that were NOT. `src/main.rs:1192-1195`:

```
// The thin `status --resume` slice: print the ledger's `## RESUME STATE` block
// verbatim (reusing the same extractor `next` uses) instead of the state projection.
// A missing ledger or absent section is a note and exit 0, not a failure (`status` is
// best-effort).
```

And `src/main.rs:461`, `StatusArgs::resume`, which is a USER-VISIBLE `--help` string. I ran `status --help` and it prints: "Exits 0 with a note when the ledger or the section is absent".

The third cause exists and behaves exactly like the two named:

```
$ agent-scaffold status --resume --source "$S/res/proj/docs/plans/p.plan.toml" --ledger-fragment "$S/res/foreign/p.ledger.md"
the ledger <S>/res/foreign/p.ledger.md is not under the plan's project root <S>/res/proj; nothing to resume
exit=0
```

The ledger exists and carries a `## RESUME STATE` section, so neither named cause applies, and the surface prints a note and exits 0.

SEVERITY medium CONFIRMED, and I considered `high` because one site is user-visible. It does not reach `high`: the help under-ENUMERATES but does not misdescribe, since the third cause produces the same shape the help promises (a note, exit 0). A user reading the help and meeting the third case is not led to a wrong action, only to a case the help did not list. That is medium.

MINIMAL REMEDY: TOKEN at both sites, every word drawn from the already-corrected twin so nothing is authored.

- `src/main.rs:1194`: "A missing ledger, an absent section, or a ledger that is not this plan's is a note and exit 0, not a failure".
- `src/main.rs:461`: "Exits 0 with a note when the ledger is absent, carries no such section, or is not this plan's."

DO NOT TOUCH `src/main.rs:368`, the `status` subcommand summary ("Best-effort; a missing file yields a partial projection"). It is a summary rather than an enumeration and it stays true. The reviewer flagged it and I agree; a fix pass that authors prose there would be defect (17) again.

### SCOPE, RULED EXPLICITLY BECAUSE THE BRIEF ASKS FOR IT: IN SCOPE

The four conditions, all of which must hold for out-of-scope:

1. PROVENANCE PREDATES THE BASE COMMIT: HOLDS. `git log -L 461,461:src/main.rs` gives `609ddcf` ("fix: anchor the metrics log and the ledger to the plan source"), which is this step's own inc1; `git log -L 1192,1195:src/main.rs` gives `e05e71f`, well before the step. Both predate inc4's base.
2. NO COMMIT IN RANGE MODIFIES THE CLAIM'S LINES: HOLDS. The only `src/main.rs` hunk in `main..HEAD` is at line 570, the `Projection.plan` doc comment.
3. INDEPENDENT SUBJECT: FAILS, ON BOTH LIMBS. The second limb is the plain one: INC2 IS WHAT FALSIFIED IT, and inc2 is this step's own increment, which is the reading settled by two triage applications (`R1C-3`, `R1C-4`, then `R2B-1`) and by the human at `Q-55-twinsites`. The first limb fails too, and this is the part worth stating: the sidecar site that OWED this change is `:367`, inc2's own documentation-impact bullet, which names `run_resume`'s doc comment and only that. `:367` is inside this file, is itself defective (`R3B-4`), and is the reason these two twins were missed. The claim is therefore about what the increment changed.
4. NO SHARED FIX: does not matter, since condition 3 already fails and all four are required.

THE COUNTER-ARGUMENT, PUT AND ANSWERED. `Q-55-currencyscope` closed the scope to named items and this is not one of them, and `:388` excludes shipped prose on the ground that "inc4 changes no behaviour, so no shipped prose goes stale". Both are true and neither reaches this. The items the human DECLINED are ones that PREDATE the step (traced to `8017a2c`, to `f230f80`, to the Status narrative), and these two do not: inc2 falsified them. `:388`'s ground is about INC4 staling prose, and inc4 did not; INC2 did, and inc2's own impact list under-named the sites. The human has already ruled on this exact shape once inside this increment: `Q-55-twinsites` admitted CODE twins of corrected sidecar claims, over the option of leaving them out of scope, on the ground that "this task has been bitten THREE TIMES by a fix landing at one site while its twin survived a literal grep". `src/main.rs` is in the INC4 impact list at `:386` and is not in the `:388` exclusions.

ONE FACT FOR THE ORCHESTRATOR, NOT A CHANGE OF VERDICT. `Q-55-twinsites` reached test comments; this reaches a `--help` string, which is the first user-visible surface any inc4 remedy has touched. The precedent covers it and I rule IN SCOPE on the precedent. If the orchestrator judges that "user-visible" is a boundary the human would want to see, the ground for asking is that fact alone, not the scope rule.

## `R3B-4` (low confirmed): VALID, fix required. IN SCOPE.

REPRODUCED, and then verified more broadly than the finding claims.

```
$ grep -n 'A missing ledger or absent section prints a note and exits 0' src/main.rs
(no output, exit 1)
```

`:367` attributes that quotation to `src/main.rs:run_resume`'s doc comment, in a present-tense frame ("whose ... clause gains the unsafe-fragment case"). The live comment reads the corrected three-cause form, so the quoted text is genuinely absent and the frame is wrong. Check 21 at `:345` states the rule this breaks in as many words: "A quotation with no match in the tree is either RE-TENSED ... or DELETED".

I RAN AN INDEPENDENT WHITESPACE-NORMALISED SWEEP rather than trusting the reviewer's, because this is the finding that decides whether check 21 currently passes. I extracted all 54 distinct double-quoted spans of 40 characters or more from the sidecar and matched each, backticks stripped and whitespace collapsed, against a normalised concatenation of `src/`, `tests/`, `pack/`, `.agents/`, `README.md`, `CHANGELOG.md` and `justfile`. The no-match set is 46 spans, and I classified every one: quotations of the ledger, decision-option labels, review questions, section headings, tool OUTPUT rather than file text, spans containing an ellipsis, and past-tense-framed quotations of pre-fix text which check 21 explicitly permits. I opened each past-framed one to confirm the frame: `:199` "BECAME FALSE", `:200` "BECAME INCOMPLETE", `:201` "HAD THE SAME DEFECT", `:202` "WAS SHORT BY ONE", `:204` "SAID", `:374` "STATED", `:375` "FRAMED ... CARRIED", `:377` "READ", and check 22's `:347` "no longer says". The sweep's one false alarm was `:52`, whose quoted comment IS in the tree at `src/main.rs:1039-1041` and failed only a literal grep because the comment wraps across `//` lines.

`:367` IS THE ONLY PRESENT-TENSE-FRAMED QUOTATION IN THE FILE, ATTRIBUTED TO A TREE FILE, WITH NO MATCH. That is an independent confirmation of the still-true lens's 70-fragment sweep, arrived at by a different extraction.

SEVERITY low CONFIRMED. One bullet, one word, no reader is misled about behaviour.

MINIMAL REMEDY: TOKEN. `:367`'s "gains the unsafe-fragment case" becomes "GAINED the unsafe-fragment case".

FIX IT WITH `R3B-3`, NOT SEPARATELY. They are one defect seen from two sides: `:367` named one of three sites, so two code twins survived. A brief that fixes the tense at `:367` and not the two code sites has fixed the symptom.

## `R3B-5` (low confirmed): VALID, fix required. IN SCOPE.

REPRODUCED. `src/main.rs:979` binds `toml_primary`; `src/main.rs:1005` opens the match. Lines 980 to 1004 are the containment guard the sentence is arguing FOR: its comment, `checked_root`, `checked_display`, `unsafe_pairing` and the refusal branch. TWENTY-FIVE lines, not the reviewer's 26, which changes nothing.

The substantive claim ("available BEFORE the match", "does not force the guard down into the arms") is TRUE and I confirmed it. Only "immediately" is wrong, and it is wrong because the guard the sentence proposed got built in the gap.

SEVERITY low CONFIRMED. MINIMAL REMEDY: DELETION of the single word "immediately". IN SCOPE: inc2 built the guard that falsified it, condition 3 fails.

## `R3C-1` (medium confirmed): VALID. OUT OF SCOPE. BACKLOG STEP.

REPRODUCED IN PART, as declared above.

BASELINE at `cb5e6ac`, all green: `cargo test` exit 0, `cargo clippy --all-targets -- -D warnings` exit 0, `render --check` exit 0 "up to date", `render --check --strict` exit 0, `validate --source docs/plans/agent-scaffold.plan.toml --workflow` exit 0 over 293 records.

`PC1` FIRES, so the rig detects what it is built to detect. Flipping `test-tmpdir-repo-assumption` to `complete` in a scratch copy and re-rendering:

```
render --check --strict            exit 0
validate --source ... --workflow   exit 1
  Roadmap step `test-tmpdir-repo-assumption` is `complete` but has no round records and no
  covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates
  logging or its review was skipped
```

`M1` REPRODUCED, and the two-state result is the informative one:

```
WITHOUT re-render:  render --check          exit 0 with a warning naming line 1814
                    render --check --strict exit 1
WITH re-render:     render --check --strict exit 0 "up to date"
                    validate --workflow     exit 0 "workflow invariants hold"
```

The strict form catches STALENESS, not falsity. Once the source and the view agree, which is the state all twenty findings were committed in, nothing fires.

`M2` REPRODUCED: the `-w1` waiver note's "13 valid findings (3, 4, 6)" replaced by "20 valid findings", re-rendered, all gates green and silent.

THE ZERO-MUTATION CORROBORATION REPRODUCED, and it needs no fixture: `checks-runner-worktree-name-collision.md:14` cites `src/checks.rs:78` for `RUNNER_PREFIX` and `:7` cites `src/checks.rs:791-792` for the worktree naming. At this commit `sed -n '78p'` is `PathBuf,` and `791,792p` is `if !listed.status.success() {` / `return Err(RunError::WorktreeSetup(format!(`. Neither range holds its subject, a population of about eleven such citations sits in the committed tree deliberately by `Q-55-check21b`, and all six gates are green over them right now.

THE MECHANISM, VERIFIED RATHER THAN ARGUED, which is what lets me accept the four mutations I did not re-run:

- No test or fixture reads the step sidecars at all: `grep -rn "agent-scaffold.steps\|workflow-enforcement-tier.md" tests/ src/` returns nothing.
- `w5_problems` (`src/workflow.rs:544-621`) reads `waiver.step`, `waiver.unit`, `waiver.increment`, `waiver.evidence`, `waiver.evidence_tier` and `waiver.reason`. I listed every `waiver.` access in the function. `waiver.note` is not among them.
- `render --check` compares the committed view against a fresh render of the same sources. It is a copy comparison and has no opinion on truth, which `M1`-with-re-render measures directly.

SEVERITY medium CONFIRMED. Not higher: nothing behavioural is wrong, nothing reaches a user, and the class IS being caught reliably by adversarial review, which is what found all twenty. Not lower: it is this project's own highest-yield defect class by a wide margin, and the recorded transferable lesson of inc3 is that the highest-value question available is whether anything would notice if the artifact were wrong.

### ON THE STEP'S OWN THESIS: I TESTED THE RESTRAINT AND THE RESTRAINT IS CORRECT

The reviewer declined to claim the 0-of-20 result falsifies the step's thesis, on the ground that `validate --workflow` is scoped to round-log evidence rather than prose truth. I did not inherit that. I asked the harder version: this step exists to remove false greens from the enforcement surface, and its closing increment produced twenty defects every deterministic check passed over. Is that a false green?

IT IS NOT, and the test that settles it is what each green ASSERTS, which I checked one at a time rather than reasoning from the step's summary:

- `workflow invariants hold` asserts W1 to W5. They do hold. `PC1` shows W3 firing at exit 1 the moment they do not.
- `95 steps, 69 questions, valid` asserts the plan parses and is structurally valid. It is.
- `up to date` asserts the committed view matches a fresh render of its sources. It does, and `M5` shows the strict form failing at exit 1 the moment it does not.
- `pack/AGENTS.md:93`, which I read in full, promises the backstop "that the required reviewed rounds happened before a step is marked complete", conditioned on instrumentation being on. That is a claim about round-log evidence and `PC1` is it being kept.

A FALSE GREEN REQUIRES A GREEN THAT ASSERTS SOMETHING IT HAS NOT ESTABLISHED. Every green this repository emits is honest about its own scope. Defect A was a false green because a check that DID NOT RUN reported success; here every check ran and reported exactly what it checked. The step's thesis survives, and reporting otherwise would be the more dramatic claim rather than the supported one.

WHERE THE REVIEWER WAS TOO GENEROUS, AND IT IS ONE PLACE. It framed `R3C-2` as another instance of a general absence. It is not: `M2` is a join whose BOTH SIDES the same command already holds in memory in the same run, and the commit that fixed `R1B-1` says so in its own message ("A breakdown checks itself against the log; a bare total does not, which is how this figure stayed wrong through three rounds"). That commit identified the mechanism and shipped it as a human convention. Everything else in the 0-of-20 tally is a class the project never claimed to cover; `R3C-2` is the one where it identified the check, wrote down why it was worth having, and then did not build it. That distinction should survive into the backlog step's ordering.

### DISPOSITION, PER FINDING AND SPECIFIC

FIX INSIDE THIS INCREMENT: `R3C-4` only, and it is the only one of the five the reviewer placed here. It is text this increment authored, in this file.

A NEW BACKLOG STEP FOR THE PROJECT: `R3C-1`, `R3C-2` and `R3C-3`, as ONE step with three mechanisms, in this order, which is descending buildability and descending certainty:

1. Mechanism 1 (`R3C-2`), the `W6` waiver-note join. Cheapest, best evidenced, both sides already parsed.
2. Mechanism 2 stage one (`R3C-3`), dangling-receipt detection: every `type:"decision"` `q_id` names a registered `[[question]]` id or a declared sub-id. Unambiguous and cheap. Stage two, comparing `chosen` against the plan prose, waits on stage one and on a human decision.
3. Mechanism 3 half A (`R3C-1`), the quotation resolver: every backticked or quoted span attributed to a named file, run as a literal search against it, ellipsis-bearing spans skipped. This automates check 21's already-decided procedure rather than inventing policy. Half B, citation resolution, needs a decision on symbolic-versus-line-numbered form and a declared suppression for the `Q-55-check21b` population, and should not be bundled into the same increment.

NOT A FIX INSIDE INC4, for all three, on the plain ground that the remedy is new mechanism and `Q-55-currencyscope` closed this increment against exactly that. A documentation-currency pass that grew a validator increment would be the widening the human declined.

### SCOPE: OUT OF SCOPE, all four conditions argued

1. Provenance predates the base commit: HOLDS. The gate set, `w5_problems`, `render --check` and `.agents/checks.toml` all long predate inc4.
2. No commit in range modifies the claim's lines: HOLDS. `main..HEAD` touches `src/main.rs` at one line and `tests/unsafe_pairings_are_refused_and_omitted.rs` at five, and no gate implementation at all.
3. INDEPENDENT SUBJECT: HOLDS, on both limbs. The claim is not about what the increment changed; it is about what the repository's gates read. And the increment's change is not what falsified it: nothing falsified it, because it was never true. This is the limb that does the real work, and it is what separates `R3C-1` from `R3B-1` through `R3B-5`, all of which inc2 falsified.
4. No shared fix: HOLDS. Its remedy is a new detector and shares nothing with any in-scope remedy.

All four hold, so under the recorded precedent this finding DOES NOT RESET THE CONVERGENCE STREAK. In this round that changes nothing, because six in-scope findings already reset it. I say that explicitly because a scope ruling reached for while it could decide the streak deserves less trust than one reached while it cannot, and this one cannot.

## `R3C-2` (medium confirmed): VALID. OUT OF SCOPE. BACKLOG STEP.

REPRODUCED, all three parts. `M2` green and silent, above. `w5_problems`' field list, above, with no `waiver.note`. And the refuting records, which are in the file the same invocation already parses:

```
workflow-enforcement-tier-inc1 top-level valid_findings per round: 3 4 6
```

The tool holds a note claiming 20 and records summing to 13 in memory at the same moment and prints `workflow invariants hold`.

SEVERITY medium CONFIRMED. A waiver is how a step is exempted from the round-count requirement and its `note` is the justification a later reader judges the exemption by; `R1B-1` measured that content wrong by 54 percent through three rounds. Not higher: the note is load-bearing for no exit code, and every structured field that IS load-bearing is checked by W5 today.

MINIMAL REMEDY, RECORDED PER THE PRECEDENT'S GUARD: a `W6` check joining the `note`'s parenthesised per-round sequence and its stated total against the `valid_findings` of the `type:"round"` records that join to the waived unit, element by element in round order. A note with no recognisable breakdown is not a failure, which makes the check opt-in by writing one. The convention is already in use at all three sibling waivers and I verified all three parse and match.

SCOPE: OUT OF SCOPE, same four conditions as `R3C-1`, condition 3 on the same ground. Does not reset the streak.

## `R3C-3` (medium confirmed): VALID. OUT OF SCOPE. BACKLOG STEP.

REPRODUCED. `PC2` first, and it does not fire:

```
$ grep -v '"q_id":"Q-55-w1figure"' <copy>/docs/metrics/workflow.jsonl > tmp && mv tmp <copy>/.../workflow.jsonl
292
$ agent-scaffold validate --source <copy>/docs/plans/agent-scaffold.plan.toml --workflow
... 292 records, valid
... 95 steps, 69 questions, valid
... workflow invariants hold
exit=0
```

The measurement, recomputed:

```
distinct q_ids in decision receipts: 51
registered [[question]] ids in the plan: 69
receipts whose q_id is NOT a registered question: 29
```

A SHARPENING THE REVIEWER DID NOT STATE, and it changes how the finding should be read. I listed all 29. Every one is a `Q-55-*` sub-id: `Q-55-anchorveto`, `Q-55-check21b`, `Q-55-conventionlesscost`, `Q-55-currencyscope`, `Q-55-emptyroot`, `Q-55-emptyrootsite`, `Q-55-endproperty`, `Q-55-existsgate`, `Q-55-fallbacksurface`, `Q-55-impactlist`, `Q-55-inc2escalation`, `Q-55-inc3escalation`, `Q-55-jsonreason`, `Q-55-ledgerdivergence`, `Q-55-ledgerslot`, `Q-55-mechanism`, `Q-55-noconvention`, `Q-55-owedcount`, `Q-55-plandoccurrency`, `Q-55-receiptcurrency`, `Q-55-refusalscope`, `Q-55-residualbound`, `Q-55-resumecost`, `Q-55-resumepairing`, `Q-55-scope`, `Q-55-spectime`, `Q-55-twinsites`, `Q-55-verifyclose`, `Q-55-w1figure`. The unjoined population is not spread across the project: it is exactly ONE question's sub-id convention, invented by THIS step, which used it 29 times.

### IS IT THE SAME DEFECT CLASS AS INC3'S RECORDED LESSON, AND DOES THAT RAISE IT?

IT IS THE SAME QUESTION, ASKED OF A DIFFERENT SUBSTRATE, AND THE ANSWER IS THE SAME. Ledger `:579` records inc3's lesson: a mutation lens asked "IF THIS CODE WERE WRONG, WOULD THE SUITE SAY SO", and one of the eleven uncaught mutations was that the suite could not detect the silent reversal of a human decision. Asked of the plan substrate, the answer is no, and `PC2` is the demonstration.

IT IS NOT THE SAME DEFECT, AND THAT IS WHY IT DOES NOT GO ABOVE MEDIUM. Inc3's defect was a mechanism that EXISTED, was NAMED as pinning that decision, and DEGENERATED silently on a machine where the process is not stopped by a file mode. That is a false green in a check the project believed covered the decision, and the remedy was six lines because the check was already there. `R3C-3` is a mechanism that never existed and was never claimed to. `pack/instrument.md:9` scopes W4 precisely: it "requires a matching receipt for every decided item whose `q_id` is after the declared W4 baseline", and a sub-id is not a `[[question]]` entry at all. There is nothing asserting coverage that is absent, so there is no false green, only an uncovered surface. Raising it to `high` on the strength of an analogy to a false green would be inflating it, and the brief tells me not to.

WHAT IT ACTUALLY COSTS, stated concretely and with both sides:

- Deleting a `Q-55-*` receipt outright is invisible to the one command that reads both substrates (`PC2`, reproduced).
- Editing the plan to the option the human REJECTED is invisible in both directions, with the receipt still in place asserting the opposite (`M2` reproduced; `M6` not re-run by me, same mechanism).
- Against that: the log is append-only and under version control, so both mutations appear in `git diff` and in a line count; the ledger narrates all 29 decisions in prose; each is also written into the `Q-55` question record in the plan TOML; and the 22 registered ids past the baseline ARE joined by W4. A human auditing by hand can reconstruct every one. The defect is that nothing mechanical does.

SEVERITY medium CONFIRMED, neither raised nor lowered. The floor under it is `PC2`: `pack/instrument.md:9` sells a receipt as "auditable evidence the human-input contract was met rather than a self-certified flag", and for 29 of 51 ids it is a record nothing reads. The ceiling over it is that the same sentence grounds that auditability in the record's CONTENT (options plus chosen), not in a mechanical join, so even that does not over-claim.

MINIMAL REMEDY, RECORDED: stage one only, and it is unambiguous. Every `type:"decision"` record's `q_id` must resolve to a registered `[[question]]` id, or to a registered id plus a declared sub-id suffix. Stage one going red on 29 existing receipts is the point, not a problem, but it means the backlog step must first put a human decision: are sub-ids registered as questions, or is a sub-id suffix convention declared? That is a decision, not an implementer's call.

SCOPE: OUT OF SCOPE. Conditions 1, 2 and 4 hold as for `R3C-1`. Condition 3 holds: the claim is about a check that does not exist, not about anything inc4 changed, and no change falsified it. Inc4 ADDED seven of the 29 unjoined receipts (`Q-55-currencyscope`, `Q-55-twinsites`, `Q-55-receiptcurrency`, `Q-55-w1figure`, `Q-55-check21b`, `Q-55-spectime`, `Q-55-impactlist`), and I considered whether that fails condition 3. It does not: adding more records to a population nothing joins is not what makes the join absent. Does not reset the streak.

## `R3C-4` (medium confirmed): VALID, fix required. IN SCOPE.

I RAN CHECK 21 AS WRITTEN, both halves, which is what the brief asks.

THE CITATION HALF. Counts reproduced exactly: 22 `file:line` occurrences and 51 symbolic `file:Identifier` occurrences. Deduplicated they are 13 and 23. I resolved all 13 line-numbered citations and every one is in bounds and holds its subject. I also resolved all 23 symbolic ones by identifier presence in the named file, and every one resolves.

THE QUOTATION HALF. Run as written, over the sweep described under `R3B-4`, it FAILS at `:367`.

SO THE HEADLINE CORRECTION IS THIS: CHECK 21 IS EXECUTABLE, IT IS FALSIFIABLE, AND IT FAILS RIGHT NOW. "A check that cannot fail is worse than no check" does not apply, and I decline the reviewer's "NOT EXECUTABLE AS WRITTEN" framing. What survives correction is still a defect, and it is two defects:

DEFECT 1 CONFIRMED. The check's headline asserts "EVERY CITATION AND EVERY QUOTATION IN THIS FILE RESOLVES", and its procedure defines how to check exactly one of the two citation forms. "Open each `file:line` citation at the cited range" has no cited range for 51 of the 73 occurrences. A symbolic citation cannot go stale by line movement, which is presumably why nobody wrote a procedure, but the check claims a coverage it does not define, and the increment that authored it exists to remove exactly that kind of false precision.

DEFECT 2 CONFIRMED. The scope is "THIS FILE". Three of the twenty findings are not in it: `R2A-1`/`R2C-1` at `docs/plans/agent-scaffold.plan.toml:1732`, `R2A-3`/`R2C-2` at `:1728`, and `R1C-5`'s twin cluster in the `Q-55` `ask` at `:1713-1736`. I opened all three ranges and confirmed they are the `Q-55` question-record sites in the plan TOML. This increment edited that file (14 lines in `main..HEAD`), and neither check 21 nor check 21b reaches it: 21b is restricted to the `src/main.rs` and `tests/` citations of three named sidecars.

DEFECT 3, THE THIN-COVERAGE CLAIM: TRUE BUT NOT LOAD-BEARING. Run as written the check reaches two of the twenty squarely and two marginally. I agree with the arithmetic and I do not treat it as part of the defect: a check is not defective for having a narrow subject. It is defective for claiming a subject wider than its procedure.

SEVERITY medium CONFIRMED, after the correction. Nothing behavioural is wrong and the unreached surface was in practice covered by reviewers. What holds it at medium rather than low is that this is the acceptance criterion for the increment's entire stated purpose, and it overclaims its own coverage, which is the increment's own defect class turned on the increment.

MINIMAL REMEDY: AUTHORED PROSE, and it is the only authored-prose remedy in scope this round. Two clauses on check 21:

- The symbolic-citation procedure: a `file:Identifier` citation is verified by the identifier existing in the named file, line numbers not being involved.
- The scope: extend it to the `Q-55` question record and the three `workflow-enforcement-tier-w*` waiver notes in `docs/plans/agent-scaffold.plan.toml`, which this increment edited.

BOTH FACTS ARE MEASURED, NOT ASSERTED: I resolved all 23 distinct symbolic citations by identifier presence and all 13 line-numbered ones by range, first-hand, before writing this. That matters because ledger `:555` records orchestrator defect (18), where the last narrowing of an acceptance check in this increment was authored on an unchecked premise and answered a `low` finding with a `medium` one. The standing cure is that the premise gets checked first; it has been.

THE TOKEN-CLASS ALTERNATIVE, RECORDED AND NOT RECOMMENDED: narrow the headline to "EVERY LINE-NUMBERED CITATION AND EVERY QUOTATION IN THIS FILE RESOLVES" and change nothing else. That removes the overclaim without authoring, and this project's calibration prefers that class. I do not recommend it, for two reasons. It retires coverage the increment ACTUALLY PERFORMED, since all 23 symbolic citations were verified and now resolve. And narrowing an acceptance check to make it true is precisely the move that produced defect (18). Given the project's authoring risk, I would keep the remedy to these two clauses and no more, and I flag it as the one place this round's fix pass can manufacture round 4's finding.

SCOPE: IN SCOPE, and condition 1 fails outright, which is the cheapest of the four. `git log -S` on check 21's headline returns `1c5b715`, the first commit of this increment's range. The check was authored inside the range by this increment.

## `R3C-5` (low confirmed): VALID. OUT OF SCOPE. PROCESS NOTE.

REPRODUCED. `grep -c -- '--strict'` over the eight inc4 reviewer and triager files returns 0 for all eight. `.agents/checks.toml:18` declares `agent-scaffold render --check docs/plans/agent-scaffold.plan.toml --strict` as the project's render gate. I ran both forms at this commit: the plain form warns at exit 0 and the strict form exits 1 on a divergence (`M1`-without-re-render, measured above).

NOT A RE-RAISE OF `R1B-3`. `R1B-3` claimed check 23 adds nothing over checks 1 and 9 and was dismissed because its premise did not reproduce. This claim is about which FORM of the gate the review passes ran, which `R1B-3` never touched. I re-read the round-1 dismissal to confirm.

SEVERITY low CONFIRMED. The reviewer is right that no harm resulted: both triages pasted the "up to date" OUTPUT text rather than reading exit codes alone, which is the safeguard check 23 exists to provide, and I re-ran both forms green.

WHAT IT IS AND IS NOT. It is a true claim about the review record, not a defect in the artifact under review. I record it as valid rather than dismissing it because the recommendation has real preventive value and costs one line, and because reporting it explicitly is what the brief asks for over a bare clean. It is worth being clear that the round would be new-valid with or without it, so nothing about this ruling touches the streak.

MINIMAL REMEDY, RECORDED: one line in the orchestrator's gate transcript convention, requiring the strict form (or `agent-scaffold checks`) and its output rather than the warning form. AUTHORED PROSE, one line, outside the plan.

SCOPE: OUT OF SCOPE. All four conditions hold. Provenance and the gate declaration long predate the base commit; no commit in range touches `.agents/checks.toml` or the brief convention; the subject is the review passes' transcripts, which is not what the increment changed and which the increment did not falsify; and no in-scope remedy shares its fix.

## Corrections to the reviewers

None of these moves a verdict or a severity. They are recorded so the fix brief carries the corrected form.

1. `R3C-4`: "NOT EXECUTABLE AS WRITTEN" and "a check that cannot fail" are wrong. Check 21 is executable and falsifiable on 13 line-numbered citations and on every quotation, and it FAILS at this commit on `:367`. The defect is an overclaimed coverage and an under-drawn scope, not unfalsifiability.
2. `R3C-4`: 22 and 51 are OCCURRENCE counts. Distinct, they are 13 and 23. The ratio argument is unaffected.
3. `R3C-3`: `pack/AGENTS.md:145` does not resolve; that file has 118 lines. The quoted sentence ("auditable evidence the human-input contract was met rather than a self-certified flag") is at `pack/instrument.md:9`, and at `AGENTS.md:145` and `.agents/AGENTS.reference.md:145` in the deployed copies. The finding is unaffected; a triager citing a nonexistent line in a detectability review is worth naming out loud.
4. `R3B-5`: 25 lines sit between the binding and the match, not 26. Immaterial.

## The three ways a review is defeated, applied

DIMENSIONS. The still-true lens states its unvaried axes and they are the right ones to worry about: uid 1000 only, Linux only, no `--instrument` scaffolds, no `..`-escape matrix. Nothing in `R3B-1` to `R3B-5` generalises past a dimension it needed to vary: each is a claim about a sentence, refuted by a single measured configuration, and a single configuration is sufficient to refute a universal. The detectability lens's HELD FIXED list is the one to watch, because a NEGATIVE result IS bounded by its dimensions: one platform, one build profile, uid 1000, no concurrency, no historical binaries. Its null result is therefore "not caught on this axis set", and I read it that way rather than as "uncatchable". That bound does not weaken `R3C-1`, whose claim is about six named commands over one committed tree, which is exactly the axis set it varied.

CONTROLS. `PC1` is the control that decides whether the six nulls are measurements, and it fires at exit 1 on the first attempt, reproduced here. `PC2` is the control that did NOT behave as expected, and the lens turned that into `R3C-3` rather than discarding it, which is the correct handling. The control I judged missing and supplied myself is `M1` in its un-rendered state: without it, "render --check is green" could mean the gate is blind or could mean the fixture never diverged. It fires on staleness and not on falsity, which discriminates the two hypotheses cleanly.

ADJUDICATION. The trap here would have been to read `R3B-1`'s remaining sites, see that the FILE now contains a correct sentence at `:157` and a correct scoping at `:182`, and conclude the surviving text looks right. That asks what remains rather than what the round-2 remedy REMOVED, which was one site out of three the reviewer named. I checked what was removed, by searching the round-2 triage for the two sites and finding zero mentions, which is how the process diagnosis above became available at all.

## Recorded residuals and settled dismissals

I checked all ten findings against inc2's four recorded residuals (the in-root bound; the single-anchor `..` case with its widened bound; `ADV-2`'s rejected-ledger context slot; `R2A-2`'s off-convention `--source` surface, an INC2-era id), against inc3's four (`R3A-1`'s inert remedy clause, an INC3-era id unrelated to this round's `R3B`/`R3C` ids; `R4A-1`'s reader-level discrimination; the plain-`validate` mode-000-file-versus-unsearchable-directory inconsistency; the containment TOCTOU under a FIFO-widened mid-run symlink swap), and against the five settled dismissals `R1A-5`, `R1A-7`, `R1A-8`, `R1B-3`, `R2A-5`.

NONE OF THE TEN IS A RE-RAISE. Three came close enough to state in place. `R3B-2` is ADJACENT to accepted cost (ii) and is NOT a re-raise of it: the cost stands as accepted and the finding is that `:104` mis-states its population. `R3C-5` is adjacent to `R1B-3` and is argued above. The still-true lens landed on the dangling `validation-constraints` handle (`F-5`, ledger `:1003`) and on `Q-55-fallbacksurface` and stopped at both, correctly.

I raised nothing on the items named out of scope entirely in my brief: `run_validate`'s "`--plan` still clap-required" claims, `src/next.rs:162` and `:181-183`, the Status narrative at `docs/plans/agent-scaffold.md:7`, and the `src/checks.rs` citations in `checks-runner-worktree-name-collision.md`. On the last, I used those citations only as `R3C-1`'s corroboration that stale citations survive all gates, which is a measurement of the GATES and not a finding that the citations are stale; `Q-55-check21b` deliberately left them so and a finding to that effect would be dismissed on that ground. No finding this round concerns line length or prose wrapping.

## Round outcome

NEW-VALID, on in-scope findings alone. Six in-scope valid findings (`R3B-1`, `R3B-2`, `R3B-3`, `R3B-4`, `R3B-5`, `R3C-4`), four out-of-scope valid (`R3C-1`, `R3C-2`, `R3C-3`, `R3C-5`) which do not reset the streak under the recorded precedent and whose minimal fixes are recorded above per its guard. Zero dismissals, so no dismissal at `high` or above and NO BACKSTOP RE-CHECK IS OWED.

Streak stays 0 of 2. This is round 3 of a cap of 5, so rounds 4 and 5 must both be clean to converge, and an in-scope valid finding in round 4 makes round 5 the cap. I was told that arithmetic before I ruled, and I record here that it moved nothing: the four out-of-scope rulings are argued against the four conditions individually and every one of them turns on condition 3 or condition 1, and the streak was already reset by `R3B-1` through `R3B-5` before I reached any of them.

## Fixture hygiene

All fixtures under `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/triage-inc4-r3/`. Nothing written to bare `/tmp`. Nothing created, moved or deleted outside that subdirectory. No `chmod` was used at any point, so none is owed a restore. Every plan and log mutation ran against a copy of `docs/` inside that subdirectory, never against the worktree; `git status --short` in this worktree shows only this file.

## ASCII check

`LC_ALL=C grep -n '[^ -~]' <this file>` returns 0 hits, verified before commit.
