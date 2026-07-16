# Triage: optional-modules increment 1 (`--module` machinery)

Range: `74083fa..f34ed1c`. Artifact class: LOW-risk (one clean round to converge). Reviewers adjudicated: opus (correctness, 2 low), sonnet (design/docs, 2 medium + 3 low). Findings deduplicated across reviewers below. Grounded against `docs/plans/agent-scaffold.md` (the Q-25 HYBRID decision, line 490; the `instrument-flag` outcome, line 553; the Pack format and Repository Layout sections, lines 61-67 and 56-58) and `src/manifest.rs`.

---

## V-1 (was sonnet F-2 medium + opus Finding 1 low): `[[var]]` module tagging is not implemented

Verdict: VALID, medium. (Confirms sonnet's medium over opus's low.)

Reasoning: the Q-25 HYBRID decision (plan line 490) states the module field applies to "an optional `module = \"<name>\"` field on `[[asset]]` (and `[[var]]`) entries". The increment added `module: Option<String>` to `AssetSpec` (`manifest.rs:57-58`) but not to `VarSpec` (`manifest.rs:79-86`), and `resolve_vars` (`manifest.rs:170-201`) is not module-filtered: it requires every declared var unconditionally (`manifest.rs:195` returns `MissingRequiredVar` for any required var not supplied). This is in scope for increment 1, whose job is "the `--module` machinery" and whose decided schema names `[[var]]` explicitly, so it is completion of the decided design, not scope expansion (Principle 8 is not tripped). Leaving it out creates a latent Principle 2 violation ("adding a module must not change core behaviour when unused"): the moment a module declares a required `[[var]]` (increment 2, the checks module, is the very next step and the design anticipates modules carrying `[[var]]` entries, plan line 469), a user who does not pass `--module <that-module>` gets a hard `MissingRequiredVar` failure for a variable belonging to a module they did not opt into. It is medium, not high/critical, because no shipped pack triggers it today (the built-in pack is module-free and tags no vars), so there is no current wrong output or data loss; it is above low because it is a decided-design omission that produces a hard failure as soon as the immediately-following increment lands, and deferring it would force increment 2 to build machinery inside a module-content step, breaking the increment separation the plan set (line 488).

Fix (implementer): add `module: Option<String>` to `VarSpec`; validate each var's module tag against the declared `[[module]]` set (a new `UndeclaredVarModule`-style error, analogous to `UndeclaredAssetModule` at `manifest.rs:302-313`); and filter the vars so a module-tagged var is only required and only resolved when its module is selected (a var tagged with an unselected module is skipped entirely, neither required nor substituted), leaving the core (untagged) vars unchanged so no-module output stays byte-identical.

Owner: implementer (code, `src/manifest.rs`; thread selected modules into `resolve_vars` or pre-filter before the call at `manifest.rs:315`).

---

## V-2 (was sonnet F-1 medium): README pack-format section does not document `[[module]]`, the asset `module` field, or `--module`

Verdict: VALID, medium.

Reasoning: `grep -n module README.md` returns no hits; the "Bring your own pack" section documents `[[asset]]`, `[[var]]`, `{{principles}}`, and `{{instrument}}` but nothing about the module machinery, so a pack author cannot discover how to declare a module, tag an asset (Principle 20, self-contained docs; the feature is undiscoverable without them). This is the same finding class as the `instrument-flag` review's "Pack format docs did not mention `{{instrument}}` or `instrument.md`", which the triager there upheld as a valid medium and which was fixed in both the README and the plan (plan line 553). Consistency with that precedent and the same reader-facing impact make this medium.

Fix (implementer): in the README pack-format section, document the `[[module]]` section (its `name` and `description` fields), the optional `module = "<name>"` field on `[[asset]]` entries (and on `[[var]]` entries once V-1 lands, to keep the docs matching the shipped schema), and the repeatable `--module <name>` flag with its interaction (untagged assets always drop; a tagged asset drops only when its module is selected; an unknown `--module` errors and writes nothing).

