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
