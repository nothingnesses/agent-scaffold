# round-log-core increment A: triage

Artifact: increment A of the `round-log-core` migration. Branch `impl/round-log-core`, HEAD `1ef49c7`, diff `f3855a3..HEAD`. Scope: `src/metrics.rs` makes `risk_class` REQUIRED on `type:"round"` records; `docs/metrics/workflow.jsonl` backfills it into the 46 historical records.

Reviewers adjudicated:
- opus (correctness): `round-log-core-A-reviewer-opus.md`, 1 medium (F1).
- sonnet (data/classification): `round-log-core-A-reviewer-sonnet.md`, 1 high (H-1), 1 medium (M-1), 2 low (L-1, L-2).

Verdicts follow. Line length is never a finding and none was raised. No high/critical finding is dismissed, so no backstop re-check is triggered.

---

## V1 (VALID, medium): `pack/instrument.md` still documents `risk_class` as optional (dedup of opus-F1 + sonnet-H-1)

Owner: isolated implementer (worktree fix, applied in this increment).
Final severity: medium.

opus-F1 and sonnet-H-1 are the SAME finding, deduplicated here. Both report that `pack/instrument.md:5` still reads "Two optional calibration fields should be included when known: `risk_class` ... and `reviewers` ... A record written without these optional fields still validates," while the increment-A code change now unconditionally calls `require_enum(obj, "risk_class", ...)`, so a `round` record without `risk_class` is rejected with `missing field \`risk_class\``. Confirmed against the worktree: `pack/instrument.md:5` is unchanged and still calls `risk_class` optional; sonnet additionally confirmed the stale prose renders verbatim into `AGENTS.md:129` and `.agents/AGENTS.reference.md:129`, which is what orchestrators actually read.

This is a genuine documentation-vs-code contradiction and violates Principle 16 (one source of truth): `pack/instrument.md` is the human-readable half of the single-source metrics schema, and it now disagrees with the validator. An orchestrator following the shipped guidance is told it may omit `risk_class` and the record still validates; it will instead hit a validation failure.

Severity adjudication (opus said medium, sonnet said high; I set medium). The failure mode is a loud, self-describing validation error (`missing field \`risk_class\``), caught deterministically by `validate` (Principle 12, fail fast and loudly), not silent data corruption or a security/data-loss issue. That bounds the impact to "misleading doc that produces a recoverable, self-explanatory error," which is medium, not high, even accounting for the wider blast radius of the generated `AGENTS.md`/`.agents/AGENTS.reference.md` copies. The wider render surface is why it is not low, but a loud validator backstop keeps it below high.

Scope: this is increment A's OWN correctness bug, not increment B's. Increment A is the change that made `risk_class` required, so the required/optional correction for `risk_class` is a direct consequence of this increment and belongs here. Increment B's prose work (`pack/LEDGER.template.md`, convergence accounting in `pack/AGENTS.md` + `pack/prompts/orchestrator.md`) is a separate reshaping and does not cover this line; leaving it to B would ship increment A with a self-contradicting schema doc.

Recommended action (implementer, in the worktree):
1. Edit `pack/instrument.md` so `risk_class` is documented as REQUIRED on `round` records: move it out of the "Two optional calibration fields" description and into the required-field list at the start of the `type: "round"` bullet, keeping its `low_risk`/`risky` semantics text.
2. Make `reviewers` the sole optional calibration field: reword the "Two optional calibration fields ... risk_class ... and reviewers" sentence to describe only `reviewers`, and change the closing "A record written without these optional fields still validates" to refer only to `reviewers` (a record written without `reviewers` still validates).
3. Regenerate the self-scaffold (`just scaffold-self`) so `AGENTS.md` and `.agents/AGENTS.reference.md` pick up the corrected prose.
4. Re-run `just test` (including the `instrument_prose_documents_every_accepted_schema_value` drift guard) and `cargo run -q -- validate --metrics docs/metrics/workflow.jsonl` to confirm green.

---

## M-1 (DEFERRED-note, not an increment-A fix): six `low_risk` tasks have no terminal clean round

Owner: orchestrator (plan-step-detail note on `workflow-invariants`). No code/data change in increment A.
Final severity: medium as a forward-note; not a defect in this artifact.

sonnet reports six task groups (`workflow.jsonl` lines 1-4, 7-8, 16: `workflow-hardening`, `convergence-accounting`, `plan-maintenance`, `pack-rebuild-tracking`, `consolidate-plan`, `user-prompts-dir`) whose last logged `round` record is `outcome:"new_valid", consecutive_clean:0`, all labelled `risk_class:"low_risk"`, so none ever reaches the `consecutive_clean:1` that W3 (`workflow-invariants`, plan line 607) will require for a `low_risk` step.

