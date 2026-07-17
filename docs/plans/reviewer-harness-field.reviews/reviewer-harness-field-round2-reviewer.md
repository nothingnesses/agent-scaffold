# reviewer-harness-field round 2 (confirming) - independent reviewer

Verdict: CLEAN round. No high/critical/medium/low defects. The two round-1 low fixes landed correctly and the full change is still sound.

Range reviewed: `305ea86..4fb4c0b` (branch `impl/reviewer-harness-field`, commits `9f87003` then `4fb4c0b`). The main working tree sits at the base, so all review was done against branch content via `git show 4fb4c0b:...` and an isolated detached worktree at `4fb4c0b` (removed after; main untouched).

## Round-1 fixes confirmed

- L-1 (drift-guard field order): FIXED. In the field-name array of `instrument_prose_documents_every_accepted_schema_value` (`src/metrics.rs`, branch lines 702-707), the order is now `reviewers`, `role`, `model`, `raw_findings`, `harness`, `human_decision`. `harness` is the last per-reviewer field, matching validator check order (role, model, raw_findings, valid_findings, then optional harness at branch lines 258-266) and the prose. Note this array is a membership set for `contains` checks, so ordering is cosmetic only; the placement is correct and readable. No functional impact.
- L-2 (doc comment purpose clause): FIXED. `require_reviewers` doc comment (branch `src/metrics.rs:231-232`) now reads "running multiple models or harnesses earns its cost", matching `pack/instrument.md` ("multiple models or harnesses").

## Core change re-verified

- `harness` is optional and validated as string only when present: `if entry.contains_key("harness") { at(require_str(entry, "harness").map(|_| ()))?; }` (branch `src/metrics.rs:265-267`). A reviewer entry without it still validates.
- The four required per-reviewer fields are unchanged: `role`, `model` (strings), `raw_findings`, `valid_findings` (non-negative ints) at branch `src/metrics.rs:258-261`.
- `reviewers` array remains optional (caller guards with `contains_key`, branch line 307) and still rejects `[]` (branch lines 245-247; test at 617).
- Prose in all three copies (`pack/instrument.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`) is byte-identical for the changed paragraph (matching md5), so the self-scaffold is regenerated consistently with the pack source. No drift.

## Checks run (against branch tip in worktree)

- `cargo test`: 116 passed, 0 failed (114 base + 2 new: `a_reviewers_element_with_a_valid_harness_is_accepted`, `a_reviewers_element_with_a_non_string_harness_is_reported`). Both assert the intended behavior; the wrong-type test's expected message `field ` + "`reviewers`" + `[0]: field ` + "`harness`" + ` has wrong type (expected string)` matches the array-position-prefixed error path.
- `cargo run -- validate --metrics docs/metrics/workflow.jsonl`: 66 records, valid. The real log still validates.
- `cargo clippy --all-targets`: clean.
- Added lines contain no non-ASCII characters. ASCII-clean.
- Only the four intended files changed across the full range (`.agents/AGENTS.reference.md`, `AGENTS.md`, `pack/instrument.md`, `src/metrics.rs`); no stray edits.

## Not re-raised (settled, no new evidence)

- M-1 missing CHANGELOG entry: orchestrator-owned merge-time bookkeeping, out of this branch.
- Opus round-1 observations L1/L2/L3 (accept-test proof, absent-harness coverage, `"harness": null` rejected as wrong-type): triager-ruled valid observations, not defects; acceptable as-is. The null-rejection behavior is a reasonable consequence of `contains_key` + `require_str`.
- Unique/marginal-valid count: explicitly out of scope.

## New issues introduced by the round-1 fixes

None. The `4fb4c0b` diff touches only the drift-guard array order and one doc-comment line; both are cosmetic/documentation and do not alter validation behavior. Tests confirm no regression.
