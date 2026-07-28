# Plan review round 4, holistic lens: `decision-folder-currency` (step 90), `prompt-drift-guard` (step 92), `Q-69`

Reviewer: independent, worktree-isolated, holistic lens. Artifact: `ab6c01d..b36c4c6` on `plan/decision-folder-currency`, reviewed as the CURRENT full text of the two step sidecars and the `Q-69` item, not only the diff. Worktree `.claude/worktrees/rev4-holistic` at `b36c4c6`. Every line cited below was re-read in this worktree immediately before citing.

Answer to the orchestrator's question 1 (could a competent implementer execute step 90 from its sidecar alone, correctly, without asking anything): NOT FULLY. For `pack/prompts/orchestrator.md:27` and `:31` the answer is yes; the passages are located exactly, the class is right, and the branch boundary is unambiguous. For the two ledger clauses (`pack/prompts/orchestrator.md:33` and `pack/AGENTS.md:63`) the answer is no as written: see `H4-1`.

Findings: 2 medium, 3 low. No high, no critical: I looked for one and found none.

---

## `H4-1` (medium): the two ACTOR-LESS instructions drop the "non-trivial" qualifier that every shipped counterpart carries, so a literal execution manufactures a new contradiction

The sidecar's per-passage instructions are inconsistent about the qualifier. For the Socratic branch it carries it:

- `docs/plans/agent-scaffold.steps/decision-folder-currency.md:20`: "make the second branch say the planner authors the non-trivial fold and the orchestrator routes it".

For the two ledger clauses it does not:

- `:21`: "`pack/prompts/orchestrator.md`, the ledger paragraph (actor-less, so ADD the actor): name the planner on the "only durable decisions ... fold into it" clause."
- `:22`: "`pack/AGENTS.md`, the "Preventing relitigation (the ledger)" paragraph (actor-less, so ADD the actor): same, on its closing "only durable decisions ... fold into the plan's steps" clause."

And the requirement summary at `:24` does not restore it: "The requirement is that each passage names the actor at its point of use ... that the checkpoint paragraph ends up saying what `pack/AGENTS.md:71` says, and that the two ledger-paragraph copies (prompt and guidance) end up saying the same thing."

Every already-shipped statement of this rule, all three of them authored by step 89, is qualified:

- `pack/AGENTS.md:41`: "when that fold is non-trivial (authoring a `[[question]]` or a `[[step]]`), the orchestrator routes it to the planner to author, as on the request-interrupt path above, rather than editing the plan directly."
- `pack/AGENTS.md:43`: "its non-trivial fold routed to the planner to author as above rather than edited in directly."
- `pack/AGENTS.md:71`: "not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job (routed as above)".

The qualifier is load-bearing, not decorative: `pack/AGENTS.md:41` explicitly contemplates a fold that is NOT non-trivial, and `pack/AGENTS.md:39` says such a request "may be folded in directly" (by the orchestrator, per `pack/prompts/orchestrator.md:29`, "Fold a trivial request ... in directly"). An implementer who follows `:21` and `:22` literally writes something of the form "only durable decisions, the ones that change the plan, fold into the plan's steps, and the planner authors that fold". Unqualified, that says the planner authors EVERY durable decision's fold, which contradicts `pack/AGENTS.md:41`, in the same shipped document, three paragraphs above.

Why this matters more than a wording nit: an unqualified restatement sitting beside a qualified one is precisely the defect `Q-69` exists to resolve. `docs/plans/agent-scaffold.plan.toml:1722` describes it: "on the main clause the exclusion reaches only a DECIDED question's fold, while on the trailing clause it reaches a `[[question]]` of any status ... This is a CONTRADICTION between two halves of one shipped sentence". Step 90 is the step that removes that class of defect; as written it can add a second instance of it, in a file that ships into every scaffolded project.

