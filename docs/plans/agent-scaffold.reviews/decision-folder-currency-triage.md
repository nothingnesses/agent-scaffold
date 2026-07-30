# Triage: `decision-folder-currency` (commit `065e511`)

Triager worktree: `.claude/worktrees/triage90`, detached at `065e511` (`git rev-parse HEAD` -> `065e5119b87a2d516caf8606a199ca12582543f6`). Independent of the implementer and of the orchestrator.

Findings judged:

- `docs/plans/agent-scaffold.reviews/decision-folder-currency-reviewer-fidelity.md`, F1 (`low`).
- `docs/plans/agent-scaffold.reviews/decision-folder-currency-reviewer-coldread.md`, F1 (`medium`).

## Verdict summary

| Finding | Reviewer severity | Verdict | Final severity |
| --- | --- | --- | --- |
| coldread F1 | `medium` | VALID (fix required) | `medium` (confirmed) |
| fidelity F1 | `low` | VALID BUT ACCEPT RESIDUAL | `low` (confirmed) |

## Citation audit (every cited `file:line` re-read at that line in this worktree)

All of them reproduce. No misnumbered citation in either findings file.

| Citation | Reproduces | What is actually there |
| --- | --- | --- |
| `pack/prompts/orchestrator.md:3` | Yes | "First, read `AGENTS.md` so you drive the workflow and honour the principles it defines." |
| `pack/prompts/orchestrator.md:25` | Yes | "...then mark it complete and move to the next." |
| `pack/prompts/orchestrator.md:27` | Yes | Checkpoint paragraph, carrying the new "Here 'update the plan's Open Questions queue' means..." clause and the "(the Checkpoints rule in `AGENTS.md`)" pointer. |
| `pack/prompts/orchestrator.md:29` | Yes | "Fold a trivial request (local, reversible, no change to scope or Success Criteria, no new open question) in directly". |
| `pack/prompts/orchestrator.md:31` | Yes | The three-branch Socratic sentence; branch 2 quoted verbatim in the coldread finding, character for character. |
| `pack/prompts/orchestrator.md:33` | Yes | Ledger paragraph with the new actor clause. |
| `pack/AGENTS.md:30` | Yes | "...the human-decision queue (`[[question]]` entries)...". |
| `pack/AGENTS.md:41` | Yes | Human-input contract, the authoritative statement. |
| `pack/AGENTS.md:43` | Yes | Socratic mode, quoted verbatim in the coldread finding. |
| `pack/AGENTS.md:63` | Yes | "Preventing relitigation", with the new actor clause and its retained reopen tail. |
| `pack/AGENTS.md:71` | Yes | "Checkpoints (the human-decision queue and progress)." |
| `pack/AGENTS.md:91` | Yes | The literal `{{isolation_policy}}` slot. |
| `AGENTS.md:91` (rendered) | Yes | The rendered fragment; its closed set is "flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor". No `[[question]]`, no `[[step]]`. |
| `AGENTS.md:97` | Yes | "Checkpoint and resuming after context loss." |
| `AGENTS.md:106` | Yes | Task-entry re-grounding, and it does contain "(the Checkpoints rule above)" referring to `:71`. The coldread's not-raised note is correct. |
| `src/plan/source.rs:62-64` | Yes | `/// The Open-Questions queue items ([[question]]).` / `#[serde(default, rename = "question")]` / `pub(crate) questions: Vec<Question>,`. |
| `src/plan/source.rs:302` | Yes | "/// One Open-Questions queue item (`[[question]]`)." |
| `src/agents_md_drift.rs:125` | Yes | `const PROMPT_DEST_PREFIX: &str = ".agents/prompts/";` |
| `src/agents_md_drift.rs:415` | Yes | `fn the_committed_role_prompts_match_a_fresh_render()`, with the non-empty assertion at `:434` and the `assert_eq!` over `normalize_wrapping` at `:450`. |
| `grep -ic question src/isolation_policy.rs` | Yes | Returns `0`. |

Other reproductions run:

- `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` -> no output, exit 0.
- `cargo test` -> 367 lib + 5 + 1 + 3 + 1 + 2 pass, 0 failures. `agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render` and `::the_committed_scaffold_matches_a_fresh_render` both pass. The known `checks::tests` worktree-naming flake did not fire on this run.
- `git diff --name-only 065e511~1 065e511` -> the five expected files and nothing else.
- Plan source at this tip: `prompt-drift-guard` (order 92) is `status = "complete"`; `decision-folder-currency` (order 90) is `status = "next"`.

## coldread F1: VALID (fix required). Severity `medium`, confirmed.

### The narrow question, settled

Is recording a decided Socratic answer as an Open-Questions item itself an act of authoring a `[[question]]` (planner work), or can it be a trivial amendment (orchestrator work)?

On the path the sentence describes, it is authoring a `[[question]]`. Grounds:

