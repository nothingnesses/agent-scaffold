# Review round 2: compaction-prep (Q-15) and resume prompt (Q-19)

Reviewer: independent reviewer (round 2), verification lens. Did not produce this change.
Diff reviewed: `git diff 082280e 16b1260` (cumulative step diff, post round-1 fixes).

Fixes verified; no new findings.

## Verification of the three settled findings

Group 1 (prompts restate the procedure they claim not to): FIXED. Both prompt
bodies are now thin triggers that name the target section without enumerating its
steps.

- `pack/user-prompts/compaction-prep.md` body: "run the pre-compaction checkpoint
  from the 'Checkpoint and resuming after context loss' section of `AGENTS.md` ...
  Tell me when the tree is clean and it is safe to compact." No flush / verify /
  commit enumeration remains. The header claim "it does not restate that
  procedure" (line 5) is now TRUE.
- `pack/user-prompts/resume.md` body: "Reconstruct your state per the 'Checkpoint
  and resuming after context loss' section of `AGENTS.md`. Act as the orchestrator
  and continue from where the plan and the ledger say the work left off; do not
  start over." The AGENTS.md -> plan -> ledger read-order enumeration is gone; the
  header claim "it does not restate it" (line 5) is now TRUE.
- Both retain only prompt-specific framing (completion signal; orchestrator /
  continuation), matching the thin `kickoff.md` pattern which points at where the
  workflow lives without restating steps. The `.agents/user-prompts/` mirrors are
  byte-identical to the `pack/` sources (confirmed by `diff`).

R1 (clean-tree-before-writer over-claim): FIXED. The pre-compaction bullet in
`pack/AGENTS.md` now reads "(the same commit-before-risk durability discipline as
before a writer, applied to a distinct trigger)". It no longer claims the
writer-scoped rule "covers" the compaction trigger; it names the shared mechanic
and states the trigger is distinct. No over-claim remains.

S3 (durable-notes referent): FIXED. The section's closing note now reads "This
names the plan, the ledger, and the plan's Open Questions queue, not any specific
harness memory feature", matching the three artifacts named in the body. The
dangling "durable notes" phrase is gone.

## Overall checks

- Prompts still usable as triggers: yes. Each names the exact target section by
  title and gives the agent its role / completion signal; neither is trimmed below
  the point of being actionable.
- Section still complete, correct, and harness-agnostic: yes. It names the plan,
  the ledger, and the Open Questions queue rather than any harness memory feature.
- Mirrors in sync / idempotent: `just scaffold-self` leaves a clean tree
  (`git status` empty); `pack/AGENTS.md`, `AGENTS.md`, and
  `.agents/AGENTS.reference.md` carry the identical section text (Principle 4).
- Asset-list test: `just test` -> 46 passed, 0 failed.
- ASCII-clean: no non-ASCII bytes in any changed file (`grep -nP '[^\x00-\x7F]'`
  over all seven changed source/mirror files plus README.md returned nothing).
- README layout entries for `compaction-prep.md` and `resume.md` remain accurate.

No new defect was introduced by the round-1 edits: no broken section reference
(both prompts and the AGENTS.md cross-links name the exact "Checkpoint and
resuming after context loss" section, which exists), and neither prompt is too
thin to use. Judged against the numbered Project Principles in
`docs/plans/agent-scaffold.md`, the change now satisfies Principle 1 (coherence,
one source of truth), Principle 2 (minimal), and Principle 4 (idempotent, no
drift).

## Severity counts

critical: 0; high: 0; medium: 0; low: 0.
