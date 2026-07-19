<!-- GENERATED FILE - do not hand-edit. Source: render-fixture.plan.toml, render-fixture.steps/, render-fixture.questions/, and the [meta].sidecars prose (front/tail). Regenerate with `agent-scaffold render render-fixture.plan.toml`; hand edits are overwritten and caught by `agent-scaffold render --check`. -->

# Render fixture plan

Status: 4 steps (1 not started, 1 in progress, 1 complete, 1 next); 1 open questions; 2 waivers (1 predates-logging, 1 accepted-at-escalation).

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

## Documentation Protocol

Generated status vocabulary (from the code constants, so it cannot drift):

- Roadmap statuses: not started, in progress, complete, skipped, next, optional, deferred, blocked on <slug>.
- Queue statuses: open, exploring, decided, superseded.

## Open Questions, Decisions, Issues and Blockers

- `Q-1` (open) An open ask still awaiting a decision.

The Q-1 body. An opaque question sidecar spliced verbatim after the one-line queue
item; render never parses it.
- `Q-2` (decided -> folded into `alpha`) A decided ask folded into alpha. Receipt: `Q-2`.

The Q-2 body. The decision reasoning lives here as prose; the options and chosen
value live only in the JSONL decision receipt, not in the TOML.
- `Q-3` (superseded by `Q-1`) A superseded ask, replaced by a later item.

The Q-3 body. This item was superseded by a later question.

## Roadmap

| Step | Status | Notes |
| --- | --- | --- |
| `alpha` | complete | waived: increment `alpha-inc1` accepted-at-escalation (record-backed) - Accepted below its streak at a human escalation. |
| `beta` | in progress | blocked on `alpha`; waived: step predates-logging (self-declared) |
| `gamma` | next | blocked on `beta` |
| `delta` | not started |  |

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

## Success Criteria

- The render engine produces a deterministic, byte-stable view from the TOML source
  and its sidecars.
- A hand-edit of the generated file, or a stale render, is caught by render --check.
