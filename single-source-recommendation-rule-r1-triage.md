# Triage: single-source-recommendation-rule (Q-60), round 1

2 valid (of which 1 blocking), 0 invalid

Backstop re-check: no HIGH or CRITICAL findings were raised or dismissed by either reviewer (reviewer A: zero findings; reviewer B: two Info). Nothing to backstop. Confirmed.

---

## B1

- Verdict: VALID.
- Final severity: Info (unchanged).
- Convergence: BLOCKING (recommend fixing this round).

Evidence (verified against the rendered `AGENTS.md` at both slot locations):
- The one fragment (`src/recommendation_rule.rs:34`) opens "The human-input contract's presentation format is settled here once and rendered from this single source...".
- Slot 1, `AGENTS.md:41` (human-input contract paragraph): "Human-input contract (how every decision is put to the human). The human-input contract's presentation format is settled here once...". "here" = the contract paragraph, which is exactly where the contract is defined. Correct.
- Slot 2, `AGENTS.md:104` (Preflight): "...(3) confirms with the human, per the human-input contract, what it will do to adhere, before proceeding. The human-input contract's presentation format is settled here once...". "here" points at the Preflight, which restates the contract but is NOT where the format is settled. The Preflight paragraph itself follows a restate-do-not-redefine pattern (it says of the isolation tiers "referencing that rule for the tier policy rather than re-defining it"), so a "settled here" claim inside the Preflight contradicts the section's own posture. Loose/inaccurate.
- Precedent does not cover it: `pack/AGENTS.md:91` is a standalone `{{isolation_policy}}` line rendering "Who isolates is settled here once..." into its OWN paragraph (`AGENTS.md:91`), so its "here" = its own definition paragraph (correct). The recommendation_rule Preflight slot is embedded mid-paragraph, not a standalone definition slot, so the precedent covers slot 1 (definition-adjacent) but not slot 2.
- The module docstring (`src/recommendation_rule.rs:17-22`) claims the fragment "reads correctly BOTH inside the contract paragraph ... and standalone in the Preflight restatement." The "names itself" strategy handles the directional-pointer problem, but "settled here once" is a residual spatial locative the docstring's reasoning does not address; it is the one word that undermines the two-context-correctness property this step exists to establish.

Why BLOCKING (judged against the plan Principles):
- The whole point of Q-60 is one canonical source that reads correctly in BOTH render contexts. A known-wrong word in the single canonical deliverable, on a RISKY step, is worth removing.
- Reviewer B's stated objection to fixing (splitting the fragment into two variants or a render conditional) is moot: the minimal fix is a one-word deletion, no split, no conditional. With that objection gone, the cost is trivial and the benefit is directly on the step's core property. Serves P1 (cleaner long-term architecture: the canonical text is accurate in both slots) at near-zero cost and full reversibility.

Recommended action (exact minimal fix): drop the word "here".

1. `src/recommendation_rule.rs:34`, in `RECOMMENDATION_RULE_FRAGMENT`, change:
   - from: `The human-input contract's presentation format is settled here once and rendered from this single source, so its copies cannot drift:`
   - to:   `The human-input contract's presentation format is settled once and rendered from this single source, so its copies cannot drift:`
2. `src/recommendation_rule.rs:58`, byte-guard substring, change:
   - from: `.contains("settled here once and rendered from this single source")`
   - to:   `.contains("settled once and rendered from this single source")`
3. Re-run `just scaffold-self` to regenerate `AGENTS.md` and `.agents/AGENTS.reference.md` so both `{{recommendation_rule}}` slots re-render and the byte-guard/drift tests (`the_committed_scaffold_carries_the_recommendation_rule_fragment`, `agents_md_drift`) pass.
4. Optional consistency touch (not test-pinned, not required): the doc comment at `src/recommendation_rule.rs:28` reads "settled once here and rendered from this single source"; drop its "here" too so the prose matches the const.

Reads correctly in both targets after the change: slot 1 "...format is settled once and rendered from this single source" (settled a single time; still true and now not tied to a spatial "here"); slot 2 same text, no false claim that the Preflight settles the format. The `isolation_policy` fragment keeps "settled here once" because its single slot is standalone and its "here" is correct; the divergence is justified by the different slot arrangement, not arbitrary.

---

## B2

- Verdict: VALID (as an observation) and correctly out-of-scope.
- Final severity: Info (unchanged).
- Convergence: NON-BLOCKING (accepted out-of-scope residual, flagged for follow-on).

Evidence:
- `AGENTS.md:65` (design-explorations paragraph, untouched; source is `pack/AGENTS.md:65`): "each option's trade-offs judged against the numbered Project Principles".
- The fragment at `AGENTS.md:41` and `AGENTS.md:104`: "judged against the plan's Project Principles by name".
- These are two descriptions of the same practice (the principles are both numbered and named), so an agent following either cites correctly; it is a wording inconsistency, not a behavioural one. The Socratic/round-cap shorthand "Principle-judged reasoning" (`AGENTS.md:43`, `AGENTS.md:57`) is an acceptable abbreviation and does not conflict.

Why out-of-scope (confirmed genuinely out of scope):
- Line 65 is a directive about what an exploration document must contain, not a recommendation-in-options human-input-contract restatement slot, so it is not one of the two `{{recommendation_rule}}` render targets this step single-sources.
- Q-60 explicitly scoped reconciling the other woven Principle-citation mentions OUT of this step. B2 correctly records the concrete location (`pack/AGENTS.md:65` -> `AGENTS.md:65`) for the follow-on reconciliation step; no action owed in this step. Serves P8 (consistent projection) once the follow-on lands.

Recommended action: none this step. Carry as a residual; the follow-on reconciliation updates `pack/AGENTS.md:65` "numbered Project Principles" -> "Project Principles by name" and re-scaffolds.

---

## Round-1 outcome recommendation

NEW VALID FINDINGS.

B1 is a valid Info finding whose one-word fix directly serves this RISKY step's core two-context-correctness property at trivial, reversible cost, so it is owed this round (round is new-valid; consecutive-clean streak does not advance). B2 is a valid but out-of-scope Info residual (Q-60 scoped it out), accepted as-is.

Exact minimal fix list (B1 only):
1. `src/recommendation_rule.rs:34`: delete "here" from the fragment ("settled here once" -> "settled once").
2. `src/recommendation_rule.rs:58`: update byte-guard substring ("settled here once and rendered from this single source" -> "settled once and rendered from this single source").
3. `just scaffold-self` to re-render both slots and satisfy the drift/byte-guard tests.
4. Optional: `src/recommendation_rule.rs:28` doc comment, drop "here" for prose consistency (not test-pinned).
