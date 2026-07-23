# Round-2 review: triager-on-findings (fix verification and final sweep)

Branch `plan/triager-on-findings`, tip `e34cc56`, base main `0ea33f6`. Round-2 re-review of the F1 fix (commit `e34cc56`) plus a final always-run sweep. Read-only on the product; only this file is committed.

## Verdict

No findings. Zero findings at any severity.

F1 is fixed, the new wording is correct and reads cleanly in context, and the sweep found no remaining always-run implication and no new contradiction introduced by the fix.

## F1 verification

The "Review reports" paragraph (`pack/AGENTS.md` line 69, mirrored identically in `AGENTS.md` line 69 and `.agents/AGENTS.reference.md` line 69) now reads:

"...which the orchestrator synthesises from the reviewers' findings files and, when a triager ran, its findings file:"

The triager findings file is now a conditional input ("when a triager ran, its findings file"), no longer an unconditional "reviewers' and the triager's findings files". This is consistent with:
- The review-entry-mode paragraph (line 47): "a separate triager when the reviewers raise one or more findings; a zero-findings review produces a clean report with no triage step".
- The Findings-files naming rule (line 67): the triager's file is `<step>-triage.md`, still defined but not implied to always exist.

The rest of the paragraph does not imply a triager always ran: the remaining clauses describe the report's structure (grouped severities, dismissed findings, kickoff handoff), its durability, its `file:line` evidence, and its single-pass ledger record, none of which assert a triager. The fix is a single clause swap; the sentence grammar is intact ("synthesises from the reviewers' findings files and, when a triager ran, its findings file: the target and the criteria..."), and no new inconsistency is introduced. The fix is identical across all three file copies.

## Final sweep (always-run implication and new contradiction)

Ran `grep -n "triager|triage|reviewers-then"` over `pack/AGENTS.md`, `pack/prompts/orchestrator.md`, and `pack/prompts/triager.md` on `e34cc56`, and examined every hit. Locations swept and cleared:

- Workflow intro (`pack/AGENTS.md` line 15): triager exception now conditional, "on any review round that has one or more findings it is never merged... ; a round where every reviewer reports zero findings is already clean and has no triage step". Conditional; no always-run.
- Triager role (line 22): "The triager runs on any review round in which the reviewers raised one or more findings; a round where every reviewer reports zero findings is already clean and needs no triage". "Zero findings" read objectively from committed reviewer files. "When the triager does run it is always a separate agent (or a human)... never collapsed into another role." Conditional-run plus never-collapse both correct.
- Phases 3, 4, 5 (lines 31-33): each spawns the triager only "if the reviewers raised one or more findings"; each states the zero-findings round skips triage. Conditional.
- Review entry mode (line 47): conditional triager, zero-findings clean report. Conditional.
- Convergence decision input (line 53): "the triager's verdicts when the round had findings, or the reviewers' zero findings when it was clean". Conditional.
- Backstop (line 59): intact and not weakened; still requires the second independent triager (or human) before a high/critical dismissal counts or settles a single reviewers-then-triager pass.
- Ledger (line 63): "on a round with findings, the separate triager that ran". Conditional; never-collapse ("separate") preserved.
- Findings-files naming rule (line 67): intact; triager file `<step>-triage.md` and re-check `<step>-triage-recheck.md` still defined.
- Review reports (line 69): the F1 fix, verified above.
- Worktree conflict-resolution (line 95): "if that review raises one or more findings, a separate triager that is never collapsed into the producer or the orchestrator". Conditional plus never-collapse.
- Preflight (line 104): the "separate-triager rule" reference is intact and not weakened.
- Orchestrator prompt (`pack/prompts/orchestrator.md` lines 5, 20, 25): triager exception conditional ("on any review round that raised one or more findings it is always a separate agent... never played by you"); zero-findings round skips triage; acceptance triager runs "only if the acceptance reviewers raise one or more shortfalls". Conditional plus never-collapse.
- Triager prompt (`pack/prompts/triager.md`): no always-run assertion.

The `reviewers-then-triager` hits (phase 5, review entry mode, backstop, review user-prompt) all name the single acceptance/review pass by its label; each is governed by conditional triager-run wording in the same or adjacent sentence, so none asserts the triager runs on every round.

Dropped rule name: `git grep "always-separate-triager"` over `pack .agents AGENTS.md` on `e34cc56` returns no hits (exit 1). No dangling reference to the renamed rule.

Never-collapse property: preserved everywhere the triager can run (Triager role line 22, conflict-resolution line 95, orchestrator prompt line 5), each stating that when the triager runs it is a separate agent or human, never merged into the producer or the orchestrator.

## Validation

Restored the product to `e34cc56` (`git checkout e34cc56 -- pack .agents AGENTS.md`), ran under the project toolchain, then restored to `HEAD`:

- `cargo test`: all tests passed (0 failed).
- `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow`: 173 metric records valid; 77 steps, 62 questions valid; workflow invariants hold.
- `cargo run -- render --check docs/plans/agent-scaffold.plan.toml`: up to date.

Worktree restored clean (`git status --short` empty) before writing this file.

## Prose

The fix text is ASCII-only (non-ASCII scan of line 69 returned no matches), no em-dashes, no unicode, no emoji, not hard-wrapped, and free of AI filler.
