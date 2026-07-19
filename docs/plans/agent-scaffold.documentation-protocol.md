## Documentation Protocol

This plan is kept current during the work.

Progress lives in the Roadmap: an ordered table of step slugs and their status (`not started`, `in progress`, `complete`, `skipped`, `next`, `optional`, `deferred`, or `blocked on <slug>`). The Roadmap is the single source of truth for status and for implementation order. The Step Details below carry each step's design, decisions, and (once done) outcome and evidence, and do not repeat the status label.

Steps are identified by a stable kebab-case slug, not a number, so a reference stays valid when the order changes: every cross-reference to a step uses its slug. Each Roadmap row maps to a detail block headed by the same slug; some detail blocks are grouped under an umbrella heading that carries context shared by several steps, and the umbrella itself is not a Roadmap entry. To add a step, add its detail block and insert its slug into the Roadmap at the right position; to reorder or reprioritise, edit the Roadmap rows. Keep the two in sync: every Roadmap slug has a detail block and vice versa.

The "Open Questions, Decisions, Issues and Blockers" section is the single living human-decision queue (the model adopted in `human-review-queue`). Each item has a stable id, a one-line ask, a status (`open`, `decided -> folded into <slug>`, or `superseded`), and a context pointer into the step or ledger that carries the detail. Resolved items are marked resolved, not deleted, so the same question is not addressed twice; the adopted decision and its reasoning are folded into the relevant step, and the queue item points to that step. The orchestrator updates the queue at every checkpoint as a required step and pushes the open items to the human rather than waiting for the human to pull them.
