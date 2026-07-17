# Review findings: reviewer-diversity (doc-quality lens)

Reviewer: doc-quality (sonnet) Diff range: c45e36e..959ab7a Files changed: `.agents/AGENTS.reference.md`, `AGENTS.md`, `pack/AGENTS.md` (3 files, 6 line changes)

---

## Finding 1: Plan Roadmap not updated - medium

The `reviewer-diversity` step in `docs/plans/agent-scaffold.md:165` remains `not started` after this commit. The single commit in the range (`959ab7a`) changes only the three AGENTS.md copies and does not touch the plan. AGENTS.md itself (Implementer role, line 23 of the pre-change file) says the implementer's job includes "keeping the plan's status current." The status block at line 3 of the plan also identifies `reviewer-diversity` as the "next" step and lists it as not started. Since this commit is the entire implementation (the step has no code component - it is a docs-only edit), the plan should have been updated in the same commit to mark the step complete and move the Status pointer. The durable record is now inconsistent: the commit message says the rule was generalised, but the roadmap contradicts it.

Evidence: `docs/plans/agent-scaffold.md:165` (`| reviewer-diversity | not started |`); `docs/plans/agent-scaffold.md:3` (Status narrative names it "next").

---

## Finding 2: Plan names reviewer.md and triager.md as in-scope; implementer left them unchanged without recording the scope change - medium

The step detail at `docs/plans/agent-scaffold.md:643` explicitly states the footprint as "a one-clause edit to `pack/AGENTS.md` (the Reviewers role line and the Design-explorations "different lenses or models" line), `pack/prompts/reviewer.md`, and `pack/prompts/triager.md`, then regenerate the self-scaffold."

The implementer left `pack/prompts/reviewer.md` and `pack/prompts/triager.md` unchanged. The reasoning ("neither carries a diversity clause; diversity is an orchestrator concern, canonical in AGENTS.md") is correct on documentation grounds: `reviewer.md` gives instructions to one reviewer agent and `triager.md` gives instructions to one triager agent; neither currently contains text about model or harness selection, and individual agents cannot control what model or harness they run in, so there is nothing coherent to add there. The implementer's call is defensible.

The problem is that no durable record explains the deviation. The plan's step detail still says those files should be changed. A future reader (or a re-check reviewer) who reads the plan and then diffs the commit will see a discrepancy with no explanation. The step detail should have been updated in this commit to reflect the corrected scope ("edit was scoped to `pack/AGENTS.md` only; `pack/prompts/reviewer.md` and `pack/prompts/triager.md` carry no diversity clause and were left unchanged") so the plan accurately documents what was decided. Principle 19 ("document the why") and Principle 16 ("one source of truth") both apply: the plan is the durable source, and right now it is wrong.

Evidence: `docs/plans/agent-scaffold.md:643` (scope statement); `959ab7a` stat (three files changed, neither prompt file among them); `pack/prompts/reviewer.md` and `pack/prompts/triager.md` (no diversity-related text).

---

## Finding 3: "harnesses" introduced in the diversity rule without a gloss at point of use - low

Principle 20 requires documentation to be self-contained for a reader without prior context. The term "harness" first appears in the document at line 3 ("It is harness-agnostic: any harness-specific file (for example `CLAUDE.md`) should point here") and at line 15 ("Where the harness supports independent sub-agents"). These uses give a contextual hint - a harness is an AI coding assistant CLI - but neither constitutes a definition.

All prior uses of "harness" are singular and refer to "the harness" in use, meaning the single CLI the reader is running. The new diversity rule is the first place in the document that the plural "harnesses" appears and implies a user might run multiple harnesses simultaneously. That is a new concept that the document never establishes. A reader arriving at "different models or harnesses where available" without having encountered the earlier uses would need to infer that "harnesses" means CLIs (from the CLAUDE.md example on line 3) and that different CLIs create independent blind spots (unstated).

The severity is low rather than medium because the inference chain is short and the CLAUDE.md example on line 3 does anchor the term. However, the first mention of the diversity concept would be a natural place for a parenthetical ("harnesses" meaning the CLI tools running the agents, such as Claude Code, Cursor, or Copilot) to satisfy Principle 20 without burdening the prose.

Evidence: `pack/AGENTS.md` Reviewers line (new text); `AGENTS.md:3` and `AGENTS.md:15` (existing uses, no definition).

