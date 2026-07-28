# Plan-review round 5 findings: fix verification and fix-induced residue

Reviewer lens: verify each round-4 fix is genuinely closed AND introduced nothing. Independent of the planner that produced the artifact and of the orchestrator that drives the loop.

Worktree `.claude/worktrees/rev5-residue`, detached at `72ca2d7` (the round-4 fix commit, which is the artifact tip). Every `file:line` below was re-read in this worktree immediately before citing it; every command below was re-run here. I did not read the round 1 to 4 reviewer files as authority: I read the four triage files for the settled ledger only, and re-derived every claim from the artifacts.

Artifact: `git diff 7707df2..72ca2d7`. Primary target: `git diff b5ddb52..72ca2d7`.

## Summary

ONE finding, `R5-1`, `low`. Every round-4 fix (`H4-1`, the `:24` constraint, `H4-5`, `H4-4`, `R4-1`, `R4-2`, `R4-3`) is CLOSED. No settled finding is disturbed. Every mechanical check is green. The residue sweep returns only the deliberate past-tense provenance sentence and its projection.

`R5-1` is in the recurring fix-induced-residue class: the new requirement bullet at `decision-folder-currency.md:28` forbids reproducing `pack/AGENTS.md:71`'s trailing rationale clause in the prompt's checkpoint paragraph, while the unchanged paragraph at `:34` still holds that same clause up, quoted verbatim, as the thing to "copy" for that same paragraph. Before the fix the two agreed; after it they point opposite ways.

---

## Fix verification, one by one

### `H4-1` (medium): CLOSED

**The new SCOPE text is factually accurate.** Three claims, all checked against the real lines.

- Claim: `pack/AGENTS.md:41` "defines inline as `authoring a [[question]] or a [[step]]`". CONFIRMED. `pack/AGENTS.md:41` reads, in the human-input contract: "A resolved decision is recorded in the plan's Open Questions section and folded into the step it affects: when that fold is non-trivial (authoring a `[[question]]` or a `[[step]]`), the orchestrator routes it to the planner to author, as on the request-interrupt path above, rather than editing the plan directly." The quoted definition is verbatim, and it is genuinely inline (a parenthetical on "non-trivial"), not a cross-reference.
- Claim: a trivial fold stays with the orchestrator, per `pack/AGENTS.md:39`. CONFIRMED as support. `:39`: "A request is trivial only if it is local, reversible, changes neither the Success Criteria nor the Roadmap scope, and raises no new open question; such a request may be folded in directly." I checked the one gap an adversarial read finds here: `:39` is worded about a trivial REQUEST, not a trivial decision-FOLD, and it does not itself name the actor. It still supports the claim, because `:41` explicitly routes the non-trivial fold "as on the request-interrupt path above" (so it is `:41` that ties the two paths together) and closes with "rather than editing the plan directly", which entails that the non-non-trivial case IS the orchestrator editing the plan directly. Not a defect.
- Claim: same, per `pack/prompts/orchestrator.md:29`. CONFIRMED directly and without inference. `:29` is a second-person imperative to the orchestrator: "Fold a trivial request (local, reversible, no change to scope or Success Criteria, no new open question) in directly; route anything non-trivial to the planner to fold into the plan". This one names the actor outright.

**All four instructions are covered, none left bare.** Checked bullet by bullet at `decision-folder-currency.md:19` to `:22`:

- `:19` (checkpoint) is scoped INLINE: "not authoring the decided decision's `[[question]]` or `[[step]]` fold".
- `:20` (Socratic) is scoped INLINE: "the planner authors the non-trivial fold".
- `:21` (prompt ledger) is scoped BY REFERENCE: "under the same scope as the other three (see SCOPE below)".
- `:22` (guidance ledger) is scoped BY REFERENCE: "and under that same scope".

The governing bullet at `:26` opens "SCOPE, governing all four instructions above", so the reference has an explicit anchor and the coverage is complete.

**The back-reference wording is unambiguous.** I tried to misresolve it and could not. `:21`'s "under the same scope as the other three" is disambiguated in the same clause by "(see SCOPE below)", which names exactly one target, and `:26` independently declares itself as governing all four. `:22`'s "that same scope" has one antecedent, `:21`'s, which resolves to `:26`. Considered and NOT raised: "the other three" is loose, because one of the other three (`:22`) carries no independent scope of its own and points back at `:21`. That circularity is broken by the parenthetical and by `:26`'s self-declaration, so it cannot send an implementer anywhere wrong. It is a wording nit, not a defect, and I am not raising it.

