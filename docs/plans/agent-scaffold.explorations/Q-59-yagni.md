# Q-59 design notes: the YAGNI / minimal-skeptic lens

## 1. The question

Should the session lifecycle, the `pause` / `compaction-prep` / `resume` user-prompts, be encoded as explicit, tool-validated states rather than the current prose-only checklists plus procedures?

Concretely, the proposal (from the `Q-59` `[[question]]` entry, `docs/plans/agent-scaffold.plan.toml:1264`) is some combination of: a lifecycle state recorded somewhere durable (`[meta]` enum in `plan.toml`, or `type:"checkpoint"` / `type:"resume"` records in `workflow.jsonl`, or a structured ledger field); a command surface (`checkpoint` / `resume` subcommands that write and validate the transition, versus validation-only where `next` / `status` refuse "do work" advice while checkpointed); and enforcement in `validate --workflow` (a resume must follow a checkpoint; no two checkpoints without an intervening resume; the checkpoint commit must be clean).

This document is the adversarial check: is this worth building at all, and if so what is the smallest version? It argues the minimal case, demands concrete evidence, and takes "do nothing" and "fold into Q-58" as first-class options, not strawmen.

## 2. What exists today (the baseline the proposal would replace or extend)

- The review-loop state machine. `next`'s `LoopState` (`src/next.rs:169-200`) models only the review loop: `ReadyToPlan`, `Blocked`, `AwaitingFirstReview`, `AwaitingFixes`, `AwaitingReviewers`, `Converged`, `Escalate`, `RiskClassConflict`, `Done`. There is no `Active` / `Checkpointed` / `Paused` lifecycle state, by design: `next` is read-only and stateless (`src/next.rs:1-8`), recomputed from the durable files each call.
- The resume-state read path already exists. `extract_resume_state` (`src/next.rs:936-955`) pulls the ledger's `## RESUME STATE` section verbatim; `next` echoes it (`src/next.rs:1003-1008`), and the shipped `status --resume` slice reuses the same helper. A resuming agent already has a tool that surfaces the transient state.
- The prose lifecycle. `.agents/user-prompts/pause.md`, `compaction-prep.md`, and `resume.md` are short checklists that trigger, but do not restate, the procedures in `AGENTS.md` "Checkpoint and resuming after context loss" (`AGENTS.md:97-102`). The checkpoint flushes the plan/ledger/queue, pushes open items to the human, verifies the Status line, and commits; resume reconstructs from those files and continues.
- Two prose guards already sit at exactly the resume boundary this proposal targets. Preflight (`AGENTS.md:104`) fires "whenever a session is resumed" and confirms adherence with the human before proceeding. Task-entry re-grounding (`AGENTS.md:106`) fires "again on resume after reconstructing state" and pushes a go/no-go brief before any step starts. Both were themselves added as drift mitigations (`Q-53`, `Q-54`); they are the incumbent answer to "an agent resuming and starting work without the human's re-confirmation."

The important framing point: the lifecycle is not un-modelled. It is modelled in prose, and it already has two point-of-action human gates on resume. Q-59 asks whether to promote that prose to tool-validated state.

## 3. The evidence test (Principle 6, Ground decisions in evidence)

The plan names P5 (make illegal states unrepresentable) and P8 (structured data first) as the drivers for Q-59. Before weighing those, ask the P6 question the plan itself insists on: what concrete failures has the implicit lifecycle actually caused, and would a validated state have prevented them? I read the ledger's `## RESUME STATE` anchor and its superseded history (`docs/plans/agent-scaffold.ledger.md`) for real incidents. There are two, and both point away from lifecycle states.

Incident 1: the stale-anchor bloat (`docs/plans/agent-scaffold.ledger.md:337`, the `Q-58` capture). The orchestrator accumulated 27 dated `CURRENT TRANSIENT STATE` anchors under one `## RESUME STATE` heading, only 1 of them live, so `next` echoed 150145 bytes of which ~149 KB was dead history. This is a real, measured failure of the implicit lifecycle. But its nature is staleness / currency of the transient content, not a missing transition guard. The fix that landed was a source-bound heading; the deeper fix is `Q-58` (project or structure the transient rather than echo whatever sits under the heading). A "this session is checkpointed" enum would not have prevented anchor accumulation; a resume-follows-checkpoint check would not have caught it.