Suggested fix (one clause, no scope change): make `:21` and `:22` name the qualifier the way `:20` does, or add one sentence at `:24` to the effect that every added actor clause carries the same non-trivial / `[[question]]`-or-`[[step]]` scope the guidance already uses at `pack/AGENTS.md:41`, `:43`, and `:71`.

## `H4-2` (medium): `prompt-drift-guard.md` still describes a step 91 that no longer exists, and asserts a build order that contradicts the plan

`docs/plans/agent-scaffold.steps/prompt-drift-guard.md:23`: "which is the reason it can land independently of steps 90 and 91."

`docs/plans/agent-scaffold.steps/prompt-drift-guard.md:25`: "Interaction with steps 90 and 91. This step does not block either of them and neither blocks it. ... the order given (90, then 91, then 92) reflects that step 90 is already reviewed and in flight".

There is no step 91. `docs/plans/agent-scaffold.plan.toml:1243` and `:1256` give `order = 90` and `order = 92`, and the plan's own record of the removal (`docs/plans/agent-scaffold.plan.toml:1734`) says the step "was removed on 2026-07-28". The ledger records the intent as "`order = 92` kept, leaving an honest gap at 91" (`docs/plans/agent-scaffold.ledger.md:347`). The gap is not honest while a sidecar states "the order given (90, then 91, then 92)" as fact.

The stale text is also in the committed projection, at `docs/plans/agent-scaffold.md:1267` and `:1269`, so it reaches the human-readable plan. Both come from the sidecar, so one edit plus a re-render fixes both.

Reproduce:

```
grep -n "steps 90 and 91\|90, then 91, then 92" docs/plans/agent-scaffold.steps/prompt-drift-guard.md docs/plans/agent-scaffold.md
grep -n "^order = " docs/plans/agent-scaffold.plan.toml | tail -3
```

Possible overlap: the round-4 brief says a parallel reviewer is checking that the removed step left no dangling references. I raise it anyway because the ledger's instruction for that check is slug-scoped ("that no dangling `exploring-item-actor-boundary` STEP reference survives", `docs/plans/agent-scaffold.ledger.md:351`) and these two references use the NUMBER, not the slug, so a slug sweep misses them. If the parallel reviewer caught it, treat this as a duplicate.

## `H4-3` (low): the derived-from-manifest guard silently loses a direction the enumerated form it rejects already covers

`docs/plans/agent-scaffold.steps/prompt-drift-guard.md:19` chooses to "DERIVE the guarded set from the manifest: render the self-scaffold asset set once, keep every asset whose `dest` starts with `.agents/prompts/`, and compare each against the committed file read from `CARGO_MANIFEST_DIR`", and states one trade-off to accept knowingly (hermeticity).

There is a second trade-off it does not state. Because the guarded set is derived from the RENDER, a prompt removed or renamed in `pack/prompts/` drops out of the set, and the orphaned committed `.agents/prompts/<role>.md` then goes unguarded and undeleted (the scaffolder never deletes assets). The enumerated form it rejects does catch this: `self_scaffold_asset` panics when the render lacks the named asset (`src/agents_md_drift.rs:66-69`, "the self-scaffold render includes an asset at {dest}").

This interacts with how the acceptance bullet is phrased: `:10` says "The guard is a two-way correspondence check, not a one-way staleness check", which reads as a claim about set correspondence as well as edit direction, and it will not be true in the membership direction.

The step is still sound and the derived form is still the better choice; this is an unstated limitation, not a wrong decision. A one-sentence note (that the derived set covers rendered assets only, and that a retired prompt leaves an unguarded orphan) would make the acceptance bullet honest, or a `.agents/prompts/` directory listing compared against the derived dest set would close it. Note if closing it: `.agents/prompts/checks-reviewer.md` is the one module-gated asset and is absent from the committed tree today (`ls .agents/prompts/` returns 7 files, `ls pack/prompts/` returns 8), so a strict set-equality check passes as-is.

## `H4-4` (low): the one unenforced part of step 90 has no check anyone could run

