# Q-59 design notes: the command surface and enforcement for the session lifecycle

Lens: what commands and validation the lifecycle mechanism needs, and how transitions are made and validated. This is one of several parallel explorer lenses on Q-59; the sibling storage lens (where the state lives) is deferred to explicitly below where the two couple. This document does not pick the storage substrate; it recommends the command surface and the enforcement, and shows how each substrate would read through it.

## 1. The question

Q-59 asks whether the session lifecycle, the `pause` / `compaction-prep` / `resume` user-prompts, should be encoded as explicit, tool-validated states rather than the current prose-only checklists plus procedures (plan `docs/plans/agent-scaffold.plan.toml` line 1266). Today:

- `next`'s `LoopState` (`src/next.rs` lines 167 to 200) models only the review loop (ReadyToPlan, Blocked, AwaitingFirstReview, AwaitingFixes, AwaitingReviewers, Converged, Escalate, RiskClassConflict, Done). It has no Active / Checkpointed dimension.
- `next` is read-only and stateless by construction (`src/next.rs` lines 1 to 8): "It writes nothing and creates no worktree or container." `status` (`src/main.rs` lines 1009 to 1078) is the same, a best-effort projection.
- `validate --workflow` is backward-looking detection, not prevention (`src/workflow.rs` lines 1 to 6): "the scaffolded workflow still writes both directly, with no runtime dependency on this binary." Its `run_checks` funnel (`src/workflow.rs` lines 206 to 221) runs round-log consistency, W3 (step convergence or a covering waiver), W4 (decision receipts), and W5 (waiver integrity).
- `.agents/user-prompts/pause.md`, `compaction-prep.md`, and `resume.md` are prose that instruct the human to have the agent run the checkpoint or resume procedure from the "Checkpoint and resuming after context loss" section of `AGENTS.md` (lines 97 to 102). None of them, and no tool path, records "this session is checkpointed", validates that a resume follows a checkpoint, or blocks work while checkpointed.

The narrow lens here: WHAT commands write and validate the transition, and WHAT `validate --workflow` should enforce.

## 2. The design space

Two orthogonal axes matter for the command surface.

Axis A, the WRITE surface (how the checkpoint / resume markers get recorded):

- A1. A `checkpoint` / `resume` subcommand pair that the tool owns: `agent-scaffold checkpoint --kind pause|compaction` and `agent-scaffold resume` write the marker AND validate the transition at write time (refuse a second checkpoint, refuse a resume with no open checkpoint, refuse a dirty tree).
- A2. Validation-only: no new write command. The agent appends the lifecycle marker the same way it appends every other workflow event today (a `round`, a `decision`, a `waiver`), and the tool only READS and VALIDATES it.
- A3. A hybrid: no write command now, but the read and validate surfaces are built so a thin writer can be added later without reshaping them.

Axis B, the READ / ENFORCE surface (independent of who writes):

- B1. `next` and `status` gain a lifecycle DIMENSION (a `session_state` field, distinct from `LoopState`) and re-route their "do work" advice while checkpointed, instructing "run the resume procedure first".
- B2. `validate --workflow` gains a lifecycle-legality check (call it W6) over the durable log: a resume must follow a checkpoint, no two checkpoints without an intervening resume, plus whatever tree-cleanliness it can actually reconstruct.

Axis B is not really optional under any serious answer: some read surface and some backstop are needed no matter how the marker is written. The genuine decision is Axis A. So the three question-stem options (a) write-command, (b) validation-only, (c) hybrid map onto A1 / A2 / A3, each carrying the same Axis-B surface (B1 plus B2).

A cross-cutting fact that constrains Axis A: the tool is today a pure detector with no runtime write path into workflow state. `scaffold` and `render` write, but they are setup and projection, not event recording; every workflow EVENT (rounds, decisions, waivers, escalations, baselines) is hand-appended by the agent and only checked by the tool. A `checkpoint` / `resume` write command would be the first time the binary records a workflow event at runtime, which is an architectural departure, not a local addition.

## 3. Options and trade-offs, judged against the plan's Project Principles