---

## Finding 4: Rationale phrasing is ambiguous on whether same-model and same-harness are independent or conjunctive blind-spot sources - low

The new Reviewers line reads: "Prefer several reviewers with different lenses, and different models or harnesses where available, since same-model and same-harness reviewers share blind spots."

"Same-model and same-harness reviewers share blind spots" is ambiguous. Two readings are available:

(a) Reviewers who are the same on BOTH dimensions share blind spots (the conjunction is conjunctive: the condition is being same-model AND same-harness together). Under this reading, diversity on either dimension (model OR harness) breaks the correlation, which is consistent with "different models or harnesses."

(b) Same-model reviewers share blind spots, and same-harness reviewers share blind spots, as two independent claims. Under this reading, you would need diversity on BOTH dimensions to eliminate all correlated blind spots, which would imply "different models AND harnesses" in the prescription, not "or."

The prescription uses "or," so reading (a) appears to be the intent. The design record at `docs/plans/agent-scaffold.md:115` says "minimising the shared blind spots that same-model (and same-harness) reviewers share," with "and same-harness" in parentheses as an addendum, which weakly suggests reading (b) (two independent dimensions). The inconsistency between the parenthetical in the design record and the "and" conjunction in the rationale clause creates a small documentation ambiguity. A reader trying to understand when the full rule is satisfied may have to guess.

A cleaner phrasing that resolves the ambiguity for reading (a): "since reviewers sharing the same model or the same harness have correlated blind spots." For reading (b): "since same-model reviewers and same-harness reviewers each share correlated blind spots."

Evidence: `pack/AGENTS.md` Reviewers line (new text, rationale clause); `docs/plans/agent-scaffold.md:115` (Q-36 entry, parenthetical wording).

---

## Finding 5: CHANGELOG has no entry for this workflow guidance change - low

`CHANGELOG.md` follows Keep a Changelog format and states "all notable changes to this project will be documented in this file." The Unreleased section already contains a "Changed" entry for prior workflow guidance hardening: "Hardened the scaffolded workflow guidance (`AGENTS.md` and the role prompts) after a design review." That entry predates this commit.

The diversity generalization is a user-facing workflow guidance change that ships to every project that runs the scaffolder. Prior guidance changes - convergence, the review ledger, the backstop triager, the acceptance-review triager - all received CHANGELOG coverage. There is no entry for this change.

The severity is low because the change is small, it does not alter the tool's CLI behavior or output format, and it could be argued the existing Changed entry's phrasing ("workflow guidance") covers ongoing evolution. But the precedent in this project is that workflow guidance changes get explicit entries, and a user upgrading agent-scaffold who wants to understand what changed between versions has no changelog signal here.

Evidence: `CHANGELOG.md:7-17` (Unreleased section, no entry for reviewer-diversity); prior Changed entry at `CHANGELOG.md:16`.

---

## Findings at each severity not raised

**critical**: No critical findings.

**high**: No high findings.

**medium**: Findings 1 and 2 above.

**low**: Findings 3, 4, and 5 above.

---

## Assessments not raised as findings

**README consistency**: The README does not describe the reviewer diversity rule at any level of detail (it mentions "independent reviewers" in passing at line 47 without specifying model diversity). The diversity generalization therefore leaves no README wording stale. No finding.

**Implementer's call to leave reviewer.md and triager.md unchanged**: Correct on documentation grounds. Those prompts are per-agent instructions; they cannot sensibly tell a single agent "be from a different model or harness." The diversity rule is an orchestrator-level spawning concern, and AGENTS.md is the right canonical location. The decision is sound; the gap is only the absence of a plan update recording the corrected scope (Finding 2).

**ASCII cleanliness**: The changed lines contain no unicode, em-dashes, or special characters. Clean.

**Design-explorations line**: The change from "different lenses or models" to "different lenses, models, or harnesses" is the cleaner of the two edits - a simple list extension with no rationale clause to misread. No finding.

**Disambiguator convention and cross-harness scenarios**: The findings-file naming convention says each reviewer's disambiguator is "its model, or an index." Two reviewers from the same model but different harnesses would need an index rather than a model name to avoid filename collisions. The "or an index" fallback handles this correctly, so there is no gap in the guidance. No finding.
