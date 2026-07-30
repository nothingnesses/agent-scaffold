# Triage verdicts: `decision-folder-currency`, round 2

Artifact under review: commit `3285320` (detached), the round-1 fix on top of the step-90 implementation `aff5432`. Findings adjudicated: `decision-folder-currency-r2-reviewer-verification.md` (1 finding) and `decision-folder-currency-r2-reviewer-coldread.md` (3 findings). Worktree `.claude/worktrees/triage90r2`, `git status --porcelain` clean except the two findings files and this one.

## Summary of verdicts

| Finding | Verdict | Final severity | In or out of step scope | One-line ground |
| --- | --- | --- | --- | --- |
| verification F1 | VALID BUT ACCEPT RESIDUAL | `low` | OUT (orchestrator's own ledger, not the reviewed product) | The count reproduces exactly (46 to 42, ledger says 45 to 43), but the error is in a transient record the orchestrator owns directly, not in the change. |
| coldread F1 | DISMISSED | would be `low` | OUT (all three legs byte-unchanged from before the step) | Two load-bearing premises fail: the gloss does leave the orchestrator a plan-file act ("raising"), and the legal actor is named (the planner) at four sites. |
| coldread F2 | VALID BUT ACCEPT RESIDUAL | `low` | Passages IN, the requested content OUT | The re-enter-review obligation is not lost (`pack/AGENTS.md:89`, `pack/prompts/orchestrator.md:29`, and the prompt's own review loop all carry it); the brief scoped these edits to the actor clause. |
| coldread F3 | VALID BUT ACCEPT RESIDUAL | `low` | OUT (a manifest and code change, not a four-passage prose edit) | Every citation reproduces including the load-bearing `pack.toml` one, but nothing is wrong today and the remedy is a rendered-asset change outside this step. |

**No fix is required. Advice to the orchestrator: score this round CLEAN.** Reasoning in the closing section.

## Reproduction discipline

Every `file:line` cited in either findings file was re-read at that line in this worktree. **All of them reproduce.** This is the second consecutive round with no misnumbered citation, after several rounds that had them. The mechanical claims were re-run rather than taken on trust:

- `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` -> no output.
- `diff AGENTS.md .agents/AGENTS.reference.md` -> no output.
- `cargo test` -> 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed. The known `checks::tests` worktree-naming flake did not fire on this run.
- The step's total change set `a7c7ce3..3285320` is five files: the two pack sources and the three deployed copies, no incidental churn.

## verification F1 (`low`). Ledger records the fix as "45 words to 43" where it is 46 to 42

**Verdict: VALID BUT ACCEPT RESIDUAL. Final severity `low` (confirmed). OUT of this step's scope.**

Reproduced exactly. Taking line 31 at `aff5432` and at `3285320`, splitting the tail after `pick the mode it needs: ` on `; `, and counting whitespace-separated tokens in branch 2:

- Old branch 2: **46** tokens (`for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision, routing its fold to the planner to author when that fold is non-trivial (authoring a `[[question]]` or a `[[step]]`) rather than editing the plan yourself`).
- New branch 2: **42** tokens.

`docs/plans/agent-scaffold.ledger.md:413` says "Re-voice branch 2 ONLY, 45 words to 43". Both numbers are wrong, and in opposite directions (one under by 1, one over by 1), which is the signature of a hand estimate rather than a counting convention. `wc -w` gives the same 46/42. I tried the reviewer's hyphen-splitting variant (48/44) and dropping the two code-span tokens (44/40); neither reproduces 45/43 either.

The same reconstruction the verification reviewer ran also holds here, and I re-ran it: substituting the old branch-2 string for the new one in the old line 31 yields the new line 31 under full string equality, branch 1 and branch 3 are byte-identical across the change, the prefix is identical, and the branch count is 3 on both sides. So the authorised text landed byte-exact and the number is purely descriptive.

**Why OUT of scope.** The step's product is four passages plus three regenerated files. The ledger is the orchestrator's own review-loop record: `pack/AGENTS.md:61` and `:63` define it as transient round-level state, separate from the plan, deleted when the task closes, and the generated isolation-policy fragment (`src/isolation_policy.rs:33`) names "recording a round record" as one of the orchestrator's own direct-on-main integration edits. It is not reviewed product content and no writer authored it.

**Why accept residual rather than fix-required.** The number is redundant with the sentence it sits in: the same sentence quotes the authorised string verbatim, so any reader who cares can count. Impact if left unfixed is an unbacked number in a file that will be deleted at task close. Ruling this fix-required would reset the streak over a two-digit slip in a document outside the reviewed product, which is not what the convergence rule is measuring.

**Practical note, not a fix requirement.** The orchestrator owns the ledger directly and may correct "45 words to 43" to "46 words to 42" in place as ordinary bookkeeping when it writes the round-2 record. That correction is an integration edit, not a fix pass, and does not make this round `new_valid`.

## coldread F1 (`low`). Alleged: the narrowed checkpoint gloss leaves the compaction-prep case with no legal actor

**Verdict: DISMISSED. Severity if it had been valid: `low`. OUT of this step's scope on both counts (pre-existing, and the remedy is new prose the brief forbids).**

The citations all reproduce:

- `pack/AGENTS.md:99` reads exactly as quoted, including "flushes the plan, the ledger, and the plan's Open Questions queue to current".
- `pack/AGENTS.md:61` reads exactly as quoted.
- `pack/LEDGER.template.md:13` reads exactly as quoted, including "do not copy step statuses or decisions here". (Note this is a different line from the `:3` the step brief cites for a different sentence; both are correct.)
- `pack/user-prompts/pause.md:7` and `pack/AGENTS.md:106` reproduce.
- Round 1's cold-read "checked and not raised" list does sit at `decision-folder-currency-reviewer-coldread.md:45-54` and does not mention `:99` or the compaction case.

So the finding is carefully sourced. It is dismissed on the inference, on three independent grounds.

**Ground 1: the premise that the gloss leaves no plan-file act is false.** The finding argues that because the glossed sentence already said "and push its open items to the human", defining "update" as "raising and pushing" makes the verb contribute nothing. That conflates the two verbs in the gloss. *Raising* an item is putting it into the plan's queue, which is a plan-file act; *pushing* is presenting it to the human. The gloss removes the licence to author a decided decision's fold and leaves raising untouched. The queue side of "sync the durable state" is therefore not empty.

**Ground 2: the premise that there is no legal actor is false.** The legal actor is the planner, and the document set names it at the point of use in four places: `pack/prompts/orchestrator.md:27`, `:31`, `:33` and `pack/AGENTS.md:41`, `:43`, `:63`, `:71`. The finding's real complaint is narrower: that no passage states the *ordering* (route the fold to the planner before the compaction flush). That is an unstated implication, not a missing actor.

**Ground 3: the dilemma has at least two exits the finding does not consider, so neither horn is forced.** The finding claims the orchestrator must either author the entry itself or carry the decision across the compaction in working context. Both are avoidable:

- The round-1 triager already ruled, at `decision-folder-currency-triage.md:63`, that where the human's answer lands on an already-queued item, "flipping it to `decided` and filling `folded_into`/`receipt` is a field amendment", and at `:71` that "the trivial case stays implicitly the orchestrator's". So the orchestrator can durably record the human's answer on the queue item before compacting, and route only the `[[question]]`/`[[step]]` authoring. That is settled, not novel.
- `pack/AGENTS.md:106` names a second durable carrier for the human's choice: "when instrumentation is on, its `type: "decision"` round-log record carrying the human's `chosen`". Appending a round record is on the orchestrator's own closed direct-on-main list.
- `pack/LEDGER.template.md:13` invites exactly "the non-plan-derivable transient in-flight state that would otherwise be lost on a compaction". A note that a planner fold is owed is transient in-flight state, not a copied decision, so recording it is licensed rather than banned.

**Ground 4, decisive on scope: every leg of the alleged tension is byte-unchanged from before this step.** I checked all three at `a7c7ce3`, the commit before the step-90 implementation:

- `pack/AGENTS.md:71` already carried the full gloss ("Here 'updates this queue' means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job (routed as above)"), added by step 89.
- `pack/AGENTS.md:99` is identical, word for word.
- `pack/AGENTS.md:61`'s "Status and decisions are owned by the plan" sentence is identical.

Step 90 did not create this configuration. It copied the already-shipped `:71` gloss into the prompt, which is precisely what the brief required at its line 19: "give it the SAME qualifier `pack/AGENTS.md:71` already carries [...] match the guidance's existing clause, do not invent a different rule." If the tension is real, it is step 89's in the guidance, and it applies to `pack/AGENTS.md` whether or not step 90 ever ran. What the narrowing removed was a *licence to author the fold* that should never have been in the prompt; calling the removal of a wrong licence a new hole inverts the step's purpose.

**Why the suggested fix would be actively harmful here.** The finding proposes adding a clause about routing before the flush to `pack/prompts/orchestrator.md:27`. The brief's line 28 forbids the checkpoint paragraph from carrying anything beyond the main clause of `pack/AGENTS.md:71`, specifically to avoid enlarging what the `Q-69` design pass has to fix. Adding an ordering rule that appears in neither the guidance nor the prompt would also put the prompt ahead of its own source, re-creating the guidance/prompt divergence this step exists to close, and it is exactly the author-new-prose fix class that this task's own record (`docs/plans/agent-scaffold.ledger.md:425`) identifies as reliably manufacturing the next round's finding.

## coldread F2 (`low`). Three routing clauses drop the "as above" pointer their `pack/AGENTS.md` counterparts carry

**Verdict: VALID BUT ACCEPT RESIDUAL. Final severity `low` (confirmed). The three passages are IN scope; the content the finding wants added is OUT of scope.**

Every citation reproduces, and the asymmetry is real as described:

- `pack/AGENTS.md:39` carries "then re-enters plan review". Confirmed.
- `pack/AGENTS.md:41` carries "as on the request-interrupt path above". Confirmed.
- `pack/AGENTS.md:43` carries "as above". Confirmed.
- `pack/AGENTS.md:71` carries "(routed as above)". Confirmed.
- `pack/prompts/orchestrator.md:27` chains via "(the Checkpoints rule in `AGENTS.md`)" to `:71`, which carries the pointer. Confirmed.
- `pack/prompts/orchestrator.md:31` branch 2, `:33`, and `pack/AGENTS.md:63` carry no pointer. Confirmed, and `:63` is indeed the odd one out among its three same-file siblings.

**Why it does not change what an orchestrator would do**, which is the question I was asked to answer. The review obligation is carried independently, and not by a single fallback but by four:

1. `pack/AGENTS.md:89`: "A writer authors the reviewed product, so its output goes through the review and convergence loop before it is accepted." The planner is a writer. This is unconditional and applies to any planner output including a fold.
2. `pack/prompts/orchestrator.md:29`, two paragraphs above `:31`, states the obligation in the prompt itself: "route anything non-trivial to the planner to fold into the plan (...), then re-enter plan review."
3. The prompt's own review loop is general, not per-path: `:5` ("Drive the phases in order [...] spawn a fresh, isolated agent for the role"), `:11` ("merge each agent's output onto main yourself [...] a review round runs as"), and `:17-21` (the counting procedure applied to every round). Nothing in the prompt frames review as an opt-in attached to particular routing clauses.
4. `pack/AGENTS.md:43`, the Socratic passage the prompt's `:31` mirrors, chains via "as above" to `:39`'s "then re-enters plan review", so the guidance a `:3`-compliant orchestrator has already read closes the chain.

The finding's harm story is an orchestrator that "merges its plan edit onto main with no round, because nothing at the point of action says otherwise". For that to land, the orchestrator has to have skipped its own general review loop, which it is running anyway to produce the round record that would document the merge.

**Why the content is out of scope.** The brief's line 27 requires each passage to "name the actor at its point of use". Line 32 then bounds that: "Prefer a short clause over a restatement: `pack/AGENTS.md:41` (the human-input contract) stays the authoritative statement of the rule, and these four points reinforce it." Line 29 is explicit for the two ledger copies: "Only the actor clause is in scope." Adding the routing's downstream consequence at three sites is a restatement beyond the actor, so it is scope expansion, which the brief's own principle grounding names ("No silent scope expansion", applied in both directions).

I weighed the reviewer's counter-argument that "the routing itself is new in these three clauses, so the incompleteness arrives with the change". It is true in the narrow sense and it is why this is accept-residual rather than dismissed. It does not carry the day, because the authoritative statement the brief designates (`pack/AGENTS.md:41`) already carries the pointer and was never in scope to change, and because the fix is a three-site hand-authored prose addition in a task whose own record says that class of pass re-seeds.

**The naming the reviewer asked for.** The choice is recorded here explicitly, so a later round does not re-raise it: the pointer is deliberately NOT restated at `pack/prompts/orchestrator.md:31`, `:33`, or `pack/AGENTS.md:63`, on the ground that `pack/AGENTS.md:89` governs every writer's output unconditionally and `pack/prompts/orchestrator.md:29` states the obligation in the same document.

## coldread F3 (`low`). The folding rule is hand-copied at seven sites with no guard

**Verdict: VALID BUT ACCEPT RESIDUAL. Final severity `low` (confirmed). OUT of this step's scope.**

This is the best-evidenced of the four and every part of it reproduces, including the load-bearing technical claim I was asked to check directly.

- The exact parenthetical appears at four authored sites: `pack/AGENTS.md:41`, `:63`, `pack/prompts/orchestrator.md:31`, `:33`. Re-ran the grep; four hits, exactly as listed.
- Three further sites state the rule in other words: `pack/AGENTS.md:43`, `:71`, `pack/prompts/orchestrator.md:27`. Confirmed by reading each. Seven total is right.
- **`pack/pack.toml:104-107` declares `prompts/orchestrator.md` with `source`, `dest`, `ownership = "reference"` and NO `render = true`, while the `AGENTS.md` asset at `pack/pack.toml:98-102` does carry `render = true`. Confirmed by reading both blocks.** The line numbers are exact.
- `src/manifest.rs:332` is `pub fn render(`, and its doc comment at `:329-331` reads "Only rendered (`render = true`) assets pass through here; verbatim assets keep their exact bytes." Confirmed at those lines.
- I verified the gating in the code rather than only in the doc comment: `src/manifest.rs:533` is `let contents = if spec.render { render(&raw, &vars) } else { raw };`. So a `{{...}}` slot placed in `pack/prompts/orchestrator.md` today would indeed ship literally into `.agents/prompts/orchestrator.md`. **The finding's central technical claim is correct.**
- The single-source precedent reproduces: `ISOLATION_POLICY_FRAGMENT` at `src/isolation_policy.rs:33`, substituted with `RECOMMENDATION_RULE_FRAGMENT` and `findings_naming::convention_fragment()` in `build_assets` at `src/main.rs:268-299` (the three `builtin.insert` calls land at `:277`, `:289`, `:299`).
- `src/agents_md_drift.rs:125` is `const PROMPT_DEST_PREFIX: &str = ".agents/prompts/";`, and `the_committed_role_prompts_match_a_fresh_render` begins at `:415`. It compares a fresh render against the committed copy, so it catches pack-to-deployed staleness only, not source-to-source disagreement. Correct as characterised.
- `docs/plans/agent-scaffold.plan.toml:1658` is `Q-60`'s `ask`, and the quoted human directive ("always restate and avoid pointers where possible, instead rendering duplicates from a single source to prevent drift") is on that line. Confirmed by grep, which returns only `1658`.

**Why OUT of scope.** The remedy is a manifest change (flip an asset to rendered), a new source fragment, a substitution in `build_assets`, a byte guard, and a rewrite of shipped prose at seven sites. This step is four hand-authored prose clauses that touch no source, no template, and no manifest. Nothing in it could have been done differently to avoid the finding, short of doing that other work.

**Why accept residual rather than fix-required.** All seven sites agree today; I checked each and every one carries either the "non-trivial (authoring a `[[question]]` or a `[[step]]`)" qualifier or the equivalent "a decided decision's `[[question]]` or `[[step]]` fold" object. There is no unqualified "the planner authors that fold" anywhere in `pack/`, `.agents/`, or the root `AGENTS.md`. The risk is future drift, not present misbehaviour, which is `low` correctly.

**On whether this should become a new roadmap step: not yet, and I would advise against minting one now.** Two reasons, and the second is the stronger.

1. The seven sites are not simple duplicates. The brief's line 15 is explicit that the two ledger-paragraph copies "are counterparts, not duplicates", differing in their tails on purpose, and the checkpoint and Socratic passages are deliberately voiced for their contexts. A single rendered fragment would have to flatten differences that were chosen, so single-sourcing here is a design question, not a mechanical extraction.
2. `Q-69` is going to rewrite part of this text. Its design pass reaches `pack/AGENTS.md:45`, `:71`'s trailing clause, and branch 3 of `pack/prompts/orchestrator.md:31`, which is one of the seven sites and sits inside another. Single-sourcing before that pass lands would be work done against text that is about to move.

What should happen instead is that the two facts the finding established get recorded so a future step does not rediscover them: that the folding rule stands at seven hand-maintained sites with no cross-source guard, and that `prompts/orchestrator.md` is a **verbatim** asset (`pack/pack.toml:104-107`), so any `{{...}}` approach must flip it to `render = true` first (`src/manifest.rs:533`). The natural owner is whichever step next touches the folding rule, which on current sequencing is the `Q-69` design pass and the step it produces. If the orchestrator prefers a durable home over a ledger note, the honest form is a candidate line against `Q-60`'s single-sourcing directive rather than a scheduled step, since the work is gated on `Q-69`.

## Overall read: advice to the orchestrator

**Score this round CLEAN.**

- Zero findings require a change to the reviewed product. One is dismissed on the merits; three are accept-residual.
- The one finding that is fix-shaped (verification F1) is a two-digit correction in the orchestrator's own transient ledger, which the orchestrator edits directly as an integration edit. Correcting it is bookkeeping, not a fix pass, and does not make the round `new_valid`.
- No finding rises above `low`. I looked for a `medium` or higher in each and did not find one; I also re-checked whether any should be re-severitised **upward** and none should. coldread F1 is the only one whose framing suggested higher stakes ("no legal actor"), and it is the one that fails on its premises.
- No dismissal at `high` or `critical`, so no backstop re-check is owed under `pack/prompts/orchestrator.md:20`.
- The step is `low_risk`, so one clean round converges it.

I want to be clear that this is not a convenience ruling. The round-1 fix is byte-exact, branch 3 is untouched, the three-branch structure still parses, the semicolon trap was avoided, the deployed parity holds under both hand `diff` and the step-92 drift guard, and the suite is green with the known flake not firing. The two lenses between them examined the fix mechanically, read the documents cold, and went looking in the adjacent guidance; what they came back with is one wrong number in a scratch file and three observations about text this step was told not to touch. That is what a converged low-risk prose step looks like.

The residuals to carry forward, so they are not lost and not re-raised:

1. `docs/plans/agent-scaffold.ledger.md:413` says "45 words to 43"; the true counts are 46 and 42.
2. The "as on the request-interrupt path" pointer is deliberately not restated at `pack/prompts/orchestrator.md:31`, `:33`, or `pack/AGENTS.md:63`; `pack/AGENTS.md:89` governs.
3. The folding rule stands at seven hand-maintained sites with no cross-source guard, and `prompts/orchestrator.md` is a verbatim asset that would have to become rendered before the established fragment mechanism could reach it. Gated on `Q-69`.
4. Carried from round 1, already settled: the brief's stale drift-guard claim at `decision-folder-currency.md:30` and `:38`.