`docs/plans/agent-scaffold.steps/decision-folder-currency.md:30` identifies the exposure precisely: `.agents/prompts/orchestrator.md` "has NO whole-file drift-guard test, so nothing fails if the regeneration is skipped and the staleness would be silent ... regenerating it is the single easiest thing to forget here."

The requirement list at `:24` then contains no criterion covering it. The three requirements are all about the prose. Nothing in the sidecar tells the implementer, or the work reviewer, how to confirm the deployed prompt copy actually moved. The live silent path is narrow but real: running the render command at `:36` updates all three deployed files at once, and two of the three are guarded, so simply forgetting the command fails `cargo test`; what fails silently is hand-editing the two deployed guidance copies to match instead of regenerating, which satisfies the drift guard and leaves the prompt copy stale.

An exact check exists and costs one line. Verified in this worktree: `pack/prompts/` contains no `{{...}}` render slot (`grep -rn "{{" pack/prompts/` returns nothing), and all seven deployed prompts are byte-identical to their pack sources today:

```
for f in orchestrator planner reviewer triager implementer clarifying-questions open-questions-gate; do
  diff -q pack/prompts/$f.md .agents/prompts/$f.md; done
```

returns nothing. Suggested fix: add to `:24` that `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` must be empty after the step.

## `H4-5` (low): "the same sentence" is inaccurate, and the derived requirement invites an out-of-scope unification

`docs/plans/agent-scaffold.steps/decision-folder-currency.md:15` says of `pack/AGENTS.md:63` that it is "the same sentence as the one above in the guidance rather than in the prompt, so fixing either one alone would leave the two copies saying different things", and `:24` requires "that the two ledger-paragraph copies (prompt and guidance) end up saying the same thing".

The two are not the same sentence and do not currently say the same thing:

- `pack/prompts/orchestrator.md:33`: "do not put individual findings in the plan's Open Questions section; only durable decisions, the ones that change the plan, fold into it."
- `pack/AGENTS.md:63`: "Never put individual findings in the plan's Open Questions section; only durable decisions, the ones that change the plan, fold into the plan's steps, and a folded decision reopens only by evidence that beats its recorded reasoning."

Impact is limited: both are edited by the same step, so "fixing either one alone" is not a live option, and the intended reading of `:24` (name the same actor the same way) is recoverable. The risk is that an implementer takes "end up saying the same thing" at face value and also unifies "fold into it" with "fold into the plan's steps", or drops the "reopens only by evidence" tail, which would be exactly the silent scope expansion this step is disciplined against (`:50`, "No silent scope expansion" applied in both directions). Suggested fix: scope the `:24` requirement to the actor clause.

---

## Verified clean (checked, and not a finding)

Step 90, executability:

