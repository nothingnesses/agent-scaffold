<!-- reviewer: round 3 (final confirming), independent fresh eyes -->
<!-- increment: structured-skeleton-inc4 (RISKY), full range 3f29a81..87fa8fd (HEAD 87fa8fd) -->
<!-- scope: W3/W4/W5 + status pointed at the TOML source behind the [meta].primary == "toml" gate -->

# Inc 4 round-3 (final confirming) review: TOML-source enforcement swap

Fresh, independent, adversarial full-increment pass. Role separation held: I did not write this code. Round 1 fixed 5 low findings; round 2 was clean twice. A clean round here converges the increment. This is a genuine re-attack, not a rubber-stamp; what I probed and what I ran are below.

## Verdict: CLEAN

No findings at any severity (low, medium, high, critical). The source-selection gate routes correctly for every input I could construct, the normalizers map faithfully into the checks' shapes, the malformed-waiver drop fails closed AND is reported, the cross-substrate W5 join stays un-launderable and substrate-correctly located, and the live Markdown/JSONL path is byte-for-byte unchanged.

## Verification (Principle 6)

Ran read-only in the worktree `structured-skeleton-inc4` with the project toolchain:

- `cargo test --all-targets`: 293 + 1 + 3 passed, 0 failed.
- `cargo clippy --all-targets -- -D warnings`: clean, no warnings.
- `cargo build`: clean; drove the built binary against hostile fixtures in a scratch dir.

Read the increment via `git diff 3f29a81 87fa8fd` and `git show 87fa8fd:<path>` from the main repo (whose working tree is at the base, so the pre-increment `waiver.line` still shows there; the actual HEAD code carries the `locator` refactor, confirmed in the diff).

## Source selection / gate: routes correctly for every input

Drove `validate --workflow` and `status --source` against crafted `.plan.toml` inputs:

- `primary = "toml"` -> TOML path. A `complete` TOML step with no rounds is caught by `check_workflow_toml` (`... toml_primary.plan.toml vs empty.jsonl: Roadmap step `toml-only`is`complete` but has no round records ...`), exit 1.
- `primary = "markdown"` -> Markdown path. The Markdown `md-only` step is caught, the TOML `toml-only` step is NOT checked. exit 1.
- Malformed TOML source -> `validate_source` reports it (`malformed `<task>.plan.toml`: TOML parse error ...`) AND the workflow check falls back to the Markdown plan (`md-only` caught). exit 1. The malformed source cannot route the check to a half-parsed TOML.
- No `--source` (the live shape) -> Markdown path, unchanged.
- `status --source` malformed -> one stderr note (`note: --source ... did not parse ...; projecting from --plan`) then falls back, exit 0; markdown-primary `--source` -> no note, projects from `--plan`; toml-primary `--source` -> projects from the TOML.

No input I could find routes the live Markdown repo to the TOML branch: the gate is `source_plan.as_ref().filter(|s| s.is_toml_primary())`, and `source_plan` is `None` whenever `--source` is absent, missing on disk, unparseable, or markdown-primary.

## Normalizers map faithfully into the check shapes

- `step_views`: `StepStatus::label()` yields the space-form Markdown vocabulary (`complete`, `in progress`, ...), and a `const _` drift guard plus a test pin every label to `plan::ROADMAP_STATUSES`. W3's guard is `status == "complete"`, which the `Complete => "complete"` label satisfies. The typed `blocked_by` is intentionally not projected (no `blocked on <slug>` string, which no check reads); documented on the fn.
- `question_views`: `Decided` renders to `format!("{}`{}`", QUEUE_FOLD_PREFIX, folded_into)` = `decided -> folded into `<slug>``, byte-identical to the Markdown decided form and matched by W4's existing `starts_with(QUEUE_FOLD_PREFIX)`gate with no fork. A decided question with an absent`folded_into`still starts with the prefix (empty backticks), so it stays in W4 scope and requires a receipt (fail-closed). The`superseded_by` parametric is intentionally not projected (no check reads it); documented on the fn.
- `waivers_from_toml`: mirrors `metrics::parse_waivers`' best-effort presence filtering exactly (increment present iff `unit == increment`; evidence present iff `record-backed`), dropping any violator; `step` comes from the enclosing TOML step; `locator` carries `TOML waiver `<id>``.
- `baseline_from_toml`: `[meta].w4_baseline` through `question_id_index` (the SAME function `metrics::parse_baseline` uses) into at most one `Baseline`. An absent or non-`Q-<n>` cutoff yields no baseline, so W4 requires a receipt for every decided item, the identical safe direction as the JSONL path with no baseline record.

