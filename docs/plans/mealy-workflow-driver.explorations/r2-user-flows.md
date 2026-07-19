# Q-51 round-2 exploration: user flows and end-to-end scenarios

Lens: USER FLOWS / END-TO-END SCENARIOS. This document makes the full Mealy-machine workflow driver concrete by walking real scenarios step by step. It is one of four round-2 design passes on Q-51; the others cover requirements and scope, design-space breadth and staging, and architecture and incremental build path. The human has confirmed the FULL DRIVER as the target; this pass validates that target against actual use, surfaces seams, and recommends an interaction model.

References read: `synthesis.md` and its four round-1 lens files (especially `io-contract-and-adoption.md` for the I/O shape and `fsm-concurrency-model.md` for the state/input/output alphabets); Q-51 `ask` in `agent-scaffold.plan.toml`; `AGENTS.md` (workflow section, roles, phases, review loop, escalation, checkpoints, isolation tiers, ledger, instrumentation schema).

## 1. Actors and their perspectives

Two actors use the driver, with different experiences.

**The orchestrator agent** is the primary caller. It calls `agent-scaffold next` at every decision point, reads the JSON output, spawns the appropriate role agent per the emitted instruction, and then calls the matching `record-*` subcommand to record the judgment and advance state. The orchestrator's loop is: `next` -> spawn role -> role produces output -> `record-*` -> `next` -> repeat. The driver is a discipline tool: if the orchestrator follows the `next_record_command` field in the tool's output faithfully, it cannot drift from the workflow rules without the tool detecting it.

**The human** does not usually call `agent-scaffold next` directly. Their interaction points are checkpoints where the driver emits an `escalate` instruction, or where the orchestrator surfaces a question per the human-input contract. The human's experience is: "the agent pauses, presents options with a recommendation, I decide, the agent continues." The driver's escalation output pre-formats the decision (options, trade-offs, recommendation, Principle references) so the orchestrator can present it cleanly without composing it from scratch. The human may also call `agent-scaffold next` independently to inspect workflow state at any moment; because the tool is read-only and stateless, this has no side effects.

## 2. Scenario 1: Kickoff

A human starts a new task. The plan template has been copied to `docs/plans/my-task.plan.toml` with a status line and a blank step list. The metrics log (`docs/metrics/workflow.jsonl`) may not yet exist. The ledger does not yet exist.

**Human's part:** paste the kickoff prompt to the orchestrator. Done.

**Orchestrator's part:**

Step 1: preflight. The orchestrator reads `AGENTS.md`, resolves the isolation tier (say, worktree), confirms with the human. No `agent-scaffold` call yet.

Step 2: the orchestrator calls:

```
agent-scaffold next \
  --source docs/plans/my-task.plan.toml \
  --isolation-tier worktree \
  --json
```

(No `--metrics` or `--ledger-fragment` yet: neither file exists.)

The driver reads the plan TOML (a fresh skeleton: no filled-in steps, no round records) and outputs:

```json
{
  "task": "my-task",
  "task_phase": "planning",
  "ready_units": [
    {
      "step": "__plan__",
      "phase": "planning",
      "status": "not-started",
      "valid_transitions": ["spawn-planner"],
      "isolation_tier": "worktree",
      "next_instruction": {
        "role": "planner",
        "prompt_path": ".agents/prompts/planner.md",
        "context": {
          "plan_source": "docs/plans/my-task.plan.toml",
          "template": "docs/plans/TEMPLATE.plan.toml"
        },
        "principle_reminders": [
          "P1 (clarify intent before coding; confirm before acting)",
          "P2 (surface open questions before implementing)"
        ],
        "filled_prompt_summary": "Spawn a planner in a worktree. Hand it: the plan TOML template and the task description. Ask it to draft the Roadmap steps, Success Criteria, and Open Questions per .agents/prompts/planner.md, then run `agent-scaffold render my-task.plan.toml` and commit.",
        "next_record_command": "agent-scaffold record-phase --task my-task --from planning --to plan_review --metrics docs/metrics/workflow.jsonl"
      }
    }
  ],
  "blocked_units": [],
  "in_review_units": [],
  "escalated_units": []
}
```

Step 3: the orchestrator creates a worktree at `.claude/worktrees/my-task-plan`, spawns the planner. The planner fills in the steps, runs `agent-scaffold render`, commits, exits.

Step 4: the orchestrator records the phase transition:

```
agent-scaffold record-phase \
  --task my-task \
  --from planning \
  --to plan_review \
  --metrics docs/metrics/workflow.jsonl
```

This appends a `type: "phase"` record. (The driver cannot tell from the plan TOML alone whether a planner has run; the transition record is the signal.)

Step 5: the orchestrator calls `next` again:

