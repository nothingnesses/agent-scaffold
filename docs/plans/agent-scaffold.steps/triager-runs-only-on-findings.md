### `triager-runs-only-on-findings`: run the separate triager only on review rounds that have findings (`Q-63`)

Change the separate-triager rule so the triager runs on any review round in which the reviewers raised one or more findings, rather than on every review round. A round where every reviewer reports zero findings is already clean and needs no triage, because there is nothing to adjudicate. This touches the guidance across `pack/AGENTS.md` (the Roles-separation paragraph, the Triager role bullet, phases 3, 4, and 5, the review-entry-mode paragraph, the convergence-decision line, and the authored-conflict-resolution rule) and the orchestrator prompt (`pack/prompts/orchestrator.md`), plus their regenerated `.agents/` copies.

The never-collapse property is preserved: when the triager does run it is still always a separate agent (or a human), never merged into the producer or the orchestrator, because the orchestrator owns convergence and cost and is biased toward dismissing findings, so letting it triage would let that bias decide which findings count. "Zero findings" is read objectively from the committed reviewer findings files, not at the orchestrator's discretion, so the carve-out cannot become a way to suppress findings.

Why: Q-63, the human's decision on 2026-07-23, chosen over keeping the always-run rule and over a loop-only carve-out; it applies to all reviewers-then-triager passes (the plan-review and work-review convergence rounds, the acceptance pass, and the review-entry-mode pass).

Deferred: the driver-side reflection is owed to the workflow FSM work in `src/next.rs`, which models a triage transition; that is deferred, evidence-gated driver work and is not done in this guidance pass.
