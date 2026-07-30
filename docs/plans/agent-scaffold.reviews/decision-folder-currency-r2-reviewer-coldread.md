# Reviewer findings: `decision-folder-currency`, round 2 (cold-read lens)

Commit under review: `3285320` (`git rev-parse HEAD` -> `3285320`, detached). Worktree: `.claude/worktrees/rev90r2-b`. Change set read as `git diff a7c7ce3..3285320 -- pack/prompts/orchestrator.md pack/AGENTS.md`, plus the three regenerated deployed copies.

Lens: start from the changed documents as they now stand and read them the way the agent they instruct would, not from the step brief.

Every `file:line` below was re-read at that line in this worktree before it was written down.

## Summary

Three findings, all `low`. Zero at `medium`, zero at `high`, zero at `critical`.

Answers to the four questions asked:

1. **Yes.** An orchestrator reading `pack/prompts/orchestrator.md` as it now stands lands on the correct rule: it does not author a decided decision's `[[question]]` or `[[step]]` fold, it routes that to the planner, and it keeps its own integration edits and trivial folds. The round-1 defect is genuinely fixed: branch 2 of `:31` no longer commands an act it routes away. I could not construct an over-restricted reading a careful agent would actually land on, because the fix narrowed the object of "rather than editing the plan yourself" to the fold, so it no longer collides with `:25` ("mark it complete") or `:29` ("Fold a trivial request ... in directly"). The under-restricted reading is gone at `:31`: the recording verb is now actor-less and the only act assigned to the reader is "emit the block" and "you route".
2. **No.** I found nothing false in the added clauses. The cross-reference target resolves (`pack/AGENTS.md:71`), `[[question]]`/`[[step]]` are real plan-source entities, and the parenthetical restates `pack/AGENTS.md:41` in substance.
3. **No direct contradiction**, but two omissions where the prompt's clauses no longer carry something their `pack/AGENTS.md` counterparts do (F2), and one wording tension against an unchanged passage in the same guidance file (F1).
4. **One clause is ambiguous** in a way an agent could resolve wrongly, at a checkpoint that is also a compaction-prep flush (F1).

Mechanical checks re-run in this worktree, all pass:

- `cargo test` -> 367 + 5 + 1 + 3 + 1 + 2 passing, 0 failures. Includes `agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render` and `::the_committed_scaffold_matches_a_fresh_render`, and the `isolation_policy` byte guard. The known `checks::tests` worktree-naming flake did not fire.
- `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` -> no output.
- `diff AGENTS.md .agents/AGENTS.reference.md` -> no output.
- `git status --short` -> clean at `3285320` before this file was written.

## F1 (`low`). The narrowed checkpoint gloss at `pack/prompts/orchestrator.md:27` leaves the compaction-prep case with no legal actor: the same document set tells the orchestrator to flush the Open Questions queue to current before a context loss, and tells it not to author the decided entry

Evidence.

`pack/prompts/orchestrator.md:27` names three checkpoints, one of which is a compaction-prep flush, opens with "sync the durable state before moving on", and then narrows the queue duty:

> A checkpoint is a step boundary (a step converged, the next not yet started), a compaction-prep flush, or an escalation. There, update the plan's Open Questions queue and push its open items to the human [...] Here "update the plan's Open Questions queue" means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job and which you route to it rather than author yourself

Note that the gloss defines "update" as "raising and pushing", while the sentence it glosses already said "and push its open items to the human". After the gloss, the "update the queue" verb contributes no plan-file act at all, so the queue side of "sync the durable state" is empty for the orchestrator.

Against that, three unchanged shipped passages still put a queue-side or plan-side flush on the orchestrator at exactly the pre-compaction moment:

