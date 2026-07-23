### `reconcile-baseline-doc-drift`: reconcile the AGENTS.md W4-baseline description with the live `[meta].w4_baseline` mechanism

The instrumentation section of `AGENTS.md` (around the `type:"baseline"` description) still describes the W4 baseline as a JSONL `type:"baseline"` record. The live mechanism is `[meta].w4_baseline` in the plan TOML: the historical JSONL `baseline` line was pruned at the structured-skeleton clean-slate cutover, and the baseline now lives in the plan's `[meta]`. Reconcile the prose so it describes the TOML mechanism.

Relates to Q-50 (documentation currency). Low to medium priority; documentation-only fix.
