# Plan-fold review (fidelity/correctness): Q-66 + Q-67

Reviewer: independent adversarial reviewer, FIDELITY and CORRECTNESS lens.
Artifact: plan fold `c0feb59..1ce5af9` (steps `reviewer-reproducible-evidence` order 88 /
`planner-folds-decisions` order 89, questions Q-66/Q-67, two decision receipts, re-rendered plan).
Worktree: `.claude/worktrees/q66-plan-review-fidelity` at `1ce5af9`. Read-only; this is the only file written.

## Gate results (deterministic)

- `cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` -> PASS.
  Tail:
  ```
  docs/metrics/workflow.jsonl: 204 records, valid
  docs/plans/agent-scaffold.plan.toml: 89 steps, 67 questions, valid
  docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
  ```
  W3/W4/W5 hold; both new decision receipts are accepted.
- `cargo run --quiet -- render --check docs/plans/agent-scaffold.plan.toml` -> PASS
  (`docs/plans/agent-scaffold.plan.toml: up to date`, exit 0). The rendered `agent-scaffold.md`
  reflects Q-66/Q-67 and steps 88/89.

## Finding 1 (LOW): "(Option B)" label for Q-66 does not match the receipt's `options` array position

- Evidence:
  - `docs/plans/agent-scaffold.plan.toml:1669` (Q-66 `ask`), verbatim: `DECIDED (Socratic, human,
    2026-07-26): TIERED REPRODUCIBLE EVIDENCE (Option B).` The same label is repeated in the sidecar,
    `docs/plans/agent-scaffold.steps/reviewer-reproducible-evidence.md:5`: `The rule (Option B, tiered
    reproducible evidence).`
  - The Q-66 decision receipt records the options in this order:
    `grep -F '"q_id":"Q-66"' docs/metrics/workflow.jsonl` ->
    `"options":["Tiered reproducible evidence","Runnable test for every finding","Triager-enforced,
    reviewer SHOULD","Keep current process"]`. Mapping array positions to letters
    (options[0]=A, options[1]=B, ...), "Tiered reproducible evidence" is position A; "Option B" is
    "Runnable test for every finding".
- Why this is a (minor) inconsistency: the receipt `options` array is the only durable, structured
  record of the option set. Its ordering puts the chosen "Tiered reproducible evidence" first
  (position A), so the parenthetical "(Option B)" in the prose contradicts the sole reproducible
  letter mapping. Contrast the established convention at `docs/plans/agent-scaffold.plan.toml:1542`
  (Q-51), where the letters follow presentation order and the chosen "Option B" is genuinely the
  second option listed ("(A) build the advisory MVP ...; (B) build the full driver now; ...").
- Caveat (keeps the severity low): the "(Option B)" label plausibly references the original Socratic
  presentation order shown to the human, which is not recorded structurally; the receipt array may
  have been written chosen-first. So the label is not provably "wrong" about history, only
  inconsistent with the durable record a reader would reconstruct from.
- No functional impact: W4 only checks `chosen` is a member of `options` (both hold); validate passes.
- Fix: drop the parenthetical letter (it references an ordering the receipt does not preserve), or
  renumber it to match the receipt array position, in both `Q-66` `ask` and the sidecar.

## Checks that PASSED (no finding), with the evidence that cleared them

- Question entries (`docs/plans/agent-scaffold.plan.toml:1664-1677`): both have `status = "decided"`,
  correct `folded_into` (Q-66 -> `reviewer-reproducible-evidence`, Q-67 -> `planner-folds-decisions`),
  and `receipt = "Q-66"` / `"Q-67"`. IDs, slugs, and orders are unique
  (`grep -cE '^id = "Q-66"$'` -> 1; `^id = "Q-67"$` -> 1; `^order = 88$` -> 1; `^order = 89$` -> 1;
  the two slugs each appear once).
- Decided-rule fidelity: the Q-66 `ask` and sidecar capture every element of the decision, the
  PROPORTIONAL/tiered qualifier, the runnable/mutation demonstration for behavioral claims
  (break C, show T still passes), the exact-command-or-`file:line` tier for doc/design/style, and the
  triager-reproduces-and-dismisses-any-non-reproducing-testable-claim half. The Q-67 `ask`/sidecar
  capture naming the PLANNER as folder at the human-input-contract / Socratic-mode / checkpoint points
  and the reinforcement that the orchestrator's closed direct-on-main list excludes authoring new
  questions/steps. Nothing drops or misstates a decided qualifier.
- Q-67 diagnosis is faithful to the live AGENTS.md text: `pack/AGENTS.md:39` has "non-trivial and
  routes to the planner to fold into the plan"; `:41` has the passive "A resolved decision is recorded
  ... and folded into the step it affects"; `:43` has the Socratic "reuses the intake and
  Open-Questions machinery"; `:71` has "the orchestrator updates this queue". All four quoted diagnoses
  reproduce.
- Receipts (W4): `chosen` in `options` for both; options match the presented sets given in the charter;
  `q_id` = Q-66/Q-67; `task` = the `folded_into` slug (`reviewer-reproducible-evidence` /
  `planner-folds-decisions`). The "past the `Q-44` baseline" claim is correct:
  `docs/plans/agent-scaffold.plan.toml:3` = `w4_baseline = "Q-44"`.
- Step scoping: Q-66 sidecar names `pack/prompts/reviewer.md` + `pack/prompts/triager.md` +
  `pack/AGENTS.md` (plus the `.agents/` regen); Q-67 sidecar names `pack/AGENTS.md` only. Matches the
  charter's expected file sets.
- Documentation-currency + step-80 guard: `agents-md-drift-guard` is order 80 (`status = "complete"`),
  and `src/agents_md_drift.rs:1-2` confirms it is a whole-file guard for `AGENTS.md` and
  `.agents/AGENTS.reference.md`. The Q-66/Q-67 sidecars describe it accurately and do not overclaim
  that it covers the prompt files.
- Step-87 synergy note is correct: `Kind::Mutation` is at `src/checks.rs:94-95` (doc comment + variant),
  step 87 is `code-value-audit-deletion-experiment` (order 87). The reviewer step correctly states it
  is prompt-and-guidance only and does NOT touch `src/checks.rs`.
- Orders/statuses: 88 (`next`) / 89 (`not-started`), no collision with existing steps (85..87 present,
  90+ absent). Consistent.
- Principle citations are accurate by name: Principle 1 "Prefer the cleaner long-term architecture over
  the smallest diff", 2 "Minimal by default", 6 "Ground decisions in evidence", 7 "Reproducible", 8
  "Structured data first, project for humans" (`docs/plans/agent-scaffold.plan.toml:1680,1685,1705,
  1710,1715`). The five cited AGENTS.md workflow-guidance phrases ("Verify, don't trust"; "Cite sources
  rather than asserting from memory"; "Tests must actually exercise the code they claim to"; "Make
  documentation self-contained"; "No silent scope expansion") each appear verbatim in the rendered
  root `AGENTS.md` (`grep -c -F` returns 1 for each). Note they are sourced from
  `pack/principles.toml`, not `pack/AGENTS.md`; the sidecars cite them as "AGENTS.md workflow guidance",
  which is correct for the rendered `AGENTS.md`.
- Empty question sidecars `Q-66.md` / `Q-67.md` are the established pattern (Q-50..Q-65 are empty too),
  not a defect.

## Summary

One LOW finding (the "(Option B)" label mismatch), which is cosmetic and does not affect validation.
All other fidelity and correctness checks pass with reproducible evidence. `validate --workflow` and
`render --check` both PASS.
