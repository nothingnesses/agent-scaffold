# Round 2 reviewer: Q-66 / Q-67 plan fold (fix verification + regression sweep)

Reviewer: fresh, independent, adversarial; read-only w.r.t. the product. This file is the only write.
Worktree: `.claude/worktrees/q66-plan-review-r2` at `cca1099`.
Fix commit under review: `44f848a..cca1099` ("apply Q-66/Q-67 plan-review round 1 fixes").
Round 1 raised 4 findings (F1 medium, F2/F3/F-fid low, all triager-valid). This round verifies
each fix closed its finding and sweeps for new issues the fixes introduced.

Grounding rule (the Q-66 rule, dogfooded): every ruling below carries a re-runnable `file:line` +
verbatim quote or a command a second party can re-run. Nothing is asserted from impression.

## Outcome

ZERO FINDINGS. Clean round. All four round-1 fixes are confirmed closed, and the adversarial
regression sweep (dropped Part 2 completeness, new step-number / line-number claims, gates, house
style) turned up nothing new. Both gates pass.

## Fix verification (each round-1 finding re-checked against its citation)

### F1 (medium, Q-67 scope) -> CLOSED

Round 1 asked to: drop Part 2's closed-list "reinforce", keep Part 1 (name the planner at the three
actor-less prose points), add a cross-reference paragraph pointing at the generated
`ISOLATION_POLICY_FRAGMENT` as the single source, state the fragment's "author no reviewed product
content" clause already implies the exclusion, cross-reference Q-51 `driver-output-generation`, keep
the step genuinely guidance-only, and stop the TOML `ask` overclaiming the reinforce scope.

Reproduced closed:
- Part 2 dropped: the round-1 second bullet ("Reinforce that the orchestrator's closed list ...
  EXCLUDES ...") is gone. `git diff 44f848a cca1099 -- docs/plans/agent-scaffold.steps/planner-folds-decisions.md`
  shows that line removed; `planner-folds-decisions.md` now has a single change bullet (line 7).
- Part 1 kept and now carries the checkpoint clarification positively:
  `planner-folds-decisions.md:7` "... is the PLANNER's job, routed by the orchestrator, not the
  orchestrator's own direct edit. At the checkpoint point this is the same boundary stated
  positively: \"the orchestrator updates this queue\" means surfacing and pushing the open items,
  not authoring the decided entries." The checkpoint clarification the old Part 2 carried is thus
  preserved, not lost.
- Cross-reference paragraph added and accurate: `planner-folds-decisions.md:9` "... it is the
  generated `ISOLATION_POLICY_FRAGMENT` (`src/isolation_policy.rs:33`), rendered into the
  `{{isolation_policy}}` slot at `pack/AGENTS.md:91` ...". Both citations reproduce:
  `sed -n '33p' src/isolation_policy.rs` = `pub(crate) const ISOLATION_POLICY_FRAGMENT: &str = "..."`
  whose tail is the four-item list "flipping a step's status, declaring an increment, recording a
  round record, and moving the ledger's resume anchor." and rationale "which author no reviewed
  product content and so stay the orchestrator's direct job rather than a spawned agent's";
  `sed -n '91p' pack/AGENTS.md` = `{{isolation_policy}}`.
- "already implies the exclusion" stated: `planner-folds-decisions.md:9` "the fragment's existing
  rationale, that those integration edits \"author no reviewed product content ...\", already
  implies that authoring a `[[question]]` or a `[[step]]`, which IS reviewed product content, is the
  planner's (spawned-writer's) work." The fragment does name the planner a spawned writer
  ("This holds for the writers (the planner and the implementer) ..."), so the implication holds.
- Q-51 `driver-output-generation` cross-referenced with a correct step number:
  `planner-folds-decisions.md:9` "the completed `driver-output-generation` step 67 (`Q-51`) ...
  (the skipped `agents-worktree-planner-scope`, step 66, was absorbed into it ...)". Verified
  against the TOML: `driver-output-generation` has `order = 67`, `status = "complete"`,
  `provenance.decisions = ["Q-51"]`, `folds = ["agents-worktree-planner-scope"]`;
  `agents-worktree-planner-scope` has `order = 66`, `status = "skipped"` and title "absorbed into
  `driver-output-generation`, which authors this clarification in the shared `isolation_policy`
  fragment (`skipped`)" (`docs/plans/agent-scaffold.plan.toml` step blocks at lines 868-888). Both
  step numbers and the "absorbed into" / "authored into the shared fragment" claims are correct.
- Genuinely guidance-only now: `planner-folds-decisions.md:11` "This step edits no prompt file and
  no source, including `src/isolation_policy.rs`: it is guidance-only, hand-editable `pack/AGENTS.md`
  prose only. It does not touch the closed list or its single source; it only names the planner ...
  at the three prose points (lines 41, 43, 71) ...". The three `pack/AGENTS.md` line cites resolve
  to the exact three points Q-67 names: `sed -n '41p;43p;71p' pack/AGENTS.md` gives 41 =
  "Human-input contract ... A resolved decision is recorded ... and folded into the step it affects"
  (actor-less passive), 43 = "reusing the intake and Open-Questions machinery" (Socratic mode),
  71 = "At every checkpoint the orchestrator updates this queue" (checkpoint). All three correct.
