# Plan-review round 4 triage: step 90 `decision-folder-currency`, step 92 `prompt-drift-guard`, `Q-69`

Triager: independent of the planner that produced the artifact and of the orchestrator that drives the loop. Worktree `.claude/worktrees/triage4-dfc`, detached at `b36c4c6`, which IS the artifact (the planner branch tip), so every reviewer line citation resolves against post-fold text rather than against main.

Inputs read directly, not transcribed: `decision-folder-currency-plan-r4-reviewer-verification.md` (`R4-1`, `R4-2`, `R4-3`), `decision-folder-currency-plan-r4-reviewer-holistic.md` (`H4-1` to `H4-5`), plus the round 1, 2 and 3 triage files for the settled ledger.

Method, per `Q-66` and the triager role: I opened every cited file at every cited line in this worktree and re-ran every cited command here. I did not accept any reviewer's reproduction. Where my reading differs from a reviewer's, I say so.

## Line numbers: all resolved

Every `file:line` either reviewer cited resolved to the quoted text in this checkout. That includes the whole set: `decision-folder-currency.md` `:7`, `:15`, `:20`, `:21`, `:22`, `:24`, `:26`, `:28`, `:30`, `:34`, `:36`, `:40`, `:46`, `:50`; `prompt-drift-guard.md` `:10`, `:19`, `:23`, `:25`; `pack/AGENTS.md` `:39`, `:41`, `:43`, `:45`, `:63`, `:65`, `:71`, `:79`, `:91`, `:108`; `pack/prompts/orchestrator.md` `:27`, `:29`, `:31`, `:33`; `plan.toml` `:1243`, `:1256`, `:1717-1734` (`:1722`, `:1724`, `:1728`, `:1732`, `:1734`); `agent-scaffold.md` `:194`, `:196`, `:206`, `:208`, `:1237`, `:1267`, `:1269`; `src/agents_md_drift.rs` `:45`, `:49`, `:66-69` (the panic is on `:68`), `:99`, `:118`, `:232`, `:308-311`; `src/manifest.rs` `:604-619`; `justfile:46-48`; `src/isolation_policy.rs:33`. No misnumbering this round, which is the first round of four with none.

## Verdict summary

| id | merges | verdict | my severity | reviewer severity | evidence reproduced |
| --- | --- | --- | --- | --- | --- |
| `H4-1` | | VALID | `medium` | `medium` | Yes, in full |
| `R4-1` | `H4-2` | VALID | `low` | `low` / `medium` | Yes, in full |
| `R4-2` | | VALID | `low` | `low` | Yes, in full |
| `R4-3` | | VALID | `low` (cosmetic) | `low` | Yes, in full |
| `H4-4` | | VALID | `low` | `low` | Yes, in full |
| `H4-5` | | VALID | `low` | `low` | Yes, in full |
| `H4-3` | | VALID BUT ACCEPT RESIDUAL | `low` | `low` | Yes, with one over-read corrected |

Six valid findings to fix, all prose edits, in three files. One new accepted residual. Nothing dismissed. No high or critical raised, and none dismissed, so no backstop re-check is owed (see "Backstop" below).

## Independent mechanical re-verification

Run in this worktree under the project toolchain, not taken from either reviewer:

```
$ cargo run --quiet -- render --check docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.plan.toml: up to date

$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl --workflow
docs/metrics/workflow.jsonl: 212 records, valid
docs/plans/agent-scaffold.plan.toml: 91 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold

$ cargo test    # all suites pass, including agents_md_drift and isolation_policy
```

Artifact scope confirmed: `git diff --stat ab6c01d..b36c4c6` touches five files, all under `docs/plans/` (plan source, generated view, empty `Q-69.md`, the two step sidecars). `git diff --stat e47f4cf..b36c4c6` (the split) touches four, including the committed deletion of `exploring-item-actor-boundary.md`. No pack file, no `src/`, no prompt. Step 92's sidecar is inside the artifact range even though the split commit did not touch it, so its content is in scope for this review.

---

## `H4-1` (VALID, `medium`): the two ACTOR-LESS instructions drop the qualifier every shipped counterpart carries

Evidence reproduced: YES, every part of the chain, checked link by link rather than as a whole.

