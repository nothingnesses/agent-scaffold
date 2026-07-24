2 FINDINGS

---

## B1

Severity: Info
Location: AGENTS.md:104 (rendered Preflight paragraph); pack/AGENTS.md:104 (template source, `{{recommendation_rule}}` embedded mid-paragraph); src/recommendation_rule.rs:34 (fragment const)

Description: "settled here once" is inaccurate in the Preflight render target. The fragment reads "The human-input contract's presentation format is settled here once and rendered from this single source." In the Preflight paragraph (AGENTS.md:104), "here" is read by a document reader as pointing to the Preflight, which is not where the format is settled. The format is settled in the Human-input contract paragraph (AGENTS.md:41). At that first render target (the contract paragraph), "here" correctly names the contract as the definition location. At the second render target (the Preflight), "here" incorrectly names the Preflight as the definition location.

Why this is a genuine defect and not a preference: the module docstring (recommendation_rule.rs:17-22) explains the design strategy as "the fragment NAMES itself as 'the human-input contract's presentation format' rather than pointing 'above' or 'below', so it reads correctly BOTH ... inside the contract paragraph ... and standalone in the Preflight restatement." That reasoning applies to the naming strategy (using "the human-input contract's presentation format" as a noun instead of a directional pointer), but does not address "settled here once." "Here" is still a spatial locative; it cannot simultaneously mean "in the contract paragraph" and "in the Preflight."

Why the isolation_policy precedent does not resolve this: the isolation_policy fragment is a standalone slot (pack/AGENTS.md:91 is the line `{{isolation_policy}}` on its own line, rendered into its own standalone paragraph starting "Who isolates is settled here once..."). In that standalone paragraph, "here" = the isolation policy's own definition paragraph, which is correct. The recommendation_rule's Preflight slot is embedded mid-paragraph in the Preflight, not a standalone definition slot. The precedent applies to target 1 (the contract paragraph, also a definition-adjacent position), not to target 2 (the Preflight restatement).

Evidence:
- AGENTS.md:41: "Human-input contract (how every decision is put to the human). The human-input contract's presentation format is settled here once..." - "here" = contract paragraph (correct, the contract IS defined here).
- AGENTS.md:104: "...and (3) confirms with the human, per the human-input contract, what it will do to adhere, before proceeding. The human-input contract's presentation format is settled here once..." - "here" = Preflight (incorrect, the format is not settled in the Preflight; the Preflight restates it).
- pack/AGENTS.md:91: standalone line `{{isolation_policy}}` (isolation_policy has its own slot; recommendation_rule does not at the Preflight).

Suggested direction: the cleanest fix would be changing the fragment's wording at the Preflight slot to read "as settled in the human-input contract above" or similar - but that requires either splitting the fragment into two variants (one per slot) or introducing a conditional in the render path, which adds complexity that may exceed the benefit for an Info finding. The current wording is liveable but a future Q should reconcile it. Changing the fragment or splitting it would also require updating the unit test at src/recommendation_rule.rs:58, which pins "settled here once and rendered from this single source." Serves P1 (cleaner long-term architecture) - a two-variant fragment or a reworded standalone phrase would remove the inaccuracy.

---

## B2

Severity: Info
Location: AGENTS.md:65 ("numbered Project Principles") vs. AGENTS.md:41 and AGENTS.md:104 ("Project Principles by name")

Description: Three different phrasings now coexist for the same instruction (cite the plan's Project Principles explicitly):
- AGENTS.md:41 and AGENTS.md:104 (new fragment): "judged against the plan's Project Principles by name"
- AGENTS.md:43 and AGENTS.md:57 (unchanged, Socratic and round-cap escalation paragraphs): "Principle-judged reasoning" (shorthand)
- AGENTS.md:65 (unchanged, design explorations paragraph): "each option's trade-offs judged against the numbered Project Principles"

The substantive inconsistency is between "by name" and "numbered Project Principles" at the full-form level. These are compatible in practice (the principles are both numbered and named, so an agent following either instruction ends up citing them the same way), but they are different descriptions. An agent reading only the design-explorations paragraph learns to cite "numbered Project Principles" while the canonical contract says "by name." The Socratic and round-cap shorthands ("Principle-judged reasoning") are acceptable abbreviations and do not conflict.

Why valid: the grep at AGENTS.md:65 shows the unchanged line, which explicitly says "numbered Project Principles," while the fragment at AGENTS.md:41 and AGENTS.md:104 says "by name." Both refer to the same practice but a reader comparing them sees inconsistent language for the same directive.

Q-60 scoped reconciling the other mentions out: this is a known artifact of the step's scope, not an unexpected defect. Acceptable for now; flagged so the follow-on reconciliation step has a concrete location list.

Suggested direction: in a follow-on step, update AGENTS.md:65 ("numbered Project Principles") to "Project Principles by name" to match the canonical fragment. No code change required beyond pack/AGENTS.md (pack/AGENTS.md:65 is the source; AGENTS.md:65 re-scaffolds from it). Serves P8 (structured data first, project for humans) - the projection should be consistent across all occurrences.

---

## Not findings (recorded for audit completeness)

Grammatical splice at both render targets: clean. The fragment is a complete sentence; sentence boundaries and capitalization are correct at both the contract paragraph (AGENTS.md:41) and the Preflight (AGENTS.md:104). "This one format" at the contract paragraph and "The standing directive it establishes" at the Preflight both connect without grammatical error. The "it" in "The standing directive it establishes" has a theoretically ambiguous antecedent (the fragment's subject vs. the Preflight) after insertion, but content resolves it immediately ("every spawned agent runs isolated" is the Preflight's directive, not the presentation format's) and is not a finding.

Faithfulness: the new fragment preserves all five semantic elements of the removed inline rule - the viable options or approaches, the trade-offs of each, a recommendation, the reasoning, and the reasoning judged against the Project Principles. The wording change from "numbered Project Principles" to "Project Principles by name" is covered under B2; no element is lost.

ASCII-cleanliness: `grep -nP '[^\x00-\x7F]'` over src/recommendation_rule.rs, pack/AGENTS.md, and AGENTS.md returned no matches. The fragment, its module, the pack source, and the rendered output are all ASCII-clean. No em-dashes, en-dashes, unicode symbols, or emoji were introduced.

rustfmt hunks in src/main.rs: accepted per standing Q-57 decision; not reviewed as findings.