- TOML `ask` no longer overclaims: the Q-67 `ask` changed from "and reinforce that the ... closed
  list ... EXCLUDES authoring new questions or steps ..." (an action item) to "The exclusion of
  authoring new questions or steps ... is already carried by the generated `ISOLATION_POLICY_FRAGMENT`
  (`src/isolation_policy.rs:33`, rendered at `pack/AGENTS.md:91`) ... this pass restates none of that
  list and edits only the three actor-less `pack/AGENTS.md` prose points ..."
  (`git diff 44f848a cca1099 -- docs/plans/agent-scaffold.plan.toml`, the Q-67 `ask` line). The
  reinforce-scope overclaim is gone; it is now framed as already-covered, not a new edit.

### F2 (low, checks-reviewer omission) -> CLOSED

Reproduced closed: `reviewer-reproducible-evidence.md:14` now reads "Deliberately out of scope:
`pack/prompts/checks-reviewer.md` and its `.agents/prompts/checks-reviewer.md` copy. The
deterministic checks reviewer already reports every finding with the strongest evidence tier, the
check name, the exact command, its exit code, and the offending `file:line`, so it already complies
with (indeed exceeds) the tiered rule; the `reviewer.md` + `triager.md` + `pack/AGENTS.md` file set
is intentional, not an omission." `ls pack/prompts/` lists `checks-reviewer.md`, and its evidence
mandate is accurate: `pack/prompts/checks-reviewer.md:13` "Report each finding with a severity and
concrete, tool-evidenced detail: the check name, the exact command, its exit code, and the offending
`file:line` from the tool's output." The out-of-scope note's factual claim reproduces.

### F3 (low, backstop composition) -> CLOSED

Reproduced closed: `reviewer-reproducible-evidence.md:5` now ends "... DISMISSES any testable claim
whose demonstration does not reproduce; when that dismissed claim is HIGH or CRITICAL, the dismissal
still passes the existing second-triager backstop re-check (`pack/AGENTS.md:59`) before it counts
toward a clean round, so the new dismissal ground composes with the backstop rather than bypassing
it." `sed -n '59p' pack/AGENTS.md` is the backstop paragraph ("before a dismissed finding at or
above the backstop severity ... have a second, independent triager (or a human) confirm the
dismissal ..."). The cite is correct and the composition claim is accurate.

### F-fid (low, Option B parenthetical) -> CLOSED

Reproduced closed: the "(Option B)" parenthetical is dropped in both places.
- TOML: Q-66 `ask` now "DECIDED (Socratic, human, 2026-07-26): TIERED REPRODUCIBLE EVIDENCE."
  (no "(Option B)") per `git diff 44f848a cca1099 -- docs/plans/agent-scaffold.plan.toml`.
- Sidecar: `reviewer-reproducible-evidence.md:5` "The rule (tiered reproducible evidence)."
  (no "(Option B)").
- Receipt unchanged: `grep -F '"q_id":"Q-66"' docs/metrics/workflow.jsonl` still yields
  `"options":["Tiered reproducible evidence", ...]`, `"chosen":"Tiered reproducible evidence"`; the
  chosen option remains at array position A, so removing the contradicting "(Option B)" is the right
  close.
- No stray "(Option B)" remains in the Q-66/Q-67 fold: `grep -n "Option B"` over the four changed
  files hits only unrelated pre-existing content (Q-51 `ask` at `plan.toml:1542`, and the Q-56
  roles-findings-naming step at `agent-scaffold.md:1078` and `:1084`), none in the Q-66/Q-67 fold.

## Adversarial regression sweep (new issues from the fixes)

- Dropping Part 2 did not leave the step incomplete or inconsistent. Part 1
  (`planner-folds-decisions.md:7`) still delivers the decided Q-67 rule verbatim: name the planner
  as the folder of a non-trivial decided decision (authoring a `[[question]]` or `[[step]]`) at the
  human-input-contract, Socratic-mode, and checkpoint points, routed by the orchestrator. The
  checkpoint "surfacing / pushing, not authoring" clarification the old Part 2 carried is preserved
  inside Part 1. The cross-reference note (line 9) explains why the exclusion needs no restatement,
  so the intent is complete, not truncated. No internal contradiction: the step asserts it "restates
  NONE of that list" and "edits only the three actor-less `pack/AGENTS.md` prose points", consistent
  throughout.
- New step-number claims are correct: "step 67" (`driver-output-generation`) and "step 66"
  (`agents-worktree-planner-scope`) match the TOML `order` fields (67 / 66). No off-by-one.
- New line-number claims are correct: `src/isolation_policy.rs:33`, `pack/AGENTS.md:91`,
  `pack/AGENTS.md:59`, and `pack/AGENTS.md` lines 41 / 43 / 71 all resolve to the cited content
  (verified above). No new inaccuracy introduced.
- House style: `grep -nP '[^\x00-\x7F]'` over both changed step files returns nothing; the changed
  lines are ASCII-only.

## Gates

- `cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --workflow`:
  ```
  docs/metrics/workflow.jsonl: 204 records, valid
  docs/plans/agent-scaffold.plan.toml: 89 steps, 67 questions, valid
  docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
  ```
  exit 0.
- `cargo run --quiet -- render docs/plans/agent-scaffold.plan.toml --check --strict`:
  ```
  docs/plans/agent-scaffold.plan.toml: up to date
  ```
  exit 0. The rendered `docs/plans/agent-scaffold.md` is a faithful render of the edited source and
  sidecars, so the roadmap and sidecar copies are consistent with the TOML and step files.

## Verdict

All 4 round-1 fixes confirmed closed. Zero new findings. Clean round.
