# reviewer-diversity triage

Triager: separate agent, independent of the implementer and the orchestrator. Diff range: `c45e36e..959ab7a` (branch `impl/reviewer-diversity`). Workflow model: review-in-worktree, merge-on-convergence. The plan and ledger are orchestrator-owned and live on main; the implementer works only in the worktree branch and does not edit the plan; the orchestrator marks the Roadmap step complete on main after convergence and merge.

Seven distinct findings after dedup, mapped to the reviewers' entries below.

---

## A. `pack/user-prompts/review.md:15` still cues only "which models"

Subsumes: opus finding 1. (No sonnet equivalent.)

Verdict: VALID. Severity: low. Owner: IMPLEMENTER (spawns another round).

Reasoning: `pack/user-prompts/review.md:15` is the human-facing review-request cue and reads "how many independent reviewers and which models". This is the one user-facing surface that invites the human to specify reviewer diversity, and the step's stated purpose (`docs/plans/agent-scaffold.md:643`) is precisely to guide a user who runs multiple CLIs to spread reviewers across harnesses. After this change the canonical rule in `AGENTS.md` says "models or harnesses" while this prompt still says "models" only, so it is a genuine, on-point consistency gap left by the generalization, not a manufactured one. It is low because the field is an optional, bracketed, free-text example that does not constrain the human, and the authoritative guidance the orchestrator reads (`AGENTS.md`) is now correct. Scope note: the plan step named only `pack/AGENTS.md`, `reviewer.md`, and `triager.md` as targets, so this file was not in the literal named list; it is nonetheless within the step's stated intent (generalize the diversity rule), which makes it a legitimate completeness fix rather than scope creep. If fixed, the generated `.agents/user-prompts/review.md` (confirmed present) must be regenerated in sync, growing the change by one file-pair. Suggested wording: "which models or harnesses".

---

## B. Plan step-detail (`agent-scaffold.md:643`) still names `reviewer.md`/`triager.md` as edit targets

Subsumes: opus finding 2 and sonnet finding 2.