```
agent-scaffold next \
  --source docs/plans/my-task.plan.toml \
  --metrics docs/metrics/workflow.jsonl \
  --ledger-fragment docs/plans/my-task.ledger.md \
  --isolation-tier worktree \
  --json
```

The driver reads the plan (now with defined steps), the log (has the `phase` record), and outputs "IN REVIEW: spawn reviewers for plan review" (see Scenario 2).

**Gap 1 (bootstrap signal):** the driver cannot distinguish a blank plan template from a filled-in plan by inspecting the TOML alone. A `type: "phase"` event (proposed above) or a `planning_complete = true` flag in the plan TOML is needed. The TOML flag is simpler (one fewer event type), but the JSONL event is consistent with the rest of the event log. This is a missing requirement in the round-1 I/O contract.

## 3. Scenario 2: Plan review loop driven to convergence

The plan is drafted and the phase record says `plan_review`. The driver outputs "spawn reviewers."

**Round 1 (new_valid findings):**

The orchestrator spawns two reviewers (different models). Reviewers write to:
- `docs/plans/my-task.reviews/__plan__-reviewer-claude-sonnet-1.md`
- `docs/plans/my-task.reviews/__plan__-reviewer-claude-opus-1.md`

The orchestrator spawns a separate triager, who writes to `docs/plans/my-task.reviews/__plan__-triage.md`. The triager confirms 2 valid findings.

Orchestrator records:

```
agent-scaffold record-round \
  --task my-task \
  --step __plan__ \
  --phase plan_review \
  --outcome new_valid \
  --risk-class low_risk \
  --valid-findings 2 \
  --severities medium,low \
  --top-dismissed-severity none \
  --metrics docs/metrics/workflow.jsonl
```

The command validates the transition (plan_review running, new_valid is legal from streak=0), appends the JSONL record, and outputs:

```
TRANSITION: __plan__ plan_review round 1: new_valid (streak stays 0, total rounds = 1)
next: route_fix -> spawn planner to address 2 valid findings, then spawn reviewers (round 2)
next_record_command: agent-scaffold record-round --task my-task --step __plan__ --phase plan_review --outcome <clean|new_valid> ...
```

The orchestrator spawns an implementer (planner role for plan revision). Planner addresses findings, re-renders, commits.

**Round 2 (clean, no high or critical dismissals):**

Orchestrator calls `next`. Driver: "spawn reviewers (round 2, fresh models)." Orchestrator spawns new reviewers. No valid findings. Triager confirms clean.

```
agent-scaffold record-round \
  --task my-task \
  --step __plan__ \
  --phase plan_review \
  --outcome clean \
  --risk-class low_risk \
  --valid-findings 0 \
  --severities "" \
  --top-dismissed-severity none \
  --metrics docs/metrics/workflow.jsonl
```

Command validates: plan_review, streak was 0, clean -> streak = 1. Required streak for low_risk = 1. 1 >= 1 -> CONVERGED. Output:

```
CONVERGED (__plan__ / plan_review):
  __plan__   plan_review, low_risk, streak 1/1 - CONVERGED (2 total rounds)
  valid transitions: start-implementing
  next: plan review is complete; compute the ready frontier.
    action: call `agent-scaffold next` to see which Roadmap steps may start.
    cleanup: commit findings files, then delete as committed deletions:
      docs/plans/my-task.reviews/__plan__-reviewer-claude-sonnet-1.md
      docs/plans/my-task.reviews/__plan__-reviewer-claude-opus-1.md
      docs/plans/my-task.reviews/__plan__-triage.md
```

**Streak counting visible to the human:** at the step-boundary checkpoint after round 2, the orchestrator reports "plan review converged in 2 rounds (1 fix round, 1 clean round); moving to implementation." The human does not decide anything here.

**Key design point:** the driver pre-computes the findings file paths using the `<step>-<role>-<disambiguator>.md` convention. This removes the orchestrator's transcription step and prevents collision. The paths appear in the `next` output before the round starts (so reviewers are given the correct path) and in the cleanup checklist after convergence.

## 4. Scenario 3: Implement and review of a Roadmap step

Plan review converged. Orchestrator calls `next`. Driver computes the ready frontier.

Say the ready step is `state-queries` (not-started, `blocked_by = []`). The orchestrator classifies the artifact as risky (wide blast-radius change to `src/main.rs`).

**Driver output (from `next`):**

```
READY (1 unit):
  state-queries   not-started -> in-progress
  isolation_tier: worktree
  next: spawn implementer in worktree
    prompt: .agents/prompts/implementer.md
    worktree: .claude/worktrees/state-queries
    branch: state-queries-impl
    context:
      step = state-queries
      plan_source = docs/plans/agent-scaffold.plan.toml
      ledger = docs/plans/agent-scaffold.ledger.md
    principle_reminders: ["P4 (small reviewable changes)", "P6 (verify: run it and test it)"]
    next_record_command: "agent-scaffold record-step-start --task agent-scaffold --step state-queries --risk-class risky --metrics docs/metrics/workflow.jsonl"
```

