# Plan review: `decision-folder-currency` (step 90) - reviewer lens: consistency and single-sourcing

Artifact: the planner fold `8d12264..5bafb42` (new `[[step]]` at order 90, its sidecar `docs/plans/agent-scaffold.steps/decision-folder-currency.md`, and the regenerated `docs/plans/agent-scaffold.md`).
Reviewer worktree: `.claude/worktrees/rev-dfc-consistency` at `5bafb42`.

4 findings: 2 medium, 2 low. No high or critical findings.

---

## CON-1: the `Q-67` question record still says the work is `pack/AGENTS.md`-only, contradicting the step it now justifies

Severity: medium

### Evidence

The new step's ONLY provenance link is `Q-67`:

```
$ grep -n 'slug = "decision-folder-currency"' -A 12 docs/plans/agent-scaffold.plan.toml
1240:slug = "decision-folder-currency"
1241:title = "name the PLANNER as the folder at the four remaining actor-ambiguous decision-folding points (the checkpoint / queue-push, Socratic-mode, and ledger paragraphs of `pack/prompts/orchestrator.md`, and the ledger paragraph of `pack/AGENTS.md`), and regenerate the deployed copies (`Q-67`)"
...
1248:[step.provenance]
1249:decisions = ["Q-67"]
```

The `Q-67` record itself is unchanged by this diff (`git diff 8d12264..5bafb42 -- docs/plans/agent-scaffold.plan.toml` touches only the new `[[step]]` block) and still reads, at `docs/plans/agent-scaffold.plan.toml:1696`:

> "this pass restates none of that list and edits only the three actor-less `pack/AGENTS.md` prose points"

and, later in the same `ask`:

> "The work this schedules edits `pack/AGENTS.md` only (guidance, no prompt or source change), then regenerates the deployed `AGENTS.md` and `.agents/AGENTS.reference.md` via `just scaffold-self`."

Step 90 schedules three edits in `pack/prompts/orchestrator.md` (sidecar lines 14, 15, 16) and the regeneration of a third deployed file, `.agents/prompts/orchestrator.md` (sidecar line 25). `docs/plans/agent-scaffold.plan.toml:1694` also still reads `folded_into = "planner-folds-decisions"`.

### Why it matters

The `[[question]]` entry is the plan's durable single source for a decision; `AGENTS.md:63` makes it the thing that stops relitigation, and `AGENTS.md:106` makes "a decision by `q_id`" a citable durable handle for the task-entry re-grounding brief. A reader who re-grounds step 90 from `Q-67` (exactly the procedure `AGENTS.md:106` prescribes) is told the decision authorises `pack/AGENTS.md` edits only, "no prompt or source change" - which makes three quarters of step 90 read as scope creep against its own provenance. This is the same class of defect the step exists to fix: a rule stated in one place and contradicted in another.

Nothing mechanical catches it: `render --check` and `validate --source --workflow` both pass (see the clean list below), because the `ask` is free prose and `folded_into` is a single `Option<String>` (`src/plan/source.rs:317`), so the schema cannot even express "folded into two steps".

The step's own "Documentation currency" section (sidecar line 23) enumerates the three deployed pack copies but omits the plan's own decision record, and the planner owns the plan, so this was fixable in the fold itself rather than schedulable.

### Suggested fix

Append a scope addendum to the `Q-67` `ask` in the same fold, for example: "SCOPE WIDENED (human, 2026-07-27): the same naming is owed at the three actor-ambiguous points in `pack/prompts/orchestrator.md` and at the ledger paragraph of `pack/AGENTS.md`; scheduled as `decision-folder-currency` (step 90), which also regenerates `.agents/prompts/orchestrator.md`." Correct or qualify the "edits `pack/AGENTS.md` only (guidance, no prompt or source change)" sentence so it describes step 89's pass rather than the decision's whole scope, and point `folded_into`'s reader at the second step.

---

## CON-2: the sweep misses the exploration-mode passages, which do not merely omit the actor - they assign `[[question]]` authoring to the ORCHESTRATOR, contradicting the clause step 89 just added

Severity: medium

### Evidence

Step 89 added this to `pack/AGENTS.md:71` (verbatim, current text):

> Here "updates this queue" means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job (routed as above): the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits, which author no reviewed product content, so a `[[question]]` or `[[step]]` (reviewed product content) is not among them.

The final clause is unqualified: a `[[question]]` is reviewed product content and is **not among** the orchestrator's direct-on-main edits. Three shipped passages instruct the orchestrator to author one anyway:

