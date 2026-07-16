# Exploration: workflow self-improvement cluster (Q-27, Q-28, Q-29)

First instance of the `docs/plans/<task>.explorations/<question>.md` convention defined in the `exploration-mode` step. Records the design-space exploration that settled the `Q-27`/`Q-28`/`Q-29` cluster on 2026-07-16. Method: two independent design explorers (opus + sonnet, different models), each covering the whole cluster from first principles, synthesised by the orchestrator, then decided by the human. This is the review analog `exploration-mode` prescribes (independent perspectives plus synthesis rather than one unchecked take); it surfaced a real design fork a solo pass would have missed.

## The question

The workflow relies on the LLM orchestrator's disposition where deterministic structure could carry the invariant instead. Three symptoms:

- `Q-27`: a skipped or unreviewed step should FAIL a check, not depend on the orchestrator not skipping (the `pause.md` incident: a `complete` step committed outside the review loop, uncaught).
- `Q-28`: token-efficient state navigation on resume instead of reading whole plan/ledger files.
- `Q-29`: a first-class mode for exploring a design space before options are decidable (the mode this document is written in).

## Design space and the fork

Both explorers converged on ~80%:

- Extend the existing validator/projection family: single source of truth per region, detection not prevention, hard-fail, no binary required at workflow runtime. Reject a stateful `agent-scaffold step complete <slug>` mutation gate (prevention; makes the CLI a second writer of step status and a runtime dependency).
- `Q-27` = a `validate --workflow` cross-reference check. The strictness tension resolves as trivial-is-declared-not-skipped: a trivial change may skip the review loop but must be declared trivial, and the check confirms the declaration, turning a silent skip into a visible one. Do not auto-detect "doc-only" (agent-owned policy).
- `Q-28` = a thin `next` plus a `status --resume` slice, not a separate `state` verb and not a wholesale navigator. Honest token analysis: the win is real only at cold resume (the unbounded ledger) and in CI/hooks, and negative per-turn when the file is already in context; scope to the resume and check paths, do not poll each turn.
- `Q-29` = an `exploring` queue status (one code line; the drift guard auto-extends), a design-notes artifact convention parallel to `.reviews/`, a named exploration intake mode, and an optional light review. Mostly documentation.
- Build order: `Q-29` first (cheapest, unblocks proper deliberation), then the enforcement machinery, then `Q-28`; separate reviewed steps.

The one real fork, on what `validate --workflow` treats as authoritative:

- Sonnet: the metrics log (cheapest, reuses `metrics.rs` and the `risk_class`/`reviewers` records; but optional, only with `--instrument`, and the same LLM-written artifact).
- Opus: the ledger, via a keystone `src/ledger.rs` parsing the round table + a formalised artifact register into a derived `WorkflowState` all three consume (stronger: the ledger is always present during a task and is the canonical review record; closes the `state-schema`-deferred ledger-parse gap; is the unbounded file where the `Q-28` token win actually lives). Bigger. Opus's design already contains sonnet's as an optional "W4 metrics agreement" corroboration, so they are not really competing; opus is the fuller architecture with sonnet's as a component.

## Trade-offs against the Project Principles

- Principle 16 (one source of truth): everything is a derived projection; the ledger register is the single home for review classification; rejected a Roadmap status column and the mutation gate because each creates a second writer/store.
- Principles 5/12 (illegal states caught, fail loudly): `validate --workflow` hard-fails, matching the family.
- Principles 2/13 (harness-agnostic, detection over prevention): no binary on PATH at workflow time; checks run in CI/hook/on demand.
- Principle 3 (evidence): PoC the ledger parser against this repo's own hand-written ledger before building out; tighten the template formats to what the real ledger can satisfy.
- Principle 4 (small changes): four separately reviewable steps, not one.

## Recommendation (orchestrator synthesis)

Adopt opus's staged spine with sonnet's discipline: the ledger as primary authoritative source, the metrics log as corroboration when instrumented, built as staged increments; keep declared-trivial (not auto-detected) and treat `Q-28` as build-after-the-parser. This coheres with the same-session metrics work (a `trivial` risk class joins the `low_risk`/`risky` we shipped).

## What NOT to build (YAGNI)

- The `agent-scaffold step complete <slug>` state-machine mutation gate.
- A `state` verb (synonym for `status --json`); a wholesale navigator; ledger querying beyond `next`/`status --resume`.
- The agent dispatch/return event log (already deferred to `workflow-viz`).
- Auto-detection of "doc-only/trivial"; mandatory exploration review; a `--workflow-strict` triviality-reason column (add only if metrics later show declared-trivial being abused).
- A new workflow phase for exploration, or a machine-parsed exploration schema.

## Decision (human, 2026-07-16)

- Enforcement source: ledger primary + metrics secondary.
- `Q-28` query commands: build after the `ledger-parse` keystone.
- Fold into the Roadmap as steps (`exploration-mode`, `ledger-parse`, `workflow-invariants`, `state-queries`) and build `exploration-mode` first.

Open sub-questions carried into the build: the plan-slug to ledger-artifact join is a checked convention (leading slug must resolve to a Roadmap slug), and `status --resume` echoing the human-authored RESUME STATE narrative is section-extraction, not derivation, so its token win is smaller than a fully-derived summary; both to be validated against a real ledger and a real compaction during the build.