1. The four per-passage instructions are inconsistent about scope. `decision-folder-currency.md:19` (checkpoint) carries the qualifier in its `pack/AGENTS.md:71` form: "not authoring the decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job and which the orchestrator routes". `:20` (Socratic) carries it in its `pack/AGENTS.md:43` form: "make the second branch say the planner authors the non-trivial fold and the orchestrator routes it". `:21` and `:22` carry no qualifier at all: "name the planner on the "only durable decisions ... fold into it" clause" and "same, on its closing ... clause". Verbatim as the reviewer quotes them.
2. The requirement summary at `:24` does not restore it. It requires that "each passage names the actor at its point of use", that the checkpoint paragraph "ends up saying what `pack/AGENTS.md:71` says", and that the two ledger copies "end up saying the same thing". No scope clause.
3. All three shipped step-89 outputs are qualified. `pack/AGENTS.md:41`: "when that fold is non-trivial (authoring a `[[question]]` or a `[[step]]`), the orchestrator routes it to the planner to author ... rather than editing the plan directly." `:43`: "its non-trivial fold routed to the planner to author as above rather than edited in directly." `:71`: "not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job (routed as above)". Note that `:41`'s parenthetical DEFINES "non-trivial" as authoring a `[[question]]` or a `[[step]]`, so `:19`'s form and `:20`'s form are the same qualifier stated two ways. That matters for the fix: the sidecar is not inconsistent between `:19` and `:20`, only between those two and `:21`/`:22`.
4. A trivial fold really does stay with the orchestrator. `pack/AGENTS.md:39`: "A request is trivial only if it is local, reversible, changes neither the Success Criteria nor the Roadmap scope, and raises no new open question; such a request may be folded in directly." `pack/prompts/orchestrator.md:29`: "Fold a trivial request (local, reversible, no change to scope or Success Criteria, no new open question) in directly". And `:41` sets up the contrast explicitly by conditioning the routing on the fold being non-trivial.

Where I differ from the reviewer, and why the finding survives it. The reviewer's strongest claim is that a literal execution "contradicts `pack/AGENTS.md:41`". I tested that and it is contestable rather than settled: the isolation-policy fragment (`src/isolation_policy.rs:33`) confines the orchestrator's direct-on-main edits to four integration edits, none of which is authoring step prose, so on one reading no fold "into the plan's steps" was ever the orchestrator's to make and an unqualified clause would be broad rather than false. I do not need to settle that to rule, because the defect that does not depend on it is the artifact's own internal inconsistency: this sidecar's central device is a TWO-CLASS split with a different operation per class (`:5`, "the operation each needs is different, so do not treat them alike"), and inside that deliberately precise list two of four instructions state the rule's scope and two state it unscoped. That is a defect in the specification an implementer executes literally, independent of how the scope question resolves. The reviewer's own suggested fix is right even on my narrower ground.

Severity `medium`, confirming the reviewer, and I considered `low`. It is the only finding this round whose unfixed outcome reaches SHIPPED pack content (`pack/AGENTS.md` and `pack/prompts/orchestrator.md` scaffold into every project); everything else this round is plan-document currency or spec precision with no shipped consequence. The failure mode it risks is a second instance of the qualified-versus-unqualified defect that `Q-69` records at `plan.toml:1722`, in the step that exists to remove that class. It is at the bottom of the band, not the top: the outcome is reversible prose, the work review has a fair chance of catching it, and `:24`'s "these four points reinforce [`pack/AGENTS.md:41`]" is a weak counterweight in the implementer's favour.

Credit where due: this was found by asking whether an implementer could execute the step, which three prior rounds of document-level review did not ask. That is a real coverage gap closed, not a rehash.

What the fix must achieve. The four per-passage instructions must state the same scope, using the qualifier already shipped at `pack/AGENTS.md:41`/`:43`/`:71` rather than a new formulation. Either per-bullet (add the qualifier to `:21` and `:22`) or once at `:24` (one sentence saying every added actor clause carries the same non-trivial / `[[question]]`-or-`[[step]]` scope the guidance already uses) satisfies it. Two constraints on the fix:

- It must COPY the existing shipped qualifier, not invent a scope. Inventing one would decide by the back door what `Q-69` reserves for its design pass.
- It must not import `pack/AGENTS.md:71`'s TRAILING clause. See the note under "What the reviewers missed" below; the same edit closes it.