**Orchestrator:** creates worktree, spawns implementer. Implementer makes changes and commits (in the worktree). Orchestrator records:

```
agent-scaffold record-step-start \
  --task agent-scaffold \
  --step state-queries \
  --risk-class risky \
  --diff-base abc123 \
  --metrics docs/metrics/workflow.jsonl
```

This appends a record noting the step has started, its risk class, and the diff base commit. The orchestrator updates the ledger with the in-flight state (step, diff_base), then calls `next`.

**Driver output:**

```
IN REVIEW (state-queries):
  state-queries   work_review, risky, streak 0/2 (need 2 consecutive clean rounds)
  valid transitions: record-round-clean, record-round-new-valid
  isolation_tier: worktree (reviewers read the branch; write findings to main)
  next: spawn reviewers on branch state-queries-impl
    prompt: .agents/prompts/reviewer.md
    context:
      step = state-queries
      artifact = src/main.rs
      diff_range = abc123..HEAD (from ledger)
      ledger = docs/plans/agent-scaffold.ledger.md
      findings_file_a = docs/plans/agent-scaffold.reviews/state-queries-reviewer-claude-sonnet-1.md
      findings_file_b = docs/plans/agent-scaffold.reviews/state-queries-reviewer-claude-opus-1.md
      triage_file = docs/plans/agent-scaffold.reviews/state-queries-triage.md
    note: risky artifact; 2 consecutive clean rounds required; streak resets to 0 on any new_valid round.
    principle_reminders: ["P5 (independent adversarial review)", "P13 (illegal states unrepresentable)"]
    next_record_command: "agent-scaffold record-round --task agent-scaffold --step state-queries --phase work_review --risk-class risky --outcome <clean|new_valid> --top-dismissed-severity <none|low|medium|high|critical>"
```

**Round 1 (new_valid):** reviewers find a bug. Triager confirms valid. Orchestrator:

```
agent-scaffold record-round \
  --task agent-scaffold --step state-queries --phase work_review \
  --outcome new_valid --risk-class risky --valid-findings 1 --severities medium \
  --top-dismissed-severity none --metrics docs/metrics/workflow.jsonl
```

Driver: streak=0, total=1, new_valid -> route_fix. Output: "spawn implementer to fix; then round 2."

Implementer fixes. Orchestrator calls `next`. Driver: "spawn reviewers (round 2)."

**Round 2 (clean, but a high-severity finding was dismissed):**

```
agent-scaffold record-round \
  --task agent-scaffold --step state-queries --phase work_review \
  --outcome clean --risk-class risky --valid-findings 0 \
  --top-dismissed-severity high --metrics docs/metrics/workflow.jsonl
```

Driver detects top-dismissed-severity=high: RECHECK required before counting this round as clean. Output:

```
RECHECK REQUIRED (state-queries, round 2):
  A dismissed finding of severity=high was flagged.
  Convergence is BLOCKED until a second, independent triager confirms the dismissal.
  valid transitions: record-recheck-upheld, record-recheck-overturned
  next: spawn a second independent triager (or a human) for the high-severity dismissal
    triage_file: docs/plans/agent-scaffold.reviews/state-queries-triage-recheck.md
    next_record_command: "agent-scaffold record-recheck --task agent-scaffold --step state-queries --result <upheld|overturned>"
```

Second triager upholds the dismissal. Orchestrator:

```
agent-scaffold record-recheck \
  --task agent-scaffold --step state-queries --result upheld \
  --metrics docs/metrics/workflow.jsonl
```

Driver: dismissal upheld, round 2 counts as clean, streak=1. Required=2. 1 < 2 -> spawn round 3. Output: "spawn reviewers (round 3)."

**Round 3 (clean, no high or critical dismissals):**

```
agent-scaffold record-round \
  --task agent-scaffold --step state-queries --phase work_review \
  --outcome clean --risk-class risky --valid-findings 0 \
  --top-dismissed-severity none --metrics docs/metrics/workflow.jsonl
```

Driver: streak=2 >= required=2 -> CONVERGED. Output:

