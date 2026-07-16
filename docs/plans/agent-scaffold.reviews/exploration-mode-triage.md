# Triage: exploration-mode (Q-29), commit `9757a64`

Triager independent of producer and both reviewers. Adjudicates:

- `exploration-mode-reviewer-opus.md` (correctness/consistency): 1 low.
- `exploration-mode-reviewer-sonnet.md` (doc quality/completeness): 4 medium, 3 low, 1 nit.

Judged against `.agents/principles.toml` and the step's own stated scope (mostly docs; NO new phase, NO machine-parsed schema, NO new required role prompt; match ceremony to stakes). The step detail (`docs/plans/agent-scaffold.md`, `### exploration-mode` step) and the commit message fix the intended model: the orchestrator "spawns the explorer(s)" who write the design-notes artifact; the orchestrator coordinates, synthesises, presents options, and owns cleanup. That model is the yardstick for the contradiction findings below.

Owner for every fix: the orchestrator (it edits `pack/AGENTS.md` / `pack/plan-template.md` / `pack/prompts/orchestrator.md`, then runs `scaffold-self` to regenerate the rendered copies `AGENTS.md`, `.agents/AGENTS.reference.md`, `docs/plans/TEMPLATE.md`, `.agents/prompts/orchestrator.md`).

Line length is never a valid finding; none was raised.

---

## VALID findings

### V1 (medium) - Intake paragraph makes the orchestrator the writer of the design-notes artifact, contradicting the intended multi-explorer model

Sources: sonnet Finding 1 (root); sonnet Finding 2 (second half, the template repeats the same phrasing); sonnet Finding 7 (folds in here). Dedup: one root defect across two paragraphs and the template.

`pack/AGENTS.md` line 45 (Design-space exploration): "the orchestrator ... runs the exploration, and writes a design-notes artifact." Plainly read, the orchestrator is the writer. This contradicts:

- the Design explorations paragraph (line 59): "prefer several independent explorers ... and synthesise their proposals," with only cleanup attributed to the orchestrator;
- the plan step detail: "spawns the explorer(s)";
- the commit message: "prefer several independent explorers plus synthesis over one take";
- `pack/prompts/orchestrator.md` line 3: the orchestrator "do[es] not plan, implement, review, or triage yourself; you spawn the roles that do."

The two readings are reconcilable only if "runs the exploration / writes a design-notes artifact" is read as "drives / causes to be written," but the plain reading has the orchestrator produce an artifact that independent agents should produce. That is a role-separation break (Principle 5, independent review; Principle 16, one source of truth between the two paragraphs). An orchestrator following the intake paragraph literally writes the file itself and skips the independent-explorer step. Genuinely contradictory, not merely loose.

Severity medium: documentation-only, and the "(see Design explorations below)" cross-reference plus the plan step mitigate, but the operative forward-facing paragraph asserts the wrong actor for a core workflow invariant.

Recommended fix (in `pack/AGENTS.md`, both intake and template prose): reword so the orchestrator records the `exploring` item and spawns the explorer(s), the explorer(s) write the design-notes artifact, and the orchestrator synthesises, presents options via the human-input contract, and owns cleanup. While rewording, fold in sonnet Finding 7: write the transition as "moves the item from `exploring` to `open` so the human can decide," so it cannot be read as skipping the human's choice before closing `exploring`.

### V2 (medium) - Explorer is an unnamed writer: file-safety / isolation coverage and parallel-collision handling are missing

Source: sonnet Finding 3.

An explorer writes `docs/plans/<task>.explorations/<question>.md`, so it is a spawned writer, but `pack/AGENTS.md` defines "writer agent" as a spawned writer role and lists only the planner and implementer; the file-safety rules ("clean tree before a writer") and writer-isolation rules key off "writer agent." An explorer therefore falls outside the discipline that Principle 18 (least privilege / least authority) motivates. Separately, several explorers running in parallel (which the Design explorations paragraph prefers) would collide on the single question-named file: the findings-files convention avoids collisions with per-writer disambiguators, but the explorations convention names the file only for the question. Both gaps are concrete: uncontained writer blast radius, and lost work from a parallel clobber.

Scope guard: the step excluded a new required role prompt, so DO NOT add an explorer prompt or a new role to the roles list. The fix is prose within the existing Design explorations paragraph, no new prompt, no new phase.

Recommended fix (in `pack/AGENTS.md`, Design explorations paragraph): state that explorers are writer agents subject to the file-safety and writer-isolation rules, and specify parallel handling: give each parallel explorer a distinct filename (as the findings-files convention disambiguates reviewers) and have the orchestrator synthesise into the question-named file, or run explorers sequentially. One or two sentences.

### V3 (medium) - `pack/prompts/orchestrator.md` was not updated; its Socratic clause competes with the new mode

Source: sonnet Finding 4.

`pack/prompts/orchestrator.md` line 27: "If the human drives by asking a question rather than giving a task, answer with the same contract and record the resolved answer as a durable Open-Questions decision." This is the Socratic branch: answer immediately with the contract. Exploration mode requires the opposite for a not-yet-decidable / deliberation request: record `exploring`, spawn explorer(s), then present options. A deliberation request looks like a question, so the proximate orchestrator-prompt instruction routes it to immediate-answer Socratic mode, competing with (and, read strictly, suppressing) the new mode. The orchestrator prompt is the operative instruction the orchestrator executes; a Success Criterion requires "pack/AGENTS.md, the prompts under pack/prompts/ ... state the same rules the same way," so the prompt currently contradicts AGENTS.md.

Scope: this updates an EXISTING prompt (the Socratic branch already lives there at line 27), parallel to how Socratic mode was reflected in the prompt. It is not a new role prompt, so it is in scope.

