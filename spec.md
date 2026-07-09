# agent-scaffold spec

Status: in progress. Name confirmed as `agent-scaffold`. Implementation Steps 1 to 3 are complete and the selection UI's non-interactive path (Step 4) is done. The interactive TUI design is now decided (see Step 4) and is the next work to build. The implementation lives in the repo (`src/`, `pack/`); this plan is the durable context for resuming after a compaction.

This document plans a tool that scaffolds the agent workflow (front-load context -> structured plan -> iterative and adversarial review -> isolated implementation -> adversarial review) into a project, so the structure does not have to be hand-rolled each time. It follows the same planning format the tool is meant to scaffold.

## Motivations

- Setting up the workflow by hand for each project is repetitive: the same planning-document skeleton, the same guidance/principles, the same reusable prompts, and (when wanted) the same worktree and container isolation.
- The setup should work in two modes: starting a new project, and adding to or editing an existing one. A greenfield-only template does not cover the second mode.
- The default should be minimal. Extra machinery (diagram prompt packs, container isolation, language starters) should be opt-in rather than imposed.
- Guidance should be harness-agnostic: `AGENTS.md` is the canonical file, with any harness-specific file (for example `CLAUDE.md`) reduced to a thin pointer to it.

## Project Principles

These govern how this tool is built.

1. Prefer the cleaner long-term architecture over the smallest diff: prioritise correctness, internal coherence, and maintainability, and when a local fix conflicts with a cleaner design, choose the cleaner design unless a concrete limitation prevents it.
2. Minimal by default. The core does one thing well; everything else is an optional module the user opts into. Adding a module must not complicate the core.
3. Safe on existing projects. Scaffolding into a populated repo must never clobber or silently overwrite existing files. Prefer create-if-absent, namespaced locations, and clearly marked idempotent edits.
4. Idempotent. Running the tool twice produces the same result as running it once, with no duplicated blocks and no drift.
5. Make illegal states unrepresentable. Work out the valid inputs and outcomes first and encode them, rather than admitting bad states and guarding against them at runtime.
6. Ground decisions in evidence. Before an approach is built out, validate it with a small proof-of-concept that builds and produces the expected output. If the candidates are exhausted, raise the impasse for a decision rather than forcing through an unvalidated approach.
7. Reproducible. Prefer the project's existing toolchain conventions (Nix, just, direnv) so a scaffolded project and its checks behave the same on any machine.

Note: the principles the tool _ships_ to other projects (the default `AGENTS.md` content) are a separate artifact from these; the shipped set and its data model are covered in Implementation Step 1 and live in `pack/principles.toml`.

## Documentation Protocol

This plan is kept current during the work. Each Implementation Step carries a `Status:` line (`not started`, `in progress`, `blocked on <x>`, `complete`). Resolved Open Questions are removed from that section and folded into the relevant step as the adopted decision plus reasoning, rather than left as a pointer, so the document does not accumulate stale decision history. When no questions remain open, the section says so.

## Open Questions, Decisions, Issues and Blockers

