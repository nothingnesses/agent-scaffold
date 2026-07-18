# decision-receipt round-2 adversarial review (reviewer: opus)

Lens: soundness of the new baseline-based W4 enforcement core.
Scope: `main..impl/decision-receipt` (tip `efe90c6`). Read and run in the worktree
`/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/decision-receipt`.

Verdict: NOT clean. One NEW LOW finding (L1). The core soundness question (is the
baseline boundary non-circular; can a current decided item be silently exempted?)
is answered: no silent false-pass path found, verified functionally. One residual
design observation (O1) is recorded for the human, not counted as a defect.

Functional verification performed (all in the worktree):
- `cargo test`: 186 + 1 + 3 pass.
- `cargo clippy`: clean (exit 0).
- Real repo `validate --workflow --plan docs/plans/agent-scaffold.md`: exit 0
  (`86 records, valid`; `50 steps, 44 open-questions items, valid`; invariants hold).
- Scratch experiment (decided Q-45 above a Q-44 cutoff, no receipt): W4 flags it;
  the below-cutoff Q-42 is exempt. Correct.
- Scratch experiment (malformed cutoff `OQ-a`, decided Q-5): BOTH defenses fire,
  `validate_log` reports `field questions_through value OQ-a is not a Q-<n> id`
  AND W4 still requires Q-5's receipt (no silent exemption). Correct.

---

## L1 (LOW) - byte-identical duplicate of the `Q-<n>` index parser feeds the W4 cutoff comparison; same drift class the round-1 fix removed for `QUEUE_FOLD_PREFIX`

`src/metrics.rs:305-307` (`question_id_index`) and `src/workflow.rs:101-103`
(`question_index`) are byte-identical:

```
id.strip_prefix("Q-").and_then(|digits| digits.parse::<u64>().ok())
```

These two functions produce the two operands of W4's exemption comparison at
`src/workflow.rs:150` (`if index <= cutoff`): `question_id_index` (via
`parse_baseline` -> `Baseline.questions_through`) produces the cutoff, and
`question_index` produces the decided item's index. The soundness of the
`index <= cutoff` placement depends on both parsing `Q-<n>` identically. They do
today, so there is no live mis-behavior; this is a drift risk, not a current bug.

Why it is a valid NEW finding and not a re-raise of the settled QUEUE_FOLD_PREFIX
low: this is a DIFFERENT symbol, and the fix commit `efe90c6` itself CREATED the
duplication (it added `question_id_index` to `metrics.rs` while `workflow.rs`
already carried `question_index` from `8d6e052`). So while the round-1 fix
deduplicated `QUEUE_FOLD_PREFIX` with the stated rationale that a "second
byte-identical copy that could drift out of sync and silently disable W4" is a
defect, the same commit left a fresh byte-identical copy of the `Q-<n>` index
parse across the metrics/workflow boundary. If one copy is later changed (a wider
id shape, whitespace trimming, a different overflow policy) and the other is not,
a decided item could be misplaced relative to the cutoff: exempted when it should
be checked (silent false-pass) or flagged when it should be exempt.

Suggested fix (parallel to the round-1 QUEUE_FOLD_PREFIX fix): expose
`metrics::question_id_index` as `pub(crate)` and call it from `w4_problems`
instead of the local `question_index`, so the cutoff and the item index are
placed by one definition. (`plan::is_question_id` is a third `Q-` treatment but
has a different signature/purpose (validity bool for parse inclusion) and does not
feed the numeric comparison, so it is out of scope for this finding.)

Severity rationale: LOW. No current defect; it is a maintainability/drift risk of
exactly the class round-1 rated LOW for QUEUE_FOLD_PREFIX. Raising it because the
fix that removed one such duplicate introduced another in the same enforcement
path.

---

## O1 (observation, not a defect) - the declared cutoff `Q-44` pre-exempts the currently-open `Q-44`

