# Review: driver-output-generation-inc2 (Reviewer 2, Sonnet)

Lens: output-contract determinism, fragment reword dual-correctness, scope discipline, style, judgment calls.
Diff: main `17dce12`..HEAD `36ed42a`; commits `26cc178` (fragment reword) + `36ed42a` (next.rs consumer).
Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/dog-inc2`.

## Findings

### D2s-1 (low) -- Whitespace reformats in `run_resume` are out of stated increment scope

`src/main.rs:1093-1094` and `1102-1105` (in the committed diff from `36ed42a`): two line-reflows in `run_resume` collapse a two-line `let ledger_path =` binding and a four-line `println!` arm into single lines. The stash@{0} description says it captures nix-fmt reformats of OTHER files (docs/ledger, source.rs), so these reformats are NOT in the stash; they are committed in the diff. The stated scope for `main.rs` is principle-threading into `NextInputs`/`run_next`; `run_resume` is outside that. No behavior change, purely whitespace. The criteria say "a stray main.rs change is medium," but the change is zero-semantic whitespace; calling it cosmetic (low) is the more accurate read. A stricter reading of the severity rubric would put it at medium.

No other findings.

## Per-check verdicts

### 1. Output determinism

CLEAN. Two successive runs of `cargo run -- next --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl` produced byte-identical output (confirmed with `diff`). Sources of variance audited: plan and metrics are stable file reads; `isolation_tier` is echoed from the CLI flag (or a literal `"unknown"` constant); the `context` map is a `BTreeMap` (deterministic iteration order, `src/next.rs:155`); no wall-clock, random, or environment-dependent values. The `principles` lookup is a slice scan (stable order, from the parsed plan). The golden tests (`golden_human_text`, `golden_json`) are real `assert_eq!` byte-compares against committed string literals; they were updated in `36ed42a` to match the de-numbered reminders (confirmed in the diff: old "Principle 5: staff..." and "Principle 7: cite..." replaced with de-numbered forms, both in `GOLDEN_HUMAN` and `GOLDEN_JSON`). Both golden tests pass.

### 2. Fragment reword dual-correctness

CLEAN. In `AGENTS.md` context (line 91), the fragment sits immediately after the tier list (lines 85-87) within the "Writer isolation (capability-tiered)" section. "Per the capability-tiered tier order in the Writer isolation rule" reads as a backward reference to that tier list; it is accurate because the tier order IS defined in the Writer isolation section. The section heading is "Writer isolation (capability-tiered)" and "the Writer isolation rule" is the accepted shorthand already used by the Worktree lifecycle paragraph (line 93: "the Writer isolation rule above"), so the reference is unambiguous. Standalone in the driver reminder (no AGENTS.md context), "in the Writer isolation rule" is a self-locating name reference; it tells the orchestrator which AGENTS.md section to consult without dangling "above." Both contexts read correctly. `render --check docs/plans/agent-scaffold.plan.toml` exits 0. Both `the_committed_scaffold_carries_the_isolation_policy_fragment` and `the_fragment_states_the_writer_classification` pass.

Minor cosmetic observation (not a finding): AGENTS.md has no section literally titled "Writer isolation rule"; the heading is "Writer isolation (capability-tiered)." A reader searching for an exact-match section title would need to scan briefly. This is not a defect because the Worktree lifecycle paragraph (line 93) already uses the same shorthand, establishing it as the document's own cross-reference name.

### 3. Scope discipline

CLEAN (with the whitespace note in D2s-1). Changed files: `.agents/AGENTS.reference.md`, `AGENTS.md`, `src/isolation_policy.rs`, `src/main.rs`, `src/next.rs`. All in scope. `docs/plans/agent-scaffold.plan.toml`, `docs/plans/agent-scaffold.ledger.md`, and `docs/metrics/workflow.jsonl` are unchanged (confirmed by `git diff main..HEAD` on those paths). stash@{0} carries nix-fmt reformats of docs/ledger and source.rs from `just scaffold-self`, correctly not committed. Convergence/verdict logic (`derive_in_progress_state`, `has_risk_class_conflict`, `select_active_increment`, `peak_consecutive_clean`) is unchanged in `next.rs`. The `spawns_writer` narrowing from `{ReadyToPlan, AwaitingFirstReview, AwaitingFixes, AwaitingReviewers}` to `{ReadyToPlan, AwaitingFixes}` is in-scope (D-d). The metrics struct reflow (`MetricsSummary { records }` expanded to multi-line) is whitespace-only, no behavior change.

### 4. Judgment calls

(a) `ESCALATE_PRINCIPLE_NAME = "Ground decisions in evidence"` (plan principle 6, `src/next.rs:73`). SOUND. The principle name maps directly to the escalate context: when the round cap is hit, the human's decision about how to proceed must be grounded in evidence (the review findings, risk, trade-offs). The principle text's second clause ("If the candidates are exhausted, raise the impasse for a decision rather than forcing through an unvalidated approach") applies directly to an exhausted-loop escalation. The first clause (proof-of-concept validation) is less on-point but does not mislead. No other plan principle (from the 8 in `docs/plans/agent-scaffold.plan.toml`) is more apt: P5 (illegal states unrepresentable), P7 (reproducible), and P8 (structured data first) are engineering principles, not decision-quality principles. The name-based lookup is immune to renumbering (the design's stated reason), and the graceful degradation when the principle is absent (emit the originated imperative alone) is correct. Verdict: the choice is sound and well-implemented.

(b) One-line reminder shape: "Writer isolation (resolved tier: ...). [fragment]" (`src/next.rs:817-822`). ACCEPTABLE. The output is a single list item carrying the lead plus the full `ISOLATION_POLICY_FRAGMENT` (~550 characters). In the human renderer this appears as one long `- ` bullet under `reminders:`. It is dense but structurally valid. The design explicitly chose to emit the fragment verbatim for drift-free guarantees; splitting it would require relaxing the `reminders_carry_the_isolation_fragment` test (which matches the whole constant) and risk the fragment drifting between the reminder and the AGENTS.md slot. The shape reflects the deliberate trade-off of length for correctness. Verdict: acceptable given the design's stated priority; a reader can scan past it, and the density is bounded by the single constant.

### 5. Regression + style

CLEAN. `cargo test --bins -- --test-threads=1`: 342 passed, 0 failed. `just clippy`: no warnings. No em-dashes, en-dashes, or non-ASCII characters in the diff (confirmed by grep). `#[cfg_attr(not(test), allow(dead_code))]` at `src/next.rs:198` (`LoopState::Done`) is a cfg-split case (constructed in tests, unreachable in the release binary) where `allow` is the correct attribute per the project rules, not `expect`. No prose hard-wrapping in new code or strings.

## Summary

One low-severity finding: D2s-1 -- two whitespace-only line-reflows in `run_resume` in `src/main.rs` are committed outside the increment's stated scope (principle-threading into `NextInputs`/`run_next`). Zero behavior impact.

Fragment dual-correctness: CLEAN -- "in the Writer isolation rule" reads accurately in AGENTS.md (backward reference to the section containing the tier list) and is self-locating standalone in the driver reminder (names the rule so the orchestrator knows where to look). No dangling "above."

Judgment call A (escalate principle): SOUND -- "Ground decisions in evidence" is the apt plan principle by name; the lookup-by-name implementation is immune to renumbering; graceful degradation is correct.

Judgment call B (one-line reminder shape): ACCEPTABLE -- dense but structurally valid; the shape is the direct consequence of emitting the verbatim fragment for drift-free guarantees, which the design prioritizes.

All tests pass, clippy clean, output deterministic.
