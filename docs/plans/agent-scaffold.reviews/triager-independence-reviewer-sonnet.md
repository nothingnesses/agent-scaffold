# Review: triager-independence (diff 0c95ab9..bd483e1)

Reviewer: claude-sonnet-4-6 (independent)

## S1 — HIGH: Cross-document inconsistency on the "(or a human)" escape valve

**Location**: `pack/prompts/orchestrator.md` paragraph 1 vs. `pack/AGENTS.md` Triager role bullet (also reflected identically in root `AGENTS.md` and `.agents/prompts/orchestrator.md`)

**Evidence**: `orchestrator.md` (new text) reads: "The triager is the exception: it is always a separate agent **(or a human)**, independent of both the producer and you, for every review round, never played by you." In contrast, `AGENTS.md`'s Triager role bullet reads: "The triager is always a separate agent, independent of both the agent that produced the artifact under review and the orchestrator, for every review round including trivial ones; it is never collapsed into another role." No "(or a human)" appears there, nor in `triager.md`'s new first paragraph.

The canonical document `AGENTS.md` — which every agent is instructed to read first — states a stricter rule than `orchestrator.md` actually enforces. An agent following only `AGENTS.md` would conclude that a human acting as triager is not permitted, because a human is not "a separate agent." An agent reading `orchestrator.md` sees the opposite. Under Principle 1 (coherence: the same rule stated the same way in every document), this is a direct violation. The "(or a human)" clause is also present in the convergence backstop ("a second, independent triager (or a human)") but absent from the main triager rule in `AGENTS.md`. The canonical document is therefore internally inconsistent with itself on whether a human may substitute.

---

## S2 — HIGH: Undefined behavior on a single-agent, no-human harness

**Location**: `pack/AGENTS.md` opening role-separation paragraph; `pack/prompts/orchestrator.md` paragraph 1

**Evidence**: Before this change, the no-sub-agents fallback read: "Where sub-agents are unavailable, one agent plays the roles in sequence but still writes down each role's output, so the separation holds on paper." This covered single-agent harnesses. The new text in `orchestrator.md` reads: "Where sub-agents are unavailable, perform the other roles yourself in sequence... The triager is the exception: it is always a separate agent (or a human), independent of both the producer and you, for every review round, never played by you." The phrase "never played by you" explicitly prohibits the old fallback for the triager, but no replacement behavior is stated for the case where neither a sub-agent nor a human is available.

A harness with only one agent and no human present is a real configuration (automated pipelines, certain offline harnesses). On such a harness, the workflow would reach the review-then-triage step and have no defined path forward: the orchestrator cannot play the triager, no sub-agent exists, no human is mentioned as available. The document says "never played by you" but not "escalate until one is found," "block," "error," or any alternative. This reachable state now has no defined outcome. Principle 5 (make illegal states unrepresentable) and Principle 1 (coherence) are violated: the state is reachable and representable but has undefined behavior.

---

## S3 — MEDIUM: Asymmetry within AGENTS.md between backstop triager and main triager on human substitution

**Location**: `pack/AGENTS.md` Convergence section (backstop paragraph, unchanged) vs. Triager role bullet (new text)

**Evidence**: The backstop reads (unchanged by this diff): "have a second, independent triager **(or a human)** confirm the dismissal." The new Triager role bullet reads: "The triager is always a separate agent" — no "(or a human)." Within the same canonical document, the backstop explicitly permits a human for the secondary triager, while the main triager rule does not extend this permission. No reasoning is given for the asymmetry.

If a human is acceptable as a backstop triager (which the document explicitly states), there is no principled reason to exclude them from the primary triager role. The asymmetry creates an implicit rule: the primary triager must be an LLM agent, but the backstop triager may be human. This is probably not the intended distinction — it looks like an omission — but because `AGENTS.md` is canonical, an agent following it literally would treat a human as an invalid primary triager and only invoke them for the backstop. The interaction with S1 is direct: orchestrator.md fixes this with "(or a human)" but AGENTS.md does not, so the canonical document is asymmetric in a way that looks unintentional.

---

## S4 — LOW: Rule stated twice within AGENTS.md with no clear single source

**Location**: `pack/AGENTS.md` opening role-separation paragraph (new sentence) and Triager role bullet (new text)

**Evidence**: The opening paragraph adds: "The triager is the one exception to collapsing: it is always a separate agent, never merged into the producer or the orchestrator, even for a trivial or low-risk review round (see the Triager role below)." The Triager role bullet then states: "The triager is always a separate agent, independent of both the agent that produced the artifact under review and the orchestrator, for every review round including trivial ones; it is never collapsed into another role." Both sentences are normative. The forward reference "(see the Triager role below)" acknowledges the duplication but does not resolve it; neither sentence clearly defers to the other as the authoritative statement.

Principle 16 (one source of truth) applies: having two normative statements of the same rule in one document creates a maintenance risk where they can diverge on a future edit. The phrasing is already slightly different ("never merged into the producer or the orchestrator" vs. "independent of both... and the orchestrator"), which could be read as two subtly different constraints. The opening paragraph's "never merged" speaks to structural role collapse; the Triager bullet's "independent of" speaks to relationship. They mean the same thing, but the difference in phrasing could invite interpretation. The forward reference should either make one normative and the other a pointer, or both should be identical.

---

## S5 — LOW: Backstop's "independent" now has ambiguous referent scope

**Location**: `pack/AGENTS.md` Convergence section, backstop paragraph (unchanged text, but meaning affected by new rule)

**Evidence**: The backstop reads: "have a second, **independent** triager (or a human) confirm the dismissal." Before this change, "independent" in context meant "a second opinion from a different agent than the first triager." After this change, the new general rule says all triagers must be independent of the producer AND the orchestrator. The word "independent" in the backstop now has ambiguous scope: does it mean (a) independent of the first triager only (the pre-change meaning), or (b) also independent of the orchestrator (by inheritance from the new general rule)?

If (a), the backstop's second triager has a weaker independence requirement than the general rule specifies — it need only be a second opinion, not also separate from the orchestrator.
If (b), the word "independent" now silently inherits two dimensions of independence without stating either, making the constraint under-specified and hard to audit.

The backstop text was not updated by this diff. A minimal fix would be to change "a second, independent triager (or a human)" to "a second triager (or a human), itself also independent of the orchestrator" or similar, to make both independence constraints explicit. Low impact because a careful reader will apply the general rule, but the unchanged backstop text is now imprecise.

