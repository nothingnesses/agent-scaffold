# Review: state-schema increment 1 (subcommand restructure)

Reviewer: sonnet (reviewer-sonnet)
Diff range: `78571a0..e32dc5a`
Artifact: the subcommand-only CLI restructure - `Cli` split into `Cli`/`Command`/`ScaffoldArgs`, `scaffold-self` justfile recipe updated, README usage examples updated.
Reviewed against: `pack/prompts/reviewer.md`, `AGENTS.md` Principles 1-7, and the `state-schema` "Design-pass decisions" block in `docs/plans/agent-scaffold.md` (lines 386-393).

---

## Caller coverage (stale invocations)

Searched all committed files outside `docs/plans/` for `agent-scaffold --` and `cargo run -- --` patterns.

- `justfile` `scaffold-self` recipe: updated to `cargo run -- scaffold --output-dir . --write --force --principles default`. Correct.
- `README.md`: every `agent-scaffold ...` invocation in the Usage section is now `agent-scaffold scaffold ...`. Confirmed for lines 131, 137, 158, 161, 164, 171, 186.
- `pack/AGENTS.md` and all `pack/prompts/` files: no CLI invocations found. No action needed.
- `.agents/` and other committed assets: no CLI invocations found.
- `target/` (build artifacts): stale references in `target/package/agent-scaffold-0.0.1/README.md` and `spec.md` - these are unpublished build artifacts, not committed source. Not a finding.

Result: no stale callers in committed, non-plan source files.

---

## README prose accuracy

The new opening paragraph (line 118) correctly describes the new surface:

> "Every action is a subcommand. Bare `agent-scaffold` (with no subcommand) prints the list of subcommands and exits; scaffolding runs under the `scaffold` verb."

Verified behavior: running bare `agent-scaffold` exits 2 and prints the help/subcommand list (`arg_required_else_help = true`). This matches the README claim.

No sentence remains implying bare invocation scaffolds, and no sentence implies flags are top-level.

---

## `--help` output accuracy

`agent-scaffold --help` output (confirmed by running the binary):

```
Scaffold the agent workflow into a project, and validate or project its state.

Usage: agent-scaffold <COMMAND>

Commands:
  scaffold  Scaffold the agent workflow into a project. On a terminal the principle selector opens unless --write or --dry-run is given
  help      Print this message or the help of the given subcommand(s)
```

`agent-scaffold scaffold --help` lists all scaffold-specific flags accurately. The selector, `--write`/`--dry-run`, and `--principles` flags all appear and are described accurately.

---

## Findings

### F-1: Top-level `about` string describes features that do not exist in this increment

**Severity: medium**

**Location:** `src/main.rs:256`

```rust
about = "Scaffold the agent workflow into a project, and validate or project its state.",
```

The phrase "validate or project its state" describes the `validate` and `status --json` subcommands decided for later increments of `state-schema`. Neither subcommand exists in this increment. A user who installs this version and runs `agent-scaffold --help` or bare `agent-scaffold` sees the about string and learns the tool validates and projects state, but there is no such command. Running `agent-scaffold validate` produces clap's "unrecognized subcommand" error.

The plan explicitly defers these to later increments ("the metrics schema + validate; then the structured-region parse + validate + status --json"), so shipping this about string now promises a capability that is not available.

**Impact:** Misleading to any user of the currently-published or locally-built binary. A user following the --help output to find validate/status will not find them.

**Fix:** Scope the about to what this increment provides, for example: "Scaffold the agent workflow into a project." Update the about when validate/status land.

---

## Clean areas (no findings)

- All callers updated correctly. No stale invocations in committed source.
- README Usage section accurately describes the new surface at every point.
- `scaffold --help` is accurate and self-consistent.
- `arg_required_else_help = true` correctly makes bare invocation print help.
- `Command::Scaffold` doc comment is accurate and not duplicative of the top-level about in a harmful way.
- Subcommand-only design, no bare default, selector under `scaffold`: all consistent with the decided design.
- No pack asset or `pack/AGENTS.md` file references the CLI.
- Tests pass (50/50) and the scaffold run is functional.
- `scaffold-self` justfile recipe is updated correctly.
