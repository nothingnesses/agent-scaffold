# Review: instrument-flag (design consistency)

Reviewer: sonnet (design-consistency lens) Diff range: a5264a2..1cd3211 Date: 2026-07-15

## What was reviewed

The single commit 1cd3211 ("feat: add opt-in --instrument flag for workflow metrics") adds `pack/instrument.md`, appends `{{instrument}}` to `pack/AGENTS.md`, adds the `--instrument` flag to the CLI, reserves the variable name, and covers it with tests. This review checks schema coherence against `workflow-calibration` and `Q-24/state-schema`, JSONL format accuracy, the fragment-not-an-asset pattern, and internal consistency.

---

## Schema coherence: `pack/instrument.md` vs. `workflow-calibration` vs. `Q-24/state-schema`

The field mapping is coherent. Every field the `workflow-calibration` "Data to gather" paragraph requests is present in `pack/instrument.md`, and every field in `pack/instrument.md` has a clear purpose traced to a calibration signal. Specific checks:

- `round` type: `artifact`, `phase` (`plan_review`/`work_review`/`acceptance`), `changed_since_prev`, `outcome` (`clean`/`new_valid`), `valid_findings`, `severities`, `consecutive_clean` - all seven fields map to the calibration's "per round" list with no gaps and no extras.
- `escalation` type: `artifact`, `human_decision` (`decision`/`resume`) - maps to calibration's "useful versus friction" signal.
- `dismissal_recheck` type: `artifact`, `result` (`upheld`/`overturned`) - matches the AGENTS.md backstop-recheck terminology and the calibration's dismissed-high-severity event.
- `intake` type: `classification` (`trivial`/`non_trivial`), `replanned` - maps to the calibration's "per human interrupt" list.

Q-24 names the four types (`round`/`escalation`/`dismissal_recheck`/`intake`) and defers the field list to `pack/instrument.md`. The `state-schema` step repeats the same four type names. All three sources are consistent.

No drift, missing fields, or phantom fields were found in the schema itself.

---

## JSONL format description

`pack/instrument.md` states: append-only, one JSON object per line, to `docs/metrics/workflow.jsonl`, creating `docs/metrics/` if it does not exist, never rewriting past lines, with a `type` discriminator. This matches the `instrument-flag` step's "Data-output format (decided)" paragraph (plan line 522) and Q-5 (`docs/metrics/workflow.jsonl`) and Q-6 (orchestrator writes directly). The format description is accurate and unambiguous.

---

## Fragment-not-an-asset pattern

`pack/instrument.md` is read via `source.read("instrument.md")` in `build_assets()` and is absent from `pack/pack.toml`. This is structurally consistent with how `principles.toml` is handled: both are special pack files read directly rather than dropped as assets. The code comment in `build_assets()` makes the analogy explicit ("A pack that ships no instrument.md renders nothing, exactly as a pack without principles.toml renders no principles."). The file is correctly absent from `pack/pack.toml` and no test or manifest entry expects it to be dropped to the target project.

---

## Findings

### F-1 (medium): Pack format documentation does not document `{{instrument}}` as reserved or `instrument.md` as a special pack file

The "Repository Layout and Current Architecture" section of `docs/plans/agent-scaffold.md` describes the pack format contract. It currently says:

> "`render = true` applies `{{name}}` substitution. `{{principles}}` is a reserved, tool-computed variable (the rendered selection); a pack may neither declare nor `--var`-set it."

After this commit `{{instrument}}` is also reserved (`src/manifest.rs` line 76: `const RESERVED_VARS: &[&str] = &["principles", "instrument"]`), but the plan text was not updated to say so. Similarly, the plan documents `principles.toml` as a special pack file ("A pack that ships `principles.toml` has its principles drive selection...") but says nothing equivalent about `instrument.md`.

A custom pack author reading the plan's pack format description would not know that `{{instrument}}` is reserved, or that adding `instrument.md` to their pack enables instrumentation support. This is the documented contract for third-party packs, so the omission is a real gap, not just a comment. The plan's own Principle 1 ("Prefer the cleaner long-term architecture; prioritise internal coherence") and Principle 7 ("Reproducible - prefer the project's toolchain conventions so things behave the same on any machine") both support keeping this description accurate.

Evidence: `docs/plans/agent-scaffold.md` pack format paragraph (around line 64-68); `src/manifest.rs` line 76; `src/main.rs` `build_assets()` call to `source.read("instrument.md")`.

### F-2 (medium): "Byte-identical when off" claim is wrong; the commit message repeats the wrong claim and the test does not verify it