The backfill is ACCURATE (sonnet confirmed, and confirmed NO misclassification): the ledger shows these tasks converged informally (human acceptance, grep-verified fixes, orchestrator validation), and none of those paths produced a logged clean round. The data correctly records what happened, so there is nothing to fix in increment A. Fabricating a clean round or relabelling would corrupt the log and violate Principle 3 (ground in evidence).

This is therefore disposition (b) from the triage brief: a valid FORWARD-note the orchestrator should ensure is captured in the `workflow-invariants` step detail, not an increment-A fix and not a dismissal. The plan's `workflow-invariants` sub-decision (b) already flags a grandfather boundary, but it currently frames it only as "steps with NO round records" ("the earliest steps have no round records ... the check likely applies only to steps whose task appears in the log or is declared trivial"). These six tasks are a DIFFERENT category: they DO appear in the log with round records, yet never reach the required streak. Under the current sub-decision (b) framing ("applies only to steps whose task appears in the log"), W3 would FLAG all six as violations. So the forward-note is a concrete refinement of an existing plan item, not a new decision.

Recommended action (orchestrator): amend the `workflow-invariants` step detail so grandfather sub-decision (b) explicitly covers the "appears-in-log-but-streak-never-reached" category (informal pre-classification-system convergence), distinct from "no round records at all." W3's grandfather boundary must not fail these six on the streak check; the leading options already named (a `trivial` completion status, or a grandfather cutoff keyed on when the classification/streak discipline began) both need to handle records that exist but predate the streak convention. No change to increment A.

---

## L-1 (DISMISSED as non-finding): risky artifact identification is complete

Owner: none. Final severity: not a defect.

L-1 is a positive confirmation, not a finding: sonnet verified that `deliberation-mode` and `no-wrap-convention` are correctly the only `risky` records and that no risky artifact was missed, checking every ledger-classified task. This is corroborating evidence that the backfill is correct; there is nothing to fix or defer. Recorded as confirmed-correct.

---

## L-2 (DISMISSED as non-finding): grandfathered `low_risk` defaults are defensible

Owner: none. Final severity: not a defect.

L-2 is likewise a confirmation, not a finding: the six pre-classification-system tasks with no ledger risk statement were defaulted to `low_risk`, which sonnet judges defensible under Principle 3 (no `risky` evidence exists; all were doc-only/small-scope; inventing `risky` would be worse). Nothing to fix. Note the substantive tension this default creates for W3 is the same structural issue tracked under M-1's forward-note; it is captured there, not separately.

---

## Extra: drift-guard gap (raised in both reviews) - out of scope for increment A, low forward-note

Owner: orchestrator (decide whether to fold into a later increment). Final severity: low.

Both reviewers observe that the drift guard (`instrument_prose_documents_every_accepted_schema_value`) only asserts that each schema field/enum NAME appears in the prose, not its required/optional status, which is why V1 slipped through green. This is a real limitation and is the mechanism that should have caught V1 (Principle 16). I am NOT raising it as an increment-A blocking fix: increment A's scope was making `risk_class` required, backfilling, and updating fixtures and the drift guard for the new field NAME, which was done; strengthening the guard to mechanically assert required-vs-optional against free-form prose is a broader, separate enhancement (prose is not a machine-parsed schema, so this is non-trivial design, not a one-line fix) and folding it in here would be silent scope expansion (Principle 8). Recommended disposition: record as a low forward-note for the orchestrator to consider folding into `workflow-invariants` or a later drift-guard hardening step, where the required/optional distinction is already load-bearing. Fixing V1's prose (V1 above) resolves the immediate contradiction regardless.

---

## Summary for the orchestrator

- V1 (VALID, medium; dedup of opus-F1 + sonnet-H-1): fix in this increment. Implementer edits `pack/instrument.md` to document `risk_class` as required and `reviewers` as the sole optional field, then `just scaffold-self`, then re-run tests + `validate --metrics`. Severity corrected to medium (sonnet's high downgraded: loud validator error, not silent corruption).
- M-1 (DEFERRED-note, medium as forward-note): data is accurate, no increment-A change. Orchestrator amends `workflow-invariants` sub-decision (b) so the grandfather boundary explicitly covers the six "appears-in-log-but-streak-never-reached" tasks, not just "no round records."
- L-1, L-2 (DISMISSED as non-findings): both are confirmations that the backfill is correct; nothing to fix. L-2's structural tension is already covered by M-1's forward-note.
- Drift-guard gap (out of scope, low forward-note): guard checks name-presence not required/optional status; record for a later hardening step, do not expand increment A.

No high/critical dismissal, so no backstop re-check needed. The one artifact change this round is V1; M-1 and the drift-guard note are orchestrator-owned plan notes, not blockers on increment A's convergence once V1 lands.