- `pack/AGENTS.md:99`: "Before a context loss (for example a compaction): the orchestrator flushes the plan, the ledger, and the plan's Open Questions queue to current, pushes any still-open queue items to the human (the checkpoint queue push above)". Verified at that line; identical at the rendered `AGENTS.md:99`.
- `pack/user-prompts/pause.md:7`, the text the human pastes: "bring the plan and the ledger up to date, commit any pending work so the tree is clean".
- `pack/AGENTS.md:106`: "a fact available only in conversation history is not citable, and reaching for one is a durability bug to fix by flushing that fact into an artifact first".

The gap this leaves. Take the concrete case the checkpoint machinery exists for: at the queue push the human decides an item, and the session is about to compact. The orchestrator may not author the fold (`:27`, `pack/AGENTS.md:71`). It may not park the decision in the ledger either: `pack/AGENTS.md:61` says "Status and decisions are owned by the plan: the ledger references the plan for them rather than restating them", the changed `pack/prompts/orchestrator.md:33` repeats it, and `pack/LEDGER.template.md:13` says "do not copy step statuses or decisions here". So the only legal path is to spawn a planner before compacting, and no passage in either document says so. An agent that resolves the tension the other way either authors the entry itself (the exact misstep `Q-67` exists to prevent) or carries the decision in working context across a compaction (the durability bug `pack/AGENTS.md:106` names).

Why this is not settled elsewhere. This is not the `pack/AGENTS.md:71` main-clause versus trailing-clause contradiction held as `Q-69`: it does not turn on whether an undecided placeholder may be minted, and it survives whichever way `Q-69` rules, because it concerns a decision that is already `decided`. Round 1 did not examine `pack/AGENTS.md:99` or the compaction case (its "checked and not raised" list at `decision-folder-currency-reviewer-coldread.md:45-54` does not mention either).

Why `low` and not `medium`. There is a benign reading of "flushes ... to current" as "commit what is already recorded", which the bullet's own closing "and commits everything" supports, and under that reading there is no conflict at all. The correct behaviour (route to the planner, then flush and commit) is derivable from the rest of the document, and a lost decision is recoverable because the human is present at the gate. It is an ambiguity at a routine moment rather than a rule collision.

Why it is not `low`-and-ignorable. The compaction-prep flush is named explicitly in the changed paragraph, and this project resumes from compaction often enough that the case is not hypothetical.

Suggested direction (a suggestion, not a requirement). One clause at `pack/prompts/orchestrator.md:27` would close it, for example noting that a decision taken at the queue push is routed to the planner before the flush, so nothing decided is carried in working context across the checkpoint. That asserts no new rule, it only names the ordering the existing rules already imply.

## F2 (`low`). Three of the four routing clauses drop the "as above" pointer their `pack/AGENTS.md` counterparts carry, and with it the "then re-enters plan review" obligation on the planner's fold

Evidence. In the guidance, every statement of the rule chains to the request-interrupt path, which is where the review obligation lives:

- `pack/AGENTS.md:39`: "routes to the planner to fold into the plan (revising the Roadmap steps and Success Criteria and resolving any new open questions), then re-enters plan review".
- `pack/AGENTS.md:41`: "the orchestrator routes it to the planner to author, **as on the request-interrupt path above**".
- `pack/AGENTS.md:43`: "its non-trivial fold routed to the planner to author **as above**".
- `pack/AGENTS.md:71`: "which is the planner's job (**routed as above**)".

The prompt's clauses:

- `pack/prompts/orchestrator.md:27` chains correctly: "(the Checkpoints rule in `AGENTS.md`)" reaches `pack/AGENTS.md:71`, which carries "(routed as above)".
- `pack/prompts/orchestrator.md:31`, branch 2: "you route to the planner to author rather than editing the plan yourself". No pointer.
- `pack/prompts/orchestrator.md:33`: "the planner authors that fold when it is non-trivial (authoring a `[[question]]` or a `[[step]]`)". No pointer.
- `pack/AGENTS.md:63`, the fourth edited passage: same clause, no pointer, and unlike its three siblings in the same file it does not say "as above".

