### `driver-isolation-reminder-scope`: widen the `next` driver's isolation reminder to the reviewer / triager-spawn states (`Q-62`)

Widen the `next` driver so its worktree-isolation reminder fires at the reviewer / triager-spawn states, not only the writer states, matching the uniform-isolation rule (Q-61) at the point of action. Concretely, widen `spawns_writer` (or add a sibling predicate) in `src/next.rs` so the isolation reminder attaches at the review-spawn states `AwaitingFirstReview` and `AwaitingReviewers`, and flip the content-pin test `the_reviewer_states_carry_no_isolation_reminder`.

Why: Q-62. The human chose option (a) on 2026-07-23, over (b) keep-writer-only and (c) split-the-reminder. Reasoning: P1 (one uniform rule, no drift between the guidance and the driver) puts the reminder at the point of action for every spawn state now that Q-61 makes every spawned agent isolate.

Cited locations: `src/next.rs` `spawns_writer`, the review states `AwaitingFirstReview` and `AwaitingReviewers`, and the content-pin test `the_reviewer_states_carry_no_isolation_reminder`.

Deferred: this is a code change in `src/next.rs` plus flipping the pin test, owed as a later build step rather than done in this fold pass.
