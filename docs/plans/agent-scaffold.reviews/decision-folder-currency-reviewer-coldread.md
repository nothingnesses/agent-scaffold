# Reviewer findings: `decision-folder-currency` (cold-read lens)

Commit under review: `065e511`. Worktree: `.claude/worktrees/rev90-b`, detached at `065e511`.

Lens: read `pack/prompts/orchestrator.md` and `pack/AGENTS.md` as they now stand, the way the agent they instruct would, rather than from the step brief. All line numbers below were read at that line in this worktree.

## Summary

One finding, severity `medium`. Zero findings at `low`, `high`, and `critical`.

Answers to the four questions asked:

1. Mostly yes, with one soft spot. The checkpoint clause (`pack/prompts/orchestrator.md:27`) and the ledger clause (`:33`) land the rule correctly and are neither over- nor under-restricted. The Socratic branch-2 clause (`:31`) is the soft spot: see F1.
2. No. Every assertion the change adds is true against the rest of the two documents. Checked: the cross-reference target "the Checkpoints rule in `AGENTS.md`" resolves (`AGENTS.md:71`, "Checkpoints (the human-decision queue and progress)"); "the planner authors that fold when it is non-trivial (authoring a `[[question]]` or a `[[step]]`)" restates `pack/AGENTS.md:41` verbatim in substance; `[[question]]` and `[[step]]` are real plan-source entities (`pack/AGENTS.md:30`; `src/plan/source.rs:62-64`, `:302`).
3. No contradiction between the four edited passages, in either file. `:27`'s object ("a decided decision's `[[question]]` or `[[step]]` fold") is exactly what `:31` and `:33` define as the non-trivial fold, so the three prompt passages agree with each other and with `pack/AGENTS.md:41`, `:63`, and `:71`. The one cross-file divergence I found is inside F1.
4. Yes, at one point: F1.

## F1 (`medium`). At `pack/prompts/orchestrator.md:31` the surviving imperative "record the resolved answer as a durable Open-Questions decision" is itself the `[[question]]` authoring that the clause appended to it routes away, so the sentence can be read as licensing the exact edit the change exists to forbid

Evidence. `pack/prompts/orchestrator.md:31`, branch 2 of the three-branch sentence, now reads:

> for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision, routing its fold to the planner to author when that fold is non-trivial (authoring a `[[question]]` or a `[[step]]`) rather than editing the plan yourself

The main verb, "record", is a second-person imperative addressed to the orchestrator. What it commands is an edit to the plan's Open Questions queue, and that queue is `[[question]]` entries in the plan TOML, not a separate artifact:

- `pack/AGENTS.md:30`: the `<task>.plan.toml` skeleton holds "the human-decision queue (`[[question]]` entries)".
- `src/plan/source.rs:62-64`: `#[serde(default, rename = "question")] pub(crate) questions: Vec<Question>`, documented at `:62` as "The Open-Questions queue items (`[[question]]`)".
- `src/plan/source.rs:302`: "One Open-Questions queue item (`[[question]]`)".

On the Socratic path there is no pre-existing item to amend: the human asks a fresh question, so recording its resolved answer as "a durable Open-Questions decision" means creating a new `[[question]]`. The same sentence names "authoring a `[[question]]`" as the non-trivial fold that must be routed to the planner. So the sentence commands the orchestrator to do the thing it then tells it to route.

Two readings, both reachable by a reasonable agent, and the wrong one is the more natural:

- Under-restricted (the likelier reading, and the one that is wrong). "Recording" and "the fold" are separate acts, so the orchestrator authors the new `[[question]]` itself and routes only downstream `[[step]]` work. This re-permits the orchestrator authoring a `[[question]]` directly on main, which `pack/AGENTS.md:41` routes to the planner and `pack/AGENTS.md:71` names as not the orchestrator's job, and which the rendered isolation-policy fragment leaves outside the orchestrator's direct-on-main set (`AGENTS.md:91`; the const carries no occurrence of the word at all, reproducible as `grep -ic question src/isolation_policy.rs` -> `0`).
- Over-restricted (less likely, but available). "Rather than editing the plan yourself" is read as a general prohibition, because "the plan" is unqualified. That collides with `pack/prompts/orchestrator.md:25` ("mark it complete") and `:29` ("Fold a trivial request ... in directly"), both of which are plan edits the orchestrator is licensed to make.

