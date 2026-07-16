# Round-2 review: session-preflight pack-doc step

Reviewer: confirming reviewer (round 2, low-risk artifact, 1 clean round required).
Commit under review: d77bd0c.

## Fix confirmation

T1b (base-currency rebase): confirmed. `pack/AGENTS.md` Worktree lifecycle paragraph now states the orchestrator rebases the worktree branch onto current main before spawning reviewers and the triager and before each fix round, with the rationale that a stale base manufactures false scope findings. Identical text present in `AGENTS.md` (regenerated via scaffold-self).

T2 (reviewers write to main repo): confirmed. Same paragraph now states read-only agents write their findings files to the main repo's `docs/plans/<task>.reviews/` (not inside the worktree), with the reason that records outlive worktree removal.

T3 (post-merge test failure path): confirmed. `pack/AGENTS.md` merge paragraph now includes: if the post-merge test check fails, the orchestrator undoes the merge so main stays clean, routes the fix to an isolated implementer and re-reviews it before retrying the fast-forward.

T4 (abandoned-step cleanup): confirmed. Same Worktree lifecycle paragraph now states that if a step is abandoned the orchestrator removes the worktree and deletes its branch without merging, invoking the commit-before-delete discipline for the workflow-managed artifacts.

D1 (conflict resolution through reviewer + separate triager, focused review): confirmed. The merge paragraph now states the resolution goes through a reviewer and a separate triager (per the always-separate-triager rule), as a focused review of just the resolution (the newly authored merge content), not a full re-review of the already-converged step.

T5 (preflight after state reconstruction): confirmed. `pack/AGENTS.md` Preflight section now states "in the resume case after reconstructing state per the 'Checkpoint and resuming after context loss' section above (matching the resume prompt's order)".

T6 (worktree-first where containers are not wired): confirmed. Both `pack/AGENTS.md` Preflight section and `pack/prompts/orchestrator.md` now use "worktree-first where containers are not wired" rather than the unqualified "worktree-first".

T7 (tier enumeration as resolved-tier statement, referencing Writer isolation rule): confirmed. `pack/AGENTS.md` Preflight section scopes the tier enumeration as "the human-facing statement of the resolved tier, referencing that rule for the tier policy rather than re-defining it". `pack/prompts/orchestrator.md` says "per the Writer isolation tier order in AGENTS.md, rather than re-defining the tier policy here". No double-definition.

T8 (resume.md says "defined in AGENTS.md"): confirmed. `pack/user-prompts/resume.md` now reads "the preflight defined in `AGENTS.md`" rather than "defined there".

## Coherence check

No contradiction found with Writer-isolation, Convergence, Review-loop, or File-safety sections. Specific checks:

- The T3 failure path (undo merge, route to isolated implementer, re-review) uses "the same path as the authored conflict resolution below", which is internally consistent with the D1 path described immediately after in the same paragraph.
- The T4 abandoned-step clause invokes "commit-before-delete discipline" correctly: this refers to committing workflow-managed files (findings files, ledger on main) before the worktree and branch are removed, not committing the unreviewed implementation changes, which aligns with the file-safety rule definition.
- The D1 focused-review scope ("just the resolution, not a full re-review of the already-converged step") does not conflict with the convergence or round-counting rules because it is explicitly a scoped pass, not a new convergence loop.
- T5 ordering (reconstruct state, then preflight) matches the resume.md order and the AGENTS.md Checkpoint section's description of the resume path.
- No dangling references found. All cross-references resolve to existing sections.

## Mechanicals

- `just scaffold-self`: exited 0, `git status --short` empty afterwards (17 files refreshed, 0 left untouched, 0 changed on disk).
- `just test`: 95 passed, 0 failed.
- `just clippy`: clean, no warnings.

## Verdict

All eight fixes confirmed. No new defect found. No regression. Artifact is clean.