```
CONVERGED (state-queries / work_review):
  state-queries   work_review, risky, streak 2/2 - CONVERGED (3 total rounds, 1 fix round)
  valid transitions: mark-step-complete
  next: merge and mark step complete
    action:
      1. Fast-forward .claude/worktrees/state-queries (state-queries-impl) into main.
      2. Run post-merge tests; if they fail, undo the merge, route to an isolated implementer, re-review.
      3. Edit docs/plans/agent-scaffold.plan.toml: [[step]] slug=state-queries, status="complete".
      4. Run `agent-scaffold render docs/plans/agent-scaffold.plan.toml`.
      5. Commit plan + ledger together.
      6. Commit-then-delete findings files:
           docs/plans/agent-scaffold.reviews/state-queries-reviewer-*.md
           docs/plans/agent-scaffold.reviews/state-queries-triage.md
           docs/plans/agent-scaffold.reviews/state-queries-triage-recheck.md
      7. Remove worktree: git worktree remove .claude/worktrees/state-queries.
    next_record_command: "(none; call `agent-scaffold next` to continue)"
```

**Human's experience:** the orchestrator reports at the step boundary "state-queries complete; 3 review rounds, 1 fix; moving to next step." The human does not decide anything here.

**Gap 2 (`record-step-start` event type):** the driver needs to know when a step transitions from not-started to in-progress in a worktree, but the plan TOML on main still says not-started while the implementer works. Without a `record-step-start` event, the driver would re-emit "spawn implementer" on every `next` call while the implementer is already running. This event type is absent from the round-1 I/O contract.

## 5. Scenario 4: Parallel work

Two steps are ready: `state-queries` and `test-driven` (both not-started, `blocked_by = []`). Orchestrator calls `next`.

Driver computes the ready frontier (both steps meet the topological-readiness predicate). Output:

```json
{
  "task_phase": "implementing",
  "ready_units": [
    {
      "step": "state-queries",
      "status": "not-started",
      "blocked_by": [],
      "isolation_tier": "worktree",
      "next_instruction": {
        "role": "implementer",
        "worktree": ".claude/worktrees/state-queries",
        "filled_prompt_summary": "Spawn implementer for state-queries in worktree .claude/worktrees/state-queries.",
        "next_record_command": "agent-scaffold record-step-start --task agent-scaffold --step state-queries --risk-class <low_risk|risky> ..."
      }
    },
    {
      "step": "test-driven",
      "status": "not-started",
      "blocked_by": [],
      "isolation_tier": "worktree",
      "next_instruction": {
        "role": "implementer",
        "worktree": ".claude/worktrees/test-driven",
        "filled_prompt_summary": "Spawn implementer for test-driven in worktree .claude/worktrees/test-driven.",
        "next_record_command": "agent-scaffold record-step-start --task agent-scaffold --step test-driven --risk-class <low_risk|risky> ..."
      }
    }
  ],
  "note": "2 units are ready. The orchestrator decides the fan-out width; both may run in parallel if isolation resources allow. Each unit is tracked independently by the driver."
}
```

**Orchestrator:** creates both worktrees in parallel, spawns two implementers. The driver does not control how many to fan out; it reports the full ready frontier. After both implementers start, the orchestrator calls `record-step-start` for each:

```
agent-scaffold record-step-start --task agent-scaffold --step state-queries --risk-class low_risk ...
agent-scaffold record-step-start --task agent-scaffold --step test-driven  --risk-class low_risk ...
```

When the orchestrator next calls `next`, the driver sees both steps in-progress (from the two step-start records) and outputs:

```
IN REVIEW (2 units):
  state-queries   work_review, low_risk, streak 0/1
  next: spawn reviewers on state-queries-impl
    findings_file_a: docs/plans/agent-scaffold.reviews/state-queries-reviewer-claude-sonnet-1.md
    ...

  test-driven   work_review, low_risk, streak 0/1
  next: spawn reviewers on test-driven-impl
    findings_file_a: docs/plans/agent-scaffold.reviews/test-driven-reviewer-claude-sonnet-1.md
    ...

  [both review loops are independent; a round in one does not affect the other]
```

The orchestrator fans out two pairs of reviewers and triagers, tracking each via separate `record-round` calls with their respective `--step` flags.

**Gap 3 (ledger schema for parallel units):** the ledger RESUME STATE was designed for one active step (it carries a single diff_base, single artifact, single streak). With two parallel units, the orchestrator must track two in-flight contexts. Either the ledger schema is extended to a list of in-flight unit entries (with one diff_base per unit), or the driver relies entirely on the JSONL log (via `record-step-start` events for each unit) and the `--ledger-fragment` flag provides only the per-unit diff-range information. The cleanest option: move diff_base and artifact into the JSONL log via `record-step-start`, and drop the dependency on the ledger for this information. The ledger then becomes the RESUME STATE narrative for the orchestrator, not a machine-readable state input. This is a design decision deferred to the architecture pass.

## 6. Scenario 5: Human interrupt mid-flow, and escalation at the total-round cap

**Part A: human interrupt.**

Mid-implementation, the human sends: "Can we also add a `--dry-run` flag to the `validate` command?"