## Malformed-waiver drop fails closed AND is reported

Hostile input (toml-primary, `complete` step, no rounds, a `record-backed` step waiver with the `evidence` field OMITTED) produced BOTH, in one run, exit 1:

- `validate_source` reports it: `... malwaiver.plan.toml: waiver `bad`on step`s`is`record-backed`but has no`evidence``.
- `waivers_from_toml` drops it, so it grants NO W3 exemption and the step is caught: `... Roadmap step `s`is`complete` but has no round records and no covering waiver`.

This is the synthesis invariant (reported-but-dropped) holding on the TOML substrate.

## W3/W4/W5 on TOML, and the un-launderable cross-substrate W5 join

- The pause.md catch fires on a `complete` TOML step with no rounds (test + live).
- Increment and step waiver exemptions work; the two real accepted-at-escalation shapes (`optional-modules-inc2cii` and the self-referential `waiver-model`) pass W3 and W5.
- W4 reads `[meta].w4_baseline`: a decided item at the cutoff is exempt, one strictly above with no receipt is flagged, one above with a JSONL receipt passes.
- W5's record-backed join is cross-substrate (mutable TOML waiver -> immutable JSONL escalation) and stays unit-scoped: a mis-tiered waiver and a waiver citing an escalation not scoped to its unit are both flagged. Live, the message names the waiver by its TOML id, not a false JSONL line: `TOML waiver `my-toml-waiver`: `record-backed`waiver cites evidence`x`but no`type:"escalation"` record ...` and `... waiver reason `predates-logging`must not carry evidence tier`record-backed``. Both W5 problems fire for the mis-tier fixture, matching the corrected S3 test comment.

## Live-path invariance

- `cargo run -- validate --metrics docs/metrics/workflow.jsonl --plan docs/plans/agent-scaffold.md --workflow` -> `docs/plans/agent-scaffold.md vs docs/metrics/workflow.jsonl: workflow invariants hold`, exit 0 (identical green).
- Files changed in the increment are exactly `src/{main,metrics,plan}.rs`, `src/plan/source.rs`, `src/workflow.rs`. `docs/plans/agent-scaffold.md` and `docs/metrics/workflow.jsonl` are untouched (0 files).
- The JSONL W5 messages reconstruct the pre-increment `round log line {n}: ...` prefix exactly via `locator`, and `locator` is `format!`-only (never a comparison, join, or enforcement input), so the type change from `usize` to `String` is a pure message carrier. `report_workflow` produces byte-identical summary/problem strings to the old inlined Markdown arm (confirmed by the identical live output).
- `render` is not touched and `source.rs` adds only methods (no serde change), so render output is unchanged; the render tests are green.

## Tests non-vacuous (Principle 11)

The 12 TOML tests call `check_workflow_toml` on real inline fixtures and assert specific problem counts and substrings (positive `is_empty` convergence cases AND negative catch/reject cases with pinned messages, including the TOML-id locator). No vacuous assertion. `waivers_from_toml`/`baseline_from_toml`/`step_views`/`question_views` are all exercised end to end through the two CLI surfaces.

## Settled items, not re-raised

- The `--workflow requires = "plan"` relaxation stays correctly DEFERRED (Inc 6, or Inc 5 at the orchestrator's discretion). I found nothing that makes it broken in a way that matters this increment: `--plan` is passed and ignored on the TOML arm, and its value need not exist on disk for the TOML path.
- The W5 locator, the four doc/help strings, the `status --source` parse note, and the mis-tier test comment were all fixed in round 1 and confirmed in round 2; I re-verified each live and they are correct. No new evidence to reopen any of them.

## Summary

Two consecutive clean rounds (round 2 x2, this round 3) now stand for this risky increment after a genuine independent re-attack. Nothing to fix.