Incident 2: the resume-time control drift (`docs/plans/agent-scaffold.ledger.md:369`, the `Q-54` capture). "Since the compaction the orchestrator had been presenting AskUserQuestion decision gates as options-with-trade-offs but omitting the explicit recommendation and the Principle-judged reasoning." This is the closest thing in the record to a genuine resume-time failure, and it is instructive precisely because a lifecycle state would not have caught it. The failure was content-drift off the human-input contract, not "resumed without a checkpoint" or "started work while paused." The fix the human chose (`Q-54`, decided) was point-of-action reinforcement: a per-gate checklist in the orchestrator prompt plus an awaiting-human-decision reminder in `next`. Not a validated state. That is direct evidence that the resume-boundary failures this project has actually hit are answered by reminders and by content-currency, not by transition validation.

Now the three failure modes the proposal is designed to prevent, weighed against the record:

- A stale resume anchor. Happened (incident 1). Owned by `Q-58` (content currency), not by a lifecycle enum.
- An agent resuming and starting work without the human's re-confirmation. No recorded instance. The mechanism that would prevent it (Preflight + re-grounding, `AGENTS.md:104-106`) already exists and already gates resume, and there is no ledger evidence of it being skipped. Note also that a compaction destroys the session, so "paused" is not a state a single agent sits in and then violates; the fresh resumed agent reads the anchor and runs the re-grounding brief regardless of any recorded flag.
- A resume without a flush (context lost before the checkpoint committed). No recorded instance. Every checkpoint in the anchor history records "Main clean at `<hash>`," so the flush-and-commit discipline has held. And structurally a post-hoc state cannot prevent this failure, only detect it after the fact; detection reduces to "the anchor is stale or the tree is dirty," which is again `Q-58` / a plain `git status`.

Verdict on evidence: zero of the three target failure modes has a recorded instance that a validated lifecycle state would have prevented. The one staleness failure is `Q-58`'s; the one drift failure was fixed by reminders. This is a thin evidence base for a new mechanism, and P6 is explicit that an approach should be validated against real need before it is built out.

## 4. The design space

### Option A: do nothing (keep the prose lifecycle)

Keep `pause.md` / `compaction-prep.md` / `resume.md` as prose checklists over the `AGENTS.md` procedures, with Preflight and re-grounding as the resume-boundary gates.

- P2 (Minimal by default): strongest here. The core keeps doing one thing; no new state, no new command, no new record type, no reopened write-path. The proposal's own capture cautions that P2 "cautions against over-building the command surface."
- P6 (Ground decisions in evidence): supported. There is no recorded failure that this option leaves unaddressed; the two real incidents are handled elsewhere (Q-58, Q-54).
- P5 (Make illegal states unrepresentable): the cost of this option. Nothing structurally prevents a resume-without-checkpoint or work-while-paused. But note P5 says encode the valid states "rather than admitting bad states and guarding against them at runtime"; the counter is that these bad states have not been observed and the resume gate is human-in-the-loop, so the "illegal state" is already caught by a person at Preflight, not admitted silently.
- P8 (Structured data first): the other cost. The transient lifecycle stays prose. But the transient's structure is exactly `Q-58`'s subject, so this cost is better booked against `Q-58` than paid twice.
- Risk: relies on orchestrator discipline. That is the general anti-drift concern the whole workflow-driver initiative (`Q-51`) exists to address, but `Q-51` was decided to be built staged and evidence-first, advisory before authoritative; this option is consistent with that posture.

### Option B: fold into Q-58 (structure the transient; no separate lifecycle mechanism)

