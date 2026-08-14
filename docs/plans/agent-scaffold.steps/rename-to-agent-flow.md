### `rename-to-agent-flow`: rename the crate and binary from agent-scaffold to `agent-flow` at a stable release (`Q-65`)

Rename the project from `agent-scaffold` to `agent-flow` (descriptive, not a metaphor), timed to a release-ready stable state so the name can be claimed on crates.io. The identity has crossed from a scaffolding tool to a structured workflow VALIDATOR plus advisory DRIVER, with `scaffold` as one bootstrap subcommand; the name should stop mis-selling a one-shot generator (Structured data first, project for humans; Prefer the cleaner long-term architecture over the smallest diff). It is NOT a meta-harness: agents INVOKE the tool and it never runs the LLM loop, so "meta-harness" is a category error. Avoid "engine", which implies the unbuilt authoritative driver and would invite name-driven scope creep against Minimal by default.

DEFERRED because it is release-timed: land it at a stable release-ready state, not speculatively.

crates.io RELEASE CHECKLIST (durable notes):

- crates.io has NO in-place rename. Publish a NEW crate `agent-flow`: set the `Cargo.toml` `name`, the binary becomes `agent-flow`, the `scaffold` subcommand stays. Update all in-repo command examples and docs that say `agent-scaffold`.
- Keep the version line continuous: reserve `agent-flow` on crates.io at 0.0.2, with the first REAL release under `agent-flow` at 0.0.3.
- The final `agent-scaffold` publish (0.0.2) README must link to `agent-flow` AND state that the `agent-scaffold` name is free for whoever wants to reclaim it, contact by opening an issue on the `agent-flow` GitHub repo. Leave the old `agent-scaffold` versions un-yanked so the name stays reclaimable. THIS BULLET IS DONE AND ITS CONTACT ROUTE RESOLVED TO A DIFFERENT REPOSITORY THAN IT NAMES; the paragraph below says what shipped and what this step must preserve.
- crates.io treats `-` and `_` as the same name, so publishing `agent-flow` also reserves `agent_flow`.

WHAT `v0.0.2` SHIPPED, AND THE ONE CONSTRAINT THIS STEP INHERITS FROM IT. Published 2026-08-14, so the checklist's first three bullets are settled facts rather than instructions. The contact-route bullet said to open an issue on the "`agent-flow` GitHub repo", and no repository carries that name until this step runs. `ship-v0-0-2` settled it by naming the CURRENT repository instead: the published README says "To ask for it, open an issue on <https://github.com/nothingnesses/agent-scaffold>, the repository that will carry the `agent-flow` rename", and it links <https://crates.io/crates/agent-flow> for the crate.

THE CONSTRAINT IS THAT THE URL IS NOW UNEDITABLE WHERE IT COUNTS. `Cargo.toml` declares `readme = "README.md"`, so that sentence is inside the published 0.0.2 artifact, crates.io renders it as the crate's front page, and a published version cannot be revised. THIS STEP MUST THEREFORE LEAVE `https://github.com/nothingnesses/agent-scaffold` REACHING THE PROJECT'S ISSUE TRACKER. Check that URL after the rename rather than before it, and if the rename breaks it, that is a decision for the human and not a reason to edit a published artifact.

THE crates.io STATE, MEASURED AGAINST THE SPARSE INDEX ON 2026-08-14, because `crates.io/crates/<name>` over `curl` returns 404 for every crate and proves nothing: `agent-flow` carries 0.0.2 with `yanked: false`, and `agent-scaffold` carries 0.0.1 and 0.0.2, both with `yanked: false`. So the reservation is done, the old name stays installable and reclaimable, and the first real `agent-flow` release starts at 0.0.3 as the checklist says.

Folded from Q-65 (rename decision) and Q-64 (the identity conclusion that determines the name). Provenance: Q-65, Q-64.
