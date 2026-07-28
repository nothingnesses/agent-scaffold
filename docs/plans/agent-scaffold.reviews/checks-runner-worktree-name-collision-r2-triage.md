# Triage, plan review round 2: `checks-runner-worktree-name-collision` (deferred step, order 93)

Artifact: `git diff a4f4c95..6d94cfc` (the whole fold), primary target `git diff HEAD~1..HEAD` (the round-1 fix commit `6d94cfc`).
Reviewer findings adjudicated: `docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-r2-reviewer.md` (`T2-1` `low`, `T2-2` `low`).
Triaged in worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage2-testiso` at `6d94cfc`, detached, independent of both the planner that wrote the step and the orchestrator driving the loop.

The human's 2026-07-28 decision to DEFER is out of scope and not re-litigated. No verdict below rests on the fix not being implemented.

**Standard applied.** Same as round 1: this is a durable RECORD and a future BRIEF, so I weight accuracy of the stated facts more heavily than I would on transient prose. The reader this step is written for is months away and has none of this round's reasoning.

## Verdict summary

| Finding | Reviewer severity | Verdict | My severity | Evidence reproduced |
| --- | --- | --- | --- | --- |
| `T2-1`: the rescoped `docs/` grep claim still does not reproduce | `low` | VALID | `low` (confirmed) | Yes, exactly |
| `T2-2`: done-conditions require three doc-comment corrections, documentation impact now requires four | `low` | VALID | `low` (confirmed) | Yes, both halves |
| `TR2-1` (triager-raised): the demonstration section's "proportional minimum" drops the red-before-green step the same section requires | n/a | VALID | `low` | Yes |

Valid findings to fix: **3**, all in one edit pass on the sidecar plus its regenerated view. Dismissed: none. Accepted residuals: none. Out of scope: none.

**Backstop status: not triggered.** Nothing was dismissed at any severity, so there is no dismissal at or above the `high` backstop threshold (`AGENTS.md:51`, `:59`) for a second triager to re-check. No `high` or `critical` finding was raised or created by this triage.

I also re-checked the reviewer's seven CLOSED verdicts on the round-1 findings by spot-sampling their load-bearing citations rather than accepting the table. `src/checks.rs:789-792`, `:845-847`, `:72-77`, `:400-405`, `:795-800`, `:862-871`, `:1438-1442`, `:1462`, `:1491`, `:1492` all read as reported; `git show HEAD~1:...` plus `sed`-extracted done-block `diff` returns `DONE BLOCK IDENTICAL`; `grep -rn "leans on the pid" docs/` hits only review files; the four `format!("{RUNNER_PREFIX}` sites are `:792`, `:1462`, `:1491`, `:1492`, exactly the set the scope section enumerates; `grep -cE "\brun\(" src/checks.rs` is 23; `AGENTS.md:124` is principle 11 verbatim; `validate --workflow` reports `217 records, valid` / `92 steps, 69 questions, valid` / `workflow invariants hold`, and `render --check --strict` reports `up to date`, both exit 0. The CLOSED verdicts stand.

## `T2-1` (`low`, VALID): the rescoped `docs/` grep claim still does not reproduce

**Evidence: REPRODUCED, exactly as the reviewer reported.** At the commit that wrote the sentence:

```
$ git grep -c "agent-scaffold-checks-run" HEAD -- README.md CHANGELOG.md docs/ pack/
HEAD:docs/plans/agent-scaffold.md:3
HEAD:docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-reviewer.md:9
HEAD:docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage.md:4
HEAD:docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md:1
HEAD:docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md:3
```

Five files; the sentence at sidecar line 94 enumerates three. I confirmed the two extra files were already committed at `a4f4c95`, the fold's own starting commit, so they were in the tree when the corrected sentence was written:

```
$ git grep -c "agent-scaffold-checks-run" a4f4c95 -- README.md CHANGELOG.md docs/ pack/
a4f4c95:docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-reviewer.md:9
a4f4c95:docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage.md:4
a4f4c95:docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md:1
```

The `README.md` / `CHANGELOG.md` / `pack/` half is correct: no hits at either commit.

**Ruling on the harder question the orchestrator posed: wrong-as-written, and the sentence should stop making a snapshot promise at all.**

First, wrong-as-written is not arguable. The parenthetical is a closed enumeration ("the only `docs/` hits are ... (A, B, C)") and the tail is an explicit universal ("so a grep run later will find those and nothing else"). Five hits, three named.

Second, the transience defence fails on the sentence's own terms, which is what settles it. The enumeration ALREADY counts one transient review artifact as a hit: "the transient step-92 findings file" is `docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md`, which is a review-round findings file under exactly the same commit-before-delete rule as this step's own round-1 reviewer and triage files. The sentence cannot both count that one and silently discount the other two; they are the same kind of object, in the same directory, with the same lifetime. So the charitable reading in which "this plan's own record of the defect" tacitly covers review files is defeated not just by the "nothing else" tail but by the enumeration's own membership.

Third, and this is why the fix is not "add two filenames": ANY enumeration of this set is wrong at almost every point in time. Adding the two round-1 files makes the sentence wrong again the moment this round's `-r2-reviewer.md` and `-r2-triage.md` land, and wrong in the other direction once the orchestrator commit-deletes all four at task close. The step is DEFERRED, so its reader is months out, by which time the review files are gone and an enumeration would name files that no longer exist. The claim's whole purpose is to be checkable by a future reader, and a snapshot list is the one form that cannot be.

The invariant underneath IS stable and IS checkable, and I confirmed it independently at `6d94cfc`: `git grep -l "agent-scaffold-checks-run" HEAD` returns `src/checks.rs` plus five files, every one of which is under `docs/plans/` and belongs to this plan's own material (the sidecar, its rendered view, and three review-round findings files). That statement is true now, true after the review files are deleted, and true for the reader the step is written for.

Note also that the round-1 triage's prescribed wording would have reproduced. It said: state "that `README.md`, `CHANGELOG.md` and `pack/` are clean and that the only `docs/` hits are the plan's own record of this defect". The planner adopted that and then ADDED a parenthetical enumeration and a "nothing else" universal. The addition is precisely what fails. That is a fact about how the fix was made, and I return to it in the convergence read.

**Severity `low`, confirming the reviewer, and on the same grounds round 1 rated `TI-4`.** The substantive conclusion is unaffected and I confirm it independently: outside `src/`, the name format appears only inside this plan's own documents, so no separate documentation step is owed. The cost is the one round 1 already articulated, that the record's cheapest checkable claim fails on the first attempt, and a reader who catches that has reason to distrust the expensive claims, which on this sidecar are the measured numbers I have confirmed are right.

**What the fix must achieve.** Drop the closed enumeration and the "nothing else" universal, and state the invariant instead: `README.md`, `CHANGELOG.md` and `pack/` carry no occurrence, and every `docs/` occurrence is inside this plan's own material (the sidecar, its rendered view, and the transient review-round files under `docs/plans/agent-scaffold.reviews/`), so nothing outside `src/` goes stale when the name format changes. Do NOT fix it by naming two more files.

## `T2-2` (`low`, VALID): the acceptance checklist enumerates three doc-comment corrections; the documentation-impact paragraph now requires four

**Evidence: REPRODUCED, both halves.**

Half one, the two lists. Sidecar line 94 (documentation impact, rewritten by the fix): "The three doc comments at `src/checks.rs:72-77`, `:400-402`, and `:845-847` spell the name format literally ... and a fourth, the comment on the naming site itself at `src/checks.rs:789-790` ... asserts the uniqueness that is currently false. All four are corrected by the same implementer". Sidecar line 62 (done conditions, untouched): "The three doc comments that spell the name format literally are corrected in the same change: `src/checks.rs:72-77` (`RUNNER_PREFIX`), `src/checks.rs:400-402` (`owning_pid`), `src/checks.rs:845-847` (`nanos`, the false premise)." I confirmed the done-conditions block is byte-identical across the fix with my own extraction and `diff` (`DONE BLOCK IDENTICAL`), and that both texts are mirrored into the rendered view at `docs/plans/agent-scaffold.md:1341` and `:1373`, so a fix must re-render.

`src/checks.rs:789-790` is verbatim as quoted, and sits directly on the naming site at `:791-792`.

Half two, the round-1 pointer. `git show HEAD~1:docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md | sed -n '55,60p'` puts the done-conditions bullets at pre-fix lines 55 to 59, with line 58 being the three-doc-comment item; the documentation-impact paragraph is pre-fix line 89 (the file is 89 lines). The round-1 triage wrote "the sidecar's documentation-impact list (line 58)" and, two clauses later, "done-condition line 55", and pre-fix line 55 IS the first done-condition bullet. So both of its line numbers point into the done-conditions block, its label "documentation-impact list" is a misnomer for line 58, and the item it cited by line number is the one that did NOT get the addition. The reviewer's reading is correct.

**Ruling: VALID, and it is fix-induced.** Before the fix the two enumerations agreed at three. After it they disagree, and the shorter one is the block whose job is to be the acceptance bar. An implementer who works "WHAT 'DONE' LOOKS LIKE", which is what that block exists for, corrects three and can close the step with `src/checks.rs:789-790` untouched, leaving in the tree the one comment the same fix pass identified as asserting the now-false uniqueness.

I considered and reject one charitable reading the reviewer did not test: that the done-condition's restrictive clause "that spell the name format literally" is a true closed description of exactly those three, so `:789-790` legitimately falls outside it. It does not survive contact with the list's own membership. `src/checks.rs:72-77` and `:400-402` do spell `agent-scaffold-checks-run-{pid}-{nanos}` literally; `:845-847` does NOT (it says "the process id in the path already provides per-process uniqueness"), and the done-condition itself concedes this by labelling it "the false premise". So the list already contains a member on the assert-the-premise ground, which is exactly `:789-790`'s ground. There is no principled category line that excludes the fourth.

**Severity `low`, confirming the reviewer.** I considered `medium`, because the failure shape is the one round 1 rated `medium` on `TI-1` (satisfy every done-condition and close the step with the defect's residue intact), and rejected it for a reason I verified rather than assumed: done-condition line 59 independently requires "the argument for why is written in the code comment", and the round-1 triage already ruled that this lands at `:789-790` because it is the comment on the naming site itself. The acceptance bar taken WHOLE therefore does reach the fourth comment; only its doc-comment bullet is under-inclusive. Second mitigation, checked: the comment at `:789-790` is the two lines immediately above the `format!` at `:791-792`, so any fix that moves name generation into a shared generator has that comment in its diff regardless. The residue is an internal inconsistency in one document about what work is required, not a likelihood that the comment rots.

**Not barred by a settled verdict.** An orchestrator could reach for the round-1 ruling that "it does not require the done-conditions to be rewritten" to dismiss this. That ruling was about the channel-D correction specifically. `T2-2` is a different question, about a doc-comment count, and it arises from text the fix pass wrote after that ruling. It is not a re-raise.

**What the fix must achieve.** One document, one list. Either extend the done-conditions bullet to the four comments the documentation-impact paragraph now requires (naming `:789-790` and why), or have the bullet point at that paragraph instead of restating a list. The two must not carry different counts of the same required work.

## `TR2-1` (`low`, VALID, triager-raised): the demonstration section's stated "proportional minimum" omits the red-before-green step the same section requires

The round-1 triage raised `TR-1` unprompted on the same footing; this is the round-2 equivalent, and it is in the section the step itself calls "the load-bearing part of this step".

**Evidence: REPRODUCED (citation, in the fix diff itself).** The fix pass rewrote the last demonstration bullet. Pre-fix (`git show HEAD~1:...`, line 80): "More machinery for a claim the unit test already settles; the uniqueness test plus the mutation is the proportional minimum." Post-fix (sidecar line 85): "the cost is real machinery, and the property test plus the linkage command is the proportional minimum."

The same fix pass added a third requirement to the section, the linkage bullet at line 83 (`TI-3`'s fix). Sidecar line 84 continues to require the red demonstration unconditionally: "Show it RED before green ... That mutation is what separates a test that pins the property from a test that merely passes, and it is the form `Q-66` names as strongest." So the section now requires three things and its own summary of the minimum names two, dropping the one it calls the strongest form. Pre-fix the section required two and the summary named both, so it was complete; the fix made it incomplete while editing that exact sentence.

**Ruling: VALID at `low`, fix in the same pass.** This is the same class as `T2-2`, one document carrying two enumerations of the required work at different lengths, created by the fix, and it costs one clause to close.

I am explicit about the two things that hold it at `low` and make it the weakest of the three:

- The sentence is ambiguous rather than plainly false. In context "the proportional minimum" is contrasted against the optional higher-fidelity extra, so it can be read as scoped to settling the LINKAGE claim, in which case it is silent about the mutation rather than excluding it. I do not think that reading rescues it, because "the proportional minimum" is unqualified normative language in a section that is the step's acceptance argument, but it is a reading a fair reader can reach.
- Unlike `T2-2`, the acceptance bar here is NOT the shorter list. Done-condition line 63 directly restates the requirement: "The uniqueness property is pinned by a test that FAILS without the fix (see the demonstration section...)". So an implementer working the done conditions is not misled; only one reading the last demonstration bullet in isolation is.

Applying the test the round-1 triage used on `TR-1`: I would not open an edit pass for this alone. `T2-1` and `T2-2` already open one, and a round-3 reviewer running the same fix-induced-residue lens has a good chance of finding it, because the fix pass touched this exact sentence. Closing it now is cheaper than closing it in round 3.

**What the fix must achieve.** The "proportional minimum" clause names every requirement the section imposes, or names none and points at the bullets instead. Either resolves it; one clause.

## Spot-check of the reviewer's four deliberately-not-raised items

The reviewer's restraint is right on all four. My reasoning differs from its own on two of them.

1. **"green against any of (a), (b), (c), (d) implemented correctly" versus done-condition "unique per call BY CONSTRUCTION".** Agree, not a finding. There is a genuine tension the reviewer identified honestly: (b) is probabilistic and (c) falls back on the clock for two calls on one thread, so a test green against a correct (c) is a test that can pass a fix which fails done-condition 1. But the wording is the round-1 triage's prescription adopted verbatim, the candidate list argues (b) and (c) down explicitly on exactly those grounds, and the bullet's subject is the test's fix-independence, not candidate selection. Re-raising it would relitigate a settled instruction without new evidence, which the ledger rule forbids.

2. **"reproduced every one of those numbers".** Agree, not a finding, and I checked it harder than the reviewer did. Against the round-1 reviewer's probe (`checks-runner-worktree-name-collision-reviewer.md:47-62`): min 20 ns reproduced exactly, `zero_deltas=0` reproduced, median 30 ns on one of two runs and 21 ns on the other, the 16-thread rate 569208 to 594658 against the recorded 568127 (close, not equal), and the two-thread rate 10933 to 16386 against the recorded 8679 (exceeded, not reproduced). So "every one of those numbers" is strictly wrong for the two-thread figure and loose for two others. It stays a non-finding because the same sentence corrects the one that materially differs, in the conservative direction ("read 8.7% as a floor"), and the other two gaps are ordinary run-to-run noise in a timing probe. Looseness, not an inaccurate record. Separately noted and also not raised: the sentence sources these numbers to "an independent probe during this step's plan review" with no pointer, and that probe's findings file is due for commit-deletion, so the provenance becomes unlocatable. It costs nothing, because the numbers themselves are quoted inline and the conclusion is stated as a floor.

3. **The step title names only the take-over presentation.** Agree, not a finding, but the reviewer's stated ground is weaker than it needs to be. "The title was equally partial before the fix, so nothing was made stale" does not distinguish this from `T2-2`, where the done-conditions block was equally three-item before the fix and the reviewer raised it anyway. The distinction that does the work is what the text is FOR: the done-conditions block is the acceptance bar, so an under-inclusive list changes what an implementer does, whereas the title is a Roadmap-table label that nobody implements from, and the reader who opens the step meets the corrected body at once. The title is also not false: its primary claim is "cannot land on one `{pid}-{nanos}` temp path", with the take-over as the illustrated consequence, and that consequence is exactly what the one recorded sighting showed. I verified the title text at `docs/plans/agent-scaffold.plan.toml:1264`. **I recommend AGAINST folding a title broadening into the fix pass**: it is the only one of these that would widen the diff into `plan.toml` and force a re-render for no acceptance-bar benefit, and both rounds so far have found their findings in text the immediately preceding pass wrote.

4. **Channel D versus the five done-conditions.** Agree, correctly not re-raised. The round-1 triage ruled explicitly that the corrected facts do not require the done-conditions to be rewritten and located the fix in candidate (a)'s trade-off. No new evidence beats that verdict, so it stays settled.

## Convergence read (advisory)

**Likely to close in one more round, but not safely so if the fix pass is made the way the last two were.**

The evidence pattern is specific and it is not random. Round 1 found 5, round 2 found 3, and every one of the 8 was in text the immediately preceding pass had just written. Round 2's three are sharper than that: all three are in sentences the round-1 FIX touched or should have touched, and in each case the pass added information without reconciling a count-bearing or scope-bearing statement elsewhere.

- `T2-1`: the pass took the triage's prescribed wording and added a closed enumeration plus a "nothing else" universal that the prescription did not have. The addition is the entire defect. Prescription would have reproduced; the embellishment does not.
- `T2-2`: the pass added a fourth item to a descriptive paragraph and did not update the checklist that counts the same items.
- `TR2-1`: the pass added a third requirement to a list and did not update that list's own summary of the minimum.

So the failure mode is not carelessness about facts. Every measured number, every `file:line`, and every code claim I checked in the fix is right, which is a high bar cleared twice. The failure mode is that the pass writes MORE than the verdict asked for, and the surplus is what breaks. That is a fix-method problem, and it has a fix-method answer.

Three concrete things that would improve the odds, in order of effect:

1. **Write to the verdict's prescribed wording and stop there.** All three of this round's findings live in surplus material. Where a triage verdict supplies an example wording ("for instance stating that ..."), adopt its scope; if the planner wants to add specificity beyond it, that addition needs its own check, because it is new unreviewed content in a document being fixed for accuracy.
2. **Prefer invariants to enumerations in any sentence a future reader might check.** This is the durable lesson from `T2-1` and it generalises to `T2-2` and `TR2-1`: all three are enumerations that went stale against something else in the same document or the same tree. The step is deferred, so every enumeration in it is a claim about a tree that will have moved. An invariant over a category survives; a list of three names does not. This is also why "add the two missing filenames" is the wrong fix for `T2-1`.
3. **Before committing, run one consistency pass over the counted things in the document.** Concretely: grep the sidecar for the numerals "three" and "four" and for the phrase "proportional minimum", and check each against the list it counts. That single pass catches `T2-2` and `TR2-1` outright, and it is the check whose absence produced both. `validate --workflow` and `render --check` cannot catch these; they are green right now with all three defects present, which I confirmed.

Two more notes on the mechanics of the next round:

- All three fixes are in one file (`docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md`) plus its regenerated view. No `plan.toml` edit is needed for any of them, and none should be made. Keeping the round-3 diff to the sidecar and the render is the smallest surface a fresh reviewer can find residue in.
- `T2-1`'s corrected sentence should be checked by actually running the grep at the fix commit before committing. It is the only one of the three whose correctness is a fact about the tree rather than about the document, and it is the one that has now failed twice.

Risk class is `low_risk` and one clean round converges. I think round 3 comes back clean if the pass is scoped to the three verdicts and the numeral check above is run; I think it comes back with a fourth if the pass adds new explanatory material to the sections it touches, which is what happened in both previous rounds.

## Tree state

`git status --porcelain` in this worktree reports two untracked files: the round-2 reviewer's findings file (copied in as input) and this triage file. No plan file, sidecar, or source file was edited; nothing was committed; no formatter was run. The only scratch files were the two done-block extracts for the `diff` above, written under the session scratchpad outside the repository. `cargo run -- validate --workflow` and `render --check --strict` were run read-only and both exit 0.
