# Plan-review round 2, step 90 `decision-folder-currency`: fix-verification and regression lens

Reviewer worktree: `.claude/worktrees/rev2-verify`, detached at `f8b3cdc`. Artifact: `e3fca03..f8b3cdc` (round-1 fix commit alone: `0905620..f8b3cdc`). Every command below was run from that worktree with the project toolchain (`direnv exec`), and every citation was re-read at the artifact commit before being written down.

Outcome: the four must-fix findings (`T-1`, `T-2`, `T-3b`, `T-6`) are CLOSED. The two leave-alone items were left alone. No high or critical finding. THREE NEW `low` findings, all documentation-accuracy defects inside the text the fix pass authored or rewrote.

---

## Verification of the four must-fix findings

### T-1 (medium): the `Q-67` `ask` declared a scope the step exceeds. CLOSED.

The offending sentence is gone and its replacement is accurate, not merely longer.

- The old claim no longer exists anywhere: `grep -n 'edits \`pack/AGENTS.md\` only (guidance, no prompt or source change), then regenerates' docs/plans/agent-scaffold.md docs/plans/agent-scaffold.plan.toml` returns nothing.
- The replacement is at `docs/plans/agent-scaffold.plan.toml:1716` (the `Q-67` `ask`) and reads, in relevant part: "SCOPE OF THE FIRST PASS (step 89, `planner-folds-decisions`, complete): it edited `pack/AGENTS.md` only (guidance, no prompt or source change), then regenerated the deployed `AGENTS.md` and `.agents/AGENTS.reference.md`. SCHEDULING EXTENSION (human, 2026-07-27): ... covering four further passages, three of them in `pack/prompts/orchestrator.md` (the checkpoint / queue-push, Socratic-mode, and ledger paragraphs) and one in `pack/AGENTS.md` (the ledger paragraph). This decision's edit surface therefore reaches a PROMPT as well as the guidance, and its full regeneration set is `AGENTS.md`, `.agents/AGENTS.reference.md`, and `.agents/prompts/orchestrator.md`".
- Each factual claim in that replacement checks out.
  - Step 89's implementation commit really did touch only those three files: `git show --stat 4f48283` -> `.agents/AGENTS.reference.md`, `AGENTS.md`, `pack/AGENTS.md`, 3 files.
  - Step 89 really did edit `pack/AGENTS.md` lines 41, 43, 71: `git show 4f48283 -- pack/AGENTS.md` has exactly two hunks, `@@ -38,9 +38,9 @@` and `@@ -68,7 +68,7 @@`, 3 changed lines.
  - The four step-90 passages and their homes match the sidecar's own list (`decision-folder-currency.md:12-15`): `pack/prompts/orchestrator.md:27` checkpoint, `:31` Socratic, `:33` ledger, `pack/AGENTS.md:63` ledger.
  - The three-file regeneration set is right: the self-scaffold emits all three (verified by running the render, below), and `diff` of a fresh render against each committed copy is empty.
  - "`folded_into: Option<String>` in `src/plan/source.rs`" is right; `folded_into` remains `"planner-folds-decisions"` at `docs/plans/agent-scaffold.plan.toml:1714`, which the triage said was correct and forced.
  - The two-step fan-out claim is right: both step 89 (`plan.toml:1224`, `status = "complete"`) and step 90 carry `[step.provenance] decisions = ["Q-67"]`.
- Propagation into the generated view is confirmed: `grep -c 'SCOPE OF THE FIRST PASS' docs/plans/agent-scaffold.md` -> 1, at `docs/plans/agent-scaffold.md:188`, the same `Q-67` queue item the round-1 finding named. `render --check` is up to date (below), so source and view agree.

### T-2 (low): the sidecar's framing mischaracterised two of the four passages. CLOSED (with one residual clause raised as `R2-2` below).

The blanket claim is gone and the new two-class split is CORRECT against the real pack lines, not merely present.

