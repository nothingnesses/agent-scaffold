# agent-scaffold spec

Status: in progress; see the Roadmap for per-step status and order. In short: the core is complete (the assets and principle data, the file-dropper with two-tier ownership, the idempotency and write-safety pass, the selection UI and its polish, and bring-your-own-template support), `init-vcs` is the prioritised next step, and the remaining steps are optional. No open questions remain. The implementation lives in the repo (`src/`, `pack/`); this spec is the durable context for resuming after a compaction, and the "Repository Layout and Current Architecture" section maps the shipped code so a fresh implementor can continue without prior context. Verification convention: `cargo clippy --all-targets -- -D warnings`, `nix fmt`, and ASCII-clean before each commit.

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

Note: the principles the tool _ships_ to other projects (the default `AGENTS.md` content) are a separate artifact from these; the shipped set and its data model are covered in the `core-assets` step and live in `pack/principles.toml`.

## Documentation Protocol

This plan is kept current during the work.

Progress lives in the Roadmap: an ordered table of step slugs and their status (`not started`, `in progress`, `complete`, `skipped`, `next`, `optional`, `deferred`, or `blocked on <slug>`). The Roadmap is the single source of truth for status and for implementation order. The Step Details below carry each step's design, decisions, and (once done) outcome and evidence, and do not repeat the status label.

Steps are identified by a stable kebab-case slug, not a number, so a reference stays valid when the order changes: every cross-reference to a step uses its slug. Each Roadmap row maps to a detail block headed by the same slug; some detail blocks are grouped under an umbrella heading that carries context shared by several steps, and the umbrella itself is not a Roadmap entry. To add a step, add its detail block and insert its slug into the Roadmap at the right position; to reorder or reprioritise, edit the Roadmap rows. Keep the two in sync: every Roadmap slug has a detail block and vice versa.

Resolved Open Questions are folded into the relevant step as the adopted decision plus reasoning, rather than left as a pointer, so the document does not accumulate stale decision history. When no questions remain open, the section says so.

## Repository Layout and Current Architecture

This section maps the code as built, so the plan can be resumed without prior conversation context. The code in the repository is the source of truth; this is an index into it. The completed steps carry the reasoning; this carries the shape.

Build, run, and verify (see `README.md` for full user-facing usage):

- Use `just build` / `just test` / `just clippy` / `just fmt`, or plain `cargo` inside `nix develop` (with direnv, run `direnv allow` once). The justfile wraps the Nix environment.
- Verification before each commit: `cargo clippy --all-targets -- -D warnings`, `nix fmt`, and all text ASCII-clean (no em-dashes, en-dashes, emoji, or other non-ASCII; use `->`, `>=`, `!=`).
- The binary is a plain Rust program (edition 2021, MSRV 1.88) that runs without Nix. Nix, just, and direnv are development conveniences only. Dependencies: `clap` (derive), `serde`, `toml`, `ratatui = "0.30"`, `include_dir`.

Directory layout:

- `src/` holds the binary crate: four modules described below.
- `pack/` is the built-in default pack: `pack.toml` (the manifest), `AGENTS.md`, `plan-template.md`, `principles.toml`, and `prompts/`. It is embedded into the binary at compile time with `include_dir`, and it also loads through the same manifest path as an external `--template` pack (the pack is dogfooded).
- `flake.nix`, `.envrc`, `justfile`, `rustfmt.toml` are the dev environment (fenix toolchain, treefmt formatters, recipes). `LICENSE` is Blue Oak Model License 1.0.0.

Modules and key types (all shipped and tested):

- `src/main.rs` (CLI and orchestration). `Cli` (clap derive) exposes `--output-dir`, `--force`, `--write`, `--dry-run` (conflicts with `--write`), `--principles`, `--principle-detail`, `--list-principles`, `--template <dir>`, and `--var key=value` (repeatable); there is no `-i` flag, the selector is the default on a terminal. `main` builds the active `manifest::PackSource` (the built-in pack, or a `--template` directory), parses `--var` into an overrides map, loads the active pack's principles via `pack_principles`, and resolves the selection. `use_selector` then decides the mode: the two-pane TUI opens when both stdin and stdout are terminals, there are principles to choose, and neither `--write` nor `--dry-run` was given; its Save modal is the write confirmation. Otherwise the run is non-interactive and writes only with `--write`. Either way `main` builds the asset set with `build_assets`, computes each asset's `Outcome` with `outcome_of` (read-only, for the plan preview), prints the plan, and writes with `apply_asset` only when confirmed. `Outcome` is `Created`/`Refreshed`/`SkippedExisting`/`Overwritten`; `outcome_of` decides it without touching disk and `apply_asset` performs the write, so the preview never writes. `#[cfg(test)] scaffold` is a build-plus-write convenience for the drop tests. User errors print to stderr and exit 2.
- `src/pack.rs` (principle data and selection). `Principle { id, name, summary, rationale, tags, default_selected, default_order, references, related }`. `parse_principles(&str)` parses a principles TOML file. `resolve_selection(principles, spec)` implements the `--principles` token grammar (comma-separated `default` / `all` / `none` / `tag:<t>` / bare-id, de-duplicated by first occurrence; keyword and tag tokens expand in `default_order`, id lists keep their given order) and returns `SelectionError { UnknownId, UnknownTag }`. `Detail { Name, Summary, Full }` with `render_principles` renders the selected set as a numbered list. `default_principles` / `DEFAULT_PRINCIPLES_TOML` are `#[cfg(test)]` only, because production reads principles through the active pack source (see `pack_principles`).
- `src/manifest.rs` (the pack format and loader; the abstraction `optional-modules` builds on). `PackSource { Embedded(&Dir), Directory(PathBuf) }` with a public `read(rel)` and a private `manifest()`; `builtin()` returns the embedded pack. `Manifest { asset: Vec<AssetSpec>, var: Vec<VarSpec> }` deserializes `pack.toml` and ignores unknown keys, so a future `[[module]]` section is forward-compatible. `AssetSpec { source, dest, ownership, render }`, `VarSpec { name, default: Option<String> }`, and `Ownership { Reference, Working }`. `render(template, vars)` does minimal `{{key}}` substitution (unknown placeholders left as-is). `load(source, builtin_vars, overrides)` reads the manifest, resolves the variable map with `resolve_vars` (built-in vars first, then each declared default, then `--var` overrides; the name `principles` is reserved), and returns `Vec<Asset { dest, contents, ownership }>` or `LoadError { Io, UndeclaredVar, MissingRequiredVar, ReservedVar }` (all exit 2).
- `src/tui.rs` (the interactive selector; TEA-lite, functions not traits, where TEA is The Elm Architecture: a single state value, a pure `update` reducer, and a render function). The only public item is `run_selection(principles, initial_included, save_summary) -> io::Result<Option<Vec<usize>>>`, which returns the chosen principle indices in order, or `None` on abort. Internally: one `App` value; `enum Mode { Editing, Filtering, Confirming { button } }`; `enum Event { Key, Resize }`; a pure `update(&mut App, Event) -> Step` reducer (`Step` is `Continue`/`Confirm`/`Abort`); a `ui` render function; and a blocking `run` loop over `next_event()` (the single seam where a concurrent event source could later be merged). No async, no custom traits. Key bindings are documented in the `selection-ui` step.

