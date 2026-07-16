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
# `--instrument` is on because we dogfood the workflow instrumentation: the rendered
# `AGENTS.md` carries the metrics-logging block, and this project logs its own review
# rounds to `docs/metrics/workflow.jsonl` (validated with `agent-scaffold validate`).
# The raw render is not formatter-clean (prettier owns Markdown wrapping via
# proseWrap=never), so run the repo-wide formatter afterwards to normalise the
# generated files. `nix fmt` formats the whole tree, not just the generated files;
# that is intentional and harmless because treefmt is idempotent (a no-op on files
# already clean), and it leaves the repo at a stable committed fixed point.
scaffold-self:
	{{ direnv_prefix }} cargo run -- scaffold --output-dir . --write --force --principles default --instrument
	{{ direnv_prefix }} nix fmt
