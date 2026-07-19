# Design notes: Q-51 driver I/O contract, harness integration, and advisory adoption path

Explorer lens: the concrete I/O contract, harness and isolation integration, and the adoption path for the Mealy-workflow-driver (Q-51).

References read: Q-51 ask, AGENTS.md (workflow section, roles, writer-isolation tiers, checkpoints, convergence rule, instrumentation section), `src/main.rs` (the `validate`, `status`, and `render` subcommand surfaces and their arguments), `src/metrics.rs` (all record types and their fields: `round`, `escalation`, `dismissal_recheck`, `intake`, `decision`, `baseline`, `waiver`), `docs/plans/agent-scaffold.plan.toml` (Project Principles P1-P8 from the TOML source), Q-24 (the "no runtime write dependency" decision), Q-28 (the `next` + `status --resume` scope decision). Project Principle numbers below are from the TOML source (`[[principle]]` entries), distinct from the generic AGENTS.md principles (cited as "Workflow Principle N").

## 1. The question

Q-51 proposes turning `agent-scaffold` into a deterministic state-machine DRIVER: the orchestrator runs the tool at each decision point, the tool parses current durable state (plan + ledger + metrics), and prints the next filled instruction prompt for the agent. This lens works out: the concrete subcommand surface and I/O; the stateless-recompute vs. hidden-state argument; how the tool stays out of worktree/container management while still emitting isolation guidance; and a concrete evidence-first adoption path.

This lens is one of four planned passes on Q-51. The other three cover the formal FSM and concurrency model, the tool-vs-prose single-source boundary and AGENTS.md generation, and the YAGNI/skeptic weighting of the driver against lighter drift mitigations. Those passes and this one are complementary; recommendations here are bounded by what falls strictly under this lens.

## 2. Design space

### 2a. Subcommand surface: three viable options

**Option A: read-only `next` subcommand (advisory, no write path)**

The tool only reads durable files and prints its assessment. The agent hand-writes every JSONL event record exactly as today.

Signature sketch:

```
agent-scaffold next
  --source docs/plans/<task>.plan.toml
  --metrics docs/metrics/workflow.jsonl
  [--ledger-fragment <path-to-ledger-file>]
  [--json]
  [--unit <step-slug>]
```

The `--ledger-fragment` is the current task ledger (`docs/plans/<task>.ledger.md`), read verbatim for the `## RESUME STATE` block that carries the transient in-flight state (current step, artifact under review, round counts) not yet derivable from the plan + log alone. Reading it keeps the tool stateless without losing the in-flight context. The tool does not write to the ledger; the orchestrator still owns it.