---

## `R4-1` (merges `H4-2`) (VALID, `low`): step 92's sidecar documents an interaction with a step that no longer exists

Merged ids: `R4-1` (reached by an ordinal sweep after the slug sweep came back clean) and `H4-2` (reached independently by a holistic read). Same two sentences, same file, same cause. One verdict.

Evidence reproduced: YES, both sentences, both projections, and the ordering claim.

```
$ grep -rn "steps 90 and 91\|90, then 91, then 92" docs/plans/agent-scaffold.steps/prompt-drift-guard.md docs/plans/agent-scaffold.md
prompt-drift-guard.md:23: ... which is the reason it can land independently of steps 90 and 91.
prompt-drift-guard.md:25: Interaction with steps 90 and 91. ... the order given (90, then 91, then 92) ...
agent-scaffold.md:1267: (same sentence, projected)
agent-scaffold.md:1269: (same sentence, projected)
$ grep -n "^order = " docs/plans/agent-scaffold.plan.toml | tail -3
1227:order = 89
1243:order = 90
1256:order = 92
```

There is no step 91. `plan.toml:1734` records the removal in the past tense; the ledger records the intent as "`order = 92` kept, leaving an honest gap at 91". Both stale sentences read in the present tense about a live sibling, and the second states an ordering the plan does not have.

Severity `low`, taking `R4-1`'s rating over `H4-2`'s `medium`. No instruction in step 92 is wrong, no decision rests on the two sentences, and the operative claim inside `:25` ("neither blocks the other") is still true and independently verified this round. The harm is a false statement in a committed doc that reaches the human-readable plan.

On the framing, since it decides whether the fix is allowed rather than whether the defect is real. The orchestrator told the planner to leave step 92 "exactly as it is", and the planner obeyed (`git diff e47f4cf..b36c4c6 -- prompt-drift-guard.md` is empty, which I re-ran). The freeze is what produced the staleness. It does not make the staleness acceptable, and it should not block the fix: removing references to a step that no longer exists changes no operative content and is documentation currency, which the reviewer role and `pack/AGENTS.md:33` both make a standing duty. I rule the currency edit inside the freeze. A CONTENT change to step 92 is a different matter, which is where `H4-3` lands.

What the fix must achieve. `prompt-drift-guard.md:23` and `:25` must not refer to a step that does not exist, and `:25` must not assert an ordering the plan does not have. Then re-render. Both projected lines follow from the one sidecar edit.

---

## `R4-2` (VALID, `low`): step 90's scope-history bullet says the class was given "its own question and step"

Evidence reproduced: YES.

```
$ grep -n "its own question and step" docs/plans/agent-scaffold.steps/decision-folder-currency.md
46:- The exploration-mode class was raised by the plan-review triager ... giving it its own question and step so the design gets decided rather than assumed.
$ grep -n "NO step yet" docs/plans/agent-scaffold.steps/decision-folder-currency.md
40: ... held as the queue item `Q-69`, status `exploring`, with a design pass owed and NO step yet ...
```

Three references inside one sidecar, two now saying there is no step (`:20`, `:40`, both rewritten by the split) and one still saying the human gave it one. I tested the past-tense defence the reviewer anticipated and reach the same conclusion for the same reason: the section is headed "Scope history, so a later reader does not re-litigate it", so its function is to state the SETTLED disposition, and the disposition it states was undone on 2026-07-28. Compare `plan.toml:1734`, which does mark its superseded fact ("it was removed on 2026-07-28"). `:46` carries no such marker.

Severity `low`, confirming the reviewer. The bullet's operative instruction ("Do not fold it back in here") is correct and unaffected, and `:40` states the current disposition six lines above.

What the fix must achieve. `:46` must not assert that a step exists. The reviewer's suggested wording works; any wording that keeps the human's 2026-07-27 call intact while recording the current disposition does.

---

## `R4-3` (VALID, `low`, cosmetic): missing space before an opening quotation mark in the `Q-69` ask

Evidence reproduced: YES.

```
$ grep -c 'qualified:"Here' docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md
docs/plans/agent-scaffold.plan.toml:1
docs/plans/agent-scaffold.md:1        (line 196)
```

Introduced by the split commit; every other quotation in the same `ask` is introduced with a space.