- The removed text: `git diff 0905620..f8b3cdc` shows "FOUR passages still leave the actor unnamed" and "None of them contradicts the now-explicit rule; they are incomplete" deleted from the framing paragraph.
- The new split is at `docs/plans/agent-scaffold.steps/decision-folder-currency.md:7-8` and puts `:33` and `pack/AGENTS.md:63` in the actor-less class and `:27` and `:31` in the actor-named class.
- Checked against the real lines, not against the round-1 report:
  - `pack/prompts/orchestrator.md:27` is second-person imperative and names the orchestrator: "There, update the plan's Open Questions queue and push its open items to the human, each per the human-input contract in `AGENTS.md`; do not wait for the human to pull them."
  - `pack/prompts/orchestrator.md:31` is likewise imperative: "for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision".
  - `pack/prompts/orchestrator.md:33` names no actor for the folding duty: "only durable decisions, the ones that change the plan, fold into it".
  - `pack/AGENTS.md:63` likewise: "only durable decisions, the ones that change the plan, fold into the plan's steps, and a folded decision reopens only by evidence that beats its recorded reasoning."
  - The conflict claims the split rests on are real: `pack/AGENTS.md:43` says the resolved answer's "non-trivial fold routed to the planner to author as above rather than edited in directly", and `pack/AGENTS.md:71` carries the added qualifier "Here \"updates this queue\" means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job (routed as above)".
- The per-passage operation labels added at `:19-22` (NARROW / REASSIGN / ADD / ADD) match the class each passage was put in, so the framing and the implementer bullets now agree.
- The re-attribution of the step-89 residual to two passages rather than one is correct in substance; only its line number is wrong, which is finding `R2-1`.

### T-3b (low): branch 2 only, branch 3 left to step 91. CLOSED.

The instruction is unambiguous and a competent implementer could not misread which branch to touch.

- `decision-folder-currency.md:20` says: "EDIT THE SECOND BRANCH ONLY. This is a single three-branch sentence (branch 1, answer a purely factual question directly; branch 2, the already-clear-options case, which is the target; branch 3, the not-yet-decidable case that tells you to \"record it as an `exploring` Open-Questions item\"). ... Leave branch 3 exactly as it is, even though the finished sentence will read oddly until `Q-69` is decided; do not re-litigate it here."
- The branch enumeration is accurate against the real line. `pack/prompts/orchestrator.md:31` ends with one sentence carrying exactly those three semicolon-separated branches in that order: "If the human drives by asking a question rather than giving a task, pick the mode it needs: answer a purely factual question directly; for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision; for one whose design space is not yet decidable ..., record it as an `exploring` Open-Questions item, spawn one or more explorers ...".
- Each branch is identified twice over, by ordinal AND by a distinguishing quote, so an implementer that miscounts still lands on the right text.
- The instruction is reinforced at a second place the implementer will read, `decision-folder-currency.md:40`: "The implementer of THIS step must leave all three untouched, in particular branch 3 of the sentence at `pack/prompts/orchestrator.md:31` that it is otherwise editing." And a third, in the new step-91 sidecar (`exploring-item-actor-boundary.md:17`), which states the same boundary from the other side.
- The out-of-scope disclosure T-3b asked for is present at `:40` and names all three exploration-mode passages, the class distinction, the owning step, and the extra deployed asset.

### T-6 (low): the regeneration instruction told the implementer to run `nix fmt`. CLOSED, and the substitute is verified to WORK, not merely to be safe.

`just scaffold-self` is gone (`grep -n 'just scaffold-self' docs/plans/agent-scaffold.steps/decision-folder-currency.md` returns only line 36, in the "Do NOT run" prohibition). The replacement at `:36` is `cargo run -- scaffold --output-dir . --write --force --principles default --instrument`.

I did not take that on trust. I checked it against the recipe and the CLI, then RAN it in a scratch copy.

