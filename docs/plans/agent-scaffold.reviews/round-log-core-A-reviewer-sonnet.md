# Review: round-log-core increment A (data backfill + risk_class required)

Reviewer: sonnet (classification correctness + design fit lens)
Branch: impl/round-log-core, HEAD 1ef49c7
Diff range: f3855a3..HEAD
Files changed: docs/metrics/workflow.jsonl, src/metrics.rs

---

## H-1 (high): `pack/instrument.md` still describes `risk_class` as optional after the code makes it required

**Location:** `pack/instrument.md` (entire file); rendered into `AGENTS.md` line 129 and `.agents/AGENTS.reference.md` line 129.

The validator in `src/metrics.rs` now unconditionally calls `require_enum(obj, "risk_class", ...)` for every `round` record, removing the previous `if obj.contains_key("risk_class")` guard. This makes `risk_class` required: any round record without it is rejected with "missing field `risk_class`". The new test (`a_round_missing_risk_class_is_reported`) confirms this.

However, `pack/instrument.md` still reads: "Two optional calibration fields should be included when known: `risk_class` ... A record written without these optional fields still validates."

The generated `AGENTS.md` and `.agents/AGENTS.reference.md` inherit this stale description verbatim. The documentation directly contradicts the code: a record WITHOUT `risk_class` does NOT validate after this change, but the AGENTS.md (what orchestrators read) says it does. An orchestrator following the current AGENTS.md documentation would omit `risk_class`, then get a "missing field `risk_class`" validation error it did not expect.

The plan for increment A explicitly states making `risk_class` required; the code change is correct and in scope. The documentation update was not done. The drift guard test only confirms that documented field/enum names appear in the prose; it does not check whether the required/optional status is correct, so it did not catch this omission.

**Fix:** Update `pack/instrument.md` to reflect that `risk_class` is now required, not optional (move it out of the "optional calibration fields" group, or restate the sentence). Regenerate the self-scaffold so `AGENTS.md` and `.agents/AGENTS.reference.md` pick up the change.

---

## M-1 (medium): Six `low_risk` tasks have no terminal clean round - inconsistency with the W3 invariant

**Location:** `docs/metrics/workflow.jsonl` lines 1-4, 7-8, 16.

These six task groups end with `outcome: "new_valid", consecutive_clean: 0` as their last logged record (no subsequent clean round):

- Lines 1-2: `workflow-hardening` (2 rounds, both new_valid, consecutive_clean=0)
- Line 3: `convergence-accounting` (1 round, new_valid, consecutive_clean=0)
- Line 4: `plan-maintenance` (1 round, new_valid, consecutive_clean=0)
- Line 7: `pack-rebuild-tracking` (1 round, new_valid, consecutive_clean=0)
- Line 8: `consolidate-plan` (1 round, new_valid, consecutive_clean=0)
- Line 16: `user-prompts-dir` (1 round, new_valid, consecutive_clean=0)

All carry `risk_class: low_risk`. The `workflow-invariants` step (plan line 607) states its key check W3: for every complete step, "the consecutive-clean streak (per artifact) must reach the required count for that class (low_risk 1, risky 2)." None of these tasks' records ever reach consecutive_clean=1.

This is historically accurate. The ledger confirms these tasks were converged informally: `workflow-hardening` and `convergence-accounting` by human acceptance ("match ceremony to stakes"), `plan-maintenance` by grep-verified fix, `pack-rebuild-tracking` by orchestrator validation, `consolidate-plan` by grep-verified clean, and `user-prompts-dir` by "verified mechanical" per the triager. None of those convergence paths produced a logged clean round. The backfill correctly records what happened.

The inconsistency is between the `risk_class: low_risk` label (which implies the formal 1-clean-round criterion applies) and the logged pattern (which shows convergence without one). The `workflow-invariants` step acknowledges the grandfather boundary issue and lists it as a sub-decision, but does not resolve it yet. When W3 is implemented, these six groups will require special handling (either a grandfather exception boundary or a `trivial` completion status). Surfacing this now so the W3 design accounts for them explicitly rather than discovering them at implementation time.

