# Round-2 review: `file-safety-rules` (Q-17)

Reviewer: independent (round 2). Diff reviewed: `git diff 742bcae a3adc50`.
Sources read in full: `pack/AGENTS.md`, `pack/prompts/orchestrator.md`,
`pack/prompts/implementer.md`, the plan step detail (docs/plans/agent-scaffold.md
lines 346-356), the numbered Project Principles (lines 18-24), the round-1 triage
(`file-safety-rules-triage.md`), and the ledger record.

## Fixes verified

Generated mirrors are in sync (a precondition, not a fix): `pack/prompts/implementer.md`
== `.agents/prompts/implementer.md`, `pack/prompts/orchestrator.md` ==
`.agents/prompts/orchestrator.md`, root `AGENTS.md` == `.agents/AGENTS.reference.md`,
and the only `pack/AGENTS.md` vs root `AGENTS.md` delta is the expected `{{principles}}`
expansion. No mirror mismatch.

- Group A: CONFIRMED. `pack/AGENTS.md` lines 43-46 add the classifying sentence
  ("the planner and the implementer are writers ... the reviewers and the triager
  are read-only ... 'Writer agent' below means a spawned writer role.") and the
  file-safety lead-in now reads "The rules, each carried out by the role it names:"
  (line 193). This defines "writer agent" (S1) and removes the "The rules:" owner
  ambiguity (S2).
- Group B: CONFIRMED, as recommended (fix option 1). `pack/prompts/orchestrator.md`
  lines 34-36 grant the orchestrator tree-hygiene reformatting ("after a writer
  finishes you may run the repo-wide formatter to normalise drift it left, which is
  tree hygiene, not implementing a step."). Within orchestrator.md this is
  self-consistent: the same file's line 3 says the orchestrator does not implement,
  and the carve-out explicitly distinguishes reformatting from "implementing a step."
  Note (not a re-raise): the carve-out lives only in orchestrator.md; `AGENTS.md`
  read standalone still defers reformatting "to the orchestrator" (line 201) next to
  the Roles line saying it "does not ... implement" without carrying the distinction.
  This is the known, deliberate consequence of choosing the orchestrator-prompt fix
  that the triager endorsed, so it is not a new finding; the orchestrator reads its
  own prompt and gets the resolution.
- Group C: CONFIRMED. The "committing the deletion" clause is dropped from the ledger
  paragraph in both `pack/AGENTS.md` (now "delete it when the task closes", line ~184)
  and `pack/prompts/orchestrator.md` (line ~22). The canonical commit-before-delete
  rule still names the ledger case explicitly ("a findings file, the ledger at task
  close, any transient artifact ... before deleting it", pack/AGENTS.md lines 197-199),
  so the ledger's at-close deletion remains covered. No information lost, no gap.

## New findings

### T1: "read-only" mischaracterises the reviewers and the triager

- Severity: low
- Location: `pack/AGENTS.md` line 44 (mirrored in root `AGENTS.md` and
  `.agents/AGENTS.reference.md`). Text added by the Group A fix.
- Evidence: The new classifying sentence says "the reviewers and the triager are
  read-only." Taken literally this is false: reviewers write findings files and the
  triager writes triage-verdict files, and this very diff adds three such files
  (`file-safety-rules-reviewer-opus.md`, `-sonnet.md`, `-triage.md`) under
  `docs/plans/agent-scaffold.reviews/`. The file-safety section itself treats those
  files as tree writes: the commit-before-delete rule lists "a findings file" as a
  workflow-managed artifact that gets committed and deleted (pack/AGENTS.md line 197).
  The intended meaning is "read-only with respect to the plan and the code" (the
  writer parenthetical "(they change the plan or the code)" implies exactly that
  contrast), so a careful reader reaches the right classification, which is why this
  is low, not medium. But the bare word "read-only" overstates it. This matters
  slightly more than a wording nit because the same read-only classification is
  meant to be reused by the not-yet-implemented `agent-isolation` step to decide
  which agents are safe to run without isolation; if that step reads "read-only"
  literally, it could conclude reviewers make no tree writes at all, which is wrong.
  A tighter phrasing (e.g. "read-only with respect to the plan and code") would close
  the gap with no added machinery (Principle 2). Judged against Principle 1
  (internal coherence): the label contradicts the document's own treatment of
  findings files as committed workflow artifacts.

## Items considered and NOT raised

- Orchestrator excluded from the "writer agent" invariant while now holding
  repo-wide-formatter authority: the definition scopes "writer agent" to spawned
  writer roles (planner, implementer), so the umbrella "Every writer agent's damage
  must stay a ... recoverable diff" does not name the orchestrator, even though the
  orchestrator writes the ledger and may reformat (the exact operation from the
  motivating clobber incident). Judged coherent by design, not a defect: the
  orchestrator is the supervising committer/recovery actor, its writes are committed
  as part of the loop, and its reformatting is gated to "after a writer finishes."
  No new contradiction.
- Group B AGENTS.md-standalone residual (see the Group B note above): a known
  consequence of the endorsed fix, not new evidence the verdict was wrong; not
  re-raised.

## Severity tally (new findings)

- critical: none.
- high: none.
- medium: none.
- low: 1 (T1).
