# Inc 3 round 2 review - DESIGN FIDELITY, OUTPUT STRUCTURE, TEST/GOLDEN INTEGRITY

Reviewer lens: confirming round (round 2) on commit `69b21c4` (fix range `17a328e..69b21c4`,
full increment `9afe567..69b21c4`). Role: did NOT write this code. Sources read: all
13 changed files in the fix diff, the committed golden, the round-1 records
(`inc3-triage.md`, `inc3-reviewer-fidelity.md`), and the synthesis section 3+3(f).

Execution: ran `cargo test --all-targets` (284 green, 0 failed) and
`cargo run -- render --check src/plan/testdata/render-fixture.plan.toml`
(reports "up to date") in the worktree. Ran `cargo clippy --all-targets -- -D warnings`
(zero warnings). Confirmed `docs/plans/agent-scaffold.md` was last touched at commit
`9afe567` (the increment base) and is untouched by this fix round.

---

## Golden faithfulness verdict

`render --check` reports the golden up to date. The golden was not hand-edited: the
`render_is_deterministic_and_matches_the_golden` test asserts `render_plan(...) ==
GOLDEN` (where `GOLDEN` is `include_str!(...)`), so any byte difference between the
rendered output and the committed file would fail the test suite. The test suite is
green. The golden is faithful to the engine output.

---

## Fix confirmation: output-changing fixes

### F1 - question bodies in `## Question Details`

CONFIRMED. `question_details_section()` (`render.rs:525-535`) produces `## Question
Details` followed by each body sidecar in `Q-<n>` index order. `questions_section()`
now produces strictly one line per item with no blank lines between items (the body
content is omitted from this section entirely). The golden confirms the correct
structure: the queue is a clean flat list, then `## Step Details`, then `## Question
Details` (with five bodies spliced verbatim). The F1 test assertion verifies `queue_at
< details_at < q1_body_at`. Markdown list fragmentation from inline bodies is gone.

### F3 - vocabulary section heading renamed

CONFIRMED. `vocabulary_section()` (`render.rs:403-413`) now emits `## Roadmap Status
Vocabulary`. The golden carries this heading. The collision with the live plan's
`## Documentation Protocol` is resolved. The fragment test asserts
`out.contains("## Roadmap Status Vocabulary")`.

### C1 - open-question count includes `exploring`

CONFIRMED. `status_line()` (`render.rs:355-363`) now filters on
`QuestionStatus::Open | QuestionStatus::Exploring`. The fixture carries Q-9
(`exploring`) and Q-10 (`open`) alongside Q-1 (`open`), so the count is 3, confirmed
in the golden ("3 open questions") and in the dedicated test
`the_status_line_counts_open_and_exploring_but_not_decided_or_superseded` which builds
a 4-question plan (open, exploring, decided, superseded) and asserts "2 open questions".

---

## Fix confirmation: remaining round-1 fixes

All 12 fix-now items from the triage are present and verified:

- C2: `difference_summary()` (`render.rs:195-240`) detects the equal-lines case and
  explicitly words it as "trailing whitespace or a trailing newline"; test confirms.
- C3: `check_render()` (`render.rs:267-275`) matches only on `ErrorKind::NotFound` for
  `committed_exists: false`; any other I/O error propagates as `Err`; test with invalid
  UTF-8 bytes confirms.
- C4: `WaiverReason::ALL` constant added to `metrics.rs`; `status_line()` iterates it
  instead of a hand-written literal array.
- C5: `ordering_is_numeric_for_questions_and_slug_tiebroken_for_equal_order_steps()`
  asserts Q-1 < Q-9 < Q-10 (numeric, not lexical) and `epsilon` < `zeta` (slug
  tiebreak, both order 5 with `zeta` declared first in TOML). The test would fail on a
  lexical sort regression. The fixture comment documents the tiebreak intent. The
  skipped/optional/deferred Status-line buckets are also asserted.
- R1: `is_safe_sidecar_ref()` (`source.rs:358-372`) rejects absolute paths and
  `..`-bearing components; `validate_source` calls it for every front and tail ref;
  three tests pin traversal, absolute, and nested cases.
- R2: `escape_cell()` (`render.rs:504-505`) now neutralizes `\r`, `\r\n`, and `\n`;
  test `escape_cell_neutralizes_carriage_returns_and_newlines` covers all three.
- R3: `write_rendered()` (`render.rs:537-565`) writes to a temp sibling, then
  `fs::rename`s atomically; `main.rs` now calls `write_rendered` instead of
  `fs::write`; failed rename cleans up the temp file.
- F4: `render_sha256` is absent from the fixture TOML.
- F5: `render_writes_exactly_one_file_and_check_is_green_after` uses `snapshot_dir()`
  to record all source bytes before the render, then asserts every source is
  byte-identical after, and that exactly one new file was created (`<task>.md`).

---

## New findings

### N1 (low) - `question_details_section` emits the heading unconditionally

`src/plan/render.rs:525-534`, `question_details_section()`

The function always returns `"## Question Details"` as its prefix, even when
`question_blobs` is empty or every body trims to an empty string. For a plan with no
`[[question]]` entries (valid per `validate_source`, which imposes no minimum count),
`assemble()` unconditionally pushes this section (`render.rs:312`), yielding a bare
`## Question Details` heading with no content before the tail or end of file.

This mirrors `step_details_section`'s identical pattern (accepted without comment in
round 1), and the doc comment explicitly says it "Mirrors `step_details_section`."
The fix comment also says "a question with no body prose contributes no entry" -- this
addresses the per-question case but not the all-bodies-empty or no-questions case.

Impact is narrow: the fixture always has questions with bodies; the live plan has
questions; a plan with zero questions is unusual in practice. But the structural
inconsistency is real: front and tail sidecars ARE guarded (`if !trimmed.is_empty()`)
before being pushed to sections, while the two details sections are not.

Suggested direction: guard both `step_details_section` and `question_details_section`
at their call sites in `assemble()` (or internally) so the heading is suppressed when
there would be no content beneath it, matching the prose-sidecar guard pattern. This
makes the no-questions edge case produce valid output rather than a dangling heading.

DEFER is acceptable. The current code is correct for every realistic plan and for the
live plan. This is a latent structural edge case, not a current correctness gap.

---

## Regression and structure check

- Test count: 284 (280 lib + 1 + 3 integration). Round-1 miscount of 275/276 is
  resolved; the fix round added tests and the count now matches the implementer's
  stated 284.
- Clippy: zero warnings under `-D warnings`.
- The live plan (`docs/plans/agent-scaffold.md`) is untouched.
- The F1/F3 changes extend the Inc 5 expected-diff set: the generated output now has
  `## Roadmap Status Vocabulary` (not `## Documentation Protocol`) and `## Question
  Details` as a distinct section. The synthesis lists "question-body relocation" as an
  Inc 5 expected diff, which is precisely what F1 implements. F3's rename adds an
  additional expected diff (heading change) that should be recorded in the Inc 5
  expected-diffs list before migration.

---

## Summary

CLEAN except for one low finding (N1, defer acceptable). All 13 round-1 findings are
fixed and confirmed. The golden is faithful: `render --check` reports up to date and
`render_is_deterministic_and_matches_the_golden` passes. F1/F3/C1 landed correctly.
