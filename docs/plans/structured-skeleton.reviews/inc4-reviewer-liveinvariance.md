# Inc 4 review: live-path invariance, gate routing, robustness

Reviewer lens: LIVE-PATH INVARIANCE, GATE ROUTING, ROBUSTNESS. Commit `36a872f` on base `3f29a81`. Reviewed from the main repo; ran read-only in the `structured-skeleton-inc4` worktree with hostile `.plan.toml` fixtures in a scratch dir.

Verdict: the live Markdown path is provably unchanged. `check_workflow` delegates to `run_checks` with byte-identical inputs, order, and the same problem strings, and the live invocation cannot reach the TOML branch (no `--source`, no auto-discovery). The gate routes correctly on every branch I probed. Findings below are all on the TOML/ future path or cosmetic, none is a live-path regression.

## Findings

### L1 (medium): the TOML workflow branch claims "needs only the metrics log" but clap still forbids running it without `--plan`

- file: `src/main.rs:381` (`#[arg(long, requires = "plan")] workflow`), against the branch comment at `src/main.rs` (the `(Some(source), _, Some(metrics_text))` arm, "TOML-sourced: needs only the metrics log (the plan comes from the TOML)").
- defect: `--workflow` carries `requires = "plan"` (pre-existing from the Markdown-only design). Inc 4 added the TOML-primary branch that ignores `plan_contents` (the `_` in the match) and whose comment says it needs only the metrics log, but the clap constraint still makes `--plan` mandatory whenever `--workflow` is present. So a TOML-primary project cannot actually run `validate --workflow --source plan.toml` without also passing an otherwise ignored `--plan`. This directly contradicts the Inc 6 contract (`docs/plans/agent-scaffold.md` line 689: Inc 6 "needs `validate` to pass on a TOML-sourced project with no Markdown plan").
- repro:
  ```
  agent-scaffold validate --metrics <emptylog>.jsonl --workflow --source <toml-primary>.plan.toml
  # error: the following required arguments were not provided: --plan <PLAN>  (exit 2)
  ```
  Adding any `--plan` path makes it work and the `--plan` content is then ignored (TOML wins), confirmed with a toml-primary pause fixture: W3 fires from the TOML, not the Markdown plan.
- not a live regression: the live repo always passes `--plan`, so this cannot change live behaviour. It is a forward-looking gap plus a misleading in-code claim.
- suggested direction: either relax the constraint so `--workflow` requires "a plan OR a source" (clap `required_unless_present`/group), deferring the real fix to Inc 6 but fixing the misleading comment now; or, if Inc 6 is meant to own this, soften the branch comment so it does not assert a capability the CLI forbids. Decision for the triager: fix in Inc 4 vs defer to Inc 6.

### L2 (low): `status --source <malformed>.toml` silently falls back with no diagnostic

- file: `src/main.rs` `toml_source` (the `Ok(source) if is_toml_primary() => Some, _ => None` arm) and its use in `run_status`.
- defect: for `status`, an unparseable `--source` is swallowed to `None` and the projection silently falls back to `--plan` (or empty when no `--plan`). A user who intended a TOML projection and has a broken source sees a Markdown/empty projection with no signal that the source failed to parse. This is by design (`status` is a non-enforcing projection, and `validate --source` is the place that reports a malformed source), so it does not weaken enforcement, but it can mask a broken source in the projection view.
- repro: `status --source <malformed>.plan.toml --plan docs/plans/agent-scaffold.md` prints the 51-step Markdown projection, exit 0, no mention of the malformed source.
- suggested direction: acceptable as-is given the enforcement path (`validate`) always reports it; optionally emit a one-line stderr note when a given `--source` fails to parse in `status`.

### L3 (low, cosmetic): a TOML waiver's W5 problem message reads "round log line N" though there is no log line

- file: `src/workflow.rs` `waivers_from_toml` (the `line: position` field) surfaced by `w5_problems` messages ("round log line {line}: ...").
- defect: `waivers_from_toml` sets `Waiver.line` to the waiver's 1-based position in TOML document order, and W5 formats it as "round log line N". For a TOML-sourced waiver there is no round-log line, so a human reading a W5 violation on a TOML plan sees a phrase pointing at a non-existent log line. Confirmed cosmetic only: every use of `waiver.line` in `workflow.rs` (lines 535, 546, 581, 593) is inside a `format!` message; it is never used in any comparison, join, or enforcement decision, so it cannot mislead enforcement, only a human reading output. The position counter increments for dropped waivers too (before `continue`), so kept waivers get unique but non-contiguous positions; still unique, no collision.
- suggested direction: minor; when it becomes worth it, phrase the W5 message per substrate (e.g. "waiver `<id>` on step `<slug>`" for TOML) rather than "round log line N".

## Invariance and gate branches verified (no finding)