Severity `low` and cosmetic, confirming the reviewer, including its own assessment that it would not object to acceptance. I rule VALID rather than accepted residual on one ground and I want the ground on the record so it is not read as padding: a fix pass is already required by `H4-1`, `R4-1` and `R4-2`, the re-render happens anyway, and the marginal cost of one character is nil. An accepted residual is for a defect whose fix costs more than the defect; this one costs nothing. It must not be cited as a reason to run a round on its own.

---

## `H4-4` (VALID, `low`): the one unenforced part of step 90 has no check anyone could run

Evidence reproduced: YES, including the proposed check's validity, which I re-verified rather than accepting.

```
$ grep -rn "{{" pack/prompts/          # no hits, so no render slot in any prompt
$ for f in orchestrator planner reviewer triager implementer clarifying-questions open-questions-gate; do
    diff -q pack/prompts/$f.md .agents/prompts/$f.md; done   # no output: all seven byte-identical
```

The exposure is stated by the sidecar itself at `:30`: `.agents/prompts/orchestrator.md` "has NO whole-file drift-guard test, so nothing fails if the regeneration is skipped and the staleness would be silent ... regenerating it is the single easiest thing to forget here." The requirement sentence at `:24` then contains three requirements, all about the prose, and nothing about the deployed copy. I checked whether a criterion lives elsewhere in the sidecar: it does not. `:28` states the duty ("must be regenerated as part of this step, not afterwards") and `:36` gives the exact command, but neither is a check a work reviewer can run to confirm it happened. The contrast is visible one file away: step 92's sidecar has an explicit "acceptance-shaped statement" section.

I narrowed the live failure path and agree with the reviewer's narrowing: simply forgetting the render command fails `cargo test`, because two of the three deployed files are drift-guarded. What fails silently is hand-editing the two guidance copies to match instead of regenerating, which satisfies the guard and leaves the prompt copy stale. Narrow, but it is exactly the exposure step 92 exists to close, occurring in the step that lands first.

Severity `low`, confirming the reviewer. Bounded, visible in the diff to anyone who looks, and reversible.

What the fix must achieve. `:24` (or the currency paragraph at `:28`) must carry one runnable criterion for the unguarded copy. `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` must be empty after the step is the exact form, and it is valid today and stays valid because the prompts carry no render slots and the sidecar forbids the implementer from running the repo-wide formatter.

---

## `H4-5` (VALID, `low`): "the same sentence" is inaccurate, and the derived requirement invites an out-of-scope unification

Evidence reproduced: YES. The two are not the same sentence, and they do not currently say the same thing:

- `pack/prompts/orchestrator.md:33`: "do not put individual findings in the plan's Open Questions section; only durable decisions, the ones that change the plan, fold into it."
- `pack/AGENTS.md:63`: "Never put individual findings in the plan's Open Questions section; only durable decisions, the ones that change the plan, fold into the plan's steps, and a folded decision reopens only by evidence that beats its recorded reasoning."

`decision-folder-currency.md:15` calls the second "the same sentence as the one above in the guidance rather than in the prompt", and `:24` requires the two copies to "end up saying the same thing".

Severity `low`, confirming the reviewer, and I agree with its own limitation of the impact: both are edited by the same step, so "fixing either one alone" is not a live option, and the intended reading of `:24` is recoverable. The live risk is narrow but real, and it is the direction that matters: an implementer taking "the same thing" at face value harmonises "fold into it" with "fold into the plan's steps", or drops the "reopens only by evidence" tail from shipped guidance. Either is a change to shipped text this step did not scope, which is the failure the sidecar's own `:50` guards against in both directions.

What the fix must achieve. `:24`'s requirement must be scoped to the actor clause, not to the sentences as wholes. This is the same sentence `H4-1`'s fix edits, so one rewrite of `:24` closes both; `:15`'s "the same sentence" is worth loosening to "the counterpart sentence" in the same pass but is not separately required.

---

## `H4-3` (VALID BUT ACCEPT RESIDUAL, `low`): the derived-from-manifest guard drops a direction the rejected enumerated form covers

Evidence reproduced: YES on the mechanism, with one over-read corrected.