Why it matters. The planner is a writer, and `pack/AGENTS.md:89` is explicit that "A writer authors the reviewed product, so its output goes through the review and convergence loop before it is accepted". An orchestrator acting on `:31` alone spawns a planner for the fold and merges its plan edit onto main with no round, because nothing at the point of action says otherwise, while the same orchestrator acting on `:29` two paragraphs earlier would re-enter plan review for a materially identical planner fold. The asymmetry is inside one file and one page of reading.

Why `low`. `pack/AGENTS.md:89` and `:39` both cover it independently, and `pack/prompts/orchestrator.md:3` orders the orchestrator to read `AGENTS.md` first, so a careful agent recovers the obligation. The damage if it lands is an unreviewed but visible and revertible plan diff.

Counter-argument, stated so the triager can weigh it. The step's charter was to name the actor, not to state the routing's downstream consequences, so this can be read as beyond the change. I raise it anyway because the routing itself is new in these three clauses: before this change none of them routed anything, so the incompleteness arrives with the change rather than pre-dating it.

Suggested direction. Either add the same three-word pointer the guidance uses ("as on the request-interrupt path") to `pack/prompts/orchestrator.md:31` and `:33` and `pack/AGENTS.md:63`, or accept it as residual on the ground that `pack/AGENTS.md:89` governs. Both are defensible; naming which one was chosen is the useful outcome.

## F3 (`low`). The folding rule is now hand-copied at seven sites across two files with no guard holding them together, in a project that single-sources exactly this class of duplicated rule

Evidence. The exact parenthetical appears four times in the authored sources:

```
grep -rno 'authoring a `\[\[question\]\]` or a `\[\[step\]\]`' pack/AGENTS.md pack/prompts/orchestrator.md
pack/AGENTS.md:41
pack/AGENTS.md:63
pack/prompts/orchestrator.md:31
pack/prompts/orchestrator.md:33
```

Three further sites state the same rule in other words: `pack/AGENTS.md:43`, `pack/AGENTS.md:71`, and `pack/prompts/orchestrator.md:27` (the last two share the wording "not authoring a decided decision's `[[question]]` or `[[step]]` fold").

Nothing guards these against each other. `src/agents_md_drift.rs:414-455` compares a fresh pack render against each committed deployed copy, filtered by `PROMPT_DEST_PREFIX` (`:125`), so it catches pack-to-deployed staleness only. A future edit to the rule in `pack/AGENTS.md` that misses `pack/prompts/orchestrator.md` (or the reverse) leaves the suite fully green, which is precisely how the four passages this step exists to fix came to disagree with `pack/AGENTS.md:39` in the first place.

The project already treats this class of duplication as a defect to remove rather than to tolerate, with a proven mechanism: `ISOLATION_POLICY_FRAGMENT` (`src/isolation_policy.rs:33`), `RECOMMENDATION_RULE_FRAGMENT`, and `findings_naming::convention_fragment()` are each authored once and substituted into an `AGENTS.md` slot by `build_assets` (`src/main.rs:268-299`), with byte-guard tests. `Q-60`'s standing directive is on the record at `docs/plans/agent-scaffold.plan.toml:1658`: "The human's general directive: always restate and avoid pointers where possible, instead rendering duplicates from a single source to prevent drift."

What makes this more than a note. The mechanism cannot reach the prompt as things stand: `pack/pack.toml:104-107` declares the `prompts/orchestrator.md` asset with no `render = true`, unlike the `AGENTS.md` asset at `pack/pack.toml:98-102`, and `manifest::render` (`src/manifest.rs:332`) is applied to rendered assets only, as its own doc comment states at `src/manifest.rs:329-331`: "Only rendered (`render = true`) assets pass through here; verbatim assets keep their exact bytes." So a `{{...}}` slot placed in the prompt today would ship literally into `.agents/prompts/orchestrator.md`. Anyone who reaches for the established pattern here has to flip that asset to rendered first, which is a real (if small) change and is worth knowing before it is attempted.

