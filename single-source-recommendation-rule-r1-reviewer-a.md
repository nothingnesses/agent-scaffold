# Review: single-source-recommendation-rule (Q-60), reviewer A

ZERO FINDINGS

## Verdict

The change faithfully mirrors the `isolation_policy.rs` precedent. Correctness, decision-fidelity, and build mechanics all hold. Nothing to fix.

## Evidence checked

1. Slot rendering (both targets, both generated files):
   - `grep -c "{{recommendation_rule}}"`: `pack/AGENTS.md` = 2 (source template), `AGENTS.md` = 0, `.agents/AGENTS.reference.md` = 0. No raw placeholder remains in the committed generated files.
   - `grep -c "reasoning judged against the plan's Project Principles by name"` (phrase unique to the fragment): `AGENTS.md` = 2, `.agents/AGENTS.reference.md` = 2, `pack/AGENTS.md` = 0. The fragment appears exactly twice in each generated file: the human-input-contract paragraph and the Preflight restatement.
   - `render` (`src/manifest.rs:337`) uses `str::replace`, which substitutes EVERY occurrence, so both slots fill from the one fragment.

2. Single-sourcing integrity:
   - The fragment is defined once: `RECOMMENDATION_RULE_FRAGMENT` at `src/recommendation_rule.rs:34`. Grep for the fragment's opening prose across `src/` returns only that one definition. No hand-copied duplicate.
   - Registered in `RESERVED_VARS` (`src/manifest.rs:199-206`) and inserted into the `builtin` var map in `build_assets` (`src/main.rs:110-113`), exactly parallel to `isolation_policy`.

3. Byte-guard tests are genuine, not tautologies:
   - `recommendation_rule::tests::the_fragment_states_the_recommendation_rule` asserts three distinct content substrings (single-source claim, options/trade-offs/recommendation/reasoning, Principles-by-name). A reword of the rule fails it. Not a tautology; it pins the rule's semantics.
   - `recommendation_rule::tests::the_committed_scaffold_carries_the_recommendation_rule_fragment` byte-compares the const against `include_str!` of the committed `AGENTS.md` and `.agents/AGENTS.reference.md`, catching a hand edit or a stale (un-rescaffolded) fragment.
   - `main.rs tests::recommendation_rule_slot_renders_the_generated_fragment` calls `build_assets` and asserts the placeholder is gone and the fragment present in both dest files, genuinely exercising the render path.
   - `agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render` (the re-render oracle) passes.

4. Regeneration correctness:
   - The drift oracle (`src/agents_md_drift.rs`, `build_assets` re-render normalized-compared to committed) passes, so the committed `AGENTS.md` + `.agents/AGENTS.reference.md` are exactly what `build_assets` produces.
   - No other slot was corrupted or dropped: `isolation_policy`, `workflow_control`, and principle-render tests all pass, and `grep -c "{{isolation_policy}}"` = 0 in both generated files.

5. Decision fidelity to Q-60:
   - Single canonical source (`RECOMMENDATION_RULE_FRAGMENT`) rendered into both the human-input-contract location and the Preflight restatement. Neither a bare pointer nor a hand-copied duplicate. The fragment names itself ("The human-input contract's presentation format ...") so it reads standalone in the Preflight, mirroring how `ISOLATION_POLICY_FRAGMENT` names the "Writer isolation rule." Confirmed.

## Command results

- `cargo test`: 351 passed, 0 failed in the lib target; all integration test binaries pass (checks_staged_hook_env 1/1, scaffold_precommit_hook 3/3, validate_* 1/1 + 2/2). The two new `recommendation_rule` tests, the new `main.rs` slot test, and the four `agents_md_drift` tests all pass.
- `cargo clippy --all-targets -- -D warnings`: Finished with no warnings.
