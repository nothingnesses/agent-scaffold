<!-- GENERATED FILE - do not hand-edit. Source: render-fixture.plan.toml, render-fixture.steps/, render-fixture.questions/, and the [meta].sidecars prose (front/tail). Regenerate with `agent-scaffold render render-fixture.plan.toml`; hand edits are overwritten and caught by `agent-scaffold render --check`. -->

# Render fixture plan

Status: 7 steps (1 not started, 1 in progress, 1 complete, 1 skipped, 1 next, 1 optional, 1 deferred); 3 open questions; 2 waivers (1 predates-logging, 1 accepted-at-escalation).

This is the render fixture intro prose. It is an opaque front-matter sidecar that
render splices verbatim, without parsing or rewriting it.

## Motivations

Why this fixture exists: to exercise the render engine end to end on a small,
self-contained plan skeleton and its prose sidecars.

## Repository Layout and Current Architecture

A front-matter prose sidecar standing in for the repository-layout index. Render
inlines it verbatim; it holds no machine-parsed structure.

## Project Principles

1. Ask clarifying questions before forging ahead - Confirm intent before writing code, and give recommendations with reasoning.
8. No silent scope expansion - Do what was asked and flag anything else rather than quietly doing it.

## Roadmap Status Vocabulary

Generated status vocabulary (from the code constants, so it cannot drift):

- Roadmap statuses: not started, in progress, complete, skipped, next, optional, deferred, blocked on <slug>.
- Queue statuses: open, exploring, decided, superseded.

## Open Questions, Decisions, Issues and Blockers

- `Q-1` (open) An open ask still awaiting a decision.
- `Q-2` (decided -> folded into `alpha`) A decided ask folded into alpha. Receipt: `Q-2`.
- `Q-3` (superseded by `Q-1`) A superseded ask, replaced by a later item.
- `Q-9` (exploring) An exploring ask, still owed a design pass.
- `Q-10` (open) A tenth open ask, to pin numeric ordering past Q-9.

## Roadmap

| Step | Status | Notes |
| --- | --- | --- |
| `alpha` | complete | waived: increment `alpha-inc1` accepted-at-escalation (record-backed) - Accepted below its streak at a human escalation.; why: decisions Q-2; findings render-fixture.findings/alpha.md; commits abc1234 |
| `beta` | in progress | blocked on `alpha`; waived: step predates-logging (self-declared) |
| `gamma` | next | blocked on `beta` |
| `delta` | not started |  |
| `epsilon` | skipped |  |
| `zeta` | optional |  |
| `eta` | deferred |  |

## Step Details

### `alpha`: The first step

The alpha step body. This opaque step sidecar is spliced verbatim into the Step
Details section; render never reads its structure.

### `beta`: The second step

The beta step body. It is blocked on alpha, which the Roadmap Notes column shows.

### `gamma`: The third step

The gamma step body, queued next behind beta.

### `delta`: The fourth step

The delta step body, not started yet.

### `epsilon`: The skipped step

The epsilon step body. This step was skipped; it shares Roadmap order 5 with zeta.

### `zeta`: The optional step

The zeta step body. It is optional and shares Roadmap order 5 with epsilon.

### `eta`: The deferred step

The eta step body. It is deferred to a later pass.

## Question Details

The Q-1 body. An opaque question sidecar spliced verbatim into the Question Details
section; render never parses it.

The Q-2 body. The decision reasoning lives here as prose; the options and chosen
value live only in the JSONL decision receipt, not in the TOML.

The Q-3 body. This item was superseded by a later question.

The Q-9 body. An exploring question, still owed a design pass before its options
are decidable.

The Q-10 body. A tenth open question, present to pin the numeric question sort.

## Success Criteria

- The render engine produces a deterministic, byte-stable view from the TOML source
  and its sidecars.
- A hand-edit of the generated file, or a stale render, is caught by render --check.
