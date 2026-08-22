# Brief: `Q-78` design pass, round 3 of the reset count, GROUND-BLIND FALSIFICATION lens

This file is the brief itself, not a findings file. The orchestrator writes it before it dispatches, so a re-dispatch after a context loss reads this file rather than the orchestrator's memory. An earlier round 3 dispatch died three times to server errors, and the briefs existed only in conversation context, so they were lost. `AGENTS.md:106` names that failure: a fact citable only from conversation history is not citable.

## Your role

You are a reviewer. You are read-only with respect to the plan and the code. You author your own findings file and nothing else. You do not fix anything you find.

## Your lens

Split each stated ground into its premise and its consequence. Build a wrong implementation for each half. The criteria must fail an implementation that falsifies the premise while the consequence still holds.

This wording is the third form of the obligation. The first form asked for one wrong implementation per criterion group, and four holes survived it. The second form asked for one per stated ground, and it closed those four. The second form was still too weak, because an attack on a ground's consequence passes whenever the consequence has another cause. Use the third form above.

A stated ground is any sentence that justifies a design choice, a risk class or a numbered RULE. It is not the operation the increment performs.

## Target

- Branch `plan/q78-design-pass`, based on `main` at `a6e1d7f`.
- Read and write inside the worktree the orchestrator names in your prompt. Do not touch the main repository.

## Scope

Five step sidecars, carrying 13 increments, all classed `risky`.

1. `docs/plans/agent-scaffold.steps/step-intent-encoding.md`, increments `-inc1`, `-inc2a` to `-inc2f`, `-inc3`.
2. `docs/plans/agent-scaffold.steps/plan-order-array-position.md`, increments `-inc1`, `-inc2`.
3. `docs/plans/agent-scaffold.steps/sidecar-status-opening-drift.md`, increment `-inc1`.
4. `docs/plans/agent-scaffold.steps/ledger-order-citation-currency.md`, increment `-inc1`.
5. `docs/plans/agent-scaffold.steps/validate-missing-source-exit.md`, increment `-inc1`.

The frozen design document is `docs/plans/step-intent-encoding.explorations/Q-78.md`. Read it as the input the criteria are measured against. Do not review it.

## What decides cleanliness

The stop condition names a class and a count, because a condition that names specific passages does not fire. A previous loop set such a condition and still ran eight rounds.

- CLASS 1, a ground-blind criterion, meaning a wrong implementation passes while it violates a stated risk ground, a numbered RULE or a cited Principle. The count that keeps the round clean is ZERO.
- CLASS 2, a second-guard hole or a non-reproducing figure. THREE OR FEWER, all `low` or `medium`, and the round is clean.

A figure counts as class 2 if and only if it sits inside a step sidecar's increment block. That means an acceptance criterion, that increment's risk-class ground, or a numbered RULE. Every other location is excluded, including `Q-78.md`, any `[[question]].ask`, the ledger and its resume anchor. The test is where the number is written, not what it is about.

Currency defects, stale counts outside an increment block, and missing receipts do not bear on cleanliness. Report them, but mark them as not bearing.

## The three rules the one clean sidecar obeys

`ledger-order-citation-currency.md` is the only sidecar in this set whose every stated figure reproduced exactly. Three rules distinguish it. Use them as the standard you measure the other four against.

1. Its search set does not contain itself.
2. It refuses, as a stated rule, to write a concrete example into itself.
3. Every figure it states is a command's output, never a pass condition.

Rule 3 is the one that broke a sibling sidecar. That sibling wrote its expected counts as fixed numbers, then edited the very files those numbers count.

## Out of scope

The design mechanism is not under review. Four recommendations survived four rounds unattacked, and no round 4 finding attacked any of them.

1. Array-position authority, with `order` deleted.
2. Two required single-line intent fields, projected through `render`, `next` and `status --step`.
3. A migration record that lives outside the plan.
4. A cited backfill in bounded batches.

The human decisions are settled and are not reopenable. They include the required fields, the per-batch split, the earliest-containing-commit rule for the backfill `source` column, the widening of the drift step to all 45 sidecars, and the four accepted residual risks. If you believe a decision is wrong, say so as a separate note, and do not file it as a finding.

Umbrella membership left this pass and became `Q-79`. Do not review it.

## Gates

Run these from the worktree root. The `<PLAN>` argument is required.

```
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl
cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
cargo run --quiet -- render --check --strict docs/plans/agent-scaffold.plan.toml
LC_ALL=C grep -rcP '[^\t\x20-\x7e]' docs/plans/
```

## Known defects, so you do not re-discover them

- `render --check --strict` exits 2 when the `<PLAN>` argument is absent. Every agent last loop hit this and worked around it silently. Report any gate line that does not run.
- `validate` exits 0 when its input file is absent, printing "nothing to validate". This applies to `--source`, `--plan` and an explicit `--metrics`. A criterion whose pass condition is a bare exit 0 therefore also passes on a tree with no plan. `validate-missing-source-exit` is the step that repairs this. Never treat a bare exit 0 as proof. Pin the `N steps, M questions, valid` line instead.
- `grep -c` exits 1 when it matches nothing, and that is the PASS case for a sweep. Never chain a sweep with `&&`.
- This shell replaces `grep` with `ugrep`. Use `/usr/bin/grep` wherever an escape or a `-P` pattern matters. A reviewer's escaping result diverged once because of this.
- Never run `nix fmt` and never run `just scaffold-self`. The repository is not formatter-clean at `HEAD` and does not enforce it. `just scaffold-self` runs `nix fmt` tree-wide and reflows about 56 files.

## File safety

Build every fixture only under the session scratchpad, in a subdirectory you name yourself. Do not write into bare `/tmp`. Do not delete anything outside your own fixture subdirectory. Never use a wildcard glob in a delete. Restore the mode of any 000 or 600 fixture before you finish.

## Your findings file

Write to the path the orchestrator gives you in your prompt.

Write as you go. Append each finding when you confirm it. Two reviewers in this loop died to a session limit having written nothing, and their work was lost.

Give every finding a severity on the four-level `low`, `medium`, `high`, `critical` scale. The severity rates the finding's impact if it is left unfixed. It is not a ranking against your other findings.

Every finding carries reproducible evidence, proportional to its claim. For a behavioural claim, give the runnable wrong implementation and its measured output. For a documentation or design claim, give an exact command or a `file:line` citation. Do not build a contrived test where a command or a citation already settles the point.

State the class of each finding, 1 or 2, against the stop condition above.

Report the counts you measured, not the counts this brief states.
