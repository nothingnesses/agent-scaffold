# Triage: `user-prompts-dir` (Q-16)

Artifact: implementation commit, diff range `9f52ef0..9c1a88d`. Adjudicated against
the numbered Project Principles, the Documentation Protocol, and the orchestrator's
status-at-convergence convention (`pack/prompts/orchestrator.md:83-85`; precedent
`agent-isolation` R3 in the ledger, line 762). Reviews adjudicated: R1 (opus), S1 and
S2 (sonnet). I did not produce the artifact and am not the orchestrator.

## R1 (opus): `ownership = "reference"` justification

Verdict: INVALID as a finding (not a defect). Informational note only.

Severity: N/A.

Reasoning: The reviewer states plainly that `ownership = "reference"` "is the right
call, not a defect", and the note is an affirmation with grounding, not a defect
report. The reasoning is correct on the merits: the kickoff prompt is copied OUT and
filled in by the human, not edited in place, so `reference` (tool-owned, refreshed
every run) keeps it from drifting from `AGENTS.md` (Principle 1) and matches the
`.agents/prompts/` role assets; `working` would freeze a potentially stale trigger.
The residual (a human who edits the file in place has edits clobbered on rerun) is the
documented, by-design contract for every `reference` asset (README.md:50-52) and is not
specific to this step. Nothing to fix.

Recommended fix: none.

## S1 (sonnet): Roadmap not updated after implementation

I split this into the two parts the reviewer bundled.

### S1(a): step status still `not started`, not `complete`

Verdict: INVALID (expected).

Severity: N/A.

Reasoning: That the implementation diff does not flip `user-prompts-dir` to `complete`
is exactly the orchestrator's documented convention: status is flipped to `complete` at
CONVERGENCE, after the work review, not in the implementation commit
(`pack/prompts/orchestrator.md:83-85`). This is the same finding as `agent-isolation`
R3, which a prior separate triager ruled INVALID/expected (ledger line 762: "plan status
still `next` in the pre-convergence diff is expected; the orchestrator flips status at
convergence, not in the implementation commit"). No new evidence beats that verdict, so
it stands. The diff correctly leaves the plan untouched.

Recommended fix: none. The completion flip is owned by the pending convergence update.

### S1(b): Roadmap `next`/order does not reflect the decided reorder

Verdict: VALID, corrected to LOW (reviewer rated the bundled S1 medium).

Reasoning: This part is NOT covered by the precedent. In `agent-isolation`, the step
being implemented WAS the one marked `next`, so a stale-until-convergence status was
internally consistent. Here the step actually being implemented (`user-prompts-dir`) is
NOT the one the Roadmap marks `next`: the table still shows `human-onboarding` as `next`
and `user-prompts-dir` as `not started`, while the orchestrator has (on the human's
confirmed go-ahead; ledger 784-793) reordered the cluster to do `user-prompts-dir`
first and is proceeding on that basis. The Documentation Protocol calls the Roadmap "the
single source of truth for status and for implementation order" and says reordering is
done by editing the Roadmap rows. So the Roadmap currently misrepresents the decided
order and priority: this is a genuine coherence gap, distinct from the status-timing
convention.

Why LOW rather than medium: the impact is well mitigated for the durable resume path.
The plan Status line (line 3) was updated to point at "the sequencing note in
`user-prompts-dir`", and the ledger's RESUME STATE block, the designated "read this
first" resume anchor, explicitly says "recommend `user-prompts-dir` first" (ledger 806-807).
An agent following the documented resume procedure (Status line plus ledger RESUME STATE)
gets the correct order; only a reader of the bare Roadmap table is misdirected to
`human-onboarding`. The order desync is real but low-impact and short-lived.

Recommended minimal fix: at the pending convergence update, in addition to marking
`user-prompts-dir` `complete`, reorder the Roadmap rows to
`user-prompts-dir -> human-onboarding -> compaction-prep` and move the `next` marker so
it points at the actual current/next step rather than `human-onboarding`. This folds
entirely into the convergence/completion update the orchestrator already intends to make;
no separate change is required. (If convergence were not imminent, the order/`next`
reflection would be worth doing now, since order is not gated by convergence the way the
completion flip is; because the update is imminent, folding it in is sufficient.)

## S2 (sonnet): one validation criterion is unsatisfiable by this step alone

Verdict: VALID, LOW (confirms the reviewer's low).

Reasoning: The `user-prompts-dir` validate block (plan line 375) lists among its criteria
"the 'Getting started' section points to `.agents/user-prompts/` rather than embedding the
kickoff prompt". That section is created by `human-onboarding` (plan line 308), which
already carries the same validation criterion (plan line 310), and `human-onboarding` is
still pending. Under the reorder (`user-prompts-dir` first), the section does not exist
yet, so this criterion cannot be signed off as part of this step. The mis-assignment is a
real, if minor, plan-authoring coherence defect: the step's own acceptance criteria
over-scope into a sibling step's territory. The other three criteria (assets drop through
the loader, the asset-list test is updated and passes, the README names the new directory)
are all met by the diff, as both reviewers confirm, so this does not block the step's own
completion.

Why LOW: it is a documentation-precision issue with no functional impact; the criterion is
correctly owned and validated by `human-onboarding` (line 310), so nothing is lost, and the
reorder is "logically defensible" (create the directory before pointing at it), as the
reviewer notes.

Recommended minimal fix: drop the "Getting started section points to `.agents/user-prompts/`"
criterion from `user-prompts-dir`'s validate block (line 375), since `human-onboarding` owns
and already validates it (line 310); or annotate it there as deferred to `human-onboarding`.
Either is a one-line plan edit that folds into the same pending convergence plan update as
S1(b); no separate change is required.

## Summary

- R1: INVALID (informational, not a defect). No fix.
- S1(a): INVALID/expected (status-at-convergence convention; `agent-isolation` R3 precedent). No fix.
- S1(b): VALID, LOW. Roadmap order/`next` does not reflect the decided reorder; mitigated by the Status line and RESUME STATE note. Fix folds into the pending convergence update (reorder rows, move `next`).
- S2: VALID, LOW. A validation criterion owned by `human-onboarding` is listed under `user-prompts-dir` and is untickable there under the reorder. Fix folds into the same convergence plan update (drop or defer the criterion).

No high or critical findings, so no second-triager backstop is required. Both valid
findings are LOW and both fold into the orchestrator's pending convergence/completion
update rather than requiring separate changes.