**Orchestrator:** runs intake assessment (itself, bounded). Determines: non-trivial (touches Success Criteria, requires a new Roadmap step). Records:

```
agent-scaffold record-intake \
  --task agent-scaffold \
  --classification non_trivial \
  --replanned false \
  --metrics docs/metrics/workflow.jsonl
```

Orchestrator presents to the human per the human-input contract: options, trade-offs, recommendation ("route to planner to revise the plan"), reasoning. Human confirms. Orchestrator spawns a planner to revise the plan. The plan re-enters plan review (a fresh convergence loop on the revised plan). The driver, on the next call, sees the phase record shows plan_review is open again and outputs "spawn reviewers on the revised plan."

**Human's experience:** one decision point. The driver provides the intake record for accurate classification. The orchestrator's presentation is the human-input contract, not a raw tool output.

**Part B: escalation at the total-round cap.**

`state-queries` has accumulated 4 rounds (3 new_valid, 1 clean with streak reset). Round 5 produces new_valid findings. The orchestrator calls:

```
agent-scaffold record-round \
  --task agent-scaffold --step state-queries --phase work_review \
  --outcome new_valid --risk-class risky --valid-findings 1 --severities high \
  --top-dismissed-severity none --metrics docs/metrics/workflow.jsonl
```

Driver: total rounds = 5 >= cap = 5, and the round was new_valid (convergence check runs first only if the round is clean, per AGENTS.md). Output:

```
ESCALATED (state-queries):
  state-queries   work_review, risky, round 5/5 (cap reached), last round: new_valid
  valid transitions: record-escalation-decision(resume | accept | send_back)
  next: escalate to human per the human-input contract
    [pre-filled escalation brief for the orchestrator to present:]

    DECISION NEEDED: state-queries review loop has reached the round cap (5 rounds).
    Current state: risky artifact; last round found 1 high-severity finding.
    Ledger: docs/plans/agent-scaffold.ledger.md (see rounds 1-5 narrative for the full history).

    Options:
      A (resume): reset counters and spawn a fresh reviewer pass.
           Use when the findings are genuine and the loop is making real progress.
      B (accept-as-is): mark step complete with an accepted-at-escalation waiver.
           Use when the residual risk is acceptable after deliberation.
           A waiver record citing this escalation is required (W5 enforced).
      C (send-back): route a specific fix to the implementer, then restart the review loop.
           Use when one clear, bounded defect is driving repeated new_valid rounds.

    Recommendation: C (send-back). The same high-severity finding has recurred; a targeted fix is
    more likely to converge than another fresh reviewer pass on an unchanged artifact.
    Principle references: P3 (ground decisions in evidence; 5 rounds of data available), P4 (small steps).

    next_record_command: "agent-scaffold record-escalation-decision --task agent-scaffold --step state-queries --decision <resume|accept|send_back> --artifact src/main.rs --human-decision <resume|decision>"
```

**Human:** reviews the ledger narrative (rounds 1-5), picks option C.

**Orchestrator records:**

```
agent-scaffold record-escalation-decision \
  --task agent-scaffold \
  --step state-queries \
  --decision send_back \
  --artifact "src/main.rs" \
  --human-decision decision \
  --metrics docs/metrics/workflow.jsonl
```

Driver: `human_decision(send_back)` -> retire this review loop, route to implementer for targeted fix. Output: "spawn implementer to address the high-severity finding; after fix, open a new work_review loop (counters reset to 0)."

**Gap 4 (loop-reset event):** after a send_back escalation, the review loop restarts with fresh counters. The driver needs a signal to distinguish "5 rounds on version A, then 2 rounds on fixed version B" from "7 rounds on the same artifact" (the latter would trigger a second escalation at round 5 of the combined count). A `record-loop-reset` event (or a second `record-step-start` on the same step) is needed as a counter-reset marker. Without it, `validate --workflow` will miscount total rounds and may trigger a false W3 failure.

## 7. Scenario 6: Resume after compaction

The orchestrator session ended mid-review. Before the session ended, the orchestrator flushed the ledger with the RESUME STATE:

```
- task: agent-scaffold
- phase: implementing
- active step: state-queries
- risk_class: risky
- consecutive_clean: 1 (of 2 required)
- total_rounds: 2
- diff_base: abc123
- current_artifact: src/main.rs
```

A new orchestrator agent starts. The human pastes `resume.md`. The orchestrator calls:

```
agent-scaffold next \
  --source docs/plans/agent-scaffold.plan.toml \
  --metrics docs/metrics/workflow.jsonl \
  --ledger-fragment docs/plans/agent-scaffold.ledger.md \
  --isolation-tier worktree \
  --json
```

The driver recomputes entirely from the durable files:

