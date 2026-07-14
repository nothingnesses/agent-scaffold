# Reviewer findings: `file-safety-rules` (Q-17)

Reviewer: independent (opus). Lens: completeness and correctness.
Diff reviewed: `git diff 742bcae d86ec0f`.

## Summary of verification (context, not a finding)

- All five rules from the step spec (docs/plans/agent-scaffold.md lines 350-354)
  are present in `pack/AGENTS.md` under the new "File safety and durability"
  block (lines 184-204): clean-tree-before-writer, commit-before-delete,
  format-only-your-own-files, validate-in-scratch, and the orchestrator recovery
  protocol. Each is stated as a named bullet.
- Role assignment matches the spec's fold-in instruction (line 356):
  `pack/prompts/implementer.md` (lines 13-18) carries format-only-your-own-files
  and validate-in-scratch; `pack/prompts/orchestrator.md` (lines 26-33) carries
  clean-tree-before-writer, commit-before-delete, and the recovery protocol. Each
  rule sits with the role that actually performs it.
- Generated mirrors are in sync. `diff` confirms root `AGENTS.md` ==
  `.agents/AGENTS.reference.md`, and both carry the identical file-safety block;
  `pack/prompts/implementer.md` == `.agents/prompts/implementer.md` and
  `pack/prompts/orchestrator.md` == `.agents/prompts/orchestrator.md`. The only
  delta between `pack/AGENTS.md` and root `AGENTS.md` is the expected
  `{{principles}}` expansion. No mirror mismatch.
- The added text is ASCII-clean (no non-ASCII bytes), no em/en-dashes, wording is
  consistent between `AGENTS.md` and the two role prompts (no contradictory
  restatement).

## R1: Incidental-reformatting duty is assigned to the orchestrator but the orchestrator prompt does not carry it

- Severity: low
- Location: `pack/AGENTS.md` line 196-199 (and mirror); `pack/prompts/implementer.md`
  lines 14-16; `pack/prompts/orchestrator.md` (whole file).
- Evidence/reasoning: The format-only rule tells the implementer to "leave
  incidental reformatting to the orchestrator" (AGENTS.md line 199,
  implementer.md line 16). This matches the step spec (plan line 352). But the
  orchestrator prompt never mentions formatting, and its opening line states "You
  do not plan, implement, review, or triage yourself; you spawn the roles that do"
  (orchestrator.md line 3). Reformatting files is a content change; a reader can
  reasonably conclude the orchestrator is barred from it too, so the duty the
  implementer defers has no role that is clearly permitted to perform it. This is
  a coherence gap against Project Principle 1 (internal coherence). Impact is low:
  reformatting is cosmetic, and the fix is a one-clause addition to the
  orchestrator prompt granting it the incidental-reformatting/tree-hygiene duty
  (which it already implicitly owns via its commit/cleanup responsibilities).
  Note the gap originates in the spec wording itself, so this is faithful to the
  step; flagged for completeness.
- Related, sub-low: the project's verification convention is repo-wide `nix fmt`
  (plan line 3), yet the rule forbids implementers from running repo-wide
  formatters and requires them to "format only the files it changed." The docs do
  not say how an implementer scopes `nix fmt` to its own files. This is a
  practical detail left to the agent, not a doc defect, but it compounds R1's
  ambiguity about who runs the repo-wide formatter.

## Severity tally

- critical: none.
- high: none.
- medium: none.
- low: 1 (R1).
