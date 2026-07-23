# Reviewer B findings: uniform-agent-isolation (Q-61)

Diff range reviewed: `1ce9de3..600e6a3` on `plan/uniform-isolation`.
Lens: faithfulness to the human's decision plus mechanical correctness (verified independently, not trusting the writer's report).

## Summary

The five load-bearing parts of the decision are encoded and the mechanical checks all pass. One faithfulness finding: the product-authority reframe reclassified explorers as read-only but left three stale writer-classifications in place, so AGENTS.md now contradicts itself about whether explorers are writers or read-only, and the Q-61 decision receipt asserts a resolution the shipped text does not carry.

## Faithfulness verification (each load-bearing part)

- Every spawned agent isolates (writers AND reviewers/triager/explorers): PRESENT in both `pack/AGENTS.md` and generated `AGENTS.md`. "Writer isolation (capability-tiered). Run every spawned agent, the writers and the read-only reviewers, triager, and explorers alike, in the strongest isolation the harness supports" (`AGENTS.md:83`), reinforced at `AGENTS.md:89` and in the rendered `{{isolation_policy}}` fragment (`src/isolation_policy.rs` `ISOLATION_POLICY_FRAGMENT`: "every spawned agent runs in the strongest isolation ... This holds for the writers ... and for the read-only reviewers, triager, and explorers alike").
- Orchestrator merges all outputs onto main: PRESENT (`AGENTS.md:91`, `AGENTS.md:93`; `pack/prompts/orchestrator.md` "merge each agent's output onto main yourself").
- Batch-merge (parallel agents merged only after they ALL finish, never as-each-completes): PRESENT in the worktree-lifecycle prose (`AGENTS.md:93`: "the orchestrator merges their worktrees onto main only after they ALL finish (a batch-merge), never as each one completes") AND in the tool's emitted orchestrator instructions (`pack/prompts/orchestrator.md` / `.agents/prompts/orchestrator.md`: "merge their worktrees onto main only after they ALL finish (a batch-merge), never as each one completes ... you batch-merge their findings onto main, then spawn the triager in its own worktree"). The human's explicit requirement that the tool instructions state it is met.
- Reviewers/triager write findings in their own worktree: PRESENT in `pack/prompts/reviewer.md` / `.agents/prompts/reviewer.md` ("inside your own worktree: you run isolated like every spawned agent, and the orchestrator merges your findings onto main") and `pack/prompts/triager.md` / `.agents/prompts/triager.md` ("inside your own worktree ... the orchestrator merges your verdicts onto main").
- Product-authority reframe of read-only vs writer: PRESENT (`AGENTS.md:89`: "that distinction is now purely about authority over the reviewed product (the plan and the code), not about isolation ... a read-only agent (a reviewer, the triager, or an explorer) is read-only with respect to that product, authoring only its own findings or notes, which inform the work but are never accepted as product").

## Findings

### F1. Explorer classification contradicts itself; the Q-61 receipt claims a resolution the text does not carry

Severity: medium

Evidence:
- `AGENTS.md:65` (the "Design explorations" paragraph, UNCHANGED by this diff): "Explorers are writer agents, so the file-safety and writer-isolation rules below apply to them."
- `AGENTS.md:83` and `AGENTS.md:89` (added by this diff): explorers are listed among the read-only agents, "the writers and the read-only reviewers, triager, and explorers alike", and explicitly "a read-only agent (a reviewer, the triager, or an explorer) is read-only with respect to that product". The rendered isolation fragment (`src/isolation_policy.rs`) likewise groups explorers under "the read-only reviewers, triager, and explorers".
- These two statements are in the SAME file and are flatly contradictory on the word "writer": `AGENTS.md:65` says explorers ARE writer agents; `AGENTS.md:89` says an explorer IS read-only. The identical contradiction exists in the pack source (`pack/AGENTS.md:65` vs the edited paragraphs), so it is not a render artifact.
- The Q-61 decision receipt makes this worse by asserting the opposite of what shipped. `docs/plans/agent-scaffold.plan.toml` (`[[question]] id = "Q-61"`) and the plan Open-Questions entry state the decision "RESOLVES the explorer-classification inconsistency (explorers are writers that isolate, consistent with the 'Explorers are writer agents' line, so they are no longer listed among read-only agents needing no worktree)." The shipped text does the reverse: it KEEPS the "Explorers are writer agents" line untouched AND newly lists explorers among the read-only agents.
- The step detail compounds it: `docs/plans/agent-scaffold.md` (uniform-agent-isolation step "Why it exists") says "explorers were already called writer agents that isolate in the Design explorations section, yet the worktree-lifecycle rule still listed them among read-only agents needing no worktree; the uniform rule removes that contradiction." The contradiction is not removed; it is relocated (now between `AGENTS.md:65` and `AGENTS.md:89`) and the receipt/step-detail claims are false of the shipped artifact.

