# agent-scaffold spec

Status: in progress. Name confirmed as `agent-scaffold`. Implementation Steps 1 to 4 are complete: the core assets, the file-dropper with two-tier ownership, the idempotency/safety pass, and the full selection UI (non-interactive flags plus the interactive two-pane ratatui TUI, `--interactive`, with undo/redo and a save-confirmation modal). Step 5 (TUI polish: search/filter and tag-based selection) is complete: its design was resolved (OQ-B/C/D adopted) and implemented as sub-steps 5a (the `Mode` enum refactor), 5b (non-interactive `tag:` selection), and 5c (the interactive Available filter); 5d (the optional include-all-visible action) was skipped by decision. Steps 6 to 9 are optional extras and not started. They were reordered so bring-your-own template support is Step 6 (it formalises the pack/manifest abstraction) and optional modules is Step 7; Step 6 is the next work and its design decisions are being worked. The implementation lives in the repo (`src/`, `pack/`); this plan is the durable context for resuming after a compaction.

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

No open questions remain. The Step 5 design questions (OQ-B mode representation, OQ-C search/filter model, OQ-D tag-based selection) are resolved and folded into Step 5 below with the adopted decisions and reasoning; the earlier questions (tool form, ownership and update model, principle set and data model, template source, non-interactive selection UI, interactive TUI design) were resolved in earlier steps and the code.

## Implementation Steps

Decisions carried from the resolved open questions:

- **Form.** A standalone Rust binary (clap CLI plus a ratatui TUI) that runs without Nix; Nix is a development-only dependency (the dev shell). Distributed via `cargo install` or prebuilt binaries, with an optional flake app for Nix users. Runs from any working directory and writes to any target via `--output-dir`, defaulting to the current directory with a namespaced layout.
- **Ownership (two-tier).** Tool-owned reference assets under `.agents/` are always refreshed; user working files (the root `AGENTS.md`, plans under `docs/plans/`) are create-if-absent, with `--force` to overwrite. An existing `AGENTS.md` is left untouched and the namespaced reference copy is refreshed for the user to merge from. Marked-block co-tenancy and a 3-way-merge `update` were considered and deferred (Step 9).
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

Status: complete. The non-interactive path: `--principles default|all|none|<ids>` selects the set and `--principle-detail name|summary|full` sets the rendering, both feeding generation and `--list-principles`. A non-interactive route and a recorded selection always remain available so the tool stays scriptable and idempotent.

The interactive TUI (ratatui, `src/tui.rs`) is built and launches on `--interactive`/`-i`, seeded from the resolved `--principles` set. First-pass design as built:

