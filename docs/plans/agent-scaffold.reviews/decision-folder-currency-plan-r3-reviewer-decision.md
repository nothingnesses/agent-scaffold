# Round 3 review: decision-readiness of `Q-69`

Reviewer: independent, round 3, DECISION READINESS lens. Target: the `Q-69` `[[question]]` entry in `docs/plans/agent-scaffold.plan.toml` (whole `ask`), its dependent step 91 sidecar `docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md`, and the rendered projection in `docs/plans/agent-scaffold.md`. Reviewed at worktree commit `981d9f5`; the recent change was inspected as `git diff 09ef94e..981d9f5`, but every judgement below is against the CURRENT full text, not the diff.

The single question this review answers: if a human read `Q-69` right now, with no other context, would they be equipped to decide correctly, or could they be misled?

Round-2 fix verification is the parallel reviewer's lens and is not covered here. `R2-3` (accepted residual) and `T-4` (accepted) are left alone. The withdrawal of the `b6ba317` commit-shape argument is treated as settled and is not re-litigated; it is only checked for whether it was honestly weighed.

Three findings: one `medium`, two `low`. No `high` and no `critical`.

---

## DEC-1 (`medium`): the recommended option permits a direct-on-main edit that the generated closed list excludes by enumeration, and neither the option set nor step 91 says whether `src/isolation_policy.rs` must change

The `ask` treats the generated fragment's list as CLOSED, three times, at `docs/plans/agent-scaffold.plan.toml:1733`:

- "the generated isolation-policy fragment below lists the orchestrator's **closed set** of direct-on-main edits" (quoting `pack/AGENTS.md:71`);
- "it is an INDEPENDENT DERIVATION from the generated `ISOLATION_POLICY_FRAGMENT` ..., whose **closed list** of integration edits contains no `[[question]]` authoring of any status";

and at `:1741`, option (a): "so (a) has to supply a REASON the fragment's **closed list** does not reach an `exploring` placeholder".

What that closed list actually says, verbatim, at `src/isolation_policy.rs:33` (rendered into the `{{isolation_policy}}` slot at `pack/AGENTS.md:91`, and present as the committed `AGENTS.md:91`):

> The only edits made directly on main are the orchestrator's own integration-level ones, which author no reviewed product content and so stay the orchestrator's direct job rather than a spawned agent's: flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor.

Reproduce:

```
sed -n '33p' src/isolation_policy.rs | grep -o "The only edits made directly on main.*"
awk 'NR==91' AGENTS.md | grep -o "The only edits made directly on main.*"
grep -n "isolation_policy" pack/AGENTS.md     # -> 91:{{isolation_policy}}
```

The sentence is an EXHAUSTIVE enumeration ("The only edits ... are ...: [four items]"), not an open-ended criterion followed by examples. The `ask` itself asserts that reading by calling it closed.

Both option (a) (the recommendation) and option (c) grant the orchestrator a FIFTH direct-on-main edit: recording an `exploring` placeholder `[[question]]` in `<task>.plan.toml`. Under the closed reading the `ask` asserts, that edit is excluded by the enumeration, not merely by the rationale clause. So:

- Option (a)'s stated remedy at `:1741` ("supply a REASON the fragment's closed list does not reach an `exploring` placeholder; that reason is option (c)'s (the placeholder authors no reviewed product content)") answers the fragment's RATIONALE clause ("which author no reviewed product content") but leaves the ENUMERATION unrepaired. A reason why an unlisted edit is harmless is not a reason why an exhaustive list of the only permitted edits does not exclude it.
- Option (c) has the same exposure. Its pitch at `:1745` is that it "resolves the contradiction by reading the generated fragment rather than by carving an exception around it", which only works if the four items are illustrative of the rationale rather than exhaustive. The `ask` never says which, and it asserts the opposite ("closed").