1. The Open Questions queue is `[[question]]` entries in the plan TOML, not a separate artifact (`pack/AGENTS.md:30`; `src/plan/source.rs:62-64`, `:302`). The reviewer's premise holds.
2. The Socratic entry mode has no create-then-decide stage. `pack/AGENTS.md:43` describes the whole flow: the human asks, the orchestrator answers with the contract, the human decides, "and the resolved answer becomes a durable Open-Questions decision like any other". Nothing registers the question in the queue first. The contrast is decisive: the exploration mode two paragraphs later (`pack/AGENTS.md:45`) does spell out a two-stage flow, "records the question as an Open-Questions item with status `exploring` ... only then does it move the item to `open`". The Socratic mode's authors wrote a create stage where they meant one, and did not write one here.
3. The schema makes the created entry substantial rather than a field touch. `QuestionStatus` (`src/plan/source.rs:333-343`) has no pre-`decided` waypoint the Socratic path passes through, and `folded_into` (`:314-317`) is "present (and required) when the item is `decided`", so recording a Socratic decision means minting a `[[question]]` AND naming the step it folded into, which either already exists or must itself be authored as a `[[step]]`. Both of those are the two acts the shipped definition of "non-trivial" names.

The trivial-amendment case is real but is not this case: if the human's question happens to match an already-queued `open` item, flipping it to `decided` and filling `folded_into`/`receipt` is a field amendment. That case exists and the qualifier must keep protecting it. It is not the case branch 2 covers.

Does the sentence as written let a careful reader land on the wrong one? Yes. "Record" is a finite second-person imperative whose object is the queue entry; the routing clause's object is a different, grammatically subordinate noun, "its fold", whose possessive antecedent is the decision. Two nouns, one commanded to the reader and one routed away, reads as two acts. An orchestrator that takes the recording as its own and routes only the downstream `[[step]]` work has followed the sentence literally and has authored a `[[question]]` directly on main, which `pack/AGENTS.md:41` routes to the planner, `:71` names as not its job, and the rendered isolation-policy fragment leaves outside its closed direct-on-main set (`AGENTS.md:91`; `grep -ic question src/isolation_policy.rs` -> `0`).

### The implementer's counter-argument, weighed

Both of its grounds fail on inspection.

**Counter 1: deleting "record the resolved answer" would move every fold off the orchestrator, including the trivial ones, which the brief's scope constraint forbids.** False premise. The scope constraint (brief line 26) forbids writing "an unqualified 'the planner authors that fold'". Re-voicing the recording verb out of the second-person imperative while keeping "non-trivial (authoring a `[[question]]` or a `[[step]]`)" verbatim moves no trivial fold anywhere: the trivial case stays implicitly the orchestrator's, exactly as it does at `pack/AGENTS.md:43`, which uses that construction and is the passage the brief told the implementer to match. The implementer treated "keep the imperative" and "keep trivial folds with the orchestrator" as one choice. They are two, and the fix below takes the second without the first.

**Counter 2: it matched the structure of `pack/AGENTS.md:43`.** False on inspection, and this is the point the finding is about. `:43` reads "the resolved answer **becomes** a durable Open-Questions decision like any other, its non-trivial fold **routed** to the planner to author as above rather than **edited in** directly". All three verbs are actor-less. The prompt keeps "record" as an imperative addressed to the orchestrator and assigns only the routing clause. The two files diverge at exactly the recording verb, which is the reviewer's cross-file sub-claim, and it reproduces.

### The brief anticipated and forbade this outcome

This is not a reviewer inventing a requirement. The brief names branch 2 as class "ACTOR NAMED, WRONG ACTOR (a contradiction; the fix CARVES OUT or REASSIGNS)" and says at line 8 that "adding a mention of the planner without narrowing what the orchestrator is told to do would leave the paragraph saying both things at once", then at line 20 instructs: "make the second branch say the planner authors the non-trivial fold and the orchestrator routes it ... **rather than leaving 'record the resolved answer' addressed to the orchestrator**". The implementer added the mention and left the imperative. The paragraph now says both things at once, which is the outcome the brief named in advance. The fidelity reviewer's check 8 examined the added wording and did not examine the surviving wording, which is how it read clean.

### Severity: `medium`, confirmed, not raised and not lowered

Not lower than `medium`: the defect is in the operative instruction the orchestrator reads at every session start; the behaviour at risk is the exact misstep the whole `Q-67` line exists to prevent, and the brief (line 60) records that this ambiguity has already caused one real misstep; and one of the four passages the step exists to fix is not fixed.

Not `high`: three of the four passages landed correctly, `pack/AGENTS.md:41`, `:43` and `:71` are unambiguous and `pack/prompts/orchestrator.md:3` orders the orchestrator to read `AGENTS.md` first, the trailing "rather than editing the plan yourself" pushes the right way, and the worst outcome is a visible, revertible diff on main. Nothing security-, data-, safety- or money-sensitive, and nothing hard to roll back.

The reviewer's second (over-restricted) reading is also real, and I confirmed both collisions it cites: `pack/prompts/orchestrator.md:25` licenses "mark it complete" and `:29` licenses "Fold a trivial request ... in directly", both of which are plan edits. It is a sub-symptom of the same unclear clause boundary rather than a separate defect, and the fix below narrows the phrase's object enough to mitigate it without a second edit.