Trade-offs: compatible with Q-24 (no write-path runtime dependency on the binary, agents can still hand-write JSONL and the workflow degrades gracefully without the binary); simple to adopt (add one `agent-scaffold next` call in the orchestrator's decision loop); but advisory only, the agent is free to deviate.

**Option B: event-recording `record-round` (and sibling) subcommands**

Add write-path subcommands that atomically APPEND a JSONL record to the metrics log AND print the new state, enforcing transition validity before writing:

```
agent-scaffold record-round
  --source docs/plans/<task>.plan.toml
  --metrics docs/metrics/workflow.jsonl
  --step <slug>
  [--increment <id>]
  --outcome <clean|new_valid>
  --risk-class <low_risk|risky>
  --artifact <description>
  --phase <plan_review|work_review|acceptance|review>
  [--valid-findings <n>]
  [--severities <s1,s2,...>]
  [--consecutive-clean <n>]
  --task <task-name>
```

The command validates the transition is legal from the current state before appending. If the step is not in-review state, or the required consecutive-clean count is already reached, it exits non-zero and prints the actual current state.

Trade-offs: Workflow Principle 13 (illegal transitions unrepresentable, on-disk schema enforced at write time) and P8 (structured data first). But it adds a runtime write dependency on the binary, the exact coupling Q-24 declined to avoid; the fallback (hand-write JSONL) is still possible but the tool becomes load-bearing during active workflow runs. This is acceptable only once the advisory phase shows agents actually run the tool consistently.

**Option C: full REPL / `drive` command (interactive state machine)**

An interactive loop that accepts event tokens ("round-clean", "triage-done", "step-merged") from stdin and transitions state, maintaining in-memory FSM state across calls.

Trade-offs: requires a persistent process (defeats the stateless-recompute argument); state lives in the process, not in the durable files; breaks if the session is interrupted. This violates the "durable state is the only truth" tenet the orchestrator's resume procedure depends on. Not recommended.

### 2b. Output structure (both machine JSON and human text)

Both `next` (Option A) and event-recording commands (Option B) share the same output structure, since the event-recording commands also print the new state after a transition.

**Machine JSON (`--json`):**

```json
{
  "task": "agent-scaffold",
  "source": "docs/plans/agent-scaffold.plan.toml",
  "metrics": "docs/metrics/workflow.jsonl",
  "ready_units": [
    {
      "step": "state-queries",
      "increment": null,
      "phase": "not-started",
      "risk_class": null,
      "consecutive_clean": 0,
      "required_streak": null,
      "valid_transitions": ["start-plan"],
      "isolation_tier": "worktree",
      "next_instruction": {
        "role": "planner",
        "prompt_path": ".agents/prompts/planner.md",
        "context": {
          "step_slug": "state-queries",
          "step_title": "`next` and a `status --resume` slice (Q-28)",
          "plan_source": "docs/plans/agent-scaffold.plan.toml",
          "ledger": "docs/plans/agent-scaffold.ledger.md",
          "blocked_by": []
        },
        "principle_reminders": [
          "P1 (cleaner long-term architecture over smallest diff)",
          "P6 (evidence-first: validate with a proof-of-concept before building out)"
        ],
        "filled_prompt_summary": "Spawn a planner in a worktree. Hand it: the plan TOML at docs/plans/agent-scaffold.plan.toml, the ledger at docs/plans/agent-scaffold.ledger.md, the step slug state-queries and its detail. Ask it to draft the increment plan per .agents/prompts/planner.md."
      }
    }
  ],
  "blocked_units": [
    {
      "step": "test-driven",
      "waiting_on": []
    }
  ],
  "in_review_units": [],
  "escalated_units": []
}
```

**Human text (no `--json`):**

```
task: agent-scaffold
source: docs/plans/agent-scaffold.plan.toml

READY (1 unit):
  state-queries   not-started -> start-plan
  isolation tier: worktree
  next: spawn planner
    prompt: .agents/prompts/planner.md
    context: step=state-queries, plan=docs/plans/agent-scaffold.plan.toml,
             ledger=docs/plans/agent-scaffold.ledger.md
    reminders: P1 (cleaner long-term arch), P6 (evidence-first)

BLOCKED (1 unit):
  test-driven   waiting_on: []  (not-started, no blockers resolved)

[no units in review, no escalations]
```

The `next_instruction` content is the load-bearing part: it fills in the role prompt path, the step slug, the artifact path (when known from the ledger), the diff range (e.g. `commit1..HEAD` from the ledger's current round section), the ledger path, and the relevant principle reminders. The tool does not generate new prose; it fills in the slots that today the orchestrator fills by reading the plan and ledger and constructing the brief manually.

### 2c. Three concrete worked examples

**Example 1: "risky step, streak 1, last round was clean"**

State read from metrics log: step `structured-skeleton`, one increment `structured-skeleton-inc3`, risk_class = `risky`, consecutive_clean = 1, required = 2.

Tool output:

```
IN REVIEW (1 unit):
  structured-skeleton / inc3   work_review, risky, streak 1/2 (need 1 more clean round)
  valid transitions: record-round-clean, record-round-new-valid, escalate
  isolation tier: worktree
  next: spawn a FRESH reviewer (diversify model/harness from the prior round)
    prompt: .agents/prompts/reviewer.md
    context: step=structured-skeleton-inc3,
             artifact=src/plan.rs (from ledger),
             diff=abc123..def456 (from ledger round 1 section),
             ledger=docs/plans/agent-scaffold.ledger.md
    note: risky artifact requires 2 consecutive clean rounds; streak is 1, so
          this is round 2 of at least 2; if new valid findings appear the streak
          resets to 0.
    reminders: Workflow P5 (independent adversarial review), P4 (small reviewable changes)
```

**Example 2: "low_risk step, streak 1, last round was clean"**

State: step `instrument-flag`, risk_class = `low_risk`, consecutive_clean = 1, required = 1. Streak reached.

Tool output:

```
CONVERGED (1 unit):
  instrument-flag   work_review, low_risk, streak 1/1 - CONVERGED
  valid transitions: mark-step-complete
  next: mark step complete
    action: edit docs/plans/agent-scaffold.plan.toml -> [[step]] slug=instrument-flag
            set status = "complete"; re-run `agent-scaffold render`; commit plan + ledger.
    reminders: Workflow P9 (leave durable notes), P8 (no silent scope expansion)
```

**Example 3: "step not-started, all blockers met, two such steps available"**

State: steps `state-queries` and `test-driven` both have status `not-started` and `blocked_by = []`.

Tool output:

```
READY (2 units, may run in parallel):

  state-queries   not-started -> start-plan
  isolation tier: worktree
  next: spawn planner in worktree at .claude/worktrees/state-queries-plan
    prompt: .agents/prompts/planner.md
    context: step=state-queries, plan=docs/plans/agent-scaffold.plan.toml

  test-driven   not-started -> start-plan
  isolation tier: worktree
  next: spawn planner in worktree at .claude/worktrees/test-driven-plan
    prompt: .agents/prompts/planner.md
    context: step=test-driven, plan=docs/plans/agent-scaffold.plan.toml

  [the orchestrator decides whether to run these in parallel or sequence;
   the tool reports both as ready and emits separate instruction prompts for each]
```

### 2d. Stateless vs. stateful: the case for stateless recompute

The tool MUST recompute all state from the durable files on every call. The reasons are:

The durable files are already the canonical state. The TOML plan owns step status (`not-started` / `in-progress` / `complete` / etc.), the JSONL metrics log owns the round records and their streaks, and the ledger owns the in-flight transient (artifact path, diff range, current round narrative). These are already committed and surviving across compaction per Workflow Principles 9 and 16. A mutable state file would be a second source of truth that could drift from the committed state, breaking the resume procedure.

Crash safety. If the orchestrator or the binary dies mid-step, the next call to `agent-scaffold next` recomputes from the files as they are at that moment and picks up cleanly. A stateful process would leave a stale in-memory FSM and require restart logic.

Compatibility with existing validate. `validate --workflow` already recomputes the workflow state from the plan TOML + metrics JSONL to check invariants. The `next` command is the same computation extended with instruction generation; sharing the same parse paths (`src/plan.rs`, `src/metrics.rs`) means the tool's view of "what is the current state" is exactly the same as the validator's view. They cannot drift from each other.

Survives parallel fan-out. When two steps are in review simultaneously (the orchestrator spawns two parallel reviewer agents), the metrics log accumulates round records for both. On the next call to `next`, the tool reads all records and reports the correct state for each unit independently. No inter-call memory is needed.

The only state not in the committed files is the in-flight ledger content (the `## RESUME STATE` block and the current-round narrative). Passing the ledger path as `--ledger-fragment` covers this without requiring the tool to own any mutable state between calls.

### 2e. Isolation and harness integration

The tool reads only committed files on main (or from the paths given on the command line). It never creates worktrees, never spawns containers, and never makes agent calls. The workflow role separation stays intact: the tool is read-only with respect to the code and the running workflow.

What the tool CAN emit is the resolved isolation tier as part of the instruction. Given the AGENTS.md isolation tier policy (container > worktree > file-safety fallback), the tool includes in each unit's `next_instruction` a field `isolation_tier` that names the tier the orchestrator should use. How the tool resolves the tier depends on what information it has: if the orchestrator passes a flag `--isolation-tier worktree` (or the harness passes it via an env var), the tool echoes it into every unit's instruction. If no tier is passed, the tool emits `"isolation_tier": "unknown"` and includes a reminder to resolve it per the AGENTS.md tier policy. This keeps the tool's output useful across harnesses without coupling it to any specific harness's container API.

For parallel fan-out: the tool emits an array of `ready_units` with independent instruction prompts, one per ready unit. The orchestrator reads this array and decides which to fan out in parallel (and with what isolation). The tool's job is to say "these N units are ready and here is what each one needs"; the harness's job is to decide the fan-out width and spawn accordingly. The boundary is clean.

The tool does not emit worktree paths or branch names; the orchestrator computes those from the step slug per the AGENTS.md naming convention. The tool emits the role prompt path (`.agents/prompts/reviewer.md`), the context fields, and the principle reminders; the orchestrator assembles the final agent prompt and spawns.

### 2f. Event-recording subcommands and Principle 13

The Q-24 decision declined a "validated-append writer" for the JSONL to avoid a runtime write dependency on the binary. That decision applies to the core workflow orchestration path (an agent writing a round record after a triage). The event-recording subcommands proposed in Option B are in the same category: they would make the binary a required tool for the write path.

This is not automatically disqualifying. The Q-24 constraint was adopted before the driver concept existed; the driver changes the cost-benefit by providing structural enforcement of legal transitions (Workflow Principle 13) that hand-writing cannot provide. But the adoption path (see section 3) argues that the write-path integration should come after advisory adoption is measured, not before. At the advisory MVP stage, read-only `next` is sufficient and avoids reopening the Q-24 tradeoff prematurely.

The YAGNI boundary (section 4) makes this explicit: defer `record-round` and siblings until the advisory phase produces evidence that agents run `next` consistently and that write-path atomic enforcement is the remaining gap.

## 3. Trade-offs judged against Project Principles

**P1 (cleaner long-term architecture over smallest diff):** The advisory `next` subcommand is the correct architectural layer: it sits above `validate` (enforcement) and below "authoritative driving" (future). Building it first as read-only keeps the cost of the first step small while establishing the right abstraction. Event-recording subcommands are the right long-term extension, deferred (not discarded).

**P2 (minimal by default):** A read-only `next` subcommand adds one new subcommand and no new required files. It uses the existing `src/plan.rs` and `src/metrics.rs` parsers (already parsing exactly the state `next` needs) and the existing `--source`/`--metrics` argument shape. The YAGNI boundary keeps the scope here.

**P3 (safe on existing projects):** The tool is fully read-only in advisory mode. It never writes to the plan, ledger, or metrics log. An existing project that does not run `next` is completely unaffected.

**P4 (idempotent):** Calling `agent-scaffold next` twice in a row with no changes to the durable files produces identical output. State recompute guarantees this.

**P5 (make illegal states unrepresentable):** In advisory mode the tool cannot enforce this directly (the agent is free to deviate). The event-recording Option B enforces it at write time. The advisory phase measures whether enforcement is needed; if agents follow the tool's suggestions reliably, the advisory path may be sufficient and Option B may be YAGNI.

**P6 (evidence-first):** This is the strongest argument for the advisory-first adoption path. The tool is a significant investment; the Q-51 ask itself notes it is "larger than structured-skeleton". Running advisory `next` and measuring whether agent adherence improves (fewer ledger deviations, fewer skipped workflow steps, fewer round-count errors) is exactly the proof-of-concept the principle demands before building the authoritative-driving and event-recording machinery.

**P7 (reproducible):** Stateless recompute from committed files makes the tool's output fully reproducible: the same files always yield the same output, on any machine.

**P8 (structured data first):** The `--json` output and the structured `next_instruction` fields make the tool's assessment machine-readable, consistent with the project's direction of structured sources projected to human views.

## 4. Recommendation

Build the advisory MVP first: a read-only `next` subcommand that takes `--source`, `--metrics`, `--ledger-fragment`, and `--json`, recomputes per-unit state from those files, and prints (a) a summary of each unit's current state, (b) the valid transitions from that state, (c) a filled instruction prompt (role prompt path plus context fields), and (d) selected principle reminders. Emit both JSON and human text.

The subcommand should be built on top of the existing `src/plan.rs` (for step status, risk classification, blocked_by DAG) and `src/metrics.rs` (for round records, streak counts, convergence check) parsers, since those already contain the state-reading logic `next` needs. The `state-queries` Roadmap step (order 46, not-started) is the natural home for this first increment; the Q-51 driver extends it with the instruction-generation layer. Coordinate with the Q-28 decision to avoid duplicating the `status --resume` scope: `status --resume` is a slice-emit for token-efficient resume; `next` adds the instruction-generation and transition-validity logic on top.

The instruction-generation does not need to synthesize new prose. It fills in a fixed slot structure: role prompt path, step slug, artifact path (from the ledger fragment), diff range (from the ledger fragment), ledger path, and a short principle-reminder list keyed to the current phase. The "filled prompt" is a parameterized assembly, not LLM generation.

For isolation integration: add a `--isolation-tier <worktree|container|file-safety>` flag the orchestrator passes based on what the harness supports. The tool echoes this into each unit's instruction so the agent knows the target tier without the tool needing to understand the harness. Default to `unknown` with a reminder to resolve.

**What to measure in the advisory phase:**

Instruct agents to run `agent-scaffold next` at each orchestrator decision point and include the output in the ledger's current-round section. After a sustained period of use (ten or more complete step cycles), check:
- Whether the tool's predicted "next action" matches what the orchestrator actually did (adherence rate).
- Whether the rate of workflow deviations (skipped convergence rounds, missing round records, missing triager, etc.) decreased relative to pre-tool runs (drift reduction).
- Whether agents reliably pass `--ledger-fragment` and the tool correctly reads in-flight state (i.e., the ledger handoff works without friction).

If adherence is high and drift is reduced, the advisory mode is sufficient and further investment (event-recording subcommands, AGENTS.md generation) should be deferred. If adherence is low, investigate why: is the tool's output unclear, is the binary unavailable to agents, or do agents ignore it? Each failure mode has a different fix (output clarity, availability, authoritative enforcement).

**What NOT to do now (YAGNI boundary):**

Do not build event-recording (`record-round`, `record-decision`, etc.) subcommands in this increment. The Q-24 "no write-path runtime dependency" constraint was deliberately adopted, and reopening it requires evidence that the advisory mode's read-only path is insufficient. That evidence comes from the measurement above.

Do not generate `AGENTS.md` from the machine definition in this increment. The FSM logic is not yet stable or complete enough to be the canonical source of the workflow prose, and the render-closure work is a large independent initiative. The advisory `next` command can coexist with a hand-authored `AGENTS.md` without drift risk as long as the tool's transition logic is derived from the same `src/metrics.rs` and `src/workflow.rs` invariants that `validate --workflow` already enforces.

Do not build blocking/authoritative mode. The tool in advisory mode prints "you should do X"; in authoritative mode it would refuse to proceed unless the agent records the correct event. Authoritative mode requires reliable binary availability in all harness environments and a clear agent instruction to "never proceed without consulting the tool", which is a harness-configuration and onboarding change as well as a code change. Defer until the advisory phase demonstrates consistent adoption.

Do not add escape-hatch machinery in this increment. Escape hatches (human overrides for edge cases the FSM does not model) are necessary for the long-term authoritative system but premature here: the advisory tool's output is ignored by the agent if the agent disagrees, which is the escape hatch. Model explicit escape hatches once the tool is authoritative and an agent genuinely needs to override it.

Do not scope `next` to per-turn polling during active work. The Q-28 decision explicitly noted "the honest token win is at cold resume and in a hook, and is negative per-turn during active work, so do not poll it each turn." The primary use cases are cold resume, CI/pre-commit enforcement, and the orchestrator's decision-point checkpoints (start of step, end of review round, escalation).
