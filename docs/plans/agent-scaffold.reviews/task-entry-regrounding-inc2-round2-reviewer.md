# task-entry-regrounding-inc2 Round 2 re-review (adversarial, read-only)

Reviewer: fresh independent Round 2 reviewer (first potential clean round after new_valid).
Scope: diff `7d8702c..4966db6` on `impl/ter-inc2`; two fix commits `06571ca` (I2-1) + `4966db6` (I2s-1) on top of the 3 build commits.

## Verdict: CLEAN

No findings. No remaining false claim, no validation bypass, no round-trip corruption, no scope creep. The deferred I2-2 (empty-string sidecar ref) is NOT re-raised.

## What I probed

FIX I2-1 (false round-trip ordering rationale):
- `src/plan/source.rs:126-132` (Step doc comment), `:151-158` (`provenance` field comment), and `:1271-1300` (test `a_populated_provenance_parses_and_round_trips_alongside_increments_and_waivers` docstring) now state the placement is a readability/grouping choice and explicitly say it is NOT a round-trip constraint. No false MUST claim remains.
- Field was NOT moved: `provenance: Option<Provenance>` still sits between `folds` (`:150`) and `increments` (`:159`) in the struct; `git show 06571ca --stat` touches only `src/plan/source.rs` with 17/16 comment-line churn, no field reorder, no validation/guard edit.
- The round-trip test body is unchanged in substance and still a genuine check: it asserts `plan == reparsed` after `toml::to_string` -> `parse_toml` on a step carrying provenance + increment + waiver, plus `validate_source(...).is_empty()`. Passes.
- Independent sanity check of the corrected claim (a fully-qualified `[step.provenance]` header binds to its `[[step]]` regardless of declaration position): via `python3 tomllib`, a `[step.provenance]` declared AFTER `[[step.increment]]` still binds to `step[0]` (keys `['slug','increment','provenance']`). Confirms declaration order does not change binding, so the old ordering constraint was indeed false and the correction is accurate.

FIX I2s-1 (exemplar cited an output commit):
- `docs/plans/agent-scaffold.plan.toml:714-715`: `task-entry-regrounding` `[step.provenance]` now carries only `decisions = ["Q-53"]`, no `commits`. `git show 4966db6` removed exactly the `commits = ["1e1d26f"]` line.
- `human-input-gate-reinforce` `[step.provenance]` still `decisions = ["Q-54"]` (`:731-732`).
- `docs/plans/agent-scaffold.md:207-208` re-rendered: row shows `why: decisions Q-53` and `why: decisions Q-54`, no commits.
- `render --check --strict docs/plans/agent-scaffold.plan.toml` -> `up to date`, exit 0. Fixture `render --check --strict` also `up to date`, exit 0. `validate --plan` on the live plan exits 0.

FINAL adversarial sweep (converging round):
- D1 validation (`src/plan/source.rs:622-671`): decisions resolve via `question_id_index` (shape) then `question_ids.contains` (existence), fail-closed; commits via `is_commit_shaped` (`:503-505`: 7..=40 lowercase hex, uppercase rejected, empty rejected by len range); findings via `is_safe_sidecar_ref`; empty-block (all three lists empty) rejected (D5). Ten new tests cover dangling decision, malformed decision id, non-hex commit, over-length (41) commit, `..` and absolute findings, empty block, and the fully-populated positive case; all non-vacuous and passing.
- `validate_source` stays pure: no fs/git calls added; `is_commit_shaped`/`is_safe_sidecar_ref` are pure string/path shape checks. Confirmed by the commit-shape and findings tests asserting shape-only rejection.
- Back-compat: `a_step_without_provenance_round_trips_with_no_provenance_key` asserts a no-provenance step deserialises to `None`, validates clean, and re-serialises with no `provenance` key (`skip_serializing_if = "Option::is_none"`).
- Render fragment (`src/plan/render.rs:494-513`): decisions->findings->commits fixed order, present-lists-only, appended LAST in `notes_cell` after blocked/waived, joined with `; `. Deterministic (Vec source order). Escaped: `notes_cell` output passes through `escape_cell` at `:467`, which escapes `|` and neutralises CR/LF, so a `|` in a findings path cannot break the table. `deny_unknown_fields` on `Provenance` (`:242`) proven by `a_mistyped_provenance_inner_key_fails_to_parse`.
- Scope: diff touches only 6 files (source.rs, render.rs, two testdata fixtures, the two live docs). No `next.rs`/metrics/W-check changes (`git diff` over those paths is empty). No `[[step.increment]]` declared for inc2 (only `task-entry-regrounding-inc1` present, `:721-723`); `task-entry-regrounding` status still `in-progress`, so no orchestrator close action leaked into the impl branch.

STYLE + regression:
- Added lines scan for non-ASCII / `--` / `#[allow`: none found. Comments use ASCII arrows (`->`) and `<->`-style ASCII only.
- `just test`: 334 + 1 + 3 + 1 + 2 pass, 0 failed (no index.lock flake this run).
- `just clippy`: clean (the manual `(b'a'..=b'f')` hex range is intentional to reject uppercase, and clippy did not object).
- `just build`: clean.
