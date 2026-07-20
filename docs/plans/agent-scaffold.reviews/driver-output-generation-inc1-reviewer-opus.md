# Reviewer 1 (opus) findings: driver-output-generation increment 1

Scope: generation mechanism, drift-guard, and fragment content of the shared
`isolation_policy` fragment (Stage-0b pattern) rendered into `AGENTS.md`.
Diff `main (041c581) .. impl/dog-inc1 (9a8ea5a)`. Reviewed in worktree
`.claude/worktrees/dog-inc1`.

## Verdict summary

Drift-guard is NON-VACUOUS in both directions and matches Stage-0b rigor.
Generation wiring is correct. `src/next.rs` is untouched. All checks green
(`cargo test --bins` 336 passed; clippy clean; `validate --source`,
`validate --workflow --source`, `render --check --strict` all exit 0). No
non-ASCII, em-dash, en-dash, or double-hyphen-as-dash in the new code or
strings; no `#[allow]`. Three low-severity findings, none blocking.

## Drift-guard non-vacuity (the load-bearing check)

`the_committed_scaffold_carries_the_isolation_policy_fragment`
(`src/isolation_policy.rs:63-78`) byte-asserts the committed root `AGENTS.md`
and `.agents/AGENTS.reference.md` each `contains(ISOLATION_POLICY_FRAGMENT)`,
mirroring `the_committed_scaffold_carries_the_generated_fragment`
(`src/workflow_spec.rs:230-248`). Verified empirically against the committed
files:

- The const appears verbatim EXACTLY once in `AGENTS.md` and once in
  `.agents/AGENTS.reference.md`; it does NOT appear in `pack/AGENTS.md` (which
  correctly still holds the `{{isolation_policy}}` placeholder source), and no
  raw `{{isolation_policy}}` leaked into the committed outputs, so substitution
  is wired.
- Direction (a), hand-edit of the committed block: a one-character change to the
  committed `AGENTS.md` fragment makes `contains(const)` false, so the test
  fails. Confirmed.
- Direction (b), source-side reword without re-scaffold: a reworded const is no
  longer a substring of the committed `AGENTS.md`, so the test fails. Confirmed.

Both directions are live. Rigor matches the Stage-0b guard.

## Findings

### D1-1 (low): fragment narrows planner isolation to the worktree tier, in tension with the capability-tiered order it references
`src/isolation_policy.rs:30` (propagated to `AGENTS.md:91`,
`.agents/AGENTS.reference.md:91`).
The fragment's first sentence states the correct capability-tiered rule ("every
spawned writer runs in the strongest isolation the harness supports, per the
capability-tiered tier order above"), but the next sentence says the planner and
exploration writers "each runs worktree-isolated exactly as the implementer
does, and the orchestrator merges its branch back on convergence". This hardcodes
the WORKTREE tier: under a container-capable harness the writer runs in a
container (tier 1), not a worktree, and "merges its branch back" is worktree-
specific integration. The worktree-lifecycle paragraph immediately below is
careful to gate on "When the resolved isolation tier is a worktree"
(`AGENTS.md:93`); the new fragment states it unconditionally. The design intent
was "planners are writers and are isolated under the same tier order"
(`driver-output-generation.design.md:105`), not pinned to worktree.
Why it matters: this is the ONE canonical single-source fragment; the imprecision
will project verbatim into the driver's writer-state reminder in increment 2.
Accurate to this repo's current practice (worktree is the resolved tier), and the
"worktree-isolated" shorthand also appears in the increment framing, so this is
low, not a correctness bug; but the fragment contradicts the tier order it itself
references. Consider "runs isolated under the same tier order as the implementer"
(and generalising the merge-back clause) to keep the fragment harness-agnostic.

### D1-2 (low): guard-parity gap versus the Stage-0b precedent (no build-path slot test, no reserved-var rejection test)
`src/main.rs:264-270`, `src/manifest.rs:140-141`.
The committed-scaffold byte-guard covers drift well, but two guards the
`workflow_control` precedent carries have no `isolation_policy` counterpart:
(1) `workflow_control_slot_renders_the_generated_fragment` (`src/main.rs:1888`)
exercises `build_assets` directly and asserts the `{{workflow_control}}`
placeholder is gone and the fragment present; there is no equivalent
build-path test for `{{isolation_policy}}`. (2)
`reserved_workflow_control_variable_is_rejected` (`src/manifest.rs:1208`) pins
that a pack cannot declare/override the reserved var; there is no equivalent for
`isolation_policy`.
Why it matters: the byte-guard only fails when the committed output is
re-scaffolded, so a broken `build_assets` insert (removed or mis-keyed) or a
typo'd `RESERVED_VARS` entry would pass all tests until the next
`scaffold-self`. The RESERVED_VARS and substitution mechanisms are generic and
verified for `workflow_control`, so risk is low, but the increment does not match
the precedent's guard rigor the design cites.

### D1-3 (low): fragment restates the writer-isolation conclusion already stated in the two paragraphs directly above it
`src/isolation_policy.rs:30` (rendered at `AGENTS.md:91`).
The opening clause "every spawned writer runs in the strongest isolation the
harness supports ... while read-only agents need none" re-asserts the Writer
isolation rule ("Run each writer agent in the strongest isolation the harness
supports", `AGENTS.md:83`) and the read-only paragraph ("Read-only agents need
no isolation", `AGENTS.md:89`) sitting immediately above. Route B intended the
fragment to ADD only the writer-classification clarification and REFERENCE the
tier order (which it does correctly: "per the capability-tiered tier order
above", no tier-list duplication). The re-assertion is a mild restatement of
adjacent prose rather than a new second source of the tier order; it reads as a
deliberate lead-in, so this is low. Trimming the lead-in to the new
classification content would tighten it.

## Confirmations (non-findings)

- Generation wiring: `{{isolation_policy}}` inserted in `build_assets`
  (`src/main.rs:264-270`) parallel to `workflow_control`; registered in
  `RESERVED_VARS` (`src/manifest.rs:141`). No other `{{...}}` slot or asset is
  broken; full test suite and all scaffold-path tests pass.
- Placement in `pack/AGENTS.md:90` (between the read-only paragraph and the
  worktree-lifecycle paragraph, inside the Writer-isolation area) is sensible;
  the generated result reads coherently in context, and the "tier order above"
  reference resolves to the rule directly above.
- Content fidelity: the classification (spawned planner and design/exploration
  writers are writers; worktree-isolated; orchestrator merges back; DISTINCT
  from the orchestrator's integration-level on-main edits: step-status flips,
  increment declarations, round records, ledger anchors) is present and matches
  the AGENTS.md Writer-isolation rule and project practice, apart from the tier-
  narrowing in D1-1. It references, not restates, the tier list.
- The content-pin test `the_fragment_states_the_writer_classification`
  (`src/isolation_policy.rs:44-60`) uses two `contains` checks rather than a full
  `assert_eq` like `control_fragment`; this is correct here, because the fragment
  is a `&'static str` (nothing computed), so a full-string assert would be
  circular. Not a finding.
- Behaviour preservation: `src/next.rs` untouched (increment 2 owns the driver's
  consumption); plan untouched; `validate` and `render --check --strict` green.
- Style: ASCII-only; no em-dash/en-dash/double-hyphen-as-dash/emoji in the new
  fragment, code, or prompt edits; no `#[allow]`.
