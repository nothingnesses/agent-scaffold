# Review: optional-modules-2a (sub-increment 2a: `{{modules}}` slot, `guidance`, `requires`)

Reviewer: sonnet (design, SoT, doc consistency) Diff range: b565103..de70ad0 Branch: impl/inc2a-modules-slot

## Verdict on the implementer's flagged design question

Dedicated `UndeclaredModuleRequire { module, requires }` is the right call. Reusing `UndeclaredModuleTag` would require shoe-horning a module-to-module relationship into fields designed for asset/var entries (`kind: &'static str`, `entry: String`). The values would be awkward (`kind = "module"`, `entry = ???`) and the message template ("{kind} `{entry}` is tagged with module `{module}`") would be wrong for this context. The dedicated variant produces a precise, unambiguous message ("module `X` requires `Y`, which no [[module]] declares"), adds no risk of future confusion, and keeps the enum semantically clean. The cost (one extra variant) is negligible.

---

## High findings

### H1 - `AssetSpec.module` and `VarSpec.module` field docs still say "selected", not "enabled"

`src/manifest.rs:54-56` and `src/manifest.rs:101-104`

Both field-level doc comments were not updated to reflect `requires`. They say a tagged asset/var is dropped only "when that module is selected with `--module <name>`". After this change a module can be enabled by a `requires` chain without ever being explicitly selected. A reader looking at these comments to understand when a tagged entry is active will get a false picture: they will not know that auto-enablement from `requires` counts.

The load function itself correctly uses `enabled` throughout (`enabled.contains(...)`), so the behaviour is right. But the field docs contradict it.

`AssetSpec.module` (lines 52-58):

```
/// `Some(name)` is dropped only when that module
/// is selected with `--module <name>`.
```

`VarSpec.module` (lines 101-107):

```
/// `Some(name)` participates only when that
/// module is selected with `--module <name>`, and is skipped entirely otherwise
```

Both should say "enabled" (whether by direct `--module` or by `requires`).

### H2 - `ModuleSpec` struct-level comment is stale after the change

`src/manifest.rs:61-65`

```
/// Membership itself is single-sourced on the assets' `module` tag;
/// this section only names each module and describes it.
```

After this change the `[[module]]` section carries `guidance` (a partial filename) and `requires` (a dependency list). The claim "only names each module and describes it" is no longer accurate; the section now governs rendering behaviour (`guidance`) and the auto-enable graph (`requires`). A reader consulting this struct comment to understand what a `[[module]]` entry does will be misled.

---

## Medium findings

### M1 - `module_guidance` doc says "trimmed" but code calls `trim_end()`, not `trim()`

`src/manifest.rs:387`

The doc comment says:

```
/// Each partial is trimmed and separated by a blank line
```

The implementation at line 404 is:

```rust
block.push_str(partial.trim_end());
```

`trim_end()` removes trailing whitespace only. Leading whitespace and leading blank lines in a guidance partial are preserved. The word "trimmed" implies both ends. If a guidance file has leading blank lines, those blank lines will appear in the `{{modules}}` block, potentially before the first partial when ordering matters. This is a doc inaccuracy that could also be read as a bug depending on intent. The fix is either to say "trailing whitespace trimmed" in the doc, or to apply `trim()` in the code if leading whitespace is also unwanted.

### M2 - `guidance` doc misleads about missing-file behaviour with "like `instrument.md`"

`src/manifest.rs:385`

```
/// (read from the pack source, like `instrument.md`);
```

The analogy to `instrument.md` suggests similar handling. In `build_assets` (`src/main.rs:222`), `instrument.md` is read with `.unwrap_or_default()`, so a missing `instrument.md` silently produces an empty string. A guidance file named in the `guidance` field is read with `source.read(guidance)?` (line 403), so a missing file propagates `LoadError::Io`. The two behave differently on absence. The analogy "like `instrument.md`" implies silent-optional, but a declared guidance file that is absent is a hard error.

This is a real distinction a pack author needs to know: `instrument.md` is optional by convention (absent = no instrumentation); a `guidance` value, once declared, requires the file to exist.