Consequence for step 91, which is the operational half of the defect. `docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md:11` (option (a)) names edits to `pack/AGENTS.md:71` and optionally `pack/AGENTS.md:45`, and gives the regeneration set as "`AGENTS.md` and `.agents/AGENTS.reference.md`". Line `:13` (option (c)) is the same set. `src/isolation_policy.rs` appears exactly once in the whole sidecar, at `:15`, and only as a do-NOT constraint ("do NOT restate the generated closed list ... whatever replaces that sentence must keep pointing rather than enumerate"):

```
grep -c "isolation_policy" docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md   # -> 1
grep -n "isolation_policy" docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md   # -> 15: the "do NOT restate" constraint
```

So an implementer following the sidecar under the recommended option ships a `pack/AGENTS.md` whose `:71` and `:45` prose permits the orchestrator to record an `exploring` placeholder directly on main, twenty lines above a generated `:91` that states those four items are "the only edits made directly on main". That is the same class of two-passages-disagree defect that `Q-69` exists to remove, reintroduced in the same file by the fix, and it would then carry relitigation protection.

Why this is a decision-readiness defect and not a disagreement with the recommendation. A human choosing (a) is told the residual cost is one recorded sentence of reasoning. The real requirement is a ruling on a question the item never puts to them: is the fragment's four-item list exhaustive (in which case (a) and (c) both require an amendment to the drift-guarded generated const at `src/isolation_policy.rs:33`, which ships into every scaffolded project's `AGENTS.md` and `.agents/AGENTS.reference.md`), or is it illustrative of the "author no reviewed product content" criterion (in which case no source change is needed and (c) is exactly right)? That ruling changes the edit surface, the blast radius, and the (a)-versus-(c) comparison, and it is invisible in the current option set.

Not a re-raise. Round 2's `NEW-2` triage (`docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-r2-triage.md:105`) framed the same area as "Either the `exploring` placeholder is argued to author no reviewed product content, which is option (c)'s move, or the guidance carries an exception to the fragment's own rationale", and the current `ask` implements the first horn. My evidence is the fragment's enumerating sentence itself ("The only edits ... are ...: [four]"), which no round-1 or round-2 finding quoted, and the point is that answering the rationale does not answer the enumeration. `NEW-4`'s excluded alternative was the ledger-side placeholder, now disposed of at `:1737`; this is a different omission.

Cheapest fix: one clause in the `ask` stating whether the four-item list is exhaustive or illustrative and, if exhaustive, adding "amend `src/isolation_policy.rs:33` and regenerate" to options (a) and (c) in the sidecar's per-option edit lists.

---

## DEC-2 (`low`): "the wider reading" is used with two opposite referents inside the `ask`'s first paragraph, so the item's headline sentence states the conflict backwards

All three citations are on `docs/plans/agent-scaffold.plan.toml:1733` and `:1747`, re-read immediately before writing:

1. `:1733`, the item's opening clause: "the actor boundary `Q-67` added at `pack/AGENTS.md:71` reads two different ways inside a single sentence and **three shipped passages follow the wider reading**."
2. `:1733`, four sentences later: "That trailing clause is not a loose restatement of the main one: it is an INDEPENDENT DERIVATION from the generated `ISOLATION_POLICY_FRAGMENT` ... **So the wider reading has a real source, and it is the trailing clause**, not the main one, that collides with the call sites."
3. `:1747`, the recommendation: "Prefer the cleaner long-term architecture over the smallest diff (Principle 1) favours making one sentence agree with itself over **bending three call sites to its wider half**."

Citations 2 and 3 fix "the wider reading" / "its wider half" as the TRAILING clause (the wide exclusion), and both say the three call sites do NOT currently follow it: `:1733` says "the trailing clause forbids what three passages prescribe", and `:1747` says (b) would have to BEND them to it. Citation 1 says the three passages FOLLOW the wider reading. Exactly one of these can be true, and the term is never defined at first use.

Reproduce:

```
grep -c "the wider reading" docs/plans/agent-scaffold.plan.toml    # -> 1 line, both uses
grep -o "three shipped passages follow the wider reading" docs/plans/agent-scaffold.plan.toml
grep -o "So the wider reading has a real source, and it is the trailing clause" docs/plans/agent-scaffold.plan.toml
grep -o "bending three call sites to its wider half" docs/plans/agent-scaffold.plan.toml
```

Why it matters here rather than being a wording quibble. This is the item's first sentence, and it is the sentence that frames the whole decision; it also renders verbatim as the opening of the queue bullet at `docs/plans/agent-scaffold.md:194`. A reader who takes it at face value has the direction of the conflict inverted: they would believe the three call sites implement the trailing clause, when the item's entire case is that the trailing clause forbids what those call sites prescribe. The body corrects it within the same paragraph, which is why this is `low` and not higher, but a durable decision record whose subject is a sentence that "reads two different ways" should not itself reuse an undefined comparative with opposite senses. The step 91 `title` at `:1254` states the same conflict correctly and unambiguously, and can serve as the model for the fix.

---

## DEC-3 (`low`): the load-bearing episode is summarised as "NOT a breach" of option (b) in the same clause that records the skipped review round, contradicting the item's own option (b) analysis

`docs/plans/agent-scaffold.plan.toml:1735`:

> Read correctly, the episode is PARTIAL COMPLIANCE with option (b), a planner authored the placeholder and the review round was skipped, NOT a breach of it.

`docs/plans/agent-scaffold.plan.toml:1743`, option (b)'s own trade-offs:

> but the review-round half was not paid, and under (b) as written a planner-authored `[[question]]` is reviewed product content, so **that round is not optional**. The honest reading is that (b)'s planner half looks affordable and its review half is the part that did not happen even when the actor rule was being consciously applied.

If (b) requires both halves and `:1743` says the review round "is not optional", then skipping it IS a breach of (b)'s review half. `:1735` names the skip and denies the breach in one clause. The two sentences give a human weighing "would (b) actually be followed?" two different signals about the only observed instance, which is the item's single piece of empirical evidence.

Reproduce:

```
grep -o "PARTIAL COMPLIANCE with option (b), a planner authored the placeholder and the review round was skipped, NOT a breach of it" docs/plans/agent-scaffold.plan.toml
grep -o "so that round is not optional" docs/plans/agent-scaffold.plan.toml
```

Direction note, so this is not read as an accusation of motivated reasoning: the inaccuracy at `:1735` cuts AGAINST the recommended option, not for it, since "not a breach" makes (b) look more followable than the evidence supports. `:1743` states the accurate version. The fix is to make `:1735` say what `:1743` says: the actor half was complied with, the review half was not.

---

## Answer to the central question

**Could a human be misled? Yes, on one point, and only one: DEC-1.**

A human deciding (a) or (c) from this item alone would understand the cost of the recommended option to be a single reworded sentence in `pack/AGENTS.md` plus a recorded reason, and would not learn that the same edit grants a direct-on-main authority that the generated, drift-guarded, ships-to-every-scaffolded-project enumeration at `src/isolation_policy.rs:33` states is not among "the only edits made directly on main". That is a cost and an edit surface the item does not disclose, and it is the exact defect class the item exists to fix.

DEC-2 and DEC-3 are comprehension defects, not decision-changing ones: a human reading the whole item gets the correct picture from the body in both cases. They are reported because this record gains relitigation protection the moment it is decided, so a later reader who reads only the framing sentence or only the episode summary would be misled about the direction of the conflict (DEC-2) or about what the one observed instance shows (DEC-3).

On everything else, the item is decision-ready: the option set is genuinely argued, no option is a straw man, the recommendation's costs are stated at least as plainly as the rejected options' costs, the withdrawal is weighed rather than acknowledged and ignored, and every Principle citation is correct by number, name, and application.

---

