# Round 2 review: fresh eyes on the never-reviewed content (`Q-69`, step 91, step 92)

Reviewer: independent, round 2, fresh-eyes lens. Target: the content added by `f8b3cdc` that no reviewer has seen, namely the new `[[question]] Q-69`, the new step 91 (`exploring-item-actor-boundary`) with its sidecar, and the new step 92 (`prompt-drift-guard`) with its sidecar. Round-1 fix verification belongs to the parallel reviewer and is not covered here.

Diff range inspected: `git diff 0905620..f8b3cdc`. All commands below were run from the review worktree at `f8b3cdc`; every `file:line` was re-read immediately before being cited.

Outcome: 4 findings (1 high, 1 medium, 2 low). No critical finding.

---

## NEW-1: the `Q-69` ask's "live evidence" claim about `b6ba317` is unsupported by the commit shape and is contradicted by the project's own ledger

Severity: `high`.

Location: `docs/plans/agent-scaffold.plan.toml:1733` (the `Q-69` `ask`), with a weaker restatement at `docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md:5`.

The claim. `Q-69`'s ask states:

> THE AMBIGUITY IS LIVE, not theoretical: commit `b6ba317` ("docs: capture Q-68 exploring backlog item for structured-first ledger") is a plain single-parent commit on main that added `[[question]] id = "Q-68"`, `status = "exploring"` straight into `docs/plans/agent-scaffold.plan.toml` with no planner branch and no review round. That is exactly what `pack/AGENTS.md:45` prescribes and exactly what `:71` reads as excluding.

It is load-bearing twice more in the same item: option (b)'s trade-offs say the heavier rule "would likely be honoured in the breach, exactly as `b6ba317` already shows", and the recommendation closes "the evidence of `b6ba317` suggests the heavier rule would simply not be followed."

Two independent problems.

(1) The commit shape cannot establish "no planner branch". This repo fast-forwards converged worktree branches onto main, which produces exactly a plain single-parent commit. Three commits the ledger explicitly records as fast-forward merges of worktree branches that went through full review rounds are all single-parent:

```
$ git log -1 --format='%h parents=%p %s' 557fa46
557fa46 parents=e8f458c docs: require reviewer findings to carry reproducible evidence (Q-66)
$ git log -1 --format='%h parents=%p %s' 4f48283
4f48283 parents=dc9686a docs: name the planner as folder of non-trivial decided decisions (Q-67)
$ git log -1 --format='%h parents=%p %s' cca1099
cca1099 parents=44f848a docs: apply Q-66/Q-67 plan-review round 1 fixes (F1 F2 F3 F-fid)
```

`docs/plans/agent-scaffold.ledger.md:355` records these same three as "ff-merged `557fa46`", "STEP 89 COMPLETE (ff `4f48283`)", and "PLAN FOLD MERGED (ff `cca1099`)", each after an isolated writer plus one or more review rounds. Single-parentness therefore carries zero information about whether a branch was used, so the inference in the ask does not follow from the evidence it cites.

(2) The project's own record says the opposite. `docs/plans/agent-scaffold.ledger.md:355` reads:

```
$ grep -n -o "NEW BACKLOG (captured 2026-07-26 by a planner, per Q-67): \`Q-68\` (\`exploring\`, DESIGN PASS OWED)" docs/plans/agent-scaffold.ledger.md
355:NEW BACKLOG (captured 2026-07-26 by a planner, per Q-67): `Q-68` (`exploring`, DESIGN PASS OWED)
```

That ledger text was written by the commit immediately after `b6ba317` (`git show 8d12264 -- docs/plans/agent-scaffold.ledger.md`), so it is the contemporaneous record of the same event. It says a PLANNER captured `Q-68`, and did so "per Q-67", meaning the actor rule was consciously applied rather than breached.