- **Scope: principles only.** Toggle inclusion and reorder principles. Modules are out of this pass, since they are specified but not yet built (Step 7); a module pane or mode is added when modules exist, reusing this pass's interaction code.
- **Layout: two-pane.** Left pane lists available (not included) principles; right pane lists included principles (numbered) in their chosen order. The focused pane is marked by border colour and the cursor highlight; the cursor wraps at both ends of a pane.
- **Key bindings.** `i`/`a` move the highlighted principle to the other pane, inserting it before (`i`) or after (`a`) that pane's cursor, with the cursor following the moved item. (This replaced an earlier Space/Shift+Space scheme: Shift+Space is not distinguishable from Space without the enhanced keyboard protocol, which is unreliable, for example on Alacritty; plain letter keys work everywhere and match vim's insert/append.) `Tab`/`BackTab`/left/right/`h`/`l` switch focus; up/down or `j`/`k` move the cursor; with the Included pane focused, Shift+up/down or `K`/`J` reorder the highlighted principle, overriding `default_order`. `u` undoes and `U` redoes. `/` opens the Available filter (type to narrow by name/id/tag, Enter applies and keeps it, Esc clears it; added in Step 5c). `Enter` opens the save modal; `q` aborts, and Esc aborts from editing but cancels the modal.
- **Undo/redo.** `u`/`U` step through whole-state snapshots. Each editing action (`i`/`a` move, `K`/`J` reorder) checkpoints the pre-edit state before mutating; a fresh edit clears the redo stack. Navigation (cursor, focus) does not checkpoint, so undo steps through edits rather than movement. Snapshots are valid states, so undo/redo cannot break the disjoint-partition invariant.
- **Detail footer.** A footer shows the highlighted principle's `name`, `summary`, and `tags`. No search/filter in this pass (48 principles scroll fine); filter is a clean later follow-up.
- **Save modal.** `Enter` is labelled save and opens a modal stating what saving will do (target directory, which files are created or refreshed, whether `--force` overwrites, and the included count) with Save and Cancel buttons. Focus defaults to Cancel so an accidental confirm never writes; left/right (or Tab/`h`/`l`) moves between buttons, Enter activates, Esc cancels. The modal is modal: editing keys are ignored while it is open. Choosing Save runs the scaffold and prints the ready-to-paste `--principles <id1,id2,...>` line (in the chosen order) for non-interactive replay; Cancel returns to editing; aborting writes nothing. `main` supplies the modal's summary lines (the file set is static; the target and `--force` behaviour come from the flags).
- **Dependencies.** `ratatui = "0.30"` (0.30.2 latest) with default features (`all-widgets`, `crossterm`, `macros`); the umbrella crate re-exports the workspace split (`ratatui-core`, `ratatui-crossterm`, `ratatui-widgets`), so no sub-crates are added directly. Events use the bundled crossterm (0.29) via `ratatui::crossterm`, not a separate dependency. Terminal lifecycle uses `ratatui::init()`/`restore()`; `init()` installs a panic hook that restores the terminal, so no hand-rolled hook is needed. A blocking `event::read()` loop is used, not async/tokio. MSRV is 1.88 (satisfied by the flake's `fenix-monthly latest`); the crate stays on edition 2021. void-installer (ratatui 0.28) was assessed and not reused: it has no toggle/reorder/multi-pane code and its binary is broken against its own WIP API; only the 0.28 terminal-lifecycle and `List`/`ListState` idioms carried over, and those are current in 0.30.
- **Internals (TEA-lite, functions not traits).** The selector is one concrete `App` value with the Elm split expressed as plain functions: `update(&mut App, Event) -> Step` (a pure-ish reducer, no IO, where `Step` is `Continue`/`Confirm`/`Abort`; unit-tested for move/reorder/undo/redo/save-modal/abort), `ui(&mut Frame, &mut App)` (a render function of state, no `Widget` impl on `App`), and a thin `run()` that wires read to update to draw. No custom traits and no generic harness; the reducer is tested by calling `update` with synthesised events, and ratatui's `Backend`/`TestBackend` is available for render tests, so no bespoke `OutputSink` is needed. This keeps the separation of concerns and testability that void-installer's generic trait framework was reaching for, without the generics.
- **Concurrency (sync now, one seam for later).** A blocking loop over a small `enum Event { Key(KeyEvent), Resize }` fed by one `next_event()` that currently wraps `event::read()` (filtering to key-press and resize events). No async, no tokio, no channel. `next_event()` is the single upgrade point: if a real concurrent producer ever lands (for example an isolation step streaming progress, or a live preview pane), it becomes a `std::sync::mpsc` receiver merging sources, still synchronous, without touching `update`/`ui`. This is void-installer's event-source decoupling minus its async runtime; full async stays off the table for a scaffolder.
- **Order round-trips (gap closed).** Reordering overrides `default_order`. `resolve_selection` was re-sorting every result through `ordered_by_default`, so an explicit id list lost the user's order; it now keeps the given order (only `default`/`all` sort by `default_order`), so the printed `--principles` line reproduces the TUI result. The `selection_modes_resolve` test asserts list order accordingly.

### 5. TUI polish and tag-based selection

Status: complete (5d skipped). Near-term selector improvements that build on the shipped TUI; each reuses the existing `App`/`update`/`ui` structure and the `next_event` seam. The sub-steps are ordered so each is validated before the next depends on it.

Decisions adopted from the resolved open questions:

- **Modes (from OQ-B).** A `Mode` enum (`Editing`, `Filtering`, `Confirming { button }`) replaces the `confirming` bool and `confirm_button`, so the modes are mutually exclusive by construction (make illegal states unrepresentable) and later modes do not multiply bools. The applied filter query lives on `App` (not inside `Filtering`) because its narrowing of the Available pane persists back in `Editing`; `Filtering` just means keystrokes edit that query, which also keeps `Mode` `Copy`.
- **Filter (from OQ-C).** The filter narrows the Available pane only, live and incremental, a case-insensitive substring over name, id, and tags, with a hand-rolled query string (no new dependency) and a visible-to-underlying index projection for the cursor and toggle. Included is never filtered, so reordering is never over a partial view.
- **Tags (from OQ-D).** `--principles` accepts `tag:<t>` tokens (bare tokens stay ids; `default`/`all`/`none` unchanged); each tag expands in `default_order`, the whole list is de-duplicated by first occurrence, and an unknown tag returns `SelectionError::UnknownTag`. Interactive tag selection reuses the filter (which matches tags), with an optional include-all-visible action on top.

Each sub-step below carries the evidence-grounding discipline: validate the adopted approach with a proof-of-concept (build plus tests plus, where relevant, a functional run); on failure fall back to the recorded next-best approach; if all are exhausted, raise the impasse rather than force an unvalidated approach.

#### 5a. Mode enum refactor

Status: complete. Replaced `confirming: bool` and `confirm_button` with `enum Mode { Editing, Confirming { button: Button } }`; `update` dispatches on `app.mode` and the focused button exists only inside `Confirming`. The `Filtering { query }` variant is introduced in 5c, where it is first used, so every commit stays free of unused-variant warnings. Validated by proof-of-concept: the 24 existing selector tests pass unchanged (retargeted from the old bool to `Mode`) and behaviour is identical (open/save/cancel, editing keys ignored while confirming); clippy `-D warnings` clean. The fallback (keep the bool with a `debug_assert`) was not needed.

#### 5b. Non-interactive tag selection

Status: complete. `resolve_selection` now treats `--principles` as a token list where each token is `default`/`all`/`none`, `tag:<t>`, or a bare id; tokens expand (keywords and tags in `default_order`, ids to themselves) and concatenate, de-duplicated by first occurrence, so a bare id list still preserves its round-trip order. `tag:<t>` naming a tag no principle carries returns `SelectionError::UnknownTag`. Validated by proof-of-concept: unit tests for tag expansion, keyword/id/tag composition with dedup and default-set-first ordering, unknown-tag error, a guard that no id collides with a reserved token, and the unchanged all/none/default/id-list behaviour; plus functional runs (`--principles tag:fp --list-principles` lists the two fp principles in order; `--principles tag:bogus` errors with exit 2). The fallback (a separate `--tags` flag) was not needed. (Since the binary now reads `tags`, the `Principle` struct's blanket `dead_code` allowance was narrowed to the still-unused `related` field.)

#### 5c. Interactive Available filter

Status: complete. `/` enters `Mode::Filtering`; the Available pane narrows live to a case-insensitive substring over name, id, and tags; typing appends to the query, Backspace deletes, Enter applies and returns to `Editing` (filter kept), Esc clears and returns. The Available cursor and toggle map through a visible-to-underlying projection (`available_visible`/`available_display`); Included is never filtered. Refinement: an item returned to Available while a filter is active is appended (its pool position is not user-meaningful, and "before/after cursor" is ambiguous under a projection); with no filter, behaviour is byte-for-byte unchanged, so all prior toggle tests pass. Validated by proof-of-concept: unit tests on the projection narrowing and persistence, tag matching, modality (editing keys type into the query), backspace/esc editing, and toggle-under-filter moving the correct principle and preserving the partition; clippy `-D warnings` clean, 33 tests pass. The interactive feel still wants a manual TTY run (not automatable here). The fallback (jump-to-first-match) was not needed.

#### 5d. Optional include-all-visible

Status: skipped (by decision). A key to move every currently-visible Available match into Included at once (tag-based bulk selection on top of the filter). Skipped because the 5c `/` filter already makes finding and adding matches quick (filter, then `i`/`a` on each), so bulk-add is not needed now; it stays minimal-by-default and can be revisited if adding many tagged principles at once becomes common. No `A` binding was added; the Step 4 key-bindings reference already includes `/` from 5c.

### 6. Bring-your-own template support

Status: not started (next; design decisions being worked). Support `--template <ref>`, where `ref` is a local path or a git URL (a Nix flake-ref is an optional extra for Nix users, not required, and the fetch must not depend on Nix), with a small manifest and minimal named-variable substitution; the built-in agent-workflow pack is the default. Reordered ahead of the optional modules (now Step 7) because it formalises the pack/manifest abstraction that modules will slot into, avoiding a later retrofit.

### 7. Optional modules

Status: not started. Package the confirmed modules as opt-in selections, each self-contained, none complicating the core. Content modules (extra assets) are expected to live in the pack manifest introduced by Step 6; behavioural modules (for example container/worktree isolation) stay tool features toggled by flags rather than pack content.

### 8. Optional greenfield flake template

Status: not started. Expose a `nix flake new` template as a convenience for the new-project case, reusing the same core assets.

### 9. Optional later enhancements

Status: not started. Marked-block augmentation of an existing `AGENTS.md`, and an opt-in merge `update` command (3-way merge), if the create-or-overwrite model proves too blunt in practice.

## Success Criteria

- One command, run from any working directory and targeting any `--output-dir`, drops the minimal core (a usable `AGENTS.md`, a planning-document template, and the core prompts) into both an empty directory and an existing repository.
- By default it creates only absent files and never modifies existing ones; `--force` is required to overwrite; a default re-run therefore reverts nothing.
- The user can choose which principles and modules to include, interactively with sane defaults or non-interactively via flags or config.
- The dropped assets are immediately usable to run one pass of the workflow (plan -> review -> implement -> review) without further setup.
- Optional modules can be added without touching or complicating the core.
- The tool can scaffold from a user-supplied template pack, not only the built-in one.