- The recipe: `justfile:46-48` is `scaffold-self:` / `{{ direnv_prefix }} cargo run -- scaffold --output-dir . --write --force --principles default --instrument` / `{{ direnv_prefix }} nix fmt`. The substituted command is line 47 verbatim, flag for flag, minus the `direnv exec .` prefix that `direnv_prefix` expands to (`justfile:6`).
- The CLI accepts every flag with the same meaning: `src/main.rs:382-424` declares `--output-dir` (default `.`), `--force`, `--write`, `--principles` (default `default`), `--instrument`. No flag the recipe passes is missing or renamed, and the two flags the substitute leaves at their defaults (`--vcs git`, `--principle-detail summary`) are the same defaults `scaffold-self` gets.
- Runnable demonstration, no mutation of my worktree's committed files:

```
$ git archive f8b3cdc | tar -x -C $S/tree            # exact artifact tree
$ git archive f8b3cdc | tar -x -C $S/pristine        # untouched reference copy
$ cargo run -- scaffold --output-dir $S/tree --write --force --principles default --instrument
   ... overwrite AGENTS.md ... refresh .agents/AGENTS.reference.md ... refresh .agents/prompts/orchestrator.md ...
Wrote to $S/tree (30 changed, 0 left untouched).                                (exit 0)
$ diff -rq $S/pristine $S/tree --exclude=.git
                                                                                (no output)
```

  The render half alone, with no formatter, leaves the tree byte-identical: all three deployed files this step must regenerate come out exactly equal to their committed copies, and nothing else in the tree changes. So the substitute reproduces `scaffold-self`'s output minus the formatter, and the sidecar's supporting claim that "the raw render is already byte-identical to both committed files" is empirically true here, not just quoted from the guard's module docs.
- The two justifying citations are correct as numbered: `pack/AGENTS.md:79` is the "Format only your own files" rule naming `nix fmt`, and `pack/AGENTS.md:108` is the Prose-formatting paragraph whose closing sentence rules on this exact case. The drift-guard symbols cited exist: `normalize_wrapping` at `src/agents_md_drift.rs:232`, `the_committed_scaffold_matches_a_fresh_render` at `:291`, and the module docs at `:24-26` do say the raw render is already byte-identical because "the pack authors each paragraph on a single line".
- Not a finding, recorded so it is not re-raised: the substitute drops the `direnv exec .` prefix the justfile carries. The plan itself sanctions this at `docs/plans/agent-scaffold.md:38` ("Use `just build` ... or plain `cargo` inside `nix develop`"), and the command ran green under `direnv exec` above.
- Also not a finding: `planner-folds-decisions.md:11` and `reviewer-reproducible-evidence.md:12` still say `just scaffold-self`. The triage named both as converged precedents and did not require them changed; step 89 is complete, so its text is a historical record. Leaving them is correct, and changing them would have been the scope creep checked for in section 3.

---

## Regressions introduced by the fixes

Checked specifically, per the brief: did the T-1 rewrite break another claim in the `Q-67` record, contradict step 89's own sidecar, or misdescribe what step 89 did?