## Verified clean (load-bearing facts and Principle citations)

Every citation below was opened and re-read at `981d9f5` immediately before writing this file.

**Citations in the `ask`, verbatim.**

- `pack/AGENTS.md:71` main clause and trailing rationale clause: both quoted verbatim in the `ask`, including the colon boundary. Confirmed with `awk 'NR==71' pack/AGENTS.md`.
- `pack/AGENTS.md:45`: "the orchestrator records the question as an Open-Questions item with status `exploring`". Verbatim.
- `pack/prompts/orchestrator.md:31`: "record it as an `exploring` Open-Questions item". Verbatim, and it is genuinely the THIRD branch of the three-branch sentence (branch 1 "answer a purely factual question directly", branch 2 "record the resolved answer as a durable Open-Questions decision", branch 3 the exploring case).
- `pack/user-prompts/explore.md:13`: "record this as an `exploring` open question". Verbatim. Restated at `:3` ("The agent records the question, runs the exploration"). The orchestrator instruction at `:7` is "Act as the orchestrator described in `.agents/prompts/orchestrator.md`". All three sub-citations correct.
- `pack/AGENTS.md:65`: "The Open-Questions item points at the exploration by path while it is `exploring`". Verbatim.
- `src/isolation_policy.rs` holds `ISOLATION_POLICY_FRAGMENT` (line 33); `pack/AGENTS.md:91` is the `{{isolation_policy}}` slot. Both correct. The fragment's four-item list genuinely contains no `[[question]]` authoring of any status, so the `ask`'s characterisation of the trailing clause as an independent derivation is correct (this is what makes DEC-1 bite, not a defect in itself).
- `src/plan/source.rs`: `QuestionStatus::Exploring` is a real typed variant (`:337`, `:363`). The exclusion argument at `:1737` stands.
- `pack/pack.toml:166-167` is exactly `source = "user-prompts/explore.md"` / `dest = ".agents/user-prompts/explore.md"`; `src/manifest.rs:615` is exactly `".agents/user-prompts/explore.md",`. Both line numbers correct to the line.
- T-3a provenance: `docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-triage.md:217` records `T-3a | VALID BUT OUT OF SCOPE | medium`. Matches the `ask`'s description.
- `[meta].w4_baseline = "Q-44"` (`docs/plans/agent-scaffold.plan.toml:3`), so `Q-69` is past the cutoff and the "receipt due only when decided" statement at `:1753` is correct.

**The re-verified load-bearing evidence (checked independently, not from any summary).**

- `grep -c "Q-68" docs/metrics/workflow.jsonl` returns `0`. Re-run in this worktree. The "no review round exists for it" claim holds.
- The ledger attribution, located BY CONTENT as the `ask` instructs: `grep -n "NEW BACKLOG" docs/plans/agent-scaffold.ledger.md` finds exactly one hit, and it reads "NEW BACKLOG (captured 2026-07-26 by a planner, per Q-67)". Verbatim match. The `ask`'s instruction to find it by quoted text rather than line number is correct practice and it works.
- Contemporaneity: `b6ba317` is dated `2026-07-26 22:37:46 +0100`; the ledger commit carrying that sentence is `8d12264`, `2026-07-26 22:40:29 +0100`. Two minutes 43 seconds, so "written three minutes later" is accurate.
- Chronological coherence: `4f48283` ("name the planner as folder of non-trivial decided decisions (Q-67)", the step-89 guidance landing) is dated `22:05:29`, 32 minutes before `b6ba317`. "About half an hour" is accurate under the natural reading (the Q-67 guidance being live). Noted but not raised: the `ask` does not cite which event it dates from, and under the other reading (the Q-67 decision being recorded in the plan, `cca1099` at `16:47:15`) the gap is about six hours. The conclusion drawn, that the attribution is chronologically coherent, holds under both, so this is not a finding.
- The commit-shape disposal is correct and verifiable. `git rev-list --parents -n 1` gives exactly one parent for each of `557fa46`, `4f48283`, `cca1099`, and `b6ba317`; the ledger records the first three as ff-merges of reviewed branches (`ledger.md:355`: "ff-merged `557fa46`", "STEP 89 COMPLETE (ff `4f48283`)", "PLAN FOLD MERGED (ff `cca1099`)"). The `ask`'s "Nothing about the commit's single-parent shape bears on the question either way" is sound and the withdrawn argument does not survive anywhere in the current text.
- `b6ba317` did add `Q-68` with `status = "exploring"` to `docs/plans/agent-scaffold.plan.toml` (plus the empty `docs/plans/agent-scaffold.questions/Q-68.md`), with parent `9e12585`, a main commit. Confirmed with `git show b6ba317 --stat` and `git show b6ba317 -- docs/plans/agent-scaffold.plan.toml`.

