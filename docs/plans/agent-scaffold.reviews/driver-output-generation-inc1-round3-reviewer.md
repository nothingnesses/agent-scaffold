## Round 3 review: driver-output-generation-inc1

Reviewer: independent, fresh-sampling confirmation (Round 3 convergence check).
Branch: impl/dog-inc1 (main 05d88c5). Artifact unchanged since Round 2.

### Verdict: CLEAN

No findings. All probes passed. Details below.

---

### Probe 1: Drift-guard non-vacuity

`the_committed_scaffold_carries_the_isolation_policy_fragment` (src/isolation_policy.rs):
- Direction 1 (hand-edit of committed output): the test uses `include_str!("../AGENTS.md")` baked at compile time. Editing AGENTS.md to alter the fragment changes the baked bytes, so `COMMITTED_AGENTS.contains(ISOLATION_POLICY_FRAGMENT)` fails. Non-vacuous.
- Direction 2 (source reword without re-scaffold): changing `ISOLATION_POLICY_FRAGMENT` in isolation_policy.rs while leaving AGENTS.md stale means the include_str! still carries the old text, which no longer matches the new const. Test fails. Non-vacuous.

`isolation_policy_slot_renders_the_generated_fragment` (src/main.rs): explicitly calls `build_assets()` with the builtin manifest and default pack, inspects the returned asset structs. Genuinely exercises the build path, not just `include_str!` bytes. Non-vacuous.

`the_fragment_states_the_writer_classification` (src/isolation_policy.rs): asserts two specific substrings ("A spawned planner is a writer", "distinct from the orchestrator's own integration-level edits on main") against the const. These are real content pins: a reword that drops either phrase fails the test. Both substrings are present in the current fragment. Non-vacuous.

### Probe 2: Fragment content

`ISOLATION_POLICY_FRAGMENT` (src/isolation_policy.rs line 33): tier-agnostic confirmed. The text says "per the capability-tiered tier order above" with no hardcoded tier names ("worktree", "branch"). It references, not restates, the tier order. Writer classification is correct: spawned planner is a writer, design/exploration writers are writers, distinct from orchestrator integration-level edits (step-status flips, increment declarations, round records, ledger anchors). Single source for this policy.

### Probe 3: Preamble

pack/prompts/orchestrator.md addition: states per-spawn facts only (role, writer status, resolved tier, worktree path). Explicitly defers to "the Writer isolation rule in AGENTS.md rather than restating it." No restatement of the classification policy or tier order. "treat its absence as a defect" framing present.

Generated .agents/prompts/orchestrator.md matches the pack source identically (confirmed via diff).

### Probe 4: Generation wiring and scaffold consistency

`{{isolation_policy}}` slot appears in pack/AGENTS.md. Generated AGENTS.md and .agents/AGENTS.reference.md both carry the expanded fragment text. The slot is absent from the generated files (substitution confirmed by `render --check` reporting "up to date" and by the slot test). src/manifest.rs adds "isolation_policy" to RESERVED_VARS, blocking pack override.

### Probe 5: Scope

src/next.rs: untouched (no diff). plan.toml, ledger, metrics: untouched (no diff). src/main.rs committed diff is exactly the intended changes: one-line doc comment update to document the new slot, the `builtin.insert` for isolation_policy, and the new slot test. One removed line (`/// overrides supplies the --var values...`) is a doc comment wrap to accommodate the new preceding slot documentation, not a stray reformat.

### Probe 6: Style and regression

No em-dashes, en-dashes, or unicode symbols found in new or modified files. No `#[allow(...)]` without cfg-split justification in new code. `just clippy` clean. Full test suite (337 tests) passes serial (`--test-threads=1`). `validate --source`, `validate --workflow`, and `render --check` all pass.
