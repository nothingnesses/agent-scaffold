# driver-isolation-reminder-scope round-1 triage

5 valid (of which 1 blocking), 0 invalid, after dedup

Scope check performed independently against the worktree code (not the reviewers' word). All cited lines read and confirmed. `git --no-pager show HEAD~1` (7923503) is the feat commit; HEAD (60e0f48) is the findings-merge. Only `src/next.rs` changed.

Dismissed HIGH/CRITICAL backstop: NONE. Both reviewers raised only medium/low/Info. No high- or critical-severity finding was dismissed, so no backstop re-check is owed.

---

## Item 1 (merged: A1 + B1) - emitted resolve-note names the wrong role

- Verdict: VALID.
- Final severity: low (re-severitised down from B's medium; up from nothing above A's low). The emitted string is genuinely self-contradictory, but the actionable content (resolve the tier per AGENTS.md policy before the spawn) stays correct and unambiguous, and it only shows at the tier-unknown sub-case of the two reviewer states. Real defect, low harm.
- Disposition: BLOCKING.
- Evidence: `src/next.rs:63-64` `TIER_RESOLVE_NOTE = "Resolve the isolation tier per the AGENTS.md tier policy before spawning the writer."`. It is emitted at `src/next.rs:849` inside the `spawns_isolated_agent()` branch (`src/next.rs:847`), whose predicate (`src/next.rs:297-305`) now includes `AwaitingFirstReview | AwaitingReviewers`. The lead was genericised to "Agent isolation (tier not yet resolved)." (`:849`) but the appended note still says "spawning the writer", so at a reviewer-spawn state with unknown tier the single emitted string says "Agent isolation ... before spawning the writer". No test catches wording: the helpers match on `.contains(TIER_RESOLVE_NOTE)` (`:1434`), i.e. presence not text.
- Reasoning for blocking: this is an operator-facing emitted-output inaccuracy introduced directly by this change (the widening is what carries the writer-worded note to reviewer states). This is a low_risk step targeting 1 clean round; a self-contradiction in the tool's own output is a fix-now regression, and the fix is a one-word const edit. Fold it now rather than ship it and reopen later.
- Recommended action: in `src/next.rs:64` replace "before spawning the writer" with "before spawning the agent" (matches the "Agent isolation" lead and the `spawns_isolated_agent` predicate name). Minimal, single-line.

## Item 2 (merged: A2-part + B2) - stale doc on TIER_RESOLVE_NOTE

- Verdict: VALID.
- Final severity: low (B2 low; A2 called Info; low is right, it is a canonical declaration-site doc-comment that now under-describes scope).
- Disposition: NON-BLOCKING, but fold into the same pass as Item 1.
- Evidence: `src/next.rs:57` "folded into the writer-isolation reminder"; `src/next.rs:61` "fires at every writer state regardless of tier". After Q-62 the enclosing block fires at all four agent-spawn states and the production inline comment was already renamed to "agent-isolation reminder" (`:844`).
- Recommended action: update `:57` and `:61-62` wording "writer-isolation reminder" -> "agent-isolation reminder" and "every writer state" -> "every agent-spawn state".

## Item 3 (B3) - stale principle_reminders field doc

- Verdict: VALID.
- Final severity: low.
- Disposition: NON-BLOCKING, fold into the same pass.
- Evidence: `src/next.rs:177` "and, at writer states, the generated isolation-policy fragment." The fragment now attaches at all four spawn states (`:847`, `:297-305`).
- Recommended action: `:177` "at writer states" -> "at agent-spawn states".

## Item 4 (B4) - stale test-section header

- Verdict: VALID.
- Final severity: low.
- Disposition: NON-BLOCKING, fold into the same pass.
- Evidence: `src/next.rs:1414` `// -- The always-on writer-isolation reminder --` vs the updated production comment `src/next.rs:844` `// The always-on agent-isolation reminder:`. Two names for one concept.
- Recommended action: `:1414` -> `// -- The always-on agent-isolation reminder --`.

## Item 5 (merged: A2-part + B5) - stale resolve-note helper doc

- Verdict: VALID.
- Final severity: low.
- Disposition: NON-BLOCKING, fold into the same pass.
- Evidence: `src/next.rs:1427-1428` "fired only when the tier is still `unknown` at a writer state". The note fires at any state where `spawns_isolated_agent()` is true (`:847`), now including the reviewer states. The helper body is correct (matches on content, `:1434`); only the doc is wrong.
- Recommended action: `:1428` "at a writer state" -> "at any agent-spawn state".

---

## Round-1 outcome recommendation

NEW VALID FINDINGS (not clean). One blocking emitted-output inaccuracy plus four stale-wording doc/comment residuals, all the same "writer -> agent" theme introduced by this widening. The doc/comment items are individually non-blocking but are the same coherent rename and trivially cheap; folding them into the single blocking fix avoids leaving staleness that contradicts the just-renamed `spawns_isolated_agent` predicate and the already-updated `:844` production comment. Recommend one implementer pass, then re-review.

Minimal fix list for the implementer (all in `src/next.rs`, all "writer -> agent" wording, no logic change):
1. `:64` emitted const: "before spawning the writer" -> "before spawning the agent". (BLOCKING)
2. `:57` doc: "writer-isolation reminder" -> "agent-isolation reminder".
3. `:61-62` doc: "every writer state regardless of tier" -> "every agent-spawn state regardless of tier".
4. `:177` field doc: "at writer states" -> "at agent-spawn states".
5. `:1414` test header: "writer-isolation reminder" -> "agent-isolation reminder".
6. `:1428` helper doc: "at a writer state" -> "at any agent-spawn state".

No test change is required for the behaviour (the flipped pin test and goldens already pin the state machine); the const-text edit (fix 1) does not break any assertion because the helpers match on `.contains(TIER_RESOLVE_NOTE)`, i.e. on the const itself, not its literal words. Implementer should re-run `cargo test` and `cargo clippy --all-targets -- -D warnings` after the edits.
