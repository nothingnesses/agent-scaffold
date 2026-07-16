# Plan-fold triage (design-pass decisions fold, commit `1f73f68`)

Triager: independent of both the producer (planner) and the orchestrator. Verdicts adjudicated against the plan text (`docs/plans/agent-scaffold.md`), the ledger LOCKED DECISIONS (`docs/plans/agent-scaffold.ledger.md` RESUME STATE), the Documentation Protocol, and the two prior triager rulings on the same failure classes (round 2 G8/G9/G10; plan-maintenance H2/M1/H1).

Findings adjudicated: T1 (sonnet F-1), T2 (sonnet F-2), T3 (sonnet F-3), T4 (sonnet F-4), T5 (sonnet F-5), T6 (opus F-1), T7 (opus F-2), T8 (opus F-3).

Result: 7 VALID (all low), 1 INVALID. No finding is higher severity than filed; T1 is LOWER than filed (medium -> low, see below). No high/critical raised or warranted, so the dismissed-high-severity backstop does not apply.

---

## T1 (sonnet F-1) - VALID, low (downgraded from medium)

Q-27/Q-30/Q-31/Q-33/Q-35/Q-36 each embed a full multi-clause `Decided (human ...): <list>` summary in the queue instead of a terse ask + status + pointer.

Verdict: VALID. The Documentation Protocol (lines 35-36, 74) states the decision and its reasoning are folded into the step and the queue item points to that step. Two prior triagers ruled this exact class the same way (round 2 G8: "trim decided queue items to ask + status + pointer, the decision lives in the step"; plan-maintenance M1: "hold to the terse ask + status + pointer queue form ... the 'Decided:' gloss is what let the drift in"). The embedded `Decided:` summaries are the drift vector the protocol exists to prevent.

Severity: LOW, not medium. Absolute impact is documentation duplication with no current inaccuracy (opus verified no drift and faithful capture). Both prior triagers rated the identical class low (G8 low, M1 low). Sonnet's medium rests on future-drift risk, which is real but not yet realised; low is the correct absolute rating and keeps this consistent with precedent.

