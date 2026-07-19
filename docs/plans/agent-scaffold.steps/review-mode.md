### `review-mode`: Human-invokable review as a fourth entry mode (`Q-35`)

Not started. Decided (`Q-35`, human, after a two-independent-explorer design pass; design record in `docs/plans/review-mode.explorations/review-entry-mode-{A,B}.md`). Add a fourth ENTRY MODE to the one workflow: the acceptance pass promoted to a human-invokable entry, so a human can prompt the agent to review a codebase (or a diff) against a plan and/or specific constraints and get a findings report. The tool is converging on ONE workflow with several entry modes (`implement` via `kickoff.md`, `explore` via `explore.md`, `review` via a new `review.md`), each a different entry and terminal over the same roles and durable artifacts; `review` reuses the review machinery wholesale (independent reviewers with distinct lenses, a separate triager, findings files, the four-level severity scale plus the high/critical dismissal-recheck backstop), so it adds no new phase or role and staffs only the read-only roles (no implement phase).

Decided shape:

- Entry: a thin `pack/user-prompts/review.md` mirroring `kickoff.md` / `explore.md`, registered in `pack.toml` and covered by the manifest asset-list test.
- Terminal: a SINGLE-PASS reviewers-then-triager (the acceptance variant, not the consecutive-clean convergence loop), KEEPING the high/critical dismissal recheck, ending in a COMMITTED, KEPT report (`docs/plans/<task>.review-report.md` or `.reviews/report.md`; the planner picks the exact path at build time) with a paste-into-`kickoff` handoff so the human can turn valid findings into an implement task. A minimal one-round ledger is created for the run and deleted at close (commit-before-delete).
- TARGET: a diff range or the whole tree at HEAD, reusing the reviewer before/after-or-range vocabulary.
- CRITERIA (composable): a named plan's Success Criteria, plus ad-hoc constraints/conditions, plus an always-present principles floor (open-ended correctness/quality).
- Instrumentation: under `--instrument`, the run logs one `round` record with a NEW `review` phase value; add `Review => "review"` to the `Phase` enum in `src/metrics.rs`, document it in `pack/instrument.md`, and extend the validator tests. This is the only code beyond prompt/pack content.
- A one-clause generalization of `pack/prompts/reviewer.md` and `pack/prompts/triager.md` so "the plan" degrades to "the criteria you were given" when a review run has no plan.

This SUBSUMES the whole-codebase acceptance-review LATER job noted elsewhere in this plan; that job becomes an instance of this mode. Footprint: mostly pack content plus the one `Phase` enum value plus the `review.md` pack.toml/manifest registration.