Why it matters: the product-authority reframe is one of the load-bearing parts of the human's decision, and the explorer classification is the sharp edge that reframe was meant to settle. A reader of `AGENTS.md` sees explorers called both "writer agents" (`:65`) and "read-only" (`:89`) and cannot tell which governs; the Q-61 receipt (a durable, W4-enforced decision record) records a resolution the code does not implement. Behavioural isolation is unaffected (explorers isolate under either label), which is why this is medium and not high. The product-authority definition itself is coherent and arguably makes read-only the correct classification for explorers (their notes are never accepted as product, exactly like reviewer findings), so the fix is to make the three stale writer-classifications agree with the reframe: update `AGENTS.md:65` (`pack/AGENTS.md`), the Q-61 receipt/question text, and the step-detail "Why it exists" paragraph, rather than to change the reframe.

### Other severities

- Critical: none.
- High: none.
- Low: none of substance. (The `next` driver still emits its isolation reminder only at the two writer-spawn states and not at reviewer/triager spawns; this is deliberately deferred and openly captured as `Q-62`, with an honest NOTE in `src/next.rs` `spawns_writer` and a content-pin test `the_reviewer_states_carry_no_isolation_reminder` pinning current behaviour. The decision's "tool instructions state it" requirement is already satisfied by the orchestrator/reviewer/triager prompts and `AGENTS.md`, so this is not a finding, only noted for the triager's context.)

## Mechanical checks (output tails)

### `validate --source docs/plans/agent-scaffold.plan.toml`
```
docs/metrics/workflow.jsonl: 159 records, valid
docs/plans/agent-scaffold.plan.toml: 69 steps, 62 questions, valid
```
69 steps and 62 questions, as expected.

### `validate --workflow --source docs/plans/agent-scaffold.plan.toml`
```
docs/metrics/workflow.jsonl: 159 records, valid
docs/plans/agent-scaffold.plan.toml: 69 steps, 62 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```
Invariants hold. W4: the Q-61 receipt exists as a single appended `type:"decision"` line in `docs/metrics/workflow.jsonl` with `chosen` = "A: every spawned agent isolates; the orchestrator merges all outputs", which is present in its `options` array. Confirmed.

### `render --check docs/plans/agent-scaffold.plan.toml`
```
docs/plans/agent-scaffold.plan.toml: up to date
```

### `cargo test`
```
test isolation_policy::tests::the_committed_scaffold_carries_the_isolation_policy_fragment ... ok
test isolation_policy::tests::the_fragment_states_the_uniform_isolation_rule ... ok
test next::tests::a_writer_state_emits_the_isolation_fragment_for_any_tier ... ok
test tests::isolation_policy_slot_renders_the_generated_fragment ... ok
...
test result: ok. 342 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
```
Plus all integration test binaries passed (checks_staged_hook_env, scaffold_precommit_hook, validate_toml_primary_skips_markdown_plan, validate_workflow_toml_source_needs_no_plan). The byte-guard `the_committed_scaffold_carries_the_isolation_policy_fragment` (fragment edit and committed `AGENTS.reference.md` agree) and the content-pin `the_fragment_states_the_uniform_isolation_rule` both pass; no guard was weakened (the renamed content-pin test tightened, adding a third assertion).

### `workflow.jsonl` single-line check (`git diff 1ce9de3..600e6a3 -- docs/metrics/workflow.jsonl`)
One appended line only (the Q-61 receipt); no existing line modified. Confirmed.

### Re-scaffold determinism
`cargo run -- scaffold --output-dir . --write --force --principles default --instrument` (no `nix fmt`) left the working tree clean (`git status --porcelain` empty for tracked files). The committed `AGENTS.md` / `.agents/AGENTS.reference.md` / `.agents/prompts/*` exactly match scaffold output.
