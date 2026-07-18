# Triage (round 1): waiver-model

Independent adjudication of reviewer-opus (O1-O5) and reviewer-sonnet (S1-S4). Every empirical claim below was reproduced by building the branch and running `validate --workflow` on constructed fixtures; the live repo still passes (`105 records, valid`, `50 steps`, `workflow invariants hold`, rc=0).

## Verdict table

| ID | Verdict | Reviewer sev | Final sev | Dedup group | Blocker? |
| -- | ------- | ------------ | --------- | ----------- | -------- |
| O1 | VALID | high | medium | A (W5 join integrity) | Yes |
| O3 | VALID | medium | medium | B (increment-waiver scoping) | Yes |
| S1 | VALID | high | medium | (migration honesty, standalone) | Yes |
| O2 | VALID (design) | low-med | low-med | B (shares W3 predicate with O3) | No (follow-up, contingent on O1) |
| O4 | VALID | low | low-med | A (same join as O1) | No (ship with O1 fix) |
| O5 | VALID (informational) | low/info | info | C (docs/tests) | No |
| S2 | VALID | medium | low | C (docs/tests) | No (cheap cleanup) |
| S3 | VALID | medium | low | A (test for O1 join) | No (ship with O1 fix) |
| S4 | VALID | low | low | C (docs/tests) | No |

Groups: A = the W5 record-backed evidence join (O1, O4, S3) — one fix. B = the W3 increment-exemption predicate `src/workflow.rs:306-309` (O3 bug, O2 design). C = docs/test cleanups (O5, S2, S4). S1 stands alone.

## Per-finding reasoning

**O1 (VALID, medium; reviewer said high).** Confirmed exit 0: a `complete` step whose only risky increment reached one clean round (peak 1, needs 2), exempted by a `record-backed` waiver citing `evidence:"TOTALLY-UNRELATED"` plus an escalation with `task:"TOTALLY-UNRELATED"` and `human_decision:"decision"`, produces `workflow invariants hold`, rc=0. The join at `src/workflow.rs:365-368` requires only that SOME escalation has a string-equal `task` and a `decision`; it never ties the escalation to the waived unit. This directly falsifies the documented anti-laundering guarantee (`EvidenceTier` doc `src/metrics.rs:180-182`, W5 doc `src/workflow.rs:334-335`): a self-declaration author can point at any decision escalation and earn the strong-tier stamp. I downgrade high->medium because O2 caps the blast radius: self-declaration already exempts the same increment, so O1 grants no EXTRA exemption power; the harm is that the "record-backed" strength signal is corrupted (an auditor trusts a stamp that means nothing), not that a new increment slips through. It remains a blocker: this is the pilot-1 defect class (the backstop certifies what it should reject) and it is exactly the "join satisfied by an unrelated escalation" path the brief flagged.

**O3 (VALID, medium).** Confirmed exit 0: two `complete` steps `alpha` (converged low_risk increment) and `beta` (short risky increment `beta-incB`), with an increment waiver `{"unit":"increment","step":"alpha","increment":"beta-incB",...}`, produces `workflow invariants hold` despite the waiver naming `alpha`. W3 (`src/workflow.rs:306-309`) keys the exemption on `increment` alone; W5 (`src/workflow.rs:353-359`) only checks `step` names some real slug, never that `leading_slug(increment) == step`. So `step` is decorative for increment waivers and W5's "names a real Roadmap step" check gives false assurance. Fix: add `&& waiver.step == step.slug` to the W3 increment predicate (tighter: a mis-scoped waiver then exempts nothing), or require `leading_slug(increment) == step` in W5.

**O2 (VALID as a design question, low-med).** Confirmed exit 0: a fresh risky increment at peak 1 exempted by a bare `self-declared` `review-skipped` increment waiver, no escalation, passes. W3 consults only unit+identity and W5 accepts `review-skipped`/`self-declared` as a valid pairing, so the entire `accepted-at-escalation`/`record-backed` machinery is optional for increments. This is not a bug on the model's own terms (see recommendation below); it is a doc-vs-behaviour mismatch: the `WaiverUnit` enum doc (`src/metrics.rs:155-156`) says an increment waiver is "for an increment accepted below its required streak at an escalation", which oversells the enforcement.

