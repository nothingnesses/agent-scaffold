# Reviewer findings: optional-modules increment 1 (--module machinery)

Lens: CORRECTNESS. Range reviewed: `74083fa..f34ed1c`. Files: `src/manifest.rs`, `src/main.rs`.

## Verdict

The core reviewed behaviour is correct. The loader filter, the two validation errors, exit-2-and-write-nothing, serde defaults, and call-site threading all check out against the code, against fixture packs, and against the plan's design-pass decisions (plan line 490). No critical, high, or medium findings. Two low findings, both about design elements not yet realised rather than defects in what was built.

## Verification performed

- `direnv exec . just test`: 83 passed, 0 failed (matches the expected 83).
- `cargo clippy --all-targets -- -D warnings`: clean.
- `just scaffold-self`: `git diff --stat -- AGENTS.md .agents/ docs/plans/TEMPLATE.md` is empty (byte-identical; module-free built-in pack renders unchanged, confirming the "no module selected -> byte-identical" claim).

Fixture-pack probing in the scratchpad (`--principles none`):

- Loader filter, order preserved: fixture with core (untagged, declared first) + `extra` tagged `extras`. No `--module` -> only `core.md`. `--module extras` -> `["core.md", "extra.md"]` in manifest order. Matches `manifest.rs:319-329` (`into_iter().filter(..).map(..)` over the `Vec`, so order is preserved for included assets).
- `LoadError::UnknownModule`: `--module bogus` -> `error: no module named bogus is declared by the pack`, real exit code 2, output dir never created (nothing written). The check (`manifest.rs:290-294`) runs before variable resolution and reading, so the load aborts before any write.
- `LoadError::UndeclaredAssetModule`, order-independent of selection: pack with a `ghost`-tagged asset and NO `[[module]]`, run with NO `--module` -> still errors `asset ghost.md is tagged with module ghost, which no [[module]] declares`, exit 2, nothing written. Confirms the per-asset tag check (`manifest.rs:297-306`) runs for every asset regardless of what is selected.
- Empty `--module ""` -> `UnknownModule("")` (exit 2), since no module is named "". Correct.
- `--module extras --module extras` (same module twice) -> deduped by the `HashSet` selection set, both assets load once. Correct.
- Declared module tagging no asset, selected -> drops nothing, no error (covered by the added test `selecting_a_declared_module_with_no_assets_drops_nothing_extra`, reproduced).
- Serde: `AssetSpec.module` (`#[serde(default)]` Option) and `Manifest.module` (`#[serde(default)]` Vec) both default to absent/empty, so packs with no `module` field and no `[[module]]` section still parse; the unchanged `builtin_manifest_lists_the_expected_assets` test passes.
- Call sites: `build_assets` is threaded to `manifest::load` (`main.rs:218`); `run_scaffold` passes `&args.modules` (`main.rs:610-616`); the test-only `scaffold` helper and the in-module tests pass `&[]`. `run_scaffold` calls `build_assets` directly, not the `scaffold` helper, so the real write path carries the flag. `LoadError` -> `eprintln!` + `std::process::exit(2)` at `main.rs:619-622`, before `init_plan` and any write.

## Findings

### Finding 1 (low): `[[var]]` module tagging from the design is not implemented and a `module` key on a `[[var]]` is silently ignored

The design-pass decision (plan `docs/plans/agent-scaffold.md` line 490) describes the hybrid schema as an optional `module = "<name>"` field "on `[[asset]]` (and `[[var]]`) entries". This increment added the field only to `AssetSpec` (`manifest.rs:57-58`), not to `VarSpec` (`manifest.rs:79-86`). Because the manifest tolerates unknown keys by design, a `module` key placed on a `[[var]]` is accepted and ignored: I confirmed with a fixture that a `[[var]]` carrying `module = "extras"` is still resolved and substituted even with no `--module` selected (output `hi world`), rather than being scoped to the module or rejected.

This is not a defect in the asset-filtering machinery that is the heart of this increment, and it may be a deliberate deferral (no shipped module needs a module-scoped variable yet, and this increment's stated scope is the `--module` machinery). It is flagged only because the design text names `[[var]]` tagging explicitly and the current behaviour accepts the tag silently rather than either honouring it or flagging it, which could mislead a pack author who follows the design. If var tagging is intended for a later increment, a one-line note in the plan pinning that would close the gap; no code change is required for correctness of increment 1.

### Finding 2 (low): duplicate `[[module]]` names are accepted silently (no one-source-of-truth check)

`declared` is built as a `HashSet<&str>` over the module names (`manifest.rs:289`), so two `[[module]]` entries with the same `name` (and differing `description`s) collapse to one and load without error. I confirmed a pack with two `name = "extras"` modules scaffolds cleanly. Filtering is unaffected (the name is either in the set or not), so this is not a filtering or validation bug. It is a mild Principle 16 (one source of truth) gap: the plan calls `[[module]]` "the authoritative set", and a pack author who duplicates a module name (with a conflicting description) gets no signal. Low impact; the description field is not yet read by the loader (`#[expect(dead_code)]` at `manifest.rs:72`), so a conflicting description has no runtime effect today. Worth a duplicate-name check when the description is actually consumed (a later increment / the TUI), rather than now.

## Not findings (per scope and checks)

- The built-in pack shipping module-free, and the absence of the checks and isolation modules (increments 2-3): out of scope by instruction.
- `ModuleSpec.description` being required (no `#[serde(default)]`): a `[[module]]` missing a description fails to parse into `LoadError::Io`. This is fail-fast and matches the design ("name and description"), so it is acceptable, not a finding.
- The `#[expect(dead_code)]` on `description`: correctly annotated with a reason; it is declared for the schema/TUI and not yet read. Consistent with the memory note preferring `expect` over `allow`.
