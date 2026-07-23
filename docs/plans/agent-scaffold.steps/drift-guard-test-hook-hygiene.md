### `drift-guard-test-hook-hygiene`: serialize the drift-guard test helper's panic-hook swap (R3-CQ-1 residual)

The `precondition_rejects` test helper in `src/agents_md_drift.rs` swaps the process-global panic hook (take a no-op hook, run `catch_unwind`, restore) without serialization. Under parallel `cargo test`, two callers' take/set pairs can interleave and transiently leave the no-op hook installed, suppressing a concurrent or subsequent real panic's backtrace.

Ruled VALID but NON-BLOCKING (low) in the `agents-md-drift-guard` round-3 triage and accepted as a residual: it is diagnostic-only (each call captures its result via `catch_unwind(...).is_err()`, independent of the hook), so it cannot flip any test's pass/fail and is not a flake; the only impact is backtrace visibility under a parallel failure.

Fix options: serialize the swap behind a `Mutex`, drop the hook swap entirely, or set the no-op hook once for the test binary. Deferred; low priority.
