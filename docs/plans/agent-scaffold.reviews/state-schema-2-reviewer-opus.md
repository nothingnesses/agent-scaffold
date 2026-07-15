# state-schema increment 2, reviewer (opus, CORRECTNESS lens)

Range reviewed: `4de2b35..bdc0955`. Artifact: the `validate` subcommand (`run_validate` in `src/main.rs`) and the metrics schema validator `src/metrics.rs`, plus the V-4 `task`/`ts` fields in `pack/instrument.md`.

Scope note: `status`, plan/ledger parsing, and cross-reference checks are increment 3; their absence is not assessed here, per the review brief.

## Verdict

The schema validator is correct and complete against the intended schema. Every documented rule is enforced, line numbering and record counting are consistent, per-line reporting continues past a bad line, and the exit codes match the spec (0 valid/absent, 1 malformed, 2 usage). No critical, high, or medium correctness findings. One low informational note follows.

## Empirical verification

- `direnv exec . just test`: 63 passed, 0 failed.
- `direnv exec . cargo clippy --all-targets -- -D warnings`: clean.
- `direnv exec . just scaffold-self` then `git diff --stat -- AGENTS.md .agents/ docs/plans/TEMPLATE.md`: empty (idempotent).
- Hand-built JSONL fixtures in scratch, run through `cargo run -- validate --metrics <path>`:
  - absent file -> stderr "nothing to validate", exit 0. Correct.
  - valid multi-record -> stdout "N records, valid", exit 0. Correct.
  - empty file -> "0 records, valid", exit 0. Correct.
  - whitespace-only file -> "0 records, valid", exit 0. Correct.
  - non-object (`[1,2,3]`) -> "record is not a JSON object", exit 1. Correct.
  - `type` as a number -> "field `type` has wrong type (expected string)", exit 1. Correct.
  - no trailing newline -> "1 records, valid", exit 0 (no phantom blank line). Correct.
  - multi-line with a bad record on line 2, a blank line 3, and bad JSON on line 4 -> reported both line 2 and line 4 with correct 1-based numbers (blank skipped, not miscounted), exit 1. Confirms per-line continuation and blank-line line-number preservation.
  - float count (`valid_findings: 1.5`) -> rejected as "not a non-negative integer", exit 1. Correct.
  - unknown subcommand and bare invocation -> exit 2, distinct from validation failure (1). Correct.
  - malformed errors go to stderr, not stdout (stdout empty when stderr suppressed). Correct.

## Rule-by-rule schema check (`src/metrics.rs`)

All enforced correctly:

- Common `type` (required string, closed enum), `task` (required string), optional `ts` (string when present): `check_record` lines 196-200. Unknown `type` -> error (line 232).
- `round`: `artifact` str, `phase` enum {plan_review, work_review, acceptance}, `changed_since_prev` bool, `outcome` enum {clean, new_valid}, `valid_findings` count>=0, `severities` array of {low, medium, high, critical}, `consecutive_clean` count>=0. Lines 203-213. Enum sets and spellings match the schema exactly (lines 58-107).
- `escalation`: `artifact` str, `human_decision` enum {decision, resume}. Lines 214-219.
- `dismissal_recheck`: `artifact` str, `result` enum {upheld, overturned}. Lines 220-225.
- `intake`: `classification` enum {trivial, non_trivial}, `replanned` bool. Lines 226-231.
- Non-negativity of counts: `require_count` uses `Number::as_u64` (lines 134-145), which rejects negatives, fractions, booleans, and strings. Verified empirically (negative via the existing unit test, float via a fixture).
- `severities` element validation: `require_severities` (lines 165-185) rejects a non-array, a non-string element, and an out-of-set element, each with an indexed message. Empty array is accepted (correct: no elements to fail).
- Optional `ts` typed only when present (lines 198-200). Verified: `ts: 123` rejected; absent `ts` accepted.
- Unknown extra fields permitted: `check_record` never iterates for unknown keys; confirmed by the `"note":"extra field ok"` record in the test fixture and the valid-run fixture.

Line numbering and counting:

- `validate_log` (lines 249-267) numbers with `index + 1` over `contents.lines()`, skipping blank/whitespace lines but keeping their positions, so numbers are 1-based and match the file as an editor shows it. Verified with the multi-line fixture.
- `count_records` (lines 240-242) counts the same non-blank lines `validate_log` parses, using the identical `line.trim().is_empty()` predicate, so "N records" and the validated set agree. Consistent.
- Continuation: `validate_log` collects one reason per bad line and continues; it does not stop at the first. `check_record` returns the first violation per line (via `?`), which is the intended one-reason-per-line behaviour.

Exit codes (`run_validate`, `src/main.rs` lines 336-353): absent -> `Ok(())` (0); valid -> stdout summary + `Ok(())` (0); malformed -> per-line `path:line: reason` to stderr + `std::process::exit(1)`. Usage errors elsewhere use `exit(2)` and clap's own parse errors also exit 2, so validation failure (1) is distinct. All verified empirically.

## Findings

### Critical

None.

### High

None. No schema rule is left unenforced; no wrong JSON-type check; no missing or wrong enum variant; counts reject negatives and fractions.

### Medium

None.

### Low

- L1 (low, informational; defensible as-is): duplicate JSON keys are silently resolved last-wins rather than flagged. `serde_json::from_str::<Value>` accepts a record like `{"type":"intake",...,"type":"round"}` and keeps the last `type`, so a record whose keys were duplicated by the hand-writing LLM validates against whichever value happened to come last (in my fixture this surfaced only incidentally, as a downstream "missing field `artifact`" once `type` became `round`; a benign duplicate of a data field would pass with no signal at all). The intended schema says nothing about duplicate keys, standard JSON permits them, and serde's last-wins is conventional, so this is not a schema-rule violation and is reasonable to leave. I note it only because the module's stated purpose is detecting subtly corrupt hand-written data (module doc lines 6-11), and a duplicated key is exactly that class of corruption; if increment 3 wants stricter parsing it could reject duplicates, but that is a design choice, not a defect in what was specified for this increment.

## Principles

Judged against Principles 1-7 and the correctness-relevant ones. The module enforces the schema at the boundary and turns malformed input into precise, located error messages (Principle 14, parse don't validate; Principle 12, fail loudly), defines each enum set once via the `enum_field!` macro so the checked set and the modelled type cannot drift (Principle 16), and the change is verified by tests and by direct runs (Principle 6). No principle violation found.