The mechanism is real. `prompt-drift-guard.md:19` derives the guarded set from the render, so a prompt removed or renamed in `pack/prompts/` drops out of the set and leaves an orphaned committed `.agents/prompts/<role>.md` unguarded and undeleted. The enumerated form it rejects does catch that: `src/agents_md_drift.rs:68` is `.unwrap_or_else(|| panic!("the self-scaffold render includes an asset at {dest}"))`, exactly as cited. The reviewer's closing note also reproduces: `.agents/prompts/` holds 7 files and `pack/prompts/` holds 8 (`checks-reviewer.md` is the module-gated one), so a strict set-equality check against the derived dest set would pass today.

The over-read: the reviewer says acceptance bullet `:10` "will not be true in the membership direction". I read `:10` in full and it defines its own term in the sentence immediately before: "A hand edit made directly to a committed `.agents/prompts/<role>.md`, with the pack left alone, MUST also fail. The guard is a two-way correspondence check, not a one-way staleness check." The two directions named are pack-side edit and deployed-side edit. On its own definition the bullet is accurate, so the finding is not that the artifact says something false.

What remains is an unstated limitation of a design choice that the reviewer itself says is still the right choice, in a paragraph that does state one other trade-off "to accept knowingly". That is improvement-shaped rather than defect-shaped, and three things settle it as a residual for me:

- Triggering it requires a deliberate removal of a prompt from the pack, at which point the person removing it is looking at the deployed copy anyway, and `src/manifest.rs:604-619` enumerates the prompt dests in an asserted list that the same removal has to touch.
- The cost if it ever fires is one orphaned prose file in a repo-local tree, detectable by an `ls`, with nothing shipped wrong.
- Closing it means a CONTENT change to a step the orchestrator froze, which `R4-1`'s currency edit does not. Unlike the currency edit, this one is a scope call, and it is not worth spending one.

Accepting does not block convergence (`pack/AGENTS.md:57`). Recorded here so a round-5 reviewer that reaches it independently is met by the ledger rule rather than by a fresh argument. If the planner is editing `prompt-drift-guard.md` for `R4-1` anyway and wants to add the one sentence, it rides free and I have no objection; it is not required and the round's outcome does not turn on it.

---

## Settled findings: not re-opened, and no new evidence against any verdict

- `T-4` (ACCEPTED RESIDUAL, the four-item paraphrase of `ISOLATION_POLICY_FRAGMENT` at `decision-folder-currency.md:26`): present, unchanged, and no reviewer re-raised it. I confirmed the `Q-69` rewrite does not add a second instance of the class.
- `R2-3` (ACCEPTED RESIDUAL, "the three actor-less `pack/AGENTS.md` prose points" in the `Q-67` ask): present, unchanged, `Q-67` untouched by the split.
- `T-5` (DISMISSED, "copy the pointing, not the list"): not re-raised. See the note below, which is adjacent to `:26` but is not `T-5`'s claim and does not disturb its verdict.
- `T-7` (DISMISSED, `step.title` projection): not re-raised, render unchanged.
- `Q-69` quoting the generated fragment's operative clause plus its item count without reproducing the four items: deliberate and correct. I re-verified the count is four against `src/isolation_policy.rs:33` and that the enumeration appears nowhere in the plan source, the sidecars, or the view.
- The `order` gap at 91: intentional, and `validate` does not object.

---

## What the reviewers missed

