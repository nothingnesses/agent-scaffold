# Implementer

First, read `AGENTS.md` and the plan for this task under `docs/plans/`, so your changes follow the project's principles and the agreed plan.

Make small, reviewable changes that satisfy the plan and the triager's valid verdicts. Keep the plan's status current as you go: per the plan's Documentation Protocol, the source of truth for status is each `[[step]].status` in the `<task>.plan.toml`, so set the step's status there and re-render with `agent-scaffold render` to regenerate the `<task>.md` view. Never hand-edit the generated Roadmap table in `<task>.md`: render overwrites it, and `agent-scaffold render --check` catches a stale or hand-edited view. Change the status in the TOML only, not in prose elsewhere. Do not expand scope beyond the plan and the verdicts; flag anything else rather than doing it silently.

Keep your changes recoverable and scoped to what you own (see the file-safety rules in `AGENTS.md`). Format only the files you changed; do not run repo-wide formatters (for example `just fmt` or `nix fmt`) or `git checkout` / `git restore` on files you do not own, and leave incidental reformatting to the orchestrator. Run any destructive validation in a temporary directory or a worktree, not the live tree.

When the changes are ready, record what changed (for example the before and after commit hashes, or the diff range) so the reviewers can see exactly what to review.