This is NOT a misclassification: `low_risk` is correct per the ledger (it is silent on all six, or explicitly says low_risk for `user-prompts-dir`). The tension is structural.

---

## L-1 (low): Risky artifact identification is complete - no omissions

The backfill correctly identifies `deliberation-mode` and `no-wrap-convention` as the only `risky` artifacts. The ledger explicitly classifies both:
- `deliberation-mode` (ledger line 273): "Classified RISKY / high-blast-radius (the core cross-cutting contract, widely depended on) -> TWO clean rounds required." Records 23-25 carry `risk_class: risky`. Round pattern: new_valid(0), clean(1), clean(2). Matches risky convergence.
- `no-wrap-convention` (ledger line 291): "Classified RISKY / high-blast-radius (repo-wide reflow of shipped pack prompts + a change to the formatter contract and the dogfooding recipe) -> TWO clean rounds required." Records 28-30 carry `risk_class: risky`. Round pattern: new_valid(0), clean(1), clean(2). Matches risky convergence.

All other tasks where the ledger states a classification are explicitly "LOW-risk": `triager-independence` (line 223), `file-safety-rules` (line 229), `agent-isolation` (line 237), `user-prompts-dir` (line 247), `human-onboarding` (line 251), `gate-prompt-clarity` (line 259), `compaction-prep` (line 265), `human-review-queue` (line 285), `findings-files` (line 299), `ledger-template` (line 305), `instrument-flag` (line 313), `state-schema-inc1` (line 319), `state-schema-inc2` (line 325), `state-schema-inc3` (line 335), `optional-modules-inc1` (line 351). All correctly backfilled as `low_risk`. No omissions.

`human-review-queue` was specifically checked: the ledger says "Classified LOW-risk" explicitly (line 285). The `state-schema` increments were also checked: all explicitly LOW-risk. No missed risky artifacts.

---

## L-2 (low): Grandfathered defaults are defensible

The six pre-classification-system tasks (workflow-hardening, convergence-accounting, plan-maintenance, workflow-doc-fixes, pack-rebuild-tracking, consolidate-plan) have no ledger statement of risk class. The implementer defaulted them to `low_risk`. This is grounded in Principle 3 (evidence-based): none carry a "risky" label in the ledger, all were doc-only or small-scope changes, and inventing a `risky` classification where no evidence exists would be worse than the conservative default. The `workflow-doc-fixes` task has a properly logged clean round (line 6, consecutive_clean=1), making it the one grandfathered task that fits the `low_risk` pattern cleanly.

---

## Spot-check accuracy

Checked every explicitly-classified task against the ledger and against their round patterns:

- file-safety-rules (lines 11-13): 3 rounds (new_valid/new_valid/clean), consecutive_clean reaches 1. Low_risk. Matches ledger (line 229: "LOW-risk...one clean round").
- deliberation-mode (lines 23-25): 3 rounds (new_valid/clean/clean), consecutive_clean reaches 2. Risky. Matches ledger (line 273: "RISKY...TWO clean rounds").
- no-wrap-convention (lines 28-30): 3 rounds (new_valid/clean/clean), consecutive_clean reaches 2. Risky. Matches ledger (line 291: "RISKY...TWO clean rounds").
- state-schema-inc2 (lines 39-42): 4 rounds (3x new_valid, then clean), consecutive_clean reaches 1. Low_risk. Matches ledger (line 325: "LOW-risk...one clean round"). The three new_valid rounds reflect three successive hardenings of the drift guard, all correctly logged.
- human-review-queue (lines 26-27): 2 rounds (new_valid/clean), consecutive_clean reaches 1. Low_risk. Matches ledger (line 285: "LOW-risk...one clean round").

All valid_finding counts and severity arrays for explicitly-classified tasks checked against the ledger round summaries. All match.

---

## Summary

1 high, 1 medium, 2 low. The single high finding is a real defect: `pack/instrument.md` and its generated copies describe `risk_class` as optional and state that records without it still validate, but the code now enforces it as required. The medium finding is a structural tension in the backfill (grandfathered informal-convergence records carry `low_risk` but lack a terminal clean round) that the W3 invariant design will need to address. No misclassifications found: the 2 risky tasks are correctly identified and no risky artifact was missed.