```
$ grep -rno "records the question[^.]*\|record it as an [^;]*\|record this as an [^,]*" pack/
pack/AGENTS.md:45:records the question as an Open-Questions item with status `exploring` (a design pass is owed), spawns one or more explorers ...
pack/prompts/orchestrator.md:31:record it as an `exploring` Open-Questions item, spawn one or more explorers to write a design-notes artifact, ...
pack/user-prompts/explore.md:13:record this as an `exploring` open question
```

- `pack/AGENTS.md:45`: "Instead of presenting options straight away, **the orchestrator records the question as an Open-Questions item with status `exploring`**".
- `pack/prompts/orchestrator.md:31`: the final sentence is a three-branch instruction to the orchestrator ("pick the mode it needs"), and its THIRD branch is "for one whose design space is not yet decidable ..., **record it as an `exploring` Open-Questions item**". The SECOND branch of that same sentence ("for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision") is precisely the passage step 90 targets (sidecar line 8, line 15).
- `pack/user-prompts/explore.md:13`, under "Act as the orchestrator" (line 7): "**record this as an `exploring` open question**". `explore.md:3` restates it: "The agent records the question".

The sidecar's completeness claim (sidecar line 3): "FOUR passages still leave the actor unnamed, three of them in `pack/prompts/orchestrator.md` and one in `pack/AGENTS.md`", and its "Deliberately out of scope" list (sidecar line 33) records only `pack/LEDGER.template.md:3`. The exploration-mode passages appear in neither list.

### Why it matters

Two distinct problems, both worse than the omissions the step does target:

1. These passages are not incomplete, they are **inconsistent**. The sidecar's own framing (line 3) is "None of them contradicts the now-explicit rule; they are incomplete". That is true of the four in scope and false of these three, so the step's diagnosis of the remaining surface is wrong, not just short.
2. The implementer will be editing the very sentence that contains one of them. Carrying out sidecar line 15 on `pack/prompts/orchestrator.md:31` produces a single sentence whose second branch says the planner authors the fold and whose third branch tells the orchestrator to author a `[[question]]` itself. A prompt that states the actor boundary two ways in one sentence is a new drift source, which is the failure mode the step is meant to close.

Whether the right resolution is to widen the step, to qualify `pack/AGENTS.md:71`'s "not among them" so it scopes to a DECIDED item, or to record a deliberate exclusion is a genuine design call for the human. What is not defensible is the current state: the step asserts the remaining set is four and does not mention these at all.

### Suggested fix

Either (a) add the exploration-mode passages to the step (and add the fourth deployed copy `.agents/user-prompts/explore.md` to the documentation-currency list, since `pack/user-prompts/explore.md` is a scaffolded asset - `pack/pack.toml:166-167` maps it to `.agents/user-prompts/explore.md`, and `src/manifest.rs:615` lists that dest), or (b) put the tension to the human per the human-input contract and record the outcome in the "Deliberately out of scope" list with its reasoning, the same treatment `pack/LEDGER.template.md:3` already gets. Do not leave it unmentioned.

---

## CON-3: the sidecar restates the generated closed list in the same paragraph that forbids restating it

Severity: low

### Evidence

`ISOLATION_POLICY_FRAGMENT` (`src/isolation_policy.rs:33`) ends:

> "... flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor."

`docs/plans/agent-scaffold.steps/decision-folder-currency.md:21` opens:

> "The orchestrator's closed list of direct-on-main integration edits **(a step's status flip, an increment declaration, a round record, and the ledger's resume anchor)** is NOT hand-authored prose: it is the generated `ISOLATION_POLICY_FRAGMENT` (`src/isolation_policy.rs`) ..."

and two sentences later:

> "This step therefore restates NONE of that list either ... Restating a generated single source in hand prose would violate the AGENTS.md workflow guidance 'One source of truth' and plan Principle 8".

The parenthetical is a nominalised paraphrase of all four generated items, in the generated order. The model the same paragraph holds up, `pack/AGENTS.md:71`, deliberately does NOT enumerate; it points ("the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits"). Nothing guards the plan prose: `render --check` and `cargo test` pass with the paraphrase present, so if the fragment ever gains a fifth item, this copy goes stale silently.

Disclosure, so the triager can weigh it fairly: this is **established precedent**, not a new habit. The identical parenthetical is already committed at `docs/plans/agent-scaffold.steps/planner-folds-decisions.md` (the "Single source of the closed list" paragraph) and inside the `Q-67` `ask` at `docs/plans/agent-scaffold.plan.toml:1696`, and step 89's plan review (whose F1 dropped a scheduled restatement in the PACK prose) did not object to the sidecar's own copy. A reasonable verdict is "accepted convention"; I raise it because the new instance is self-contradictory on its face and because this reviewer's lens is exactly single-sourcing.

### Suggested fix