Cross-file divergence, part of the same defect. The guidance counterpart at `pack/AGENTS.md:43` uses an actor-less verb, "the resolved answer becomes a durable Open-Questions decision like any other, its non-trivial fold routed to the planner to author as above rather than edited in directly". The prompt instead assigns the recording to the orchestrator in the imperative. So after this change the two files no longer say the same thing about who records a Socratic decision: the guidance assigns no one, the prompt assigns the orchestrator, and assigning it to the orchestrator is what conflicts with the routing rule both files carry.

Why `medium` and not lower. This is the one of the four passages where the pre-change text was described as an actor contradiction rather than an omission, and the appended clause did not remove the contradicting imperative, it sat beside it. The file is read by the orchestrator at every session start and has no whole-file drift guard, and the behaviour at risk (the orchestrator authoring a decided decision's `[[question]]` on main) is the specific misstep the whole `Q-67` line of work exists to prevent.

Why not higher. The trailing "rather than editing the plan yourself" and the two guidance passages (`pack/AGENTS.md:41`, `:71`) both push toward the correct behaviour, so a careful agent that follows the prompt's own instruction to read `AGENTS.md` first (`pack/prompts/orchestrator.md:3`) can recover the right rule. The damage, if it lands, is a visible uncommitted or committed diff on main, not a silent or unrecoverable state change.

Suggested direction (the fix is the reviewer's suggestion, not a requirement). Make the recording share the routing, for example by replacing the imperative with the guidance's own construction: "for a question whose options are already clear, emit the block; the resolved answer becomes a durable Open-Questions decision, whose fold you route to the planner to author when it is non-trivial (authoring a `[[question]]` or a `[[step]]`) rather than authoring it yourself". That removes the imperative that conflicts, keeps the shipped non-trivial qualifier unchanged, restates nothing from the generated fragment, and leaves branch 3 untouched.

## Checked and deliberately not raised

These were examined and judged not to be findings; recorded so the coverage is visible.

- The pointer "(the Checkpoints rule in `AGENTS.md`)" at `pack/prompts/orchestrator.md:27` against the two similarly named sections, `AGENTS.md:71` "Checkpoints (the human-decision queue and progress)" and `AGENTS.md:97` "Checkpoint and resuming after context loss". `AGENTS.md:106` already uses "the Checkpoints rule above" to mean `:71` and the spelled-out section title to mean `:97`, so the prompt follows an established convention and resolves unambiguously.
- `pack/prompts/orchestrator.md:27` carrying no explicit "non-trivial" qualifier. Its object, "a decided decision's `[[question]]` or `[[step]]` fold", is exactly what `:31` and `:33` define the non-trivial fold to be, so the qualifier is carried by the object. It also matches `pack/AGENTS.md:71`'s main clause word for word in substance.
- `pack/AGENTS.md:63`'s new clause against `:41`. Consistent; the `:63` copy names the author without naming the router, but `:41` remains the authoritative statement and the guidance is read whole.
- Regeneration of the deployed copies. `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` produces no output; no pack prompt contains a `{{...}}` slot (`grep -rc "{{" pack/prompts/` returns `0` for all eight files), so the byte-identity check is meaningful. `cargo test` is fully green (367 + 5 + 3 + 2 + 1 + 1 passing, 0 failures), including the `agents_md_drift` whole-file guard and the `isolation_policy` / `recommendation_rule` / `findings_naming` byte guards, so `AGENTS.md` and `.agents/AGENTS.reference.md` are a fresh render. `git status --short` is clean at `065e511`.
- `pack/prompts/planner.md` currency. It already frames folding a change into the plan as planner work (`pack/prompts/planner.md:7`, `:9`), so this change does not make it stale.
- Pre-existing items outside this change: the two senses of "non-trivial" (a trivial request at `:29` versus a trivial fold at `:31`/`:33`) do not collide at any specific line, because `:29`'s trivial-request definition excludes a new open question and any Roadmap-scope change, which is exactly what would make a fold non-trivial. Line length and prose wrapping were not considered. The branch-2 / branch-3 unevenness held as `Q-69`, the exploration-mode passages, `pack/user-prompts/explore.md`, and `pack/LEDGER.template.md` were treated as out of scope per the review brief.
