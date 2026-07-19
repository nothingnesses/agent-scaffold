### `greenfield-flake`: Optional greenfield flake template

Not started; needs a design-questions pass first. Expose a greenfield template (a `templates` output in `flake.nix`, copied by `nix flake new -t`) as a convenience for the new-project case.

Core design tension to resolve before implementing: the core assets are embedded in the binary and rendered at runtime from a selected principle set, but a `nix flake new` template is copied verbatim and cannot render. So decide how the template produces a rendered `AGENTS.md`:

- (a) The template ships a thin init (a flake app, that is a runnable `nix run` output, or a note) that runs `agent-scaffold`, keeping the binary as the single rendering mechanism, with no duplicated content.
- (b) The template ships pre-rendered default assets directly. Simpler to consume, but duplicates the pack content and drifts from it over time.

The recommendation leans to (a), per Principle 1 (one rendering path) and Principle 4 (no drift), but this must be validated in the design pass. Evidence-grounding: a project created from the template initialises cleanly and its `AGENTS.md` matches an `agent-scaffold --principles default` run; fall back to (b) only if (a) cannot be made to work without Nix at scaffold time.