Treat Q-59 as not its own mechanism. `Q-58` (`docs/plans/agent-scaffold.plan.toml:1259`) already asks whether `next` should project a structured transient resume-state rather than echo prose. Let the transient's structure carry whatever minimal lifecycle signal is worth having (for example a single "checkpoint commit" or "last-updated" field on the one live transient record), and let `next` / `status` project it. No `checkpoint` / `resume` subcommands, no new record types, no `validate --workflow` transition rules.

- P1 (Prefer the cleaner long-term architecture over the smallest diff): strongest here. Q-59 and Q-58 both act on the same object, the transient resume-state; the plan says Q-59 "COMPOSES with Q-58" and a design pass "should reconcile Q-59 WITH Q-58 rather than bolt one on." One structured transient with the currency signal built in is cleaner than a transient (Q-58) plus a parallel lifecycle state machine (Q-59) that both describe the same "where is this session" fact.
- P8 (Structured data first): supported without duplication. The transient becomes structured once, in Q-58's work, and the lifecycle currency is a field on it, not a second substrate.
- P5 (Make illegal states unrepresentable): partially served. A structured transient with a checkpoint-commit field lets `next` / `status` compute "the anchor is N commits behind HEAD" or "the tree is dirty at resume," which is the detectable part of the lifecycle. It does not encode the full transition grammar, but per section 3 the transition grammar has no evidence behind it.
- P2 (Minimal by default): supported. This adds at most one field and one read-only derivation to work that is already planned; it does not add a command family or a write-path.
- Cost: it defers the transition-validation ambition entirely. That is the point: build the part with evidence (currency of the transient) and drop the part without (transition grammar) until evidence arrives.

### Option C: minimal standalone build (one validated field plus one read-only check)

If a lifecycle signal must be its own thing rather than a Q-58 field: record a single checkpoint marker (the commit the checkpoint was written at, e.g. a `[meta].last_checkpoint` in `plan.toml` or one `type:"checkpoint"` record), and add exactly one read-only check that `next` / `status` surface: at resume, warn if the tree is dirty or if the recorded checkpoint commit is far behind HEAD. No `resume` counterpart record, no "refuse to advise while checkpointed," no `validate --workflow` transition enforcement.

- P2: acceptable but weaker than A or B, because even one field plus one check is a new mechanism where none of its target failures has occurred.
- P4 (Idempotent) and P7 (Reproducible): a read-only check is fine on both; it recomputes from durable files and does not write.
- P5 / P8: modestly served (a structured marker, a detectable staleness signal), but this duplicates what Option B gets for free inside Q-58, so it scores worse on P1 than B.
- Verdict within the option: if anything ships, it should ship as a Q-58 field (Option B), not as a standalone marker (Option C), because C pays a second-substrate cost for the same signal.

### Option D: the full proposal (state + command surface + validate enforcement)

Record a lifecycle enum, add `checkpoint` / `resume` subcommands that write and validate the transition, and enforce resume-follows-checkpoint / no-double-checkpoint / clean-at-checkpoint in `validate --workflow`.

- P5 / P8: this is where the proposal scores best; the lifecycle becomes typed and its illegal transitions become unrepresentable.
- P2 (Minimal by default): strongly against. It adds a command family, at least two record types or a new `[meta]` state plus its invariants, and new validator rules, for failure modes with no recorded instances.
- P6 (Ground decisions in evidence): against. Building the full grammar ahead of a single observed transition failure is exactly the "forcing through an unvalidated approach" P6 warns off; the proof-of-concept it would demand does not have a failure to reproduce.
- Adoption / scope conflict: the `checkpoint` / `resume` write-subcommands reopen `Q-24` (the guarded write-path), which `Q-51`'s decided staging explicitly placed in "Stage 3+, evidence-gated," after the advisory tiers. The whole driver was decided to be advisory-first and to consume human/agent judgment as input, never to make control decisions itself; a `resume`-validating subcommand that refuses "do work" advice edges the tool from advising toward gating, ahead of the evidence gate the human set. This is the single largest reason to decline D now.
- P1: a full lifecycle FSM parallel to the review-loop FSM is arguably cleaner in the abstract, but only if the transient (Q-58) and the lifecycle (Q-59) are unified; built as a separate mechanism it is the "bolt one on" outcome the plan warns against.

