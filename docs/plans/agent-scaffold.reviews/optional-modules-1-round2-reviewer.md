# Round-2 confirming review: optional-modules increment 1 (`--module` machinery)

Range verified: `139a750..a7ae6ed` (the implementer fix commit). Artifact class: LOW-risk (one clean round to converge). This is the round-2 confirming pass over the round-1 fixes (V-1, V-2, V-4 to the implementer; V-3 to the orchestrator). I verified each fix by reading the diff and `src/manifest.rs`, ran the sanity checks, and exercised the machinery adversarially at the CLI (real binary), not only through the unit tests.

## Verdict

The round-1 fixes all landed and are correct. One new finding: a low-severity plan-doc drift introduced BY the round-1 rename (a stale `LoadError` variant name in the plan's Repository-Layout description). No medium/high/critical findings.

## Sanity checks (all pass)

- `just test`: 88 tests pass (matches the expected ~88).
- `cargo clippy --all-targets -- -D warnings`: clean.
- `just scaffold-self`: byte-identical. `git diff --stat -- AGENTS.md .agents/ docs/plans/TEMPLATE.md` is empty and the tree stays clean; the built-in pack remains module-free.

## Fix verification

### V-1 (var module tagging) - CONFIRMED

`VarSpec` now carries `module: Option<String>` (`manifest.rs:86-92`). `load` pre-filters vars to core-plus-selected before `resolve_vars` (`manifest.rs:358-367`), so a var tagged with an unselected module is skipped entirely: not required, not defaulted, absent from the substitution map. The KEY behaviour holds: a module's REQUIRED var does not force itself on a user who did not select the module.

Verified at the CLI with a fixture pack (`[[module]] diagrams`, a module-tagged required var `diagram_title`, a module-tagged asset `diagram.md`):

- No `--module`: exit 0, only the core asset drops (`core.md` = `hi world`); the required tagged var is NOT demanded. (KEY behaviour.)
- `--module diagrams`, var absent: `error: required variable 'diagram_title' was not supplied` (`MissingRequiredVar`), exit 2, nothing written.
- `--module diagrams --var diagram_title=Flow`: exit 0, both `core.md` and `diagram.md` drop, tagged var renders (`diagram: Flow`).
- Two-module pack, select only `a`: module `b`'s required var does not fire; only `core.md` + `amod.md` drop; no panic.

The chosen behaviour for `--var` on a skipped var (UndeclaredVar) is sensible and consistent: with no `--module`, `--var diagram_title=Flow` gives `error: no variable named 'diagram_title' is declared by the pack`, exit 2, nothing written. This is exactly the treatment of a variable the pack never declared, matching the doc comment and the fail-loud principle (a stray `--var` is never silently ignored). Consistent.

### Unified `UndeclaredModuleTag { kind, entry, module }` - CONFIRMED

The variant covers both entry kinds (`manifest.rs:133-141`), populated with `kind: "asset"` and `kind: "var"` at the two validation sites (`manifest.rs:332-353`), checked for every entry regardless of selection. The `Display` message is accurate for both: `var 'ghost' is tagged with module 'ghost', which no [[module]] declares` (CLI-confirmed) and, symmetrically, `asset ...` for the asset case (unit test `a_var_tagged_with_an_undeclared_module_errors` and the renamed asset test cover both). Errors exit 2, nothing written.

### V-4 (duplicate module names) - CONFIRMED

`declared` is now built with an insert-and-check loop that returns `LoadError::DuplicateModule` on a repeated name (`manifest.rs:318-323`), before the `--module` and tag checks. CLI-confirmed: a pack with two `[[module]] name = "extras"` sections yields `error: module 'extras' is declared by more than one [[module]] section`, exit 2, nothing written. Message accurate.

### V-2 (README) - CONFIRMED

The README's new "Optional modules" section (README.md:241-269) documents the `[[module]]` section (`name`/`description`), the `module = "<name>"` tag on BOTH `[[asset]]` and `[[var]]` entries, and the repeatable `--module` flag. It states the load semantics accurately and consistently with the code: an untagged entry is core; a tagged entry applies only when its module is selected; with no module selected a tagged var's default does not apply, it is not required, and a `--var` naming it is rejected as undeclared; a selected module's vars behave like core ones; core output stays byte-identical to a module-free pack. It also documents that an unknown `--module`, a dangling tag, or a duplicate `[[module]]` name is an error and nothing is written. All consistent with the observed behaviour.

## New finding

### F-1 (low): plan Repository-Layout description names the OLD `LoadError` variant `UndeclaredAssetModule`

Evidence: `docs/plans/agent-scaffold.md:58` lists the loader's errors as `LoadError { Io, UndeclaredVar, MissingRequiredVar, ReservedVar, UnknownModule, UndeclaredAssetModule, DuplicateModule }`. The code renamed that variant to `UndeclaredModuleTag { kind, entry, module }` in the very fix under review (`manifest.rs:133`; the rename is in the `139a750..a7ae6ed` diff). So the plan now names a variant that no longer exists in the code.

This is genuinely NEW, not a re-raise of the settled V-3: V-3 (the plan-staleness finding) was applied by the orchestrator at/around `139a750`, which correctly reflected the round-1 state where the variant was still `UndeclaredAssetModule`. The implementer's round-1 fix then renamed it as part of unifying asset and var tags, which the orchestrator's V-3 edit predated. The result is a fresh drift the V-3 pass could not have caught.

Severity low: it misleads only resume context (a fresh implementer reading line 58 to understand `load()` would look for a variant that is not there), with no runtime effect. Same class and severity as V-3. Owner: orchestrator (the plan is orchestrator-owned; the implementer must not edit `docs/plans/`). Fix: change `UndeclaredAssetModule` to `UndeclaredModuleTag` at line 58. The Pack-format section (lines 61-67) is accurate and needs no change; the drift is only the one variant name.

## Consistency (plan docs vs code)

Otherwise the plan matches the code. Line 58's type signatures are correct (`module` fields on `AssetSpec`/`VarSpec`, `ModuleSpec { name, description }`, `load(source, builtin_vars, overrides, selected_modules)`, the "filters out unselected modules' assets and vars" description, `DuplicateModule` present, all-exit-2). The Pack-format block (lines 61-67) accurately describes the `module?` tag on assets and vars, the `[[module]] { name, description }` section, the repeatable `--module`, the uniqueness requirement, and the three pack-authoring/usage errors. The `Q-25` HYBRID decision (line 491) is faithfully implemented. Only F-1 is inconsistent.

## Severity counts

- Critical: none.
- High: none.
- Medium: none.
- Low: 1 (F-1, plan-doc drift; orchestrator-owned).

The three implementer-owned round-1 fixes (V-1, V-2, V-4) and the shared-variant rename are all correct and verified at both the unit-test and CLI levels; scaffold-self stays byte-identical and the built-in pack stays module-free. The one new finding is a low-severity orchestrator-owned plan-doc drift caused by the round-1 rename.
