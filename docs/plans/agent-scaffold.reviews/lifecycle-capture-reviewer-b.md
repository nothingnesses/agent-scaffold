# Reviewer B (mechanical / schema lens) findings: `plan/lifecycle-capture` (987a253, range 245200e..987a253)

Lens: mechanical and schema correctness, independently re-verified. Not trusting the writer's report.

## Verdict: No findings.

Every mechanical and schema check passed. Evidence below.

## Toolchain command outputs (run in the worktree, tails)

### 1. `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml`
```
docs/metrics/workflow.jsonl: 157 records, valid
docs/plans/agent-scaffold.plan.toml: 68 steps, 60 questions, valid
```
Matches the expected 68 steps / 60 questions. Passes.

### 2. `cargo run -- validate --workflow --source docs/plans/agent-scaffold.plan.toml`
```
docs/metrics/workflow.jsonl: 157 records, valid
docs/plans/agent-scaffold.plan.toml: 68 steps, 60 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```
Passes. W4 reasoning confirmed: Q-57 is `decided` (was `open` at 245200e) and sits after the `w4_baseline` (Q-44), so W4 requires a matching `type:"decision"` receipt with `q_id:"Q-57"`. That receipt is the single line appended to `docs/metrics/workflow.jsonl` (see check 4). W4 passes because the receipt exists; I confirmed the validator does not silently pass without it (the appended line is the only new record and it carries `q_id:"Q-57"`).

### 3. `cargo run -- render --check docs/plans/agent-scaffold.plan.toml`
```
docs/plans/agent-scaffold.plan.toml: up to date
```
The generated `docs/plans/agent-scaffold.md` is not stale.

### 4. `cargo test`
All test binaries pass (0 failed across the suite; sample tails: `checks_staged_runs_under_a_hook_environment ... ok`; the three `scaffold_precommit_hook` tests ok; `validate_toml_primary_skips_markdown_plan` ok; both `validate_workflow_toml_source_needs_no_plan` tests ok). The byte-guard / `include_str!` tests in `src/isolation_policy.rs`, `src/workflow_spec.rs`, `src/pack.rs`, `src/plan/render.rs` still hold; none were weakened. `pack/AGENTS.md` was edited and `AGENTS.md` regenerated, and the guards still pass.

## Schema and integrity checks (all clean)

- APPEND-ONLY integrity (`git diff 245200e..987a253 -- docs/metrics/workflow.jsonl`): exactly one `+` line, zero `-` lines. No existing line modified. The appended line is valid JSON (jq parsed it) with all required decision fields present: `type="decision"`, `task="lifecycle-capture"`, `q_id="Q-57"`, `options` is a non-empty array (length 3), `recommendation` present, `chosen` present, plus `ts`. Membership verified via jq: `rec_in_opts:true` and `chosen_in_opts:true`, so both `recommendation` ("Format the code and exclude the prose docs from the formatter") and `chosen` ("Change policy: accept incidental reflow, stop reverting and stashing it") are members of `options`. The receipt's `chosen`/`recommendation` are also consistent with the plan's Q-57 prose (accept incidental reflow, chosen over the format-and-exclude recommendation).

- `[[step]] formatter-reflow-convention` has a unique `order`: `order = 68` appears only at line 898 (the new step). All 68 steps have exactly 68 `order` lines with no duplicate values (`uniq -d` on the sorted order values is empty); 68 is the max, no collision with any existing step.

- Q-59 and Q-60 are well-formed `[[question]]` entries with valid `status` values: Q-59 `status = "exploring"`, Q-60 `status = "open"`. Both are in the accepted status vocabulary. Q-57 correctly flipped `open -> decided` and gained `folded_into = "formatter-reflow-convention"` and `receipt = "Q-57"`.

- `"lifecycle-capture"` is present in `[meta].orphan_tasks` (added between `consolidate-plan` and `metrics-fields`, alphabetical order preserved).

- New empty question sidecars `Q-59.md` and `Q-60.md` are consistent with the existing convention: all 60 sidecars under `docs/plans/agent-scaffold.questions/` are empty (60 total, 60 empty). Both new files are git-tracked.

- Regeneration really happened, no manual drift: the new convention paragraph ("Incidental formatter reflow is accepted, not a finding...") is present in all three of `pack/AGENTS.md`, `AGENTS.md`, and `.agents/AGENTS.reference.md`, and `AGENTS.md` is byte-identical to `.agents/AGENTS.reference.md`. I re-ran the raw scaffold (`cargo run -- scaffold --output-dir . --write --force --principles default --instrument`, the scaffold-self recipe minus the trailing `nix fmt`) in the worktree: `git status --short` was empty afterward, i.e. the committed `AGENTS.md`, `.agents/AGENTS.reference.md`, and all 30 scaffolded files match exactly what `scaffold` produces from the pack. No hand-edited drift, and the worktree was left clean by the regeneration (no restoration needed).

## Notes

- Line-length / prose-wrapping and incidental formatter reflow were treated as non-findings per instruction. The change itself is the policy that codifies exactly that.
