# Reviewer findings: state-schema increment 1 (subcommand restructure)

Reviewer lens: correctness and behaviour-preservation. Range `78571a0..e32dc5a`.
Model: opus.

## Verdict

CLEAN. No findings at any severity (critical / high / medium / low). The refactor converts the flags-only CLI into a subcommand-only CLI introducing only the `scaffold` verb, and preserves behaviour exactly. Absence of `validate` / `status` is expected for this increment and is not treated as a finding.

## What was checked and the evidence

### Flag preservation (all 11 flags)

Compared the old `Cli` struct (`git show 78571a0:src/main.rs`) field-by-field against the new `ScaffoldArgs` (`src/main.rs:271-309`). Every flag moved verbatim: `output_dir`, `force`, `vcs`, `write`, `dry_run`, `principles`, `principle_detail`, `list_principles`, `template`, `var`, `instrument`. For each, the `#[arg(...)]` attribute set, defaults (`default_value = "."`, `default_value_t = Vcs::Git`, `default_value = "default"`, `default_value_t = Detail::Summary`), `value_name = "KEY=VALUE"`, `long = "var"`, `conflicts_with = "write"` on `dry_run`, `value_enum` markers, field order, and doc comments are all byte-identical. No dropped flag, no renamed long option, no changed default, no lost attribute.

Empirically confirmed against `scaffold --help`: all 11 options print with their original help text, defaults, and possible-value lists (git/none, name/summary/full).

### `run_scaffold` body vs old `main`

The diff is a mechanical `cli.<field>` -> `args.<field>` rename plus one `rustfmt` reflow of the `build_assets(...)` call (multi-arg wrap; no semantic change). Control flow is identical: the `--var` malformed-entry rejection, `resolve_selection`, the `list_principles` early return, the selector branch, the non-interactive `(selected, args.write)` branch, `init_plan` / `run_git_init`, `apply_asset`, and all `eprintln!("error: ...")` + `std::process::exit(2)` paths are unchanged. Tail (`if write { ... } else { ... } Ok(())`, `src/main.rs:430-448`) matches the old tail exactly. `main` now only dispatches `Command::Scaffold(args) => run_scaffold(args)` (`src/main.rs:311-315`).

### `use_selector` signature and open condition

`use_selector` now takes `&ScaffoldArgs` (`src/main.rs:216`); the open condition (`!write && !dry_run && !principles.is_empty() && stdin/stdout is_terminal`) is unchanged, only `cli.` -> `args.` (`src/main.rs:219-225`).

### `arg_required_else_help`

Set on the top-level `#[command(...)]` (`src/main.rs:257`). Verified empirically: bare `agent-scaffold` prints the subcommand list and exits 2.

### Empirical verification

- Byte-identical scaffold: `just scaffold-self` then `git diff --stat -- AGENTS.md .agents/ docs/plans/TEMPLATE.md` is EMPTY. Pass.
- `just test`: 50 passed, 0 failed. Pass (matches expected 50).
- `cargo clippy --all-targets -- -D warnings`: clean, no warnings. Pass.
- Functional (scratch temp dir): `scaffold --output-dir <tmp> --write --force --principles default` -> "Wrote to ... (15 changed, 0 left untouched)", exit 0, AGENTS.md + docs written. Pass.
- `scaffold --dry-run --write` -> clap conflict error, exit 2. Pass.
- Bare `agent-scaffold` -> subcommand list, exit 2. Pass.
- `scaffold --help` -> all flags with original help text. Pass.
- Error-exit paths under the verb: malformed `--var badentry` -> `error: --var expects key=value, got \`badentry\``, exit 2; missing `--template /nonexistent/pack/dir` -> `error: No such file or directory (os error 2)`, exit 2. Both preserved. Pass.

### Docs / justfile consistency

`README.md` usage examples updated to the `agent-scaffold scaffold ...` form; `justfile` `scaffold-self` recipe updated to `cargo run -- scaffold ...`. Both consistent with the new surface. The one deliberate behaviour change (bare invocation no longer scaffolds; the selector opens on `agent-scaffold scaffold` on a terminal) matches the design-pass decision recorded in `docs/plans/agent-scaffold.md` and is documented in README.

## Severity summary

- critical: none.
- high: none.
- medium: none.
- low: none.
