# Backlog-clearing review: mechanical / consistency / prose lens

Reviewer B. Branch `plan/backlog-clearing`, commit `aec4144`, base `c8308ef`.
Lens: mechanical correctness, source-vs-generated consistency, prose rules, Markdown structure, validation.

## Verdict

No valid findings at high/medium. One low-severity stylistic observation recorded below for the triager to judge.

## What I checked

SCOPE. `git diff --stat c8308ef..aec4144` shows exactly the five expected files and nothing else: `pack/AGENTS.md`, `pack/instrument.md`, `pack/isolation-guidance.md` (sources) plus the regenerated `AGENTS.md` and `.agents/AGENTS.reference.md`. No plan TOML, step sidecar, ledger, metrics JSONL, or Rust source is touched. Scope is clean.

SOURCE-vs-GENERATED. Verified by the byte-guard / drift-guard tests in `cargo test`, all passing:
- `workflow_spec::tests::the_committed_scaffold_carries_the_generated_fragment` ... ok
- `isolation_policy::tests::the_committed_scaffold_carries_the_isolation_policy_fragment` ... ok
- `tests::generated_agents_has_principles_and_no_placeholder` ... ok
- `plan::render::tests::a_one_byte_hand_edit_of_the_generated_file_is_a_mismatch` ... ok
The pack edits (`pack/AGENTS.md`, and the `{{instrument}}` / isolation fragments) are mirrored in the generated `AGENTS.md` and `.agents/AGENTS.reference.md`; no source edit is unmirrored and no generated edit lacks a source. The generated diffs match the source diffs hunk-for-hunk.

PROSE RULES. No em-dashes, en-dashes, double-hyphens-as-dashes, unicode, or emoji in any added line: `git diff | grep '^+' | grep -P '[^\x00-\x7F]'` returns nothing across both the pack sources and the generated files. No AI filler in added text (scanned for "worth noting", "seamless", "robust", "leverage", "delve", "furthermore", "moreover", "utilize", "streamline"; none present). No hard-wrapping: every edit is a whole-line replacement inside an existing paragraph or bullet; no manual mid-sentence line break was introduced. The changed sentences read cleanly and grammatically in their paragraphs:
- Phase 5 (Accept): the currency-check rewrite ("The reviewer verifies that currency ... any staleness it finds is a shortfall that routes back to implementation, where the implementer updates the docs and prompts the change made stale, the same path the shortfalls below take") reads coherently and its "shortfalls below" back-reference resolves to the same paragraph's shortfall-routing sentence.
- File safety intro: broadening "Every writer agent's damage" -> "Every spawned agent's damage" and "running writers under isolation" -> "running an agent under isolation" is grammatical and consistent with the Writer-isolation rule, which already covers every spawned agent.
- Prose-formatting paragraph: the added clause "and leaves incidental reformatting to the orchestrator" matches the identical clause already present in the "Format only your own files" bullet it back-references, so the two are now consistent.
- instrument.md decision/baseline hunks: the `[meta].w4_baseline` / `type: "baseline"` two-flow description and the new `task`-carries-`folded_into` convention sentence read cleanly.

MARKDOWN STRUCTURE. All edits stay inside existing bullets/paragraphs. No numbered list, bullet nesting, or code-span was broken. Backtick balance verified on every added pack line (each line has an even backtick count). Code spans in the added instrument.md text (`[meta].w4_baseline`, `[meta].primary = "toml"`, `type: "baseline"`, `q_id`, `task`, `folded_into`) are all well-formed.

FACTUAL ACCURACY of the instrument.md claims (a consistency check against the code and plan):
- `[meta].w4_baseline` exists in code (`src/plan/source.rs:109`) and in the plan TOML (`docs/plans/agent-scaffold.plan.toml:3`, value `Q-44`). Confirmed.
- `[meta].primary` exists (`src/plan/source.rs:112`, `is_toml_primary` at :412) and the plan sets `primary = "toml"` (`:4`). So `[meta].primary = "toml"` is accurate.
- Q-46 is the decision receipt that moved waivers/baseline to TOML (plan `:1277` records `q_id:"Q-46"` as that decision), so "the TOML field superseded per Q-46" is accurate. The plan even carries a step (`:970`) titled to reconcile the AGENTS.md W4-baseline description with the live mechanism, confirming this edit is the intended backlog item.

VALIDATION (branch tree checked out into worktree, run, then restored):
- `cargo test`: 342 passed, 0 failed (lib/bin suite), plus integration suites `checks_staged_hook_env` 1/0, `scaffold_precommit_hook` 3/0, `validate_toml_primary_skips_markdown_plan` 1/0, `validate_workflow_toml_source_needs_no_plan` 2/0. All green, drift guards included.
- `validate --source docs/plans/agent-scaffold.plan.toml --workflow`: `165 records, valid`; `75 steps, 62 questions, valid`; `workflow invariants hold`. Matches the expected 75/62/165.
- `render --check docs/plans/agent-scaffold.plan.toml`: `up to date`.
After running, the branch checkout was reverted with `git checkout HEAD -- pack .agents AGENTS.md`; `git status` shows only this findings file.

## Low-severity observation

### L1. Two names for the same flow in adjacent instrument.md hunks
- Severity: low.
- Evidence: in the `type: "decision"` bullet the flow is called "the TOML-sourced flow" and "the Markdown-sourced flow"; in the `type: "baseline"` bullet (next line) it is called "the plan-TOML flow (`[meta].primary = "toml"`)" and "the Markdown-sourced flow".
- Why it could matter: two labels for the same TOML-primary flow sit one line apart. Both are plain descriptive prose (not defined terms) and both are clear in context, so this is a stylistic nit, not a correctness or consistency defect. Recorded only so the triager can decide whether to unify the wording; safe to dismiss.
