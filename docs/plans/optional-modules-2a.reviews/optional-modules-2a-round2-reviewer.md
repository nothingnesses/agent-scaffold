# optional-modules 2a, round 2 (confirming) reviewer

Range reviewed: `b565103..25731c5` (commits `de70ad0`, `25731c5`). Files changed: `README.md`, `pack/AGENTS.md`, `src/main.rs`, `src/manifest.rs`. No other files touched, no generated-file drift.

Verdict: CLEAN round. No new findings at any severity. All round-1 fixes (A-H) landed correctly, the core 2a machinery is unchanged in behavior and still correct, and the round-1 fixes introduced nothing new.

## Round-1 fixes verified

- A (present, correct): `AssetSpec.module` doc (`src/manifest.rs:52-57`) and `VarSpec.module` doc (`src/manifest.rs:107-113`) now state the entry participates when the module "is enabled (selected directly with `--module <name>`, or pulled in transitively by another module's `requires`)". Matches the actual `enabled` filter in `load` (`src/manifest.rs:487-503`).
- B (present, correct): `ModuleSpec` struct comment (`src/manifest.rs:62-68`) now names both `guidance` and `requires`.
- C (present, correct): `guidance` field doc (`src/manifest.rs:77-86`) states a declared guidance file is REQUIRED and explicitly contrasts with the silently-optional `instrument.md`. The old misleading "like instrument.md" optionality is gone. This matches the hard-fail behavior in `module_guidance`.
- D (present, correct): new test `a_missing_guidance_file_errors_when_its_module_is_enabled` (`src/manifest.rs:1159-1193`).
- E (present, correct): the missing-guidance error is wrapped at the `module_guidance` call site (`src/manifest.rs:299-307`) via `io::Error::new(error.kind(), format!(...))`. It preserves `error.kind()` (still `NotFound`) and names both the module and the filename. `PackSource::read` and all other read sites are untouched; the variant stays `LoadError::Io`.
- F/G/H (present, correct): `module_guidance` doc says each partial "has its trailing whitespace trimmed" (`src/manifest.rs:278`, matches `partial.trim_end()` at `:308`); `expand_modules` doc reworded to "single source of the algorithm, not of a shared result" (`src/manifest.rs:216-221`); `requires` field doc says "validated in `expand_modules`" (`src/manifest.rs:145-147`, matches the dangling-requires check at `:242-251`).

## New test genuinely asserts the error path (Principle 11)

`a_missing_guidance_file_errors_when_its_module_is_enabled` (`src/manifest.rs:1159-1193`) writes a pack whose `base` module declares `guidance = "absent-guide.md"` but deliberately does not ship that file, enables `base`, and requires `module_guidance` to return `Err(LoadError::Io(_))` with `error.kind() == NotFound` and a message containing both `base` and `absent-guide.md`. The non-error branch is a hard `panic!`. This is a real negative assertion, not a smoke test, and it pins the wrapped-message contract from fix E.

## Core 2a machinery re-checked (unchanged, correct)

- `{{modules}}` reserved: `RESERVED_VARS` includes `"modules"` (`src/manifest.rs:135`); `reserved_modules_variable_is_rejected` covers both declare and `--var` paths.
- Byte-identical scaffold: `module_guidance` returns `""` for the module-free built-in; `pack/AGENTS.md` appends `\n\n{{modules}}`, and `render` does `format!("{}\n", out.trim_end())` (`src/manifest.rs:288`), so an empty slot normalises the tail to a single trailing newline. `modules_slot_renders_empty_for_the_module_free_builtin` (`src/main.rs`) asserts no leftover placeholder, ends with one `\n`, and no `\n\n` tail. The mid-body blank-line-run cosmetic is 2b (out of scope, not re-raised).
- `requires` transitive expansion terminates: `expand_modules` (`src/manifest.rs:222-270`) uses `enabled.insert(...)` as a visited guard and a `pending` stack, pushing a requirement only when not already enabled. Manually traced cycle (a<->b), self-require, and diamond: all terminate, each module enabled once. `a_requires_cycle_terminates` confirms.
- Guidance concatenation in declaration order: `module_guidance` iterates `manifest.module` in declaration order and emits only enabled modules (`src/manifest.rs:292-312`), independent of selection order. `requires_auto_enables_a_dependencys_guidance_and_assets` confirms `base` before `extra`.
- Single-sourced `expand_modules`: both `load` (`src/manifest.rs:467`) and `module_guidance` (`src/manifest.rs:290`) call it; the duplicate/unknown/dangling-requires validation lives in one place. `load`'s separate `declared` set (`:471`) is only for tag checks and is safe because `expand_modules` already rejected duplicate declarations.

## Gate results

- `just test`: 124 passed, 0 failed.
- `just clippy` and `cargo clippy --all-targets -- -D warnings`: no warnings, no errors.
- ASCII-clean: no non-ASCII bytes in the added lines.

## Settled findings (not re-raised)

CHANGELOG (orchestrator-owned), the extra nice-to-have tests (triager-ruled not required), and the internal blank-line-run cosmetic (2b). No new evidence found that would overturn any of these verdicts.