**O4 (VALID, low-med; reviewer said low).** `check_record` guards `step` and `increment` non-empty (`src/metrics.rs:485-487, 503-505`) but not `evidence` (`src/metrics.rs:518`), and `parse_waivers` filters `step`/`increment` non-empty but not `evidence` (`src/metrics.rs:793`). Opus called this "not a false-pass on its own." I reproduced a FULL false-pass: a `record-backed` waiver with `evidence:""` plus an escalation with `task:""` (neither field is guarded non-empty by `check_record` or `parse_escalations`) yields `3 records, valid` and `workflow invariants hold`, rc=0, exempting a short risky increment on empty evidence. So O4 is a real (contrived) exit-0 path, slightly worse than opus rated it. Fix belongs with O1: guard `evidence` non-empty in both `check_record` and `parse_waivers`, and guard escalation `task` non-empty.

**O5 (VALID, informational).** An inert (wrong-status step) or duplicate waiver grants no extra exemption, so neither is a correctness defect. Agree it is not a required change; a "waiver names a real but non-`complete` step" advisory is optional low-value polish.

**S1 (VALID, medium; reviewer said high).** Confirmed: `convergence-accounting`, `pack-rebuild-tracking`, `user-prompts-dir` each carry `reason:"predates-logging"` (lines 102-104) yet each has a round record (lines 3, 7, 16: `outcome:"new_valid"`, `consecutive_clean:0`, `low_risk`). `predates-logging` literally means "no records; done before logging existed" (b1), and these are the b2 "converged informally before per-round logging was disciplined" case, which the design keeps separate. W5 does not catch it (it only checks the reason<->tier pairing, satisfied either way). It is a factual mislabel in the flagship dogfooded migration whose whole claim is honest, visible exemptions, so it blocks a clean round even though it causes no false pass. I set medium not high: it is a data-label error, trivially fixable, no soundness impact.

**S2 (VALID, low; reviewer said medium).** The comments at `src/metrics.rs:455-459` and `:664` predict the waiver-model step would add a W3 cutoff to the `baseline` type. This implementation chose per-unit `type:"waiver"` records instead, so the comments now describe a not-taken path and point a reader to look for a cutoff that does not exist. Actively misleading but behaviourally inert; cheap cleanup, fold into the fix pass.

**S3 (VALID, low).** The join at `src/workflow.rs:365-368` correctly requires `HumanDecision::Decision`, but the only failing W5 test uses an empty escalation slice; no test exercises a matching-`task` escalation with `human_decision:"resume"`. Real gap, but it is a subset of the Group A join hardening; add the `resume` test when fixing O1.

**S4 (VALID, low).** The migration's three b2 waivers use `increment` equal to the bare step slug (no `-inc` suffix), matching a `task` with no suffix that `leading_slug` returns whole. This works (the live repo passes) but no test pins the bare-slug shape. Cheap regression guard; add one test.

## Recommendation on O2 (the design decision)

Recommend (a): accept self-declared increment waivers as intended, and fix the doc, NOT (b) restrict increment-unit waivers to `accepted-at-escalation`. This is a follow-up, NOT a convergence blocker, and it is contingent on O1 being fixed.

Reasoning. Before this change, `trivial` was a self-declared Roadmap STATUS that skipped review for any step; the model's stated intent is to make that exemption a visible, declared log line, not to remove the self-declaration capability. The two tiers exist to stop a weak self-declaration being laundered into looking like an independent escalation, not to forbid self-declaration. Option (b) as written forbids self-declared increment waivers outright, which would break the migration's own legitimate b2 case: the three b2 steps HAVE a round record (so they are increment-unit, not step-unit) and were reviewed informally, not escalated, so they genuinely need a self-declared increment waiver. (b) cannot express that. The code also cannot distinguish "historical informal convergence" from "fresh unconverged" from the record shape alone, so (b) would over-restrict. Under the model's philosophy the guardrails are visibility plus the pause.md catch plus the anti-laundering tiers; a self-declared risky-shortfall waiver is honest and auditable (a human sees a risky increment waived by bare author declaration and can challenge it). So (a) is the principled, minimal choice. The doc fix: reword `src/metrics.rs:155-156` and the schema narrative so an increment waiver is described as "a self-declared review-skipped/predates-logging exemption OR an accepted-at-escalation exemption", not implying increment shortfalls MUST be escalation-backed. Contingency: (a) only holds if the record-backed tier actually means something when used, which is O1. If O1 is not fixed, the softened doc would describe a strong/weak tier distinction the code does not honour. So fix O1 first, then land the O2 doc softening as a follow-up.

