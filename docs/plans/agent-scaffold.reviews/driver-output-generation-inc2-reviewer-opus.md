# Reviewer (opus) findings: driver-output-generation increment 2 (src/next.rs consumer)

Lens: driver-logic correctness, verdict-preservation, principle projection, test honesty.
Diff reviewed: main (17dce12) .. impl/dog-inc2 (36ed42a), commits 26cc178 (fragment reword) + 36ed42a (next.rs consumer).
Worktree: /home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/dog-inc2

## Verdict: CLEAN

No correctness, verdict-preservation, byte-guard, projection, or test-honesty defects found. One low informational note below (not a defect).

## What I probed and confirmed

CHANGE A (spawns_writer narrowing, D-d):
- `spawns_writer` is now `matches!(self, LoopState::ReadyToPlan | LoopState::AwaitingFixes)` (src/next.rs:277-279), exactly the two writer-spawning states; the two read-only reviewer states are excluded.
- The only call site is src/next.rs:816 (reminder assembly in `build_instruction`). It is NOT read by any verdict/transition/convergence code (grep-confirmed: no other `spawns_writer` usage in src/). Narrowing cannot leak into the verdict.
- Test `the_reviewer_states_carry_no_isolation_reminder` (src/next.rs:1471-1490) asserts BOTH review states, at BOTH known and unknown tier, carry neither the fragment nor the resolve-note. Passes.

CHANGE B (always-on isolation reminder, verbatim const, self-locating reword):
- Writer states unconditionally push `format!("{lead} {ISOLATION_POLICY_FRAGMENT}")` (src/next.rs:814-822); the old `tier == "unknown"` gate is gone. The emitted policy is the shared const verbatim (src/next.rs uses `ISOLATION_POLICY_FRAGMENT` directly; test `a_writer_state_emits_the_isolation_fragment_for_any_tier` matches the WHOLE const, not a paraphrase). Single source, no drift path.
- The reword ("...tier order above" -> "...tier order in the Writer isolation rule", src/isolation_policy.rs:33) reads correctly inside AGENTS.md: the "Writer isolation (capability-tiered)." heading sits at AGENTS.md:83 with the tier order at :83-87, and the fragment at :91 references that rule by name. Standalone in the driver reminder it names a findable rule instead of a dangling "above".
- inc1 byte-guard `the_committed_scaffold_carries_the_isolation_policy_fragment` (src/isolation_policy.rs:66) and content-pin `the_fragment_states_the_writer_classification` (:47) both PASS after the re-scaffold (AGENTS.md and .agents/AGENTS.reference.md regenerated in the diff). `render --check docs/plans/agent-scaffold.plan.toml` reports "up to date". The build-path slot test `isolation_policy_slot_renders_the_generated_fragment` passes.

CHANGE C (principle projection, D-a + D-e):
- Terse reminders de-numbered: every `base_reminders` string stripped of "Principle N:" (src/next.rs:299-321). The escalate base contract reminder now says "judged against the Project Principles" (no number, src/next.rs:316). Sweep test `no_terse_reminder_carries_a_numeric_principle_citation` (src/next.rs:1493-1516) covers all nine states and genuinely fails if a number returns.
- No numeric citation remains in any emitted output string. `grep 'Principle [0-9]'` over src/next.rs hits only DOC COMMENTS and TEST COMMENTS (lines 60, 190, 281, 840, 935, 1496), which are pre-existing code-governance references, not driver output.
- `projected_principle_reminder` (src/next.rs:339-346) looks up `ESCALATE_PRINCIPLE_NAME` ("Ground decisions in evidence") by NAME and emits `name` + plan `n` (as locator) + `text`, live from `context.principles`. Threaded correctly: main.rs carries `source_plan.principles` for the TOML source and `Vec::new()` for the Markdown/no-source paths (src/main.rs:1116-1136), into `NextInputs.principles` (:1172) -> `LoopContext.principles` (src/next.rs:512, 533). The plan carries "Ground decisions in evidence" at n=6 (docs/plans/agent-scaffold.plan.toml:1259-1261), so the live projection shows "plan principle 6" (not a hardcoded AGENTS.md number). Test `the_escalate_reminder_projects_a_real_plan_principle_by_name` asserts real text + "plan principle 6".
- Graceful degradation is real: `projected_principle_reminder` returns `None` when the principle is absent (the `?` on `find`), and build_instruction only pushes on `Some`. Test `the_escalate_reminder_degrades_when_the_principle_is_absent` (empty principles, the Markdown case) asserts the base contract reminder is still present AND no reminder mentions the principle name. Passes.

VERDICT-PRESERVATION:
- LoopState derivation, transition tables, `peak_consecutive_clean` (src/next.rs:671, 727; src/workflow.rs:407), and `select_active_loop` are behaviourally unchanged. The next.rs diff to `select_active_loop`, `next_agrees_with_w3`, `extract_resume_state`, the metrics map, and `build_context` is pure rustfmt reflow (no token/logic change); the differential assertions in `next_agrees_with_w3` are identical round data, only reformatted.
- The differential test `next_agrees_with_w3` and the transition/state tests pass. All 342 bin tests pass under `--test-threads=1`. clippy clean.

TEST HONESTY (Principle 11):
- Goldens (GOLDEN_TEXT / GOLDEN_JSON) updated to the de-numbered AwaitingReviewers reminders (src/next.rs:1632-1633, 1668-1669). The golden is a reviewer state, so it also implicitly pins that reviewer states carry NO isolation reminder. Genuine new intended output, not weakened.
- New tests match the full shared const / real projected text / TIER_RESOLVE_NOTE (not vacuous contains-of-a-substring-of-anything). Each fails on the regression it guards.

STYLE:
- No em-dashes, en-dashes, unicode, or emoji in new code or strings. The only `--` hits in the diff are `// -- Section --` dividers, which are the sanctioned plain-dash section style.
- `#[expect]`/`#[allow]`: no new lint attributes introduced.

## Low (informational, not a defect)

D2-1  low  src/next.rs:1592-1607
There is no full golden for a WRITER-state instruction showing the assembled `"Writer isolation (resolved tier: X). <fragment>"` lead + inline fragment as one block; the golden fixture is the AwaitingReviewers (reviewer) state. The writer-state output shape is instead covered by targeted contains-checks (`a_writer_state_emits_the_isolation_fragment_for_any_tier`, `a_writer_state_echoes_the_resolved_tier_in_the_isolation_reminder`, `an_unknown_tier_adds_the_resolve_note_at_a_writer_state`), which is adequate. A writer-state golden would additionally pin the exact lead-plus-fragment concatenation and reminder ordering, but its absence is not a correctness gap. No action required for this increment.
