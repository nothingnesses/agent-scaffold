# Plan-fold review: design-pass decisions fold (commit 1f73f68)

Reviewer: sonnet (plan hygiene, consistency, and principles lens)
Diff: `git diff 827ea15..1f73f68 -- docs/plans/agent-scaffold.md`

---

## F-1: Queue items Q-27/Q-30/Q-31/Q-33/Q-35/Q-36 embed decision detail instead of a terse ask + pointer

**Severity: medium**

**Evidence:**

The Documentation Protocol (lines 35-36 of the plan) is explicit: "the adopted decision and its reasoning are folded into the relevant step, and the queue item points to that step." The pattern for correct items is visible in Q-1 through Q-3: `<one-line ask>. Decision and reasoning in <slug>.`

The six items introduced or updated by this fold embed multi-sentence decision summaries:

- Q-30 (new): "Decided (human, after a two-independent-explorer design pass): an optional module hard-depending on the checks module, a separate `test-author` writer role, a two-artifact/two-gate-profile phase-4 split with red/green as a phase parameter (not a checks.toml field), test execution single-sourced as a `kind = "test"` checks entry, and a frozen-tests tripwire. Decision and reasoning in `test-driven`..."
- Q-31 (new): "Decided (human, same design pass): hard-depends on `test-driven`, a `kind = "mutation"` checks entry, fires once per step after green convergence, diff-scoped, time-budgeted and risk-scaled, reuses the checks-reviewer at a distinct gate, routes a surviving mutant back to the test artifact, and degrades to a recorded skip when no tool exists. Decision and reasoning in `mutation`..."
- Q-33 (newly decided): "Decided (human, after the two-explorer cross-harness design pass...): increment 3 STAYS pointer-scaffolding now; the container-wrapper shim (and the `.agents/spawn.toml` role->launch-command map) is DEFERRED to a future evidence-grounded increment (the harness owns spawning; cross-environment agents are reachable only via a Bash shell-out launch path, never the native Agent tool). Decision and reasoning in `optional-modules` (increment 3)..."
- Q-35 (new): "Decided (human, after a two-explorer design pass): INTEGRATE it as a fourth ENTRY MODE of the one workflow (the acceptance pass promoted to a human entry; single-pass reviewers-then-triager, no implement phase, read-only roles), NOT a separate workflow, with a thin `review.md` prompt, a diff-or-whole-tree target, composable criteria, a committed KEPT report, and a `review` metrics phase value. It subsumes the whole-codebase acceptance-review LATER job. Decision and reasoning in `review-mode`..."
- Q-36 (new): "Decided (human, after the two-explorer cross-harness design pass...): ADOPT NOW the always-on core docs rule generalising the diversity rule from 'different models where available' to 'different models OR harnesses where available' (folded into `reviewer-diversity`); DEFER the spawn-map... Decision and reasoning in `reviewer-diversity` and `optional-modules`..."
- Q-27 (updated): the old entry was trimmed, but still says "Decided (human, after a two-independent-explorer design pass): `validate --workflow` (detection, not prevention), reading the JSONL round log and the plan; the two owed sub-decisions (a `trivial` status declaring a review-skipped completion, a `grandfathered` status exempting pre-logging legacy steps) are now settled too. Decision and reasoning in `workflow-invariants`..."

Every one of these summaries is also present verbatim (or more completely) in the owning step detail block. If any step's wording changes, the queue item drifts. This is the same failure class flagged as G8 (round 2, triager verdict: "trim decided queue items to ask + status + pointer, the decision lives in the step") and M1 in plan-maintenance review.

**Fix:** Trim each of these six items to the one-line ask + status + pointer. For example, Q-30 should read: "`Q-30` (decided -> folded into `test-driven`) test-driven process module: optional module adding a per-step test spec and a test-first phase before implementation. Decision and reasoning in `test-driven`; design record in `docs/plans/test-modules.explorations/`."

---

## F-2: Q-26 had decision detail added by the fold (worsening a pre-existing problem)

**Severity: low**

**Evidence:**

Q-26 already had decision detail before the fold (the option (a)/(b)/(c) breakdown). The fold added the following sentence: "The increment-2 config LAYOUT is now settled by the checks-config design pass (the `[[check]]` schema, the `{{modules}}` render slot, the `checks-reviewer` role, and the `agent-scaffold checks` / `--with-precommit-hook` hook path); the settled layout lives in `optional-modules`."

The detailed schema listing (`[[check]]` schema, `{{modules}}` render slot, etc.) belongs in the `optional-modules` step, not in the queue item. The pointer "the settled layout lives in `optional-modules`" is present and correct, but the enumeration that precedes it is duplicate decision detail.

**Fix:** Remove the schema enumeration, keeping only: "Unblocks building increment 2 of `optional-modules`. The settled config layout lives in `optional-modules`. Decision and reasoning fold into `optional-modules`."

---

## F-3: Q-36 uses non-standard dual-slug status form

**Severity: low**

**Evidence:**

The Documentation Protocol (line 36 and line 74) specifies the status form as `decided -> folded into <slug>` (singular slug). Q-36 reads: `decided -> folded into reviewer-diversity and optional-modules`.

