# Round-2 triage: `file-safety-rules` (Q-17)

Triager: independent (round 2). Finding reviewed: T1 from
`file-safety-rules-round2-reviewer.md`. Sources read: `pack/AGENTS.md` (the
classifying sentence at lines 43-46 and the commit-before-delete rule at lines
197-199), `docs/plans/agent-scaffold.md` (Project Principles 1 and 2; the
`agent-isolation` step detail at lines 358-365; the `findings-files` step detail
at lines 326-336).

## T1: "read-only" mischaracterises the reviewers and the triager

Verdict: VALID. Severity: low (reviewer's rating confirmed).

The classifying sentence says "the reviewers and the triager are read-only."
That label is genuinely imprecise. The same document, in its commit-before-delete
rule, names "a findings file" as a committed workflow artifact - written by a
reviewer or triager and committed to the tree before deletion. This is a real tree
write, not an editorial quibble. The literal label therefore contradicts the
document's own description of reviewer and triager output, which is a Principle 1
(internal coherence) failure.

The intended meaning is recoverable. The writer-side parenthetical "(they change
the plan or the code)" implies by contrast that "read-only" means "read-only with
respect to the plan and code", and a careful reader reaches the right
classification. That is why this is low, not medium: no current behavior is broken
and no agent is currently being misdirected.

The finding is low rather than higher for a second reason: the `agent-isolation`
downstream risk is real but bounded. The `agent-isolation` step uses the same
"read-only" classification to decide which agents need no isolation ("Read-only
agents (reviewers reading, the triager) need no isolation"). The isolation
rationale is sound in principle - reviewers do not touch code or plan files, the
artifacts isolation is meant to protect - so an `agent-isolation` implementer
following the design intent would reach the right rule even with the imprecise
label. However, a literal reading of "read-only" could also cause the implementer
to skip file-safety machinery for reviewer writes (the findings files themselves),
which would be wrong. Closing this gap before `agent-isolation` is implemented
costs almost nothing.

The fix is a one-clause parenthetical: change "the reviewers and the triager are
read-only" to "the reviewers and the triager are read-only (with respect to the
plan and code)". This aligns the label with the document's own commit-before-delete
treatment of findings files, matches the implied contrast of the writer-side
parenthetical, and satisfies Principle 2 (no added machinery, one short phrase).
It also tightens the isolation carve-out's scope before `agent-isolation` is
implemented.

Recommended action: fix now. The clarification is minimal (Principle 2), closes a
documented internal inconsistency (Principle 1), and pre-empts a concrete
misreading risk in the next Roadmap step (`agent-isolation`). Dismissing it would
leave a label that contradicts the same document's own evidence about what
reviewers and triagers write.