Pack format (the contract shared by the built-in pack and `--template` packs):

- A pack is a directory containing `pack.toml` plus the source files it references. External packs are passed with `--template <dir>`.
- `pack.toml` has `[[asset]]` entries `{ source, dest, ownership = "reference"|"working", render = true|false }` (one source may map to several assets; `render` defaults to false) and optional `[[var]]` entries `{ name, default? }` (an absent `default` makes the variable required).
- `ownership = "reference"` assets (conventionally under `.agents/`) are always (re)written; `ownership = "working"` assets are created only if absent, unless `--force`.
- `render = true` applies `{{name}}` substitution. `{{principles}}` is a reserved, tool-computed variable (the rendered selection); a pack may neither declare nor `--var`-set it. Other variables come from `[[var]]` defaults, overridden by `--var key=value`; setting an undeclared variable, or leaving a required one unset, is an error and nothing is written.
- A pack that ships `principles.toml` has its principles drive selection, `--list-principles`, the interactive selector, and `{{principles}}`; a pack without one simply has no principles to select.

One run's data flow: parse CLI, build the `PackSource`, read and parse the pack's `principles.toml`, `resolve_selection` (or the interactive TUI) chooses and orders principles, `render_principles` produces the `{{principles}}` value, `manifest::load` resolves the variables and reads/renders each asset, `outcome_of` computes each asset's plan outcome, and, once writing is confirmed, `apply_asset` drops each honouring ownership and `--force`.

## Open Questions, Decisions, Issues and Blockers

No open questions remain. OQ-K (initialising a version-control repository) was resolved and folded into `init-vcs`. Earlier questions were likewise folded into their steps: the questions OQ-I (variable declaration schema) and OQ-J (principle source under `--template`) are in `external-packs` and `pack-owned-principles` (OQ-I adopted a `[[var]]` array-of-tables schema; OQ-J adopted pack-owned principles, split into a built-in-principles interim in `external-packs` and pack-owned principles in `pack-owned-principles`); OQ-E (manifest and dogfooding), OQ-F (schema and variables), and OQ-G (embedding) are in the bring-your-own-template steps; OQ-H (git-URL fetch) was deferred to `git-url-fetch`; and the pre-template questions (tool form, ownership and update model, principle set and data model, non-interactive selection UI, interactive TUI design, and the selection-polish questions OQ-B/C/D) were resolved in earlier steps and the code.

Note on labels: `OQ-<letter>` (for example OQ-B or OQ-H) are historical identifiers for open questions that have since been resolved and folded into their steps. They are provenance markers, not live entries, so a reference such as "from OQ-E" points to a decision now recorded inline in the relevant step, not to a separate section here.

## Roadmap

Steps in implementation order, with status. The Roadmap is the single source of truth for status; the slug in each row keys the matching detail block under "Step Details". `next` marks the prioritised next work; `optional` and `deferred` mark not-started work that is not on the critical path.

| Step                    | Status   |
| ----------------------- | -------- |
| `core-assets`           | complete |
| `file-dropper`          | complete |
| `idempotency-safety`    | complete |
| `selection-ui`          | complete |
| `mode-enum`             | complete |
| `tag-selection`         | complete |
| `available-filter`      | complete |
| `include-all-visible`   | skipped  |
| `pack-manifest`         | complete |
| `external-packs`        | complete |
| `pack-owned-principles` | complete |
| `init-vcs`              | next     |
| `optional-modules`      | optional |
| `greenfield-flake`      | optional |
| `later-enhancements`    | optional |
| `git-url-fetch`         | deferred |
| `tui-authoring`         | optional |

## Step Details

Decisions carried from the resolved open questions:

- **Form.** A standalone Rust binary (clap CLI plus a ratatui TUI) that runs without Nix; Nix is a development-only dependency (the dev shell). Distributed via `cargo install` or prebuilt binaries, with an optional flake app for Nix users. Runs from any working directory and writes to any target via `--output-dir`, defaulting to the current directory with a namespaced layout.
- **Ownership (two-tier).** Tool-owned reference assets under `.agents/` are always refreshed; user working files (the root `AGENTS.md`, plans under `docs/plans/`) are create-if-absent, with `--force` to overwrite. An existing `AGENTS.md` is left untouched and the namespaced reference copy is refreshed for the user to merge from. Marked-block co-tenancy and a 3-way-merge `update` were considered and deferred (`later-enhancements`).
- **Template source.** A built-in default pack, with optional `--template <local-path>` for bring-your-own packs (`byo-template`); git-URL fetch is deferred to `git-url-fetch` (must not depend on Nix) and a flake-ref stays out of scope.
- **Principle data.** Structured TOML (`pack/principles.toml`). Each principle has `id`, `name`, `summary`, `rationale`, `tags` (category tags plus applicability tags `universal`/`static-types`/`fp`/`oop`), `default_selected`, `default_order`, and optional `references`/`related`. The selected set renders as a numbered list ordered by `default_order`, at `name`, `summary`, or `full` detail. The default set is about two dozen principles, applicability-tagged so it adapts to the project type (see `pack/principles.toml` for the live set and counts); the shipped prompts and guidance stay principle-agnostic and defer to the selected set.
- **Optional modules (not yet built).** A diagram prompt pack, a justfile of recipes, language starters, and container/worktree isolation. Each is defined concretely in `optional-modules`; the isolation module would integrate the separate agent-box (`https://github.com/0xferrous/agent-box`) and agent-images (`https://github.com/nothingnesses/agent-images`) projects, which are not part of this repository.

### `core-assets`: Author the minimal core assets and principle data

Wrote the canonical `AGENTS.md` guidance (with a `{{principles}}` placeholder), the plan-document template, the three core prompts, and the principle data as structured TOML (initially 48 principles, 21 default; see `pack/principles.toml` for the current set, which has grown since), parsed and tested. Content policy: the shipped prompts and guidance stay principle-agnostic and defer to the user-selected principle set in `AGENTS.md`, rather than baking in specific opinions a user may not have chosen (the open-questions gate was corrected to follow this, and the prompts instruct the agent to read `AGENTS.md` to align with the current principles). The workflow's two review steps (in `AGENTS.md`, supported by `adversarial-review.md`) instruct spawning separate, independent reviewer sub-agents where the harness supports them, handing each the change context (for the work review, the before and after commit hashes or the diff range) and an assume-there-are-issues stance, and forbidding the parent from sharing its own opinions so each reviewer investigates without inherited bias.

### `file-dropper`: Proof-of-concept the file-dropper

The Rust binary drops the asset set under `--output-dir`, refreshing tool-owned reference assets under `.agents/`, creating absent working files (root `AGENTS.md`, `docs/plans/TEMPLATE.md`), and leaving existing working files untouched unless `--force` is given. `AGENTS.md` is generated by rendering the selected principles into the pack guidance template. Covered by unit tests.

### `idempotency-safety`: Idempotency and safety pass

Re-run refreshes reference assets and skips existing working files (tests cover create, refresh-versus-skip, and `--force` overwrite); `--output-dir` targets any directory.

Write safety and default mode (added later): writes are off unless confirmed, and the interactive selector is the default on a terminal. A run always builds the asset set and prints a plan preview (a present-tense line per asset: `create` / `refresh` / `skip (exists)` / `overwrite`). The mode is chosen by `use_selector`: when both stdin and stdout are terminals, there are principles to choose, and neither `--write` nor `--dry-run` was given, the two-pane selector opens and its Save modal is the confirmation (Save writes, Cancel or abort writes nothing). `--write` takes the non-interactive path and applies the changes (using `--principles`), skipping the selector; `--dry-run` prints the plan and exits without writing (it conflicts with `--write`); and with no terminal and no `--write` (a pipe or CI) the run previews and writes nothing. There is no `-i` flag and no y/N prompt; the selector's modal replaced the prompt. `--force` stays orthogonal (it decides overwrite-versus-skip for existing working files; `--write` decides whether to write at all). Implementation: `outcome_of` computes each asset's outcome without touching the filesystem (so the preview never writes), and `apply_asset` performs the write; terminal detection uses `std::io::IsTerminal` (no new dependency). Validated by a unit test that planning creates nothing, plus functional runs of the no-flags preview, `--write` (byte-identical to the pre-change golden), `--dry-run`, and the `--dry-run`-plus-`--write` conflict (exit 2). The auto-launch of the selector on a terminal needs a manual TTY run (not automatable here).

### `selection-ui`: Selection UI

The non-interactive path: `--principles default|all|none|<ids>` selects the set and `--principle-detail name|summary|full` sets the rendering, both feeding generation and `--list-principles`. A non-interactive route and a recorded selection always remain available so the tool stays scriptable and idempotent.

The interactive TUI (ratatui, `src/tui.rs`) is built and, since the later default-mode change, opens by default on an interactive terminal (there is no `-i` flag; `--write` and `--dry-run` take the non-interactive path), seeded from the resolved `--principles` set. First-pass design as built:

- **Scope: principles only.** Toggle inclusion and reorder principles. Modules are out of this pass, since they are specified but not yet built (`optional-modules`); a module pane or mode is added when modules exist, reusing this pass's interaction code.
- **Layout: two-pane.** Left pane lists available (not included) principles; right pane lists included principles (numbered) in their chosen order. The focused pane is marked by border colour and the cursor highlight; the cursor wraps at both ends of a pane.
- **Key bindings.** `i`/`a` move the highlighted principle to the other pane, inserting it before (`i`) or after (`a`) that pane's cursor, with the cursor following the moved item. (This replaced an earlier Space/Shift+Space scheme: Shift+Space is not distinguishable from Space without the enhanced keyboard protocol, which is unreliable, for example on Alacritty; plain letter keys work everywhere and match vim's insert/append.) `Tab`/`BackTab`/left/right/`h`/`l` switch focus; up/down or `j`/`k` move the cursor; with the Included pane focused, Shift+up/down or `K`/`J` reorder the highlighted principle, overriding `default_order`. `u` undoes and `U` redoes. `/` opens the Available filter (type to narrow by name/id/tag, Enter applies and keeps it, Esc clears it; added in `available-filter`). `Enter` opens the save modal; `q` aborts, and Esc aborts from editing but cancels the modal.
- **Undo/redo.** `u`/`U` step through whole-state snapshots. Each editing action (`i`/`a` move, `K`/`J` reorder) checkpoints the pre-edit state before mutating; a fresh edit clears the redo stack. Navigation (cursor, focus) does not checkpoint, so undo steps through edits rather than movement. Snapshots are valid states, so undo/redo cannot break the disjoint-partition invariant.
- **Detail footer.** A footer shows the highlighted principle's `name`, `summary`, and `tags`. No search/filter in this pass (the few dozen principles scroll fine); filter is a clean later follow-up.
- **Save modal.** `Enter` is labelled save and opens a modal stating what saving will do (target directory, which files are created or refreshed, whether `--force` overwrites, and the included count) with Save and Cancel buttons. Focus defaults to Cancel so an accidental confirm never writes; left/right (or Tab/`h`/`l`) moves between buttons, Enter activates, Esc cancels. The modal is modal: editing keys are ignored while it is open. Choosing Save runs the scaffold and prints the ready-to-paste `--principles <id1,id2,...>` line (in the chosen order) for non-interactive replay; Cancel returns to editing; aborting writes nothing. `main` supplies the modal's summary lines (the file set is static; the target and `--force` behaviour come from the flags).
- **Dependencies.** `ratatui = "0.30"` (0.30.2 latest) with default features (`all-widgets`, `crossterm`, `macros`); the umbrella crate re-exports the workspace split (`ratatui-core`, `ratatui-crossterm`, `ratatui-widgets`), so no sub-crates are added directly. Events use the bundled crossterm (0.29) via `ratatui::crossterm`, not a separate dependency. Terminal lifecycle uses `ratatui::init()`/`restore()`; `init()` installs a panic hook that restores the terminal, so no hand-rolled hook is needed. A blocking `event::read()` loop is used, not async/tokio. MSRV is 1.88 (satisfied by the flake's `fenix-monthly latest`); the crate stays on edition 2021. void-installer (an earlier ratatui 0.28 TUI prototype in a separate repository, not part of this one) was assessed and not reused: it has no toggle/reorder/multi-pane code and its binary is broken against its own WIP API; only the 0.28 terminal-lifecycle and `List`/`ListState` idioms carried over, and those are current in 0.30.
- **Internals (TEA-lite, functions not traits).** The selector is one concrete `App` value with the Elm split expressed as plain functions: `update(&mut App, Event) -> Step` (a pure-ish reducer, no IO, where `Step` is `Continue`/`Confirm`/`Abort`; unit-tested for move/reorder/undo/redo/save-modal/abort), `ui(&mut Frame, &mut App)` (a render function of state, no `Widget` impl on `App`), and a thin `run()` that wires read to update to draw. No custom traits and no generic harness; the reducer is tested by calling `update` with synthesised events, and ratatui's `Backend`/`TestBackend` is available for render tests, so no bespoke `OutputSink` is needed. This keeps the separation of concerns and testability that void-installer's generic trait framework was reaching for, without the generics.
- **Concurrency (sync now, one seam for later).** A blocking loop over a small `enum Event { Key(KeyEvent), Resize }` fed by one `next_event()` that currently wraps `event::read()` (filtering to key-press and resize events). No async, no tokio, no channel. `next_event()` is the single upgrade point: if a real concurrent producer ever lands (for example an isolation step streaming progress, or a live preview pane), it becomes a `std::sync::mpsc` receiver merging sources, still synchronous, without touching `update`/`ui`. This is void-installer's event-source decoupling minus its async runtime; full async stays off the table for a scaffolder.
- **Order round-trips (gap closed).** Reordering overrides `default_order`. `resolve_selection` was re-sorting every result through `ordered_by_default`, so an explicit id list lost the user's order; it now keeps the given order (only `default`/`all` sort by `default_order`), so the printed `--principles` line reproduces the TUI result. The `selection_modes_resolve` test asserts list order accordingly.

### TUI polish and tag-based selection

Shared context for the four steps below (`mode-enum`, `tag-selection`, `available-filter`, `include-all-visible`); this umbrella is not itself a Roadmap entry. These are near-term selector improvements that build on the shipped TUI; each reuses the existing `App`/`update`/`ui` structure and the `next_event` seam. The sub-steps are ordered so each is validated before the next depends on it.

Decisions adopted from the resolved open questions:

- **Modes (from OQ-B).** A `Mode` enum (`Editing`, `Filtering`, `Confirming { button }`) replaces the `confirming` bool and `confirm_button`, so the modes are mutually exclusive by construction (make illegal states unrepresentable) and later modes do not multiply bools. The applied filter query lives on `App` (not inside `Filtering`) because its narrowing of the Available pane persists back in `Editing`; `Filtering` just means keystrokes edit that query, which also keeps `Mode` `Copy`.
- **Filter (from OQ-C).** The filter narrows the Available pane only, live and incremental, a case-insensitive substring over name, id, and tags, with a hand-rolled query string (no new dependency) and a visible-to-underlying index projection for the cursor and toggle. Included is never filtered, so reordering is never over a partial view.
- **Tags (from OQ-D).** `--principles` accepts `tag:<t>` tokens (bare tokens stay ids; `default`/`all`/`none` unchanged); each tag expands in `default_order`, the whole list is de-duplicated by first occurrence, and an unknown tag returns `SelectionError::UnknownTag`. Interactive tag selection reuses the filter (which matches tags), with an optional include-all-visible action on top.

Each sub-step below carries the evidence-grounding discipline: validate the adopted approach with a proof-of-concept (build plus tests plus, where relevant, a functional run); on failure fall back to the recorded next-best approach; if all are exhausted, raise the impasse rather than force an unvalidated approach.

#### `mode-enum`: Mode enum refactor

Replaced `confirming: bool` and `confirm_button` with `enum Mode { Editing, Confirming { button: Button } }`; `update` dispatches on `app.mode` and the focused button exists only inside `Confirming`. The `Filtering` variant is introduced in `available-filter`, where it is first used, so every commit stays free of unused-variant warnings (`available-filter` added it as a unit variant with the query on `App`). Validated by proof-of-concept: the 24 existing selector tests pass unchanged (retargeted from the old bool to `Mode`) and behaviour is identical (open/save/cancel, editing keys ignored while confirming); clippy `-D warnings` clean. The fallback (keep the bool with a `debug_assert`) was not needed.

#### `tag-selection`: Non-interactive tag selection

`resolve_selection` now treats `--principles` as a token list where each token is `default`/`all`/`none`, `tag:<t>`, or a bare id; tokens expand (keywords and tags in `default_order`, ids to themselves) and concatenate, de-duplicated by first occurrence, so a bare id list still preserves its round-trip order. `tag:<t>` naming a tag no principle carries returns `SelectionError::UnknownTag`. Validated by proof-of-concept: unit tests for tag expansion, keyword/id/tag composition with dedup and default-set-first ordering, unknown-tag error, a guard that no id collides with a reserved token, and the unchanged all/none/default/id-list behaviour; plus functional runs (`--principles tag:fp --list-principles` lists the two fp principles in order; `--principles tag:bogus` errors with exit 2). The fallback (a separate `--tags` flag) was not needed. (Since the binary now reads `tags`, the `Principle` struct's blanket `dead_code` allowance was narrowed to the still-unused `related` field.)

#### `available-filter`: Interactive Available filter

`/` enters `Mode::Filtering`; the Available pane narrows live to a case-insensitive substring over name, id, and tags; typing appends to the query, Backspace deletes, Enter applies and returns to `Editing` (filter kept), Esc clears and returns. The Available cursor and toggle map through a visible-to-underlying projection (`available_visible`/`available_display`); Included is never filtered. Refinement: an item returned to Available while a filter is active is appended (its pool position is not user-meaningful, and "before/after cursor" is ambiguous under a projection); with no filter, behaviour is byte-for-byte unchanged, so all prior toggle tests pass. Validated by proof-of-concept: unit tests on the projection narrowing and persistence, tag matching, modality (editing keys type into the query), backspace/esc editing, and toggle-under-filter moving the correct principle and preserving the partition; clippy `-D warnings` clean, 33 tests pass. The interactive feel still wants a manual TTY run (not automatable here). The fallback (jump-to-first-match) was not needed.

#### `include-all-visible`: Optional include-all-visible

Skipped by decision. A key to move every currently-visible Available match into Included at once (tag-based bulk selection on top of the filter). Skipped because the `available-filter` `/` filter already makes finding and adding matches quick (filter, then `i`/`a` on each), so bulk-add is not needed now; it stays minimal-by-default and can be revisited if adding many tagged principles at once becomes common. No `A` binding was added; the `selection-ui` key-bindings reference already includes `/` from `available-filter`.

### Bring-your-own template

Shared context for the three steps below (`pack-manifest`, `external-packs`, `pack-owned-principles`); this umbrella is not itself a Roadmap entry. Support `--template <local-path>` so a project can scaffold from a user-supplied pack, not only the built-in one (git-URL fetch is deferred to `git-url-fetch`; a Nix flake-ref stays out of scope). Sequenced ahead of optional modules (`optional-modules`) because it formalises the pack/manifest abstraction modules slot into.

Background: the original `assets()` in `src/main.rs` hardcoded the mapping from each built-in pack file to its destination path, ownership (reference vs working), and whether it is rendered (substituted) or copied verbatim; one source can fan out to several assets (`AGENTS.md` becomes both the working root file and the `.agents/` reference, both rendered). These steps turn that mapping into data.

Decisions adopted from the resolved open questions:

- **Manifest, dogfooded (OQ-E).** A pack is a directory with a `pack.toml` manifest declaring its assets and variables; the built-in pack ships the same manifest and loads through one path, replacing the hardcoded mapping in `assets()`. Encodes the pack structure as data (Principle 5), one load path with no built-in-versus-external special-casing (Principle 1).
- **Schema (OQ-F).** Each `[[asset]]` is `{ source, dest, ownership = reference|working, render }`; a source may map to several assets. `render = true` applies minimal `{{name}}` substitution with no template engine (Principle 2): `{{principles}}` is a built-in variable computed from the selection, and packs declare other variables (with defaults) that `--var key=value` overrides. A `[[module]]` section is reserved by keeping the parser tolerant of unknown keys (serde ignores them), so these steps carry no dead field and `optional-modules` adds the field when it consumes modules. Behavioural modules (isolation) stay tool flags, not manifest content.
- **Embedding (OQ-G).** The built-in `pack/` is embedded with the `include_dir` crate so built-in and external packs both present as a directory of files under one loader, removing the hand-maintained filename-to-content map (Principle 1); accepted over dependency-free `include_str!` per the user's decision.

Each sub-step is evidence-grounded: validate the approach with a proof-of-concept (build plus tests plus, where relevant, a functional run); on failure fall back to the recorded next-best; if exhausted, raise the impasse.

#### `pack-manifest`: Pack manifest and built-in refactor

Added `pack/pack.toml` declaring the seven built-in assets (`source`, `dest`, `ownership`, `render`), embedded `pack/` with `include_dir`, and added the `manifest` module: `Ownership`, `AssetSpec`, `Manifest`, a resolved `Asset { dest, contents, ownership }`, a `PackSource` (`Embedded`/`Directory`) with a read-by-relative-path interface, `render(template, vars)` (`{{key}}` substitution, unknown placeholders left as-is), and `load(source, vars)` producing the asset set in manifest order. `main::assets()` and its `include_str!` constants are gone; `scaffold` now builds a `{{principles}}` variable from the selection and calls `manifest::load(&manifest::builtin(), &vars)`. `pack::render_agents` was removed (substitution now lives in `manifest::render`); `pack::render_principles` supplies the `{{principles}}` value. Validated by proof-of-concept: scaffold output is byte-identical to the pre-refactor golden (`diff -r` clean across all seven files), all four existing drop tests pass unchanged, plus four new `manifest` tests (render substitution, the built-in asset list, render-only-the-rendered-assets, and a filesystem `Directory` fixture proving the loader is source-agnostic); clippy `-D warnings` clean, 37 tests pass. The `Directory` variant carries `#[allow(dead_code)]` (constructed only in the test build until `external-packs` wires `--template`; `expect` would be unfulfilled in the test build). The fallback (keep the hardcoded `assets()`) was not needed.

#### `external-packs`: External local-path packs and variables

Added `--template <path>` (an external pack directory loaded through the same manifest loader as the built-in) and `--var key=value` (repeatable). `main` builds the `PackSource` once (built-in or `Directory`) and threads it into `scaffold`. The manifest gained a `[[var]]` array (`VarSpec { name, default: Option<String> }`); `manifest::load` now takes the built-in variables and the `--var` overrides, resolves the substitution map (`resolve_vars`), and returns a `LoadError` (`Io`, `UndeclaredVar`, `MissingRequiredVar`, `ReservedVar`, with `Display` and exit 2). Validated by proof-of-concept: 41 tests pass (four new `manifest` variable tests: default-applies/override-wins, missing-required error, undeclared-override error, reserved-name rejection for both declaration and override); the built-in pack still scaffolds byte-identically with no `--template` (`diff -r` clean); and a functional external-pack run confirmed rendered-versus-verbatim assets, ownership, optional defaults, overrides, and that each error path (missing required, undeclared, reserved, malformed `--var`, missing template directory) exits 2 and writes nothing. clippy `-D warnings` clean. The `Directory` variant's earlier `#[allow(dead_code)]` was removed (now constructed by `main`). The fallback (split loaders) was not needed. Interim limitation per OQ-J: `{{principles}}` and selection still come from the built-in set even under `--template`; `pack-owned-principles` removes this.

Decisions adopted from the resolved open questions:

- **Variable schema (OQ-I).** Variables are declared as a `[[var]]` array of tables, each `{ name, default? }` (a `VarSpec { name: String, default: Option<String> }`); a present `default` marks the variable optional, an absent `default` marks it required. Chosen over a `[vars]` name-to-default table (cannot express a required variable) and a string-or-table `[vars]` (untagged parsing complexity): the array of tables encodes required-versus-optional as `Option` rather than a sentinel (Principle 5), matches the `[[asset]]` convention already in the manifest (Principle 1), and needs no template engine (Principle 2).
- **Variable resolution and validation.** Resolve the substitution map from three sources: the built-in `principles` variable, then each declared variable's default, then `--var` overrides. Errors (a `TemplateError` enum with `Display`, exiting 2 like `SelectionError`): a `--var` naming a variable the pack does not declare; a required variable (no default) that no `--var` supplies; and any use of the reserved name `principles` (a pack declaring a `[[var]]` named `principles`, or a `--var principles=...`), mirroring the existing reserved-token guard in `resolve_selection`. Unknown `{{...}}` placeholders in asset bodies stay left as-is, as in `pack-manifest`.
- **Principle source, interim (OQ-J).** Here, `{{principles}}` and principle selection still come from the built-in embedded set even under `--template`; this is a documented interim limitation that `pack-owned-principles` removes. To keep that change localized (no back-tracking), `main` constructs the `PackSource` (built-in or `--template` `Directory`) once and threads it into `scaffold` in this step, so the asset load already flows through the chosen source.

Validate: a fixture pack (a temp directory with a `pack.toml` declaring an optional and a required variable and a templated asset that uses them) scaffolds correctly, honouring ownership, render, and `--var`; the required-variable, undeclared-variable, and reserved-name errors each fire; and the built-in pack still scaffolds byte-identically with no `--template`. Fallback: if a filesystem source cannot share the embedded loader cleanly, split the loaders but keep one manifest schema, and raise it.

#### `pack-owned-principles`: Pack-owned principles

Principles are now a property of the active pack. `PackSource::read` was made public and `main` gained a `pack_principles(source)` helper that parses the pack's `principles.toml` (empty set when the pack ships none; a malformed file is a parse error exiting 2); `main` uses it in place of `pack::default_principles()`, so selection, `--list-principles`, the interactive selector, and the `{{principles}}` value all reflect the active pack. For the built-in pack this reads the same embedded `principles.toml` the drop uses, so the built-in path is unchanged. `pack::default_principles`/`DEFAULT_PRINCIPLES_TOML` became `#[cfg(test)]` (production reads through the source; they remain for the pack-data tests), keeping the production build free of dead code. Validated by proof-of-concept: 44 tests pass (three new: built-in principles read through the source match the helper, an external pack's `principles.toml` drives selection with no built-in leak, and a pack with no `principles.toml` yields an empty set); functional runs confirmed `--list-principles --template` and `--principles all --template` list the pack's principles, a `--template` scaffold renders the pack's principles into `AGENTS.md` (no built-in principle present) and drops the pack's `principles.toml`, and a pack without principles renders `{{principles}}` empty without error; the built-in scaffold is byte-identical to the golden; clippy `-D warnings` clean. The fallback (keep the built-in set for external packs) was not needed. Known minor limitation (out of scope): the interactive save modal's summary text still names the built-in file set even under `--template`.

### `init-vcs`: Initialise a VCS repository (git by default)

The prioritised next step to implement (design resolved, OQ-K). Add an opt-in `--init` flag that initialises an empty version-control repository in the output directory, git by default. Adopted decisions (from OQ-K):

- Opt-in only: `--init` triggers it; without the flag nothing changes, keeping the core minimal (Principle 2) and safe on existing projects (Principle 3).
- Mechanism: shell out to `git init` on the output directory (no libgit2 or Nix dependency, consistent with `git-url-fetch`). If `--init` is given but the `git` binary is not installed, the run errors (exit 2) rather than silently skipping.
- Write-safety and idempotency: initialising is a write, so it appears in the plan preview as its own line (for example `init  <git repository>`, or `skip (exists)` when the target is already a repository) and runs only when writing is confirmed (`--write` or the selector's Save). If a `.git` directory is already present the repository is skipped, never reinitialised, so existing repos and re-runs stay safe (Principles 3 and 4). The repository is left empty (no initial commit); committing the scaffolded files stays the user's choice.
- Extensibility: git is the default and only VCS for now; a future `--vcs <name>` (defaulting to git) can add other systems such as jj or hg without changing `--init`'s meaning.

Evidence-grounding: functional runs confirm that `--init --write` creates `.git` in a fresh directory (with the init line shown in the plan), that a re-run over an existing repository skips it, that a dry run or an unconfirmed run performs no init, and that `--init` with `git` unavailable errors (exit 2); add an automated test where feasible (for example asserting the plan and the skip-on-existing decision without requiring a git binary). Fallback: if driving `git init` portably proves problematic, raise it rather than adding a git-library dependency without agreement.

### `optional-modules`: Optional modules

Not started; a design-questions pass is needed before implementing (as with `git-url-fetch`).

Goal: package opt-in additions as self-contained modules, each off by default, none complicating the core (Principle 2: adding a module must not change core behaviour when unused). Two kinds are expected:

- Content modules: extra assets. These are expected to live in the pack manifest via a `[[module]]` section, which `pack.toml` parsing already tolerates (unknown keys are ignored, so current packs stay valid). A module groups a set of `[[asset]]` entries (and possibly `[[var]]` entries) under a name the user opts into. When no module is selected, none of its assets are dropped, so the core output is unchanged.
- Behavioural modules: tool features rather than dropped files. These change how the tool runs, not what it writes, so they are expected to be tool flags rather than pack content.

Candidate modules (illustrative; the design pass confirms which to build and their exact contents and ownership). Each of these was named loosely in earlier planning; the concrete intent is:

- Diagram prompt pack (content): additional reusable prompts, in the same form as the built-in `.agents/prompts/` files, that guide an agent to produce and keep project diagrams current, for example architecture, sequence, and data-flow diagrams written as Mermaid (a text-based diagram syntax) or ASCII and embedded in the docs. Dropped as extra prompt assets (conventionally under `.agents/prompts/`) when opted in.
- Justfile of recipes (content): a `justfile` dropped at the project root with ready-made task recipes (build, test, lint, format, a `verify` aggregate, and workflow helpers), matching this project's own just-plus-Nix convention. Opt-in so a project that does not use `just` is not burdened; ownership is a working file (create-if-absent), since a project may already have its own `justfile`.
- Language starters (content): per-language starter skeletons, each a self-contained set of assets keyed to a language (for example a Rust starter with a flake, a `Cargo.toml`, and a `src/` layout, and equivalents for other languages), so a greenfield project gets a working toolchain alongside the agent workflow. Each starter is its own module, opted into by language.
- Container and worktree isolation (behavioural): run an agent's implementation phase in an isolated environment (a git worktree and/or a container) so its changes are sandboxed and easy to review or discard rather than applied straight to the working tree. Because this is behaviour, it is a tool flag, not dropped content, and it would integrate the separate agent-box (`https://github.com/0xferrous/agent-box`) and agent-images (`https://github.com/nothingnesses/agent-images`) projects (separate repositories, not part of this one) rather than reimplement them.

Open sub-questions to resolve in the design pass (record them in Open Questions, with approaches, trade-offs, a recommendation, and reasoning judged against the Project Principles, before implementing):

- The `[[module]]` schema: how a module names itself and references its assets/variables (for example an inline group, or asset entries tagged with a `module = "<name>"` key), and how the loader includes only opted-in modules while the unnamed core assets always load.
- The selection mechanism: a `--module <name>` flag (repeatable), a manifest default-on/off per module, and whether the interactive TUI gains a module pane or mode (`selection-ui` deliberately scoped the TUI to principles and left room to reuse its interaction code for modules).
- Behavioural-module scope: what container/worktree isolation actually does here, whether it belongs in this tool at all or stays a separate concern the scaffolded assets point to, and how it integrates agent-box/agent-images without a heavy dependency.
- Interaction with `--template`: whether external packs may define their own modules (they should, since the manifest is the shared contract), and how `--module` validates against the active pack.

Each resulting sub-step must be evidence-grounded: validate with a proof-of-concept (build plus tests plus, where relevant, a functional run); on failure fall back to the recorded next-best; if exhausted, raise the impasse. A guiding invariant for validation: with no module selected, the scaffold output must be byte-identical to today's core output.

### `greenfield-flake`: Optional greenfield flake template

Not started; needs a design-questions pass first. Expose a greenfield template (a `templates` output in `flake.nix`, copied by `nix flake new -t`) as a convenience for the new-project case.

Core design tension to resolve before implementing: the core assets are embedded in the binary and rendered at runtime from a selected principle set, but a `nix flake new` template is copied verbatim and cannot render. So decide how the template produces a rendered `AGENTS.md`:

- (a) The template ships a thin init (a flake app, that is a runnable `nix run` output, or a note) that runs `agent-scaffold`, keeping the binary as the single rendering mechanism, with no duplicated content.
- (b) The template ships pre-rendered default assets directly. Simpler to consume, but duplicates the pack content and drifts from it over time.

The recommendation leans to (a), per Principle 1 (one rendering path) and Principle 4 (no drift), but this must be validated in the design pass. Evidence-grounding: a project created from the template initialises cleanly and its `AGENTS.md` matches an `agent-scaffold --principles default` run; fall back to (b) only if (a) cannot be made to work without Nix at scaffold time.

### `later-enhancements`: Optional later enhancements

Not started; conditional and needs a design-questions pass first. Two independent enhancements, to be taken up only if the create-if-absent plus `--force`-overwrite model (see the Ownership decision) proves too blunt in practice:

- Marked-block augmentation of an existing `AGENTS.md`: insert and update a tool-managed block between explicit markers, idempotently, so a hand-written `AGENTS.md` can carry the generated principles without being overwritten. Hard parts to design: the marker format, idempotent re-runs (replace the block in place, never duplicate it), and what belongs inside the block versus the user's own prose.
- An opt-in `update` command doing a 3-way merge: merge upstream pack changes into a user-edited working file. Hard part: where the base (the version last scaffolded) is stored, since a 3-way merge needs a base, the user's edits, and the new version.

Each is evidence-grounded (proof-of-concept plus tests, with a byte-identical-when-unchanged invariant for the augmentation case); record the design questions in Open Questions before implementing.

### `git-url-fetch`: Optional git-URL template fetch

Not started (deferred). Extend `--template` to accept a git URL, fetching the pack (shell out to the `git` binary, no Nix dependency) into a cache directory and then loading it through the same manifest path as a local pack (`byo-template`). This was deferred (it was open question OQ-H; see the note on OQ labels in the Open Questions section) as a much-later optional extra because the core bring-your-own value is already delivered by local-path packs. Sub-decisions to settle in the design pass: ref selection (which branch, tag, or commit of the repository to fetch), an optional in-repo subdirectory (allowing a pack that lives in a subdirectory of the repository rather than at its root), where and how to cache fetched packs, and the fallback behaviour when the `git` binary is not installed. When taken up, run the design-questions pass on those first.

### `tui-authoring`: Optional TUI pack authoring

Not started; optional and needs a design-questions pass first. Add a way to create and edit custom packs through the TUI, so a user can author a bring-your-own pack (`byo-template`) without hand-writing `pack.toml` and `principles.toml`.

Goal: an interactive editor for a pack's structure, reusing the TEA-lite `App`/`update`/`ui` patterns and shared widgets from the principle selector (`selection-ui` and the polish steps) without complicating the existing selection flow (Principle 2). It edits the pack manifest and principle data as structured forms and writes the pack directory back out; editing arbitrary asset source-file bodies is expected to stay out of scope (that is a text editor's job).

Scope (to confirm in the design pass): create a new pack directory with a `pack.toml`; add, edit, remove, and reorder `[[asset]]` entries (`source`, `dest`, `ownership`, `render`) and `[[var]]` entries (`name`, `default`); add, edit, and remove principles (the full `Principle` model: `id`, `name`, `summary`, `rationale`, `tags`, `default_selected`, `default_order`, `references`, `related`); and validate on save with the same rules the loader enforces (unique ids and `default_order`, no id or variable name colliding with a reserved token, referenced source files present).

Open sub-questions to resolve in the design pass (record in Open Questions with approaches, trade-offs, a recommendation, and reasoning judged against the Project Principles):

- CLI surface: a new flag (for example `--edit-pack <dir>` and `--new-pack <dir>`) versus introducing subcommands (`agent-scaffold pack new|edit <dir>`). The CLI is flag-only today, so subcommands are a structural change (clap supports both).
- Serialization: writing `pack.toml` with `toml` (lossy: drops comments and reflows the file) versus `toml_edit` (format- and comment-preserving). Editing an existing hand-written pack must not destroy its comments or unrelated content (Principle 3, safe on existing), which points toward `toml_edit`, but that is a new dependency to weigh (Principle 2). The `manifest` types are `Deserialize`-only today and would need `Serialize`, or an equivalent `toml_edit` mapping.
- TUI composition: whether pack authoring is a separate top-level TUI flow that reuses the shared reducer and widget patterns, or new modes on the existing `App`. Keep the principle selector uncomplicated; a module or pack pane was already anticipated as a reuse point (`selection-ui` scoped the selector to principles deliberately).
- Round-trip and safety: creating then loading a pack via `--template` must scaffold correctly, and re-editing a pack must preserve everything the editor does not touch.

Evidence-grounding: author a pack in the TUI, then load it with `--template` and scaffold, asserting the assets, variables, and principles round-trip; editing an existing pack leaves its untouched content (including manifest comments, if `toml_edit` is chosen) intact. Fall back to the recorded next-best on failure; if exhausted, raise the impasse.

## Success Criteria

- One command, run from any working directory and targeting any `--output-dir`, drops the minimal core (a usable `AGENTS.md`, a planning-document template, and the core prompts) into both an empty directory and an existing repository. Writes are off unless confirmed: a run previews the plan and only writes with `--write` or the selector's Save confirmation (`--dry-run` previews explicitly), so nothing is written by accident.
- By default it creates only absent files and never modifies existing ones; `--force` is required to overwrite; a default re-run therefore reverts nothing.
- The user can choose which principles and modules to include, interactively with sane defaults or non-interactively via flags or config.
- The dropped assets are immediately usable to run one pass of the workflow (plan -> review -> implement -> review) without further setup.
- Optional modules can be added without touching or complicating the core.
- The tool can scaffold from a user-supplied template pack, not only the built-in one.