Drop the parenthetical and let the pointer carry it: "The orchestrator's closed list of direct-on-main integration edits is NOT hand-authored prose: it is the generated `ISOLATION_POLICY_FRAGMENT` (`src/isolation_policy.rs:33`), rendered into the `{{isolation_policy}}` slot at `pack/AGENTS.md:91` ...". The implementer can read the four items from the cited source, which is the behaviour the paragraph is asking for.

---

## CON-4: the step tells the implementer to regenerate via `just scaffold-self`, which runs the repo-wide formatter the file-safety rule forbids implementers from running

Severity: low

### Evidence

Sidecar line 23:

> "Documentation currency (which deployed files this makes stale). `pack/` is the source; **`just scaffold-self` regenerates the deployed copies and then runs `nix fmt`**. Three deployed files go stale and **must be regenerated as part of this step**, not afterwards"

`justfile:46-48`:

```
scaffold-self:
	{{ direnv_prefix }} cargo run -- scaffold --output-dir . --write --force --principles default --instrument
	{{ direnv_prefix }} nix fmt
```

`pack/AGENTS.md:79` (and its deployed copy `AGENTS.md:79`), the "File safety and durability" rules:

> "Format only your own files. An implementer formats only the files it changed; **it must not run repo-wide formatters (for example `just fmt` or `nix fmt`)** ... and leaves incidental reformatting to the orchestrator."

`pack/prompts/orchestrator.md:9` assigns the complementary duty to the orchestrator: "Incidental reformatting is yours: after a writer finishes you may run the repo-wide formatter to normalise drift it left".

The formatter step is also unnecessary for the guard the sidecar relies on: `src/agents_md_drift.rs` normalises prettier's wrapping on both sides before comparing (`normalize_wrapping`, and the module doc note that "at the time this guard was written the raw render is already byte-identical to both committed files"), so the raw `scaffold` render alone satisfies the drift guard.

I did not verify whether the tree is currently formatter-clean, because running the formatter is out of bounds for this review; the finding rests on the rule conflict alone, which the citations settle.

### Why it matters

The step, as written, directs a writer agent to do the one thing the file-safety rules single out as a writer's boundary violation, and the split of duties (`AGENTS.md:79` plus `pack/prompts/orchestrator.md:9`) exists precisely so an implementer's diff stays scoped to the files it owns. Step 89's sidecar has the same wording, so this too is precedent rather than a novelty; it is worth correcting once rather than propagating a third time.

### Suggested fix

Reword the documentation-currency instruction to name the implementer's half only, for example: "the implementer regenerates the three deployed copies with `cargo run -- scaffold --output-dir . --write --force --principles default --instrument` (the render half of `just scaffold-self`) and leaves the repo-wide `nix fmt` to the orchestrator, per the 'Format only your own files' rule in `AGENTS.md`."

---

## Checked and clean (non-findings)

Recorded so a later round does not redo them.

**Single-sourcing against the four generated fragments.** Read `src/isolation_policy.rs`, `src/recommendation_rule.rs`, `src/workflow_spec.rs` (`control_fragment`), `src/findings_naming.rs`. The step schedules NO restatement of any of them in the pack. `RECOMMENDATION_RULE_FRAGMENT`, `control_fragment`, and the findings-naming fragment are untouched and unquoted by the step. The one restatement is in the plan sidecar itself (CON-3), not in the scheduled pack prose. The sidecar's instruction "copy the pointing, not the list" is followed one sentence later by "The prompt may reference the rule in `AGENTS.md` rather than reproduce the fragment's contents", which resolves the otherwise-dangling "the ... fragment below" pointer for a document that has no `{{isolation_policy}}` slot; `pack/prompts/orchestrator.md:13` already establishes that referencing convention, so an implementer has a model. Not a finding.

**Would the planned change contradict the already-shipped guidance?** Read the four targets (`pack/prompts/orchestrator.md:27,31,33` and `pack/AGENTS.md:63`) against their twins (`pack/AGENTS.md:41,43,71`). For the four passages in scope the instruction is "match the guidance's existing clause, do not invent a different rule", and the twins say a mutually consistent thing. No contradiction, only the exploration-mode adjacency raised as CON-2.

**Documentation-impact list.** Verified complete for the four targets. `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` -> identical (so the deployed copy is verbatim, as claimed); `diff AGENTS.md .agents/AGENTS.reference.md` -> identical. `grep -rn "only durable decisions" .` and `grep -rn "update the plan's Open Questions queue" .` find no fourth deployed copy of either sentence, and no test fixture embeds the prompt prose.

**The drift-guard claim, verified mechanically rather than read.**

```
$ grep -rn "include_str!\|include_bytes!\|include_dir!" src/ tests/ build.rs
```

