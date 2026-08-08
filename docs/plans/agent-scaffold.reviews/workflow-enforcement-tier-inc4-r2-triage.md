# `workflow-enforcement-tier-inc4`, round 2, triage

Adjudicates the three round-2 findings files against the increment range `363ac06..8a42b32` in worktree `.claude/worktrees/triage-inc4-r2` on branch `triage/wet-inc4-r2`. The reviewers name the pre-rebase hashes (`218c8c3`, `5b529eb`, `a534d69`, `2eb06f5`); the rebased forms in this worktree are `9b01f34`, `297bfce`, `102e328` and `8a42b32`, and the diffs are identical. Every fixture was built under `<scratchpad>/triage-inc4-r2/` only. NO CHMOD WAS USED, so nothing was owed a restore, and nothing outside that subdirectory was created, moved or deleted.

## Summary

RAW findings: 13 (A: 5, B: 5, C: 3). DEDUPLICATED: 11. Both alleged pairs are confirmed as one finding each, on the evidence and not on the description.

| id | reviewer severity | triage severity | scope | verdict |
| --- | --- | --- | --- | --- |
| `R2A-1` + `R2C-1` | medium / medium | low (CORRECTED DOWN) | IN SCOPE | VALID, fix required (ONE finding) |
| `R2A-2` | medium | medium (confirmed) | IN SCOPE | VALID, fix required |
| `R2A-3` + `R2C-2` | medium / medium | medium (confirmed) | IN SCOPE | VALID, fix required (ONE finding) |
| `R2A-4` | low | low (confirmed) | IN SCOPE | VALID, fix required |
| `R2A-5` | low | low (confirmed) | n/a | DISMISSED |
| `R2B-1` | medium | medium (confirmed) | IN SCOPE | VALID, fix required |
| `R2B-2` | medium | medium (confirmed) | IN SCOPE | VALID, fix required |
| `R2B-3` | medium | medium (confirmed) | IN SCOPE | VALID, fix required |
| `R2B-4` | medium | medium (confirmed) | OUT OF SCOPE | VALID, minimal fix recorded, streak not reset by it |
| `R2B-5` | low | low (confirmed) | IN SCOPE | VALID, fix required |
| `R2C-3` | medium | low (CORRECTED DOWN) | IN SCOPE | VALID, fix required |

VALID: 10 deduplicated. VALID BUT ACCEPT RESIDUAL: 0. DISMISSED: 1.

IN-SCOPE VALID: 9. OUT-OF-SCOPE VALID: 1 (`R2B-4`).

SEVERITY MIX OF THE VALID SET, after correction: 0 critical, 0 high, 6 medium, 4 low. TWO severities were corrected, both DOWNWARD, and both because the reviewer's stated ground did not carry the rating: `R2A-1`/`R2C-1` (its load-bearing "the re-tensed claim is still false" demonstration does not reproduce) and `R2C-3` (round 1's triage rated the identical claim shape `low` when taken alone, and reserved `medium` for a different member of that finding). No severity was corrected upward.

NOTHING WAS DISMISSED AT `high` OR ABOVE. The single dismissal is `low`, so the independent dismissal re-check is NOT triggered.

THE ROUND IS NEW-VALID, AND IT IS NEW-VALID ON IN-SCOPE FINDINGS ALONE. The streak stays 0 of the 2 this `risky` increment needs. That outcome is settled by the five IN-SCOPE `medium` findings and does not depend on any `low` ruling, on the `R2A-5` dismissal, or on the out-of-scope ruling for `R2B-4`. I was told the convergence arithmetic before I ruled, and I am recording that I was told: round 2 being new-valid puts rounds 3 and 4 in play against a cap of 5. It did not move a verdict. The place the pressure would have shown is the `R2B-4` scope ruling, so that one is argued against all four conditions in full below rather than asserted, and it is the ONLY finding I ruled out of scope.

REPRODUCED FIRST-HAND (evidence re-run, not read): ALL ELEVEN. Nothing was judged on citation alone. Three findings required purpose-built fixtures (`R2B-1`, `R2B-2`, `R2B-4`), one required a measurement on the binary (`R2A-4`), and the rest were settled by opening the cited ranges and by `git show`/`git log -S` against the commits that wrote the sentences.

## Per-reviewer attribution, for the round record

| reviewer | lens | raw | valid |
| --- | --- | --- | --- |
| A | fix-induced residue (the round 1 fix diff only) | 5 | 4 |
| B | cold complete read of the sidecar | 5 | 5 |
| C | rendered-view reader | 3 | 3 |

Per-reviewer `valid_findings` credit each reviewer for its own raised finding, so the shared `R2A-1`/`R2C-1` and `R2A-3`/`R2C-2` are counted in BOTH A's 4 and C's 3 while counting ONCE in the round total of 10. The per-reviewer sum (12) is expected to exceed the round-level total (10); that is the convention `AGENTS.md` states for the `reviewers` array, and it is the same pattern round 1 recorded (13 against 11).

B is the round's productive lens by a clear margin: 5 raw, 5 valid, and three of its five sit in blocks NEITHER the build pass NOR the fix pass opened. A's lens was scoped to the fix diff by construction and returned 4 of 5, but its one dismissal and its one downgrade are both in the same finding family, which is worth noting for lens selection next round.

## Deduplication, confirmed on evidence

PAIR 1: `R2A-1` AND `R2C-1` ARE ONE FINDING. Both name `docs/plans/agent-scaffold.plan.toml:1732` (rendered `docs/plans/agent-scaffold.md:166`), both quote the same sentence, both assert the same defect (the `README.md:228` citation does not resolve to the quoted sentence, which is at `README.md:238`), and both prescribe the same one-token remedy (`228` to `238`). The subject, the site, the falsifying fact and the remedy are identical. They differ ONLY in the ground offered for severity, and that difference is what decides the severity below rather than the count.

A TRANSCRIPT DISCREPANCY BETWEEN THE TWO, settled by running it, because it goes to which reviewer's evidence I can rely on:

```
$ sed -n '228p' README.md
# --workflow would join /elsewhere/docs/plans/their-task.plan.toml against
$ sed -n '231p' README.md
# `--source` and `--plan` pair
```

`R2A-1`'s transcript is CORRECT. `R2C-1`'s transcript prints line 231's content under a `228p` command, so it is off by three lines. The substantive claim is unaffected: both lines are comment lines inside the same fenced shell example, and neither is the quoted sentence.

PAIR 2: `R2A-3` AND `R2C-2` ARE ONE FINDING. Both name `docs/plans/agent-scaffold.plan.toml:1728` (rendered `docs/plans/agent-scaffold.md:162`), both quote the same sentence, both identify the same surviving present-tense tail ("after the tier policy lands, meets a hard failure from a check the guidance still promises them"), both refute it with the same site (`pack/AGENTS.md:93`), and both prescribe a token-level re-tense. `R2A-3` adds the contrast with sidecar `:294`'s conditional form; `R2C-2` adds the rendered-view contrast with `docs/plans/agent-scaffold.md:1534`. Two supporting observations on one defect.

NO OTHER PAIR SURVIVES INSPECTION. `R2A-5` and `R2C-3` both concern sidecar `:304` and are NOT duplicates: `R2A-5` is about "the guarded half" losing its antecedent to the `R1C-3` deletion, `R2C-3` is about "succeed today" and "answer today" in the paragraph's FIRST sentence. Different clauses, different falsifying facts, opposite remedy classes, and they receive opposite verdicts. `R2A-4` and `R2B-3` both concern the `#[serde(skip)]` family but at different lines (`:195` and `:206`) with different claims. `R2B-2` and `R2B-3` are adjacent lines and separate claims.

---

# The four things that needed more than a verdict

## (1) THE FACTUAL CONFLICT: ROUND 2'S REVIEWER IS RIGHT AND ROUND 1'S TRIAGER IS WRONG

