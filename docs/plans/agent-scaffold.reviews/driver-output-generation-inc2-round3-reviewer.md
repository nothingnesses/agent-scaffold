# Round 3 review: driver-output-generation-inc2

Reviewer: independent fresh-sampling confirmation (Round 3).
Branch: `impl/dog-inc2`. Main base: `ad2379e`. Branch HEAD unchanged since Round 2.
Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/dog-inc2`.

## Probes run

- `just clippy`: clean (no warnings).
- `cargo test --bins -- --test-threads=1`: 342 passed, 0 failed.
- `cargo run -- next --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl` x2: byte-identical (md5 confirmed).
- `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml`: 153 records, 67 steps, 58 questions, valid.
- `cargo run -- validate --workflow --source docs/plans/agent-scaffold.plan.toml`: workflow invariants hold.
- `cargo run -- render --check docs/plans/agent-scaffold.plan.toml`: up to date.
- Diff reviewed: `git diff main..HEAD --name-only` returns exactly `{.agents/AGENTS.reference.md, AGENTS.md, src/isolation_policy.rs, src/main.rs, src/next.rs}`. Plan/ledger/metrics untouched (zero diff).

## Verdict-preservation (safety-critical) - CLEAN

`spawns_writer()` is consumed exclusively in `build_instruction()` (line 816 of `src/next.rs`), for reminder assembly. `projected_principle_reminder()` and `ISOLATION_POLICY_FRAGMENT` are likewise consumed only in `build_instruction()`. Neither touches `select_active_loop()`, `build_in_progress_loop()`, the streak/convergence arithmetic, or any state-transition logic. `next_agrees_with_w3` passes. All transition-row tests pass. No path found where the three changes leak into verdict, state selection, convergence, or the streak.

## Change B: ISOLATION_POLICY_FRAGMENT at writer states - CLEAN

The fragment is emitted verbatim via the shared `ISOLATION_POLICY_FRAGMENT` const (not a hand-copy). The byte-guard test (`the_committed_scaffold_carries_the_isolation_policy_fragment`) passes, confirming AGENTS.md and AGENTS.reference.md carry exactly the same bytes. The content-pin test (`the_fragment_states_the_writer_classification`) also passes.

The "in the Writer isolation rule" reword reads correctly in AGENTS.md (the fragment sits inside the "Writer isolation (capability-tiered)" section at line 83, so the name resolves to the current section) and standalone in the driver reminder (the name tells the reader to consult AGENTS.md's "Writer isolation" section for the tier order, which is a stable pointer, unlike the old "above").

`a_writer_state_emits_the_isolation_fragment_for_any_tier` confirms the fragment fires unconditionally at both `ReadyToPlan` and `AwaitingFixes`, with both known and unknown tier. `the_reviewer_states_carry_no_isolation_reminder` confirms `AwaitingFirstReview` and `AwaitingReviewers` carry no fragment for either tier. `a_writer_state_echoes_the_resolved_tier_in_the_isolation_reminder` and `an_unknown_tier_adds_the_resolve_note_at_a_writer_state` both pass. All inc1 byte-guard and build-path slot tests still pass.

## Change C: de-numbered reminders + escalate principle projection - CLEAN

`no_terse_reminder_carries_a_numeric_principle_citation` sweeps all nine `LoopState` variants' `base_reminders()` for "Principle " and passes. Grepping the emitted reminder strings (const values and format! calls) in `src/next.rs` confirms no "Principle N" string reaches output; the "Principle N" occurrences found by grep are all in code comments only.

The name lookup in `projected_principle_reminder()` uses `principle.name == ESCALATE_PRINCIPLE_NAME` where `ESCALATE_PRINCIPLE_NAME = "Ground decisions in evidence"`. This is a name-based find, not a fixed index, so it is immune to renumbering. `the_escalate_reminder_projects_a_real_plan_principle_by_name` confirms the projected reminder carries the real name, real text, and the plan's own number (not a hardcoded AGENTS.md number), and uses a fixture principle numbered 6 (not 1), ruling out accidental index coupling. `the_escalate_reminder_degrades_when_the_principle_is_absent` confirms no reminder names the principle when the plan carries none (empty principles slice, as for Markdown source), and the base contract reminder remains present. No dangling "Principle N" emitted in the degraded path.

## Determinism - CLEAN

Two sequential runs of `cargo run -- next --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl` produce byte-identical stdout (md5 `0698ab1be42f27453fd2841dc2af9e0e` on both runs).

## Scope - ONE LOW FINDING

Plan/ledger/metrics: untouched (zero diff). No run_resume hunks (D2s-1 is fixed by the `da2eb17` style-restore commit, which is present on the branch). The changed source files are exactly the three intended (`src/next.rs`, `src/main.rs`, `src/isolation_policy.rs`) plus `AGENTS.md` and `.agents/AGENTS.reference.md` (both reflecting the fragment reword).

The following stray rustfmt reformattings remain in `src/next.rs`, in functions whose logic is not changed by this increment. These are distinct from D2s-1 (which was in `run_resume` in `src/main.rs` and was fixed):

### D2r3-1 (low) - stray reformatting in next.rs, four unchanged functions

`src/next.rs`, multiple locations.

Four functions that this increment does not logically modify contain incidental rustfmt reformatting that differs from main's form:

- `project()`: the `metrics:` struct field was expanded from a single line (`metrics: inputs.metrics_records.map(|records| MetricsSummary { records }),`) to a three-line form.
- `select_active_loop()`: the first `if let Some(step) =\n    steps.iter()...` was reformatted to a method-chained multi-line form.
- `build_context()`: the `review_findings` three-line `format!()` call was collapsed to a two-line assignment.
- `extract_resume_state()`: the final four-line `if in_section { ... } else { None }` block was collapsed to a single line.

These are semantically equivalent, all 342 tests pass, and they have no impact on verdict or output. They add noise to the diff of functions that readers expect to be unchanged. Severity is low: the run_resume fix in `da2eb17` demonstrates the project's preference for restoring main's form, but these four instances were not addressed there.

## Style - CLEAN

No em-dashes, en-dashes, double-hyphens used as dashes, or other unicode/special characters found in the three source files. No `#[allow(...)]` attributes in any of the three source files (zero hits). `just clippy` is clean.

## Summary

One finding, low severity. Verdict-preservation: clean (no path from Change A/B/C into state selection, convergence, or the streak). Byte-guard: clean (both scaffold copies carry the exact fragment). Determinism: clean (byte-identical). Plan/ledger/metrics: untouched.

| id     | sev  | location                                   | what                                                                                 |
|--------|------|--------------------------------------------|--------------------------------------------------------------------------------------|
| D2r3-1 | low  | src/next.rs, project/select_active_loop/build_context/extract_resume_state | Four stray rustfmt reflows in unchanged functions; semantically inert but add diff noise |