Every content guard embeds only `../AGENTS.md` and `../.agents/AGENTS.reference.md` (`agents_md_drift.rs:45,49`; `isolation_policy.rs:41,45`; `recommendation_rule.rs:42,46`; `workflow_spec.rs:211,215`; `findings_naming.rs:92,96`). `src/agents_md_drift.rs:299-300` re-renders exactly those two dests. No test embeds or compares any `.agents/prompts/*` file: the only occurrence of `.agents/prompts/orchestrator.md` in `src/` outside `pack.toml` is `src/manifest.rs:604`, which is a dest-LIST assertion (`assert_eq!(dests, vec![...])`), not a content check. The sidecar's claim is correct in both directions. Note the same is true of `.agents/LEDGER.template.md` (`src/manifest.rs:612` is likewise a dest-list entry only).

**The `pack/LEDGER.template.md:3` exclusion.** Its primary reason is sound: "Durable decisions do not live here; they fold into the plan" is a note about what the ledger file is FOR, not a statement of the folding duty, so naming an actor there adds a restatement where the actor is irrelevant. Its SECONDARY reason ("would drag a fourth deployed asset ... into the regeneration set for no gain") is weak, since `.agents/LEDGER.template.md` is no more or less guarded than `.agents/prompts/orchestrator.md`, which the step already accepts into the regeneration set; but the exclusion stands on the primary reason and is human-confirmed. Not raised as a finding.

**Plan-internal consistency.** `order = 90` is unique (`grep -n "order = " docs/plans/agent-scaffold.plan.toml | sort -t= -k2 -n` shows 71..90 with no duplicate). No overlap with step 89: `planner-folds-decisions.md` states it covers `pack/AGENTS.md` lines 41, 43, 71 and "edits no prompt file and no source", while step 90 covers `pack/prompts/orchestrator.md:27,31,33` and `pack/AGENTS.md:63` - disjoint. No conflict with the deferred `workflow-toml-rule-fragments` (83), whose own rationale is that "prose remains the cleaner substrate for the still-evolving logic" until its evidence gate clears, which is exactly what step 90 does; and none with `rename-to-agent-flow` (84), a release-timed rename. Step 88 (`reviewer-reproducible-evidence`, `Q-66`) is unrelated in surface.

**Generated view versus TOML source.**

```
$ cargo run -- render --check docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.plan.toml: up to date        (exit 0)
$ cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 209 records, valid
docs/plans/agent-scaffold.plan.toml: 90 steps, 68 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold   (exit 0)
$ cargo test
366 + 5 + 1 + 3 + 1 + 2 passed; 0 failed
```

The regenerated Status line's arithmetic checks out: 2 + 2 + 60 + 4 + 1 + 3 + 18 = 90, and the per-status counts match `grep -n 'status = ' docs/plans/agent-scaffold.plan.toml | awk -F'"' '{print $2}' | sort | uniq -c`. `status = "next"` is a valid `StepStatus` variant (`src/plan/source.rs:183`, "Queued next").

**TOML `title` versus sidecar `###` heading.** They differ, but that is the plan's established convention (checked against `reviewer-reproducible-evidence`, `planner-folds-decisions`, and `code-value-audit-static`: every one pairs a long TOML title with a short sidecar heading). Not drift.

**Other actor-less folding language considered and cleared.** `pack/AGENTS.md:39` ("such a request may be folded in directly") is passive, but its paragraph names the orchestrator at line 37 and its prompt counterpart `pack/prompts/orchestrator.md:29` addresses the orchestrator directly ("Fold a trivial request ... in directly; route anything non-trivial to the planner"), so the actor is not in doubt. `pack/prompts/open-questions-gate.md:7` ("add them to the plan's ... section") is the PLANNER's gate prompt (`AGENTS.md:20`), so the actor is the reader. `pack/prompts/planner.md:7` and `pack/AGENTS.md:30` both name the planner. `README.md:92` ("Orchestrator folds it in directly") is the trivial-intake branch, consistent with `pack/AGENTS.md:39`. `pack/plan-template.plan.toml:11` ("every decision it later folds in") refers to the project, not a role. `pack/user-prompts/pause.md` and `compaction-prep.md` are thin triggers that reference `AGENTS.md` rather than restating it.

**Principle citations in the sidecar.** Checked against `[[principle]]` in the TOML: Principle 1 "Prefer the cleaner long-term architecture over the smallest diff", Principle 2 "Minimal by default", Principle 8 "Structured data first, project for humans" all cited by their correct plan numbers and names.

**No new `[[question]]` and no receipt.** The reasoning at sidecar line 37 holds: W4 keys on a decided `[[question]]` past `[meta].w4_baseline`, this step registers none, and `validate --workflow` passes (above). Correct.