**Principle citations: all four correct by number AND name against the plan's 8 `[[principle]]` entries (`plan.toml:1755-1793`), and none uses `AGENTS.md`'s different 22-item numbering.**

- "Prefer the cleaner long-term architecture over the smallest diff (Principle 1)" matches `n = 1`. The argument made ("making one sentence agree with itself") follows from the principle's TEXT, which names "internal coherence".
- "Minimal by default (Principle 2)" matches `n = 2`. Checked specifically for a title-versus-text mismatch, since `n = 2`'s recorded text is about the tool's core-versus-optional-module architecture rather than about process ceremony. NOT raised, because the ceremony reading is the plan's own established convention, not something invented here: `plan.toml:1524` already writes "the TRADE-OFFS (the Principle 2 ceremony cost vs the benefit)" in a human-directed item. Judging `Q-69` against a narrower reading than the plan applies elsewhere would be re-litigating a plan-wide convention.
- "Make illegal states unrepresentable (Principle 5)" matches `n = 5`, and it is explicitly flagged as "read as its documentation analogue", so the reader is warned it is an analogy. The analogy is sound against the text (encode the valid outcomes up front rather than guarding at runtime -> a checkable token rather than a per-edit judgement).
- "Structured data first, project for humans (Principle 8)" matches `n = 8`. The precedence claim at `:1749` ("Principle 8's declared precedence is over Principles 2 and 3 and so does not arbitrate this split") matches `n = 8`'s text exactly ("when this conflicts with Principle 2 (minimal) or Principle 3 (safe on existing projects) at this stage, this wins, and it sharpens Principle 1"). The item correctly declines to use it as a tie-breaker.
- The step 91 sidecar at `:15` cites "the AGENTS.md workflow guidance 'One source of truth' and plan Principle 8", correctly keeping the two numbering systems apart rather than writing "Principle 16" in the plan's numbering.

**Option set, distinctness, straw men.**

- The three options are genuinely distinct in their pack edits, and the sidecar's per-option edit lists confirm it: (a) rescopes `:71`'s closing clause on the status token, (b) leaves `:71` alone and rewrites the three call sites, (c) rewrites `:71`'s closing clause to turn on the content criterion.
- No straw man. (b) is given real strengths ("one flat, memorable rule", "needs no reconciliation with the generated fragment, since it is what the fragment already implies", planner half observed affordable) and an explicit condition under which a human should choose it ("A human who weighs a single flat rule above that proportionality argument should pick (b)"), which is a real preference a human might hold. (c) is called "the stronger REASONING" and "the only option that resolves the contradiction by reading the generated fragment".
- The (a)+(c) hybrid the recommendation actually proposes IS carryable: `exploring-item-actor-boundary.md:11` already instructs the (a) implementer to carry (c)'s "authors no reviewed product content" reason, so the recommendation and the step agree. (Subject to DEC-1, which is about whether that reason suffices, not about whether it is actionable.)
- The one excluded alternative (keeping the placeholder ledger-side) is excluded WITH a reason at `:1737`, not silently. Considered and not raised: a "change nothing, accept the residual" option is not in the set, but it is available to a human under `AGENTS.md:57` regardless, and the item establishes that reading the whole sentence does not dissolve the contradiction, so its omission does not mislead.

