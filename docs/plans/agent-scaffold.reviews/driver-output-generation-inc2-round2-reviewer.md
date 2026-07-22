# driver-output-generation inc2 - Round 2 re-review (fresh adversarial reviewer)

Scope: diff `main (a26d904)..HEAD (fb2ccf6)` on `impl/dog-inc2`, read/tested in the
worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/dog-inc2`.
Fix under review: `fb2ccf6` (D2s-1 whitespace revert) atop `909588c` / `e8cb9e6`.

## Verdict: CLEAN

No findings. This is the first potential clean round after the new_valid round; I found
no defects with fresh eyes and did not rubber-stamp. D2-1 (no full golden for the writer
block) was ruled VALID-NON-ACTIONABLE and is NOT re-raised.

## What I probed

D2s-1 FIX (item 1). `git diff main..HEAD -- src/main.rs` now shows ONLY the intended
`run_next` principle-threading: the tuple gains `principles`, `source_plan.principles
.clone()`, `Vec::new()` on both degrade paths, and `principles: &principles` into
`NextInputs`. The two `run_resume` reflows (the `let ledger_path` binding and the `None`
`println!` arm) are gone; `fb2ccf6` is a pure 6-add/2-remove whitespace revert restoring
main's multi-line forms, and introduces no new stray reformatting. Confirmed the current
`run_resume` body matches main.

NO REGRESSION from the revert (item 2). The revert is whitespace-only; behaviour
unchanged. `just clippy` clean; `cargo test --bins -- --test-threads=1` = 342 passed,
0 failed.

CHANGE A (item 3A). `spawns_writer` = `matches!(self, ReadyToPlan | AwaitingFixes)` only
(src/next.rs:276). Tests confirm the isolation fragment/resolve-note are ABSENT at both
reviewer states (`the_reviewer_states_carry_no_isolation_reminder`, both tiers) and
PRESENT at both writer states for known AND unknown tier
(`a_writer_state_emits_the_isolation_fragment_for_any_tier`).

CHANGE B (item 3B). The driver emits `ISOLATION_POLICY_FRAGMENT` verbatim via
`principle_reminders.push(format!("{lead} {ISOLATION_POLICY_FRAGMENT}"))` (single source,
no paraphrase). The reword ("per the capability-tiered tier order in the Writer isolation
rule") is byte-identical across the const (src/isolation_policy.rs:33), AGENTS.md, and
.agents/AGENTS.reference.md, so it reads correctly both inline and standalone. Byte-guard
+ content-pin + build-path tests pass (`the_committed_scaffold_carries_the_isolation
_policy_fragment`, `the_fragment_states_the_writer_classification`, `isolation_policy_slot
_renders_the_generated_fragment`). `render --check` = up to date.

CHANGE C (item 3C). No base reminder carries a numeric "Principle N" citation
(`no_terse_reminder_carries_a_numeric_principle_citation` sweeps all 9 states). The
escalate reminder projects a REAL plan principle by NAME: `ESCALATE_PRINCIPLE_NAME =
"Ground decisions in evidence"` matches an actual `[[principle]]` in the live plan
(docs/plans/agent-scaffold.plan.toml:1260), and it degrades to the originated imperative
alone when absent (Markdown/missing), never a dangling number
(`the_escalate_reminder_projects_a_real_plan_principle_by_name`,
`the_escalate_reminder_degrades_when_the_principle_is_absent`).

VERDICT-PRESERVATION (item 4, safety-critical): UNCHANGED. `spawns_writer` is consumed
ONLY at src/next.rs:816 and `projected_principle_reminder` ONLY at src/next.rs:810, both
inside `build_instruction` (reminder assembly). `principles` is threaded solely into that
projection. Neither touches `LoopState` selection, convergence, `valid_transitions`, or
the round/streak verdict. `next_agrees_with_w3` differential and the transition-row tests
pass.

OUTPUT DETERMINISM (item 5). `next` run twice against the live plan is byte-identical;
goldens are genuine byte-compares reflecting the new de-numbered output
(`golden_human_text`, `golden_json` pass). Live `validate`, `validate --workflow`, and
`render --check` all exit 0.

STYLE (item 6). No non-ASCII / em-dash / en-dash / emoji in any added line; no `#[allow]`
introduced.

Byte-guard verdict: intact. Verdict-preservation verdict: intact.