I also checked the one substantive risk this fix could have created: whether `:26`'s "non-trivial" formulation (`:41`'s form) silently WIDENS `:19`'s narrower "a decided decision's ... fold" (`:71`'s form) along the decidedness axis, which is precisely the axis of the `Q-69` contradiction. It does not, on two grounds. First, `:19` and `:28` both pin the checkpoint paragraph to `:71`'s decided-decision form, and `:19` closes "match the guidance's existing clause, do not invent a different rule", so the more specific instruction governs that passage. Second, all four target passages are about DECIDED durable decisions by their own subject matter (the two ledger passages say "only durable decisions, the ones that change the plan"; the Socratic branch 2 is the already-clear-options case), so `:26`'s scope cannot reach an `exploring` placeholder in any of them. `:26`'s own "widening or narrowing it here would settle by the back door" sentence guards the same risk explicitly.

### The `:24` constraint (`pack/AGENTS.md:71` MAIN clause only): CLOSED

`pack/AGENTS.md:71` in full, read in this worktree:

> Checkpoints (the human-decision queue and progress). The plan's "Open Questions, Decisions, Issues and Blockers" section is a single living human-decision queue of the decisions the human owns, in the item format the plan template defines. At every checkpoint the orchestrator updates this queue and pushes its open items to the human, each per the human-input contract, rather than waiting for the human to pull them: a new human would not know to watch it, and a pull-only model is fragile. Here "updates this queue" means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job (routed as above): the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits, which author no reviewed product content, so a `[[question]]` or `[[step]]` (reviewed product content) is not among them.

- The quoted main-clause text at `decision-folder-currency.md:28` is VERBATIM against that line, with the only difference being the standard nesting convention (the shipped line's double quotes around `updates this queue` become single quotes inside the sidecar's double-quoted span). Not a fidelity defect.
- The main/trailing split is drawn at the RIGHT place. The last colon in the sentence falls after "(routed as above)", and everything after it is the isolation-policy rationale. `:28` says "NOT its trailing rationale clause after the colon", which is exactly that boundary. `plan.toml:1722` draws the same split at the same colon, so the two are consistent with each other.
- The quote at `:28` stops at "which is the planner's job" and drops "(routed as above)", which is inside the main clause. Considered and NOT raised: the routing element is carried by the operative per-passage instruction at `:19` ("which is the planner's job and which the orchestrator routes"), so nothing is lost to the implementer.

See `R5-1`: the constraint itself is correct, but `:34` was not reconciled with it.

### `H4-5`: CLOSED

`decision-folder-currency.md:15` now reads: "Actor-less, and the counterpart sentence to the one above, in the guidance rather than in the prompt, so fixing either one alone would leave the two copies disagreeing about the actor. They are counterparts, not duplicates: this one continues 'and a folded decision reopens only by evidence that beats its recorded reasoning', which the prompt's copy does not carry."

Both halves check out against the real text.

- `pack/AGENTS.md:63` ends: "Never put individual findings in the plan's Open Questions section; only durable decisions, the ones that change the plan, fold into the plan's steps, and a folded decision reopens only by evidence that beats its recorded reasoning." The quoted tail is verbatim.
- `pack/prompts/orchestrator.md:33` opens: "The ledger is separate from the plan: do not put individual findings in the plan's Open Questions section; only durable decisions, the ones that change the plan, fold into it." It stops at "fold into it" and carries NO reopens tail. So "which the prompt's copy does not carry" is correct, and "counterparts, not duplicates" is correct (they also differ on "Never" versus "do not" and on "the plan's steps" versus "it").

The derived requirement at `:29` is correspondingly scoped to the ACTOR CLAUSE and names the tail that must not be dropped. That closes the `H4-5` risk direction the round-4 triager identified.

### `H4-4`: CLOSED, and the criterion is sound

The criterion at `decision-folder-currency.md:30` is runnable and both of its supporting claims reproduce here.

```
$ diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md ; echo "exit: $?"
exit: 0
```

No output, exit 0. The criterion is therefore satisfiable and its "no output" form is the state of the tree today, so a work reviewer running it after the step gets a meaningful signal rather than a pre-existing failure.

The supporting claim that `pack/prompts/` contains no `{{...}}` template slots:

```
$ grep -rc "{{" pack/prompts/
pack/prompts/clarifying-questions.md:0
pack/prompts/checks-reviewer.md:0
pack/prompts/triager.md:0
pack/prompts/implementer.md:0
pack/prompts/orchestrator.md:0
pack/prompts/reviewer.md:0
pack/prompts/open-questions-gate.md:0
pack/prompts/planner.md:0
```

