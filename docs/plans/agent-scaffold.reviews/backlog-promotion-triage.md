# Triage: backlog-promotion plan pass (branch impl/backlog-plan)

## B-1 (low): mis-attributed source of finding I2-2 in the sidecar step file

VERDICT: VALID
ACTIONABLE THIS ROUND? yes
SEVERITY: low (agree; cosmetic/traceability, not correctness or schema)

### Reasoning (with file:line)

The reviewer is correct: the attribution is wrong.

- `docs/plans/agent-scaffold.steps/sidecar-ref-empty-string.md:3` says the deferred
  cleanup is "issue I2-2, from the structured-skeleton implementation reviews".
- The ledger traces I2-2 to the `task-entry-regrounding` inc2 review, not to the
  structured-skeleton reviews:
  - `docs/plans/agent-scaffold.ledger.md:341` (anchor 2026-07-20h) records ROUND 1 of
    `task-entry-regrounding` inc2 (metrics record 141) raising "I2-2 (low,
    VALID-BUT-DEFER) `findings = [""]` validates clean via the inherited
    `is_safe_sidecar_ref("")` gap; belongs in the SHARED sidecar-ref rule ... -> BACKLOG".
  - `docs/plans/agent-scaffold.ledger.md:339` (20i) and `:337` (20j) carry the same
    I2-2 backlog item forward as the deferred `is_safe_sidecar_ref("")` empty-string
    acceptance from that inc2 review.
- The structured-skeleton reviews used a different id prefix and raised a DIFFERENT
  sidecar-ref concern: `docs/plans/agent-scaffold.ledger.md:381` (anchor 2026-07-19b,
  structured-skeleton Inc 3) tracks the LEXICAL `is_safe_sidecar_ref` symlink-escape
  residual as "L1", not I2-2, and never mentions the empty-string acceptance. So the
  empty-string gap (I2-2) did not originate there.

Technical content of the sidecar is otherwise correct (the `is_safe_sidecar_ref("")`
== true gap, the shared-rule scope covering both `[meta].sidecars` refs and
`[step.provenance]` findings refs). Only the source attribution is wrong. This is a
Principle 7 (cite real sources) miss, but purely a traceability nit, so low is right.

### Smallest correct fix

In `docs/plans/agent-scaffold.steps/sidecar-ref-empty-string.md:3`, replace the opening
clause:

  Deferred cleanup (issue I2-2, from the structured-skeleton implementation reviews).

with:

  Deferred cleanup (issue I2-2, from the `task-entry-regrounding` inc2 review; ledger anchors 2026-07-20h/i/j).

No other change. The rest of line 3 stays as-is.
