# Round-3 review: `file-safety-rules` (Q-17)

Reviewer: independent (round 3). Diff reviewed: `git diff 742bcae aeea55b` (the
full step). Sources read in full: `pack/AGENTS.md` (Roles classifying sentence and
the "File safety and durability" section), `pack/prompts/orchestrator.md`,
`pack/prompts/implementer.md`, the plan step detail and numbered Project Principles
(`docs/plans/agent-scaffold.md`), and the round-1 and round-2 triage records.
Final step commit is `aeea55b` and it is at `HEAD`.

## Round-2 fix verified

- Text landed exactly as required. `pack/AGENTS.md` now reads: "the planner and the
  implementer are writers (they change the plan or the code) and the reviewers and
  the triager are read-only with respect to the plan and code (they write only their
  own findings files); ... 'Writer agent' below means a spawned writer role." This
  is the T1 fix (round-2 triage: "read-only" made precise), and it is more thorough
  than the minimal parenthetical the triager suggested: the added "(they write only
  their own findings files)" makes the tree-writes-are-findings-files fact explicit
  rather than merely narrowing the scope of "read-only".
- Coherent with commit-before-delete. The commit-before-delete rule names "a
  findings file" as a workflow-managed artifact committed before deletion. The new
  parenthetical "(they write only their own findings files)" acknowledges exactly
  those writes, so the classifying sentence no longer contradicts the document's own
  treatment of findings files (the Principle 1 gap T1 raised is closed).
- Coherent with the "writer agent" definition. Writers are defined as the planner
  and implementer, the orchestrator is the loop driver/ledger keeper that spawns
  them, and the final sentence pins "'Writer agent' below means a spawned writer
  role." The umbrella invariant ("Every writer agent's damage must stay a visible,
  committed-or-recoverable diff") and the clean-tree bullet ("before spawning a
  writer agent") both resolve against this definition.

## Generated mirrors in sync

At commit `aeea55b`:

- root `AGENTS.md` == `.agents/AGENTS.reference.md` (identical).
- `pack/prompts/implementer.md` == `.agents/prompts/implementer.md` (identical).
- `pack/prompts/orchestrator.md` == `.agents/prompts/orchestrator.md` (identical).
- The only `pack/AGENTS.md` vs root `AGENTS.md` delta is the expected `{{principles}}`
  template expansion. No mirror mismatch.

## Final coherence pass (five rules, ownership)

Checked all five rules across `pack/AGENTS.md`, `pack/prompts/orchestrator.md`, and
`pack/prompts/implementer.md`. Each maps to a defined owner and is carried in that
owner's prompt, matching the plan's fold-in instruction:

1. Clean tree before a writer -> orchestrator (the spawner, per the classifying
   sentence "spawning the writers"). Carried in orchestrator.md.
2. Commit before delete -> orchestrator (the committer/ledger keeper). Carried in
   orchestrator.md. The pre-existing ledger paragraph's redundant "committing the
   deletion" clause was removed (round-1 Group C), so the ledger case is now owned
   once by the canonical rule.
3. Format only your own files -> implementer; incidental reformatting deferred to
   the orchestrator, whose prompt now carries the tree-hygiene carve-out (round-1
   Group B). Carried in implementer.md and orchestrator.md.
4. Validate in scratch -> implementer (a writer). Carried in implementer.md.
5. Recover on interrupt -> orchestrator (explicitly named). Carried in
   orchestrator.md.

No rule is unowned, no gating term ("writer agent") is undefined, and I found no
contradiction between the umbrella invariant, the five bullets, and the two role
prompts. The orchestrator's exclusion from the "writer agent" invariant while
holding the recovery and reformatting duties is coherent by design (it is the
supervising committer/recovery actor), as round 2 already recorded; no new evidence
contradicts that.

## Settled findings not re-raised

- Round-1 Groups A/B/C (fixed) and round-2 T1 (fixed) are confirmed landed above.
  No new evidence that any verdict was wrong.
- The lead-in "The rules, each carried out by the role it names" while three of the
  five bullets do not name a role in their own text: this is the endorsed round-1
  Group A resolution (classifying sentence plus lead-in reword). Ownership is
  derivable from the classifying sentence and the role prompts, so it is coherent;
  not a new defect and not re-raised.

## Verdict

Fixes verified; no new findings.

## Severity tally (new findings)

- critical: none.
- high: none.
- medium: none.
- low: none.
