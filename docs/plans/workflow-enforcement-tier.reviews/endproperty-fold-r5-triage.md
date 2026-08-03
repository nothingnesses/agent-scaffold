# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 5 TRIAGE of the verification round after the escalation

Triager: independent of the planner, of both round 5 reviewers, of all four earlier triagers and of the round 4 backstop. READ-ONLY with respect to the reviewed product. This file is the only thing written and no fix is applied.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-ep5`, branch `triage/q55-ep5`, at `cda71ee`. Binary built here with `cargo build` (increment 1 in the tree, increment 2 absent, so every increment-2 verdict below is a hand computation over paths this worktree's own binary printed, and each is labelled as one).

POSTURE. The counters are RETIRED. The human authorised a specific fix (`cda71ee`) and ONE verification round, which under the counter rule took the "send it back for a specific fix" branch, so there is no streak arithmetic left to do here. The only question is whether the authorised fix is sound. One lens returned zero findings and the other returned four, so the round is NOT clean and the authorised "clean result merges" path did not fire.

METHOD. I built every fixture myself, in BOTH nesting directions, from the descriptions rather than from either reviewer's commands, and I read every root and every resolved artifact off this worktree's own binary rather than inheriting a number. Nothing below rests on a reviewer's transcript.

## Verdicts

| id | verdict | final severity | ground |
| --- | --- | --- | --- |
| `R5A-1` | VALID | medium (unchanged) | `:183`'s new nesting qualifier is stated symmetrically and I measured it false in the reverse direction, where the checked-plan rooting fires on BOTH artifacts; it contradicts `:229`, corrected by the same commit. |
| `R5A-2` | VALID | low (down from medium) | `:267`'s new "the ledger being echoed verbatim" is unqualified and is false for `status --resume` on the default-ledger route, which `:182`'s categorical same-root rule gates; both sides verified. |
| `R5A-3` | VALID | medium (unchanged) | `:271`'s new "an over-refusal rather than a silence" is falsified by my own fallback fixture and contradicted by accepted cost (i) sixteen lines above it; both sides verified. |
| `R5A-4` | VALID BUT ACCEPT AS RESIDUAL | low (unchanged) | The antecedent was genuinely removed, but "the guard wins" is not a false claim and the measured outcome in the same sentence supplies the contrast. |

DEDUPLICATED VALID COUNT: 3. SEVERITIES: medium, medium, low. Plus one low accepted as a residual.

No two of the three are the same defect: `R5A-1` is at `:183` and is about WHICH LAYOUTS the predicate catches, `R5A-2` is at `:267` and is about WHICH SURFACE echoes, `R5A-3` is at `:271` and is about WHAT CAUSES the bound. Three sites, three claims, no overlap.

## THE `:183` CONFLICT, RESOLVED: THE VERIFICATION LENS IS RIGHT, AND THE TWO LENSES AGREE ON THE MEASUREMENT

This was the round's central disagreement and it is not a disagreement about what the tool does. It is a disagreement about what the sentence says.

THE SENTENCE, as `cda71ee` left it at `:183` (both clauses are additions of this commit; `git diff HEAD~1 HEAD --word-diff` shows `[-that;-]{+that WHERE THE TWO PROJECTS DO NOT NEST;+}` and `[-case.-]{+case, and where they DO nest neither rooting catches it (the IN-ROOT BOUND below).+}`):

> The predicate rooted on the checked plan catches that WHERE THE TWO PROJECTS DO NOT NEST; an anchor-rooted one cannot, for the same reason it cannot catch the metrics case, and where they DO nest neither rooting catches it (the IN-ROOT BOUND below).

MY OWN FIXTURES, both directions, built from the sentence's own stated configuration (a Markdown-primary `--source` in one project beside a `--plan` in another). No symlink anywhere in either (`find -type l` returns 0), so the lexical and canonical readings coincide and neither reading of `:157` can separate them.

FIXTURE `REV`, REVERSE NESTING. The OUTER project `P` supplies the `--source` and the ledger; the INNER project `projQ`, at `P/packages/projQ`, supplies the `--plan`.

```
$ AS validate --plan $S/REV/P/packages/projQ/docs/plans/Q.md --workflow
no metrics log at /tmp/claude-1000/triage-ep5-scratch/fix/REV/P/packages/projQ/docs/metrics/workflow.jsonl; nothing to validate
    # root(checked plan) = .../REV/P/packages/projQ, through the convention branch

