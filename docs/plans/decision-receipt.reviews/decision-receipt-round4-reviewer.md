# decision-receipt round-4 reviewer findings

Branch `impl/decision-receipt` tip `5d6fa21`, diff `main..impl/decision-receipt` (main `a3fdce5`).
Fresh, independent whole-artifact read. READ-ONLY.

## Verdict: CLEAN

No remaining valid defect. The artifact is ready to converge.

## What I verified

### Build / run (worktree `.claude/worktrees/decision-receipt`)
- `just test`: 186 unit tests + integration suites pass, 0 failures.
- `just clippy` (`cargo clippy --all-targets`): clean, no warnings.
- `cargo run -- validate --workflow --plan docs/plans/agent-scaffold.md`: exits 0
  ("86 records, valid"; "50 steps, 44 open-questions items, valid"; "workflow
  invariants hold").

### Soundness of W4 + the two record validators (functional, scratch copies)
- Strict-vs-best-effort split intact. A `decision` receipt for an above-cutoff
  item with `chosen` NOT in `options` (best-effort `parse_decisions` projects it
  on `q_id` alone, so W4 would treat it as satisfied) is still reported by
  `validate_log` (`field chosen ... is not one of the presented options`) and the
  run exits 1. Confirmed in `run_validate` (src/main.rs:539-557): `validate_log`
  runs unconditionally on the present metrics file and its errors feed the same
  `problems` list as the workflow check, so a malformed decision/baseline record
  can never silently exempt anything.
- W4 fires end-to-end: baseline Q-44 + decided Q-45 with NO receipt -> flagged;
  same with a valid Q-45 receipt -> W4 passes; NO baseline + decided Q-45 with no
  receipt -> flagged. All exit codes as expected.
- No panic path in the new code: `require_options` (as_array/as_str, no unwrap),
  `question_id_index` (strip_prefix + `parse::<u64>` returning `Option`, so an
  overflowing or non-numeric id yields `None`, not a panic), and `w4_problems`
  (no indexing) are all total.
- Boundary is a DECLARED cutoff read from `type:"baseline"`, not derived from the
  receipt set (round-2 conclusion holds on a fresh read): a missing receipt cannot
  move its own exemption. `.last()` gives deterministic last-one-wins; `None`
  (no baseline) exempts nothing, the safe direction.

### Coherence
- Schemas, `pack/instrument.md`/`AGENTS.md`/`.agents/AGENTS.reference.md` prose,
  the drift guard, and the tests agree. The drift-guard test (src/metrics.rs:1010)
  reads the real `include_str!("../pack/instrument.md")` and asserts every accepted
  type (`decision`, `baseline` added) and every checked field (`q_id`, `options`,
  `recommendation`, `chosen`, `questions_through` added) is documented. Nothing
  checked-but-undocumented; the documented fields are exactly the checked ones.
- Migration baseline record is well-formed and correct:
  `{"type":"baseline","task":"decision-receipt","questions_through":"Q-44","ts":"2026-07-18"}`.
  `task` is required for all records (src/metrics.rs:321) and present; the live plan
  has 42 decided-and-folded items (all `<= Q-44`), Q-43 superseded, Q-44 open, so
  the Q-44 cutoff correctly exempts every pre-mechanism decided item. (The Q-44
  cutoff pre-exempting the still-open Q-44 is the round-1..3 ACCEPTED, intended
  behavior; no new evidence it is wrong.)

### Regression
- W3, the round-log consistency check, and the byte-identical default scaffold are
  untouched by the diff (the metrics.rs additions are new arms/functions/tests;
  plan.rs changes only the `QUEUE_FOLD_PREFIX` visibility to `pub(crate)` with a
  doc note; main.rs unchanged). AGENTS.md and .agents/AGENTS.reference.md carry the
  identical instrument-block edit (same blob hashes in the diff).
- `pub(crate) QUEUE_FOLD_PREFIX` and `question_id_index` are single-sourced and
  reused by W4, so the fold-prefix and cutoff-id parsing cannot drift between
  plan.rs / metrics.rs / workflow.rs.
- No non-ASCII, em/en dash, or double-hyphen-as-dash in any added source, docs, or
  the JSONL line. `#[expect]`/`#[allow]` convention not implicated (none added).

### Test quality (Principle 11)
- The boundary is genuinely exercised, not just smoke-tested. Plausible mutations
  are caught: `<= cutoff` -> `< cutoff` fails the at-cutoff exemption test;
  dropping the `Some(cutoff)` guard fails the at-or-below test; making the
  no-baseline case exempt everything fails `w4_with_no_baseline_requires_a_receipt`;
  removing the `chosen in options` check fails
  `a_decision_with_chosen_not_in_options_is_rejected`; removing the empty-array
  check fails `a_decision_with_an_empty_options_array_is_reported`. Both the
  strictly-above (flag) and above-with-receipt (pass) directions, and both
  no-baseline directions, are tested, plus non-decided items ignored and the
  best-effort projections' skip-vs-project behavior.

## Conclusion

CLEAN. This is a converging clean round; no valid defect remains.