Markdown-path invariance (the load-bearing check):

- `check_workflow(&str, &str)` now calls `run_checks` with `parse_roadmap`, `parse_questions`, `parse_rounds`, `parse_decisions`, `parse_baseline`, `parse_waivers`, `parse_escalations` in the exact same order as the pre-Inc-4 body, and `run_checks` runs `round_log_consistency_problems` then extends with `w3_problems`, `w4_problems`, `w5_problems` in the same order. Same steps/questions/rounds/decisions/baselines/waivers/escalations, same checks, same ordering. Byte-identical.
- `report_workflow` produces the same summary string ("<src> vs <metrics>: workflow invariants hold") and the same per-problem string ("<src> vs <metrics>: <problem>") as the old inline code; only iteration changed from `&Vec` to by-value, same output.
- Live run is green and unchanged: `validate --metrics docs/metrics/workflow.jsonl --plan docs/plans/agent-scaffold.md --workflow` -> "docs/plans/agent-scaffold.md vs docs/metrics/workflow.jsonl: workflow invariants hold", exit 0. Full test suite: 293 + 1 + 3 pass.

Gate routing (`toml_source` / `is_toml_primary`), every branch probed:

- `--source` absent (the live repo): `source_plan`/`toml_source` is `None` -> Markdown path. There is no default value on `--source` and no auto-discovery of `.plan.toml`, so the live repo can never reach the TOML branch.
- `--source` a `.plan.toml` with `primary = "markdown"` -> `is_toml_primary()` false -> Markdown path (verified: 51-step Markdown projection / Markdown workflow check, green).
- `--source` with `[meta].primary` absent (default `markdown`) -> Markdown path (verified).
- `--source` with `primary = "toml"` -> TOML path (verified: pause fixture fails W3, exit 1; clean fixture green).
- `--source` malformed/unparseable: in `validate`, `validate_source` REPORTS "malformed `<task>.plan.toml`: ..." (exit 1) and the workflow gate falls back to Markdown; enforcement cannot be silently bypassed because any parse error always yields a problem -> non-zero exit. In `status`, silent fallback (see L2).
- `--source` toml-primary given alongside `--plan`: the TOML wins (the match arm ignores `plan_contents` via `_`); verified the Markdown `--plan` content is not consulted. Sound: this is the gate's intent.
- No input found under which the live Markdown repo routes to TOML, or a Markdown-primary/absent source routes to TOML.

STATUS parity:

- toml-primary `--source` projects from the TOML (`step_views`/`question_views`); markdown-primary or absent source projects from `--plan`; live `status` (no source) unchanged (51 steps, 125 records). Verified.

The two flagged choices:

- `waivers_from_toml` drop mirror: a presence-rule-violating TOML waiver (step-unit carrying an `increment`; increment-unit with no `increment`; record-backed with no `evidence`; self-declared with `evidence`) is DROPPED, exactly as `parse_waivers` drops the JSONL equivalent, so it grants no W3 exemption. Verified end-to-end: a step-unit waiver carrying an `increment` over a `complete` no-rounds step is dropped, W3 fires, AND `validate --source` reports "waiver `w` on step `s` has unit `step` but carries an `increment`" (exit 1), so the drop is never invisible. The `reason <-> evidence_tier` pairing is deliberately NOT filtered here (W5 reports it, matching `parse_waivers`). TOML enums make bad reason/tier values a parse error at the boundary, so the JSONL "bad enum drops the waiver" path is subsumed by parse.
- `Waiver.line` "round log line N" message: cosmetic-only, cannot mislead enforcement (L3).

Robustness:

- No `unwrap`/index on TOML-derived data in `waivers_from_toml`, `baseline_from_toml`, `step_views`, `question_views`, or `run_checks`; the only `.max().unwrap_or(0)` is the pre-existing W3 peak-streak on JSONL rounds. `question_views` uses `unwrap_or_default()` for a missing `folded_into` (safe; yields an empty fold target, still W4-in-scope, the safe require-a-receipt direction).
- `baseline_from_toml`: an absent or non-`Q-<n>` `[meta].w4_baseline` yields NO baseline, so W4 requires a receipt for every decided item (the safe direction), and `validate --source` reports a non-`Q-<n>` cutoff.
- Zero-step / zero-waiver toml-primary source with an empty log: no panic, W3/W4 vacuous, round-log consistency still runs; green. Verified (`empty.plan.toml`).
- W4 decided-gate parity: `question_views` renders `QuestionStatus::Decided` + `folded_into` to `decided -> folded into \`<slug>\``, which starts with `QUEUE_FOLD_PREFIX`, so W4's `starts_with(QUEUE_FOLD_PREFIX)` selects the same decided set as the Markdown parser; other statuses map to plain labels that do not match the prefix, so scope is identical across substrates.