**Recommendation honesty.**

- The withdrawal is weighed, not merely acknowledged: `:1751` states it, states that it "narrows the margin", records the surviving pro-(b) datum ("the observed cost of (b)'s planner half was low, which is the best evidence available for (b) and is now recorded as such"), re-derives the recommendation on other grounds, and names the condition for choosing (b) instead.
- The withdrawn inference is not smuggled back. The surviving anti-(b) point at `:1743` ("its review half is the part that did not happen") rests on the independently verified absence of a review round (`grep -c "Q-68" docs/metrics/workflow.jsonl` -> 0), not on any claim about WHO authored the commit, so it is a different argument on different evidence, and the guidance it measures against was live at the time (step 89 landed `22:05`, the commit is `22:37`).
- Option (a)'s costs are stated at least as plainly as (b)'s and (c)'s: `:1741` says "The real cost is NOT the wording", names the unexplained-exception risk, and adds the secondary two-dimensional-boundary cost.

**Actionability of the recommended option, against the real tree.**

- Option (a)'s regeneration set is correct: `pack/pack.toml:28-29` maps `source = "AGENTS.md"` to `dest = "AGENTS.md"` and `:99-100` maps it to `dest = ".agents/AGENTS.reference.md"`; those are the only two dests for that source, so "`AGENTS.md` and `.agents/AGENTS.reference.md`" is exactly right.
- Option (b)'s fourth-asset claim is correct: `pack/prompts/orchestrator.md` has one dest, `.agents/prompts/orchestrator.md` (`pack/pack.toml:105-106`), and `pack/user-prompts/explore.md` has one dest, `.agents/user-prompts/explore.md` (`:166-167`), giving four deployed assets under (b) against two under (a) and (c).
- The sidecar's drift-guard caveat is correct: `src/agents_md_drift.rs` embeds only `../AGENTS.md` and `../.agents/AGENTS.reference.md` (`:45`, `:49`) and compares only those two against a fresh render (`:299`, `:300`), so the prompt and user-prompt copies are genuinely unguarded.
- The substituted regeneration command at sidecar `:19` matches the first line of the `scaffold-self` recipe (`justfile:47`), with `nix fmt` (`justfile:48`) deliberately omitted. Whether it reproduces `scaffold-self`'s output byte for byte minus the formatter is the parallel reviewer's fix-verification item and is not adjudicated here.

**Rendered projection (`docs/plans/agent-scaffold.md`).**

- Not misleading and not truncated. `cargo run -- render --check docs/plans/agent-scaffold.plan.toml` reports "docs/plans/agent-scaffold.plan.toml: up to date". The queue bullet at `docs/plans/agent-scaffold.md:194-214` carries the full `ask`, including the "WHAT THE ONE OBSERVED INSTANCE ACTUALLY SHOWS (corrected...)" paragraph, the "WHAT THE OPTION SET TAKES AS GIVEN" exclusion, all three options with their costs, and the "On (b), with the correction applied honestly" paragraph containing "THAT ARGUMENT IS WITHDRAWN". Nothing about the correction or the withdrawal is lost in projection. A human who reads only the rendered view sees exactly what the TOML says, so DEC-1, DEC-2 and DEC-3 apply identically to both views and no additional projection defect exists.

**Self-containment.**

- Every fact in the `ask` is citable from a durable artifact: pack and source files by `file:line`, the commit by hash, the ledger by quoted text (deliberately, with the line-rot reason given), and the metrics file by a re-runnable `grep -c`. No fact is conversation-only. The one provenance reference to a transient artifact (`T-3a`) is accompanied by "evidence reproduced" and the evidence is in fact reproduced inline, so the item survives the findings files being deleted at task close.