Zero in all eight. I also confirmed the byte-identity generalises, which is what makes the "stays meaningful" claim hold rather than being an accident of one file: all seven deployed prompts are identical to their pack sources (`orchestrator`, `planner`, `reviewer`, `triager`, `implementer`, `clarifying-questions`, `open-questions-gate` all `diff`-clean; `checks-reviewer` is module-gated and correctly not deployed, matching the caveat at `prompt-drift-guard.md:21`).

The criterion's third support ("this step forbids the implementer from running the repo-wide formatter") is also real, not asserted: `:44` gives the render-half-only command and forbids `just scaffold-self`, and `justfile:46-48` confirms that recipe is the render followed by `nix fmt`. `pack/AGENTS.md:79` forbids a writer running repo-wide formatters and `pack/AGENTS.md:108` rules on this exact case ("a writer does not proactively run a repo-wide formatter and leaves incidental reformatting to the orchestrator"). Both citations resolve.

### `R4-1`: CLOSED

`prompt-drift-guard.md:23` now reads "which is the reason it can land independently of step 90", and `:25` now reads "Interaction with step 90. This step does not block step 90 and step 90 does not block it. ... the order as it stands reflects that step 90 is already reviewed and in flight".

- Accurate: there is no step 91. `grep -n "^order = " docs/plans/agent-scaffold.plan.toml | tail -3` returns `1227:order = 89`, `1243:order = 90`, `1256:order = 92`.
- The operative claim SURVIVES intact: "This step does not block step 90 and step 90 does not block it", followed unchanged by the reordering argument and "since the two touch disjoint files". Nothing operative was lost in the rewording.
- The replacement of "the order given (90, then 91, then 92)" with "the order as it stands" removes the false ordering without asserting a new one, which is the minimal correct move.

### `R4-2`: CLOSED

`decision-folder-currency.md:54` now reads: "... the human kept it OUT of this step (2026-07-27), giving it its own question so the design gets decided rather than assumed. That call was first carried out as a question plus a step; on 2026-07-28 the human demoted the question to `exploring` and removed the step, because its edits depended on a decision not yet made, so the current disposition is `Q-69` with a design pass owed and no step."

- Accurate history. `git log --oneline` shows `b5ddb52 docs: demote Q-69 to exploring and drop its premature step`, and `plan.toml:1734` independently records "An earlier draft of this fold carried a step (`exploring-item-actor-boundary`, order 91) ... it was removed on 2026-07-28 because its content depended on a decision that has not been made". The two accounts agree on the date, the act, and the reason.
- Internally consistent with the rest of the file. `:20` says `Q-69` is "with a design pass owed and NO step yet"; `:48` says "with a design pass owed and NO step yet"; `:54` now says "with a design pass owed and no step". All three agree. The 2026-07-27 human call is preserved rather than rewritten, and the operative instruction ("Do not fold it back in here") is untouched.

### `R4-3`: CLOSED

```
$ grep -c 'qualified:"Here' docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md
docs/plans/agent-scaffold.plan.toml:0
docs/plans/agent-scaffold.md:0
```

`plan.toml:1722` now reads "Its MAIN clause is qualified: \"Here 'updates this queue' means ...". The space is present in the source and in the projection.

---

## The residue sweep

The prescribed sweep, run in this worktree:

```
$ grep -rn "step 91\|steps 90 and 91\|order 91\|91, then 92\|own question and step" docs/plans/ | grep -v "\.reviews/"
docs/plans/agent-scaffold.plan.toml:1734:  ... (`exploring-item-actor-boundary`, order 91) ... it was removed on 2026-07-28 ...
docs/plans/agent-scaffold.md:208:         ... (same sentence, projected) ...
docs/plans/agent-scaffold.ledger.md:341:  ... step 91 owns branch 3 ... (historical round record)
docs/plans/agent-scaffold.ledger.md:347:  ... step 91 REMOVED (committed deletion) ... (historical split record)
```

WITHIN THE ARTIFACT the only hits are `plan.toml:1734` and its projection at `agent-scaffold.md:208`, both the deliberate past-tense provenance sentence. That is exactly the expected result. The two ledger hits are out of scope per the brief and are correct historical narrative; I am not raising them.

I widened the sweep beyond what was prescribed, to catch a removed-step reference the fixed strings would miss:

```
$ grep -rn "\b91\b" docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.steps/ \
    docs/plans/agent-scaffold.questions/ docs/plans/agent-scaffold.md
```

