## Documentation Protocol

<How this plan is kept current during the work. The structured `<task>.plan.toml` is the single source of truth: the Roadmap status lives in each `[[step]].status`, the human-decision queue in the `[[question]]` entries, and the numbered principles in `[[principle]]`. Edit the TOML and the prose sidecars, then run `agent-scaffold render` to regenerate this `<task>.md`, and `agent-scaffold render --check` before committing both together. Never hand-edit the generated view; edits are overwritten by the next render.>
