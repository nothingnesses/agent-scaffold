# Round 3 triage: driver-output-generation-inc2

Triager: independent of reviewers, implementer, orchestrator. READ-ONLY inspection.
Branch: `impl/dog-inc2` (HEAD `da2eb17`). Main base: `ad2379e`.
Finding under judgement: D2r3-1 (low).

## Verdict: VALID

ACTIONABLE THIS ROUND? yes.
Round outcome: NEW_VALID (the clean streak resets from 1 to 0).

## Reasoning

I confirmed all four reflows against `git diff main..HEAD -- src/next.rs`:

- `project()`: `metrics: inputs.metrics_records.map(|records| MetricsSummary { records }),`
  (main, one line) expanded to a three-line brace form. The only logical change
  inc2 makes to `project()` is the added `principles: inputs.principles,` line in the
  `LoopContext` construction; the `metrics` field belongs to the separate
  `NextProjection` construction and is unrelated to that change. Incidental.
- `select_active_loop()`: the `if let Some(step) = steps.iter().filter(...).min_by_key(...)`
  head reflowed to a four-line method chain. This function's LOGIC is untouched by inc2.
- `build_context()`: the three-line `review_findings` `format!()` collapsed to a
  two-line assignment. Logic untouched by inc2.
- `extract_resume_state()`: the final four-line `if in_section { ... } else { None }`
  collapsed to one line. Logic untouched by inc2.

Three of these four (`select_active_loop`, `build_context`, `extract_resume_state`)
sit in functions inc2 does NOT logically modify at all. That is the SAME situation as
D2s-1's `run_resume` hunks (stray reflow in an otherwise-untouched function), not the
milder "reflow near a heavily-edited region" case. The "next.rs is heavily edited, so
nearby reflow is expected" defense only really covers the `project()` metrics hunk, and
even there the reflowed field is not the field inc2 changed.

The deciding factor is CONSISTENCY within this very increment. D2s-1 was ruled VALID and
fixed one round earlier (commit `da2eb17` reverted the two `main.rs`/`run_resume` reflows
to main's form). D2r3-1 is the identical class of finding. Accepting it now, after
fixing its twin last round, would be an internally inconsistent ruling that hollows out
the project's intent-only-diff discipline and is exploitable. The round-3 reviewer
flagged it correctly.

I weighed DEFER honestly. The strongest defer argument is that the ROOT CAUSE is Q-57
(the committed codebase is not consistently formatted to the project's own
rustfmt/treefmt config, e.g. `use_small_heuristics = "Max"`, so every edit-then-format
reflows unrelated lines), which has ~38 stash entries behind it, and that hunk-by-hunk
reverting is churn treating a symptom. That argument is real, but it points at fixing
Q-57, NOT at accepting individual scope violations: accepting them makes the codebase
drift further from its config and makes Q-57 worse. Until Q-57 is resolved, the
established discipline (and the D2s-1 precedent) governs, so the correct call is VALID.

## Smallest fix (with SWEEP, do NOT re-run a formatter)

Revert to main's form every purely-incidental, whitespace-only reflow in `next.rs` that
lands in a function whose LOGIC this increment did not change. Edit the specific lines
back by hand; do not run rustfmt/treefmt (that is what introduced the reflows). The
committed `git diff main..HEAD -- src/next.rs` must become intent-only.

Revert the four named hunks:

1. `project()`: restore `metrics: inputs.metrics_records.map(|records| MetricsSummary { records }),`
   on one line.
2. `select_active_loop()`: restore the two-line
   `if let Some(step) = \n steps.iter().filter(...).min_by_key(...)` head.
3. `build_context()`: restore the three-line `review_findings` `format!()` call.
4. `extract_resume_state()`: restore the four-line `if in_section { ... } else { None }`
   block.

IMPORTANT (to avoid ANOTHER streak reset from a partial fix): the sweep MUST also cover
the same class of incidental reflow in the TEST module of `next.rs`, which the reviewer
did not enumerate but which is present in the diff and lands in tests inc2 did not
logically change:

- the `assert_eq!(loop_.next_instruction.context.get("blocked_by")...)` reflow;
- the `next_converged = projection.active_loop.as_ref().is_some_and(...)` chain reflow
  (in the `assert_differential` helper);
- the several `round(...)`-argument expansions and the `(1 ..= 5).map(...)` closure
  expansion in `next_agrees_with_w3`.

Revert each of those to main's form as well. After the sweep, re-run
`cargo test --bins -- --test-threads=1` (expect 342 passing) and confirm the residual
`git diff main..HEAD -- src/next.rs` contains only inc2's intended logic (the
`ISOLATION_POLICY_FRAGMENT`/`Principle` imports, `TIER_RESOLVE_NOTE`,
`ESCALATE_PRINCIPLE_NAME`, `projected_principle_reminder`, `spawns_writer` narrowing,
de-numbered `base_reminders`, `principles` threading, `build_instruction` reminder
assembly, the doc-comment updates, and the new/rewritten tests) with zero cosmetic
reflow of unchanged code.

## Q-57 evidence recommendation: YES, add the note

Recommend the orchestrator append a note to Q-57 (treefmt/stash hygiene). This finding is
now concrete, repeated evidence for resolving the treefmt strategy: within a SINGLE
increment, the same edit-then-format-reflows-unrelated-lines problem produced D2s-1 (fixed
`da2eb17`) and then D2r3-1 (this round), each an incidental-reflow scope violation, and
D2r3-1 resets the RISKY streak a second time. The recurring streak resets and repeated
findings are the cost of the codebase not being formatted to its own config once; that
is exactly the friction Q-57 exists to remove. The note should record: two same-class
findings in one increment, one streak reset attributable purely to cosmetic reflow, and
~38 stash entries already tracking the same root cause, as the case for prioritising
Q-57's resolution (format the whole codebase to config once) over continuing to revert
reflow hunk-by-hunk per increment.