Note on the terseness bar: the plan is genuinely split. Q-1..Q-19 are terse (ask + `Decision and reasoning in <slug>.`). Q-20/Q-24/Q-25/Q-28/Q-29/Q-32/Q-34 already carry full `Decided:` summaries (pre-existing, unflagged, out of scope for this fold's review). The DOCUMENTED bar and the twice-ruled bar are terse, so the target is terse; the pre-existing verbose items are not the bar, they are un-cleaned violations. The fold itself IMPROVED Q-27 (it was longer pre-fold). Trimming these six moves toward the target state.

Exact trimming target (ask + status + step pointer + design-record pointer; drop the `Decided (human ...): <clause list>` sentence entirely, since it is reproduced verbatim in the owning step):

- Q-27 -> keep: "deterministic workflow-invariant enforcement: a deterministic check so a skipped or unreviewed step fails rather than relying on the orchestrator's discretion (motivating incident 2026-07-16: the `pause.md` step committed outside the review loop, which no check would have caught). Decision and reasoning in `workflow-invariants`; design record in `docs/plans/workflow-invariants.explorations/`." Drop "Decided (human, after a two-independent-explorer design pass): `validate --workflow` ... are now settled too."
- Q-30 -> keep (sonnet's suggested target is correct): "test-driven process module: optional module adding a per-step test spec and a test-first phase before implementation. Decision and reasoning in `test-driven`; design record in `docs/plans/test-modules.explorations/`. Related: `Q-31`." Drop the "Decided (human ...): an optional module hard-depending ... frozen-tests tripwire" clause.
- Q-31 -> keep: "mutation-testing module: an optional module layered on top of `test-driven` that mutates the code and confirms the tests kill the mutants (tests are not vacuous). Decision and reasoning in `mutation`; design record in `docs/plans/test-modules.explorations/`." Drop the "Decided (human, same design pass): hard-depends ... recorded skip when no tool exists" clause.
- Q-33 -> keep: "real agent-box/agent-images integration for our own spawned agents (vs pointer scaffolding): whether `optional-modules` increment 3 grows a real container-integration path or stays pointer-scaffolding. Decision and reasoning in `optional-modules` (increment 3); design record in `docs/plans/cross-harness-isolation.explorations/`." Drop the "Decided (human ...): increment 3 STAYS ... never the native Agent tool" clause.
- Q-35 -> keep: "human-invokable code-review mode, integrated as a fourth entry mode: a human prompts the agent to review the current tree or a diff against a plan and/or constraints, and gets a findings report. Decision and reasoning in `review-mode`; design record in `docs/plans/review-mode.explorations/`." Drop the "Decided (human ...): INTEGRATE it as a fourth ENTRY MODE ... acceptance-review LATER job" clause.
- Q-36 -> keep: "cross-harness / cross-model agent spawning: generalise reviewer/triager diversity beyond one harness and let the orchestrator spawn agents on different models or CLIs to minimise shared blind spots. Decision and reasoning in `reviewer-diversity` (adopt-now docs rule) and `optional-modules` increment 3 (deferred spawn-map); design record in `docs/plans/cross-harness-isolation.explorations/`." Drop the "Decided (human ...): ADOPT NOW ... deferred growth path" clause. (This wording also carries the which-step-owns-which-half disambiguation, which is why T3 needs no separate fix.)

---

## T2 (sonnet F-2) - VALID, low

The fold added a schema-enumeration sentence to Q-26 (`[[check]]` schema, `{{modules}}` render slot, `checks-reviewer` role, `agent-scaffold checks` / `--with-precommit-hook` hook path) that belongs only in `optional-modules`.

Verdict: VALID. Same class as T1: the enumerated schema specifics are step detail, and the pointer "the settled layout lives in `optional-modules`" is already present and sufficient. Severity low (documentation duplication, no inaccuracy).

Fix: remove the schema enumeration from the added sentence, keeping the pointer. Minimal target: "... Unblocks building increment 2 of `optional-modules`; the settled config layout lives in `optional-modules`. Decision and reasoning fold into `optional-modules`." (The pre-existing option (a)/(b)/(c) gloss in Q-26 is out of scope for this fold and can be left, though a follow-up terseness sweep would trim it too.) Apply in the same trim pass as T1.

---

## T3 (sonnet F-3) - INVALID (dismissed)

Q-36's status is the dual-slug form `decided -> folded into reviewer-diversity and optional-modules`; sonnet says the format specifies a single `<slug>` and that "Q-36 is the only such case in the plan."

Verdict: INVALID. The finding's load-bearing premise is factually wrong. Q-19 (line 98) already uses the identical dual-slug form: `(decided -> folded into compaction-prep and user-prompts-dir)`. That item predates this fold and has survived multiple review/triage rounds, so the "X and Y" form for a genuinely two-home decision is an established, validator-accepted convention, not a one-off deviation. Q-36's decision genuinely has two homes per the locked decision (the adopt-now always-on docs rule -> `reviewer-diversity`; the deferred spawn-map/shim -> `optional-modules` increment 3), so the dual-slug accurately reflects it. Splitting into two items (sonnet's option b) would misrepresent one cross-harness decision as two separate questions; collapsing to one slug (option a) would drop a real home. The `<slug>` in the Documentation Protocol is a schema placeholder, not a prohibition on multi-target folds, as Q-19 demonstrates. The validator accepts it. No fix needed. (The which-half-lives-where disambiguation is handled by the T1 pointer rewrite for Q-36.)

---

## T4 (sonnet F-4) - VALID, low

The Workflow self-improvement cluster umbrella (line 588) still lists the build order as "... then the `ledger-parse` keystone, then `workflow-invariants`, then `state-queries`", but `ledger-parse` is SKIPPED.

Verdict: VALID. `ledger-parse` shows `skipped` in the Roadmap and its detail block, and `workflow-invariants`/`state-queries` were re-scoped by the fold to remove the ledger-parse dependency (per the RESUME STATE directive to "REMOVE the stale `ledger-parse` dependency"). The umbrella build-order sentence is a stale reference the fold missed; it asserts a skipped step as the keystone gating the two live steps, which would mislead an agent resuming from the umbrella. Severity low: the Roadmap (the declared source of truth for order/status) and both step detail blocks already carry the correct picture, so this is one stale spot among correct signals.

Fix: update the build-order sentence to reflect the skip, e.g. "Build order: `exploration-mode` (complete), then `workflow-invariants` (next), then `state-queries`; the `ledger-parse` keystone was skipped once `round-log-core` promoted the JSONL log to the single structured store, so `workflow-invariants` reads `src/plan.rs` + `src/metrics.rs` directly." The cluster-membership parenthetical on the same line may also drop `ledger-parse` or annotate it "(skipped)"; either is acceptable.

---

## T5 (sonnet F-5) - VALID, low

"(Principle 5, Principle 6)" in the `optional-modules` increment-2 rationale ("... the same 'do not trust the LLM, verify with a tool' ethos as the metrics validator and the schema drift guards (Principle 5, Principle 6)") leaks AGENTS.md numbering.

Verdict: VALID, verified against both schemes.
- Plan Project Principles: P5 = "Make illegal states unrepresentable"; P6 = "Ground decisions in evidence (validate with a proof-of-concept)".
- AGENTS.md Principles: P5 = "Have independent or adversarial review"; P6 = "Verify, don't trust - Run it and test it rather than asserting success".

The cited context "do not trust the LLM, verify with a tool" + "metrics validator and schema drift guards" maps almost exactly to AGENTS.md P6 (verify, don't trust) and AGENTS.md P5 (independent/adversarial review). Under the plan's own numbering, P5 (make illegal states unrepresentable) does not fit "verify with a tool" at all, and P6 (ground decisions in evidence / proof-of-concept) is only a partial fit. So "(Principle 5, Principle 6)" matches AGENTS.md, not the plan; this is the same leak class as round 2 G10 and plan-maintenance H1, whose established remedy was "use the plan's own 1-7 numbering." The citation rode through unchanged in the fold's rewrite of the surrounding section.

Fix: replace with the plan-numbered principle(s) that actually apply. Best single match is plan Principle 6 (ground decisions in evidence) for "verify with a tool". If a second citation is wanted, plan Principle 7 (reproducible: prefer the project's toolchain conventions so a scaffolded project and its checks behave the same on any machine) fits the deterministic-checks module better than P5; the planner should pick and confirm the reasoning, and drop the P5 citation (make-illegal-states does not apply here).

---

## T6 (opus F-1) - VALID, low

The Status resume-anchor sentence says "The remaining planned work is `optional-modules` then `workflow-calibration`", omitting the four newly-added not-started steps (`reviewer-diversity`, `review-mode`, `test-driven`, `mutation`) and the process cluster (`workflow-invariants`, `state-queries`).

Verdict: VALID. The fold added four not-started Roadmap steps and edited this exact sentence but did not update the enumeration, so the resume anchor's summary of remaining work is understated. Severity low: the Roadmap is the declared source of truth for order/status and carries all of them, and the sentence does already gesture at `review-mode` via the subsumption clause, capping the impact.

Fix: extend the enumeration to name the process cluster and the four new steps as also planned/not-started, or point the reader to the Roadmap for the complete not-started set rather than naming a partial list.

---

## T7 (opus F-2) - VALID, low

In `optional-modules` increment (2), the four settled-schema bullets (`.agents/checks.toml`, `{{modules}}` slot, `checks-reviewer`, hook path) and the `Rationale:` paragraph sit at column 0, peer to the `- Increment (1/2/3)` bullets, so they read as sibling increments rather than increment-2 sub-points.

Verdict: VALID. Increment (1) and increment (3) are each a single self-contained top-level bullet; breaking increment (2) into a lead bullet plus four peer bullets plus a peer Rationale paragraph makes the list read as if there are extra increments and invites misattribution. Severity low: it renders readably and `validate --plan` does not care, so impact is cosmetic/clarity.

Fix: indent the four schema bullets and the Rationale under the increment (2) bullet (nest them), or introduce an explicit "Increment (2) settled layout:" sub-list, so the increment-2 content nests like the other increments' content.

---

## T8 (opus F-3) - VALID, low

In `workflow-invariants`, W3's per-step-slug phrasing ("filter `round` records whose `task` leading-slug ... equals the step slug; if any exist, their `risk_class` must be consistent and the consecutive-clean streak (per artifact) must reach the required count") is in tension with the BUILD-TIME MUSTS two paragraphs later, which state the streak/consistency checks must run PER INCREMENT.

Verdict: VALID. Read literally, W3 groups all records by the stripped step slug and requires `risk_class` consistency across that set; for `round-log-core` (incA `low_risk` + incB `risky`) that set is inconsistent, so W3-as-stated would false-flag it. The later paragraph is the correction, so the block is internally self-correcting and faithfully reproduces both halves of the locked decision (which itself states "the streak/consistency checks must run PER INCREMENT ... else round-log-core false-flags"). But a builder reading only the W3 summary could implement the false-flagging step-level version. Severity low: clarity nit, not drift; both requirements are present and the locked decision leaves reconciliation to build time.

Fix: fold the per-increment qualifier into the W3 sentence itself, e.g. "... group the filtered records BY INCREMENT (the leading slug plus its `-inc<x>` suffix), and within each increment the `risk_class` must be consistent and the consecutive-clean streak reach the required count", so W3 does not read as a step-level check the later paragraph then contradicts.

---

## Rollup

| # | Reviewer | Verdict | Severity | Resolution |
| - | -------- | ------- | -------- | ---------- |
| T1 | sonnet F-1 | VALID | low (was medium) | Trim Q-27/Q-30/Q-31/Q-33/Q-35/Q-36 to terse ask + status + step pointer + design-record pointer (exact targets above). |
| T2 | sonnet F-2 | VALID | low | Remove the added schema enumeration from Q-26, keep the pointer. |
| T3 | sonnet F-3 | INVALID | n/a | Dual-slug form is precedented (Q-19), validator-accepted, and accurately reflects the two-home locked decision. No change. |
| T4 | sonnet F-4 | VALID | low | Fix the cluster umbrella build-order sentence to reflect `ledger-parse` skipped. |
| T5 | sonnet F-5 | VALID | low | Replace "(Principle 5, Principle 6)" with plan-numbered principle(s); best match plan P6 (+ optionally P7), drop P5. |
| T6 | opus F-1 | VALID | low | Extend the Status enumeration to include the process cluster + four new steps, or point to the Roadmap. |
| T7 | opus F-2 | VALID | low | Indent the four increment-2 schema bullets + Rationale under increment (2). |
| T8 | opus F-3 | VALID | low | Fold the per-increment qualifier into the W3 sentence. |

Total VALID: 7 (all low). Total INVALID: 1 (T3).

All seven valid findings are plan edits to `docs/plans/agent-scaffold.md`; the planner (or the orchestrator, since these are plan-doc edits) applies them. T1 and T2 are the same terseness/de-dup class and should be done in one queue-trim pass. None warrant a re-review round on their own (all low, mechanical); an orchestrator grep/read verification of the specific edits is sufficient to confirm.