$ AS validate --source $S/REV/P/docs/plans/A.plan.toml --plan $S/REV/P/packages/projQ/docs/plans/Q.md --workflow
no metrics log at /tmp/claude-1000/triage-ep5-scratch/fix/REV/P/docs/metrics/workflow.jsonl; nothing to validate
    # resolved LOG = .../REV/P/docs/metrics/workflow.jsonl, anchored on the --source

$ AS next --source $S/REV/P/docs/plans/A.plan.toml --plan $S/REV/P/packages/projQ/docs/plans/Q.md
task: A
source: /tmp/claude-1000/triage-ep5-scratch/fix/REV/P/packages/projQ/docs/plans/Q.md
metrics: no log found
...
RESUME STATE (verbatim from the ledger):
## RESUME STATE

MARKER-OUTER-P-RESUME
exit=0
    # steps come from the SECOND project (projQ), the ledger from the FIRST (P):
    # .../REV/P/docs/plans/A.ledger.md is the only ledger in the fixture.
```

THE ONE UNBUILT STEP, a path-prefix comparison over three measured canonical paths with no free parameters. Root `.../REV/P/packages/projQ`; resolved log `.../REV/P/docs/metrics/workflow.jsonl`; resolved ledger `.../REV/P/docs/plans/A.ledger.md`. NEITHER artifact is under the root. The checked-plan-rooted predicate FIRES ON BOTH. The two projects nest. So "where they DO nest neither rooting catches it" is FALSE on this layout.

FIXTURE `FWD`, FORWARD NESTING, the control. The OUTER project `O` supplies the `--plan`; the INNER project `projA`, at `O/packages/projA`, supplies the `--source` and the ledger. Root `.../FWD/O` (measured); resolved log `.../FWD/O/packages/projA/docs/metrics/workflow.jsonl` (measured); resolved ledger `.../FWD/O/packages/projA/docs/plans/A.ledger.md` (measured, `next` echoes `MARKER-INNER-PROJA-RESUME`). BOTH are under the root, containment is silent, and the sentence holds. So the sentence is true in one nesting direction and false in the other, on the same pair of projects with the roles swapped.

WHAT EACH LENS ACTUALLY DID, and it is not a measurement dispute:

- The TRUTH lens DID build the reversed case. Its claim 1 fixture `rev` put the CHECKED project inside the FOREIGN one and it recorded the divergence correctly and in terms: the foreign log "is a SIBLING branch of the root ... not a descendant of it. By the rule's own words the log lies OUTSIDE the checked plan's root subtree, so containment would REFUSE it once inc2 is built", and it called that "the sharpest confirmation available that 'root subtree' is doing real work ... reversing which project contains which flips the verdict".
- But it applied `rev` only to CLAIM 1, the in-root bound at `:267`, which is stated DIRECTIONALLY ("outside the checked plan's root subtree") and is therefore true in both directions. Its CLAIM 4, the `:183` sentence, was tested on only two configurations: a SIBLING pair (not nested, caught) and a FORWARD-nested pair (nested, not caught). It never crossed its own `rev` fixture with the `:183` sentence. That is the whole of the gap.
- The VERIFICATION lens built the same reversed configuration as its fixture `N` and applied it to `:183`, where the symmetric wording makes it a falsification.

SO THE TWO LENSES AGREE ON THE MEASUREMENT AND DISAGREE ONLY ON COVERAGE. Both measured that reversed nesting diverges and that the checked-plan rooting would refuse. My own `REV` fixture reproduces that independently, on the LEDGER as well as the log. `:183` is written over "the two projects" and "nest", which is symmetric; the behaviour is directional. The verification lens is right.

IT ALSO CONTRADICTS THE SITE THE SAME COMMIT CORRECTED, and I verified both sides. `:229`, rewritten by `cda71ee`, states the same condition DIRECTIONALLY and correctly: "a `--source` in a different project reaches this only when that project is not NESTED inside the root." On `REV` the `--source`'s project `P` is NOT nested inside the root `projQ` (it contains it), so `:229` says `ledger-not-this-project` fires, which matches my measurement. `:183` says neither rooting catches it. One commit, two sentences, opposite answers on one layout, and the JSON vocabulary an implementer builds to is the one that is right.

WHY THE TRUTH LENS'S ZERO IS NOT THEREBY WORTHLESS. Its other five claims were tested harder than any prior round tested anything (three nesting depths, two occupants of one subtree, both root-derivation routes, six symlink placements plus a mirror control, a code read of `run_next` and a plan-wide owner search), and I found nothing wrong with any of those verdicts. Its zero is a real result on five of six targets. It is wrong on one, and the one it is wrong on is the one its own fixture set already contained the falsifier for.

## `R5A-1` VALID, medium. `:183`'s nesting qualifier is symmetric and the behaviour is directional

SITE: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:183`, projection `docs/plans/agent-scaffold.md:1578` (regenerated, not hand-edited: `render --check --strict` prints "up to date" at exit 0).

