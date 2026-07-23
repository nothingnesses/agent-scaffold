# Review: triager-on-findings (Q-63) - capture fidelity + receipt/plan integrity + mechanical

Reviewer B lens. Branch `plan/triager-on-findings`, commit `423bf9b`, base main `cfeec01`.

## Verdict

No valid findings. Reviewed the capture fidelity, receipt integrity, plan/sidecar
integrity, scope, and mechanical (validator/test) aspects and found nothing to raise.

## What I checked

### Q-63 question capture (docs/plans/agent-scaffold.plan.toml)

- The new `[[question]]` has `id = "Q-63"`, `status = "decided"`,
  `folded_into = "triager-runs-only-on-findings"`, `receipt = "Q-63"`. All present and correct.
- The `ask` records the decision faithfully: findings-only rule for ALL reviewers-then-triager
  passes (plan-review and work-review convergence rounds, the acceptance pass, and the
  review-entry-mode pass); "zero findings" read objectively from the committed reviewer findings
  files, not at the orchestrator's discretion; never-collapse preserved (still always a separate
  agent or a human when the triager does run); chosen over the always-run rule and over the
  loop-only carve-out; driver-side reflection deferred to the workflow FSM work in `src/next.rs`.
- Not overstated as built: the ask says "This is guidance-only for now; the driver (`src/next.rs`)
  reflects it later ... deferred, evidence-gated driver work."

### Q-63 build step (docs/plans/agent-scaffold.plan.toml)

- New `[[step]]` `slug = "triager-runs-only-on-findings"`, `status = "not-started"`, `order = 78`,
  `[step.provenance] decisions = ["Q-63"]`. Correct.
- `order = 78` is unique (one occurrence) and is the next value after the prior max of 77.
- Totals: 78 steps, 63 questions in the plan (matches the expected counts).

### Step sidecar (docs/plans/agent-scaffold.steps/triager-runs-only-on-findings.md)

- Describes the guidance change (Roles-separation paragraph, Triager role bullet, phases 3/4/5,
  review-entry-mode paragraph, convergence-decision line, authored-conflict-resolution rule, the
  orchestrator prompt, and the regenerated `.agents/` copies).
- States the never-collapse property is preserved and the objective "zero findings" read.
- Records the deferred driver-side reflection owed to the workflow FSM work in `src/next.rs`.

### Question sidecar convention

- `docs/plans/agent-scaffold.questions/Q-63.md` is a 0-byte placeholder. Spot-checked siblings
  `Q-62.md` (0 bytes) and `Q-61.md` (0 bytes): both are also 0-byte placeholders, so Q-63 matches
  the existing convention and is not an anomaly.

### Receipt integrity (docs/metrics/workflow.jsonl)

- Record count 173 -> 174. The change is a pure append: the diff modifies no existing line and
  adds exactly one line at the end.
- The appended record: `type:"decision"`, `q_id:"Q-63"`, `task:"triager-runs-only-on-findings"`
  (equals the `folded_into` slug), `options` is a non-empty array
  `["Adopt, all passes","Adopt, loop only","Keep always-run"]`, `recommendation:"Adopt, all passes"`,
  `chosen:"Adopt, all passes"`.
- `chosen` IS a member of `options`: "Adopt, all passes" appears verbatim as the first option.

### Scope

- Changed files are exactly the expected set: `pack/AGENTS.md`, `pack/prompts/orchestrator.md`,
  the generated copies (`AGENTS.md`, `.agents/AGENTS.reference.md`, `.agents/prompts/orchestrator.md`),
  `docs/plans/agent-scaffold.plan.toml`, the two new sidecars (step + question),
  `docs/plans/agent-scaffold.md`, and `docs/metrics/workflow.jsonl`. Ten files, no others.
- No `src/`, no ledger, no other file touched.
- `pack/prompts/triager.md` was NOT changed (intentional); it does not appear in the diff. The only
  "triager" match in the diff name list is the new step sidecar
  `docs/plans/agent-scaffold.steps/triager-runs-only-on-findings.md`, which is expected.

### Generated view

- `docs/plans/agent-scaffold.md` contains the Q-63 / step additions. Covered by `render --check`
  below (up to date), so no byte-level check was needed.

### Prose (added plan text and sidecar)

- No em-dashes, en-dashes, curly quotes, emoji, or other non-ASCII in the added plan.toml lines,
  the step sidecar, or the appended jsonl record. ASCII only. No hard-wrapping introduced. No AI
  filler phrasings.

## Validator and test results

Ran against `423bf9b` content checked out into the worktree, then restored:

- `validate --source ...plan.toml`: 174 records valid; 78 steps, 63 questions, valid.
- `validate --source ...plan.toml --workflow`: 174 records valid; 78 steps, 63 questions, valid;
  workflow invariants hold.
- `render --check ...plan.toml`: up to date.
- `cargo test`: all pass (342 + 3 + 2 + 1 + 1), 0 failed.