Two slugs joined with "and" is not part of the allowed form. The validator passes it (exit 0, confirming the validator does not enforce the singular-slug constraint), but the form departs from the documented convention and leaves ambiguous which step is the primary decision-reasoning home.

Q-36 is the only such case in the plan; all other multi-target items (e.g. Q-4/Q-5/Q-6 that fold into `instrument-flag`) are separate queue items that each point to a single slug.

**Fix:** Either (a) keep Q-36 as a single item and pick the primary owning step (e.g. `reviewer-diversity`, since the always-on docs rule is the adopt-now half, and note `optional-modules` for the deferred half in the pointer text), or (b) split Q-36 into two items, one per slug. Option (a) is lighter and consistent with how Q-4/Q-5/Q-6 are treated as separate items rather than one combined item.

---

## F-4: Workflow self-improvement cluster umbrella still names `ledger-parse` as the build-order keystone

**Severity: low**

**Evidence:**

Line 588 of the plan (inside the `### Workflow self-improvement cluster` umbrella heading):

> "Build order: `exploration-mode` (cheapest, unblocks proper deliberation), then the `ledger-parse` keystone, then `workflow-invariants`, then `state-queries`; each a separately reviewed step."

`ledger-parse` is now `skipped` (Roadmap line 162; confirmed by its detail block: "Skipped. Its premise... evaporated once `round-log-core`..."). The build order asserts it as the keystone that gates `workflow-invariants` and `state-queries`, but this is no longer true: `workflow-invariants` is `next` and reads `src/plan.rs` + `src/metrics.rs` directly with no `src/ledger.rs`.

The RESUME STATE (in the ledger, at commit 827ea15) specifically directed the fold to "REMOVE the stale `ledger-parse` dependency from `Q-27`/`Q-28`". The fold correctly removed it from Q-27 and Q-28 but missed this sentence in the umbrella.

A reader resuming from the umbrella heading would incorrectly conclude that `ledger-parse` must be built before `workflow-invariants`.

**Fix:** Update the build-order sentence to: "Build order: `exploration-mode` (complete), then `workflow-invariants` (next), then `state-queries`; `ledger-parse` was skipped (its premise evaporated when `round-log-core` promoted the JSONL log to the single structured store)."

---

## F-5: Principle citations "(Principle 5, Principle 6)" in the fold-rewritten optional-modules rationale likely leak AGENTS.md numbering

**Severity: low**

**Evidence:**

The `optional-modules` increment (2) rationale (line ~521) contains: "...the same 'do not trust the LLM, verify with a tool' ethos as the metrics validator and the schema drift guards (Principle 5, Principle 6)."

The plan's own Project Principles (lines 18-24) are numbered 1-7:
- Principle 5: Make illegal states unrepresentable
- Principle 6: Ground decisions in evidence (validate with a proof-of-concept before building)

The context "do not trust the LLM, verify with a tool" maps much more precisely to AGENTS.md's numbering:
- AGENTS.md Principle 5: Have independent or adversarial review
- AGENTS.md Principle 6: Verify, don't trust

The plan's Principle 6 (ground in evidence) is a partial match, and Principle 5 (make illegal states unrepresentable) is a stretch. The round-2 plan-maintenance finding G10 previously flagged this exact pattern ("'Principle 13'/'5' mix the plan's 1-7 numbering with AGENTS.md's") and the remedy was "use the plan's own 1-7 numbering."

This citation was retained unchanged through the fold's significant rewrite of the surrounding increment (2) section.

**Fix:** Re-examine which plan Principles (1-7) actually ground the rationale, and replace the citation with accurate plan-numbered ones. The best match within the plan's own set is Principle 6 (ground decisions in evidence) for "verify with a tool", and the citation of Principle 5 should either be dropped or replaced with a plan Principle that actually applies (e.g. Principle 2: minimal by default, or Principle 4: idempotent). Document the reasoning for whichever plan Principles are cited.

---

## Summary

| # | Title | Severity |
| - | ----- | -------- |
| F-1 | Q-27/Q-30/Q-31/Q-33/Q-35/Q-36 embed decision detail in queue (recurring failure class) | medium |
| F-2 | Q-26 had extra detail added by the fold | low |
| F-3 | Q-36 uses non-standard dual-slug status form | low |
| F-4 | Cluster umbrella still names `ledger-parse` as the build-order keystone (stale) | low |
| F-5 | `(Principle 5, Principle 6)` citation in fold-rewritten text likely leaks AGENTS.md numbering | low |

Validator result: exit 0 (46 steps, 36 queue items, valid). No non-ASCII characters found. No orphan detail blocks or missing Roadmap rows for the four new steps. The four new Roadmap entries (`reviewer-diversity`, `review-mode`, `test-driven`, `mutation`) each have a correctly-headed `### <slug>` detail block and `not started` is a valid status. The `exploring` -> `decided` transitions for Q-30/Q-31/Q-33/Q-35/Q-36 are structurally correct. No stale step count in any umbrella (the workflow cluster umbrella was not updated but still lists 4 steps, matching the Roadmap). No incorrect step-status restatements in new detail blocks beyond the plan-wide convention of leading "Not started." lines.