Owner: implementer (README is code-side, not the plan).

---

## V-3 (was sonnet F-3 low + F-4 low): the plan's Pack-format and Repository-Layout descriptions are stale

Verdict: VALID, low.

Reasoning: the plan's "Pack format" block (lines 61-67) and the Repository Layout "Modules and key types" descriptions for `src/manifest.rs` (line 58) and `src/main.rs` (line 56) predate this increment. The `manifest.rs` description still reads `Manifest { asset, var }` (no `module: Vec<ModuleSpec>`), `AssetSpec { source, dest, ownership, render }` (no `module: Option<String>`), `load(source, builtin_vars, overrides)` (no `selected_modules` parameter), and `LoadError { Io, UndeclaredVar, MissingRequiredVar, ReservedVar }` (no `UnknownModule` / `UndeclaredAssetModule`); the `main.rs` flag list omits `--module`. The plan's Repository Layout is the stated resume context for a fresh implementer (plan Status line and line 50), so stale type signatures and a missing error variant will mislead a later agent reading it to understand `load()` (Principle 9, durable notes; Principle 16, one source of truth). Low, because it misleads only resume context and changes no runtime behaviour. The `instrument-flag` step set the precedent of updating this plan section alongside the README (line 553), so the same update is owed here.

Fix (orchestrator): update plan lines 61-67 and the Repository Layout descriptions at lines 56-58 to the shipped schema: add the `[[module]]` section and the `module` optional field on `[[asset]]` (and `[[var]]` after V-1), the `selected_modules` parameter on `load()`, the two (soon three, with V-1) new `LoadError` variants, and the `--module` flag.

Owner: orchestrator (the plan is orchestrator-owned; the implementer must not edit `docs/plans/`). Noted here for the orchestrator to apply, not routed to the implementer.

---

## V-4 (was sonnet F-5 low + opus Finding 2 low): duplicate `[[module]]` names are silently deduped

Verdict: VALID, low.

Reasoning: `declared` is built as a `HashSet<&str>` over module names (`manifest.rs:295`), so two `[[module]]` entries sharing a `name` (with differing `description`s) collapse to one and load without error, while the sibling structural checks in the same block hard-fail (an unknown `--module` is `UnknownModule`, `manifest.rs:297-301`; a dangling asset tag is `UndeclaredAssetModule`, `manifest.rs:302-313`). A pack-authoring structural error accepted silently is inconsistent with that fail-loud treatment (Principle 12, fail fast and loudly) and with the plan calling `[[module]]` "the authoritative set" (Principle 16, one source of truth). Low, because the impact is benign today: the deduped set still validates `--module` and filters assets correctly, and the conflicting `description` has no runtime effect since it is unread (`#[expect(dead_code)]`, `manifest.rs:72`). The fix is trivial and sits in the same validation block as its two siblings, so completing the fail-loud set now is the cleaner long-term design (Principle 17) rather than waiting until the description is consumed.

Fix (implementer): after building `declared`, add a `DuplicateModule(String)` (or equivalent) `LoadError` variant and check `manifest.module.len() == declared.len()` (or detect the first repeated name), failing the load so nothing is written, matching the other structural pack-authoring errors.

Owner: implementer (code, `src/manifest.rs`).

---

## Round outcome

Round 1 is NOT clean: four deduplicated findings, all VALID (2 medium, 2 low), none dismissed, none invalid. No high- or critical-severity dismissal occurred, so no backstop re-check is required. Three findings are implementer-owned code/README fixes (V-1, V-2, V-4) and one is orchestrator-owned plan docs (V-3). Because new valid findings landed, the consecutive-clean streak stays at zero: the implementer addresses V-1, V-2, V-4 and the orchestrator applies V-3, then a fresh round 2 runs on the revised artifact. As a LOW-risk artifact it needs one clean round to converge, so round 2 clean would close the loop.
