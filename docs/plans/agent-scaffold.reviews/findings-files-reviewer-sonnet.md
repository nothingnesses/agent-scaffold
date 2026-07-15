# Review findings: `findings-files` step

Reviewer: sonnet (cross-document consistency and principles lens) Commit range: `4ebe8a3..a8a75f5`

---

## Finding 1 - medium

**Location:** `pack/AGENTS.md` "Findings files" paragraph vs `pack/prompts/reviewer.md` new paragraph (both added in this diff).

**Problem:** The two documents give different naming tokens for reviewer output files, and reviewer.md's token is internally ambiguous.

`pack/AGENTS.md` uses `<step>-<role>.md` as the example: "for example `<step>-<role>.md` for a reviewer". `pack/prompts/reviewer.md` uses `<step>-<your-role-or-model>.md`: "Put them in `docs/plans/<task>.reviews/<step>-<your-role-or-model>.md`". The tokens are different words for the same slot.

This matters in the parallel-reviewer scenario the feature is designed to handle. The stated motivation in the plan step is that "parallel writers never contend for one file." When two reviewer agents run in parallel with different models, AGENTS.md's `<step>-<role>.md` would lead both to name their file identically (e.g., `step-reviewer.md`), reproducing the exact contention the feature intends to prevent. reviewer.md's `<your-role-or-model>` is intended to allow differentiation by model, but it is itself ambiguous: "role-or-model" could mean "use one or the other" rather than "combine them", so an agent might reasonably pick just one and still collide. Neither document resolves this unambiguously.

An orchestrator reading AGENTS.md to understand the filename format it will see in the ledger, or a human trying to predict output paths, gets a different answer than an agent reading reviewer.md.

**Principle (plan):** Plan Principle 1 (prefer the cleaner long-term architecture, which requires internal coherence). The inconsistency also violates the "one source of truth" rule (AGENTS.md Principle 16, which the plan inherits) since the reviewer filename format is stated differently in two places.

---

## Finding 2 - low

**Location:** `pack/AGENTS.md`, Triager role bullet (the changed line in the diff).

**Problem:** The reviewer bullet and the triager bullet are treated asymmetrically with respect to the forward-reference to the "Findings files" subsection.

The reviewer bullet (as changed) reads: "report each finding, with a severity and concrete evidence, to its own findings file (see Findings files below)". The triager bullet (as changed) reads: "returning a verdict for each to its own file". The triager also writes to a findings file per the same subsection and per `pack/prompts/triager.md`, but the triager bullet carries no "(see Findings files below)" pointer. A reader scanning role bullets to understand output conventions is directed to look up the convention for the reviewer but not for the triager.

**Principle (plan):** Plan Principle 1 (internal coherence; a symmetric convention stated asymmetrically is harder to maintain and easier to misread).

---

## No findings of high or critical severity.

---

## Not raised (confirmed non-issues)

- Placement of "Findings files" between "Preventing relitigation (the ledger)" and "Checkpoints" is coherent: the subsection explains how findings feed the ledger, so the transition is logical.
- The split between AGENTS.md (mechanism overview) and each role prompt (role-specific instruction) is clean. orchestrator.md restates the directory and cleanup rule, but that is role-specific content (the orchestrator owns cleanup) and mirrors the existing pattern established by `file-safety-rules`. No free-standing full-mechanism duplication that would drift independently.
- Principle numbering: no principle numbers are cited in any of the changed pack files, so no mismatch to report.
- The triager and orchestrator filenames are consistent across all four changed files (`<step>-triage.md` in AGENTS.md and triager.md; "per-agent files under `docs/plans/<task>.reviews/`" in orchestrator.md without specifying a format, which is appropriate since the orchestrator reads rather than writes reviewer files).