1. `:24`'s "ends up saying what `pack/AGENTS.md:71` says" points at a sentence whose trailing half the artifact itself flags as defective. Read whole, `pack/AGENTS.md:71` is the main clause (qualified) PLUS the trailing rationale clause that `plan.toml:1722` and `decision-folder-currency.md:40` both identify as one half of the live `Q-69` contradiction. Taken literally, `:24` asks the implementer to make the prompt's checkpoint paragraph say what that whole sentence says, which would put a second copy of the defective clause into `pack/prompts/orchestrator.md` and expand what `Q-69`'s design pass has to fix from three shipped passages to four, one of them contradicting branch 3 of the same file two paragraphs above. I am deliberately NOT raising this as a separate finding, for two reasons: `:19` is the operative instruction for that passage and it spells out the main clause only, closing with "match the guidance's existing clause, do not invent a different rule"; and the round-1 triager already fixed the reading of `:26`'s "copy the pointing, not the list" as naming the FORM (point rather than enumerate), which is the safe form. So the risk is second-order. It costs nothing to close because `:24` is being rewritten for `H4-1` and `H4-5` regardless: scope the checkpoint requirement to `pack/AGENTS.md:71`'s MAIN clause. This is not a re-raise of `T-5`, whose claim was that the instruction is non-executable in a prompt with no fragment; that verdict stands and I found no evidence against it.
2. The ledger's live resume block is stale against the artifact, which is the ORCHESTRATOR's to fix, not the planner's, and is outside the artifact (`ab6c01d..b36c4c6` touches five files, none of them the ledger). `docs/plans/agent-scaffold.ledger.md:339` still reads "tip `01ab195`", "`Q-69` (`open`)", "step 91 `exploring-item-actor-boundary`" and "the BRANCH is at 92 steps / 69 questions". The branch tip is `b36c4c6`, `Q-69` is `exploring`, step 91 is deleted, and the branch is at 91 steps. A later paragraph in the same block ("`Q-69` (now `exploring`, see the SPLIT below)") and the SPLIT entry at `:347` are current, so the block is half-updated and internally inconsistent. A resuming agent reads `:339` first, because it is the IN FLIGHT anchor. OUT OF SCOPE as a finding against this artifact; flagged for the orchestrator to refresh when it records round 4.
3. Both reviewers reported step 92 clean on soundness and I did not re-derive their whole reproduction, but I did re-run the two that would matter most if wrong: `cargo test` passes with the existing `agents_md_drift` and `isolation_policy` guards, and the 7-versus-8 prompt-count asymmetry that the `checks-reviewer.md` caveat depends on holds.

---

## Backstop

Not triggered. I raised no finding at `high` or `critical`, neither reviewer did, and I dismissed nothing at any severity: the six valid findings go to fix and the seventh is an accepted residual, which is a valid finding resolved by accepting its risk (`pack/AGENTS.md:57`), not a dismissal. No second triager confirmation is owed before this round can be scored.

---

## Convergence judgement (advisory, for the human, not a verdict)

Asked plainly: CONVERGING. Not structural. The evidence, counted rather than impressionistic.

Where round 4's defects landed, against where earlier rounds' landed:

