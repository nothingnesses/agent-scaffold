# Review: round-log-core increment A, round 2

Reviewer: confirming reviewer (claude-sonnet-4-6)
Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/round-log-core`
Branch: `impl/round-log-core`, HEAD `fc7f2b6`
Commits reviewed: `1824e7d` (feat: require risk_class + backfill), `fc7f2b6` (docs: V1 doc fix)

## V1 confirmation

`pack/instrument.md` now lists `risk_class` in the required-field sequence alongside `consecutive_clean`, and singles out `reviewers` as the sole optional calibration field with the explicit sentence "A record written without the optional `reviewers` field still validates." The old "Two optional calibration fields" phrasing is gone.

AGENTS.md line 129 and `.agents/AGENTS.reference.md` line 129 carry the identical updated prose, byte-for-byte matching `instrument.md`. V1 landed correctly and rendered into both derived files.

## Increment-A substance

- `src/metrics.rs` line 264: `require_enum(obj, "risk_class", RiskClass::VARIANTS, ...)` - required unconditionally on every `round` record.
- `src/metrics.rs` lines 270-271: `if obj.contains_key("reviewers") { require_reviewers(...) }` - optional, only validated when present.
- Test `the_optional_reviewers_field_is_accepted_present_or_absent` (line 452): confirms a round with `risk_class` plus `reviewers` is valid, and a round with `risk_class` but no `reviewers` is equally valid.
- Test `a_round_missing_risk_class_is_reported` (line 463): confirms a round without `risk_class` produces the error `"missing field \`risk_class\`"`.
- `docs/metrics/workflow.jsonl`: 52 round records, all carry `risk_class`; values are `low_risk` and `risky` only, no missing entries.

## Mechanicals

- `just test`: 96 passed, 0 failed, 0 ignored.
- `just clippy`: clean (no warnings, successful compile).
- `just scaffold-self` then `git status --short`: empty output, working tree clean and byte-identical after re-render.
- `direnv exec . cargo run -q -- validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl`: `docs/metrics/workflow.jsonl: 52 records, valid` and `docs/plans/agent-scaffold.md: 42 steps, 35 open-questions items, valid`.

## Drift guard

`instrument_prose_documents_every_accepted_schema_value` (line 534) includes `risk_class` in the field checklist (line 561) and `RiskClass` in the enum checklist (line 588). It passes as part of the 96 total and continues to enforce that the prose documents every accepted schema value.

## Verdict

Clean. No new findings. V1 confirmed. Increment A converges.