Verdict: VALID. Severity: low (downgraded from sonnet's medium). Owner: ORCHESTRATOR (resolved directly on main, does NOT spawn another round).

Reasoning: I confirmed `pack/prompts/reviewer.md` and `pack/prompts/triager.md` contain no diversity/model/harness language (grep returned no match). The implementer's decision to leave them unchanged is correct: neither prompt carries a diversity clause to generalize, diversity is an orchestrator-spawning concern canonical in `AGENTS.md`, and adding it to a single-agent prompt would both be incoherent (an agent cannot choose its own model/harness) and duplicate the canonical rule (Principle 16, one source of truth). The plan step detail at line 643 therefore names two targets that were intentionally not edited, which is a real discrepancy in the durable record. It is downgraded to low because it is plan-doc bookkeeping, not a product or correctness defect in the shipped artifact, and under the merge-on-convergence model plan reconciliation is expected orchestrator work done on main post-merge. When the orchestrator closes the step it should correct the file list (or add an Outcome note) recording that `reviewer.md`/`triager.md` were deliberately left unchanged and why. This is not an implementer fix and should not trigger a fresh review round.

---

## C. No CHANGELOG entry for the guidance change

Subsumes: opus finding 3 and sonnet finding 5.

Verdict: VALID. Severity: low. Owner: IMPLEMENTER in principle (release note ships with the product change), but at this severity the orchestrator may fold it during merge-back rather than spawn a dedicated round.

Reasoning: The scaffolded `AGENTS.md` is the shipped product; a user scaffolding a new project now gets materially different guidance (harness diversity), and the project's own precedent is that scaffolded-workflow-guidance changes get a CHANGELOG entry (the Unreleased `### Changed` entry "Hardened the scaffolded workflow guidance ..." at `CHANGELOG.md:16`). So by precedent this is CHANGELOG-worthy and currently missing. It is low because it is a one-clause edit, the release is still Unreleased, and the cleanest resolution is folding a single clause into the existing "Hardened the scaffolded workflow guidance" entry rather than adding a new bullet, which the existing entry's broad phrasing arguably already gestures at. Because it is low and foldable into an existing entry, treating it as merge-time bookkeeping (like the plan reconciliation in B) is acceptable and avoids a heavyweight round; if it is instead added in the branch it is an implementer edit.

---

## D. Instrumentation prose (`pack/instrument.md`) still frames review diversity as model-only

Subsumes: opus finding 4 (informational).

Verdict: VALID as an observation, but NO ACTION this step. Severity: low / informational. Owner: none now; tracked to the deferred `optional-modules` increment 3.

Reasoning: The `{{instrument}}` section describes the `reviewers[].model` field and "the value of running multiple models", which still frames diversity as model-only after the rule generalized to harnesses. This is a metrics-schema concern. The step is explicitly docs-only with no code, and the cross-harness spawn/calibration machinery is deliberately deferred to `optional-modules` increment 3 (`docs/plans/agent-scaffold.md`, and reconfirmed in the `Q-36` queue item at line 115, which splits the adopt-now docs rule from the deferred spawn-map). Correct disposition is to record the incompleteness against that deferred step, not to fix it here. Opus itself did not recommend a fix; I concur.

---

## E. Plan Roadmap still shows `reviewer-diversity` as `not started` (not marked complete)

Subsumes: sonnet finding 1.

Verdict: INVALID. Severity: n/a (expected artifact, not a defect).

Reasoning: Under the review-in-worktree, merge-on-convergence model, the orchestrator marks a Roadmap step `complete` on main AFTER the review loop converges and the branch merges, NOT in the branch commit, and the implementer does not edit the plan at all. The step's review loop has not converged yet (this triage is part of it), so marking it complete now would be wrong. Sonnet's premise, that the implementer's "keeping the plan's status current" duty covers the Roadmap complete-marking, does not hold under this model: Roadmap status is orchestrator-owned on main, reconciled post-merge. The `not started` status in the branch is the expected state, not an inconsistency the reviewed change should have fixed. (The genuine plan-reconciliation work that IS owed, correcting the step-detail target list, is captured separately as finding B.)

---

## F. "harnesses" introduced without a gloss (Principle 20)

Adjudicates the disagreement: sonnet finding 3 (low) vs opus explicit no-finding.

Verdict: INVALID. Severity: n/a.

Reasoning: I side with opus. "harness" is defined-by-use and pervasive in the document: `AGENTS.md:3` introduces it ("harness-agnostic: any harness-specific file (for example `CLAUDE.md`)") and line 15 uses it again ("Where the harness supports independent sub-agents"). The plural "harnesses" meaning several such CLIs is a one-step inference from the existing anchor, well within Principle 20's self-contained bar. Sonnet's suggested parenthetical would name specific third-party products (Cursor, Copilot) inside canonical guidance, which dates the document and adds noise, so it is arguably a regression rather than an improvement. No gloss is needed; the change is consistent with established terminology.

---

## G. Rationale-clause ambiguity ("same-model and same-harness reviewers share blind spots")

Subsumes: sonnet finding 4. Relates to opus's verification note (opus judged the rationale logically sound).

Verdict: INVALID. Severity: n/a.

Reasoning: The clause reads "different models or harnesses where available, since same-model and same-harness reviewers share blind spots." The natural parse of "same-model and same-harness reviewers" is a single noun phrase with two adjectives, denoting reviewers identical on both axes, that is the worst case. That reading is consistent with the "or" prescription: differing on either axis breaks the correlation. Sonnet's alternative reading (b), treating "and" as joining two independent subject-predicate claims and thereby implying "different models AND harnesses", requires a strained parse against the sentence structure. The prescription's "or" and the rationale's worst-case "and" are complementary, not contradictory (opus reached the same conclusion in its verification section). The `Q-36` queue wording "same-model (and same-harness)" is a design-record note, not the shipped rule, and does not make the shipped clause a defect. There is at most a marginal stylistic sharpening available, but no genuine defect requiring a change.

---

## Severity roll-up

- critical: none.
- high: none.
- medium: none. (Sonnet's two mediums, B and E, are downgraded: B to low, E to invalid.)
- low, valid, actionable: A (implementer, spawns round), B (orchestrator, no round), C (implementer or merge-time fold).
- low, valid, no action now: D (deferred to `optional-modules` increment 3).
- invalid: E (expected under the merge-on-convergence model), F (terminology is self-contained), G (natural reading is consistent).

## Round outcome for the ledger

Only finding A requires an implementer pack fix in the worktree, so it is the sole trigger for another review round. B and C are resolvable at merge-back by the orchestrator (plan step-detail correction and CHANGELOG clause) without a dedicated round. D is tracked to a deferred step. E, F, and G are dismissed. If the orchestrator judges A's low severity as not worth reopening the pack (given the canonical `AGENTS.md` rule is already correct and the review.md cue is optional free text), the remaining valid items are all orchestrator/merge-time bookkeeping and the round is otherwise clean.