RULING: VALID, on my own measurement above and on the internal contradiction with `:229`.

SEVERITY: MEDIUM, unchanged. It is an affirmative claim about which layouts the specified predicate reaches, sitting in the behavioural specification bullet for `next`, and it is false in a layout that is an ordinary monorepo invocation (the repository-root plan as `--source`, a package's plan as `--plan`). A reader of `:183` in that layout is told the guard does not reach it and will read inc2's new refusal and new omission as a regression. What holds it at medium rather than high: `:229`, in the same file, specifies the normative vocabulary correctly and directionally, and `:267`'s in-root bound is directional too, so an implementer building to the vocabulary builds the right thing; the false sentence is a scoping commentary standing beside two correct normative statements.

## `R5A-2` VALID, re-severitised DOWN to low. `:267`'s outcome clause is unqualified where one surface answers the other way

SITE: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:267`, projection `docs/plans/agent-scaffold.md:1662`. The clause is an addition of this commit (`[-still greens.-]{+the ledger being echoed verbatim.+}`).

BOTH SIDES OF THE ALLEGED CONTRADICTION, VERIFIED.

SIDE A, `:267` as `cda71ee` left it: "CONTAINMENT REFUSES ONLY WHAT LIES OUTSIDE THE CHECKED PLAN'S ROOT SUBTREE, so every foreign artifact inside that subtree is invisible to it: a log copied to this plan's own `docs/metrics/`, and equally a NESTED project's own log and ledger at their own conventional paths, the log then joining by bare slug and the ledger being echoed verbatim." The outcome clause carries no surface qualifier, and it sits under "What this step does not fix", the section whose stated audience is that "an implementer must NOT 'fix' them, and a reviewer must NOT raise them as defects" (`:253`).

SIDE B, `:182` as the SAME COMMIT left it: "a `--source` and a `--plan` that both exist must resolve to the SAME root or the block is omitted". Categorical, with no containment in it at all.

MEASURED ON MY `FWD` FIXTURE, which is the forward-nested layout the clause describes: root of the `--source` (`projA`) is `.../FWD/O/packages/projA` and root of the `--plan` (`O`) is `.../FWD/O`, both read off the binary; both paths exist; the roots DIFFER. So `:182` omits the block on `status --resume`. `next` does not: its ledger test is containment (`:183`, `:229`), the resolved ledger `.../FWD/O/packages/projA/docs/plans/A.ledger.md` IS under `.../FWD/O`, so containment is silent and the echo stands (measured today: `MARKER-INNER-PROJA-RESUME`). Two surfaces, one file, opposite specified answers, and `:267` states one of them as the outcome.

WHY I RE-SEVERITISE DOWN TO LOW, and this is a genuine correction to the reviewer rather than a discount. The clause is true on one route into the same layout and false on the other. Reached by an explicit `--ledger-fragment` naming the nested project's ledger with only a `--source` present, `:182`'s "both exist" precondition does not hold, "with one alone the anchor is the root, as today" applies, the nested ledger is inside the root, containment is silent, and `status --resume` DOES echo it verbatim. Reached by the default ledger under the divergent pairing, which is the only route that reaches BOTH the log and the ledger without an explicit flag and is therefore the route the sentence's own two-artifact framing points at, `:182` gates it and `status --resume` omits. So the defect is an UNQUALIFIED OVERSTATEMENT, not a false statement about the mechanism, and the normative rule an implementer builds to is correct and categorical. That is a low.

NOT FILED, AND SURVIVING: the underlying two-surface divergence itself, that `:179`'s "The trigger is the SAME containment predicate ... The predicate is never re-implemented per surface (One source of truth)" sits above a `status --resume` rule that tests two roots for AGREEMENT rather than containment. The round 4 backstop recorded it as "adjacent and NOT filed" and put it in front of the human; no round has ruled on it; `cda71ee` did not create it and the prescribed fix does not close it. It stays where the backstop left it.

## `R5A-3` VALID, medium. `:271`'s direction claim is false against the code and against cost (i)

SITE: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:271`, projection `docs/plans/agent-scaffold.md:1666`. Wholly added by this commit, and it is the one added sentence that belongs to NONE of the four authorised parts.

BOTH SIDES OF THE ALLEGED CONTRADICTION, VERIFIED.

SIDE A, `:271`: "That fallback is NOT the in-root bound's cause either; its direction is the opposite one, an over-refusal rather than a silence."

SIDE B, accepted cost (i) at `:255`, sixteen lines above it: the same fallback "derives the root from a source path with no parents to walk, falls back to the source's own directory, and looks for `docs/metrics/workflow.jsonl` beneath it, which does not exist. The project's real log is never read. ... the containment guard STRUCTURALLY CANNOT catch it, because the wrong path is still inside the right project: containment is not correctness. ... this case becomes a HARD FAILURE naming the path it looked for rather than a silent green". That is the fallback, producing a SILENCE, with containment structurally blind for the in-root bound's own reason, in the document's own words.

MEASURED, on my own conventionless fixture `G`, built independently. A TOML-primary `myplan.plan.toml` sits directly at `$S/G/repo` with no `docs/plans` ancestor, so its root comes through the FALLBACK, and a scaffolded project sits nested at `repo/vendor/projA` carrying a copy of this repository's log at its own conventional path. The plan's single step carries the borrowed slug `triager-runs-only-on-findings` at `complete` and has no evidence of its own.

```
$ AS validate --source $S/G/repo/myplan.plan.toml --workflow
no metrics log at /tmp/claude-1000/triage-ep5-scratch/fix/G/repo/docs/metrics/workflow.jsonl; nothing to validate
    # root = .../G/repo, through the FALLBACK

$ AS validate --source $S/G/repo/myplan.plan.toml --metrics $S/G/repo/vendor/projA/docs/metrics/workflow.jsonl --workflow
/tmp/claude-1000/triage-ep5-scratch/fix/G/repo/vendor/projA/docs/metrics/workflow.jsonl: <n> records, valid
/tmp/claude-1000/triage-ep5-scratch/fix/G/repo/myplan.plan.toml: 1 steps, 0 questions, valid
... vs .../G/repo/vendor/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0

$ # CONTROL, the same run against an EMPTY log in the same subtree:
$ AS validate --source $S/G/repo/myplan.plan.toml --metrics $S/G/repo/vendor/projB/docs/metrics/workflow.jsonl --workflow
... : Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records and no covering waiver; ...
exit=1
```

(The record count is elided; a standing rule forbids asserting a count of anything in `docs/metrics/workflow.jsonl`, and nothing here turns on the number.) The fallback WIDENED the root to `.../G/repo`, which contains `vendor/projA`; the foreign log is inside it; containment is silent; the pairing greens on foreign records, and the control proves the green comes from those records rather than from an empty file. The direction there is a SILENCE, and it is the in-root bound exactly.

RULING: VALID. Both halves of the sentence overshoot. The fallback IS one route into the bound, so "NOT the in-root bound's cause" is too strong, and its direction is not uniformly an over-refusal, so the second half is simply false.

WHERE THE OVERSHOOT CAME FROM, which matters for the fix. The round 4 backstop wrote, of costs (iii) and (iv), "Those are over-refusals through the fallback ... the direction is the opposite one", and separately measured and stated the fallback's own property at target 3: "The fallback is one way to widen the subtree and needs nothing of the sort to happen", i.e. SUFFICIENT AND NOT NECESSARY. The fix generalised the first (a claim about two costs) into a claim about the fallback itself, and dropped the second. My fixture `G` reproduces the backstop's own `F3` result independently.

SEVERITY: MEDIUM, unchanged. This sentence exists to route queued work to a root cause, and it states something about the code that is measurably false, which is precisely the ground on which the human declined a fifth accepted cost. It also mis-scopes the queued work in the other direction: `Q-55-noconvention`'s rejected alternative to the fallback was a hard error, and taking it would have closed one route into the bound.

## `R5A-4` VALID BUT ACCEPT AS RESIDUAL, low. The orphaned antecedent is real and is not a wrong claim

SITE: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:257`, projection `docs/plans/agent-scaffold.md:1652`.

THE PLANNER'S DISCLOSURE AND THE REVIEWER'S ALLEGATION, BOTH VERIFIED against `git diff HEAD~1 HEAD --word-diff`. The prescribed mechanism replacement is `Where <root>/docs/plans is a symlink to <root>/elsewhere, the [-lexical default-]{+canonicalised plan+} and the [-canonical guard disagree about which project the plan belongs to,-]{+canonicalised log land under different roots,+} and the guard wins`. So the edit removed BOTH "the lexical default" and "the canonical guard disagree", and "and the guard wins" is retained CONTEXT, not an addition. The planner disclosed repairing the sibling antecedent sixty words later ("the disagreement" to "the divergence") and did not mention this one. The reviewer is factually right: one of two antecedents was repaired.

RULING: VALID BUT ACCEPT AS RESIDUAL. The clause is not a false claim. "Wins" reads naturally as the guard's verdict prevailing over a run that would otherwise have succeeded, the heading two words earlier already names the predicate as the actor ("BECOMES A FALSE POSITIVE ON THE PREDICATE"), and the colon clause immediately supplies the contrast as a measurement ("going from reading its 37-record log to `exit=1 REFUSED`"), which names both the answer that lost and the answer that won. `:165` defines the guard against the lexical default earlier in the document. No reader is misled about any behaviour.

SEVERITY: LOW, unchanged. It is a readability artefact of a prescribed edit, in the one region the round 4 triage prescribed word for word.

IF A FIX PASS RUNS ANYWAY, the four-word deletion should ride along, because it authors nothing and a deletion cannot re-seed, and because leaving it means a sixth reader may re-file it. It is not on its own worth a fix pass.

## Prescribed minimal fixes, with authored word counts and measured site counts

SITE SWEEP, run over ALL of `docs/plans/agent-scaffold.steps/` and `docs/plans/agent-scaffold.plan.toml`. Every one of the four is a SINGLE authored site, all four in `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, with one REGENERATED projection twin each in `docs/plans/agent-scaffold.md`. `grep -rln` over the steps directory returns only `workflow-enforcement-tier.md` for all four phrases; `docs/plans/agent-scaffold.plan.toml` carries ZERO occurrences of any of them; `docs/plans/agent-scaffold.ledger.md` carries zero occurrences of "over-refusal". So no site count exceeds 1 authored plus 1 generated, and no fix touches the plan TOML or the ledger. The projection is regenerated with `render`, never hand-edited.

`R5A-1`, A NARROWING TO `:229`'S OWN VOCABULARY. At `:183`, replace "catches that WHERE THE TWO PROJECTS DO NOT NEST" with "catches that WHERE THE `--source` LIES OUTSIDE THE CHECKED PLAN'S ROOT", and "and where they DO nest neither rooting catches it" with "and where it lies INSIDE, neither rooting catches it". AUTHORED: 10 words, of which 7 are lifted from `:229`'s existing "a `--source` ... lies outside it" wording. DELETED: 8 words. NET: +2 words. SITES: 1 authored, 1 generated.

WHY THAT TEST IS EXACT rather than a paraphrase, checked against the code: `default_ledger_path` returns `anchor.parent().join(<task>.ledger.md)`, so the ledger sits in the `--source`'s own directory. If the `--source` is outside the root then its parent is outside the root (a child of an in-root directory is in-root, so the contrapositive holds), hence the ledger is outside and the predicate fires; if the `--source` is inside, the ledger is inside and containment is silent. The `--source` test and the ledger test are the same test on this configuration.

A PURE DELETION IS AVAILABLE HERE AND IS WORSE, stated so the option is not lost. Deleting the whole final sentence removes the false claim and authors nothing, but it also removes the TRUE and load-bearing claim that an anchor-rooted predicate cannot catch the ledger case, which is what check 14g's fourth run ("THIS RUN, NOT CHECK 14b, is what separates an anchor-rooted projection from a checked-plan-rooted one", `:333`) pins and which `Q-55-endproperty`'s rooting decision rests on for the ledger half. Deleting only the two added clauses is worse still: it restores the unqualified "catches that" the round 4 backstop falsified.

`R5A-2`, A PURE DELETION. At `:267`, drop "and the ledger being echoed verbatim". AUTHORED: 0. DELETED: 6 words. SITES: 1 authored, 1 generated. The sentence then reads "... a NESTED project's own log and ledger at their own conventional paths, the log then joining by bare slug", which is what the human authorised ("containment refuses only what lies outside the checked plan's root subtree, so a nested project's log and ledger are invisible to it") and is true on both surfaces. `next`'s echo outcome is already stated at `:183`, so nothing is lost. The two-word narrowing "echoed verbatim by `next`" is the alternative; I prefer the deletion, because the narrowing writes a second surface-specific outcome into a paragraph whose subject is one predicate, and because the outcome clause was never part of the authorised content in the first place.

`R5A-3`, A NARROWING THAT IS MOSTLY DELETION. At `:271`, replace "That fallback is NOT the in-root bound's cause either; its direction is the opposite one, an over-refusal rather than a silence." with "That fallback is not REQUIRED for the in-root bound." AUTHORED: 9 words, of which 2 ("REQUIRED for") are new; the rest are reused from the sentence being replaced. DELETED: 21 words. NET: -12 words. SITES: 1 authored, 1 generated. That is exactly what the round 4 backstop measured and what my fixture `G` reproduces, it still stops an implementer queueing the bound under the wrong root cause, and it does not delete the whole sentence, which the human authorised a statement at.

`R5A-4`, A PURE DELETION, only if a fix pass runs. At `:257`, drop ", and the guard wins". AUTHORED: 0. DELETED: 4 words. SITES: 1 authored, 1 generated.

TOTALS FOR THE THREE VALID FINDINGS: 19 authored words (9 of them genuinely new tokens), 35 deleted, NET -16. Adding `R5A-4` makes it NET -20. No fix authors a new claim, no fix introduces a term the document does not already define, and two of the four are pure deletions.

## Backstop rule

I DISMISSED NOTHING. All four findings are ruled VALID, one of them accepted as a residual. Nothing was rated `high` or `critical`, so NO BACKSTOP RE-CHECK IS OWED.

## What I checked and did not file

- THE OUT-OF-SCOPE LIST HELD in both review files and in my own reading. Nothing on line length or wrapping. Nothing on the increment-1-falsified present-tense `src/main.rs` claims. Nothing on increments 1 or 3. Nothing on the mechanism defect itself, which the human decided to RECORD rather than close and for which the same-project-root replacement was explicitly declined. Costs (i), (iii) and (iv) are untouched as stated, and `R5A-3` cites cost (i) as EVIDENCE rather than raising it as its subject. None of the four already-ruled residuals is re-raised.
- THE PLAN TOML AND LEDGER SYMLINK SITES. Not re-opened. Both reviewers and the planner agree they sit inside dated decision records corrected by appending, and my sweep confirms none of the four fix sites reaches them.
- THE SCOPE LOCK. `git diff HEAD~1 HEAD --stat` is two files, 18 lines, 9 in each, and the two sets correspond. `render --check --strict` prints "up to date" at exit 0, so the projection is a fresh render. `:251` still says four accepted costs and the enumeration still stops at (iv). No Rust, no test, no plan TOML, no ledger, no review file was touched.
- THE TRUTH LENS'S OTHER FIVE VERDICTS. I found no error in any of them and I did not re-derive them from scratch, having spent the budget on the conflict I was asked to resolve. Its claim 5 code reading of `run_next` matches what I read: the ledger path is computed from `args.ledger_fragment` or `default_ledger_path(&task, &args.source, &args.plan)` and reads no `Round` field, so no record filter can reach it.
- THE ROUND 4 TRIAGE'S SITE SWEEP IS WRONG AS A FACT, as the verification lens records: `docs/plans/agent-scaffold.plan.toml:1714` does carry symlink text. That defect is in completed review history, not in the reviewed product, so it is not a finding; I repeat it here only so a later pass does not reuse "the plan TOML carries no symlink text" as a settled result.

## THE PATTERN, SHARPENED, because the "authored prose re-seeds" reading is not quite what happened

I measured the fix's own totals rather than inheriting them: `git diff --word-diff=porcelain` on the step file gives 265 AUTHORED words against 35 DELETED, which matches the verification lens exactly. And the split it reports holds: the roughly 97 words in the two parts the round 4 triage prescribed word for word produced ONE low, and the roughly 161 words that were priced at nothing produced everything else.

BUT THE UNPRICED WORDS ARE NOT UNIFORMLY BAD, AND THAT IS THE USEFUL RESULT. Of the four unpriced sites, TWO are clean and were ruled TRUE by BOTH lenses independently: `:229`'s containment restatement (about 29 words) and `:269`'s ownership negative (about 37 words). The three that failed are `:183`'s nesting qualifier, `:267`'s outcome clause and `:271`'s direction claim. What those three share is not that they are prose. It is that EACH STATES A SCOPE OR AN OUTCOME MORE GENERALLY THAN WHAT WAS MEASURED: a symmetric condition where the behaviour is directional, an unqualified surface outcome where two surfaces answer differently, and a single direction attributed to a mechanism measured to go both ways. The two clean ones are narrowed to exactly their measurement, and one of them (`:229`) is the sentence that catches `:183` out.

THE OPERATIONAL RULE THAT FOLLOWS, and it should be carried into any further fix pass on this artifact: prefer deletion, and where a claim must be stated, STATE IT IN THE DIRECTION IT WAS MEASURED IN and in vocabulary the document already defines. All three prescribed fixes above obey that, which is why two of them delete and the third copies `:229`.

## Recommendation to the orchestrator

THE AUTHORISED PATH WAS "FIX, ONE ROUND, MERGE ON A CLEAN RESULT" AND THE ROUND WAS NOT CLEAN, SO THIS GOES BACK TO THE HUMAN. Four live options, with what each costs and what it risks.

OPTION A, APPLY THE THREE FIXES AND MERGE WITHOUT A FURTHER ROUND. RECOMMENDED. Cost: one writer pass in a worktree, a re-render, both validates, one commit. The diff is three sites, 19 authored words against 35 deleted, net -16, two of the three pure deletions and the third lifting seven of its ten words from `:229`. Risk: the fixes land unreviewed. That risk is the smallest it has been at any point in this fold, for three reasons. The class of change with a clean track record here, measured five times, is exactly deletion-and-narrowing, and this diff is that class almost entirely. The one fix that authors anything copies the vocabulary of `:229`, which BOTH round 5 lenses ruled TRUE independently and which is the sentence that caught `:183` out, so it is the most-verified wording in the fold. And I measured both nesting directions myself, so the replacement's correctness does not rest on either reviewer's transcript. What it gives up: a formal reviewer signature on a sixteen-word net deletion.

OPTION B, APPLY AND RUN ONE MORE ROUND. Cost: two more reviewer agents plus a triage, on an artifact that has now consumed five rounds, fourteen review files, one backstop and one escalation, to review a net sixteen-word deletion. It also re-opens arithmetic the escalation deliberately retired, and the round would need its own framing to avoid being read as round 6 of a converging loop. Risk: low that it finds a real defect in a deletion-dominant diff, but every prior round on this artifact has found something, and each fix has in turn authored prose. This is the option that trades a real cost for a small amount of certainty. Choose it only if the human wants the signature more than the close.

OPTION C, ACCEPT SOME OR ALL AS RESIDUALS AND MERGE AS-IS. Cost: zero. Risk: uneven across the four. Accepting `R5A-4` costs nothing and I have already ruled it a residual. Accepting `R5A-2` costs little, since the normative rule at `:182` is correct and categorical and only a descriptive outcome clause overstates. Accepting `R5A-1` and `R5A-3` is the expensive half: `:183` would stay false in one nesting direction and in direct contradiction with `:229` in the same file, which is the SAME sentence the round 4 backstop overturned a dismissal over, so a sixth reader would be the fifth to rediscover it; and `:271` would stay a measurably wrong statement about the code in a sentence whose job is routing queued work, which is precisely the ground on which the human declined a fifth accepted cost. A middle position is coherent: fix `R5A-1` and `R5A-3`, accept `R5A-2` and `R5A-4`. That is one site fewer and eleven words fewer than option A, and it is the option to take if the human wants the absolute minimum diff.

OPTION D, REVERT THE UNPRICED PART OF THE ESCALATION FIX AND MERGE THE PRICED PART ALONE. NOT RECOMMENDED, and I want to be concrete about why, because the framing is attractive and the evidence is against it. The unpriced part is `:183`, `:229`, `:267`, `:269` and `:271`. Reverting `:183` and `:229` restores the exact two sentences the round 4 BACKSTOP ruled FALSE, which is strictly worse than the state being reverted from. Reverting `:267` restores the instance-bound recording that four independent readers rediscovered from scratch across four rounds and that the backstop's separate judgement called inadequate on four counts. Reverting `:269` deletes a claim BOTH round 5 lenses ruled TRUE and which records a real ownership gap. So a full revert re-introduces two ruled-false sentences and one inadequate recording in order to remove one false sentence and two overstatements. It also un-does content the human explicitly authorised, which cannot be done without going back to them regardless. THE ONE COHERENT FRAGMENT OF IT: reverting `:271` ALONE. That sentence belongs to none of the four authorised parts, it is the single most unpriced thing in the commit, and deleting it outright is a 21-word deletion that authors nothing. It is a defensible alternative to `R5A-3`'s prescribed narrowing, and it costs the reader only the negative statement that the bound is not the fallback's alone. I prefer the narrowing, because the sentence's purpose (stopping the bound being queued under the wrong root cause) is worth nine words and the backstop measured the true version of it.

WHAT I WOULD DO: option A. If the human wants the diff smaller than that, the middle position under option C (fix `R5A-1` and `R5A-3`, accept `R5A-2` and `R5A-4`) is the next best and loses nothing structural.

ONE THING TO CARRY WHICHEVER OPTION IS TAKEN. The two-surface ledger divergence (`:182`'s agreement test against `:183`'s containment test, under `:179`'s claim that the predicate is never re-implemented per surface) is NOT closed by any option above. The round 4 backstop put it in front of the human as an adjacent observation, the round 5 verification lens flagged that it survives its own fix, and no round has ruled on it. It is a real open question about the specification, not a defect in `cda71ee`, and it should not be allowed to vanish into a merge.

## Scratch hygiene

Every fixture was built under `/tmp/claude-1000/triage-ep5-scratch/`, created for this triage and removed after the evidence above was captured. Nothing was written to bare `/tmp`. DIRECTORIES LEFT IN `/tmp`: 0 (the harness-provided session scratch tree under `/tmp/claude-1000/` is not counted).