Every remaining hit is one of three benign kinds: the `plan.toml:1734` / `agent-scaffold.md:208` provenance pair; `pack/AGENTS.md:91` (a line citation into the pack for the `{{isolation_policy}}` slot, which I verified resolves); and `agent-scaffold.md:5` "Status: 91 steps", which is the correct current count. No live reference to a step 91 survives anywhere in the plan source, the sidecars, the question sidecars, or the generated view.

I also swept for the exact pre-fix wordings the round-4 fixes replaced, in case a copy survived elsewhere:

```
$ grep -rn "end up saying the same thing\|the same sentence as the one above" docs/plans/ | grep -v "\.reviews/"
(no output)
```

---

## Full re-read of the three sidecars and the `Q-69` entry

I re-read `decision-folder-currency.md` (61 lines), `prompt-drift-guard.md` (25 lines), the empty `Q-69.md`, and the whole `Q-69` `ask` at `plan.toml:1717-1734`, looking for statements the round-4 fixes made stale, internally inconsistent, or factually wrong. One finding, `R5-1`, below. Everything else held:

- `:38`'s "this one has NO whole-file drift-guard test" and `:30`'s new "This is the one part of the currency work no test enforces" agree rather than duplicate: two of the three deployed files are guarded by `src/agents_md_drift.rs`, the prompt copy is not.
- `:42`'s "Closing the unguarded gap on `.agents/prompts/` is scheduled separately as `prompt-drift-guard` (order 92); this step must not wait on it" is consistent with the new `:30` manual criterion (a hand check now, an automated guard later) and with `prompt-drift-guard.md:25`'s rewritten independence claim.
- The `Q-69` `ask` is untouched by the fix apart from the one space, and nothing the fix changed bears on it. Its "in each of the two review rounds it existed" is scoped to "an earlier draft of this item", that is the option-carrying draft that existed for rounds 2 and 3, so round 4's zero-premise-defect result does not make it stale.
- I re-verified the `Q-69` citations that could rot: `pack/AGENTS.md:45`, `:65`, `:71`, `pack/user-prompts/explore.md:3`, `:7`, `:13`, `pack/pack.toml:166-167`, `src/manifest.rs:615`, and `src/isolation_policy.rs:33`. All resolve to the quoted text. The const's closing sentence is "The only edits made directly on main are the orchestrator's own integration-level ones ...: flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor" (four items), and `grep -in "question" src/isolation_policy.rs` returns nothing, which is the claim `plan.toml:1724` makes.

---

## `R5-1` (`low`): the new "not the trailing clause" requirement at `:28` was not reconciled with `:34`'s "copy the pointing"

**Files and lines.** `docs/plans/agent-scaffold.steps/decision-folder-currency.md:28` against `:34`; projected at `docs/plans/agent-scaffold.md:1219` against `:1225`.

**The two passages, verbatim from this worktree.**

`:28` (ADDED by the round-4 fix commit `72ca2d7`), in the "the requirements are these" list:

> - The checkpoint paragraph ends up saying what the MAIN clause of `pack/AGENTS.md:71` says, that is the "Here 'updates this queue' means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job" half, and NOT its trailing rationale clause after the colon. That trailing clause is one side of the live `Q-69` contradiction (see "Deliberately out of scope" below), so reproducing it here would put a second instance of it into `pack/prompts/orchestrator.md`, two paragraphs above the branch-3 sentence it contradicts, and enlarge what that design pass has to fix.

`:34` (UNCHANGED by the fix), about the same passage of the same file:

> The checkpoint paragraph is where this bites hardest, because its `pack/AGENTS.md:71` model ends by POINTING at the fragment ("the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits, which author no reviewed product content, so a `[[question]]` or `[[step]]` (reviewed product content) is not among them"): copy the pointing, not the list.

**The defect.** "The pointing", quoted verbatim inside `:34`, IS `pack/AGENTS.md:71`'s trailing rationale clause after the colon, character for character. So for one and the same passage, `:34` says "copy the pointing" and `:28` says do not reproduce it. `:34` is not a general observation that happens to brush the subject: it names the checkpoint paragraph explicitly ("The checkpoint paragraph is where this bites hardest"), and its containing paragraph is an instruction to the implementer, not commentary ("This step therefore restates NONE of that list either", "Where a passage benefits from the connection, CROSS-REFERENCE the fragment").

