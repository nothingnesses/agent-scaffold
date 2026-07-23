# uniform-agent-isolation, round 3 (final confirmation) reviewer-e findings

Reviewer: reviewer-e (round 3, the second of the two consecutive confirmation rounds a risky artifact requires).
Range under review: `1ce9de3..856ddf1`.

## Verdict: No new findings. Round is CLEAN.

I read the whole uniform-isolation rule end to end (both the guidance prose and the emitted tool instructions) looking for a NEW contradiction, ambiguity, illegal state, or gap not already in the ledger. I found none. Detail below.

## What I checked

### Prose coherence and single-source, no drift
- The three rendered copies of `AGENTS.md` (`AGENTS.md`, `pack/AGENTS.md`, `.agents/AGENTS.reference.md`) carry byte-identical edits across all five touched paragraphs (Design explorations, Findings files, Writer isolation, the who-isolates single-source paragraph, Worktree lifecycle, Preflight). No copy was left on the old wording.
- `{{isolation_policy}}` single source (`src/isolation_policy.rs::ISOLATION_POLICY_FRAGMENT`) is rewritten to "every spawned agent runs isolated ... the writers and the read-only reviewers, triager, and explorers alike, because even a findings or an exploration file is a write", with the orchestrator-integration carve-out intact. Its module doc, the rustdoc, and both drift-guard tests were updated to match. This is the one place who-isolates is stated, projected into the rendered views; no divergent restatement remains.

### Emitted tool instructions match the AGENTS.md prose
- Orchestrator prompt: "Spawn every agent ... in the strongest isolation ... and merge each agent's output onto main yourself"; the batch-merge-after-all-finish rule and the explicit review-round ordering (reviewers finish -> batch-merge -> triager in its own worktree reads merged findings -> merge verdict) all match the AGENTS.md Worktree lifecycle paragraph.
- Reviewer prompt: writes findings "inside your own worktree ... the orchestrator merges your findings onto main". Matches.
- Triager prompt: reads reviewers' files "under `docs/plans/<task>.reviews/`, which the orchestrator has merged onto main from the reviewers' worktrees" and writes verdicts in its own worktree. Consistent with the ordering (triager is spawned after the batch-merge, off a main that already carries the merged findings).
- `pack/isolation-guidance.md`: the former "Read-only agents ... need no isolation and run without a container or a worktree" sentence is replaced by "Every spawned agent, including the reviewers, the triager, and the explorers, runs under the resolved isolation tier ... and the orchestrator merges their outputs onto main". No stale carve-out survives in the pack asset body.

### Illegal-state / contradiction sweep
- The former self-contradiction (explorers called "writer agents" in Design explorations vs. listed among "read-only agents needing no worktree" in Worktree lifecycle) is resolved consistently: explorers are now classified read-only w.r.t. the reviewed product but still isolate like every spawned agent. Both paragraphs now agree.
- "read-only vs writer" is reframed as authority over the reviewed product, not isolation; the Writer isolation paragraph states this explicitly and the fragment, prompts, and success criterion all use the reframed sense consistently. No residual place ties "read-only" to "no isolation".
- The driver (`src/next.rs::spawns_writer`) is deliberately left unchanged; the code comment and the content-pin test `the_reviewer_states_carry_no_isolation_reminder` are rewritten to say the reviewer states still carry no reminder BY CURRENT DESIGN and that widening it is a separate open decision (`Q-62`). This is a documented, captured open question, not a prose/driver contradiction: the isolation instruction for read-only agents is carried by their role prompts and by AGENTS.md, and the driver reminder's tier-echo is writer-specific. Coherent.
- `Q-61` recorded `decided`/`folded_into = "uniform-agent-isolation"` with `receipt = "Q-61"`; `Q-62` recorded `open`. The Step Detail, success criterion, and `.plan.toml` step/question rows are mutually consistent, and `validate --workflow` confirms the receipt (W4) and the round-log invariants hold.

### Ledger residuals (NOT re-raised)
- The two consciously-accepted LOW framing residuals (the File-safety baseline intro still scoped to "writer agent" at `AGENTS.md:75`, and the `pack/isolation-guidance.md` "writer"-only example framing at lines 3/30/37) remain as accepted. I found no NEW evidence that the acceptance was wrong, so per the no-relitigation rule I do not re-raise them. The line-3 and line-37 example framing is unchanged; the only line-34 sentence that carried the stale read-only carve-out was correctly fixed in this diff.

## Mechanical checks (run in worktree `ui-review-e`, all green)

`cargo run -- validate --source docs/plans/agent-scaffold.plan.toml`:
```
docs/metrics/workflow.jsonl: 159 records, valid
docs/plans/agent-scaffold.plan.toml: 69 steps, 62 questions, valid
```

`cargo run -- validate --workflow --source docs/plans/agent-scaffold.plan.toml`:
```
docs/metrics/workflow.jsonl: 159 records, valid
docs/plans/agent-scaffold.plan.toml: 69 steps, 62 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

`cargo run -- render --check docs/plans/agent-scaffold.plan.toml`:
```
docs/plans/agent-scaffold.plan.toml: up to date
```

`cargo test`:
```
running 342 tests
test result: ok. 342 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
(plus doc/integration suites: 1, 3, 1, 2 tests, all ok)
```

The isolation-policy byte-guard and content-pin tests (`the_fragment_states_the_uniform_isolation_rule`, the AGENTS.reference byte-compare, and the `next.rs` `the_reviewer_states_carry_no_isolation_reminder` pin) are inside the passing suite.

## Severity summary
- critical: none.
- high: none.
- medium: none.
- low: none new (the two ledger LOWs remain accepted; no new evidence against that acceptance).