- From the JSONL log: state-queries had 2 rounds (round 1: new_valid, round 2: clean after upheld recheck) -> consecutive_clean=1.
- From the `record-step-start` event: state-queries is in-progress, risk_class=risky.
- From the ledger fragment: diff_base=abc123, artifact=src/main.rs.
- From the plan TOML: state-queries status=not-started (still; the implementer's work is in the worktree, not yet merged).

Driver reconciles these signals (the step-start record takes priority over the TOML status for in-flight units) and outputs:

```
IN REVIEW (state-queries):
  state-queries   work_review, risky, streak 1/2 (need 1 more clean round)
  valid transitions: record-round-clean, record-round-new-valid
  isolation_tier: worktree
  next: spawn a FRESH reviewer (diversify model from prior rounds)
    context:
      artifact = src/main.rs
      diff_range = abc123..HEAD
      ledger = docs/plans/agent-scaffold.ledger.md
      findings_file = docs/plans/agent-scaffold.reviews/state-queries-reviewer-<model>-3.md
      triage_file = docs/plans/agent-scaffold.reviews/state-queries-triage-3.md
    note: round 2 was clean (recheck upheld); this is round 3; risky artifact requires streak 2, currently 1.
    principle_reminders: ["P5 (fresh reviewer, independent of prior rounds)", "Workflow: do not re-raise settled findings without new evidence"]
    next_record_command: "agent-scaffold record-round --task agent-scaffold --step state-queries --phase work_review --risk-class risky --outcome <clean|new_valid> ..."
```

The new orchestrator agent resumes immediately. It does not need to re-read 5 round narratives; the driver has summarized the state. This is the compaction-survival property: stateless recompute from committed files reconstructs exactly where the workflow was.

**Human's experience:** the agent resumes without asking the human to re-explain anything. The preflight is brief; the driver carries the rest.

## 8. Scenario 7: Human override (exception the machine did not anticipate)

During state-queries review, round 3, a reviewer finding cites a file that does not exist in the diff. The orchestrator and the human determine this is a reviewer error; the artifact is actually clean. But the standard `clean` path requires a triager round, and the standard `new_valid` path would route a non-existent finding back to the implementer. Neither fits.

The orchestrator uses the override mechanism, which is a FIRST-CLASS, RECORDED transition, not a bypass:

```
agent-scaffold record-override \
  --task agent-scaffold \
  --step state-queries \
  --to accepted \
  --reason "Round 3 finding cites a file absent from the diff; confirmed reviewer error after human review of the raw diff. Human accepted the artifact after direct inspection. See ledger entry 2026-07-19." \
  --metrics docs/metrics/workflow.jsonl
```

Driver validates: `accepted` is a defined terminal state in the step machine, reachable from `in-review` via override. The transition is total (no hole in the function). Command appends the override record and outputs:

```
OVERRIDE RECORDED (state-queries):
  state-queries   -> accepted (human override, triggered at round 3)
  override logged with reason.
  next: mark step complete
    action:
      1. Mark [[step]] slug=state-queries status="complete" in plan TOML.
      2. Run `agent-scaffold render`.
      3. Commit plan + ledger (include override justification in commit message).
      4. Commit-then-delete findings files.
      5. Remove worktree.
    note: `agent-scaffold validate --workflow` will see this override record and must treat it as
          valid evidence for the step's completion (analogous to how W3 accepts a waiver record).
```

The override is in the log. Every future `validate --workflow` run sees it. Every `agent-scaffold next` call sees it. The machine remains the single source of truth for what happened, even when a human forced a move the normal rules would not take.

**Gap 5 (W5 override handling):** `validate --workflow` W5 does not currently handle override records. An extension is needed: if a step is `complete` and its work_review loop did not reach the required streak, but an `override` record names that step with a reason, W5 should accept the step as legitimately closed rather than flagging it as convergence-failure. This parallels how W3 accepts a `waiver` record. The override record is evidence-backed (it names the reason and the human decision) and is analogous to `accepted-at-escalation`.

## 9. Scenario 8: Acceptance and task close

All Roadmap steps are `complete`. Orchestrator calls `next`.

Driver reads: plan TOML shows all steps complete; no acceptance round record exists. Output:

```
ACCEPTANCE READY:
  All 8 Roadmap steps complete.
  next: spawn acceptance reviewers
    prompt: .agents/prompts/reviewer.md
    context:
      criteria: docs/plans/agent-scaffold.plan.toml [[success_criteria]] and sidecars
      artifact: full codebase at HEAD (no diff range; review against criteria, not a diff)
      ledger: docs/plans/agent-scaffold.ledger.md
      findings_file_a: docs/plans/agent-scaffold.reviews/acceptance-reviewer-claude-sonnet-1.md
      findings_file_b: docs/plans/agent-scaffold.reviews/acceptance-reviewer-claude-opus-1.md
      triage_file: docs/plans/agent-scaffold.reviews/acceptance-triage.md
    note: acceptance is a SINGLE reviewers-then-triager pass, not a convergence loop;
          backstop applies (any dismissed high or critical finding requires a recheck);
          if a shortfall is found, route back to planner or implementer, not to a new acceptance round.
    principle_reminders: ["P6 (verify: run it and test it)"]
    next_record_command: "agent-scaffold record-round --task agent-scaffold --step __acceptance__ --phase acceptance --outcome <met|shortfall> ..."
```

Reviewers confirm all criteria met. Triager confirms. No high or critical dismissals. Orchestrator records:

```
agent-scaffold record-round \
  --task agent-scaffold \
  --step __acceptance__ \
  --phase acceptance \
  --outcome met \
  --risk-class low_risk \
  --valid-findings 0 \
  --metrics docs/metrics/workflow.jsonl
```

Driver output:

```
TASK COMPLETE:
  Acceptance: all criteria met.
  valid transitions: close-task
  next: close the task
    action:
      1. Run `agent-scaffold validate --workflow --source ... --metrics ...` (exit 0 is the pass).
      2. Commit ledger with final state.
      3. Delete ledger as committed deletion.
      4. Commit-then-delete all remaining findings files under docs/plans/agent-scaffold.reviews/.
      5. Keep the metrics log; it accumulates across tasks and is never reset.
      6. Push if a remote is configured.
```

Orchestrator runs `validate --workflow`. Exits 0. Commits. Done.

**Human's experience:** the orchestrator reports "task complete; validation clean." The human reviews the final commit and signs off. The driver's last output is also a clean closure checklist, not a summary the orchestrator must compose from scratch.

## 10. Friction and gaps found

The scenarios above reveal the following missing requirements and awkward seams in the round-1 design.

**Gap 1 (bootstrap signal).** The driver cannot distinguish "plan template not yet filled" from "planner has run, plan review may start" using the TOML alone. A `type: "phase"` event record (or a `planning_complete` flag in the plan TOML) is required. The TOML flag is simpler (no new event type); the JSONL event is consistent with the rest of the log. Either closes the gap; the architecture pass should decide.

**Gap 2 (`record-step-start` event type).** The driver needs to know when a step moves to in-progress in a worktree, before the plan TOML is updated on main. Without `record-step-start`, the driver re-emits "spawn implementer" on every `next` call while the implementer is already running. This event must carry the step slug, risk_class, diff_base commit, and worktree branch so the driver can reconstruct the in-flight state without the ledger.

**Gap 3 (parallel in-flight units and the ledger schema).** The ledger RESUME STATE carries one diff_base and one artifact. With parallel units, the orchestrator must track per-unit diff_base values. The cleanest resolution: move diff_base and artifact into `record-step-start` events in the JSONL log, making the ledger purely a human-readable narrative rather than a machine-readable state input. The `--ledger-fragment` flag would then be optional (for human context) rather than required for state reconstruction. This is a deliberate design decision: the JSONL log becomes the single machine-readable state source; the ledger is for the orchestrator and the human to read.

**Gap 4 (loop-reset event after send_back escalation).** After a send_back escalation decision, the review loop restarts with fresh counters. Without a reset marker in the JSONL log, `validate --workflow` cannot distinguish "5 rounds on version A, then 2 rounds on fixed version B" from "7 rounds on the same artifact." A `record-loop-reset` event (or a second `record-step-start` on the same step with a `loop: 2` field) is needed.

**Gap 5 (override records and W5).** `record-override` is proposed as a first-class mechanism for human exceptions that do not fit the standard transition alphabet. `validate --workflow` W5 must be extended to accept an override record as valid completion evidence for a step that ended without meeting the convergence streak. This parallels the existing waiver mechanism: the override record is the "accepted-at-escalation" for situations that don't go through the escalation path.

**Gap 6 (`next_record_command` is load-bearing).** Every scenario shows the driver including a `next_record_command` field in its JSON output: the exact `record-*` command the orchestrator should run after executing the instruction, with all flags pre-filled except the judgment placeholders (`<clean|new_valid>`, `<upheld|overturned>`, etc.). This field is critical for usability: it eliminates the orchestrator's need to remember which `record-*` command maps to which state, and which flags apply. Without it, the orchestrator must assemble the command from memory, which is a primary drift surface. This must be a first-class output field, not an optional convenience.

**Gap 7 (findings file paths pre-computed by the driver).** The driver should pre-compute the full set of expected findings file paths for the current round (reviewers plus triager plus optional recheck triager), using the `<step>-<role>-<disambiguator>.md` convention from AGENTS.md, and include them in the `next` output before the round starts. This removes the transcription step where the orchestrator invents these paths and risks a naming collision. Reviewers are given the correct path directly from the driver's output.

**Gap 8 (risk class conservatism).** The `risk_class` judgment must be supplied with `record-step-start` and cannot be inferred by the driver from the code. If the orchestrator omits it, the driver should default to `risky` (the conservative choice) and emit a warning, rather than silently defaulting to `low_risk`. A silent `low_risk` default would let a risky artifact converge with one clean round when two are required, which defeats the backstop.

## 11. Recommendation: the interaction model

The scenarios converge on a **semi-authoritative event-recording model**. Here are the three options with trade-offs and the recommendation.

**Option A: read-only advisory (`next` only, no `record-*`).**

The driver suggests; the orchestrator hand-writes JSONL; `validate --workflow` catches drift post-hoc.

Trade-offs against Project Principles: P13 (illegal states) is only approximated (the tool detects illegal states after the fact, not at write time). P4 (small steps) is served (this is essentially the advisory MVP). P6 (evidence-first) is served. The gap: hand-writing JSONL is error-prone, and the driver's point-of-action suggestion is advisory only; an orchestrator under token pressure may skip it.

**Option B: semi-authoritative event-recording (`next` + `record-*`, non-blocking, recommended).**

`agent-scaffold next` emits the next instruction and the pre-filled `record-*` command. `agent-scaffold record-*` validates the transition before appending; it exits non-zero and prints the current state if the transition is illegal. The orchestrator is NOT blocked from proceeding without calling `record-*`; the tool holds no lock and imposes no runtime dependency on the binary for the workflow to execute. An orchestrator that skips `record-*` accumulates a gap in the JSONL log, which `validate --workflow` flags at next run.

Trade-offs against Project Principles: P13 (illegal states refused at write time in real time, not just detected post-hoc); P5 (the tool cannot represent a state where an illegal transition was accepted); P18 (least authority: the tool validates but does not own the worktree or the agent spawn); P3 (compatible with evidence-first: the advisory path degrades gracefully if the orchestrator ignores the tool). Cost: reopens the Q-24 "no write-path runtime dependency" tradeoff, which was deliberately adopted. The resolution: the Q-24 constraint was set before the driver concept existed; the driver changes the cost-benefit by providing structural enforcement (P13) that hand-writing cannot. The advisory phase (the staged build path) provides the evidence gate.

**Option C: fully blocking / harness-enforced authoritative driving.**

The harness enforces that `record-*` must succeed before the next agent spawns. Requires hook infrastructure outside the driver itself.

Trade-offs: maximally enforces P13 but requires a harness-level integration (a blocking hook, or a wrapper that refuses to spawn agents without a successful `record-*` call). This is orthogonal to the driver code and can be layered on top of Option B later. Building it in the first increment of the full driver is premature.

**Recommendation: Option B.** It captures the full driver's structural enforcement value (illegal transitions refused in real time; P13), keeps the tool's authority bounded (P18), survives graceful degradation (if the orchestrator skips `record-*`, the log has a gap and `validate` flags it, but the workflow does not deadlock), and does not require harness hook infrastructure in the first increment. The `next_record_command` field in the driver's output minimizes the orchestrator's chance of skipping or mis-assembling the `record-*` call. Full blocking (Option C) can be layered on top of Option B once consistent adherence is measured.

## 12. YAGNI boundary

Do NOT build in the first increment of the full driver:

- Fully blocking / harness-enforced authoritative driving. The harness hook layer is orthogonal to the driver code and can be added after Option B shows consistent adoption.
- AGENTS.md generation from the machine spec. The round-1 tool-vs-prose lens identified this as a separate, hard subsystem. The full driver can co-exist with a hand-maintained AGENTS.md while the machine spec stabilizes.
- Increment-level DAG scheduling. The FSM lens confirmed the scheduler operates at step granularity; increment ordering within a step stays the orchestrator's sequential choice until evidence shows finer scheduling is needed.
- An interactive REPL / `drive` command. The `next` + `record-*` pair achieves the same result without a persistent process, and the persistent-process approach breaks stateless-recompute (Principle 9).
- Per-turn `next` polling during active work. Q-28 decided this is a negative token trade during active work. `next` is called at decision points (start of a step, after a round completes, at escalation, on resume), not continuously.
- LLM-generated instruction text. The `filled_prompt_summary` and `next_record_command` fields are parameterized assembly from fixed templates. Do not attempt to synthesize these with an LLM; the templates are the right substrate.
- `record-phase` as a separate event type IF the plan TOML `planning_complete` flag resolves Gap 1 more simply. Prefer the simpler mechanism; the architecture pass should decide which.
