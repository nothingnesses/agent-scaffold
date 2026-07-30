# Review: `decision-folder-currency` (commit `065e511`), lens: fidelity to the brief

Reviewer worktree: `.claude/worktrees/rev90-a`, detached at `065e511`. Brief: `docs/plans/agent-scaffold.steps/decision-folder-currency.md`.

Verdict: the change does what the brief specifies, and nothing else. One `low` finding, about the currency of the brief's own enforcement claim rather than about the edit. No `critical`, no `high`, no `medium` findings; I looked for each of those and found none.

## Acceptance check (brief line 30)

```
$ diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md
$ echo $?
0
```

No output. The deployed prompt is byte-identical to its pack source.

`cargo test`: 367 lib tests pass, all integration binaries pass, zero failures, including `agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render` (`src/agents_md_drift.rs:375`, which asserts both the root `AGENTS.md` and `.agents/AGENTS.reference.md`) and `agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render` (`src/agents_md_drift.rs:415`). The known `checks::tests` flake did not fire on either of my two runs.

## Findings

### F1 (`low`): the brief's claim that the deployed orchestrator prompt is unguarded is stale at this tip

`docs/plans/agent-scaffold.steps/decision-folder-currency.md:38` states that `.agents/prompts/orchestrator.md` "has NO whole-file drift-guard test, so nothing fails if the regeneration is skipped and the staleness would be silent", and line 30 calls the hand `diff` "the one part of the currency work no test enforces". At `065e511` that is no longer true: `src/agents_md_drift.rs:415`, `the_committed_role_prompts_match_a_fresh_render`, renders the pack and compares every asset whose destination starts with `PROMPT_DEST_PREFIX` (`src/agents_md_drift.rs:125`, `.agents/prompts/`) against the committed bytes, and asserts the filtered set is non-empty so it cannot pass vacuously. `prompt-drift-guard` (order 92) landed before this step ran, so the gap the brief describes as open was already closed.

Reproduce:

```
$ grep -n "PROMPT_DEST_PREFIX" src/agents_md_drift.rs
$ cargo test the_committed_role_prompts_match_a_fresh_render
```

Impact if left unfixed: low, and it is not a defect in the change. The regeneration was done and both the hand check and the guard pass, so nothing is wrong in the tree. What is wrong is a durable sidecar telling a later reader that a guard does not exist when it does, which could justify skipping a regeneration check on a future prompt edit. The fix belongs to whoever owns the plan, not to the implementer, who is forbidden from editing the step file.

## Checks that passed, with evidence

1. **Exactly four passages, and only those four.** `git diff -U0 065e511~1 065e511` touches `pack/prompts/orchestrator.md` lines 27, 31, 33 and `pack/AGENTS.md` line 63, one line each, which are precisely the four the brief names at its lines 12 to 15. The two actor-less ones (`pack/prompts/orchestrator.md:33`, `pack/AGENTS.md:63`) gained an actor; the checkpoint paragraph (`:27`) was narrowed with the `pack/AGENTS.md:71` qualifier; the Socratic paragraph (`:31`) reassigned the non-trivial fold to the planner. `git diff --name-only 065e511~1 065e511` lists five files and no others: the two pack sources plus the three deployed copies.

2. **The scope qualifier is on every added actor clause.** `grep -rno "the planner authors that fold[^)]\{0,60\}" pack/ AGENTS.md .agents/` returns five hits (`pack/prompts/orchestrator.md:33`, `pack/AGENTS.md:63`, and the three deployed copies at the same lines), every one of them continuing "when it is non-trivial (authoring a `[[question]]` or a `[[step]]`)". There is no unqualified instance anywhere in the tree. The other two added clauses carry the same qualifier: `pack/prompts/orchestrator.md:27` says "not authoring a decided decision's `[[question]]` or `[[step]]` fold", and `:31` says "when that fold is non-trivial (authoring a `[[question]]` or a `[[step]]`)". A trivial fold therefore still sits with the orchestrator in all four passages.

3. **Nothing restated from the generated fragment.** `git diff 065e511~1 065e511 | grep '^+' | grep -iE "status flip|increment declaration|round record|resume anchor|isolation.policy"` returns nothing. The checkpoint paragraph points at "the Checkpoints rule in `AGENTS.md`" instead, and that target exists and is correctly named: `pack/AGENTS.md:71` begins "Checkpoints (the human-decision queue and progress)." The four direct-on-main edits are not enumerated anywhere in the added text.

