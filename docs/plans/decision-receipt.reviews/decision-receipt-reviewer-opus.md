# decision-receipt review: enforcement correctness and adversarial edge cases (opus)

Reviewed `main..impl/decision-receipt` (`70903bd` vs `db3ecd1`) in the worktree at
`/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/decision-receipt`.
Built and ran functionally: 183 tests pass, clippy clean, real repo
`validate --workflow` exits 0. Findings below; the top one is functionally
demonstrated on scratch fixtures, not assumed.

## F1. W4's min-index boundary silently false-PASSES a forgotten receipt (and false-FLAGS on a low-id receipt) [high]

Evidence: `src/workflow.rs:370` (`w4_problems`):

```
let Some(boundary) = decisions.iter().filter_map(|d| question_index(&d.q_id)).min() else { ... };
...
if index < boundary { continue; }
if !decisions.iter().any(|d| d.q_id == question.id) { push problem }
```

The boundary is the MINIMUM `q_id` index among the receipts that exist. This
conflates "predates the mechanism" with "index below the lowest recorded
receipt," and both failure directions are real. I confirmed both by running the
built binary, not by reading.

FALSE-PASS (the dangerous direction the lens warns about). Decide two items in
scope, Q-44 and Q-45, record a receipt for Q-45 only (the Q-44 receipt is
forgotten). The boundary becomes 45, so Q-44 (index 44 < 45) is silently
exempted even though it is a current, decided-and-folded item with no receipt,
exactly what W4 is supposed to catch. Scratch run:

```
plan: Q-44 (decided -> folded into step-a), Q-45 (decided -> folded into step-b)
log:  a single receipt for Q-45
$ agent-scaffold validate --workflow --plan plan2.md --metrics log1.jsonl
... workflow invariants hold
exit=0
```

Nothing else catches this: there is no malformed record for the strict
`validate_log` to report (the receipt just does not exist), so the E11
strict/best-effort split does not backstop it. The check provably cannot flag a
missing receipt for any decided item below the lowest recorded receipt index, so
it cannot guarantee the earliest (lowest-id) decision in any batch gets a
receipt whenever a later one is recorded. This generalises the documented
"zero-records requires nothing" bootstrap gap into a gap that also fires with
records present, which the doc comment does NOT acknowledge.

FALSE-FLAG (the other direction). A single receipt written for a low id, whether
a typo (intending `Q-44`, writing `Q-4` or `Q-1`) or a later re-decision of an
old item, drops the boundary and retroactively pulls dozens of historical items
into scope. Scratch run against the real plan (Q-1..Q-42 all decided-folded with
no receipts):

```
log: a single receipt for Q-1
$ agent-scaffold validate --workflow --plan docs/plans/agent-scaffold.md --metrics loglow.jsonl
-> 41 problems: "Open-Questions item `Q-2` .. `Q-42` is decided ... but has no matching receipt"
exit=1
```

One mistyped `q_id` turns the enforcement layer into 41 false positives over
items it is explicitly meant to exempt.

Why it matters: this is the enforcement backstop, and the min-index boundary is
non-monotonic under the natural operation of the log. It both fails to flag a
genuinely-missing current receipt (silent false-pass) and floods on a single
low-id receipt (false-flag). The unit tests only cover the happy monotonic path
(Q-44 recorded, Q-45 flagged) and never exercise a recorded receipt whose index
is below an unreceipted in-scope decided item, so they pass while the hole
stands. A per-item requirement (every decided-and-folded item at/after a FIXED
baseline id needs its own receipt), or a persisted `baseline` marker, would close
both directions; the derived-min boundary cannot.

## F2. Duplicated `QUEUE_FOLD_PREFIX` has no cross-check guard; drift silently disables W4 [low]

Evidence: `src/workflow.rs:48` and `src/plan.rs:87` both define
`const QUEUE_FOLD_PREFIX: &str = "decided -> folded into ";`. They are
byte-identical today (verified). `src/plan.rs:780` guards the plan TEMPLATE
against the `plan.rs` copy, but nothing ties the `workflow.rs` copy to either.
If the fold-status vocabulary is ever changed in `plan.rs` + template, the
`workflow.rs` copy drifts unnoticed: `question.status.starts_with(...)` at
`workflow.rs:131` matches nothing, every folded item falls out of W4's scope,
and W4 silently false-passes (requires no receipts at all) with all tests still
green. Why it matters: a drift here is a silent, total disablement of the check,
not a visible error. A one-line test asserting the two constants are equal (or
exposing one `pub(crate)` and reusing it) would pin them. Low because the
vocabulary is stable and the change is deliberate when it happens.

## F3. W4 trusts any projected receipt even when its `chosen`/`options` are malformed [low]

Evidence: `parse_decisions` (`src/metrics.rs:155`) projects any `decision`
record that merely has a string `q_id`, ignoring the `chosen in options`
constraint. `w4_problems` then treats it as a satisfying receipt via
`any(|d| d.q_id == question.id)`. So a receipt with `chosen` not in `options`
still satisfies W4. This is safe in `validate` ONLY because `validate_log`
(strict) runs alongside `check_workflow` under `--workflow` (`src/main.rs:541`
and `:600`), so the malformed record makes the whole command exit non-zero. I
verified both run together. Worth noting rather than a defect: W4 in isolation
would accept a malformed receipt as proof; the guarantee depends entirely on the
two checks always running in the same invocation. Low/informational.

## Clean confirmations (verified, not assumed)

- Strict vs best-effort split (E11) holds. A `decision` missing `q_id`, with a
  non-string `q_id`, or with `chosen` not in `options` is dropped by
  `parse_decisions` (invisible to W4) but reported by `validate_log`
  (`missing field ...` / `field chosen ... is not one of the presented options`).
  Both run under `validate --workflow`, so no malformed decision record is
  invisible to BOTH. Verified against the tests and by running the binary.
- `require_options` (`src/metrics.rs:81`): empty array rejected
  (`field options is empty`), non-string element rejected
  (`field options[1] has wrong type`), missing field rejected, and a non-array
  `options` returns an error (no panic). Verified. Duplicate options (e.g.
  `["A","A"]`) are accepted; that is permissive but not a correctness issue.
- `check_record` `decision` arm and `parse_decisions` are panic-free on a
  non-object line / non-array `options` (guarded by `as_object` / `as_array`).
- `w4_problems` returns empty with no decision records (current-plan-stays-green
  requirement); real repo `validate --workflow` exits 0. Verified.
- A `q_id` that is not `Q-<n>` does not parse (`question_index` -> `None`),
  contributes nothing to the boundary, and matches no live question; no crash.
- The drift-guard test extension (`src/metrics.rs:863`, `:872`) does add
  `"decision"` and the four fields `q_id`/`options`/`recommendation`/`chosen`,
  and the prose in `pack/instrument.md` contains each backticked, so the guard
  is satisfied and genuinely checks their presence. Caveat (pre-existing guard
  design, not new): the field check is a flat substring match not anchored to
  the `decision` type, so it only proves each name appears somewhere in the
  prose, not that it is documented under `decision`. Adequate here because all
  four names are unique to the decision line. Informational.
- `src/plan.rs`/`pack/plan-template.md` untouched; default (no `--instrument`)
  scaffold untouched; only the `--instrument` self-scaffold
  (`AGENTS.md`/`.agents/AGENTS.reference.md`) and `pack/instrument.md` changed,
  rendering byte-identical prose. No em-dash/en-dash/non-ASCII in the diff
  (`->` used throughout). 183 tests pass, clippy clean.
