# Review: human-review-queue step (3599756..fbd3ffa)

Lens: cross-document consistency and principles. Pack source files only.

---

## S1 - medium

**Location:** `pack/AGENTS.md` line 231 (Checkpoints paragraph); `pack/plan-template.md` line 36 (Open Questions placeholder).

**Problem:** The queue item field list (stable id, one-line ask, status, pointer) appears in full in both files. AGENTS.md names all four fields, then adds "(the plan template defines the item format)". The template then defines the same four fields again, plus enum values and pointer detail. The parenthetical note claims the template is the canonical format source, but AGENTS.md has already given a reader the complete field list before reaching that note. The intended split (AGENTS.md = behavior, template = format) is not realized: field names live in both files. If a field is renamed in the template, AGENTS.md must also be updated, and the two can drift independently.

The plan step detail for `human-review-queue` explicitly sets this as the validation criterion: "the queue's format and the required push step are stated once and consistently." That criterion is not met.

**Principle:** 16 (one source of truth).

---

## S2 - low

**Location:** `pack/plan-template.md` line 36, last sentence of Open Questions placeholder: "The orchestrator updates this queue at every checkpoint and pushes the open items to you."

**Problem:** This is behavioral instruction to the orchestrator placed inside a format-definition placeholder. The same rule is already the authoritative statement in `pack/AGENTS.md` (Checkpoints paragraph) and `pack/prompts/orchestrator.md` (the checkpoint paragraph). The template's job is to define item format for the planner; it is not the right location for orchestrator behavior. In practice the rule propagates into every concrete plan's Documentation Protocol section (as it did in `docs/plans/agent-scaffold.md` lines 36 and 73), creating per-plan copies. If the push behavior ever changes (cadence, trigger, or scope), those copies need updating alongside the canonical AGENTS.md, which is exactly the fragility Principle 16 targets. A template note pointing to AGENTS.md for the behavioral rule ("maintained per the push-at-checkpoint rule in AGENTS.md") would avoid the replication without losing the informational context for the planner.

**Principle:** 16 (one source of truth).

---

## S3 - low

**Location:** `pack/user-prompts/kickoff.md` line 19 vs `pack/AGENTS.md` line 233 and `pack/prompts/orchestrator.md` line 97.

**Problem:** The autonomous cadence option is named "run autonomously to acceptance" in kickoff.md but "run autonomously through to acceptance" in both AGENTS.md and orchestrator.md. The word "through" is dropped in the human-facing kickoff prompt. The meaning is clear either way, but an orchestrator whose kickoff says "run autonomously to acceptance" matching against the phrase "run autonomously through to acceptance" in its guidance may treat them as distinct tokens. Keeping the exact phrase consistent across all three documents would eliminate the ambiguity entirely.

**Principle:** 16 (one source of truth for vocabulary); no numbered principle maps precisely, but consistent naming is a prerequisite for unambiguous behavior.

---

## Not raised

- **Line length / wrapping**: per standing convention, not a finding.
- **Principle-numbering leaks**: none found. All references use generic wording ("the numbered Project Principles") rather than citing AGENTS.md principle numbers directly.
- **Status enum drift**: the enum values (`open`, `decided -> folded into <slug>`, `superseded`) appear only in the template (and the derived plan), not in AGENTS.md or orchestrator.md. AGENTS.md deliberately defers to the template for the full format, so this is consistent with the intended split, not a drift. An orchestrator reading only AGENTS.md would not see the enum, but it also reads the plan, which carries the enum in the Documentation Protocol section after plan creation. No finding.
- **Two "Checkpoint" sections in AGENTS.md**: "Checkpoints (the human-decision queue and progress)" (new, line 231) and "Checkpoint and resuming after context loss" (existing, line 277), with a forward reference ("The compaction checkpoint below...") spanning the intervening "File safety and durability" and "Writer isolation" sections. Mildly awkward navigation, but the reference is unambiguous and the sections are conceptually distinct. Below the threshold for a finding.
- **Autonomous mode and queue push interaction**: kickoff.md says autonomous means "interrupted only for decisions that need you"; the Checkpoints section says the queue push happens at every checkpoint regardless of cadence. These are consistent (the push is the mechanism that creates a decision interrupt) and no finding is needed.