The plan's eight Project Principles, by name (`docs/plans/agent-scaffold.plan.toml` lines 1273 to 1311): Prefer the cleaner long-term architecture over the smallest diff (P1); Minimal by default (P2); Safe on existing projects (P3); Idempotent (P4); Make illegal states unrepresentable (P5); Ground decisions in evidence (P6); Reproducible (P7); Structured data first, project for humans (P8).

### Option A1: a `checkpoint` / `resume` write-command pair

The tool owns the transition. `agent-scaffold checkpoint --kind pause|compaction` checks the tree is clean and the session is not already checkpointed, then appends a well-formed marker; `agent-scaffold resume` checks an open checkpoint exists, then appends a resume marker. The user-prompts become thin wrappers: "run `agent-scaffold checkpoint --kind pause`, then confirm the tree is clean."

- Make illegal states unrepresentable (P5): strongest here at the transition boundary. `checkpoint` refuses to write a second checkpoint and `resume` refuses to write without an open one, so the illegal transition is never recorded rather than caught afterwards. This is the parse-don't-validate posture applied to the write. The catch: the tool is NOT the sole writer of the durable log (the agent appends other records by hand), so a hand-appended illegal marker is still possible; the validate backstop (B2) is still required, which means A1 does not actually remove the need for W6. A1 is therefore B2 plus a convenient correct path, not a replacement for it.
- Structured data first, project for humans (P8): well served, the marker is always well-formed because the tool writes it.
- Idempotent (P4): a `resume` run twice must be a no-op-or-refuse, and `checkpoint` twice must refuse; achievable but it is new invariants to get right in a new write path.
- Prefer the cleaner long-term architecture (P1): mixed. It centralises the transition, but it breaks the tool's current clean separation (detection-only, no runtime write dependency, `src/workflow.rs` lines 1 to 6). Making the binary a runtime write dependency of the workflow is a real coherence cost: a project that runs the workflow now needs the tool installed and invoked to record a lifecycle event, where today it needs the tool for nothing at runtime.
- Minimal by default (P2): weakest here. Two new subcommands, a new write path, a git-cleanliness probe, and the idempotency invariants, for a transition the agent could record with one appended line. P2 cautions against exactly this, and the plan captured Q-59 as `exploring` partly for this reason (line 1266).
- Safe on existing projects (P3): a write command that appends to a committed log is safe if it is append-only and refuses on conflict, but it is more moving parts to keep safe than a validate-only reader.
- Reproducible (P7) and Ground decisions in evidence (P6): the clean-tree gate needs `git status`, so behaviour now depends on git being present and on the working-tree state at run time, a small reproducibility wrinkle. P6 argues we have no evidence yet that hand-authoring the marker is error-prone enough to justify owning the write.

### Option A2: validation-only (the agent writes the marker, the tool reads and validates)

No new write command. The lifecycle marker is recorded by the agent as part of the existing checkpoint / resume procedure, exactly as rounds and decisions are recorded today. The tool gains the read surface (B1) and the validate backstop (B2) only.

- Prefer the cleaner long-term architecture (P1): strongest here. It keeps the tool's detection-only architecture intact and records the lifecycle event by the SAME mechanism as every other workflow event (agent appends, tool validates), so there is one recording model, not two. This is the coherent extension the plan's "reconcile, do not bolt on" wording asks for (line 1266).
- Minimal by default (P2): strongest here. No new subcommand; the surface is a new field on the existing `next` / `status` projection and a new check inside the existing `validate --workflow` funnel (`src/workflow.rs` line 216 onward).
- Structured data first, project for humans (P8): well served. The marker is structured and the tool projects it; P8 is about the DATA being structured and projected, not about the tool being the writer, so the agent appending a structured record satisfies it.
- Make illegal states unrepresentable (P5): served backward rather than at the boundary. An illegal marker CAN be written by hand, and W6 catches it in `validate` (which the commit gate and CI run). This is exactly how the workflow already treats round-log consistency and W3: the illegal record is representable but reported. Weaker than A1 at the instant of writing, but identical in end-state enforcement, and consistent with the rest of the tool.
- Idempotent (P4): trivially satisfied, `validate` and `next` are pure reads, run any number of times with no effect.
- Safe on existing projects (P3) and Reproducible (P7): best here. No new write path, no git probe at write time; `validate` and `next` read durable files as they already do.
- Ground decisions in evidence (P6): this is the P6-preferred option: build the enforcement (which is load-bearing) and defer the write-command convenience until there is evidence the discipline load is real.