4. **Branch 3 of the Socratic sentence is byte-unchanged.** The word diff of `pack/prompts/orchestrator.md:31` shows a single replacement, "decision;" to "decision, routing its fold to the planner to author when that fold is non-trivial (authoring a `[[question]]` or a `[[step]]`) rather than editing the plan yourself;", entirely inside branch 2. Grepping the full branch-3 string ("for one whose design space is not yet decidable ... the design-space exploration mode in `AGENTS.md`.") against `git show 065e511~1:pack/prompts/orchestrator.md` and against the working file both return one match, so the branch survived intact. `Q-69` gains nothing to fix here.

5. **Out-of-scope files untouched.** `git diff --name-only 065e511~1 065e511 | grep -E 'explore|LEDGER'` returns nothing, so `pack/user-prompts/explore.md` and `pack/LEDGER.template.md` are untouched. `pack/AGENTS.md` changed at line 63 only, so the design-space-exploration paragraph at `pack/AGENTS.md:45` is untouched.

6. **The guidance copy kept its tail, and only the actor clause is shared.** `pack/AGENTS.md:63` still ends "and a folded decision reopens only by evidence that beats its recorded reasoning", with the new clause inserted before it. The prompt copy at `pack/prompts/orchestrator.md:33` has no such tail, and the two share exactly the clause "the planner authors that fold when it is non-trivial (authoring a `[[question]]` or a `[[step]]`)" and nothing more. The rest of the two sentences was not harmonised.

7. **Regeneration.** All three deployed files are in the same commit and changed at the mirrored lines: `.agents/prompts/orchestrator.md` at 27, 31, 33, and both `AGENTS.md` and `.agents/AGENTS.reference.md` at 63. The hand `diff` is empty and the drift guards pass.

8. **Wording matched the shipped guidance rather than inventing.** Two added phrases looked at first like widenings and are not. "rather than editing the plan yourself" in the Socratic branch is the second-person form of `pack/AGENTS.md:41`'s "rather than editing the plan directly", which sits inside the same "when that fold is non-trivial" condition there too, so it does not read as a blanket ban on the orchestrator's own direct-on-main plan edits. "which you route to it rather than author yourself" in the checkpoint paragraph is the expansion of `pack/AGENTS.md:71`'s "(routed as above)". Neither invents a rule.

## Ruling on the reported brief conflict

The implementer's reading is correct, and its resolution is the right one.

The conflict is real. `pack/AGENTS.md:71`'s only pointer at the generated fragment is the clause after the colon, "the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits, which author no reviewed product content, so a `[[question]]` or `[[step]]` (reviewed product content) is not among them". Brief line 34 quotes exactly that clause as the "pointing" to copy; brief line 28 names exactly that clause as the trailing rationale not to reproduce. They cannot both be followed literally.

Line 28 wins on two grounds. It is the instruction that carries a substantive reason (the clause drops the "a decided decision's ... fold" qualifier and so is one side of the live `Q-69` contradiction, and reproducing it would seat a second instance two paragraphs above the branch-3 sentence it contradicts). And line 34's own closing sentence sanctions the substitute the implementer chose: "The prompt may reference the rule in `AGENTS.md` rather than reproduce the fragment's contents, and it must not enumerate the four direct-on-main edits itself." So the chosen pointer, "(the Checkpoints rule in `AGENTS.md`)", satisfies line 34's actual prohibition (nothing restated, nothing enumerated) while honouring line 28. Line 34's "copy the pointing" is best read as advice on how to get the connection without copying the list, not as a requirement to reproduce that specific sentence.

The result does not plant a second instance of the `Q-69` contradiction. The new sentence at `pack/prompts/orchestrator.md:27` keeps the qualifier that the `Q-69` clause drops: it excludes only "authoring a decided decision's `[[question]]` or `[[step]]` fold", and says nothing about a `[[question]]` in general, so it makes no claim about the `exploring` item that branch 3 of `:31` tells the orchestrator to record. The two paragraphs are consistent with each other as they now stand.

One residual I considered and am not raising: the new cross-reference sends the reader to `pack/AGENTS.md:71`, whose trailing clause is the `Q-69` contradiction, so a reader who follows the pointer still meets it. That is a pointer to a pre-existing contradiction, not a new instance of it, and brief line 34 explicitly permits referencing the rule in `AGENTS.md`. It does not enlarge the `Q-69` design pass.

## Not raised, per the settled list

Line length and prose wrapping; the exploration-mode passages and `pack/LEDGER.template.md` being left unfixed; the Socratic sentence reading unevenly across branches 2 and 3.
