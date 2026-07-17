# Review: `reviewer-harness-field` (reviewer-sonnet)

Reviewer lens: single-source-of-truth, code-vs-prose consistency, doc clarity. Diff range: `305ea86..9f87003`. Commit: `9f87003`. Files touched: `pack/instrument.md`, `src/metrics.rs`, `AGENTS.md`, `.agents/AGENTS.reference.md`.

---

## Critical

None.

## High

None.

## Medium

### M-1: CHANGELOG entry absent for a user-visible schema addition

`CHANGELOG.md` exists and its `[Unreleased]` section is actively maintained. The change adds `harness` to the `reviewers[]` documentation in `pack/instrument.md`, which is embedded verbatim into every scaffolded project's `AGENTS.md`. That makes this a user-visible schema and documentation change: any project scaffolded after this commit, or re-scaffolded via `scaffold-self`, picks up the new field description.

The existing [Unreleased] section already carries a comparable entry:

> Generalised the reviewer and explorer diversity guidance in `AGENTS.md` from "different models where available" to "different models or harnesses where available"...

The `harness` field addition is the same class of change (adds harness to the instrumentation schema that ships in `AGENTS.md`) and is tagged `feat:` in the commit message. No entry appears under [Unreleased] for this commit.

Files: `CHANGELOG.md` (no change in diff).

## Low

### L-1: Drift-guard field list ordering inconsistent with validator and prose

The drift-guard test's field array in `src/metrics.rs` (around line 705) inserts `harness` between `model` and `raw_findings`:

```
"model",
"harness",   <- inserted here
"raw_findings",
```

But in `require_reviewers` the validation order is `role`, `model`, `raw_findings`, `valid_findings`, then `harness` (optionally, last). The prose in `pack/instrument.md` matches the code: "a string `role`, a string `model`, and non-negative integer `raw_findings` and `valid_findings` counts, plus an optional string `harness`".

The test's comment (`src/metrics.rs` around line 672) says the list "mirrors what `check_record` matches on and requires". After this insertion the list no longer mirrors the ordering in either the validator or the prose, making it a misleading reference for someone reading it to understand the schema structure. The test itself still passes because it checks for presence (`` `harness` `` in prose), not ordering.

Suggested fix: move `"harness"` after `"valid_findings"` in the drift-guard field array so it matches both the validation order in `require_reviewers` and the prose's "plus an optional string `harness`" phrasing.

Files: `src/metrics.rs` ~line 705.

### L-2: `require_reviewers` doc comment purpose clause not updated to mention harnesses

The doc comment on `require_reviewers` was updated in `src/metrics.rs` to add the harness field description (correct), but the purpose clause at the end was left unchanged:

```
/// per-reviewer breakdown used to calibrate reviewer productivity and whether
/// running multiple models earns its cost
```

The corresponding prose in `pack/instrument.md` was correctly extended to "running multiple models or harnesses can be calibrated". The doc comment's purpose clause now under-states the function's scope: it covers harness tracking too, but still mentions only models.

Suggested fix: update the clause to "running multiple models or harnesses earns its cost" (matching the prose) so the two statements are consistent.

Files: `src/metrics.rs` around line 235.

---

## Checks: no issues found

- `harness` appears backtick-wrapped twice in `pack/instrument.md` (`\`harness\``), satisfying the drift-guard's backtick anchor requirement.
- The validator behavior (optional, string-when-present) matches the prose description ("optional string `harness`") exactly. No over- or under-statement.
- The regenerated `AGENTS.md` and `.agents/AGENTS.reference.md` are identical to the `pack/instrument.md` change; no drift between the pack source and the self-scaffolded copies.
- The example values (`claude-code`, `codex`, `gemini-cli`) are illustrative and framed as "for example"; all three refer to current CLI tools. The prose does not over-specify by implying these are the only valid values.
- The existing per-reviewer counting semantics (raw/valid counts, deduplication note) are intact; the `harness` insertion does not confuse that paragraph.
- Both new validator tests (valid harness accepted; non-string harness reported) match the plan's specified scope and cover the two material cases.
- No unicode or non-ASCII characters introduced.
