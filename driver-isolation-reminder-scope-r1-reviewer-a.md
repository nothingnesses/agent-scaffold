2 FINDINGS

Verdict: the change faithfully implements Q-62 option (a). The isolation reminder now fires at all four spawn states with the tier echo and (when tier unknown) the resolve-note; the fragment is single-sourced, not forked; the flipped test genuinely pins the new behaviour; goldens genuinely pin the rendered reviewer-state line. `cargo test` = 348 passed, 0 failed; `cargo clippy --all-targets -- -D warnings` = clean. The two findings below are low/Info content/doc-staleness, not correctness defects in the state machine.

Evidence for fidelity (no finding, recorded for the record):
- `src/next.rs:297-305`: `spawns_isolated_agent` matches exactly `ReadyToPlan | AwaitingFixes | AwaitingFirstReview | AwaitingReviewers`; non-spawn states (Blocked, Converged, Escalate, RiskClassConflict, Done) excluded. Exhaustive.
- `src/next.rs:847`: the ONLY call site of the predicate. `grep -rn spawns_writer src/ tests/` returns nothing; rename is complete, no missed call site.
- `src/next.rs:849-853`: full reminder (lead + `ISOLATION_POLICY_FRAGMENT`) fires at any tier; resolve-note (`TIER_RESOLVE_NOTE`) folded into lead only when `isolation_tier == "unknown"`. Option (c) split not present; the tier echo is emitted at reviewer states too.
- `src/isolation_policy.rs:33`: `ISOLATION_POLICY_FRAGMENT` is a single const; not forked or restated in next.rs (next.rs imports it, next.rs:29).
- Flipped test `the_reviewer_states_now_carry_the_isolation_reminder` (`src/next.rs:1487-1512`): asserts fragment IS present at both reviewer states, and `reminders_carry_the_resolve_note == (tier == "unknown")` for both. Not a tautology; asserts the new positive behaviour and the tier-conditional note.
- Golden substitution `golden_with_fragment` (`src/next.rs:1749-1755`) replaces `__ISOLATION_FRAGMENT__` with the real const in the EXPECTED string, then compares against the actual render (`golden_human`/`golden_json`, 1758-1768). The surrounding literal `- Agent isolation (resolved tier: worktree). ` (human, `src/next.rs:1704`) and `"Agent isolation (resolved tier: worktree). ..."` (json, `src/next.rs:1740`) is pinned verbatim; the placeholder only stands in for the long const, so drift is still caught. Golden state is `awaiting-reviewers` at tier `worktree` (`src/next.rs:1666,1677,1689`), so no resolve-note expected, and none appears. Correct.

---

A1 | low | src/next.rs:63-64
Description: `TIER_RESOLVE_NOTE` emitted text ends "...before spawning the writer." This note now fires at the two reviewer/triager-spawn states (`AwaitingFirstReview`, `AwaitingReviewers`) when their tier is unknown, where the agent about to be spawned is a REVIEWER, not a writer. The emitted user-facing instruction therefore says "spawning the writer" at a reviewer spawn.
Why valid: `build_instruction` (src/next.rs:847-853) pushes `TIER_RESOLVE_NOTE` for every state where `spawns_isolated_agent()` is true, which now includes the reviewer states; the const string was written for the writer-only scope and was not updated. The lead ("Agent isolation ...") was correctly genericised from "Writer isolation", but the note it precedes still names "the writer". No test catches this because the helpers match on `contains(TIER_RESOLVE_NOTE)` (src/next.rs:1429-1434), i.e. presence, not wording.
Impact: content-accuracy only; the actionable instruction (resolve the tier per AGENTS.md policy) is still correct and the state machine behaviour is right. Not a correctness bug in logic, but a user-facing inaccuracy introduced by the scope widening.
Suggested direction: generalise the note wording to "before spawning the agent" (or "before the spawn"), consistent with the "Agent isolation" lead rename, rather than "the writer".

A2 | Info | src/next.rs:57-62, 1427-1428
Description: Stale doc comments still scope the resolve-note/fragment to "writer state". The `TIER_RESOLVE_NOTE` doc (src/next.rs:57-62) says "folded into the writer-isolation reminder ... fires at every writer state"; the `reminders_carry_the_resolve_note` helper doc (src/next.rs:1427-1428) says "fired only when the tier is still `unknown` at a writer state". Both now also apply at reviewer states. (`isolation_policy.rs:8,15,21-22` similarly still say "writer-state isolation reminder"/"the writers", though those describe the fragment's broader who-isolates content and are less wrong.)
Why valid: after Q-62 the predicate is `spawns_isolated_agent` covering four states; these comments were not updated and now under-describe the scope.
Impact: documentation only; no behavioural effect.
Suggested direction: update the two next.rs doc comments to say "agent-spawn state" / "at any agent spawn", matching the renamed predicate and lead.