Why this matters rather than being a nit. This is the only empirical evidence in the item, it is offered under the heading "THE AMBIGUITY IS LIVE, not theoretical", and it is used twice to argue against option (b). If the ledger is right, the observed practice already IS option (b) (a planner authored the placeholder), which is the reverse of the inference drawn. The human is being asked to pick a workflow rule partly on the strength of a claim about their own project's behaviour that the project's own record refutes.

Provenance note, so this is not read as re-raising a settled finding. The claim originated with the step-90 plan-review triager (`docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-triage.md:76` and `:177`, which assert the same "no planner branch and no review round"). I am not re-opening T-3a's verdict, which was about the contradiction and is sound. The new evidence is `ledger.md:355`, which neither the reviewers nor the triager cited, and the new artifact is `Q-69`, which promoted an unverified upstream claim into the plan of record.

What the fix must achieve. Either drop the "no planner branch and no review round" assertion and keep only what is verifiable (a `[[question]]` with `status = "exploring"` was added to the plan on main, and no round record exists for it: `grep -c "Q-68" docs/metrics/workflow.jsonl` returns `0`), or reconcile the claim against `ledger.md:355` and state which record is authoritative. If the ledger is right, option (b)'s "honoured in the breach" trade-off and the recommendation's closing sentence both need rewriting, because the one datum cited against (b) becomes a datum for it.

---

## NEW-2: `Q-69` and step 91 say `pack/AGENTS.md:71` states the exclusion "without qualification", omitting the qualifier in the same sentence

Severity: `medium`.

Locations, all new in `f8b3cdc`:

- `docs/plans/agent-scaffold.plan.toml:1733` (`Q-69` ask): "the actor boundary `Q-67` added at `pack/AGENTS.md:71` is stated without qualification".
- `docs/plans/agent-scaffold.plan.toml:1254` (step 91 `title`): "`pack/AGENTS.md:71` excludes a `[[question]]` from the orchestrator's direct-on-main edits without qualification".
- `docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md:5`: "`pack/AGENTS.md:71` says without qualification that ...".
- Same wording also lands at `docs/plans/agent-scaffold.steps/decision-folder-currency.md:40` (the T-3b fix, which is the parallel reviewer's territory; listed only so the triager sees every instance).

The evidence. `pack/AGENTS.md:71`'s relevant sentence, read in full:

```
$ awk 'NR==71' pack/AGENTS.md
```

> Here "updates this queue" means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job (routed as above): the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits, which author no reviewed product content, so a `[[question]]` or `[[step]]` (reviewed product content) is not among them.

The quoted trailing clause is verbatim and is genuinely unqualified on its own. But the subject of the sentence is "authoring a DECIDED DECISION'S `[[question]]` or `[[step]]` FOLD", and `Q-69`'s quotation begins after that clause with no ellipsis and no mention of it. The guidance's other authoritative statement of the same rule is scoped the same way: `pack/AGENTS.md:41` says "when that fold is non-trivial (authoring a `[[question]]` or a `[[step]]`), the orchestrator routes it to the planner to author", of "a resolved decision".

Why this is material and not a wording quibble. `Q-69`'s recommended option (a) is stated as "QUALIFY `pack/AGENTS.md:71` so its exclusion scopes to a DECIDED question's fold", and step 91's sidecar renders it as "Narrow its closing clause so the exclusion scopes to a DECIDED question's fold". The sentence's main clause already scopes it to a decided decision's fold. So the real defect is narrower than presented: a trailing rationale clause over-generalises past the subject of its own sentence. Presenting the rule as flatly unqualified inflates the contradiction, makes option (a) look like a rule change when it is close to a clarification of a clause that already carries the intended scope, and correspondingly inflates the cost of option (b) relative to the status quo.

Internal inconsistency inside the same commit. `docs/plans/agent-scaffold.steps/decision-folder-currency.md:12`, added by the same planner in the same commit, quotes the qualifier correctly: "Its `pack/AGENTS.md:71` counterpart already carries the step-89 qualifier (\"Here 'updates this queue' means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job\")". So one artifact in the commit knows the qualifier exists and three others say there is none.

What the fix must achieve. State the defect precisely: `pack/AGENTS.md:71`'s closing rationale clause generalises past its own sentence's subject (a decided decision's fold) and so reads as excluding any `[[question]]`, including an `exploring` one. Then re-describe option (a) against that accurate baseline, and re-check whether the (a) versus (b) cost comparison still holds once (a) is understood as tightening one clause rather than adding a scope the guidance lacks.

---

## NEW-3: the `Q-69` reasoning never names Principle 8, the plan's declared tie-breaker over the Principle it leans on

Severity: `low`.

Location: `docs/plans/agent-scaffold.plan.toml:1733`, the RECOMMENDATION paragraph.

The recommendation names Principle 1, Principle 2, and Principle 5, and each name matches its number in the plan's eight `[[principle]]` entries (verified below in "Verified clean"). The gap is which principle is absent. Option (c) is argued on exactly one-source-of-truth grounds ("adds no new rule and keeps ONE criterion, which is the strongest one-source-of-truth answer"), and Principle 8's text (`docs/plans/agent-scaffold.plan.toml:1784-1785`) is the plan's home for that reasoning: it ends "it sharpens Principle 1 (cleaner long-term architecture) and Principle 16-equivalent one-source-of-truth thinking", and it states "when this conflicts with Principle 2 (minimal) or Principle 3 (safe on existing projects) at this stage, this wins".

That matters here because the recommendation rejects option (b) partly on Principle 2 and rejects option (c) on a Principle 5 analogue, and Principle 8 is the one principle whose text both speaks directly to (c)'s argument and explicitly outranks Principle 2 at this stage. The project's established usage supports applying Principle 8 to prose single-sourcing, not only to data formats: the parent `Q-67` ask cites it that way (`docs/plans/agent-scaffold.plan.toml:1719`), and step 91's own sidecar cites it that way ("it serves the AGENTS.md workflow guidance \"One source of truth\" and plan Principle 8", `docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md:15`).

This is not a contract violation. `pack/AGENTS.md:41` renders `RECOMMENDATION_RULE_FRAGMENT` (`src/recommendation_rule.rs:34`), which requires "the reasoning judged against the plan's Project Principles by name", and three named principles satisfy that. It is a completeness gap in the argument put to the human.

What the fix must achieve. Address Principle 8 explicitly in the (a) versus (c) comparison, including its precedence clause over Principle 2, so the human sees why the recommendation still lands on (a) with the plan's own tie-breaker on the table.

---

## NEW-4: the option set omits the "do not put the placeholder in the plan's `[[question]]` array at all" option

Severity: `low`.

Location: `docs/plans/agent-scaffold.plan.toml:1733`, the OPTIONS block.

Options (a), (b), and (c) all take as given that an `exploring` placeholder is a `[[question]]` in `<task>.plan.toml`, and then argue about who may author it. A fourth option is not considered: keep `pack/AGENTS.md:71`'s flat rule exactly as written and stop putting the undecided placeholder in the plan's question array, recording it instead in the orchestrator-owned ledger queue until the exploration converges, at which point the planner folds the resulting `open` question into the plan.

Evidence that this is inside the design space rather than invented for the review:

- The project already draws the ledger-versus-plan line for exactly this reason, at `pack/prompts/orchestrator.md:33`: "The ledger is separate from the plan: do not put individual findings in the plan's Open Questions section; only durable decisions, the ones that change the plan, fold into it."
- The `Q-68` capture the ask cites in fact used BOTH homes: `b6ba317` wrote the `[[question]]`, and `8d12264` ("docs: record Q-68 (structured-first ledger) backlog capture in ledger queue") wrote the ledger entry at `docs/plans/agent-scaffold.ledger.md:355`. So the ledger-side placeholder already exists in practice; the open question is whether the plan-side one needs to as well.

Its trade-offs are real on both sides and would need writing out honestly: it is the only option that leaves the flat rule untouched AND adds no ceremony, but the ledger is per-task and deleted at task close (`pack/AGENTS.md:69`: "the ledger is deleted at task close"; `Q-68` at `docs/plans/agent-scaffold.plan.toml:1726` states "the ledger is per-task and DELETED at task close while `workflow.jsonl` is cross-task"), so an owed design pass could vanish with it, and `pack/AGENTS.md:45`'s stated purpose ("keeps a not-yet-decidable question visible and distinct from one merely awaiting a choice") would have to be met some other way.

What the fix must achieve. Either add the option with its trade-offs, or add one sentence saying it was considered and why it was excluded, so the human can see the boundary of the option set rather than infer it.

---

## Verified clean (checked, found correct, no finding)

Recorded explicitly so a later round does not spend effort re-deriving these.

`Q-69` citation accuracy (every one re-read at `f8b3cdc`, all verbatim):

- `pack/AGENTS.md:45`: "the orchestrator records the question as an Open-Questions item with status `exploring`". Exact.
- `pack/prompts/orchestrator.md:31`, third branch: "record it as an `exploring` Open-Questions item". Exact, and it is the third branch of the three-branch sentence.
- `pack/user-prompts/explore.md:13`: "record this as an `exploring` open question". Exact. `:7` is "Act as the orchestrator described in `.agents/prompts/orchestrator.md`", as claimed. The "(restated at `:3`)" parenthetical is loose (line 3 says "The agent records the question" and does not use the word `exploring`) but is not wrong enough to raise.
- `pack/pack.toml:166-167` maps `user-prompts/explore.md` to `.agents/user-prompts/explore.md`; `src/manifest.rs:615` lists that dest. Both exact.
- `pack/AGENTS.md:71`'s quoted trailing clause is verbatim. The problem is what surrounds it, covered by NEW-2.

`Q-69` status hygiene: `status = "open"`, no `folded_into` key, no `receipt` key (`git diff 0905620..f8b3cdc -- docs/plans/agent-scaffold.plan.toml`). `docs/plans/agent-scaffold.questions/Q-69.md` is a 0-byte sidecar, which matches every other question sidecar in the directory (`find docs/plans/agent-scaffold.questions -size +0` returns nothing but the directory itself).

Principle citations: the plan has exactly eight `[[principle]]` entries at `docs/plans/agent-scaffold.plan.toml:1747-1785`. Number-to-name mapping verified: 1 "Prefer the cleaner long-term architecture over the smallest diff", 2 "Minimal by default", 3 "Safe on existing projects", 4 "Idempotent", 5 "Make illegal states unrepresentable", 6 "Ground decisions in evidence", 7 "Reproducible", 8 "Structured data first, project for humans". `Q-69` cites 1, 2, and 5 and names each correctly; step 91's sidecar cites 8 correctly; step 92's sidecar cites 6, 8, and 1 correctly. No instance of the 22-item `AGENTS.md` numbering leaking in. I specifically checked whether Principle 2 ("The core does one thing well; everything else is an optional module the user opts into") is misapplied by being used against process ceremony rather than against tool scope, and concluded it is not a finding: the plan uses Principle 2 in that broad "do not over-build" sense throughout (for example "Minimal by default (cheap slice only)" and "do not spend measurement budget where the acute problem is fixed"). The Principle 5 citation is explicitly flagged in the ask as "read as its documentation analogue", which is honest labelling rather than a misapplication.

Human-input contract compliance: `pack/AGENTS.md:41` renders `{{recommendation_rule}}`, whose single source is `RECOMMENDATION_RULE_FRAGMENT` at `src/recommendation_rule.rs:34`: "the viable options or approaches, the trade-offs of each, a recommendation, and the reasoning, with the reasoning judged against the plan's Project Principles by name." `Q-69` supplies three labelled options, an explicit trade-offs sentence for each, an explicit "RECOMMENDATION: (a)", and Principle-named reasoning, including an explicit statement of which option is the closest rival and under what preference it would win. It also states "NOT DECIDED. The human decides per the human-input contract." The recommendation is argued, not asserted, though see NEW-1 (one of its two supporting arguments rests on a bad datum) and NEW-3.

Step 91 accuracy:

- The per-option regeneration sets are correct against the real deployed layout. `pack/pack.toml:28-29` and `:99-100` map `AGENTS.md` to both `AGENTS.md` and `.agents/AGENTS.reference.md`, so options (a) and (c) regenerate exactly those two. `pack/pack.toml:105-106` maps `prompts/orchestrator.md` to `.agents/prompts/orchestrator.md` and `:166-167` maps the explore user-prompt, so option (b)'s four-file set is right.
- The regeneration command it gives (`cargo run -- scaffold --output-dir . --write --force --principles default --instrument`) is exactly the render half of `justfile:46-48`, and its instruction to leave the repo-wide `nix fmt` to the orchestrator matches `pack/AGENTS.md:79` and `pack/AGENTS.md:108`, both quoted correctly.
- The step-90 interaction is handled correctly in both directions. Step 91 owns branch 3 of `pack/prompts/orchestrator.md:31` and only under option (b); `docs/plans/agent-scaffold.steps/decision-folder-currency.md:20` independently instructs step 90's implementer "EDIT THE SECOND BRANCH ONLY ... Leave branch 3 exactly as it is". The two sidecars agree, and step 91's claim that the boundary holds under either landing order is correct.
- Its statement that the whole-file drift guard covers `AGENTS.md` and `.agents/AGENTS.reference.md` but not the prompt or user-prompt copies is correct (`src/agents_md_drift.rs:45,49`).
- `blocked_by = []` with `status = "not-started"` is the only representable state for a step blocked on an open question rather than on another step: `src/plan/source.rs:167-190` documents that "blockedness is now the typed `blocked_by` list, so a step is never `blocked`", and there is no status for "awaiting a human decision". The sidecar carries the blockedness in prose, which is the right place given the schema.

Step 92 accuracy, including its key trap:

- THE `checks-reviewer.md` MODULE-GATING CLAIM IS CORRECT, verified three ways. `pack/pack.toml:219-223` declares the asset with `module = "checks"`, and `pack/pack.toml:11-14` declares the `checks` module; `src/manifest.rs:58-64` documents that a `Some(name)` module tag is "dropped only when that module is enabled"; `justfile:46-48` shows `scaffold-self` passes no `--module`. Empirically the file is absent from the tree: `ls .agents/prompts/` returns the seven core prompts and no `checks-reviewer.md`, while `ls pack/prompts/` returns eight files including it. The manifest's own reference-asset test list (`src/manifest.rs:604-619`) omits it, and the module-gated assertions live separately at `src/manifest.rs:658` and `:685`. So a guard that expected it WOULD fail on a correct tree, exactly as the sidecar warns. One imprecision, not raised as a finding: the sidecar attributes the gating to `src/manifest.rs`, whereas the declaration is in `pack/pack.toml:223` and `src/manifest.rs` implements the filter; both are findable from the pointer given.
- The derive-from-the-manifest proposal is feasible against the real code. `src/manifest.rs:270-279` exposes `pub struct Asset { pub dest, pub contents, ... }`, and `src/agents_md_drift.rs:58-70` already contains `self_scaffold_asset(dest)`, which calls `build_assets` under the pinned self-scaffold config and finds an asset by `dest`. Generalising it to retain every asset whose `dest` starts with `.agents/prompts/` is a small change to existing code, not new machinery. A bonus the sidecar does not claim: because that render passes no modules, the derived form excludes `checks-reviewer.md` automatically, so the trap it warns about is structurally avoided by the implementation it prefers.
- Both reuse targets exist with the names given: `normalize_wrapping` (`src/agents_md_drift.rs:232`) and `assert_no_unprotected_construct` (`src/agents_md_drift.rs:99`), and the sidecar's description of what each does matches their doc comments (`:72-98` and `:196-231`). The normalisation rationale it gives is a faithful summary of the module docs at `src/agents_md_drift.rs:12-29`.
- The precondition would NOT trip on the current prompt files, so the hedge resolves favourably. Checked directly: `grep -c '^\s*```' .agents/prompts/*.md` returns 0 for all seven, and `grep -nP '\S[ ]{2,}\S|\t|\xc2\xa0|[ ]+$' .agents/prompts/*.md` plus `grep -n '^[[:space:]]\+' .agents/prompts/*.md` return nothing, so every line is already in canonical whitespace form.
- The acceptance-shaped bullets are testable as written, including the two-way claim. An equality check between a fresh render and the committed copy fails in both directions by construction, and direction one (edit the pack, do not regenerate) really does propagate into the test binary because `build.rs` emits `cargo:rerun-if-changed` for `pack/` and every file under it, so the embedded pack re-embeds on a pack edit. Without that, direction one would have been vacuous; it is not.
- Its gap claim is accurate. `grep -rn "include_str!" src/` embeds only `../AGENTS.md`, `../.agents/AGENTS.reference.md`, `../pack/workflow.toml`, `../pack/principles.toml`, `../pack/instrument.md`, and two test fixtures; the nine-file list the sidecar gives matches. `grep -rn "\.agents/prompts" src tests` finds only the manifest's dest-list test, `src/next.rs` path construction, and the module-gated `checks-reviewer` assertions. Nothing compares a committed prompt copy to a render. Also confirmed the seven deployed prompts are currently byte-identical to their pack sources (`cmp` on each pair), so the guard would pass on a correct tree today.

Restatement of generated single sources: none in the new content. Neither new sidecar nor `Q-69` reproduces the `ISOLATION_POLICY_FRAGMENT` closed list; `grep -n "status flip, an increment declaration" docs/plans/agent-scaffold.steps/exploring-item-actor-boundary.md docs/plans/agent-scaffold.steps/prompt-drift-guard.md` returns nothing, so the round-1 T-4 accepted residual is not propagated into the new artifacts. Step 91's sidecar instead states the constraint as "do NOT restate the generated closed list ... whatever replaces that sentence must keep pointing rather than enumerate", which is correct. `Q-69`'s five-word quotation of the fragment's "author no reviewed product content" criterion is a pointer, matching the existing `Q-67` precedent, not a restatement. `Q-69`'s one-clause summary of the W4 rule is accurate against `pack/instrument.md:9`, which is a hand-authored pack file rather than a generated Rust fragment. No paraphrase of `WorkflowSpec::control_fragment` or `src/findings_naming.rs` appears in any of the three artifacts.

Duplication and overlap with existing plan entries: none. Step 92 extends step 80 (`agents-md-drift-guard`, `order = 80`, `complete`) to new files rather than redoing it, and correctly names it as the pattern's origin. Step 83 (`workflow-toml-rule-fragments`, `deferred`) is a different change (per-phase rule fragments in `workflow.toml` plus a `{{workflow_phases}}` generated section behind the `Q-64` evidence gate) and does not collide. Step 91's subject (`Q-69`) is disjoint from step 90's four passages, which the two sidecars agree on. No decided question is contradicted.

Mechanical checks, all green at `f8b3cdc`:

```
$ cargo run --quiet -- render --check --strict docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.plan.toml: up to date

$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 210 records, valid
docs/plans/agent-scaffold.plan.toml: 92 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

Order collision check: `grep '^order = ' docs/plans/agent-scaffold.plan.toml | sort | uniq -d` returns nothing across all 92 steps, and 91 and 92 are the two highest. The generated view's Status line updated consistently (`92 steps (4 not started, ...); 6 open questions`), and `render --check --strict` above confirms the projection matches the source.

Step 92 carrying no `[step.provenance]` block is correct, not an omission: it settles no question, and `validate` accepts it.

Nothing was raised on line length or prose wrapping, per the project rule.