- All citations in the step-90 sidecar reproduce exactly against this worktree: `pack/prompts/orchestrator.md:27` (the checkpoint paragraph, "There, update the plan's Open Questions queue and push its open items to the human"), `:31` (the Socratic paragraph, "for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision"), `:33` (the ledger paragraph); `pack/AGENTS.md:41`, `:43`, `:45`, `:63`, `:71`, `:79`, `:91` (the `{{isolation_policy}}` slot), `:108`; `justfile:46-48`; `pack/LEDGER.template.md:3`; `pack/user-prompts/explore.md:3`, `:7`, `:13`; `pack/pack.toml:166-167`; `src/manifest.rs:615`. No misnumbering found.
- The TWO-CLASS split is correct against the real lines. `:33` and `pack/AGENTS.md:63` state the folding with an intransitive "decisions fold into ..." and name no author; `:27` and `:31` are second-person imperatives inside paragraphs addressed to the orchestrator, so they do name an actor and it is the wrong one. Calling the first pair "passive voice" is loose grammatically but the substantive claim holds.
- The three-branch boundary at `pack/prompts/orchestrator.md:31` is unambiguous. The sentence is "If the human drives by asking a question rather than giving a task, pick the mode it needs: answer a purely factual question directly; for a question whose options are already clear, ...; for one whose design space is not yet decidable ..., record it as an `exploring` Open-Questions item, ...". Three semicolon-separated branches, each quoted in the sidecar at `:20`, with an explicit "EDIT THE SECOND BRANCH ONLY" and an explicit "Leave branch 3 exactly as it is". An implementer could not touch the wrong branch by accident.
- The regeneration command at `:36` is exactly `justfile:47` (`cargo run -- scaffold --output-dir . --write --force --principles default --instrument`), with only `justfile:48` (`nix fmt`) omitted. Every flag exists: `output_dir`, `force`, `write`, `principles`, `instrument` at `src/main.rs:385`, `388`, `394`, `400`, `420`. The config matches the one the drift guard pins for its render (`src/agents_md_drift.rs:52-64`: built-in pack, `default` selection, `Summary` detail, no vars, no modules, `instrument = true`), so the raw render is compared against the right thing. The `nix fmt` prohibition is correctly grounded in `pack/AGENTS.md:79` and `:108`.
- The claim that the raw render satisfies the guard without the formatter holds: `src/agents_md_drift.rs:24-29` records that the raw render is byte-identical to both committed files, and both sides pass through `normalize_wrapping` (`:313-322`) regardless.
- The deployed-file set at `:28-34` is complete. `.agents/prompts/orchestrator.md` is a manifest asset (`src/manifest.rs:604`); `AGENTS.md` and `.agents/AGENTS.reference.md` are the two guarded renders and are currently byte-identical to each other. Nothing else is affected: no file under `src/`, `tests/`, `build.rs`, `README.md`, or `CHANGELOG.md` asserts on or quotes any of the four passages (`grep -rn "Open Questions queue\|only durable decisions\|updates this queue\|Open-Questions decision" src/ tests/ build.rs README.md CHANGELOG.md` returns only unrelated hits), so there is no hidden test to break.
- The no-restatement constraint at `:26` is correctly grounded. The rationale it quotes ("author no reviewed product content and so stay the orchestrator's direct job rather than a spawned agent's") is verbatim from `src/isolation_policy.rs:33`, and `pack/prompts/orchestrator.md` carries no `{{...}}` slot, so the sidecar's instruction to reference the rule in `AGENTS.md` rather than reproduce the fragment is the right resolution for a prompt that has no fragment "below" to point at.
- CHANGELOG silence is consistent, not a gap. Neither `planner-folds-decisions.md` nor `agents-md-drift-guard.md` mentions CHANGELOG, `CHANGELOG.md` carries no entry for the `Q-67` work, and its last commit is `3db79c4`. Step 90 continues existing practice.
- Running the prescribed command inside an isolated git worktree is safe: `inside_git_repo` (`src/main.rs:174-178`) tests `ancestor.join(".git").exists()`, which is true for a worktree's `.git` FILE, so `init_plan` returns `SkipExists` rather than re-initialising.

Step 92, soundness:

