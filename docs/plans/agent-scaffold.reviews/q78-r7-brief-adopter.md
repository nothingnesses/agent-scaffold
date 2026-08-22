# Brief: `Q-78` design pass, round 3 of the reset count, ADOPTER AND EXECUTABILITY lens

This file is the brief itself, not a findings file. The orchestrator writes it before it dispatches, so a re-dispatch after a context loss reads this file rather than the orchestrator's memory. An earlier round 3 dispatch died three times to server errors, and the briefs existed only in conversation context, so they were lost. `AGENTS.md:106` names that failure: a fact citable only from conversation history is not citable.

## Your role

You are a reviewer. You are read-only with respect to the plan and the code. You author your own findings file and nothing else. You do not fix anything you find.

## Why this lens exists

This lens has never been run on this artefact. Six review rounds pointed at the design, at consistency, at the acceptance criteria, at implementability and at ground-blindness. None asked what the change costs somebody who is not this repository.

`agent-flow` ships a pack. Any project can scaffold from it. This pass makes two intent fields REQUIRED in the shipped pack as well as here. The human weighed and accepted the stated cost: once the migration finishes, a new step in any scaffolded project needs two prose sentences before its plan parses, and the template must ship placeholder values. The human also accepted the residual that a required field is satisfiable by a placeholder forever, which is the property `title` has carried since the schema shipped.

Your job is to find where that stated cost is wrong, understated, or unbuildable. The decision itself is settled and is not reopenable.

## Your two questions

1. THE ADOPTER QUESTION. What must a project that scaffolds from the new pack write before its plan parses? Answer it by construction, not by reading. Scaffold a fresh project into your fixture directory, apply what the sidecars specify, and record what actually fails and what the operator must type. State the failure message verbatim.
2. THE EXECUTABILITY QUESTION. Can an implementer execute each increment as written, in the declared order, using only the commands the sidecar states? Run them. An increment whose criteria cannot be evaluated without a command the sidecar does not give is a finding.

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

The pack lives under `pack/`. The frozen design document is `docs/plans/step-intent-encoding.explorations/Q-78.md`. Read the design document as the input, and do not review it.

## How to scaffold a fresh project

Use this command. Never run `just scaffold-self`, because its second line runs `nix fmt` tree-wide over a repository that is not formatter-clean, and it reflows about 56 files.

```
cargo run --quiet -- scaffold --output-dir <your-fixture-dir> --write --force --principles default --instrument
```

## What the adopter cost turns on

Read these and test each one against a scaffolded tree.

- The schema. `Step` is at `src/plan/source.rs:129` and carries `#[serde(deny_unknown_fields)]`. A required field with no default makes an existing plan fail to parse.
- The projection. `roadmap_section` at `src/plan/render.rs:465` writes the slug, the status and a Notes cell. `step_details_section` at `src/plan/render.rs:573` iterates `for (_, body)` and discards the `Step`, so `[[step]].title` reaches no rendered surface today. Check what the new fields actually reach, and whether the sidecars claim more.
- The selector. `select_active_loop` at `src/next.rs:704` picks the lowest-order in-progress step, then the lowest-order ready pending step, then the lowest-order pending step. Deleting `order` changes the input to this function. An adopter's `next` output must stay defined.
- The template. If the pack must ship placeholder values, find where, and check that a scaffolded plan parses immediately after `scaffold` with no hand editing.

## Out of scope

The design mechanism is not under review. Four recommendations survived four rounds unattacked.

1. Array-position authority, with `order` deleted.
2. Two required single-line intent fields, projected through `render`, `next` and `status --step`.
3. A migration record that lives outside the plan.
4. A cited backfill in bounded batches.

The human decisions are settled. Umbrella membership left this pass and became `Q-79`. Do not review it.

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
- `validate` exits 0 when its input file is absent, printing "nothing to validate". This applies to `--source`, `--plan` and an explicit `--metrics`. `--workflow` correctly exits 1, and its help already publishes the rule that a check which did not run must not report success. A criterion whose pass condition is a bare exit 0 therefore also passes on a tree with no plan. `validate-missing-source-exit` is the step that repairs this. Pin the `N steps, M questions, valid` line instead.
- `grep -c` exits 1 when it matches nothing, and that is the PASS case for a sweep. Never chain a sweep with `&&`.
- This shell replaces `grep` with `ugrep`. Use `/usr/bin/grep` wherever an escape or a `-P` pattern matters.
- Never run `nix fmt`.

## File safety

Build every fixture only under the session scratchpad, in a subdirectory you name yourself. Do not write into bare `/tmp`. Do not delete anything outside your own fixture subdirectory. Never use a wildcard glob in a delete. Restore the mode of any 000 or 600 fixture before you finish.

## Your findings file

Write to the path the orchestrator gives you in your prompt.

Write as you go. Append each finding when you confirm it. Two reviewers in this loop died to a session limit having written nothing, and their work was lost.

Give every finding a severity on the four-level `low`, `medium`, `high`, `critical` scale. The severity rates the finding's impact if it is left unfixed. It is not a ranking against your other findings.

Every finding carries reproducible evidence, proportional to its claim. For a behavioural claim, give the runnable demonstration and its measured output. For a documentation or design claim, give an exact command or a `file:line` citation.

Report the counts you measured, not the counts this brief states.
