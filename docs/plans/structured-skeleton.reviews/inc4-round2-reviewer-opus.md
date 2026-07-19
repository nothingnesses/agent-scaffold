# Inc 4 round-2 (confirming) review: locator refactor + live-path invariance

Confirming round (round 2) on increment 4 of `structured-skeleton`. Lens: the `metrics::Waiver` `line: usize` -> `locator: String` refactor, live-path invariance, the two W5 TOML negatives, regression, and item 3 (the `status --source` parse note). Fix commit `87fa8fd` over round-1 `36a872f`; full increment `3f29a81..87fa8fd`. Reviewed read-only from the main repo and ran in the `structured-skeleton-inc4` worktree.

## Verdict: CLEAN

No findings at any severity (low, medium, high, critical). The round-1 fixes are correct and complete, and the live Markdown/JSONL path is provably unchanged. Details of what I verified are below so this is not a rubber-stamp.

## Construction sites: every `metrics::Waiver` sets `locator`, none missed

`git grep "Waiver {" 87fa8fd` returns exactly these `metrics::Waiver` sites, and each sets `locator`:

- `src/metrics.rs:894` (`parse_waivers`, JSONL): `locator: format!("round log line {}", index + 1)`.
- `src/workflow.rs:245` (`waivers_from_toml`, TOML): `locator: format!("TOML waiver `{}`", waiver.id)`.
- `src/metrics.rs:1482`, `:1491` (the `parse_waivers` test): `locator: "round log line 1"` / `"round log line 5"`.

The other two `Waiver {` hits (`src/plan/source.rs:237` decl, `:777`/`:790` tests) are a DIFFERENT struct, `source::Waiver`, which has its own `id` field and is unrelated to the `metrics::Waiver` locator. No metrics-Waiver site defaults to an empty locator, and no construction site was left stale by the field swap.

No stale `line` remnant on the waiver: `git grep "\.line" 87fa8fd -- src/metrics.rs src/workflow.rs` shows only `errors[0].line` (a validation-error struct) and `round.line` (a `Round` record); there is no surviving `waiver.line`. The renamed field compiles and clippy is clean, so no dead code from the swap.

`locator` is used only in `format!` at `src/workflow.rs:534,545,580,592` (all four W5 messages); it never enters a comparison, join, dedup, or enforcement decision, so the `usize -> String` type change is purely a message-carrier change. This preserves the round-1 finding that `line` was `format!`-only.

## Live JSONL W5 messages are byte-identical to the pre-increment baseline

For the JSONL path, `locator == "round log line {index+1}"` and each W5 `format!` is now `"{}: ..."` fed `waiver.locator`, reconstructing `"round log line {index+1}: ..."` -- the exact prefix the pre-increment code produced from `waiver.line`. The remainder of every W5 message string is unchanged in `3f29a81..87fa8fd` (diff confirms only the `line -> locator` placeholder swap, no text edits). So JSONL W5 output is byte-identical.

Live invocation is identical green: `cargo run -- validate --metrics docs/metrics/workflow.jsonl --plan docs/plans/agent-scaffold.md --workflow` -> `docs/plans/agent-scaffold.md vs docs/metrics/workflow.jsonl: workflow invariants hold`, exit 0. The live repo cannot reach the TOML branch (no `--source`, no auto-discovery), so no W5 wording on the Markdown/JSONL path can change.

## The two W5 negatives are non-vacuous and assert the correct locator

`check_workflow_toml_w5_rejects_a_mis_tiered_waiver` (workflow.rs:1582) and `_rejects_a_wrong_escalation_waiver` (:1606) both call `check_workflow_toml` on a real TOML fixture and assert `problem.contains("TOML waiver `w`")` AND the substantive W5 substring. If the locator were empty or wrong the `contains` would fail, so they are pinned to the substrate-correct id. Confirmed live with a hostile fixture (id `my-waiver-id`):

```
... vs .../empty.jsonl: TOML waiver `my-waiver-id`: waiver reason `predates-logging` must not carry evidence tier `record-backed`
... vs .../empty.jsonl: TOML waiver `my-waiver-id`: `record-backed` waiver cites evidence `x` but no `type:"escalation"` record ... is scoped to this waiver's unit
```

exit 1. The message names the waiver by its `[[step.waiver]].id`, not a JSONL line. The mis-tier test comment was also corrected to state that TWO W5 problems fire (the S3 fix); this matches the two lines above.

## Regression

In the worktree with the project toolchain:

- `cargo test --all-targets`: 293 + 1 + 3 passed, 0 failed.
- `cargo clippy --all-targets -- -D warnings`: clean, no warnings.

## Item 3: the `status --source` parse note

`toml_source` (`src/main.rs:839`) is called only from `run_status` (`:874`); `validate` derives its source via its own inline `plan::parse_toml(&contents).ok()` (`:762`) and reports a malformed source through `validate_source` (`:740`) into the problem list, so the note cannot leak into `validate` and enforcement/exit codes there are unchanged.

Verified behavior of the note:

- Malformed `--source` to `status`: one `note: --source <path> did not parse ...` on STDERR, projection falls back to `--plan`, exit 0 (unchanged).
- Markdown-primary `--source` to `status`: NO note (the `Ok(_) => None` arm), exit 0. So the note fires only on a genuine parse failure, is stderr-only, and changes neither the exit code nor enforcement (status is non-enforcing regardless).

## Settled round-1 findings, spot-checked as resolved

- MSG-LOCATOR (C1+L3+S1): resolved by the locator refactor; live TOML message now carries the id. Confirmed above.
- S2 + L1-comment: `--source`/`--workflow` help, the `run_validate` doc, and the `(Some(source), _, ...)` branch comment now state that a TOML-primary `--source` drives the check and that `--plan` stays clap-required (the relaxation deferred). Accurate.
- L2: covered by item 3 above.
- S3: covered by the mis-tier test comment fix above.
- The L1 CLI relaxation remains correctly deferred (not an Inc 4 shortfall); no new evidence to reopen it.

## Summary

Two consecutive clean rounds now stand for this risky increment. Nothing to fix.