No open questions remain. The earlier open questions (the tool's form, the ownership and update model, the shipped principle set and its data model, the template source, the non-interactive selection UI, and the interactive TUI design) are all resolved and folded into the Implementation Steps below and into the code.

## Implementation Steps

Decisions carried from the resolved open questions:

- **Form.** A standalone Rust binary (clap CLI plus a ratatui TUI) that runs without Nix; Nix is a development-only dependency (the dev shell). Distributed via `cargo install` or prebuilt binaries, with an optional flake app for Nix users. Runs from any working directory and writes to any target via `--output-dir`, defaulting to the current directory with a namespaced layout.
- **Ownership (two-tier).** Tool-owned reference assets under `.agents/` are always refreshed; user working files (the root `AGENTS.md`, plans under `docs/plans/`) are create-if-absent, with `--force` to overwrite. An existing `AGENTS.md` is left untouched and the namespaced reference copy is refreshed for the user to merge from. Marked-block co-tenancy and a 3-way-merge `update` were considered and deferred (Step 8).
- **Template source.** A built-in default pack, with optional `--template <ref>` (a local path or a git URL; the fetch must not depend on Nix, and a flake-ref is only an optional extra) for bring-your-own packs.
- **Principle data.** Structured TOML (`pack/principles.toml`). Each principle has `id`, `name`, `summary`, `rationale`, `tags` (category tags plus applicability tags `universal`/`static-types`/`fp`/`oop`), `default_selected`, `default_order`, and optional `references`/`related`. The selected set renders as a numbered list ordered by `default_order`, at `name`, `summary`, or `full` detail. The default set is about 21 principles, applicability-tagged so it adapts to the project type; the shipped prompts and guidance stay principle-agnostic and defer to the selected set.
- **Optional modules (not yet built).** A diagram prompt pack, container and worktree isolation (integrating agent-box and agent-images), a justfile of recipes, and language starters.

### 1. Author the minimal core assets and principle data

Status: complete. Wrote the canonical `AGENTS.md` guidance (with a `{{principles}}` placeholder), the plan-document template, the three core prompts, and the principle data as structured TOML (48 principles, 21 default), parsed and tested. Content policy: the shipped prompts and guidance stay principle-agnostic and defer to the user-selected principle set in `AGENTS.md`, rather than baking in specific opinions a user may not have chosen (the open-questions gate was corrected to follow this, and the prompts instruct the agent to read `AGENTS.md` to align with the current principles).

### 2. Proof-of-concept the file-dropper

Status: complete. The Rust binary drops the asset set under `--output-dir`, refreshing tool-owned reference assets under `.agents/`, creating absent working files (root `AGENTS.md`, `docs/plans/TEMPLATE.md`), and leaving existing working files untouched unless `--force` is given. `AGENTS.md` is generated by rendering the selected principles into the pack guidance template. Covered by unit tests.

### 3. Idempotency and safety pass

Status: complete. Re-run refreshes reference assets and skips existing working files (tests cover create, refresh-versus-skip, and `--force` overwrite); `--output-dir` targets any directory.

### 4. Selection UI

Status: in progress. The non-interactive path is done: `--principles default|all|none|<ids>` selects the set and `--principle-detail name|summary|full` sets the rendering, both feeding generation and `--list-principles`. A non-interactive route and a recorded selection must always remain available so the tool stays scriptable and idempotent.

The interactive TUI (ratatui) is designed and not yet built. First-pass design, decided with the user:

- **Scope: principles only.** Toggle inclusion and reorder principles. Modules are out of this pass, since they are specified but not yet built (Step 5); a module pane or mode is added when modules exist, reusing this pass's interaction code.
- **Layout: two-pane.** Left pane lists available (not included) principles; right pane lists included principles in their chosen order. Space/Enter toggles the highlighted item between panes; Tab (or left/right) switches focus; up/down moves the cursor; with the included pane focused, a reorder binding (Shift+up/down, or `K`/`J`) moves the highlighted principle up or down, overriding `default_order` for the project.
- **Detail footer.** A one- or two-line footer shows the highlighted principle's `summary` and `tags`. No search/filter in this pass (48 principles scroll fine); filter is a clean later follow-up.
- **Confirm and abort.** Enter-to-confirm runs the scaffold immediately with the chosen selection and order, and prints the resolved selection as a ready-to-paste `--principles <id1,id2,...>` line (in the chosen order) for non-interactive replay. Esc/`q` aborts and writes nothing.
- **Dependencies.** `ratatui = "0.30"` (0.30.2 latest) with default features (`all-widgets`, `crossterm`, `macros`); the umbrella crate re-exports the workspace split (`ratatui-core`, `ratatui-crossterm`, `ratatui-widgets`), so no sub-crates are added directly. Events use the bundled crossterm (0.29) via `ratatui::crossterm`, not a separate dependency. Terminal lifecycle uses `ratatui::init()`/`restore()`; `init()` installs a panic hook that restores the terminal, so no hand-rolled hook is needed. A blocking `event::read()` loop is used, not async/tokio. MSRV is 1.88 (satisfied by the flake's `fenix-monthly latest`); the crate stays on edition 2021. void-installer (ratatui 0.28) was assessed and not reused: it has no toggle/reorder/multi-pane code and its binary is broken against its own WIP API; only the 0.28 terminal-lifecycle and `List`/`ListState` idioms carried over, and those are current in 0.30.
- **Internals (TEA-lite, functions not traits).** The selector is one concrete `App` value with the Elm split expressed as plain functions: `update(&mut App, Event) -> ControlFlow` (a pure-ish reducer, no IO, unit-tested for toggle/reorder/confirm), `ui(frame, &App)` (a pure render function, no `Widget` impl on `App`), and a thin `run()` that wires read to update to draw. No custom traits and no generic harness; test doubles come from ratatui's `Backend`/`TestBackend`, not a bespoke `OutputSink`. This keeps the separation of concerns and testability that void-installer's generic trait framework was reaching for, without the generics.
- **Concurrency (sync now, one seam for later).** A blocking loop over a small `enum Event { Key(KeyEvent), Resize(u16, u16) }` fed by one `next_event()` that currently wraps `event::read()`. No async, no tokio, no channel. `next_event()` is the single upgrade point: if a real concurrent producer ever lands (for example an isolation step streaming progress, or a live preview pane), it becomes a `std::sync::mpsc` receiver merging sources, still synchronous, without touching `update`/`ui`. This is void-installer's event-source decoupling minus its async runtime; full async stays off the table for a scaffolder.
- **Order round-trips (implementation gap to close in this step).** Reordering overrides `default_order`, but `resolve_selection` currently re-sorts every result through `ordered_by_default`, so an explicit id list would not preserve the user's order. Change the comma-list path in `resolve_selection` to keep the given order (only `default`/`all` sort by `default_order`), so the printed `--principles` line reproduces the TUI result. The `selection_modes_resolve` test's ordered-id assertion flips from `default_order` to list order accordingly.

### 5. Optional modules

Status: not started. Package the confirmed modules as opt-in selections, each self-contained, none complicating the core.

### 6. Bring-your-own template support

Status: not started. Support `--template <ref>`, where `ref` is a local path or a git URL (a Nix flake-ref is an optional extra for Nix users, not required, and the fetch must not depend on Nix), with a small manifest and minimal named-variable substitution; the built-in agent-workflow pack is the default.

### 7. Optional greenfield flake template

Status: not started. Expose a `nix flake new` template as a convenience for the new-project case, reusing the same core assets.

### 8. Optional later enhancements

Status: not started. Marked-block augmentation of an existing `AGENTS.md`, and an opt-in merge `update` command (3-way merge), if the create-or-overwrite model proves too blunt in practice.

## Success Criteria

- One command, run from any working directory and targeting any `--output-dir`, drops the minimal core (a usable `AGENTS.md`, a planning-document template, and the core prompts) into both an empty directory and an existing repository.
- By default it creates only absent files and never modifies existing ones; `--force` is required to overwrite; a default re-run therefore reverts nothing.
- The user can choose which principles and modules to include, interactively with sane defaults or non-interactively via flags or config.
- The dropped assets are immediately usable to run one pass of the workflow (plan -> review -> implement -> review) without further setup.
- Optional modules can be added without touching or complicating the core.
- The tool can scaffold from a user-supplied template pack, not only the built-in one.
