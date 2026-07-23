# Review: Q-65 capture (faithfulness + mechanical lens)

Branch `plan/q65-capture`, commit `ce2bef2`, base main `5140821`.
Scope reviewed: `docs/plans/agent-scaffold.plan.toml` (the new Q-65 block), the generated
`docs/plans/agent-scaffold.md`, and the new sidecar `docs/plans/agent-scaffold.questions/Q-65.md`.

## Verdict

Reviewed under the faithfulness and mechanical lens. No valid findings. The capture is faithful,
complete, and mechanically sound.

## Structure confirmed

- Q-65 is a valid `exploring` question: `id = "Q-65"`, `status = "exploring"`, a single `ask` string.
- No `folded_into`, no `receipt`, no step attached. Same shape as the `exploring` questions Q-52 and Q-64.
- Sidecar `docs/plans/agent-scaffold.questions/Q-65.md` exists in `ce2bef2` (blob `e69de29`, git-empty) and is 0 bytes.

## Faithfulness and completeness confirmed (all eight parts present)

1. Two-part question present: scaffolding-vs-workflow-engine, and whether/what to rename.
2. Identity analysis present: `scaffold` is now one bootstrap subcommand; `validate --workflow` enforces,
   `render` projects, `next` drives; described as a harness-agnostic structured workflow engine/driver
   (a control plane for a role-separated, convergence-gated workflow).
3. NOT-a-meta-harness category-error point present: a harness runs the agent (LLM loop and tool execution),
   which this tool never does; agents invoke the tool, so meta-harness is a category error.
4. Rename-worth-it present with the comprehension-tax argument and the human's not-precious /
   no-backwards-compat / no-brand-recognition stance (stated in both the opener and the RENAME paragraph).
5. Timing present: the name follows the settled identity; resolved as part of the Q-64 design pass.
6. Candidate name directions present: descriptive (`agentflow`, `flowgate`, `agentctl`) and metaphor
   (`cox`/`coxswain`, `maestro`/`podium`, `helm`), with the orchestrator lean toward a descriptive name
   and `agentflow`/`flowgate` as front-runners.
7. P8/P1 principle judgment present: P8 (structured data first, project for humans) and P1 (cleaner
   long-term architecture) argue the public name should accurately project the tool's identity.
8. Rides-along-with-Q-64 note present: no separate design pass owed beyond Q-64's; the question rides along.

## Cross-references confirmed

- Q-65 links to `Q-51` (the workflow-driver) and `Q-64` (tool-self-sufficiency / AGENTS.md-necessity);
  both exist in `ce2bef2`. Q-64's `ask` is confirmed to be about whether a separate AGENTS.md should exist
  and how far the tool can be pushed toward self-sufficiency, matching Q-65's description.
- Principle names used correctly: P8 = structured-data-first, project for humans; P1 = cleaner long-term
  architecture. Consistent with Q-64's usage.

## Scope confirmed

Only three files changed: `M docs/plans/agent-scaffold.md`, `M docs/plans/agent-scaffold.plan.toml`,
`A docs/plans/agent-scaffold.questions/Q-65.md`. No scope leak.

## Validators (ran and read output, then restored docs to HEAD)

- `validate --source docs/plans/agent-scaffold.plan.toml --workflow`: PASS.
  177 records valid; 78 steps, 65 questions, valid; workflow invariants hold.
- `render --check docs/plans/agent-scaffold.plan.toml`: PASS. Plan reported up to date.

## Prose

The Q-65 block is ASCII-clean: no em-dashes, no unicode, no emoji, no hard-wrapping within paragraphs,
no AI filler.
