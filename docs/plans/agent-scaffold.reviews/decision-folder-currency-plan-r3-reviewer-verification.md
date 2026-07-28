# Plan-review findings, step 90 `decision-folder-currency`, round 3, fix-verification and regression lens

Reviewer worktree: `.claude/worktrees/rev3-verify`, detached at `981d9f5`. Artifact: `e66d11a..981d9f5`; the round-2 fix commit alone is `09ef94e..981d9f5`. Every citation below was re-read in this worktree immediately before it was written, and every command below was run in this worktree.

Outcome: all six round-2 findings (`NEW-1`, `NEW-2`, `NEW-3`, `NEW-4`, `R2-1`, `R2-2`) are CLOSED. No regressions. No scope violations: `R2-3` is untouched and the round-1 `T-4`, `T-5`, `T-7` verdicts were not reopened. Mechanical checks pass with the expected counts.

Two NEW findings, both `low`, both in text the round-2 fix commit authored. Neither reopens a closed finding and neither changes the direction of any argument in `Q-69`. Zero findings at `medium`, `high`, or `critical`.

---

## Mechanical checks (run in this worktree)

```
$ cargo run --quiet -- render --check docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.plan.toml: up to date

$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl --workflow
docs/metrics/workflow.jsonl: 211 records, valid
docs/plans/agent-scaffold.plan.toml: 92 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

92 steps, 69 questions, 211 records, as expected. The generated view is current, so every correction below is present in `docs/plans/agent-scaffold.md` as well as in the source.

`Q-69` state. The whole entry is three fields:

```
$ awk '/^id = "Q-69"/,/^\[\[principle\]\]/' docs/plans/agent-scaffold.plan.toml | grep -n '^id\|^status\|^ask\|^folded_into\|^receipt\|^\[\['
1:id = "Q-69"
2:status = "open"
3:ask = """who records an `exploring` Open-Questions item, ...
25:[[principle]]
```

`status = "open"`, no `receipt`, no `folded_into`. Correct for an undecided item, and consistent with the `ask`'s closing "No decision receipt is owed while this item is `open`".

`docs/plans/agent-scaffold.questions/Q-69.md` is empty (0 bytes). That is the project's convention, not an omission: all 69 question sidecars are empty (`find docs/plans/agent-scaffold.questions -name '*.md' -size +0 | wc -l` -> `0`, against 69 total), the `ask` living in the TOML.

---

## NEW-1: CLOSED

Underlying facts re-verified independently, not taken from the triage.

```
$ for c in 557fa46 4f48283 cca1099 b6ba317 8d12264; do git log -1 --format="%h parents=[%p] %ad %s" --date=iso $c; done
557fa46 parents=[e8f458c] 2026-07-26 17:06:11 +0100 docs: require reviewer findings to carry reproducible evidence (Q-66)
4f48283 parents=[dc9686a] 2026-07-26 22:05:29 +0100 docs: name the planner as folder of non-trivial decided decisions (Q-67)
cca1099 parents=[44f848a] 2026-07-26 16:47:15 +0100 docs: apply Q-66/Q-67 plan-review round 1 fixes (F1 F2 F3 F-fid)
b6ba317 parents=[9e12585] 2026-07-26 22:37:46 +0100 docs: capture Q-68 exploring backlog item for structured-first ledger
8d12264 parents=[b6ba317] 2026-07-26 22:40:29 +0100 docs: record Q-68 (structured-first ledger) backlog capture in ledger queue

$ grep -c "Q-68" docs/metrics/workflow.jsonl
0
```

All three cited commits are single-parent, so the commit-shape inference carries no information, exactly as the corrected text now says. The ledger records all three as fast-forward integrations of reviewed branches; the three strings reproduce verbatim in `docs/plans/agent-scaffold.ledger.md`: "STEP 88 COMPLETE: ff-merged `557fa46`", "STEP 89 COMPLETE (ff `4f48283`)", "PLAN FOLD MERGED (ff `cca1099`)".

The ledger's attribution reproduces verbatim, located by content: `grep -n 'NEW BACKLOG' docs/plans/agent-scaffold.ledger.md` -> line 355, containing "NEW BACKLOG (captured 2026-07-26 by a planner, per Q-67)". `Q-67` landed at 22:05:29 and `b6ba317` at 22:37:46, so the "about half an hour" and "written three minutes later" claims in the corrected text are both accurate (32 minutes; 2 minutes 43 seconds).

`b6ba317` is an ancestor of `HEAD` and of `main`, and `git show b6ba317 -- docs/plans/agent-scaffold.plan.toml` shows it adding `[[question]] id = "Q-68"` with `status = "exploring"`. So the retained verifiable half is true.

Fix verification against the triage's five requirements:

1. The "plain single-parent commit" framing is gone from the sidecar entirely (`docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md:5` no longer contains the phrase). In `plan.toml:1735` the phrase survives only inside an explicit rebuttal, "Nothing about the commit's single-parent shape bears on the question either way", followed by the three-commit demonstration. That is a deviation from the triage's literal "drop it entirely from both places", but it satisfies the requirement's purpose: the datum is no longer offered as evidence, it is neutralised, and the neutralisation is itself verifiable (above). Not a finding.
2. The "no planner branch" assertion is gone: `grep -rn "no planner branch" docs/plans/` returns nothing.
3. Only verifiable claims are kept, and the "no review round" half is retained with its command: "NO review round exists for it (`grep -c \"Q-68\" docs/metrics/workflow.jsonl` returns 0)" at `plan.toml:1735`. Reproduced above.
4. Option (b)'s trade-off was rewritten. `plan.toml:1743` no longer contains "honoured in the breach"; it now reads "The `b6ba317` episode bears on this and cuts BOTH ways ... the planner half of (b) was paid, apparently without difficulty, which counts AGAINST the objection that (b) is too heavy to be followed; but the review-round half was not paid". The recommendation's closer was also rewritten: `grep -rn "would simply not be followed" docs/plans/` returns nothing, and `plan.toml:1751` now opens "On (b), with the correction applied honestly ... THAT ARGUMENT IS WITHDRAWN, and with it the claim that the heavier rule would not be followed."
5. The ledger is cited by quoted text, not by line number. See `R2-1`.

Nothing anywhere still leans on the withdrawn inference: `grep -rn "single-parent\|honoured in the breach\|would simply not be followed\|no planner branch" docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.questions/` returns only the two rebuttal instances (source and its render).

See `R3-1` below for a residual phrasing inconsistency inside the corrected paragraph. It does not reopen `NEW-1`.

---

## NEW-2: CLOSED

`pack/AGENTS.md:71` read in full in this worktree (`sed -n '71p' pack/AGENTS.md`). Its main clause is qualified:

> Here "updates this queue" means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job (routed as above):

and only the trailing rationale clause after the colon is not:

> the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits, which author no reviewed product content, so a `[[question]]` or `[[step]]` (reviewed product content) is not among them.

The new characterisation is accurate on both halves.

All four locations are fixed. `grep -rn "without qualification" docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.questions/` returns nothing. The replacements:

- `plan.toml:1733` (`Q-69` ask): "reads two different ways inside a single sentence", then quotes the main clause and the trailing clause separately and says "it is the trailing clause, not the main one, that collides with the call sites".
- `plan.toml:1254` (step 91 `title`): "the trailing rationale clause of `pack/AGENTS.md:71` excludes a `[[question]]` of ANY status from the orchestrator's direct-on-main edits, dropping the qualifier its own main clause carries".
- `docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md:5`: "`pack/AGENTS.md:71`'s MAIN clause is already qualified ...; its TRAILING rationale clause after the colon drops that qualifier".
- `docs/plans/agent-scaffold.steps/decision-folder-currency.md:40`: "The TRAILING rationale clause of `pack/AGENTS.md:71` ... dropping the \"a decided decision's ... fold\" qualifier that the same sentence's main clause carries".

`exploring-item-actor-boundary.md:11` still opens option (a) with "Narrow its closing clause", the already-correct localisation the triage said not to change. It was extended, not altered: the round-2 diff on that line adds "matching the qualifier the same sentence's main clause already carries" and the DO-NOT-treat-this-as-a-pure-rewording sentence the triage asked for. The correct phrase is intact.

The planner's added claim about the trailing clause's provenance checks out. `src/isolation_policy.rs:33` defines `ISOLATION_POLICY_FRAGMENT`, whose closed list is "flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor" and which contains no `[[question]]` authoring of any status (the string "question" does not occur in the fragment). `grep -n "isolation_policy" pack/AGENTS.md` -> `91:{{isolation_policy}}`, so the fragment is rendered into that slot in `pack/AGENTS.md`, and `pack/AGENTS.md:71`'s "the generated isolation-policy fragment below" points forward at line 91. Both halves of the claim are accurate.

---

## NEW-3: CLOSED

`grep -n '^\[\[principle\]\]' docs/plans/agent-scaffold.plan.toml` -> 8 entries (1755, 1760, 1765, 1770, 1775, 1780, 1785, 1790). Every principle cited in `Q-69` matches its number and name as committed:

- Principle 1 = "Prefer the cleaner long-term architecture over the smallest diff". Cited by that exact name at `plan.toml:1747` and `:1749`.
- Principle 2 = "Minimal by default". Cited by that exact name at `:1747`.
- Principle 5 = "Make illegal states unrepresentable". Cited by that exact name at `:1749`.
- Principle 8 = "Structured data first, project for humans". Cited by that exact name at `:1745` (option (c)'s trade-offs) and `:1749`.

Principle 8 is now named where option (c)'s argument rests on it: `:1745` says option (c) "keeps ONE criterion, which is Structured data first, project for humans (Principle 8) and the one-source-of-truth thinking it sharpens", and `:1749` says "Option (c) is the stronger REASONING, and it rests on Structured data first, project for humans (Principle 8)". Principle 8's own text ends "it sharpens Principle 1 (cleaner long-term architecture) and Principle 16-equivalent one-source-of-truth thinking", so "the one-source-of-truth thinking it sharpens" is an accurate paraphrase.

The precedence claim is accurate and correctly disclaimed. `:1749` says "Principle 8's declared precedence is over Principles 2 and 3 and so does not arbitrate this split, which turns on Principles 5 and 1; it is named here because it owns (c)'s argument, not as a tie-breaker." Principle 8's committed text says "when this conflicts with Principle 2 (minimal) or Principle 3 (safe on existing projects) at this stage, this wins", and declares no precedence over 1 or 5. Principle 3 is indeed "Safe on existing projects". The triage's instruction ("Do NOT assert that Principle 8's precedence clause decides the matter") is followed exactly.

---

## NEW-4: CLOSED

The added paragraph is at `plan.toml:1737`, opening "WHAT THE OPTION SET TAKES AS GIVEN: that the placeholder is a plan `[[question]]` at all." Both of its evidence claims verify:

- `exploring` is a typed `QuestionStatus` variant in the structured source. `src/plan/source.rs:333-337` declares the enum with `Exploring => "exploring"`, `:352` lists `QuestionStatus::Exploring` in `ALL`, `:363` gives its label, and `:377` is a compile-time drift guard on the variant count.
- `pack/AGENTS.md:65` contains, verbatim, "The Open-Questions item points at the exploration by path while it is `exploring`". Confirmed by reading line 65 in full; it is the Design-explorations paragraph.

The paragraph's conclusion ("relocating it would orphan a schema variant and break the mechanism by which an exploration is referenced; that is a schema and entry-mode change beyond this question, closer to `Q-68`'s territory") follows from those two facts, and it states the exclusion as a boundary of the option set rather than silently omitting it, which is what the finding asked for.

---

## R2-1: CLOSED

No stale `:345` citation survives in any artifact: `grep -rn "ledger.md:345" .` returns hits only inside `docs/plans/agent-scaffold.reviews/` (the round-1 reviewer/triage files and the round-2 files that raise the defect), which are the historical review record and are not artifact text.

Both sidecar citations now cite by quoted text:

- `docs/plans/agent-scaffold.steps/decision-folder-currency.md:7`: "recorded in its residual note in `docs/plans/agent-scaffold.ledger.md`, which names \"`pack/prompts/orchestrator.md:33` AND the parallel `pack/AGENTS.md:63`\" and no others."
- `docs/plans/agent-scaffold.steps/decision-folder-currency.md:14`: "named in its accepted-residual note in `docs/plans/agent-scaffold.ledger.md` (locate it by the quoted text above, not by line number)."

The quoted text appears verbatim, exactly once, in the ledger:

```
$ grep -c 'pack/prompts/orchestrator.md:33` AND the parallel `pack/AGENTS.md:63`' docs/plans/agent-scaffold.ledger.md
1
```

The correction is present in the generated view too (`render --check` is clean).

See `R3-2` below for a defect in the justification clause the fix added at `:7`. It does not affect the closure of `R2-1`: the citation itself now resolves.

---

## R2-2: CLOSED

`docs/plans/agent-scaffold.steps/decision-folder-currency.md:3` now reads "reads while scheduling that follow-up found two more points, of a related kind but not the same class (see below): FOUR passages are still out of step with the rule". That is the triage's second suggested wording and it is consistent with the rest of the sidecar:

- `:5` "The four fall into TWO CLASSES, and the operation each needs is different, so do not treat them alike" is now anticipated by "(see below)" rather than contradicted.
- `:7` says the two actor-less passages "and only these two" are what the step-89 residual named, and `:8` says of the other two "These two were never covered by the step-89 residual". Both are consistent with "not the same class".
- The per-passage bullets at `:12-15` and the per-passage operations at `:19-22` still distinguish the classes.

No sentence in the sidecar now asserts the four are alike.

---

## Regressions: none found

The round-2 fix commit touched exactly four files (`git diff --name-only 09ef94e..981d9f5`): `docs/plans/agent-scaffold.md` (generated), `docs/plans/agent-scaffold.plan.toml`, and the two sidecars. Within `plan.toml` it touched exactly two hunks (`git diff 09ef94e..981d9f5 -- docs/plans/agent-scaffold.plan.toml | grep '^@@'` -> `@@ -1251,7 +1251,7 @@` and `@@ -1730,17 +1730,25 @@`), which are the step 91 `title` and the `Q-69` `ask` and nothing else.

`Q-69` internal consistency after the rewrite. The three options are still mutually exclusive and each still carries trade-offs; the recommendation is still (a); the new "On (a) versus (c)" and "On (b)" paragraphs do not contradict the option trade-offs above them. One residual phrasing inconsistency is raised as `R3-1`.

Step 91's sidecar still agrees with `Q-69`:

- `exploring-item-actor-boundary.md:5` and `plan.toml:1735` give the same account of `b6ba317` (planner-attributed by the ledger, no review round, partial compliance with (b) rather than a breach), and the sidecar defers to the item ("The full evidence and its correction are in the `Q-69` queue item; do not re-derive them here"), so there is one source.
- `:11` (option (a)) now carries the same "must reconcile with the generated fragment" constraint the item's option (a) states.
- `:12` (option (b)) and `:13` (option (c)) match the item's (b) and (c), including the fourth-deployed-asset point.
- `:15`'s no-restatement constraint and `:19`'s regeneration instruction are unchanged and still correct.

Citations in the changed text all resolve (spot-checked in this worktree): `pack/prompts/orchestrator.md:31` third branch is "record it as an `exploring` Open-Questions item"; `pack/user-prompts/explore.md:13` is "record this as an `exploring` open question", `:3` restates it and `:7` is "Act as the orchestrator described in `.agents/prompts/orchestrator.md`"; `pack/pack.toml:166-167` are the `source`/`dest` lines for `user-prompts/explore.md`; `src/manifest.rs:615` is `".agents/user-prompts/explore.md"`; `pack/AGENTS.md:79` is "Format only your own files" and `:108` is "Prose formatting"; `src/agents_md_drift.rs` exists.

---

## Scope violations: none found

- `R2-3` (accepted residual) is untouched. `grep -c "three actor-less \`pack/AGENTS.md\` prose points" docs/plans/agent-scaffold.plan.toml` -> `1`, inside the `Q-67` `ask` at `plan.toml:1719`, and the round-2 commit's plan.toml hunks (above) do not include line 1719. The clause is present and unchanged.
- `T-4` (accepted residual: the sidecar paraphrases the generated closed list). `decision-folder-currency.md:26` still contains "(a step's status flip, an increment declaration, a round record, and the ledger's resume anchor)". Untouched, so the residual stands as accepted rather than being "helpfully" fixed. The sibling record `docs/plans/agent-scaffold.steps/planner-folds-decisions.md` is not in the fold's file list at all (`git diff --name-only e66d11a..981d9f5`), so the corpus consistency `T-4`'s acceptance rested on is preserved.
- `T-5` (INVALID). `decision-folder-currency.md:26` still ends "copy the pointing, not the list. The prompt may reference the rule in `AGENTS.md` rather than reproduce the fragment's contents, and it must not enumerate the four direct-on-main edits itself." Not reopened.
- `T-7` (INVALID). Step 90's `title` and heading are unchanged by the round-2 commit; the only `title` edit is step 91's, which `NEW-2` required.

---

## R3-1 (new, `low`): the corrected `Q-69` paragraph disclaims the planner attribution and then asserts it

Severity: `low`.

`docs/plans/agent-scaffold.plan.toml:1735` says, three sentences apart:

> What is NOT established is who authored it.

and

> Read correctly, the episode is PARTIAL COMPLIANCE with option (b), a planner authored the placeholder and the review round was skipped, NOT a breach of it.

The second sentence states as fact the thing the first declares unestablished. The intervening sentences give the ledger's attribution and its chronological coherence, so the intended reading is plainly "on the ledger's account, a planner authored it", but the sentence does not say that, and the assertion is reused downstream: `plan.toml:1751` says "the observed cost of (b)'s planner half was low, which is the best evidence available for (b)", again unhedged. Only `plan.toml:1743` hedges it, with "the planner half of (b) was paid, apparently without difficulty".

Reproduce: `sed -n '1735p;1743p;1751p' docs/plans/agent-scaffold.plan.toml`, or read the same three passages in the generated view at `docs/plans/agent-scaffold.md:196`, `:204`, `:212`.

Why it is a finding rather than a nit. `Q-69` is an undecided item going to a human under the human-input contract, and the round-2 `NEW-1` verdict turned on exactly this: an unestablished attribution stated as fact in this paragraph. The rewrite reverses the direction of the error (it now favours (b) rather than opposing it) and discloses its source inline, but the same "asserted where the item itself says it is not established" shape is still present in the conclusion sentence and in one downstream reuse.

Why it is `low`, not higher. The evidence is quoted in the item, so the human can see exactly what the claim rests on and how strong it is; both the hedged and the unhedged reading point the same way, so no argument inverts; the honest-correction paragraph at `:1751` explicitly names the earlier over-read and withdraws it; and the item is still `open`, so nothing has been decided on it.

Fix. One clause. For example, at `:1735` "on that record, a planner authored the placeholder and the review round was skipped", and at `:1751` "the ledger-recorded cost of (b)'s planner half was low". Alternatively, drop "What is NOT established is who authored it" and say instead that the only record that speaks to it attributes the capture to a planner, which is what the paragraph then argues.

---

## R3-2 (new, `low`): the `R2-1` fix justifies its hedge with a reason that is both inaccurate and backwards

Severity: `low`.

`docs/plans/agent-scaffold.steps/decision-folder-currency.md:7` ends:

> (Find that note by the quoted text, not by a line number: the ledger is append-only in practice, so any line citation into it rots.)

Two problems in one clause.

First, the inference runs backwards. A strictly append-only file is the one case in which line citations do NOT rot: appending at the end leaves every existing line number fixed. Line citations rot because content is inserted or rewritten ABOVE the cited line, which is the opposite of append-only.

Second, the factual claim is contradicted by the very churn that caused `R2-1`. The step-89 residual note moved from line 345 to line 355 because ten lines landed before it, mid-file:

```
$ git diff --numstat caeee2b e3fca03 -- docs/plans/agent-scaffold.ledger.md
11	1	docs/plans/agent-scaffold.ledger.md
$ git diff caeee2b e3fca03 -- docs/plans/agent-scaffold.ledger.md | grep '^@@'
@@ -334,7 +334,17 @@ We are DOGFOODING the role-separated workflow on this repo itself (it is self-sc
```

One line rewritten and ten added, starting at line 334, well above the note. So the ledger is edited in place, not append-only, and it is that in-place editing (not appending) that broke the citation.

The parallel hedge in the `Q-69` `ask` at `plan.toml:1735` says "the file is append-heavy and line citations into it rot", which is looser and does not carry the same error, so the two hedges also disagree with each other.

Impact if unfixed: the instruction the clause justifies ("find that note by the quoted text") is correct and `R2-1` stays closed either way, so this costs a reader a moment of confusion in an implementer-facing sidecar. That is why it is `low` and not higher.

Fix. Replace the reason with the accurate one, for example "the ledger's resume block is rewritten in place, so line numbers into it shift". Deleting the clause entirely and keeping "Find that note by the quoted text, not by a line number" also works and matches the hedge the same sidecar already uses for its pack citations at `:10`.

---

## Summary

| id | verdict | severity |
| --- | --- | --- |
| `NEW-1` | CLOSED | n/a |
| `NEW-2` | CLOSED | n/a |
| `NEW-3` | CLOSED | n/a |
| `NEW-4` | CLOSED | n/a |
| `R2-1` | CLOSED | n/a |
| `R2-2` | CLOSED | n/a |
| `R2-3` | untouched, residual intact | n/a |
| `R3-1` | new | `low` |
| `R3-2` | new | `low` |

No findings at `medium`, `high`, or `critical`. No regressions and no scope violations.