### The minimal fix

Single site. `grep -rn "record the resolved answer"` across the tree returns the construct in exactly ONE authored source, `pack/prompts/orchestrator.md:31`, plus its one generated copy at `.agents/prompts/orchestrator.md:31`. Every other hit is a plan, brief, or findings document quoting it. No other pack file, prompt, user prompt, or template carries the construct.

Edit branch 2 only, at `pack/prompts/orchestrator.md:31`.

Old:

> for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision, routing its fold to the planner to author when that fold is non-trivial (authoring a `[[question]]` or a `[[step]]`) rather than editing the plan yourself;

New:

> for a question whose options are already clear, emit the block, and the resolved answer becomes a durable Open-Questions decision whose non-trivial fold (authoring a `[[question]]` or a `[[step]]`) you route to the planner to author rather than editing the plan yourself;

Why this and not more. It is a re-voicing, not new prose: 45 words to 43, no rule asserted that was not already there. It removes the only orchestrator-addressed imperative in the branch that commands a plan-source write, leaving "emit the block", which is correctly the orchestrator's. It keeps the shipped qualifier verbatim, so a trivial fold stays with the orchestrator and the brief's scope constraint is untouched. It names both actors at the point of use (brief line 27). It mirrors `pack/AGENTS.md:43`'s "becomes ... routed to the planner to author ... rather than" construction, so the cross-file divergence closes as a side effect rather than needing its own edit. It narrows the object of "rather than editing the plan yourself" to the fold, which mitigates the over-restricted reading.

Two constraints on any alternative wording the implementer prefers:

1. **No semicolon inside branch 2.** The three branches of this sentence are delimited by semicolons, so an internal one makes it read as four branches. The reviewer's own suggested wording ("emit the block; the resolved answer becomes...") introduces exactly that and should not be taken literally.
2. **Branch 3 is byte-untouched** (`Q-69`). The fix above does not touch it and does not change how unevenly the two branches read against each other, which is settled and accepted.

Regeneration, mandatory and part of the same commit: run the render half only, `cargo run -- scaffold --output-dir . --write --force --principles default --instrument`, then confirm `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` is empty. `pack/AGENTS.md` is not touched by this fix, so `AGENTS.md` and `.agents/AGENTS.reference.md` should come back unchanged. Do not run `just scaffold-self` (repo-wide `nix fmt`, forbidden to an implementer by `pack/AGENTS.md:79`).

## fidelity F1: VALID BUT ACCEPT RESIDUAL. Severity `low`, confirmed.

The claim reproduces in full. `docs/plans/agent-scaffold.steps/decision-folder-currency.md:38` states that `.agents/prompts/orchestrator.md` "has NO whole-file drift-guard test, so nothing fails if the regeneration is skipped and the staleness would be silent", and line 30 calls the hand `diff` "the one part of the currency work no test enforces". At `065e511` both are false: `the_committed_role_prompts_match_a_fresh_render` (`src/agents_md_drift.rs:415`) filters the fresh render by `PROMPT_DEST_PREFIX` (`:125`, `.agents/prompts/`), asserts the filtered set is non-empty so it cannot pass vacuously (`:434`), and byte-compares each committed copy under `normalize_wrapping` (`:450`). It passes at this tip. The plan source confirms the ordering the reviewer asserts: `prompt-drift-guard` (order 92) is `complete` while `decision-folder-currency` (order 90) is `next`, so the gap the brief describes as open was closed before this step ran.

Accepted as residual rather than sent back, on four grounds.

1. It is not a defect in the change under review. Step 92 made the brief stale, not commit `065e511`, so it falls outside this change's own documentation-currency obligation (`pack/AGENTS.md:33`).
2. The staleness over-warns rather than under-warns. Its operative instruction is "check this by hand", so acting on the stale note costs a redundant `diff` and nothing else. A future prompt edit that skips regeneration now fails `cargo test` loudly, so the note cannot license a silent failure.
3. The fix is out of the implementer's hands (the brief forbids it editing the step file) and needs a planner pass plus a re-render, since the text is projected into `docs/plans/agent-scaffold.md` as well as living in the sidecar.
4. The sidecar becomes a historical scheduling record the moment step 90 closes, and the brief already treats its own content as as-of-scheduling ("quoted as of the commit that schedules this step", line 10).

Advice, not a requirement: if a planner pass runs against the plan for any other reason before step 90 closes, striking the "NOTE: unlike the two guidance copies below, this one has NO whole-file drift-guard test..." sentence at line 38 and the "This is the one part of the currency work no test enforces" clause at line 30 is a near-free deletion-class amendment worth folding in. Do not spawn a planner solely for it.

## Scoring recommendation (advice to the orchestrator; the orchestrator decides)

**NEW_VALID.** One valid `medium` finding requiring a fix (coldread F1) and one valid `low` accepted as residual. An accepted residual does not block convergence, but the `medium` does: the consecutive-clean streak resets to zero, the implementer applies the branch-2 fix and regenerates, and a fresh round runs on the revised artifact.

No dismissals, so no high-or-critical dismissal re-check is owed on this round.