## 5. Recommendation

Decline Option D now; fold Q-59 into Q-58 (Option B), with Option A (do nothing beyond the existing prose gates) as the acceptable fallback if Q-58's own resolution does not need a currency field.

Reasoning, against the principles by name:

- P6 (Ground decisions in evidence) is decisive and points at fold/defer. The three failure modes the full build targets have zero recorded instances that a validated lifecycle state would have prevented. The two real resume-boundary incidents were a staleness failure (Q-58's territory) and a content-drift failure (fixed by point-of-action reminders under Q-54). Building the transition grammar now is building ahead of evidence.
- P1 (Prefer the cleaner long-term architecture) favours reconciling with Q-58 over a standalone mechanism. Q-58 and Q-59 both operate on the transient resume-state; the plan itself says to reconcile rather than bolt on. The clean design is one structured transient that carries its own currency signal, not a transient plus a parallel lifecycle FSM.
- P2 (Minimal by default) rules out D and C as standalone builds. The minimal version of any lifecycle signal is a single field on the Q-58 transient plus a read-only staleness derivation in `next` / `status`, which costs almost nothing on top of already-planned work.
- P5 (Make illegal states unrepresentable) is the proposal's best card and is only partially satisfied by fold-in. That is the honest tension. The mitigation is that the "illegal states" here are caught by a human at Preflight and re-grounding (`AGENTS.md:104-106`), which already gate resume, so they are not admitted silently; and the detectable part of P5 (is this anchor stale, is the tree dirty) is exactly what a Q-58 currency field gives `next` to report. P5 does not compel encoding a transition grammar for transitions that have never been violated.
- P8 (Structured data first) is served once, inside Q-58, rather than paid twice. Structuring the transient is Q-58's job; the lifecycle currency rides on it.
- Adoption posture (`Q-51` decided): the driver is advisory-first and the write-path is Stage 3+ and evidence-gated. A `checkpoint` / `resume` write-and-validate subcommand pair jumps that gate and reopens `Q-24`. Keeping Q-59 read-only-or-nothing respects the staging the human already chose.

Concretely: when Q-58 is designed, give the single live transient record one machine-checkable currency field (the commit the checkpoint was written at, or a monotonic last-updated marker), and have `next` / `status` derive and show "resume anchor is current / N commits stale / tree dirty at resume." That captures the only lifecycle signal with evidence behind it (currency of the transient) at the marginal cost of a field, and it composes with Q-58 instead of competing with it. Everything else in Q-59 waits for a first recorded transition failure.

## 6. What NOT to build

- No `checkpoint` / `resume` subcommand family. It reopens `Q-24`'s guarded write-path, which `Q-51`'s decided staging placed at Stage 3+ behind an evidence gate, and it pushes the advisory tool toward gating.
- No new record types (`type:"checkpoint"` / `type:"resume"`) in `workflow.jsonl`, and no new `[meta]` lifecycle enum, as a standalone mechanism. If any currency signal is wanted, it is one field on Q-58's structured transient, not a second substrate.
- No `validate --workflow` transition enforcement (resume-follows-checkpoint, no-double-checkpoint, clean-at-checkpoint). There is no recorded transition failure to enforce against, and clean-at-checkpoint duplicates `git status`; add enforcement only if such a failure is ever observed (P6).
- No "refuse to advise while checkpointed" behaviour in `next` / `status`. It moves the tool from advising to gating, against the `Q-51` advisory-first decision; at most, `next` may show a staleness note, which is a report, not a refusal.
- No `Paused` / `Active` sub-state carried in memory across the compaction boundary. A compaction destroys the session, so a fresh resumed agent gets its "should I be doing work" answer from Preflight and re-grounding (`AGENTS.md:104-106`), which already gate resume; a persisted flag adds nothing a human gate does not already provide.
- Do not restate any of the `AGENTS.md` checkpoint / Preflight / re-grounding procedures in a new place. If a currency field lands, `next` references those sections; it does not duplicate them (P8 one-source).