### Option A3: hybrid (validation-only now, writer-ready surface)

Build A2 exactly, but shape the read field and the W6 check so a later `checkpoint` / `resume` writer (A1) can be added without reshaping them: the marker schema, the `session_state` enum, and the transition table are defined once and shared, so a future write command would just append the same marker these already read. This is A2 as the built artifact plus a named seam, not a third mechanism.

- All principles score as A2, since the built surface IS A2. P1 gains slightly: naming the seam records the intended future shape so a later writer is a clean extension rather than a retrofit. P2 holds because nothing extra ships now.

## 4. Recommendation

Recommend Option A3: validation-only now (A2), with the read and validate surfaces shaped so a thin `checkpoint` / `resume` writer can be added later without reshaping them. Concretely:

Write surface (now): none. The lifecycle marker is appended by the agent as one step of the existing checkpoint and resume procedures, the same recording model as rounds, decisions, and waivers. Reasoning: this keeps the tool's detection-only architecture coherent (P1, `src/workflow.rs` lines 1 to 6), stays minimal (P2), and defers the write command until evidence shows hand-authoring is error-prone (P6). A1's stronger P5 story does not remove the need for the validate backstop, because the tool is not the sole log writer, so A1 is strictly additive convenience, not a substitute for W6.

Read surface: `next` and `status` gain a `session_state` dimension DISTINCT from `LoopState`, not folded into it. The lifecycle (Active versus Checkpointed{kind}) is orthogonal to the review-loop sub-state: a session can be checkpointed whether the active loop is AwaitingFixes or Converged. Folding the two orthogonal axes into one enum would manufacture illegal combinations and a combinatorial mess, the opposite of P5. So add a top-level `session_state` field to `NextProjection` (`src/next.rs` around lines 78 to 98) and to the human render, carrying its own valid transitions (`checkpoint -> resume`), exactly as `LoopState` carries `valid_transitions`. When `session_state` is Checkpointed, `next` re-routes: it suppresses or clearly gates the writer / reviewer instruction and instead emits "session is checkpointed (`<kind>` at `<commit>`); run the resume procedure before doing work", which is the "refuse do-work advice while checkpointed" behaviour. `next` thus becomes the read-side entry point on resume: checkpointed on the first call, Active after the agent records the resume marker and completes the preflight (`AGENTS.md` line 104). Two independent state machines, both projected by the one advisory driver (P8).

Enforce surface: `validate --workflow` gains one new check inside the existing `run_checks` funnel (`src/workflow.rs` line 216 onward), call it W6, the lifecycle-legality check. It reads the ordered lifecycle markers from the durable log and reports:

- A `resume` with no preceding unmatched `checkpoint` (a resume that does not follow a checkpoint).
- A `checkpoint` while already checkpointed (two checkpoints with no intervening resume).
- A dangling open checkpoint is NOT itself a violation (a session legitimately sits checkpointed); it is reported as state, not a fault.

The tree-cleanliness constraint needs an honest split. "The checkpoint commit must be clean" is a live-working-tree property: whether uncommitted work existed at checkpoint time cannot be reconstructed from a durable log after the fact (a commit is by definition a clean snapshot; the risk is uncommitted changes, which leave no durable trace). So enforce clean-tree at the checkpoint BOUNDARY (the procedure, and the future write command if built, checks `git status --porcelain` is empty), NOT in `validate`. W6 enforces only what it can reconstruct purely from durable files: the transition SEQUENCE. If the storage lens (q59-data) has the checkpoint marker carry the HEAD commit sha, W6 can additionally check that the sha names a real, reachable commit, but it should not claim to verify tree-cleanliness it cannot see. Stating this split honestly is itself P5 and the fail-loud posture: do not pretend to enforce an invariant the inputs cannot support.

