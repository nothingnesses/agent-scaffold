# Plan-review triage, step 90 `decision-folder-currency`, round 1

Triager worktree: `.claude/worktrees/triage-dfc` (detached at `87fb84f` = main + both reviewers' findings files). Artifact under review: `8d12264..5bafb42`, on branch `plan/decision-folder-currency`.

Reviewers triaged:

- `docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-reviewer-fidelity.md` (`FID-1`..`FID-5`).
- `docs/plans/agent-scaffold.reviews/decision-folder-currency-plan-reviewer-consistency.md` (`CON-1`..`CON-4`).

Nine raw findings deduplicate to seven real issues. `FID-1` and `CON-1` are the same issue reached by two lenses; `FID-3` and `CON-3` likewise. `CON-2` splits into a substantive out-of-scope issue and a bounded in-artifact one, so it carries two verdicts.

Outcome: 4 valid findings the planner must fix (1 medium, 3 low), 1 accepted residual (low), 1 out-of-scope item needing a backlog entry raised with the human (medium), 2 invalid. No high or critical finding was raised and none was dismissed, so the convergence backstop re-check does not fire.

Every finding's evidence was reproduced or attempted, per decision `Q-66` and the Triager role in `AGENTS.md`. Reproduction results are recorded per finding below.

Reproduction environment. Reading of `pack/`, `src/`, `justfile`, and the ledger was done in the triage worktree at `87fb84f`, where those paths are byte-identical to `5bafb42` (the artifact diff touches only `docs/plans/agent-scaffold.md`, `docs/plans/agent-scaffold.plan.toml`, and the new sidecar). The three artifact-state files were extracted with `git archive 5bafb42 docs` into a scratch directory and read there, so no artifact-state claim was checked against a stale copy.

---

## T-1 (merges `FID-1` and `CON-1`): the `Q-67` question record still declares its work `pack/AGENTS.md`-only, contradicting the step it now justifies

Verdict: VALID. Severity: `medium` (confirming both reviewers).

Evidence reproduced: YES, in full, for both reviewers.

- `grep -n 'The work this schedules edits' docs/plans/agent-scaffold.plan.toml` on the artifact state returns line 1696, the `Q-67` `ask`, exactly as `FID-1` reports.
- Both quoted clauses are verbatim in that `ask`: "this pass restates none of that list and edits only the three actor-less `pack/AGENTS.md` prose points", and "The work this schedules edits `pack/AGENTS.md` only (guidance, no prompt or source change), then regenerates the deployed `AGENTS.md` and `.agents/AGENTS.reference.md` via `just scaffold-self`."
- `docs/plans/agent-scaffold.plan.toml:1694` is `folded_into = "planner-folds-decisions"`, as `CON-1` reports.
- `FID-1`'s counting demonstration reproduces exactly: `grep -c 'no prompt or source change' docs/plans/agent-scaffold.md` -> 1; `grep -c 'pack/prompts/orchestrator.md' docs/plans/agent-scaffold.steps/decision-folder-currency.md` -> 9.
- The `ask` is projected into the generated view at `docs/plans/agent-scaffold.md:188`, so the contradiction is visible in the human-facing plan, not only in the TOML.
- The schema note is correct: `src/plan/source.rs:314-317` declares `folded_into: Option<String>`, so it cannot name two steps. `FID-1` is right that this is not itself the defect.

Why it is valid. The new step's only provenance is `decisions = ["Q-67"]`, and `Q-67`'s own record states a scope the step exceeds in three of its four passages plus one of its three regeneration targets. This is not a stale cosmetic: `AGENTS.md:106` (task-entry re-grounding) makes "a decision by `q_id`" one of the named durable handles the orchestrator must cite when it enters a step, and `AGENTS.md:63` makes the folded decision the thing that stops relitigation. So the workflow actively directs the next reader to the passage that now misdescribes the work. It is also precisely the defect class this step exists to close: a rule stated in one place and contradicted in another.

Not out of scope and not a residual: the planner owns the plan, the `Q-67` record is part of the plan this fold edits, and no human decision is needed to make a decision record describe what was decided.

Severity. `medium`, not `high`: it is a documentation record, fully reversible, contained to the plan, and a reader who opens the step sidecar gets the correct scope immediately. Not `low`: it will be read at a specific prescribed moment (the step-entry re-ground) by the exact agent whose job is to check "was this asked for?".

What the fix must achieve. The `Q-67` `ask` must stop asserting a scope the decision no longer has. Concretely: qualify or replace the "edits `pack/AGENTS.md` only (guidance, no prompt or source change) ... regenerates the deployed `AGENTS.md` and `.agents/AGENTS.reference.md`" sentence so it describes step 89's pass specifically, and record the 2026-07-27 human scheduling extension: that the decision's remaining currency work was scheduled as `decision-folder-currency` (order 90) covering four further passages, three of them in `pack/prompts/orchestrator.md`, and that the decision's regeneration set therefore also includes `.agents/prompts/orchestrator.md`. The same qualification is owed to the earlier "this pass restates none of that list and edits only the three actor-less `pack/AGENTS.md` prose points" clause. Re-render afterwards so `docs/plans/agent-scaffold.md:188` carries the corrected text. Leaving `folded_into = "planner-folds-decisions"` is correct and forced by the schema; the prose must carry the pointer to the second step.

---

## T-2 (`FID-2`): the sidecar's framing sentence mischaracterises two of the four in-scope passages

Verdict: VALID. Severity: `low` (confirming the reviewer).

Evidence reproduced: PRIMARY YES, one SUPPORTING CITATION MISNUMBERED (does not affect the verdict).

- `pack/prompts/orchestrator.md:27` reproduces verbatim, and is second-person imperative addressed to the orchestrator: "There, update the plan's Open Questions queue and push its open items to the human, each per the human-input contract in `AGENTS.md`".
- `pack/prompts/orchestrator.md:31` reproduces verbatim, likewise imperative: "for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision".
- `pack/prompts/orchestrator.md:33` and `pack/AGENTS.md:63` reproduce verbatim and are genuinely actor-less ("only durable decisions, the ones that change the plan, fold into it" / "fold into the plan's steps").
- The sidecar's own concession reproduces: "so the guidance and the prompt currently disagree about what the verb licenses".
- MISNUMBERED: `FID-2` cites the step-89 triager's characterisation at `docs/plans/agent-scaffold.ledger.md:337`. Line 337 is the "CURRENT TRANSIENT STATE" block and does not contain that text. The quoted string "passively (silent on the actor, NOT contradicting the now-explicit `AGENTS.md:41`)" is verbatim at `docs/plans/agent-scaffold.ledger.md:345`, in the same file, eight lines later. The citation is wrong; the quote is real.

That misnumbering does not sink the finding, because it is a supporting citation, not the demonstration. The load-bearing evidence is the two prompt lines, which reproduce exactly. Reading the correct line actually STRENGTHENS the finding: the step-89 triager's residual note at `ledger.md:345` applies "still describe decision-folding passively (silent on the actor, NOT contradicting the now-explicit `AGENTS.md:41`)" to exactly two passages, "`pack/prompts/orchestrator.md:33` AND the parallel `pack/AGENTS.md:63`", and to no others. The step-90 sidecar extends that characterisation ("None of them contradicts the now-explicit rule; they are incomplete") to all four, including `:27` and `:31`, which the step-89 triager never covered and which do not fit it.

Why it is valid. At `:27` and `:31` the actor is named and it is the orchestrator, in the imperative. `:31`'s "record the resolved answer as a durable Open-Questions decision" conflicts with `pack/AGENTS.md:43`, which says the non-trivial fold is "routed to the planner to author"; `:27`'s unqualified "update the plan's Open Questions queue" is the exact verb that `pack/AGENTS.md:71` needed a whole added sentence to qualify. So the sidecar's diagnosis is wrong for the two passages it itself calls the most load-bearing, and the operation those two need is a carve-out or a reassignment, not an addition.

Severity. `low`, not higher: the per-passage implementer bullets state the right operation for both (`:27` "match the guidance's existing clause"; `:31` "name the planner as the author of the non-trivial fold and keep the orchestrator's own duty (routing it) explicit"), so an implementer who works from the bullets lands correctly, and a half-fix that leaves a contradictory paragraph would be caught by the work review.

What the fix must achieve. Split the framing so the two classes are distinct: `pack/prompts/orchestrator.md:33` and `pack/AGENTS.md:63` are actor-less; `pack/prompts/orchestrator.md:27` and `:31` name the orchestrator and so currently license it to do what `pack/AGENTS.md:41` / `:43` / `:71` reserve for the planner. Drop "None of them contradicts the now-explicit rule; they are incomplete", or restrict it to the two actor-less passages. While there, correct the attribution of the step-89 residual: `ledger.md:345` names `:33` AND `pack/AGENTS.md:63`, not `:33` alone.

---

## T-3a (`CON-2`, substantive half): the exploration-mode passages instruct the orchestrator to author a `[[question]]`, contradicting the clause step 89 added

Verdict: VALID BUT OUT OF SCOPE. Severity: `medium` (confirming the reviewer's rating for the underlying issue).

Evidence reproduced: YES, in full, and I found live corroboration the reviewer did not cite.

- `pack/AGENTS.md:71` reproduces verbatim, including the categorical closing clause: "the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits, which author no reviewed product content, so a `[[question]]` or `[[step]]` (reviewed product content) is not among them."
- `pack/AGENTS.md:45` reproduces verbatim: "the orchestrator records the question as an Open-Questions item with status `exploring`".
- `pack/prompts/orchestrator.md:31`, third branch, reproduces verbatim: "for one whose design space is not yet decidable ..., record it as an `exploring` Open-Questions item". It is in the same sentence as the second branch that step 90 targets.
- `pack/user-prompts/explore.md:13` reproduces verbatim ("record this as an `exploring` open question"), under "Act as the orchestrator described in `.agents/prompts/orchestrator.md`" at `:7`; `explore.md:3` reproduces ("The agent records the question").
- The fourth-deployed-asset claim reproduces: `pack/pack.toml:166-167` maps `user-prompts/explore.md` -> `.agents/user-prompts/explore.md`, and `src/manifest.rs:615` lists that dest.
- Corroboration neither reviewer cited: the tension is not hypothetical, it has already been exercised in this repo. Commit `b6ba317` ("docs: capture Q-68 exploring backlog item for structured-first ledger") is a plain non-merge commit on main that adds a `[[question]]` `id = "Q-68"`, `status = "exploring"` straight to `docs/plans/agent-scaffold.plan.toml`, with no planner branch and no review round, five commits before the artifact under review. That is exactly the edit `pack/AGENTS.md:45` prescribes and exactly what `:71`'s final clause reads as excluding. (I did not establish which role authored it, so I offer this as evidence that the ambiguity is live, not as a finding of a misstep.)

Why it is valid. An `exploring` Open-Questions item IS a `[[question]]` in the structured source, as `Q-68` demonstrates. `pack/AGENTS.md:71`'s final clause is unqualified: a `[[question]]` is not among the orchestrator's direct-on-main edits, full stop. Three shipped passages tell the orchestrator to author one. That is a genuine inconsistency in the shipped pack, of the same family as, and arguably sharper than, the four omissions step 90 targets, since these instruct rather than merely fail to qualify.

Why it is OUT OF SCOPE rather than a defect the planner must fix here. The scope of this step was set by a human on 2026-07-27 after the planner correctly raised a scope question instead of expanding on its own judgement: the human included `pack/prompts/orchestrator.md:27` and excluded `pack/LEDGER.template.md:3`, fixing the sanctioned set at four passages. The exploration-mode class was not before the human, and the resolution is a real design choice with at least three defensible answers (widen the sweep to the exploration passages; qualify `pack/AGENTS.md:71`'s "not among them" so it scopes to a DECIDED item, which would make the exploration passages consistent as written; or record a deliberate exclusion). Choosing among those is a human decision under the human-input contract, and under `Q-67`'s own rule authoring the resulting step is planner work routed through the plan. Absorbing it into step 90 would be exactly the silent scope expansion the workflow forbids. The issue does not evaporate: it lands as the backlog item below.

Severity if left unaddressed anywhere: `medium`. It is a live contradiction in shipped guidance and in the prompt the orchestrator reads every session, and one instance already appears to have fired. It is not `high`: it is prose, no code or data is at risk, and the practical consequence so far is a plan record authored on main rather than through a planner branch.

Backlog item to raise with the human (verbatim, so the orchestrator can put it through the human-input contract):

> The actor boundary added by `Q-67` at `pack/AGENTS.md:71` ("a `[[question]]` or `[[step]]` (reviewed product content) is not among" the orchestrator's direct-on-main edits) is unqualified, but three shipped passages instruct the orchestrator to author an `exploring` `[[question]]` itself: `pack/AGENTS.md:45`, `pack/prompts/orchestrator.md:31` (third branch), and `pack/user-prompts/explore.md:13` (restated at `:3`). Commit `b6ba317` shows the pattern already exercised on main (`Q-68`, `status = "exploring"`, authored directly into `docs/plans/agent-scaffold.plan.toml` with no planner branch). Options: (a) widen the actor-naming sweep to the exploration-mode passages, adding `.agents/user-prompts/explore.md` to the regeneration set (`pack/pack.toml:166-167`, `src/manifest.rs:615`); (b) qualify `pack/AGENTS.md:71` so the exclusion scopes to a DECIDED item's fold, making the exploration passages correct as written and recording that recording an undecided `exploring` item is an orchestrator queue-maintenance edit; (c) record a deliberate exclusion with reasoning. This is a separate step from `decision-folder-currency`, whose scope the human fixed at four passages on 2026-07-27.

---

## T-3b (`CON-2`, in-artifact half): the sidecar does not disclose the exploration-mode class it leaves standing, in the very sentence its implementer will edit

Verdict: VALID. Severity: `low`.

Evidence reproduced: YES (same citations as T-3a). Additionally confirmed against the artifact: the sidecar's "Deliberately out of scope, considered and excluded on purpose rather than overlooked" section lists only `pack/LEDGER.template.md:3`, and the "Scope history, so a later reader does not re-litigate it" paragraph records only the `:27` overrule.

Why it is valid, separately from T-3a. The step's implementer is instructed to edit `pack/prompts/orchestrator.md:31`, which is a single three-branch sentence. Branch 2 is the target. Branch 3 carries one instance of the T-3a issue. After the edit that sentence will state the actor boundary two ways in one breath: the planner authors the decided fold, and you (the orchestrator) record the `exploring` item. An implementer meeting that will either re-litigate it mid-step or quietly extend the fix into branch 3, which is beyond the four passages the human sanctioned. The sidecar already has the two devices for preventing exactly this ("Deliberately out of scope" and "Scope history, so a later reader does not re-litigate it"), and it already uses them for the one exclusion the human ruled on. Recording this one costs a sentence and requires no decision about the substance.

Severity `low`: the consequence is wasted implementer time or a bounded scope overrun, both visible in the work review.

What the fix must achieve. Add the exploration-mode passages to the sidecar's "Deliberately out of scope" list, naming `pack/AGENTS.md:45`, `pack/prompts/orchestrator.md:31` (third branch, in the same sentence as the second target), and `pack/user-prompts/explore.md:13`, stating that they are a distinct class (they name the orchestrator and instruct it to author an `exploring` `[[question]]`, rather than omitting the actor), that they are NOT covered by this step's four-passage scope, and that they are raised to the human as a separate item. Instruct the implementer explicitly not to touch branch 3 of `pack/prompts/orchestrator.md:31`. If the human has given the separate item an id by then, point at it; if not, point at this triage file by path.

---

## T-4 (merges `FID-3` and `CON-3`): the sidecar paraphrases the generated closed list in the paragraph that forbids restating it

Verdict: VALID BUT ACCEPT RESIDUAL. Severity: `low` (confirming both reviewers).

Evidence reproduced: YES, in full, including the precedent both reviewers disclosed.

- `src/isolation_policy.rs:33` is the `ISOLATION_POLICY_FRAGMENT` const, and it ends verbatim "... flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor."
- The sidecar's parenthetical "(a step's status flip, an increment declaration, a round record, and the ledger's resume anchor)" is a four-item nominalised paraphrase in the generated order, as claimed.
- Precedent confirmed at both cited sites: `docs/plans/agent-scaffold.steps/planner-folds-decisions.md:9` carries the identical parenthetical, and so does the `Q-67` `ask` at `docs/plans/agent-scaffold.plan.toml:1696`. A tree-wide grep for the phrase returns only those two, the new sidecar, the generated `agent-scaffold.md`, and the two reviewer files. Both instances are in already-converged artifacts.
- The "no guard covers it" claim reproduces: `grep -rn 'include_str!' src/ --include=*.rs` embeds only `../AGENTS.md`, `../.agents/AGENTS.reference.md`, `../pack/workflow.toml`, `../pack/principles.toml`, `../pack/instrument.md`, and two test fixtures. Nothing embeds or compares `docs/plans/*.steps/*.md`.

Why the residual is right to accept. The duplication is real and unguarded, but three things bound it. First, it lives in the plan record, which is task-scoped and describes a moment in time; the sidecar even says its quotations are "as of the commit that schedules this step". It is not in the shipped pack, which is what the paragraph is protecting. Second, it names its source inline in the same sentence (`src/isolation_policy.rs`, and the `{{isolation_policy}}` slot at `pack/AGENTS.md:91`), so a reader who cares is one hop from the authority. Third, it is an established convention in two converged artifacts including the `Q-67` record itself; fixing this one instance would leave step 90's sidecar inconsistent with step 89's and with the decision record, for no gain to the implementer, whose instruction is "do not enumerate" and does not need the items enumerated to be followed.

On the self-contradiction both reviewers lead with: it is weaker than stated. "This step therefore restates NONE of that list either" refers to what the step does to the pack, not to what the sidecar's own prose does; the sentence is loose, not false. That further lowers the cost of leaving it.

If the human wants it clean, it is a three-instance plan-prose sweep (`planner-folds-decisions.md:9`, the `Q-67` `ask`, and this sidecar), not a step-90 fix. Not blocking convergence.

---

## T-5 (`FID-4`): "copy the pointing, not the list" is said to be non-executable in the prompt

Verdict: INVALID.

Evidence reproduced: YES, every citation. `grep -c '{{' pack/prompts/orchestrator.md` -> 0. `grep -n 'fragment' pack/prompts/orchestrator.md` -> no match. `pack/prompts/orchestrator.md:11`, `:13`, `:15` each use the "see the ... rule in `AGENTS.md`" form, verbatim as described.

Why the finding nonetheless fails. The evidence establishes that the prompt has no fragment; it does not establish that the sidecar's instruction is ambiguous or unachievable, which is the claim. The sentence immediately after the one quoted resolves it explicitly and prohibitively: "The prompt may reference the rule in `AGENTS.md` rather than reproduce the fragment's contents, and it must not enumerate the four direct-on-main edits itself." Read as written, the sidecar states the prompt-side form and forbids the wrong one. "Copy the pointing, not the list" plainly names the FORM (point rather than enumerate) in a sentence whose subject is `pack/AGENTS.md:71` as a MODEL; it is not an instruction to transcribe a literal string, and the reading required for the finding to bite has an implementer copying "the ... fragment below" into a file the same paragraph tells it has no such fragment. The three worked examples the reviewer itself cites make the correct form unmissable, and the second reviewer examined the same clause independently and cleared it as not a finding.

A finding must show the artifact is defective, not that one clause read in isolation is loose. Since the planner is editing this sidecar anyway for T-1, T-2, and T-3b, tightening the phrase to name the prompt-side form directly costs nothing and is worth doing opportunistically, but it is not required and does not block convergence.

---

## T-6 (`CON-4`): the step's regeneration instruction directs the implementer to run the repo-wide formatter

Verdict: VALID. Severity: `low` (confirming the reviewer).

Evidence reproduced: YES, in full, and I add one citation the reviewer did not use that settles the rule conflict outright.

- `justfile:46-48` reproduces exactly: `scaffold-self:` followed by `cargo run -- scaffold --output-dir . --write --force --principles default --instrument` and `nix fmt`.
- `pack/AGENTS.md:79` reproduces verbatim: "Format only your own files. An implementer formats only the files it changed; it must not run repo-wide formatters (for example `just fmt` or `nix fmt`) or `git checkout` / `git restore` on files it does not own, and leaves incidental reformatting to the orchestrator."
- `pack/prompts/orchestrator.md:9` does NOT contain the sentence the reviewer quotes ("Incidental reformatting is yours: after a writer finishes you may run the repo-wide formatter..."); line 9 is the file-safety paragraph ("Keep the tree recoverable; git is your durability substrate..."). This one supporting citation is misplaced. It is not load-bearing: the rule conflict is settled by `pack/AGENTS.md:79` alone, which reproduces exactly.
- The reviewer's drift-guard point reproduces: `src/agents_md_drift.rs` normalises prettier's wrapping degrees of freedom on both sides (`normalize_wrapping`, module docs at `:13-25`, which note the raw render is already byte-identical to both committed files because the pack authors each paragraph on a single line). So the raw `scaffold` render alone satisfies the guard.
- Citation I add, which the reviewer did not need but which removes any judgement call: `pack/AGENTS.md:108` (Prose formatting) says the incidental-reflow convention "is distinct from the 'Format only your own files' file-safety rule above, which still holds that a writer does not proactively run a repo-wide formatter and leaves incidental reformatting to the orchestrator". `AGENTS.md` therefore rules on the exact case directly.
- The justfile's own comment (lines 41-45) confirms the blast radius: "`nix fmt` formats the whole tree, not just the generated files".

Why it is valid. The sidecar states the mechanism ("`just scaffold-self` regenerates the deployed copies and then runs `nix fmt`") and immediately imposes the duty ("Three deployed files go stale and must be regenerated as part of this step, not afterwards"). An implementer reads that as "run `just scaffold-self`", whose second line is a repo-wide `nix fmt` that `pack/AGENTS.md:79` forbids it from running and `pack/AGENTS.md:108` re-confirms is the orchestrator's job. The practical cost is an implementer diff polluted with reformatting of files it does not own, in a step whose entire subject is who is allowed to author what.

Why NOT an accepted residual despite the precedent (`planner-folds-decisions.md:11` and `reviewer-reproducible-evidence.md:12` both say the same, and both converged). The rule is explicit and now doubly stated, the step is not yet implemented so the fix is free, and this step's regeneration set uniquely includes `.agents/prompts/orchestrator.md`, which the sidecar itself establishes has NO drift guard, making the regeneration instruction more load-bearing here than in either precedent. Correcting it once is cheaper than propagating it a third time.

Severity `low`: bounded, visible in the diff, and reversible.

What the fix must achieve. The documentation-currency instruction must name the implementer's half only: regenerate the three deployed copies with the render half (`cargo run -- scaffold --output-dir . --write --force --principles default --instrument`), and leave the repo-wide `nix fmt` to the orchestrator, citing the "Format only your own files" rule. It must not tell the implementer to run `just scaffold-self`.

---

## T-7 (`FID-5`): the rendered heading is vaguer than the TOML `title`, and `step.title` is never projected

Verdict: INVALID as a finding against this artifact. (One real observation inside it becomes a low-value backlog note, below.)

Evidence reproduced: YES, all of it.

- `docs/plans/agent-scaffold.md:1176` carries the short heading; `docs/plans/agent-scaffold.plan.toml:1241` carries the long `title`. They differ as described.
- `step.title` really is never projected. `src/plan/render.rs:296` uses `plan.meta.title` and nothing else; a grep over `src/` for `.title` finds only `[meta].title` uses, three ratatui widget titles in `src/tui.rs`, and the struct field declaration at `src/plan/source.rs:139`. No `status` or `validate` path reads it.

Why the finding fails. The long-TOML-title / short-sidecar-heading pairing is the plan's established convention, not drift. Verified against three other steps: `reviewer-reproducible-evidence`, `planner-folds-decisions`, and `code-value-audit-static` each pair a long enumerating `title` with a short slug-and-topic `###` heading (`decision-receipt` has them equal only because its title is already short). The second reviewer checked the same three and reached the same conclusion.

The overclaim sub-argument does not hold either. "the remaining actor-ambiguous decision-folding points" heads a section whose first paragraph says "FOUR passages" and lists them, and whose "Deliberately out of scope" section names the exclusion; no reader of that section is misled by its heading. The step-89 analogy fails on inspection: that nit was a title that still named DROPPED work, a factual inaccuracy, and it was fixed at commit `dc9686a`; the current step-89 title is Part-1-only and correct. A shorter heading is not the same defect.

Backlog note (low value, recorded so it is durable rather than lost). `[[step]].title` is a required field on all 90 steps and is consumed by nothing: no render projection, no `status`, no `validate`, no TUI. Under plan Principle 8 and one-source-of-truth it is either dead data or an unbuilt projection. That is a schema and render-engine question spanning the whole plan, not a step-90 fix; raise it with the human only if a `Q-44`-family cleanup pass is already open.

---

## Issues I found while reproducing, that both reviewers missed

1. `pack/AGENTS.md:71`'s categorical clause has already fired on this repo. Commit `b6ba317` adds `Q-68` (`status = "exploring"`) as a `[[question]]` directly to `docs/plans/agent-scaffold.plan.toml` in a plain non-merge commit on main, five commits before the artifact. Neither reviewer cited it. It is recorded under T-3a as corroboration that the exploration-mode tension is live rather than theoretical.

2. The sidecar's opening "Next." deviates from the plan's own documented protocol, on a stronger ground than the reviewer used. `docs/plans/agent-scaffold.documentation-protocol.md` states that the Step Details "carry each step's design, decisions, and (once done) outcome and evidence, and do not repeat the status label". The fidelity reviewer spotted the "Next." lead word (its non-finding 12) but grounded its decision not to raise it only on pervasiveness and the step-88 precedent; the protocol sentence makes it a deviation from a stated project rule, not merely a stylistic habit. My verdict is unchanged: ACCEPTED RESIDUAL. The reviewer's factual point stands, and I confirmed it, that the deviation is pervasive (`planner-folds-decisions`, `decision-receipt`, `round-log-core`, `session-preflight`, `structured-skeleton`, `workflow-invariants`, and `reviewer-reproducible-evidence` all open with a status word that is now stale against their `status = "complete"`), and this new instance is currently accurate. The cure is a project-wide sidecar sweep, not a step-90 fix. Recorded here with the protocol citation so a later sweep has the authority to point at.

3. Checked and cleared, so a later round does not raise it: the new `[[step]]` block carries no `risk_class`, and that is correct, not an omission. `risk_class` is a field of `Increment`, not of `Step` (`src/plan/source.rs:135-166`, `#[serde(deny_unknown_fields)]`), and the artifact's risk classification is recorded in the ledger's round-records narrative per `AGENTS.md:56`, not in the plan.

---

## Independent verification of the mechanical claims

Both reviewers reported the toolchain green. I re-ran the checks myself against the artifact state extracted to a scratch directory, rather than accepting the reports:

```
$ agent-scaffold render <scratch>/docs/plans/agent-scaffold.plan.toml --check
<scratch>/docs/plans/agent-scaffold.plan.toml: up to date                      (exit 0)
$ agent-scaffold validate --source <scratch>/docs/plans/agent-scaffold.plan.toml --metrics <scratch>/docs/metrics/workflow.jsonl
209 records, valid; 90 steps, 68 questions, valid                              (exit 0)
$ agent-scaffold validate ... --workflow
workflow invariants hold                                                       (exit 0)
```

I also re-ran the documentation-impact sweep independently of both reviewers, grepping the tree for each of the three affected clauses. The deployed set is exactly `.agents/prompts/orchestrator.md`, `AGENTS.md`, and `.agents/AGENTS.reference.md`, as the sidecar claims; `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` is empty, confirming the verbatim-copy characterisation; and the no-content-guard claim for `.agents/prompts/orchestrator.md` holds (`src/manifest.rs:604` is a dest-list entry, and no `include_str!` in `src/` embeds it). The sidecar's warning that skipping that regeneration would be a silent staleness is accurate.

---

## Disposition

Must fix before this artifact can be re-reviewed:

| id | merges | verdict | severity |
| --- | --- | --- | --- |
| T-1 | `FID-1`, `CON-1` | VALID | `medium` |
| T-2 | `FID-2` | VALID | `low` |
| T-3b | `CON-2` (in-artifact half) | VALID | `low` |
| T-6 | `CON-4` | VALID | `low` |

Not blocking:

| id | merges | verdict | severity |
| --- | --- | --- | --- |
| T-3a | `CON-2` (substantive half) | VALID BUT OUT OF SCOPE, backlog item above | `medium` |
| T-4 | `FID-3`, `CON-3` | VALID BUT ACCEPT RESIDUAL | `low` |
| T-5 | `FID-4` | INVALID | n/a |
| T-7 | `FID-5` | INVALID (low-value backlog note attached) | n/a |

No dismissal at or above the backstop severity (`high`), so no second-triager re-check is owed on this round.

All four must-fix items are edits to the plan source (the `Q-67` `ask` in `docs/plans/agent-scaffold.plan.toml` for T-1; the sidecar `docs/plans/agent-scaffold.steps/decision-folder-currency.md` for T-2, T-3b, and T-6) plus a re-render of `docs/plans/agent-scaffold.md`. T-2, T-3b, and T-6 touch three different paragraphs of the same sidecar and can be done in one pass.
