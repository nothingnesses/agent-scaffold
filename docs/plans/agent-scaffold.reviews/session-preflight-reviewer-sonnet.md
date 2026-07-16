# Review: session-preflight

Reviewer: sonnet (doc quality, clarity, completeness)
Branch: impl/session-preflight (HEAD 6eadf66)
Diff range: 1d76f1e..HEAD
Files changed: .agents/AGENTS.reference.md, .agents/prompts/orchestrator.md, .agents/user-prompts/kickoff.md, .agents/user-prompts/resume.md, AGENTS.md, pack/AGENTS.md, pack/prompts/orchestrator.md, pack/user-prompts/kickoff.md, pack/user-prompts/resume.md

---

## Findings

### F1 - Medium: Findings-file location is ambiguous when a step is under worktree review

**Location**: `pack/AGENTS.md` (and mirrors), "Worktree lifecycle and merge-back" paragraph 1; existing "Findings files" rule (line 61 of AGENTS.md).

The new worktree lifecycle section says reviewers and the triager "need no worktree of their own; they read the branch under review." It does not say where they write their findings files. The existing "Findings files" rule says each reviewer writes to `docs/plans/<task>.reviews/`, but that rule predates the worktree model and does not say whether that path is resolved against the main repo or the worktree branch. These two rules together leave the question open.

In practice the correct answer is the main repo (so the orchestrator and other agents can read the findings without switching between trees), but a reader following only the prose cannot determine this. The orchestrator's briefing to a reviewer must already supply an absolute path to get this right, which is not documented.

The gap is most acute for the "Worktree lifecycle and merge-back" section because that section adds the worktree-review model while the "Findings files" rule that governs where findings land is silent on the cross-tree case.

### F2 - Medium: Scope creep - worktree lifecycle and merge-back model is not in the step spec

**Location**: `pack/AGENTS.md` (and mirrors), "Worktree lifecycle and merge-back" paragraphs 1 and 2; commits 46940e7 and 6eadf66.

Q-32's stated deliverables are: "a preflight section in `pack/AGENTS.md`, a pointer from `pack/user-prompts/kickoff.md` and `pack/user-prompts/resume.md`, and the orchestrator's duty in `pack/prompts/orchestrator.md`." The diff adds approximately 400 words of new doctrine across two paragraphs - the worktree lifecycle (who creates/removes the worktree, when the merge happens, where findings live) and the authored-conflict distinction (clean merge as integration vs. conflict resolution as implementation). Neither paragraph is mentioned in the step spec.

Per Principle 8, this should have been flagged to the human before being included, not added silently. The content may be correct and useful, but the step spec only asked for a Preflight section plus three update/pointer changes.

### F3 - Low: AGENTS.md Preflight rule does not specify ordering relative to state reconstruction on resume

**Location**: `pack/AGENTS.md` (and mirrors), "Preflight" paragraph; `pack/user-prompts/resume.md`.

AGENTS.md says the preflight runs "whenever a session is resumed (from the resume prompt, after a compaction or a pause)" without specifying whether it runs before or after state reconstruction from the plan and ledger. The resume.md pointer correctly orders them: "Reconstruct your state per the 'Checkpoint and resuming after context loss' section of AGENTS.md, then run the preflight... before doing any work." But a reader of AGENTS.md alone would not know this ordering.

The ordering matters: confirming what the orchestrator will adhere to is more precise after the orchestrator has read the plan's Status line and knows where the work is. The AGENTS.md Preflight rule should state "on resume, after reconstructing state" rather than leaving the ordering implicit in the resume.md pointer.

### F4 - Low: Abandoned-step worktree clean-up path is unspecified

**Location**: `pack/AGENTS.md` (and mirrors), "Worktree lifecycle and merge-back" paragraph 1.

The lifecycle describes a linear happy path: create worktree -> implement -> review loops -> converge -> merge -> remove. There is no instruction for the case where a step is abandoned before convergence - for example because the plan is revised to drop that step, or because the human decides to cancel. The worktree would be left orphaned with no documented clean-up path. A reader following only this prose cannot determine what to do with an active worktree when a step does not converge.

### F5 - Nit: "Defined there" in resume.md is ambiguous; kickoff.md is explicit

**Location**: `pack/user-prompts/resume.md` (and `.agents/user-prompts/resume.md`).

The phrase "run the preflight defined there" requires resolving "there" back to "AGENTS.md" two clauses earlier. The kickoff.md pointer uses the explicit form "run the preflight defined in `AGENTS.md`." Instructions that agents must follow should be explicit. Using the same explicit form in resume.md removes any ambiguity about the antecedent.

### F6 - Nit: orchestrator.md restates isolation tiers; AGENTS.md Preflight says to reference the rule, not restate it

**Location**: `pack/prompts/orchestrator.md` (and `.agents/prompts/orchestrator.md`), new preflight paragraph; `pack/AGENTS.md` "Preflight" section, item (2).

AGENTS.md's Preflight rule says the orchestrator should detect and state the available tier "referencing that rule rather than restating the tiers." The new orchestrator.md paragraph tells the orchestrator to "detect and state the writer-isolation tier actually available in this harness (container via agent-box / agent-images if wired, else a worktree, else the file-safety fallback)" - restating the full tier list inline in the prompt.

These serve different purposes (AGENTS.md instructs on preflight output; orchestrator.md gives the agent detection context), so they are not logically contradictory. But together they create a tension an agent following both instructions might notice: it is told both "here is the tier list" and "reference the rule rather than restate the tiers." The "referencing rather than restating" clause in AGENTS.md would benefit from an explicit scope qualification - it applies to the human-facing preflight output, not to the agent's own prompt context.

---

## Non-findings

**README not updated**: The README mentions kickoff.md and resume.md in a file-tree listing but does not describe workflow internals in detail. Not updating it for the preflight is correct; the kickoff/resume prompts are the entry points and they are updated. No action needed.

**Non-pack file changes (.agents/ and root AGENTS.md)**: The diff updates the repo's instantiated copies (.agents/AGENTS.reference.md, .agents/prompts/orchestrator.md, .agents/user-prompts/kickoff.md, .agents/user-prompts/resume.md, AGENTS.md) alongside the pack templates. This is correct because the build system re-embeds the pack into the binary but does not overwrite existing AGENTS.md on disk (it only creates absent files). Manual sync of the instantiated copies is the right approach for dogfooding, and the synced content is identical to the pack template expansions. This is not scope creep.

**Preflight coverage of all three Q-32 actions**: The Preflight paragraph in AGENTS.md correctly covers all three actions from Q-32: restate disciplines (1), detect and state the isolation tier (2), confirm with human (3). The standing directive addition (worktree-first writer isolation) is within Q-32's "folds in the human's standing directive" language. No gap.

**Discoverability/consistency of pointers**: kickoff.md, resume.md, and orchestrator.md all consistently point at the Preflight rule in AGENTS.md. The AGENTS.md Preflight paragraph references "the kickoff prompt" and "the resume prompt" back at the user-prompts. Cross-references are consistent and bi-directional.

---

## Summary

Critical: 0. High: 0. Medium: 2 (F1 findings-file location during worktree reviews; F2 unscoped worktree lifecycle doctrine). Low: 2 (F3 preflight-vs-reconstruction ordering implicit; F4 abandoned-worktree path unspecified). Nit: 2 (F5 "defined there" pronoun; F6 "referencing vs. restating" tension).