- No contradiction with step 89's sidecar. `planner-folds-decisions.md:11` says step 89 "edits no prompt file and no source ... only names the planner as the folder of decided entries at the three prose points (lines 41, 43, 71)", and `plan.toml:1224-1226` gives step 89 `status = "complete"`. The new "SCOPE OF THE FIRST PASS (step 89, ..., complete)" sentence says the same thing.
- No misdescription of step 89's actual work: verified against commit `4f48283` above.
- The rest of the `Q-67` record survives intact. The receipt sentence still names `type:"decision"` `q_id:"Q-67"`, and the new "the 2026-07-27 extension is a scheduling sub-decision under this same already-decided item, so it adds no question and owes no second receipt" is consistent with `[meta].w4_baseline = "Q-44"` (`plan.toml:3`) and with the step-90 sidecar's own "No new question and no receipt" paragraph at `:48`.
- The new clause "the second pass (step 90, below) holds to the same no-restatement constraint" matches `decision-folder-currency.md:26` ("This step therefore restates NONE of that list either"). It inherits the T-4 looseness the triage already weighed and accepted (the claim is about what the step does to the PACK, not about the sidecar's own prose), so it is not a new defect.
- One claim the rewrite left standing IS now in tension with the class-split the same commit added: finding `R2-3` below.
- Step 89's sidecar was not edited by this commit (`git diff 0905620..f8b3cdc --stat` lists no `planner-folds-decisions.md`), so nothing there regressed.

## The leave-alone items

All respected. No scope violation.

- T-4 (accepted residual, the four-item parenthetical) is untouched in all three places the triage enumerated: `docs/plans/agent-scaffold.steps/planner-folds-decisions.md:9`, `docs/plans/agent-scaffold.steps/decision-folder-currency.md:26`, and the `Q-67` `ask` (`grep -c` over `agent-scaffold.plan.toml` -> 1).
- T-5 (dismissed, "copy the pointing, not the list") is untouched: the phrase is still at `decision-folder-currency.md:26`, and `git diff 0905620..f8b3cdc` shows that whole paragraph as unchanged context.
- T-7 (dismissed, the short rendered heading) is untouched: `decision-folder-currency.md:1` still reads "### `decision-folder-currency`: name the planner as the folder at the remaining actor-ambiguous decision-folding points (`Q-67`)", and the step-90 `[[step]]` block, including its long `title`, is outside every hunk of the fix commit.

## Mechanical checks, re-run

```
$ cargo run -- render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date                                    (exit 0)

$ cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl
docs/metrics/workflow.jsonl: 210 records, valid
docs/plans/agent-scaffold.plan.toml: 92 steps, 69 questions, valid                 (exit 0)

$ cargo run -- validate --source ... --metrics ... --workflow
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold  (exit 0)

$ cargo test
378 tests across 6 suites, 0 failed
```

- 92 steps, 69 questions: confirmed.
- Metrics unchanged: `git diff e3fca03..f8b3cdc --stat -- docs/metrics/` is empty, and the file is 210 lines.
- No receipt was added for the new `open` `Q-69`: `grep -o '"q_id":"Q-6[0-9]"' docs/metrics/workflow.jsonl | sort | uniq -c` shows one each for `Q-60`..`Q-67` and nothing for `Q-68` or `Q-69`. Correct, since `Q-69` is `open`.
- The empty `docs/plans/agent-scaffold.questions/Q-69.md` is the convention, not an omission: all 69 question sidecars are zero-byte (`find docs/plans/agent-scaffold.questions/ -size 0 -type f | wc -l` -> 69).
- Step 92 carrying no `[step.provenance]` is fine: only 31 of the 92 steps have one.

---

## New findings

### `R2-1` (`low`): the sidecar's two new ledger citations point at the wrong line, in the artifact's own tree

The T-2 fix added an attribution to the step-89 triager's residual note and cited it as `docs/plans/agent-scaffold.ledger.md:345`. At the artifact commit that line is a different block entirely; the note is ten lines further down.

Evidence, re-run at `f8b3cdc`:

```
$ grep -n 'silent on the actor' docs/plans/agent-scaffold.ledger.md
355:RESUME/NEXT: step `code-value-audit-static` (86) is now `in-progress`. ...

$ awk 'NR==345' docs/plans/agent-scaffold.ledger.md | cut -c1-72
BLOCKED HERE (2026-07-27). Plan-review ROUND 2 COULD NOT BE RUN: four co
```

The quoted string "passively (silent on the actor, NOT contradicting the now-explicit `AGENTS.md:41`)" and the "`pack/prompts/orchestrator.md:33` AND the parallel `pack/AGENTS.md:63`" attribution both live in line 355. Line 345 is the round-2 blocked-here resume block and contains neither.

The wrong number appears twice in the source and twice in the generated view: `docs/plans/agent-scaffold.steps/decision-folder-currency.md:7`, `:14`, and `docs/plans/agent-scaffold.md:1197`, `:1204`.

This is not an inherited stale number that drifted after the fact. Both citations were ADDED by the fix commit (`git diff 0905620..f8b3cdc` shows them as `+` lines), and the ledger already had the note at 355 at the fix commit's parent `0905620` and at the round-2 base `e3fca03` (checked with `git show <rev>:docs/plans/agent-scaffold.ledger.md | grep -n 'silent on the actor'` for all three revisions, all -> 355). The number was copied from the triage file, where it was correct against a different tree, without re-reading it in the tree being edited.

Why it matters, and why `low`: the attribution and the quote are both correct, so a reader who greps the quote finds the note; the cost is a wasted lookup and one more misnumbered citation in a review chain that already lost two findings to exactly this in round 1. It is also a bad citation target on its own terms: the ledger's line numbering shifts on every checkpoint append, and the same sidecar hedges its pack citations ("find them by paragraph, not by line number, if they have since moved") while giving the ledger no such hedge.

Fix: cite `docs/plans/agent-scaffold.ledger.md:355`, or better, cite the note by its quoted text and drop the line number, since the ledger is append-heavy.

### `R2-2` (`low`): the framing sentence still calls the two new passages "of the same kind", which the class-split three lines later denies

`docs/plans/agent-scaffold.steps/decision-folder-currency.md:3` reads: "Its review round recorded an accepted-residual follow-up, and reads while scheduling that follow-up found two more points of the same kind: FOUR passages are still out of step with the rule".

The "accepted-residual follow-up" is the step-89 triager's note, which names `pack/prompts/orchestrator.md:33` and `pack/AGENTS.md:63` (ledger line 355, quoted above). The "two more points" are therefore `:27` and `:31`. But `:5` says "The four fall into TWO CLASSES, and the operation each needs is different, so do not treat them alike", and `:8` puts `:27` and `:31` in the opposite class from the residual's two, adding "These two were never covered by the step-89 residual".

So the same paragraph pair asserts that the two new passages are of the same kind as the residual's two, and that they are not. The fix pass corrected the second half of line 3's sentence ("still leave the actor unnamed" -> "are still out of step with the rule") and left "of the same kind" standing, which reasserts precisely the sameness the split was added to deny.

Severity `low`: the class-split immediately follows and is unambiguous, and the per-passage bullets at `:19-22` carry the right operation, so an implementer who reads on is not misled. But this sidecar's whole subject is a rule stated one way in one place and another elsewhere, and the sentence is the first thing a reader meets.

Fix: drop "of the same kind" or replace it with "of a related kind, but not the same class (see below)".

### `R2-3` (`low`): the rewritten `Q-67` `ask` still calls step 89's three edited points "actor-less", which the same commit's class-split contradicts for the parallel passage

`docs/plans/agent-scaffold.plan.toml:1716` (rewritten in this commit) says: "the FIRST pass (step 89) restates none of that list and edits only the three actor-less `pack/AGENTS.md` prose points".

Step 89's three points are lines 41, 43, 71 (`git show 4f48283 -- pack/AGENTS.md`, two hunks). The pre-edit line 71 is not actor-less; `git show 4f48283 -- pack/AGENTS.md` shows the removed line as "At every checkpoint the orchestrator updates this queue and pushes its open items to the human, each per the human-input contract, rather than waiting for the human to pull them", which names the orchestrator.

The contradiction is internal to the fix commit. `docs/plans/agent-scaffold.steps/decision-folder-currency.md:8` classifies the PROMPT twin of that same passage as the opposite class and says so using that very passage as its reference point: "`:27`'s unqualified \"update the plan's Open Questions queue\" is the exact verb `pack/AGENTS.md:71` needed an added sentence to qualify". By that reasoning `pack/AGENTS.md:71` was "actor named, wrong actor" before step 89, not actor-less. The artifact now describes two near-identical passages, one in the guidance and one in the prompt, in mutually exclusive terms.

Counter-argument, stated so the triager can weigh it rather than having to reconstruct it: "actor-less" can be read as "the FOLDING actor was unnamed", which is true of pre-edit line 71 (it did not mention folding at all), and the identical phrasing is pre-existing in a converged artifact at `docs/plans/agent-scaffold.steps/planner-folds-decisions.md:11` ("the three prose points (lines 41, 43, 71) where the actor was previously unnamed"). On that reading this is the same class as `T-4`: a loose but conventional phrase in plan prose, fixable in one sweep rather than here.

Severity `low` either way: it is a description of completed work in a decided record, fully reversible, and no implementer instruction depends on it.

Fix, if taken: change "the three actor-less `pack/AGENTS.md` prose points" to "the three `pack/AGENTS.md` prose points where the folding actor was unstated", which is true of all three and consistent with the step-90 class-split.

---

## Checked and cleared, so a later round does not re-raise them

1. `pack/prompts/orchestrator.md:33` and `pack/AGENTS.md:63` are described as stating the duty "in the passive voice". Strictly they are active voice with an inanimate subject ("only durable decisions ... fold into it"). The operative claim, that they name no actor, is correct, and the step-89 triager used the same loose word. Not a finding.
2. The sidecar's quotation of the `pack/AGENTS.md:71` qualifier at `:12` uses single quotes around "updates this queue" where the source uses double quotes, and stops before "(routed as above)". Both are ordinary quotation shortening; the quoted content is verbatim.
3. Steps 91 and 92 are not silent scope expansion. `docs/plans/agent-scaffold.ledger.md:343` records both as authored from human decisions ("TWO NEW STEPS authored from human decisions on the triager's out-of-scope item and on a verified gap"), and `T-3a` explicitly routed the exploration-mode class to a separate human-decided item. Their content is the parallel reviewer's lens, not mine.
4. All newly written citations in the changed and added text resolve correctly at `f8b3cdc`: `pack/AGENTS.md:45`, `:63`, `:71`, `:79`, `:91`, `:108`; `pack/user-prompts/explore.md:3`, `:7`, `:13`; `pack/pack.toml:166-167` (`source = "user-prompts/explore.md"` / `dest = ".agents/user-prompts/explore.md"`); `src/manifest.rs:615` (`".agents/user-prompts/explore.md"`); `justfile:46-48`; `src/agents_md_drift.rs` `normalize_wrapping` (`:232`), `assert_no_unprotected_construct` (`:99`), `the_committed_scaffold_matches_a_fresh_render` (`:291`). `R2-1`'s ledger line is the only citation in the change set that does not resolve.
5. The changed and added plan prose is ASCII-clean: `grep -nP '[^\x00-\x7F]'` over the three step sidecars returns nothing, and the only ` -- ` occurrences are the `cargo run --` argument separator.

## Summary

| id | severity | file | one line |
| --- | --- | --- | --- |
| `R2-1` | `low` | `decision-folder-currency.md:7`, `:14` | the step-89 residual note is at `ledger.md:355`, not `:345`; wrong at the moment it was written |
| `R2-2` | `low` | `decision-folder-currency.md:3` | "two more points of the same kind" contradicts the two-class split at `:5-8` |
| `R2-3` | `low` | `agent-scaffold.plan.toml:1716` | "three actor-less `pack/AGENTS.md` prose points" is false of pre-edit line 71, which named the orchestrator |

No `medium`, `high`, or `critical` finding. `T-1`, `T-2`, `T-3b`, and `T-6` are all CLOSED; `T-4`, `T-5`, and `T-7` were left alone; the mechanical checks are green.
