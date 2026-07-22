# driver-output-generation inc2 (next.rs consumer, RISKY) - round 4 reviewer

Diff reviewed: `main` (0b1eace) .. `impl/dog-inc2` HEAD (5543ccc). Confirming round.

## Verdict: CLEAN

No defects found. All six review axes verified against a rebuilt, freshly tested worktree.
The diff is intent-only (no incidental formatter reflow present, so the formatter policy
did not apply).

## What was probed

1. VERDICT-PRESERVATION (safety-critical): PASS. `spawns_writer` is referenced only at
   `src/next.rs:812` inside `build_instruction` (reminder assembly); the plan's principles
   flow only into `projected_principle_reminder` (`src/next.rs:339`) and thence into reminder
   assembly (`src/next.rs:806`). Neither touches state selection, convergence, streak, or the
   transition verdict. `next::tests::next_agrees_with_w3` passes; the full transition suite is
   green.
2. CHANGE B (isolation fragment): PASS. `ISOLATION_POLICY_FRAGMENT` is emitted verbatim via
   `format!("{lead} {ISOLATION_POLICY_FRAGMENT}")` at writer states only (`src/next.rs:812-818`),
   single source shared with the AGENTS.md render slot. The reword "per the capability-tiered
   tier order in the Writer isolation rule" is self-locating: it reads correctly inside AGENTS.md
   (rule header "Writer isolation (capability-tiered)" sits just above) and standalone in the
   driver reminder. `render --check` reports up to date; the inc1 byte-guard
   (`isolation_policy::tests::the_committed_scaffold_carries_the_isolation_policy_fragment`),
   content-pin (`the_fragment_states_the_writer_classification`), and the workflow_spec scaffold
   guard all pass. `next::tests::the_reviewer_states_carry_no_isolation_reminder` confirms the
   policy does not attach to the two read-only reviewer states.
3. CHANGE C (de-numbered reminders + projected escalate principle): PASS. No terse reminder
   carries a numeric "Principle N" citation (swept by
   `no_terse_reminder_carries_a_numeric_principle_citation`, all nine states). The escalate
   reminder projects the real plan `[[principle]]` "Ground decisions in evidence" by NAME with
   the plan's own number as a locator (present at plan.toml:1260); it degrades to the originated
   imperative alone when absent (`the_escalate_reminder_projects_a_real_plan_principle_by_name`,
   `the_escalate_reminder_degrades_when_the_principle_is_absent`).
4. DETERMINISM: PASS. `next` run twice against the live plan/metrics is byte-identical. Goldens
   are genuine (updated to the de-numbered strings, not stubbed).
5. REGRESSION: PASS. `just clippy` clean; 342 tests pass serial (`--test-threads=1`);
   `validate` and `validate --workflow` both green (154 records, 67 steps, invariants hold).
6. STYLE (substance): PASS. No em-dashes, en-dashes, dash-substitute double-hyphens, unicode,
   or emoji in the added code or strings. The only `--` matches are the ASCII `// -- Section --`
   dividers, which are the sanctioned plain-dash style. No `#[allow]` introduced.

## Key verdicts
- Verdict-preservation: PRESERVED (spawns_writer + principle projection consumed only in
  reminder assembly, never in convergence/state selection).
- Byte-guard: INTACT (isolation_policy byte-guard + workflow_spec scaffold guard + render --check
  all green).