`owning_pid` EXISTS. It is at `src/checks.rs:561`, alive, called, and documented.

```
$ grep -n 'fn owning_pid\|const RUNNER_PREFIX\|fn nanos\|fn dead_pid\|struct WorktreeGuard\|fn pid_is_alive\|fn prune_orphan_worktrees' src/checks.rs
98:const RUNNER_PREFIX: &str = "agent-scaffold-checks-run-";
345:struct WorktreeGuard {
416:fn pid_is_alive(pid: u32) -> bool {
561:fn owning_pid(dir_name: &str) -> Option<u32> {
588:fn prune_orphan_worktrees(repo: &Path) {
1023:fn nanos() -> u128 {
1613:    fn dead_pid() -> u32 {

$ sed -n '555,563p' src/checks.rs
/// Parse the owning pid out of a runner worktree directory name of the form
/// `agent-scaffold-checks-run-{pid}-{nanos}-{seq}` (see `reserve_runner_worktree`).
/// Only the first `-`-separated segment after the prefix is read, so appending
/// further components never changes what this returns. Returns `None` when the name
/// does not carry a parseable pid, so the caller can skip reclamation
/// conservatively.
fn owning_pid(dir_name: &str) -> Option<u32> {
    dir_name.strip_prefix(RUNNER_PREFIX)?.split('-').next()?.parse().ok()
}
```

Round 1's triage said, at `R1C-6`: "The sibling `src/checks.rs:400-405` was correctly left alone; it now resolves to `fn git`, its `owning_pid` subject having been replaced", and named the two classes as "citations whose SUBJECT MOVED (`:862-871`, `fn scratch`, which still exists) and citations whose SUBJECT WAS REPLACED (`:400-405`, `owning_pid`, which no longer exists)". The second half of that is FALSE. `:400-405` does now resolve to `git`'s body, which is why the citation is stale, but the SUBJECT it names was not replaced: it moved from `:400` to `:561` and its documentation was rewritten around `reserve_runner_worktree`. A rewritten doc comment on a surviving function is the MOVED class, not the REPLACED class.

The ledger carries the same error at `docs/plans/agent-scaffold.ledger.md:553`: "that file holds TWO classes of stale citation, SUBJECT-MOVED (`fn scratch`, which still exists) and SUBJECT-REPLACED (`owning_pid`, which does not)".

I SAY THIS PLAINLY BECAUSE I WAS ASKED TO. AN ORCHESTRATOR DECISION PRODUCED A WORSE DEFECT THAN THE `low` FINDING IT WAS FIXING. `R1C-6`'s cost was that check 21b's "AND ONLY THOSE" was untrue of ONE citation, in the direction of a narrower disclosure. The authorised remedy narrowed check 21b with 23 authored words at sidecar `:346`:

> THE EXCLUSION IS THE REPLACED-SUBJECT CLASS ONLY: a `src/checks.rs` citation whose subject MOVED and still exists is re-pointed with the rest.

Check 21b is an acceptance criterion, so that sentence states a required post-condition. AT LEAST FIVE CITED RANGES IN `checks-runner-worktree-name-collision.md` MEET IT AND WERE NOT RE-POINTED. Every one opened at its cited range and its named subject located separately:

| sidecar line | cited range | named subject | what the range holds today | subject now at |
| --- | --- | --- | --- | --- |
| `:14` | `src/checks.rs:78` | `RUNNER_PREFIX` | `PathBuf,` (an import) | `:98` |
| `:14` | `src/checks.rs:848-852` | `nanos()` | a `Command` builder body | `:1023` |
| `:53`, `:61` | `src/checks.rs:400-405` | `owning_pid` | `git`'s `git_command().arg("-C")` body | `:561` |
| `:67` | `src/checks.rs:1438-1442` | `dead_pid()` | a `scratch("paths-skip")` test body | `:1613` |
| `:26` | `src/checks.rs:329-342` | `WorktreeGuard` | `impl From<io::Error> for RunError` | `:345` |