The migration record is `{"type":"baseline",...,"questions_through":"Q-44"}`. In
the live plan the decided-and-folded frontier is `Q-42` (`Q-43` is `superseded`,
which is never in W4 scope; `Q-44` is `open`). So the minimal cutoff that exempts
today's decided set is `Q-42`; `Q-44` additionally pre-exempts the currently-OPEN
`Q-44`. When `Q-44` is eventually decided under the mechanism, W4 will not require
a receipt for it (`44 <= 44`), even though that decision is made after adoption.

I do not count this as a defect: the exemption is DECLARED and VISIBLE in the log
(the fix's whole ethos is a declared, auditable cutoff, never a silently inferred
one), and it matches the step's stated intent (exempt current `Q-1..Q-44`, require
receipts for `Q-45+`). Treating "every queue item that existed at adoption" rather
than "every item already decided at adoption" as the boundary is a coherent, and
deliberately chosen, definition. Recorded only so the human can confirm that
letting `Q-44`'s eventual decision go un-receipted is intended; if not, the
baseline would need to have been `Q-43`. No code change required for soundness.

---

## Soundness attacks that FAILED (the fix holds) - confirmations

- Non-circular boundary: the cutoff is read from an independent `type:"baseline"`
  record, never derived from the receipt set. A missing/forgotten receipt cannot
  move the cutoff (verified: Q-45 above a Q-44 cutoff with no receipt is flagged,
  and adding/removing that receipt does not change the cutoff). The round-1
  circularity (min-index self-moving boundary) is genuinely gone.
- No-baseline case: `cutoff` is `None`, so nothing is exempt and every decided
  item requires a receipt (unit tests `w4_with_no_baseline_*` plus the
  `parse_baseline` empty projection). No silent "off" state.
- Too-high declared cutoff over-exempts, but visibly: a `questions_through` of an
  arbitrarily high `Q-<n>` is a valid record that `validate_log` accepts and W4
  honours, exempting more items. This is a declared, on-the-record decision, not a
  silent false-pass, consistent with the declared-cutoff model. Acceptable.
- Multiple baselines: `parse_baseline` preserves file order and `w4_problems`
  takes `.last()`. Deterministic and file-order-stable. A malformed baseline
  interleaved between two valid ones is dropped by `parse_baseline` (so `.last()`
  is the last VALID cutoff) AND reported by `validate_log`, so it cannot silently
  supersede.
- Malformed cutoff cannot be invisible to BOTH paths: `check_record`'s `baseline`
  arm and `parse_baseline` use the SAME `question_id_index` predicate, so a record
  that the schema accepts is exactly what the projection reads; a non-`Q-<n>`,
  missing, or wrong-typed `questions_through` is dropped by `parse_baseline`
  (treated as NO exemption, the safe direction) and simultaneously reported by
  `validate_log`. Verified functionally end to end.
- Schema arm no-panic: `check_record` guards non-object (`record is not a JSON
  object`) and missing `type`/`task`/`questions_through` via `require_str` before
  the `Q-<n>` check; no `unwrap`/index panic on malformed input. The `baseline`
  and `decision` arms return `Err`, never panic (fuzzed via the wrong-typed,
  missing, empty-array, non-array, and non-string-element unit tests, all green).
- `decision` cross-field constraint: `chosen` must be a member of the non-empty
  string `options` array; the `chosen`-not-in-`options` and empty/non-array/
  non-string-element cases are all rejected with precise messages.
- Regression: W3, the round-log consistency check, and the byte-identical default
  scaffold all still pass (full `cargo test` green; real `validate --workflow`
  exit 0). Exposing `plan::QUEUE_FOLD_PREFIX` as `pub(crate)` is a visibility-only
  change with no behavioral effect (the constant value is unchanged and W4 reuses
  the single definition).
- Conventions: no em-dash/en-dash/double-hyphen-as-dash/emoji/non-ASCII in the
  added lines; no `#[allow]` (none introduced); comments use ASCII `->`.