## Recommendation on S1 (migration honesty)

Change all three (lines 102-104: `convergence-accounting`, `pack-rebuild-tracking`, `user-prompts-dir`) from `reason:"predates-logging"` to `reason:"review-skipped"`. Both pair with `self-declared`, so W5 stays satisfied; the unit stays `increment` (each has a round record, so the step is not the no-records case). `review-skipped` is the correct label of the two self-declared reasons: each step ran ONE `new_valid` review round that found low findings (4, 4, 2) with `consecutive_clean:0`, then was accepted without running the convergence (clean-streak) rounds. The convergence review was skipped, even though the first round was not, and the label must not claim these steps have no records when they do. `predates-logging` must be reserved for the eleven true b1 zero-record step waivers (which S1 and O5 both confirm are accurate). If the author instead wants to argue these predate the convergence DISCIPLINE, that argument must be written down explicitly rather than left as a `predates-logging` label that contradicts the visible round record; `review-skipped` avoids that and is the honest default.

## Additional finding both reviewers under-stated (root cause of Group A + B)

For an increment-unit record-backed waiver, W5 today validates `step`, `increment`, and `evidence` in ISOLATION, none tied to the exempted increment: `step` need only be some real slug (O3), `evidence` need only match some decision escalation anywhere (O1), and `evidence`/escalation-`task` are not even required non-empty (O4). Composed, the worst case is a `record-backed` increment waiver that names the WRONG step and cites an UNRELATED (or empty) escalation, exempting an arbitrary short risky increment while displaying the strongest possible tier. Opus found the three pieces separately; the composition is the real defect and wants ONE coherent fix: for an increment-unit waiver require (1) `leading_slug(increment) == step` (O3), and for the record-backed tier (2) `escalation.task == increment` (O1) with (3) `evidence`/`task` non-empty (O4). That is exactly the migration's own convention on line 105 (`evidence == increment == escalation.task == optional-modules-inc2cii`), so tying the join to the unit codifies the pattern the honest record already follows. I found no further false-pass in the no-records step catch (increment waivers do not rescue it; confirmed) or the risk-class-inconsistency path (waivers never reach it; `continue` precedes the check).

## Must-fix before this round is clean

Blockers (round cannot be called clean until these land):

- O1: tie the W5 record-backed join to the waived unit (`escalation.task == waiver.increment` for increment waivers; `leading_slug(escalation.task) == waiver.step` for step waivers). Ship the O4 non-empty guards (`evidence`, escalation `task`) and the S3 `resume`-decision test in the same fix.
- O3: cross-check the increment waiver's `step` against the increment (`waiver.step == step.slug` in the W3 predicate, or `leading_slug(increment) == step` in W5).
- S1: relabel the three b2 waivers (lines 102-104) `predates-logging` -> `review-skipped`.

Follow-ups (do not block convergence; land as cleanup, several are same-file and cheap):

- O2: reword the increment-waiver docs (`src/metrics.rs:155-156` and schema narrative) so self-declaration is not implied to be forbidden; land AFTER O1 so the tier distinction it describes is real.
- S2: remove/rewrite the two stale `baseline` comments (`src/metrics.rs:455-459`, `:664`) that predict a W3 cutoff on `baseline` that this step did not add.
- S4: add a bare-slug increment-waiver test.
- O5: optional advisory for a waiver naming a non-`complete` step; low value, skip unless cheap.
