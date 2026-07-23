# Review findings: Q-64 capture (faithfulness + mechanical lens)

Branch `plan/q64-capture`, commit `b6b9661`, base main `272e9eb`. Reviewed the Q-64
Open-Questions capture under the faithfulness and mechanical lens. No valid findings.
The capture is faithful, complete, and mechanically sound.

## Result: no findings

## Structure (confirmed correct)

- Q-64 is a valid `exploring` question: `id = "Q-64"`, `status = "exploring"`, and a
  single `ask` string. It carries NO `folded_into`, NO `receipt`, NO step reference, and
  no metrics/receipt fields, which is the correct shape for a question captured before it
  is decidable.
- Field shape matches the existing `exploring` sibling Q-52 exactly (id, status,
  ask only; no folded_into/receipt).
- The question sidecar `docs/plans/agent-scaffold.questions/Q-64.md` exists in the commit
  as a 0-byte file (blob `e69de29`, the canonical empty-blob hash), matching the sibling
  placeholders Q-52, Q-60, Q-61, Q-62, Q-63 (all the same empty-blob hash).

## Faithfulness and completeness (all major parts present)

The `ask` carries the full design-space framing. Confirmed present:

- The TWO-PART question: (1) does AGENTS.md still make sense / is the tool by itself
  insufficient; (2) how far the tool can be pushed toward self-sufficiency.
- The STARTING ANALYSIS: tool alone insufficient for three reasons (judgment and
  behavioral guidance the tool can prompt but not enforce; bootstrap and
  harness-agnosticism; the human-readable contract), AND a hand-maintained AGENTS.md is
  also wrong because of drift (the driver principle-numbering bug and the W4-baseline doc
  drift are cited), with the synthesis being a GENERATED AGENTS.md single-sourced with the
  tool.
- The LADDER: (1) DELIVERY via `next`; (2) SOURCE via a widened `workflow.toml` plus
  generating the AGENTS.md workflow section; (3) ENFORCEMENT via `record-round`
  subcommands and the opt-in `validate --workflow --strict` boundary gate plus invariants;
  (4) BOOTSTRAP to a generated stub.
- The IRREDUCIBLE FLOOR: (a) judgment; (b) unobservable behavioral compliance; (c)
  self-bootstrap; plus the CHECKABLE-versus-PROMPTABLE test for whether a rule can move
  into the tool.
- The RECONCILIATION with Q-51 (completing the workflow-driver vision plus widening
  generation) and Q-58 (the structured transient as one input the state reconstructor
  needs).
- The `exploring` framing: a multi-explorer design pass is owed with candidate lenses, and
  a design record is owed under `docs/plans/agent-scaffold.explorations/` named for Q-64.

No major part is dropped or misstated.

## Cross-reference accuracy (confirmed)

- The claim that Q-51's own text names generating the AGENTS.md workflow section from the
  machine definition is TRUE: Q-51's `ask` contains "the AGENTS.md workflow section is
  GENERATED from the machine definition".
- The `ISOLATION_POLICY_FRAGMENT` / `{{isolation_policy}}` single-sourcing claim is taken
  as given per the review brief.
- Q-58 is about the driver projecting the transient resume-state, consistent with Q-64's
  "structured transient is one input the state reconstructor needs".
- Principle citations match the plan's `[[principle]]` names: P1 = "Prefer the cleaner
  long-term architecture over the smallest diff", P2 = "Minimal by default", P6 = "Ground
  decisions in evidence", P8 = "Structured data first, project for humans". P8/P1 are
  cited as driving the generated-projection direction and P2/P6 as cautioning against
  building the whole ladder speculatively; both usages are correct.

## Scope (confirmed clean)

The diff `272e9eb..b6b9661` touches only three files: `docs/plans/agent-scaffold.plan.toml`
(the Q-64 block, +17 lines), `docs/plans/agent-scaffold.md` (the generated projection: open
questions count 5 -> 6 and the rendered Q-64 entry, +15/-1), and the new 0-byte
`docs/plans/agent-scaffold.questions/Q-64.md`. No pack, src, ledger, metrics, or step
sidecar changed.

## Validators (run and read)

- `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow`:
  176 records valid; 78 steps, 64 questions, valid; workflow invariants hold.
- `cargo run -- render --check docs/plans/agent-scaffold.plan.toml`: up to date.

Both match expectations (78 steps, 64 questions, 176 records, invariants hold, render up to
date). The `docs` tree was restored to HEAD after running.

## Prose (confirmed clean)

The Q-64 `ask` is ASCII-only (no non-ASCII bytes), no em-dashes or unicode or emoji, uses
blank-line-separated unwrapped paragraphs (matching other long asks), and has no AI filler.
