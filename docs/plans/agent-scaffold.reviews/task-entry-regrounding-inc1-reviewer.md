# Review: task-entry-regrounding-inc1 (Part A prose discipline)

Reviewer: independent adversarial reviewer
Diff range: main..impl/ter-inc1 (main b949a1c, branch HEAD a611491)
Worktree: /home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/ter-inc1

## Findings

### I1-1 - medium

**File:** `pack/prompts/orchestrator.md:23` (and `.agents/prompts/orchestrator.md:23`)

**What is wrong:** The orchestrator pointer restates the brief's four-element content list. The pointer says:

> "brief the step from durable artifacts (what it is, why it exists, the cited evidence, and what you are about to do)"

The same four elements appear verbatim in the AGENTS.md subsection at `pack/AGENTS.md:104`:

> "it re-grounds the step in a short brief: what the task is, why it exists (its provenance), the cited evidence, and what it is about to do"

These are two sources for the brief's scope. If the brief's content list changes in the AGENTS.md subsection (for example, adding a fifth element), the orchestrator pointer would be stale, stating a different scope.

**Additional evidence of self-contradiction:** The pointer immediately follows this enumeration with "rather than restating that discipline here" - an instruction not to restate, placed right after restating the brief's contents. This is an internal contradiction in the pointer itself.

**Why the build plan prescribed less:** The build plan (`docs/plans/task-entry-regrounding.build-plan.md:40`) prescribed the gloss as "(brief from durable artifacts + go/no-go per the human-input contract, scaled to stakes)" - without expanding the brief's elements. The implementer went beyond that and added the four-element enumeration.

**Why it matters:** Principle 8 (one source of truth). The brief's content list is a load-bearing rule: it defines what the orchestrator must produce at each step entry. Having it in two places creates a drift source. A pointer to AGENTS.md need not enumerate what AGENTS.md says; it need only name the discipline and say where to find it.

---

### I1-2 - low

**File:** `pack/AGENTS.md:104`

**What is wrong - inconsistent citation style:** Three of the four cited sections are named by their heading:
- "the human-input contract above" (references `pack/AGENTS.md:41`)
- "the 'Checkpoint and resuming after context loss' section above" (references `pack/AGENTS.md:95`)
- "the Checkpoints rule above" (references `pack/AGENTS.md:71`)

The fourth citation - the `type: "decision"` spec in the Instrumentation section - is referenced only as "the round log's `type: \"decision\"` record with the human's recorded `chosen`" without naming the "Instrumentation (metrics logging)" section (`AGENTS.md:135` in the rendered output) that defines it. A reader following this reference must independently discover that the round log spec lives in the Instrumentation section.

**What is wrong - unconditional reference to conditional artifact:** The new subsection lives in the unconditional body of `pack/AGENTS.md` (line 104, before the `{{instrument}}` placeholder at line 114). The `type: "decision"` record is defined only in the `{{instrument}}` section, which is omitted for non-instrumented scaffolds. Other places in the unconditional template that mention instrumentation-conditional behaviors qualify them with "when instrumentation is on" (e.g., `pack/AGENTS.md:61`, `pack/AGENTS.md:63`). The new subsection does not include this qualifier. A non-instrumented scaffold would render the Task-entry re-grounding section with a reference to "the round log's `type: \"decision\"` record" while the Instrumentation section (and the round log itself) would not exist.

**Why it matters:** Inconsistent citation style relative to the other three named references. In a non-instrumented scaffold, the reference is dangling. Severity is low because the dogfood repo is instrumented and the accuracy of the reference in that context is correct; this is a pack template coherence issue.

---

## Checks with no findings

**Check 1 (REFERENCE-NOT-RESTATE) - the AGENTS.md subsection itself:** The subsection cites the existing machinery rather than restating it. The human-input contract is named and not restated. The Checkpoint-and-resuming section is named and not restated; "working context is not durable" is a brief inline gloss attributed immediately to that section, not a parallel source. The Checkpoints rule is named; "the checkpoint push flushes and raises the open items as one step converges" is a brief explanatory clause establishing the entry/exit relationship, not a restatement of the queue-push rule. The stakes-scaling examples ("a low-stakes step entry is a one-line brief and an implicit go, a high-stakes one the full brief and an explicit go/no-go") are step-entry-specific applications (including the "implicit go" concept not present in the contract) rather than plain restatements. The Principle 8 violation is in the orchestrator pointer (I1-1), not the AGENTS.md subsection.

**Check 2 (ACCURACY):** Cross-references are accurate. The `type: "decision"` record does have `q_id` and `chosen` fields per the Instrumentation spec at `AGENTS.md:143`. "Phase-4 step entry" accurately identifies the implement phase (`pack/AGENTS.md:32`). The entry/exit characterization (re-grounding is entry-side, checkpoint push is exit-side) is coherent with the Checkpoints section (`pack/AGENTS.md:71-73`): checkpoint fires when a step converges (exit), re-grounding fires before the next step starts (entry). The "Checkpoint and resuming" durability framing ("working context is not durable") matches the section at `pack/AGENTS.md:95`.

**Check 3 (PLACEMENT + COHERENCE):** Correct placement: the subsection appears between Preflight (`pack/AGENTS.md:102`) and "Prose formatting" (`pack/AGENTS.md:106`), immediately after Preflight as the build plan required. It reads as the per-step sibling of the per-session Preflight. The entry/exit relationship to the checkpoint queue push is clearly stated and does not duplicate the queue-push rule.

**Check 4 (LIGHTNESS / stakes-scaling):** Clean. The subsection explicitly defers to the human-input contract for the scaling prescription and says it "does not build a separate ceremony of its own." No new escalation or gating apparatus is invented.

**Check 5 (ORCHESTRATOR-PROMPT POINTER - placement and length):** The pointer is short (one sentence added into the existing "Implement step by step" paragraph at `pack/prompts/orchestrator.md:23`), placed sensibly at the step-entry point, and explicitly names the subsection ("the task-entry re-grounding in `AGENTS.md`"). The content-list restatement is addressed in I1-1.

**Check 6 (DOGFOOD REGEN CONSISTENCY):** All three generated files match the pack source. `diff` of the Task-entry re-grounding paragraph between `pack/AGENTS.md` and `AGENTS.md` produces no output. Same for `.agents/AGENTS.reference.md`. The orchestrator pointer is byte-identical in `pack/prompts/orchestrator.md` and `.agents/prompts/orchestrator.md`. Including `.agents/AGENTS.reference.md` is correct: `src/manifest.rs` and `src/main.rs:1893` show the scaffold command generates both `AGENTS.md` and `.agents/AGENTS.reference.md` as paired outputs of the same render; the `scaffold-self` justfile recipe produces both, so including it is not an over-reach.

**Check 7 (STYLE):** No em-dashes, en-dashes, unicode arrows, or other disallowed characters found. No bullet list in the new section (bullet-punctuation rule N/A). No hard-wrap. The section heading and paragraph style match the surrounding document.