- `Q-69`: ZERO findings on its substance. This is the first round since the item was created in which a fresh reviewer did NOT find a defect in its premises. Round 2 found `NEW-4` (the option set never stated its own boundary); round 3 found `DEC-1` (the fragment's exhaustiveness unsettled, capable of inverting an argument). Both were invisible to the round before, and the round-3 triager called that the signature of options authored before the design space was mapped. The human's split removed the option set and demoted the item to `exploring`. Two independent round-4 lenses examined the rewritten item at length, one of them re-verifying every empirical claim in it against the commits and the const, and neither found a premise defect. The single structural problem this loop ever had was correctly diagnosed, correctly treated, and the treatment is now verified. That is the most important fact for this decision.
- FIX-INDUCED RESIDUE from the round-3 split: `R4-1`, `R4-2`, `R4-3`, that is three of the six valid findings. All one class (a removed step's ordinal, plus one typo), all catchable by a single grep, none touching design.
- NEW-LENS FINDINGS on text no prior round had read as an executable instruction: `H4-1`, `H4-4`, `H4-5`, the other three. All in one region of one sidecar (`:15`, `:21`, `:22`, `:24`), and `H4-1` and `H4-5` are closed by rewriting a single sentence.

Severity is falling monotonically across rounds: round 2 produced a `high` (`NEW-1`), round 3 a `medium` (`DEC-1`), round 4 one `medium` at the bottom of the band and five `low`s. Every mechanical check has been green in all four rounds and is green now. Nothing in the fix list requires a design decision, reopens a settled finding, or touches `Q-69`'s substance; the fixes are disjoint across three files with no interactions.

The one honest caution, which is not "structural" but is predictive. Fix passes have authored the next round's findings twice running: round 2's fix produced `R3-2`, and round 3's split produced half of this round's valid findings. That, not defect density, is what would cost round 5. It is also fully addressable, because the exact sweeps are known:

```
$ grep -rn "step 91\|steps 90 and 91\|order 91\|91, then 92\|own question and step" \
    docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.md
$ cargo run -- render --check docs/plans/agent-scaffold.plan.toml
```

I ran that sweep in this worktree. It returns exactly the three stale spots the findings name (`prompt-drift-guard.md:23`, `:25`, `decision-folder-currency.md:46`), plus `plan.toml:1734`, which is correctly past-tense and must NOT be changed. After the fix it should return only `plan.toml:1734` and its projection.

A second, weaker caution the human should weigh honestly: round 4 found three defects in text three prior rounds passed over, because it ran a lens (can an implementer execute this?) that no prior round ran. That says "no round has found X" is weak evidence about lenses not yet run. It is weaker than it sounds here, because the obvious lenses are now spent (fidelity, consistency, verification three times, decision quality, executability/holistic), and because the artifact is 6 sentences of prose away from a state two independent reviewers have already declared clean everywhere else.

Practical arithmetic for the decision: the artifact is classified `low_risk` at loop-open, so ONE clean round converges. A clean round 5 ends the loop outright. If round 5 is not clean, `pack/AGENTS.md:57` applies the convergence check before the cap, so the only path to escalation is round 5 producing a NEW valid finding, at which point the human gets the decision with a complete ledger. That is a bounded, acceptable downside.

Recommendation, judged against the plan's Project Principles by name: fix all six and run round 5.

- GROUND DECISIONS IN EVIDENCE (Principle 6). The premise-defect generator is gone and verified gone by two lenses; the remaining six are text edits whose fixes are each specified to the sentence. Accepting them instead would trade a nil-cost fix for a shipped-prose risk in `H4-1`'s case.
- MINIMAL BY DEFAULT (Principle 2). One pass, three files, no design decisions, no new items, no scope growth. This is the smallest thing that closes the list.
- NO SILENT SCOPE EXPANSION. The one finding that would have expanded step 92's content (`H4-3`) is accepted as a residual rather than folded in, and the step-92 edit that IS required is currency only.

What I would ask of the fix pass, since this is where the last two rounds lost their next round: run the sweep above and the re-render before handing off, and re-read the four per-passage bullets at `:19` to `:22` for scope consistency AFTER editing `:24`, because that edit and those bullets can drift apart. I would also suggest the round-5 brief name fix-induced residue as an explicit lens for one reviewer, since that class has produced findings in each of the last two rounds and is the only class with a demonstrated recurrence.

---

## Disposition

Must fix before round 5 (six findings, three files, one pass):

| id | file | what |
| --- | --- | --- |
| `H4-1` | `decision-folder-currency.md` `:21`, `:22`, or `:24` | make all four per-passage instructions carry the shipped qualifier; copy `pack/AGENTS.md:41`/`:43`/`:71`, invent nothing |
| `H4-5` | `decision-folder-currency.md` `:24` (and `:15`) | scope the two-copies requirement to the actor clause; same sentence as `H4-1` |
| `H4-4` | `decision-folder-currency.md` `:24` or `:28` | add the runnable criterion for the unguarded deployed prompt copy |
| `R4-2` | `decision-folder-currency.md` `:46` | stop asserting a step exists |
| `R4-1` (merges `H4-2`) | `prompt-drift-guard.md` `:23`, `:25` | remove the references to step 91 and the "(90, then 91, then 92)" ordering; currency only, no content change |
| `R4-3` | `plan.toml:1722` | one space before the opening quotation mark |

Then `cargo run -- render --check` (re-render first; the four projected lines in `agent-scaffold.md` follow from the sidecar and plan-source edits), plus the ordinal sweep above.

Also required of the fix, carried from `H4-1`'s reasoning and item 1 under "What the reviewers missed": the rewritten `:24` must scope the checkpoint requirement to `pack/AGENTS.md:71`'s MAIN clause, so the fix cannot import the trailing clause `Q-69` holds.

Not blocking:

| id | verdict |
| --- | --- |
| `H4-3` | VALID BUT ACCEPT RESIDUAL, `low`. Do not re-raise without new evidence. |

Out of scope for this artifact, raised for the orchestrator: the ledger's IN FLIGHT anchor at `docs/plans/agent-scaffold.ledger.md:339` is stale against `b36c4c6` (tip, `Q-69` status, step 91, step count). Not a planner defect, not in the artifact, and not a finding against it.

Carried accepted residuals, unchanged and re-confirmed present: `T-4`, `R2-3`. Carried dismissals, not reopened: `T-5`, `T-7`.