The `instrument-flag` step in the plan (line 524) states:

> "Evidence-grounding: with the flag on, a scaffolded project's workflow appends parseable JSONL records that a small script aggregates into the calibration inputs; **with it off, the scaffold output is byte-identical to the non-instrumented run.**"

The commit message repeats this: "empty and byte-identical to today when off" and "byte-identical-when-off verified."

From the code:

1. `pack/AGENTS.md` now ends with `{{principles}}\n\n{{instrument}}\n` (the diff adds a blank line and the `{{instrument}}` line).
2. When `instrument=false`, `build_assets()` sets `instrument_block = String::new()`, so `{{instrument}}` is substituted with `""`.
3. The `render()` function (`src/manifest.rs`) does plain `str::replace` with no whitespace normalisation.
4. Result: the rendered AGENTS.md ends with `<principles-content>\n\n\n` (two extra newlines) instead of the pre-commit `<principles-content>\n`.

The output is not byte-identical; it has one extra blank line at the end whenever `--instrument` is absent or false.

The test `instrument_off_omits_the_block_and_on_includes_it` checks only that the section heading is absent and the placeholder is not verbatim in the output. It does not compare the output against any pre-commit baseline and therefore does not verify the claim. The claim in both the plan's evidence-grounding and the commit message is unverified and incorrect.

This matters for two reasons. First, `scaffold-self` run with `--instrument=false` (the default) would now produce a modified live AGENTS.md (extra trailing blank lines), which would show up as an unwanted diff. Second, the plan's Principle 6 says "Ground decisions in evidence... validate it with a small proof-of-concept that builds and produces the expected output"; the stated evidence-grounding is wrong, so the validation this principle demands was not done.

Evidence: `pack/AGENTS.md` diff (trailing `\n\n{{instrument}}`); `src/manifest.rs` `render()` function (no whitespace handling); `src/main.rs` `build_assets()` (empty string for instrument=false); test in `src/main.rs` lines 617-643 (does not assert output identity); plan line 524.

A fix would be either to adjust the template so the surrounding blank line is only present when the block is non-empty (for example by folding the leading blank line into `instrument.md` itself, then stripping the trailing newline from the fragment before insertion), or to update the plan and commit claim to say "the instrumentation section is absent" rather than "byte-identical."

### F-3 (medium): The plan's Roadmap and Status line were not updated when the implementation was committed

The Roadmap shows `instrument-flag | in progress` (`docs/plans/agent-scaffold.md` line 144). The commit 1cd3211 appears to complete this step (the `--instrument` flag, the `pack/instrument.md` fragment, the reserved-var guard, and the tests are all in place), but no change was made to `docs/plans/agent-scaffold.md` in this commit (confirmed: `git diff a5264a2..1cd3211 -- docs/plans/agent-scaffold.md` is empty).

The plan's Documentation Protocol says: "Progress lives in the Roadmap... The implementer keeps it current." The Roadmap is described as "the single source of truth for status and for implementation order." Leaving it at `in progress` after the step is implemented means the Roadmap diverges from reality; a workflow resumed from the plan would believe this step still needs work.

Evidence: `docs/plans/agent-scaffold.md` line 144 (status `in progress`); no plan diff in the range a5264a2..1cd3211.

---

## What is clean

- Schema coherence: all four record types and all their fields in `pack/instrument.md` map exactly to the `workflow-calibration` "Data to gather" list. No field is missing; no field is orphaned.
- JSONL format: the file path, append-only constraint, create-dir instruction, and type discriminator in `pack/instrument.md` are all consistent with the plan's Q-5, Q-6, and "Data-output format (decided)" paragraph.
- `{{instrument}}` variable naming: consistent with the `{{principles}}` convention (double-braces, lowercase snake_case).
- Reserved-var guard: `instrument` added to `RESERVED_VARS` in `src/manifest.rs`; both the declared-by-pack case and the `--var` override case are tested by `reserved_instrument_variable_is_rejected`.
- `instrument.md` not in `pack.toml`: correct, it is a render fragment not an asset, and no manifest entry or test expects it to be dropped.
- Flag help text: accurately describes the flag's effect and references `docs/metrics/workflow.jsonl` (consistent with Q-5).
- Q-24/state-schema alignment: the four type names in `pack/instrument.md` match exactly what Q-24 and the `state-schema` step reference; the schema is defined once in `pack/instrument.md` and not re-specified elsewhere.
- No undocumented scope expansion: the change adds only what the decided design specifies.
