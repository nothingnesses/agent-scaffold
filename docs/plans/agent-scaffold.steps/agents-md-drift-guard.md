### `agents-md-drift-guard`: whole-file byte-equality drift guard for the committed root AGENTS.md (`Q-64`)

Add a whole-file byte-equality guard asserting that the committed root AGENTS.md equals a fresh `build_assets` render, the `render --check` equivalent for AGENTS.md. Today only per-fragment `.contains()` guards exist; nothing asserts that a fresh render byte-equals the committed file, so a hand-edit of authored prose or a dropped generated slot is not caught the way `render --check` catches an edit of a `<task>.md` sidecar (Q-64-generation-architecture sec 2, 5.3).

This is the cheapest and highest-value item in the Option-A core and is independent of any slot-widening: it catches the W4-baseline drift class at near-zero cost. It serves Make illegal states unrepresentable (a divergence between the authored template plus generated slots and the committed file becomes a failing check, not a latent bug) and Structured data first, project for humans (the committed human view is pinned to a fresh projection of the source).

Endorsed-now core item under `generated-projection` (Q-64, Option A refined). Build now.