And beyond those five, the same property holds for `:72-77`, `:388-392`, `:400-402`, `:407-461`, `:425-428` and `:845-847`, whose subjects (`RUNNER_PREFIX`'s comment, the dependency-discipline comment, `owning_pid`'s comment, the pid-liveness gate with `pid_is_alive` at `:416` and `prune_orphan_worktrees` at `:588`, and `nanos()`'s doc comment) all still exist. The pass re-pointed exactly ONE `src/checks.rs` citation (`:862-871` to `:1037-1046`, `fn scratch`).

So the clause is untrue of at least five cited ranges rather than one, and it reopens in the opposite direction to `R1C-6`: it commits the increment to re-pointing citations in a file whose closure work `Q-55-currencyscope` put outside inc4, two sentences after the same paragraph says "pulling it in would widen a scope the human closed (`Q-55-currencyscope`)".

THE MINIMAL REMEDY AND ITS CLASS. Two steps, the first settled and the second a scope call the orchestrator owns, exactly as `R1C-6`'s triage said.

- STEP 1, SETTLED, DELETION-CLASS: delete the 23 authored words at sidecar `:346`. They are false of the tree, they define a class the file does not satisfy, and nothing else depends on them. This is a pure deletion.
- STEP 2, THE ORCHESTRATOR'S CALL, two shapes:
  - REVERT, DELETION-CLASS. Also revert the one `src/checks.rs` re-point at `checks-runner-worktree-name-collision.md:55`, from `:1037-1046` back to `:862-871`. Check 21b's "AND ONLY THOSE" is then true as originally written, and NOTHING IS AUTHORED at all. The orchestrator declined this in round 1 on the ground that reverting "would restore a citation that is stale for another reason and make the file worse for a reader". That ground is unchanged and is still real. What has changed is what it was weighed against: it was costed against a clause believed to be a true narrowing, and the clause is false.
  - DISCLOSE THE SINGLE EXCEPTION, AUTHORED, about twelve words. Name the one re-point concretely in the existing disclosure sentence and define no class, for example "except `:1037-1046`, re-pointed because `test-tmpdir-repo-assumption.md:35` already cites that exact range for that exact helper". I verified that ground: `test-tmpdir-repo-assumption.md:35` does cite `src/checks.rs:1037-1046` for `fn scratch(name)`, and `:1037-1046` is `fn scratch(name)` today.

I do not choose between the two, on the same reasoning `R1C-6`'s triage gave. I record that the first is deletion-class and the second is authored, and that this project has six measurements that an authoring fix pass manufactures the next round's finding while a deletion-class pass re-seeds nothing.

## (2) THE OUT-OF-SCOPE PRECEDENT, RULED AGAINST ALL FOUR CONDITIONS FOR EVERY FINDING

The precedent, set by an earlier increment's round 3 triager and described as binding on later loops: A VALID FINDING THAT IS OUT OF SCOPE FOR THE INCREMENT DOES NOT RESET THE CONVERGENCE STREAK, on four conditions, ALL required. (1) provenance predates the base commit; (2) no commit in range modifies the claim's lines; (3) INDEPENDENT SUBJECT, meaning the claim is not about what the increment changed AND the increment's change is not what falsified it; (4) no shared fix.

THE BASE COMMIT IS `363ac06` (2026-08-07, "docs: record the inc4 planner pass and the twin-sites decision"). The changed-line map for the sidecar over `363ac06..8a42b32`, taken from the hunk headers rather than asserted, is lines 44, 46, 52, 102, 129, 133, 139, 173, 189, 195, 201-202, 204, 206, 208, 225, 255, 257, 259, 273, 282, 294, 296, 300, 304, 308-309, 312, 339, 345-348, 352, 356-359, 365-366, 368-369, 374-377 and 382-388 of the current file. The plan TOML's changed lines are 1330 and 1719-1739.

| finding | (1) provenance predates base | (2) no in-range edit of the line | (3) independent subject | (4) no shared fix | RULING |
| --- | --- | --- | --- | --- | --- |
| `R2A-1`+`R2C-1` | YES (`6141549`, 2026-08-02) | NO, `102e328` edited `:1732` ("says" to "said") | NO | YES | IN SCOPE |
| `R2A-2` | NO, authored in range by `102e328` | NO | NO | YES | IN SCOPE |
| `R2A-3`+`R2C-2` | YES (`e019b83`, 2026-07-31) | NO, `102e328` edited `:1728` ("reads" to "read", "carry" to "carried") | NO | YES | IN SCOPE |
| `R2A-4` | YES (`75c962d`, 2026-07-31) | NO, `102e328` edited `:195` | NO | YES | IN SCOPE |
| `R2B-1` | YES (`0dac831`, 2026-08-01) | YES, `:157` untouched | NO | YES | IN SCOPE |
| `R2B-2` | YES (`8b0d88f`, 2026-07-31) | NO, `9b01f34` edited `:204` ("says it is" to "SAID it was") | NO | YES | IN SCOPE |
| `R2B-3` | YES (`75c962d`, 2026-07-31) | NO, `102e328` deleted the first clause of `:206` | NO | YES | IN SCOPE |
| `R2B-4` | YES (`a5786ae`, 2026-08-03) | YES, `:342` untouched | YES | YES | OUT OF SCOPE |
| `R2B-5` | NO, authored in range by `9b01f34` | NO | NO | YES | IN SCOPE |
| `R2C-3` | YES (`e019b83`, 2026-07-31) | NO, `102e328` edited `:304` | NO | YES | IN SCOPE |

SEVEN OF THE TEN FAIL CONDITION 2 OUTRIGHT, which is the cheapest and least arguable of the four: a commit in range modified the very line carrying the claim. Two more (`R2A-2`, `R2B-5`) fail condition 1, having been authored inside the range. Only `R2B-1` and `R2B-4` reach condition 3 at all.

CONDITION 3 FOR `R2B-1`, ARGUED. Its claim at sidecar `:157` is a present-tense statement about how the surfaces behave, in the file whose claim currency IS the increment, and it was falsified by inc2, which is THIS STEP'S OWN INCREMENT. Round 1's triage admitted `R1C-3` and `R1C-4` on exactly that reading of condition 3 ("a stale claim THE INCREMENT'S OWN CHANGE BROKE is in scope regardless of authorship, and inc2 is what added the `status --json` assertions"), and the human applied the same reading in deciding `Q-55-twinsites`. The reading is therefore settled in this task by two triage applications and one human decision, and I follow it rather than re-litigating it. Both limbs of condition 3 fail here: the claim IS about what the increment changed (the increment's declared subject is this file's stale claims, and `Q-55-currencyscope` item (1), as re-derived by the planner to sixteen sites, covers "everything inc2 and inc3 falsified"), and the falsifying change IS this step's own. IN SCOPE.

CONDITION 3 FOR `R2B-4`, ARGUED RATHER THAN ASSERTED, BECAUSE THIS IS WHERE A DOCUMENTATION-CURRENCY INCREMENT MAKES THE CONDITION DELICATE. The lazy move is to say that every stale claim in this file is "about what the increment changed", which would make condition 3 unsatisfiable here and the precedent inapplicable to inc4 by construction. That is not what the human drew. `Q-55-currencyscope`'s line is between claims THIS STEP FALSIFIED (in) and claims false for another cause (out, and explicitly declined as the third option that was put). `R2B-4`'s claim was never true: it is an ambiguity authored on 2026-08-03 for inc2, wrong on the day it was written, not drift any increment caused. So the second limb holds without argument, and the first limb holds too:

- No acceptance check inc4 authored reaches it. Check 21 governs "EVERY CITATION AND EVERY QUOTATION IN THIS FILE" and this defect is neither a citation nor a quotation. 21b governs the three named sidecars. 22 governs `Projection.plan`. 23 is the render-and-validate gate.
- Neither the build pass nor the fix pass opened `:342` or anything in its block, confirmed against the hunk map above.
- The falsity is a phrase reused for two structurally different placements, not a tense, a numeral, a citation or an inverted negative result, so it is outside every remedy class the increment applied.

The counter-argument, which is real and which I record rather than hide: `:282`'s own review question is "does every claim this step leaves behind match the tree it leaves behind", and that question is broader than `Q-55-currencyscope` and does reach check 19. I rule the human's closed scope governs over the file's own self-description, on the same ground the ledger states for the three declined items ("A WRITER MUST NOT PULL THEM IN, AND A REVIEWER MUST NOT RAISE THEIR CONTINUED PRESENCE AS A FINDING"), and because ruling otherwise would make condition 3 unsatisfiable for this increment and quietly retire a precedent the project recorded as binding.

THE GUARDS THE PRECEDENT CARRIES ARE APPLIED. The minimal fix for `R2B-4` is recorded below anyway; the totals report the category explicitly (IN-SCOPE VALID 9, OUT-OF-SCOPE VALID 1) rather than as a bare "clean"; and a reviewer may re-raise it next round. The out-of-scope ruling changes NOTHING about this round's outcome, because the round is new-valid on nine in-scope findings without it.

## (3) THE HALF-FIX PATTERN: ONE SYSTEMIC DEFECT, NOT SIX INDEPENDENT ONES, AND THE BOUND IS THE ONE ALREADY RECORDED AS (17)

Six deduplicated findings allege that a remedy repaired one clause and left an adjacent clause carrying the same falsity: `R2A-1`/`R2C-1`, `R2A-3`/`R2C-2`, `R2A-4`, `R2B-2`, `R2B-3` and `R2C-3`. (`R2A-5` alleged a seventh and is dismissed; see its entry.) The three that do NOT fit the pattern are `R2B-1` and `R2B-4`, which are in blocks neither pass opened, and `R2B-5`, which is the increment falsifying its own self-description.

I tested the three candidate explanations the brief names, and the answer is the third.

WAS THE REMEDY APPLIED TOO NARROWLY? NO. I checked every one of the eleven round-1 remedies against the diff and the writer applied each one exactly as prescribed, no more and no less. The one place the prescribed form did not fit a site, the writer refused to read in an authorisation and reported it, which produced `Q-55-w1figure`. The application is not the defect.

WAS THE PRESCRIBED REMEDY TOO NARROW? PARTLY, AND IN A WAY THAT IS ITSELF A SYMPTOM. `R2A-4` and `R2B-3` are the clearest: round 1's triage prescribed "exactly two token substitutions" at `:195` and "delete the first clause through 'anywhere', keeping the paragraph's opening and its `skip_serializing_if` half, WHICH IS TRUE" at `:206`. In both cases the prescription was bounded by the SENTENCE FRAGMENT the reviewer had quoted, and in `R2B-3`'s case the triage's own "which is true" was wrong: the `skip_serializing_if` PREMISE is true, but the INFERENCE the same clause draws from it ("so an `Option::None` serialises as an explicit `null` rather than vanishing") is false, and the two are one sentence.

WAS THE BRIEF THAT CARRIED IT TOO NARROW? YES, AND THIS IS THE ONE EXPLANATION THAT COVERS ALL SIX. Every round-1 remedy was a PER-SITE prescription derived from a PER-SITE finding, and the fix pass's reach was therefore bounded by the reviewers' enumeration rather than by the CLASS the enumeration was drawn from. That bound has two spellings, and the orchestrator has already recorded the second:

- SPELLING A, THE ENUMERATION BOUND. Where the round named a class ("re-tense the present-tense claims about the tree in this file", "delete the inverted negative result"), the brief still listed sites, and each list was short by one member of its own class. `R2A-4` is the sentence's tail; `R2B-3` is the same sentence's second half; `R2B-2` is the next sentence on the same line the build pass had already re-tensed; `R2C-3` is the third instance of the "today" class the fix pass corrected twice at `:257` and `:259` and missed at `:304`, a line it opened in the same commit.
- SPELLING B, THE FORM BOUND, ALREADY RECORDED AS DEFECT (17). Where the brief stated the FORM of the authorised edit ("change only the TENSE of each of the eight claims"), the form did not fit every site the decision covered. `R2A-1`'s falsity is a NUMERAL (a line number), which is the same shape as the waiver figure that produced (17) and `Q-55-w1figure`; `R2A-3`'s remaining falsity is a tense in a clause the round-1 enumeration did not name.

SO THE ANSWER TO THE QUESTION AS PUT: THE SAME OVER-NARROW BOUND EXPLAINS THE REST, AND THE REMEDY IS ONE RULE RATHER THAN SIX PATCHES. Defect (17)'s recorded cure is "WHEN A BRIEF STATES THE FORM OF AN AUTHORISED EDIT, CHECK THAT FORM AGAINST EVERY SITE THE DECISION COVERS BEFORE PUTTING IT TO THE HUMAN". The generalisation this round measures is one level out: WHEN A ROUND'S REMEDY IS A CLASS SWEEP, THE BRIEF MUST SCOPE IT TO THE CLASS OVER THE WHOLE ENCLOSING SENTENCE AND PARAGRAPH, NOT TO THE FRAGMENT THE FINDING QUOTED, AND MUST REQUIRE THE WRITER TO REPORT EVERY FURTHER SITE THE CLASS REACHES. Six of this round's ten valid findings are inside a sentence or a paragraph a round-1 remedy already opened, so a class-scoped sweep of the same six paragraphs would have closed all six in one pass.

The six patches are still owed individually, because the rule is prospective and the sites are false now. What the rule changes is the shape of the NEXT fix brief, and whether round 3 gets a seventh instance of this pattern.

## (4) `R2B-4`: MEASURED, AND IT IS WORSE THAN THE REVIEWER MEASURED

Acceptance check 19 at sidecar `:342`:

> A SECOND LAYOUT PINS THE LOG SIDE: `<root>/docs/metrics` a SYMLINK to a sibling directory, with the plan where it belongs, gives the same refusal and the same omission.

The FIRST layout in the same check uses the identical phrase, and accepted cost (ii) at `:257` spells that first layout out as "Where `<root>/docs/plans` is a symlink to `<root>/elsewhere`", so within this file "a sibling directory" means a sibling of `docs`, INSIDE the root. Built exactly that way on the log side, and the check returns the OPPOSITE of its stated result. Three layouts, one binary built at `8a42b32`, all under `<scratchpad>/triage-inc4-r2/f19/`, all outside any git repository (`git rev-parse --is-inside-work-tree` gives `fatal: not a git repository`), each project holding a ONE-record log so the record count identifies the file that was read:

```
# LAYOUT 1 (control), the PLAN side, the check's own first layout: sym1/docs/plans -> sym1/elsewhere
$ agent-scaffold validate --source .../f19/sym1/docs/plans/p.plan.toml --workflow
--workflow would join .../f19/sym1/docs/plans/p.plan.toml against .../f19/sym1/docs/metrics/workflow.jsonl,
which is not under the plan's project root .../f19/sym1/elsewhere; pass a `--metrics` under that root, ...
exit=1                                                          <- REFUSED, as the check says
$ agent-scaffold status --source .../f19/sym1/docs/plans/p.plan.toml
plan: 1 steps (1 complete); 0 open-questions items
metrics: unavailable, the round log ... is not under the plan's project root .../f19/sym1/elsewhere, ...
exit=0                                                          <- OMITTED, as the check says

# LAYOUT 2a, THE CHECK AS WRITTEN: sym2a/docs/metrics -> sym2a/elsewhere (a sibling of `docs`, INSIDE the root)
$ agent-scaffold validate --source .../f19/sym2a/docs/plans/p.plan.toml --workflow
.../f19/sym2a/docs/metrics/workflow.jsonl: 1 records, valid
.../f19/sym2a/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
.../f19/sym2a/docs/plans/p.plan.toml vs .../f19/sym2a/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0                                                          <- NOT REFUSED. A FULL GREEN.
$ agent-scaffold status --source .../f19/sym2a/docs/plans/p.plan.toml
plan: 1 steps (1 complete); 0 open-questions items
metrics: 1 records                                              <- NOT OMITTED. The log was read and counted.

# LAYOUT 2b, WHAT THE SUITE BUILDS: two/docs/metrics -> two-metrics (a sibling of the ROOT, OUTSIDE it)
$ agent-scaffold validate --source .../f19/two/docs/plans/p.plan.toml --workflow
--workflow would join ... which is not under the plan's project root .../f19/two; ...
exit=1                                                          <- REFUSED
$ agent-scaffold status --source .../f19/two/docs/plans/p.plan.toml
metrics: unavailable, the round log ... is not under the plan's project root .../f19/two, ...
exit=0                                                          <- OMITTED
```

IT REPRODUCES, AND MORE STRONGLY THAN REPORTED. `R2B-4` measured layout 2a at exit 1 with a W3 verdict, because its own fixture's record carried a W3 problem. With a clean record the same layout gives `workflow invariants hold` AT EXIT 0. So a reviewer who builds the layout the check names gets not merely "not the refusal" but a positive assertion that the invariants hold, plus a counted log on `status`. That is the false-green shape this whole step exists to remove, appearing in the check that pins the step's accepted cost.

IS THE CHECK WRONG, OR DOES THE SUITE BUILD A DIFFERENT LAYOUT? BOTH, AND THE CHECK IS THE ODD ONE OUT. The suite builds the OUT-OF-ROOT layout and says so in its own comment:

```
$ sed -n '1618,1623p' tests/unsafe_pairings_are_refused_and_omitted.rs
    // Layout 2, the LOG side: `<root>/docs/metrics` is a symlink out of the root.
    let two = root.join("two");
    write(&two.join("docs").join("plans").join("p.plan.toml"), &plan_toml("complete"));
    write(&root.join("two-metrics").join("workflow.jsonl"), &log(&["borrowed-step"]));
    fs::create_dir_all(two.join("docs")).unwrap();
    symlink(&root.join("two-metrics"), &two.join("docs").join("metrics"));
```

WHICH OF THE TWO THE SIDECAR'S OTHER SITES AGREE WITH: THE SUITE. Every other site in the file and both shipped documents state the general rule, and the general rule is the out-of-root one.

- Sidecar `:257`, accepted cost (ii) itself: "THE COST IS THE DIVERGENCE AND NOT THE LAYOUT: any symlink that makes the canonicalised checked plan and the canonicalised resolved log fall under different roots produces it, on either side, and `docs/plans` is the placement that was MEASURED rather than the population."
- `README.md:236`: "A layout where `docs/plans` or `docs/metrics` is a symlink pointing somewhere the other one is not under will now be refused".
- `CHANGELOG.md`, `## [Unreleased]` / `Changed`: "a layout in which `docs/plans` or `docs/metrics` is a symlink that lands the plan and the log under different real roots".

So the tool is right, the suite is right, the accepted cost is right, and check 19's second layout is the only site that reuses one phrase for two structurally different placements.

RECORDED, BECAUSE IT BEARS ON SEVERITY AND ON WHETHER THIS IS NEW. The project already holds this measurement. Ledger `:517` records inc2's judgement call (c): "The implementer read check 19's 'a SYMLINK to a sibling directory' as a sibling of the ROOT; the triager ran BOTH readings and found that on the LOG side an in-root symlink target is NOT refused at all, so only the root-sibling reading can pin accepted cost (ii) there ... Right layout on each side, nothing owed." That ruling settled which layout the IMPLEMENTER should build. It did not correct the check's wording, and no finding was filed against it, so the wording defect has been standing and undeclared since 2026-08-03. It is not on the recorded-residuals list I was given (inc2's four and inc3's four), so it is not dismissible on that ground, but it does mean this is a known fact in a new place rather than a discovery.

SEVERITY medium CONFIRMED, on round 1's own recorded reasoning for `R1C-4`: "A reader sent to verify an accepted cost and finding the opposite behaviour is the failure this section exists to prevent." Not `high`: the suite builds the right layout and passes, no gate is broken, and no behaviour is wrong.

MINIMAL REMEDY: TOKEN-LEVEL, four words, in the second layout only. "a SYMLINK to a sibling directory" becomes "a SYMLINK out of the plan's project root", matching the test's own comment and `:257`'s general statement. No new fact is authored.

---

# The findings

## `R2A-1` + `R2C-1` (SEVERITY CORRECTED DOWN, medium to low): VALID, fix required. ONE FINDING. IN SCOPE.

REPRODUCED, and the reproduction is what moves the severity.

THE CITATION DOES NOT RESOLVE TODAY. `docs/plans/agent-scaffold.plan.toml:1732` (rendered `docs/plans/agent-scaffold.md:166`) reads "`README.md:228` said 'Unlike `validate` it never fails on a missing or malformed file ...'". `README.md:228` is a comment line inside a fenced shell example; the sentence is at `README.md:238`. Both confirmed above. The rendered view carries `README.md:228` at `:166` and `README.md:238` at `:1568`, `:1754`, `:1761` and `:1764`.

`R2A-1`'S LOAD-BEARING SEVERITY ARGUMENT DOES NOT REPRODUCE, AND `R2B`'S OBSERVATION 1 HAS THE CONTROL THAT SETTLES IT. `R2A-1` argues that "said" does not make the sentence true either, because the paragraph is stamped 2026-07-31 and the sentence was at line 226 on that day. That measures the date the PARAGRAPH was stamped, not the date the CITATION was written, and the discriminating control was available and not run:

```
$ git log --format='%h %ad %s' --date=short -S'README.md:228' -- docs/plans/agent-scaffold.plan.toml
6141549 2026-08-02 docs: cite src/main.rs and src/next.rs by symbol rather than by line number
$ git log --format='%h %ad %s' --date=short -S'README.md:226' -- docs/plans/agent-scaffold.plan.toml
6141549 2026-08-02 docs: cite src/main.rs and src/next.rs by symbol rather than by line number
e019b83 2026-07-31 docs: fold the refusal-scope decision and schedule the TMPDIR suite defect
```

The paragraph was written on 2026-07-31 citing `:226`, which was correct then. A deliberate citation-currency commit refreshed it to `:228` on 2026-08-02, and the sentence sat at line 228 from `609ddcf` (08-01) to `b236b10` (08-03). So "`README.md:228` said ..." is TRUE of a real window, and the rendered view's two line numbers for one quotation are not a contradiction once the tenses are read: `:166` says "said" of `:228` and `:1568` says "does not merely promise ... it says" of `:238`, which is exactly what a sentence that moved looks like. This is the ADJUDICATION failure mode named in my brief, applied to a date axis: the right dimension was varied (the reviewer printed the whole line-number history) and then ruled on by asking about the paragraph's stamp rather than about what the fix pass's edit REMOVED.

WHAT SURVIVES, AND IT IS A DEFECT. The citation does not resolve, in an increment whose declared subject is claims that no longer match the tree, and THE SAME PASS CORRECTED THE SAME CITATION TO `:238` AT SIDECAR `:173`. One artifact, one quotation, two line numbers, one of them corrected by this pass and its twin left. That is the recorded twin-site failure mode, and check 21's own rule states the action for this input ("A citation whose subject moved is RE-POINTED at the subject"), even though check 21 is scoped to the sidecar and does not govern the `Q-55` `ask`.

SEVERITY low, CORRECTED DOWN from both reviewers' medium. The sentence carries no false assertion; no reader is misled about the tool or the tree; no acceptance check governs the site; and the correcting sibling is four lines away in the rendered view. What it costs a reader is a citation that lands in a code fence, which is `low`.

MINIMAL REMEDY: TOKEN, one substitution, `228` to `238` at `docs/plans/agent-scaffold.plan.toml:1732`, then re-render. Nothing is authored, and "said" plus `:238` is coherent, so the tense need not move again.

NOTE FOR THE ORCHESTRATOR, NOT A CONDITION OF THE VERDICT. `Q-55-receiptcurrency` authorised a TENSE change on these eight sites, and this site's remaining falsity is a NUMERAL, which is the (17) shape. The ledger's own ground for `Q-55-w1figure` reaches it without a fresh option set: "the append convention protects the decision-time REASONING", and a citation re-point revises no reasoning. Whether that is enough to act without going back to the human is the orchestrator's call, and it is the same call, on the same decision, with the same answer already recorded once.

## `R2A-2` (medium confirmed): VALID, fix required. IN SCOPE.

REPRODUCED IN FULL. See item (1) above for the evidence, the five-plus qualifying citations, the falsified premise, and the two-step remedy.

SEVERITY medium CONFIRMED. Check 21b is an acceptance criterion the round uses to settle whether inc4 is done, and as narrowed the increment does not meet it. Not `high`: nothing behavioural, no gate broken, and a reader of the sidecar is not misled about the tool. Not `low`: `R1C-6` was `low` because its cost was one citation and a narrower disclosure; this is at least five and it moves a boundary the human closed in the widening direction.

MINIMAL REMEDY: DELETION of the 23 authored words at sidecar `:346`, plus a scope call between a further DELETION (revert the one re-point) and an AUTHORED twelve-word disclosure. Full statement in item (1).

## `R2A-3` + `R2C-2` (medium confirmed): VALID, fix required. ONE FINDING. IN SCOPE.

REPRODUCED. `docs/plans/agent-scaffold.plan.toml:1728` (rendered `docs/plans/agent-scaffold.md:162`):

> (1) The DOCUMENTATION half of SE-3 is IN SCOPE: the two-tier split was undocumented in the scaffolded AGENTS.md, so a non-instrumented user read an unconditional promise of the `validate --workflow` backstop (`pack/AGENTS.md:93`) and, after the tier policy lands, meets a hard failure from a check the guidance still promises them.

The tail is false of the tree. `pack/AGENTS.md:93` and the deployed `AGENTS.md:93` both now carry "when instrumentation is on, the deterministic `validate --workflow` check is the backstop ... and on a project with no round log yet, which every project scaffolded without `--instrument` remains, that check exits non-zero reporting that it could not run rather than passing". The unconditional form is absent from both files:

```
$ grep -cF 'the deterministic `validate --workflow` check, once built, is the backstop' pack/AGENTS.md AGENTS.md
pack/AGENTS.md:0
AGENTS.md:0
```

That is defect D, which inc3 closed and acceptance check 20 pins. So the decision record asserts in the present tense that a defect this step closed is open.

THE PROSPECTIVE-MOOD DEFENCE DOES NOT CARRY, AND THE FIX PASS ITSELF IS THE REASON. The writer's ground for leaving the clause was that it "was a prediction written before the policy landed". The same pass, in the same commit, converted the exactly parallel prospective clause at sidecar `:255` from "After the tier policy lands, this case becomes a HARD FAILURE" to "After the tier policy landed, this case became a HARD FAILURE". So the pass already ruled that a prediction the work averted is re-tensed rather than preserved, and applied that ruling once and withheld it once. The sidecar's own conditional form at `:294` ("a tier policy WITHOUT its qualifier leaves a non-instrumented user hitting a hard error from a check the guidance still promises them unconditionally") stays true precisely because it is a counterfactual; the `Q-55` version is not.

SEVERITY medium CONFIRMED, on the same ground round 1's triage rated `R1A-3`/`R1C-2` medium: a present-tense claim about the tree, in a sentence whose neighbouring verbs the same pass re-tensed, which a reader can only reconcile by supplying a tense the sentence does not have. Not `high`: nothing computes on it and the correct statement is in the same document at `docs/plans/agent-scaffold.md:1534`.

MINIMAL REMEDY: TOKEN, a re-tense of the tail, the same class the pass applied twice in this sentence and once at sidecar `:255`. "after the tier policy lands, meets" to "after the tier policy landed, would have met", and "the guidance still promises them" to "the guidance still promised them". No fact is added.

TWIN-SITE WARNING, RECORDED SO THE FIX DOES NOT MANUFACTURE THE NEXT INSTANCE, AND NOT RAISED AS A FINDING BY ME (no reviewer raised it and a triager does not raise findings). Sidecar `:8`, the defect D bullet at the head of the file, still reads "The scaffolded `AGENTS.md` promises the `validate --workflow` backstop without qualification", unqualified present tense, in a block (`:1` to `:43`) neither pass opened. A remedy at plan TOML `:1728` alone leaves the plan asserting defect D closed at `:139` and open at `:8`. `R2A-3` names this and correctly declines to claim it; B's cold read covered the file and did not raise it either, so it is unruled either way and a round-3 reviewer may reach it.

## `R2A-4` (low confirmed): VALID, fix required. IN SCOPE.

REPRODUCED, both halves. Sidecar `:195` now reads "`no_active_loop_reason` WAS `#[serde(skip)]` ... and `status`'s `Projection` HAD no reason field at all, so under `--json` an omitted part serialises as a bare `null` with nothing distinguishing why."

The two re-tensed premises are true of the past, checked against the commit that wrote the sentence rather than assumed:

```
$ git show 75c962d:src/next.rs | grep -n -B3 'no_active_loop_reason' | head -4
114-    /// Why there is no active loop, for the human renderer. Not serialised (the JSON
115-    /// contract is exactly the fields above); recomputed each call, never stored.
116-    #[serde(skip)]
117:    pub(crate) no_active_loop_reason: Option<String>,
$ git show 75c962d:src/main.rs | sed -n '/^struct Projection/,/^}/p'
struct Projection {
    /// The plan projection, present only when a readable `--plan` was given.
    plan: Option<PlanProjection>,
    /// The metrics summary, present only when the metrics log exists.
    metrics: Option<MetricsProjection>,
}
```

The conclusion is false of the present, measured on the binary at `8a42b32` rather than read, in a fixture outside any repository:

```
$ agent-scaffold next --json --source <away>/docs/plans/p.plan.toml --metrics docs/metrics/workflow.jsonl
{
  "task": "p", "metrics": null, "metrics_absent_reason": "log-not-this-project",
  "active_loop": null, "resume_state": null, "resume_state_absent_reason": "ledger-absent",
  "no_active_loop_reason": "metrics-not-this-project"
}
exit=0
```

An omitted part serialises as `null` WITH a reason distinguishing why, on both surfaces, which is the whole point of `Q-55-jsonreason`.

SEVERITY low CONFIRMED. The two capitalised past-tense verbs two clauses earlier carry the whole sentence historically, and the paragraph is framed "THE PROBLEM, in the form that decided it", so a careful reader is not misled. It is a finding because the fix pass's own act of re-tensing the premises is what left the dependent conclusion stranded, and because it is checkable in one command.

MINIMAL REMEDY: TOKEN, one substitution, "serialises" to "serialised". Nothing is authored.

## `R2A-5` (low confirmed): DISMISSED

REPRODUCED, and the reproduction is what dismisses it. The finding is that the `R1C-3` deletion left "the guarded half" at sidecar `:304` without an antecedent, entailing that `status --json` is the unguarded half, which is the claim the deletion removed.

THE ANTECEDENT SURVIVES, IN THE SAME SENTENCE PAIR, NOT FOURTEEN LINES AWAY. The word-diff shows exactly what the deletion took and what it left:

```
$ git diff --word-diff=porcelain 9b01f34 HEAD -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
@@ -301,11 +301,11 @@
-golden, and one of the two commands (`status --json`) has no test on its serialisation at all, so that half is carried by the acceptance check rather than by the suite.
+golden.
```

So the surviving text reads "the increment now also changes a DOCUMENTED JSON CONTRACT on two commands, falsifying four doc comments and BREAKING A BYTE-COMPARE GOLDEN. None of this changes the class ... but it widens what the two rounds have to cover and a reviewer who checks only `next --json` has checked the guarded half." "Breaking a byte-compare golden" is the immediately preceding clause, `next --json` is the half that has the golden (`GOLDEN_JSON` at `src/next.rs:2077`, asserted at `:2139`, and there is no `status` golden), and sidecar `:208` states the surviving half of the same fact in the PRESENT tense and TRUE: "`status --json` has NO golden".

So the sentence carries no false assertion. What the finding relies on is an implicature ("guarded" implies its complement is unguarded), and the paragraph's own text defeats it by supplying the grounding that makes the term true. Removing a FALSE grounding and leaving a term whose remaining grounding is TRUE is a correct outcome, not a residue.

I LOOKED FOR THE OTHER ANSWER AND SAY SO. The reviewer's own case for calling it a defect is that the clause is doing live work (telling a reviewer that checking one surface is not enough) through a term whose plain reading is now wrong. The work IS live and is done correctly under the golden reading. And the remedy edits a closed increment's risk paragraph for no reader gain, which is the class this project measures as re-seeding, and is the same shape round 1's triage dismissed `R1A-5` on ("a misreading the cited site already prevents").

DISMISSED. `low`, so the high/critical backstop is not engaged.

## `R2B-1` (medium confirmed): VALID, fix required. IN SCOPE.

REPRODUCED on a purpose-built fixture. Sidecar `:157`, present tense, in the section neither pass opened:

> Where NO plan is read there is no root, so the predicate does not fire and every surface behaves as it does today, which is the answer the no-anchor case above already gets; on `validate --workflow` that case is the match's own `(None, None, _)` arm, already a hard problem for its own reason.

The `validate --workflow` half is true. The "every surface" half is false. MEASURED on a Markdown-primary `--source` with NO `--plan`, which is the "no plan is read" configuration on `status` and `next`, with an explicit `--metrics` naming another project's log, from inside the fixture and outside any repository:

```
$ agent-scaffold status --source docs/plans/p.plan.toml --metrics <foreign>/docs/metrics/workflow.jsonl --json
{
  "plan": null,
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project"
}
exit=0

$ agent-scaffold next --source docs/plans/p.plan.toml --metrics <foreign>/docs/metrics/workflow.jsonl --json
{
  "task": "p", "source": "no plan source", "metrics": null,
  "metrics_absent_reason": "log-not-this-project", "active_loop": null,
  "resume_state": null, "resume_state_absent_reason": "ledger-absent",
  "no_active_loop_reason": "no-plan-steps"
}
exit=0
```

`"plan": null` and `"source": "no plan source"` are the tool saying it read no plan. `log-not-this-project` is the predicate firing anyway, on a root supplied from the anchors. (My `no_active_loop_reason` differs from the reviewer's because my fixture has no readable Markdown plan and so no steps; nothing in the finding turns on it.)

THE PROJECT'S OWN RECORD SAYS THE SENTENCE IS THE PRE-FIX DESIGN. `src/main.rs:containment_roots`'s doc comment states the anchor fallback for `status` and `next` explicitly, and ledger `:509` records the amendment: "there is ONE PREDICATE AND TWO ROOT-SUPPLY POLICIES, and the second (`resume_roots`, decided by `Q-55-resumepairing` for 'the surface that reads no plan') is INCOMPLETELY APPLIED, because `next` and `status` also read no plan in that configuration and never switch to it ... RECORD THE AMENDED FORM". The amended form was recorded in the ledger and in the code and never carried back into the sidecar, which is the durable design record the queued validation-constraints step inherits (`:269` and `:271` route work to it from this very section).

SEVERITY medium CONFIRMED. No behaviour is wrong and the shipped prose gets it right (`README.md:236` and the `CHANGELOG.md` `Changed` entry both state the anchor fallback for all three surfaces), so no user is misled. What earns medium rather than low is that the sentence asserts the ABSENCE of a guard that is present, in the exact configuration whose unguarded form was found at `high` inside inc2 (`ADV-1`), in the file a later step inherits as its specification.

MINIMAL REMEDY: TOKEN, a re-tense so the paragraph reads as the design as specified before `ADV-1` amended it, matching what the pass already did at `:255`. "so the predicate does not fire and every surface behaves as it does today" becomes "so the predicate did not fire and every surface behaved as it did then". No fact is added and the amended form is already recorded in `containment_roots`'s doc comment, so nothing needs authoring here. DELETION is NOT available as the cheaper class: the preceding clause "the predicate does not fire" carries the same falsity, so deleting only the "every surface" clause leaves the sentence false and deleting both leaves it without a subject. That is worth stating because this project's remedy-class preference would otherwise reach for deletion first.

## `R2B-2` (medium confirmed): VALID, fix required. IN SCOPE.

REPRODUCED. Sidecar `:204` asserts an exhaustive cause list:

> `active_loop` is `None` ONLY when there are no steps or when every step is terminal.

MEASURED FALSE on a plan with ONE step at `in-progress`, which is neither stated cause, paired with an unpairable round log:

```
$ agent-scaffold next --json --source <away>/docs/plans/p.plan.toml --metrics docs/metrics/workflow.jsonl
{
  "task": "p", "metrics": null, "metrics_absent_reason": "log-not-this-project",
  "active_loop": null, "resume_state": null, "resume_state_absent_reason": "ledger-absent",
  "no_active_loop_reason": "metrics-not-this-project"
}
exit=0
```

The file specifies the third cause nineteen lines later at `:223` ("`metrics-not-this-project`, the NEW case"), and `src/next.rs` now agrees with `:223` rather than with `:204`.

THE PASS TOUCHED THIS LINE AND STOPPED ONE SENTENCE SHORT, confirmed against the diff rather than asserted:

```
$ git diff -U0 363ac06 HEAD -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md   # at @@ -204 +204 @@
-... `active_loop`'s doc comment ... says it is `None` when "all steps complete, ..."
+... `active_loop`'s doc comment ... SAID it was `None` when "all steps complete, ..."
```

NOT THE EXCLUDED ITEM, AND I CHECKED BEFORE RULING. `Q-55-currencyscope` and my own brief put `src/next.rs:162` and `:181-183` out of scope. Those are the CODE's doc comments. This is the SIDECAR's own assertion at `:204`, a different artifact, inside the file check 21 governs, falsified by this step's own inc2, on a line a commit in range modified.

SEVERITY medium CONFIRMED. It is an exhaustiveness claim about derived output, which is the class this step's own human-authorised round-3 sweep ruled must be DELETED rather than narrowed, and the file states both the claim and its refutation nineteen lines apart. Not `high`: nothing computes on it and the code comment is now right.

MINIMAL REMEDY: DELETION. Delete the sentence. The paragraph's point (the blocked-steps case is not a cause of `None`, so do not add a variant for it) is carried entirely by the sentence before it and the two after it, which I checked by reading the paragraph without it. Deleting loses nothing and re-seeds nothing.

## `R2B-3` (medium confirmed): VALID, fix required. IN SCOPE.

REPRODUCED. Sidecar `:206` as it now stands, after the `R1A-2`/`R1C-1` deletion:

> WHAT THE SWEEP FOUND NOTHING OF ... No `skip_serializing_if` appears in either `src/next.rs` or `src/main.rs`, so an `Option::None` serialises as an explicit `null` rather than vanishing (visible in the golden as `"resume_state": null`), and the new field must follow that convention: ALWAYS PRESENT, `null` in the normal case.

The premise is true and the inference is false:

```
$ grep -rn 'skip_serializing_if' src/ | grep -c 'next.rs\|main.rs'
0
$ grep -rn 'serde(skip)' src/
src/next.rs:198:    #[serde(skip)]
src/next.rs:202:    #[serde(skip)]
$ git log --format='%h %ad %s' --date=short -S'metrics_absent_note' -- src/next.rs
8beb1c2 2026-08-03 feat: refuse and omit on a round log or ledger the plan cannot vouch for
```

`:199` is `metrics_absent_note: Option<String>` and `:203` is `resume_state_absent_note: Option<String>`, both added by inc2, both dropped from the JSON entirely. Measured on the binary: in the `R2B-1` and `R2B-2` runs above, `metrics_absent_reason` is `log-not-this-project`, which by the fields' own doc comment means `metrics_absent_note` is `Some`, and no `metrics_absent_note` key appears in the output at all. So a `Some` vanishes as well as a `None`.

THIS IS THE SURVIVING HALF OF THE SENTENCE `R1A-2`/`R1C-1` HALF-FIXED, AND THE PRESCRIPTION IS WHERE IT STOPPED. Round 1's triage prescribed deleting the first clause and keeping "its `skip_serializing_if` half, WHICH IS TRUE". The PREMISE is true; the INFERENCE the same clause draws from it is not, because `#[serde(skip)]` drops a field just as `skip_serializing_if` would. The fix pass did exactly what it was told.

SEVERITY medium CONFIRMED, on round 1's own recorded reasoning for the twin: "a recorded NEGATIVE RESULT that has silently inverted is worse than a stale positive claim, because its whole function is to let a later reader skip a search". That ground applies unchanged to the half that stayed.

MINIMAL REMEDY: DELETION. Cut "so an `Option::None` serialises as an explicit `null` rather than vanishing (visible in the golden as `"resume_state": null`), and", leaving "No `skip_serializing_if` appears in either `src/next.rs` or `src/main.rs`, and the new field must follow that convention: ALWAYS PRESENT, `null` in the normal case." The instruction to the implementer survives, the falsifiable generalisation goes, nothing is authored.

## `R2B-4` (medium confirmed): VALID, minimal fix recorded. OUT OF SCOPE.

REPRODUCED and ruled in full in item (4) above, including the scope ruling against all four conditions in item (2). Summary of the ruling: the fact is measured and worse than reported (a full `workflow invariants hold` at exit 0 and a counted log, not merely the absence of a refusal); the check text is the odd one out against the suite, `:257`, `README.md:236` and `CHANGELOG.md`; the claim was NEVER TRUE and this step's changes are not what falsified it; the minimal remedy is TOKEN-LEVEL, four words.

OUT OF SCOPE DOES NOT MEAN NOT REAL. The fix is recorded, the category is reported explicitly in the totals rather than folded into a "clean", and a round-3 reviewer may re-raise it. It does not reset the convergence streak, and in this round that changes nothing, because the streak is already reset by nine in-scope findings.

## `R2B-5` (low confirmed): VALID, fix required. IN SCOPE.

REPRODUCED. Sidecar `:282` says of inc4 "`Projection.plan`'s false doc comment at `src/main.rs:Projection` (`Q-55-plandoccurrency`), WHICH IS THE INCREMENT'S ONE SOURCE CHANGE", and `:385` repeats it ("the increment's one source change"). The increment made three comment corrections across two files:

```
$ git show --stat --format='%h %s' 297bfce
297bfce docs: correct three stale comment claims for inc4
 src/main.rs                                      | 2 +-
 tests/unsafe_pairings_are_refused_and_omitted.rs | 5 ++---
```

The two test edits are real and correct, and I read the diff rather than the subject line: `tests/unsafe_pairings_are_refused_and_omitted.rs:156` lost the count word from "the first of inc2's four owed red-then-green cases", and `:1369-1370` lost "and no test on its serialisation at all". Those are the two twins `Q-55-twinsites` authorised, so the edits were decided rather than smuggled; what is wrong is the sidecar's own count of them.

THE CLAIM WAS TRUE WHEN WRITTEN AND THE INCREMENT FALSIFIED IT, which is what makes it the increment's own failure mode rather than a planning error: `9b01f34` authored `:282`, and `297bfce` made the test edits afterwards on the same branch.

THE DOCUMENTATION-IMPACT LIST HAS THE SAME GAP and reads as exhaustive because it enumerates its exclusions ("NOT `README.md`, NOT `pack/AGENTS.md` ... NOT `CHANGELOG.md`"). `tests/unsafe_pairings_are_refused_and_omitted.rs` is in neither the positive list nor the negative one, and I confirmed the consequence: check 21 governs "this file", 21b the three named sidecars, 22 `Projection.plan`, 23 the render-and-validate gate, so NO acceptance check states those two edits.

SEVERITY low CONFIRMED. Both edits are correct, no reader is misled about the tool, and nothing downstream computes on the count. It is a finding because "the increment's one source change" is a false enumeration the pass authored about itself, which is `:308`'s own named failure mode landing on the pass's self-description.

MINIMAL REMEDY, IN TWO PARTS WITH DIFFERENT CLASSES. The count is DELETION-class: strike "which is the increment's one source change" at `:282` and ", the increment's one source change," at `:385`. Both sentences stand without it and check 22 is still named. The documentation-impact gap is AUTHORED, one bullet naming the test file and its two comment corrections; the alternative is to accept it as a residual on the ground that the ledger records the edits under `Q-55-twinsites`. I record the split rather than choosing, because the deletion half is unarguable and the authored half is a judgement about how complete that list must be.

## `R2C-3` (SEVERITY CORRECTED DOWN, medium to low): VALID, fix required. IN SCOPE.

REPRODUCED. Sidecar `:304`, the inc2 risk-classification paragraph:

> It INTRODUCES a non-zero exit on validator invocations that SUCCEED TODAY AND withholds output from projection invocations that ANSWER TODAY, and it does so with a MEASURED FALSE POSITIVE already in hand (accepted cost (ii), the symlinked `docs/plans` directory) ...

Inc2 has shipped, and its effect on the named example is that such invocations no longer succeed. My layout 1 in the `f19` fixture above IS that example, and it exits 1 with the containment refusal on `validate --workflow` and omits the metrics half on `status` at exit 0.

THE SAME PASS CORRECTED THE IDENTICAL CLAIM SHAPE TWICE, IN THE SAME FILE AND THE SAME COMMIT, AND MISSED THE THIRD ON A LINE IT OPENED:

```
$ git diff --word-diff=porcelain 9b01f34 HEAD -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
@@ -252,11 +252,11 @@
-works today,
+worked before inc2,
-greens today
+greened before inc2
@@ -301,11 +301,11 @@
-golden, and one of the two commands (`status --json`) has no test ...
+golden.
```

`:257` and `:259` were re-tensed; `:304` was opened for a different deletion in the same hunk run and its "today" pair was left.

SEVERITY low, CORRECTED DOWN from the reviewer's medium. The reviewer rated it "consistent with `R1C-4`'s own severity", but round 1's triage was explicit that `R1C-4`'s medium came from member (a) alone and that "taken alone (b) and (c) are `low`, because each paragraph's next clause states the correct post-change behaviour". This is the (b)/(c) shape, on the same accepted cost, and it carries an extra mitigation those two did not: the clause is subordinate to "It INTRODUCES", a verb the pass also left and which frames the whole sentence at classification time, so a reader has a stronger cue here than at `:257` that the sentence describes the pre-inc2 state. That is `low`.

MINIMAL REMEDY: TOKEN, "succeed today" to "succeeded before inc2" and "answer today" to "answered before inc2", matching the phrasing already used at `:257` and `:259` for the same fact. No new prose, no restructuring.

---

## Mechanical gates, run first-hand in this worktree

```
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s)

$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date                                   exit=0

$ agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 289 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold   exit=0
```

Both gates are green and neither can catch any finding in this round: the rendered view is a faithful render of a source that disagrees with itself, which is the property round 1 recorded for `R1C-5` and which holds again for `R2A-1`/`R2C-1`.

## Recorded residuals and settled findings, checked against every finding before ruling

I checked all thirteen raw findings against the eight recorded residuals (inc2's four: the in-root bound; the single-anchor `..` case with its widened bound; `ADV-2`'s rejected-ledger context slot; the inc2-era `R2A-2`'s off-convention `--source` surface. Inc3's four: `R3A-1`'s inert remedy clause; `R4A-1`'s reader-level discrimination; the plain-`validate` mode-000-file-versus-unsearchable-directory inconsistency; the containment TOCTOU). NONE OF THE THIRTEEN IS A RE-RAISE OF ANY OF THE EIGHT.

I checked all thirteen against round 1's four dismissals (`R1A-5`, `R1A-7`, `R1A-8`, `R1B-3`). NONE IS A RE-RAISE, and no reviewer brought new evidence against any of those four verdicts.

Nothing was raised on `run_validate`'s "`--plan` still clap-required" claims, on `src/next.rs:162` or `:181-183`, or on the Status narrative at `docs/plans/agent-scaffold.md:7`. `R2B-2` explicitly tested itself against the `src/next.rs` exclusion before raising and was right to conclude it does not apply. No finding concerns line length or prose wrapping.

ROUND 1'S SETTLED FINDINGS ARE NOT RE-OPENED, with one necessary exception that is a correction of FACT rather than of verdict: `R1C-6`'s VERDICT (valid, `low`, fix required) stands untouched, and what is corrected is a factual premise inside its reasoning (`owning_pid` exists), on new evidence, which is exactly the condition the triager prompt names for re-adjudicating a settled matter. The finding whose verdict this affects is `R2A-2`, which is a new finding against the remedy, not a re-opening of `R1C-6`.

## What this triage varied, and what it held fixed

VARIED. Plan substrate (TOML-primary, Markdown-primary, no plan resolved at all). Layout (conventional; `docs/plans` symlinked to an in-root sibling; `docs/metrics` symlinked to an in-root sibling; `docs/metrics` symlinked out of the root). `--metrics` state (anchored default, explicit relative, explicit naming another project's log). Step status (`in-progress`, `complete`). Surface (`validate --workflow`, `status`, `status --json`, `next`, `next --json`). Record content (a clean converged record and, by contrast with the reviewer's transcript, a W3-problem record, which is what showed layout 2a's result is a full green rather than a different exit-1). Commit axis (`git show <commit>:<path>` at `75c962d`, `e019b83`, `609ddcf`, `b236b10`, `8beb1c2`, and `git log -S` on the citation strings themselves, which is the control that decided `R2A-1`'s severity).

HELD FIXED, so a defect here survives this triage. One platform (Linux, local filesystem), one build profile (debug), one binary (built at `8a42b32`), uid 1000 only: I did NOT run anything under `unshare -Ur`, so a uid-dependent difference in check 16's root cell is untested here, as it was in round 1. I ran no concurrency and no TOCTOU case. I did NOT rebuild any historical binary, so every "before inc2 this printed ..." clause is unverified by me, exactly as B records for its own lens. I did NOT re-derive the whole citation sweep or the whole completeness sweep: my job was the thirteen findings, and a defect no reviewer raised survives this round. I did not sweep the sidecar's untouched head block (`:1` to `:43`), which both A and B name as unexamined and where the defect D twin at `:8` sits.