Recommended fix (in `pack/prompts/orchestrator.md`, the line-27 question-handling clause): split the question case: a decidable question is answered with the contract (Socratic, as now); a not-yet-decidable / deliberation question, or one whose design space the orchestrator judges genuinely open, is recorded as an `exploring` item, the explorer(s) are spawned, options are then presented via the contract. Keep it brief and defer detail to AGENTS.md.

### V4 (low) - Plan-template omits the orchestrator-initiated trigger for `exploring`

Source: sonnet Finding 2 (first half).

`pack/plan-template.md` line 24: "use it when the human asks to deliberate rather than to decide." `pack/AGENTS.md` line 45 gives two triggers: the human asks to deliberate, OR the orchestrator finds a decision's design space genuinely open. The template omits the second. Downgraded from the reviewer's medium to low: the template is a terse inline summary that defers to AGENTS.md ("The orchestrator maintains this queue under the push-at-checkpoint rule in AGENTS.md"), and AGENTS.md carries the authoritative two-trigger contract the orchestrator reads, so the impact is limited. Still a cheap consistency fix worth making.

Recommended fix: either add the orchestrator-initiated trigger to the template clause ("or when the orchestrator judges a decision's design space too open to present options"), or keep the template terse and drop the trigger enumeration entirely, deferring to AGENTS.md (avoids duplicating the trigger contract in two places, Principle 16). Either resolves it; I lean to the second to keep one source of truth.

### V5 (low) - Template's pointer-target enumeration does not include the exploration artifact

Source: opus Finding 1. Same template line as V4; fix in the same edit.

`pack/plan-template.md` enumerates each item's pointer as "to the step or ledger that carries the detail," but an `exploring` item points at the exploration file under `docs/plans/<task>.explorations/`, which is neither the step nor the ledger. No validator impact (pointer targets are never validated, for any status), purely documentation precision: a reader following the template alone would not learn where an `exploring` item's pointer goes.

Recommended fix: generalise the enumeration to "the step, the ledger, or the exploration that carries the detail" (or add the exploration as a third target). Fold into the V4 template edit.

### V6 (nit) - Cleanup timing for explorations is less specified than for findings files

Source: sonnet Finding 8.

Findings files state explicit delete triggers ("when a round is fully resolved, or at task close"); the Design explorations paragraph says only "commit-before-delete ... unless the exploration is worth keeping as a durable design record," with no timing. The keep-or-delete disposition is intentional (an exploration may be a durable design record; the plan even keeps `docs/plans/agent-scaffold.explorations/workflow-process-cluster.md`), so do not remove that flexibility. Only the delete-case timing is under-specified.

Recommended fix (optional, low value): name the delete trigger for the delete case, e.g. "at task close, unless worth keeping as a durable design record." Marginal; fine to skip.

---

## DISMISSED findings

### D1 - Lifecycle silent on nested sub-questions surfaced during an exploration (sonnet Finding 5)

DISMISSED. This is covered by composition: a subsidiary undecidable question is itself a new Open-Questions item and takes its own `exploring` status and file by the same rules; no special-case prose is needed. Spelling out nested-exploration handling is the kind of elaboration the step deliberately excluded ("match ceremony to stakes"; YAGNI). The claimed "improvise" gap does not exist because the general machinery recurses. Final severity: none. If the orchestrator wants a half-sentence noting that a subsidiary undecidable question becomes its own `exploring` item, that is optional polish, not a required fix.

### D2 - README diagram omits exploration (and Socratic) modes (sonnet Finding 6)

DISMISSED. The README diagram shows only the request flow; the Socratic mode is likewise absent by prior accepted design, so the README deliberately operates above entry-mode detail and this commit introduces no new inconsistency. The Success Criterion that names the README diagram requires the artifacts to "state the same rules the same way," and the README makes no entry-mode claim to contradict. The reviewer themselves scoped this as "not a defect ... a discoverability gap." Out of scope for this step. Final severity: none. An optional one-line "see AGENTS.md for additional entry modes" is a nice-to-have, not required, and arguably belongs to a later docs pass rather than this step.

---

## Summary for the orchestrator

- V1 (medium): reword the intake paragraph (and its template echo) so the orchestrator spawns the explorer(s) who write the artifact; orchestrator synthesises/presents/owns cleanup. Fold in the `exploring`->`open`-so-the-human-can-decide clarification. `pack/AGENTS.md` + `pack/plan-template.md`.
- V2 (medium): in the Design explorations paragraph, name explorers as writer agents under the file-safety/isolation rules and specify parallel-collision handling (distinct filenames + synthesis, or sequential). Prose only; no new role prompt. `pack/AGENTS.md`.
- V3 (medium): update the orchestrator prompt's question-handling clause to branch a not-yet-decidable/deliberation question into exploration mode rather than immediate Socratic answer. `pack/prompts/orchestrator.md`.
- V4 (low): add the orchestrator-initiated trigger to the template, or drop trigger enumeration and defer to AGENTS.md. `pack/plan-template.md`.
- V5 (low): include the exploration artifact in the template's pointer-target enumeration. Same edit as V4.
- V6 (nit): optionally name the explorations delete-case timing. Optional.
- D1, D2: dismissed (composition covers nesting; README deliberately above entry-mode detail).

After editing the `pack/` sources, regenerate the rendered copies with `scaffold-self` and re-run `just test` (the `plan_template_documents_every_accepted_status` drift guard and the rendered-copy sync test must stay green). None of V1-V6 touches `src/plan.rs`; the code change and its tests were confirmed correct by the opus reviewer and are not in question.