Why `low`. Nothing is wrong in the shipped text today, all seven sites agree, and the risk is future drift rather than present misbehaviour. This is not a challenge to `Q-67`'s decision to restate the actor at each point of use, which I take as settled; it is about the restatements being hand-maintained rather than rendered.

Suggested direction. Record it as a known drift risk owned by whichever step next touches the folding rule, or, if it is judged worth the change, fold "make `prompts/orchestrator.md` a rendered asset" into the same step that single-sources the fragment. Do not spawn work solely for this.

## Checked and deliberately not raised

Recorded so the coverage is visible and so the triager can see where I stopped.

- **The branch-2 fix itself.** The re-voicing satisfies both constraints the round-1 triager set (`decision-folder-currency-triage.md:103-106`): branch 2 contains no internal semicolon, so the three-branch structure still parses, and branch 3 is byte-untouched (`git diff aff5432..3285320 -- pack/prompts/orchestrator.md` shows one changed line, and the only difference inside it is branch 2). The attributive form "whose non-trivial fold" now matches `pack/AGENTS.md:43`'s "its non-trivial fold", so the cross-file divergence the round-1 finding named is closed.
- **The trivial fold has no stated actor and no stated definition.** The complement of "authoring a `[[question]]` or a `[[step]]`" is nowhere spelled out, and the one concrete instance (amending an already-queued item to `decided` and filling `folded_into`) is not among the four direct-on-main edits the generated fragment names. Not raised: the round-1 triager considered exactly this case and ruled the trivial amendment legitimately the orchestrator's (`decision-folder-currency-triage.md:63`, `:71`), and whether the fragment's four-item list is exhaustive or illustrative is `Q-69` premise 1, which is explicitly out of scope.
- **The branch-2 versus branch-3 unevenness**, branch 3 minting a `[[question]]`, the exploration-mode passages, `pack/user-prompts/explore.md`, and `pack/LEDGER.template.md`: out of scope per the review brief.
- **The two senses of "non-trivial"** (a trivial request at `pack/prompts/orchestrator.md:29`, a trivial fold at `:31`/`:33`). I looked for a line where they actually collide and did not find one: `:29`'s trivial request excludes a new open question and any Roadmap-scope change, which is exactly what would make a fold non-trivial, so the two definitions agree on every case.
- **The `(the Checkpoints rule in `AGENTS.md`)` pointer** against the two similarly named sections `AGENTS.md:71` and `AGENTS.md:97`. Round 1 settled it; `AGENTS.md:106` uses the same short form for `:71`, so the convention is established.
- **Pronoun collision at `pack/prompts/orchestrator.md:33`**: "fold into it, and the planner authors that fold when it is non-trivial" uses "it" for the plan and then for the fold eight words apart. The parenthetical disambiguates immediately and the alternative reading is nonsense, so no reasonable agent resolves it wrongly. `pack/AGENTS.md:63` does not have the collision because it says "the plan's steps".
- **`pack/AGENTS.md:61`'s "The implementer keeps the status current"** against `pack/prompts/orchestrator.md:23`/`:25` telling the orchestrator to "mark the step complete". A real tension, but it pre-dates this change, is unrelated to decision folding, and the isolation-policy fragment licenses the orchestrator's status flip either way.
- **Currency of the other role prompts.** `pack/prompts/planner.md:5`, `:7`, `:9` and `pack/prompts/open-questions-gate.md:7` already frame authoring `[[question]]` entries and folding changes into the plan as planner work, so the change makes neither stale. `pack/prompts/implementer.md`, `reviewer.md`, and `triager.md` say nothing about folding: `grep -n "fold" pack/prompts/implementer.md pack/prompts/reviewer.md pack/prompts/triager.md` returns one hit, `implementer.md:5`, and it is the substring inside "agent-scaffold render".
- **Line length and prose wrapping**: never findings here, not considered.