**Why this is fix-induced and not pre-existing.** Before `72ca2d7`, the requirement sentence said only "that the checkpoint paragraph ends up saying what `pack/AGENTS.md:71` says", with no main/trailing split (see `git diff b5ddb52..72ca2d7 -- docs/plans/agent-scaffold.steps/decision-folder-currency.md`, the removed line). Under that wording, "says what `:71` says" INCLUDED the trailing clause and `:34`'s "copy the pointing" agreed with it. The fix narrowed one of the pair and left the other alone, and the two now point opposite ways. This is the exact class the round-4 triager predicted would cost round 5.

**Why it is `low` and not higher.** Three things bound it.

- Both readings still forbid the enumeration, which is the thing step 89's finding F1, `T-4` and Principle 8 actually protect. Neither reading can produce a restatement of the generated list.
- `:28` is explicit, emphatic and states its own reason, whereas `:34`'s "copy the pointing" is a subordinate clause in a paragraph whose thrust is a prohibition on the list. A careful implementer resolves toward `:28`.
- The worst outcome is one clause in `pack/prompts/orchestrator.md` that `Q-69`'s design pass has to fix along with the three passages it already owns. Reversible prose, visible in the diff, and the work review has a fair chance of catching it.

**Why it is not a re-raise of `T-5`.** `T-5` was DISMISSED on the claim that "copy the pointing, not the list" is non-executable in a prompt that has no fragment. That is a different claim and I found no evidence against its verdict. My claim is that `:34` now conflicts with text that did not exist when `T-5` was ruled. Likewise, the round-4 triager's decision not to raise the trailing-clause risk as a separate finding rested on `:19` spelling out the main clause only and on the round-1 FORM reading of "copy the pointing, not the list"; neither of those addresses a conflict with `:28`, because `:28` did not exist yet. On the round-1 FORM reading the conflict softens to "`:34` now holds up as the model for this passage a clause `:28` forbids for this passage", which is still a statement a reader has to do work to reconcile.

**What a fix would be.** One clause on `:34`, for example noting that for the checkpoint paragraph the pointing is itself out of scope per the requirement above, so the instruction bites only where a passage does carry the connection. It costs one sentence and the re-render happens anyway. A triager could also reasonably rule this an accepted residual on the ground that `:28` dominates on any careful read; I would not object to that verdict, and it should not on its own justify another round.

---

## Settled items: not disturbed

- **Step 92 frozen except the two currency sentences.** CONFIRMED mechanically: `git diff b5ddb52..72ca2d7 --numstat` reports `2 2 docs/plans/agent-scaffold.steps/prompt-drift-guard.md`, that is exactly two lines changed, and the diff shows they are `:23` and `:25`, the two sentences `R4-1` named. No content change.
- **`H4-3` (new accepted residual, the derived-from-manifest guard silently dropping a removed prompt).** Not closed, not mentioned, and not required to be: `prompt-drift-guard.md:19` and `:21` are unchanged. I reached the mechanism independently while reading `:19` and I am NOT raising it, per the ledger rule.
- **`T-4` (accepted residual, the four-item parenthetical paraphrasing `ISOLATION_POLICY_FRAGMENT`).** Present and unchanged at `decision-folder-currency.md:34`: "(a step's status flip, an increment declaration, a round record, and the ledger's resume anchor)". Not re-raised. I confirmed the fix added no second instance of the class anywhere in the artifact.
- **`R2-3` (accepted residual, "the three actor-less `pack/AGENTS.md` prose points" in the `Q-67` ask).** Present and unchanged at `plan.toml:1706`. `Q-67` is untouched by the fix commit, which changed exactly one line of `plan.toml` (`1722`). Not re-raised.
- **`T-5` and `T-7` (dismissed).** Not reopened. I found no evidence against either verdict. `R5-1` is expressly not a `T-5` re-raise, for the reason given above.
- **`Q-69`'s elided quotation of the generated fragment** (operative clause plus item count, no reproduction of the four items). Deliberate and correct. I re-verified the count is four against `src/isolation_policy.rs:33` and that the enumeration appears nowhere in the `Q-69` `ask` or its projection.
- **The `order` gap at 91.** Intentional; `validate` does not object.

---

## Mechanical checks, run in this worktree

```
$ cargo run --quiet -- render --check docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.plan.toml: up to date

$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 213 records, valid
docs/plans/agent-scaffold.plan.toml: 91 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

Both exit 0. 91 steps, 69 questions, 213 records, as expected. The generated view is a current projection of the source, so the four projected lines the fix touched are in sync.

---

## Severity roll-up

- `critical`: none.
- `high`: none.
- `medium`: none.
- `low`: one, `R5-1`.