### M3 - "one source of truth for the enabled set" claim in `expand_modules` doc overclaims

`src/manifest.rs:328-330`

```
/// The returned set drives both asset/variable filtering in `load` and the guidance
/// concatenation in `module_guidance`, so those two agree on which modules are on (one
/// source of truth for the enabled set).
```

The enabled set is not single-sourced in the data sense: `expand_modules` is called once inside `module_guidance` and again inside `load`, which are both called from `build_assets` (`src/main.rs:224-226`). The manifest is also parsed twice (`source.manifest()` in `module_guidance`, then again in `load`). The function is the single source of truth for the _algorithm_, but the result is computed and discarded twice rather than computed once and shared. The comment claims an SoT guarantee it does not deliver. A reader may incorrectly assume the two call sites share a result.

The practical risk is low (the algorithm is idempotent and the pack is immutable within a build), but the comment is inaccurate and could mask a future drift if one call site is changed without updating the other.

---

## Low findings

### L1 - `ModuleSpec.requires` doc says "validated in `load`" but validation is in `expand_modules`

`src/manifest.rs:82-84`

```
/// A name here must be declared in a `[[module]]` section (validated in `load`);
```

The validation is performed in `expand_modules` (lines 349-357), which `load` calls (line 443). While the statement is technically accurate (the error will fire during a `load` call path), a reader looking in `load` for the requires validation will find only the delegation and no inline check. The `guidance` field does not mention validation at all. Saying "validated in `expand_modules`" would be precise; or the comment could say "validated before any asset is read" to stay at the observable level.

This is the same language used on `AssetSpec.module` and `VarSpec.module`, where the check genuinely is inline in `load`. For `requires` the check is one level deeper, so the comment is slightly wrong.

### L2 - No test for a declared `guidance` file that does not exist

`src/manifest.rs` (test section)

There is a test for `UndeclaredModuleRequire` when `requires` names a missing module, but no parallel test for when the `guidance` field names a file that does not exist on disk. That path returns `LoadError::Io`. Given that the `instrument.md` analogy is already misleading (see M2), a test verifying the error behaviour would anchor the contract and protect it from a future change to silent-skip. The gap is small but notable next to the otherwise thorough test coverage for the new paths.

### L3 - CHANGELOG entry warranted but not present

`{{modules}}`, `guidance`, and `requires` are user-visible pack-authoring features. A pack author who reads the CHANGELOG to understand what the tool supports will not find them. The plan indicates a single changelog entry may be written at the end of increment 2 rather than per sub-increment, which is a legitimate approach. Flagging here so the orchestrator can confirm that intent and not accidentally close the increment without the entry.

---

## Items checked with no finding

- `RESERVED_VARS` updated to include `"modules"` (`src/manifest.rs:126`). Consistent with `{{principles}}` and `{{instrument}}`.
- README updated correctly: `{{modules}}` listed as the third reserved variable, the pack-format section extended with `guidance` and `requires` in the example TOML, and the prose explaining `requires` auto-enable, cycle tolerance, and the new error variant is accurate and matches the code.
- Ordering contract (declaration order) is documented in both the README and the `module_guidance` doc comment, and the code iterates `&manifest.module` to implement it. Consistent across all three.
- `expand_modules` returns a `HashSet<String>` (not ordered) used only for membership tests; declaration order is preserved only in `module_guidance` by iterating the `modules` slice. No ordering confusion.
- `pack/AGENTS.md` places `{{modules}}` after `{{instrument}}` at the tail. Correct position per the plan.
- `#[expect(dead_code, ...)]` is used on `description` (not `#[allow]`). Correct per project convention.
- Cycle test (`a_requires_cycle_terminates`) is present and exercises both directions.
- `enabled.insert(name.clone())` guards the BFS expansion so a cycle cannot loop. Fixed-point termination is correct.
- Error messages are ASCII-clean; no unicode or special characters observed.
- `UndeclaredModuleTag` comment ("Shared across both entry kinds...") remains accurate after the change; the new variant does not affect it.