- Every code claim reproduces. `src/agents_md_drift.rs:45` and `:49` are the two `include_str!` embeds; no `include_str!` anywhere in `src/` touches `.agents/prompts/`. `normalize_wrapping` is at `:232` and `assert_no_unprotected_construct` at `:99`, both as described, and the precondition is asserted on both sides at `:308-311`. The "seven files" count is right (7 files in `.agents/prompts/`, 8 in `pack/prompts/`).
- The `checks-reviewer.md` caveat is correct: it is module-gated (`src/manifest.rs:658`, `:685`), absent from the no-module manifest list (`src/manifest.rs:604-619`), and absent from the committed tree, so a guard expecting it would indeed fail on a correct tree. Under the derived form the filter excludes it automatically.
- The precondition will NOT trip today. I ran the exact canonical-form predicate from `src/agents_md_drift.rs:118` (fence-aware, `split_whitespace().join(" ")` equality) over all 7 files in `.agents/prompts/`, all 6 in `.agents/user-prompts/`, and `.agents/LEDGER.template.md`: zero violations. So step 92 is executable without first hardening `normalize_wrapping`, and the widening the scope boundary contemplates is likewise unblocked.
- The mutation demonstration required at `:12` will actually work: `build.rs` emits `cargo:rerun-if-changed` recursively for `pack/` and every file under it, so a pack-prompt edit forces a re-embed and the failure is visible; the deployed side is read at test time, so a hand edit needs no rebuild at all. Both acceptance bullets `:9` and `:10` are demonstrable as written.
- Ordering: no hazard. Neither step blocks the other, and the claim is true in both directions. If 92 lands first the guard passes immediately (all seven copies are in sync now) and step 90's regeneration becomes enforced; if 90 lands first, nothing about 92 changes. The two sidecars agree; `decision-folder-currency.md:34` ("this step must not wait on it") and `prompt-drift-guard.md:25` are not in conflict.
- Step 92 having no `[step.provenance]` is consistent with practice, not a defect: 30 of 91 steps carry one, and all 8 uses of the `findings` field point at durable exploration or design documents, never at a transient `.reviews/` file. The sidecar's justification rests on its own reproducible `include_str!` sweep rather than on the reviewer count, so nothing becomes uncitable when the round's findings files are deleted.

`Q-69`:

- All of its evidence reproduces. `grep -c "Q-68" docs/metrics/workflow.jsonl` returns 0. Commit `b6ba317` exists and adds `Q-68` to `docs/plans/agent-scaffold.plan.toml` plus an empty `Q-68.md`. `exploring` is a typed `QuestionStatus` variant (`src/plan/source.rs:337`, `:363`). `ISOLATION_POLICY_FRAGMENT` (`src/isolation_policy.rs:33`) contains no occurrence of "question". The ledger's "NEW BACKLOG (captured 2026-07-26 by a planner, per Q-67)" entry and the step-89 accepted-residual note naming "`pack/prompts/orchestrator.md:33` AND the parallel `pack/AGENTS.md:63`" are both present and findable by their quoted text.
- The empty `docs/plans/agent-scaffold.questions/Q-69.md` matches all 69 question sidecars, every one of which is 0 bytes. The exploration directory `docs/plans/exploring-item-actor-boundary.explorations/` not existing yet matches the `Q-68` precedent the item cites.
- The item is self-contained for a resuming agent: it states the contradiction with both halves quoted, records both unsettled premises, keeps the candidate directions explicitly non-recommended, names where explorers write and what they owe back, and warns against deciding before the pass. Nothing in it depends on conversation-only facts.

Whole artifact:

- Coherence between the three items holds apart from `H4-2`. Step 90 edits branch 2 of `pack/prompts/orchestrator.md:31` and `Q-69` owns branch 3; both say so, and the sidecar accepts in writing that the sentence reads oddly in the interim (`:20`). Step 92 would guard the deployed copy of the file step 90 edits, and neither depends on the other landing first.
- `cargo run -- render docs/plans/agent-scaffold.plan.toml --check --strict` reports "up to date" (exit 0). `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --plan docs/plans/agent-scaffold.md --workflow` reports 212 records valid, 91 steps / 69 questions valid, workflow invariants hold (exit 0). The Status line's counts reconcile: 3 + 2 + 60 + 4 + 1 + 3 + 18 = 91.
- Not re-raised, per the round-4 brief: `R2-3` (the "three actor-less `pack/AGENTS.md` prose points" clause in the `Q-67` ask), `T-4` (the four-item `ISOLATION_POLICY_FRAGMENT` paraphrase at `decision-folder-currency.md:26`), `Q-69` quoting the fragment's operative clause plus its item count without reproducing the items, and the intentional `order` gap at 91 (`validate` does not object). I found no new evidence against any of those verdicts.
