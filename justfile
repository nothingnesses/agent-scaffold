set shell := ["bash", "-c"]
set tempdir := "/tmp"

# Load the Nix development environment via direnv for local recipes.
skip_direnv := env_var_or_default("SKIP_DIRENV", "")
direnv_prefix := if skip_direnv != "" { "" } else { "direnv exec . " }

# Display list of commands.
default:
	@just --list

# Approve the direnv environment after reviewing `.envrc` and Nix flake changes.
allow-env:
	direnv allow

# Build the tool.
build:
	{{ direnv_prefix }} cargo build

# Run the tool; pass arguments after `--`, e.g. `just run -- --help`.
run *args:
	{{ direnv_prefix }} cargo run -- {{ args }}

# Run the tests.
test:
	{{ direnv_prefix }} cargo test

# Lint with Clippy.
clippy:
	{{ direnv_prefix }} cargo clippy --all-targets

# Format all files through the Nix formatter.
fmt:
	{{ direnv_prefix }} nix fmt

# Regenerate the project's own scaffolded assets from the pack, so the committed
# `AGENTS.md`, `.agents/`, and plan template stay in sync with the pack (dogfooding).
# The raw render is not guaranteed formatter-clean (prettier owns Markdown wrapping,
# proseWrap=never), so format the output afterwards to reach a stable committed state.
scaffold-self:
	{{ direnv_prefix }} cargo run -- --output-dir . --write --force --principles default
	{{ direnv_prefix }} nix fmt
