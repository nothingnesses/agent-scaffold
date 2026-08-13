### `workflow-audit-followups`: the 2026-08-13 audit's remaining recommendations, held as one deferred pointer rather than a backlog

Deferred, deliberately, and deliberately EMPTY of content.

The 2026-08-13 audit made ten ordered recommendations. Recommendation 1 is the release, which is `ship-v0-0-2`. Recommendations 2 to 10 stay where they were written: `docs/plans/workflow-calibration.explorations/2026-08-13-audit-when-the-loop-turned.md`, under `## Recommendations`, with the measurements that justify each of them in the sections above it. They are not copied here, not split into steps, and not summarised.

WHY THIS STEP IS A POINTER AND NOT A PLAN. The same audit measured that the share of this project's steps whose provenance is its own review process rose from 8.3% to 54.2%, monotonically, and that 74.3% of 1,022 commits touched neither product code nor tests. Turning ten recommendations into ten steps would be that mechanism running once more, at the moment the evidence for it arrived. One deferred pointer keeps them findable without creating work that then generates its own review rounds.

WHEN TO OPEN IT. After `v0.0.2` is published and `rename-to-agent-flow` has run, and then only against evidence from use rather than against the recommendations' own ordering. Read the audit record first, in full, and pick from it; do not treat this step as a queue to drain (Principle 2, Minimal by default; Principle 6, Ground decisions in evidence).

ONE THING TO CARRY FORWARD when it is opened, because it is the audit's own assessment of leverage and it is cheap to lose: recommendation 3, constraining what a fix pass may do, is the single highest-leverage variable the audit found. Deletion-only and number-edit-only fix passes re-seed findings at close to zero; fix passes that author prose re-seed at close to 100%. The project had measured that five separate times without acting on it.
