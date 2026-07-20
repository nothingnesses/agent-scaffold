Clean. The one-word fix (removal of trailing "Deferred." from `agents-worktree-planner-scope.md`) is correct, surgical, and introduces no new issues.

Verified:

- F-1 resolved: "Deferred." is absent from `docs/plans/agent-scaffold.steps/agents-worktree-planner-scope.md` (line 3 now ends "No open decision, a known change."); the corresponding line in the compiled `docs/plans/agent-scaffold.md` is updated identically.
- Commit `137787f` touched exactly two files: the sidecar and the compiled plan. No `src/`, no ledger, no metrics, no other docs.
- `validate --source docs/plans/agent-scaffold.plan.toml`: 145 records valid, 67 steps, 57 questions, valid.
- `validate --workflow --source docs/plans/agent-scaffold.plan.toml`: workflow invariants hold.
- `render --check docs/plans/agent-scaffold.plan.toml`: up to date.
- No em-dashes, no unicode, no emoji, no hard-wrapped prose.
- No new findings.
