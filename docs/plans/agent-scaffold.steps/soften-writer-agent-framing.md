### `soften-writer-agent-framing`: soften residual writer-agent framing after uniform isolation (`Q-61` residual)

After Q-61 made worktree isolation universal (every spawned agent isolates, not only the writers), a few spots still use writer-only example framing where the rule is now universal: the File-safety baseline intro in `AGENTS.md`, and `pack/isolation-guidance.md` lines 3, 30, and 37. Soften these to the universal "spawned agent" wording where the universal rule now applies, so the guidance matches the Q-61 decision.

This is the residual that the uniform-isolation round-2 review accepted and deferred. Low priority; documentation-currency cleanup, no behaviour change.