Composition with Q-58: reconcile, do not bolt on (plan line 1266). Q-58 asks whether `next` should PROJECT the transient resume-state rather than verbatim-echo the ledger's `## RESUME STATE` block (`src/main.rs` lines 1092 to 1109 and 1144 to 1156). If Q-58 goes structured, the transient in-flight pointer (Q-58) and the session lifecycle state (Q-59) are neighbours in the SAME structured resume object, both projected by the SAME `next` path. Q-58 owns the transient-content half; Q-59 owns the lifecycle-state half; the command surface (a `next` that projects, plus a validate backstop) is shared, not duplicated. If Q-58 stays verbatim-echo, Q-59's `session_state` is still a clean separate field and the recommendation stands unchanged; the two do not block each other.

Storage note that bears on this surface (deferred to q59-data, flagged because it changes whether the read surface works universally): if the lifecycle marker lives in the JSONL log, it exists only for `--instrument` projects, so a non-instrumented project gets no lifecycle read or enforcement, reopening the Q-55 two-tier gap. `next` and `status` already read the LEDGER unconditionally for the RESUME STATE block (`src/main.rs` lines 1092 to 1109), so a ledger-resident or plan-`[meta]`-resident lifecycle field is readable by `next` for every project regardless of instrumentation, which the read-side re-routing (B1) needs to be universal. This argues the lifecycle marker belongs in a substrate `next` already loads (the ledger or the plan), not in the opt-in JSONL log, independent of where round records live. The command-surface recommendation is substrate-agnostic, but it reads best, and enforces universally, from a substrate that is always present.

Mapping the three user-prompts onto this surface:

- `pause.md` and `compaction-prep.md`: stay prose that instruct the agent to run the checkpoint procedure (flush the plan, the ledger, and the queue; commit so the tree is clean; record where the task stands and what comes next). The only change is that the procedure now ALSO appends the structured lifecycle marker (`kind = pause` or `kind = compaction`). They do NOT become thin wrappers calling a command, because no write command ships in this recommendation. If the future writer (A1) is later adopted, they can shrink to "run `agent-scaffold checkpoint --kind pause`, then confirm the tree is clean."
- `resume.md`: stays prose, and gains one concrete instruction: run `agent-scaffold next` first. On resume `next` reports `session_state = Checkpointed` and the resume steps; after the agent reconstructs state, runs the preflight, and records the resume marker, `next` reports `session_state = Active` and returns to normal loop advice. This makes the read surface the resume entry point rather than a separate ceremony.

## 5. What NOT to build (the YAGNI boundary)

- Do NOT build the `checkpoint` / `resume` write subcommands now. They are additive convenience over a one-line appended marker; the validate backstop (W6) is needed regardless, so the write command does not remove work, it adds a new runtime write path and new idempotency invariants. Build it only when evidence (P6) shows hand-authoring the marker is error-prone in practice. Reserve the verbs and the marker schema so the later addition is a clean extension (A3), but ship nothing for them.
- Do NOT fold `session_state` into `LoopState`. They are orthogonal axes; keep two enums so illegal combinations stay unrepresentable (P5).
- Do NOT try to verify tree-cleanliness inside `validate --workflow`. It is a live-tree property the durable log cannot reconstruct; enforce it at the checkpoint boundary and let W6 enforce only the transition sequence (and, at most, that a stored commit sha is real).
- Do NOT make `next` or `validate` BLOCK or refuse to run while checkpointed. `next` is advisory (`src/next.rs` lines 1 to 8); it RE-ROUTES its advice, it does not gate execution. The blocking backstop is `validate --workflow` in the commit gate and CI, consistent with how every other invariant is enforced.
- Do NOT add a third lifecycle state beyond Active and Checkpointed{kind} (for example a distinct "Paused" separate from "Checkpointed"). Pause, compaction-prep, and end-of-session all resolve to the same durable fact: the session is checkpointed and owes a resume. The `kind` field carries why; the state machine has two states. Adding more states is speculative until a transition genuinely needs to be treated differently.
- Do NOT introduce a git dependency into `validate --workflow` for this check. Keep its inputs the durable files it already reads, so it stays reproducible (P7) and runs anywhere.
