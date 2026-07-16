# Review: optional-modules increment 1 (`--module` machinery)

Reviewer: sonnet (design consistency and documentation lens)
Diff range: `74083fa..f34ed1c`

---

## F-1 (medium): README pack-format section does not document `[[module]]`, `module = "<name>"`, or `--module`

The README's "Bring your own pack" section (lines 213-241) documents `[[asset]]`, `[[var]]`, `{{principles}}`, and `{{instrument}}` in full, but says nothing about:
- The `[[module]]` section and its `name` / `description` fields.
- The `module = "<name>"` field that `[[asset]]` entries now accept.
- The `--module <name>` flag and its interaction with the pack.

Evidence: `grep -n "module" README.md` returns no hits. The section ends at line 241 and the "Development" section follows immediately.

Precedent: the `instrument-flag` step produced the same finding class ("the Pack format docs did not mention `{{instrument}}` or `instrument.md`"), which the triager upheld as a valid medium finding (per the `instrument-flag` outcome in `docs/plans/agent-scaffold.md`). Both locations were then updated. This increment did not update the README.

A pack author reading the README cannot learn how to declare a module, tag an asset for it, or select it. Without documentation they cannot use the feature even when the machinery is present. This is the highest-priority documentation gap in this increment.

Files: `README.md` lines 213-241 (no module content).

---

## F-2 (medium): Decided hybrid schema omits `[[var]]` module tagging; `VarSpec` has no `module` field

The Q-25 design-pass decision (recorded in `docs/plans/agent-scaffold.md`, `optional-modules` step, "Module schema (HYBRID)") states:

> "assets self-declare membership with an optional `module = "<name>"` field on `[[asset]]` (and `[[var]]`) entries"

The implementation adds `module: Option<String>` to `AssetSpec` and filters assets by module in `load()`, but does not add a `module` field to `VarSpec` (`src/manifest.rs` lines 79-86) and does not apply module filtering to the variable resolution path (`resolve_vars`, called at line 291 in `load()`).

Consequence: a future module (for example the checks module in Increment 2) that needs to declare a required variable cannot scope that variable to its module. A user who does not opt in with `--module checks` would still get a "missing required variable" error for any required variable the module declares, because `resolve_vars` sees every declared `[[var]]` regardless of module selection. This is a latent correctness bug that will become visible the moment any module adds a required `[[var]]` entry.

The departure from the decided schema is in-scope for Increment 1, because Increment 1 is described as building "the `--module` machinery" and the schema decision explicitly includes var entries.

Files: `src/manifest.rs` lines 79-86 (`VarSpec` struct); `src/manifest.rs` `load()` (the `resolve_vars` call is not filtered); `docs/plans/agent-scaffold.md` line 490 (the Q-25 decision text).

---

## F-3 (low): Plan's "Pack format" section in Repository Layout is stale

The "Pack format (the contract shared by the built-in pack and `--template` packs)" block (`docs/plans/agent-scaffold.md` lines 61-67) was updated when `--instrument` was added (the `instrument-flag` outcome notes "the Pack format docs (README and this plan) now document both reserved variables and the fragment pattern"). This increment did not update that section. It still describes `[[asset]]` as `{ source, dest, ownership, render }` and says nothing about the `module` optional field, the `[[module]]` section, or `--module`.

This section is orchestrator-owned (it is part of the plan, not the README), so its staleness is lower impact than F-1. Still, it is one of the two canonical documentation locations for pack authors and future agents picking up the work, and the `instrument-flag` step set the precedent of updating both.

Files: `docs/plans/agent-scaffold.md` lines 61-67.

---

## F-4 (low): Plan's "Modules and key types" description for `manifest.rs` and `main.rs` is stale

The Repository Layout descriptions of `src/manifest.rs` (line 58) and `src/main.rs` (line 56) in `docs/plans/agent-scaffold.md` are not updated to reflect the changes in this increment.

Specific stale claims:
- `Manifest { asset: Vec<AssetSpec>, var: Vec<VarSpec> }` - missing `module: Vec<ModuleSpec>`.
- `AssetSpec { source, dest, ownership, render }` - missing `module: Option<String>`.
- `load(source, builtin_vars, overrides)` - missing the new `selected_modules: &[String]` parameter.
- `LoadError { Io, UndeclaredVar, MissingRequiredVar, ReservedVar }` - missing `UnknownModule` and `UndeclaredAssetModule`.
- The `main.rs` flag list ends at `--var key=value (repeatable)` with no mention of `--module`.

The plan's Repository Layout section is the resume context that lets a future implementer understand the codebase without prior conversation context. Stale type signatures and a missing flag are low severity but will mislead a fresh agent that reads this to understand what `load()` takes or what error variants exist.

Files: `docs/plans/agent-scaffold.md` lines 56-58.

---

## F-5 (low): Duplicate `[[module]]` names in a pack are silently accepted

In `load()`, the declared-module set is built as:

```rust
let declared: HashSet<&str> = manifest.module.iter().map(|m| m.name.as_str()).collect();
```

If a pack declares two `[[module]]` entries with the same `name`, the `HashSet` silently de-duplicates them. No error is returned. This violates Principle 5 (make illegal states unrepresentable) and Principle 12 (fail fast and loudly): a pack-authoring error should be caught and reported rather than silently accepted.

The impact today is benign (the de-duplicated set still validates `--module` and filters assets correctly), but failing loudly on a structural pack error is the stated design intent for the rest of the validation logic (dangling asset tags and unknown `--module` selections both return hard errors rather than silent fallbacks).

A check analogous to the undeclared-var check (see `UndeclaredVar` / `ReservedVar`) would address this: after building `declared`, verify that `manifest.module.len() == declared.len()`, returning a new `DuplicateModule(String)` error variant if not.

Files: `src/manifest.rs` `load()`, line building `declared` (the `HashSet` construction).

---

## Non-findings

These items were checked and found acceptable:

- `#[expect(dead_code)]` on `ModuleSpec.description`: consistent with how `Principle.related` was carried before use; uses the correct `#[expect]` form (not `#[allow]`); the reason string records the intent ("declared for the schema and TUI; not yet read by the loader"). Acceptable.
- `--template` interaction: the module validation runs through `load()` regardless of the pack source, so external packs defining modules is coherent with the decided design. Acceptable.
- Module-free built-in pack: intentional by design decision. Not a finding.
- Absence of checks/isolation modules: those are Increments 2-3. Not a finding.
- Error messages (`UnknownModule`, `UndeclaredAssetModule`): readable and give the pack name in backticks for easy identification.
- `--module` help text: clear and consistent with the `--var` style; the "Unknown modules are an error" trailing sentence matches the behavior.
- Error on unknown `--module` is verified: `cargo run -- scaffold --module bogus --dry-run` produces `error: no module named 'bogus' is declared by the pack` and exits 2.
- `--module` appears in `scaffold --help` output.
- Test coverage: four tests cover the core scenarios (core-only, selected-module, unknown-module, dangling-asset-tag, empty-module). The selection-with-no-matching-assets case is also covered. Adequate for Increment 1.
