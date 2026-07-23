### `rename-to-agent-flow`: rename the crate and binary from agent-scaffold to `agent-flow` at a stable release (`Q-65`)

Rename the project from `agent-scaffold` to `agent-flow` (descriptive, not a metaphor), timed to a release-ready stable state so the name can be claimed on crates.io. The identity has crossed from a scaffolding tool to a structured workflow VALIDATOR plus advisory DRIVER, with `scaffold` as one bootstrap subcommand; the name should stop mis-selling a one-shot generator (Structured data first, project for humans; Prefer the cleaner long-term architecture over the smallest diff). It is NOT a meta-harness: agents INVOKE the tool and it never runs the LLM loop, so "meta-harness" is a category error. Avoid "engine", which implies the unbuilt authoritative driver and would invite name-driven scope creep against Minimal by default.

DEFERRED because it is release-timed: land it at a stable release-ready state, not speculatively.

crates.io RELEASE CHECKLIST (durable notes):

- crates.io has NO in-place rename. Publish a NEW crate `agent-flow`: set the `Cargo.toml` `name`, the binary becomes `agent-flow`, the `scaffold` subcommand stays. Update all in-repo command examples and docs that say `agent-scaffold`.
- Keep the version line continuous: reserve `agent-flow` on crates.io at 0.0.2, with the first REAL release under `agent-flow` at 0.0.3.
- The final `agent-scaffold` publish (0.0.2) README must link to `agent-flow` AND state that the `agent-scaffold` name is free for whoever wants to reclaim it, contact by opening an issue on the `agent-flow` GitHub repo. Yank the old `agent-scaffold` versions only AFTER the redirect and the new crate are live.
- crates.io treats `-` and `_` as the same name, so publishing `agent-flow` also reserves `agent_flow`.

Folded from Q-65 (rename decision) and Q-64 (the identity conclusion that determines the name). Provenance: Q-65, Q-64.
